#[cfg(any(target_os = "android", target_os = "ios"))]
mod app_view;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod ffi;
#[cfg(any(target_os = "android", target_os = "ios"))]
pub use ffi::*;
use app_view::App;

#[allow(unused_variables)]
pub fn create_breakout_app(
) -> App {

}

#[cfg(any(target_os = "android", target_os = "ios"))]
pub(crate) fn change_input(app: &mut App, key_code: KeyCode, state: ButtonState) {

}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[allow(clippy::type_complexity)]
pub(crate) fn close_bevy_window(mut app: Box<App>) {

}
