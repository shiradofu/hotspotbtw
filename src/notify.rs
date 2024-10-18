use notify_rust::Notification;

#[cfg(target_os = "macos")]
pub fn init() {
    let app_id = notify_rust::get_bundle_identifier_or_default("terminal");
    let _ = notify_rust::set_application(&app_id);
}

#[cfg(all(unix, not(target_os = "macos")))]
pub fn init() {}

#[cfg(target_os = "windows")]
pub fn init() {}

pub fn battery_percentage(device_name: &str, percentage: i8) -> Result<(), String> {
    let main_message = format!("{} battery reaches {}%", device_name, percentage);

    Notification::new()
        .summary(&main_message)
        .body("Let's take the battery out!")
        .show()
        .map_err(|_| format!("failed to send notification: {}", main_message))
        .map(|_| Ok(()))?
}
