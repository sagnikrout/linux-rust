//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/display/drm_dp_tunnel.h
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
// Copyright © 2023 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_tunnel_ref {
    pub tunnel: *mut drm_dp_tunnel,
    pub tracker: *mut ref_tracker,
}

extern "C" {
    pub fn drm_dp_tunnel_destroy(tunnel: *mut drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_enable_bw_alloc(tunnel: *mut drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_disable_bw_alloc(tunnel: *mut drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_bw_alloc_is_enabled(tunnel: *const drm_dp_tunnel) -> bool;
}
extern "C" {
    pub fn drm_dp_tunnel_pr_optimization_supported(tunnel: *const drm_dp_tunnel) -> bool;
}
extern "C" {
    pub fn drm_dp_tunnel_alloc_bw(tunnel: *mut drm_dp_tunnel, bw: c_int) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_get_allocated_bw(tunnel: *mut drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_update_state(tunnel: *mut drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_set_io_error(tunnel: *mut drm_dp_tunnel);
}
extern "C" {
    pub fn drm_dp_tunnel_128b132b_supported(tunnel: *const drm_dp_tunnel) -> bool;
}
extern "C" {
    pub fn drm_dp_tunnel_128b132b_lane0_mapping_supported(tunnel: *const drm_dp_tunnel) -> bool;
}
extern "C" {
    pub fn drm_dp_tunnel_128b132b_dprx_rates(tunnel: *const drm_dp_tunnel) -> u8;
}
extern "C" {
    pub fn drm_dp_tunnel_max_dprx_rate(tunnel: *const drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_max_dprx_lane_count(tunnel: *const drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_available_bw(tunnel: *const drm_dp_tunnel) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_atomic_get_required_bw(tunnel_state: *const drm_dp_tunnel_state) -> c_int;
}
extern "C" {
    pub fn drm_dp_tunnel_mgr_destroy(mgr: *mut drm_dp_tunnel_mgr);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

