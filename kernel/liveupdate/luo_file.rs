//! Automatically rewritten from C to Rust
//! Source: kernel/liveupdate/luo_file.c
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
// DOC: LUO File Descriptors
//
// LUO provides the infrastructure to preserve specific, stateful file
// descriptors across a kexec-based live update. The primary goal is to allow
// workloads, such as virtual machines using vfio, memfd, or iommufd, to
// retain access to their essential resources without interruption.
//
// The framework is built around a callback-based handler model and a well-
// defined lifecycle for each preserved file.
//
// Handler Registration:
// Kernel modules responsible for a specific file type (e.g., memfd, vfio)
// register a &struct liveupdate_file_handler. This handler provides a set of
// callbacks that LUO invokes at different stages of the update process, most
// notably:
//
// - can_preserve(): A lightweight check to determine if the handler is
// compatible with a given 'struct file'.
// - preserve(): The heavyweight operation that saves the file's state and
// returns an opaque u64 handle. This is typically performed while the
// workload is still active to minimize the downtime during the
// actual reboot transition.
// - unpreserve(): Cleans up any resources allocated by .preserve(), called
// if the preservation process is aborted before the reboot (i.e. session is
// closed).
// - freeze(): A final pre-reboot opportunity to prepare the state for kexec.
// We are already in reboot syscall, and therefore userspace cannot mutate
// the file anymore.
// - unfreeze(): Undoes the actions of .freeze(), called if the live update
// is aborted after the freeze phase.
// - retrieve(): Reconstructs the file in the new kernel from the preserved
// handle.
// - finish(): Performs final check and cleanup in the new kernel. After
// succesul finish call, LUO gives up ownership to this file.
//
// File Preservation Lifecycle happy path:
//
// 1. Preserve (Normal Operation): A userspace agent preserves files one by one
// via an ioctl. For each file, luo_preserve_file() finds a compatible
// handler, calls its .preserve() operation, and creates an internal &struct
// luo_file to track the live state.
//
// 2. Freeze (Pre-Reboot): Just before the kexec, luo_file_freeze() is called.
// It iterates through all preserved files, calls their respective .freeze()
// operation, and serializes their final metadata (compatible string, token,
// and data handle) into a contiguous memory block for KHO.
//
// 3. Deserialize: After kexec, luo_file_deserialize() runs when session gets
// deserialized (which is when /dev/liveupdate is first opened). It reads the
// serialized data from the KHO memory region and reconstructs the in-memory
// list of &struct luo_file instances for the new kernel, linking them to
// their corresponding handlers.
//
// 4. Retrieve (New Kernel - Userspace Ready): The userspace agent can now
// restore file descriptors by providing a token. luo_retrieve_file()
// searches for the matching token, calls the handler's .retrieve() op to
// re-create the 'struct file', and returns a new FD. Files can be
// retrieved in ANY order.
//
// 5. Finish (New Kernel - Cleanup): Once a session retrival is complete,
// luo_file_finish() is called. It iterates through all files, invokes their
// .finish() operations for final cleanup, and releases all associated kernel
// resources.
//
// File Preservation Lifecycle unhappy paths:
//
// 1. Abort Before Reboot: If the userspace agent aborts the live update
// process before calling reboot (e.g., by closing the session file
// descriptor), the session's release handler calls
// luo_file_unpreserve_files(). This invokes the .unpreserve() callback on
// all preserved files, ensuring all allocated resources are cleaned up and
// returning the system to a clean state.
//
// 2. Freeze Failure: During the reboot() syscall, if any handler's .freeze()
// op fails, the .unfreeze() op is invoked on all previously *successful
// freezes to roll back their state. The reboot() syscall then returns an
// error to userspace, canceling the live update.
//
// 3. Finish Failure: In the new kernel, if a handler's .finish() op fails,
// the luo_file_finish() operation is aborted. LUO retains ownership of
// all files within that session, including those that were not yet
// processed. The userspace agent can attempt to call the finish operation
// again later. If the issue cannot be resolved, these resources will be held
// by LUO until the next live update cycle, at which point they will be
// discarded.
//

pub static mut luo_file_handler_list: usize = 0;
// Keep track of files being preserved by LUO
pub static mut luo_preserved_files: usize = 0;
//
// struct luo_file - Represents a single preserved file instance.
// @fh:            Pointer to the &struct liveupdate_file_handler that manages
// this type of file.
// @file:          Pointer to the kernel's &struct file that is being preserved.
// This is NULL in the new kernel until the file is successfully
// retrieved.
// @serialized_data: The opaque u64 handle to the serialized state of the file.
// This handle is passed back to the handler's .freeze(),
// .retrieve(), and .finish() callbacks, allowing it to track
// and update its serialized state across phases.
// @private_data:  Pointer to the private data for the file used to hold runtime
// state that is not preserved. Set by the handler's .preserve()
// callback, and must be freed in the handler's .unpreserve()
// callback.
// @retrieve_status: Status code indicating whether a user/kernel in the new kernel has
// successfully called retrieve() on this file. This prevents
// multiple retrieval attempts. A value of 0 means a retrieve()
// has not been attempted, a positive value means the retrieve()
// was successful, and a negative value means the retrieve()
// failed, and the value is the error code of the call.
// @mutex:         A mutex that protects the fields of this specific instance
// (e.g., @retrieved, @file), ensuring that operations like
// retrieving or finishing a file are atomic.
// @list:          The list_head linking this instance into its parent
// file_set's list of preserved files.
// @token:         The user-provided unique token used to identify this file.
//
// This structure is the core in-kernel representation of a single file being
// managed through a live update. An instance is created by luo_preserve_file()
// to link a 'struct file' to its corresponding handler, a user-provided token,
and the serialized state handle returned by the handler's .preserve()
// operation.
//
// These instances are tracked in a per-file_set list. The @serialized_data
// field, which holds a handle to the file's serialized state, may be updated
// during the .freeze() callback before being serialized for the next kernel.
// After reboot, these structures are recreated by luo_file_deserialize() and
// are finally cleaned up by luo_file_finish().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct luo_file {
    pub fh: *mut liveupdate_file_handler,
    pub file: *mut file,
    pub serialized_data: u64,
    pub private_data: *mut c_void,
    pub retrieve_status: c_int,
    pub mutex: mutex,
    pub list: list_head,
    pub token: u64,
}

#[no_mangle]
pub unsafe extern "C" fn luo_get_id(fh: *mut liveupdate_file_handler, file: *mut file) -> c_ulong {
    return fh.ops.get_id ? fh.ops.get_id(file) : (unsigned long)file;
    }
#[no_mangle]
unsafe extern "C" fn luo_token_is_used(file_set: *mut luo_file_set, token: u64) -> bool {
pub static mut iter: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(iter, &file_set.files_list, list) {
    if (iter.token == token) {
    return true;
    }
    }
    return false;
    }
//
// luo_preserve_file - Initiate the preservation of a file descriptor.
// @file_set: The file_set to which the preserved file will be added.
// @token:    A unique, user-provided identifier for the file.
// @fd:       The file descriptor to be preserved.
//
// This function orchestrates the first phase of preserving a file. Upon entry,
// it takes a reference to the 'struct file' via fget(), effectively making LUO
// a co-owner of the file. This reference is held until the file is either
// unpreserved or successfully finished in the next kernel, preventing the file
// from being prematurely destroyed.
//
// This function orchestrates the first phase of preserving a file. It performs
// the following steps:
//
// 1. Validates that the @token is not already in use within the file_set.
// 2. Ensures the file_set's memory for files serialization is allocated
// (allocates if needed).
// 3. Iterates through registered handlers, calling can_preserve() to find one
// compatible with the given @fd.
// 4. Calls the handler's .preserve() operation, which saves the file's state
// and returns an opaque private data handle.
// 5. Adds the new instance to the file_set's internal list.
//
// On success, LUO takes a reference to the 'struct file' and considers it
// under its management until it is unpreserved or finished.
//
// In case of any failure, all intermediate allocations (file reference, memory
// for the 'luo_file' struct, etc.) are cleaned up before returning an error.
//
// Context: Can be called from an ioctl handler during normal system operation.
// Return: 0 on success. Returns a negative errno on failure:
// -EEXIST if the token is already used.
// -EBUSY if the file descriptor is already preserved by another session.
// -EBADF if the file descriptor is invalid.
// -ENOSPC if the file_set is full.
// -ENOENT if no compatible handler is found.
// -ENOMEM on memory allocation failure.
// Other erros might be returned by .preserve().
//
#[no_mangle]
pub unsafe extern "C" fn luo_preserve_file(file_set: *mut luo_file_set, token: u64, fd: c_int) -> c_int {
pub static mut args: liveupdate_file_op_args = 0;
pub static mut fh: *mut c_void = core::ptr::null_mut();
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
pub static mut file: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (luo_token_is_used(file_set, token)) {
    return -EEXIST;
    }
    err = kho_block_set_grow(&file_set.block_set, file_set.count + 1);
    if (err) {
    return err;
    }
    file = fget(fd);
    if (!file) {
    err = -EBADF;
// goto;
    }
    err = -ENOENT;
    down_read(&luo_register_rwlock);
    list_private_for_each_entry(fh, &luo_file_handler_list, list) {
    if (fh.ops.can_preserve(fh, file)) {
    if (try_module_get(fh.ops.owner)) {
    err = 0;
    }
    break;
    }
    }
    up_read(&luo_register_rwlock);
// err is still -ENOENT if no handler was found
    if (err) {
// goto;
    }
    err = xa_insert(&luo_preserved_files, luo_get_id(fh, file),
    file, GFP_KERNEL);
    if (err) {
// goto;
    }
    err = luo_flb_file_preserve(fh);
    if (err) {
// goto;
    }
    luo_file = kzalloc_obj(*luo_file);
    if (!luo_file) {
    err = -ENOMEM;
// goto;
    }
    luo_file.file = file;
    luo_file.fh = fh;
    luo_file.token = token;
    mutex_init(&luo_file.mutex);
    args.handler = fh;
    args.file = file;
    err = fh.ops.preserve(&args);
    if (err) {
// goto;
    }
    luo_file.serialized_data = args.serialized_data;
    luo_file.private_data = args.private_data;
    list_add_tail(&luo_file.list, &file_set.files_list);
    file_set.count += 1;
    return 0;
// label;
    kfree(luo_file);
// label;
    luo_flb_file_unpreserve(fh);
// label;
    xa_erase(&luo_preserved_files, luo_get_id(fh, file));
// label;
    module_put!(fh.ops.owner);
// label;
    fput(file);
// label;
    kho_block_set_shrink(&file_set.block_set, file_set.count);
    return err;
    }
//
// luo_file_unpreserve_files - Unpreserves all files from a file_set.
// @file_set: The files to be cleaned up.
//
// This function serves as the primary cleanup path for a file_set. It is
// invoked when the userspace agent closes the file_set's file descriptor.
//
// For each file, it performs the following cleanup actions:
// 1. Calls the handler's .unpreserve() callback to allow the handler to
// release any resources it allocated.
// 2. Removes the file from the file_set's internal tracking list.
// 3. Releases the reference to the 'struct file' that was taken by
// luo_preserve_file() via fput(), returning ownership.
// 4. Frees the memory associated with the internal 'struct luo_file'.
//
// After all individual files are unpreserved, it frees the contiguous memory
// block that was allocated to hold their serialization data.
//
#[no_mangle]
pub unsafe extern "C" fn luo_file_unpreserve_files(file_set: *mut luo_file_set) {
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
    while (!list_empty(&file_set.files_list)) {
pub static mut args: liveupdate_file_op_args = 0;
    luo_file = list_last_entry(&file_set.files_list, luo_file, list);
    args.handler = luo_file.fh;
    args.file = luo_file.file;
    args.serialized_data = luo_file.serialized_data;
    args.private_data = luo_file.private_data;
    luo_file.fh.ops.unpreserve(&args);
    luo_flb_file_unpreserve(luo_file.fh);
    xa_erase(&luo_preserved_files,
    luo_get_id(luo_file.fh, luo_file.file));
    module_put!(luo_file.fh.ops.owner);
    list_del(&luo_file.list);
    file_set.count -= 1;
    kho_block_set_shrink(&file_set.block_set, file_set.count);
    fput(luo_file.file);
    mutex_destroy(&luo_file.mutex);
    kfree(luo_file);
    }
    kho_block_set_destroy(&file_set.block_set);
    }
#[no_mangle]
pub unsafe extern "C" fn luo_file_freeze_one(file_set: *mut luo_file_set, luo_file: *mut luo_file) -> c_int {
pub static mut err: c_int = 0;
    guard(mutex)(&luo_file.mutex);
    if (luo_file.fh.ops.freeze) {
pub static mut args: liveupdate_file_op_args = 0;
    args.handler = luo_file.fh;
    args.file = luo_file.file;
    args.serialized_data = luo_file.serialized_data;
    args.private_data = luo_file.private_data;
    err = luo_file.fh.ops.freeze(&args);
    if (!err) {
    luo_file.serialized_data = args.serialized_data;
    }
    }
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_file_unfreeze_one(file_set: *mut luo_file_set, luo_file: *mut luo_file) {
    guard(mutex)(&luo_file.mutex);
    if (luo_file.fh.ops.unfreeze) {
pub static mut args: liveupdate_file_op_args = 0;
    args.handler = luo_file.fh;
    args.file = luo_file.file;
    args.serialized_data = luo_file.serialized_data;
    args.private_data = luo_file.private_data;
    luo_file.fh.ops.unfreeze(&args);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __luo_file_unfreeze(file_set: *mut luo_file_set, failed_entry: *mut luo_file) {
    let mut files_list = &file_set.files_list;
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
    list_for_each_entry(luo_file, files_list, list) {
    if (luo_file == failed_entry) {
    break;
    }
    luo_file_unfreeze_one(file_set, luo_file);
    }
    kho_block_set_clear(&file_set.block_set);
    }
//
// luo_file_freeze - Freezes all preserved files and serializes their metadata.
// @file_set:     The file_set whose files are to be frozen.
// @file_set_ser: Where to put the serialized file_set.
//
// This function is called from the reboot() syscall path, just before the
// kernel transitions to the new image via kexec. Its purpose is to perform the
// final preparation and serialization of all preserved files in the file_set.
//
// It iterates through each preserved file in FIFO order (the order of
// preservation) and performs two main actions:
//
// 1. Freezes the File: It calls the handler's .freeze() callback for each
// file. This gives the handler a final opportunity to quiesce the device or
// prepare its state for the upcoming reboot. The handler may update its
// private data handle during this step.
//
// 2. Serializes Metadata: After a successful freeze, it copies the final file
// metadata—the handler's compatible string, the user token, and the final
// private data handle—into the pre-allocated contiguous memory buffer
// (file_set->files) that will be handed over to the next kernel via KHO.
//
// Error Handling (Rollback):
// This function is atomic. If any handler's .freeze() operation fails, the
// entire live update is aborted. The __luo_file_unfreeze() helper is
// immediately called to invoke the .unfreeze() op on all files that were
// successfully frozen before the point of failure, rolling them back to a
// running state. The function then returns an error, causing the reboot()
// syscall to fail.
//
// Context: Called only from the liveupdate_reboot() path.
// Return: 0 on success, or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn luo_file_freeze(file_set: *mut luo_file_set, file_set_ser: *mut luo_file_set_ser) -> c_int {
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
    let mut err = 0;
    if (!file_set.count) {
    return 0;
    }
    kho_block_set_it_init(&it, &file_set.block_set);
    list_for_each_entry(luo_file, &file_set.files_list, list) {
    let mut file_ser = kho_block_set_it_reserve_entry(&it);
// This should not fail normally as blocks were pre-allocated
    if (WARN_ON_ONCE!(!file_ser)) {
    err = -ENOSPC;
// goto;
    }
    err = luo_file_freeze_one(file_set, luo_file);
    if (err < 0) {
    pr_warn!("Freeze failed for token[%#0llx] handler[%s] err[%pe]\n",
    luo_file.token, luo_file.fh.compatible,
    ERR_PTR(err));
// goto;
    }
    strscpy(file_ser.compatible, luo_file.fh.compatible,
    sizeof!(file_ser.compatible));
    file_ser.data = luo_file.serialized_data;
    file_ser.token = luo_file.token;
    }
    file_set_ser.count = file_set.count;
    file_set_ser.files = kho_block_set_head_pa(&file_set.block_set);
    return 0;
// label;
    __luo_file_unfreeze(file_set, luo_file);
    return err;
    }
//
// luo_file_unfreeze - Unfreezes all files in a file_set and clear serialization
// @file_set:     The file_set whose files are to be unfrozen.
// @file_set_ser: Serialized file_set.
//
// This function rolls back the state of all files in a file_set after the
// freeze phase has begun but must be aborted. It is the counterpart to
// luo_file_freeze().
//
// It invokes the __luo_file_unfreeze() helper with a NULL argument, which
// signals the helper to iterate through all files in the file_set and call
// their respective .unfreeze() handler callbacks.
//
// Context: This is called when the live update is aborted during
// the reboot() syscall, after luo_file_freeze() has been called.
//
#[no_mangle]
pub unsafe extern "C" fn luo_file_unfreeze(file_set: *mut luo_file_set, file_set_ser: *mut luo_file_set_ser) {
    if (!file_set.count) {
    return;
    }
    __luo_file_unfreeze(file_set, core::ptr::null_mut());
    memset(file_set_ser, 0, sizeof!(*file_set_ser));
    }
//
// luo_retrieve_file - Restores a preserved file from a file_set by its token.
// @file_set: The file_set from which to retrieve the file.
// @token:    The unique token identifying the file to be restored.
// @filep:    Output parameter; on success, this is populated with a pointer
// to the newly retrieved 'struct file'.
//
// This function is the primary mechanism for recreating a file in the new
// kernel after a live update. It searches the file_set's list of deserialized
// files for an entry matching the provided @token.
//
// The operation is idempotent: if a file has already been successfully
// retrieved, this function will simply return a pointer to the existing
// 'struct file' and report success without re-executing the retrieve
// operation. This is handled by checking the 'retrieved' flag under a lock.
//
// File retrieval can happen in any order; it is not bound by the order of
// preservation.
//
// Context: Can be called from an ioctl or other in-kernel code in the new
// kernel.
// Return: 0 on success. Returns a negative errno on failure:
// -ENOENT if no file with the matching token is found.
// Any error code returned by the handler's .retrieve() op.
//
#[no_mangle]
pub unsafe extern "C" fn luo_retrieve_file(file_set: *mut luo_file_set, token: u64, filep: *mut *mut file) -> c_int {
pub static mut args: liveupdate_file_op_args = 0;
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
pub static mut found: bool = false;
    let mut err = 0;
    if (list_empty(&file_set.files_list)) {
    return -ENOENT;
    }
    list_for_each_entry(luo_file, &file_set.files_list, list) {
    if (luo_file.token == token) {
    found = true;
    break;
    }
    }
    if (!found) {
    return -ENOENT;
    }
    guard(mutex)(&luo_file.mutex);
    if (luo_file.retrieve_status < 0) {
// Retrieve was attempted and it failed. Return the error code.
    return luo_file.retrieve_status;
    }
    if (luo_file.retrieve_status > 0) {
//
// Someone is asking for this file again, so get a reference
// for them.
//
    get_file(luo_file.file);
// filep = luo_file->file;
    return 0;
    }
    args.handler = luo_file.fh;
    args.serialized_data = luo_file.serialized_data;
    err = luo_file.fh.ops.retrieve(&args);
    if (err) {
// Keep the error code for later use.
    luo_file.retrieve_status = err;
    return err;
    }
    luo_file.file = args.file;
// Get reference so we can keep this file in LUO until finish
    get_file(luo_file.file);
    WARN_ON!(xa_insert(&luo_preserved_files,
    luo_get_id(luo_file.fh, luo_file.file),
    luo_file.file, GFP_KERNEL));
// filep = luo_file->file;
    luo_file.retrieve_status = 1;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_file_can_finish_one(file_set: *mut luo_file_set, luo_file: *mut luo_file) -> c_int {
pub static mut can_finish: bool = true;
    guard(mutex)(&luo_file.mutex);
    if (luo_file.fh.ops.can_finish) {
pub static mut args: liveupdate_file_op_args = 0;
    args.handler = luo_file.fh;
    args.file = luo_file.file;
    args.serialized_data = luo_file.serialized_data;
    args.retrieve_status = luo_file.retrieve_status;
    can_finish = luo_file.fh.ops.can_finish(&args);
    }
    return can_finish ? 0 : -EBUSY;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_file_finish_one(file_set: *mut luo_file_set, luo_file: *mut luo_file) {
pub static mut args: liveupdate_file_op_args = 0;
    guard(mutex)(&luo_file.mutex);
    args.handler = luo_file.fh;
    args.file = luo_file.file;
    args.serialized_data = luo_file.serialized_data;
    args.retrieve_status = luo_file.retrieve_status;
    luo_file.fh.ops.finish(&args);
    luo_flb_file_finish(luo_file.fh);
    }
//
// luo_file_finish - Completes the lifecycle for all files in a file_set.
// @file_set: The file_set to be finalized.
//
// This function orchestrates the final teardown of a live update file_set in
// the new kernel. It should be called after all necessary files have been
// retrieved and the userspace agent is ready to release the preserved state.
//
// The function iterates through all tracked files. For each file, it performs
// the following sequence of cleanup actions:
//
// 1. If file is not yet retrieved, retrieves it, and calls can_finish() on
// every file in the file_set. If all can_finish return true, continue to
// finish.
// 2. Calls the handler's .finish() callback (via luo_file_finish_one) to
// allow for final resource cleanup within the handler.
// 3. Releases LUO's ownership reference on the 'struct file' via fput(). This
// is the counterpart to the get_file() call in luo_retrieve_file().
// 4. Removes the 'struct luo_file' from the file_set's internal list.
// 5. Frees the memory for the 'struct luo_file' instance itself.
//
// After successfully finishing all individual files, it frees the
// contiguous memory block that was used to transfer the serialized metadata
// from the previous kernel.
//
// Error Handling (Atomic Failure):
// This operation is atomic. If any handler's .can_finish() op fails, the entire
// function aborts immediately and returns an error.
//
// Context: Can be called from an ioctl handler in the new kernel.
// Return: 0 on success, or a negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn luo_file_finish(file_set: *mut luo_file_set) -> c_int {
    let mut files_list = &file_set.files_list;
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!file_set.count) {
    return 0;
    }
    list_for_each_entry(luo_file, files_list, list) {
    err = luo_file_can_finish_one(file_set, luo_file);
    if (err) {
    return err;
    }
    }
    while (!list_empty(&file_set.files_list)) {
    luo_file = list_last_entry(&file_set.files_list, luo_file, list);
    luo_file_finish_one(file_set, luo_file);
    if (luo_file.file) {
    xa_erase(&luo_preserved_files,
    luo_get_id(luo_file.fh, luo_file.file));
    fput(luo_file.file);
    }
    module_put!(luo_file.fh.ops.owner);
    list_del(&luo_file.list);
    file_set.count -= 1;
    kho_block_set_shrink(&file_set.block_set, file_set.count);
    mutex_destroy(&luo_file.mutex);
    kfree(luo_file);
    }
    kho_block_set_destroy(&file_set.block_set);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_file_deserialize_one(file_set: *mut luo_file_set, ser: *mut luo_file_ser) -> c_int {
pub static mut fh: *mut c_void = core::ptr::null_mut();
pub static mut handler_found: bool = false;
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
    down_read(&luo_register_rwlock);
    list_private_for_each_entry(fh, &luo_file_handler_list, list) {
    if (!strcmp(fh.compatible, ser.compatible)) {
    if (try_module_get(fh.ops.owner)) {
    handler_found = true;
    }
    break;
    }
    }
    up_read(&luo_register_rwlock);
    if (!handler_found) {
    pr_warn!("No registered handler for compatible '%.*s'\n",
    (int)sizeof!(ser.compatible),
    ser.compatible);
    return -ENOENT;
    }
    luo_file = kzalloc_obj(*luo_file);
    if (!luo_file) {
    module_put!(fh.ops.owner);
    return -ENOMEM;
    }
    luo_file.fh = fh;
    luo_file.file = core::ptr::null_mut();
    luo_file.serialized_data = ser.data;
    luo_file.token = ser.token;
    mutex_init(&luo_file.mutex);
    list_add_tail(&luo_file.list, &file_set.files_list);
    return 0;
    }
//
// luo_file_deserialize - Reconstructs the list of preserved files in the new kernel.
// @file_set:     The incoming file_set to fill with deserialized data.
// @file_set_ser: Serialized KHO file_set data from the previous kernel.
//
// This function is called during the early boot process of the new kernel. It
// takes the raw, contiguous memory block of 'struct luo_file_ser' entries,
// provided by the previous kernel, and transforms it back into a live,
// in-memory linked list of 'struct luo_file' instances.
//
// For each serialized entry, it performs the following steps:
// 1. Reads the 'compatible' string.
// 2. Searches the global list of registered file handlers for one that
// matches the compatible string.
// 3. Allocates a new 'struct luo_file'.
// 4. Populates the new structure with the deserialized data (token, private
// data handle) and links it to the found handler. The 'file' pointer is
// initialized to NULL, as the file has not been retrieved yet.
// 5. Adds the new 'struct luo_file' to the file_set's files_list.
//
// This prepares the file_set for userspace, which can later call
// luo_retrieve_file() to restore the actual file descriptors.
//
// Context: Called from session deserialization.
//
#[no_mangle]
pub unsafe extern "C" fn luo_file_deserialize(file_set: *mut luo_file_set, file_set_ser: *mut luo_file_set_ser) -> c_int {
pub static mut file_ser: *mut c_void = core::ptr::null_mut();
pub static mut it: usize = 0;
    let mut err = 0;
    if (!file_set_ser.files) {
    WARN_ON!(file_set_ser.count);
    return 0;
    }
    file_set.count = 0;
    err = kho_block_set_restore(&file_set.block_set, file_set_ser.files);
    if (err) {
    return err;
    }
//
// Note on error handling:
//
// If deserialization fails (e.g., allocation failure or corrupt data),
// we intentionally skip cleanup of files that were already restored.
//
// A partial failure leaves the preserved state inconsistent.
// Implementing a safe "undo" to unwind complex dependencies (sessions,
// files, hardware state) is error-prone and provides little value, as
// the system is effectively in a broken state.
//
// We treat these resources as leaked. The expected recovery path is for
// userspace to detect the failure and trigger a reboot, which will
// reliably reset devices and reclaim memory.
//
    kho_block_set_it_init(&it, &file_set.block_set);
    while ((file_ser = kho_block_set_it_read_entry(&it))) {
    err = luo_file_deserialize_one(file_set, file_ser);
    if (err) {
// goto;
    }
    file_set.count += 1;
    }
    if (file_set.count != file_set_ser.count) {
    pr_warn!("File count mismatch: expected %llu, found %llu\n",
    file_set_ser.count, file_set.count);
    err = -EINVAL;
// goto;
    }
    return 0;
// label;
    while (!list_empty(&file_set.files_list)) {
pub static mut luo_file: *mut c_void = core::ptr::null_mut();
    luo_file = list_first_entry(&file_set.files_list, luo_file, list);
    list_del(&luo_file.list);
    module_put!(luo_file.fh.ops.owner);
    mutex_destroy(&luo_file.mutex);
    kfree(luo_file);
    }
    file_set.count = 0;
    kho_block_set_destroy(&file_set.block_set);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn luo_file_set_init(file_set: *mut luo_file_set) {
    INIT_LIST_HEAD(&file_set.files_list);
    kho_block_set_init(&file_set.block_set, sizeof!(luo_file_ser));
    }
#[no_mangle]
pub unsafe extern "C" fn luo_file_set_destroy(file_set: *mut luo_file_set) {
    WARN_ON!(file_set.count);
    WARN_ON!(!list_empty(&file_set.files_list));
    WARN_ON!(!kho_block_set_is_empty(&file_set.block_set));
    }
//
// liveupdate_register_file_handler - Register a file handler with LUO.
// @fh: Pointer to a caller-allocated &struct liveupdate_file_handler.
// The caller must initialize this structure, including a unique
// 'compatible' string and a valid 'fh' callbacks. This function adds the
// handler to the global list of supported file handlers.
//
// Context: Typically called during module initialization for file types that
// support live update preservation.
//
// Return: 0 on success. Negative errno on failure.
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_register_file_handler(fh: *mut liveupdate_file_handler) -> c_int {
pub static mut fh_iter: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    if (!liveupdate_enabled()) {
    return -EOPNOTSUPP;
    }
// Sanity check that all required callbacks are set
    if (!fh.ops.preserve || !fh.ops.unpreserve || !fh.ops.retrieve ||
    !fh.ops.finish || !fh.ops.can_preserve) {
    return -EINVAL;
    }
    down_write(&luo_register_rwlock);
// Check for duplicate compatible strings
    list_private_for_each_entry(fh_iter, &luo_file_handler_list, list) {
    if (!strcmp(fh_iter.compatible, fh.compatible)) {
    pr_err!("File handler registration failed: Compatible string '%s' already registered.\n",
    fh.compatible);
    err = -EEXIST;
// goto;
    }
    }
    INIT_LIST_HEAD(&ACCESS_PRIVATE(fh, flb_list));
    INIT_LIST_HEAD(&ACCESS_PRIVATE(fh, list));
    list_add_tail(&ACCESS_PRIVATE(fh, list), &luo_file_handler_list);
    up_write(&luo_register_rwlock);
    liveupdate_test_register(fh);
    return 0;
// label;
    up_write(&luo_register_rwlock);
    return err;
    }
//
// liveupdate_unregister_file_handler - Unregister a liveupdate file handler
// @fh: The file handler to unregister
//
// Unregisters the file handler from the liveupdate core. This function
// reverses the operations of liveupdate_register_file_handler().
//
#[no_mangle]
pub unsafe extern "C" fn liveupdate_unregister_file_handler(fh: *mut liveupdate_file_handler) {
    if (!liveupdate_enabled()) {
    return;
    }
    guard(rwsem_write)(&luo_register_rwlock);
    luo_flb_unregister_all(fh);
    list_del(&ACCESS_PRIVATE(fh, list));
    }