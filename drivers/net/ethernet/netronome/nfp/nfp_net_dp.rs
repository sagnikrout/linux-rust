//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/netronome/nfp/nfp_net_dp.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2019 Netronome Systems, Inc.

//
// nfp_net_tx_full() - check if the TX ring is full
// @tx_ring: TX ring to check
// @dcnt:    Number of descriptors that need to be enqueued (must be >= 1)
//
// This function checks, based on the *host copy* of read/write
// pointer if a given TX ring is full.  The real TX queue may have
// some newly made available slots.
//
// Return: True if the ring is full.
//
extern "C" {
    pub fn nfp_qcp_rd_ptr_read(_arg: tx_ring->qcp_q) -> return;
}
//
// nfp_net_irq_unmask() - Unmask automasked interrupt
// @nn:       NFP Network structure
// @entry_nr: MSI-X table entry
//
// Clear the ICR for the IRQ entry.
//
// Common
extern "C" {
    pub fn nfp_net_vec_clear_ring_data(nn: *mut nfp_net, idx: c_uint);
}
extern "C" {
    pub fn nfp_net_rx_rings_prepare(nn: *mut nfp_net, dp: *mut nfp_net_dp) -> c_int;
}
extern "C" {
    pub fn nfp_net_tx_rings_prepare(nn: *mut nfp_net, dp: *mut nfp_net_dp) -> c_int;
}
extern "C" {
    pub fn nfp_net_rx_rings_free(dp: *mut nfp_net_dp);
}
extern "C" {
    pub fn nfp_net_tx_rings_free(dp: *mut nfp_net_dp);
}
extern "C" {
    pub fn nfp_net_rx_ring_reset(rx_ring: *mut nfp_net_rx_ring);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nfp_nfd_version {
    NFP_NFD_VER_NFD3,
    NFP_NFD_VER_NFDK,
}

//
// struct nfp_dp_ops - Hooks to wrap different implementation of different dp
// @version:			Indicate dp type
// @tx_min_desc_per_pkt:	Minimal TX descs needed for each packet
// @cap_mask:			Mask of supported features
// @dma_mask:			DMA addressing capability
// @poll:			Napi poll for normal rx/tx
// @xsk_poll:			Napi poll when xsk is enabled
// @ctrl_poll:			Tasklet poll for ctrl rx/tx
// @xmit:			Xmit for normal path
// @ctrl_tx_one:		Xmit for ctrl path
// @rx_ring_fill_freelist:	Give buffers from the ring to FW
// @tx_ring_alloc:		Allocate resource for a TX ring
// @tx_ring_reset:		Free any untransmitted buffers and reset pointers
// @tx_ring_free:		Free resources allocated to a TX ring
// @tx_ring_bufs_alloc:		Allocate resource for each TX buffer
// @tx_ring_bufs_free:		Free resources allocated to each TX buffer
// @print_tx_descs:		Show TX ring's info for debug purpose
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_dp_ops {
    pub version: nfp_nfd_version,
    pub tx_min_desc_per_pkt: c_uint,
    pub cap_mask: u32,
    pub dma_mask: u64,
    pub budget): *mut *mut *mut int (poll)(struct napi_struct napi, int,
    pub budget): *mut *mut *mut int (xsk_poll)(struct napi_struct napi, int,
    pub t): *mut *mut void (ctrl_poll)(struct tasklet_struct,
    pub netdev): *mut *mut *mut netdev_tx_t (xmit)(struct sk_buff skb, struct net_device,
    pub old): *mut *mut sk_buff skb, bool,
    pub rx_ring): *mut nfp_net_rx_ring,
    pub tx_ring): *mut nfp_net_tx_ring,
    pub tx_ring): *mut nfp_net_tx_ring,
    pub tx_ring): *mut *mut void (tx_ring_free)(struct nfp_net_tx_ring,
    pub tx_ring): *mut nfp_net_tx_ring,
    pub tx_ring): *mut nfp_net_tx_ring,
    pub d_wr_p): u32 d_rd_p, u32,
}

extern "C" {
    pub fn nfp_net_tx(skb: *mut sk_buff, netdev: *mut net_device) -> netdev_tx_t;
}
