//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/scx_flatcg.h
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


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fcg_stat_idx {
    FCG_STAT_ACT,
    FCG_STAT_DEACT,
    FCG_STAT_LOCAL,
    FCG_STAT_GLOBAL,

    FCG_STAT_HWT_UPDATES,
    FCG_STAT_HWT_CACHE,
    FCG_STAT_HWT_SKIP,
    FCG_STAT_HWT_RACE,

    FCG_STAT_ENQ_SKIP,
    FCG_STAT_ENQ_RACE,

    FCG_STAT_CNS_KEEP,
    FCG_STAT_CNS_EXPIRE,
    FCG_STAT_CNS_EMPTY,
    FCG_STAT_CNS_GONE,

    FCG_STAT_PNC_NO_CGRP,
    FCG_STAT_PNC_NEXT,
    FCG_STAT_PNC_EMPTY,
    FCG_STAT_PNC_GONE,
    FCG_STAT_PNC_RACE,
    FCG_STAT_PNC_FAIL,

    FCG_STAT_BAD_REMOVAL,

    FCG_NR_STATS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fcg_cgrp_ctx {
    pub nr_active: u32,
    pub nr_runnable: u32,
    pub queued: u32,
    pub weight: u32,
    pub hweight: u32,
    pub child_weight_sum: u64,
    pub hweight_gen: u64,
    pub cvtime_delta: i64,
    pub tvtime_now: u64,
}
