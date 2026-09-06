//! Automatically rewritten from C to Rust
//! Source: kernel/capability.c
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


























// SPDX-License-Identifier: GPL-2.0
//
// linux/kernel/capability.c
//
// Copyright (C) 1997  Andrew Main <zefram@fysh.org>
//
// Integrated into 2.1.97+,  Andrew G. Morgan <morgan@kernel.org>
// 30 May 2002:	Cleanup, Robert M. Love <rml@tech9.net>
//

pub static mut file_caps_enabled: c_int = 1;
#[no_mangle]
unsafe extern "C" fn file_caps_disable(str: *mut c_char) -> c_int {
    file_caps_enabled = 0;
    return 1;
    }
// __setup;

//
// More recent versions of libcap are available from:
//
// http://www.kernel.org/pub/linux/libs/security/linux-privs
//
#[no_mangle]
unsafe extern "C" fn warn_legacy_capability_use() {
    pr_info_once!("warning: `%s' uses 32-bit capabilities (legacy support in use)\n",
    current.comm);
    }
//
// Version 2 capabilities worked fine, but the linux/capability.h file
// that accompanied their introduction encouraged their use without
// the necessary user-space source code changes. As such, we have
// created a version 3 with equivalent functionality to version 2, but
// with a header change to protect legacy source code from using
// version 2 when it wanted to use version 1. If your system has code
// that trips the following warning, it is using version 2 specific
// capabilities and may be doing so insecurely.
//
// The remedy is to either upgrade your version of libcap (to 2.10+,
// if the application is linked against it), or recompile your
// application with modern kernel headers and this warning will go
// away.
//
#[no_mangle]
unsafe extern "C" fn warn_deprecated_v2() {
    pr_info_once!("warning: `%s' uses deprecated v2 capabilities in a way that may be insecure\n",
    current.comm);
    }
//
// Version check. Return the number of u32s in each capability flag
// array, or a negative value on error.
//
#[no_mangle]
unsafe extern "C" fn cap_validate_magic(header: cap_user_header_t, tocopy: *mut unsigned) -> c_int {
    let mut version = 0;
    if (get_user(version, &header.version)) {
    return -EFAULT;
    }
    match (version) {
    _LINUX_CAPABILITY_VERSION_1 => {
    warn_legacy_capability_use();
// tocopy = _LINUX_CAPABILITY_U32S_1;
    // break;
    }
    _LINUX_CAPABILITY_VERSION_2 => {
    warn_deprecated_v2();
    fallthrough;	/* v3 is otherwise equivalent to v2 */
    }
    _LINUX_CAPABILITY_VERSION_3 => {
// tocopy = _LINUX_CAPABILITY_U32S_3;
    // break;
    }
    _ => {
    if (put_user((u32)_KERNEL_CAPABILITY_VERSION, &header.version)) {
    return -EFAULT;
    }
    return -EINVAL;
    }
    }
    return 0;
    }
//
// The only thing that can change the capabilities of the current
// process is the current process. As such, we can't be in this code
// at the same time as we are in the process of setting capabilities
// in this process. The net result is that we can limit our use of
// locks to when we are reading the caps of another process.
//
#[no_mangle]
pub unsafe extern "C" fn cap_get_target_pid() {
    let mut ret = 0;
    if (pid && (pid != task_pid_vnr(current))) {
    let mut target = core::ptr::null_mut();
    rcu_read_lock();
    target = find_task_by_vpid(pid);
    if (!target) {
    ret = -ESRCH;
    }
    else {
    ret = security_capget(target, pEp, pIp, pPp);
    }
    rcu_read_unlock();
    } else {
    ret = security_capget(current, pEp, pIp, pPp);
    }
    return ret;
    }
//
// sys_capget - get the capabilities of a given process.
// @header: pointer to struct that contains capability version and
// target pid data
// @dataptr: pointer to struct that contains the effective, permitted,
// and inheritable capabilities that are returned
//
// Returns 0 on success and < 0 on error.
//
#[no_mangle]
pub unsafe extern "C" fn sys_capget() {
pub static mut ret: c_int = 0;
    let mut pid = 0;
pub static mut tocopy: c_uint = 0;
    kernel_cap_t pE, pI, pP;
    struct __user_cap_data_struct kdata[2];
    ret = cap_validate_magic(header, &tocopy);
    if ((dataptr == core::ptr::null_mut()) || (ret != 0)) {
    return ((dataptr == core::ptr::null_mut()) && (ret == -EINVAL)) ? 0 : ret;
    }
    if (get_user(pid, &header.pid)) {
    return -EFAULT;
    }
    if (pid < 0) {
    return -EINVAL;
    }
    ret = cap_get_target_pid(pid, &pE, &pI, &pP);
    if (ret) {
    return ret;
    }
//
// Annoying legacy format with 64-bit capabilities exposed
// as two sets of 32-bit fields, so we need to split the
// capability values up.
//
    kdata[0].effective   = pE.val; kdata[1].effective   = pE.val >> 32;
    kdata[0].permitted   = pP.val; kdata[1].permitted   = pP.val >> 32;
    kdata[0].inheritable = pI.val; kdata[1].inheritable = pI.val >> 32;
//
// Note, in the case, tocopy < _KERNEL_CAPABILITY_U32S,
// we silently drop the upper capabilities here. This
// has the effect of making older libcap
// implementations implicitly drop upper capability
// bits when they perform a: capget/modify/capset
// sequence.
//
// This behavior is considered fail-safe
// behavior. Upgrading the application to a newer
// version of libcap will enable access to the newer
// capabilities.
//
// An alternative would be to return an error here
// (-ERANGE), but that causes legacy applications to
// unexpectedly fail; the capget/modify/capset aborts
// before modification is attempted and the application
// fails.
//
    if (copy_to_user(dataptr, kdata, tocopy * sizeof!(kdata[0]))) {
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mk_kernel_cap(low: u32, high: u32) -> kernel_cap_t {
    return (kernel_cap_t) { (low | ((u64)high << 32)) & CAP_VALID_MASK };
    }
//
// sys_capset - set capabilities for a process or  a group of processes
// @header: pointer to struct that contains capability version and
// target pid data
// @data: pointer to struct that contains the effective, permitted,
// and inheritable capabilities
//
// Set capabilities for the current process only.  The ability to any other
// process(es) has been deprecated and removed.
//
// The restrictions on setting capabilities are specified as:
//
// I: any raised capabilities must be a subset of the old permitted
// P: any raised capabilities must be a subset of the old permitted
// E: must be set to a subset of new permitted
//
// Returns 0 on success and < 0 on error.
//
#[no_mangle]
pub unsafe extern "C" fn sys_capset() {
pub static mut __user_cap_data_struct: usize = 0;
    let mut tocopy = 0;
    let mut copybytes = 0;
    kernel_cap_t inheritable, permitted, effective;
    let mut new = core::ptr::null_mut();
    let mut ret = 0;
    let mut pid = 0;
    ret = cap_validate_magic(header, &tocopy);
    if (ret != 0) {
    return ret;
    }
    if (get_user(pid, &header.pid)) {
    return -EFAULT;
    }
// may only affect current now
    if (pid != 0 && pid != task_pid_vnr(current)) {
    return -EPERM;
    }
    copybytes = tocopy * sizeof!(__user_cap_data_struct);
    if (copybytes > sizeof!(kdata)) {
    return -EFAULT;
    }
    if (copy_from_user(&kdata, data, copybytes)) {
    return -EFAULT;
    }
    effective   = mk_kernel_cap(kdata[0].effective,   kdata[1].effective);
    permitted   = mk_kernel_cap(kdata[0].permitted,   kdata[1].permitted);
    inheritable = mk_kernel_cap(kdata[0].inheritable, kdata[1].inheritable);
    new = prepare_creds();
    if (!new) {
    return -ENOMEM;
    }
    ret = security_capset(new, current_cred(),
    &effective, &inheritable, &permitted);
    if (ret < 0) {
// goto;
    }
    audit_log_capset(new, current_cred());
    return commit_creds(new);
// label;
    abort_creds(new);
    return ret;
    }
//
// has_ns_capability - Does a task have a capability in a specific user ns
// @t: The task in question
// @ns: target user namespace
// @cap: The capability to be tested for
//
// Return true if the specified task has the given superior capability
// currently in effect to the specified user namespace, false if not.
//
// Note that this does not set PF_SUPERPRIV on the task.
//
#[no_mangle]
pub unsafe extern "C" fn has_ns_capability() {
    let mut ret = 0;
    rcu_read_lock();
    ret = security_capable(__task_cred(t), ns, cap, CAP_OPT_NONE);
    rcu_read_unlock();
    return (ret == 0);
    }
//
// has_ns_capability_noaudit - Does a task have a capability (unaudited)
// in a specific user ns.
// @t: The task in question
// @ns: target user namespace
// @cap: The capability to be tested for
//
// Return true if the specified task has the given superior capability
// currently in effect to the specified user namespace, false if not.
// Do not write an audit message for the check.
//
// Note that this does not set PF_SUPERPRIV on the task.
//
#[no_mangle]
pub unsafe extern "C" fn has_ns_capability_noaudit() {
    let mut ret = 0;
    rcu_read_lock();
    ret = security_capable(__task_cred(t), ns, cap, CAP_OPT_NOAUDIT);
    rcu_read_unlock();
    return (ret == 0);
    }
//
// has_capability_noaudit - Does a task have a capability (unaudited) in the
// initial user ns
// @t: The task in question
// @cap: The capability to be tested for
//
// Return true if the specified task has the given superior capability
// currently in effect to init_user_ns, false if not.  Don't write an
// audit message for the check.
//
// Note that this does not set PF_SUPERPRIV on the task.
//
#[no_mangle]
pub unsafe extern "C" fn has_capability_noaudit(t: *mut task_struct, cap: c_int) -> bool {
    return has_ns_capability_noaudit(t, &init_user_ns, cap);
    }
#[no_mangle]
pub unsafe extern "C" fn ns_capable_common() {
    let mut capable = 0;
    if (unlikely(!cap_valid(cap))) {
    pr_crit("capable() called with invalid cap=%u\n", cap);
// BUG;
    }
    capable = security_capable(current_cred(), ns, cap, opts);
    if (capable == 0) {
    current.flags |= PF_SUPERPRIV;
    return true;
    }
    return false;
    }
//
// ns_capable - Determine if the current task has a superior capability in effect
// @ns:  The usernamespace we want the capability in
// @cap: The capability to be tested for
//
// Return true if the current task has the given superior capability currently
// available for use, false if not.
//
// This sets PF_SUPERPRIV on the task if the capability is available on the
// assumption that it's about to be used.
//
#[no_mangle]
pub unsafe extern "C" fn ns_capable(ns: *mut user_namespace, cap: c_int) -> bool {
    return ns_capable_common(ns, cap, CAP_OPT_NONE);
    }
// EXPORT_SYMBOL;
//
// ns_capable_noaudit - Determine if the current task has a superior capability
// (unaudited) in effect
// @ns:  The usernamespace we want the capability in
// @cap: The capability to be tested for
//
// Return true if the current task has the given superior capability currently
// available for use, false if not.
//
// This sets PF_SUPERPRIV on the task if the capability is available on the
// assumption that it's about to be used.
//
#[no_mangle]
pub unsafe extern "C" fn ns_capable_noaudit(ns: *mut user_namespace, cap: c_int) -> bool {
    return ns_capable_common(ns, cap, CAP_OPT_NOAUDIT);
    }
// EXPORT_SYMBOL;
//
// ns_capable_setid - Determine if the current task has a superior capability
// in effect, while signalling that this check is being done from within a
// setid or setgroups syscall.
// @ns:  The usernamespace we want the capability in
// @cap: The capability to be tested for
//
// Return true if the current task has the given superior capability currently
// available for use, false if not.
//
// This sets PF_SUPERPRIV on the task if the capability is available on the
// assumption that it's about to be used.
//
#[no_mangle]
pub unsafe extern "C" fn ns_capable_setid(ns: *mut user_namespace, cap: c_int) -> bool {
    return ns_capable_common(ns, cap, CAP_OPT_INSETID);
    }
// EXPORT_SYMBOL;
//
// capable - Determine if the current task has a superior capability in effect
// @cap: The capability to be tested for
//
// Return true if the current task has the given superior capability currently
// available for use, false if not.
//
// This sets PF_SUPERPRIV on the task if the capability is available on the
// assumption that it's about to be used.
//
#[no_mangle]
pub unsafe extern "C" fn capable(cap: c_int) -> bool {
    return ns_capable(&init_user_ns, cap);
    }
// EXPORT_SYMBOL;
//
// capable_noaudit - Determine if the current task has a superior
// capability in effect by checking the process's effective
// capabilities (unaudited).
// @cap: The capability to be tested for
//
// This is the same as capable(), except it uses CAP_OPT_NOAUDIT as to prevent
// issuing spurious audit messages.
//
// This sets PF_SUPERPRIV on the task if the capability is available on the
// assumption that it's about to be used.
//
#[no_mangle]
pub unsafe extern "C" fn capable_noaudit(cap: c_int) -> bool {
    return ns_capable_noaudit(&init_user_ns, cap);
    }
// EXPORT_SYMBOL;

//
// file_ns_capable - Determine if the file's opener had a capability in effect
// @file:  The file we want to check
// @ns:  The usernamespace we want the capability in
// @cap: The capability to be tested for
//
// Return true if task that opened the file had a capability in effect
// when the file was opened.
//
// This does not set PF_SUPERPRIV because the caller may not
// actually be privileged.
//
#[no_mangle]
pub unsafe extern "C" fn file_ns_capable() {
    if (WARN_ON_ONCE!(!cap_valid(cap))) {
    return false;
    }
    if (security_capable(file.f_cred, ns, cap, CAP_OPT_NONE) == 0) {
    return true;
    }
    return false;
    }
// EXPORT_SYMBOL;
//
// privileged_wrt_inode_uidgid - Do capabilities in the namespace work over the inode?
// @ns: The user namespace in question
// @idmap: idmap of the mount @inode was found from
// @inode: The inode in question
//
// Return true if the inode uid and gid are within the namespace.
//
#[no_mangle]
pub unsafe extern "C" fn privileged_wrt_inode_uidgid() {
    return vfsuid_has_mapping(ns, i_uid_into_vfsuid(idmap, inode)) &&
    vfsgid_has_mapping(ns, i_gid_into_vfsgid(idmap, inode));
    }
//
// capable_wrt_inode_uidgid - Check nsown_capable and uid and gid mapped
// @idmap: idmap of the mount @inode was found from
// @inode: The inode in question
// @cap: The capability in question
//
// Return true if the current task has the given capability targeted at
// its own user namespace and that the given inode's uid and gid are
// mapped into the current user namespace.
//
#[no_mangle]
pub unsafe extern "C" fn capable_wrt_inode_uidgid() {
    let mut ns = current_user_ns();
    return ns_capable(ns, cap) &&
    privileged_wrt_inode_uidgid(ns, idmap, inode);
    }
// EXPORT_SYMBOL;
//
// ptracer_capable - Determine if the ptracer holds CAP_SYS_PTRACE in the namespace
// @tsk: The task that may be ptraced
// @ns: The user namespace to search for CAP_SYS_PTRACE in
//
// Return true if the task that is ptracing the current task had CAP_SYS_PTRACE
// in the specified user namespace.
//
#[no_mangle]
pub unsafe extern "C" fn ptracer_capable(tsk: *mut task_struct, ns: *mut user_namespace) -> bool {
    let mut ret = 0;  /* An absent tracer adds no restrictions */
    let mut cred = core::ptr::null_mut();
    rcu_read_lock();
    cred = rcu_dereference(tsk.ptracer_cred);
    if (cred) {
    ret = security_capable(cred, ns, CAP_SYS_PTRACE,
    CAP_OPT_NOAUDIT);
    }
    rcu_read_unlock();
    return (ret == 0);
    }