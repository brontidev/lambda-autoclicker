// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod settings;

use crate::settings::{
    AppSettingsData, ClickButton, ClickQuantity, FixedIntervalOptions, IntervalMode, MousePos,
    RepeatMode,
};
use rand::RngExt;
use std::{fs, path::PathBuf, sync::{Arc, Mutex}, time::Duration};
use std::sync::atomic::{AtomicBool, Ordering};

use thread_priority::{ set_current_thread_priority as set_thread_priority, ThreadPriority::Max };


use enigo::{Button, Enigo, Mouse, Settings};
use tauri::{AppHandle, Emitter, Listener, Manager, WebviewWindowBuilder};

#[cfg(target_os = "macos")]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

#[cfg(target_os = "windows")]
use window_vibrancy::apply_blur;

use state_sync::{InvalidationEvent, Revision, SnapshotEnvelope};

use spin_sleep::SpinSleeper;

#[derive(Default)]
struct AppSettings {
    revision: Revision,
    data: AppSettingsData,
}

struct AppState {
    enigo: Arc<Mutex<Enigo>>,
    settings: AppSettings,
    is_clicking: Arc<AtomicBool>,
}

type TauriState<'a> = tauri::State<'a, Mutex<AppState>>;

#[tauri::command]
fn toggle_clicking(app: AppHandle, state: TauriState) {
    // Clone the atomic and toggle it outside the lock
    let state_ref = state.clone();
    let is_clicking_arc = {
        let binding = state_ref.lock().unwrap();
        binding.is_clicking.clone()
    };

    let new_val = !is_clicking_arc.load(Ordering::Relaxed);
    is_clicking_arc.store(new_val, Ordering::SeqCst);
    app.emit("clicking-update", new_val).unwrap();

    if new_val {
        tauri::async_runtime::spawn_blocking(move || {
            let _ = set_thread_priority(Max);
            start_clicking(app);
        });
    }
}

fn fixed_interval_duration(options: &FixedIntervalOptions) -> Duration {
    Duration::from_millis(
        u64::from(options.hours) * 3_600_000
            + u64::from(options.minutes) * 60_000
            + u64::from(options.seconds) * 1_000
            + u64::from(options.milliseconds),
    )
}

fn interval_duration(settings: &AppSettingsData) -> Duration {
    match settings.interval_mode {
        IntervalMode::Fixed => fixed_interval_duration(&settings.interval_fixed_options),
        IntervalMode::Random => {
            let min = u64::from(settings.interval_rand_options.min);
            let max = u64::from(settings.interval_rand_options.max);
            let mut rng = rand::rng();
            let delay = if min <= max {
                rng.random_range(min..=max)
            } else {
                min
            };

            Duration::from_millis(delay)
        }
    }
}

fn start_clicking(app: AppHandle) {
    // Grab a snapshot of settings and clone Arc handles so the loop doesn't lock global state
    let (settings, enigo_arc, is_clicking_arc) = {
        let binding = app.state::<Mutex<AppState>>();
        let binding = binding.lock().unwrap();
        (
            binding.settings.data.clone(),
            binding.enigo.clone(),
            binding.is_clicking.clone(),
        )
    };

    let MousePos { x, y } = settings.mouse_position;
    let button: Button = match settings.click_button {
        ClickButton::Left => Button::Left,
        ClickButton::Middle => Button::Middle,
        ClickButton::Right => Button::Right,
    };

    let mut clicks: u32 = 0;
    let spin_sleeper = SpinSleeper::default();

    loop {
        // fast atomic check to see if we should stop
        if !is_clicking_arc.load(Ordering::Relaxed) {
            break;
        }

        {
            let mut enigo = enigo_arc.lock().unwrap();

            if settings.set_mouse_position {
                enigo.move_mouse(x, y, enigo::Coordinate::Abs).unwrap();
            }

            enigo.button(button, enigo::Direction::Click).unwrap();
            if settings.click_quantity == ClickQuantity::Double {
                enigo.button(button, enigo::Direction::Click).unwrap();
            }
        }

        clicks += 1;
        if settings.repeat_mode == RepeatMode::Count {
            if settings.repeat_count <= clicks {
                break;
            }
        }

        let delay = interval_duration(&settings);
        if !delay.is_zero() {
            spin_sleeper.sleep(delay);
        }
    }

    // clear the atomic and notify UI
    is_clicking_arc.store(false, Ordering::SeqCst);
    app.emit("clicking-update", false).unwrap();
}

#[tauri::command]
fn get_settings(state: TauriState) -> SnapshotEnvelope<AppSettingsData> {
    let state = state.lock().unwrap();
    SnapshotEnvelope {
        revision: state.settings.revision.to_string(),
        data: state.settings.data.clone(),
    }
}

#[tauri::command]
fn update_settings(
    app: tauri::AppHandle,
    state: TauriState,
    settings: AppSettingsData,
) -> Result<SnapshotEnvelope<AppSettingsData>, String> {
    let mut state = state.lock().unwrap();
    state.settings.data = settings;
    state.settings.revision = state.settings.revision.next();

    app.emit(
        "settings:invalidated",
        InvalidationEvent {
            topic: "settings".to_string(),
            revision: state.settings.revision.to_string(),
        },
    )
    .map_err(|e| e.to_string())?;

    Ok(SnapshotEnvelope {
        revision: state.settings.revision.to_string(),
        data: state.settings.data.clone(),
    })
}

fn get_settings_path(app: &AppHandle) -> PathBuf {
    app.path().app_data_dir().unwrap().join("settings.json")
}

#[tauri::command]
fn save_settings(
    app: AppHandle,
    snapshot: SnapshotEnvelope<AppSettingsData>,
) -> Result<(), String> {
    let path = get_settings_path(&app);
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(&path, serde_json::to_string_pretty(&snapshot).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
fn load_settings(app: AppHandle) -> Option<SnapshotEnvelope<AppSettingsData>> {
    let path = get_settings_path(&app);
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<SnapshotEnvelope<AppSettingsData>>(&s).ok())
}

#[tauri::command]
fn open_picker(app: AppHandle) {
    let app_window = app.get_webview_window("main").unwrap();
    let picker_window =
        WebviewWindowBuilder::new(&app, "picker", tauri::WebviewUrl::App("app/picker".into()))
            .transparent(true)
            .accept_first_mouse(true)
            .decorations(false)
            .position(0.0, 0.0)
            .closable(false)
            .visible(false)
            .always_on_top(true)
            .minimizable(false)
            .resizable(false)
            .build()
            .unwrap();

    let app_w: tauri::WebviewWindow = app_window.clone();
    picker_window.once("tauri://webview-created", move |_| {
        let _ = app_w.minimize();
    });

    let app_w: tauri::WebviewWindow = app_window.clone();
    picker_window.once("tauri://destroyed", move |_| {
        let _ = app_w.unminimize();
    });

    let app_for_pick = app.clone();
    picker_window.listen("picked", move |_e| {
        let state = app_for_pick.state::<Mutex<AppState>>();
        let mut binding = state.lock().unwrap();
        let (x, y) = {
            let enigo = binding.enigo.lock().unwrap();
            enigo.location().unwrap()
        };

        binding.settings.data.mouse_position = MousePos { x, y };
        binding.settings.revision = binding.settings.revision.next();

        let _ = app_for_pick.emit(
            "settings:invalidated",
            InvalidationEvent {
                topic: "settings".to_string(),
                revision: binding.settings.revision.to_string(),
            },
        );
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            load_settings,
            save_settings,
            open_picker,
            toggle_clicking
        ])
        .setup(|app| {
            let enigo = Enigo::new(&Settings::default()).or(Err("Failed to load enigo"))?;
            app.manage(Mutex::new(AppState {
                enigo: Arc::new(Mutex::new(enigo)),
                settings: AppSettings::default(),
                is_clicking: Arc::new(AtomicBool::new(false)),
            }));

            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
                .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
