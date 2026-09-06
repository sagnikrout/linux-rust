//! Automatically rewritten from C to Rust
//! Source: kernel/irq/ipi.c
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
// Copyright (C) 2015 Imagination Technologies Ltd
// Author: Qais Yousef <qais.yousef@imgtec.com>
//
// This file contains driver APIs to the IPI subsystem.
//

//
// irq_reserve_ipi() - Setup an IPI to destination cpumask
// @domain:	IPI domain
// @dest:	cpumask of CPUs which can receive the IPI
//
// Allocate a virq that can be used to send IPI to any CPU in dest mask.
//
// Return: Linux IRQ number on success or error code on failure
//
#[no_mangle]
pub unsafe extern "C" fn irq_reserve_ipi(domain: *mut irq_domain, dest: *mut cpumask) -> c_int {
    let mut nr_irqs = 0;
    let mut offset = 0;
pub static mut data: *mut c_void = core::ptr::null_mut();
    let mut virq = 0;
    let mut i = 0;
    if (!domain ||!irq_domain_is_ipi(domain)) {
    pr_warn!("Reservation on a non IPI domain\n");
    return -EINVAL;
    }
    if (!cpumask_subset(dest, cpu_possible_mask)) {
    pr_warn!("Reservation is not in possible_cpu_mask\n");
    return -EINVAL;
    }
    nr_irqs = cpumask_weight(dest);
    if (!nr_irqs) {
    pr_warn!("Reservation for empty destination mask\n");
    return -EINVAL;
    }
    if (irq_domain_is_ipi_single(domain)) {
//
// If the underlying implementation uses a single HW irq on
// all cpus then we only need a single Linux irq number for
// it. We have no restrictions vs. the destination mask. The
// underlying implementation can deal with holes nicely.
//
    nr_irqs = 1;
    offset = 0;
    } else {
    let mut next = 0;
//
// The IPI requires a separate HW irq on each CPU. We require
// that the destination mask is consecutive. If an
// implementation needs to support holes, it can reserve
// several IPI ranges.
//
    offset = cpumask_first(dest);
//
// Find a hole and if found look for another set bit after the
// hole. For now we don't support this scenario.
//
    next = cpumask_next_zero(offset, dest);
    if (next < nr_cpu_ids) {
    next = cpumask_next(next, dest);
    }
    if (next < nr_cpu_ids) {
    pr_warn!("Destination mask has holes\n");
    return -EINVAL;
    }
    }
    virq = irq_domain_alloc_descs(-1, nr_irqs, 0, NUMA_NO_NODE, core::ptr::null_mut());
    if (virq <= 0) {
    pr_warn!("Can't reserve IPI, failed to alloc descs\n");
    return -ENOMEM;
    }
    virq = __irq_domain_alloc_irqs(domain, virq, nr_irqs, NUMA_NO_NODE,
     dest, true, core::ptr::null_mut());
    if (virq <= 0) {
    pr_warn!("Can't reserve IPI, failed to alloc hw irqs\n");
// goto;
    }
    while (i < nr_irqs) {
    data = irq_get_irq_data(virq + i);
    cpumask_copy(data.common.affinity, dest);
    data.common.ipi_offset = offset;
    irq_set_status_flags(virq + i, IRQ_NO_BALANCING);
    }
    return virq;
// label;
    irq_free_descs(virq, nr_irqs);
    return -EBUSY;
    }
//
// irq_destroy_ipi() - unreserve an IPI that was previously allocated
// @irq:	Linux IRQ number to be destroyed
// @dest:	cpumask of CPUs which should have the IPI removed
//
// The IPIs allocated with irq_reserve_ipi() are returned to the system
// destroying all virqs associated with them.
//
// Return: %0 on success or error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn irq_destroy_ipi(irq: c_uint, dest: *const cpumask) -> c_int {
    let mut data = irq_get_irq_data(irq);
pub static mut ipimask: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    let mut nr_irqs = 0;
    if (!irq || !data) {
    return -EINVAL;
    }
    domain = data.domain;
    if (WARN_ON!(domain == core::ptr::null_mut())) {
    return -EINVAL;
    }
    if (!irq_domain_is_ipi(domain)) {
    pr_warn!("Trying to destroy a non IPI domain!\n");
    return -EINVAL;
    }
    ipimask = irq_data_get_affinity_mask(data);
    if (!ipimask || WARN_ON!(!cpumask_subset(dest, ipimask))) {
//
// Must be destroying a subset of CPUs to which this IPI
// was set up to target
//
    return -EINVAL;
    }
    if (irq_domain_is_ipi_per_cpu(domain)) {
    irq = irq + cpumask_first(dest) - data.common.ipi_offset;
    nr_irqs = cpumask_weight(dest);
    } else {
    nr_irqs = 1;
    }
    irq_domain_free_irqs(irq, nr_irqs);
    return 0;
    }
//
// ipi_get_hwirq - Get the hwirq associated with an IPI to a CPU
// @irq:	Linux IRQ number
// @cpu:	the target CPU
//
// When dealing with coprocessors IPI, we need to inform the coprocessor of
// the hwirq it needs to use to receive and send IPIs.
//
// Return: hwirq value on success or INVALID_HWIRQ on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ipi_get_hwirq(irq: c_uint, cpu: c_uint) -> irq_hw_number_t {
    let mut data = irq_get_irq_data(irq);
pub static mut ipimask: *mut c_void = core::ptr::null_mut();
    if (!data || cpu >= nr_cpu_ids) {
    return INVALID_HWIRQ;
    }
    ipimask = irq_data_get_affinity_mask(data);
    if (!ipimask || !cpumask_test_cpu(cpu, ipimask)) {
    return INVALID_HWIRQ;
    }
//
// Get the real hardware irq number if the underlying implementation
// uses a separate irq per cpu. If the underlying implementation uses
// a single hardware irq for all cpus then the IPI send mechanism
// needs to take care of the cpu destinations.
//
    if (irq_domain_is_ipi_per_cpu(data.domain)) {
    data = irq_get_irq_data(irq + cpu - data.common.ipi_offset);
    }
    return data ? irqd_to_hwirq(data) : INVALID_HWIRQ;
    }
    EXPORT_SYMBOL_GPL(ipi_get_hwirq);
#[no_mangle]
pub unsafe extern "C" fn ipi_send_verify(chip: *mut irq_chip, data: *mut irq_data, dest: *mut cpumask, cpu: c_uint) -> c_int {
pub static mut ipimask: *mut c_void = core::ptr::null_mut();
    if (!chip || !data) {
    return -EINVAL;
    }
    if (!chip.ipi_send_single && !chip.ipi_send_mask) {
    return -EINVAL;
    }
    if (cpu >= nr_cpu_ids) {
    return -EINVAL;
    }
    ipimask = irq_data_get_affinity_mask(data);
    if (!ipimask) {
    return -EINVAL;
    }
    if (dest) {
    if (!cpumask_subset(dest, ipimask)) {
    return -EINVAL;
    }
    } else {
    if (!cpumask_test_cpu(cpu, ipimask)) {
    return -EINVAL;
    }
    }
    return 0;
    }
//
// __ipi_send_single - send an IPI to a target Linux SMP CPU
// @desc:	pointer to irq_desc of the IRQ
// @cpu:	destination CPU, must in the destination mask passed to
// irq_reserve_ipi()
//
// This function is for architecture or core code to speed up IPI sending. Not
// usable from driver code.
//
// Return: %0 on success or negative error number on failure.
//
#[no_mangle]
pub unsafe extern "C" fn __ipi_send_single(desc: *mut irq_desc, cpu: c_uint) -> c_int {
    let mut data = irq_desc_get_irq_data(desc);
    let mut chip = irq_data_get_irq_chip(data);

//
// Minimise the overhead by omitting the checks for Linux SMP IPIs.
// Since the callers should be arch or core code which is generally
// trusted, only check for errors when debugging.
//
    if (WARN_ON_ONCE!(ipi_send_verify(chip, data, core::ptr::null_mut(), cpu))) {
    return -EINVAL;
    }

    if (!chip.ipi_send_single) {
    chip.ipi_send_mask(data, cpumask_of(cpu));
    return 0;
    }
// FIXME: Store this information in irqdata flags
    if (irq_domain_is_ipi_per_cpu(data.domain) &&
    cpu != data.common.ipi_offset) {
// use the correct data for that cpu
pub static mut irq: unsigned = 0;
    data = irq_get_irq_data(irq);
    }
    chip.ipi_send_single(data, cpu);
    return 0;
    }
//
// __ipi_send_mask - send an IPI to target Linux SMP CPU(s)
// @desc:	pointer to irq_desc of the IRQ
// @dest:	dest CPU(s), must be a subset of the mask passed to
// irq_reserve_ipi()
//
// This function is for architecture or core code to speed up IPI sending. Not
// usable from driver code.
//
// Return: %0 on success or negative error number on failure.
//
#[no_mangle]
pub unsafe extern "C" fn __ipi_send_mask(desc: *mut irq_desc, dest: *const cpumask) -> c_int {
    let mut data = irq_desc_get_irq_data(desc);
    let mut chip = irq_data_get_irq_chip(data);
    let mut cpu = 0;

//
// Minimise the overhead by omitting the checks for Linux SMP IPIs.
// Since the callers should be arch or core code which is generally
// trusted, only check for errors when debugging.
//
    if (WARN_ON_ONCE!(ipi_send_verify(chip, data, dest, 0))) {
    return -EINVAL;
    }

    if (chip.ipi_send_mask) {
    chip.ipi_send_mask(data, dest);
    return 0;
    }
    if (irq_domain_is_ipi_per_cpu(data.domain)) {
pub static mut base: c_uint = 0;
    for_each_cpu(cpu, dest) {
pub static mut irq: unsigned = 0;
    data = irq_get_irq_data(irq);
    chip.ipi_send_single(data, cpu);
    }
    } else {
    for_each_cpu(cpu, dest) {
    chip.ipi_send_single(data, cpu);
    }
    }
    return 0;
    }
//
// ipi_send_single - Send an IPI to a single CPU
// @virq:	Linux IRQ number from irq_reserve_ipi()
// @cpu:	destination CPU, must in the destination mask passed to
// irq_reserve_ipi()
//
// Return: %0 on success or negative error number on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ipi_send_single(virq: c_uint, cpu: c_uint) -> c_int {
    let mut desc = irq_to_desc(virq);
    let mut data = desc ? irq_desc_get_irq_data(desc) : core::ptr::null_mut();
    let mut chip = data ? irq_data_get_irq_chip(data) : core::ptr::null_mut();
    if (WARN_ON_ONCE!(ipi_send_verify(chip, data, core::ptr::null_mut(), cpu))) {
    return -EINVAL;
    }
    return __ipi_send_single(desc, cpu);
    }
    EXPORT_SYMBOL_GPL(ipi_send_single);
//
// ipi_send_mask - Send an IPI to target CPU(s)
// @virq:	Linux IRQ number from irq_reserve_ipi()
// @dest:	dest CPU(s), must be a subset of the mask passed to
// irq_reserve_ipi()
//
// Return: %0 on success or negative error number on failure.
//
#[no_mangle]
pub unsafe extern "C" fn ipi_send_mask(virq: c_uint, dest: *const cpumask) -> c_int {
    let mut desc = irq_to_desc(virq);
    let mut data = desc ? irq_desc_get_irq_data(desc) : core::ptr::null_mut();
    let mut chip = data ? irq_data_get_irq_chip(data) : core::ptr::null_mut();
    if (WARN_ON_ONCE!(ipi_send_verify(chip, data, dest, 0))) {
    return -EINVAL;
    }
    return __ipi_send_mask(desc, dest);
    }
    EXPORT_SYMBOL_GPL(ipi_send_mask);