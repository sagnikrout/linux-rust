//! Automatically rewritten from C to Rust
//! Source: mm/kasan/tags.c
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
// This file contains common tag-based KASAN code.
//
// Copyright (c) 2018 Google, Inc.
// Copyright (c) 2020 Google, Inc.
//

    enum kasan_arg_stacktrace {
    KASAN_ARG_STACKTRACE_DEFAULT,
    KASAN_ARG_STACKTRACE_OFF,
    KASAN_ARG_STACKTRACE_ON,
    };
    static enum kasan_arg_stacktrace kasan_arg_stacktrace __initdata;
// Whether to collect alloc/free stack traces.
    DEFINE_STATIC_KEY_TRUE(kasan_flag_stacktrace);
// Non-zero, as initial pointer values are 0.

    struct kasan_stack_ring stack_ring = {
    .lock = __RW_LOCK_UNLOCKED(stack_ring.lock)
    };
// kasan.stacktrace=off/on
#[no_mangle]
unsafe extern "C" fn early_kasan_flag_stacktrace(arg: *mut c_char) -> int __init {
    static int __init early_kasan_flag_stacktrace(char *arg)
    {
    if (!arg)
    return -EINVAL;
    if (!strcmp(arg, "off"))
    kasan_arg_stacktrace = KASAN_ARG_STACKTRACE_OFF;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !strcmp(arg, _arg: "on")) -> else {
    else if (!strcmp(arg, "on"))
    kasan_arg_stacktrace = KASAN_ARG_STACKTRACE_ON;
    else
    return -EINVAL;
    return 0;
    }
    early_param("kasan.stacktrace", early_kasan_flag_stacktrace);
// kasan.stack_ring_size=<number of entries>
#[no_mangle]
unsafe extern "C" fn early_kasan_flag_stack_ring_size(arg: *mut c_char) -> int __init {
    static int __init early_kasan_flag_stack_ring_size(char *arg)
    {
    if (!arg)
    return -EINVAL;
    return kstrtoul(arg, 0, &stack_ring.size);
    }
    early_param("kasan.stack_ring_size", early_kasan_flag_stack_ring_size);
#[no_mangle]
pub unsafe extern "C" fn kasan_init_tags() -> void __init {
    void __init kasan_init_tags(void)
    {
    switch (kasan_arg_stacktrace) {
    case KASAN_ARG_STACKTRACE_DEFAULT:
// Default is specified by kasan_flag_stacktrace definition.
    break;
    case KASAN_ARG_STACKTRACE_OFF:
    static_branch_disable(&kasan_flag_stacktrace);
    break;
    case KASAN_ARG_STACKTRACE_ON:
    static_branch_enable(&kasan_flag_stacktrace);
    break;
    }
    if (kasan_stack_collection_enabled()) {
    if (!stack_ring.size)
    stack_ring.size = KASAN_STACK_RING_SIZE_DEFAULT;
    stack_ring.entries = memblock_alloc(
    sizeof(stack_ring.entries[0]) * stack_ring.size,
    SMP_CACHE_BYTES);
    if (WARN_ON(!stack_ring.entries))
    static_branch_disable(&kasan_flag_stacktrace);
    }
    }
    static void save_stack_info(struct kmem_cache *cache, void *object,
    gfp_t gfp_flags, bool is_free)
    {
    unsigned long flags;
    depot_stack_handle_t stack, old_stack;
    u64 pos;
    struct kasan_stack_ring_entry *entry;
    void *old_ptr;
    stack = kasan_save_stack(gfp_flags,
    STACK_DEPOT_FLAG_CAN_ALLOC | STACK_DEPOT_FLAG_GET);
//
// Prevent save_stack_info() from modifying stack ring
// when kasan_complete_mode_report_info() is walking it.
//
    read_lock_irqsave(&stack_ring.lock, flags);
    next:
    pos = atomic64_fetch_add(1, &stack_ring.pos);
    entry = &stack_ring.entries[pos % stack_ring.size];
// Detect stack ring entry slots that are being written to.
    old_ptr = READ_ONCE(entry.ptr);
    if (old_ptr == STACK_RING_BUSY_PTR)
    goto next; /* Busy slot. */
    if (!try_cmpxchg(&entry.ptr, &old_ptr, STACK_RING_BUSY_PTR))
    goto next; /* Busy slot. */
    old_stack = entry.track.stack;
    entry.size = cache.object_size;
    kasan_set_track(&entry.track, stack);
    entry.is_free = is_free;
    entry.ptr = object;
    read_unlock_irqrestore(&stack_ring.lock, flags);
    if (old_stack)
    stack_depot_put(old_stack);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_save_alloc_info(cache: *mut kmem_cache, object: *mut c_void, flags: gfp_t) {
    void kasan_save_alloc_info(struct kmem_cache *cache, void *object, gfp_t flags)
    {
    save_stack_info(cache, object, flags, false);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_save_free_info(cache: *mut kmem_cache, object: *mut c_void) {
    void kasan_save_free_info(struct kmem_cache *cache, void *object)
    {
    save_stack_info(cache, object, 0, true);
    }
