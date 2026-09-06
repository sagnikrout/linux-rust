//! Automatically rewritten from C to Rust
//! Source: tools/lib/ctype.c
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
// linux/lib/ctype.c
//
// Copyright (C) 1991, 1992  Linus Torvalds
//

    const unsigned char _ctype[] = {
    _C,_C,_C,_C,_C,_C,_C,_C,				/* 0-7 */
    _C,_C|_S,_C|_S,_C|_S,_C|_S,_C|_S,_C,_C,			/* 8-15 */
    _C,_C,_C,_C,_C,_C,_C,_C,				/* 16-23 */
    _C,_C,_C,_C,_C,_C,_C,_C,				/* 24-31 */
    _S|_SP,_P,_P,_P,_P,_P,_P,_P,				/* 32-39 */
    _P,_P,_P,_P,_P,_P,_P,_P,				/* 40-47 */
    _D,_D,_D,_D,_D,_D,_D,_D,				/* 48-55 */
    _D,_D,_P,_P,_P,_P,_P,_P,				/* 56-63 */
    _P,_U|_X,_U|_X,_U|_X,_U|_X,_U|_X,_U|_X,_U,		/* 64-71 */
    _U,_U,_U,_U,_U,_U,_U,_U,				/* 72-79 */
    _U,_U,_U,_U,_U,_U,_U,_U,				/* 80-87 */
    _U,_U,_U,_P,_P,_P,_P,_P,				/* 88-95 */
    _P,_L|_X,_L|_X,_L|_X,_L|_X,_L|_X,_L|_X,_L,		/* 96-103 */
    _L,_L,_L,_L,_L,_L,_L,_L,				/* 104-111 */
    _L,_L,_L,_L,_L,_L,_L,_L,				/* 112-119 */
    _L,_L,_L,_P,_P,_P,_P,_C,				/* 120-127 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,			/* 128-143 */
    0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,			/* 144-159 */
    _S|_SP,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,	/* 160-175 */
    _P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,_P,	/* 176-191 */
    _U,_U,_U,_U,_U,_U,_U,_U,_U,_U,_U,_U,_U,_U,_U,_U,	/* 192-207 */
    _U,_U,_U,_U,_U,_U,_U,_P,_U,_U,_U,_U,_U,_U,_U,_L,	/* 208-223 */
    _L,_L,_L,_L,_L,_L,_L,_L,_L,_L,_L,_L,_L,_L,_L,_L,	/* 224-239 */
    _L,_L,_L,_L,_L,_L,_L,_P,_L,_L,_L,_L,_L,_L,_L,_L};	/* 240-255 */
