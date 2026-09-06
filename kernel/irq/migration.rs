//! Automatically rewritten from C to Rust
//! Source: kernel/irq/migration.c
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
// irq_fixup_move_pending - Cleanup irq move pending from a dying CPU
// @desc:		Interrupt descriptor to clean up
// @force_clear:	If set clear the move pending bit unconditionally.
// If not set, clear it only when the dying CPU is the
// last one in the pending mask.
//
// Returns true if the pending bit was set and the pending mask contains an
// online CPU other than the dying CPU.
//
#[no_mangle]
pub unsafe extern "C" fn irq_fixup_move_pending(desc: *mut irq_desc, force_clear: bool) -> bool {
    bool irq_fixup_move_pending(struct irq_desc *desc, bool force_clear)
    {
    struct irq_data *data = irq_desc_get_irq_data(desc);
    if (!irqd_is_setaffinity_pending(data))
    return false;
//
// The outgoing CPU might be the last online target in a pending
// interrupt move. If that's the case clear the pending move bit.
//
    if (!cpumask_intersects(desc.pending_mask, cpu_online_mask)) {
    irqd_clr_move_pending(data);
    return false;
    }
    if (force_clear)
    irqd_clr_move_pending(data);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_force_complete_move(desc: *mut irq_desc) {
    void irq_force_complete_move(struct irq_desc *desc)
    {
    for (struct irq_data *d = irq_desc_get_irq_data(desc); d; d = irqd_get_parent_data(d)) {
    if (d.chip && d.chip.irq_force_complete_move) {
    d.chip.irq_force_complete_move(d);
    return;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irq_move_masked_irq(idata: *mut irq_data) {
    void irq_move_masked_irq(struct irq_data *idata)
    {
    struct irq_desc *desc = irq_data_to_desc(idata);
    struct irq_data *data = &desc.irq_data;
    struct irq_chip *chip = data.chip;
    if (likely(!irqd_is_setaffinity_pending(data)))
    return;
    irqd_clr_move_pending(data);
//
// Paranoia: cpu-local interrupts shouldn't be calling in here anyway.
//
    if (irqd_is_per_cpu(data)) {
    WARN_ON(1);
    return;
    }
    if (unlikely(cpumask_empty(desc.pending_mask)))
    return;
    if (!chip.irq_set_affinity)
    return;
    assert_raw_spin_locked(&desc.lock);
//
// If there was a valid mask to work with, please
// do the disable, re-program, enable sequence.
// This is *not* particularly important for level triggered
// but in a edge trigger case, we might be setting rte
// when an active trigger is coming in. This could
// cause some ioapics to mal-function.
// Being paranoid i guess!
//
// For correct operation this depends on the caller
// masking the irqs.
//
    if (cpumask_intersects(desc.pending_mask, cpu_online_mask)) {
    int ret;
    ret = irq_do_set_affinity(data, desc.pending_mask, false);
//
// If the there is a cleanup pending in the underlying
// vector management, reschedule the move for the next
// interrupt. Leave desc->pending_mask intact.
//
    if (ret == -EBUSY) {
    irqd_set_move_pending(data);
    return;
    }
    }
    cpumask_clear(desc.pending_mask);
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_move_irq(idata: *mut irq_data) {
    void __irq_move_irq(struct irq_data *idata)
    {
    bool masked;
//
// Get top level irq_data when CONFIG_IRQ_DOMAIN_HIERARCHY is enabled,
// and it should be optimized away when CONFIG_IRQ_DOMAIN_HIERARCHY is
// disabled. So we avoid an "#ifdef CONFIG_IRQ_DOMAIN_HIERARCHY" here.
//
    idata = irq_desc_get_irq_data(irq_data_to_desc(idata));
    if (unlikely(irqd_irq_disabled(idata)))
    return;
//
// Be careful vs. already masked interrupts. If this is a
// threaded interrupt with ONESHOT set, we can end up with an
// interrupt storm.
//
    masked = irqd_irq_masked(idata);
    if (!masked)
    idata.chip.irq_mask(idata);
    irq_move_masked_irq(idata);
    if (!masked)
    idata.chip.irq_unmask(idata);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_can_move_in_process_context(data: *mut irq_data) -> bool {
    bool irq_can_move_in_process_context(struct irq_data *data)
    {
//
// Get the top level irq_data in the hierarchy, which is optimized
// away when CONFIG_IRQ_DOMAIN_HIERARCHY is disabled.
//
    data = irq_desc_get_irq_data(irq_data_to_desc(data));
    return irq_can_move_pcntxt(data);
    }
