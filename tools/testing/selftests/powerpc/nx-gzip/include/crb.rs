//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/powerpc/nx-gzip/include/crb.h
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

// CCW 842 CI/FC masks
// NX P8 workbook, section 4.3.1, figure 4-6
// "CI/FC Boundary by NX CT type"
//

// Chapter 6.5.8 Coprocessor-Completion Block (CCB)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coprocessor_completion_block {
    pub value: __be64,
    pub address: __be64,
    pub __aligned(CCB_ALIGN): },
// Chapter 6.5.7 Coprocessor-Status Block (CSB)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coprocessor_status_block {
    pub flags: __u8,
    pub cs: __u8,
    pub cc: __u8,
    pub ce: __u8,
    pub count: __be32,
    pub address: __be64,
    pub __aligned(CSB_ALIGN): },
// Chapter 6.5.10 Data-Descriptor List (DDL)
// each list contains one or more Data-Descriptor Entries (DDE)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_descriptor_entry {
    pub flags: __be16,
    pub count: __u8,
    pub index: __u8,
    pub length: __be32,
    pub address: __be64,
    pub __aligned(DDE_ALIGN): },
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
    pub reserved: [__u8; 48],
    pub csb: coprocessor_status_block,
    pub __aligned(CRB_ALIGN): },

// RFC02167 Initiate Coprocessor Instructions document
// Chapter 8.2.1.1.1 RS
// Chapter 8.2.3 Coprocessor Directive
// Chapter 8.2.4 Execution
//
// The CCW must be converted to BE before passing to icswx()
//

