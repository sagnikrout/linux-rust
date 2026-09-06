//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/siw/siw_mem.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Bernard Metzler <bmt@zurich.ibm.com>
// Copyright (c) 2008-2019, IBM Corporation
extern "C" {
    pub fn siw_umem_release(umem: *mut siw_umem);
}
extern "C" {
    pub fn siw_pbl_get_buffer(pbl: *mut siw_pbl, off: u64, len: *mut c_int, idx: *mut c_int) -> dma_addr_t;
}
extern "C" {
    pub fn siw_invalidate_stag(pd: *mut ib_pd, stag: u32) -> c_int;
}
extern "C" {
    pub fn siw_wqe_put_mem(wqe: *mut siw_wqe, op: siw_opcode);
}
extern "C" {
    pub fn siw_mr_drop_mem(mr: *mut siw_mr);
}
extern "C" {
    pub fn siw_free_mem(ref: *mut kref);
}
// mem = NULL;

//
// siw_get_upage()
//
// Get page pointer for address on given umem.
//
// @umem: two dimensional list of page pointers
// @addr: user virtual address
//
