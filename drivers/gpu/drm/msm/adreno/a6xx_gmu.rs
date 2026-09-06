//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/adreno/a6xx_gmu.h
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
// Copyright (c) 2017 The Linux Foundation. All rights reserved.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_gmu_bo {
    pub obj: *mut drm_gem_object,
    pub virt: *mut c_void,
    pub size: usize,
    pub iova: u64,
}

pub const GMU_MAX_GX_FREQS: c_int = 32;
pub const GMU_MAX_CX_FREQS: c_int = 6;
pub const GMU_MAX_BCMS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_bcm {
    pub name: *mut c_char,
    pub buswidth: c_uint,
    pub fixed: bool,
    pub perfmode: c_uint,
    pub perfmode_bw: c_uint,
}

//
// These define the different GMU wake up options - these define how both the
// CPU and the GMU bring up the hardware
//
// THe GMU has already been booted and the rentention registers are active
pub const GMU_WARM_BOOT: c_int = 0;
// the GMU is coming up for the first time or back from a power collapse
pub const GMU_COLD_BOOT: c_int = 1;
//
// These define the level of control that the GMU has - the higher the number
// the more things that the GMU hardware controls on its own.
//
// The GMU does not do any idle state management
pub const GMU_IDLE_STATE_ACTIVE: c_int = 0;
// Unknown power state. Not exposed by the firmware. For documentation purpose only
pub const GMU_IDLE_STATE_RESERVED: c_int = 1;
// The GMU manages SPTP power collapse
pub const GMU_IDLE_STATE_SPTP: c_int = 2;
// The GMU does automatic IFPC (intra-frame power collapse)
pub const GMU_IDLE_STATE_IFPC: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct a6xx_gmu {
    pub dev: *mut device,
// For serializing communication with the GMU:
    pub lock: mutex,
    pub vm: *mut drm_gpuvm,
    pub mmio: *mut void __iomem,
    pub mmio_offset: u32,
    pub rscc: *mut void __iomem,
    pub hfi_irq: c_int,
    pub gmu_irq: c_int,
    pub gxpd: *mut device,
    pub cxpd: *mut device,
    pub idle_level: c_int,
    pub hfi: a6xx_gmu_bo,
    pub debug: a6xx_gmu_bo,
    pub icache: a6xx_gmu_bo,
    pub dcache: a6xx_gmu_bo,
    pub dummy: a6xx_gmu_bo,
    pub log: a6xx_gmu_bo,
    pub nr_clocks: c_int,
    pub clocks: *mut clk_bulk_data,
    pub core_clk: *mut clk,
    pub hub_clk: *mut clk,
// current performance index set externally
    pub current_perf_index: c_int,
    pub nr_gpu_freqs: c_int,
    pub gpu_freqs: [c_ulong; GMU_MAX_GX_FREQS],
    pub gx_arc_votes: [u32; GMU_MAX_GX_FREQS],
    pub dep_arc_votes: [u32; GMU_MAX_GX_FREQS],
    pub acd_table: a6xx_hfi_acd_table,
    pub nr_gpu_bws: c_int,
    pub gpu_bw_table: [c_ulong; GMU_MAX_GX_FREQS],
    pub gpu_ib_votes: [u32; GMU_MAX_GX_FREQS][GMU_MAX_BCMS],
    pub nr_gmu_freqs: c_int,
    pub gmu_freqs: [c_ulong; GMU_MAX_CX_FREQS],
    pub cx_arc_votes: [u32; GMU_MAX_CX_FREQS],
    pub freq: c_ulong,
    pub queues: [a6xx_hfi_queue; HFI_MAX_QUEUES],
    pub initialized: bool,
    pub hung: bool,
    pub /: *mut *mut bool legacy; / a618 or a630,
// For power domain callback
    pub pd_nb: notifier_block,
    pub pd_gate: completion,
    pub qmp: *mut qmp,
    pub bw_table: *mut a6xx_hfi_msg_bw_table,
// To check if we can trigger sleep seq at PDC. Cleared in a6xx_rpmh_stop()
pub const GMU_STATUS_FW_START: c_int = 0;
// To track if PDC sleep seq was done
pub const GMU_STATUS_PDC_SLEEP: c_int = 1;
// To track Perfcounter OOB set status
pub const GMU_STATUS_OOB_PERF_SET: c_int = 2;
// To track whether secure world init was done
pub const GMU_STATUS_SECURE_INIT: c_int = 3;
    pub status: c_ulong,
}

// The 'offset' is based on GPU's start address. Adjust it
extern "C" {
    pub fn readl(GMU_BYTE_OFFSET(gmu: gmu->mmio +, _arg: offset)) -> return;
}

extern "C" {
    pub fn readl(2): gmu->rscc + (offset <<) -> return;
}

//
// These are the available OOB (out of band requests) to the GMU where "out of
// band" means that the CPU talks to the GMU directly and not through HFI.
// Normally this works by writing a ITCM/DTCM register and then triggering a
// interrupt (the "request" bit) and waiting for an acknowledgment (the "ack"
// bit). The state is cleared by writing the "clear' bit to the GMU interrupt.
//
// These are used to force the GMU/GPU to stay on during a critical sequence or
// for hardware workarounds.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum a6xx_gmu_oob_state {
//
// Let the GMU know that a boot or slumber operation has started. The value in
// REG_A6XX_GMU_BOOT_SLUMBER_OPTION lets the GMU know which operation we are
// doing
//
    GMU_OOB_BOOT_SLUMBER = 0,
//
// Let the GMU know to not turn off any GPU registers while the CPU is in a
// critical section
//
    GMU_OOB_GPU_SET,
//
// Set a new power level for the GPU when the CPU is doing frequency scaling
//
    GMU_OOB_DCVS_SET,
//
// Used to keep the GPU on for CPU-side reads of performance counters.
//
    GMU_OOB_PERFCOUNTER_SET,
}

extern "C" {
    pub fn a6xx_hfi_init(gmu: *mut a6xx_gmu);
}
extern "C" {
    pub fn a6xx_hfi_start(gmu: *mut a6xx_gmu, boot_state: c_int) -> c_int;
}
extern "C" {
    pub fn a6xx_hfi_stop(gmu: *mut a6xx_gmu);
}
extern "C" {
    pub fn a6xx_hfi_send_prep_slumber(gmu: *mut a6xx_gmu) -> c_int;
}
extern "C" {
    pub fn a6xx_hfi_set_freq(gmu: *mut a6xx_gmu, perf_index: u32, bw_index: u32) -> c_int;
}
extern "C" {
    pub fn a6xx_gmu_gx_is_on(adreno_gpu: *mut adreno_gpu) -> bool;
}
extern "C" {
    pub fn a7xx_gmu_gx_is_on(adreno_gpu: *mut adreno_gpu) -> bool;
}
extern "C" {
    pub fn a8xx_gmu_gx_is_on(adreno_gpu: *mut adreno_gpu) -> bool;
}
extern "C" {
    pub fn a6xx_gmu_sptprac_is_on(gmu: *mut a6xx_gmu) -> bool;
}
extern "C" {
    pub fn a6xx_sptprac_disable(gmu: *mut a6xx_gmu);
}
extern "C" {
    pub fn a6xx_sptprac_enable(gmu: *mut a6xx_gmu) -> c_int;
}
