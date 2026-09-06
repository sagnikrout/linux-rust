//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/evsel_config.h
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
pub const __PERF_EVSEL_CONFIG_H: c_int = 1;

//
// The 'struct evsel_config_term' is used to pass event
// specific configuration data to evsel__config routine.
// It is allocated within event parsing and attached to
// evsel::config_terms list head.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum evsel_term_type {
    EVSEL__CONFIG_TERM_PERIOD,
    EVSEL__CONFIG_TERM_FREQ,
    EVSEL__CONFIG_TERM_TIME,
    EVSEL__CONFIG_TERM_CALLGRAPH,
    EVSEL__CONFIG_TERM_STACK_USER,
    EVSEL__CONFIG_TERM_INHERIT,
    EVSEL__CONFIG_TERM_MAX_STACK,
    EVSEL__CONFIG_TERM_MAX_EVENTS,
    EVSEL__CONFIG_TERM_OVERWRITE,
    EVSEL__CONFIG_TERM_DRV_CFG,
    EVSEL__CONFIG_TERM_BRANCH,
    EVSEL__CONFIG_TERM_PERCORE,
    EVSEL__CONFIG_TERM_AUX_OUTPUT,
    EVSEL__CONFIG_TERM_AUX_ACTION,
    EVSEL__CONFIG_TERM_AUX_SAMPLE_SIZE,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG1,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG2,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG3,
    EVSEL__CONFIG_TERM_USR_CHG_CONFIG4,
    EVSEL__CONFIG_TERM_RATIO_TO_PREV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct evsel_config_term {
    pub list: list_head,
    pub type: evsel_term_type,
    pub free_str: bool,
    pub period: u64,
    pub freq: u64,
    pub time: bool,
    pub stack_user: u64,
    pub max_stack: c_int,
    pub inherit: bool,
    pub overwrite: bool,
    pub max_events: c_ulong,
    pub percore: bool,
    pub aux_output: bool,
    pub aux_sample_size: u32,
    pub cfg_chg: u64,
    pub str: *mut c_char,
    pub cpu: c_int,
    pub val: u64,
    pub val: },
    pub weak: bool,
}

