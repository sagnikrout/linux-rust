//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/ttm/ttm_kmap_iter.h
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2021 Intel Corporation
//

//
// struct ttm_kmap_iter_ops - Ops structure for a struct
// ttm_kmap_iter.
// @maps_tt: Whether the iterator maps TT memory directly, as opposed
// mapping a TT through an aperture. Both these modes have
// struct ttm_resource_manager::use_tt set, but the latter typically
// returns is_iomem == true from ttm_mem_io_reserve.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_kmap_iter_ops {
//
// @map_local: Map a PAGE_SIZE part of the resource using
// kmap_local semantics.
// @res_iter: Pointer to the struct ttm_kmap_iter representing
// the resource.
// @dmap: The struct iosys_map holding the virtual address after
// the operation.
// @i: The location within the resource to map. PAGE_SIZE granularity.
//
    pub i): *mut *mut iosys_map dmap, pgoff_t,
//
// @unmap_local: Unmap a PAGE_SIZE part of the resource previously
// mapped using kmap_local.
// @res_iter: Pointer to the struct ttm_kmap_iter representing
// the resource.
// @dmap: The struct iosys_map holding the virtual address after
// the operation.
//
    pub dmap): *mut iosys_map,
    pub maps_tt: bool,
}

//
// struct ttm_kmap_iter - Iterator for kmap_local type operations on a
// resource.
// @ops: Pointer to the operations struct.
//
// This struct is intended to be embedded in a resource-specific specialization
// implementing operations for the resource.
//
// Nothing stops us from extending the operations to vmap, vmap_pfn etc,
// replacing some or parts of the ttm_bo_util. cpu-map functionality.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ttm_kmap_iter {
    pub ops: *const ttm_kmap_iter_ops,
}
