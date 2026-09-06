//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/libeth/xsk.h
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

// ``XDP_TXMD_FLAGS_VALID`` is defined only under ``CONFIG_XDP_SOCKETS``

// ``XDP_TX`` bulking
//
// libeth_xsk_tx_queue_head - internal helper for queueing XSk ``XDP_TX`` head
// @bq: XDP Tx bulk to queue the head frag to
// @xdp: XSk buffer with the head to queue
//
// Return: false if it's the only frag of the frame, true if it's an S/G frame.
//
// libeth_xsk_tx_queue_frag - internal helper for queueing XSk ``XDP_TX`` frag
// @bq: XDP Tx bulk to queue the frag to
// @frag: XSk frag to queue
//
// libeth_xsk_tx_queue_bulk - internal helper for queueing XSk ``XDP_TX`` frame
// @bq: XDP Tx bulk to queue the frame to
// @xdp: XSk buffer to queue
// @flush_bulk: driver callback to flush the bulk to the HW queue
//
// Return: true on success, false on flush error.
//
// libeth_xsk_tx_fill_buf - internal helper to fill XSk ``XDP_TX`` &libeth_sqe
// @frm: XDP Tx frame from the bulk
// @i: index on the HW queue
// @sq: XDPSQ abstraction for the queue
// @priv: private data
//
// Return: XDP Tx descriptor with the synced DMA and other info to pass to
// the driver callback.
//
// libeth_xsk_tx_flush_bulk - wrapper to define flush of XSk ``XDP_TX`` bulk
// @bq: bulk to flush
// @flags: Tx flags, see __libeth_xdp_tx_flush_bulk()
// @prep: driver callback to prepare the queue
// @xmit: driver callback to fill a HW descriptor
//
// Use via LIBETH_XSK_DEFINE_FLUSH_TX() to define an XSk ``XDP_TX`` driver
// callback.
//

// XSk TMO
//
// libeth_xsktmo_req_csum - XSk Tx metadata op to request checksum offload
// @csum_start: unused
// @csum_offset: unused
// @priv: &libeth_xdp_tx_desc from the filling helper
//
// Generic implementation of ::tmo_request_checksum. Works only when HW doesn't
// require filling checksum offsets and other parameters beside the checksum
// request bit.
// Consider using within @libeth_xsktmo unless the driver requires HW-specific
// callbacks.
//
// Only to inline the callbacks below, use @libeth_xsktmo in drivers instead
//
// __libeth_xsk_xmit_fill_buf_md - internal helper to prepare XSk xmit w/meta
// @xdesc: &xdp_desc from the XSk buffer pool
// @sq: XDPSQ abstraction for the queue
// @priv: XSk Tx metadata ops
//
// Same as __libeth_xsk_xmit_fill_buf(), but requests metadata pointer and
// fills additional fields in &libeth_xdp_tx_desc to ask for metadata offload.
//
// Return: XDP Tx descriptor with the DMA, metadata request bits, and other
// info to pass to the driver callback.
//
// XSk xmit implementation
//
// __libeth_xsk_xmit_fill_buf - internal helper to prepare XSk xmit w/o meta
// @xdesc: &xdp_desc from the XSk buffer pool
// @sq: XDPSQ abstraction for the queue
//
// Return: XDP Tx descriptor with the DMA and other info to pass to
// the driver callback.
//
// libeth_xsk_xmit_fill_buf - internal helper to prepare an XSk xmit
// @frm: &xdp_desc from the XSk buffer pool
// @i: index on the HW queue
// @sq: XDPSQ abstraction for the queue
// @priv: XSk Tx metadata ops
//
// Depending on the metadata ops presence (determined at compile time), calls
// the quickest helper to build a libeth XDP Tx descriptor.
//
// Return: XDP Tx descriptor with the synced DMA, metadata request bits,
// and other info to pass to the driver callback.
//
// libeth_xsk_xmit_do_bulk - send XSk xmit frames
// @pool: XSk buffer pool containing the frames to send
// @xdpsq: opaque pointer to driver's XDPSQ struct
// @budget: maximum number of frames can be sent
// @tmo: optional XSk Tx metadata ops
// @prep: driver callback to build a &libeth_xdpsq
// @xmit: driver callback to put frames to a HW queue
// @finalize: driver callback to start a transmission
//
// Implements generic XSk xmit. Always turns on XSk Tx wakeup as it's assumed
// lazy cleaning is used and interrupts are disabled for the queue.
// HW descriptor filling is unrolled by ``LIBETH_XDP_TX_BATCH`` to optimize
// writes.
// Note that unlike other XDP Tx ops, the queue must be locked and cleaned
// prior to calling this function to already know available @budget.
// @prepare must only build a &libeth_xdpsq and return ``U32_MAX``.
//
// Return: false if @budget was exhausted, true otherwise.
//
// Rx polling path
//
// libeth_xsk_tx_init_bulk - initialize XDP Tx bulk for an XSk Rx NAPI poll
// @bq: bulk to initialize
// @prog: RCU pointer to the XDP program (never %NULL)
// @dev: target &net_device
// @xdpsqs: array of driver XDPSQ structs
// @num: number of active XDPSQs, the above array length
//
// Should be called on an onstack XDP Tx bulk before the XSk NAPI polling loop.
// Initializes all the needed fields to run libeth_xdp functions.
// Never checks if @prog is %NULL or @num == 0 as XDP must always be enabled
// when hitting this path.
//

//
// libeth_xsk_process_buff - attach XSk Rx buffer to &libeth_xdp_buff
// @head: head XSk buffer to attach the XSk buffer to (or %NULL)
// @xdp: XSk buffer to process
// @len: received data length from the descriptor
//
// If @head == %NULL, treats the XSk buffer as head and initializes
// the required fields. Otherwise, attaches the buffer as a frag.
// Already performs DMA sync-for-CPU and frame start prefetch
// (for head buffers only).
//
// Return: head XSk buffer on success or if the descriptor must be skipped
// (empty), %NULL if there is no space for a new frag.
//
extern "C" {
    pub fn libeth_xsk_buff_add_frag(_arg: head, _arg: xdp) -> return;
}
//
// __libeth_xsk_run_prog - run XDP program on XSk buffer
// @xdp: XSk buffer to run the prog on
// @bq: buffer bulk for ``XDP_TX`` queueing
//
// Internal inline abstraction to run XDP program on XSk Rx path. Handles
// only the most common ``XDP_REDIRECT`` inline, the rest is processed
// externally.
// Reports an XDP prog exception on errors.
//
// Return: libeth_xdp prog verdict depending on the prog's verdict.
//
extern "C" {
    pub fn __libeth_xsk_run_prog_slow(_arg: xdp, _arg: bq, _arg: act, _arg: ret) -> return;
}
//
// libeth_xsk_run_prog - run XDP program on XSk path and handle all verdicts
// @xdp: XSk buffer to process
// @bq: XDP Tx bulk to queue ``XDP_TX`` buffers
// @fl: driver ``XDP_TX`` bulk flush callback
//
// Run the attached XDP program and handle all possible verdicts.
// Prefer using it via LIBETH_XSK_DEFINE_RUN{,_PASS,_PROG}().
//
// Return: libeth_xdp prog verdict depending on the prog's verdict.
//

//
// __libeth_xsk_run_pass - helper to run XDP program and handle the result
// @xdp: XSk buffer to process
// @bq: XDP Tx bulk to queue ``XDP_TX`` frames
// @napi: NAPI to build an skb and pass it up the stack
// @rs: onstack libeth RQ stats
// @md: metadata that should be filled to the XSk buffer
// @prep: callback for filling the metadata
// @run: driver wrapper to run XDP program
// @populate: driver callback to populate an skb with the HW descriptor data
//
// Inline abstraction, XSk's counterpart of __libeth_xdp_run_pass(), see its
// doc for details.
//
// Return: false if the polling loop must be exited due to lack of free
// buffers, true otherwise.
//
// libeth_xsk_run_pass - helper to run XDP program and handle the result
// @xdp: XSk buffer to process
// @bq: XDP Tx bulk to queue ``XDP_TX`` frames
// @napi: NAPI to build an skb and pass it up the stack
// @rs: onstack libeth RQ stats
// @desc: pointer to the HW descriptor for that frame
// @run: driver wrapper to run XDP program
// @populate: driver callback to populate an skb with the HW descriptor data
//
// Wrapper around the underscored version when "fill the descriptor metadata"
// means just writing the pointer to the HW descriptor as @xdp->desc.
//

//
// libeth_xsk_finalize_rx - finalize XDPSQ after an XSk NAPI polling loop
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
// Typical driver XSk Rx flow would be (excl. bulk and buff init, frag attach):
//
// LIBETH_XDP_DEFINE_START();
// LIBETH_XSK_DEFINE_FLUSH_TX(static driver_xsk_flush_tx, driver_xsk_tx_prep,
// driver_xdp_xmit);
// LIBETH_XSK_DEFINE_RUN(static driver_xsk_run, driver_xsk_run_prog,
// driver_xsk_flush_tx, driver_populate_skb);
// LIBETH_XSK_DEFINE_FINALIZE(static driver_xsk_finalize_rx,
// driver_xsk_flush_tx, driver_xdp_finalize_sq);
// LIBETH_XDP_DEFINE_END();
//
// This will build a set of 4 static functions. The compiler is free to decide
// whether to inline them.
// Then, in the NAPI polling function:
//
// while (packets < budget) {
// // ...
// if (!driver_xsk_run(xdp, &bq, napi, &rs, desc))
// break;
// }
// driver_xsk_finalize_rx(&bq);
//
// LIBETH_XSK_DEFINE_FLUSH_TX - define a driver XSk ``XDP_TX`` flush function
// @name: name of the function to define
// @prep: driver callback to clean an XDPSQ
// @xmit: driver callback to write a HW Tx descriptor
//

//
// LIBETH_XSK_DEFINE_RUN_PROG - define a driver XDP program run function
// @name: name of the function to define
// @flush: driver callback to flush an XSk ``XDP_TX`` bulk
//

//
// LIBETH_XSK_DEFINE_RUN_PASS - define a driver buffer process + pass function
// @name: name of the function to define
// @run: driver callback to run XDP program (above)
// @populate: driver callback to fill an skb with HW descriptor info
//

//
// LIBETH_XSK_DEFINE_RUN - define a driver buffer process, run + pass function
// @name: name of the function to define
// @run: name of the XDP prog run function to define
// @flush: driver callback to flush an XSk ``XDP_TX`` bulk
// @populate: driver callback to fill an skb with HW descriptor info
//

//
// LIBETH_XSK_DEFINE_FINALIZE - define a driver XSk NAPI poll finalize function
// @name: name of the function to define
// @flush: driver callback to flush an XSk ``XDP_TX`` bulk
// @finalize: driver callback to finalize an XDPSQ and run the timer
//

// Refilling
//
// struct libeth_xskfq - structure representing an XSk buffer (fill) queue
// @fp: hotpath part of the structure
// @pool: &xsk_buff_pool for buffer management
// @fqes: array of XSk buffer pointers
// @descs: opaque pointer to the HW descriptor array
// @ntu: index of the next buffer to poll
// @count: number of descriptors/buffers the queue has
// @pending: current number of XSkFQEs to refill
// @thresh: threshold below which the queue is refilled
// @buf_len: HW-writeable length per each buffer
// @truesize: step between consecutive buffers, 0 if none exists
// @nid: ID of the closest NUMA node with memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_xskfq {
    pub pool: *mut xsk_buff_pool,
    pub fqes: *mut libeth_xdp_buff,
    pub descs: *mut c_void,
    pub ntu: u32,
    pub count: u32,
// Cold fields
    pub pending: u32,
    pub thresh: u32,
    pub buf_len: u32,
    pub truesize: u32,
    pub nid: c_int,
}

extern "C" {
    pub fn libeth_xskfq_create(fq: *mut libeth_xskfq) -> c_int;
}
extern "C" {
    pub fn libeth_xskfq_destroy(fq: *mut libeth_xskfq);
}
//
// libeth_xsk_buff_xdp_get_dma - get DMA address of XSk &libeth_xdp_buff
// @xdp: buffer to get the DMA addr for
//

//
// libeth_xskfqe_alloc - allocate @n XSk Rx buffers
// @fq: hotpath part of the XSkFQ, usually onstack
// @n: number of buffers to allocate
// @fill: driver callback to write DMA addresses to HW descriptors
//
// Note that @fq->ntu gets updated, but ::pending must be recalculated
// by the caller.
//
// Return: number of buffers refilled.
//
// .ndo_xsk_wakeup
extern "C" {
    pub fn libeth_xsk_init_wakeup(csd: *mut call_single_data_t, napi: *mut napi_struct);
}
extern "C" {
    pub fn libeth_xsk_wakeup(csd: *mut call_single_data_t, qid: u32);
}
// Pool setup
extern "C" {
    pub fn libeth_xsk_setup_pool(dev: *mut net_device, qid: u32, enable: bool) -> c_int;
}
