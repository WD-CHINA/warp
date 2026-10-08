use warpui::App;

use super::*;
use crate::settings_view::settings_page::FilteredPageType;

#[test]
fn english_language_search_is_scoped() {
    assert_language_filter("language");
}

#[test]
fn chinese_language_search_is_scoped() {
    assert_language_filter("界面 语言");
}

fn assert_language_filter(query: &'static str) {
    App::test((), |mut app| async move {
        app.update(|ctx| {
            let mut page = PageType::new_uncategorized(
                vec![Box::new(LanguageWidget), Box::new(OtherWidget)],
                Some(PageTitle::new("Account")),
            );
            assert!(page.update_filter(query, ctx).is_truthy());
            let FilteredPageType::Uncategorized { widgets, title, .. } = page.get_filtered() else {
                panic!("expected Uncategorized page");
            };
            assert_eq!(widgets.len(), 1);
            assert!(title.is_some());
            page.update_filter("", ctx);
            let FilteredPageType::Uncategorized { widgets, .. } = page.get_filtered() else {
                panic!("expected Uncategorized page");
            };
            assert_eq!(widgets.len(), 2);
        });
    });
}

struct OtherWidget;

impl SettingsWidget for OtherWidget {
    type View = MainSettingsPageView;

    fn search_terms(&self) -> &str {
        "unrelated"
    }

    fn render(&self, _: &Self::View, _: &Appearance, _: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
