//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mediatek/mtk_eth_soc.h
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
// Copyright (C) 2009-2016 John Crispin <blogic@openwrt.org>
// Copyright (C) 2009-2016 Felix Fietkau <nbd@openwrt.org>
// Copyright (C) 2013-2016 Michael Lee <igvtee@gmail.com>
//

pub const MTK_MAX_DSA_PORTS: c_int = 7;

pub const MTK_QDMA_NUM_QUEUES: c_int = 16;
pub const MTK_QDMA_PAGE_SIZE: c_int = 2048;
pub const MTK_MAX_RX_LENGTH: c_int = 1536;
pub const MTK_MAX_RX_LENGTH_2K: c_int = 2048;
pub const MTK_TX_DMA_BUF_LEN: c_uint = 0x3fff;
pub const MTK_TX_DMA_BUF_LEN_V2: c_uint = 0xffff;
pub const MTK_QDMA_RING_SIZE: c_int = 2048;

pub const MTK_FQ_DMA_HEAD: c_int = 32;
pub const MTK_FQ_DMA_LENGTH: c_int = 2048;

pub const MTK_DMA_DUMMY_DESC: c_uint = 0xffffffff;

pub const MTK_QRX_OFFSET: c_uint = 0x10;
pub const MTK_MAX_RX_RING_NUM: c_int = 4;
pub const MTK_HW_LRO_DMA_SIZE: c_int = 8;

pub const MTK_MAX_LRO_IP_CNT: c_int = 2;

pub const MTK_HW_LRO_MAX_AGG_CNT: c_int = 64;
pub const MTK_HW_LRO_BW_THRE: c_int = 3000;
pub const MTK_HW_LRO_REPLACE_DELTA: c_int = 1000;
pub const MTK_HW_LRO_SDL_REMAIN_ROOM: c_int = 1522;
// Frame Engine Global Configuration

// Frame Engine Global Reset Register
pub const MTK_RST_GL: c_uint = 0x04;

// Frame Engine Interrupt Status Register
pub const MTK_INT_STATUS2: c_uint = 0x08;
pub const MTK_FE_INT_ENABLE: c_uint = 0x0c;

// PDMA HW LRO Alter Flow Timer Register
pub const MTK_PDMA_LRO_ALT_REFRESH_TIMER: c_uint = 0x1c;
// Frame Engine Interrupt Grouping Register
pub const MTK_FE_INT_GRP: c_uint = 0x20;
// CDMP Ingress Control Register
pub const MTK_CDMQ_IG_CTRL: c_uint = 0x1400;

// CDMQ Exgress Control Register
pub const MTK_CDMQ_EG_CTRL: c_uint = 0x1404;
// CDMP Ingress Control Register
pub const MTK_CDMP_IG_CTRL: c_uint = 0x400;

// CDMP Exgress Control Register
pub const MTK_CDMP_EG_CTRL: c_uint = 0x404;
// GDM Exgress Control Register

pub const MTK_GDMA_TO_PDMA: c_uint = 0x0;
pub const MTK_GDMA_DROP_ALL: c_uint = 0x7777;
// GDM Egress Control Register

// Unicast Filter MAC Address Register - Low

// Unicast Filter MAC Address Register - High

// legacy DT support for internal SRAM
pub const MTK_ETH_SRAM_OFFSET: c_uint = 0x40000;
pub const MTK_ETH_SRAM_GRANULARITY: c_int = 32;
pub const MTK_ETH_NETSYS_V2_SRAM_SIZE: c_uint = 0x40000;
// FE global misc reg
pub const MTK_FE_GLO_MISC: c_uint = 0x124;
// PSE Free Queue Flow Control
pub const PSE_FQFC_CFG1: c_uint = 0x100;
pub const PSE_FQFC_CFG2: c_uint = 0x104;
pub const PSE_DROP_CFG: c_uint = 0x108;

// PSE Last FreeQ Page Request Control
pub const PSE_DUMY_REQ: c_uint = 0x10C;
// PSE_DUMY_REQ is not a typo but actually called like that also in
// MediaTek's datasheet
//

pub const DUMMY_PAGE_THR: c_uint = 0x1;
// PSE Input Queue Reservation Register

// PSE Output Queue Threshold Register

// GDM and CDM Threshold
pub const MTK_GDM2_THRES: c_uint = 0x1530;
pub const MTK_CDMW0_THRES: c_uint = 0x164c;
pub const MTK_CDMW1_THRES: c_uint = 0x1650;
pub const MTK_CDME0_THRES: c_uint = 0x1654;
pub const MTK_CDME1_THRES: c_uint = 0x1658;
pub const MTK_CDMM_THRES: c_uint = 0x165c;
// PDMA HW LRO Control Registers
pub const MTK_PDMA_LRO_CTRL_DW0: c_uint = 0x980;

pub const MTK_PDMA_LRO_CTRL_DW1: c_uint = 0x984;
pub const MTK_PDMA_LRO_CTRL_DW2: c_uint = 0x988;
pub const MTK_PDMA_LRO_CTRL_DW3: c_uint = 0x98c;

// PDMA Global Configuration Register
pub const MTK_PDMA_LRO_SDL: c_uint = 0x3000;
pub const MTK_RX_CFG_SDL_OFFSET: c_int = 16;
// PDMA Reset Index Register

// PDMA Delay Interrupt Register

pub const MTK_PDMA_DELAY_RX_PINT_SHIFT: c_int = 8;
pub const MTK_PDMA_DELAY_RX_PTIME_SHIFT: c_int = 0;

pub const MTK_PDMA_DELAY_TX_PINT_SHIFT: c_int = 24;
pub const MTK_PDMA_DELAY_TX_PTIME_SHIFT: c_int = 16;
pub const MTK_PDMA_DELAY_PINT_MASK: c_uint = 0x7f;
pub const MTK_PDMA_DELAY_PTIME_MASK: c_uint = 0xff;
// PDMA HW LRO Alter Flow Delta Register
pub const MTK_PDMA_LRO_ALT_SCORE_DELTA: c_uint = 0xa4c;
// PDMA HW LRO IP Setting Registers
pub const MTK_LRO_RX_RING0_DIP_DW0: c_uint = 0xb04;

// PDMA HW LRO Ring Control Registers
pub const MTK_LRO_RX_RING0_CTRL_DW1: c_uint = 0xb28;
pub const MTK_LRO_RX_RING0_CTRL_DW2: c_uint = 0xb2c;
pub const MTK_LRO_RX_RING0_CTRL_DW3: c_uint = 0xb30;

// QDMA TX Queue Configuration Registers
pub const MTK_QTX_OFFSET: c_uint = 0x10;
pub const QDMA_RES_THRES: c_int = 4;
// QDMA Tx Queue Scheduler Configuration Registers

// QDMA TX Scheduler Rate Control Register

// QDMA Global Configuration Register

pub const MTK_DMA_BUSY_TIMEOUT_US: c_int = 1000000;
// QDMA V2 Global Configuration Register

// QDMA Flow Control Register

pub const FC_THRES_MIN: c_uint = 0x4444;
// QDMA Interrupt Status Register

// QDMA Interrupt grouping registers

// QDMA TX NUM

pub const MTK_QDMA_GMAC2_QID: c_int = 8;
pub const MTK_TX_DMA_BUF_SHIFT: c_int = 8;
// QDMA V2 descriptor txd6

// QDMA V2 descriptor txd5

// QDMA V2 descriptor txd4
pub const TX_DMA_FPORT_SHIFT_V2: c_int = 8;
pub const TX_DMA_FPORT_MASK_V2: c_uint = 0xf;

// QDMA descriptor txd4

pub const TX_DMA_FPORT_SHIFT: c_int = 25;
pub const TX_DMA_FPORT_MASK: c_uint = 0x7;

// QDMA descriptor txd3

// PDMA on MT7628

// QDMA descriptor rxd2

// QDMA descriptor rxd3

// QDMA descriptor rxd4

// QDMA descriptor rxd4

// PDMA descriptor rxd5

// PDMA V2 descriptor rxd3

// PHY Polling and SMI Master Control registers
pub const MTK_PPSC: c_uint = 0x10000;

pub const MDC_MAX_FREQ: c_int = 25000000;
pub const MDC_MAX_DIVIDER: c_int = 63;
// PHY Indirect Access Control registers
pub const MTK_PHY_IAC: c_uint = 0x10004;

pub const MTK_MAC_MISC: c_uint = 0x1000c;
pub const MTK_MAC_MISC_V3: c_uint = 0x10010;

// XMAC status registers

// GSW bridge registers

pub const GSWTX_IPG_SHIFT: c_int = 16;

pub const GSWRX_IPG_SHIFT: c_int = 0;
pub const GSW_IPG_11: c_int = 11;
// Mac control registers

pub const MAC_MCR_MAX_RX_1518: c_uint = 0x0;
pub const MAC_MCR_MAX_RX_1536: c_uint = 0x1;
pub const MAC_MCR_MAX_RX_1552: c_uint = 0x2;
pub const MAC_MCR_MAX_RX_2048: c_uint = 0x3;

// Mac EEE control registers

// Mac status registers

// TRGMII RXC control register
pub const TRGMII_RCK_CTRL: c_uint = 0x10300;

pub const NUM_TRGMII_CTRL: c_int = 5;
// TRGMII RXC control register
pub const TRGMII_TCK_CTRL: c_uint = 0x10340;

// TRGMII TX Drive Strength

// TRGMII Interface mode register
pub const INTF_MODE: c_uint = 0x10390;

pub const INTF_MODE_RGMII_10_100: c_int = 0;
// XFI Mac control registers

pub const XMAC_MCR_TRX_DISABLE: c_uint = 0xf;

// XFI Mac logic reset registers

// XFI Mac count global control

// GPIO port control registers for GMAC 2
pub const GPIO_OD33_CTRL8: c_uint = 0x4c0;
pub const GPIO_BIAS_CTRL: c_uint = 0xed0;
pub const GPIO_DRV_SEL10: c_uint = 0xf00;
// ethernet subsystem chip id register
pub const ETHSYS_CHIPID0_3: c_uint = 0x0;
pub const ETHSYS_CHIPID4_7: c_uint = 0x4;
pub const MT7623_ETH: c_int = 7623;
pub const MT7622_ETH: c_int = 7622;
pub const MT7621_ETH: c_int = 7621;
// ethernet system control register
pub const ETHSYS_SYSCFG: c_uint = 0x10;

// ethernet subsystem config register
pub const ETHSYS_SYSCFG0: c_uint = 0x14;
pub const SYSCFG0_GE_MASK: c_uint = 0x3;

// ethernet subsystem clock register
pub const ETHSYS_CLKCFG0: c_uint = 0x2c;

// ethernet reset control register
pub const ETHSYS_RSTCTRL: c_uint = 0x34;

// ethernet reset check idle register
pub const ETHSYS_FE_RST_CHK_IDLE_EN: c_uint = 0x28;
// ethernet dma channel agent map
pub const ETHSYS_DMA_AG_MAP: c_uint = 0x408;

// Infrasys subsystem config registers
pub const INFRA_MISC2: c_uint = 0x70c;

// Top misc registers
pub const TOP_MISC_NETSYS_PCS_MUX: c_uint = 0x0;

pub const USB_PHY_SWITCH_REG: c_uint = 0x218;

pub const SGMII_QPHY_SEL: c_uint = 0x2;
// MT7628/88 specific stuff
pub const MT7628_PDMA_OFFSET: c_uint = 0x0800;
pub const MT7628_SDM_OFFSET: c_uint = 0x0c00;

// Counter / stat register

pub const MTK_FE_CDM1_FSM: c_uint = 0x220;
pub const MTK_FE_CDM2_FSM: c_uint = 0x224;
pub const MTK_FE_CDM3_FSM: c_uint = 0x238;
pub const MTK_FE_CDM4_FSM: c_uint = 0x298;
pub const MTK_FE_CDM5_FSM: c_uint = 0x318;
pub const MTK_FE_CDM6_FSM: c_uint = 0x328;
pub const MTK_FE_GDM1_FSM: c_uint = 0x228;
pub const MTK_FE_GDM2_FSM: c_uint = 0x22C;

pub const MTK_FE_IRQ_SHARED: c_int = 0;
pub const MTK_FE_IRQ_TX: c_int = 0;
pub const MTK_FE_IRQ_RX: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_rx_dma {
    pub rxd1: c_uint,
    pub rxd2: c_uint,
    pub rxd3: c_uint,
    pub rxd4: c_uint,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_rx_dma_v2 {
    pub rxd1: c_uint,
    pub rxd2: c_uint,
    pub rxd3: c_uint,
    pub rxd4: c_uint,
    pub rxd5: c_uint,
    pub rxd6: c_uint,
    pub rxd7: c_uint,
    pub rxd8: c_uint,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_tx_dma {
    pub txd1: c_uint,
    pub txd2: c_uint,
    pub txd3: c_uint,
    pub txd4: c_uint,
    pub __aligned(4): } __packed,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_tx_dma_v2 {
    pub txd1: c_uint,
    pub txd2: c_uint,
    pub txd3: c_uint,
    pub txd4: c_uint,
    pub txd5: c_uint,
    pub txd6: c_uint,
    pub txd7: c_uint,
    pub txd8: c_uint,
    pub __aligned(4): } __packed,
    pub mtk_eth: struct,
    pub mtk_mac: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_xdp_stats {
    pub rx_xdp_redirect: u64,
    pub rx_xdp_pass: u64,
    pub rx_xdp_drop: u64,
    pub rx_xdp_tx: u64,
    pub rx_xdp_tx_errors: u64,
    pub tx_xdp_xmit: u64,
    pub tx_xdp_xmit_errors: u64,
}

// struct mtk_hw_stats - the structure that holds the traffic statistics.
// @stats_lock:		make sure that stats operations are atomic
// @reg_offset:		the status register offset of the SoC
// @syncp:		the refcount
//
// All of the supported SoCs have hardware counters for traffic statistics.
// Whenever the status IRQ triggers we can read the latest stats from these
// counters and store them in this struct.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_hw_stats {
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_skip: u64,
    pub tx_collisions: u64,
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_overflow: u64,
    pub rx_fcs_errors: u64,
    pub rx_short_errors: u64,
    pub rx_long_errors: u64,
    pub rx_checksum_errors: u64,
    pub rx_flow_control_packets: u64,
    pub xdp_stats: mtk_xdp_stats,
    pub stats_lock: spinlock_t,
    pub reg_offset: u32,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_tx_flags {
// PDMA descriptor can point at 1-2 segments. This enum allows us to
// track how memory was allocated so that it can be freed properly.
//
    MTK_TX_FLAGS_SINGLE0	= 0x01,
    MTK_TX_FLAGS_PAGE0	= 0x02,
}

// This enum allows us to identify how the clock is defined on the array of the
// clock in the order
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_clks_map {
    MTK_CLK_ETHIF,
    MTK_CLK_SGMIITOP,
    MTK_CLK_ESW,
    MTK_CLK_GP0,
    MTK_CLK_GP1,
    MTK_CLK_GP2,
    MTK_CLK_GP3,
    MTK_CLK_XGP1,
    MTK_CLK_XGP2,
    MTK_CLK_XGP3,
    MTK_CLK_CRYPTO,
    MTK_CLK_FE,
    MTK_CLK_TRGPLL,
    MTK_CLK_SGMII_TX_250M,
    MTK_CLK_SGMII_RX_250M,
    MTK_CLK_SGMII_CDR_REF,
    MTK_CLK_SGMII_CDR_FB,
    MTK_CLK_SGMII2_TX_250M,
    MTK_CLK_SGMII2_RX_250M,
    MTK_CLK_SGMII2_CDR_REF,
    MTK_CLK_SGMII2_CDR_FB,
    MTK_CLK_SGMII_CK,
    MTK_CLK_ETH2PLL,
    MTK_CLK_WOCPU0,
    MTK_CLK_WOCPU1,
    MTK_CLK_NETSYS0,
    MTK_CLK_NETSYS1,
    MTK_CLK_ETHWARP_WOCPU2,
    MTK_CLK_ETHWARP_WOCPU1,
    MTK_CLK_ETHWARP_WOCPU0,
    MTK_CLK_TOP_SGM_0_SEL,
    MTK_CLK_TOP_SGM_1_SEL,
    MTK_CLK_TOP_ETH_GMII_SEL,
    MTK_CLK_TOP_ETH_REFCK_50M_SEL,
    MTK_CLK_TOP_ETH_SYS_200M_SEL,
    MTK_CLK_TOP_ETH_SYS_SEL,
    MTK_CLK_TOP_ETH_XGMII_SEL,
    MTK_CLK_TOP_ETH_MII_SEL,
    MTK_CLK_TOP_NETSYS_SEL,
    MTK_CLK_TOP_NETSYS_500M_SEL,
    MTK_CLK_TOP_NETSYS_PAO_2X_SEL,
    MTK_CLK_TOP_NETSYS_SYNC_250M_SEL,
    MTK_CLK_TOP_NETSYS_PPEFB_250M_SEL,
    MTK_CLK_TOP_NETSYS_WARP_SEL,
    MTK_CLK_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_dev_state {
    MTK_HW_INIT,
    MTK_RESETTING
}

// PSE Port Definition
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_pse_port {
    PSE_ADMA_PORT = 0,
    PSE_GDM1_PORT,
    PSE_GDM2_PORT,
    PSE_PPE0_PORT,
    PSE_PPE1_PORT,
    PSE_QDMA_TX_PORT,
    PSE_QDMA_RX_PORT,
    PSE_DROP_PORT,
    PSE_WDMA0_PORT,
    PSE_WDMA1_PORT,
    PSE_TDMA_PORT,
    PSE_NONE_PORT,
    PSE_PPE2_PORT,
    PSE_WDMA2_PORT,
    PSE_EIP197_PORT,
    PSE_GDM3_PORT,
    PSE_PORT_MAX
}

// GMAC Identifier
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_gmac_id {
    MTK_GMAC1_ID = 0,
    MTK_GMAC2_ID,
    MTK_GMAC3_ID,
    MTK_GMAC_ID_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_tx_buf_type {
    MTK_TYPE_SKB,
    MTK_TYPE_XDP_TX,
    MTK_TYPE_XDP_NDO,
}

// struct mtk_tx_buf -	This struct holds the pointers to the memory pointed at
// by the TX descriptor	s
// @skb:		The SKB pointer of the packet being sent
// @dma_addr0:		The base addr of the first segment
// @dma_len0:		The length of the first segment
// @dma_addr1:		The base addr of the second segment
// @dma_len1:		The length of the second segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_tx_buf {
    pub type: mtk_tx_buf_type,
    pub data: *mut c_void,
    pub mac_id: u16,
    pub flags: u16,
}

// struct mtk_tx_ring -	This struct holds info describing a TX ring
// @dma:		The descriptor ring
// @buf:		The memory pointed at by the ring
// @phys:		The physical addr of tx_buf
// @next_free:		Pointer to the next free descriptor
// @last_free:		Pointer to the last free descriptor
// @last_free_ptr:	Hardware pointer value of the last free descriptor
// @thresh:		The threshold of minimum amount of free descriptors
// @free_count:		QDMA uses a linked list. Track how many free descriptors
// are present
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_tx_ring {
    pub dma: *mut c_void,
    pub buf: *mut mtk_tx_buf,
    pub phys: dma_addr_t,
    pub next_free: *mut mtk_tx_dma,
    pub last_free: *mut mtk_tx_dma,
    pub last_free_ptr: u32,
    pub thresh: u16,
    pub free_count: core::sync::atomic::AtomicI32,
    pub dma_size: c_int,
    pub /: *mut *mut *mut mtk_tx_dma dma_pdma; / For MT7628/88 PDMA handling,
    pub phys_pdma: dma_addr_t,
    pub cpu_idx: c_int,
}

// PDMA rx ring mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_rx_flags {
    MTK_RX_FLAGS_NORMAL = 0,
    MTK_RX_FLAGS_HWLRO,
    MTK_RX_FLAGS_QDMA,
}

// struct mtk_rx_ring -	This struct holds info describing a RX ring
// @dma:		The descriptor ring
// @data:		The memory pointed at by the ring
// @phys:		The physical addr of rx_buf
// @frag_size:		How big can each fragment be
// @buf_size:		The size of each packet buffer
// @calc_idx:		The current head of ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_rx_ring {
    pub dma: *mut c_void,
    pub data: *mut u8,
    pub phys: dma_addr_t,
    pub frag_size: u16,
    pub buf_size: u16,
    pub dma_size: u16,
    pub calc_idx_update: bool,
    pub calc_idx: u16,
    pub crx_idx_reg: u32,
// page_pool
    pub page_pool: *mut page_pool,
    pub xdp_q: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mkt_eth_capabilities {
    MTK_RGMII_BIT = 0,
    MTK_TRGMII_BIT,
    MTK_SGMII_BIT,
    MTK_2P5GPHY_BIT,
    MTK_ESW_BIT,
    MTK_GEPHY_BIT,
    MTK_MUX_BIT,
    MTK_INFRA_BIT,
    MTK_SHARED_SGMII_BIT,
    MTK_HWLRO_BIT,
    MTK_SHARED_INT_BIT,
    MTK_TRGMII_MT7621_CLK_BIT,
    MTK_QDMA_BIT,
    MTK_SOC_MT7628_BIT,
    MTK_RSTCTRL_PPE1_BIT,
    MTK_RSTCTRL_PPE2_BIT,
    MTK_U3_COPHY_V2_BIT,
    MTK_SRAM_BIT,
    MTK_36BIT_DMA_BIT,

// MUX BITS
    MTK_ETH_MUX_GDM1_TO_GMAC1_ESW_BIT,
    MTK_ETH_MUX_GMAC2_GMAC0_TO_GEPHY_BIT,
    MTK_ETH_MUX_U3_GMAC2_TO_QPHY_BIT,
    MTK_ETH_MUX_GMAC2_TO_2P5GPHY_BIT,
    MTK_ETH_MUX_GMAC1_GMAC2_TO_SGMII_RGMII_BIT,
    MTK_ETH_MUX_GMAC12_TO_GEPHY_SGMII_BIT,

// PATH BITS
    MTK_ETH_PATH_GMAC1_RGMII_BIT,
    MTK_ETH_PATH_GMAC1_TRGMII_BIT,
    MTK_ETH_PATH_GMAC1_SGMII_BIT,
    MTK_ETH_PATH_GMAC2_RGMII_BIT,
    MTK_ETH_PATH_GMAC2_SGMII_BIT,
    MTK_ETH_PATH_GMAC2_2P5GPHY_BIT,
    MTK_ETH_PATH_GMAC2_GEPHY_BIT,
    MTK_ETH_PATH_GDM1_ESW_BIT,
}

// Supported hardware group on SoCs

// Supported path present on SoCs

// MUXes present on SoCs
// 0: GDM1 -> GMAC1, 1: GDM1 -> ESW

// 0: GMAC2 -> GEPHY, 1: GMAC0 -> GePHY

// 0: U3 -> QPHY, 1: GMAC2 -> QPHY

// 2: GMAC1 -> SGMII, 3: GMAC2 -> SGMII

// 2: GMAC2 -> 2P5GPHY

// 0: GMACx -> GEPHY, 1: GMACx -> SGMII where x is 1 or 2

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_tx_dma_desc_info {
    pub addr: dma_addr_t,
    pub size: u32,
    pub vlan_tci: u16,
    pub qid: u16,
    pub gso:1: u8,
    pub csum:1: u8,
    pub vlan:1: u8,
    pub first:1: u8,
    pub last:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_reg_map {
    pub tx_irq_mask: u32,
    pub tx_irq_status: u32,
    pub /: *mut *mut u32 rx_ptr; / rx base pointer,
    pub /: *mut *mut u32 rx_cnt_cfg; / rx max count configuration,
    pub /: *mut *mut u32 pcrx_ptr; / rx cpu pointer,
    pub /: *mut *mut u32 glo_cfg; / global configuration,
    pub /: *mut *mut u32 rst_idx; / reset index,
    pub /: *mut *mut u32 delay_irq; / delay interrupt,
    pub /: *mut *mut u32 irq_status; / interrupt status,
    pub /: *mut *mut u32 irq_mask; / interrupt mask,
    pub adma_rx_dbg0: u32,
    pub int_grp: u32,
    pub pdma: },
    pub /: *mut *mut u32 qtx_cfg; / tx queue configuration,
    pub /: *mut *mut u32 qtx_sch; / tx queue scheduler configuration,
    pub /: *mut *mut u32 rx_ptr; / rx base pointer,
    pub /: *mut *mut u32 rx_cnt_cfg; / rx max count configuration,
    pub /: *mut *mut u32 qcrx_ptr; / rx cpu pointer,
    pub /: *mut *mut u32 glo_cfg; / global configuration,
    pub /: *mut *mut u32 rst_idx; / reset index,
    pub /: *mut *mut u32 delay_irq; / delay interrupt,
    pub /: *mut *mut u32 fc_th; / flow control,
    pub int_grp: u32,
    pub /: *mut *mut u32 hred; / interrupt mask,
    pub /: *mut *mut u32 ctx_ptr; / tx acquire cpu pointer,
    pub /: *mut *mut u32 dtx_ptr; / tx acquire dma pointer,
    pub /: *mut *mut u32 crx_ptr; / tx release cpu pointer,
    pub /: *mut *mut u32 drx_ptr; / tx release dma pointer,
    pub /: *mut *mut u32 fq_head; / fq head pointer,
    pub /: *mut *mut u32 fq_tail; / fq tail pointer,
    pub /: *mut *mut u32 fq_count; / fq free page count,
    pub /: *mut *mut u32 fq_blen; / fq free page buffer length,
    pub /: *mut *mut u32 tx_sch_rate; / tx scheduler rate control registers,
    pub qdma: },
    pub gdm1_cnt: u32,
    pub gdma_to_ppe: [u32; 3],
    pub ppe_base: u32,
    pub wdma_base: [u32; 3],
    pub pse_iq_sta: u32,
    pub pse_oq_sta: u32,
}

// struct mtk_eth_data -	This is the structure holding all differences
// among various platforms
// @reg_map			Soc register map.
// @ana_rgc3:                   The offset for register ANA_RGC3 related to
// sgmiisys syscon
// @caps			Flags shown the extra capability for the SoC
// @hw_features			Flags shown HW features
// @required_clks		Flags shown the bitmap for required clocks on
// the target SoC
// @required_pctl		A bool value to show whether the SoC requires
// the extra setup for those pins used by GMAC.
// @hash_offset			Flow table hash offset.
// @version			SoC version.
// @foe_entry_size		Foe table entry size.
// @has_accounting		Bool indicating support for accounting of
// offloaded flows.
// @desc_size			Tx/Rx DMA descriptor size.
// @irq_done_mask		Rx irq done register mask.
// @dma_l4_valid		Rx DMA valid register mask.
// @dma_max_len			Max DMA tx/rx buffer length.
// @dma_len_offset		Tx/Rx DMA length field offset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_soc_data {
    pub reg_map: *const mtk_reg_map,
    pub ana_rgc3: u32,
    pub caps: u64,
    pub required_clks: u64,
    pub required_pctl: bool,
    pub offload_version: u8,
    pub hash_offset: u8,
    pub version: u8,
    pub ppe_num: u8,
    pub foe_entry_size: u16,
    pub hw_features: netdev_features_t,
    pub has_accounting: bool,
    pub disable_pll_modes: bool,
    pub desc_size: u32,
    pub dma_max_len: u32,
    pub dma_len_offset: u32,
    pub dma_size: u32,
    pub fq_dma_size: u32,
    pub tx: },
    pub desc_size: u32,
    pub irq_done_mask: u32,
    pub dma_l4_valid: u32,
    pub dma_max_len: u32,
    pub dma_len_offset: u32,
    pub dma_size: u32,
    pub rx: },
}

// currently no SoC has more than 3 macs
pub const MTK_MAX_DEVS: c_int = 3;
// struct mtk_eth -	This is the main datasructure for holding the state
// of the driver
// @dev:		The device pointer
// @dma_dev:		The device pointer used for dma mapping/alloc
// @base:		The mapped register i/o base
// @sram_pool:		Pointer to SRAM pool used for DMA descriptor rings
// @page_lock:		Make sure that register operations are atomic
// @tx_irq__lock:	Make sure that IRQ register operations are atomic
// @rx_irq__lock:	Make sure that IRQ register operations are atomic
// @dim_lock:		Make sure that Net DIM operations are atomic
// @dummy_dev:		we run 2 netdevs on 1 physical DMA ring and need a
// dummy for NAPI to work
// @netdev:		The netdev instances
// @mac:		Each netdev is linked to a physical MAC
// @irq:		The IRQ that we are using
// @msg_enable:		Ethtool msg level
// @ethsys:		The register map pointing at the range used to setup
// MII modes
// @infra:              The register map pointing at the range used to setup
// SGMII and GePHY path
// @sgmii_pcs:		Pointers to mtk-pcs-lynxi phylink_pcs instances
// @pctl:		The register map pointing at the range used to setup
// GMAC port drive/slew values
// @dma_refcnt:		track how many netdevs are using the DMA engine
// @tx_ring:		Pointer to the memory holding info about the TX ring
// @rx_ring:		Pointer to the memory holding info about the RX ring
// @rx_ring_qdma:	Pointer to the memory holding info about the QDMA RX ring
// @tx_napi:		The TX NAPI struct
// @rx_napi:		The RX NAPI struct
// @rx_events:		Net DIM RX event counter
// @rx_packets:		Net DIM RX packet counter
// @rx_bytes:		Net DIM RX byte counter
// @rx_dim:		Net DIM RX context
// @tx_events:		Net DIM TX event counter
// @tx_packets:		Net DIM TX packet counter
// @tx_bytes:		Net DIM TX byte counter
// @tx_dim:		Net DIM TX context
// @scratch_ring:	Newer SoCs need memory for a second HW managed TX ring
// @phy_scratch_ring:	physical address of scratch_ring
// @scratch_head:	The scratch memory that scratch_ring points to.
// @clks:		clock array for all clocks required
// @mii_bus:		If there is a bus we need to create an instance for it
// @pending_work:	The workqueue used to reset the dma ring
// @state:		Initialization and runtime state of the device
// @soc:		Holding specific data among various SoCs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_eth {
    pub dev: *mut device,
    pub dma_dev: *mut device,
    pub base: *mut void __iomem,
    pub sram_pool: *mut gen_pool,
    pub page_lock: spinlock_t,
    pub tx_irq_lock: spinlock_t,
    pub rx_irq_lock: spinlock_t,
    pub dummy_dev: *mut net_device,
    pub netdev: [*mut net_device; MTK_MAX_DEVS],
    pub mac: [*mut mtk_mac; MTK_MAX_DEVS],
    pub irq: [c_int; MTK_FE_IRQ_NUM],
    pub msg_enable: u32,
    pub sysclk: c_ulong,
    pub ethsys: *mut regmap,
    pub infra: *mut regmap,
    pub sgmii_pcs: [*mut phylink_pcs; MTK_MAX_DEVS],
    pub pctl: *mut regmap,
    pub hwlro: bool,
    pub dma_refcnt: refcount_t,
    pub tx_ring: mtk_tx_ring,
    pub rx_ring: [mtk_rx_ring; MTK_MAX_RX_RING_NUM],
    pub rx_ring_qdma: mtk_rx_ring,
    pub tx_napi: napi_struct,
    pub rx_napi: napi_struct,
    pub scratch_ring: *mut c_void,
    pub phy_scratch_ring: dma_addr_t,
    pub scratch_head: [*mut c_void; MTK_FQ_DMA_HEAD],
    pub clks: [*mut clk; MTK_CLK_MAX],
    pub mii_bus: *mut mii_bus,
    pub mdc_divider: c_uint,
    pub pending_work: work_struct,
    pub state: c_ulong,
    pub soc: *const mtk_soc_data,
    pub dim_lock: spinlock_t,
    pub rx_events: u32,
    pub rx_packets: u32,
    pub rx_bytes: u32,
    pub rx_dim: dim,
    pub tx_events: u32,
    pub tx_packets: u32,
    pub tx_bytes: u32,
    pub tx_dim: dim,
    pub ip_align: c_int,
    pub dsa_meta: [*mut metadata_dst; MTK_MAX_DSA_PORTS],
    pub ppe: [*mut mtk_ppe; 3],
    pub flow_table: rhashtable,
    pub prog: *mut bpf_prog __rcu,
    pub monitor_work: delayed_work,
    pub wdidx: u32,
    pub wdma_hang_count: u8,
    pub qdma_hang_count: u8,
    pub adma_hang_count: u8,
    pub reset: },
}

// struct mtk_mac -	the structure that holds the info about the MACs of the
// SoC
// @id:			The number of the MAC
// @interface:		Interface mode kept for detecting change in hw settings
// @of_node:		Our devicetree node
// @hw:			Backpointer to our main datastruture
// @hw_stats:		Packet statistics counter
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_mac {
    pub id: c_int,
    pub interface: phy_interface_t,
    pub ppe_idx: u8,
    pub speed: c_int,
    pub of_node: *mut device_node,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
    pub hw: *mut mtk_eth,
    pub hw_stats: *mut mtk_hw_stats,
    pub hwlro_ip: [__be32; MTK_MAX_LRO_IP_CNT],
    pub hwlro_ip_cnt: c_int,
    pub syscfg0: c_uint,
    pub device_notifier: notifier_block,
}

// the struct describing the SoC. these are declared in the soc_xyz.c files
extern "C" {
    pub fn FIELD_PREP(_arg: MTK_FOE_IB1_BIND_VLAN_LAYER_V2, _arg: val) -> return;
}
extern "C" {
    pub fn FIELD_PREP(_arg: MTK_FOE_IB1_BIND_VLAN_LAYER, _arg: val) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: MTK_FOE_IB1_BIND_VLAN_LAYER_V2, _arg: val) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: MTK_FOE_IB1_BIND_VLAN_LAYER, _arg: val) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: MTK_FOE_IB1_PACKET_TYPE_V2, _arg: val) -> return;
}
extern "C" {
    pub fn FIELD_GET(_arg: MTK_FOE_IB1_PACKET_TYPE, _arg: val) -> return;
}
// read the hardware status register
extern "C" {
    pub fn mtk_stats_update_mac(mac: *mut mtk_mac);
}
extern "C" {
    pub fn mtk_w32(eth: *mut mtk_eth, val: u32, reg: unsigned);
}
extern "C" {
    pub fn mtk_r32(eth: *mut mtk_eth, reg: unsigned) -> u32;
}
extern "C" {
    pub fn mtk_m32(eth: *mut mtk_eth, mask: u32, set: u32, reg: c_uint) -> u32;
}
extern "C" {
    pub fn mtk_gmac_sgmii_path_setup(eth: *mut mtk_eth, mac_id: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_gmac_2p5gphy_path_setup(eth: *mut mtk_eth, mac_id: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_gmac_gephy_path_setup(eth: *mut mtk_eth, mac_id: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_gmac_rgmii_path_setup(eth: *mut mtk_eth, mac_id: c_int) -> c_int;
}
extern "C" {
    pub fn mtk_eth_offload_init(eth: *mut mtk_eth, id: u8) -> c_int;
}
extern "C" {
    pub fn mtk_flow_offload_cleanup(eth: *mut mtk_eth, list: *mut list_head);
}
extern "C" {
    pub fn mtk_eth_set_dma_device(eth: *mut mtk_eth, dma_dev: *mut device);
}
