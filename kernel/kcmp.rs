//! Automatically rewritten from C to Rust
//! Source: kernel/kcmp.c
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
// We don't expose the real in-memory order of objects for security reasons.
// But still the comparison results should be suitable for sorting. So we
// obfuscate kernel pointers values and compare the production instead.
//
// The obfuscation is done in two steps. First we xor the kernel pointer with
// a random value, which puts pointer into a new position in a reordered space.
// Secondly we multiply the xor production with a large odd random number to
// permute its bits even more (the odd multiplier guarantees that the product
// is unique ever after the high bits are truncated, since any odd number is
// relative prime to 2^n).
//
// Note also that the obfuscation itself is invisible to userspace and if needed
// it can be changed to an alternate scheme.
//
    static unsigned long cookies[KCMP_TYPES][2] __read_mostly;
#[no_mangle]
unsafe extern "C" fn kptr_obfuscate(v: c_long, type: c_int) -> c_long {
    return (v ^ cookies[type][0]) * cookies[type][1];
    }
//
// 0 - equal, i.e. v1 = v2
// 1 - less than, i.e. v1 < v2
// 2 - greater than, i.e. v1 > v2
// 3 - not equal but ordering unavailable (reserved for future)
//
#[no_mangle]
unsafe extern "C" fn kcmp_ptr(v1: *mut c_void, v2: *mut c_void, type: kcmp_type) -> c_int {
    long t1, t2;
    t1 = kptr_obfuscate((long)v1, type);
    t2 = kptr_obfuscate((long)v2, type);
    return (t1 < t2) | ((t1 > t2) << 1);
    }
// The caller must have pinned the task
#[no_mangle]
pub unsafe extern "C" fn get_file_raw_ptr() {
    let mut file = core::ptr::null_mut();
    file = fget_task(task, idx);
    if (file) {
    fput(file);
    }
    return file;
    }
#[no_mangle]
unsafe extern "C" fn kcmp_unlock(l1: *mut rw_semaphore, l2: *mut rw_semaphore) {
    if (likely(l2 != l1)) {
    up_read(l2);
    }
    up_read(l1);
    }
#[no_mangle]
unsafe extern "C" fn kcmp_lock(l1: *mut rw_semaphore, l2: *mut rw_semaphore) -> c_int {
    let mut err = 0;
    if (l2 > l1) {
    swap(l1, l2);
    }
    err = down_read_killable(l1);
    if (!err && likely(l1 != l2)) {
    err = down_read_killable_nested(l2, SINGLE_DEPTH_NESTING);
    if (err) {
    up_read(l1);
    }
    }
    return err;
    }

#[no_mangle]
pub unsafe extern "C" fn kcmp_epoll_target() {
    struct file *filp, *filp_epoll, *filp_tgt;
    let mut slot;
    if (copy_from_user(&slot, uslot, sizeof(slot))) {
    return -EFAULT;
    }
    filp = get_file_raw_ptr(task1, idx1);
    if (!filp) {
    return -EBADF;
    }
    filp_epoll = fget_task(task2, slot.efd);
    if (!filp_epoll) {
    return -EBADF;
    }
    filp_tgt = get_epoll_tfile_raw_ptr(filp_epoll, slot.tfd, slot.toff);
    fput(filp_epoll);
    if (IS_ERR(filp_tgt)) {
    return PTR_ERR(filp_tgt);
    }
    return kcmp_ptr(filp, filp_tgt, KCMP_FILE);
    }

#[no_mangle]
pub unsafe extern "C" fn kcmp_epoll_target() {
    return -EOPNOTSUPP;
    }

#[no_mangle]
pub unsafe extern "C" fn sys_kcmp() {
    struct task_struct *task1, *task2;
    let mut ret = 0;
    rcu_read_lock();
//
// Tasks are looked up in caller's PID namespace only.
//
    task1 = find_task_by_vpid(pid1);
    task2 = find_task_by_vpid(pid2);
    if (unlikely(!task1 || !task2)) {
    goto err_no_task;
    }
    get_task_struct(task1);
    get_task_struct(task2);
    rcu_read_unlock();
//
// One should have enough rights to inspect task details.
//
    ret = kcmp_lock(&task1.signal.exec_update_lock,
    &task2.signal.exec_update_lock);
    if (ret) {
    goto err;
    }
    if (!ptrace_may_access(task1, PTRACE_MODE_READ_REALCREDS) ||
    !ptrace_may_access(task2, PTRACE_MODE_READ_REALCREDS)) {
    ret = -EPERM;
    goto err_unlock;
    }
    match (type) {
    KCMP_FILE => { {
    struct file *filp1, *filp2;
    filp1 = get_file_raw_ptr(task1, idx1);
    filp2 = get_file_raw_ptr(task2, idx2);
    if (filp1 && filp2) {
    ret = kcmp_ptr(filp1, filp2, KCMP_FILE);
    }
    else {
    ret = -EBADF;
    }
    break;
    }
    KCMP_VM => {
    ret = kcmp_ptr(task1.mm, task2.mm, KCMP_VM);
    break;
    KCMP_FILES => {
    ret = kcmp_ptr(task1.files, task2.files, KCMP_FILES);
    break;
    KCMP_FS => {
    ret = kcmp_ptr(task1.real_fs, task2.real_fs, KCMP_FS);
    break;
    KCMP_SIGHAND => {
    ret = kcmp_ptr(task1.sighand, task2.sighand, KCMP_SIGHAND);
    break;
    KCMP_IO => {
    ret = kcmp_ptr(task1.io_context, task2.io_context, KCMP_IO);
    break;
    KCMP_SYSVSEM => {

    ret = kcmp_ptr(task1.sysvsem.undo_list,
    task2.sysvsem.undo_list,
    KCMP_SYSVSEM);

    ret = -EOPNOTSUPP;

    break;
    KCMP_EPOLL_TFD => {
    ret = kcmp_epoll_target(task1, task2, idx1, idx2);
    break;
    _ => {
    ret = -EINVAL;
    break;
    }
    err_unlock:
    kcmp_unlock(&task1.signal.exec_update_lock,
    &task2.signal.exec_update_lock);
    err:
    put_task_struct(task1);
    put_task_struct(task2);
    return ret;
    err_no_task:
    rcu_read_unlock();
    return -ESRCH;
    }
#[no_mangle]
unsafe extern "C" fn kcmp_cookies_init() -> __init int {
    let mut i = 0;
    get_random_bytes(cookies, sizeof(cookies));
    for (i = 0; i < KCMP_TYPES; i++)
    cookies[i][1] |= (~(~0UL >>  1) | 1);
    return 0;
    }
// arch_initcall;
}
}
}
}
}
}
}
}
}