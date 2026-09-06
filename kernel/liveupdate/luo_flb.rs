//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/luo_flb.c
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
// Copyright (c) 2025, Google LLC.
// Pasha Tatashin <pasha.tatashin@soleen.com>
//
// DOC: LUO File Lifecycle Bound Global Data
//
// File-Lifecycle-Bound (FLB) objects provide a mechanism for managing global
// state that is shared across multiple live-updatable files. The lifecycle of
// this shared state is tied to the preservation of the files that depend on it.
//
// An FLB represents a global resource, such as the IOMMU core state, that is
// required by multiple file descriptors (e.g., all VFIO fds).
//
// The preservation of the FLB's state is triggered when the *first* file
// depending on it is preserved. The cleanup of this state (unpreserve or
// finish) is triggered when the *last* file depending on it is unpreserved or
// finished.
//
// Handler Dependency: A file handler declares its dependency on one or more
// FLBs by registering them via liveupdate_register_flb().
//
// Callback Model: Each FLB is defined by a set of operations
// (&struct liveupdate_flb_ops) that LUO invokes at key points:
//
// - .preserve(): Called for the first file. Saves global state.
// - .unpreserve(): Called for the last file (if aborted pre-reboot).
// - .retrieve(): Called on-demand in the new kernel to restore the state.
// - .finish(): Called for the last file in the new kernel for cleanup.
//
// This reference-counted approach ensures that shared state is saved exactly
// once and restored exactly once, regardless of how many files depend on it,
// and that its lifecycle is correctly managed across the kexec transition.
//

    sizeof!(luo_flb_header_ser)) / sizeof!(luo_flb_ser))
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_flb_header {
    pub header_ser: *mut luo_flb_header_ser,
    pub ser: *mut luo_flb_ser,
    pub active: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_flb_global {
    pub incoming: luo_flb_header,
    pub outgoing: luo_flb_header,
    pub list: list_head,
    pub count: c_long,
}

pub static mut luo_flb_global: usize = 0;
//
// struct luo_flb_link - Links an FLB definition to a file handler's internal
// list of dependencies.
// @flb:  A pointer to the registered &struct liveupdate_flb definition.
// @list: The list_head for linking.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_flb_link {
    pub flb: *mut liveupdate_flb,
    pub list: list_head,
}

// luo_flb_get_private - Access private field, and if needed initialize it.
#[no_mangle]
pub unsafe extern "C" fn luo_flb_get_private(flb: *mut liveupdate_flb) -> *mut c_void {
    let mut private = &ACCESS_PRIVATE(flb, private);
pub static mut luo_flb_init_lock: usize = 0;
    if (smp_load_acquire(&private.initialized)) {
    return private;
    }
    guard(spinlock)(&luo_flb_init_lock);
    if (!private.initialized) {
    mutex_init(&private.incoming.lock);
    mutex_init(&private.outgoing.lock);
    INIT_LIST_HEAD(&private.list);
    private.users = 0;
    smp_store_release(&private.initialized, true);
    }
    return private;
    }
#[no_mangle]
unsafe extern "C" fn luo_flb_file_preserve_one(flb: *mut liveupdate_flb) -> c_int {
    let mut private = luo_flb_get_private(flb);
    scoped_guard(mutex, &private.outgoing.lock) {
    if (!refcount_read(&private.outgoing.count)) {
pub static mut args: liveupdate_flb_op_args = 0;
    let mut err = 0;
    if (!try_module_get(flb.ops.owner)) {
    return -ENODEV;
    }
    args.flb = flb;
    err = flb.ops.preserve(&args);
    if (err) {
    module_put!(flb.ops.owner);
    return err;
    }
    private.outgoing.data = args.data;
    private.outgoing.obj = args.obj;
    refcount_set(&private.outgoing.count, 1);
    } else {
    refcount_inc(&private.outgoing.count);
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn liveupdate_flb_put_outgoing(flb: *mut liveupdate_flb) {
    let mut private = luo_flb_get_private(flb);
    scoped_guard(mutex, &private.outgoing.lock) {
    if (refcount_dec_and_test(&private.outgoing.count)) {
pub static mut args: liveupdate_flb_op_args = 0;
    args.flb = flb;
    args.data = private.outgoing.data;
    args.obj = private.outgoing.obj;
    if (flb.ops.unpreserve) {
    flb.ops.unpreserve(&args);
    }
    private.outgoing.data = 0;
    private.outgoing.obj = core::ptr::null_mut();
    module_put!(flb.ops.owner);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn luo_flb_retrieve_one(flb: *mut liveupdate_flb) -> c_int {
    let mut private = luo_flb_get_private(flb);
    let mut fh = &luo_flb_global.incoming;
pub static mut args: liveupdate_flb_op_args = 0;
pub static mut found: bool = false;
    let mut err = 0;
    lockdep_assert_held(&private.incoming.lock);
    if (private.incoming.finished) {
    return -ENODATA;
    }
    if (private.incoming.retrieve_status < 0) {
    return private.incoming.retrieve_status;
    }
    if (private.incoming.retrieve_status > 0) {
    return 0;
    }
    if (!fh.active) {
    return -ENODATA;
    }
    while (i < fh.header_ser.count) {
    if (!strcmp(fh.ser[i].name, flb.compatible)) {
    private.incoming.data = fh.ser[i].data;
    refcount_set(&private.incoming.count, fh.ser[i].count);
    found = true;
    break;
    }
    }
    if (!found) {
    return -ENOENT;
    }
    if (!try_module_get(flb.ops.owner)) {
    return -ENODEV;
    }
    args.flb = flb;
    args.data = private.incoming.data;
    err = flb.ops.retrieve(&args);
    if (err) {
    private.incoming.retrieve_status = err;
    module_put!(flb.ops.owner);
    return err;
    }
    private.incoming.obj = args.obj;
    private.incoming.retrieve_status = 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn liveupdate_flb_put_incoming(flb: *mut liveupdate_flb) {
    let mut private = luo_flb_get_private(flb);
pub static mut args: liveupdate_flb_op_args = 0;
    scoped_guard(mutex, &private.incoming.lock) {
    if (!refcount_dec_and_test(&private.incoming.count)) {
    return;
    }
    if (private.incoming.retrieve_status <= 0) {
pub static mut err: c_int = 0;
    if (WARN_ON!(err)) {
    return;
    }
    }
    args.flb = flb;
    args.obj = private.incoming.obj;
    flb.ops.finish(&args);
    private.incoming.data = 0;
    private.incoming.obj = core::ptr::null_mut();
    private.incoming.finished = true;
    module_put!(flb.ops.owner);
    }
    }
//
// luo_flb_file_preserve - Notifies FLBs that a file is about to be preserved.
// @fh: The file handler for the preserved file.
//
// This function iterates through all FLBs associated with the given file
// handler. It increments the reference count for each FLB. If the count becomes
// 1, it triggers the FLB's .preserve() callback to save the global state.
//
// This operation is atomic. If any FLB's .preserve() op fails, it will roll
// back by calling .unpreserve() on any FLBs that were successfully preserved
// during this call.
//
// Context: Called from luo_preserve_file()
// Return: 0 on success, or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn luo_flb_file_preserve(fh: *mut liveupdate_file_handler) -> c_int {
    let mut flb_list = &ACCESS_PRIVATE(fh, flb_list);
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    down_read(&luo_register_rwlock);
    list_for_each_entry(iter, flb_list, list) {
    err = luo_flb_file_preserve_one(iter.flb);
    if (err) {
// goto;
    }
    }
    up_read(&luo_register_rwlock);
    return 0;
// label;
    list_for_each_entry_continue_reverse(iter, flb_list, list) {
    liveupdate_flb_put_outgoing(iter.flb);
    }
    up_read(&luo_register_rwlock);
    return err;
    }
//
// luo_flb_file_unpreserve - Notifies FLBs that a dependent file was unpreserved.
// @fh: The file handler for the unpreserved file.
//
// This function iterates through all FLBs associated with the given file
// handler, in reverse order of registration. It decrements the reference count
// for each FLB. If the count becomes 0, it triggers the FLB's .unpreserve()
// callback to clean up the global state.
//
// Context: Called when a preserved file is being cleaned up before reboot
// (e.g., from luo_file_unpreserve_files()).
//
#[no_mangle]
pub unsafe extern "C" fn luo_flb_file_unpreserve(fh: *mut liveupdate_file_handler) {
    let mut flb_list = &ACCESS_PRIVATE(fh, flb_list);
pub static mut iter: *mut c_void = core::ptr::null_mut();
    guard(rwsem_read)(&luo_register_rwlock);
    list_for_each_entry_reverse(iter, flb_list, list) {
    liveupdate_flb_put_outgoing(iter.flb);
    }
    }
//
// luo_flb_file_finish - Notifies FLBs that a dependent file has been finished.
// @fh: The file handler for the finished file.
//
// This function iterates through all FLBs associated with the given file
// handler, in reverse order of registration. It decrements the incoming
// reference count for each FLB. If the count becomes 0, it triggers the FLB's
// .finish() callback for final cleanup in the new kernel.
//
// Context: Called from luo_file_finish() for each file being finished.
//
#[no_mangle]
pub unsafe extern "C" fn luo_flb_file_finish(fh: *mut liveupdate_file_handler) {
    let mut flb_list = &ACCESS_PRIVATE(fh, flb_list);
pub static mut iter: *mut c_void = core::ptr::null_mut();
    guard(rwsem_read)(&luo_register_rwlock);
    list_for_each_entry_reverse(iter, flb_list, list) {
    liveupdate_flb_put_incoming(iter.flb);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn luo_flb_unregister_one(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb) {
    let mut private = luo_flb_get_private(flb);
    let mut flb_list = &ACCESS_PRIVATE(fh, flb_list);
pub static mut iter: *mut c_void = core::ptr::null_mut();
pub static mut found: bool = false;
// Find and remove the link from the file handler's list
    list_for_each_entry(iter, flb_list, list) {
    if (iter.flb == flb) {
    list_del(&iter.list);
    kfree(iter);
    found = true;
    break;
    }
    }
    if (!found) {
    pr_warn!("Failed to unregister FLB '%s': not found in file handler '%s'\n",
    flb.compatible, fh.compatible);
    return;
    }
    private.users -= 1;
//
// If this is the last file-handler with which we are registred, remove
// from the global list.
//
    if (!private.users) {
    list_del_init(&private.list);
    luo_flb_global.count -= 1;
    }
    }
//
// luo_flb_unregister_all - Unregister all FLBs associated with a file handler.
// @fh: The file handler whose FLBs should be unregistered.
//
// This function iterates through the list of FLBs associated with the given
// file handler and unregisters them all one by one.
//
#[no_mangle]
pub unsafe extern "C" fn luo_flb_unregister_all(fh: *mut liveupdate_file_handler) {
    let mut flb_list = &ACCESS_PRIVATE(fh, flb_list);
    let mut iter = core::ptr::null_mut();
    let mut tmp = core::ptr::null_mut();
    if (!liveupdate_enabled()) {
    return;
    }
    lockdep_assert_held_write(&luo_register_rwlock);
    list_for_each_entry_safe(iter, tmp, flb_list, list) {
    luo_flb_unregister_one(fh, iter.flb);
    }
    }
//
// liveupdate_register_flb - Associate an FLB with a file handler and register it globally.
// @fh:   The file handler that will now depend on the FLB.
// @flb:  The File-Lifecycle-Bound object to associate.
//
// Establishes a dependency, informing the LUO core that whenever a file of
// type @fh is preserved, the state of @flb must also be managed.
//
// On the first registration of a given @flb object, it is added to a global
// registry. This function checks for duplicate registrations, both for a
// specific handler and globally, and ensures the total number of unique
// FLBs does not exceed the system limit.
//
// Context: Typically called from a subsystem's module init function after
// both the handler and the FLB have been defined and initialized.
// Return: 0 on success. Returns a negative errno on failure:
// -EINVAL if arguments are NULL or not initialized.
// -ENOMEM on memory allocation failure.
// -EEXIST if this FLB is already registered with this handler.
// -ENOSPC if the maximum number of global FLBs has been reached.
// -EOPNOTSUPP if live update is disabled or not configured.
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_register_flb(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb) -> c_int {
    let mut private = luo_flb_get_private(flb);
    let mut flb_list = &ACCESS_PRIVATE(fh, flb_list);
    struct luo_flb_link *link __free(kfree) = core::ptr::null_mut();
pub static mut gflb: *mut c_void = core::ptr::null_mut();
pub static mut iter: *mut c_void = core::ptr::null_mut();
    if (!liveupdate_enabled()) {
    return -EOPNOTSUPP;
    }
    if (WARN_ON!(!flb.ops.preserve || !flb.ops.unpreserve ||
    !flb.ops.retrieve || !flb.ops.finish)) {
    return -EINVAL;
    }
//
// File handler must already be registered, as it initializes the
// flb_list
//
    if (WARN_ON!(list_empty(&ACCESS_PRIVATE(fh, list)))) {
    return -EINVAL;
    }
    link = kzalloc_obj(*link);
    if (!link) {
    return -ENOMEM;
    }
    guard(rwsem_write)(&luo_register_rwlock);
// Check that this FLB is not already linked to this file handler
    list_for_each_entry(iter, flb_list, list) {
    if (iter.flb == flb) {
    return -EEXIST;
    }
    }
//
// If this FLB is not linked to global list it's the first time the FLB
// is registered
//
    if (!private.users) {
    if (WARN_ON!(!list_empty(&private.list))) {
    return -EINVAL;
    }
    if (luo_flb_global.count == LUO_FLB_MAX) {
    return -ENOSPC;
    }
// Check that compatible string is unique in global list
    list_private_for_each_entry(gflb, &luo_flb_global.list, private.list) {
    if (!strcmp(gflb.compatible, flb.compatible)) {
    return -EEXIST;
    }
    }
    list_add_tail(&private.list, &luo_flb_global.list);
    luo_flb_global.count += 1;
    }
// Finally, link the FLB to the file handler
    private.users += 1;
    link.flb = flb;
    list_add_tail(&no_free_ptr(link).list, flb_list);
    return 0;
    }
//
// liveupdate_unregister_flb - Remove an FLB dependency from a file handler.
// @fh:   The file handler that is currently depending on the FLB.
// @flb:  The File-Lifecycle-Bound object to remove.
//
// Removes the association between the specified file handler and the FLB
// previously established by liveupdate_register_flb().
//
// This function manages the global lifecycle of the FLB. It decrements the
// FLB's usage count. If this was the last file handler referencing this FLB,
// the FLB is removed from the global registry and the reference to its
// owner module (acquired during registration) is released.
//
// Context: It is typically called from a subsystem's module exit function.
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_unregister_flb(fh: *mut liveupdate_file_handler, flb: *mut liveupdate_flb) {
    if (!liveupdate_enabled()) {
    return;
    }
    guard(rwsem_write)(&luo_register_rwlock);
    luo_flb_unregister_one(fh, flb);
    }
//
// liveupdate_flb_get_incoming - Retrieve the incoming FLB object.
// @flb:  The FLB definition.
// @objp: Output parameter; will be populated with the live shared object.
//
// Returns a pointer to its shared live object for the incoming (post-reboot)
// path.
//
// If this is the first time the object is requested in the new kernel, this
// function will trigger the FLB's .retrieve() callback to reconstruct the
// object from its preserved state. Subsequent calls will return the same
// cached object.
//
// Return: 0 on success, or a negative errno on failure. -ENODATA means no
// incoming FLB data, -ENOENT means specific flb not found in the incoming
// data, -ENODEV if the FLB's module is unloading, and -EOPNOTSUPP when
// live update is disabled or not configured.
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_flb_get_incoming(flb: *mut liveupdate_flb, objp: *mut c_void) -> c_int {
    let mut private = luo_flb_get_private(flb);
    if (!liveupdate_enabled()) {
    return -EOPNOTSUPP;
    }
    guard(mutex)(&private.incoming.lock);
    if (!private.incoming.obj) {
pub static mut err: c_int = 0;
    if (err) {
    return err;
    }
    }
    refcount_inc(&private.incoming.count);
// objp = private->incoming.obj;
    return 0;
    }
//
// liveupdate_flb_get_outgoing - Retrieve the outgoing FLB object.
// @flb:  The FLB definition.
// @objp: Output parameter; will be populated with the live shared object.
//
// Returns a pointer to its shared live object for the outgoing (pre-reboot)
// path.
//
// This function assumes the object has already been created by the FLB's
// .preserve() callback, which is triggered when the first dependent file
// is preserved.
//
// Return: 0 on success, or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_flb_get_outgoing(flb: *mut liveupdate_flb, objp: *mut c_void) -> c_int {
    let mut private = luo_flb_get_private(flb);
    if (!liveupdate_enabled()) {
    return -EOPNOTSUPP;
    }
    guard(mutex)(&private.outgoing.lock);
    if (!private.outgoing.obj) {
    return -ENOENT;
    }
    refcount_inc(&private.outgoing.count);
// objp = private->outgoing.obj;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_flb_setup_outgoing(flbs_pa: *mut u64) -> c_int {
pub static mut header_ser: *mut c_void = core::ptr::null_mut();
    header_ser = kho_alloc_preserve(LUO_FLB_PGCNT << PAGE_SHIFT);
    if (IS_ERR(header_ser)) {
    return PTR_ERR(header_ser);
    }
// flbs_pa = virt_to_phys(header_ser);
    header_ser.pgcnt = LUO_FLB_PGCNT;
    luo_flb_global.outgoing.header_ser = header_ser;
    luo_flb_global.outgoing.ser = (header_ser + 1);
    luo_flb_global.outgoing.active = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_flb_setup_incoming(flbs_pa: u64)  {
pub static mut header_ser: *mut c_void = core::ptr::null_mut();
    if (!flbs_pa) {
    return;
    }
    header_ser = phys_to_virt(flbs_pa);
    luo_flb_global.incoming.header_ser = header_ser;
    luo_flb_global.incoming.ser = (header_ser + 1);
    luo_flb_global.incoming.active = true;
    }
//
// luo_flb_serialize - Serializes all active FLB objects for KHO.
//
// This function is called from the reboot path. It iterates through all
// registered File-Lifecycle-Bound (FLB) objects. For each FLB that has been
// preserved (i.e., its reference count is greater than zero), it writes its
// metadata into the memory region designated for Kexec Handover.
//
// The serialized data includes the FLB's compatibility string, its opaque
// data handle, and the final reference count. This allows the new kernel to
// find the appropriate handler and reconstruct the FLB's state.
//
// Context: Called from liveupdate_reboot() just before kho_finalize().
//
#[no_mangle]
pub unsafe extern "C" fn luo_flb_serialize() {
    let mut fh = &luo_flb_global.outgoing;
pub static mut gflb: *mut c_void = core::ptr::null_mut();
pub static mut i: c_int = 0;
    guard(rwsem_read)(&luo_register_rwlock);
    list_private_for_each_entry(gflb, &luo_flb_global.list, private.list) {
    let mut private = luo_flb_get_private(gflb);
pub static mut count: c_long = 0;
    if (count > 0) {
    strscpy(fh.ser[i].name, gflb.compatible,
    sizeof!(fh.ser[i].name));
    fh.ser[i].data = private.outgoing.data;
    fh.ser[i].count = count;
    i += 1;
    }
    }
    fh.header_ser.count = i;
    }