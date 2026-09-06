//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/dcss/dcss-dev.h
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

pub const SET: c_uint = 0x04;
pub const CLR: c_uint = 0x08;
pub const TGL: c_uint = 0x0C;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_type_data {
    pub name: *const c_char,
    pub blkctl_ofs: u32,
    pub ctxld_ofs: u32,
    pub rdsrc_ofs: u32,
    pub wrscl_ofs: u32,
    pub dtg_ofs: u32,
    pub scaler_ofs: u32,
    pub ss_ofs: u32,
    pub dpr_ofs: u32,
    pub dtrc_ofs: u32,
    pub dec400d_ofs: u32,
    pub hdr10_ofs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_debug_reg {
    pub name: *mut c_char,
    pub ofs: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcss_ctxld_ctx_type {
    CTX_DB,
    CTX_SB_HP, /* high-priority */
    CTX_SB_LP, /* low-priority  */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcss_dev {
    pub dev: *mut device,
    pub devtype: *const dcss_type_data,
    pub of_port: *mut device_node,
    pub start_addr: u32,
    pub blkctl: *mut dcss_blkctl,
    pub ctxld: *mut dcss_ctxld,
    pub dpr: *mut dcss_dpr,
    pub dtg: *mut dcss_dtg,
    pub ss: *mut dcss_ss,
    pub hdr10: *mut dcss_hdr10,
    pub scaler: *mut dcss_scaler,
    pub dtrc: *mut dcss_dtrc,
    pub dec400d: *mut dcss_dec400d,
    pub wrscl: *mut dcss_wrscl,
    pub rdsrc: *mut dcss_rdsrc,
    pub apb_clk: *mut clk,
    pub axi_clk: *mut clk,
    pub pix_clk: *mut clk,
    pub rtrm_clk: *mut clk,
    pub dtrc_clk: *mut clk,
    pub pll_src_clk: *mut clk,
    pub pll_phy_ref_clk: *mut clk,
    pub hdmi_output: bool,
    pub data): *mut *mut void (disable_callback)(void,
    pub disable_completion: completion,
}

extern "C" {
    pub fn dcss_dev_destroy(dcss: *mut dcss_dev);
}
extern "C" {
    pub fn dcss_enable_dtg_and_ss(dcss: *mut dcss_dev);
}
extern "C" {
    pub fn dcss_disable_dtg_and_ss(dcss: *mut dcss_dev);
}
// BLKCTL
extern "C" {
    pub fn dcss_blkctl_init(dcss: *mut dcss_dev, blkctl_base: c_ulong) -> c_int;
}
extern "C" {
    pub fn dcss_blkctl_cfg(blkctl: *mut dcss_blkctl);
}
// CTXLD
extern "C" {
    pub fn dcss_ctxld_init(dcss: *mut dcss_dev, ctxld_base: c_ulong) -> c_int;
}
extern "C" {
    pub fn dcss_ctxld_exit(ctxld: *mut dcss_ctxld);
}
extern "C" {
    pub fn dcss_ctxld_resume(dcss_ctxld: *mut dcss_ctxld) -> c_int;
}
extern "C" {
    pub fn dcss_ctxld_suspend(dcss_ctxld: *mut dcss_ctxld) -> c_int;
}
extern "C" {
    pub fn dcss_ctxld_kick(ctxld: *mut dcss_ctxld);
}
extern "C" {
    pub fn dcss_ctxld_is_flushed(ctxld: *mut dcss_ctxld) -> bool;
}
extern "C" {
    pub fn dcss_ctxld_enable(ctxld: *mut dcss_ctxld) -> c_int;
}
extern "C" {
    pub fn dcss_ctxld_assert_locked(ctxld: *mut dcss_ctxld);
}
// DPR
extern "C" {
    pub fn dcss_dpr_init(dcss: *mut dcss_dev, dpr_base: c_ulong) -> c_int;
}
extern "C" {
    pub fn dcss_dpr_exit(dpr: *mut dcss_dpr);
}
extern "C" {
    pub fn dcss_dpr_write_sysctrl(dpr: *mut dcss_dpr);
}
extern "C" {
    pub fn dcss_dpr_set_res(dpr: *mut dcss_dpr, ch_num: c_int, xres: u32, yres: u32);
}
extern "C" {
    pub fn dcss_dpr_enable(dpr: *mut dcss_dpr, ch_num: c_int, en: bool);
}
extern "C" {
    pub fn dcss_dpr_set_rotation(dpr: *mut dcss_dpr, ch_num: c_int, rotation: u32);
}
// DTG
extern "C" {
    pub fn dcss_dtg_init(dcss: *mut dcss_dev, dtg_base: c_ulong) -> c_int;
}
extern "C" {
    pub fn dcss_dtg_exit(dtg: *mut dcss_dtg);
}
extern "C" {
    pub fn dcss_dtg_vblank_irq_valid(dtg: *mut dcss_dtg) -> bool;
}
extern "C" {
    pub fn dcss_dtg_vblank_irq_enable(dtg: *mut dcss_dtg, en: bool);
}
extern "C" {
    pub fn dcss_dtg_vblank_irq_clear(dtg: *mut dcss_dtg);
}
extern "C" {
    pub fn dcss_dtg_sync_set(dtg: *mut dcss_dtg, vm: *mut videomode);
}
extern "C" {
    pub fn dcss_dtg_css_set(dtg: *mut dcss_dtg);
}
extern "C" {
    pub fn dcss_dtg_enable(dtg: *mut dcss_dtg);
}
extern "C" {
    pub fn dcss_dtg_shutoff(dtg: *mut dcss_dtg);
}
extern "C" {
    pub fn dcss_dtg_is_enabled(dtg: *mut dcss_dtg) -> bool;
}
extern "C" {
    pub fn dcss_dtg_ctxld_kick_irq_enable(dtg: *mut dcss_dtg, en: bool);
}
extern "C" {
    pub fn dcss_dtg_global_alpha_changed(dtg: *mut dcss_dtg, ch_num: c_int, alpha: c_int) -> bool;
}
extern "C" {
    pub fn dcss_dtg_ch_enable(dtg: *mut dcss_dtg, ch_num: c_int, en: bool);
}
// SUBSAM
extern "C" {
    pub fn dcss_ss_init(dcss: *mut dcss_dev, subsam_base: c_ulong) -> c_int;
}
extern "C" {
    pub fn dcss_ss_exit(ss: *mut dcss_ss);
}
extern "C" {
    pub fn dcss_ss_enable(ss: *mut dcss_ss);
}
extern "C" {
    pub fn dcss_ss_shutoff(ss: *mut dcss_ss);
}
extern "C" {
    pub fn dcss_ss_subsam_set(ss: *mut dcss_ss);
}
// SCALER
extern "C" {
    pub fn dcss_scaler_init(dcss: *mut dcss_dev, scaler_base: c_ulong) -> c_int;
}
extern "C" {
    pub fn dcss_scaler_exit(scl: *mut dcss_scaler);
}
extern "C" {
    pub fn dcss_scaler_ch_enable(scl: *mut dcss_scaler, ch_num: c_int, en: bool);
}
extern "C" {
    pub fn dcss_scaler_write_sclctrl(scl: *mut dcss_scaler);
}
