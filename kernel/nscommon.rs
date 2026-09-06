//! Automatically rewritten from C to Rust
//! Source: kernel/nscommon.c
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



// SPDX-License-Identifier: GPL-2.0-only
// Copyright (c) 2025 Christian Brauner <brauner@kernel.org>

#[no_mangle]
unsafe extern "C" fn ns_debug(ns: *mut ns_common, ops: *const proc_ns_operations) {
    match (ns.ns_type) {

    CLONE_NEWCGROUP => {
    VFS_WARN_ON_ONCE(ops != &cgroupns_operations);
    // break;

    }
    CLONE_NEWIPC => {
    VFS_WARN_ON_ONCE(ops != &ipcns_operations);
    // break;

    }
    CLONE_NEWNS => {
    VFS_WARN_ON_ONCE(ops != &mntns_operations);
    // break;

    }
    CLONE_NEWNET => {
    VFS_WARN_ON_ONCE(ops != &netns_operations);
    // break;

    }
    CLONE_NEWPID => {
    VFS_WARN_ON_ONCE(ops != &pidns_operations);
    // break;

    }
    CLONE_NEWTIME => {
    VFS_WARN_ON_ONCE(ops != &timens_operations);
    // break;

    }
    CLONE_NEWUSER => {
    VFS_WARN_ON_ONCE(ops != &userns_operations);
    // break;

    }
    CLONE_NEWUTS => {
    VFS_WARN_ON_ONCE(ops != &utsns_operations);
    // break;

    }
    }
    }

#[no_mangle]
pub unsafe extern "C" fn __ns_common_init(ns: *mut ns_common, ns_type: u32, ops: *const proc_ns_operations, inum: c_int) -> c_int {
pub static mut ret: c_int = 0;
    refcount_set(&ns.__ns_ref, 1);
    ns.stashed = core::ptr::null_mut();
    ns.ops = ops;
    ns.ns_id = 0;
    ns.ns_type = ns_type;
    ns_tree_node_init(&ns.ns_tree_node);
    ns_tree_node_init(&ns.ns_unified_node);
    ns_tree_node_init(&ns.ns_owner_node);
    ns_tree_root_init(&ns.ns_owner_root);

    ns_debug(ns, ops);

    if (inum) {
    ns.inum = inum;
    }
    else {
    ret = proc_alloc_inum(&ns.inum);
    }
    if (ret) {
    return ret;
    }
//
// Tree ref starts at 0. It's incremented when namespace enters
// active use (installed in nsproxy) and decremented when all
// active uses are gone. Initial namespaces are always active.
//
    if (is_ns_init_inum(ns)) {
    atomic_set(&ns.__ns_ref_active, 1);
    }
    else {
    atomic_set(&ns.__ns_ref_active, 0);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __ns_common_free(ns: *mut ns_common) {
    proc_free_inum(ns.inum);
    }
#[no_mangle]
pub unsafe extern "C" fn ns_owner(ns: *mut ns_common) -> *mut ns_common __must_check {
pub static mut owner: *mut c_void = core::ptr::null_mut();
    if (unlikely(!ns.ops)) {
    return core::ptr::null_mut();
    }
    VFS_WARN_ON_ONCE(!ns.ops.owner);
    owner = ns.ops.owner(ns);
    VFS_WARN_ON_ONCE(!owner && ns != to_ns_common(&init_user_ns));
    if (!owner) {
    return core::ptr::null_mut();
    }
// Skip init_user_ns as it's always active
    if (owner == &init_user_ns) {
    return core::ptr::null_mut();
    }
    return to_ns_common(owner);
    }
//
// The active reference count works by having each namespace that gets
// created take a single active reference on its owning user namespace.
// That single reference is only released once the child namespace's
// active count itself goes down.
//
// A regular namespace tree might look as follow:
// Legend:
// + : adding active reference
// - : dropping active reference
// x : always active (initial namespace)
//
// net_ns          pid_ns
// 
// +      +
// user_ns1 (2)
// |
// ipc_ns     |     uts_ns
// \    |
// +   +   +
// user_ns2 (3)
// |
// cgroup_ns       |       mnt_ns
// \      |
// x     x     x
// init_user_ns (1)
//
// If both net_ns and pid_ns put their last active reference on
// themselves it will cascade to user_ns1 dropping its own active
// reference and dropping one active reference on user_ns2:
//
// net_ns          pid_ns
// 
// -      -
// user_ns1 (0)
// |
// ipc_ns     |     uts_ns
// \    |
// +   -   +
// user_ns2 (2)
// |
// cgroup_ns       |       mnt_ns
// \      |
// x     x     x
// init_user_ns (1)
//
// The iteration stops once we reach a namespace that still has active
// references.
//
#[no_mangle]
pub unsafe extern "C" fn __ns_ref_active_put(ns: *mut ns_common) {
// Initial namespaces are always active.
    if (is_ns_init_id(ns)) {
    return;
    }
    if (!atomic_dec_and_test(&ns.__ns_ref_active)) {
    VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) < 0);
    return;
    }
    VFS_WARN_ON_ONCE(is_ns_init_id(ns));
    VFS_WARN_ON_ONCE(!__ns_ref_read(ns));
    for (;;) {
    ns = ns_owner(ns);
    if (!ns) {
    return;
    }
    VFS_WARN_ON_ONCE(is_ns_init_id(ns));
    if (!atomic_dec_and_test(&ns.__ns_ref_active)) {
    VFS_WARN_ON_ONCE(__ns_ref_active_read(ns) < 0);
    return;
    }
    }
    }
//
// The active reference count works by having each namespace that gets
// created take a single active reference on its owning user namespace.
// That single reference is only released once the child namespace's
// active count itself goes down. This makes it possible to efficiently
// resurrect a namespace tree:
//
// A regular namespace tree might look as follow:
// Legend:
// + : adding active reference
// - : dropping active reference
// x : always active (initial namespace)
//
// net_ns          pid_ns
// 
// +      +
// user_ns1 (2)
// |
// ipc_ns     |     uts_ns
// \    |
// +   +   +
// user_ns2 (3)
// |
// cgroup_ns       |       mnt_ns
// \      |
// x     x     x
// init_user_ns (1)
//
// If both net_ns and pid_ns put their last active reference on
// themselves it will cascade to user_ns1 dropping its own active
// reference and dropping one active reference on user_ns2:
//
// net_ns          pid_ns
// 
// -      -
// user_ns1 (0)
// |
// ipc_ns     |     uts_ns
// \    |
// +   -   +
// user_ns2 (2)
// |
// cgroup_ns       |       mnt_ns
// \      |
// x     x     x
// init_user_ns (1)
//
// Assume the whole tree is dead but all namespaces are still active:
//
// net_ns          pid_ns
// 
// -      -
// user_ns1 (0)
// |
// ipc_ns     |     uts_ns
// \    |
// -   -   -
// user_ns2 (0)
// |
// cgroup_ns       |       mnt_ns
// \      |
// x     x     x
// init_user_ns (1)
//
// Now assume the net_ns gets resurrected (.e.g., via the SIOCGSKNS ioctl()):
//
// net_ns          pid_ns
// 
// +      -
// user_ns1 (0)
// |
// ipc_ns     |     uts_ns
// \    |
// -   +   -
// user_ns2 (0)
// |
// cgroup_ns       |       mnt_ns
// \      |
// x     x     x
// init_user_ns (1)
//
// If net_ns had a zero reference count and we bumped it we also need to
// take another reference on its owning user namespace. Similarly, if
// pid_ns had a zero reference count it also needs to take another
// reference on its owning user namespace. So both net_ns and pid_ns
// will each have their own reference on the owning user namespace.
//
// If the owning user namespace user_ns1 had a zero reference count then
// it also needs to take another reference on its owning user namespace
// and so on.
//
#[no_mangle]
pub unsafe extern "C" fn __ns_ref_active_get(ns: *mut ns_common) {
    let mut prev = 0;
// Initial namespaces are always active.
    if (is_ns_init_id(ns)) {
    return;
    }
// If we didn't resurrect the namespace we're done.
    prev = atomic_fetch_add(1, &ns.__ns_ref_active);
    VFS_WARN_ON_ONCE(prev < 0);
    if (likely(prev)) {
    return;
    }
//
// We did resurrect it. Walk the ownership hierarchy upwards
// until we found an owning user namespace that is active.
//
    for (;;) {
    ns = ns_owner(ns);
    if (!ns) {
    return;
    }
    VFS_WARN_ON_ONCE(is_ns_init_id(ns));
    prev = atomic_fetch_add(1, &ns.__ns_ref_active);
    VFS_WARN_ON_ONCE(prev < 0);
    if (likely(prev)) {
    return;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn may_see_all_namespaces() -> bool {
    return (task_active_pid_ns(current) == &init_pid_ns) &&
    ns_capable_noaudit(init_pid_ns.user_ns, CAP_SYS_ADMIN);
    }