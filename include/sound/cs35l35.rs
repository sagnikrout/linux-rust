//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/cs35l35.h
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
// linux/sound/cs35l35.h -- Platform data for CS35l35
//
// Copyright (c) 2016 Cirrus Logic Inc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct classh_cfg {
//
// Class H Algorithm Control Variables
// You can either have it done
// automatically or you can adjust
// these variables for tuning
//
// if you do not enable the internal algorithm
// you will get a set of mixer controls for
// Class H tuning
//
// Section 4.3 of the datasheet
//
    pub classh_bst_override: bool,
    pub classh_algo_enable: bool,
    pub classh_bst_max_limit: c_int,
    pub classh_mem_depth: c_int,
    pub classh_release_rate: c_int,
    pub classh_headroom: c_int,
    pub classh_wk_fet_disable: c_int,
    pub classh_wk_fet_delay: c_int,
    pub classh_wk_fet_thld: c_int,
    pub classh_vpch_auto: c_int,
    pub classh_vpch_rate: c_int,
    pub classh_vpch_man: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct monitor_cfg {
//
// Signal Monitor Data
// highly configurable signal monitoring
// data positioning and different types of
// monitoring data.
//
// Section 4.8.2 - 4.8.4 of the datasheet
//
    pub is_present: bool,
    pub imon_specs: bool,
    pub vmon_specs: bool,
    pub vpmon_specs: bool,
    pub vbstmon_specs: bool,
    pub vpbrstat_specs: bool,
    pub zerofill_specs: bool,
    pub imon_dpth: u8,
    pub imon_loc: u8,
    pub imon_frm: u8,
    pub imon_scale: u8,
    pub vmon_dpth: u8,
    pub vmon_loc: u8,
    pub vmon_frm: u8,
    pub vpmon_dpth: u8,
    pub vpmon_loc: u8,
    pub vpmon_frm: u8,
    pub vbstmon_dpth: u8,
    pub vbstmon_loc: u8,
    pub vbstmon_frm: u8,
    pub vpbrstat_dpth: u8,
    pub vpbrstat_loc: u8,
    pub vpbrstat_frm: u8,
    pub zerofill_dpth: u8,
    pub zerofill_loc: u8,
    pub zerofill_frm: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs35l35_platform_data {
// Stereo (2 Device)
    pub stereo: bool,
// serial port drive strength
    pub sp_drv_str: c_int,
// serial port drive in unused slots
    pub sp_drv_unused: c_int,
// Boost Power Down with FET
    pub bst_pdn_fet_on: bool,
// Boost Voltage : used if ClassH Algo Enabled
    pub bst_vctl: c_int,
// Boost Converter Peak Current CTRL
    pub bst_ipk: c_int,
// Amp Gain Zero Cross
    pub gain_zc: bool,
// Audio Input Location
    pub aud_channel: c_int,
// Advisory Input Location
    pub adv_channel: c_int,
// Shared Boost for stereo
    pub shared_bst: bool,
// Specifies this amp is using an external boost supply
    pub ext_bst: bool,
// Inductor Value
    pub boost_ind: c_int,
// ClassH Algorithm
    pub classh_algo: classh_cfg,
// Monitor Config
    pub mon_cfg: monitor_cfg,
}
