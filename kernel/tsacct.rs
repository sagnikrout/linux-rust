//! Automatically rewritten from C to Rust
//! Source: kernel/tsacct.c
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



// SPDX-License-Identifier: GPL-2.0-or-later
//
// tsacct.c - System accounting over taskstats interface
//
// Copyright (C) Jay Lan,	<jlan@sgi.com>
//

//
// fill in basic accounting fields
//
#[no_mangle]
pub unsafe extern "C" fn bacct_add_tsk(user_ns: *mut user_namespace, pid_ns: *mut pid_namespace, stats: *mut taskstats, tsk: *mut task_struct) {
pub static mut tcred: *mut c_void = core::ptr::null_mut();
    u64 utime, stime, utimescaled, stimescaled;
    u64 now_ns, delta;
    let mut btime;
    BUILD_BUG_ON!(TS_COMM_LEN < TASK_COMM_LEN);
// calculate task elapsed time in nsec
    now_ns = ktime_get_ns();
// store whole group time first
    delta = now_ns - tsk.group_leader.start_time;
// Convert to micro seconds
    do_div(delta, NSEC_PER_USEC);
    stats.ac_tgetime = delta;
    delta = now_ns - tsk.start_time;
    do_div(delta, NSEC_PER_USEC);
    stats.ac_etime = delta;
// Convert to seconds for btime (note y2106 limit)
    btime = ktime_get_real_seconds() - div_u64(delta, USEC_PER_SEC);
    stats.ac_btime = clamp_t(time64_t, btime, 0, U32_MAX);
    stats.ac_btime64 = btime;
    if (tsk.flags & PF_EXITING) {
    stats.ac_exitcode = tsk.exit_code;
    }
    if (thread_group_leader(tsk) && (tsk.flags & PF_FORKNOEXEC)) {
    stats.ac_flag |= AFORK;
    }
    if (tsk.flags & PF_SUPERPRIV) {
    stats.ac_flag |= ASU;
    }
    if (tsk.flags & PF_DUMPCORE) {
    stats.ac_flag |= ACORE;
    }
    if (tsk.flags & PF_SIGNALED) {
    stats.ac_flag |= AXSIG;
    }
    stats.ac_nice	 = task_nice(tsk);
    stats.ac_sched	 = tsk.policy;
    stats.ac_pid	 = task_pid_nr_ns(tsk, pid_ns);
    stats.ac_tgid   = task_tgid_nr_ns(tsk, pid_ns);
    stats.ac_ppid	 = task_ppid_nr_ns(tsk, pid_ns);
    rcu_read_lock();
    tcred = __task_cred(tsk);
    stats.ac_uid	 = from_kuid_munged(user_ns, tcred.uid);
    stats.ac_gid	 = from_kgid_munged(user_ns, tcred.gid);
    rcu_read_unlock();
    task_cputime(tsk, &utime, &stime);
    stats.ac_utime = div_u64(utime, NSEC_PER_USEC);
    stats.ac_stime = div_u64(stime, NSEC_PER_USEC);
    task_cputime_scaled(tsk, &utimescaled, &stimescaled);
    stats.ac_utimescaled = div_u64(utimescaled, NSEC_PER_USEC);
    stats.ac_stimescaled = div_u64(stimescaled, NSEC_PER_USEC);
    stats.ac_minflt = tsk.min_flt;
    stats.ac_majflt = tsk.maj_flt;
    strscpy_pad(stats.ac_comm, tsk.comm);
    }

pub const KB: c_int = 1024;

//
// fill in extended accounting fields
//
#[no_mangle]
pub unsafe extern "C" fn xacct_add_tsk(stats: *mut taskstats, p: *mut task_struct) {
pub static mut mm: *mut c_void = core::ptr::null_mut();
// convert pages-nsec/1024 to Mbyte-usec, see __acct_update_integrals
    stats.coremem = p.acct_rss_mem1 * PAGE_SIZE;
    do_div(stats.coremem, 1000 * KB);
    stats.virtmem = p.acct_vm_mem1 * PAGE_SIZE;
    do_div(stats.virtmem, 1000 * KB);
    mm = get_task_mm(p);
    if (mm) {
// adjust to KB unit
    stats.hiwater_rss   = get_mm_hiwater_rss(mm) * PAGE_SIZE / KB;
    stats.hiwater_vm    = get_mm_hiwater_vm(mm)  * PAGE_SIZE / KB;
    mmput(mm);
    }
    stats.read_char	= p.ioac.rchar & KB_MASK;
    stats.write_char	= p.ioac.wchar & KB_MASK;
    stats.read_syscalls	= p.ioac.syscr & KB_MASK;
    stats.write_syscalls	= p.ioac.syscw & KB_MASK;

    stats.read_bytes	= p.ioac.read_bytes & KB_MASK;
    stats.write_bytes	= p.ioac.write_bytes & KB_MASK;
    stats.cancelled_write_bytes = p.ioac.cancelled_write_bytes & KB_MASK;

    stats.read_bytes	= 0;
    stats.write_bytes	= 0;
    stats.cancelled_write_bytes = 0;

    }

#[no_mangle]
pub unsafe extern "C" fn __acct_update_integrals(tsk: *mut task_struct, utime: u64, stime: u64) {
    u64 time, delta;
    if (unlikely(!tsk.mm || (tsk.flags & PF_KTHREAD))) {
    return;
    }
    time = stime + utime;
    delta = time - tsk.acct_timexpd;
    if (delta < TICK_NSEC) {
    return;
    }
    tsk.acct_timexpd = time;
//
// Divide by 1024 to avoid overflow, and to avoid division.
// The final unit reported to userspace is Mbyte-usecs,
// the rest of the math is done in xacct_add_tsk.
//
    tsk.acct_rss_mem1 += delta * get_mm_rss(tsk.mm) >> 10;
    tsk.acct_vm_mem1 += delta * READ_ONCE(tsk.mm.total_vm) >> 10;
    }
//
// acct_update_integrals - update mm integral fields in task_struct
// @tsk: task_struct for accounting
//
#[no_mangle]
pub unsafe extern "C" fn acct_update_integrals(tsk: *mut task_struct) {
    u64 utime, stime;
    let mut flags = 0;
    local_irq_save(flags);
    task_cputime(tsk, &utime, &stime);
    __acct_update_integrals(tsk, utime, stime);
    local_irq_restore(flags);
    }
//
// acct_account_cputime - update mm integral after cputime update
// @tsk: task_struct for accounting
//
#[no_mangle]
pub unsafe extern "C" fn acct_account_cputime(tsk: *mut task_struct) {
    __acct_update_integrals(tsk, tsk.utime, tsk.stime);
    }
//
// acct_clear_integrals - clear the mm integral fields in task_struct
// @tsk: task_struct whose accounting fields are cleared
//
#[no_mangle]
pub unsafe extern "C" fn acct_clear_integrals(tsk: *mut task_struct) {
    tsk.acct_timexpd = 0;
    tsk.acct_rss_mem1 = 0;
    tsk.acct_vm_mem1 = 0;
    }