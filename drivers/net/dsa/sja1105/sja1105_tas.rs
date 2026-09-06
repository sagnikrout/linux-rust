//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/sja1105/sja1105_tas.h
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
// Copyright (c) 2019, Vladimir Oltean <olteanv@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_tas_state {
    SJA1105_TAS_STATE_DISABLED,
    SJA1105_TAS_STATE_ENABLED_NOT_RUNNING,
    SJA1105_TAS_STATE_RUNNING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sja1105_ptp_op {
    SJA1105_PTP_NONE,
    SJA1105_PTP_CLOCKSTEP,
    SJA1105_PTP_ADJUSTFREQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_gate_entry {
    pub list: list_head,
    pub rule: *mut sja1105_rule,
    pub interval: i64,
    pub gate_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_gating_config {
    pub cycle_time: u64,
    pub base_time: i64,
    pub num_entries: c_int,
    pub entries: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_tas_data {
    pub offload: [*mut tc_taprio_qopt_offload; SJA1105_MAX_NUM_PORTS],
    pub gating_cfg: sja1105_gating_config,
    pub state: sja1105_tas_state,
    pub last_op: sja1105_ptp_op,
    pub tas_work: work_struct,
    pub earliest_base_time: i64,
    pub oper_base_time: i64,
    pub max_cycle_time: u64,
    pub enabled: bool,
}

extern "C" {
    pub fn sja1105_tas_setup(ds: *mut dsa_switch);
}
extern "C" {
    pub fn sja1105_tas_teardown(ds: *mut dsa_switch);
}
extern "C" {
    pub fn sja1105_tas_clockstep(ds: *mut dsa_switch);
}
extern "C" {
    pub fn sja1105_tas_adjfreq(ds: *mut dsa_switch);
}
extern "C" {
    pub fn sja1105_init_scheduling(priv: *mut sja1105_private) -> c_int;
}

// C doesn't allow empty structures, bah!
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sja1105_tas_data {
    pub dummy: u8,
}

