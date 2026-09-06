//! Automatically rewritten from C to Rust
//! Source: kernel/irq_work.c
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

macro_rules! EXPORT_SYMBOL { ($($tt:tt)*) => {}; }
macro_rules! EXPORT_SYMBOL_GPL { ($($tt:tt)*) => {}; }
macro_rules! MODULE_LICENSE { ($($tt:tt)*) => {}; }
macro_rules! MODULE_AUTHOR { ($($tt:tt)*) => {}; }
macro_rules! MODULE_DESCRIPTION { ($($tt:tt)*) => {}; }
macro_rules! MODULE_ALIAS { ($($tt:tt)*) => {}; }
macro_rules! module_init { ($($tt:tt)*) => {}; }
macro_rules! module_exit { ($($tt:tt)*) => {}; }
macro_rules! early_initcall { ($($tt:tt)*) => {}; }
macro_rules! core_initcall { ($($tt:tt)*) => {}; }
macro_rules! postcore_initcall { ($($tt:tt)*) => {}; }
macro_rules! arch_initcall { ($($tt:tt)*) => {}; }
macro_rules! subsys_initcall { ($($tt:tt)*) => {}; }
macro_rules! fs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct seq_file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct task_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cred { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct file { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notifier_block { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct raw_notifier_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2010 Red Hat, Inc., Peter Zijlstra
//
// Provides a framework for enqueueing and running callbacks from hardirq
// context. The enqueueing is NMI-safe.
//
// static DEFINE_PER_CPU(struct llist_head, raised_list);
// static DEFINE_PER_CPU(struct llist_head, lazy_list);
// static DEFINE_PER_CPU(struct task_struct *, irq_workd);
#[no_mangle]
unsafe extern "C" fn wake_irq_workd() {
    struct task_struct *tsk = __this_cpu_read(irq_workd);
    if (!llist_empty(this_cpu_ptr(&lazy_list)) && tsk) {
    wake_up_process(tsk);
    }
    }

#[no_mangle]
unsafe extern "C" fn irq_work_wake(entry: *mut irq_work) {
    wake_irq_workd();
    }
    static DEFINE_PER_CPU(struct irq_work, irq_work_wakeup) =
// IRQ_WORK_INIT_HARD;

#[no_mangle]
unsafe extern "C" fn irq_workd_should_run(cpu: c_uint) -> c_int {
    return !llist_empty(this_cpu_ptr(&lazy_list));
    }
//
// Claim the entry so that no one else will poke at it.
//
#[no_mangle]
unsafe extern "C" fn irq_work_claim(work: *mut irq_work) -> bool {
    let mut oflags = 0;
    oflags = atomic_fetch_or(IRQ_WORK_CLAIMED | CSD_TYPE_IRQ_WORK, &work.node.a_flags);
//
// If the work is already pending, no need to raise the IPI.
// The pairing smp_mb() in irq_work_single() makes sure
// everything we did before is visible.
//
    if (oflags & IRQ_WORK_PENDING) {
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_irq_work_raise() -> void __weak {
//
// Lame architectures will get the timer tick callback
//
    }
#[no_mangle]
unsafe extern "C" fn irq_work_raise(work: *mut irq_work) -> __always_inline void {
    if (trace_ipi_send_cpu_enabled() && arch_irq_work_has_interrupt()) {
    trace_call__ipi_send_cpu(smp_processor_id(), _RET_IP_, work.func);
    }
    arch_irq_work_raise();
    }
// Enqueue on current CPU, work must already be claimed and preempt disabled
#[no_mangle]
unsafe extern "C" fn __irq_work_queue_local(work: *mut irq_work) {
    let mut list = core::ptr::null_mut();
pub static mut rt_lazy_work: bool = false;
pub static mut lazy_work: bool = false;
    let mut work_flags = 0;
    work_flags = atomic_read(&work.node.a_flags);
    if (work_flags & IRQ_WORK_LAZY) {
    lazy_work = true;
    }
    else if (IS_ENABLED(CONFIG_PREEMPT_RT) &&
    !(work_flags & IRQ_WORK_HARD_IRQ))
    rt_lazy_work = true;
    if (lazy_work || rt_lazy_work) {
    list = this_cpu_ptr(&lazy_list);
    }
    else {
    list = this_cpu_ptr(&raised_list);
    }
    if (!llist_add(&work.node.llist, list)) {
    return;
    }
// If the work is "lazy", handle it from next tick if any
    if (!lazy_work || tick_nohz_tick_stopped()) {
    irq_work_raise(work);
    }
    }
// Enqueue the irq work @work on the current CPU
#[no_mangle]
pub unsafe extern "C" fn irq_work_queue(work: *mut irq_work) -> bool {
// Only queue if not already pending
    if (!irq_work_claim(work)) {
    return false;
    }
// Queue the entry and raise the IPI if needed.
    preempt_disable();
    __irq_work_queue_local(work);
    preempt_enable();
    return true;
    }
// EXPORT_SYMBOL_GPL;
//
// Enqueue the irq_work @work on @cpu unless it's already pending
// somewhere.
//
// Can be re-enqueued while the callback is still in progress.
//
#[no_mangle]
pub unsafe extern "C" fn irq_work_queue_on(work: *mut irq_work, cpu: c_int) -> bool {

    return irq_work_queue(work);

// All work should have been flushed before going offline
// WARN_ON_ONCE;
// Only queue if not already pending
    if (!irq_work_claim(work)) {
    return false;
    }
    kasan_record_aux_stack(work);
    preempt_disable();
    if (cpu != smp_processor_id()) {
// Arch remote IPI send/receive backend aren't NMI safe
// WARN_ON_ONCE;
//
// On PREEMPT_RT the items which are not marked as
// IRQ_WORK_HARD_IRQ are added to the lazy list and a HARD work
// item is used on the remote CPU to wake the thread.
//
    if (IS_ENABLED(CONFIG_PREEMPT_RT) &&
    !(atomic_read(&work.node.a_flags) & IRQ_WORK_HARD_IRQ)) {
    if (!llist_add(&work.node.llist, &per_cpu(lazy_list, cpu))) {
    goto out;
    }
    work = &per_cpu(irq_work_wakeup, cpu);
    if (!irq_work_claim(work)) {
    goto out;
    }
    }
    __smp_call_single_queue(cpu, &work.node.llist);
    } else {
    __irq_work_queue_local(work);
    }
    out:
    preempt_enable();
    return true;

    }
#[no_mangle]
pub unsafe extern "C" fn irq_work_needs_cpu() -> bool {
    struct llist_head *raised, *lazy;
    raised = this_cpu_ptr(&raised_list);
    lazy = this_cpu_ptr(&lazy_list);
    if (llist_empty(raised) || arch_irq_work_has_interrupt()) {
    if (llist_empty(lazy)) {
    return false;
    }
    }
// All work should have been flushed before going offline
// WARN_ON_ONCE;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_work_single(arg: *mut c_void) {
    struct irq_work *work = arg;
    let mut flags = 0;
//
// Clear the PENDING bit, after this point the @work can be re-used.
// The PENDING bit acts as a lock, and we own it, so we can clear it
// without atomic ops.
//
    flags = atomic_read(&work.node.a_flags);
    flags &= ~IRQ_WORK_PENDING;
    atomic_set(&work.node.a_flags, flags);
//
// See irq_work_claim().
//
    smp_mb();
    lockdep_irq_work_enter(flags);
    work.func(work);
    lockdep_irq_work_exit(flags);
//
// Clear the BUSY bit, if set, and return to the free state if no-one
// else claimed it meanwhile.
//
    (void)atomic_cmpxchg(&work.node.a_flags, flags, flags & ~IRQ_WORK_BUSY);
    if ((IS_ENABLED(CONFIG_PREEMPT_RT) && !irq_work_is_hard(work)) ||
    !arch_irq_work_has_interrupt())
    rcuwait_wake_up(&work.irqwait);
    }
#[no_mangle]
unsafe extern "C" fn irq_work_run_list(list: *mut llist_head) {
    struct irq_work *work, *tmp;
    let mut llnode = core::ptr::null_mut();
//
// On PREEMPT_RT IRQ-work which is not marked as HARD will be processed
// in a per-CPU thread in preemptible context. Only the items which are
// marked as IRQ_WORK_HARD_IRQ will be processed in hardirq context.
//
// BUG_ON;
    if (llist_empty(list)) {
    return;
    }
    llnode = llist_del_all(list);
    llist_for_each_entry_safe(work, tmp, llnode, node.llist)
    irq_work_single(work);
    }
//
// hotplug calls this through:
// hotplug_cfd() -> flush_smp_call_function_queue()
//
#[no_mangle]
pub unsafe extern "C" fn irq_work_run() {
    irq_work_run_list(this_cpu_ptr(&raised_list));
    if (!IS_ENABLED(CONFIG_PREEMPT_RT)) {
    irq_work_run_list(this_cpu_ptr(&lazy_list));
    }
    else {
    wake_irq_workd();
    }
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
pub unsafe extern "C" fn irq_work_tick() {
    struct llist_head *raised = this_cpu_ptr(&raised_list);
    if (!llist_empty(raised) && !arch_irq_work_has_interrupt()) {
    irq_work_run_list(raised);
    }
    if (!IS_ENABLED(CONFIG_PREEMPT_RT)) {
    irq_work_run_list(this_cpu_ptr(&lazy_list));
    }
    else {
    wake_irq_workd();
    }
    }
//
// Synchronize against the irq_work @entry, ensures the entry is not
// currently in use.
//
#[no_mangle]
pub unsafe extern "C" fn irq_work_sync(work: *mut irq_work) {
    lockdep_assert_irqs_enabled();
    might_sleep();
    if ((IS_ENABLED(CONFIG_PREEMPT_RT) && !irq_work_is_hard(work)) ||
    !arch_irq_work_has_interrupt()) {
    rcuwait_wait_event(&work.irqwait, !irq_work_is_busy(work),
    TASK_UNINTERRUPTIBLE);
//
// Ensure irq_work_single() does not access @work
// after removing IRQ_WORK_BUSY. It is always
// accessed within a RCU-read section.
//
    synchronize_rcu();
    return;
    }
    while (irq_work_is_busy(work))
    cpu_relax();
    }
// EXPORT_SYMBOL_GPL;
#[no_mangle]
unsafe extern "C" fn run_irq_workd(cpu: c_uint) {
    guard(rcu)();
    irq_work_run_list(this_cpu_ptr(&lazy_list));
    }
#[no_mangle]
unsafe extern "C" fn irq_workd_setup(cpu: c_uint) {
    sched_set_fifo_low(current);
    }
pub static mut smp_hotplug_thread: usize = 0;
#[no_mangle]
unsafe extern "C" fn irq_work_init_threads() -> __init int {
    if (IS_ENABLED(CONFIG_PREEMPT_RT)) {
// BUG_ON;
    }
    return 0;
    }
// early_initcall;