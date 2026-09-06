//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/dp_htt.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
// HTT definitions
pub const HTT_TAG_TCL_METADATA_VERSION: c_int = 5;

// vdev meta data

// peer meta data

// Global sequence number
pub const HTT_TCL_META_DATA_TYPE_GLOBAL_SEQ_NUM: c_int = 3;

pub const HTT_TX_MLO_MCAST_HOST_REINJECT_BASE_VDEV_ID: c_int = 128;
// HTT tx completion is overlaid in wbm_release_ring

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_wbm_completion {
    pub rsvd0: [__le32; 2],
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub info3: __le32,
    pub info4: __le32,
    pub rsvd1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_h2t_msg_type {
    HTT_H2T_MSG_TYPE_VERSION_REQ		= 0,
    HTT_H2T_MSG_TYPE_SRING_SETUP		= 0xb,
    HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG	= 0xc,
    HTT_H2T_MSG_TYPE_EXT_STATS_CFG		= 0x10,
    HTT_H2T_MSG_TYPE_PPDU_STATS_CFG		= 0x11,
    HTT_H2T_MSG_TYPE_VDEV_TXRX_STATS_CFG	= 0x1a,
    HTT_H2T_MSG_TYPE_TX_MONITOR_CFG		= 0x1b,
}

pub const HTT_OPTION_TCL_METADATA_VER_V1: c_int = 1;
pub const HTT_OPTION_TCL_METADATA_VER_V2: c_int = 2;

pub const HTT_TCL_METADATA_VER_SZ: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ver_req_cmd {
    pub ver_reg_info: __le32,
    pub tcl_metadata_version: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_srng_ring_type {
    HTT_HW_TO_SW_RING,
    HTT_SW_TO_HW_RING,
    HTT_SW_TO_SW_RING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_srng_ring_id {
    HTT_RXDMA_HOST_BUF_RING,
    HTT_RXDMA_MONITOR_STATUS_RING,
    HTT_RXDMA_MONITOR_BUF_RING,
    HTT_RXDMA_MONITOR_DESC_RING,
    HTT_RXDMA_MONITOR_DEST_RING,
    HTT_HOST1_TO_FW_RXBUF_RING,
    HTT_HOST2_TO_FW_RXBUF_RING,
    HTT_RXDMA_NON_MONITOR_DEST_RING,
    HTT_RXDMA_HOST_BUF_RING2,
    HTT_TX_MON_HOST2MON_BUF_RING,
    HTT_TX_MON_MON2HOST_DEST_RING,
    HTT_RX_MON_HOST2MON_BUF_RING,
    HTT_RX_MON_MON2HOST_DEST_RING,
}

// host -> target  HTT_SRING_SETUP message
//
// After target is booted up, Host can send SRING setup message for
// each host facing LMAC SRING. Target setups up HW registers based
// on setup message and confirms back to Host if response_required is set.
// Host should wait for confirmation message before sending new SRING
// setup message
//
// The message would appear as follows:
//
// |31            24|23    20|19|18 16|15|14          8|7                0|
// |--------------- +-----------------+----------------+------------------|
// |    ring_type   |      ring_id    |    pdev_id     |     msg_type     |
// |----------------------------------------------------------------------|
// |                          ring_base_addr_lo                           |
// |----------------------------------------------------------------------|
// |                         ring_base_addr_hi                            |
// |----------------------------------------------------------------------|
// |ring_misc_cfg_flag|ring_entry_size|            ring_size              |
// |----------------------------------------------------------------------|
// |                         ring_head_offset32_remote_addr_lo            |
// |----------------------------------------------------------------------|
// |                         ring_head_offset32_remote_addr_hi            |
// |----------------------------------------------------------------------|
// |                         ring_tail_offset32_remote_addr_lo            |
// |----------------------------------------------------------------------|
// |                         ring_tail_offset32_remote_addr_hi            |
// |----------------------------------------------------------------------|
// |                          ring_msi_addr_lo                            |
// |----------------------------------------------------------------------|
// |                          ring_msi_addr_hi                            |
// |----------------------------------------------------------------------|
// |                          ring_msi_data                               |
// |----------------------------------------------------------------------|
// |         intr_timer_th            |IM|      intr_batch_counter_th     |
// |----------------------------------------------------------------------|
// |          reserved        |RR|PTCF|        intr_low_threshold         |
// |----------------------------------------------------------------------|
// Where
// IM = sw_intr_mode
// RR = response_required
// PTCF = prefetch_timer_cfg
//
// The message is interpreted as follows:
// dword0  - b'0:7   - msg_type: This will be set to
// HTT_H2T_MSG_TYPE_SRING_SETUP
// b'8:15  - pdev_id:
// 0 (for rings at SOC/UMAC level),
// 1/2/3 mac id (for rings at LMAC level)
// b'16:23 - ring_id: identify which ring is to setup,
// more details can be got from enum htt_srng_ring_id
// b'24:31 - ring_type: identify type of host rings,
// more details can be got from enum htt_srng_ring_type
// dword1  - b'0:31  - ring_base_addr_lo: Lower 32bits of ring base address
// dword2  - b'0:31  - ring_base_addr_hi: Upper 32bits of ring base address
// dword3  - b'0:15  - ring_size: size of the ring in unit of 4-bytes words
// b'16:23 - ring_entry_size: Size of each entry in 4-byte word units
// b'24:31 - ring_misc_cfg_flag: Valid only for HW_TO_SW_RING and
// SW_TO_HW_RING.
// Refer to HTT_SRING_SETUP_RING_MISC_CFG_RING defs.
// dword4  - b'0:31  - ring_head_off32_remote_addr_lo:
// Lower 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the head
// element within the ring.
// (The head offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword5  - b'0:31  - ring_head_off32_remote_addr_hi:
// Upper 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the head
// element within the ring.
// (The head offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword6  - b'0:31  - ring_tail_off32_remote_addr_lo:
// Lower 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the tail
// element within the ring.
// (The tail offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword7  - b'0:31  - ring_tail_off32_remote_addr_hi:
// Upper 32 bits of memory address of the remote variable
// storing the 4-byte word offset that identifies the tail
// element within the ring.
// (The tail offset variable has type u32.)
// Valid for HW_TO_SW and SW_TO_SW rings.
// dword8  - b'0:31  - ring_msi_addr_lo: Lower 32bits of MSI cfg address
// valid only for HW_TO_SW_RING and SW_TO_HW_RING
// dword9  - b'0:31  - ring_msi_addr_hi: Upper 32bits of MSI cfg address
// valid only for HW_TO_SW_RING and SW_TO_HW_RING
// dword10 - b'0:31  - ring_msi_data: MSI data
// Refer to HTT_SRING_SETUP_RING_MSC_CFG_xxx defs
// valid only for HW_TO_SW_RING and SW_TO_HW_RING
// dword11 - b'0:14  - intr_batch_counter_th:
// batch counter threshold is in units of 4-byte words.
// HW internally maintains and increments batch count.
// (see SRING spec for detail description).
// When batch count reaches threshold value, an interrupt
// is generated by HW.
// b'15    - sw_intr_mode:
// This configuration shall be static.
// Only programmed at power up.
// 0: generate pulse style sw interrupts
// 1: generate level style sw interrupts
// b'16:31 - intr_timer_th:
// The timer init value when timer is idle or is
// initialized to start downcounting.
// In 8us units (to cover a range of 0 to 524 ms)
// dword12 - b'0:15  - intr_low_threshold:
// Used only by Consumer ring to generate ring_sw_int_p.
// Ring entries low threshold water mark, that is used
// in combination with the interrupt timer as well as
// the clearing of the level interrupt.
// b'16:18 - prefetch_timer_cfg:
// Used only by Consumer ring to set timer mode to
// support Application prefetch handling.
// The external tail offset/pointer will be updated
// at following intervals:
// 3'b000: (Prefetch feature disabled; used only for debug)
// 3'b001: 1 usec
// 3'b010: 4 usec
// 3'b011: 8 usec (default)
// 3'b100: 16 usec
// Others: Reserved
// b'19    - response_required:
// Host needs HTT_T2H_MSG_TYPE_SRING_SETUP_DONE as response
// b'20:31 - reserved:  reserved for future use
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_srng_setup_cmd {
    pub info0: __le32,
    pub ring_base_addr_lo: __le32,
    pub ring_base_addr_hi: __le32,
    pub info1: __le32,
    pub ring_head_off32_remote_addr_lo: __le32,
    pub ring_head_off32_remote_addr_hi: __le32,
    pub ring_tail_off32_remote_addr_lo: __le32,
    pub ring_tail_off32_remote_addr_hi: __le32,
    pub ring_msi_addr_lo: __le32,
    pub ring_msi_addr_hi: __le32,
    pub msi_data: __le32,
    pub intr_info: __le32,
    pub info2: __le32,
    pub __packed: },
// host -> target FW  PPDU_STATS config message
//
// @details
// The following field definitions describe the format of the HTT host
// to target FW for PPDU_STATS_CFG msg.
// The message allows the host to configure the PPDU_STATS_IND messages
// produced by the target.
//
// |31          24|23          16|15           8|7            0|
// |-----------------------------------------------------------|
// |    REQ bit mask             |   pdev_mask  |   msg type   |
// |-----------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: identifies this is a req to configure ppdu_stats_ind from target
// Value: 0x11
// - PDEV_MASK
// Bits 8:15
// Purpose: identifies which pdevs this PPDU stats configuration applies to
// Value: This is a overloaded field, refer to usage and interpretation of
// PDEV in interface document.
// Bit   8    :  Reserved for SOC stats
// Bit 9 - 15 :  Indicates PDEV_MASK in DBDC
// Indicates MACID_MASK in DBS
// - REQ_TLV_BIT_MASK
// Bits 16:31
// Purpose: each set bit indicates the corresponding PPDU stats TLV type
// needs to be included in the target's PPDU_STATS_IND messages.
// Value: refer htt_ppdu_stats_tlv_tag_t <<<???
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_cfg_cmd {
    pub msg: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_ppdu_stats_tag_type {
    HTT_PPDU_STATS_TAG_COMMON,
    HTT_PPDU_STATS_TAG_USR_COMMON,
    HTT_PPDU_STATS_TAG_USR_RATE,
    HTT_PPDU_STATS_TAG_USR_MPDU_ENQ_BITMAP_64,
    HTT_PPDU_STATS_TAG_USR_MPDU_ENQ_BITMAP_256,
    HTT_PPDU_STATS_TAG_SCH_CMD_STATUS,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_COMMON,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_BA_BITMAP_64,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_BA_BITMAP_256,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_ACK_BA_STATUS,
    HTT_PPDU_STATS_TAG_USR_COMPLTN_FLUSH,
    HTT_PPDU_STATS_TAG_USR_COMMON_ARRAY,
    HTT_PPDU_STATS_TAG_INFO,
    HTT_PPDU_STATS_TAG_TX_MGMTCTRL_PAYLOAD,

// New TLV's are added above to this line
    HTT_PPDU_STATS_TAG_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_stats_internal_ppdu_frametype {
    HTT_STATS_PPDU_FTYPE_CTRL,
    HTT_STATS_PPDU_FTYPE_DATA,
    HTT_STATS_PPDU_FTYPE_BAR,
    HTT_STATS_PPDU_FTYPE_MAX
}

// HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG Message
//
// details:
// HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG message is sent by host to
// configure RXDMA rings.
// The configuration is per ring based and includes both packet subtypes
// and PPDU/MPDU TLVs.
//
// The message would appear as follows:
//
// |31   29|28|27|26|25|24|23       16|15             8|7             0|
// |-------+--+--+--+--+--+-----------+----------------+---------------|
// | rsvd1 |ED|DT|OV|PS|SS|  ring_id  |     pdev_id    |    msg_type   |
// |-------------------------------------------------------------------|
// |              rsvd2               |           ring_buffer_size     |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_0                 |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_1                 |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_2                 |
// |-------------------------------------------------------------------|
// |                        packet_type_enable_flags_3                 |
// |-------------------------------------------------------------------|
// |                         tlv_filter_in_flags                       |
// |-------------------------------------------------------------------|
// Where:
// PS = pkt_swap
// SS = status_swap
// The message is interpreted as follows:
// dword0 - b'0:7   - msg_type: This will be set to
// HTT_H2T_MSG_TYPE_RX_RING_SELECTION_CFG
// b'8:15  - pdev_id:
// 0 (for rings at SOC/UMAC level),
// 1/2/3 mac id (for rings at LMAC level)
// b'16:23 - ring_id : Identify the ring to configure.
// More details can be got from enum htt_srng_ring_id
// b'24    - status_swap: 1 is to swap status TLV
// b'25    - pkt_swap:  1 is to swap packet TLV
// b'26    - rx_offset_valid (OV): flag to indicate rx offsets
// configuration fields are valid
// b'27    - drop_thresh_valid (DT): flag to indicate if the
// rx_drop_threshold field is valid
// b'28    - rx_mon_global_en: Enable/Disable global register
// configuration in Rx monitor module.
// b'29:31 - rsvd1:  reserved for future use
// dword1 - b'0:16  - ring_buffer_size: size of buffers referenced by rx ring,
// in byte units.
// Valid only for HW_TO_SW_RING and SW_TO_HW_RING
// - b'16:31 - rsvd2: Reserved for future use
// dword2 - b'0:31  - packet_type_enable_flags_0:
// Enable MGMT packet from 0b0000 to 0b1001
// bits from low to high: FP, MD, MO - 3 bits
// FP: Filter_Pass
// MD: Monitor_Direct
// MO: Monitor_Other
// 10 mgmt subtypes * 3 bits -> 30 bits
// Refer to PKT_TYPE_ENABLE_FLAG0_xxx_MGMT_xxx defs
// dword3 - b'0:31  - packet_type_enable_flags_1:
// Enable MGMT packet from 0b1010 to 0b1111
// bits from low to high: FP, MD, MO - 3 bits
// Refer to PKT_TYPE_ENABLE_FLAG1_xxx_MGMT_xxx defs
// dword4 - b'0:31 -  packet_type_enable_flags_2:
// Enable CTRL packet from 0b0000 to 0b1001
// bits from low to high: FP, MD, MO - 3 bits
// Refer to PKT_TYPE_ENABLE_FLAG2_xxx_CTRL_xxx defs
// dword5 - b'0:31  - packet_type_enable_flags_3:
// Enable CTRL packet from 0b1010 to 0b1111,
// MCAST_DATA, UCAST_DATA, NULL_DATA
// bits from low to high: FP, MD, MO - 3 bits
// Refer to PKT_TYPE_ENABLE_FLAG3_xxx_CTRL_xxx defs
// dword6 - b'0:31 -  tlv_filter_in_flags:
// Filter in Attention/MPDU/PPDU/Header/User tlvs
// Refer to CFG_TLV_FILTER_IN_FLAG defs
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_filter_tlv_flags {
    HTT_RX_FILTER_TLV_FLAGS_MPDU_START		= BIT(0),
    HTT_RX_FILTER_TLV_FLAGS_MSDU_START		= BIT(1),
    HTT_RX_FILTER_TLV_FLAGS_RX_PACKET		= BIT(2),
    HTT_RX_FILTER_TLV_FLAGS_MSDU_END		= BIT(3),
    HTT_RX_FILTER_TLV_FLAGS_MPDU_END		= BIT(4),
    HTT_RX_FILTER_TLV_FLAGS_PACKET_HEADER		= BIT(5),
    HTT_RX_FILTER_TLV_FLAGS_PER_MSDU_HEADER		= BIT(6),
    HTT_RX_FILTER_TLV_FLAGS_ATTENTION		= BIT(7),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_START		= BIT(8),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END		= BIT(9),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END_USER_STATS	= BIT(10),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END_USER_STATS_EXT	= BIT(11),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_END_STATUS_DONE	= BIT(12),
    HTT_RX_FILTER_TLV_FLAGS_PPDU_START_USER_INFO	= BIT(13),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_mgmt_pkt_filter_tlv_flags0 {
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_REQ		= BIT(0),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_REQ		= BIT(1),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_REQ		= BIT(2),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_RESP		= BIT(3),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_RESP		= BIT(4),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_ASSOC_RESP		= BIT(5),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_REQ	= BIT(6),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_REQ	= BIT(7),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_REQ	= BIT(8),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_RESP	= BIT(9),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_RESP	= BIT(10),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_REASSOC_RESP	= BIT(11),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_REQ		= BIT(12),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_REQ		= BIT(13),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_REQ		= BIT(14),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_RESP		= BIT(15),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_RESP		= BIT(16),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_RESP		= BIT(17),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_TIMING_ADV	= BIT(18),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_TIMING_ADV	= BIT(19),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_PROBE_TIMING_ADV	= BIT(20),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_RESERVED_7		= BIT(21),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_RESERVED_7		= BIT(22),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_RESERVED_7		= BIT(23),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_BEACON		= BIT(24),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_BEACON		= BIT(25),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_BEACON		= BIT(26),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS0_ATIM		= BIT(27),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS0_ATIM		= BIT(28),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS0_ATIM		= BIT(29),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_mgmt_pkt_filter_tlv_flags1 {
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_DISASSOC		= BIT(0),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_DISASSOC		= BIT(1),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_DISASSOC		= BIT(2),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_AUTH		= BIT(3),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_AUTH		= BIT(4),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_AUTH		= BIT(5),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_DEAUTH		= BIT(6),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_DEAUTH		= BIT(7),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_DEAUTH		= BIT(8),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION		= BIT(9),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION		= BIT(10),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION		= BIT(11),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION_NOACK	= BIT(12),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION_NOACK	= BIT(13),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_ACTION_NOACK	= BIT(14),
    HTT_RX_FP_MGMT_PKT_FILTER_TLV_FLAGS1_RESERVED_15	= BIT(15),
    HTT_RX_MD_MGMT_PKT_FILTER_TLV_FLAGS1_RESERVED_15	= BIT(16),
    HTT_RX_MO_MGMT_PKT_FILTER_TLV_FLAGS1_RESERVED_15	= BIT(17),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_ctrl_pkt_filter_tlv_flags2 {
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_1	= BIT(0),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_1	= BIT(1),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_1	= BIT(2),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_2	= BIT(3),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_2	= BIT(4),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_2	= BIT(5),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_TRIGGER	= BIT(6),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_TRIGGER	= BIT(7),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_TRIGGER	= BIT(8),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_4	= BIT(9),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_4	= BIT(10),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_RESERVED_4	= BIT(11),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_BF_REP_POLL	= BIT(12),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_BF_REP_POLL	= BIT(13),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_BF_REP_POLL	= BIT(14),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_VHT_NDP	= BIT(15),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_VHT_NDP	= BIT(16),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_VHT_NDP	= BIT(17),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_FRAME_EXT	= BIT(18),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_FRAME_EXT	= BIT(19),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_FRAME_EXT	= BIT(20),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_WRAPPER	= BIT(21),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_WRAPPER	= BIT(22),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_CTRL_WRAPPER	= BIT(23),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_BAR		= BIT(24),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_BAR		= BIT(25),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_BAR		= BIT(26),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS2_BA			= BIT(27),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS2_BA			= BIT(28),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS2_BA			= BIT(29),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_ctrl_pkt_filter_tlv_flags3 {
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_PSPOLL		= BIT(0),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_PSPOLL		= BIT(1),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_PSPOLL		= BIT(2),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_RTS		= BIT(3),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_RTS		= BIT(4),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_RTS		= BIT(5),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_CTS		= BIT(6),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_CTS		= BIT(7),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_CTS		= BIT(8),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_ACK		= BIT(9),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_ACK		= BIT(10),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_ACK		= BIT(11),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND		= BIT(12),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND		= BIT(13),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND		= BIT(14),
    HTT_RX_FP_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND_ACK		= BIT(15),
    HTT_RX_MD_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND_ACK		= BIT(16),
    HTT_RX_MO_CTRL_PKT_FILTER_TLV_FLAGS3_CFEND_ACK		= BIT(17),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_rx_data_pkt_filter_tlv_flasg3 {
    HTT_RX_FP_DATA_PKT_FILTER_TLV_FLASG3_MCAST	= BIT(18),
    HTT_RX_MD_DATA_PKT_FILTER_TLV_FLASG3_MCAST	= BIT(19),
    HTT_RX_MO_DATA_PKT_FILTER_TLV_FLASG3_MCAST	= BIT(20),
    HTT_RX_FP_DATA_PKT_FILTER_TLV_FLASG3_UCAST	= BIT(21),
    HTT_RX_MD_DATA_PKT_FILTER_TLV_FLASG3_UCAST	= BIT(22),
    HTT_RX_MO_DATA_PKT_FILTER_TLV_FLASG3_UCAST	= BIT(23),
    HTT_RX_FP_DATA_PKT_FILTER_TLV_FLASG3_NULL_DATA	= BIT(24),
    HTT_RX_MD_DATA_PKT_FILTER_TLV_FLASG3_NULL_DATA	= BIT(25),
    HTT_RX_MO_DATA_PKT_FILTER_TLV_FLASG3_NULL_DATA	= BIT(26),
}

// msdu start. mpdu end, attention, rx hdr tlv's are not subscribed

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_selection_cfg_cmd {
    pub info0: __le32,
    pub info1: __le32,
    pub pkt_type_en_flags0: __le32,
    pub pkt_type_en_flags1: __le32,
    pub pkt_type_en_flags2: __le32,
    pub pkt_type_en_flags3: __le32,
    pub rx_filter_tlv: __le32,
    pub rx_packet_offset: __le32,
    pub rx_mpdu_offset: __le32,
    pub rx_msdu_offset: __le32,
    pub rx_attn_offset: __le32,
    pub info2: __le32,
    pub reserved: [__le32; 2],
    pub rx_mpdu_start_end_mask: __le32,
    pub rx_msdu_end_word_mask: __le32,
    pub info3: __le32,
    pub __packed: },
pub const HTT_RX_RING_TLV_DROP_THRESHOLD_VALUE: c_int = 32;
pub const HTT_RX_RING_DEFAULT_DMA_LENGTH: c_uint = 0x7;
pub const HTT_RX_RING_PKT_TLV_OFFSET: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_rx_ring_tlv_filter {
    pub /: *mut *mut u32 rx_filter; / see htt_rx_filter_tlv_flags,
    pub /: *mut *mut u32 pkt_filter_flags0; / MGMT,
    pub /: *mut *mut u32 pkt_filter_flags1; / MGMT,
    pub /: *mut *mut u32 pkt_filter_flags2; / CTRL,
    pub /: *mut *mut u32 pkt_filter_flags3; / DATA,
    pub offset_valid: bool,
    pub rx_packet_offset: u16,
    pub rx_header_offset: u16,
    pub rx_mpdu_end_offset: u16,
    pub rx_mpdu_start_offset: u16,
    pub rx_msdu_end_offset: u16,
    pub rx_msdu_start_offset: u16,
    pub rx_attn_offset: u16,
    pub rx_mpdu_start_wmask: u16,
    pub rx_mpdu_end_wmask: u16,
    pub rx_msdu_end_wmask: u32,
    pub conf_len_ctrl: u32,
    pub conf_len_mgmt: u32,
    pub conf_len_data: u32,
    pub rx_drop_threshold: u16,
    pub enable_log_mgmt_type: bool,
    pub enable_log_ctrl_type: bool,
    pub enable_log_data_type: bool,
    pub enable_rx_tlv_offset: bool,
    pub rx_tlv_offset: u16,
    pub drop_threshold_valid: bool,
    pub rxmon_disable: bool,
}

pub const HTT_STATS_FRAME_CTRL_TYPE_MGMT: c_uint = 0x0;
pub const HTT_STATS_FRAME_CTRL_TYPE_CTRL: c_uint = 0x1;
pub const HTT_STATS_FRAME_CTRL_TYPE_DATA: c_uint = 0x2;
pub const HTT_STATS_FRAME_CTRL_TYPE_RESV: c_uint = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_ring_selection_cfg_cmd {
    pub info0: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub tlv_filter_mask_in0: __le32,
    pub tlv_filter_mask_in1: __le32,
    pub tlv_filter_mask_in2: __le32,
    pub tlv_filter_mask_in3: __le32,
    pub reserved: [__le32; 3],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tx_ring_tlv_filter {
    pub tx_mon_downstream_tlv_flags: u32,
    pub tx_mon_upstream_tlv_flags0: u32,
    pub tx_mon_upstream_tlv_flags1: u32,
    pub tx_mon_upstream_tlv_flags2: u32,
    pub tx_mon_mgmt_filter: bool,
    pub tx_mon_data_filter: bool,
    pub tx_mon_ctrl_filter: bool,
    pub tx_mon_pkt_dma_len: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_tx_mon_upstream_tlv_flags0 {
    HTT_TX_FILTER_TLV_FLAGS0_RESPONSE_START_STATUS		= BIT(1),
    HTT_TX_FILTER_TLV_FLAGS0_RESPONSE_END_STATUS		= BIT(2),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_START		= BIT(3),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_END		= BIT(4),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_START_PPDU	= BIT(5),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_USER_PPDU	= BIT(6),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_ACK_OR_BA	= BIT(7),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_1K_BA		= BIT(8),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_START_PROT	= BIT(9),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_PROT		= BIT(10),
    HTT_TX_FILTER_TLV_FLAGS0_TX_FES_STATUS_USER_RESPONSE	= BIT(11),
    HTT_TX_FILTER_TLV_FLAGS0_RX_FRAME_BITMAP_ACK		= BIT(12),
    HTT_TX_FILTER_TLV_FLAGS0_RX_FRAME_1K_BITMAP_ACK		= BIT(13),
    HTT_TX_FILTER_TLV_FLAGS0_COEX_TX_STATUS			= BIT(14),
    HTT_TX_FILTER_TLV_FLAGS0_RECEIVED_RESPONSE_INFO		= BIT(15),
    HTT_TX_FILTER_TLV_FLAGS0_RECEIVED_RESPONSE_INFO_PART2	= BIT(16),
}

// HTT message target->host
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_t2h_msg_type {
    HTT_T2H_MSG_TYPE_VERSION_CONF,
    HTT_T2H_MSG_TYPE_PEER_MAP	= 0x3,
    HTT_T2H_MSG_TYPE_PEER_UNMAP	= 0x4,
    HTT_T2H_MSG_TYPE_RX_ADDBA	= 0x5,
    HTT_T2H_MSG_TYPE_PKTLOG		= 0x8,
    HTT_T2H_MSG_TYPE_SEC_IND	= 0xb,
    HTT_T2H_MSG_TYPE_PEER_MAP2	= 0x1e,
    HTT_T2H_MSG_TYPE_PEER_UNMAP2	= 0x1f,
    HTT_T2H_MSG_TYPE_PPDU_STATS_IND = 0x1d,
    HTT_T2H_MSG_TYPE_EXT_STATS_CONF = 0x1c,
    HTT_T2H_MSG_TYPE_BKPRESSURE_EVENT_IND = 0x24,
    HTT_T2H_MSG_TYPE_MLO_TIMESTAMP_OFFSET_IND = 0x28,
    HTT_T2H_MSG_TYPE_MLO_RX_PEER_MAP = 0x29,
    HTT_T2H_MSG_TYPE_PEER_MAP3	= 0x2b,
    HTT_T2H_MSG_TYPE_VDEV_TXRX_STATS_PERIODIC_IND = 0x2c,
}

pub const HTT_TARGET_VERSION_MAJOR: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_version_conf_msg {
    pub version: __le32,
    pub __packed: },

pub const HTT_T2H_PEER_MAP_INFO2_NEXT_HOP_S: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_peer_map_event {
    pub info: __le32,
    pub mac_addr_l32: __le32,
    pub info1: __le32,
    pub info2: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_peer_unmap_event {
    pub info: __le32,
    pub mac_addr_l32: __le32,
    pub info1: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_mlo_peer_map_event {
    pub info0: __le32,
    pub mac_addr_l32: __le32,
    pub info1: __le32,
    pub reserved: [__le32; 5],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_resp_msg {
    pub version_msg: htt_t2h_version_conf_msg,
    pub peer_map_ev: htt_t2h_peer_map_event,
    pub peer_unmap_ev: htt_t2h_peer_unmap_event,
    pub mlo_peer_map_ev: htt_t2h_mlo_peer_map_event,
}

pub const HTT_VDEV_TXRX_STATS_COMMON_TLV: c_int = 0;
pub const HTT_VDEV_TXRX_STATS_HW_STATS_TLV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_vdev_txrx_stats_ind {
    pub vdev_id: __le32,
    pub rx_msdu_byte_cnt_lo: __le32,
    pub rx_msdu_byte_cnt_hi: __le32,
    pub rx_msdu_cnt_lo: __le32,
    pub rx_msdu_cnt_hi: __le32,
    pub tx_msdu_byte_cnt_lo: __le32,
    pub tx_msdu_byte_cnt_hi: __le32,
    pub tx_msdu_cnt_lo: __le32,
    pub tx_msdu_cnt_hi: __le32,
    pub tx_retry_cnt_lo: __le32,
    pub tx_retry_cnt_hi: __le32,
    pub tx_retry_byte_cnt_lo: __le32,
    pub tx_retry_byte_cnt_hi: __le32,
    pub tx_drop_cnt_lo: __le32,
    pub tx_drop_cnt_hi: __le32,
    pub tx_drop_byte_cnt_lo: __le32,
    pub tx_drop_byte_cnt_hi: __le32,
    pub msdu_ttl_cnt_lo: __le32,
    pub msdu_ttl_cnt_hi: __le32,
    pub msdu_ttl_byte_cnt_lo: __le32,
    pub msdu_ttl_byte_cnt_hi: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_t2h_vdev_common_stats_tlv {
    pub soc_drop_count_lo: __le32,
    pub soc_drop_count_hi: __le32,
    pub __packed: },
// ppdu stats
//
// @details
// The following field definitions describe the format of the HTT target
// to host ppdu stats indication message.
//
// |31                         16|15   12|11   10|9      8|7            0 |
// |----------------------------------------------------------------------|
// |    payload_size             | rsvd  |pdev_id|mac_id  |    msg type   |
// |----------------------------------------------------------------------|
// |                          ppdu_id                                     |
// |----------------------------------------------------------------------|
// |                        Timestamp in us                               |
// |----------------------------------------------------------------------|
// |                          reserved                                    |
// |----------------------------------------------------------------------|
// |                    type-specific stats info                          |
// |                     (see htt_ppdu_stats.h)                           |
// |----------------------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: Identifies this is a PPDU STATS indication
// message.
// Value: 0x1d
// - mac_id
// Bits 9:8
// Purpose: mac_id of this ppdu_id
// Value: 0-3
// - pdev_id
// Bits 11:10
// Purpose: pdev_id of this ppdu_id
// Value: 0-3
// 0 (for rings at SOC level),
// 1/2/3 PDEV -> 0/1/2
// - payload_size
// Bits 31:16
// Purpose: total tlv size
// Value: payload_size in bytes
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_ppdu_stats_msg {
    pub info: __le32,
    pub ppdu_id: __le32,
    pub timestamp: __le32,
    pub rsvd: __le32,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_tlv {
    pub header: __le32,
    pub value: [u8; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HTT_PPDU_STATS_BW {
    HTT_PPDU_STATS_BANDWIDTH_5MHZ   = 0,
    HTT_PPDU_STATS_BANDWIDTH_10MHZ  = 1,
    HTT_PPDU_STATS_BANDWIDTH_20MHZ  = 2,
    HTT_PPDU_STATS_BANDWIDTH_40MHZ  = 3,
    HTT_PPDU_STATS_BANDWIDTH_80MHZ  = 4,
    HTT_PPDU_STATS_BANDWIDTH_160MHZ = 5, /* includes 80+80 */
    HTT_PPDU_STATS_BANDWIDTH_DYN    = 6,
}

// bw - HTT_PPDU_STATS_BW

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_common {
    pub ppdu_id: __le32,
    pub sched_cmdid: __le16,
    pub ring_id: u8,
    pub num_users: u8,
    pub %HTT_PPDU_STATS_COMMON_FLAGS_*/: *mut *mut __le32 flags; /,
    pub chain_mask: __le32,
    pub /: *mut *mut __le32 fes_duration_us; / frame exchange sequence,
    pub ppdu_sch_eval_start_tstmp_us: __le32,
    pub ppdu_sch_end_tstmp_us: __le32,
    pub ppdu_start_tstmp_us: __le32,
// BIT [15 :  0] - phy mode (WLAN_PHY_MODE) with which ppdu was transmitted
// BIT [31 : 16] - bandwidth (in MHz) with which ppdu was transmitted
//
    pub phy_mode: __le16,
    pub bw_mhz: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_ppdu_stats_gi {
    HTT_PPDU_STATS_SGI_0_8_US,
    HTT_PPDU_STATS_SGI_0_4_US,
    HTT_PPDU_STATS_SGI_1_6_US,
    HTT_PPDU_STATS_SGI_3_2_US,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HTT_PPDU_STATS_PPDU_TYPE {
    HTT_PPDU_STATS_PPDU_TYPE_SU,
    HTT_PPDU_STATS_PPDU_TYPE_MU_MIMO,
    HTT_PPDU_STATS_PPDU_TYPE_MU_OFDMA,
    HTT_PPDU_STATS_PPDU_TYPE_MU_MIMO_OFDMA,
    HTT_PPDU_STATS_PPDU_TYPE_UL_TRIG,
    HTT_PPDU_STATS_PPDU_TYPE_BURST_BCN,
    HTT_PPDU_STATS_PPDU_TYPE_UL_BSR_RESP,
    HTT_PPDU_STATS_PPDU_TYPE_UL_BSR_TRIG,
    HTT_PPDU_STATS_PPDU_TYPE_UL_RESP,
    HTT_PPDU_STATS_PPDU_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_user_rate {
    pub tid_num: u8,
    pub reserved0: u8,
    pub sw_peer_id: __le16,
    pub %HTT_PPDU_STATS_USER_RATE_INFO0_*/: *mut *mut __le32 info0; /,
    pub ru_end: __le16,
    pub ru_start: __le16,
    pub resp_ru_end: __le16,
    pub resp_ru_start: __le16,
    pub /: *mut *mut __le32 info1; / %HTT_PPDU_STATS_USER_RATE_INFO1_,
    pub /: *mut *mut __le32 rate_flags; / %HTT_PPDU_STATS_USER_RATE_FLAGS_,
// Note: resp_rate_info is only valid for if resp_type is UL
    pub /: *mut *mut __le32 resp_rate_flags; / %HTT_PPDU_STATS_USER_RATE_RESP_FLAGS_,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum htt_ppdu_stats_usr_compln_status {
    HTT_PPDU_STATS_USER_STATUS_OK,
    HTT_PPDU_STATS_USER_STATUS_FILTERED,
    HTT_PPDU_STATS_USER_STATUS_RESP_TIMEOUT,
    HTT_PPDU_STATS_USER_STATUS_RESP_MISMATCH,
    HTT_PPDU_STATS_USER_STATUS_ABORT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_usr_cmpltn_cmn {
    pub status: u8,
    pub tid_num: u8,
    pub sw_peer_id: __le16,
// RSSI value of last ack packet (units = dB above noise floor)
    pub ack_rssi: __le32,
    pub mpdu_tried: __le16,
    pub mpdu_success: __le16,
    pub %HTT_PPDU_STATS_USR_CMPLTN_CMN_FLAGS_LONG_RETRIES*/: *mut *mut __le32 flags; /,
    pub __packed: },

pub const HTT_PPDU_STATS_NON_QOS_TID: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_usr_cmpltn_ack_ba_status {
    pub ppdu_id: __le32,
    pub sw_peer_id: __le16,
    pub reserved0: __le16,
    pub /: *mut *mut __le32 info; / %HTT_PPDU_STATS_USR_CMPLTN_CMN_INFO_,
    pub current_seq: __le16,
    pub start_seq: __le16,
    pub success_bytes: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_user_stats {
    pub peer_id: u16,
    pub delay_ba: u16,
    pub tlv_flags: u32,
    pub is_valid_peer_id: bool,
    pub rate: htt_ppdu_stats_user_rate,
    pub cmpltn_cmn: htt_ppdu_stats_usr_cmpltn_cmn,
    pub ack_ba: htt_ppdu_stats_usr_cmpltn_ack_ba_status,
}

pub const HTT_PPDU_STATS_MAX_USERS: c_int = 8;
pub const HTT_PPDU_DESC_MAX_DEPTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats {
    pub common: htt_ppdu_stats_common,
    pub user_stats: [htt_ppdu_user_stats; HTT_PPDU_STATS_MAX_USERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ppdu_stats_info {
    pub tlv_bitmap: u32,
    pub ppdu_id: u32,
    pub frame_type: u32,
    pub frame_ctrl: u32,
    pub delay_ba: u32,
    pub bar_num_users: u32,
    pub ppdu_stats: htt_ppdu_stats,
    pub list: list_head,
}

// @brief target -> host MLO offset indiciation message
//
// @details
// The following field definitions describe the format of the HTT target
// to host mlo offset indication message.
//
// |31        29|28    |26|25  22|21 16|15  13|12     10 |9     8|7     0|
// |---------------------------------------------------------------------|
// |   rsvd1    | mac_freq                    |chip_id   |pdev_id|msgtype|
// |---------------------------------------------------------------------|
// |                           sync_timestamp_lo_us                      |
// |---------------------------------------------------------------------|
// |                           sync_timestamp_hi_us                      |
// |---------------------------------------------------------------------|
// |                           mlo_offset_lo                             |
// |---------------------------------------------------------------------|
// |                           mlo_offset_hi                             |
// |---------------------------------------------------------------------|
// |                           mlo_offset_clcks                          |
// |---------------------------------------------------------------------|
// |   rsvd2           | mlo_comp_clks |mlo_comp_us                      |
// |---------------------------------------------------------------------|
// |   rsvd3                   |mlo_comp_timer                           |
// |---------------------------------------------------------------------|
// Header fields
// - MSG_TYPE
// Bits 7:0
// Purpose: Identifies this is a MLO offset indication msg
// - PDEV_ID
// Bits 9:8
// Purpose: Pdev of this MLO offset
// - CHIP_ID
// Bits 12:10
// Purpose: chip_id of this MLO offset
// - MAC_FREQ
// Bits 28:13
// - SYNC_TIMESTAMP_LO_US
// Purpose: clock frequency of the mac HW block in MHz
// Bits: 31:0
// Purpose: lower 32 bits of the WLAN global time stamp at which
// last sync interrupt was received
// - SYNC_TIMESTAMP_HI_US
// Bits: 31:0
// Purpose: upper 32 bits of WLAN global time stamp at which
// last sync interrupt was received
// - MLO_OFFSET_LO
// Bits: 31:0
// Purpose: lower 32 bits of the MLO offset in us
// - MLO_OFFSET_HI
// Bits: 31:0
// Purpose: upper 32 bits of the MLO offset in us
// - MLO_COMP_US
// Bits: 15:0
// Purpose: MLO time stamp compensation applied in us
// - MLO_COMP_CLCKS
// Bits: 25:16
// Purpose: MLO time stamp compensation applied in clock ticks
// - MLO_COMP_TIMER
// Bits: 21:0
// Purpose: Periodic timer at which compensation is applied
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_htt_mlo_offset_msg {
    pub info: __le32,
    pub sync_timestamp_lo_us: __le32,
    pub sync_timestamp_hi_us: __le32,
    pub mlo_offset_hi: __le32,
    pub mlo_offset_lo: __le32,
    pub mlo_offset_clks: __le32,
    pub mlo_comp_clks: __le32,
    pub mlo_comp_timer: __le32,
    pub __packed: },
// @brief host -> target FW extended statistics retrieve
//
// @details
// The following field definitions describe the format of the HTT host
// to target FW extended stats retrieve message.
// The message specifies the type of stats the host wants to retrieve.
//
// |31          24|23          16|15           8|7            0|
// |-----------------------------------------------------------|
// |   reserved   | stats type   |   pdev_mask  |   msg type   |
// |-----------------------------------------------------------|
// |                   config param [0]                        |
// |-----------------------------------------------------------|
// |                   config param [1]                        |
// |-----------------------------------------------------------|
// |                   config param [2]                        |
// |-----------------------------------------------------------|
// |                   config param [3]                        |
// |-----------------------------------------------------------|
// |                         reserved                          |
// |-----------------------------------------------------------|
// |                        cookie LSBs                        |
// |-----------------------------------------------------------|
// |                        cookie MSBs                        |
// |-----------------------------------------------------------|
// Header fields:
// - MSG_TYPE
// Bits 7:0
// Purpose: identifies this is a extended stats upload request message
// Value: 0x10
// - PDEV_MASK
// Bits 8:15
// Purpose: identifies the mask of PDEVs to retrieve stats from
// Value: This is a overloaded field, refer to usage and interpretation of
// PDEV in interface document.
// Bit   8    :  Reserved for SOC stats
// Bit 9 - 15 :  Indicates PDEV_MASK in DBDC
// Indicates MACID_MASK in DBS
// - STATS_TYPE
// Bits 23:16
// Purpose: identifies which FW statistics to upload
// Value: Defined by htt_dbg_ext_stats_type (see htt_stats.h)
// - Reserved
// Bits 31:24
// - CONFIG_PARAM [0]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - CONFIG_PARAM [1]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - CONFIG_PARAM [2]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - CONFIG_PARAM [3]
// Bits 31:0
// Purpose: give an opaque configuration value to the specified stats type
// Value: stats-type specific configuration value
// Refer to htt_stats.h for interpretation for each stats sub_type
// - Reserved [31:0] for future use.
// - COOKIE_LSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: LSBs of the opaque cookie specified by the host-side requestor
// - COOKIE_MSBS
// Bits 31:0
// Purpose: Provide a mechanism to match a target->host stats confirmation
// message with its preceding host->target stats request message.
// Value: MSBs of the opaque cookie specified by the host-side requestor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ext_stats_cfg_hdr {
    pub msg_type: u8,
    pub pdev_mask: u8,
    pub stats_type: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ext_stats_cfg_cmd {
    pub hdr: htt_ext_stats_cfg_hdr,
    pub cfg_param0: __le32,
    pub cfg_param1: __le32,
    pub cfg_param2: __le32,
    pub cfg_param3: __le32,
    pub reserved: __le32,
    pub cookie_lsb: __le32,
    pub cookie_msb: __le32,
    pub __packed: },
// htt stats config default params
pub const HTT_STAT_DEFAULT_RESET_START_OFFSET: c_int = 0;
pub const HTT_STAT_DEFAULT_CFG0_ALL_HWQS: c_uint = 0xffffffff;
pub const HTT_STAT_DEFAULT_CFG0_ALL_TXQS: c_uint = 0xffffffff;
pub const HTT_STAT_DEFAULT_CFG0_ALL_CMDQS: c_uint = 0xffff;
pub const HTT_STAT_DEFAULT_CFG0_ALL_RINGS: c_uint = 0xffff;
pub const HTT_STAT_DEFAULT_CFG0_ACTIVE_PEERS: c_uint = 0xff;
pub const HTT_STAT_DEFAULT_CFG0_CCA_CUMULATIVE: c_uint = 0x00;
pub const HTT_STAT_DEFAULT_CFG0_ACTIVE_VDEVS: c_uint = 0x00;
// HTT_DBG_EXT_STATS_PEER_INFO
// PARAMS:
// @config_param0:
// [Bit0] - [0] for sw_peer_id, [1] for mac_addr based request
// [Bit15 : Bit 1] htt_peer_stats_req_mode_t
// [Bit31 : Bit16] sw_peer_id
// @config_param1:
// peer_stats_req_type_mask:32 (enum htt_peer_stats_tlv_enum)
// 0 bit htt_peer_stats_cmn_tlv
// 1 bit htt_peer_details_tlv
// 2 bit htt_tx_peer_rate_stats_tlv
// 3 bit htt_rx_peer_rate_stats_tlv
// 4 bit htt_tx_tid_stats_tlv/htt_tx_tid_stats_v1_tlv
// 5 bit htt_rx_tid_stats_tlv
// 6 bit htt_msdu_flow_stats_tlv
// @config_param2: [Bit31 : Bit0] mac_addr31to0
// @config_param3: [Bit15 : Bit0] mac_addr47to32
// [Bit31 : Bit16] reserved
//

pub const HTT_STAT_DEFAULT_PEER_REQ_TYPE: c_uint = 0x7f;
// Used to set different configs to the specified stats type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_ext_stats_cfg_params {
    pub cfg0: u32,
    pub cfg1: u32,
    pub cfg2: u32,
    pub cfg3: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vdev_stats_offload_timer_duration {
    ATH12K_STATS_TIMER_DUR_500MS = 1,
    ATH12K_STATS_TIMER_DUR_1SEC = 2,
    ATH12K_STATS_TIMER_DUR_2SEC = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct htt_mac_addr {
    pub mac_addr_l32: __le32,
    pub mac_addr_h16: __le32,
    pub __packed: },
    pub dp): *mut int ath12k_dp_htt_connect(struct ath12k_dp,
    pub ring_type): int mac_id, enum hal_ring_type,
    pub skb): *mut sk_buff,
    pub data): *mut c_void,
    pub ab): *mut int ath12k_dp_tx_htt_h2t_ver_req_msg(struct ath12k_base,
    pub mask): *mut *mut int ath12k_dp_tx_htt_h2t_ppdu_stats_req(struct ath12k ar, u32,
    pub cookie): u64,
    pub reset): *mut *mut int ath12k_dp_tx_htt_rx_monitor_mode_ring_config(struct ath12k ar, bool,
    pub tlv_filter): *mut htt_rx_ring_tlv_filter,
    pub htt_tlv_filter): *mut htt_tx_ring_tlv_filter,
    pub reset): *mut *mut int ath12k_dp_tx_htt_monitor_mode_ring_config(struct ath12k ar, bool,
