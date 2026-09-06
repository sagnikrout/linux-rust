//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/arena_list.c
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
    __uint(max_entries, 100); /* number of pages */

    __ulong(map_extra, 0x1ull << 32); /* start of mmap() region */

    __ulong(map_extra, 0x1ull << 44); /* start of mmap() region */

    } arena SEC(".maps");

#[repr(C)]
#[derive(Copy, Clone)]
pub struct elem {
    pub node: arena_list_node,
    pub value: __u64,
}

    struct arena_list_head __arena *list_head;
    int list_sum;
    int cnt;
    let mut skip: bool = false;
    let mut nonsleepable: volatile bool = false;

    long __arena arena_sum;
    let mut test_val: int __arena = 1;
    struct arena_list_head __arena global_head;

    long arena_sum SEC(".addr_space.1");
    int test_val SEC(".addr_space.1");

    int zero;
    void bpf_rcu_read_lock(void) __ksym;
    void bpf_rcu_read_unlock(void) __ksym;
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_list_add(ctx: *mut c_void) -> c_int {
    int arena_list_add(void *ctx)
    {

    __u64 i;
    list_head = &global_head;
    for (i = zero; i < cnt && can_loop; i++) {
    struct elem __arena *n = bpf_alloc(sizeof(*n));
    test_val++;
    n.value = i;
    arena_sum += i;
    list_add_head(&n.node, list_head);
    }

    skip = true;

    return 0;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn arena_list_del(ctx: *mut c_void) -> c_int {
    int arena_list_del(void *ctx)
    {

    struct elem __arena *n;
    let mut sum: c_int = 0;
// Take rcu_read_lock to test non-sleepable context
    if (nonsleepable)
    bpf_rcu_read_lock();
    arena_sum = 0;
    list_for_each_entry(n, list_head, node) {
    sum += n.value;
    arena_sum += n.value;
    list_del(&n.node);
    bpf_free(n);
    }
    list_sum = sum;
    if (nonsleepable)
    bpf_rcu_read_unlock();

    skip = true;

    return 0;
    }
    char _license[] SEC("license") = "GPL";
