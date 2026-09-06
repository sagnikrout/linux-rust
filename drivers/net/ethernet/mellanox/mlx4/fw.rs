//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx4/fw.h
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
// Copyright (c) 2004, 2005 Topspin Communications.  All rights reserved.
// Copyright (c) 2005, 2006, 2007, 2008 Mellanox Technologies. All rights reserved.
// Copyright (c) 2006, 2007 Cisco Systems.  All rights reserved.
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
pub struct mlx4_mod_stat_cfg {
    pub log_pg_sz: u8,
    pub log_pg_sz_m: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_port_cap {
    pub link_state: u8,
    pub supported_port_types: u8,
    pub suggested_type: u8,
    pub default_sense: u8,
    pub log_max_macs: u8,
    pub log_max_vlans: u8,
    pub ib_mtu: c_int,
    pub max_port_width: c_int,
    pub max_vl: c_int,
    pub max_tc_eth: c_int,
    pub max_gids: c_int,
    pub max_pkeys: c_int,
    pub def_mac: u64,
    pub eth_mtu: u16,
    pub trans_type: c_int,
    pub vendor_oui: c_int,
    pub wavelength: u16,
    pub trans_code: u64,
    pub dmfs_optimized_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_dev_cap {
    pub max_srq_sz: c_int,
    pub max_qp_sz: c_int,
    pub reserved_qps: c_int,
    pub max_qps: c_int,
    pub reserved_srqs: c_int,
    pub max_srqs: c_int,
    pub max_cq_sz: c_int,
    pub reserved_cqs: c_int,
    pub max_cqs: c_int,
    pub max_mpts: c_int,
    pub reserved_eqs: c_int,
    pub max_eqs: c_int,
    pub num_sys_eqs: c_int,
    pub reserved_mtts: c_int,
    pub reserved_mrws: c_int,
    pub max_requester_per_qp: c_int,
    pub max_responder_per_qp: c_int,
    pub max_rdma_global: c_int,
    pub local_ca_ack_delay: c_int,
    pub num_ports: c_int,
    pub max_msg_sz: u32,
    pub stat_rate_support: u16,
    pub fs_log_max_ucast_qp_range_size: c_int,
    pub fs_max_num_qp_per_entry: c_int,
    pub flags: u64,
    pub flags2: u64,
    pub reserved_uars: c_int,
    pub uar_size: c_int,
    pub min_page_sz: c_int,
    pub bf_reg_size: c_int,
    pub bf_regs_per_page: c_int,
    pub max_sq_sg: c_int,
    pub max_sq_desc_sz: c_int,
    pub max_rq_sg: c_int,
    pub max_rq_desc_sz: c_int,
    pub max_qp_per_mcg: c_int,
    pub reserved_mgms: c_int,
    pub max_mcgs: c_int,
    pub reserved_pds: c_int,
    pub max_pds: c_int,
    pub reserved_xrcds: c_int,
    pub max_xrcds: c_int,
    pub qpc_entry_sz: c_int,
    pub rdmarc_entry_sz: c_int,
    pub altc_entry_sz: c_int,
    pub aux_entry_sz: c_int,
    pub srq_entry_sz: c_int,
    pub cqc_entry_sz: c_int,
    pub eqc_entry_sz: c_int,
    pub dmpt_entry_sz: c_int,
    pub cmpt_entry_sz: c_int,
    pub mtt_entry_sz: c_int,
    pub resize_srq: c_int,
    pub bmme_flags: u32,
    pub reserved_lkey: u32,
    pub max_icm_sz: u64,
    pub max_gso_sz: c_int,
    pub max_rss_tbl_sz: c_int,
    pub max_counters: u32,
    pub dmfs_high_rate_qpn_base: u32,
    pub dmfs_high_rate_qpn_range: u32,
    pub rl_caps: mlx4_rate_limit_caps,
    pub health_buffer_addrs: u32,
    pub 1]: mlx4_port_cap port_cap[MLX4_MAX_PORTS +,
    pub 1]: bool wol_port[MLX4_MAX_PORTS +,
    pub map_clock_to_user: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_func_cap {
    pub num_ports: u8,
    pub flags: u8,
    pub pf_context_behaviour: u32,
    pub qp_quota: c_int,
    pub cq_quota: c_int,
    pub srq_quota: c_int,
    pub mpt_quota: c_int,
    pub mtt_quota: c_int,
    pub max_eq: c_int,
    pub reserved_eq: c_int,
    pub mcg_quota: c_int,
    pub spec_qps: mlx4_spec_qps,
    pub reserved_lkey: u32,
    pub physical_port: u8,
    pub flags0: u8,
    pub flags1: u8,
    pub phys_port_id: u64,
    pub extra_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_func {
    pub bus: c_int,
    pub device: c_int,
    pub function: c_int,
    pub physical_function: c_int,
    pub rsvd_eqs: c_int,
    pub max_eq: c_int,
    pub rsvd_uars: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_adapter {
    pub board_id: [c_char; MLX4_BOARD_ID_LEN],
    pub inta_pin: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_init_hca_param {
    pub qpc_base: u64,
    pub rdmarc_base: u64,
    pub auxc_base: u64,
    pub altc_base: u64,
    pub srqc_base: u64,
    pub cqc_base: u64,
    pub eqc_base: u64,
    pub mc_base: u64,
    pub dmpt_base: u64,
    pub cmpt_base: u64,
    pub mtt_base: u64,
    pub global_caps: u64,
    pub log_mc_entry_sz: u8,
    pub log_mc_hash_sz: u8,
    pub /: *mut *mut u16 hca_core_clock; / Internal Clock Frequency (in MHz),
    pub log_num_qps: u8,
    pub log_num_srqs: u8,
    pub log_num_cqs: u8,
    pub log_num_eqs: u8,
    pub num_sys_eqs: u16,
    pub log_rd_per_qp: u8,
    pub log_mc_table_sz: u8,
    pub log_mpt_sz: u8,
    pub log_uar_sz: u8,
    pub /: *mut *mut u8 mw_enabled; / Enable memory windows,
    pub /: *mut *mut u8 uar_page_sz; / log pg sz in 4k chunks,
    pub /: *mut *mut u8 steering_mode; / for QUERY_HCA,
    pub /: *mut *mut u8 dmfs_high_steer_mode; / for QUERY_HCA,
    pub dev_cap_enabled: u64,
    pub /: *mut *mut u16 cqe_size; / For use only when CQE stride feature enabled,
    pub /: *mut *mut u16 eqe_size; / For use only when EQE stride feature enabled,
    pub rss_ip_frags: u8,
    pub /: *mut *mut u8 phv_check_en; / for QUERY_HCA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_init_ib_param {
    pub port_width: c_int,
    pub vl_cap: c_int,
    pub mtu_cap: c_int,
    pub gid_cap: u16,
    pub pkey_cap: u16,
    pub set_guid0: c_int,
    pub guid0: u64,
    pub set_node_guid: c_int,
    pub node_guid: u64,
    pub set_si_guid: c_int,
    pub si_guid: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_set_ib_param {
    pub set_si_guid: c_int,
    pub reset_qkey_viol: c_int,
    pub si_guid: u64,
    pub cap_mask: u32,
}

extern "C" {
    pub fn mlx4_dev_cap_dump(dev: *mut mlx4_dev, dev_cap: *mut mlx4_dev_cap);
}
extern "C" {
    pub fn mlx4_QUERY_DEV_CAP(dev: *mut mlx4_dev, dev_cap: *mut mlx4_dev_cap) -> c_int;
}
extern "C" {
    pub fn mlx4_QUERY_PORT(dev: *mut mlx4_dev, port: c_int, port_cap: *mut mlx4_port_cap) -> c_int;
}
extern "C" {
    pub fn mlx4_QUERY_FUNC(dev: *mut mlx4_dev, func: *mut mlx4_func, slave: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_MAP_FA(dev: *mut mlx4_dev, icm: *mut mlx4_icm) -> c_int;
}
extern "C" {
    pub fn mlx4_UNMAP_FA(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_RUN_FW(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_QUERY_FW(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_QUERY_ADAPTER(dev: *mut mlx4_dev, adapter: *mut mlx4_adapter) -> c_int;
}
extern "C" {
    pub fn mlx4_INIT_HCA(dev: *mut mlx4_dev, param: *mut mlx4_init_hca_param) -> c_int;
}
extern "C" {
    pub fn mlx4_QUERY_HCA(dev: *mut mlx4_dev, param: *mut mlx4_init_hca_param) -> c_int;
}
extern "C" {
    pub fn mlx4_CLOSE_HCA(dev: *mut mlx4_dev, panic: c_int) -> c_int;
}
extern "C" {
    pub fn mlx4_map_cmd(dev: *mut mlx4_dev, op: u16, icm: *mut mlx4_icm, virt: u64) -> c_int;
}
extern "C" {
    pub fn mlx4_SET_ICM_SIZE(dev: *mut mlx4_dev, icm_size: u64, aux_pages: *mut u64) -> c_int;
}
extern "C" {
    pub fn mlx4_MAP_ICM_AUX(dev: *mut mlx4_dev, icm: *mut mlx4_icm) -> c_int;
}
extern "C" {
    pub fn mlx4_UNMAP_ICM_AUX(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_NOP(dev: *mut mlx4_dev) -> c_int;
}
extern "C" {
    pub fn mlx4_MOD_STAT_CFG(dev: *mut mlx4_dev, cfg: *mut mlx4_mod_stat_cfg) -> c_int;
}
extern "C" {
    pub fn mlx4_opreq_action(work: *mut work_struct);
}
