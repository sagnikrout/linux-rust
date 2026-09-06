//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/red.h
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

// Random Early Detection (RED) algorithm.
//
// Adaptative RED : An Algorithm for Increasing the Robustness of RED's AQM
// (Sally FLoyd, Ramakrishna Gummadi, and Scott Shenker) August 2001
//
// Every 500 ms:
// if (avg > target and max_p <= 0.5)
// increase max_p : max_p += alpha;
// else if (avg < target and max_p >= 0.01)
// decrease max_p : max_p *= beta;
//
// target :[qth_min + 0.4*(qth_min - qth_max),
// qth_min + 0.6*(qth_min - qth_max)].
// alpha : min(0.01, max_p / 4)
// beta : 0.9
// max_P is a Q0.32 fixed point number (with 32 bits mantissa)
// max_P between 0.01 and 0.5 (1% - 50%) [ Its no longer a negative power of two ]
//

pub const RED_STAB_SIZE: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct red_stats {
    pub /: *mut *mut u32 prob_drop; / Early probability drops,
    pub /: *mut *mut u32 prob_mark; / Early probability marks,
    pub /: *mut *mut u32 forced_drop; / Forced drops, qavg > max_thresh,
    pub /: *mut *mut u32 forced_mark; / Forced marks, qavg > max_thresh,
    pub /: *mut *mut u32 pdrop; / Drops due to queue limits,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct red_parms {
// Parameters
    pub /: *mut *mut u32 qth_min; / Min avg length threshold: Wlog scaled,
    pub /: *mut *mut u32 qth_max; / Max avg length threshold: Wlog scaled,
    pub Scell_max: u32,
    pub /: *mut *mut u32 max_P; / probability, [0 .. 1.0] 32 scaled,
// reciprocal_value(max_P / qth_delta)
    pub max_P_reciprocal: reciprocal_value,
    pub /: *mut *mut u32 qth_delta; / max_th - min_th,
    pub /: *mut *mut *mut u32 target_min; / min_th + 0.4(max_th - min_th),
    pub /: *mut *mut *mut u32 target_max; / min_th + 0.6(max_th - min_th),
    pub Scell_log: u8,
    pub /: *mut *mut u8 Wlog; / log(W),
    pub /: *mut *mut u8 Plog; / random number bits,
    pub Stab: [u8; RED_STAB_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct red_vars {
// Variables
    pub random: *mut *mut int qcount; / Number of packets since last,
    pub /: *mut *mut u32 qR; / Cached random number,
    pub /: *mut *mut unsigned long qavg; / Average queue length: Wlog scaled,
    pub /: *mut *mut ktime_t qidlestart; / Start of current idle period,
}

// Reset average queue length, the value is strictly bound
// to the parameters below, resetting hurts a bit but leaving
// it might result in an unreasonable qavg for a while. --TGR
//
// p_flags = flags;
// p_userbits = qopt_flags & ~historic_mask;
// RED Adaptative target :
// [min_th + 0.4*(min_th - max_th),
// min_th + 0.6*(min_th - max_th)].
//
// The problem: ideally, average length queue recalculation should
// be done over constant clock intervals. This is too expensive, so
// that the calculation is driven by outgoing packets.
// When the queue is idle we have to model this clock by hand.
//
// SF+VJ proposed to "generate":
//
// m = idletime / (average_pkt_size / bandwidth)
//
// dummy packets as a burst after idle time, i.e.
//
// v->qavg *= (1-W)^m
//
// This is an apparently overcomplicated solution (f.e. we have to
// precompute a table to make this calculation in reasonable time)
// I believe that a simpler model may be used here,
// but it is field for experiments.
//
// Approximate initial part of exponent with linear function:
//
// (1-W)^m ~= 1-mW + ...
//
// Seems, it is the best solution to
// problem of too coarse exponent tabulation.
//
// NOTE: v->qavg is fixed point number with point at Wlog.
// The formula below is equivalent to floating point
// version:
//
// qavg = qavg*(1-W) + backlog*W;
//
// --ANK (980924)
//
extern "C" {
    pub fn red_calc_qavg_no_idle_time(_arg: p, _arg: v, _arg: backlog) -> return;
}
extern "C" {
    pub fn red_calc_qavg_from_idle_time(_arg: p, _arg: v) -> return;
}
extern "C" {
    pub fn reciprocal_divide(_arg: get_random_u32(), _arg: p->max_P_reciprocal) -> return;
}
// The formula used below causes questions.
//
// v->qavg is fixed point number with point at Wlog
