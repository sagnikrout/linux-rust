//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/events.c
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
// Copyright (C) 2025 Google LLC
// Author: Vincent Donnefort <vdonnefort@google.com>
//

#[no_mangle]
pub unsafe extern "C" fn __tracing_enable_event(id: c_ushort, enable: bool) -> c_int {
    int __tracing_enable_event(unsigned short id, bool enable)
    {
    struct hyp_event_id *event_id = &__hyp_event_ids_start[id];
    atomic_t *enabled;
    if (event_id >= __hyp_event_ids_end)
    return -EINVAL;
    enabled = hyp_fixmap_map(__hyp_pa(&event_id.enabled));
    atomic_set(enabled, enable);
    hyp_fixmap_unmap();
    return 0;
    }
