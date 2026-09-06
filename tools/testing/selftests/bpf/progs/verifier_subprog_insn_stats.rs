//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_subprog_insn_stats.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timer_value {
    pub timer: bpf_timer,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct timer_value);
    } timer_map SEC(".maps");
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 4) -> __success {
    __success __log_level(4)
    __msg("subprog 0 (stats_main_only) main insns_self 2 insns_total 2 stack 0")
    __msg("processed 2 insns")
#[no_mangle]
pub unsafe extern "C" fn stats_main_only() -> __naked int {
    __naked int stats_main_only(void)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    );
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn stats_chain_leaf() -> c_int {
    static int stats_chain_leaf(void)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    );
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn stats_chain_parent() -> c_int {
    static int stats_chain_parent(void)
    {
    asm volatile (
    "call stats_chain_leaf;"
    "exit;"
    );
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 4) -> __success {
    __success __log_level(4)
//
// self: 2 + 2 + 2 = 6
// totals: leaf 2, parent 2 + 2 = 4, main 2 + 4 = 6
//
    __msg("subprog 0 (stats_static_chain) main insns_self 2 insns_total 6 stack 0")
    __msg("subprog {{[0-9]+}} (stats_chain_parent) static insns_self 2 insns_total 4 stack 0")
    __msg("subprog {{[0-9]+}} (stats_chain_leaf) static insns_self 2 insns_total 2 stack 0")
    __msg("processed 6 insns")
#[no_mangle]
pub unsafe extern "C" fn stats_static_chain() -> __naked int {
    __naked int stats_static_chain(void)
    {
    asm volatile (
    "call stats_chain_parent;"
    "exit;"
    );
    }
    __naked __noinline __used
#[no_mangle]
unsafe extern "C" fn stats_shared_leaf() -> c_int {
    static int stats_shared_leaf(void)
    {
    asm volatile (
    "r0 = 0;"
    "exit;"
    );
    }
    __naked __noinline __used
#[no_mangle]
pub unsafe extern "C" fn stats_global_root() -> c_int {
    int stats_global_root(void)
    {
    asm volatile (
    "call stats_shared_leaf;"
    "exit;"
    );
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 4) -> __success {
    __success __log_level(4)
//
// stats_shared_leaf is explored once under each independent root.
// self: main 3 + leaf 4 + global 2 = 9
// root totals: main 5 + global 4 = 9
//
    __msg("subprog 0 (stats_shared_roots) main insns_self 3 insns_total 5 stack 0")
    __msg("subprog {{[0-9]+}} (stats_shared_leaf) static insns_self 4 insns_total 4 stack 0")
    __msg("subprog {{[0-9]+}} (stats_global_root) global insns_self 2 insns_total 4 stack 0")
    __msg("processed 9 insns")
#[no_mangle]
pub unsafe extern "C" fn stats_shared_roots() -> __naked int {
    __naked int stats_shared_roots(void)
    {
    asm volatile (
    "call stats_shared_leaf;"
    "call stats_global_root;"
    "exit;"
    );
    }
    __noinline __used
#[no_mangle]
unsafe extern "C" fn stats_async_leaf(map: *mut c_void, key: *mut __u32, timer: *mut bpf_timer) -> c_int {
    static int stats_async_leaf(void *map, __u32 *key, struct bpf_timer *timer)
    {
    return 0;
    }
    __noinline __used
    static __u64 stats_async_schedule(struct bpf_map *map, __u32 *key,
    struct timer_value *value, void *ctx)
    {
    asm volatile (
    "r1 = %[timer];"
    "r2 = %[stats_async_leaf];"
    "call %[bpf_timer_set_callback];"
    :
    : [timer] "r" (value),
    __imm_ptr(stats_async_leaf),
    __imm(bpf_timer_set_callback)
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 4) -> __success {
    __success __log_level(4)
//
// self: 9 + 7 + 2 = 18
// totals: leaf 2, scheduler 7, main root 18
//
    __msg("subprog 0 (stats_async_direct) main insns_self 9 insns_total 18 stack 0")
    __msg("subprog {{[0-9]+}} (stats_async_schedule) static insns_self 7 insns_total 7 stack 0")
    __msg("subprog {{[0-9]+}} (stats_async_leaf) static insns_self 2 insns_total 2 stack 0")
    __msg("processed 18 insns")
#[no_mangle]
pub unsafe extern "C" fn stats_async_direct() -> __naked int {
    __naked int stats_async_direct(void)
    {
    asm volatile (
    "r1 = %[timer_map] ll;"
    "r2 = %[stats_async_schedule];"
    "r3 = 0;"
    "r4 = 0;"
    "call %[bpf_for_each_map_elem];"
    "r0 = 0;"
    "exit;"
    :
    : __imm_addr(timer_map),
    __imm_ptr(stats_async_schedule),
    __imm(bpf_for_each_map_elem)
    : __clobber_common
    );
    }
    __noinline __used
#[no_mangle]
unsafe extern "C" fn stats_async_nested_leaf(map: *mut c_void, key: *mut __u32, timer: *mut bpf_timer) -> c_int {
    static int stats_async_nested_leaf(void *map, __u32 *key, struct bpf_timer *timer)
    {
    return 0;
    }
    __noinline __used
#[no_mangle]
unsafe extern "C" fn stats_async_outer(map: *mut c_void, key: *mut __u32, timer: *mut bpf_timer) -> c_int {
    static int stats_async_outer(void *map, __u32 *key, struct bpf_timer *timer)
    {
    asm volatile (
    "r1 = %[timer];"
    "r2 = %[stats_async_nested_leaf];"
    "call %[bpf_timer_set_callback];"
    :
    : [timer] "r" (timer),
    __imm_ptr(stats_async_nested_leaf),
    __imm(bpf_timer_set_callback)
    : __clobber_common
    );
    return 0;
    }
    __noinline __used
    static __u64 stats_async_nested_schedule(struct bpf_map *map, __u32 *key,
    struct timer_value *value, void *ctx)
    {
    asm volatile (
    "r1 = %[timer];"
    "r2 = %[stats_async_outer];"
    "call %[bpf_timer_set_callback];"
    :
    : [timer] "r" (value),
    __imm_ptr(stats_async_outer),
    __imm(bpf_timer_set_callback)
    : __clobber_common
    );
    return 0;
    }
    SEC("?raw_tp")
#[no_mangle]
pub unsafe extern "C" fn __log_level(_arg: 4) -> __success {
    __success __log_level(4)
//
// self: 9 + 7 + 7 + 2 = 25
// totals: leaf 2, outer 7, scheduler 7, main root 25
//
    __msg("subprog 0 (stats_async_nested) main insns_self 9 insns_total 25 stack 0")
    __msg("subprog {{[0-9]+}} (stats_async_nested_schedule) static insns_self 7 insns_total 7 stack 0")
    __msg("subprog {{[0-9]+}} (stats_async_outer) static insns_self 7 insns_total 7 stack 0")
    __msg("subprog {{[0-9]+}} (stats_async_nested_leaf) static insns_self 2 insns_total 2 stack 0")
    __msg("processed 25 insns")
#[no_mangle]
pub unsafe extern "C" fn stats_async_nested() -> __naked int {
    __naked int stats_async_nested(void)
    {
    asm volatile (
    "r1 = %[timer_map] ll;"
    "r2 = %[stats_async_nested_schedule];"
    "r3 = 0;"
    "r4 = 0;"
    "call %[bpf_for_each_map_elem];"
    "r0 = 0;"
    "exit;"
    :
    : __imm_addr(timer_map),
    __imm_ptr(stats_async_nested_schedule),
    __imm(bpf_for_each_map_elem)
    : __clobber_common
    );
    }
    char _license[] SEC("license") = "GPL";
