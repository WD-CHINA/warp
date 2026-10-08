use warpui::App;

use super::*;
use crate::settings_view::settings_page::FilteredPageType;

#[test]
fn chinese_pane_search_keeps_only_matching_widget_and_category() {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = PageType::new_categorized(
                vec![
                    Category::new(
                        "Panes",
                        vec![
                            Box::new(DimInactivePanesWidget::default()),
                            Box::new(FocusFollowsMouseWidget::default()),
                        ],
                    ),
                    Category::new("Blocks", vec![Box::new(CompactModeWidget::default())]),
                ],
                None,
            );
            assert!(page.update_filter("调暗 窗格", ctx).is_truthy());
            let FilteredPageType::Categorized { categories, .. } = page.get_filtered() else {
                panic!("expected categorized page");
            };
            assert_eq!(categories.len(), 1);
            assert_eq!(categories[0].title, "Panes");
            assert_eq!(categories[0].widgets.len(), 1);
            page.update_filter("", ctx);
            let FilteredPageType::Categorized { categories, .. } = page.get_filtered() else {
                panic!("expected categorized page");
            };
            assert_eq!(categories.len(), 2);
            assert_eq!(categories[0].widgets.len(), 2);
        });
    });
}
