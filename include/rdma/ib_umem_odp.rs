//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_umem_odp.h
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
// Copyright (c) 2014 Mellanox Technologies. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umem_odp {
    pub umem: ib_umem,
    pub notifier: mmu_interval_notifier,
    pub tgid: *mut pid,
    pub map: hmm_dma_map,
//
// The umem_mutex protects the page_list field of an ODP
// umem, allowing only a single thread to map/unmap pages. The mutex
// also protects access to the mmu notifier counters.
//
    pub umem_mutex: mutex,
    pub /: *mut *mut *mut void private; / for the HW driver to use.,
    pub npages: c_int,
//
// An implicit odp umem cannot be DMA mapped, has 0 length, and serves
// only as an anchor for the driver to hold onto the per_mm. FIXME:
// This should be removed and drivers should work with the per_mm
// directly.
//
    pub is_implicit_odp: bool,
    pub page_shift: c_uint,
}

extern "C" {
    pub fn container_of(_arg: umem, ib_umem_odp: struct, _arg: umem) -> return;
}
// Returns the first page of an ODP umem.
// Returns the address of the page after the last one of an ODP umem.

extern "C" {
    pub fn ib_umem_odp_release(umem_odp: *mut ib_umem_odp);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

