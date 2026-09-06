//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/device.h
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
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
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

pub const MLX5_SET_HOST_ENDIANNESS: c_int = 0;

pub const MLX5_SET_HOST_ENDIANNESS: c_uint = 0x80;

// helper macros

// insert a value to a struct

// ((__be32 *)(p) + __mlx5_dw_off(typ, fld)) = \

// ((__be32 *)(p) + __mlx5_dw_off(typ, fld)) = \

// ((__be64 *)(p) + __mlx5_64_off(typ, fld)) = cpu_to_be64(v); \

// ((__be16 *)(p) + __mlx5_16_off(typ, fld)) = \
// Big endian getters

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_inline_modes {
    MLX5_INLINE_MODE_NONE,
    MLX5_INLINE_MODE_L2,
    MLX5_INLINE_MODE_IP,
    MLX5_INLINE_MODE_TCP_UDP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wqe_page_fault_type {
    MLX5_WQE_PF_TYPE_RMP = 0,
    MLX5_WQE_PF_TYPE_REQ_SEND_OR_WRITE = 1,
    MLX5_WQE_PF_TYPE_RESP = 2,
    MLX5_WQE_PF_TYPE_REQ_READ_OR_ATOMIC = 3,
}

pub const MLX5_UMR_FLEX_ALIGNMENT: c_uint = 0x40;

// mlx5 components can subscribe to any one of these events via
// mlx5_eq_notifier_register API.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_event {
// Special value to subscribe to any event
    MLX5_EVENT_TYPE_NOTIFY_ANY	   = 0x0,
// HW events enum start: comp events are not subscribable
    MLX5_EVENT_TYPE_COMP		   = 0x0,
// HW Async events enum start: subscribable events
    MLX5_EVENT_TYPE_PATH_MIG	   = 0x01,
    MLX5_EVENT_TYPE_COMM_EST	   = 0x02,
    MLX5_EVENT_TYPE_SQ_DRAINED	   = 0x03,
    MLX5_EVENT_TYPE_SRQ_LAST_WQE	   = 0x13,
    MLX5_EVENT_TYPE_SRQ_RQ_LIMIT	   = 0x14,

    MLX5_EVENT_TYPE_CQ_ERROR	   = 0x04,
    MLX5_EVENT_TYPE_WQ_CATAS_ERROR	   = 0x05,
    MLX5_EVENT_TYPE_PATH_MIG_FAILED	   = 0x07,
    MLX5_EVENT_TYPE_WQ_INVAL_REQ_ERROR = 0x10,
    MLX5_EVENT_TYPE_WQ_ACCESS_ERROR	   = 0x11,
    MLX5_EVENT_TYPE_SRQ_CATAS_ERROR	   = 0x12,
    MLX5_EVENT_TYPE_OBJECT_CHANGE	   = 0x27,

    MLX5_EVENT_TYPE_INTERNAL_ERROR	   = 0x08,
    MLX5_EVENT_TYPE_PORT_CHANGE	   = 0x09,
    MLX5_EVENT_TYPE_GPIO_EVENT	   = 0x15,
    MLX5_EVENT_TYPE_PORT_MODULE_EVENT  = 0x16,
    MLX5_EVENT_TYPE_TEMP_WARN_EVENT    = 0x17,
    MLX5_EVENT_TYPE_XRQ_ERROR	   = 0x18,
    MLX5_EVENT_TYPE_REMOTE_CONFIG	   = 0x19,
    MLX5_EVENT_TYPE_GENERAL_EVENT	   = 0x22,
    MLX5_EVENT_TYPE_MONITOR_COUNTER    = 0x24,
    MLX5_EVENT_TYPE_PPS_EVENT          = 0x25,

    MLX5_EVENT_TYPE_DB_BF_CONGESTION   = 0x1a,
    MLX5_EVENT_TYPE_STALL_EVENT	   = 0x1b,

    MLX5_EVENT_TYPE_CMD		   = 0x0a,
    MLX5_EVENT_TYPE_PAGE_REQUEST	   = 0xb,

    MLX5_EVENT_TYPE_PAGE_FAULT	   = 0xc,
    MLX5_EVENT_TYPE_NIC_VPORT_CHANGE   = 0xd,

    MLX5_EVENT_TYPE_ESW_FUNCTIONS_CHANGED = 0xe,
    MLX5_EVENT_TYPE_VHCA_STATE_CHANGE = 0xf,

    MLX5_EVENT_TYPE_DCT_DRAINED        = 0x1c,
    MLX5_EVENT_TYPE_DCT_KEY_VIOLATION  = 0x1d,

    MLX5_EVENT_TYPE_FPGA_ERROR         = 0x20,
    MLX5_EVENT_TYPE_FPGA_QP_ERROR      = 0x21,

    MLX5_EVENT_TYPE_DEVICE_TRACER      = 0x26,

    MLX5_EVENT_TYPE_MAX                = 0x100,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_driver_event {
    MLX5_DRIVER_EVENT_TYPE_TRAP = 0,
    MLX5_DRIVER_EVENT_UPLINK_NETDEV,
    MLX5_DRIVER_EVENT_MACSEC_SA_ADDED,
    MLX5_DRIVER_EVENT_MACSEC_SA_DELETED,
    MLX5_DRIVER_EVENT_SF_PEER_DEVLINK,
    MLX5_DRIVER_EVENT_AFFILIATION_DONE,
    MLX5_DRIVER_EVENT_AFFILIATION_REMOVED,
    MLX5_DRIVER_EVENT_ACTIVE_BACKUP_LAG_CHANGE_LOWERSTATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_tls_static_params_seg {
    pub ctx: [u8; MLX5_ST_SZ_BYTES(tls_static_params)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_tls_progress_params_seg {
    pub tis_tir_num: __be32,
    pub ctx: [u8; MLX5_ST_SZ_BYTES(tls_progress_params)],
}

//
// Max wqe size for rdma read is 512 bytes, so this
// limits our max_sge_rd as the wqe needs to fit:
// - ctrl segment (16 bytes)
// - rdma segment (16 bytes)
// - scatter elements (16 bytes each)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_odp_transport_cap_bits {
    MLX5_ODP_SUPPORT_SEND	 = 1 << 31,
    MLX5_ODP_SUPPORT_RECV	 = 1 << 30,
    MLX5_ODP_SUPPORT_WRITE	 = 1 << 29,
    MLX5_ODP_SUPPORT_READ	 = 1 << 28,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_odp_caps {
    pub reserved: [c_char; 0x10],
    pub rc_odp_caps: __be32,
    pub uc_odp_caps: __be32,
    pub ud_odp_caps: __be32,
    pub per_transport_caps: },
    pub reserved2: [c_char; 0xe4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_layout {
    pub type: u8,
    pub rsvd0: [u8; 3],
    pub inlen: __be32,
    pub in_ptr: __be64,
    pub in: [__be32; 4],
    pub out: [__be32; 4],
    pub out_ptr: __be64,
    pub outlen: __be32,
    pub token: u8,
    pub sig: u8,
    pub rsvd1: u8,
    pub status_own: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_rfr_severity_bit_offsets {
    MLX5_CRR_BIT_OFFSET = 0x6,
    MLX5_RFR_BIT_OFFSET = 0x7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct health_buffer {
    pub assert_var: [__be32; 6],
    pub rsvd0: [__be32; 2],
    pub assert_exit_ptr: __be32,
    pub assert_callra: __be32,
    pub rsvd1: [__be32; 1],
    pub time: __be32,
    pub fw_ver: __be32,
    pub hw_id: __be32,
    pub rfr_severity: u8,
    pub rsvd2: [u8; 3],
    pub irisc_index: u8,
    pub synd: u8,
    pub ext_synd: __be16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_initializing_bit_offsets {
    MLX5_FW_RESET_SUPPORTED_OFFSET = 30,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_cmd_addr_l_sz_offset {
    MLX5_NIC_IFC_OFFSET = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_init_seg {
    pub fw_rev: __be32,
    pub cmdif_rev_fw_sub: __be32,
    pub rsvd0: [__be32; 2],
    pub cmdq_addr_h: __be32,
    pub cmdq_addr_l_sz: __be32,
    pub cmd_dbell: __be32,
    pub rsvd1: [__be32; 120],
    pub initializing: __be32,
    pub health: health_buffer,
    pub rsvd2: [__be32; 878],
    pub cmd_exec_to: __be32,
    pub cmd_q_init_to: __be32,
    pub internal_timer_h: __be32,
    pub internal_timer_l: __be32,
    pub rsvd3: [__be32; 2],
    pub health_counter: __be32,
    pub rsvd4: [__be32; 11],
    pub real_time_h: __be32,
    pub real_time_l: __be32,
    pub rsvd5: [__be32; 1006],
    pub ieee1588_clk: __be64,
    pub ieee1588_clk_type: __be32,
    pub clr_intx: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_comp {
    pub reserved: [__be32; 6],
    pub cqn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_qp_srq {
    pub reserved1: [__be32; 5],
    pub type: u8,
    pub reserved2: [u8; 3],
    pub qp_srq_n: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_cq_err {
    pub cqn: __be32,
    pub reserved1: [u8; 7],
    pub syndrome: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_xrq_err {
    pub reserved1: [__be32; 5],
    pub type_xrqn: __be32,
    pub reserved2: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_port_state {
    pub reserved0: [u8; 8],
    pub port: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_gpio {
    pub reserved0: [__be32; 2],
    pub gpio_event: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_congestion {
    pub type: u8,
    pub rsvd0: u8,
    pub congestion_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_stall_vl {
    pub rsvd0: [u8; 3],
    pub port_vl: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_cmd {
    pub vector: __be32,
    pub rsvd: [__be32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_page_req {
    pub ec_function: __be16,
    pub func_id: __be16,
    pub num_pages: __be32,
    pub rsvd1: [__be32; 5],
}

pub const MEMORY_SCHEME_PAGE_FAULT_GRANULARITY: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_page_fault {
    pub bytes_committed: __be32,
    pub reserved1: u16,
    pub wqe_index: __be16,
    pub reserved2: u16,
    pub packet_length: __be16,
    pub token: __be32,
    pub reserved4: [u8; 8],
    pub pftype_wq: __be32,
    pub wqe: } __packed,
    pub bytes_committed: __be32,
    pub r_key: __be32,
    pub reserved1: u16,
    pub packet_length: __be16,
    pub rdma_op_len: __be32,
    pub rdma_va: __be64,
    pub pftype_token: __be32,
    pub rdma: } __packed,
    pub flags: u8,
    pub reserved1: u8,
    pub post_demand_fault_pages: __be16,
    pub pre_demand_fault_pages: __be16,
    pub token47_32: __be16,
    pub token31_0: __be32,
//
// FW changed from specifying the fault size in byte
// count to 4k pages granularity. The size specified
// in pages uses bits 31:12, to keep backward
// compatibility.
//
    pub demand_fault_pages: __be32,
    pub mkey: __be32,
    pub va: __be64,
    pub memory: } __packed,
    pub __packed: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_vport_change {
    pub rsvd0: [u8; 2],
    pub vport_num: __be16,
    pub rsvd1: [__be32; 6],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_port_module {
    pub reserved_at_0: [u8; 1],
    pub module: u8,
    pub reserved_at_2: [u8; 1],
    pub module_status: u8,
    pub reserved_at_4: [u8; 2],
    pub error_type: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_pps {
    pub rsvd0: [u8; 3],
    pub pin: u8,
    pub rsvd1: [u8; 4],
    pub time_sec: __be32,
    pub time_nsec: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_dct {
    pub reserved: [__be32; 6],
    pub dctn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_temp_warning {
    pub sensor_warning_msb: __be64,
    pub sensor_warning_lsb: __be64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_obj_change {
    pub rsvd0: [u8; 2],
    pub obj_type: __be16,
    pub obj_id: __be32,
    pub __packed: },
pub const SYNC_RST_STATE_MASK: c_uint = 0xf;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sync_rst_state_type {
    MLX5_SYNC_RST_STATE_RESET_REQUEST	= 0x0,
    MLX5_SYNC_RST_STATE_RESET_NOW		= 0x1,
    MLX5_SYNC_RST_STATE_RESET_ABORT		= 0x2,
    MLX5_SYNC_RST_STATE_RESET_UNLOAD	= 0x3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_sync_fw_update {
    pub reserved_at_0: [u8; 3],
    pub sync_rst_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe_vhca_state {
    pub ec_function: __be16,
    pub function_id: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union ev_data {
    pub raw: [__be32; 7],
    pub cmd: mlx5_eqe_cmd,
    pub comp: mlx5_eqe_comp,
    pub qp_srq: mlx5_eqe_qp_srq,
    pub cq_err: mlx5_eqe_cq_err,
    pub port: mlx5_eqe_port_state,
    pub gpio: mlx5_eqe_gpio,
    pub cong: mlx5_eqe_congestion,
    pub stall_vl: mlx5_eqe_stall_vl,
    pub req_pages: mlx5_eqe_page_req,
    pub page_fault: mlx5_eqe_page_fault,
    pub vport_change: mlx5_eqe_vport_change,
    pub port_module: mlx5_eqe_port_module,
    pub pps: mlx5_eqe_pps,
    pub dct: mlx5_eqe_dct,
    pub temp_warning: mlx5_eqe_temp_warning,
    pub xrq_err: mlx5_eqe_xrq_err,
    pub sync_fw_update: mlx5_eqe_sync_fw_update,
    pub vhca_state: mlx5_eqe_vhca_state,
    pub obj_change: mlx5_eqe_obj_change,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_eqe {
    pub rsvd0: u8,
    pub type: u8,
    pub rsvd1: u8,
    pub sub_type: u8,
    pub rsvd2: [__be32; 7],
    pub data: ev_data,
    pub rsvd3: __be16,
    pub signature: u8,
    pub owner: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cmd_prot_block {
    pub data: [u8; MLX5_CMD_DATA_BLOCK_SIZE],
    pub rsvd0: [u8; 48],
    pub next: __be64,
    pub block_num: __be32,
    pub rsvd1: u8,
    pub token: u8,
    pub ctrl_sig: u8,
    pub sig: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_err_cqe {
    pub rsvd0: [u8; 32],
    pub srqn: __be32,
    pub rsvd1: [u8; 18],
    pub vendor_err_synd: u8,
    pub syndrome: u8,
    pub s_wqe_opcode_qpn: __be32,
    pub wqe_counter: __be16,
    pub signature: u8,
    pub op_own: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cqe64 {
    pub tls_outer_l3_tunneled: u8,
    pub rsvd0: u8,
    pub wqe_id: __be16,
    pub tcppsh_abort_dupack: u8,
    pub min_ttl: u8,
    pub tcp_win: __be16,
    pub ack_seq_num: __be32,
    pub lro: },
    pub reserved0:1: u8,
    pub match:1: u8,
    pub flush:1: u8,
    pub reserved3:5: u8,
    pub header_size: u8,
    pub header_entry_index: __be16,
    pub data_offset: __be32,
    pub shampo: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_mini_cqe8 {
    pub rx_hash_result: __be32,
    pub checksum: __be16,
    pub stridx: __be16,
}

pub const MLX5_MINI_CQE_ARRAY_SIZE: c_int = 8;
// num_of_mini_cqes is zero based
pub const MLX5_MPWQE_LOG_NUM_STRIDES_EXT_BASE: c_int = 3;
pub const MLX5_MPWQE_LOG_NUM_STRIDES_BASE: c_int = 9;
pub const MLX5_MPWQE_LOG_NUM_STRIDES_MAX: c_int = 16;
pub const MLX5_MPWQE_LOG_STRIDE_SZ_BASE: c_int = 6;
pub const MLX5_MPWQE_LOG_STRIDE_SZ_MAX: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpwrq_cqe_bc {
    pub filler_consumed_strides: __be16,
    pub byte_cnt: __be16,
}

extern "C" {
    pub fn be16_to_cpu(_arg: bc->byte_cnt) -> return;
}
extern "C" {
    pub fn mpwrq_get_cqe_bc_consumed_strides(_arg: bc) -> return;
}
extern "C" {
    pub fn be16_to_cpu(_arg: cqe->wqe_counter) -> return;
}
// cqe->rss_hash_type[3:2] - IP destination selected for hash
// (00 = none,  01 = IPv4, 10 = IPv6, 11 = Reserved)
//
// cqe->rss_hash_type[7:6] - L4 destination selected for hash
// (00 = none, 01 = TCP. 10 = UDP, 11 = IPSEC.SPI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_sig_err_cqe {
    pub rsvd0: [u8; 16],
    pub expected_trans_sig: __be32,
    pub actual_trans_sig: __be32,
    pub expected_reftag: __be32,
    pub actual_reftag: __be32,
    pub syndrome: __be16,
    pub rsvd22: [u8; 2],
    pub mkey: __be32,
    pub err_offset: __be64,
    pub rsvd30: [u8; 8],
    pub qpn: __be32,
    pub rsvd38: [u8; 2],
    pub signature: u8,
    pub op_own: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_srq_next_seg {
    pub rsvd0: [u8; 2],
    pub next_wqe_index: __be16,
    pub signature: u8,
    pub rsvd1: [u8; 11],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mlx5_ext_cqe {
    pub grh: ib_grh,
    pub inl: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_cqe128 {
    pub inl_grh: mlx5_ext_cqe,
    pub cqe64: mlx5_cqe64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_mkey_seg {
// This is a two bit field occupying bits 31-30.
// bit 31 is always 0,
// bit 30 is zero for regular MRs and 1 (e.g free) for UMRs that do not have translation
//
    pub status: u8,
    pub pcie_control: u8,
    pub flags: u8,
    pub version: u8,
    pub qpn_mkey7_0: __be32,
    pub rsvd1: [u8; 4],
    pub flags_pd: __be32,
    pub start_addr: __be64,
    pub len: __be64,
    pub bsfs_octo_size: __be32,
    pub rsvd2: [u8; 16],
    pub xlt_oct_size: __be32,
    pub rsvd3: [u8; 3],
    pub log2_page_size: u8,
    pub rsvd4: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_list_type {
    MLX5_NVPRT_LIST_TYPE_UC   = 0x0,
    MLX5_NVPRT_LIST_TYPE_MC   = 0x1,
    MLX5_NVPRT_LIST_TYPE_VLAN = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_wol_mode {
    MLX5_WOL_DISABLE        = 0,
    MLX5_WOL_SECURED_MAGIC  = 1 << 1,
    MLX5_WOL_MAGIC          = 1 << 2,
    MLX5_WOL_ARP            = 1 << 3,
    MLX5_WOL_BROADCAST      = 1 << 4,
    MLX5_WOL_MULTICAST      = 1 << 5,
    MLX5_WOL_UNICAST        = 1 << 6,
    MLX5_WOL_PHY_ACTIVITY   = 1 << 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_mpls_supported_fields {
    MLX5_FIELD_SUPPORT_MPLS_LABEL = 1 << 0,
    MLX5_FIELD_SUPPORT_MPLS_EXP   = 1 << 1,
    MLX5_FIELD_SUPPORT_MPLS_S_BOS = 1 << 2,
    MLX5_FIELD_SUPPORT_MPLS_TTL   = 1 << 3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_flex_parser_protos {
    MLX5_FLEX_PROTO_GENEVE	      = 1 << 3,
    MLX5_FLEX_PROTO_CW_MPLS_GRE   = 1 << 4,
    MLX5_FLEX_PROTO_CW_MPLS_UDP   = 1 << 5,
    MLX5_FLEX_PROTO_ICMP	      = 1 << 8,
    MLX5_FLEX_PROTO_ICMPV6	      = 1 << 9,
}

// MLX5 DEV CAPs
// TODO: EAT.ME
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_cap_mode {
    HCA_CAP_OPMOD_GET_MAX	= 0,
    HCA_CAP_OPMOD_GET_CUR	= 1,
}

// Any new cap addition must update mlx5_hca_caps_alloc() to allocate
// capability memory.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_cap_type {
    MLX5_CAP_GENERAL = 0,
    MLX5_CAP_ETHERNET_OFFLOADS,
    MLX5_CAP_ODP,
    MLX5_CAP_ATOMIC,
    MLX5_CAP_ROCE,
    MLX5_CAP_IPOIB_OFFLOADS,
    MLX5_CAP_IPOIB_ENHANCED_OFFLOADS,
    MLX5_CAP_FLOW_TABLE,
    MLX5_CAP_ESWITCH_FLOW_TABLE,
    MLX5_CAP_ESWITCH,
    MLX5_CAP_QOS = 0xc,
    MLX5_CAP_DEBUG,
    MLX5_CAP_RESERVED_14,
    MLX5_CAP_DEV_MEM,
    MLX5_CAP_RESERVED_16,
    MLX5_CAP_TLS,
    MLX5_CAP_VDPA_EMULATION = 0x13,
    MLX5_CAP_DEV_EVENT = 0x14,
    MLX5_CAP_IPSEC,
    MLX5_CAP_CRYPTO = 0x1a,
    MLX5_CAP_SHAMPO = 0x1d,
    MLX5_CAP_PSP = 0x1e,
    MLX5_CAP_MACSEC = 0x1f,
    MLX5_CAP_GENERAL_2 = 0x20,
    MLX5_CAP_PORT_SELECTION = 0x25,
    MLX5_CAP_ADV_VIRTUALIZATION = 0x26,
    MLX5_CAP_ADV_RDMA = 0x28,
    MLX5_CAP_TLP_EMULATION = 0x2a,
// NUM OF CAP Types
    MLX5_CAP_NUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_pcam_reg_groups {
    MLX5_PCAM_REGS_5000_TO_507F                 = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_pcam_feature_groups {
    MLX5_PCAM_FEATURE_ENHANCED_FEATURES         = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_mcam_reg_groups {
    MLX5_MCAM_REGS_FIRST_128                    = 0x0,
    MLX5_MCAM_REGS_0x9100_0x917F                = 0x2,
    MLX5_MCAM_REGS_0x9180_0x91FF                = 0x3,
    MLX5_MCAM_REGS_NUM                          = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_mcam_feature_groups {
    MLX5_MCAM_FEATURE_ENHANCED_FEATURES         = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_qcam_reg_groups {
    MLX5_QCAM_REGS_FIRST_128                    = 0x0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_qcam_feature_groups {
    MLX5_QCAM_FEATURE_ENHANCED_FEATURES         = 0x0,
}

// GET Dev Caps macros

pub const MLX5_RDMA_RX_NUM_COUNTERS_PRIOS: c_int = 6;
pub const MLX5_RDMA_TX_NUM_COUNTERS_PRIOS: c_int = 4;
pub const MLX5_BY_PASS_NUM_REGULAR_PRIOS: c_int = 16;
pub const MLX5_BY_PASS_NUM_DONT_TRAP_PRIOS: c_int = 16;
pub const MLX5_BY_PASS_NUM_MULTICAST_PRIOS: c_int = 1;

