//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/a6xx_gpu.h
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
// Copyright (c) 2017, 2019 The Linux Foundation. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_gpu_lock {
    pub gpu_req: u32,
    pub cpu_req: u32,
    pub turn: u32,
// a6xx:
    pub list_length: u16,
    pub list_offset: u16,
}

// a7xx+:
//
// struct a6xx_info - a6xx specific information from device table
//
// @hwcg: hw clock gating register sequence
// @protect: CP_PROTECT settings
// @pwrup_reglist pwrup reglist for preemption
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_info {
    pub hwcg: *const adreno_reglist,
    pub protect: *const adreno_protect,
    pub pwrup_reglist: *const adreno_reglist_list,
    pub dyn_pwrup_reglist: *const adreno_reglist_pipe_list,
    pub ifpc_reglist: *const adreno_reglist_list,
    pub gbif_cx: *const adreno_reglist,
    pub nonctxt_reglist: *const adreno_reglist_pipe,
    pub max_slices: u32,
    pub gmu_chipid: u32,
    pub gmu_cgc_mode: u32,
    pub prim_fifo_threshold: u32,
    pub bcms: *const a6xx_bcm,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_gpu {
    pub base: adreno_gpu,
    pub sqe_bo: *mut drm_gem_object,
    pub sqe_iova: u64,
    pub aqe_bo: *mut drm_gem_object,
    pub aqe_iova: u64,
    pub cur_ring: *mut msm_ringbuffer,
    pub next_ring: *mut msm_ringbuffer,
    pub preempt_bo: [*mut drm_gem_object; MSM_GPU_MAX_RINGS],
    pub preempt: [*mut c_void; MSM_GPU_MAX_RINGS],
    pub preempt_iova: [u64; MSM_GPU_MAX_RINGS],
    pub preempt_smmu_bo: [*mut drm_gem_object; MSM_GPU_MAX_RINGS],
    pub preempt_smmu: [*mut c_void; MSM_GPU_MAX_RINGS],
    pub preempt_smmu_iova: [u64; MSM_GPU_MAX_RINGS],
    pub last_seqno: [u32; MSM_GPU_MAX_RINGS],
    pub preempt_state: core::sync::atomic::AtomicI32,
    pub eval_lock: spinlock_t,
    pub preempt_timer: timer_list,
    pub preempt_level: c_uint,
    pub uses_gmem: bool,
    pub skip_save_restore: bool,
    pub preempt_postamble_bo: *mut drm_gem_object,
    pub preempt_postamble_ptr: *mut c_void,
    pub preempt_postamble_iova: u64,
    pub preempt_postamble_len: u64,
    pub postamble_enabled: bool,
    pub gmu: a6xx_gmu,
    pub shadow_bo: *mut drm_gem_object,
    pub shadow_iova: u64,
    pub shadow: *mut u32,
    pub pwrup_reglist_bo: *mut drm_gem_object,

    pub pwrup_reglist_ptr: *mut c_void,
    pub pwrup_reglist_iova: u64,
    pub pwrup_reglist_emitted: bool,
//
// Offset of start of SEL regs appended to pwrup_reglist.  This
// is equal to lock->dynamic_list_len if no SEL regs are appended
// to the end of the dynamic reglist.
//
    pub dynamic_sel_reglist_offset: u16,
    pub has_whereami: bool,
    pub cx_misc_mmio: *mut void __iomem,
    pub llc_slice: *mut c_void,
    pub htw_llc_slice: *mut c_void,
    pub have_mmu500: bool,
    pub hung: bool,
    pub cached_aperture: u32,
    pub aperture_lock: spinlock_t,
    pub slice_mask: u32,
}

//
// In order to do lockless preemption we use a simple state machine to progress
// through the process.
//
// PREEMPT_NONE - no preemption in progress.  Next state START.
// PREEMPT_START - The trigger is evaluating if preemption is possible. Next
// states: TRIGGERED, NONE
// PREEMPT_FINISH - An intermediate state before moving back to NONE. Next
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
pub enum a6xx_preempt_state {
    PREEMPT_NONE = 0,
    PREEMPT_START,
    PREEMPT_FINISH,
    PREEMPT_TRIGGERED,
    PREEMPT_FAULTED,
    PREEMPT_PENDING,
}

//
// struct a6xx_preempt_record is a shared buffer between the microcode and the
// CPU to store the state for preemption. The record itself is much larger
// (2112k) but most of that is used by the CP for storage.
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
// @magic: Must always be 0xAE399D6EUL
// @info: Type of the record - written 0 by the CPU, updated by the CP
// @errno: preemption error record
// @data: Data field in YIELD and SET_MARKER packets, Written and used by CP
// @cntl: Value of RB_CNTL written by CPU, save/restored by CP
// @rptr: Value of RB_RPTR written by CPU, save/restored by CP
// @wptr: Value of RB_WPTR written by CPU, save/restored by CP
// @_pad: Reserved/padding
// @rptr_addr: Value of RB_RPTR_ADDR_LO|HI written by CPU, save/restored by CP
// @rbase: Value of RB_BASE written by CPU, save/restored by CP
// @counter: GPU address of the storage area for the preemption counters
// @bv_rptr_addr: Value of BV_RB_RPTR_ADDR_LO|HI written by CPU, save/restored by CP
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_preempt_record {
    pub magic: u32,
    pub info: u32,
    pub errno: u32,
    pub data: u32,
    pub cntl: u32,
    pub rptr: u32,
    pub wptr: u32,
    pub _pad: u32,
    pub rptr_addr: u64,
    pub rbase: u64,
    pub counter: u64,
    pub bv_rptr_addr: u64,
}

pub const A6XX_PREEMPT_RECORD_MAGIC: c_uint = 0xAE399D6EUL;
pub const PREEMPT_SMMU_INFO_SIZE: c_int = 4096;

//
// The preemption counter block is a storage area for the value of the
// preemption counters that are saved immediately before context switch. We
// append it on to the end of the allocation for the preemption record.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a7xx_cp_smmu_info {
    pub magic: u32,
    pub _pad4: u32,
    pub ttbr0: u64,
    pub asid: u32,
    pub context_idr: u32,
    pub context_bank: u32,
}

pub const GEN7_CP_SMMU_INFO_MAGIC: c_uint = 0x241350d5UL;
//
// Given a register and a count, return a value to program into
// REG_CP_PROTECT_REG(n) - this will block both reads and writes for
// _len + 1 registers starting at _reg.
//

//
// Same as above, but allow reads over the range. For areas of mixed use (such
// as performance counters) this allows us to protect a much larger range with a
// single register
//

extern "C" {
    pub fn msm_rmw(2): a6xx_gpu->cx_misc_mmio + (reg <<, _arg: mask, _arg: or) -> return;
}
extern "C" {
    pub fn readl(2): a6xx_gpu->cx_misc_mmio + (reg <<) -> return;
}

extern "C" {
    pub fn a6xx_gmu_resume(gpu: *mut a6xx_gpu) -> c_int;
}
extern "C" {
    pub fn a6xx_gmu_stop(gpu: *mut a6xx_gpu) -> c_int;
}
extern "C" {
    pub fn a6xx_gmu_wait_for_idle(gmu: *mut a6xx_gmu) -> c_int;
}
extern "C" {
    pub fn a6xx_gmu_isidle(gmu: *mut a6xx_gmu) -> bool;
}
extern "C" {
    pub fn a6xx_gmu_set_oob(gmu: *mut a6xx_gmu, state: a6xx_gmu_oob_state) -> c_int;
}
extern "C" {
    pub fn a6xx_gmu_clear_oob(gmu: *mut a6xx_gmu, state: a6xx_gmu_oob_state);
}
extern "C" {
    pub fn a6xx_gmu_init(a6xx_gpu: *mut a6xx_gpu, node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn a6xx_gmu_wrapper_init(a6xx_gpu: *mut a6xx_gpu, node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn a6xx_gmu_remove(a6xx_gpu: *mut a6xx_gpu);
}
extern "C" {
    pub fn a6xx_gmu_sysprof_setup(gpu: *mut msm_gpu, force_on: bool);
}
extern "C" {
    pub fn a6xx_preempt_init(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a6xx_preempt_hw_init(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a6xx_preempt_trigger(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a6xx_preempt_irq(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a6xx_preempt_fini(gpu: *mut msm_gpu);
}
// Return true if we are in a preempt state
//
// Make sure the read to preempt_state is ordered with respect to reads
// of other variables before ...
//
// ... and after.
extern "C" {
    pub fn a6xx_gmu_get_freq(gpu: *mut msm_gpu) -> c_ulong;
}
extern "C" {
    pub fn a6xx_gpu_state_put(state: *mut msm_gpu_state) -> c_int;
}
extern "C" {
    pub fn a6xx_bus_clear_pending_transactions(adreno_gpu: *mut adreno_gpu, gx_off: bool);
}
extern "C" {
    pub fn a6xx_gpu_sw_reset(gpu: *mut msm_gpu, assert: bool);
}
extern "C" {
    pub fn a6xx_fenced_write(gpu: *mut a6xx_gpu, offset: u32, value: u64, mask: u32, is_64b: bool) -> c_int;
}
extern "C" {
    pub fn a6xx_flush(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer);
}
extern "C" {
    pub fn a6xx_flush_yield(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer);
}
extern "C" {
    pub fn a6xx_zap_shader_init(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn a8xx_bus_clear_pending_transactions(adreno_gpu: *mut adreno_gpu, gx_off: bool);
}
extern "C" {
    pub fn a8xx_fault_handler(arg: *mut c_void, iova: c_ulong, flags: c_int, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn a8xx_flush(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer);
}
extern "C" {
    pub fn a8xx_gmu_get_timestamp(gpu: *mut msm_gpu) -> u64;
}
extern "C" {
    pub fn a8xx_gpu_busy(gpu: *mut msm_gpu, out_sample_rate: *mut c_ulong) -> u64;
}
extern "C" {
    pub fn a8xx_gpu_feature_probe(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn a8xx_gpu_get_slice_info(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a8xx_hw_init(gpu: *mut msm_gpu) -> c_int;
}
extern "C" {
    pub fn a8xx_irq(gpu: *mut msm_gpu) -> irqreturn_t;
}
extern "C" {
    pub fn a8xx_llc_activate(a6xx_gpu: *mut a6xx_gpu);
}
extern "C" {
    pub fn a8xx_preempt_hw_init(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a8xx_preempt_trigger(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a8xx_preempt_irq(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a8xx_progress(gpu: *mut msm_gpu, ring: *mut msm_ringbuffer) -> bool;
}
extern "C" {
    pub fn a8xx_perfcntr_flush(gpu: *mut msm_gpu);
}
extern "C" {
    pub fn a8xx_recover(gpu: *mut msm_gpu);
}
