//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/stacktrace.c
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
// The unwinder implementation depends on the nVHE mode:
//
// 1) Non-protected nVHE mode - the host can directly access the
// HYP stack pages and unwind the HYP stack in EL1. This saves having
// to allocate shared buffers for the host to read the unwinded
// stacktrace.
//
// 2) pKVM (protected nVHE) mode - the host cannot directly access
// the HYP memory. The stack is unwinded in EL2 and dumped to a shared
// buffer where the host can read and print the stacktrace.
//
// Copyright (C) 2022 Google LLC
//

#[no_mangle]
unsafe extern "C" fn stackinfo_get_overflow() -> stack_info {
    static struct stack_info stackinfo_get_overflow(void)
    {
    struct kvm_nvhe_stacktrace_info *stacktrace_info
    = this_cpu_ptr_nvhe_sym(kvm_stacktrace_info);
    let mut low: c_ulong = (unsigned long)stacktrace_info.overflow_stack_base;
    let mut high: c_ulong = low + OVERFLOW_STACK_SIZE;
    return (struct stack_info) {
    .low = low,
    .high = high,
    };
    }
#[no_mangle]
unsafe extern "C" fn stackinfo_get_overflow_kern_va() -> stack_info {
    static struct stack_info stackinfo_get_overflow_kern_va(void)
    {
    let mut low: c_ulong = (unsigned long)this_cpu_ptr_nvhe_sym(overflow_stack);
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
    struct kvm_nvhe_stacktrace_info *stacktrace_info
    = this_cpu_ptr_nvhe_sym(kvm_stacktrace_info);
    let mut low: c_ulong = (unsigned long)stacktrace_info.stack_base;
    let mut high: c_ulong = low + NVHE_STACK_SIZE;
    return (struct stack_info) {
    .low = low,
    .high = high,
    };
    }
#[no_mangle]
unsafe extern "C" fn stackinfo_get_hyp_kern_va() -> stack_info {
    static struct stack_info stackinfo_get_hyp_kern_va(void)
    {
    let mut low: c_ulong = (unsigned long)*this_cpu_ptr(&kvm_arm_hyp_stack_base);
    let mut high: c_ulong = low + NVHE_STACK_SIZE;
    return (struct stack_info) {
    .low = low,
    .high = high,
    };
    }
//
// kvm_nvhe_stack_kern_va - Convert KVM nVHE HYP stack addresses to a kernel VAs
//
// The nVHE hypervisor stack is mapped in the flexible 'private' VA range, to
// allow for guard pages below the stack. Consequently, the fixed offset address
// translation macros won't work here.
//
// The kernel VA is calculated as an offset from the kernel VA of the hypervisor
// stack base.
//
// Returns true on success and updates @addr to its corresponding kernel VA;
// otherwise returns false.
//
#[no_mangle]
unsafe extern "C" fn kvm_nvhe_stack_kern_va(addr: *mut c_ulong, size: c_ulong) -> bool {
    static bool kvm_nvhe_stack_kern_va(unsigned long *addr, unsigned long size)
    {
    struct stack_info stack_hyp, stack_kern;
    stack_hyp = stackinfo_get_hyp();
    stack_kern = stackinfo_get_hyp_kern_va();
    if (stackinfo_on_stack(&stack_hyp, *addr, size))
    goto found;
    stack_hyp = stackinfo_get_overflow();
    stack_kern = stackinfo_get_overflow_kern_va();
    if (stackinfo_on_stack(&stack_hyp, *addr, size))
    goto found;
    return false;
    found:
// addr = *addr - stack_hyp.low + stack_kern.low;
    return true;
    }
//
// Convert a KVN nVHE HYP frame record address to a kernel VA
//
#[no_mangle]
unsafe extern "C" fn kvm_nvhe_stack_kern_record_va(addr: *mut c_ulong) -> bool {
    static bool kvm_nvhe_stack_kern_record_va(unsigned long *addr)
    {
    return kvm_nvhe_stack_kern_va(addr, 16);
    }
#[no_mangle]
unsafe extern "C" fn unwind_next(state: *mut unwind_state) -> c_int {
    static int unwind_next(struct unwind_state *state)
    {
//
// The FP is in the hypervisor VA space. Convert it to the kernel VA
// space so it can be unwound by the regular unwind functions.
//
    if (!kvm_nvhe_stack_kern_record_va(&state.fp))
    return -EINVAL;
    return unwind_next_frame_record(state);
    }
    static void unwind(struct unwind_state *state,
    stack_trace_consume_fn consume_entry, void *cookie)
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
// kvm_nvhe_dump_backtrace_entry - Symbolize and print an nVHE backtrace entry
//
// @arg    : the hypervisor offset, used for address translation
// @where  : the program counter corresponding to the stack frame
//
#[no_mangle]
unsafe extern "C" fn kvm_nvhe_dump_backtrace_entry(arg: *mut c_void, where: c_ulong) -> bool {
    static bool kvm_nvhe_dump_backtrace_entry(void *arg, unsigned long where)
    {
    let mut va_mask: c_ulong = GENMASK_ULL(__hyp_va_bits - 1, 0);
    let mut hyp_offset: c_ulong = (unsigned long)arg;
// Mask tags and convert to kern addr
    where = (where & va_mask) + hyp_offset;
    kvm_err(" [<%016lx>] %pB\n", where, (void *)(where + kaslr_offset()));
    return true;
    }
#[no_mangle]
unsafe extern "C" fn kvm_nvhe_dump_backtrace_start() {
    static void kvm_nvhe_dump_backtrace_start(void)
    {
    kvm_err("nVHE call trace:\n");
    }
#[no_mangle]
unsafe extern "C" fn kvm_nvhe_dump_backtrace_end() {
    static void kvm_nvhe_dump_backtrace_end(void)
    {
    kvm_err("---[ end nVHE call trace ]---\n");
    }
//
// hyp_dump_backtrace - Dump the non-protected nVHE backtrace.
//
// @hyp_offset: hypervisor offset, used for address translation.
//
// The host can directly access HYP stack pages in non-protected
// mode, so the unwinding is done directly from EL1. This removes
// the need for shared buffers between host and hypervisor for
// the stacktrace.
//
#[no_mangle]
unsafe extern "C" fn hyp_dump_backtrace(hyp_offset: c_ulong) {
    static void hyp_dump_backtrace(unsigned long hyp_offset)
    {
    struct kvm_nvhe_stacktrace_info *stacktrace_info;
    struct stack_info stacks[] = {
    stackinfo_get_overflow_kern_va(),
    stackinfo_get_hyp_kern_va(),
    };
    struct unwind_state state = {
    .stacks = stacks,
    .nr_stacks = ARRAY_SIZE(stacks),
    };
    stacktrace_info = this_cpu_ptr_nvhe_sym(kvm_stacktrace_info);
    kvm_nvhe_unwind_init(&state, stacktrace_info.fp, stacktrace_info.pc);
    kvm_nvhe_dump_backtrace_start();
    unwind(&state, kvm_nvhe_dump_backtrace_entry, (void *)hyp_offset);
    kvm_nvhe_dump_backtrace_end();
    }

    DECLARE_KVM_NVHE_PER_CPU(unsigned long [NVHE_STACKTRACE_SIZE/sizeof(long)],
    pkvm_stacktrace);
//
// pkvm_dump_backtrace - Dump the protected nVHE HYP backtrace.
//
// @hyp_offset: hypervisor offset, used for address translation.
//
// Dumping of the pKVM HYP backtrace is done by reading the
// stack addresses from the shared stacktrace buffer, since the
// host cannot directly access hypervisor memory in protected
// mode.
//
#[no_mangle]
unsafe extern "C" fn pkvm_dump_backtrace(hyp_offset: c_ulong) {
    static void pkvm_dump_backtrace(unsigned long hyp_offset)
    {
    unsigned long *stacktrace
    = (unsigned long *) this_cpu_ptr_nvhe_sym(pkvm_stacktrace);
    int i;
    kvm_nvhe_dump_backtrace_start();
// The saved stacktrace is terminated by a null entry
    for (i = 0;
    i < ARRAY_SIZE(kvm_nvhe_sym(pkvm_stacktrace)) && stacktrace[i];
    i++)
    kvm_nvhe_dump_backtrace_entry((void *)hyp_offset, stacktrace[i]);
    kvm_nvhe_dump_backtrace_end();
    }

#[no_mangle]
unsafe extern "C" fn pkvm_dump_backtrace(hyp_offset: c_ulong) {
    static void pkvm_dump_backtrace(unsigned long hyp_offset)
    {
    kvm_err("Cannot dump pKVM nVHE stacktrace: !CONFIG_PKVM_STACKTRACE\n");
    }

//
// kvm_nvhe_dump_backtrace - Dump KVM nVHE hypervisor backtrace.
//
// @hyp_offset: hypervisor offset, used for address translation.
//
#[no_mangle]
pub unsafe extern "C" fn kvm_nvhe_dump_backtrace(hyp_offset: c_ulong) {
    void kvm_nvhe_dump_backtrace(unsigned long hyp_offset)
    {
    if (is_protected_kvm_enabled())
    pkvm_dump_backtrace(hyp_offset);
    else
    hyp_dump_backtrace(hyp_offset);
    }
