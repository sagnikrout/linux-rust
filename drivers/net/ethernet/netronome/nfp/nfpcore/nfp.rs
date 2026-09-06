//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2015-2018 Netronome Systems, Inc.
//
// nfp.h
// Interface for NFP device access and query functions.
//

// Implemented in nfp_hwinfo.c
extern "C" {
    pub fn nfp_hwinfo_get_packed_str_size(hwinfo: *mut nfp_hwinfo) -> u32;
}
// Implemented in nfp_nsp.c, low level functions
extern "C" {
    pub fn nfp_nsp_config_modified(state: *mut nfp_nsp) -> bool;
}
extern "C" {
    pub fn nfp_nsp_config_set_modified(state: *mut nfp_nsp, modified: bool);
}
extern "C" {
    pub fn nfp_nsp_config_idx(state: *mut nfp_nsp) -> c_uint;
}
extern "C" {
    pub fn nfp_nsp_config_clear_state(state: *mut nfp_nsp);
}
extern "C" {
    pub fn nfp_nsp_read_eth_table(state: *mut nfp_nsp, buf: *mut c_void, size: c_uint) -> c_int;
}
extern "C" {
    pub fn nfp_nsp_read_identify(state: *mut nfp_nsp, buf: *mut c_void, size: c_uint) -> c_int;
}
// Implemented in nfp_resource.c
// All keys are CRC32-POSIX of the 8-byte identification string
// ARM/PCI vNIC Interfaces 0..3

// NFP Hardware Info Database

// Service Processor

// Netronone Flow Firmware Table

// MAC Statistics Accumulator

extern "C" {
    pub fn nfp_resource_table_init(cpp: *mut nfp_cpp) -> c_int;
}
extern "C" {
    pub fn nfp_resource_release(res: *mut nfp_resource);
}
extern "C" {
    pub fn nfp_resource_wait(cpp: *mut nfp_cpp, name: *const c_char, secs: c_uint) -> c_int;
}
extern "C" {
    pub fn nfp_resource_cpp_id(res: *mut nfp_resource) -> u32;
}
extern "C" {
    pub fn nfp_resource_address(res: *mut nfp_resource) -> u64;
}
extern "C" {
    pub fn nfp_resource_size(res: *mut nfp_resource) -> u64;
}
