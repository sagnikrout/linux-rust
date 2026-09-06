//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_top/dml2_top_interfaces.c
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

#[no_mangle]
pub unsafe extern "C" fn dml2_get_instance_size_bytes() -> c_uint {
    unsigned int dml2_get_instance_size_bytes(void)
    {
    return sizeof(struct dml2_instance);
    }
#[no_mangle]
pub unsafe extern "C" fn dml2_initialize_instance(in_out: *mut dml2_initialize_instance_in_out) -> bool {
    bool dml2_initialize_instance(struct dml2_initialize_instance_in_out *in_out)
    {
    switch (in_out.options.project_id) {
    case dml2_project_dcn4x_stage1:
    case dml2_project_dcn4x_stage2:
    case dml2_project_dcn4x_stage2_auto_drr_svp:
    case dml2_project_dcn42:
    return dml2_top_soc15_initialize_instance(in_out);
    case dml2_project_dcn5x_utm:
    case dml2_project_dcn6x_soc_var_a:
    case dml2_project_dcn6x_soc_var_b:
    return dml2_top_utm_initialize_instance(in_out);
    case dml2_project_dcn4x_utm:
    case dml2_project_dcn5x:
    case dml2_project_invalid:
    default:
    return false;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dml2_check_mode_supported(in_out: *mut dml2_check_mode_supported_in_out) -> bool {
    bool dml2_check_mode_supported(struct dml2_check_mode_supported_in_out *in_out)
    {
    if (!in_out.dml2_instance.funcs.check_mode_supported)
    return false;
    return in_out.dml2_instance.funcs.check_mode_supported(in_out);
    }
#[no_mangle]
pub unsafe extern "C" fn dml2_build_mode_programming(in_out: *mut dml2_build_mode_programming_in_out) -> bool {
    bool dml2_build_mode_programming(struct dml2_build_mode_programming_in_out *in_out)
    {
    if (!in_out.dml2_instance.funcs.build_mode_programming)
    return false;
    return in_out.dml2_instance.funcs.build_mode_programming(in_out);
    }
#[no_mangle]
pub unsafe extern "C" fn dml2_build_mcache_programming(in_out: *mut dml2_build_mcache_programming_in_out) -> bool {
    bool dml2_build_mcache_programming(struct dml2_build_mcache_programming_in_out *in_out)
    {
    if (!in_out.dml2_instance.funcs.build_mcache_programming)
    return false;
    return in_out.dml2_instance.funcs.build_mcache_programming(in_out);
    }
