//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/calxeda/xgmac.c
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
// Copyright 2010-2011 Calxeda, Inc.
//

// XGMAC Register definitions
pub const XGMAC_CONTROL: c_uint = 0x00000000	/* MAC Configuration */;
pub const XGMAC_FRAME_FILTER: c_uint = 0x00000004	/* MAC Frame Filter */;
pub const XGMAC_FLOW_CTRL: c_uint = 0x00000018	/* MAC Flow Control */;
pub const XGMAC_VLAN_TAG: c_uint = 0x0000001C	/* VLAN Tags */;
pub const XGMAC_VERSION: c_uint = 0x00000020	/* Version */;
pub const XGMAC_VLAN_INCL: c_uint = 0x00000024	/* VLAN tag for tx frames */;
pub const XGMAC_LPI_CTRL: c_uint = 0x00000028	/* LPI Control and Status */;
pub const XGMAC_LPI_TIMER: c_uint = 0x0000002C	/* LPI Timers Control */;
pub const XGMAC_TX_PACE: c_uint = 0x00000030	/* Transmit Pace and Stretch */;
pub const XGMAC_VLAN_HASH: c_uint = 0x00000034	/* VLAN Hash Table */;
pub const XGMAC_DEBUG: c_uint = 0x00000038	/* Debug */;
pub const XGMAC_INT_STAT: c_uint = 0x0000003C	/* Interrupt and Control */;

pub const XGMAC_NUM_HASH: c_int = 16;
pub const XGMAC_OMR: c_uint = 0x00000400;
pub const XGMAC_REMOTE_WAKE: c_uint = 0x00000700	/* Remote Wake-Up Frm Filter */;
pub const XGMAC_PMT: c_uint = 0x00000704	/* PMT Control and Status */;
pub const XGMAC_MMC_CTRL: c_uint = 0x00000800	/* XGMAC MMC Control */;
pub const XGMAC_MMC_INTR_RX: c_uint = 0x00000804	/* Receive Interrupt */;
pub const XGMAC_MMC_INTR_TX: c_uint = 0x00000808	/* Transmit Interrupt */;
pub const XGMAC_MMC_INTR_MASK_RX: c_uint = 0x0000080c	/* Receive Interrupt Mask */;
pub const XGMAC_MMC_INTR_MASK_TX: c_uint = 0x00000810	/* Transmit Interrupt Mask */;
// Hardware TX Statistics Counters
pub const XGMAC_MMC_TXOCTET_GB_LO: c_uint = 0x00000814;
pub const XGMAC_MMC_TXOCTET_GB_HI: c_uint = 0x00000818;
pub const XGMAC_MMC_TXFRAME_GB_LO: c_uint = 0x0000081C;
pub const XGMAC_MMC_TXFRAME_GB_HI: c_uint = 0x00000820;
pub const XGMAC_MMC_TXBCFRAME_G: c_uint = 0x00000824;
pub const XGMAC_MMC_TXMCFRAME_G: c_uint = 0x0000082C;
pub const XGMAC_MMC_TXUCFRAME_GB: c_uint = 0x00000864;
pub const XGMAC_MMC_TXMCFRAME_GB: c_uint = 0x0000086C;
pub const XGMAC_MMC_TXBCFRAME_GB: c_uint = 0x00000874;
pub const XGMAC_MMC_TXUNDERFLOW: c_uint = 0x0000087C;
pub const XGMAC_MMC_TXOCTET_G_LO: c_uint = 0x00000884;
pub const XGMAC_MMC_TXOCTET_G_HI: c_uint = 0x00000888;
pub const XGMAC_MMC_TXFRAME_G_LO: c_uint = 0x0000088C;
pub const XGMAC_MMC_TXFRAME_G_HI: c_uint = 0x00000890;
pub const XGMAC_MMC_TXPAUSEFRAME: c_uint = 0x00000894;
pub const XGMAC_MMC_TXVLANFRAME: c_uint = 0x0000089C;
// Hardware RX Statistics Counters
pub const XGMAC_MMC_RXFRAME_GB_LO: c_uint = 0x00000900;
pub const XGMAC_MMC_RXFRAME_GB_HI: c_uint = 0x00000904;
pub const XGMAC_MMC_RXOCTET_GB_LO: c_uint = 0x00000908;
pub const XGMAC_MMC_RXOCTET_GB_HI: c_uint = 0x0000090C;
pub const XGMAC_MMC_RXOCTET_G_LO: c_uint = 0x00000910;
pub const XGMAC_MMC_RXOCTET_G_HI: c_uint = 0x00000914;
pub const XGMAC_MMC_RXBCFRAME_G: c_uint = 0x00000918;
pub const XGMAC_MMC_RXMCFRAME_G: c_uint = 0x00000920;
pub const XGMAC_MMC_RXCRCERR: c_uint = 0x00000928;
pub const XGMAC_MMC_RXRUNT: c_uint = 0x00000930;
pub const XGMAC_MMC_RXJABBER: c_uint = 0x00000934;
pub const XGMAC_MMC_RXUCFRAME_G: c_uint = 0x00000970;
pub const XGMAC_MMC_RXLENGTHERR: c_uint = 0x00000978;
pub const XGMAC_MMC_RXPAUSEFRAME: c_uint = 0x00000988;
pub const XGMAC_MMC_RXOVERFLOW: c_uint = 0x00000990;
pub const XGMAC_MMC_RXVLANFRAME: c_uint = 0x00000998;
pub const XGMAC_MMC_RXWATCHDOG: c_uint = 0x000009a0;
// DMA Control and Status Registers
pub const XGMAC_DMA_BUS_MODE: c_uint = 0x00000f00	/* Bus Mode */;
pub const XGMAC_DMA_TX_POLL: c_uint = 0x00000f04	/* Transmit Poll Demand */;
pub const XGMAC_DMA_RX_POLL: c_uint = 0x00000f08	/* Received Poll Demand */;
pub const XGMAC_DMA_RX_BASE_ADDR: c_uint = 0x00000f0c	/* Receive List Base */;
pub const XGMAC_DMA_TX_BASE_ADDR: c_uint = 0x00000f10	/* Transmit List Base */;
pub const XGMAC_DMA_STATUS: c_uint = 0x00000f14	/* Status Register */;
pub const XGMAC_DMA_CONTROL: c_uint = 0x00000f18	/* Ctrl (Operational Mode) */;
pub const XGMAC_DMA_INTR_ENA: c_uint = 0x00000f1c	/* Interrupt Enable */;
pub const XGMAC_DMA_MISS_FRAME_CTR: c_uint = 0x00000f20	/* Missed Frame Counter */;
pub const XGMAC_DMA_RI_WDOG_TIMER: c_uint = 0x00000f24	/* RX Intr Watchdog Timer */;
pub const XGMAC_DMA_AXI_BUS: c_uint = 0x00000f28	/* AXI Bus Mode */;
pub const XGMAC_DMA_AXI_STATUS: c_uint = 0x00000f2C	/* AXI Status */;
pub const XGMAC_DMA_HW_FEATURE: c_uint = 0x00000f58	/* Enabled Hardware Features */;
pub const XGMAC_ADDR_AE: c_uint = 0x80000000;
// PMT Control and Status
pub const XGMAC_PMT_POINTER_RESET: c_uint = 0x80000000;
pub const XGMAC_PMT_GLBL_UNICAST: c_uint = 0x00000200;
pub const XGMAC_PMT_WAKEUP_RX_FRM: c_uint = 0x00000040;
pub const XGMAC_PMT_MAGIC_PKT: c_uint = 0x00000020;
pub const XGMAC_PMT_WAKEUP_FRM_EN: c_uint = 0x00000004;
pub const XGMAC_PMT_MAGIC_PKT_EN: c_uint = 0x00000002;
pub const XGMAC_PMT_POWERDOWN: c_uint = 0x00000001;
pub const XGMAC_CONTROL_SPD: c_uint = 0x40000000	/* Speed control */;
pub const XGMAC_CONTROL_SPD_MASK: c_uint = 0x60000000;
pub const XGMAC_CONTROL_SPD_1G: c_uint = 0x60000000;
pub const XGMAC_CONTROL_SPD_2_5G: c_uint = 0x40000000;
pub const XGMAC_CONTROL_SPD_10G: c_uint = 0x00000000;
pub const XGMAC_CONTROL_SARC: c_uint = 0x10000000	/* Source Addr Insert/Replace */;
pub const XGMAC_CONTROL_SARK_MASK: c_uint = 0x18000000;
pub const XGMAC_CONTROL_CAR: c_uint = 0x04000000	/* CRC Addition/Replacement */;
pub const XGMAC_CONTROL_CAR_MASK: c_uint = 0x06000000;
pub const XGMAC_CONTROL_DP: c_uint = 0x01000000	/* Disable Padding */;
pub const XGMAC_CONTROL_WD: c_uint = 0x00800000	/* Disable Watchdog on rx */;
pub const XGMAC_CONTROL_JD: c_uint = 0x00400000	/* Jabber disable */;
pub const XGMAC_CONTROL_JE: c_uint = 0x00100000	/* Jumbo frame */;
pub const XGMAC_CONTROL_LM: c_uint = 0x00001000	/* Loop-back mode */;
pub const XGMAC_CONTROL_IPC: c_uint = 0x00000400	/* Checksum Offload */;
pub const XGMAC_CONTROL_ACS: c_uint = 0x00000080	/* Automatic Pad/FCS Strip */;
pub const XGMAC_CONTROL_DDIC: c_uint = 0x00000010	/* Disable Deficit Idle Count */;
pub const XGMAC_CONTROL_TE: c_uint = 0x00000008	/* Transmitter Enable */;
pub const XGMAC_CONTROL_RE: c_uint = 0x00000004	/* Receiver Enable */;
// XGMAC Frame Filter defines
pub const XGMAC_FRAME_FILTER_PR: c_uint = 0x00000001	/* Promiscuous Mode */;
pub const XGMAC_FRAME_FILTER_HUC: c_uint = 0x00000002	/* Hash Unicast */;
pub const XGMAC_FRAME_FILTER_HMC: c_uint = 0x00000004	/* Hash Multicast */;
pub const XGMAC_FRAME_FILTER_DAIF: c_uint = 0x00000008	/* DA Inverse Filtering */;
pub const XGMAC_FRAME_FILTER_PM: c_uint = 0x00000010	/* Pass all multicast */;
pub const XGMAC_FRAME_FILTER_DBF: c_uint = 0x00000020	/* Disable Broadcast frames */;
pub const XGMAC_FRAME_FILTER_SAIF: c_uint = 0x00000100	/* Inverse Filtering */;
pub const XGMAC_FRAME_FILTER_SAF: c_uint = 0x00000200	/* Source Address Filter */;
pub const XGMAC_FRAME_FILTER_HPF: c_uint = 0x00000400	/* Hash or perfect Filter */;
pub const XGMAC_FRAME_FILTER_VHF: c_uint = 0x00000800	/* VLAN Hash Filter */;
pub const XGMAC_FRAME_FILTER_VPF: c_uint = 0x00001000	/* VLAN Perfect Filter */;
pub const XGMAC_FRAME_FILTER_RA: c_uint = 0x80000000	/* Receive all mode */;
// XGMAC FLOW CTRL defines
pub const XGMAC_FLOW_CTRL_PT_MASK: c_uint = 0xffff0000	/* Pause Time Mask */;
pub const XGMAC_FLOW_CTRL_PT_SHIFT: c_int = 16;
pub const XGMAC_FLOW_CTRL_DZQP: c_uint = 0x00000080	/* Disable Zero-Quanta Phase */;
pub const XGMAC_FLOW_CTRL_PLT: c_uint = 0x00000020	/* Pause Low Threshold */;
pub const XGMAC_FLOW_CTRL_PLT_MASK: c_uint = 0x00000030	/* PLT MASK */;
pub const XGMAC_FLOW_CTRL_UP: c_uint = 0x00000008	/* Unicast Pause Frame Detect */;
pub const XGMAC_FLOW_CTRL_RFE: c_uint = 0x00000004	/* Rx Flow Control Enable */;
pub const XGMAC_FLOW_CTRL_TFE: c_uint = 0x00000002	/* Tx Flow Control Enable */;
pub const XGMAC_FLOW_CTRL_FCB_BPA: c_uint = 0x00000001	/* Flow Control Busy ... */;
// XGMAC_INT_STAT reg
pub const XGMAC_INT_STAT_PMTIM: c_uint = 0x00800000	/* PMT Interrupt Mask */;
pub const XGMAC_INT_STAT_PMT: c_uint = 0x0080		/* PMT Interrupt Status */;
pub const XGMAC_INT_STAT_LPI: c_uint = 0x0040		/* LPI Interrupt Status */;
// DMA Bus Mode register defines
pub const DMA_BUS_MODE_SFT_RESET: c_uint = 0x00000001	/* Software Reset */;
pub const DMA_BUS_MODE_DSL_MASK: c_uint = 0x0000007c	/* Descriptor Skip Length */;

pub const DMA_BUS_MODE_ATDS: c_uint = 0x00000080	/* Alternate Descriptor Size */;
// Programmable burst length
pub const DMA_BUS_MODE_PBL_MASK: c_uint = 0x00003f00	/* Programmable Burst Len */;
pub const DMA_BUS_MODE_PBL_SHIFT: c_int = 8;
pub const DMA_BUS_MODE_FB: c_uint = 0x00010000	/* Fixed burst */;
pub const DMA_BUS_MODE_RPBL_MASK: c_uint = 0x003e0000	/* Rx-Programmable Burst Len */;
pub const DMA_BUS_MODE_RPBL_SHIFT: c_int = 17;
pub const DMA_BUS_MODE_USP: c_uint = 0x00800000;
pub const DMA_BUS_MODE_8PBL: c_uint = 0x01000000;
pub const DMA_BUS_MODE_AAL: c_uint = 0x02000000;
// DMA Bus Mode register defines
pub const DMA_BUS_PR_RATIO_MASK: c_uint = 0x0000c000	/* Rx/Tx priority ratio */;
pub const DMA_BUS_PR_RATIO_SHIFT: c_int = 14;
pub const DMA_BUS_FB: c_uint = 0x00010000	/* Fixed Burst */;
// DMA Control register defines
pub const DMA_CONTROL_ST: c_uint = 0x00002000	/* Start/Stop Transmission */;
pub const DMA_CONTROL_SR: c_uint = 0x00000002	/* Start/Stop Receive */;
pub const DMA_CONTROL_DFF: c_uint = 0x01000000	/* Disable flush of rx frames */;
pub const DMA_CONTROL_OSF: c_uint = 0x00000004	/* Operate on 2nd tx frame */;
// DMA Normal interrupt
pub const DMA_INTR_ENA_NIE: c_uint = 0x00010000	/* Normal Summary */;
pub const DMA_INTR_ENA_AIE: c_uint = 0x00008000	/* Abnormal Summary */;
pub const DMA_INTR_ENA_ERE: c_uint = 0x00004000	/* Early Receive */;
pub const DMA_INTR_ENA_FBE: c_uint = 0x00002000	/* Fatal Bus Error */;
pub const DMA_INTR_ENA_ETE: c_uint = 0x00000400	/* Early Transmit */;
pub const DMA_INTR_ENA_RWE: c_uint = 0x00000200	/* Receive Watchdog */;
pub const DMA_INTR_ENA_RSE: c_uint = 0x00000100	/* Receive Stopped */;
pub const DMA_INTR_ENA_RUE: c_uint = 0x00000080	/* Receive Buffer Unavailable */;
pub const DMA_INTR_ENA_RIE: c_uint = 0x00000040	/* Receive Interrupt */;
pub const DMA_INTR_ENA_UNE: c_uint = 0x00000020	/* Tx Underflow */;
pub const DMA_INTR_ENA_OVE: c_uint = 0x00000010	/* Receive Overflow */;
pub const DMA_INTR_ENA_TJE: c_uint = 0x00000008	/* Transmit Jabber */;
pub const DMA_INTR_ENA_TUE: c_uint = 0x00000004	/* Transmit Buffer Unavail */;
pub const DMA_INTR_ENA_TSE: c_uint = 0x00000002	/* Transmit Stopped */;
pub const DMA_INTR_ENA_TIE: c_uint = 0x00000001	/* Transmit Interrupt */;

    DMA_INTR_ENA_TUE | DMA_INTR_ENA_TIE)

    DMA_INTR_ENA_RWE | DMA_INTR_ENA_RSE | \
    DMA_INTR_ENA_RUE | DMA_INTR_ENA_UNE | \
    DMA_INTR_ENA_OVE | DMA_INTR_ENA_TJE | \
    DMA_INTR_ENA_TSE)
// DMA default interrupt mask

// DMA Status register defines
pub const DMA_STATUS_GMI: c_uint = 0x08000000	/* MMC interrupt */;
pub const DMA_STATUS_GLI: c_uint = 0x04000000	/* GMAC Line interface int */;
pub const DMA_STATUS_EB_MASK: c_uint = 0x00380000	/* Error Bits Mask */;
pub const DMA_STATUS_EB_TX_ABORT: c_uint = 0x00080000	/* Error Bits - TX Abort */;
pub const DMA_STATUS_EB_RX_ABORT: c_uint = 0x00100000	/* Error Bits - RX Abort */;
pub const DMA_STATUS_TS_MASK: c_uint = 0x00700000	/* Transmit Process State */;
pub const DMA_STATUS_TS_SHIFT: c_int = 20;
pub const DMA_STATUS_RS_MASK: c_uint = 0x000e0000	/* Receive Process State */;
pub const DMA_STATUS_RS_SHIFT: c_int = 17;
pub const DMA_STATUS_NIS: c_uint = 0x00010000	/* Normal Interrupt Summary */;
pub const DMA_STATUS_AIS: c_uint = 0x00008000	/* Abnormal Interrupt Summary */;
pub const DMA_STATUS_ERI: c_uint = 0x00004000	/* Early Receive Interrupt */;
pub const DMA_STATUS_FBI: c_uint = 0x00002000	/* Fatal Bus Error Interrupt */;
pub const DMA_STATUS_ETI: c_uint = 0x00000400	/* Early Transmit Interrupt */;
pub const DMA_STATUS_RWT: c_uint = 0x00000200	/* Receive Watchdog Timeout */;
pub const DMA_STATUS_RPS: c_uint = 0x00000100	/* Receive Process Stopped */;
pub const DMA_STATUS_RU: c_uint = 0x00000080	/* Receive Buffer Unavailable */;
pub const DMA_STATUS_RI: c_uint = 0x00000040	/* Receive Interrupt */;
pub const DMA_STATUS_UNF: c_uint = 0x00000020	/* Transmit Underflow */;
pub const DMA_STATUS_OVF: c_uint = 0x00000010	/* Receive Overflow */;
pub const DMA_STATUS_TJT: c_uint = 0x00000008	/* Transmit Jabber Timeout */;
pub const DMA_STATUS_TU: c_uint = 0x00000004	/* Transmit Buffer Unavail */;
pub const DMA_STATUS_TPS: c_uint = 0x00000002	/* Transmit Process Stopped */;
pub const DMA_STATUS_TI: c_uint = 0x00000001	/* Transmit Interrupt */;
// Common MAC defines
pub const MAC_ENABLE_TX: c_uint = 0x00000008	/* Transmitter Enable */;
pub const MAC_ENABLE_RX: c_uint = 0x00000004	/* Receiver Enable */;
// XGMAC Operation Mode Register
pub const XGMAC_OMR_TSF: c_uint = 0x00200000	/* TX FIFO Store and Forward */;
pub const XGMAC_OMR_FTF: c_uint = 0x00100000	/* Flush Transmit FIFO */;
pub const XGMAC_OMR_TTC: c_uint = 0x00020000	/* Transmit Threshold Ctrl */;
pub const XGMAC_OMR_TTC_MASK: c_uint = 0x00030000;
pub const XGMAC_OMR_RFD: c_uint = 0x00006000	/* FC Deactivation Threshold */;
pub const XGMAC_OMR_RFD_MASK: c_uint = 0x00007000	/* FC Deact Threshold MASK */;
pub const XGMAC_OMR_RFA: c_uint = 0x00000600	/* FC Activation Threshold */;
pub const XGMAC_OMR_RFA_MASK: c_uint = 0x00000E00	/* FC Act Threshold MASK */;
pub const XGMAC_OMR_EFC: c_uint = 0x00000100	/* Enable Hardware FC */;
pub const XGMAC_OMR_FEF: c_uint = 0x00000080	/* Forward Error Frames */;
pub const XGMAC_OMR_DT: c_uint = 0x00000040	/* Drop TCP/IP csum Errors */;
pub const XGMAC_OMR_RSF: c_uint = 0x00000020	/* RX FIFO Store and Forward */;
pub const XGMAC_OMR_RTC_256: c_uint = 0x00000018	/* RX Threshold Ctrl */;
pub const XGMAC_OMR_RTC_MASK: c_uint = 0x00000018	/* RX Threshold Ctrl MASK */;
// XGMAC HW Features Register
pub const DMA_HW_FEAT_TXCOESEL: c_uint = 0x00010000	/* TX Checksum offload */;
pub const XGMAC_MMC_CTRL_CNT_FRZ: c_uint = 0x00000008;
// XGMAC Descriptor Defines

pub const RXDESC_EXT_STATUS: c_uint = 0x00000001;
pub const RXDESC_CRC_ERR: c_uint = 0x00000002;
pub const RXDESC_RX_ERR: c_uint = 0x00000008;
pub const RXDESC_RX_WDOG: c_uint = 0x00000010;
pub const RXDESC_FRAME_TYPE: c_uint = 0x00000020;
pub const RXDESC_GIANT_FRAME: c_uint = 0x00000080;
pub const RXDESC_LAST_SEG: c_uint = 0x00000100;
pub const RXDESC_FIRST_SEG: c_uint = 0x00000200;
pub const RXDESC_VLAN_FRAME: c_uint = 0x00000400;
pub const RXDESC_OVERFLOW_ERR: c_uint = 0x00000800;
pub const RXDESC_LENGTH_ERR: c_uint = 0x00001000;
pub const RXDESC_SA_FILTER_FAIL: c_uint = 0x00002000;
pub const RXDESC_DESCRIPTOR_ERR: c_uint = 0x00004000;
pub const RXDESC_ERROR_SUMMARY: c_uint = 0x00008000;
pub const RXDESC_FRAME_LEN_OFFSET: c_int = 16;
pub const RXDESC_FRAME_LEN_MASK: c_uint = 0x3fff0000;
pub const RXDESC_DA_FILTER_FAIL: c_uint = 0x40000000;
pub const RXDESC1_END_RING: c_uint = 0x00008000;
pub const RXDESC_IP_PAYLOAD_MASK: c_uint = 0x00000003;
pub const RXDESC_IP_PAYLOAD_UDP: c_uint = 0x00000001;
pub const RXDESC_IP_PAYLOAD_TCP: c_uint = 0x00000002;
pub const RXDESC_IP_PAYLOAD_ICMP: c_uint = 0x00000003;
pub const RXDESC_IP_HEADER_ERR: c_uint = 0x00000008;
pub const RXDESC_IP_PAYLOAD_ERR: c_uint = 0x00000010;
pub const RXDESC_IPV4_PACKET: c_uint = 0x00000040;
pub const RXDESC_IPV6_PACKET: c_uint = 0x00000080;
pub const TXDESC_UNDERFLOW_ERR: c_uint = 0x00000001;
pub const TXDESC_JABBER_TIMEOUT: c_uint = 0x00000002;
pub const TXDESC_LOCAL_FAULT: c_uint = 0x00000004;
pub const TXDESC_REMOTE_FAULT: c_uint = 0x00000008;
pub const TXDESC_VLAN_FRAME: c_uint = 0x00000010;
pub const TXDESC_FRAME_FLUSHED: c_uint = 0x00000020;
pub const TXDESC_IP_HEADER_ERR: c_uint = 0x00000040;
pub const TXDESC_PAYLOAD_CSUM_ERR: c_uint = 0x00000080;
pub const TXDESC_ERROR_SUMMARY: c_uint = 0x00008000;
pub const TXDESC_SA_CTRL_INSERT: c_uint = 0x00040000;
pub const TXDESC_SA_CTRL_REPLACE: c_uint = 0x00080000;
pub const TXDESC_2ND_ADDR_CHAINED: c_uint = 0x00100000;
pub const TXDESC_END_RING: c_uint = 0x00200000;
pub const TXDESC_CSUM_IP: c_uint = 0x00400000;
pub const TXDESC_CSUM_IP_PAYLD: c_uint = 0x00800000;
pub const TXDESC_CSUM_ALL: c_uint = 0x00C00000;
pub const TXDESC_CRC_EN_REPLACE: c_uint = 0x01000000;
pub const TXDESC_CRC_EN_APPEND: c_uint = 0x02000000;
pub const TXDESC_DISABLE_PAD: c_uint = 0x04000000;
pub const TXDESC_FIRST_SEG: c_uint = 0x10000000;
pub const TXDESC_LAST_SEG: c_uint = 0x20000000;
pub const TXDESC_INTERRUPT: c_uint = 0x40000000;
pub const DESC_OWN: c_uint = 0x80000000;
pub const DESC_BUFFER1_SZ_MASK: c_uint = 0x00001fff;
pub const DESC_BUFFER2_SZ_MASK: c_uint = 0x1fff0000;
pub const DESC_BUFFER2_SZ_OFFSET: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgmac_dma_desc {
    pub flags: __le32,
    pub buf_size: __le32,
    pub /: *mut *mut __le32 buf1_addr; / Buffer 1 Address Pointer,
    pub /: *mut *mut __le32 buf2_addr; / Buffer 2 Address Pointer,
    pub ext_status: __le32,
    pub res: [__le32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgmac_extra_stats {
// Transmit errors
    pub tx_jabber: c_ulong,
    pub tx_frame_flushed: c_ulong,
    pub tx_payload_error: c_ulong,
    pub tx_ip_header_error: c_ulong,
    pub tx_local_fault: c_ulong,
    pub tx_remote_fault: c_ulong,
// Receive errors
    pub rx_watchdog: c_ulong,
    pub rx_da_filter_fail: c_ulong,
    pub rx_payload_error: c_ulong,
    pub rx_ip_header_error: c_ulong,
// Tx/Rx IRQ errors
    pub tx_process_stopped: c_ulong,
    pub rx_buf_unav: c_ulong,
    pub rx_process_stopped: c_ulong,
    pub tx_early: c_ulong,
    pub fatal_bus_error: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgmac_priv {
    pub dma_rx: *mut xgmac_dma_desc,
    pub rx_skbuff: *mut sk_buff,
    pub rx_tail: c_uint,
    pub rx_head: c_uint,
    pub dma_tx: *mut xgmac_dma_desc,
    pub tx_skbuff: *mut sk_buff,
    pub tx_head: c_uint,
    pub tx_tail: c_uint,
    pub tx_irq_cnt: c_int,
    pub base: *mut void __iomem,
    pub dma_buf_sz: c_uint,
    pub dma_rx_phy: dma_addr_t,
    pub dma_tx_phy: dma_addr_t,
    pub dev: *mut net_device,
    pub device: *mut device,
    pub napi: napi_struct,
    pub max_macs: c_int,
    pub xstats: xgmac_extra_stats,
    pub stats_lock: spinlock_t,
    pub pmt_irq: c_int,
    pub rx_pause: c_char,
    pub tx_pause: c_char,
    pub wolopts: c_int,
    pub tx_timeout_work: work_struct,
}

// XGMAC Configuration Settings
pub const XGMAC_MAX_MTU: c_int = 9000;
pub const PAUSE_TIME: c_uint = 0x400;
pub const DMA_RX_RING_SZ: c_int = 256;
pub const DMA_TX_RING_SZ: c_int = 128;
// minimum number of free TX descriptors required to wake up TX process

// DMA descriptor ring helpers

    dma_ring_space((p).tx_head, (p).tx_tail, DMA_TX_RING_SZ)
// XGMAC Descriptor Access Helpers
#[no_mangle]
pub unsafe extern "C" fn desc_set_buf_len(p: *mut xgmac_dma_desc, buf_sz: u32) {
    static inline void desc_set_buf_len(struct xgmac_dma_desc *p, u32 buf_sz)
    {
    if (buf_sz > MAX_DESC_BUF_SZ)
    p.buf_size = cpu_to_le32(MAX_DESC_BUF_SZ |
    (buf_sz - MAX_DESC_BUF_SZ) << DESC_BUFFER2_SZ_OFFSET);
    else
    p.buf_size = cpu_to_le32(buf_sz);
    }
#[no_mangle]
pub unsafe extern "C" fn desc_get_buf_len(p: *mut xgmac_dma_desc) -> c_int {
    static inline int desc_get_buf_len(struct xgmac_dma_desc *p)
    {
    let mut len: u32 = le32_to_cpu(p.buf_size);
    return (len & DESC_BUFFER1_SZ_MASK) +
    ((len & DESC_BUFFER2_SZ_MASK) >> DESC_BUFFER2_SZ_OFFSET);
    }
    static inline void desc_init_rx_desc(struct xgmac_dma_desc *p, int ring_size,
    int buf_sz)
    {
    struct xgmac_dma_desc *end = p + ring_size - 1;
    memset(p, 0, sizeof(*p) * ring_size);
    for (; p <= end; p++)
    desc_set_buf_len(p, buf_sz);
    end.buf_size |= cpu_to_le32(RXDESC1_END_RING);
    }
#[no_mangle]
pub unsafe extern "C" fn desc_init_tx_desc(p: *mut xgmac_dma_desc, ring_size: u32) {
    static inline void desc_init_tx_desc(struct xgmac_dma_desc *p, u32 ring_size)
    {
    memset(p, 0, sizeof(*p) * ring_size);
    p[ring_size - 1].flags = cpu_to_le32(TXDESC_END_RING);
    }
#[no_mangle]
pub unsafe extern "C" fn desc_get_owner(p: *mut xgmac_dma_desc) -> c_int {
    static inline int desc_get_owner(struct xgmac_dma_desc *p)
    {
    return le32_to_cpu(p.flags) & DESC_OWN;
    }
#[no_mangle]
pub unsafe extern "C" fn desc_set_rx_owner(p: *mut xgmac_dma_desc) {
    static inline void desc_set_rx_owner(struct xgmac_dma_desc *p)
    {
// Clear all fields and set the owner
    p.flags = cpu_to_le32(DESC_OWN);
    }
#[no_mangle]
pub unsafe extern "C" fn desc_set_tx_owner(p: *mut xgmac_dma_desc, flags: u32) {
    static inline void desc_set_tx_owner(struct xgmac_dma_desc *p, u32 flags)
    {
    let mut tmpflags: u32 = le32_to_cpu(p.flags);
    tmpflags &= TXDESC_END_RING;
    tmpflags |= flags | DESC_OWN;
    p.flags = cpu_to_le32(tmpflags);
    }
#[no_mangle]
pub unsafe extern "C" fn desc_clear_tx_owner(p: *mut xgmac_dma_desc) {
    static inline void desc_clear_tx_owner(struct xgmac_dma_desc *p)
    {
    let mut tmpflags: u32 = le32_to_cpu(p.flags);
    tmpflags &= TXDESC_END_RING;
    p.flags = cpu_to_le32(tmpflags);
    }
#[no_mangle]
pub unsafe extern "C" fn desc_get_tx_ls(p: *mut xgmac_dma_desc) -> c_int {
    static inline int desc_get_tx_ls(struct xgmac_dma_desc *p)
    {
    return le32_to_cpu(p.flags) & TXDESC_LAST_SEG;
    }
#[no_mangle]
pub unsafe extern "C" fn desc_get_tx_fs(p: *mut xgmac_dma_desc) -> c_int {
    static inline int desc_get_tx_fs(struct xgmac_dma_desc *p)
    {
    return le32_to_cpu(p.flags) & TXDESC_FIRST_SEG;
    }
#[no_mangle]
pub unsafe extern "C" fn desc_get_buf_addr(p: *mut xgmac_dma_desc) -> u32 {
    static inline u32 desc_get_buf_addr(struct xgmac_dma_desc *p)
    {
    return le32_to_cpu(p.buf1_addr);
    }
    static inline void desc_set_buf_addr(struct xgmac_dma_desc *p,
    u32 paddr, int len)
    {
    p.buf1_addr = cpu_to_le32(paddr);
    if (len > MAX_DESC_BUF_SZ)
    p.buf2_addr = cpu_to_le32(paddr + MAX_DESC_BUF_SZ);
    }
    static inline void desc_set_buf_addr_and_size(struct xgmac_dma_desc *p,
    u32 paddr, int len)
    {
    desc_set_buf_len(p, len);
    desc_set_buf_addr(p, paddr, len);
    }
#[no_mangle]
pub unsafe extern "C" fn desc_get_rx_frame_len(p: *mut xgmac_dma_desc) -> c_int {
    static inline int desc_get_rx_frame_len(struct xgmac_dma_desc *p)
    {
    let mut data: u32 = le32_to_cpu(p.flags);
    let mut len: u32 = (data & RXDESC_FRAME_LEN_MASK) >> RXDESC_FRAME_LEN_OFFSET;
    if (data & RXDESC_FRAME_TYPE)
    len -= ETH_FCS_LEN;
    return len;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_dma_flush_tx_fifo(ioaddr: *mut void __iomem) {
    static void xgmac_dma_flush_tx_fifo(void __iomem *ioaddr)
    {
    let mut timeout: c_int = 1000;
    let mut reg: u32 = readl(ioaddr + XGMAC_OMR);
    writel(reg | XGMAC_OMR_FTF, ioaddr + XGMAC_OMR);
    while ((timeout-- > 0) && readl(ioaddr + XGMAC_OMR) & XGMAC_OMR_FTF)
    udelay(1);
    }
#[no_mangle]
unsafe extern "C" fn desc_get_tx_status(priv: *mut xgmac_priv, p: *mut xgmac_dma_desc) -> c_int {
    static int desc_get_tx_status(struct xgmac_priv *priv, struct xgmac_dma_desc *p)
    {
    struct xgmac_extra_stats *x = &priv.xstats;
    let mut status: u32 = le32_to_cpu(p.flags);
    if (!(status & TXDESC_ERROR_SUMMARY))
    return 0;
    netdev_dbg(priv.dev, "tx desc error = 0x%08x\n", status);
    if (status & TXDESC_JABBER_TIMEOUT)
    x.tx_jabber++;
    if (status & TXDESC_FRAME_FLUSHED)
    x.tx_frame_flushed++;
    if (status & TXDESC_UNDERFLOW_ERR)
    xgmac_dma_flush_tx_fifo(priv.base);
    if (status & TXDESC_IP_HEADER_ERR)
    x.tx_ip_header_error++;
    if (status & TXDESC_LOCAL_FAULT)
    x.tx_local_fault++;
    if (status & TXDESC_REMOTE_FAULT)
    x.tx_remote_fault++;
    if (status & TXDESC_PAYLOAD_CSUM_ERR)
    x.tx_payload_error++;
    return -1;
    }
#[no_mangle]
unsafe extern "C" fn desc_get_rx_status(priv: *mut xgmac_priv, p: *mut xgmac_dma_desc) -> c_int {
    static int desc_get_rx_status(struct xgmac_priv *priv, struct xgmac_dma_desc *p)
    {
    struct xgmac_extra_stats *x = &priv.xstats;
    let mut ret: c_int = CHECKSUM_UNNECESSARY;
    let mut status: u32 = le32_to_cpu(p.flags);
    let mut ext_status: u32 = le32_to_cpu(p.ext_status);
    if (status & RXDESC_DA_FILTER_FAIL) {
    netdev_dbg(priv.dev, "XGMAC RX : Dest Address filter fail\n");
    x.rx_da_filter_fail++;
    return -1;
    }
// All frames should fit into a single buffer
    if (!(status & RXDESC_FIRST_SEG) || !(status & RXDESC_LAST_SEG))
    return -1;
// Check if packet has checksum already
    if ((status & RXDESC_FRAME_TYPE) && (status & RXDESC_EXT_STATUS) &&
    !(ext_status & RXDESC_IP_PAYLOAD_MASK))
    ret = CHECKSUM_NONE;
    netdev_dbg(priv.dev, "rx status - frame type=%d, csum = %d, ext stat %08x\n",
    (status & RXDESC_FRAME_TYPE) ? 1 : 0, ret, ext_status);
    if (!(status & RXDESC_ERROR_SUMMARY))
    return ret;
// Handle any errors
    if (status & (RXDESC_DESCRIPTOR_ERR | RXDESC_OVERFLOW_ERR |
    RXDESC_GIANT_FRAME | RXDESC_LENGTH_ERR | RXDESC_CRC_ERR))
    return -1;
    if (status & RXDESC_EXT_STATUS) {
    if (ext_status & RXDESC_IP_HEADER_ERR)
    x.rx_ip_header_error++;
    if (ext_status & RXDESC_IP_PAYLOAD_ERR)
    x.rx_payload_error++;
    netdev_dbg(priv.dev, "IP checksum error - stat %08x\n",
    ext_status);
    return CHECKSUM_NONE;
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn xgmac_mac_enable(ioaddr: *mut void __iomem) {
    static inline void xgmac_mac_enable(void __iomem *ioaddr)
    {
    let mut value: u32 = readl(ioaddr + XGMAC_CONTROL);
    value |= MAC_ENABLE_RX | MAC_ENABLE_TX;
    writel(value, ioaddr + XGMAC_CONTROL);
    value = readl(ioaddr + XGMAC_DMA_CONTROL);
    value |= DMA_CONTROL_ST | DMA_CONTROL_SR;
    writel(value, ioaddr + XGMAC_DMA_CONTROL);
    }
#[no_mangle]
pub unsafe extern "C" fn xgmac_mac_disable(ioaddr: *mut void __iomem) {
    static inline void xgmac_mac_disable(void __iomem *ioaddr)
    {
    let mut value: u32 = readl(ioaddr + XGMAC_DMA_CONTROL);
    value &= ~(DMA_CONTROL_ST | DMA_CONTROL_SR);
    writel(value, ioaddr + XGMAC_DMA_CONTROL);
    value = readl(ioaddr + XGMAC_CONTROL);
    value &= ~(MAC_ENABLE_TX | MAC_ENABLE_RX);
    writel(value, ioaddr + XGMAC_CONTROL);
    }
    static void xgmac_set_mac_addr(void __iomem *ioaddr, const unsigned char *addr,
    int num)
    {
    u32 data;
    if (addr) {
    data = (addr[5] << 8) | addr[4] | (num ? XGMAC_ADDR_AE : 0);
    writel(data, ioaddr + XGMAC_ADDR_HIGH(num));
    data = (addr[3] << 24) | (addr[2] << 16) | (addr[1] << 8) | addr[0];
    writel(data, ioaddr + XGMAC_ADDR_LOW(num));
    } else {
    writel(0, ioaddr + XGMAC_ADDR_HIGH(num));
    writel(0, ioaddr + XGMAC_ADDR_LOW(num));
    }
    }
    static void xgmac_get_mac_addr(void __iomem *ioaddr, unsigned char *addr,
    int num)
    {
    u32 hi_addr, lo_addr;
// Read the MAC address from the hardware
    hi_addr = readl(ioaddr + XGMAC_ADDR_HIGH(num));
    lo_addr = readl(ioaddr + XGMAC_ADDR_LOW(num));
// Extract the MAC address from the high and low words
    addr[0] = lo_addr & 0xff;
    addr[1] = (lo_addr >> 8) & 0xff;
    addr[2] = (lo_addr >> 16) & 0xff;
    addr[3] = (lo_addr >> 24) & 0xff;
    addr[4] = hi_addr & 0xff;
    addr[5] = (hi_addr >> 8) & 0xff;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_set_flow_ctrl(priv: *mut xgmac_priv, rx: c_int, tx: c_int) -> c_int {
    static int xgmac_set_flow_ctrl(struct xgmac_priv *priv, int rx, int tx)
    {
    u32 reg;
    let mut flow: c_uint = 0;
    priv.rx_pause = rx;
    priv.tx_pause = tx;
    if (rx || tx) {
    if (rx)
    flow |= XGMAC_FLOW_CTRL_RFE;
    if (tx)
    flow |= XGMAC_FLOW_CTRL_TFE;
    flow |= XGMAC_FLOW_CTRL_PLT | XGMAC_FLOW_CTRL_UP;
    flow |= (PAUSE_TIME << XGMAC_FLOW_CTRL_PT_SHIFT);
    writel(flow, priv.base + XGMAC_FLOW_CTRL);
    reg = readl(priv.base + XGMAC_OMR);
    reg |= XGMAC_OMR_EFC;
    writel(reg, priv.base + XGMAC_OMR);
    } else {
    writel(0, priv.base + XGMAC_FLOW_CTRL);
    reg = readl(priv.base + XGMAC_OMR);
    reg &= ~XGMAC_OMR_EFC;
    writel(reg, priv.base + XGMAC_OMR);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_rx_refill(priv: *mut xgmac_priv) {
    static void xgmac_rx_refill(struct xgmac_priv *priv)
    {
    struct xgmac_dma_desc *p;
    dma_addr_t paddr;
    let mut bufsz: c_int = priv.dev.mtu + ETH_HLEN + ETH_FCS_LEN;
    while (dma_ring_space(priv.rx_head, priv.rx_tail, DMA_RX_RING_SZ) > 1) {
    let mut entry: c_int = priv.rx_head;
    struct sk_buff *skb;
    p = priv.dma_rx + entry;
    if (priv.rx_skbuff[entry] == core::ptr::null_mut()) {
    skb = netdev_alloc_skb_ip_align(priv.dev, bufsz);
    if (unlikely(skb == core::ptr::null_mut()))
    break;
    paddr = dma_map_single(priv.device, skb.data,
    priv.dma_buf_sz - NET_IP_ALIGN,
    DMA_FROM_DEVICE);
    if (dma_mapping_error(priv.device, paddr)) {
    dev_kfree_skb_any(skb);
    break;
    }
    priv.rx_skbuff[entry] = skb;
    desc_set_buf_addr(p, paddr, priv.dma_buf_sz);
    }
    netdev_dbg(priv.dev, "rx ring: head %d, tail %d\n",
    priv.rx_head, priv.rx_tail);
    priv.rx_head = dma_ring_incr(priv.rx_head, DMA_RX_RING_SZ);
    desc_set_rx_owner(p);
    }
    }
//
// xgmac_dma_desc_rings_init - init the RX/TX descriptor rings
// @dev: net device structure
// Description:  this function initializes the DMA RX/TX descriptors
// and allocates the socket buffers.
//
#[no_mangle]
unsafe extern "C" fn xgmac_dma_desc_rings_init(dev: *mut net_device) -> c_int {
    static int xgmac_dma_desc_rings_init(struct net_device *dev)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    unsigned int bfsize;
// Set the Buffer size according to the MTU;
// The total buffer size including any IP offset must be a multiple
// of 8 bytes.
//
    bfsize = ALIGN(dev.mtu + ETH_HLEN + ETH_FCS_LEN + NET_IP_ALIGN, 8);
    netdev_dbg(priv.dev, "mtu [%d] bfsize [%d]\n", dev.mtu, bfsize);
    priv.rx_skbuff = kzalloc_objs(struct sk_buff *, DMA_RX_RING_SZ);
    if (!priv.rx_skbuff)
    return -ENOMEM;
    priv.dma_rx = dma_alloc_coherent(priv.device,
    DMA_RX_RING_SZ *
    sizeof(struct xgmac_dma_desc),
    &priv.dma_rx_phy,
    GFP_KERNEL);
    if (!priv.dma_rx)
    goto err_dma_rx;
    priv.tx_skbuff = kzalloc_objs(struct sk_buff *, DMA_TX_RING_SZ);
    if (!priv.tx_skbuff)
    goto err_tx_skb;
    priv.dma_tx = dma_alloc_coherent(priv.device,
    DMA_TX_RING_SZ *
    sizeof(struct xgmac_dma_desc),
    &priv.dma_tx_phy,
    GFP_KERNEL);
    if (!priv.dma_tx)
    goto err_dma_tx;
    netdev_dbg(priv.dev, "DMA desc rings: virt addr (Rx %p, "
    "Tx %p)\n\tDMA phy addr (Rx 0x%08x, Tx 0x%08x)\n",
    priv.dma_rx, priv.dma_tx,
    (unsigned int)priv.dma_rx_phy, (unsigned int)priv.dma_tx_phy);
    priv.rx_tail = 0;
    priv.rx_head = 0;
    priv.dma_buf_sz = bfsize;
    desc_init_rx_desc(priv.dma_rx, DMA_RX_RING_SZ, priv.dma_buf_sz);
    xgmac_rx_refill(priv);
    priv.tx_tail = 0;
    priv.tx_head = 0;
    desc_init_tx_desc(priv.dma_tx, DMA_TX_RING_SZ);
    writel(priv.dma_tx_phy, priv.base + XGMAC_DMA_TX_BASE_ADDR);
    writel(priv.dma_rx_phy, priv.base + XGMAC_DMA_RX_BASE_ADDR);
    return 0;
    err_dma_tx:
    kfree(priv.tx_skbuff);
    err_tx_skb:
    dma_free_coherent(priv.device,
    DMA_RX_RING_SZ * sizeof(struct xgmac_dma_desc),
    priv.dma_rx, priv.dma_rx_phy);
    err_dma_rx:
    kfree(priv.rx_skbuff);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_free_rx_skbufs(priv: *mut xgmac_priv) {
    static void xgmac_free_rx_skbufs(struct xgmac_priv *priv)
    {
    int i;
    struct xgmac_dma_desc *p;
    if (!priv.rx_skbuff)
    return;
    for (i = 0; i < DMA_RX_RING_SZ; i++) {
    struct sk_buff *skb = priv.rx_skbuff[i];
    if (skb == core::ptr::null_mut())
    continue;
    p = priv.dma_rx + i;
    dma_unmap_single(priv.device, desc_get_buf_addr(p),
    priv.dma_buf_sz - NET_IP_ALIGN, DMA_FROM_DEVICE);
    dev_kfree_skb_any(skb);
    priv.rx_skbuff[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn xgmac_free_tx_skbufs(priv: *mut xgmac_priv) {
    static void xgmac_free_tx_skbufs(struct xgmac_priv *priv)
    {
    int i;
    struct xgmac_dma_desc *p;
    if (!priv.tx_skbuff)
    return;
    for (i = 0; i < DMA_TX_RING_SZ; i++) {
    if (priv.tx_skbuff[i] == core::ptr::null_mut())
    continue;
    p = priv.dma_tx + i;
    if (desc_get_tx_fs(p))
    dma_unmap_single(priv.device, desc_get_buf_addr(p),
    desc_get_buf_len(p), DMA_TO_DEVICE);
    else
    dma_unmap_page(priv.device, desc_get_buf_addr(p),
    desc_get_buf_len(p), DMA_TO_DEVICE);
    if (desc_get_tx_ls(p))
    dev_kfree_skb_any(priv.tx_skbuff[i]);
    priv.tx_skbuff[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn xgmac_free_dma_desc_rings(priv: *mut xgmac_priv) {
    static void xgmac_free_dma_desc_rings(struct xgmac_priv *priv)
    {
// Release the DMA TX/RX socket buffers
    xgmac_free_rx_skbufs(priv);
    xgmac_free_tx_skbufs(priv);
// Free the consistent memory allocated for descriptor rings
    if (priv.dma_tx) {
    dma_free_coherent(priv.device,
    DMA_TX_RING_SZ * sizeof(struct xgmac_dma_desc),
    priv.dma_tx, priv.dma_tx_phy);
    priv.dma_tx = core::ptr::null_mut();
    }
    if (priv.dma_rx) {
    dma_free_coherent(priv.device,
    DMA_RX_RING_SZ * sizeof(struct xgmac_dma_desc),
    priv.dma_rx, priv.dma_rx_phy);
    priv.dma_rx = core::ptr::null_mut();
    }
    kfree(priv.rx_skbuff);
    priv.rx_skbuff = core::ptr::null_mut();
    kfree(priv.tx_skbuff);
    priv.tx_skbuff = core::ptr::null_mut();
    }
//
// xgmac_tx_complete:
// @priv: private driver structure
// Description: it reclaims resources after transmission completes.
//
#[no_mangle]
unsafe extern "C" fn xgmac_tx_complete(priv: *mut xgmac_priv) {
    static void xgmac_tx_complete(struct xgmac_priv *priv)
    {
    while (dma_ring_cnt(priv.tx_head, priv.tx_tail, DMA_TX_RING_SZ)) {
    let mut entry: c_uint = priv.tx_tail;
    struct sk_buff *skb = priv.tx_skbuff[entry];
    struct xgmac_dma_desc *p = priv.dma_tx + entry;
// Check if the descriptor is owned by the DMA.
    if (desc_get_owner(p))
    break;
    netdev_dbg(priv.dev, "tx ring: curr %d, dirty %d\n",
    priv.tx_head, priv.tx_tail);
    if (desc_get_tx_fs(p))
    dma_unmap_single(priv.device, desc_get_buf_addr(p),
    desc_get_buf_len(p), DMA_TO_DEVICE);
    else
    dma_unmap_page(priv.device, desc_get_buf_addr(p),
    desc_get_buf_len(p), DMA_TO_DEVICE);
// Check tx error on the last segment
    if (desc_get_tx_ls(p)) {
    desc_get_tx_status(priv, p);
    dev_consume_skb_any(skb);
    }
    priv.tx_skbuff[entry] = core::ptr::null_mut();
    priv.tx_tail = dma_ring_incr(entry, DMA_TX_RING_SZ);
    }
// Ensure tx_tail is visible to xgmac_xmit
    smp_mb();
    if (unlikely(netif_queue_stopped(priv.dev) &&
    (tx_dma_ring_space(priv) > MAX_SKB_FRAGS)))
    netif_wake_queue(priv.dev);
    }
#[no_mangle]
unsafe extern "C" fn xgmac_tx_timeout_work(work: *mut work_struct) {
    static void xgmac_tx_timeout_work(struct work_struct *work)
    {
    u32 reg, value;
    struct xgmac_priv *priv =
    container_of(work, struct xgmac_priv, tx_timeout_work);
    napi_disable(&priv.napi);
    writel(0, priv.base + XGMAC_DMA_INTR_ENA);
    netif_tx_lock(priv.dev);
    reg = readl(priv.base + XGMAC_DMA_CONTROL);
    writel(reg & ~DMA_CONTROL_ST, priv.base + XGMAC_DMA_CONTROL);
    do {
    value = readl(priv.base + XGMAC_DMA_STATUS) & 0x700000;
    } while (value && (value != 0x600000));
    xgmac_free_tx_skbufs(priv);
    desc_init_tx_desc(priv.dma_tx, DMA_TX_RING_SZ);
    priv.tx_tail = 0;
    priv.tx_head = 0;
    writel(priv.dma_tx_phy, priv.base + XGMAC_DMA_TX_BASE_ADDR);
    writel(reg | DMA_CONTROL_ST, priv.base + XGMAC_DMA_CONTROL);
    writel(DMA_STATUS_TU | DMA_STATUS_TPS | DMA_STATUS_NIS | DMA_STATUS_AIS,
    priv.base + XGMAC_DMA_STATUS);
    netif_tx_unlock(priv.dev);
    netif_wake_queue(priv.dev);
    napi_enable(&priv.napi);
// Enable interrupts
    writel(DMA_INTR_DEFAULT_MASK, priv.base + XGMAC_DMA_STATUS);
    writel(DMA_INTR_DEFAULT_MASK, priv.base + XGMAC_DMA_INTR_ENA);
    }
#[no_mangle]
unsafe extern "C" fn xgmac_hw_init(dev: *mut net_device) -> c_int {
    static int xgmac_hw_init(struct net_device *dev)
    {
    u32 value, ctrl;
    int limit;
    struct xgmac_priv *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
// Save the ctrl register value
    ctrl = readl(ioaddr + XGMAC_CONTROL) & XGMAC_CONTROL_SPD_MASK;
// SW reset
    value = DMA_BUS_MODE_SFT_RESET;
    writel(value, ioaddr + XGMAC_DMA_BUS_MODE);
    limit = 15000;
    while (limit-- &&
    (readl(ioaddr + XGMAC_DMA_BUS_MODE) & DMA_BUS_MODE_SFT_RESET))
    cpu_relax();
    if (limit < 0)
    return -EBUSY;
    value = (0x10 << DMA_BUS_MODE_PBL_SHIFT) |
    (0x10 << DMA_BUS_MODE_RPBL_SHIFT) |
    DMA_BUS_MODE_FB | DMA_BUS_MODE_ATDS | DMA_BUS_MODE_AAL;
    writel(value, ioaddr + XGMAC_DMA_BUS_MODE);
    writel(0, ioaddr + XGMAC_DMA_INTR_ENA);
// Mask power mgt interrupt
    writel(XGMAC_INT_STAT_PMTIM, ioaddr + XGMAC_INT_STAT);
// XGMAC requires AXI bus init. This is a 'magic number' for now
    writel(0x0077000E, ioaddr + XGMAC_DMA_AXI_BUS);
    ctrl |= XGMAC_CONTROL_DDIC | XGMAC_CONTROL_JE | XGMAC_CONTROL_ACS |
    XGMAC_CONTROL_CAR;
    if (dev.features & NETIF_F_RXCSUM)
    ctrl |= XGMAC_CONTROL_IPC;
    writel(ctrl, ioaddr + XGMAC_CONTROL);
    writel(DMA_CONTROL_OSF, ioaddr + XGMAC_DMA_CONTROL);
// Set the HW DMA mode and the COE
    writel(XGMAC_OMR_TSF | XGMAC_OMR_RFD | XGMAC_OMR_RFA |
    XGMAC_OMR_RTC_256,
    ioaddr + XGMAC_OMR);
// Reset the MMC counters
    writel(1, ioaddr + XGMAC_MMC_CTRL);
    return 0;
    }
//
// xgmac_open - open entry point of the driver
// @dev : pointer to the device structure.
// Description:
// This function is the open entry point of the driver.
// Return value:
// 0 on success and an appropriate (-)ve integer as defined in errno.h
// file on failure.
//
#[no_mangle]
unsafe extern "C" fn xgmac_open(dev: *mut net_device) -> c_int {
    static int xgmac_open(struct net_device *dev)
    {
    int ret;
    struct xgmac_priv *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
// Check that the MAC address is valid.  If its not, refuse
// to bring the device up. The user must specify an
// address using the following linux command:
// ifconfig eth0 hw ether xx:xx:xx:xx:xx:xx
    if (!is_valid_ether_addr(dev.dev_addr)) {
    eth_hw_addr_random(dev);
    netdev_dbg(priv.dev, "generated random MAC address %pM\n",
    dev.dev_addr);
    }
    memset(&priv.xstats, 0, sizeof(struct xgmac_extra_stats));
// Initialize the XGMAC and descriptors
    xgmac_hw_init(dev);
    xgmac_set_mac_addr(ioaddr, dev.dev_addr, 0);
    xgmac_set_flow_ctrl(priv, priv.rx_pause, priv.tx_pause);
    ret = xgmac_dma_desc_rings_init(dev);
    if (ret < 0)
    return ret;
// Enable the MAC Rx/Tx
    xgmac_mac_enable(ioaddr);
    napi_enable(&priv.napi);
    netif_start_queue(dev);
// Enable interrupts
    writel(DMA_INTR_DEFAULT_MASK, ioaddr + XGMAC_DMA_STATUS);
    writel(DMA_INTR_DEFAULT_MASK, ioaddr + XGMAC_DMA_INTR_ENA);
    return 0;
    }
//
// xgmac_stop - close entry point of the driver
// @dev : device pointer.
// Description:
// This is the stop entry point of the driver.
//
#[no_mangle]
unsafe extern "C" fn xgmac_stop(dev: *mut net_device) -> c_int {
    static int xgmac_stop(struct net_device *dev)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    if (readl(priv.base + XGMAC_DMA_INTR_ENA))
    napi_disable(&priv.napi);
    writel(0, priv.base + XGMAC_DMA_INTR_ENA);
    netif_tx_disable(dev);
// Disable the MAC core
    xgmac_mac_disable(priv.base);
// Release and free the Rx/Tx resources
    xgmac_free_dma_desc_rings(priv);
    return 0;
    }
//
// xgmac_xmit:
// @skb : the socket buffer
// @dev : device pointer
// Description : Tx entry point of the driver.
//
#[no_mangle]
unsafe extern "C" fn xgmac_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t xgmac_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    unsigned int entry;
    int i;
    u32 irq_flag;
    let mut nfrags: c_int = skb_shinfo(skb).nr_frags;
    struct xgmac_dma_desc *desc, *first;
    unsigned int desc_flags;
    unsigned int len;
    dma_addr_t paddr;
    priv.tx_irq_cnt = (priv.tx_irq_cnt + 1) & (DMA_TX_RING_SZ/4 - 1);
    irq_flag = priv.tx_irq_cnt ? 0 : TXDESC_INTERRUPT;
    desc_flags = (skb.ip_summed == CHECKSUM_PARTIAL) ?
    TXDESC_CSUM_ALL : 0;
    entry = priv.tx_head;
    desc = priv.dma_tx + entry;
    first = desc;
    len = skb_headlen(skb);
    paddr = dma_map_single(priv.device, skb.data, len, DMA_TO_DEVICE);
    if (dma_mapping_error(priv.device, paddr)) {
    dev_kfree_skb_any(skb);
    return NETDEV_TX_OK;
    }
    priv.tx_skbuff[entry] = skb;
    desc_set_buf_addr_and_size(desc, paddr, len);
    for (i = 0; i < nfrags; i++) {
    skb_frag_t *frag = &skb_shinfo(skb).frags[i];
    len = skb_frag_size(frag);
    paddr = skb_frag_dma_map(priv.device, frag, 0, len,
    DMA_TO_DEVICE);
    if (dma_mapping_error(priv.device, paddr))
    goto dma_err;
    entry = dma_ring_incr(entry, DMA_TX_RING_SZ);
    desc = priv.dma_tx + entry;
    priv.tx_skbuff[entry] = skb;
    desc_set_buf_addr_and_size(desc, paddr, len);
    if (i < (nfrags - 1))
    desc_set_tx_owner(desc, desc_flags);
    }
// Interrupt on completition only for the latest segment
    if (desc != first)
    desc_set_tx_owner(desc, desc_flags |
    TXDESC_LAST_SEG | irq_flag);
    else
    desc_flags |= TXDESC_LAST_SEG | irq_flag;
// Set owner on first desc last to avoid race condition
    wmb();
    desc_set_tx_owner(first, desc_flags | TXDESC_FIRST_SEG);
    writel(1, priv.base + XGMAC_DMA_TX_POLL);
    priv.tx_head = dma_ring_incr(entry, DMA_TX_RING_SZ);
// Ensure tx_head update is visible to tx completion
    smp_mb();
    if (unlikely(tx_dma_ring_space(priv) <= MAX_SKB_FRAGS)) {
    netif_stop_queue(dev);
// Ensure netif_stop_queue is visible to tx completion
    smp_mb();
    if (tx_dma_ring_space(priv) > MAX_SKB_FRAGS)
    netif_start_queue(dev);
    }
    return NETDEV_TX_OK;
    dma_err:
    entry = priv.tx_head;
    for ( ; i > 0; i--) {
    entry = dma_ring_incr(entry, DMA_TX_RING_SZ);
    desc = priv.dma_tx + entry;
    priv.tx_skbuff[entry] = core::ptr::null_mut();
    dma_unmap_page(priv.device, desc_get_buf_addr(desc),
    desc_get_buf_len(desc), DMA_TO_DEVICE);
    desc_clear_tx_owner(desc);
    }
    desc = first;
    dma_unmap_single(priv.device, desc_get_buf_addr(desc),
    desc_get_buf_len(desc), DMA_TO_DEVICE);
    dev_kfree_skb_any(skb);
    return NETDEV_TX_OK;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_rx(priv: *mut xgmac_priv, limit: c_int) -> c_int {
    static int xgmac_rx(struct xgmac_priv *priv, int limit)
    {
    unsigned int entry;
    let mut count: c_uint = 0;
    struct xgmac_dma_desc *p;
    while (count < limit) {
    int ip_checksum;
    struct sk_buff *skb;
    int frame_len;
    if (!dma_ring_cnt(priv.rx_head, priv.rx_tail, DMA_RX_RING_SZ))
    break;
    entry = priv.rx_tail;
    p = priv.dma_rx + entry;
    if (desc_get_owner(p))
    break;
    count++;
    priv.rx_tail = dma_ring_incr(priv.rx_tail, DMA_RX_RING_SZ);
// read the status of the incoming frame
    ip_checksum = desc_get_rx_status(priv, p);
    if (ip_checksum < 0)
    continue;
    skb = priv.rx_skbuff[entry];
    if (unlikely(!skb)) {
    netdev_err(priv.dev, "Inconsistent Rx descriptor chain\n");
    break;
    }
    priv.rx_skbuff[entry] = core::ptr::null_mut();
    frame_len = desc_get_rx_frame_len(p);
    netdev_dbg(priv.dev, "RX frame size %d, COE status: %d\n",
    frame_len, ip_checksum);
    skb_put(skb, frame_len);
    dma_unmap_single(priv.device, desc_get_buf_addr(p),
    priv.dma_buf_sz - NET_IP_ALIGN, DMA_FROM_DEVICE);
    skb.protocol = eth_type_trans(skb, priv.dev);
    skb.ip_summed = ip_checksum;
    if (ip_checksum == CHECKSUM_NONE)
    netif_receive_skb(skb);
    else
    napi_gro_receive(&priv.napi, skb);
    }
    xgmac_rx_refill(priv);
    return count;
    }
//
// xgmac_poll - xgmac poll method (NAPI)
// @napi : pointer to the napi structure.
// @budget : maximum number of packets that the current CPU can receive from
// all interfaces.
// Description :
// This function implements the reception process.
// Also it runs the TX completion thread
//
#[no_mangle]
unsafe extern "C" fn xgmac_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int xgmac_poll(struct napi_struct *napi, int budget)
    {
    struct xgmac_priv *priv = container_of(napi,
    struct xgmac_priv, napi);
    let mut work_done: c_int = 0;
    xgmac_tx_complete(priv);
    work_done = xgmac_rx(priv, budget);
    if (work_done < budget) {
    napi_complete_done(napi, work_done);
    __raw_writel(DMA_INTR_DEFAULT_MASK, priv.base + XGMAC_DMA_INTR_ENA);
    }
    return work_done;
    }
//
// xgmac_tx_timeout
// @dev : Pointer to net device structure
// @txqueue: index of the hung transmit queue
//
// Description: this function is called when a packet transmission fails to
// complete within a reasonable tmrate. The driver will mark the error in the
// netdev structure and arrange for the device to be reset to a sane state
// in order to transmit a new packet.
//
#[no_mangle]
unsafe extern "C" fn xgmac_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void xgmac_tx_timeout(struct net_device *dev, unsigned int txqueue)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    schedule_work(&priv.tx_timeout_work);
    }
//
// xgmac_set_rx_mode - entry point for multicast addressing
// @dev : pointer to the device structure
// Description:
// This function is a driver entry point which gets called by the kernel
// whenever multicast addresses must be enabled/disabled.
// Return value:
// void.
//
#[no_mangle]
unsafe extern "C" fn xgmac_set_rx_mode(dev: *mut net_device) {
    static void xgmac_set_rx_mode(struct net_device *dev)
    {
    int i;
    struct xgmac_priv *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
    let mut value: c_uint = 0;
    u32 hash_filter[XGMAC_NUM_HASH];
    let mut reg: c_int = 1;
    struct netdev_hw_addr *ha;
    let mut use_hash: bool = false;
    netdev_dbg(priv.dev, "# mcasts %d, # unicast %d\n",
    netdev_mc_count(dev), netdev_uc_count(dev));
    if (dev.flags & IFF_PROMISC)
    value |= XGMAC_FRAME_FILTER_PR;
    memset(hash_filter, 0, sizeof(hash_filter));
    if (netdev_uc_count(dev) > priv.max_macs) {
    use_hash = true;
    value |= XGMAC_FRAME_FILTER_HUC | XGMAC_FRAME_FILTER_HPF;
    }
    netdev_for_each_uc_addr(ha, dev) {
    if (use_hash) {
    let mut bit_nr: u32 = ~ether_crc(ETH_ALEN, ha.addr) >> 23;
// The most significant 4 bits determine the register to
// use (H/L) while the other 5 bits determine the bit
// within the register.
    hash_filter[bit_nr >> 5] |= 1 << (bit_nr & 31);
    } else {
    xgmac_set_mac_addr(ioaddr, ha.addr, reg);
    reg++;
    }
    }
    if (dev.flags & IFF_ALLMULTI) {
    value |= XGMAC_FRAME_FILTER_PM;
    goto out;
    }
    if ((netdev_mc_count(dev) + reg - 1) > priv.max_macs) {
    use_hash = true;
    value |= XGMAC_FRAME_FILTER_HMC | XGMAC_FRAME_FILTER_HPF;
    } else {
    use_hash = false;
    }
    netdev_for_each_mc_addr(ha, dev) {
    if (use_hash) {
    let mut bit_nr: u32 = ~ether_crc(ETH_ALEN, ha.addr) >> 23;
// The most significant 4 bits determine the register to
// use (H/L) while the other 5 bits determine the bit
// within the register.
    hash_filter[bit_nr >> 5] |= 1 << (bit_nr & 31);
    } else {
    xgmac_set_mac_addr(ioaddr, ha.addr, reg);
    reg++;
    }
    }
    out:
    for (i = reg; i <= priv.max_macs; i++)
    xgmac_set_mac_addr(ioaddr, core::ptr::null_mut(), i);
    for (i = 0; i < XGMAC_NUM_HASH; i++)
    writel(hash_filter[i], ioaddr + XGMAC_HASH(i));
    writel(value, ioaddr + XGMAC_FRAME_FILTER);
    }
//
// xgmac_change_mtu - entry point to change MTU size for the device.
// @dev : device pointer.
// @new_mtu : the new MTU size for the device.
// Description: the Maximum Transfer Unit (MTU) is used by the network layer
// to drive packet transmission. Ethernet has an MTU of 1500 octets
// (ETH_DATA_LEN). This value can be changed with ifconfig.
// Return value:
// 0 on success and an appropriate (-)ve integer as defined in errno.h
// file on failure.
//
#[no_mangle]
unsafe extern "C" fn xgmac_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int xgmac_change_mtu(struct net_device *dev, int new_mtu)
    {
// Stop everything, get ready to change the MTU
    if (!netif_running(dev))
    return 0;
// Bring interface down, change mtu and bring interface back up
    xgmac_stop(dev);
    WRITE_ONCE(dev.mtu, new_mtu);
    return xgmac_open(dev);
    }
#[no_mangle]
unsafe extern "C" fn xgmac_pmt_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xgmac_pmt_interrupt(int irq, void *dev_id)
    {
    u32 intr_status;
    struct net_device *dev = (struct net_device *)dev_id;
    struct xgmac_priv *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
    intr_status = __raw_readl(ioaddr + XGMAC_INT_STAT);
    if (intr_status & XGMAC_INT_STAT_PMT) {
    netdev_dbg(priv.dev, "received Magic frame\n");
// clear the PMT bits 5 and 6 by reading the PMT
    readl(ioaddr + XGMAC_PMT);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t xgmac_interrupt(int irq, void *dev_id)
    {
    u32 intr_status;
    struct net_device *dev = (struct net_device *)dev_id;
    struct xgmac_priv *priv = netdev_priv(dev);
    struct xgmac_extra_stats *x = &priv.xstats;
// read the status register (CSR5)
    intr_status = __raw_readl(priv.base + XGMAC_DMA_STATUS);
    intr_status &= __raw_readl(priv.base + XGMAC_DMA_INTR_ENA);
    __raw_writel(intr_status, priv.base + XGMAC_DMA_STATUS);
// It displays the DMA process states (CSR5 register)
// ABNORMAL interrupts
    if (unlikely(intr_status & DMA_STATUS_AIS)) {
    if (intr_status & DMA_STATUS_TJT) {
    netdev_err(priv.dev, "transmit jabber\n");
    x.tx_jabber++;
    }
    if (intr_status & DMA_STATUS_RU)
    x.rx_buf_unav++;
    if (intr_status & DMA_STATUS_RPS) {
    netdev_err(priv.dev, "receive process stopped\n");
    x.rx_process_stopped++;
    }
    if (intr_status & DMA_STATUS_ETI) {
    netdev_err(priv.dev, "transmit early interrupt\n");
    x.tx_early++;
    }
    if (intr_status & DMA_STATUS_TPS) {
    netdev_err(priv.dev, "transmit process stopped\n");
    x.tx_process_stopped++;
    schedule_work(&priv.tx_timeout_work);
    }
    if (intr_status & DMA_STATUS_FBI) {
    netdev_err(priv.dev, "fatal bus error\n");
    x.fatal_bus_error++;
    }
    }
// TX/RX NORMAL interrupts
    if (intr_status & (DMA_STATUS_RI | DMA_STATUS_TU | DMA_STATUS_TI)) {
    __raw_writel(DMA_INTR_ABNORMAL, priv.base + XGMAC_DMA_INTR_ENA);
    napi_schedule(&priv.napi);
    }
    return IRQ_HANDLED;
    }

// Polling receive - used by NETCONSOLE and other diagnostic tools
// to allow network I/O with interrupts disabled.
#[no_mangle]
unsafe extern "C" fn xgmac_poll_controller(dev: *mut net_device) {
    static void xgmac_poll_controller(struct net_device *dev)
    {
    disable_irq(dev.irq);
    xgmac_interrupt(dev.irq, dev);
    enable_irq(dev.irq);
    }

    static void
    xgmac_get_stats64(struct net_device *dev,
    struct rtnl_link_stats64 *storage)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    void __iomem *base = priv.base;
    u32 count;
    spin_lock_bh(&priv.stats_lock);
    writel(XGMAC_MMC_CTRL_CNT_FRZ, base + XGMAC_MMC_CTRL);
    storage.rx_bytes = readl(base + XGMAC_MMC_RXOCTET_G_LO);
    storage.rx_bytes |= (u64)(readl(base + XGMAC_MMC_RXOCTET_G_HI)) << 32;
    storage.rx_packets = readl(base + XGMAC_MMC_RXFRAME_GB_LO);
    storage.multicast = readl(base + XGMAC_MMC_RXMCFRAME_G);
    storage.rx_crc_errors = readl(base + XGMAC_MMC_RXCRCERR);
    storage.rx_length_errors = readl(base + XGMAC_MMC_RXLENGTHERR);
    storage.rx_missed_errors = readl(base + XGMAC_MMC_RXOVERFLOW);
    storage.tx_bytes = readl(base + XGMAC_MMC_TXOCTET_G_LO);
    storage.tx_bytes |= (u64)(readl(base + XGMAC_MMC_TXOCTET_G_HI)) << 32;
    count = readl(base + XGMAC_MMC_TXFRAME_GB_LO);
    storage.tx_errors = count - readl(base + XGMAC_MMC_TXFRAME_G_LO);
    storage.tx_packets = count;
    storage.tx_fifo_errors = readl(base + XGMAC_MMC_TXUNDERFLOW);
    writel(0, base + XGMAC_MMC_CTRL);
    spin_unlock_bh(&priv.stats_lock);
    }
#[no_mangle]
unsafe extern "C" fn xgmac_set_mac_address(dev: *mut net_device, p: *mut c_void) -> c_int {
    static int xgmac_set_mac_address(struct net_device *dev, void *p)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
    struct sockaddr *addr = p;
    if (!is_valid_ether_addr(addr.sa_data))
    return -EADDRNOTAVAIL;
    eth_hw_addr_set(dev, addr.sa_data);
    xgmac_set_mac_addr(ioaddr, dev.dev_addr, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_set_features(dev: *mut net_device, features: netdev_features_t) -> c_int {
    static int xgmac_set_features(struct net_device *dev, netdev_features_t features)
    {
    u32 ctrl;
    struct xgmac_priv *priv = netdev_priv(dev);
    void __iomem *ioaddr = priv.base;
    let mut changed: netdev_features_t = dev.features ^ features;
    if (!(changed & NETIF_F_RXCSUM))
    return 0;
    ctrl = readl(ioaddr + XGMAC_CONTROL);
    if (features & NETIF_F_RXCSUM)
    ctrl |= XGMAC_CONTROL_IPC;
    else
    ctrl &= ~XGMAC_CONTROL_IPC;
    writel(ctrl, ioaddr + XGMAC_CONTROL);
    return 0;
    }
    static const struct net_device_ops xgmac_netdev_ops = {
    .ndo_open = xgmac_open,
    .ndo_start_xmit = xgmac_xmit,
    .ndo_stop = xgmac_stop,
    .ndo_change_mtu = xgmac_change_mtu,
    .ndo_set_rx_mode = xgmac_set_rx_mode,
    .ndo_tx_timeout = xgmac_tx_timeout,
    .ndo_get_stats64 = xgmac_get_stats64,

    .ndo_poll_controller = xgmac_poll_controller,

    .ndo_set_mac_address = xgmac_set_mac_address,
    .ndo_set_features = xgmac_set_features,
    };
    static int xgmac_ethtool_get_link_ksettings(struct net_device *dev,
    struct ethtool_link_ksettings *cmd)
    {
    cmd.base.autoneg = 0;
    cmd.base.duplex = DUPLEX_FULL;
    cmd.base.speed = 10000;
    ethtool_convert_legacy_u32_to_link_mode(cmd.link_modes.supported, 0);
    ethtool_convert_legacy_u32_to_link_mode(cmd.link_modes.advertising, 0);
    return 0;
    }
    static void xgmac_get_pauseparam(struct net_device *netdev,
    struct ethtool_pauseparam *pause)
    {
    struct xgmac_priv *priv = netdev_priv(netdev);
    pause.rx_pause = priv.rx_pause;
    pause.tx_pause = priv.tx_pause;
    }
    static int xgmac_set_pauseparam(struct net_device *netdev,
    struct ethtool_pauseparam *pause)
    {
    struct xgmac_priv *priv = netdev_priv(netdev);
    if (pause.autoneg)
    return -EINVAL;
    return xgmac_set_flow_ctrl(priv, pause.rx_pause, pause.tx_pause);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgmac_stats {
    pub stat_string: [c_char; ETH_GSTRING_LEN],
    pub stat_offset: c_int,
    pub is_reg: bool,
}

    { #m, offsetof(struct xgmac_priv, xstats.m), false }

    { #m, reg_offset, true }
    static const struct xgmac_stats xgmac_gstrings_stats[] = {
    XGMAC_STAT(tx_frame_flushed),
    XGMAC_STAT(tx_payload_error),
    XGMAC_STAT(tx_ip_header_error),
    XGMAC_STAT(tx_local_fault),
    XGMAC_STAT(tx_remote_fault),
    XGMAC_STAT(tx_early),
    XGMAC_STAT(tx_process_stopped),
    XGMAC_STAT(tx_jabber),
    XGMAC_STAT(rx_buf_unav),
    XGMAC_STAT(rx_process_stopped),
    XGMAC_STAT(rx_payload_error),
    XGMAC_STAT(rx_ip_header_error),
    XGMAC_STAT(rx_da_filter_fail),
    XGMAC_STAT(fatal_bus_error),
    XGMAC_HW_STAT(rx_watchdog, XGMAC_MMC_RXWATCHDOG),
    XGMAC_HW_STAT(tx_vlan, XGMAC_MMC_TXVLANFRAME),
    XGMAC_HW_STAT(rx_vlan, XGMAC_MMC_RXVLANFRAME),
    XGMAC_HW_STAT(tx_pause, XGMAC_MMC_TXPAUSEFRAME),
    XGMAC_HW_STAT(rx_pause, XGMAC_MMC_RXPAUSEFRAME),
    };

    static void xgmac_get_ethtool_stats(struct net_device *dev,
    struct ethtool_stats *dummy,
    u64 *data)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    void *p = priv;
    int i;
    for (i = 0; i < XGMAC_STATS_LEN; i++) {
    if (xgmac_gstrings_stats[i].is_reg)
// data++ = readl(priv->base +
    xgmac_gstrings_stats[i].stat_offset);
    else
// data++ = *(u32 *)(p +
    xgmac_gstrings_stats[i].stat_offset);
    }
    }
#[no_mangle]
unsafe extern "C" fn xgmac_get_sset_count(netdev: *mut net_device, sset: c_int) -> c_int {
    static int xgmac_get_sset_count(struct net_device *netdev, int sset)
    {
    switch (sset) {
    case ETH_SS_STATS:
    return XGMAC_STATS_LEN;
    default:
    return -EINVAL;
    }
    }
    static void xgmac_get_strings(struct net_device *dev, u32 stringset,
    u8 *data)
    {
    int i;
    u8 *p = data;
    switch (stringset) {
    case ETH_SS_STATS:
    for (i = 0; i < XGMAC_STATS_LEN; i++) {
    memcpy(p, xgmac_gstrings_stats[i].stat_string,
    ETH_GSTRING_LEN);
    p += ETH_GSTRING_LEN;
    }
    break;
    default:
    WARN_ON(1);
    break;
    }
    }
    static void xgmac_get_wol(struct net_device *dev,
    struct ethtool_wolinfo *wol)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    if (device_can_wakeup(priv.device)) {
    wol.supported = WAKE_MAGIC | WAKE_UCAST;
    wol.wolopts = priv.wolopts;
    }
    }
    static int xgmac_set_wol(struct net_device *dev,
    struct ethtool_wolinfo *wol)
    {
    struct xgmac_priv *priv = netdev_priv(dev);
    let mut support: u32 = WAKE_MAGIC | WAKE_UCAST;
    if (!device_can_wakeup(priv.device))
    return -ENOTSUPP;
    if (wol.wolopts & ~support)
    return -EINVAL;
    priv.wolopts = wol.wolopts;
    if (wol.wolopts) {
    device_set_wakeup_enable(priv.device, 1);
    enable_irq_wake(dev.irq);
    } else {
    device_set_wakeup_enable(priv.device, 0);
    disable_irq_wake(dev.irq);
    }
    return 0;
    }
    static const struct ethtool_ops xgmac_ethtool_ops = {
    .get_link = ethtool_op_get_link,
    .get_pauseparam = xgmac_get_pauseparam,
    .set_pauseparam = xgmac_set_pauseparam,
    .get_ethtool_stats = xgmac_get_ethtool_stats,
    .get_strings = xgmac_get_strings,
    .get_wol = xgmac_get_wol,
    .set_wol = xgmac_set_wol,
    .get_sset_count = xgmac_get_sset_count,
    .get_link_ksettings = xgmac_ethtool_get_link_ksettings,
    };
//
// xgmac_probe
// @pdev: platform device pointer
// Description: the driver is initialized through platform_device.
//
#[no_mangle]
unsafe extern "C" fn xgmac_probe(pdev: *mut platform_device) -> c_int {
    static int xgmac_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0;
    struct resource *res;
    struct net_device *ndev = core::ptr::null_mut();
    struct xgmac_priv *priv = core::ptr::null_mut();
    u8 addr[ETH_ALEN];
    u32 uid;
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    if (!res)
    return -ENODEV;
    if (!request_mem_region(res.start, resource_size(res), pdev.name))
    return -EBUSY;
    ndev = alloc_etherdev(sizeof(struct xgmac_priv));
    if (!ndev) {
    ret = -ENOMEM;
    goto err_alloc;
    }
    SET_NETDEV_DEV(ndev, &pdev.dev);
    priv = netdev_priv(ndev);
    platform_set_drvdata(pdev, ndev);
    ndev.netdev_ops = &xgmac_netdev_ops;
    ndev.ethtool_ops = &xgmac_ethtool_ops;
    spin_lock_init(&priv.stats_lock);
    INIT_WORK(&priv.tx_timeout_work, xgmac_tx_timeout_work);
    priv.device = &pdev.dev;
    priv.dev = ndev;
    priv.rx_pause = 1;
    priv.tx_pause = 1;
    priv.base = ioremap(res.start, resource_size(res));
    if (!priv.base) {
    netdev_err(ndev, "ioremap failed\n");
    ret = -ENOMEM;
    goto err_io;
    }
    uid = readl(priv.base + XGMAC_VERSION);
    netdev_info(ndev, "h/w version is 0x%x\n", uid);
// Figure out how many valid mac address filter registers we have
    writel(1, priv.base + XGMAC_ADDR_HIGH(31));
    if (readl(priv.base + XGMAC_ADDR_HIGH(31)) == 1)
    priv.max_macs = 31;
    else
    priv.max_macs = 7;
    writel(0, priv.base + XGMAC_DMA_INTR_ENA);
    ndev.irq = platform_get_irq(pdev, 0);
    if (ndev.irq == -ENXIO) {
    netdev_err(ndev, "No irq resource\n");
    ret = ndev.irq;
    goto err_irq;
    }
    ret = request_irq(ndev.irq, xgmac_interrupt, 0,
    dev_name(&pdev.dev), ndev);
    if (ret < 0) {
    netdev_err(ndev, "Could not request irq %d - ret %d)\n",
    ndev.irq, ret);
    goto err_irq;
    }
    priv.pmt_irq = platform_get_irq(pdev, 1);
    if (priv.pmt_irq == -ENXIO) {
    netdev_err(ndev, "No pmt irq resource\n");
    ret = priv.pmt_irq;
    goto err_pmt_irq;
    }
    ret = request_irq(priv.pmt_irq, xgmac_pmt_interrupt, 0,
    dev_name(&pdev.dev), ndev);
    if (ret < 0) {
    netdev_err(ndev, "Could not request irq %d - ret %d)\n",
    priv.pmt_irq, ret);
    goto err_pmt_irq;
    }
    device_set_wakeup_capable(&pdev.dev, 1);
    if (device_can_wakeup(priv.device))
    priv.wolopts = WAKE_MAGIC;	/* Magic Frame as default */
    ndev.hw_features = NETIF_F_SG | NETIF_F_HIGHDMA;
    if (readl(priv.base + XGMAC_DMA_HW_FEATURE) & DMA_HW_FEAT_TXCOESEL)
    ndev.hw_features |= NETIF_F_IP_CSUM | NETIF_F_IPV6_CSUM |
    NETIF_F_RXCSUM;
    ndev.features |= ndev.hw_features;
    ndev.priv_flags |= IFF_UNICAST_FLT;
// MTU range: 46 - 9000
    ndev.min_mtu = ETH_ZLEN - ETH_HLEN;
    ndev.max_mtu = XGMAC_MAX_MTU;
// Get the MAC address
    xgmac_get_mac_addr(priv.base, addr, 0);
    eth_hw_addr_set(ndev, addr);
    if (!is_valid_ether_addr(ndev.dev_addr))
    netdev_warn(ndev, "MAC address %pM not valid",
    ndev.dev_addr);
    netif_napi_add(ndev, &priv.napi, xgmac_poll);
    ret = register_netdev(ndev);
    if (ret)
    goto err_reg;
    return 0;
    err_reg:
    netif_napi_del(&priv.napi);
    free_irq(priv.pmt_irq, ndev);
    err_pmt_irq:
    free_irq(ndev.irq, ndev);
    err_irq:
    iounmap(priv.base);
    err_io:
    free_netdev(ndev);
    err_alloc:
    release_mem_region(res.start, resource_size(res));
    return ret;
    }
//
// xgmac_remove
// @pdev: platform device pointer
// Description: this function resets the TX/RX processes, disables the MAC RX/TX
// changes the link status, releases the DMA descriptor rings,
// unregisters the MDIO bus and unmaps the allocated memory.
//
#[no_mangle]
unsafe extern "C" fn xgmac_remove(pdev: *mut platform_device) {
    static void xgmac_remove(struct platform_device *pdev)
    {
    struct net_device *ndev = platform_get_drvdata(pdev);
    struct xgmac_priv *priv = netdev_priv(ndev);
    struct resource *res;
    xgmac_mac_disable(priv.base);
// Free the IRQ lines
    free_irq(ndev.irq, ndev);
    free_irq(priv.pmt_irq, ndev);
    unregister_netdev(ndev);
    netif_napi_del(&priv.napi);
    iounmap(priv.base);
    res = platform_get_resource(pdev, IORESOURCE_MEM, 0);
    release_mem_region(res.start, resource_size(res));
    free_netdev(ndev);
    }

#[no_mangle]
unsafe extern "C" fn xgmac_pmt(ioaddr: *mut void __iomem, mode: c_ulong) {
    static void xgmac_pmt(void __iomem *ioaddr, unsigned long mode)
    {
    let mut pmt: c_uint = 0;
    if (mode & WAKE_MAGIC)
    pmt |= XGMAC_PMT_POWERDOWN | XGMAC_PMT_MAGIC_PKT_EN;
    if (mode & WAKE_UCAST)
    pmt |= XGMAC_PMT_POWERDOWN | XGMAC_PMT_GLBL_UNICAST;
    writel(pmt, ioaddr + XGMAC_PMT);
    }
#[no_mangle]
unsafe extern "C" fn xgmac_suspend(dev: *mut device) -> c_int {
    static int xgmac_suspend(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    struct xgmac_priv *priv = netdev_priv(ndev);
    u32 value;
    if (!ndev || !netif_running(ndev))
    return 0;
    netif_device_detach(ndev);
    napi_disable(&priv.napi);
    writel(0, priv.base + XGMAC_DMA_INTR_ENA);
    if (device_may_wakeup(priv.device)) {
// Stop TX/RX DMA Only
    value = readl(priv.base + XGMAC_DMA_CONTROL);
    value &= ~(DMA_CONTROL_ST | DMA_CONTROL_SR);
    writel(value, priv.base + XGMAC_DMA_CONTROL);
    xgmac_pmt(priv.base, priv.wolopts);
    } else
    xgmac_mac_disable(priv.base);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xgmac_resume(dev: *mut device) -> c_int {
    static int xgmac_resume(struct device *dev)
    {
    struct net_device *ndev = dev_get_drvdata(dev);
    struct xgmac_priv *priv = netdev_priv(ndev);
    void __iomem *ioaddr = priv.base;
    if (!netif_running(ndev))
    return 0;
    xgmac_pmt(ioaddr, 0);
// Enable the MAC and DMA
    xgmac_mac_enable(ioaddr);
    writel(DMA_INTR_DEFAULT_MASK, ioaddr + XGMAC_DMA_STATUS);
    writel(DMA_INTR_DEFAULT_MASK, ioaddr + XGMAC_DMA_INTR_ENA);
    netif_device_attach(ndev);
    napi_enable(&priv.napi);
    return 0;
    }

    static SIMPLE_DEV_PM_OPS(xgmac_pm_ops, xgmac_suspend, xgmac_resume);
    static const struct of_device_id xgmac_of_match[] = {
    { .compatible = "calxeda,hb-xgmac", },
    {},
    };
    MODULE_DEVICE_TABLE(of, xgmac_of_match);
    static struct platform_driver xgmac_driver = {
    .driver = {
    .name = "calxedaxgmac",
    .of_match_table = xgmac_of_match,
    .pm = &xgmac_pm_ops,
    },
    .probe = xgmac_probe,
    .remove = xgmac_remove,
    };
    module_platform_driver(xgmac_driver);
    MODULE_AUTHOR("Calxeda, Inc.");
    MODULE_DESCRIPTION("Calxeda 10G XGMAC driver");
    MODULE_LICENSE("GPL v2");
