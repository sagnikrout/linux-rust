//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/intel/qat/qat_common/adf_cfg_common.h
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


// SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only)
// Copyright(c) 2014 - 2020 Intel Corporation

pub const ADF_CFG_MAX_STR_LEN: c_int = 64;

pub const ADF_CFG_SERV_RING_PAIR_0_SHIFT: c_int = 0;
pub const ADF_CFG_SERV_RING_PAIR_1_SHIFT: c_int = 3;
pub const ADF_CFG_SERV_RING_PAIR_2_SHIFT: c_int = 6;
pub const ADF_CFG_SERV_RING_PAIR_3_SHIFT: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_cfg_service_type {
    UNUSED = 0,
    CRYPTO,
    COMP,
    SYM,
    ASYM,
    DECOMP,
    USED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_cfg_val_type {
    ADF_DEC,
    ADF_STR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum adf_device_type {
    DEV_UNKNOWN = 0,
    DEV_DH895XCC,
    DEV_DH895XCCVF,
    DEV_C62X,
    DEV_C62XVF,
    DEV_C3XXX,
    DEV_C3XXXVF,
    DEV_4XXX,
    DEV_420XX,
    DEV_6XXX,
}
