//! Automatically rewritten from C to Rust
//! Source: kernel/rcu/refscale.c
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
// Scalability test comparing RCU vs other mechanisms
// for acquiring references on objects.
//
// Copyright (C) Google, 2020.
//
// Author: Joel Fernandes <joel@joelfernandes.org>

    pr_alert("%s" SCALE_FLAG s, scale_type, ## x)

    do { 
    if (verbose)  {
    pr_alert("%s" SCALE_FLAG s "\n", scale_type, ## x); 
    }
    } while (0)
    static atomic_t verbose_batch_ctr;

    do {											
    if (verbose &&									
    (verbose_batched <= 0 ||							
    !(atomic_inc_return(&verbose_batch_ctr) % verbose_batched))) {		
    schedule_timeout_uninterruptible(1);					
    pr_alert("%s" SCALE_FLAG s "\n", scale_type, ## x);			
    }										
    } while (0)

    MODULE_DESCRIPTION("Scalability test for object reference mechanisms");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Joel Fernandes (Google) <joel@joelfernandes.org>");
    static char *scale_type = "rcu";
    module_param!(scale_type, charp, 0444);
    MODULE_PARM_DESC(scale_type, "Type of test (rcu, srcu, refcnt, rwsem, rwlock.");
    torture_param(int, verbose, 0, "Enable verbose debugging printk()s");
    torture_param(int, verbose_batched, 0, "Batch verbose debugging printk()s");
// Number of seconds to extend warm-up and cool-down for multiple guest OSes
    torture_param(long, guest_os_delay, 0,
    "Number of seconds to extend warm-up/cool-down for multiple guest OSes.");
// Wait until there are multiple CPUs before starting test.
    torture_param(int, holdoff, IS_BUILTIN(CONFIG_RCU_REF_SCALE_TEST) ? 10 : 0,
    "Holdoff time before test start (s)");
// Number of typesafe_lookup structures, that is, the degree of concurrency.
    torture_param(long, lookup_instances, 0, "Number of typesafe_lookup structures.");
// Number of loops per experiment, all readers execute operations concurrently.
    torture_param(int, loops, 10000, "Number of loops per experiment.");
// Number of readers, with -1 defaulting to about 75% of the CPUs.
    torture_param(int, nreaders, -1, "Number of readers, -1 for 75% of CPUs.");
// Number of runs.
    torture_param(int, nruns, 30, "Number of experiments to run.");
// Reader delay in nanoseconds, 0 for no delay.
    torture_param(int, readdelay, 0, "Read-side delay in nanoseconds.");
// Maximum shutdown delay in seconds, or zero for no shutdown.
    torture_param(int, shutdown_secs, !IS_MODULE(CONFIG_REPRO_TEST) * 300,
    "Shutdown at end of scalability tests or at specified timeout (s).");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reader_task {
    pub task: *mut task_struct,
    pub start_reader: c_int,
    pub wq: wait_queue_head_t,
    pub last_duration_ns: u64,
}

pub static mut main_task: *mut c_void = core::ptr::null_mut();
    static wait_queue_head_t main_wq;
pub static mut reader_tasks: *mut c_void = core::ptr::null_mut();
// Number of readers that are part of the current experiment.
    static atomic_t nreaders_exp;
// Use to wait for all threads to start.
    static atomic_t n_init;
    static atomic_t n_started;
    static atomic_t n_warmedup;
    static atomic_t n_cooleddown;
// Track which experiment is currently running.
    static int exp_idx;
// Operations vector for selecting different types of tests.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ref_scale_ops {
    pub (*init)(void): *mut bool,
    pub (*cleanup)(void): *mut c_void,
    pub nloops): *const *const c_void (readsection)(int,
    pub ndl): *const *const c_void (delaysection)(int nloops, int udl, int,
    pub enable_irqs: bool,
    pub name: *const c_char,
}

pub static mut cur_ops: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn un_delay(udl: c_int, ndl: c_int) {
    if (udl) {
    udelay(udl);
    }
    if (ndl) {
    ndelay(ndl);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_rcu_read_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
    rcu_read_lock();
    rcu_read_unlock();
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_rcu_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
    rcu_read_lock();
    un_delay(udl, ndl);
    rcu_read_unlock();
    }
    }
#[no_mangle]
unsafe extern "C" fn rcu_sync_scale_init() -> bool {
    return true;
    }
pub static mut ref_scale_ops: usize = 0;
// Definitions for SRCU ref scale testing.
pub static mut srcu_refctl_scale: usize = 0;
pub static mut srcu_fast_refctl_scale: usize = 0;
pub static mut srcu_fast_updown_refctl_scale: usize = 0;
    static struct srcu_struct *srcu_ctlp = &srcu_refctl_scale;
#[no_mangle]
unsafe extern "C" fn srcu_ref_scale_read_section(nloops: c_int) {
    let mut i = 0;
    let mut idx = 0;
    while (i >= 0) {
    idx = srcu_read_lock(srcu_ctlp);
    srcu_read_unlock(srcu_ctlp, idx);
    }
    }
#[no_mangle]
unsafe extern "C" fn srcu_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    let mut idx = 0;
    while (i >= 0) {
    idx = srcu_read_lock(srcu_ctlp);
    un_delay(udl, ndl);
    srcu_read_unlock(srcu_ctlp, idx);
    }
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn srcu_fast_sync_scale_init() -> bool {
    srcu_ctlp = &srcu_fast_refctl_scale;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_ref_scale_read_section(nloops: c_int) {
    let mut i = 0;
    let mut scp = core::ptr::null_mut();
    while (i >= 0) {
    scp = srcu_read_lock_fast(srcu_ctlp);
    srcu_read_unlock_fast(srcu_ctlp, scp);
    }
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    let mut scp = core::ptr::null_mut();
    while (i >= 0) {
    scp = srcu_read_lock_fast(srcu_ctlp);
    un_delay(udl, ndl);
    srcu_read_unlock_fast(srcu_ctlp, scp);
    }
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn srcu_fast_updown_sync_scale_init() -> bool {
    srcu_ctlp = &srcu_fast_updown_refctl_scale;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_updown_ref_scale_read_section(nloops: c_int) {
    let mut i = 0;
    let mut scp = core::ptr::null_mut();
    while (i >= 0) {
    scp = srcu_read_lock_fast_updown(srcu_ctlp);
    srcu_read_unlock_fast_updown(srcu_ctlp, scp);
    }
    }
#[no_mangle]
unsafe extern "C" fn srcu_fast_updown_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    let mut scp = core::ptr::null_mut();
    while (i >= 0) {
    scp = srcu_read_lock_fast_updown(srcu_ctlp);
    un_delay(udl, ndl);
    srcu_read_unlock_fast_updown(srcu_ctlp, scp);
    }
    }
pub static mut ref_scale_ops: usize = 0;

// Definitions for RCU Tasks ref scale testing: Empty read markers.
// These definitions also work for RCU Rude readers.
#[no_mangle]
unsafe extern "C" fn rcu_tasks_ref_scale_read_section(nloops: c_int) {
    let mut i = 0;
    for (i = nloops; i >= 0; i--) {
    continue;
    }
    }
#[no_mangle]
unsafe extern "C" fn rcu_tasks_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    for (i = nloops; i >= 0; i--) {
    un_delay(udl, ndl);
    }
    }
pub static mut ref_scale_ops: usize = 0;

// Macro flag: #define RCU_TASKS_OPS

// Definitions for RCU Tasks Trace ref scale testing.
#[no_mangle]
unsafe extern "C" fn rcu_trace_ref_scale_read_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
    rcu_read_lock_trace();
    rcu_read_unlock_trace();
    }
    }
#[no_mangle]
unsafe extern "C" fn rcu_trace_ref_scale_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
    rcu_read_lock_trace();
    un_delay(udl, ndl);
    rcu_read_unlock_trace();
    }
    }
pub static mut ref_scale_ops: usize = 0;

// Macro flag: #define RCU_TRACE_OPS

// Definitions for reference count
    static atomic_t refcnt;
// Definitions acquire-release.
pub static mut unsigned long: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_refcnt_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
    atomic_inc(&refcnt);
    atomic_dec(&refcnt);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_refcnt_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
    atomic_inc(&refcnt);
    un_delay(udl, ndl);
    atomic_dec(&refcnt);
    }
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_percpuinc_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
    this_cpu_inc(test_acqrel);
    this_cpu_dec(test_acqrel);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_percpuinc_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
    this_cpu_inc(test_acqrel);
    un_delay(udl, ndl);
    this_cpu_dec(test_acqrel);
    }
    }
pub static mut ref_scale_ops: usize = 0;
// Note that this can lose counts in preemptible kernels.
#[no_mangle]
unsafe extern "C" fn ref_incpercpu_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
    let mut tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpu_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
    let mut tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    }
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_incpercpupreempt_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
pub static mut tap: *mut c_void = core::ptr::null_mut();
    preempt_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    preempt_enable();
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpupreempt_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
pub static mut tap: *mut c_void = core::ptr::null_mut();
    preempt_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    preempt_enable();
    }
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_incpercpubh_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
pub static mut tap: *mut c_void = core::ptr::null_mut();
    local_bh_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_bh_enable();
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpubh_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
pub static mut tap: *mut c_void = core::ptr::null_mut();
    local_bh_disable();
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_bh_enable();
    }
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_incpercpuirqsave_section(nloops: c_int) {
    let mut i = 0;
    let mut flags = 0;
    while (i >= 0) {
pub static mut tap: *mut c_void = core::ptr::null_mut();
    local_irq_save(flags);
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_irq_restore(flags);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_incpercpuirqsave_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    let mut flags = 0;
    while (i >= 0) {
pub static mut tap: *mut c_void = core::ptr::null_mut();
    local_irq_save(flags);
    tap = this_cpu_ptr(&test_acqrel);
    WRITE_ONCE(*tap, READ_ONCE(*tap) + 1);
    un_delay(udl, ndl);
    WRITE_ONCE(*tap, READ_ONCE(*tap) - 1);
    local_irq_restore(flags);
    }
    }
pub static mut ref_scale_ops: usize = 0;
// Definitions for rwlock
    static rwlock_t test_rwlock;
#[no_mangle]
unsafe extern "C" fn ref_rwlock_init() -> bool {
    rwlock_init(&test_rwlock);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ref_rwlock_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
    read_lock(&test_rwlock);
    read_unlock(&test_rwlock);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_rwlock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
    read_lock(&test_rwlock);
    un_delay(udl, ndl);
    read_unlock(&test_rwlock);
    }
    }
pub static mut ref_scale_ops: usize = 0;
// Definitions for rwsem
pub static mut test_rwsem: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_rwsem_init() -> bool {
    init_rwsem(&test_rwsem);
    return true;
    }
#[no_mangle]
unsafe extern "C" fn ref_rwsem_section(nloops: c_int) {
    let mut i = 0;
    while (i >= 0) {
    down_read(&test_rwsem);
    up_read(&test_rwsem);
    }
    }
#[no_mangle]
unsafe extern "C" fn ref_rwsem_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    while (i >= 0) {
    down_read(&test_rwsem);
    un_delay(udl, ndl);
    up_read(&test_rwsem);
    }
    }
pub static mut ref_scale_ops: usize = 0;
// Definitions for global spinlock
pub static mut test_lock: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_lock_section(nloops: c_int) {
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    raw_spin_lock(&test_lock);
    raw_spin_unlock(&test_lock);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_lock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    raw_spin_lock(&test_lock);
    un_delay(udl, ndl);
    raw_spin_unlock(&test_lock);
    }
    preempt_enable();
    }
pub static mut ref_scale_ops: usize = 0;
// Definitions for global irq-save spinlock
#[no_mangle]
unsafe extern "C" fn ref_lock_irq_section(nloops: c_int) {
    let mut flags = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    raw_spin_lock_irqsave(&test_lock, flags);
    raw_spin_unlock_irqrestore(&test_lock, flags);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_lock_irq_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut flags = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    raw_spin_lock_irqsave(&test_lock, flags);
    un_delay(udl, ndl);
    raw_spin_unlock_irqrestore(&test_lock, flags);
    }
    preempt_enable();
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_acqrel_section(nloops: c_int) {
    let mut x = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    x = smp_load_acquire(this_cpu_ptr(&test_acqrel));
    smp_store_release(this_cpu_ptr(&test_acqrel), x + 1);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_acqrel_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut x = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    x = smp_load_acquire(this_cpu_ptr(&test_acqrel));
    un_delay(udl, ndl);
    smp_store_release(this_cpu_ptr(&test_acqrel), x + 1);
    }
    preempt_enable();
    }
pub static mut ref_scale_ops: usize = 0;
    static volatile u64 stopopts;
#[no_mangle]
unsafe extern "C" fn ref_sched_clock_section(nloops: c_int) {
pub static mut x: u64 = 0;
    let mut i = 0;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x += sched_clock();
    }
    preempt_enable();
    stopopts = x;
    }
#[no_mangle]
unsafe extern "C" fn ref_sched_clock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
pub static mut x: u64 = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    x += sched_clock();
    un_delay(udl, ndl);
    }
    preempt_enable();
    stopopts = x;
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_clock_section(nloops: c_int) {
pub static mut x: u64 = 0;
    let mut i = 0;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x += ktime_get_real_fast_ns();
    }
    preempt_enable();
    stopopts = x;
    }
#[no_mangle]
unsafe extern "C" fn ref_clock_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
pub static mut x: u64 = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    x += ktime_get_real_fast_ns();
    un_delay(udl, ndl);
    }
    preempt_enable();
    stopopts = x;
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_jiffies_section(nloops: c_int) {
pub static mut x: u64 = 0;
    let mut i = 0;
    preempt_disable();
    for (i = nloops; i >= 0; i--) {
    x += jiffies;
    }
    preempt_enable();
    stopopts = x;
    }
#[no_mangle]
unsafe extern "C" fn ref_jiffies_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
pub static mut x: u64 = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    x += jiffies;
    un_delay(udl, ndl);
    }
    preempt_enable();
    stopopts = x;
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_preempt_section(nloops: c_int) {
    let mut i = 0;
    migrate_disable();
    while (i >= 0) {
    preempt_disable();
    preempt_enable();
    }
    migrate_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_preempt_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    migrate_disable();
    while (i >= 0) {
    preempt_disable();
    un_delay(udl, ndl);
    preempt_enable();
    }
    migrate_enable();
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_bh_section(nloops: c_int) {
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    local_bh_disable();
    local_bh_enable();
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_bh_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    local_bh_disable();
    un_delay(udl, ndl);
    local_bh_enable();
    }
    preempt_enable();
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_irq_section(nloops: c_int) {
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    local_irq_disable();
    local_irq_enable();
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_irq_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    local_irq_disable();
    un_delay(udl, ndl);
    local_irq_enable();
    }
    preempt_enable();
    }
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn ref_irqsave_section(nloops: c_int) {
    let mut flags = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    local_irq_save(flags);
    local_irq_restore(flags);
    }
    preempt_enable();
    }
#[no_mangle]
unsafe extern "C" fn ref_irqsave_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut flags = 0;
    let mut i = 0;
    preempt_disable();
    while (i >= 0) {
    local_irq_save(flags);
    un_delay(udl, ndl);
    local_irq_restore(flags);
    }
    preempt_enable();
    }
pub static mut ref_scale_ops: usize = 0;
//
// Methods leveraging SLAB_TYPESAFE_BY_RCU.
//
// Item to look up in a typesafe manner.  Array of pointers to these.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct refscale_typesafe {
    pub flavors: atomic_t rts_refctr; // Used by all,
    pub rts_lock: spinlock_t,
    pub rts_seqlock: seqlock_t,
    pub a: c_uint,
    pub b: c_uint,
}

pub static mut typesafe_kmem_cachep: *mut c_void = core::ptr::null_mut();
pub static mut rtsarray: *mut c_void = core::ptr::null_mut();
    static long rtsarray_size;
pub static mut refscale_rand: usize = 0;
    static bool (*rts_acquire)(refscale_typesafe *rtsp, unsigned int *start);
    static bool (*rts_release)(refscale_typesafe *rtsp, unsigned int start);
// Conditionally acquire an explicit in-structure reference count.
#[no_mangle]
unsafe extern "C" fn typesafe_ref_acquire(rtsp: *mut refscale_typesafe, start: *mut c_uint) -> bool {
    return atomic_inc_not_zero(&rtsp.rts_refctr);
    }
// Unconditionally release an explicit in-structure reference count.
#[no_mangle]
unsafe extern "C" fn typesafe_ref_release(rtsp: *mut refscale_typesafe, start: c_uint) -> bool {
    if (!atomic_dec_return(&rtsp.rts_refctr)) {
    WRITE_ONCE(rtsp.a, rtsp.a + 1);
    kmem_cache_free(typesafe_kmem_cachep, rtsp);
    }
    return true;
    }
// Unconditionally acquire an explicit in-structure spinlock.
#[no_mangle]
unsafe extern "C" fn typesafe_lock_acquire(rtsp: *mut refscale_typesafe, start: *mut c_uint) -> bool {
    spin_lock(&rtsp.rts_lock);
    return true;
    }
// Unconditionally release an explicit in-structure spinlock.
#[no_mangle]
unsafe extern "C" fn typesafe_lock_release(rtsp: *mut refscale_typesafe, start: c_uint) -> bool {
    spin_unlock(&rtsp.rts_lock);
    return true;
    }
// Unconditionally acquire an explicit in-structure sequence lock.
#[no_mangle]
unsafe extern "C" fn typesafe_seqlock_acquire(rtsp: *mut refscale_typesafe, start: *mut c_uint) -> bool {
// start = read_seqbegin(&rtsp->rts_seqlock);
    return true;
    }
// Conditionally release an explicit in-structure sequence lock.  Return
// true if this release was successful, that is, if no retry is required.
#[no_mangle]
unsafe extern "C" fn typesafe_seqlock_release(rtsp: *mut refscale_typesafe, start: c_uint) -> bool {
    return !read_seqretry(&rtsp.rts_seqlock, start);
    }
// Do a read-side critical section with the specified delay in
// microseconds and nanoseconds inserted so as to increase probability
// of failure.
#[no_mangle]
unsafe extern "C" fn typesafe_delay_section(nloops: c_int, udl: c_int, ndl: c_int) {
    let mut a = 0;
    let mut b = 0;
    let mut i = 0;
    let mut idx = 0;
pub static mut rtsp: *mut c_void = core::ptr::null_mut();
    let mut start = 0;
    while (i >= 0) {
    preempt_disable();
    idx = torture_random(this_cpu_ptr(&refscale_rand)) % rtsarray_size;
    preempt_enable();
// label;
    rcu_read_lock();
    rtsp = rcu_dereference(rtsarray[idx]);
    a = READ_ONCE(rtsp.a);
    if (!rts_acquire(rtsp, &start)) {
    rcu_read_unlock();
// goto;
    }
    if (a != READ_ONCE(rtsp.a)) {
    (void)rts_release(rtsp, start);
    rcu_read_unlock();
// goto;
    }
    un_delay(udl, ndl);
    b = READ_ONCE(rtsp.a);
// Remember, seqlock read-side release can fail.
    if (!rts_release(rtsp, start)) {
    rcu_read_unlock();
// goto;
    }
    WARN_ONCE(a != b, "Re-read of .a changed from %u to %u.\n", a, b);
    b = rtsp.b;
    rcu_read_unlock();
    WARN_ON_ONCE!(a * a != b);
    }
    }
// Because the acquisition and release methods are expensive, there
// is no point in optimizing away the un_delay() function's two checks.
// Thus simply define typesafe_read_section() as a simple wrapper around
// typesafe_delay_section().
#[no_mangle]
unsafe extern "C" fn typesafe_read_section(nloops: c_int) {
    typesafe_delay_section(nloops, 0, 0);
    }
// Allocate and initialize one refscale_typesafe structure.
#[no_mangle]
pub unsafe extern "C" fn typesafe_alloc_one() -> *mut c_void {
pub static mut rtsp: *mut c_void = core::ptr::null_mut();
    rtsp = kmem_cache_alloc(typesafe_kmem_cachep, GFP_KERNEL);
    if (!rtsp) {
    return core::ptr::null_mut();
    }
    atomic_set(&rtsp.rts_refctr, 1);
    WRITE_ONCE(rtsp.a, rtsp.a + 1);
    WRITE_ONCE(rtsp.b, rtsp.a * rtsp.a);
    return rtsp;
    }
// Slab-allocator constructor for refscale_typesafe structures created
// out of a new slab of system memory.
#[no_mangle]
unsafe extern "C" fn refscale_typesafe_ctor(rtsp_in: *mut c_void) {
    let mut rtsp = rtsp_in;
    spin_lock_init(&rtsp.rts_lock);
    seqlock_init(&rtsp.rts_seqlock);
    preempt_disable();
    rtsp.a = torture_random(this_cpu_ptr(&refscale_rand));
    preempt_enable();
    }
pub static mut typesafe_ref_ops: usize = 0;
pub static mut typesafe_lock_ops: usize = 0;
pub static mut typesafe_seqlock_ops: usize = 0;
// Initialize for a typesafe test.
#[no_mangle]
unsafe extern "C" fn typesafe_init() -> bool {
    let mut idx = 0;
pub static mut si: c_long = 0;
    typesafe_kmem_cachep = kmem_cache_create("refscale_typesafe",
    sizeof!(refscale_typesafe), sizeof!,
    SLAB_TYPESAFE_BY_RCU, refscale_typesafe_ctor);
    if (!typesafe_kmem_cachep) {
    return false;
    }
    if (si < 0) {
    si = -si * nr_cpu_ids;
    }

    else if (si == 0) {
    si = nr_cpu_ids;
    }
    rtsarray_size = si;
    rtsarray = kzalloc_objs(*rtsarray, si);
    if (!rtsarray) {
    return false;
    }
    while (idx < rtsarray_size) {
    rtsarray[idx] = typesafe_alloc_one();
    if (!rtsarray[idx]) {
    return false;
    }
    }
    if (cur_ops == &typesafe_ref_ops) {
    rts_acquire = typesafe_ref_acquire;
    rts_release = typesafe_ref_release;
    } else if (cur_ops == &typesafe_lock_ops) {
    rts_acquire = typesafe_lock_acquire;
    rts_release = typesafe_lock_release;
    } else if (cur_ops == &typesafe_seqlock_ops) {
    rts_acquire = typesafe_seqlock_acquire;
    rts_release = typesafe_seqlock_release;
    } else {
    WARN_ON_ONCE!(1);
    return false;
    }
    return true;
    }
// Clean up after a typesafe test.
#[no_mangle]
unsafe extern "C" fn typesafe_cleanup() {
    let mut idx = 0;
    if (rtsarray) {
    for (idx = 0; idx < rtsarray_size; idx++) {
    kmem_cache_free(typesafe_kmem_cachep, rtsarray[idx]);
    }
    kfree(rtsarray);
    rtsarray = core::ptr::null_mut();
    rtsarray_size = 0;
    }
    kmem_cache_destroy(typesafe_kmem_cachep);
    typesafe_kmem_cachep = core::ptr::null_mut();
    rts_acquire = core::ptr::null_mut();
    rts_release = core::ptr::null_mut();
    }
// The typesafe_init() function distinguishes these structures by address.
pub static mut ref_scale_ops: usize = 0;
pub static mut ref_scale_ops: usize = 0;
pub static mut ref_scale_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn rcu_scale_one_reader() {
    if (readdelay <= 0) {
    cur_ops.readsection(loops);
    }
    else {
    cur_ops.delaysection(loops, readdelay / 1000, readdelay % 1000);
    }
    }
// Warm up cache, or, if needed run a series of rcu_scale_one_reader()
// to allow multiple rcuscale guest OSes to collect mutually valid data.
#[no_mangle]
unsafe extern "C" fn rcu_scale_warm_cool() {
pub static mut jdone: c_ulong = 0;
    do {
    rcu_scale_one_reader();
    cond_resched();
    } while (time_before(jiffies, jdone));
    }
// Reader kthread.  Repeatedly does empty RCU read-side
// critical section, minimizing update-side interference.
#[no_mangle]
pub unsafe extern "C" fn ref_scale_reader(arg: *mut c_void) -> c_int {
    let mut flags = 0;
pub static mut me: c_long = 0;
    let mut rt = &(reader_tasks[me]);
    let mut start = 0;
    let mut duration = 0;
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: task started", me);
    WARN_ON_ONCE!(set_cpus_allowed_ptr(current, cpumask_of(me % nr_cpu_ids)));
    set_user_nice(current, MAX_NICE);
    atomic_inc(&n_init);
    if (holdoff) {
    schedule_timeout_interruptible(holdoff * HZ);
    }
// label;
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: waiting to start next experiment on cpu %d", me, raw_smp_processor_id());
// Wait for signal that this reader can start.
    wait_event(rt.wq, (atomic_read(&nreaders_exp) && smp_load_acquire(&rt.start_reader)) ||
    torture_must_stop());
    if (torture_must_stop()) {
// goto;
    }
// Make sure that the CPU is affinitized appropriately during testing.
    WARN_ON_ONCE!(raw_smp_processor_id() != me % nr_cpu_ids);
    WRITE_ONCE(rt.start_reader, 0);
    if (!atomic_dec_return(&n_started)) {
    while (atomic_read_acquire(&n_started))
    cpu_relax();
    }
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: experiment %d started", me, exp_idx);
// To reduce noise, do an initial cache-warming invocation, check
// in, and then keep warming until everyone has checked in.
    rcu_scale_one_reader();
    if (!atomic_dec_return(&n_warmedup)) {
    while (atomic_read_acquire(&n_warmedup))
    rcu_scale_one_reader();
    }
// Also keep interrupts disabled when it is safe to do so, which
// it is not for local_bh_enable().  This also has the effect of
// preventing entries into slow path for rcu_read_unlock().
    if (!cur_ops.enable_irqs) {
    local_irq_save(flags);
    }
    start = ktime_get_mono_fast_ns();
    rcu_scale_one_reader();
    duration = ktime_get_mono_fast_ns() - start;
    if (!cur_ops.enable_irqs) {
    local_irq_restore(flags);
    }
    rt.last_duration_ns = WARN_ON_ONCE!(duration < 0) ? 0 : duration;
// To reduce runtime-skew noise, do maintain-load invocations until
// everyone is done.
    if (!atomic_dec_return(&n_cooleddown)) {
    while (atomic_read_acquire(&n_cooleddown))
    rcu_scale_one_reader();
    }
    if (atomic_dec_and_test(&nreaders_exp)) {
    wake_up(&main_wq);
    }
    VERBOSE_SCALEOUT_BATCH("ref_scale_reader %ld: experiment %d ended, (readers remaining=%d)",
    me, exp_idx, atomic_read(&nreaders_exp));
    if (!torture_must_stop()) {
// goto;
    }
// label;
    torture_kthread_stopping("ref_scale_reader");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn reset_readers() {
    let mut i = 0;
pub static mut rt: *mut c_void = core::ptr::null_mut();
    while (i < nreaders) {
    rt = &(reader_tasks[i]);
    rt.last_duration_ns = 0;
    }
    }
// Print the results of each reader and return the sum of all their durations.
#[no_mangle]
unsafe extern "C" fn process_durations(n: c_int) -> u64 {
    let mut i = 0;
pub static mut rt: *mut c_void = core::ptr::null_mut();
pub static mut s: usize = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut sum: u64 = 0;
    buf = kmalloc(800 + 64, GFP_KERNEL);
    if (!buf) {
    return 0;
    }
    seq_buf_init(&s, buf, 800 + 64);
    seq_buf_printf(&s, "Experiment #%d (Format: <THREAD-NUM>:<Total loop time in ns>)",
    exp_idx);
    while (i < n && !torture_must_stop()) {
    rt = &(reader_tasks[i]);
    if (i % 5 == 0) {
    seq_buf_putc(&s, '\n');
    }
    if (seq_buf_used(&s) >= 800) {
    pr_alert("%s", seq_buf_str(&s));
    seq_buf_clear(&s);
    }
    seq_buf_printf(&s, "%d: %llu\t", i, rt.last_duration_ns);
    sum += rt.last_duration_ns;
    }
    pr_alert("%s\n", seq_buf_str(&s));
    kfree(buf);
    return sum;
    }
// forward_decl: ref_scale_cleanup;
// The main_func is the main orchestrator, it performs a bunch of
// experiments.  For every experiment, it orders all the readers
// involved to start and waits for them to finish the experiment. It
// then reads their timestamps and starts the next experiment. Each
// experiment progresses from 1 concurrent reader to N of them at which
// point all the timestamps are printed.
#[no_mangle]
unsafe extern "C" fn main_func(arg: *mut c_void) -> c_int {
    let mut exp = 0;
    let mut r = 0;
    char buf1[64];
pub static mut buf: *mut c_void = core::ptr::null_mut();
pub static mut result_avg: *mut c_void = core::ptr::null_mut();
    set_cpus_allowed_ptr(current, cpumask_of(nreaders % nr_cpu_ids));
    set_user_nice(current, MAX_NICE);
    VERBOSE_SCALEOUT("main_func task started");
    result_avg = kcalloc(nruns, sizeof!(*result_avg), GFP_KERNEL);
    buf = kzalloc(800 + 64, GFP_KERNEL);
    if (!result_avg || !buf) {
    SCALEOUT_ERRSTRING("out of memory");
// goto;
    }
    if (holdoff) {
    schedule_timeout_interruptible(holdoff * HZ);
    }
// Wait for all threads to start.
    atomic_inc(&n_init);
    while (atomic_read(&n_init) < nreaders + 1) {
    schedule_timeout_uninterruptible(1);
    }
// Start exp readers up per experiment
    rcu_scale_warm_cool();
    while (exp < nruns && !torture_must_stop()) {
    if (torture_must_stop()) {
// goto;
    }
    reset_readers();
    atomic_set(&nreaders_exp, nreaders);
    atomic_set(&n_started, nreaders);
    atomic_set(&n_warmedup, nreaders);
    atomic_set(&n_cooleddown, nreaders);
    exp_idx = exp;
    while (r < nreaders) {
    smp_store_release(&reader_tasks[r].start_reader, 1);
    wake_up(&reader_tasks[r].wq);
    }
    VERBOSE_SCALEOUT("main_func: experiment started, waiting for %d readers",
    nreaders);
    wait_event(main_wq,
    !atomic_read(&nreaders_exp) || torture_must_stop());
    VERBOSE_SCALEOUT("main_func: experiment ended");
    if (torture_must_stop()) {
// goto;
    }
    result_avg[exp] = div_u64(1000 * process_durations(nreaders), nreaders * loops);
    }
    rcu_scale_warm_cool();
// Print the average of all experiments
    SCALEOUT("END OF TEST. Calculating average duration per loop (nanoseconds)...\n");
    pr_alert("Runs\tTime(ns)\n");
    while (exp < nruns) {
    let mut avg = 0;
    let mut rem = 0;
    avg = div_u64_rem(result_avg[exp], 1000, &rem);
    sprintf(buf1, "%d\t%llu.%03u\n", exp + 1, avg, rem);
    strcat(buf, buf1);
    if (strlen(buf) >= 800) {
    pr_alert("%s", buf);
    buf[0] = 0;
    }
    }
    pr_alert("%s", buf);
// label;
// This will shutdown everything including us.
    if (shutdown_secs) {
    main_task = core::ptr::null_mut();  // Avoid self-kill deadlock.
    ref_scale_cleanup();
    kernel_power_off();
    }
// Wait for torture to stop us
    while (!torture_must_stop()) {
    schedule_timeout_uninterruptible(1);
    }
// label;
    torture_kthread_stopping("main_func");
    kfree(result_avg);
    kfree(buf);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ref_scale_print_module_parms(cur_ops: *mut ref_scale_ops, tag: *mut c_char) {
    pr_alert("%s" SCALE_FLAG
    "--- %s:  verbose=%d verbose_batched=%d shutdown_secs=%d holdoff=%d lookup_instances=%ld loops=%d nreaders=%d nruns=%d readdelay=%d\n", scale_type, tag,
    verbose, verbose_batched, shutdown_secs, holdoff, lookup_instances, loops, nreaders, nruns, readdelay);
    }
#[no_mangle]
pub unsafe extern "C" fn ref_scale_cleanup() {
    let mut i = 0;
    if (torture_cleanup_begin()) {
    return;
    }
    if (!cur_ops) {
    torture_cleanup_end();
    return;
    }
    if (reader_tasks) {
    for (i = 0; i < nreaders; i++) {
    torture_stop_kthread("ref_scale_reader",
    reader_tasks[i].task);
    }
    }
    kfree(reader_tasks);
    reader_tasks = core::ptr::null_mut();
    torture_stop_kthread("main_task", main_task);
// Do scale-type-specific cleanup operations.
    if (cur_ops.cleanup != core::ptr::null_mut()) {
    cur_ops.cleanup();
    }
    torture_cleanup_end();
    }
    static int __init
    ref_scale_init(void)
    {
    let mut i = 0;
pub static mut firsterr: c_int = 0;
    static const struct ref_scale_ops *scale_ops[] = {
    &rcu_ops, &srcu_ops, &srcu_fast_ops, &srcu_fast_updown_ops,
    RCU_TRACE_OPS RCU_TASKS_OPS
    &refcnt_ops, &percpuinc_ops, &incpercpu_ops, &incpercpupreempt_ops,
    &incpercpubh_ops, &incpercpuirqsave_ops,
    &rwlock_ops, &rwsem_ops, &lock_ops, &lock_irq_ops, &acqrel_ops,
    &sched_clock_ops, &clock_ops, &jiffies_ops,
    &preempt_ops, &bh_ops, &irq_ops, &irqsave_ops,
    &typesafe_ref_ops, &typesafe_lock_ops, &typesafe_seqlock_ops,
    };
    if (!torture_init_begin(scale_type, verbose)) {
    return -EBUSY;
    }
    while (i < ARRAY_SIZE!(scale_ops)) {
    cur_ops = scale_ops[i];
    if (strcmp(scale_type, cur_ops.name) == 0) {
    break;
    }
    }
    if (i == ARRAY_SIZE!(scale_ops)) {
    pr_alert("rcu-scale: invalid scale type: \"%s\"\n", scale_type);
    pr_alert("rcu-scale types:");
    for (i = 0; i < ARRAY_SIZE!(scale_ops); i++) {
    pr_cont(" %s", scale_ops[i].name);
    }
    pr_cont("\n");
    firsterr = -EINVAL;
    cur_ops = core::ptr::null_mut();
// goto;
    }
    if (cur_ops.init) {
    if (!cur_ops.init()) {
    }
    firsterr = -EUCLEAN;
// goto;
    }
    ref_scale_print_module_parms(cur_ops, "Start of test");
// Shutdown task
    if (shutdown_secs) {
    firsterr = torture_shutdown_init(shutdown_secs, ref_scale_cleanup);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
// Reader tasks (default to ~75% of online CPUs).
    if (nreaders < 0) {
    nreaders = (num_online_cpus() >> 1) + (num_online_cpus() >> 2);
    }
    if (WARN_ONCE(loops <= 0, "%s: loops = %d, adjusted to 1\n", __func__, loops)) {
    loops = 1;
    }
    if (WARN_ONCE(nreaders <= 0, "%s: nreaders = %d, adjusted to 1\n", __func__, nreaders)) {
    nreaders = 1;
    }
    if (WARN_ONCE(nruns <= 0, "%s: nruns = %d, adjusted to 1\n", __func__, nruns)) {
    nruns = 1;
    }
    if (WARN_ONCE(loops > INT_MAX / nreaders,
    "%s: nreaders * loops will overflow, adjusted loops to %d",
    __func__, INT_MAX / nreaders)) {
    loops = INT_MAX / nreaders;
    }
    reader_tasks = kzalloc_objs(reader_tasks[0], nreaders);
    if (!reader_tasks) {
    SCALEOUT_ERRSTRING("out of memory");
    firsterr = -ENOMEM;
// goto;
    }
    VERBOSE_SCALEOUT("Starting %d reader threads", nreaders);
    while (i < nreaders) {
    init_waitqueue_head(&reader_tasks[i].wq);
    firsterr = torture_create_kthread(ref_scale_reader, i,
    reader_tasks[i].task);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
// Main Task
    init_waitqueue_head(&main_wq);
    firsterr = torture_create_kthread(main_func, core::ptr::null_mut(), main_task);
    if (torture_init_error(firsterr)) {
// goto;
    }
    torture_init_end();
    return 0;
// label;
    torture_init_end();
    ref_scale_cleanup();
    if (shutdown_secs) {
    WARN_ON!(!IS_MODULE(CONFIG_RCU_REF_SCALE_TEST));
    kernel_power_off();
    }
    return firsterr;
    }
    module_init!(ref_scale_init);
    module_exit!(ref_scale_cleanup);