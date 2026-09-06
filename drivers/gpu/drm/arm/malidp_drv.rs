//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/arm/malidp_drv.h
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
//
// (C) COPYRIGHT 2016 ARM Limited. All rights reserved.
// Author: Liviu Dudau <Liviu.Dudau@arm.com>
//
// ARM Mali DP500/DP550/DP650 KMS/DRM driver structures
//

pub const MALIDP_CONFIG_VALID_INIT: c_int = 0;
pub const MALIDP_CONFIG_VALID_DONE: c_int = 1;
pub const MALIDP_CONFIG_START: c_uint = 0xd0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_error_stats {
    pub num_errors: i32,
    pub last_error_status: u32,
    pub last_error_vblank: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_drm {
    pub base: drm_device,
    pub dev: *mut malidp_hw_device,
    pub crtc: drm_crtc,
    pub mw_connector: drm_writeback_connector,
    pub wq: wait_queue_head_t,
    pub event: *mut drm_pending_vblank_event,
    pub config_valid: core::sync::atomic::AtomicI32,
    pub core_id: u32,

    pub de_errors: malidp_error_stats,
    pub se_errors: malidp_error_stats,
// Protects errors stats
    pub errors_lock: spinlock_t,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_plane {
    pub base: drm_plane,
    pub hwdev: *mut malidp_hw_device,
    pub layer: *const malidp_layer,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmu_prefetch_mode {
    MALIDP_PREFETCH_MODE_NONE,
    MALIDP_PREFETCH_MODE_PARTIAL,
    MALIDP_PREFETCH_MODE_FULL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_plane_state {
    pub base: drm_plane_state,
// size of the required rotation memory if plane is rotated
    pub rotmem_size: u32,
// internal format ID
    pub format: u8,
    pub n_planes: u8,
    pub mmu_prefetch_mode: mmu_prefetch_mode,
    pub mmu_prefetch_pgsize: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct malidp_crtc_state {
    pub base: drm_crtc_state,
    pub gamma_coeffs: [u32; MALIDP_COEFFTAB_NUM_COEFFS],
    pub coloradj_coeffs: [u32; MALIDP_COLORADJ_NUM_COEFFS],
    pub scaler_config: malidp_se_config,
// Bitfield of all the planes that have requested a scaled output.
    pub scaled_planes_mask: u8,
}

extern "C" {
    pub fn malidp_de_planes_init(drm: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn malidp_crtc_init(drm: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn malidp_hw_format_is_linear_only(format: u32) -> bool;
}
extern "C" {
    pub fn malidp_hw_format_is_afbc_only(format: u32) -> bool;
}

// often used combination of rotational bits

