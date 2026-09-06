//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/etnaviv/etnaviv_gpu.h
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
// Copyright (C) 2015-2018 Etnaviv Project
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_chip_identity {
    pub model: u32,
    pub revision: u32,
    pub product_id: u32,
    pub customer_id: u32,
    pub eco_id: u32,
// Supported feature fields.
    pub features: u32,
// Supported minor feature fields.
    pub minor_features0: u32,
    pub minor_features1: u32,
    pub minor_features2: u32,
    pub minor_features3: u32,
    pub minor_features4: u32,
    pub minor_features5: u32,
    pub minor_features6: u32,
    pub minor_features7: u32,
    pub minor_features8: u32,
    pub minor_features9: u32,
    pub minor_features10: u32,
    pub minor_features11: u32,
// Number of streams supported.
    pub stream_count: u32,
// Total number of temporary registers per thread.
    pub register_max: u32,
// Maximum number of threads.
    pub thread_count: u32,
// Number of shader cores.
    pub shader_core_count: u32,
// Number of Neural Network cores.
    pub nn_core_count: u32,
// Size of the vertex cache.
    pub vertex_cache_size: u32,
// Number of entries in the vertex output buffer.
    pub vertex_output_buffer_size: u32,
// Number of pixel pipes.
    pub pixel_pipes: u32,
// Number of instructions.
    pub instruction_count: u32,
// Number of constants.
    pub num_constants: u32,
// Buffer size
    pub buffer_size: u32,
// Number of varyings
    pub varyings_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etnaviv_sec_mode {
    ETNA_SEC_NONE = 0,
    ETNA_SEC_KERNEL,
    ETNA_SEC_TZ
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_event {
    pub fence: *mut dma_fence,
    pub submit: *mut etnaviv_gem_submit,
    pub event): *mut *mut *mut void (sync_point)(struct etnaviv_gpu gpu, struct etnaviv_event,
}

pub const ETNA_NR_EVENTS: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etnaviv_gpu_state {
    ETNA_GPU_STATE_UNKNOWN = 0,
    ETNA_GPU_STATE_IDENTIFIED,
    ETNA_GPU_STATE_RESET,
    ETNA_GPU_STATE_INITIALIZED,
    ETNA_GPU_STATE_RUNNING,
    ETNA_GPU_STATE_FAULT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_gpu {
    pub drm: *mut drm_device,
    pub cooling: *mut thermal_cooling_device,
    pub dev: *mut device,
    pub lock: mutex,
    pub identity: etnaviv_chip_identity,
    pub sec_mode: etnaviv_sec_mode,
    pub wq: *mut workqueue_struct,
    pub sched_lock: mutex,
    pub sched: drm_gpu_scheduler,
    pub state: etnaviv_gpu_state,
// 'ring'-buffer:
    pub buffer: etnaviv_cmdbuf,
    pub exec_state: c_int,
// event management:
    pub ETNA_NR_EVENTS): DECLARE_BITMAP(event_bitmap,,
    pub event: [etnaviv_event; ETNA_NR_EVENTS],
    pub event_free: completion,
    pub event_spinlock: spinlock_t,
    pub idle_mask: u32,
// Fencing support
    pub user_fences: xarray,
    pub next_user_fence: u32,
    pub next_fence: u32,
    pub completed_fence: u32,
    pub fence_event: wait_queue_head_t,
    pub fence_context: u64,
    pub fence_spinlock: spinlock_t,
// worker for handling 'sync' points:
    pub sync_point_work: work_struct,
    pub sync_point_event: c_int,
// hang detection
    pub hangcheck_dma_addr: u32,
    pub hangcheck_primid: u32,
    pub hangcheck_fence: u32,
    pub mmio: *mut void __iomem,
    pub irq: c_int,
    pub mmu_context: *mut etnaviv_iommu_context,
    pub flush_seq: c_uint,
// Power Control:
    pub clk_bus: *mut clk,
    pub clk_reg: *mut clk,
    pub clk_core: *mut clk,
    pub clk_shader: *mut clk,
    pub rst: *mut reset_control,
    pub freq_scale: c_uint,
    pub fe_waitcycles: c_uint,
    pub base_rate_core: c_ulong,
    pub base_rate_shader: c_ulong,
}

// On some variants, such as the GC7000r6009, some FE registers
// need two reads to be consistent. Do that extra read here and
// throw away the result.
//
extern "C" {
    pub fn readl(reg: gpu->mmio +) -> return;
}
// Power registers in GC300 < 2.0 are offset by 0x100
extern "C" {
    pub fn readl(gpu_fix_power_address(gpu: gpu->mmio +, _arg: reg)) -> return;
}
extern "C" {
    pub fn etnaviv_gpu_get_param(gpu: *mut etnaviv_gpu, param: u32, value: *mut u64) -> c_int;
}
extern "C" {
    pub fn etnaviv_gpu_init(gpu: *mut etnaviv_gpu) -> c_int;
}
extern "C" {
    pub fn etnaviv_fill_identity_from_hwdb(gpu: *mut etnaviv_gpu) -> bool;
}

extern "C" {
    pub fn etnaviv_gpu_debugfs(gpu: *mut etnaviv_gpu, m: *mut seq_file) -> c_int;
}

extern "C" {
    pub fn etnaviv_gpu_recover_hang(submit: *mut etnaviv_gem_submit);
}
extern "C" {
    pub fn etnaviv_gpu_retire(gpu: *mut etnaviv_gpu);
}
extern "C" {
    pub fn etnaviv_gpu_pm_get_sync(gpu: *mut etnaviv_gpu) -> c_int;
}
extern "C" {
    pub fn etnaviv_gpu_pm_put(gpu: *mut etnaviv_gpu);
}
extern "C" {
    pub fn etnaviv_gpu_wait_idle(gpu: *mut etnaviv_gpu, timeout_ms: c_uint) -> c_int;
}
extern "C" {
    pub fn etnaviv_gpu_start_fe(gpu: *mut etnaviv_gpu, address: u32, prefetch: u16);
}
