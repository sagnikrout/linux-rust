//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/wil6210/txrx_edma.h
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
// Copyright (c) 2012-2016,2018-2019, The Linux Foundation. All rights reserved.
//

// limit status ring size in range [ring size..max ring size]

// RX sring order should be bigger than RX ring order

pub const WIL_DEFAULT_RX_STATUS_RING_ID: c_int = 0;
pub const WIL_RX_DESC_RING_ID: c_int = 0;
pub const WIL_RX_STATUS_IRQ_IDX: c_int = 0;
pub const WIL_TX_STATUS_IRQ_IDX: c_int = 1;

// Error field

pub const WIL_RX_EDMA_DLPF_LU_MISS_CID_TID_MASK: c_uint = 0x7;
pub const WIL_RX_EDMA_DLPF_LU_HIT_CID_TID_MASK: c_uint = 0xf;
pub const WIL_RX_EDMA_DLPF_LU_MISS_CID_POS: c_int = 2;
pub const WIL_RX_EDMA_DLPF_LU_HIT_CID_POS: c_int = 4;
pub const WIL_RX_EDMA_DLPF_LU_MISS_TID_POS: c_int = 5;

pub const WIL_EDMA_DESC_TX_MAC_CFG_0_QID_POS: c_int = 16;
pub const WIL_EDMA_DESC_TX_MAC_CFG_0_QID_LEN: c_int = 6;
pub const WIL_EDMA_DESC_TX_CFG_EOP_POS: c_int = 0;
pub const WIL_EDMA_DESC_TX_CFG_EOP_LEN: c_int = 1;
pub const WIL_EDMA_DESC_TX_CFG_TSO_DESC_TYPE_POS: c_int = 3;
pub const WIL_EDMA_DESC_TX_CFG_TSO_DESC_TYPE_LEN: c_int = 2;
pub const WIL_EDMA_DESC_TX_CFG_SEG_EN_POS: c_int = 5;
pub const WIL_EDMA_DESC_TX_CFG_SEG_EN_LEN: c_int = 1;
pub const WIL_EDMA_DESC_TX_CFG_INSERT_IP_CHKSUM_POS: c_int = 6;
pub const WIL_EDMA_DESC_TX_CFG_INSERT_IP_CHKSUM_LEN: c_int = 1;
pub const WIL_EDMA_DESC_TX_CFG_INSERT_TCP_CHKSUM_POS: c_int = 7;
pub const WIL_EDMA_DESC_TX_CFG_INSERT_TCP_CHKSUM_LEN: c_int = 1;
pub const WIL_EDMA_DESC_TX_CFG_L4_TYPE_POS: c_int = 15;
pub const WIL_EDMA_DESC_TX_CFG_L4_TYPE_LEN: c_int = 1;
pub const WIL_EDMA_DESC_TX_CFG_PSEUDO_HEADER_CALC_EN_POS: c_int = 5;
pub const WIL_EDMA_DESC_TX_CFG_PSEUDO_HEADER_CALC_EN_LEN: c_int = 1;
// Enhanced Rx descriptor - MAC part
// [dword 0] : Reserved
// [dword 1] : Reserved
// [dword 2] : Reserved
// [dword 3]
// bit  0..15 : Buffer ID
// bit 16..31 : Reserved
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_rx_enhanced_mac {
    pub d: [u32; 3],
    pub buff_id: __le16,
    pub reserved: u16,
    pub __packed: },
// Enhanced Rx descriptor - DMA part
// [dword 0] - Reserved
// [dword 1]
// bit  0..31 : addr_low:32 The payload buffer address, bits 0-31
// [dword 2]
// bit  0..15 : addr_high_low:16 The payload buffer address, bits 32-47
// bit 16..31 : Reserved
// [dword 3]
// bit  0..15 : addr_high_high:16 The payload buffer address, bits 48-63
// bit 16..31 : length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_rx_enhanced_dma {
    pub d0: u32,
    pub addr: wil_ring_dma_addr,
    pub w5: u16,
    pub addr_high_high: __le16,
    pub length: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_rx_enhanced_desc {
    pub mac: wil_ring_rx_enhanced_mac,
    pub dma: wil_ring_rx_enhanced_dma,
    pub __packed: },
// Enhanced Tx descriptor - DMA part
// [dword 0]
// Same as legacy
// [dword 1]
// bit  0..31 : addr_low:32 The payload buffer address, bits 0-31
// [dword 2]
// bit  0..15 : addr_high_low:16 The payload buffer address, bits 32-47
// bit 16..23 : ip_length:8 The IP header length for the TX IP checksum
// offload feature
// bit 24..30 : mac_length:7
// bit     31 : ip_version:1 1 - IPv4, 0 - IPv6
// [dword 3]
// bit  0..15 : addr_high_high:16 The payload buffer address, bits 48-63
// bit 16..31 : length
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_tx_enhanced_dma {
    pub l4_hdr_len: u8,
    pub cmd: u8,
    pub w1: u16,
    pub addr: wil_ring_dma_addr,
    pub ip_length: u8,
    pub /: *mut *mut u8 b11; / 0..6: mac_length; 7:ip_version,
    pub addr_high_high: __le16,
    pub length: __le16,
    pub __packed: },
// Enhanced Tx descriptor - MAC part
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
// bit  5..14 : reserved0:10
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
// bit 22..23 : reserved0:2
// bit	   24 : Dest ID extension:1
// bit 25..31 : reserved0:7
// [dword 3]
// bit  0..15 : tso_mss:16
// bit 16..31 : descriptor_scratchpad:16 - mailbox between driver and ucode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_tx_enhanced_mac {
    pub d: [u32; 3],
    pub tso_mss: __le16,
    pub scratchpad: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_tx_enhanced_desc {
    pub mac: wil_ring_tx_enhanced_mac,
    pub dma: wil_ring_tx_enhanced_dma,
    pub __packed: },
pub const TX_STATUS_DESC_READY_POS: c_int = 7;
// Enhanced TX status message
// [dword 0]
// bit  0.. 7 : Number of Descriptor:8 - The number of descriptors that
// are used to form the packets. It  is needed for WB when
// releasing the packet
// bit  8..15 : tx_ring_id:8 The transmission ring ID that is related to
// the message
// bit 16..23 : Status:8 - The TX status Code
// 0x0 - A successful transmission
// 0x1 - Retry expired
// 0x2 - Lifetime Expired
// 0x3 - Released
// 0x4-0xFF - Reserved
// bit 24..30 : Reserved:7
// bit     31 : Descriptor Ready bit:1 - It is initiated to
// zero by the driver when the ring is created. It is set by the HW
// to one for each completed status message. Each wrap around,
// the DR bit value is flipped.
// [dword 1]
// bit 0..31  : timestamp:32 - Set when MPDU is transmitted.
// [dword 2]
// bit  0.. 4 : MCS:5 - The transmitted MCS value
// bit      5 : Reserved:1
// bit  6.. 7 : CB mode:2 - 0-DMG 1-EDMG 2-Wide
// bit  8..12 : QID:5 - The QID that was used for the transmission
// bit 13..15 : Reserved:3
// bit 16..20 : Num of MSDUs:5 - Number of MSDUs in the aggregation
// bit 21..22 : Reserved:2
// bit     23 : Retry:1 - An indication that the transmission was retried
// bit 24..31 : TX-Sector:8 - the antenna sector that was used for
// transmission
// [dword 3]
// bit  0..11 : Sequence number:12 - The Sequence Number that was used
// for the MPDU transmission
// bit 12..31 : Reserved:20
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_ring_tx_status {
    pub num_descriptors: u8,
    pub ring_id: u8,
    pub status: u8,
    pub /: *mut *mut u8 desc_ready; / Only the last bit should be set,
    pub timestamp: u32,
    pub d2: u32,
    pub /: *mut *mut u16 seq_number; / Only the first 12 bits,
    pub w7: u16,
    pub __packed: },
// Enhanced Rx status message - compressed part
// [dword 0]
// bit  0.. 2 : L2 Rx Status:3 - The L2 packet reception Status
// 0-Success, 1-MIC Error, 2-Key Error, 3-Replay Error,
// 4-A-MSDU Error, 5-Reserved, 6-Reserved, 7-FCS Error
// bit  3.. 4 : L3 Rx Status:2 - Bit0 - L3I - L3 identified and checksum
// calculated, Bit1- L3Err - IPv4 Checksum Error
// bit  5.. 6 : L4 Rx Status:2 - Bit0 - L4I - L4 identified and checksum
// calculated, Bit1- L4Err - TCP/UDP Checksum Error
// bit      7 : Reserved:1
// bit  8..19 : Flow ID:12 - MSDU flow ID
// bit     20 : MID_V:1 - The MAC ID field is valid
// bit 21..22 : MID:2 - The MAC ID
// bit     23 : L3T:1 - IP types: 0-IPv6, 1-IPv4
// bit     24 : L4T:1 - Layer 4 Type: 0-UDP, 1-TCP
// bit     25 : BC:1 - The received MPDU is broadcast
// bit     26 : MC:1 - The received MPDU is multicast
// bit     27 : Raw:1 - The MPDU received with no translation
// bit     28 : Sec:1 - The FC control (b14) - Frame Protected
// bit     29 : Error:1 - An error is set when (L2 status != 0) ||
// (L3 status == 3) || (L4 status == 3)
// bit     30 : EOP:1 - End of MSDU signaling. It is set to mark the end
// of the transfer, otherwise the status indicates buffer
// only completion.
// bit     31 : Descriptor Ready bit:1 - It is initiated to
// zero by the driver when the ring is created. It is set
// by the HW to one for each completed status message.
// Each wrap around, the DR bit value is flipped.
// [dword 1]
// bit  0.. 5 : MAC Len:6 - The number of bytes that are used for L2 header
// bit  6..11 : IPLEN:6 - The number of DW that are used for L3 header
// bit 12..15 : I4Len:4 - The number of DW that are used for L4 header
// bit 16..21 : MCS:6 - The received MCS field from the PLCP Header
// bit 22..23 : CB mode:2 - The CB Mode: 0-DMG, 1-EDMG, 2-Wide
// bit 24..27 : Data Offset:4 - The data offset, a code that describe the
// payload shift from the beginning of the buffer:
// 0 - 0 Bytes, 3 - 2 Bytes
// bit     28 : A-MSDU Present:1 - The QoS (b7) A-MSDU present field
// bit     29 : A-MSDU Type:1 The QoS (b8) A-MSDU Type field
// bit     30 : A-MPDU:1 - Packet is part of aggregated MPDU
// bit     31 : Key ID:1 - The extracted Key ID from the encryption header
// [dword 2]
// bit  0..15 : Buffer ID:16 - The Buffer Identifier
// bit 16..31 : Length:16 - It indicates the valid bytes that are stored
// in the current descriptor buffer. For multiple buffer
// descriptor, SW need to sum the total descriptor length
// in all buffers to produce the packet length
// [dword 3]
// bit  0..31  : timestamp:32 - The MPDU Timestamp.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_rx_status_compressed {
    pub d0: u32,
    pub d1: u32,
    pub buff_id: __le16,
    pub length: __le16,
    pub timestamp: u32,
    pub __packed: },
// Enhanced Rx status message - extension part
// [dword 0]
// bit  0.. 4 : QID:5 - The Queue Identifier that the packet is received
// from
// bit  5.. 7 : Reserved:3
// bit  8..11 : TID:4 - The QoS (b3-0) TID Field
// bit 12..15   Source index:4 - The Source index that was found
// bit 16..18 : Destination index:3 - The Destination index that
// bit 19..20 : DS Type:2 - The FC Control (b9-8) - From / To DS
// bit 21..22 : MIC ICR:2 - this signal tells the DMA to assert an
// bit     23 : ESOP:1 - The QoS (b4) ESOP field
// bit     24 : RDG:1
// bit 25..31 : Reserved:7
// [dword 1]
// bit  0.. 1 : Frame Type:2 - The FC Control (b3-2) - MPDU Type
// bit  2.. 5 : Syb type:4 - The FC Control (b7-4) - Frame Subtype
// bit  6..11 : Ext sub type:6 - The FC Control (b11-8) - Frame Extended
// Subtype
// bit 12..13 : ACK Policy:2 - The QoS (b6-5) ACK Policy fields
// bit 14     : DECRYPT_BYP:1 - The MPDU is bypass by the decryption unit
// bit 15..23 : Reserved:9
// bit 24..31 : RSSI/SNR:8 - The RSSI / SNR measurement for the received
// MPDU
// [dword 2]
// bit  0..11 : SN:12 - The received Sequence number field
// bit 12..15 : Reserved:4
// bit 16..31 : PN bits [15:0]:16
// [dword 3]
// bit  0..31 : PN bits [47:16]:32
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_rx_status_extension {
    pub d0: u32,
    pub d1: u32,
    pub /: *mut *mut __le16 seq_num; / only lower 12 bits,
    pub pn_15_0: u16,
    pub pn_47_16: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wil_rx_status_extended {
    pub comp: wil_rx_status_compressed,
    pub ext: wil_rx_status_extension,
    pub __packed: },
    pub )skb->cb: *mut return (void,
    pub )msg)->length: *mut return ((struct wil_rx_status_compressed,
    pub 21): 16,,
    pub 23): 22,,
    pub 19): 8,,
    pub 26): 26,,
//
// In case of DLPF miss the parsing of flow Id should be as follows:
// dest_id:2
// src_id :3 - cid
// tid:3
// Otherwise:
// tid:4
// cid:4
//
    pub wil_rx_status_get_flow_id(msg): u16 val =,
// CID is in bits 2..4
// CID is in bits 4..7
    pub wil_rx_status_get_flow_id(msg): u16 val =,
// TID is in bits 5..7
// TID is in bits 0..3
    pub WIL_RX_EDMA_DLPF_LU_MISS_CID_TID_MASK: return val &,
    pub 30): 30,,
    pub 0: *mut *mut (s->va + (s->elem_size  s->swhead)))->buff_id =,
    pub )msg)->buff_id: *mut return ((struct wil_rx_status_compressed,
    pub 27): 24,,
    pub 0: case 0: return,
    pub 2: case 3: return,
    pub 0xFF: default: return,
    pub IEEE80211_FTYPE_DATA: return,
    pub 2: 0, 1) <<,
    pub 0: return,
    pub 2: 0, 5) <<,
    pub 0: return,
    pub )msg)->ext.seq_num: *mut return ((struct wil_rx_status_extended,
// retry bit is missing in EDMA HW. return 1 to be on the safe side
    pub 1: return,
    pub /: *mut *mut return 0; / use the default MID,
    pub 22): 21,,
    pub 29): 29,,
    pub 2): 0,,
    pub 4): 3,,
    pub 6): 5,,
// L4	L3	Expected result
// 0	0	Ok. No L3 and no L4 known protocols found.
// Treated as L2 packet. (no offloads on this packet)
// 0	1	Ok. It means that L3 was found, and checksum check passed.
// No known L4 protocol was found.
// 0	2	It means that L3 protocol was found, and checksum check failed.
// No L4 known protocol was found.
// 1	any	Ok. It means that L4 was found, and checksum check passed.
// 3	0	Not a possible scenario.
// 3	1	Recalculate. It means that L3 protocol was found, and checksum
// passed. But L4 checksum failed. Need to see if really failed,
// or due to fragmentation.
// 3	2	Both L3 and L4 checksum check failed.
//
    pub wil_rx_status_get_l3_rx_status(msg): int l3_rx_status =,
    pub wil_rx_status_get_l4_rx_status(msg): int l4_rx_status =,
    pub CHECKSUM_UNNECESSARY: return,
    pub CHECKSUM_UNNECESSARY: return,
// L2 packet
    pub CHECKSUM_NONE: return,
// If HW reports bad checksum, let IP stack re-check it
// For example, HW doesn't understand Microsoft IP stack that
// mis-calculates TCP checksum - if it should be 0x0,
// it writes 0xffff in violation of RFC 1624
//
    pub CHECKSUM_NONE: return,
    pub 28): 28,,
    pub 31): 31,,
    pub 4): return WIL_GET_BITS(msg->d2, 0,,
    pub ring->size: return (ring->swhead + 1) %,
    pub cpu_to_le32(lower_32_bits(pa)): addr->addr_low =,
    pub cpu_to_le16((u16)upper_32_bits(pa)): addr->addr_high =,
// addr_high_high = cpu_to_le16((u16)(upper_32_bits(pa) >> 16));
    pub 48): ((u64)le16_to_cpu(dma->addr_high_high) <<,
    pub 48): ((u64)le16_to_cpu(dma->addr_high_high) <<,
    pub wil): *mut void wil_configure_interrupt_moderation_edma(struct wil6210_priv,
    pub sring): *mut wil_status_ring,
    pub quota): *mut *mut void wil_rx_handle_edma(struct wil6210_priv wil, int,
    pub wil): *mut void wil_init_txrx_ops_edma(struct wil6210_priv,
