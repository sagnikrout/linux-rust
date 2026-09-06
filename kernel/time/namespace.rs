//! Automatically rewritten from C to Rust
//! Source: kernel/time/namespace.c
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
// Author: Andrei Vagin <avagin@openvz.org>
// Author: Dmitry Safonov <dima@arista.com>
//

    ktime_t do_timens_ktime_to_host(clockid_t clockid, ktime_t tim, timens_offsets *ns_offsets)
    {
    let mut offset;
    match (clockid) {
    CLOCK_MONOTONIC => {
    offset = timespec64_to_ktime(ns_offsets.monotonic);
    // break;
    }
    CLOCK_BOOTTIME => {
    }
    CLOCK_BOOTTIME_ALARM => {
    offset = timespec64_to_ktime(ns_offsets.boottime);
    // break;
    }
    _ => {
    return tim;
    }
    }
//
// Check that @tim value is in [offset, KTIME_MAX + offset]
// and subtract offset.
//
    if (tim < offset) {
//
// User can specify @tim *absolute* value - if it's lesser than
// the time namespace's offset - it's already expired.
//
    tim = 0;
    } else {
    tim = ktime_sub(tim, offset);
    if (unlikely(tim > KTIME_MAX)) {
    tim = KTIME_MAX;
    }
    }
    return tim;
    }
    EXPORT_SYMBOL_GPL(do_timens_ktime_to_host);
#[no_mangle]
pub unsafe extern "C" fn inc_time_namespaces(ns: *mut user_namespace) -> *mut c_void {
    return inc_ucount(ns, current_euid(), UCOUNT_TIME_NAMESPACES);
    }
#[no_mangle]
unsafe extern "C" fn dec_time_namespaces(ucounts: *mut ucounts) {
    dec_ucount(ucounts, UCOUNT_TIME_NAMESPACES);
    }
//
// clone_time_ns - Clone a time namespace
// @user_ns:	User namespace which owns a new namespace.
// @old_ns:	Namespace to clone
//
// Clone @old_ns and set the clone refcount to 1
//
// Return: The new namespace or ERR_PTR.
//
#[no_mangle]
pub unsafe extern "C" fn clone_time_ns(user_ns: *mut user_namespace, old_ns: *mut time_namespace) -> *mut c_void {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut ucounts: *mut c_void = core::ptr::null_mut();
    let mut err = 0;
    err = -ENOSPC;
    ucounts = inc_time_namespaces(user_ns);
    if (!ucounts) {
// goto;
    }
    err = -ENOMEM;
    ns = kzalloc_obj(*ns, GFP_KERNEL_ACCOUNT);
    if (!ns) {
// goto;
    }
    err = timens_vdso_alloc_vvar_page(ns);
    if (err) {
// goto;
    }
    err = ns_common_init(ns);
    if (err) {
// goto;
    }
    ns.ucounts = ucounts;
    ns.user_ns = get_user_ns(user_ns);
    ns.offsets = old_ns.offsets;
    ns.frozen_offsets = false;
    ns_tree_add(ns);
    return ns;
// label;
    timens_vdso_free_vvar_page(ns);
// label;
    kfree(ns);
// label;
    dec_time_namespaces(ucounts);
// label;
    return ERR_PTR(err);
    }
//
// copy_time_ns - Create timens_for_children from @old_ns
// @flags:	Cloning flags
// @user_ns:	User namespace which owns a new namespace.
// @old_ns:	Namespace to clone
//
// If CLONE_NEWTIME specified in @flags, creates a new timens_for_children;
// adds a refcounter to @old_ns otherwise.
//
// Return: timens_for_children namespace or ERR_PTR.
//
#[no_mangle]
pub unsafe extern "C" fn copy_time_ns(flags: u64, user_ns: *mut user_namespace, old_ns: *mut time_namespace) -> *mut c_void {
    if (!(flags & CLONE_NEWTIME)) {
    return get_time_ns(old_ns);
    }
    return clone_time_ns(user_ns, old_ns);
    }
pub static mut timens_offset_lock: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn free_time_ns(ns: *mut time_namespace) {
    ns_tree_remove(ns);
    dec_time_namespaces(ns.ucounts);
    put_user_ns(ns.user_ns);
    ns_common_free(ns);
    timens_vdso_free_vvar_page(ns);
// Concurrent nstree traversal depends on a grace period.
    kfree_rcu(ns, ns.ns_rcu);
    }
#[no_mangle]
pub unsafe extern "C" fn timens_get(task: *mut task_struct) -> *mut c_void {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut nsproxy: *mut c_void = core::ptr::null_mut();
    guard(task_lock)(task);
    nsproxy = task.nsproxy;
    if (!nsproxy) {
    return core::ptr::null_mut();
    }
    ns = nsproxy.time_ns;
    get_time_ns(ns);
    return &ns.ns;
    }
#[no_mangle]
pub unsafe extern "C" fn timens_for_children_get(task: *mut task_struct) -> *mut c_void {
pub static mut ns: *mut c_void = core::ptr::null_mut();
pub static mut nsproxy: *mut c_void = core::ptr::null_mut();
    guard(task_lock)(task);
    nsproxy = task.nsproxy;
    if (!nsproxy) {
    return core::ptr::null_mut();
    }
    ns = nsproxy.time_ns_for_children;
    get_time_ns(ns);
    return &ns.ns;
    }
#[no_mangle]
unsafe extern "C" fn timens_put(ns: *mut ns_common) {
    put_time_ns(to_time_ns(ns));
    }
#[no_mangle]
unsafe extern "C" fn timens_install(nsset: *mut nsset, new: *mut ns_common) -> c_int {
    let mut nsproxy = nsset.nsproxy;
    let mut ns = to_time_ns(new);
    if (!current_is_single_threaded()) {
    return -EUSERS;
    }
    if (!ns_capable(ns.user_ns, CAP_SYS_ADMIN) ||
    !ns_capable(nsset.cred.user_ns, CAP_SYS_ADMIN)) {
    return -EPERM;
    }
    get_time_ns(ns);
    put_time_ns(nsproxy.time_ns);
    nsproxy.time_ns = ns;
    get_time_ns(ns);
    put_time_ns(nsproxy.time_ns_for_children);
    nsproxy.time_ns_for_children = ns;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn timens_on_fork(nsproxy: *mut nsproxy, tsk: *mut task_struct) {
    let mut nsc = &nsproxy.time_ns_for_children.ns;
    let mut ns = to_time_ns(nsc);
// create_new_namespaces() already incremented the ref counter
    if (nsproxy.time_ns == nsproxy.time_ns_for_children) {
    return;
    }
    get_time_ns(ns);
    put_time_ns(nsproxy.time_ns);
    nsproxy.time_ns = ns;
    timens_commit(tsk, ns);
    }
#[no_mangle]
pub unsafe extern "C" fn timens_owner(ns: *mut ns_common) -> *mut c_void {
    return to_time_ns(ns).user_ns;
    }
#[no_mangle]
unsafe extern "C" fn show_offset(m: *mut seq_file, clockid: c_int, ts: *mut timespec64) {
pub static mut clock: *mut c_void = core::ptr::null_mut();
    match (clockid) {
    CLOCK_BOOTTIME => {
    clock = "boottime";
    // break;
    }
    CLOCK_MONOTONIC => {
    clock = "monotonic";
    // break;
    }
    _ => {
    clock = "unknown";
    // break;
    }
    }
    seq_printf(m, "%-10s %10lld %9ld\n", clock, ts.tv_sec, ts.tv_nsec);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_timens_show_offsets(p: *mut task_struct, m: *mut seq_file) {
    struct time_namespace *time_ns __free(time_ns) = core::ptr::null_mut();
    let mut ns = timens_for_children_get(p);
    if (!ns) {
    return;
    }
    time_ns = to_time_ns(ns);
    show_offset(m, CLOCK_MONOTONIC, &time_ns.offsets.monotonic);
    show_offset(m, CLOCK_BOOTTIME, &time_ns.offsets.boottime);
    }
#[no_mangle]
pub unsafe extern "C" fn proc_timens_set_offset(file: *mut file, p: *mut task_struct, offsets: *mut proc_timens_offset, noffsets: c_int) -> c_int {
    struct time_namespace *time_ns __free(time_ns) = core::ptr::null_mut();
    let mut ns = timens_for_children_get(p);
pub static mut tp: usize = 0;
    let mut i = 0;
    if (!ns) {
    return -ESRCH;
    }
    time_ns = to_time_ns(ns);
    if (!file_ns_capable(file, time_ns.user_ns, CAP_SYS_TIME)) {
    return -EPERM;
    }
    while (i < noffsets) {
    let mut off = &offsets[i];
    match (off.clockid) {
    CLOCK_MONOTONIC => {
    ktime_get_ts64(&tp);
    // break;
    }
    CLOCK_BOOTTIME => {
    ktime_get_boottime_ts64(&tp);
    // break;
    }
    _ => {
    return -EINVAL;
    }
    }
    if (off.val.tv_sec > KTIME_SEC_MAX || off.val.tv_sec < -KTIME_SEC_MAX) {
    return -ERANGE;
    }
    if (off.val.tv_nsec < 0 || off.val.tv_nsec >= NSEC_PER_SEC) {
    return -EINVAL;
    }
    tp = timespec64_add(tp, off.val);
//
// KTIME_SEC_MAX is divided by 2 to be sure that KTIME_MAX is
// still unreachable.
//
    if (tp.tv_sec < 0 || tp.tv_sec > KTIME_SEC_MAX / 2) {
    return -ERANGE;
    }
    }
    guard(mutex)(&timens_offset_lock);
    if (time_ns.frozen_offsets) {
    return -EACCES;
    }
// Don't report errors after this line
    while (i < noffsets) {
    let mut off = &offsets[i];
    let mut offset = core::ptr::null_mut();
    match (off.clockid) {
    CLOCK_MONOTONIC => {
    offset = &time_ns.offsets.monotonic;
    // break;
    }
    CLOCK_BOOTTIME => {
    offset = &time_ns.offsets.boottime;
    // break;
    }
    }
// offset = off->val;
    }
    return 0;
    }
pub static mut proc_ns_operations: usize = 0;
pub static mut proc_ns_operations: usize = 0;
pub static mut time_namespace: usize = 0;
    EXPORT_SYMBOL_GPL(init_time_ns);
#[no_mangle]
pub unsafe extern "C" fn time_ns_init()  {
    ns_tree_add(&init_time_ns);
    }