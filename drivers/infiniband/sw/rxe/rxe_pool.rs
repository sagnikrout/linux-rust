//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/sw/rxe/rxe_pool.h
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
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rxe_elem_type {
    RXE_TYPE_UC,
    RXE_TYPE_PD,
    RXE_TYPE_AH,
    RXE_TYPE_SRQ,
    RXE_TYPE_QP,
    RXE_TYPE_CQ,
    RXE_TYPE_MR,
    RXE_TYPE_MW,
    RXE_NUM_TYPES,		/* keep me last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_pool_elem {
    pub pool: *mut rxe_pool,
    pub obj: *mut c_void,
    pub ref_cnt: kref,
    pub list: list_head,
    pub complete: completion,
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rxe_pool {
    pub rxe: *mut rxe_dev,
    pub name: *const c_char,
    pub elem): *mut *mut void (cleanup)(struct rxe_pool_elem,
    pub type: rxe_elem_type,
    pub max_elem: c_uint,
    pub num_elem: core::sync::atomic::AtomicI32,
    pub elem_size: usize,
    pub elem_offset: usize,
    pub xa: xarray,
    pub limit: xa_limit,
    pub next: u32,
}

// initialize a pool of objects with given limit on
// number of elements. gets parameters from rxe_type_info
// pool elements will be allocated out of a slab cache
//
// free resources from object pool
extern "C" {
    pub fn rxe_pool_cleanup(pool: *mut rxe_pool);
}
// connect already allocated object to pool

// lookup an indexed object from index. takes a reference on object
extern "C" {
    pub fn __rxe_get(elem: *mut rxe_pool_elem) -> c_int;
}

extern "C" {
    pub fn __rxe_put(elem: *mut rxe_pool_elem) -> c_int;
}

extern "C" {
    pub fn __rxe_cleanup(elem: *mut rxe_pool_elem, sleepable: bool) -> c_int;
}

extern "C" {
    pub fn __rxe_finalize(elem: *mut rxe_pool_elem);
}

