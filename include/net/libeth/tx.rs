//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/libeth/tx.h
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
// Copyright (C) 2024-2025 Intel Corporation

// Tx buffer completion
//
// enum libeth_sqe_type - type of &libeth_sqe to act on Tx completion
// @LIBETH_SQE_EMPTY: unused/empty OR XDP_TX/XSk frame, no action required
// @LIBETH_SQE_CTX: context descriptor with empty SQE, no action required
// @LIBETH_SQE_SLAB: kmalloc-allocated buffer, unmap and kfree()
// @LIBETH_SQE_FRAG: mapped skb frag, only unmap DMA
// @LIBETH_SQE_SKB: &sk_buff, unmap and napi_consume_skb(), update stats
// @__LIBETH_SQE_XDP_START: separator between skb and XDP types
// @LIBETH_SQE_XDP_TX: &skb_shared_info, libeth_xdp_return_buff_bulk(), stats
// @LIBETH_SQE_XDP_XMIT: &xdp_frame, unmap and xdp_return_frame_bulk(), stats
// @LIBETH_SQE_XDP_XMIT_FRAG: &xdp_frame frag, only unmap DMA
// @LIBETH_SQE_XSK_TX: &libeth_xdp_buff on XSk queue, xsk_buff_free(), stats
// @LIBETH_SQE_XSK_TX_FRAG: &libeth_xdp_buff frag on XSk queue, xsk_buff_free()
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libeth_sqe_type {
    LIBETH_SQE_EMPTY		= 0U,
    LIBETH_SQE_CTX,
    LIBETH_SQE_SLAB,
    LIBETH_SQE_FRAG,
    LIBETH_SQE_SKB,

    __LIBETH_SQE_XDP_START,
    LIBETH_SQE_XDP_TX		= __LIBETH_SQE_XDP_START,
    LIBETH_SQE_XDP_XMIT,
    LIBETH_SQE_XDP_XMIT_FRAG,
    LIBETH_SQE_XSK_TX,
    LIBETH_SQE_XSK_TX_FRAG,
}

//
// struct libeth_sqe - represents a Send Queue Element / Tx buffer
// @type: type of the buffer, see the enum above
// @rs_idx: index of the last buffer from the batch this one was sent in
// @raw: slab buffer to free via kfree()
// @skb: &sk_buff to consume
// @sinfo: skb shared info of an XDP_TX frame
// @xdpf: XDP frame from ::ndo_xdp_xmit()
// @xsk: XSk Rx frame from XDP_TX action
// @dma: DMA address to unmap
// @len: length of the mapped region to unmap
// @nr_frags: number of frags in the frame this buffer belongs to
// @packets: number of physical packets sent for this frame
// @bytes: number of physical bytes sent for this frame
// @priv: driver-private scratchpad
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_sqe {
    pub type:32: libeth_sqe_type,
    pub rs_idx: u32,
    pub raw: *mut c_void,
    pub skb: *mut sk_buff,
    pub sinfo: *mut skb_shared_info,
    pub xdpf: *mut xdp_frame,
    pub xsk: *mut libeth_xdp_buff,
}

//
// LIBETH_SQE_CHECK_PRIV - check the driver's private SQE data
// @p: type or name of the object the driver wants to fit into &libeth_sqe
//
// Make sure the driver's private data fits into libeth_sqe::priv. To be used
// right after its declaration.
//

//
// struct libeth_cq_pp - completion queue poll params
// @dev: &device to perform DMA unmapping
// @bq: XDP frame bulk to combine return operations
// @ss: onstack NAPI stats to fill
// @xss: onstack XDPSQ NAPI stats to fill
// @xdp_tx: number of XDP-not-XSk frames processed
// @napi: whether it's called from the NAPI context
//
// libeth uses this structure to access objects needed for performing full
// Tx complete operation without passing lots of arguments and change the
// prototypes each time a new one is added.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_cq_pp {
    pub dev: *mut device,
    pub bq: *mut xdp_frame_bulk,
    pub ss: *mut libeth_sq_napi_stats,
    pub xss: *mut libeth_xdpsq_napi_stats,
}

//
// libeth_tx_complete - perform Tx completion for one SQE
// @sqe: SQE to complete
// @cp: poll params
//
// Do Tx complete for all the types of buffers, incl. freeing, unmapping,
// updating the stats etc.
//
extern "C" {
    pub fn libeth_tx_complete_any(sqe: *mut libeth_sqe, cp: *mut libeth_cq_pp);
}
