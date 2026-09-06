//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs35l36.h
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
// linux/sound/cs35l36.h -- Platform data for CS35L36
//
// Copyright 2018 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l36_vpbr_cfg {
    pub is_present: bool,
    pub vpbr_en: bool,
    pub vpbr_thld: c_int,
    pub vpbr_atk_rate: c_int,
    pub vpbr_atk_vol: c_int,
    pub vpbr_max_attn: c_int,
    pub vpbr_wait: c_int,
    pub vpbr_rel_rate: c_int,
    pub vpbr_mute_en: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l36_platform_data {
    pub multi_amp_mode: bool,
    pub dcm_mode: bool,
    pub amp_pcm_inv: bool,
    pub imon_pol_inv: bool,
    pub vmon_pol_inv: bool,
    pub boost_ind: c_int,
    pub bst_vctl: c_int,
    pub bst_vctl_sel: c_int,
    pub bst_ipk: c_int,
    pub extern_boost: bool,
    pub temp_warn_thld: c_int,
    pub irq_drv_sel: c_int,
    pub irq_gpio_sel: c_int,
    pub vpbr_config: cs35l36_vpbr_cfg,
}
