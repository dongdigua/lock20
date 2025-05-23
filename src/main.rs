use gtk4 as gtk;
use gtk4::glib::{self, clone, ControlFlow};
use gtk4::gdk;
use gtk::prelude::*;
use gtk4_session_lock::Instance as SessionLockInstance;

fn main() {
    if !gtk4_session_lock::is_supported() {
        println!("Session lock not supported")
    }

    let app = gtk::Application::new(
        Some("com.github.wmww.gtk4-layer-shell.session-lock-example"),
        Default::default(),
    );

    app.connect_activate(activate);
    app.run();
}

fn activate(app: &gtk::Application) {
    glib::timeout_add_seconds(10, clone!(
        #[strong] app,
        #[upgrade_or] ControlFlow::Break,
        move || {
            display_lock(&app);
            ControlFlow::Break
        }));
}
fn display_lock(app: &gtk::Application) {
    let lock = SessionLockInstance::new();

    if !lock.lock() {
        // Error message already shown when handling the ::failed signal
        return;
    }

    let display = gdk::Display::default().unwrap();
    let monitors = display.monitors();

    for monitor in monitors.iter::<glib::Object>() {
        let monitor = monitor.unwrap().downcast::<gdk::Monitor>().unwrap();
        let window = gtk::ApplicationWindow::new(app);
        lock.assign_window_to_monitor(&window, &monitor);

        let button = gtk::Button::builder()
            .label("Unlock")
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        let lock = lock.clone();
        button.connect_clicked(move |_| lock.unlock());

        window.set_child(Some(&button));
        window.present();
    }

}

