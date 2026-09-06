//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/res_spin_lock_fail.c
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
// Copyright (c) 2024-2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arr_elem {
    pub lock: bpf_res_spin_lock,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct arr_elem);
    } arrmap SEC(".maps");
    long value;
    struct bpf_spin_lock lock __hidden SEC(".data.A");
    struct bpf_res_spin_lock res_lock __hidden SEC(".data.B");
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(object": "point to map value or allocated) -> __failure {
    __failure __msg("point to map value or allocated object")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_arg(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_arg(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    bpf_res_spin_lock((struct bpf_res_spin_lock *)bpf_core_cast(&elem.lock, struct __sk_buff));
    bpf_res_spin_lock(&elem.lock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "AA deadlock) -> __failure {
    __failure __msg("AA deadlock detected")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_AA(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_AA(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    bpf_res_spin_lock(&elem.lock);
    bpf_res_spin_lock(&elem.lock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(detected": "AA deadlock) -> __failure {
    __failure __msg("AA deadlock detected")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_cond_AA(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_cond_AA(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    if (bpf_res_spin_lock(&elem.lock))
    return 0;
    bpf_res_spin_lock(&elem.lock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "unlock of different) -> __failure {
    __failure __msg("unlock of different lock")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_mismatch_1(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_mismatch_1(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    if (bpf_res_spin_lock(&elem.lock))
    return 0;
    bpf_res_spin_unlock(&res_lock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "unlock of different) -> __failure {
    __failure __msg("unlock of different lock")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_mismatch_2(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_mismatch_2(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    if (bpf_res_spin_lock(&res_lock))
    return 0;
    bpf_res_spin_unlock(&elem.lock);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "unlock of different) -> __failure {
    __failure __msg("unlock of different lock")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_irq_mismatch_1(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_irq_mismatch_1(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    unsigned long f1;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    bpf_local_irq_save(&f1);
    if (bpf_res_spin_lock(&res_lock))
    return 0;
    bpf_res_spin_unlock_irqrestore(&res_lock, &f1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(lock": "unlock of different) -> __failure {
    __failure __msg("unlock of different lock")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_irq_mismatch_2(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_irq_mismatch_2(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    unsigned long f1;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    if (bpf_res_spin_lock_irqsave(&res_lock, &f1))
    return 0;
    bpf_res_spin_unlock(&res_lock);
    return 0;
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_ooo(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_ooo(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    if (bpf_res_spin_lock(&res_lock))
    return 0;
    if (bpf_res_spin_lock(&elem.lock)) {
    bpf_res_spin_unlock(&res_lock);
    return 0;
    }
    bpf_res_spin_unlock(&elem.lock);
    bpf_res_spin_unlock(&res_lock);
    return 0;
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_ooo_irq(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_ooo_irq(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    unsigned long f1, f2;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    if (bpf_res_spin_lock_irqsave(&res_lock, &f1))
    return 0;
    if (bpf_res_spin_lock_irqsave(&elem.lock, &f2)) {
    bpf_res_spin_unlock_irqrestore(&res_lock, &f1);
// We won't have a unreleased IRQ flag error here.
    return 0;
    }
    bpf_res_spin_unlock_irqrestore(&elem.lock, &f2);
    bpf_res_spin_unlock_irqrestore(&res_lock, &f1);
    return 0;
    }
    struct bpf_res_spin_lock lock1 __hidden SEC(".data.OO1");
    struct bpf_res_spin_lock lock2 __hidden SEC(".data.OO2");
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(order": "bpf_res_spin_unlock cannot be out of) -> __failure {
    __failure __msg("bpf_res_spin_unlock cannot be out of order")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_ooo_unlock(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_ooo_unlock(struct __sk_buff *ctx)
    {
    if (bpf_res_spin_lock(&lock1))
    return 0;
    if (bpf_res_spin_lock(&lock2)) {
    bpf_res_spin_unlock(&lock1);
    return 0;
    }
    bpf_res_spin_unlock(&lock1);
    bpf_res_spin_unlock(&lock2);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(0": "off 1 doesn't point to 'struct bpf_res_spin_lock' that is at) -> __failure {
    __failure __msg("off 1 doesn't point to 'struct bpf_res_spin_lock' that is at 0")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_bad_off(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_bad_off(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem)
    return 0;
    bpf_res_spin_lock((void *)&elem.lock + 1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(offset": "R1 doesn't have constant offset. bpf_res_spin_lock has to be at the constant) -> __failure {
    __failure __msg("R1 doesn't have constant offset. bpf_res_spin_lock has to be at the constant offset")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_var_off(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_var_off(struct __sk_buff *ctx)
    {
    struct arr_elem *elem;
    let mut val: u64 = value;
    elem = bpf_map_lookup_elem(&arrmap, &(int){0});
    if (!elem) {
// FIXME: Only inline assembly use in assert macro doesn't emit
// BTF definition.
    bpf_throw(0);
    return 0;
    }
    bpf_assert_range(val, 0, 40);
    bpf_res_spin_lock((void *)&value + val);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_res_spin_lock": "map 'res_spin.bss' has no valid) -> __failure {
    __failure __msg("map 'res_spin.bss' has no valid bpf_res_spin_lock")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_no_lock_map(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_no_lock_map(struct __sk_buff *ctx)
    {
    bpf_res_spin_lock((void *)&value + 1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_res_spin_lock": "local 'kptr' has no valid) -> __failure {
    __failure __msg("local 'kptr' has no valid bpf_res_spin_lock")
#[no_mangle]
pub unsafe extern "C" fn res_spin_lock_no_lock_kptr(ctx: *mut __sk_buff) -> c_int {
    int res_spin_lock_no_lock_kptr(struct __sk_buff *ctx)
    {
    struct { int i; } *p = bpf_obj_new(typeof(*p));
    if (!p)
    return 0;
    bpf_res_spin_lock((void *)p);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
