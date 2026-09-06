//! Automatically rewritten from C to Rust
//! Source: block/blk-flush.c
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
// Functions to sequence PREFLUSH and FUA writes.
//
// Copyright (C) 2011		Max Planck Institute for Gravitational Physics
// Copyright (C) 2011		Tejun Heo <tj@kernel.org>
//
// REQ_{PREFLUSH|FUA} requests are decomposed to sequences consisted of three
// optional steps - PREFLUSH, DATA and POSTFLUSH - according to the request
// properties and hardware capability.
//
// If a request doesn't have data, only REQ_PREFLUSH makes sense, which
// indicates a simple flush request.  If there is data, REQ_PREFLUSH indicates
// that the device cache should be flushed before the data is executed, and
// REQ_FUA means that the data must be on non-volatile media on request
// completion.
//
// If the device doesn't have writeback cache, PREFLUSH and FUA don't make any
// difference.  The requests are either completed immediately if there's no data
// or executed as normal requests otherwise.
//
// If the device has writeback cache and supports FUA, REQ_PREFLUSH is
// translated to PREFLUSH but REQ_FUA is passed down directly with DATA.
//
// If the device has writeback cache and doesn't support FUA, REQ_PREFLUSH
// is translated to PREFLUSH and REQ_FUA to POSTFLUSH.
//
// The actual execution of flush is double buffered.  Whenever a request
// needs to execute PRE or POSTFLUSH, it queues at
// fq->flush_queue[fq->flush_pending_idx].  Once certain criteria are met, a
// REQ_OP_FLUSH is issued and the pending_idx is toggled.  When the flush
// completes, all the requests which were pending are proceeded to the next
// step.  This allows arbitrary merging of different types of PREFLUSH/FUA
// requests.
//
// Currently, the following conditions are used to determine when to issue
// flush.
//
// C1. At any given time, only one flush shall be in progress.  This makes
// double buffering sufficient.
//
// C2. Flush is deferred if any request is executing DATA of its sequence.
// This avoids issuing separate POSTFLUSHes for requests which shared
// PREFLUSH.
//
// C3. The second condition is ignored if there is a request which has
// waited longer than FLUSH_PENDING_TIMEOUT.  This is to avoid
// starvation in the unlikely case where there are continuous stream of
// FUA (without PREFLUSH) requests.
//
// For devices which support FUA, it isn't clear whether C2 (and thus C3)
// is beneficial.
//
// Note that a sequenced PREFLUSH/FUA request with DATA is completed twice.
// Once while executing DATA and again after the whole sequence is
// complete.  The first completion updates the contained bio but doesn't
// finish it so that the bio submitter is notified only after the whole
// sequence is complete.  This is implemented by testing RQF_FLUSH_SEQ in
// req_bio_endio().
//
// The above peculiarity requires that each PREFLUSH/FUA request has only one
// bio attached to it, which is guaranteed as they aren't allowed to be
// merged in the usual way.
//

// PREFLUSH/FUA sequences
    enum {
    REQ_FSEQ_PREFLUSH	= (1 << 0), /* pre-flushing in progress */
    REQ_FSEQ_DATA		= (1 << 1), /* data write in progress */
    REQ_FSEQ_POSTFLUSH	= (1 << 2), /* post-flushing in progress */
    REQ_FSEQ_DONE		= (1 << 3),
    REQ_FSEQ_ACTIONS	= REQ_FSEQ_PREFLUSH | REQ_FSEQ_DATA |
    REQ_FSEQ_POSTFLUSH,
//
// If flush has been pending longer than the following timeout,
// it's issued even if flush_data requests are still in flight.
//
    FLUSH_PENDING_TIMEOUT	= 5 * HZ,
    };
// forward_decl: blk_kick_flush;
#[no_mangle]
pub unsafe extern "C" fn blk_get_flush_queue(ctx: *mut blk_mq_ctx) -> *mut c_void {
    return blk_mq_map_queue(REQ_OP_FLUSH, ctx).fq;
    }
#[no_mangle]
unsafe extern "C" fn blk_flush_cur_seq(rq: *mut request) -> c_uint {
    return 1 << ffz(rq.flush.seq);
    }
#[no_mangle]
unsafe extern "C" fn blk_flush_restore_request(rq: *mut request) {
//
// After flush data completion, @rq->bio is %NULL but we need to
// complete the bio again.  @rq->biotail is guaranteed to equal the
// original @rq->bio.  Restore it.
//
    rq.bio = rq.biotail;
    if (rq.bio) {
    rq.__sector = rq.bio.bi_iter.bi_sector;
    }
// make @rq a normal request
    rq.rq_flags &= ~RQF_FLUSH_SEQ;
    rq.end_io = rq.flush.saved_end_io;
    }
#[no_mangle]
unsafe extern "C" fn blk_account_io_flush(rq: *mut request) {
    let mut part = rq.q.disk.part0;
    part_stat_lock();
    part_stat_inc(part, ios[STAT_FLUSH]);
    part_stat_add(part, nsecs[STAT_FLUSH],
    blk_time_get_ns() - rq.start_time_ns);
    part_stat_unlock();
    }
//
// blk_flush_complete_seq - complete flush sequence
// @rq: PREFLUSH/FUA request being sequenced
// @fq: flush queue
// @seq: sequences to complete (mask of %REQ_FSEQ_*, can be zero)
// @error: whether an error occurred
//
// @rq just completed @seq part of its flush sequence, record the
// completion and trigger the next step.
//
// CONTEXT:
// spin_lock_irq(fq->mq_flush_lock)
//
#[no_mangle]
pub unsafe extern "C" fn blk_flush_complete_seq(rq: *mut request, fq: *mut blk_flush_queue, seq: c_uint, error: blk_status_t) {
    let mut q = rq.q;
    let mut pending = &fq.flush_queue[fq.flush_pending_idx];
    let mut cmd_flags;
    BUG_ON!(rq.flush.seq & seq);
    rq.flush.seq |= seq;
    cmd_flags = rq.cmd_flags;
    if (likely(!error)) {
    seq = blk_flush_cur_seq(rq);
    }
    else {
    seq = REQ_FSEQ_DONE;
    }
    match (seq) {
    REQ_FSEQ_PREFLUSH => {
    }
    REQ_FSEQ_POSTFLUSH => {
// queue for flush
    if (list_empty(pending)) {
    fq.flush_pending_since = jiffies;
    }
    list_add_tail(&rq.queuelist, pending);
    // break;
    }
    REQ_FSEQ_DATA => {
    fq.flush_data_in_flight += 1;
    spin_lock(&q.requeue_lock);
    list_move(&rq.queuelist, &q.requeue_list);
    spin_unlock(&q.requeue_lock);
    blk_mq_kick_requeue_list(q);
    // break;
    }
    REQ_FSEQ_DONE => {
//
// @rq was previously adjusted by blk_insert_flush() for
// flush sequencing and may already have gone through the
// flush data request completion path.  Restore @rq for
// normal completion and end it.
//
    list_del_init(&rq.queuelist);
    blk_flush_restore_request(rq);
    blk_mq_end_request(rq, error);
    // break;
    }
    _ => {
    BUG();
    }
    }
    blk_kick_flush(q, fq, cmd_flags);
    }
    static enum rq_end_io_ret flush_end_io(request *flush_rq,
    blk_status_t error,
    const struct io_comp_batch *iob)
    {
    let mut q = flush_rq.q;
pub static mut running: *mut c_void = core::ptr::null_mut();
    let mut rq = core::ptr::null_mut();
    let mut n = core::ptr::null_mut();
pub static mut flags: c_ulong = 0;
    let mut fq = blk_get_flush_queue(flush_rq.mq_ctx);
// release the tag's ownership to the req cloned from
    spin_lock_irqsave(&fq.mq_flush_lock, flags);
    if (!req_ref_put_and_test(flush_rq)) {
    fq.rq_status = error;
    spin_unlock_irqrestore(&fq.mq_flush_lock, flags);
    return RQ_END_IO_NONE;
    }
    blk_account_io_flush(flush_rq);
//
// Flush request has to be marked as IDLE when it is really ended
// because its .end_io() is called from timeout code path too for
// avoiding use-after-free.
//
    WRITE_ONCE(flush_rq.state, MQ_RQ_IDLE);
    if (fq.rq_status != BLK_STS_OK) {
    error = fq.rq_status;
    fq.rq_status = BLK_STS_OK;
    }
    if (!q.elevator) {
    flush_rq.tag = BLK_MQ_NO_TAG;
    } else {
    blk_mq_put_driver_tag(flush_rq);
    flush_rq.internal_tag = BLK_MQ_NO_TAG;
    }
    running = &fq.flush_queue[fq.flush_running_idx];
    BUG_ON!(fq.flush_pending_idx == fq.flush_running_idx);
// account completion of the flush request
    fq.flush_running_idx ^= 1;
// and push the waiting requests to the next stage
    list_for_each_entry_safe(rq, n, running, queuelist) {
pub static mut seq: c_uint = 0;
    BUG_ON!(seq != REQ_FSEQ_PREFLUSH && seq != REQ_FSEQ_POSTFLUSH);
    list_del_init(&rq.queuelist);
    blk_flush_complete_seq(rq, fq, seq, error);
    }
    spin_unlock_irqrestore(&fq.mq_flush_lock, flags);
    return RQ_END_IO_NONE;
    }
#[no_mangle]
pub unsafe extern "C" fn is_flush_rq(rq: *mut request) -> bool {
    return rq.end_io == flush_end_io;
    }
//
// blk_kick_flush - consider issuing flush request
// @q: request_queue being kicked
// @fq: flush queue
// @flags: cmd_flags of the original request
//
// Flush related states of @q have changed, consider issuing flush request.
// Please read the comment at the top of this file for more info.
//
// CONTEXT:
// spin_lock_irq(fq->mq_flush_lock)
//
#[no_mangle]
pub unsafe extern "C" fn blk_kick_flush(q: *mut request_queue, fq: *mut blk_flush_queue, flags: blk_opf_t) {
    let mut pending = &fq.flush_queue[fq.flush_pending_idx];
    let mut first_rq = list_first_entry(pending, request, queuelist);
    let mut flush_rq = fq.flush_rq;
// C1 described at the top of this file
    if (fq.flush_pending_idx != fq.flush_running_idx || list_empty(pending)) {
    return;
    }
// C2 and C3
    if (fq.flush_data_in_flight &&
    time_before(jiffies,
    fq.flush_pending_since + FLUSH_PENDING_TIMEOUT)) {
    return;
    }
//
// Issue flush and toggle pending_idx.  This makes pending_idx
// different from running_idx, which means flush is in flight.
//
    fq.flush_pending_idx ^= 1;
    blk_rq_init(q, flush_rq);
//
// In case of none scheduler, borrow tag from the first request
// since they can't be in flight at the same time. And acquire
// the tag's ownership for flush req.
//
// In case of IO scheduler, flush rq need to borrow scheduler tag
// just for cheating put/get driver tag.
//
    flush_rq.mq_ctx = first_rq.mq_ctx;
    flush_rq.mq_hctx = first_rq.mq_hctx;
    if (!q.elevator) {
    flush_rq.tag = first_rq.tag;
    }
    else {
    flush_rq.internal_tag = first_rq.internal_tag;
    }
    flush_rq.cmd_flags = REQ_OP_FLUSH | REQ_PREFLUSH;
    flush_rq.cmd_flags |= (flags & REQ_DRV) | (flags & REQ_FAILFAST_MASK);
    flush_rq.rq_flags |= RQF_FLUSH_SEQ;
    flush_rq.end_io = flush_end_io;
//
// Order WRITE ->end_io and WRITE rq->ref, and its pair is the one
// implied in refcount_inc_not_zero() called from
// blk_mq_find_and_get_req(), which orders WRITE/READ flush_rq->ref
// and READ flush_rq->end_io
//
    smp_wmb();
    req_ref_set(flush_rq, 1);
    spin_lock(&q.requeue_lock);
    list_add_tail(&flush_rq.queuelist, &q.flush_list);
    spin_unlock(&q.requeue_lock);
    blk_mq_kick_requeue_list(q);
    }
    static enum rq_end_io_ret mq_flush_data_end_io(request *rq,
    blk_status_t error,
    const struct io_comp_batch *iob)
    {
    let mut q = rq.q;
    let mut hctx = rq.mq_hctx;
    let mut ctx = rq.mq_ctx;
    let mut flags = 0;
    let mut fq = blk_get_flush_queue(ctx);
    if (q.elevator) {
    WARN_ON!(rq.tag < 0);
    blk_mq_put_driver_tag(rq);
    }
//
// After populating an empty queue, kick it to avoid stall.  Read
// the comment in flush_end_io().
//
    spin_lock_irqsave(&fq.mq_flush_lock, flags);
    fq.flush_data_in_flight -= 1;
//
// May have been corrupted by rq->rq_next reuse, we need to
// re-initialize rq->queuelist before reusing it here.
//
    INIT_LIST_HEAD(&rq.queuelist);
    blk_flush_complete_seq(rq, fq, REQ_FSEQ_DATA, error);
    spin_unlock_irqrestore(&fq.mq_flush_lock, flags);
    blk_mq_sched_restart(hctx);
    return RQ_END_IO_NONE;
    }
#[no_mangle]
unsafe extern "C" fn blk_rq_init_flush(rq: *mut request) {
    rq.flush.seq = 0;
    rq.rq_flags |= RQF_FLUSH_SEQ;
    rq.flush.saved_end_io = rq.end_io; /* Usually core::ptr::null_mut() */
    rq.end_io = mq_flush_data_end_io;
    }
//
// Insert a PREFLUSH/FUA request into the flush state machine.
// Returns true if the request has been consumed by the flush state machine,
// or false if the caller should continue to process it.
//
#[no_mangle]
pub unsafe extern "C" fn blk_insert_flush(rq: *mut request) -> bool {
    let mut q = rq.q;
    let mut fq = blk_get_flush_queue(rq.mq_ctx);
pub static mut supports_fua: bool = false;
pub static mut policy: c_uint = 0;
// FLUSH/FUA request must never be merged
    WARN_ON_ONCE!(rq.bio != rq.biotail);
    if (blk_rq_sectors(rq)) {
    policy |= REQ_FSEQ_DATA;
    }
//
// Check which flushes we need to sequence for this operation.
//
    if (blk_queue_write_cache(q)) {
    if (rq.cmd_flags & REQ_PREFLUSH) {
    policy |= REQ_FSEQ_PREFLUSH;
    }
    if ((rq.cmd_flags & REQ_FUA) && !supports_fua) {
    policy |= REQ_FSEQ_POSTFLUSH;
    }
    }
//
// @policy now records what operations need to be done.  Adjust
// REQ_PREFLUSH and FUA for the driver.
//
    rq.cmd_flags &= ~REQ_PREFLUSH;
    if (!supports_fua) {
    rq.cmd_flags &= ~REQ_FUA;
    }
//
// REQ_PREFLUSH|REQ_FUA implies REQ_SYNC, so if we clear any
// of those flags, we have to set REQ_SYNC to avoid skewing
// the request accounting.
//
    rq.cmd_flags |= REQ_SYNC;
    match (policy) {
    0 => {
//
// An empty flush handed down from a stacking driver may
// translate into nothing if the underlying device does not
// advertise a write-back cache.  In this case, simply
// complete the request.
//
    blk_mq_end_request(rq, 0);
    return true;
    }
    REQ_FSEQ_DATA => {
//
// If there's data, but no flush is necessary, the request can
// be processed directly without going through flush machinery.
// Queue for normal execution.
//
    return false;
    }
    REQ_FSEQ_DATA | REQ_FSEQ_POSTFLUSH => {
//
// Initialize the flush fields and completion handler to trigger
// the post flush, and then just pass the command on.
//
    blk_rq_init_flush(rq);
    rq.flush.seq |= REQ_FSEQ_PREFLUSH;
    spin_lock_irq(&fq.mq_flush_lock);
    fq.flush_data_in_flight += 1;
    spin_unlock_irq(&fq.mq_flush_lock);
    return false;
    }
    _ => {
//
// Mark the request as part of a flush sequence and submit it
// for further processing to the flush state machine.
//
    blk_rq_init_flush(rq);
    spin_lock_irq(&fq.mq_flush_lock);
    blk_flush_complete_seq(rq, fq, REQ_FSEQ_ACTIONS & ~policy, 0);
    spin_unlock_irq(&fq.mq_flush_lock);
    return true;
    }
    }
    }
//
// blkdev_issue_flush - queue a flush
// @bdev:	blockdev to issue flush for
//
// Description:
// Issue a flush for the block device in question.
//
#[no_mangle]
pub unsafe extern "C" fn blkdev_issue_flush(bdev: *mut block_device) -> c_int {
pub static mut bio: usize = 0;
    bio_init(&bio, bdev, core::ptr::null_mut(), 0, REQ_OP_WRITE | REQ_PREFLUSH);
    return submit_bio_wait(&bio);
    }
    EXPORT_SYMBOL(blkdev_issue_flush);
#[no_mangle]
pub unsafe extern "C" fn blk_alloc_flush_queue(node: c_int, cmd_size: c_int, flags: gfp_t) -> *mut c_void {
pub static mut fq: *mut c_void = core::ptr::null_mut();
pub static mut rq_sz: c_int = 0;
    fq = kzalloc_node(sizeof!(*fq), flags, node);
    if (!fq) {
// goto;
    }
    spin_lock_init(&fq.mq_flush_lock);
    rq_sz = round_up(rq_sz + cmd_size, cache_line_size());
    fq.flush_rq = kzalloc_node(rq_sz, flags, node);
    if (!fq.flush_rq) {
// goto;
    }
    INIT_LIST_HEAD(&fq.flush_queue[0]);
    INIT_LIST_HEAD(&fq.flush_queue[1]);
    return fq;
// label;
    kfree(fq);
// label;
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn blk_free_flush_queue(fq: *mut blk_flush_queue) {
// bio based request queue hasn't flush queue
    if (!fq) {
    return;
    }
    kfree(fq.flush_rq);
    kfree(fq);
    }
//
// Allow driver to set its own lock class to fq->mq_flush_lock for
// avoiding lockdep complaint.
//
// flush_end_io() may be called recursively from some driver, such as
// nvme-loop, so lockdep may complain 'possible recursive locking' because
// all 'struct blk_flush_queue' instance share same mq_flush_lock lock class
// key. We need to assign different lock class for these driver's
// fq->mq_flush_lock for avoiding the lockdep warning.
//
// Use dynamically allocated lock class key for each 'blk_flush_queue'
// instance is over-kill, and more worse it introduces horrible boot delay
// issue because synchronize_rcu() is implied in lockdep_unregister_key which
// is called for each hctx release. SCSI probing may synchronously create and
// destroy lots of MQ request_queues for non-existent devices, and some robot
// test kernel always enable lockdep option. It is observed that more than half
// an hour is taken during SCSI MQ probe with per-fq lock class.
//
#[no_mangle]
pub unsafe extern "C" fn blk_mq_hctx_set_fq_lock_class(hctx: *mut blk_mq_hw_ctx, key: *mut lock_class_key) {
    lockdep_set_class(&hctx.fq.mq_flush_lock, key);
    }
    EXPORT_SYMBOL_GPL(blk_mq_hctx_set_fq_lock_class);