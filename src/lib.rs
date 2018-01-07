#![no_std]

#![feature(untagged_unions)]

use core::mem;

#[allow(unions_with_drop_fields)]
#[derive(Clone, Copy)]
pub union Slot<T> { x: T }

impl<T> Slot<T> {
    #[inline]
    pub fn new() -> Self { unsafe { mem::uninitialized() } }

    #[inline]
    pub unsafe fn get(self) -> T { self.x }

    #[inline]
    pub unsafe fn as_ref(&self) -> &T { &self.x }

    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { &mut self.x }
}

impl<T> From<T> for Slot<T> {
    #[inline]
    fn from(x: T) -> Self { Slot { x } }
}
