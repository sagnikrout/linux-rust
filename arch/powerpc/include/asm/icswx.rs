//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/icswx.h
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
// ICSWX api
//
// Copyright (C) 2015 IBM Corp.
//
// This provides the Initiate Coprocessor Store Word Indexed (ICSWX)
// instruction.  This instruction is used to communicate with PowerPC
// coprocessors.  This also provides definitions of the structures used
// to communicate with the coprocessor.
//
// The RFC02130: Coprocessor Architecture document is the reference for
// everything in this file unless otherwise noted.
//

// Chapter 6.5.8 Coprocessor-Completion Block (CCB)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coprocessor_completion_block {
    pub value: __be64,
    pub address: __be64,
    pub __aligned(CCB_ALIGN): } __packed,
// Chapter 6.5.7 Coprocessor-Status Block (CSB)

// P9 DD2 NX Workbook 3.2 (Table 4-36): Address translation fault

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coprocessor_status_block {
    pub flags: u8,
    pub cs: u8,
    pub cc: u8,
    pub ce: u8,
    pub count: __be32,
    pub address: __be64,
    pub __aligned(CSB_ALIGN): } __packed,
// Chapter 6.5.10 Data-Descriptor List (DDL)
// each list contains one or more Data-Descriptor Entries (DDE)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_descriptor_entry {
    pub flags: __be16,
    pub count: u8,
    pub index: u8,
    pub length: __be32,
    pub address: __be64,
    pub __aligned(DDE_ALIGN): } __packed,
// 4.3.2 NX-stamped Fault CRB

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx_fault_stamp {
    pub fault_storage_addr: __be64,
    pub reserved: __be16,
    pub flags: __u8,
    pub fault_status: __u8,
    pub pswid: __be32,
    pub __aligned(NX_STAMP_ALIGN): } __packed,
// Chapter 6.5.2 Coprocessor-Request Block (CRB)

// Coprocessor Status Block field
// ADDRESS	address of CSB
// C		CCB is valid
// AT		0 = addrs are virtual, 1 = addrs are phys
// M		enable perf monitor
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coprocessor_request_block {
    pub ccw: __be32,
    pub flags: __be32,
    pub csb_addr: __be64,
    pub source: data_descriptor_entry,
    pub target: data_descriptor_entry,
    pub ccb: coprocessor_completion_block,
    pub nx: nx_fault_stamp,
    pub reserved: [u8; 16],
    pub stamp: },
    pub reserved: [u8; 32],
    pub csb: coprocessor_status_block,
    pub __aligned(128): },
// RFC02167 Initiate Coprocessor Instructions document
// Chapter 8.2.1.1.1 RS
// Chapter 8.2.3 Coprocessor Directive
// Chapter 8.2.4 Execution
//
// The CCW must be converted to BE before passing to icswx()
//

// RFC02167 Initiate Coprocessor Instructions document
// Chapter 8.2.1 Initiate Coprocessor Store Word Indexed (ICSWX)
// Chapter 8.2.4.1 Condition Register 0
//

    pub ccw: __be64 ccw_reg =,
    pub cr: u32,
// NB: the same structures are used by VAS-NX
    pub 128): *mut *mut BUILD_BUG_ON(sizeof(crb) !=,
    pub "memory"): : "cr0",,
    pub 0xf): return (int)((cr >> 28) &,
