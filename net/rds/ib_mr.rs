//! Automatically rewritten from C Header to Rust Module
//! Source: net/rds/ib_mr.h
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


//
// Copyright (c) 2016 Oracle.  All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const RDS_MR_1M_MSG_SIZE: c_int = 256;
pub const RDS_MR_8K_MSG_SIZE: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rds_ib_fr_state {
    FRMR_IS_FREE,	/* mr invalidated & ready for use */
    FRMR_IS_INUSE,	/* mr is in use or used & can be invalidated */
    FRMR_IS_STALE,	/* Stale MR and needs to be dropped  */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_frmr {
    pub mr: *mut ib_mr,
    pub fr_state: rds_ib_fr_state,
    pub fr_inv: bool,
    pub fr_inv_done: wait_queue_head_t,
    pub fr_reg: bool,
    pub fr_reg_done: wait_queue_head_t,
    pub fr_wr: ib_send_wr,
    pub dma_npages: c_uint,
    pub sg_byte_len: c_uint,
}

// This is stored as mr->r_trans_private.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_mr {
    pub work: delayed_work,
    pub device: *mut rds_ib_device,
    pub pool: *mut rds_ib_mr_pool,
    pub ic: *mut rds_ib_connection,
    pub llnode: llist_node,
// unmap_list is for freeing
    pub unmap_list: list_head,
    pub remap_count: c_uint,
    pub sg: *mut scatterlist,
    pub sg_len: c_uint,
    pub sg_dma_len: c_int,
    pub odp:1: u8,
    pub frmr: rds_ib_frmr,
    pub mr: *mut ib_mr,
    pub u: },
}

// Our own little MR pool
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rds_ib_mr_pool {
    pub pool_type: c_uint,
    pub /: *mut *mut mutex flush_lock; / serialize fmr invalidate,
    pub /: *mut *mut delayed_work flush_worker; / flush worker,
    pub /: *mut *mut atomic_t item_count; / total # of MRs,
    pub /: *mut *mut atomic_t dirty_count; / # dirty of MRs,
    pub /: *mut *mut llist_head drop_list; / MRs not reached max_maps,
    pub /: *mut *mut llist_head free_list; / unused MRs,
    pub /: *mut *mut llist_head clean_list; / unused & unmapped MRs,
    pub flush_wait: wait_queue_head_t,
    pub /: *mut *mut spinlock_t clean_lock; / "clean_list" concurrency,
    pub /: *mut *mut atomic_t free_pinned; / memory pinned by free MRs,
    pub max_items: c_ulong,
    pub max_items_soft: c_ulong,
    pub max_free_pinned: c_ulong,
    pub max_pages: c_uint,
}

extern "C" {
    pub fn rds_ib_destroy_mr_pool(: *mut rds_ib_mr_pool);
}
extern "C" {
    pub fn rds_ib_sync_mr(trans_private: *mut c_void, dir: c_int);
}
extern "C" {
    pub fn rds_ib_free_mr(trans_private: *mut c_void, invalidate: c_int);
}
extern "C" {
    pub fn rds_ib_flush_mrs();
}
extern "C" {
    pub fn rds_ib_mr_init() -> c_int;
}
extern "C" {
    pub fn rds_ib_mr_exit();
}
extern "C" {
    pub fn rds_ib_get_lkey(trans_private: *mut c_void) -> u32;
}
extern "C" {
    pub fn __rds_ib_teardown_mr(: *mut rds_ib_mr);
}
extern "C" {
    pub fn rds_ib_teardown_mr(: *mut rds_ib_mr);
}
extern "C" {
    pub fn rds_ib_flush_mr_pool(: *mut rds_ib_mr_pool, _arg: c_int, : *mut rds_ib_mr) -> c_int;
}
extern "C" {
    pub fn rds_ib_free_frmr_list(: *mut rds_ib_mr);
}
