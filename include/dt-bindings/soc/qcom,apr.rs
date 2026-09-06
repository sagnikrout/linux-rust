//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/soc/qcom,apr.h
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
// Domain IDs
pub const APR_DOMAIN_SIM: c_uint = 0x1;
pub const APR_DOMAIN_PC: c_uint = 0x2;
pub const APR_DOMAIN_MODEM: c_uint = 0x3;
pub const APR_DOMAIN_ADSP: c_uint = 0x4;
pub const APR_DOMAIN_APPS: c_uint = 0x5;
pub const APR_DOMAIN_MAX: c_uint = 0x6;
// ADSP service IDs
pub const APR_SVC_ADSP_CORE: c_uint = 0x3;
pub const APR_SVC_AFE: c_uint = 0x4;
pub const APR_SVC_VSM: c_uint = 0x5;
pub const APR_SVC_VPM: c_uint = 0x6;
pub const APR_SVC_ASM: c_uint = 0x7;
pub const APR_SVC_ADM: c_uint = 0x8;
pub const APR_SVC_ADSP_MVM: c_uint = 0x09;
pub const APR_SVC_ADSP_CVS: c_uint = 0x0A;
pub const APR_SVC_ADSP_CVP: c_uint = 0x0B;
pub const APR_SVC_USM: c_uint = 0x0C;
pub const APR_SVC_LSM: c_uint = 0x0D;
pub const APR_SVC_VIDC: c_uint = 0x16;
pub const APR_SVC_MAX: c_uint = 0x17;
