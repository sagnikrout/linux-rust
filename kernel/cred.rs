//! Automatically rewritten from C to Rust
//! Source: kernel/cred.c
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
pub struct ctl_table_header { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_root { pub _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_table_set { pub _opaque: [u8; 0] }

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
pub struct proc_dir_entry { pub _opaque: [u8; 0] }

pub type pid_type = c_int;
pub type cpu_pm_event = c_int;
pub type spinlock_t = u32;
pub type raw_spinlock_t = u32;
pub type kernel_cap_t = u64;
pub type cap_user_header_t = *mut c_void;
pub type cap_user_data_t = *mut c_void;
pub type async_cookie_t = u64;
pub type atomic_long_t = core::sync::atomic::AtomicI64;


























// SPDX-License-Identifier: GPL-2.0-or-later
// Task credentials management - see Documentation/security/credentials.rst
//
// Copyright (C) 2008 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

    printk("[%-5.5s%5u] " FMT "\n",					
    current.comm, current.pid, ##__VA_ARGS__)

    do {									
    if (0) {
    no_printk("[%-5.5s%5u] " FMT "\n",			
    current.comm, current.pid, ##__VA_ARGS__);	
    }
    } while (0)

pub static mut cred_jar: *mut c_void = core::ptr::null_mut();
//
// The RCU callback to actually dispose of a set of credentials
//
#[no_mangle]
unsafe extern "C" fn put_cred_rcu(rcu: *mut rcu_head) {
    let mut cred = container_of!(rcu, cred, rcu);
    kdebug("put_cred_rcu(%p)", cred);
    if (atomic_long_read(&cred.usage) != 0) {
    panic("CRED: put_cred_rcu() sees %p with usage %ld\n",
    cred, atomic_long_read(&cred.usage));
    }
    security_cred_free(cred);
    key_put(cred.session_keyring);
    key_put(cred.process_keyring);
    key_put(cred.thread_keyring);
    key_put(cred.request_key_auth);
    if (cred.group_info) {
    put_group_info(cred.group_info);
    }
    free_uid(cred.user);
    if (cred.ucounts) {
    put_ucounts(cred.ucounts);
    }
    put_user_ns(cred.user_ns);
    kmem_cache_free(cred_jar, cred);
    }
//
// __put_cred - Destroy a set of credentials
// @cred: The record to release
//
// Destroy a set of credentials on which no references remain.
//
#[no_mangle]
pub unsafe extern "C" fn __put_cred(cred: *mut cred) {
    kdebug("__put_cred(%p{%ld})", cred,
    atomic_long_read(&cred.usage));
// BUG_ON;
// BUG_ON;
// BUG_ON;
    if (cred.non_rcu) {
    put_cred_rcu(&cred.rcu);
    }
    else {
    call_rcu(&cred.rcu, put_cred_rcu);
    }
    }
// EXPORT_SYMBOL;
//
// Clean up a task's credentials when it exits
//
#[no_mangle]
pub unsafe extern "C" fn exit_creds(tsk: *mut task_struct) {
    let mut real_cred = core::ptr::null_mut();
    let mut cred = core::ptr::null_mut();
    kdebug("exit_creds(%u,%p,%p,{%ld})", tsk.pid, tsk.real_cred, tsk.cred,
    atomic_long_read(&tsk.cred.usage));
    real_cred =  tsk.real_cred;
    tsk.real_cred = core::ptr::null_mut();
    cred =  tsk.cred;
    tsk.cred = core::ptr::null_mut();
    if (real_cred == cred) {
    put_cred_many(cred, 2);
    } else {
    put_cred(real_cred);
    put_cred(cred);
    }

    key_put(tsk.cached_requested_key);
    tsk.cached_requested_key = core::ptr::null_mut();

    }
//
// get_task_cred - Get another task's objective credentials
// @task: The task to query
//
// Get the objective credentials of a task, pinning them so that they can't go
// away.  Accessing a task's credentials directly is not permitted.
//
// The caller must also make sure task doesn't get deleted, either by holding a
// ref on task or by holding tasklist_lock to prevent it from being unlinked.
//
    const struct cred *get_task_cred(task_struct *task)
    {
    let mut cred = core::ptr::null_mut();
    rcu_read_lock();
    do {
    cred = __task_cred((task));
// BUG_ON;
    } while (!get_cred_rcu(cred));
    rcu_read_unlock();
    return cred;
    }
// EXPORT_SYMBOL;
//
// Allocate blank credentials, such that the credentials can be filled in at a
// later date without risk of ENOMEM.
//
#[no_mangle]
pub unsafe extern "C" fn cred_alloc_blank() {
    let mut new = core::ptr::null_mut();
    new = kmem_cache_zalloc(cred_jar, GFP_KERNEL);
    if (!new) {
    return core::ptr::null_mut();
    }
    atomic_long_set(&new.usage, 1);
    if (security_cred_alloc_blank(new, GFP_KERNEL_ACCOUNT) < 0) {
// goto;
    }
    return new;
// label;
    abort_creds(new);
    return core::ptr::null_mut();
    }
//
// prepare_creds - Prepare a new set of credentials for modification
//
// Prepare a new set of task credentials for modification.  A task's creds
// shouldn't generally be modified directly, therefore this function is used to
// prepare a new copy, which the caller then modifies and then commits by
// calling commit_creds().
//
// Preparation involves making a copy of the objective creds for modification.
//
// Returns a pointer to the new creds-to-be if successful, NULL otherwise.
//
// Call commit_creds() or abort_creds() to clean up.
//
#[no_mangle]
pub unsafe extern "C" fn prepare_creds() {
    let mut task = current;
    let mut old = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    new = kmem_cache_alloc(cred_jar, GFP_KERNEL);
    if (!new) {
    return core::ptr::null_mut();
    }
    kdebug("prepare_creds() alloc %p", new);
    old = task.cred;
    memcpy(new, old, sizeof!(cred));
    new.non_rcu = 0;
    atomic_long_set(&new.usage, 1);
    get_group_info(new.group_info);
    get_uid(new.user);
    get_user_ns(new.user_ns);

    key_get(new.session_keyring);
    key_get(new.process_keyring);
    key_get(new.thread_keyring);
    key_get(new.request_key_auth);

    new.security = core::ptr::null_mut();

    new.ucounts = get_ucounts(new.ucounts);
    if (!new.ucounts) {
// goto;
    }
    if (security_prepare_creds(new, old, GFP_KERNEL_ACCOUNT) < 0) {
// goto;
    }
    return new;
// label;
    abort_creds(new);
    return core::ptr::null_mut();
    }
// EXPORT_SYMBOL;
//
// Prepare credentials for current to perform an execve()
// - The caller must hold ->cred_guard_mutex
//
#[no_mangle]
pub unsafe extern "C" fn prepare_exec_creds() {
    let mut new = core::ptr::null_mut();
    new = prepare_creds();
    if (!new) {
    return new;
    }

// newly exec'd tasks don't get a thread keyring
    key_put(new.thread_keyring);
    new.thread_keyring = core::ptr::null_mut();
// inherit the session keyring; new process keyring
    key_put(new.process_keyring);
    new.process_keyring = core::ptr::null_mut();

    new.suid = new.fsuid = new.euid;
    new.sgid = new.fsgid = new.egid;
    return new;
    }
//
// Copy credentials for the new process created by fork()
//
// We share if we can, but under some circumstances we have to generate a new
// set.
//
// The new process gets the current process's subjective credentials as its
// objective and subjective credentials
//
#[no_mangle]
pub unsafe extern "C" fn copy_creds(p: *mut task_struct, clone_flags: u64) -> c_int {
    let mut new = core::ptr::null_mut();
    let mut ret = 0;

    p.cached_requested_key = core::ptr::null_mut();

    if (

    !p.cred.thread_keyring &&

    clone_flags & CLONE_THREAD
    ) {
    p.real_cred = get_cred_many(p.cred, 2);
    kdebug("share_creds(%p{%ld})",
    p.cred, atomic_long_read(&p.cred.usage));
    inc_rlimit_ucounts(task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1);
    get_cred_namespaces(p);
    return 0;
    }
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    if (clone_flags & CLONE_NEWUSER) {
    ret = create_user_ns(new);
    if (ret < 0) {
// goto;
    }
    ret = set_cred_ucounts(new);
    if (ret < 0) {
// goto;
    }
    }

// new threads get their own thread keyrings if their parent already
// had one
    if (new.thread_keyring) {
    key_put(new.thread_keyring);
    new.thread_keyring = core::ptr::null_mut();
    if (clone_flags & CLONE_THREAD) {
    install_thread_keyring_to_cred(new);
    }
    }
// The process keyring is only shared between the threads in a process;
// anything outside of those threads doesn't inherit.
//
    if (!(clone_flags & CLONE_THREAD)) {
    key_put(new.process_keyring);
    new.process_keyring = core::ptr::null_mut();
    }

    p.cred = p.real_cred = get_cred(new);
    inc_rlimit_ucounts(task_ucounts(p), UCOUNT_RLIMIT_NPROC, 1);
    get_cred_namespaces(p);
    return 0;
// label;
    put_cred(new);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cred_cap_issubset(set: *const cred, subset: *const cred) -> bool {
    let mut set_ns = set.user_ns;
    let mut subset_ns = subset.user_ns;
// If the two credentials are in the same user namespace see if
// the capabilities of subset are a subset of set.
//
    if (set_ns == subset_ns) {
    return cap_issubset(subset.cap_permitted, set.cap_permitted);
    }
// The credentials are in a different user namespaces
// therefore one is a subset of the other only if a set is an
// ancestor of subset and set->euid is owner of subset or one
// of subsets ancestors.
//
    while (subset_ns != &init_user_ns) {
    if ((set_ns == subset_ns.parent)  &&
    uid_eq(subset_ns.owner, set.euid)) {
    return true;
    }
    }
    return false;
    }
//
// commit_creds - Install new credentials upon the current task
// @new: The credentials to be assigned
//
// Install a new set of credentials to the current task, using RCU to replace
// the old set.  Both the objective and the subjective credentials pointers are
// updated.  This function may not be called if the subjective credentials are
// in an overridden state.
//
// This function eats the caller's reference to the new credentials.
//
// Always returns 0 thus allowing this function to be tail-called at the end
// of, say, sys_setgid().
//
#[no_mangle]
pub unsafe extern "C" fn commit_creds(new: *mut cred) -> c_int {
    let mut task = current;
    let mut old = task.real_cred;
    kdebug("commit_creds(%p{%ld})", new,
    atomic_long_read(&new.usage));
// BUG_ON;
// BUG_ON;
    get_cred(new); /* we will require a ref for the subj creds too */
// dumpability changes
    if (!uid_eq(old.euid, new.euid) ||
    !gid_eq(old.egid, new.egid) ||
    !uid_eq(old.fsuid, new.fsuid) ||
    !gid_eq(old.fsgid, new.fsgid) ||
    !cred_cap_issubset(old, new)) {
// mm-less tasks share init_task's exec_state
    if (task.mm) {
    task_exec_state_set_dumpable(suid_dumpable);
    }
    task.pdeath_signal = 0;
//
// If a task drops privileges and becomes nondumpable,
// the dumpability change must become visible before
// the credential change; otherwise, a __ptrace_may_access()
// racing with this change may be able to attach to a task it
// shouldn't be able to attach to (as if the task had dropped
// privileges without becoming nondumpable).
// Pairs with a read barrier in __ptrace_may_access().
//
    smp_wmb();
    }
// alter the thread keyring
    if (!uid_eq(new.fsuid, old.fsuid)) {
    key_fsuid_changed(new);
    }
    if (!gid_eq(new.fsgid, old.fsgid)) {
    key_fsgid_changed(new);
    }
// do it
// RLIMIT_NPROC limits on user->processes have already been checked
// in set_user().
//
    if (new.user != old.user || new.user_ns != old.user_ns) {
    inc_rlimit_ucounts(new.ucounts, UCOUNT_RLIMIT_NPROC, 1);
    }
    rcu_assign_pointer(task.real_cred, new);
    rcu_assign_pointer(task.cred, new);
    if (new.user != old.user || new.user_ns != old.user_ns) {
    dec_rlimit_ucounts(old.ucounts, UCOUNT_RLIMIT_NPROC, 1);
    }
    if (new.user_ns != old.user_ns) {
    switch_cred_namespaces(old, new);
    }
// send notifications
    if (!uid_eq(new.uid,   old.uid)  ||
    !uid_eq(new.euid,  old.euid) ||
    !uid_eq(new.suid,  old.suid) ||
    !uid_eq(new.fsuid, old.fsuid)) {
    proc_id_connector(task, PROC_EVENT_UID);
    }
    if (!gid_eq(new.gid,   old.gid)  ||
    !gid_eq(new.egid,  old.egid) ||
    !gid_eq(new.sgid,  old.sgid) ||
    !gid_eq(new.fsgid, old.fsgid)) {
    proc_id_connector(task, PROC_EVENT_GID);
    }
// release the old obj and subj refs both
    put_cred_many(old, 2);
    return 0;
    }
// EXPORT_SYMBOL;
//
// abort_creds - Discard a set of credentials and unlock the current task
// @new: The credentials that were going to be applied
//
// Discard a set of credentials that were under construction and unlock the
// current task.
//
#[no_mangle]
pub unsafe extern "C" fn abort_creds(new: *mut cred) {
    kdebug("abort_creds(%p{%ld})", new,
    atomic_long_read(&new.usage));
// BUG_ON;
    put_cred(new);
    }
// EXPORT_SYMBOL;
//
// cred_fscmp - Compare two credentials with respect to filesystem access.
// @a: The first credential
// @b: The second credential
//
// cred_cmp() will return zero if both credentials have the same
// fsuid, fsgid, and supplementary groups.  That is, if they will both
// provide the same access to files based on mode/uid/gid.
// If the credentials are different, then either -1 or 1 will
// be returned depending on whether @a comes before or after @b
// respectively in an arbitrary, but stable, ordering of credentials.
//
// Return: -1, 0, or 1 depending on comparison
//
#[no_mangle]
pub unsafe extern "C" fn cred_fscmp(a: *const cred, b: *const cred) -> c_int {
    let mut ga = core::ptr::null_mut();
    let mut gb = core::ptr::null_mut();
    let mut g = 0;
    if (a == b) {
    return 0;
    }
    if (uid_lt(a.fsuid, b.fsuid)) {
    return -1;
    }
    if (uid_gt(a.fsuid, b.fsuid)) {
    return 1;
    }
    if (gid_lt(a.fsgid, b.fsgid)) {
    return -1;
    }
    if (gid_gt(a.fsgid, b.fsgid)) {
    return 1;
    }
    ga = a.group_info;
    gb = b.group_info;
    if (ga == gb) {
    return 0;
    }
    if (ga == core::ptr::null_mut()) {
    return -1;
    }
    if (gb == core::ptr::null_mut()) {
    return 1;
    }
    if (ga.ngroups < gb.ngroups) {
    return -1;
    }
    if (ga.ngroups > gb.ngroups) {
    return 1;
    }
    while (g < ga.ngroups) {
    if (gid_lt(ga.gid[g], gb.gid[g])) {
    return -1;
    }
    if (gid_gt(ga.gid[g], gb.gid[g])) {
    return 1;
    }
    }
    return 0;
    }
// EXPORT_SYMBOL;
#[no_mangle]
pub unsafe extern "C" fn set_cred_ucounts(new: *mut cred) -> c_int {
    struct ucounts *new_ucounts, *old_ucounts = new.ucounts;
//
// This optimization is needed because alloc_ucounts() uses locks
// for table lookups.
//
    if (old_ucounts.ns == new.user_ns && uid_eq(old_ucounts.uid, new.uid)) {
    return 0;
    }
    if (!(new_ucounts = alloc_ucounts(new.user_ns, new.uid))) {
    return -EAGAIN;
    }
    new.ucounts = new_ucounts;
    put_ucounts(old_ucounts);
    return 0;
    }
//
// initialise the credentials stuff
//
#[no_mangle]
pub unsafe extern "C" fn cred_init() -> c_int {
// allocate a slab in which we can store credentials
    cred_jar = KMEM_CACHE(cred,
    SLAB_HWCACHE_ALIGN | SLAB_PANIC | SLAB_ACCOUNT);
    }
//
// prepare_kernel_cred - Prepare a set of credentials for a kernel service
// @daemon: A userspace daemon to be used as a reference
//
// Prepare a set of credentials for a kernel service.  This can then be used to
// override a task's own credentials so that work can be done on behalf of that
// task that requires a different subjective context.
//
// @daemon is used to provide a base cred, with the security data derived from
// that; if this is "&init_task", they'll be set to 0, no groups, full
// capabilities, and no keys.
//
// The caller may change these controls afterwards if desired.
//
// Returns the new credentials or NULL if out of memory.
//
#[no_mangle]
pub unsafe extern "C" fn prepare_kernel_cred() {
    let mut old = core::ptr::null_mut();
    let mut new = core::ptr::null_mut();
    if (WARN_ON_ONCE!(!daemon)) {
    return core::ptr::null_mut();
    }
    new = kmem_cache_alloc(cred_jar, GFP_KERNEL);
    if (!new) {
    return core::ptr::null_mut();
    }
    kdebug("prepare_kernel_cred() alloc %p", new);
    old = get_task_cred(daemon);
// new = *old;
    new.non_rcu = 0;
    atomic_long_set(&new.usage, 1);
    get_uid(new.user);
    get_user_ns(new.user_ns);
    get_group_info(new.group_info);

    new.session_keyring = core::ptr::null_mut();
    new.process_keyring = core::ptr::null_mut();
    new.thread_keyring = core::ptr::null_mut();
    new.request_key_auth = core::ptr::null_mut();
    new.jit_keyring = KEY_REQKEY_DEFL_THREAD_KEYRING;

    new.security = core::ptr::null_mut();

    new.ucounts = get_ucounts(new.ucounts);
    if (!new.ucounts) {
// goto;
    }
    if (security_prepare_creds(new, old, GFP_KERNEL_ACCOUNT) < 0) {
// goto;
    }
    put_cred(old);
    return new;
// label;
    put_cred(new);
    put_cred(old);
    return core::ptr::null_mut();
    }
// EXPORT_SYMBOL;
//
// set_security_override - Set the security ID in a set of credentials
// @new: The credentials to alter
// @secid: The LSM security ID to set
//
// Set the LSM security ID in a set of credentials so that the subjective
// security is overridden when an alternative set of credentials is used.
//
#[no_mangle]
pub unsafe extern "C" fn set_security_override(new: *mut cred, secid: u32) -> c_int {
    return security_kernel_act_as(new, secid);
    }
// EXPORT_SYMBOL;
//
// set_create_files_as - Set the LSM file create context in a set of credentials
// @new: The credentials to alter
// @inode: The inode to take the context from
//
// Change the LSM file creation context in a set of credentials to be the same
// as the object context of the specified inode, so that the new inodes have
// the same MAC context as that inode.
//
#[no_mangle]
pub unsafe extern "C" fn set_create_files_as(new: *mut cred, inode: *mut inode) -> c_int {
    if (!uid_valid(inode.i_uid) || !gid_valid(inode.i_gid)) {
    return -EINVAL;
    }
    new.fsuid = inode.i_uid;
    new.fsgid = inode.i_gid;
    return security_kernel_create_files_as(new, inode);
    }
// EXPORT_SYMBOL;