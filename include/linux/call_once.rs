//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/call_once.h
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


pub const ONCE_NOT_STARTED: c_int = 0;
pub const ONCE_RUNNING: c_int = 1;
pub const ONCE_COMPLETED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct once {
    pub state: core::sync::atomic::AtomicI32,
    pub lock: mutex,
}

//
// call_once - Ensure a function has been called exactly once
//
// @once: Tracking struct
// @cb: Function to be called
//
// If @once has never completed successfully before, call @cb and, if
// it returns a zero or positive value, mark @once as completed.  Return
// the value returned by @cb
//
// If @once has completed successfully before, return 0.
//
// The call to @cb is implicitly surrounded by a mutex, though for
// efficiency the * function avoids taking it after the first call.
//
// Pairs with atomic_set_release() below.
