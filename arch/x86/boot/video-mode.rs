//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/video-mode.c
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
// Copyright 2007-2008 rPath, Inc. - All Rights Reserved
//
// -----------------------------------------------------------------------
//
// arch/i386/boot/video-mode.c
//
// Set the video mode.  This is separated out into a different
// file in order to be shared with the ACPI wakeup code.
//

//
// Common variables
//
    int adapter;		/* 0=CGA/MDA/HGC, 1=EGA, 2=VGA+ */
    int force_x, force_y;	/* Don't query the BIOS for cols/rows */
    int do_restore;		/* Screen contents changed during mode flip */
    int graphic_mode;	/* Graphic mode with linear frame buffer */
// Probe the video drivers and have them generate their mode lists.
#[no_mangle]
pub unsafe extern "C" fn probe_cards(unsafe: c_int) {
    void probe_cards(int unsafe)
    {
    struct card_info *card;
    static u8 probed[2];
    if (probed[unsafe])
    return;
    probed[unsafe] = 1;
    for (card = video_cards; card < video_cards_end; card++) {
    if (card.unsafe == unsafe) {
    if (card.probe)
    card.nmodes = card.probe();
    else
    card.nmodes = 0;
    }
    }
    }
// Test if a mode is defined
#[no_mangle]
pub unsafe extern "C" fn mode_defined(mode: u16) -> c_int {
    int mode_defined(u16 mode)
    {
    struct card_info *card;
    struct mode_info *mi;
    int i;
    for (card = video_cards; card < video_cards_end; card++) {
    mi = card.modes;
    for (i = 0; i < card.nmodes; i++, mi++) {
    if (mi.mode == mode)
    return 1;
    }
    }
    return 0;
    }
// Set mode (without recalc)
#[no_mangle]
unsafe extern "C" fn raw_set_mode(mode: u16, real_mode: *mut u16) -> c_int {
    static int raw_set_mode(u16 mode, u16 *real_mode)
    {
    int nmode, i;
    struct card_info *card;
    struct mode_info *mi;
// Drop the recalc bit if set
    mode &= ~VIDEO_RECALC;
// Scan for mode based on fixed ID, position, or resolution
    nmode = 0;
    for (card = video_cards; card < video_cards_end; card++) {
    mi = card.modes;
    for (i = 0; i < card.nmodes; i++, mi++) {
    let mut visible: c_int = mi.x || mi.y;
    if ((mode == nmode && visible) ||
    mode == mi.mode ||
    mode == (mi.y << 8)+mi.x) {
// real_mode = mi->mode;
    return card.set_mode(mi);
    }
    if (visible)
    nmode++;
    }
    }
// Nothing found?  Is it an "exceptional" (unprobed) mode?
    for (card = video_cards; card < video_cards_end; card++) {
    if (mode >= card.xmode_first &&
    mode < card.xmode_first+card.xmode_n) {
    struct mode_info mix;
// real_mode = mix.mode = mode;
    mix.x = mix.y = 0;
    return card.set_mode(&mix);
    }
    }
// Otherwise, failure...
    return -1;
    }
//
// Recalculate the vertical video cutoff (hack!)
//
#[no_mangle]
unsafe extern "C" fn vga_recalc_vertical() {
    static void vga_recalc_vertical(void)
    {
    unsigned int font_size, rows;
    u16 crtc;
    u8 pt, ov;
    set_fs(0);
    font_size = rdfs8(0x485); /* BIOS: font size (pixels) */
    rows = force_y ? force_y : rdfs8(0x484)+1; /* Text rows */
    rows *= font_size;	/* Visible scan lines */
    rows--;			/* ... minus one */
    crtc = vga_crtc();
    pt = in_idx(crtc, 0x11);
    pt &= ~0x80;		/* Unlock CR0-7 */
    out_idx(pt, crtc, 0x11);
    out_idx((u8)rows, crtc, 0x12); /* Lower height register */
    ov = in_idx(crtc, 0x07); /* Overflow register */
    ov &= 0xbd;
    ov |= (rows >> (8-1)) & 0x02;
    ov |= (rows >> (9-6)) & 0x40;
    out_idx(ov, crtc, 0x07);
    }
// Set mode (with recalc if specified)
#[no_mangle]
pub unsafe extern "C" fn set_mode(mode: u16) -> c_int {
    int set_mode(u16 mode)
    {
    int rv;
    u16 real_mode;
// Very special mode numbers...
    if (mode == VIDEO_CURRENT_MODE)
    return 0;	/* Nothing to do... */
#[no_mangle]
pub unsafe extern "C" fn if(NORMAL_VGA: mode ==) -> else {
    else if (mode == NORMAL_VGA)
    mode = VIDEO_80x25;
#[no_mangle]
pub unsafe extern "C" fn if(EXTENDED_VGA: mode ==) -> else {
    else if (mode == EXTENDED_VGA)
    mode = VIDEO_8POINT;
    rv = raw_set_mode(mode, &real_mode);
    if (rv)
    return rv;
    if (mode & VIDEO_RECALC)
    vga_recalc_vertical();
// Save the canonical mode number for the kernel, not
    an alias, size specification or menu position */

    boot_params.hdr.vid_mode = real_mode;

    return 0;
    }
