//! Automatically rewritten from C to Rust
//! Source: kernel/scs.c
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
// Shadow Call Stack support.
//
// Copyright (C) 2019 Google LLC
//

    DEFINE_STATIC_KEY_FALSE(dynamic_scs_enabled);

#[no_mangle]
unsafe extern "C" fn __scs_account(s: *mut c_void, account: c_int) {
    static void __scs_account(void *s, int account)
    {
    struct page *scs_page = vmalloc_to_page(s);
    mod_node_page_state(page_pgdat(scs_page), NR_KERNEL_SCS_KB,
    account * (SCS_SIZE / SZ_1K));
    }
// Matches NR_CACHED_STACKS for VMAP_STACK
pub const NR_CACHED_SCS: c_int = 2;
    static DEFINE_PER_CPU(void *, scs_cache[NR_CACHED_SCS]);
    static void *__scs_alloc(int node)
    {
    int i;
    void *s;
    for (i = 0; i < NR_CACHED_SCS; i++) {
    s = this_cpu_xchg(scs_cache[i], core::ptr::null_mut());
    if (s) {
    s = kasan_unpoison_vmalloc(s, SCS_SIZE,
    KASAN_VMALLOC_PROT_NORMAL);
    memset(s, 0, SCS_SIZE);
    goto out;
    }
    }
    s = __vmalloc_node_range(SCS_SIZE, 1, VMALLOC_START, VMALLOC_END,
    GFP_SCS, PAGE_KERNEL, 0, node,
    __builtin_return_address(0));
    out:
    return kasan_reset_tag(s);
    }
    void *scs_alloc(int node)
    {
    void *s;
    s = __scs_alloc(node);
    if (!s)
    return core::ptr::null_mut();
// __scs_magic(s) = SCS_END_MAGIC;
//
// Poison the allocation to catch unintentional accesses to
// the shadow stack when KASAN is enabled.
//
    kasan_poison_vmalloc(s, SCS_SIZE);
    __scs_account(s, 1);
    return s;
    }
#[no_mangle]
pub unsafe extern "C" fn scs_free(s: *mut c_void) {
    void scs_free(void *s)
    {
    int i;
    __scs_account(s, -1);
//
// We cannot sleep as this can be called in interrupt context,
// so use this_cpu_cmpxchg to update the cache, and vfree_atomic
// to free the stack.
//
    for (i = 0; i < NR_CACHED_SCS; i++)
    if (this_cpu_cmpxchg(scs_cache[i], 0, s) == core::ptr::null_mut())
    return;
    kasan_unpoison_vmalloc(s, SCS_SIZE, KASAN_VMALLOC_PROT_NORMAL);
    vfree_atomic(s);
    }
#[no_mangle]
unsafe extern "C" fn scs_cleanup(cpu: c_uint) -> c_int {
    static int scs_cleanup(unsigned int cpu)
    {
    int i;
    void **cache = per_cpu_ptr(scs_cache, cpu);
    for (i = 0; i < NR_CACHED_SCS; i++) {
    vfree(cache[i]);
    cache[i] = core::ptr::null_mut();
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scs_init() -> void __init {
    void __init scs_init(void)
    {
    if (!scs_is_enabled())
    return;
    cpuhp_setup_state(CPUHP_BP_PREPARE_DYN, "scs:scs_cache", core::ptr::null_mut(),
    scs_cleanup);
    }
#[no_mangle]
pub unsafe extern "C" fn scs_prepare(tsk: *mut task_struct, node: c_int) -> c_int {
    int scs_prepare(struct task_struct *tsk, int node)
    {
    void *s;
    if (!scs_is_enabled())
    return 0;
    s = scs_alloc(node);
    if (!s)
    return -ENOMEM;
    task_scs(tsk) = task_scs_sp(tsk) = s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scs_check_usage(tsk: *mut task_struct) {
    static void scs_check_usage(struct task_struct *tsk)
    {
    static unsigned long highest;
    unsigned long *p, prev, curr = highest, used = 0;
    if (!IS_ENABLED(CONFIG_DEBUG_STACK_USAGE))
    return;
    for (p = task_scs(tsk); p < __scs_magic(task_scs(tsk)); ++p) {
    if (!READ_ONCE_NOCHECK(*p))
    break;
    used += sizeof(*p);
    }
    while (used > curr) {
    prev = cmpxchg_relaxed(&highest, curr, used);
    if (prev == curr) {
    pr_info("%s (%d): highest shadow stack usage: %lu bytes\n",
    tsk.comm, task_pid_nr(tsk), used);
    break;
    }
    curr = prev;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn scs_release(tsk: *mut task_struct) {
    void scs_release(struct task_struct *tsk)
    {
    void *s = task_scs(tsk);
    if (!scs_is_enabled() || !s)
    return;
    WARN(task_scs_end_corrupted(tsk),
    "corrupted shadow stack detected when freeing task\n");
    scs_check_usage(tsk);
    scs_free(s);
    }
