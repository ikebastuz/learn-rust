#[cfg(test)]
mod tests {
    const TEST_FILE_PATH_VIEW: &str = "./tests/view";
    const TEST_FILE_PATH_EDIT: &str = "./tests/edit";
    use crate::fs::{FolderEntry, FolderEntryType};
    use crate::App;

    fn setup_app_view() -> App {
        App::new(Some(TEST_FILE_PATH_VIEW.to_string()))
    }

    fn setup_app_edit() -> App {
        App::new(Some(TEST_FILE_PATH_EDIT.to_string()))
    }

    fn assert_item_at_index_is(app: &App, index: usize, kind: FolderEntryType) {
        assert_eq!(
            app.get_current_folder()
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
            app.get_current_folder()
                .unwrap()
                .entries
                .get(index)
                .unwrap()
                .title,
            title
        );
    }

    fn get_entry_by_kind(app: &App, kind: FolderEntryType) -> Vec<FolderEntry> {
        app.get_current_folder()
            .unwrap()
            .entries
            .iter()
            .filter(|e| e.kind == kind)
            .cloned()
            .collect()
    }

    fn assert_parent_folder_state(app: &App) {
        assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 3);
    }

    fn assert_parent_folder_a_state(app: &App) {
        assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 2);
        assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 0);
    }

    fn assert_delete_folder_state(app: &App) {
        assert_eq!(get_entry_by_kind(app, FolderEntryType::File).len(), 3);
        assert_eq!(get_entry_by_kind(app, FolderEntryType::Folder).len(), 1);
    }

    fn assert_cursor_index(app: &App, index: usize) {
        assert_eq!(app.get_current_folder().unwrap().cursor_index, index);
    }

    mod file_tree {
        use super::*;

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

        #[test]
        fn has_correct_amount_file_tree_keys() {
            let app = setup_app_view();

            let file_tree = app.file_tree_map;

            assert_eq!(file_tree.keys().len(), 4);
        }
    }

    mod cursor {
        use super::*;

        #[test]
        fn updates_cursor_position() {
            let mut app = setup_app_view();

            assert_cursor_index(&mut app, 0);

            app.cursor_down();
            assert_cursor_index(&mut app, 1);

            app.cursor_up();
            assert_cursor_index(&mut app, 0);
        }

        #[test]
        fn stops_cursor_at_very_top() {
            let mut app = setup_app_view();

            assert_cursor_index(&mut app, 0);

            for _ in 0..10 {
                app.cursor_up();
            }

            assert_cursor_index(&mut app, 0);
        }

        #[test]
        fn stops_cursor_at_very_bottom() {
            let mut app = setup_app_view();

            for _ in 0..20 {
                app.cursor_down();
            }
            assert_cursor_index(&mut app, 6);
        }
    }

    mod handle_enter {
        use super::*;

        #[test]
        fn updates_current_tree_when_enters_subfolder() {
            let mut app = setup_app_view();

            app.cursor_down();
            app.enter_pressed();

            assert_cursor_index(&app, 0);
            assert_parent_folder_a_state(&app);
        }

        #[test]
        fn navigates_back_to_parent_folder() {
            let mut app = setup_app_view();

            app.cursor_down();
            app.enter_pressed();

            assert_parent_folder_a_state(&app);

            app.enter_pressed();
            assert_parent_folder_state(&app);
            assert_cursor_index(&app, 1);
        }

        #[test]
        fn does_nothing_when_tries_to_enter_file() {
            let mut app = setup_app_view();

            app.cursor_down();
            app.cursor_down();
            app.cursor_down();
            app.cursor_down();
            app.cursor_down();
            assert_cursor_index(&app, 5);

            app.enter_pressed();

            assert_cursor_index(&app, 5);
            assert_parent_folder_state(&app);
        }
    }

    mod delete {
        use super::*;
        use std::fs::{self, File};
        use std::io::Write;

        const TEST_FILE_SIZE: usize = 446;

        fn generate_lorem_ipsum() -> String {
            String::from(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
    Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
    Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi \
    ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit \
    in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur \
    sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt \
    mollit anim id est laborum.",
            )
        }

        /// - folder_1
        ///     - folder_2
        ///         - folder_3
        ///         - file_1
        ///         - file_2
        ///         - file_3
        ///     - file_1
        ///     - file_2
        ///     - file_3
        /// - file_1
        /// - file_2
        /// - file_3
        fn create_testing_files() {
            fs::create_dir_all(TEST_FILE_PATH_EDIT).expect("Failed to create test folder");

            let mut folder_path = format!("{}", TEST_FILE_PATH_EDIT);

            for folder_index in 1..4 {
                for file_index in 1..4 {
                    let file_name = format!("file_to_delete_{}.txt", file_index);
                    let file_path = format!("{}/{}", folder_path, file_name);
                    let mut file = File::create(&file_path).expect("Failed to create test file");
                    writeln!(file, "{}", generate_lorem_ipsum())
                        .expect("Failed to write to test file");
                }

                folder_path = format!("{}/folder_to_delete_{}", folder_path, folder_index);

                fs::create_dir_all(&folder_path).expect("Failed to create test folder");
            }
        }

        fn cleanup_testing_files() {
            if let Err(err) = fs::remove_dir_all(TEST_FILE_PATH_EDIT) {
                eprintln!("Failed to remove test folder: {}", err);
            }
        }

        #[test]
        fn has_correct_initial_state() {
            create_testing_files();
            let app = setup_app_edit();
            assert_delete_folder_state(&app);
            cleanup_testing_files();
        }

        #[test]
        fn does_nothing_when_cursor_is_at_the_top() {
            create_testing_files();
            let mut app = setup_app_edit();
            assert_cursor_index(&app, 0);
            assert_delete_folder_state(&app);
            app.delete_pressed();
            assert_delete_folder_state(&app);
            cleanup_testing_files();
        }

        #[test]
        fn deletes_folder() {
            create_testing_files();
            let mut app = setup_app_edit();
            assert_delete_folder_state(&app);
            app.cursor_down();
            app.delete_pressed();
            assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 3);
            assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 0);
            cleanup_testing_files();
        }

        #[test]
        fn deletes_file() {
            create_testing_files();
            let mut app = setup_app_edit();
            assert_delete_folder_state(&app);
            app.cursor_down();
            app.cursor_down();
            app.delete_pressed();
            assert_eq!(get_entry_by_kind(&app, FolderEntryType::File).len(), 2);
            assert_eq!(get_entry_by_kind(&app, FolderEntryType::Folder).len(), 1);
            cleanup_testing_files();
        }

        #[test]
        fn updated_current_folder_size() {
            create_testing_files();
            let mut app = setup_app_edit();

            let root_entry = app.get_current_folder().unwrap();
            assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

            app.cursor_down();
            app.cursor_down();
            app.delete_pressed();

            let root_entry_updated = app.get_current_folder().unwrap();
            assert_eq!(root_entry_updated.get_size(), (TEST_FILE_SIZE * 8) as u64);

            cleanup_testing_files();
        }

        #[test]
        fn deleting_file_updates_parent_folders_sizes() {
            create_testing_files();
            let mut app = setup_app_edit();

            let root_entry = app.get_current_folder().unwrap();
            assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

            app.cursor_down();
            app.enter_pressed();

            let folder_1 = app.get_current_folder().unwrap();
            assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

            app.cursor_down();
            app.enter_pressed();

            let folder_2 = app.get_current_folder().unwrap();
            assert_eq!(folder_2.get_size(), (TEST_FILE_SIZE * 3) as u64);

            app.cursor_down();
            app.cursor_down();
            app.delete_pressed();

            let folder_2_upd = app.get_current_folder().unwrap();
            assert_eq!(folder_2_upd.get_size(), (TEST_FILE_SIZE * 2) as u64);

            app.cursor_up();
            app.cursor_up();
            app.enter_pressed();

            let folder_1_upd = app.get_current_folder().unwrap();
            assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 5) as u64);
            assert_eq!(
                folder_1_upd.get_selected_entry_size(),
                (TEST_FILE_SIZE * 2) as u64
            );

            app.cursor_up();
            app.enter_pressed();

            let root_entry_upd = app.get_current_folder().unwrap();
            assert_eq!(root_entry_upd.get_size(), (TEST_FILE_SIZE * 8) as u64);
            assert_eq!(
                root_entry_upd.get_selected_entry_size(),
                (TEST_FILE_SIZE * 5) as u64
            );

            cleanup_testing_files();
        }

        #[test]
        fn deleting_folder_updates_parent_folders_sizes() {
            create_testing_files();
            let mut app = setup_app_edit();

            let root_entry = app.get_current_folder().unwrap();
            assert_eq!(root_entry.get_size(), (TEST_FILE_SIZE * 9) as u64);

            app.cursor_down();
            app.enter_pressed();

            let folder_1 = app.get_current_folder().unwrap();
            assert_eq!(folder_1.get_size(), (TEST_FILE_SIZE * 6) as u64);

            app.cursor_down();
            app.delete_pressed();

            let folder_1_upd = app.get_current_folder().unwrap();
            assert_eq!(folder_1_upd.get_size(), (TEST_FILE_SIZE * 3) as u64);

            app.cursor_up();
            app.enter_pressed();

            let root_entry_upd = app.get_current_folder().unwrap();
            assert_eq!(root_entry_upd.get_size(), (TEST_FILE_SIZE * 6) as u64);
            assert_eq!(
                root_entry_upd.get_selected_entry_size(),
                (TEST_FILE_SIZE * 3) as u64
            );

            cleanup_testing_files();
        }
    }
}
