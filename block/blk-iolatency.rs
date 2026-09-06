//! Automatically rewritten from C to Rust
//! Source: block/blk-iolatency.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Block rq-qos base io controller
//
// This works similar to wbt with a few exceptions
//
// - It's bio based, so the latency covers the whole block layer in addition to
// the actual io.
// - We will throttle all IO that comes in here if we need to.
// - We use the mean latency over the 100ms window.  This is because writes can
// be particularly fast, which could give us a false sense of the impact of
// other workloads on our protected workload.
// - By default there's no throttling, we set the queue_depth to UINT_MAX so
// that we can have as many outstanding bio's as we're allowed to.  Only at
// throttle time do we pay attention to the actual queue depth.
//
// The hierarchy works like the cpu controller does, we track the latency at
// every configured node, and each configured node has it's own independent
// queue depth.  This means that we only care about our latency targets at the
// peer level.  Some group at the bottom of the hierarchy isn't going to affect
// a group at the end of some other path if we're only configred at leaf level.
//
// Consider the following
//
// root blkg
// /                     
// fast (target=5ms)     slow (target=10ms)
// /     \                  /        
// a        b          normal(15ms)   unloved
//
// "a" and "b" have no target, but their combined io under "fast" cannot exceed
// an average latency of 5ms.  If it does then we will throttle the "slow"
// group.  In the case of "normal", if it exceeds its 15ms target, we will
// throttle "unloved", but nobody else.
//
// In this example "fast", "slow", and "normal" will be the only groups actually
// accounting their io latencies.  We have to walk up the heirarchy to the root
// on every submit and complete so we can do the appropriate stat recording and
// adjust the queue depth of ourselves if needed.
//
// There are 2 ways we throttle IO.
//
// 1) Queue depth throttling.  As we throttle down we will adjust the maximum
// number of IO's we're allowed to have in flight.  This starts at (u64)-1 down
// to 1.  If the group is only ever submitting IO for itself then this is the
// only way we throttle.
//
// 2) Induced delay throttling.  This is for the case that a group is generating
// IO that has to be issued by the root cg to avoid priority inversion. So think
// REQ_META or REQ_SWAP.  If we are already at qd == 1 and we're getting a lot
// of work done for us on behalf of the root cg and are being asked to scale
// down more then we induce a latency at userspace return.  We accumulate the
// total amount of time we need to be punished by doing
//
// total_time += min_lat_nsec - actual_io_completion
//
// and then at throttle time will do
//
// throttle_time = min(total_time, NSEC_PER_SEC)
//
// This induced delay will throttle back the activity that is generating the
// root cg issued io's, wethere that's some metadata intensive operation or the
// group is using so much memory that it is pushing us into swap.
//
// Copyright (C) 2018 Josef Bacik
//

pub static mut blkcg_policy_iolatency: usize = 0;
    let mut iolatency_grp;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_iolatency {
    pub rqos: rq_qos,
    pub timer: timer_list,
//
// ->enabled is the master enable switch gating the throttling logic and
// inflight tracking. The number of cgroups which have iolat enabled is
// tracked in ->enable_cnt, and ->enable is flipped on/off accordingly
// from ->enable_work with the request_queue frozen. For details, See
// blkiolatency_enable_work_fn().
//
    pub enabled: bool,
    pub enable_cnt: core::sync::atomic::AtomicI32,
    pub enable_work: work_struct,
}

#[no_mangle]
pub unsafe extern "C" fn BLKIOLATENCY(rqos: *mut rq_qos) -> *mut c_void {
    return container_of!(rqos, blk_iolatency, rqos);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct child_latency_info {
    pub lock: spinlock_t,
// Last time we adjusted the scale of everybody.
    pub last_scale_event: u64,
// The latency that we missed.
    pub scale_lat: u64,
// Total io's from all of our children for the last summation.
    pub nr_samples: u64,
// The guy who actually changed the latency numbers.
    pub scale_grp: *mut iolatency_grp,
// Cookie to tell if we need to scale up or down.
    pub scale_cookie: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percentile_stats {
    pub total: u64,
    pub missed: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct latency_stat {
    union {
    pub ps: percentile_stats,
    pub rqs: blk_rq_stat,
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iolatency_grp {
    pub pd: blkg_policy_data,
    pub stats: *mut latency_stat ,
    pub cur_stat: latency_stat,
    pub blkiolat: *mut blk_iolatency,
    pub max_depth: c_uint,
    pub rq_wait: rq_wait,
    pub window_start: core::sync::atomic::AtomicI64,
    pub scale_cookie: core::sync::atomic::AtomicI32,
    pub min_lat_nsec: u64,
    pub cur_win_nsec: u64,
// total running average of our io latency.
    pub lat_avg: u64,
// Our current number of IO's for the last summation.
    pub nr_samples: u64,
    pub ssd: bool,
    pub child_lat: child_latency_info,
}

//
// These are the constants used to fake the fixed-point moving average
// calculation just like load average.  The call to calc_load() folds
// (FIXED_1 (2048) - exp_factor) * new_sample into lat_avg.  The sampling
// window size is bucketed to try to approximately calculate average
// latency such that 1/exp (decay rate) is [1 min, 2.5 min) when windows
// elapse immediately.  Note, windows only elapse with IO activity.  Idle
// periods extend the most recent window.
//
pub const BLKIOLATENCY_NR_EXP_FACTORS: c_int = 5;

    (BLKIOLATENCY_NR_EXP_FACTORS - 1))
    static const u64 iolatency_exp_factors[BLKIOLATENCY_NR_EXP_FACTORS] = {
    2045, // exp(1/600) - 600 samples
    2039, // exp(1/240) - 240 samples
    2031, // exp(1/120) - 120 samples
    2023, // exp(1/80)  - 80 samples
    2014, // exp(1/60)  - 60 samples
    };
#[no_mangle]
pub unsafe extern "C" fn pd_to_lat(pd: *mut blkg_policy_data) -> *mut c_void {
    return pd ? container_of!(pd, iolatency_grp, pd) : core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn blkg_to_lat(blkg: *mut blkcg_gq) -> *mut c_void {
    return pd_to_lat(blkg_to_pd(blkg, &blkcg_policy_iolatency));
    }
#[no_mangle]
pub unsafe extern "C" fn lat_to_blkg(iolat: *mut iolatency_grp) -> *mut c_void {
    return pd_to_blkg(&iolat.pd);
    }
#[no_mangle]
pub unsafe extern "C" fn latency_stat_init(iolat: *mut iolatency_grp, stat: *mut latency_stat) {
    if (iolat.ssd) {
    stat.ps.total = 0;
    stat.ps.missed = 0;
    } else {
    blk_rq_stat_init(&stat.rqs);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn latency_stat_sum(iolat: *mut iolatency_grp, sum: *mut latency_stat, stat: *mut latency_stat) {
    if (iolat.ssd) {
    sum.ps.total += stat.ps.total;
    sum.ps.missed += stat.ps.missed;
    } else {
    blk_rq_stat_sum(&sum.rqs, &stat.rqs);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn latency_stat_record_time(iolat: *mut iolatency_grp, req_time: u64) {
    let mut stat = get_cpu_ptr(iolat.stats);
    if (iolat.ssd) {
    if (req_time >= iolat.min_lat_nsec) {
    stat.ps.missed += 1;
    }
    stat.ps.total += 1;
    } else {
    blk_rq_stat_add(&stat.rqs, req_time);
    }
    put_cpu_ptr(stat);
    }
#[no_mangle]
pub unsafe extern "C" fn latency_sum_ok(iolat: *mut iolatency_grp, stat: *mut latency_stat) -> bool {
    if (iolat.ssd) {
pub static mut thresh: u64 = 0;
    thresh = max(thresh, 1ULL);
    return stat.ps.missed < thresh;
    }
    return stat.rqs.mean <= iolat.min_lat_nsec;
    }
#[no_mangle]
pub unsafe extern "C" fn latency_stat_samples(iolat: *mut iolatency_grp, stat: *mut latency_stat) -> u64 {
    if (iolat.ssd) {
    return stat.ps.total;
    }
    return stat.rqs.nr_samples;
    }
#[no_mangle]
pub unsafe extern "C" fn iolat_update_total_lat_avg(iolat: *mut iolatency_grp, stat: *mut latency_stat) {
    let mut exp_idx = 0;
    if (iolat.ssd) {
    return;
    }
//
// calc_load() takes in a number stored in fixed point representation.
// Because we are using this for IO time in ns, the values stored
// are significantly larger than the FIXED_1 denominator (2048).
// Therefore, rounding errors in the calculation are negligible and
// can be ignored.
//
    exp_idx = min_t(int, BLKIOLATENCY_NR_EXP_FACTORS - 1,
    div64_u64(iolat.cur_win_nsec,
    BLKIOLATENCY_EXP_BUCKET_SIZE));
    iolat.lat_avg = calc_load(iolat.lat_avg,
    iolatency_exp_factors[exp_idx],
    stat.rqs.mean);
    }
#[no_mangle]
unsafe extern "C" fn iolat_cleanup_cb(rqw: *mut rq_wait, private_data: *mut c_void) {
    atomic_dec(&rqw.inflight);
    wake_up(&rqw.wait);
    }
#[no_mangle]
unsafe extern "C" fn iolat_acquire_inflight(rqw: *mut rq_wait, private_data: *mut c_void) -> bool {
    let mut iolat = private_data;
    return rq_wait_inc_below(rqw, iolat.max_depth);
    }
#[no_mangle]
pub unsafe extern "C" fn __blkcg_iolatency_throttle(rqos: *mut rq_qos, iolat: *mut iolatency_grp, issue_as_root: bool, use_memdelay: bool) {
    let mut rqw = &iolat.rq_wait;
pub static mut use_delay: unsigned = 0;
    if (use_delay) {
    blkcg_schedule_throttle(rqos.disk, use_memdelay);
    }
//
// To avoid priority inversions we want to just take a slot if we are
// issuing as root.  If we're being killed off there's no point in
// delaying things, we may have been killed by OOM so throttling may
// make recovery take even longer, so just let the IO's through so the
// task can go away.
//
    if (issue_as_root || fatal_signal_pending(current)) {
    atomic_inc(&rqw.inflight);
    return;
    }
    rq_qos_wait(rqw, iolat, iolat_acquire_inflight, iolat_cleanup_cb);
    }
pub const SCALE_DOWN_FACTOR: c_int = 2;
pub const SCALE_UP_FACTOR: c_int = 4;
#[no_mangle]
pub unsafe extern "C" fn scale_amount(qd: c_ulong, up: bool) -> c_ulong {
    return max(up ? qd >> SCALE_UP_FACTOR : qd >> SCALE_DOWN_FACTOR, 1UL);
    }
//
// We scale the qd down faster than we scale up, so we need to use this helper
// to adjust the scale_cookie accordingly so we don't prematurely get
// scale_cookie at DEFAULT_SCALE_COOKIE and unthrottle too much.
//
// Each group has their own local copy of the last scale cookie they saw, so if
// the global scale cookie goes up or down they know which way they need to go
// based on their last knowledge of it.
//
#[no_mangle]
pub unsafe extern "C" fn scale_cookie_change(blkiolat: *mut blk_iolatency, lat_info: *mut child_latency_info, up: bool) {
pub static mut qd: c_ulong = 0;
pub static mut scale: c_ulong = 0;
pub static mut old: c_ulong = 0;
pub static mut max_scale: c_ulong = 0;
pub static mut diff: c_ulong = 0;
    if (old < DEFAULT_SCALE_COOKIE) {
    diff = DEFAULT_SCALE_COOKIE - old;
    }
    if (up) {
    if (scale + old > DEFAULT_SCALE_COOKIE) {
    atomic_set(&lat_info.scale_cookie,
    DEFAULT_SCALE_COOKIE);
    }

    else if (diff > qd) {
    atomic_inc(&lat_info.scale_cookie);
    }
    else {
    atomic_add(scale, &lat_info.scale_cookie);
    }
    } else {
//
// We don't want to dig a hole so deep that it takes us hours to
// dig out of it.  Just enough that we don't throttle/unthrottle
// with jagged workloads but can still unthrottle once pressure
// has sufficiently dissipated.
//
    if (diff > qd) {
    if (diff < max_scale) {
    atomic_dec(&lat_info.scale_cookie);
    }
    } else {
    atomic_sub(scale, &lat_info.scale_cookie);
    }
    }
    }
//
// Change the queue depth of the iolatency_grp.  We add 1/16th of the
// queue depth at a time so we don't get wild swings and hopefully dial in to
// fairer distribution of the overall queue depth.  We halve the queue depth
// at a time so we can scale down queue depth quickly from default unlimited
// to target.
//
#[no_mangle]
unsafe extern "C" fn scale_change(iolat: *mut iolatency_grp, up: bool) {
pub static mut qd: c_ulong = 0;
pub static mut scale: c_ulong = 0;
pub static mut old: c_ulong = 0;
    if (old > qd) {
    old = qd;
    }
    if (up) {
    if (old == 1 && blkcg_unuse_delay(lat_to_blkg(iolat))) {
    return;
    }
    if (old < qd) {
    old += scale;
    old = min(old, qd);
    iolat.max_depth = old;
    wake_up_all(&iolat.rq_wait.wait);
    }
    } else {
    old >>= 1;
    iolat.max_depth = max(old, 1UL);
    }
    }
// Check our parent and see if the scale cookie has changed.
#[no_mangle]
unsafe extern "C" fn check_scale_change(iolat: *mut iolatency_grp) {
pub static mut parent: *mut c_void = core::ptr::null_mut();
pub static mut lat_info: *mut c_void = core::ptr::null_mut();
    let mut cur_cookie = 0;
pub static mut our_cookie: c_uint = 0;
    let mut scale_lat = 0;
pub static mut direction: c_int = 0;
    parent = blkg_to_lat(lat_to_blkg(iolat).parent);
    if (!parent) {
    return;
    }
    lat_info = &parent.child_lat;
    cur_cookie = atomic_read(&lat_info.scale_cookie);
    scale_lat = READ_ONCE(lat_info.scale_lat);
    if (cur_cookie < our_cookie) {
    direction = -1;
    }

    else if (cur_cookie > our_cookie) {
    direction = 1;
    }
    else {
    return;
    }
    if (!atomic_try_cmpxchg(&iolat.scale_cookie, &our_cookie, cur_cookie)) {
// Somebody beat us to the punch, just bail.
    return;
    }
    if (direction < 0 && iolat.min_lat_nsec) {
    let mut samples_thresh = 0;
    if (!scale_lat || iolat.min_lat_nsec <= scale_lat) {
    return;
    }
//
// Sometimes high priority groups are their own worst enemy, so
// instead of taking it out on some poor other group that did 5%
// or less of the IO's for the last summation just skip this
// scale down event.
//
    samples_thresh = lat_info.nr_samples * 5;
    samples_thresh = max(1ULL, div64_u64(samples_thresh, 100));
    if (iolat.nr_samples <= samples_thresh) {
    return;
    }
    }
// We're as low as we can go.
    if (iolat.max_depth == 1 && direction < 0) {
    blkcg_use_delay(lat_to_blkg(iolat));
    return;
    }
// We're back to the default cookie, unthrottle all the things.
    if (cur_cookie == DEFAULT_SCALE_COOKIE) {
    blkcg_clear_delay(lat_to_blkg(iolat));
    iolat.max_depth = UINT_MAX;
    wake_up_all(&iolat.rq_wait.wait);
    return;
    }
    scale_change(iolat, direction > 0);
    }
#[no_mangle]
unsafe extern "C" fn blkcg_iolatency_throttle(rqos: *mut rq_qos, bio: *mut bio) {
    let mut blkiolat = BLKIOLATENCY(rqos);
    let mut blkg = bio.bi_blkg;
pub static mut issue_as_root: bool = false;
    if (!blkiolat.enabled) {
    return;
    }
    while (blkg && blkg.parent) {
    let mut iolat = blkg_to_lat(blkg);
    if (!iolat) {
    blkg = blkg.parent;
    continue;
    }
    check_scale_change(iolat);
    __blkcg_iolatency_throttle(rqos, iolat, issue_as_root,
    (bio.bi_opf & REQ_SWAP) == REQ_SWAP);
    blkg = blkg.parent;
    }
    if (!timer_pending(&blkiolat.timer)) {
    mod_timer(&blkiolat.timer, jiffies + HZ);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn iolatency_record_time(iolat: *mut iolatency_grp, start: u64, now: u64, issue_as_root: bool) {
    let mut req_time = 0;
    if (now <= start) {
    return;
    }
    req_time = now - start;
//
// We don't want to count issue_as_root bio's in the cgroups latency
// statistics as it could skew the numbers downwards.
//
    if (unlikely(issue_as_root && iolat.max_depth != UINT_MAX)) {
pub static mut sub: u64 = 0;
    if (req_time < sub) {
    blkcg_add_delay(lat_to_blkg(iolat), now, sub - req_time);
    }
    return;
    }
    latency_stat_record_time(iolat, req_time);
    }

pub const BLKIOLATENCY_MIN_GOOD_SAMPLES: c_int = 5;
#[no_mangle]
unsafe extern "C" fn iolatency_check_latencies(iolat: *mut iolatency_grp, now: u64) {
    let mut blkg = lat_to_blkg(iolat);
pub static mut parent: *mut c_void = core::ptr::null_mut();
pub static mut lat_info: *mut c_void = core::ptr::null_mut();
pub static mut stat: usize = 0;
    let mut flags = 0;
    let mut cpu = 0;
    latency_stat_init(iolat, &stat);
    preempt_disable();
    for_each_possible_cpu(cpu) {
pub static mut s: *mut c_void = core::ptr::null_mut();
    s = per_cpu_ptr(iolat.stats, cpu);
    latency_stat_sum(iolat, &stat, s);
    latency_stat_init(iolat, s);
    }
    preempt_enable();
    parent = blkg_to_lat(blkg.parent);
    if (!parent) {
    return;
    }
    lat_info = &parent.child_lat;
    iolat_update_total_lat_avg(iolat, &stat);
// Everything is ok and we don't need to adjust the scale.
    if (latency_sum_ok(iolat, &stat) &&
    atomic_read(&lat_info.scale_cookie) == DEFAULT_SCALE_COOKIE) {
    return;
    }
// Somebody beat us to the punch, just bail.
    spin_lock_irqsave(&lat_info.lock, flags);
    latency_stat_sum(iolat, &iolat.cur_stat, &stat);
    lat_info.nr_samples -= iolat.nr_samples;
    lat_info.nr_samples += latency_stat_samples(iolat, &iolat.cur_stat);
    iolat.nr_samples = latency_stat_samples(iolat, &iolat.cur_stat);
    if ((lat_info.last_scale_event >= now ||
    now - lat_info.last_scale_event < BLKIOLATENCY_MIN_ADJUST_TIME)) {
// goto;
    }
    if (latency_sum_ok(iolat, &iolat.cur_stat) &&
    latency_sum_ok(iolat, &stat)) {
    if (latency_stat_samples(iolat, &iolat.cur_stat) <
    BLKIOLATENCY_MIN_GOOD_SAMPLES) {
// goto;
    }
    if (lat_info.scale_grp == iolat) {
    lat_info.last_scale_event = now;
    scale_cookie_change(iolat.blkiolat, lat_info, true);
    }
    } else if (lat_info.scale_lat == 0 ||
    lat_info.scale_lat >= iolat.min_lat_nsec) {
    lat_info.last_scale_event = now;
    if (!lat_info.scale_grp ||
    lat_info.scale_lat > iolat.min_lat_nsec) {
    WRITE_ONCE(lat_info.scale_lat, iolat.min_lat_nsec);
    lat_info.scale_grp = iolat;
    }
    scale_cookie_change(iolat.blkiolat, lat_info, false);
    }
    latency_stat_init(iolat, &iolat.cur_stat);
// label;
    spin_unlock_irqrestore(&lat_info.lock, flags);
    }
#[no_mangle]
unsafe extern "C" fn blkcg_iolatency_done_bio(rqos: *mut rq_qos, bio: *mut bio) {
pub static mut blkg: *mut c_void = core::ptr::null_mut();
pub static mut rqw: *mut c_void = core::ptr::null_mut();
pub static mut iolat: *mut c_void = core::ptr::null_mut();
    let mut window_start = 0;
    let mut now = 0;
pub static mut issue_as_root: bool = false;
pub static mut inflight: c_int = 0;
    blkg = bio.bi_blkg;
    if (!blkg || !bio_flagged(bio, BIO_QOS_THROTTLED)) {
    return;
    }
    iolat = blkg_to_lat(bio.bi_blkg);
    if (!iolat) {
    return;
    }
    if (!iolat.blkiolat.enabled) {
    return;
    }
    now = blk_time_get_ns();
    while (blkg && blkg.parent) {
    iolat = blkg_to_lat(blkg);
    if (!iolat) {
    blkg = blkg.parent;
    continue;
    }
    rqw = &iolat.rq_wait;
    inflight = atomic_dec_return(&rqw.inflight);
    WARN_ON_ONCE!(inflight < 0);
//
// If bi_status is BLK_STS_AGAIN, the bio wasn't actually
// submitted, so do not account for it.
//
    if (iolat.min_lat_nsec && bio.bi_status != BLK_STS_AGAIN) {
    iolatency_record_time(iolat, bio.issue_time_ns, now,
    issue_as_root);
    window_start = atomic64_read(&iolat.window_start);
    if (now > window_start &&
    (now - window_start) >= iolat.cur_win_nsec) {
    if (atomic64_try_cmpxchg(&iolat.window_start,
    &window_start, now)) {
    iolatency_check_latencies(iolat, now);
    }
    }
    }
    wake_up(&rqw.wait);
    blkg = blkg.parent;
    }
    }
#[no_mangle]
unsafe extern "C" fn blkcg_iolatency_exit(rqos: *mut rq_qos) {
    let mut blkiolat = BLKIOLATENCY(rqos);
    timer_shutdown_sync(&blkiolat.timer);
    flush_work(&blkiolat.enable_work);
    blkcg_deactivate_policy(rqos.disk, &blkcg_policy_iolatency);
    kfree(blkiolat);
    }
pub static mut rq_qos_ops: usize = 0;
#[no_mangle]
unsafe extern "C" fn blkiolatency_timer_fn(t: *mut timer_list) {
    let mut blkiolat = timer_container_of(blkiolat, t,
    timer);
pub static mut blkg: *mut c_void = core::ptr::null_mut();
pub static mut pos_css: *mut c_void = core::ptr::null_mut();
pub static mut now: u64 = 0;
    rcu_read_lock();
    blkg_for_each_descendant_pre(blkg, pos_css,
    blkiolat.rqos.disk.queue.root_blkg) {
pub static mut iolat: *mut c_void = core::ptr::null_mut();
pub static mut lat_info: *mut c_void = core::ptr::null_mut();
    let mut flags = 0;
    let mut cookie = 0;
//
// We could be exiting, don't access the pd unless we have a
// ref on the blkg.
//
    if (!blkg_tryget(blkg)) {
    continue;
    }
    iolat = blkg_to_lat(blkg);
    if (!iolat) {
// goto;
    }
    lat_info = &iolat.child_lat;
    cookie = atomic_read(&lat_info.scale_cookie);
    if (cookie >= DEFAULT_SCALE_COOKIE) {
// goto;
    }
    spin_lock_irqsave(&lat_info.lock, flags);
    if (lat_info.last_scale_event >= now) {
// goto;
    }
//
// We scaled down but don't have a scale_grp, scale up and carry
// on.
//
    if (lat_info.scale_grp == core::ptr::null_mut()) {
    scale_cookie_change(iolat.blkiolat, lat_info, true);
// goto;
    }
//
// It's been 5 seconds since our last scale event, clear the
// scale grp in case the group that needed the scale down isn't
// doing any IO currently.
//
    if (now - lat_info.last_scale_event >=
    ((u64)NSEC_PER_SEC * 5)) {
    lat_info.scale_grp = core::ptr::null_mut();
    }
// label;
    spin_unlock_irqrestore(&lat_info.lock, flags);
// label;
    blkg_put(blkg);
    }
    rcu_read_unlock();
    }
//
// blkiolatency_enable_work_fn - Enable or disable iolatency on the device
// @work: enable_work of the blk_iolatency of interest
//
// iolatency needs to keep track of the number of in-flight IOs per cgroup. This
// is relatively expensive as it involves walking up the hierarchy twice for
// every IO. Thus, if iolatency is not enabled in any cgroup for the device, we
// want to disable the in-flight tracking.
//
// We have to make sure that the counting is balanced - we don't want to leak
// the in-flight counts by disabling accounting in the completion path while IOs
// are in flight. This is achieved by ensuring that no IO is in flight by
// freezing the queue while flipping ->enabled. As this requires a sleepable
// context, ->enabled flipping is punted to this work function.
//
#[no_mangle]
unsafe extern "C" fn blkiolatency_enable_work_fn(work: *mut work_struct) {
    let mut blkiolat = container_of!(work, blk_iolatency,
    enable_work);
    let mut enabled = 0;
//
// There can only be one instance of this function running for @blkiolat
// and it's guaranteed to be executed at least once after the latest
// ->enabled_cnt modification. Acting on the latest ->enable_cnt is
// sufficient.
//
// Also, we know @blkiolat is safe to access as ->enable_work is flushed
// in blkcg_iolatency_exit().
//
    enabled = atomic_read(&blkiolat.enable_cnt);
    if (enabled != blkiolat.enabled) {
    let mut q = blkiolat.rqos.disk.queue;
    let mut memflags = 0;
    memflags = blk_mq_freeze_queue(blkiolat.rqos.disk.queue);
    blkiolat.enabled = enabled;
    if (enabled) {
    blk_queue_flag_set(QUEUE_FLAG_BIO_ISSUE_TIME, q);
    }
    else {
    blk_queue_flag_clear(QUEUE_FLAG_BIO_ISSUE_TIME, q);
    }
    blk_mq_unfreeze_queue(blkiolat.rqos.disk.queue, memflags);
    }
    }
#[no_mangle]
unsafe extern "C" fn blk_iolatency_init(disk: *mut gendisk) -> c_int {
pub static mut blkiolat: *mut c_void = core::ptr::null_mut();
    let mut ret = 0;
    blkiolat = kzalloc_obj(*blkiolat);
    if (!blkiolat) {
    return -ENOMEM;
    }
    ret = rq_qos_add(&blkiolat.rqos, disk, RQ_QOS_LATENCY,
    &blkcg_iolatency_ops);
    if (ret) {
// goto;
    }
    ret = blkcg_activate_policy(disk, &blkcg_policy_iolatency);
    if (ret) {
// goto;
    }
    timer_setup(&blkiolat.timer, blkiolatency_timer_fn, 0);
    INIT_WORK(&blkiolat.enable_work, blkiolatency_enable_work_fn);
    return 0;
// label;
    rq_qos_del(&blkiolat.rqos);
// label;
    kfree(blkiolat);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iolatency_set_min_lat_nsec(blkg: *mut blkcg_gq, val: u64) {
    let mut iolat = blkg_to_lat(blkg);
    let mut blkiolat = iolat.blkiolat;
pub static mut oldval: u64 = 0;
    iolat.min_lat_nsec = val;
    iolat.cur_win_nsec = max_t(u64, val << 4, BLKIOLATENCY_MIN_WIN_SIZE);
    iolat.cur_win_nsec = min_t(u64, iolat.cur_win_nsec,
    BLKIOLATENCY_MAX_WIN_SIZE);
    if (!oldval && val) {
    if (atomic_inc_return(&blkiolat.enable_cnt) == 1) {
    schedule_work(&blkiolat.enable_work);
    }
    }
    if (oldval && !val) {
    blkcg_clear_delay(blkg);
    if (atomic_dec_return(&blkiolat.enable_cnt) == 0) {
    schedule_work(&blkiolat.enable_work);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn iolatency_clear_scaling(blkg: *mut blkcg_gq) {
    if (blkg.parent) {
    let mut iolat = blkg_to_lat(blkg.parent);
pub static mut lat_info: *mut c_void = core::ptr::null_mut();
    if (!iolat) {
    return;
    }
    lat_info = &iolat.child_lat;
    spin_lock(&lat_info.lock);
    atomic_set(&lat_info.scale_cookie, DEFAULT_SCALE_COOKIE);
    lat_info.last_scale_event = 0;
    lat_info.scale_grp = core::ptr::null_mut();
    lat_info.scale_lat = 0;
    spin_unlock(&lat_info.lock);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn iolatency_set_limit(of: *mut kernfs_open_file, buf: *mut c_char, nbytes: size_t, off: loff_t) -> ssize_t {
    let mut blkcg = css_to_blkcg(of_css(of));
pub static mut blkg: *mut c_void = core::ptr::null_mut();
pub static mut ctx: usize = 0;
pub static mut iolat: *mut c_void = core::ptr::null_mut();
    let mut p = core::ptr::null_mut();
    let mut tok = core::ptr::null_mut();
pub static mut lat_val: u64 = 0;
    let mut oldval = 0;
    let mut ret = 0;
    blkg_conf_init(&ctx, buf);
    ret = blkg_conf_open_bdev(&ctx);
    if (ret) {
    return ret;
    }
//
// blk_iolatency_init() may fail after rq_qos_add() succeeds which can
// confuse iolat_rq_qos() test. Make the test and init atomic.
//
    lockdep_assert_held(&ctx.bdev.bd_queue.rq_qos_mutex);
    if (!iolat_rq_qos(ctx.bdev.bd_queue)) {
    ret = blk_iolatency_init(ctx.bdev.bd_disk);
    }
    if (ret) {
// goto;
    }
    ret = blkg_conf_prep(blkcg, &blkcg_policy_iolatency, &ctx);
    if (ret) {
// goto;
    }
    iolat = blkg_to_lat(ctx.blkg);
    p = ctx.body;
    ret = -EINVAL;
    while ((tok = strsep(&p, " "))) {
    char key[16];
    char val[21];	/* 18446744073709551616 */
    if (sscanf(tok, "%15[^=]=%20s", key, val) != 2) {
// goto;
    }
    if (!strcmp(key, "target")) {
    let mut v = 0;
    if (!strcmp(val, "max")) {
    lat_val = 0;
    }

    else if (sscanf(val, "%llu", &v) == 1) {
    lat_val = v * NSEC_PER_USEC;
    }
    else {
// goto;
    }
    } else {
// goto;
    }
    }
// Walk up the tree to see if our new val is lower than it should be.
    blkg = ctx.blkg;
    oldval = iolat.min_lat_nsec;
    iolatency_set_min_lat_nsec(blkg, lat_val);
    if (oldval != iolat.min_lat_nsec) {
    iolatency_clear_scaling(blkg);
    }
    ret = 0;
// label;
    blkg_conf_unprep(&ctx);
// label;
    blkg_conf_close_bdev(&ctx);
    return ret ?: nbytes;
    }
#[no_mangle]
pub unsafe extern "C" fn iolatency_prfill_limit(sf: *mut seq_file, pd: *mut blkg_policy_data, off: c_int) -> u64 {
    let mut iolat = pd_to_lat(pd);
    let mut dname = blkg_dev_name(pd.blkg);
    if (!dname || !iolat.min_lat_nsec) {
    return 0;
    }
    seq_printf(sf, "%s target=%llu\n",
    dname, div_u64(iolat.min_lat_nsec, NSEC_PER_USEC));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iolatency_print_limit(sf: *mut seq_file, v: *mut c_void) -> c_int {
    blkcg_print_blkgs(sf, css_to_blkcg(seq_css(sf)),
    iolatency_prfill_limit,
    &blkcg_policy_iolatency, seq_cft(sf).private, false);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iolatency_ssd_stat(iolat: *mut iolatency_grp, s: *mut seq_file) {
pub static mut stat: usize = 0;
    let mut cpu = 0;
    latency_stat_init(iolat, &stat);
    preempt_disable();
    for_each_possible_cpu(cpu) {
pub static mut s: *mut c_void = core::ptr::null_mut();
    s = per_cpu_ptr(iolat.stats, cpu);
    latency_stat_sum(iolat, &stat, s);
    }
    preempt_enable();
    if (iolat.max_depth == UINT_MAX) {
    seq_printf(s, " missed=%llu total=%llu depth=max",
    (unsigned long long)stat.ps.missed,
    (unsigned long long)stat.ps.total);
    }
    else {
    seq_printf(s, " missed=%llu total=%llu depth=%u",
    (unsigned long long)stat.ps.missed,
    (unsigned long long)stat.ps.total,
    iolat.max_depth);
    }
    }
#[no_mangle]
unsafe extern "C" fn iolatency_pd_stat(pd: *mut blkg_policy_data, s: *mut seq_file) {
    let mut iolat = pd_to_lat(pd);
    unsigned long long avg_lat;
    unsigned long long cur_win;
    if (!blkcg_debug_stats) {
    return;
    }
    if (iolat.ssd) {
    return iolatency_ssd_stat(iolat, s);
    }
    avg_lat = div64_u64(iolat.lat_avg, NSEC_PER_USEC);
    cur_win = div64_u64(iolat.cur_win_nsec, NSEC_PER_MSEC);
    if (iolat.max_depth == UINT_MAX) {
    seq_printf(s, " depth=max avg_lat=%llu win=%llu",
    avg_lat, cur_win);
    }
    else {
    seq_printf(s, " depth=%u avg_lat=%llu win=%llu",
    iolat.max_depth, avg_lat, cur_win);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn iolatency_pd_alloc(disk: *mut gendisk, blkcg: *mut blkcg, gfp: gfp_t) -> *mut c_void {
pub static mut iolat: *mut c_void = core::ptr::null_mut();
    iolat = kzalloc_node(sizeof!(*iolat), gfp, disk.node_id);
    if (!iolat) {
    return core::ptr::null_mut();
    }
    iolat.stats = __alloc_percpu_gfp(sizeof!(latency_stat),
    __alignof__(latency_stat), gfp);
    if (!iolat.stats) {
    kfree(iolat);
    return core::ptr::null_mut();
    }
    return &iolat.pd;
    }
#[no_mangle]
unsafe extern "C" fn iolatency_pd_init(pd: *mut blkg_policy_data) {
    let mut iolat = pd_to_lat(pd);
    let mut blkg = lat_to_blkg(iolat);
    let mut rqos = iolat_rq_qos(blkg.q);
    let mut blkiolat = BLKIOLATENCY(rqos);
pub static mut now: u64 = 0;
    let mut cpu = 0;
    iolat.ssd = !blk_queue_rot(blkg.q);
    for_each_possible_cpu(cpu) {
pub static mut stat: *mut c_void = core::ptr::null_mut();
    stat = per_cpu_ptr(iolat.stats, cpu);
    latency_stat_init(iolat, stat);
    }
    latency_stat_init(iolat, &iolat.cur_stat);
    rq_wait_init(&iolat.rq_wait);
    spin_lock_init(&iolat.child_lat.lock);
    iolat.max_depth = UINT_MAX;
    iolat.blkiolat = blkiolat;
    iolat.cur_win_nsec = 100 * NSEC_PER_MSEC;
    atomic64_set(&iolat.window_start, now);
//
// We init things in list order, so the pd for the parent may not be
// init'ed yet for whatever reason.
//
    if (blkg.parent && blkg_to_pd(blkg.parent, &blkcg_policy_iolatency)) {
    let mut parent = blkg_to_lat(blkg.parent);
    atomic_set(&iolat.scale_cookie,
    atomic_read(&parent.child_lat.scale_cookie));
    } else {
    atomic_set(&iolat.scale_cookie, DEFAULT_SCALE_COOKIE);
    }
    atomic_set(&iolat.child_lat.scale_cookie, DEFAULT_SCALE_COOKIE);
    }
#[no_mangle]
unsafe extern "C" fn iolatency_pd_offline(pd: *mut blkg_policy_data) {
    let mut iolat = pd_to_lat(pd);
    let mut blkg = lat_to_blkg(iolat);
    iolatency_set_min_lat_nsec(blkg, 0);
    iolatency_clear_scaling(blkg);
    }
#[no_mangle]
unsafe extern "C" fn iolat_release(rcu: *mut rcu_head) {
    let mut pd = container_of!(rcu, blkg_policy_data, rcu_head);
    let mut iolat = pd_to_lat(pd);
    free_percpu(iolat.stats);
    kfree(iolat);
    }
#[no_mangle]
unsafe extern "C" fn iolatency_pd_free(pd: *mut blkg_policy_data) {
    let mut blkg = pd_to_blkg(pd);
//
// Groups throttled as collateral have min_lat_nsec == 0, so
// iolatency_pd_offline() leaves their delay set.  Drop it here, where
// no in-flight bio can re-arm it via check_scale_change().
//
    if (blkg) {
    blkcg_clear_delay(blkg);
    }
    call_rcu(&pd.rcu_head, iolat_release);
    }
pub static mut cftype: usize = 0;
pub static mut blkcg_policy: usize = 0;
#[no_mangle]
unsafe extern "C" fn iolatency_init() -> c_int {
    return blkcg_policy_register(&blkcg_policy_iolatency);
    }
#[no_mangle]
unsafe extern "C" fn iolatency_exit()  {
    blkcg_policy_unregister(&blkcg_policy_iolatency);
    }
    module_init!(iolatency_init);
    module_exit!(iolatency_exit);