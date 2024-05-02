#[cfg(test)]
mod tests {
    const TEST_FILE_PATH_VIEW: &str = "./tests/view";
    const TEST_FILE_PATH_EDIT: &str = "./tests/edit";
    use crate::App;

    fn setup_app_view() -> App {
        App::new(Some(TEST_FILE_PATH_VIEW.to_string()))
    }

    fn setup_app_edit() -> App {
        App::new(Some(TEST_FILE_PATH_EDIT.to_string()))
    }

    fn assert_parent_folder_state(app: &App) {
        assert_eq!(app.get_current_dir_list().unwrap().files.len(), 2);
        assert_eq!(app.get_current_dir_list().unwrap().folders.len(), 2);
    }

    fn assert_folder1_state(app: &App) {
        assert_eq!(app.get_current_dir_list().unwrap().files.len(), 2);
        assert_eq!(app.get_current_dir_list().unwrap().folders.len(), 0);
    }

    fn assert_delete_folder_state(app: &App) {
        assert_eq!(app.get_current_dir_list().unwrap().files.len(), 2);
        assert_eq!(app.get_current_dir_list().unwrap().folders.len(), 1);
    }

    fn assert_cursor_index(app: &App, index: usize) {
        assert_eq!(app.get_current_dir_list().unwrap().cursor_index, index);
    }

    mod file_tree {
        use super::*;

        #[test]
        fn has_correct_amount_of_current_folder_files_and_folders() {
            let app = setup_app_view();

            assert_parent_folder_state(&app)
        }

        #[test]
        fn has_correct_amount_file_tree_keys() {
            let app = setup_app_view();

            let file_tree = app.file_tree_map;

            assert_eq!(file_tree.keys().len(), 3);
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

            for _ in 0..10 {
                app.cursor_down();
            }
            assert_cursor_index(&mut app, 4);
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
            assert_folder1_state(&app);
        }

        #[test]
        fn navigates_back_to_parent_folder() {
            let mut app = setup_app_view();

            app.cursor_down();
            app.enter_pressed();

            assert_folder1_state(&app);

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
            assert_cursor_index(&app, 3);

            app.enter_pressed();

            assert_cursor_index(&app, 3);
            assert_parent_folder_state(&app);
        }
    }

    mod delete {
        use super::*;
        use std::fs::{self, File};
        use std::io::Write;

        const TEST_FOLDER_NAME: &str = "folder_for_delete";
        const TEST_FILE_NAMES: [&str; 2] = ["file1_to_delete.txt", "file2_to_delete.txt"];
        const TEST_FILE_SIZE: usize = 447;

        fn generate_lorem_ipsum() -> String {
            String::from(
                "Lorem ipsum dolor sit amet, consectetur adipiscing elit. \
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi \
ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit \
in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur \
sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt \
mollit anim id est laborum.
",
            )
        }

        fn cleanup_testing_files() {
            let folder_path = format!("{}/{}", TEST_FILE_PATH_EDIT, TEST_FOLDER_NAME);
            if let Err(err) = fs::remove_dir_all(&folder_path) {
                eprintln!("Failed to remove test folder: {}", err);
            }
        }

        fn create_testing_files() {
            let folder_path = format!("{}", TEST_FILE_PATH_EDIT);
            let sub_folder_path = format!("{}/{}", TEST_FILE_PATH_EDIT, TEST_FOLDER_NAME);
            fs::create_dir_all(&sub_folder_path).expect("Failed to create test folder");

            for file_name in &TEST_FILE_NAMES {
                let file_path = format!("{}/{}", folder_path, file_name);
                let mut file = File::create(&file_path).expect("Failed to create test file");
                writeln!(file, "{}", generate_lorem_ipsum()).expect("Failed to write to test file");

                let sub_file_path = format!("{}/{}", sub_folder_path, file_name);
                let mut file = File::create(&sub_file_path).expect("Failed to create test file");
                writeln!(file, "{}", generate_lorem_ipsum()).expect("Failed to write to test file");
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
            assert_eq!(app.get_current_dir_list().unwrap().files.len(), 2);
            assert_eq!(app.get_current_dir_list().unwrap().folders.len(), 0);
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
            assert_eq!(app.get_current_dir_list().unwrap().files.len(), 1);
            assert_eq!(app.get_current_dir_list().unwrap().folders.len(), 1);
            cleanup_testing_files();
        }

        #[test]
        fn updated_current_folder_size() {
            create_testing_files();
            let mut app = setup_app_edit();

            let root_entry = app.get_current_dir_list().unwrap();
            assert_eq!(root_entry.total_size, (TEST_FILE_SIZE * 4) as u64);

            app.cursor_down();
            app.cursor_down();
            app.delete_pressed();

            let root_entry_updated = app.get_current_dir_list().unwrap();
            assert_eq!(root_entry_updated.total_size, (TEST_FILE_SIZE * 3) as u64);

            cleanup_testing_files();
        }

        #[test]
        fn updated_parent_folder_size() {
            create_testing_files();
            let mut app = setup_app_edit();

            let root_entry = app.get_current_dir_list().unwrap();
            assert_eq!(root_entry.total_size, (TEST_FILE_SIZE * 4) as u64);

            app.cursor_down();
            app.enter_pressed();
            app.cursor_down();
            app.delete_pressed();
            app.cursor_up();
            app.enter_pressed();

            let root_entry_updated = app.get_current_dir_list().unwrap();
            assert_eq!(root_entry_updated.total_size, (TEST_FILE_SIZE * 3) as u64);

            cleanup_testing_files();
        }
    }
}
