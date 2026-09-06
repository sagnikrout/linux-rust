//! Automatically rewritten from C to Rust
//! Source: kernel/irq/irqdesc.c
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
// This file contains the interrupt descriptor management code. Detailed
// information is available in Documentation/core-api/genericirq.rst
//

//
// lockdep: we want to handle all irq_desc locks as a single lock-class:
//
pub static mut irq_desc_lock_class: usize = 0;

#[no_mangle]
unsafe extern "C" fn irq_affinity_setup(str: *mut c_char) -> c_int {
    alloc_bootmem_cpumask_var(&irq_default_affinity);
    cpulist_parse(str, irq_default_affinity);
//
// Set at least the boot cpu. We don't want to end up with
// bugreports caused by random commandline masks
//
    cpumask_set_cpu(smp_processor_id(), irq_default_affinity);
    return 1;
    }
    __setup!("irqaffinity=", irq_affinity_setup);
#[no_mangle]
unsafe extern "C" fn init_irq_default_affinity()  {
    if (!cpumask_available(irq_default_affinity)) {
    zalloc_cpumask_var(&irq_default_affinity, GFP_NOWAIT);
    }
    if (cpumask_empty(irq_default_affinity)) {
    cpumask_setall(irq_default_affinity);
    }
    }

#[no_mangle]
unsafe extern "C" fn init_irq_default_affinity()  {
    }

#[no_mangle]
unsafe extern "C" fn alloc_masks(desc: *mut irq_desc, node: c_int) -> c_int {
    if (!zalloc_cpumask_var_node(&desc.irq_common_data.affinity,
    GFP_KERNEL, node)) {
    return -ENOMEM;
    }

    if (!zalloc_cpumask_var_node(&desc.irq_common_data.effective_affinity,
    GFP_KERNEL, node)) {
    free_cpumask_var(desc.irq_common_data.affinity);
    return -ENOMEM;
    }

    if (!zalloc_cpumask_var_node(&desc.pending_mask, GFP_KERNEL, node)) {

    free_cpumask_var(desc.irq_common_data.effective_affinity);

    free_cpumask_var(desc.irq_common_data.affinity);
    return -ENOMEM;
    }

    return 0;
    }
#[no_mangle]
unsafe extern "C" fn irq_redirect_work(work: *mut irq_work) {
    handle_irq_desc(container_of!(work, irq_desc, redirect.work));
    }
#[no_mangle]
unsafe extern "C" fn desc_smp_init(desc: *mut irq_desc, node: c_int, affinity: *const cpumask) {
    if (!affinity) {
    affinity = irq_default_affinity;
    }
    cpumask_copy(desc.irq_common_data.affinity, affinity);

    cpumask_clear(desc.pending_mask);

    desc.irq_common_data.node = node;

    desc.redirect.work = IRQ_WORK_INIT_HARD(irq_redirect_work);
    }
#[no_mangle]
unsafe extern "C" fn free_masks(desc: *mut irq_desc) {

    free_cpumask_var(desc.pending_mask);

    free_cpumask_var(desc.irq_common_data.affinity);

    free_cpumask_var(desc.irq_common_data.effective_affinity);

    }

#[no_mangle]
pub unsafe extern "C" fn alloc_masks(desc: *mut irq_desc, node: c_int) -> c_int { return 0; }
#[no_mangle]
pub unsafe extern "C" fn desc_smp_init(desc: *mut irq_desc, node: c_int, affinity: *mut cpumask) { }
#[no_mangle]
pub unsafe extern "C" fn free_masks(desc: *mut irq_desc) { }

#[no_mangle]
pub unsafe extern "C" fn desc_set_defaults(irq: c_uint, desc: *mut irq_desc, node: c_int, affinity: *mut cpumask, owner: *mut module) {
    desc.irq_common_data.handler_data = core::ptr::null_mut();
    desc.irq_common_data.msi_desc = core::ptr::null_mut();
    desc.irq_data.common = &desc.irq_common_data;
    desc.irq_data.irq = irq;
    desc.irq_data.chip = &no_irq_chip;
    desc.irq_data.chip_data = core::ptr::null_mut();
    irq_settings_clr_and_set(desc, ~0, _IRQ_DEFAULT_INIT_FLAGS);
    irqd_set(&desc.irq_data, IRQD_IRQ_DISABLED);
    irqd_set(&desc.irq_data, IRQD_IRQ_MASKED);
    desc.handle_irq = handle_bad_irq;
    desc.depth = 1;
    desc.irq_count = 0;
    desc.irqs_unhandled = 0;
    desc.tot_count = 0;
    desc.name = core::ptr::null_mut();
    desc.owner = owner;
    rcuref_init(&desc.refcnt, 1);
    desc_smp_init(desc, node, affinity);
    }
pub static mut : unsigned int total_nr_irqs = 0;
//
// irq_get_nr_irqs() - Number of interrupts supported by the system.
//
#[no_mangle]
pub unsafe extern "C" fn irq_get_nr_irqs() -> c_uint {
    return total_nr_irqs;
    }
    EXPORT_SYMBOL_GPL(irq_get_nr_irqs);
//
// irq_set_nr_irqs() - Set the number of interrupts supported by the system.
// @nr: New number of interrupts.
//
// Return: @nr.
//
#[no_mangle]
pub unsafe extern "C" fn irq_set_nr_irqs(nr: c_uint) -> unsigned int __init {
    total_nr_irqs = nr;
    irq_proc_calc_prec();
    return nr;
    }
pub static mut sparse_irq_lock: usize = 0;
    static struct maple_tree sparse_irqs = MTREE_INIT_EXT(sparse_irqs,
    MT_FLAGS_ALLOC_RANGE |
    MT_FLAGS_LOCK_EXTERN |
    MT_FLAGS_USE_RCU,
    sparse_irq_lock);
#[no_mangle]
unsafe extern "C" fn irq_find_free_area(from: c_uint, cnt: c_uint) -> c_int {
    MA_STATE(mas, &sparse_irqs, 0, 0);
    if (mas_empty_area(&mas, from, MAX_SPARSE_IRQS, cnt)) {
    return -ENOSPC;
    }
    return mas.index;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_find_desc_at_or_after(offset: c_uint) -> *mut c_void {
pub static mut index: c_ulong = 0;
    lockdep_assert_in_rcu_read_lock();
    return mt_find(&sparse_irqs, &index, total_nr_irqs);
    }
#[no_mangle]
unsafe extern "C" fn irq_insert_desc(irq: c_uint, desc: *mut irq_desc) {
    MA_STATE(mas, &sparse_irqs, irq, irq);
    WARN_ON!(mas_store_gfp(&mas, desc, GFP_KERNEL) != 0);
    }
#[no_mangle]
unsafe extern "C" fn delete_irq_desc(irq: c_uint) {
    MA_STATE(mas, &sparse_irqs, irq, irq);
    mas_erase(&mas);
    }

pub static mut irq_kobj_type: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn init_desc(desc: *mut irq_desc, irq: c_int, node: c_int, flags: c_uint, affinity: *mut cpumask, owner: *mut module) -> c_int {
    desc.kstat_irqs = alloc_percpu(irqstat);
    if (!desc.kstat_irqs) {
    return -ENOMEM;
    }
    if (alloc_masks(desc, node)) {
    free_percpu(desc.kstat_irqs);
    return -ENOMEM;
    }
    raw_spin_lock_init(&desc.lock);
    lockdep_set_class(&desc.lock, &irq_desc_lock_class);
    mutex_init(&desc.request_mutex);
    init_waitqueue_head(&desc.wait_for_threads);
    desc_set_defaults(irq, desc, node, affinity, owner);
    irqd_set(&desc.irq_data, flags);
    irq_resend_init(desc);

    kobject_init(&desc.kobj, &irq_kobj_type);
    init_rcu_head(&desc.rcu);

    return 0;
    }

// forward_decl: irq_kobj_release;

pub static mut irq_kobj_base: *mut c_void = core::ptr::null_mut();

    static struct kobj_attribute _name##_attr = __ATTR_RO(_name)
#[no_mangle]
unsafe extern "C" fn per_cpu_count_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let mut desc = container_of!(kobj, irq_desc, kobj);
pub static mut ret: isize = 0;
    let mut p = "";
    let mut cpu = 0;
    for_each_possible_cpu(cpu) {
pub static mut c: c_uint = 0;
    ret += sysfs_emit_at(buf, ret, "%s%u", p, c);
    p = ",";
    }
    ret += sysfs_emit_at(buf, ret, "\n");
    return ret;
    }
    IRQ_ATTR_RO(per_cpu_count);
#[no_mangle]
unsafe extern "C" fn chip_name_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let mut desc = container_of!(kobj, irq_desc, kobj);
    guard(raw_spinlock_irq)(&desc.lock);
    if (desc.irq_data.chip && desc.irq_data.chip.name) {
    return sysfs_emit(buf, "%s\n", desc.irq_data.chip.name);
    }
    return 0;
    }
    IRQ_ATTR_RO(chip_name);
#[no_mangle]
unsafe extern "C" fn hwirq_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let mut desc = container_of!(kobj, irq_desc, kobj);
    guard(raw_spinlock_irq)(&desc.lock);
    if (desc.irq_data.domain) {
    return sysfs_emit(buf, "%lu\n", desc.irq_data.hwirq);
    }
    return 0;
    }
    IRQ_ATTR_RO(hwirq);
#[no_mangle]
unsafe extern "C" fn type_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let mut desc = container_of!(kobj, irq_desc, kobj);
    guard(raw_spinlock_irq)(&desc.lock);
    return sysfs_emit(buf, "%s\n", irqd_is_level_type(&desc.irq_data) ? "level" : "edge");
    }
    IRQ_ATTR_RO(type);
#[no_mangle]
unsafe extern "C" fn wakeup_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let mut desc = container_of!(kobj, irq_desc, kobj);
    guard(raw_spinlock_irq)(&desc.lock);
    return sysfs_emit(buf, "%s\n", str_enabled_disabled(irqd_is_wakeup_set(&desc.irq_data)));
    }
    IRQ_ATTR_RO(wakeup);
#[no_mangle]
unsafe extern "C" fn name_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let mut desc = container_of!(kobj, irq_desc, kobj);
    guard(raw_spinlock_irq)(&desc.lock);
    if (desc.name) {
    return sysfs_emit(buf, "%s\n", desc.name);
    }
    return 0;
    }
    IRQ_ATTR_RO(name);
#[no_mangle]
unsafe extern "C" fn actions_show(kobj: *mut kobject, attr: *mut kobj_attribute, buf: *mut c_char) -> isize {
    let mut desc = container_of!(kobj, irq_desc, kobj);
pub static mut action: *mut c_void = core::ptr::null_mut();
pub static mut ret: isize = 0;
    let mut p = "";
    scoped_guard(raw_spinlock_irq, &desc.lock) {
    for_each_action_of_desc(desc, action) {
    ret += sysfs_emit_at(buf, ret, "%s%s", p, action.name);
    p = ",";
    }
    }
    if (ret) {
    ret += sysfs_emit_at(buf, ret, "\n");
    }
    return ret;
    }
    IRQ_ATTR_RO(actions);
    static struct attribute *irq_attrs[] = {
    &per_cpu_count_attr.attr,
    &chip_name_attr.attr,
    &hwirq_attr.attr,
    &type_attr.attr,
    &wakeup_attr.attr,
    &name_attr.attr,
    &actions_attr.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(irq);
pub static mut kobj_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn irq_sysfs_add(irq: c_int, desc: *mut irq_desc) {
    if (irq_kobj_base) {
//
// Continue even in case of failure as this is nothing
// crucial and failures in the late irq_sysfs_init()
// cannot be rolled back.
//
    if (kobject_add(&desc.kobj, irq_kobj_base, "%d", irq)) {
    pr_warn!("Failed to add kobject for irq %d\n", irq);
    }
    else {
    desc.istate |= IRQS_SYSFS;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn irq_sysfs_del(desc: *mut irq_desc) {
//
// Only invoke kobject_del() when kobject_add() was successfully
// invoked for the descriptor. This covers both early boot, where
// sysfs is not initialized yet, and the case of a failed
// kobject_add() invocation.
//
    if (desc.istate & IRQS_SYSFS) {
    kobject_del(&desc.kobj);
    }
    }
#[no_mangle]
unsafe extern "C" fn irq_sysfs_init() -> c_int {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut irq = 0;
// Prevent concurrent irq alloc/free
    guard(mutex)(&sparse_irq_lock);
    irq_kobj_base = kobject_create_and_add("irq", kernel_kobj);
    if (!irq_kobj_base) {
    return -ENOMEM;
    }
// Add the already allocated interrupts
    for_each_irq_desc(irq, desc) {
    irq_sysfs_add(irq, desc);
    }
    return 0;
    }
    postcore_initcall!(irq_sysfs_init);

pub static mut kobj_type: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn irq_sysfs_add(irq: c_int, desc: *mut irq_desc) {}
#[no_mangle]
pub unsafe extern "C" fn irq_sysfs_del(desc: *mut irq_desc) {}

#[no_mangle]
pub unsafe extern "C" fn irq_to_desc(irq: c_uint) -> *mut c_void {
    return mtree_load(&sparse_irqs, irq);
    }

    EXPORT_SYMBOL_GPL(irq_to_desc);

#[no_mangle]
pub unsafe extern "C" fn irq_lock_sparse() {
    mutex_lock(&sparse_irq_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_unlock_sparse() {
    mutex_unlock(&sparse_irq_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_desc(irq: c_int, node: c_int, flags: c_uint, affinity: *mut cpumask, owner: *mut module) -> *mut c_void {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    desc = kzalloc_node(sizeof!(*desc), GFP_KERNEL, node);
    if (!desc) {
    return core::ptr::null_mut();
    }
    ret = init_desc(desc, irq, node, flags, affinity, owner);
    if (unlikely(ret)) {
    kfree(desc);
    return core::ptr::null_mut();
    }
    return desc;
    }
#[no_mangle]
unsafe extern "C" fn irq_kobj_release(kobj: *mut kobject) {
    let mut desc = container_of!(kobj, irq_desc, kobj);
    free_masks(desc);
    free_percpu(desc.kstat_irqs);
    kfree(desc);
    }
#[no_mangle]
unsafe extern "C" fn delayed_free_desc(rhp: *mut rcu_head) {
    let mut desc = container_of!(rhp, irq_desc, rcu);
    kobject_put(&desc.kobj);
    }
#[no_mangle]
pub unsafe extern "C" fn irq_desc_free_rcu(desc: *mut irq_desc) {
//
// We free the descriptor, masks and stat fields via RCU. That
// allows demultiplex interrupts to do rcu based management of
// the child interrupts.
// This also allows us to use rcu in kstat_irqs_usr().
//
    call_rcu(&desc.rcu, delayed_free_desc);
    }
#[no_mangle]
unsafe extern "C" fn free_desc(irq: c_uint) {
    let mut desc = irq_to_desc(irq);
    irq_remove_debugfs_entry(desc);
    unregister_irq_proc(irq, desc);
//
// sparse_irq_lock protects also show_interrupts() and
// kstat_irq_usr(). Once we deleted the descriptor from the
// sparse tree we can free it. Access in proc will fail to
// lookup the descriptor.
//
// The sysfs entry must be serialized against a concurrent
// irq_sysfs_init() as well.
//
    irq_sysfs_del(desc);
    delete_irq_desc(irq);
    irq_desc_put_ref(desc);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_descs(start: c_uint, cnt: c_uint, node: c_int, affinity: *mut irq_affinity_desc, owner: *mut module) -> c_int {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
// Validate affinity mask(s)
    if (affinity) {
    while (i < cnt) {
    if (cpumask_empty(&affinity[i].mask)) {
    return -EINVAL;
    }
    }
    }
    while (i < cnt) {
    let mut mask = core::ptr::null_mut();
pub static mut flags: c_uint = 0;
    if (affinity) {
    if (affinity.is_managed) {
    flags = IRQD_AFFINITY_MANAGED |
    IRQD_MANAGED_SHUTDOWN;
    }
    flags |= IRQD_AFFINITY_SET;
    mask = &affinity.mask;
    node = cpu_to_node(cpumask_first(mask));
    affinity += 1;
    }
    desc = alloc_desc(start + i, node, flags, mask, owner);
    if (!desc) {
// goto;
    }
    irq_insert_desc(start + i, desc);
    irq_sysfs_add(start + i, desc);
    irq_add_debugfs_entry(start + i, desc);
    }
    return start;
// label;
    for (i -= 1; i >= 0; i--) {
    free_desc(start + i);
    }
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn irq_expand_nr_irqs(nr: c_uint) -> bool {
    if (nr > MAX_SPARSE_IRQS) {
    return false;
    }
    total_nr_irqs = nr;
    irq_proc_calc_prec();
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn early_irq_init() -> c_int {
    int i, initcnt, node = first_online_node;
pub static mut desc: *mut c_void = core::ptr::null_mut();
    init_irq_default_affinity();
// Let arch update nr_irqs and return the nr of preallocated irqs
    initcnt = arch_probe_nr_irqs();
    printk("NR_IRQS: %d, nr_irqs: %d, preallocated irqs: %d\n",
    NR_IRQS, total_nr_irqs, initcnt);
    if (WARN_ON!(total_nr_irqs > MAX_SPARSE_IRQS)) {
    total_nr_irqs = MAX_SPARSE_IRQS;
    }
    if (WARN_ON!(initcnt > MAX_SPARSE_IRQS)) {
    initcnt = MAX_SPARSE_IRQS;
    }
    if (initcnt > total_nr_irqs) {
    total_nr_irqs = initcnt;
    }
    while (i < initcnt) {
    desc = alloc_desc(i, node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    irq_insert_desc(i, desc);
    }
    irq_proc_calc_prec();
    return arch_early_irq_init();
    }

    struct irq_desc irq_desc[NR_IRQS] __cacheline_aligned_in_smp = {
    [0 ... NR_IRQS-1] = {
    .handle_irq	= handle_bad_irq,
    .depth		= 1,
    .lock		= __RAW_SPIN_LOCK_UNLOCKED(irq_desc.lock),
    }
    };
#[no_mangle]
#[no_mangle]
// duplicate fn: early_irq_init
pub unsafe extern "C" fn early_irq_init_dup() -> c_int {
    int count, i, node = first_online_node;
    let mut ret = 0;
    init_irq_default_affinity();
    pr_info!("NR_IRQS: %d\n", NR_IRQS);
    count = ARRAY_SIZE!(irq_desc);
    while (i < count) {
    ret = init_desc(irq_desc + i, i, node, 0, core::ptr::null_mut(), core::ptr::null_mut());
    if (unlikely(ret)) {
// goto;
    }
    }
    irq_proc_calc_prec();
    return arch_early_irq_init();
// label;
    while (--i >= 0) {
    free_masks(irq_desc + i);
    free_percpu(irq_desc[i].kstat_irqs);
    }
    return ret;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: irq_to_desc
pub unsafe extern "C" fn irq_to_desc_dup(irq: c_uint) -> *mut c_void {
    return (irq < NR_IRQS) ? irq_desc + irq : core::ptr::null_mut();
    }
    EXPORT_SYMBOL(irq_to_desc);
#[no_mangle]
unsafe extern "C" fn free_desc(irq: c_uint) {
    let mut desc = irq_to_desc(irq);
    let mut cpu = 0;
    scoped_guard(raw_spinlock_irqsave, &desc.lock)
    desc_set_defaults(irq, desc, irq_desc_get_node(desc), core::ptr::null_mut(), core::ptr::null_mut());
    for_each_possible_cpu(cpu) {
// per_cpu_ptr(desc->kstat_irqs, cpu) = (irqstat) { };
    }
    delete_irq_desc(irq);
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: alloc_descs
pub unsafe extern "C" fn alloc_descs_dup(start: c_uint, cnt: c_uint, node: c_int, affinity: *mut irq_affinity_desc, owner: *mut module) -> c_int {
    let mut i = 0;
    while (i < cnt) {
    let mut desc = irq_to_desc(start + i);
    desc.owner = owner;
    irq_insert_desc(start + i, desc);
    }
    return start;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_expand_nr_irqs(nr: c_uint) -> bool {
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn irq_mark_irq(irq: c_uint) {
    guard(mutex)(&sparse_irq_lock);
    irq_insert_desc(irq, irq_desc + irq);
    }

#[no_mangle]
pub unsafe extern "C" fn handle_irq_desc(desc: *mut irq_desc) -> c_int {
pub static mut data: *mut c_void = core::ptr::null_mut();
    if (!desc) {
    return -EINVAL;
    }
    data = irq_desc_get_irq_data(desc);
    if (WARN_ON_ONCE!(!in_hardirq() && irqd_is_handle_enforce_irqctx(data))) {
    return -EPERM;
    }
    generic_handle_irq_desc(desc);
    return 0;
    }
//
// generic_handle_irq - Invoke the handler for a particular irq
// @irq:	The irq number to handle
//
// Returns:	0 on success, or -EINVAL if conversion has failed
//
// This function must be called from an IRQ context with irq regs
// initialized.
//
#[no_mangle]
pub unsafe extern "C" fn generic_handle_irq(irq: c_uint) -> c_int {
    return handle_irq_desc(irq_to_desc(irq));
    }
    EXPORT_SYMBOL_GPL(generic_handle_irq);
//
// generic_handle_irq_safe - Invoke the handler for a particular irq from any
// context.
// @irq:	The irq number to handle
//
// Returns:	0 on success, a negative value on error.
//
// This function can be called from any context (IRQ or process context). It
// will report an error if not invoked from IRQ context and the irq has been
// marked to enforce IRQ-context only.
//
#[no_mangle]
pub unsafe extern "C" fn generic_handle_irq_safe(irq: c_uint) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    local_irq_save(flags);
    ret = handle_irq_desc(irq_to_desc(irq));
    local_irq_restore(flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(generic_handle_irq_safe);

//
// generic_handle_domain_irq - Invoke the handler for a HW irq belonging
// to a domain.
// @domain:	The domain where to perform the lookup
// @hwirq:	The HW irq number to convert to a logical one
//
// Returns:	0 on success, or -EINVAL if conversion has failed
//
// This function must be called from an IRQ context with irq regs
// initialized.
//
#[no_mangle]
pub unsafe extern "C" fn generic_handle_domain_irq(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> c_int {
    return handle_irq_desc(irq_resolve_mapping(domain, hwirq));
    }
    EXPORT_SYMBOL_GPL(generic_handle_domain_irq);
//
// generic_handle_irq_safe - Invoke the handler for a HW irq belonging
// to a domain from any context.
// @domain:	The domain where to perform the lookup
// @hwirq:	The HW irq number to convert to a logical one
//
// Returns:	0 on success, a negative value on error.
//
// This function can be called from any context (IRQ or process
// context). If the interrupt is marked as 'enforce IRQ-context only' then
// the function must be invoked from hard interrupt context.
//
#[no_mangle]
pub unsafe extern "C" fn generic_handle_domain_irq_safe(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> c_int {
    let mut flags = 0;
    let mut ret = 0;
    local_irq_save(flags);
    ret = handle_irq_desc(irq_resolve_mapping(domain, hwirq));
    local_irq_restore(flags);
    return ret;
    }
    EXPORT_SYMBOL_GPL(generic_handle_domain_irq_safe);
//
// generic_handle_domain_nmi - Invoke the handler for a HW nmi belonging
// to a domain.
// @domain:	The domain where to perform the lookup
// @hwirq:	The HW irq number to convert to a logical one
//
// Returns:	0 on success, or -EINVAL if conversion has failed
//
// This function must be called from an NMI context with irq regs
// initialized.
//
#[no_mangle]
pub unsafe extern "C" fn generic_handle_domain_nmi(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> c_int {
    WARN_ON_ONCE!(!in_nmi());
    return handle_irq_desc(irq_resolve_mapping(domain, hwirq));
    }

#[no_mangle]
unsafe extern "C" fn demux_redirect_remote(desc: *mut irq_desc) -> bool {
    guard(raw_spinlock)(&desc.lock);
    let mut m = irq_data_get_effective_affinity_mask(&desc.irq_data);
pub static mut target_cpu: c_uint = 0;
    if (desc.irq_data.chip.irq_pre_redirect) {
    desc.irq_data.chip.irq_pre_redirect(&desc.irq_data);
    }
//
// If the interrupt handler is already running on a CPU that's included
// in the interrupt's affinity mask, redirection is not necessary.
//
    if (cpumask_test_cpu(smp_processor_id(), m)) {
    return false;
    }
//
// The desc->action check protects against IRQ shutdown: __free_irq() sets
// desc->action to NULL while holding desc->lock, which we also hold.
//
// Calling irq_work_queue_on() here is safe w.r.t. CPU unplugging:
// - takedown_cpu() schedules multi_cpu_stop() on all active CPUs,
// including the one that's taken down.
// - multi_cpu_stop() acts like a barrier, which means all active
// CPUs go through MULTI_STOP_DISABLE_IRQ and disable hard IRQs
// *before* the dying CPU runs take_cpu_down() in MULTI_STOP_RUN.
// - Hard IRQs are re-enabled at the end of multi_cpu_stop(), *after
// the dying CPU has run take_cpu_down() in MULTI_STOP_RUN.
// - Since we run in hard IRQ context, we run either before or after
// take_cpu_down() but never concurrently.
// - If we run before take_cpu_down(), the dying CPU hasn't been marked
// offline yet (it's marked via take_cpu_down() -> __cpu_disable()),
// so the WARN in irq_work_queue_on() can't occur.
// - Furthermore, the work item we queue will be flushed later via
// take_cpu_down() -> cpuhp_invoke_callback_range_nofail() ->
// smpcfd_dying_cpu() -> irq_work_run().
// - If we run after take_cpu_down(), target_cpu has been already
// updated via take_cpu_down() -> __cpu_disable(), which eventually
// calls irq_do_set_affinity() during IRQ migration. So, target_cpu
// no longer points to the dying CPU in this case.
//
    if (desc.action) {
    irq_work_queue_on(&desc.redirect.work, target_cpu);
    }
    return true;
    }

#[no_mangle]
unsafe extern "C" fn demux_redirect_remote(desc: *mut irq_desc) -> bool {
    return false;
    }

//
// generic_handle_demux_domain_irq - Invoke the handler for a hardware interrupt
// of a demultiplexing domain.
// @domain:	The domain where to perform the lookup
// @hwirq:	The hardware interrupt number to convert to a logical one
//
// Returns:	True on success, or false if lookup has failed
//
#[no_mangle]
pub unsafe extern "C" fn generic_handle_demux_domain_irq(domain: *mut irq_domain, hwirq: irq_hw_number_t) -> bool {
    let mut desc = irq_resolve_mapping(domain, hwirq);
    if (unlikely(!desc)) {
    return false;
    }
    if (demux_redirect_remote(desc)) {
    return true;
    }
    return !handle_irq_desc(desc);
    }
    EXPORT_SYMBOL_GPL(generic_handle_demux_domain_irq);

// Dynamic interrupt handling
//
// irq_free_descs - free irq descriptors
// @from:	Start of descriptor range
// @cnt:	Number of consecutive irqs to free
//
#[no_mangle]
pub unsafe extern "C" fn irq_free_descs(from: c_uint, cnt: c_uint) {
    let mut i = 0;
    if (from >= total_nr_irqs || (from + cnt) > total_nr_irqs) {
    return;
    }
    guard(mutex)(&sparse_irq_lock);
    for (i = 0; i < cnt; i++) {
    free_desc(from + i);
    }
    }
    EXPORT_SYMBOL_GPL(irq_free_descs);
//
// __irq_alloc_descs - allocate and initialize a range of irq descriptors
// @irq:	Allocate for specific irq number if irq >= 0
// @from:	Start the search from this irq number
// @cnt:	Number of consecutive irqs to allocate.
// @node:	Preferred node on which the irq descriptor should be allocated
// @owner:	Owning module (can be NULL)
// @affinity:	Optional pointer to an affinity mask array of size @cnt which
// hints where the irq descriptors should be allocated and which
// default affinities to use
//
// Returns the first irq number or error code
//
    int __ref __irq_alloc_descs(int irq, unsigned int from, unsigned int cnt, int node, module *owner, const struct irq_affinity_desc *affinity)
    {
    let mut start = 0;
    if (!cnt) {
    return -EINVAL;
    }
    if (irq >= 0) {
    if (from > irq) {
    return -EINVAL;
    }
    from = irq;
    } else {
//
// For interrupts which are freely allocated the
// architecture can force a lower bound to the @from
// argument. x86 uses this to exclude the GSI space.
//
    from = arch_dynirq_lower_bound(from);
    }
    guard(mutex)(&sparse_irq_lock);
    start = irq_find_free_area(from, cnt);
    if (irq >=0 && start != irq) {
    return -EEXIST;
    }
    if (start + cnt > total_nr_irqs) {
    if (!irq_expand_nr_irqs(start + cnt)) {
    return -ENOMEM;
    }
    }
    return alloc_descs(start, cnt, node, affinity, owner);
    }
    EXPORT_SYMBOL_GPL(__irq_alloc_descs);
//
// irq_get_next_irq - get next allocated irq number
// @offset:	where to start the search
//
// Returns next irq number after offset or total_nr_irqs if none is found.
//
#[no_mangle]
pub unsafe extern "C" fn irq_get_next_irq(offset: c_uint) -> c_uint {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    guard(rcu)();
    desc = irq_find_desc_at_or_after(offset);
    return desc ? irq_desc_get_irq(desc) : total_nr_irqs;
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_get_desc_lock(irq: c_uint, flags: *mut c_ulong, bus: bool, check: c_uint) -> *mut c_void {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    desc = irq_to_desc(irq);
    if (!desc) {
    return core::ptr::null_mut();
    }
    if (check & _IRQ_DESC_CHECK) {
    if ((check & _IRQ_DESC_PERCPU) && !irq_settings_is_per_cpu_devid(desc)) {
    return core::ptr::null_mut();
    }
    if (!(check & _IRQ_DESC_PERCPU) && irq_settings_is_per_cpu_devid(desc)) {
    return core::ptr::null_mut();
    }
    }
    if (bus) {
    chip_bus_lock(desc);
    }
    raw_spin_lock_irqsave(&desc.lock, *flags);
    return desc;
    }
#[no_mangle]
pub unsafe extern "C" fn __irq_put_desc_unlock(desc: *mut irq_desc, flags: c_ulong, bus: bool) {
    raw_spin_unlock_irqrestore(&desc.lock, flags);
    if (bus) {
    chip_bus_sync_unlock(desc);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn irq_set_percpu_devid(irq: c_uint) -> c_int {
    let mut desc = irq_to_desc(irq);
    if (!desc || desc.percpu_enabled) {
    return -EINVAL;
    }
    desc.percpu_enabled = kzalloc_obj(*desc.percpu_enabled);
    if (!desc.percpu_enabled) {
    return -ENOMEM;
    }
    irq_set_percpu_devid_flags(irq);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kstat_incr_irq_this_cpu(irq: c_uint) {
    kstat_incr_irqs_this_cpu(irq_to_desc(irq));
    }
//
// kstat_irqs_cpu - Get the statistics for an interrupt on a cpu
// @irq:	The interrupt number
// @cpu:	The cpu number
//
// Returns the sum of interrupt counts on @cpu since boot for
// @irq. The caller must ensure that the interrupt is not removed
// concurrently.
//
#[no_mangle]
pub unsafe extern "C" fn kstat_irqs_cpu(irq: c_uint, cpu: c_int) -> c_uint {
    let mut desc = irq_to_desc(irq);
    return desc ? irq_desc_kstat_cpu(desc, cpu) : 0;
    }
#[no_mangle]
unsafe extern "C" fn kstat_irqs_desc(desc: *mut irq_desc, cpumask: *const cpumask) -> c_uint {
pub static mut sum: c_uint = 0;
    let mut cpu = 0;
    if (!irq_settings_is_per_cpu_devid(desc) &&
    !irq_settings_is_per_cpu(desc) &&
    !irq_is_nmi(desc)) {
    return data_race(desc.tot_count);
    }
    for_each_cpu(cpu, cpumask) {
    sum += data_race(per_cpu(desc.kstat_irqs.cnt, cpu));
    }
    return sum;
    }
#[no_mangle]
unsafe extern "C" fn kstat_irqs(irq: c_uint) -> c_uint {
    let mut desc = irq_to_desc(irq);
    if (!desc) {
    return 0;
    }
    return kstat_irqs_desc(desc, cpu_possible_mask);
    }

#[no_mangle]
pub unsafe extern "C" fn kstat_snapshot_irqs() {
pub static mut desc: *mut c_void = core::ptr::null_mut();
    let mut irq = 0;
    for_each_irq_desc(irq, desc) {
    this_cpu_write(desc.kstat_irqs.ref, this_cpu_read(desc.kstat_irqs.cnt));
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kstat_get_irq_since_snapshot(irq: c_uint) -> c_uint {
    let mut desc = irq_to_desc(irq);
    if (!desc) {
    return 0;
    }
    return this_cpu_read(desc.kstat_irqs.cnt) - this_cpu_read(desc.kstat_irqs.ref);
    }

//
// kstat_irqs_usr - Get the statistics for an interrupt from thread context
// @irq:	The interrupt number
//
// Returns the sum of interrupt counts on all cpus since boot for @irq.
//
// It uses rcu to protect the access since a concurrent removal of an
// interrupt descriptor is observing an rcu grace period before
// delayed_free_desc()/irq_kobj_release().
//
#[no_mangle]
pub unsafe extern "C" fn kstat_irqs_usr(irq: c_uint) -> c_uint {
    let mut sum = 0;
    rcu_read_lock();
    sum = kstat_irqs(irq);
    rcu_read_unlock();
    return sum;
    }

#[no_mangle]
pub unsafe extern "C" fn __irq_set_lockdep_class(irq: c_uint, lock_class: *mut lock_class_key, request_class: *mut lock_class_key) {
    let mut desc = irq_to_desc(irq);
    if (desc) {
    lockdep_set_class(&desc.lock, lock_class);
    lockdep_set_class(&desc.request_mutex, request_class);
    }
    }
    EXPORT_SYMBOL_GPL(__irq_set_lockdep_class);