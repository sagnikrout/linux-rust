//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_ddi_buf_trans.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2020 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hsw_ddi_buf_trans {
    pub /: *mut *mut u32 trans1; / balance leg enable, de-emph level,
    pub /: *mut *mut u32 trans2; / vref sel, vswing,
    pub /: *mut *mut u8 i_boost; / SKL: I_boost; valid: 0x0, 0x1, 0x3, 0x7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bxt_ddi_buf_trans {
    pub /: *mut *mut u8 margin; / swing value,
    pub /: *mut *mut u8 scale; / scale value,
    pub /: *mut *mut u8 enable; / scale enable,
    pub deemphasis: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icl_ddi_buf_trans {
    pub dw2_swing_sel: u8,
    pub dw7_n_scalar: u8,
    pub dw4_cursor_coeff: u8,
    pub dw4_post_cursor_2: u8,
    pub dw4_post_cursor_1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icl_mg_phy_ddi_buf_trans {
    pub cri_txdeemph_override_11_6: u8,
    pub cri_txdeemph_override_5_0: u8,
    pub cri_txdeemph_override_17_12: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tgl_dkl_phy_ddi_buf_trans {
    pub vswing: u8,
    pub preshoot: u8,
    pub de_emphasis: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dg2_snps_phy_buf_trans {
    pub vswing: u8,
    pub pre_cursor: u8,
    pub post_cursor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xe3plpd_lt_phy_buf_trans {
    pub txswing: u8,
    pub txswing_level: u8,
    pub pre_cursor: u8,
    pub main_cursor: u8,
    pub post_cursor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union intel_ddi_buf_trans_entry {
    pub hsw: hsw_ddi_buf_trans,
    pub bxt: bxt_ddi_buf_trans,
    pub icl: icl_ddi_buf_trans,
    pub mg: icl_mg_phy_ddi_buf_trans,
    pub dkl: tgl_dkl_phy_ddi_buf_trans,
    pub snps: dg2_snps_phy_buf_trans,
    pub lt: xe3plpd_lt_phy_buf_trans,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_ddi_buf_trans {
    pub entries: *const intel_ddi_buf_trans_entry,
    pub num_entries: u8,
    pub hdmi_default_entry: u8,
}

extern "C" {
    pub fn is_hobl_buf_trans(table: *const intel_ddi_buf_trans) -> bool;
}
extern "C" {
    pub fn intel_ddi_buf_trans_init(encoder: *mut intel_encoder);
}
