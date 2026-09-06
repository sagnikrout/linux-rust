//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/apm/xgene/xgene_enet_main.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Applied Micro X-Gene SoC Ethernet Driver
//
// Copyright (c) 2014, Applied Micro Circuits Corporation
// Authors: Iyappan Subramanian <isubramanian@apm.com>
// Ravi Patel <rapatel@apm.com>
// Keyur Chudgar <kchudgar@apm.com>
//

pub const ETHER_MIN_PACKET: c_int = 64;
pub const ETHER_STD_PACKET: c_int = 1518;
pub const XGENE_ENET_STD_MTU: c_int = 1536;
pub const XGENE_ENET_MAX_MTU: c_int = 9600;

pub const NUM_PKT_BUF: c_int = 1024;
pub const NUM_BUFPOOL: c_int = 32;
pub const NUM_NXTBUFPOOL: c_int = 8;
pub const MAX_EXP_BUFFS: c_int = 256;
pub const NUM_MSS_REG: c_int = 4;
pub const XGENE_MIN_ENET_FRAME_SIZE: c_int = 60;
pub const XGENE_MAX_ENET_IRQ: c_int = 16;
pub const XGENE_NUM_RX_RING: c_int = 8;
pub const XGENE_NUM_TX_RING: c_int = 8;
pub const XGENE_NUM_TXC_RING: c_int = 8;
pub const START_CPU_BUFNUM_0: c_int = 0;
pub const START_ETH_BUFNUM_0: c_int = 2;
pub const START_BP_BUFNUM_0: c_uint = 0x22;
pub const START_RING_NUM_0: c_int = 8;
pub const START_CPU_BUFNUM_1: c_int = 12;
pub const START_ETH_BUFNUM_1: c_int = 10;
pub const START_BP_BUFNUM_1: c_uint = 0x2A;
pub const START_RING_NUM_1: c_int = 264;
pub const XG_START_CPU_BUFNUM_1: c_int = 12;
pub const XG_START_ETH_BUFNUM_1: c_int = 2;
pub const XG_START_BP_BUFNUM_1: c_uint = 0x22;
pub const XG_START_RING_NUM_1: c_int = 264;
pub const X2_START_CPU_BUFNUM_0: c_int = 0;
pub const X2_START_ETH_BUFNUM_0: c_int = 0;
pub const X2_START_BP_BUFNUM_0: c_uint = 0x20;
pub const X2_START_RING_NUM_0: c_int = 0;
pub const X2_START_CPU_BUFNUM_1: c_uint = 0xc;
pub const X2_START_ETH_BUFNUM_1: c_int = 0;
pub const X2_START_BP_BUFNUM_1: c_uint = 0x20;
pub const X2_START_RING_NUM_1: c_int = 256;
pub const IRQ_ID_SIZE: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_id {
    XGENE_ENET1 = 1,
    XGENE_ENET2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xgene_enet_buf_len {
    SIZE_2K = 2048,
    SIZE_4K = 4096,
    SIZE_16K = 16384
}

// software context of a descriptor ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_enet_desc_ring {
    pub ndev: *mut net_device,
    pub id: u16,
    pub num: u16,
    pub head: u16,
    pub tail: u16,
    pub exp_buf_tail: u16,
    pub slots: u16,
    pub irq: u16,
    pub irq_name: [c_char; IRQ_ID_SIZE],
    pub size: u32,
    pub state: [u32; X2_NUM_RING_CONFIG],
    pub cmd_base: *mut void __iomem,
    pub cmd: *mut void __iomem,
    pub dma: dma_addr_t,
    pub irq_mbox_dma: dma_addr_t,
    pub irq_mbox_addr: *mut c_void,
    pub dst_ring_num: u16,
    pub nbufpool: u16,
    pub npagepool: c_int,
    pub index: u8,
    pub flags: u32,
    pub (*rx_skb): *mut sk_buff,
    pub (*cp_skb): *mut sk_buff,
    pub frag_dma_addr: *mut dma_addr_t,
    pub (*frag_page): *mut page,
    pub cfgsize: xgene_enet_ring_cfgsize,
    pub cp_ring: *mut xgene_enet_desc_ring,
    pub buf_pool: *mut xgene_enet_desc_ring,
    pub page_pool: *mut xgene_enet_desc_ring,
    pub napi: napi_struct,
    pub desc_addr: *mut c_void,
    pub raw_desc: *mut xgene_enet_raw_desc,
    pub raw_desc16: *mut xgene_enet_raw_desc16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_mac_ops {
    pub pdata): *mut *mut void (init)(struct xgene_enet_pdata,
    pub pdata): *mut *mut void (reset)(struct xgene_enet_pdata,
    pub pdata): *mut *mut void (tx_enable)(struct xgene_enet_pdata,
    pub pdata): *mut *mut void (rx_enable)(struct xgene_enet_pdata,
    pub pdata): *mut *mut void (tx_disable)(struct xgene_enet_pdata,
    pub pdata): *mut *mut void (rx_disable)(struct xgene_enet_pdata,
    pub tx): *mut *mut *mut *mut void (get_drop_cnt)(struct xgene_enet_pdata pdata, u32 rx, u32,
    pub pdata): *mut *mut void (set_speed)(struct xgene_enet_pdata,
    pub pdata): *mut *mut void (set_mac_addr)(struct xgene_enet_pdata,
    pub framesize): *mut *mut *mut void (set_framesize)(struct xgene_enet_pdata pdata, int,
    pub index): *mut *mut *mut void (set_mss)(struct xgene_enet_pdata pdata, u16 mss, u8,
    pub work): *mut *mut void (link_state)(struct work_struct,
    pub enable): *mut *mut *mut void (enable_tx_pause)(struct xgene_enet_pdata pdata, bool,
    pub enable): *mut *mut *mut void (flowctl_rx)(struct xgene_enet_pdata pdata, bool,
    pub enable): *mut *mut *mut void (flowctl_tx)(struct xgene_enet_pdata pdata, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_port_ops {
    pub pdata): *mut *mut int (reset)(struct xgene_enet_pdata,
    pub ring): *mut xgene_enet_desc_ring,
    pub nxtbufpool_id): u32 dst_ring_num, u16 bufpool_id, u16,
    pub pdata): *mut *mut void (shutdown)(struct xgene_enet_pdata,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_ring_ops {
    pub num_ring_config: u8,
    pub num_ring_id_shift: u8,
    pub ): *mut *mut *mut xgene_enet_desc_ring  (setup)(xgene_enet_desc_ring,
    pub ): *mut *mut void (clear)(struct xgene_enet_desc_ring,
    pub int): *mut *mut *mut void (wr_cmd)(struct xgene_enet_desc_ring ,,
    pub ): *mut *mut u32 (len)(struct xgene_enet_desc_ring,
    pub ): *mut *mut void (coalesce)(struct xgene_enet_desc_ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_cle_ops {
    pub pdata): *mut *mut int (cle_init)(struct xgene_enet_pdata,
}

// ethernet private data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_enet_pdata {
    pub ndev: *mut net_device,
    pub mdio_bus: *mut mii_bus,
    pub phy_speed: c_int,
    pub clk: *mut clk,
    pub pdev: *mut platform_device,
    pub enet_id: xgene_enet_id,
    pub tx_ring: [*mut xgene_enet_desc_ring; XGENE_NUM_TX_RING],
    pub rx_ring: [*mut xgene_enet_desc_ring; XGENE_NUM_RX_RING],
    pub tx_level: [u16; XGENE_NUM_TX_RING],
    pub txc_level: [u16; XGENE_NUM_TX_RING],
    pub dev_name: *mut c_char,
    pub rx_buff_cnt: u32,
    pub tx_qcnt_hi: u32,
    pub irqs: [u32; XGENE_MAX_ENET_IRQ],
    pub rxq_cnt: u8,
    pub txq_cnt: u8,
    pub cq_cnt: u8,
    pub eth_csr_addr: *mut void __iomem,
    pub eth_ring_if_addr: *mut void __iomem,
    pub eth_diag_csr_addr: *mut void __iomem,
    pub mcx_mac_addr: *mut void __iomem,
    pub mcx_mac_csr_addr: *mut void __iomem,
    pub mcx_stats_addr: *mut void __iomem,
    pub base_addr: *mut void __iomem,
    pub pcs_addr: *mut void __iomem,
    pub ring_csr_addr: *mut void __iomem,
    pub ring_cmd_addr: *mut void __iomem,
    pub phy_mode: c_int,
    pub rm: xgene_enet_rm,
    pub cle: xgene_enet_cle,
    pub extd_stats: *mut u64,
    pub false_rflr: u64,
    pub vlan_rjbr: u64,
    pub /: *mut *mut spinlock_t stats_lock; / statistics lock,
    pub mac_ops: *const xgene_mac_ops,
    pub /: *mut *mut spinlock_t mac_lock; / mac lock,
    pub port_ops: *const xgene_port_ops,
    pub ring_ops: *mut xgene_ring_ops,
    pub cle_ops: *const xgene_cle_ops,
    pub link_work: delayed_work,
    pub port_id: u32,
    pub cpu_bufnum: u8,
    pub eth_bufnum: u8,
    pub bp_bufnum: u8,
    pub ring_num: u16,
    pub mss: [u32; NUM_MSS_REG],
    pub mss_refcnt: [u32; NUM_MSS_REG],
    pub /: *mut *mut spinlock_t mss_lock; / mss lock,
    pub tx_delay: u8,
    pub rx_delay: u8,
    pub mdio_driver: bool,
    pub sfp_rdy: *mut gpio_desc,
    pub sfp_gpio_en: bool,
    pub pause_autoneg: u32,
    pub tx_pause: bool,
    pub rx_pause: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xgene_indirect_ctl {
    pub addr: *mut void __iomem,
    pub ctl: *mut void __iomem,
    pub cmd: *mut void __iomem,
    pub cmd_done: *mut void __iomem,
}

extern "C" {
    pub fn xgene_enet_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn xgene_extd_stats_init(pdata: *mut xgene_enet_pdata) -> c_int;
}
