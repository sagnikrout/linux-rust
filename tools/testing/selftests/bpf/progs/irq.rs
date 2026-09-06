//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/irq.c
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

    unsigned long global_flags;
    extern void bpf_local_irq_save(unsigned long *) __weak __ksym;
    extern void bpf_local_irq_restore(unsigned long *) __weak __ksym;
    extern int bpf_copy_from_user_str(void *dst, u32 dst__sz, const void *unsafe_ptr__ign, u64 flags) __weak __ksym;
    struct bpf_res_spin_lock lockA __hidden SEC(".data.A");
    struct bpf_res_spin_lock lockB __hidden SEC(".data.B");
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "R1 doesn't point to an irq flag on) -> __failure {
    __failure __msg("R1 doesn't point to an irq flag on stack")
#[no_mangle]
pub unsafe extern "C" fn irq_save_bad_arg(ctx: *mut __sk_buff) -> c_int {
    int irq_save_bad_arg(struct __sk_buff *ctx)
    {
    bpf_local_irq_save(&global_flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(stack": "R1 doesn't point to an irq flag on) -> __failure {
    __failure __msg("R1 doesn't point to an irq flag on stack")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_bad_arg(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_bad_arg(struct __sk_buff *ctx)
    {
    bpf_local_irq_restore(&global_flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_missing_2(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_missing_2(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    bpf_local_irq_save(&flags1);
    bpf_local_irq_save(&flags2);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_missing_3(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_missing_3(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    bpf_local_irq_save(&flags1);
    bpf_local_irq_save(&flags2);
    bpf_local_irq_save(&flags3);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_missing_3_minus_2(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_missing_3_minus_2(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    bpf_local_irq_save(&flags1);
    bpf_local_irq_save(&flags2);
    bpf_local_irq_save(&flags3);
    bpf_local_irq_restore(&flags3);
    bpf_local_irq_restore(&flags2);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn local_irq_save(flags: *mut c_ulong) -> __noinline void {
    static __noinline void local_irq_save(unsigned long *flags)
    {
    bpf_local_irq_save(flags);
    }
#[no_mangle]
unsafe extern "C" fn local_irq_restore(flags: *mut c_ulong) -> __noinline void {
    static __noinline void local_irq_restore(unsigned long *flags)
    {
    bpf_local_irq_restore(flags);
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_missing_1_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_missing_1_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags;
    local_irq_save(&flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_missing_2_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_missing_2_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    local_irq_save(&flags1);
    local_irq_save(&flags2);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_missing_3_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_missing_3_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    local_irq_save(&flags1);
    local_irq_save(&flags2);
    local_irq_save(&flags3);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed) -> __failure {
    __failure __msg("BPF_EXIT instruction in main prog cannot be used inside bpf_local_irq_save-ed region")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_missing_3_minus_2_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_missing_3_minus_2_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    local_irq_save(&flags1);
    local_irq_save(&flags2);
    local_irq_save(&flags3);
    local_irq_restore(&flags3);
    local_irq_restore(&flags2);
    return 0;
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn irq_balance(ctx: *mut __sk_buff) -> c_int {
    int irq_balance(struct __sk_buff *ctx)
    {
    unsigned long flags;
    local_irq_save(&flags);
    local_irq_restore(&flags);
    return 0;
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn irq_balance_n(ctx: *mut __sk_buff) -> c_int {
    int irq_balance_n(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    local_irq_save(&flags1);
    local_irq_save(&flags2);
    local_irq_save(&flags3);
    local_irq_restore(&flags3);
    local_irq_restore(&flags2);
    local_irq_restore(&flags1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn local_irq_balance() -> __noinline void {
    static __noinline void local_irq_balance(void)
    {
    unsigned long flags;
    local_irq_save(&flags);
    local_irq_restore(&flags);
    }
#[no_mangle]
unsafe extern "C" fn local_irq_balance_n() -> __noinline void {
    static __noinline void local_irq_balance_n(void)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    local_irq_save(&flags1);
    local_irq_save(&flags2);
    local_irq_save(&flags3);
    local_irq_restore(&flags3);
    local_irq_restore(&flags2);
    local_irq_restore(&flags1);
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn irq_balance_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_balance_subprog(struct __sk_buff *ctx)
    {
    local_irq_balance();
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(bpf_copy_from_user#": "sleepable helper) -> __failure {
    __failure __msg("sleepable helper bpf_copy_from_user#")
#[no_mangle]
pub unsafe extern "C" fn irq_sleepable_helper(ctx: *mut c_void) -> c_int {
    int irq_sleepable_helper(void *ctx)
    {
    unsigned long flags;
    u32 data;
    local_irq_save(&flags);
    bpf_copy_from_user(&data, sizeof(data), core::ptr::null_mut());
    local_irq_restore(&flags);
    return 0;
    }
    SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
#[no_mangle]
pub unsafe extern "C" fn __msg(region": "kernel func bpf_copy_from_user_str is sleepable within IRQ-disabled) -> __failure {
    __failure __msg("kernel func bpf_copy_from_user_str is sleepable within IRQ-disabled region")
#[no_mangle]
pub unsafe extern "C" fn irq_sleepable_kfunc(ctx: *mut c_void) -> c_int {
    int irq_sleepable_kfunc(void *ctx)
    {
    unsigned long flags;
    u32 data;
    local_irq_save(&flags);
    bpf_copy_from_user_str(&data, sizeof(data), core::ptr::null_mut(), 0);
    local_irq_restore(&flags);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn global_local_irq_balance() -> int __noinline {
    int __noinline global_local_irq_balance(void)
    {
    local_irq_balance_n();
    return 0;
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn irq_global_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_global_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags;
    bpf_local_irq_save(&flags);
    global_local_irq_balance();
    bpf_local_irq_restore(&flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(order": "cannot restore irq state out of) -> __failure {
    __failure __msg("cannot restore irq state out of order")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_ooo(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_ooo(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    bpf_local_irq_save(&flags1);
    bpf_local_irq_save(&flags2);
    bpf_local_irq_restore(&flags1);
    bpf_local_irq_restore(&flags2);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(order": "cannot restore irq state out of) -> __failure {
    __failure __msg("cannot restore irq state out of order")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_ooo_3(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_ooo_3(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    bpf_local_irq_save(&flags1);
    bpf_local_irq_save(&flags2);
    bpf_local_irq_restore(&flags2);
    bpf_local_irq_save(&flags3);
    bpf_local_irq_restore(&flags1);
    bpf_local_irq_restore(&flags3);
    return 0;
    }
    static __noinline void local_irq_save_3(unsigned long *flags1, unsigned long *flags2,
    unsigned long *flags3)
    {
    local_irq_save(flags1);
    local_irq_save(flags2);
    local_irq_save(flags3);
    }
    SEC("?tc")
    __success
#[no_mangle]
pub unsafe extern "C" fn irq_restore_3_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_3_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    local_irq_save_3(&flags1, &flags2, &flags3);
    bpf_local_irq_restore(&flags3);
    bpf_local_irq_restore(&flags2);
    bpf_local_irq_restore(&flags1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(order": "cannot restore irq state out of) -> __failure {
    __failure __msg("cannot restore irq state out of order")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_4_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_4_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    unsigned long flags4;
    local_irq_save_3(&flags1, &flags2, &flags3);
    bpf_local_irq_restore(&flags3);
    bpf_local_irq_save(&flags4);
    bpf_local_irq_restore(&flags4);
    bpf_local_irq_restore(&flags1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(order": "cannot restore irq state out of) -> __failure {
    __failure __msg("cannot restore irq state out of order")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_ooo_3_subprog(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_ooo_3_subprog(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    unsigned long flags2;
    unsigned long flags3;
    local_irq_save_3(&flags1, &flags2, &flags3);
    bpf_local_irq_restore(&flags3);
    bpf_local_irq_restore(&flags2);
    bpf_local_irq_save(&flags3);
    bpf_local_irq_restore(&flags1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(initialized": "expected an) -> __failure {
    __failure __msg("expected an initialized")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_invalid(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_invalid(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    let mut flags: c_ulong = 0xfaceb00c;
    bpf_local_irq_save(&flags1);
    bpf_local_irq_restore(&flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(uninitialized": "expected) -> __failure {
    __failure __msg("expected uninitialized")
#[no_mangle]
pub unsafe extern "C" fn irq_save_invalid(ctx: *mut __sk_buff) -> c_int {
    int irq_save_invalid(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    bpf_local_irq_save(&flags1);
    bpf_local_irq_save(&flags1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(initialized": "expected an) -> __failure {
    __failure __msg("expected an initialized")
#[no_mangle]
pub unsafe extern "C" fn irq_restore_iter(ctx: *mut __sk_buff) -> c_int {
    int irq_restore_iter(struct __sk_buff *ctx)
    {
    struct bpf_iter_num it;
    bpf_iter_num_new(&it, 0, 42);
    bpf_local_irq_restore((unsigned long *)&it);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(id=1": "Unreleased reference) -> __failure {
    __failure __msg("Unreleased reference id=1")
#[no_mangle]
pub unsafe extern "C" fn irq_save_iter(ctx: *mut __sk_buff) -> c_int {
    int irq_save_iter(struct __sk_buff *ctx)
    {
    struct bpf_iter_num it;
// Ensure same sized slot has st->ref_obj_id set, so we reject based on
// slot_type != STACK_IRQ_FLAG...
//
    _Static_assert(sizeof(it) == sizeof(unsigned long), "broken iterator size");
    bpf_iter_num_new(&it, 0, 42);
    bpf_local_irq_save((unsigned long *)&it);
    bpf_local_irq_restore((unsigned long *)&it);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(initialized": "expected an) -> __failure {
    __failure __msg("expected an initialized")
#[no_mangle]
pub unsafe extern "C" fn irq_flag_overwrite(ctx: *mut __sk_buff) -> c_int {
    int irq_flag_overwrite(struct __sk_buff *ctx)
    {
    unsigned long flags;
    bpf_local_irq_save(&flags);
    flags = 0xdeadbeef;
    bpf_local_irq_restore(&flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(initialized": "expected an) -> __failure {
    __failure __msg("expected an initialized")
#[no_mangle]
pub unsafe extern "C" fn irq_flag_overwrite_partial(ctx: *mut __sk_buff) -> c_int {
    int irq_flag_overwrite_partial(struct __sk_buff *ctx)
    {
    unsigned long flags;
    bpf_local_irq_save(&flags);
// (((char *)&flags) + 1) = 0xff;
    bpf_local_irq_restore(&flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(order": "cannot restore irq state out of) -> __failure {
    __failure __msg("cannot restore irq state out of order")
#[no_mangle]
pub unsafe extern "C" fn irq_ooo_refs_array(ctx: *mut __sk_buff) -> c_int {
    int irq_ooo_refs_array(struct __sk_buff *ctx)
    {
    unsigned long flags[4];
    struct { int i; } *p;
// refs=1
    bpf_local_irq_save(&flags[0]);
// refs=1,2
    p = bpf_obj_new(typeof(*p));
    if (!p) {
    bpf_local_irq_restore(&flags[0]);
    return 0;
    }
// refs=1,2,3
    bpf_local_irq_save(&flags[1]);
// refs=1,2,3,4
    bpf_local_irq_save(&flags[2]);
// Now when we remove ref=2, the verifier must not break the ordering in
// the refs array between 1,3,4. With an older implementation, the
// verifier would swap the last element with the removed element, but to
// maintain the stack property we need to use memmove.
//
    bpf_obj_drop(p);
// Save and restore to reset active_irq_id to 3, as the ordering is now
// refs=1,4,3. When restoring the linear scan will find prev_id in order
// as 3 instead of 4.
//
    bpf_local_irq_save(&flags[3]);
    bpf_local_irq_restore(&flags[3]);
// With the incorrect implementation, we can release flags[1], flags[2],
// and flags[0], i.e. in the wrong order.
//
    bpf_local_irq_restore(&flags[1]);
    bpf_local_irq_restore(&flags[2]);
    bpf_local_irq_restore(&flags[0]);
    return 0;
    }
    int __noinline
    global_subprog(int i)
    {
    if (i)
    bpf_printk("%p", &i);
    return i;
    }
    int __noinline
    global_sleepable_helper_subprog(int i)
    {
    if (i)
    bpf_copy_from_user(&i, sizeof(i), core::ptr::null_mut());
    return i;
    }
    int __noinline
    global_sleepable_kfunc_subprog(int i)
    {
    if (i)
    bpf_copy_from_user_str(&i, sizeof(i), core::ptr::null_mut(), 0);
    global_subprog(i);
    return i;
    }
    int __noinline
    global_subprog_calling_sleepable_global(int i)
    {
    if (!i)
    global_sleepable_kfunc_subprog(i);
    return i;
    }
    SEC("?syscall")
    __success
#[no_mangle]
pub unsafe extern "C" fn irq_non_sleepable_global_subprog(ctx: *mut c_void) -> c_int {
    int irq_non_sleepable_global_subprog(void *ctx)
    {
    unsigned long flags;
    bpf_local_irq_save(&flags);
    global_subprog(0);
    bpf_local_irq_restore(&flags);
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(function": "sleepable global) -> __failure {
    __failure __msg("sleepable global function")
#[no_mangle]
pub unsafe extern "C" fn irq_sleepable_helper_global_subprog(ctx: *mut c_void) -> c_int {
    int irq_sleepable_helper_global_subprog(void *ctx)
    {
    unsigned long flags;
    bpf_local_irq_save(&flags);
    global_sleepable_helper_subprog(0);
    bpf_local_irq_restore(&flags);
    return 0;
    }
    SEC("?syscall")
#[no_mangle]
pub unsafe extern "C" fn __msg(function": "sleepable global) -> __failure {
    __failure __msg("sleepable global function")
#[no_mangle]
pub unsafe extern "C" fn irq_sleepable_global_subprog_indirect(ctx: *mut c_void) -> c_int {
    int irq_sleepable_global_subprog_indirect(void *ctx)
    {
    unsigned long flags;
    bpf_local_irq_save(&flags);
    global_subprog_calling_sleepable_global(0);
    bpf_local_irq_restore(&flags);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(order": "cannot restore irq state out of) -> __failure {
    __failure __msg("cannot restore irq state out of order")
#[no_mangle]
pub unsafe extern "C" fn irq_ooo_lock_cond_inv(ctx: *mut __sk_buff) -> c_int {
    int irq_ooo_lock_cond_inv(struct __sk_buff *ctx)
    {
    unsigned long flags1, flags2;
    if (bpf_res_spin_lock_irqsave(&lockA, &flags1))
    return 0;
    if (bpf_res_spin_lock_irqsave(&lockB, &flags2)) {
    bpf_res_spin_unlock_irqrestore(&lockA, &flags1);
    return 0;
    }
    bpf_res_spin_unlock_irqrestore(&lockB, &flags1);
    bpf_res_spin_unlock_irqrestore(&lockA, &flags2);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "function calls are not) -> __failure {
    __failure __msg("function calls are not allowed")
#[no_mangle]
pub unsafe extern "C" fn irq_wrong_kfunc_class_1(ctx: *mut __sk_buff) -> c_int {
    int irq_wrong_kfunc_class_1(struct __sk_buff *ctx)
    {
    unsigned long flags1;
    if (bpf_res_spin_lock_irqsave(&lockA, &flags1))
    return 0;
// For now, bpf_local_irq_restore is not allowed in critical section,
// but this test ensures error will be caught with kfunc_class when it's
// opened up. Tested by temporarily permitting this kfunc in critical
// section.
//
    bpf_local_irq_restore(&flags1);
    bpf_res_spin_unlock_irqrestore(&lockA, &flags1);
    return 0;
    }
    SEC("?tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(allowed": "function calls are not) -> __failure {
    __failure __msg("function calls are not allowed")
#[no_mangle]
pub unsafe extern "C" fn irq_wrong_kfunc_class_2(ctx: *mut __sk_buff) -> c_int {
    int irq_wrong_kfunc_class_2(struct __sk_buff *ctx)
    {
    unsigned long flags1, flags2;
    bpf_local_irq_save(&flags1);
    if (bpf_res_spin_lock_irqsave(&lockA, &flags2))
    return 0;
    bpf_local_irq_restore(&flags2);
    bpf_res_spin_unlock_irqrestore(&lockA, &flags1);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
