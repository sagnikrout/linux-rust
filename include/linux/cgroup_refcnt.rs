//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cgroup_refcnt.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


//
// css_get - obtain a reference on the specified css
// @css: target css
//
// The caller must already have a reference.
//
// css_get_many - obtain references on the specified css
// @css: target css
// @n: number of references to get
//
// The caller must already have a reference.
//
// css_tryget - try to obtain a reference on the specified css
// @css: target css
//
// Obtain a reference on @css unless it already has reached zero and is
// being released.  This function doesn't care whether @css is on or
// offline.  The caller naturally needs to ensure that @css is accessible
// but doesn't have to be holding a reference on it - IOW, RCU protected
// access is good enough for this function.  Returns %true if a reference
// count was successfully obtained; %false otherwise.
//
extern "C" {
    pub fn percpu_ref_tryget(_arg: &css->refcnt) -> return;
}
//
// css_tryget_online - try to obtain a reference on the specified css if online
// @css: target css
//
// Obtain a reference on @css if it's online.  The caller naturally needs
// to ensure that @css is accessible but doesn't have to be holding a
// reference on it - IOW, RCU protected access is good enough for this
// function.  Returns %true if a reference count was successfully obtained;
// %false otherwise.
//
extern "C" {
    pub fn percpu_ref_tryget_live(_arg: &css->refcnt) -> return;
}
//
// css_put - put a css reference
// @css: target css
//
// Put a reference obtained via css_get() and css_tryget_online().
//
// css_put_many - put css references
// @css: target css
// @n: number of references to put
//
// Put references obtained via css_get() and css_tryget_online().
//
