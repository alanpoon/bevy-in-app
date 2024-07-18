use super::{AppView, AppViewWindow};
use std::collections::HashMap;
#[derive(Debug)]
pub struct Entity{

}
#[derive(Debug)]
pub struct WindowWrapper<T>(T);
impl <T>WindowWrapper<T>{
    fn new(app_view:T)->Self{
        WindowWrapper(app_view)
    }
}
#[derive(Debug, Default)]
pub struct AppViews {
    views: HashMap<super::WindowId, AppViewWindow>,
    entity_to_window_id: HashMap<Entity, super::WindowId>,
}

impl AppViews {
    pub fn create_window(
        &mut self,
        #[cfg(target_os = "ios")] view_obj: super::IOSViewObj,
        #[cfg(target_os = "android")] view_obj: super::AndroidViewObj,
        entity: Entity,
    ) -> AppViewWindow {
        let app_view = AppViewWindow(WindowWrapper::new(AppView::new(view_obj)));
        app_view
    }

    /// Get the AppView that is associated with our entity.
    pub fn get_view(&self, entity: Entity) -> Option<&AppViewWindow> {
        None
    }

    /// This should mostly just be called when the window is closing.
    pub fn remove_view(&mut self, entity: Entity) -> Option<AppViewWindow> {
        None
    }
}
