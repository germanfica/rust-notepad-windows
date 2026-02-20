extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::ffi::OsString;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use nwd::NwgUi;
use nwg::{NativeUi, fatal_message};

#[derive(Default, NwgUi)]
pub struct NotepadApp {
    #[nwg_control(size: (500, 400), position: (300, 300), title: "Rust Notepad", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [NotepadApp::exit] )]
    window: nwg::Window,

    #[nwg_control(size: (480, 330), position: (10, 10))]
    text_edit: nwg::TextBox,

    #[nwg_control(parent: window)]
    #[nwg_events(OnMenuItemSelected: [NotepadApp::menu_handler])]
    menu: nwg::Menu,

    #[nwg_control(parent: menu, text: "File")]
    file_menu: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Open")]
    open_menu: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Save")]
    save_menu: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Exit")]
    exit_menu: nwg::MenuItem,
}

impl NotepadApp {
    fn menu_handler(&self, item: &nwg::MenuItem) {
        if item == &self.open_menu {
            self.open_file();
        } else if item == &self.save_menu {
            self.save_file();
        } else if item == &self.exit_menu {
            self.exit();
        }
    }

    fn open_file(&self) {
        let mut dialog = nwg::FileDialog::default();
        nwg::FileDialog::builder()
            .title("Open File")
            .action(nwg::FileDialogAction::Open)
            .filters("Text Files(*.txt)|*.txt")
            .build(&mut dialog)
            .unwrap();

        if dialog.run(Some(&self.window)) {
            match dialog.get_selected_item() {
                Ok(file_path) => {
                    let path = Path::new(&file_path);
                    let mut file = match File::open(&path) {
                        Ok(file) => file,
                        Err(_) => {
                            fatal_message("Error", "Failed to open the file.");
                            return;
                        }
                    };

                    let mut contents = String::new();
                    file.read_to_string(&mut contents).unwrap();
                    self.text_edit.set_text(&contents);
                }
                Err(_) => {
                    fatal_message("Error", "Failed to get the selected item.");
                }
            }
        }
    }

    fn save_file(&self) {
        let mut dialog = nwg::FileDialog::default();
        nwg::FileDialog::builder()
            .title("Save File")
            .action(nwg::FileDialogAction::Save)
            .filters("Text Files(*.txt)|*.txt")
            .build(&mut dialog)
            .unwrap();

        if dialog.run(Some(&self.window)) {
            match dialog.get_selected_item() {
                Ok(file_path) => {
                    let path = Path::new(&file_path);
                    let mut file = match File::create(&path) {
                        Ok(file) => file,
                        Err(_) => {
                            fatal_message("Error", "Failed to save the file.");
                            return;
                        }
                    };

                    if let Err(_) = file.write_all(self.text_edit.text().as_bytes()) {
                        fatal_message("Error", "Failed to write to the file.");
                    }
                }
                Err(_) => {
                    fatal_message("Error", "Failed to get the selected item.");
                }
            }
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    let _app = NotepadApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
