//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_arena.c
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
    __uint(max_entries, 2); /* arena of two pages close to 32-bit boundary*/
    __ulong(map_extra, ARENA_VM_START); /* start of mmap() region */
    } arena SEC(".maps");
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc1_nosleep(ctx: *mut c_void) -> c_int {
    int basic_alloc1_nosleep(void *ctx)
    {

    volatile int __arena *page1, *page2, *no_page;
    page1 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page1)
    return 1;
// page1 = 1;
    page2 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page2)
    return 2;
// page2 = 2;
    no_page = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (no_page)
    return 3;
    if (*page1 != 1)
    return 4;
    if (*page2 != 2)
    return 5;
    bpf_arena_free_pages(&arena, (void __arena *)page2, 1);
    if (*page1 != 1)
    return 6;
    if (*page2 != 0 && *page2 != 2) /* use-after-free should return 0 or the stored value */
    return 7;

    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc1(ctx: *mut c_void) -> c_int {
    int basic_alloc1(void *ctx)
    {

    volatile int __arena *page1, *page2, *no_page, *page3;
    page1 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page1)
    return 1;
// page1 = 1;
    page2 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page2)
    return 2;
// page2 = 2;
    no_page = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (no_page)
    return 3;
    if (*page1 != 1)
    return 4;
    if (*page2 != 2)
    return 5;
    bpf_arena_free_pages(&arena, (void __arena *)page2, 1);
    if (*page1 != 1)
    return 6;
    if (*page2 != 0) /* use-after-free should return 0 */
    return 7;
    page3 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page3)
    return 8;
// page3 = 3;
    if (page2 != page3)
    return 9;
    if (*page1 != 1)
    return 10;

    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn free_scalar_below_arena(ctx: *mut c_void) -> c_int {
    int free_scalar_below_arena(void *ctx)
    {
    void __arena *page1, *page2, *page3;
    let mut bad_addr: __u64 = ARENA_VM_START - __PAGE_SIZE;
    page1 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page1)
    return 1;
    page2 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (!page2)
    return 2;
    page3 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (page3)
    return 3;
    bpf_arena_free_pages(&arena, (void __arena *)bad_addr, 1);
    page3 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 1, NUMA_NO_NODE, 0);
    if (page3)
    return 4;
    return 0;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc2_nosleep(ctx: *mut c_void) -> c_int {
    int basic_alloc2_nosleep(void *ctx)
    {

    volatile char __arena *page1, *page2, *page3, *page4;
    page1 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 2, NUMA_NO_NODE, 0);
    if (!page1)
    return 1;
    page2 = page1 + __PAGE_SIZE;
    page3 = page1 + __PAGE_SIZE * 2;
    page4 = page1 - __PAGE_SIZE;
// page1 = 1;
// page2 = 2;
// page3 = 3;
// page4 = 4;
    if (*page1 != 1)
    return 1;
    if (*page2 != 2)
    return 2;
    if (*page3 != 0)
    return 3;
    if (*page4 != 0)
    return 4;
    bpf_arena_free_pages(&arena, (void __arena *)page1, 2);
    if (*page1 != 0 && *page1 != 1)
    return 5;
    if (*page2 != 0 && *page2 != 2)
    return 6;
    if (*page3 != 0)
    return 7;
    if (*page4 != 0)
    return 8;

    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc2(ctx: *mut c_void) -> c_int {
    int basic_alloc2(void *ctx)
    {

    volatile char __arena *page1, *page2, *page3, *page4;
    page1 = bpf_arena_alloc_pages(&arena, core::ptr::null_mut(), 2, NUMA_NO_NODE, 0);
    if (!page1)
    return 1;
    page2 = page1 + __PAGE_SIZE;
    page3 = page1 + __PAGE_SIZE * 2;
    page4 = page1 - __PAGE_SIZE;
// page1 = 1;
// page2 = 2;
// page3 = 3;
// page4 = 4;
    if (*page1 != 1)
    return 1;
    if (*page2 != 2)
    return 2;
    if (*page3 != 0)
    return 3;
    if (*page4 != 0)
    return 4;
    bpf_arena_free_pages(&arena, (void __arena *)page1, 2);
    if (*page1 != 0)
    return 5;
    if (*page2 != 0)
    return 6;
    if (*page3 != 0)
    return 7;
    if (*page4 != 0)
    return 8;

    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_arena___l {
    pub map: bpf_map,
    pub __attribute__((preserve_access_index)): },
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(__log_level(2: 0)) -> __success {
    __success __retval(0) __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc3_nosleep(ctx: *mut c_void) -> c_int {
    int basic_alloc3_nosleep(void *ctx)
    {
    pub )&arena: *mut *mut bpf_arena___l ar = (bpf_arena___l,
    pub pages: *mut volatile char __arena,
    pub 0): pages = bpf_arena_alloc_pages(&ar->map, NULL, ar->map.max_entries, NUMA_NO_NODE,,
    if (!pages)
    pub 1: return,
    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(__log_level(2: 0)) -> __success {
    __success __retval(0) __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn basic_alloc3(ctx: *mut c_void) -> c_int {
    int basic_alloc3(void *ctx)
    {
    pub )&arena: *mut *mut bpf_arena___l ar = (bpf_arena___l,
    pub pages: *mut volatile char __arena,
    pub 0): pages = bpf_arena_alloc_pages(&ar->map, NULL, ar->map.max_entries, NUMA_NO_NODE,,
    if (!pages)
    pub 1: return,
    pub 0: return,
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve1_nosleep(ctx: *mut c_void) -> c_int {
    int basic_reserve1_nosleep(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
    pub 0): page = bpf_arena_alloc_pages(&arena, NULL, 1, NUMA_NO_NODE,,
    if (!page)
    pub 1: return,
    pub __PAGE_SIZE: page +=,
// Reserve the second page
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret)
    pub 2: return,
// Try to explicitly allocate the reserved page.
    pub 0): page = bpf_arena_alloc_pages(&arena, page, 1, NUMA_NO_NODE,,
    if (page)
    pub 3: return,
// Try to implicitly allocate the page (since there's only 2 of them).
    pub 0): page = bpf_arena_alloc_pages(&arena, NULL, 1, NUMA_NO_NODE,,
    if (page)
    pub 4: return,

    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve1(ctx: *mut c_void) -> c_int {
    int basic_reserve1(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
    pub 0): page = bpf_arena_alloc_pages(&arena, NULL, 1, NUMA_NO_NODE,,
    if (!page)
    pub 1: return,
    pub __PAGE_SIZE: page +=,
// Reserve the second page
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret)
    pub 2: return,
// Try to explicitly allocate the reserved page.
    pub 0): page = bpf_arena_alloc_pages(&arena, page, 1, NUMA_NO_NODE,,
    if (page)
    pub 3: return,
// Try to implicitly allocate the page (since there's only 2 of them).
    pub 0): page = bpf_arena_alloc_pages(&arena, NULL, 1, NUMA_NO_NODE,,
    if (page)
    pub 4: return,

    pub 0: return,
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve2_nosleep(ctx: *mut c_void) -> c_int {
    int basic_reserve2_nosleep(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
    pub arena_base(&arena): page =,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret)
    pub 1: return,
    pub 0): page = bpf_arena_alloc_pages(&arena, page, 1, NUMA_NO_NODE,,
    if ((u64)page)
    pub 2: return,

    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn basic_reserve2(ctx: *mut c_void) -> c_int {
    int basic_reserve2(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
    pub arena_base(&arena): page =,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret)
    pub 1: return,
    pub 0): page = bpf_arena_alloc_pages(&arena, page, 1, NUMA_NO_NODE,,
    if ((u64)page)
    pub 2: return,

    pub 0: return,
    }
// Reserve the same page twice, should return -EBUSY.
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_twice_nosleep(ctx: *mut c_void) -> c_int {
    int reserve_twice_nosleep(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
    pub arena_base(&arena): page =,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret)
    pub 1: return,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret != -EBUSY)
    pub 2: return,

    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_twice(ctx: *mut c_void) -> c_int {
    int reserve_twice(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
    pub arena_base(&arena): page =,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret)
    pub 1: return,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret != -EBUSY)
    pub 2: return,

    pub 0: return,
    }
// Try to reserve past the end of the arena.
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_invalid_region_nosleep(ctx: *mut c_void) -> c_int {
    int reserve_invalid_region_nosleep(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
// Try a NULL pointer.
    pub 3): ret = bpf_arena_reserve_pages(&arena, NULL,,
    if (ret != -EINVAL)
    pub 1: return,
    pub arena_base(&arena): page =,
    pub 3): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret != -EINVAL)
    pub 2: return,
    pub 4096): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret != -EINVAL)
    pub 3: return,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page, (1ULL << 32) -,
    if (ret != -EINVAL)
    pub 4: return,

    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn reserve_invalid_region(ctx: *mut c_void) -> c_int {
    int reserve_invalid_region(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
// Try a NULL pointer.
    pub 3): ret = bpf_arena_reserve_pages(&arena, NULL,,
    if (ret != -EINVAL)
    pub 1: return,
    pub arena_base(&arena): page =,
    pub 3): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret != -EINVAL)
    pub 2: return,
    pub 4096): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret != -EINVAL)
    pub 3: return,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page, (1ULL << 32) -,
    if (ret != -EINVAL)
    pub 4: return,

    pub 0: return,
    }
    SEC("iter.s/bpf_map")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 2) -> __success {
    __success __log_level(2)
#[no_mangle]
pub unsafe extern "C" fn iter_maps1(ctx: *mut bpf_iter__bpf_map) -> c_int {
    int iter_maps1(struct bpf_iter__bpf_map *ctx)
    {
    pub ctx->map: *mut *mut bpf_map map =,
    if (!map)
    pub 0: return,
    pub 0): bpf_arena_alloc_pages(map, NULL, map->max_entries, 0,,
    pub 0: return,
    }
    SEC("iter.s/bpf_map")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_map": "expected pointer to STRUCT) -> __failure {
    __failure __msg("expected pointer to STRUCT bpf_map")
#[no_mangle]
pub unsafe extern "C" fn iter_maps2(ctx: *mut bpf_iter__bpf_map) -> c_int {
    int iter_maps2(struct bpf_iter__bpf_map *ctx)
    {
    pub ctx->meta->seq: *mut *mut seq_file seq =,
    pub 0): *mut *mut bpf_arena_alloc_pages((void )seq, NULL, 1, 0,,
    pub 0: return,
    }
    SEC("iter.s/bpf_map")
#[no_mangle]
pub unsafe extern "C" fn __msg(_arg: "untrusted_ptr_bpf_map") -> __failure {
    __failure __msg("untrusted_ptr_bpf_map")
#[no_mangle]
pub unsafe extern "C" fn iter_maps3(ctx: *mut bpf_iter__bpf_map) -> c_int {
    int iter_maps3(struct bpf_iter__bpf_map *ctx)
    {
    pub ctx->map: *mut *mut bpf_map map =,
    if (!map)
    pub 0: return,
    pub 0): bpf_arena_alloc_pages(map->inner_map_meta, NULL, map->max_entries, 0,,
    pub 0: return,
    }
    pub arena_bpf_test_lock: private(ARENA_TESTS) struct bpf_spin_lock,
// Use the arena kfunc API while under a BPF lock.
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_kfuncs_under_bpf_lock(ctx: *mut c_void) -> c_int {
    int arena_kfuncs_under_bpf_lock(void *ctx)
    {

    pub page: *mut char __arena,
    pub ret: c_int,
// Get a separate region of the arena.
    pub arena_base(&arena): page =,
    pub 1): ret = bpf_arena_reserve_pages(&arena, page,,
    if (ret) {
    pub 1: return,
    }
    pub 1): bpf_arena_free_pages(&arena, page,,
    pub 0): page = bpf_arena_alloc_pages(&arena, NULL, 1, NUMA_NO_NODE,,
    if (!page) {
    pub 2: return,
    }
    pub 1): bpf_arena_free_pages(&arena, page,,

    pub 0: return,
    }

//
// Test that scalar += PTR_TO_ARENA correctly upgrades the
// destination register to a PTR_TO_ARENA.
//
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn scalar_add_arena_ptr(ctx: *mut c_void) -> c_int {
    int scalar_add_arena_ptr(void *ctx)
    {
    pub arena_ptr: *mut *mut int __arena scalar,,
    pub arena_base(&arena): *mut *mut volatile char __arena base =,
    asm volatile (
    pub 8192;": "%[arena_ptr] =,
    pub 0x1);": "%[arena_ptr] = addr_space_cast(%[arena_ptr], 0x0,,
    pub 12;": "%[scalar] =,
    pub %[arena_ptr];": "%[scalar] +=,
    : [scalar] "=r"(scalar),
    [arena_ptr] "=&r"(arena_ptr)
    : "r"(base)
    :
    pub 0: return,
    }
//
// Tests that PTR_TO_ARENA + PTR_TO_ARENA is allowed.
//
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_ptr_add_arena_ptr(ctx: *mut c_void) -> c_int {
    int arena_ptr_add_arena_ptr(void *ctx)
    {
    pub arena_ptr1: *mut *mut int __arena arena_ptr2,,
// Needed for the verifier to link the arena to the subprog.
    pub arena_base(&arena): *mut *mut volatile char __arena base =,
    asm volatile (
    pub 8192;": "%[arena_ptr1] =,
    pub 0x1);": "%[arena_ptr1] = addr_space_cast(%[arena_ptr1], 0x0,,
    pub 4096;": "%[arena_ptr2] =,
    pub 0x1);": "%[arena_ptr2] = addr_space_cast(%[arena_ptr2], 0x0,,
    pub %[arena_ptr1];": "%[arena_ptr2] +=,
    : [arena_ptr2] "=r"(arena_ptr2),
    [arena_ptr1] "=&r"(arena_ptr1)
    : "r"(base)
    :
    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn scalar_xor_arena_ptr(ctx: *mut c_void) -> c_int {
    int scalar_xor_arena_ptr(void *ctx)
    {
    pub arena_ptr: *mut *mut int __arena scalar,,
    pub arena_base(&arena): *mut *mut volatile char __arena base =,
    asm volatile (
    pub 8192;": "%[arena_ptr] =,
    pub 0x1);": "%[arena_ptr] = addr_space_cast(%[arena_ptr], 0x0,,
    pub 12;": "%[scalar] =,
    pub %[arena_ptr];": "%[scalar] ^=,
    : [scalar] "=r"(scalar),
    [arena_ptr] "=&r"(arena_ptr)
    : "r"(base)
    :
    pub 0: return,
    }
//
// Tests that PTR_TO_ARENA and non-arena pointers can be added.
//
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn arena_ptr_add_to_non_arena_ptr(ctx: *mut c_void) -> c_int {
    int arena_ptr_add_to_non_arena_ptr(void *ctx)
    {
    pub asm("r3"): *mut *mut register int __arena arena_ptr,
    pub asm("r4"): *mut *mut register void dst,
    pub arena_base(&arena): *mut *mut volatile char __arena base =,
    asm volatile (
    pub 8192;": "%[arena_ptr] =,
    pub 0x1);": "%[arena_ptr] = addr_space_cast(%[arena_ptr], 0x0,,
    pub %[ctx];": "%[dst] =,
    pub %[arena_ptr];": "%[dst] +=,
    : [arena_ptr] "=&r"(arena_ptr),
    [dst] "=&r"(dst)
    : [ctx] "r"(ctx), "r"(base)
    :
    pub 0: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn non_arena_ptr_add_to_arena_ptr(ctx: *mut c_void) -> c_int {
    int non_arena_ptr_add_to_arena_ptr(void *ctx)
    {
    pub asm("r3"): *mut *mut register int __arena arena_ptr,
    pub asm("r4"): *mut *mut register void src,
    pub arena_base(&arena): *mut *mut volatile char __arena base =,
    asm volatile (
    pub 8192;": "%[arena_ptr] =,
    pub 0x1);": "%[arena_ptr] = addr_space_cast(%[arena_ptr], 0x0,,
    pub %[ctx];": "%[src] =,
    pub %[src];": "%[arena_ptr] +=,
    : [arena_ptr] "=&r"(arena_ptr),
    [src] "=&r"(src)
    : [ctx] "r"(ctx), "r"(base)
    :
    pub 0: return,
    }
    SEC("socket")
    __description("arena and stack atomic at the same instruction")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointers": "same insn cannot be used with different) -> __failure {
    __failure __msg("same insn cannot be used with different pointers")
    __arch_x86_64
    __load_if_JITed()
#[no_mangle]
pub unsafe extern "C" fn mixed_arena_stack_atomic() -> __naked void {
    __naked void mixed_arena_stack_atomic(void)
    {
    asm volatile ("					\
    pub \: r1 = %[arena] ll;,
    pub \: r6 = r10;,
    pub \: r6 += -8;,
    pub \: r9 = 0;,
// (u64 *)(r6 + 0) = r9;				\
    pub \: r7 = 8192;,
    pub \: r7 = addr_space_cast(r7, 0, 1);,
    pub \: call %[bpf_get_prandom_u32];,
    pub \: if w0 != 0 goto 1f;,
    pub \: r8 = r6;,
    pub \: goto 2f;,
    pub \: 1: r8 = r7;,
    pub \: 2: r9 = 1;,
    pub \: *mut *mut *mut lock (u64 )(r8 + 0) += r9;,
    pub \: r0 = 0;,
    pub \: exit;,
    "	:
    : __imm_addr(arena),
    __imm(bpf_get_prandom_u32)
    pub __clobber_all): :,
    }

    static __noinline
    u32 __arena *check_arena_arg_nonglobal(u32 __arena *arg)
    {
    pub arg: *mut volatile u32 val =,
// arg = val + 1;
    pub arg: return,
    }
    __weak
    u32 __arena *check_arena_arg_global(u32 __arena *arg)
    {
    pub arg: *mut volatile u32 val =,
// arg = val + 1;
    pub arg: return,
    }
    __weak
    u32 volatile __arena *check_arena_arg_quals1(u32 volatile __arena *arg1, u32 __arena volatile *arg2)
    {
// arg1 = *arg1 + 1;
// arg2 = *arg1 + 1;
    pub arg2: return,
    }
    __weak
    u32 __arena volatile *check_arena_arg_quals2(u32 volatile __arena *arg1, u32 __arena volatile *arg2)
    {
// arg1 = *arg1 + 1;
// arg2 = *arg2 + 1;
    pub arg2: return,
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn check_arena_arg_ret(ctx: *mut c_void) -> c_int {
    int check_arena_arg_ret(void *ctx)
    {
    pub 0): *mut *mut u32 __arena page = bpf_arena_alloc_pages(&arena, NULL, 1, NUMA_NO_NODE,,
    pub page: *mut *mut u32 __arena arg =,
    pub arg1: *mut u32 __arena volatile,
    pub ret1: *mut u32 __arena volatile,
    pub arg2: *mut u32 volatile __arena,
    pub ret2: *mut u32 volatile __arena,
    if (!arg)
    pub 1: return,
// Make sure we use {arg, ret}{1, 2}.
    pub check_arena_arg_nonglobal(page): arg =,
    pub check_arena_arg_global(arg): arg =,
    pub page: arg1 = arg2 =,
    pub arg2): ret1 = check_arena_arg_quals1(arg1,,
    pub arg2): ret2 = check_arena_arg_quals2(arg1,,
    if (!(*ret1 ||*ret2))
    pub -EINVAL: return,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
