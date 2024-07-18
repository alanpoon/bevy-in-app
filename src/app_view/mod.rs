use std::ops::Deref;
use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
pub(crate) struct WindowId(Uuid);

impl WindowId {
    pub fn new() -> Self {
        WindowId(Uuid::new_v4())
    }
}
pub struct App{}
#[cfg_attr(target_os = "ios", path = "ios.rs")]
#[cfg_attr(target_os = "android", path = "android.rs")]
mod view;
pub use view::*;

mod app_views;
use app_views::AppViews;

#[derive(Clone, Debug)]
pub(crate) struct SendSyncWrapper<T>(pub(crate) T);

unsafe impl<T> Send for SendSyncWrapper<T> {}
unsafe impl<T> Sync for SendSyncWrapper<T> {}

// 封装 AppViewWindow
#[derive(Debug)]
pub struct AppViewWindow(pub(crate) WindowWrapper<AppView>);

impl Deref for AppViewWindow {
    type Target = AppView;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct AppViewPlugin;

impl Plugin for AppViewPlugin {
    fn build(&self, app: &mut App) {

    }
}

#[allow(unused, clippy::type_complexity)]
pub fn create_bevy_window(app: &mut App) {

}
