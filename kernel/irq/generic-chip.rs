//! Automatically rewritten from C to Rust
//! Source: kernel/irq/generic-chip.c
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
// Library implementing the most common irq chip callback functions
//
// Copyright (C) 2011, Thomas Gleixner
//

pub static mut gc_list: usize = 0;
pub static mut gc_lock: usize = 0;
//
// irq_gc_noop - NOOP function
// @d: irq_data
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_noop(d: *mut irq_data) {
    }
    EXPORT_SYMBOL_GPL(irq_gc_noop);
//
// irq_gc_mask_disable_reg - Mask chip via disable register
// @d: irq_data
//
// Chip has separate enable/disable registers instead of a single mask
// register.
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_mask_disable_reg(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, mask, ct.regs.disable);
// ct->mask_cache &= ~mask;
    }
    EXPORT_SYMBOL_GPL(irq_gc_mask_disable_reg);
//
// irq_gc_mask_set_bit - Mask chip via setting bit in mask register
// @d: irq_data
//
// Chip has a single mask register. Values of this register are cached
// and protected by gc->lock
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_mask_set_bit(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
// ct->mask_cache |= mask;
    irq_reg_writel(gc, *ct.mask_cache, ct.regs.mask);
    }
    EXPORT_SYMBOL_GPL(irq_gc_mask_set_bit);
//
// irq_gc_mask_clr_bit - Mask chip via clearing bit in mask register
// @d: irq_data
//
// Chip has a single mask register. Values of this register are cached
// and protected by gc->lock
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_mask_clr_bit(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
// ct->mask_cache &= ~mask;
    irq_reg_writel(gc, *ct.mask_cache, ct.regs.mask);
    }
    EXPORT_SYMBOL_GPL(irq_gc_mask_clr_bit);
//
// irq_gc_unmask_enable_reg - Unmask chip via enable register
// @d: irq_data
//
// Chip has separate enable/disable registers instead of a single mask
// register.
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_unmask_enable_reg(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, mask, ct.regs.enable);
// ct->mask_cache |= mask;
    }
    EXPORT_SYMBOL_GPL(irq_gc_unmask_enable_reg);
//
// irq_gc_ack_set_bit - Ack pending interrupt via setting bit
// @d: irq_data
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_ack_set_bit(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, mask, ct.regs.ack);
    }
    EXPORT_SYMBOL_GPL(irq_gc_ack_set_bit);
//
// irq_gc_ack_clr_bit - Ack pending interrupt via clearing bit
// @d: irq_data
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_ack_clr_bit(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, mask, ct.regs.ack);
    }
//
// irq_gc_mask_disable_and_ack_set - Mask and ack pending interrupt
// @d: irq_data
//
// This generic implementation of the irq_mask_ack method is for chips
// with separate enable/disable registers instead of a single mask
// register and where a pending interrupt is acknowledged by setting a
// bit.
//
// Note: This is the only permutation currently used.  Similar generic
// functions should be added here if other permutations are required.
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_mask_disable_and_ack_set(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, mask, ct.regs.disable);
// ct->mask_cache &= ~mask;
    irq_reg_writel(gc, mask, ct.regs.ack);
    }
    EXPORT_SYMBOL_GPL(irq_gc_mask_disable_and_ack_set);
//
// irq_gc_eoi - EOI interrupt
// @d: irq_data
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_eoi(d: *mut irq_data) {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = irq_data_get_chip_type(d);
pub static mut mask: u32 = 0;
    guard(raw_spinlock)(&gc.lock);
    irq_reg_writel(gc, mask, ct.regs.eoi);
    }
//
// irq_gc_set_wake - Set/clr wake bit for an interrupt
// @d:  irq_data
// @on: Indicates whether the wake bit should be set or cleared
//
// For chips where the wake from suspend functionality is not
// configured in a separate register and the wakeup active state is
// just stored in a bitmask.
//
#[no_mangle]
pub unsafe extern "C" fn irq_gc_set_wake(d: *mut irq_data, on: c_uint) -> c_int {
    let mut gc = irq_data_get_irq_chip_data(d);
pub static mut mask: u32 = 0;
    if (!(mask & gc.wake_enabled)) {
    return -EINVAL;
    }
    guard(raw_spinlock)(&gc.lock);
    if (on) {
    gc.wake_active |= mask;
    }
    else {
    gc.wake_active &= ~mask;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(irq_gc_set_wake);
#[no_mangle]
unsafe extern "C" fn irq_readl_be(addr: *mut c_void) -> u32 {
    return ioread32be(addr);
    }
#[no_mangle]
unsafe extern "C" fn irq_writel_be(val: u32, addr: *mut c_void) {
    iowrite32be(val, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_init_generic_chip(gc: *mut irq_chip_generic, name: *mut c_char, num_ct: c_int, irq_base: c_uint, reg_base: *mut c_void, handler: irq_flow_handler_t) {
    let mut ct = gc.chip_types;
    let mut i = 0;
    raw_spin_lock_init(&gc.lock);
    gc.num_ct = num_ct;
    gc.irq_base = irq_base;
    gc.reg_base = reg_base;
    for (i = 0; i < num_ct; i++) {
    ct[i].chip.name = name;
    }
    gc.chip_types.handler = handler;
    }
//
// irq_alloc_generic_chip - Allocate a generic chip and initialize it
// @name:	Name of the irq chip
// @num_ct:	Number of irq_chip_type instances associated with this
// @irq_base:	Interrupt base nr for this chip
// @reg_base:	Register base address (virtual)
// @handler:	Default flow handler associated with this chip
//
// Returns an initialized irq_chip_generic structure. The chip defaults
// to the primary (index 0) irq_chip_type and @handler
//
#[no_mangle]
pub unsafe extern "C" fn irq_alloc_generic_chip(name: *mut c_char, num_ct: c_int, irq_base: c_uint, reg_base: *mut c_void, handler: irq_flow_handler_t) -> *mut c_void {
pub static mut gc: *mut c_void = core::ptr::null_mut();
    gc = kzalloc_flex(*gc, chip_types, num_ct);
    if (gc) {
    irq_init_generic_chip(gc, name, num_ct, irq_base, reg_base,
    handler);
    }
    return gc;
    }
    EXPORT_SYMBOL_GPL(irq_alloc_generic_chip);
#[no_mangle]
pub unsafe extern "C" fn irq_gc_init_mask_cache(gc: *mut irq_chip_generic, flags: irq_gc_flags) {
    let mut ct = gc.chip_types;
    let mut mskptr = &gc.mask_cache, mskreg = ct.regs.mask;
    let mut i = 0;
    while (i < gc.num_ct) {
    if (flags & IRQ_GC_MASK_CACHE_PER_TYPE) {
    mskptr = &ct[i].mask_cache_priv;
    mskreg = ct[i].regs.mask;
    }
    ct[i].mask_cache = mskptr;
    if (flags & IRQ_GC_INIT_MASK_CACHE) {
// mskptr = irq_reg_readl(gc, mskreg);
    }
    }
    }
//
// irq_domain_alloc_generic_chips - Allocate generic chips for an irq domain
// @d:		irq domain for which to allocate chips
// @info:	Generic chip information
//
// Return: 0 on success, negative error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_alloc_generic_chips(d: *mut irq_domain, info: *mut irq_domain_chip_generic_info) -> c_int {
pub static mut dgc: *mut c_void = core::ptr::null_mut();
pub static mut gc: *mut c_void = core::ptr::null_mut();
    let mut numchips = 0;
    let mut i = 0;
    let mut dgc_sz = 0;
    let mut gc_sz = 0;
    let mut sz = 0;
pub static mut tmp: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    if (d.gc) {
    return -EBUSY;
    }
    numchips = DIV_ROUND_UP(d.revmap_size, info.irqs_per_chip);
    if (!numchips) {
    return -EINVAL;
    }
// Allocate a pointer, generic chip and chiptypes for each chip
    gc_sz = struct_size(gc, chip_types, info.num_ct);
    dgc_sz = struct_size(dgc, gc, numchips);
    sz = dgc_sz + numchips * gc_sz;
    tmp = dgc = kzalloc(sz, GFP_KERNEL);
    if (!dgc) {
    return -ENOMEM;
    }
    dgc.irqs_per_chip = info.irqs_per_chip;
    dgc.num_chips = numchips;
    dgc.irq_flags_to_set = info.irq_flags_to_set;
    dgc.irq_flags_to_clear = info.irq_flags_to_clear;
    dgc.gc_flags = info.gc_flags;
    dgc.exit = info.exit;
    d.gc = dgc;
// Calc pointer to the first generic chip
    tmp += dgc_sz;
    while (i < numchips) {
// Store the pointer to the generic chip
    dgc.gc[i] = gc = tmp;
    irq_init_generic_chip(gc, info.name, info.num_ct,
    i * dgc.irqs_per_chip, core::ptr::null_mut(),
    info.handler);
    gc.domain = d;
    if (dgc.gc_flags & IRQ_GC_BE_IO) {
    gc.reg_readl = &irq_readl_be;
    gc.reg_writel = &irq_writel_be;
    }
    if (info.init) {
    ret = info.init(gc);
    if (ret) {
// goto;
    }
    }
    scoped_guard (raw_spinlock_irqsave, &gc_lock)
    list_add_tail(&gc.list, &gc_list);
// Calc pointer to the next generic chip
    tmp += gc_sz;
    }
    return 0;
// label;
    while (i--) {
    if (dgc.exit) {
    dgc.exit(dgc.gc[i]);
    }
    irq_remove_generic_chip(dgc.gc[i], ~0U, 0, 0);
    }
    d.gc = core::ptr::null_mut();
    kfree(dgc);
    return ret;
    }
    EXPORT_SYMBOL_GPL(irq_domain_alloc_generic_chips);
//
// irq_domain_remove_generic_chips - Remove generic chips from an irq domain
// @d: irq domain for which generic chips are to be removed
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_remove_generic_chips(d: *mut irq_domain) {
    let mut dgc = d.gc;
    let mut i = 0;
    if (!dgc) {
    return;
    }
    while (i < dgc.num_chips) {
    if (dgc.exit) {
    dgc.exit(dgc.gc[i]);
    }
    irq_remove_generic_chip(dgc.gc[i], ~0U, 0, 0);
    }
    d.gc = core::ptr::null_mut();
    kfree(dgc);
    }
    EXPORT_SYMBOL_GPL(irq_domain_remove_generic_chips);
//
// __irq_alloc_domain_generic_chips - Allocate generic chips for an irq domain
// @d:			irq domain for which to allocate chips
// @irqs_per_chip:	Number of interrupts each chip handles (max 32)
// @num_ct:		Number of irq_chip_type instances associated with this
// @name:		Name of the irq chip
// @handler:		Default flow handler associated with these chips
// @clr:		IRQ_* bits to clear in the mapping function
// @set:		IRQ_* bits to set in the mapping function
// @gcflags:		Generic chip specific setup flags
//
#[no_mangle]
pub unsafe extern "C" fn __irq_alloc_domain_generic_chips(d: *mut irq_domain, irqs_per_chip: c_int, num_ct: c_int, name: *mut c_char, handler: irq_flow_handler_t, clr: c_uint, set: c_uint, gcflags: irq_gc_flags) -> c_int {
pub static mut irq_domain_chip_generic_info: usize = 0;
    return irq_domain_alloc_generic_chips(d, &info);
    }
    EXPORT_SYMBOL_GPL(__irq_alloc_domain_generic_chips);
#[no_mangle]
pub unsafe extern "C" fn __irq_get_domain_generic_chip(d: *mut irq_domain, hw_irq: c_uint) -> *mut c_void {
    let mut dgc = d.gc;
    let mut idx = 0;
    if (!dgc) {
    return ERR_PTR(-ENODEV);
    }
    idx = hw_irq / dgc.irqs_per_chip;
    if (idx >= dgc.num_chips) {
    return ERR_PTR(-EINVAL);
    }
    return dgc.gc[idx];
    }
//
// irq_get_domain_generic_chip - Get a pointer to the generic chip of a hw_irq
// @d:			irq domain pointer
// @hw_irq:		Hardware interrupt number
//
#[no_mangle]
pub unsafe extern "C" fn irq_get_domain_generic_chip(d: *mut irq_domain, hw_irq: c_uint) -> *mut c_void {
    let mut gc = __irq_get_domain_generic_chip(d, hw_irq);
    return !IS_ERR(gc) ? gc : core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(irq_get_domain_generic_chip);
//
// Separate lockdep classes for interrupt chip which can nest irq_desc
// lock and request mutex.
//
pub static mut irq_nested_lock_class: usize = 0;
pub static mut irq_nested_request_class: usize = 0;
//
// irq_map_generic_chip - Map a generic chip for an irq domain
//
#[no_mangle]
pub unsafe extern "C" fn irq_map_generic_chip(d: *mut irq_domain, virq: c_uint, hw_irq: irq_hw_number_t) -> c_int {
    let mut data = irq_domain_get_irq_data(d, virq);
    let mut dgc = d.gc;
pub static mut gc: *mut c_void = core::ptr::null_mut();
pub static mut ct: *mut c_void = core::ptr::null_mut();
pub static mut chip: *mut c_void = core::ptr::null_mut();
    let mut idx = 0;
    gc = __irq_get_domain_generic_chip(d, hw_irq);
    if (IS_ERR(gc)) {
    return PTR_ERR(gc);
    }
    idx = hw_irq % dgc.irqs_per_chip;
    if (test_bit(idx, &gc.unused)) {
    return -ENOTSUPP;
    }
    if (test_bit(idx, &gc.installed)) {
    return -EBUSY;
    }
    ct = gc.chip_types;
    chip = &ct.chip;
// We only init the cache for the first mapping of a generic chip
    if (!gc.installed) {
    guard(raw_spinlock_irqsave)(&gc.lock);
    irq_gc_init_mask_cache(gc, dgc.gc_flags);
    }
// Mark the interrupt as installed
    set_bit(idx, &gc.installed);
    if (dgc.gc_flags & IRQ_GC_INIT_NESTED_LOCK) {
    irq_set_lockdep_class(virq, &irq_nested_lock_class,
    &irq_nested_request_class);
    }
    if (chip.irq_calc_mask) {
    chip.irq_calc_mask(data);
    }
    else {
    data.mask = 1 << idx;
    }
    irq_domain_set_info(d, virq, hw_irq, chip, gc, ct.handler, core::ptr::null_mut(), core::ptr::null_mut());
    irq_modify_status(virq, dgc.irq_flags_to_clear, dgc.irq_flags_to_set);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_unmap_generic_chip(d: *mut irq_domain, virq: c_uint) {
    let mut data = irq_domain_get_irq_data(d, virq);
    let mut dgc = d.gc;
pub static mut hw_irq: c_uint = 0;
pub static mut gc: *mut c_void = core::ptr::null_mut();
    let mut irq_idx = 0;
    gc = irq_get_domain_generic_chip(d, hw_irq);
    if (!gc) {
    return;
    }
    irq_idx = hw_irq % dgc.irqs_per_chip;
    clear_bit(irq_idx, &gc.installed);
    irq_domain_set_info(d, virq, hw_irq, &no_irq_chip, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(),
    core::ptr::null_mut());
    }
pub static mut irq_domain_ops: usize = 0;
    EXPORT_SYMBOL_GPL(irq_generic_chip_ops);
//
// irq_setup_generic_chip - Setup a range of interrupts with a generic chip
// @gc:		Generic irq chip holding all data
// @msk:	Bitmask holding the irqs to initialize relative to gc->irq_base
// @flags:	Flags for initialization
// @clr:	IRQ_* bits to clear
// @set:	IRQ_* bits to set
//
// Set up max. 32 interrupts starting from gc->irq_base. Note, this
// initializes all interrupts to the primary irq_chip_type and its
// associated handler.
//
#[no_mangle]
pub unsafe extern "C" fn irq_setup_generic_chip(gc: *mut irq_chip_generic, msk: u32, flags: irq_gc_flags, clr: c_uint, set: c_uint) {
    let mut ct = gc.chip_types;
    let mut chip = &ct.chip;
    let mut i = 0;
    scoped_guard (raw_spinlock, &gc_lock)
    list_add_tail(&gc.list, &gc_list);
    irq_gc_init_mask_cache(gc, flags);
    while (msk) {
    if (!(msk & 0x01)) {
    continue;
    }
    if (flags & IRQ_GC_INIT_NESTED_LOCK) {
    irq_set_lockdep_class(i, &irq_nested_lock_class,
    &irq_nested_request_class);
    }
    if (!(flags & IRQ_GC_NO_MASK)) {
    let mut d = irq_get_irq_data(i);
    if (chip.irq_calc_mask) {
    chip.irq_calc_mask(d);
    }
    else {
    d.mask = 1 << (i - gc.irq_base);
    }
    }
    irq_set_chip_and_handler(i, chip, ct.handler);
    irq_set_chip_data(i, gc);
    irq_modify_status(i, clr, set);
    }
    gc.irq_cnt = i - gc.irq_base;
    }
    EXPORT_SYMBOL_GPL(irq_setup_generic_chip);
//
// irq_setup_alt_chip - Switch to alternative chip
// @d:		irq_data for this interrupt
// @type:	Flow type to be initialized
//
// Only to be called from chip->irq_set_type() callbacks.
//
#[no_mangle]
pub unsafe extern "C" fn irq_setup_alt_chip(d: *mut irq_data, type: c_uint) -> c_int {
    let mut gc = irq_data_get_irq_chip_data(d);
    let mut ct = gc.chip_types;
    let mut i = 0;
    while (i < gc.num_ct) {
    if (ct.type & type) {
    d.chip = &ct.chip;
    irq_data_to_desc(d).handle_irq = ct.handler;
    return 0;
    }
    }
    return -EINVAL;
    }
    EXPORT_SYMBOL_GPL(irq_setup_alt_chip);
//
// irq_remove_generic_chip - Remove a chip
// @gc:		Generic irq chip holding all data
// @msk:	Bitmask holding the irqs to initialize relative to gc->irq_base
// @clr:	IRQ_* bits to clear
// @set:	IRQ_* bits to set
//
// Remove up to 32 interrupts starting from gc->irq_base.
//
#[no_mangle]
pub unsafe extern "C" fn irq_remove_generic_chip(gc: *mut irq_chip_generic, msk: u32, clr: c_uint, set: c_uint) {
    let mut i = 0;
    let mut virq = 0;
    scoped_guard (raw_spinlock, &gc_lock)
    list_del(&gc.list);
    while (msk) {
    if (!(msk & 0x01)) {
    continue;
    }
//
// Interrupt domain based chips store the base hardware
// interrupt number in gc::irq_base. Otherwise gc::irq_base
// contains the base Linux interrupt number.
//
    if (gc.domain) {
    virq = irq_find_mapping(gc.domain, gc.irq_base + i);
    if (!virq) {
    continue;
    }
    } else {
    virq = gc.irq_base + i;
    }
// Remove handler first. That will mask the irq line
    irq_set_handler(virq, core::ptr::null_mut());
    irq_set_chip(virq, &no_irq_chip);
    irq_set_chip_data(virq, core::ptr::null_mut());
    irq_modify_status(virq, clr, set);
    }
    }
    EXPORT_SYMBOL_GPL(irq_remove_generic_chip);
#[no_mangle]
pub unsafe extern "C" fn irq_gc_get_irq_data(gc: *mut irq_chip_generic) -> *mut c_void {
    let mut virq = 0;
    if (!gc.domain) {
    return irq_get_irq_data(gc.irq_base);
    }
//
// We don't know which of the irqs has been actually
// installed. Use the first one.
//
    if (!gc.installed) {
    return core::ptr::null_mut();
    }
    virq = irq_find_mapping(gc.domain, gc.irq_base + __ffs(gc.installed));
    return virq ? irq_get_irq_data(virq) : core::ptr::null_mut();
    }

#[no_mangle]
unsafe extern "C" fn irq_gc_suspend(data: *mut c_void) -> c_int {
pub static mut gc: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(gc, &gc_list, list) {
    let mut ct = gc.chip_types;
    if (ct.chip.irq_suspend) {
    let mut data = irq_gc_get_irq_data(gc);
    if (data) {
    ct.chip.irq_suspend(data);
    }
    }
    if (gc.suspend) {
    gc.suspend(gc);
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_gc_resume(data: *mut c_void) {
pub static mut gc: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(gc, &gc_list, list) {
    let mut ct = gc.chip_types;
    if (gc.resume) {
    gc.resume(gc);
    }
    if (ct.chip.irq_resume) {
    let mut data = irq_gc_get_irq_data(gc);
    if (data) {
    ct.chip.irq_resume(data);
    }
    }
    }
    }

#[no_mangle]
unsafe extern "C" fn irq_gc_shutdown(data: *mut c_void) {
pub static mut gc: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(gc, &gc_list, list) {
    let mut ct = gc.chip_types;
    if (ct.chip.irq_pm_shutdown) {
    let mut data = irq_gc_get_irq_data(gc);
    if (data) {
    ct.chip.irq_pm_shutdown(data);
    }
    }
    }
    }
pub static mut syscore_ops: usize = 0;
pub static mut syscore: usize = 0;
#[no_mangle]
unsafe extern "C" fn irq_gc_init_ops() -> c_int {
    register_syscore(&irq_gc_syscore);
    return 0;
    }
    device_initcall!(irq_gc_init_ops);