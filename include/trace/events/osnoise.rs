//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/osnoise.h
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
// osnoise sample structure definition. Used to store the statistics of a
// sample run.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct osnoise_sample {
    pub /: *mut *mut u64 runtime; / runtime,
    pub /: *mut *mut u64 noise; / noise,
    pub /: *mut *mut u64 max_sample; / max single noise sample,
    pub /: *mut *mut int hw_count; / # HW (incl. hypervisor) interference,
    pub /: *mut *mut int nmi_count; / # NMIs during this sample,
    pub /: *mut *mut int irq_count; / # IRQs during this sample,
    pub /: *mut *mut int softirq_count; / # softirqs during this sample,
    pub /: *mut *mut int thread_count; / # threads during this sample,
}

//
// timerlat sample structure definition. Used to store the statistics of
// a sample run.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct timerlat_sample {
    pub /: *mut *mut u64 timer_latency; / timer_latency,
    pub /: *mut *mut unsigned int seqnum; / unique sequence,
    pub /: *mut *mut int context; / timer context,
}

// This part must be outside protection
