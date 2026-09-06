//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/vmw_pvrdma/pvrdma_verbs.h
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
// Copyright (c) 2012-2016 VMware, Inc.  All rights reserved.
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of EITHER the GNU General Public License
// version 2 as published by the Free Software Foundation or the BSD
// 2-Clause License. This program is distributed in the hope that it
// will be useful, but WITHOUT ANY WARRANTY; WITHOUT EVEN THE IMPLIED
// WARRANTY OF MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU General Public License version 2 for more details at
// http://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html.
//
// You should have received a copy of the GNU General Public License
// along with this program available in the file COPYING in the main
// directory of this source tree.
//
// The BSD 2-Clause License
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
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
// FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
// COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT,
// INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED
// OF THE POSSIBILITY OF SUCH DAMAGE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union pvrdma_gid {
    pub raw: [u8; 16],
    pub subnet_prefix: __be64,
    pub interface_id: __be64,
    pub global: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_link_layer {
    PVRDMA_LINK_LAYER_UNSPECIFIED,
    PVRDMA_LINK_LAYER_INFINIBAND,
    PVRDMA_LINK_LAYER_ETHERNET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_mtu {
    PVRDMA_MTU_256  = 1,
    PVRDMA_MTU_512  = 2,
    PVRDMA_MTU_1024 = 3,
    PVRDMA_MTU_2048 = 4,
    PVRDMA_MTU_4096 = 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_port_state {
    PVRDMA_PORT_NOP			= 0,
    PVRDMA_PORT_DOWN		= 1,
    PVRDMA_PORT_INIT		= 2,
    PVRDMA_PORT_ARMED		= 3,
    PVRDMA_PORT_ACTIVE		= 4,
    PVRDMA_PORT_ACTIVE_DEFER	= 5,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_port_cap_flags {
    PVRDMA_PORT_SM				= 1 <<  1,
    PVRDMA_PORT_NOTICE_SUP			= 1 <<  2,
    PVRDMA_PORT_TRAP_SUP			= 1 <<  3,
    PVRDMA_PORT_OPT_IPD_SUP			= 1 <<  4,
    PVRDMA_PORT_AUTO_MIGR_SUP		= 1 <<  5,
    PVRDMA_PORT_SL_MAP_SUP			= 1 <<  6,
    PVRDMA_PORT_MKEY_NVRAM			= 1 <<  7,
    PVRDMA_PORT_PKEY_NVRAM			= 1 <<  8,
    PVRDMA_PORT_LED_INFO_SUP		= 1 <<  9,
    PVRDMA_PORT_SM_DISABLED			= 1 << 10,
    PVRDMA_PORT_SYS_IMAGE_GUID_SUP		= 1 << 11,
    PVRDMA_PORT_PKEY_SW_EXT_PORT_TRAP_SUP	= 1 << 12,
    PVRDMA_PORT_EXTENDED_SPEEDS_SUP		= 1 << 14,
    PVRDMA_PORT_CM_SUP			= 1 << 16,
    PVRDMA_PORT_SNMP_TUNNEL_SUP		= 1 << 17,
    PVRDMA_PORT_REINIT_SUP			= 1 << 18,
    PVRDMA_PORT_DEVICE_MGMT_SUP		= 1 << 19,
    PVRDMA_PORT_VENDOR_CLASS_SUP		= 1 << 20,
    PVRDMA_PORT_DR_NOTICE_SUP		= 1 << 21,
    PVRDMA_PORT_CAP_MASK_NOTICE_SUP		= 1 << 22,
    PVRDMA_PORT_BOOT_MGMT_SUP		= 1 << 23,
    PVRDMA_PORT_LINK_LATENCY_SUP		= 1 << 24,
    PVRDMA_PORT_CLIENT_REG_SUP		= 1 << 25,
    PVRDMA_PORT_IP_BASED_GIDS		= 1 << 26,
    PVRDMA_PORT_CAP_FLAGS_MAX		= PVRDMA_PORT_IP_BASED_GIDS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_port_width {
    PVRDMA_WIDTH_1X		= 1,
    PVRDMA_WIDTH_4X		= 2,
    PVRDMA_WIDTH_8X		= 4,
    PVRDMA_WIDTH_12X	= 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_port_speed {
    PVRDMA_SPEED_SDR	= 1,
    PVRDMA_SPEED_DDR	= 2,
    PVRDMA_SPEED_QDR	= 4,
    PVRDMA_SPEED_FDR10	= 8,
    PVRDMA_SPEED_FDR	= 16,
    PVRDMA_SPEED_EDR	= 32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_port_attr {
    pub state: pvrdma_port_state,
    pub max_mtu: pvrdma_mtu,
    pub active_mtu: pvrdma_mtu,
    pub gid_tbl_len: u32,
    pub port_cap_flags: u32,
    pub max_msg_sz: u32,
    pub bad_pkey_cntr: u32,
    pub qkey_viol_cntr: u32,
    pub pkey_tbl_len: u16,
    pub lid: u16,
    pub sm_lid: u16,
    pub lmc: u8,
    pub max_vl_num: u8,
    pub sm_sl: u8,
    pub subnet_timeout: u8,
    pub init_type_reply: u8,
    pub active_width: u8,
    pub active_speed: u8,
    pub phys_state: u8,
    pub reserved: [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_global_route {
    pub dgid: pvrdma_gid,
    pub flow_label: u32,
    pub sgid_index: u8,
    pub hop_limit: u8,
    pub traffic_class: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_grh {
    pub version_tclass_flow: __be32,
    pub paylen: __be16,
    pub next_hdr: u8,
    pub hop_limit: u8,
    pub sgid: pvrdma_gid,
    pub dgid: pvrdma_gid,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_ah_flags {
    PVRDMA_AH_GRH = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_rate {
    PVRDMA_RATE_PORT_CURRENT	= 0,
    PVRDMA_RATE_2_5_GBPS		= 2,
    PVRDMA_RATE_5_GBPS		= 5,
    PVRDMA_RATE_10_GBPS		= 3,
    PVRDMA_RATE_20_GBPS		= 6,
    PVRDMA_RATE_30_GBPS		= 4,
    PVRDMA_RATE_40_GBPS		= 7,
    PVRDMA_RATE_60_GBPS		= 8,
    PVRDMA_RATE_80_GBPS		= 9,
    PVRDMA_RATE_120_GBPS		= 10,
    PVRDMA_RATE_14_GBPS		= 11,
    PVRDMA_RATE_56_GBPS		= 12,
    PVRDMA_RATE_112_GBPS		= 13,
    PVRDMA_RATE_168_GBPS		= 14,
    PVRDMA_RATE_25_GBPS		= 15,
    PVRDMA_RATE_100_GBPS		= 16,
    PVRDMA_RATE_200_GBPS		= 17,
    PVRDMA_RATE_300_GBPS		= 18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_ah_attr {
    pub grh: pvrdma_global_route,
    pub dlid: u16,
    pub vlan_id: u16,
    pub sl: u8,
    pub src_path_bits: u8,
    pub static_rate: u8,
    pub ah_flags: u8,
    pub port_num: u8,
    pub dmac: [u8; 6],
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_cq_notify_flags {
    PVRDMA_CQ_SOLICITED		= 1 << 0,
    PVRDMA_CQ_NEXT_COMP		= 1 << 1,
    PVRDMA_CQ_SOLICITED_MASK	= PVRDMA_CQ_SOLICITED |
    PVRDMA_CQ_NEXT_COMP,
    PVRDMA_CQ_REPORT_MISSED_EVENTS	= 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_qp_cap {
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_inline_data: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_sig_type {
    PVRDMA_SIGNAL_ALL_WR,
    PVRDMA_SIGNAL_REQ_WR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_qp_type {
    PVRDMA_QPT_SMI,
    PVRDMA_QPT_GSI,
    PVRDMA_QPT_RC,
    PVRDMA_QPT_UC,
    PVRDMA_QPT_UD,
    PVRDMA_QPT_RAW_IPV6,
    PVRDMA_QPT_RAW_ETHERTYPE,
    PVRDMA_QPT_RAW_PACKET = 8,
    PVRDMA_QPT_XRC_INI = 9,
    PVRDMA_QPT_XRC_TGT,
    PVRDMA_QPT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_qp_create_flags {
    PVRDMA_QP_CREATE_IPOPVRDMA_UD_LSO		= 1 << 0,
    PVRDMA_QP_CREATE_BLOCK_MULTICAST_LOOPBACK	= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_qp_attr_mask {
    PVRDMA_QP_STATE			= 1 << 0,
    PVRDMA_QP_CUR_STATE		= 1 << 1,
    PVRDMA_QP_EN_SQD_ASYNC_NOTIFY	= 1 << 2,
    PVRDMA_QP_ACCESS_FLAGS		= 1 << 3,
    PVRDMA_QP_PKEY_INDEX		= 1 << 4,
    PVRDMA_QP_PORT			= 1 << 5,
    PVRDMA_QP_QKEY			= 1 << 6,
    PVRDMA_QP_AV			= 1 << 7,
    PVRDMA_QP_PATH_MTU		= 1 << 8,
    PVRDMA_QP_TIMEOUT		= 1 << 9,
    PVRDMA_QP_RETRY_CNT		= 1 << 10,
    PVRDMA_QP_RNR_RETRY		= 1 << 11,
    PVRDMA_QP_RQ_PSN		= 1 << 12,
    PVRDMA_QP_MAX_QP_RD_ATOMIC	= 1 << 13,
    PVRDMA_QP_ALT_PATH		= 1 << 14,
    PVRDMA_QP_MIN_RNR_TIMER		= 1 << 15,
    PVRDMA_QP_SQ_PSN		= 1 << 16,
    PVRDMA_QP_MAX_DEST_RD_ATOMIC	= 1 << 17,
    PVRDMA_QP_PATH_MIG_STATE	= 1 << 18,
    PVRDMA_QP_CAP			= 1 << 19,
    PVRDMA_QP_DEST_QPN		= 1 << 20,
    PVRDMA_QP_ATTR_MASK_MAX		= PVRDMA_QP_DEST_QPN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_qp_state {
    PVRDMA_QPS_RESET,
    PVRDMA_QPS_INIT,
    PVRDMA_QPS_RTR,
    PVRDMA_QPS_RTS,
    PVRDMA_QPS_SQD,
    PVRDMA_QPS_SQE,
    PVRDMA_QPS_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_mig_state {
    PVRDMA_MIG_MIGRATED,
    PVRDMA_MIG_REARM,
    PVRDMA_MIG_ARMED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_mw_type {
    PVRDMA_MW_TYPE_1 = 1,
    PVRDMA_MW_TYPE_2 = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_srq_attr {
    pub max_wr: u32,
    pub max_sge: u32,
    pub srq_limit: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvrdma_qp_attr {
    pub qp_state: pvrdma_qp_state,
    pub cur_qp_state: pvrdma_qp_state,
    pub path_mtu: pvrdma_mtu,
    pub path_mig_state: pvrdma_mig_state,
    pub qkey: u32,
    pub rq_psn: u32,
    pub sq_psn: u32,
    pub dest_qp_num: u32,
    pub qp_access_flags: u32,
    pub pkey_index: u16,
    pub alt_pkey_index: u16,
    pub en_sqd_async_notify: u8,
    pub sq_draining: u8,
    pub max_rd_atomic: u8,
    pub max_dest_rd_atomic: u8,
    pub min_rnr_timer: u8,
    pub port_num: u8,
    pub timeout: u8,
    pub retry_cnt: u8,
    pub rnr_retry: u8,
    pub alt_port_num: u8,
    pub alt_timeout: u8,
    pub reserved: [u8; 5],
    pub cap: pvrdma_qp_cap,
    pub ah_attr: pvrdma_ah_attr,
    pub alt_ah_attr: pvrdma_ah_attr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_send_flags {
    PVRDMA_SEND_FENCE	= 1 << 0,
    PVRDMA_SEND_SIGNALED	= 1 << 1,
    PVRDMA_SEND_SOLICITED	= 1 << 2,
    PVRDMA_SEND_INLINE	= 1 << 3,
    PVRDMA_SEND_IP_CSUM	= 1 << 4,
    PVRDMA_SEND_FLAGS_MAX	= PVRDMA_SEND_IP_CSUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pvrdma_access_flags {
    PVRDMA_ACCESS_LOCAL_WRITE	= 1 << 0,
    PVRDMA_ACCESS_REMOTE_WRITE	= 1 << 1,
    PVRDMA_ACCESS_REMOTE_READ	= 1 << 2,
    PVRDMA_ACCESS_REMOTE_ATOMIC	= 1 << 3,
    PVRDMA_ACCESS_MW_BIND		= 1 << 4,
    PVRDMA_ZERO_BASED		= 1 << 5,
    PVRDMA_ACCESS_ON_DEMAND		= 1 << 6,
    PVRDMA_ACCESS_FLAGS_MAX		= PVRDMA_ACCESS_ON_DEMAND,
}

extern "C" {
    pub fn pvrdma_mmap(context: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn pvrdma_alloc_ucontext(uctx: *mut ib_ucontext, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn pvrdma_dealloc_ucontext(context: *mut ib_ucontext);
}
extern "C" {
    pub fn pvrdma_alloc_pd(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn pvrdma_dealloc_pd(ibpd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn pvrdma_dereg_mr(mr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn pvrdma_destroy_cq(cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn pvrdma_poll_cq(ibcq: *mut ib_cq, num_entries: c_int, wc: *mut ib_wc) -> c_int;
}
extern "C" {
    pub fn pvrdma_req_notify_cq(cq: *mut ib_cq, flags: ib_cq_notify_flags) -> c_int;
}
extern "C" {
    pub fn pvrdma_destroy_ah(ah: *mut ib_ah, flags: u32) -> c_int;
}
extern "C" {
    pub fn pvrdma_query_srq(srq: *mut ib_srq, srq_attr: *mut ib_srq_attr) -> c_int;
}
extern "C" {
    pub fn pvrdma_destroy_srq(srq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn pvrdma_destroy_qp(qp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
