//! Android platform hooks used by the native Horizon touch layer.
//! Keyboard and document callbacks are wired in the Android activity integration.
use horizon_client::{gamepad::Pulse, osk::Output};

pub fn active() -> bool { false }
pub fn hide() {}
pub fn show(_text: &str) {}
pub fn show_multiline(_text: &str) {}
pub fn poll() -> Vec<Output> { Vec::new() }
pub fn vibrate(_pulse: Pulse) {}
pub fn cancel_vibration() {}
pub fn open_documents(_id: &str) -> bool { false }
pub fn performance_power() -> serde_json::Value { serde_json::Value::Null }
pub fn field_at(p: [f32; 2]) -> bool {
    horizon_client::widgets::last_targets().iter().any(|t| {
        let [x, y, w, h] = t.rect;
        t.kind == horizon_client::widgets::Kind::Field
            && (x..x + w).contains(&p[0])
            && (y..y + h).contains(&p[1])
    })
}
pub fn capture_mouse(_capture: bool) -> bool { false }
pub fn clipboard_get() -> Option<String> { None }
pub fn clipboard_set(_text: &str) {}
pub fn copy_login_code(_text: &str) -> bool { false }
pub fn open_url(_url: &str) -> bool { false }
pub fn finish() {}
