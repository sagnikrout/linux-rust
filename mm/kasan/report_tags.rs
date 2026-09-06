//! Automatically rewritten from C to Rust
//! Source: mm/kasan/report_tags.c
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
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Copyright (c) 2020 Google, Inc.
//

    extern struct kasan_stack_ring stack_ring;
    static const char *get_common_bug_type(struct kasan_report_info *info)
    {
//
// If access_size is a negative number, then it has reason to be
// defined as out-of-bounds bug type.
//
// Casting negative numbers to size_t would indeed turn up as
// a large size_t and its value will be larger than ULONG_MAX/2,
// so that this can qualify as out-of-bounds.
//
    if (info.access_addr + info.access_size < info.access_addr)
    return "out-of-bounds";
    return "invalid-access";
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_complete_mode_report_info(info: *mut kasan_report_info) {
    void kasan_complete_mode_report_info(struct kasan_report_info *info)
    {
    unsigned long flags;
    u64 pos;
    struct kasan_stack_ring_entry *entry;
    let mut alloc_found: bool = false, free_found = false;
    if ((!info.cache || !info.object) && !info.bug_type) {
    info.bug_type = get_common_bug_type(info);
    return;
    }
    write_lock_irqsave(&stack_ring.lock, flags);
    pos = atomic64_read(&stack_ring.pos);
//
// The loop below tries to find stack ring entries relevant to the
// buggy object. This is a best-effort process.
//
// First, another object with the same tag can be allocated in place of
// the buggy object. Also, since the number of entries is limited, the
// entries relevant to the buggy object can be overwritten.
//
    for (u64 i = pos - 1; i != pos - 1 - stack_ring.size; i--) {
    if (alloc_found && free_found)
    break;
    entry = &stack_ring.entries[i % stack_ring.size];
    if (kasan_reset_tag(entry.ptr) != info.object ||
    get_tag(entry.ptr) != get_tag(info.access_addr) ||
    info.cache.object_size != entry.size)
    continue;
    if (entry.is_free) {
//
// Second free of the same object.
// Give up on trying to find the alloc entry.
//
    if (free_found)
    break;
    memcpy(&info.free_track, &entry.track,
    sizeof(info.free_track));
    free_found = true;
//
// If a free entry is found first, the bug is likely
// a use-after-free.
//
    if (!info.bug_type)
    info.bug_type = "slab-use-after-free";
    } else {
// Second alloc of the same object. Give up.
    if (alloc_found)
    break;
    memcpy(&info.alloc_track, &entry.track,
    sizeof(info.alloc_track));
    alloc_found = true;
//
// If an alloc entry is found first, the bug is likely
// an out-of-bounds.
//
    if (!info.bug_type)
    info.bug_type = "slab-out-of-bounds";
    }
    }
    write_unlock_irqrestore(&stack_ring.lock, flags);
// Assign the common bug type if no entries were found.
    if (!info.bug_type)
    info.bug_type = get_common_bug_type(info);
    }
