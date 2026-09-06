//! Automatically rewritten from C to Rust
//! Source: block/mq-deadline.c
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
// MQ Deadline i/o scheduler - adaptation of the legacy deadline scheduler,
// for the blk-mq scheduling framework
//
// Copyright (C) 2016 Jens Axboe <axboe@kernel.dk>
//

//
// See Documentation/block/deadline-iosched.rst
//
    static const int read_expire = HZ / 2;  /* max time before a read is submitted. */
    static const int write_expire = 5 * HZ; /* ditto for writes, these limits are SOFT! */
//
// Time after which to dispatch lower priority requests even if higher
// priority requests are pending.
//
pub static mut prio_aging_expire: int = 0;
    static const int writes_starved = 2;    /* max times reads can starve a write */
    static const int fifo_batch = 16;       // # of sequential requests treated as one
    by the above parameters. For throughput. */
    enum dd_data_dir {
    DD_READ		= READ,
    DD_WRITE	= WRITE,
    };
    enum { DD_DIR_COUNT = 2 };
    enum dd_prio {
    DD_RT_PRIO	= 0,
    DD_BE_PRIO	= 1,
    DD_IDLE_PRIO	= 2,
    DD_PRIO_MAX	= 2,
    };
    enum { DD_PRIO_COUNT = 3 };
//
// I/O statistics per I/O priority. It is fine if these counters overflow.
// What matters is that these counters are at least as wide as
// log2(max_outstanding_requests).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_stats_per_prio {
    pub inserted: u32,
    pub merged: u32,
    pub dispatched: u32,
    pub completed: core::sync::atomic::AtomicI32,
}

//
// Deadline scheduler data per I/O priority (enum dd_prio). Requests are
// present on both sort_list[] and fifo_list[].
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dd_per_prio {
    pub sort_list: [rb_root; DD_DIR_COUNT],
    pub fifo_list: [list_head; DD_DIR_COUNT],
// Position of the most recently dispatched request.
    pub latest_pos: [sector_t; DD_DIR_COUNT],
    pub stats: io_stats_per_prio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct deadline_data {
//
// run time data
//
    pub dispatch: list_head,
    pub per_prio: [dd_per_prio; DD_PRIO_COUNT],
// Data direction of latest dispatched request.
    pub last_dir: dd_data_dir,
//     pub /: *mut *mut unsigned int batching; / number of sequential requests made,
//     pub /: *mut *mut unsigned int starved; / times reads have starved writes,
//
// settings that change how the i/o scheduler behaves
//
    pub fifo_expire: [c_int; DD_DIR_COUNT],
    pub fifo_batch: c_int,
    pub writes_starved: c_int,
    pub front_merges: c_int,
    pub prio_aging_expire: c_int,
    pub lock: spinlock_t,
}

// Maps an I/O priority class to a deadline scheduler priority.
    static const enum dd_prio ioprio_class_to_prio[] = {
    [IOPRIO_CLASS_NONE]	= DD_BE_PRIO,
    [IOPRIO_CLASS_RT]	= DD_RT_PRIO,
    [IOPRIO_CLASS_BE]	= DD_BE_PRIO,
    [IOPRIO_CLASS_IDLE]	= DD_IDLE_PRIO,
    };
#[no_mangle]
pub unsafe extern "C" fn deadline_rb_root(per_prio: *mut dd_per_prio, rq: *mut request) -> *mut c_void {
    return &per_prio.sort_list[rq_data_dir(rq)];
    }
//
// Returns the I/O priority class (IOPRIO_CLASS_*) that has been assigned to a
// request.
//
#[no_mangle]
unsafe extern "C" fn dd_rq_ioclass(rq: *mut request) -> u8 {
    return IOPRIO_PRIO_CLASS(req_get_ioprio(rq));
    }
//
// Return the first request for which blk_rq_pos() >= @pos.
//
#[no_mangle]
pub unsafe extern "C" fn deadline_from_pos(per_prio: *mut dd_per_prio, data_dir: dd_data_dir, pos: sector_t) -> *mut c_void {
    let mut node = per_prio.sort_list[data_dir].rb_node;
    struct request *rq, *res = core::ptr::null_mut();
    while (node) {
    rq = rb_entry_rq(node);
    if (blk_rq_pos(rq) >= pos) {
    res = rq;
    node = node.rb_left;
    } else {
    node = node.rb_right;
    }
    }
    return res;
    }
#[no_mangle]
pub unsafe extern "C" fn deadline_add_rq_rb(per_prio: *mut dd_per_prio, rq: *mut request) {
    let mut root = deadline_rb_root(per_prio, rq);
    elv_rb_add(root, rq);
    }
#[no_mangle]
pub unsafe extern "C" fn deadline_del_rq_rb(per_prio: *mut dd_per_prio, rq: *mut request) {
    elv_rb_del(deadline_rb_root(per_prio, rq), rq);
    }
//
// remove rq from rbtree and fifo.
//
#[no_mangle]
pub unsafe extern "C" fn deadline_remove_request(q: *mut request_queue, per_prio: *mut dd_per_prio, rq: *mut request) {
    list_del_init(&rq.queuelist);
//
// We might not be on the rbtree, if we are doing an insert merge
//
    if (!RB_EMPTY_NODE(&rq.rb_node)) {
    deadline_del_rq_rb(per_prio, rq);
    }
    elv_rqhash_del(q, rq);
    if (q.last_merge == rq) {
    q.last_merge = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn dd_request_merged(q: *mut request_queue, req: *mut request, type: elv_merge) {
    let mut dd = q.elevator.elevator_data;
pub static mut ioprio_class: u8 = 0;
pub static mut prio: dd_prio = 0;
    let mut per_prio = &dd.per_prio[prio];
//
// if the merge was a front merge, we need to reposition request
//
    if (type == ELEVATOR_FRONT_MERGE) {
    elv_rb_del(deadline_rb_root(per_prio, req), req);
    deadline_add_rq_rb(per_prio, req);
    }
    }
//
// Callback function that is invoked after @next has been merged into @req.
//
#[no_mangle]
pub unsafe extern "C" fn dd_merged_requests(q: *mut request_queue, req: *mut request, next: *mut request) {
    let mut dd = q.elevator.elevator_data;
pub static mut ioprio_class: u8 = 0;
pub static mut prio: dd_prio = 0;
    lockdep_assert_held(&dd.lock);
    dd.per_prio[prio].stats.merged += 1;
//
// if next expires before rq, assign its expire time to rq
// and move into next position (next will be deleted) in fifo
//
    if (!list_empty(&req.queuelist) && !list_empty(&next.queuelist)) {
    if (time_before((unsigned long)next.fifo_time,
    (unsigned long)req.fifo_time)) {
    list_move(&req.queuelist, &next.queuelist);
    req.fifo_time = next.fifo_time;
    }
    }
//
// kill knowledge of next, this one is a goner
//
    deadline_remove_request(q, &dd.per_prio[prio], next);
    }
//
// move an entry to dispatch queue
//
#[no_mangle]
pub unsafe extern "C" fn deadline_move_request(per_prio: *mut dd_per_prio, rq: *mut request) {
//
// take it off the sort and fifo list
//
    deadline_remove_request(rq.q, per_prio, rq);
    }
// Number of requests queued for a given priority level.
#[no_mangle]
unsafe extern "C" fn dd_queued(dd: *mut deadline_data, prio: dd_prio) -> u32 {
    let mut stats = &dd.per_prio[prio].stats;
    lockdep_assert_held(&dd.lock);
    return stats.inserted - atomic_read(&stats.completed);
    }
//
// deadline_check_fifo returns true if and only if there are expired requests
// in the FIFO list. Requires !list_empty(&dd->fifo_list[data_dir]).
//
#[no_mangle]
pub unsafe extern "C" fn deadline_check_fifo(per_prio: *mut dd_per_prio, data_dir: dd_data_dir) -> bool {
    let mut rq = rq_entry_fifo(per_prio.fifo_list[data_dir].next);
    return time_is_before_eq_jiffies((unsigned long)rq.fifo_time);
    }
//
// For the specified data direction, return the next request to
// dispatch using arrival ordered lists.
//
#[no_mangle]
pub unsafe extern "C" fn deadline_fifo_request(per_prio: *mut dd_per_prio, data_dir: dd_data_dir) -> *mut c_void {
    if (list_empty(&per_prio.fifo_list[data_dir])) {
    return core::ptr::null_mut();
    }
    return rq_entry_fifo(per_prio.fifo_list[data_dir].next);
    }
//
// For the specified data direction, return the next request to
// dispatch using sector position sorted lists.
//
#[no_mangle]
pub unsafe extern "C" fn deadline_next_request(per_prio: *mut dd_per_prio, data_dir: dd_data_dir) -> *mut c_void {
    return deadline_from_pos(per_prio, data_dir,
    per_prio.latest_pos[data_dir]);
    }
//
// Returns true if and only if @rq started after @latest_start where
// @latest_start is in jiffies.
//
#[no_mangle]
pub unsafe extern "C" fn started_after(dd: *mut deadline_data, rq: *mut request, latest_start: c_ulong) -> bool {
pub static mut start_time: c_ulong = 0;
    start_time -= dd.fifo_expire[rq_data_dir(rq)];
    return time_after(start_time, latest_start);
    }
#[no_mangle]
pub unsafe extern "C" fn dd_start_request(dd: *mut deadline_data, data_dir: dd_data_dir, rq: *mut request) -> *mut c_void {
pub static mut ioprio_class: u8 = 0;
pub static mut prio: dd_prio = 0;
    dd.per_prio[prio].latest_pos[data_dir] = blk_rq_pos(rq);
    dd.per_prio[prio].stats.dispatched += 1;
    rq.rq_flags |= RQF_STARTED;
    return rq;
    }
//
// deadline_dispatch_requests selects the best request according to
// read/write expire, fifo_batch, etc and with a start time <= @latest_start.
//
#[no_mangle]
pub unsafe extern "C" fn __dd_dispatch_request(dd: *mut deadline_data, per_prio: *mut dd_per_prio, latest_start: c_ulong) -> *mut c_void {
    let mut rq = core::ptr::null_mut();
    let mut next_rq = core::ptr::null_mut();
    enum dd_data_dir data_dir;
    lockdep_assert_held(&dd.lock);
//
// batches are currently reads XOR writes
//
    rq = deadline_next_request(per_prio, dd.last_dir);
    if (rq && dd.batching < dd.fifo_batch) {
// we have a next request and are still entitled to batch
    data_dir = rq_data_dir(rq);
// goto;
    }
//
// at this point we are not running a batch. select the appropriate
// data direction (read / write)
//
    if (!list_empty(&per_prio.fifo_list[DD_READ])) {
    BUG_ON!(RB_EMPTY_ROOT(&per_prio.sort_list[DD_READ]));
    if (deadline_fifo_request(per_prio, DD_WRITE) &&
    (dd.starved++ >= dd.writes_starved)) {
// goto;
    }
    data_dir = DD_READ;
// goto;
    }
//
// there are either no reads or writes have been starved
//
    if (!list_empty(&per_prio.fifo_list[DD_WRITE])) {
// label;
    BUG_ON!(RB_EMPTY_ROOT(&per_prio.sort_list[DD_WRITE]));
    dd.starved = 0;
    data_dir = DD_WRITE;
// goto;
    }
    return core::ptr::null_mut();
// label;
//
// we are not running a batch, find best request for selected data_dir
//
    next_rq = deadline_next_request(per_prio, data_dir);
    if (deadline_check_fifo(per_prio, data_dir) || !next_rq) {
//
// A deadline has expired, the last request was in the other
// direction, or we have run out of higher-sectored requests.
// Start again from the request with the earliest expiry time.
//
    rq = deadline_fifo_request(per_prio, data_dir);
    } else {
//
// The last req was the same dir and we have a next request in
// sort order. No expired requests so continue on from here.
//
    rq = next_rq;
    }
    if (!rq) {
    return core::ptr::null_mut();
    }
    dd.last_dir = data_dir;
    dd.batching = 0;
// label;
    if (started_after(dd, rq, latest_start)) {
    return core::ptr::null_mut();
    }
//
// rq is the selected appropriate request.
//
    dd.batching += 1;
    deadline_move_request(per_prio, rq);
    return dd_start_request(dd, data_dir, rq);
    }
//
// Check whether there are any requests with priority other than DD_RT_PRIO
// that were inserted more than prio_aging_expire jiffies ago.
//
#[no_mangle]
pub unsafe extern "C" fn dd_dispatch_prio_aged_requests(dd: *mut deadline_data, now: c_ulong) -> *mut c_void {
pub static mut rq: *mut c_void = core::ptr::null_mut();
    enum dd_prio prio;
    let mut prio_cnt = 0;
    lockdep_assert_held(&dd.lock);
    prio_cnt = !!dd_queued(dd, DD_RT_PRIO) + !!dd_queued(dd, DD_BE_PRIO) +
    !!dd_queued(dd, DD_IDLE_PRIO);
    if (prio_cnt < 2) {
    return core::ptr::null_mut();
    }
    while (prio <= DD_PRIO_MAX) {
    rq = __dd_dispatch_request(dd, &dd.per_prio[prio],
    now - dd.prio_aging_expire);
    if (rq) {
    return rq;
    }
    }
    return core::ptr::null_mut();
    }
//
// Called from blk_mq_run_hw_queue() -> __blk_mq_sched_dispatch_requests().
//
// One confusing aspect here is that we get called for a specific
// hardware queue, but we may return a request that is for a
// different hardware queue. This is because mq-deadline has shared
// state for all hardware queues, in terms of sorting, FIFOs, etc.
//
#[no_mangle]
pub unsafe extern "C" fn dd_dispatch_request(hctx: *mut blk_mq_hw_ctx) -> *mut c_void {
    let mut dd = hctx.queue.elevator.elevator_data;
pub static mut now: c_ulong = 0;
pub static mut rq: *mut c_void = core::ptr::null_mut();
    enum dd_prio prio;
    spin_lock(&dd.lock);
    if (!list_empty(&dd.dispatch)) {
    rq = list_first_entry(&dd.dispatch, request, queuelist);
    list_del_init(&rq.queuelist);
    dd_start_request(dd, rq_data_dir(rq), rq);
// goto;
    }
    rq = dd_dispatch_prio_aged_requests(dd, now);
    if (rq) {
// goto;
    }
//
// Next, dispatch requests in priority order. Ignore lower priority
// requests if any higher priority requests are pending.
//
    while (prio <= DD_PRIO_MAX) {
    rq = __dd_dispatch_request(dd, &dd.per_prio[prio], now);
    if (rq || dd_queued(dd, prio)) {
    break;
    }
    }
// label;
    spin_unlock(&dd.lock);
    return rq;
    }
#[no_mangle]
unsafe extern "C" fn dd_limit_depth(opf: blk_opf_t, data: *mut blk_mq_alloc_data) {
    if (!blk_mq_is_sync_read(opf)) {
    data.shallow_depth = data.q.async_depth;
    }
    }
// Called by blk_mq_init_sched() and blk_mq_update_nr_requests().
#[no_mangle]
unsafe extern "C" fn dd_depth_updated(q: *mut request_queue) {
    blk_mq_set_min_shallow_depth(q, q.async_depth);
    }
#[no_mangle]
unsafe extern "C" fn dd_exit_sched(e: *mut elevator_queue) {
    let mut dd = e.elevator_data;
    enum dd_prio prio;
    while (prio <= DD_PRIO_MAX) {
    let mut per_prio = &dd.per_prio[prio];
    let mut stats = &per_prio.stats;
    let mut queued;
    WARN_ON_ONCE!(!list_empty(&per_prio.fifo_list[DD_READ]));
    WARN_ON_ONCE!(!list_empty(&per_prio.fifo_list[DD_WRITE]));
    spin_lock(&dd.lock);
    queued = dd_queued(dd, prio);
    spin_unlock(&dd.lock);
    WARN_ONCE(queued != 0,
    "statistics for priority %d: i %u m %u d %u c %u\n",
    prio, stats.inserted, stats.merged,
    stats.dispatched, atomic_read(&stats.completed));
    }
    kfree(dd);
    }
//
// initialize elevator private data (deadline_data).
//
#[no_mangle]
unsafe extern "C" fn dd_init_sched(q: *mut request_queue, eq: *mut elevator_queue) -> c_int {
pub static mut dd: *mut c_void = core::ptr::null_mut();
    enum dd_prio prio;
    dd = kzalloc_node(sizeof!(*dd), GFP_KERNEL, q.node);
    if (!dd) {
    return -ENOMEM;
    }
    eq.elevator_data = dd;
    INIT_LIST_HEAD(&dd.dispatch);
    while (prio <= DD_PRIO_MAX) {
    let mut per_prio = &dd.per_prio[prio];
    INIT_LIST_HEAD(&per_prio.fifo_list[DD_READ]);
    INIT_LIST_HEAD(&per_prio.fifo_list[DD_WRITE]);
    per_prio.sort_list[DD_READ] = RB_ROOT;
    per_prio.sort_list[DD_WRITE] = RB_ROOT;
    }
    dd.fifo_expire[DD_READ] = read_expire;
    dd.fifo_expire[DD_WRITE] = write_expire;
    dd.writes_starved = writes_starved;
    dd.front_merges = 1;
    dd.last_dir = DD_WRITE;
    dd.fifo_batch = fifo_batch;
    dd.prio_aging_expire = prio_aging_expire;
    spin_lock_init(&dd.lock);
// We dispatch from request queue wide instead of hw queue
    blk_queue_flag_set(QUEUE_FLAG_SQ_SCHED, q);
    q.elevator = eq;
    q.async_depth = q.nr_requests;
    dd_depth_updated(q);
    return 0;
    }
//
// Try to merge @bio into an existing request. If @bio has been merged into
// an existing request, store the pointer to that request into *@rq.
//
#[no_mangle]
pub unsafe extern "C" fn dd_request_merge(q: *mut request_queue, rq: *mut *mut request, bio: *mut bio) -> c_int {
    let mut dd = q.elevator.elevator_data;
pub static mut ioprio_class: u8 = 0;
pub static mut prio: dd_prio = 0;
    let mut per_prio = &dd.per_prio[prio];
pub static mut sector: sector_t = 0;
pub static mut __rq: *mut c_void = core::ptr::null_mut();
    if (!dd.front_merges) {
    return ELEVATOR_NO_MERGE;
    }
    __rq = elv_rb_find(&per_prio.sort_list[bio_data_dir(bio)], sector);
    if (__rq) {
    BUG_ON!(sector != blk_rq_pos(__rq));
    if (elv_bio_merge_ok(__rq, bio)) {
// rq = __rq;
    if (blk_discard_mergable(__rq)) {
    return ELEVATOR_DISCARD_MERGE;
    }
    return ELEVATOR_FRONT_MERGE;
    }
    }
    return ELEVATOR_NO_MERGE;
    }
//
// Attempt to merge a bio into an existing request. This function is called
// before @bio is associated with a request.
//
#[no_mangle]
pub unsafe extern "C" fn dd_bio_merge(q: *mut request_queue, bio: *mut bio, nr_segs: c_uint) -> bool {
    let mut dd = q.elevator.elevator_data;
    let mut free = core::ptr::null_mut();
    let mut ret = 0;
    spin_lock(&dd.lock);
    ret = blk_mq_sched_try_merge(q, bio, nr_segs, &free);
    spin_unlock(&dd.lock);
    if (free) {
    blk_mq_free_request(free);
    }
    return ret;
    }
//
// add rq to rbtree and fifo
//
#[no_mangle]
pub unsafe extern "C" fn dd_insert_request(hctx: *mut blk_mq_hw_ctx, rq: *mut request, flags: blk_insert_t, free: *mut list_head) {
    let mut q = hctx.queue;
    let mut dd = q.elevator.elevator_data;
pub static mut data_dir: dd_data_dir = 0;
pub static mut ioprio: u16 = 0;
pub static mut ioprio_class: u8 = 0;
pub static mut per_prio: *mut c_void = core::ptr::null_mut();
    enum dd_prio prio;
    lockdep_assert_held(&dd.lock);
    prio = ioprio_class_to_prio[ioprio_class];
    per_prio = &dd.per_prio[prio];
    if (!rq.elv.priv[0]) {
    per_prio.stats.inserted += 1;
    }
    rq.elv.priv[0] = per_prio;
    if (blk_mq_sched_try_insert_merge(q, rq, free)) {
    return;
    }
    trace_block_rq_insert(rq);
    if (flags & BLK_MQ_INSERT_AT_HEAD) {
    list_add(&rq.queuelist, &dd.dispatch);
    rq.fifo_time = jiffies;
    } else {
    deadline_add_rq_rb(per_prio, rq);
    if (rq_mergeable(rq)) {
    elv_rqhash_add(q, rq);
    if (!q.last_merge) {
    q.last_merge = rq;
    }
    }
//
// set expire time and add to fifo list
//
    rq.fifo_time = jiffies + dd.fifo_expire[data_dir];
    list_add_tail(&rq.queuelist, &per_prio.fifo_list[data_dir]);
    }
    }
//
// Called from blk_mq_insert_request() or blk_mq_dispatch_list().
//
#[no_mangle]
pub unsafe extern "C" fn dd_insert_requests(hctx: *mut blk_mq_hw_ctx, list: *mut list_head, flags: blk_insert_t) {
    let mut q = hctx.queue;
    let mut dd = q.elevator.elevator_data;
pub static mut free: usize = 0;
    spin_lock(&dd.lock);
    while (!list_empty(list)) {
pub static mut rq: *mut c_void = core::ptr::null_mut();
    rq = list_first_entry(list, request, queuelist);
    list_del_init(&rq.queuelist);
    dd_insert_request(hctx, rq, flags, &free);
    }
    spin_unlock(&dd.lock);
    blk_mq_free_requests(&free);
    }
// Callback from inside blk_mq_rq_ctx_init().
#[no_mangle]
unsafe extern "C" fn dd_prepare_request(rq: *mut request) {
    rq.elv.priv[0] = core::ptr::null_mut();
    }
//
// Callback from inside blk_mq_free_request().
//
#[no_mangle]
unsafe extern "C" fn dd_finish_request(rq: *mut request) {
    let mut per_prio = rq.elv.priv[0];
//
// The block layer core may call dd_finish_request() without having
// called dd_insert_requests(). Skip requests that bypassed I/O
// scheduling. See also blk_mq_request_bypass_insert().
//
    if (per_prio) {
    atomic_inc(&per_prio.stats.completed);
    }
    }
#[no_mangle]
unsafe extern "C" fn dd_has_work_for_prio(per_prio: *mut dd_per_prio) -> bool {
    return !list_empty_careful(&per_prio.fifo_list[DD_READ]) ||
    !list_empty_careful(&per_prio.fifo_list[DD_WRITE]);
    }
#[no_mangle]
unsafe extern "C" fn dd_has_work(hctx: *mut blk_mq_hw_ctx) -> bool {
    let mut dd = hctx.queue.elevator.elevator_data;
    enum dd_prio prio;
    if (!list_empty_careful(&dd.dispatch)) {
    return true;
    }
    for (prio = 0; prio <= DD_PRIO_MAX; prio++) {
    if (dd_has_work_for_prio(&dd.per_prio[prio]))
    return true;
    }
    return false;
    }
//
// sysfs parts below
//

#[no_mangle]
pub unsafe extern "C" fn __FUNC(e: *mut elevator_queue, page: *mut c_char) -> ssize_t {									
    let mut dd = e.elevator_data;			
    
    return sysfs_emit(page, "%d\n", __VAR);				
    }

    SHOW_JIFFIES(deadline_read_expire_show, dd.fifo_expire[DD_READ]);
    SHOW_JIFFIES(deadline_write_expire_show, dd.fifo_expire[DD_WRITE]);
    SHOW_JIFFIES(deadline_prio_aging_expire_show, dd.prio_aging_expire);
    SHOW_INT(deadline_writes_starved_show, dd.writes_starved);
    SHOW_INT(deadline_front_merges_show, dd.front_merges);
    SHOW_INT(deadline_fifo_batch_show, dd.fifo_batch);

#[no_mangle]
#[no_mangle]
// duplicate fn: __FUNC
pub unsafe extern "C" fn __FUNC_dup(e: *mut elevator_queue, page: *mut c_char, count: size_t) -> ssize_t {									
    let mut dd = e.elevator_data;			
    let mut __data = 0;
    let mut __ret = 0;						
    
    __ret = kstrtoint(page, 0, &__data);				
    if (__ret < 0)							 {
    return __ret;						
    }
    if (__data < (MIN))						 {
    __data = (MIN);						
    }
    else if (__data > (MAX))					 {
    __data = (MAX);						
    }
// (__PTR) = __CONV(__data);					
    return count;							
    }

    STORE_FUNCTION(__FUNC, __PTR, MIN, MAX, )

    STORE_FUNCTION(__FUNC, __PTR, MIN, MAX, msecs_to_jiffies)
    STORE_JIFFIES(deadline_read_expire_store, &dd.fifo_expire[DD_READ], 0, INT_MAX);
    STORE_JIFFIES(deadline_write_expire_store, &dd.fifo_expire[DD_WRITE], 0, INT_MAX);
    STORE_JIFFIES(deadline_prio_aging_expire_store, &dd.prio_aging_expire, 0, INT_MAX);
    STORE_INT(deadline_writes_starved_store, &dd.writes_starved, INT_MIN, INT_MAX);
    STORE_INT(deadline_front_merges_store, &dd.front_merges, 0, 1);
    STORE_INT(deadline_fifo_batch_store, &dd.fifo_batch, 0, INT_MAX);

    __ATTR(name, 0644, deadline_##name##_show, deadline_##name##_store)
pub static mut elv_fs_entry: usize = 0;

    ((rq).elevator.elevator_data)

    static void *deadline_##name##_fifo_start(seq_file *m,		
    loff_t *pos)			
    __acquires(&DD_DATA_FROM_RQ(RQ_FROM_SEQ_FILE(m)).lock)		
    {									
    let mut q = m.private;				
    let mut dd = q.elevator.elevator_data;		
    let mut per_prio = &dd.per_prio[prio];		
    
    spin_lock(&dd.lock);						
    return seq_list_start(&per_prio.fifo_list[data_dir], *pos);	
    }									
    
    static void *deadline_##name##_fifo_next(seq_file *m, void *v,	
    loff_t *pos)			
    {									
    let mut q = m.private;				
    let mut dd = q.elevator.elevator_data;		
    let mut per_prio = &dd.per_prio[prio];		
    
    return seq_list_next(v, &per_prio.fifo_list[data_dir], pos);	
    }									
    
    static void deadline_##name##_fifo_stop(seq_file *m, void *v)	
    __releases(&DD_DATA_FROM_RQ(RQ_FROM_SEQ_FILE(m)).lock)		
    {									
    let mut q = m.private;				
    let mut dd = q.elevator.elevator_data;		
    
    spin_unlock(&dd.lock);						
    }									
    
    static const struct seq_operations deadline_##name##_fifo_seq_ops = {	
    .start	= deadline_##name##_fifo_start,				
    .next	= deadline_##name##_fifo_next,				
    .stop	= deadline_##name##_fifo_stop,				
    .show	= blk_mq_debugfs_rq_show,				
    };									
    
    static int deadline_##name##_next_rq_show(void *data, seq_file *m)		
    {									
    let mut q = data;					
    let mut dd = q.elevator.elevator_data;		
    let mut per_prio = &dd.per_prio[prio];		
pub static mut rq: *mut c_void = core::ptr::null_mut();						
    
    rq = deadline_from_pos(per_prio, data_dir,			
    per_prio.latest_pos[data_dir]);		
    if (rq)								 {
    __blk_mq_debugfs_rq_show(m, rq);			
    }
    return 0;							
    }
    DEADLINE_DEBUGFS_DDIR_ATTRS(DD_RT_PRIO, DD_READ, read0);
    DEADLINE_DEBUGFS_DDIR_ATTRS(DD_RT_PRIO, DD_WRITE, write0);
    DEADLINE_DEBUGFS_DDIR_ATTRS(DD_BE_PRIO, DD_READ, read1);
    DEADLINE_DEBUGFS_DDIR_ATTRS(DD_BE_PRIO, DD_WRITE, write1);
    DEADLINE_DEBUGFS_DDIR_ATTRS(DD_IDLE_PRIO, DD_READ, read2);
    DEADLINE_DEBUGFS_DDIR_ATTRS(DD_IDLE_PRIO, DD_WRITE, write2);

#[no_mangle]
unsafe extern "C" fn deadline_batching_show(data: *mut c_void, m: *mut seq_file) -> c_int {
    let mut q = data;
    let mut dd = q.elevator.elevator_data;
    seq_printf(m, "%u\n", dd.batching);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn deadline_starved_show(data: *mut c_void, m: *mut seq_file) -> c_int {
    let mut q = data;
    let mut dd = q.elevator.elevator_data;
    seq_printf(m, "%u\n", dd.starved);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dd_queued_show(data: *mut c_void, m: *mut seq_file) -> c_int {
    let mut q = data;
    let mut dd = q.elevator.elevator_data;
    u32 rt, be, idle;
    spin_lock(&dd.lock);
    rt = dd_queued(dd, DD_RT_PRIO);
    be = dd_queued(dd, DD_BE_PRIO);
    idle = dd_queued(dd, DD_IDLE_PRIO);
    spin_unlock(&dd.lock);
    seq_printf(m, "%u %u %u\n", rt, be, idle);
    return 0;
    }
// Number of requests owned by the block driver for a given priority.
#[no_mangle]
unsafe extern "C" fn dd_owned_by_driver(dd: *mut deadline_data, prio: dd_prio) -> u32 {
    let mut stats = &dd.per_prio[prio].stats;
    lockdep_assert_held(&dd.lock);
    return stats.dispatched + stats.merged -
    atomic_read(&stats.completed);
    }
#[no_mangle]
unsafe extern "C" fn dd_owned_by_driver_show(data: *mut c_void, m: *mut seq_file) -> c_int {
    let mut q = data;
    let mut dd = q.elevator.elevator_data;
    u32 rt, be, idle;
    spin_lock(&dd.lock);
    rt = dd_owned_by_driver(dd, DD_RT_PRIO);
    be = dd_owned_by_driver(dd, DD_BE_PRIO);
    idle = dd_owned_by_driver(dd, DD_IDLE_PRIO);
    spin_unlock(&dd.lock);
    seq_printf(m, "%u %u %u\n", rt, be, idle);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn deadline_dispatch_start(m: *mut seq_file, lock: *mut loff_tpos)
    __acquires(&DD_DATA_FROM_RQ(RQ_FROM_SEQ_FILE(m)).) -> *mut c_void {
    let mut q = m.private;
    let mut dd = q.elevator.elevator_data;
    spin_lock(&dd.lock);
    return seq_list_start(&dd.dispatch, *pos);
    }
#[no_mangle]
pub unsafe extern "C" fn deadline_dispatch_next(m: *mut seq_file, v: *mut c_void, pos: *mut loff_t) -> *mut c_void {
    let mut q = m.private;
    let mut dd = q.elevator.elevator_data;
    return seq_list_next(v, &dd.dispatch, pos);
    }
#[no_mangle]
unsafe extern "C" fn deadline_dispatch_stop(m: *mut seq_file, v: *mut c_void) {
    let mut q = m.private;
    let mut dd = q.elevator.elevator_data;
    spin_unlock(&dd.lock);
    }
pub static mut seq_operations: usize = 0;

    {#name "_fifo_list", 0400,					
    .seq_ops = &deadline_##name##_fifo_seq_ops}

    {#name "_next_rq", 0400, deadline_##name##_next_rq_show}
pub static mut blk_mq_debugfs_attr: usize = 0;

pub static mut elevator_type: usize = 0;
    MODULE_ALIAS("mq-deadline-iosched");
#[no_mangle]
unsafe extern "C" fn deadline_init() -> c_int {
    return elv_register(&mq_deadline);
    }
#[no_mangle]
unsafe extern "C" fn deadline_exit()  {
    elv_unregister(&mq_deadline);
    }
    module_init!(deadline_init);
    module_exit!(deadline_exit);
    MODULE_AUTHOR("Jens Axboe, Damien Le Moal and Bart Van Assche");
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("MQ deadline IO scheduler");