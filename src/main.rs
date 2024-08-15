extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use nwd::NwgUi;
use nwg::{NativeUi, fatal_message};

#[derive(Default, NwgUi)]
pub struct BasicApp {
    //#[nwg_control(size: (500, 400), position: (300, 300), title: "Rust Notepad", flags: "WINDOW|VISIBLE")]
    // https://docs.rs/native-windows-gui/latest/native_windows_gui/struct.WindowFlags.html
    #[nwg_control(size: (500, 400), position: (300, 300), title: "Rust Notepad", flags: "MAIN_WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    #[nwg_control(text: "Heisenberg", size: (280, 25), position: (10, 10))]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name", size: (280, 60), position: (10, 40))]
    #[nwg_layout_item(layout: grid, col: 0, row: 1)]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button,

    #[nwg_control(size: (480, 330), position: (10, 10))]
    #[nwg_layout_item(layout: grid, col: 0, row: 2, row_span: 7)]
    text_edit: nwg::TextBox,

    #[nwg_control(parent: window, text: "File")]
    //#[nwg_events(OnMenuItemSelected: [BasicApp::menu_handler])]
    menu: nwg::Menu,

    #[nwg_control(parent: menu, text: "Open")]
    #[nwg_events(OnMenuItemSelected: [BasicApp::open_file])]
    open_menu: nwg::MenuItem,

    #[nwg_control(parent: menu, text: "Save")]
    #[nwg_events(OnMenuItemSelected: [BasicApp::save_file])]
    save_menu: nwg::MenuItem,

    #[nwg_control(parent: menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [BasicApp::exit])]
    exit_menu: nwg::MenuItem,
}

impl BasicApp {

    fn say_hello(&self) {
        nwg::simple_message("Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    fn say_goodbye(&self) {
        nwg::simple_message("Goodbye", &format!("Goodbye {}", self.name_edit.text()));
        nwg::stop_thread_dispatch();
    }

    fn open_file(&self) {
        println!("Open file!");

        // Crear un diálogo de archivo
        let mut dialog = nwg::FileDialog::default();

        let fixed_path = "C:\\Users\\germa\\Downloads";

        let dir = ::std::env::current_dir().unwrap();
        let builder_result = nwg::FileDialog::builder()
            .title("Open File")
            .action(nwg::FileDialogAction::Open)
            .default_folder(fixed_path)
            .filters("Text(*.txt)|Any(*.*)")
            .build(&mut dialog);

        // Verificar si el diálogo fue construido correctamente
        if builder_result.is_ok() {
            println!("Creación del dialog exitosa!");

            if dialog.run(Some(&self.window)) {
                //let mut selectedItem = dialog.get_selected_item();

                //if let Some(file_path) = dialog.get_selected_item() // no devuelve option el dialog
                match dialog.get_selected_item() {
                    Ok(file_path) => {
                        let path = Path::new(&file_path);
                        let mut file = match File::open(&path) {
                            Ok(file) => file,
                            Err(_) => {
                                nwg::fatal_message("Error", "Failed to open the file.");
                                //return;
                            }
                        };

                        let mut contents = String::new();
                        if file.read_to_string(&mut contents).is_ok() {
                            self.text_edit.set_text(&contents);
                        }
                    }
                    Err(e) => {
                        nwg::fatal_message("Error", &format!("Failed to get the selected file: {:?}", e));
                    }
                }
            }
        }else {
            nwg::fatal_message("Error", "Failed to create the file dialog.");
        }
    }

    fn save_file(&self) {
        println!("Save file!");

        // Crear y configurar el diálogo
        let mut dialog = nwg::FileDialog::default();

        let fixed_path = "C:\\Users\\germa\\Downloads";

        let builder_result = nwg::FileDialog::builder()
            .title("Save File")
            .action(nwg::FileDialogAction::Save)
            .default_folder(fixed_path)
            .filters("Text(*.txt)|Any(*.*)")
            .build(&mut dialog);

        // Verificar si el diálogo fue construido correctamente
        if builder_result.is_ok() {
            // Ejecutar el diálogo
            if dialog.run(Some(&self.window)) {
                match dialog.get_selected_item() {
                    Ok(file_path) => {
                        let path = Path::new(&file_path);
                        let mut file = match File::create(&path) {
                            Ok(file) => file,
                            Err(_) => {
                                nwg::fatal_message("Error", "Failed to create the file.");
                                return;
                            }
                        };

                        if let Err(_) = file.write_all(self.text_edit.text().as_bytes()) {
                            nwg::fatal_message("Error", "Failed to write to the file.");
                        } else {
                            nwg::simple_message("Success", "File saved successfully!");
                        }
                    }
                    Err(e) => {
                        nwg::fatal_message("Error", &format!("Failed to get the selected file: {:?}", e));
                    }
                }
            }
        } else {
            nwg::fatal_message("Error", "Failed to create the file dialog.");
        }
    }
    
    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
