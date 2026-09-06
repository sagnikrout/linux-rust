//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/msm_perfcntr.h
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
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//

//
// This is a subset of the tables used by mesa.  We don't need to
// enumerate the countables on the kernel side.
//
// Describes a single counter:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_perfcntr_counter {
// offset of the SELect register to choose what to count:
    pub select_reg: unsigned,
// additional SEL regs to enable slice counters (gen8+)
    pub slice_select_regs: [unsigned; 2],
// offset of the lo/hi 32b to read current counter value:
    pub counter_reg_lo: unsigned,
    pub counter_reg_hi: unsigned,
// TODO some counters have enable/clear registers
}

// Describes an entire counter group:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_perfcntr_group {
    pub name: *const c_char,
    pub pipe: adreno_pipe,
    pub num_counters: unsigned,
    pub counters: *const msm_perfcntr_counter,
}

//
// struct msm_perfcntr_stream - state for a single open stream fd
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_perfcntr_stream {
// @gpu: Back-link to the GPU
    pub gpu: *mut msm_gpu,
// @sample_timer: Timer to sample counters
    pub sample_timer: hrtimer,
// @poll_wq: Wait queue for waiting for OA data to be available
    pub poll_wq: wait_queue_head_t,
// @sample_period_ns: Sampling period
    pub sample_period_ns: u64,
// @nr_groups: # of counter groups with enabled counters
    pub nr_groups: u32,
// @seqno: counter for collected samples
    pub seqno: u32,
// @sel_fence: Fence for SEL reg programming
    pub sel_fence: u32,
//
// @sel_work: Worker for SEL reg programming
//
// Initial SEL reg programming (as opposed to restoring the SEL
// regs on runpm resume) must run on the same ordered wq as is
// used by drm_sched, to serialize it with GEM_SUBMITs written
// into the same ringbuffer.
//
    pub sel_work: work_struct,
//
// @sample_work: Worker for collecting samples
//
    pub sample_work: kthread_work,
//
// @read_lock:
//
// Fifo access is synchronied on the producer side by virtue
// of there being a single timer collecting samples and writing
// into the fifo.  It is protected on the consumer side by
// @read_lock.
//
    pub read_lock: mutex,
//
// @group_idx: array of nr_groups
//
// Maps the order of groups in PERFCNTR_CONFIG ioctl to group idx,
// so that results in the results stream can be ordered to match
// the ioctl call that setup the stream
//
    pub group_idx: *mut u32,
// @fifo: circular buffer for samples
    pub fifo: circ_buf,
// @fifo_size: circular buffer size
    pub fifo_size: usize,
// @period_size: size of data for single sampling period
    pub period_size: usize,
}

extern "C" {
    pub fn msm_perfcntr_group_idx(stream: *const msm_perfcntr_stream, n: u32) -> u32;
}
extern "C" {
    pub fn msm_perfcntr_counter_base(stream: *const msm_perfcntr_stream, group_idx: u32) -> u32;
}
//
// struct msm_perfcntr_context_state - per-msm_context counter state
//
// A given counter can either be unused, reserved for global counter
// collection exclusively, or reserved for local per-context counter
// collection inclusively.  Multiple contexts can reserve the same
// counter, since SEL reg programming and counter begin/end sampling
// happen locally (within a single GEM_SUBMIT ioctl).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_perfcntr_context_state {
// @dummy: Some compilers dislike structs with only a flex array
    pub dummy: unsigned,
//
// @reserved_counters:
//
// The number of reserved counters indexed by perfcntr group.
//
    pub reserved_counters: [unsigned; ],
}

