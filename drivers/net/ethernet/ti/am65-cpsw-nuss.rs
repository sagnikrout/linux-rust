//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/am65-cpsw-nuss.h
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
// Copyright (C) 2020 Texas Instruments Incorporated - http://www.ti.com
//

pub const HOST_PORT_NUM: c_int = 0;

pub const AM65_CPSW_PORT_VLAN_REG_OFFSET: c_uint = 0x014;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_slave_data {
    pub mac_only: bool,
    pub mac_sl: *mut cpsw_sl,
    pub port_np: *mut device_node,
    pub phy_if: phy_interface_t,
    pub ifphy: *mut phy,
    pub serdes_phy: *mut phy,
    pub rx_pause: bool,
    pub tx_pause: bool,
    pub mac_addr: [u8; ETH_ALEN],
    pub port_vlan: c_int,
    pub phylink: *mut phylink,
    pub phylink_config: phylink_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_port {
    pub common: *mut am65_cpsw_common,
    pub ndev: *mut net_device,
    pub name: *const c_char,
    pub port_id: u32,
    pub port_base: *mut void __iomem,
    pub sgmii_base: *mut void __iomem,
    pub stat_base: *mut void __iomem,
    pub fetch_ram_base: *mut void __iomem,
    pub disabled: bool,
    pub slave: am65_cpsw_slave_data,
    pub tx_ts_enabled: bool,
    pub rx_ts_filter: hwtstamp_rx_filters,
    pub qos: am65_cpsw_qos,
    pub devlink_port: devlink_port,
    pub xdp_prog: *mut bpf_prog,
    pub xdp_rxq: [xdp_rxq_info; AM65_CPSW_MAX_QUEUES],
// Only for suspend resume context
    pub vid_context: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum am65_cpsw_tx_buf_type {
    AM65_CPSW_TX_BUF_TYPE_SKB,
    AM65_CPSW_TX_BUF_TYPE_XDP_TX,
    AM65_CPSW_TX_BUF_TYPE_XDP_NDO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_host {
    pub common: *mut am65_cpsw_common,
    pub port_base: *mut void __iomem,
    pub stat_base: *mut void __iomem,
// Only for suspend resume context
    pub vid_context: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_tx_chn {
    pub dma_dev: *mut device,
    pub napi_tx: napi_struct,
    pub common: *mut am65_cpsw_common,
    pub desc_pool: *mut k3_cppi_desc_pool,
    pub tx_chn: *mut k3_udma_glue_tx_channel,
    pub /: *mut *mut spinlock_t lock; / protect TX rings in multi-port mode,
    pub tx_hrtimer: hrtimer,
    pub tx_pace_timeout: c_ulong,
    pub irq: c_int,
    pub id: u32,
    pub descs_num: u32,
    pub dsize_log2: c_uchar,
    pub tx_chn_name: [c_char; 128],
    pub rate_mbps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_rx_flow {
    pub id: u32,
    pub napi_rx: napi_struct,
    pub common: *mut am65_cpsw_common,
    pub irq: c_int,
    pub irq_disabled: bool,
    pub rx_hrtimer: hrtimer,
    pub rx_pace_timeout: c_ulong,
    pub page_pool: *mut page_pool,
    pub name: [c_char; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_tx_swdata {
    pub ndev: *mut net_device,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_swdata {
    pub flow_id: u32,
    pub page: *mut page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_rx_chn {
    pub dev: *mut device,
    pub dma_dev: *mut device,
    pub desc_pool: *mut k3_cppi_desc_pool,
    pub rx_chn: *mut k3_udma_glue_rx_channel,
    pub descs_num: u32,
    pub dsize_log2: c_uchar,
    pub flows: [am65_cpsw_rx_flow; AM65_CPSW_MAX_QUEUES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_pdata {
    pub quirks: u32,
    pub extra_modes: u64,
    pub fdqring_mode: k3_ring_mode,
    pub ale_dev_id: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cpsw_devlink_param_id {
    AM65_CPSW_DEVLINK_PARAM_ID_BASE = DEVLINK_PARAM_GENERIC_ID_MAX,
    AM65_CPSW_DL_PARAM_SWITCH_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_devlink {
    pub common: *mut am65_cpsw_common,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_common {
    pub dev: *mut device,
    pub mdio_dev: *mut device,
    pub pdata: am65_cpsw_pdata,
    pub ss_base: *mut void __iomem,
    pub cpsw_base: *mut void __iomem,
    pub port_num: u32,
    pub host: am65_cpsw_host,
    pub ports: *mut am65_cpsw_port,
    pub disabled_ports_mask: u32,
    pub dma_ndev: *mut net_device,
    pub /: *mut *mut int usage_count; / number of opened ports,
    pub ale: *mut cpsw_ale,
    pub tx_ch_num: c_int,
    pub tx_ch_rate_msk: u32,
    pub rx_flow_id_base: u32,
    pub tx_chns: [am65_cpsw_tx_chn; AM65_CPSW_MAX_QUEUES],
    pub tdown_complete: completion,
    pub tdown_cnt: core::sync::atomic::AtomicI32,
    pub rx_ch_num_flows: c_int,
    pub rx_chns: am65_cpsw_rx_chn,
    pub nuss_ver: u32,
    pub cpsw_ver: u32,
    pub bus_freq: c_ulong,
    pub pf_p0_rx_ptype_rrobin: bool,
    pub cpts: *mut am65_cpts,
    pub est_enabled: c_int,
    pub iet_enabled: bool,
    pub is_emac_mode: bool,
    pub br_members: u16,
    pub default_vlan: c_int,
    pub devlink: *mut devlink,
    pub hw_bridge_dev: *mut net_device,
    pub am65_cpsw_netdevice_nb: notifier_block,
    pub switch_id: [c_uchar; MAX_PHYS_ITEM_ID_LEN],
// only for suspend/resume context restore
    pub ale_context: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct am65_cpsw_ndev_priv {
    pub msg_enable: u32,
    pub port: *mut am65_cpsw_port,
    pub offload_fwd_mark: bool,
// Serialize access to MAC Merge state between ethtool requests
// and link state updates
//
    pub mm_lock: mutex,
}

extern "C" {
    pub fn am65_cpsw_nuss_set_p0_ptype(common: *mut am65_cpsw_common);
}
extern "C" {
    pub fn am65_cpsw_port_dev_check(dev: *const net_device) -> bool;
}
