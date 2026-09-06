//! Automatically rewritten from C to Rust
//! Source: kernel/time/jiffies.c
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
// This file contains the jiffies based clocksource.
//
// Copyright (C) 2004, 2005 IBM, John Stultz (johnstul@us.ibm.com)
//

#[no_mangle]
unsafe extern "C" fn jiffies_read(cs: *mut clocksource) -> u64 {
    return (u64) jiffies;
    }
//
// The Jiffies based clocksource is the lowest common
// denominator clock source which should function on
// all systems. It has the same coarse resolution as
// the timer interrupt frequency HZ and it suffers
// inaccuracies caused by missed or lost timer
// interrupts and the inability for the timer
// interrupt hardware to accurately tick at the
// requested HZ value. It is also not recommended
// for "tick-less" systems.
//
pub static mut clocksource: usize = 0;
    __cacheline_aligned_in_smp DEFINE_RAW_SPINLOCK(jiffies_lock);
    __cacheline_aligned_in_smp seqcount_raw_spinlock_t jiffies_seq =
    SEQCNT_RAW_SPINLOCK_ZERO(jiffies_seq, &jiffies_lock);

#[no_mangle]
pub unsafe extern "C" fn get_jiffies_64() -> u64 {
    let mut seq = 0;
    let mut ret = 0;
    do {
    seq = read_seqcount_begin(&jiffies_seq);
    ret = jiffies_64;
    } while (read_seqcount_retry(&jiffies_seq, seq));
    return ret;
    }
    EXPORT_SYMBOL(get_jiffies_64);

    EXPORT_SYMBOL(jiffies);
    static bool cs_jiffies_registered __initdata;
#[no_mangle]
pub unsafe extern "C" fn clocksource_default_clock() -> *mut clocksource  __init __weak {
    if (!cs_jiffies_registered) {
    __clocksource_register(&clocksource_jiffies);
    cs_jiffies_registered = true;
    }
    return &clocksource_jiffies;
    }
pub static mut refined_jiffies: usize = 0;
#[no_mangle]
pub unsafe extern "C" fn register_refined_jiffies(cycles_per_second: c_long)  {
    u64 nsec_per_tick, shift_hz;
    let mut cycles_per_tick = 0;
    refined_jiffies = clocksource_jiffies;
    refined_jiffies.name = "refined-jiffies";
    refined_jiffies.rating += 1;
// Calc cycles per tick
    cycles_per_tick = (cycles_per_second + HZ/2)/HZ;
// shift_hz stores hz<<8 for extra accuracy
    shift_hz = (u64)cycles_per_second << 8;
    shift_hz += cycles_per_tick/2;
    do_div(shift_hz, cycles_per_tick);
// Calculate nsec_per_tick using shift_hz
    nsec_per_tick = (u64)NSEC_PER_SEC << 8;
    nsec_per_tick += (u32)shift_hz/2;
    do_div(nsec_per_tick, (u32)shift_hz);
    refined_jiffies.mult = ((u32)nsec_per_tick) << JIFFIES_SHIFT;
    __clocksource_register(&refined_jiffies);
    }

#[no_mangle]
unsafe extern "C" fn mult_hz(val: c_ulong) -> c_ulong {
    return val * HZ;
    }
#[no_mangle]
unsafe extern "C" fn div_hz(val: c_ulong) -> c_ulong {
    return val / HZ;
    }
#[no_mangle]
unsafe extern "C" fn sysctl_u2k_int_conv_hz(negp: *const bool, u_ptr: *const c_ulong, k_ptr: *mut c_int) -> c_int {
    return proc_int_u2k_conv_uop(u_ptr, k_ptr, negp, mult_hz);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_k2u_int_conv_hz(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *const c_int) -> c_int {
    return proc_int_k2u_conv_kop(u_ptr, k_ptr, negp, div_hz);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_u2k_int_conv_userhz(negp: *const bool, u_ptr: *const c_ulong, k_ptr: *mut c_int) -> c_int {
    return proc_int_u2k_conv_uop(u_ptr, k_ptr, negp, clock_t_to_jiffies);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_jiffies_to_clock_t(val: c_ulong) -> c_ulong {
    return jiffies_to_clock_t(val);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_k2u_int_conv_userhz(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *const c_int) -> c_int {
    return proc_int_k2u_conv_kop(u_ptr, k_ptr, negp, sysctl_jiffies_to_clock_t);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_msecs_to_jiffies(val: c_ulong) -> c_ulong {
    return msecs_to_jiffies(val);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_u2k_int_conv_ms(negp: *const bool, u_ptr: *const c_ulong, k_ptr: *mut c_int) -> c_int {
    return proc_int_u2k_conv_uop(u_ptr, k_ptr, negp, sysctl_msecs_to_jiffies);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_jiffies_to_msecs(val: c_ulong) -> c_ulong {
    return jiffies_to_msecs(val);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_k2u_int_conv_ms(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *const c_int) -> c_int {
    return proc_int_k2u_conv_kop(u_ptr, k_ptr, negp, sysctl_jiffies_to_msecs);
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_int_conv_jiffies(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, false,
    sysctl_u2k_int_conv_hz, sysctl_k2u_int_conv_hz);
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_int_conv_userhz_jiffies(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, false,
    sysctl_u2k_int_conv_userhz,
    sysctl_k2u_int_conv_userhz);
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_int_conv_ms_jiffies(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, false,
    sysctl_u2k_int_conv_ms, sysctl_k2u_int_conv_ms);
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_int_conv_ms_jiffies_minmax(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_int_conv(negp, u_ptr, k_ptr, dir, tbl, false,
    sysctl_u2k_int_conv_ms, sysctl_k2u_int_conv_ms);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_u2k_ulong_conv_ms(u_ptr: *const c_ulong, k_ptr: *mut c_ulong) -> c_int {
    return proc_ulong_u2k_conv_uop(u_ptr, k_ptr, sysctl_msecs_to_jiffies);
    }
#[no_mangle]
unsafe extern "C" fn sysctl_k2u_ulong_conv_ms(u_ptr: *mut c_ulong, k_ptr: *const c_ulong) -> c_int {
    return proc_ulong_k2u_conv_kop(u_ptr, k_ptr, sysctl_jiffies_to_msecs);
    }
#[no_mangle]
pub unsafe extern "C" fn do_proc_ulong_conv_ms_jiffies(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_ulong, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return proc_ulong_conv(u_ptr, k_ptr, dir, tbl, false,
    sysctl_u2k_ulong_conv_ms, sysctl_k2u_ulong_conv_ms);
    }

#[no_mangle]
#[no_mangle]
// duplicate fn: do_proc_int_conv_jiffies
pub unsafe extern "C" fn do_proc_int_conv_jiffies_dup(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: do_proc_int_conv_userhz_jiffies
pub unsafe extern "C" fn do_proc_int_conv_userhz_jiffies_dup(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: do_proc_int_conv_ms_jiffies
pub unsafe extern "C" fn do_proc_int_conv_ms_jiffies_dup(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: do_proc_int_conv_ms_jiffies_minmax
pub unsafe extern "C" fn do_proc_int_conv_ms_jiffies_minmax_dup(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_int, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return -ENOSYS;
    }
#[no_mangle]
#[no_mangle]
// duplicate fn: do_proc_ulong_conv_ms_jiffies
pub unsafe extern "C" fn do_proc_ulong_conv_ms_jiffies_dup(negp: *mut bool, u_ptr: *mut c_ulong, k_ptr: *mut c_ulong, dir: c_int, tbl: *mut ctl_table) -> c_int {
    return -ENOSYS;
    }

//
// proc_dointvec_jiffies - read a vector of integers as seconds
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) integer
// values from/to the user buffer, treated as an ASCII string.
// The values read are assumed to be in seconds, and are converted into
// jiffies.
//
// Returns 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_jiffies(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return proc_dointvec_conv(table, dir, buffer, lenp, ppos,
    do_proc_int_conv_jiffies);
    }
    EXPORT_SYMBOL(proc_dointvec_jiffies);
//
// proc_dointvec_userhz_jiffies - read a vector of integers as 1/USER_HZ seconds
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: pointer to the file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) integer
// values from/to the user buffer, treated as an ASCII string.
// The values read are assumed to be in 1/USER_HZ seconds, and
// are converted into jiffies.
//
// Returns 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_userhz_jiffies(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return proc_dointvec_conv(table, dir, buffer, lenp, ppos,
    do_proc_int_conv_userhz_jiffies);
    }
    EXPORT_SYMBOL(proc_dointvec_userhz_jiffies);
//
// proc_dointvec_ms_jiffies - read a vector of integers as 1 milliseconds
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: the current position in the file
//
// Reads/writes up to table->maxlen/sizeof!(unsigned int) integer
// values from/to the user buffer, treated as an ASCII string.
// The values read are assumed to be in 1/1000 seconds, and
// are converted into jiffies.
//
// Returns 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_ms_jiffies(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return proc_dointvec_conv(table, dir, buffer, lenp, ppos,
    do_proc_int_conv_ms_jiffies);
    }
    EXPORT_SYMBOL(proc_dointvec_ms_jiffies);
#[no_mangle]
pub unsafe extern "C" fn proc_dointvec_ms_jiffies_minmax(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return proc_dointvec_conv(table, dir, buffer, lenp, ppos,
    do_proc_int_conv_ms_jiffies_minmax);
    }
//
// proc_doulongvec_ms_jiffies_minmax - read a vector of millisecond values with min/max values
// @table: the sysctl table
// @dir: %TRUE if this is a write to the sysctl file
// @buffer: the user buffer
// @lenp: the size of the user buffer
// @ppos: file position
//
// Reads/writes up to table->maxlen/sizeof!(unsigned long) unsigned long
// values from/to the user buffer, treated as an ASCII string. The values
// are treated as milliseconds, and converted to jiffies when they are stored.
//
// This routine will ensure the values are within the range specified by
// table->extra1 (min) and table->extra2 (max).
//
// Returns 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn proc_doulongvec_ms_jiffies_minmax(table: *mut ctl_table, dir: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    return proc_doulongvec_conv(table, dir, buffer, lenp, ppos,
    do_proc_ulong_conv_ms_jiffies);
    }
    EXPORT_SYMBOL(proc_doulongvec_ms_jiffies_minmax);