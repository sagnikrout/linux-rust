//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/a5xx_gpu.h
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
// Copyright (c) 2016-2017 The Linux Foundation. All rights reserved.
//

// Bringing over the hack from the previous targets

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a5xx_gpu {
    pub base: adreno_gpu,
    pub pm4_bo: *mut drm_gem_object,
    pub pm4_iova: u64,
    pub pfp_bo: *mut drm_gem_object,
    pub pfp_iova: u64,
    pub gpmu_bo: *mut drm_gem_object,
    pub gpmu_iova: u64,
    pub gpmu_dwords: u32,
    pub lm_leakage: u32,
    pub cur_ring: *mut msm_ringbuffer,
    pub next_ring: *mut msm_ringbuffer,
    pub preempt_bo: [*mut drm_gem_object; MSM_GPU_MAX_RINGS],
    pub preempt_counters_bo: [*mut drm_gem_object; MSM_GPU_MAX_RINGS],
    pub preempt: [*mut a5xx_preempt_record; MSM_GPU_MAX_RINGS],
    pub preempt_iova: [u64; MSM_GPU_MAX_RINGS],
    pub last_seqno: [u32; MSM_GPU_MAX_RINGS],
    pub preempt_state: core::sync::atomic::AtomicI32,
    pub preempt_start_lock: spinlock_t,
    pub preempt_timer: timer_list,
    pub shadow_bo: *mut drm_gem_object,
    pub shadow_iova: u64,
    pub shadow: *mut u32,
// True if the microcode supports the WHERE_AM_I opcode
    pub has_whereami: bool,
}

extern "C" {
    pub fn a5xx_debugfs_init(gpu: *mut msm_gpu, minor: *mut drm_minor);
}

//
// In order to do lockless preemption we use a simple state machine to progress
// through the process.
//
// PREEMPT_NONE - no preemption in progress.  Next state START.
// PREEMPT_START - The trigger is evaulating if preemption is possible. Next
// states: TRIGGERED, NONE
// PREEMPT_ABORT - An intermediate state before moving back to NONE. Next
// state: NONE.
// PREEMPT_TRIGGERED: A preemption has been executed on the hardware. Next
// states: FAULTED, PENDING
// PREEMPT_FAULTED: A preemption timed out (never completed). This will trigger
// recovery.  Next state: N/A
// PREEMPT_PENDING: Preemption complete interrupt fired - the callback is
// checking the success of the operation. Next state: FAULTED, NONE.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum preempt_state {
    PREEMPT_NONE = 0,
    PREEMPT_START,
    PREEMPT_ABORT,
    PREEMPT_TRIGGERED,
    PREEMPT_FAULTED,
    PREEMPT_PENDING,
}

//
// struct a5xx_preempt_record is a shared buffer between the microcode and the
// CPU to store the state for preemption. The record itself is much larger
// (64k) but most of that is used by the CP for storage.
//
// There is a preemption record assigned per ringbuffer. When the CPU triggers a
// preemption, it fills out the record with the useful information (wptr, ring
// base, etc) and the microcode uses that information to set up the CP following
// the preemption.  When a ring is switched out, the CP will save the ringbuffer
// state back to the record. In this way, once the records are properly set up
// the CPU can quickly switch back and forth between ringbuffers by only
// updating a few registers (often only the wptr).
//
// These are the CPU aware registers in the record:
// @magic: Must always be 0x27C4BAFC
// @info: Type of the record - written 0 by the CPU, updated by the CP
// @data: Data field from SET_RENDER_MODE or a checkpoint. Written and used by
// the CP
// @cntl: Value of RB_CNTL written by CPU, save/restored by CP
// @rptr: Value of RB_RPTR written by CPU, save/restored by CP
// @wptr: Value of RB_WPTR written by CPU, save/restored by CP
// @rptr_addr: Value of RB_RPTR_ADDR written by CPU, save/restored by CP
// @rbase: Value of RB_BASE written by CPU, save/restored by CP
// @counter: GPU address of the storage area for the performance counters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a5xx_preempt_record {
    pub magic: u32,
    pub info: u32,
    pub data: u32,
    pub cntl: u32,
    pub rptr: u32,
    pub wptr: u32,
    pub rptr_addr: u64,
    pub rbase: u64,
    pub counter: u64,
}

// Magic identifier for the preemption record
pub const A5XX_PREEMPT_RECORD_MAGIC: c_uint = 0x27C4BAFCUL;
//
// Even though the structure above is only a few bytes, we need a full 64k to
// store the entire preemption record from the CP
//

//
// The preemption counter block is a storage area for the value of the
// preemption counters that are saved immediately before context switch. We
// append it on to the end of the allocation for the preemption record.
//

extern "C" {
    pub fn a5xx_power_init(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn a5xx_gpmu_ucode_init(gpu: *mut msm_gpu);
}

extern "C" {
    pub fn a5xx_idle(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer) -> bool;
}
extern "C" {
    pub fn a5xx_set_hwcg(gpu: *mut msm_gpu, state: bool);
}
extern "C" {
    pub fn a5xx_preempt_init(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a5xx_preempt_hw_init(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a5xx_preempt_trigger(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a5xx_preempt_irq(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a5xx_preempt_fini(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a5xx_flush(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer, sync: bool);
}
// Return true if we are in a preempt state
