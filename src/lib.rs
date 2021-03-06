#![no_std]

#![feature(const_fn)]
#![feature(const_fn_union)]
#![feature(untagged_unions)]

use core::ptr;

#[allow(unions_with_drop_fields)]
#[derive(Copy)]
pub union Slot<T> { pub x: T }

impl<T> Slot<T> {
    #[inline]
    pub const fn new() -> Self {
        #[allow(unions_with_drop_fields)]
        union U<T> { u: (), v: Slot<T> }
        unsafe { U { u: () }.v }
    }

    #[inline]
    pub unsafe fn as_ref(&self) -> &T { &self.x }

    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { &mut self.x }

    #[inline]
    pub unsafe fn unwrap(self) -> T { self.x }
}

impl<T> Clone for Slot<T> {
    #[inline]
    fn clone(&self) -> Self { unsafe { ptr::read(self) } }
}

impl<T> Default for Slot<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}

impl<T> From<T> for Slot<T> {
    #[inline]
    fn from(x: T) -> Self { Slot { x } }
}
