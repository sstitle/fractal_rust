use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Label, Orientation, Align};
use rand::Rng;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(320)
            .default_height(200)
            .title("Counter")
            .build();

        // Create a vertical box to hold the label and buttons.
        let vbox = gtk::Box::new(Orientation::Vertical, 5);
        vbox.set_margin_top(20);
        vbox.set_margin_bottom(20);
        vbox.set_margin_start(20);
        vbox.set_margin_end(20);
        vbox.set_halign(Align::Center);
        vbox.set_valign(Align::Center);

        // Create a label with large font and initial color.
        let label = Label::new(Some("0"));
        label.set_markup("<span size='xx-large' foreground='black'>0</span>");
        vbox.append(&label);

        // Create an increment button.
        let increment_button = Button::with_label("Increment");
        vbox.append(&increment_button);

        // Connect increment button click event to increment the label and change color.
        increment_button.connect_clicked({
            let label = label.clone();
            move |_| {
                let current_value: i32 = label.text().parse().unwrap_or(0);
                label.set_markup(&format!(
                    "<span size='xx-large' foreground='{}'>{}</span>",
                    random_color(),
                    current_value + 1
                ));
            }
        });

        // Create a decrement button.
        let decrement_button = Button::with_label("Decrement");
        vbox.append(&decrement_button);

        // Connect decrement button click event to decrement the label and change color.
        decrement_button.connect_clicked({
            let label = label.clone();
            move |_| {
                let current_value: i32 = label.text().parse().unwrap_or(0);
                label.set_markup(&format!(
                    "<span size='xx-large' foreground='{}'>{}</span>",
                    random_color(),
                    current_value - 1
                ));
            }
        });

        // Create a reset button.
        let reset_button = Button::with_label("Reset");
        vbox.append(&reset_button);

        // Connect reset button click event to reset the label and change color.
        reset_button.connect_clicked({
            let label = label.clone();
            move |_| {
                label.set_markup("<span size='xx-large' foreground='black'>0</span>");
            }
        });

        // Add the vbox to the window.
        window.set_child(Some(&vbox));

        // Show the window.
        window.present();
    });

    app.run()
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
