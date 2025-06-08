use gtk4 as gtk;
use gtk::prelude::*;
use gtk::glib::{self, ControlFlow};
use gtk::gdk;

use notify_rust::{Notification, Hint};

use nix::sys::signal::{self, Signal, SigHandler};

use std::sync::atomic::{AtomicBool, Ordering};

mod ui;
mod log;

static SKIP: AtomicBool = AtomicBool::new(false);

extern "C" fn handle_sigusr1(signal: nix::libc::c_int) {
    let signal = Signal::try_from(signal).unwrap();
    SKIP.store(signal == Signal::SIGUSR1, Ordering::Relaxed);
}

fn main() {
    if !gtk4_session_lock::is_supported() {
        println!("Session lock not supported");
        return;
    }

    let handler = SigHandler::Handler(handle_sigusr1);
    unsafe { signal::signal(Signal::SIGUSR1, handler) }.unwrap();

    let app = gtk::Application::new(
        Some("com.github.dongdigua.lock20"),
        Default::default(),
    );

    app.connect_startup(move |app| {
        load_css();
        // thank you ChatGPT
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

fn load_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(".background{background-color: black;} label{color:white;}");
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn schedule_lock(app: gtk::Application) {
    // for testing: must > 31s, otherwise the sleep below will block the display before unlocking
    glib::timeout_add_seconds_local(1190, move || {
        std::thread::spawn(move || {
            // FIXME: the thread will wait until the button is clicked or the notification is closed
            Notification::new()
                .summary("10 seconds remaining before lock")
                .body("Your screen will get locked for 20 seconds to make sure that you relax your eyes.")
                .action("skip", "skip")
                .timeout(std::time::Duration::from_millis(5000))
                .show()
                .unwrap()
                .wait_for_action(|action| match action {
                    "skip" => SKIP.store(true, Ordering::Relaxed),
                    "__closed" => debug!("closed"), // deprecate in 5.0.0
                    _ => debug!("what?")
                });
            debug!("notification closed");
        });

        std::thread::sleep(std::time::Duration::from_millis(10000));

        if SKIP.load(Ordering::Relaxed) {
            println!("skipped");
            SKIP.store(false, Ordering::Relaxed);
        } else {
            ui::do_lock(app.clone());
        }
        ControlFlow::Continue
    });
}

