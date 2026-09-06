//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qualcomm/emac/emac.h
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
// Copyright (c) 2013-2016, The Linux Foundation. All rights reserved.
//

// EMAC base register offsets
pub const EMAC_DMA_MAS_CTRL: c_uint = 0x1400;
pub const EMAC_IRQ_MOD_TIM_INIT: c_uint = 0x1408;
pub const EMAC_BLK_IDLE_STS: c_uint = 0x140c;
pub const EMAC_PHY_LINK_DELAY: c_uint = 0x141c;
pub const EMAC_SYS_ALIV_CTRL: c_uint = 0x1434;
pub const EMAC_MAC_CTRL: c_uint = 0x1480;
pub const EMAC_MAC_IPGIFG_CTRL: c_uint = 0x1484;
pub const EMAC_MAC_STA_ADDR0: c_uint = 0x1488;
pub const EMAC_MAC_STA_ADDR1: c_uint = 0x148c;
pub const EMAC_HASH_TAB_REG0: c_uint = 0x1490;
pub const EMAC_HASH_TAB_REG1: c_uint = 0x1494;
pub const EMAC_MAC_HALF_DPLX_CTRL: c_uint = 0x1498;
pub const EMAC_MAX_FRAM_LEN_CTRL: c_uint = 0x149c;
pub const EMAC_WOL_CTRL0: c_uint = 0x14a0;
pub const EMAC_RSS_KEY0: c_uint = 0x14b0;
pub const EMAC_H1TPD_BASE_ADDR_LO: c_uint = 0x14e0;
pub const EMAC_H2TPD_BASE_ADDR_LO: c_uint = 0x14e4;
pub const EMAC_H3TPD_BASE_ADDR_LO: c_uint = 0x14e8;
pub const EMAC_INTER_SRAM_PART9: c_uint = 0x1534;
pub const EMAC_DESC_CTRL_0: c_uint = 0x1540;
pub const EMAC_DESC_CTRL_1: c_uint = 0x1544;
pub const EMAC_DESC_CTRL_2: c_uint = 0x1550;
pub const EMAC_DESC_CTRL_10: c_uint = 0x1554;
pub const EMAC_DESC_CTRL_12: c_uint = 0x1558;
pub const EMAC_DESC_CTRL_13: c_uint = 0x155c;
pub const EMAC_DESC_CTRL_3: c_uint = 0x1560;
pub const EMAC_DESC_CTRL_4: c_uint = 0x1564;
pub const EMAC_DESC_CTRL_5: c_uint = 0x1568;
pub const EMAC_DESC_CTRL_14: c_uint = 0x156c;
pub const EMAC_DESC_CTRL_15: c_uint = 0x1570;
pub const EMAC_DESC_CTRL_16: c_uint = 0x1574;
pub const EMAC_DESC_CTRL_6: c_uint = 0x1578;
pub const EMAC_DESC_CTRL_8: c_uint = 0x1580;
pub const EMAC_DESC_CTRL_9: c_uint = 0x1584;
pub const EMAC_DESC_CTRL_11: c_uint = 0x1588;
pub const EMAC_TXQ_CTRL_0: c_uint = 0x1590;
pub const EMAC_TXQ_CTRL_1: c_uint = 0x1594;
pub const EMAC_TXQ_CTRL_2: c_uint = 0x1598;
pub const EMAC_RXQ_CTRL_0: c_uint = 0x15a0;
pub const EMAC_RXQ_CTRL_1: c_uint = 0x15a4;
pub const EMAC_RXQ_CTRL_2: c_uint = 0x15a8;
pub const EMAC_RXQ_CTRL_3: c_uint = 0x15ac;
pub const EMAC_BASE_CPU_NUMBER: c_uint = 0x15b8;
pub const EMAC_DMA_CTRL: c_uint = 0x15c0;
pub const EMAC_MAILBOX_0: c_uint = 0x15e0;
pub const EMAC_MAILBOX_5: c_uint = 0x15e4;
pub const EMAC_MAILBOX_6: c_uint = 0x15e8;
pub const EMAC_MAILBOX_13: c_uint = 0x15ec;
pub const EMAC_MAILBOX_2: c_uint = 0x15f4;
pub const EMAC_MAILBOX_3: c_uint = 0x15f8;
pub const EMAC_INT_STATUS: c_uint = 0x1600;
pub const EMAC_INT_MASK: c_uint = 0x1604;
pub const EMAC_MAILBOX_11: c_uint = 0x160c;
pub const EMAC_AXI_MAST_CTRL: c_uint = 0x1610;
pub const EMAC_MAILBOX_12: c_uint = 0x1614;
pub const EMAC_MAILBOX_9: c_uint = 0x1618;
pub const EMAC_MAILBOX_10: c_uint = 0x161c;
pub const EMAC_ATHR_HEADER_CTRL: c_uint = 0x1620;
pub const EMAC_RXMAC_STATC_REG0: c_uint = 0x1700;
pub const EMAC_RXMAC_STATC_REG22: c_uint = 0x1758;
pub const EMAC_TXMAC_STATC_REG0: c_uint = 0x1760;
pub const EMAC_TXMAC_STATC_REG24: c_uint = 0x17c0;
pub const EMAC_CLK_GATE_CTRL: c_uint = 0x1814;
pub const EMAC_CORE_HW_VERSION: c_uint = 0x1974;
pub const EMAC_MISC_CTRL: c_uint = 0x1990;
pub const EMAC_MAILBOX_7: c_uint = 0x19e0;
pub const EMAC_MAILBOX_8: c_uint = 0x19e4;
pub const EMAC_IDT_TABLE0: c_uint = 0x1b00;
pub const EMAC_RXMAC_STATC_REG23: c_uint = 0x1bc8;
pub const EMAC_RXMAC_STATC_REG24: c_uint = 0x1bcc;
pub const EMAC_TXMAC_STATC_REG25: c_uint = 0x1bd0;
pub const EMAC_MAILBOX_15: c_uint = 0x1bd4;
pub const EMAC_MAILBOX_16: c_uint = 0x1bd8;
pub const EMAC_INT1_MASK: c_uint = 0x1bf0;
pub const EMAC_INT1_STATUS: c_uint = 0x1bf4;
pub const EMAC_INT2_MASK: c_uint = 0x1bf8;
pub const EMAC_INT2_STATUS: c_uint = 0x1bfc;
pub const EMAC_INT3_MASK: c_uint = 0x1c00;
pub const EMAC_INT3_STATUS: c_uint = 0x1c04;
// EMAC_DMA_MAS_CTRL
pub const DEV_ID_NUM_BMSK: c_uint = 0x7f000000;
pub const DEV_ID_NUM_SHFT: c_int = 24;
pub const DEV_REV_NUM_BMSK: c_uint = 0xff0000;
pub const DEV_REV_NUM_SHFT: c_int = 16;
pub const INT_RD_CLR_EN: c_uint = 0x4000;
pub const IRQ_MODERATOR2_EN: c_uint = 0x800;
pub const IRQ_MODERATOR_EN: c_uint = 0x400;
pub const LPW_CLK_SEL: c_uint = 0x80;
pub const LPW_STATE: c_uint = 0x20;
pub const LPW_MODE: c_uint = 0x10;
pub const SOFT_RST: c_uint = 0x1;
// EMAC_IRQ_MOD_TIM_INIT
pub const IRQ_MODERATOR2_INIT_BMSK: c_uint = 0xffff0000;
pub const IRQ_MODERATOR2_INIT_SHFT: c_int = 16;
pub const IRQ_MODERATOR_INIT_BMSK: c_uint = 0xffff;
pub const IRQ_MODERATOR_INIT_SHFT: c_int = 0;
// EMAC_INT_STATUS

// EMAC_MAILBOX_6
pub const RFD2_PROC_IDX_BMSK: c_uint = 0xfff0000;
pub const RFD2_PROC_IDX_SHFT: c_int = 16;
pub const RFD2_PROD_IDX_BMSK: c_uint = 0xfff;
pub const RFD2_PROD_IDX_SHFT: c_int = 0;
// EMAC_CORE_HW_VERSION
pub const MAJOR_BMSK: c_uint = 0xf0000000;
pub const MAJOR_SHFT: c_int = 28;
pub const MINOR_BMSK: c_uint = 0xfff0000;
pub const MINOR_SHFT: c_int = 16;
pub const STEP_BMSK: c_uint = 0xffff;
pub const STEP_SHFT: c_int = 0;
// EMAC_EMAC_WRAPPER_CSR1

// EMAC_EMAC_WRAPPER_CSR2
pub const HDRIVE_BMSK: c_uint = 0x3000;
pub const HDRIVE_SHFT: c_int = 12;

pub const EMAC_DEV_ID: c_uint = 0x0040;
// SGMII v2 per lane registers
pub const SGMII_LN_RSM_START: c_uint = 0x029C;
// SGMII v2 PHY common registers
pub const SGMII_PHY_CMN_CTRL: c_uint = 0x0408;
pub const SGMII_PHY_CMN_RESET_CTRL: c_uint = 0x0410;
// SGMII v2 PHY registers per lane
pub const SGMII_PHY_LN_OFFSET: c_uint = 0x0400;
pub const SGMII_PHY_LN_LANE_STATUS: c_uint = 0x00DC;
pub const SGMII_PHY_LN_BIST_GEN0: c_uint = 0x008C;
pub const SGMII_PHY_LN_BIST_GEN1: c_uint = 0x0090;
pub const SGMII_PHY_LN_BIST_GEN2: c_uint = 0x0094;
pub const SGMII_PHY_LN_BIST_GEN3: c_uint = 0x0098;
pub const SGMII_PHY_LN_CDR_CTRL1: c_uint = 0x005C;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum emac_clk_id {
    EMAC_CLK_AXI,
    EMAC_CLK_CFG_AHB,
    EMAC_CLK_HIGH_SPEED,
    EMAC_CLK_MDIO,
    EMAC_CLK_TX,
    EMAC_CLK_RX,
    EMAC_CLK_SYS,
    EMAC_CLK_CNT
}

pub const EMAC_LINK_SPEED_UNKNOWN: c_uint = 0x0;

pub const EMAC_MAX_SETUP_LNK_CYCLE: c_int = 100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_stats {
// rx
    pub /: *mut *mut u64 rx_ok; / good packets,
    pub /: *mut *mut u64 rx_bcast; / good broadcast packets,
    pub /: *mut *mut u64 rx_mcast; / good multicast packets,
    pub /: *mut *mut u64 rx_pause; / pause packet,
    pub /: *mut *mut u64 rx_ctrl; / control packets other than pause frame.,
    pub /: *mut *mut u64 rx_fcs_err; / packets with bad FCS.,
    pub /: *mut *mut u64 rx_len_err; / packets with length mismatch,
    pub /: *mut *mut u64 rx_byte_cnt; / good bytes count (without FCS),
    pub /: *mut *mut u64 rx_runt; / runt packets,
    pub /: *mut *mut u64 rx_frag; / fragment count,
    pub /: *mut *mut u64 rx_sz_64; / packets that are 64 bytes,
    pub /: *mut *mut u64 rx_sz_65_127; / packets that are 65-127 bytes,
    pub /: *mut *mut u64 rx_sz_128_255; / packets that are 128-255 bytes,
    pub /: *mut *mut u64 rx_sz_256_511; / packets that are 256-511 bytes,
    pub /: *mut *mut u64 rx_sz_512_1023; / packets that are 512-1023 bytes,
    pub /: *mut *mut u64 rx_sz_1024_1518; / packets that are 1024-1518 bytes,
    pub bytes*/: *mut *mut u64 rx_sz_1519_max; / packets that are 1519-MTU,
    pub /: *mut *mut u64 rx_sz_ov; / packets that are >MTU bytes (truncated),
    pub /: *mut *mut u64 rx_rxf_ov; / packets dropped due to RX FIFO overflow,
    pub /: *mut *mut u64 rx_align_err; / alignment errors,
    pub /: *mut *mut u64 rx_bcast_byte_cnt; / broadcast packets byte count (without FCS),
    pub /: *mut *mut u64 rx_mcast_byte_cnt; / multicast packets byte count (without FCS),
    pub /: *mut *mut u64 rx_err_addr; / packets dropped due to address filtering,
    pub /: *mut *mut u64 rx_crc_align; / CRC align errors,
    pub /: *mut *mut u64 rx_jabbers; / jabbers,
// tx
    pub /: *mut *mut u64 tx_ok; / good packets,
    pub /: *mut *mut u64 tx_bcast; / good broadcast packets,
    pub /: *mut *mut u64 tx_mcast; / good multicast packets,
    pub /: *mut *mut u64 tx_pause; / pause packets,
    pub /: *mut *mut u64 tx_exc_defer; / packets with excessive deferral,
    pub /: *mut *mut u64 tx_ctrl; / control packets other than pause frame,
    pub /: *mut *mut u64 tx_defer; / packets that are deferred.,
    pub /: *mut *mut u64 tx_byte_cnt; / good bytes count (without FCS),
    pub /: *mut *mut u64 tx_sz_64; / packets that are 64 bytes,
    pub /: *mut *mut u64 tx_sz_65_127; / packets that are 65-127 bytes,
    pub /: *mut *mut u64 tx_sz_128_255; / packets that are 128-255 bytes,
    pub /: *mut *mut u64 tx_sz_256_511; / packets that are 256-511 bytes,
    pub /: *mut *mut u64 tx_sz_512_1023; / packets that are 512-1023 bytes,
    pub /: *mut *mut u64 tx_sz_1024_1518; / packets that are 1024-1518 bytes,
    pub /: *mut *mut u64 tx_sz_1519_max; / packets that are 1519-MTU bytes,
    pub /: *mut *mut u64 tx_1_col; / packets single prior collision,
    pub /: *mut *mut u64 tx_2_col; / packets with multiple prior collisions,
    pub /: *mut *mut u64 tx_late_col; / packets with late collisions,
    pub /: *mut *mut u64 tx_abort_col; / packets aborted due to excess collisions,
    pub /: *mut *mut u64 tx_underrun; / packets aborted due to FIFO underrun,
    pub /: *mut *mut u64 tx_rd_eop; / count of reads beyond EOP,
    pub /: *mut *mut u64 tx_len_err; / packets with length mismatch,
    pub /: *mut *mut u64 tx_trunc; / packets truncated due to size >MTU,
    pub /: *mut *mut u64 tx_bcast_byte; / broadcast packets byte count (without FCS),
    pub /: *mut *mut u64 tx_mcast_byte; / multicast packets byte count (without FCS),
    pub /: *mut *mut u64 tx_col; / collisions,
    pub /: *mut *mut spinlock_t lock; / prevent multiple simultaneous readers,
}

// RSS hstype Definitions
pub const EMAC_RSS_HSTYP_IPV4_EN: c_uint = 0x00000001;
pub const EMAC_RSS_HSTYP_TCP4_EN: c_uint = 0x00000002;
pub const EMAC_RSS_HSTYP_IPV6_EN: c_uint = 0x00000004;
pub const EMAC_RSS_HSTYP_TCP6_EN: c_uint = 0x00000008;

pub const EMAC_DEF_RX_BUF_SIZE: c_int = 1536;

pub const EMAC_MIN_ETH_FRAME_SIZE: c_int = 68;
pub const EMAC_DEF_TX_QUEUES: c_int = 1;
pub const EMAC_DEF_RX_QUEUES: c_int = 1;
pub const EMAC_MIN_TX_DESCS: c_int = 128;
pub const EMAC_MIN_RX_DESCS: c_int = 128;
pub const EMAC_MAX_TX_DESCS: c_int = 16383;
pub const EMAC_MAX_RX_DESCS: c_int = 2047;
pub const EMAC_DEF_TX_DESCS: c_int = 512;
pub const EMAC_DEF_RX_DESCS: c_int = 256;
pub const EMAC_DEF_RX_IRQ_MOD: c_int = 250;
pub const EMAC_DEF_TX_IRQ_MOD: c_int = 250;

// by default check link every 4 seconds

// emac_irq per-device (per-adapter) irq properties.
// @irq:	irq number.
// @mask	mask to use over status register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_irq {
    pub irq: c_uint,
    pub mask: u32,
}

// The device's main data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_adapter {
    pub netdev: *mut net_device,
    pub mii_bus: *mut mii_bus,
    pub phydev: *mut phy_device,
    pub base: *mut void __iomem,
    pub csr: *mut void __iomem,
    pub phy: emac_sgmii,
    pub stats: emac_stats,
    pub irq: emac_irq,
    pub clk: [*mut clk; EMAC_CLK_CNT],
// All Descriptor memory
    pub ring_header: emac_ring_header,
    pub tx_q: emac_tx_queue,
    pub rx_q: emac_rx_queue,
    pub tx_desc_cnt: c_uint,
    pub rx_desc_cnt: c_uint,
    pub /: *mut *mut unsigned int rrd_size; / in quad words,
    pub /: *mut *mut unsigned int rfd_size; / in quad words,
    pub /: *mut *mut unsigned int tpd_size; / in quad words,
    pub rxbuf_size: c_uint,
// Flow control / pause frames support. If automatic=True, do whatever
// the PHY does. Otherwise, use tx_flow_control and rx_flow_control.
//
    pub automatic: bool,
    pub tx_flow_control: bool,
    pub rx_flow_control: bool,
// True == use single-pause-frame mode.
    pub single_pause_mode: bool,
// Ring parameter
    pub tpd_burst: u8,
    pub rfd_burst: u8,
    pub dmaw_dly_cnt: c_uint,
    pub dmar_dly_cnt: c_uint,
    pub dmar_block: emac_dma_req_block,
    pub dmaw_block: emac_dma_req_block,
    pub dma_order: emac_dma_order,
    pub irq_mod: u32,
    pub preamble: u32,
    pub work_thread: work_struct,
    pub msg_enable: u16,
    pub reset_lock: mutex,
}

extern "C" {
    pub fn emac_reinit_locked(adpt: *mut emac_adapter) -> c_int;
}
extern "C" {
    pub fn emac_reg_update32(addr: *mut void __iomem, mask: u32, val: u32);
}
extern "C" {
    pub fn emac_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn emac_update_hw_stats(adpt: *mut emac_adapter);
}
