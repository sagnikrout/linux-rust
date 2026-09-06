//! Automatically rewritten from C to Rust
//! Source: kernel/acct.c
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
// linux/kernel/acct.c
//
// BSD Process Accounting for Linux
//
// Author: Marco van Wieringen <mvw@planets.elm.net>
//
// Some code based on ideas and code from:
// Thomas K. Dyas <tdyas@eden.rutgers.edu>
//
// This file implements BSD-style process accounting. Whenever any
// process exits, an accounting record of type "struct acct" is
// written to the file specified with the acct() system call. It is
// up to user-level programs to do useful things with the accounting
// log. The kernel just provides the raw accounting information.
//
// (C) Copyright 1995 - 1997 Marco van Wieringen - ELM Consultancy B.V.
//
// Plugged two leaks. 1) It didn't return acct_file into the free_filps if
// the file happened to be read-only. 2) If the accounting was suspended
// due to the lack of space it happily allowed to reopen it and completely
// lost the old acct_file. 3/10/98, Al Viro.
//
// Now we silently close acct_file on attempt to reopen. Cleaned sys_acct().
// XTerms and EMACS are manifestations of pure evil. 21/10/98, AV.
//
// Fixed a nasty interaction with sys_umount(). If the accounting
// was suspeneded we failed to stop it on umount(). Messy.
// Another one: remount to readonly didn't stop accounting.
// Question: what should we do if we have CAP_SYS_ADMIN but not
// CAP_SYS_PACCT? Current code does the following: umount returns -EBUSY
// unless we are messing with the root. In that case we are getting a
// real mess with do_remount_sb(). 9/11/98, AV.
//
// Fixed a bunch of races (and pair of leaks). Probably not the best way,
// but this one obviously doesn't introduce deadlocks. Later. BTW, found
// one race (and leak) in BSD implementation.
// OK, that's better. ANOTHER race and leak in BSD variant. There always
// is one more bug... 10/11/98, AV.
//
// Oh, fsck... Oopsable SMP race in do_process_acct() - we must hold
// ->mmap_lock to walk the vma list of current->mm. Nasty, since it leaks
// a struct file opened for write. Fixed. 2/6/2000, AV.
//

//
// These constants control the amount of freespace that suspend and
// resume the process accounting system, and the time delay between
// each check.
// Turned into sysctl-controllable parameters. AV, 12/11/98
//
pub static mut acct_parm: [usize; 3] = [0; 3];

pub static mut ctl_table: usize = 0;
#[no_mangle]
unsafe extern "C" fn kernel_acct_sysctls_init() -> __init int {
    register_sysctl_init("kernel", kern_acct_table);
    return 0;
    }
// late_initcall;

//
// External references and all of the globals.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsd_acct_struct {
    pub pin: fs_pin,
    pub count: atomic_long_t,
    pub rcu: rcu_head,
    pub lock: mutex,
    pub active: bool,
    pub check_space: bool,
    pub needcheck: c_ulong,
    pub file: *mut file,
    pub ns: *mut pid_namespace,
    pub work: work_struct,
    pub done: completion,
    pub ac: acct_t,
}

// forward_decl: fill_ac;
// forward_decl: acct_write_process;
//
// Check the amount of free space and suspend/resume accordingly.
//
#[no_mangle]
unsafe extern "C" fn check_free_space(acct: *mut bsd_acct_struct) -> bool {
    let mut sbuf;
    if (!acct.check_space) {
    return acct.active;
    }
// May block
    if (vfs_statfs(&acct.file.f_path, &sbuf)) {
    return acct.active;
    }
    if (acct.active) {
pub static mut suspend: u64 = 0;
    do_div(suspend, 100);
    if (sbuf.f_bavail <= suspend) {
    acct.active = false;
    pr_info!("Process accounting paused\n");
    }
    } else {
pub static mut resume: u64 = 0;
    do_div(resume, 100);
    if (sbuf.f_bavail >= resume) {
    acct.active = true;
    pr_info!("Process accounting resumed\n");
    }
    }
    acct.needcheck = jiffies + ACCT_TIMEOUT*HZ;
    return acct.active;
    }
#[no_mangle]
unsafe extern "C" fn acct_put(p: *mut bsd_acct_struct) {
    if (atomic_long_dec_and_test(&p.count)) {
    kfree_rcu(p, rcu);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn to_acct() {
    return p ? container_of!(p, bsd_acct_struct, pin) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn acct_get() {
    let mut res = core::ptr::null_mut();
// label;
    smp_rmb();
    rcu_read_lock();
    res = to_acct(READ_ONCE(ns.bacct));
    if (!res) {
    rcu_read_unlock();
    return core::ptr::null_mut();
    }
    if (!atomic_long_inc_not_zero(&res.count)) {
    rcu_read_unlock();
    cpu_relax();
// goto;
    }
    rcu_read_unlock();
    mutex_lock(&res.lock);
    if (res != to_acct(READ_ONCE(ns.bacct))) {
    mutex_unlock(&res.lock);
    acct_put(res);
// goto;
    }
    return res;
    }
#[no_mangle]
unsafe extern "C" fn acct_pin_kill(pin: *mut fs_pin) {
    let mut acct = to_acct(pin);
    mutex_lock(&acct.lock);
//
// Fill the accounting struct with the exiting task's info
// before punting to the workqueue.
//
    fill_ac(acct);
    schedule_work(&acct.work);
    wait_for_completion(&acct.done);
    cmpxchg(&acct.ns.bacct, pin, core::ptr::null_mut());
    mutex_unlock(&acct.lock);
    pin_remove(pin);
    acct_put(acct);
    }
#[no_mangle]
unsafe extern "C" fn close_work(work: *mut work_struct) {
    let mut acct = container_of!(work, bsd_acct_struct, work);
    let mut file = acct.file;
// We were fired by acct_pin_kill() which holds acct->lock.
    acct_write_process(acct);
    if (file.f_op.flush) {
    file.f_op.flush(file, core::ptr::null_mut());
    }
    __fput_sync(file);
    complete(&acct.done);
    }
    DEFINE_FREE(fput_sync, file *, if (!IS_ERR_OR_NULL(_T)) __fput_sync(_T))
#[no_mangle]
unsafe extern "C" fn acct_on(name: *const char ) -> c_int {
// Difference from BSD - they don't do O_APPEND
pub static mut open_flags: c_int = 0;
    let mut ns = task_active_pid_ns(current);
    struct file *original_file __free(fput) = core::ptr::null_mut();	// in that order
    struct path internal __free(path_put) = {};	// in that order
    struct file *file __free(fput_sync) = core::ptr::null_mut();	// in that order
    let mut acct = core::ptr::null_mut();
    let mut mnt = core::ptr::null_mut();
    let mut old = core::ptr::null_mut();
// CLASS;
    original_file = file_open_name(pathname, open_flags, 0);
    if (IS_ERR(original_file)) {
    return PTR_ERR(original_file);
    }
    mnt = mnt_clone_internal(&original_file.f_path);
    if (IS_ERR(mnt)) {
    return PTR_ERR(mnt);
    }
    internal.mnt = mnt;
    internal.dentry = dget(mnt.mnt_root);
    file = dentry_open(&internal, open_flags, current_cred());
    if (IS_ERR(file)) {
    return PTR_ERR(file);
    }
    if (!S_ISREG(file_inode(file).i_mode)) {
    return -EACCES;
    }
// Exclude kernel internal filesystems.
    if (file_inode(file).i_sb.s_flags & (SB_NOUSER | SB_KERNMOUNT)) {
    return -EINVAL;
    }
// Exclude procfs and sysfs.
    if (file_inode(file).i_sb.s_type.fs_flags & FS_USERNS_MOUNT_RESTRICTED) {
    return -EINVAL;
    }
    if (!(file.f_mode & FMODE_CAN_WRITE)) {
    return -EIO;
    }
    acct = kzalloc_obj(bsd_acct_struct);
    if (!acct) {
    return -ENOMEM;
    }
    atomic_long_set(&acct.count, 1);
    init_fs_pin(&acct.pin, acct_pin_kill);
    acct.file = no_free_ptr(file);
    acct.needcheck = jiffies;
    acct.ns = ns;
    mutex_init(&acct.lock);
// INIT_WORK;
    init_completion(&acct.done);
    mutex_lock_nested(&acct.lock, 1);	/* nobody has seen it yet */
    pin_insert(&acct.pin, original_file.f_path.mnt);
    rcu_read_lock();
    old = xchg(&ns.bacct, &acct.pin);
    mutex_unlock(&acct.lock);
    pin_kill(old);
    return 0;
    }
// static DEFINE_MUTEX(acct_on_mutex);
//
// sys_acct - enable/disable process accounting
// @name: file name for accounting records or NULL to shutdown accounting
//
// sys_acct() is the only system call needed to implement process
// accounting. It takes the name of the file where accounting records
// should be written. If the filename is NULL, accounting will be
// shutdown.
//
// Returns: 0 for success or negative errno values for failure.
//
#[no_mangle]
pub unsafe extern "C" fn sys_acct() {
pub static mut error: c_int = 0;
    if (!capable(CAP_SYS_PACCT)) {
    return -EPERM;
    }
    if (name) {
    mutex_lock(&acct_on_mutex);
    error = acct_on(name);
    mutex_unlock(&acct_on_mutex);
    } else {
    rcu_read_lock();
    pin_kill(task_active_pid_ns(current).bacct);
    }
    return error;
    }
#[no_mangle]
pub unsafe extern "C" fn acct_exit_ns(ns: *mut pid_namespace) {
    rcu_read_lock();
    pin_kill(ns.bacct);
    }
//
// encode an u64 into a comp_t
//
// This routine has been adopted from the encode_comp_t() function in
// the kern_acct.c file of the FreeBSD operating system. The encoding
// is a 13-bit fraction with a 3-bit (base 8) exponent.
//

#[no_mangle]
unsafe extern "C" fn encode_comp_t(value: u64) -> comp_t {
    let mut exp = 0;
    let mut rnd = 0;
    exp = rnd = 0;
    while (value > MAXFRACT) {
    rnd = value & (1 << (EXPSIZE - 1));	/* Round up? */
    value >>= EXPSIZE;	/* Base 8 exponent == 3 bit shift. */
    exp += 1;
    }
//
// If we need to round up, do it (and handle overflow correctly).
//
    if (rnd && (++value > MAXFRACT)) {
    value >>= EXPSIZE;
    exp += 1;
    }
    if (exp > (((comp_t) ~0U) >> MANTSIZE)) {
    return (comp_t) ~0U;
    }
//
// Clean it up and polish it off.
//
    exp <<= MANTSIZE;		/* Shift the exponent into place */
    exp += value;			/* and add on the mantissa. */
    return exp;
    }

//
// encode an u64 into a comp2_t (24 bits)
//
// Format: 5 bit base 2 exponent, 20 bits mantissa.
// The leading bit of the mantissa is not stored, but implied for
// non-zero exponents.
// Largest encodable value is 50 bits.
//

#[no_mangle]
unsafe extern "C" fn encode_comp2_t(value: u64) -> comp2_t {
    let mut exp = 0;
    let mut rnd = 0;
    exp = (value > (MAXFRACT2>>1));
    rnd = 0;
    while (value > MAXFRACT2) {
    rnd = value & 1;
    value >>= 1;
    exp += 1;
    }
//
// If we need to round up, do it (and handle overflow correctly).
//
    if (rnd && (++value > MAXFRACT2)) {
    value >>= 1;
    exp += 1;
    }
    if (exp > MAXEXP2) {
// Overflow. Return largest representable number instead.
    return (1ul << (MANTSIZE2+EXPSIZE2-1)) - 1;
    } else {
    return (value & (MAXFRACT2>>1)) | (exp << (MANTSIZE2-1));
    }
    }

//
// encode an u64 into a 32 bit IEEE float
//
#[no_mangle]
unsafe extern "C" fn encode_float(value: u64) -> u32 {
pub static mut exp: unsigned = 190;
pub static mut u: c_uint = 0;
    if (value == 0) {
    return 0;
    }
    while ((s64)value > 0) {
    value <<= 1;
    exp -= 1;
    }
    u = (u32)(value >> 40) & 0x7fffffu;
    return u | (exp << 23);
    }

//
// Write an accounting entry for an exiting process
//
// The acct_process() call is the workhorse of the process
// accounting system. The struct acct is built here and then written
// into the accounting file. This function should only be called from
// do_exit() or when switching to a different output file.
//
#[no_mangle]
unsafe extern "C" fn fill_ac(acct: *mut bsd_acct_struct) {
    let mut pacct = &current.signal.pacct;
    let mut file = acct.file;
    let mut ac = &acct.ac;
    u64 elapsed, run_time;
    let mut btime;
    let mut tty = core::ptr::null_mut();
    lockdep_assert_held(&acct.lock);
    if (time_is_after_jiffies(acct.needcheck)) {
    acct.check_space = false;
// Don't fill in @ac if nothing will be written.
    if (!acct.active) {
    return;
    }
    } else {
    acct.check_space = true;
    }
//
// Fill the accounting struct with the needed info as recorded
// by the different kernel functions.
//
    memset(ac, 0, sizeof!(acct_t));
    ac.ac_version = ACCT_VERSION | ACCT_BYTEORDER;
    strscpy(ac.ac_comm, current.comm, sizeof!(ac.ac_comm));
// calculate run_time in nsec
    run_time = ktime_get_ns();
    run_time -= current.group_leader.start_time;
// convert nsec -> AHZ
    elapsed = nsec_to_AHZ(run_time);

    ac.ac_etime = encode_float(elapsed);

    ac.ac_etime = encode_comp_t(elapsed < (unsigned long) -1l ?
    (unsigned long) elapsed : (unsigned long) -1l);

    {
// new enlarged etime field
pub static mut etime: comp2_t = 0;
    ac.ac_etime_hi = etime >> 16;
    ac.ac_etime_lo = (u16) etime;
    }

    do_div(elapsed, AHZ);
    btime = ktime_get_real_seconds() - elapsed;
    ac.ac_btime = clamp_t(time64_t, btime, 0, U32_MAX);

    ac.ac_ahz = AHZ;

    spin_lock_irq(&current.sighand.siglock);
    tty = current.signal.tty;	/* Safe as we hold the siglock */
    ac.ac_tty = tty ? old_encode_dev(tty_devnum(tty)) : 0;
    ac.ac_utime = encode_comp_t(nsec_to_AHZ(pacct.ac_utime));
    ac.ac_stime = encode_comp_t(nsec_to_AHZ(pacct.ac_stime));
    ac.ac_flag = pacct.ac_flag;
    ac.ac_mem = encode_comp_t(pacct.ac_mem);
    ac.ac_minflt = encode_comp_t(pacct.ac_minflt);
    ac.ac_majflt = encode_comp_t(pacct.ac_majflt);
    ac.ac_exitcode = pacct.ac_exitcode;
    spin_unlock_irq(&current.sighand.siglock);
// we really need to bite the bullet and change layout
    ac.ac_uid = from_kuid_munged(file.f_cred.user_ns, current_uid());
    ac.ac_gid = from_kgid_munged(file.f_cred.user_ns, current_gid());

// backward-compatible 16 bit fields
    ac.ac_uid16 = ac.ac_uid;
    ac.ac_gid16 = ac.ac_gid;

    {
    let mut ns = acct.ns;
    ac.ac_pid = task_tgid_nr_ns(current, ns);
    rcu_read_lock();
    ac.ac_ppid = task_tgid_nr_ns(rcu_dereference(current.real_parent), ns);
    rcu_read_unlock();
    }

    }
#[no_mangle]
unsafe extern "C" fn acct_write_process(acct: *mut bsd_acct_struct) {
    let mut file = acct.file;
    let mut ac = &acct.ac;
// Perform file operations on behalf of whoever enabled accounting
    scoped_with_creds(file.f_cred) {
//
// First check to see if there is enough free_space to continue
// the process accounting system. Then get freeze protection. If
// the fs is frozen, just skip the write as we could deadlock
// the system otherwise.
//
    if (check_free_space(acct) && file_start_write_trylock(file)) {
// it's been opened O_APPEND, so position is irrelevant
pub static mut pos: loff_t = 0;
    __kernel_write(file, ac, sizeof!(acct_t), &pos);
    file_end_write(file);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn do_acct_process(acct: *mut bsd_acct_struct) {
    let mut flim = 0;
// Accounting records are not subject to resource limits.
    flim = rlimit(RLIMIT_FSIZE);
    current.signal.rlim[RLIMIT_FSIZE].rlim_cur = RLIM_INFINITY;
    fill_ac(acct);
    acct_write_process(acct);
    current.signal.rlim[RLIMIT_FSIZE].rlim_cur = flim;
    }
//
// acct_collect - collect accounting information into pacct_struct
// @exitcode: task exit code
// @group_dead: not 0, if this thread is the last one in the process.
//
#[no_mangle]
pub unsafe extern "C" fn acct_collect(exitcode: c_long, group_dead: c_int) {
    let mut pacct = &current.signal.pacct;
    u64 utime, stime;
pub static mut vsize: c_ulong = 0;
    if (group_dead && current.mm) {
    let mut mm = current.mm;
// VMA_ITERATOR;
    let mut vma = core::ptr::null_mut();
    mmap_read_lock(mm);
    for_each_vma(vmi, vma) {
    vsize += vma.vm_end - vma.vm_start;
    }
    mmap_read_unlock(mm);
    }
    spin_lock_irq(&current.sighand.siglock);
    if (group_dead) {
    pacct.ac_mem = vsize / 1024;
    }
    if (thread_group_leader(current)) {
    pacct.ac_exitcode = exitcode;
    if (current.flags & PF_FORKNOEXEC) {
    pacct.ac_flag |= AFORK;
    }
    }
    if (current.flags & PF_SUPERPRIV) {
    pacct.ac_flag |= ASU;
    }
    if (current.flags & PF_DUMPCORE) {
    pacct.ac_flag |= ACORE;
    }
    if (current.flags & PF_SIGNALED) {
    pacct.ac_flag |= AXSIG;
    }
    task_cputime(current, &utime, &stime);
    pacct.ac_utime += utime;
    pacct.ac_stime += stime;
    pacct.ac_minflt += current.min_flt;
    pacct.ac_majflt += current.maj_flt;
    spin_unlock_irq(&current.sighand.siglock);
    }
#[no_mangle]
unsafe extern "C" fn slow_acct_process(ns: *mut pid_namespace) {
    while (ns) {
    let mut acct = acct_get(ns);
    if (acct) {
    do_acct_process(acct);
    mutex_unlock(&acct.lock);
    acct_put(acct);
    }
    }
    }
//
// acct_process - handles process accounting for an exiting task
//
#[no_mangle]
pub unsafe extern "C" fn acct_process() {
    let mut ns = core::ptr::null_mut();
//
// This loop is safe lockless, since current is still
// alive and holds its namespace, which in turn holds
// its parent.
//
    while (ns != core::ptr::null_mut()) {
    if (ns.bacct) {
    break;
    }
    }
    if (unlikely(ns)) {
    slow_acct_process(ns);
    }
    }