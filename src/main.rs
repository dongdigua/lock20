use gtk4 as gtk;
use gtk4::glib::{self, clone, ControlFlow};
use gtk4::gio::prelude::*;
use gtk4::gdk;
use gtk::prelude::*;
use gtk4_session_lock::Instance as SessionLockInstance;
use adw;

use notify_rust::Notification;

use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    if !gtk4_session_lock::is_supported() {
        println!("Session lock not supported");
        return;
    }

    let app = adw::Application::new(
        Some("com.github.wmww.gtk4-layer-shell.session-lock-example"),
        Default::default(),
    );

    app.connect_startup(move |app| {
        let dummy = gtk::ApplicationWindow::builder()
            .application(app)
            .default_width(0)
            .default_height(0)
            .visible(false)   // ah, I still have a window => holds
            .build();
        app.add_window(&dummy);
    });

    app.connect_activate(move |app| schedule_lock(app.clone()));
    app.run();
}

fn schedule_lock(app: adw::Application) {
    // thank you ChatGPT
    glib::timeout_add_seconds_local_once(1190, move || {
        Notification::new()
            .summary("10 seconds remaining before lock")
            .body("Your screen will get locked for 20 seconds to make sure that you relax your eyes. Run twenty -k to stop.")
            .show()
            .unwrap();
        std::thread::sleep(std::time::Duration::from_secs(10));
        do_lock(app.clone());
    });
}

fn do_lock(app: adw::Application) {
    let lock = SessionLockInstance::new();
    lock.connect_unlocked(clone!(
        #[weak] app,
        move |_| {
            for w in app.windows() {
                w.close();
            }
            schedule_lock(app.clone());
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
        let window = gtk::ApplicationWindow::new(&app);
        lock.assign_window_to_monitor(&window, &monitor);

        let label = gtk::Label::default();
        window.set_child(Some(&label));

        let lock_clone = lock.clone();
        let countdown = Rc::new(RefCell::new(20));
        label.set_text("20"); // initial
        let tick = move || {
            let mut secs = *countdown.borrow();
            if secs == 0 {
                lock_clone.unlock();
                return ControlFlow::Break;
            }
            secs -= 1;
            *countdown.borrow_mut() = secs;
            label.set_markup(&format!("<span font='{}'>{}</span>", 30-secs, secs));
            ControlFlow::Continue
        };

        glib::source::timeout_add_seconds_local(1, tick);

        window.present();
    }
}

