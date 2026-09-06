//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/eswitch.h
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


// SPDX-License-Identifier: (GPL-2.0+ OR BSD-3-Clause)
//
// Copyright (c) 2018 Mellanox Technologies. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_switchdev_event {
    MLX5_SWITCHDEV_EVENT_PAIR,
    MLX5_SWITCHDEV_EVENT_UNPAIR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eswitch_rep_ops {
    pub rep): *mut *mut *mut int (load)(struct mlx5_core_dev dev, struct mlx5_eswitch_rep,
    pub rep): *mut *mut void (unload)(struct mlx5_eswitch_rep,
    pub rep): *mut *mut *mut void (get_proto_dev)(struct mlx5_eswitch_rep,
    pub data): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eswitch_rep_data {
    pub priv: *mut c_void,
    pub state: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eswitch_rep {
    pub rep_data: [mlx5_eswitch_rep_data; NUM_REP_TYPES],
    pub vport: u16,
    pub vlan: u16,
// Only IB rep is using vport_index
    pub vport_index: u16,
    pub vlan_refcount: u32,
    pub esw: *mut mlx5_eswitch,
}

extern "C" {
    pub fn mlx5_eswitch_unregister_vport_reps(esw: *mut mlx5_eswitch, rep_type: u8);
}

extern "C" {
    pub fn mlx5_eswitch_reg_c1_loopback_enabled(esw: *const mlx5_eswitch) -> bool;
}
extern "C" {
    pub fn mlx5_eswitch_vport_match_metadata_enabled(esw: *const mlx5_eswitch) -> bool;
}
// Reg C0 usage:
// Reg C0 = < ESW_PFNUM_BITS(4) | ESW_VPORT BITS(12) | ESW_REG_C0_OBJ(16) >
//
// Highest 4 bits of the reg c0 is the PF_NUM (range 0-15), 12 bits of
// unique non-zero vport id (range 1-4095). The rest (lowest 16 bits) is left
// for user data objects managed by a common mapping context.
// PFNUM + VPORT comprise the SOURCE_PORT matching.
//
pub const ESW_VPORT_BITS: c_int = 12;
pub const ESW_PFNUM_BITS: c_int = 4;

extern "C" {
    pub fn GENMASK(_arg: 31, ESW_SOURCE_PORT_METADATA_BITS: 32 -) -> return;
}
// Reg C1 usage:
// Reg C1 = < Reserved(1) | ESW_TUN_ID(12) | ESW_TUN_OPTS(11) | ESW_ZONE_ID(8) >
//
// Highest bit is reserved for other offloads as marker bit, next 12 bits of reg c1
// is the encapsulation tunnel id, next 11 bits is encapsulation tunnel options,
// and the lowest 8 bits are used for zone id.
//
// Zone id is used to restore CT flow when packet misses on chain.
//
// Tunnel id and options are used together to restore the tunnel info metadata
// on miss and to support inner header rewrite by means of implicit chain 0
// flows.
//
pub const ESW_RESERVED_BITS: c_int = 1;
pub const ESW_ZONE_ID_BITS: c_int = 8;
pub const ESW_TUN_OPTS_BITS: c_int = 11;
pub const ESW_TUN_ID_BITS: c_int = 12;

// 0x7FF is a reserved mapping

// 0x7FE is a reserved mapping for bridge ingress push vlan mark

// reuse tun_opts for the mapped ipsec obj id when tun_id is 0 (invalid)

extern "C" {
    pub fn mlx5_eswitch_mode(dev: *const mlx5_core_dev) -> u8;
}
extern "C" {
    pub fn mlx5_eswitch_get_total_vports(dev: *const mlx5_core_dev) -> u16;
}

// The returned number is valid only when the dev is eswitch manager.
