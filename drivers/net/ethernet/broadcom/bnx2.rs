//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnx2.h
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


// bnx2.h: QLogic bnx2 network driver.
//
// Copyright (c) 2004-2014 Broadcom Corporation
// Copyright (c) 2014-2015 QLogic Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
// Written by: Michael Chan  (mchan@broadcom.com)
//
// Hardware data structures and register definitions automatically
// generated from RTL code. Do not modify.
//
// tx_bd definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_tx_bd {
    pub tx_bd_haddr_hi: u32,
    pub tx_bd_haddr_lo: u32,
    pub tx_bd_mss_nbytes: u32,

    pub tx_bd_vlan_tag_flags: u32,

}

//
// rx_bd definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_rx_bd {
    pub rx_bd_haddr_hi: u32,
    pub rx_bd_haddr_lo: u32,
    pub rx_bd_len: u32,
    pub rx_bd_flags: u32,

}

pub const BNX2_RX_ALIGN: c_int = 16;
//
// status_block definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_block {
    pub status_attn_bits: u32,

    pub status_attn_bits_ack: u32,

    pub status_tx_quick_consumer_index0: u16,
    pub status_tx_quick_consumer_index1: u16,
    pub status_tx_quick_consumer_index2: u16,
    pub status_tx_quick_consumer_index3: u16,
    pub status_rx_quick_consumer_index0: u16,
    pub status_rx_quick_consumer_index1: u16,
    pub status_rx_quick_consumer_index2: u16,
    pub status_rx_quick_consumer_index3: u16,
    pub status_rx_quick_consumer_index4: u16,
    pub status_rx_quick_consumer_index5: u16,
    pub status_rx_quick_consumer_index6: u16,
    pub status_rx_quick_consumer_index7: u16,
    pub status_rx_quick_consumer_index8: u16,
    pub status_rx_quick_consumer_index9: u16,
    pub status_rx_quick_consumer_index10: u16,
    pub status_rx_quick_consumer_index11: u16,
    pub status_rx_quick_consumer_index12: u16,
    pub status_rx_quick_consumer_index13: u16,
    pub status_rx_quick_consumer_index14: u16,
    pub status_rx_quick_consumer_index15: u16,
    pub status_completion_producer_index: u16,
    pub status_cmd_consumer_index: u16,
    pub status_idx: u16,
    pub status_unused: u8,
    pub status_blk_num: u8,

    pub status_tx_quick_consumer_index1: u16,
    pub status_tx_quick_consumer_index0: u16,
    pub status_tx_quick_consumer_index3: u16,
    pub status_tx_quick_consumer_index2: u16,
    pub status_rx_quick_consumer_index1: u16,
    pub status_rx_quick_consumer_index0: u16,
    pub status_rx_quick_consumer_index3: u16,
    pub status_rx_quick_consumer_index2: u16,
    pub status_rx_quick_consumer_index5: u16,
    pub status_rx_quick_consumer_index4: u16,
    pub status_rx_quick_consumer_index7: u16,
    pub status_rx_quick_consumer_index6: u16,
    pub status_rx_quick_consumer_index9: u16,
    pub status_rx_quick_consumer_index8: u16,
    pub status_rx_quick_consumer_index11: u16,
    pub status_rx_quick_consumer_index10: u16,
    pub status_rx_quick_consumer_index13: u16,
    pub status_rx_quick_consumer_index12: u16,
    pub status_rx_quick_consumer_index15: u16,
    pub status_rx_quick_consumer_index14: u16,
    pub status_cmd_consumer_index: u16,
    pub status_completion_producer_index: u16,
    pub status_blk_num: u8,
    pub status_unused: u8,
    pub status_idx: u16,

}

//
// status_block definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct status_block_msix {

    pub status_tx_quick_consumer_index: u16,
    pub status_rx_quick_consumer_index: u16,
    pub status_completion_producer_index: u16,
    pub status_cmd_consumer_index: u16,
    pub status_unused: u32,
    pub status_idx: u16,
    pub status_unused2: u8,
    pub status_blk_num: u8,

    pub status_rx_quick_consumer_index: u16,
    pub status_tx_quick_consumer_index: u16,
    pub status_cmd_consumer_index: u16,
    pub status_completion_producer_index: u16,
    pub status_unused: u32,
    pub status_blk_num: u8,
    pub status_unused2: u8,
    pub status_idx: u16,

}

pub const BNX2_SBLK_MSIX_ALIGN_SIZE: c_int = 128;
//
// statistics_block definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_block {
    pub stat_IfHCInOctets_hi: u32,
    pub stat_IfHCInOctets_lo: u32,
    pub stat_IfHCInBadOctets_hi: u32,
    pub stat_IfHCInBadOctets_lo: u32,
    pub stat_IfHCOutOctets_hi: u32,
    pub stat_IfHCOutOctets_lo: u32,
    pub stat_IfHCOutBadOctets_hi: u32,
    pub stat_IfHCOutBadOctets_lo: u32,
    pub stat_IfHCInUcastPkts_hi: u32,
    pub stat_IfHCInUcastPkts_lo: u32,
    pub stat_IfHCInMulticastPkts_hi: u32,
    pub stat_IfHCInMulticastPkts_lo: u32,
    pub stat_IfHCInBroadcastPkts_hi: u32,
    pub stat_IfHCInBroadcastPkts_lo: u32,
    pub stat_IfHCOutUcastPkts_hi: u32,
    pub stat_IfHCOutUcastPkts_lo: u32,
    pub stat_IfHCOutMulticastPkts_hi: u32,
    pub stat_IfHCOutMulticastPkts_lo: u32,
    pub stat_IfHCOutBroadcastPkts_hi: u32,
    pub stat_IfHCOutBroadcastPkts_lo: u32,
    pub stat_emac_tx_stat_dot3statsinternalmactransmiterrors: u32,
    pub stat_Dot3StatsCarrierSenseErrors: u32,
    pub stat_Dot3StatsFCSErrors: u32,
    pub stat_Dot3StatsAlignmentErrors: u32,
    pub stat_Dot3StatsSingleCollisionFrames: u32,
    pub stat_Dot3StatsMultipleCollisionFrames: u32,
    pub stat_Dot3StatsDeferredTransmissions: u32,
    pub stat_Dot3StatsExcessiveCollisions: u32,
    pub stat_Dot3StatsLateCollisions: u32,
    pub stat_EtherStatsCollisions: u32,
    pub stat_EtherStatsFragments: u32,
    pub stat_EtherStatsJabbers: u32,
    pub stat_EtherStatsUndersizePkts: u32,
    pub stat_EtherStatsOverrsizePkts: u32,
    pub stat_EtherStatsPktsRx64Octets: u32,
    pub stat_EtherStatsPktsRx65Octetsto127Octets: u32,
    pub stat_EtherStatsPktsRx128Octetsto255Octets: u32,
    pub stat_EtherStatsPktsRx256Octetsto511Octets: u32,
    pub stat_EtherStatsPktsRx512Octetsto1023Octets: u32,
    pub stat_EtherStatsPktsRx1024Octetsto1522Octets: u32,
    pub stat_EtherStatsPktsRx1523Octetsto9022Octets: u32,
    pub stat_EtherStatsPktsTx64Octets: u32,
    pub stat_EtherStatsPktsTx65Octetsto127Octets: u32,
    pub stat_EtherStatsPktsTx128Octetsto255Octets: u32,
    pub stat_EtherStatsPktsTx256Octetsto511Octets: u32,
    pub stat_EtherStatsPktsTx512Octetsto1023Octets: u32,
    pub stat_EtherStatsPktsTx1024Octetsto1522Octets: u32,
    pub stat_EtherStatsPktsTx1523Octetsto9022Octets: u32,
    pub stat_XonPauseFramesReceived: u32,
    pub stat_XoffPauseFramesReceived: u32,
    pub stat_OutXonSent: u32,
    pub stat_OutXoffSent: u32,
    pub stat_FlowControlDone: u32,
    pub stat_MacControlFramesReceived: u32,
    pub stat_XoffStateEntered: u32,
    pub stat_IfInFramesL2FilterDiscards: u32,
    pub stat_IfInRuleCheckerDiscards: u32,
    pub stat_IfInFTQDiscards: u32,
    pub stat_IfInMBUFDiscards: u32,
    pub stat_IfInRuleCheckerP4Hit: u32,
    pub stat_CatchupInRuleCheckerDiscards: u32,
    pub stat_CatchupInFTQDiscards: u32,
    pub stat_CatchupInMBUFDiscards: u32,
    pub stat_CatchupInRuleCheckerP4Hit: u32,
    pub stat_GenStat00: u32,
    pub stat_GenStat01: u32,
    pub stat_GenStat02: u32,
    pub stat_GenStat03: u32,
    pub stat_GenStat04: u32,
    pub stat_GenStat05: u32,
    pub stat_GenStat06: u32,
    pub stat_GenStat07: u32,
    pub stat_GenStat08: u32,
    pub stat_GenStat09: u32,
    pub stat_GenStat10: u32,
    pub stat_GenStat11: u32,
    pub stat_GenStat12: u32,
    pub stat_GenStat13: u32,
    pub stat_GenStat14: u32,
    pub stat_GenStat15: u32,
    pub stat_FwRxDrop: u32,
}

//
// l2_fhdr definition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct l2_fhdr {
    pub l2_fhdr_status: u32,

    pub l2_fhdr_hash: u32,

    pub l2_fhdr_pkt_len: u16,
    pub l2_fhdr_vlan_tag: u16,
    pub l2_fhdr_ip_xsum: u16,
    pub l2_fhdr_tcp_udp_xsum: u16,

    pub l2_fhdr_vlan_tag: u16,
    pub l2_fhdr_pkt_len: u16,
    pub l2_fhdr_tcp_udp_xsum: u16,
    pub l2_fhdr_ip_xsum: u16,

}

//
// l2_context definition
//
pub const BNX2_L2CTX_TYPE: c_uint = 0x00000000;

pub const BNX2_L2CTX_TX_HOST_BIDX: c_uint = 0x00000088;
pub const BNX2_L2CTX_EST_NBD: c_uint = 0x00000088;
pub const BNX2_L2CTX_CMD_TYPE: c_uint = 0x00000088;

pub const BNX2_L2CTX_TX_HOST_BSEQ: c_uint = 0x00000090;
pub const BNX2_L2CTX_TSCH_BSEQ: c_uint = 0x00000094;
pub const BNX2_L2CTX_TBDR_BSEQ: c_uint = 0x00000098;
pub const BNX2_L2CTX_TBDR_BOFF: c_uint = 0x0000009c;
pub const BNX2_L2CTX_TBDR_BIDX: c_uint = 0x0000009c;
pub const BNX2_L2CTX_TBDR_BHADDR_HI: c_uint = 0x000000a0;
pub const BNX2_L2CTX_TBDR_BHADDR_LO: c_uint = 0x000000a4;
pub const BNX2_L2CTX_TXP_BOFF: c_uint = 0x000000a8;
pub const BNX2_L2CTX_TXP_BIDX: c_uint = 0x000000a8;
pub const BNX2_L2CTX_TXP_BSEQ: c_uint = 0x000000ac;
pub const BNX2_L2CTX_TYPE_XI: c_uint = 0x00000080;
pub const BNX2_L2CTX_CMD_TYPE_XI: c_uint = 0x00000240;
pub const BNX2_L2CTX_TBDR_BHADDR_HI_XI: c_uint = 0x00000258;
pub const BNX2_L2CTX_TBDR_BHADDR_LO_XI: c_uint = 0x0000025c;
//
// l2_bd_chain_context definition
//
pub const BNX2_L2CTX_BD_PRE_READ: c_uint = 0x00000000;
pub const BNX2_L2CTX_CTX_SIZE: c_uint = 0x00000000;
pub const BNX2_L2CTX_CTX_TYPE: c_uint = 0x00000000;
pub const BNX2_L2CTX_FLOW_CTRL_ENABLE: c_uint = 0x000000ff;

pub const BNX2_L2CTX_HOST_BDIDX: c_uint = 0x00000004;
pub const BNX2_L2CTX_L5_STATUSB_NUM_SHIFT: c_int = 16;
pub const BNX2_L2CTX_L2_STATUSB_NUM_SHIFT: c_int = 24;

pub const BNX2_L2CTX_HOST_BSEQ: c_uint = 0x00000008;
pub const BNX2_L2CTX_NX_BSEQ: c_uint = 0x0000000c;
pub const BNX2_L2CTX_NX_BDHADDR_HI: c_uint = 0x00000010;
pub const BNX2_L2CTX_NX_BDHADDR_LO: c_uint = 0x00000014;
pub const BNX2_L2CTX_NX_BDIDX: c_uint = 0x00000018;
pub const BNX2_L2CTX_HOST_PG_BDIDX: c_uint = 0x00000044;
pub const BNX2_L2CTX_PG_BUF_SIZE: c_uint = 0x00000048;
pub const BNX2_L2CTX_RBDC_KEY: c_uint = 0x0000004c;
pub const BNX2_L2CTX_RBDC_JUMBO_KEY: c_uint = 0x3ffe;
pub const BNX2_L2CTX_NX_PG_BDHADDR_HI: c_uint = 0x00000050;
pub const BNX2_L2CTX_NX_PG_BDHADDR_LO: c_uint = 0x00000054;
//
// pci_config_l definition
// offset: 0000
//
pub const BNX2_PCICFG_MSI_CONTROL: c_uint = 0x00000058;

pub const BNX2_PCICFG_MISC_CONFIG: c_uint = 0x00000068;

pub const BNX2_PCICFG_MISC_STATUS: c_uint = 0x0000006c;

pub const BNX2_PCICFG_PCI_CLOCK_CONTROL_BITS: c_uint = 0x00000070;

pub const BNX2_PCICFG_REG_WINDOW_ADDRESS: c_uint = 0x00000078;

pub const BNX2_PCICFG_REG_WINDOW: c_uint = 0x00000080;
pub const BNX2_PCICFG_INT_ACK_CMD: c_uint = 0x00000084;

pub const BNX2_PCICFG_INT_ACK_CMD_INT_NUM_SHIFT: c_int = 24;
pub const BNX2_PCICFG_STATUS_BIT_SET_CMD: c_uint = 0x00000088;
pub const BNX2_PCICFG_STATUS_BIT_CLEAR_CMD: c_uint = 0x0000008c;
pub const BNX2_PCICFG_MAILBOX_QUEUE_ADDR: c_uint = 0x00000090;
pub const BNX2_PCICFG_MAILBOX_QUEUE_DATA: c_uint = 0x00000094;
pub const BNX2_PCICFG_DEVICE_CONTROL: c_uint = 0x000000b4;

//
// pci_reg definition
// offset: 0x400
//
pub const BNX2_PCI_GRC_WINDOW_ADDR: c_uint = 0x00000400;

pub const BNX2_PCI_GRC_WINDOW2_BASE: c_uint = 0xc000;
pub const BNX2_PCI_GRC_WINDOW3_BASE: c_uint = 0xe000;
pub const BNX2_PCI_CONFIG_1: c_uint = 0x00000404;

pub const BNX2_PCI_CONFIG_2: c_uint = 0x00000408;

pub const BNX2_PCI_CONFIG_3: c_uint = 0x0000040c;

pub const BNX2_PCI_PM_DATA_A: c_uint = 0x00000410;

pub const BNX2_PCI_PM_DATA_B: c_uint = 0x00000414;

pub const BNX2_PCI_SWAP_DIAG0: c_uint = 0x00000418;
pub const BNX2_PCI_SWAP_DIAG1: c_uint = 0x0000041c;
pub const BNX2_PCI_EXP_ROM_ADDR: c_uint = 0x00000420;

pub const BNX2_PCI_EXP_ROM_DATA: c_uint = 0x00000424;
pub const BNX2_PCI_VPD_INTF: c_uint = 0x00000428;

pub const BNX2_PCI_VPD_ADDR_FLAG: c_uint = 0x0000042c;
pub const BNX2_PCI_VPD_ADDR_FLAG_MSK: c_uint = 0x0000ffff;

pub const BNX2_PCI_VPD_DATA: c_uint = 0x00000430;
pub const BNX2_PCI_ID_VAL1: c_uint = 0x00000434;

pub const BNX2_PCI_ID_VAL2: c_uint = 0x00000438;

pub const BNX2_PCI_ID_VAL3: c_uint = 0x0000043c;

pub const BNX2_PCI_ID_VAL4: c_uint = 0x00000440;

pub const BNX2_PCI_ID_VAL5: c_uint = 0x00000444;

pub const BNX2_PCI_PCIX_EXTENDED_STATUS: c_uint = 0x00000448;

pub const BNX2_PCI_ID_VAL6: c_uint = 0x0000044c;

pub const BNX2_PCI_MSI_DATA: c_uint = 0x00000450;

pub const BNX2_PCI_MSI_ADDR_H: c_uint = 0x00000454;
pub const BNX2_PCI_MSI_ADDR_L: c_uint = 0x00000458;

pub const BNX2_PCI_CFG_ACCESS_CMD: c_uint = 0x0000045c;

pub const BNX2_PCI_CFG_ACCESS_DATA: c_uint = 0x00000460;
pub const BNX2_PCI_MSI_MASK: c_uint = 0x00000464;

pub const BNX2_PCI_MSI_PEND: c_uint = 0x00000468;

pub const BNX2_PCI_PM_DATA_C: c_uint = 0x0000046c;

pub const BNX2_PCI_MSIX_CONTROL: c_uint = 0x000004c0;

pub const BNX2_PCI_MSIX_TBL_OFF_BIR: c_uint = 0x000004c4;

pub const BNX2_PCI_MSIX_PBA_OFF_BIT: c_uint = 0x000004c8;

pub const BNX2_PCI_PCIE_CAPABILITY: c_uint = 0x000004d0;

pub const BNX2_PCI_DEVICE_CAPABILITY: c_uint = 0x000004d4;

pub const BNX2_PCI_LINK_CAPABILITY: c_uint = 0x000004dc;

pub const BNX2_PCI_PCIE_DEVICE_CAPABILITY_2: c_uint = 0x000004e4;

pub const BNX2_PCI_PCIE_LINK_CAPABILITY_2: c_uint = 0x000004e8;

pub const BNX2_PCI_GRC_WINDOW1_ADDR: c_uint = 0x00000610;

pub const BNX2_PCI_GRC_WINDOW2_ADDR: c_uint = 0x00000614;

pub const BNX2_PCI_GRC_WINDOW3_ADDR: c_uint = 0x00000618;

pub const BNX2_MSIX_TABLE_ADDR: c_uint = 0x318000;
pub const BNX2_MSIX_PBA_ADDR: c_uint = 0x31c000;
//
// misc_reg definition
// offset: 0x800
//
pub const BNX2_MISC_COMMAND: c_uint = 0x00000800;

pub const BNX2_MISC_CFG: c_uint = 0x00000804;

pub const BNX2_MISC_ID: c_uint = 0x00000808;

pub const BNX2_MISC_ENABLE_STATUS_BITS: c_uint = 0x0000080c;

pub const BNX2_MISC_ENABLE_SET_BITS: c_uint = 0x00000810;

pub const BNX2_MISC_ENABLE_CLR_BITS: c_uint = 0x00000814;

pub const BNX2_MISC_CLOCK_CONTROL_BITS: c_uint = 0x00000818;

pub const BNX2_MISC_SPIO: c_uint = 0x0000081c;

pub const BNX2_MISC_SPIO_INT: c_uint = 0x00000820;

pub const BNX2_MISC_CONFIG_LFSR: c_uint = 0x00000824;

pub const BNX2_MISC_LFSR_MASK_BITS: c_uint = 0x00000828;

pub const BNX2_MISC_ARB_REQ0: c_uint = 0x0000082c;
pub const BNX2_MISC_ARB_REQ1: c_uint = 0x00000830;
pub const BNX2_MISC_ARB_REQ2: c_uint = 0x00000834;
pub const BNX2_MISC_ARB_REQ3: c_uint = 0x00000838;
pub const BNX2_MISC_ARB_REQ4: c_uint = 0x0000083c;
pub const BNX2_MISC_ARB_FREE0: c_uint = 0x00000840;
pub const BNX2_MISC_ARB_FREE1: c_uint = 0x00000844;
pub const BNX2_MISC_ARB_FREE2: c_uint = 0x00000848;
pub const BNX2_MISC_ARB_FREE3: c_uint = 0x0000084c;
pub const BNX2_MISC_ARB_FREE4: c_uint = 0x00000850;
pub const BNX2_MISC_ARB_REQ_STATUS0: c_uint = 0x00000854;
pub const BNX2_MISC_ARB_REQ_STATUS1: c_uint = 0x00000858;
pub const BNX2_MISC_ARB_REQ_STATUS2: c_uint = 0x0000085c;
pub const BNX2_MISC_ARB_REQ_STATUS3: c_uint = 0x00000860;
pub const BNX2_MISC_ARB_REQ_STATUS4: c_uint = 0x00000864;
pub const BNX2_MISC_ARB_GNT0: c_uint = 0x00000868;

pub const BNX2_MISC_ARB_GNT1: c_uint = 0x0000086c;

pub const BNX2_MISC_ARB_GNT2: c_uint = 0x00000870;

pub const BNX2_MISC_ARB_GNT3: c_uint = 0x00000874;

pub const BNX2_MISC_RESERVED1: c_uint = 0x00000878;

pub const BNX2_MISC_RESERVED2: c_uint = 0x0000087c;

pub const BNX2_MISC_SM_ASF_CONTROL: c_uint = 0x00000880;

pub const BNX2_MISC_SMB_IN: c_uint = 0x00000884;

pub const BNX2_MISC_SMB_OUT: c_uint = 0x00000888;

pub const BNX2_MISC_SMB_WATCHDOG: c_uint = 0x0000088c;

pub const BNX2_MISC_SMB_HEARTBEAT: c_uint = 0x00000890;

pub const BNX2_MISC_SMB_POLL_ASF: c_uint = 0x00000894;

pub const BNX2_MISC_SMB_POLL_LEGACY: c_uint = 0x00000898;

pub const BNX2_MISC_SMB_RETRAN: c_uint = 0x0000089c;

pub const BNX2_MISC_SMB_TIMESTAMP: c_uint = 0x000008a0;

pub const BNX2_MISC_PERR_ENA0: c_uint = 0x000008a4;

pub const BNX2_MISC_PERR_ENA1: c_uint = 0x000008a8;

pub const BNX2_MISC_PERR_ENA2: c_uint = 0x000008ac;

pub const BNX2_MISC_DEBUG_VECTOR_SEL: c_uint = 0x000008b0;

pub const BNX2_MISC_VREG_CONTROL: c_uint = 0x000008b4;

pub const BNX2_MISC_FINAL_CLK_CTL_VAL: c_uint = 0x000008b8;

pub const BNX2_MISC_GP_HW_CTL0: c_uint = 0x000008bc;

pub const BNX2_MISC_GP_HW_CTL1: c_uint = 0x000008c0;

pub const BNX2_MISC_NEW_HW_CTL: c_uint = 0x000008c4;

pub const BNX2_MISC_NEW_CORE_CTL: c_uint = 0x000008c8;

pub const BNX2_MISC_ECO_HW_CTL: c_uint = 0x000008cc;

pub const BNX2_MISC_ECO_CORE_CTL: c_uint = 0x000008d0;

pub const BNX2_MISC_PPIO: c_uint = 0x000008d4;

pub const BNX2_MISC_PPIO_INT: c_uint = 0x000008d8;

pub const BNX2_MISC_RESET_NUMS: c_uint = 0x000008dc;

pub const BNX2_MISC_CS16_ERR: c_uint = 0x000008e0;

pub const BNX2_MISC_SPIO_EVENT: c_uint = 0x000008e4;

pub const BNX2_MISC_PPIO_EVENT: c_uint = 0x000008e8;

pub const BNX2_MISC_DUAL_MEDIA_CTRL: c_uint = 0x000008ec;

pub const BNX2_MISC_OTP_CMD1: c_uint = 0x000008f0;

pub const BNX2_MISC_OTP_CMD2: c_uint = 0x000008f4;

pub const BNX2_MISC_OTP_STATUS: c_uint = 0x000008f8;

pub const BNX2_MISC_OTP_SHIFT1_CMD: c_uint = 0x000008fc;

pub const BNX2_MISC_OTP_SHIFT1_DATA: c_uint = 0x00000900;
pub const BNX2_MISC_OTP_SHIFT2_CMD: c_uint = 0x00000904;

pub const BNX2_MISC_OTP_SHIFT2_DATA: c_uint = 0x00000908;
pub const BNX2_MISC_BIST_CS0: c_uint = 0x0000090c;

pub const BNX2_MISC_BIST_MEMSTATUS0: c_uint = 0x00000910;
pub const BNX2_MISC_BIST_CS1: c_uint = 0x00000914;

pub const BNX2_MISC_BIST_MEMSTATUS1: c_uint = 0x00000918;
pub const BNX2_MISC_BIST_CS2: c_uint = 0x0000091c;

pub const BNX2_MISC_BIST_MEMSTATUS2: c_uint = 0x00000920;
pub const BNX2_MISC_BIST_CS3: c_uint = 0x00000924;

pub const BNX2_MISC_BIST_MEMSTATUS3: c_uint = 0x00000928;
pub const BNX2_MISC_BIST_CS4: c_uint = 0x0000092c;

pub const BNX2_MISC_BIST_MEMSTATUS4: c_uint = 0x00000930;
pub const BNX2_MISC_BIST_CS5: c_uint = 0x00000934;

pub const BNX2_MISC_BIST_MEMSTATUS5: c_uint = 0x00000938;
pub const BNX2_MISC_MEM_TM0: c_uint = 0x0000093c;

pub const BNX2_MISC_USPLL_CTRL: c_uint = 0x00000940;

pub const BNX2_MISC_PERR_STATUS0: c_uint = 0x00000944;

pub const BNX2_MISC_PERR_STATUS1: c_uint = 0x00000948;

pub const BNX2_MISC_PERR_STATUS2: c_uint = 0x0000094c;

pub const BNX2_MISC_LCPLL_CTRL0: c_uint = 0x00000950;

pub const BNX2_MISC_LCPLL_CTRL1: c_uint = 0x00000954;

pub const BNX2_MISC_LCPLL_STATUS: c_uint = 0x00000958;

pub const BNX2_MISC_OSCFUNDS_CTRL: c_uint = 0x0000095c;

//
// nvm_reg definition
// offset: 0x6400
//
pub const BNX2_NVM_COMMAND: c_uint = 0x00006400;

pub const BNX2_NVM_STATUS: c_uint = 0x00006404;

pub const BNX2_NVM_WRITE: c_uint = 0x00006408;

pub const BNX2_NVM_ADDR: c_uint = 0x0000640c;

pub const BNX2_NVM_READ: c_uint = 0x00006410;

pub const BNX2_NVM_CFG1: c_uint = 0x00006414;

pub const BNX2_NVM_CFG2: c_uint = 0x00006418;

pub const BNX2_NVM_CFG3: c_uint = 0x0000641c;

pub const BNX2_NVM_SW_ARB: c_uint = 0x00006420;

pub const BNX2_NVM_ACCESS_ENABLE: c_uint = 0x00006424;

pub const BNX2_NVM_WRITE1: c_uint = 0x00006428;

pub const BNX2_NVM_CFG4: c_uint = 0x0000642c;

pub const BNX2_NVM_RECONFIG: c_uint = 0x00006430;

//
// dma_reg definition
// offset: 0xc00
//
pub const BNX2_DMA_COMMAND: c_uint = 0x00000c00;

pub const BNX2_DMA_STATUS: c_uint = 0x00000c04;

pub const BNX2_DMA_CONFIG: c_uint = 0x00000c08;

pub const BNX2_DMA_BLACKOUT: c_uint = 0x00000c0c;

pub const BNX2_DMA_READ_MASTER_SETTING_0: c_uint = 0x00000c10;

pub const BNX2_DMA_READ_MASTER_SETTING_1: c_uint = 0x00000c14;

pub const BNX2_DMA_WRITE_MASTER_SETTING_0: c_uint = 0x00000c18;

pub const BNX2_DMA_WRITE_MASTER_SETTING_1: c_uint = 0x00000c1c;

pub const BNX2_DMA_ARBITER: c_uint = 0x00000c20;

pub const BNX2_DMA_ARB_TIMERS: c_uint = 0x00000c24;

pub const BNX2_DMA_DEBUG_VECT_PEEK: c_uint = 0x00000c2c;

pub const BNX2_DMA_TAG_RAM_00: c_uint = 0x00000c30;

pub const BNX2_DMA_TAG_RAM_01: c_uint = 0x00000c34;

pub const BNX2_DMA_TAG_RAM_02: c_uint = 0x00000c38;

pub const BNX2_DMA_TAG_RAM_03: c_uint = 0x00000c3c;

pub const BNX2_DMA_TAG_RAM_04: c_uint = 0x00000c40;

pub const BNX2_DMA_TAG_RAM_05: c_uint = 0x00000c44;

pub const BNX2_DMA_TAG_RAM_06: c_uint = 0x00000c48;

pub const BNX2_DMA_TAG_RAM_07: c_uint = 0x00000c4c;

pub const BNX2_DMA_TAG_RAM_08: c_uint = 0x00000c50;

pub const BNX2_DMA_TAG_RAM_09: c_uint = 0x00000c54;

pub const BNX2_DMA_TAG_RAM_10: c_uint = 0x00000c58;

pub const BNX2_DMA_TAG_RAM_11: c_uint = 0x00000c5c;

pub const BNX2_DMA_RCHAN_STAT_22: c_uint = 0x00000c60;
pub const BNX2_DMA_RCHAN_STAT_30: c_uint = 0x00000c64;
pub const BNX2_DMA_RCHAN_STAT_31: c_uint = 0x00000c68;
pub const BNX2_DMA_RCHAN_STAT_32: c_uint = 0x00000c6c;
pub const BNX2_DMA_RCHAN_STAT_40: c_uint = 0x00000c70;
pub const BNX2_DMA_RCHAN_STAT_41: c_uint = 0x00000c74;
pub const BNX2_DMA_RCHAN_STAT_42: c_uint = 0x00000c78;
pub const BNX2_DMA_RCHAN_STAT_50: c_uint = 0x00000c7c;
pub const BNX2_DMA_RCHAN_STAT_51: c_uint = 0x00000c80;
pub const BNX2_DMA_RCHAN_STAT_52: c_uint = 0x00000c84;
pub const BNX2_DMA_RCHAN_STAT_60: c_uint = 0x00000c88;
pub const BNX2_DMA_RCHAN_STAT_61: c_uint = 0x00000c8c;
pub const BNX2_DMA_RCHAN_STAT_62: c_uint = 0x00000c90;
pub const BNX2_DMA_RCHAN_STAT_70: c_uint = 0x00000c94;
pub const BNX2_DMA_RCHAN_STAT_71: c_uint = 0x00000c98;
pub const BNX2_DMA_RCHAN_STAT_72: c_uint = 0x00000c9c;
pub const BNX2_DMA_WCHAN_STAT_00: c_uint = 0x00000ca0;

pub const BNX2_DMA_WCHAN_STAT_01: c_uint = 0x00000ca4;

pub const BNX2_DMA_WCHAN_STAT_02: c_uint = 0x00000ca8;

pub const BNX2_DMA_WCHAN_STAT_10: c_uint = 0x00000cac;
pub const BNX2_DMA_WCHAN_STAT_11: c_uint = 0x00000cb0;
pub const BNX2_DMA_WCHAN_STAT_12: c_uint = 0x00000cb4;
pub const BNX2_DMA_WCHAN_STAT_20: c_uint = 0x00000cb8;
pub const BNX2_DMA_WCHAN_STAT_21: c_uint = 0x00000cbc;
pub const BNX2_DMA_WCHAN_STAT_22: c_uint = 0x00000cc0;
pub const BNX2_DMA_WCHAN_STAT_30: c_uint = 0x00000cc4;
pub const BNX2_DMA_WCHAN_STAT_31: c_uint = 0x00000cc8;
pub const BNX2_DMA_WCHAN_STAT_32: c_uint = 0x00000ccc;
pub const BNX2_DMA_WCHAN_STAT_40: c_uint = 0x00000cd0;
pub const BNX2_DMA_WCHAN_STAT_41: c_uint = 0x00000cd4;
pub const BNX2_DMA_WCHAN_STAT_42: c_uint = 0x00000cd8;
pub const BNX2_DMA_WCHAN_STAT_50: c_uint = 0x00000cdc;
pub const BNX2_DMA_WCHAN_STAT_51: c_uint = 0x00000ce0;
pub const BNX2_DMA_WCHAN_STAT_52: c_uint = 0x00000ce4;
pub const BNX2_DMA_WCHAN_STAT_60: c_uint = 0x00000ce8;
pub const BNX2_DMA_WCHAN_STAT_61: c_uint = 0x00000cec;
pub const BNX2_DMA_WCHAN_STAT_62: c_uint = 0x00000cf0;
pub const BNX2_DMA_WCHAN_STAT_70: c_uint = 0x00000cf4;
pub const BNX2_DMA_WCHAN_STAT_71: c_uint = 0x00000cf8;
pub const BNX2_DMA_WCHAN_STAT_72: c_uint = 0x00000cfc;
pub const BNX2_DMA_ARB_STAT_00: c_uint = 0x00000d00;

pub const BNX2_DMA_ARB_STAT_01: c_uint = 0x00000d04;

pub const BNX2_DMA_FUSE_CTRL0_CMD: c_uint = 0x00000f00;

pub const BNX2_DMA_FUSE_CTRL0_DATA: c_uint = 0x00000f04;
pub const BNX2_DMA_FUSE_CTRL1_CMD: c_uint = 0x00000f08;

pub const BNX2_DMA_FUSE_CTRL1_DATA: c_uint = 0x00000f0c;
pub const BNX2_DMA_FUSE_CTRL2_CMD: c_uint = 0x00000f10;

pub const BNX2_DMA_FUSE_CTRL2_DATA: c_uint = 0x00000f14;
//
// context_reg definition
// offset: 0x1000
//
pub const BNX2_CTX_COMMAND: c_uint = 0x00001000;

pub const BNX2_CTX_STATUS: c_uint = 0x00001004;

pub const BNX2_CTX_VIRT_ADDR: c_uint = 0x00001008;

pub const BNX2_CTX_PAGE_TBL: c_uint = 0x0000100c;

pub const BNX2_CTX_DATA_ADR: c_uint = 0x00001010;

pub const BNX2_CTX_DATA: c_uint = 0x00001014;
pub const BNX2_CTX_LOCK: c_uint = 0x00001018;

pub const BNX2_CTX_CTX_CTRL: c_uint = 0x0000101c;

pub const BNX2_CTX_CTX_DATA: c_uint = 0x00001020;
pub const BNX2_CTX_ACCESS_STATUS: c_uint = 0x00001040;

pub const BNX2_CTX_DBG_LOCK_STATUS: c_uint = 0x00001044;

pub const BNX2_CTX_CACHE_CTRL_STATUS: c_uint = 0x00001048;

pub const BNX2_CTX_CACHE_CTRL_SM_STATUS: c_uint = 0x0000104c;

pub const BNX2_CTX_CACHE_STATUS: c_uint = 0x00001050;

pub const BNX2_CTX_DMA_STATUS: c_uint = 0x00001054;

pub const BNX2_CTX_REP_STATUS: c_uint = 0x00001058;

pub const BNX2_CTX_CKSUM_ERROR_STATUS: c_uint = 0x0000105c;

pub const BNX2_CTX_CHNL_LOCK_STATUS_0: c_uint = 0x00001080;

pub const BNX2_CTX_CHNL_LOCK_STATUS_1: c_uint = 0x00001084;
pub const BNX2_CTX_CHNL_LOCK_STATUS_2: c_uint = 0x00001088;
pub const BNX2_CTX_CHNL_LOCK_STATUS_3: c_uint = 0x0000108c;
pub const BNX2_CTX_CHNL_LOCK_STATUS_4: c_uint = 0x00001090;
pub const BNX2_CTX_CHNL_LOCK_STATUS_5: c_uint = 0x00001094;
pub const BNX2_CTX_CHNL_LOCK_STATUS_6: c_uint = 0x00001098;
pub const BNX2_CTX_CHNL_LOCK_STATUS_7: c_uint = 0x0000109c;
pub const BNX2_CTX_CHNL_LOCK_STATUS_8: c_uint = 0x000010a0;
pub const BNX2_CTX_CHNL_LOCK_STATUS_9: c_uint = 0x000010a4;
pub const BNX2_CTX_CACHE_DATA: c_uint = 0x000010c4;
pub const BNX2_CTX_HOST_PAGE_TBL_CTRL: c_uint = 0x000010c8;

pub const BNX2_CTX_HOST_PAGE_TBL_DATA0: c_uint = 0x000010cc;

pub const BNX2_CTX_HOST_PAGE_TBL_DATA1: c_uint = 0x000010d0;
pub const BNX2_CTX_CAM_CTRL: c_uint = 0x000010d4;

//
// emac_reg definition
// offset: 0x1400
//
pub const BNX2_EMAC_MODE: c_uint = 0x00001400;

pub const BNX2_EMAC_STATUS: c_uint = 0x00001404;

pub const BNX2_EMAC_ATTENTION_ENA: c_uint = 0x00001408;

pub const BNX2_EMAC_LED: c_uint = 0x0000140c;

pub const BNX2_EMAC_MAC_MATCH0: c_uint = 0x00001410;
pub const BNX2_EMAC_MAC_MATCH1: c_uint = 0x00001414;
pub const BNX2_EMAC_MAC_MATCH2: c_uint = 0x00001418;
pub const BNX2_EMAC_MAC_MATCH3: c_uint = 0x0000141c;
pub const BNX2_EMAC_MAC_MATCH4: c_uint = 0x00001420;
pub const BNX2_EMAC_MAC_MATCH5: c_uint = 0x00001424;
pub const BNX2_EMAC_MAC_MATCH6: c_uint = 0x00001428;
pub const BNX2_EMAC_MAC_MATCH7: c_uint = 0x0000142c;
pub const BNX2_EMAC_MAC_MATCH8: c_uint = 0x00001430;
pub const BNX2_EMAC_MAC_MATCH9: c_uint = 0x00001434;
pub const BNX2_EMAC_MAC_MATCH10: c_uint = 0x00001438;
pub const BNX2_EMAC_MAC_MATCH11: c_uint = 0x0000143c;
pub const BNX2_EMAC_MAC_MATCH12: c_uint = 0x00001440;
pub const BNX2_EMAC_MAC_MATCH13: c_uint = 0x00001444;
pub const BNX2_EMAC_MAC_MATCH14: c_uint = 0x00001448;
pub const BNX2_EMAC_MAC_MATCH15: c_uint = 0x0000144c;
pub const BNX2_EMAC_MAC_MATCH16: c_uint = 0x00001450;
pub const BNX2_EMAC_MAC_MATCH17: c_uint = 0x00001454;
pub const BNX2_EMAC_MAC_MATCH18: c_uint = 0x00001458;
pub const BNX2_EMAC_MAC_MATCH19: c_uint = 0x0000145c;
pub const BNX2_EMAC_MAC_MATCH20: c_uint = 0x00001460;
pub const BNX2_EMAC_MAC_MATCH21: c_uint = 0x00001464;
pub const BNX2_EMAC_MAC_MATCH22: c_uint = 0x00001468;
pub const BNX2_EMAC_MAC_MATCH23: c_uint = 0x0000146c;
pub const BNX2_EMAC_MAC_MATCH24: c_uint = 0x00001470;
pub const BNX2_EMAC_MAC_MATCH25: c_uint = 0x00001474;
pub const BNX2_EMAC_MAC_MATCH26: c_uint = 0x00001478;
pub const BNX2_EMAC_MAC_MATCH27: c_uint = 0x0000147c;
pub const BNX2_EMAC_MAC_MATCH28: c_uint = 0x00001480;
pub const BNX2_EMAC_MAC_MATCH29: c_uint = 0x00001484;
pub const BNX2_EMAC_MAC_MATCH30: c_uint = 0x00001488;
pub const BNX2_EMAC_MAC_MATCH31: c_uint = 0x0000148c;
pub const BNX2_EMAC_BACKOFF_SEED: c_uint = 0x00001498;

pub const BNX2_EMAC_RX_MTU_SIZE: c_uint = 0x0000149c;

pub const BNX2_EMAC_SERDES_CNTL: c_uint = 0x000014a4;

pub const BNX2_EMAC_SERDES_STATUS: c_uint = 0x000014a8;

pub const BNX2_EMAC_MDIO_COMM: c_uint = 0x000014ac;

pub const BNX2_EMAC_MDIO_STATUS: c_uint = 0x000014b0;

pub const BNX2_EMAC_MDIO_MODE: c_uint = 0x000014b4;

pub const BNX2_EMAC_MDIO_AUTO_STATUS: c_uint = 0x000014b8;

pub const BNX2_EMAC_TX_MODE: c_uint = 0x000014bc;

pub const BNX2_EMAC_TX_STATUS: c_uint = 0x000014c0;

pub const BNX2_EMAC_TX_LENGTHS: c_uint = 0x000014c4;

pub const BNX2_EMAC_RX_MODE: c_uint = 0x000014c8;

pub const BNX2_EMAC_RX_STATUS: c_uint = 0x000014cc;

pub const BNX2_EMAC_MULTICAST_HASH0: c_uint = 0x000014d0;
pub const BNX2_EMAC_MULTICAST_HASH1: c_uint = 0x000014d4;
pub const BNX2_EMAC_MULTICAST_HASH2: c_uint = 0x000014d8;
pub const BNX2_EMAC_MULTICAST_HASH3: c_uint = 0x000014dc;
pub const BNX2_EMAC_MULTICAST_HASH4: c_uint = 0x000014e0;
pub const BNX2_EMAC_MULTICAST_HASH5: c_uint = 0x000014e4;
pub const BNX2_EMAC_MULTICAST_HASH6: c_uint = 0x000014e8;
pub const BNX2_EMAC_MULTICAST_HASH7: c_uint = 0x000014ec;
pub const BNX2_EMAC_CKSUM_ERROR_STATUS: c_uint = 0x000014f0;

pub const BNX2_EMAC_RX_STAT_IFHCINOCTETS: c_uint = 0x00001500;
pub const BNX2_EMAC_RX_STAT_IFHCINBADOCTETS: c_uint = 0x00001504;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSFRAGMENTS: c_uint = 0x00001508;
pub const BNX2_EMAC_RX_STAT_IFHCINUCASTPKTS: c_uint = 0x0000150c;
pub const BNX2_EMAC_RX_STAT_IFHCINMULTICASTPKTS: c_uint = 0x00001510;
pub const BNX2_EMAC_RX_STAT_IFHCINBROADCASTPKTS: c_uint = 0x00001514;
pub const BNX2_EMAC_RX_STAT_DOT3STATSFCSERRORS: c_uint = 0x00001518;
pub const BNX2_EMAC_RX_STAT_DOT3STATSALIGNMENTERRORS: c_uint = 0x0000151c;
pub const BNX2_EMAC_RX_STAT_DOT3STATSCARRIERSENSEERRORS: c_uint = 0x00001520;
pub const BNX2_EMAC_RX_STAT_XONPAUSEFRAMESRECEIVED: c_uint = 0x00001524;
pub const BNX2_EMAC_RX_STAT_XOFFPAUSEFRAMESRECEIVED: c_uint = 0x00001528;
pub const BNX2_EMAC_RX_STAT_MACCONTROLFRAMESRECEIVED: c_uint = 0x0000152c;
pub const BNX2_EMAC_RX_STAT_XOFFSTATEENTERED: c_uint = 0x00001530;
pub const BNX2_EMAC_RX_STAT_DOT3STATSFRAMESTOOLONG: c_uint = 0x00001534;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSJABBERS: c_uint = 0x00001538;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSUNDERSIZEPKTS: c_uint = 0x0000153c;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSPKTS64OCTETS: c_uint = 0x00001540;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSPKTS65OCTETSTO127OCTETS: c_uint = 0x00001544;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSPKTS128OCTETSTO255OCTETS: c_uint = 0x00001548;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSPKTS256OCTETSTO511OCTETS: c_uint = 0x0000154c;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSPKTS512OCTETSTO1023OCTETS: c_uint = 0x00001550;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSPKTS1024OCTETSTO1522OCTETS: c_uint = 0x00001554;
pub const BNX2_EMAC_RX_STAT_ETHERSTATSPKTSOVER1522OCTETS: c_uint = 0x00001558;
pub const BNX2_EMAC_RXMAC_DEBUG0: c_uint = 0x0000155c;
pub const BNX2_EMAC_RXMAC_DEBUG1: c_uint = 0x00001560;

pub const BNX2_EMAC_RXMAC_DEBUG2: c_uint = 0x00001564;

pub const BNX2_EMAC_RXMAC_DEBUG3: c_uint = 0x00001568;

pub const BNX2_EMAC_RXMAC_DEBUG4: c_uint = 0x0000156c;

pub const BNX2_EMAC_RXMAC_DEBUG5: c_uint = 0x00001570;

pub const BNX2_EMAC_RX_STAT_FALSECARRIERERRORS: c_uint = 0x00001574;
pub const BNX2_EMAC_RX_STAT_AC0: c_uint = 0x00001580;
pub const BNX2_EMAC_RX_STAT_AC1: c_uint = 0x00001584;
pub const BNX2_EMAC_RX_STAT_AC2: c_uint = 0x00001588;
pub const BNX2_EMAC_RX_STAT_AC3: c_uint = 0x0000158c;
pub const BNX2_EMAC_RX_STAT_AC4: c_uint = 0x00001590;
pub const BNX2_EMAC_RX_STAT_AC5: c_uint = 0x00001594;
pub const BNX2_EMAC_RX_STAT_AC6: c_uint = 0x00001598;
pub const BNX2_EMAC_RX_STAT_AC7: c_uint = 0x0000159c;
pub const BNX2_EMAC_RX_STAT_AC8: c_uint = 0x000015a0;
pub const BNX2_EMAC_RX_STAT_AC9: c_uint = 0x000015a4;
pub const BNX2_EMAC_RX_STAT_AC10: c_uint = 0x000015a8;
pub const BNX2_EMAC_RX_STAT_AC11: c_uint = 0x000015ac;
pub const BNX2_EMAC_RX_STAT_AC12: c_uint = 0x000015b0;
pub const BNX2_EMAC_RX_STAT_AC13: c_uint = 0x000015b4;
pub const BNX2_EMAC_RX_STAT_AC14: c_uint = 0x000015b8;
pub const BNX2_EMAC_RX_STAT_AC15: c_uint = 0x000015bc;
pub const BNX2_EMAC_RX_STAT_AC16: c_uint = 0x000015c0;
pub const BNX2_EMAC_RX_STAT_AC17: c_uint = 0x000015c4;
pub const BNX2_EMAC_RX_STAT_AC18: c_uint = 0x000015c8;
pub const BNX2_EMAC_RX_STAT_AC19: c_uint = 0x000015cc;
pub const BNX2_EMAC_RX_STAT_AC20: c_uint = 0x000015d0;
pub const BNX2_EMAC_RX_STAT_AC21: c_uint = 0x000015d4;
pub const BNX2_EMAC_RX_STAT_AC22: c_uint = 0x000015d8;
pub const BNX2_EMAC_RXMAC_SUC_DBG_OVERRUNVEC: c_uint = 0x000015dc;
pub const BNX2_EMAC_RX_STAT_AC_28: c_uint = 0x000015f4;
pub const BNX2_EMAC_TX_STAT_IFHCOUTOCTETS: c_uint = 0x00001600;
pub const BNX2_EMAC_TX_STAT_IFHCOUTBADOCTETS: c_uint = 0x00001604;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSCOLLISIONS: c_uint = 0x00001608;
pub const BNX2_EMAC_TX_STAT_OUTXONSENT: c_uint = 0x0000160c;
pub const BNX2_EMAC_TX_STAT_OUTXOFFSENT: c_uint = 0x00001610;
pub const BNX2_EMAC_TX_STAT_FLOWCONTROLDONE: c_uint = 0x00001614;
pub const BNX2_EMAC_TX_STAT_DOT3STATSSINGLECOLLISIONFRAMES: c_uint = 0x00001618;
pub const BNX2_EMAC_TX_STAT_DOT3STATSMULTIPLECOLLISIONFRAMES: c_uint = 0x0000161c;
pub const BNX2_EMAC_TX_STAT_DOT3STATSDEFERREDTRANSMISSIONS: c_uint = 0x00001620;
pub const BNX2_EMAC_TX_STAT_DOT3STATSEXCESSIVECOLLISIONS: c_uint = 0x00001624;
pub const BNX2_EMAC_TX_STAT_DOT3STATSLATECOLLISIONS: c_uint = 0x00001628;
pub const BNX2_EMAC_TX_STAT_IFHCOUTUCASTPKTS: c_uint = 0x0000162c;
pub const BNX2_EMAC_TX_STAT_IFHCOUTMULTICASTPKTS: c_uint = 0x00001630;
pub const BNX2_EMAC_TX_STAT_IFHCOUTBROADCASTPKTS: c_uint = 0x00001634;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSPKTS64OCTETS: c_uint = 0x00001638;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSPKTS65OCTETSTO127OCTETS: c_uint = 0x0000163c;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSPKTS128OCTETSTO255OCTETS: c_uint = 0x00001640;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSPKTS256OCTETSTO511OCTETS: c_uint = 0x00001644;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSPKTS512OCTETSTO1023OCTETS: c_uint = 0x00001648;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSPKTS1024OCTETSTO1522OCTETS: c_uint = 0x0000164c;
pub const BNX2_EMAC_TX_STAT_ETHERSTATSPKTSOVER1522OCTETS: c_uint = 0x00001650;
pub const BNX2_EMAC_TX_STAT_DOT3STATSINTERNALMACTRANSMITERRORS: c_uint = 0x00001654;
pub const BNX2_EMAC_TXMAC_DEBUG0: c_uint = 0x00001658;
pub const BNX2_EMAC_TXMAC_DEBUG1: c_uint = 0x0000165c;

pub const BNX2_EMAC_TXMAC_DEBUG2: c_uint = 0x00001660;

pub const BNX2_EMAC_TXMAC_DEBUG3: c_uint = 0x00001664;

pub const BNX2_EMAC_TXMAC_DEBUG4: c_uint = 0x00001668;

pub const BNX2_EMAC_TX_STAT_AC0: c_uint = 0x00001680;
pub const BNX2_EMAC_TX_STAT_AC1: c_uint = 0x00001684;
pub const BNX2_EMAC_TX_STAT_AC2: c_uint = 0x00001688;
pub const BNX2_EMAC_TX_STAT_AC3: c_uint = 0x0000168c;
pub const BNX2_EMAC_TX_STAT_AC4: c_uint = 0x00001690;
pub const BNX2_EMAC_TX_STAT_AC5: c_uint = 0x00001694;
pub const BNX2_EMAC_TX_STAT_AC6: c_uint = 0x00001698;
pub const BNX2_EMAC_TX_STAT_AC7: c_uint = 0x0000169c;
pub const BNX2_EMAC_TX_STAT_AC8: c_uint = 0x000016a0;
pub const BNX2_EMAC_TX_STAT_AC9: c_uint = 0x000016a4;
pub const BNX2_EMAC_TX_STAT_AC10: c_uint = 0x000016a8;
pub const BNX2_EMAC_TX_STAT_AC11: c_uint = 0x000016ac;
pub const BNX2_EMAC_TX_STAT_AC12: c_uint = 0x000016b0;
pub const BNX2_EMAC_TX_STAT_AC13: c_uint = 0x000016b4;
pub const BNX2_EMAC_TX_STAT_AC14: c_uint = 0x000016b8;
pub const BNX2_EMAC_TX_STAT_AC15: c_uint = 0x000016bc;
pub const BNX2_EMAC_TX_STAT_AC16: c_uint = 0x000016c0;
pub const BNX2_EMAC_TX_STAT_AC17: c_uint = 0x000016c4;
pub const BNX2_EMAC_TX_STAT_AC18: c_uint = 0x000016c8;
pub const BNX2_EMAC_TX_STAT_AC19: c_uint = 0x000016cc;
pub const BNX2_EMAC_TX_STAT_AC20: c_uint = 0x000016d0;
pub const BNX2_EMAC_TXMAC_SUC_DBG_OVERRUNVEC: c_uint = 0x000016d8;
pub const BNX2_EMAC_TX_RATE_LIMIT_CTRL: c_uint = 0x000016fc;

//
// rpm_reg definition
// offset: 0x1800
//
pub const BNX2_RPM_COMMAND: c_uint = 0x00001800;

pub const BNX2_RPM_STATUS: c_uint = 0x00001804;

pub const BNX2_RPM_CONFIG: c_uint = 0x00001808;

pub const BNX2_RPM_MGMT_PKT_CTRL: c_uint = 0x0000180c;

pub const BNX2_RPM_VLAN_MATCH0: c_uint = 0x00001810;

pub const BNX2_RPM_VLAN_MATCH1: c_uint = 0x00001814;

pub const BNX2_RPM_VLAN_MATCH2: c_uint = 0x00001818;

pub const BNX2_RPM_VLAN_MATCH3: c_uint = 0x0000181c;

pub const BNX2_RPM_SORT_USER0: c_uint = 0x00001820;

pub const BNX2_RPM_SORT_USER1: c_uint = 0x00001824;

pub const BNX2_RPM_SORT_USER2: c_uint = 0x00001828;

pub const BNX2_RPM_SORT_USER3: c_uint = 0x0000182c;

pub const BNX2_RPM_STAT_L2_FILTER_DISCARDS: c_uint = 0x00001840;
pub const BNX2_RPM_STAT_RULE_CHECKER_DISCARDS: c_uint = 0x00001844;
pub const BNX2_RPM_STAT_IFINFTQDISCARDS: c_uint = 0x00001848;
pub const BNX2_RPM_STAT_IFINMBUFDISCARD: c_uint = 0x0000184c;
pub const BNX2_RPM_STAT_RULE_CHECKER_P4_HIT: c_uint = 0x00001850;
pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION0: c_uint = 0x00001854;

pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION1: c_uint = 0x00001858;

pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION2: c_uint = 0x0000185c;

pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION3: c_uint = 0x00001860;

pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION4: c_uint = 0x00001864;

pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION5: c_uint = 0x00001868;

pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION6: c_uint = 0x0000186c;

pub const BNX2_RPM_IPV6_PROGRAMMABLE_EXTENSION7: c_uint = 0x00001870;

pub const BNX2_RPM_STAT_AC0: c_uint = 0x00001880;
pub const BNX2_RPM_STAT_AC1: c_uint = 0x00001884;
pub const BNX2_RPM_STAT_AC2: c_uint = 0x00001888;
pub const BNX2_RPM_STAT_AC3: c_uint = 0x0000188c;
pub const BNX2_RPM_STAT_AC4: c_uint = 0x00001890;
pub const BNX2_RPM_RC_CNTL_16: c_uint = 0x000018e0;

pub const BNX2_RPM_RC_VALUE_MASK_16: c_uint = 0x000018e4;

pub const BNX2_RPM_RC_CNTL_17: c_uint = 0x000018e8;

pub const BNX2_RPM_RC_VALUE_MASK_17: c_uint = 0x000018ec;

pub const BNX2_RPM_RC_CNTL_18: c_uint = 0x000018f0;

pub const BNX2_RPM_RC_VALUE_MASK_18: c_uint = 0x000018f4;

pub const BNX2_RPM_RC_CNTL_19: c_uint = 0x000018f8;

pub const BNX2_RPM_RC_VALUE_MASK_19: c_uint = 0x000018fc;

pub const BNX2_RPM_RC_CNTL_0: c_uint = 0x00001900;

pub const BNX2_RPM_RC_VALUE_MASK_0: c_uint = 0x00001904;

pub const BNX2_RPM_RC_CNTL_1: c_uint = 0x00001908;

pub const BNX2_RPM_RC_VALUE_MASK_1: c_uint = 0x0000190c;

pub const BNX2_RPM_RC_CNTL_2: c_uint = 0x00001910;

pub const BNX2_RPM_RC_VALUE_MASK_2: c_uint = 0x00001914;

pub const BNX2_RPM_RC_CNTL_3: c_uint = 0x00001918;

pub const BNX2_RPM_RC_VALUE_MASK_3: c_uint = 0x0000191c;

pub const BNX2_RPM_RC_CNTL_4: c_uint = 0x00001920;

pub const BNX2_RPM_RC_VALUE_MASK_4: c_uint = 0x00001924;

pub const BNX2_RPM_RC_CNTL_5: c_uint = 0x00001928;

pub const BNX2_RPM_RC_VALUE_MASK_5: c_uint = 0x0000192c;

pub const BNX2_RPM_RC_CNTL_6: c_uint = 0x00001930;

pub const BNX2_RPM_RC_VALUE_MASK_6: c_uint = 0x00001934;

pub const BNX2_RPM_RC_CNTL_7: c_uint = 0x00001938;

pub const BNX2_RPM_RC_VALUE_MASK_7: c_uint = 0x0000193c;

pub const BNX2_RPM_RC_CNTL_8: c_uint = 0x00001940;

pub const BNX2_RPM_RC_VALUE_MASK_8: c_uint = 0x00001944;

pub const BNX2_RPM_RC_CNTL_9: c_uint = 0x00001948;

pub const BNX2_RPM_RC_VALUE_MASK_9: c_uint = 0x0000194c;

pub const BNX2_RPM_RC_CNTL_10: c_uint = 0x00001950;

pub const BNX2_RPM_RC_VALUE_MASK_10: c_uint = 0x00001954;

pub const BNX2_RPM_RC_CNTL_11: c_uint = 0x00001958;

pub const BNX2_RPM_RC_VALUE_MASK_11: c_uint = 0x0000195c;

pub const BNX2_RPM_RC_CNTL_12: c_uint = 0x00001960;

pub const BNX2_RPM_RC_VALUE_MASK_12: c_uint = 0x00001964;

pub const BNX2_RPM_RC_CNTL_13: c_uint = 0x00001968;

pub const BNX2_RPM_RC_VALUE_MASK_13: c_uint = 0x0000196c;

pub const BNX2_RPM_RC_CNTL_14: c_uint = 0x00001970;

pub const BNX2_RPM_RC_VALUE_MASK_14: c_uint = 0x00001974;

pub const BNX2_RPM_RC_CNTL_15: c_uint = 0x00001978;

pub const BNX2_RPM_RC_VALUE_MASK_15: c_uint = 0x0000197c;

pub const BNX2_RPM_RC_CONFIG: c_uint = 0x00001980;

pub const BNX2_RPM_DEBUG0: c_uint = 0x00001984;

pub const BNX2_RPM_DEBUG1: c_uint = 0x00001988;

pub const BNX2_RPM_DEBUG2: c_uint = 0x0000198c;

pub const BNX2_RPM_DEBUG3: c_uint = 0x00001990;

pub const BNX2_RPM_DEBUG4: c_uint = 0x00001994;

pub const BNX2_RPM_DEBUG5: c_uint = 0x00001998;

pub const BNX2_RPM_DEBUG6: c_uint = 0x0000199c;

pub const BNX2_RPM_DEBUG7: c_uint = 0x000019a0;

pub const BNX2_RPM_DEBUG8: c_uint = 0x000019a4;

pub const BNX2_RPM_DEBUG9: c_uint = 0x000019a8;

pub const BNX2_RPM_ACPI_DBG_BUF_W00: c_uint = 0x000019c0;
pub const BNX2_RPM_ACPI_DBG_BUF_W01: c_uint = 0x000019c4;
pub const BNX2_RPM_ACPI_DBG_BUF_W02: c_uint = 0x000019c8;
pub const BNX2_RPM_ACPI_DBG_BUF_W03: c_uint = 0x000019cc;
pub const BNX2_RPM_ACPI_DBG_BUF_W10: c_uint = 0x000019d0;
pub const BNX2_RPM_ACPI_DBG_BUF_W11: c_uint = 0x000019d4;
pub const BNX2_RPM_ACPI_DBG_BUF_W12: c_uint = 0x000019d8;
pub const BNX2_RPM_ACPI_DBG_BUF_W13: c_uint = 0x000019dc;
pub const BNX2_RPM_ACPI_DBG_BUF_W20: c_uint = 0x000019e0;
pub const BNX2_RPM_ACPI_DBG_BUF_W21: c_uint = 0x000019e4;
pub const BNX2_RPM_ACPI_DBG_BUF_W22: c_uint = 0x000019e8;
pub const BNX2_RPM_ACPI_DBG_BUF_W23: c_uint = 0x000019ec;
pub const BNX2_RPM_ACPI_DBG_BUF_W30: c_uint = 0x000019f0;
pub const BNX2_RPM_ACPI_DBG_BUF_W31: c_uint = 0x000019f4;
pub const BNX2_RPM_ACPI_DBG_BUF_W32: c_uint = 0x000019f8;
pub const BNX2_RPM_ACPI_DBG_BUF_W33: c_uint = 0x000019fc;
pub const BNX2_RPM_ACPI_BYTE_ENABLE_CTRL: c_uint = 0x00001a00;

pub const BNX2_RPM_ACPI_PATTERN_CTRL: c_uint = 0x00001a04;

pub const BNX2_RPM_ACPI_DATA: c_uint = 0x00001a08;

pub const BNX2_RPM_ACPI_PATTERN_LEN0: c_uint = 0x00001a0c;

pub const BNX2_RPM_ACPI_PATTERN_LEN1: c_uint = 0x00001a10;

pub const BNX2_RPM_ACPI_PATTERN_CRC0: c_uint = 0x00001a18;

pub const BNX2_RPM_ACPI_PATTERN_CRC1: c_uint = 0x00001a1c;

pub const BNX2_RPM_ACPI_PATTERN_CRC2: c_uint = 0x00001a20;

pub const BNX2_RPM_ACPI_PATTERN_CRC3: c_uint = 0x00001a24;

pub const BNX2_RPM_ACPI_PATTERN_CRC4: c_uint = 0x00001a28;

pub const BNX2_RPM_ACPI_PATTERN_CRC5: c_uint = 0x00001a2c;

pub const BNX2_RPM_ACPI_PATTERN_CRC6: c_uint = 0x00001a30;

pub const BNX2_RPM_ACPI_PATTERN_CRC7: c_uint = 0x00001a34;

//
// rlup_reg definition
// offset: 0x2000
//
pub const BNX2_RLUP_RSS_CONFIG: c_uint = 0x0000201c;

pub const BNX2_RLUP_RSS_COMMAND: c_uint = 0x00002048;

pub const BNX2_RLUP_RSS_DATA: c_uint = 0x0000204c;
//
// rbuf_reg definition
// offset: 0x200000
//
pub const BNX2_RBUF_COMMAND: c_uint = 0x00200000;

pub const BNX2_RBUF_STATUS1: c_uint = 0x00200004;

pub const BNX2_RBUF_STATUS2: c_uint = 0x00200008;

pub const BNX2_RBUF_CONFIG: c_uint = 0x0020000c;

pub const BNX2_RBUF_FW_BUF_ALLOC: c_uint = 0x00200010;

pub const BNX2_RBUF_FW_BUF_FREE: c_uint = 0x00200014;

pub const BNX2_RBUF_FW_BUF_SEL: c_uint = 0x00200018;

pub const BNX2_RBUF_CONFIG2: c_uint = 0x0020001c;

pub const BNX2_RBUF_CONFIG3: c_uint = 0x00200020;

pub const BNX2_RBUF_PKT_DATA: c_uint = 0x00208000;
pub const BNX2_RBUF_CLIST_DATA: c_uint = 0x00210000;
pub const BNX2_RBUF_BUF_DATA: c_uint = 0x00220000;
//
// rv2p_reg definition
// offset: 0x2800
//
pub const BNX2_RV2P_COMMAND: c_uint = 0x00002800;

pub const BNX2_RV2P_STATUS: c_uint = 0x00002804;

pub const BNX2_RV2P_CONFIG: c_uint = 0x00002808;

pub const BNX2_RV2P_GEN_BFR_ADDR_0: c_uint = 0x00002810;

pub const BNX2_RV2P_GEN_BFR_ADDR_1: c_uint = 0x00002814;

pub const BNX2_RV2P_GEN_BFR_ADDR_2: c_uint = 0x00002818;

pub const BNX2_RV2P_GEN_BFR_ADDR_3: c_uint = 0x0000281c;

pub const BNX2_RV2P_INSTR_HIGH: c_uint = 0x00002830;

pub const BNX2_RV2P_INSTR_LOW: c_uint = 0x00002834;

pub const BNX2_RV2P_PROC1_ADDR_CMD: c_uint = 0x00002838;

pub const BNX2_RV2P_PROC2_ADDR_CMD: c_uint = 0x0000283c;

pub const BNX2_RV2P_PROC1_GRC_DEBUG: c_uint = 0x00002840;
pub const BNX2_RV2P_PROC2_GRC_DEBUG: c_uint = 0x00002844;
pub const BNX2_RV2P_GRC_PROC_DEBUG: c_uint = 0x00002848;
pub const BNX2_RV2P_DEBUG_VECT_PEEK: c_uint = 0x0000284c;

pub const BNX2_RV2P_MPFE_PFE_CTL: c_uint = 0x00002afc;

pub const BNX2_RV2P_RV2PPQ: c_uint = 0x00002b40;
pub const BNX2_RV2P_PFTQ_CMD: c_uint = 0x00002b78;

pub const BNX2_RV2P_PFTQ_CTL: c_uint = 0x00002b7c;

pub const BNX2_RV2P_RV2PTQ: c_uint = 0x00002b80;
pub const BNX2_RV2P_TFTQ_CMD: c_uint = 0x00002bb8;

pub const BNX2_RV2P_TFTQ_CTL: c_uint = 0x00002bbc;

pub const BNX2_RV2P_RV2PMQ: c_uint = 0x00002bc0;
pub const BNX2_RV2P_MFTQ_CMD: c_uint = 0x00002bf8;

pub const BNX2_RV2P_MFTQ_CTL: c_uint = 0x00002bfc;

//
// mq_reg definition
// offset: 0x3c00
//
pub const BNX2_MQ_COMMAND: c_uint = 0x00003c00;

pub const BNX2_MQ_STATUS: c_uint = 0x00003c04;

pub const BNX2_MQ_CONFIG: c_uint = 0x00003c08;

pub const BNX2_MQ_ENQUEUE1: c_uint = 0x00003c0c;

pub const BNX2_MQ_ENQUEUE2: c_uint = 0x00003c10;
pub const BNX2_MQ_BAD_WR_ADDR: c_uint = 0x00003c14;
pub const BNX2_MQ_BAD_RD_ADDR: c_uint = 0x00003c18;
pub const BNX2_MQ_KNL_BYP_WIND_START: c_uint = 0x00003c1c;

pub const BNX2_MQ_KNL_WIND_END: c_uint = 0x00003c20;

pub const BNX2_MQ_KNL_WRITE_MASK1: c_uint = 0x00003c24;
pub const BNX2_MQ_KNL_TX_MASK1: c_uint = 0x00003c28;
pub const BNX2_MQ_KNL_CMD_MASK1: c_uint = 0x00003c2c;
pub const BNX2_MQ_KNL_COND_ENQUEUE_MASK1: c_uint = 0x00003c30;
pub const BNX2_MQ_KNL_RX_V2P_MASK1: c_uint = 0x00003c34;
pub const BNX2_MQ_KNL_WRITE_MASK2: c_uint = 0x00003c38;
pub const BNX2_MQ_KNL_TX_MASK2: c_uint = 0x00003c3c;
pub const BNX2_MQ_KNL_CMD_MASK2: c_uint = 0x00003c40;
pub const BNX2_MQ_KNL_COND_ENQUEUE_MASK2: c_uint = 0x00003c44;
pub const BNX2_MQ_KNL_RX_V2P_MASK2: c_uint = 0x00003c48;
pub const BNX2_MQ_KNL_BYP_WRITE_MASK1: c_uint = 0x00003c4c;
pub const BNX2_MQ_KNL_BYP_TX_MASK1: c_uint = 0x00003c50;
pub const BNX2_MQ_KNL_BYP_CMD_MASK1: c_uint = 0x00003c54;
pub const BNX2_MQ_KNL_BYP_COND_ENQUEUE_MASK1: c_uint = 0x00003c58;
pub const BNX2_MQ_KNL_BYP_RX_V2P_MASK1: c_uint = 0x00003c5c;
pub const BNX2_MQ_KNL_BYP_WRITE_MASK2: c_uint = 0x00003c60;
pub const BNX2_MQ_KNL_BYP_TX_MASK2: c_uint = 0x00003c64;
pub const BNX2_MQ_KNL_BYP_CMD_MASK2: c_uint = 0x00003c68;
pub const BNX2_MQ_KNL_BYP_COND_ENQUEUE_MASK2: c_uint = 0x00003c6c;
pub const BNX2_MQ_KNL_BYP_RX_V2P_MASK2: c_uint = 0x00003c70;
pub const BNX2_MQ_MEM_WR_ADDR: c_uint = 0x00003c74;

pub const BNX2_MQ_MEM_WR_DATA0: c_uint = 0x00003c78;

pub const BNX2_MQ_MEM_WR_DATA1: c_uint = 0x00003c7c;

pub const BNX2_MQ_MEM_WR_DATA2: c_uint = 0x00003c80;

pub const BNX2_MQ_MEM_RD_ADDR: c_uint = 0x00003c84;

pub const BNX2_MQ_MEM_RD_DATA0: c_uint = 0x00003c88;

pub const BNX2_MQ_MEM_RD_DATA1: c_uint = 0x00003c8c;

pub const BNX2_MQ_MEM_RD_DATA2: c_uint = 0x00003c90;

pub const BNX2_MQ_MAP_L2_3: c_uint = 0x00003d2c;

pub const BNX2_MQ_MAP_L2_3_DEFAULT: c_uint = 0x82004646;
pub const BNX2_MQ_MAP_L2_5: c_uint = 0x00003d34;

//
// tsch_reg definition
// offset: 0x4c00
//
pub const BNX2_TSCH_TSS_CFG: c_uint = 0x00004c1c;

//
// tbdr_reg definition
// offset: 0x5000
//
pub const BNX2_TBDR_COMMAND: c_uint = 0x00005000;

pub const BNX2_TBDR_STATUS: c_uint = 0x00005004;

pub const BNX2_TBDR_CONFIG: c_uint = 0x00005008;

pub const BNX2_TBDR_DEBUG_VECT_PEEK: c_uint = 0x0000500c;

pub const BNX2_TBDR_CKSUM_ERROR_STATUS: c_uint = 0x00005010;

pub const BNX2_TBDR_TBDRQ: c_uint = 0x000053c0;
pub const BNX2_TBDR_FTQ_CMD: c_uint = 0x000053f8;

pub const BNX2_TBDR_FTQ_CTL: c_uint = 0x000053fc;

//
// tbdc definition
// offset: 0x5400
//
pub const BNX2_TBDC_COMMAND: c_uint = 0x5400;

pub const BNX2_TBDC_STATUS: c_uint = 0x5404;

pub const BNX2_TBDC_BD_ADDR: c_uint = 0x5424;
pub const BNX2_TBDC_BIDX: c_uint = 0x542c;

pub const BNX2_TBDC_CID: c_uint = 0x5430;
pub const BNX2_TBDC_CAM_OPCODE: c_uint = 0x5434;

//
// tdma_reg definition
// offset: 0x5c00
//
pub const BNX2_TDMA_COMMAND: c_uint = 0x00005c00;

pub const BNX2_TDMA_STATUS: c_uint = 0x00005c04;

pub const BNX2_TDMA_CONFIG: c_uint = 0x00005c08;

pub const BNX2_TDMA_PAYLOAD_PROD: c_uint = 0x00005c0c;

pub const BNX2_TDMA_DBG_WATCHDOG: c_uint = 0x00005c10;
pub const BNX2_TDMA_DBG_TRIGGER: c_uint = 0x00005c14;
pub const BNX2_TDMA_DMAD_FSM: c_uint = 0x00005c80;

pub const BNX2_TDMA_DMAD_STATUS: c_uint = 0x00005c84;

pub const BNX2_TDMA_DR_INTF_FSM: c_uint = 0x00005c88;

pub const BNX2_TDMA_DR_INTF_STATUS: c_uint = 0x00005c8c;

pub const BNX2_TDMA_PUSH_FSM: c_uint = 0x00005c90;
pub const BNX2_TDMA_BD_IF_DEBUG: c_uint = 0x00005c94;
pub const BNX2_TDMA_DMAD_IF_DEBUG: c_uint = 0x00005c98;
pub const BNX2_TDMA_CTX_IF_DEBUG: c_uint = 0x00005c9c;
pub const BNX2_TDMA_TPBUF_IF_DEBUG: c_uint = 0x00005ca0;
pub const BNX2_TDMA_DR_IF_DEBUG: c_uint = 0x00005ca4;
pub const BNX2_TDMA_TPATQ_IF_DEBUG: c_uint = 0x00005ca8;
pub const BNX2_TDMA_TDMA_ILOCK_CKSUM: c_uint = 0x00005cac;

pub const BNX2_TDMA_TDMA_PCIE_CKSUM: c_uint = 0x00005cb0;

pub const BNX2_TDMA_TDMAQ: c_uint = 0x00005fc0;
pub const BNX2_TDMA_FTQ_CMD: c_uint = 0x00005ff8;

pub const BNX2_TDMA_FTQ_CTL: c_uint = 0x00005ffc;

//
// hc_reg definition
// offset: 0x6800
//
pub const BNX2_HC_COMMAND: c_uint = 0x00006800;

pub const BNX2_HC_STATUS: c_uint = 0x00006804;

pub const BNX2_HC_CONFIG: c_uint = 0x00006808;

pub const BNX2_HC_ATTN_BITS_ENABLE: c_uint = 0x0000680c;
pub const BNX2_HC_STATUS_ADDR_L: c_uint = 0x00006810;
pub const BNX2_HC_STATUS_ADDR_H: c_uint = 0x00006814;
pub const BNX2_HC_STATISTICS_ADDR_L: c_uint = 0x00006818;
pub const BNX2_HC_STATISTICS_ADDR_H: c_uint = 0x0000681c;
pub const BNX2_HC_TX_QUICK_CONS_TRIP: c_uint = 0x00006820;

pub const BNX2_HC_COMP_PROD_TRIP: c_uint = 0x00006824;

pub const BNX2_HC_RX_QUICK_CONS_TRIP: c_uint = 0x00006828;

pub const BNX2_HC_RX_TICKS: c_uint = 0x0000682c;

pub const BNX2_HC_TX_TICKS: c_uint = 0x00006830;

pub const BNX2_HC_COM_TICKS: c_uint = 0x00006834;

pub const BNX2_HC_CMD_TICKS: c_uint = 0x00006838;

pub const BNX2_HC_PERIODIC_TICKS: c_uint = 0x0000683c;

pub const BNX2_HC_STAT_COLLECT_TICKS: c_uint = 0x00006840;

pub const BNX2_HC_STATS_TICKS: c_uint = 0x00006844;

pub const BNX2_HC_STATS_INTERRUPT_STATUS: c_uint = 0x00006848;

pub const BNX2_HC_STAT_MEM_DATA: c_uint = 0x0000684c;
pub const BNX2_HC_STAT_GEN_SEL_0: c_uint = 0x00006850;

pub const BNX2_HC_STAT_GEN_SEL_1: c_uint = 0x00006854;

pub const BNX2_HC_STAT_GEN_SEL_2: c_uint = 0x00006858;

pub const BNX2_HC_STAT_GEN_SEL_3: c_uint = 0x0000685c;

pub const BNX2_HC_STAT_GEN_STAT0: c_uint = 0x00006888;
pub const BNX2_HC_STAT_GEN_STAT1: c_uint = 0x0000688c;
pub const BNX2_HC_STAT_GEN_STAT2: c_uint = 0x00006890;
pub const BNX2_HC_STAT_GEN_STAT3: c_uint = 0x00006894;
pub const BNX2_HC_STAT_GEN_STAT4: c_uint = 0x00006898;
pub const BNX2_HC_STAT_GEN_STAT5: c_uint = 0x0000689c;
pub const BNX2_HC_STAT_GEN_STAT6: c_uint = 0x000068a0;
pub const BNX2_HC_STAT_GEN_STAT7: c_uint = 0x000068a4;
pub const BNX2_HC_STAT_GEN_STAT8: c_uint = 0x000068a8;
pub const BNX2_HC_STAT_GEN_STAT9: c_uint = 0x000068ac;
pub const BNX2_HC_STAT_GEN_STAT10: c_uint = 0x000068b0;
pub const BNX2_HC_STAT_GEN_STAT11: c_uint = 0x000068b4;
pub const BNX2_HC_STAT_GEN_STAT12: c_uint = 0x000068b8;
pub const BNX2_HC_STAT_GEN_STAT13: c_uint = 0x000068bc;
pub const BNX2_HC_STAT_GEN_STAT14: c_uint = 0x000068c0;
pub const BNX2_HC_STAT_GEN_STAT15: c_uint = 0x000068c4;
pub const BNX2_HC_STAT_GEN_STAT_AC0: c_uint = 0x000068c8;
pub const BNX2_HC_STAT_GEN_STAT_AC1: c_uint = 0x000068cc;
pub const BNX2_HC_STAT_GEN_STAT_AC2: c_uint = 0x000068d0;
pub const BNX2_HC_STAT_GEN_STAT_AC3: c_uint = 0x000068d4;
pub const BNX2_HC_STAT_GEN_STAT_AC4: c_uint = 0x000068d8;
pub const BNX2_HC_STAT_GEN_STAT_AC5: c_uint = 0x000068dc;
pub const BNX2_HC_STAT_GEN_STAT_AC6: c_uint = 0x000068e0;
pub const BNX2_HC_STAT_GEN_STAT_AC7: c_uint = 0x000068e4;
pub const BNX2_HC_STAT_GEN_STAT_AC8: c_uint = 0x000068e8;
pub const BNX2_HC_STAT_GEN_STAT_AC9: c_uint = 0x000068ec;
pub const BNX2_HC_STAT_GEN_STAT_AC10: c_uint = 0x000068f0;
pub const BNX2_HC_STAT_GEN_STAT_AC11: c_uint = 0x000068f4;
pub const BNX2_HC_STAT_GEN_STAT_AC12: c_uint = 0x000068f8;
pub const BNX2_HC_STAT_GEN_STAT_AC13: c_uint = 0x000068fc;
pub const BNX2_HC_STAT_GEN_STAT_AC14: c_uint = 0x00006900;
pub const BNX2_HC_STAT_GEN_STAT_AC15: c_uint = 0x00006904;
pub const BNX2_HC_STAT_GEN_STAT_AC: c_uint = 0x000068c8;
pub const BNX2_HC_VIS: c_uint = 0x00006908;

pub const BNX2_HC_VIS_1: c_uint = 0x0000690c;

pub const BNX2_HC_DEBUG_VECT_PEEK: c_uint = 0x00006910;

pub const BNX2_HC_COALESCE_NOW: c_uint = 0x00006914;

pub const BNX2_HC_MSIX_BIT_VECTOR: c_uint = 0x00006918;

pub const BNX2_HC_SB_CONFIG_1: c_uint = 0x00006a00;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_1: c_uint = 0x00006a04;

pub const BNX2_HC_COMP_PROD_TRIP_1: c_uint = 0x00006a08;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_1: c_uint = 0x00006a0c;

pub const BNX2_HC_RX_TICKS_1: c_uint = 0x00006a10;

pub const BNX2_HC_TX_TICKS_1: c_uint = 0x00006a14;

pub const BNX2_HC_COM_TICKS_1: c_uint = 0x00006a18;

pub const BNX2_HC_CMD_TICKS_1: c_uint = 0x00006a1c;

pub const BNX2_HC_PERIODIC_TICKS_1: c_uint = 0x00006a20;

pub const BNX2_HC_SB_CONFIG_2: c_uint = 0x00006a24;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_2: c_uint = 0x00006a28;

pub const BNX2_HC_COMP_PROD_TRIP_2: c_uint = 0x00006a2c;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_2: c_uint = 0x00006a30;

pub const BNX2_HC_RX_TICKS_2: c_uint = 0x00006a34;

pub const BNX2_HC_TX_TICKS_2: c_uint = 0x00006a38;

pub const BNX2_HC_COM_TICKS_2: c_uint = 0x00006a3c;

pub const BNX2_HC_CMD_TICKS_2: c_uint = 0x00006a40;

pub const BNX2_HC_PERIODIC_TICKS_2: c_uint = 0x00006a44;

pub const BNX2_HC_SB_CONFIG_3: c_uint = 0x00006a48;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_3: c_uint = 0x00006a4c;

pub const BNX2_HC_COMP_PROD_TRIP_3: c_uint = 0x00006a50;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_3: c_uint = 0x00006a54;

pub const BNX2_HC_RX_TICKS_3: c_uint = 0x00006a58;

pub const BNX2_HC_TX_TICKS_3: c_uint = 0x00006a5c;

pub const BNX2_HC_COM_TICKS_3: c_uint = 0x00006a60;

pub const BNX2_HC_CMD_TICKS_3: c_uint = 0x00006a64;

pub const BNX2_HC_PERIODIC_TICKS_3: c_uint = 0x00006a68;

pub const BNX2_HC_SB_CONFIG_4: c_uint = 0x00006a6c;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_4: c_uint = 0x00006a70;

pub const BNX2_HC_COMP_PROD_TRIP_4: c_uint = 0x00006a74;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_4: c_uint = 0x00006a78;

pub const BNX2_HC_RX_TICKS_4: c_uint = 0x00006a7c;

pub const BNX2_HC_TX_TICKS_4: c_uint = 0x00006a80;

pub const BNX2_HC_COM_TICKS_4: c_uint = 0x00006a84;

pub const BNX2_HC_CMD_TICKS_4: c_uint = 0x00006a88;

pub const BNX2_HC_PERIODIC_TICKS_4: c_uint = 0x00006a8c;

pub const BNX2_HC_SB_CONFIG_5: c_uint = 0x00006a90;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_5: c_uint = 0x00006a94;

pub const BNX2_HC_COMP_PROD_TRIP_5: c_uint = 0x00006a98;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_5: c_uint = 0x00006a9c;

pub const BNX2_HC_RX_TICKS_5: c_uint = 0x00006aa0;

pub const BNX2_HC_TX_TICKS_5: c_uint = 0x00006aa4;

pub const BNX2_HC_COM_TICKS_5: c_uint = 0x00006aa8;

pub const BNX2_HC_CMD_TICKS_5: c_uint = 0x00006aac;

pub const BNX2_HC_PERIODIC_TICKS_5: c_uint = 0x00006ab0;

pub const BNX2_HC_SB_CONFIG_6: c_uint = 0x00006ab4;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_6: c_uint = 0x00006ab8;

pub const BNX2_HC_COMP_PROD_TRIP_6: c_uint = 0x00006abc;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_6: c_uint = 0x00006ac0;

pub const BNX2_HC_RX_TICKS_6: c_uint = 0x00006ac4;

pub const BNX2_HC_TX_TICKS_6: c_uint = 0x00006ac8;

pub const BNX2_HC_COM_TICKS_6: c_uint = 0x00006acc;

pub const BNX2_HC_CMD_TICKS_6: c_uint = 0x00006ad0;

pub const BNX2_HC_PERIODIC_TICKS_6: c_uint = 0x00006ad4;

pub const BNX2_HC_SB_CONFIG_7: c_uint = 0x00006ad8;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_7: c_uint = 0x00006adc;

pub const BNX2_HC_COMP_PROD_TRIP_7: c_uint = 0x00006ae0;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_7: c_uint = 0x00006ae4;

pub const BNX2_HC_RX_TICKS_7: c_uint = 0x00006ae8;

pub const BNX2_HC_TX_TICKS_7: c_uint = 0x00006aec;

pub const BNX2_HC_COM_TICKS_7: c_uint = 0x00006af0;

pub const BNX2_HC_CMD_TICKS_7: c_uint = 0x00006af4;

pub const BNX2_HC_PERIODIC_TICKS_7: c_uint = 0x00006af8;

pub const BNX2_HC_SB_CONFIG_8: c_uint = 0x00006afc;

pub const BNX2_HC_TX_QUICK_CONS_TRIP_8: c_uint = 0x00006b00;

pub const BNX2_HC_COMP_PROD_TRIP_8: c_uint = 0x00006b04;

pub const BNX2_HC_RX_QUICK_CONS_TRIP_8: c_uint = 0x00006b08;

pub const BNX2_HC_RX_TICKS_8: c_uint = 0x00006b0c;

pub const BNX2_HC_TX_TICKS_8: c_uint = 0x00006b10;

pub const BNX2_HC_COM_TICKS_8: c_uint = 0x00006b14;

pub const BNX2_HC_CMD_TICKS_8: c_uint = 0x00006b18;

pub const BNX2_HC_PERIODIC_TICKS_8: c_uint = 0x00006b1c;

//
// txp_reg definition
// offset: 0x40000
//
pub const BNX2_TXP_CPU_MODE: c_uint = 0x00045000;

pub const BNX2_TXP_CPU_STATE: c_uint = 0x00045004;

pub const BNX2_TXP_CPU_EVENT_MASK: c_uint = 0x00045008;

pub const BNX2_TXP_CPU_PROGRAM_COUNTER: c_uint = 0x0004501c;
pub const BNX2_TXP_CPU_INSTRUCTION: c_uint = 0x00045020;
pub const BNX2_TXP_CPU_DATA_ACCESS: c_uint = 0x00045024;
pub const BNX2_TXP_CPU_INTERRUPT_ENABLE: c_uint = 0x00045028;
pub const BNX2_TXP_CPU_INTERRUPT_VECTOR: c_uint = 0x0004502c;
pub const BNX2_TXP_CPU_INTERRUPT_SAVED_PC: c_uint = 0x00045030;
pub const BNX2_TXP_CPU_HW_BREAKPOINT: c_uint = 0x00045034;

pub const BNX2_TXP_CPU_DEBUG_VECT_PEEK: c_uint = 0x00045038;

pub const BNX2_TXP_CPU_LAST_BRANCH_ADDR: c_uint = 0x00045048;

pub const BNX2_TXP_CPU_REG_FILE: c_uint = 0x00045200;
pub const BNX2_TXP_TXPQ: c_uint = 0x000453c0;
pub const BNX2_TXP_FTQ_CMD: c_uint = 0x000453f8;

pub const BNX2_TXP_FTQ_CTL: c_uint = 0x000453fc;

pub const BNX2_TXP_SCRATCH: c_uint = 0x00060000;
//
// tpat_reg definition
// offset: 0x80000
//
pub const BNX2_TPAT_CPU_MODE: c_uint = 0x00085000;

pub const BNX2_TPAT_CPU_STATE: c_uint = 0x00085004;

pub const BNX2_TPAT_CPU_EVENT_MASK: c_uint = 0x00085008;

pub const BNX2_TPAT_CPU_PROGRAM_COUNTER: c_uint = 0x0008501c;
pub const BNX2_TPAT_CPU_INSTRUCTION: c_uint = 0x00085020;
pub const BNX2_TPAT_CPU_DATA_ACCESS: c_uint = 0x00085024;
pub const BNX2_TPAT_CPU_INTERRUPT_ENABLE: c_uint = 0x00085028;
pub const BNX2_TPAT_CPU_INTERRUPT_VECTOR: c_uint = 0x0008502c;
pub const BNX2_TPAT_CPU_INTERRUPT_SAVED_PC: c_uint = 0x00085030;
pub const BNX2_TPAT_CPU_HW_BREAKPOINT: c_uint = 0x00085034;

pub const BNX2_TPAT_CPU_DEBUG_VECT_PEEK: c_uint = 0x00085038;

pub const BNX2_TPAT_CPU_LAST_BRANCH_ADDR: c_uint = 0x00085048;

pub const BNX2_TPAT_CPU_REG_FILE: c_uint = 0x00085200;
pub const BNX2_TPAT_TPATQ: c_uint = 0x000853c0;
pub const BNX2_TPAT_FTQ_CMD: c_uint = 0x000853f8;

pub const BNX2_TPAT_FTQ_CTL: c_uint = 0x000853fc;

pub const BNX2_TPAT_SCRATCH: c_uint = 0x000a0000;
//
// rxp_reg definition
// offset: 0xc0000
//
pub const BNX2_RXP_CPU_MODE: c_uint = 0x000c5000;

pub const BNX2_RXP_CPU_STATE: c_uint = 0x000c5004;

pub const BNX2_RXP_CPU_EVENT_MASK: c_uint = 0x000c5008;

pub const BNX2_RXP_CPU_PROGRAM_COUNTER: c_uint = 0x000c501c;
pub const BNX2_RXP_CPU_INSTRUCTION: c_uint = 0x000c5020;
pub const BNX2_RXP_CPU_DATA_ACCESS: c_uint = 0x000c5024;
pub const BNX2_RXP_CPU_INTERRUPT_ENABLE: c_uint = 0x000c5028;
pub const BNX2_RXP_CPU_INTERRUPT_VECTOR: c_uint = 0x000c502c;
pub const BNX2_RXP_CPU_INTERRUPT_SAVED_PC: c_uint = 0x000c5030;
pub const BNX2_RXP_CPU_HW_BREAKPOINT: c_uint = 0x000c5034;

pub const BNX2_RXP_CPU_DEBUG_VECT_PEEK: c_uint = 0x000c5038;

pub const BNX2_RXP_CPU_LAST_BRANCH_ADDR: c_uint = 0x000c5048;

pub const BNX2_RXP_CPU_REG_FILE: c_uint = 0x000c5200;
pub const BNX2_RXP_PFE_PFE_CTL: c_uint = 0x000c537c;

pub const BNX2_RXP_RXPCQ: c_uint = 0x000c5380;
pub const BNX2_RXP_CFTQ_CMD: c_uint = 0x000c53b8;

pub const BNX2_RXP_CFTQ_CTL: c_uint = 0x000c53bc;

pub const BNX2_RXP_RXPQ: c_uint = 0x000c53c0;
pub const BNX2_RXP_FTQ_CMD: c_uint = 0x000c53f8;

pub const BNX2_RXP_FTQ_CTL: c_uint = 0x000c53fc;

pub const BNX2_RXP_SCRATCH: c_uint = 0x000e0000;
pub const BNX2_RXP_SCRATCH_RXP_FLOOD: c_uint = 0x000e0024;
pub const BNX2_RXP_SCRATCH_RSS_TBL_SZ: c_uint = 0x000e0038;
pub const BNX2_RXP_SCRATCH_RSS_TBL: c_uint = 0x000e003c;
pub const BNX2_RXP_SCRATCH_RSS_TBL_MAX_ENTRIES: c_int = 128;
//
// com_reg definition
// offset: 0x100000
//
pub const BNX2_COM_CKSUM_ERROR_STATUS: c_uint = 0x00100000;

pub const BNX2_COM_CPU_MODE: c_uint = 0x00105000;

pub const BNX2_COM_CPU_STATE: c_uint = 0x00105004;

pub const BNX2_COM_CPU_EVENT_MASK: c_uint = 0x00105008;

pub const BNX2_COM_CPU_PROGRAM_COUNTER: c_uint = 0x0010501c;
pub const BNX2_COM_CPU_INSTRUCTION: c_uint = 0x00105020;
pub const BNX2_COM_CPU_DATA_ACCESS: c_uint = 0x00105024;
pub const BNX2_COM_CPU_INTERRUPT_ENABLE: c_uint = 0x00105028;
pub const BNX2_COM_CPU_INTERRUPT_VECTOR: c_uint = 0x0010502c;
pub const BNX2_COM_CPU_INTERRUPT_SAVED_PC: c_uint = 0x00105030;
pub const BNX2_COM_CPU_HW_BREAKPOINT: c_uint = 0x00105034;

pub const BNX2_COM_CPU_DEBUG_VECT_PEEK: c_uint = 0x00105038;

pub const BNX2_COM_CPU_LAST_BRANCH_ADDR: c_uint = 0x00105048;

pub const BNX2_COM_CPU_REG_FILE: c_uint = 0x00105200;
pub const BNX2_COM_COMTQ_PFE_PFE_CTL: c_uint = 0x001052bc;

pub const BNX2_COM_COMXQ: c_uint = 0x00105340;
pub const BNX2_COM_COMXQ_FTQ_CMD: c_uint = 0x00105378;

pub const BNX2_COM_COMXQ_FTQ_CTL: c_uint = 0x0010537c;

pub const BNX2_COM_COMTQ: c_uint = 0x00105380;
pub const BNX2_COM_COMTQ_FTQ_CMD: c_uint = 0x001053b8;

pub const BNX2_COM_COMTQ_FTQ_CTL: c_uint = 0x001053bc;

pub const BNX2_COM_COMQ: c_uint = 0x001053c0;
pub const BNX2_COM_COMQ_FTQ_CMD: c_uint = 0x001053f8;

pub const BNX2_COM_COMQ_FTQ_CTL: c_uint = 0x001053fc;

pub const BNX2_COM_SCRATCH: c_uint = 0x00120000;
pub const BNX2_FW_RX_LOW_LATENCY: c_uint = 0x00120058;
pub const BNX2_FW_RX_DROP_COUNT: c_uint = 0x00120084;
//
// cp_reg definition
// offset: 0x180000
//
pub const BNX2_CP_CKSUM_ERROR_STATUS: c_uint = 0x00180000;

pub const BNX2_CP_CPU_MODE: c_uint = 0x00185000;

pub const BNX2_CP_CPU_STATE: c_uint = 0x00185004;

pub const BNX2_CP_CPU_EVENT_MASK: c_uint = 0x00185008;

pub const BNX2_CP_CPU_PROGRAM_COUNTER: c_uint = 0x0018501c;
pub const BNX2_CP_CPU_INSTRUCTION: c_uint = 0x00185020;
pub const BNX2_CP_CPU_DATA_ACCESS: c_uint = 0x00185024;
pub const BNX2_CP_CPU_INTERRUPT_ENABLE: c_uint = 0x00185028;
pub const BNX2_CP_CPU_INTERRUPT_VECTOR: c_uint = 0x0018502c;
pub const BNX2_CP_CPU_INTERRUPT_SAVED_PC: c_uint = 0x00185030;
pub const BNX2_CP_CPU_HW_BREAKPOINT: c_uint = 0x00185034;

pub const BNX2_CP_CPU_DEBUG_VECT_PEEK: c_uint = 0x00185038;

pub const BNX2_CP_CPU_LAST_BRANCH_ADDR: c_uint = 0x00185048;

pub const BNX2_CP_CPU_REG_FILE: c_uint = 0x00185200;
pub const BNX2_CP_CPQ_PFE_PFE_CTL: c_uint = 0x001853bc;

pub const BNX2_CP_CPQ: c_uint = 0x001853c0;
pub const BNX2_CP_CPQ_FTQ_CMD: c_uint = 0x001853f8;

pub const BNX2_CP_CPQ_FTQ_CTL: c_uint = 0x001853fc;

pub const BNX2_CP_SCRATCH: c_uint = 0x001a0000;
pub const BNX2_FW_MAX_ISCSI_CONN: c_uint = 0x001a0080;
//
// mcp_reg definition
// offset: 0x140000
//
pub const BNX2_MCP_MCP_CONTROL: c_uint = 0x00140080;

pub const BNX2_MCP_MCP_ATTENTION_STATUS: c_uint = 0x00140084;

pub const BNX2_MCP_MCP_HEARTBEAT_CONTROL: c_uint = 0x00140088;

pub const BNX2_MCP_MCP_HEARTBEAT_STATUS: c_uint = 0x0014008c;

pub const BNX2_MCP_MCP_HEARTBEAT: c_uint = 0x00140090;

pub const BNX2_MCP_WATCHDOG_RESET: c_uint = 0x00140094;

pub const BNX2_MCP_WATCHDOG_CONTROL: c_uint = 0x00140098;

pub const BNX2_MCP_ACCESS_LOCK: c_uint = 0x0014009c;

pub const BNX2_MCP_TOE_ID: c_uint = 0x001400a0;

pub const BNX2_MCP_MAILBOX_CFG: c_uint = 0x001400a4;

pub const BNX2_MCP_MAILBOX_CFG_OTHER_FUNC: c_uint = 0x001400a8;

pub const BNX2_MCP_MCP_DOORBELL: c_uint = 0x001400ac;

pub const BNX2_MCP_DRIVER_DOORBELL: c_uint = 0x001400b0;

pub const BNX2_MCP_DRIVER_DOORBELL_OTHER_FUNC: c_uint = 0x001400b4;

pub const BNX2_MCP_CPU_MODE: c_uint = 0x00145000;

pub const BNX2_MCP_CPU_STATE: c_uint = 0x00145004;

pub const BNX2_MCP_CPU_EVENT_MASK: c_uint = 0x00145008;

pub const BNX2_MCP_CPU_PROGRAM_COUNTER: c_uint = 0x0014501c;
pub const BNX2_MCP_CPU_INSTRUCTION: c_uint = 0x00145020;
pub const BNX2_MCP_CPU_DATA_ACCESS: c_uint = 0x00145024;
pub const BNX2_MCP_CPU_INTERRUPT_ENABLE: c_uint = 0x00145028;
pub const BNX2_MCP_CPU_INTERRUPT_VECTOR: c_uint = 0x0014502c;
pub const BNX2_MCP_CPU_INTERRUPT_SAVED_PC: c_uint = 0x00145030;
pub const BNX2_MCP_CPU_HW_BREAKPOINT: c_uint = 0x00145034;

pub const BNX2_MCP_CPU_DEBUG_VECT_PEEK: c_uint = 0x00145038;

pub const BNX2_MCP_CPU_LAST_BRANCH_ADDR: c_uint = 0x00145048;

pub const BNX2_MCP_CPU_REG_FILE: c_uint = 0x00145200;
pub const BNX2_MCP_MCPQ: c_uint = 0x001453c0;
pub const BNX2_MCP_MCPQ_FTQ_CMD: c_uint = 0x001453f8;

pub const BNX2_MCP_MCPQ_FTQ_CTL: c_uint = 0x001453fc;

pub const BNX2_MCP_ROM: c_uint = 0x00150000;
pub const BNX2_MCP_SCRATCH: c_uint = 0x00160000;
pub const BNX2_MCP_STATE_P1: c_uint = 0x0016f9c8;
pub const BNX2_MCP_STATE_P0: c_uint = 0x0016fdc8;
pub const BNX2_MCP_STATE_P1_5708: c_uint = 0x001699c8;
pub const BNX2_MCP_STATE_P0_5708: c_uint = 0x00169dc8;

pub const BNX2_SHM_HDR_SIGNATURE_SIG_MASK: c_uint = 0xffff0000;
pub const BNX2_SHM_HDR_SIGNATURE_SIG: c_uint = 0x53530000;
pub const BNX2_SHM_HDR_SIGNATURE_VER_MASK: c_uint = 0x000000ff;
pub const BNX2_SHM_HDR_SIGNATURE_VER_ONE: c_uint = 0x00000001;

pub const NUM_MC_HASH_REGISTERS: c_int = 8;
// PHY_ID1: bits 31-16; PHY_ID2: bits 15-0.
pub const PHY_BCM5706_PHY_ID: c_uint = 0x00206160;

// 5708 Serdes PHY registers
pub const BCM5708S_BMCR_FORCE_2500: c_uint = 0x20;
pub const BCM5708S_UP1: c_uint = 0xb;
pub const BCM5708S_UP1_2G5: c_uint = 0x1;
pub const BCM5708S_BLK_ADDR: c_uint = 0x1f;
pub const BCM5708S_BLK_ADDR_DIG: c_uint = 0x0000;
pub const BCM5708S_BLK_ADDR_DIG3: c_uint = 0x0002;
pub const BCM5708S_BLK_ADDR_TX_MISC: c_uint = 0x0005;
// Digital Block
pub const BCM5708S_1000X_CTL1: c_uint = 0x10;
pub const BCM5708S_1000X_CTL1_FIBER_MODE: c_uint = 0x0001;
pub const BCM5708S_1000X_CTL1_AUTODET_EN: c_uint = 0x0010;
pub const BCM5708S_1000X_CTL2: c_uint = 0x11;
pub const BCM5708S_1000X_CTL2_PLLEL_DET_EN: c_uint = 0x0001;
pub const BCM5708S_1000X_STAT1: c_uint = 0x14;
pub const BCM5708S_1000X_STAT1_SGMII: c_uint = 0x0001;
pub const BCM5708S_1000X_STAT1_LINK: c_uint = 0x0002;
pub const BCM5708S_1000X_STAT1_FD: c_uint = 0x0004;
pub const BCM5708S_1000X_STAT1_SPEED_MASK: c_uint = 0x0018;
pub const BCM5708S_1000X_STAT1_SPEED_10: c_uint = 0x0000;
pub const BCM5708S_1000X_STAT1_SPEED_100: c_uint = 0x0008;
pub const BCM5708S_1000X_STAT1_SPEED_1G: c_uint = 0x0010;
pub const BCM5708S_1000X_STAT1_SPEED_2G5: c_uint = 0x0018;
pub const BCM5708S_1000X_STAT1_TX_PAUSE: c_uint = 0x0020;
pub const BCM5708S_1000X_STAT1_RX_PAUSE: c_uint = 0x0040;
// Digital3 Block
pub const BCM5708S_DIG_3_0: c_uint = 0x10;
pub const BCM5708S_DIG_3_0_USE_IEEE: c_uint = 0x0001;
// Tx/Misc Block
pub const BCM5708S_TX_ACTL1: c_uint = 0x15;
pub const BCM5708S_TX_ACTL1_DRIVER_VCM: c_uint = 0x30;
pub const BCM5708S_TX_ACTL3: c_uint = 0x17;
pub const MII_BNX2_EXT_STATUS: c_uint = 0x11;

pub const MII_BNX2_AUX_CTL: c_uint = 0x18;
pub const AUX_CTL_MISC_CTL: c_uint = 0x7007;

pub const MII_BNX2_DSP_RW_PORT: c_uint = 0x15;
pub const MII_BNX2_DSP_ADDRESS: c_uint = 0x17;
pub const MII_BNX2_DSP_EXPAND_REG: c_uint = 0x0f00;

pub const MII_EXPAND_REG1_RUDI_C: c_uint = 0x20;

pub const MII_BNX2_MISC_SHADOW: c_uint = 0x1c;
pub const MISC_SHDW_AN_DBG: c_uint = 0x6800;
pub const MISC_SHDW_AN_DBG_NOSYNC: c_uint = 0x0002;
pub const MISC_SHDW_AN_DBG_RUDI_INVALID: c_uint = 0x0100;
pub const MISC_SHDW_MODE_CTL: c_uint = 0x7c00;
pub const MISC_SHDW_MODE_CTL_SIG_DET: c_uint = 0x0010;
pub const MII_BNX2_BLK_ADDR: c_uint = 0x1f;
pub const MII_BNX2_BLK_ADDR_IEEE0: c_uint = 0x0000;
pub const MII_BNX2_BLK_ADDR_GP_STATUS: c_uint = 0x8120;
pub const MII_BNX2_GP_TOP_AN_STATUS1: c_uint = 0x1b;
pub const MII_BNX2_GP_TOP_AN_SPEED_MSK: c_uint = 0x3f00;
pub const MII_BNX2_GP_TOP_AN_SPEED_10: c_uint = 0x0000;
pub const MII_BNX2_GP_TOP_AN_SPEED_100: c_uint = 0x0100;
pub const MII_BNX2_GP_TOP_AN_SPEED_1G: c_uint = 0x0200;
pub const MII_BNX2_GP_TOP_AN_SPEED_2_5G: c_uint = 0x0300;
pub const MII_BNX2_GP_TOP_AN_SPEED_1GKV: c_uint = 0x0d00;
pub const MII_BNX2_GP_TOP_AN_FD: c_uint = 0x8;
pub const MII_BNX2_BLK_ADDR_SERDES_DIG: c_uint = 0x8300;
pub const MII_BNX2_SERDES_DIG_1000XCTL1: c_uint = 0x10;
pub const MII_BNX2_SD_1000XCTL1_FIBER: c_uint = 0x01;
pub const MII_BNX2_SD_1000XCTL1_AUTODET: c_uint = 0x10;
pub const MII_BNX2_SERDES_DIG_MISC1: c_uint = 0x18;
pub const MII_BNX2_SD_MISC1_FORCE_MSK: c_uint = 0xf;
pub const MII_BNX2_SD_MISC1_FORCE_2_5G: c_uint = 0x0;
pub const MII_BNX2_SD_MISC1_FORCE: c_uint = 0x10;
pub const MII_BNX2_BLK_ADDR_OVER1G: c_uint = 0x8320;
pub const MII_BNX2_OVER1G_UP1: c_uint = 0x19;
pub const MII_BNX2_BLK_ADDR_BAM_NXTPG: c_uint = 0x8350;
pub const MII_BNX2_BAM_NXTPG_CTL: c_uint = 0x10;
pub const MII_BNX2_NXTPG_CTL_BAM: c_uint = 0x1;
pub const MII_BNX2_NXTPG_CTL_T2: c_uint = 0x2;
pub const MII_BNX2_BLK_ADDR_CL73_USERB0: c_uint = 0x8370;
pub const MII_BNX2_CL73_BAM_CTL1: c_uint = 0x12;
pub const MII_BNX2_CL73_BAM_EN: c_uint = 0x8000;
pub const MII_BNX2_CL73_BAM_STA_MGR_EN: c_uint = 0x4000;
pub const MII_BNX2_CL73_BAM_NP_AFT_BP_EN: c_uint = 0x2000;
pub const MII_BNX2_BLK_ADDR_AER: c_uint = 0xffd0;
pub const MII_BNX2_AER_AER: c_uint = 0x1e;
pub const MII_BNX2_AER_AER_AN_MMD: c_uint = 0x3800;
pub const MII_BNX2_BLK_ADDR_COMBO_IEEEB0: c_uint = 0xffe0;

pub const MAX_ETHERNET_JUMBO_PACKET_SIZE: c_int = 9000;
pub const BNX2_RX_COPY_THRESH: c_int = 128;
pub const BNX2_MISC_ENABLE_DEFAULT: c_uint = 0x17ffffff;
pub const BNX2_START_UNICAST_ADDRESS_INDEX: c_int = 4;
pub const BNX2_END_UNICAST_ADDRESS_INDEX: c_int = 7;

pub const DMA_READ_CHANS: c_int = 5;
pub const DMA_WRITE_CHANS: c_int = 3;
// Use CPU native page size up to 16K for the ring sizes.

pub const BNX2_PAGE_BITS: c_int = 14;

pub const BNX2_MAX_RX_RINGS: c_int = 8;
pub const BNX2_MAX_RX_PG_RINGS: c_int = 32;

// Context size.
pub const CTX_SHIFT: c_int = 7;

pub const PHY_CTX_SHIFT: c_int = 6;

pub const MB_KERNEL_CTX_SHIFT: c_int = 8;

pub const MAX_CID_CNT: c_uint = 0x4000;

pub const INVALID_CID_ADDR: c_uint = 0xffffffff;
pub const TX_CID: c_int = 16;
pub const TX_TSS_CID: c_int = 32;
pub const RX_CID: c_int = 0;
pub const RX_RSS_CID: c_int = 4;
pub const RX_MAX_RSS_RINGS: c_int = 7;

pub const TX_MAX_TSS_RINGS: c_int = 7;

//
// This driver uses new build_skb() API :
// RX ring buffer contains pointer to kmalloc() data only,
// skb are built only after Hardware filled the frame.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_sw_bd {
    pub data: *mut u8,
}

// Its faster to compute this from data than storing it in sw_bd
// (less cache misses)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_sw_pg {
    pub page: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_sw_tx_bd {
    pub skb: *mut sk_buff,
    pub is_gso: c_ushort,
    pub nr_frags: c_ushort,
}

// Buffered flash (Atmel: AT45DB011B) specific information
pub const SEEPROM_PAGE_BITS: c_int = 2;

pub const SEEPROM_PAGE_SIZE: c_int = 4;
pub const SEEPROM_TOTAL_SIZE: c_int = 65536;
pub const BUFFERED_FLASH_PAGE_BITS: c_int = 9;

pub const BUFFERED_FLASH_PAGE_SIZE: c_int = 264;
pub const BUFFERED_FLASH_TOTAL_SIZE: c_uint = 0x21000;
pub const SAIFUN_FLASH_PAGE_BITS: c_int = 8;

pub const SAIFUN_FLASH_PAGE_SIZE: c_int = 256;
pub const SAIFUN_FLASH_BASE_TOTAL_SIZE: c_int = 65536;
pub const ST_MICRO_FLASH_PAGE_BITS: c_int = 8;

pub const ST_MICRO_FLASH_PAGE_SIZE: c_int = 256;
pub const ST_MICRO_FLASH_BASE_TOTAL_SIZE: c_int = 65536;
pub const BCM5709_FLASH_PAGE_BITS: c_int = 8;

pub const BCM5709_FLASH_PAGE_SIZE: c_int = 256;
pub const NVRAM_TIMEOUT_COUNT: c_int = 30000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flash_spec {
    pub strapping: u32,
    pub config1: u32,
    pub config2: u32,
    pub config3: u32,
    pub write1: u32,
    pub flags: u32,
pub const BNX2_NV_BUFFERED: c_uint = 0x00000001;
pub const BNX2_NV_TRANSLATE: c_uint = 0x00000002;
pub const BNX2_NV_WREN: c_uint = 0x00000004;
    pub page_bits: u32,
    pub page_size: u32,
    pub addr_mask: u32,
    pub total_size: u32,
    pub name: *mut u8,
}

pub const BNX2_MAX_MSIX_HW_VEC: c_int = 9;
pub const BNX2_MAX_MSIX_VEC: c_int = 9;

pub const BNX2_MIN_MSIX_VEC: c_int = 2;

pub const BNX2_MIN_MSIX_VEC: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_irq {
    pub handler: irq_handler_t,
    pub vector: c_uint,
    pub requested: u8,
    pub 2]: char name[IFNAMSIZ +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_tx_ring_info {
    pub tx_prod_bseq: u32,
    pub tx_prod: u16,
    pub tx_bidx_addr: u32,
    pub tx_bseq_addr: u32,
    pub tx_desc_ring: *mut bnx2_tx_bd,
    pub tx_buf_ring: *mut bnx2_sw_tx_bd,
    pub tx_cons: u16,
    pub hw_tx_cons: u16,
    pub tx_desc_mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_rx_ring_info {
    pub rx_prod_bseq: u32,
    pub rx_prod: u16,
    pub rx_cons: u16,
    pub rx_bidx_addr: u32,
    pub rx_bseq_addr: u32,
    pub rx_pg_bidx_addr: u32,
    pub rx_pg_prod: u16,
    pub rx_pg_cons: u16,
    pub rx_buf_ring: *mut bnx2_sw_bd,
    pub rx_desc_ring: [*mut bnx2_rx_bd; BNX2_MAX_RX_RINGS],
    pub rx_pg_ring: *mut bnx2_sw_pg,
    pub rx_pg_desc_ring: [*mut bnx2_rx_bd; BNX2_MAX_RX_PG_RINGS],
    pub rx_desc_mapping: [dma_addr_t; BNX2_MAX_RX_RINGS],
    pub rx_pg_desc_mapping: [dma_addr_t; BNX2_MAX_RX_PG_RINGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_napi {
    pub ____cacheline_aligned: napi_napi,
    pub bp: *mut bnx2,
    pub msi: *mut status_block,
    pub msix: *mut status_block_msix,
    pub status_blk: },
    pub hw_tx_cons_ptr: *mut u16,
    pub hw_rx_cons_ptr: *mut u16,
    pub last_status_idx: u32,
    pub int_num: u32,

    pub cnic_tag: u32,
    pub cnic_present: c_int,

    pub rx_ring: bnx2_rx_ring_info,
    pub tx_ring: bnx2_tx_ring_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2 {
// Fields used in the tx and intr/napi performance paths are grouped
// together in the beginning of the structure.
    pub regview: *mut void __iomem,
    pub dev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub intr_sem: core::sync::atomic::AtomicI32,
    pub flags: u32,
pub const BNX2_FLAG_PCIX: c_uint = 0x00000001;
pub const BNX2_FLAG_PCI_32BIT: c_uint = 0x00000002;
pub const BNX2_FLAG_MSIX_CAP: c_uint = 0x00000004;
pub const BNX2_FLAG_NO_WOL: c_uint = 0x00000008;
pub const BNX2_FLAG_USING_MSI: c_uint = 0x00000020;
pub const BNX2_FLAG_ASF_ENABLE: c_uint = 0x00000040;
pub const BNX2_FLAG_MSI_CAP: c_uint = 0x00000080;
pub const BNX2_FLAG_ONE_SHOT_MSI: c_uint = 0x00000100;
pub const BNX2_FLAG_PCIE: c_uint = 0x00000200;
pub const BNX2_FLAG_USING_MSIX: c_uint = 0x00000400;

pub const BNX2_FLAG_JUMBO_BROKEN: c_uint = 0x00000800;
pub const BNX2_FLAG_CAN_KEEP_VLAN: c_uint = 0x00001000;
pub const BNX2_FLAG_BROKEN_STATS: c_uint = 0x00002000;
    pub bnx2_napi: [bnx2_napi; BNX2_MAX_MSIX_VEC],
    pub /: *mut *mut u32 rx_buf_use_size; / useable size,
    pub /: *mut *mut u32 rx_buf_size; / with alignment,
    pub rx_copy_thresh: u32,
    pub rx_jumbo_thresh: u32,
    pub rx_max_ring_idx: u32,
    pub rx_max_pg_ring_idx: u32,
// TX constants
    pub tx_ring_size: c_int,
    pub tx_wake_thresh: u32,

    pub cnic_ops: *mut cnic_ops __rcu,
    pub cnic_data: *mut c_void,

// End of fields used in the performance code paths.
    pub current_interval: c_uint,

    pub timer: timer_list,
    pub reset_task: work_struct,
// Used to synchronize phy accesses.
    pub phy_lock: spinlock_t,
    pub indirect_lock: spinlock_t,
    pub phy_flags: u32,
pub const BNX2_PHY_FLAG_SERDES: c_uint = 0x00000001;
pub const BNX2_PHY_FLAG_CRC_FIX: c_uint = 0x00000002;
pub const BNX2_PHY_FLAG_PARALLEL_DETECT: c_uint = 0x00000004;
pub const BNX2_PHY_FLAG_2_5G_CAPABLE: c_uint = 0x00000008;
pub const BNX2_PHY_FLAG_INT_MODE_MASK: c_uint = 0x00000300;
pub const BNX2_PHY_FLAG_INT_MODE_AUTO_POLLING: c_uint = 0x00000100;
pub const BNX2_PHY_FLAG_INT_MODE_LINK_READY: c_uint = 0x00000200;
pub const BNX2_PHY_FLAG_DIS_EARLY_DAC: c_uint = 0x00000400;
pub const BNX2_PHY_FLAG_REMOTE_PHY_CAP: c_uint = 0x00000800;
pub const BNX2_PHY_FLAG_FORCED_DOWN: c_uint = 0x00001000;
pub const BNX2_PHY_FLAG_NO_PARALLEL: c_uint = 0x00002000;
pub const BNX2_PHY_FLAG_MDIX: c_uint = 0x00004000;
    pub mii_bmcr: u32,
    pub mii_bmsr: u32,
    pub mii_bmsr1: u32,
    pub mii_adv: u32,
    pub mii_lpa: u32,
    pub mii_up1: u32,
    pub chip_id: u32,
// chip num:16-31, rev:12-15, metal:4-11, bond_id:0-3

pub const BNX2_CHIP_5706: c_uint = 0x57060000;
pub const BNX2_CHIP_5708: c_uint = 0x57080000;
pub const BNX2_CHIP_5709: c_uint = 0x57090000;

pub const BNX2_CHIP_REV_Ax: c_uint = 0x00000000;
pub const BNX2_CHIP_REV_Bx: c_uint = 0x00001000;
pub const BNX2_CHIP_REV_Cx: c_uint = 0x00002000;

pub const BNX2_CHIP_ID_5706_A0: c_uint = 0x57060000;
pub const BNX2_CHIP_ID_5706_A1: c_uint = 0x57060010;
pub const BNX2_CHIP_ID_5706_A2: c_uint = 0x57060020;
pub const BNX2_CHIP_ID_5708_A0: c_uint = 0x57080000;
pub const BNX2_CHIP_ID_5708_B0: c_uint = 0x57081000;
pub const BNX2_CHIP_ID_5708_B1: c_uint = 0x57081010;
pub const BNX2_CHIP_ID_5709_A0: c_uint = 0x57090000;
pub const BNX2_CHIP_ID_5709_A1: c_uint = 0x57090010;
// A serdes chip will have the first bit of the bond id set.
pub const BNX2_CHIP_BOND_SERDES_BIT: c_uint = 0x01;
    pub phy_addr: u32,
    pub phy_id: u32,
    pub bus_speed_mhz: u16,
    pub wol: u8,
    pub pad: u8,
    pub fw_wr_seq: u16,
    pub fw_drv_pulse_wr_seq: u16,
    pub fw_last_msg: u32,
    pub rx_max_ring: c_int,
    pub rx_ring_size: c_int,
    pub rx_max_pg_ring: c_int,
    pub rx_pg_ring_size: c_int,
    pub tx_quick_cons_trip: u16,
    pub tx_quick_cons_trip_int: u16,
    pub rx_quick_cons_trip: u16,
    pub rx_quick_cons_trip_int: u16,
    pub comp_prod_trip: u16,
    pub comp_prod_trip_int: u16,
    pub tx_ticks: u16,
    pub tx_ticks_int: u16,
    pub com_ticks: u16,
    pub com_ticks_int: u16,
    pub cmd_ticks: u16,
    pub cmd_ticks_int: u16,
    pub rx_ticks: u16,
    pub rx_ticks_int: u16,
    pub stats_ticks: u32,
    pub status_blk_mapping: dma_addr_t,
    pub status_blk: *mut c_void,
    pub stats_blk: *mut statistics_block,
    pub temp_stats_blk: *mut statistics_block,
    pub stats_blk_mapping: dma_addr_t,
    pub ctx_pages: c_int,
    pub ctx_blk: [*mut c_void; 4],
    pub ctx_blk_mapping: [dma_addr_t; 4],
    pub hc_cmd: u32,
    pub rx_mode: u32,
    pub req_line_speed: u16,
    pub req_duplex: u8,
    pub phy_port: u8,
    pub link_up: u8,
    pub line_speed: u16,
    pub duplex: u8,
    pub /: *mut *mut u8 flow_ctrl; / actual flow ctrl settings,
// may be different from
// req_flow_ctrl if autoneg
    pub advertising: u32,
    pub /: *mut *mut u8 req_flow_ctrl; / flow ctrl advertisement,
// settings or forced
// settings
    pub autoneg: u8,
pub const AUTONEG_SPEED: c_int = 1;
pub const AUTONEG_FLOW_CTRL: c_int = 2;
    pub loopback: u8,
pub const MAC_LOOPBACK: c_int = 1;
pub const PHY_LOOPBACK: c_int = 2;
    pub serdes_an_pending: u8,
    pub mac_addr: [u8; 8],
    pub shmem_base: u32,
    pub fw_version: [c_char; 32],
    pub pm_cap: c_int,
    pub pcix_cap: c_int,
    pub flash_info: *const flash_spec,
    pub flash_size: u32,
    pub status_stats_size: c_int,
    pub irq_tbl: [bnx2_irq; BNX2_MAX_MSIX_VEC],
    pub irq_nvecs: c_int,
    pub func: u8,
    pub num_tx_rings: u8,
    pub num_rx_rings: u8,
    pub num_req_tx_rings: c_int,
    pub num_req_rx_rings: c_int,
    pub leds_save: u32,
    pub idle_chk_status_idx: u32,

    pub cnic_lock: mutex,
    pub cnic_eth_dev: cnic_eth_dev,
    pub ): *mut *mut *mut cnic_eth_dev (cnic_probe)(net_device,

    pub mips_firmware: *const firmware,
    pub rv2p_firmware: *const firmware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_reg {
    pub mode: u32,
    pub mode_value_halt: u32,
    pub mode_value_sstep: u32,
    pub state: u32,
    pub state_value_clear: u32,
    pub gpr0: u32,
    pub evmask: u32,
    pub pc: u32,
    pub inst: u32,
    pub bp: u32,
    pub spad_base: u32,
    pub mips_view_base: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_fw_file_section {
    pub addr: __be32,
    pub len: __be32,
    pub offset: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_mips_fw_file_entry {
    pub start_addr: __be32,
    pub text: bnx2_fw_file_section,
    pub data: bnx2_fw_file_section,
    pub rodata: bnx2_fw_file_section,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_rv2p_fw_file_entry {
    pub rv2p: bnx2_fw_file_section,
    pub fixup: [__be32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_mips_fw_file {
    pub com: bnx2_mips_fw_file_entry,
    pub cp: bnx2_mips_fw_file_entry,
    pub rxp: bnx2_mips_fw_file_entry,
    pub tpat: bnx2_mips_fw_file_entry,
    pub txp: bnx2_mips_fw_file_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnx2_rv2p_fw_file {
    pub proc1: bnx2_rv2p_fw_file_entry,
    pub proc2: bnx2_rv2p_fw_file_entry,
}

pub const RV2P_P1_FIXUP_PAGE_SIZE_IDX: c_int = 0;
pub const RV2P_BD_PAGE_SIZE_MSK: c_uint = 0xffff;

pub const RV2P_PROC1: c_int = 0;
pub const RV2P_PROC2: c_int = 1;
// This value (in milliseconds) determines the frequency of the driver
// issuing the PULSE message code.  The firmware monitors this periodic
// pulse to determine when to switch to an OS-absent mode.
pub const BNX2_DRV_PULSE_PERIOD_MS: c_int = 250;
// This value (in milliseconds) determines how long the driver should
// wait for an acknowledgement from the firmware before timing out.  Once
// the firmware has timed out, the driver will assume there is no firmware
// running and there won't be any firmware-driver synchronization during a
// driver reset.
pub const BNX2_FW_ACK_TIME_OUT_MS: c_int = 1000;
pub const BNX2_DRV_RESET_SIGNATURE: c_uint = 0x00000000;
pub const BNX2_DRV_RESET_SIGNATURE_MAGIC: c_uint = 0x4841564b /* HAVK */;
// #define DRV_RESET_SIGNATURE_MAGIC		 0x47495352 /* RSIG
pub const BNX2_DRV_MB: c_uint = 0x00000004;
pub const BNX2_DRV_MSG_CODE: c_uint = 0xff000000;
pub const BNX2_DRV_MSG_CODE_RESET: c_uint = 0x01000000;
pub const BNX2_DRV_MSG_CODE_UNLOAD: c_uint = 0x02000000;
pub const BNX2_DRV_MSG_CODE_SHUTDOWN: c_uint = 0x03000000;
pub const BNX2_DRV_MSG_CODE_SUSPEND_WOL: c_uint = 0x04000000;
pub const BNX2_DRV_MSG_CODE_FW_TIMEOUT: c_uint = 0x05000000;
pub const BNX2_DRV_MSG_CODE_PULSE: c_uint = 0x06000000;
pub const BNX2_DRV_MSG_CODE_DIAG: c_uint = 0x07000000;
pub const BNX2_DRV_MSG_CODE_SUSPEND_NO_WOL: c_uint = 0x09000000;
pub const BNX2_DRV_MSG_CODE_UNLOAD_LNK_DN: c_uint = 0x0b000000;
pub const BNX2_DRV_MSG_CODE_KEEP_VLAN_UPDATE: c_uint = 0x0d000000;
pub const BNX2_DRV_MSG_CODE_CMD_SET_LINK: c_uint = 0x10000000;
pub const BNX2_DRV_MSG_DATA: c_uint = 0x00ff0000;
pub const BNX2_DRV_MSG_DATA_WAIT0: c_uint = 0x00010000;
pub const BNX2_DRV_MSG_DATA_WAIT1: c_uint = 0x00020000;
pub const BNX2_DRV_MSG_DATA_WAIT2: c_uint = 0x00030000;
pub const BNX2_DRV_MSG_DATA_WAIT3: c_uint = 0x00040000;
pub const BNX2_DRV_MSG_SEQ: c_uint = 0x0000ffff;
pub const BNX2_FW_MB: c_uint = 0x00000008;
pub const BNX2_FW_MSG_ACK: c_uint = 0x0000ffff;
pub const BNX2_FW_MSG_STATUS_MASK: c_uint = 0x00ff0000;
pub const BNX2_FW_MSG_STATUS_OK: c_uint = 0x00000000;
pub const BNX2_FW_MSG_STATUS_FAILURE: c_uint = 0x00ff0000;
pub const BNX2_LINK_STATUS: c_uint = 0x0000000c;
pub const BNX2_LINK_STATUS_INIT_VALUE: c_uint = 0xffffffff;
pub const BNX2_LINK_STATUS_LINK_UP: c_uint = 0x1;
pub const BNX2_LINK_STATUS_LINK_DOWN: c_uint = 0x0;
pub const BNX2_LINK_STATUS_SPEED_MASK: c_uint = 0x1e;

pub const BNX2_DRV_PULSE_MB: c_uint = 0x00000010;
pub const BNX2_DRV_PULSE_SEQ_MASK: c_uint = 0x00007fff;
// Indicate to the firmware not to go into the
// OS absent when it is not getting driver pulse.
// This is used for debugging.
pub const BNX2_DRV_MSG_DATA_PULSE_CODE_ALWAYS_ALIVE: c_uint = 0x00080000;
pub const BNX2_DRV_MB_ARG0: c_uint = 0x00000014;

pub const BNX2_DEV_INFO_SIGNATURE: c_uint = 0x00000020;
pub const BNX2_DEV_INFO_SIGNATURE_MAGIC: c_uint = 0x44564900;
pub const BNX2_DEV_INFO_SIGNATURE_MAGIC_MASK: c_uint = 0xffffff00;
pub const BNX2_DEV_INFO_FEATURE_CFG_VALID: c_uint = 0x01;
pub const BNX2_DEV_INFO_SECONDARY_PORT: c_uint = 0x80;
pub const BNX2_DEV_INFO_DRV_ALWAYS_ALIVE: c_uint = 0x40;
pub const BNX2_SHARED_HW_CFG_PART_NUM: c_uint = 0x00000024;
pub const BNX2_SHARED_HW_CFG_POWER_DISSIPATED: c_uint = 0x00000034;
pub const BNX2_SHARED_HW_CFG_POWER_STATE_D3_MASK: c_uint = 0xff000000;
pub const BNX2_SHARED_HW_CFG_POWER_STATE_D2_MASK: c_uint = 0xff0000;
pub const BNX2_SHARED_HW_CFG_POWER_STATE_D1_MASK: c_uint = 0xff00;
pub const BNX2_SHARED_HW_CFG_POWER_STATE_D0_MASK: c_uint = 0xff;

pub const BNX2_SHARED_HW_CFG_CONFIG: c_uint = 0x0000003c;
pub const BNX2_SHARED_HW_CFG_DESIGN_NIC: c_int = 0;
pub const BNX2_SHARED_HW_CFG_DESIGN_LOM: c_uint = 0x1;
pub const BNX2_SHARED_HW_CFG_PHY_COPPER: c_int = 0;
pub const BNX2_SHARED_HW_CFG_PHY_FIBER: c_uint = 0x2;
pub const BNX2_SHARED_HW_CFG_PHY_2_5G: c_uint = 0x20;
pub const BNX2_SHARED_HW_CFG_PHY_BACKPLANE: c_uint = 0x40;
pub const BNX2_SHARED_HW_CFG_LED_MODE_SHIFT_BITS: c_int = 8;
pub const BNX2_SHARED_HW_CFG_LED_MODE_MASK: c_uint = 0x300;
pub const BNX2_SHARED_HW_CFG_LED_MODE_MAC: c_int = 0;
pub const BNX2_SHARED_HW_CFG_LED_MODE_GPHY1: c_uint = 0x100;
pub const BNX2_SHARED_HW_CFG_LED_MODE_GPHY2: c_uint = 0x200;
pub const BNX2_SHARED_HW_CFG_GIG_LINK_ON_VAUX: c_uint = 0x8000;
pub const BNX2_SHARED_HW_CFG_CONFIG2: c_uint = 0x00000040;
pub const BNX2_SHARED_HW_CFG2_NVM_SIZE_MASK: c_uint = 0x00fff000;
pub const BNX2_DEV_INFO_BC_REV: c_uint = 0x0000004c;
pub const BNX2_PORT_HW_CFG_MAC_UPPER: c_uint = 0x00000050;
pub const BNX2_PORT_HW_CFG_UPPERMAC_MASK: c_uint = 0xffff;
pub const BNX2_PORT_HW_CFG_MAC_LOWER: c_uint = 0x00000054;
pub const BNX2_PORT_HW_CFG_CONFIG: c_uint = 0x00000058;
pub const BNX2_PORT_HW_CFG_CFG_TXCTL3_MASK: c_uint = 0x0000ffff;
pub const BNX2_PORT_HW_CFG_CFG_DFLT_LINK_MASK: c_uint = 0x001f0000;
pub const BNX2_PORT_HW_CFG_CFG_DFLT_LINK_AN: c_uint = 0x00000000;
pub const BNX2_PORT_HW_CFG_CFG_DFLT_LINK_1G: c_uint = 0x00030000;
pub const BNX2_PORT_HW_CFG_CFG_DFLT_LINK_2_5G: c_uint = 0x00040000;
pub const BNX2_PORT_HW_CFG_IMD_MAC_A_UPPER: c_uint = 0x00000068;
pub const BNX2_PORT_HW_CFG_IMD_MAC_A_LOWER: c_uint = 0x0000006c;
pub const BNX2_PORT_HW_CFG_IMD_MAC_B_UPPER: c_uint = 0x00000070;
pub const BNX2_PORT_HW_CFG_IMD_MAC_B_LOWER: c_uint = 0x00000074;
pub const BNX2_PORT_HW_CFG_ISCSI_MAC_UPPER: c_uint = 0x00000078;
pub const BNX2_PORT_HW_CFG_ISCSI_MAC_LOWER: c_uint = 0x0000007c;
pub const BNX2_DEV_INFO_PER_PORT_HW_CONFIG2: c_uint = 0x000000b4;
pub const BNX2_DEV_INFO_FORMAT_REV: c_uint = 0x000000c4;
pub const BNX2_DEV_INFO_FORMAT_REV_MASK: c_uint = 0xff000000;

pub const BNX2_SHARED_FEATURE: c_uint = 0x000000c8;
pub const BNX2_SHARED_FEATURE_MASK: c_uint = 0xffffffff;
pub const BNX2_PORT_FEATURE: c_uint = 0x000000d8;
pub const BNX2_PORT2_FEATURE: c_uint = 0x00000014c;
pub const BNX2_PORT_FEATURE_WOL_ENABLED: c_uint = 0x01000000;
pub const BNX2_PORT_FEATURE_MBA_ENABLED: c_uint = 0x02000000;
pub const BNX2_PORT_FEATURE_ASF_ENABLED: c_uint = 0x04000000;
pub const BNX2_PORT_FEATURE_IMD_ENABLED: c_uint = 0x08000000;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_MASK: c_uint = 0xf;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_DISABLED: c_uint = 0x0;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_64K: c_uint = 0x1;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_128K: c_uint = 0x2;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_256K: c_uint = 0x3;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_512K: c_uint = 0x4;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_1M: c_uint = 0x5;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_2M: c_uint = 0x6;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_4M: c_uint = 0x7;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_8M: c_uint = 0x8;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_16M: c_uint = 0x9;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_32M: c_uint = 0xa;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_64M: c_uint = 0xb;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_128M: c_uint = 0xc;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_256M: c_uint = 0xd;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_512M: c_uint = 0xe;
pub const BNX2_PORT_FEATURE_BAR1_SIZE_1G: c_uint = 0xf;
pub const BNX2_PORT_FEATURE_WOL: c_uint = 0xdc;
pub const BNX2_PORT2_FEATURE_WOL: c_uint = 0x150;
pub const BNX2_PORT_FEATURE_WOL_DEFAULT_SHIFT_BITS: c_int = 4;
pub const BNX2_PORT_FEATURE_WOL_DEFAULT_MASK: c_uint = 0x30;
pub const BNX2_PORT_FEATURE_WOL_DEFAULT_DISABLE: c_int = 0;
pub const BNX2_PORT_FEATURE_WOL_DEFAULT_MAGIC: c_uint = 0x10;
pub const BNX2_PORT_FEATURE_WOL_DEFAULT_ACPI: c_uint = 0x20;
pub const BNX2_PORT_FEATURE_WOL_DEFAULT_MAGIC_AND_ACPI: c_uint = 0x30;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_MASK: c_uint = 0xf;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_AUTONEG: c_int = 0;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_10HALF: c_int = 1;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_10FULL: c_int = 2;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_100HALF: c_int = 3;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_100FULL: c_int = 4;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_1000HALF: c_int = 5;
pub const BNX2_PORT_FEATURE_WOL_LINK_SPEED_1000FULL: c_int = 6;
pub const BNX2_PORT_FEATURE_WOL_AUTONEG_ADVERTISE_1000: c_uint = 0x40;
pub const BNX2_PORT_FEATURE_WOL_RESERVED_PAUSE_CAP: c_uint = 0x400;
pub const BNX2_PORT_FEATURE_WOL_RESERVED_ASYM_PAUSE_CAP: c_uint = 0x800;
pub const BNX2_PORT_FEATURE_MBA: c_uint = 0xe0;
pub const BNX2_PORT2_FEATURE_MBA: c_uint = 0x154;
pub const BNX2_PORT_FEATURE_MBA_BOOT_AGENT_TYPE_SHIFT_BITS: c_int = 0;
pub const BNX2_PORT_FEATURE_MBA_BOOT_AGENT_TYPE_MASK: c_uint = 0x3;
pub const BNX2_PORT_FEATURE_MBA_BOOT_AGENT_TYPE_PXE: c_int = 0;
pub const BNX2_PORT_FEATURE_MBA_BOOT_AGENT_TYPE_RPL: c_int = 1;
pub const BNX2_PORT_FEATURE_MBA_BOOT_AGENT_TYPE_BOOTP: c_int = 2;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_SHIFT_BITS: c_int = 2;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_MASK: c_uint = 0x3c;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_AUTONEG: c_int = 0;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_10HALF: c_uint = 0x4;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_10FULL: c_uint = 0x8;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_100HALF: c_uint = 0xc;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_100FULL: c_uint = 0x10;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_1000HALF: c_uint = 0x14;
pub const BNX2_PORT_FEATURE_MBA_LINK_SPEED_1000FULL: c_uint = 0x18;
pub const BNX2_PORT_FEATURE_MBA_SETUP_PROMPT_ENABLE: c_uint = 0x40;
pub const BNX2_PORT_FEATURE_MBA_HOTKEY_CTRL_S: c_int = 0;
pub const BNX2_PORT_FEATURE_MBA_HOTKEY_CTRL_B: c_uint = 0x80;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_SHIFT_BITS: c_int = 8;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_MASK: c_uint = 0xff00;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_DISABLED: c_int = 0;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_1K: c_uint = 0x100;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_2K: c_uint = 0x200;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_4K: c_uint = 0x300;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_8K: c_uint = 0x400;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_16K: c_uint = 0x500;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_32K: c_uint = 0x600;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_64K: c_uint = 0x700;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_128K: c_uint = 0x800;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_256K: c_uint = 0x900;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_512K: c_uint = 0xa00;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_1M: c_uint = 0xb00;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_2M: c_uint = 0xc00;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_4M: c_uint = 0xd00;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_8M: c_uint = 0xe00;
pub const BNX2_PORT_FEATURE_MBA_EXP_ROM_SIZE_16M: c_uint = 0xf00;
pub const BNX2_PORT_FEATURE_MBA_MSG_TIMEOUT_SHIFT_BITS: c_int = 16;
pub const BNX2_PORT_FEATURE_MBA_MSG_TIMEOUT_MASK: c_uint = 0xf0000;
pub const BNX2_PORT_FEATURE_MBA_BIOS_BOOTSTRAP_SHIFT_BITS: c_int = 20;
pub const BNX2_PORT_FEATURE_MBA_BIOS_BOOTSTRAP_MASK: c_uint = 0x300000;
pub const BNX2_PORT_FEATURE_MBA_BIOS_BOOTSTRAP_AUTO: c_int = 0;
pub const BNX2_PORT_FEATURE_MBA_BIOS_BOOTSTRAP_BBS: c_uint = 0x100000;
pub const BNX2_PORT_FEATURE_MBA_BIOS_BOOTSTRAP_INT18H: c_uint = 0x200000;
pub const BNX2_PORT_FEATURE_MBA_BIOS_BOOTSTRAP_INT19H: c_uint = 0x300000;
pub const BNX2_PORT_FEATURE_IMD: c_uint = 0xe4;
pub const BNX2_PORT2_FEATURE_IMD: c_uint = 0x158;
pub const BNX2_PORT_FEATURE_IMD_LINK_OVERRIDE_DEFAULT: c_int = 0;
pub const BNX2_PORT_FEATURE_IMD_LINK_OVERRIDE_ENABLE: c_int = 1;
pub const BNX2_PORT_FEATURE_VLAN: c_uint = 0xe8;
pub const BNX2_PORT2_FEATURE_VLAN: c_uint = 0x15c;
pub const BNX2_PORT_FEATURE_MBA_VLAN_TAG_MASK: c_uint = 0xffff;
pub const BNX2_PORT_FEATURE_MBA_VLAN_ENABLE: c_uint = 0x10000;
pub const BNX2_MFW_VER_PTR: c_uint = 0x00000014c;
pub const BNX2_BC_STATE_RESET_TYPE: c_uint = 0x000001c0;
pub const BNX2_BC_STATE_RESET_TYPE_SIG: c_uint = 0x00005254;
pub const BNX2_BC_STATE_RESET_TYPE_SIG_MASK: c_uint = 0x0000ffff;

pub const BNX2_BC_RESET_TYPE: c_uint = 0x000001c0;
pub const BNX2_BC_STATE: c_uint = 0x000001c4;
pub const BNX2_BC_STATE_ERR_MASK: c_uint = 0x0000ff00;
pub const BNX2_BC_STATE_SIGN: c_uint = 0x42530000;
pub const BNX2_BC_STATE_SIGN_MASK: c_uint = 0xffff0000;

pub const BNX2_BC_STATE_CONDITION: c_uint = 0x000001c8;
pub const BNX2_CONDITION_MFW_RUN_UNKNOWN: c_uint = 0x00000000;
pub const BNX2_CONDITION_MFW_RUN_IPMI: c_uint = 0x00002000;
pub const BNX2_CONDITION_MFW_RUN_UMP: c_uint = 0x00004000;
pub const BNX2_CONDITION_MFW_RUN_NCSI: c_uint = 0x00006000;
pub const BNX2_CONDITION_MFW_RUN_NONE: c_uint = 0x0000e000;
pub const BNX2_CONDITION_MFW_RUN_MASK: c_uint = 0x0000e000;
pub const BNX2_CONDITION_PM_STATE_MASK: c_uint = 0x00030000;
pub const BNX2_CONDITION_PM_STATE_FULL: c_uint = 0x00030000;
pub const BNX2_CONDITION_PM_STATE_PREP: c_uint = 0x00020000;
pub const BNX2_CONDITION_PM_STATE_UNPREP: c_uint = 0x00010000;
pub const BNX2_BC_STATE_DEBUG_CMD: c_uint = 0x1dc;
pub const BNX2_BC_STATE_BC_DBG_CMD_SIGNATURE: c_uint = 0x42440000;
pub const BNX2_BC_STATE_BC_DBG_CMD_SIGNATURE_MASK: c_uint = 0xffff0000;
pub const BNX2_BC_STATE_BC_DBG_CMD_LOOP_CNT_MASK: c_uint = 0xffff;
pub const BNX2_BC_STATE_BC_DBG_CMD_LOOP_INFINITE: c_uint = 0xffff;
pub const BNX2_FW_EVT_CODE_MB: c_uint = 0x354;
pub const BNX2_FW_EVT_CODE_SW_TIMER_EXPIRATION_EVENT: c_uint = 0x00000000;
pub const BNX2_FW_EVT_CODE_LINK_EVENT: c_uint = 0x00000001;
pub const BNX2_DRV_ACK_CAP_MB: c_uint = 0x364;
pub const BNX2_DRV_ACK_CAP_SIGNATURE: c_uint = 0x35450000;
pub const BNX2_CAPABILITY_SIGNATURE_MASK: c_uint = 0xFFFF0000;
pub const BNX2_FW_CAP_MB: c_uint = 0x368;
pub const BNX2_FW_CAP_SIGNATURE: c_uint = 0xaa550000;
pub const BNX2_FW_ACK_DRV_SIGNATURE: c_uint = 0x52500000;
pub const BNX2_FW_CAP_SIGNATURE_MASK: c_uint = 0xffff0000;
pub const BNX2_FW_CAP_REMOTE_PHY_CAPABLE: c_uint = 0x00000001;
pub const BNX2_FW_CAP_REMOTE_PHY_PRESENT: c_uint = 0x00000002;
pub const BNX2_FW_CAP_MFW_CAN_KEEP_VLAN: c_uint = 0x00000008;
pub const BNX2_FW_CAP_BC_CAN_KEEP_VLAN: c_uint = 0x00000010;

pub const BNX2_RPHY_SIGNATURE: c_uint = 0x36c;
pub const BNX2_RPHY_LOAD_SIGNATURE: c_uint = 0x5a5a5a5a;
pub const BNX2_RPHY_FLAGS: c_uint = 0x370;
pub const BNX2_RPHY_SERDES_LINK: c_uint = 0x374;
pub const BNX2_RPHY_COPPER_LINK: c_uint = 0x378;
pub const BNX2_ISCSI_INITIATOR: c_uint = 0x3dc;
pub const BNX2_ISCSI_INITIATOR_EN: c_uint = 0x00080000;
pub const BNX2_ISCSI_MAX_CONN: c_uint = 0x3e4;
pub const BNX2_ISCSI_MAX_CONN_MASK: c_uint = 0xffff0000;
pub const BNX2_ISCSI_MAX_CONN_SHIFT: c_int = 16;
pub const HOST_VIEW_SHMEM_BASE: c_uint = 0x167c00;

