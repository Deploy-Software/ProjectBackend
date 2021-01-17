use crate::store::Store;
use crate::views::nav_bar_view::ActivePage;
use crate::views::nav_bar_view::NavBarView;
use crate::Msg;

use virtual_dom_rs::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

pub struct HomeView {
    store: Rc<RefCell<Store>>,
}

impl HomeView {
    pub fn new(store: Rc<RefCell<Store>>) -> HomeView {
        HomeView { store }
    }
}

impl View for HomeView {
    fn render(&self) -> VirtualNode {
        let nav_bar = NavBarView::new(ActivePage::Home).render();

        let store = Rc::clone(&self.store);

        let click_count = self.store.borrow().click_count();
        let click_count = &*click_count.to_string();

        let click_component = html! { <strong style="font-size: 30px">{ click_count }</strong> };

        html! {
        <div>
          <div class="h-screen flex overflow-hidden bg-white">
            { nav_bar }
            <main
              class="flex-1 relative z-0 overflow-y-auto focus:outline-none"
              tabIndex={0}
            >
              <div class="hidden mt-8 mx-8 sm:block">
                <div>
                  <h2 class="text-gray-500 text-xs font-medium uppercase tracking-wide">
                    Targets
                  </h2>
                  <dl class="mt-5 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                 </dl>
                </div>
                <div class="mt-4">
                  <h2 class="text-gray-500 text-xs font-medium uppercase tracking-wide">
                    Work
                  </h2>
                  <dl class="mt-5 grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-3">
                 </dl>
                </div>
              </div>
            </main>
          </div>
        </div>
          }
    }
}
