//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/time.c
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
// Copyright (c) 1991,1992,1995  Linus Torvalds
// Copyright (c) 1994  Alan Modra
// Copyright (c) 1995  Markus Kuhn
// Copyright (c) 1996  Ingo Molnar
// Copyright (c) 1998  Andrea Arcangeli
// Copyright (c) 2002,2006  Vojtech Pavlik
// Copyright (c) 2003  Andi Kleen
//

#[no_mangle]
pub unsafe extern "C" fn profile_pc(regs: *mut pt_regs) -> c_ulong {
    unsigned long profile_pc(struct pt_regs *regs)
    {
    return instruction_pointer(regs);
    }
    EXPORT_SYMBOL(profile_pc);
//
// Default timer interrupt handler for PIT/HPET
//
#[no_mangle]
unsafe extern "C" fn timer_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t timer_interrupt(int irq, void *dev_id)
    {
    global_clock_event.event_handler(global_clock_event);
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn setup_default_timer_irq() -> void __init {
    static void __init setup_default_timer_irq(void)
    {
    let mut flags: c_ulong = IRQF_NOBALANCING | IRQF_IRQPOLL | IRQF_TIMER;
//
// Unconditionally register the legacy timer interrupt; even
// without legacy PIC/PIT we need this for the HPET0 in legacy
// replacement mode.
//
    if (request_irq(0, timer_interrupt, flags, "timer", core::ptr::null_mut()))
    pr_info("Failed to register legacy timer interrupt\n");
    }
// Default timer init function
#[no_mangle]
pub unsafe extern "C" fn hpet_time_init() -> void __init {
    void __init hpet_time_init(void)
    {
    if (!hpet_enable()) {
    if (!pit_timer_init())
    return;
    }
    setup_default_timer_irq();
    }
#[no_mangle]
unsafe extern "C" fn x86_late_time_init() -> __init void {
    static __init void x86_late_time_init(void)
    {
//
// Before PIT/HPET init, select the interrupt mode. This is required
// to make the decision whether PIT should be initialized correct.
//
    x86_init.irqs.intr_mode_select();
// Setup the legacy timers
    x86_init.timers.timer_init();
//
// After PIT/HPET timers init, set up the final interrupt mode for
// delivering IRQs.
//
    x86_init.irqs.intr_mode_init();
    tsc_init();
    if (cpu_feature_enabled(X86_FEATURE_WAITPKG))
    use_tpause_delay();
    }
//
// Initialize TSC and delay the periodic timer init to
// late x86_late_time_init() so ioremap works.
//
#[no_mangle]
pub unsafe extern "C" fn time_init() -> void __init {
    void __init time_init(void)
    {
    late_time_init = x86_late_time_init;
    }
//
// Sanity check the vdso related archdata content.
//
#[no_mangle]
pub unsafe extern "C" fn clocksource_arch_init(cs: *mut clocksource) {
    void clocksource_arch_init(struct clocksource *cs)
    {
    if (cs.vdso_clock_mode == VDSO_CLOCKMODE_NONE)
    return;
    if (cs.mask != CLOCKSOURCE_MASK(64)) {
    pr_warn("clocksource %s registered with invalid mask %016llx for VDSO. Disabling VDSO support.\n",
    cs.name, cs.mask);
    cs.vdso_clock_mode = VDSO_CLOCKMODE_NONE;
    }
    }
