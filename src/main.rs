use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Orientation, Align};
use rand::Rng;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(build_ui);

    app.run()
}

fn build_ui(app: &Application) {
    let window = create_main_window(app);
    let vbox = create_vbox();

    let label = create_label();
    vbox.append(&label);

    let increment_button = create_button("Increment");
    connect_button(&increment_button, &label, 1);
    vbox.append(&increment_button);

    let decrement_button = create_button("Decrement");
    connect_button(&decrement_button, &label, -1);
    vbox.append(&decrement_button);

    let reset_button = create_button("Reset");
    connect_reset_button(&reset_button, &label);
    vbox.append(&reset_button);

    window.set_child(Some(&vbox));
    window.present();
}

fn create_main_window(app: &Application) -> ApplicationWindow {
    ApplicationWindow::builder()
        .application(app)
        .default_width(320)
        .default_height(200)
        .title("Counter")
        .build()
}

fn create_vbox() -> gtk::Box {
    let vbox = gtk::Box::new(Orientation::Vertical, 5);
    vbox.set_margin_top(20);
    vbox.set_margin_bottom(20);
    vbox.set_margin_start(20);
    vbox.set_margin_end(20);
    vbox.set_halign(Align::Center);
    vbox.set_valign(Align::Center);
    vbox
}

fn create_label() -> Label {
    let label = Label::new(Some("0"));
    label.set_markup(&format_label_markup("black", 0));
    label
}

fn create_button(label: &str) -> Button {
    Button::with_label(label)
}

fn connect_button(button: &Button, label: &Label, increment: i32) {
    button.connect_clicked({
        let label = label.clone();
        move |_| {
            let current_value: i32 = label.text().parse().unwrap_or(0);
            label.set_markup(&format_label_markup(random_color().as_str(), current_value + increment));
        }
    });
}

fn connect_reset_button(button: &Button, label: &Label) {
    button.connect_clicked({
        let label = label.clone();
        move |_| {
            label.set_markup(&format_label_markup("black", 0));
        }
    });
}

fn format_label_markup(color: &str, value: i32) -> String {
    format!(
        "<span size='xx-large' foreground='{}'>{}</span>",
        color, value
    )
}

fn random_color() -> String {
    let mut rng = rand::rng();
    format!(
        "#{:02X}{:02X}{:02X}",
        rng.random_range(0..256),
        rng.random_range(0..256),
        rng.random_range(0..256)
    )
}
