//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/termios.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// Most architectures have straight copies of the x86 code, with
// varying levels of bug fixes on top. Usually it's a good idea
// to use this generic version instead, but be careful to avoid
// ABI changes.
// New architectures should not provide their own version.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

pub const NCC: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct termio {
    pub /: *mut *mut unsigned short c_iflag; / input mode flags,
    pub /: *mut *mut unsigned short c_oflag; / output mode flags,
    pub /: *mut *mut unsigned short c_cflag; / control mode flags,
    pub /: *mut *mut unsigned short c_lflag; / local mode flags,
    pub /: *mut *mut unsigned char c_line; / line discipline,
    pub /: *mut *mut unsigned char c_cc[NCC]; / control characters,
}

// modem lines
pub const TIOCM_LE: c_uint = 0x001;
pub const TIOCM_DTR: c_uint = 0x002;
pub const TIOCM_RTS: c_uint = 0x004;
pub const TIOCM_ST: c_uint = 0x008;
pub const TIOCM_SR: c_uint = 0x010;
pub const TIOCM_CTS: c_uint = 0x020;
pub const TIOCM_CAR: c_uint = 0x040;
pub const TIOCM_RNG: c_uint = 0x080;
pub const TIOCM_DSR: c_uint = 0x100;

pub const TIOCM_OUT1: c_uint = 0x2000;
pub const TIOCM_OUT2: c_uint = 0x4000;
pub const TIOCM_LOOP: c_uint = 0x8000;
// ioctl (fd, TIOCSERGETLSR, &result) where result may be as below
