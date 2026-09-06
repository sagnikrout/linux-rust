//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlxsw/spectrum_ipip.h
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
// Copyright (c) 2017-2018 Mellanox Technologies. All rights reserved

extern "C" {
    pub fn mlxsw_sp_l3addr_is_zero(addr: mlxsw_sp_l3addr) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlxsw_sp_ipip_type {
    MLXSW_SP_IPIP_TYPE_GRE4,
    MLXSW_SP_IPIP_TYPE_GRE6,
    MLXSW_SP_IPIP_TYPE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_ipip_parms {
    pub proto: mlxsw_sp_l3proto,
    pub saddr: mlxsw_sp_l3addr,
    pub daddr: mlxsw_sp_l3addr,
    pub link: c_int,
    pub ikey: u32,
    pub okey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_ipip_entry {
    pub ipipt: mlxsw_sp_ipip_type,
    pub /: *mut *mut *mut net_device ol_dev; / Overlay.,
    pub ol_lb: *mut mlxsw_sp_rif_ipip_lb,
    pub decap_fib_entry: *mut mlxsw_sp_fib_entry,
    pub ipip_list_node: list_head,
    pub parms: mlxsw_sp_ipip_parms,
    pub dip_kvdl_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlxsw_sp_ipip_ops {
    pub dev_type: c_int,
    pub /: *mut *mut mlxsw_sp_l3proto ul_proto; / Underlay.,
    pub inc_parsing_depth: bool,
    pub double_rif_entry: bool,
    pub ol_dev): *const *const (parms_init)(struct net_device,
    pub ratr_pl): *mut bool force, char,
    pub ol_dev): *const net_device,
// Return a configuration for creating an overlay loopback RIF.
    pub ol_dev): *const net_device,
    pub tunnel_index): u32,
    pub extack): *mut netlink_ext_ack,
    pub ipip_entry): *mut mlxsw_sp_ipip_entry,
    pub ipip_entry): *const mlxsw_sp_ipip_entry,
}
