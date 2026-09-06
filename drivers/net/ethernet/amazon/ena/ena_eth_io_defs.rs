//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/amazon/ena/ena_eth_io_defs.h
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
//
// Copyright 2015-2020 Amazon.com, Inc. or its affiliates. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_eth_io_l3_proto_index {
    ENA_ETH_IO_L3_PROTO_UNKNOWN                 = 0,
    ENA_ETH_IO_L3_PROTO_IPV4                    = 8,
    ENA_ETH_IO_L3_PROTO_IPV6                    = 11,
    ENA_ETH_IO_L3_PROTO_FCOE                    = 21,
    ENA_ETH_IO_L3_PROTO_ROCE                    = 22,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ena_eth_io_l4_proto_index {
    ENA_ETH_IO_L4_PROTO_UNKNOWN                 = 0,
    ENA_ETH_IO_L4_PROTO_TCP                     = 12,
    ENA_ETH_IO_L4_PROTO_UDP                     = 13,
    ENA_ETH_IO_L4_PROTO_ROUTEABLE_ROCE          = 23,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_tx_desc {
// 15:0 : length - Buffer length in bytes, must
// include any packet trailers that the ENA supposed
// to update like End-to-End CRC, Authentication GMAC
// etc. This length must not include the
// 'Push_Buffer' length. This length must not include
// the 4-byte added in the end for 802.3 Ethernet FCS
// 21:16 : req_id_hi - Request ID[15:10]
// 22 : reserved22 - MBZ
// 23 : meta_desc - MBZ
// 24 : phase
// 25 : reserved1 - MBZ
// 26 : first - Indicates first descriptor in
// transaction
// 27 : last - Indicates last descriptor in
// transaction
// 28 : comp_req - Indicates whether completion
// should be posted, after packet is transmitted.
// Valid only for first descriptor
// 30:29 : reserved29 - MBZ
// 31 : reserved31 - MBZ
//
    pub len_ctrl: u32,
// 3:0 : l3_proto_idx - L3 protocol. This field
// required when l3_csum_en,l3_csum or tso_en are set.
// 4 : DF - IPv4 DF, must be 0 if packet is IPv4 and
// DF flags of the IPv4 header is 0. Otherwise must
// be set to 1
// 6:5 : reserved5
// 7 : tso_en - Enable TSO, For TCP only.
// 12:8 : l4_proto_idx - L4 protocol. This field need
// to be set when l4_csum_en or tso_en are set.
// 13 : l3_csum_en - enable IPv4 header checksum.
// 14 : l4_csum_en - enable TCP/UDP checksum.
// 15 : ethernet_fcs_dis - when set, the controller
// will not append the 802.3 Ethernet Frame Check
// Sequence to the packet
// 16 : reserved16
// 17 : l4_csum_partial - L4 partial checksum. when
// set to 0, the ENA calculates the L4 checksum,
// where the Destination Address required for the
// TCP/UDP pseudo-header is taken from the actual
// packet L3 header. when set to 1, the ENA doesn't
// calculate the sum of the pseudo-header, instead,
// the checksum field of the L4 is used instead. When
// TSO enabled, the checksum of the pseudo-header
// must not include the tcp length field. L4 partial
// checksum should be used for IPv6 packet that
// contains Routing Headers.
// 20:18 : reserved18 - MBZ
// 21 : reserved21 - MBZ
// 31:22 : req_id_lo - Request ID[9:0]
//
    pub meta_ctrl: u32,
    pub buff_addr_lo: u32,
// address high and header size
// 15:0 : addr_hi - Buffer Pointer[47:32]
// 23:16 : reserved16_w2
// 31:24 : header_length - Header length. For Low
// Latency Queues, this fields indicates the number
// of bytes written to the headers' memory. For
// normal queues, if packet is TCP or UDP, and longer
// than max_header_size, then this field should be
// set to the sum of L4 header offset and L4 header
// size(without options), otherwise, this field
// should be set to 0. For both modes, this field
// must not exceed the max_header_size.
// max_header_size value is reported by the Max
// Queues Feature descriptor
//
    pub buff_addr_hi_hdr_sz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_tx_meta_desc {
// 9:0 : req_id_lo - Request ID[9:0]
// 11:10 : reserved10 - MBZ
// 12 : reserved12 - MBZ
// 13 : reserved13 - MBZ
// 14 : ext_valid - if set, offset fields in Word2
// are valid Also MSS High in Word 0 and bits [31:24]
// in Word 3
// 15 : reserved15
// 19:16 : mss_hi
// 20 : eth_meta_type - 0: Tx Metadata Descriptor, 1:
// Extended Metadata Descriptor
// 21 : meta_store - Store extended metadata in queue
// cache
// 22 : reserved22 - MBZ
// 23 : meta_desc - MBO
// 24 : phase
// 25 : reserved25 - MBZ
// 26 : first - Indicates first descriptor in
// transaction
// 27 : last - Indicates last descriptor in
// transaction
// 28 : comp_req - Indicates whether completion
// should be posted, after packet is transmitted.
// Valid only for first descriptor
// 30:29 : reserved29 - MBZ
// 31 : reserved31 - MBZ
//
    pub len_ctrl: u32,
// 5:0 : req_id_hi
// 31:6 : reserved6 - MBZ
//
    pub word1: u32,
// 7:0 : l3_hdr_len
// 15:8 : l3_hdr_off
// 21:16 : l4_hdr_len_in_words - counts the L4 header
// length in words. there is an explicit assumption
// that L4 header appears right after L3 header and
// L4 offset is based on l3_hdr_off+l3_hdr_len
// 31:22 : mss_lo
//
    pub word2: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_tx_cdesc {
// Request ID[15:0]
    pub req_id: u16,
    pub status: u8,
// flags
// 0 : phase
// 7:1 : reserved1
//
    pub flags: u8,
    pub sub_qid: u16,
    pub sq_head_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_rx_desc {
// In bytes. 0 means 64KB
    pub length: u16,
// MBZ
    pub reserved2: u8,
// 0 : phase
// 1 : reserved1 - MBZ
// 2 : first - Indicates first descriptor in
// transaction
// 3 : last - Indicates last descriptor in transaction
// 4 : comp_req
// 5 : reserved5 - MBO
// 7:6 : reserved6 - MBZ
//
    pub ctrl: u8,
    pub req_id: u16,
// MBZ
    pub reserved6: u16,
    pub buff_addr_lo: u32,
    pub buff_addr_hi: u16,
// MBZ
    pub reserved16_w3: u16,
}

// 4-word format Note: all ethernet parsing information are valid only when
// last=1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_rx_cdesc_base {
// 4:0 : l3_proto_idx
// 6:5 : src_vlan_cnt
// 7 : reserved7 - MBZ
// 12:8 : l4_proto_idx
// 13 : l3_csum_err - when set, either the L3
// checksum error detected, or, the controller didn't
// validate the checksum. This bit is valid only when
// l3_proto_idx indicates IPv4 packet
// 14 : l4_csum_err - when set, either the L4
// checksum error detected, or, the controller didn't
// validate the checksum. This bit is valid only when
// l4_proto_idx indicates TCP/UDP packet, and,
// ipv4_frag is not set. This bit is valid only when
// l4_csum_checked below is set.
// 15 : ipv4_frag - Indicates IPv4 fragmented packet
// 16 : l4_csum_checked - L4 checksum was verified
// (could be OK or error), when cleared the status of
// checksum is unknown
// 23:17 : reserved17 - MBZ
// 24 : phase
// 25 : l3_csum2 - second checksum engine result
// 26 : first - Indicates first descriptor in
// transaction
// 27 : last - Indicates last descriptor in
// transaction
// 29:28 : reserved28
// 30 : buffer - 0: Metadata descriptor. 1: Buffer
// Descriptor was used
// 31 : reserved31
//
    pub status: u32,
    pub length: u16,
    pub req_id: u16,
// 32-bit hash result
    pub hash: u32,
    pub sub_qid: u16,
    pub offset: u8,
    pub reserved: u8,
}

// 8-word format
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_rx_cdesc_ext {
    pub base: ena_eth_io_rx_cdesc_base,
    pub buff_addr_lo: u32,
    pub buff_addr_hi: u16,
    pub reserved16: u16,
    pub reserved_w6: u32,
    pub reserved_w7: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_intr_reg {
// 14:0 : rx_intr_delay
// 29:15 : tx_intr_delay
// 30 : intr_unmask
// 31 : reserved
//
    pub intr_control: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ena_eth_io_numa_node_cfg_reg {
// 7:0 : numa
// 30:8 : reserved
// 31 : enabled
//
    pub numa_cfg: u32,
}

// tx_desc

pub const ENA_ETH_IO_TX_DESC_REQ_ID_HI_SHIFT: c_int = 16;

pub const ENA_ETH_IO_TX_DESC_META_DESC_SHIFT: c_int = 23;

pub const ENA_ETH_IO_TX_DESC_PHASE_SHIFT: c_int = 24;

pub const ENA_ETH_IO_TX_DESC_FIRST_SHIFT: c_int = 26;

pub const ENA_ETH_IO_TX_DESC_LAST_SHIFT: c_int = 27;

pub const ENA_ETH_IO_TX_DESC_COMP_REQ_SHIFT: c_int = 28;

pub const ENA_ETH_IO_TX_DESC_DF_SHIFT: c_int = 4;

pub const ENA_ETH_IO_TX_DESC_TSO_EN_SHIFT: c_int = 7;

pub const ENA_ETH_IO_TX_DESC_L4_PROTO_IDX_SHIFT: c_int = 8;

pub const ENA_ETH_IO_TX_DESC_L3_CSUM_EN_SHIFT: c_int = 13;

pub const ENA_ETH_IO_TX_DESC_L4_CSUM_EN_SHIFT: c_int = 14;

pub const ENA_ETH_IO_TX_DESC_ETHERNET_FCS_DIS_SHIFT: c_int = 15;

pub const ENA_ETH_IO_TX_DESC_L4_CSUM_PARTIAL_SHIFT: c_int = 17;

pub const ENA_ETH_IO_TX_DESC_REQ_ID_LO_SHIFT: c_int = 22;

pub const ENA_ETH_IO_TX_DESC_HEADER_LENGTH_SHIFT: c_int = 24;

// tx_meta_desc

pub const ENA_ETH_IO_TX_META_DESC_EXT_VALID_SHIFT: c_int = 14;

pub const ENA_ETH_IO_TX_META_DESC_MSS_HI_SHIFT: c_int = 16;

pub const ENA_ETH_IO_TX_META_DESC_ETH_META_TYPE_SHIFT: c_int = 20;

pub const ENA_ETH_IO_TX_META_DESC_META_STORE_SHIFT: c_int = 21;

pub const ENA_ETH_IO_TX_META_DESC_META_DESC_SHIFT: c_int = 23;

pub const ENA_ETH_IO_TX_META_DESC_PHASE_SHIFT: c_int = 24;

pub const ENA_ETH_IO_TX_META_DESC_FIRST_SHIFT: c_int = 26;

pub const ENA_ETH_IO_TX_META_DESC_LAST_SHIFT: c_int = 27;

pub const ENA_ETH_IO_TX_META_DESC_COMP_REQ_SHIFT: c_int = 28;

pub const ENA_ETH_IO_TX_META_DESC_L3_HDR_OFF_SHIFT: c_int = 8;

pub const ENA_ETH_IO_TX_META_DESC_L4_HDR_LEN_IN_WORDS_SHIFT: c_int = 16;

pub const ENA_ETH_IO_TX_META_DESC_MSS_LO_SHIFT: c_int = 22;

// tx_cdesc

// rx_desc

pub const ENA_ETH_IO_RX_DESC_FIRST_SHIFT: c_int = 2;

pub const ENA_ETH_IO_RX_DESC_LAST_SHIFT: c_int = 3;

pub const ENA_ETH_IO_RX_DESC_COMP_REQ_SHIFT: c_int = 4;

// rx_cdesc_base

pub const ENA_ETH_IO_RX_CDESC_BASE_SRC_VLAN_CNT_SHIFT: c_int = 5;

pub const ENA_ETH_IO_RX_CDESC_BASE_L4_PROTO_IDX_SHIFT: c_int = 8;

pub const ENA_ETH_IO_RX_CDESC_BASE_L3_CSUM_ERR_SHIFT: c_int = 13;

pub const ENA_ETH_IO_RX_CDESC_BASE_L4_CSUM_ERR_SHIFT: c_int = 14;

pub const ENA_ETH_IO_RX_CDESC_BASE_IPV4_FRAG_SHIFT: c_int = 15;

pub const ENA_ETH_IO_RX_CDESC_BASE_L4_CSUM_CHECKED_SHIFT: c_int = 16;

pub const ENA_ETH_IO_RX_CDESC_BASE_PHASE_SHIFT: c_int = 24;

pub const ENA_ETH_IO_RX_CDESC_BASE_L3_CSUM2_SHIFT: c_int = 25;

pub const ENA_ETH_IO_RX_CDESC_BASE_FIRST_SHIFT: c_int = 26;

pub const ENA_ETH_IO_RX_CDESC_BASE_LAST_SHIFT: c_int = 27;

pub const ENA_ETH_IO_RX_CDESC_BASE_BUFFER_SHIFT: c_int = 30;

// intr_reg

pub const ENA_ETH_IO_INTR_REG_TX_INTR_DELAY_SHIFT: c_int = 15;

pub const ENA_ETH_IO_INTR_REG_INTR_UNMASK_SHIFT: c_int = 30;

// numa_node_cfg_reg

pub const ENA_ETH_IO_NUMA_NODE_CFG_REG_ENABLED_SHIFT: c_int = 31;

