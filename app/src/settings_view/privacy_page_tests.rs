use warpui::App;

use super::*;
use crate::settings_view::settings_page::FilteredPageType;

#[test]
fn chinese_privacy_search_preserves_page_title_and_clears() {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = PageType::new_uncategorized(
                vec![
                    Box::new(DataManagementWidget::default()),
                    Box::new(PrivacyPolicyWidget::default()),
                ],
                Some(PageTitle::new("Privacy")),
            );
            assert!(page.update_filter("隐私 政策", ctx).is_truthy());
            let FilteredPageType::Uncategorized { widgets, title, .. } = page.get_filtered() else {
                panic!("expected uncategorized page");
            };
            assert_eq!(widgets.len(), 1);
            assert!(title.is_some());
            page.update_filter("", ctx);
            let FilteredPageType::Uncategorized { widgets, .. } = page.get_filtered() else {
                panic!("expected uncategorized page");
            };
            assert_eq!(widgets.len(), 2);
        });
    });
}
