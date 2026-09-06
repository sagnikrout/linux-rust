//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/boot/gamecube.c
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
//
// arch/powerpc/boot/gamecube.c
//
// Nintendo GameCube bootwrapper support
// Copyright (C) 2004-2009 The GameCube Linux Team
// Copyright (C) 2008,2009 Albert Herranz
//

    BSS_STACK(8192);
#[no_mangle]
pub unsafe extern "C" fn platform_init(r3: c_ulong, r4: c_ulong, r5: c_ulong) {
    void platform_init(unsigned long r3, unsigned long r4, unsigned long r5)
    {
    let mut heapsize: u32 = 16*1024*1024 - (u32)_end;
    simple_alloc_init(_end, heapsize, 32, 64);
    fdt_init(_dtb_start);
    if (ug_probe())
    console_ops.write = ug_console_write;
    }
