//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_nve.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
// Copyright (c) 2018 Mellanox Technologies. All rights reserved

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_nve_config {
    pub type: mlxsw_sp_nve_type,
    pub ttl: u8,
    pub learning_en:1: u8,
    pub udp_dport: __be16,
    pub flowlabel: __be32,
    pub ul_tb_id: u32,
    pub ul_proto: mlxsw_sp_l3proto,
    pub ul_sip: mlxsw_sp_l3addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_nve {
    pub config: mlxsw_sp_nve_config,
    pub mc_list_ht: rhashtable,
    pub ipv6_ht: rhashtable,
    pub /: *mut *mut list_head ipv6_addr_list; / Saves hash table nodes.,
    pub mlxsw_sp: *mut mlxsw_sp,
    pub nve_ops_arr: *const mlxsw_sp_nve_ops,
    pub /: *mut *mut unsigned int num_nve_tunnels; / Protected by RTNL,
    pub num_max_mc_entries: [c_uint; MLXSW_SP_L3_PROTO_MAX],
    pub tunnel_index: u32,
    pub /: *mut *mut u16 ul_rif_index; / Reserved for Spectrum,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_nve_ops {
    pub type: mlxsw_sp_nve_type,
    pub extack): *mut netlink_ext_ack,
    pub config): *mut mlxsw_sp_nve_config,
    pub config): *const mlxsw_sp_nve_config,
    pub nve): *mut *mut void (fini)(struct mlxsw_sp_nve,
    pub extack): *mut netlink_ext_ack,
    pub vni): *const *const *const void (fdb_clear_offload)(struct net_device nve_dev, __be32,
}
