//! Automatically rewritten from C to Rust
//! Source: kernel/power/qos.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Power Management Quality of Service (PM QoS) support base.
//
// Copyright (C) 2020 Intel Corporation
//
// Authors:
// Mark Gross <mgross@linux.intel.com>
// Rafael J. Wysocki <rafael.j.wysocki@intel.com>
//
// Provided here is an interface for specifying PM QoS dependencies.  It allows
// entities depending on QoS constraints to register their requests which are
// aggregated as appropriate to produce effective constraints (target values)
// that can be monitored by entities needing to respect them, either by polling
// or through a built-in notification mechanism.
//
// In addition to the basic functionality, more specific interfaces for managing
// global CPU latency QoS requests and frequency QoS requests are provided.
//
// #define DEBUG

//
// locking rule: all changes to constraints or notifiers lists
// or pm_qos_object list and pm_qos_objects need to happen with pm_qos_lock
// held, taken with _irqsave.  One lock to rule them all
//
pub static mut pm_qos_lock: usize = 0;
//
// pm_qos_read_value - Return the current effective constraint value.
// @c: List of PM QoS constraint requests.
//
#[no_mangle]
pub unsafe extern "C" fn pm_qos_read_value(c: *mut pm_qos_constraints) -> i32 {
    return READ_ONCE(c.target_value);
    }
#[no_mangle]
unsafe extern "C" fn pm_qos_get_value(c: *mut pm_qos_constraints) -> c_int {
    if (plist_head_empty(&c.list)) {
    return c.no_constraint_value;
    }
    match (c.type) {
    PM_QOS_MIN => {
    return plist_first(&c.list).prio;
    }
    PM_QOS_MAX => {
    return plist_last(&c.list).prio;
    }
    _ => {
    WARN(1, "Unknown PM QoS type in %s\n", __func__);
    return PM_QOS_DEFAULT_VALUE;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn pm_qos_set_value(c: *mut pm_qos_constraints, value: i32) {
    WRITE_ONCE(c.target_value, value);
    }
//
// pm_qos_update_target - Update a list of PM QoS constraint requests.
// @c: List of PM QoS requests.
// @node: Target list entry.
// @action: Action to carry out (add, update or remove).
// @value: New request value for the target list entry.
//
// Update the given list of PM QoS constraint requests, @c, by carrying an
// @action involving the @node list entry and @value on it.
//
// The recognized values of @action are PM_QOS_ADD_REQ (store @value in @node
// and add it to the list), PM_QOS_UPDATE_REQ (remove @node from the list, store
// @value in it and add it to the list again), and PM_QOS_REMOVE_REQ (remove
// @node from the list, ignore @value).
//
// Return: 1 if the aggregate constraint value has changed, 0  otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pm_qos_update_target(c: *mut pm_qos_constraints, node: *mut plist_node, action: pm_qos_req_action, value: c_int) -> c_int {
    let mut prev_value = 0;
    let mut curr_value = 0;
    let mut new_value = 0;
    let mut flags = 0;
    spin_lock_irqsave(&pm_qos_lock, flags);
    prev_value = pm_qos_get_value(c);
    if (value == PM_QOS_DEFAULT_VALUE) {
    new_value = c.default_value;
    }
    else {
    new_value = value;
    }
    match (action) {
    PM_QOS_REMOVE_REQ => {
    plist_del(node, &c.list);
    // break;
    }
    PM_QOS_UPDATE_REQ => {
//
// To change the list, atomically remove, reinit with new value
// and add, then see if the aggregate has changed.
//
    plist_del(node, &c.list);
    fallthrough;
    }
    PM_QOS_ADD_REQ => {
    plist_node_init(node, new_value);
    plist_add(node, &c.list);
    // break;
    }
    _ => {
// no action
    ;
    }
    }
    curr_value = pm_qos_get_value(c);
    pm_qos_set_value(c, curr_value);
    spin_unlock_irqrestore(&pm_qos_lock, flags);
    trace_pm_qos_update_target(action, prev_value, curr_value);
    if (prev_value == curr_value) {
    return 0;
    }
    if (c.notifiers) {
    blocking_notifier_call_chain(c.notifiers, curr_value, core::ptr::null_mut());
    }
    return 1;
    }
//
// pm_qos_flags_remove_req - Remove device PM QoS flags request.
// @pqf: Device PM QoS flags set to remove the request from.
// @req: Request to remove from the set.
//
#[no_mangle]
pub unsafe extern "C" fn pm_qos_flags_remove_req(pqf: *mut pm_qos_flags, req: *mut pm_qos_flags_request) {
pub static mut val: i32 = 0;
    list_del(&req.node);
    list_for_each_entry(req, &pqf.list, node) {
    val |= req.flags;
    }
    pqf.effective_flags = val;
    }
//
// pm_qos_update_flags - Update a set of PM QoS flags.
// @pqf: Set of PM QoS flags to update.
// @req: Request to add to the set, to modify, or to remove from the set.
// @action: Action to take on the set.
// @val: Value of the request to add or modify.
//
// Return: 1 if the aggregate constraint value has changed, 0 otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn pm_qos_update_flags(pqf: *mut pm_qos_flags, req: *mut pm_qos_flags_request, action: pm_qos_req_action, val: s32) -> bool {
    let mut irqflags = 0;
    s32 prev_value, curr_value;
    spin_lock_irqsave(&pm_qos_lock, irqflags);
    prev_value = list_empty(&pqf.list) ? 0 : pqf.effective_flags;
    match (action) {
    PM_QOS_REMOVE_REQ => {
    pm_qos_flags_remove_req(pqf, req);
    // break;
    }
    PM_QOS_UPDATE_REQ => {
    pm_qos_flags_remove_req(pqf, req);
    fallthrough;
    }
    PM_QOS_ADD_REQ => {
    req.flags = val;
    INIT_LIST_HEAD(&req.node);
    list_add_tail(&req.node, &pqf.list);
    pqf.effective_flags |= val;
    // break;
    }
    _ => {
// no action
    ;
    }
    }
    curr_value = list_empty(&pqf.list) ? 0 : pqf.effective_flags;
    spin_unlock_irqrestore(&pm_qos_lock, irqflags);
    trace_pm_qos_update_flags(action, prev_value, curr_value);
    return prev_value != curr_value;
    }

// Definitions related to the CPU latency QoS.
pub static mut pm_qos_constraints: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_value_invalid(value: i32) -> bool {
    return value < 0 && value != PM_QOS_DEFAULT_VALUE;
    }
//
// cpu_latency_qos_limit - Return current system-wide CPU latency QoS limit.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_limit() -> i32 {
    return pm_qos_read_value(&cpu_latency_constraints);
    }
//
// cpu_latency_qos_request_active - Check the given PM QoS request.
// @req: PM QoS request to check.
//
// Return: 'true' if @req has been added to the CPU latency QoS list, 'false'
// otherwise.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_request_active(req: *mut pm_qos_request) -> bool {
    return req.qos == &cpu_latency_constraints;
    }
    EXPORT_SYMBOL_GPL(cpu_latency_qos_request_active);
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_apply(req: *mut pm_qos_request, action: pm_qos_req_action, value: s32) {
pub static mut ret: c_int = 0;
    if (ret > 0) {
    wake_up_all_idle_cpus();
    }
    }
//
// cpu_latency_qos_add_request - Add new CPU latency QoS request.
// @req: Pointer to a preallocated handle.
// @value: Requested constraint value.
//
// Use @value to initialize the request handle pointed to by @req, insert it as
// a new entry to the CPU latency QoS list and recompute the effective QoS
// constraint for that list.
//
// Callers need to save the handle for later use in updates and removal of the
// QoS request represented by it.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_add_request(req: *mut pm_qos_request, value: i32) {
    if (!req || cpu_latency_qos_value_invalid(value)) {
    return;
    }
    if (cpu_latency_qos_request_active(req)) {
    WARN(1, "%s called for already added request\n", __func__);
    return;
    }
    trace_pm_qos_add_request(value);
    req.qos = &cpu_latency_constraints;
    cpu_latency_qos_apply(req, PM_QOS_ADD_REQ, value);
    }
    EXPORT_SYMBOL_GPL(cpu_latency_qos_add_request);
//
// cpu_latency_qos_update_request - Modify existing CPU latency QoS request.
// @req : QoS request to update.
// @new_value: New requested constraint value.
//
// Use @new_value to update the QoS request represented by @req in the CPU
// latency QoS list along with updating the effective constraint value for that
// list.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_update_request(req: *mut pm_qos_request, new_value: i32) {
    if (!req || cpu_latency_qos_value_invalid(new_value)) {
    return;
    }
    if (!cpu_latency_qos_request_active(req)) {
    WARN(1, "%s called for unknown object\n", __func__);
    return;
    }
    trace_pm_qos_update_request(new_value);
    if (new_value == req.node.prio) {
    return;
    }
    cpu_latency_qos_apply(req, PM_QOS_UPDATE_REQ, new_value);
    }
    EXPORT_SYMBOL_GPL(cpu_latency_qos_update_request);
//
// cpu_latency_qos_remove_request - Remove existing CPU latency QoS request.
// @req: QoS request to remove.
//
// Remove the CPU latency QoS request represented by @req from the CPU latency
// QoS list along with updating the effective constraint value for that list.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_remove_request(req: *mut pm_qos_request) {
    if (!req) {
    return;
    }
    if (!cpu_latency_qos_request_active(req)) {
    WARN(1, "%s called for unknown object\n", __func__);
    return;
    }
    trace_pm_qos_remove_request(PM_QOS_DEFAULT_VALUE);
    cpu_latency_qos_apply(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE);
    memset(req, 0, sizeof!(*req));
    }
    EXPORT_SYMBOL_GPL(cpu_latency_qos_remove_request);
// User space interface to the CPU latency QoS via misc device.
#[no_mangle]
unsafe extern "C" fn cpu_latency_qos_open(inode: *mut inode, filp: *mut file) -> c_int {
pub static mut req: *mut c_void = core::ptr::null_mut();
    req = kzalloc_obj(*req);
    if (!req) {
    return -ENOMEM;
    }
    cpu_latency_qos_add_request(req, PM_QOS_DEFAULT_VALUE);
    filp.private_data = req;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn cpu_latency_qos_release(inode: *mut inode, filp: *mut file) -> c_int {
    let mut req = filp.private_data;
    filp.private_data = core::ptr::null_mut();
    cpu_latency_qos_remove_request(req);
    kfree(req);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_read(filp: *mut file, buf: *mut c_char, count: size_t, f_pos: *mut loff_t) -> ssize_t {
    let mut req = filp.private_data;
    let mut flags = 0;
    let mut value = 0;
    if (!req || !cpu_latency_qos_request_active(req)) {
    return -EINVAL;
    }
    spin_lock_irqsave(&pm_qos_lock, flags);
    value = pm_qos_get_value(&cpu_latency_constraints);
    spin_unlock_irqrestore(&pm_qos_lock, flags);
    return simple_read_from_buffer(buf, count, f_pos, &value, sizeof!(s32));
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_latency_qos_write(filp: *mut file, buf: *mut c_char, count: size_t, f_pos: *mut loff_t) -> ssize_t {
    let mut value = 0;
    if (count == sizeof!(s32)) {
    if (copy_from_user(&value, buf, sizeof!(s32))) {
    return -EFAULT;
    }
    } else {
    let mut ret = 0;
    ret = kstrtos32_from_user(buf, count, 16, &value);
    if (ret) {
    return ret;
    }
    }
    cpu_latency_qos_update_request(filp.private_data, value);
    return count;
    }
pub static mut file_operations: usize = 0;
pub static mut miscdevice: usize = 0;

// The CPU system wakeup latency QoS.
pub static mut pm_qos_constraints: usize = 0;
//
// cpu_wakeup_latency_qos_limit - Current CPU system wakeup latency QoS limit.
//
// Returns the current CPU system wakeup latency QoS limit that may have been
// requested by user space.
//
#[no_mangle]
pub unsafe extern "C" fn cpu_wakeup_latency_qos_limit() -> i32 {
    return pm_qos_read_value(&cpu_wakeup_latency_constraints);
    }
#[no_mangle]
unsafe extern "C" fn cpu_wakeup_latency_qos_open(inode: *mut inode, filp: *mut file) -> c_int {
pub static mut req: *mut c_void = core::ptr::null_mut();
    req = kzalloc_obj(*req);
    if (!req) {
    return -ENOMEM;
    }
    req.qos = &cpu_wakeup_latency_constraints;
    pm_qos_update_target(req.qos, &req.node, PM_QOS_ADD_REQ,
    PM_QOS_RESUME_LATENCY_NO_CONSTRAINT);
    filp.private_data = req;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_wakeup_latency_qos_release(inode: *mut inode, filp: *mut file) -> c_int {
    let mut req = filp.private_data;
    filp.private_data = core::ptr::null_mut();
    pm_qos_update_target(req.qos, &req.node, PM_QOS_REMOVE_REQ,
    PM_QOS_RESUME_LATENCY_NO_CONSTRAINT);
    kfree(req);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_wakeup_latency_qos_read(filp: *mut file, buf: *mut c_char, count: size_t, f_pos: *mut loff_t) -> ssize_t {
pub static mut value: i32 = 0;
    return simple_read_from_buffer(buf, count, f_pos, &value, sizeof!(s32));
    }
#[no_mangle]
pub unsafe extern "C" fn cpu_wakeup_latency_qos_write(filp: *mut file, buf: *mut c_char, count: size_t, f_pos: *mut loff_t) -> ssize_t {
    let mut req = filp.private_data;
    let mut value = 0;
    if (count == sizeof!(s32)) {
    if (copy_from_user(&value, buf, sizeof!(s32))) {
    return -EFAULT;
    }
    } else {
    let mut ret = 0;
    ret = kstrtos32_from_user(buf, count, 16, &value);
    if (ret) {
    return ret;
    }
    }
    if (value < 0) {
    return -EINVAL;
    }
    pm_qos_update_target(req.qos, &req.node, PM_QOS_UPDATE_REQ, value);
    return count;
    }
pub static mut file_operations: usize = 0;
pub static mut miscdevice: usize = 0;

#[no_mangle]
unsafe extern "C" fn cpu_latency_qos_init() -> c_int {
    let mut ret = 0;
    ret = misc_register(&cpu_latency_qos_miscdev);
    if (ret < 0) {
    pr_err!("%s: %s setup failed\n", __func__,
    cpu_latency_qos_miscdev.name);
    return ret;
    }

    ret = misc_register(&cpu_wakeup_latency_qos_miscdev);
    if (ret < 0) {
    pr_err!("%s: %s setup failed\n", __func__,
    cpu_wakeup_latency_qos_miscdev.name);
    misc_deregister(&cpu_latency_qos_miscdev);
    return ret;
    }

    return 0;
    }
    late_initcall!(cpu_latency_qos_init);

// Definitions related to the frequency QoS below.
#[no_mangle]
pub unsafe extern "C" fn freq_qos_value_invalid(value: i32) -> bool {
    return value < 0 && value != PM_QOS_DEFAULT_VALUE;
    }
//
// freq_constraints_init - Initialize frequency QoS constraints.
// @qos: Frequency QoS constraints to initialize.
//
#[no_mangle]
pub unsafe extern "C" fn freq_constraints_init(qos: *mut freq_constraints) {
pub static mut c: *mut c_void = core::ptr::null_mut();
    c = &qos.min_freq;
    plist_head_init(&c.list);
    c.target_value = FREQ_QOS_MIN_DEFAULT_VALUE;
    c.default_value = FREQ_QOS_MIN_DEFAULT_VALUE;
    c.no_constraint_value = FREQ_QOS_MIN_DEFAULT_VALUE;
    c.type = PM_QOS_MAX;
    c.notifiers = &qos.min_freq_notifiers;
    BLOCKING_INIT_NOTIFIER_HEAD(c.notifiers);
    c = &qos.max_freq;
    plist_head_init(&c.list);
    c.target_value = FREQ_QOS_MAX_DEFAULT_VALUE;
    c.default_value = FREQ_QOS_MAX_DEFAULT_VALUE;
    c.no_constraint_value = FREQ_QOS_MAX_DEFAULT_VALUE;
    c.type = PM_QOS_MIN;
    c.notifiers = &qos.max_freq_notifiers;
    BLOCKING_INIT_NOTIFIER_HEAD(c.notifiers);
    }
//
// freq_qos_read_value - Get frequency QoS constraint for a given list.
// @qos: Constraints to evaluate.
// @type: QoS request type.
//
#[no_mangle]
pub unsafe extern "C" fn freq_qos_read_value(qos: *mut freq_constraints, type: freq_qos_req_type) -> s32 {
    let mut ret = 0;
    match (type) {
    FREQ_QOS_MIN => {
    ret = IS_ERR_OR_NULL(qos) ?
    FREQ_QOS_MIN_DEFAULT_VALUE :
    pm_qos_read_value(&qos.min_freq);
    // break;
    }
    FREQ_QOS_MAX => {
    ret = IS_ERR_OR_NULL(qos) ?
    FREQ_QOS_MAX_DEFAULT_VALUE :
    pm_qos_read_value(&qos.max_freq);
    // break;
    }
    _ => {
    WARN_ON!(1);
    ret = 0;
    }
    }
    return ret;
    }
//
// freq_qos_apply - Add/modify/remove frequency QoS request.
// @req: Constraint request to apply.
// @action: Action to perform (add/update/remove).
// @value: Value to assign to the QoS request.
//
// This is only meant to be called from inside pm_qos, not drivers.
//
#[no_mangle]
pub unsafe extern "C" fn freq_qos_apply(req: *mut freq_qos_request, action: pm_qos_req_action, value: s32) -> c_int {
    let mut ret = 0;
    match (req.type) {
    FREQ_QOS_MIN => {
    ret = pm_qos_update_target(&req.qos.min_freq, &req.pnode,
    action, value);
    // break;
    }
    FREQ_QOS_MAX => {
    ret = pm_qos_update_target(&req.qos.max_freq, &req.pnode,
    action, value);
    // break;
    }
    _ => {
    ret = -EINVAL;
    }
    }
    return ret;
    }
//
// freq_qos_add_request - Insert new frequency QoS request into a given list.
// @qos: Constraints to update.
// @req: Preallocated request object.
// @type: Request type.
// @value: Request value.
//
// Insert a new entry into the @qos list of requests, recompute the effective
// QoS constraint value for that list and initialize the @req object.  The
// caller needs to save that object for later use in updates and removal.
//
// Return 1 if the effective constraint value has changed, 0 if the effective
// constraint value has not changed, or a negative error code on failures.
//
#[no_mangle]
pub unsafe extern "C" fn freq_qos_add_request(qos: *mut freq_constraints, req: *mut freq_qos_request, type: freq_qos_req_type, value: s32) -> c_int {
    let mut ret = 0;
    if (IS_ERR_OR_NULL(qos) || !req || freq_qos_value_invalid(value)) {
    return -EINVAL;
    }
    if (WARN(freq_qos_request_active(req),
    "%s() called for active request\n", __func__)) {
    return -EINVAL;
    }
    req.qos = qos;
    req.type = type;
    ret = freq_qos_apply(req, PM_QOS_ADD_REQ, value);
    if (ret < 0) {
    req.qos = core::ptr::null_mut();
    req.type = 0;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(freq_qos_add_request);
//
// freq_qos_update_request - Modify existing frequency QoS request.
// @req: Request to modify.
// @new_value: New request value.
//
// Update an existing frequency QoS request along with the effective constraint
// value for the list of requests it belongs to.
//
// Return 1 if the effective constraint value has changed, 0 if the effective
// constraint value has not changed, or a negative error code on failures.
//
#[no_mangle]
pub unsafe extern "C" fn freq_qos_update_request(req: *mut freq_qos_request, new_value: i32) -> c_int {
    if (!req || freq_qos_value_invalid(new_value)) {
    return -EINVAL;
    }
    if (WARN(!freq_qos_request_active(req),
    "%s() called for unknown object\n", __func__)) {
    return -EINVAL;
    }
    if (req.pnode.prio == new_value) {
    return 0;
    }
    return freq_qos_apply(req, PM_QOS_UPDATE_REQ, new_value);
    }
    EXPORT_SYMBOL_GPL(freq_qos_update_request);
//
// freq_qos_remove_request - Remove frequency QoS request from its list.
// @req: Request to remove.
//
// Remove the given frequency QoS request from the list of constraints it
// belongs to and recompute the effective constraint value for that list.
//
// Return 1 if the effective constraint value has changed, 0 if the effective
// constraint value has not changed, or a negative error code on failures.
//
#[no_mangle]
pub unsafe extern "C" fn freq_qos_remove_request(req: *mut freq_qos_request) -> c_int {
    let mut ret = 0;
    if (!req) {
    return -EINVAL;
    }
    if (WARN(!freq_qos_request_active(req),
    "%s() called for unknown object\n", __func__)) {
    return -EINVAL;
    }
    ret = freq_qos_apply(req, PM_QOS_REMOVE_REQ, PM_QOS_DEFAULT_VALUE);
    req.qos = core::ptr::null_mut();
    req.type = 0;
    return ret;
    }
    EXPORT_SYMBOL_GPL(freq_qos_remove_request);
//
// freq_qos_add_notifier - Add frequency QoS change notifier.
// @qos: List of requests to add the notifier to.
// @type: Request type.
// @notifier: Notifier block to add.
//
#[no_mangle]
pub unsafe extern "C" fn freq_qos_add_notifier(qos: *mut freq_constraints, type: freq_qos_req_type, notifier: *mut notifier_block) -> c_int {
    let mut ret = 0;
    if (IS_ERR_OR_NULL(qos) || !notifier) {
    return -EINVAL;
    }
    match (type) {
    FREQ_QOS_MIN => {
    ret = blocking_notifier_chain_register(qos.min_freq.notifiers,
    notifier);
    // break;
    }
    FREQ_QOS_MAX => {
    ret = blocking_notifier_chain_register(qos.max_freq.notifiers,
    notifier);
    // break;
    }
    _ => {
    WARN_ON!(1);
    ret = -EINVAL;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(freq_qos_add_notifier);
//
// freq_qos_remove_notifier - Remove frequency QoS change notifier.
// @qos: List of requests to remove the notifier from.
// @type: Request type.
// @notifier: Notifier block to remove.
//
#[no_mangle]
pub unsafe extern "C" fn freq_qos_remove_notifier(qos: *mut freq_constraints, type: freq_qos_req_type, notifier: *mut notifier_block) -> c_int {
    let mut ret = 0;
    if (IS_ERR_OR_NULL(qos) || !notifier) {
    return -EINVAL;
    }
    match (type) {
    FREQ_QOS_MIN => {
    ret = blocking_notifier_chain_unregister(qos.min_freq.notifiers,
    notifier);
    // break;
    }
    FREQ_QOS_MAX => {
    ret = blocking_notifier_chain_unregister(qos.max_freq.notifiers,
    notifier);
    // break;
    }
    _ => {
    WARN_ON!(1);
    ret = -EINVAL;
    }
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(freq_qos_remove_notifier);