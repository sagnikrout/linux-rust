//! Automatically rewritten from C to Rust
//! Source: kernel/time/posix-clock.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Support for dynamic clock devices
//
// Copyright (C) 2010 OMICRON electronics GmbH
//

//
// Returns NULL if the posix_clock instance attached to 'fp' is old and stale.
//
#[no_mangle]
pub unsafe extern "C" fn get_posix_clock(fp: *mut file) -> *mut c_void {
    let mut pccontext = fp.private_data;
    let mut clk = pccontext.clk;
    down_read(&clk.rwsem);
    if (!clk.zombie) {
    return clk;
    }
    up_read(&clk.rwsem);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn put_posix_clock(clk: *mut posix_clock) {
    up_read(&clk.rwsem);
    }
#[no_mangle]
pub unsafe extern "C" fn posix_clock_read(fp: *mut file, buf: *mut c_char, count: size_t, ppos: *mut loff_t) -> ssize_t {
    let mut pccontext = fp.private_data;
    let mut clk = get_posix_clock(fp);
pub static mut err: c_int = 0;
    if (!clk) {
    return -ENODEV;
    }
    if (clk.ops.read) {
    err = clk.ops.read(pccontext, fp.f_flags, buf, count);
    }
    put_posix_clock(clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn posix_clock_poll(fp: *mut file, wait: *mut poll_table) -> __poll_t {
    let mut pccontext = fp.private_data;
    let mut clk = get_posix_clock(fp);
pub static mut result: __poll_t = 0;
    if (!clk) {
    return EPOLLERR;
    }
    if (clk.ops.poll) {
    result = clk.ops.poll(pccontext, fp, wait);
    }
    put_posix_clock(clk);
    return result;
    }
#[no_mangle]
pub unsafe extern "C" fn posix_clock_ioctl(fp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long {
    let mut pccontext = fp.private_data;
    let mut clk = get_posix_clock(fp);
pub static mut err: c_int = 0;
    if (!clk) {
    return -ENODEV;
    }
    if (clk.ops.ioctl) {
    err = clk.ops.ioctl(pccontext, cmd, arg);
    }
    put_posix_clock(clk);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn posix_clock_open(inode: *mut inode, fp: *mut file) -> c_int {
    let mut err = 0;
    let mut clk = container_of!(inode.i_cdev, posix_clock, cdev);
pub static mut pccontext: *mut c_void = core::ptr::null_mut();
    down_read(&clk.rwsem);
    if (clk.zombie) {
    err = -ENODEV;
// goto;
    }
    pccontext = kzalloc_obj(*pccontext);
    if (!pccontext) {
    err = -ENOMEM;
// goto;
    }
    pccontext.clk = clk;
    pccontext.fp = fp;
    if (clk.ops.open) {
    err = clk.ops.open(pccontext, fp.f_mode);
    if (err) {
    kfree(pccontext);
// goto;
    }
    }
    fp.private_data = pccontext;
    get_device(clk.dev);
    err = 0;
// label;
    up_read(&clk.rwsem);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn posix_clock_release(inode: *mut inode, fp: *mut file) -> c_int {
    let mut pccontext = fp.private_data;
pub static mut clk: *mut c_void = core::ptr::null_mut();
pub static mut err: c_int = 0;
    if (!pccontext) {
    return -ENODEV;
    }
    clk = pccontext.clk;
    if (clk.ops.release) {
    err = clk.ops.release(pccontext);
    }
    put_device(clk.dev);
    kfree(pccontext);
    fp.private_data = core::ptr::null_mut();
    return err;
    }
pub static mut file_operations: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn posix_clock_register(clk: *mut posix_clock, dev: *mut device) -> c_int {
    let mut err = 0;
    init_rwsem(&clk.rwsem);
    cdev_init(&clk.cdev, &posix_clock_file_operations);
    err = cdev_device_add(&clk.cdev, dev);
    if (err) {
    pr_err!("%s unable to add device %d:%d\n",
    dev_name(dev), MAJOR(dev.devt), MINOR(dev.devt));
    return err;
    }
    clk.cdev.owner = clk.ops.owner;
    clk.dev = dev;
    return 0;
    }
    EXPORT_SYMBOL_GPL(posix_clock_register);
#[no_mangle]
pub unsafe extern "C" fn posix_clock_unregister(clk: *mut posix_clock) {
    cdev_device_del(&clk.cdev, clk.dev);
    down_write(&clk.rwsem);
    clk.zombie = true;
    up_write(&clk.rwsem);
    put_device(clk.dev);
    }
    EXPORT_SYMBOL_GPL(posix_clock_unregister);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct posix_clock_desc {
    pub fp: *mut file,
    pub clk: *mut posix_clock,
}

#[no_mangle]
unsafe extern "C" fn get_clock_desc(id: clockid_t, cd: *mut posix_clock_desc) -> c_int {
    let mut fp = fget(clockid_to_fd(id));
pub static mut err: c_int = 0;
    if (!fp) {
    return err;
    }
    if (fp.f_op.open != posix_clock_open || !fp.private_data) {
// goto;
    }
    cd.fp = fp;
    cd.clk = get_posix_clock(fp);
    err = cd.clk ? 0 : -ENODEV;
// label;
    if (err) {
    fput(fp);
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn put_clock_desc(cd: *mut posix_clock_desc) {
    put_posix_clock(cd.clk);
    fput(cd.fp);
    }
#[no_mangle]
unsafe extern "C" fn pc_clock_adjtime(id: clockid_t, tx: *mut __kernel_timex) -> c_int {
pub static mut cd: usize = 0;
    let mut err = 0;
    err = get_clock_desc(id, &cd);
    if (err) {
    return err;
    }
    if (tx.modes && (cd.fp.f_mode & FMODE_WRITE) == 0) {
    err = -EACCES;
// goto;
    }
    if (cd.clk.ops.clock_adjtime) {
    err = cd.clk.ops.clock_adjtime(cd.clk, tx);
    }
    else {
    err = -EOPNOTSUPP;
    }
// label;
    put_clock_desc(&cd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pc_clock_gettime(id: clockid_t, ts: *mut timespec64) -> c_int {
pub static mut cd: usize = 0;
    let mut err = 0;
    err = get_clock_desc(id, &cd);
    if (err) {
    return err;
    }
    if (cd.clk.ops.clock_gettime) {
    err = cd.clk.ops.clock_gettime(cd.clk, ts);
    }
    else {
    err = -EOPNOTSUPP;
    }
    put_clock_desc(&cd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pc_clock_getres(id: clockid_t, ts: *mut timespec64) -> c_int {
pub static mut cd: usize = 0;
    let mut err = 0;
    err = get_clock_desc(id, &cd);
    if (err) {
    return err;
    }
    if (cd.clk.ops.clock_getres) {
    err = cd.clk.ops.clock_getres(cd.clk, ts);
    }
    else {
    err = -EOPNOTSUPP;
    }
    put_clock_desc(&cd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn pc_clock_settime(id: clockid_t, ts: *const timespec64) -> c_int {
pub static mut cd: usize = 0;
    let mut err = 0;
    if (!timespec64_valid_strict(ts)) {
    return -EINVAL;
    }
    err = get_clock_desc(id, &cd);
    if (err) {
    return err;
    }
    if ((cd.fp.f_mode & FMODE_WRITE) == 0) {
    err = -EACCES;
// goto;
    }
    if (cd.clk.ops.clock_settime) {
    err = cd.clk.ops.clock_settime(cd.clk, ts);
    }
    else {
    err = -EOPNOTSUPP;
    }
// label;
    put_clock_desc(&cd);
    return err;
    }
pub static mut k_clock: usize = 0;