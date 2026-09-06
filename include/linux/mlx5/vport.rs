//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/vport.h
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
// Copyright (c) 2013-2015, Mellanox Technologies, Ltd.  All rights reserved.
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

pub const MLX5_MAX_TX_SPEED_UNIT: c_int = 100;
// Vport number for each function must keep unchanged
extern "C" {
    pub fn mlx5_query_vport_state(mdev: *mut mlx5_core_dev, opmod: u8, vport: u16) -> u8;
}
extern "C" {
    pub fn mlx5_query_mac_address(mdev: *mut mlx5_core_dev, addr: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_query_min_inline(mdev: *mut mlx5_core_dev, min_inline: *mut u8);
}
extern "C" {
    pub fn mlx5_query_nic_vport_mtu(mdev: *mut mlx5_core_dev, mtu: *mut u16) -> c_int;
}
extern "C" {
    pub fn mlx5_modify_nic_vport_mtu(mdev: *mut mlx5_core_dev, mtu: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_nic_vport_enable_roce(mdev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_nic_vport_disable_roce(mdev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_nic_vport_update_local_lb(mdev: *mut mlx5_core_dev, enable: bool) -> c_int;
}
extern "C" {
    pub fn mlx5_nic_vport_query_local_lb(mdev: *mut mlx5_core_dev, status: *mut bool) -> c_int;
}
extern "C" {
    pub fn mlx5_nic_vport_unaffiliate_multiport(port_mdev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_query_nic_system_image_guid(mdev: *mut mlx5_core_dev) -> u64;
}
extern "C" {
    pub fn mlx5_vport_get_vhca_id(dev: *mut mlx5_core_dev, vport: u16, vhca_id: *mut u16) -> c_int;
}
