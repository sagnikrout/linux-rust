//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/esw/acl/ofld.h
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

// Eswitch acl egress external APIs
extern "C" {
    pub fn esw_acl_egress_ofld_setup(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) -> c_int;
}
extern "C" {
    pub fn esw_acl_egress_ofld_cleanup(vport: *mut mlx5_vport);
}
extern "C" {
    pub fn esw_acl_egress_ofld_bounce_rule_destroy(vport: *mut mlx5_vport, rule_index: c_int);
}
extern "C" {
    pub fn mlx5_esw_acl_egress_vport_unbond(esw: *mut mlx5_eswitch, vport_num: u16) -> c_int;
}
// Eswitch acl ingress external APIs
extern "C" {
    pub fn esw_acl_ingress_ofld_setup(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport) -> c_int;
}
extern "C" {
    pub fn esw_acl_ingress_ofld_cleanup(esw: *mut mlx5_eswitch, vport: *mut mlx5_vport);
}
extern "C" {
    pub fn mlx5_esw_acl_ingress_vport_drop_rule_destroy(esw: *mut mlx5_eswitch, vport_num: u16);
}
extern "C" {
    pub fn mlx5_esw_acl_ingress_vport_drop_rule_create(esw: *mut mlx5_eswitch, vport_num: u16) -> c_int;
}

