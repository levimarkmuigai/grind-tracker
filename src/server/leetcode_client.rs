use std::collections::HashMap;

use reqwest::{Client, header};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::server::{actors::ProblemFromLeetcode, db, error::ServerError};

const SLUGS: &[&str] = &[
    // 1. Arrays & hashing
    "contains-duplicate",
    "valid-anagram",
    "two-sum",
    "group-anagrams",
    "top-k-frequent-elements",
    "product-of-array-except-self",
    // 2. Two pointers
    "valid-palindrome",
    "two-sum-ii-input-array-is-sorted",
    "3sum",
    "container-with-most-water",
    // 3. Sliding window
    "best-time-to-buy-and-sell-stock",
    "longest-substring-without-repeating-characters",
    "permutation-in-string",
    // 4. Stack
    "valid-parentheses",
    "min-stack",
    "daily-temperatures",
    "minimum-remove-to-make-valid-parentheses",
    // 5. Binary search
    "binary-search",
    "search-in-rotated-sorted-array",
    "koko-eating-bananas",
    // 6. Linked list
    "reverse-linked-list",
    "merge-two-sorted-lists",
    "linked-list-cycle",
    "remove-nth-node-from-end-of-list",
    "lru-cache",
    // 7. Trees
    "invert-binary-tree",
    "maximum-depth-of-binary-tree",
    "validate-binary-search-tree",
    "binary-tree-level-order-traversal",
    "lowest-common-ancestor-of-a-binary-search-tree",
    // 8. Heap
    "kth-largest-element-in-an-array",
    "find-median-from-data-stream",
    // 9. Graphs
    "number-of-islands",
    "clone-graph",
    "course-schedule",
    "rotting-oranges",
    // 10. Backtracking
    "subsets",
    "combination-sum",
    "generate-parentheses",
    "palindrome-partitioning",
    // 11. DP
    "climbing-stairs",
    "house-robber",
    "coin-change",
    "longest-increasing-subsequence",
    "partition-equal-subset-sum",
    // 12. Intervals / greedy
    "merge-intervals",
    "insert-interval",
    "partition-labels",
];

#[derive(Deserialize)]
struct GraphQlResponse {
    data: HashMap<String, ProblemFromLeetcode>,
}

async fn fetch_problems() -> Result<Vec<ProblemFromLeetcode>, ServerError> {
    let client = Client::new();
    let url = "https://leetcode.com/graphql";

    let fields = SLUGS
        .iter()
        .enumerate()
        .map(|(i, slug)| {
            format!(
                r#"q{i}: question(titleSlug: "{slug}") {{
                questionFrontendId
                title
                titleSlug
                difficulty
                topicTags {{ name }}
            }}"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let payload = serde_json::json!({
        "query": format!("query neetcodeStyleSet {{ {fields} }}"),
    });

    let res = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::USER_AGENT, "grind-tracker-cli")
        .header(header::REFERER, "https://leetcode.com/")
        .json(&payload)
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let net_error = res.error_for_status_ref().unwrap_err();
        let error_body = res.text().await?;

        tracing::error!("leetcode graphql error {}:{}", status, error_body);

        return Err(ServerError::Network(net_error));
    }

    let parsed_res = res.json::<GraphQlResponse>().await?;

    Ok(parsed_res.data.into_values().collect())
}

pub async fn seed_db_if_needed(pool: &SqlitePool) -> Result<(), ServerError> {
    let count = db::problem_count(pool).await?;

    if count >= 75 {
        tracing::info!(
            "database already seeded with problems {}, skipping fetch",
            count
        );
        return Ok(());
    }

    tracing::info!("found {} problems, fetching from leetcode...", count);

    let problems: Vec<ProblemFromLeetcode> = fetch_problems().await?;

    for problem in problems {
        let joined_tags = problem
            .topic_tag
            .into_iter()
            .map(|t| t.name)
            .collect::<Vec<String>>()
            .join(", ");

        db::insert_problem_with_card(
            pool,
            problem.frontend_id,
            problem.title,
            problem.slug,
            joined_tags,
            problem.level,
        )
        .await?;
    }

    tracing::info!("successfully seeded problems into db");

    Ok(())
}
