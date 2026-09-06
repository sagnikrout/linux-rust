//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/edac/mce_amd.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tt_ids {
    TT_INSTR = 0,
    TT_DATA,
    TT_GEN,
    TT_RESV,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ll_ids {
    LL_RESV = 0,
    LL_L1,
    LL_L2,
    LL_LG,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ii_ids {
    II_MEM = 0,
    II_RESV,
    II_IO,
    II_GEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rrrr_ids {
    R4_GEN	= 0,
    R4_RD,
    R4_WR,
    R4_DRD,
    R4_DWR,
    R4_IRD,
    R4_PREF,
    R4_EVICT,
    R4_SNOOP,
}

//
// per-family decoder ops
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_decoder_ops {
    pub u8): *mut *mut bool (mc0_mce)(u16,,
    pub u8): *mut *mut bool (mc1_mce)(u16,,
    pub u8): *mut *mut bool (mc2_mce)(u16,,
}

extern "C" {
    pub fn amd_register_ecc_decoder((*f)(int: *mut c_void, ): *mut mce);
}
extern "C" {
    pub fn amd_unregister_ecc_decoder((*f)(int: *mut c_void, ): *mut mce);
}
