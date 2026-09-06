//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/appldata.h
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
// Copyright IBM Corp. 2006
//
// Author(s): Melissa Howland <melissah@us.ibm.com>
//

pub const APPLDATA_START_INTERVAL_REC: c_uint = 0x80;
pub const APPLDATA_STOP_REC: c_uint = 0x81;
pub const APPLDATA_GEN_EVENT_REC: c_uint = 0x82;
pub const APPLDATA_START_CONFIG_REC: c_uint = 0x83;
//
// Parameter list for DIAGNOSE X'DC'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appldata_parameter_list {
    pub diag: u16,
    pub function: u8,
    pub parlist_length: u8,
    pub unused01: u32,
    pub reserved: u16,
    pub buffer_length: u16,
    pub unused02: u32,
    pub product_id_addr: u64,
    pub buffer_addr: u64,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct appldata_product_id {
    pub /: *mut *mut char prod_nr[7]; / product number,
    pub /: *mut *mut u16 prod_fn; / product function,
    pub /: *mut *mut u8 record_nr; / record number,
    pub /: *mut *mut u16 version_nr; / version,
    pub /: *mut *mut u16 release_nr; / release,
    pub /: *mut *mut u16 mod_lvl; / modification level,
// C attribute field omitted
    pub ry: c_int,
    pub -EOPNOTSUPP: return,
    pub 0xdc: parm_list->diag =,
    pub fn: parm_list->function =,
    pub sizeof(*parm_list): *mut parm_list->parlist_length =,
    pub length: parm_list->buffer_length =,
    pub virt_to_phys(id): parm_list->product_id_addr =,
    pub virt_to_phys(buffer): parm_list->buffer_addr =,
    pub "cc"): :,
    pub ry: return,
