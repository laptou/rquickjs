use core::{cell::Cell, sync::atomic::AtomicBool};
#[cfg(feature = "std")]
use std::sync::Once;

use crate::qjs;

/// The type of identifier of class
#[cfg_attr(feature = "doc-cfg", doc(cfg(feature = "classes")))]
pub struct ClassId {
    id: Cell<qjs::JSClassID>,
    #[cfg(feature = "std")]
    once: Once,
    #[cfg(not(feature = "std"))]
    is_init: AtomicBool,
}

unsafe impl Send for ClassId {}
// #[cfg(feature = "std")]
unsafe impl Sync for ClassId {}

impl ClassId {
    /// Create a new class id.
    pub const fn new() -> Self {
        Self {
            id: Cell::new(0),
            #[cfg(feature = "std")]
            once: Once::new(),
            #[cfg(not(feature = "std"))]
            is_init: AtomicBool::new(false),
        }
    }

    /// Get the class Id.
    /// Will initialize itself if it has not done so.
    pub fn get(&self) -> qjs::JSClassID {
        self.init();
        self.id.get()
    }

    /// Initialize the class ID.
    /// Can be called multiple times but will only be initialized once.
    fn init(&self) {
        #[cfg(feature = "std")]
        self.once.call_once(|| {
            let mut id = 0;
            unsafe { qjs::JS_NewClassID(&mut id) };
            self.id.set(id);
        });

        #[cfg(not(feature = "std"))]
        {
            let was_init = self.is_init.fetch_or(true, core::sync::atomic::Ordering::SeqCst);
            if was_init { return }

            let mut id = 0;
            unsafe { qjs::JS_NewClassID(&mut id) };
            self.id.set(id);
        }
    }
}
