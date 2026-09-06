//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/mpc/dcn42/dcn42_mpc.h
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
// Copyright 2026 Advanced Micro Devices, Inc.

// Macro flag: #define MPC_RMCM_REG_LIST_DCN42(inst)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_mpc_shift {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_mpc_mask {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_mpc_registers {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn42_mpc {
    pub base: mpc,
    pub mpcc_in_use_mask: c_int,
    pub num_mpcc: c_int,
    pub mpc_regs: *const dcn42_mpc_registers,
    pub mpc_shift: *const dcn42_mpc_shift,
    pub mpc_mask: *const dcn42_mpc_mask,
    pub num_rmu: c_int,
}

extern "C" {
    pub fn mpc42_init_mpcc(mpcc: *mut mpcc, mpcc_inst: c_int);
}
// RMCM
