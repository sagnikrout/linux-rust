//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/cavium/thunder/q_struct.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// This file contains HW queue descriptor formats, config register
// structures etc
//
// Copyright (C) 2015 Cavium, Inc.
//
// Load transaction types for reading segment bytes specified by
// NIC_SEND_GATHER_S[LD_TYPE].
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nic_send_ld_type_e {
    NIC_SEND_LD_TYPE_E_LDD = 0x0,
    NIC_SEND_LD_TYPE_E_LDT = 0x1,
    NIC_SEND_LD_TYPE_E_LDWB = 0x2,
    NIC_SEND_LD_TYPE_E_ENUM_LAST = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ether_type_algorithm {
    ETYPE_ALG_NONE = 0x0,
    ETYPE_ALG_SKIP = 0x1,
    ETYPE_ALG_ENDPARSE = 0x2,
    ETYPE_ALG_VLAN = 0x3,
    ETYPE_ALG_VLAN_STRIP = 0x4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum layer3_type {
    L3TYPE_NONE = 0x00,
    L3TYPE_GRH = 0x01,
    L3TYPE_IPV4 = 0x04,
    L3TYPE_IPV4_OPTIONS = 0x05,
    L3TYPE_IPV6 = 0x06,
    L3TYPE_IPV6_OPTIONS = 0x07,
    L3TYPE_ET_STOP = 0x0D,
    L3TYPE_OTHER = 0x0E,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum layer4_type {
    L4TYPE_NONE = 0x00,
    L4TYPE_IPSEC_ESP = 0x01,
    L4TYPE_IPFRAG = 0x02,
    L4TYPE_IPCOMP = 0x03,
    L4TYPE_TCP = 0x04,
    L4TYPE_UDP = 0x05,
    L4TYPE_SCTP = 0x06,
    L4TYPE_GRE = 0x07,
    L4TYPE_ROCE_BTH = 0x08,
    L4TYPE_OTHER = 0x0E,
}

// CPI and RSSI configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpi_algorithm_type {
    CPI_ALG_NONE = 0x0,
    CPI_ALG_VLAN = 0x1,
    CPI_ALG_VLAN16 = 0x2,
    CPI_ALG_DIFF = 0x3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rss_algorithm_type {
    RSS_ALG_NONE = 0x00,
    RSS_ALG_PORT = 0x01,
    RSS_ALG_IP = 0x02,
    RSS_ALG_TCP_IP = 0x03,
    RSS_ALG_UDP_IP = 0x04,
    RSS_ALG_SCTP_IP = 0x05,
    RSS_ALG_GRE_IP = 0x06,
    RSS_ALG_ROCE = 0x07,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rss_hash_cfg {
    RSS_HASH_L2ETC = 0x00,
    RSS_HASH_IP = 0x01,
    RSS_HASH_TCP = 0x02,
    RSS_HASH_TCP_SYN_DIS = 0x03,
    RSS_HASH_UDP = 0x04,
    RSS_HASH_L4ETC = 0x05,
    RSS_HASH_ROCE = 0x06,
    RSS_L3_BIDI = 0x07,
    RSS_L4_BIDI = 0x08,
}

// Completion queue entry types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqe_type {
    CQE_TYPE_INVALID = 0x0,
    CQE_TYPE_RX = 0x2,
    CQE_TYPE_RX_SPLIT = 0x3,
    CQE_TYPE_RX_TCP = 0x4,
    CQE_TYPE_SEND = 0x8,
    CQE_TYPE_SEND_PTP = 0x9,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqe_rx_tcp_status {
    CQE_RX_STATUS_VALID_TCP_CNXT = 0x00,
    CQE_RX_STATUS_INVALID_TCP_CNXT = 0x0F,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqe_send_status {
    CQE_SEND_STATUS_GOOD = 0x00,
    CQE_SEND_STATUS_DESC_FAULT = 0x01,
    CQE_SEND_STATUS_HDR_CONS_ERR = 0x11,
    CQE_SEND_STATUS_SUBDESC_ERR = 0x12,
    CQE_SEND_STATUS_IMM_SIZE_OFLOW = 0x80,
    CQE_SEND_STATUS_CRC_SEQ_ERR = 0x81,
    CQE_SEND_STATUS_DATA_SEQ_ERR = 0x82,
    CQE_SEND_STATUS_MEM_SEQ_ERR = 0x83,
    CQE_SEND_STATUS_LOCK_VIOL = 0x84,
    CQE_SEND_STATUS_LOCK_UFLOW = 0x85,
    CQE_SEND_STATUS_DATA_FAULT = 0x86,
    CQE_SEND_STATUS_TSTMP_CONFLICT = 0x87,
    CQE_SEND_STATUS_TSTMP_TIMEOUT = 0x88,
    CQE_SEND_STATUS_MEM_FAULT = 0x89,
    CQE_SEND_STATUS_CSUM_OVERLAP = 0x8A,
    CQE_SEND_STATUS_CSUM_OVERFLOW = 0x8B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqe_rx_tcp_end_reason {
    CQE_RX_TCP_END_FIN_FLAG_DET = 0,
    CQE_RX_TCP_END_INVALID_FLAG = 1,
    CQE_RX_TCP_END_TIMEOUT = 2,
    CQE_RX_TCP_END_OUT_OF_SEQ = 3,
    CQE_RX_TCP_END_PKT_ERR = 4,
    CQE_RX_TCP_END_QS_DISABLED = 0x0F,
}

// Packet protocol level error enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqe_rx_err_level {
    CQE_RX_ERRLVL_RE = 0x0,
    CQE_RX_ERRLVL_L2 = 0x1,
    CQE_RX_ERRLVL_L3 = 0x2,
    CQE_RX_ERRLVL_L4 = 0x3,
}

// Packet protocol level error type enumeration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cqe_rx_err_opcode {
    CQE_RX_ERR_RE_NONE = 0x0,
    CQE_RX_ERR_RE_PARTIAL = 0x1,
    CQE_RX_ERR_RE_JABBER = 0x2,
    CQE_RX_ERR_RE_FCS = 0x7,
    CQE_RX_ERR_RE_TERMINATE = 0x9,
    CQE_RX_ERR_RE_RX_CTL = 0xb,
    CQE_RX_ERR_PREL2_ERR = 0x1f,
    CQE_RX_ERR_L2_FRAGMENT = 0x20,
    CQE_RX_ERR_L2_OVERRUN = 0x21,
    CQE_RX_ERR_L2_PFCS = 0x22,
    CQE_RX_ERR_L2_PUNY = 0x23,
    CQE_RX_ERR_L2_MAL = 0x24,
    CQE_RX_ERR_L2_OVERSIZE = 0x25,
    CQE_RX_ERR_L2_UNDERSIZE = 0x26,
    CQE_RX_ERR_L2_LENMISM = 0x27,
    CQE_RX_ERR_L2_PCLP = 0x28,
    CQE_RX_ERR_IP_NOT = 0x41,
    CQE_RX_ERR_IP_CHK = 0x42,
    CQE_RX_ERR_IP_MAL = 0x43,
    CQE_RX_ERR_IP_MALD = 0x44,
    CQE_RX_ERR_IP_HOP = 0x45,
    CQE_RX_ERR_L3_ICRC = 0x46,
    CQE_RX_ERR_L3_PCLP = 0x47,
    CQE_RX_ERR_L4_MAL = 0x61,
    CQE_RX_ERR_L4_CHK = 0x62,
    CQE_RX_ERR_UDP_LEN = 0x63,
    CQE_RX_ERR_L4_PORT = 0x64,
    CQE_RX_ERR_TCP_FLAG = 0x65,
    CQE_RX_ERR_TCP_OFFSET = 0x66,
    CQE_RX_ERR_L4_PCLP = 0x67,
    CQE_RX_ERR_RBDR_TRUNC = 0x70,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqe_rx_t {

    pub /: *mut *mut u64 cqe_type:4; / W0,
    pub stdn_fault:1: u64,
    pub rsvd0:1: u64,
    pub rq_qs:7: u64,
    pub rq_idx:3: u64,
    pub rsvd1:12: u64,
    pub rss_alg:4: u64,
    pub rsvd2:4: u64,
    pub rb_cnt:4: u64,
    pub vlan_found:1: u64,
    pub vlan_stripped:1: u64,
    pub vlan2_found:1: u64,
    pub vlan2_stripped:1: u64,
    pub l4_type:4: u64,
    pub l3_type:4: u64,
    pub l2_present:1: u64,
    pub err_level:3: u64,
    pub err_opcode:8: u64,
    pub /: *mut *mut u64 pkt_len:16; / W1,
    pub l2_ptr:8: u64,
    pub l3_ptr:8: u64,
    pub l4_ptr:8: u64,
    pub cq_pkt_len:8: u64,
    pub align_pad:3: u64,
    pub rsvd3:1: u64,
    pub chan:12: u64,
    pub /: *mut *mut u64 rss_tag:32; / W2,
    pub vlan_tci:16: u64,
    pub vlan_ptr:8: u64,
    pub vlan2_ptr:8: u64,
    pub /: *mut *mut u64 rb3_sz:16; / W3,
    pub rb2_sz:16: u64,
    pub rb1_sz:16: u64,
    pub rb0_sz:16: u64,
    pub /: *mut *mut u64 rb7_sz:16; / W4,
    pub rb6_sz:16: u64,
    pub rb5_sz:16: u64,
    pub rb4_sz:16: u64,
    pub /: *mut *mut u64 rb11_sz:16; / W5,
    pub rb10_sz:16: u64,
    pub rb9_sz:16: u64,
    pub rb8_sz:16: u64,

    pub err_opcode:8: u64,
    pub err_level:3: u64,
    pub l2_present:1: u64,
    pub l3_type:4: u64,
    pub l4_type:4: u64,
    pub vlan2_stripped:1: u64,
    pub vlan2_found:1: u64,
    pub vlan_stripped:1: u64,
    pub vlan_found:1: u64,
    pub rb_cnt:4: u64,
    pub rsvd2:4: u64,
    pub rss_alg:4: u64,
    pub rsvd1:12: u64,
    pub rq_idx:3: u64,
    pub rq_qs:7: u64,
    pub rsvd0:1: u64,
    pub stdn_fault:1: u64,
    pub /: *mut *mut u64 cqe_type:4; / W0,
    pub chan:12: u64,
    pub rsvd3:1: u64,
    pub align_pad:3: u64,
    pub cq_pkt_len:8: u64,
    pub l4_ptr:8: u64,
    pub l3_ptr:8: u64,
    pub l2_ptr:8: u64,
    pub /: *mut *mut u64 pkt_len:16; / W1,
    pub vlan2_ptr:8: u64,
    pub vlan_ptr:8: u64,
    pub vlan_tci:16: u64,
    pub /: *mut *mut u64 rss_tag:32; / W2,
    pub rb0_sz:16: u64,
    pub rb1_sz:16: u64,
    pub rb2_sz:16: u64,
    pub /: *mut *mut u64 rb3_sz:16; / W3,
    pub rb4_sz:16: u64,
    pub rb5_sz:16: u64,
    pub rb6_sz:16: u64,
    pub /: *mut *mut u64 rb7_sz:16; / W4,
    pub rb8_sz:16: u64,
    pub rb9_sz:16: u64,
    pub rb10_sz:16: u64,
    pub /: *mut *mut u64 rb11_sz:16; / W5,

    pub rb0_ptr:64: u64,
    pub rb1_ptr:64: u64,
    pub rb2_ptr:64: u64,
    pub rb3_ptr:64: u64,
    pub rb4_ptr:64: u64,
    pub rb5_ptr:64: u64,
    pub rb6_ptr:64: u64,
    pub rb7_ptr:64: u64,
    pub rb8_ptr:64: u64,
    pub rb9_ptr:64: u64,
    pub rb10_ptr:64: u64,
    pub rb11_ptr:64: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqe_rx_tcp_err_t {

    pub /: *mut *mut u64 cqe_type:4; / W0,
    pub rsvd0:60: u64,
    pub /: *mut *mut u64 rsvd1:4; / W1,
    pub partial_first:1: u64,
    pub rsvd2:27: u64,
    pub rbdr_bytes:8: u64,
    pub rsvd3:24: u64,

    pub rsvd0:60: u64,
    pub cqe_type:4: u64,
    pub rsvd3:24: u64,
    pub rbdr_bytes:8: u64,
    pub rsvd2:27: u64,
    pub partial_first:1: u64,
    pub rsvd1:4: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqe_rx_tcp_t {

    pub /: *mut *mut u64 cqe_type:4; / W0,
    pub rsvd0:52: u64,
    pub cq_tcp_status:8: u64,
    pub /: *mut *mut u64 rsvd1:32; / W1,
    pub tcp_cntx_bytes:8: u64,
    pub rsvd2:8: u64,
    pub tcp_err_bytes:16: u64,

    pub cq_tcp_status:8: u64,
    pub rsvd0:52: u64,
    pub /: *mut *mut u64 cqe_type:4; / W0,
    pub tcp_err_bytes:16: u64,
    pub rsvd2:8: u64,
    pub tcp_cntx_bytes:8: u64,
    pub /: *mut *mut u64 rsvd1:32; / W1,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cqe_send_t {

    pub /: *mut *mut u64 cqe_type:4; / W0,
    pub rsvd0:4: u64,
    pub sqe_ptr:16: u64,
    pub rsvd1:4: u64,
    pub rsvd2:10: u64,
    pub sq_qs:7: u64,
    pub sq_idx:3: u64,
    pub rsvd3:8: u64,
    pub send_status:8: u64,
    pub /: *mut *mut u64 ptp_timestamp:64; / W1,

    pub send_status:8: u64,
    pub rsvd3:8: u64,
    pub sq_idx:3: u64,
    pub sq_qs:7: u64,
    pub rsvd2:10: u64,
    pub rsvd1:4: u64,
    pub sqe_ptr:16: u64,
    pub rsvd0:4: u64,
    pub /: *mut *mut u64 cqe_type:4; / W0,
    pub /: *mut *mut u64 ptp_timestamp:64; / W1,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cq_desc_t {
    pub u: [u64; 64],
    pub snd_hdr: cqe_send_t,
    pub rx_hdr: cqe_rx_t,
    pub rx_tcp_hdr: cqe_rx_tcp_t,
    pub rx_tcp_err_hdr: cqe_rx_tcp_err_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbdr_entry_t {
    pub buf_addr: u64,
}

// TCP reassembly context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbe_tcp_cnxt_t {

    pub tcp_pkt_cnt:12: u64,
    pub rsvd1:4: u64,
    pub align_hdr_bytes:4: u64,
    pub align_ptr_bytes:4: u64,
    pub ptr_bytes:16: u64,
    pub rsvd2:24: u64,
    pub cqe_type:4: u64,
    pub rsvd0:54: u64,
    pub tcp_end_reason:2: u64,
    pub tcp_status:4: u64,

    pub tcp_status:4: u64,
    pub tcp_end_reason:2: u64,
    pub rsvd0:54: u64,
    pub cqe_type:4: u64,
    pub rsvd2:24: u64,
    pub ptr_bytes:16: u64,
    pub align_ptr_bytes:4: u64,
    pub align_hdr_bytes:4: u64,
    pub rsvd1:4: u64,
    pub tcp_pkt_cnt:12: u64,

}

// Always Big endian
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rx_hdr_t {
    pub opaque:32: u64,
    pub rss_flow:8: u64,
    pub skip_length:6: u64,
    pub disable_rss:1: u64,
    pub disable_tcp_reassembly:1: u64,
    pub nodrop:1: u64,
    pub dest_alg:2: u64,
    pub rsvd0:2: u64,
    pub dest_rq:11: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum send_l4_csum_type {
    SEND_L4_CSUM_DISABLE = 0x00,
    SEND_L4_CSUM_UDP = 0x01,
    SEND_L4_CSUM_TCP = 0x02,
    SEND_L4_CSUM_SCTP = 0x03,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum send_crc_alg {
    SEND_CRCALG_CRC32 = 0x00,
    SEND_CRCALG_CRC32C = 0x01,
    SEND_CRCALG_ICRC = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum send_load_type {
    SEND_LD_TYPE_LDD = 0x00,
    SEND_LD_TYPE_LDT = 0x01,
    SEND_LD_TYPE_LDWB = 0x02,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum send_mem_alg_type {
    SEND_MEMALG_SET = 0x00,
    SEND_MEMALG_ADD = 0x08,
    SEND_MEMALG_SUB = 0x09,
    SEND_MEMALG_ADDLEN = 0x0A,
    SEND_MEMALG_SUBLEN = 0x0B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum send_mem_dsz_type {
    SEND_MEMDSZ_B64 = 0x00,
    SEND_MEMDSZ_B32 = 0x01,
    SEND_MEMDSZ_B8 = 0x03,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sq_subdesc_type {
    SQ_DESC_TYPE_INVALID = 0x00,
    SQ_DESC_TYPE_HEADER = 0x01,
    SQ_DESC_TYPE_CRC = 0x02,
    SQ_DESC_TYPE_IMMEDIATE = 0x03,
    SQ_DESC_TYPE_GATHER = 0x04,
    SQ_DESC_TYPE_MEMORY = 0x05,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_crc_subdesc {

    pub rsvd1:32: u64,
    pub crc_ival:32: u64,
    pub subdesc_type:4: u64,
    pub crc_alg:2: u64,
    pub rsvd0:10: u64,
    pub crc_insert_pos:16: u64,
    pub hdr_start:16: u64,
    pub crc_len:16: u64,

    pub crc_len:16: u64,
    pub hdr_start:16: u64,
    pub crc_insert_pos:16: u64,
    pub rsvd0:10: u64,
    pub crc_alg:2: u64,
    pub subdesc_type:4: u64,
    pub crc_ival:32: u64,
    pub rsvd1:32: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_gather_subdesc {

    pub /: *mut *mut u64 subdesc_type:4; / W0,
    pub ld_type:2: u64,
    pub rsvd0:42: u64,
    pub size:16: u64,
    pub /: *mut *mut u64 rsvd1:15; / W1,
    pub addr:49: u64,

    pub size:16: u64,
    pub rsvd0:42: u64,
    pub ld_type:2: u64,
    pub /: *mut *mut u64 subdesc_type:4; / W0,
    pub addr:49: u64,
    pub /: *mut *mut u64 rsvd1:15; / W1,

}

// SQ immediate subdescriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_imm_subdesc {

    pub /: *mut *mut u64 subdesc_type:4; / W0,
    pub rsvd0:46: u64,
    pub len:14: u64,
    pub /: *mut *mut u64 data:64; / W1,

    pub len:14: u64,
    pub rsvd0:46: u64,
    pub /: *mut *mut u64 subdesc_type:4; / W0,
    pub /: *mut *mut u64 data:64; / W1,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_mem_subdesc {

    pub /: *mut *mut u64 subdesc_type:4; / W0,
    pub mem_alg:4: u64,
    pub mem_dsz:2: u64,
    pub wmem:1: u64,
    pub rsvd0:21: u64,
    pub offset:32: u64,
    pub /: *mut *mut u64 rsvd1:15; / W1,
    pub addr:49: u64,

    pub offset:32: u64,
    pub rsvd0:21: u64,
    pub wmem:1: u64,
    pub mem_dsz:2: u64,
    pub mem_alg:4: u64,
    pub /: *mut *mut u64 subdesc_type:4; / W0,
    pub addr:49: u64,
    pub /: *mut *mut u64 rsvd1:15; / W1,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_hdr_subdesc {

    pub subdesc_type:4: u64,
    pub tso:1: u64,
    pub /: *mut *mut u64 post_cqe:1; / Post CQE on no error also,
    pub dont_send:1: u64,
    pub tstmp:1: u64,
    pub subdesc_cnt:8: u64,
    pub csum_l4:2: u64,
    pub csum_l3:1: u64,
    pub csum_inner_l4:2: u64,
    pub csum_inner_l3:1: u64,
    pub rsvd0:2: u64,
    pub l4_offset:8: u64,
    pub l3_offset:8: u64,
    pub rsvd1:4: u64,
    pub /: *mut *mut u64 tot_len:20; / W0,
    pub rsvd2:24: u64,
    pub inner_l4_offset:8: u64,
    pub inner_l3_offset:8: u64,
    pub tso_start:8: u64,
    pub rsvd3:2: u64,
    pub /: *mut *mut u64 tso_max_paysize:14; / W1,

    pub tot_len:20: u64,
    pub rsvd1:4: u64,
    pub l3_offset:8: u64,
    pub l4_offset:8: u64,
    pub rsvd0:2: u64,
    pub csum_inner_l3:1: u64,
    pub csum_inner_l4:2: u64,
    pub csum_l3:1: u64,
    pub csum_l4:2: u64,
    pub subdesc_cnt:8: u64,
    pub tstmp:1: u64,
    pub dont_send:1: u64,
    pub /: *mut *mut u64 post_cqe:1; / Post CQE on no error also,
    pub tso:1: u64,
    pub /: *mut *mut u64 subdesc_type:4; / W0,
    pub tso_max_paysize:14: u64,
    pub rsvd3:2: u64,
    pub tso_start:8: u64,
    pub inner_l3_offset:8: u64,
    pub inner_l4_offset:8: u64,
    pub /: *mut *mut u64 rsvd2:24; / W1,

}

// Queue config register formats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rq_cfg {

    pub reserved_2_63:62: u64,
    pub ena:1: u64,
    pub tcp_ena:1: u64,

    pub tcp_ena:1: u64,
    pub ena:1: u64,
    pub reserved_2_63:62: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cq_cfg {

    pub reserved_43_63:21: u64,
    pub ena:1: u64,
    pub reset:1: u64,
    pub caching:1: u64,
    pub reserved_35_39:5: u64,
    pub qsize:3: u64,
    pub reserved_25_31:7: u64,
    pub avg_con:9: u64,
    pub reserved_0_15:16: u64,

    pub reserved_0_15:16: u64,
    pub avg_con:9: u64,
    pub reserved_25_31:7: u64,
    pub qsize:3: u64,
    pub reserved_35_39:5: u64,
    pub caching:1: u64,
    pub reset:1: u64,
    pub ena:1: u64,
    pub reserved_43_63:21: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sq_cfg {

    pub reserved_32_63:32: u64,
    pub cq_limit:8: u64,
    pub reserved_20_23:4: u64,
    pub ena:1: u64,
    pub reserved_18_18:1: u64,
    pub reset:1: u64,
    pub ldwb:1: u64,
    pub reserved_11_15:5: u64,
    pub qsize:3: u64,
    pub reserved_3_7:5: u64,
    pub tstmp_bgx_intf:3: u64,

    pub tstmp_bgx_intf:3: u64,
    pub reserved_3_7:5: u64,
    pub qsize:3: u64,
    pub reserved_11_15:5: u64,
    pub ldwb:1: u64,
    pub reset:1: u64,
    pub reserved_18_18:1: u64,
    pub ena:1: u64,
    pub reserved_20_23:4: u64,
    pub cq_limit:8: u64,
    pub reserved_32_63:32: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rbdr_cfg {

    pub reserved_45_63:19: u64,
    pub ena:1: u64,
    pub reset:1: u64,
    pub ldwb:1: u64,
    pub reserved_36_41:6: u64,
    pub qsize:4: u64,
    pub reserved_25_31:7: u64,
    pub avg_con:9: u64,
    pub reserved_12_15:4: u64,
    pub lines:12: u64,

    pub lines:12: u64,
    pub reserved_12_15:4: u64,
    pub avg_con:9: u64,
    pub reserved_25_31:7: u64,
    pub qsize:4: u64,
    pub reserved_36_41:6: u64,
    pub ldwb:1: u64,
    pub reset:1: u64,
    pub 1: u64 ena:,
    pub reserved_45_63:19: u64,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qs_cfg {

    pub reserved_32_63:32: u64,
    pub ena:1: u64,
    pub reserved_27_30:4: u64,
    pub sq_ins_ena:1: u64,
    pub sq_ins_pos:6: u64,
    pub lock_ena:1: u64,
    pub lock_viol_cqe_ena:1: u64,
    pub send_tstmp_ena:1: u64,
    pub be:1: u64,
    pub reserved_7_15:9: u64,
    pub vnic:7: u64,

    pub vnic:7: u64,
    pub reserved_7_15:9: u64,
    pub be:1: u64,
    pub send_tstmp_ena:1: u64,
    pub lock_viol_cqe_ena:1: u64,
    pub lock_ena:1: u64,
    pub sq_ins_pos:6: u64,
    pub sq_ins_ena:1: u64,
    pub reserved_27_30:4: u64,
    pub ena:1: u64,
    pub reserved_32_63:32: u64,

}
