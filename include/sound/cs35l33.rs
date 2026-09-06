//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs35l33.h
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
// linux/sound/cs35l33.h -- Platform data for CS35l33
//
// Copyright (c) 2016 Cirrus Logic Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l33_hg {
    pub enable_hg_algo: bool,
    pub mem_depth: c_uint,
    pub release_rate: c_uint,
    pub hd_rm: c_uint,
    pub ldo_thld: c_uint,
    pub ldo_path_disable: c_uint,
    pub ldo_entry_delay: c_uint,
    pub vp_hg_auto: bool,
    pub vp_hg: c_uint,
    pub vp_hg_rate: c_uint,
    pub vp_hg_va: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l33_pdata {
// Boost Controller Voltage Setting
    pub boost_ctl: c_uint,
// Boost Controller Peak Current
    pub boost_ipk: c_uint,
// Amplifier Drive Select
    pub amp_drv_sel: c_uint,
// soft volume ramp
    pub ramp_rate: c_uint,
// IMON adc scale
    pub imon_adc_scale: c_uint,
// H/G algo configuration
    pub hg_config: cs35l33_hg,
}
