//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/termios_internal.h
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


// SPDX-License-Identifier: GPL-2.0

// intr=^C		quit=^\		erase=del	kill=^U
//

// Macro flag: #define INIT_C_CC_VDSUSP_EXTRA

extern "C" {
    pub fn user_termio_to_kernel_termios(: *mut ktermios, : *mut termio __user) -> c_int;
}
extern "C" {
    pub fn kernel_termios_to_user_termio(: *mut termio __user, : *mut ktermios) -> c_int;
}

extern "C" {
    pub fn user_termios_to_kernel_termios(: *mut ktermios, : *mut termios2 __user) -> c_int;
}
extern "C" {
    pub fn kernel_termios_to_user_termios(: *mut termios2 __user, : *mut ktermios) -> c_int;
}
extern "C" {
    pub fn user_termios_to_kernel_termios_1(: *mut ktermios, : *mut termios __user) -> c_int;
}
extern "C" {
    pub fn kernel_termios_to_user_termios_1(: *mut termios __user, : *mut ktermios) -> c_int;
}

extern "C" {
    pub fn user_termios_to_kernel_termios(: *mut ktermios, : *mut termios __user) -> c_int;
}
extern "C" {
    pub fn kernel_termios_to_user_termios(: *mut termios __user, : *mut ktermios) -> c_int;
}

