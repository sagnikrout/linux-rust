//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/drmem.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// drmem.h: Power specific logical memory block representation
//
// Copyright 2017 IBM Corporation
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drmem_lmb {
    pub base_addr: u64,
    pub drc_index: u32,
    pub aa_index: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drmem_lmb_info {
    pub lmbs: *mut drmem_lmb,
    pub n_lmbs: c_int,
    pub lmb_size: u64,
}

//
// DLPAR code paths can take several milliseconds per element
// when interacting with firmware. Ensure that we don't
// unfairly monopolize the CPU.
//

//
// The of_drconf_cell_v1 struct defines the layout of the LMB data
// specified in the ibm,dynamic-memory device tree property.
// The property itself is a 32-bit value specifying the number of
// LMBs followed by an array of of_drconf_cell_v1 entries, one
// per LMB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_drconf_cell_v1 {
    pub base_addr: __be64,
    pub drc_index: __be32,
    pub reserved: __be32,
    pub aa_index: __be32,
    pub flags: __be32,
}

//
// Version 2 of the ibm,dynamic-memory property is defined as a
// 32-bit value specifying the number of LMB sets followed by an
// array of of_drconf_cell_v2 entries, one per LMB set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct of_drconf_cell_v2 {
    pub seq_lmbs: u32,
    pub base_addr: u64,
    pub drc_index: u32,
    pub aa_index: u32,
    pub flags: u32,
    pub __packed: },
pub const DRCONF_MEM_ASSIGNED: c_uint = 0x00000008;
pub const DRCONF_MEM_AI_INVALID: c_uint = 0x00000040;
pub const DRCONF_MEM_RESERVED: c_uint = 0x00000080;
pub const DRCONF_MEM_HOTREMOVABLE: c_uint = 0x00000100;
    pub drmem_info->lmb_size: return,
pub const DRMEM_LMB_RESERVED: c_uint = 0x80000000;
    pub DRMEM_LMB_RESERVED: lmb->flags |=,
    pub ~DRMEM_LMB_RESERVED: lmb->flags &=,
    pub DRMEM_LMB_RESERVED: return lmb->flags &,
    pub drmem_lmb_memory_max(void): u64,
    pub )): *const *const *const *const *const int (func)(struct drmem_lmb , __be32 , void,
    pub drmem_update_dt(void): c_int,

    pub )): *const *const *const *const *const int (func)(struct drmem_lmb , __be32 , void,
    pub prop): *mut void drmem_update_lmbs(struct property,

    pub 0xffffffff: lmb->aa_index =,
