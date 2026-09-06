//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/meta/fbnic/fbnic_csr.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) Meta Platforms, Inc. and affiliates.

// Defines the minimum firmware version required by the driver

// Defines the minimum firmware version required for firmware logs

// Driver can request that firmware sends all cached logs in bulk. This
// feature was enabled on older firmware however firmware has a bug
// which attempted to send 30 messages per mbx message which caused an
// overflow flooding the mailbox. This results in a kernel warning
// related to corrupt mailbox messages.
//
// If firmware is new enough only request sending historical logs when
// the log buffer is empty to prevent duplicate logs.
//

pub const PCI_DEVICE_ID_META_FBNIC_ASIC: c_uint = 0x0013;

// Transmit Work Descriptor Format
// Length, Type, Offset Masks and Shifts

// Flags and Type

// MSS and Completion Req

// Tx Completion Descriptor Format

// Rx Buffer Descriptor Format
//
// The layout of this can vary depending on the page size of the system.
//
// If the page size is 4K then the layout will simply consist of ID for
// the 16 most significant bits, and the lower 46 are essentially the page
// address with the lowest 12 bits being reserved 0 due to the fact that
// a page will be aligned.
//
// If the page size is larger than 4K then the lower n bits of the ID and
// page address will be reserved for the fragment ID. This fragment will
// be 4K in size and will be used to index both the DMA address and the ID
// by the same amount.
//

// Rx Completion Queue Descriptors

// Address/Length Completion Descriptors

// Header AL specific values

// Optional Metadata Completion Descriptors

// Metadata Completion Descriptors

// Register Definitions
//
// The registers are laid as indexes into an le32 array. As such the actual
// address is 4 times the index value. Below each register is defined as 3
// fields, name, index, and Address.
//
// Name				Index		Address
//
// Interrupt Registers
pub const FBNIC_CSR_START_INTR: c_uint = 0x00000	/* CSR section delimiter */;

pub const FBNIC_INTR_STATUS_CNT: c_int = 8;

pub const FBNIC_INTR_MASK_CNT: c_int = 8;

pub const FBNIC_INTR_SET_CNT: c_int = 8;

pub const FBNIC_INTR_CLEAR_CNT: c_int = 8;

pub const FBNIC_INTR_SW_STATUS_CNT: c_int = 8;

pub const FBNIC_INTR_SW_AC_MODE_CNT: c_int = 8;

pub const FBNIC_INTR_MASK_SET_CNT: c_int = 8;

pub const FBNIC_INTR_MASK_CLEAR_CNT: c_int = 8;

pub const FBNIC_CSR_END_INTR: c_uint = 0x0005f	/* CSR section delimiter */;
// Interrupt MSIX Registers
pub const FBNIC_CSR_START_INTR_CQ: c_uint = 0x00400	/* CSR section delimiter */;

pub const FBNIC_INTR_CQ_REARM_CNT: c_int = 256;

pub const FBNIC_INTR_RCQ_TIMEOUT_CNT: c_int = 256;

pub const FBNIC_INTR_TCQ_TIMEOUT_CNT: c_int = 256;
pub const FBNIC_CSR_END_INTR_CQ: c_uint = 0x007fe	/* CSR section delimiter */;
// Global QM Tx registers
pub const FBNIC_CSR_START_QM_TX: c_uint = 0x00800	/* CSR section delimiter */;

pub const FBNIC_QM_TWQ_IDLE_CNT: c_int = 8;
pub const FBNIC_QM_TWQ_DEFAULT_META_L: c_uint = 0x00818		/* 0x02060 */;
pub const FBNIC_QM_TWQ_DEFAULT_META_H: c_uint = 0x00819		/* 0x02064 */;
pub const FBNIC_QM_TQS_CTL0: c_uint = 0x0081b		/* 0x0206c */;

pub const FBNIC_QM_TQS_CTL1: c_uint = 0x0081c		/* 0x02070 */;

pub const FBNIC_QM_TQS_MTU_CTL0: c_uint = 0x0081d		/* 0x02074 */;
pub const FBNIC_QM_TQS_MTU_CTL1: c_uint = 0x0081e		/* 0x02078 */;

pub const FBNIC_QM_TCQ_IDLE_CNT: c_int = 4;
pub const FBNIC_QM_TCQ_CTL0: c_uint = 0x0082d		/* 0x020b4 */;

pub const FBNIC_QM_TQS_IDLE_CNT: c_int = 8;
pub const FBNIC_QM_TQS_EDT_TS_RANGE: c_uint = 0x00849		/* 0x2124 */;

pub const FBNIC_QM_TDE_IDLE_CNT: c_int = 8;
pub const FBNIC_QM_TNI_TDF_CTL: c_uint = 0x0086c		/* 0x021b0 */;

pub const FBNIC_QM_TNI_TDE_CTL: c_uint = 0x0086d		/* 0x021b4 */;

pub const FBNIC_QM_TNI_TCM_CTL: c_uint = 0x0086e		/* 0x021b8 */;

pub const FBNIC_CSR_END_QM_TX: c_uint = 0x00873	/* CSR section delimiter */;
// Global QM Rx registers
pub const FBNIC_CSR_START_QM_RX: c_uint = 0x00c00	/* CSR section delimiter */;

pub const FBNIC_QM_RCQ_IDLE_CNT: c_int = 4;
pub const FBNIC_QM_RCQ_CTL0: c_uint = 0x00c0c		/* 0x03030 */;

pub const FBNIC_QM_HPQ_IDLE_CNT: c_int = 4;

pub const FBNIC_QM_PPQ_IDLE_CNT: c_int = 4;
pub const FBNIC_QM_RNI_RBP_CTL: c_uint = 0x00c2d		/* 0x030b4 */;

pub const FBNIC_QM_RNI_RDE_CTL: c_uint = 0x00c2e		/* 0x030b8 */;

pub const FBNIC_QM_RNI_RCM_CTL: c_uint = 0x00c2f		/* 0x030bc */;

pub const FBNIC_CSR_END_QM_RX: c_uint = 0x00c34	/* CSR section delimiter */;
// TCE registers
pub const FBNIC_CSR_START_TCE: c_uint = 0x04000	/* CSR section delimiter */;
pub const FBNIC_TCE_REG_BASE: c_uint = 0x04000		/* 0x10000 */;
pub const FBNIC_TCE_LSO_CTRL: c_uint = 0x04000		/* 0x10000 */;

pub const FBNIC_TCE_CSO_CTRL: c_uint = 0x04001		/* 0x10004 */;

pub const FBNIC_TCE_TXB_CTRL: c_uint = 0x04002		/* 0x10008 */;

pub const FBNIC_TCE_TXB_ENQ_WRR_CTRL: c_uint = 0x04003		/* 0x1000c */;

pub const FBNIC_TCE_TXB_TEI_Q0_CTRL: c_uint = 0x04004		/* 0x10010 */;
pub const FBNIC_TCE_TXB_TEI_Q1_CTRL: c_uint = 0x04005		/* 0x10014 */;
pub const FBNIC_TCE_TXB_MC_Q_CTRL: c_uint = 0x04006		/* 0x10018 */;
pub const FBNIC_TCE_TXB_RX_TEI_Q_CTRL: c_uint = 0x04007		/* 0x1001c */;
pub const FBNIC_TCE_TXB_RX_BMC_Q_CTRL: c_uint = 0x04008		/* 0x10020 */;

pub const FBNIC_TCE_TXB_TEI_DWRR_CTRL: c_uint = 0x04009		/* 0x10024 */;

pub const FBNIC_TCE_TXB_NTWRK_DWRR_CTRL: c_uint = 0x0400a		/* 0x10028 */;

pub const FBNIC_TCE_TXB_CLDR_CFG: c_uint = 0x0400b		/* 0x1002c */;

pub const FBNIC_TCE_TXB_CLDR_SLOT_CFG_CNT: c_int = 16;

pub const FBNIC_TCE_BMC_MAX_PKTSZ: c_uint = 0x0403a		/* 0x100e8 */;

pub const FBNIC_TCE_MC_MAX_PKTSZ: c_uint = 0x0403b		/* 0x100ec */;

pub const FBNIC_TCE_SOP_PROT_CTRL: c_uint = 0x0403c		/* 0x100f0 */;

pub const FBNIC_TCE_DROP_CTRL: c_uint = 0x0403d		/* 0x100f4 */;

pub const FBNIC_TCE_TTI_CM_DROP_PKTS: c_uint = 0x0403e		/* 0x100f8 */;
pub const FBNIC_TCE_TTI_CM_DROP_BYTE_L: c_uint = 0x0403f		/* 0x100fc */;
pub const FBNIC_TCE_TTI_CM_DROP_BYTE_H: c_uint = 0x04040		/* 0x10100 */;
pub const FBNIC_TCE_TTI_FRAME_DROP_PKTS: c_uint = 0x04041		/* 0x10104 */;
pub const FBNIC_TCE_TTI_FRAME_DROP_BYTE_L: c_uint = 0x04042		/* 0x10108 */;
pub const FBNIC_TCE_TTI_FRAME_DROP_BYTE_H: c_uint = 0x04043		/* 0x1010c */;
pub const FBNIC_TCE_TBI_DROP_PKTS: c_uint = 0x04044		/* 0x10110 */;
pub const FBNIC_TCE_TBI_DROP_BYTE_L: c_uint = 0x04045		/* 0x10114 */;
pub const FBNIC_TCE_TCAM_IDX2DEST_MAP: c_uint = 0x0404A		/* 0x10128 */;

pub const FBNIC_TCE_TXB_TX_BMC_Q_CTRL: c_uint = 0x0404B		/* 0x1012c */;
pub const FBNIC_TCE_TXB_BMC_DWRR_CTRL: c_uint = 0x0404C		/* 0x10130 */;

pub const FBNIC_TCE_TXB_TEI_DWRR_CTRL_EXT: c_uint = 0x0404D		/* 0x10134 */;

pub const FBNIC_TCE_TXB_BMC_DWRR_CTRL_EXT: c_uint = 0x0404F		/* 0x1013c */;
pub const FBNIC_CSR_END_TCE: c_uint = 0x04050	/* CSR section delimiter */;
// TCE RAM registers
pub const FBNIC_CSR_START_TCE_RAM: c_uint = 0x04200	/* CSR section delimiter */;

pub const FBNIC_CSR_END_TCE_RAM: c_uint = 0x0421F	/* CSR section delimiter */;
// TMI registers
pub const FBNIC_CSR_START_TMI: c_uint = 0x04400	/* CSR section delimiter */;
pub const FBNIC_TMI_SOP_PROT_CTRL: c_uint = 0x04400		/* 0x11000 */;
pub const FBNIC_TMI_DROP_CTRL: c_uint = 0x04401		/* 0x11004 */;

pub const FBNIC_TMI_DROP_PKTS: c_uint = 0x04402		/* 0x11008 */;
pub const FBNIC_TMI_DROP_BYTE_L: c_uint = 0x04403		/* 0x1100c */;
pub const FBNIC_TMI_ILLEGAL_PTP_REQS: c_uint = 0x04409		/* 0x11024 */;
pub const FBNIC_TMI_GOOD_PTP_TS: c_uint = 0x0440a		/* 0x11028 */;
pub const FBNIC_TMI_BAD_PTP_TS: c_uint = 0x0440b		/* 0x1102c */;

pub const FBNIC_CSR_END_TMI: c_uint = 0x0443f	/* CSR section delimiter */;
// Precision Time Protocol Registers
pub const FBNIC_CSR_START_PTP: c_uint = 0x04800 /* CSR section delimiter */;
pub const FBNIC_PTP_REG_BASE: c_uint = 0x04800		/* 0x12000 */;
pub const FBNIC_PTP_CTRL: c_uint = 0x04800		/* 0x12000 */;

pub const FBNIC_PTP_ADJUST: c_uint = 0x04801		/* 0x12004 */;

pub const FBNIC_PTP_INIT_HI: c_uint = 0x04802		/* 0x12008 */;
pub const FBNIC_PTP_INIT_LO: c_uint = 0x04803		/* 0x1200c */;
pub const FBNIC_PTP_NUDGE_NS: c_uint = 0x04804		/* 0x12010 */;
pub const FBNIC_PTP_NUDGE_SUBNS: c_uint = 0x04805		/* 0x12014 */;
pub const FBNIC_PTP_ADD_VAL_NS: c_uint = 0x04806		/* 0x12018 */;

pub const FBNIC_PTP_ADD_VAL_SUBNS: c_uint = 0x04807		/* 0x1201c */;
pub const FBNIC_PTP_CTR_VAL_HI: c_uint = 0x04808		/* 0x12020 */;
pub const FBNIC_PTP_CTR_VAL_LO: c_uint = 0x04809		/* 0x12024 */;
pub const FBNIC_PTP_MONO_PTP_CTR_HI: c_uint = 0x0480a		/* 0x12028 */;
pub const FBNIC_PTP_MONO_PTP_CTR_LO: c_uint = 0x0480b		/* 0x1202c */;
pub const FBNIC_PTP_CDC_FIFO_STATUS: c_uint = 0x0480c		/* 0x12030 */;
pub const FBNIC_PTP_SPARE: c_uint = 0x0480d		/* 0x12034 */;
pub const FBNIC_CSR_END_PTP: c_uint = 0x0480d /* CSR section delimiter */;
// Rx Buffer Registers
pub const FBNIC_CSR_START_RXB: c_uint = 0x08000	/* CSR section delimiter */;
// Unused

pub const FBNIC_RXB_CT_SIZE_CNT: c_int = 8;

pub const FBNIC_RXB_PAUSE_DROP_CTRL: c_uint = 0x08008		/* 0x20020 */;

pub const FBNIC_RXB_PAUSE_THLD_CNT: c_int = 8;

pub const FBNIC_RXB_DROP_THLD_CNT: c_int = 8;

pub const FBNIC_RXB_PAUSE_STORM_CNT: c_int = 4;

pub const FBNIC_RXB_PAUSE_STORM_UNIT_WR: c_uint = 0x0801d		/* 0x20074 */;

pub const FBNIC_RXB_ECN_THLD_CNT: c_int = 8;

pub const FBNIC_RXB_PBUF_CFG_CNT: c_int = 8;

pub const FBNIC_RXB_DWRR_RDE_WEIGHT0: c_uint = 0x0802f		/* 0x200bc */;

pub const FBNIC_RXB_DWRR_RDE_WEIGHT1: c_uint = 0x08030		/* 0x200c0 */;

pub const FBNIC_RXB_DWRR_BMC_WEIGHT: c_uint = 0x08031		/* 0x200c4 */;

pub const FBNIC_RXB_CLDR_PRIO_CFG_CNT: c_int = 16;
pub const FBNIC_RXB_ENDIAN_FCS: c_uint = 0x08044		/* 0x20110 */;
// Unused

pub const FBNIC_RXB_PBUF_CREDIT_CNT: c_int = 8;

pub const FBNIC_RXB_INTF_CREDIT: c_uint = 0x0804f		/* 0x2013C */;

pub const FBNIC_RXB_ERR_INTR_STS: c_uint = 0x08050		/* 0x20140 */;

pub const FBNIC_RXB_ERR_INTR_MASK: c_uint = 0x08052		/* 0x20148 */;

// Unused

pub const FBNIC_RXB_PAUSE_STORM_UNIT_RD: c_uint = 0x08125		/* 0x20494 */;

pub const FBNIC_RXB_DWRR_RDE_WEIGHT0_EXT: c_uint = 0x08143		/* 0x2050c */;
pub const FBNIC_RXB_DWRR_RDE_WEIGHT1_EXT: c_uint = 0x08144		/* 0x20510 */;
pub const FBNIC_CSR_END_RXB: c_uint = 0x081b1	/* CSR section delimiter */;
// Rx Parser and Classifier Registers
pub const FBNIC_CSR_START_RPC: c_uint = 0x08400	/* CSR section delimiter */;
pub const FBNIC_RPC_RMI_CONFIG: c_uint = 0x08400		/* 0x21000 */;

pub const FBNIC_RPC_ACT_TBL0_DEFAULT: c_uint = 0x0840a		/* 0x21028 */;

pub const FBNIC_RPC_ACT_TBL1_DEFAULT: c_uint = 0x0840b		/* 0x2102c */;

pub const FBNIC_RPC_RSS_KEY_BIT_LEN: c_int = 425;

pub const FBNIC_RPC_CNTR_TCP_OPT_ERR: c_uint = 0x0849e		/* 0x21278 */;
pub const FBNIC_RPC_CNTR_UNKN_ETYPE: c_uint = 0x0849f		/* 0x2127c */;
pub const FBNIC_RPC_CNTR_IPV4_FRAG: c_uint = 0x084a0		/* 0x21280 */;
pub const FBNIC_RPC_CNTR_IPV6_FRAG: c_uint = 0x084a1		/* 0x21284 */;
pub const FBNIC_RPC_CNTR_IPV4_ESP: c_uint = 0x084a2		/* 0x21288 */;
pub const FBNIC_RPC_CNTR_IPV6_ESP: c_uint = 0x084a3		/* 0x2128c */;
pub const FBNIC_RPC_CNTR_UNKN_EXT_HDR: c_uint = 0x084a4		/* 0x21290 */;
pub const FBNIC_RPC_CNTR_OUT_OF_HDR_ERR: c_uint = 0x084a5		/* 0x21294 */;
pub const FBNIC_RPC_CNTR_OVR_SIZE_ERR: c_uint = 0x084a6		/* 0x21298 */;
pub const FBNIC_RPC_TCAM_MACDA_VALIDATE: c_uint = 0x0852d		/* 0x214b4 */;

pub const FBNIC_CSR_END_RPC: c_uint = 0x0856b	/* CSR section delimiter */;
// RPC RAM Registers
pub const FBNIC_CSR_START_RPC_RAM: c_uint = 0x08800	/* CSR section delimiter */;

pub const FBNIC_RPC_ACT_TBL_NUM_ENTRIES: c_int = 64;
// TCAM Tables

// 64 Action TCAM Entries, 12 registers
// 3 mixed, src port, dst port, 6 L4 words, and Validate
//

pub const FBNIC_RPC_RSS_TBL_COUNT: c_int = 2;
pub const FBNIC_RPC_RSS_TBL_SIZE: c_int = 256;
pub const FBNIC_CSR_END_RPC_RAM: c_uint = 0x08f1f	/* CSR section delimiter */;
// Fab Registers
pub const FBNIC_CSR_START_FAB: c_uint = 0x0C000 /* CSR section delimiter */;
pub const FBNIC_FAB_AXI4_AR_SPACER_2_CFG: c_uint = 0x0C005		/* 0x30014 */;

pub const FBNIC_CSR_END_FAB: c_uint = 0x0C020	    /* CSR section delimiter */;
// Master Registers
pub const FBNIC_CSR_START_MASTER: c_uint = 0x0C400	/* CSR section delimiter */;
pub const FBNIC_MASTER_SPARE_0: c_uint = 0x0C41B		/* 0x3106c */;
pub const FBNIC_CSR_END_MASTER: c_uint = 0x0C452	/* CSR section delimiter */;
// MAC PCS registers
pub const FBNIC_CSR_START_PCS: c_uint = 0x10000 /* CSR section delimiter */;

pub const FBNIC_CSR_END_PCS: c_uint = 0x10668 /* CSR section delimiter */;
pub const FBNIC_CSR_START_RSFEC: c_uint = 0x10800 /* CSR section delimiter */;

// We have 4 RSFEC engines present in our part, however we are only using 1.
// As such only CCW(0) and NCCW(0) will never be non-zero and the other
// registers can be ignored.
//

pub const FBNIC_PCS_MAX_LANES: c_int = 4;

pub const FBNIC_CSR_END_RSFEC: c_uint = 0x108c8 /* CSR section delimiter */;
// MAC MAC registers (ASIC only)
pub const FBNIC_CSR_START_MAC_MAC: c_uint = 0x11000 /* CSR section delimiter */;
pub const FBNIC_MAC_COMMAND_CONFIG: c_uint = 0x11002		/* 0x44008 */;

pub const FBNIC_MAC_CL01_PAUSE_QUANTA: c_uint = 0x11015		/* 0x44054 */;
pub const FBNIC_MAC_CL01_QUANTA_THRESH: c_uint = 0x11019		/* 0x44064 */;
pub const FBNIC_CSR_END_MAC_MAC: c_uint = 0x11028 /* CSR section delimiter */;
// Signals from MAC, AN, PCS, and LED CSR registers (ASIC only)
pub const FBNIC_CSR_START_SIG: c_uint = 0x11800 /* CSR section delimiter */;
pub const FBNIC_SIG_MAC_IN0: c_uint = 0x11800		/* 0x46000 */;

pub const FBNIC_SIG_PCS_OUT0: c_uint = 0x11808		/* 0x46020 */;

pub const FBNIC_SIG_PCS_OUT1: c_uint = 0x11809		/* 0x46024 */;

pub const FBNIC_SIG_PCS_INTR_STS: c_uint = 0x11814		/* 0x46050 */;

pub const FBNIC_SIG_PCS_INTR_MASK: c_uint = 0x11816		/* 0x46058 */;
pub const FBNIC_CSR_END_SIG: c_uint = 0x1184e /* CSR section delimiter */;
pub const FBNIC_CSR_START_MAC_STAT: c_uint = 0x11a00;
pub const FBNIC_MAC_STAT_RX_XOFF_STB_L: c_uint = 0x11a00		/* 0x46800 */;
pub const FBNIC_MAC_STAT_RX_XOFF_STB_H: c_uint = 0x11a01		/* 0x46804 */;
pub const FBNIC_MAC_STAT_TX_XOFF_STB_L: c_uint = 0x11a04		/* 0x46810 */;
pub const FBNIC_MAC_STAT_TX_XOFF_STB_H: c_uint = 0x11a05		/* 0x46814 */;
pub const FBNIC_MAC_STAT_RX_BYTE_COUNT_L: c_uint = 0x11a08		/* 0x46820 */;
pub const FBNIC_MAC_STAT_RX_BYTE_COUNT_H: c_uint = 0x11a09		/* 0x46824 */;
pub const FBNIC_MAC_STAT_RX_ALIGN_ERROR_L: c_uint = 0x11a0a		/* 0x46828 */;
pub const FBNIC_MAC_STAT_RX_ALIGN_ERROR_H: c_uint = 0x11a0b		/* 0x4682c */;
pub const FBNIC_MAC_STAT_RX_TOOLONG_L: c_uint = 0x11a0e		/* 0x46838 */;
pub const FBNIC_MAC_STAT_RX_TOOLONG_H: c_uint = 0x11a0f		/* 0x4683c */;
pub const FBNIC_MAC_STAT_RX_RECEIVED_OK_L: c_uint = 0x11a12		/* 0x46848 */;
pub const FBNIC_MAC_STAT_RX_RECEIVED_OK_H: c_uint = 0x11a13		/* 0x4684c */;

pub const FBNIC_MAC_STAT_RX_IFINERRORS_L: c_uint = 0x11a18		/* 0x46860 */;
pub const FBNIC_MAC_STAT_RX_IFINERRORS_H: c_uint = 0x11a19		/* 0x46864 */;
pub const FBNIC_MAC_STAT_RX_MULTICAST_L: c_uint = 0x11a1c		/* 0x46870 */;
pub const FBNIC_MAC_STAT_RX_MULTICAST_H: c_uint = 0x11a1d		/* 0x46874 */;
pub const FBNIC_MAC_STAT_RX_BROADCAST_L: c_uint = 0x11a1e		/* 0x46878 */;
pub const FBNIC_MAC_STAT_RX_BROADCAST_H: c_uint = 0x11a1f		/* 0x4687c */;
pub const FBNIC_MAC_STAT_RX_UNDERSIZE_L: c_uint = 0x11a24		/* 0x46890 */;
pub const FBNIC_MAC_STAT_RX_UNDERSIZE_H: c_uint = 0x11a25		/* 0x46894 */;

pub const FBNIC_MAC_STAT_RX_OVERSIZE_L: c_uint = 0x11a34		/* 0x468d0 */;
pub const FBNIC_MAC_STAT_RX_OVERSSIZE_H: c_uint = 0x11a35		/* 0x468d4 */;
pub const FBNIC_MAC_STAT_RX_JABBER_L: c_uint = 0x11a36		/* 0x468d8 */;
pub const FBNIC_MAC_STAT_RX_JABBER_H: c_uint = 0x11a37		/* 0x468dc */;
pub const FBNIC_MAC_STAT_RX_FRAGMENT_L: c_uint = 0x11a38		/* 0x468e0 */;
pub const FBNIC_MAC_STAT_RX_FRAGMENT_H: c_uint = 0x11a39		/* 0x468e4 */;

pub const FBNIC_MAC_STAT_TX_BYTE_COUNT_L: c_uint = 0x11a3e		/* 0x468f8 */;
pub const FBNIC_MAC_STAT_TX_BYTE_COUNT_H: c_uint = 0x11a3f		/* 0x468fc */;

pub const FBNIC_MAC_STAT_TX_IFOUTERRORS_L: c_uint = 0x11a46		/* 0x46918 */;
pub const FBNIC_MAC_STAT_TX_IFOUTERRORS_H: c_uint = 0x11a47		/* 0x4691c */;
pub const FBNIC_MAC_STAT_TX_MULTICAST_L: c_uint = 0x11a4a		/* 0x46928 */;
pub const FBNIC_MAC_STAT_TX_MULTICAST_H: c_uint = 0x11a4b		/* 0x4692c */;
pub const FBNIC_MAC_STAT_TX_BROADCAST_L: c_uint = 0x11a4c		/* 0x46930 */;
pub const FBNIC_MAC_STAT_TX_BROADCAST_H: c_uint = 0x11a4d		/* 0x46934 */;

// PCIE Comphy Registers
pub const FBNIC_CSR_START_PCIE_SS_COMPHY: c_uint = 0x2442e /* CSR section delimiter */;
pub const FBNIC_CSR_END_PCIE_SS_COMPHY: c_uint = 0x279d7	/* CSR section delimiter */;
// PUL User Registers
pub const FBNIC_CSR_START_PUL_USER: c_uint = 0x31000	/* CSR section delimiter */;
pub const FBNIC_PUL_OB_TLP_HDR_AW_CFG: c_uint = 0x3103d		/* 0xc40f4 */;

pub const FBNIC_PUL_OB_TLP_HDR_AR_CFG: c_uint = 0x3103e		/* 0xc40f8 */;

pub const FBNIC_CSR_END_PUL_USER: c_uint = 0x310ea	/* CSR section delimiter */;
// Queue Registers
//
// The queue register offsets are specific for a given queue grouping. So to
// find the actual register offset it is necessary to combine FBNIC_QUEUE(n)
// with the register to get the actual register offset like so:
// FBNIC_QUEUE_TWQ0_CTL(n) == FBNIC_QUEUE(n) + FBNIC_QUEUE_TWQ0_CTL
//
pub const FBNIC_CSR_START_QUEUE: c_uint = 0x40000	/* CSR section delimiter */;
pub const FBNIC_QUEUE_STRIDE: c_uint = 0x400		/* 0x1000 */;
// Macro flag: #define FBNIC_QUEUE(n)\
pub const FBNIC_QUEUE_TWQ0_CTL: c_uint = 0x000		/* 0x000 */;
pub const FBNIC_QUEUE_TWQ1_CTL: c_uint = 0x001		/* 0x004 */;

pub const FBNIC_QUEUE_TWQ0_TAIL: c_uint = 0x002		/* 0x008 */;
pub const FBNIC_QUEUE_TWQ1_TAIL: c_uint = 0x003		/* 0x00c */;
pub const FBNIC_QUEUE_TWQ0_PTRS: c_uint = 0x004		/* 0x010 */;
pub const FBNIC_QUEUE_TWQ1_PTRS: c_uint = 0x005		/* 0x014 */;

pub const FBNIC_QUEUE_TWQ0_SIZE: c_uint = 0x00a		/* 0x028 */;
pub const FBNIC_QUEUE_TWQ1_SIZE: c_uint = 0x00b		/* 0x02c */;

pub const FBNIC_QUEUE_TWQ0_BAL: c_uint = 0x020		/* 0x080 */;

pub const FBNIC_QUEUE_TWQ0_BAH: c_uint = 0x021		/* 0x084 */;
pub const FBNIC_QUEUE_TWQ1_BAL: c_uint = 0x022		/* 0x088 */;
pub const FBNIC_QUEUE_TWQ1_BAH: c_uint = 0x023		/* 0x08c */;
// Tx Work Queue Statistics Registers
pub const FBNIC_QUEUE_TWQ0_PKT_CNT: c_uint = 0x062		/* 0x188 */;
pub const FBNIC_QUEUE_TWQ0_ERR_CNT: c_uint = 0x063		/* 0x18c */;
pub const FBNIC_QUEUE_TWQ1_PKT_CNT: c_uint = 0x072		/* 0x1c8 */;
pub const FBNIC_QUEUE_TWQ1_ERR_CNT: c_uint = 0x073		/* 0x1cc */;
// Tx Completion Queue Registers
pub const FBNIC_QUEUE_TCQ_CTL: c_uint = 0x080		/* 0x200 */;

pub const FBNIC_QUEUE_TCQ_HEAD: c_uint = 0x081		/* 0x204 */;
pub const FBNIC_QUEUE_TCQ_PTRS: c_uint = 0x082		/* 0x208 */;

pub const FBNIC_QUEUE_TCQ_SIZE: c_uint = 0x084		/* 0x210 */;

pub const FBNIC_QUEUE_TCQ_BAL: c_uint = 0x0a0		/* 0x280 */;
pub const FBNIC_QUEUE_TCQ_BAH: c_uint = 0x0a1		/* 0x284 */;
// Tx Interrupt Manager Registers
pub const FBNIC_QUEUE_TIM_CTL: c_uint = 0x0c0		/* 0x300 */;

pub const FBNIC_QUEUE_TIM_THRESHOLD: c_uint = 0x0c1		/* 0x304 */;

pub const FBNIC_QUEUE_TIM_CLEAR: c_uint = 0x0c2		/* 0x308 */;

pub const FBNIC_QUEUE_TIM_SET: c_uint = 0x0c3		/* 0x30c */;

pub const FBNIC_QUEUE_TIM_MASK: c_uint = 0x0c4		/* 0x310 */;

pub const FBNIC_QUEUE_TIM_TIMER: c_uint = 0x0c5		/* 0x314 */;
pub const FBNIC_QUEUE_TIM_COUNTS: c_uint = 0x0c6		/* 0x318 */;

// Rx Completion Queue Registers
pub const FBNIC_QUEUE_RCQ_CTL: c_uint = 0x200		/* 0x800 */;

pub const FBNIC_QUEUE_RCQ_HEAD: c_uint = 0x201		/* 0x804 */;
pub const FBNIC_QUEUE_RCQ_PTRS: c_uint = 0x202		/* 0x808 */;

pub const FBNIC_QUEUE_RCQ_SIZE: c_uint = 0x204		/* 0x810 */;

pub const FBNIC_QUEUE_RCQ_BAL: c_uint = 0x220		/* 0x880 */;
pub const FBNIC_QUEUE_RCQ_BAH: c_uint = 0x221		/* 0x884 */;
// Rx Buffer Descriptor Queue Registers
pub const FBNIC_QUEUE_BDQ_CTL: c_uint = 0x240		/* 0x900 */;

pub const FBNIC_QUEUE_BDQ_HPQ_TAIL: c_uint = 0x241		/* 0x904 */;
pub const FBNIC_QUEUE_BDQ_PPQ_TAIL: c_uint = 0x242		/* 0x908 */;
pub const FBNIC_QUEUE_BDQ_HPQ_PTRS: c_uint = 0x243		/* 0x90c */;
pub const FBNIC_QUEUE_BDQ_PPQ_PTRS: c_uint = 0x244		/* 0x910 */;

pub const FBNIC_QUEUE_BDQ_HPQ_SIZE: c_uint = 0x247		/* 0x91c */;
pub const FBNIC_QUEUE_BDQ_PPQ_SIZE: c_uint = 0x248		/* 0x920 */;

pub const FBNIC_QUEUE_BDQ_HPQ_BAL: c_uint = 0x260		/* 0x980 */;
pub const FBNIC_QUEUE_BDQ_HPQ_BAH: c_uint = 0x261		/* 0x984 */;
pub const FBNIC_QUEUE_BDQ_PPQ_BAL: c_uint = 0x262		/* 0x988 */;
pub const FBNIC_QUEUE_BDQ_PPQ_BAH: c_uint = 0x263		/* 0x98c */;
// Rx DMA Engine Configuration
pub const FBNIC_QUEUE_RDE_CTL0: c_uint = 0x2a0		/* 0xa80 */;

pub const FBNIC_QUEUE_RDE_CTL1: c_uint = 0x2a1		/* 0xa84 */;

// Rx Per CQ Statistics Counters
pub const FBNIC_QUEUE_RDE_PKT_CNT: c_uint = 0x2a2		/* 0xa88 */;
pub const FBNIC_QUEUE_RDE_PKT_ERR_CNT: c_uint = 0x2a3		/* 0xa8c */;
pub const FBNIC_QUEUE_RDE_CQ_DROP_CNT: c_uint = 0x2a4		/* 0xa90 */;
pub const FBNIC_QUEUE_RDE_BDQ_DROP_CNT: c_uint = 0x2a5		/* 0xa94 */;
// Rx Interrupt Manager Registers
pub const FBNIC_QUEUE_RIM_CTL: c_uint = 0x2c0		/* 0xb00 */;

pub const FBNIC_QUEUE_RIM_THRESHOLD: c_uint = 0x2c1		/* 0xb04 */;

pub const FBNIC_QUEUE_RIM_CLEAR: c_uint = 0x2c2		/* 0xb08 */;

pub const FBNIC_QUEUE_RIM_SET: c_uint = 0x2c3		/* 0xb0c */;

pub const FBNIC_QUEUE_RIM_MASK: c_uint = 0x2c4		/* 0xb10 */;

pub const FBNIC_QUEUE_RIM_COAL_STATUS: c_uint = 0x2c5		/* 0xb14 */;

pub const FBNIC_MAX_QUEUES: c_int = 128;

// BAR 4 CSRs
// The IPC mailbox consists of 32 mailboxes, with each mailbox consisting
// of 32 4 byte registers. We will use 2 registers per descriptor so the
// length of the mailbox is reduced to 16.
//
// Currently we use an offset of 0x6000 on BAR4 for the mailbox so we just
// have to do the math and determine the offset based on the mailbox
// direction and index inside that mailbox.
//
pub const FBNIC_IPC_MBX_DESC_LEN: c_int = 16;

// Use first register in mailbox to flush writes

// OTP Registers
// These registers are accessible via bar4 offset and are written by CMRT
// on boot. For the write status, the register is broken up in half with OTP
// Write Data Status occupying the top 16 bits and the ECC status occupying the
// bottom 16 bits.
//
pub const FBNIC_NS_OTP_STATUS: c_uint = 0x0021d;
pub const FBNIC_NS_OTP_WRITE_STATUS: c_uint = 0x0021e;

//
// enum fbnic_reg_self_test_codes - return codes from self test routines
//
// This is the code that is returned from the register self test
// routines.
//
// The test either returns success or the register number
// that failed during the test.
//
// @FBNIC_REG_TEST_SUCCESS: no errors
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fbnic_reg_self_test_codes {
    FBNIC_REG_TEST_SUCCESS = 0,
}

extern "C" {
    pub fn fbnic_csr_regs_test(fbd: *mut fbnic_dev) -> fbnic_reg_self_test_codes;
}
