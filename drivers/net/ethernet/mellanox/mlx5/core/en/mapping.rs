//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/mapping.h
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
// Copyright (c) 2019 Mellanox Technologies
extern "C" {
    pub fn mapping_add(ctx: *mut mapping_ctx, data: *mut c_void, id: *mut u32) -> c_int;
}
extern "C" {
    pub fn mapping_remove(ctx: *mut mapping_ctx, id: u32) -> c_int;
}
extern "C" {
    pub fn mapping_find(ctx: *mut mapping_ctx, id: u32, data: *mut c_void) -> c_int;
}
// mapping uses an xarray to map data to ids in add(), and for find().
// For locking, it uses a internal xarray spin lock for add()/remove(),
// find() uses rcu_read_lock().
// Choosing delayed_removal postpones the removal of a previously mapped
// id by MAPPING_GRACE_PERIOD milliseconds.
// This is to avoid races against hardware, where we mark the packet in
// hardware with a previous id, and quick remove() and add() reusing the same
// previous id. Then find() will get the new mapping instead of the old
// which was used to mark the packet.
//
extern "C" {
    pub fn mapping_destroy(ctx: *mut mapping_ctx);
}
// adds mapping with an id or get an existing mapping with the same id
//
