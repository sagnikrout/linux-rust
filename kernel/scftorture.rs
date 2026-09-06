//! Automatically rewritten from C to Rust
//! Source: kernel/scftorture.c
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



// SPDX-License-Identifier: GPL-2.0+
//
// Torture test for smp_call_function() and friends.
//
// Copyright (C) Facebook, 2020.
//
// Author: Paul E. McKenney <paulmck@kernel.org>

    do { if (verbose) pr_alert(SCFTORT_FLAG s "\n", ## x); } while (0)

    MODULE_DESCRIPTION("Torture tests on the smp_call_function() family of primitives");
    MODULE_LICENSE("GPL");
    MODULE_AUTHOR("Paul E. McKenney <paulmck@kernel.org>");
// Wait until there are multiple CPUs before starting test.
    torture_param(int, holdoff, IS_BUILTIN(CONFIG_SCF_TORTURE_TEST) ? 10 : 0,
    "Holdoff time before test start (s)");
    torture_param(int, longwait, 0, "Include ridiculously long waits? (seconds)");
    torture_param(int, nthreads, -1, "# threads, defaults to -1 for all CPUs.");
    torture_param(int, onoff_holdoff, 0, "Time after boot before CPU hotplugs (s)");
    torture_param(int, onoff_interval, 0, "Time between CPU hotplugs (s), 0=disable");
    torture_param(int, shutdown_secs, 0, "Shutdown time (ms), <= zero to disable.");
    torture_param(int, stat_interval, 60, "Number of seconds between stats printk()s.");
    torture_param(int, stutter, 5, "Number of jiffies to run/halt test, 0=disable");
    torture_param(bool, use_cpus_read_lock, 0, "Use cpus_read_lock() to exclude CPU hotplug.");
    torture_param(int, verbose, 0, "Enable verbose debugging printk()s");
    torture_param(int, weight_resched, -1, "Testing weight for resched_cpu() operations.");
    torture_param(int, weight_single, -1, "Testing weight for single-CPU no-wait operations.");
    torture_param(int, weight_single_rpc, -1, "Testing weight for single-CPU RPC operations.");
    torture_param(int, weight_single_wait, -1, "Testing weight for single-CPU operations.");
    torture_param(int, weight_many, -1, "Testing weight for multi-CPU no-wait operations.");
    torture_param(int, weight_many_wait, -1, "Testing weight for multi-CPU operations.");
    torture_param(int, weight_all, -1, "Testing weight for all-CPU no-wait operations.");
    torture_param(int, weight_all_wait, -1, "Testing weight for all-CPU operations.");
    static char *torture_type = "";

    torture_param(bool, shutdown, SCFTORT_SHUTDOWN, "Shutdown at end of torture test.");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scf_statistics {
    pub task: *mut task_struct,
    pub cpu: c_int,
    pub n_resched: c_longlong,
    pub n_single: c_longlong,
    pub n_single_ofl: c_longlong,
    pub n_single_rpc: c_longlong,
    pub n_single_rpc_ofl: c_longlong,
    pub n_single_wait: c_longlong,
    pub n_single_wait_ofl: c_longlong,
    pub n_many: c_longlong,
    pub n_many_wait: c_longlong,
    pub n_all: c_longlong,
    pub n_all_wait: c_longlong,
}

pub static mut scf_stats_p: *mut c_void = core::ptr::null_mut();
pub static mut scf_torture_stats_task: *mut c_void = core::ptr::null_mut();
// static DEFINE_PER_CPU(long long, scf_invoked_count);
// static DEFINE_PER_CPU(llist_head, scf_free_pool);
// Data for random primitive selection
pub const SCF_PRIM_RESCHED: c_int = 0;
pub const SCF_PRIM_SINGLE: c_int = 1;
pub const SCF_PRIM_SINGLE_RPC: c_int = 2;
pub const SCF_PRIM_MANY: c_int = 3;
pub const SCF_PRIM_ALL: c_int = 4;

// except for SCF_PRIM_RESCHED and
// SCF_PRIM_SINGLE_RPC.
    static char *scf_prim_name[] = {
    "resched_cpu",
    "smp_call_function_single",
    "smp_call_function_single_rpc",
    "smp_call_function_many",
    "smp_call_function",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scf_selector {
    pub scfs_weight: c_ulong,
    pub scfs_prim: c_int,
    pub scfs_wait: bool,
}

    static struct scf_selector scf_sel_array[SCF_NPRIMS];
    static int scf_sel_array_len;
    static unsigned long scf_sel_totweight;
// Communicate between caller and handler.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scf_check {
    pub scfc_in: bool,
    pub scfc_out: bool,
    pub _single().: int scfc_cpu; // -1 for not,
    pub scfc_wait: bool,
    pub scfc_rpc: bool,
    pub scfc_completion: completion,
    pub scf_node: llist_node,
}

// Use to wait for all threads to start.
    static atomic_t n_started;
    static atomic_t n_errs;
    static atomic_t n_mb_in_errs;
    static atomic_t n_mb_out_errs;
    static atomic_t n_alloc_errs;
    static bool scfdone;
    static char *bangstr = "";
// static DEFINE_TORTURE_RANDOM_PERCPU(scf_torture_rand);
// forward_decl: resched_cpu; // An alternative IPI vector.
#[no_mangle]
unsafe extern "C" fn scf_add_to_free_list(scfcp: *mut scf_check) {
pub static mut pool: *mut c_void = core::ptr::null_mut();
    let mut cpu = 0;
    if (!scfcp) {
    return;
    }
    cpu = raw_smp_processor_id() % nthreads;
    pool = &per_cpu(scf_free_pool, cpu);
    llist_add(&scfcp.scf_node, pool);
    }
#[no_mangle]
unsafe extern "C" fn scf_cleanup_free_list(cpu: c_uint) {
pub static mut pool: *mut c_void = core::ptr::null_mut();
pub static mut node: *mut c_void = core::ptr::null_mut();
pub static mut scfcp: *mut c_void = core::ptr::null_mut();
    pool = &per_cpu(scf_free_pool, cpu);
    node = llist_del_all(pool);
    while (node) {
    scfcp = llist_entry(node, scf_check, scf_node);
    node = node.next;
    kfree(scfcp);
    }
    }
// Print torture statistics.  Caller must ensure serialization.
#[no_mangle]
unsafe extern "C" fn scf_torture_stats_print() {
    let mut cpu = 0;
    let mut i = 0;
pub static mut invoked_count: c_longlong = 0;
pub static mut isdone: bool = false;
pub static mut scfs: scf_statistics = 0;
    for_each_possible_cpu(cpu) {
    invoked_count += data_race(per_cpu(scf_invoked_count, cpu));
    }
    while (i < nthreads) {
    scfs.n_resched += scf_stats_p[i].n_resched;
    scfs.n_single += scf_stats_p[i].n_single;
    scfs.n_single_ofl += scf_stats_p[i].n_single_ofl;
    scfs.n_single_rpc += scf_stats_p[i].n_single_rpc;
    scfs.n_single_rpc_ofl += scf_stats_p[i].n_single_rpc_ofl;
    scfs.n_single_wait += scf_stats_p[i].n_single_wait;
    scfs.n_single_wait_ofl += scf_stats_p[i].n_single_wait_ofl;
    scfs.n_many += scf_stats_p[i].n_many;
    scfs.n_many_wait += scf_stats_p[i].n_many_wait;
    scfs.n_all += scf_stats_p[i].n_all;
    scfs.n_all_wait += scf_stats_p[i].n_all_wait;
    }
    if (atomic_read(&n_errs) || atomic_read(&n_mb_in_errs) ||
    atomic_read(&n_mb_out_errs) ||
    (!IS_ENABLED!(CONFIG_KASAN) && atomic_read(&n_alloc_errs))) {
    bangstr = "!!! ";
    }
    pr_alert("%s %sscf_invoked_count %s: %lld resched: %lld single: %lld/%lld single_ofl: %lld/%lld single_rpc: %lld single_rpc_ofl: %lld many: %lld/%lld all: %lld/%lld ",
    SCFTORT_FLAG, bangstr, isdone ? "VER" : "ver", invoked_count, scfs.n_resched,
    scfs.n_single, scfs.n_single_wait, scfs.n_single_ofl, scfs.n_single_wait_ofl,
    scfs.n_single_rpc, scfs.n_single_rpc_ofl,
    scfs.n_many, scfs.n_many_wait, scfs.n_all, scfs.n_all_wait);
    torture_onoff_stats();
    pr_cont("ste: %d stnmie: %d stnmoe: %d staf: %d\n", atomic_read(&n_errs),
    atomic_read(&n_mb_in_errs), atomic_read(&n_mb_out_errs),
    atomic_read(&n_alloc_errs));
    }
// Periodically prints torture statistics, if periodic statistics printing
// was specified via the stat_interval module parameter.
#[no_mangle]
pub unsafe extern "C" fn scf_torture_stats(arg: *mut c_void) -> c_int {
    VERBOSE_TOROUT_STRING("scf_torture_stats task started");
    do {
    schedule_timeout_interruptible(stat_interval * HZ);
    scf_torture_stats_print();
    torture_shutdown_absorb("scf_torture_stats");
    } while (!torture_must_stop());
    torture_kthread_stopping("scf_torture_stats");
    return 0;
    }
// Add a primitive to the scf_sel_array[].
#[no_mangle]
unsafe extern "C" fn scf_sel_add(weight: c_ulong, prim: c_int, wait: bool) {
    let mut scfsp = &scf_sel_array[scf_sel_array_len];
// If no weight, if array would overflow, if computing three-place
// percentages would overflow, or if the scf_prim_name[] array would
// overflow, don't bother.  In the last three two cases, complain.
    if (!weight ||
    WARN_ON_ONCE!(scf_sel_array_len >= ARRAY_SIZE!(scf_sel_array)) ||
    WARN_ON_ONCE!(0 - 100000 * weight <= 100000 * scf_sel_totweight) ||
    WARN_ON_ONCE!(prim >= ARRAY_SIZE!(scf_prim_name))) {
    return;
    }
    scf_sel_totweight += weight;
    scfsp.scfs_weight = scf_sel_totweight;
    scfsp.scfs_prim = prim;
    scfsp.scfs_wait = wait;
    scf_sel_array_len += 1;
    }
// Dump out weighting percentages for scf_prim_name[] array.
#[no_mangle]
unsafe extern "C" fn scf_sel_dump() {
    let mut i = 0;
pub static mut oldw: c_ulong = 0;
pub static mut scfsp: *mut c_void = core::ptr::null_mut();
    let mut w = 0;
    while (i < scf_sel_array_len) {
    scfsp = &scf_sel_array[i];
    w = (scfsp.scfs_weight - oldw) * 100000 / scf_sel_totweight;
    pr_info!("%s: %3lu.%03lu %s(%s)\n", __func__, w / 1000, w % 1000,
    scf_prim_name[scfsp.scfs_prim],
    scfsp.scfs_wait ? "wait" : "nowait");
    oldw = scfsp.scfs_weight;
    }
    }
// Randomly pick a primitive and wait/nowait, based on weightings.
#[no_mangle]
pub unsafe extern "C" fn scf_sel_rand(trsp: *mut torture_random_state) -> *mut c_void {
    let mut i = 0;
pub static mut w: c_ulong = 0;
    for (i = 0; i < scf_sel_array_len; i++) {
    if (scf_sel_array[i].scfs_weight >= w)
    return &scf_sel_array[i];
    }
    WARN_ON_ONCE!(1);
    return &scf_sel_array[0];
    }
// Update statistics and occasionally burn up mass quantities of CPU time,
// if told to do so via scftorture.longwait.  Otherwise, occasionally burn
// a little bit.
#[no_mangle]
unsafe extern "C" fn scf_handler(scfc_in: *mut c_void) {
    let mut i = 0;
    let mut j = 0;
pub static mut r: c_ulong = 0;
    let mut scfcp = scfc_in;
    if (likely(scfcp)) {
    WRITE_ONCE(scfcp.scfc_out, false); // For multiple receivers.
    if (WARN_ON_ONCE!(unlikely(!READ_ONCE(scfcp.scfc_in)))) {
    atomic_inc(&n_mb_in_errs);
    }
    }
    this_cpu_inc(scf_invoked_count);
    if (longwait <= 0) {
    if (!(r & 0xffc0)) {
    udelay(r & 0x3f);
// goto;
    }
    }
    if (r & 0xfff) {
// goto;
    }
    r = (r >> 12);
    if (longwait <= 0) {
    udelay((r & 0xff) + 1);
// goto;
    }
    r = r % longwait + 1;
    while (i < r) {
    while (j < 1000) {
    udelay(1000);
    cpu_relax();
    }
    }
// label;
    if (unlikely(!scfcp)) {
    return;
    }
    if (scfcp.scfc_wait) {
    WRITE_ONCE(scfcp.scfc_out, true);
    if (scfcp.scfc_rpc) {
    complete(&scfcp.scfc_completion);
    }
    } else {
    scf_add_to_free_list(scfcp);
    }
    }
// As above, but check for correct CPU.
#[no_mangle]
unsafe extern "C" fn scf_handler_1(scfc_in: *mut c_void) {
    let mut scfcp = scfc_in;
    if (likely(scfcp) && WARN_ONCE(smp_processor_id() != scfcp.scfc_cpu, "%s: Wanted CPU %d got CPU %d\n", __func__, scfcp.scfc_cpu, smp_processor_id())) {
    atomic_inc(&n_errs);
    }
    scf_handler(scfcp);
    }
// Randomly do an smp_call_function*() invocation.
#[no_mangle]
unsafe extern "C" fn scftorture_invoke_one(scfp: *mut scf_statistics, trsp: *mut torture_random_state) {
pub static mut allocfail: bool = false;
    let mut cpu;
pub static mut ret: c_int = 0;
    let mut scfcp = core::ptr::null_mut();
    let mut scfsp = scf_sel_rand(trsp);
    let mut is_single = (scfsp.scfs_prim == SCF_PRIM_SINGLE ||
    scfsp.scfs_prim == SCF_PRIM_SINGLE_RPC);
    if (scfsp.scfs_prim == SCF_PRIM_SINGLE || scfsp.scfs_wait) {
    scfcp = kmalloc_obj(*scfcp, GFP_ATOMIC);
    if (!scfcp) {
    WARN_ON_ONCE!(!IS_ENABLED!(CONFIG_KASAN));
    atomic_inc(&n_alloc_errs);
    allocfail = true;
    } else {
    scfcp.scfc_cpu = -1;
    scfcp.scfc_wait = scfsp.scfs_wait;
    scfcp.scfc_out = false;
    scfcp.scfc_rpc = false;
    }
    }
    if (use_cpus_read_lock) {
    cpus_read_lock();
    }
    match (scfsp.scfs_prim) {
    SCF_PRIM_RESCHED => {
    if (IS_BUILTIN(CONFIG_SCF_TORTURE_TEST)) {
    cpu = torture_random(trsp) % nr_cpu_ids;
    scfp.n_resched += 1;
    resched_cpu(cpu);
    this_cpu_inc(scf_invoked_count);
    }
    // break;
    }
    SCF_PRIM_SINGLE => {
    cpu = torture_random(trsp) % nr_cpu_ids;
    if (scfsp.scfs_wait) {
    scfp.n_single_wait += 1;
    }
    else {
    scfp.n_single += 1;
    }
    if (scfcp) {
    scfcp.scfc_cpu = cpu;
    barrier(); // Prevent race-reduction compiler optimizations.
    scfcp.scfc_in = true;
    }
    ret = smp_call_function_single(cpu, scf_handler_1, scfcp, scfsp.scfs_wait);
    if (ret) {
    if (scfsp.scfs_wait) {
    scfp.n_single_wait_ofl += 1;
    }
    else {
    scfp.n_single_ofl += 1;
    }
    scf_add_to_free_list(scfcp);
    scfcp = core::ptr::null_mut();
    }
    // break;
    }
    SCF_PRIM_SINGLE_RPC => {
    if (!scfcp) {
    // break;
    }
    cpu = torture_random(trsp) % nr_cpu_ids;
    scfp.n_single_rpc += 1;
    scfcp.scfc_cpu = cpu;
    scfcp.scfc_wait = true;
    init_completion(&scfcp.scfc_completion);
    scfcp.scfc_rpc = true;
    barrier(); // Prevent race-reduction compiler optimizations.
    scfcp.scfc_in = true;
    ret = smp_call_function_single(cpu, scf_handler_1, scfcp, 0);
    if (!ret) {
    if (use_cpus_read_lock) {
    cpus_read_unlock();
    }
    wait_for_completion(&scfcp.scfc_completion);
    if (use_cpus_read_lock) {
    cpus_read_lock();
    }
    } else {
    scfp.n_single_rpc_ofl += 1;
    scf_add_to_free_list(scfcp);
    scfcp = core::ptr::null_mut();
    }
    // break;
    }
    SCF_PRIM_MANY => {
    if (scfsp.scfs_wait) {
    scfp.n_many_wait += 1;
    }
    else {
    scfp.n_many += 1;
    }
    if (scfcp) {
    barrier(); // Prevent race-reduction compiler optimizations.
    scfcp.scfc_in = true;
    }
    smp_call_function_many(cpu_online_mask, scf_handler, scfcp, scfsp.scfs_wait);
    // break;
    }
    SCF_PRIM_ALL => {
    if (scfsp.scfs_wait) {
    scfp.n_all_wait += 1;
    }
    else {
    scfp.n_all += 1;
    }
    if (scfcp) {
    barrier(); // Prevent race-reduction compiler optimizations.
    scfcp.scfc_in = true;
    }
    smp_call_function(scf_handler, scfcp, scfsp.scfs_wait);
    // break;
    }
    _ => {
    WARN_ON_ONCE!(1);
    if (scfcp) {
    scfcp.scfc_out = true;
    }
    }
    }
    if (scfcp && scfsp.scfs_wait) {
    if (WARN_ON_ONCE!(((use_cpus_read_lock && num_online_cpus() > 1) || is_single) &&
    !scfcp.scfc_out)) {
    pr_warn!("%s: Memory-ordering failure, scfs_prim: %d.\n", __func__, scfsp.scfs_prim);
    atomic_inc(&n_mb_out_errs); // Leak rather than trash!
    } else {
    scf_add_to_free_list(scfcp);
    }
    barrier(); // Prevent race-reduction compiler optimizations.
    }
    if (use_cpus_read_lock) {
    cpus_read_unlock();
    }
    if (allocfail) {
    schedule_timeout_idle((1 + longwait) * HZ);  // Let no-wait handlers complete.
    }

    else if (!(torture_random(trsp) & 0xfff)) {
    schedule_timeout_uninterruptible(1);
    }
    }
// SCF test kthread.  Repeatedly does calls to members of the
// smp_call_function() family of functions.
#[no_mangle]
unsafe extern "C" fn scftorture_invoker(arg: *mut c_void) -> c_int {
    let mut cpu = 0;
    let mut curcpu = 0;
pub static mut rand: usize = 0;
    let mut scfp = arg;
pub static mut was_offline: bool = false;
    VERBOSE_SCFTORTOUT("scftorture_invoker %d: task started", scfp.cpu);
    cpu = scfp.cpu % nr_cpu_ids;
    WARN_ON_ONCE!(set_cpus_allowed_ptr(current, cpumask_of(cpu)));
    set_user_nice(current, MAX_NICE);
    if (holdoff) {
    schedule_timeout_interruptible(holdoff * HZ);
    }
    VERBOSE_SCFTORTOUT("scftorture_invoker %d: Waiting for all SCF torturers from cpu %d", scfp.cpu, raw_smp_processor_id());
// Make sure that the CPU is affinitized appropriately during testing.
    curcpu = raw_smp_processor_id();
    WARN_ONCE(curcpu != cpu,
    "%s: Wanted CPU %d, running on %d, nr_cpu_ids = %d\n",
    __func__, scfp.cpu, curcpu, nr_cpu_ids);
    if (atomic_dec_return(&n_started)) {
    while (atomic_read_acquire(&n_started)) {
    }
    if (torture_must_stop()) {
    VERBOSE_SCFTORTOUT("scftorture_invoker %d ended before starting", scfp.cpu);
// goto;
    }
    schedule_timeout_uninterruptible(1);
    }
    VERBOSE_SCFTORTOUT("scftorture_invoker %d started", scfp.cpu);
    do {
    scf_cleanup_free_list(cpu);
    scftorture_invoke_one(scfp, &rand);
    while (cpu_is_offline(cpu) && !torture_must_stop()) {
    schedule_timeout_interruptible(HZ / 5);
    was_offline = true;
    }
    if (was_offline) {
    set_cpus_allowed_ptr(current, cpumask_of(cpu));
    was_offline = false;
    }
    cond_resched();
    stutter_wait("scftorture_invoker");
    } while (!torture_must_stop());
    VERBOSE_SCFTORTOUT("scftorture_invoker %d ended", scfp.cpu);
// label;
    torture_kthread_stopping("scftorture_invoker");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn scftorture_print_module_parms(tag: *mut c_char) {
    pr_alert(SCFTORT_FLAG
    "--- %s:  verbose=%d holdoff=%d longwait=%d nthreads=%d onoff_holdoff=%d onoff_interval=%d shutdown_secs=%d stat_interval=%d stutter=%d use_cpus_read_lock=%d, weight_resched=%d, weight_single=%d, weight_single_rpc=%d, weight_single_wait=%d, weight_many=%d, weight_many_wait=%d, weight_all=%d, weight_all_wait=%d\n", tag,
    verbose, holdoff, longwait, nthreads, onoff_holdoff, onoff_interval, shutdown, stat_interval, stutter, use_cpus_read_lock, weight_resched, weight_single, weight_single_rpc, weight_single_wait, weight_many, weight_many_wait, weight_all, weight_all_wait);
    }
#[no_mangle]
unsafe extern "C" fn scf_cleanup_handler(unused: *mut c_void) {
    }
#[no_mangle]
unsafe extern "C" fn scf_torture_cleanup() {
    let mut i = 0;
    if (torture_cleanup_begin()) {
    return;
    }
    WRITE_ONCE(scfdone, true);
    if (nthreads && scf_stats_p) {
    for (i = 0; i < nthreads; i++)
    }
    torture_stop_kthread("scftorture_invoker", scf_stats_p[i].task);
    else {
// goto;
    }
    smp_call_function(scf_cleanup_handler, core::ptr::null_mut(), 1);
    torture_stop_kthread(scf_torture_stats, scf_torture_stats_task);
    scf_torture_stats_print();  // -After- the stats thread is stopped!
    kfree(scf_stats_p);  // -After- the last stats print has completed!
    scf_stats_p = core::ptr::null_mut();
    for (i = 0; i < nr_cpu_ids; i++) {
    scf_cleanup_free_list(i);
    }
    if (atomic_read(&n_errs) || atomic_read(&n_mb_in_errs) || atomic_read(&n_mb_out_errs)) {
    scftorture_print_module_parms("End of test: FAILURE");
    }

    else if (torture_onoff_failures()) {
    scftorture_print_module_parms("End of test: LOCK_HOTPLUG");
    }
    else {
    scftorture_print_module_parms("End of test: SUCCESS");
    }
// label;
    torture_cleanup_end();
    }
#[no_mangle]
unsafe extern "C" fn scf_torture_init() -> c_int {
    let mut i = 0;
pub static mut firsterr: c_int = 0;
pub static mut weight_resched1: c_ulong = 0;
pub static mut weight_single1: c_ulong = 0;
pub static mut weight_single_rpc1: c_ulong = 0;
pub static mut weight_single_wait1: c_ulong = 0;
pub static mut weight_many1: c_ulong = 0;
pub static mut weight_many_wait1: c_ulong = 0;
pub static mut weight_all1: c_ulong = 0;
pub static mut weight_all_wait1: c_ulong = 0;
    if (!torture_init_begin(SCFTORT_STRING, verbose)) {
    return -EBUSY;
    }
    scftorture_print_module_parms("Start of test");
    if (weight_resched <= 0 &&
    weight_single <= 0 && weight_single_rpc <= 0 && weight_single_wait <= 0 &&
    weight_many <= 0 && weight_many_wait <= 0 &&
    weight_all <= 0 && weight_all_wait <= 0) {
    weight_resched1 = weight_resched == 0 ? 0 : 2 * nr_cpu_ids;
    weight_single1 = weight_single == 0 ? 0 : 2 * nr_cpu_ids;
    weight_single_rpc1 = weight_single_rpc == 0 ? 0 : 2 * nr_cpu_ids;
    weight_single_wait1 = weight_single_wait == 0 ? 0 : 2 * nr_cpu_ids;
    weight_many1 = weight_many == 0 ? 0 : 2;
    weight_many_wait1 = weight_many_wait == 0 ? 0 : 2;
    weight_all1 = weight_all == 0 ? 0 : 1;
    weight_all_wait1 = weight_all_wait == 0 ? 0 : 1;
    } else {
    if (weight_resched == -1) {
    weight_resched1 = 0;
    }
    if (weight_single == -1) {
    weight_single1 = 0;
    }
    if (weight_single_rpc == -1) {
    weight_single_rpc1 = 0;
    }
    if (weight_single_wait == -1) {
    weight_single_wait1 = 0;
    }
    if (weight_many == -1) {
    weight_many1 = 0;
    }
    if (weight_many_wait == -1) {
    weight_many_wait1 = 0;
    }
    if (weight_all == -1) {
    weight_all1 = 0;
    }
    if (weight_all_wait == -1) {
    weight_all_wait1 = 0;
    }
    }
    if (weight_resched1 == 0 && weight_single1 == 0 && weight_single_rpc1 == 0 &&
    weight_single_wait1 == 0 && weight_many1 == 0 && weight_many_wait1 == 0 &&
    weight_all1 == 0 && weight_all_wait1 == 0) {
    SCFTORTOUT_ERRSTRING("all zero weights makes no sense");
    firsterr = -EINVAL;
// goto;
    }
    if (IS_BUILTIN(CONFIG_SCF_TORTURE_TEST)) {
    scf_sel_add(weight_resched1, SCF_PRIM_RESCHED, false);
    }

    else if (weight_resched1) {
    SCFTORTOUT_ERRSTRING("built as module, weight_resched ignored");
    }
    scf_sel_add(weight_single1, SCF_PRIM_SINGLE, false);
    scf_sel_add(weight_single_rpc1, SCF_PRIM_SINGLE_RPC, true);
    scf_sel_add(weight_single_wait1, SCF_PRIM_SINGLE, true);
    scf_sel_add(weight_many1, SCF_PRIM_MANY, false);
    scf_sel_add(weight_many_wait1, SCF_PRIM_MANY, true);
    scf_sel_add(weight_all1, SCF_PRIM_ALL, false);
    scf_sel_add(weight_all_wait1, SCF_PRIM_ALL, true);
    scf_sel_dump();
    if (onoff_interval > 0) {
    firsterr = torture_onoff_init(onoff_holdoff * HZ, onoff_interval, core::ptr::null_mut());
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    if (shutdown_secs > 0) {
    firsterr = torture_shutdown_init(shutdown_secs, scf_torture_cleanup);
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
// Worker tasks invoking smp_call_function().
    if (nthreads < 0) {
    nthreads = num_online_cpus();
    }
    scf_stats_p = kzalloc_objs(scf_stats_p[0], nthreads);
    if (!scf_stats_p) {
    SCFTORTOUT_ERRSTRING("out of memory");
    firsterr = -ENOMEM;
// goto;
    }
    VERBOSE_SCFTORTOUT("Starting %d smp_call_function() threads", nthreads);
    atomic_set(&n_started, nthreads);
    while (i < nthreads) {
    scf_stats_p[i].cpu = i;
    firsterr = torture_create_kthread(scftorture_invoker, &scf_stats_p[i],
    scf_stats_p[i].task);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    if (stat_interval > 0) {
    firsterr = torture_create_kthread(scf_torture_stats, core::ptr::null_mut(), scf_torture_stats_task);
    if (torture_init_error(firsterr)) {
// goto;
    }
    }
    torture_init_end();
    return 0;
// label;
    torture_init_end();
    scf_torture_cleanup();
    if (shutdown_secs) {
    WARN_ON!(!IS_MODULE(CONFIG_SCF_TORTURE_TEST));
    kernel_power_off();
    }
    return firsterr;
    }
    module_init!(scf_torture_init);
    module_exit!(scf_torture_cleanup);