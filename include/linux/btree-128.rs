//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/btree-128.h
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


// SPDX-License-Identifier: GPL-2.0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btree_head128 {
    pub mempool): btree_init_mempool(&head->h,,
    pub btree_init(&head->h): return,
    pub k2}: u64 key[2] = {k1,,
    pub )&key): *mut return btree_lookup(&head->h, &btree_geo128, (unsigned long,
    pub k2}: *mut *mut u64 key[2] = {k1,,
    pub val: *mut c_void,
    pub )&key): *mut (unsigned long,
// k1 = key[0];
// k2 = key[1];
    pub val: return,
    pub k2}: u64 key[2] = {k1,,
    pub gfp): *mut *mut (unsigned long )&key, val,,
    pub k2}: u64 key[2] = {k1,,
    pub val): *mut *mut (unsigned long )&key,,
    pub k2}: u64 key[2] = {k1,,
    pub )&key): *mut return btree_remove(&head->h, &btree_geo128, (unsigned long,
    pub key: [u64; 2],
    pub val: *mut c_void,
    pub )&key[0]): *mut val = btree_last(&head->h, &btree_geo128, (unsigned long,
// k1 = key[0];
// k2 = key[1];
    pub val: return,
    pub gfp): return btree_merge(&target->h, &victim->h, &btree_geo128,,
    pub __func): *mut size_t index, void,
    pub index): u64 key1, u64 key2, size_t,
    pub func2): visitor128,,
    pub func2): visitor128,,

    pub \: for (val = btree_last128(head, &k1, &k2);,
    pub \: val;,
