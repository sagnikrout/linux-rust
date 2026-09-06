//! Automatically rewritten from C to Rust
//! Source: drivers/video/fbdev/aty/radeon_backlight.c
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
// Backlight code for ATI Radeon based graphic cards
//
// Copyright (c) 2000 Ani Joshi <ajoshi@kernel.crashing.org>
// Copyright (c) 2003 Benjamin Herrenschmidt <benh@kernel.crashing.org>
// Copyright (c) 2006 Michael Hanselmann <linux-kernel@hansmi.ch>
//

pub const MAX_RADEON_LEVEL: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_bl_privdata {
    pub rinfo: *mut radeonfb_info,
    pub negative: u8,
}

    static int radeon_bl_get_level_brightness(struct radeon_bl_privdata *pdata,
    int level)
    {
    int rlevel;
// Get and convert the value
// No locking of bl_curve since we read a single value
    rlevel = pdata.rinfo.info.bl_curve[level] *
    FB_BACKLIGHT_MAX / MAX_RADEON_LEVEL;
    if (rlevel < 0)
    rlevel = 0;
#[no_mangle]
pub unsafe extern "C" fn if(MAX_RADEON_LEVEL: rlevel >) -> else {
    else if (rlevel > MAX_RADEON_LEVEL)
    rlevel = MAX_RADEON_LEVEL;
    if (pdata.negative)
    rlevel = MAX_RADEON_LEVEL - rlevel;
    return rlevel;
    }
#[no_mangle]
unsafe extern "C" fn radeon_bl_update_status(bd: *mut backlight_device) -> c_int {
    static int radeon_bl_update_status(struct backlight_device *bd)
    {
    struct radeon_bl_privdata *pdata = bl_get_data(bd);
    struct radeonfb_info *rinfo = pdata.rinfo;
    u32 lvds_gen_cntl, tmpPixclksCntl;
    int level;
    if (rinfo.mon1_type != MT_LCD)
    return 0;
// We turn off the LCD completely instead of just dimming the
// backlight. This provides some greater power saving and the display
// is useless without backlight anyway.
//
    level = backlight_get_brightness(bd);
    timer_delete_sync(&rinfo.lvds_timer);
    radeon_engine_idle();
    lvds_gen_cntl = INREG(LVDS_GEN_CNTL);
    if (level > 0) {
    lvds_gen_cntl &= ~LVDS_DISPLAY_DIS;
    if (!(lvds_gen_cntl & LVDS_BLON) || !(lvds_gen_cntl & LVDS_ON)) {
    lvds_gen_cntl |= (rinfo.init_state.lvds_gen_cntl & LVDS_DIGON);
    lvds_gen_cntl |= LVDS_BLON | LVDS_EN;
    OUTREG(LVDS_GEN_CNTL, lvds_gen_cntl);
    lvds_gen_cntl &= ~LVDS_BL_MOD_LEVEL_MASK;
    lvds_gen_cntl |=
    (radeon_bl_get_level_brightness(pdata, level) <<
    LVDS_BL_MOD_LEVEL_SHIFT);
    lvds_gen_cntl |= LVDS_ON;
    lvds_gen_cntl |= (rinfo.init_state.lvds_gen_cntl & LVDS_BL_MOD_EN);
    rinfo.pending_lvds_gen_cntl = lvds_gen_cntl;
    mod_timer(&rinfo.lvds_timer,
    jiffies + msecs_to_jiffies(rinfo.panel_info.pwr_delay));
    } else {
    lvds_gen_cntl &= ~LVDS_BL_MOD_LEVEL_MASK;
    lvds_gen_cntl |=
    (radeon_bl_get_level_brightness(pdata, level) <<
    LVDS_BL_MOD_LEVEL_SHIFT);
    OUTREG(LVDS_GEN_CNTL, lvds_gen_cntl);
    }
    rinfo.init_state.lvds_gen_cntl &= ~LVDS_STATE_MASK;
    rinfo.init_state.lvds_gen_cntl |= rinfo.pending_lvds_gen_cntl
    & LVDS_STATE_MASK;
    } else {
// Asic bug, when turning off LVDS_ON, we have to make sure
    RADEON_PIXCLK_LVDS_ALWAYS_ON bit is off
//
    tmpPixclksCntl = INPLL(PIXCLKS_CNTL);
    if (rinfo.is_mobility || rinfo.is_IGP)
    OUTPLLP(PIXCLKS_CNTL, 0, ~PIXCLK_LVDS_ALWAYS_ONb);
    lvds_gen_cntl &= ~(LVDS_BL_MOD_LEVEL_MASK | LVDS_BL_MOD_EN);
    lvds_gen_cntl |= (radeon_bl_get_level_brightness(pdata, 0) <<
    LVDS_BL_MOD_LEVEL_SHIFT);
    lvds_gen_cntl |= LVDS_DISPLAY_DIS;
    OUTREG(LVDS_GEN_CNTL, lvds_gen_cntl);
    udelay(100);
    lvds_gen_cntl &= ~(LVDS_ON | LVDS_EN);
    OUTREG(LVDS_GEN_CNTL, lvds_gen_cntl);
    lvds_gen_cntl &= ~(LVDS_DIGON);
    rinfo.pending_lvds_gen_cntl = lvds_gen_cntl;
    mod_timer(&rinfo.lvds_timer,
    jiffies + msecs_to_jiffies(rinfo.panel_info.pwr_delay));
    if (rinfo.is_mobility || rinfo.is_IGP)
    OUTPLL(PIXCLKS_CNTL, tmpPixclksCntl);
    }
    rinfo.init_state.lvds_gen_cntl &= ~LVDS_STATE_MASK;
    rinfo.init_state.lvds_gen_cntl |= (lvds_gen_cntl & LVDS_STATE_MASK);
    return 0;
    }
    static const struct backlight_ops radeon_bl_data = {
    .update_status	= radeon_bl_update_status,
    };
#[no_mangle]
pub unsafe extern "C" fn radeonfb_bl_init(rinfo: *mut radeonfb_info) {
    void radeonfb_bl_init(struct radeonfb_info *rinfo)
    {
    struct backlight_properties props;
    struct backlight_device *bd;
    struct radeon_bl_privdata *pdata;
    char name[12];
    if (rinfo.mon1_type != MT_LCD)
    return;

    if (!pmac_has_backlight_type("ati") &&
    !pmac_has_backlight_type("mnca"))
    return;

    pdata = kmalloc_obj(struct radeon_bl_privdata);
    if (!pdata) {
    printk("radeonfb: Memory allocation failed\n");
    goto error;
    }
    snprintf(name, sizeof(name), "radeonbl%d", rinfo.info.node);
    memset(&props, 0, sizeof(struct backlight_properties));
    props.type = BACKLIGHT_RAW;
    props.max_brightness = FB_BACKLIGHT_LEVELS - 1;
    bd = backlight_device_register(name, rinfo.info.device, pdata,
    &radeon_bl_data, &props);
    if (IS_ERR(bd)) {
    rinfo.info.bl_dev = core::ptr::null_mut();
    printk("radeonfb: Backlight registration failed\n");
    goto error;
    }
    pdata.rinfo = rinfo;
// Pardon me for that hack... maybe some day we can figure out in what
// direction backlight should work on a given panel?
//
    pdata.negative =
    (rinfo.family != CHIP_FAMILY_RV200 &&
    rinfo.family != CHIP_FAMILY_RV250 &&
    rinfo.family != CHIP_FAMILY_RV280 &&
    rinfo.family != CHIP_FAMILY_RV350);

    pdata.negative = pdata.negative ||
    of_machine_is_compatible("PowerBook4,3") ||
    of_machine_is_compatible("PowerBook6,3") ||
    of_machine_is_compatible("PowerBook6,5");

    rinfo.info.bl_dev = bd;
    fb_bl_default_curve(rinfo.info, 0,
    63 * FB_BACKLIGHT_MAX / MAX_RADEON_LEVEL,
    217 * FB_BACKLIGHT_MAX / MAX_RADEON_LEVEL);
    bd.props.brightness = bd.props.max_brightness;
    bd.props.power = BACKLIGHT_POWER_ON;
    backlight_update_status(bd);
    printk("radeonfb: Backlight initialized (%s)\n", name);
    return;
    error:
    kfree(pdata);
    return;
    }
#[no_mangle]
pub unsafe extern "C" fn radeonfb_bl_exit(rinfo: *mut radeonfb_info) {
    void radeonfb_bl_exit(struct radeonfb_info *rinfo)
    {
    struct backlight_device *bd = rinfo.info.bl_dev;
    if (bd) {
    struct radeon_bl_privdata *pdata;
    pdata = bl_get_data(bd);
    backlight_device_unregister(bd);
    kfree(pdata);
    rinfo.info.bl_dev = core::ptr::null_mut();
    printk("radeonfb: Backlight unloaded\n");
    }
    }
