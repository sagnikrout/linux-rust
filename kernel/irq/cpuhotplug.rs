//! Automatically rewritten from C to Rust
//! Source: kernel/irq/cpuhotplug.c
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
// === KERNEL_MACRO_PRELUDE_START ===
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
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! IS_ENABLED { ($($tt:tt)*) => { false }; }
macro_rules! DECLARE_WORK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_WAKE_Q { ($($tt:tt)*) => {}; }
macro_rules! LLIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! LIST_HEAD { ($($tt:tt)*) => {}; }
macro_rules! SET_UID { ($($tt:tt)*) => {}; }
macro_rules! SET_GID { ($($tt:tt)*) => {}; }
macro_rules! list_for_each_entry { ($($tt:tt)*) => { if false }; }
macro_rules! list_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! llist_for_each_entry_safe { ($($tt:tt)*) => { if false }; }
macro_rules! pr_info_once { ($($tt:tt)*) => {}; }
macro_rules! pr_info { ($($tt:tt)*) => {}; }
macro_rules! pr_warn { ($($tt:tt)*) => {}; }
macro_rules! pr_err { ($($tt:tt)*) => {}; }
macro_rules! pr_debug { ($($tt:tt)*) => {}; }
macro_rules! early_param { ($($tt:tt)*) => {}; }
macro_rules! BUILD_BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! WARN_ON { ($($tt:tt)*) => { false }; }
macro_rules! WARN_ON_ONCE { ($($tt:tt)*) => { false }; }
macro_rules! BUG_ON { ($($tt:tt)*) => {}; }
macro_rules! BUG { () => {}; }
macro_rules! IS_ERR { ($($tt:tt)*) => { false }; }
macro_rules! PTR_ERR { ($($tt:tt)*) => { 0 }; }
macro_rules! ERR_PTR { ($($tt:tt)*) => { core::ptr::null_mut() }; }

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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_namespace { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc64_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kern_ipc_perm { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_params { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_queue { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_msgseg { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_sender { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msg_receiver { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sembuf { pub sem_num: u16, pub sem_op: i16, pub sem_flg: i16 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sem_array { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmid_kernel { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shm_file_data { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wake_q_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct work_struct { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct llist_head { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct list_head { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;
pub type key_t = i32;
pub type kuid_t = u32;
pub type kgid_t = u32;
pub type int = c_int;
pub type uint = c_uint;
pub type ulong = c_ulong;
pub type long = c_long;
pub type void = c_void;

// Standard Linux Error Codes
pub const EPERM: c_int = 1;
pub const ENOENT: c_int = 2;
pub const ESRCH: c_int = 3;
pub const EINTR: c_int = 4;
pub const EIO: c_int = 5;
pub const ENXIO: c_int = 6;
pub const E2BIG: c_int = 7;
pub const ENOEXEC: c_int = 8;
pub const EBADF: c_int = 9;
pub const ECHILD: c_int = 10;
pub const EAGAIN: c_int = 11;
pub const ENOMEM: c_int = 12;
pub const EACCES: c_int = 13;
pub const EFAULT: c_int = 14;
pub const EBUSY: c_int = 16;
pub const EEXIST: c_int = 17;
pub const EXDEV: c_int = 18;
pub const ENODEV: c_int = 19;
pub const ENOTDIR: c_int = 20;
pub const EISDIR: c_int = 21;
pub const EINVAL: c_int = 22;
pub const ENFILE: c_int = 23;
pub const EMFILE: c_int = 24;
pub const ENOSPC: c_int = 28;
pub const EROFS: c_int = 30;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn printk(fmt: *const c_char, ...) -> c_int;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
}
// === KERNEL_MACRO_PRELUDE_END ===


// SPDX-License-Identifier: GPL-2.0
//
// Generic cpu hotunplug interrupt migration code copied from the
// arch/arm implementation
//
// Copyright (C) Russell King
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

// For !GENERIC_IRQ_EFFECTIVE_AFF_MASK this looks at general affinity mask
#[no_mangle]
pub unsafe extern "C" fn irq_needs_fixup(d: *mut irq_data) -> bool {
    let mut m = irq_data_get_effective_affinity_mask(d);
pub static mut cpu: c_uint = 0;

//
// The cpumask_empty() check is a workaround for interrupt chips,
// which do not implement effective affinity, but the architecture has
// enabled the config switch. Use the general affinity mask instead.
//
    if (cpumask_empty(m)) {
    m = irq_data_get_affinity_mask(d);
    }
//
// Sanity check. If the mask is not empty when excluding the outgoing
// CPU then it must contain at least one online CPU. The outgoing CPU
// has been removed from the online mask already.
//
    if (cpumask_any_but(m, cpu) < nr_cpu_ids &&
    !cpumask_intersects(m, cpu_online_mask)) {
//
// If this happens then there was a missed IRQ fixup at some
// point. Warn about it and enforce fixup.
//
    pr_warn!("Eff. affinity %*pbl of IRQ %u contains only offline CPUs after offlining CPU %u\n",
    cpumask_pr_args(m), d.irq, cpu);
    return true;
    }

    return cpumask_test_cpu(cpu, m);
    }
#[no_mangle]
unsafe extern "C" fn migrate_one_irq(desc: *mut irq_desc) -> bool {
    let mut d = irq_desc_get_irq_data(desc);
    let mut chip = irq_data_get_irq_chip(d);
pub static mut maskchip: bool = false;
pub static mut affinity: *mut c_void = core::ptr::null_mut();
pub static mut brokeaff: bool = false;
    let mut err = 0;
//
// IRQ chip might be already torn down, but the irq descriptor is
// still in the radix tree. Also if the chip has no affinity setter,
// nothing can be done here.
//
    if (!chip || !chip.irq_set_affinity) {
    pr_debug!("IRQ %u: Unable to migrate away\n", d.irq);
    return false;
    }
//
// Complete an eventually pending irq move cleanup. If this
// interrupt was moved in hard irq context, then the vectors need
// to be cleaned up. It can't wait until this interrupt actually
// happens and this CPU was involved.
//
    irq_force_complete_move(desc);
//
// No move required, if:
// - Interrupt is per cpu
// - Interrupt is not started
// - Affinity mask does not include this CPU.
//
// Note: Do not check desc->action as this might be a chained
// interrupt.
//
    if (irqd_is_per_cpu(d) || !irqd_is_started(d) || !irq_needs_fixup(d)) {
//
// If an irq move is pending, abort it if the dying CPU is
// the sole target.
//
    irq_fixup_move_pending(desc, false);
    return false;
    }
//
// If there is a setaffinity pending, then try to reuse the pending
// mask, so the last change of the affinity does not get lost. If
// there is no move pending or the pending mask does not contain
// any online CPU, use the current affinity mask.
//
    if (irq_fixup_move_pending(desc, true)) {
    affinity = irq_desc_get_pending_mask(desc);
    }
    else {
    affinity = irq_data_get_affinity_mask(d);
    }
// Mask the chip for interrupts which cannot move in process context
    if (maskchip && chip.irq_mask) {
    chip.irq_mask(d);
    }
    if (!cpumask_intersects(affinity, cpu_online_mask)) {
//
// If the interrupt is managed, then shut it down and leave
// the affinity untouched.
//
    if (irqd_affinity_is_managed(d)) {
    irqd_set_managed_shutdown(d);
    irq_shutdown_and_deactivate(desc);
    return false;
    }
    affinity = cpu_online_mask;
    brokeaff = true;
    }
//
// Do not set the force argument of irq_do_set_affinity() as this
// disables the masking of offline CPUs from the supplied affinity
// mask and therefore might keep/reassign the irq to the outgoing
// CPU.
//
    err = irq_do_set_affinity(d, affinity, false);
//
// If there are online CPUs in the affinity mask, but they have no
// vectors left to make the migration work, try to break the
// affinity by migrating to any online CPU.
//
    if (err == -ENOSPC && !irqd_affinity_is_managed(d) && affinity != cpu_online_mask) {
    pr_debug!("IRQ%u: set affinity failed for %*pbl, re-try with online CPUs\n",
    d.irq, cpumask_pr_args(affinity));
    affinity = cpu_online_mask;
    brokeaff = true;
    err = irq_do_set_affinity(d, affinity, false);
    }
    if (err) {
    pr_warn_ratelimited("IRQ%u: set affinity failed(%d).\n",
    d.irq, err);
    brokeaff = false;
    }
    if (maskchip && chip.irq_unmask) {
    chip.irq_unmask(d);
    }
    return brokeaff;
    }
//
// irq_migrate_all_off_this_cpu - Migrate irqs away from offline cpu
//
// The current CPU has been marked offline.  Migrate IRQs off this CPU.
// If the affinity settings do not allow other CPUs, force them onto any
// available CPU.
//
// Note: we must iterate over all IRQs, whether they have an attached
// action structure or not, as we need to get chained interrupts too.
//
#[no_mangle]
pub unsafe extern "C" fn irq_migrate_all_off_this_cpu() {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut irq = 0;
    for_each_active_irq(irq) {
    let mut affinity_broken = 0;
    desc = irq_to_desc(irq);
    scoped_guard(raw_spinlock, &desc.lock) {
    affinity_broken = migrate_one_irq(desc);
    if (affinity_broken && desc.affinity_notify) {
    irq_affinity_schedule_notify_work(desc);
    }
    }
    if (affinity_broken) {
    pr_debug_ratelimited("IRQ %u: no longer affine to CPU%u\n",
    irq, smp_processor_id());
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn hk_should_isolate(data: *mut irq_data, cpu: c_uint) -> bool {
pub static mut hk_mask: *mut c_void = core::ptr::null_mut();
    if (!housekeeping_enabled(HK_TYPE_MANAGED_IRQ)) {
    return false;
    }
    hk_mask = housekeeping_cpumask(HK_TYPE_MANAGED_IRQ);
    if (cpumask_subset(irq_data_get_effective_affinity_mask(data), hk_mask)) {
    return false;
    }
    return cpumask_test_cpu(cpu, hk_mask);
    }
#[no_mangle]
unsafe extern "C" fn irq_restore_affinity_of_irq(desc: *mut irq_desc, cpu: c_uint) {
    let mut data = irq_desc_get_irq_data(desc);
    let mut affinity = irq_data_get_affinity_mask(data);
    if (!irqd_affinity_is_managed(data) || !desc.action ||
    !irq_data_get_irq_chip(data) || !cpumask_test_cpu(cpu, affinity)) {
    return;
    }
    if (irqd_is_managed_and_shutdown(data)) {
    irq_startup_managed(desc);
    }
//
// If the interrupt can only be directed to a single target
// CPU then it is already assigned to a CPU in the affinity
// mask. No point in trying to move it around unless the
// isolation mechanism requests to move it to an upcoming
// housekeeping CPU.
//
    if (!irqd_is_single_target(data) || hk_should_isolate(data, cpu)) {
    irq_set_affinity_locked(data, affinity, false);
    }
    }
//
// irq_affinity_online_cpu - Restore affinity for managed interrupts
// @cpu:	Upcoming CPU for which interrupts should be restored
//
#[no_mangle]
pub unsafe extern "C" fn irq_affinity_online_cpu(cpu: c_uint) -> c_int {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut irq = 0;
    irq_lock_sparse();
    for_each_active_irq(irq) {
    desc = irq_to_desc(irq);
    scoped_guard(raw_spinlock_irq, &desc.lock)
    irq_restore_affinity_of_irq(desc, cpu);
    }
    irq_unlock_sparse();
    return 0;
    }