use ratatui::layout::{Constraint, Layout, Rect};

pub mod footer;
pub mod stats;
pub mod table;

fn area(area: Rect) -> (Rect, Rect, Rect) {
    let [stats, table, footer] = Layout::vertical([
        Constraint::Length(10),
        Constraint::Min(0),
        Constraint::Length(2),
    ])
    .areas(area);

    (stats, table, footer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn areas_tile_correctly() {
        let terminal = Rect::new(0, 0, 120, 40);
        let (stats, table, footer) = area(terminal);

        assert_eq!(stats.height, 10);
        assert_eq!(footer.height, 2);

        assert_eq!(stats.height + footer.height, 40 - table.height);

        assert_eq!(stats.height + table.height + footer.height, 40);

        assert_eq!(stats.width, 120);
        assert_eq!(table.width, 120);
        assert_eq!(footer.width, 120);
    }
}
