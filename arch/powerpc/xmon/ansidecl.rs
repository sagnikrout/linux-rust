//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/xmon/ansidecl.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// ANSI and traditional C compatibility macros
//
// ANSI and traditional C compatibility macros
extern "C" {
    pub fn PARAMS(_arg: (int, _arg: char)) -> static int foo;
}
extern "C" {
    pub fn EXFUN(_arg: foo, _arg: (int, _arg: char)) -> static int;
}
extern "C" {
    pub fn PARAMS(DOTS): *mut *mut (CONST char format) -> int printf;
}
//
pub const _ANSIDECL_H: c_int = 1;
// Every source file includes this file,
// LINTLIBRARY

// All known AIX compilers implement these things (but don't always

pub const ANSI_PROTOTYPES: c_int = 1;

// Macro flag: #define	NOARGS
// Macro flag: #define	CONST

// Macro flag: #define	const

// Macro flag: #define	VOLATILE
// Macro flag: #define	SIGNED
// Macro flag: #define	DOTS

