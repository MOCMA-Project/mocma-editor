use gtk::prelude::*;

fn main() {
    //Crée l'app
    let app = gtk::Application::builder().application_id("editor.mocma.project").build();

    //Charge le CSS
    app.connect_startup(|_| load_css());

    //Crée la fenêtre
    app.connect_activate(build_ui);

    //Lance l'app
    app.run();
}

//Charger le CSS
fn load_css() {
    // Load the CSS file and add it to the provider
    let provider = gtk::CssProvider::new();
    provider.load_from_data(include_bytes!("style.css"));

    // Add the provider to the default screen
    gtk::StyleContext::add_provider_for_display(
        &gtk::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &gtk::Application){
    //Crée la fenêtre
    let window = gtk::ApplicationWindow::builder().application(app).title("Mocma Editor").build();

    //Ajoute la classe CSS "window" à la fenêtre
    window.set_css_classes(&["window"]);

    //Crée un bouton avec le texte "Useless button" et une marge de 20 de tous les côtés
    let button = gtk::Button::builder()
        .label("Useless button")
        .margin_start(20)
        .margin_top(20)
        .margin_end(20)
        .margin_bottom(20)
        .build();

    //Ajoute le bouton à la fenêtre
    window.set_child(Some(&button));

    //Affiche la fenêtre
    window.present();
}