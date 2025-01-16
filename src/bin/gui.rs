use external_app_sync::{config::Config, launch_agent::LaunchAgent};
use iced::widget::{button, checkbox, column, container, row, text, Column};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};
use std::path::PathBuf;
use tracing::error;

pub fn main() -> iced::Result {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    AppSync::run(Settings::default())
}

#[derive(Debug)]
struct AppSync {
    external_path: Option<PathBuf>,
    notify_on_disconnect: bool,
    daemon_running: bool,
    log_messages: Vec<String>,
    launch_agent: LaunchAgent,
}

#[derive(Debug, Clone)]
enum Message {
    SelectFolder,
    FolderSelected(Option<PathBuf>),
    ToggleNotifications(bool),
    ToggleDaemon,
    ToggleLaunchAgent,
}

impl Application for AppSync {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let config = Config::load();
        
        // Get the path to our daemon executable
        let current_exe = std::env::current_exe().expect("Failed to get executable path");
        let daemon_path = current_exe.parent().unwrap().join("external_app_sync_daemon");
        let launch_agent = LaunchAgent::new(daemon_path);
        
        let initial_state = AppSync {
            external_path: config.as_ref().map(|c| c.external_apps_path.clone()),
            notify_on_disconnect: config.as_ref().map_or(false, |c| c.notify_on_disconnect),
            daemon_running: false,
            log_messages: Vec::new(),
            launch_agent,
        };
        
        (initial_state, Command::none())
    }

    fn title(&self) -> String {
        String::from("External App Sync")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SelectFolder => {
                // Show native folder picker
                Command::perform(
                    async {
                        let dialog = native_dialog::FileDialog::new()
                            .set_location("~/")
                            .add_filter("Apps Folder", &["app"])
                            .show_open_single_dir();
                        
                        match dialog {
                            Ok(Some(path)) => Some(path),
                            _ => None,
                        }
                    },
                    Message::FolderSelected,
                )
            }
            Message::FolderSelected(Some(path)) => {
                self.external_path = Some(path.clone());
                
                // Save configuration
                let config = Config {
                    external_apps_path: path,
                    notify_on_disconnect: self.notify_on_disconnect,
                };
                
                if let Err(e) = config.save() {
                    error!("Failed to save config: {}", e);
                }
                
                Command::none()
            }
            Message::FolderSelected(None) => Command::none(),
            Message::ToggleNotifications(enabled) => {
                self.notify_on_disconnect = enabled;
                
                if let Some(path) = &self.external_path {
                    let config = Config {
                        external_apps_path: path.clone(),
                        notify_on_disconnect: enabled,
                    };
                    
                    if let Err(e) = config.save() {
                        error!("Failed to save config: {}", e);
                    }
                }
                
                Command::none()
            }
            Message::ToggleDaemon => {
                self.daemon_running = !self.daemon_running;
                Command::none()
            }
            Message::ToggleLaunchAgent => {
                if self.launch_agent.is_installed() {
                    if let Err(e) = self.launch_agent.uninstall() {
                        error!("Failed to uninstall launch agent: {}", e);
                    }
                } else {
                    if let Err(e) = self.launch_agent.install() {
                        error!("Failed to install launch agent: {}", e);
                    }
                }
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let title = text("External App Sync")
            .size(24);
        
        let folder_section = column![
            text("External Apps Folder:").size(16),
            row![
                text(self.external_path
                    .as_ref()
                    .map_or("No folder selected".to_string(), |p| p.to_string_lossy().into_owned())),
                button("Browse").on_press(Message::SelectFolder),
            ].spacing(10),
        ].spacing(10);
        
        let settings_section = column![
            checkbox(
                "Notify when external drive disconnects",
                self.notify_on_disconnect,
                Message::ToggleNotifications,
            ),
            button(if self.daemon_running {
                "Stop Monitoring"
            } else {
                "Start Monitoring"
            })
            .on_press(Message::ToggleDaemon),
            button(if self.launch_agent.is_installed() {
                "Disable Auto-Start"
            } else {
                "Enable Auto-Start"
            })
            .on_press(Message::ToggleLaunchAgent),
        ].spacing(10);
        
        let log_section = column(
            self.log_messages
                .iter()
                .map(|msg| text(msg).into())
                .collect(),
        )
        .spacing(5);
        
        let content = Column::new()
            .push(title)
            .push(folder_section)
            .push(settings_section)
            .push(text("Log:").size(16))
            .push(log_section)
            .spacing(20)
            .padding(20);
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .into()
    }
}
