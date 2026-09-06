//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/acl/helper.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2020 Mellanox Technologies Inc. All rights reserved.

// General acl helper functions
// Egress acl helper functions
extern "C" {
    pub fn esw_acl_egress_table_destroy(vport: *mut mlx5_vport);
}
extern "C" {
    pub fn esw_acl_egress_vlan_destroy(vport: *mut mlx5_vport);
}
extern "C" {
    pub fn esw_acl_egress_vlan_grp_create(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) -> c_int;
}
extern "C" {
    pub fn esw_acl_egress_vlan_grp_destroy(vport: *mut mlx5_vport);
}
// Ingress acl helper functions
extern "C" {
    pub fn esw_acl_ingress_table_destroy(vport: *mut mlx5_vport);
}
extern "C" {
    pub fn esw_acl_ingress_allow_rule_destroy(vport: *mut mlx5_vport);
}
