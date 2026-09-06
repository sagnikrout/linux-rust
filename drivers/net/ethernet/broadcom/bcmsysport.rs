//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bcmsysport.h
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
// Broadcom BCM7xxx System Port Ethernet MAC driver
//
// Copyright (C) 2014 Broadcom Corporation
//

// Receive/transmit descriptor format
pub const DESC_ADDR_HI_STATUS_LEN: c_uint = 0x00;
pub const DESC_ADDR_HI_SHIFT: c_int = 0;
pub const DESC_ADDR_HI_MASK: c_uint = 0xff;
pub const DESC_STATUS_SHIFT: c_int = 8;
pub const DESC_STATUS_MASK: c_uint = 0x3ff;
pub const DESC_LEN_SHIFT: c_int = 18;
pub const DESC_LEN_MASK: c_uint = 0x7fff;
pub const DESC_ADDR_LO: c_uint = 0x04;
// HW supports 40-bit addressing hence the

// Default RX buffer allocation size
pub const RX_BUF_LENGTH: c_int = 2048;
// Body(1500) + EH_SIZE(14) + VLANTAG(4) + BRCMTAG(4) + FCS(4) = 1526.
// 1536 is multiple of 256 bytes
//
pub const ENET_BRCM_TAG_LEN: c_int = 4;
pub const ENET_PAD: c_int = 10;

// Transmit status block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_tsb {
    pub pcp_dei_vid: u32,
pub const PCP_DEI_MASK: c_uint = 0xf;
pub const VID_SHIFT: c_int = 4;
pub const VID_MASK: c_uint = 0xfff;
    pub l4_ptr_dest_map: u32,
pub const L4_CSUM_PTR_MASK: c_uint = 0x1ff;
pub const L4_PTR_SHIFT: c_int = 9;
pub const L4_PTR_MASK: c_uint = 0x1ff;

pub const DEST_MAP_SHIFT: c_int = 20;
pub const DEST_MAP_MASK: c_uint = 0x1ff;
}

// Receive status block uses the same
// definitions as the DMA descriptor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_rsb {
    pub rx_status_len: u32,
    pub brcm_egress_tag: u32,
}

// Common Receive/Transmit status bits

// Receive Status bits
pub const RX_STATUS_UCAST: c_int = 0;
pub const RX_STATUS_BCAST: c_uint = 0x04;
pub const RX_STATUS_MCAST: c_uint = 0x08;
pub const RX_STATUS_L2_MCAST: c_uint = 0x0c;

// Transmit Status bits
pub const TX_STATUS_VLAN_NO_ACT: c_uint = 0x00;
pub const TX_STATUS_VLAN_PCP_TSB: c_uint = 0x01;
pub const TX_STATUS_VLAN_QUEUE: c_uint = 0x02;
pub const TX_STATUS_VLAN_VID_TSB: c_uint = 0x03;

pub const TX_STATUS_BRCM_TAG_NO_ACT: c_int = 0;
pub const TX_STATUS_BRCM_TAG_ZERO: c_uint = 0x10;
pub const TX_STATUS_BRCM_TAG_ONE_QUEUE: c_uint = 0x20;
pub const TX_STATUS_BRCM_TAG_ONE_TSB: c_uint = 0x30;

// Specific register definitions
pub const SYS_PORT_TOPCTRL_OFFSET: c_int = 0;
pub const REV_CNTL: c_uint = 0x00;
pub const REV_MASK: c_uint = 0xffff;
pub const RX_FLUSH_CNTL: c_uint = 0x04;

pub const TX_FLUSH_CNTL: c_uint = 0x08;

pub const MISC_CNTL: c_uint = 0x0c;

// Level-2 Interrupt controller offsets and defines
pub const SYS_PORT_INTRL2_0_OFFSET: c_uint = 0x200;
pub const SYS_PORT_INTRL2_1_OFFSET: c_uint = 0x240;
pub const INTRL2_CPU_STATUS: c_uint = 0x00;
pub const INTRL2_CPU_SET: c_uint = 0x04;
pub const INTRL2_CPU_CLEAR: c_uint = 0x08;
pub const INTRL2_CPU_MASK_STATUS: c_uint = 0x0c;
pub const INTRL2_CPU_MASK_SET: c_uint = 0x10;
pub const INTRL2_CPU_MASK_CLEAR: c_uint = 0x14;
// Level-2 instance 0 interrupt bits

// SYSTEMPORT Lite groups the TX queues interrupts on instance 0
pub const INTRL2_0_TDMA_MBDONE_SHIFT: c_int = 12;

// RXCHK offset and defines
pub const SYS_PORT_RXCHK_OFFSET: c_uint = 0x300;
pub const RXCHK_CONTROL: c_uint = 0x00;

pub const RXCHK_BRCM_TAG_MATCH_SHIFT: c_int = 4;
pub const RXCHK_BRCM_TAG_MATCH_MASK: c_uint = 0xff;

pub const RXCHK_BRCM_TAG0: c_uint = 0x04;

pub const RXCHK_BRCM_TAG0_MASK: c_uint = 0x24;

pub const RXCHK_BRCM_TAG_MATCH_STATUS: c_uint = 0x44;
pub const RXCHK_ETHERTYPE: c_uint = 0x48;
pub const RXCHK_BAD_CSUM_CNTR: c_uint = 0x4C;
pub const RXCHK_OTHER_DISC_CNTR: c_uint = 0x50;
pub const RXCHK_BRCM_TAG_MAX: c_int = 8;
pub const RXCHK_BRCM_TAG_CID_SHIFT: c_int = 16;
pub const RXCHK_BRCM_TAG_CID_MASK: c_uint = 0xff;
// TXCHCK offsets and defines
pub const SYS_PORT_TXCHK_OFFSET: c_uint = 0x380;
pub const TXCHK_PKT_RDY_THRESH: c_uint = 0x00;
// Receive buffer offset and defines
pub const SYS_PORT_RBUF_OFFSET: c_uint = 0x400;
pub const RBUF_CONTROL: c_uint = 0x00;

pub const RBUF_RESUME_THRESH_SHIFT: c_int = 4;
pub const RBUF_RESUME_THRESH_MASK: c_uint = 0xff;
pub const RBUF_OK_TO_SEND_SHIFT: c_int = 12;
pub const RBUF_OK_TO_SEND_MASK: c_uint = 0xff;

// SYSTEMPORT Lite uses two bits here

pub const RBUF_PKT_RDY_THRESH: c_uint = 0x04;
pub const RBUF_STATUS: c_uint = 0x08;

pub const RBUF_OVFL_DISC_CNTR: c_uint = 0x0c;
pub const RBUF_ERR_PKT_CNTR: c_uint = 0x10;
// Transmit buffer offset and defines
pub const SYS_PORT_TBUF_OFFSET: c_uint = 0x600;
pub const TBUF_CONTROL: c_uint = 0x00;

pub const TBUF_MAX_PKT_THRESH_SHIFT: c_int = 1;
pub const TBUF_MAX_PKT_THRESH_MASK: c_uint = 0x1f;
pub const TBUF_FULL_THRESH_SHIFT: c_int = 8;
pub const TBUF_FULL_THRESH_MASK: c_uint = 0x1f;
// UniMAC offset and defines
pub const SYS_PORT_UMAC_OFFSET: c_uint = 0x800;
pub const UMAC_MIB_START: c_uint = 0x400;
// There is a 0xC gap between the end of RX and beginning of TX stats and then
// between the end of TX stats and the beginning of the RX RUNT
//
pub const UMAC_MIB_STAT_OFFSET: c_uint = 0xc;
pub const UMAC_MIB_CTRL: c_uint = 0x580;

// These offsets are valid for SYSTEMPORT and SYSTEMPORT Lite
pub const UMAC_MPD_CTRL: c_uint = 0x620;

pub const MSEQ_LEN_SHIFT: c_int = 16;
pub const MSEQ_LEN_MASK: c_uint = 0xff;

pub const UMAC_PSW_MS: c_uint = 0x624;
pub const UMAC_PSW_LS: c_uint = 0x628;
pub const UMAC_MDF_CTRL: c_uint = 0x650;
pub const UMAC_MDF_ADDR: c_uint = 0x654;
// Only valid on SYSTEMPORT Lite
pub const SYS_PORT_GIB_OFFSET: c_uint = 0x1000;
pub const GIB_CONTROL: c_uint = 0x00;

pub const GIB_GTX_CLK_SEL_SHIFT: c_int = 4;

pub const GIB_FCS_STRIP_SHIFT: c_int = 6;

pub const GIB_PREAMBLE_LEN_SHIFT: c_int = 12;
pub const GIB_PREAMBLE_LEN_MASK: c_uint = 0xf;
pub const GIB_IPG_LEN_SHIFT: c_int = 16;
pub const GIB_IPG_LEN_MASK: c_uint = 0x3f;
pub const GIB_PAD_EXTENSION_SHIFT: c_int = 22;
pub const GIB_PAD_EXTENSION_MASK: c_uint = 0x3f;
pub const GIB_MAC1: c_uint = 0x08;
pub const GIB_MAC0: c_uint = 0x0c;
// Receive DMA offset and defines
pub const SYS_PORT_RDMA_OFFSET: c_uint = 0x2000;
pub const RDMA_CONTROL: c_uint = 0x1000;

pub const RDMA_BUF_DATA_OFFSET_SHIFT: c_int = 4;
pub const RDMA_BUF_DATA_OFFSET_MASK: c_uint = 0x3ff;
pub const RDMA_STATUS: c_uint = 0x1004;

pub const RDMA_SCB_BURST_SIZE: c_uint = 0x1008;
pub const RDMA_RING_BUF_SIZE: c_uint = 0x100c;
pub const RDMA_RING_SIZE_SHIFT: c_int = 16;
pub const RDMA_WRITE_PTR_HI: c_uint = 0x1010;
pub const RDMA_WRITE_PTR_LO: c_uint = 0x1014;
pub const RDMA_OVFL_DISC_CNTR: c_uint = 0x1018;
pub const RDMA_PROD_INDEX: c_uint = 0x1018;
pub const RDMA_PROD_INDEX_MASK: c_uint = 0xffff;
pub const RDMA_CONS_INDEX: c_uint = 0x101c;
pub const RDMA_CONS_INDEX_MASK: c_uint = 0xffff;
pub const RDMA_START_ADDR_HI: c_uint = 0x1020;
pub const RDMA_START_ADDR_LO: c_uint = 0x1024;
pub const RDMA_END_ADDR_HI: c_uint = 0x1028;
pub const RDMA_END_ADDR_LO: c_uint = 0x102c;
pub const RDMA_MBDONE_INTR: c_uint = 0x1030;
pub const RDMA_INTR_THRESH_MASK: c_uint = 0x1ff;
pub const RDMA_TIMEOUT_SHIFT: c_int = 16;
pub const RDMA_TIMEOUT_MASK: c_uint = 0xffff;
pub const RDMA_XON_XOFF_THRESH: c_uint = 0x1034;
pub const RDMA_XON_XOFF_THRESH_MASK: c_uint = 0xffff;
pub const RDMA_XOFF_THRESH_SHIFT: c_int = 16;
pub const RDMA_READ_PTR_HI: c_uint = 0x1038;
pub const RDMA_READ_PTR_LO: c_uint = 0x103c;
pub const RDMA_OVERRIDE: c_uint = 0x1040;

pub const RDMA_TEST: c_uint = 0x1044;

pub const RDMA_DEBUG: c_uint = 0x1048;
// Transmit DMA offset and defines

pub const SYS_PORT_TDMA_OFFSET: c_uint = 0x4000;
pub const TDMA_WRITE_PORT_OFFSET: c_uint = 0x0000;

// Register offsets and defines relatives to a specific ring number
pub const RING_HEAD_TAIL_PTR: c_uint = 0x00;
pub const RING_HEAD_MASK: c_uint = 0x7ff;
pub const RING_TAIL_SHIFT: c_int = 11;
pub const RING_TAIL_MASK: c_uint = 0x7ff;

pub const RING_COUNT: c_uint = 0x04;
pub const RING_COUNT_MASK: c_uint = 0x7ff;
pub const RING_BUFF_DONE_SHIFT: c_int = 11;
pub const RING_BUFF_DONE_MASK: c_uint = 0x7ff;
pub const RING_MAX_HYST: c_uint = 0x08;
pub const RING_MAX_THRESH_MASK: c_uint = 0x7ff;
pub const RING_HYST_THRESH_SHIFT: c_int = 11;
pub const RING_HYST_THRESH_MASK: c_uint = 0x7ff;
pub const RING_INTR_CONTROL: c_uint = 0x0c;
pub const RING_INTR_THRESH_MASK: c_uint = 0x7ff;

pub const RING_TIMEOUT_SHIFT: c_int = 16;
pub const RING_TIMEOUT_MASK: c_uint = 0xffff;
pub const RING_PROD_CONS_INDEX: c_uint = 0x10;
pub const RING_PROD_INDEX_MASK: c_uint = 0xffff;
pub const RING_CONS_INDEX_SHIFT: c_int = 16;
pub const RING_CONS_INDEX_MASK: c_uint = 0xffff;
pub const RING_MAPPING: c_uint = 0x14;
pub const RING_QID_MASK: c_uint = 0x7;
pub const RING_PORT_ID_SHIFT: c_int = 3;
pub const RING_PORT_ID_MASK: c_uint = 0x7;

pub const RING_CREDIT_SHIFT: c_int = 8;
pub const RING_CREDIT_MASK: c_uint = 0xffff;
pub const RING_PCP_DEI_VID: c_uint = 0x18;
pub const RING_VID_MASK: c_uint = 0x7ff;

pub const RING_PCP_SHIFT: c_int = 13;
pub const RING_PCP_MASK: c_uint = 0x7;
pub const RING_PKT_SIZE_ADJ_SHIFT: c_int = 16;
pub const RING_PKT_SIZE_ADJ_MASK: c_uint = 0xf;
pub const TDMA_DESC_RING_SIZE: c_int = 28;
// Defininition for a given TX ring base address

// Ring indexed register addreses

pub const TDMA_CONTROL: c_uint = 0x600;
pub const TDMA_EN: c_int = 0;
pub const TSB_EN: c_int = 1;
// Uses 2 bits on SYSTEMPORT Lite and shifts everything by 1 bit, we
// keep the SYSTEMPORT layout here and adjust with tdma_control_bit()
//
pub const TSB_SWAP0: c_int = 2;
pub const TSB_SWAP1: c_int = 3;
pub const ACB_ALGO: c_int = 3;
pub const BUF_DATA_OFFSET_SHIFT: c_int = 4;
pub const BUF_DATA_OFFSET_MASK: c_uint = 0x3ff;
pub const VLAN_EN: c_int = 14;
pub const SW_BRCM_TAG: c_int = 15;
pub const WNC_KPT_SIZE_UPDATE: c_int = 16;
pub const SYNC_PKT_SIZE: c_int = 17;
pub const ACH_TXDONE_DELAY_SHIFT: c_int = 18;
pub const ACH_TXDONE_DELAY_MASK: c_uint = 0xff;
pub const TDMA_STATUS: c_uint = 0x604;

pub const TDMA_SCB_BURST_SIZE: c_uint = 0x608;
pub const TDMA_OVER_MAX_THRESH_STATUS: c_uint = 0x60c;
pub const TDMA_OVER_HYST_THRESH_STATUS: c_uint = 0x610;
pub const TDMA_TPID: c_uint = 0x614;
pub const TDMA_FREE_LIST_HEAD_TAIL_PTR: c_uint = 0x618;
pub const TDMA_FREE_HEAD_MASK: c_uint = 0x7ff;
pub const TDMA_FREE_TAIL_SHIFT: c_int = 11;
pub const TDMA_FREE_TAIL_MASK: c_uint = 0x7ff;
pub const TDMA_FREE_LIST_COUNT: c_uint = 0x61c;
pub const TDMA_FREE_LIST_COUNT_MASK: c_uint = 0x7ff;
pub const TDMA_TIER2_ARB_CTRL: c_uint = 0x620;
pub const TDMA_ARB_MODE_RR: c_int = 0;
pub const TDMA_ARB_MODE_WEIGHT_RR: c_uint = 0x1;
pub const TDMA_ARB_MODE_STRICT: c_uint = 0x2;
pub const TDMA_ARB_MODE_DEFICIT_RR: c_uint = 0x3;
pub const TDMA_CREDIT_SHIFT: c_int = 4;
pub const TDMA_CREDIT_MASK: c_uint = 0xffff;
pub const TDMA_TIER1_ARB_0_CTRL: c_uint = 0x624;

pub const TDMA_TIER1_ARB_0_QUEUE_EN: c_uint = 0x628;
pub const TDMA_TIER1_ARB_1_CTRL: c_uint = 0x62c;
pub const TDMA_TIER1_ARB_1_QUEUE_EN: c_uint = 0x630;
pub const TDMA_TIER1_ARB_2_CTRL: c_uint = 0x634;
pub const TDMA_TIER1_ARB_2_QUEUE_EN: c_uint = 0x638;
pub const TDMA_TIER1_ARB_3_CTRL: c_uint = 0x63c;
pub const TDMA_TIER1_ARB_3_QUEUE_EN: c_uint = 0x640;
pub const TDMA_SCB_ENDIAN_OVERRIDE: c_uint = 0x644;

pub const TDMA_TEST: c_uint = 0x648;

pub const TDMA_DEBUG: c_uint = 0x64c;
// Number of Receive hardware descriptor words
pub const SP_NUM_HW_RX_DESC_WORDS: c_int = 1024;
pub const SP_LT_NUM_HW_RX_DESC_WORDS: c_int = 512;
// Internal linked-list RAM size
pub const SP_NUM_TX_DESC: c_int = 1536;
pub const SP_LT_NUM_TX_DESC: c_int = 256;
pub const WORDS_PER_DESC: c_int = 2;
// Rx/Tx common counter group.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_pkt_counters {
    pub /: *mut *mut u32 cnt_64; / RO Received/Transmited 64 bytes packet,
    pub /: *mut *mut u32 cnt_127; / RO Rx/Tx 127 bytes packet,
    pub /: *mut *mut u32 cnt_255; / RO Rx/Tx 65-255 bytes packet,
    pub /: *mut *mut u32 cnt_511; / RO Rx/Tx 256-511 bytes packet,
    pub /: *mut *mut u32 cnt_1023; / RO Rx/Tx 512-1023 bytes packet,
    pub /: *mut *mut u32 cnt_1518; / RO Rx/Tx 1024-1518 bytes packet,
    pub /: *mut *mut u32 cnt_mgv; / RO Rx/Tx 1519-1522 good VLAN packet,
    pub packet*/: *mut *mut u32 cnt_2047; / RO Rx/Tx 1522-2047 bytes,
    pub packet*/: *mut *mut u32 cnt_4095; / RO Rx/Tx 2048-4095 bytes,
    pub packet*/: *mut *mut u32 cnt_9216; / RO Rx/Tx 4096-9216 bytes,
}

// RSV, Receive Status Vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_rx_counters {
    pub pkt_cnt: bcm_sysport_pkt_counters,
    pub count*/: *mut *mut u32 pkt; / RO (0x428) Received pkt,
    pub /: *mut *mut u32 bytes; / RO Received byte count,
    pub /: *mut *mut u32 mca; / RO # of Received multicast pkt,
    pub /: *mut *mut u32 bca; / RO # of Receive broadcast pkt,
    pub /: *mut *mut u32 fcs; / RO # of Received FCS error,
    pub pkt*/: *mut *mut u32 cf; / RO # of Received control frame,
    pub /: *mut *mut u32 pf; / RO # of Received pause frame pkt,
    pub /: *mut *mut u32 uo; / RO # of unknown op code pkt,
    pub /: *mut *mut u32 aln; / RO # of alignment error count,
    pub /: *mut *mut u32 flr; / RO # of frame length out of range count,
    pub /: *mut *mut u32 cde; / RO # of code error pkt,
    pub /: *mut *mut u32 fcr; / RO # of carrier sense error pkt,
    pub pkt*/: *mut *mut u32 ovr; / RO # of oversize,
    pub /: *mut *mut u32 jbr; / RO # of jabber count,
    pub pkt*/: *mut *mut u32 mtue; / RO # of MTU error,
    pub /: *mut *mut u32 pok; / RO # of Received good pkt,
    pub /: *mut *mut u32 uc; / RO # of unicast pkt,
    pub /: *mut *mut u32 ppp; / RO # of PPP pkt,
    pub /: *mut *mut u32 rcrc; / RO (0x470),# of CRC match pkt,
}

// TSV, Transmit Status Vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_tx_counters {
    pub pkt_cnt: bcm_sysport_pkt_counters,
    pub /: *mut *mut u32 pkts; / RO (0x4a8) Transmited pkt,
    pub /: *mut *mut u32 mca; / RO # of xmited multicast pkt,
    pub /: *mut *mut u32 bca; / RO # of xmited broadcast pkt,
    pub /: *mut *mut u32 pf; / RO # of xmited pause frame count,
    pub /: *mut *mut u32 cf; / RO # of xmited control frame count,
    pub /: *mut *mut u32 fcs; / RO # of xmited FCS error count,
    pub /: *mut *mut u32 ovr; / RO # of xmited oversize pkt,
    pub /: *mut *mut u32 drf; / RO # of xmited deferral pkt,
    pub pkt*/: *mut *mut u32 edf; / RO # of xmited Excessive deferral,
    pub /: *mut *mut u32 scl; / RO # of xmited single collision pkt,
    pub pkt*/: *mut *mut u32 mcl; / RO # of xmited multiple collision,
    pub /: *mut *mut u32 lcl; / RO # of xmited late collision pkt,
    pub pkt*/: *mut *mut u32 ecl; / RO # of xmited excessive collision,
    pub pkt*/: *mut *mut u32 frg; / RO # of xmited fragments,
    pub /: *mut *mut u32 ncl; / RO # of xmited total collision count,
    pub count*/: *mut *mut u32 jbr; / RO # of xmited jabber,
    pub /: *mut *mut u32 bytes; / RO # of xmited byte count,
    pub /: *mut *mut u32 pok; / RO # of xmited good pkt,
    pub /: *mut *mut u32 uc; / RO (0x4f0) # of xmited unicast pkt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_mib {
    pub rx: bcm_sysport_rx_counters,
    pub tx: bcm_sysport_tx_counters,
    pub rx_runt_cnt: u32,
    pub rx_runt_fcs: u32,
    pub rx_runt_fcs_align: u32,
    pub rx_runt_bytes: u32,
    pub rxchk_bad_csum: u32,
    pub rxchk_other_pkt_disc: u32,
    pub rbuf_ovflow_cnt: u32,
    pub rbuf_err_cnt: u32,
    pub rdma_ovflow_cnt: u32,
    pub alloc_rx_buff_failed: u32,
    pub rx_dma_failed: u32,
    pub tx_dma_failed: u32,
    pub tx_realloc_tsb: u32,
    pub tx_realloc_tsb_failed: u32,
}

// HW maintains a large list of counters
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm_sysport_stat_type {
    BCM_SYSPORT_STAT_NETDEV = -1,
    BCM_SYSPORT_STAT_NETDEV64,
    BCM_SYSPORT_STAT_MIB_RX,
    BCM_SYSPORT_STAT_MIB_TX,
    BCM_SYSPORT_STAT_RUNT,
    BCM_SYSPORT_STAT_RXCHK,
    BCM_SYSPORT_STAT_RBUF,
    BCM_SYSPORT_STAT_RDMA,
    BCM_SYSPORT_STAT_SOFT,
}

// Macros to help define ethtool statistics

// TX bytes and packets
pub const NUM_SYSPORT_TXQ_STAT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_stats {
    pub stat_string: [c_char; ETH_GSTRING_LEN],
    pub stat_sizeof: c_int,
    pub stat_offset: c_int,
    pub type: bcm_sysport_stat_type,
// reg offset from UMAC base for misc counters
    pub reg_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_stats64 {
// 64bit stats on 32bit/64bit Machine
    pub rx_packets: u64,
    pub rx_bytes: u64,
    pub tx_packets: u64,
    pub tx_bytes: u64,
}

// Software house keeping helper structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_cb {
    pub /: *mut *mut *mut sk_buff skb; / SKB for RX packets,
    pub /: *mut *mut *mut void __iomem bd_addr; / Buffer descriptor PHYS addr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bcm_sysport_type {
    SYSTEMPORT = 0,
    SYSTEMPORT_LITE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_hw_params {
    pub is_lite: bool,
    pub num_rx_desc_words: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_net_dim {
    pub use_dim: u16,
    pub event_ctr: u16,
    pub packets: c_ulong,
    pub bytes: c_ulong,
    pub dim: dim,
}

// Software view of the TX ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_tx_ring {
    pub /: *mut *mut spinlock_t lock; / Ring lock for tx reclaim/xmit,
    pub /: *mut *mut napi_napi; / NAPI per tx queue,
    pub /: *mut *mut unsigned int index; / Ring index,
    pub /: *mut *mut unsigned int size; / Ring current size,
    pub /: *mut *mut unsigned int alloc_size; / Ring one-time allocated size,
    pub /: *mut *mut unsigned int desc_count; / Number of descriptors,
    pub /: *mut *mut unsigned int curr_desc; / Current descriptor,
    pub /: *mut *mut unsigned int c_index; / Last consumer index,
    pub /: *mut *mut unsigned int clean_index; / Current clean index,
    pub /: *mut *mut *mut bcm_sysport_cb cbs; / Transmit control blocks,
    pub /: *mut *mut *mut bcm_sysport_priv priv; / private context backpointer,
    pub /: *mut *mut unsigned long packets; / packets statistics,
    pub /: *mut *mut unsigned long bytes; / bytes statistics,
    pub /: *mut *mut unsigned int switch_queue; / switch port queue number,
    pub /: *mut *mut unsigned int switch_port; / switch port queue number,
    pub /: *mut *mut bool inspect; / inspect switch port and queue,
}

// Driver private structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcm_sysport_priv {
    pub base: *mut void __iomem,
    pub irq0_stat: u32,
    pub irq0_mask: u32,
    pub irq1_stat: u32,
    pub irq1_mask: u32,
    pub is_lite: bool,
    pub num_rx_desc_words: c_uint,
    pub ____cacheline_aligned: napi_napi,
    pub netdev: *mut net_device,
    pub pdev: *mut platform_device,
    pub irq0: c_int,
    pub irq1: c_int,
    pub wol_irq: c_int,
// Transmit rings
    pub desc_lock: spinlock_t,
    pub tx_rings: *mut bcm_sysport_tx_ring,
// Receive queue
    pub rx_bds: *mut void __iomem,
    pub rx_cbs: *mut bcm_sysport_cb,
    pub num_rx_bds: c_uint,
    pub rx_read_ptr: c_uint,
    pub rx_c_index: c_uint,
    pub dim: bcm_sysport_net_dim,
    pub rx_max_coalesced_frames: u32,
    pub rx_coalesce_usecs: u32,
// PHY device
    pub phy_dn: *mut device_node,
    pub phy_interface: phy_interface_t,
    pub old_pause: c_int,
    pub old_link: c_int,
    pub old_duplex: c_int,
// Misc fields
    pub rx_chk_en:1: c_uint,
    pub tsb_en:1: c_uint,
    pub crc_fwd:1: c_uint,
    pub rev: u16,
    pub wolopts: u32,
    pub sopass: [u8; SOPASS_MAX],
    pub wol_irq_disabled:1: c_uint,
    pub clk: *mut clk,
    pub wol_clk: *mut clk,
// MIB related fields
    pub mib: bcm_sysport_mib,
// Ethtool
    pub msg_enable: u32,
    pub RXCHK_BRCM_TAG_MAX): DECLARE_BITMAP(filters,,
    pub filters_loc: [u32; RXCHK_BRCM_TAG_MAX],
    pub stats64: bcm_sysport_stats64,
// For atomic update generic 64bit value on 32bit Machine
    pub syncp: u64_stats_sync,
// map information between switch port queues and local queues
    pub netdev_notifier: notifier_block,
    pub per_port_num_tx_queues: c_uint,
    pub 8]: *mut *mut *mut bcm_sysport_tx_ring ring_map[DSA_MAX_PORTS,
}

// I/O accessors register helpers

