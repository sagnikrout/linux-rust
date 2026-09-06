//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/objagg.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2018 Mellanox Technologies. All rights reserved
#[repr(C)]
#[derive(Copy, Clone)]
pub struct objagg_ops {
    pub obj_size: usize,
    pub obj): *const c_void,
    pub obj): *mut *mut *mut *mut *mut void  (delta_create)(void priv, void parent_obj, void,
    pub delta_priv): *mut *mut *mut void (delta_destroy)(void priv, void,
    pub root_id): *mut *mut *mut *mut *mut void  (root_create)(void priv, void obj, unsigned int,

    pub root_priv): *mut *mut *mut void (root_destroy)(void priv, void,
}

extern "C" {
    pub fn objagg_obj_put(objagg: *mut objagg, objagg_obj: *mut objagg_obj);
}
extern "C" {
    pub fn objagg_destroy(objagg: *mut objagg);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct objagg_obj_stats {
    pub user_count: c_uint,
    pub /: *mut *mut unsigned int delta_user_count; / includes delta object users,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct objagg_obj_stats_info {
    pub stats: objagg_obj_stats,
    pub /: *mut *mut *mut objagg_obj objagg_obj; / associated object,
    pub is_root: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct objagg_stats {
    pub root_count: c_uint,
    pub stats_info_count: c_uint,
    pub stats_info: [objagg_obj_stats_info; ],
}

extern "C" {
    pub fn objagg_stats_put(objagg_stats: *const objagg_stats);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum objagg_opt_algo_type {
    OBJAGG_OPT_ALGO_SIMPLE_GREEDY,
}

extern "C" {
    pub fn objagg_hints_put(objagg_hints: *mut objagg_hints);
}
