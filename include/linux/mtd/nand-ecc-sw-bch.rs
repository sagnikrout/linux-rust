//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mtd/nand-ecc-sw-bch.h
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
// Copyright © 2011 Ivan Djelic <ivan.djelic@parrot.com>
//
// This file is the header for the NAND BCH ECC implementation.
//

//
// struct nand_ecc_sw_bch_conf - private software BCH ECC engine structure
// @req_ctx: Save request context and tweak the original request to fit the
// engine needs
// @code_size: Number of bytes needed to store a code (one code per step)
// @calc_buf: Buffer to use when calculating ECC bytes
// @code_buf: Buffer to use when reading (raw) ECC bytes from the chip
// @bch: BCH control structure
// @errloc: error location array
// @eccmask: XOR ecc mask, allows erased pages to be decoded as valid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nand_ecc_sw_bch_conf {
    pub req_ctx: nand_ecc_req_tweak_ctx,
    pub code_size: c_uint,
    pub calc_buf: *mut u8,
    pub code_buf: *mut u8,
    pub bch: *mut bch_control,
    pub errloc: *mut c_uint,
    pub eccmask: *mut c_uchar,
}

extern "C" {
    pub fn nand_ecc_sw_bch_init_ctx(nand: *mut nand_device) -> c_int;
}
extern "C" {
    pub fn nand_ecc_sw_bch_cleanup_ctx(nand: *mut nand_device);
}

