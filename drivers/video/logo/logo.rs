//! Automatically rewritten from C to Rust
//! Source: drivers/video/logo/logo.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Linux logo to be displayed on boot
//
// Copyright (C) 1996 Larry Ewing (lewing@isc.tamu.edu)
// Copyright (C) 1996,1998 Jakub Jelinek (jj@sunsite.mff.cuni.cz)
// Copyright (C) 2001 Greg Banks <gnb@alphalink.com.au>
// Copyright (C) 2001 Jan-Benedict Glaw <jbglaw@lug-owl.de>
// Copyright (C) 2003 Geert Uytterhoeven <geert@linux-m68k.org>
//

    static bool nologo;
    module_param(nologo, bool, 0);
    MODULE_PARM_DESC(nologo, "Disables startup logo");
//
// Logos are located in the initdata, and will be freed in kernel_init.
// Use late_init to mark the logos as freed to prevent any further use.
//
    static bool logos_freed;
#[no_mangle]
unsafe extern "C" fn fb_logo_late_init() -> int __init {
    static int __init fb_logo_late_init(void)
    {
    logos_freed = true;
    return 0;
    }
    late_initcall_sync(fb_logo_late_init);
// logo's are marked __initdata. Use __ref to tell
// modpost that it is intended that this function uses data
// marked __initdata.
//
#[no_mangle]
pub unsafe extern "C" fn fb_find_logo(depth: c_int) -> *const linux_logo  __ref {
    const struct linux_logo * __ref fb_find_logo(int depth)
    {
    const struct linux_logo *logo = core::ptr::null_mut();
    if (nologo || logos_freed)
    return core::ptr::null_mut();

    if (depth >= 1)
    logo = &logo_linux_mono;

    if (depth >= 4)
    logo = &logo_linux_vga16;

    if (depth >= 8)
    logo = &logo_linux_clut224;

    return logo;
    }
    EXPORT_SYMBOL_GPL(fb_find_logo);
