//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_pool.c
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2023 Microchip Technology Inc. and its subsidiaries.
//

#[no_mangle]
unsafe extern "C" fn sparx5_pool_id_to_idx(id: u32) -> u32 {
    static u32 sparx5_pool_id_to_idx(u32 id)
    {
    return --id;
    }
#[no_mangle]
pub unsafe extern "C" fn sparx5_pool_idx_to_id(idx: u32) -> u32 {
    u32 sparx5_pool_idx_to_id(u32 idx)
    {
    return ++idx;
    }
// Release resource from pool.
// Return reference count on success, otherwise return error.
//
#[no_mangle]
pub unsafe extern "C" fn sparx5_pool_put(pool: *mut sparx5_pool_entry, size: c_int, id: u32) -> c_int {
    int sparx5_pool_put(struct sparx5_pool_entry *pool, int size, u32 id)
    {
    struct sparx5_pool_entry *e_itr;
    e_itr = (pool + sparx5_pool_id_to_idx(id));
    if (e_itr.ref_cnt == 0)
    return -EINVAL;
    return --e_itr.ref_cnt;
    }
// Get resource from pool.
// Return reference count on success, otherwise return error.
//
#[no_mangle]
pub unsafe extern "C" fn sparx5_pool_get(pool: *mut sparx5_pool_entry, size: c_int, id: *mut u32) -> c_int {
    int sparx5_pool_get(struct sparx5_pool_entry *pool, int size, u32 *id)
    {
    struct sparx5_pool_entry *e_itr;
    int i;
    for (i = 0, e_itr = pool; i < size; i++, e_itr++) {
    if (e_itr.ref_cnt == 0) {
// id = sparx5_pool_idx_to_id(i);
    return ++e_itr.ref_cnt;
    }
    }
    return -ENOSPC;
    }
// Get resource from pool that matches index.
// Return reference count on success, otherwise return error.
//
    int sparx5_pool_get_with_idx(struct sparx5_pool_entry *pool, int size, u32 idx,
    u32 *id)
    {
    struct sparx5_pool_entry *e_itr;
    int i, ret = -ENOSPC;
    for (i = 0, e_itr = pool; i < size; i++, e_itr++) {
// Pool index of first free entry
    if (e_itr.ref_cnt == 0 && ret == -ENOSPC)
    ret = i;
// Tc index already in use ?
    if (e_itr.idx == idx && e_itr.ref_cnt > 0) {
    ret = i;
    break;
    }
    }
// Did we find a free entry?
    if (ret >= 0) {
// id = sparx5_pool_idx_to_id(ret);
    e_itr = (pool + ret);
    e_itr.idx = idx;
    return ++e_itr.ref_cnt;
    }
    return ret;
    }
