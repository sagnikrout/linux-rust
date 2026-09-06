//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/mediatek/clk-pllfh.h
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
// Copyright (c) 2022 MediaTek Inc.
// Author: Edward-JW Yang <edward-jw.yang@mediatek.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fh_pll_state {
    pub base: *mut void __iomem,
    pub fh_enable: u32,
    pub ssc_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fh_pll_data {
    pub pll_id: c_int,
    pub fh_id: c_int,
    pub fh_ver: c_int,
    pub fhx_offset: u32,
    pub dds_mask: u32,
    pub slope0_value: u32,
    pub slope1_value: u32,
    pub sfstrx_en: u32,
    pub frddsx_en: u32,
    pub fhctlx_en: u32,
    pub tgl_org: u32,
    pub dvfs_tri: u32,
    pub pcwchg: u32,
    pub dt_val: u32,
    pub df_val: u32,
    pub updnlmt_shft: u32,
    pub msk_frddsx_dys: u32,
    pub msk_frddsx_dts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_pllfh_data {
    pub state: fh_pll_state,
    pub data: fh_pll_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fh_pll_regs {
    pub reg_hp_en: *mut void __iomem,
    pub reg_clk_con: *mut void __iomem,
    pub reg_rst_con: *mut void __iomem,
    pub reg_slope0: *mut void __iomem,
    pub reg_slope1: *mut void __iomem,
    pub reg_cfg: *mut void __iomem,
    pub reg_updnlmt: *mut void __iomem,
    pub reg_dds: *mut void __iomem,
    pub reg_dvfs: *mut void __iomem,
    pub reg_mon: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_fh {
    pub clk_pll: mtk_clk_pll,
    pub regs: fh_pll_regs,
    pub pllfh_data: *mut mtk_pllfh_data,
    pub ops: *const fh_operation,
    pub lock: *mut spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fh_operation {
    pub postdiv): c_uint,
    pub rate): *mut *mut *mut int (ssc_enable)(struct mtk_fh fh, u32,
}
