//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/platforms/8xx/mpc8xx.h
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
// Prototypes, etc. for the Freescale MPC8xx embedded cpu chips
// May need to be cleaned as the port goes on ...
//
// Copyright (C) 2008 Jochen Friedrich <jochen@scram.de>
//
// This file is licensed under the terms of the GNU General Public License
// version 2. This program is licensed "as is" without any warranty of any
// kind, whether express or implied.
//

// Macro flag: #define __MPC8xx_H
extern "C" {
    pub fn mpc8xx_restart(cmd: *mut c_char) -> void __noreturn;
}
extern "C" {
    pub fn mpc8xx_calibrate_decr();
}
extern "C" {
    pub fn mpc8xx_set_rtc_time(tm: *mut rtc_time) -> c_int;
}
extern "C" {
    pub fn mpc8xx_get_rtc_time(tm: *mut rtc_time);
}
extern "C" {
    pub fn mpc8xx_get_irq() -> c_uint;
}
