//! Automatically rewritten from C to Rust
//! Source: kernel/umh.c
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
//
// umh - the kernel usermode helper
//

pub static mut usermodehelper_bset: kernel_cap_t = 0;
pub static mut usermodehelper_inheritable: kernel_cap_t = 0;
// static DEFINE_SPINLOCK(umh_sysctl_lock);
// static DECLARE_RWSEM(umhelper_sem);
#[no_mangle]
unsafe extern "C" fn call_usermodehelper_freeinfo(info: *mut subprocess_info) {
    if (info.cleanup) {
    (*info.cleanup)(info);
    }
    kfree(info);
    }
#[no_mangle]
unsafe extern "C" fn umh_complete(sub_info: *mut subprocess_info) {
    let mut comp = xchg(&sub_info.complete, core::ptr::null_mut());
//
// See call_usermodehelper_exec(). If xchg() returns NULL
// we own sub_info, the UMH_KILLABLE caller has gone away
// or the caller used UMH_NO_WAIT.
//
    if (comp) {
    complete(comp);
    }
    else {
    call_usermodehelper_freeinfo(sub_info);
    }
    }
//
// This is the task which runs the usermode application
//
#[no_mangle]
unsafe extern "C" fn call_usermodehelper_exec_async(data: *mut c_void) -> c_int {
    let mut sub_info = data;
pub static mut new: *mut c_void = core::ptr::null_mut();
    let mut retval = 0;
    spin_lock_irq(&current.sighand.siglock);
    flush_signal_handlers(current, 1);
    spin_unlock_irq(&current.sighand.siglock);
//
// Usermodehelper threads get a copy of userspace init's
// fs_struct. Reset umask to the default.
//
    current.fs.umask = 0022;
//
// Our parent (unbound workqueue) runs with elevated scheduling
// priority. Avoid propagating that into the userspace child.
//
    set_user_nice(current, 0);
    retval = -ENOMEM;
    new = prepare_kernel_cred(current);
    if (!new) {
// goto;
    }
    spin_lock(&umh_sysctl_lock);
    new.cap_bset = cap_intersect(usermodehelper_bset, new.cap_bset);
    new.cap_inheritable = cap_intersect(usermodehelper_inheritable,
    new.cap_inheritable);
    spin_unlock(&umh_sysctl_lock);
    if (sub_info.init) {
    retval = sub_info.init(sub_info, new);
    if (retval) {
    abort_creds(new);
// goto;
    }
    }
    commit_creds(new);
    wait_for_initramfs();
    retval = kernel_execve(sub_info.path,
    (const char *const *)sub_info.argv,
    (const char *const *)sub_info.envp);
// label;
    sub_info.retval = retval;
//
// call_usermodehelper_exec_sync() will call umh_complete
// if UHM_WAIT_PROC.
//
    if (!(sub_info.wait & UMH_WAIT_PROC)) {
    umh_complete(sub_info);
    }
    if (!retval) {
    return 0;
    }
    do_exit(0);
    }
// Handles UMH_WAIT_PROC.
#[no_mangle]
unsafe extern "C" fn call_usermodehelper_exec_sync(sub_info: *mut subprocess_info) {
    let mut pid = 0;
// If SIGCLD is ignored do_wait won't populate the status.
    kernel_sigaction(SIGCHLD, SIG_DFL);
    pid = user_mode_thread(call_usermodehelper_exec_async, sub_info, SIGCHLD);
    if (pid < 0) {
    sub_info.retval = pid;
    }
    else {
    kernel_wait(pid, &sub_info.retval);
    }
// Restore default kernel sig handler
    kernel_sigaction(SIGCHLD, SIG_IGN);
    umh_complete(sub_info);
    }
//
// We need to create the usermodehelper kernel thread from a task that is affine
// to an optimized set of CPUs (or nohz housekeeping ones) such that they
// inherit a widest affinity irrespective of call_usermodehelper() callers with
// possibly reduced affinity (eg: per-cpu workqueues). We don't want
// usermodehelper targets to contend a busy CPU.
//
// Unbound workqueues provide such wide affinity and allow to block on
// UMH_WAIT_PROC requests without blocking pending request (up to some limit).
//
// Besides, workqueues provide the privilege level that caller might not have
// to perform the usermodehelper request.
//
#[no_mangle]
unsafe extern "C" fn call_usermodehelper_exec_work(work: *mut work_struct) {
    let mut sub_info = container_of!(work, subprocess_info, work);
    if (sub_info.wait & UMH_WAIT_PROC) {
    call_usermodehelper_exec_sync(sub_info);
    } else {
    let mut pid = 0;
//
// Use CLONE_PARENT to reparent it to kthreadd; we do not
// want to pollute current->children, and we need a parent
// that always ignores SIGCHLD to ensure auto-reaping.
//
    pid = user_mode_thread(call_usermodehelper_exec_async, sub_info,
    CLONE_PARENT | SIGCHLD);
    if (pid < 0) {
    sub_info.retval = pid;
    umh_complete(sub_info);
    }
    }
    }
//
// If set, call_usermodehelper_exec() will exit immediately returning -EBUSY
// (used for preventing user land processes from being created after the user
// land has been frozen during a system-wide hibernation or suspend operation).
// Should always be manipulated under umhelper_sem acquired for write.
//
pub static mut usermodehelper_disabled: umh_disable_depth = 0;
// Number of helpers running
pub static mut running_helpers: atomic_t = 0;
//
// Wait queue head used by usermodehelper_disable() to wait for all running
// helpers to finish.
//
// static DECLARE_WAIT_QUEUE_HEAD(running_helpers_waitq);
//
// Used by usermodehelper_read_lock_wait() to wait for usermodehelper_disabled
// to become 'false'.
//
// static DECLARE_WAIT_QUEUE_HEAD(usermodehelper_disabled_waitq);
//
// Time to wait for running_helpers to become zero before the setting of
// usermodehelper_disabled in usermodehelper_disable() fails
//

#[no_mangle]
pub unsafe extern "C" fn usermodehelper_read_trylock() -> c_int {
pub static mut wait: usize = 0;
pub static mut ret: c_int = 0;
    down_read(&umhelper_sem);
    for (;;) {
    prepare_to_wait(&usermodehelper_disabled_waitq, &wait,
    TASK_INTERRUPTIBLE);
    if (!usermodehelper_disabled) {
    break;
    }
    if (usermodehelper_disabled == UMH_DISABLED) {
    ret = -EAGAIN;
    }
    up_read(&umhelper_sem);
    if (ret) {
    break;
    }
    schedule();
    try_to_freeze();
    down_read(&umhelper_sem);
    }
    finish_wait(&usermodehelper_disabled_waitq, &wait);
    return ret;
    }
    EXPORT_SYMBOL_GPL(usermodehelper_read_trylock);
#[no_mangle]
pub unsafe extern "C" fn usermodehelper_read_lock_wait(timeout: c_long) -> c_long {
pub static mut wait: usize = 0;
    if (timeout < 0) {
    return -EINVAL;
    }
    down_read(&umhelper_sem);
    for (;;) {
    prepare_to_wait(&usermodehelper_disabled_waitq, &wait,
    TASK_UNINTERRUPTIBLE);
    if (!usermodehelper_disabled) {
    break;
    }
    up_read(&umhelper_sem);
    timeout = schedule_timeout(timeout);
    if (!timeout) {
    break;
    }
    down_read(&umhelper_sem);
    }
    finish_wait(&usermodehelper_disabled_waitq, &wait);
    return timeout;
    }
    EXPORT_SYMBOL_GPL(usermodehelper_read_lock_wait);
#[no_mangle]
pub unsafe extern "C" fn usermodehelper_read_unlock() {
    up_read(&umhelper_sem);
    }
    EXPORT_SYMBOL_GPL(usermodehelper_read_unlock);
//
// __usermodehelper_set_disable_depth - Modify usermodehelper_disabled.
// @depth: New value to assign to usermodehelper_disabled.
//
// Change the value of usermodehelper_disabled (under umhelper_sem locked for
// writing) and wakeup tasks waiting for it to change.
//
#[no_mangle]
pub unsafe extern "C" fn __usermodehelper_set_disable_depth(depth: umh_disable_depth) {
    down_write(&umhelper_sem);
    usermodehelper_disabled = depth;
    wake_up(&usermodehelper_disabled_waitq);
    up_write(&umhelper_sem);
    }
//
// __usermodehelper_disable - Prevent new helpers from being started.
// @depth: New value to assign to usermodehelper_disabled.
//
// Set usermodehelper_disabled to @depth and wait for running helpers to exit.
//
#[no_mangle]
pub unsafe extern "C" fn __usermodehelper_disable(depth: umh_disable_depth) -> c_int {
    let mut retval = 0;
    if (!depth) {
    return -EINVAL;
    }
    down_write(&umhelper_sem);
    usermodehelper_disabled = depth;
    up_write(&umhelper_sem);
//
// From now on call_usermodehelper_exec() won't start any new
// helpers, so it is sufficient if running_helpers turns out to
// be zero at one point (it may be increased later, but that
// doesn't matter).
//
    retval = wait_event_timeout(running_helpers_waitq,
    atomic_read(&running_helpers) == 0,
    RUNNING_HELPERS_TIMEOUT);
    if (retval) {
    return 0;
    }
    __usermodehelper_set_disable_depth(UMH_ENABLED);
    return -EAGAIN;
    }
#[no_mangle]
unsafe extern "C" fn helper_lock() {
    atomic_inc(&running_helpers);
    smp_mb__after_atomic();
    }
#[no_mangle]
unsafe extern "C" fn helper_unlock() {
    if (atomic_dec_and_test(&running_helpers)) {
    wake_up(&running_helpers_waitq);
    }
    }
//
// call_usermodehelper_setup - prepare to call a usermode helper
// @path: path to usermode executable
// @argv: arg vector for process
// @envp: environment for process
// @gfp_mask: gfp mask for memory allocation
// @init: an init function
// @cleanup: a cleanup function
// @data: arbitrary context sensitive data
//
// Returns either %NULL on allocation failure, or a subprocess_info
// structure.  This should be passed to call_usermodehelper_exec to
// exec the process and free the structure.
//
// The init function is used to customize the helper process prior to
// exec.  A non-zero return code causes the process to error out, exit,
// and return the failure to the calling process
//
// The cleanup function is just before the subprocess_info is about to
// be freed.  This can be used for freeing the argv and envp.  The
// Function must be runnable in either a process context or the
// context in which call_usermodehelper_exec is called.
//
#[no_mangle]
pub unsafe extern "C" fn call_usermodehelper_setup(path: *mut c_char, argv: *mut *mut c_char, envp: *mut *mut c_char, gfp_mask: gfp_t, info: *mut *mut int (init)( subprocess_info, data: *mut c_void) -> *mut c_void {
pub static mut sub_info: *mut c_void = core::ptr::null_mut();
    sub_info = kzalloc_obj(subprocess_info, gfp_mask);
    if (!sub_info) {
// goto;
    }
    INIT_WORK(&sub_info.work, call_usermodehelper_exec_work);

    sub_info.path = CONFIG_STATIC_USERMODEHELPER_PATH;

    sub_info.path = path;

    sub_info.argv = argv;
    sub_info.envp = envp;
    sub_info.cleanup = cleanup;
    sub_info.init = init;
    sub_info.data = data;
// label;
    return sub_info;
    }
    EXPORT_SYMBOL(call_usermodehelper_setup);
//
// call_usermodehelper_exec - start a usermode application
// @sub_info: information about the subprocess
// @wait: wait for the application to finish and return status.
// when UMH_NO_WAIT don't wait at all, but you get no useful error back
// when the program couldn't be exec'ed. This makes it safe to call
// from interrupt context.
//
// Runs a user-space application.  The application is started
// asynchronously if wait is not set, and runs as a child of system workqueues.
// (ie. it runs with full root capabilities and optimized affinity).
//
// Note: successful return value does not guarantee the helper was called at
// all. You can't rely on sub_info->{init,cleanup} being called even for
// UMH_WAIT_* wait modes as STATIC_USERMODEHELPER_PATH="" turns all helpers
// into a successful no-op.
//
#[no_mangle]
pub unsafe extern "C" fn call_usermodehelper_exec(sub_info: *mut subprocess_info, wait: c_int) -> c_int {
pub static mut state: c_uint = 0;
pub static mut done: usize = 0;
pub static mut retval: c_int = 0;
    if (!sub_info.path) {
    call_usermodehelper_freeinfo(sub_info);
    return -EINVAL;
    }
    helper_lock();
    if (usermodehelper_disabled) {
    retval = -EBUSY;
// goto;
    }
//
// If there is no binary for us to call, then just return and get out of
// here.  This allows us to set STATIC_USERMODEHELPER_PATH to "" and
// disable all call_usermodehelper() calls.
//
    if (strlen(sub_info.path) == 0) {
// goto;
    }
//
// Set the completion pointer only if there is a waiter.
// This makes it possible to use umh_complete to free
// the data structure in case of UMH_NO_WAIT.
//
    sub_info.complete = (wait == UMH_NO_WAIT) ? core::ptr::null_mut() : &done;
    sub_info.wait = wait;
    queue_work(system_dfl_wq, &sub_info.work);
    if (wait == UMH_NO_WAIT)	/* task has freed sub_info */ {
// goto;
    }
    if (wait & UMH_FREEZABLE) {
    state |= TASK_FREEZABLE;
    }
    if (wait & UMH_KILLABLE) {
    retval = wait_for_completion_state(&done, state | TASK_KILLABLE);
    if (!retval) {
// goto;
    }
// umh_complete() will see NULL and free sub_info
    if (xchg(&sub_info.complete, core::ptr::null_mut())) {
// goto;
    }
//
// fallthrough; in case of -ERESTARTSYS now do uninterruptible
// wait_for_completion_state(). Since umh_complete() shall call
// complete() in a moment if xchg() above returned NULL, this
// uninterruptible wait_for_completion_state() will not block
// SIGKILL'ed processes for long.
//
    }
    wait_for_completion_state(&done, state);
// label;
    retval = sub_info.retval;
// label;
    call_usermodehelper_freeinfo(sub_info);
// label;
    helper_unlock();
    return retval;
    }
    EXPORT_SYMBOL(call_usermodehelper_exec);
//
// call_usermodehelper() - prepare and start a usermode application
// @path: path to usermode executable
// @argv: arg vector for process
// @envp: environment for process
// @wait: wait for the application to finish and return status.
// when UMH_NO_WAIT don't wait at all, but you get no useful error back
// when the program couldn't be exec'ed. This makes it safe to call
// from interrupt context.
//
// This function is the equivalent to use call_usermodehelper_setup() and
// call_usermodehelper_exec().
//
#[no_mangle]
pub unsafe extern "C" fn call_usermodehelper(path: *const c_char, argv: *mut c_char, envp: *mut c_char, wait: c_int) -> c_int {
pub static mut info: *mut c_void = core::ptr::null_mut();
pub static mut gfp_mask: gfp_t = 0;
    info = call_usermodehelper_setup(path, argv, envp, gfp_mask,
    core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut());
    if (info == core::ptr::null_mut()) {
    return -ENOMEM;
    }
    return call_usermodehelper_exec(info, wait);
    }
    EXPORT_SYMBOL(call_usermodehelper);

#[no_mangle]
pub unsafe extern "C" fn proc_cap_handler(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
pub static mut t: usize = 0;
    unsigned long cap_array[2];
    kernel_cap_t new_cap, *cap;
    let mut err = 0;
    if (write && (!capable(CAP_SETPCAP) ||
    !capable(CAP_SYS_MODULE))) {
    return -EPERM;
    }
//
// convert from the global kernel_cap_t to the ulong array to print to
// userspace if this is a read.
//
// Legacy format: capabilities are exposed as two 32-bit values
//
    cap = table.data;
    spin_lock(&umh_sysctl_lock);
    cap_array[0] = (u32) cap.val;
    cap_array[1] = cap.val >> 32;
    spin_unlock(&umh_sysctl_lock);
    t = *table;
    t.data = &cap_array;
//
// actually read or write and array of ulongs from userspace.  Remember
// these are least significant 32 bits first
//
    err = proc_doulongvec_minmax(&t, write, buffer, lenp, ppos);
    if (err < 0) {
    return err;
    }
    new_cap.val = (u32)cap_array[0];
    new_cap.val += (u64)cap_array[1] << 32;
//
// Drop everything not in the new_cap (but don't add things)
//
    if (write) {
    spin_lock(&umh_sysctl_lock);
// cap = cap_intersect(*cap, new_cap);
    spin_unlock(&umh_sysctl_lock);
    }
    return 0;
    }
pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_umh_sysctls() -> c_int {
    register_sysctl_init("kernel/usermodehelper", usermodehelper_table);
    return 0;
    }
    early_initcall!(init_umh_sysctls);