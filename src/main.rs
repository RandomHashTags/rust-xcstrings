use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Builder, FileDialog, Label, gio, glib};
use glib::clone;
use std::collections::HashMap;
use std::string::String;
use std::fs;
use serde::{Deserialize, Serialize};

const APP_ID: &str = "me.randomhashtags.XCStrings";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_startup(setup_shortcuts);
    app.connect_activate(build_ui);
    app.run()
}

fn setup_shortcuts(app: &Application) {
    app.set_accels_for_action("win.open", &["<Ctrl>o"]);
    app.set_accels_for_action("window.close", &["<Ctrl>q"]);
}

fn build_ui(app: &Application) {
    // We create the main window.
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Rust XCStrings")
        .build();
    setup_actions(&window);

    // Show the window.
    window.present();
}

fn setup_actions(window: &ApplicationWindow) {
    let action_open = gio::SimpleAction::new("open", None);
    action_open.connect_activate(clone!(@weak window => move |_, _| {
        let file_dialog = FileDialog::builder().modal(false).build();
        file_dialog.open(Some(&window), None::<&gio::Cancellable>, clone!(@weak window => move |result| {
            let text:String;
            match result {
                 Ok(file) => {
                    let path = file.path().unwrap();
                    text = fs::read_to_string(path).unwrap();
                    load(text);
                 },
                 Err(e) => text = format!("Error: {e:#?}")
            };
        }));
    }));
    window.add_action(&action_open);
}
fn load(text: String) {
    let deserialized:StringCatalog = serde_json::from_str(&text).unwrap();
    let lang:String = deserialized.sourceLanguage;
    let version:String = deserialized.version;
    println!("{lang};{version}");
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalog {
    sourceLanguage: String,
    version: String
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogEntry {
    comment: Option<String>,
    extractionState: StringCatalogExtractionState,
    localizations: HashMap<String, StringCatalogLocalization>
}

#[derive(Serialize, Deserialize, Debug)]
enum StringCatalogExtractionState {
    manual, migrated
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogStringUnit {
    stringUnit: StringCatalogUnit
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogUnit {
    state: StringCatalogUnitState,
    value: String
}

#[derive(Serialize, Deserialize, Debug)]
enum StringCatalogUnitState {
    needs_review, new, stale, translated
}


#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogLocalization {
    stringUnit: Option<StringCatalogUnit>,
    variations: StringCatalogVariations,
    substitutions: HashMap<String, StringCatalogSubstitution>
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogVariations {
    plural: Option<StringCatalogVariationPlural>
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogSubstitution {
    argNum: i64,
    formatSpecifier: String,
    variations: StringCatalogSubstitutionVariations
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogSubstitutionVariations {
    plural: Option<StringCatalogVariationPlural>
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogVariationDevice {
    appletv: Option<StringCatalogRawVariations>,
    applevision: Option<StringCatalogRawVariations>,
    applewatch: Option<StringCatalogRawVariations>,
    ipad: Option<StringCatalogRawVariations>,
    iphone: Option<StringCatalogRawVariations>,
    ipod: Option<StringCatalogRawVariations>,
    mac: Option<StringCatalogRawVariations>,
    other: Option<StringCatalogRawVariations>
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogVariationPlural {
    zero: Option<StringCatalogStringUnit>,
    one: Option<StringCatalogStringUnit>,
    two: Option<StringCatalogStringUnit>,
    few: Option<StringCatalogStringUnit>,
    many: Option<StringCatalogStringUnit>,
    other: Option<StringCatalogStringUnit>
}

#[derive(Serialize, Deserialize, Debug)]
struct StringCatalogRawVariations {
    variations: StringCatalogSubstitutionVariations,
    stringUnit: StringCatalogStringUnit
}