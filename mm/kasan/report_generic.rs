//! Automatically rewritten from C to Rust
//! Source: mm/kasan/report_generic.c
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
// This file contains generic KASAN specific error reporting code.
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// Author: Andrey Ryabinin <ryabinin.a.a@gmail.com>
//
// Some code borrowed from https://github.com/xairy/kasan-prototype by
// Andrey Konovalov <andreyknvl@gmail.com>
//

    const void *kasan_find_first_bad_addr(const void *addr, size_t size)
    {
    const void *p = addr;
    if (!addr_has_metadata(p))
    return p;
    while (p < addr + size && !(*(u8 *)kasan_mem_to_shadow(p)))
    p += KASAN_GRANULE_SIZE;
    return p;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_get_alloc_size(object: *mut c_void, cache: *mut kmem_cache) -> usize {
    size_t kasan_get_alloc_size(void *object, struct kmem_cache *cache)
    {
    let mut size: usize = 0;
    u8 *shadow;
//
// Skip the addr_has_metadata check, as this function only operates on
// slab memory, which must have metadata.
//
// The loop below returns 0 for freed objects, for which KASAN cannot
// calculate the allocation size based on the metadata.
//
    shadow = (u8 *)kasan_mem_to_shadow(object);
    while (size < cache.object_size) {
    if (*shadow == 0)
    size += KASAN_GRANULE_SIZE;
#[no_mangle]
pub unsafe extern "C" fn if(1: *mut *mut *mut shadow >= 1 && shadow <= KASAN_GRANULE_SIZE -) -> else {
    else if (*shadow >= 1 && *shadow <= KASAN_GRANULE_SIZE - 1)
    return size + *shadow;
    else
    return size;
    shadow++;
    }
    return cache.object_size;
    }
    static const char *get_shadow_bug_type(struct kasan_report_info *info)
    {
    const char *bug_type = "unknown-crash";
    u8 *shadow_addr;
    shadow_addr = (u8 *)kasan_mem_to_shadow(info.first_bad_addr);
//
// If shadow byte value is in [0, KASAN_GRANULE_SIZE) we can look
// at the next shadow byte to determine the type of the bad access.
//
    if (*shadow_addr > 0 && *shadow_addr <= KASAN_GRANULE_SIZE - 1)
    shadow_addr++;
    switch (*shadow_addr) {
    case 0 ... KASAN_GRANULE_SIZE - 1:
//
// In theory it's still possible to see these shadow values
// due to a data race in the kernel code.
//
    bug_type = "out-of-bounds";
    break;
    case KASAN_PAGE_REDZONE:
    case KASAN_SLAB_REDZONE:
    bug_type = "slab-out-of-bounds";
    break;
    case KASAN_GLOBAL_REDZONE:
    bug_type = "global-out-of-bounds";
    break;
    case KASAN_STACK_LEFT:
    case KASAN_STACK_MID:
    case KASAN_STACK_RIGHT:
    case KASAN_STACK_PARTIAL:
    bug_type = "stack-out-of-bounds";
    break;
    case KASAN_PAGE_FREE:
    bug_type = "use-after-free";
    break;
    case KASAN_SLAB_FREE:
    case KASAN_SLAB_FREE_META:
    bug_type = "slab-use-after-free";
    break;
    case KASAN_ALLOCA_LEFT:
    case KASAN_ALLOCA_RIGHT:
    bug_type = "alloca-out-of-bounds";
    break;
    case KASAN_VMALLOC_INVALID:
    bug_type = "vmalloc-out-of-bounds";
    break;
    }
    return bug_type;
    }
    static const char *get_wild_bug_type(struct kasan_report_info *info)
    {
    const char *bug_type = "unknown-crash";
    if ((unsigned long)info.access_addr < PAGE_SIZE)
    bug_type = "null-ptr-deref";
#[no_mangle]
pub unsafe extern "C" fn if(TASK_SIZE: (unsigned long)info->access_addr <) -> else {
    else if ((unsigned long)info.access_addr < TASK_SIZE)
    bug_type = "user-memory-access";
    else
    bug_type = "wild-memory-access";
    return bug_type;
    }
    static const char *get_bug_type(struct kasan_report_info *info)
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
    if (addr_has_metadata(info.access_addr))
    return get_shadow_bug_type(info);
    return get_wild_bug_type(info);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_complete_mode_report_info(info: *mut kasan_report_info) {
    void kasan_complete_mode_report_info(struct kasan_report_info *info)
    {
    struct kasan_alloc_meta *alloc_meta;
    struct kasan_free_meta *free_meta;
    if (!info.bug_type)
    info.bug_type = get_bug_type(info);
    if (!info.cache || !info.object)
    return;
    alloc_meta = kasan_get_alloc_meta(info.cache, info.object);
    if (alloc_meta)
    memcpy(&info.alloc_track, &alloc_meta.alloc_track,
    sizeof(info.alloc_track));
    if (*(u8 *)kasan_mem_to_shadow(info.object) == KASAN_SLAB_FREE_META) {
// Free meta must be present with KASAN_SLAB_FREE_META.
    free_meta = kasan_get_free_meta(info.cache, info.object);
    memcpy(&info.free_track, &free_meta.free_track,
    sizeof(info.free_track));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_metadata_fetch_row(buffer: *mut c_char, row: *mut c_void) {
    void kasan_metadata_fetch_row(char *buffer, void *row)
    {
    memcpy(buffer, kasan_mem_to_shadow(row), META_BYTES_PER_ROW);
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_print_aux_stacks(cache: *mut kmem_cache, object: *const c_void) {
    void kasan_print_aux_stacks(struct kmem_cache *cache, const void *object)
    {
    struct kasan_alloc_meta *alloc_meta;
    alloc_meta = kasan_get_alloc_meta(cache, object);
    if (!alloc_meta)
    return;
    if (alloc_meta.aux_stack[0]) {
    pr_err("Last potentially related work creation:\n");
    stack_depot_print(alloc_meta.aux_stack[0]);
    pr_err("\n");
    }
    if (alloc_meta.aux_stack[1]) {
    pr_err("Second to last potentially related work creation:\n");
    stack_depot_print(alloc_meta.aux_stack[1]);
    pr_err("\n");
    }
    }

    static bool __must_check tokenize_frame_descr(const char **frame_descr,
    char *token, size_t max_tok_len,
    unsigned long *value)
    {
    const char *sep = strchr(*frame_descr, ' ');
    if (sep == core::ptr::null_mut())
    sep = *frame_descr + strlen(*frame_descr);
    if (token != core::ptr::null_mut()) {
    let mut tok_len: usize = sep - *frame_descr;
    if (tok_len + 1 > max_tok_len) {
    pr_err("internal error: frame description too long: %s\n",
// frame_descr);
    return false;
    }
// Copy token (+ 1 byte for '\0').
    strscpy(token, *frame_descr, tok_len + 1);
    }
// Advance frame_descr past separator.
// frame_descr = sep + 1;
    if (value != core::ptr::null_mut() && kstrtoul(token, 10, value)) {
    pr_err("internal error: not a valid number: %s\n", token);
    return false;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn print_decoded_frame_descr(frame_descr: *const c_char) {
    static void print_decoded_frame_descr(const char *frame_descr)
    {
//
// We need to parse the following string:
// "n alloc_1 alloc_2 ... alloc_n"
// where alloc_i looks like
// "offset size len name"
// or "offset size len name:line".
//
    char token[64];
    unsigned long num_objects;
    if (!tokenize_frame_descr(&frame_descr, token, sizeof(token),
    &num_objects))
    return;
    pr_err("\n");
    pr_err("This frame has %lu %s:\n", num_objects,
    num_objects == 1 ? "object" : "objects");
    while (num_objects--) {
    unsigned long offset;
    unsigned long size;
// access offset
    if (!tokenize_frame_descr(&frame_descr, token, sizeof(token),
    &offset))
    return;
// access size
    if (!tokenize_frame_descr(&frame_descr, token, sizeof(token),
    &size))
    return;
// name length (unused)
    if (!tokenize_frame_descr(&frame_descr, core::ptr::null_mut(), 0, core::ptr::null_mut()))
    return;
// object name
    if (!tokenize_frame_descr(&frame_descr, token, sizeof(token),
    core::ptr::null_mut()))
    return;
// Strip line number; without filename it's not very helpful.
    strreplace(token, ':', '\0');
// Finally, print object information.
    pr_err(" [%lu, %lu) '%s'", offset, offset + size, token);
    }
    }
// Returns true only if the address is on the current task's stack.
    static bool __must_check get_address_stack_frame_info(const void *addr,
    unsigned long *offset,
    const char **frame_descr,
    const void **frame_pc)
    {
    unsigned long aligned_addr;
    unsigned long mem_ptr;
    const u8 *shadow_bottom;
    const u8 *shadow_ptr;
    const unsigned long *frame;
    BUILD_BUG_ON(IS_ENABLED(CONFIG_STACK_GROWSUP));
    aligned_addr = round_down((unsigned long)addr, sizeof(long));
    mem_ptr = round_down(aligned_addr, KASAN_GRANULE_SIZE);
    shadow_ptr = kasan_mem_to_shadow((void *)aligned_addr);
    shadow_bottom = kasan_mem_to_shadow(end_of_stack(current));
    while (shadow_ptr >= shadow_bottom && *shadow_ptr != KASAN_STACK_LEFT) {
    shadow_ptr--;
    mem_ptr -= KASAN_GRANULE_SIZE;
    }
    while (shadow_ptr >= shadow_bottom && *shadow_ptr == KASAN_STACK_LEFT) {
    shadow_ptr--;
    mem_ptr -= KASAN_GRANULE_SIZE;
    }
    if (shadow_ptr < shadow_bottom)
    return false;
    frame = (const unsigned long *)(mem_ptr + KASAN_GRANULE_SIZE);
    if (frame[0] != KASAN_CURRENT_STACK_FRAME_MAGIC) {
    pr_err("internal error: frame has invalid marker: %lu\n",
    frame[0]);
    return false;
    }
// offset = (unsigned long)addr - (unsigned long)frame;
// frame_descr = (const char *)frame[1];
// frame_pc = (void *)frame[2];
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kasan_print_address_stack_frame(addr: *const c_void) {
    void kasan_print_address_stack_frame(const void *addr)
    {
    unsigned long offset;
    const char *frame_descr;
    const void *frame_pc;
    if (WARN_ON(!object_is_on_stack(addr)))
    return;
    pr_err("The buggy address belongs to stack of task %s/%d\n",
    current.comm, task_pid_nr(current));
    if (!get_address_stack_frame_info(addr, &offset, &frame_descr,
    &frame_pc))
    return;
    pr_err(" and is located at offset %lu in frame:\n", offset);
    pr_err(" %pS\n", frame_pc);
    if (!frame_descr)
    return;
    print_decoded_frame_descr(frame_descr);
    }

    void __asan_report_load##size##_noabort(void *addr) \
    {                                                         \
    kasan_report(addr, size, false, _RET_IP_);	  \
    }                                                         \
    EXPORT_SYMBOL(__asan_report_load##size##_noabort)

    void __asan_report_store##size##_noabort(void *addr) \
    {                                                          \
    kasan_report(addr, size, true, _RET_IP_);	   \
    }                                                          \
    EXPORT_SYMBOL(__asan_report_store##size##_noabort)
    DEFINE_ASAN_REPORT_LOAD(1);
    DEFINE_ASAN_REPORT_LOAD(2);
    DEFINE_ASAN_REPORT_LOAD(4);
    DEFINE_ASAN_REPORT_LOAD(8);
    DEFINE_ASAN_REPORT_LOAD(16);
    DEFINE_ASAN_REPORT_STORE(1);
    DEFINE_ASAN_REPORT_STORE(2);
    DEFINE_ASAN_REPORT_STORE(4);
    DEFINE_ASAN_REPORT_STORE(8);
    DEFINE_ASAN_REPORT_STORE(16);
#[no_mangle]
pub unsafe extern "C" fn __asan_report_load_n_noabort(addr: *mut c_void, size: isize) {
    void __asan_report_load_n_noabort(void *addr, ssize_t size)
    {
    kasan_report(addr, size, false, _RET_IP_);
    }
    EXPORT_SYMBOL(__asan_report_load_n_noabort);
#[no_mangle]
pub unsafe extern "C" fn __asan_report_store_n_noabort(addr: *mut c_void, size: isize) {
    void __asan_report_store_n_noabort(void *addr, ssize_t size)
    {
    kasan_report(addr, size, true, _RET_IP_);
    }
    EXPORT_SYMBOL(__asan_report_store_n_noabort);
