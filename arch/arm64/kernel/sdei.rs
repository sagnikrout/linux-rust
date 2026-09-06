//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/sdei.c
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
// Copyright (C) 2017 Arm Ltd.

    unsigned long sdei_exit_mode;
//
// VMAP'd stacks checking for stack overflow on exception using sp as a scratch
// register, meaning SDEI has to switch to its own stack. We need two stacks as
// a critical event may interrupt a normal event that has just taken a
// synchronous exception, and is using sp as scratch register. For a critical
// event interrupting a normal event, we can't reliably tell if we were on the
// sdei stack.
// For now, we allocate stacks when the driver is probed.
//
    DECLARE_PER_CPU(unsigned long *, sdei_stack_normal_ptr);
    DECLARE_PER_CPU(unsigned long *, sdei_stack_critical_ptr);
    DEFINE_PER_CPU(unsigned long *, sdei_stack_normal_ptr);
    DEFINE_PER_CPU(unsigned long *, sdei_stack_critical_ptr);
    DECLARE_PER_CPU(unsigned long *, sdei_shadow_call_stack_normal_ptr);
    DECLARE_PER_CPU(unsigned long *, sdei_shadow_call_stack_critical_ptr);

    DEFINE_PER_CPU(unsigned long *, sdei_shadow_call_stack_normal_ptr);
    DEFINE_PER_CPU(unsigned long *, sdei_shadow_call_stack_critical_ptr);

    DEFINE_PER_CPU(struct sdei_registered_event *, sdei_active_normal_event);
    DEFINE_PER_CPU(struct sdei_registered_event *, sdei_active_critical_event);
#[no_mangle]
unsafe extern "C" fn _free_sdei_stack(ptr: *mut *mut unsigned long  __percpu, cpu: c_int) {
    static void _free_sdei_stack(unsigned long * __percpu *ptr, int cpu)
    {
    unsigned long *p;
    p = per_cpu(*ptr, cpu);
    if (p) {
    per_cpu(*ptr, cpu) = core::ptr::null_mut();
    vfree(p);
    }
    }
#[no_mangle]
unsafe extern "C" fn free_sdei_stacks() {
    static void free_sdei_stacks(void)
    {
    int cpu;
    for_each_possible_cpu(cpu) {
    _free_sdei_stack(&sdei_stack_normal_ptr, cpu);
    _free_sdei_stack(&sdei_stack_critical_ptr, cpu);
    }
    }
#[no_mangle]
unsafe extern "C" fn _init_sdei_stack(ptr: *mut *mut unsigned long  __percpu, cpu: c_int) -> c_int {
    static int _init_sdei_stack(unsigned long * __percpu *ptr, int cpu)
    {
    unsigned long *p;
    p = arch_alloc_vmap_stack(SDEI_STACK_SIZE, cpu_to_node(cpu));
    if (!p)
    return -ENOMEM;
    per_cpu(*ptr, cpu) = p;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_sdei_stacks() -> c_int {
    static int init_sdei_stacks(void)
    {
    int cpu;
    let mut err: c_int = 0;
    for_each_possible_cpu(cpu) {
    err = _init_sdei_stack(&sdei_stack_normal_ptr, cpu);
    if (err)
    break;
    err = _init_sdei_stack(&sdei_stack_critical_ptr, cpu);
    if (err)
    break;
    }
    if (err)
    free_sdei_stacks();
    return err;
    }
#[no_mangle]
unsafe extern "C" fn _free_sdei_scs(ptr: *mut *mut unsigned long  __percpu, cpu: c_int) {
    static void _free_sdei_scs(unsigned long * __percpu *ptr, int cpu)
    {
    void *s;
    s = per_cpu(*ptr, cpu);
    if (s) {
    per_cpu(*ptr, cpu) = core::ptr::null_mut();
    scs_free(s);
    }
    }
#[no_mangle]
unsafe extern "C" fn free_sdei_scs() {
    static void free_sdei_scs(void)
    {
    int cpu;
    for_each_possible_cpu(cpu) {
    _free_sdei_scs(&sdei_shadow_call_stack_normal_ptr, cpu);
    _free_sdei_scs(&sdei_shadow_call_stack_critical_ptr, cpu);
    }
    }
#[no_mangle]
unsafe extern "C" fn _init_sdei_scs(ptr: *mut *mut unsigned long  __percpu, cpu: c_int) -> c_int {
    static int _init_sdei_scs(unsigned long * __percpu *ptr, int cpu)
    {
    void *s;
    s = scs_alloc(cpu_to_node(cpu));
    if (!s)
    return -ENOMEM;
    per_cpu(*ptr, cpu) = s;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_sdei_scs() -> c_int {
    static int init_sdei_scs(void)
    {
    int cpu;
    let mut err: c_int = 0;
    if (!scs_is_enabled())
    return 0;
    for_each_possible_cpu(cpu) {
    err = _init_sdei_scs(&sdei_shadow_call_stack_normal_ptr, cpu);
    if (err)
    break;
    err = _init_sdei_scs(&sdei_shadow_call_stack_critical_ptr, cpu);
    if (err)
    break;
    }
    if (err)
    free_sdei_scs();
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn sdei_arch_get_entry_point(conduit: c_int) -> c_ulong {
    unsigned long sdei_arch_get_entry_point(int conduit)
    {
//
// SDEI works between adjacent exception levels. If we booted at EL1 we
// assume a hypervisor is marshalling events. If we booted at EL2 and
// dropped to EL1 because we don't support VHE, then we can't support
// SDEI.
//
    if (is_hyp_nvhe()) {
    pr_err("Not supported on this hardware/boot configuration\n");
    goto out_err;
    }
    if (init_sdei_stacks())
    goto out_err;
    if (init_sdei_scs())
    goto out_err_free_stacks;
    sdei_exit_mode = (conduit == SMCCC_CONDUIT_HVC) ? SDEI_EXIT_HVC : SDEI_EXIT_SMC;

    if (arm64_kernel_unmapped_at_el0()) {
    unsigned long offset;
    offset = (unsigned long)__sdei_asm_entry_trampoline -
    (unsigned long)__entry_tramp_text_start;
    return TRAMP_VALIAS + offset;
    } else

    return (unsigned long)__sdei_asm_handler;
    out_err_free_stacks:
    free_sdei_stacks();
    out_err:
    return 0;
    }
//
// do_sdei_event() returns one of:
// SDEI_EV_HANDLED -  success, return to the interrupted context.
// SDEI_EV_FAILED  -  failure, return this error code to firmware.
// virtual-address -  success, return to this address.
//
    unsigned long __kprobes do_sdei_event(struct pt_regs *regs,
    struct sdei_registered_event *arg)
    {
    u32 mode;
    int i, err = 0;
    let mut clobbered_registers: c_int = 4;
    let mut elr: u64 = read_sysreg(elr_el1);
    u32 kernel_mode = read_sysreg(CurrentEL) | 1;	/* +SPSel */
    let mut vbar: c_ulong = read_sysreg(vbar_el1);
    if (arm64_kernel_unmapped_at_el0())
    clobbered_registers++;
// Retrieve the missing registers values
    for (i = 0; i < clobbered_registers; i++) {
// from within the handler, this call always succeeds
    sdei_api_event_context(i, &regs.regs[i]);
    }
    err = sdei_event_handler(regs, arg);
    if (err)
    return SDEI_EV_FAILED;
    if (elr != read_sysreg(elr_el1)) {
//
// We took a synchronous exception from the SDEI handler.
// This could deadlock, and if you interrupt KVM it will
// hyp-panic instead.
//
    pr_warn("unsafe: exception during handler\n");
    }
    mode = regs.pstate & (PSR_MODE32_BIT | PSR_MODE_MASK);
//
// If we interrupted the kernel with interrupts masked, we always go
// back to wherever we came from.
//
    if (mode == kernel_mode && regs_irqs_disabled(regs))
    return SDEI_EV_HANDLED;
//
// Otherwise, we pretend this was an IRQ. This lets user space tasks
// receive signals before we return to them, and KVM to invoke it's
// world switch to do the same.
//
// See DDI0487B.a Table D1-7 'Vector offsets from vector table base
// address'.
//
    if (mode == kernel_mode)
    return vbar + 0x280;
#[no_mangle]
pub unsafe extern "C" fn if(PSR_MODE32_BIT: mode &) -> else {
    else if (mode & PSR_MODE32_BIT)
    return vbar + 0x680;
    return vbar + 0x480;
    }
