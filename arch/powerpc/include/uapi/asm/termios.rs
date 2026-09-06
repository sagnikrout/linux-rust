//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/termios.h
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


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// Liberally adapted from alpha/termios.h.  In particular, the c_cc[]
// fields have been reordered so that termio & termios share the
// common subset in the same order (for brain dead programs that don't
// know or care about the differences).
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sgttyb {
    pub sg_ispeed: c_char,
    pub sg_ospeed: c_char,
    pub sg_erase: c_char,
    pub sg_kill: c_char,
    pub sg_flags: c_short,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tchars {
    pub t_intrc: c_char,
    pub t_quitc: c_char,
    pub t_startc: c_char,
    pub t_stopc: c_char,
    pub t_eofc: c_char,
    pub t_brkc: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ltchars {
    pub t_suspc: c_char,
    pub t_dsuspc: c_char,
    pub t_rprntc: c_char,
    pub t_flushc: c_char,
    pub t_werasc: c_char,
    pub t_lnextc: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct winsize {
    pub ws_row: c_ushort,
    pub ws_col: c_ushort,
    pub ws_xpixel: c_ushort,
    pub ws_ypixel: c_ushort,
}

pub const NCC: c_int = 10;
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

// c_cc characters
pub const _VINTR: c_int = 0;
pub const _VQUIT: c_int = 1;
pub const _VERASE: c_int = 2;
pub const _VKILL: c_int = 3;
pub const _VEOF: c_int = 4;
pub const _VMIN: c_int = 5;
pub const _VEOL: c_int = 6;
pub const _VTIME: c_int = 7;
pub const _VEOL2: c_int = 8;
pub const _VSWTC: c_int = 9;
