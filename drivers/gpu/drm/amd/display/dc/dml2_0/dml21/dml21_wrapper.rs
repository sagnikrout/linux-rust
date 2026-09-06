//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/dml21_wrapper.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

//
// dml21_create - Creates dml21_context.
// @in_dc: dc.
// @dml_ctx: Created dml21 context.
// @config: dml21 configuration options.
//
// Create of DML21 is done as part of dc_state creation.
// DML21 IP, SOC and STATES are initialized at
// creation time.
//
// Return: True if dml2 is successfully created, false otherwise.
//
extern "C" {
    pub fn dml21_create(in_dc: *const dc, dml_ctx: *mut dml2_context, config: *const dml2_configuration_options) -> bool;
}
extern "C" {
    pub fn dml21_destroy(dml2: *mut dml2_context);
}
// Structure for inputting external SOCBB and DCNIP values for tool based debugging.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socbb_ip_params_external {
    pub ip_params: dml2_ip_capabilities,
    pub soc_bb: dml2_soc_bb,
}

// mcache parameters decided by dml
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_mcache_params {
    pub valid: bool,
//
// For iMALL, dedicated mall mcaches are required (sharing of last
// slice possible), for legacy phantom or phantom without return
// the only mall mcaches need to be valid.
//
    pub requires_dedicated_mall_mcache: bool,
    pub num_mcaches_plane0: c_uint,
    pub num_mcaches_plane1: c_uint,
//
// Generally, plane0/1 slices must use a disjoint set of caches
// but in some cases the final segement of the two planes can
// use the same cache. If plane0_plane1 is set, then this is
// allowed.
//
// Similarly, the caches allocated to MALL prefetcher are generally
// disjoint, but if mall_prefetch is set, then the final segment
// between the main and the mall pixel requestor can use the same
// cache.
//
// Note that both bits may be set at the same time.
//
    pub mall_comb_mcache_p0: bool,
    pub mall_comb_mcache_p1: bool,
    pub plane0_plane1: bool,
    pub last_slice_sharing: },
//
// A plane is divided into vertical slices of mcaches,
// which wrap on the surface width.
//
// For example, if the surface width is 7680, and split into
// three slices of equal width, the boundary array would contain
// [2560, 5120, 7680]
//
// The assignments are
// 0 = [0 .. 2559]
// 1 = [2560 .. 5119]
// 2 = [5120 .. 7679]
// 0 = [7680 .. INF]
// The final element implicitly is the same as the first, and
// at first seems invalid since it is never referenced (since)
// it is outside the surface. However, its useful when shifting
// (see below).
//
// For any given valid mcache assignment, a shifted version, wrapped
// on the surface width boundary is also assumed to be valid.
//
// For example, shifting [2560, 5120, 7680] by -50 results in
// [2510, 5170, 7630].
//
// The assignments are now:
// 0 = [0 .. 2509]
// 1 = [2510 .. 5169]
// 2 = [5170 .. 7629]
// 0 = [7630 .. INF]
//
    pub 1]: int mcache_x_offsets_plane0[DML2_MAX_MCACHES +,
    pub 1]: int mcache_x_offsets_plane1[DML2_MAX_MCACHES +,
}
