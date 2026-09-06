//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/txrx.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2012-2016 Qualcomm Atheros, Inc.
// Copyright (c) 2018-2019, The Linux Foundation. All rights reserved.
//

// default size of MAC Tx/Rx buffers

// how many bytes to reserve for rtap header?

// Tx/Rx path
// Tx descriptor - MAC part
// [dword 0]
// bit  0.. 9 : lifetime_expiry_value:10
// bit     10 : interrupt_en:1
// bit     11 : status_en:1
// bit 12..13 : txss_override:2
// bit     14 : timestamp_insertion:1
// bit     15 : duration_preserve:1
// bit 16..21 : reserved0:6
// bit 22..26 : mcs_index:5
// bit     27 : mcs_en:1
// bit 28..30 : reserved1:3
// bit     31 : sn_preserved:1
// [dword 1]
// bit  0.. 3 : pkt_mode:4
// bit      4 : pkt_mode_en:1
// bit      5 : mac_id_en:1
// bit   6..7 : mac_id:2
// bit  8..14 : reserved0:7
// bit     15 : ack_policy_en:1
// bit 16..19 : dst_index:4
// bit     20 : dst_index_en:1
// bit 21..22 : ack_policy:2
// bit     23 : lifetime_en:1
// bit 24..30 : max_retry:7
// bit     31 : max_retry_en:1
// [dword 2]
// bit  0.. 7 : num_of_descriptors:8
// bit  8..17 : reserved:10
// bit 18..19 : l2_translation_type:2 00 - bypass, 01 - 802.3, 10 - 802.11
// bit     20 : snap_hdr_insertion_en:1
// bit     21 : vlan_removal_en:1
// bit 22..31 : reserved0:10
// [dword 3]
// bit  0.. 31: ucode_cmd:32
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_tx_mac {
    pub d: [u32; 3],
    pub ucode_cmd: u32,
    pub __packed: },
// TX MAC Dword 0
pub const MAC_CFG_DESC_TX_0_LIFETIME_EXPIRY_VALUE_POS: c_int = 0;
pub const MAC_CFG_DESC_TX_0_LIFETIME_EXPIRY_VALUE_LEN: c_int = 10;
pub const MAC_CFG_DESC_TX_0_LIFETIME_EXPIRY_VALUE_MSK: c_uint = 0x3FF;
pub const MAC_CFG_DESC_TX_0_INTERRUP_EN_POS: c_int = 10;
pub const MAC_CFG_DESC_TX_0_INTERRUP_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_0_INTERRUP_EN_MSK: c_uint = 0x400;
pub const MAC_CFG_DESC_TX_0_STATUS_EN_POS: c_int = 11;
pub const MAC_CFG_DESC_TX_0_STATUS_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_0_STATUS_EN_MSK: c_uint = 0x800;
pub const MAC_CFG_DESC_TX_0_TXSS_OVERRIDE_POS: c_int = 12;
pub const MAC_CFG_DESC_TX_0_TXSS_OVERRIDE_LEN: c_int = 2;
pub const MAC_CFG_DESC_TX_0_TXSS_OVERRIDE_MSK: c_uint = 0x3000;
pub const MAC_CFG_DESC_TX_0_TIMESTAMP_INSERTION_POS: c_int = 14;
pub const MAC_CFG_DESC_TX_0_TIMESTAMP_INSERTION_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_0_TIMESTAMP_INSERTION_MSK: c_uint = 0x4000;
pub const MAC_CFG_DESC_TX_0_DURATION_PRESERVE_POS: c_int = 15;
pub const MAC_CFG_DESC_TX_0_DURATION_PRESERVE_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_0_DURATION_PRESERVE_MSK: c_uint = 0x8000;
pub const MAC_CFG_DESC_TX_0_MCS_INDEX_POS: c_int = 22;
pub const MAC_CFG_DESC_TX_0_MCS_INDEX_LEN: c_int = 5;
pub const MAC_CFG_DESC_TX_0_MCS_INDEX_MSK: c_uint = 0x7C00000;
pub const MAC_CFG_DESC_TX_0_MCS_EN_POS: c_int = 27;
pub const MAC_CFG_DESC_TX_0_MCS_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_0_MCS_EN_MSK: c_uint = 0x8000000;
pub const MAC_CFG_DESC_TX_0_SN_PRESERVED_POS: c_int = 31;
pub const MAC_CFG_DESC_TX_0_SN_PRESERVED_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_0_SN_PRESERVED_MSK: c_uint = 0x80000000;
// TX MAC Dword 1
pub const MAC_CFG_DESC_TX_1_PKT_MODE_POS: c_int = 0;
pub const MAC_CFG_DESC_TX_1_PKT_MODE_LEN: c_int = 4;
pub const MAC_CFG_DESC_TX_1_PKT_MODE_MSK: c_uint = 0xF;
pub const MAC_CFG_DESC_TX_1_PKT_MODE_EN_POS: c_int = 4;
pub const MAC_CFG_DESC_TX_1_PKT_MODE_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_1_PKT_MODE_EN_MSK: c_uint = 0x10;
pub const MAC_CFG_DESC_TX_1_MAC_ID_EN_POS: c_int = 5;
pub const MAC_CFG_DESC_TX_1_MAC_ID_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_1_MAC_ID_EN_MSK: c_uint = 0x20;
pub const MAC_CFG_DESC_TX_1_MAC_ID_POS: c_int = 6;
pub const MAC_CFG_DESC_TX_1_MAC_ID_LEN: c_int = 2;
pub const MAC_CFG_DESC_TX_1_MAC_ID_MSK: c_uint = 0xc0;
pub const MAC_CFG_DESC_TX_1_ACK_POLICY_EN_POS: c_int = 15;
pub const MAC_CFG_DESC_TX_1_ACK_POLICY_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_1_ACK_POLICY_EN_MSK: c_uint = 0x8000;
pub const MAC_CFG_DESC_TX_1_DST_INDEX_POS: c_int = 16;
pub const MAC_CFG_DESC_TX_1_DST_INDEX_LEN: c_int = 4;
pub const MAC_CFG_DESC_TX_1_DST_INDEX_MSK: c_uint = 0xF0000;
pub const MAC_CFG_DESC_TX_1_DST_INDEX_EN_POS: c_int = 20;
pub const MAC_CFG_DESC_TX_1_DST_INDEX_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_1_DST_INDEX_EN_MSK: c_uint = 0x100000;
pub const MAC_CFG_DESC_TX_1_ACK_POLICY_POS: c_int = 21;
pub const MAC_CFG_DESC_TX_1_ACK_POLICY_LEN: c_int = 2;
pub const MAC_CFG_DESC_TX_1_ACK_POLICY_MSK: c_uint = 0x600000;
pub const MAC_CFG_DESC_TX_1_LIFETIME_EN_POS: c_int = 23;
pub const MAC_CFG_DESC_TX_1_LIFETIME_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_1_LIFETIME_EN_MSK: c_uint = 0x800000;
pub const MAC_CFG_DESC_TX_1_MAX_RETRY_POS: c_int = 24;
pub const MAC_CFG_DESC_TX_1_MAX_RETRY_LEN: c_int = 7;
pub const MAC_CFG_DESC_TX_1_MAX_RETRY_MSK: c_uint = 0x7F000000;
pub const MAC_CFG_DESC_TX_1_MAX_RETRY_EN_POS: c_int = 31;
pub const MAC_CFG_DESC_TX_1_MAX_RETRY_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_1_MAX_RETRY_EN_MSK: c_uint = 0x80000000;
// TX MAC Dword 2
pub const MAC_CFG_DESC_TX_2_NUM_OF_DESCRIPTORS_POS: c_int = 0;
pub const MAC_CFG_DESC_TX_2_NUM_OF_DESCRIPTORS_LEN: c_int = 8;
pub const MAC_CFG_DESC_TX_2_NUM_OF_DESCRIPTORS_MSK: c_uint = 0xFF;
pub const MAC_CFG_DESC_TX_2_RESERVED_POS: c_int = 8;
pub const MAC_CFG_DESC_TX_2_RESERVED_LEN: c_int = 10;
pub const MAC_CFG_DESC_TX_2_RESERVED_MSK: c_uint = 0x3FF00;
pub const MAC_CFG_DESC_TX_2_L2_TRANSLATION_TYPE_POS: c_int = 18;
pub const MAC_CFG_DESC_TX_2_L2_TRANSLATION_TYPE_LEN: c_int = 2;
pub const MAC_CFG_DESC_TX_2_L2_TRANSLATION_TYPE_MSK: c_uint = 0xC0000;
pub const MAC_CFG_DESC_TX_2_SNAP_HDR_INSERTION_EN_POS: c_int = 20;
pub const MAC_CFG_DESC_TX_2_SNAP_HDR_INSERTION_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_2_SNAP_HDR_INSERTION_EN_MSK: c_uint = 0x100000;
pub const MAC_CFG_DESC_TX_2_VLAN_REMOVAL_EN_POS: c_int = 21;
pub const MAC_CFG_DESC_TX_2_VLAN_REMOVAL_EN_LEN: c_int = 1;
pub const MAC_CFG_DESC_TX_2_VLAN_REMOVAL_EN_MSK: c_uint = 0x200000;
// TX MAC Dword 3
pub const MAC_CFG_DESC_TX_3_UCODE_CMD_POS: c_int = 0;
pub const MAC_CFG_DESC_TX_3_UCODE_CMD_LEN: c_int = 32;
pub const MAC_CFG_DESC_TX_3_UCODE_CMD_MSK: c_uint = 0xFFFFFFFF;
// TX DMA Dword 0
pub const DMA_CFG_DESC_TX_0_L4_LENGTH_POS: c_int = 0;
pub const DMA_CFG_DESC_TX_0_L4_LENGTH_LEN: c_int = 8;
pub const DMA_CFG_DESC_TX_0_L4_LENGTH_MSK: c_uint = 0xFF;
pub const DMA_CFG_DESC_TX_0_CMD_EOP_POS: c_int = 8;
pub const DMA_CFG_DESC_TX_0_CMD_EOP_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_0_CMD_EOP_MSK: c_uint = 0x100;
pub const DMA_CFG_DESC_TX_0_CMD_MARK_WB_POS: c_int = 9;
pub const DMA_CFG_DESC_TX_0_CMD_MARK_WB_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_0_CMD_MARK_WB_MSK: c_uint = 0x200;
pub const DMA_CFG_DESC_TX_0_CMD_DMA_IT_POS: c_int = 10;
pub const DMA_CFG_DESC_TX_0_CMD_DMA_IT_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_0_CMD_DMA_IT_MSK: c_uint = 0x400;
pub const DMA_CFG_DESC_TX_0_SEGMENT_BUF_DETAILS_POS: c_int = 11;
pub const DMA_CFG_DESC_TX_0_SEGMENT_BUF_DETAILS_LEN: c_int = 2;
pub const DMA_CFG_DESC_TX_0_SEGMENT_BUF_DETAILS_MSK: c_uint = 0x1800;
pub const DMA_CFG_DESC_TX_0_TCP_SEG_EN_POS: c_int = 13;
pub const DMA_CFG_DESC_TX_0_TCP_SEG_EN_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_0_TCP_SEG_EN_MSK: c_uint = 0x2000;
pub const DMA_CFG_DESC_TX_0_IPV4_CHECKSUM_EN_POS: c_int = 14;
pub const DMA_CFG_DESC_TX_0_IPV4_CHECKSUM_EN_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_0_IPV4_CHECKSUM_EN_MSK: c_uint = 0x4000;
pub const DMA_CFG_DESC_TX_0_TCP_UDP_CHECKSUM_EN_POS: c_int = 15;
pub const DMA_CFG_DESC_TX_0_TCP_UDP_CHECKSUM_EN_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_0_TCP_UDP_CHECKSUM_EN_MSK: c_uint = 0x8000;
pub const DMA_CFG_DESC_TX_0_QID_POS: c_int = 16;
pub const DMA_CFG_DESC_TX_0_QID_LEN: c_int = 5;
pub const DMA_CFG_DESC_TX_0_QID_MSK: c_uint = 0x1F0000;
pub const DMA_CFG_DESC_TX_0_PSEUDO_HEADER_CALC_EN_POS: c_int = 21;
pub const DMA_CFG_DESC_TX_0_PSEUDO_HEADER_CALC_EN_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_0_PSEUDO_HEADER_CALC_EN_MSK: c_uint = 0x200000;
pub const DMA_CFG_DESC_TX_0_L4_TYPE_POS: c_int = 30;
pub const DMA_CFG_DESC_TX_0_L4_TYPE_LEN: c_int = 2;
pub const DMA_CFG_DESC_TX_0_L4_TYPE_MSK: c_uint = 0xC0000000 /* L4 type: 0-UDP, 2-TCP */;
pub const DMA_CFG_DESC_TX_OFFLOAD_CFG_MAC_LEN_POS: c_int = 0;
pub const DMA_CFG_DESC_TX_OFFLOAD_CFG_MAC_LEN_LEN: c_int = 7;
pub const DMA_CFG_DESC_TX_OFFLOAD_CFG_MAC_LEN_MSK: c_uint = 0x7F /* MAC hdr len */;
pub const DMA_CFG_DESC_TX_OFFLOAD_CFG_L3T_IPV4_POS: c_int = 7;
pub const DMA_CFG_DESC_TX_OFFLOAD_CFG_L3T_IPV4_LEN: c_int = 1;
pub const DMA_CFG_DESC_TX_OFFLOAD_CFG_L3T_IPV4_MSK: c_uint = 0x80 /* 1-IPv4, 0-IPv6 */;

// Tx descriptor - DMA part
// [dword 0]
// bit  0.. 7 : l4_length:8 layer 4 length
// bit      8 : cmd_eop:1 This descriptor is the last one in the packet
// bit      9 : reserved
// bit     10 : cmd_dma_it:1 immediate interrupt
// bit 11..12 : SBD - Segment Buffer Details
// 00 - Header Segment
// 01 - First Data Segment
// 10 - Medium Data Segment
// 11 - Last Data Segment
// bit     13 : TSE - TCP Segmentation Enable
// bit     14 : IIC - Directs the HW to Insert IPv4 Checksum
// bit     15 : ITC - Directs the HW to Insert TCP/UDP Checksum
// bit 16..20 : QID - The target QID that the packet should be stored
// in the MAC.
// bit     21 : PO - Pseudo header Offload:
// 0 - Use the pseudo header value from the TCP checksum field
// 1- Calculate Pseudo header Checksum
// bit     22 : NC - No UDP Checksum
// bit 23..29 : reserved
// bit 30..31 : L4T - Layer 4 Type: 00 - UDP , 10 - TCP , 10, 11 - Reserved
// If L4Len equal 0, no L4 at all
// [dword 1]
// bit  0..31 : addr_low:32 The payload buffer low address
// [dword 2]
// bit  0..15 : addr_high:16 The payload buffer high address
// bit 16..23 : ip_length:8 The IP header length for the TX IP checksum
// offload feature
// bit 24..30 : mac_length:7
// bit     31 : ip_version:1 1 - IPv4, 0 - IPv6
// [dword 3]
// [byte 12] error
// bit  0   2 : mac_status:3
// bit  3   7 : reserved:5
// [byte 13] status
// bit      0 : DU:1 Descriptor Used
// bit  1   7 : reserved:7
// [word 7] length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_tx_dma {
    pub d0: u32,
    pub addr: wil_ring_dma_addr,
    pub ip_length: u8,
    pub /: *mut *mut u8 b11; / 0..6: mac_length; 7:ip_version,
    pub /: *mut *mut u8 error; / 0..2: err; 3..7: reserved;,
    pub /: *mut *mut u8 status; / 0: used; 1..7; reserved,
    pub length: __le16,
    pub __packed: },
// TSO type used in dma descriptor d0 bits 11-12
}

// Rx descriptor - MAC part
// [dword 0]
// bit  0.. 3 : tid:4 The QoS (b3-0) TID Field
// bit  4.. 6 : cid:3 The Source index that  was found during parsing the TA.
// This field is used to define the source of the packet
// bit      7 : MAC_id_valid:1, 1 if MAC virtual number is valid.
// bit  8.. 9 : mid:2 The MAC virtual number
// bit 10..11 : frame_type:2 : The FC (b3-2) - MPDU Type
// (management, data, control and extension)
// bit 12..15 : frame_subtype:4 : The FC (b7-4) - Frame Subtype
// bit 16..27 : seq_number:12 The received Sequence number field
// bit 28..31 : extended:4 extended subtype
// [dword 1]
// bit  0.. 3 : reserved
// bit  4.. 5 : key_id:2
// bit      6 : decrypt_bypass:1
// bit      7 : security:1 FC (b14)
// bit  8.. 9 : ds_bits:2 FC (b9-8)
// bit     10 : a_msdu_present:1  QoS (b7)
// bit     11 : a_msdu_type:1  QoS (b8)
// bit     12 : a_mpdu:1  part of AMPDU aggregation
// bit     13 : broadcast:1
// bit     14 : mutlicast:1
// bit     15 : reserved:1
// bit 16..20 : rx_mac_qid:5 The Queue Identifier that the packet
// is received from
// bit 21..24 : mcs:4
// bit 25..28 : mic_icr:4 this signal tells the DMA to assert an interrupt
// after it writes the packet
// bit 29..31 : reserved:3
// [dword 2]
// bit  0.. 2 : time_slot:3 The timeslot that the MPDU is received
// bit  3.. 4 : fc_protocol_ver:1 The FC (b1-0) - Protocol Version
// bit      5 : fc_order:1 The FC Control (b15) -Order
// bit  6.. 7 : qos_ack_policy:2 The QoS (b6-5) ack policy Field
// bit      8 : esop:1 The QoS (b4) ESOP field
// bit      9 : qos_rdg_more_ppdu:1 The QoS (b9) RDG field
// bit 10..14 : qos_reserved:5 The QoS (b14-10) Reserved field
// bit     15 : qos_ac_constraint:1 QoS (b15)
// bit 16..31 : pn_15_0:16 low 2 bytes of PN
// [dword 3]
// bit  0..31 : pn_47_16:32 high 4 bytes of PN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_rx_mac {
    pub d0: u32,
    pub d1: u32,
    pub w4: u16,
    pub pn_15_0: u16,
    pub pn_47_16: u32,
    pub __packed: },
// Rx descriptor - DMA part
// [dword 0]
// bit  0.. 7 : l4_length:8 layer 4 length. The field is only valid if
// L4I bit is set
// bit      8 : cmd_eop:1 set to 1
// bit      9 : cmd_rt:1 set to 1
// bit     10 : cmd_dma_it:1 immediate interrupt
// bit 11..15 : reserved:5
// bit 16..29 : phy_info_length:14 It is valid when the PII is set.
// When the FFM bit is set bits 29-27 are used for
// Flex Filter Match. Matching Index to one of the L2
// EtherType Flex Filter
// bit 30..31 : l4_type:2 valid if the L4I bit is set in the status field
// 00 - UDP, 01 - TCP, 10, 11 - reserved
// [dword 1]
// bit  0..31 : addr_low:32 The payload buffer low address
// [dword 2]
// bit  0..15 : addr_high:16 The payload buffer high address
// bit 16..23 : ip_length:8 The filed is valid only if the L3I bit is set
// bit 24..30 : mac_length:7
// bit     31 : ip_version:1 1 - IPv4, 0 - IPv6
// [dword 3]
// [byte 12] error
// bit      0 : FCS:1
// bit      1 : MIC:1
// bit      2 : Key miss:1
// bit      3 : Replay:1
// bit      4 : L3:1 IPv4 checksum
// bit      5 : L4:1 TCP/UDP checksum
// bit  6   7 : reserved:2
// [byte 13] status
// bit      0 : DU:1 Descriptor Used
// bit      1 : EOP:1 The descriptor indicates the End of Packet
// bit      2 : error:1
// bit      3 : MI:1 MAC Interrupt is asserted (according to parser decision)
// bit      4 : L3I:1 L3 identified and checksum calculated
// bit      5 : L4I:1 L4 identified and checksum calculated
// bit      6 : PII:1 PHY Info Included in the packet
// bit      7 : FFM:1 EtherType Flex Filter Match
// [word 7] length
//

// Error field

// Status field

// IEEE 802.11, 8.5.2 EAPOL-Key frames

pub const WIL_EAP_NONCE_LEN: c_int = 32;
pub const WIL_EAP_KEY_RSC_LEN: c_int = 8;
pub const WIL_EAP_REPLAY_COUNTER_LEN: c_int = 8;
pub const WIL_EAP_KEY_IV_LEN: c_int = 16;
pub const WIL_EAP_KEY_ID_LEN: c_int = 8;
}

pub const WIL_EAPOL_KEY_TYPE_RSN: c_int = 2;
pub const WIL_EAPOL_KEY_TYPE_WPA: c_int = 254;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_1x_hdr {
    pub version: u8,
    pub type: u8,
    pub length: __be16,
// followed by data
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_eapol_key {
    pub type: u8,
    pub key_info: __be16,
    pub key_length: __be16,
    pub replay_counter: [u8; WIL_EAP_REPLAY_COUNTER_LEN],
    pub key_nonce: [u8; WIL_EAP_NONCE_LEN],
    pub key_iv: [u8; WIL_EAP_KEY_IV_LEN],
    pub key_rsc: [u8; WIL_EAP_KEY_RSC_LEN],
    pub key_id: [u8; WIL_EAP_KEY_ID_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_rx_dma {
    pub d0: u32,
    pub addr: wil_ring_dma_addr,
    pub ip_length: u8,
    pub b11: u8,
    pub error: u8,
    pub status: u8,
    pub length: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_tx_desc {
    pub mac: vring_tx_mac,
    pub dma: vring_tx_dma,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union wil_tx_desc {
    pub legacy: vring_tx_desc,
    pub enhanced: wil_tx_enhanced_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vring_rx_desc {
    pub mac: vring_rx_mac,
    pub dma: vring_rx_dma,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union wil_rx_desc {
    pub legacy: vring_rx_desc,
    pub enhanced: wil_rx_enhanced_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union wil_ring_desc {
    pub tx: wil_tx_desc,
    pub rx: wil_rx_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_rx_info {
    pub cid: u8,
}

// this struct will be stored in the skb cb buffer
// max length of the struct is limited to 48 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct skb_rx_info {
    pub rx_desc: vring_rx_desc,
    pub rx_info: packet_rx_info,
}

extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d0, _arg: 0, _arg: 3) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d0, _arg: 4, _arg: 6) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d0, _arg: 10, _arg: 11) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d0, _arg: 12, _arg: 15) -> return;
}
// 1-st byte (with frame type/subtype) of FC field
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d0, _arg: 16, _arg: 27) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d0, _arg: 28, _arg: 31) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d0, _arg: 31, _arg: 31) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d1, _arg: 4, _arg: 5) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d1, _arg: 7, _arg: 7) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d1, _arg: 8, _arg: 9) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d1, _arg: 21, _arg: 24) -> return;
}
extern "C" {
    pub fn WIL_GET_BITS(_arg: d->mac.d1, _arg: 13, _arg: 14) -> return;
}
extern "C" {
    pub fn is_unicast_ether_addr(sk_requests_wifi_status(skb->sk: da) &&) -> return;
}
// Used space in Tx ring
// Available space in Tx ring
// In Enhanced DMA ring 0 is reserved for RX
// similar to ieee80211_ version, but FC contain only 1-st byte
// wil_val_in_range - check if value in [min,max)
extern "C" {
    pub fn wil_netif_rx_any(skb: *mut sk_buff, ndev: *mut net_device);
}
extern "C" {
    pub fn wil_rx_reorder(wil: *mut wil6210_priv, skb: *mut sk_buff);
}
extern "C" {
    pub fn wil_tx_data_init(txdata: *mut wil_ring_tx_data);
}
extern "C" {
    pub fn wil_init_txrx_ops_legacy_dma(wil: *mut wil6210_priv);
}
