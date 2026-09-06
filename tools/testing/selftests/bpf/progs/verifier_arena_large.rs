//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_arena_large.c
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
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.
// Macro flag: #define BPF_NO_KFUNC_PROTOTYPES

    struct {
    __uint(type, BPF_MAP_TYPE_ARENA);
    __uint(map_flags, BPF_F_MMAPABLE);
    __uint(max_entries, ARENA_SIZE / PAGE_SIZE);
    } arena SEC(".maps");
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn big_alloc1(ctx: *mut c_void) -> c_int {
    int big_alloc1(void *ctx)
    {

    volatile char __arena *page1, *page2, *no_page, *page3;
    u64 base;
    base = (u64)arena_base(&arena);
    page1 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page1)
    return 1;
    if ((u64)page1 != base)
    return 15;
// page1 = 1;
    page2 = bpf_arena_alloc_pages(&arena, (void __arena *)(ARENA_SIZE - 2 * PAGE_SIZE),
    1, NUMA_NO_NODE, 0);
    if (!page2)
    return 2;
// page2 = 2;
// Test for the guard region at the end of the arena.
    no_page = bpf_arena_alloc_pages(&arena, (void __arena *)ARENA_SIZE - PAGE_SIZE,
    1, NUMA_NO_NODE, 0);
    if (no_page)
    return 16;
    no_page = bpf_arena_alloc_pages(&arena, (void __arena *)ARENA_SIZE,
    1, NUMA_NO_NODE, 0);
    if (no_page)
    return 3;
    if (*page1 != 1)
    return 4;
    if (*page2 != 2)
    return 5;
    bpf_arena_free_pages(&arena, (void __arena *)page1, 1);
    if (*page2 != 2)
    return 6;
    if (*page1 != 0) /* use-after-free should return 0 */
    return 7;
    page3 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page3)
    return 8;
// page3 = 3;
    if (page1 != page3)
    return 9;
    if (*page2 != 2)
    return 10;
    if (*(page1 + PAGE_SIZE) != 0)
    return 11;
    if (*(page1 - PAGE_SIZE) != 0)
    return 12;
    if (*(page2 + PAGE_SIZE) != 0)
    return 13;
    if (*(page2 - PAGE_SIZE) != 0)
    return 14;

    return 0;
    }
// Try to access a reserved page. Behavior should be identical with accessing unallocated pages.
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn access_reserved(ctx: *mut c_void) -> c_int {
    int access_reserved(void *ctx)
    {

    volatile char __arena *page;
    char __arena *base;
    let mut len: usize = 4;
    int ret, i;
// Get a separate region of the arena.
    page = base = arena_base(&arena) + 16384 * PAGE_SIZE;
    ret = bpf_arena_reserve_pages(&arena, base, len);
    if (ret)
    return 1;
// Try to dirty reserved memory.
    for (i = 0; i < len && can_loop; i++)
// page = 0x5a;
    for (i = 0; i < len && can_loop; i++) {
    page = (volatile char __arena *)(base + i * PAGE_SIZE);
//
// Error out in case either the write went through,
// or the address has random garbage.
//
    if (*page == 0x5a)
    return 2 + 2 * i;
    if (*page)
    return 2 + 2 * i + 1;
    }

    return 0;
    }
// Try to allocate a region overlapping with a reservation.
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn request_partially_reserved(ctx: *mut c_void) -> c_int {
    int request_partially_reserved(void *ctx)
    {

    volatile char __arena *page;
    char __arena *base;
    int ret;
// Add an arbitrary page offset.
    page = base = arena_base(&arena) + 4096 * __PAGE_SIZE;
    ret = bpf_arena_reserve_pages(&arena, base + 3 * __PAGE_SIZE, 4);
    if (ret)
    return 1;
    page = bpf_arena_alloc_pages(&arena, base, 5, NUMA_NO_NODE, 0);
    if ((u64)page != 0ULL)
    return 2;

    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn free_reserved(ctx: *mut c_void) -> c_int {
    int free_reserved(void *ctx)
    {

    char __arena *addr;
    char __arena *page;
    int ret;
// Add an arbitrary page offset.
    addr = arena_base(&arena) + 32768 * __PAGE_SIZE;
    page = bpf_arena_alloc_pages(&arena, addr, 2, NUMA_NO_NODE, 0);
    if (!page)
    return 1;
    ret = bpf_arena_reserve_pages(&arena, addr + 2 * __PAGE_SIZE, 2);
    if (ret)
    return 2;
//
// Reserved and allocated pages should be interchangeable for
// bpf_arena_free_pages(). Free a reserved and an allocated
// page with a single call.
//
    bpf_arena_free_pages(&arena, addr + __PAGE_SIZE , 2);
// The free call above should have succeeded, so this allocation should too.
    page = bpf_arena_alloc_pages(&arena, addr + __PAGE_SIZE, 2, NUMA_NO_NODE, 0);
    if (!page)
    return 3;

    return 0;
    }

pub const PAGE_CNT: c_int = 100;
    __u8 __arena * __arena page[PAGE_CNT]; /* occupies the first page */
    __u8 __arena *base;
//
// Check that arena's range_tree algorithm allocates pages sequentially
// on the first pass and then fills in all gaps on the second pass.
//
    __noinline int alloc_pages(int page_cnt, int pages_atonce, bool first_pass,
    int max_idx, int step)
    {
    __u8 __arena *pg;
    int i, pg_idx;
    for (i = 0; i < page_cnt; i++) {
    pg = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), pages_atonce,
    NUMA_NO_NODE, 0);
    if (!pg)
    return step;
    pg_idx = (unsigned long) (pg - base) / PAGE_SIZE;
    if (first_pass) {
// Pages must be allocated sequentially
    if (pg_idx != i)
    return step + 100;
    } else {
// Allocator must fill into gaps
    if (pg_idx >= max_idx || (pg_idx & 1))
    return step + 200;
    }
// pg = pg_idx;
    page[pg_idx] = pg;
    cond_break;
    }
    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn big_alloc2(ctx: *mut c_void) -> c_int {
    int big_alloc2(void *ctx)
    {
    __u8 __arena *pg;
    int i, err;
    base = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!base)
    return 1;
    bpf_arena_free_pages(&arena, (void __arena *)base, 1);
    err = alloc_pages(PAGE_CNT, 1, true, PAGE_CNT, 2);
    if (err)
    return err;
// Clear all even pages
    for (i = 0; i < PAGE_CNT; i += 2) {
    pg = page[i];
    if (*pg != i)
    return 3;
    bpf_arena_free_pages(&arena, (void __arena *)pg, 1);
    page[i] = core::ptr::null_mut();
    cond_break;
    }
// Allocate into freed gaps
    err = alloc_pages(PAGE_CNT / 2, 1, false, PAGE_CNT, 4);
    if (err)
    return err;
// Free pairs of pages
    for (i = 0; i < PAGE_CNT; i += 4) {
    pg = page[i];
    if (*pg != i)
    return 5;
    bpf_arena_free_pages(&arena, (void __arena *)pg, 2);
    page[i] = core::ptr::null_mut();
    barrier();
    page[i + 1] = core::ptr::null_mut();
    cond_break;
    }
// Allocate 2 pages at a time into freed gaps
    err = alloc_pages(PAGE_CNT / 4, 2, false, PAGE_CNT, 6);
    if (err)
    return err;
// Check pages without freeing
    for (i = 0; i < PAGE_CNT; i += 2) {
    pg = page[i];
    if (*pg != i)
    return 7;
    cond_break;
    }
    pg = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!pg)
    return 8;
//
// The first PAGE_CNT pages are occupied. The new page
// must be above.
//
    if ((pg - base) / PAGE_SIZE < PAGE_CNT)
    return 9;
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn big_alloc3(ctx: *mut c_void) -> c_int {
    int big_alloc3(void *ctx)
    {

    char __arena *pages;
    u64 i;
//
// Allocate 2051 pages in one go to check how kmalloc_nolock() handles large requests.
// Since kmalloc_nolock() can allocate up to 1024 struct page * at a time, this call should
// result in three batches: two batches of 1024 pages each, followed by a final batch of 3
// pages.
//
    pages = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 2051, NUMA_NO_NODE, 0);
    if (!pages)
    return 0;
    bpf_for(i, 0, 2051)
    pages[i * PAGE_SIZE] = 123;
    bpf_for(i, 0, 2051)
    if (pages[i * PAGE_SIZE] != 123)
    return i;
    bpf_arena_free_pages(&arena, pages, 2051);

    return 0;
    }

    char _license[] SEC("license") = "GPL";
