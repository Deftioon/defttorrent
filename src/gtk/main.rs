use adw::prelude::*;
use adw::Application;
use gtk::glib::{self};
use std::sync::{Arc, Mutex};
use glib::MainContext;

mod ui {
    use super::*;
    use gtk::{Box, Button, Label, ListBox, Orientation, ProgressBar};

    #[derive(Default)]
    pub struct TorrentState {
        pub progress: Arc<Mutex<f64>>,
        pub active: Arc<Mutex<bool>>,
    }

    pub struct MainWindow {
        pub window: adw::ApplicationWindow,
        pub torrent_list: ListBox,
        pub status_label: Label,
        pub start_stop_btn: Button,
        pub state: TorrentState,
    }

    impl MainWindow {
        pub fn new(app: &Application) -> Arc<Self> {
            let window = adw::ApplicationWindow::builder()
                .application(app)
                .default_width(800)
                .default_height(600)
                .title("Rust Torrent Client")
                .build();

            let header = adw::HeaderBar::new();
            let content = Box::builder()
                .orientation(Orientation::Vertical)
                .spacing(12)
                .build();

            let torrent_list = ListBox::builder()
                .selection_mode(gtk::SelectionMode::None)
                .css_classes(["boxed-list"])
                .build();

            let add_btn = Button::builder()
                .icon_name("document-open-symbolic")
                .tooltip_text("Add Torrent")
                .build();

            let start_stop_btn = Button::builder()
                .icon_name("media-playback-start-symbolic")
                .tooltip_text("Start Download")
                .build();

            let status_label = Label::new(None);
            let progress_bar = ProgressBar::builder()
                .margin_top(12)
                .margin_bottom(12)
                .margin_start(12)
                .margin_end(12)
                .build();

            header.pack_start(&add_btn);
            header.pack_end(&start_stop_btn);

            content.append(&header);
            content.append(&torrent_list);
            content.append(&progress_bar);
            content.append(&status_label);

            window.set_content(Some(&content));

            Arc::new(Self {
                window,
                torrent_list,
                status_label,
                start_stop_btn,
                state: TorrentState::default(),
            })
        }

        pub fn add_torrent_entry(&self, name: &str) {
            let row = adw::ActionRow::builder()
                .title(name)
                .subtitle("0.0% - 0 KiB/s")
                .build();
            self.torrent_list.append(&row);
        }

        pub fn update_progress(&self, progress: f64) {
            let progress = progress.clamp(0.0, 1.0);
            self.status_label
                .set_text(&format!("Downloading: {:.1}%", progress * 100.0));
        }
    }
}

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.example.TorrentClient")
        .build();

    app.connect_activate(|app| {
        let main_win = ui::MainWindow::new(app);
        let state = Arc::new(ui::TorrentState {
            progress: Arc::clone(&main_win.state.progress),
            active: Arc::clone(&main_win.state.active),
        });

        // Use weak reference to avoid circular dependencies
        let main_win_weak = Arc::downgrade(&main_win);
        main_win.start_stop_btn.connect_clicked(move |btn| {
            let mut active = main_win.state.active.lock().unwrap();
            *active = !*active;
            
            let icon = if *active { 
                "media-playback-stop-symbolic" 
            } else { 
                "media-playback-start-symbolic" 
            };
            btn.set_icon_name(icon);
        });

        let (sender, receiver) = glib::MainContext::channel::<f64>(glib::Priority::DEFAULT);
                
        std::thread::spawn(move || {
            let mut progress = 0.0;
            loop {
                std::thread::sleep(std::time::Duration::from_secs(1));
                let active = *state.active.lock().unwrap();
                if active && progress < 1.0 {
                    progress += 0.01;
                    if let Err(e) = sender.send(progress) {
                        eprintln!("Error sending progress: {}", e);
                        break;
                    }
                }
            }
        });

        let main_win_weak = Arc::downgrade(&main_win);
        receiver.attach(None, move |progress| {
            if let Some(main_win) = main_win_weak.upgrade() {
                main_win.update_progress(progress);
                *main_win.state.progress.lock().unwrap() = progress;
            }
            glib::ControlFlow::Continue
        });

        main_win.add_torrent_entry("Ubuntu ISO");
        main_win.add_torrent_entry("Rust Documentation");
        main_win.add_torrent_entry("Example File");

        main_win.window.present();
    });

    app.run()
}