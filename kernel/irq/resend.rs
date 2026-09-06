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

// hlist_head to handle software resend of interrupts: HLIST_HEAD(irq_resend_list);
pub static mut irq_resend_lock: usize = 0;
//
// Run software resends of IRQ's
//
#[no_mangle]
unsafe extern "C" fn resend_irqs(unused: *mut tasklet_struct) {
    guard(raw_spinlock_irq)(&irq_resend_lock);
    while (!hlist_empty(&irq_resend_list)) {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    desc = hlist_entry(irq_resend_list.first, irq_desc,  resend_node);
    hlist_del_init(&desc.resend_node);
    raw_spin_unlock(&irq_resend_lock);
    desc.handle_irq(desc);
    raw_spin_lock(&irq_resend_lock);
    }
    }
// Tasklet to handle resend: DECLARE_TASKLET(resend_tasklet, resend_irqs);
#[no_mangle]
unsafe extern "C" fn irq_sw_resend(desc: *mut irq_desc) -> c_int {
//
// Validate whether this interrupt can be safely injected from
// non interrupt context
//
    if (irqd_is_handle_enforce_irqctx(&desc.irq_data)) {
    return -EINVAL;
    }
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
    if (!desc.parent_irq) {
    return -EINVAL;
    }
    desc = irq_to_desc(desc.parent_irq);
    if (!desc) {
    return -EINVAL;
    }
    }
// Add to resend_list and activate the softirq:
    scoped_guard(raw_spinlock, &irq_resend_lock) {
    if (hlist_unhashed(&desc.resend_node)) {
    hlist_add_head(&desc.resend_node, &irq_resend_list);
    }
    }
    tasklet_schedule(&resend_tasklet);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn clear_irq_resend(desc: *mut irq_desc) {
    guard(raw_spinlock)(&irq_resend_lock);
    hlist_del_init(&desc.resend_node);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_resend_init(desc: *mut irq_desc) {
    INIT_HLIST_NODE(&desc.resend_node);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: clear_irq_resend
pub unsafe extern "C" fn clear_irq_resend_dup(desc: *mut irq_desc) {}
#[no_mangle]
#[no_mangle]
// duplicate fn: irq_resend_init
pub unsafe extern "C" fn irq_resend_init_dup(desc: *mut irq_desc) {}
#[no_mangle]
unsafe extern "C" fn irq_sw_resend(desc: *mut irq_desc) -> c_int {
    return -EINVAL;
    }

#[no_mangle]
unsafe extern "C" fn try_retrigger(desc: *mut irq_desc) -> c_int {
    if (desc.irq_data.chip.irq_retrigger) {
    return desc.irq_data.chip.irq_retrigger(&desc.irq_data);
    }

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
pub static mut err: c_int = 0;
//
// We do not resend level type interrupts. Level type interrupts
// are resent by hardware when they are still active. Clear the
// pending bit so suspend/resume does not get confused.
//
    if (irq_settings_is_level(desc)) {
    desc.istate &= ~IRQS_PENDING;
    return -EINVAL;
    }
    if (desc.istate & IRQS_REPLAY) {
    return -EBUSY;
    }
    if (!(desc.istate & IRQS_PENDING) && !inject) {
    return 0;
    }
    desc.istate &= ~IRQS_PENDING;
    if (!try_retrigger(desc)) {
    err = irq_sw_resend(desc);
    }
// If the retrigger was successful, mark it with the REPLAY bit
    if (!err) {
    desc.istate |= IRQS_REPLAY;
    }
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
pub static mut err: c_int = 0;
// Try the state injection hardware interface first
    if (!irq_set_irqchip_state(irq, IRQCHIP_STATE_PENDING, true)) {
    return 0;
    }
// That failed, try via the resend mechanism
    scoped_irqdesc_get_and_buslock(irq, 0) {
    let mut desc = scoped_irqdesc;
//
// Only try to inject when the interrupt is:
// - not NMI type
// - activated
//
    if (!irq_is_nmi(desc) && irqd_is_activated(&desc.irq_data)) {
    err = check_irq_resend(desc, true);
    }
    }
    return err;
    }
    EXPORT_SYMBOL_GPL(irq_inject_interrupt);