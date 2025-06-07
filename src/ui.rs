use gtk4 as gtk;
use gtk::prelude::*;
use gtk::glib::{self, clone, ControlFlow};
use gtk::gdk;
use gtk4_session_lock::Instance as SessionLockInstance;

use std::rc::Rc;
use std::cell::RefCell;

pub fn do_lock(app: gtk::Application) {
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
