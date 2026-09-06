//! Automatically rewritten from C to Rust
//! Source: mm/kasan/sw_tags.c
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
// This file contains core software tag-based KASAN code.
//
// Copyright (c) 2018 Google, Inc.
// Author: Andrey Konovalov <andreyknvl@google.com>
//

    static DEFINE_PER_CPU(u32, prng_state);
#[no_mangle]
pub unsafe extern "C" fn kasan_init_sw_tags() -> void __init {
    void __init kasan_init_sw_tags(void)
    {
    int cpu;
    for_each_possible_cpu(cpu)
    per_cpu(prng_state, cpu) = (u32)get_cycles();
    kasan_init_tags();
    kasan_enable();
    pr_info("KernelAddressSanitizer initialized (sw-tags, stacktrace=%s)\n",
    str_on_off(kasan_stack_collection_enabled()));
    }
//
// If a preemption happens between this_cpu_read and this_cpu_write, the only
// side effect is that we'll give a few allocated in different contexts objects
// the same tag. Since tag-based KASAN is meant to be used a probabilistic
// bug-detection debug feature, this doesn't have significant negative impact.
//
// Ideally the tags use strong randomness to prevent any attempts to predict
// them during explicit exploit attempts. But strong randomness is expensive,
// and we did an intentional trade-off to use a PRNG. This non-atomic RMW
// sequence has in fact positive effect, since interrupts that randomly skew
// PRNG at unpredictable points do only good.
//
#[no_mangle]
pub unsafe extern "C" fn kasan_random_tag() -> u8 {
    u8 kasan_random_tag(void)
    {
    let mut state: u32 = this_cpu_read(prng_state);
    state = 1664525 * state + 1013904223;
    this_cpu_write(prng_state, state);
    return (u8)(state % (KASAN_TAG_MAX + 1));
    }
    bool kasan_check_range(const void *addr, size_t size, bool write,
    unsigned long ret_ip)
    {
    u8 tag;
    u8 *shadow_first, *shadow_last, *shadow;
    void *untagged_addr;
    if (unlikely(size == 0))
    return true;
    if (unlikely(addr + size < addr))
    return !kasan_report(addr, size, write, ret_ip);
    tag = get_tag((const void *)addr);
//
// Ignore accesses for pointers tagged with 0xff (native kernel
// pointer tag) to suppress false positives caused by kmap.
//
// Some kernel code was written to account for archs that don't keep
// high memory mapped all the time, but rather map and unmap particular
// pages when needed. Instead of storing a pointer to the kernel memory,
// this code saves the address of the page structure and offset within
// that page for later use. Those pages are then mapped and unmapped
// with kmap/kunmap when necessary and virt_to_page is used to get the
// virtual address of the page. For arm64 (that keeps the high memory
// mapped all the time), kmap is turned into a page_address call.
// The issue is that with use of the page_address + virt_to_page
// sequence the top byte value of the original pointer gets lost (gets
// set to KASAN_TAG_KERNEL (0xFF)).
//
    if (tag == KASAN_TAG_KERNEL)
    return true;
    untagged_addr = kasan_reset_tag((const void *)addr);
    if (unlikely(!addr_has_metadata(untagged_addr)))
    return !kasan_report(addr, size, write, ret_ip);
    shadow_first = kasan_mem_to_shadow(untagged_addr);
    shadow_last = kasan_mem_to_shadow(untagged_addr + size - 1);
    for (shadow = shadow_first; shadow <= shadow_last; shadow++) {
    if (*shadow != tag) {
    return !kasan_report(addr, size, write, ret_ip);
    }
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_byte_accessible(addr: *const c_void) -> bool {
    bool kasan_byte_accessible(const void *addr)
    {
    let mut tag: u8 = get_tag(addr);
    void *untagged_addr = kasan_reset_tag(addr);
    u8 shadow_byte;
    if (!addr_has_metadata(untagged_addr))
    return false;
    shadow_byte = READ_ONCE(*(u8 *)kasan_mem_to_shadow(untagged_addr));
    let mut tag: return = = KASAN_TAG_KERNEL || tag == shadow_byte;
    }

    void __hwasan_load##size##_noabort(void *addr)			\
    {								\
    kasan_check_range(addr, size, false, _RET_IP_);		\
    }								\
    EXPORT_SYMBOL(__hwasan_load##size##_noabort);			\
    void __hwasan_store##size##_noabort(void *addr)			\
    {								\
    kasan_check_range(addr, size, true, _RET_IP_);		\
    }								\
    EXPORT_SYMBOL(__hwasan_store##size##_noabort)
    DEFINE_HWASAN_LOAD_STORE(1);
    DEFINE_HWASAN_LOAD_STORE(2);
    DEFINE_HWASAN_LOAD_STORE(4);
    DEFINE_HWASAN_LOAD_STORE(8);
    DEFINE_HWASAN_LOAD_STORE(16);
#[no_mangle]
pub unsafe extern "C" fn __hwasan_loadN_noabort(addr: *mut c_void, size: isize) {
    void __hwasan_loadN_noabort(void *addr, ssize_t size)
    {
    kasan_check_range(addr, size, false, _RET_IP_);
    }
    EXPORT_SYMBOL(__hwasan_loadN_noabort);
#[no_mangle]
pub unsafe extern "C" fn __hwasan_storeN_noabort(addr: *mut c_void, size: isize) {
    void __hwasan_storeN_noabort(void *addr, ssize_t size)
    {
    kasan_check_range(addr, size, true, _RET_IP_);
    }
    EXPORT_SYMBOL(__hwasan_storeN_noabort);
#[no_mangle]
pub unsafe extern "C" fn __hwasan_tag_memory(addr: *mut c_void, tag: u8, size: isize) {
    void __hwasan_tag_memory(void *addr, u8 tag, ssize_t size)
    {
    kasan_poison(addr, size, tag, false);
    }
    EXPORT_SYMBOL(__hwasan_tag_memory);
    void kasan_tag_mismatch(void *addr, unsigned long access_info,
    unsigned long ret_ip)
    {
    kasan_report(addr, 1 << (access_info & 0xf), access_info & 0x10,
    ret_ip);
    }
