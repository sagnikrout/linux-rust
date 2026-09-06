//! Automatically rewritten from C to Rust
//! Source: kernel/irq/autoprobe.c
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
// Copyright (C) 1992, 1998-2004 Linus Torvalds, Ingo Molnar
//
// This file contains the interrupt probing code and driver APIs.
//

//
// Autodetection depends on the fact that any interrupt that
// comes in on to an unassigned handler will get stuck with
// "IRQS_WAITING" cleared and the interrupt disabled.
//
pub static mut probing_active: usize = 0;
//
// probe_irq_on	- begin an interrupt autodetect
//
// Commence probing for an interrupt. The interrupts are scanned
// and a mask of potential interrupt lines is returned.
//
#[no_mangle]
pub unsafe extern "C" fn probe_irq_on() -> c_ulong {
pub static mut desc: *mut c_void = core::ptr::null_mut();
pub static mut mask: c_ulong = 0;
    let mut i = 0;
//
// quiesce the kernel, or at least the asynchronous portion
//
    async_synchronize_full();
    mutex_lock(&probing_active);
//
// something may have generated an irq long ago and we want to
// flush such a longstanding irq before considering it as spurious.
//
    for_each_irq_desc_reverse(i, desc) {
    guard(raw_spinlock_irq)(&desc.lock);
    if (!desc.action && irq_settings_can_probe(desc)) {
//
// Some chips need to know about probing in
// progress:
//
    if (desc.irq_data.chip.irq_set_type) {
    desc.irq_data.chip.irq_set_type(&desc.irq_data, IRQ_TYPE_PROBE);
    }
    irq_activate_and_startup(desc, IRQ_NORESEND);
    }
    }
// Wait for longstanding interrupts to trigger.
    msleep(20);
//
// enable any unassigned irqs
// (we must startup again here because if a longstanding irq
// happened in the previous stage, it may have masked itself)
//
    for_each_irq_desc_reverse(i, desc) {
    guard(raw_spinlock_irq)(&desc.lock);
    if (!desc.action && irq_settings_can_probe(desc)) {
    desc.istate |= IRQS_AUTODETECT | IRQS_WAITING;
    if (irq_activate_and_startup(desc, IRQ_NORESEND)) {
    desc.istate |= IRQS_PENDING;
    }
    }
    }
//
// Wait for spurious interrupts to trigger
//
    msleep(100);
//
// Now filter out any obviously spurious interrupts
//
    for_each_irq_desc(i, desc) {
    guard(raw_spinlock_irq)(&desc.lock);
    if (desc.istate & IRQS_AUTODETECT) {
// It triggered already - consider it spurious.
    if (!(desc.istate & IRQS_WAITING)) {
    desc.istate &= ~IRQS_AUTODETECT;
    irq_shutdown_and_deactivate(desc);
    } else if (i < 32) {
    mask |= 1 << i;
    }
    }
    }
    return mask;
    }
    EXPORT_SYMBOL(probe_irq_on);
//
// probe_irq_mask - scan a bitmap of interrupt lines
// @val:	mask of interrupts to consider
//
// Scan the interrupt lines and return a bitmap of active
// autodetect interrupts. The interrupt probe logic state
// is then returned to its previous value.
//
// Note: we need to scan all the irq's even though we will
// only return autodetect irq numbers - just so that we reset
// them all to a known state.
//
#[no_mangle]
pub unsafe extern "C" fn probe_irq_mask(val: c_ulong) -> c_uint {
pub static mut mask: c_uint = 0;
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    for_each_irq_desc(i, desc) {
    guard(raw_spinlock_irq)(&desc.lock);
    if (desc.istate & IRQS_AUTODETECT) {
    if (i < 16 && !(desc.istate & IRQS_WAITING)) {
    mask |= 1 << i;
    }
    desc.istate &= ~IRQS_AUTODETECT;
    irq_shutdown_and_deactivate(desc);
    }
    }
    mutex_unlock(&probing_active);
    return mask & val;
    }
    EXPORT_SYMBOL(probe_irq_mask);
//
// probe_irq_off	- end an interrupt autodetect
// @val: mask of potential interrupts (unused)
//
// Scans the unused interrupt lines and returns the line which
// appears to have triggered the interrupt. If no interrupt was
// found then zero is returned. If more than one interrupt is
// found then minus the first candidate is returned to indicate
// their is doubt.
//
// The interrupt probe logic state is returned to its previous
// value.
//
// BUGS: When used in a module (which arguably shouldn't happen)
// nothing prevents two IRQ probe callers from overlapping. The
// results of this are non-optimal.
//
#[no_mangle]
pub unsafe extern "C" fn probe_irq_off(val: c_ulong) -> c_int {
    int i, irq_found = 0, nr_of_irqs = 0;
pub static mut desc: *mut c_void = core::ptr::null_mut();
    for_each_irq_desc(i, desc) {
    guard(raw_spinlock_irq)(&desc.lock);
    if (desc.istate & IRQS_AUTODETECT) {
    if (!(desc.istate & IRQS_WAITING)) {
    if (!nr_of_irqs) {
    irq_found = i;
    }
    nr_of_irqs += 1;
    }
    desc.istate &= ~IRQS_AUTODETECT;
    irq_shutdown_and_deactivate(desc);
    }
    }
    mutex_unlock(&probing_active);
    if (nr_of_irqs > 1) {
    irq_found = -irq_found;
    }
    return irq_found;
    }
    EXPORT_SYMBOL(probe_irq_off);