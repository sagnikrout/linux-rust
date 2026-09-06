//! Automatically rewritten from C to Rust
//! Source: kernel/latencytop.c
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
// latencytop.c: Latency display infrastructure
//
// (C) Copyright 2008 Intel Corporation
// Author: Arjan van de Ven <arjan@linux.intel.com>
//
// CONFIG_LATENCYTOP enables a kernel latency tracking infrastructure that is
// used by the "latencytop" userspace tool. The latency that is tracked is not
// the 'traditional' interrupt latency (which is primarily caused by something
// else consuming CPU), but instead, it is the latency an application encounters
// because the kernel sleeps on its behalf for various reasons.
//
// This code tracks 2 levels of statistics:
// 1) System level latency
// 2) Per process latency
//
// The latency is stored in fixed sized data structures in an accumulated form;
// if the "same" latency cause is hit twice, this will be tracked as one entry
// in the data structure. Both the count, total accumulated latency and maximum
// latency are tracked in this data structure. When the fixed size structure is
// full, no new causes are tracked until the buffer is flushed by writing to
// the /proc file; the userspace tool does this on a regular basis.
//
// A latency cause is identified by a stringified backtrace at the point that
// the scheduler gets invoked. The userland tool will use this string to
// identify the cause of the latency in human readable form.
//
// The information is exported via /proc/latency_stats and /proc/<pid>/latency.
// These files look like this:
//
// Latency Top version : v0.1
// 70 59433 4897 i915_irq_wait drm_ioctl vfs_ioctl do_vfs_ioctl sys_ioctl
// |    |    |    |
// |    |    |    +----> the stringified backtrace
// |    |    +---------> The maximum latency for this entry in microseconds
// |    +--------------> The accumulated latency for this entry (microseconds)
// +-------------------> The number of times this entry is hit
//
// (note: the average latency is the accumulated latency divided by the number
// of times)
//
// static DEFINE_RAW_SPINLOCK(latency_lock);
pub const MAXLR: c_int = 128;
    static struct latency_record latency_record[MAXLR];
    let mut latencytop_enabled = 0;

#[no_mangle]
pub unsafe extern "C" fn sysctl_latencytop(table: *mut ctl_table, write: c_int, buffer: *mut c_void, lenp: *mut size_t, ppos: *mut loff_t) -> c_int {
    let mut err = 0;
    err = proc_dointvec(table, write, buffer, lenp, ppos);
    if (latencytop_enabled) {
    force_schedstat_enabled();
    }
    return err;
    }
pub static mut ctl_table: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn clear_tsk_latency_tracing(p: *mut task_struct) {
    let mut flags = 0;
    raw_spin_lock_irqsave(&latency_lock, flags);
    memset(&p.latency_record, 0, sizeof!(p.latency_record));
    p.latency_record_count = 0;
    raw_spin_unlock_irqrestore(&latency_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn clear_global_latency_tracing() {
    let mut flags = 0;
    raw_spin_lock_irqsave(&latency_lock, flags);
    memset(&latency_record, 0, sizeof!(latency_record));
    raw_spin_unlock_irqrestore(&latency_lock, flags);
    }
    static void __sched
    account_global_scheduler_latency(task_struct *tsk, latency_record *lat)
    {
pub static mut firstnonnull: c_int = 0;
    let mut i = 0;
// skip kernel threads for now
    if (!tsk.mm) {
    return;
    }
    while (i < MAXLR) {
    int q, same = 1;
// Nothing stored:
    if (!latency_record[i].backtrace[0]) {
    if (firstnonnull > i) {
    firstnonnull = i;
    }
    continue;
    }
    while (q < LT_BACKTRACEDEPTH) {
pub static mut record: c_ulong = 0;
    if (latency_record[i].backtrace[q] != record) {
    same = 0;
    break;
    }
// 0 entry marks end of backtrace:
    if (!record) {
    break;
    }
    }
    if (same) {
    latency_record[i].count += 1;
    latency_record[i].time += lat.time;
    if (lat.time > latency_record[i].max) {
    latency_record[i].max = lat.time;
    }
    return;
    }
    }
    i = firstnonnull;
    if (i >= MAXLR) {
    return;
    }
// Allocted a new one:
    memcpy(&latency_record[i], lat, sizeof!(latency_record));
    }
//
// __account_scheduler_latency - record an occurred latency
// @tsk: the task struct of the task hitting the latency
// @usecs: the duration of the latency in microseconds
// @inter: 1 if the sleep was interruptible, 0 if uninterruptible
//
// This function is the main entry point for recording latency entries
// as called by the scheduler.
//
// This function has a few special cases to deal with normal 'non-latency'
// sleeps: specifically, interruptible sleep longer than 5 msec is skipped
// since this usually is caused by waiting for events via select() and co.
//
// Negative latencies (caused by time going backwards) are also explicitly
// skipped.
//
    void __sched
    __account_scheduler_latency(task_struct *tsk, int usecs, int inter)
    {
    let mut flags = 0;
    let mut i = 0;
    let mut q = 0;
pub static mut lat: usize = 0;
// Long interruptible waits are generally user requested...
    if (inter && usecs > 5000) {
    return;
    }
// Negative sleeps are time going backwards
// Zero-time sleeps are non-interesting
    if (usecs <= 0) {
    return;
    }
    memset(&lat, 0, sizeof!(lat));
    lat.count = 1;
    lat.time = usecs;
    lat.max = usecs;
    stack_trace_save_tsk(tsk, lat.backtrace, LT_BACKTRACEDEPTH, 0);
    raw_spin_lock_irqsave(&latency_lock, flags);
    account_global_scheduler_latency(tsk, &lat);
    while (i < tsk.latency_record_count) {
pub static mut mylat: *mut c_void = core::ptr::null_mut();
pub static mut same: c_int = 1;
    mylat = &tsk.latency_record[i];
    while (q < LT_BACKTRACEDEPTH) {
pub static mut record: c_ulong = 0;
    if (mylat.backtrace[q] != record) {
    same = 0;
    break;
    }
// 0 entry is end of backtrace
    if (!record) {
    break;
    }
    }
    if (same) {
    mylat.count += 1;
    mylat.time += lat.time;
    if (lat.time > mylat.max) {
    mylat.max = lat.time;
    }
// goto;
    }
    }
//
// short term hack; if we're > 32 we stop; future we recycle:
//
    if (tsk.latency_record_count >= LT_SAVECOUNT) {
// goto;
    }
// Allocated a new one:
    i = tsk.latency_record_count += 1;
    memcpy(&tsk.latency_record[i], &lat, sizeof!(latency_record));
// label;
    raw_spin_unlock_irqrestore(&latency_lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn lstats_show(m: *mut seq_file, v: *mut c_void) -> c_int {
    let mut i = 0;
    seq_puts(m, "Latency Top version : v0.1\n");
    while (i < MAXLR) {
    let mut lr = &latency_record[i];
    if (lr.backtrace[0]) {
    let mut q = 0;
    seq_printf(m, "%i %lu %lu",
    lr.count, lr.time, lr.max);
    while (q < LT_BACKTRACEDEPTH) {
pub static mut bt: c_ulong = 0;
    if (!bt) {
    break;
    }
    seq_printf(m, " %ps", bt);
    }
    seq_puts(m, "\n");
    }
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lstats_write(file: *mut file, buf: *mut c_char, count: size_t, offs: *mut loff_t) -> ssize_t {
    clear_global_latency_tracing();
    return count;
    }
#[no_mangle]
unsafe extern "C" fn lstats_open(inode: *mut inode, filp: *mut file) -> c_int {
    return single_open(filp, lstats_show, core::ptr::null_mut());
    }
pub static mut proc_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn init_lstats_procfs() -> c_int {
    proc_create("latency_stats", 0644, core::ptr::null_mut(), &lstats_proc_ops);

    register_sysctl_init("kernel", latencytop_sysctl);

    return 0;
    }
    device_initcall!(init_lstats_procfs);