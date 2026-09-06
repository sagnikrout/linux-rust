//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/transobj.h
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

extern "C" {
    pub fn mlx5_core_alloc_transport_domain(dev: *mut mlx5_core_dev, tdn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_dealloc_transport_domain(dev: *mut mlx5_core_dev, tdn: u32);
}
extern "C" {
    pub fn mlx5_core_modify_rq(dev: *mut mlx5_core_dev, rqn: u32, in: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_destroy_rq(dev: *mut mlx5_core_dev, rqn: u32);
}
extern "C" {
    pub fn mlx5_core_query_rq(dev: *mut mlx5_core_dev, rqn: u32, out: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_modify_sq(dev: *mut mlx5_core_dev, sqn: u32, in: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_destroy_sq(dev: *mut mlx5_core_dev, sqn: u32);
}
extern "C" {
    pub fn mlx5_core_query_sq(dev: *mut mlx5_core_dev, sqn: u32, out: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_query_sq_state(dev: *mut mlx5_core_dev, sqn: u32, state: *mut u8) -> c_int;
}
extern "C" {
    pub fn mlx5_core_create_tir(dev: *mut mlx5_core_dev, in: *mut u32, tirn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_modify_tir(dev: *mut mlx5_core_dev, tirn: u32, in: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_destroy_tir(dev: *mut mlx5_core_dev, tirn: u32);
}
extern "C" {
    pub fn mlx5_core_create_tis(dev: *mut mlx5_core_dev, in: *mut u32, tisn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_modify_tis(dev: *mut mlx5_core_dev, tisn: u32, in: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_destroy_tis(dev: *mut mlx5_core_dev, tisn: u32);
}
extern "C" {
    pub fn mlx5_core_destroy_rqt(dev: *mut mlx5_core_dev, rqtn: u32);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_hairpin_params {
    pub log_data_size: u8,
    pub log_num_packets: u8,
    pub q_counter: u16,
    pub num_channels: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_hairpin {
    pub func_mdev: *mut mlx5_core_dev,
    pub peer_mdev: *mut mlx5_core_dev,
    pub num_channels: c_int,
    pub rqn: *mut u32,
    pub sqn: *mut u32,
    pub peer_gone: bool,
}

extern "C" {
    pub fn mlx5_core_hairpin_destroy(pair: *mut mlx5_hairpin);
}
extern "C" {
    pub fn mlx5_core_hairpin_clear_dead_peer(hp: *mut mlx5_hairpin);
}
