//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kbd_kern.h
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

//
// kbd->xxx contains the VC-local things (flag settings etc..)
//
// Note: externally visible are LED_SCR, LED_NUM, LED_CAP defined in kd.h
// The code in KDGETLED / KDSETLED depends on the internal and
// external order being the same.
//
// Note: lockstate is used as index in the array key_map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kbd_struct {
    pub lockstate: c_uchar,
// 8 modifiers - the names do not have any meaning at all;

    pub /: *mut *mut unsigned char slockstate; / for `sticky' Shift, Ctrl, etc.,
    pub ledmode:1: c_uchar,

    pub /: *mut *mut unsigned char ledflagstate:4; / flags, not lights,
    pub default_ledflagstate:4: c_uchar,

    pub /: *mut *mut unsigned char kbdmode:3; / one 3-bit value,

    pub modeflags:5: c_uchar,

}

extern "C" {
    pub fn kbd_init() -> c_int;
}
extern "C" {
    pub fn setledstate(kbd: *mut kbd_struct, led: c_uint);
}
extern "C" {
    pub fn void(led: *mut *mut kbd_ledfunc)(unsigned int) -> extern;
}
extern "C" {
    pub fn set_console(nr: c_int) -> c_int;
}
extern "C" {
    pub fn schedule_console_callback();
}

pub const BRL_UC_ROW: c_uint = 0x2800;
// keyboard.c
extern "C" {
    pub fn vt_set_leds_compute_shiftstate();
}
// defkeymap.c
