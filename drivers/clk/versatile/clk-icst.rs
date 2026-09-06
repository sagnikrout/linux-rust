//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/clk/versatile/clk-icst.h
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
// enum icst_control_type - the type of ICST control register
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icst_control_type {
    ICST_VERSATILE, /* The standard type, all control bits available */
    ICST_INTEGRATOR_AP_CM, /* Only 8 bits of VDW available */
    ICST_INTEGRATOR_AP_SYS, /* Only 8 bits of VDW available */
    ICST_INTEGRATOR_AP_PCI, /* Odd bit pattern storage */
    ICST_INTEGRATOR_CP_CM_CORE, /* Only 8 bits of VDW and 3 bits of OD */
    ICST_INTEGRATOR_CP_CM_MEM, /* Only 8 bits of VDW and 3 bits of OD */
    ICST_INTEGRATOR_IM_PD1, /* Like the Versatile, all control bits */
}

//
// struct clk_icst_desc - descriptor for the ICST VCO
// @params: ICST parameters
// @vco_offset: offset to the ICST VCO from the provided memory base
// @lock_offset: offset to the ICST VCO locking register from the provided
// memory base
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clk_icst_desc {
    pub params: *const icst_params,
    pub vco_offset: u32,
    pub lock_offset: u32,
}
