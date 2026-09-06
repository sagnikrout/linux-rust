//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/asp2/bcmasp.h
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

pub const ASP_INTR2_OFFSET: c_uint = 0x1000;
pub const ASP_INTR2_STATUS: c_uint = 0x0;
pub const ASP_INTR2_SET: c_uint = 0x4;
pub const ASP_INTR2_CLEAR: c_uint = 0x8;
pub const ASP_INTR2_MASK_STATUS: c_uint = 0xc;
pub const ASP_INTR2_MASK_SET: c_uint = 0x10;
pub const ASP_INTR2_MASK_CLEAR: c_uint = 0x14;

pub const ASP_WAKEUP_INTR2_OFFSET: c_uint = 0x1200;
pub const ASP_WAKEUP_INTR2_STATUS: c_uint = 0x0;
pub const ASP_WAKEUP_INTR2_SET: c_uint = 0x4;
pub const ASP_WAKEUP_INTR2_CLEAR: c_uint = 0x8;
pub const ASP_WAKEUP_INTR2_MASK_STATUS: c_uint = 0xc;
pub const ASP_WAKEUP_INTR2_MASK_SET: c_uint = 0x10;
pub const ASP_WAKEUP_INTR2_MASK_CLEAR: c_uint = 0x14;

pub const ASP_CTRL2_OFFSET: c_uint = 0x2000;
pub const ASP_CTRL2_CORE_CLOCK_SELECT: c_uint = 0x0;

pub const ASP_CTRL2_CPU_CLOCK_SELECT: c_uint = 0x4;

pub const ASP_TX_ANALYTICS_OFFSET: c_uint = 0x4c000;
pub const ASP_TX_ANALYTICS_CTRL: c_uint = 0x0;
pub const ASP_RX_ANALYTICS_OFFSET: c_uint = 0x98000;
pub const ASP_RX_ANALYTICS_CTRL: c_uint = 0x0;
pub const ASP_RX_CTRL_OFFSET: c_uint = 0x9f000;
pub const ASP_RX_CTRL_UMAC_0_FRAME_COUNT: c_uint = 0x8;
pub const ASP_RX_CTRL_UMAC_1_FRAME_COUNT: c_uint = 0xc;
pub const ASP_RX_CTRL_FB_0_FRAME_COUNT: c_uint = 0x14;
pub const ASP_RX_CTRL_FB_1_FRAME_COUNT: c_uint = 0x18;
pub const ASP_RX_CTRL_FB_8_FRAME_COUNT: c_uint = 0x1c;
pub const ASP_RX_CTRL_FB_9_FRAME_COUNT: c_uint = 0x20;
pub const ASP_RX_CTRL_FB_10_FRAME_COUNT: c_uint = 0x24;
pub const ASP_RX_CTRL_FB_OUT_FRAME_COUNT: c_uint = 0x28;
pub const ASP_RX_CTRL_FB_FILT_OUT_FRAME_COUNT: c_uint = 0x2c;
pub const ASP_RX_CTRL_FLUSH: c_uint = 0x30;

pub const ASP_RX_CTRL_FB_RX_FIFO_DEPTH: c_uint = 0x38;
pub const ASP_RX_FILTER_OFFSET: c_uint = 0x80000;
pub const ASP_RX_FILTER_BLK_CTRL: c_uint = 0x0;

pub const ASP_RX_FILTER_MDA_CFG_EN_SHIFT: c_int = 8;

pub const ASP_RX_FILTER_NET_OFFSET_MAX: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum asp_rx_net_filter_block {
    ASP_RX_FILTER_NET_L2 = 0,
    ASP_RX_FILTER_NET_L3_0,
    ASP_RX_FILTER_NET_L3_1,
    ASP_RX_FILTER_NET_L4,
    ASP_RX_FILTER_NET_BLOCK_MAX
}

pub const ASP_EDPKT_OFFSET: c_uint = 0x9c000;
pub const ASP_EDPKT_ENABLE: c_uint = 0x4;

pub const ASP_EDPKT_HDR_CFG: c_uint = 0xc;
pub const ASP_EDPKT_HDR_SZ_SHIFT: c_int = 2;
pub const ASP_EDPKT_HDR_SZ_32: c_int = 0;
pub const ASP_EDPKT_HDR_SZ_64: c_int = 1;
pub const ASP_EDPKT_HDR_SZ_96: c_int = 2;
pub const ASP_EDPKT_HDR_SZ_128: c_int = 3;
pub const ASP_EDPKT_BURST_BUF_PSCAL_TOUT: c_uint = 0x10;
pub const ASP_EDPKT_BURST_BUF_WRITE_TOUT: c_uint = 0x14;
pub const ASP_EDPKT_BURST_BUF_READ_TOUT: c_uint = 0x18;
pub const ASP_EDPKT_RX_TS_COUNTER: c_uint = 0x38;
pub const ASP_EDPKT_ENDI: c_uint = 0x48;
pub const ASP_EDPKT_ENDI_DESC_SHIFT: c_int = 8;
pub const ASP_EDPKT_ENDI_NO_BT_SWP: c_int = 0;
pub const ASP_EDPKT_ENDI_BT_SWP_WD: c_int = 1;
pub const ASP_EDPKT_RX_PKT_CNT: c_uint = 0x138;
pub const ASP_EDPKT_HDR_EXTR_CNT: c_uint = 0x13c;
pub const ASP_EDPKT_HDR_OUT_CNT: c_uint = 0x140;
pub const ASP_EDPKT_SPARE_REG: c_uint = 0x174;

pub const ASP_CTRL_OFFSET: c_uint = 0x101000;
pub const ASP_CTRL_ASP_SW_INIT: c_uint = 0x04;

pub const ASP_CTRL_CLOCK_CTRL: c_uint = 0x04;

pub const ASP_CTRL_CLOCK_CTRL_ASP_RGMII_SHIFT: c_int = 2;

pub const ASP_CTRL_CORE_CLOCK_SELECT: c_uint = 0x08;

pub const ASP_CTRL_SCRATCH_0: c_uint = 0x0c;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_tx_cb {
    pub skb: *mut sk_buff,
    pub bytes_sent: c_uint,
    pub last: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_res {
// Per interface resources
// Port
    pub umac: *mut void __iomem,
    pub umac2fb: *mut void __iomem,
    pub rgmii: *mut void __iomem,
// TX slowpath/configuration
    pub tx_spb_ctrl: *mut void __iomem,
    pub tx_spb_top: *mut void __iomem,
    pub tx_epkt_core: *mut void __iomem,
    pub tx_pause_ctrl: *mut void __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_desc {
    pub buf: u64,

// 39:0 (TX/RX) bits 0-39 of buf addr
// 40 (RX) checksum
// 41 (RX) crc_error
// 42 (RX) rx_symbol_error
// 43 (RX) non_octet_aligned
// 44 (RX) pkt_truncated
// 45 Reserved
// 56:46 (RX) mac_filter_id
// 60:57 (RX) rx_port_num (0-unicmac0, 1-unimac1)
// 61 Reserved
// 63:62 (TX) forward CRC, overwrite CRC
//
    pub size: u32,
    pub flags: u32,

// 0 (TX) tx_int_en
// 1 (TX/RX) SOF
// 2 (TX/RX) EOF
// 3 (TX) epkt_command
// 6:4 (TX) PA
// 7 (TX) pause at desc end
// 8 (TX) scram_start
// 9 (TX) scram_end
// 10 (TX) PCPP
// 11 (TX) PPPP
// 14:12 Reserved
// 15 (TX) pid ch Valid
// 19:16 (TX) data_pkt_type
// 32:20 (TX) pid_channel (RX) nw_filter_id
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_intf_stats64 {
// Rx Stats
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_errors: u64_stats_t,
    pub rx_dropped: u64_stats_t,
    pub rx_crc_errs: u64_stats_t,
    pub rx_sym_errs: u64_stats_t,
// Tx Stats
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_mib_counters {
    pub edpkt_ts: u32,
    pub edpkt_rx_pkt_cnt: u32,
    pub edpkt_hdr_ext_cnt: u32,
    pub edpkt_hdr_out_cnt: u32,
    pub umac_frm_cnt: u32,
    pub fb_frm_cnt: u32,
    pub fb_rx_fifo_depth: u32,
    pub fb_out_frm_cnt: u32,
    pub fb_filt_out_frm_cnt: u32,
    pub alloc_rx_skb_failed: u32,
    pub tx_dma_failed: u32,
    pub mc_filters_full_cnt: u32,
    pub uc_filters_full_cnt: u32,
    pub filters_combine_cnt: u32,
    pub promisc_filters_cnt: u32,
    pub tx_realloc_offload_failed: u32,
    pub tx_timeout_cnt: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_intf {
    pub list: list_head,
    pub ndev: *mut net_device,
    pub parent: *mut bcmasp_priv,
// ASP Ch
    pub channel: c_int,
    pub port: c_int,
// Used for splitting shared resources
    pub index: c_int,
    pub tx_napi: napi_struct,
// TX ring, starts on a new cacheline boundary
    pub tx_spb_dma: *mut void __iomem,
    pub tx_spb_index: c_int,
    pub tx_spb_clean_index: c_int,
    pub tx_spb_cpu: *mut bcmasp_desc,
    pub tx_spb_dma_addr: dma_addr_t,
    pub tx_spb_dma_valid: dma_addr_t,
    pub tx_spb_dma_read: dma_addr_t,
    pub tx_cbs: *mut bcmasp_tx_cb,
// RX ring, starts on a new cacheline boundary
    pub rx_edpkt_cfg: *mut void __iomem,
    pub rx_edpkt_dma: *mut void __iomem,
    pub rx_edpkt_index: c_int,
    pub rx_edpkt_cpu: *mut bcmasp_desc,
    pub rx_edpkt_dma_addr: dma_addr_t,
    pub rx_edpkt_dma_read: dma_addr_t,
    pub rx_edpkt_dma_valid: dma_addr_t,
// Streaming RX data ring (RBUF_4K mode)
    pub rx_ring_cpu: *mut c_void,
    pub rx_ring_dma: dma_addr_t,
    pub rx_ring_dma_valid: dma_addr_t,
    pub rx_buf_order: c_int,
// Page pool for recycling RX SKB data pages
    pub rx_page_pool: *mut page_pool,
    pub rx_napi: napi_struct,
    pub res: bcmasp_res,
    pub crc_fwd: c_uint,
// PHY device
    pub phy_dn: *mut device_node,
    pub ndev_dn: *mut device_node,
    pub phy_interface: phy_interface_t,
    pub internal_phy: bool,
    pub old_pause: c_int,
    pub old_link: c_int,
    pub old_duplex: c_int,
    pub msg_enable: u32,
// Statistics
    pub stats64: bcmasp_intf_stats64,
    pub mib: bcmasp_mib_counters,
    pub wolopts: u32,
    pub sopass: [u8; SOPASS_MAX],
}

pub const NUM_NET_FILTERS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_net_filter {
    pub fs: ethtool_rx_flow_spec,
    pub claimed: bool,
    pub wake_filter: bool,
    pub port: c_int,
    pub ch: c_int,
    pub hw_index: c_uint,
}

pub const NUM_MDA_FILTERS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_mda_filter {
// Current owner of this filter
    pub port: c_int,
    pub en: bool,
    pub addr: [u8; ETH_ALEN],
    pub mask: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_plat_data {
    pub slow): *mut *mut *mut void (core_clock_select)(struct bcmasp_priv priv, bool,
    pub en): *mut *mut *mut void (eee_fixup)(struct bcmasp_intf priv, bool,
    pub num_mda_filters: c_uint,
    pub num_net_filters: c_uint,
    pub tx_chan_offset: c_uint,
    pub rx_ctrl_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_priv {
    pub pdev: *mut platform_device,
    pub clk: *mut clk,
    pub irq: c_int,
    pub irq_mask: u32,
// Used if shared wol irq
    pub wol_lock: mutex,
    pub wol_irq: c_int,
    pub wol_irq_enabled_mask: c_ulong,
    pub slow): *mut *mut *mut void (core_clock_select)(struct bcmasp_priv priv, bool,
    pub en): *mut *mut *mut void (eee_fixup)(struct bcmasp_intf intf, bool,
    pub num_mda_filters: c_uint,
    pub num_net_filters: c_uint,
    pub tx_chan_offset: c_uint,
    pub rx_ctrl_offset: c_uint,
    pub base: *mut void __iomem,
    pub intfs: list_head,
    pub mda_filters: *mut bcmasp_mda_filter,
// MAC destination address filters lock
    pub mda_lock: spinlock_t,
// Protects accesses to ASP_CTRL_CLOCK_CTRL
    pub clk_lock: spinlock_t,
    pub net_filters: *mut bcmasp_net_filter,
// Network filter lock
    pub net_lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcmasp_pkt_offload {
    pub nop: __be32,
    pub header: __be32,
    pub header2: __be32,
    pub epkt: __be32,
    pub end: __be32,
}

extern "C" {
    pub fn bcmasp_interface_destroy(intf: *mut bcmasp_intf);
}
extern "C" {
    pub fn bcmasp_enable_tx_irq(intf: *mut bcmasp_intf, en: c_int);
}
extern "C" {
    pub fn bcmasp_enable_rx_irq(intf: *mut bcmasp_intf, en: c_int);
}
extern "C" {
    pub fn bcmasp_enable_phy_irq(intf: *mut bcmasp_intf, en: c_int);
}
extern "C" {
    pub fn bcmasp_flush_rx_port(intf: *mut bcmasp_intf);
}
extern "C" {
    pub fn bcmasp_interface_suspend(intf: *mut bcmasp_intf) -> c_int;
}
extern "C" {
    pub fn bcmasp_interface_resume(intf: *mut bcmasp_intf) -> c_int;
}
extern "C" {
    pub fn bcmasp_set_promisc(intf: *mut bcmasp_intf, en: bool);
}
extern "C" {
    pub fn bcmasp_set_allmulti(intf: *mut bcmasp_intf, en: bool);
}
extern "C" {
    pub fn bcmasp_set_broad(intf: *mut bcmasp_intf, en: bool);
}
extern "C" {
    pub fn bcmasp_disable_all_filters(intf: *mut bcmasp_intf);
}
extern "C" {
    pub fn bcmasp_core_clock_set_intf(intf: *mut bcmasp_intf, en: bool);
}
extern "C" {
    pub fn bcmasp_netfilt_get_active(intf: *mut bcmasp_intf) -> c_int;
}
extern "C" {
    pub fn bcmasp_netfilt_suspend(intf: *mut bcmasp_intf);
}
extern "C" {
    pub fn bcmasp_enable_wol(intf: *mut bcmasp_intf, en: bool);
}
