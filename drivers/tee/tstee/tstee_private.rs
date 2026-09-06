//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/tee/tstee/tstee_private.h
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
// Copyright (c) 2023, Arm Limited
//

//
// The description of the ABI implemented in this file is available at
// https://trusted-services.readthedocs.io/en/v1.0.0/developer/service-access-protocols.html#abi
//
// UUID of this protocol

// Protocol version

// Status codes

// RPC control register

// Interface ID for RPC management operations

// Management calls

// Service call

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstee {
    pub ffa_dev: *mut ffa_device,
    pub teedev: *mut tee_device,
    pub pool: *mut tee_shm_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_session {
    pub iface_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ts_context_data {
    pub sess_list: xarray,
}
