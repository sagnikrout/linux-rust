//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/cpsw_priv.h
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
//
// Texas Instruments Ethernet Switch Driver
//

pub const ALE_ALL_PORTS: c_uint = 0x7;

pub const CPSW_VERSION_1: c_uint = 0x19010a;
pub const CPSW_VERSION_2: c_uint = 0x19010c;
pub const CPSW_VERSION_3: c_uint = 0x19010f;
pub const CPSW_VERSION_4: c_uint = 0x190112;
pub const HOST_PORT_NUM: c_int = 0;
pub const CPSW_ALE_PORTS_NUM: c_int = 3;
pub const CPSW_SLAVE_PORTS_NUM: c_int = 2;
pub const SLIVER_SIZE: c_uint = 0x40;
pub const CPSW1_HOST_PORT_OFFSET: c_uint = 0x028;
pub const CPSW1_SLAVE_OFFSET: c_uint = 0x050;
pub const CPSW1_SLAVE_SIZE: c_uint = 0x040;
pub const CPSW1_CPDMA_OFFSET: c_uint = 0x100;
pub const CPSW1_STATERAM_OFFSET: c_uint = 0x200;
pub const CPSW1_HW_STATS: c_uint = 0x400;
pub const CPSW1_CPTS_OFFSET: c_uint = 0x500;
pub const CPSW1_ALE_OFFSET: c_uint = 0x600;
pub const CPSW1_SLIVER_OFFSET: c_uint = 0x700;
pub const CPSW1_WR_OFFSET: c_uint = 0x900;
pub const CPSW2_HOST_PORT_OFFSET: c_uint = 0x108;
pub const CPSW2_SLAVE_OFFSET: c_uint = 0x200;
pub const CPSW2_SLAVE_SIZE: c_uint = 0x100;
pub const CPSW2_CPDMA_OFFSET: c_uint = 0x800;
pub const CPSW2_HW_STATS: c_uint = 0x900;
pub const CPSW2_STATERAM_OFFSET: c_uint = 0xa00;
pub const CPSW2_CPTS_OFFSET: c_uint = 0xc00;
pub const CPSW2_ALE_OFFSET: c_uint = 0xd00;
pub const CPSW2_SLIVER_OFFSET: c_uint = 0xd80;
pub const CPSW2_BD_OFFSET: c_uint = 0x2000;
pub const CPSW2_WR_OFFSET: c_uint = 0x1200;
pub const CPDMA_RXTHRESH: c_uint = 0x0c0;
pub const CPDMA_RXFREE: c_uint = 0x0e0;
pub const CPDMA_TXHDP: c_uint = 0x00;
pub const CPDMA_RXHDP: c_uint = 0x20;
pub const CPDMA_TXCP: c_uint = 0x40;
pub const CPDMA_RXCP: c_uint = 0x60;
pub const CPSW_RX_VLAN_ENCAP_HDR_SIZE: c_int = 4;

pub const RX_PRIORITY_MAPPING: c_uint = 0x76543210;
pub const TX_PRIORITY_MAPPING: c_uint = 0x33221100;
pub const CPDMA_TX_PRIORITY_MAP: c_uint = 0x76543210;

pub const CPSW_ALE_VLAN_AWARE: c_int = 1;

pub const CPSW_CMINTMAX_CNT: c_int = 63;
pub const CPSW_CMINTMIN_CNT: c_int = 2;

pub const IRQ_NUM: c_int = 2;
pub const CPSW_MAX_QUEUES: c_int = 8;
pub const CPSW_CPDMA_DESCS_POOL_SIZE_DEFAULT: c_int = 256;

pub const CPSW_FIFO_QUEUE_TYPE_SHIFT: c_int = 16;
pub const CPSW_FIFO_SHAPE_EN_SHIFT: c_int = 16;
pub const CPSW_FIFO_RATE_EN_SHIFT: c_int = 20;
pub const CPSW_TC_NUM: c_int = 4;

pub const CPSW_PCT_MASK: c_uint = 0x7f;
pub const CPSW_BD_RAM_SIZE: c_uint = 0x2000;
pub const CPSW_RX_VLAN_ENCAP_HDR_PRIO_SHIFT: c_int = 29;

pub const CPSW_RX_VLAN_ENCAP_HDR_VID_SHIFT: c_int = 16;
pub const CPSW_RX_VLAN_ENCAP_HDR_PKT_TYPE_SHIFT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_wr_regs {
    pub id_ver: u32,
    pub soft_reset: u32,
    pub control: u32,
    pub int_control: u32,
    pub rx_thresh_en: u32,
    pub rx_en: u32,
    pub tx_en: u32,
    pub misc_en: u32,
    pub mem_allign1: [u32; 8],
    pub rx_thresh_stat: u32,
    pub rx_stat: u32,
    pub tx_stat: u32,
    pub misc_stat: u32,
    pub mem_allign2: [u32; 8],
    pub rx_imax: u32,
    pub tx_imax: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_ss_regs {
    pub id_ver: u32,
    pub control: u32,
    pub soft_reset: u32,
    pub stat_port_en: u32,
    pub ptype: u32,
    pub soft_idle: u32,
    pub thru_rate: u32,
    pub gap_thresh: u32,
    pub tx_start_wds: u32,
    pub flow_control: u32,
    pub vlan_ltype: u32,
    pub ts_ltype: u32,
    pub dlr_ltype: u32,
}

// CPSW_PORT_V1
pub const CPSW1_MAX_BLKS: c_uint = 0x00 /* Maximum FIFO Blocks */;
pub const CPSW1_BLK_CNT: c_uint = 0x04 /* FIFO Block Usage Count (Read Only) */;
pub const CPSW1_TX_IN_CTL: c_uint = 0x08 /* Transmit FIFO Control */;
pub const CPSW1_PORT_VLAN: c_uint = 0x0c /* VLAN Register */;
pub const CPSW1_TX_PRI_MAP: c_uint = 0x10 /* Tx Header Priority to Switch Pri Mapping */;
pub const CPSW1_TS_CTL: c_uint = 0x14 /* Time Sync Control */;
pub const CPSW1_TS_SEQ_LTYPE: c_uint = 0x18 /* Time Sync Sequence ID Offset and Msg Type */;
pub const CPSW1_TS_VLAN: c_uint = 0x1c /* Time Sync VLAN1 and VLAN2 */;
// CPSW_PORT_V2
pub const CPSW2_CONTROL: c_uint = 0x00 /* Control Register */;
pub const CPSW2_MAX_BLKS: c_uint = 0x08 /* Maximum FIFO Blocks */;
pub const CPSW2_BLK_CNT: c_uint = 0x0c /* FIFO Block Usage Count (Read Only) */;
pub const CPSW2_TX_IN_CTL: c_uint = 0x10 /* Transmit FIFO Control */;
pub const CPSW2_PORT_VLAN: c_uint = 0x14 /* VLAN Register */;
pub const CPSW2_TX_PRI_MAP: c_uint = 0x18 /* Tx Header Priority to Switch Pri Mapping */;
pub const CPSW2_TS_SEQ_MTYPE: c_uint = 0x1c /* Time Sync Sequence ID Offset and Msg Type */;
// CPSW_PORT_V1 and V2
pub const SA_LO: c_uint = 0x20 /* CPGMAC_SL Source Address Low */;
pub const SA_HI: c_uint = 0x24 /* CPGMAC_SL Source Address High */;
pub const SEND_PERCENT: c_uint = 0x28 /* Transmit Queue Send Percentages */;
// CPSW_PORT_V2 only
pub const RX_DSCP_PRI_MAP0: c_uint = 0x30 /* Rx DSCP Priority to Rx Packet Mapping */;
pub const RX_DSCP_PRI_MAP1: c_uint = 0x34 /* Rx DSCP Priority to Rx Packet Mapping */;
pub const RX_DSCP_PRI_MAP2: c_uint = 0x38 /* Rx DSCP Priority to Rx Packet Mapping */;
pub const RX_DSCP_PRI_MAP3: c_uint = 0x3c /* Rx DSCP Priority to Rx Packet Mapping */;
pub const RX_DSCP_PRI_MAP4: c_uint = 0x40 /* Rx DSCP Priority to Rx Packet Mapping */;
pub const RX_DSCP_PRI_MAP5: c_uint = 0x44 /* Rx DSCP Priority to Rx Packet Mapping */;
pub const RX_DSCP_PRI_MAP6: c_uint = 0x48 /* Rx DSCP Priority to Rx Packet Mapping */;
pub const RX_DSCP_PRI_MAP7: c_uint = 0x4c /* Rx DSCP Priority to Rx Packet Mapping */;
// Bit definitions for the CPSW2_CONTROL register

// Bit definitions for the CPSW2_TS_SEQ_MTYPE register

// The PTP event messages - Sync, Delay_Req, Pdelay_Req, and Pdelay_Resp.

// Bit definitions for the CPSW1_TS_CTL register

pub const CPSW_V1_MSG_TYPE_OFS: c_int = 16;
// Bit definitions for the CPSW1_TS_SEQ_LTYPE register
pub const CPSW_V1_SEQ_ID_OFS_SHIFT: c_int = 16;
pub const CPSW_MAX_BLKS_TX: c_int = 15;
pub const CPSW_MAX_BLKS_TX_SHIFT: c_int = 4;
pub const CPSW_MAX_BLKS_RX: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_host_regs {
    pub max_blks: u32,
    pub blk_cnt: u32,
    pub tx_in_ctl: u32,
    pub port_vlan: u32,
    pub tx_pri_map: u32,
    pub cpdma_tx_pri_map: u32,
    pub cpdma_rx_chan_map: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_slave_data {
    pub slave_node: *mut device_node,
    pub phy_node: *mut device_node,
    pub phy_id: [c_char; MII_BUS_ID_SIZE],
    pub phy_if: phy_interface_t,
    pub mac_addr: [u8; ETH_ALEN],
    pub /: *mut *mut u16 dual_emac_res_vlan; / Reserved VLAN for DualEMAC,
    pub ifphy: *mut phy,
    pub disabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_platform_data {
    pub slave_data: *mut cpsw_slave_data,
    pub /: *mut *mut u32 ss_reg_ofs; / Subsystem control register offset,
    pub /: *mut *mut u32 channels; / number of cpdma channels (symmetric),
    pub /: *mut *mut u32 slaves; / number of slave cpgmac ports,
    pub /: *mut *mut u32 active_slave;/ time stamping, ethtool and SIOCGMIIPHY slave,
    pub /: *mut *mut u32 bd_ram_size; /buffer descriptor ram size,
    pub /: *mut *mut u32 mac_control; / Mac control register,
    pub mode*/: *mut *mut u16 default_vlan; / Def VLAN for ALE lookup in VLAN aware,
    pub /: *mut *mut bool dual_emac; / Enable Dual EMAC mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_slave {
    pub regs: *mut void __iomem,
    pub slave_num: c_int,
    pub mac_control: u32,
    pub data: *mut cpsw_slave_data,
    pub phy: *mut phy_device,
    pub ndev: *mut net_device,
    pub port_vlan: u32,
    pub mac_sl: *mut cpsw_sl,
}

extern "C" {
    pub fn readl_relaxed(offset: slave->regs +) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_vector {
    pub ch: *mut cpdma_chan,
    pub budget: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_common {
    pub dev: *mut device,
    pub data: cpsw_platform_data,
    pub napi_rx: napi_struct,
    pub napi_tx: napi_struct,
    pub regs: *mut cpsw_ss_regs __iomem,
    pub wr_regs: *mut cpsw_wr_regs __iomem,
    pub hw_stats: *mut u8 __iomem,
    pub host_port_regs: *mut cpsw_host_regs __iomem,
    pub version: u32,
    pub coal_intvl: u32,
    pub bus_freq_mhz: u32,
    pub rx_packet_max: c_int,
    pub descs_pool_size: c_int,
    pub slaves: *mut cpsw_slave,
    pub dma: *mut cpdma_ctlr,
    pub txv: [cpsw_vector; CPSW_MAX_QUEUES],
    pub rxv: [cpsw_vector; CPSW_MAX_QUEUES],
    pub ale: *mut cpsw_ale,
    pub quirk_irq: bool,
    pub rx_irq_disabled: bool,
    pub tx_irq_disabled: bool,
    pub irqs_table: [u32; IRQ_NUM],
    pub misc_irq: c_int,
    pub cpts: *mut cpts,
    pub devlink: *mut devlink,
    pub tx_ch_num: int rx_ch_num,,
    pub speed: c_int,
    pub usage_count: c_int,
    pub page_pool: [*mut page_pool; CPSW_MAX_QUEUES],
    pub br_members: u8,
    pub hw_bridge_dev: *mut net_device,
    pub ale_bypass: bool,
    pub base_mac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_ale_ratelimit {
    pub cookie: c_ulong,
    pub rate_packet_ps: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpsw_priv {
    pub ndev: *mut net_device,
    pub dev: *mut device,
    pub msg_enable: u32,
    pub mac_addr: [u8; ETH_ALEN],
    pub rx_pause: bool,
    pub tx_pause: bool,
    pub mqprio_hw: bool,
    pub fifo_bw: [c_int; CPSW_TC_NUM],
    pub shp_cfg_speed: c_int,
    pub tx_ts_enabled: c_int,
    pub rx_ts_enabled: c_int,
    pub xdp_prog: *mut bpf_prog,
    pub xdp_rxq: [xdp_rxq_info; CPSW_MAX_QUEUES],
    pub xdpi: xdp_attachment_info,
    pub emac_port: u32,
    pub cpsw: *mut cpsw_common,
    pub offload_fwd_mark: c_int,
    pub tx_packet_min: u32,
    pub ale_bc_ratelimit: cpsw_ale_ratelimit,
    pub ale_mc_ratelimit: cpsw_ale_ratelimit,
    pub rx_mode_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_sync_ctx {
    pub ndev: *mut net_device,
    pub /: *const *const *const u8 addr; / address to be synched,
    pub /: *mut *mut int consumed; / number of address instances,
    pub /: *mut *mut int flush; / flush flag,
}

pub const CPSW_XDP_CONSUMED: c_int = 1;
pub const CPSW_XDP_PASS: c_int = 0;
// The buf includes headroom compatible with both skb and xdpf

extern "C" {
    pub fn cpsw_split_res(cpsw: *mut cpsw_common);
}
extern "C" {
    pub fn cpsw_fill_rx_channels(priv: *mut cpsw_priv) -> c_int;
}
extern "C" {
    pub fn cpsw_intr_enable(cpsw: *mut cpsw_common);
}
extern "C" {
    pub fn cpsw_intr_disable(cpsw: *mut cpsw_common);
}
extern "C" {
    pub fn cpsw_tx_handler(token: *mut c_void, len: c_int, status: c_int);
}
extern "C" {
    pub fn cpsw_create_xdp_rxqs(cpsw: *mut cpsw_common) -> c_int;
}
extern "C" {
    pub fn cpsw_destroy_xdp_rxqs(cpsw: *mut cpsw_common);
}
extern "C" {
    pub fn cpsw_ndo_bpf(ndev: *mut net_device, bpf: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn cpsw_tx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cpsw_rx_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cpsw_misc_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cpsw_tx_mq_poll(napi_tx: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_tx_poll(napi_tx: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_rx_mq_poll(napi_rx: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_rx_poll(napi_rx: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_rx_vlan_encap(skb: *mut sk_buff);
}
extern "C" {
    pub fn cpsw_soft_reset(module: *const c_char, reg: *mut void __iomem);
}
extern "C" {
    pub fn cpsw_set_slave_mac(slave: *mut cpsw_slave, priv: *mut cpsw_priv);
}
extern "C" {
    pub fn cpsw_ndo_tx_timeout(ndev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn cpsw_need_resplit(cpsw: *mut cpsw_common) -> c_int;
}
extern "C" {
    pub fn cpsw_ndo_set_tx_maxrate(ndev: *mut net_device, queue: c_int, rate: u32) -> c_int;
}
extern "C" {
    pub fn cpsw_shp_is_off(priv: *mut cpsw_priv) -> bool;
}
extern "C" {
    pub fn cpsw_cbs_resume(slave: *mut cpsw_slave, priv: *mut cpsw_priv);
}
extern "C" {
    pub fn cpsw_mqprio_resume(slave: *mut cpsw_slave, priv: *mut cpsw_priv);
}
extern "C" {
    pub fn cpsw_qos_clsflower_resume(priv: *mut cpsw_priv);
}
// ethtool
extern "C" {
    pub fn cpsw_get_msglevel(ndev: *mut net_device) -> u32;
}
extern "C" {
    pub fn cpsw_set_msglevel(ndev: *mut net_device, value: u32);
}
extern "C" {
    pub fn cpsw_get_sset_count(ndev: *mut net_device, sset: c_int) -> c_int;
}
extern "C" {
    pub fn cpsw_get_strings(ndev: *mut net_device, stringset: u32, data: *mut u8);
}
extern "C" {
    pub fn cpsw_get_wol(ndev: *mut net_device, wol: *mut ethtool_wolinfo);
}
extern "C" {
    pub fn cpsw_set_wol(ndev: *mut net_device, wol: *mut ethtool_wolinfo) -> c_int;
}
extern "C" {
    pub fn cpsw_get_regs_len(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn cpsw_get_regs(ndev: *mut net_device, regs: *mut ethtool_regs, p: *mut c_void);
}
extern "C" {
    pub fn cpsw_ethtool_op_begin(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn cpsw_ethtool_op_complete(ndev: *mut net_device);
}
extern "C" {
    pub fn cpsw_get_channels(ndev: *mut net_device, ch: *mut ethtool_channels);
}
extern "C" {
    pub fn cpsw_get_eee(ndev: *mut net_device, edata: *mut ethtool_keee) -> c_int;
}
extern "C" {
    pub fn cpsw_nway_reset(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn cpsw_get_ts_info(ndev: *mut net_device, info: *mut kernel_ethtool_ts_info) -> c_int;
}
