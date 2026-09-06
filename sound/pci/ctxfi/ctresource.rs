//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctresource.h
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
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File	ctresource.h
//
// @Brief
// This file contains the definition of generic hardware resources for
// resource management.
//
// @Author	Liu Chun
// @Date 	May 13 2008
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RSCTYP {
    SRC,
    SRCIMP,
    AMIXER,
    SUM,
    DAIO,
    NUM_RSCTYP	/* This must be the last one and less than 16 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsc {
    pub /: *mut *mut u32 idx:12; / The index of a resource,
    pub /: *mut *mut u32 type:4; / The type (RSCTYP) of a resource,
    pub /: *mut *mut u32 conj:12; / Current conjugate index,
    pub /: *mut *mut u32 msr:4; / The Master Sample Rate a resource working on,
    pub /: *mut *mut *mut void ctrl_blk; / Chip specific control info block for a resource,
    pub /: *mut *mut *mut hw hw; / Chip specific object for hardware access means,
    pub /: *const *const *const rsc_ops ops; / Generic resource operations,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsc_ops {
    pub /: *mut *mut *mut *mut void (master)(struct rsc rsc); / Move to master resource,
    pub /: *mut *mut *mut *mut void (next_conj)(struct rsc rsc); / Move to next conjugate resource,
    pub /: *const *const *const *const int (index)(struct rsc rsc); / Return the index of resource,
// Return the output slot number
    pub rsc): *const *const int (output_slot)(struct rsc,
}

extern "C" {
    pub fn rsc_uninit(rsc: *mut rsc) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rsc_mgr {
    pub /: *mut *mut RSCTYP type; / The type (RSCTYP) of resource to manage,
    pub /: *mut *mut unsigned int amount; / The total amount of a kind of resource,
    pub /: *mut *mut unsigned int avail; / The amount of currently available resources,
    pub /: *mut *mut *mut unsigned char rscs; / The bit-map for resource allocation,
    pub /: *mut *mut *mut void ctrl_blk; / Chip specific control info block,
    pub /: *mut *mut *mut hw hw; / Chip specific object for hardware access,
}

// Resource management is based on bit-map mechanism
extern "C" {
    pub fn rsc_mgr_uninit(mgr: *mut rsc_mgr) -> c_int;
}
extern "C" {
    pub fn mgr_get_resource(mgr: *mut rsc_mgr, n: c_uint, ridx: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn mgr_put_resource(mgr: *mut rsc_mgr, n: c_uint, idx: c_uint) -> c_int;
}
