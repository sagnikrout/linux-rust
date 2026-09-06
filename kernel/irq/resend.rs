//! Automatically rewritten from C to Rust
//! Source: kernel/irq/resend.c
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
// Copyright (C) 1992, 1998-2006 Linus Torvalds, Ingo Molnar
// Copyright (C) 2005-2006, Thomas Gleixner
//
// This file contains the IRQ-resend code
//
// If the interrupt is waiting to be processed, we try to re-run it.
// We can't directly run it from here since the caller might be in an
// interrupt-protected region. Not all irq controller chips can
// retrigger interrupts at the hardware level, so in those cases
// we allow the resending of IRQs via a tasklet.
//

// hlist_head to handle software resend of interrupts:
    static HLIST_HEAD(irq_resend_list);
    static DEFINE_RAW_SPINLOCK(irq_resend_lock);
//
// Run software resends of IRQ's
//
#[no_mangle]
unsafe extern "C" fn resend_irqs(unused: *mut tasklet_struct) {
    static void resend_irqs(struct tasklet_struct *unused)
    {
    guard(raw_spinlock_irq)(&irq_resend_lock);
    while (!hlist_empty(&irq_resend_list)) {
    struct irq_desc *desc;
    desc = hlist_entry(irq_resend_list.first, struct irq_desc,  resend_node);
    hlist_del_init(&desc.resend_node);
    raw_spin_unlock(&irq_resend_lock);
    desc.handle_irq(desc);
    raw_spin_lock(&irq_resend_lock);
    }
    }
// Tasklet to handle resend:
    static DECLARE_TASKLET(resend_tasklet, resend_irqs);
#[no_mangle]
unsafe extern "C" fn irq_sw_resend(desc: *mut irq_desc) -> c_int {
    static int irq_sw_resend(struct irq_desc *desc)
    {
//
// Validate whether this interrupt can be safely injected from
// non interrupt context
//
    if (irqd_is_handle_enforce_irqctx(&desc.irq_data))
    return -EINVAL;
//
// If the interrupt is running in the thread context of the parent
// irq we need to be careful, because we cannot trigger it
// directly.
//
    if (irq_settings_is_nested_thread(desc)) {
//
// If the parent_irq is valid, we retrigger the parent,
// otherwise we do nothing.
//
    if (!desc.parent_irq)
    return -EINVAL;
    desc = irq_to_desc(desc.parent_irq);
    if (!desc)
    return -EINVAL;
    }
// Add to resend_list and activate the softirq:
    scoped_guard(raw_spinlock, &irq_resend_lock) {
    if (hlist_unhashed(&desc.resend_node))
    hlist_add_head(&desc.resend_node, &irq_resend_list);
    }
    tasklet_schedule(&resend_tasklet);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn clear_irq_resend(desc: *mut irq_desc) {
    void clear_irq_resend(struct irq_desc *desc)
    {
    guard(raw_spinlock)(&irq_resend_lock);
    hlist_del_init(&desc.resend_node);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_resend_init(desc: *mut irq_desc) {
    void irq_resend_init(struct irq_desc *desc)
    {
    INIT_HLIST_NODE(&desc.resend_node);
    }

    void clear_irq_resend(struct irq_desc *desc) {}
    void irq_resend_init(struct irq_desc *desc) {}
#[no_mangle]
unsafe extern "C" fn irq_sw_resend(desc: *mut irq_desc) -> c_int {
    static int irq_sw_resend(struct irq_desc *desc)
    {
    return -EINVAL;
    }

#[no_mangle]
unsafe extern "C" fn try_retrigger(desc: *mut irq_desc) -> c_int {
    static int try_retrigger(struct irq_desc *desc)
    {
    if (desc.irq_data.chip.irq_retrigger)
    return desc.irq_data.chip.irq_retrigger(&desc.irq_data);

    return irq_chip_retrigger_hierarchy(&desc.irq_data);

    return 0;

    }
//
// IRQ resend
//
// Is called with interrupts disabled and desc->lock held.
//
#[no_mangle]
pub unsafe extern "C" fn check_irq_resend(desc: *mut irq_desc, inject: bool) -> c_int {
    int check_irq_resend(struct irq_desc *desc, bool inject)
    {
    let mut err: c_int = 0;
//
// We do not resend level type interrupts. Level type interrupts
// are resent by hardware when they are still active. Clear the
// pending bit so suspend/resume does not get confused.
//
    if (irq_settings_is_level(desc)) {
    desc.istate &= ~IRQS_PENDING;
    return -EINVAL;
    }
    if (desc.istate & IRQS_REPLAY)
    return -EBUSY;
    if (!(desc.istate & IRQS_PENDING) && !inject)
    return 0;
    desc.istate &= ~IRQS_PENDING;
    if (!try_retrigger(desc))
    err = irq_sw_resend(desc);
// If the retrigger was successful, mark it with the REPLAY bit
    if (!err)
    desc.istate |= IRQS_REPLAY;
    return err;
    }

//
// irq_inject_interrupt - Inject an interrupt for testing/error injection
// @irq:	The interrupt number
//
// This function must only be used for debug and testing purposes!
//
// Especially on x86 this can cause a premature completion of an interrupt
// affinity change causing the interrupt line to become stale. Very
// unlikely, but possible.
//
// The injection can fail for various reasons:
// - Interrupt is not activated
// - Interrupt is NMI type or currently replaying
// - Interrupt is level type
// - Interrupt does not support hardware retrigger and software resend is
// either not enabled or not possible for the interrupt.
//
#[no_mangle]
pub unsafe extern "C" fn irq_inject_interrupt(irq: c_uint) -> c_int {
    int irq_inject_interrupt(unsigned int irq)
    {
    let mut err: c_int = -EINVAL;
// Try the state injection hardware interface first
    if (!irq_set_irqchip_state(irq, IRQCHIP_STATE_PENDING, true))
    return 0;
// That failed, try via the resend mechanism
    scoped_irqdesc_get_and_buslock(irq, 0) {
    struct irq_desc *desc = scoped_irqdesc;
//
// Only try to inject when the interrupt is:
// - not NMI type
// - activated
//
    if (!irq_is_nmi(desc) && irqd_is_activated(&desc.irq_data))
    err = check_irq_resend(desc, true);
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(irq_inject_interrupt);
