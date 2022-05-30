#![allow(unused_mut)]
//! mut_immut is a small crate with a function "change" which allows you to change value of a variable without mutating it.

/// Semi-safe useless function.
pub fn change<T>(src: &T, to: T) {
    unsafe {
        let mut ptr: usize = (src as *const T) as usize;
        *(ptr as *mut T) = to;
    }
}

/// Getting &mut from &.
pub fn get_mut<T>(src: &T) -> &mut T {
    unsafe {
        let mut ptr: usize = (src as *const T) as usize;
        return &mut (*(ptr as *mut T))
    }
}