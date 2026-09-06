//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/ipoib/ipoib.h
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
// Copyright (c) 2017, Mellanox Technologies. All rights reserved.
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

pub const MLX5I_MAX_NUM_TC: c_int = 1;
pub const MLX5_IB_GRH_BYTES: c_int = 40;
pub const MLX5_IPOIB_ENCAP_LEN: c_int = 4;
pub const MLX5_IPOIB_PSEUDO_LEN: c_int = 20;

// ipoib rdma netdev's private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5i_priv {
    pub /: *mut *mut rdma_netdev rn; / keep this first,
    pub qpn: u32,
    pub tisn: u32,
    pub sub_interface: bool,
    pub num_sub_interfaces: u32,
    pub qkey: u32,
    pub pkey_index: u16,
    pub qpn_htbl: *mut mlx5i_pkey_qpn_ht,
    pub parent_dev: *mut net_device,
    pub mlx5e_priv: [*mut c_char; ],
}

extern "C" {
    pub fn mlx5i_create_tis(mdev: *mut mlx5_core_dev, underlay_qpn: u32, tisn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5i_get_tisn(mdev: *mut mlx5_core_dev, priv: *mut mlx5e_priv, lag_port: u8, tc: u8) -> u32;
}
// Underlay QP create/destroy functions
extern "C" {
    pub fn mlx5i_create_underlay_qp(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5i_destroy_underlay_qp(mdev: *mut mlx5_core_dev, qpn: u32);
}
// Underlay QP state modification init/uninit functions
extern "C" {
    pub fn mlx5i_init_underlay_qp(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5i_uninit_underlay_qp(priv: *mut mlx5e_priv);
}
// Allocate/Free underlay QPN to net-device hash table
extern "C" {
    pub fn mlx5i_pkey_qpn_ht_init(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx5i_pkey_qpn_ht_cleanup(netdev: *mut net_device);
}
// Add/Remove an underlay QPN to net-device mapping to/from the hash table
extern "C" {
    pub fn mlx5i_pkey_add_qpn(netdev: *mut net_device, qpn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5i_pkey_del_qpn(netdev: *mut net_device, qpn: u32) -> c_int;
}
// Get the net-device corresponding to the given underlay QPN
// Shared ndo functions
extern "C" {
    pub fn mlx5i_dev_init(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx5i_dev_cleanup(dev: *mut net_device);
}
// Parent profile functions
extern "C" {
    pub fn mlx5i_init(mdev: *mut mlx5_core_dev, netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn mlx5i_cleanup(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5i_update_nic_rx(priv: *mut mlx5e_priv) -> c_int;
}
// Get child interface nic profile
// Extract mlx5e_priv from IPoIB netdev

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_eth_pad {
    pub rsvd0: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5i_tx_wqe {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub datagram: mlx5_wqe_datagram_seg,
    pub pad: mlx5_wqe_eth_pad,
    pub eth: mlx5_wqe_eth_seg,
    pub data: [mlx5_wqe_data_seg; ],
}

extern "C" {
    pub fn mlx5i_get_stats(dev: *mut net_device, stats: *mut rtnl_link_stats64);
}
// Reference management for child to parent interfaces.
extern "C" {
    pub fn mlx5i_parent_put(netdev: *mut net_device);
}

