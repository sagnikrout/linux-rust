//! Automatically rewritten from C to Rust
//! Source: lib/cmpxchg-emu.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Emulated 1-byte cmpxchg operation for architectures lacking direct
// support for this size.  This is implemented in terms of 4-byte cmpxchg
// operations.
//
// Copyright (C) 2024 Paul E. McKenney.
//

    union u8_32 {
    u8 b[4];
    u32 w;
    };
// Emulate one-byte cmpxchg() in terms of 4-byte cmpxchg.
#[no_mangle]
pub unsafe extern "C" fn cmpxchg_emu_u8(p: *mut volatile u8, old: uintptr_t, new: uintptr_t) -> uintptr_t {
    uintptr_t cmpxchg_emu_u8(volatile u8 *p, uintptr_t old, uintptr_t new)
    {
    u32 *p32 = (u32 *)(((uintptr_t)p) & ~0x3);
    let mut i: c_int = ((uintptr_t)p) & 0x3;
    union u8_32 old32;
    union u8_32 new32;
    u32 ret;
    ret = READ_ONCE(*p32);
    do {
    old32.w = ret;
    if (old32.b[i] != old)
    return old32.b[i];
    new32.w = old32.w;
    new32.b[i] = new;
    instrument_atomic_read_write(p, 1);
    ret = data_race(cmpxchg(p32, old32.w, new32.w)); // Overridden above.
    } while (ret != old32.w);
    return old;
    }
    EXPORT_SYMBOL_GPL(cmpxchg_emu_u8);
