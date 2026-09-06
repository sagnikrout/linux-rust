//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/list_debug.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2022 - Google LLC
// Author: Keir Fraser <keirf@google.com>
//

#[no_mangle]
pub unsafe extern "C" fn nvhe_check_data_corruption(v: bool) -> __must_check bool {
    static inline __must_check bool nvhe_check_data_corruption(bool v)
    {
    return v;
    }

    nvhe_check_data_corruption(({					 \
    bool corruption = unlikely(condition);			 \
    if (corruption) {					 \
    if (IS_ENABLED(CONFIG_BUG_ON_DATA_CORRUPTION)) { \
    BUG();				 	 \
    } else						 \
    WARN_ON(1);				 \
    }							 \
    corruption;						 \
    }))
// The predicates checked here are taken from lib/list_debug.c.
    __list_valid_slowpath
    bool __list_add_valid_or_report(struct list_head *new, struct list_head *prev,
    struct list_head *next)
    {
    if (NVHE_CHECK_DATA_CORRUPTION(next.prev != prev) ||
    NVHE_CHECK_DATA_CORRUPTION(prev.next != next) ||
    NVHE_CHECK_DATA_CORRUPTION(new == prev || new == next))
    return false;
    return true;
    }
    __list_valid_slowpath
#[no_mangle]
pub unsafe extern "C" fn __list_del_entry_valid_or_report(entry: *mut list_head) -> bool {
    bool __list_del_entry_valid_or_report(struct list_head *entry)
    {
    struct list_head *prev, *next;
    prev = entry.prev;
    next = entry.next;
    if (NVHE_CHECK_DATA_CORRUPTION(next == LIST_POISON1) ||
    NVHE_CHECK_DATA_CORRUPTION(prev == LIST_POISON2) ||
    NVHE_CHECK_DATA_CORRUPTION(prev.next != entry) ||
    NVHE_CHECK_DATA_CORRUPTION(next.prev != entry))
    return false;
    return true;
    }
