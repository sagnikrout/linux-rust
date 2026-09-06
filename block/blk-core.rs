//! Automatically rewritten from C to Rust
//! Source: block/blk-core.c
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
// Copyright (C) 1991, 1992 Linus Torvalds
// Copyright (C) 1994,      Karl Keyte: Added support for disk statistics
// Elevator latency, (C) 2000  Andrea Arcangeli <andrea@suse.de> SuSE
// Queue request tables / lock, selectable elevator, Jens Axboe <axboe@suse.de>
// kernel-doc documentation started by NeilBrown <neilb@cse.unsw.edu.au>
// -  July2000
// bio rewrite, highmem i/o, etc, Jens Axboe <axboe@suse.de> - may 2001
//
// This handles all read/write requests to block devices
//

// Macro flag: #define CREATE_TRACE_POINTS

pub static mut blk_debugfs_root: *mut c_void = core::ptr::null_mut();
    EXPORT_TRACEPOINT_SYMBOL_GPL(block_bio_remap);
    EXPORT_TRACEPOINT_SYMBOL_GPL(block_rq_remap);
    EXPORT_TRACEPOINT_SYMBOL_GPL(block_bio_complete);
    EXPORT_TRACEPOINT_SYMBOL_GPL(block_split);
    EXPORT_TRACEPOINT_SYMBOL_GPL(block_unplug);
    EXPORT_TRACEPOINT_SYMBOL_GPL(block_rq_insert);
pub static mut blk_queue_ida: usize = 0;
//
// For queue allocation
//
pub static mut blk_requestq_cachep: *mut c_void = core::ptr::null_mut();
//
// Controlling structure to kblockd
//
pub static mut kblockd_workqueue: *mut c_void = core::ptr::null_mut();
//
// blk_queue_flag_set - atomically set a queue flag
// @flag: flag to be set
// @q: request queue
//
#[no_mangle]
pub unsafe extern "C" fn blk_queue_flag_set(flag: c_uint, q: *mut request_queue) {
    set_bit(flag, &q.queue_flags);
    }
    EXPORT_SYMBOL(blk_queue_flag_set);
//
// blk_queue_flag_clear - atomically clear a queue flag
// @flag: flag to be cleared
// @q: request queue
//
#[no_mangle]
pub unsafe extern "C" fn blk_queue_flag_clear(flag: c_uint, q: *mut request_queue) {
    clear_bit(flag, &q.queue_flags);
    }
    EXPORT_SYMBOL(blk_queue_flag_clear);

    static const char *const blk_op_name[] = {
    REQ_OP_NAME(READ),
    REQ_OP_NAME(WRITE),
    REQ_OP_NAME(FLUSH),
    REQ_OP_NAME(DISCARD),
    REQ_OP_NAME(SECURE_ERASE),
    REQ_OP_NAME(ZONE_RESET),
    REQ_OP_NAME(ZONE_RESET_ALL),
    REQ_OP_NAME(ZONE_OPEN),
    REQ_OP_NAME(ZONE_CLOSE),
    REQ_OP_NAME(ZONE_FINISH),
    REQ_OP_NAME(ZONE_APPEND),
    REQ_OP_NAME(WRITE_ZEROES),
    REQ_OP_NAME(DRV_IN),
    REQ_OP_NAME(DRV_OUT),
    };

//
// blk_op_str - Return the string "name" for an operation REQ_OP_name.
// @op: a request operation.
//
// Convert a request operation REQ_OP_name into the string "name". Useful for
// debugging and tracing BIOs and requests. For an invalid request operation
// code, the string "UNKNOWN" is returned.
//
    inline const char *blk_op_str(enum req_op op)
    {
    let mut op_str = "UNKNOWN";
    if (op < ARRAY_SIZE!(blk_op_name) && blk_op_name[op]) {
    op_str = blk_op_name[op];
    }
    return op_str;
    }
    EXPORT_SYMBOL_GPL(blk_op_str);
#[no_mangle]
pub unsafe extern "C" fn str_to_blk_op(op: *const c_char) -> enum req_op {
    let mut i = 0;
    for (i = 0; i < ARRAY_SIZE!(blk_op_name); i++) {
    if (blk_op_name[i] && !strcmp(blk_op_name[i], op))
    return (enum req_op)i;
    }
    return REQ_OP_LAST;
    }

    [BLK_STS_##_tag] = {				
    .errno		= _errno,		
    .tag		= __stringify(_tag),	
    .name		= _desc,		
    }
pub static mut blk_errors: usize = 0;

#[no_mangle]
pub unsafe extern "C" fn errno_to_blk_status(errno: c_int) -> blk_status_t {
    let mut i = 0;
    while (i < ARRAY_SIZE!(blk_errors)) {
    if (blk_errors[i].errno == errno) {
    return ( blk_status_t)i;
    }
    }
    return BLK_STS_IOERR;
    }
    EXPORT_SYMBOL_GPL(errno_to_blk_status);
#[no_mangle]
pub unsafe extern "C" fn blk_status_to_errno(status: blk_status_t) -> c_int {
pub static mut idx: c_int = 0;
    if (WARN_ON_ONCE!(idx >= ARRAY_SIZE!(blk_errors))) {
    return -EIO;
    }
    return blk_errors[idx].errno;
    }
    EXPORT_SYMBOL_GPL(blk_status_to_errno);
    const char *blk_status_to_str(blk_status_t status)
    {
pub static mut idx: c_int = 0;
    if (WARN_ON_ONCE!(idx >= ARRAY_SIZE!(blk_errors))) {
    return "<null>";
    }
    return blk_errors[idx].name;
    }
    const char *blk_status_to_tag(blk_status_t status)
    {
pub static mut idx: c_int = 0;
    if (WARN_ON_ONCE!(idx >= ARRAY_SIZE!(blk_errors) || !blk_errors[idx].tag)) {
    return "<null>";
    }
    return blk_errors[idx].tag;
    }
#[no_mangle]
pub unsafe extern "C" fn tag_to_blk_status(tag: *const c_char) -> blk_status_t {
    let mut i = 0;
    while (i < ARRAY_SIZE!(blk_errors)) {
    if (blk_errors[i].tag &&
    !strcmp(blk_errors[i].tag, tag)) {
    return ( blk_status_t)i;
    }
    }
//
// Return BLK_STS_OK for mismatches as this function is intended to
// parse error status values.
//
    return BLK_STS_OK;
    }
//
// blk_sync_queue - cancel any pending callbacks on a queue
// @q: the queue
//
// Description:
// The block layer may perform asynchronous callback activity
// on a queue, such as calling the unplug function after a timeout.
// A block device may call blk_sync_queue to ensure that any
// such activity is cancelled, thus allowing it to release resources
// that the callbacks might use. The caller must already have made sure
// that its ->submit_bio will not re-add plugging prior to calling
// this function.
//
// This function does not cancel any asynchronous activity arising
// out of elevator or throttling code. That would require elevator_exit()
// and blkcg_exit_queue() to be called with queue lock initialized.
//
#[no_mangle]
pub unsafe extern "C" fn blk_sync_queue(q: *mut request_queue) {
    timer_delete_sync(&q.timeout);
    cancel_work_sync(&q.timeout_work);
    }
    EXPORT_SYMBOL(blk_sync_queue);
//
// blk_set_pm_only - increment pm_only counter
// @q: request queue pointer
//
#[no_mangle]
pub unsafe extern "C" fn blk_set_pm_only(q: *mut request_queue) {
    atomic_inc(&q.pm_only);
    }
    EXPORT_SYMBOL_GPL(blk_set_pm_only);
#[no_mangle]
pub unsafe extern "C" fn blk_clear_pm_only(q: *mut request_queue) {
    let mut pm_only = 0;
    pm_only = atomic_dec_return(&q.pm_only);
    WARN_ON_ONCE!(pm_only < 0);
    if (pm_only == 0) {
    wake_up_all(&q.mq_freeze_wq);
    }
    }
    EXPORT_SYMBOL_GPL(blk_clear_pm_only);
#[no_mangle]
unsafe extern "C" fn blk_free_queue_rcu(rcu_head: *mut rcu_head) {
    let mut q = container_of!(rcu_head, request_queue, rcu_head);
    percpu_ref_exit(&q.q_usage_counter);
    kmem_cache_free(blk_requestq_cachep, q);
    }
#[no_mangle]
unsafe extern "C" fn blk_free_queue(q: *mut request_queue) {
    blk_free_queue_stats(q.stats);
    if (queue_is_mq(q)) {
    blk_mq_release(q);
    }
    ida_free(&blk_queue_ida, q.id);
    lockdep_unregister_key(&q.io_lock_cls_key);
    lockdep_unregister_key(&q.q_lock_cls_key);
    call_rcu(&q.rcu_head, blk_free_queue_rcu);
    }
//
// blk_put_queue - decrement the request_queue refcount
// @q: the request_queue structure to decrement the refcount for
//
// Decrements the refcount of the request_queue and free it when the refcount
// reaches 0.
//
#[no_mangle]
pub unsafe extern "C" fn blk_put_queue(q: *mut request_queue) {
    if (refcount_dec_and_test(&q.refs)) {
    blk_free_queue(q);
    }
    }
    EXPORT_SYMBOL(blk_put_queue);
#[no_mangle]
pub unsafe extern "C" fn blk_queue_start_drain(q: *mut request_queue) -> bool {
//
// When queue DYING flag is set, we need to block new req
// entering queue, so we call blk_freeze_queue_start() to
// prevent I/O from crossing blk_queue_enter().
//
pub static mut freeze: bool = false;
    if (queue_is_mq(q)) {
    blk_mq_wake_waiters(q);
    }
// Make blk_queue_enter() reexamine the DYING flag.
    wake_up_all(&q.mq_freeze_wq);
    return freeze;
    }
//
// blk_queue_enter() - try to increase q->q_usage_counter
// @q: request queue pointer
// @flags: BLK_MQ_REQ_NOWAIT and/or BLK_MQ_REQ_PM
//
#[no_mangle]
pub unsafe extern "C" fn blk_queue_enter(q: *mut request_queue, flags: blk_mq_req_flags_t) -> c_int {
pub static mut pm: bool = false;
    while (!blk_try_enter_queue(q, pm)) {
    if (flags & BLK_MQ_REQ_NOWAIT) {
    return -EAGAIN;
    }
//
// read pair of barrier in blk_freeze_queue_start(), we need to
// order reading __PERCPU_REF_DEAD flag of .q_usage_counter and
// reading .mq_freeze_depth or queue dying flag, otherwise the
// following wait may never return if the two reads are
// reordered.
//
    smp_rmb();
    wait_event(q.mq_freeze_wq,
    (!q.mq_freeze_depth &&
    blk_pm_resume_queue(pm, q)) ||
    blk_queue_dying(q));
    if (blk_queue_dying(q)) {
    return -ENODEV;
    }
    }
    rwsem_acquire_read(&q.q_lockdep_map, 0, 0, _RET_IP_);
    rwsem_release(&q.q_lockdep_map, _RET_IP_);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn __bio_queue_enter(q: *mut request_queue, bio: *mut bio) -> c_int {
    while (!blk_try_enter_queue(q, false)) {
    let mut disk = bio.bi_bdev.bd_disk;
    if (bio.bi_opf & REQ_NOWAIT) {
    if (test_bit(GD_DEAD, &disk.state)) {
// goto;
    }
    bio_wouldblock_error(bio);
    return -EAGAIN;
    }
//
// read pair of barrier in blk_freeze_queue_start(), we need to
// order reading __PERCPU_REF_DEAD flag of .q_usage_counter and
// reading .mq_freeze_depth or queue dying flag, otherwise the
// following wait may never return if the two reads are
// reordered.
//
    smp_rmb();
    wait_event(q.mq_freeze_wq,
    (!q.mq_freeze_depth &&
    blk_pm_resume_queue(false, q)) ||
    test_bit(GD_DEAD, &disk.state));
    if (test_bit(GD_DEAD, &disk.state)) {
// goto;
    }
    }
    rwsem_acquire_read(&q.io_lockdep_map, 0, 0, _RET_IP_);
    rwsem_release(&q.io_lockdep_map, _RET_IP_);
    return 0;
// label;
    bio_io_error(bio);
    return -ENODEV;
    }
#[no_mangle]
pub unsafe extern "C" fn blk_queue_exit(q: *mut request_queue) {
    percpu_ref_put(&q.q_usage_counter);
    }
#[no_mangle]
unsafe extern "C" fn blk_queue_usage_counter_release(ref: *mut percpu_ref) {
    let mut q = container_of!(ref, request_queue, q_usage_counter);
    wake_up_all(&q.mq_freeze_wq);
    }
#[no_mangle]
unsafe extern "C" fn blk_rq_timed_out_timer(t: *mut timer_list) {
    let mut q = timer_container_of(q, t, timeout);
    kblockd_schedule_work(&q.timeout_work);
    }
#[no_mangle]
unsafe extern "C" fn blk_timeout_work(work: *mut work_struct) {
    }
#[no_mangle]
pub unsafe extern "C" fn blk_alloc_queue(lim: *mut queue_limits, node_id: c_int) -> *mut c_void {
pub static mut q: *mut c_void = core::ptr::null_mut();
    let mut error = 0;
    q = kmem_cache_alloc_node(blk_requestq_cachep, GFP_KERNEL | __GFP_ZERO,
    node_id);
    if (!q) {
    return ERR_PTR(-ENOMEM);
    }
    q.last_merge = core::ptr::null_mut();
    q.id = ida_alloc(&blk_queue_ida, GFP_KERNEL);
    if (q.id < 0) {
    error = q.id;
// goto;
    }
    q.stats = blk_alloc_queue_stats();
    if (!q.stats) {
    error = -ENOMEM;
// goto;
    }
    error = blk_set_default_limits(lim);
    if (error) {
// goto;
    }
    q.limits = *lim;
    q.node = node_id;
    atomic_set(&q.nr_active_requests_shared_tags, 0);
    timer_setup(&q.timeout, blk_rq_timed_out_timer, 0);
    INIT_WORK(&q.timeout_work, blk_timeout_work);
    INIT_LIST_HEAD(&q.icq_list);
    refcount_set(&q.refs, 1);
    mutex_init(&q.debugfs_mutex);
    mutex_init(&q.elevator_lock);
    mutex_init(&q.sysfs_lock);
    mutex_init(&q.limits_lock);
    mutex_init(&q.rq_qos_mutex);
    spin_lock_init(&q.queue_lock);
    init_waitqueue_head(&q.mq_freeze_wq);
    mutex_init(&q.mq_freeze_lock);
    blkg_init_queue(q);
//
// Init percpu_ref in atomic mode so that it's faster to shutdown.
// See blk_register_queue() for details.
//
    error = percpu_ref_init(&q.q_usage_counter,
    blk_queue_usage_counter_release,
    PERCPU_REF_INIT_ATOMIC, GFP_KERNEL);
    if (error) {
// goto;
    }
    lockdep_register_key(&q.io_lock_cls_key);
    lockdep_register_key(&q.q_lock_cls_key);
    lockdep_init_map(&q.io_lockdep_map, "&q.q_usage_counter(io)",
    &q.io_lock_cls_key, 0);
    lockdep_init_map(&q.q_lockdep_map, "&q.q_usage_counter(queue)",
    &q.q_lock_cls_key, 0);
// Teach lockdep about lock ordering (reclaim WRT queue freeze lock).
    fs_reclaim_acquire(GFP_KERNEL);
    rwsem_acquire_read(&q.io_lockdep_map, 0, 0, _RET_IP_);
    rwsem_release(&q.io_lockdep_map, _RET_IP_);
    fs_reclaim_release(GFP_KERNEL);
    q.nr_requests = BLKDEV_DEFAULT_RQ;
    q.async_depth = BLKDEV_DEFAULT_RQ;
    return q;
// label;
    blk_free_queue_stats(q.stats);
// label;
    ida_free(&blk_queue_ida, q.id);
// label;
    kmem_cache_free(blk_requestq_cachep, q);
    return ERR_PTR(error);
    }
//
// blk_get_queue - increment the request_queue refcount
// @q: the request_queue structure to increment the refcount for
//
// Increment the refcount of the request_queue kobject.
//
// Context: Any context.
//
#[no_mangle]
pub unsafe extern "C" fn blk_get_queue(q: *mut request_queue) -> bool {
    if (unlikely(blk_queue_dying(q))) {
    return false;
    }
    refcount_inc(&q.refs);
    return true;
    }
    EXPORT_SYMBOL(blk_get_queue);

pub static mut fail_make_request: usize = 0;
#[no_mangle]
unsafe extern "C" fn setup_fail_make_request(str: *mut c_char) -> c_int {
    return setup_fault_attr(&fail_make_request, str);
    }
    __setup!("fail_make_request=", setup_fail_make_request);
#[no_mangle]
pub unsafe extern "C" fn should_fail_request(part: *mut block_device, bytes: c_uint) -> bool {
    return bdev_test_flag(part, BD_MAKE_IT_FAIL) &&
    should_fail(&fail_make_request, bytes);
    }
#[no_mangle]
unsafe extern "C" fn fail_make_request_debugfs() -> c_int {
    let mut dir = fault_create_debugfs_attr("fail_make_request",
    core::ptr::null_mut(), &fail_make_request);
    return PTR_ERR_OR_ZERO(dir);
    }
    late_initcall!(fail_make_request_debugfs);

#[no_mangle]
pub unsafe extern "C" fn bio_check_ro(bio: *mut bio) {
    if (op_is_write(bio_op(bio)) && bdev_read_only(bio.bi_bdev)) {
    if (op_is_flush(bio.bi_opf) && !bio_sectors(bio)) {
    return;
    }
    if (bdev_test_flag(bio.bi_bdev, BD_RO_WARNED)) {
    return;
    }
    bdev_set_flag(bio.bi_bdev, BD_RO_WARNED);
//
// Use ioctl to set underlying disk of raid/dm to read-only
// will trigger this.
//
    pr_warn!("Trying to write to read-only block-device %pg\n",
    bio.bi_bdev);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn should_fail_bio(bio: *mut bio) -> c_int {
    if (should_fail_request(bdev_whole(bio.bi_bdev), bio.bi_iter.bi_size)) {
    return -EIO;
    }
    return 0;
    }
    ALLOW_ERROR_INJECTION(should_fail_bio, ERRNO);
//
// Check whether this bio extends beyond the end of the device or partition.
// This may well happen - the kernel calls bread() without checking the size of
// the device, e.g., when mounting a file system.
//
#[no_mangle]
pub unsafe extern "C" fn bio_check_eod(bio: *mut bio) -> c_int {
pub static mut maxsector: sector_t = 0;
pub static mut nr_sectors: c_uint = 0;
    if (nr_sectors &&
    (nr_sectors > maxsector ||
    bio.bi_iter.bi_sector > maxsector - nr_sectors)) {
    if (!maxsector) {
    return -EIO;
    }
    pr_info_ratelimited("%s: attempt to access beyond end of device\n"
    "%pg: rw=%d, sector=%llu, nr_sectors = %u limit=%llu\n",
    current.comm, bio.bi_bdev, bio.bi_opf,
    bio.bi_iter.bi_sector, nr_sectors, maxsector);
    return -EIO;
    }
    return 0;
    }
//
// Remap block n of partition p to block n+start(p) of the disk.
//
#[no_mangle]
unsafe extern "C" fn blk_partition_remap(bio: *mut bio) -> c_int {
    let mut p = bio.bi_bdev;
    if (unlikely(should_fail_request(p, bio.bi_iter.bi_size))) {
    return -EIO;
    }
    if (bio_sectors(bio)) {
    bio.bi_iter.bi_sector += p.bd_start_sect;
    trace_block_bio_remap(bio, p.bd_dev,
    bio.bi_iter.bi_sector -
    p.bd_start_sect);
    }
    bio_set_flag(bio, BIO_REMAPPED);
    return 0;
    }
//
// Check write append to a zoned block device.
//
    static inline blk_status_t blk_check_zone_append(request_queue *q, bio *bio)
    {
pub static mut nr_sectors: c_int = 0;
// Only applicable to zoned block devices
    if (!bdev_is_zoned(bio.bi_bdev)) {
    return BLK_STS_NOTSUPP;
    }
// The bio sector must point to the start of a sequential zone
    if (!bdev_is_zone_start(bio.bi_bdev, bio.bi_iter.bi_sector)) {
    return BLK_STS_IOERR;
    }
//
// Not allowed to cross zone boundaries. Otherwise, the BIO will be
// split and could result in non-contiguous sectors being written in
// different zones.
//
    if (nr_sectors > q.limits.chunk_sectors) {
    return BLK_STS_IOERR;
    }
// Make sure the BIO is small enough and will not get split
    if (nr_sectors > q.limits.max_zone_append_sectors) {
    return BLK_STS_IOERR;
    }
    bio.bi_opf |= REQ_NOMERGE;
    return BLK_STS_OK;
    }
#[no_mangle]
unsafe extern "C" fn __submit_bio(bio: *mut bio) {
    if (!bdev_test_flag(bio.bi_bdev, BD_HAS_SUBMIT_BIO)) {
    blk_mq_submit_bio(bio);
    } else if (likely(bio_queue_enter(bio) == 0)) {
    let mut disk = bio.bi_bdev.bd_disk;
    if ((bio.bi_opf & REQ_POLLED) &&
    !(disk.queue.limits.features & BLK_FEAT_POLL)) {
    bio_endio_status(bio, BLK_STS_NOTSUPP);
    }
    else {
    disk.fops.submit_bio(bio);
    }
    blk_queue_exit(disk.queue);
    }
    }
//
// The loop in this function may be a bit non-obvious, and so deserves some
// explanation:
//
// - Before entering the loop, bio->bi_next is NULL (as all callers ensure
// that), so we have a list with a single bio.
// - We pretend that we have just taken it off a longer list, so we assign
// bio_list to a pointer to the bio_list_on_stack, thus initialising the
// bio_list of new bios to be added.  ->submit_bio() may indeed add some more
// bios through a recursive call to submit_bio_noacct.  If it did, we find a
// non-NULL value in bio_list and re-enter the loop from the top.
// - In this case we really did just take the bio off the top of the list (no
// pretending) and so remove it from bio_list, and call into ->submit_bio()
// again.
//
// bio_list_on_stack[0] contains bios submitted by the current ->submit_bio.
// bio_list_on_stack[1] contains bios that were submitted before the current
// ->submit_bio(), but that haven't been processed yet.
//
#[no_mangle]
unsafe extern "C" fn __submit_bio_noacct(bio: *mut bio) {
    struct bio_list bio_list_on_stack[2];
    BUG_ON!(bio.bi_next);
    bio_list_init(&bio_list_on_stack[0]);
    current.bio_list = bio_list_on_stack;
    do {
    let mut q = bdev_get_queue(bio.bi_bdev);
    struct bio_list lower, same;
//
// Create a fresh bio_list for all subordinate requests.
//
    bio_list_on_stack[1] = bio_list_on_stack[0];
    bio_list_init(&bio_list_on_stack[0]);
    __submit_bio(bio);
//
// Sort new bios into those for a lower level and those for the
// same level.
//
    bio_list_init(&lower);
    bio_list_init(&same);
    while ((bio = bio_list_pop(&bio_list_on_stack[0])) != core::ptr::null_mut()) {
    if (q == bdev_get_queue(bio.bi_bdev))
    bio_list_add(&same, bio);
    }
    else {
    bio_list_add(&lower, bio);
    }
//
// Now assemble so we handle the lowest level first.
//
    bio_list_merge(&bio_list_on_stack[0], &lower);
    bio_list_merge(&bio_list_on_stack[0], &same);
    bio_list_merge(&bio_list_on_stack[0], &bio_list_on_stack[1]);
    } while ((bio = bio_list_pop(&bio_list_on_stack[0])));
    current.bio_list = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn __submit_bio_noacct_mq(bio: *mut bio) {
pub static mut bio_list: usize = 0;
    current.bio_list = bio_list;
    do {
    __submit_bio(bio);
    } while ((bio = bio_list_pop(&bio_list[0])));
    current.bio_list = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn submit_bio_noacct_nocheck(bio: *mut bio, split: bool) {
    if (unlikely(blk_error_inject(bio))) {
    return;
    }
    blk_cgroup_bio_start(bio);
    if (!bio_flagged(bio, BIO_TRACE_COMPLETION)) {
    trace_block_bio_queue(bio);
//
// Now that enqueuing has been traced, we need to trace
// completion as well.
//
    bio_set_flag(bio, BIO_TRACE_COMPLETION);
    }
//
// We only want one ->submit_bio to be active at a time, else stack
// usage with stacked devices could be a problem.  Use current->bio_list
// to collect a list of requests submitted by a ->submit_bio method
// while it is active, and then process them after it returned.
//
    if (current.bio_list) {
    if (split) {
    bio_list_add_head(&current.bio_list[0], bio);
    }
    else {
    bio_list_add(&current.bio_list[0], bio);
    }
    } else if (!bdev_test_flag(bio.bi_bdev, BD_HAS_SUBMIT_BIO)) {
    __submit_bio_noacct_mq(bio);
    } else {
    __submit_bio_noacct(bio);
    }
    }
    static blk_status_t blk_validate_atomic_write_op_size(request_queue *q, bio *bio)
    {
    if (bio.bi_iter.bi_size > queue_atomic_write_unit_max_bytes(q)) {
    return BLK_STS_INVAL;
    }
    if (bio.bi_iter.bi_size % queue_atomic_write_unit_min_bytes(q)) {
    return BLK_STS_INVAL;
    }
    return BLK_STS_OK;
    }
//
// submit_bio_noacct - re-submit a bio to the block device layer for I/O
// @bio:  The bio describing the location in memory and on the device.
//
// This is a version of submit_bio() that shall only be used for I/O that is
// resubmitted to lower level drivers by stacking block drivers.  All file
// systems and other upper level users of the block layer should use
// submit_bio() instead.
//
#[no_mangle]
pub unsafe extern "C" fn submit_bio_noacct(bio: *mut bio) {
    let mut bdev = bio.bi_bdev;
    let mut q = bdev_get_queue(bdev);
pub static mut status: blk_status_t = 0;
    might_sleep();
//
// For a REQ_NOWAIT based request, return -EOPNOTSUPP
// if queue does not support NOWAIT.
//
    if ((bio.bi_opf & REQ_NOWAIT) && !bdev_nowait(bdev)) {
// goto;
    }
    if (bio_has_crypt_ctx(bio)) {
    if (WARN_ON_ONCE!(!bio_has_data(bio))) {
// goto;
    }
    if (!blk_crypto_supported(bio)) {
// goto;
    }
    }
    if (should_fail_bio(bio)) {
// goto;
    }
    bio_check_ro(bio);
    if (!bio_flagged(bio, BIO_REMAPPED)) {
    if (unlikely(bio_check_eod(bio))) {
// goto;
    }
    if (bdev_is_partition(bdev) &&
    unlikely(blk_partition_remap(bio))) {
// goto;
    }
    }
//
// Filter flush bio's early so that bio based drivers without flush
// support don't have to worry about them.
//
    if (op_is_flush(bio.bi_opf)) {
    if (WARN_ON_ONCE!(bio_op(bio) != REQ_OP_WRITE &&
    bio_op(bio) != REQ_OP_ZONE_APPEND)) {
// goto;
    }
    if (!bdev_write_cache(bdev)) {
    bio.bi_opf &= ~(REQ_PREFLUSH | REQ_FUA);
    if (!bio_sectors(bio)) {
    status = BLK_STS_OK;
// goto;
    }
    }
    }
    switch (bio_op(bio)) {
    case REQ_OP_READ:
    break;
    case REQ_OP_WRITE:
    if (bio.bi_opf & REQ_ATOMIC) {
    status = blk_validate_atomic_write_op_size(q, bio);
    if (status != BLK_STS_OK) {
// goto;
    }
    }
    break;
    case REQ_OP_FLUSH:
//
// REQ_OP_FLUSH can't be submitted through bios, it is only
// synthetized in struct request by the flush state machine.
//
// goto;
    case REQ_OP_DISCARD:
    if (!bdev_max_discard_sectors(bdev)) {
// goto;
    }
    break;
    case REQ_OP_SECURE_ERASE:
    if (!bdev_max_secure_erase_sectors(bdev)) {
// goto;
    }
    break;
    case REQ_OP_ZONE_APPEND:
    status = blk_check_zone_append(q, bio);
    if (status != BLK_STS_OK) {
// goto;
    }
    break;
    case REQ_OP_WRITE_ZEROES:
    if (!q.limits.max_write_zeroes_sectors) {
// goto;
    }
    break;
    case REQ_OP_ZONE_OPEN:
    case REQ_OP_ZONE_CLOSE:
    case REQ_OP_ZONE_RESET:
    case REQ_OP_ZONE_FINISH:
// Zone management operations require sequential zones.
    if (!bdev_zone_is_seq(bio.bi_bdev, bio.bi_iter.bi_sector)) {
// goto;
    }
    break;
    case REQ_OP_ZONE_RESET_ALL:
    if (!bdev_is_zoned(bio.bi_bdev)) {
// goto;
    }
    break;
    case REQ_OP_DRV_IN:
    case REQ_OP_DRV_OUT:
//
// Driver private operations are only used with passthrough
// requests.
//
    fallthrough;
// label;
// goto;
    }
    if (blk_throtl_bio(bio)) {
    return;
    }
    submit_bio_noacct_nocheck(bio, false);
    return;
// label;
    status = BLK_STS_NOTSUPP;
// label;
    bio_endio_status(bio, status);
    }
    EXPORT_SYMBOL(submit_bio_noacct);
#[no_mangle]
unsafe extern "C" fn bio_set_ioprio(bio: *mut bio) {
// Nobody set ioprio so far? Initialize it based on task's nice value
    if (IOPRIO_PRIO_CLASS(bio.bi_ioprio) == IOPRIO_CLASS_NONE) {
    bio.bi_ioprio = get_current_ioprio();
    }
    blkcg_set_ioprio(bio);
    }
//
// submit_bio - submit a bio to the block device layer for I/O
// @bio: The &struct bio which describes the I/O
//
// submit_bio() is used to submit I/O requests to block devices.  It is passed a
// fully set up &struct bio that describes the I/O that needs to be done.  The
// bio will be sent to the device described by the bi_bdev field.
//
// The success/failure status of the request, along with notification of
// completion, is delivered asynchronously through the ->bi_end_io() callback
// in @bio.  The bio must NOT be touched by the caller until ->bi_end_io() has
// been called.
//
#[no_mangle]
pub unsafe extern "C" fn submit_bio(bio: *mut bio) {
    if (bio_op(bio) == REQ_OP_READ) {
    task_io_account_read(bio.bi_iter.bi_size);
    count_vm_events(PGPGIN, bio_sectors(bio));
    } else if (bio_op(bio) == REQ_OP_WRITE) {
    count_vm_events(PGPGOUT, bio_sectors(bio));
    }
    bio_set_ioprio(bio);
    submit_bio_noacct(bio);
    }
    EXPORT_SYMBOL(submit_bio);
//
// bio_poll - poll for BIO completions
// @bio: bio to poll for
// @iob: batches of IO
// @flags: BLK_POLL_* flags that control the behavior
//
// Poll for completions on queue associated with the bio. Returns number of
// completed entries found.
//
// Note: the caller must either be the context that submitted @bio, or
// be in a RCU critical section to prevent freeing of @bio.
//
#[no_mangle]
pub unsafe extern "C" fn bio_poll(bio: *mut bio, iob: *mut io_comp_batch, flags: c_uint) -> c_int {
pub static mut cookie: blk_qc_t = 0;
pub static mut bdev: *mut c_void = core::ptr::null_mut();
pub static mut q: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
    bdev = READ_ONCE(bio.bi_bdev);
    if (!bdev) {
    return 0;
    }
    q = bdev_get_queue(bdev);
    if (cookie == BLK_QC_T_NONE) {
    return 0;
    }
    blk_flush_plug(current.plug, false);
//
// We need to be able to enter a frozen queue, similar to how
// timeouts also need to do that. If that is blocked, then we can
// have pending IO when a queue freeze is started, and then the
// wait for the freeze to finish will wait for polled requests to
// timeout as the poller is preventer from entering the queue and
// completing them. As long as we prevent new IO from being queued,
// that should be all that matters.
//
    if (!percpu_ref_tryget(&q.q_usage_counter)) {
    return 0;
    }
    if (queue_is_mq(q)) {
    ret = blk_mq_poll(q, cookie, iob, flags);
    } else {
    let mut disk = q.disk;
    if ((q.limits.features & BLK_FEAT_POLL) && disk &&
    disk.fops.poll_bio) {
    ret = disk.fops.poll_bio(bio, iob, flags);
    }
    }
    blk_queue_exit(q);
    return ret;
    }
    EXPORT_SYMBOL_GPL(bio_poll);
//
// Helper to implement file_operations.iopoll.  Requires the bio to be stored
// in iocb->private, and cleared before freeing the bio.
//
#[no_mangle]
pub unsafe extern "C" fn iocb_bio_iopoll(kiocb: *mut kiocb, iob: *mut io_comp_batch, flags: c_uint) -> c_int {
pub static mut bio: *mut c_void = core::ptr::null_mut();
pub static mut ret: c_int = 0;
//
// Note: the bio cache only uses SLAB_TYPESAFE_BY_RCU, so bio can
// point to a freshly allocated bio at this point.  If that happens
// we have a few cases to consider:
//
// 1) the bio is being initialized and bi_bdev is NULL.  We can just
// simply nothing in this case
// 2) the bio points to a not poll enabled device.  bio_poll will catch
// this and return 0
// 3) the bio points to a poll capable device, including but not
// limited to the one that the original bio pointed to.  In this
// case we will call into the actual poll method and poll for I/O,
// even if we don't need to, but it won't cause harm either.
//
// For cases 2) and 3) above the RCU grace period ensures that bi_bdev
// is still allocated. Because partitions hold a reference to the whole
// device bdev and thus disk, the disk is also still valid.  Grabbing
// a reference to the queue in bio_poll() ensures the hctxs and requests
// are still valid as well.
//
    rcu_read_lock();
    bio = READ_ONCE(kiocb.private);
    if (bio) {
    ret = bio_poll(bio, iob, flags);
    }
    rcu_read_unlock();
    return ret;
    }
    EXPORT_SYMBOL_GPL(iocb_bio_iopoll);
#[no_mangle]
pub unsafe extern "C" fn update_io_ticks(part: *mut block_device, now: c_ulong, end: bool) {
    let mut stamp = 0;
// label;
    stamp = READ_ONCE(part.bd_stamp);
    if (unlikely(time_after(now, stamp)) &&
    likely(try_cmpxchg(&part.bd_stamp, &stamp, now)) &&
    (end || bdev_count_inflight(part))) {
    __part_stat_add(part, io_ticks, now - stamp);
    }
    if (bdev_is_partition(part)) {
    part = bdev_whole(part);
// goto;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn bdev_start_io_acct(bdev: *mut block_device, op: req_op, start_time: c_ulong) -> c_ulong {
    part_stat_lock();
    update_io_ticks(bdev, start_time, false);
    bdev_inc_in_flight(bdev, op);
    part_stat_unlock();
    return start_time;
    }
    EXPORT_SYMBOL(bdev_start_io_acct);
//
// bio_start_io_acct - start I/O accounting for bio based drivers
// @bio:	bio to start account for
//
// Returns the start time that should be passed back to bio_end_io_acct().
//
#[no_mangle]
pub unsafe extern "C" fn bio_start_io_acct(bio: *mut bio) -> c_ulong {
    return bdev_start_io_acct(bio.bi_bdev, bio_op(bio), jiffies);
    }
    EXPORT_SYMBOL_GPL(bio_start_io_acct);
#[no_mangle]
pub unsafe extern "C" fn bdev_end_io_acct(bdev: *mut block_device, op: req_op, sectors: c_uint, start_time: c_ulong) {
pub static mut sgrp: c_int = 0;
pub static mut now: c_ulong = 0;
pub static mut duration: c_ulong = 0;
    part_stat_lock();
    update_io_ticks(bdev, now, true);
    part_stat_inc(bdev, ios[sgrp]);
    part_stat_add(bdev, sectors[sgrp], sectors);
    part_stat_add(bdev, nsecs[sgrp], jiffies_to_nsecs(duration));
    bdev_dec_in_flight(bdev, op);
    part_stat_unlock();
    }
    EXPORT_SYMBOL(bdev_end_io_acct);
#[no_mangle]
pub unsafe extern "C" fn bio_end_io_acct_remapped(bio: *mut bio, start_time: c_ulong, orig_bdev: *mut block_device) {
    bdev_end_io_acct(orig_bdev, bio_op(bio), bio_sectors(bio), start_time);
    }
    EXPORT_SYMBOL_GPL(bio_end_io_acct_remapped);
//
// blk_lld_busy - Check if underlying low-level drivers of a device are busy
// @q : the queue of the device being checked
//
// Description:
// Check if underlying low-level drivers of a device are busy.
// If the drivers want to export their busy state, they must set own
// exporting function using blk_queue_lld_busy() first.
//
// Basically, this function is used only by request stacking drivers
// to stop dispatching requests to underlying devices when underlying
// devices are busy.  This behavior helps more I/O merging on the queue
// of the request stacking driver and prevents I/O throughput regression
// on burst I/O load.
//
// Return:
// 0 - Not busy (The request stacking driver should dispatch request)
// 1 - Busy (The request stacking driver should stop dispatching request)
//
#[no_mangle]
pub unsafe extern "C" fn blk_lld_busy(q: *mut request_queue) -> c_int {
    if (queue_is_mq(q) && q.mq_ops.busy) {
    return q.mq_ops.busy(q);
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(blk_lld_busy);
#[no_mangle]
pub unsafe extern "C" fn kblockd_schedule_work(work: *mut work_struct) -> c_int {
    return queue_work(kblockd_workqueue, work);
    }
    EXPORT_SYMBOL(kblockd_schedule_work);
#[no_mangle]
pub unsafe extern "C" fn kblockd_mod_delayed_work_on(cpu: c_int, dwork: *mut delayed_work, delay: c_ulong) -> c_int {
    return mod_delayed_work_on(cpu, kblockd_workqueue, dwork, delay);
    }
    EXPORT_SYMBOL(kblockd_mod_delayed_work_on);
#[no_mangle]
pub unsafe extern "C" fn blk_start_plug_nr_ios(plug: *mut blk_plug, nr_ios: c_ushort) {
    let mut tsk = current;
//
// If this is a nested plug, don't actually assign it.
//
    if (tsk.plug) {
    return;
    }
    plug.cur_ktime = 0;
    rq_list_init(&plug.mq_list);
    rq_list_init(&plug.cached_rqs);
    plug.nr_ios = min_t(unsigned short, nr_ios, BLK_MAX_REQUEST_COUNT);
    plug.rq_count = 0;
    plug.multiple_queues = false;
    plug.has_elevator = false;
    INIT_LIST_HEAD(&plug.cb_list);
//
// Store ordering should not be needed here, since a potential
// preempt will imply a full memory barrier
//
    tsk.plug = plug;
    }
//
// blk_start_plug - initialize blk_plug and track it inside the task_struct
// @plug:	The &struct blk_plug that needs to be initialized
//
// Description:
// blk_start_plug() indicates to the block layer an intent by the caller
// to submit multiple I/O requests in a batch.  The block layer may use
// this hint to defer submitting I/Os from the caller until blk_finish_plug()
// is called.  However, the block layer may choose to submit requests
// before a call to blk_finish_plug() if the number of queued I/Os
// exceeds %BLK_MAX_REQUEST_COUNT, or if the size of the I/O is larger than
// %BLK_PLUG_FLUSH_SIZE.  The queued I/Os may also be submitted early if
// the task schedules (see below).
//
// Tracking blk_plug inside the task_struct will help with auto-flushing the
// pending I/O should the task end up blocking between blk_start_plug() and
// blk_finish_plug(). This is important from a performance perspective, but
// also ensures that we don't deadlock. For instance, if the task is blocking
// for a memory allocation, memory reclaim could end up wanting to free a
// page belonging to that request that is currently residing in our private
// plug. By flushing the pending I/O when the process goes to sleep, we avoid
// this kind of deadlock.
//
#[no_mangle]
pub unsafe extern "C" fn blk_start_plug(plug: *mut blk_plug) {
    blk_start_plug_nr_ios(plug, 1);
    }
    EXPORT_SYMBOL(blk_start_plug);
#[no_mangle]
unsafe extern "C" fn flush_plug_callbacks(plug: *mut blk_plug, from_schedule: bool) {
pub static mut callbacks: usize = 0;
    while (!list_empty(&plug.cb_list)) {
    list_splice_init(&plug.cb_list, &callbacks);
    while (!list_empty(&callbacks)) {
    let mut cb = list_first_entry(&callbacks, blk_plug_cb,
    list);
    list_del(&cb.list);
    cb.callback(cb, from_schedule);
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_check_plugged(unplug: blk_plug_cb_fn, data: *mut c_void, size: c_int) -> *mut c_void {
    let mut plug = current.plug;
pub static mut cb: *mut c_void = core::ptr::null_mut();
    if (!plug) {
    return core::ptr::null_mut();
    }
    list_for_each_entry(cb, &plug.cb_list, list) {
    if (cb.callback == unplug && cb.data == data)
    return cb;
    }
// Not currently on the callback list
    BUG_ON!(size < sizeof!(*cb));
    cb = kzalloc(size, GFP_ATOMIC);
    if (cb) {
    cb.data = data;
    cb.callback = unplug;
    list_add(&cb.list, &plug.cb_list);
    }
    return cb;
    }
    EXPORT_SYMBOL(blk_check_plugged);
#[no_mangle]
pub unsafe extern "C" fn __blk_flush_plug(plug: *mut blk_plug, from_schedule: bool) {
    if (!list_empty(&plug.cb_list)) {
    flush_plug_callbacks(plug, from_schedule);
    }
    blk_mq_flush_plug_list(plug, from_schedule);
//
// Unconditionally flush out cached requests, even if the unplug
// event came from schedule. Since we know hold references to the
// queue for cached requests, we don't want a blocked task holding
// up a queue freeze/quiesce event.
//
    if (unlikely(!rq_list_empty(&plug.cached_rqs))) {
    blk_mq_free_plug_rqs(plug);
    }
    plug.cur_ktime = 0;
    current.flags &= ~PF_BLOCK_TS;
    }
//
// blk_finish_plug - mark the end of a batch of submitted I/O
// @plug:	The &struct blk_plug passed to blk_start_plug()
//
// Description:
// Indicate that a batch of I/O submissions is complete.  This function
// must be paired with an initial call to blk_start_plug().  The intent
// is to allow the block layer to optimize I/O submission.  See the
// documentation for blk_start_plug() for more information.
//
#[no_mangle]
pub unsafe extern "C" fn blk_finish_plug(plug: *mut blk_plug) {
    if (plug == current.plug) {
    __blk_flush_plug(plug, false);
    current.plug = core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL(blk_finish_plug);
#[no_mangle]
pub unsafe extern "C" fn blk_io_schedule() {
// Prevent hang_check timer from firing at us during very long I/O
pub static mut timeout: c_ulong = 0;
    if (timeout) {
    io_schedule_timeout(timeout);
    }
    else {
    io_schedule();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn blk_dev_init() -> c_int {
    BUILD_BUG_ON!(( u32)REQ_OP_LAST >= (1 << REQ_OP_BITS));
    BUILD_BUG_ON!(REQ_OP_BITS + REQ_FLAG_BITS > 8 *
    sizeof_field(request, cmd_flags));
    BUILD_BUG_ON!(REQ_OP_BITS + REQ_FLAG_BITS > 8 *
    sizeof_field(bio, bi_opf));
// used for unplugging and affects IO latency/throughput - HIGHPRI
    kblockd_workqueue = alloc_workqueue("kblockd",
    WQ_MEM_RECLAIM | WQ_HIGHPRI | WQ_PERCPU, 0);
    if (!kblockd_workqueue) {
    panic("Failed to create kblockd\n");
    }
    blk_requestq_cachep = KMEM_CACHE(request_queue, SLAB_PANIC);
    blk_debugfs_root = debugfs_create_dir("block", core::ptr::null_mut());
    return 0;
    }