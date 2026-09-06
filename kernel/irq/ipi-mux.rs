//! Automatically rewritten from C to Rust
//! Source: kernel/irq/ipi-mux.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Multiplex several virtual IPIs over a single HW IPI.
//
// Copyright The Asahi Linux Contributors
// Copyright (c) 2022 Ventana Micro Systems Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipi_mux_cpu {
    pub enable: core::sync::atomic::AtomicI32,
    pub bits: core::sync::atomic::AtomicI32,
}

    static struct ipi_mux_cpu  *ipi_mux_pcpu;
pub static mut ipi_mux_domain: *mut c_void = core::ptr::null_mut();
    static void (*ipi_mux_send)(unsigned int cpu);
#[no_mangle]
unsafe extern "C" fn ipi_mux_mask(d: *mut irq_data) {
    let mut icpu = this_cpu_ptr(ipi_mux_pcpu);
    atomic_andnot(BIT(irqd_to_hwirq(d)), &icpu.enable);
    }
#[no_mangle]
unsafe extern "C" fn ipi_mux_unmask(d: *mut irq_data) {
    let mut icpu = this_cpu_ptr(ipi_mux_pcpu);
pub static mut ibit: u32 = 0;
    atomic_or(ibit, &icpu.enable);
//
// The atomic_or() above must complete before the atomic_read()
// below to avoid racing ipi_mux_send_mask().
//
    smp_mb__after_atomic();
// If a pending IPI was unmasked, raise a parent IPI immediately.
    if (atomic_read(&icpu.bits) & ibit) {
    ipi_mux_send(smp_processor_id());
    }
    }
#[no_mangle]
unsafe extern "C" fn ipi_mux_send_mask(d: *mut irq_data, mask: *const cpumask) {
    let mut icpu = this_cpu_ptr(ipi_mux_pcpu);
pub static mut ibit: u32 = 0;
    let mut pending = 0;
    let mut cpu = 0;
    for_each_cpu(cpu, mask) {
    icpu = per_cpu_ptr(ipi_mux_pcpu, cpu);
//
// This sequence is the mirror of the one in ipi_mux_unmask();
// see the comment there. Additionally, release semantics
// ensure that the vIPI flag set is ordered after any shared
// memory accesses that precede it. This therefore also pairs
// with the atomic_fetch_andnot in ipi_mux_process().
//
    pending = atomic_fetch_or_release(ibit, &icpu.bits);
//
// The atomic_fetch_or_release() above must complete
// before the atomic_read() below to avoid racing with
// ipi_mux_unmask().
//
    smp_mb__after_atomic();
//
// The flag writes must complete before the physical IPI is
// issued to another CPU. This is implied by the control
// dependency on the result of atomic_read() below, which is
// itself already ordered after the vIPI flag write.
//
    if (!(pending & ibit) && (atomic_read(&icpu.enable) & ibit)) {
    ipi_mux_send(cpu);
    }
    }
    }
pub static mut irq_chip: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn ipi_mux_domain_alloc(d: *mut irq_domain, virq: c_uint, nr_irqs: c_uint, arg: *mut c_void) -> c_int {
    let mut i = 0;
    while (i < nr_irqs) {
    irq_set_percpu_devid(virq + i);
    irq_domain_set_info(d, virq + i, i, &ipi_mux_chip, core::ptr::null_mut(),
    handle_percpu_devid_irq, core::ptr::null_mut(), core::ptr::null_mut());
    }
    return 0;
    }
pub static mut irq_domain_ops: usize = 0;
//
// ipi_mux_process - Process multiplexed virtual IPIs
//
#[no_mangle]
pub unsafe extern "C" fn ipi_mux_process() {
    let mut icpu = this_cpu_ptr(ipi_mux_pcpu);
    let mut hwirq;
    let mut ipis = 0;
    let mut en = 0;
//
// Reading enable mask does not need to be ordered as long as
// this function is called from interrupt handler because only
// the CPU itself can change it's own enable mask.
//
    en = atomic_read(&icpu.enable);
//
// Clear the IPIs we are about to handle. This pairs with the
// atomic_fetch_or_release() in ipi_mux_send_mask().
//
    ipis = atomic_fetch_andnot(en, &icpu.bits) & en;
    for_each_set_bit(hwirq, &ipis, BITS_PER_TYPE(int)) {
    generic_handle_domain_irq(ipi_mux_domain, hwirq);
    }
    }
//
// ipi_mux_create - Create virtual IPIs multiplexed on top of a single
// parent IPI.
// @nr_ipi:		number of virtual IPIs to create. This should
// be <= BITS_PER_TYPE(int)
// @mux_send:		callback to trigger parent IPI for a particular CPU
//
// Returns first virq of the newly created virtual IPIs upon success
// or <=0 upon failure
//
#[no_mangle]
pub unsafe extern "C" fn ipi_mux_create(nr_ipi: c_uint, cpu): *mut *mut c_void (mux_send)(unsigned int) -> c_int {
#[no_mangle]
#[no_mangle]
// duplicate fn: ipi_mux_create
pub unsafe extern "C" fn ipi_mux_create_dup(nr_ipi: c_uint) -> c_int {
pub static mut fwnode: *mut c_void = core::ptr::null_mut();
pub static mut domain: *mut c_void = core::ptr::null_mut();
    let mut rc = 0;
    if (ipi_mux_domain) {
    return -EEXIST;
    }
    if (BITS_PER_TYPE(int) < nr_ipi || !mux_send) {
    return -EINVAL;
    }
    ipi_mux_pcpu = alloc_percpu(typeof(*ipi_mux_pcpu));
    if (!ipi_mux_pcpu) {
    return -ENOMEM;
    }
    fwnode = irq_domain_alloc_named_fwnode("IPI-Mux");
    if (!fwnode) {
    pr_err!("unable to create IPI Mux fwnode\n");
    rc = -ENOMEM;
// goto;
    }
    domain = irq_domain_create_linear(fwnode, nr_ipi,
    &ipi_mux_domain_ops, core::ptr::null_mut());
    if (!domain) {
    pr_err!("unable to add IPI Mux domain\n");
    rc = -ENOMEM;
// goto;
    }
    domain.flags |= IRQ_DOMAIN_FLAG_IPI_SINGLE;
    irq_domain_update_bus_token(domain, DOMAIN_BUS_IPI);
    rc = irq_domain_alloc_irqs(domain, nr_ipi, NUMA_NO_NODE, core::ptr::null_mut());
    if (rc <= 0) {
    pr_err!("unable to alloc IRQs from IPI Mux domain\n");
// goto;
    }
    ipi_mux_domain = domain;
    ipi_mux_send = mux_send;
    return rc;
// label;
    irq_domain_remove(domain);
// label;
    irq_domain_free_fwnode(fwnode);
// label;
    free_percpu(ipi_mux_pcpu);
    return rc;
    }
}
