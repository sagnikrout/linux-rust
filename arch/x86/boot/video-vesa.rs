//! Automatically rewritten from C to Rust
//! Source: arch/x86/boot/video-vesa.c
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
// VESA text modes
//

// VESA information
    static struct vesa_general_info vginfo;
    static struct vesa_mode_info vminfo;
    static __videocard video_vesa;

    static void vesa_store_mode_params_graphics(void);

    static inline void vesa_store_mode_params_graphics(void) {}

#[no_mangle]
unsafe extern "C" fn vesa_probe() -> c_int {
    static int vesa_probe(void)
    {
    struct biosregs ireg, oreg;
    u16 mode;
    addr_t mode_ptr;
    struct mode_info *mi;
    let mut nmodes: c_int = 0;
    video_vesa.modes = GET_HEAP(struct mode_info, 0);
    initregs(&ireg);
    ireg.ax = 0x4f00;
    ireg.di = (size_t)&vginfo;
    intcall(0x10, &ireg, &oreg);
    if (oreg.ax != 0x004f ||
    vginfo.signature != VESA_MAGIC ||
    vginfo.version < 0x0102)
    return 0;	/* Not present */
    set_fs(vginfo.video_mode_ptr.seg);
    mode_ptr = vginfo.video_mode_ptr.off;
    while ((mode = rdfs16(mode_ptr)) != 0xffff) {
    mode_ptr += 2;
    if (!heap_free(sizeof(struct mode_info)))
    break;	/* Heap full, can't save mode info */
    if (mode & ~0x1ff)
    continue;
    memset(&vminfo, 0, sizeof(vminfo)); /* Just in case... */
    ireg.ax = 0x4f01;
    ireg.cx = mode;
    ireg.di = (size_t)&vminfo;
    intcall(0x10, &ireg, &oreg);
    if (oreg.ax != 0x004f)
    continue;
    if ((vminfo.mode_attr & 0x15) == 0x05) {
// Text Mode, TTY BIOS supported,
    supported by hardware */
    mi = GET_HEAP(struct mode_info, 1);
    mi.mode  = mode + VIDEO_FIRST_VESA;
    mi.depth = 0; /* text */
    mi.x     = vminfo.h_res;
    mi.y     = vminfo.v_res;
    nmodes++;
    } else if ((vminfo.mode_attr & 0x99) == 0x99 &&
    (vminfo.memory_layout == 4 ||
    vminfo.memory_layout == 6) &&
    vminfo.memory_planes == 1) {

// Graphics mode, color, linear frame buffer
    supported.  Only register the mode if
    if framebuffer is configured, however,
    otherwise the user will be left without a screen. */
    mi = GET_HEAP(struct mode_info, 1);
    mi.mode = mode + VIDEO_FIRST_VESA;
    mi.depth = vminfo.bpp;
    mi.x = vminfo.h_res;
    mi.y = vminfo.v_res;
    nmodes++;

    }
    }
    return nmodes;
    }
#[no_mangle]
unsafe extern "C" fn vesa_set_mode(mode: *mut mode_info) -> c_int {
    static int vesa_set_mode(struct mode_info *mode)
    {
    struct biosregs ireg, oreg;
    int is_graphic;
    let mut vesa_mode: u16 = mode.mode - VIDEO_FIRST_VESA;
    memset(&vminfo, 0, sizeof(vminfo)); /* Just in case... */
    initregs(&ireg);
    ireg.ax = 0x4f01;
    ireg.cx = vesa_mode;
    ireg.di = (size_t)&vminfo;
    intcall(0x10, &ireg, &oreg);
    if (oreg.ax != 0x004f)
    return -1;
    if ((vminfo.mode_attr & 0x15) == 0x05) {
// It's a supported text mode
    is_graphic = 0;

    } else if ((vminfo.mode_attr & 0x99) == 0x99) {
// It's a graphics mode with linear frame buffer
    is_graphic = 1;
    vesa_mode |= 0x4000; /* Request linear frame buffer */

    } else {
    return -1;	/* Invalid mode */
    }
    initregs(&ireg);
    ireg.ax = 0x4f02;
    ireg.bx = vesa_mode;
    intcall(0x10, &ireg, &oreg);
    if (oreg.ax != 0x004f)
    return -1;
    graphic_mode = is_graphic;
    if (!is_graphic) {
// Text mode
    force_x = mode.x;
    force_y = mode.y;
    do_restore = 1;
    } else {
// Graphics mode
    vesa_store_mode_params_graphics();
    }
    return 0;
    }

// Switch DAC to 8-bit mode
#[no_mangle]
unsafe extern "C" fn vesa_dac_set_8bits() {
    static void vesa_dac_set_8bits(void)
    {
    struct biosregs ireg, oreg;
    let mut dac_size: u8 = 6;
// If possible, switch the DAC to 8-bit mode
    if (vginfo.capabilities & 1) {
    initregs(&ireg);
    ireg.ax = 0x4f08;
    ireg.bh = 0x08;
    intcall(0x10, &ireg, &oreg);
    if (oreg.ax == 0x004f)
    dac_size = oreg.bh;
    }
// Set the color sizes to the DAC size, and offsets to 0
    boot_params.screen_info.red_size   = dac_size;
    boot_params.screen_info.green_size = dac_size;
    boot_params.screen_info.blue_size  = dac_size;
    boot_params.screen_info.rsvd_size  = dac_size;
    boot_params.screen_info.red_pos    = 0;
    boot_params.screen_info.green_pos  = 0;
    boot_params.screen_info.blue_pos   = 0;
    boot_params.screen_info.rsvd_pos   = 0;
    }
// Save the VESA protected mode info
#[no_mangle]
unsafe extern "C" fn vesa_store_pm_info() {
    static void vesa_store_pm_info(void)
    {
    struct biosregs ireg, oreg;
    initregs(&ireg);
    ireg.ax = 0x4f0a;
    intcall(0x10, &ireg, &oreg);
    if (oreg.ax != 0x004f)
    return;
    boot_params.screen_info.vesapm_seg = oreg.es;
    boot_params.screen_info.vesapm_off = oreg.di;
    }
//
// Save video mode parameters for graphics mode
//
#[no_mangle]
unsafe extern "C" fn vesa_store_mode_params_graphics() {
    static void vesa_store_mode_params_graphics(void)
    {
// Tell the kernel we're in VESA graphics mode
    boot_params.screen_info.orig_video_isVGA = VIDEO_TYPE_VLFB;
// Mode parameters
    boot_params.screen_info.vesa_attributes = vminfo.mode_attr;
    boot_params.screen_info.lfb_linelength = vminfo.logical_scan;
    boot_params.screen_info.lfb_width = vminfo.h_res;
    boot_params.screen_info.lfb_height = vminfo.v_res;
    boot_params.screen_info.lfb_depth = vminfo.bpp;
    boot_params.screen_info.pages = vminfo.image_planes;
    boot_params.screen_info.lfb_base = vminfo.lfb_ptr;
    memcpy(&boot_params.screen_info.red_size,
    &vminfo.rmask, 8);
// General parameters
    boot_params.screen_info.lfb_size = vginfo.total_memory;
    if (vminfo.bpp <= 8)
    vesa_dac_set_8bits();
    vesa_store_pm_info();
    }
//
// Save EDID information for the kernel; this is invoked, separately,
// after mode-setting.
//
#[no_mangle]
pub unsafe extern "C" fn vesa_store_edid() {
    void vesa_store_edid(void)
    {

    struct biosregs ireg, oreg;
// Apparently used as a nonsense token...
    memset(&boot_params.edid_info, 0x13, sizeof(boot_params.edid_info));
    if (vginfo.version < 0x0200)
    return;		/* EDID requires VBE 2.0+ */
    initregs(&ireg);
    ireg.ax = 0x4f15;		/* VBE DDC */
// ireg.bx = 0x0000; */		/* Report DDC capabilities
// ireg.cx = 0;	*/		/* Controller 0
    ireg.es = 0;			/* ES:DI must be 0 by spec */
    intcall(0x10, &ireg, &oreg);
    if (oreg.ax != 0x004f)
    return;		/* No EDID */
// BH = time in seconds to transfer EDD information
// BL = DDC level supported
    ireg.ax = 0x4f15;		/* VBE DDC */
    ireg.bx = 0x0001;		/* Read EDID */
// ireg.cx = 0; */		/* Controller 0
// ireg.dx = 0;	*/		/* EDID block number
    ireg.es = ds();
    ireg.di =(size_t)&boot_params.edid_info; /* (ES:)Pointer to block */
    intcall(0x10, &ireg, &oreg);

    }

    static __videocard video_vesa =
    {
    .card_name	= "VESA",
    .probe		= vesa_probe,
    .set_mode	= vesa_set_mode,
    .xmode_first	= VIDEO_FIRST_VESA,
    .xmode_n	= 0x200,
    };
