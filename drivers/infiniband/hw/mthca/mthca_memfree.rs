//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mthca/mthca_memfree.h
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
// Copyright (c) 2004, 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Cisco Systems.  All rights reserved.
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_icm_chunk {
    pub list: list_head,
    pub npages: c_int,
    pub nsg: c_int,
    pub mem: [scatterlist; MTHCA_ICM_CHUNK_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_icm {
    pub chunk_list: list_head,
    pub refcount: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_icm_table {
    pub virt: u64,
    pub num_icm: c_int,
    pub num_obj: c_int,
    pub obj_size: c_int,
    pub lowmem: c_int,
    pub coherent: c_int,
    pub mutex: mutex,
    pub __counted_by(num_icm): *mut *mut mthca_icm icm[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_icm_iter {
    pub icm: *mut mthca_icm,
    pub chunk: *mut mthca_icm_chunk,
    pub page_idx: c_int,
}

extern "C" {
    pub fn mthca_free_icm(dev: *mut mthca_dev, icm: *mut mthca_icm, coherent: c_int);
}
extern "C" {
    pub fn mthca_free_icm_table(dev: *mut mthca_dev, table: *mut mthca_icm_table);
}
extern "C" {
    pub fn mthca_table_get(dev: *mut mthca_dev, table: *mut mthca_icm_table, obj: c_int) -> c_int;
}
extern "C" {
    pub fn mthca_table_put(dev: *mut mthca_dev, table: *mut mthca_icm_table, obj: c_int);
}
extern "C" {
    pub fn sg_dma_address(_arg: &iter->chunk->mem[iter->page_idx]) -> return;
}
extern "C" {
    pub fn sg_dma_len(_arg: &iter->chunk->mem[iter->page_idx]) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_db_page {
    pub MTHCA_DB_REC_PER_PAGE): DECLARE_BITMAP(used,,
    pub db_rec: *mut __be64,
    pub mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_db_table {
    pub npages: c_int,
    pub max_group1: c_int,
    pub min_group2: c_int,
    pub page: *mut mthca_db_page,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mthca_db_type {
    MTHCA_DB_TYPE_INVALID   = 0x0,
    MTHCA_DB_TYPE_CQ_SET_CI = 0x1,
    MTHCA_DB_TYPE_CQ_ARM    = 0x2,
    MTHCA_DB_TYPE_SQ        = 0x3,
    MTHCA_DB_TYPE_RQ        = 0x4,
    MTHCA_DB_TYPE_SRQ       = 0x5,
    MTHCA_DB_TYPE_GROUP_SEP = 0x7
}

extern "C" {
    pub fn mthca_init_db_tab(dev: *mut mthca_dev) -> c_int;
}
extern "C" {
    pub fn mthca_cleanup_db_tab(dev: *mut mthca_dev);
}
extern "C" {
    pub fn mthca_free_db(dev: *mut mthca_dev, type: c_int, db_index: c_int);
}
