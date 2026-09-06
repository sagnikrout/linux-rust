//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/sfc/ef10_regs.h
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
// Copyright 2012-2017 Solarflare Communications Inc.
//
// EF10 hardware architecture definitions have a name prefix following
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
// D: Huntington A0
//
// If the definition has been changed or removed in later revisions
// then <max-rev> is the last revision to which the definition applies;
// otherwise it is "Z".
//
// EF10 registers and descriptors
//
// BIU_HW_REV_ID_REG:
pub const ER_DZ_BIU_HW_REV_ID: c_uint = 0x00000000;
pub const ERF_DZ_HW_REV_ID_LBN: c_int = 0;
pub const ERF_DZ_HW_REV_ID_WIDTH: c_int = 32;
// BIU_MC_SFT_STATUS_REG:
pub const ER_DZ_BIU_MC_SFT_STATUS: c_uint = 0x00000010;
pub const ER_DZ_BIU_MC_SFT_STATUS_STEP: c_int = 4;
pub const ER_DZ_BIU_MC_SFT_STATUS_ROWS: c_int = 8;
pub const ERF_DZ_MC_SFT_STATUS_LBN: c_int = 0;
pub const ERF_DZ_MC_SFT_STATUS_WIDTH: c_int = 32;
// BIU_INT_ISR_REG:
pub const ER_DZ_BIU_INT_ISR: c_uint = 0x00000090;
pub const ERF_DZ_ISR_REG_LBN: c_int = 0;
pub const ERF_DZ_ISR_REG_WIDTH: c_int = 32;
// MC_DB_LWRD_REG:
pub const ER_DZ_MC_DB_LWRD: c_uint = 0x00000200;
pub const ERF_DZ_MC_DOORBELL_L_LBN: c_int = 0;
pub const ERF_DZ_MC_DOORBELL_L_WIDTH: c_int = 32;
// MC_DB_HWRD_REG:
pub const ER_DZ_MC_DB_HWRD: c_uint = 0x00000204;
pub const ERF_DZ_MC_DOORBELL_H_LBN: c_int = 0;
pub const ERF_DZ_MC_DOORBELL_H_WIDTH: c_int = 32;
// EVQ_RPTR_REG:
pub const ER_DZ_EVQ_RPTR: c_uint = 0x00000400;
pub const ER_DZ_EVQ_RPTR_STEP: c_int = 8192;
pub const ER_DZ_EVQ_RPTR_ROWS: c_int = 2048;
pub const ERF_DZ_EVQ_RPTR_VLD_LBN: c_int = 15;
pub const ERF_DZ_EVQ_RPTR_VLD_WIDTH: c_int = 1;
pub const ERF_DZ_EVQ_RPTR_LBN: c_int = 0;
pub const ERF_DZ_EVQ_RPTR_WIDTH: c_int = 15;
// EVQ_TMR_REG:
pub const ER_DZ_EVQ_TMR: c_uint = 0x00000420;
pub const ER_DZ_EVQ_TMR_STEP: c_int = 8192;
pub const ER_DZ_EVQ_TMR_ROWS: c_int = 2048;
pub const ERF_FZ_TC_TMR_REL_VAL_LBN: c_int = 16;
pub const ERF_FZ_TC_TMR_REL_VAL_WIDTH: c_int = 14;
pub const ERF_DZ_TC_TIMER_MODE_LBN: c_int = 14;
pub const ERF_DZ_TC_TIMER_MODE_WIDTH: c_int = 2;
pub const ERF_DZ_TC_TIMER_VAL_LBN: c_int = 0;
pub const ERF_DZ_TC_TIMER_VAL_WIDTH: c_int = 14;
// RX_DESC_UPD_REG:
pub const ER_DZ_RX_DESC_UPD: c_uint = 0x00000830;
pub const ER_DZ_RX_DESC_UPD_STEP: c_int = 8192;
pub const ER_DZ_RX_DESC_UPD_ROWS: c_int = 2048;
pub const ERF_DZ_RX_DESC_WPTR_LBN: c_int = 0;
pub const ERF_DZ_RX_DESC_WPTR_WIDTH: c_int = 12;
// TX_DESC_UPD_REG:
pub const ER_DZ_TX_DESC_UPD: c_uint = 0x00000a10;
pub const ER_DZ_TX_DESC_UPD_STEP: c_int = 8192;
pub const ER_DZ_TX_DESC_UPD_ROWS: c_int = 2048;
pub const ERF_DZ_RSVD_LBN: c_int = 76;
pub const ERF_DZ_RSVD_WIDTH: c_int = 20;
pub const ERF_DZ_TX_DESC_WPTR_LBN: c_int = 64;
pub const ERF_DZ_TX_DESC_WPTR_WIDTH: c_int = 12;
pub const ERF_DZ_TX_DESC_HWORD_LBN: c_int = 32;
pub const ERF_DZ_TX_DESC_HWORD_WIDTH: c_int = 32;
pub const ERF_DZ_TX_DESC_LWORD_LBN: c_int = 0;
pub const ERF_DZ_TX_DESC_LWORD_WIDTH: c_int = 32;
// DRIVER_EV
pub const ESF_DZ_DRV_CODE_LBN: c_int = 60;
pub const ESF_DZ_DRV_CODE_WIDTH: c_int = 4;
pub const ESF_DZ_DRV_SUB_CODE_LBN: c_int = 56;
pub const ESF_DZ_DRV_SUB_CODE_WIDTH: c_int = 4;
pub const ESE_DZ_DRV_TIMER_EV: c_int = 3;
pub const ESE_DZ_DRV_START_UP_EV: c_int = 2;
pub const ESE_DZ_DRV_WAKE_UP_EV: c_int = 1;
pub const ESF_DZ_DRV_SUB_DATA_LBN: c_int = 0;
pub const ESF_DZ_DRV_SUB_DATA_WIDTH: c_int = 56;
pub const ESF_DZ_DRV_EVQ_ID_LBN: c_int = 0;
pub const ESF_DZ_DRV_EVQ_ID_WIDTH: c_int = 14;
pub const ESF_DZ_DRV_TMR_ID_LBN: c_int = 0;
pub const ESF_DZ_DRV_TMR_ID_WIDTH: c_int = 14;
// EVENT_ENTRY
pub const ESF_DZ_EV_CODE_LBN: c_int = 60;
pub const ESF_DZ_EV_CODE_WIDTH: c_int = 4;
pub const ESE_DZ_EV_CODE_MCDI_EV: c_int = 12;
pub const ESE_DZ_EV_CODE_DRIVER_EV: c_int = 5;
pub const ESE_DZ_EV_CODE_TX_EV: c_int = 2;
pub const ESE_DZ_EV_CODE_RX_EV: c_int = 0;

pub const ESF_DZ_EV_DATA_LBN: c_int = 0;
pub const ESF_DZ_EV_DATA_WIDTH: c_int = 60;
// MC_EVENT
pub const ESF_DZ_MC_CODE_LBN: c_int = 60;
pub const ESF_DZ_MC_CODE_WIDTH: c_int = 4;
pub const ESF_DZ_MC_OVERRIDE_HOLDOFF_LBN: c_int = 59;
pub const ESF_DZ_MC_OVERRIDE_HOLDOFF_WIDTH: c_int = 1;
pub const ESF_DZ_MC_DROP_EVENT_LBN: c_int = 58;
pub const ESF_DZ_MC_DROP_EVENT_WIDTH: c_int = 1;
pub const ESF_DZ_MC_SOFT_LBN: c_int = 0;
pub const ESF_DZ_MC_SOFT_WIDTH: c_int = 58;
// RX_EVENT
pub const ESF_DZ_RX_CODE_LBN: c_int = 60;
pub const ESF_DZ_RX_CODE_WIDTH: c_int = 4;
pub const ESF_DZ_RX_OVERRIDE_HOLDOFF_LBN: c_int = 59;
pub const ESF_DZ_RX_OVERRIDE_HOLDOFF_WIDTH: c_int = 1;
pub const ESF_DZ_RX_DROP_EVENT_LBN: c_int = 58;
pub const ESF_DZ_RX_DROP_EVENT_WIDTH: c_int = 1;
pub const ESF_DD_RX_EV_RSVD2_LBN: c_int = 54;
pub const ESF_DD_RX_EV_RSVD2_WIDTH: c_int = 4;
pub const ESF_EZ_RX_TCP_UDP_INNER_CHKSUM_ERR_LBN: c_int = 57;
pub const ESF_EZ_RX_TCP_UDP_INNER_CHKSUM_ERR_WIDTH: c_int = 1;
pub const ESF_EZ_RX_IP_INNER_CHKSUM_ERR_LBN: c_int = 56;
pub const ESF_EZ_RX_IP_INNER_CHKSUM_ERR_WIDTH: c_int = 1;
pub const ESF_EZ_RX_EV_RSVD2_LBN: c_int = 54;
pub const ESF_EZ_RX_EV_RSVD2_WIDTH: c_int = 2;
pub const ESF_DZ_RX_EV_SOFT2_LBN: c_int = 52;
pub const ESF_DZ_RX_EV_SOFT2_WIDTH: c_int = 2;
pub const ESF_DZ_RX_DSC_PTR_LBITS_LBN: c_int = 48;
pub const ESF_DZ_RX_DSC_PTR_LBITS_WIDTH: c_int = 4;
pub const ESF_DE_RX_L4_CLASS_LBN: c_int = 45;
pub const ESF_DE_RX_L4_CLASS_WIDTH: c_int = 3;
pub const ESE_DE_L4_CLASS_RSVD7: c_int = 7;
pub const ESE_DE_L4_CLASS_RSVD6: c_int = 6;
pub const ESE_DE_L4_CLASS_RSVD5: c_int = 5;
pub const ESE_DE_L4_CLASS_RSVD4: c_int = 4;
pub const ESE_DE_L4_CLASS_RSVD3: c_int = 3;
pub const ESE_DE_L4_CLASS_UDP: c_int = 2;
pub const ESE_DE_L4_CLASS_TCP: c_int = 1;
pub const ESE_DE_L4_CLASS_UNKNOWN: c_int = 0;
pub const ESF_FZ_RX_FASTPD_INDCTR_LBN: c_int = 47;
pub const ESF_FZ_RX_FASTPD_INDCTR_WIDTH: c_int = 1;
pub const ESF_FZ_RX_L4_CLASS_LBN: c_int = 45;
pub const ESF_FZ_RX_L4_CLASS_WIDTH: c_int = 2;
pub const ESE_FZ_L4_CLASS_RSVD3: c_int = 3;
pub const ESE_FZ_L4_CLASS_UDP: c_int = 2;
pub const ESE_FZ_L4_CLASS_TCP: c_int = 1;
pub const ESE_FZ_L4_CLASS_UNKNOWN: c_int = 0;
pub const ESF_DZ_RX_L3_CLASS_LBN: c_int = 42;
pub const ESF_DZ_RX_L3_CLASS_WIDTH: c_int = 3;
pub const ESE_DZ_L3_CLASS_RSVD7: c_int = 7;
pub const ESE_DZ_L3_CLASS_IP6_FRAG: c_int = 6;
pub const ESE_DZ_L3_CLASS_ARP: c_int = 5;
pub const ESE_DZ_L3_CLASS_IP4_FRAG: c_int = 4;
pub const ESE_DZ_L3_CLASS_FCOE: c_int = 3;
pub const ESE_DZ_L3_CLASS_IP6: c_int = 2;
pub const ESE_DZ_L3_CLASS_IP4: c_int = 1;
pub const ESE_DZ_L3_CLASS_UNKNOWN: c_int = 0;
pub const ESF_DZ_RX_ETH_TAG_CLASS_LBN: c_int = 39;
pub const ESF_DZ_RX_ETH_TAG_CLASS_WIDTH: c_int = 3;
pub const ESE_DZ_ETH_TAG_CLASS_RSVD7: c_int = 7;
pub const ESE_DZ_ETH_TAG_CLASS_RSVD6: c_int = 6;
pub const ESE_DZ_ETH_TAG_CLASS_RSVD5: c_int = 5;
pub const ESE_DZ_ETH_TAG_CLASS_RSVD4: c_int = 4;
pub const ESE_DZ_ETH_TAG_CLASS_RSVD3: c_int = 3;
pub const ESE_DZ_ETH_TAG_CLASS_VLAN2: c_int = 2;
pub const ESE_DZ_ETH_TAG_CLASS_VLAN1: c_int = 1;
pub const ESE_DZ_ETH_TAG_CLASS_NONE: c_int = 0;
pub const ESF_DZ_RX_ETH_BASE_CLASS_LBN: c_int = 36;
pub const ESF_DZ_RX_ETH_BASE_CLASS_WIDTH: c_int = 3;
pub const ESE_DZ_ETH_BASE_CLASS_LLC_SNAP: c_int = 2;
pub const ESE_DZ_ETH_BASE_CLASS_LLC: c_int = 1;
pub const ESE_DZ_ETH_BASE_CLASS_ETH2: c_int = 0;
pub const ESF_DZ_RX_MAC_CLASS_LBN: c_int = 35;
pub const ESF_DZ_RX_MAC_CLASS_WIDTH: c_int = 1;
pub const ESE_DZ_MAC_CLASS_MCAST: c_int = 1;
pub const ESE_DZ_MAC_CLASS_UCAST: c_int = 0;
pub const ESF_DD_RX_EV_SOFT1_LBN: c_int = 32;
pub const ESF_DD_RX_EV_SOFT1_WIDTH: c_int = 3;
pub const ESF_EZ_RX_EV_SOFT1_LBN: c_int = 34;
pub const ESF_EZ_RX_EV_SOFT1_WIDTH: c_int = 1;
pub const ESF_EZ_RX_ENCAP_HDR_LBN: c_int = 32;
pub const ESF_EZ_RX_ENCAP_HDR_WIDTH: c_int = 2;
pub const ESE_EZ_ENCAP_HDR_GRE: c_int = 2;
pub const ESE_EZ_ENCAP_HDR_VXLAN: c_int = 1;
pub const ESE_EZ_ENCAP_HDR_NONE: c_int = 0;
pub const ESF_DD_RX_EV_RSVD1_LBN: c_int = 30;
pub const ESF_DD_RX_EV_RSVD1_WIDTH: c_int = 2;
pub const ESF_EZ_RX_EV_RSVD1_LBN: c_int = 31;
pub const ESF_EZ_RX_EV_RSVD1_WIDTH: c_int = 1;
pub const ESF_EZ_RX_ABORT_LBN: c_int = 30;
pub const ESF_EZ_RX_ABORT_WIDTH: c_int = 1;
pub const ESF_DZ_RX_ECC_ERR_LBN: c_int = 29;
pub const ESF_DZ_RX_ECC_ERR_WIDTH: c_int = 1;
pub const ESF_DZ_RX_TRUNC_ERR_LBN: c_int = 29;
pub const ESF_DZ_RX_TRUNC_ERR_WIDTH: c_int = 1;
pub const ESF_DZ_RX_CRC1_ERR_LBN: c_int = 28;
pub const ESF_DZ_RX_CRC1_ERR_WIDTH: c_int = 1;
pub const ESF_DZ_RX_CRC0_ERR_LBN: c_int = 27;
pub const ESF_DZ_RX_CRC0_ERR_WIDTH: c_int = 1;
pub const ESF_DZ_RX_TCPUDP_CKSUM_ERR_LBN: c_int = 26;
pub const ESF_DZ_RX_TCPUDP_CKSUM_ERR_WIDTH: c_int = 1;
pub const ESF_DZ_RX_IPCKSUM_ERR_LBN: c_int = 25;
pub const ESF_DZ_RX_IPCKSUM_ERR_WIDTH: c_int = 1;
pub const ESF_DZ_RX_ECRC_ERR_LBN: c_int = 24;
pub const ESF_DZ_RX_ECRC_ERR_WIDTH: c_int = 1;
pub const ESF_DZ_RX_QLABEL_LBN: c_int = 16;
pub const ESF_DZ_RX_QLABEL_WIDTH: c_int = 5;
pub const ESF_DZ_RX_PARSE_INCOMPLETE_LBN: c_int = 15;
pub const ESF_DZ_RX_PARSE_INCOMPLETE_WIDTH: c_int = 1;
pub const ESF_DZ_RX_CONT_LBN: c_int = 14;
pub const ESF_DZ_RX_CONT_WIDTH: c_int = 1;
pub const ESF_DZ_RX_BYTES_LBN: c_int = 0;
pub const ESF_DZ_RX_BYTES_WIDTH: c_int = 14;
// RX_KER_DESC
pub const ESF_DZ_RX_KER_RESERVED_LBN: c_int = 62;
pub const ESF_DZ_RX_KER_RESERVED_WIDTH: c_int = 2;
pub const ESF_DZ_RX_KER_BYTE_CNT_LBN: c_int = 48;
pub const ESF_DZ_RX_KER_BYTE_CNT_WIDTH: c_int = 14;
pub const ESF_DZ_RX_KER_BUF_ADDR_LBN: c_int = 0;
pub const ESF_DZ_RX_KER_BUF_ADDR_WIDTH: c_int = 48;
// TX_CSUM_TSTAMP_DESC
pub const ESF_DZ_TX_DESC_IS_OPT_LBN: c_int = 63;
pub const ESF_DZ_TX_DESC_IS_OPT_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_TYPE_LBN: c_int = 60;
pub const ESF_DZ_TX_OPTION_TYPE_WIDTH: c_int = 3;
pub const ESE_DZ_TX_OPTION_DESC_TSO: c_int = 7;
pub const ESE_DZ_TX_OPTION_DESC_VLAN: c_int = 6;
pub const ESE_DZ_TX_OPTION_DESC_CRC_CSUM: c_int = 0;
pub const ESF_DZ_TX_OPTION_TS_AT_TXDP_LBN: c_int = 8;
pub const ESF_DZ_TX_OPTION_TS_AT_TXDP_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_INNER_UDP_TCP_CSUM_LBN: c_int = 7;
pub const ESF_DZ_TX_OPTION_INNER_UDP_TCP_CSUM_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_INNER_IP_CSUM_LBN: c_int = 6;
pub const ESF_DZ_TX_OPTION_INNER_IP_CSUM_WIDTH: c_int = 1;
pub const ESF_DZ_TX_TIMESTAMP_LBN: c_int = 5;
pub const ESF_DZ_TX_TIMESTAMP_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_CRC_MODE_LBN: c_int = 2;
pub const ESF_DZ_TX_OPTION_CRC_MODE_WIDTH: c_int = 3;
pub const ESE_DZ_TX_OPTION_CRC_FCOIP_MPA: c_int = 5;
pub const ESE_DZ_TX_OPTION_CRC_FCOIP_FCOE: c_int = 4;
pub const ESE_DZ_TX_OPTION_CRC_ISCSI_HDR_AND_PYLD: c_int = 3;
pub const ESE_DZ_TX_OPTION_CRC_ISCSI_HDR: c_int = 2;
pub const ESE_DZ_TX_OPTION_CRC_FCOE: c_int = 1;
pub const ESE_DZ_TX_OPTION_CRC_OFF: c_int = 0;
pub const ESF_DZ_TX_OPTION_UDP_TCP_CSUM_LBN: c_int = 1;
pub const ESF_DZ_TX_OPTION_UDP_TCP_CSUM_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_IP_CSUM_LBN: c_int = 0;
pub const ESF_DZ_TX_OPTION_IP_CSUM_WIDTH: c_int = 1;
// TX_EVENT
pub const ESF_DZ_TX_CODE_LBN: c_int = 60;
pub const ESF_DZ_TX_CODE_WIDTH: c_int = 4;
pub const ESF_DZ_TX_OVERRIDE_HOLDOFF_LBN: c_int = 59;
pub const ESF_DZ_TX_OVERRIDE_HOLDOFF_WIDTH: c_int = 1;
pub const ESF_DZ_TX_DROP_EVENT_LBN: c_int = 58;
pub const ESF_DZ_TX_DROP_EVENT_WIDTH: c_int = 1;
pub const ESF_DD_TX_EV_RSVD_LBN: c_int = 48;
pub const ESF_DD_TX_EV_RSVD_WIDTH: c_int = 10;
pub const ESF_EZ_TCP_UDP_INNER_CHKSUM_ERR_LBN: c_int = 57;
pub const ESF_EZ_TCP_UDP_INNER_CHKSUM_ERR_WIDTH: c_int = 1;
pub const ESF_EZ_IP_INNER_CHKSUM_ERR_LBN: c_int = 56;
pub const ESF_EZ_IP_INNER_CHKSUM_ERR_WIDTH: c_int = 1;
pub const ESF_EZ_TX_EV_RSVD_LBN: c_int = 48;
pub const ESF_EZ_TX_EV_RSVD_WIDTH: c_int = 8;
pub const ESF_DZ_TX_SOFT2_LBN: c_int = 32;
pub const ESF_DZ_TX_SOFT2_WIDTH: c_int = 16;
pub const ESF_DD_TX_SOFT1_LBN: c_int = 24;
pub const ESF_DD_TX_SOFT1_WIDTH: c_int = 8;
pub const ESF_EZ_TX_CAN_MERGE_LBN: c_int = 31;
pub const ESF_EZ_TX_CAN_MERGE_WIDTH: c_int = 1;
pub const ESF_EZ_TX_SOFT1_LBN: c_int = 24;
pub const ESF_EZ_TX_SOFT1_WIDTH: c_int = 7;
pub const ESF_DZ_TX_QLABEL_LBN: c_int = 16;
pub const ESF_DZ_TX_QLABEL_WIDTH: c_int = 5;
pub const ESF_DZ_TX_DESCR_INDX_LBN: c_int = 0;
pub const ESF_DZ_TX_DESCR_INDX_WIDTH: c_int = 16;
// TX_KER_DESC
pub const ESF_DZ_TX_KER_TYPE_LBN: c_int = 63;
pub const ESF_DZ_TX_KER_TYPE_WIDTH: c_int = 1;
pub const ESF_DZ_TX_KER_CONT_LBN: c_int = 62;
pub const ESF_DZ_TX_KER_CONT_WIDTH: c_int = 1;
pub const ESF_DZ_TX_KER_BYTE_CNT_LBN: c_int = 48;
pub const ESF_DZ_TX_KER_BYTE_CNT_WIDTH: c_int = 14;
pub const ESF_DZ_TX_KER_BUF_ADDR_LBN: c_int = 0;
pub const ESF_DZ_TX_KER_BUF_ADDR_WIDTH: c_int = 48;
// TX_PIO_DESC
pub const ESF_DZ_TX_PIO_TYPE_LBN: c_int = 63;
pub const ESF_DZ_TX_PIO_TYPE_WIDTH: c_int = 1;
pub const ESF_DZ_TX_PIO_OPT_LBN: c_int = 60;
pub const ESF_DZ_TX_PIO_OPT_WIDTH: c_int = 3;
pub const ESE_DZ_TX_OPTION_DESC_PIO: c_int = 1;
pub const ESF_DZ_TX_PIO_CONT_LBN: c_int = 59;
pub const ESF_DZ_TX_PIO_CONT_WIDTH: c_int = 1;
pub const ESF_DZ_TX_PIO_BYTE_CNT_LBN: c_int = 32;
pub const ESF_DZ_TX_PIO_BYTE_CNT_WIDTH: c_int = 12;
pub const ESF_DZ_TX_PIO_BUF_ADDR_LBN: c_int = 0;
pub const ESF_DZ_TX_PIO_BUF_ADDR_WIDTH: c_int = 12;
// TX_TSO_DESC
pub const ESF_DZ_TX_DESC_IS_OPT_LBN: c_int = 63;
pub const ESF_DZ_TX_DESC_IS_OPT_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_TYPE_LBN: c_int = 60;
pub const ESF_DZ_TX_OPTION_TYPE_WIDTH: c_int = 3;
pub const ESE_DZ_TX_OPTION_DESC_TSO: c_int = 7;
pub const ESE_DZ_TX_OPTION_DESC_VLAN: c_int = 6;
pub const ESE_DZ_TX_OPTION_DESC_CRC_CSUM: c_int = 0;
pub const ESF_DZ_TX_TSO_OPTION_TYPE_LBN: c_int = 56;
pub const ESF_DZ_TX_TSO_OPTION_TYPE_WIDTH: c_int = 4;
pub const ESE_DZ_TX_TSO_OPTION_DESC_FATSO2B: c_int = 3;
pub const ESE_DZ_TX_TSO_OPTION_DESC_FATSO2A: c_int = 2;
pub const ESE_DZ_TX_TSO_OPTION_DESC_ENCAP: c_int = 1;
pub const ESE_DZ_TX_TSO_OPTION_DESC_NORMAL: c_int = 0;
pub const ESF_DZ_TX_TSO_TCP_FLAGS_LBN: c_int = 48;
pub const ESF_DZ_TX_TSO_TCP_FLAGS_WIDTH: c_int = 8;
pub const ESF_DZ_TX_TSO_IP_ID_LBN: c_int = 32;
pub const ESF_DZ_TX_TSO_IP_ID_WIDTH: c_int = 16;
pub const ESF_DZ_TX_TSO_TCP_SEQNO_LBN: c_int = 0;
pub const ESF_DZ_TX_TSO_TCP_SEQNO_WIDTH: c_int = 32;
// TX_TSO_V2_DESC_A
pub const ESF_DZ_TX_DESC_IS_OPT_LBN: c_int = 63;
pub const ESF_DZ_TX_DESC_IS_OPT_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_TYPE_LBN: c_int = 60;
pub const ESF_DZ_TX_OPTION_TYPE_WIDTH: c_int = 3;
pub const ESE_DZ_TX_OPTION_DESC_TSO: c_int = 7;
pub const ESE_DZ_TX_OPTION_DESC_VLAN: c_int = 6;
pub const ESE_DZ_TX_OPTION_DESC_CRC_CSUM: c_int = 0;
pub const ESF_DZ_TX_TSO_OPTION_TYPE_LBN: c_int = 56;
pub const ESF_DZ_TX_TSO_OPTION_TYPE_WIDTH: c_int = 4;
pub const ESE_DZ_TX_TSO_OPTION_DESC_FATSO2B: c_int = 3;
pub const ESE_DZ_TX_TSO_OPTION_DESC_FATSO2A: c_int = 2;
pub const ESE_DZ_TX_TSO_OPTION_DESC_ENCAP: c_int = 1;
pub const ESE_DZ_TX_TSO_OPTION_DESC_NORMAL: c_int = 0;
pub const ESF_DZ_TX_TSO_IP_ID_LBN: c_int = 32;
pub const ESF_DZ_TX_TSO_IP_ID_WIDTH: c_int = 16;
pub const ESF_DZ_TX_TSO_TCP_SEQNO_LBN: c_int = 0;
pub const ESF_DZ_TX_TSO_TCP_SEQNO_WIDTH: c_int = 32;
// TX_TSO_V2_DESC_B
pub const ESF_DZ_TX_DESC_IS_OPT_LBN: c_int = 63;
pub const ESF_DZ_TX_DESC_IS_OPT_WIDTH: c_int = 1;
pub const ESF_DZ_TX_OPTION_TYPE_LBN: c_int = 60;
pub const ESF_DZ_TX_OPTION_TYPE_WIDTH: c_int = 3;
pub const ESE_DZ_TX_OPTION_DESC_TSO: c_int = 7;
pub const ESE_DZ_TX_OPTION_DESC_VLAN: c_int = 6;
pub const ESE_DZ_TX_OPTION_DESC_CRC_CSUM: c_int = 0;
pub const ESF_DZ_TX_TSO_OPTION_TYPE_LBN: c_int = 56;
pub const ESF_DZ_TX_TSO_OPTION_TYPE_WIDTH: c_int = 4;
pub const ESE_DZ_TX_TSO_OPTION_DESC_FATSO2B: c_int = 3;
pub const ESE_DZ_TX_TSO_OPTION_DESC_FATSO2A: c_int = 2;
pub const ESE_DZ_TX_TSO_OPTION_DESC_ENCAP: c_int = 1;
pub const ESE_DZ_TX_TSO_OPTION_DESC_NORMAL: c_int = 0;
pub const ESF_DZ_TX_TSO_TCP_MSS_LBN: c_int = 32;
pub const ESF_DZ_TX_TSO_TCP_MSS_WIDTH: c_int = 16;
pub const ESF_DZ_TX_TSO_OUTER_IPID_LBN: c_int = 0;
pub const ESF_DZ_TX_TSO_OUTER_IPID_WIDTH: c_int = 16;
//
// TX_DESC_UPD_REG: Transmit descriptor update register.
// We may write just one dword of these registers.
//

// The workaround for bug 35388 requires multiplexing writes through
// the TX_DESC_UPD_DWORD address.
// TX_DESC_UPD: 0ppppppppppp               (bit 11 lost)
// EVQ_RPTR:    1000hhhhhhhh, 1001llllllll (split into high and low bits)
// EVQ_TMR:     11mmvvvvvvvv               (bits 8:13 of value lost)
//

pub const ERF_DD_EVQ_IND_RPTR_FLAGS_LBN: c_int = 8;
pub const ERF_DD_EVQ_IND_RPTR_FLAGS_WIDTH: c_int = 4;
pub const EFE_DD_EVQ_IND_RPTR_FLAGS_HIGH: c_int = 8;
pub const EFE_DD_EVQ_IND_RPTR_FLAGS_LOW: c_int = 9;
pub const ERF_DD_EVQ_IND_RPTR_LBN: c_int = 0;
pub const ERF_DD_EVQ_IND_RPTR_WIDTH: c_int = 8;
pub const ERF_DD_EVQ_IND_TIMER_FLAGS_LBN: c_int = 10;
pub const ERF_DD_EVQ_IND_TIMER_FLAGS_WIDTH: c_int = 2;
pub const EFE_DD_EVQ_IND_TIMER_FLAGS: c_int = 3;
pub const ERF_DD_EVQ_IND_TIMER_MODE_LBN: c_int = 8;
pub const ERF_DD_EVQ_IND_TIMER_MODE_WIDTH: c_int = 2;
pub const ERF_DD_EVQ_IND_TIMER_VAL_LBN: c_int = 0;
pub const ERF_DD_EVQ_IND_TIMER_VAL_WIDTH: c_int = 8;
// TX_PIOBUF
// PIO buffer aperture (paged)
//
pub const ER_DZ_TX_PIOBUF: c_int = 4096;
pub const ER_DZ_TX_PIOBUF_SIZE: c_int = 2048;
// RX packet prefix
pub const ES_DZ_RX_PREFIX_HASH_OFST: c_int = 0;
pub const ES_DZ_RX_PREFIX_VLAN1_OFST: c_int = 4;
pub const ES_DZ_RX_PREFIX_VLAN2_OFST: c_int = 6;
pub const ES_DZ_RX_PREFIX_PKTLEN_OFST: c_int = 8;
pub const ES_DZ_RX_PREFIX_TSTAMP_OFST: c_int = 10;
pub const ES_DZ_RX_PREFIX_SIZE: c_int = 14;
