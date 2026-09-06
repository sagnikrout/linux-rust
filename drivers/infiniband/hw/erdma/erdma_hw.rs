//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/erdma/erdma_hw.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Authors: Cheng Xu <chengyou@linux.alibaba.com>
// Kai Shen <kaishen@linux.alibaba.com>
// Copyright (c) 2020-2022, Alibaba Group.

// PCIe device related definition.
pub const ERDMA_PCI_WIDTH: c_int = 64;
pub const ERDMA_FUNC_BAR: c_int = 0;
pub const ERDMA_MISX_BAR: c_int = 2;

// MSI-X related.

pub const ERDMA_MSIX_VECTOR_CMDQ: c_int = 0;
// RoCEv2 related
pub const ERDMA_ROCEV2_GID_SIZE: c_int = 16;
pub const ERDMA_MAX_PKEYS: c_int = 1;
pub const ERDMA_DEFAULT_PKEY: c_uint = 0xFFFF;
// erdma device protocol type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_proto_type {
    ERDMA_PROTO_IWARP = 0,
    ERDMA_PROTO_ROCEV2 = 1,
    ERDMA_PROTO_COUNT = 2,
}

// PCIe Bar0 Registers.
pub const ERDMA_REGS_VERSION_REG: c_uint = 0x0;
pub const ERDMA_REGS_DEV_PROTO_REG: c_uint = 0xC;
pub const ERDMA_REGS_DEV_CTRL_REG: c_uint = 0x10;
pub const ERDMA_REGS_DEV_ST_REG: c_uint = 0x14;
pub const ERDMA_REGS_NETDEV_MAC_L_REG: c_uint = 0x18;
pub const ERDMA_REGS_NETDEV_MAC_H_REG: c_uint = 0x1C;
pub const ERDMA_REGS_CMDQ_SQ_ADDR_L_REG: c_uint = 0x20;
pub const ERDMA_REGS_CMDQ_SQ_ADDR_H_REG: c_uint = 0x24;
pub const ERDMA_REGS_CMDQ_CQ_ADDR_L_REG: c_uint = 0x28;
pub const ERDMA_REGS_CMDQ_CQ_ADDR_H_REG: c_uint = 0x2C;
pub const ERDMA_REGS_CMDQ_DEPTH_REG: c_uint = 0x30;
pub const ERDMA_REGS_CMDQ_EQ_DEPTH_REG: c_uint = 0x34;
pub const ERDMA_REGS_CMDQ_EQ_ADDR_L_REG: c_uint = 0x38;
pub const ERDMA_REGS_CMDQ_EQ_ADDR_H_REG: c_uint = 0x3C;
pub const ERDMA_REGS_AEQ_ADDR_L_REG: c_uint = 0x40;
pub const ERDMA_REGS_AEQ_ADDR_H_REG: c_uint = 0x44;
pub const ERDMA_REGS_AEQ_DEPTH_REG: c_uint = 0x48;
pub const ERDMA_REGS_GRP_NUM_REG: c_uint = 0x4c;
pub const ERDMA_REGS_AEQ_DB_REG: c_uint = 0x50;
pub const ERDMA_CMDQ_SQ_DB_HOST_ADDR_REG: c_uint = 0x60;
pub const ERDMA_CMDQ_CQ_DB_HOST_ADDR_REG: c_uint = 0x68;
pub const ERDMA_CMDQ_EQ_DB_HOST_ADDR_REG: c_uint = 0x70;
pub const ERDMA_AEQ_DB_HOST_ADDR_REG: c_uint = 0x78;
pub const ERDMA_REGS_STATS_TSO_IN_PKTS_REG: c_uint = 0x80;
pub const ERDMA_REGS_STATS_TSO_OUT_PKTS_REG: c_uint = 0x88;
pub const ERDMA_REGS_STATS_TSO_OUT_BYTES_REG: c_uint = 0x90;
pub const ERDMA_REGS_STATS_TX_DROP_PKTS_REG: c_uint = 0x98;
pub const ERDMA_REGS_STATS_TX_BPS_METER_DROP_PKTS_REG: c_uint = 0xa0;
pub const ERDMA_REGS_STATS_TX_PPS_METER_DROP_PKTS_REG: c_uint = 0xa8;
pub const ERDMA_REGS_STATS_RX_PKTS_REG: c_uint = 0xc0;
pub const ERDMA_REGS_STATS_RX_BYTES_REG: c_uint = 0xc8;
pub const ERDMA_REGS_STATS_RX_DROP_PKTS_REG: c_uint = 0xd0;
pub const ERDMA_REGS_STATS_RX_BPS_METER_DROP_PKTS_REG: c_uint = 0xd8;
pub const ERDMA_REGS_STATS_RX_PPS_METER_DROP_PKTS_REG: c_uint = 0xe0;
pub const ERDMA_REGS_CEQ_DB_BASE_REG: c_uint = 0x100;
pub const ERDMA_CMDQ_SQDB_REG: c_uint = 0x200;
pub const ERDMA_CMDQ_CQDB_REG: c_uint = 0x300;
// DEV_CTRL_REG details.
pub const ERDMA_REG_DEV_CTRL_RESET_MASK: c_uint = 0x00000001;
pub const ERDMA_REG_DEV_CTRL_INIT_MASK: c_uint = 0x00000002;
// DEV_ST_REG details.
pub const ERDMA_REG_DEV_ST_RESET_DONE_MASK: c_uint = 0x00000001U;
pub const ERDMA_REG_DEV_ST_INIT_DONE_MASK: c_uint = 0x00000002U;
// eRDMA PCIe DBs definition.
pub const ERDMA_BAR_DB_SPACE_BASE: c_int = 4096;

pub const ERDMA_SDB_SHARED_PAGE_INDEX: c_int = 95;
// Doorbell related.
pub const ERDMA_DB_SIZE: c_int = 8;

pub const ERDMA_PAGE_SIZE_SUPPORT: c_uint = 0x7FFFF000;
// Hardware page size definition
pub const ERDMA_HW_PAGE_SHIFT: c_int = 12;
pub const ERDMA_HW_PAGE_SIZE: c_int = 4096;
// WQE related.
pub const EQE_SIZE: c_int = 16;
pub const EQE_SHIFT: c_int = 4;
pub const RQE_SIZE: c_int = 32;
pub const RQE_SHIFT: c_int = 5;
pub const CQE_SIZE: c_int = 32;
pub const CQE_SHIFT: c_int = 5;
pub const SQEBB_SIZE: c_int = 32;
pub const SQEBB_SHIFT: c_int = 5;

pub const ERDMA_MAX_SQE_SIZE: c_int = 128;
pub const ERDMA_MAX_WQEBB_PER_SQE: c_int = 4;
// CMDQ related.
pub const ERDMA_CMDQ_MAX_OUTSTANDING: c_int = 128;
pub const ERDMA_CMDQ_SQE_SIZE: c_int = 128;
// cmdq sub module definition.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CMDQ_WQE_SUB_MOD {
    CMDQ_SUBMOD_RDMA = 0,
    CMDQ_SUBMOD_COMMON = 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CMDQ_RDMA_OPCODE {
    CMDQ_OPCODE_QUERY_DEVICE = 0,
    CMDQ_OPCODE_CREATE_QP = 1,
    CMDQ_OPCODE_DESTROY_QP = 2,
    CMDQ_OPCODE_MODIFY_QP = 3,
    CMDQ_OPCODE_CREATE_CQ = 4,
    CMDQ_OPCODE_DESTROY_CQ = 5,
    CMDQ_OPCODE_REFLUSH = 6,
    CMDQ_OPCODE_REG_MR = 8,
    CMDQ_OPCODE_DEREG_MR = 9,
    CMDQ_OPCODE_SET_GID = 14,
    CMDQ_OPCODE_CREATE_AH = 15,
    CMDQ_OPCODE_DESTROY_AH = 16,
    CMDQ_OPCODE_QUERY_QP = 17,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CMDQ_COMMON_OPCODE {
    CMDQ_OPCODE_CREATE_EQ = 0,
    CMDQ_OPCODE_DESTROY_EQ = 1,
    CMDQ_OPCODE_QUERY_FW_INFO = 2,
    CMDQ_OPCODE_CONF_MTU = 3,
    CMDQ_OPCODE_GET_STATS = 4,
    CMDQ_OPCODE_CONF_DEVICE = 5,
    CMDQ_OPCODE_ALLOC_DB = 8,
    CMDQ_OPCODE_FREE_DB = 9,
}

// cmdq-SQE HDR

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_destroy_cq_req {
    pub hdr: u64,
    pub cqn: u32,
}

pub const ERDMA_EQ_TYPE_AEQ: c_int = 0;
pub const ERDMA_EQ_TYPE_CEQ: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_create_eq_req {
    pub hdr: u64,
    pub qbuf_addr: u64,
    pub vector_idx: u8,
    pub eqn: u8,
    pub depth: u8,
    pub qtype: u8,
    pub db_dma_addr_l: u32,
    pub db_dma_addr_h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_destroy_eq_req {
    pub hdr: u64,
    pub rsvd0: u64,
    pub vector_idx: u8,
    pub eqn: u8,
    pub rsvd1: u8,
    pub qtype: u8,
}

// config device cfg

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_config_device_req {
    pub hdr: u64,
    pub cfg: u32,
    pub rsvd: [u32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_config_mtu_req {
    pub hdr: u64,
    pub mtu: u32,
}

// ext db requests(alloc and free) cfg

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_ext_db_req {
    pub hdr: u64,
    pub cfg: u32,
    pub rdb_off: u16,
    pub sdb_off: u16,
    pub rsvd0: u16,
    pub cdb_off: u16,
    pub rsvd1: [u32; 3],
}

// alloc db response qword 0 definition

// create_cq cfg0

// create_cq cfg1

// create_cq cfg2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_create_cq_req {
    pub hdr: u64,
    pub cfg0: u32,
    pub qbuf_addr_l: u32,
    pub qbuf_addr_h: u32,
    pub cfg1: u32,
    pub cq_dbrec_dma: u64,
    pub first_page_offset: u32,
    pub cfg2: u32,
}

// regmr/deregmr cfg0

// regmr cfg1

// regmr cfg2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_reg_mr_req {
    pub hdr: u64,
    pub cfg0: u32,
    pub cfg1: u32,
    pub start_va: u64,
    pub size: u32,
    pub cfg2: u32,
    pub phy_addr: [u64; 4],
    pub rsvd: u64,
    pub size_h: u32,
    pub mtt_cnt_h: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_dereg_mr_req {
    pub hdr: u64,
    pub cfg: u32,
}

// create_av cfg0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_av_cfg {
    pub cfg0: u32,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub sl: u8,
    pub rsvd: u8,
    pub udp_sport: u16,
    pub sgid_index: u16,
    pub dmac: [u8; ETH_ALEN],
    pub padding: [u8; 2],
    pub dgid: [u8; ERDMA_ROCEV2_GID_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_create_ah_req {
    pub hdr: u64,
    pub pdn: u32,
    pub ahn: u32,
    pub av_cfg: erdma_av_cfg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_destroy_ah_req {
    pub hdr: u64,
    pub pdn: u32,
    pub ahn: u32,
}

// modify qp cfg

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_modify_qp_req {
    pub hdr: u64,
    pub cfg: u32,
    pub cookie: u32,
    pub dip: __be32,
    pub sip: __be32,
    pub sport: __be16,
    pub dport: __be16,
    pub send_nxt: u32,
    pub recv_nxt: u32,
}

// modify qp cfg1 for roce device

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_mod_qp_req_rocev2 {
    pub hdr: u64,
    pub cfg0: u32,
    pub cfg1: u32,
    pub attr_mask: u32,
    pub qkey: u32,
    pub rq_psn: u32,
    pub sq_psn: u32,
    pub av_cfg: erdma_av_cfg,
}

// query qp response mask

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_query_qp_req_rocev2 {
    pub hdr: u64,
    pub qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_qp_type {
    ERDMA_QPT_RC = 0,
    ERDMA_QPT_UD = 1,
}

// create qp cfg0

// create qp cfg1

// create qp cfg2

// create qp cqn_mtt_cfg

// create qp mtt_cfg

// create qp db cfg

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_create_qp_req {
    pub hdr: u64,
    pub cfg0: u32,
    pub cfg1: u32,
    pub sq_cqn_mtt_cfg: u32,
    pub rq_cqn_mtt_cfg: u32,
    pub sq_buf_addr: u64,
    pub rq_buf_addr: u64,
    pub sq_mtt_cfg: u32,
    pub rq_mtt_cfg: u32,
    pub sq_dbrec_dma: u64,
    pub rq_dbrec_dma: u64,
    pub sq_mtt_entry: [u64; 3],
    pub rq_mtt_entry: [u64; 3],
    pub db_cfg: u32,
    pub cfg2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_destroy_qp_req {
    pub hdr: u64,
    pub qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_reflush_req {
    pub hdr: u64,
    pub qpn: u32,
    pub sq_pi: u32,
    pub rq_pi: u32,
}

pub const ERDMA_HW_RESP_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_query_req {
    pub hdr: u64,
    pub rsvd: u32,
    pub index: u32,
    pub target_addr: u64,
    pub target_length: u32,
}

pub const ERDMA_HW_RESP_MAGIC: c_uint = 0x5566;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_query_resp_hdr {
    pub magic: u16,
    pub ver: u8,
    pub length: u8,
    pub index: u32,
    pub rsvd: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_query_stats_resp {
    pub hdr: erdma_cmdq_query_resp_hdr,
    pub tx_req_cnt: u64,
    pub tx_packets_cnt: u64,
    pub tx_bytes_cnt: u64,
    pub tx_drop_packets_cnt: u64,
    pub tx_bps_meter_drop_packets_cnt: u64,
    pub tx_pps_meter_drop_packets_cnt: u64,
    pub rx_packets_cnt: u64,
    pub rx_bytes_cnt: u64,
    pub rx_drop_packets_cnt: u64,
    pub rx_bps_meter_drop_packets_cnt: u64,
    pub rx_pps_meter_drop_packets_cnt: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_network_type {
    ERDMA_NETWORK_TYPE_IPV4 = 0,
    ERDMA_NETWORK_TYPE_IPV6 = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_set_gid_op {
    ERDMA_SET_GID_OP_ADD = 0,
    ERDMA_SET_GID_OP_DEL = 1,
}

// set gid cfg

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cmdq_set_gid_req {
    pub hdr: u64,
    pub cfg: u32,
    pub gid: [u8; ERDMA_ROCEV2_GID_SIZE],
}

// cap qword 0 definition

// cap qword 1 definition

pub const ERDMA_NQP_PER_QBLOCK: c_int = 1024;

// CQE hdr

pub const ERDMA_CQE_QTYPE_SQ: c_int = 0;
pub const ERDMA_CQE_QTYPE_RQ: c_int = 1;
pub const ERDMA_CQE_QTYPE_CMDQ: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_cqe {
    pub hdr: __be32,
    pub qe_idx: __be32,
    pub qpn: __be32,
    pub imm_data: __le32,
    pub inv_rkey: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_sge {
    pub addr: __aligned_le64,
    pub length: __le32,
    pub key: __le32,
}

// Receive Queue Element
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_rqe {
    pub qe_idx: __le16,
    pub rsvd0: __le16,
    pub qpn: __le32,
    pub rsvd1: __le32,
    pub rsvd2: __le32,
    pub to: __le64,
    pub length: __le32,
    pub stag: __le32,
}

// SQE

// REG MR attrs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_write_sqe {
    pub hdr: __le64,
    pub imm_data: __be32,
    pub length: __le32,
    pub sink_stag: __le32,
    pub sink_to_l: __le32,
    pub sink_to_h: __le32,
    pub rsvd: __le32,
    pub sgl: [erdma_sge; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_send_sqe_rc {
    pub hdr: __le64,
    pub imm_data: __be32,
    pub invalid_stag: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_send_sqe_ud {
    pub hdr: __le64,
    pub imm_data: __be32,
    pub length: __le32,
    pub qkey: __le32,
    pub dst_qpn: __le32,
    pub ahn: __le32,
    pub rsvd: __le32,
    pub sgl: [erdma_sge; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_readreq_sqe {
    pub hdr: __le64,
    pub invalid_stag: __le32,
    pub length: __le32,
    pub sink_stag: __le32,
    pub sink_to_l: __le32,
    pub sink_to_h: __le32,
    pub rsvd: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_atomic_sqe {
    pub hdr: __le64,
    pub rsvd: __le64,
    pub fetchadd_swap_data: __le64,
    pub cmp_data: __le64,
    pub remote: erdma_sge,
    pub sgl: erdma_sge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_reg_mr_sqe {
    pub hdr: __le64,
    pub addr: __le64,
    pub length: __le32,
    pub stag: __le32,
    pub attrs: __le32,
    pub rsvd: __le32,
}

// EQ related.
pub const ERDMA_DEFAULT_EQ_DEPTH: c_int = 4096;
// ceqe

// aeqe

pub const ERDMA_AE_TYPE_QP_FATAL_EVENT: c_int = 0;
pub const ERDMA_AE_TYPE_QP_ERQ_ERR_EVENT: c_int = 1;
pub const ERDMA_AE_TYPE_ACC_ERR_EVENT: c_int = 2;
pub const ERDMA_AE_TYPE_CQ_ERR: c_int = 3;
pub const ERDMA_AE_TYPE_OTHER_ERROR: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erdma_aeqe {
    pub hdr: __le32,
    pub event_data0: __le32,
    pub event_data1: __le32,
    pub rsvd: __le32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_opcode {
    ERDMA_OP_WRITE = 0,
    ERDMA_OP_READ = 1,
    ERDMA_OP_SEND = 2,
    ERDMA_OP_SEND_WITH_IMM = 3,

    ERDMA_OP_RECEIVE = 4,
    ERDMA_OP_RECV_IMM = 5,
    ERDMA_OP_RECV_INV = 6,

    ERDMA_OP_RSVD0 = 7,
    ERDMA_OP_RSVD1 = 8,
    ERDMA_OP_WRITE_WITH_IMM = 9,

    ERDMA_OP_RSVD2 = 10,
    ERDMA_OP_RSVD3 = 11,

    ERDMA_OP_RSP_SEND_IMM = 12,
    ERDMA_OP_SEND_WITH_INV = 13,

    ERDMA_OP_REG_MR = 14,
    ERDMA_OP_LOCAL_INV = 15,
    ERDMA_OP_READ_WITH_INV = 16,
    ERDMA_OP_ATOMIC_CAS = 17,
    ERDMA_OP_ATOMIC_FAA = 18,
    ERDMA_NUM_OPCODES = 19,
    ERDMA_OP_INVALID = ERDMA_NUM_OPCODES + 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_wc_status {
    ERDMA_WC_SUCCESS = 0,
    ERDMA_WC_GENERAL_ERR = 1,
    ERDMA_WC_RECV_WQE_FORMAT_ERR = 2,
    ERDMA_WC_RECV_STAG_INVALID_ERR = 3,
    ERDMA_WC_RECV_ADDR_VIOLATION_ERR = 4,
    ERDMA_WC_RECV_RIGHT_VIOLATION_ERR = 5,
    ERDMA_WC_RECV_PDID_ERR = 6,
    ERDMA_WC_RECV_WARRPING_ERR = 7,
    ERDMA_WC_SEND_WQE_FORMAT_ERR = 8,
    ERDMA_WC_SEND_WQE_ORD_EXCEED = 9,
    ERDMA_WC_SEND_STAG_INVALID_ERR = 10,
    ERDMA_WC_SEND_ADDR_VIOLATION_ERR = 11,
    ERDMA_WC_SEND_RIGHT_VIOLATION_ERR = 12,
    ERDMA_WC_SEND_PDID_ERR = 13,
    ERDMA_WC_SEND_WARRPING_ERR = 14,
    ERDMA_WC_FLUSH_ERR = 15,
    ERDMA_WC_RETRY_EXC_ERR = 16,
    ERDMA_NUM_WC_STATUS
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum erdma_vendor_err {
    ERDMA_WC_VENDOR_NO_ERR = 0,
    ERDMA_WC_VENDOR_INVALID_RQE = 1,
    ERDMA_WC_VENDOR_RQE_INVALID_STAG = 2,
    ERDMA_WC_VENDOR_RQE_ADDR_VIOLATION = 3,
    ERDMA_WC_VENDOR_RQE_ACCESS_RIGHT_ERR = 4,
    ERDMA_WC_VENDOR_RQE_INVALID_PD = 5,
    ERDMA_WC_VENDOR_RQE_WRAP_ERR = 6,
    ERDMA_WC_VENDOR_INVALID_SQE = 0x20,
    ERDMA_WC_VENDOR_ZERO_ORD = 0x21,
    ERDMA_WC_VENDOR_SQE_INVALID_STAG = 0x30,
    ERDMA_WC_VENDOR_SQE_ADDR_VIOLATION = 0x31,
    ERDMA_WC_VENDOR_SQE_ACCESS_ERR = 0x32,
    ERDMA_WC_VENDOR_SQE_INVALID_PD = 0x33,
    ERDMA_WC_VENDOR_SQE_WARP_ERR = 0x34
}
