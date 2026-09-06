//! Automatically rewritten from C to Rust
//! Source: kernel/locking/locktorture.c
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
// Module-based torture test facility for locking
//
// Copyright (C) IBM Corporation, 2014
//
// Authors: Paul E. McKenney <paulmck@linux.ibm.com>
// Davidlohr Bueso <dave@stgolabs.net>
// Based on kernel/rcu/torture.c.
//

    MODULE_DESCRIPTION("torture test facility for locking");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Paul E. McKenney <paulmck@linux.ibm.com>");
    torture_param(int, acq_writer_lim, 0, "Write_acquisition time limit (jiffies).");
    torture_param(int, call_rcu_chains, 0, "Self-propagate call_rcu() chains during test (0=disable).");
    torture_param(int, long_hold, 100, "Do occasional long hold of lock (ms), 0=disable");
    torture_param(int, nested_locks, 0, "Number of nested locks (max = 8)");
    torture_param(int, nreaders_stress, -1, "Number of read-locking stress-test threads");
    torture_param(int, nwriters_stress, -1, "Number of write-locking stress-test threads");
    torture_param(int, onoff_holdoff, 0, "Time after boot before CPU hotplugs (s)");
    torture_param(int, onoff_interval, 0, "Time between CPU hotplugs (s), 0=disable");
    torture_param(int, rt_boost, 2,
    "Do periodic rt-boost. 0=Disable, 1=Only for rt_mutex, 2=For all lock types.");
    torture_param(int, rt_boost_factor, 50, "A factor determining how often rt-boost happens.");
    torture_param(int, shuffle_interval, 3, "Number of jiffies between shuffles, 0=disable");
    torture_param(int, shutdown_secs, 0, "Shutdown time (j), <= zero to disable.");
    torture_param(int, stat_interval, 60, "Number of seconds between stats printk()s");
    torture_param(int, stutter, 5, "Number of jiffies to run/halt test, 0=disable");
    torture_param(int, verbose, 1, "Enable verbose debugging printk()s");
    torture_param(int, writer_fifo, 0, "Run writers at sched_set_fifo() priority");
// Going much higher trips "BUG: MAX_LOCKDEP_CHAIN_HLOCKS too low!" errors
pub const MAX_NESTED_LOCKS: c_int = 8;
    static char *torture_type = IS_ENABLED!(CONFIG_PREEMPT_RT) ? "raw_spin_lock" : "spin_lock";
    module_param!(torture_type, charp, 0444);
    MODULE_PARM_DESC(torture_type,
    "Type of lock to torture (spin_lock, spin_lock_irq, mutex_lock, ...)");
    static cpumask_var_t bind_readers; // Bind the readers to the specified set of CPUs.
    static cpumask_var_t bind_writers; // Bind the writers to the specified set of CPUs.
// Parse a cpumask kernel parameter.  If there are more users later on,
// this might need to got to a more central location.
#[no_mangle]
unsafe extern "C" fn param_set_cpumask(val: *const c_char, kp: *const kernel_param) -> c_int {
    let mut cm_bind = kp.arg;
    let mut ret = 0;
pub static mut s: *mut c_void = core::ptr::null_mut();
    if (!alloc_cpumask_var(cm_bind, GFP_KERNEL)) {
    s = "Out of memory";
    ret = -ENOMEM;
// goto;
    }
    ret = cpulist_parse(val, *cm_bind);
    if (!ret) {
    return ret;
    }
    s = "Bad CPU range";
// label;
    pr_warn!("%s: %s, all CPUs set\n", kp.name, s);
    cpumask_setall(*cm_bind);
    return ret;
    }
// Output a cpumask kernel parameter.
#[no_mangle]
unsafe extern "C" fn param_get_cpumask(buffer: *mut c_char, kp: *const kernel_param) -> c_int {
    let mut cm_bind = kp.arg;
    return sprintf(buffer, "%*pbl", cpumask_pr_args(*cm_bind));
    }
#[no_mangle]
unsafe extern "C" fn cpumask_nonempty(mask: cpumask_var_t) -> bool {
    return cpumask_available(mask) && !cpumask_empty(mask);
    }
pub static mut kernel_param_ops: usize = 0;
    module_param_cb!(bind_readers, &lt_bind_ops, &bind_readers, 0444);
    module_param_cb!(bind_writers, &lt_bind_ops, &bind_writers, 0444);
// forward_decl: torture_sched_setaffinity;
pub static mut stats_task: *mut c_void = core::ptr::null_mut();
pub static mut writer_tasks: *mut c_void = core::ptr::null_mut();
pub static mut reader_tasks: *mut c_void = core::ptr::null_mut();
    static bool lock_is_write_held;
    static atomic_t lock_is_read_held;
    static unsigned long last_lock_release;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_stress_stats {
    pub n_lock_fail: c_long,
    pub n_lock_acquired: c_long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct call_rcu_chain {
    pub crc_rh: rcu_head,
    pub crc_stop: bool,
}

pub static mut call_rcu_chain_list: *mut c_void = core::ptr::null_mut();
// Forward reference.
// forward_decl: lock_torture_cleanup;
//
// Operations vector for selecting different types of tests.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_torture_ops {
    pub (*init)(void): *mut c_void,
    pub (*exit)(void): *mut c_void,
    pub lockset): *mut *mut int (nested_lock)(int tid, u32,
    pub tid): *mut *mut int (writelock)(int,
    pub trsp): *mut *mut c_void (write_delay)(torture_random_state,
    pub trsp): *mut *mut c_void (task_boost)(torture_random_state,
    pub tid): *mut *mut c_void (writeunlock)(int,
    pub lockset): *mut *mut c_void (nested_unlock)(int tid, u32,
    pub tid): *mut *mut int (readlock)(int,
    pub trsp): *mut *mut c_void (read_delay)(torture_random_state,
    pub tid): *mut *mut c_void (readunlock)(int,
//     pub /: *mut *mut unsigned long flags; / for irq spinlocks,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lock_torture_cxt {
    pub nrealwriters_stress: c_int,
    pub nrealreaders_stress: c_int,
    pub debug_lock: bool,
    pub init_called: bool,
    pub n_lock_torture_errors: core::sync::atomic::AtomicI32,
    pub cur_ops: *mut lock_torture_ops,
//     pub /: *mut *mut *mut lock_stress_stats lwsa; / writer statistics,
//     pub /: *mut *mut *mut lock_stress_stats lrsa; / reader statistics,
}

pub static mut lock_torture_cxt: usize = 0;
//
// Definitions for lock torture testing.
//
#[no_mangle]
unsafe extern "C" fn torture_lock_busted_write_lock(__maybe_unused: int tid) -> c_int {
    return 0;  /* BUGGY, do not use in real life!!! */
    }
#[no_mangle]
unsafe extern "C" fn torture_lock_busted_write_delay(trsp: *mut torture_random_state) {
// We want a long delay occasionally to force massive contention.
    if (long_hold && !(torture_random(trsp) % (cxt.nrealwriters_stress * 2000 * long_hold))) {
    mdelay(long_hold);
    }
    if (!(torture_random(trsp) % (cxt.nrealwriters_stress * 20000))) {
    torture_preempt_schedule();  /* Allow test to be preempted. */
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_lock_busted_write_unlock(__maybe_unused: int tid) {
// BUGGY, do not use in real life!!!
    }
#[no_mangle]
unsafe extern "C" fn __torture_rt_boost(trsp: *mut torture_random_state) {
pub static mut factor: c_uint = 0;
    if (!rt_task(current)) {
//
// Boost priority once every rt_boost_factor operations. When
// the task tries to take the lock, the rtmutex it will account
// for the new priority, and do any corresponding pi-dance.
//
    if (trsp && !(torture_random(trsp) %
    (cxt.nrealwriters_stress * factor))) {
    sched_set_fifo(current);
    } else /* common case, do nothing */
    return;
    } else {
//
// The task will remain boosted for another 10 * rt_boost_factor
// operations, then restored back to its original prio, and so
// forth.
//
// When @trsp is nil, we want to force-reset the task for
// stopping the kthread.
//
    if (!trsp || !(torture_random(trsp) %
    (cxt.nrealwriters_stress * factor * 2))) {
    sched_set_normal(current, 0);
    } else /* common case, do nothing */
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_rt_boost(trsp: *mut torture_random_state) {
    if (rt_boost != 2) {
    return;
    }
    __torture_rt_boost(trsp);
    }
pub static mut lock_torture_ops: usize = 0;
pub static mut torture_spinlock: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_spin_lock_write_lock(__maybe_unused: int tid) -> c_int {
    spin_lock(&torture_spinlock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_spin_lock_write_delay(trsp: *mut torture_random_state) {
pub static mut shortdelay_us: c_ulong = 2;
    let mut j = 0;
// We want a short delay mostly to emulate likely code, and
// we want a long delay occasionally to force massive contention.
//
    if (long_hold && !(torture_random(trsp) % (cxt.nrealwriters_stress * 2000 * long_hold))) {
    j = jiffies;
    mdelay(long_hold);
    pr_alert("%s: delay = %lu jiffies.\n", __func__, jiffies - j);
    }
    if (!(torture_random(trsp) % (cxt.nrealwriters_stress * 200 * shortdelay_us))) {
    udelay(shortdelay_us);
    }
    if (!(torture_random(trsp) % (cxt.nrealwriters_stress * 20000))) {
    torture_preempt_schedule();  /* Allow test to be preempted. */
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_spin_lock_write_unlock(__maybe_unused: int tid) {
    spin_unlock(&torture_spinlock);
    }
pub static mut lock_torture_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_spin_lock_write_lock_irq(__maybe_unused: int tid) -> c_int {
    let mut flags = 0;
    spin_lock_irqsave(&torture_spinlock, flags);
    cxt.cur_ops.flags = flags;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_lock_spin_write_unlock_irq(__maybe_unused: int tid) {
    spin_unlock_irqrestore(&torture_spinlock, cxt.cur_ops.flags);
    }
pub static mut lock_torture_ops: usize = 0;
pub static mut torture_raw_spinlock: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_raw_spin_lock_write_lock(__maybe_unused: int tid) -> c_int {
    raw_spin_lock(&torture_raw_spinlock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_raw_spin_lock_write_unlock(__maybe_unused: int tid) {
    raw_spin_unlock(&torture_raw_spinlock);
    }
pub static mut lock_torture_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_raw_spin_lock_write_lock_irq(__maybe_unused: int tid) -> c_int {
    let mut flags = 0;
    raw_spin_lock_irqsave(&torture_raw_spinlock, flags);
    cxt.cur_ops.flags = flags;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_raw_spin_lock_write_unlock_irq(__maybe_unused: int tid) {
    raw_spin_unlock_irqrestore(&torture_raw_spinlock, cxt.cur_ops.flags);
    }
pub static mut lock_torture_ops: usize = 0;

    static rqspinlock_t rqspinlock;
#[no_mangle]
unsafe extern "C" fn torture_raw_res_spin_write_lock(__maybe_unused: int tid) -> c_int {
    raw_res_spin_lock(&rqspinlock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_raw_res_spin_write_unlock(__maybe_unused: int tid) {
    raw_res_spin_unlock(&rqspinlock);
    }
pub static mut lock_torture_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_raw_res_spin_write_lock_irq(__maybe_unused: int tid) -> c_int {
    let mut flags = 0;
    raw_res_spin_lock_irqsave(&rqspinlock, flags);
    cxt.cur_ops.flags = flags;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_raw_res_spin_write_unlock_irq(__maybe_unused: int tid) {
    raw_res_spin_unlock_irqrestore(&rqspinlock, cxt.cur_ops.flags);
    }
pub static mut lock_torture_ops: usize = 0;

pub static mut torture_rwlock: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_rwlock_write_lock(__maybe_unused: int tid) -> c_int {
    write_lock(&torture_rwlock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_write_delay(trsp: *mut torture_random_state) {
pub static mut shortdelay_us: c_ulong = 2;
// We want a short delay mostly to emulate likely code, and
// we want a long delay occasionally to force massive contention.
//
    if (long_hold && !(torture_random(trsp) % (cxt.nrealwriters_stress * 2000 * long_hold))) {
    mdelay(long_hold);
    }
    else {
    udelay(shortdelay_us);
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_write_unlock(__maybe_unused: int tid) {
    write_unlock(&torture_rwlock);
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_read_lock(__maybe_unused: int tid) -> c_int {
    read_lock(&torture_rwlock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_read_delay(trsp: *mut torture_random_state) {
pub static mut shortdelay_us: c_ulong = 10;
// We want a short delay mostly to emulate likely code, and
// we want a long delay occasionally to force massive contention.
//
    if (long_hold && !(torture_random(trsp) % (cxt.nrealreaders_stress * 2000 * long_hold))) {
    mdelay(long_hold);
    }
    else {
    udelay(shortdelay_us);
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_read_unlock(__maybe_unused: int tid) {
    read_unlock(&torture_rwlock);
    }
pub static mut lock_torture_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_rwlock_write_lock_irq(__maybe_unused: int tid) -> c_int {
    let mut flags = 0;
    write_lock_irqsave(&torture_rwlock, flags);
    cxt.cur_ops.flags = flags;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_write_unlock_irq(__maybe_unused: int tid) {
    write_unlock_irqrestore(&torture_rwlock, cxt.cur_ops.flags);
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_read_lock_irq(__maybe_unused: int tid) -> c_int {
    let mut flags = 0;
    read_lock_irqsave(&torture_rwlock, flags);
    cxt.cur_ops.flags = flags;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_rwlock_read_unlock_irq(__maybe_unused: int tid) {
    read_unlock_irqrestore(&torture_rwlock, cxt.cur_ops.flags);
    }
pub static mut lock_torture_ops: usize = 0;
pub static mut torture_mutex: usize = 0;
    static struct mutex torture_nested_mutexes[MAX_NESTED_LOCKS];
    static struct lock_class_key nested_mutex_keys[MAX_NESTED_LOCKS];
#[no_mangle]
unsafe extern "C" fn torture_mutex_init() {
    let mut i = 0;
    for (i = 0; i < MAX_NESTED_LOCKS; i++) {
    __mutex_init(&torture_nested_mutexes[i], __func__,
    &nested_mutex_keys[i]);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn torture_mutex_nested_lock(__maybe_unused: int tid, lockset: u32) -> c_int {
    let mut i = 0;
    for (i = 0; i < nested_locks; i++) {
    if (lockset & (1 << i))
    mutex_lock(&torture_nested_mutexes[i]);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_mutex_lock(__maybe_unused: int tid) -> c_int {
    mutex_lock(&torture_mutex);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_mutex_delay(trsp: *mut torture_random_state) {
// We want a long delay occasionally to force massive contention.
    if (long_hold && !(torture_random(trsp) % (cxt.nrealwriters_stress * 2000 * long_hold))) {
    mdelay(long_hold * 5);
    }
    if (!(torture_random(trsp) % (cxt.nrealwriters_stress * 20000))) {
    torture_preempt_schedule();  /* Allow test to be preempted. */
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_mutex_unlock(__maybe_unused: int tid) {
    mutex_unlock(&torture_mutex);
    }
#[no_mangle]
pub unsafe extern "C" fn torture_mutex_nested_unlock(__maybe_unused: int tid, lockset: u32) {
    let mut i = 0;
    for (i = nested_locks - 1; i >= 0; i--) {
    if (lockset & (1 << i))
    mutex_unlock(&torture_nested_mutexes[i]);
    }
    }
pub static mut lock_torture_ops: usize = 0;

//
// The torture ww_mutexes should belong to the same lock class as
// torture_ww_class to avoid lockdep problem. The ww_mutex_init()
// function is called for initialization to ensure that.
//
pub static mut torture_ww_class: usize = 0;
    static struct ww_mutex torture_ww_mutex_0, torture_ww_mutex_1, torture_ww_mutex_2;
pub static mut ww_acquire_ctxs: *mut c_void = core::ptr::null_mut();
#[no_mangle]
unsafe extern "C" fn torture_ww_mutex_init() {
    ww_mutex_init(&torture_ww_mutex_0, &torture_ww_class);
    ww_mutex_init(&torture_ww_mutex_1, &torture_ww_class);
    ww_mutex_init(&torture_ww_mutex_2, &torture_ww_class);
    ww_acquire_ctxs = kmalloc_objs(*ww_acquire_ctxs,
    cxt.nrealwriters_stress);
    if (!ww_acquire_ctxs) {
    VERBOSE_TOROUT_STRING("ww_acquire_ctx: Out of memory");
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_ww_mutex_exit() {
    kfree(ww_acquire_ctxs);
    }
#[no_mangle]
unsafe extern "C" fn torture_ww_mutex_lock(tid: c_int) -> c_int {
pub static mut list: usize = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct reorder_lock {
    pub link: list_head,
    pub lock: *mut ww_mutex,
    pub ln: *mut *mut } locks[3], ll,,
    pub &ww_acquire_ctxs[tid]: *mut *mut ww_acquire_ctx ctx =,
    pub &torture_ww_mutex_0: locks[0].lock =,
    pub &list): list_add(&locks[0].link,,
    pub &torture_ww_mutex_1: locks[1].lock =,
    pub &list): list_add(&locks[1].link,,
    pub &torture_ww_mutex_2: locks[2].lock =,
    pub &list): list_add(&locks[2].link,,
    pub &torture_ww_class): ww_acquire_init(ctx,,
    list_for_each_entry(ll, &list, link) {
    pub err: c_int,
    pub ctx): err = ww_mutex_lock(ll->lock,,
    if (!err) {
    pub ll: ln =,
    list_for_each_entry_continue_reverse(ln, &list, link)
    if (err != -EDEADLK)
    pub err: return,
    pub ctx): ww_mutex_lock_slow(ll->lock,,
    pub &list): list_move(&ll->link,,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn torture_ww_mutex_unlock(tid: c_int) {
    }
    pub &ww_acquire_ctxs[tid]: *mut *mut ww_acquire_ctx ctx =,
    }
pub static mut lock_torture_ops: usize = 0;

pub static mut torture_rwsem: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_rwsem_down_write(__maybe_unused: int tid) -> c_int {
    down_write(&torture_rwsem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_rwsem_write_delay(trsp: *mut torture_random_state) {
// We want a long delay occasionally to force massive contention.
    if (long_hold && !(torture_random(trsp) % (cxt.nrealwriters_stress * 2000 * long_hold))) {
    mdelay(long_hold * 10);
    }
    if (!(torture_random(trsp) % (cxt.nrealwriters_stress * 20000))) {
    torture_preempt_schedule();  /* Allow test to be preempted. */
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_rwsem_up_write(__maybe_unused: int tid) {
    up_write(&torture_rwsem);
    }
#[no_mangle]
unsafe extern "C" fn torture_rwsem_down_read(__maybe_unused: int tid) -> c_int {
    down_read(&torture_rwsem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_rwsem_read_delay(trsp: *mut torture_random_state) {
// We want a long delay occasionally to force massive contention.
    if (long_hold && !(torture_random(trsp) % (cxt.nrealreaders_stress * 2000 * long_hold))) {
    mdelay(long_hold * 2);
    }
    else {
    mdelay(long_hold / 2);
    }
    if (!(torture_random(trsp) % (cxt.nrealreaders_stress * 20000))) {
    torture_preempt_schedule();  /* Allow test to be preempted. */
    }
    }
#[no_mangle]
unsafe extern "C" fn torture_rwsem_up_read(__maybe_unused: int tid) {
    up_read(&torture_rwsem);
    }
pub static mut lock_torture_ops: usize = 0;

pub static mut pcpu_rwsem: usize = 0;
#[no_mangle]
unsafe extern "C" fn torture_percpu_rwsem_init() {
    BUG_ON!(percpu_init_rwsem(&pcpu_rwsem));
    }
#[no_mangle]
unsafe extern "C" fn torture_percpu_rwsem_exit() {
    percpu_free_rwsem(&pcpu_rwsem);
    }
#[no_mangle]
unsafe extern "C" fn torture_percpu_rwsem_down_write(__maybe_unused: int tid) -> c_int {
    percpu_down_write(&pcpu_rwsem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_percpu_rwsem_up_write(__maybe_unused: int tid) {
    percpu_up_write(&pcpu_rwsem);
    }
#[no_mangle]
unsafe extern "C" fn torture_percpu_rwsem_down_read(__maybe_unused: int tid) -> c_int {
    percpu_down_read(&pcpu_rwsem);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn torture_percpu_rwsem_up_read(__maybe_unused: int tid) {
    percpu_up_read(&pcpu_rwsem);
    }
pub static mut lock_torture_ops: usize = 0;
//
// Lock torture writer kthread.  Repeatedly acquires and releases
// the lock, checking for duplicate acquisitions.
//
#[no_mangle]
unsafe extern "C" fn lock_torture_writer(arg: *mut c_void) -> c_int {
    let mut j = 0;
    let mut j1 = 0;
    let mut lockset_mask = 0;
    let mut lwsp = arg;
pub static mut rand: usize = 0;
    let mut skip_main_lock = 0;
pub static mut tid: c_int = 0;
    VERBOSE_TOROUT_STRING("lock_torture_writer task started");
    if (!rt_task(current)) {
    set_user_nice(current, MAX_NICE);
    }
    do {
    if ((torture_random(&rand) & 0xfffff) == 0) {
    schedule_timeout_uninterruptible(1);
    }
    lockset_mask = torture_random(&rand);
//
// When using nested_locks, we want to occasionally
// skip the main lock so we can avoid always serializing
// the lock chains on that central lock. By skipping the
// main lock occasionally, we can create different
// contention patterns (allowing for multiple disjoint
// blocked trees)
//
    skip_main_lock = (nested_locks &&
    !(torture_random(&rand) % 100));
    cxt.cur_ops.task_boost(&rand);
    if (cxt.cur_ops.nested_lock) {
    cxt.cur_ops.nested_lock(tid, lockset_mask);
    }
    if (!skip_main_lock) {
    if (acq_writer_lim > 0) {
    j = jiffies;
    }
    cxt.cur_ops.writelock(tid);
    if (WARN_ON_ONCE!(lock_is_write_held)) {
    lwsp.n_lock_fail += 1;
    }
    lock_is_write_held = true;
    if (WARN_ON_ONCE!(atomic_read(&lock_is_read_held))) {
    lwsp.n_lock_fail += 1; /* rare, but... */
    }
    if (acq_writer_lim > 0) {
    j1 = jiffies;
    WARN_ONCE(time_after(j1, j + acq_writer_lim),
    "%s: Lock acquisition took %lu jiffies.\n",
    __func__, j1 - j);
    }
    lwsp.n_lock_acquired += 1;
    cxt.cur_ops.write_delay(&rand);
    lock_is_write_held = false;
    WRITE_ONCE(last_lock_release, jiffies);
    cxt.cur_ops.writeunlock(tid);
    }
    if (cxt.cur_ops.nested_unlock) {
    cxt.cur_ops.nested_unlock(tid, lockset_mask);
    }
    stutter_wait("lock_torture_writer");
    } while (!torture_must_stop());
    cxt.cur_ops.task_boost(core::ptr::null_mut()); /* reset prio */
    torture_kthread_stopping("lock_torture_writer");
    return 0;
    }
//
// Lock torture reader kthread.  Repeatedly acquires and releases
// the reader lock.
//
#[no_mangle]
unsafe extern "C" fn lock_torture_reader(arg: *mut c_void) -> c_int {
    let mut lrsp = arg;
pub static mut tid: c_int = 0;
pub static mut rand: usize = 0;
    VERBOSE_TOROUT_STRING("lock_torture_reader task started");
    set_user_nice(current, MAX_NICE);
    do {
    if ((torture_random(&rand) & 0xfffff) == 0) {
    schedule_timeout_uninterruptible(1);
    }
    cxt.cur_ops.readlock(tid);
    atomic_inc(&lock_is_read_held);
    if (WARN_ON_ONCE!(lock_is_write_held)) {
    lrsp.n_lock_fail += 1; /* rare, but... */
    }
    lrsp.n_lock_acquired += 1;
    cxt.cur_ops.read_delay(&rand);
    atomic_dec(&lock_is_read_held);
    cxt.cur_ops.readunlock(tid);
    stutter_wait("lock_torture_reader");
    } while (!torture_must_stop());
    torture_kthread_stopping("lock_torture_reader");
    return 0;
    }
//
// Create an lock-torture-statistics message in the specified buffer.
//
#[no_mangle]
pub unsafe extern "C" fn __torture_print_stats(page: *mut c_char, statp: *mut lock_stress_stats, write: bool) {
    let mut cur = 0;
pub static mut fail: bool = false;
    let mut i = 0;
    let mut n_stress = 0;
pub static mut max: c_long = 0;
pub static mut sum: c_longlong = 0;
    n_stress = write ? cxt.nrealwriters_stress : cxt.nrealreaders_stress;
    while (i < n_stress) {
    if (data_race(statp[i].n_lock_fail)) {
    fail = true;
    }
    cur = data_race(statp[i].n_lock_acquired);
    sum += cur;
    if (max < cur) {
    max = cur;
    }
    if (min > cur) {
    min = cur;
    }
    }
    page += sprintf(page,
    "%s:  Total: %lld  Max/Min: %ld/%ld %s  Fail: %d %s\n",
    write ? "Writes" : "Reads ",
    sum, max, min,
    !onoff_interval && max / 2 > min ? "???" : "",
    fail, fail ? "!!!" : "");
    if (fail) {
    atomic_inc(&cxt.n_lock_torture_errors);
    }
    }
//
// Print torture statistics.  Caller must ensure that there is only one
// call to this function at a given time!!!  This is normally accomplished
// by relying on the module system to only have one copy of the module
// loaded, and then by giving the lock_torture_stats kthread full control
// (or the init/cleanup functions when lock_torture_stats thread is not
// running).
//
#[no_mangle]
unsafe extern "C" fn lock_torture_stats_print() {
pub static mut size: c_int = 0;
pub static mut buf: *mut c_void = core::ptr::null_mut();
    if (cxt.cur_ops.readlock) {
    size += cxt.nrealreaders_stress * 200 + 8192;
    }
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf) {
    pr_err!("lock_torture_stats_print: Out of memory, need: %d",
    size);
    return;
    }
    __torture_print_stats(buf, cxt.lwsa, true);
    pr_alert("%s", buf);
    kfree(buf);
    if (cxt.cur_ops.readlock) {
    buf = kmalloc(size, GFP_KERNEL);
    if (!buf) {
    pr_err!("lock_torture_stats_print: Out of memory, need: %d",
    size);
    return;
    }
    __torture_print_stats(buf, cxt.lrsa, false);
    pr_alert("%s", buf);
    kfree(buf);
    }
    }
//
// Periodically prints torture statistics, if periodic statistics printing
// was specified via the stat_interval module parameter.
//
// No need to worry about fullstop here, since this one doesn't reference
// volatile state or register callbacks.
//
#[no_mangle]
unsafe extern "C" fn lock_torture_stats(arg: *mut c_void) -> c_int {
    VERBOSE_TOROUT_STRING("lock_torture_stats task started");
    do {
    schedule_timeout_interruptible(stat_interval * HZ);
    lock_torture_stats_print();
    torture_shutdown_absorb("lock_torture_stats");
    } while (!torture_must_stop());
    torture_kthread_stopping("lock_torture_stats");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn lock_torture_print_module_parms(cur_ops: *mut lock_torture_ops, tag: *mut c_char) {
    static cpumask_t cpumask_all;
    let mut rcmp = cpumask_nonempty(bind_readers) ? bind_readers : &cpumask_all;
    let mut wcmp = cpumask_nonempty(bind_writers) ? bind_writers : &cpumask_all;
    cpumask_setall(&cpumask_all);
    pr_alert("%s" TORTURE_FLAG
    "--- %s%s: acq_writer_lim=%d bind_readers=%*pbl bind_writers=%*pbl call_rcu_chains=%d long_hold=%d nested_locks=%d nreaders_stress=%d nwriters_stress=%d onoff_holdoff=%d onoff_interval=%d rt_boost=%d rt_boost_factor=%d shuffle_interval=%d shutdown_secs=%d stat_interval=%d stutter=%d verbose=%d writer_fifo=%d\n",
    torture_type, tag, cxt.debug_lock ? " [debug]": "",
    acq_writer_lim, cpumask_pr_args(rcmp), cpumask_pr_args(wcmp),
    call_rcu_chains, long_hold, nested_locks, cxt.nrealreaders_stress,
    cxt.nrealwriters_stress, onoff_holdoff, onoff_interval, rt_boost,
    rt_boost_factor, shuffle_interval, shutdown_secs, stat_interval, stutter,
    verbose, writer_fifo);
    }
// If requested, maintain call_rcu() chains to keep a grace period always
// in flight.  These increase the probability of getting an RCU CPU stall
// warning and associated diagnostics when a locking primitive stalls.
#[no_mangle]
unsafe extern "C" fn call_rcu_chain_cb(rhp: *mut rcu_head) {
    let mut crcp = container_of!(rhp, call_rcu_chain, crc_rh);
    if (!smp_load_acquire(&crcp.crc_stop)) {
    (void)start_poll_synchronize_rcu(); // Start one grace period...
    call_rcu(&crcp.crc_rh, call_rcu_chain_cb); // ... and later start another.
    }
    }
// Start the requested number of call_rcu() chains.
#[no_mangle]
unsafe extern "C" fn call_rcu_chain_init() -> c_int {
    let mut i = 0;
    if (call_rcu_chains <= 0) {
    return 0;
    }
    call_rcu_chain_list = kzalloc_objs(*call_rcu_chain_list,
    call_rcu_chains);
    if (!call_rcu_chain_list) {
    return -ENOMEM;
    }
    while (i < call_rcu_chains) {
    call_rcu_chain_list[i].crc_stop = false;
    call_rcu(&call_rcu_chain_list[i].crc_rh, call_rcu_chain_cb);
    }
    return 0;
    }
// Stop all of the call_rcu() chains.
#[no_mangle]
unsafe extern "C" fn call_rcu_chain_cleanup() {
    let mut i = 0;
    if (!call_rcu_chain_list) {
    return;
    }
    for (i = 0; i < call_rcu_chains; i++) {
    smp_store_release(&call_rcu_chain_list[i].crc_stop, true);
    }
    rcu_barrier();
    kfree(call_rcu_chain_list);
    call_rcu_chain_list = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn lock_torture_cleanup() {
    let mut i = 0;
    if (torture_cleanup_begin()) {
    return;
    }
//
// Indicates early cleanup, meaning that the test has not run,
// such as when passing bogus args when loading the module.
// However cxt->cur_ops.init() may have been invoked, so beside
// perform the underlying torture-specific cleanups, cur_ops.exit()
// will be invoked if needed.
//
    if (!cxt.lwsa && !cxt.lrsa) {
// goto;
    }
    if (writer_tasks) {
    for (i = 0; i < cxt.nrealwriters_stress; i++) {
    torture_stop_kthread(lock_torture_writer, writer_tasks[i]);
    }
    kfree(writer_tasks);
    writer_tasks = core::ptr::null_mut();
    }
    if (reader_tasks) {
    for (i = 0; i < cxt.nrealreaders_stress; i++) {
    torture_stop_kthread(lock_torture_reader,
    reader_tasks[i]);
    }
    kfree(reader_tasks);
    reader_tasks = core::ptr::null_mut();
    }
    torture_stop_kthread(lock_torture_stats, stats_task);
    lock_torture_stats_print();  /* -After- the stats thread is stopped! */
    if (atomic_read(&cxt.n_lock_torture_errors)) {
    lock_torture_print_module_parms(cxt.cur_ops,
    "End of test: FAILURE");
    }

    else if (torture_onoff_failures()) {
    lock_torture_print_module_parms(cxt.cur_ops,
    "End of test: LOCK_HOTPLUG");
    }
    else {
    lock_torture_print_module_parms(cxt.cur_ops,
    "End of test: SUCCESS");
    }
    kfree(cxt.lwsa);
    cxt.lwsa = core::ptr::null_mut();
    kfree(cxt.lrsa);
    cxt.lrsa = core::ptr::null_mut();
    call_rcu_chain_cleanup();
// label;
    if (cxt.init_called) {
    if (cxt.cur_ops.exit) {
    cxt.cur_ops.exit();
    }
    cxt.init_called = false;
    }
    free_cpumask_var(bind_readers);
    free_cpumask_var(bind_writers);
    torture_cleanup_end();
    }
#[no_mangle]
unsafe extern "C" fn lock_torture_init() -> c_int {
    let mut i = 0;
    let mut j = 0;
pub static mut firsterr: c_int = 0;
    static struct lock_torture_ops *torture_ops[] = {
    &lock_busted_ops,
    &spin_lock_ops, &spin_lock_irq_ops,
    &raw_spin_lock_ops, &raw_spin_lock_irq_ops,

    &raw_res_spin_lock_ops, &raw_res_spin_lock_irq_ops,

    &rw_lock_ops, &rw_lock_irq_ops,
    &mutex_lock_ops,
    &ww_mutex_lock_ops,

    &rtmutex_lock_ops,

    &rwsem_lock_ops,
    &percpu_rwsem_lock_ops,
    };
    if (!torture_init_begin(torture_type, verbose)) {
    return -EBUSY;
    }
// Process args and tell the world that the torturer is on the job.
    while (i < ARRAY_SIZE!(torture_ops)) {
    cxt.cur_ops = torture_ops[i];
    if (strcmp(torture_type, cxt.cur_ops.name) == 0) {
    break;
    }
    }
    if (i == ARRAY_SIZE!(torture_ops)) {
    pr_alert("lock-torture: invalid torture type: \"%s\"\n",
    torture_type);
    pr_alert("lock-torture types:");
    for (i = 0; i < ARRAY_SIZE!(torture_ops); i++) {
    pr_alert(" %s", torture_ops[i].name);
    }
    pr_alert("\n");
    firsterr = -EINVAL;
// goto;
    }
    if (nwriters_stress == 0 &&
    (!cxt.cur_ops.readlock || nreaders_stress == 0)) {
    pr_alert("lock-torture: must run at least one locking thread\n");
    firsterr = -EINVAL;
// goto;
    }
    if (nwriters_stress >= 0) {
    cxt.nrealwriters_stress = nwriters_stress;
    }
    else {
    cxt.nrealwriters_stress = 2 * num_online_cpus();
    }
    if (cxt.cur_ops.init) {
    cxt.cur_ops.init();
    cxt.init_called = true;
    }

    if (str_has_prefix(torture_type, "mutex")) {
    cxt.debug_lock = true;
    }

    if (str_has_prefix(torture_type, "rtmutex")) {
    cxt.debug_lock = true;
    }

    if ((str_has_prefix(torture_type, "spin")) ||
    (str_has_prefix(torture_type, "rw_lock"))) {
    cxt.debug_lock = true;
    }

// Initialize the statistics so that each run gets its own numbers.
    if (nwriters_stress) {
    lock_is_write_held = false;
    cxt.lwsa = kmalloc_objs(*cxt.lwsa, cxt.nrealwriters_stress);
    if (cxt.lwsa == core::ptr::null_mut()) {
    VERBOSE_TOROUT_STRING("cxt.lwsa: Out of memory");
    firsterr = -ENOMEM;
// goto;
    }
    while (i < cxt.nrealwriters_stress) {
    cxt.lwsa[i].n_lock_fail = 0;
    cxt.lwsa[i].n_lock_acquired = 0;
    }
    }
    if (cxt.cur_ops.readlock) {
    if (nreaders_stress >= 0) {
    cxt.nrealreaders_stress = nreaders_stress;
    }
    else {
//
// By default distribute evenly the number of
// readers and writers. We still run the same number
// of threads as the writer-only locks default.
//
    if (nwriters_stress < 0) /* user doesn't care */ {
    cxt.nrealwriters_stress = num_online_cpus();
    }
    cxt.nrealreaders_stress = cxt.nrealwriters_stress;
    }
    if (nreaders_stress) {
    cxt.lrsa = kmalloc_objs(*cxt.lrsa,
    cxt.nrealreaders_stress);
    if (cxt.lrsa == core::ptr::null_mut()) {
    VERBOSE_TOROUT_STRING("cxt.lrsa: Out of memory");
    firsterr = -ENOMEM;
    kfree(cxt.lwsa);
    cxt.lwsa = core::ptr::null_mut();
// goto;
    }
    while (i < cxt.nrealreaders_stress) {
    cxt.lrsa[i].n_lock_fail = 0;
    cxt.lrsa[i].n_lock_acquired = 0;
    }
    }
    }
    firsterr = call_rcu_chain_init();
    if (torture_init_error(firsterr)) {
// goto;
    }
    lock_torture_print_module_parms(cxt.cur_ops, "Start of test");
// Prepare torture context.
    if (onoff_interval > 0) {
    firsterr = torture_onoff_init(onoff_holdoff * HZ,
    onoff_interval * HZ, core::ptr::null_mut());
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    if (shuffle_interval > 0) {
    firsterr = torture_shuffle_init(shuffle_interval);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    if (shutdown_secs > 0) {
    firsterr = torture_shutdown_init(shutdown_secs,
    lock_torture_cleanup);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    if (stutter > 0) {
    firsterr = torture_stutter_init(stutter, stutter);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    if (nwriters_stress) {
    writer_tasks = kzalloc_objs(writer_tasks[0],
    cxt.nrealwriters_stress);
    if (writer_tasks == core::ptr::null_mut()) {
    TOROUT_ERRSTRING("writer_tasks: Out of memory");
    firsterr = -ENOMEM;
// goto;
    }
    }
// cap nested_locks to MAX_NESTED_LOCKS
    if (nested_locks > MAX_NESTED_LOCKS) {
    nested_locks = MAX_NESTED_LOCKS;
    }
    if (cxt.cur_ops.readlock) {
    reader_tasks = kzalloc_objs(reader_tasks[0],
    cxt.nrealreaders_stress);
    if (reader_tasks == core::ptr::null_mut()) {
    TOROUT_ERRSTRING("reader_tasks: Out of memory");
    kfree(writer_tasks);
    writer_tasks = core::ptr::null_mut();
    firsterr = -ENOMEM;
// goto;
    }
    }
//
// Create the kthreads and start torturing (oh, those poor little locks).
//
// TODO: Note that we interleave writers with readers, giving writers a
// slight advantage, by creating its kthread first. This can be modified
// for very specific needs, or even let the user choose the policy, if
// ever wanted.
//
    while (i < cxt.nrealwriters_stress ||
    j < cxt.nrealreaders_stress) {
    if (i >= cxt.nrealwriters_stress) {
// goto;
    }
// Create writer.
    firsterr = torture_create_kthread_cb(lock_torture_writer, &cxt.lwsa[i],
    writer_tasks[i],
    writer_fifo ? sched_set_fifo : core::ptr::null_mut());
    if (torture_init_error(firsterr)) {
// goto;
    }
    if (cpumask_nonempty(bind_writers)) {
    torture_sched_setaffinity(writer_tasks[i].pid, bind_writers, true);
    }
// label;
    if (cxt.cur_ops.readlock == core::ptr::null_mut() || (j >= cxt.nrealreaders_stress)) {
    continue;
    }
// Create reader.
    firsterr = torture_create_kthread(lock_torture_reader, &cxt.lrsa[j],
    reader_tasks[j]);
    if (torture_init_error(firsterr)) {
// goto;
    }
    if (cpumask_nonempty(bind_readers)) {
    torture_sched_setaffinity(reader_tasks[j].pid, bind_readers, true);
    }
    }
    if (stat_interval > 0) {
    firsterr = torture_create_kthread(lock_torture_stats, core::ptr::null_mut(),
    stats_task);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    torture_init_end();
    return 0;
// label;
    torture_init_end();
    lock_torture_cleanup();
    if (shutdown_secs) {
    WARN_ON!(!IS_MODULE(CONFIG_LOCK_TORTURE_TEST));
    kernel_power_off();
    }
    return firsterr;
    }
    module_init!(lock_torture_init);
    module_exit!(lock_torture_cleanup);