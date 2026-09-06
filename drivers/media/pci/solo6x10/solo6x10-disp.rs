//! Automatically rewritten from C to Rust
//! Source: drivers/media/pci/solo6x10/solo6x10-disp.c
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
// Copyright (C) 2010-2013 Bluecherry, LLC <https://www.bluecherrydvr.com>
//
// Original author:
// Ben Collins <bcollins@ubuntu.com>
//
// Additional work by:
// John Brooks <john.brooks@bluecherry.net>
//

pub const SOLO_VCLK_DELAY: c_int = 3;
pub const SOLO_PROGRESSIVE_VSIZE: c_int = 1024;
pub const SOLO_MOT_THRESH_W: c_int = 64;
pub const SOLO_MOT_THRESH_H: c_int = 64;
pub const SOLO_MOT_THRESH_SIZE: c_int = 8192;

pub const SOLO_MOT_FLAG_SIZE: c_int = 1024;

#[no_mangle]
unsafe extern "C" fn solo_vin_config(solo_dev: *mut solo_dev) {
    static void solo_vin_config(struct solo_dev *solo_dev)
    {
    solo_dev.vin_hstart = 8;
    solo_dev.vin_vstart = 2;
    solo_reg_write(solo_dev, SOLO_SYS_VCLK,
    SOLO_VCLK_SELECT(2) |
    SOLO_VCLK_VIN1415_DELAY(SOLO_VCLK_DELAY) |
    SOLO_VCLK_VIN1213_DELAY(SOLO_VCLK_DELAY) |
    SOLO_VCLK_VIN1011_DELAY(SOLO_VCLK_DELAY) |
    SOLO_VCLK_VIN0809_DELAY(SOLO_VCLK_DELAY) |
    SOLO_VCLK_VIN0607_DELAY(SOLO_VCLK_DELAY) |
    SOLO_VCLK_VIN0405_DELAY(SOLO_VCLK_DELAY) |
    SOLO_VCLK_VIN0203_DELAY(SOLO_VCLK_DELAY) |
    SOLO_VCLK_VIN0001_DELAY(SOLO_VCLK_DELAY));
    solo_reg_write(solo_dev, SOLO_VI_ACT_I_P,
    SOLO_VI_H_START(solo_dev.vin_hstart) |
    SOLO_VI_V_START(solo_dev.vin_vstart) |
    SOLO_VI_V_STOP(solo_dev.vin_vstart +
    solo_dev.video_vsize));
    solo_reg_write(solo_dev, SOLO_VI_ACT_I_S,
    SOLO_VI_H_START(solo_dev.vout_hstart) |
    SOLO_VI_V_START(solo_dev.vout_vstart) |
    SOLO_VI_V_STOP(solo_dev.vout_vstart +
    solo_dev.video_vsize));
    solo_reg_write(solo_dev, SOLO_VI_ACT_P,
    SOLO_VI_H_START(0) |
    SOLO_VI_V_START(1) |
    SOLO_VI_V_STOP(SOLO_PROGRESSIVE_VSIZE));
    solo_reg_write(solo_dev, SOLO_VI_CH_FORMAT,
    SOLO_VI_FD_SEL_MASK(0) | SOLO_VI_PROG_MASK(0));
// On 6110, initialize mozaic darkness strength
    if (solo_dev.type == SOLO_DEV_6010)
    solo_reg_write(solo_dev, SOLO_VI_FMT_CFG, 0);
    else
    solo_reg_write(solo_dev, SOLO_VI_FMT_CFG, 16 << 22);
    solo_reg_write(solo_dev, SOLO_VI_PAGE_SW, 2);
    if (solo_dev.video_type == SOLO_VO_FMT_TYPE_NTSC) {
    solo_reg_write(solo_dev, SOLO_VI_PB_CONFIG,
    SOLO_VI_PB_USER_MODE);
    solo_reg_write(solo_dev, SOLO_VI_PB_RANGE_HV,
    SOLO_VI_PB_HSIZE(858) | SOLO_VI_PB_VSIZE(246));
    solo_reg_write(solo_dev, SOLO_VI_PB_ACT_V,
    SOLO_VI_PB_VSTART(4) |
    SOLO_VI_PB_VSTOP(4 + 240));
    } else {
    solo_reg_write(solo_dev, SOLO_VI_PB_CONFIG,
    SOLO_VI_PB_USER_MODE | SOLO_VI_PB_PAL);
    solo_reg_write(solo_dev, SOLO_VI_PB_RANGE_HV,
    SOLO_VI_PB_HSIZE(864) | SOLO_VI_PB_VSIZE(294));
    solo_reg_write(solo_dev, SOLO_VI_PB_ACT_V,
    SOLO_VI_PB_VSTART(4) |
    SOLO_VI_PB_VSTOP(4 + 288));
    }
    solo_reg_write(solo_dev, SOLO_VI_PB_ACT_H, SOLO_VI_PB_HSTART(16) |
    SOLO_VI_PB_HSTOP(16 + 720));
    }
#[no_mangle]
unsafe extern "C" fn solo_vout_config_cursor(dev: *mut solo_dev) {
    static void solo_vout_config_cursor(struct solo_dev *dev)
    {
    int i;
// Load (blank) cursor bitmap mask (2bpp)
    for (i = 0; i < 20; i++)
    solo_reg_write(dev, SOLO_VO_CURSOR_MASK(i), 0);
    solo_reg_write(dev, SOLO_VO_CURSOR_POS, 0);
    solo_reg_write(dev, SOLO_VO_CURSOR_CLR,
    (0x80 << 24) | (0x80 << 16) | (0x10 << 8) | 0x80);
    solo_reg_write(dev, SOLO_VO_CURSOR_CLR2, (0xe0 << 8) | 0x80);
    }
#[no_mangle]
unsafe extern "C" fn solo_vout_config(solo_dev: *mut solo_dev) {
    static void solo_vout_config(struct solo_dev *solo_dev)
    {
    solo_dev.vout_hstart = 6;
    solo_dev.vout_vstart = 8;
    solo_reg_write(solo_dev, SOLO_VO_FMT_ENC,
    solo_dev.video_type |
    SOLO_VO_USER_COLOR_SET_NAV |
    SOLO_VO_USER_COLOR_SET_NAH |
    SOLO_VO_NA_COLOR_Y(0) |
    SOLO_VO_NA_COLOR_CB(0) |
    SOLO_VO_NA_COLOR_CR(0));
    solo_reg_write(solo_dev, SOLO_VO_ACT_H,
    SOLO_VO_H_START(solo_dev.vout_hstart) |
    SOLO_VO_H_STOP(solo_dev.vout_hstart +
    solo_dev.video_hsize));
    solo_reg_write(solo_dev, SOLO_VO_ACT_V,
    SOLO_VO_V_START(solo_dev.vout_vstart) |
    SOLO_VO_V_STOP(solo_dev.vout_vstart +
    solo_dev.video_vsize));
    solo_reg_write(solo_dev, SOLO_VO_RANGE_HV,
    SOLO_VO_H_LEN(solo_dev.video_hsize) |
    SOLO_VO_V_LEN(solo_dev.video_vsize));
// Border & background colors
    solo_reg_write(solo_dev, SOLO_VO_BORDER_LINE_COLOR,
    (0xa0 << 24) | (0x88 << 16) | (0xa0 << 8) | 0x88);
    solo_reg_write(solo_dev, SOLO_VO_BORDER_FILL_COLOR,
    (0x10 << 24) | (0x8f << 16) | (0x10 << 8) | 0x8f);
    solo_reg_write(solo_dev, SOLO_VO_BKG_COLOR,
    (16 << 24) | (128 << 16) | (16 << 8) | 128);
    solo_reg_write(solo_dev, SOLO_VO_DISP_ERASE, SOLO_VO_DISP_ERASE_ON);
    solo_reg_write(solo_dev, SOLO_VI_WIN_SW, 0);
    solo_reg_write(solo_dev, SOLO_VO_ZOOM_CTRL, 0);
    solo_reg_write(solo_dev, SOLO_VO_FREEZE_CTRL, 0);
    solo_reg_write(solo_dev, SOLO_VO_DISP_CTRL, SOLO_VO_DISP_ON |
    SOLO_VO_DISP_ERASE_COUNT(8) |
    SOLO_VO_DISP_BASE(SOLO_DISP_EXT_ADDR));
    solo_vout_config_cursor(solo_dev);
// Enable channels we support
    solo_reg_write(solo_dev, SOLO_VI_CH_ENA,
    (1 << solo_dev.nr_chans) - 1);
    }
    static int solo_dma_vin_region(struct solo_dev *solo_dev, u32 off,
    u16 val, int reg_size)
    {
    __le16 *buf;
    let mut n: c_int = 64, size = n * sizeof(*buf);
    int i, ret = 0;
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf)
    return -ENOMEM;
    for (i = 0; i < n; i++)
    buf[i] = cpu_to_le16(val);
    for (i = 0; i < reg_size; i += size) {
    ret = solo_p2m_dma(solo_dev, 1, buf,
    SOLO_MOTION_EXT_ADDR(solo_dev) + off + i,
    size, 0, 0);
    if (ret)
    break;
    }
    kfree(buf);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn solo_set_motion_threshold(solo_dev: *mut solo_dev, ch: u8, val: u16) -> c_int {
    int solo_set_motion_threshold(struct solo_dev *solo_dev, u8 ch, u16 val)
    {
    if (ch > solo_dev.nr_chans)
    return -EINVAL;
    return solo_dma_vin_region(solo_dev, SOLO_MOT_FLAG_AREA +
    (ch * SOLO_MOT_THRESH_SIZE * 2),
    val, SOLO_MOT_THRESH_SIZE);
    }
    int solo_set_motion_block(struct solo_dev *solo_dev, u8 ch,
    const u16 *thresholds)
    {
    let mut size: unsigned = sizeof(u16) * 64;
    let mut off: u32 = SOLO_MOT_FLAG_AREA + ch * SOLO_MOT_THRESH_SIZE * 2;
    __le16 *buf;
    int x, y;
    let mut ret: c_int = 0;
    buf = kzalloc(size, GFP_KERNEL);
    if (buf == core::ptr::null_mut())
    return -ENOMEM;
    for (y = 0; y < SOLO_MOTION_SZ; y++) {
    for (x = 0; x < SOLO_MOTION_SZ; x++)
    buf[x] = cpu_to_le16(thresholds[y * SOLO_MOTION_SZ + x]);
    ret |= solo_p2m_dma(solo_dev, 1, buf,
    SOLO_MOTION_EXT_ADDR(solo_dev) + off + y * size,
    size, 0, 0);
    }
    kfree(buf);
    return ret;
    }
// First 8k is motion flag (512 bytes * 16). Following that is an 8k+8k
// threshold and working table for each channel. At least that's what the
// spec says. However, this code (taken from rdk) has some mystery 8k
// block right after the flag area, before the first thresh table.
#[no_mangle]
unsafe extern "C" fn solo_motion_config(solo_dev: *mut solo_dev) {
    static void solo_motion_config(struct solo_dev *solo_dev)
    {
    int i;
    for (i = 0; i < solo_dev.nr_chans; i++) {
// Clear motion flag area
    solo_dma_vin_region(solo_dev, i * SOLO_MOT_FLAG_SIZE, 0x0000,
    SOLO_MOT_FLAG_SIZE);
// Clear working cache table
    solo_dma_vin_region(solo_dev, SOLO_MOT_FLAG_AREA +
    (i * SOLO_MOT_THRESH_SIZE * 2) +
    SOLO_MOT_THRESH_SIZE, 0x0000,
    SOLO_MOT_THRESH_SIZE);
// Set default threshold table
    solo_set_motion_threshold(solo_dev, i, SOLO_DEF_MOT_THRESH);
    }
// Default motion settings
    solo_reg_write(solo_dev, SOLO_VI_MOT_ADR, SOLO_VI_MOTION_EN(0) |
    (SOLO_MOTION_EXT_ADDR(solo_dev) >> 16));
    solo_reg_write(solo_dev, SOLO_VI_MOT_CTRL,
    SOLO_VI_MOTION_FRAME_COUNT(3) |
    SOLO_VI_MOTION_SAMPLE_LENGTH(solo_dev.video_hsize / 16)
// | SOLO_VI_MOTION_INTR_START_STOP
    | SOLO_VI_MOTION_SAMPLE_COUNT(10));
    solo_reg_write(solo_dev, SOLO_VI_MOTION_BORDER, 0);
    solo_reg_write(solo_dev, SOLO_VI_MOTION_BAR, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn solo_disp_init(solo_dev: *mut solo_dev) -> c_int {
    int solo_disp_init(struct solo_dev *solo_dev)
    {
    int i;
    solo_dev.video_hsize = 704;
    if (solo_dev.video_type == SOLO_VO_FMT_TYPE_NTSC) {
    solo_dev.video_vsize = 240;
    solo_dev.fps = 30;
    } else {
    solo_dev.video_vsize = 288;
    solo_dev.fps = 25;
    }
    solo_vin_config(solo_dev);
    solo_motion_config(solo_dev);
    solo_vout_config(solo_dev);
    for (i = 0; i < solo_dev.nr_chans; i++)
    solo_reg_write(solo_dev, SOLO_VI_WIN_ON(i), 1);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn solo_disp_exit(solo_dev: *mut solo_dev) {
    void solo_disp_exit(struct solo_dev *solo_dev)
    {
    int i;
    solo_reg_write(solo_dev, SOLO_VO_DISP_CTRL, 0);
    solo_reg_write(solo_dev, SOLO_VO_ZOOM_CTRL, 0);
    solo_reg_write(solo_dev, SOLO_VO_FREEZE_CTRL, 0);
    for (i = 0; i < solo_dev.nr_chans; i++) {
    solo_reg_write(solo_dev, SOLO_VI_WIN_CTRL0(i), 0);
    solo_reg_write(solo_dev, SOLO_VI_WIN_CTRL1(i), 0);
    solo_reg_write(solo_dev, SOLO_VI_WIN_ON(i), 0);
    }
// Set default border
    for (i = 0; i < 5; i++)
    solo_reg_write(solo_dev, SOLO_VO_BORDER_X(i), 0);
    for (i = 0; i < 5; i++)
    solo_reg_write(solo_dev, SOLO_VO_BORDER_Y(i), 0);
    solo_reg_write(solo_dev, SOLO_VO_BORDER_LINE_MASK, 0);
    solo_reg_write(solo_dev, SOLO_VO_BORDER_FILL_MASK, 0);
    solo_reg_write(solo_dev, SOLO_VO_RECTANGLE_CTRL(0), 0);
    solo_reg_write(solo_dev, SOLO_VO_RECTANGLE_START(0), 0);
    solo_reg_write(solo_dev, SOLO_VO_RECTANGLE_STOP(0), 0);
    solo_reg_write(solo_dev, SOLO_VO_RECTANGLE_CTRL(1), 0);
    solo_reg_write(solo_dev, SOLO_VO_RECTANGLE_START(1), 0);
    solo_reg_write(solo_dev, SOLO_VO_RECTANGLE_STOP(1), 0);
    }
