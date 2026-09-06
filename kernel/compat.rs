//! Automatically rewritten from C to Rust
//! Source: kernel/compat.c
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


























// SPDX-License-Identifier: GPL-2.0-only
//
// linux/kernel/compat.c
//
// Kernel compatibililty routines for e.g. 32 bit syscall support
// on 64 bit kernels.
//
// Copyright (C) 2002-2003 Stephen Rothwell, IBM Corporation
//

//
// sys_sigprocmask SIG_SETMASK sets the first (compat) word of the
// blocked set of signals to the supplied signal set
//
#[no_mangle]
pub unsafe extern "C" fn compat_sig_setmask(blocked: *mut sigset_t, set: compat_sigset_word) {
    memcpy(blocked.sig, &set, sizeof(set));
    }
#[no_mangle]
pub unsafe extern "C" fn sys_sigprocmask() {
    old_sigset_t old_set, new_set;
    let mut new_blocked;
    old_set = current.blocked.sig[0];
    if (nset) {
    if (get_user(new_set, nset)) {
    return -EFAULT;
    }
    new_set &= ~(sigmask(SIGKILL) | sigmask(SIGSTOP));
    new_blocked = current.blocked;
    match (how) {
    SIG_BLOCK => {
    sigaddsetmask(&new_blocked, new_set);
    break;
    SIG_UNBLOCK => {
    sigdelsetmask(&new_blocked, new_set);
    break;
    SIG_SETMASK => {
    compat_sig_setmask(&new_blocked, new_set);
    break;
    _ => {
    return -EINVAL;
    }
    set_current_blocked(&new_blocked);
    }
    if (oset) {
    if (put_user(old_set, oset)) {
    return -EFAULT;
    }
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn put_compat_rusage(r: *const rusage, ru: *mut compat_rusage __user) -> c_int {
    let mut r32;
    memset(&r32, 0, sizeof(r32));
    r32.ru_utime.tv_sec = r.ru_utime.tv_sec;
    r32.ru_utime.tv_usec = r.ru_utime.tv_usec;
    r32.ru_stime.tv_sec = r.ru_stime.tv_sec;
    r32.ru_stime.tv_usec = r.ru_stime.tv_usec;
    r32.ru_maxrss = r.ru_maxrss;
    r32.ru_ixrss = r.ru_ixrss;
    r32.ru_idrss = r.ru_idrss;
    r32.ru_isrss = r.ru_isrss;
    r32.ru_minflt = r.ru_minflt;
    r32.ru_majflt = r.ru_majflt;
    r32.ru_nswap = r.ru_nswap;
    r32.ru_inblock = r.ru_inblock;
    r32.ru_oublock = r.ru_oublock;
    r32.ru_msgsnd = r.ru_msgsnd;
    r32.ru_msgrcv = r.ru_msgrcv;
    r32.ru_nsignals = r.ru_nsignals;
    r32.ru_nvcsw = r.ru_nvcsw;
    r32.ru_nivcsw = r.ru_nivcsw;
    if (copy_to_user(ru, &r32, sizeof(r32))) {
    return -EFAULT;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn compat_get_user_cpu_mask() {
    unsigned long *k;
    if (len < cpumask_size()) {
    memset(new_mask, 0, cpumask_size());
    }
#[no_mangle]
pub unsafe extern "C" fn if(cpumask_size(): len >) -> else {
    else if (len > cpumask_size())
    len = cpumask_size();
    k = cpumask_bits(new_mask);
    return compat_get_bitmap(k, user_mask_ptr, len * 8);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_sched_setaffinity() {
    let mut new_mask;
    let mut retval = 0;
    if (!alloc_cpumask_var(&new_mask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    retval = compat_get_user_cpu_mask(user_mask_ptr, len, new_mask);
    if (retval) {
    goto out;
    }
    retval = sched_setaffinity(pid, new_mask);
    out:
    free_cpumask_var(new_mask);
    return retval;
    }
#[no_mangle]
pub unsafe extern "C" fn sys_sched_getaffinity() {
    let mut ret = 0;
    let mut mask;
    if ((len * BITS_PER_BYTE) < nr_cpu_ids) {
    return -EINVAL;
    }
    if (len & (sizeof(compat_ulong_t)-1)) {
    return -EINVAL;
    }
    if (!zalloc_cpumask_var(&mask, GFP_KERNEL)) {
    return -ENOMEM;
    }
    ret = sched_getaffinity(pid, mask);
    if (ret == 0) {
pub static mut retlen: c_uint = min(len, cpumask_size());
    if (compat_put_bitmap(user_mask_ptr, cpumask_bits(mask), retlen * 8)) {
    ret = -EFAULT;
    }
    else {
    ret = retlen;
    }
    }
    free_cpumask_var(mask);
    return ret;
    }
//
// We currently only need the following fields from the sigevent
// structure: sigev_value, sigev_signo, sig_notify and (sometimes
// sigev_notify_thread_id).  The others are handled in user mode.
// We also assume that copying sigev_value.sival_int is sufficient
// to keep all the bits of sigev_value.sival_ptr intact.
//
#[no_mangle]
pub unsafe extern "C" fn get_compat_sigevent() {
    memset(event, 0, sizeof(*event));
    return (!access_ok(u_event, sizeof(*u_event)) ||
    __get_user(event.sigev_value.sival_int,
    &u_event.sigev_value.sival_int) ||
    __get_user(event.sigev_signo, &u_event.sigev_signo) ||
    __get_user(event.sigev_notify, &u_event.sigev_notify) ||
    __get_user(event.sigev_notify_thread_id,
    &u_event.sigev_notify_thread_id))
    ? -EFAULT : 0;
    }
#[no_mangle]
pub unsafe extern "C" fn compat_get_bitmap() {
    let mut nr_compat_longs = 0;
// align bitmap up to nearest compat_long_t boundary
    bitmap_size = ALIGN(bitmap_size, BITS_PER_COMPAT_LONG);
    nr_compat_longs = BITS_TO_COMPAT_LONGS(bitmap_size);
    if (!user_read_access_begin(umask, bitmap_size / 8)) {
    return -EFAULT;
    }
    while (nr_compat_longs > 1) {
    compat_ulong_t l1, l2;
    unsafe_get_user(l1, umask++, Efault);
    unsafe_get_user(l2, umask++, Efault);
// mask++ = ((unsigned long)l2 << BITS_PER_COMPAT_LONG) | l1;
    nr_compat_longs -= 2;
    }
    if (nr_compat_longs) {
    unsafe_get_user(*mask, umask++, Efault);
    }
    user_read_access_end();
    return 0;
    Efault:
    user_read_access_end();
    return -EFAULT;
    }
#[no_mangle]
pub unsafe extern "C" fn compat_put_bitmap() {
    let mut nr_compat_longs = 0;
// align bitmap up to nearest compat_long_t boundary
    bitmap_size = ALIGN(bitmap_size, BITS_PER_COMPAT_LONG);
    nr_compat_longs = BITS_TO_COMPAT_LONGS(bitmap_size);
    if (!user_write_access_begin(umask, bitmap_size / 8)) {
    return -EFAULT;
    }
    while (nr_compat_longs > 1) {
pub static mut m: c_ulong = *mask++;
    unsafe_put_user((compat_ulong_t)m, umask++, Efault);
    unsafe_put_user(m >> BITS_PER_COMPAT_LONG, umask++, Efault);
    nr_compat_longs -= 2;
    }
    if (nr_compat_longs) {
    unsafe_put_user((compat_ulong_t)*mask, umask++, Efault);
    }
    user_write_access_end();
    return 0;
    Efault:
    user_write_access_end();
    return -EFAULT;
    }
#[no_mangle]
pub unsafe extern "C" fn get_compat_sigset() {

    let mut v;
    if (copy_from_user(&v, compat, sizeof(compat_sigset_t))) {
    return -EFAULT;
    }
    match (_NSIG_WORDS) {
    4 => { set.sig[3] = v.sig[6] | (((long)v.sig[7]) << 32 );
    fallthrough;
    3 => { set.sig[2] = v.sig[4] | (((long)v.sig[5]) << 32 );
    fallthrough;
    2 => { set.sig[1] = v.sig[2] | (((long)v.sig[3]) << 32 );
    fallthrough;
    1 => { set.sig[0] = v.sig[0] | (((long)v.sig[1]) << 32 );
    }

    if (copy_from_user(set, compat, sizeof(compat_sigset_t))) {
    return -EFAULT;
    }

    return 0;
    }
// EXPORT_SYMBOL_GPL;

}
}
}
}
}
}
}
}
}