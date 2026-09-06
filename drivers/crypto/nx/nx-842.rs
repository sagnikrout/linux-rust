//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/crypto/nx/nx-842.h
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

// Restrictions on Data Descriptor List (DDL) and Entry (DDE) buffers
//
// From NX P8 workbook, sec 4.9.1 "842 details"
// Each DDE buffer is 128 byte aligned
// Each DDE buffer size is a multiple of 32 bytes (except the last)
// The last DDE buffer size is a multiple of 8 bytes
//

// Arbitrary DDL length limit
// Allows max buffer size of MAX-1 to MAX pages
// (depending on alignment)
//

// CCW 842 CI/FC masks
// NX P8 workbook, section 4.3.1, figure 4-6
// "CI/FC Boundary by NX CT type"
//

// CCW Function Codes (FC) for 842
// NX P8 workbook, section 4.9, table 4-28
// "Function Code Definitions for 842 Memory Compression"
//

// CSB CC Error Types for 842
// NX P8 workbook, section 4.10.3, table 4-30
// "Reported Error Types Summary Table"
//
// These are all duplicates of existing codes defined in icswx.h.

// These are specific to NX
// 842 codes

// sym crypt codes

// asym crypt codes

//
// HW error - Job did not finish in the maximum time allowed.
// Job terminated.
//

// These are reserved for hypervisor use

// No valid interrupt server (P9 or later).

// CCB Completion Modes (CM) for 842
// NX P8 workbook, section 4.3, figure 4-5
// "CRB Details - Normal Cop_Req (CL=00, C=1)"
//

extern "C" {
    pub fn __pa(_arg: addr) -> return;
}
extern "C" {
    pub fn page_to_phys(offset_in_page(addr: vmalloc_to_page(addr)) +) -> return;
}
//
// This provides the driver's constraints.  Different nx842 implementations
// may have varying requirements.  The constraints are:
// @alignment:	All buffers should be aligned to this
// @multiple:		All buffer lengths should be a multiple of this
// @minimum:		Buffer lengths must not be less than this amount
// @maximum:		Buffer lengths must not be more than this amount
//
// The constraints apply to all buffers and lengths, both input and output,
// for both compression and decompression, except for the minimum which
// only applies to compression input and decompression output; the
// compressed data can be less than the minimum constraint.  It can be
// assumed that compressed data will always adhere to the multiple
// constraint.
//
// The driver may succeed even if these constraints are violated;
// however the driver can return failure or suffer reduced performance
// if any constraint is not met.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx842_constraints {
    pub alignment: c_int,
    pub multiple: c_int,
    pub minimum: c_int,
    pub maximum: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx842_driver {
    pub name: *mut c_char,
    pub owner: *mut module,
    pub workmem_size: usize,
    pub constraints: *mut nx842_constraints,
    pub wrkmem): *mut c_void,
    pub wrkmem): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx842_crypto_header_group {
    pub /: *mut *mut __be16 padding; / unused bytes at start of group,
    pub /: *mut *mut __be32 compressed_length; / compressed bytes in group,
    pub /: *mut *mut __be32 uncompressed_length; / bytes after decompression,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx842_crypto_header {
// New members MUST be added within the struct_group() macro below.
    pub /: *mut *mut __be16 magic; / NX842_CRYPTO_MAGIC,
    pub /: *mut *mut __be16 ignore; / decompressed end bytes to ignore,
    pub /: *mut *mut u8 groups; / total groups in this header,
    pub __counted_by(groups): nx842_crypto_header_group group[],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nx842_crypto_ctx {
    pub lock: spinlock_t,
    pub wmem: *mut u8,
    pub dbounce: *mut *mut u8 sbounce,,
    pub header: nx842_crypto_header_hdr,
    pub group: [nx842_crypto_header_group; NX842_CRYPTO_GROUP_MAX],
    pub driver: *mut nx842_driver,
}

extern "C" {
    pub fn nx842_crypto_free_ctx(ctx: *mut c_void);
}
