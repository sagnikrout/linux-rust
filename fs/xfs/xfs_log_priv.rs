//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_log_priv.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2000-2003,2005 Silicon Graphics, Inc.
// All Rights Reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_log_iovec {
    pub /: *mut *mut *mut void i_addr;/ beginning address of region,
    pub /: *mut *mut int i_len; / length in bytes of region,
    pub /: *mut *mut uint i_type; / type of region,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_log_vec {
    pub /: *mut *mut list_head lv_list; / CIL lv chain ptrs,
    pub /: *mut *mut uint32_t lv_order_id; / chain ordering info,
    pub /: *mut *mut int lv_niovecs; / number of iovecs in lv,
    pub /: *mut *mut *mut xfs_log_iovec lv_iovecp; / iovec array,
    pub /: *mut *mut *mut xfs_log_item lv_item; / owner,
    pub /: *mut *mut *mut char lv_buf; / formatted buffer,
    pub /: *mut *mut int lv_bytes; / accounted space in buffer,
    pub /: *mut *mut int lv_buf_used; / buffer space used so far,
    pub /: *mut *mut int lv_alloc_size; / size of allocated lv,
}

//
// get client id from packed copy.
//
// this hack is here because the xlog_pack code copies four bytes
// of xlog_op_header containing the fields oh_clientid, oh_flags
// and oh_res2 into the packed copy.
//
// later on this four byte chunk is treated as an int and the
// client id is pulled out.
//
// this has endian issues, of course.
//
// In core log state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xlog_iclog_state {
    XLOG_STATE_ACTIVE,	/* Current IC log being written to */
    XLOG_STATE_WANT_SYNC,	/* Want to sync this iclog; no more writes */
    XLOG_STATE_SYNCING,	/* This IC log is syncing */
    XLOG_STATE_DONE_SYNC,	/* Done syncing to disk */
    XLOG_STATE_CALLBACK,	/* Callback functions now */
    XLOG_STATE_DIRTY,	/* Dirty IC log, not ready for ACTIVE status */
}

//
// In core log flags
//

//
// Log ticket flags
//

//
// Below are states for covering allocation transactions.
// By covering, we mean changing the h_tail_lsn in the last on-disk
// log write such that no allocation transactions will be re-done during
// recovery after a system crash. Recovery starts at the last on-disk
// log write.
//
// These states are used to insert dummy log entries to cover
// space allocation transactions which can undo non-transactional changes
// after a crash. Writes to a file with space
// already allocated do not result in any transactions. Allocations
// might include space beyond the EOF. So if we just push the EOF a
// little, the last transaction for the file could contain the wrong
// size. If there is no file system activity, after an allocation
// transaction, and the system crashes, the allocation transaction
// will get replayed and the file will be truncated. This could
// be hours/days/... after the allocation occurred.
//
// The fix for this is to do two dummy transactions when the
// system is idle. We need two dummy transaction because the h_tail_lsn
// in the log record header needs to point beyond the last possible
// non-dummy transaction. The first dummy changes the h_tail_lsn to
// the first transaction before the dummy. The second dummy causes
// h_tail_lsn to point to the first dummy. Recovery starts at h_tail_lsn.
//
// These dummy transactions get committed when everything
// is idle (after there has been some activity).
//
// There are 5 states used to control this.
//
// IDLE -- no logging has been done on the file system or
// we are done covering previous transactions.
// NEED -- logging has occurred and we need a dummy transaction
// when the log becomes idle.
// DONE -- we were in the NEED state and have committed a dummy
// transaction.
// NEED2 -- we detected that a dummy transaction has gone to the
// on disk log with no other transactions.
// DONE2 -- we committed a dummy transaction when in the NEED2 state.
//
// There are two places where we switch states:
//
// 1.) In xfs_sync, when we detect an idle log and are in NEED or NEED2.
// We commit the dummy transaction and switch to DONE or DONE2,
// respectively. In all other states, we don't do anything.
//
// 2.) When we finish writing the on-disk log (xlog_state_clean_log).
//
// No matter what state we are in, if this isn't the dummy
// transaction going out, the next state is NEED.
// So, if we aren't in the DONE or DONE2 states, the next state
// is NEED. We can't be finishing a write of the dummy record
// unless it was committed and the state switched to DONE or DONE2.
//
// If we are in the DONE state and this was a write of the
// dummy transaction, we move to NEED2.
//
// If we are in the DONE2 state and this was a write of the
// dummy transaction, we move to IDLE.
//
// Writing only one dummy transaction can get appended to
// one file space allocation. When this happens, the log recovery
// code replays the space allocation and a file could be truncated.
// This is why we have the NEED2 and DONE2 states before going idle.
//
pub const XLOG_STATE_COVER_IDLE: c_int = 0;
pub const XLOG_STATE_COVER_NEED: c_int = 1;
pub const XLOG_STATE_COVER_DONE: c_int = 2;
pub const XLOG_STATE_COVER_NEED2: c_int = 3;
pub const XLOG_STATE_COVER_DONE2: c_int = 4;
pub const XLOG_COVER_OPS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_ticket {
    pub /: *mut *mut list_head t_queue; / reserve/write queue,
    pub /: *mut *mut *mut task_t_task; / task that owns this ticket,
    pub /: *mut *mut xlog_tid_t t_tid; / transaction identifier,
    pub /: *mut *mut atomic_t t_ref; / ticket reference count,
    pub /: *mut *mut int t_curr_res; / current reservation,
    pub /: *mut *mut int t_unit_res; / unit reservation,
    pub /: *mut *mut char t_ocnt; / original unit count,
    pub /: *mut *mut char t_cnt; / current unit count,
    pub /: *mut *mut uint8_t t_flags; / properties of reservation,
    pub /: *mut *mut int t_iclog_hdrs; / iclog hdrs in t_curr_res,
}

//
// In-core log structure.
//
// - ic_forcewait is used to implement synchronous forcing of the iclog to disk.
// - ic_next is the pointer to the next iclog in the ring.
// - ic_log is a pointer back to the global log structure.
// - ic_size is the full size of the log buffer, minus the cycle headers.
// - ic_offset is the current number of bytes written to in this iclog.
// - ic_refcnt is bumped when someone is writing to the log.
// - ic_state is the state of the iclog.
//
// Because of cacheline contention on large machines, we need to separate
// various resources onto different cachelines. To start with, make the
// structure cacheline aligned. The following fields can be contended on
// by independent processes:
//
// - ic_callbacks
// - ic_refcnt
// - fields protected by the global l_icloglock
//
// so we need to ensure that these fields are located in separate cachelines.
// We'll put all the read-only and l_icloglock fields in the first cacheline,
// and move everything else out to subsequent cachelines.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_in_core {
    pub ic_force_wait: wait_queue_head_t,
    pub ic_write_wait: wait_queue_head_t,
    pub ic_next: *mut xlog_in_core,
    pub ic_prev: *mut xlog_in_core,
    pub ic_log: *mut xlog,
    pub ic_size: u32,
    pub ic_offset: u32,
    pub ic_state: xlog_iclog_state,
    pub ic_flags: c_uint,
    pub /: *mut *mut *mut void ic_datap; / pointer to iclog data,
    pub ic_callbacks: list_head,
// reference counts need their own cacheline
    pub ____cacheline_aligned_in_smp: atomic_t ic_refcnt,
    pub ic_header: *mut xlog_rec_header,

    pub 1: bool ic_fail_crc :,

    pub ic_sema: semaphore,
    pub ic_end_io_work: work_struct,
    pub ic_bio: bio,
    pub ic_bvec: [bio_vec; ],
}

//
// The CIL context is used to aggregate per-transaction details as well be
// passed to the iclog for checkpoint post-commit processing.  After being
// passed to the iclog, another context needs to be allocated for tracking the
// next set of transactions to be aggregated into a checkpoint.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_cil_ctx {
    pub cil: *mut xfs_cil,
    pub /: *mut *mut xfs_csn_t sequence; / chkpt sequence #,
    pub /: *mut *mut xfs_lsn_t start_lsn; / first LSN of chkpt commit,
    pub /: *mut *mut xfs_lsn_t commit_lsn; / chkpt commit record lsn,
    pub commit_iclog: *mut xlog_in_core,
    pub /: *mut *mut *mut xlog_ticket ticket; / chkpt ticket,
    pub /: *mut *mut atomic_t space_used; / aggregate size of regions,
    pub busy_extents: xfs_busy_extents,
    pub /: *mut *mut list_head log_items; / log items in chkpt,
    pub /: *mut *mut list_head lv_chain; / logvecs being pushed,
    pub iclog_entry: list_head,
    pub /: *mut *mut list_head committing; / ctx committing list,
    pub push_work: work_struct,
    pub order_id: core::sync::atomic::AtomicI32,
//
// CPUs that could have added items to the percpu CIL data.  Access is
// coordinated with xc_ctx_lock.
//
    pub cil_pcpmask: cpumask,
}

//
// Per-cpu CIL tracking items
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_cil_pcp {
    pub space_used: i32,
    pub space_reserved: u32,
    pub busy_extents: list_head,
    pub log_items: list_head,
}

//
// Committed Item List structure
//
// This structure is used to track log items that have been committed but not
// yet written into the log. It is used only when the delayed logging mount
// option is enabled.
//
// This structure tracks the list of committing checkpoint contexts so
// we can avoid the problem of having to hold out new transactions during a
// flush until we have a the commit record LSN of the checkpoint. We can
// traverse the list of committing contexts in xlog_cil_push_lsn() to find a
// sequence match and extract the commit LSN directly from there. If the
// checkpoint is still in the process of committing, we can block waiting for
// the commit LSN to be determined as well. This should make synchronous
// operations almost as efficient as the old logging methods.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_cil {
    pub xc_log: *mut xlog,
    pub xc_flags: c_ulong,
    pub xc_iclog_hdrs: core::sync::atomic::AtomicI32,
    pub xc_push_wq: *mut workqueue_struct,
    pub ____cacheline_aligned_in_smp: rw_semaphore xc_ctx_lock,
    pub xc_ctx: *mut xfs_cil_ctx,
    pub ____cacheline_aligned_in_smp: spinlock_t xc_push_lock,
    pub xc_push_seq: xfs_csn_t,
    pub xc_push_commit_stable: bool,
    pub xc_committing: list_head,
    pub xc_commit_wait: wait_queue_head_t,
    pub xc_start_wait: wait_queue_head_t,
    pub xc_current_sequence: xfs_csn_t,
    pub /: *mut *mut wait_queue_head_t xc_push_wait; / background push throttle,
    pub /: *mut *mut *mut void __percpu xc_pcp; / percpu CIL structures,
    pub ____cacheline_aligned_in_smp: },
// xc_flags bit values
pub const XLOG_CIL_EMPTY: c_int = 1;
pub const XLOG_CIL_PCP_SPACE: c_int = 2;
//
// The amount of log space we allow the CIL to aggregate is difficult to size.
// Whatever we choose, we have to make sure we can get a reservation for the
// log space effectively, that it is large enough to capture sufficient
// relogging to reduce log buffer IO significantly, but it is not too large for
// the log or induces too much latency when writing out through the iclogs. We
// track both space consumed and the number of vectors in the checkpoint
// context, so we need to decide which to use for limiting.
//
// Every log buffer we write out during a push needs a header reserved, which
// is at least one sector and more for v2 logs. Hence we need a reservation of
// at least 512 bytes per 32k of log space just for the LR headers. That means
// 16KB of reservation per megabyte of delayed logging space we will consume,
// plus various headers.  The number of headers will vary based on the num of
// io vectors, so limiting on a specific number of vectors is going to result
// in transactions of varying size. IOWs, it is more consistent to track and
// limit space consumed in the log rather than by the number of objects being
// logged in order to prevent checkpoint ticket overruns.
//
// Further, use of static reservations through the log grant mechanism is
// problematic. It introduces a lot of complexity (e.g. reserve grant vs write
// grant) and a significant deadlock potential because regranting write space
// can block on log pushes. Hence if we have to regrant log space during a log
// push, we can deadlock.
//
// However, we can avoid this by use of a dynamic "reservation stealing"
// technique during transaction commit whereby unused reservation space in the
// transaction ticket is transferred to the CIL ctx commit ticket to cover the
// space needed by the checkpoint transaction. This means that we never need to
// specifically reserve space for the CIL checkpoint transaction, nor do we
// need to regrant space once the checkpoint completes. This also means the
// checkpoint transaction ticket is specific to the checkpoint context, rather
// than the CIL itself.
//
// With dynamic reservations, we can effectively make up arbitrary limits for
// the checkpoint size so long as they don't violate any other size rules.
// Recovery imposes a rule that no transaction exceed half the log, so we are
// limited by that.  Furthermore, the log transaction reservation subsystem
// tries to keep 25% of the log free, so we need to keep below that limit or we
// risk running out of free log space to start any new transactions.
//
// In order to keep background CIL push efficient, we only need to ensure the
// CIL is large enough to maintain sufficient in-memory relogging to avoid
// repeated physical writes of frequently modified metadata. If we allow the CIL
// to grow to a substantial fraction of the log, then we may be pinning hundreds
// of megabytes of metadata in memory until the CIL flushes. This can cause
// issues when we are running low on memory - pinned memory cannot be reclaimed,
// and the CIL consumes a lot of memory. Hence we need to set an upper physical
// size limit for the CIL that limits the maximum amount of memory pinned by the
// CIL but does not limit performance by reducing relogging efficiency
// significantly.
//
// As such, the CIL push threshold ends up being the smaller of two thresholds:
// - a threshold large enough that it allows CIL to be pushed and progress to be
// made without excessive blocking of incoming transaction commits. This is
// defined to be 12.5% of the log space - half the 25% push threshold of the
// AIL.
// - small enough that it doesn't pin excessive amounts of memory but maintains
// close to peak relogging efficiency. This is defined to be 16x the iclog
// buffer window (32MB) as measurements have shown this to be roughly the
// point of diminishing performance increases under highly concurrent
// modification workloads.
//
// To prevent the CIL from overflowing upper commit size bounds, we introduce a
// new threshold at which we block committing transactions until the background
// CIL commit commences and switches to a new context. While this is not a hard
// limit, it forces the process committing a transaction to the CIL to block and
// yeild the CPU, giving the CIL push work a chance to be scheduled and start
// work. This prevents a process running lots of transactions from overfilling
// the CIL because it is not yielding the CPU. We set the blocking limit at
// twice the background push space threshold so we keep in line with the AIL
// push thresholds.
//
// Note: this is not a -hard- limit as blocking is applied after the transaction
// is inserted into the CIL and the push has been triggered. It is largely a
// throttling mechanism that allows the CIL push to be scheduled and run. A hard
// limit will be difficult to implement without introducing global serialisation
// in the CIL commit fast path, and it's not at all clear that we actually need
// such hard limits given the ~7 years we've run without a hard limit before
// finding the first situation where a checkpoint size overflow actually
// occurred. Hence the simple throttle, and an ASSERT check to tell us that
// we've overrun the max size.
//

//
// ticket grant locks, queues and accounting have their own cachlines
// as these are quite hot and can be operated on concurrently.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog_grant_head {
    pub ____cacheline_aligned_in_smp: spinlock_t lock,
    pub waiters: list_head,
    pub grant: core::sync::atomic::AtomicI64,
}

//
// The reservation head lsn is not made up of a cycle number and block number.
// Instead, it uses a cycle number and byte number.  Logs don't expect to
// overflow 31 bits worth of byte offset, so using a byte number will mean
// that round off problems won't occur when releasing partial reservations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xlog {
// The following fields don't need locking
    pub /: *mut *mut *mut xfs_mount l_mp; / mount point,
    pub /: *mut *mut *mut xfs_ail l_ailp; / AIL log is working with,
    pub /: *mut *mut *mut xfs_cil l_cilp; / CIL log is working with,
    pub /: *mut *mut *mut xfs_buftarg l_targ; / buftarg of log,
    pub /: *mut *mut *mut workqueue_l_ioend_workqueue; / for I/O completions,
    pub /: *mut *mut delayed_work l_work; / background flush work,
    pub /: *mut *mut long l_opstate; / operational state,
    pub /: *mut *mut *mut uint l_quotaoffs_flag; / XFS_DQ_, for QUOTAOFFs,
    pub l_buf_cancel_table: *mut list_head,
    pub /: *mut *mut list_head r_dfops; / recovered log intent items,
    pub /: *mut *mut int l_iclog_hsize; / size of iclog header,
    pub /: *mut *mut uint l_sectBBsize; / sector size in BBs (2^n),
    pub /: *mut *mut int l_iclog_size; / size of log in bytes,
    pub /: *mut *mut int l_iclog_bufs; / number of iclog buffers,
    pub /: *mut *mut xfs_daddr_t l_logBBstart; / start block of log,
    pub /: *mut *mut int l_logsize; / size of log in bytes,
    pub /: *mut *mut int l_logBBsize; / size of log in BB chunks,
// The following block of fields are changed while holding icloglock
    pub ____cacheline_aligned_in_smp: wait_queue_head_t l_flush_wait,
// waiting for iclog flush
    pub disk: *mut *mut int l_covered_state;/ state of "covering,
// log entries"
    pub /: *mut *mut *mut xlog_in_core l_iclog; / head log queue,
    pub /: *mut *mut spinlock_t l_icloglock; / grab to change iclog state,
    pub /: *mut *mut int l_curr_cycle; / Cycle number of log writes,
    pub last: *mut *mut int l_prev_cycle; / Cycle number before,
// block increment
    pub /: *mut *mut int l_curr_block; / current logical log block,
    pub /: *mut *mut int l_prev_block; / previous logical log block,
//
// l_tail_lsn is atomic so it can be set and read without needing to
// hold specific locks. To avoid operations contending with other hot
// objects, it on a separate cacheline.
//
// lsn of 1st LR with unflushed * buffers
    pub ____cacheline_aligned_in_smp: atomic64_t l_tail_lsn,
    pub l_reserve_head: xlog_grant_head,
    pub l_write_head: xlog_grant_head,
    pub l_tail_space: u64,
    pub l_kobj: xfs_kobj,
// log recovery lsn tracking (for buffer submission
    pub l_recovery_lsn: xfs_lsn_t,
    pub /: *mut *mut uint32_t l_iclog_roundoff;/ padding roundoff,
}

//
// Bits for operational state
//

extern "C" {
    pub fn test_bit(_arg: XLOG_RECOVERY_NEEDED, _arg: &log->l_opstate) -> return;
}
extern "C" {
    pub fn test_bit(_arg: XLOG_ACTIVE_RECOVERY, _arg: &log->l_opstate) -> return;
}
extern "C" {
    pub fn test_bit(_arg: XLOG_IO_ERROR, _arg: &log->l_opstate) -> return;
}
//
// Wait until the xlog_force_shutdown() has marked the log as shut down
// so xlog_is_shutdown() will always return true.
//
// common routines
extern "C" {
    pub fn xlog_print_tic_res(mp: *mut xfs_mount, ticket: *mut xlog_ticket);
}
extern "C" {
    pub fn xlog_print_trans(: *mut xfs_trans);
}
extern "C" {
    pub fn xfs_log_ticket_ungrant(log: *mut xlog, ticket: *mut xlog_ticket);
}
extern "C" {
    pub fn xfs_log_ticket_regrant(log: *mut xlog, ticket: *mut xlog_ticket);
}
//
// When we crack an atomic LSN, we sample it first so that the value will not
// change while we are cracking it into the component values. This means we
// will always get consistent component values to work from. This should always
// be used to sample and crack LSNs that are stored and updated in atomic
// variables.
//
// cycle = CYCLE_LSN(val);
// block = BLOCK_LSN(val);
//
// Calculate and assign a value to an atomic LSN variable from component pieces.
//
// Committed Item List interfaces
//
extern "C" {
    pub fn xlog_cil_init(log: *mut xlog) -> c_int;
}
extern "C" {
    pub fn xlog_cil_init_post_recovery(log: *mut xlog);
}
extern "C" {
    pub fn xlog_cil_destroy(log: *mut xlog);
}
extern "C" {
    pub fn xlog_cil_empty(log: *mut xlog) -> bool;
}
//
// CIL force routines
//
extern "C" {
    pub fn xlog_cil_flush(log: *mut xlog);
}
extern "C" {
    pub fn xlog_cil_force_seq(log: *mut xlog, sequence: xfs_csn_t) -> xfs_lsn_t;
}
//
// Wrapper function for waiting on a wait queue serialised against wakeups
// by a spinlock. This matches the semantics of all the wait queues used in the
// log code.
//
// Calculate the distance between two LSNs in bytes
extern "C" {
    pub fn BBTOB(lo_block: hi_block -) -> return;
}
//
// The LSN is valid so long as it is behind the current LSN. If it isn't, this
// means that the next log record that includes this metadata could have a
// smaller LSN. In turn, this means that the modification in the log would not
// replay.
//
// First, sample the current lsn without locking to avoid added
// contention from metadata I/O. The current cycle and block are updated
// (in xlog_state_switch_iclogs()) and read here in a particular order
// to avoid false negatives (e.g., thinking the metadata LSN is valid
// when it is not).
//
// The current block is always rewound before the cycle is bumped in
// xlog_state_switch_iclogs() to ensure the current LSN is never seen in
// a transiently forward state. Instead, we can see the LSN in a
// transiently behind state if we happen to race with a cycle wrap.
//
// If the metadata LSN appears invalid, it's possible the check
// above raced with a wrap to the next log cycle. Grab the lock
// to check for sure.
//
// Log vector and shadow buffers can be large, so we need to use kvmalloc() here
// to ensure success. Unfortunately, kvmalloc() only allows GFP_KERNEL contexts
// to fall back to vmalloc, so we can't actually do anything useful with gfp
// flags to control the kmalloc() behaviour within kvmalloc(). Hence kmalloc()
// will do direct reclaim and compaction in the slow path, both of which are
// horrendously expensive. We just want kmalloc to fail fast and fall back to
// vmalloc if it can't get something straight away from the free lists or
// buddy allocator. Hence we have to open code kvmalloc outselves here.
//
// This assumes that the caller uses memalloc_nofs_save task context here, so
// despite the use of GFP_KERNEL here, we are going to be doing GFP_NOFS
// allocations. This is actually the only way to make vmalloc() do GFP_NOFS
// allocations, so lets just all pretend this is a GFP_KERNEL context
// operation....
//
// Given a count of iovecs and space for a log item, compute the space we need
// in the log to store that data plus the log headers.
//
extern "C" {
    pub fn round_up(_arg: nbytes, _arg: sizeof(uint64_t)) -> return;
}
//
// Cycles over XLOG_CYCLE_DATA_SIZE overflow into the extended header that was
// added for v2 logs.  Addressing for the cycles array there is off by one,
// because the first batch of cycles is in the original header.
//
