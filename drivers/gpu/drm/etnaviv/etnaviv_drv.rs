//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/etnaviv/etnaviv_drv.h
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
pub struct etnaviv_file_private {
    pub id: c_int,
    pub mmu: *mut etnaviv_iommu_context,
    pub sched_entity: [drm_sched_entity; ETNA_MAX_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct etnaviv_drm_private {
    pub num_gpus: c_int,
    pub gpu: [*mut etnaviv_gpu; ETNA_MAX_PIPES],
    pub shm_gfp_mask: gfp_t,
    pub cmdbuf_suballoc: *mut etnaviv_cmdbuf_suballoc,
    pub mmu_global: *mut etnaviv_iommu_global,
    pub active_contexts: xarray,
    pub next_context_id: u32,
// list of GEM objects:
    pub gem_lock: mutex,
    pub gem_list: list_head,
// ppu flop reset data
    pub flop_reset_data_ppu: *mut etnaviv_cmdbuf,
}

extern "C" {
    pub fn etnaviv_gem_mmap_offset(obj: *mut drm_gem_object, offset: *mut u64) -> c_int;
}
extern "C" {
    pub fn etnaviv_gem_prime_vmap(obj: *mut drm_gem_object, map: *mut iosys_map) -> c_int;
}
extern "C" {
    pub fn etnaviv_gem_prime_pin(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn etnaviv_gem_prime_unpin(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn etnaviv_gem_cpu_fini(obj: *mut drm_gem_object) -> c_int;
}
extern "C" {
    pub fn etnaviv_gem_free_object(obj: *mut drm_gem_object);
}
extern "C" {
    pub fn etnaviv_buffer_init(gpu: *mut etnaviv_gpu) -> u16;
}
extern "C" {
    pub fn etnaviv_buffer_config_mmuv2(gpu: *mut etnaviv_gpu, mtlb_addr: u32, safe_addr: u32) -> u16;
}
extern "C" {
    pub fn etnaviv_buffer_config_pta(gpu: *mut etnaviv_gpu, id: c_ushort) -> u16;
}
extern "C" {
    pub fn etnaviv_buffer_end(gpu: *mut etnaviv_gpu);
}
extern "C" {
    pub fn etnaviv_sync_point_queue(gpu: *mut etnaviv_gpu, event: c_uint);
}
extern "C" {
    pub fn etnaviv_validate_init();
}

//
// Etnaviv timeouts are specified wrt CLOCK_MONOTONIC, not jiffies.
// We need to calculate the timeout in terms of number of jiffies
// between the specified timeout and the current CLOCK_MONOTONIC time.
//
// timeouts before "now" have already expired
extern "C" {
    pub fn timespec64_to_jiffies(_arg: &ts) -> return;
}
