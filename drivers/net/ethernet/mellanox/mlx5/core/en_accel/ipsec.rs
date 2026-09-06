//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en_accel/ipsec.h
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
// Copyright (c) 2017 Mellanox Technologies. All rights reserved.
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

pub const MLX5E_IPSEC_SADB_RX_BITS: c_int = 10;
pub const MLX5E_IPSEC_ESN_SCOPE_MID: c_uint = 0x80000000L;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_gcm_keymat {
    pub seq_iv: u64,
    pub salt: u32,
    pub icv_len: u32,
    pub key_len: u32,
    pub 32]: u32 aes_key[256 /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct upspec {
    pub dport: u16,
    pub dport_mask: u16,
    pub sport: u16,
    pub sport_mask: u16,
    pub proto: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ipsec_lft {
    pub hard_packet_limit: u64,
    pub soft_packet_limit: u64,
    pub numb_rounds_hard: u64,
    pub numb_rounds_soft: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_replay_esn {
    pub replay_window: u32,
    pub esn: u32,
    pub esn_msb: u32,
    pub 1: u8 overlap :,
    pub 1: u8 trigger :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_addr {
    pub a4: __be32,
    pub a6: [__be32; 4],
    pub saddr: },
    pub m4: __be32,
    pub m6: [__be32; 4],
    pub smask: },
    pub a4: __be32,
    pub a6: [__be32; 4],
    pub daddr: },
    pub m4: __be32,
    pub m6: [__be32; 4],
    pub dmask: },
    pub family: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_accel_esp_xfrm_attrs {
    pub spi: u32,
    pub mode: u32,
    pub aes_gcm: aes_gcm_keymat,
    pub addrs: mlx5e_ipsec_addr,
    pub upspec: upspec,
    pub 2: u8 dir :,
    pub 2: u8 type :,
    pub 1: u8 drop :,
    pub 1: u8 encap :,
    pub replay_esn: mlx5_replay_esn,
    pub authsize: u32,
    pub reqid: u32,
    pub lft: mlx5_ipsec_lft,
    pub smac: [u8; ETH_ALEN],
    pub sport: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_ipsec_cap {
    MLX5_IPSEC_CAP_CRYPTO		= 1 << 0,
    MLX5_IPSEC_CAP_ESN		= 1 << 1,
    MLX5_IPSEC_CAP_PACKET_OFFLOAD	= 1 << 2,
    MLX5_IPSEC_CAP_ROCE             = 1 << 3,
    MLX5_IPSEC_CAP_PRIO             = 1 << 4,
    MLX5_IPSEC_CAP_TUNNEL           = 1 << 5,
    MLX5_IPSEC_CAP_ESPINUDP         = 1 << 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_hw_stats {
    pub ipsec_rx_pkts: u64,
    pub ipsec_rx_bytes: u64,
    pub ipsec_rx_drop_pkts: u64,
    pub ipsec_rx_drop_bytes: u64,
    pub ipsec_rx_drop_mismatch_sa_sel: u64,
    pub ipsec_tx_pkts: u64,
    pub ipsec_tx_bytes: u64,
    pub ipsec_tx_drop_pkts: u64,
    pub ipsec_tx_drop_bytes: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_sw_stats {
    pub ipsec_rx_drop_sp_alloc: core::sync::atomic::AtomicI64,
    pub ipsec_rx_drop_sadb_miss: core::sync::atomic::AtomicI64,
    pub ipsec_tx_drop_bundle: core::sync::atomic::AtomicI64,
    pub ipsec_tx_drop_no_state: core::sync::atomic::AtomicI64,
    pub ipsec_tx_drop_not_ip: core::sync::atomic::AtomicI64,
    pub ipsec_tx_drop_trailer: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_work {
    pub work: work_struct,
    pub sa_entry: *mut mlx5e_ipsec_sa_entry,
    pub data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_netevent_data {
    pub addr: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_dwork {
    pub dwork: delayed_work,
    pub sa_entry: *mut mlx5e_ipsec_sa_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_aso {
    pub ctx: [u8 __aligned(64); MLX5_ST_SZ_BYTES(ipsec_aso)],
    pub dma_addr: dma_addr_t,
    pub aso: *mut mlx5_aso,
// Protect ASO WQ access, as it is global to whole IPsec
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_rx_create_attr {
    pub ns: *mut mlx5_flow_namespace,
    pub ttc: *mut mlx5_ttc_table,
    pub family: u32,
    pub prio: c_int,
    pub pol_level: c_int,
    pub pol_miss_level: c_int,
    pub sa_level: c_int,
    pub status_level: c_int,
    pub chains_ns: mlx5_flow_namespace_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_ft {
    pub /: *mut *mut mutex mutex; / Protect changes to this struct,
    pub pol: *mut mlx5_flow_table,
    pub sa: *mut mlx5_flow_table,
    pub sa_sel: *mut mlx5_flow_table,
    pub status: *mut mlx5_flow_table,
    pub refcnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_drop {
    pub rule: *mut mlx5_flow_handle,
    pub fc: *mut mlx5_fc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_rule {
    pub rule: *mut mlx5_flow_handle,
    pub status_pass: *mut mlx5_flow_handle,
    pub sa_sel: *mut mlx5_flow_handle,
    pub modify_hdr: *mut mlx5_modify_hdr,
    pub pkt_reformat: *mut mlx5_pkt_reformat,
    pub fc: *mut mlx5_fc,
    pub replay: mlx5e_ipsec_drop,
    pub auth: mlx5e_ipsec_drop,
    pub trailer: mlx5e_ipsec_drop,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_miss {
    pub group: *mut mlx5_flow_group,
    pub rule: *mut mlx5_flow_handle,
    pub fc: *mut mlx5_fc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_tx_create_attr {
    pub prio: c_int,
    pub pol_level: c_int,
    pub sa_level: c_int,
    pub cnt_level: c_int,
    pub chains_ns: mlx5_flow_namespace_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_mpv_work {
    pub event: c_int,
    pub work: work_struct,
    pub slave_priv: *mut mlx5e_priv,
    pub master_priv: *mut mlx5e_priv,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec {
    pub mdev: *mut mlx5_core_dev,
    pub sadb: xarray,
    pub sw_stats: mlx5e_ipsec_sw_stats,
    pub hw_stats: mlx5e_ipsec_hw_stats,
    pub wq: *mut workqueue_struct,
    pub comp: completion,
    pub fs: *mut mlx5e_flow_steering,
    pub rx_ipv4: *mut mlx5e_ipsec_rx,
    pub rx_ipv6: *mut mlx5e_ipsec_rx,
    pub rx_esw: *mut mlx5e_ipsec_rx,
    pub tx: *mut mlx5e_ipsec_tx,
    pub tx_esw: *mut mlx5e_ipsec_tx,
    pub aso: *mut mlx5e_ipsec_aso,
    pub nb: notifier_block,
    pub netevent_nb: notifier_block,
    pub roce: *mut mlx5_ipsec_fs,
    pub 1: u8 is_uplink_rep:,
    pub mpv_work: mlx5e_ipsec_mpv_work,
    pub ipsec_obj_id_map: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_esn_state {
    pub esn: u32,
    pub esn_msb: u32,
    pub 1: u8 overlap:,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_limits {
    pub round: u64,
    pub 1: u8 soft_limit_hit :,
    pub 1: u8 fix_limit :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_sa_entry {
    pub esn_state: mlx5e_ipsec_esn_state,
    pub x: *mut xfrm_state,
    pub dev: *mut net_device,
    pub ipsec: *mut mlx5e_ipsec,
    pub attrs: mlx5_accel_esp_xfrm_attrs,
    pub xo): *mut xfrm_offload,
    pub ipsec_obj_id: u32,
    pub enc_key_id: u32,
    pub ipsec_rule: mlx5e_ipsec_rule,
    pub work: *mut mlx5e_ipsec_work,
    pub dwork: *mut mlx5e_ipsec_dwork,
    pub limits: mlx5e_ipsec_limits,
    pub rx_mapped_id: u32,
    pub ctx: [u8; MLX5_ST_SZ_BYTES(ipsec_aso)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_accel_pol_xfrm_attrs {
    pub addrs: mlx5e_ipsec_addr,
    pub upspec: upspec,
    pub action: u8,
    pub 2: u8 type :,
    pub 2: u8 dir :,
    pub reqid: u32,
    pub prio: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ipsec_pol_entry {
    pub x: *mut xfrm_policy,
    pub ipsec: *mut mlx5e_ipsec,
    pub ipsec_rule: mlx5e_ipsec_rule,
    pub attrs: mlx5_accel_pol_xfrm_attrs,
}

extern "C" {
    pub fn mlx5e_ipsec_init(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_ipsec_cleanup(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_ipsec_build_netdev(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_accel_ipsec_fs_cleanup(ipsec: *mut mlx5e_ipsec);
}
extern "C" {
    pub fn mlx5e_accel_ipsec_fs_init(ipsec: *mut mlx5e_ipsec, devcom: *mut mlx5_devcom_comp_dev) -> c_int;
}
extern "C" {
    pub fn mlx5e_accel_ipsec_fs_add_rule(sa_entry: *mut mlx5e_ipsec_sa_entry) -> c_int;
}
extern "C" {
    pub fn mlx5e_accel_ipsec_fs_del_rule(sa_entry: *mut mlx5e_ipsec_sa_entry);
}
extern "C" {
    pub fn mlx5e_accel_ipsec_fs_add_pol(pol_entry: *mut mlx5e_ipsec_pol_entry) -> c_int;
}
extern "C" {
    pub fn mlx5e_accel_ipsec_fs_del_pol(pol_entry: *mut mlx5e_ipsec_pol_entry);
}
extern "C" {
    pub fn mlx5e_accel_ipsec_fs_modify(sa_entry: *mut mlx5e_ipsec_sa_entry);
}
extern "C" {
    pub fn mlx5e_ipsec_fs_tunnel_allowed(sa_entry: *mut mlx5e_ipsec_sa_entry) -> bool;
}
extern "C" {
    pub fn mlx5_ipsec_create_sa_ctx(sa_entry: *mut mlx5e_ipsec_sa_entry) -> c_int;
}
extern "C" {
    pub fn mlx5_ipsec_free_sa_ctx(sa_entry: *mut mlx5e_ipsec_sa_entry);
}
extern "C" {
    pub fn mlx5_ipsec_device_caps(mdev: *mut mlx5_core_dev) -> u32;
}
extern "C" {
    pub fn mlx5e_ipsec_aso_init(ipsec: *mut mlx5e_ipsec) -> c_int;
}
extern "C" {
    pub fn mlx5e_ipsec_aso_cleanup(ipsec: *mut mlx5e_ipsec);
}
extern "C" {
    pub fn mlx5e_ipsec_send_event(priv: *mut mlx5e_priv, event: c_int);
}
extern "C" {
    pub fn mlx5e_ipsec_disable_events(priv: *mut mlx5e_priv);
}

