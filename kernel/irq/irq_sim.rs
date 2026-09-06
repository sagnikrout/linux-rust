//! Automatically rewritten from C to Rust
//! Source: kernel/irq/irq_sim.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Copyright (C) 2017-2018 Bartosz Golaszewski <brgl@bgdev.pl>
// Copyright (C) 2020 Bartosz Golaszewski <bgolaszewski@baylibre.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_sim_work_ctx {
    pub work: irq_work,
    pub irq_count: c_uint,
    pub pending: *mut c_ulong,
    pub domain: *mut irq_domain,
    pub ops: irq_sim_ops,
    pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_sim_irq_ctx {
    pub enabled: bool,
    pub work_ctx: *mut irq_sim_work_ctx,
}

#[no_mangle]
unsafe extern "C" fn irq_sim_irqmask(data: *mut irq_data) {
    let mut irq_ctx = irq_data_get_irq_chip_data(data);
    irq_ctx.enabled = false;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_irqunmask(data: *mut irq_data) {
    let mut irq_ctx = irq_data_get_irq_chip_data(data);
    irq_ctx.enabled = true;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_set_type(data: *mut irq_data, type: c_uint) -> c_int {
// We only support rising and falling edge trigger types.
    if (type & ~IRQ_TYPE_EDGE_BOTH) {
    return -EINVAL;
    }
    irqd_set_trigger_type(data, type);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_sim_get_irqchip_state(data: *mut irq_data, which: irqchip_irq_state, state: *mut bool) -> c_int {
    let mut irq_ctx = irq_data_get_irq_chip_data(data);
pub static mut hwirq: irq_hw_number_t = 0;
    match (which) {
    IRQCHIP_STATE_PENDING => {
    if (irq_ctx.enabled) {
// state = test_bit(hwirq, irq_ctx->work_ctx->pending);
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_sim_set_irqchip_state(data: *mut irq_data, which: irqchip_irq_state, state: bool) -> c_int {
    let mut irq_ctx = irq_data_get_irq_chip_data(data);
pub static mut hwirq: irq_hw_number_t = 0;
    match (which) {
    IRQCHIP_STATE_PENDING => {
    if (irq_ctx.enabled) {
    assign_bit(hwirq, irq_ctx.work_ctx.pending, state);
    if (state) {
    irq_work_queue(&irq_ctx.work_ctx.work);
    }
    }
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_request_resources(data: *mut irq_data) -> c_int {
    let mut irq_ctx = irq_data_get_irq_chip_data(data);
    let mut work_ctx = irq_ctx.work_ctx;
pub static mut hwirq: irq_hw_number_t = 0;
    if (work_ctx.ops.irq_sim_irq_requested) {
    return work_ctx.ops.irq_sim_irq_requested(work_ctx.domain,
    hwirq,
    work_ctx.user_data);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_release_resources(data: *mut irq_data) {
    let mut irq_ctx = irq_data_get_irq_chip_data(data);
    let mut work_ctx = irq_ctx.work_ctx;
pub static mut hwirq: irq_hw_number_t = 0;
    if (work_ctx.ops.irq_sim_irq_released) {
    work_ctx.ops.irq_sim_irq_released(work_ctx.domain, hwirq,
    work_ctx.user_data);
    }
    }
pub static mut irq_chip: usize = 0;
#[no_mangle]
unsafe extern "C" fn irq_sim_handle_irq(work: *mut irq_work) {
pub static mut work_ctx: *mut c_void = core::ptr::null_mut();
pub static mut offset: c_uint = 0;
    let mut irqnum = 0;
    work_ctx = container_of!(work, irq_sim_work_ctx, work);
    while (!bitmap_empty(work_ctx.pending, work_ctx.irq_count)) {
    offset = find_next_bit(work_ctx.pending,
    work_ctx.irq_count, offset);
    clear_bit(offset, work_ctx.pending);
    irqnum = irq_find_mapping(work_ctx.domain, offset);
    handle_simple_irq(irq_to_desc(irqnum));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irq_sim_domain_map(domain: *mut irq_domain, virq: c_uint, hw: irq_hw_number_t) -> c_int {
    let mut work_ctx = domain.host_data;
pub static mut irq_ctx: *mut c_void = core::ptr::null_mut();
    irq_ctx = kzalloc_obj(*irq_ctx);
    if (!irq_ctx) {
    return -ENOMEM;
    }
    irq_set_chip(virq, &irq_sim_irqchip);
    irq_set_chip_data(virq, irq_ctx);
    irq_set_handler(virq, handle_simple_irq);
    irq_modify_status(virq, IRQ_NOREQUEST | IRQ_NOAUTOEN, IRQ_NOPROBE);
    irq_ctx.work_ctx = work_ctx;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_sim_domain_unmap(domain: *mut irq_domain, virq: c_uint) {
pub static mut irq_ctx: *mut c_void = core::ptr::null_mut();
pub static mut irqd: *mut c_void = core::ptr::null_mut();
    irqd = irq_domain_get_irq_data(domain, virq);
    irq_ctx = irq_data_get_irq_chip_data(irqd);
    irq_set_handler(virq, core::ptr::null_mut());
    irq_domain_reset_irq_data(irqd);
    kfree(irq_ctx);
    }
pub static mut irq_domain_ops: usize = 0;
//
// irq_domain_create_sim - Create a new interrupt simulator irq_domain and
// allocate a range of dummy interrupts.
//
// @fwnode: fwnode_handle to be associated with this domain.
// @num_irqs:   Number of interrupts to allocate.
//
// On success: return a new irq_domain object.
// On failure: a negative errno wrapped with ERR_PTR().
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_create_sim(fwnode: *mut fwnode_handle, num_irqs: c_uint) -> *mut c_void {
    return irq_domain_create_sim_full(fwnode, num_irqs, core::ptr::null_mut(), core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(irq_domain_create_sim);
#[no_mangle]
pub unsafe extern "C" fn irq_domain_create_sim_full(fwnode: *mut fwnode_handle, num_irqs: c_uint, ops: *mut irq_sim_ops, data: *mut c_void) -> *mut c_void {
    struct irq_sim_work_ctx *work_ctx __free(kfree) =
    kzalloc_obj(*work_ctx);
    if (!work_ctx) {
    return ERR_PTR(-ENOMEM);
    }
    unsigned long *pending __free(bitmap) = bitmap_zalloc(num_irqs, GFP_KERNEL);
    if (!pending) {
    return ERR_PTR(-ENOMEM);
    }
    work_ctx.domain = irq_domain_create_linear(fwnode, num_irqs,
    &irq_sim_domain_ops,
    work_ctx);
    if (!work_ctx.domain) {
    return ERR_PTR(-ENOMEM);
    }
    work_ctx.irq_count = num_irqs;
    work_ctx.work = IRQ_WORK_INIT_HARD(irq_sim_handle_irq);
    work_ctx.pending = no_free_ptr(pending);
    work_ctx.user_data = data;
    if (ops) {
    memcpy(&work_ctx.ops, ops, sizeof!(*ops));
    }
    return no_free_ptr(work_ctx).domain;
    }
    EXPORT_SYMBOL_GPL(irq_domain_create_sim_full);
//
// irq_domain_remove_sim - Deinitialize the interrupt simulator domain: free
// the interrupt descriptors and allocated memory.
//
// @domain:     The interrupt simulator domain to tear down.
//
#[no_mangle]
pub unsafe extern "C" fn irq_domain_remove_sim(domain: *mut irq_domain) {
    let mut work_ctx = domain.host_data;
    irq_work_sync(&work_ctx.work);
    bitmap_free(work_ctx.pending);
    kfree(work_ctx);
    irq_domain_remove(domain);
    }
    EXPORT_SYMBOL_GPL(irq_domain_remove_sim);
#[no_mangle]
unsafe extern "C" fn devm_irq_domain_remove_sim(data: *mut c_void) {
    let mut domain = data;
    irq_domain_remove_sim(domain);
    }
//
// devm_irq_domain_create_sim - Create a new interrupt simulator for
// a managed device.
//
// @dev:        Device to initialize the simulator object for.
// @fwnode: fwnode_handle to be associated with this domain.
// @num_irqs:   Number of interrupts to allocate
//
// On success: return a new irq_domain object.
// On failure: a negative errno wrapped with ERR_PTR().
//
#[no_mangle]
pub unsafe extern "C" fn devm_irq_domain_create_sim(dev: *mut device, fwnode: *mut fwnode_handle, num_irqs: c_uint) -> *mut c_void {
    return devm_irq_domain_create_sim_full(dev, fwnode, num_irqs,
    core::ptr::null_mut(), core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(devm_irq_domain_create_sim);
#[no_mangle]
pub unsafe extern "C" fn devm_irq_domain_create_sim_full(dev: *mut device, fwnode: *mut fwnode_handle, num_irqs: c_uint, ops: *mut irq_sim_ops, data: *mut c_void) -> *mut c_void {
pub static mut domain: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    domain = irq_domain_create_sim_full(fwnode, num_irqs, ops, data);
    if (IS_ERR(domain)) {
    return domain;
    }
    ret = devm_add_action_or_reset(dev, devm_irq_domain_remove_sim, domain);
    if (ret) {
    return ERR_PTR(ret);
    }
    return domain;
    }
    EXPORT_SYMBOL_GPL(devm_irq_domain_create_sim_full);