//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/imx/dcss/dcss-ss.c
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
// Copyright 2019 NXP.
//

pub const DCSS_SS_SYS_CTRL: c_uint = 0x00;

pub const DCSS_SS_DISPLAY: c_uint = 0x10;
pub const LRC_X_POS: c_int = 0;

pub const LRC_Y_POS: c_int = 16;

pub const DCSS_SS_HSYNC: c_uint = 0x20;
pub const DCSS_SS_VSYNC: c_uint = 0x30;
pub const SYNC_START_POS: c_int = 0;

pub const SYNC_END_POS: c_int = 16;

pub const DCSS_SS_DE_ULC: c_uint = 0x40;
pub const ULC_X_POS: c_int = 0;

pub const ULC_Y_POS: c_int = 16;

pub const DCSS_SS_DE_LRC: c_uint = 0x50;
pub const DCSS_SS_MODE: c_uint = 0x60;
pub const PIPE_MODE_POS: c_int = 0;

pub const DCSS_SS_COEFF: c_uint = 0x70;
pub const HORIZ_A_POS: c_int = 0;

pub const HORIZ_B_POS: c_int = 4;

pub const HORIZ_C_POS: c_int = 8;

pub const HORIZ_H_NORM_POS: c_int = 12;

pub const VERT_A_POS: c_int = 16;

pub const VERT_B_POS: c_int = 20;

pub const VERT_C_POS: c_int = 24;

pub const VERT_H_NORM_POS: c_int = 28;

pub const DCSS_SS_CLIP_CB: c_uint = 0x80;
pub const DCSS_SS_CLIP_CR: c_uint = 0x90;
pub const CLIP_MIN_POS: c_int = 0;

pub const CLIP_MAX_POS: c_int = 0;

pub const DCSS_SS_INTER_MODE: c_uint = 0xA0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_ss {
    pub dev: *mut device,
    pub base_reg: *mut void __iomem,
    pub base_ofs: u32,
    pub ctxld: *mut dcss_ctxld,
    pub ctx_id: u32,
    pub in_use: bool,
}

#[no_mangle]
unsafe extern "C" fn dcss_ss_write(ss: *mut dcss_ss, val: u32, ofs: u32) {
    static void dcss_ss_write(struct dcss_ss *ss, u32 val, u32 ofs)
    {
    if (!ss.in_use)
    dcss_writel(val, ss.base_reg + ofs);
    dcss_ctxld_write(ss.ctxld, ss.ctx_id, val,
    ss.base_ofs + ofs);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ss_init(dcss: *mut dcss_dev, ss_base: c_ulong) -> c_int {
    int dcss_ss_init(struct dcss_dev *dcss, unsigned long ss_base)
    {
    struct dcss_ss *ss;
    ss = devm_kzalloc(dcss.dev, sizeof(*ss), GFP_KERNEL);
    if (!ss)
    return -ENOMEM;
    dcss.ss = ss;
    ss.dev = dcss.dev;
    ss.ctxld = dcss.ctxld;
    ss.base_reg = devm_ioremap(ss.dev, ss_base, SZ_4K);
    if (!ss.base_reg) {
    dev_err(ss.dev, "ss: unable to remap ss base\n");
    return -ENOMEM;
    }
    ss.base_ofs = ss_base;
    ss.ctx_id = CTX_SB_HP;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ss_exit(ss: *mut dcss_ss) {
    void dcss_ss_exit(struct dcss_ss *ss)
    {
// stop SS
    dcss_writel(0, ss.base_reg + DCSS_SS_SYS_CTRL);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ss_subsam_set(ss: *mut dcss_ss) {
    void dcss_ss_subsam_set(struct dcss_ss *ss)
    {
    dcss_ss_write(ss, 0x41614161, DCSS_SS_COEFF);
    dcss_ss_write(ss, 0, DCSS_SS_MODE);
    dcss_ss_write(ss, 0x03ff0000, DCSS_SS_CLIP_CB);
    dcss_ss_write(ss, 0x03ff0000, DCSS_SS_CLIP_CR);
    }
    void dcss_ss_sync_set(struct dcss_ss *ss, struct videomode *vm,
    bool phsync, bool pvsync)
    {
    u16 lrc_x, lrc_y;
    u16 hsync_start, hsync_end;
    u16 vsync_start, vsync_end;
    u16 de_ulc_x, de_ulc_y;
    u16 de_lrc_x, de_lrc_y;
    lrc_x = vm.hfront_porch + vm.hback_porch + vm.hsync_len +
    vm.hactive - 1;
    lrc_y = vm.vfront_porch + vm.vback_porch + vm.vsync_len +
    vm.vactive - 1;
    dcss_ss_write(ss, (lrc_y << LRC_Y_POS) | lrc_x, DCSS_SS_DISPLAY);
    hsync_start = vm.hfront_porch + vm.hback_porch + vm.hsync_len +
    vm.hactive - 1;
    hsync_end = vm.hsync_len - 1;
    dcss_ss_write(ss, (phsync ? SYNC_POL : 0) |
    ((u32)hsync_end << SYNC_END_POS) | hsync_start,
    DCSS_SS_HSYNC);
    vsync_start = vm.vfront_porch - 1;
    vsync_end = vm.vfront_porch + vm.vsync_len - 1;
    dcss_ss_write(ss, (pvsync ? SYNC_POL : 0) |
    ((u32)vsync_end << SYNC_END_POS) | vsync_start,
    DCSS_SS_VSYNC);
    de_ulc_x = vm.hsync_len + vm.hback_porch - 1;
    de_ulc_y = vm.vsync_len + vm.vfront_porch + vm.vback_porch;
    dcss_ss_write(ss, SYNC_POL | ((u32)de_ulc_y << ULC_Y_POS) | de_ulc_x,
    DCSS_SS_DE_ULC);
    de_lrc_x = vm.hsync_len + vm.hback_porch + vm.hactive - 1;
    de_lrc_y = vm.vsync_len + vm.vfront_porch + vm.vback_porch +
    vm.vactive - 1;
    dcss_ss_write(ss, (de_lrc_y << LRC_Y_POS) | de_lrc_x, DCSS_SS_DE_LRC);
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ss_enable(ss: *mut dcss_ss) {
    void dcss_ss_enable(struct dcss_ss *ss)
    {
    dcss_ss_write(ss, RUN_EN, DCSS_SS_SYS_CTRL);
    ss.in_use = true;
    }
#[no_mangle]
pub unsafe extern "C" fn dcss_ss_shutoff(ss: *mut dcss_ss) {
    void dcss_ss_shutoff(struct dcss_ss *ss)
    {
    dcss_writel(0, ss.base_reg + DCSS_SS_SYS_CTRL);
    ss.in_use = false;
    }
