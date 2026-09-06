//! Automatically rewritten from C to Rust
//! Source: kernel/irq/chip.c
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
// Copyright (C) 2005-2006, Thomas Gleixner, Russell King
//
// This file contains the core interrupt handling code, for irq-chip based
// architectures. Detailed information is available in
// Documentation/core-api/genericirq.rst
//

#[no_mangle]
unsafe extern "C" fn bad_chained_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    WARN_ONCE(1, "Chained irq %d should not call an action\n", irq);
    return IRQ_NONE;
    }
//
// Chained handlers should never call action on their IRQ. This default
// action will emit warning if such thing happens.
//
pub static mut irqaction: usize = 0;
//
// irq_set_chip - set the irq chip for an irq
// @irq:	irq number
// @chip:	pointer to irq chip description structure
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_chip(irq: c_uint, chip: *const irq_chip) -> c_int {
pub static mut ret: c_int = 0;
    scoped_irqdesc_get_and_lock(irq, 0) {
    scoped_irqdesc.irq_data.chip = (chip ?: &no_irq_chip);
    ret = 0;
    }
    if (!ret) {
// For !CONFIG_SPARSE_IRQ make the irq show up in allocated_irqs.
    irq_mark_irq(irq);
    irq_proc_update_chip(chip);
    }
    return ret;
    }
    EXPORT_SYMBOL(irq_set_chip);
//
// irq_set_irq_type - set the irq trigger type for an irq
// @irq:	irq number
// @type:	IRQ_TYPE_{LEVEL,EDGE}_* value - see include/linux/irq.h
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_irq_type(irq: c_uint, type: c_uint) -> c_int {
    scoped_irqdesc_get_and_buslock(irq, IRQ_GET_DESC_CHECK_GLOBAL)
    return __irq_set_trigger(scoped_irqdesc, type);
    return -EINVAL;
    }
    EXPORT_SYMBOL(irq_set_irq_type);
//
// irq_set_handler_data - set irq handler data for an irq
// @irq:	Interrupt number
// @data:	Pointer to interrupt specific data
//
// Set the hardware irq controller data for an irq
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_handler_data(irq: c_uint, data: *mut c_void) -> c_int {
    scoped_irqdesc_get_and_lock(irq, 0) {
    scoped_irqdesc.irq_common_data.handler_data = data;
    return 0;
    }
    return -EINVAL;
    }
    EXPORT_SYMBOL(irq_set_handler_data);
//
// irq_set_msi_desc_off - set MSI descriptor data for an irq at offset
// @irq_base:	Interrupt number base
// @irq_offset:	Interrupt number offset
// @entry:		Pointer to MSI descriptor data
//
// Set the MSI descriptor entry for an irq at offset
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_msi_desc_off(irq_base: c_uint, irq_offset: c_uint, entry: *mut msi_desc) -> c_int {
    scoped_irqdesc_get_and_lock(irq_base + irq_offset, IRQ_GET_DESC_CHECK_GLOBAL) {
    scoped_irqdesc.irq_common_data.msi_desc = entry;
    if (entry && !irq_offset) {
    entry.irq = irq_base;
    }
    return 0;
    }
    return -EINVAL;
    }
//
// irq_set_msi_desc - set MSI descriptor data for an irq
// @irq:	Interrupt number
// @entry:	Pointer to MSI descriptor data
//
// Set the MSI descriptor entry for an irq
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_msi_desc(irq: c_uint, entry: *mut msi_desc) -> c_int {
    return irq_set_msi_desc_off(irq, 0, entry);
    }
//
// irq_set_chip_data - set irq chip data for an irq
// @irq:	Interrupt number
// @data:	Pointer to chip specific data
//
// Set the hardware irq chip data for an irq
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_chip_data(irq: c_uint, data: *mut c_void) -> c_int {
    scoped_irqdesc_get_and_lock(irq, 0) {
    scoped_irqdesc.irq_data.chip_data = data;
    return 0;
    }
    return -EINVAL;
    }
    EXPORT_SYMBOL(irq_set_chip_data);
#[no_mangle]
pub unsafe extern "C" fn irq_get_irq_data(irq: c_uint) -> *mut c_void {
    let mut desc = irq_to_desc(irq);
    return desc ? &desc.irq_data : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(irq_get_irq_data);
#[no_mangle]
unsafe extern "C" fn irq_state_clr_disabled(desc: *mut irq_desc) {
    irqd_clear(&desc.irq_data, IRQD_IRQ_DISABLED);
    }
#[no_mangle]
unsafe extern "C" fn irq_state_clr_masked(desc: *mut irq_desc) {
    irqd_clear(&desc.irq_data, IRQD_IRQ_MASKED);
    }
#[no_mangle]
unsafe extern "C" fn irq_state_clr_started(desc: *mut irq_desc) {
    irqd_clear(&desc.irq_data, IRQD_IRQ_STARTED);
    }
#[no_mangle]
unsafe extern "C" fn irq_state_set_started(desc: *mut irq_desc) {
    irqd_set(&desc.irq_data, IRQD_IRQ_STARTED);
    }
    enum {
    IRQ_STARTUP_NORMAL,
    IRQ_STARTUP_MANAGED,
    IRQ_STARTUP_ABORT,
    };

#[no_mangle]
pub unsafe extern "C" fn __irq_startup_managed(desc: *mut irq_desc, aff: *mut cpumask, force: bool) -> c_int {
    let mut d = irq_desc_get_irq_data(desc);
    if (!irqd_affinity_is_managed(d)) {
    return IRQ_STARTUP_NORMAL;
    }
    irqd_clr_managed_shutdown(d);
    if (!cpumask_intersects(aff, cpu_online_mask)) {
//
// Catch code which fiddles with enable_irq() on a managed
// and potentially shutdown IRQ. Chained interrupt
// installment or irq auto probing should not happen on
// managed irqs either.
//
    if (WARN_ON_ONCE!(force)) {
    return IRQ_STARTUP_ABORT;
    }
//
// The interrupt was requested, but there is no online CPU
// in it's affinity mask. Put it into managed shutdown
// state and let the cpu hotplug mechanism start it up once
// a CPU in the mask becomes available.
//
    return IRQ_STARTUP_ABORT;
    }
//
// Managed interrupts have reserved resources, so this should not
// happen.
//
    if (WARN_ON!(irq_domain_activate_irq(d, false))) {
    return IRQ_STARTUP_ABORT;
    }
    return IRQ_STARTUP_MANAGED;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_startup_managed(desc: *mut irq_desc) {
    let mut d = irq_desc_get_irq_data(desc);
//
// Clear managed-shutdown flag, so we don't repeat managed-startup for
// multiple hotplugs, and cause imbalanced disable depth.
//
    irqd_clr_managed_shutdown(d);
//
// Only start it up when the disable depth is 1, so that a disable,
// hotunplug, hotplug sequence does not end up enabling it during
// hotplug unconditionally.
//
    desc.depth -= 1;
    if (!desc.depth) {
    irq_startup(desc, IRQ_RESEND, IRQ_START_COND);
    }
    }

    static __always_inline int
    __irq_startup_managed(irq_desc *desc, const struct cpumask *aff,
    bool force)
    {
    return IRQ_STARTUP_NORMAL;
    }

#[no_mangle]
unsafe extern "C" fn irq_enable(desc: *mut irq_desc) {
    if (!irqd_irq_disabled(&desc.irq_data)) {
    unmask_irq(desc);
    } else {
    irq_state_clr_disabled(desc);
    if (desc.irq_data.chip.irq_enable) {
    desc.irq_data.chip.irq_enable(&desc.irq_data);
    irq_state_clr_masked(desc);
    } else {
    unmask_irq(desc);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn __irq_startup(desc: *mut irq_desc) -> c_int {
    let mut d = irq_desc_get_irq_data(desc);
pub static mut ret: c_int = 0;
// Warn if this interrupt is not activated but try nevertheless
    WARN_ON_ONCE!(!irqd_is_activated(d));
    if (d.chip.irq_startup) {
    ret = d.chip.irq_startup(d);
    irq_state_clr_disabled(desc);
    irq_state_clr_masked(desc);
    } else {
    irq_enable(desc);
    }
    irq_state_set_started(desc);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_startup(desc: *mut irq_desc, resend: bool, force: bool) -> c_int {
    let mut d = irq_desc_get_irq_data(desc);
    let mut aff = irq_data_get_affinity_mask(d);
pub static mut ret: c_int = 0;
    desc.depth = 0;
    if (irqd_is_started(d)) {
    irq_enable(desc);
    } else {
    switch (__irq_startup_managed(desc, aff, force)) {
    case IRQ_STARTUP_NORMAL:
    if (d.chip.flags & IRQCHIP_AFFINITY_PRE_STARTUP) {
    irq_setup_affinity(desc);
    }
    ret = __irq_startup(desc);
    if (!(d.chip.flags & IRQCHIP_AFFINITY_PRE_STARTUP)) {
    irq_setup_affinity(desc);
    }
    break;
    case IRQ_STARTUP_MANAGED:
    irq_do_set_affinity(d, aff, false);
    ret = __irq_startup(desc);
    break;
    case IRQ_STARTUP_ABORT:
    desc.depth = 1;
    irqd_set_managed_shutdown(d);
    return 0;
    }
    }
    if (resend) {
    check_irq_resend(desc, false);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_activate(desc: *mut irq_desc) -> c_int {
    let mut d = irq_desc_get_irq_data(desc);
    if (!irqd_affinity_is_managed(d)) {
    return irq_domain_activate_irq(d, false);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_activate_and_startup(desc: *mut irq_desc, resend: bool) -> c_int {
    if (WARN_ON!(irq_activate(desc))) {
    return 0;
    }
    return irq_startup(desc, resend, IRQ_START_FORCE);
    }
// forward_decl: __irq_disable;
#[no_mangle]
pub unsafe extern "C" fn irq_shutdown(desc: *mut irq_desc) {
    if (irqd_is_started(&desc.irq_data)) {
    clear_irq_resend(desc);
//
// Increment disable depth, so that a managed shutdown on
// CPU hotunplug preserves the actual disabled state when the
// CPU comes back online. See irq_startup_managed().
//
    desc.depth += 1;
    if (desc.irq_data.chip.irq_shutdown) {
    desc.irq_data.chip.irq_shutdown(&desc.irq_data);
    irq_state_set_disabled(desc);
    irq_state_set_masked(desc);
    } else {
    __irq_disable(desc, true);
    }
    irq_state_clr_started(desc);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irq_shutdown_and_deactivate(desc: *mut irq_desc) {
    irq_shutdown(desc);
//
// This must be called even if the interrupt was never started up,
// because the activation can happen before the interrupt is
// available for request/startup. It has it's own state tracking so
// it's safe to call it unconditionally.
//
    irq_domain_deactivate_irq(&desc.irq_data);
    }
#[no_mangle]
unsafe extern "C" fn __irq_disable(desc: *mut irq_desc, mask: bool) {
    if (irqd_irq_disabled(&desc.irq_data)) {
    if (mask) {
    mask_irq(desc);
    }
    } else {
    irq_state_set_disabled(desc);
    if (desc.irq_data.chip.irq_disable) {
    desc.irq_data.chip.irq_disable(&desc.irq_data);
    irq_state_set_masked(desc);
    } else if (mask) {
    mask_irq(desc);
    }
    }
    }
//
// irq_disable - Mark interrupt disabled
// @desc:	irq descriptor which should be disabled
//
// If the chip does not implement the irq_disable callback, we
// use a lazy disable approach. That means we mark the interrupt
// disabled, but leave the hardware unmasked. That's an
// optimization because we avoid the hardware access for the
// common case where no interrupt happens after we marked it
// disabled. If an interrupt happens, then the interrupt flow
// handler masks the line at the hardware level and marks it
// pending.
//
// If the interrupt chip does not implement the irq_disable callback,
// a driver can disable the lazy approach for a particular irq line by
// calling 'irq_set_status_flags(irq, IRQ_DISABLE_UNLAZY)'. This can
// be used for devices which cannot disable the interrupt at the
// device level under certain circumstances and have to use
// disable_irq[_nosync] instead.
//
#[no_mangle]
pub unsafe extern "C" fn irq_disable(desc: *mut irq_desc) {
    __irq_disable(desc, irq_settings_disable_unlazy(desc));
    }
#[no_mangle]
pub unsafe extern "C" fn irq_percpu_enable(desc: *mut irq_desc, cpu: c_uint) {
    if (desc.irq_data.chip.irq_enable) {
    desc.irq_data.chip.irq_enable(&desc.irq_data);
    }
    else {
    desc.irq_data.chip.irq_unmask(&desc.irq_data);
    }
    cpumask_set_cpu(cpu, desc.percpu_enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_percpu_disable(desc: *mut irq_desc, cpu: c_uint) {
    if (desc.irq_data.chip.irq_disable) {
    desc.irq_data.chip.irq_disable(&desc.irq_data);
    }
    else {
    desc.irq_data.chip.irq_mask(&desc.irq_data);
    }
    cpumask_clear_cpu(cpu, desc.percpu_enabled);
    }
#[no_mangle]
pub unsafe extern "C" fn mask_ack_irq(desc: *mut irq_desc) {
    if (desc.irq_data.chip.irq_mask_ack) {
    desc.irq_data.chip.irq_mask_ack(&desc.irq_data);
    irq_state_set_masked(desc);
    } else {
    mask_irq(desc);
    if (desc.irq_data.chip.irq_ack) {
    desc.irq_data.chip.irq_ack(&desc.irq_data);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mask_irq(desc: *mut irq_desc) {
    if (irqd_irq_masked(&desc.irq_data)) {
    return;
    }
    if (desc.irq_data.chip.irq_mask) {
    desc.irq_data.chip.irq_mask(&desc.irq_data);
    irq_state_set_masked(desc);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn unmask_irq(desc: *mut irq_desc) {
    if (!irqd_irq_masked(&desc.irq_data)) {
    return;
    }
    if (desc.irq_data.chip.irq_unmask) {
    desc.irq_data.chip.irq_unmask(&desc.irq_data);
    irq_state_clr_masked(desc);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn unmask_threaded_irq(desc: *mut irq_desc) {
    let mut chip = desc.irq_data.chip;
    if (chip.flags & IRQCHIP_EOI_THREADED) {
    chip.irq_eoi(&desc.irq_data);
    }
    unmask_irq(desc);
    }
// Busy wait until INPROGRESS is cleared
#[no_mangle]
unsafe extern "C" fn irq_wait_on_inprogress(desc: *mut irq_desc) -> bool {
    if (IS_ENABLED!(CONFIG_SMP)) {
    do {
    raw_spin_unlock(&desc.lock);
    while (irqd_irq_inprogress(&desc.irq_data)) {
    cpu_relax();
    }
    raw_spin_lock(&desc.lock);
    } while (irqd_irq_inprogress(&desc.irq_data));
// Might have been disabled in meantime
    return !irqd_irq_disabled(&desc.irq_data) && desc.action;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn irq_can_handle_pm(desc: *mut irq_desc) -> bool {
    let mut irqd = &desc.irq_data;
pub static mut aff: *mut c_void = core::ptr::null_mut();
//
// If the interrupt is not in progress and is not an armed
// wakeup interrupt, proceed.
//
    if (!irqd_has_set(irqd, IRQD_IRQ_INPROGRESS | IRQD_WAKEUP_ARMED)) {
    return true;
    }
//
// If the interrupt is an armed wakeup source, mark it pending
// and suspended, disable it and notify the pm core about the
// event.
//
    if (unlikely(irqd_has_set(irqd, IRQD_WAKEUP_ARMED))) {
    irq_pm_handle_wakeup(desc);
    return false;
    }
// Check whether the interrupt is polled on another CPU
    if (unlikely(desc.istate & IRQS_POLL_INPROGRESS)) {
    if (WARN_ONCE(irq_poll_cpu == smp_processor_id(),
    "irq poll in progress on cpu %d for irq %d\n",
    smp_processor_id(), desc.irq_data.irq)) {
    return false;
    }
    return irq_wait_on_inprogress(desc);
    }
// The below works only for single target interrupts
    if (!IS_ENABLED!(CONFIG_GENERIC_IRQ_EFFECTIVE_AFF_MASK) ||
    !irqd_is_single_target(irqd) || desc.handle_irq != handle_edge_irq) {
    return false;
    }
//
// If the interrupt affinity was moved to this CPU and the
// interrupt is currently handled on the previous target CPU, then
// busy wait for INPROGRESS to be cleared. Otherwise for edge type
// interrupts the handler might get stuck on the previous target:
//
// CPU 0			CPU 1 (new target)
// handle_edge_irq()
// repeat:
// handle_event()		handle_edge_irq()
// if (INPROGESS) {
// set(PENDING);
// mask();
// return;
// }
// if (PENDING) {
// clear(PENDING);
// unmask();
// goto repeat;
// }
//
// This happens when the device raises interrupts with a high rate
// and always before handle_event() completes and the CPU0 handler
// can clear INPROGRESS. This has been observed in virtual machines.
//
    aff = irq_data_get_effective_affinity_mask(irqd);
    if (cpumask_first(aff) != smp_processor_id()) {
    return false;
    }
    return irq_wait_on_inprogress(desc);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_can_handle_actions(desc: *mut irq_desc) -> bool {
    desc.istate &= ~(IRQS_REPLAY | IRQS_WAITING);
    if (unlikely(!desc.action || irqd_irq_disabled(&desc.irq_data))) {
    desc.istate |= IRQS_PENDING;
    return false;
    }
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_can_handle(desc: *mut irq_desc) -> bool {
    if (!irq_can_handle_pm(desc)) {
    return false;
    }
    return irq_can_handle_actions(desc);
    }
//
// handle_nested_irq - Handle a nested irq from a irq thread
// @irq:	the interrupt number
//
// Handle interrupts which are nested into a threaded interrupt
// handler. The handler function is called inside the calling threads
// context.
//
#[no_mangle]
pub unsafe extern "C" fn handle_nested_irq(irq: c_uint) {
    let mut desc = irq_to_desc(irq);
pub static mut action: *mut c_void = core::ptr::null_mut();
    let mut action_ret;
    might_sleep();
    scoped_guard(raw_spinlock_irq, &desc.lock) {
    if (!irq_can_handle_actions(desc)) {
    return;
    }
    action = desc.action;
    kstat_incr_irqs_this_cpu(desc);
    atomic_inc(&desc.threads_active);
    }
    action_ret = IRQ_NONE;
    for_each_action_of_desc(desc, action) {
    action_ret |= action.thread_fn(action.irq, action.dev_id);
    }
    if (!irq_settings_no_debug(desc)) {
    note_interrupt(desc, action_ret);
    }
    wake_threads_waitq(desc);
    }
    EXPORT_SYMBOL_GPL(handle_nested_irq);
//
// handle_simple_irq - Simple and software-decoded IRQs.
// @desc:	the interrupt description structure for this irq
//
// Simple interrupts are either sent from a demultiplexing interrupt
// handler or come from hardware, where no interrupt hardware control is
// necessary.
//
// Note: The caller is expected to handle the ack, clear, mask and unmask
// issues if necessary.
//
#[no_mangle]
pub unsafe extern "C" fn handle_simple_irq(desc: *mut irq_desc) {
    guard(raw_spinlock)(&desc.lock);
    if (!irq_can_handle_pm(desc)) {
    if (irqd_needs_resend_when_in_progress(&desc.irq_data)) {
    desc.istate |= IRQS_PENDING;
    }
    return;
    }
    if (!irq_can_handle_actions(desc)) {
    return;
    }
    kstat_incr_irqs_this_cpu(desc);
    handle_irq_event(desc);
    }
    EXPORT_SYMBOL_GPL(handle_simple_irq);
//
// handle_untracked_irq - Simple and software-decoded IRQs.
// @desc:	the interrupt description structure for this irq
//
// Untracked interrupts are sent from a demultiplexing interrupt handler
// when the demultiplexer does not know which device it its multiplexed irq
// domain generated the interrupt. IRQ's handled through here are not
// subjected to stats tracking, randomness, or spurious interrupt
// detection.
//
// Note: Like handle_simple_irq, the caller is expected to handle the ack,
// clear, mask and unmask issues if necessary.
//
#[no_mangle]
pub unsafe extern "C" fn handle_untracked_irq(desc: *mut irq_desc) {
    scoped_guard(raw_spinlock, &desc.lock) {
    if (!irq_can_handle(desc)) {
    return;
    }
    desc.istate &= ~IRQS_PENDING;
    irqd_set(&desc.irq_data, IRQD_IRQ_INPROGRESS);
    }
    __handle_irq_event_percpu(desc);
    scoped_guard(raw_spinlock, &desc.lock)
    irqd_clear(&desc.irq_data, IRQD_IRQ_INPROGRESS);
    }
    EXPORT_SYMBOL_GPL(handle_untracked_irq);
//
// Called unconditionally from handle_level_irq() and only for oneshot
// interrupts from handle_fasteoi_irq()
//
#[no_mangle]
unsafe extern "C" fn cond_unmask_irq(desc: *mut irq_desc) {
//
// We need to unmask in the following cases:
// - Standard level irq (IRQF_ONESHOT is not set)
// - Oneshot irq which did not wake the thread (caused by a
// spurious interrupt or a primary handler handling it
// completely).
//
    if (!irqd_irq_disabled(&desc.irq_data) &&
    irqd_irq_masked(&desc.irq_data) && !desc.threads_oneshot) {
    unmask_irq(desc);
    }
    }
//
// handle_level_irq - Level type irq handler
// @desc:	the interrupt description structure for this irq
//
// Level type interrupts are active as long as the hardware line has the
// active level. This may require to mask the interrupt and unmask it after
// the associated handler has acknowledged the device, so the interrupt
// line is back to inactive.
//
#[no_mangle]
pub unsafe extern "C" fn handle_level_irq(desc: *mut irq_desc) {
    guard(raw_spinlock)(&desc.lock);
    mask_ack_irq(desc);
    if (!irq_can_handle(desc)) {
    return;
    }
    kstat_incr_irqs_this_cpu(desc);
    handle_irq_event(desc);
    cond_unmask_irq(desc);
    }
    EXPORT_SYMBOL_GPL(handle_level_irq);
#[no_mangle]
unsafe extern "C" fn cond_unmask_eoi_irq(desc: *mut irq_desc, chip: *mut irq_chip) {
    if (!(desc.istate & IRQS_ONESHOT)) {
    chip.irq_eoi(&desc.irq_data);
    return;
    }
//
// We need to unmask in the following cases:
// - Oneshot irq which did not wake the thread (caused by a
// spurious interrupt or a primary handler handling it
// completely).
//
    if (!irqd_irq_disabled(&desc.irq_data) &&
    irqd_irq_masked(&desc.irq_data) && !desc.threads_oneshot) {
    chip.irq_eoi(&desc.irq_data);
    unmask_irq(desc);
    } else if (!(chip.flags & IRQCHIP_EOI_THREADED)) {
    chip.irq_eoi(&desc.irq_data);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn cond_eoi_irq(chip: *mut irq_chip, data: *mut irq_data) {
    if (!(chip.flags & IRQCHIP_EOI_IF_HANDLED)) {
    chip.irq_eoi(data);
    }
    }
//
// handle_fasteoi_irq - irq handler for transparent controllers
// @desc:	the interrupt description structure for this irq
//
// Only a single callback will be issued to the chip: an ->eoi() call when
// the interrupt has been serviced. This enables support for modern forms
// of interrupt handlers, which handle the flow details in hardware,
// transparently.
//
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_irq(desc: *mut irq_desc) {
    let mut chip = desc.irq_data.chip;
    guard(raw_spinlock)(&desc.lock);
//
// When an affinity change races with IRQ handling, the next interrupt
// can arrive on the new CPU before the original CPU has completed
// handling the previous one - it may need to be resent.
//
    if (!irq_can_handle_pm(desc)) {
    if (irqd_needs_resend_when_in_progress(&desc.irq_data)) {
    desc.istate |= IRQS_PENDING;
    }
    cond_eoi_irq(chip, &desc.irq_data);
    return;
    }
    if (!irq_can_handle_actions(desc)) {
    mask_irq(desc);
    cond_eoi_irq(chip, &desc.irq_data);
    return;
    }
    kstat_incr_irqs_this_cpu(desc);
    if (desc.istate & IRQS_ONESHOT) {
    mask_irq(desc);
    }
    handle_irq_event(desc);
    cond_unmask_eoi_irq(desc, chip);
//
// When the race described above happens this will resend the interrupt.
//
    if (unlikely(desc.istate & IRQS_PENDING)) {
    check_irq_resend(desc, false);
    }
    }
    EXPORT_SYMBOL_GPL(handle_fasteoi_irq);
//
// handle_fasteoi_nmi - irq handler for NMI interrupt lines
// @desc:	the interrupt description structure for this irq
//
// A simple NMI-safe handler, considering the restrictions
// from request_nmi.
//
// Only a single callback will be issued to the chip: an ->eoi()
// call when the interrupt has been serviced. This enables support
// for modern forms of interrupt handlers, which handle the flow
// details in hardware, transparently.
//
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_nmi(desc: *mut irq_desc) {
    let mut chip = irq_desc_get_chip(desc);
    let mut action = desc.action;
pub static mut irq: c_uint = 0;
    let mut res;
    __kstat_incr_irqs_this_cpu(desc);
    trace_irq_handler_entry(irq, action);
//
// NMIs cannot be shared, there is only one action.
//
    res = action.handler(irq, action.dev_id);
    trace_irq_handler_exit(irq, action, res);
    if (chip.irq_eoi) {
    chip.irq_eoi(&desc.irq_data);
    }
    }
    EXPORT_SYMBOL_GPL(handle_fasteoi_nmi);
//
// handle_edge_irq - edge type IRQ handler
// @desc:	the interrupt description structure for this irq
//
// Interrupt occurs on the falling and/or rising edge of a hardware
// signal. The occurrence is latched into the irq controller hardware and
// must be acked in order to be reenabled. After the ack another interrupt
// can happen on the same source even before the first one is handled by
// the associated event handler. If this happens it might be necessary to
// disable (mask) the interrupt depending on the controller hardware. This
// requires to reenable the interrupt inside of the loop which handles the
// interrupts which have arrived while the handler was running. If all
// pending interrupts are handled, the loop is left.
//
#[no_mangle]
pub unsafe extern "C" fn handle_edge_irq(desc: *mut irq_desc) {
    guard(raw_spinlock)(&desc.lock);
    if (!irq_can_handle(desc)) {
    desc.istate |= IRQS_PENDING;
    mask_ack_irq(desc);
    return;
    }
    kstat_incr_irqs_this_cpu(desc);
// Start handling the irq
    desc.irq_data.chip.irq_ack(&desc.irq_data);
    do {
    if (unlikely(!desc.action)) {
    mask_irq(desc);
    return;
    }
//
// When another irq arrived while we were handling
// one, we could have masked the irq.
// Reenable it, if it was not disabled in meantime.
//
    if (unlikely(desc.istate & IRQS_PENDING)) {
    if (!irqd_irq_disabled(&desc.irq_data) &&
    irqd_irq_masked(&desc.irq_data)) {
    unmask_irq(desc);
    }
    }
    handle_irq_event(desc);
    } while ((desc.istate & IRQS_PENDING) && !irqd_irq_disabled(&desc.irq_data));
    }
    EXPORT_SYMBOL(handle_edge_irq);
//
// handle_percpu_irq - Per CPU local irq handler
// @desc:	the interrupt description structure for this irq
//
// Per CPU interrupts on SMP machines without locking requirements
//
#[no_mangle]
pub unsafe extern "C" fn handle_percpu_irq(desc: *mut irq_desc) {
    let mut chip = irq_desc_get_chip(desc);
//
// PER CPU interrupts are not serialized. Do not touch
// desc->tot_count.
//
    __kstat_incr_irqs_this_cpu(desc);
    if (chip.irq_ack) {
    chip.irq_ack(&desc.irq_data);
    }
    handle_irq_event_percpu(desc);
    if (chip.irq_eoi) {
    chip.irq_eoi(&desc.irq_data);
    }
    }
//
// handle_percpu_devid_irq - Per CPU local irq handler with per cpu dev ids
// @desc:	the interrupt description structure for this irq
//
// Per CPU interrupts on SMP machines without locking requirements. Same as
// handle_percpu_irq() above but with the following extras:
//
// action->percpu_dev_id is a pointer to percpu variables which
// contain the real device id for the cpu on which this handler is
// called.
//
// May be used for NMI interrupt lines, and so may be called in IRQ or NMI
// context.
//
#[no_mangle]
pub unsafe extern "C" fn handle_percpu_devid_irq(desc: *mut irq_desc) {
    let mut chip = irq_desc_get_chip(desc);
pub static mut irq: c_uint = 0;
pub static mut cpu: c_uint = 0;
pub static mut action: *mut c_void = core::ptr::null_mut();
    let mut res;
//
// PER CPU interrupts are not serialized. Do not touch
// desc->tot_count.
//
    __kstat_incr_irqs_this_cpu(desc);
    if (chip.irq_ack) {
    chip.irq_ack(&desc.irq_data);
    }
    for (action = desc.action; action; action = action.next) {
    if (cpumask_test_cpu(cpu, action.affinity))
    break;
    }
    if (likely(action)) {
    trace_irq_handler_entry(irq, action);
    res = action.handler(irq, raw_cpu_ptr(action.percpu_dev_id));
    trace_irq_handler_exit(irq, action, res);
    } else {
pub static mut enabled: bool = false;
    if (enabled) {
    irq_percpu_disable(desc, cpu);
    }
    pr_err_once("Spurious%s percpu IRQ%u on CPU%u\n",
    enabled ? " and unmasked" : "", irq, cpu);
    }
    if (!in_nmi()) {
    add_interrupt_randomness(irq);
    }
    if (chip.irq_eoi) {
    chip.irq_eoi(&desc.irq_data);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_do_set_handler(desc: *mut irq_desc, handle: irq_flow_handler_t, is_chained: c_int, name: *mut c_char) {
    if (!handle) {
    handle = handle_bad_irq;
    } else {
    let mut irq_data = &desc.irq_data;

//
// With hierarchical domains we might run into a
// situation where the outermost chip is not yet set
// up, but the inner chips are there.  Instead of
// bailing we install the handler, but obviously we
// cannot enable/startup the interrupt at this point.
//
    while (irq_data) {
    if (irq_data.chip != &no_irq_chip) {
    break;
    }
//
// Bail out if the outer chip is not set up
// and the interrupt supposed to be started
// right away.
//
    if (WARN_ON!(is_chained)) {
    return;
    }
// Try the parent
    irq_data = irq_data.parent_data;
    }

    if (WARN_ON!(!irq_data || irq_data.chip == &no_irq_chip)) {
    return;
    }
    }
// Uninstall?
    if (handle == handle_bad_irq) {
    if (desc.irq_data.chip != &no_irq_chip) {
    mask_ack_irq(desc);
    }
    irq_state_set_disabled(desc);
    if (is_chained) {
    desc.action = core::ptr::null_mut();
    irq_chip_pm_put(irq_desc_get_irq_data(desc));
    }
    desc.depth = 1;
    }
    desc.handle_irq = handle;
    desc.name = name;
    if (handle != handle_bad_irq && is_chained) {
pub static mut type: c_uint = 0;
//
// We're about to start this interrupt immediately,
// hence the need to set the trigger configuration.
// But the .set_type callback may have overridden the
// flow handler, ignoring that we're dealing with a
// chained interrupt. Reset it immediately because we
// do know better.
//
    if (type != IRQ_TYPE_NONE) {
    __irq_set_trigger(desc, type);
    desc.handle_irq = handle;
    }
    irq_settings_set_noprobe(desc);
    irq_settings_set_norequest(desc);
    irq_settings_set_nothread(desc);
    desc.action = &chained_action;
    WARN_ON!(irq_chip_pm_get(irq_desc_get_irq_data(desc)));
    irq_activate_and_startup(desc, IRQ_RESEND);
    }
    irq_proc_update_valid(desc);
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_set_handler(irq: c_uint, handle: irq_flow_handler_t, is_chained: c_int, name: *mut c_char) {
    scoped_irqdesc_get_and_buslock(irq, 0)
    __irq_do_set_handler(scoped_irqdesc, handle, is_chained, name);
    }
    EXPORT_SYMBOL_GPL(__irq_set_handler);
#[no_mangle]
pub unsafe extern "C" fn irq_set_chained_handler_and_data(irq: c_uint, handle: irq_flow_handler_t, data: *mut c_void) {
    scoped_irqdesc_get_and_buslock(irq, 0) {
    let mut desc = scoped_irqdesc;
    desc.irq_common_data.handler_data = data;
    __irq_do_set_handler(desc, handle, 1, core::ptr::null_mut());
    }
    }
    EXPORT_SYMBOL_GPL(irq_set_chained_handler_and_data);
#[no_mangle]
pub unsafe extern "C" fn irq_set_chip_and_handler_name(irq: c_uint, chip: *mut irq_chip, handle: irq_flow_handler_t, name: *mut c_char) {
    irq_set_chip(irq, chip);
    __irq_set_handler(irq, handle, 0, name);
    }
    EXPORT_SYMBOL_GPL(irq_set_chip_and_handler_name);
#[no_mangle]
pub unsafe extern "C" fn irq_modify_status(irq: c_uint, clr: c_ulong, set: c_ulong) {
    scoped_irqdesc_get_and_lock(irq, 0) {
    let mut desc = scoped_irqdesc;
    unsigned long trigger, tmp;
//
// Warn when a driver sets the no autoenable flag on an already
// active interrupt.
//
    WARN_ON_ONCE!(!desc.depth && (set & _IRQ_NOAUTOEN));
    irq_settings_clr_and_set(desc, clr, set);
    trigger = irqd_get_trigger_type(&desc.irq_data);
    irqd_clear(&desc.irq_data, IRQD_NO_BALANCING | IRQD_PER_CPU |
    IRQD_TRIGGER_MASK | IRQD_LEVEL);
    if (irq_settings_has_no_balance_set(desc)) {
    irqd_set(&desc.irq_data, IRQD_NO_BALANCING);
    }
    if (irq_settings_is_per_cpu(desc)) {
    irqd_set(&desc.irq_data, IRQD_PER_CPU);
    }
    if (irq_settings_is_level(desc)) {
    irqd_set(&desc.irq_data, IRQD_LEVEL);
    }
    tmp = irq_settings_get_trigger_mask(desc);
    if (tmp != IRQ_TYPE_NONE) {
    trigger = tmp;
    }
    irqd_set(&desc.irq_data, trigger);
    irq_proc_update_valid(desc);
    }
    }
    EXPORT_SYMBOL_GPL(irq_modify_status);

//
// irq_cpu_online - Invoke all irq_cpu_online functions.
//
// Iterate through all irqs and invoke the chip.irq_cpu_online()
// for each.
//
#[no_mangle]
pub unsafe extern "C" fn irq_cpu_online() {
    let mut irq = 0;
    for_each_active_irq(irq) {
    let mut desc = irq_to_desc(irq);
pub static mut chip: *mut c_void = core::ptr::null_mut();
    if (!desc) {
    continue;
    }
    guard(raw_spinlock_irqsave)(&desc.lock);
    chip = irq_data_get_irq_chip(&desc.irq_data);
    if (chip && chip.irq_cpu_online &&
    (!(chip.flags & IRQCHIP_ONOFFLINE_ENABLED) ||
    !irqd_irq_disabled(&desc.irq_data))) {
    chip.irq_cpu_online(&desc.irq_data);
    }
    }
    }
//
// irq_cpu_offline - Invoke all irq_cpu_offline functions.
//
// Iterate through all irqs and invoke the chip.irq_cpu_offline()
// for each.
//
#[no_mangle]
pub unsafe extern "C" fn irq_cpu_offline() {
    let mut irq = 0;
    for_each_active_irq(irq) {
    let mut desc = irq_to_desc(irq);
pub static mut chip: *mut c_void = core::ptr::null_mut();
    if (!desc) {
    continue;
    }
    guard(raw_spinlock_irqsave)(&desc.lock);
    chip = irq_data_get_irq_chip(&desc.irq_data);
    if (chip && chip.irq_cpu_offline &&
    (!(chip.flags & IRQCHIP_ONOFFLINE_ENABLED) ||
    !irqd_irq_disabled(&desc.irq_data))) {
    chip.irq_cpu_offline(&desc.irq_data);
    }
    }
    }

//
// handle_fasteoi_ack_irq - irq handler for edge hierarchy stacked on
// transparent controllers
//
// @desc:	the interrupt description structure for this irq
//
// Like handle_fasteoi_irq(), but for use with hierarchy where the irq_chip
// also needs to have its ->irq_ack() function called.
//
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_ack_irq(desc: *mut irq_desc) {
    let mut chip = desc.irq_data.chip;
    guard(raw_spinlock)(&desc.lock);
    if (!irq_can_handle_pm(desc)) {
    cond_eoi_irq(chip, &desc.irq_data);
    return;
    }
    if (unlikely(!irq_can_handle_actions(desc))) {
    mask_irq(desc);
    cond_eoi_irq(chip, &desc.irq_data);
    return;
    }
    kstat_incr_irqs_this_cpu(desc);
    if (desc.istate & IRQS_ONESHOT) {
    mask_irq(desc);
    }
    desc.irq_data.chip.irq_ack(&desc.irq_data);
    handle_irq_event(desc);
    cond_unmask_eoi_irq(desc, chip);
    }
    EXPORT_SYMBOL_GPL(handle_fasteoi_ack_irq);
//
// handle_fasteoi_mask_irq - irq handler for level hierarchy stacked on
// transparent controllers
//
// @desc:	the interrupt description structure for this irq
//
// Like handle_fasteoi_irq(), but for use with hierarchy where the irq_chip
// also needs to have its ->irq_mask_ack() function called.
//
#[no_mangle]
pub unsafe extern "C" fn handle_fasteoi_mask_irq(desc: *mut irq_desc) {
    let mut chip = desc.irq_data.chip;
    guard(raw_spinlock)(&desc.lock);
    mask_ack_irq(desc);
    if (!irq_can_handle(desc)) {
    cond_eoi_irq(chip, &desc.irq_data);
    return;
    }
    kstat_incr_irqs_this_cpu(desc);
    handle_irq_event(desc);
    cond_unmask_eoi_irq(desc, chip);
    }
    EXPORT_SYMBOL_GPL(handle_fasteoi_mask_irq);

#[no_mangle]
pub unsafe extern "C" fn irq_chip_pre_redirect_parent(data: *mut irq_data) {
    data = data.parent_data;
    data.chip.irq_pre_redirect(data);
    }
    EXPORT_SYMBOL_GPL(irq_chip_pre_redirect_parent);

//
// irq_chip_set_parent_state - set the state of a parent interrupt.
//
// @data: Pointer to interrupt specific data
// @which: State to be restored (one of IRQCHIP_STATE_*)
// @val: Value corresponding to @which
//
// Conditional success, if the underlying irqchip does not implement it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_parent_state(data: *mut irq_data, which: irqchip_irq_state, val: bool) -> c_int {
    data = data.parent_data;
    if (!data || !data.chip.irq_set_irqchip_state) {
    return 0;
    }
    return data.chip.irq_set_irqchip_state(data, which, val);
    }
    EXPORT_SYMBOL_GPL(irq_chip_set_parent_state);
//
// irq_chip_get_parent_state - get the state of a parent interrupt.
//
// @data: Pointer to interrupt specific data
// @which: one of IRQCHIP_STATE_* the caller wants to know
// @state: a pointer to a boolean where the state is to be stored
//
// Conditional success, if the underlying irqchip does not implement it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_get_parent_state(data: *mut irq_data, which: irqchip_irq_state, state: *mut bool) -> c_int {
    data = data.parent_data;
    if (!data || !data.chip.irq_get_irqchip_state) {
    return 0;
    }
    return data.chip.irq_get_irqchip_state(data, which, state);
    }
    EXPORT_SYMBOL_GPL(irq_chip_get_parent_state);
//
// irq_chip_shutdown_parent - Shutdown the parent interrupt
// @data:	Pointer to interrupt specific data
//
// Invokes the irq_shutdown() callback of the parent if available or falls
// back to irq_chip_disable_parent().
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_shutdown_parent(data: *mut irq_data) {
    let mut parent = data.parent_data;
    if (parent.chip.irq_shutdown) {
    parent.chip.irq_shutdown(parent);
    }
    else {
    irq_chip_disable_parent(data);
    }
    }
    EXPORT_SYMBOL_GPL(irq_chip_shutdown_parent);
//
// irq_chip_startup_parent - Startup the parent interrupt
// @data:	Pointer to interrupt specific data
//
// Invokes the irq_startup() callback of the parent if available or falls
// back to irq_chip_enable_parent().
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_startup_parent(data: *mut irq_data) -> c_uint {
    let mut parent = data.parent_data;
    if (parent.chip.irq_startup) {
    return parent.chip.irq_startup(parent);
    }
    irq_chip_enable_parent(data);
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_chip_startup_parent);
//
// irq_chip_enable_parent - Enable the parent interrupt (defaults to unmask if
// NULL)
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_enable_parent(data: *mut irq_data) {
    data = data.parent_data;
    if (data.chip.irq_enable) {
    data.chip.irq_enable(data);
    }
    else {
    data.chip.irq_unmask(data);
    }
    }
    EXPORT_SYMBOL_GPL(irq_chip_enable_parent);
//
// irq_chip_disable_parent - Disable the parent interrupt (defaults to mask if
// NULL)
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_disable_parent(data: *mut irq_data) {
    data = data.parent_data;
    if (data.chip.irq_disable) {
    data.chip.irq_disable(data);
    }
    else {
    data.chip.irq_mask(data);
    }
    }
    EXPORT_SYMBOL_GPL(irq_chip_disable_parent);
//
// irq_chip_ack_parent - Acknowledge the parent interrupt
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_ack_parent(data: *mut irq_data) {
    data = data.parent_data;
    data.chip.irq_ack(data);
    }
    EXPORT_SYMBOL_GPL(irq_chip_ack_parent);
//
// irq_chip_mask_parent - Mask the parent interrupt
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_mask_parent(data: *mut irq_data) {
    data = data.parent_data;
    data.chip.irq_mask(data);
    }
    EXPORT_SYMBOL_GPL(irq_chip_mask_parent);
//
// irq_chip_mask_ack_parent - Mask and acknowledge the parent interrupt
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_mask_ack_parent(data: *mut irq_data) {
    data = data.parent_data;
    data.chip.irq_mask_ack(data);
    }
    EXPORT_SYMBOL_GPL(irq_chip_mask_ack_parent);
//
// irq_chip_unmask_parent - Unmask the parent interrupt
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_unmask_parent(data: *mut irq_data) {
    data = data.parent_data;
    data.chip.irq_unmask(data);
    }
    EXPORT_SYMBOL_GPL(irq_chip_unmask_parent);
//
// irq_chip_eoi_parent - Invoke EOI on the parent interrupt
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_eoi_parent(data: *mut irq_data) {
    data = data.parent_data;
    data.chip.irq_eoi(data);
    }
    EXPORT_SYMBOL_GPL(irq_chip_eoi_parent);
//
// irq_chip_set_affinity_parent - Set affinity on the parent interrupt
// @data:	Pointer to interrupt specific data
// @dest:	The affinity mask to set
// @force:	Flag to enforce setting (disable online checks)
//
// Conditional, as the underlying parent chip might not implement it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_affinity_parent(data: *mut irq_data, dest: *mut cpumask, force: bool) -> c_int {
    data = data.parent_data;
    if (data.chip.irq_set_affinity) {
    return data.chip.irq_set_affinity(data, dest, force);
    }
    return -ENOSYS;
    }
    EXPORT_SYMBOL_GPL(irq_chip_set_affinity_parent);
//
// irq_chip_set_type_parent - Set IRQ type on the parent interrupt
// @data:	Pointer to interrupt specific data
// @type:	IRQ_TYPE_{LEVEL,EDGE}_* value - see include/linux/irq.h
//
// Conditional, as the underlying parent chip might not implement it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_type_parent(data: *mut irq_data, type: c_uint) -> c_int {
    data = data.parent_data;
    if (data.chip.irq_set_type) {
    return data.chip.irq_set_type(data, type);
    }
    return -ENOSYS;
    }
    EXPORT_SYMBOL_GPL(irq_chip_set_type_parent);
//
// irq_chip_retrigger_hierarchy - Retrigger an interrupt in hardware
// @data:	Pointer to interrupt specific data
//
// Iterate through the domain hierarchy of the interrupt and check
// whether a hw retrigger function exists. If yes, invoke it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_retrigger_hierarchy(data: *mut irq_data) -> c_int {
    for (data = data.parent_data; data; data = data.parent_data) {
    if (data.chip && data.chip.irq_retrigger)
    return data.chip.irq_retrigger(data);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_chip_retrigger_hierarchy);
//
// irq_chip_set_vcpu_affinity_parent - Set vcpu affinity on the parent interrupt
// @data:	Pointer to interrupt specific data
// @vcpu_info:	The vcpu affinity information
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_vcpu_affinity_parent(data: *mut irq_data, vcpu_info: *mut c_void) -> c_int {
    data = data.parent_data;
    if (data.chip.irq_set_vcpu_affinity) {
    return data.chip.irq_set_vcpu_affinity(data, vcpu_info);
    }
    return -ENOSYS;
    }
    EXPORT_SYMBOL_GPL(irq_chip_set_vcpu_affinity_parent);
//
// irq_chip_set_wake_parent - Set/reset wake-up on the parent interrupt
// @data:	Pointer to interrupt specific data
// @on:		Whether to set or reset the wake-up capability of this irq
//
// Conditional, as the underlying parent chip might not implement it.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_set_wake_parent(data: *mut irq_data, on: c_uint) -> c_int {
    data = data.parent_data;
    if (data.chip.flags & IRQCHIP_SKIP_SET_WAKE) {
    return 0;
    }
    if (data.chip.irq_set_wake) {
    return data.chip.irq_set_wake(data, on);
    }
    return -ENOSYS;
    }
    EXPORT_SYMBOL_GPL(irq_chip_set_wake_parent);
//
// irq_chip_request_resources_parent - Request resources on the parent interrupt
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_request_resources_parent(data: *mut irq_data) -> c_int {
    data = data.parent_data;
    if (data.chip.irq_request_resources) {
    return data.chip.irq_request_resources(data);
    }
// no error on missing optional irq_chip::irq_request_resources
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_chip_request_resources_parent);
//
// irq_chip_release_resources_parent - Release resources on the parent interrupt
// @data:	Pointer to interrupt specific data
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_release_resources_parent(data: *mut irq_data) {
    data = data.parent_data;
    if (data.chip.irq_release_resources) {
    data.chip.irq_release_resources(data);
    }
    }
    EXPORT_SYMBOL_GPL(irq_chip_release_resources_parent);

#[no_mangle]
pub unsafe extern "C" fn irq_chip_redirect_set_affinity(data: *mut irq_data, dest: *const cpumask, force: bool) -> c_int {
    let mut redir = &irq_data_to_desc(data).redirect;
    WRITE_ONCE(redir.target_cpu, cpumask_first(dest));
    irq_data_update_effective_affinity(data, dest);
    return IRQ_SET_MASK_OK_DONE;
    }
    EXPORT_SYMBOL_GPL(irq_chip_redirect_set_affinity);

//
// irq_chip_compose_msi_msg - Compose msi message for a irq chip
// @data:	Pointer to interrupt specific data
// @msg:	Pointer to the MSI message
//
// For hierarchical domains we find the first chip in the hierarchy
// which implements the irq_compose_msi_msg callback. For non
// hierarchical we use the top level chip.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) -> c_int {
pub static mut pos: *mut c_void = core::ptr::null_mut();
    for (pos = core::ptr::null_mut(); !pos && data; data = irqd_get_parent_data(data)) {
    if (data.chip && data.chip.irq_compose_msi_msg) {
    pos = data;
    }
    }
    if (!pos) {
    return -ENOSYS;
    }
    pos.chip.irq_compose_msi_msg(pos, msg);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_get_pm_device(data: *mut irq_data) -> *mut c_void {
    if (data.domain) {
    return data.domain.pm_dev;
    }
    return core::ptr::null_mut();
    }
//
// irq_chip_pm_get - Enable power for an IRQ chip
// @data:	Pointer to interrupt specific data
//
// Enable the power to the IRQ chip referenced by the interrupt data
// structure.
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_pm_get(data: *mut irq_data) -> c_int {
    let mut dev = irq_get_pm_device(data);
pub static mut retval: c_int = 0;
    if (IS_ENABLED!(CONFIG_PM) && dev) {
    retval = pm_runtime_resume_and_get(dev);
    }
    return retval;
    }
//
// irq_chip_pm_put - Drop a PM reference on an IRQ chip
// @data:	Pointer to interrupt specific data
//
// Drop a power management reference, acquired via irq_chip_pm_get(), on the IRQ
// chip represented by the interrupt data structure.
//
// Note that this will not disable power to the IRQ chip until this function
// has been called for all IRQs that have called irq_chip_pm_get() and it may
// not disable power at all (if user space prevents that, for example).
//
#[no_mangle]
pub unsafe extern "C" fn irq_chip_pm_put(data: *mut irq_data) {
    let mut dev = irq_get_pm_device(data);
    if (dev) {
    pm_runtime_put(dev);
    }
    }