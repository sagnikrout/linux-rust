//! Automatically rewritten from C to Rust
//! Source: block/kyber-iosched.c
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
// The Kyber I/O scheduler. Controls latency by throttling queue depths using
// scalable techniques.
//
// Copyright (C) 2017 Facebook
//

// Macro flag: #define CREATE_TRACE_POINTS

//
// Scheduling domains: the device is divided into multiple domains based on the
// request type.
//
    enum {
    KYBER_READ,
    KYBER_WRITE,
    KYBER_DISCARD,
    KYBER_OTHER,
    KYBER_NUM_DOMAINS,
    };
    static const char *kyber_domain_names[] = {
    [KYBER_READ] = "READ",
    [KYBER_WRITE] = "WRITE",
    [KYBER_DISCARD] = "DISCARD",
    [KYBER_OTHER] = "OTHER",
    };
    enum {
//
// In order to prevent starvation of synchronous requests by a flood of
// asynchronous requests, we reserve 25% of requests for synchronous
// operations.
//
    KYBER_DEFAULT_ASYNC_PERCENT = 75,
    };
//
// Maximum device-wide depth for each scheduling domain.
//
// Even for fast devices with lots of tags like NVMe, you can saturate the
// device with only a fraction of the maximum possible queue depth. So, we cap
// these to a reasonable value.
//
    static const unsigned int kyber_depth[] = {
    [KYBER_READ] = 256,
    [KYBER_WRITE] = 128,
    [KYBER_DISCARD] = 64,
    [KYBER_OTHER] = 16,
    };
//
// Default latency targets for each scheduling domain.
//
    static const u64 kyber_latency_targets[] = {
    [KYBER_READ] = 2ULL * NSEC_PER_MSEC,
    [KYBER_WRITE] = 10ULL * NSEC_PER_MSEC,
    [KYBER_DISCARD] = 5ULL * NSEC_PER_SEC,
    };
//
// Batch size (number of requests we'll dispatch in a row) for each scheduling
// domain.
//
    static const unsigned int kyber_batch_size[] = {
    [KYBER_READ] = 16,
    [KYBER_WRITE] = 8,
    [KYBER_DISCARD] = 1,
    [KYBER_OTHER] = 1,
    };
//
// Requests latencies are recorded in a histogram with buckets defined relative
// to the target latency:
//
// <= 1/4 * target latency
// <= 1/2 * target latency
// <= 3/4 * target latency
// <= target latency
// <= 1 1/4 * target latency
// <= 1 1/2 * target latency
// <= 1 3/4 * target latency
// > 1 3/4 * target latency
//
    enum {
//
// The width of the latency histogram buckets is
// 1 / (1 << KYBER_LATENCY_SHIFT) * target latency.
//
    KYBER_LATENCY_SHIFT = 2,
//
// The first (1 << KYBER_LATENCY_SHIFT) buckets are <= target latency,
// thus, "good".
//
    KYBER_GOOD_BUCKETS = 1 << KYBER_LATENCY_SHIFT,
// There are also (1 << KYBER_LATENCY_SHIFT) "bad" buckets.
    KYBER_LATENCY_BUCKETS = 2 << KYBER_LATENCY_SHIFT,
    };
//
// We measure both the total latency and the I/O latency (i.e., latency after
// submitting to the device).
//
    enum {
    KYBER_TOTAL_LATENCY,
    KYBER_IO_LATENCY,
    };
    static const char *kyber_latency_type_names[] = {
    [KYBER_TOTAL_LATENCY] = "total",
    [KYBER_IO_LATENCY] = "I/O",
    };
//
// Per-cpu latency histograms: total latency and I/O latency for each scheduling
// domain except for KYBER_OTHER.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kyber_cpu_latency {
    pub buckets: [core::sync::atomic::AtomicI32; KYBER_OTHER][2][KYBER_LATENCY_BUCKETS],
}

//
// There is a same mapping between ctx & hctx and kcq & khd,
// we use request->mq_ctx->index_hw to index the kcq in khd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kyber_ctx_queue {
//
// Used to ensure operations on rq_list and kcq_map to be an atmoic one.
// Also protect the rqs on rq_list when merge.
//
    pub lock: spinlock_t,
    pub rq_list: [list_head; KYBER_NUM_DOMAINS],
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kyber_queue_data {
    pub q: *mut request_queue,
    pub dev: dev_t,
//
// Each scheduling domain has a limited number of in-flight requests
// device-wide, limited by these tokens.
//
    pub domain_tokens: [sbitmap_queue; KYBER_NUM_DOMAINS],
    pub cpu_latency: *mut kyber_cpu_latency ,
// Timer for stats aggregation and adjusting domain tokens.
    pub timer: timer_list,
    pub latency_buckets: [c_uint; KYBER_OTHER][2][KYBER_LATENCY_BUCKETS],
    pub latency_timeout: [c_ulong; KYBER_OTHER],
    pub domain_p99: [c_int; KYBER_OTHER],
// Target latencies in nanoseconds.
    pub latency_targets: [u64; KYBER_OTHER],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kyber_hctx_data {
    pub lock: spinlock_t,
    pub rqs: [list_head; KYBER_NUM_DOMAINS],
    pub cur_domain: c_uint,
    pub batching: c_uint,
    pub kcqs: *mut kyber_ctx_queue,
    pub kcq_map: [sbitmap; KYBER_NUM_DOMAINS],
    pub domain_wait: [sbq_wait; KYBER_NUM_DOMAINS],
    pub domain_ws: [*mut sbq_wait_state; KYBER_NUM_DOMAINS],
    pub wait_index: [core::sync::atomic::AtomicI32; KYBER_NUM_DOMAINS],
}

// forward_decl: kyber_domain_wake;
#[no_mangle]
unsafe extern "C" fn kyber_sched_domain(opf: blk_opf_t) -> c_uint {
    match (opf & REQ_OP_MASK) {
    REQ_OP_READ => {
    return KYBER_READ;
    }
    REQ_OP_WRITE => {
    return KYBER_WRITE;
    }
    REQ_OP_DISCARD => {
    return KYBER_DISCARD;
    }
    _ => {
    return KYBER_OTHER;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn flush_latency_buckets(kqd: *mut kyber_queue_data, cpu_latency: *mut kyber_cpu_latency, sched_domain: c_uint, type: c_uint) {
    let mut buckets = kqd.latency_buckets[sched_domain][type];
    let mut cpu_buckets = cpu_latency.buckets[sched_domain][type];
    let mut bucket = 0;
    for (bucket = 0; bucket < KYBER_LATENCY_BUCKETS; bucket++) {
    buckets[bucket] += atomic_xchg(&cpu_buckets[bucket], 0);
    }
    }
//
// Calculate the histogram bucket with the given percentile rank, or -1 if there
// aren't enough samples yet.
//
#[no_mangle]
pub unsafe extern "C" fn calculate_percentile(kqd: *mut kyber_queue_data, sched_domain: c_uint, type: c_uint, percentile: c_uint) -> c_int {
    let mut buckets = kqd.latency_buckets[sched_domain][type];
    unsigned int bucket, samples = 0, percentile_samples;
    for (bucket = 0; bucket < KYBER_LATENCY_BUCKETS; bucket++) {
    samples += buckets[bucket];
    }
    if (!samples) {
    return -1;
    }
//
// We do the calculation once we have 500 samples or one second passes
// since the first sample was recorded, whichever comes first.
//
    if (!kqd.latency_timeout[sched_domain]) {
    kqd.latency_timeout[sched_domain] = max(jiffies + HZ, 1UL);
    }
    if (samples < 500 &&
    time_is_after_jiffies(kqd.latency_timeout[sched_domain])) {
    return -1;
    }
    kqd.latency_timeout[sched_domain] = 0;
    percentile_samples = DIV_ROUND_UP(samples * percentile, 100);
    while (bucket < KYBER_LATENCY_BUCKETS - 1) {
    if (buckets[bucket] >= percentile_samples) {
    break;
    }
    percentile_samples -= buckets[bucket];
    }
    memset(buckets, 0, sizeof!(kqd.latency_buckets[sched_domain][type]));
    trace_kyber_latency(kqd.dev, kyber_domain_names[sched_domain],
    kyber_latency_type_names[type], percentile,
    bucket + 1, 1 << KYBER_LATENCY_SHIFT, samples);
    return bucket;
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_resize_domain(kqd: *mut kyber_queue_data, sched_domain: c_uint, depth: c_uint) {
    depth = clamp(depth, 1U, kyber_depth[sched_domain]);
    if (depth != kqd.domain_tokens[sched_domain].sb.depth) {
    sbitmap_queue_resize(&kqd.domain_tokens[sched_domain], depth);
    trace_kyber_adjust(kqd.dev, kyber_domain_names[sched_domain],
    depth);
    }
    }
#[no_mangle]
unsafe extern "C" fn kyber_timer_fn(t: *mut timer_list) {
    let mut kqd = timer_container_of(kqd, t, timer);
    let mut sched_domain = 0;
    let mut cpu = 0;
pub static mut bad: bool = false;
// Sum all of the per-cpu latency histograms.
    for_each_possible_cpu(cpu) {
pub static mut cpu_latency: *mut c_void = core::ptr::null_mut();
    cpu_latency = per_cpu_ptr(kqd.cpu_latency, cpu);
    while (sched_domain < KYBER_OTHER) {
    flush_latency_buckets(kqd, cpu_latency, sched_domain,
    KYBER_TOTAL_LATENCY);
    flush_latency_buckets(kqd, cpu_latency, sched_domain,
    KYBER_IO_LATENCY);
    }
    }
//
// Check if any domains have a high I/O latency, which might indicate
// congestion in the device. Note that we use the p90; we don't want to
// be too sensitive to outliers here.
//
    while (sched_domain < KYBER_OTHER) {
    let mut p90 = 0;
    p90 = calculate_percentile(kqd, sched_domain, KYBER_IO_LATENCY,
    90);
    if (p90 >= KYBER_GOOD_BUCKETS) {
    bad = true;
    }
    }
//
// Adjust the scheduling domain depths. If we determined that there was
// congestion, we throttle all domains with good latencies. Either way,
// we ease up on throttling domains with bad latencies.
//
    while (sched_domain < KYBER_OTHER) {
    let mut orig_depth = 0;
    let mut depth = 0;
    let mut p99 = 0;
    p99 = calculate_percentile(kqd, sched_domain,
    KYBER_TOTAL_LATENCY, 99);
//
// This is kind of subtle: different domains will not
// necessarily have enough samples to calculate the latency
// percentiles during the same window, so we have to remember
// the p99 for the next time we observe congestion; once we do,
// we don't want to throttle again until we get more data, so we
// reset it to -1.
//
    if (bad) {
    if (p99 < 0) {
    p99 = kqd.domain_p99[sched_domain];
    }
    kqd.domain_p99[sched_domain] = -1;
    } else if (p99 >= 0) {
    kqd.domain_p99[sched_domain] = p99;
    }
    if (p99 < 0) {
    continue;
    }
//
// If this domain has bad latency, throttle less. Otherwise,
// throttle more iff we determined that there is congestion.
//
// The new depth is scaled linearly with the p99 latency vs the
// latency target. E.g., if the p99 is 3/4 of the target, then
// we throttle down to 3/4 of the current depth, and if the p99
// is 2x the target, then we double the depth.
//
    if (bad || p99 >= KYBER_GOOD_BUCKETS) {
    orig_depth = kqd.domain_tokens[sched_domain].sb.depth;
    depth = (orig_depth * (p99 + 1)) >> KYBER_LATENCY_SHIFT;
    kyber_resize_domain(kqd, sched_domain, depth);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_queue_data_alloc(q: *mut request_queue) -> *mut c_void {
pub static mut kqd: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    let mut i = 0;
    kqd = kzalloc_node(sizeof!(*kqd), GFP_KERNEL, q.node);
    if (!kqd) {
// goto;
    }
    kqd.q = q;
    kqd.dev = disk_devt(q.disk);
    kqd.cpu_latency = alloc_percpu_gfp(kyber_cpu_latency,
    GFP_KERNEL | __GFP_ZERO);
    if (!kqd.cpu_latency) {
// goto;
    }
    timer_setup(&kqd.timer, kyber_timer_fn, 0);
    while (i < KYBER_NUM_DOMAINS) {
    WARN_ON!(!kyber_depth[i]);
    WARN_ON!(!kyber_batch_size[i]);
    ret = sbitmap_queue_init_node(&kqd.domain_tokens[i],
    kyber_depth[i], -1, false,
    GFP_KERNEL, q.node);
    if (ret) {
    while (--i >= 0) {
    sbitmap_queue_free(&kqd.domain_tokens[i]);
    }
// goto;
    }
    }
    while (i < KYBER_OTHER) {
    kqd.domain_p99[i] = -1;
    kqd.latency_targets[i] = kyber_latency_targets[i];
    }
    return kqd;
// label;
    free_percpu(kqd.cpu_latency);
// label;
    kfree(kqd);
// label;
    return ERR_PTR(ret);
    }
#[no_mangle]
unsafe extern "C" fn kyber_depth_updated(q: *mut request_queue) {
    blk_mq_set_min_shallow_depth(q, q.async_depth);
    }
#[no_mangle]
unsafe extern "C" fn kyber_init_sched(q: *mut request_queue, eq: *mut elevator_queue) -> c_int {
    blk_stat_enable_accounting(q);
    blk_queue_flag_clear(QUEUE_FLAG_SQ_SCHED, q);
    q.elevator = eq;
    q.async_depth = q.nr_requests * KYBER_DEFAULT_ASYNC_PERCENT / 100;
    kyber_depth_updated(q);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_alloc_sched_data(q: *mut request_queue) -> *mut c_void {
pub static mut kqd: *mut c_void = core::ptr::null_mut();
    kqd = kyber_queue_data_alloc(q);
    if (IS_ERR(kqd)) {
    return core::ptr::null_mut();
    }
    return kqd;
    }
#[no_mangle]
unsafe extern "C" fn kyber_exit_sched(e: *mut elevator_queue) {
    let mut kqd = e.elevator_data;
    timer_shutdown_sync(&kqd.timer);
    blk_stat_disable_accounting(kqd.q);
    }
#[no_mangle]
unsafe extern "C" fn kyber_free_sched_data(elv_data: *mut c_void) {
    let mut kqd = elv_data;
    let mut i = 0;
    if (!kqd) {
    return;
    }
    for (i = 0; i < KYBER_NUM_DOMAINS; i++) {
    sbitmap_queue_free(&kqd.domain_tokens[i]);
    }
    free_percpu(kqd.cpu_latency);
    kfree(kqd);
    }
#[no_mangle]
unsafe extern "C" fn kyber_ctx_queue_init(kcq: *mut kyber_ctx_queue) {
    let mut i = 0;
    spin_lock_init(&kcq.lock);
    for (i = 0; i < KYBER_NUM_DOMAINS; i++) {
    INIT_LIST_HEAD(&kcq.rq_list[i]);
    }
    }
#[no_mangle]
unsafe extern "C" fn kyber_init_hctx(hctx: *mut blk_mq_hw_ctx, hctx_idx: c_uint) -> c_int {
pub static mut khd: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    khd = kmalloc_node(sizeof!(*khd), GFP_KERNEL, hctx.numa_node);
    if (!khd) {
    return -ENOMEM;
    }
    khd.kcqs = kmalloc_array_node(hctx.nr_ctx,
    sizeof!(kyber_ctx_queue),
    GFP_KERNEL, hctx.numa_node);
    if (!khd.kcqs) {
// goto;
    }
    for (i = 0; i < hctx.nr_ctx; i++) {
    kyber_ctx_queue_init(&khd.kcqs[i]);
    }
    while (i < KYBER_NUM_DOMAINS) {
    if (sbitmap_init_node(&khd.kcq_map[i], hctx.nr_ctx,
    ilog2(8), GFP_KERNEL, hctx.numa_node,
    false, false)) {
    while (--i >= 0) {
    sbitmap_free(&khd.kcq_map[i]);
    }
// goto;
    }
    }
    spin_lock_init(&khd.lock);
    while (i < KYBER_NUM_DOMAINS) {
    INIT_LIST_HEAD(&khd.rqs[i]);
    khd.domain_wait[i].sbq = core::ptr::null_mut();
    init_waitqueue_func_entry(&khd.domain_wait[i].wait,
    kyber_domain_wake);
    khd.domain_wait[i].wait.private = hctx;
    INIT_LIST_HEAD(&khd.domain_wait[i].wait.entry);
    atomic_set(&khd.wait_index[i], 0);
    }
    khd.cur_domain = 0;
    khd.batching = 0;
    hctx.sched_data = khd;
    return 0;
// label;
    kfree(khd.kcqs);
// label;
    kfree(khd);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn kyber_exit_hctx(hctx: *mut blk_mq_hw_ctx, hctx_idx: c_uint) {
    let mut khd = hctx.sched_data;
    let mut i = 0;
    for (i = 0; i < KYBER_NUM_DOMAINS; i++) {
    sbitmap_free(&khd.kcq_map[i]);
    }
    kfree(khd.kcqs);
    kfree(hctx.sched_data);
    }
#[no_mangle]
unsafe extern "C" fn rq_get_domain_token(rq: *mut request) -> c_int {
    return (long)rq.elv.priv[0];
    }
#[no_mangle]
unsafe extern "C" fn rq_set_domain_token(rq: *mut request, token: c_int) {
    rq.elv.priv[0] = (long)token;
    }
#[no_mangle]
pub unsafe extern "C" fn rq_clear_domain_token(kqd: *mut kyber_queue_data, rq: *mut request) {
    let mut sched_domain = 0;
    let mut nr = 0;
    nr = rq_get_domain_token(rq);
    if (nr != -1) {
    sched_domain = kyber_sched_domain(rq.cmd_flags);
    sbitmap_queue_clear(&kqd.domain_tokens[sched_domain], nr,
    rq.mq_ctx.cpu);
    }
    }
#[no_mangle]
unsafe extern "C" fn kyber_limit_depth(opf: blk_opf_t, data: *mut blk_mq_alloc_data) {
    if (!blk_mq_is_sync_read(opf)) {
    data.shallow_depth = data.q.async_depth;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_bio_merge(q: *mut request_queue, bio: *mut bio, nr_segs: c_uint) -> bool {
    let mut ctx = blk_mq_get_ctx(q);
    let mut hctx = blk_mq_map_queue(bio.bi_opf, ctx);
    let mut khd = hctx.sched_data;
    let mut kcq = &khd.kcqs[ctx.index_hw[hctx.type]];
pub static mut sched_domain: c_uint = 0;
    let mut rq_list = &kcq.rq_list[sched_domain];
    let mut merged = 0;
    spin_lock(&kcq.lock);
    merged = blk_bio_list_merge(hctx.queue, rq_list, bio, nr_segs);
    spin_unlock(&kcq.lock);
    return merged;
    }
#[no_mangle]
unsafe extern "C" fn kyber_prepare_request(rq: *mut request) {
    rq_set_domain_token(rq, -1);
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_insert_requests(hctx: *mut blk_mq_hw_ctx, rq_list: *mut list_head, flags: blk_insert_t) {
    let mut khd = hctx.sched_data;
    let mut rq = core::ptr::null_mut();
    let mut next = core::ptr::null_mut();
    list_for_each_entry_safe(rq, next, rq_list, queuelist) {
pub static mut sched_domain: c_uint = 0;
    let mut kcq = &khd.kcqs[rq.mq_ctx.index_hw[hctx.type]];
    let mut head = &kcq.rq_list[sched_domain];
    spin_lock(&kcq.lock);
    trace_block_rq_insert(rq);
    if (flags & BLK_MQ_INSERT_AT_HEAD) {
    list_move(&rq.queuelist, head);
    }
    else {
    list_move_tail(&rq.queuelist, head);
    }
    sbitmap_set_bit(&khd.kcq_map[sched_domain],
    rq.mq_ctx.index_hw[hctx.type]);
    spin_unlock(&kcq.lock);
    }
    }
#[no_mangle]
unsafe extern "C" fn kyber_finish_request(rq: *mut request) {
    let mut kqd = rq.q.elevator.elevator_data;
    rq_clear_domain_token(kqd, rq);
    }
#[no_mangle]
pub unsafe extern "C" fn add_latency_sample(cpu_latency: *mut kyber_cpu_latency, sched_domain: c_uint, type: c_uint, target: u64, latency: u64) {
    let mut bucket = 0;
    let mut divisor = 0;
    if (latency > 0) {
    divisor = max_t(u64, target >> KYBER_LATENCY_SHIFT, 1);
    bucket = min_t(unsigned int, div64_u64(latency - 1, divisor),
    KYBER_LATENCY_BUCKETS - 1);
    } else {
    bucket = 0;
    }
    atomic_inc(&cpu_latency.buckets[sched_domain][type][bucket]);
    }
#[no_mangle]
unsafe extern "C" fn kyber_completed_request(rq: *mut request, now: u64) {
    let mut kqd = rq.q.elevator.elevator_data;
pub static mut cpu_latency: *mut c_void = core::ptr::null_mut();
    let mut sched_domain = 0;
    let mut target = 0;
    sched_domain = kyber_sched_domain(rq.cmd_flags);
    if (sched_domain == KYBER_OTHER) {
    return;
    }
    cpu_latency = get_cpu_ptr(kqd.cpu_latency);
    target = kqd.latency_targets[sched_domain];
    add_latency_sample(cpu_latency, sched_domain, KYBER_TOTAL_LATENCY,
    target, now - rq.start_time_ns);
    add_latency_sample(cpu_latency, sched_domain, KYBER_IO_LATENCY, target,
    now - rq.io_start_time_ns);
    put_cpu_ptr(kqd.cpu_latency);
    timer_reduce(&kqd.timer, jiffies + HZ / 10);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct flush_kcq_data {
    pub khd: *mut kyber_hctx_data,
    pub sched_domain: c_uint,
    pub list: *mut list_head,
}

#[no_mangle]
unsafe extern "C" fn flush_busy_kcq(sb: *mut sbitmap, bitnr: c_uint, data: *mut c_void) -> bool {
    let mut flush_data = data;
    let mut kcq = &flush_data.khd.kcqs[bitnr];
    spin_lock(&kcq.lock);
    list_splice_tail_init(&kcq.rq_list[flush_data.sched_domain],
    flush_data.list);
    sbitmap_clear_bit(sb, bitnr);
    spin_unlock(&kcq.lock);
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_flush_busy_kcqs(khd: *mut kyber_hctx_data, sched_domain: c_uint, list: *mut list_head) {
pub static mut flush_kcq_data: usize = 0;
    sbitmap_for_each_set(&khd.kcq_map[sched_domain],
    flush_busy_kcq, &data);
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_domain_wake(wqe: *mut wait_queue_entry_t, mode: c_uint, flags: c_int, key: *mut c_void) -> c_int {
    let mut hctx = READ_ONCE(wqe.private);
    let mut wait = container_of!(wqe, sbq_wait, wait);
    sbitmap_del_wait_queue(wait);
    blk_mq_run_hw_queue(hctx, true);
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_get_domain_token(kqd: *mut kyber_queue_data, khd: *mut kyber_hctx_data, hctx: *mut blk_mq_hw_ctx) -> c_int {
pub static mut sched_domain: c_uint = 0;
    let mut domain_tokens = &kqd.domain_tokens[sched_domain];
    let mut wait = &khd.domain_wait[sched_domain];
pub static mut ws: *mut c_void = core::ptr::null_mut();
    let mut nr = 0;
    nr = __sbitmap_queue_get(domain_tokens);
//
// If we failed to get a domain token, make sure the hardware queue is
// run when one becomes available. Note that this is serialized on
// khd->lock, but we still need to be careful about the waker.
//
    if (nr < 0 && list_empty_careful(&wait.wait.entry)) {
    ws = sbq_wait_ptr(domain_tokens,
    &khd.wait_index[sched_domain]);
    khd.domain_ws[sched_domain] = ws;
    sbitmap_add_wait_queue(domain_tokens, ws, wait);
//
// Try again in case a token was freed before we got on the wait
// queue.
//
    nr = __sbitmap_queue_get(domain_tokens);
    }
//
// If we got a token while we were on the wait queue, remove ourselves
// from the wait queue to ensure that all wake ups make forward
// progress. It's possible that the waker already deleted the entry
// between the !list_empty_careful() check and us grabbing the lock, but
// list_del_init() is okay with that.
//
    if (nr >= 0 && !list_empty_careful(&wait.wait.entry)) {
    ws = khd.domain_ws[sched_domain];
    spin_lock_irq(&ws.wait.lock);
    sbitmap_del_wait_queue(wait);
    spin_unlock_irq(&ws.wait.lock);
    }
    return nr;
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_dispatch_cur_domain(kqd: *mut kyber_queue_data, khd: *mut kyber_hctx_data, hctx: *mut blk_mq_hw_ctx) -> *mut c_void {
pub static mut rqs: *mut c_void = core::ptr::null_mut();
pub static mut rq: *mut c_void = core::ptr::null_mut();
    let mut nr = 0;
    rqs = &khd.rqs[khd.cur_domain];
//
// If we already have a flushed request, then we just need to get a
// token for it. Otherwise, if there are pending requests in the kcqs,
// flush the kcqs, but only if we can get a token. If not, we should
// leave the requests in the kcqs so that they can be merged. Note that
// khd->lock serializes the flushes, so if we observed any bit set in
// the kcq_map, we will always get a request.
//
    rq = list_first_entry_or_null(rqs, request, queuelist);
    if (rq) {
    nr = kyber_get_domain_token(kqd, khd, hctx);
    if (nr >= 0) {
    khd.batching += 1;
    rq_set_domain_token(rq, nr);
    list_del_init(&rq.queuelist);
    return rq;
    } else {
    trace_kyber_throttled(kqd.dev,
    kyber_domain_names[khd.cur_domain]);
    }
    } else if (sbitmap_any_bit_set(&khd.kcq_map[khd.cur_domain])) {
    nr = kyber_get_domain_token(kqd, khd, hctx);
    if (nr >= 0) {
    kyber_flush_busy_kcqs(khd, khd.cur_domain, rqs);
    rq = list_first_entry(rqs, request, queuelist);
    khd.batching += 1;
    rq_set_domain_token(rq, nr);
    list_del_init(&rq.queuelist);
    return rq;
    } else {
    trace_kyber_throttled(kqd.dev,
    kyber_domain_names[khd.cur_domain]);
    }
    }
// There were either no pending requests or no tokens.
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn kyber_dispatch_request(hctx: *mut blk_mq_hw_ctx) -> *mut c_void {
    let mut kqd = hctx.queue.elevator.elevator_data;
    let mut khd = hctx.sched_data;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    let mut i = 0;
    spin_lock(&khd.lock);
//
// First, if we are still entitled to batch, try to dispatch a request
// from the batch.
//
    if (khd.batching < kyber_batch_size[khd.cur_domain]) {
    rq = kyber_dispatch_cur_domain(kqd, khd, hctx);
    if (rq) {
// goto;
    }
    }
//
// Either,
// 1. We were no longer entitled to a batch.
// 2. The domain we were batching didn't have any requests.
// 3. The domain we were batching was out of tokens.
//
// Start another batch. Note that this wraps back around to the original
// domain if no other domains have requests or tokens.
//
    khd.batching = 0;
    while (i < KYBER_NUM_DOMAINS) {
    if (khd.cur_domain == KYBER_NUM_DOMAINS - 1) {
    khd.cur_domain = 0;
    }
    else {
    khd.cur_domain += 1;
    }
    rq = kyber_dispatch_cur_domain(kqd, khd, hctx);
    if (rq) {
// goto;
    }
    }
    rq = core::ptr::null_mut();
// label;
    spin_unlock(&khd.lock);
    return rq;
    }
#[no_mangle]
unsafe extern "C" fn kyber_has_work(hctx: *mut blk_mq_hw_ctx) -> bool {
    let mut khd = hctx.sched_data;
    let mut i = 0;
    while (i < KYBER_NUM_DOMAINS) {
    if (!list_empty_careful(&khd.rqs[i]) ||
    sbitmap_any_bit_set(&khd.kcq_map[i])) {
    return true;
    }
    }
    return false;
    }

    static ssize_t kyber_##name##_lat_show(elevator_queue *e,	
    char *page)			
    {									
    let mut kqd = e.elevator_data;		
    
    return sprintf(page, "%llu\n", kqd.latency_targets[domain]);	
    }									
    
    static ssize_t kyber_##name##_lat_store(elevator_queue *e,	
    const char *page, size_t count)	
    {									
    let mut kqd = e.elevator_data;		
    unsigned long long nsec;					
    let mut ret = 0;							
    
    ret = kstrtoull(page, 10, &nsec);				
    if (ret)							 {
    return ret;						
    }
    
    kqd.latency_targets[domain] = nsec;				
    
    return count;							
    }
    KYBER_LAT_SHOW_STORE(KYBER_READ, read);
    KYBER_LAT_SHOW_STORE(KYBER_WRITE, write);

pub static mut elv_fs_entry: usize = 0;

    static int kyber_##name##_tokens_show(void *data, seq_file *m)	
    {									
    let mut q = data;					
    let mut kqd = q.elevator.elevator_data;	
    
    sbitmap_queue_show(&kqd.domain_tokens[domain], m);		
    return 0;							
    }									
    
    static void *kyber_##name##_rqs_start(seq_file *m, loff_t *pos)	
    __acquires(&KYBER_HCTX_DATA(HCTX_FROM_SEQ_FILE(m)).lock)	
    {									
    let mut hctx = m.private;			
    let mut khd = hctx.sched_data;			
    
    spin_lock(&khd.lock);						
    return seq_list_start(&khd.rqs[domain], *pos);			
    }									
    
    static void *kyber_##name##_rqs_next(seq_file *m, void *v,	
    loff_t *pos)			
    {									
    let mut hctx = m.private;			
    let mut khd = hctx.sched_data;			
    
    return seq_list_next(v, &khd.rqs[domain], pos);		
    }									
    
    static void kyber_##name##_rqs_stop(seq_file *m, void *v)	
    __releases(&KYBER_HCTX_DATA(HCTX_FROM_SEQ_FILE(m)).lock)	
    {									
    let mut hctx = m.private;			
    let mut khd = hctx.sched_data;			
    
    spin_unlock(&khd.lock);					
    }									
    
    static const struct seq_operations kyber_##name##_rqs_seq_ops = {	
    .start	= kyber_##name##_rqs_start,				
    .next	= kyber_##name##_rqs_next,				
    .stop	= kyber_##name##_rqs_stop,				
    .show	= blk_mq_debugfs_rq_show,				
    };									
    
    static int kyber_##name##_waiting_show(void *data, seq_file *m)	
    {									
    let mut hctx = data;				
    let mut khd = hctx.sched_data;			
    let mut wait = &khd.domain_wait[domain].wait;	
    
    seq_printf(m, "%d\n", !list_empty_careful(&wait.entry));	
    return 0;							
    }
    KYBER_DEBUGFS_DOMAIN_ATTRS(KYBER_READ, read)
    KYBER_DEBUGFS_DOMAIN_ATTRS(KYBER_WRITE, write)
    KYBER_DEBUGFS_DOMAIN_ATTRS(KYBER_DISCARD, discard)
    KYBER_DEBUGFS_DOMAIN_ATTRS(KYBER_OTHER, other)

#[no_mangle]
unsafe extern "C" fn kyber_cur_domain_show(data: *mut c_void, m: *mut seq_file) -> c_int {
    let mut hctx = data;
    let mut khd = hctx.sched_data;
    seq_printf(m, "%s\n", kyber_domain_names[khd.cur_domain]);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn kyber_batching_show(data: *mut c_void, m: *mut seq_file) -> c_int {
    let mut hctx = data;
    let mut khd = hctx.sched_data;
    seq_printf(m, "%u\n", khd.batching);
    return 0;
    }

    {#name "_tokens", 0400, kyber_##name##_tokens_show}
pub static mut blk_mq_debugfs_attr: usize = 0;

    {#name "_rqs", 0400, .seq_ops = &kyber_##name##_rqs_seq_ops},	
    {#name "_waiting", 0400, kyber_##name##_waiting_show}
pub static mut blk_mq_debugfs_attr: usize = 0;

pub static mut elevator_type: usize = 0;
#[no_mangle]
unsafe extern "C" fn kyber_init() -> c_int {
    return elv_register(&kyber_sched);
    }
#[no_mangle]
unsafe extern "C" fn kyber_exit()  {
    elv_unregister(&kyber_sched);
    }
    module_init!(kyber_init);
    module_exit!(kyber_exit);
    MODULE_AUTHOR("Omar Sandoval");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("Kyber I/O scheduler");