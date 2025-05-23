use gtk4 as gtk;
use gtk4::glib::{self, clone, ControlFlow};
use gtk4::gio::prelude::*;
use gtk4::gdk;
use gtk::prelude::*;
use gtk4_session_lock::Instance as SessionLockInstance;
use adw;
use std::time::{Duration,Instant};
use std::thread;
use std::sync::mpsc::sync_channel;

fn main() {
    if !gtk4_session_lock::is_supported() {
        println!("Session lock not supported")
    }

    init();
}

fn init() {
    thread::sleep(Duration::from_secs(5));
    lock();
}

fn lock() {
    let app = adw::Application::new(
        Some("com.github.wmww.gtk4-layer-shell.session-lock-example"),
        Default::default(),
    );


    app.connect_activate(activate);
    app.run();
}

fn activate(app: &adw::Application) {
    let lock = SessionLockInstance::new();
    lock.connect_unlocked(clone!(
        #[weak] app,
        move |_| {
            for w in app.windows() {
                w.close();
            }
        }
    ));


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

        let label = gtk::Label::default();

        window.set_child(Some(&label));
        let tick = move || {
            let time = format!("{:?}", Instant::now());
            label.set_text(&time);

            // we could return glib::ControlFlow::Break to stop our clock after this tick
            // glib::ControlFlow::Break
        };

        let lock = lock.clone();
        let do_unlock = clone!(
            #[weak] lock,
            move || lock.unlock()
        );
        glib::source::timeout_add_seconds_local_once(5, do_unlock);
        // glib::source::timeout_add_seconds_local_once(1, tick);

        window.present();
    }
}

