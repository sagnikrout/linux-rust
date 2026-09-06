//! Automatically rewritten from C to Rust
//! Source: kernel/module/dups.c
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
// kmod dups - the kernel module autoloader duplicate suppressor
//
// Copyright (C) 2023 Luis Chamberlain <mcgrof@kernel.org>
//

pub static mut enable_dups_trace: bool = false;
    module_param!(enable_dups_trace, bool_enable_only, 0644);
// A mutex-protected list of active kmod requests.
pub static mut kmod_dup_mutex: usize = 0;
pub static mut dup_kmod_reqs: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmod_dup_req {
    pub refcount: refcount_t,
    pub list: list_head,
    pub name: [c_char; MODULE_NAME_LEN],
    pub first_req_done: completion,
    pub delete_work: delayed_work,
    pub dup_ret: c_int,
}

#[no_mangle]
unsafe extern "C" fn get_kmod_req(kmod_req: *mut kmod_dup_req) {
    refcount_inc(&kmod_req.refcount);
    }
#[no_mangle]
unsafe extern "C" fn put_kmod_req(kmod_req: *mut kmod_dup_req) {
    if (refcount_dec_and_test(&kmod_req.refcount)) {
    kfree(kmod_req);
    }
    }
    DEFINE_FREE(put_kmod_req, kmod_dup_req *, if (_T) put_kmod_req(_T))
#[no_mangle]
pub unsafe extern "C" fn kmod_dup_request_lookup(module_name: *mut c_char) -> *mut c_void {
pub static mut kmod_req: *mut c_void = core::ptr::null_mut();
    lockdep_assert_held(&kmod_dup_mutex);
    list_for_each_entry(kmod_req, &dup_kmod_reqs, list) {
    if (!strcmp(kmod_req.name, module_name)) {
    return kmod_req;
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn kmod_dup_request_delete(work: *mut work_struct) {
pub static mut kmod_req: *mut c_void = core::ptr::null_mut();
    kmod_req = container_of!(to_delayed_work(work), kmod_dup_req, delete_work);
//
// The typical situation is a module successully loaded. In that
// situation the module will be present already in userspace. If
// new requests come in after that, userspace will already know the
// module is loaded so will just return 0 right away. There is still
// a small chance right after we delete this entry new request_module()
// calls may happen after that, they can happen. These heuristics
// are to protect finit_module() abuse for auto-loading, if modules
// are still tryign to auto-load even if a module is already loaded,
// that's on them, and those inneficiencies should not be fixed by
// kmod. The inneficies there are a call to modprobe and modprobe
// just returning 0.
//
    scoped_guard(mutex, &kmod_dup_mutex)
    list_del(&kmod_req.list);
    put_kmod_req(kmod_req);
    }
#[no_mangle]
pub unsafe extern "C" fn alloc_kmod_req(module_name: *mut c_char) -> *mut c_void {
    let mut kmod_req = kzalloc_obj(*kmod_req);
    if (!kmod_req) {
    return core::ptr::null_mut();
    }
    refcount_set(&kmod_req.refcount, 1);
    strscpy(kmod_req.name, module_name);
    INIT_DELAYED_WORK(&kmod_req.delete_work, kmod_dup_request_delete);
    init_completion(&kmod_req.first_req_done);
    return kmod_req;
    }
#[no_mangle]
pub unsafe extern "C" fn kmod_dup_request_exists_wait(module_name: *mut c_char, wait: bool, dup_ret: *mut c_int) -> bool {
    struct kmod_dup_req *kmod_req __free(put_kmod_req) = core::ptr::null_mut();
    let mut ret = 0;
    scoped_guard(mutex, &kmod_dup_mutex) {
pub static mut new_kmod_req: *mut c_void = core::ptr::null_mut();
    kmod_req = kmod_dup_request_lookup(module_name);
    if (kmod_req) {
    get_kmod_req(kmod_req);
    break;
    }
//
// If the first request that came through for a module
// was with request_module_nowait() we cannot wait for it
// and share its return value with other users which may
// have used request_module() and need a proper return value
// so just skip using them as an anchor.
//
// If a prior request to this one came through with
// request_module() though, then a request_module_nowait()
// would benefit from duplicate detection.
//
    if (!wait) {
    pr_debug!("New request_module_nowait() for %s -- cannot track duplicates for this request\n", module_name);
    return false;
    }
//
// There was no duplicate, just add the request so we can
// keep tab on duplicates later.
//
    pr_debug!("New request_module() for %s\n", module_name);
    new_kmod_req = alloc_kmod_req(module_name);
    if (!new_kmod_req) {
    return false;
    }
    list_add(&new_kmod_req.list, &dup_kmod_reqs);
    return false;
    }
// We are dealing with a duplicate request now
//
// To fix these try to use try_then_request_module() instead as that
// will check if the component you are looking for is present or not.
// You could also just queue a single request to load the module once,
// instead of having each and everything you need try to request for
// the module.
//
// Duplicate request_module() calls  can cause quite a bit of wasted
// vmalloc() space when racing with userspace.
//
    if (enable_dups_trace) {
    WARN(1, "module-autoload: duplicate request for module %s\n", module_name);
    }
    else {
    pr_warn!("module-autoload: duplicate request for module %s\n", module_name);
    }
    if (!wait) {
//
// If request_module_nowait() was used then the user just
// wanted to issue the request and if another module request
// was already its way with the same name we don't care for
// the return value either. Let duplicate request_module_nowait()
// calls bail out right away.
//
// dup_ret = 0;
    return true;
    }
//
// If a duplicate request_module() was used they *may* care for
// the return value, so we have no other option but to wait for
// the first caller to complete. If the first caller used
// the request_module_nowait() call, subsquent callers will
// deal with the comprmise of getting a successful call with this
// optimization enabled ...
//
    ret = wait_for_completion_state(&kmod_req.first_req_done,
    TASK_KILLABLE);
    if (ret) {
// dup_ret = ret;
    return true;
    }
// Now the duplicate request has the same exact return value as the first request
// dup_ret = kmod_req->dup_ret;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kmod_dup_request_announce(module_name: *mut c_char, ret: c_int) {
pub static mut kmod_req: *mut c_void = core::ptr::null_mut();
//
// Look for a kmod_dup_req previously added in
// kmod_dup_request_exists_wait(). Note that a request_module_nowait()
// without its own kmod_dup_req entry can announce a result of
// a concurrent request_module() call.
//
    scoped_guard(mutex, &kmod_dup_mutex) {
    kmod_req = kmod_dup_request_lookup(module_name);
    if (!kmod_req || completion_done(&kmod_req.first_req_done)) {
    return;
    }
    kmod_req.dup_ret = ret;
// Inform all duplicate waiters to check the return value.
    complete_all(&kmod_req.first_req_done);
    }
//
// Now that we have allowed prior request_module() calls to go on
// with life, let's schedule deleting this entry. We don't have
// to do it right away, but we *eventually* want to do it so to not
// let this linger forever as this is just a boot optimization for
// possible abuses of vmalloc() incurred by finit_module() thrashing.
//
    queue_delayed_work(system_dfl_wq, &kmod_req.delete_work, 60 * HZ);
    }