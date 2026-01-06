use iced::{
    widget::{button, column, container, row, slider, text, Space, checkbox},
    window, Alignment, Element, Length, Settings, Theme, Subscription, Application, Command,
    time::Duration,
};
use crate::config::Config;
use crate::daemon::r#loop::DaemonState;
use crate::setup;
use crate::gui::style;

pub fn run() -> iced::Result {
    let icon = load_icon();
    
    NvidiaWormhole::run(Settings {
        default_font: iced::Font::DEFAULT,
        window: window::Settings {
            icon,
            platform_specific: window::settings::PlatformSpecific {
                application_id: "com.github.doublepranks.nvidia-wormhole".into(),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Settings::default()
    })
}

fn load_icon() -> Option<window::Icon> {
    let bytes = include_bytes!("../../assets/icon.png");
    let image = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();
    window::icon::from_rgba(rgba, width, height).ok()
}

#[derive(Debug, Clone)]
pub enum Message {
    FanPointChanged(usize, f64),
    OpenLink(String),
    Tick,
    InstallPermissions,
    ToggleAutostart(bool),
}

#[derive(Default)]
pub struct Flags;

pub struct NvidiaWormhole {
    daemon_state: DaemonState,
    config: Config,
    
    // UI State
    current_temp: u32,
    current_speed: u32,
    current_usage: u32,
    gpu_name: String,
    fan_speed_points: [f64; 4],
    
    // Setup State
    has_permissions: bool,
    autostart_enabled: bool,
    setup_message: Option<String>,
}

impl Application for NvidiaWormhole {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = Flags;

    fn new(_flags: Flags) -> (Self, Command<Message>) {
        let has_permissions = setup::check_permissions();
        let autostart_enabled = setup::is_autostart_enabled();
        
        // Get GPU Name
        let gpu_name = crate::hardware::probe::get_gpu_name(0).unwrap_or_else(|_| "Nvidia GPU".to_string());
        
        let config = Config::load().unwrap_or_else(|e| {
            log::warn!("Failed to load config: {}, using defaults", e);
            Config::default()
        });
        
        let fan_speed_points = config.curve_speeds_f64();
        let interval_ms = config.interval_ms;
        
        let daemon_state = DaemonState::new(config.curve.clone());
        // Only start daemon if permissions are already granted
        if has_permissions {
            daemon_state.start(interval_ms);
        }

        (
            Self {
                daemon_state,
                config,
                current_temp: 0,
                current_speed: 0,
                current_usage: 0,
                gpu_name,
                fan_speed_points,
                has_permissions,
                autostart_enabled,
                setup_message: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Nvidia Wormhole")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FanPointChanged(idx, val) => {
                if idx < self.fan_speed_points.len() {
                    self.fan_speed_points[idx] = val;
                    self.config.set_curve_speeds(&self.fan_speed_points);
                    if let Ok(mut curve) = self.daemon_state.curve.lock() {
                        *curve = self.config.curve.clone();
                    }
                    if let Err(e) = self.config.save() {
                        log::error!("Failed to save config: {}", e);
                    }
                }
            }
            Message::OpenLink(url) => {
                let _ = open::that(url);
            }
            Message::Tick => {
                if let Ok(status) = self.daemon_state.status.lock() {
                    self.current_temp = status.current_temp;
                    self.current_speed = status.current_speed;
                    self.current_usage = status.gpu_usage;
                }
                self.has_permissions = setup::check_permissions();
            }
            Message::InstallPermissions => {
                match setup::install_sudoers() {
                    Ok(_) => {
                        self.has_permissions = true;
                        self.setup_message = Some("✓ Permissions installed!".into());
                        // Now that we have permissions, start the daemon
                        if !self.daemon_state.running.load(std::sync::atomic::Ordering::Relaxed) {
                            self.daemon_state.start(self.config.interval_ms);
                        }
                    }
                    Err(e) => {
                        self.setup_message = Some(format!("✗ Failed: {}", e));
                    }
                }
            }
            Message::ToggleAutostart(enabled) => {
                let binary_path = std::env::current_exe()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| "nvidia-wormhole".into());
                
                let result = if enabled {
                    setup::create_autostart_entry(&binary_path)
                } else {
                    setup::remove_autostart_entry()
                };
                
                match result {
                    Ok(_) => {
                        self.autostart_enabled = enabled;
                        self.setup_message = Some(if enabled {
                            "✓ Autostart enabled".into()
                        } else {
                            "✓ Autostart disabled".into()
                        });
                    }
                    Err(e) => {
                        self.setup_message = Some(format!("✗ Failed: {}", e));
                    }
                }
            }
        }
        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick)
    }

    fn view(&self) -> Element<'_, Message> {
        // 1. Permission Warning
        let permission_warning: Element<'_, Message> = if !self.has_permissions {
            container(
                column![
                    text("⚠ Permission Setup Required").size(16),
                    text("nvidia-settings needs sudo access").size(12),
                    button("Install Service Requirements")
                        .on_press(Message::InstallPermissions)
                        .padding(10),
                ]
                .spacing(10)
                .align_items(Alignment::Center)
            )
            .padding(15)
            .style(style::warning_card)
            .width(Length::Fill)
            .into()
        } else {
            Space::with_height(0).into()
        };

        // 2. Dashboard
        let dashboard = row![
            container(
                column![
                    text("GPU TEMP").size(12).style(style::TEXT_DIM),
                    text(format!("{}°C", self.current_temp)).size(36).style(style::NVIDIA_GREEN)
                ].align_items(Alignment::Center)
            )
            .style(style::metric_card)
            .padding(20)
            .width(Length::Fill),
            
            Space::with_width(20),
            
            container(
                column![
                    text("GPU USAGE").size(12).style(style::TEXT_DIM),
                    text(format!("{}%", self.current_usage)).size(36).style(style::NVIDIA_GREEN)
                ].align_items(Alignment::Center)
            )
            .style(style::metric_card)
            .padding(20)
            .width(Length::Fill),

            Space::with_width(20),
            
            container(
                column![
                    text("FAN SPEED").size(12).style(style::TEXT_DIM),
                    text(format!("{}%", self.current_speed)).size(36).style(style::NVIDIA_GREEN)
                ].align_items(Alignment::Center)
            )
            .style(style::metric_card)
            .padding(20)
            .width(Length::Fill),
        ]
        .width(Length::Fill);

        // 3. Curve Editor
        let temps = [30, 50, 70, 85];
        let mut sliders_col = column![
            text("Fan Curve Configuration").size(16)
        ].spacing(15);
        
        for (i, &temp) in temps.iter().enumerate() {
            let val = self.fan_speed_points[i];
            let row_item = row![
                text(format!("{: >3}°C", temp)).width(45).style(style::TEXT_DIM),
                slider(0.0..=100.0, val, move |v| Message::FanPointChanged(i, v))
                    .step(1.0)
                    .width(Length::Fill),
                text(format!("{: >3.0}%", val)).width(45).style(style::NVIDIA_GREEN),
            ]
            .spacing(15)
            .align_items(Alignment::Center);
            
            sliders_col = sliders_col.push(row_item);
        }

        let curve_panel = container(sliders_col)
            .padding(20)
            .width(Length::Fill)
            .style(style::card);

        // 4. Settings
        let autostart_checkbox = checkbox(
            "Start daemon on login",
            self.autostart_enabled,
        ).on_toggle(Message::ToggleAutostart);
        
        let setup_status: Element<'_, Message> = match &self.setup_message {
            Some(msg) => text(msg).size(12).style(style::NVIDIA_GREEN).into(),
            None => Space::with_height(0).into(),
        };

        let settings_panel = container(
            column![
                text("Settings").size(16),
                autostart_checkbox,
                setup_status,
            ]
            .spacing(10)
        )
        .padding(20)
        .width(Length::Fill)
        .style(style::card);

        // 5. Footer
        let footer = container(
            row![
                button(text("⭐ GitHub").size(14))
                    .on_press(Message::OpenLink("https://github.com/Doublepranks/nvidia-wormhole".into()))
                    .padding([8, 16]),
                button(text("☕ Donate").size(14))
                    .on_press(Message::OpenLink("https://www.paypal.com/donate/?hosted_button_id=PWCZCAATEGK3Q".into()))
                    .padding([8, 16]),
                button(text("🐦 X / Twitter").size(14))
                    .on_press(Message::OpenLink("https://x.com/sampantojapa".into()))
                    .padding([8, 16]),
            ]
            .spacing(15)
            .align_items(Alignment::Center)
        )
        .style(style::card)
        .padding(10);

        container(
            column![
                text(&self.gpu_name).size(24).style(style::NVIDIA_GREEN),
                permission_warning,
                Space::with_height(10),
                dashboard,
                Space::with_height(10),
                curve_panel,
                Space::with_height(10),
                settings_panel,
                Space::with_height(Length::Fill),
                footer
            ]
            .spacing(10)
            .padding(30)
            .align_items(Alignment::Center)
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
