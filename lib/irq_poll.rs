//! Automatically rewritten from C to Rust
//! Source: lib/irq_poll.c
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
// Functions related to interrupt-poll handling in the block layer. This
// is similar to NAPI for network devices.
//

    let mut __read_mostly: static unsigned int irq_poll_budget = 256;
    static DEFINE_PER_CPU(struct list_head, blk_cpu_iopoll);
//
// irq_poll_sched - Schedule a run of the iopoll handler
// @iop:      The parent iopoll structure
//
// Description:
// Add this irq_poll structure to the pending poll list and trigger the
// raise of the blk iopoll softirq.
//
#[no_mangle]
pub unsafe extern "C" fn irq_poll_sched(iop: *mut irq_poll) {
    void irq_poll_sched(struct irq_poll *iop)
    {
    unsigned long flags;
    if (test_bit(IRQ_POLL_F_DISABLE, &iop.state))
    return;
    if (test_and_set_bit(IRQ_POLL_F_SCHED, &iop.state))
    return;
    local_irq_save(flags);
    list_add_tail(&iop.list, this_cpu_ptr(&blk_cpu_iopoll));
    raise_softirq_irqoff(IRQ_POLL_SOFTIRQ);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(irq_poll_sched);
//
// __irq_poll_complete - Mark this @iop as un-polled again
// @iop:      The parent iopoll structure
//
// Description:
// See irq_poll_complete(). This function must be called with interrupts
// disabled.
//
#[no_mangle]
unsafe extern "C" fn __irq_poll_complete(iop: *mut irq_poll) {
    static void __irq_poll_complete(struct irq_poll *iop)
    {
    list_del(&iop.list);
    smp_mb__before_atomic();
    clear_bit_unlock(IRQ_POLL_F_SCHED, &iop.state);
    }
//
// irq_poll_complete - Mark this @iop as un-polled again
// @iop:      The parent iopoll structure
//
// Description:
// If a driver consumes less than the assigned budget in its run of the
// iopoll handler, it'll end the polled mode by calling this function. The
// iopoll handler will not be invoked again before irq_poll_sched()
// is called.
//
#[no_mangle]
pub unsafe extern "C" fn irq_poll_complete(iop: *mut irq_poll) {
    void irq_poll_complete(struct irq_poll *iop)
    {
    unsigned long flags;
    local_irq_save(flags);
    __irq_poll_complete(iop);
    local_irq_restore(flags);
    }
    EXPORT_SYMBOL(irq_poll_complete);
#[no_mangle]
unsafe extern "C" fn irq_poll_softirq() -> void __latent_entropy {
    static void __latent_entropy irq_poll_softirq(void)
    {
    struct list_head *list = this_cpu_ptr(&blk_cpu_iopoll);
    let mut rearm: c_int = 0, budget = irq_poll_budget;
    let mut start_time: c_ulong = jiffies;
    local_irq_disable();
    while (!list_empty(list)) {
    struct irq_poll *iop;
    int work, weight;
//
// If softirq window is exhausted then punt.
//
    if (budget <= 0 || time_after(jiffies, start_time)) {
    rearm = 1;
    break;
    }
    local_irq_enable();
// Even though interrupts have been re-enabled, this
// access is safe because interrupts can only add new
// entries to the tail of this list, and only ->poll()
// calls can remove this head entry from the list.
//
    iop = list_entry(list.next, struct irq_poll, list);
    weight = iop.weight;
    work = 0;
    if (test_bit(IRQ_POLL_F_SCHED, &iop.state))
    work = iop.poll(iop, weight);
    budget -= work;
    local_irq_disable();
//
// Drivers must not modify the iopoll state, if they
// consume their assigned weight (or more, some drivers can't
// easily just stop processing, they have to complete an
// entire mask of commands).In such cases this code
// still "owns" the iopoll instance and therefore can
// move the instance around on the list at-will.
//
    if (work >= weight) {
    if (test_bit(IRQ_POLL_F_DISABLE, &iop.state))
    __irq_poll_complete(iop);
    else
    list_move_tail(&iop.list, list);
    }
    }
    if (rearm)
    __raise_softirq_irqoff(IRQ_POLL_SOFTIRQ);
    local_irq_enable();
    }
//
// irq_poll_disable - Disable iopoll on this @iop
// @iop:      The parent iopoll structure
//
// Description:
// Disable io polling and wait for any pending callbacks to have completed.
//
#[no_mangle]
pub unsafe extern "C" fn irq_poll_disable(iop: *mut irq_poll) {
    void irq_poll_disable(struct irq_poll *iop)
    {
    set_bit(IRQ_POLL_F_DISABLE, &iop.state);
    while (test_and_set_bit(IRQ_POLL_F_SCHED, &iop.state))
    msleep(1);
    clear_bit(IRQ_POLL_F_DISABLE, &iop.state);
    }
    EXPORT_SYMBOL(irq_poll_disable);
//
// irq_poll_enable - Enable iopoll on this @iop
// @iop:      The parent iopoll structure
//
// Description:
// Enable iopoll on this @iop. Note that the handler run will not be
// scheduled, it will only mark it as active.
//
#[no_mangle]
pub unsafe extern "C" fn irq_poll_enable(iop: *mut irq_poll) {
    void irq_poll_enable(struct irq_poll *iop)
    {
    BUG_ON(!test_bit(IRQ_POLL_F_SCHED, &iop.state));
    smp_mb__before_atomic();
    clear_bit_unlock(IRQ_POLL_F_SCHED, &iop.state);
    }
    EXPORT_SYMBOL(irq_poll_enable);
//
// irq_poll_init - Initialize this @iop
// @iop:      The parent iopoll structure
// @weight:   The default weight (or command completion budget)
// @poll_fn:  The handler to invoke
//
// Description:
// Initialize and enable this irq_poll structure.
//
#[no_mangle]
pub unsafe extern "C" fn irq_poll_init(iop: *mut irq_poll, weight: c_int, poll_fn: *mut irq_poll_fn) {
    void irq_poll_init(struct irq_poll *iop, int weight, irq_poll_fn *poll_fn)
    {
    memset(iop, 0, sizeof(*iop));
    INIT_LIST_HEAD(&iop.list);
    iop.weight = weight;
    iop.poll = poll_fn;
    }
    EXPORT_SYMBOL(irq_poll_init);
#[no_mangle]
unsafe extern "C" fn irq_poll_cpu_dead(cpu: c_uint) -> c_int {
    static int irq_poll_cpu_dead(unsigned int cpu)
    {
//
// If a CPU goes away, splice its entries to the current CPU and
// set the POLL softirq bit. The local_bh_disable()/enable() pair
// ensures that it is handled. Otherwise the current CPU could
// reach idle with the POLL softirq pending.
//
    local_bh_disable();
    local_irq_disable();
    list_splice_init(&per_cpu(blk_cpu_iopoll, cpu),
    this_cpu_ptr(&blk_cpu_iopoll));
    __raise_softirq_irqoff(IRQ_POLL_SOFTIRQ);
    local_irq_enable();
    local_bh_enable();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_poll_setup() -> __init int {
    static __init int irq_poll_setup(void)
    {
    int i;
    for_each_possible_cpu(i)
    INIT_LIST_HEAD(&per_cpu(blk_cpu_iopoll, i));
    open_softirq(IRQ_POLL_SOFTIRQ, irq_poll_softirq);
    cpuhp_setup_state_nocalls(CPUHP_IRQ_POLL_DEAD, "irq_poll:dead", core::ptr::null_mut(),
    irq_poll_cpu_dead);
    return 0;
    }
    subsys_initcall(irq_poll_setup);
