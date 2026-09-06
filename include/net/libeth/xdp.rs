//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/libeth/xdp.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2025 Intel Corporation

//
// Defined as bits to be able to use them as a mask on Rx.
// Also used as internal return values on Tx.
//
// &xdp_buff_xsk is the largest structure &libeth_xdp_buff gets casted to,
// pick maximum pointer-compatible alignment.
//

//
// struct libeth_xdp_buff - libeth extension over &xdp_buff
// @base: main &xdp_buff
// @data: shortcut for @base.data
// @desc: RQ descriptor containing metadata for this buffer
// @priv: driver-private scratchspace
//
// The main reason for this is to have a pointer to the descriptor to be able
// to quickly get frame metadata from xdpmo and driver buff-to-xdp callbacks
// (as well as bigger alignment).
// Pointer/layout-compatible with &xdp_buff and &xdp_buff_xsk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdp_buff {
    pub base: xdp_buff,
    pub data: *mut c_void,
}

//
// __LIBETH_XDP_ONSTACK_BUFF - declare a &libeth_xdp_buff on the stack
// @name: name of the variable to declare
// @...: sizeof() of the driver-private data
//

//
// LIBETH_XDP_ONSTACK_BUFF - declare a &libeth_xdp_buff on the stack
// @name: name of the variable to declare
// @...: type or variable name of the driver-private data
//

// Macro flag: #define __libeth_xdp_psz0(...)

// Performs XSK_CHECK_PRIV_TYPE()

// XDPSQ sharing
//
// libeth_xdpsq_num - calculate optimal number of XDPSQs for this device + sys
// @rxq: current number of active Rx queues
// @txq: current number of active Tx queues
// @max: maximum number of Tx queues
//
// Each RQ must have its own XDPSQ for XSk pairs, each CPU must have own XDPSQ
// for lockless sending (``XDP_TX``, .ndo_xdp_xmit()). Cap the maximum of these
// two with the number of SQs the device can have (minus used ones).
//
// Return: number of XDP Tx queues the device needs to use.
//
extern "C" {
    pub fn min(_arg: max(nr_cpu_ids, _arg: rxq), txq: max -) -> return;
}
//
// libeth_xdpsq_shared - whether XDPSQs can be shared between several CPUs
// @num: number of active XDPSQs
//
// Return: true if there's no 1:1 XDPSQ/CPU association, false otherwise.
//
// libeth_xdpsq_id - get XDPSQ index corresponding to this CPU
// @num: number of active XDPSQs
//
// Helper for libeth_xdp routines, do not use in drivers directly.
//
// Return: XDPSQ index needs to be used on this CPU.
//
// libeth_xdpsq_get - initialize &libeth_xdpsq_lock
// @lock: lock to initialize
// @dev: netdev which this lock belongs to
// @share: whether XDPSQs can be shared
//
// Tracks the current XDPSQ association and enables the static lock
// if needed.
//
// libeth_xdpsq_put - deinitialize &libeth_xdpsq_lock
// @lock: lock to deinitialize
// @dev: netdev which this lock belongs to
//
// Tracks the current XDPSQ association and disables the static lock
// if needed.
//
extern "C" {
    pub fn __libeth_xdpsq_lock(lock: *mut libeth_xdpsq_lock);
}
extern "C" {
    pub fn __libeth_xdpsq_unlock(lock: *mut libeth_xdpsq_lock);
}
//
// libeth_xdpsq_lock - grab &libeth_xdpsq_lock if needed
// @lock: lock to take
//
// Touches the underlying spinlock only if the static key is enabled
// and the queue itself is marked as shareable.
//
// libeth_xdpsq_unlock - free &libeth_xdpsq_lock if needed
// @lock: lock to free
//
// Touches the underlying spinlock only if the static key is enabled
// and the queue itself is marked as shareable.
//
// XDPSQ clean-up timers
//
// libeth_xdpsq_deinit_timer - deinitialize &libeth_xdpsq_timer
// @timer: timer to deinitialize
//
// Flush and disable the underlying workqueue.
//
// libeth_xdpsq_queue_timer - run &libeth_xdpsq_timer
// @timer: timer to queue
//
// Should be called after the queue was filled and the transmission was run
// to complete the pending buffers if no further sending will be done in a
// second (-> lazy cleaning won't happen).
// If the timer was already run, it will be requeued back to one second
// timeout again.
//
// libeth_xdpsq_run_timer - wrapper to run a queue clean-up on a timer event
// @work: workqueue belonging to the corresponding timer
// @poll: driver-specific completion queue poll function
//
// Run the polling function on the locked queue and requeue the timer if
// there's more work to do.
// Designed to be used via LIBETH_XDP_DEFINE_TIMER() below.
//
// Common Tx bits
//
// enum - libeth_xdp internal Tx flags
// @LIBETH_XDP_TX_BULK: one bulk size at which it will be flushed to the queue
// @LIBETH_XDP_TX_BATCH: batch size for which the queue fill loop is unrolled
// @LIBETH_XDP_TX_DROP: indicates the send function must drop frames not sent
// @LIBETH_XDP_TX_NDO: whether the send function is called from .ndo_xdp_xmit()
// @LIBETH_XDP_TX_XSK: whether the function is called for ``XDP_TX`` for XSk
//
// enum - &libeth_xdp_tx_frame and &libeth_xdp_tx_desc flags
// @LIBETH_XDP_TX_LEN: only for ``XDP_TX``, [15:0] of ::len_fl is actual length
// @LIBETH_XDP_TX_CSUM: for XSk xmit, enable checksum offload
// @LIBETH_XDP_TX_XSKMD: for XSk xmit, mask of the metadata bits
// @LIBETH_XDP_TX_FIRST: indicates the frag is the first one of the frame
// @LIBETH_XDP_TX_LAST: whether the frag is the last one of the frame
// @LIBETH_XDP_TX_MULTI: whether the frame contains several frags
// @LIBETH_XDP_TX_FLAGS: only for ``XDP_TX``, [31:16] of ::len_fl is flags
//
// struct libeth_xdp_tx_frame - represents one XDP Tx element
// @data: frame start pointer for ``XDP_TX``
// @len_fl: ``XDP_TX``, combined flags [31:16] and len [15:0] field for speed
// @soff: ``XDP_TX``, offset from @data to the start of &skb_shared_info
// @frag: one (non-head) frag for ``XDP_TX``
// @xdpf: &xdp_frame for the head frag for .ndo_xdp_xmit()
// @dma: DMA address of the non-head frag for .ndo_xdp_xmit()
// @xsk: ``XDP_TX`` for XSk, XDP buffer for any frag
// @len: frag length for XSk ``XDP_TX`` and .ndo_xdp_xmit()
// @flags: Tx flags for the above
// @opts: combined @len + @flags for the above for speed
// @desc: XSk xmit descriptor for direct casting
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdp_tx_frame {
// ``XDP_TX``
    pub data: *mut c_void,
    pub len_fl: u32,
    pub soff: u32,
}

// ``XDP_TX`` frag
// .ndo_xdp_xmit(), XSk ``XDP_TX``
// XSk xmit
//
// struct libeth_xdp_tx_bulk - XDP Tx frame bulk for bulk sending
// @prog: corresponding active XDP program, %NULL for .ndo_xdp_xmit()
// @dev: &net_device which the frames are transmitted on
// @xdpsq: shortcut to the corresponding driver-specific XDPSQ structure
// @act_mask: Rx only, mask of all the XDP prog verdicts for that NAPI session
// @count: current number of frames in @bulk
// @bulk: array of queued frames for bulk Tx
//
// All XDP Tx operations except XSk xmit queue each frame to the bulk first
// and flush it when @count reaches the array end. Bulk is always placed on
// the stack for performance. One bulk element contains all the data necessary
// for sending a frame and then freeing it on completion.
// For XSk xmit, Tx descriptor array from &xsk_buff_pool is casted directly
// to &libeth_xdp_tx_frame as they are compatible and the bulk structure is
// not used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdp_tx_bulk {
    pub prog: *const bpf_prog,
    pub dev: *mut net_device,
    pub xdpsq: *mut c_void,
    pub act_mask: u32,
    pub count: u32,
    pub bulk: [libeth_xdp_tx_frame; LIBETH_XDP_TX_BULK],
    pub libeth_xdp_tx_frame)): } __aligned(sizeof(struct,
//
// LIBETH_XDP_ONSTACK_BULK - declare &libeth_xdp_tx_bulk on the stack
// @bq: name of the variable to declare
//
// Helper to declare a bulk on the stack with a compiler hint that it should
// not be initialized automatically (with `CONFIG_INIT_STACK_ALL_*`) for
// performance reasons.
//

//
// struct libeth_xdpsq - abstraction for an XDPSQ
// @pool: XSk buffer pool for XSk ``XDP_TX`` and xmit
// @sqes: array of Tx buffers from the actual queue struct
// @descs: opaque pointer to the HW descriptor array
// @ntu: pointer to the next free descriptor index
// @count: number of descriptors on that queue
// @pending: pointer to the number of sent-not-completed descs on that queue
// @xdp_tx: pointer to the above, but only for non-XSk-xmit frames
// @lock: corresponding XDPSQ lock
//
// Abstraction for driver-independent implementation of Tx. Placed on the stack
// and filled by the driver before the transmission, so that the generic
// functions can access and modify driver-specific resources.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdpsq {
    pub pool: *mut xsk_buff_pool,
    pub sqes: *mut libeth_sqe,
    pub descs: *mut c_void,
    pub ntu: *mut u32,
    pub count: u32,
    pub pending: *mut u32,
    pub xdp_tx: *mut u32,
    pub lock: *mut libeth_xdpsq_lock,
}

//
// struct libeth_xdp_tx_desc - abstraction for an XDP Tx descriptor
// @addr: DMA address of the frame
// @len: length of the frame
// @flags: XDP Tx flags
// @opts: combined @len + @flags for speed
//
// Filled by the generic functions and then passed to driver-specific functions
// to fill a HW Tx descriptor, always placed on the [function] stack.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xdp_tx_desc {
    pub addr: dma_addr_t,
    pub len: u32,
    pub flags: u32,
}

//
// libeth_xdp_ptr_to_priv - convert pointer to a libeth_xdp u64 priv
// @ptr: pointer to convert
//
// The main sending function passes private data as the largest scalar, u64.
// Use this helper when you want to pass a pointer there.
//

//
// libeth_xdp_priv_to_ptr - convert libeth_xdp u64 priv to a pointer
// @priv: private data to convert
//
// The main sending function passes private data as the largest scalar, u64.
// Use this helper when your callback takes this u64 and you want to convert
// it back to a pointer.
//

//
// On 64-bit systems, assigning one u64 is faster than two u32s. When ::len
// occupies lowest 32 bits (LE), whole ::opts can be assigned directly instead.
//

pub const __LIBETH_WORD_ACCESS: c_int = 1;

//
// libeth_xdp_tx_xmit_bulk - main XDP Tx function
// @bulk: array of frames to send
// @xdpsq: pointer to the driver-specific XDPSQ struct
// @n: number of frames to send
// @unroll: whether to unroll the queue filling loop for speed
// @priv: driver-specific private data
// @prep: callback for cleaning the queue and filling abstract &libeth_xdpsq
// @fill: internal callback for filling &libeth_sqe and &libeth_xdp_tx_desc
// @xmit: callback for filling a HW descriptor with the frame info
//
// Internal abstraction for placing @n XDP Tx frames on the HW XDPSQ. Used for
// all types of frames: ``XDP_TX``, .ndo_xdp_xmit(), XSk ``XDP_TX``, and XSk
// xmit.
// @prep must lock the queue as this function releases it at the end. @unroll
// greatly increases the object code size, but also greatly increases XSk xmit
// performance; for other types of frames, it's not enabled.
// The compilers inline all those onstack abstractions to direct data accesses.
//
// Return: number of frames actually placed on the queue, <= @n. The function
// can't fail, but can send less frames if there's no enough free descriptors
// available. The actual free space is returned by @prep from the driver.
//
// sq.ntu = ntu;
// sq.pending += n;
// sq.xdp_tx += n;
// ``XDP_TX`` bulking
extern "C" {
    pub fn libeth_xdp_return_buff_slow(xdp: *mut libeth_xdp_buff);
}
//
// libeth_xdp_tx_queue_head - internal helper for queueing one ``XDP_TX`` head
// @bq: XDP Tx bulk to queue the head frag to
// @xdp: XDP buffer with the head to queue
//
// Return: false if it's the only frag of the frame, true if it's an S/G frame.
//
// libeth_xdp_tx_queue_frag - internal helper for queueing one ``XDP_TX`` frag
// @bq: XDP Tx bulk to queue the frag to
// @frag: frag to queue
//
// libeth_xdp_tx_queue_bulk - internal helper for queueing one ``XDP_TX`` frame
// @bq: XDP Tx bulk to queue the frame to
// @xdp: XDP buffer to queue
// @flush_bulk: driver callback to flush the bulk to the HW queue
//
// Return: true on success, false on flush error.
//
// libeth_xdp_tx_fill_stats - fill &libeth_sqe with ``XDP_TX`` frame stats
// @sqe: SQ element to fill
// @desc: libeth_xdp Tx descriptor
// @sinfo: &skb_shared_info for this frame
//
// Internal helper for filling an SQE with the frame stats, do not use in
// drivers. Fills the number of frags and bytes for this frame.
//

//
// libeth_xdp_tx_fill_buf - internal helper to fill one ``XDP_TX`` &libeth_sqe
// @frm: XDP Tx frame from the bulk
// @i: index on the HW queue
// @sq: XDPSQ abstraction for the queue
// @priv: private data
//
// Return: XDP Tx descriptor with the synced DMA and other info to pass to
// the driver callback.
//
// __libeth_xdp_tx_flush_bulk - internal helper to flush one XDP Tx bulk
// @bq: bulk to flush
// @flags: XDP TX flags (.ndo_xdp_xmit(), XSk etc.)
// @prep: driver-specific callback to prepare the queue for sending
// @fill: libeth_xdp callback to fill &libeth_sqe and &libeth_xdp_tx_desc
// @xmit: driver callback to fill a HW descriptor
//
// Internal abstraction to create bulk flush functions for drivers. Used for
// everything except XSk xmit.
//
// Return: true if anything was sent, false otherwise.
//
extern "C" {
    pub fn likely(_arg: sent) -> return;
}
//
// libeth_xdp_tx_flush_bulk - wrapper to define flush of one ``XDP_TX`` bulk
// @bq: bulk to flush
// @flags: Tx flags, see above
// @prep: driver callback to prepare the queue
// @xmit: driver callback to fill a HW descriptor
//
// Use via LIBETH_XDP_DEFINE_FLUSH_TX() to define an ``XDP_TX`` driver
// callback.
//

// .ndo_xdp_xmit() implementation
//
// libeth_xdp_xmit_init_bulk - internal helper to initialize bulk for XDP xmit
// @bq: bulk to initialize
// @dev: target &net_device
// @xdpsqs: array of driver-specific XDPSQ structs
// @num: number of active XDPSQs (the above array length)
//

//
// libeth_xdp_xmit_frame_dma - internal helper to access DMA of an &xdp_frame
// @xf: pointer to the XDP frame
//
// There's no place in &libeth_xdp_tx_frame to store DMA address for an
// &xdp_frame head. The headroom is used then, the address is placed right
// after the frame struct, naturally aligned.
//
// Return: pointer to the DMA address to use.
//

//
// libeth_xdp_xmit_queue_head - internal helper for queueing one XDP xmit head
// @bq: XDP Tx bulk to queue the head frag to
// @xdpf: XDP frame with the head to queue
// @dev: device to perform DMA mapping
//
// Return: ``LIBETH_XDP_DROP`` on DMA mapping error,
// ``LIBETH_XDP_PASS`` if it's the only frag in the frame,
// ``LIBETH_XDP_TX`` if it's an S/G frame.
//
// libeth_xdp_xmit_frame_dma(xdpf) = dma;
//
// libeth_xdp_xmit_queue_frag - internal helper for queueing one XDP xmit frag
// @bq: XDP Tx bulk to queue the frag to
// @frag: frag to queue
// @dev: device to perform DMA mapping
//
// Return: true on success, false on DMA mapping error.
//
// libeth_xdp_xmit_queue_bulk - internal helper for queueing one XDP xmit frame
// @bq: XDP Tx bulk to queue the frame to
// @xdpf: XDP frame to queue
// @flush_bulk: driver callback to flush the bulk to the HW queue
//
// Return: ``LIBETH_XDP_TX`` on success,
// ``LIBETH_XDP_DROP`` if the frame should be dropped by the stack,
// ``LIBETH_XDP_ABORTED`` if the frame will be dropped by libeth_xdp.
//
// libeth_xdp_xmit_fill_buf - internal helper to fill one XDP xmit &libeth_sqe
// @frm: XDP Tx frame from the bulk
// @i: index on the HW queue
// @sq: XDPSQ abstraction for the queue
// @priv: private data
//
// Return: XDP Tx descriptor with the mapped DMA and other info to pass to
// the driver callback.
//
// libeth_xdp_xmit_flush_bulk - wrapper to define flush of one XDP xmit bulk
// @bq: bulk to flush
// @flags: Tx flags, see __libeth_xdp_tx_flush_bulk()
// @prep: driver callback to prepare the queue
// @xmit: driver callback to fill a HW descriptor
//
// Use via LIBETH_XDP_DEFINE_FLUSH_XMIT() to define an XDP xmit driver
// callback.
//

//
// __libeth_xdp_xmit_do_bulk - internal function to implement .ndo_xdp_xmit()
// @bq: XDP Tx bulk to queue frames to
// @frames: XDP frames passed by the stack
// @n: number of frames
// @flags: flags passed by the stack
// @flush_bulk: driver callback to flush an XDP xmit bulk
// @finalize: driver callback to finalize sending XDP Tx frames on the queue
//
// Perform common checks, map the frags and queue them to the bulk, then flush
// the bulk to the XDPSQ. If requested by the stack, finalize the queue.
//
// Return: number of frames send or -errno on error.
//
// libeth_xdp_xmit_do_bulk - implement full .ndo_xdp_xmit() in driver
// @dev: target &net_device
// @n: number of frames to send
// @fr: XDP frames to send
// @f: flags passed by the stack
// @xqs: array of XDPSQs driver structs
// @nqs: number of active XDPSQs, the above array length
// @fl: driver callback to flush an XDP xmit bulk
// @fin: driver cabback to finalize the queue
//
// If the driver has active XDPSQs, perform common checks and send the frames.
// Finalize the queue, if requested.
//
// Return: number of frames sent or -errno on error.
//

// Rx polling path
//
// libeth_xdp_tx_init_bulk - initialize an XDP Tx bulk for Rx NAPI poll
// @bq: bulk to initialize
// @prog: RCU pointer to the XDP program (can be %NULL)
// @dev: target &net_device
// @xdpsqs: array of driver XDPSQ structs
// @num: number of active XDPSQs, the above array length
//
// Should be called on an onstack XDP Tx bulk before the NAPI polling loop.
// Initializes all the needed fields to run libeth_xdp functions. If @num == 0,
// assumes XDP is not enabled.
// Do not use for XSk, it has its own optimized helper.
//

extern "C" {
    pub fn __libeth_xdp_return_stash(stash: *mut libeth_xdp_buff_stash);
}
//
// libeth_xdp_init_buff - initialize a &libeth_xdp_buff for Rx NAPI poll
// @dst: onstack buffer to initialize
// @src: XDP buffer stash placed on the queue
// @rxq: registered &xdp_rxq_info corresponding to this queue
//
// Should be called before the main NAPI polling loop. Loads the content of
// the previously saved stash or initializes the buffer from scratch.
// Do not use for XSk.
//
// libeth_xdp_save_buff - save a partially built buffer on a queue
// @dst: XDP buffer stash placed on the queue
// @src: onstack buffer to save
//
// Should be called after the main NAPI polling loop. If the loop exited before
// the buffer was finished, saves its content on the queue, so that it can be
// completed during the next poll. Otherwise, clears the stash.
//
// libeth_xdp_return_stash - free an XDP buffer stash from a queue
// @stash: stash to free
//
// If the queue is about to be destroyed, but it still has an incompleted
// buffer stash, this helper should be called to free it.
//
// libeth_xdp_return_buff - free/recycle &libeth_xdp_buff
// @xdp: buffer to free
//
// Hotpath helper to free &libeth_xdp_buff. Comparing to xdp_return_buff(),
// it's faster as it gets inlined and always assumes order-0 pages and safe
// direct recycling. Zeroes @xdp->data to avoid UAFs.
//

//
// libeth_xdp_prepare_buff - fill &libeth_xdp_buff with head FQE data
// @xdp: XDP buffer to attach the head to
// @fqe: FQE containing the head buffer
// @len: buffer len passed from HW
//
// Internal, use libeth_xdp_process_buff() instead. Initializes XDP buffer
// head with the Rx buffer data: data pointer, length, headroom, and
// truesize/tailroom. Zeroes the flags.
//
// libeth_xdp_process_buff - attach Rx buffer to &libeth_xdp_buff
// @xdp: XDP buffer to attach the Rx buffer to
// @fqe: Rx buffer to process
// @len: received data length from the descriptor
//
// If the XDP buffer is empty, attaches the Rx buffer as head and initializes
// the required fields. Otherwise, attaches the buffer as a frag.
// Already performs DMA sync-for-CPU and frame start prefetch
// (for head buffers only).
//
// Return: true on success, false if the descriptor must be skipped (empty or
// no space for a new frag).
//
extern "C" {
    pub fn libeth_xdp_buff_add_frag(_arg: xdp, _arg: fqe, _arg: len) -> return;
}
//
// libeth_xdp_buff_stats_frags - update onstack RQ stats with XDP frags info
// @ss: onstack stats to update
// @xdp: buffer to account
//
// Internal helper used by __libeth_xdp_run_pass(), do not call directly.
// Adds buffer's frags count and total len to the onstack stats.
//
// __libeth_xdp_run_prog - run XDP program on an XDP buffer
// @xdp: XDP buffer to run the prog on
// @bq: buffer bulk for ``XDP_TX`` queueing
//
// Internal inline abstraction to run XDP program. Handles ``XDP_DROP``
// and ``XDP_REDIRECT`` only, the rest is processed levels up.
// Reports an XDP prog exception on errors.
//
// Return: libeth_xdp prog verdict depending on the prog's verdict.
//
extern "C" {
    pub fn libeth_xdp_prog_exception(_arg: bq, _arg: xdp, _arg: act, _arg: 0) -> return;
}
//
// __libeth_xdp_run_flush - run XDP program and handle ``XDP_TX`` verdict
// @xdp: XDP buffer to run the prog on
// @bq: buffer bulk for ``XDP_TX`` queueing
// @run: internal callback for running XDP program
// @queue: internal callback for queuing ``XDP_TX`` frame
// @flush_bulk: driver callback for flushing a bulk
//
// Internal inline abstraction to run XDP program and additionally handle
// ``XDP_TX`` verdict. Used by both XDP and XSk, hence @run and @queue.
// Do not use directly.
//
// Return: libeth_xdp prog verdict depending on the prog's verdict.
//
// libeth_xdp_run_prog - run XDP program (non-XSk path) and handle all verdicts
// @xdp: XDP buffer to process
// @bq: XDP Tx bulk to queue ``XDP_TX`` buffers
// @fl: driver ``XDP_TX`` bulk flush callback
//
// Run the attached XDP program and handle all possible verdicts. XSk has its
// own version.
// Prefer using it via LIBETH_XDP_DEFINE_RUN{,_PASS,_PROG}().
//
// Return: true if the buffer should be passed up the stack, false if the poll
// should go to the next buffer.
//

//
// __libeth_xdp_run_pass - helper to run XDP program and handle the result
// @xdp: XDP buffer to process
// @bq: XDP Tx bulk to queue ``XDP_TX`` frames
// @napi: NAPI to build an skb and pass it up the stack
// @rs: onstack libeth RQ stats
// @md: metadata that should be filled to the XDP buffer
// @prep: callback for filling the metadata
// @run: driver wrapper to run XDP program
// @populate: driver callback to populate an skb with the HW descriptor data
//
// Inline abstraction that does the following (non-XSk path):
// 1) adds frame size and frag number (if needed) to the onstack stats;
// 2) fills the descriptor metadata to the onstack &libeth_xdp_buff
// 3) runs XDP program if present;
// 4) handles all possible verdicts;
// 5) on ``XDP_PASS`, builds an skb from the buffer;
// 6) populates it with the descriptor metadata;
// 7) passes it up the stack.
//
// In most cases, number 2 means just writing the pointer to the HW descriptor
// to the XDP buffer. If so, please use LIBETH_XDP_DEFINE_RUN{,_PASS}()
// wrappers to build a driver function.
//
// libeth_xdp_run_pass - helper to run XDP program and handle the result
// @xdp: XDP buffer to process
// @bq: XDP Tx bulk to queue ``XDP_TX`` frames
// @napi: NAPI to build an skb and pass it up the stack
// @ss: onstack libeth RQ stats
// @desc: pointer to the HW descriptor for that frame
// @run: driver wrapper to run XDP program
// @populate: driver callback to populate an skb with the HW descriptor data
//
// Wrapper around the underscored version when "fill the descriptor metadata"
// means just writing the pointer to the HW descriptor as @xdp->desc.
//

//
// libeth_xdp_finalize_rx - finalize XDPSQ after a NAPI polling loop (non-XSk)
// @bq: ``XDP_TX`` frame bulk
// @flush: driver callback to flush the bulk
// @finalize: driver callback to start sending the frames and run the timer
//
// Flush the bulk if there are frames left to send, kick the queue and flush
// the XDP maps.
//

//
// Helpers to reduce boilerplate code in drivers.
//
// Typical driver Rx flow would be (excl. bulk and buff init, frag attach):
//
// LIBETH_XDP_DEFINE_START();
// LIBETH_XDP_DEFINE_FLUSH_TX(static driver_xdp_flush_tx, driver_xdp_tx_prep,
// driver_xdp_xmit);
// LIBETH_XDP_DEFINE_RUN(static driver_xdp_run, driver_xdp_run_prog,
// driver_xdp_flush_tx, driver_populate_skb);
// LIBETH_XDP_DEFINE_FINALIZE(static driver_xdp_finalize_rx,
// driver_xdp_flush_tx, driver_xdp_finalize_sq);
// LIBETH_XDP_DEFINE_END();
//
// This will build a set of 4 static functions. The compiler is free to decide
// whether to inline them.
// Then, in the NAPI polling function:
//
// while (packets < budget) {
// // ...
// driver_xdp_run(xdp, &bq, napi, &rs, desc);
// }
// driver_xdp_finalize_rx(&bq);
//

//
// LIBETH_XDP_DEFINE_TIMER - define a driver XDPSQ cleanup timer callback
// @name: name of the function to define
// @poll: Tx polling/completion function
//

//
// LIBETH_XDP_DEFINE_FLUSH_TX - define a driver ``XDP_TX`` bulk flush function
// @name: name of the function to define
// @prep: driver callback to clean an XDPSQ
// @xmit: driver callback to write a HW Tx descriptor
//

//
// LIBETH_XDP_DEFINE_FLUSH_XMIT - define a driver XDP xmit bulk flush function
// @name: name of the function to define
// @prep: driver callback to clean an XDPSQ
// @xmit: driver callback to write a HW Tx descriptor
//

//
// LIBETH_XDP_DEFINE_RUN_PROG - define a driver XDP program run function
// @name: name of the function to define
// @flush: driver callback to flush an ``XDP_TX`` bulk
//

//
// LIBETH_XDP_DEFINE_RUN_PASS - define a driver buffer process + pass function
// @name: name of the function to define
// @run: driver callback to run XDP program (above)
// @populate: driver callback to fill an skb with HW descriptor info
//

//
// LIBETH_XDP_DEFINE_RUN - define a driver buffer process, run + pass function
// @name: name of the function to define
// @run: name of the XDP prog run function to define
// @flush: driver callback to flush an ``XDP_TX`` bulk
// @populate: driver callback to fill an skb with HW descriptor info
//

//
// LIBETH_XDP_DEFINE_FINALIZE - define a driver Rx NAPI poll finalize function
// @name: name of the function to define
// @flush: driver callback to flush an ``XDP_TX`` bulk
// @finalize: driver callback to finalize an XDPSQ and run the timer
//

// XMO
//
// libeth_xdp_buff_to_rq - get RQ pointer from an XDP buffer pointer
// @xdp: &libeth_xdp_buff corresponding to the queue
// @type: typeof() of the driver Rx queue structure
// @member: name of &xdp_rxq_info inside @type
//
// Often times, pointer to the RQ is needed when reading/filling metadata from
// HW descriptors. The helper can be used to quickly jump from an XDP buffer
// to the queue corresponding to its &xdp_rxq_info without introducing
// additional fields (&libeth_xdp_buff is precisely 1 cacheline long on x64).
//

//
// libeth_xdpmo_rx_hash - convert &libeth_rx_pt to an XDP RSS hash metadata
// @hash: pointer to the variable to write the hash to
// @rss_type: pointer to the variable to write the hash type to
// @val: hash value from the HW descriptor
// @pt: libeth parsed packet type
//
// Handle zeroed/non-available hash and convert libeth parsed packet type to
// the corresponding XDP RSS hash type. To be called at the end of
// xdp_metadata_ops idpf_xdpmo::xmo_rx_hash() implementation.
// Note that if the driver doesn't use a constant packet type lookup table but
// generates it at runtime, it must call libeth_rx_pt_gen_hash_type(pt) to
// generate XDP RSS hash type for each packet type.
//
// Return: 0 on success, -ENODATA when the hash is not available.
//
// hash = val;
// rss_type = pt.hash_type;
// Tx buffer completion
extern "C" {
    pub fn libeth_xsk_buff_free_slow(xdp: *mut libeth_xdp_buff);
}
//
// __libeth_xdp_complete_tx - complete sent XDPSQE
// @sqe: SQ element / Tx buffer to complete
// @cp: Tx polling/completion params
// @bulk: internal callback to bulk-free ``XDP_TX`` buffers
// @xsk: internal callback to free XSk ``XDP_TX`` buffers
//
// Use the non-underscored version in drivers instead. This one is shared
// internally with libeth_tx_complete_any().
// Complete an XDPSQE of any type of XDP frame. This includes DMA unmapping
// when needed, buffer freeing, stats update, and SQE invalidation.
//
// Misc
extern "C" {
    pub fn libeth_xdp_queue_threshold(count: u32) -> u32;
}
extern "C" {
    pub fn libeth_xdp_set_redirect(dev: *mut net_device, enable: bool);
}
//
// libeth_xdp_set_features - set XDP features for netdev
// @dev: &net_device to configure
// @...: optional params, see __libeth_xdp_set_features()
//
// Set all the features libeth_xdp supports, including .ndo_xdp_xmit(). That
// said, it should be used only when XDPSQs are always available regardless
// of whether an XDP prog is attached to @dev.
//

//
// libeth_xdp_set_features_noredir - enable all libeth_xdp features w/o redir
// @dev: target &net_device
// @...: optional params, see __libeth_xdp_set_features()
//
// Enable everything except the .ndo_xdp_xmit() feature, use when XDPSQs are
// not available right after netdev registration.
//

