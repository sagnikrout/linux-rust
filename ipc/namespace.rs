//! Automatically rewritten from C to Rust
//! Source: ipc/namespace.c
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
macro_rules! printk { ($($tt:tt)*) => { 0 }; }
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
macro_rules! rootfs_initcall { ($($tt:tt)*) => {}; }
macro_rules! device_initcall { ($($tt:tt)*) => {}; }
macro_rules! late_initcall { ($($tt:tt)*) => {}; }
macro_rules! pure_initcall { ($($tt:tt)*) => {}; }
macro_rules! __setup { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_MUTEX { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_SPINLOCK { ($($tt:tt)*) => {}; }
macro_rules! DEFINE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DECLARE_PER_CPU { ($($tt:tt)*) => {}; }
macro_rules! DEFINE { ($($tt:tt)*) => {}; }
macro_rules! ARRAY_SIZE { ($($tt:tt)*) => { 1 }; }
macro_rules! min_t { ($($tt:tt)*) => { 0 }; }
macro_rules! max_t { ($($tt:tt)*) => { 0 }; }
macro_rules! container_of { ($($tt:tt)*) => { core::ptr::null_mut() }; }
macro_rules! sizeof { ($($tt:tt)*) => { 0usize }; }
macro_rules! MKDEV { ($($tt:tt)*) => { 0u32 }; }
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
macro_rules! pr_warn_once { ($($tt:tt)*) => {}; }
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
pub struct ipc_perm { pub uid: uid_t, pub gid: gid_t, pub mode: mode_t, pub key: key_t, pub cuid: uid_t, pub cgid: gid_t, pub seq: u32 }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipc_ids { pub _opaque: [u8; 0] }

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
pub type compat_uptr_t = u32;
pub type compat_long_t = i32;
pub type compat_ulong_t = u32;
pub type compat_size_t = u32;
pub type __compat_uid_t = u32;
pub type __compat_gid_t = u32;
pub type compat_mode_t = u32;
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
pub const ENOSYS: c_int = 38;
pub const EIDRM: c_int = 43;
pub const EOPNOTSUPP: c_int = 95;
pub const ENOTSUPP: c_int = 524;
pub const SHMLBA: usize = 4096;
pub const COMPAT_SHMLBA: usize = 4096;

// Standard File Mode Constants
pub const S_IFCHR: u32 = 0x2000;
pub const S_IFDIR: u32 = 0x4000;
pub const S_IFREG: u32 = 0x8000;
pub const S_IFBLK: u32 = 0x6000;
pub const S_IFIFO: u32 = 0x1000;
pub const S_IFLNK: u32 = 0xa000;
pub const S_IFSOCK: u32 = 0xc000;
pub const S_IRWXU: u32 = 0x01c0;
pub const S_IRUSR: u32 = 0x0100;
pub const S_IWUSR: u32 = 0x0080;
pub const S_IXUSR: u32 = 0x0040;
pub const S_IRUGO: u32 = 0x0124;
pub const S_IWUGO: u32 = 0x0092;
pub const S_IXUGO: u32 = 0x0049;

// Standard Memory Constants
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;
pub const GFP_KERNEL: c_uint = 0xcc0;
pub const GFP_ATOMIC: c_uint = 0x80000;
pub const GFP_NOWAIT: c_uint = 0;

// Standard Core Primitives
extern "C" {
    pub static current: *mut task_struct;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> bool;
    pub fn kmalloc(size: usize, flags: c_uint) -> *mut c_void;
    pub fn kfree(ptr: *mut c_void);
    pub fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    pub fn usermodehelper_enable();
    pub fn new_encode_dev(dev: u32) -> u32;
}

pub unsafe fn init_mkdir<T>(_path: T, _mode: u32) -> c_int { 0 }
pub unsafe fn init_mknod<T>(_path: T, _mode: u32, _dev: u32) -> c_int { 0 }
// === KERNEL_MACRO_PRELUDE_END ===



// SPDX-License-Identifier: GPL-2.0
//
// linux/ipc/namespace.c
// Copyright (C) 2006 Pavel Emelyanov <xemul@openvz.org> OpenVZ, SWsoft Inc.
//

//
// The work queue is used to avoid the cost of synchronize_rcu in kern_unmount.
//
// forward_decl: free_ipc;
pub static mut free_ipc_work: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn inc_ipc_namespaces(ns: *mut user_namespace) -> *mut c_void {
    return inc_ucount(ns, current_euid(), UCOUNT_IPC_NAMESPACES);
    }
#[no_mangle]
unsafe extern "C" fn dec_ipc_namespaces(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_IPC_NAMESPACES);
    }
#[no_mangle]
pub unsafe extern "C" fn create_ipc_ns(user_ns: *mut user_namespace, old_ns: *mut ipc_namespace) -> *mut c_void {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut ucounts: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    err = -ENOSPC;
    // label: again
    ucounts = inc_ipc_namespaces(user_ns);
    if (!ucounts) {
//
// IPC namespaces are freed asynchronously, by free_ipc_work.
// If frees were pending, flush_work will wait, and
// return true. Fail the allocation if no frees are pending.
//
    if (flush_work(&free_ipc_work)) {
// goto;
    }
// goto;
    }
    err = -ENOMEM;
    ns = kzalloc_obj(ipc_namespace, GFP_KERNEL_ACCOUNT);
    if (ns == core::ptr::null_mut()) {
// goto;
    }
    err = ns_common_init(ns);
    if (err) {
// goto;
    }
    ns_tree_gen_id(ns);
    ns.user_ns = get_user_ns(user_ns);
    ns.ucounts = ucounts;
    err = mq_init_ns(ns);
    if (err) {
// goto;
    }
    err = -ENOMEM;
    if (!setup_mq_sysctls(ns)) {
// goto;
    }
    if (!setup_ipc_sysctls(ns)) {
// goto;
    }
    err = msg_init_ns(ns);
    if (err) {
// goto;
    }
    sem_init_ns(ns);
    shm_init_ns(ns);
    ns_tree_add_raw(ns);
    return ns;
    // label: fail_ipc
    retire_ipc_sysctls(ns);
    // label: fail_mq_sysctls
    retire_mq_sysctls(ns);
    // label: fail_mq_mount
    mntput(ns.mq_mnt);
    // label: fail_put
    put_user_ns(ns.user_ns);
    ns_common_free(ns);
    // label: fail_free
    kfree(ns);
    // label: fail_dec
    dec_ipc_namespaces(ucounts);
    // label: fail
    return ERR_PTR(err);
    }
#[no_mangle]
pub unsafe extern "C" fn copy_ipcs(flags: u64, user_ns: *mut user_namespace, ns: *mut ipc_namespace) -> *mut c_void {
    if (!(flags & CLONE_NEWIPC)) {
    return get_ipc_ns(ns);
    }
    return create_ipc_ns(user_ns, ns);
    }
//
// free_ipcs - free all ipcs of one type
// @ns:   the namespace to remove the ipcs from
// @ids:  the table of ipcs to free
// @free: the function called to free each individual ipc
//
// Called for each kind of ipc when an ipc_namespace exits.
//
#[no_mangle]
pub unsafe extern "C" fn free_ipcs(ns: *mut ipc_namespace, ids: *mut ipc_ids, free: *mut c_void) {
pub static mut perm: *mut c_void = core::ptr::null_mut();
    let mut next_id = 0;
    let mut total = 0;
    let mut in_use = 0;
    down_write(&ids.rwsem);
    in_use = ids.in_use;
    while (total < in_use) {
    perm = idr_find(&ids.ipcs_idr, next_id);
    if (perm == core::ptr::null_mut()) {
    continue;
    }
    rcu_read_lock();
    ipc_lock_object(perm);
    free(ns, perm);
    total += 1;
    }
    up_write(&ids.rwsem);
    }
#[no_mangle]
unsafe extern "C" fn free_ipc_ns(ns: *mut ipc_namespace) {
//
// Caller needs to wait for an RCU grace period to have passed
// after making the mount point inaccessible to new accesses.
//
    mntput(ns.mq_mnt);
    sem_exit_ns(ns);
    msg_exit_ns(ns);
    shm_exit_ns(ns);
    retire_mq_sysctls(ns);
    retire_ipc_sysctls(ns);
    dec_ipc_namespaces(ns.ucounts);
    put_user_ns(ns.user_ns);
    ns_common_free(ns);
    kfree(ns);
    }
pub static mut free_ipc_list: usize = 0;
#[no_mangle]
unsafe extern "C" fn free_ipc(unused: *mut work_struct) {
    let mut node = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
    let mut t = core::ptr::null_mut();
    if false {
    if (ns_ref_put_and_lock(ns, &mq_lock)) {
    mq_clear_sbinfo(ns);
    spin_unlock(&mq_lock);
    ns_tree_remove(ns);
    if (llist_add(&ns.mnt_llist, &free_ipc_list)) {
    schedule_work(&free_ipc_work);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ipcns_get(task: *mut task_struct) -> *mut c_void {
    let mut ns = core::ptr::null_mut();
pub static mut nsproxy: *mut c_void = core::ptr::null_mut();
    task_lock(task);
    nsproxy = task.nsproxy;
    if (nsproxy) {
    ns = get_ipc_ns(nsproxy.ipc_ns);
    }
    task_unlock(task);
    return if ns { &ns.ns } else { core::ptr::null_mut() };
    }
#[no_mangle]
unsafe extern "C" fn ipcns_put(ns: *mut ns_common) {
    return put_ipc_ns(to_ipc_ns(ns));
    }
#[no_mangle]
unsafe extern "C" fn ipcns_install(nsset: *mut nsset, new: *mut ns_common) -> c_int {
    let mut nsproxy = core::ptr::null_mut();
    let mut ns = core::ptr::null_mut();
    if (!ns_capable(ns.user_ns, CAP_SYS_ADMIN) ||
    !ns_capable(nsset.cred.user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    put_ipc_ns(nsproxy.ipc_ns);
    nsproxy.ipc_ns = get_ipc_ns(ns);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ipcns_owner(ns: *mut ns_common) -> *mut c_void {
    return to_ipc_ns(ns).user_ns;
    }
pub static mut proc_ns_operations: usize = 0;
}