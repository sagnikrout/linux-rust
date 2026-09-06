//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/send.h
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
// Copyright (c) 2024 NVIDIA Corporation & Affiliates
// As a single operation requires at least two WQEBBS.
// This means a maximum of 16 such operations per rule.
//
pub const MAX_WQES_PER_RULE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_wqe_opcode {
    MLX5HWS_WQE_OPCODE_TBL_ACCESS = 0x2c,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_wqe_opmod {
    MLX5HWS_WQE_OPMOD_GTA_STE = 0,
    MLX5HWS_WQE_OPMOD_GTA_MOD_ARG = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_wqe_gta_opcode {
    MLX5HWS_WQE_GTA_OP_ACTIVATE = 0,
    MLX5HWS_WQE_GTA_OP_DEACTIVATE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_wqe_gta_opmod {
    MLX5HWS_WQE_GTA_OPMOD_STE = 0,
    MLX5HWS_WQE_GTA_OPMOD_MOD_ARG = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_wqe_gta_sz {
    MLX5HWS_WQE_SZ_GTA_CTRL = 48,
    MLX5HWS_WQE_SZ_GTA_DATA = 64,
}

// WQE Control segment.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_wqe_ctrl_seg {
    pub opmod_idx_opcode: __be32,
    pub qpn_ds: __be32,
    pub flags: __be32,
    pub imm: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_wqe_gta_ctrl_seg {
    pub op_dirix: __be32,
    pub stc_ix: [__be32; 5],
    pub rsvd0: [__be32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_wqe_gta_data_seg_ste {
    pub rsvd0_ctr_id: __be32,
    pub rsvd1_definer: __be32,
    pub rsvd2: [__be32; 3],
    pub action: [__be32; 3],
    pub tag: [__be32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_wqe_gta_data_seg_arg {
    pub action_args: [__be32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_wqe_gta {
    pub gta_ctrl: mlx5hws_wqe_gta_ctrl_seg,
    pub seg_ste: mlx5hws_wqe_gta_data_seg_ste,
    pub seg_arg: mlx5hws_wqe_gta_data_seg_arg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_ring_cq {
    pub mdev: *mut mlx5_core_dev,
    pub wq: mlx5_cqwq,
    pub wq_ctrl: mlx5_wq_ctrl,
    pub mcq: mlx5_core_cq,
    pub poll_wqe: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_ring_priv {
    pub rule: *mut mlx5hws_rule,
    pub user_data: *mut c_void,
    pub num_wqebbs: u32,
    pub id: u32,
    pub retry_id: u32,
    pub used_id: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_ring_dep_wqe {
    pub wqe_ctrl: mlx5hws_wqe_gta_ctrl_seg,
    pub wqe_data: mlx5hws_wqe_gta_data_seg_ste,
    pub rule: *mut mlx5hws_rule,
    pub rtc_0: u32,
    pub rtc_1: u32,
    pub retry_rtc_0: u32,
    pub retry_rtc_1: u32,
    pub direct_index: u32,
    pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_ring_sq {
    pub mdev: *mut mlx5_core_dev,
    pub cur_post: u16,
    pub buf_mask: u16,
    pub wr_priv: *mut mlx5hws_send_ring_priv,
    pub last_idx: c_uint,
    pub dep_wqe: *mut mlx5hws_send_ring_dep_wqe,
    pub head_dep_idx: c_uint,
    pub tail_dep_idx: c_uint,
    pub sqn: u32,
    pub wq: mlx5_wq_cyc,
    pub wq_ctrl: mlx5_wq_ctrl,
    pub uar_map: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_ring {
    pub send_cq: mlx5hws_send_ring_cq,
    pub send_sq: mlx5hws_send_ring_sq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_completed_poll_entry {
    pub user_data: *mut c_void,
    pub status: mlx5hws_flow_op_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_completed_poll {
    pub entries: *mut mlx5hws_completed_poll_entry,
    pub ci: u16,
    pub pi: u16,
    pub mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_engine {
    pub send_ring: mlx5hws_send_ring,
    pub /: *mut *mut *mut mlx5_uars_page uar; / Uar is shared between rings of a queue,
    pub completed: mlx5hws_completed_poll,
    pub used_entries: u16,
    pub num_entries: u16,
    pub err: bool,
    pub error_cqe_printed: bool,
    pub /: *mut *mut mutex lock; / Protects the send engine,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_engine_post_ctrl {
    pub queue: *mut mlx5hws_send_engine,
    pub send_ring: *mut mlx5hws_send_ring,
    pub num_wqebbs: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_engine_post_attr {
    pub opcode: u8,
    pub opmod: u8,
    pub notify_hw: u8,
    pub fence: u8,
    pub match_definer_id: u8,
    pub range_definer_id: u8,
    pub len: usize,
    pub rule: *mut mlx5hws_rule,
    pub id: u32,
    pub retry_id: u32,
    pub used_id: *mut u32,
    pub user_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5hws_send_ste_attr {
    pub rtc_0: u32,
    pub rtc_1: u32,
    pub retry_rtc_0: u32,
    pub retry_rtc_1: u32,
    pub used_id_rtc_0: *mut u32,
    pub used_id_rtc_1: *mut u32,
    pub wqe_tag_is_jumbo: bool,
    pub gta_opcode: u8,
    pub direct_index: u32,
    pub send_attr: mlx5hws_send_engine_post_attr,
    pub wqe_tag: *mut mlx5hws_rule_match_tag,
    pub range_wqe_tag: *mut mlx5hws_rule_match_tag,
    pub wqe_ctrl: *mut mlx5hws_wqe_gta_ctrl_seg,
    pub wqe_data: *mut mlx5hws_wqe_gta_data_seg_ste,
    pub range_wqe_data: *mut mlx5hws_wqe_gta_data_seg_ste,
}

extern "C" {
    pub fn mlx5hws_send_abort_new_dep_wqe(queue: *mut mlx5hws_send_engine);
}
extern "C" {
    pub fn mlx5hws_send_all_dep_wqe(queue: *mut mlx5hws_send_engine);
}
extern "C" {
    pub fn mlx5hws_send_queues_close(ctx: *mut mlx5hws_context);
}
extern "C" {
    pub fn mlx5hws_send_engine_flush_queue(queue: *mut mlx5hws_send_engine);
}
