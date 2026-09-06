//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/ef100_regs.h
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
// Driver for Solarflare network controllers and boards
// Copyright 2018 Solarflare Communications Inc.
// Copyright 2019-2022 Xilinx Inc.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License version 2 as published
// by the Free Software Foundation, incorporated herein by reference.
//
// EF100 hardware architecture definitions have a name prefix following
// the format:
//
// E<type>_<min-rev><max-rev>_
//
// The following <type> strings are used:
//
// MMIO register  Host memory structure
// -------------------------------------------------------------
// Address     R
// Bitfield    RF             SF
// Enumerator  FE             SE
//
// <min-rev> is the first revision to which the definition applies:
//
// G: Riverhead
//
// If the definition has been changed or removed in later revisions
// then <max-rev> is the last revision to which the definition applies;
// otherwise it is "Z".
//
// EF100 registers and descriptors
//
// HW_REV_ID_REG: Hardware revision info register
pub const ER_GZ_HW_REV_ID: c_uint = 0x00000000;
// NIC_REV_ID: SoftNIC revision info register
pub const ER_GZ_NIC_REV_ID: c_uint = 0x00000004;
// NIC_MAGIC: Signature register that should contain a well-known value
pub const ER_GZ_NIC_MAGIC: c_uint = 0x00000008;
pub const ERF_GZ_NIC_MAGIC_LBN: c_int = 0;
pub const ERF_GZ_NIC_MAGIC_WIDTH: c_int = 32;
pub const EFE_GZ_NIC_MAGIC_EXPECTED: c_uint = 0xEF100FCB;
// MC_SFT_STATUS: MC soft status
pub const ER_GZ_MC_SFT_STATUS: c_uint = 0x00000010;
pub const ER_GZ_MC_SFT_STATUS_STEP: c_int = 4;
pub const ER_GZ_MC_SFT_STATUS_ROWS: c_int = 2;
// MC_DB_LWRD_REG: MC doorbell register, low word
pub const ER_GZ_MC_DB_LWRD: c_uint = 0x00000020;
// MC_DB_HWRD_REG: MC doorbell register, high word
pub const ER_GZ_MC_DB_HWRD: c_uint = 0x00000024;
// EVQ_INT_PRIME: Prime EVQ
pub const ER_GZ_EVQ_INT_PRIME: c_uint = 0x00000040;
pub const ERF_GZ_IDX_LBN: c_int = 16;
pub const ERF_GZ_IDX_WIDTH: c_int = 16;
pub const ERF_GZ_EVQ_ID_LBN: c_int = 0;
pub const ERF_GZ_EVQ_ID_WIDTH: c_int = 16;
// INT_AGG_RING_PRIME: Prime interrupt aggregation ring.
pub const ER_GZ_INT_AGG_RING_PRIME: c_uint = 0x00000048;
// defined as ERF_GZ_IDX_LBN 16; access=WO reset=0x0
// defined as ERF_GZ_IDX_WIDTH 16
pub const ERF_GZ_RING_ID_LBN: c_int = 0;
pub const ERF_GZ_RING_ID_WIDTH: c_int = 16;
// EVQ_TMR: EVQ timer control
pub const ER_GZ_EVQ_TMR: c_uint = 0x00000104;
pub const ER_GZ_EVQ_TMR_STEP: c_int = 65536;
pub const ER_GZ_EVQ_TMR_ROWS: c_int = 1024;
// EVQ_UNSOL_CREDIT_GRANT_SEQ: Grant credits for unsolicited events.
pub const ER_GZ_EVQ_UNSOL_CREDIT_GRANT_SEQ: c_uint = 0x00000108;
pub const ER_GZ_EVQ_UNSOL_CREDIT_GRANT_SEQ_STEP: c_int = 65536;
pub const ER_GZ_EVQ_UNSOL_CREDIT_GRANT_SEQ_ROWS: c_int = 1024;
// EVQ_DESC_CREDIT_GRANT_SEQ: Grant credits for descriptor proxy events.
pub const ER_GZ_EVQ_DESC_CREDIT_GRANT_SEQ: c_uint = 0x00000110;
pub const ER_GZ_EVQ_DESC_CREDIT_GRANT_SEQ_STEP: c_int = 65536;
pub const ER_GZ_EVQ_DESC_CREDIT_GRANT_SEQ_ROWS: c_int = 1024;
// RX_RING_DOORBELL: Ring Rx doorbell.
pub const ER_GZ_RX_RING_DOORBELL: c_uint = 0x00000180;
pub const ER_GZ_RX_RING_DOORBELL_STEP: c_int = 65536;
pub const ER_GZ_RX_RING_DOORBELL_ROWS: c_int = 1024;
pub const ERF_GZ_RX_RING_PIDX_LBN: c_int = 16;
pub const ERF_GZ_RX_RING_PIDX_WIDTH: c_int = 16;
// TX_RING_DOORBELL: Ring Tx doorbell.
pub const ER_GZ_TX_RING_DOORBELL: c_uint = 0x00000200;
pub const ER_GZ_TX_RING_DOORBELL_STEP: c_int = 65536;
pub const ER_GZ_TX_RING_DOORBELL_ROWS: c_int = 1024;
pub const ERF_GZ_TX_RING_PIDX_LBN: c_int = 16;
pub const ERF_GZ_TX_RING_PIDX_WIDTH: c_int = 16;
// TX_DESC_PUSH: Tx ring descriptor push. Reserved for future use.
pub const ER_GZ_TX_DESC_PUSH: c_uint = 0x00000210;
pub const ER_GZ_TX_DESC_PUSH_STEP: c_int = 65536;
pub const ER_GZ_TX_DESC_PUSH_ROWS: c_int = 1024;
// THE_TIME: NIC hardware time
pub const ER_GZ_THE_TIME: c_uint = 0x00000280;
pub const ER_GZ_THE_TIME_STEP: c_int = 65536;
pub const ER_GZ_THE_TIME_ROWS: c_int = 1024;
pub const ERF_GZ_THE_TIME_SECS_LBN: c_int = 32;
pub const ERF_GZ_THE_TIME_SECS_WIDTH: c_int = 32;
pub const ERF_GZ_THE_TIME_NANOS_LBN: c_int = 2;
pub const ERF_GZ_THE_TIME_NANOS_WIDTH: c_int = 30;
pub const ERF_GZ_THE_TIME_CLOCK_IN_SYNC_LBN: c_int = 1;
pub const ERF_GZ_THE_TIME_CLOCK_IN_SYNC_WIDTH: c_int = 1;
pub const ERF_GZ_THE_TIME_CLOCK_IS_SET_LBN: c_int = 0;
pub const ERF_GZ_THE_TIME_CLOCK_IS_SET_WIDTH: c_int = 1;
// PARAMS_TLV_LEN: Size of design parameters area in bytes
pub const ER_GZ_PARAMS_TLV_LEN: c_uint = 0x00000c00;
pub const ER_GZ_PARAMS_TLV_LEN_STEP: c_int = 65536;
pub const ER_GZ_PARAMS_TLV_LEN_ROWS: c_int = 1024;
// PARAMS_TLV: Design parameters
pub const ER_GZ_PARAMS_TLV: c_uint = 0x00000c04;
pub const ER_GZ_PARAMS_TLV_STEP: c_int = 65536;
pub const ER_GZ_PARAMS_TLV_ROWS: c_int = 1024;
// EW_EMBEDDED_EVENT
pub const ESF_GZ_EV_256_EVENT_LBN: c_int = 0;
pub const ESF_GZ_EV_256_EVENT_WIDTH: c_int = 64;
pub const ESE_GZ_EW_EMBEDDED_EVENT_STRUCT_SIZE: c_int = 64;
// NMMU_PAGESZ_2M_ADDR
pub const ESF_GZ_NMMU_2M_PAGE_SIZE_ID_LBN: c_int = 59;
pub const ESF_GZ_NMMU_2M_PAGE_SIZE_ID_WIDTH: c_int = 5;
pub const ESE_GZ_NMMU_PAGE_SIZE_2M: c_int = 9;
pub const ESF_GZ_NMMU_2M_PAGE_ID_LBN: c_int = 21;
pub const ESF_GZ_NMMU_2M_PAGE_ID_WIDTH: c_int = 38;
pub const ESF_GZ_NMMU_2M_PAGE_OFFSET_LBN: c_int = 0;
pub const ESF_GZ_NMMU_2M_PAGE_OFFSET_WIDTH: c_int = 21;
pub const ESE_GZ_NMMU_PAGESZ_2M_ADDR_STRUCT_SIZE: c_int = 64;
// PARAM_TLV
pub const ESF_GZ_TLV_VALUE_LBN: c_int = 16;
pub const ESF_GZ_TLV_VALUE_WIDTH: c_int = 8;
pub const ESE_GZ_TLV_VALUE_LENMIN: c_int = 8;
pub const ESE_GZ_TLV_VALUE_LENMAX: c_int = 2040;
pub const ESF_GZ_TLV_LEN_LBN: c_int = 8;
pub const ESF_GZ_TLV_LEN_WIDTH: c_int = 8;
pub const ESF_GZ_TLV_TYPE_LBN: c_int = 0;
pub const ESF_GZ_TLV_TYPE_WIDTH: c_int = 8;
pub const ESE_GZ_DP_NMMU_GROUP_SIZE: c_int = 5;
pub const ESE_GZ_DP_EVQ_UNSOL_CREDIT_SEQ_BITS: c_int = 4;
pub const ESE_GZ_DP_TX_EV_NUM_DESCS_BITS: c_int = 3;
pub const ESE_GZ_DP_RX_EV_NUM_PACKETS_BITS: c_int = 2;
pub const ESE_GZ_DP_PARTIAL_TSTAMP_SUB_NANO_BITS: c_int = 1;
pub const ESE_GZ_DP_PAD: c_int = 0;
pub const ESE_GZ_PARAM_TLV_STRUCT_SIZE: c_int = 24;
// PCI_EXPRESS_XCAP_HDR
pub const ESF_GZ_PCI_EXPRESS_XCAP_NEXT_LBN: c_int = 20;
pub const ESF_GZ_PCI_EXPRESS_XCAP_NEXT_WIDTH: c_int = 12;
pub const ESF_GZ_PCI_EXPRESS_XCAP_VER_LBN: c_int = 16;
pub const ESF_GZ_PCI_EXPRESS_XCAP_VER_WIDTH: c_int = 4;
pub const ESE_GZ_PCI_EXPRESS_XCAP_VER_VSEC: c_int = 1;
pub const ESF_GZ_PCI_EXPRESS_XCAP_ID_LBN: c_int = 0;
pub const ESF_GZ_PCI_EXPRESS_XCAP_ID_WIDTH: c_int = 16;
pub const ESE_GZ_PCI_EXPRESS_XCAP_ID_VNDR: c_uint = 0xb;
pub const ESE_GZ_PCI_EXPRESS_XCAP_HDR_STRUCT_SIZE: c_int = 32;
// RHEAD_BASE_EVENT
pub const ESF_GZ_E_TYPE_LBN: c_int = 60;
pub const ESF_GZ_E_TYPE_WIDTH: c_int = 4;
pub const ESF_GZ_EV_EVQ_PHASE_LBN: c_int = 59;
pub const ESF_GZ_EV_EVQ_PHASE_WIDTH: c_int = 1;
pub const ESE_GZ_RHEAD_BASE_EVENT_STRUCT_SIZE: c_int = 64;
// RHEAD_EW_EVENT
pub const ESF_GZ_EV_256_EV32_PHASE_LBN: c_int = 255;
pub const ESF_GZ_EV_256_EV32_PHASE_WIDTH: c_int = 1;
pub const ESF_GZ_EV_256_EV32_TYPE_LBN: c_int = 251;
pub const ESF_GZ_EV_256_EV32_TYPE_WIDTH: c_int = 4;
pub const ESE_GZ_EF100_EVEW_VIRTQ_DESC: c_int = 2;
pub const ESE_GZ_EF100_EVEW_TXQ_DESC: c_int = 1;
pub const ESE_GZ_EF100_EVEW_64BIT: c_int = 0;
pub const ESE_GZ_RHEAD_EW_EVENT_STRUCT_SIZE: c_int = 256;
// RX_DESC
pub const ESF_GZ_RX_BUF_ADDR_LBN: c_int = 0;
pub const ESF_GZ_RX_BUF_ADDR_WIDTH: c_int = 64;
pub const ESE_GZ_RX_DESC_STRUCT_SIZE: c_int = 64;
// TXQ_DESC_PROXY_EVENT
pub const ESF_GZ_EV_TXQ_DP_VI_ID_LBN: c_int = 128;
pub const ESF_GZ_EV_TXQ_DP_VI_ID_WIDTH: c_int = 16;
pub const ESF_GZ_EV_TXQ_DP_TXQ_DESC_LBN: c_int = 0;
pub const ESF_GZ_EV_TXQ_DP_TXQ_DESC_WIDTH: c_int = 128;
pub const ESE_GZ_TXQ_DESC_PROXY_EVENT_STRUCT_SIZE: c_int = 144;
// TX_DESC_TYPE
pub const ESF_GZ_TX_DESC_TYPE_LBN: c_int = 124;
pub const ESF_GZ_TX_DESC_TYPE_WIDTH: c_int = 4;
pub const ESE_GZ_TX_DESC_TYPE_DESC2CMPT: c_int = 7;
pub const ESE_GZ_TX_DESC_TYPE_MEM2MEM: c_int = 4;
pub const ESE_GZ_TX_DESC_TYPE_SEG: c_int = 3;
pub const ESE_GZ_TX_DESC_TYPE_TSO: c_int = 2;
pub const ESE_GZ_TX_DESC_TYPE_PREFIX: c_int = 1;
pub const ESE_GZ_TX_DESC_TYPE_SEND: c_int = 0;
pub const ESE_GZ_TX_DESC_TYPE_STRUCT_SIZE: c_int = 128;
// VIRTQ_DESC_PROXY_EVENT
pub const ESF_GZ_EV_VQ_DP_AVAIL_ENTRY_LBN: c_int = 144;
pub const ESF_GZ_EV_VQ_DP_AVAIL_ENTRY_WIDTH: c_int = 16;
pub const ESF_GZ_EV_VQ_DP_VI_ID_LBN: c_int = 128;
pub const ESF_GZ_EV_VQ_DP_VI_ID_WIDTH: c_int = 16;
pub const ESF_GZ_EV_VQ_DP_VIRTQ_DESC_LBN: c_int = 0;
pub const ESF_GZ_EV_VQ_DP_VIRTQ_DESC_WIDTH: c_int = 128;
pub const ESE_GZ_VIRTQ_DESC_PROXY_EVENT_STRUCT_SIZE: c_int = 160;
// XIL_CFGBAR_TBL_ENTRY
pub const ESF_GZ_CFGBAR_CONT_CAP_OFF_HI_LBN: c_int = 96;
pub const ESF_GZ_CFGBAR_CONT_CAP_OFF_HI_WIDTH: c_int = 32;
pub const ESF_GZ_CFGBAR_CONT_CAP_OFFSET_LBN: c_int = 68;
pub const ESF_GZ_CFGBAR_CONT_CAP_OFFSET_WIDTH: c_int = 60;
pub const ESE_GZ_CONT_CAP_OFFSET_BYTES_SHIFT: c_int = 4;
pub const ESF_GZ_CFGBAR_EF100_FUNC_CTL_WIN_OFF_LBN: c_int = 67;
pub const ESF_GZ_CFGBAR_EF100_FUNC_CTL_WIN_OFF_WIDTH: c_int = 29;
pub const ESE_GZ_EF100_FUNC_CTL_WIN_OFF_SHIFT: c_int = 4;
pub const ESF_GZ_CFGBAR_CONT_CAP_OFF_LO_LBN: c_int = 68;
pub const ESF_GZ_CFGBAR_CONT_CAP_OFF_LO_WIDTH: c_int = 28;
pub const ESF_GZ_CFGBAR_CONT_CAP_RSV_LBN: c_int = 67;
pub const ESF_GZ_CFGBAR_CONT_CAP_RSV_WIDTH: c_int = 1;
pub const ESF_GZ_CFGBAR_EF100_BAR_LBN: c_int = 64;
pub const ESF_GZ_CFGBAR_EF100_BAR_WIDTH: c_int = 3;
pub const ESE_GZ_CFGBAR_EF100_BAR_NUM_INVALID: c_int = 7;
pub const ESE_GZ_CFGBAR_EF100_BAR_NUM_EXPANSION_ROM: c_int = 6;
pub const ESF_GZ_CFGBAR_CONT_CAP_BAR_LBN: c_int = 64;
pub const ESF_GZ_CFGBAR_CONT_CAP_BAR_WIDTH: c_int = 3;
pub const ESE_GZ_CFGBAR_CONT_CAP_BAR_NUM_INVALID: c_int = 7;
pub const ESE_GZ_CFGBAR_CONT_CAP_BAR_NUM_EXPANSION_ROM: c_int = 6;
pub const ESF_GZ_CFGBAR_ENTRY_SIZE_LBN: c_int = 32;
pub const ESF_GZ_CFGBAR_ENTRY_SIZE_WIDTH: c_int = 32;
pub const ESE_GZ_CFGBAR_ENTRY_SIZE_EF100: c_int = 12;
pub const ESE_GZ_CFGBAR_ENTRY_HEADER_SIZE: c_int = 8;
pub const ESF_GZ_CFGBAR_ENTRY_LAST_LBN: c_int = 28;
pub const ESF_GZ_CFGBAR_ENTRY_LAST_WIDTH: c_int = 1;
pub const ESF_GZ_CFGBAR_ENTRY_REV_LBN: c_int = 20;
pub const ESF_GZ_CFGBAR_ENTRY_REV_WIDTH: c_int = 8;
pub const ESE_GZ_CFGBAR_ENTRY_REV_EF100: c_int = 0;
pub const ESF_GZ_CFGBAR_ENTRY_FORMAT_LBN: c_int = 0;
pub const ESF_GZ_CFGBAR_ENTRY_FORMAT_WIDTH: c_int = 20;
pub const ESE_GZ_CFGBAR_ENTRY_LAST: c_uint = 0xfffff;
pub const ESE_GZ_CFGBAR_ENTRY_CONT_CAP_ADDR: c_uint = 0xffffe;
pub const ESE_GZ_CFGBAR_ENTRY_EF100: c_uint = 0xef100;
pub const ESE_GZ_XIL_CFGBAR_TBL_ENTRY_STRUCT_SIZE: c_int = 128;
// XIL_CFGBAR_VSEC
pub const ESF_GZ_VSEC_TBL_OFF_HI_LBN: c_int = 64;
pub const ESF_GZ_VSEC_TBL_OFF_HI_WIDTH: c_int = 32;
pub const ESE_GZ_VSEC_TBL_OFF_HI_BYTES_SHIFT: c_int = 32;
pub const ESF_GZ_VSEC_TBL_OFF_LO_LBN: c_int = 36;
pub const ESF_GZ_VSEC_TBL_OFF_LO_WIDTH: c_int = 28;
pub const ESE_GZ_VSEC_TBL_OFF_LO_BYTES_SHIFT: c_int = 4;
pub const ESF_GZ_VSEC_TBL_BAR_LBN: c_int = 32;
pub const ESF_GZ_VSEC_TBL_BAR_WIDTH: c_int = 4;
pub const ESE_GZ_VSEC_BAR_NUM_INVALID: c_int = 7;
pub const ESE_GZ_VSEC_BAR_NUM_EXPANSION_ROM: c_int = 6;
pub const ESF_GZ_VSEC_LEN_LBN: c_int = 20;
pub const ESF_GZ_VSEC_LEN_WIDTH: c_int = 12;
pub const ESE_GZ_VSEC_LEN_HIGH_OFFT: c_int = 16;
pub const ESE_GZ_VSEC_LEN_MIN: c_int = 12;
pub const ESF_GZ_VSEC_VER_LBN: c_int = 16;
pub const ESF_GZ_VSEC_VER_WIDTH: c_int = 4;
pub const ESE_GZ_VSEC_VER_XIL_CFGBAR: c_int = 0;
pub const ESF_GZ_VSEC_ID_LBN: c_int = 0;
pub const ESF_GZ_VSEC_ID_WIDTH: c_int = 16;
pub const ESE_GZ_XILINX_VSEC_ID: c_uint = 0x20;
pub const ESE_GZ_XIL_CFGBAR_VSEC_STRUCT_SIZE: c_int = 96;
// rh_egres_hclass
pub const ESF_GZ_RX_PREFIX_HCLASS_TUN_OUTER_L4_CSUM_LBN: c_int = 15;
pub const ESF_GZ_RX_PREFIX_HCLASS_TUN_OUTER_L4_CSUM_WIDTH: c_int = 1;
pub const ESF_GZ_RX_PREFIX_HCLASS_TUN_OUTER_L3_CLASS_LBN: c_int = 13;
pub const ESF_GZ_RX_PREFIX_HCLASS_TUN_OUTER_L3_CLASS_WIDTH: c_int = 2;
pub const ESF_GZ_RX_PREFIX_HCLASS_NT_OR_INNER_L4_CSUM_LBN: c_int = 12;
pub const ESF_GZ_RX_PREFIX_HCLASS_NT_OR_INNER_L4_CSUM_WIDTH: c_int = 1;
pub const ESF_GZ_RX_PREFIX_HCLASS_NT_OR_INNER_L4_CLASS_LBN: c_int = 10;
pub const ESF_GZ_RX_PREFIX_HCLASS_NT_OR_INNER_L4_CLASS_WIDTH: c_int = 2;
pub const ESF_GZ_RX_PREFIX_HCLASS_NT_OR_INNER_L3_CLASS_LBN: c_int = 8;
pub const ESF_GZ_RX_PREFIX_HCLASS_NT_OR_INNER_L3_CLASS_WIDTH: c_int = 2;
pub const ESF_GZ_RX_PREFIX_HCLASS_TUNNEL_CLASS_LBN: c_int = 5;
pub const ESF_GZ_RX_PREFIX_HCLASS_TUNNEL_CLASS_WIDTH: c_int = 3;
pub const ESF_GZ_RX_PREFIX_HCLASS_L2_N_VLAN_LBN: c_int = 3;
pub const ESF_GZ_RX_PREFIX_HCLASS_L2_N_VLAN_WIDTH: c_int = 2;
pub const ESF_GZ_RX_PREFIX_HCLASS_L2_CLASS_LBN: c_int = 2;
pub const ESF_GZ_RX_PREFIX_HCLASS_L2_CLASS_WIDTH: c_int = 1;
pub const ESF_GZ_RX_PREFIX_HCLASS_L2_STATUS_LBN: c_int = 0;
pub const ESF_GZ_RX_PREFIX_HCLASS_L2_STATUS_WIDTH: c_int = 2;
pub const ESE_GZ_RH_EGRES_HCLASS_STRUCT_SIZE: c_int = 16;
// sf_driver
pub const ESF_GZ_DRIVER_E_TYPE_LBN: c_int = 60;
pub const ESF_GZ_DRIVER_E_TYPE_WIDTH: c_int = 4;
pub const ESF_GZ_DRIVER_PHASE_LBN: c_int = 59;
pub const ESF_GZ_DRIVER_PHASE_WIDTH: c_int = 1;
pub const ESF_GZ_DRIVER_DATA_LBN: c_int = 0;
pub const ESF_GZ_DRIVER_DATA_WIDTH: c_int = 59;
pub const ESE_GZ_SF_DRIVER_STRUCT_SIZE: c_int = 64;
// sf_ev_rsvd
pub const ESF_GZ_EV_RSVD_TBD_NEXT_LBN: c_int = 34;
pub const ESF_GZ_EV_RSVD_TBD_NEXT_WIDTH: c_int = 3;
pub const ESF_GZ_EV_RSVD_EVENT_GEN_FLAGS_LBN: c_int = 30;
pub const ESF_GZ_EV_RSVD_EVENT_GEN_FLAGS_WIDTH: c_int = 4;
pub const ESF_GZ_EV_RSVD_SRC_QID_LBN: c_int = 18;
pub const ESF_GZ_EV_RSVD_SRC_QID_WIDTH: c_int = 12;
pub const ESF_GZ_EV_RSVD_SEQ_NUM_LBN: c_int = 2;
pub const ESF_GZ_EV_RSVD_SEQ_NUM_WIDTH: c_int = 16;
pub const ESF_GZ_EV_RSVD_TBD_LBN: c_int = 0;
pub const ESF_GZ_EV_RSVD_TBD_WIDTH: c_int = 2;
pub const ESE_GZ_SF_EV_RSVD_STRUCT_SIZE: c_int = 37;
// sf_flush_evnt
pub const ESF_GZ_EV_FLSH_E_TYPE_LBN: c_int = 60;
pub const ESF_GZ_EV_FLSH_E_TYPE_WIDTH: c_int = 4;
pub const ESF_GZ_EV_FLSH_PHASE_LBN: c_int = 59;
pub const ESF_GZ_EV_FLSH_PHASE_WIDTH: c_int = 1;
pub const ESF_GZ_EV_FLSH_SUB_TYPE_LBN: c_int = 53;
pub const ESF_GZ_EV_FLSH_SUB_TYPE_WIDTH: c_int = 6;
pub const ESF_GZ_EV_FLSH_RSVD_LBN: c_int = 10;
pub const ESF_GZ_EV_FLSH_RSVD_WIDTH: c_int = 43;
pub const ESF_GZ_EV_FLSH_LABEL_LBN: c_int = 4;
pub const ESF_GZ_EV_FLSH_LABEL_WIDTH: c_int = 6;
pub const ESF_GZ_EV_FLSH_FLUSH_TYPE_LBN: c_int = 0;
pub const ESF_GZ_EV_FLSH_FLUSH_TYPE_WIDTH: c_int = 4;
pub const ESE_GZ_SF_FLUSH_EVNT_STRUCT_SIZE: c_int = 64;
// sf_rx_pkts
pub const ESF_GZ_EV_RXPKTS_E_TYPE_LBN: c_int = 60;
pub const ESF_GZ_EV_RXPKTS_E_TYPE_WIDTH: c_int = 4;
pub const ESF_GZ_EV_RXPKTS_PHASE_LBN: c_int = 59;
pub const ESF_GZ_EV_RXPKTS_PHASE_WIDTH: c_int = 1;
pub const ESF_GZ_EV_RXPKTS_RSVD_LBN: c_int = 22;
pub const ESF_GZ_EV_RXPKTS_RSVD_WIDTH: c_int = 37;
pub const ESF_GZ_EV_RXPKTS_Q_LABEL_LBN: c_int = 16;
pub const ESF_GZ_EV_RXPKTS_Q_LABEL_WIDTH: c_int = 6;
pub const ESF_GZ_EV_RXPKTS_NUM_PKT_LBN: c_int = 0;
pub const ESF_GZ_EV_RXPKTS_NUM_PKT_WIDTH: c_int = 16;
pub const ESE_GZ_SF_RX_PKTS_STRUCT_SIZE: c_int = 64;
// sf_rx_prefix
pub const ESF_GZ_RX_PREFIX_VLAN_STRIP_TCI_LBN: c_int = 160;
pub const ESF_GZ_RX_PREFIX_VLAN_STRIP_TCI_WIDTH: c_int = 16;
pub const ESF_GZ_RX_PREFIX_CSUM_FRAME_LBN: c_int = 144;
pub const ESF_GZ_RX_PREFIX_CSUM_FRAME_WIDTH: c_int = 16;
pub const ESF_GZ_RX_PREFIX_INGRESS_MPORT_LBN: c_int = 128;
pub const ESF_GZ_RX_PREFIX_INGRESS_MPORT_WIDTH: c_int = 16;
pub const ESF_GZ_RX_PREFIX_USER_MARK_LBN: c_int = 96;
pub const ESF_GZ_RX_PREFIX_USER_MARK_WIDTH: c_int = 32;
pub const ESF_GZ_RX_PREFIX_RSS_HASH_LBN: c_int = 64;
pub const ESF_GZ_RX_PREFIX_RSS_HASH_WIDTH: c_int = 32;
pub const ESF_GZ_RX_PREFIX_PARTIAL_TSTAMP_LBN: c_int = 34;
pub const ESF_GZ_RX_PREFIX_PARTIAL_TSTAMP_WIDTH: c_int = 30;
pub const ESF_GZ_RX_PREFIX_VSWITCH_STATUS_LBN: c_int = 33;
pub const ESF_GZ_RX_PREFIX_VSWITCH_STATUS_WIDTH: c_int = 1;
pub const ESF_GZ_RX_PREFIX_VLAN_STRIPPED_LBN: c_int = 32;
pub const ESF_GZ_RX_PREFIX_VLAN_STRIPPED_WIDTH: c_int = 1;
pub const ESF_GZ_RX_PREFIX_CLASS_LBN: c_int = 16;
pub const ESF_GZ_RX_PREFIX_CLASS_WIDTH: c_int = 16;
pub const ESF_GZ_RX_PREFIX_USER_FLAG_LBN: c_int = 15;
pub const ESF_GZ_RX_PREFIX_USER_FLAG_WIDTH: c_int = 1;
pub const ESF_GZ_RX_PREFIX_RSS_HASH_VALID_LBN: c_int = 14;
pub const ESF_GZ_RX_PREFIX_RSS_HASH_VALID_WIDTH: c_int = 1;
pub const ESF_GZ_RX_PREFIX_LENGTH_LBN: c_int = 0;
pub const ESF_GZ_RX_PREFIX_LENGTH_WIDTH: c_int = 14;
pub const ESE_GZ_SF_RX_PREFIX_STRUCT_SIZE: c_int = 176;
// sf_rxtx_generic
pub const ESF_GZ_EV_BARRIER_LBN: c_int = 167;
pub const ESF_GZ_EV_BARRIER_WIDTH: c_int = 1;
pub const ESF_GZ_EV_RSVD_LBN: c_int = 130;
pub const ESF_GZ_EV_RSVD_WIDTH: c_int = 37;
pub const ESF_GZ_EV_DPRXY_LBN: c_int = 129;
pub const ESF_GZ_EV_DPRXY_WIDTH: c_int = 1;
pub const ESF_GZ_EV_VIRTIO_LBN: c_int = 128;
pub const ESF_GZ_EV_VIRTIO_WIDTH: c_int = 1;
pub const ESF_GZ_EV_COUNT_LBN: c_int = 0;
pub const ESF_GZ_EV_COUNT_WIDTH: c_int = 128;
pub const ESE_GZ_SF_RXTX_GENERIC_STRUCT_SIZE: c_int = 168;
// sf_ts_stamp
pub const ESF_GZ_EV_TS_E_TYPE_LBN: c_int = 60;
pub const ESF_GZ_EV_TS_E_TYPE_WIDTH: c_int = 4;
pub const ESF_GZ_EV_TS_PHASE_LBN: c_int = 59;
pub const ESF_GZ_EV_TS_PHASE_WIDTH: c_int = 1;
pub const ESF_GZ_EV_TS_RSVD_LBN: c_int = 56;
pub const ESF_GZ_EV_TS_RSVD_WIDTH: c_int = 3;
pub const ESF_GZ_EV_TS_STATUS_LBN: c_int = 54;
pub const ESF_GZ_EV_TS_STATUS_WIDTH: c_int = 2;
pub const ESF_GZ_EV_TS_Q_LABEL_LBN: c_int = 48;
pub const ESF_GZ_EV_TS_Q_LABEL_WIDTH: c_int = 6;
pub const ESF_GZ_EV_TS_DESC_ID_LBN: c_int = 32;
pub const ESF_GZ_EV_TS_DESC_ID_WIDTH: c_int = 16;
pub const ESF_GZ_EV_TS_PARTIAL_STAMP_LBN: c_int = 0;
pub const ESF_GZ_EV_TS_PARTIAL_STAMP_WIDTH: c_int = 32;
pub const ESE_GZ_SF_TS_STAMP_STRUCT_SIZE: c_int = 64;
// sf_tx_cmplt
pub const ESF_GZ_EV_TXCMPL_E_TYPE_LBN: c_int = 60;
pub const ESF_GZ_EV_TXCMPL_E_TYPE_WIDTH: c_int = 4;
pub const ESF_GZ_EV_TXCMPL_PHASE_LBN: c_int = 59;
pub const ESF_GZ_EV_TXCMPL_PHASE_WIDTH: c_int = 1;
pub const ESF_GZ_EV_TXCMPL_RSVD_LBN: c_int = 22;
pub const ESF_GZ_EV_TXCMPL_RSVD_WIDTH: c_int = 37;
pub const ESF_GZ_EV_TXCMPL_Q_LABEL_LBN: c_int = 16;
pub const ESF_GZ_EV_TXCMPL_Q_LABEL_WIDTH: c_int = 6;
pub const ESF_GZ_EV_TXCMPL_NUM_DESC_LBN: c_int = 0;
pub const ESF_GZ_EV_TXCMPL_NUM_DESC_WIDTH: c_int = 16;
pub const ESE_GZ_SF_TX_CMPLT_STRUCT_SIZE: c_int = 64;
// sf_tx_desc2cmpt_dsc_fmt
pub const ESF_GZ_D2C_TGT_VI_ID_LBN: c_int = 108;
pub const ESF_GZ_D2C_TGT_VI_ID_WIDTH: c_int = 16;
pub const ESF_GZ_D2C_CMPT2_LBN: c_int = 107;
pub const ESF_GZ_D2C_CMPT2_WIDTH: c_int = 1;
pub const ESF_GZ_D2C_ABS_VI_ID_LBN: c_int = 106;
pub const ESF_GZ_D2C_ABS_VI_ID_WIDTH: c_int = 1;
pub const ESF_GZ_D2C_ORDERED_LBN: c_int = 105;
pub const ESF_GZ_D2C_ORDERED_WIDTH: c_int = 1;
pub const ESF_GZ_D2C_SKIP_N_LBN: c_int = 97;
pub const ESF_GZ_D2C_SKIP_N_WIDTH: c_int = 8;
pub const ESF_GZ_D2C_RSVD_LBN: c_int = 64;
pub const ESF_GZ_D2C_RSVD_WIDTH: c_int = 33;
pub const ESF_GZ_D2C_COMPLETION_LBN: c_int = 0;
pub const ESF_GZ_D2C_COMPLETION_WIDTH: c_int = 64;
pub const ESE_GZ_SF_TX_DESC2CMPT_DSC_FMT_STRUCT_SIZE: c_int = 124;
// sf_tx_mem2mem_dsc_fmt
pub const ESF_GZ_M2M_ADDR_SPC_EN_LBN: c_int = 123;
pub const ESF_GZ_M2M_ADDR_SPC_EN_WIDTH: c_int = 1;
pub const ESF_GZ_M2M_TRANSLATE_ADDR_LBN: c_int = 122;
pub const ESF_GZ_M2M_TRANSLATE_ADDR_WIDTH: c_int = 1;
pub const ESF_GZ_M2M_RSVD_LBN: c_int = 120;
pub const ESF_GZ_M2M_RSVD_WIDTH: c_int = 2;
pub const ESF_GZ_M2M_ADDR_SPC_ID_LBN: c_int = 84;
pub const ESF_GZ_M2M_ADDR_SPC_ID_WIDTH: c_int = 36;
pub const ESF_GZ_M2M_LEN_MINUS_1_LBN: c_int = 64;
pub const ESF_GZ_M2M_LEN_MINUS_1_WIDTH: c_int = 20;
pub const ESF_GZ_M2M_ADDR_LBN: c_int = 0;
pub const ESF_GZ_M2M_ADDR_WIDTH: c_int = 64;
pub const ESE_GZ_SF_TX_MEM2MEM_DSC_FMT_STRUCT_SIZE: c_int = 124;
// sf_tx_ovr_dsc_fmt
pub const ESF_GZ_TX_PREFIX_MARK_EN_LBN: c_int = 123;
pub const ESF_GZ_TX_PREFIX_MARK_EN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_PREFIX_INGRESS_MPORT_EN_LBN: c_int = 122;
pub const ESF_GZ_TX_PREFIX_INGRESS_MPORT_EN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_PREFIX_INLINE_CAPSULE_META_LBN: c_int = 121;
pub const ESF_GZ_TX_PREFIX_INLINE_CAPSULE_META_WIDTH: c_int = 1;
pub const ESF_GZ_TX_PREFIX_EGRESS_MPORT_EN_LBN: c_int = 120;
pub const ESF_GZ_TX_PREFIX_EGRESS_MPORT_EN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_PREFIX_RSRVD_LBN: c_int = 64;
pub const ESF_GZ_TX_PREFIX_RSRVD_WIDTH: c_int = 56;
pub const ESF_GZ_TX_PREFIX_EGRESS_MPORT_LBN: c_int = 48;
pub const ESF_GZ_TX_PREFIX_EGRESS_MPORT_WIDTH: c_int = 16;
pub const ESF_GZ_TX_PREFIX_INGRESS_MPORT_LBN: c_int = 32;
pub const ESF_GZ_TX_PREFIX_INGRESS_MPORT_WIDTH: c_int = 16;
pub const ESF_GZ_TX_PREFIX_MARK_LBN: c_int = 0;
pub const ESF_GZ_TX_PREFIX_MARK_WIDTH: c_int = 32;
pub const ESE_GZ_SF_TX_OVR_DSC_FMT_STRUCT_SIZE: c_int = 124;
// sf_tx_seg_dsc_fmt
pub const ESF_GZ_TX_SEG_ADDR_SPC_EN_LBN: c_int = 123;
pub const ESF_GZ_TX_SEG_ADDR_SPC_EN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_SEG_TRANSLATE_ADDR_LBN: c_int = 122;
pub const ESF_GZ_TX_SEG_TRANSLATE_ADDR_WIDTH: c_int = 1;
pub const ESF_GZ_TX_SEG_RSVD2_LBN: c_int = 120;
pub const ESF_GZ_TX_SEG_RSVD2_WIDTH: c_int = 2;
pub const ESF_GZ_TX_SEG_ADDR_SPC_ID_LBN: c_int = 84;
pub const ESF_GZ_TX_SEG_ADDR_SPC_ID_WIDTH: c_int = 36;
pub const ESF_GZ_TX_SEG_RSVD_LBN: c_int = 80;
pub const ESF_GZ_TX_SEG_RSVD_WIDTH: c_int = 4;
pub const ESF_GZ_TX_SEG_LEN_LBN: c_int = 64;
pub const ESF_GZ_TX_SEG_LEN_WIDTH: c_int = 16;
pub const ESF_GZ_TX_SEG_ADDR_LBN: c_int = 0;
pub const ESF_GZ_TX_SEG_ADDR_WIDTH: c_int = 64;
pub const ESE_GZ_SF_TX_SEG_DSC_FMT_STRUCT_SIZE: c_int = 124;
// sf_tx_std_dsc_fmt
pub const ESF_GZ_TX_SEND_VLAN_INSERT_TCI_LBN: c_int = 108;
pub const ESF_GZ_TX_SEND_VLAN_INSERT_TCI_WIDTH: c_int = 16;
pub const ESF_GZ_TX_SEND_VLAN_INSERT_EN_LBN: c_int = 107;
pub const ESF_GZ_TX_SEND_VLAN_INSERT_EN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_SEND_TSTAMP_REQ_LBN: c_int = 106;
pub const ESF_GZ_TX_SEND_TSTAMP_REQ_WIDTH: c_int = 1;
pub const ESF_GZ_TX_SEND_CSO_OUTER_L4_LBN: c_int = 105;
pub const ESF_GZ_TX_SEND_CSO_OUTER_L4_WIDTH: c_int = 1;
pub const ESF_GZ_TX_SEND_CSO_OUTER_L3_LBN: c_int = 104;
pub const ESF_GZ_TX_SEND_CSO_OUTER_L3_WIDTH: c_int = 1;
pub const ESF_GZ_TX_SEND_CSO_INNER_L3_LBN: c_int = 101;
pub const ESF_GZ_TX_SEND_CSO_INNER_L3_WIDTH: c_int = 3;
pub const ESF_GZ_TX_SEND_RSVD_LBN: c_int = 99;
pub const ESF_GZ_TX_SEND_RSVD_WIDTH: c_int = 2;
pub const ESF_GZ_TX_SEND_CSO_PARTIAL_EN_LBN: c_int = 97;
pub const ESF_GZ_TX_SEND_CSO_PARTIAL_EN_WIDTH: c_int = 2;
pub const ESF_GZ_TX_SEND_CSO_PARTIAL_CSUM_W_LBN: c_int = 92;
pub const ESF_GZ_TX_SEND_CSO_PARTIAL_CSUM_W_WIDTH: c_int = 5;
pub const ESF_GZ_TX_SEND_CSO_PARTIAL_START_W_LBN: c_int = 83;
pub const ESF_GZ_TX_SEND_CSO_PARTIAL_START_W_WIDTH: c_int = 9;
pub const ESF_GZ_TX_SEND_NUM_SEGS_LBN: c_int = 78;
pub const ESF_GZ_TX_SEND_NUM_SEGS_WIDTH: c_int = 5;
pub const ESF_GZ_TX_SEND_LEN_LBN: c_int = 64;
pub const ESF_GZ_TX_SEND_LEN_WIDTH: c_int = 14;
pub const ESF_GZ_TX_SEND_ADDR_LBN: c_int = 0;
pub const ESF_GZ_TX_SEND_ADDR_WIDTH: c_int = 64;
pub const ESE_GZ_SF_TX_STD_DSC_FMT_STRUCT_SIZE: c_int = 124;
// sf_tx_tso_dsc_fmt
pub const ESF_GZ_TX_TSO_VLAN_INSERT_TCI_LBN: c_int = 108;
pub const ESF_GZ_TX_TSO_VLAN_INSERT_TCI_WIDTH: c_int = 16;
pub const ESF_GZ_TX_TSO_VLAN_INSERT_EN_LBN: c_int = 107;
pub const ESF_GZ_TX_TSO_VLAN_INSERT_EN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_TSTAMP_REQ_LBN: c_int = 106;
pub const ESF_GZ_TX_TSO_TSTAMP_REQ_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_CSO_OUTER_L4_LBN: c_int = 105;
pub const ESF_GZ_TX_TSO_CSO_OUTER_L4_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_CSO_OUTER_L3_LBN: c_int = 104;
pub const ESF_GZ_TX_TSO_CSO_OUTER_L3_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_CSO_INNER_L3_LBN: c_int = 101;
pub const ESF_GZ_TX_TSO_CSO_INNER_L3_WIDTH: c_int = 3;
pub const ESF_GZ_TX_TSO_RSVD_LBN: c_int = 94;
pub const ESF_GZ_TX_TSO_RSVD_WIDTH: c_int = 7;
pub const ESF_GZ_TX_TSO_CSO_INNER_L4_LBN: c_int = 93;
pub const ESF_GZ_TX_TSO_CSO_INNER_L4_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_INNER_L4_OFF_W_LBN: c_int = 85;
pub const ESF_GZ_TX_TSO_INNER_L4_OFF_W_WIDTH: c_int = 8;
pub const ESF_GZ_TX_TSO_INNER_L3_OFF_W_LBN: c_int = 77;
pub const ESF_GZ_TX_TSO_INNER_L3_OFF_W_WIDTH: c_int = 8;
pub const ESF_GZ_TX_TSO_OUTER_L4_OFF_W_LBN: c_int = 69;
pub const ESF_GZ_TX_TSO_OUTER_L4_OFF_W_WIDTH: c_int = 8;
pub const ESF_GZ_TX_TSO_OUTER_L3_OFF_W_LBN: c_int = 64;
pub const ESF_GZ_TX_TSO_OUTER_L3_OFF_W_WIDTH: c_int = 5;
pub const ESF_GZ_TX_TSO_PAYLOAD_LEN_LBN: c_int = 42;
pub const ESF_GZ_TX_TSO_PAYLOAD_LEN_WIDTH: c_int = 22;
pub const ESF_GZ_TX_TSO_HDR_LEN_W_LBN: c_int = 34;
pub const ESF_GZ_TX_TSO_HDR_LEN_W_WIDTH: c_int = 8;
pub const ESF_GZ_TX_TSO_ED_OUTER_UDP_LEN_LBN: c_int = 33;
pub const ESF_GZ_TX_TSO_ED_OUTER_UDP_LEN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_ED_INNER_IP_LEN_LBN: c_int = 32;
pub const ESF_GZ_TX_TSO_ED_INNER_IP_LEN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_ED_OUTER_IP_LEN_LBN: c_int = 31;
pub const ESF_GZ_TX_TSO_ED_OUTER_IP_LEN_WIDTH: c_int = 1;
pub const ESF_GZ_TX_TSO_ED_INNER_IP4_ID_LBN: c_int = 29;
pub const ESF_GZ_TX_TSO_ED_INNER_IP4_ID_WIDTH: c_int = 2;
pub const ESF_GZ_TX_TSO_ED_OUTER_IP4_ID_LBN: c_int = 27;
pub const ESF_GZ_TX_TSO_ED_OUTER_IP4_ID_WIDTH: c_int = 2;
pub const ESF_GZ_TX_TSO_PAYLOAD_NUM_SEGS_LBN: c_int = 17;
pub const ESF_GZ_TX_TSO_PAYLOAD_NUM_SEGS_WIDTH: c_int = 10;
pub const ESF_GZ_TX_TSO_HDR_NUM_SEGS_LBN: c_int = 14;
pub const ESF_GZ_TX_TSO_HDR_NUM_SEGS_WIDTH: c_int = 3;
pub const ESF_GZ_TX_TSO_MSS_LBN: c_int = 0;
pub const ESF_GZ_TX_TSO_MSS_WIDTH: c_int = 14;
pub const ESE_GZ_SF_TX_TSO_DSC_FMT_STRUCT_SIZE: c_int = 124;
// Enum D2VIO_MSG_OP
pub const ESE_GZ_QUE_JBDNE: c_int = 3;
pub const ESE_GZ_QUE_EVICT: c_int = 2;
pub const ESE_GZ_QUE_EMPTY: c_int = 1;
pub const ESE_GZ_NOP: c_int = 0;
// Enum DESIGN_PARAMS
pub const ESE_EF100_DP_GZ_RX_MAX_RUNT: c_int = 17;
pub const ESE_EF100_DP_GZ_VI_STRIDES: c_int = 16;
pub const ESE_EF100_DP_GZ_NMMU_PAGE_SIZES: c_int = 15;
pub const ESE_EF100_DP_GZ_EVQ_TIMER_TICK_NANOS: c_int = 14;
pub const ESE_EF100_DP_GZ_MEM2MEM_MAX_LEN: c_int = 13;
pub const ESE_EF100_DP_GZ_COMPAT: c_int = 12;
pub const ESE_EF100_DP_GZ_TSO_MAX_NUM_FRAMES: c_int = 11;
pub const ESE_EF100_DP_GZ_TSO_MAX_PAYLOAD_NUM_SEGS: c_int = 10;
pub const ESE_EF100_DP_GZ_TSO_MAX_PAYLOAD_LEN: c_int = 9;
pub const ESE_EF100_DP_GZ_TXQ_SIZE_GRANULARITY: c_int = 8;
pub const ESE_EF100_DP_GZ_RXQ_SIZE_GRANULARITY: c_int = 7;
pub const ESE_EF100_DP_GZ_TSO_MAX_HDR_NUM_SEGS: c_int = 6;
pub const ESE_EF100_DP_GZ_TSO_MAX_HDR_LEN: c_int = 5;
pub const ESE_EF100_DP_GZ_RX_L4_CSUM_PROTOCOLS: c_int = 4;
pub const ESE_EF100_DP_GZ_NMMU_GROUP_SIZE: c_int = 3;
pub const ESE_EF100_DP_GZ_EVQ_UNSOL_CREDIT_SEQ_BITS: c_int = 2;
pub const ESE_EF100_DP_GZ_PARTIAL_TSTAMP_SUB_NANO_BITS: c_int = 1;
pub const ESE_EF100_DP_GZ_PAD: c_int = 0;
// Enum DESIGN_PARAM_DEFAULTS
pub const ESE_EF100_DP_GZ_TSO_MAX_PAYLOAD_LEN_DEFAULT: c_uint = 0x3fffff;
pub const ESE_EF100_DP_GZ_TSO_MAX_NUM_FRAMES_DEFAULT: c_int = 8192;
pub const ESE_EF100_DP_GZ_MEM2MEM_MAX_LEN_DEFAULT: c_int = 8192;
pub const ESE_EF100_DP_GZ_RX_L4_CSUM_PROTOCOLS_DEFAULT: c_uint = 0x1106;
pub const ESE_EF100_DP_GZ_TSO_MAX_PAYLOAD_NUM_SEGS_DEFAULT: c_uint = 0x3ff;
pub const ESE_EF100_DP_GZ_RX_MAX_RUNT_DEFAULT: c_int = 640;
pub const ESE_EF100_DP_GZ_EVQ_TIMER_TICK_NANOS_DEFAULT: c_int = 512;
pub const ESE_EF100_DP_GZ_NMMU_PAGE_SIZES_DEFAULT: c_int = 512;
pub const ESE_EF100_DP_GZ_TSO_MAX_HDR_LEN_DEFAULT: c_int = 192;
pub const ESE_EF100_DP_GZ_RXQ_SIZE_GRANULARITY_DEFAULT: c_int = 64;
pub const ESE_EF100_DP_GZ_TXQ_SIZE_GRANULARITY_DEFAULT: c_int = 64;
pub const ESE_EF100_DP_GZ_NMMU_GROUP_SIZE_DEFAULT: c_int = 32;
pub const ESE_EF100_DP_GZ_VI_STRIDES_DEFAULT: c_int = 16;
pub const ESE_EF100_DP_GZ_EVQ_UNSOL_CREDIT_SEQ_BITS_DEFAULT: c_int = 7;
pub const ESE_EF100_DP_GZ_TSO_MAX_HDR_NUM_SEGS_DEFAULT: c_int = 4;
pub const ESE_EF100_DP_GZ_PARTIAL_TSTAMP_SUB_NANO_BITS_DEFAULT: c_int = 2;
pub const ESE_EF100_DP_GZ_COMPAT_DEFAULT: c_int = 0;
// Enum HOST_IF_CONSTANTS
pub const ESE_GZ_FCW_LEN: c_uint = 0x4C;
pub const ESE_GZ_RX_PKT_PREFIX_LEN: c_int = 22;
// Enum PCI_CONSTANTS
pub const ESE_GZ_PCI_BASE_CONFIG_SPACE_SIZE: c_int = 256;
pub const ESE_GZ_PCI_EXPRESS_XCAP_HDR_SIZE: c_int = 4;
// Enum RH_DSC_TYPE
pub const ESE_GZ_TX_TOMB: c_uint = 0xF;
pub const ESE_GZ_TX_VIO: c_uint = 0xE;
pub const ESE_GZ_TX_TSO_OVRRD: c_uint = 0x8;
pub const ESE_GZ_TX_D2CMP: c_uint = 0x7;
pub const ESE_GZ_TX_DATA: c_uint = 0x6;
pub const ESE_GZ_TX_D2M: c_uint = 0x5;
pub const ESE_GZ_TX_M2M: c_uint = 0x4;
pub const ESE_GZ_TX_SEG: c_uint = 0x3;
pub const ESE_GZ_TX_TSO: c_uint = 0x2;
pub const ESE_GZ_TX_OVRRD: c_uint = 0x1;
pub const ESE_GZ_TX_SEND: c_uint = 0x0;
// Enum RH_HCLASS_L2_CLASS
pub const ESE_GZ_RH_HCLASS_L2_CLASS_E2_0123VLAN: c_int = 1;
pub const ESE_GZ_RH_HCLASS_L2_CLASS_OTHER: c_int = 0;
// Enum RH_HCLASS_L2_STATUS
pub const ESE_GZ_RH_HCLASS_L2_STATUS_RESERVED: c_int = 3;
pub const ESE_GZ_RH_HCLASS_L2_STATUS_FCS_ERR: c_int = 2;
pub const ESE_GZ_RH_HCLASS_L2_STATUS_LEN_ERR: c_int = 1;
pub const ESE_GZ_RH_HCLASS_L2_STATUS_OK: c_int = 0;
// Enum RH_HCLASS_L3_CLASS
pub const ESE_GZ_RH_HCLASS_L3_CLASS_OTHER: c_int = 3;
pub const ESE_GZ_RH_HCLASS_L3_CLASS_IP6: c_int = 2;
pub const ESE_GZ_RH_HCLASS_L3_CLASS_IP4BAD: c_int = 1;
pub const ESE_GZ_RH_HCLASS_L3_CLASS_IP4GOOD: c_int = 0;
// Enum RH_HCLASS_L4_CLASS
pub const ESE_GZ_RH_HCLASS_L4_CLASS_OTHER: c_int = 3;
pub const ESE_GZ_RH_HCLASS_L4_CLASS_FRAG: c_int = 2;
pub const ESE_GZ_RH_HCLASS_L4_CLASS_UDP: c_int = 1;
pub const ESE_GZ_RH_HCLASS_L4_CLASS_TCP: c_int = 0;
// Enum RH_HCLASS_L4_CSUM
pub const ESE_GZ_RH_HCLASS_L4_CSUM_GOOD: c_int = 1;
pub const ESE_GZ_RH_HCLASS_L4_CSUM_BAD_OR_UNKNOWN: c_int = 0;
// Enum RH_HCLASS_TUNNEL_CLASS
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_RESERVED_7: c_int = 7;
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_RESERVED_6: c_int = 6;
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_RESERVED_5: c_int = 5;
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_RESERVED_4: c_int = 4;
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_GENEVE: c_int = 3;
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_NVGRE: c_int = 2;
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_VXLAN: c_int = 1;
pub const ESE_GZ_RH_HCLASS_TUNNEL_CLASS_NONE: c_int = 0;
// Enum SF_CTL_EVENT_SUBTYPE
pub const ESE_GZ_EF100_CTL_EV_EVQ_TIMEOUT: c_uint = 0x3;
pub const ESE_GZ_EF100_CTL_EV_FLUSH: c_uint = 0x2;
pub const ESE_GZ_EF100_CTL_EV_TIME_SYNC: c_uint = 0x1;
pub const ESE_GZ_EF100_CTL_EV_UNSOL_OVERFLOW: c_uint = 0x0;
// Enum SF_EVENT_TYPE
pub const ESE_GZ_EF100_EV_DRIVER: c_uint = 0x5;
pub const ESE_GZ_EF100_EV_MCDI: c_uint = 0x4;
pub const ESE_GZ_EF100_EV_CONTROL: c_uint = 0x3;
pub const ESE_GZ_EF100_EV_TX_TIMESTAMP: c_uint = 0x2;
pub const ESE_GZ_EF100_EV_TX_COMPLETION: c_uint = 0x1;
pub const ESE_GZ_EF100_EV_RX_PKTS: c_uint = 0x0;
// Enum SF_EW_EVENT_TYPE
pub const ESE_GZ_EF100_EWEV_VIRTQ_DESC: c_uint = 0x2;
pub const ESE_GZ_EF100_EWEV_TXQ_DESC: c_uint = 0x1;
pub const ESE_GZ_EF100_EWEV_64BIT: c_uint = 0x0;
// Enum TX_DESC_CSO_PARTIAL_EN
pub const ESE_GZ_TX_DESC_CSO_PARTIAL_EN_TCP: c_int = 2;
pub const ESE_GZ_TX_DESC_CSO_PARTIAL_EN_UDP: c_int = 1;
pub const ESE_GZ_TX_DESC_CSO_PARTIAL_EN_OFF: c_int = 0;
// Enum TX_DESC_CS_INNER_L3
pub const ESE_GZ_TX_DESC_CS_INNER_L3_GENEVE: c_int = 3;
pub const ESE_GZ_TX_DESC_CS_INNER_L3_NVGRE: c_int = 2;
pub const ESE_GZ_TX_DESC_CS_INNER_L3_VXLAN: c_int = 1;
pub const ESE_GZ_TX_DESC_CS_INNER_L3_OFF: c_int = 0;
// Enum TX_DESC_IP4_ID
pub const ESE_GZ_TX_DESC_IP4_ID_INC_MOD16: c_int = 2;
pub const ESE_GZ_TX_DESC_IP4_ID_INC_MOD15: c_int = 1;
pub const ESE_GZ_TX_DESC_IP4_ID_NO_OP: c_int = 0;
// Enum VIRTIO_NET_HDR_F
pub const ESE_GZ_NEEDS_CSUM: c_uint = 0x1;
// Enum VIRTIO_NET_HDR_GSO
pub const ESE_GZ_TCPV6: c_uint = 0x4;
pub const ESE_GZ_UDP: c_uint = 0x3;
pub const ESE_GZ_TCPV4: c_uint = 0x1;
pub const ESE_GZ_NONE: c_uint = 0x0;
//
pub const ESF_GZ_EV_DEBUG_EVENT_GEN_FLAGS_LBN: c_int = 44;
pub const ESF_GZ_EV_DEBUG_EVENT_GEN_FLAGS_WIDTH: c_int = 4;
pub const ESF_GZ_EV_DEBUG_SRC_QID_LBN: c_int = 32;
pub const ESF_GZ_EV_DEBUG_SRC_QID_WIDTH: c_int = 12;
pub const ESF_GZ_EV_DEBUG_SEQ_NUM_LBN: c_int = 16;
pub const ESF_GZ_EV_DEBUG_SEQ_NUM_WIDTH: c_int = 16;
