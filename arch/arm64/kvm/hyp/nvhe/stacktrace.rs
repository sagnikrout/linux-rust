//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/nvhe/stacktrace.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// KVM nVHE hypervisor stack tracing support.
//
// Copyright (C) 2022 Google LLC
//

    DEFINE_PER_CPU(unsigned long [OVERFLOW_STACK_SIZE/sizeof(long)], overflow_stack)
    __aligned(16);
    DEFINE_PER_CPU(struct kvm_nvhe_stacktrace_info, kvm_stacktrace_info);
//
// hyp_prepare_backtrace - Prepare non-protected nVHE backtrace.
//
// @fp : frame pointer at which to start the unwinding.
// @pc : program counter at which to start the unwinding.
//
// Save the information needed by the host to unwind the non-protected
// nVHE hypervisor stack in EL1.
//
#[no_mangle]
unsafe extern "C" fn hyp_prepare_backtrace(fp: c_ulong, pc: c_ulong) {
    static void hyp_prepare_backtrace(unsigned long fp, unsigned long pc)
    {
    struct kvm_nvhe_stacktrace_info *stacktrace_info = this_cpu_ptr(&kvm_stacktrace_info);
    struct kvm_nvhe_init_params *params = this_cpu_ptr(&kvm_init_params);
    stacktrace_info.stack_base = (unsigned long)(params.stack_hyp_va - NVHE_STACK_SIZE);
    stacktrace_info.overflow_stack_base = (unsigned long)this_cpu_ptr(overflow_stack);
    stacktrace_info.fp = fp;
    stacktrace_info.pc = pc;
    }

    DEFINE_PER_CPU(unsigned long [NVHE_STACKTRACE_SIZE/sizeof(long)], pkvm_stacktrace);
#[no_mangle]
unsafe extern "C" fn stackinfo_get_overflow() -> stack_info {
    static struct stack_info stackinfo_get_overflow(void)
    {
    let mut low: c_ulong = (unsigned long)this_cpu_ptr(overflow_stack);
    let mut high: c_ulong = low + OVERFLOW_STACK_SIZE;
    return (struct stack_info) {
    .low = low,
    .high = high,
    };
    }
#[no_mangle]
unsafe extern "C" fn stackinfo_get_hyp() -> stack_info {
    static struct stack_info stackinfo_get_hyp(void)
    {
    struct kvm_nvhe_init_params *params = this_cpu_ptr(&kvm_init_params);
    let mut high: c_ulong = params.stack_hyp_va;
    let mut low: c_ulong = high - NVHE_STACK_SIZE;
    return (struct stack_info) {
    .low = low,
    .high = high,
    };
    }
#[no_mangle]
unsafe extern "C" fn unwind_next(state: *mut unwind_state) -> c_int {
    static int unwind_next(struct unwind_state *state)
    {
    return unwind_next_frame_record(state);
    }
    static void notrace unwind(struct unwind_state *state,
    stack_trace_consume_fn consume_entry,
    void *cookie)
    {
    while (1) {
    int ret;
    if (!consume_entry(cookie, state.pc))
    break;
    ret = unwind_next(state);
    if (ret < 0)
    break;
    }
    }
//
// pkvm_save_backtrace_entry - Saves a protected nVHE HYP stacktrace entry
//
// @arg    : index of the entry in the stacktrace buffer
// @where  : the program counter corresponding to the stack frame
//
// Save the return address of a stack frame to the shared stacktrace buffer.
// The host can access this shared buffer from EL1 to dump the backtrace.
//
#[no_mangle]
unsafe extern "C" fn pkvm_save_backtrace_entry(arg: *mut c_void, where: c_ulong) -> bool {
    static bool pkvm_save_backtrace_entry(void *arg, unsigned long where)
    {
    unsigned long *stacktrace = this_cpu_ptr(pkvm_stacktrace);
    int *idx = (int *)arg;
//
// Need 2 free slots: 1 for current entry and 1 for the
// delimiter.
//
    if (*idx > ARRAY_SIZE(pkvm_stacktrace) - 2)
    return false;
    stacktrace[*idx] = where;
    stacktrace[++*idx] = 0UL;
    return true;
    }
//
// pkvm_save_backtrace - Saves the protected nVHE HYP stacktrace
//
// @fp : frame pointer at which to start the unwinding.
// @pc : program counter at which to start the unwinding.
//
// Save the unwinded stack addresses to the shared stacktrace buffer.
// The host can access this shared buffer from EL1 to dump the backtrace.
//
#[no_mangle]
unsafe extern "C" fn pkvm_save_backtrace(fp: c_ulong, pc: c_ulong) {
    static void pkvm_save_backtrace(unsigned long fp, unsigned long pc)
    {
    struct stack_info stacks[] = {
    stackinfo_get_overflow(),
    stackinfo_get_hyp(),
    };
    struct unwind_state state = {
    .stacks = stacks,
    .nr_stacks = ARRAY_SIZE(stacks),
    };
    let mut idx: c_int = 0;
    kvm_nvhe_unwind_init(&state, fp, pc);
    unwind(&state, pkvm_save_backtrace_entry, &idx);
    }

#[no_mangle]
unsafe extern "C" fn pkvm_save_backtrace(fp: c_ulong, pc: c_ulong) {
    static void pkvm_save_backtrace(unsigned long fp, unsigned long pc)
    {
    }

//
// kvm_nvhe_prepare_backtrace - prepare to dump the nVHE backtrace
//
// @fp : frame pointer at which to start the unwinding.
// @pc : program counter at which to start the unwinding.
//
// Saves the information needed by the host to dump the nVHE hypervisor
// backtrace.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_nvhe_prepare_backtrace(fp: c_ulong, pc: c_ulong) {
    void kvm_nvhe_prepare_backtrace(unsigned long fp, unsigned long pc)
    {
    if (is_protected_kvm_enabled())
    pkvm_save_backtrace(fp, pc);
    else
    hyp_prepare_backtrace(fp, pc);
    }
