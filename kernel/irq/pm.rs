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
// Copyright (C) 2009 Rafael J. Wysocki <rjw@sisk.pl>, Novell Inc.
//
// This file contains power management functions related to interrupts.
//

#[no_mangle]
pub unsafe extern "C" fn irq_pm_handle_wakeup(desc: *mut irq_desc) {
    irqd_clear(&desc.irq_data, IRQD_WAKEUP_ARMED);
    desc.istate |= IRQS_SUSPENDED | IRQS_PENDING;
    desc.depth += 1;
    irq_disable(desc);
    pm_system_irq_wakeup(irq_desc_get_irq(desc));
    }
//
// Called from __setup_irq() with desc->lock held after @action has
// been installed in the action chain.
//
#[no_mangle]
pub unsafe extern "C" fn irq_pm_install_action(desc: *mut irq_desc, action: *mut irqaction) {
    desc.nr_actions += 1;
    if (action.flags & IRQF_FORCE_RESUME) {
    desc.force_resume_depth += 1;
    }
    WARN_ON_ONCE!(desc.force_resume_depth &&
    desc.force_resume_depth != desc.nr_actions);
    if (action.flags & IRQF_NO_SUSPEND) {
    desc.no_suspend_depth += 1;
    }

    else if (action.flags & IRQF_COND_SUSPEND) {
    desc.cond_suspend_depth += 1;
    }
    WARN_ON_ONCE!(desc.no_suspend_depth &&
    (desc.no_suspend_depth + desc.cond_suspend_depth) != desc.nr_actions);
    }
//
// Called from __free_irq() with desc->lock held after @action has
// been removed from the action chain.
//
#[no_mangle]
pub unsafe extern "C" fn irq_pm_remove_action(desc: *mut irq_desc, action: *mut irqaction) {
    desc.nr_actions -= 1;
    if (action.flags & IRQF_FORCE_RESUME) {
    desc.force_resume_depth -= 1;
    }
    if (action.flags & IRQF_NO_SUSPEND) {
    desc.no_suspend_depth -= 1;
    }

    else if (action.flags & IRQF_COND_SUSPEND) {
    desc.cond_suspend_depth -= 1;
    }
    }
#[no_mangle]
unsafe extern "C" fn suspend_device_irq(desc: *mut irq_desc) -> bool {
pub static mut chipflags: c_ulong = 0;
    let mut irqd = &desc.irq_data;
    if (!desc.action || irq_desc_is_chained(desc) ||
    desc.no_suspend_depth) {
    return false;
    }
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
    if (chipflags & IRQCHIP_MASK_ON_SUSPEND) {
    mask_irq(desc);
    }
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
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut irq = 0;
    for_each_irq_desc(irq, desc) {
    let mut sync = 0;
    if (irq_settings_is_nested_thread(desc)) {
    continue;
    }
    scoped_guard(raw_spinlock_irqsave, &desc.lock)
    sync = suspend_device_irq(desc);
    if (sync) {
    synchronize_irq(irq);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn resume_irq(desc: *mut irq_desc) {
    let mut irqd = &desc.irq_data;
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
    if (desc.istate & IRQS_SUSPENDED) {
// goto;
    }
// Force resume the interrupt?
    if (!desc.force_resume_depth) {
    return;
    }
// Pretend that it got disabled !
    desc.depth += 1;
    irq_state_set_disabled(desc);
    irq_state_set_masked(desc);
// label;
    desc.istate &= ~IRQS_SUSPENDED;
    __enable_irq(desc);
    }
#[no_mangle]
unsafe extern "C" fn resume_irqs(want_early: bool) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut irq = 0;
    for_each_irq_desc(irq, desc) {
pub static mut is_early: bool = false;
    if (!is_early && want_early) {
    continue;
    }
    if (irq_settings_is_nested_thread(desc)) {
    continue;
    }
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
    scoped_irqdesc_get_and_buslock(irq, IRQ_GET_DESC_CHECK_GLOBAL) {
    let mut desc = scoped_irqdesc;
    if (!(desc.istate & IRQS_SUSPENDED) || !irqd_is_wakeup_set(&desc.irq_data)) {
    return;
    }
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
    resume_irqs(true);
    }
pub static mut syscore_ops: usize = 0;
pub static mut syscore: usize = 0;
#[no_mangle]
unsafe extern "C" fn irq_pm_init_ops() -> c_int {
    register_syscore(&irq_pm_syscore);
    return 0;
    }
    device_initcall!(irq_pm_init_ops);
//
// resume_device_irqs - enable interrupt lines disabled by suspend_device_irqs()
//
// Enable all non-%IRQF_EARLY_RESUME interrupt lines previously
// disabled by suspend_device_irqs() that have the IRQS_SUSPENDED flag
// set as well as those with %IRQF_FORCE_RESUME.
//
#[no_mangle]
pub unsafe extern "C" fn resume_device_irqs() {
    resume_irqs(false);
    }