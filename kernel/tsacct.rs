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
    void bacct_add_tsk(struct user_namespace *user_ns,
    struct pid_namespace *pid_ns,
    struct taskstats *stats, struct task_struct *tsk)
    {
    const struct cred *tcred;
    u64 utime, stime, utimescaled, stimescaled;
    u64 now_ns, delta;
    time64_t btime;
    BUILD_BUG_ON(TS_COMM_LEN < TASK_COMM_LEN);
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
    if (tsk.flags & PF_EXITING)
    stats.ac_exitcode = tsk.exit_code;
    if (thread_group_leader(tsk) && (tsk.flags & PF_FORKNOEXEC))
    stats.ac_flag |= AFORK;
    if (tsk.flags & PF_SUPERPRIV)
    stats.ac_flag |= ASU;
    if (tsk.flags & PF_DUMPCORE)
    stats.ac_flag |= ACORE;
    if (tsk.flags & PF_SIGNALED)
    stats.ac_flag |= AXSIG;
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
    struct mm_struct *mm;
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

    static void __acct_update_integrals(struct task_struct *tsk,
    u64 utime, u64 stime)
    {
    u64 time, delta;
    if (unlikely(!tsk.mm || (tsk.flags & PF_KTHREAD)))
    return;
    time = stime + utime;
    delta = time - tsk.acct_timexpd;
    if (delta < TICK_NSEC)
    return;
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
    unsigned long flags;
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
