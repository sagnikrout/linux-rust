//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/video-bios.c
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
// -*- linux-c -*- -------------------------------------------------------
//
// Copyright (C) 1991, 1992 Linus Torvalds
// Copyright 2007 rPath, Inc. - All Rights Reserved
// Copyright 2009 Intel Corporation; author H. Peter Anvin
//
// -----------------------------------------------------------------------
//
// Standard video BIOS modes
//
// We have two options for this; silent and scanned.
//

    static __videocard video_bios;
// Set a conventional BIOS mode
    static int set_bios_mode(u8 mode);
#[no_mangle]
unsafe extern "C" fn bios_set_mode(mi: *mut mode_info) -> c_int {
    static int bios_set_mode(struct mode_info *mi)
    {
    return set_bios_mode(mi.mode - VIDEO_FIRST_BIOS);
    }
#[no_mangle]
unsafe extern "C" fn set_bios_mode(mode: u8) -> c_int {
    static int set_bios_mode(u8 mode)
    {
    struct biosregs ireg, oreg;
    u8 new_mode;
    initregs(&ireg);
    ireg.al = mode;		/* AH=0x00 Set Video Mode */
    intcall(0x10, &ireg, core::ptr::null_mut());
    ireg.ah = 0x0f;		/* Get Current Video Mode */
    intcall(0x10, &ireg, &oreg);
    do_restore = 1;		/* Assume video contents were lost */
// Not all BIOSes are clean with the top bit
    new_mode = oreg.al & 0x7f;
    if (new_mode == mode)
    return 0;	/* Mode change OK */

    if (new_mode != boot_params.screen_info.orig_video_mode) {
// Mode setting failed, but we didn't end up where we
    started.  That's bad.  Try to revert to the original
    video mode. */
    ireg.ax = boot_params.screen_info.orig_video_mode;
    intcall(0x10, &ireg, core::ptr::null_mut());
    }

    return -1;
    }
#[no_mangle]
unsafe extern "C" fn bios_probe() -> c_int {
    static int bios_probe(void)
    {
    u8 mode;

    let mut saved_mode: u8 = 0x03;

    let mut saved_mode: u8 = boot_params.screen_info.orig_video_mode;

    u16 crtc;
    struct mode_info *mi;
    let mut nmodes: c_int = 0;
    if (adapter != ADAPTER_EGA && adapter != ADAPTER_VGA)
    return 0;
    set_fs(0);
    crtc = vga_crtc();
    video_bios.modes = GET_HEAP(struct mode_info, 0);
    for (mode = 0x14; mode <= 0x7f; mode++) {
    if (!heap_free(sizeof(struct mode_info)))
    break;
    if (mode_defined(VIDEO_FIRST_BIOS+mode))
    continue;
    if (set_bios_mode(mode))
    continue;
// Try to verify that it's a text mode.
// Attribute Controller: make graphics controller disabled
    if (in_idx(0x3c0, 0x10) & 0x01)
    continue;
// Graphics Controller: verify Alpha addressing enabled
    if (in_idx(0x3ce, 0x06) & 0x01)
    continue;
// CRTC cursor location low should be zero(?)
    if (in_idx(crtc, 0x0f))
    continue;
    mi = GET_HEAP(struct mode_info, 1);
    mi.mode = VIDEO_FIRST_BIOS+mode;
    mi.depth = 0;	/* text */
    mi.x = rdfs16(0x44a);
    mi.y = rdfs8(0x484)+1;
    nmodes++;
    }
    set_bios_mode(saved_mode);
    return nmodes;
    }
    static __videocard video_bios =
    {
    .card_name	= "BIOS",
    .probe		= bios_probe,
    .set_mode	= bios_set_mode,
    .unsafe		= 1,
    .xmode_first	= VIDEO_FIRST_BIOS,
    .xmode_n	= 0x80,
    };
