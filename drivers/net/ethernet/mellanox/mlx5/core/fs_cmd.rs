//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/fs_cmd.h
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
// Copyright (c) 2015, Mellanox Technologies. All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_flow_cmds {
    pub next_ft): *mut mlx5_flow_table,
    pub ft): *mut mlx5_flow_table,
    pub next_ft): *mut mlx5_flow_table,
    pub fg): *mut mlx5_flow_group,
    pub fg): *mut mlx5_flow_group,
    pub fte): *mut fs_fte,
    pub fte): *mut fs_fte,
    pub fte): *mut fs_fte,
    pub disconnect): bool,
    pub pkt_reformat): *mut mlx5_pkt_reformat,
    pub pkt_reformat): *mut mlx5_pkt_reformat,
    pub modify_hdr): *mut mlx5_modify_hdr,
    pub modify_hdr): *mut mlx5_modify_hdr,
    pub peer_vhca_id): u16,
    pub ns): *mut *mut int (create_ns)(struct mlx5_flow_root_namespace,
    pub ns): *mut *mut int (destroy_ns)(struct mlx5_flow_root_namespace,
    pub match_mask): *mut u16 format_id, u32,
    pub definer_id): c_int,
    pub ft_type): fs_flow_table_type,
}

extern "C" {
    pub fn mlx5_cmd_fc_alloc(dev: *mut mlx5_core_dev, id: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_fc_free(dev: *mut mlx5_core_dev, id: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_fc_get_bulk_query_out_len(bulk_len: c_int) -> c_int;
}
extern "C" {
    pub fn mlx5_fs_cmd_set_l2table_entry_silent(dev: *mut mlx5_core_dev, silent_mode: u8) -> c_int;
}
extern "C" {
    pub fn mlx5_fs_cmd_set_tx_flow_table_root(dev: *mut mlx5_core_dev, ft_id: u32, disconnect: bool) -> c_int;
}
