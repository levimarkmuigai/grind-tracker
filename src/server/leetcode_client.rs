use reqwest::{Client, header};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::server::{actors::ProblemFromLeetcode, db, error::ServerError};

#[derive(Deserialize)]
struct GraphQlResponse {
    data: GraphQLData,
}

#[derive(Deserialize)]
struct GraphQLData {
    #[serde(rename = "questionList")]
    question_list: QuestionList,
}

#[derive(Deserialize)]
struct QuestionList {
    data: Vec<ProblemFromLeetcode>,
}

async fn fetch_problems() -> Result<Vec<ProblemFromLeetcode>, ServerError> {
    let client = Client::new();
    let url = "https://leetcode.com/graphql";

    let payload = serde_json::json!({
        "query": r#"
        query problemsetQuestionList($categorySlug: String, $limit: Int, $skip: Int, $filters: QuestionListFilterInput) {
            questionList(categorySlug: $categorySlug, limit: $limit, skip: $skip, filters: $filters) {
                data {
                    questionFrontendId
                    title
                    titleSlug
                    difficulty
                    topicTags {
                        name
                    }
                }
            }
        }
    "#,
        "variables": {
            "categorySlug": "",
            "skip": 0,
            "limit": 50,
            "filters": {
                "searchKeywords": "pareto50"
            }
        }
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

    Ok(parsed_res.data.question_list.data)
}

pub async fn seed_db_if_needed(pool: &SqlitePool) -> Result<(), ServerError> {
    let count = db::pareto50_count(pool).await?;

    if count >= 50 {
        tracing::info!(
            "database already seeded with problems {}, skipping fetch",
            count
        );
        return Ok(());
    }

    tracing::info!(
        "found {} problems, fetching Pareto50 from leetcode...",
        count
    );

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
