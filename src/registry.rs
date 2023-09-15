/*
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};

// Not public API. Used by generated code.
#[doc(hidden)]
pub struct Registry {
  head: AtomicPtr<Node>,
}

#[doc(hidden)]
pub struct Node {
  pub value: &'static dyn ErasedNode,
  pub next: UnsafeCell<Option<&'static Node>>,
}
unsafe impl Send for Node {}

pub trait ErasedNode: Sync {
  unsafe fn submit(&self, node: &'static Node);
}

impl<T: Collect> ErasedNode for T {
  unsafe fn submit(&self, node: &'static Node) {
    T::registry().submit(node);
  }
}

pub trait Collect: Sync + Sized + 'static {
  fn registry() -> &'static Registry;
}

//  pub const new() -> 
//}

*/
