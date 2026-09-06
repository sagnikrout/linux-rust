//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/nvidia/nv_backlight.c
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
// Backlight code for nVidia based graphic cards
//
// Copyright 2004 Antonino Daplas <adaplas@pol.net>
// Copyright (c) 2006 Michael Hanselmann <linux-kernel@hansmi.ch>
//

// We do not have any information about which values are allowed, thus
// we used safe values.
//
pub const MIN_LEVEL: c_uint = 0x158;
pub const MAX_LEVEL: c_uint = 0x534;

    static int nvidia_bl_get_level_brightness(struct nvidia_par *par,
    int level)
    {
    struct fb_info *info = pci_get_drvdata(par.pci_dev);
    int nlevel;
// Get and convert the value
// No locking of bl_curve since we read a single value
    nlevel = MIN_LEVEL + info.bl_curve[level] * LEVEL_STEP;
    if (nlevel < 0)
    nlevel = 0;
#[no_mangle]
pub unsafe extern "C" fn if(MIN_LEVEL: nlevel <) -> else {
    else if (nlevel < MIN_LEVEL)
    nlevel = MIN_LEVEL;
#[no_mangle]
pub unsafe extern "C" fn if(MAX_LEVEL: nlevel >) -> else {
    else if (nlevel > MAX_LEVEL)
    nlevel = MAX_LEVEL;
    return nlevel;
    }
#[no_mangle]
unsafe extern "C" fn nvidia_bl_update_status(bd: *mut backlight_device) -> c_int {
    static int nvidia_bl_update_status(struct backlight_device *bd)
    {
    struct nvidia_par *par = bl_get_data(bd);
    u32 tmp_pcrt, tmp_pmc, fpcontrol;
    let mut level: c_int = backlight_get_brightness(bd);
    if (!par.FlatPanel)
    return 0;
    tmp_pmc = NV_RD32(par.PMC, 0x10F0) & 0x0000FFFF;
    tmp_pcrt = NV_RD32(par.PCRTC0, 0x081C) & 0xFFFFFFFC;
    fpcontrol = NV_RD32(par.PRAMDAC, 0x0848) & 0xCFFFFFCC;
    if (level > 0) {
    tmp_pcrt |= 0x1;
    tmp_pmc |= (1 << 31); /* backlight bit */
    tmp_pmc |= nvidia_bl_get_level_brightness(par, level) << 16;
    fpcontrol |= par.fpSyncs;
    } else
    fpcontrol |= 0x20000022;
    NV_WR32(par.PCRTC0, 0x081C, tmp_pcrt);
    NV_WR32(par.PMC, 0x10F0, tmp_pmc);
    NV_WR32(par.PRAMDAC, 0x848, fpcontrol);
    return 0;
    }
    static const struct backlight_ops nvidia_bl_ops = {
    .update_status	= nvidia_bl_update_status,
    };
#[no_mangle]
pub unsafe extern "C" fn nvidia_bl_init(par: *mut nvidia_par) {
    void nvidia_bl_init(struct nvidia_par *par)
    {
    struct backlight_properties props;
    struct fb_info *info = pci_get_drvdata(par.pci_dev);
    struct backlight_device *bd;
    char name[12];
    if (!par.FlatPanel)
    return;

    if (!machine_is(powermac) ||
    !pmac_has_backlight_type("mnca"))
    return;

    snprintf(name, sizeof(name), "nvidiabl%d", info.node);
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = FB_BACKLIGHT_LEVELS - 1;
    bd = backlight_device_register(name, info.device, par, &nvidia_bl_ops,
    &props);
    if (IS_ERR(bd)) {
    info.bl_dev = core::ptr::null_mut();
    printk(KERN_WARNING "nvidia: Backlight registration failed\n");
    goto error;
    }
    info.bl_dev = bd;
    fb_bl_default_curve(info, 0,
    0x158 * FB_BACKLIGHT_MAX / MAX_LEVEL,
    0x534 * FB_BACKLIGHT_MAX / MAX_LEVEL);
    bd.props.brightness = bd.props.max_brightness;
    bd.props.power = BACKLIGHT_POWER_ON;
    backlight_update_status(bd);
    printk("nvidia: Backlight initialized (%s)\n", name);
    error:
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn nvidia_bl_exit(par: *mut nvidia_par) {
    void nvidia_bl_exit(struct nvidia_par *par)
    {
    struct fb_info *info = pci_get_drvdata(par.pci_dev);
    struct backlight_device *bd = info.bl_dev;
    backlight_device_unregister(bd);
    printk("nvidia: Backlight unloaded\n");
    }
