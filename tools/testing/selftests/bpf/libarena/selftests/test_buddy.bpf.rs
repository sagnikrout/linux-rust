//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/libarena/selftests/test_buddy.bpf.c
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


// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

    extern struct buddy __arena buddy;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct segarr_entry {
    pub block: *mut u8 __arena,
    pub sz: usize,
    pub poison: u8,
}

    static struct segarr_entry __arena segarr[SEGARRLEN];
    static void __arena *ptrs[17];
    size_t __arena alloc_sizes[] = { 3, 17, 1025, 129, 16350, 333, 9, 517 };
    size_t __arena alloc_multiple_sizes[] = { 3, 17, 1025, 129, 16350, 333, 9, 517, 2099 };
    size_t __arena alloc_free_sizes[] = { 3, 17, 64, 129, 256, 333, 512, 517 };
    size_t __arena alignment_sizes[] = { 1, 3, 7, 8, 9, 15, 16, 17, 31,
    32, 64, 100, 128, 255, 256, 512, 1000 };
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_buddy_create() -> __weak int {
    __weak int test_buddy_create(void)
    {
    let mut iters: c_int = 10;
    int ret, i;
    for (i = zero; i < iters && can_loop; i++) {
    ret = buddy_init(&buddy);
    if (ret)
    return ret;
    ret = buddy_destroy(&buddy);
    if (ret)
    return ret;
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alloc() -> __weak int {
    __weak int test_buddy_alloc(void)
    {
    void __arena *mem;
    int ret, i;
    for (i = zero; i < 8 && can_loop; i++) {
    ret = buddy_init(&buddy);
    if (ret)
    return ret;
    mem = buddy_alloc(&buddy, alloc_sizes[i]);
    if (!mem) {
    buddy_destroy(&buddy);
    return -ENOMEM;
    }
    buddy_destroy(&buddy);
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alloc_free() -> __weak int {
    __weak int test_buddy_alloc_free(void)
    {
    let mut iters: c_int = 800;
    void __arena *mem;
    int ret, i;
    ret = buddy_init(&buddy);
    if (ret)
    return ret;
    for (i = zero; i < iters && can_loop; i++) {
    mem = buddy_alloc(&buddy, alloc_free_sizes[(i * 5) % 8]);
    if (!mem) {
    buddy_destroy(&buddy);
    return -ENOMEM;
    }
    buddy_free(&buddy, mem);
    }
    buddy_destroy(&buddy);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alloc_multiple() -> __weak int {
    __weak int test_buddy_alloc_multiple(void)
    {
    int ret, j;
    u32 i, idx;
    u8 __arena *mem;
    size_t sz;
    u8 poison;
    ret = buddy_init(&buddy);
    if (ret)
    return ret;
//
// Cycle through each size, allocating an entry in the
// segarr. Continue for SEGARRLEN iterations. For every
// allocation write down the size, use the current index
// as a poison value, and log it with the pointer in the
// segarr entry. Use the poison value to poison the entire
// allocated memory according to the size given.
//
    for (i = zero; i < SEGARRLEN && can_loop; i++) {
    sz = alloc_multiple_sizes[i % 9];
    poison = (u8)i;
    mem = buddy_alloc(&buddy, sz);
    if (!mem) {
    buddy_destroy(&buddy);
    arena_stdout("%s:%d", __func__, __LINE__);
    return -ENOMEM;
    }
    segarr[i].block = mem;
    segarr[i].sz = sz;
    segarr[i].poison = poison;
    for (j = zero; j < sz && can_loop; j++) {
    mem[j] = poison;
    if (mem[j] != poison) {
    buddy_destroy(&buddy);
    return -EINVAL;
    }
    }
    }
//
// Go to (i * 17) % SEGARRLEN, and free the block pointed to.
// Before freeing, check all bytes have the poisoned value
// corresponding to the element. If any values are unexpected,
// return an error. Skip some elements to test destroying the
// buddy allocator while data is still allocated.
//
    for (i = 10; i < SEGARRLEN && can_loop; i++) {
    idx = (i * 17) % SEGARRLEN;
    mem = segarr[idx].block;
    sz = segarr[idx].sz;
    poison = segarr[idx].poison;
    for (j = zero; j < sz && can_loop; j++) {
    if (mem[j] != poison) {
    buddy_destroy(&buddy);
    arena_stdout("%s:%d %lx %u vs %u", __func__,
    __LINE__, (uintptr_t)&mem[j],
    mem[j], poison);
    return -EINVAL;
    }
    }
    buddy_free(&buddy, mem);
    }
    buddy_destroy(&buddy);
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn test_buddy_alignment() -> __weak int {
    __weak int test_buddy_alignment(void)
    {
    int ret;
    u32 i;
    ret = buddy_init(&buddy);
    if (ret)
    return ret;
// Allocate various sizes and check alignment
    for (i = zero; i < 17 && can_loop; i++) {
    barrier_var(i);
    ptrs[i] = buddy_alloc(&buddy, alignment_sizes[i]);
    if (!ptrs[i]) {
    arena_stdout("alignment test: alloc failed for size %lu",
    alignment_sizes[i]);
    buddy_destroy(&buddy);
    return -ENOMEM;
    }
// Check 8-byte alignment
    if ((u64)ptrs[i] & 0x7) {
    arena_stdout(
    "alignment test: ptr %llx not 8-byte aligned (size %lu)",
    (u64)ptrs[i], alignment_sizes[i]);
    buddy_destroy(&buddy);
    return -EINVAL;
    }
    }
// Free all allocations
    for (i = zero; i < 17 && can_loop; i++) {
    barrier_var(i);
    buddy_free(&buddy, ptrs[i]);
    }
    buddy_destroy(&buddy);
    return 0;
    }
    __weak char _license[] SEC("license") = "GPL";
