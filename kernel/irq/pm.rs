//! Automatically rewritten from C to Rust
//! Source: kernel/irq/pm.c
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
// Copyright (C) 2009 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//
// This file contains power management functions related to interrupts.
//

#[no_mangle]
pub unsafe extern "C" fn irq_pm_handle_wakeup(desc: *mut irq_desc) {
    void irq_pm_handle_wakeup(struct irq_desc *desc)
    {
    irqd_clear(&desc.irq_data, IRQD_WAKEUP_ARMED);
    desc.istate |= IRQS_SUSPENDED | IRQS_PENDING;
    desc.depth++;
    irq_disable(desc);
    pm_system_irq_wakeup(irq_desc_get_irq(desc));
    }
//
// Called from __setup_irq() with desc->lock held after @action has
// been installed in the action chain.
//
#[no_mangle]
pub unsafe extern "C" fn irq_pm_install_action(desc: *mut irq_desc, action: *mut irqaction) {
    void irq_pm_install_action(struct irq_desc *desc, struct irqaction *action)
    {
    desc.nr_actions++;
    if (action.flags & IRQF_FORCE_RESUME)
    desc.force_resume_depth++;
    WARN_ON_ONCE(desc.force_resume_depth &&
    desc.force_resume_depth != desc.nr_actions);
    if (action.flags & IRQF_NO_SUSPEND)
    desc.no_suspend_depth++;
#[no_mangle]
pub unsafe extern "C" fn if(IRQF_COND_SUSPEND: action->flags &) -> else {
    else if (action.flags & IRQF_COND_SUSPEND)
    desc.cond_suspend_depth++;
    WARN_ON_ONCE(desc.no_suspend_depth &&
    (desc.no_suspend_depth + desc.cond_suspend_depth) != desc.nr_actions);
    }
//
// Called from __free_irq() with desc->lock held after @action has
// been removed from the action chain.
//
#[no_mangle]
pub unsafe extern "C" fn irq_pm_remove_action(desc: *mut irq_desc, action: *mut irqaction) {
    void irq_pm_remove_action(struct irq_desc *desc, struct irqaction *action)
    {
    desc.nr_actions--;
    if (action.flags & IRQF_FORCE_RESUME)
    desc.force_resume_depth--;
    if (action.flags & IRQF_NO_SUSPEND)
    desc.no_suspend_depth--;
#[no_mangle]
pub unsafe extern "C" fn if(IRQF_COND_SUSPEND: action->flags &) -> else {
    else if (action.flags & IRQF_COND_SUSPEND)
    desc.cond_suspend_depth--;
    }
#[no_mangle]
unsafe extern "C" fn suspend_device_irq(desc: *mut irq_desc) -> bool {
    static bool suspend_device_irq(struct irq_desc *desc)
    {
    let mut chipflags: c_ulong = irq_desc_get_chip(desc).flags;
    struct irq_data *irqd = &desc.irq_data;
    if (!desc.action || irq_desc_is_chained(desc) ||
    desc.no_suspend_depth)
    return false;
    if (irqd_is_wakeup_set(irqd)) {
    irqd_set(irqd, IRQD_WAKEUP_ARMED);
    if ((chipflags & IRQCHIP_ENABLE_WAKEUP_ON_SUSPEND) &&
    irqd_irq_disabled(irqd)) {
//
// Interrupt marked for wakeup is in disabled state.
// Enable interrupt here to unmask/enable in irqchip
// to be able to resume with such interrupts.
//
    __enable_irq(desc);
    irqd_set(irqd, IRQD_IRQ_ENABLED_ON_SUSPEND);
    }
//
// We return true here to force the caller to issue
// synchronize_irq(). We need to make sure that the
// IRQD_WAKEUP_ARMED is visible before we return from
// suspend_device_irqs().
//
    return true;
    }
    desc.istate |= IRQS_SUSPENDED;
    __disable_irq(desc);
//
// Hardware which has no wakeup source configuration facility
// requires that the non wakeup interrupts are masked at the
// chip level. The chip implementation indicates that with
// IRQCHIP_MASK_ON_SUSPEND.
//
    if (chipflags & IRQCHIP_MASK_ON_SUSPEND)
    mask_irq(desc);
    return true;
    }
//
// suspend_device_irqs - disable all currently enabled interrupt lines
//
// During system-wide suspend or hibernation device drivers need to be
// prevented from receiving interrupts and this function is provided
// for this purpose.
//
// So we disable all interrupts and mark them IRQS_SUSPENDED except
// for those which are unused, those which are marked as not
// suspendable via an interrupt request with the flag IRQF_NO_SUSPEND
// set and those which are marked as active wakeup sources.
//
// The active wakeup sources are handled by the flow handler entry
// code which checks for the IRQD_WAKEUP_ARMED flag, suspends the
// interrupt and notifies the pm core about the wakeup.
//
#[no_mangle]
pub unsafe extern "C" fn suspend_device_irqs() {
    void suspend_device_irqs(void)
    {
    struct irq_desc *desc;
    int irq;
    for_each_irq_desc(irq, desc) {
    bool sync;
    if (irq_settings_is_nested_thread(desc))
    continue;
    scoped_guard(raw_spinlock_irqsave, &desc.lock)
    sync = suspend_device_irq(desc);
    if (sync)
    synchronize_irq(irq);
    }
    }
#[no_mangle]
unsafe extern "C" fn resume_irq(desc: *mut irq_desc) {
    static void resume_irq(struct irq_desc *desc)
    {
    struct irq_data *irqd = &desc.irq_data;
    irqd_clear(irqd, IRQD_WAKEUP_ARMED);
    if (irqd_is_enabled_on_suspend(irqd)) {
//
// Interrupt marked for wakeup was enabled during suspend
// entry. Disable such interrupts to restore them back to
// original state.
//
    __disable_irq(desc);
    irqd_clear(irqd, IRQD_IRQ_ENABLED_ON_SUSPEND);
    }
    if (desc.istate & IRQS_SUSPENDED)
    goto resume;
// Force resume the interrupt?
    if (!desc.force_resume_depth)
    return;
// Pretend that it got disabled !
    desc.depth++;
    irq_state_set_disabled(desc);
    irq_state_set_masked(desc);
    resume:
    desc.istate &= ~IRQS_SUSPENDED;
    __enable_irq(desc);
    }
#[no_mangle]
unsafe extern "C" fn resume_irqs(want_early: bool) {
    static void resume_irqs(bool want_early)
    {
    struct irq_desc *desc;
    int irq;
    for_each_irq_desc(irq, desc) {
    let mut is_early: bool = desc.action &&	desc.action.flags & IRQF_EARLY_RESUME;
    if (!is_early && want_early)
    continue;
    if (irq_settings_is_nested_thread(desc))
    continue;
    guard(raw_spinlock_irqsave)(&desc.lock);
    resume_irq(desc);
    }
    }
//
// rearm_wake_irq - rearm a wakeup interrupt line after signaling wakeup
// @irq: Interrupt to rearm
//
#[no_mangle]
pub unsafe extern "C" fn rearm_wake_irq(irq: c_uint) {
    void rearm_wake_irq(unsigned int irq)
    {
    scoped_irqdesc_get_and_buslock(irq, IRQ_GET_DESC_CHECK_GLOBAL) {
    struct irq_desc *desc = scoped_irqdesc;
    if (!(desc.istate & IRQS_SUSPENDED) || !irqd_is_wakeup_set(&desc.irq_data))
    return;
    desc.istate &= ~IRQS_SUSPENDED;
    irqd_set(&desc.irq_data, IRQD_WAKEUP_ARMED);
    __enable_irq(desc);
    }
    }
//
// irq_pm_syscore_resume - enable interrupt lines early
// @data: syscore context
//
// Enable all interrupt lines with %IRQF_EARLY_RESUME set.
//
#[no_mangle]
unsafe extern "C" fn irq_pm_syscore_resume(data: *mut c_void) {
    static void irq_pm_syscore_resume(void *data)
    {
    resume_irqs(true);
    }
    static const struct syscore_ops irq_pm_syscore_ops = {
    .resume		= irq_pm_syscore_resume,
    };
    static struct syscore irq_pm_syscore = {
    .ops = &irq_pm_syscore_ops,
    };
#[no_mangle]
unsafe extern "C" fn irq_pm_init_ops() -> int __init {
    static int __init irq_pm_init_ops(void)
    {
    register_syscore(&irq_pm_syscore);
    return 0;
    }
    device_initcall(irq_pm_init_ops);
//
// resume_device_irqs - enable interrupt lines disabled by suspend_device_irqs()
//
// Enable all non-%IRQF_EARLY_RESUME interrupt lines previously
// disabled by suspend_device_irqs() that have the IRQS_SUSPENDED flag
// set as well as those with %IRQF_FORCE_RESUME.
//
#[no_mangle]
pub unsafe extern "C" fn resume_device_irqs() {
    void resume_device_irqs(void)
    {
    resume_irqs(false);
    }
