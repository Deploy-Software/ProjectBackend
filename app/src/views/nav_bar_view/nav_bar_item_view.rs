use virtual_dom_rs::prelude::*;

pub struct NavBarItemView {
    path: &'static str,
    text: &'static str,
}

impl NavBarItemView {
    pub fn new(path: &'static str, text: &'static str) -> NavBarItemView {
        NavBarItemView { path, text }
    }
}

impl View for NavBarItemView {
    fn render(&self) -> VirtualNode {
        html! {
         <a
           href={self.path}
           class="text-gray-600 hover:bg-gray-50 hover:text-gray-900 group flex items-center px-2 py-2 text-sm font-medium rounded-md"
         >
           { self.text }
         </a>
        }
    }
}
