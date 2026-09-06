//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_umem.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2007 Cisco Systems.  All rights reserved.
// Copyright (c) 2020 Intel Corporation.  All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umem {
    pub ibdev: *mut ib_device,
    pub owning_mm: *mut mm_struct,
    pub iova: u64,
    pub length: usize,
    pub address: c_ulong,
    pub dma_attrs: c_ulong,
    pub 1: u32 writable :,
    pub 1: u32 is_odp :,
    pub 1: u32 is_dmabuf :,
    pub sgt_append: sg_append_table,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umem_dmabuf {
    pub umem: ib_umem,
    pub attach: *mut dma_buf_attachment,
    pub sgt: *mut sg_table,
    pub first_sg: *mut scatterlist,
    pub last_sg: *mut scatterlist,
    pub first_sg_offset: c_ulong,
    pub last_sg_trim: c_ulong,
    pub priv): *mut *mut void (pinned_revoke)(void,
    pub private: *mut c_void,
    pub 1: u8 pinned :,
    pub 1: u8 revoked :,
}

extern "C" {
    pub fn container_of(_arg: umem, ib_umem_dmabuf: struct, _arg: umem) -> return;
}
// Returns the offset of the umem start relative to the first page.
extern "C" {
    pub fn sg_dma_address(ib_umem_offset(umem: umem->sgt_append.sgt.sgl) +) -> return;
}
extern "C" {
    pub fn ib_umem_start_dma_addr(1: umem) & (pgsz -) -> return;
}
extern "C" {
    pub fn ib_umem_num_dma_blocks(_arg: umem, _arg: PAGE_SIZE) -> return;
}

extern "C" {
    pub fn ib_umem_get_attr_or_va(_arg: device, _arg: NULL, _arg: 0, _arg: addr, _arg: size, _arg: access) -> return;
}
extern "C" {
    pub fn ib_umem_release(umem: *mut ib_umem);
}
//
// ib_umem_find_best_pgoff - Find best HW page size
//
// @umem: umem struct
// @pgsz_bitmap: bitmap of HW supported page sizes
// @pgoff_bitmask: Mask of bits that can be represented with an offset
//
// This is very similar to ib_umem_find_best_pgsz() except instead of accepting
// an IOVA it accepts a bitmask specifying what address bits can be represented
// with a page offset.
//
// For instance if the HW has multiple page sizes, requires 64 byte alignemnt,
// and can support aligned offsets up to 4032 then pgoff_bitmask would be
// "111111000000".
//
// If the pgoff_bitmask requires either alignment in the low bit or an
// unavailable page size for the high bits, this function returns 0.
//
// Returns: best HW page size for the parameters or 0 if none available
// for the given parameters.
//
extern "C" {
    pub fn ib_umem_dmabuf_map_pages(umem_dmabuf: *mut ib_umem_dmabuf) -> c_int;
}
extern "C" {
    pub fn ib_umem_dmabuf_unmap_pages(umem_dmabuf: *mut ib_umem_dmabuf);
}
extern "C" {
    pub fn ib_umem_dmabuf_release(umem_dmabuf: *mut ib_umem_dmabuf);
}
extern "C" {
    pub fn ib_umem_dmabuf_revoke_lock(umem_dmabuf: *mut ib_umem_dmabuf);
}
extern "C" {
    pub fn ib_umem_dmabuf_revoke_unlock(umem_dmabuf: *mut ib_umem_dmabuf);
}
extern "C" {
    pub fn ib_umem_dmabuf_revoke(umem_dmabuf: *mut ib_umem_dmabuf);
}
extern "C" {
    pub fn ib_umem_check_rereg(umem: *mut ib_umem, flags: c_int, new_access_flags: c_int) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

