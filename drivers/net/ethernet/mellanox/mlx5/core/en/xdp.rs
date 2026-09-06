//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/xdp.h
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


//
// Copyright (c) 2018, Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

pub const MLX5E_XDP_INLINE_WQE_MAX_DS_CNT: c_int = 16;

// XDP packets can be transmitted in different ways. On completion, we need to
// distinguish between them to clean up things in a proper way.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5e_xdp_xmit_mode {
// An xdp_frame was transmitted due to either XDP_REDIRECT from another
// device or XDP_TX from an XSK RQ. The frame has to be unmapped and
// returned.
//
    MLX5E_XDP_XMIT_MODE_FRAME,

// The xdp_frame was created in place as a result of XDP_TX from a
// regular RQ. No DMA remapping happened, and the page belongs to us.
//
    MLX5E_XDP_XMIT_MODE_PAGE,

// No xdp_frame was created at all, the transmit happened from a UMEM
// page. The UMEM Completion Ring producer pointer has to be increased.
//
    MLX5E_XDP_XMIT_MODE_XSK,
}

// xmit_mode entry is pushed to the fifo per packet, followed by multiple
// entries, as follows:
//
// MLX5E_XDP_XMIT_MODE_FRAME:
// xdpf, dma_addr_1, dma_addr_2, ... , dma_addr_num.
// 'num' is derived from xdpf.
//
// MLX5E_XDP_XMIT_MODE_PAGE:
// num, page_1, page_2, ... , page_num.
//
// MLX5E_XDP_XMIT_MODE_XSK:
// frame.xsk_meta.
//
pub const MLX5E_XDP_FIFO_ENTRIES2DS_MAX_RATIO: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5e_xdp_info {
    pub mode: mlx5e_xdp_xmit_mode,
    pub xdpf: *mut xdp_frame,
    pub dma_addr: dma_addr_t,
    pub frame: },
    pub rq: *mut mlx5e_rq,
    pub num: u8,
    pub page: *mut page,
    pub page: },
    pub xsk_meta: xsk_tx_metadata_compl,
}

extern "C" {
    pub fn mlx5e_xdp_mpwqe_complete(sq: *mut mlx5e_xdpsq);
}
extern "C" {
    pub fn mlx5e_poll_xdpsq_cq(cq: *mut mlx5e_cq) -> bool;
}
extern "C" {
    pub fn mlx5e_free_xdpsq_descs(sq: *mut mlx5e_xdpsq);
}
extern "C" {
    pub fn mlx5e_set_xmit_fp(sq: *mut mlx5e_xdpsq, is_mpw: bool);
}
extern "C" {
    pub fn mlx5e_xdp_rx_poll_complete(rq: *mut mlx5e_rq);
}
// Let other device's napi(s) and XSK wakeups see our new state.
extern "C" {
    pub fn test_bit(_arg: MLX5E_STATE_XDP_TX_ENABLED, _arg: &priv->state) -> return;
}
extern "C" {
    pub fn test_bit(_arg: MLX5E_STATE_XDP_ACTIVE, _arg: &priv->state) -> return;
}
// Enable inline WQEs to shift some load from a congested HCA (HW) to
// a less congested cpu (SW).
//
pub const MLX5E_XDP_INLINE_WATERMARK_LOW: c_int = 10;
pub const MLX5E_XDP_INLINE_WATERMARK_HIGH: c_int = 128;
extern "C" {
    pub fn mlx5e_tx_mpwqe_is_full(_arg: session) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xdp_wqe_info {
    pub num_wqebbs: u8,
    pub num_pkts: u8,
}
