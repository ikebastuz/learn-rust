#[cfg(test)]
mod tests2 {
    const TEST_FILE_PATH_VIEW: &str = "./tests/view";
    use crate::App;

    fn setup_app_view() -> App {
        App::new(Some(TEST_FILE_PATH_VIEW.to_string()))
    }

    mod file_tree {
        use crate::fs::FolderEntryType;

        use super::*;

        fn assert_item_at_index_is(app: &App, index: usize, kind: FolderEntryType) {
            assert_eq!(
                app.get_current_dir_list()
                    .unwrap()
                    .entries
                    .get(index)
                    .unwrap()
                    .kind,
                kind
            );
        }

        fn assert_item_at_index_title(app: &App, index: usize, title: String) {
            assert_eq!(
                app.get_current_dir_list()
                    .unwrap()
                    .entries
                    .get(index)
                    .unwrap()
                    .title,
                title
            );
        }

        #[test]
        fn test_ordering_by_kind() {
            let app = setup_app_view();

            assert_item_at_index_is(&app, 0, FolderEntryType::Parent);
            assert_item_at_index_is(&app, 1, FolderEntryType::Folder);
            assert_item_at_index_is(&app, 2, FolderEntryType::Folder);
            assert_item_at_index_is(&app, 3, FolderEntryType::Folder);
            assert_item_at_index_is(&app, 4, FolderEntryType::File);
            assert_item_at_index_is(&app, 5, FolderEntryType::File);
            assert_item_at_index_is(&app, 6, FolderEntryType::File);
        }

        #[test]
        fn test_ordering_by_title() {
            let app = setup_app_view();

            assert_item_at_index_title(&app, 0, "..".to_string());
            assert_item_at_index_title(&app, 1, "a_folder".to_string());
            assert_item_at_index_title(&app, 2, "b_folder".to_string());
            assert_item_at_index_title(&app, 3, "c_folder".to_string());
            assert_item_at_index_title(&app, 4, "a_root_file.txt".to_string());
            assert_item_at_index_title(&app, 5, "d_root_file.txt".to_string());
            assert_item_at_index_title(&app, 6, "z_root_file.txt".to_string());
        }
    }
}
