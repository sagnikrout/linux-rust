//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/icssg/icssg_prueth.h
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
// Texas Instruments ICSSG Ethernet driver
//
// Copyright (C) 2018-2022 Texas Instruments Incorporated - https://www.ti.com
//

pub const ICSS_SLICE0: c_int = 0;
pub const ICSS_SLICE1: c_int = 1;
pub const ICSS_FW_PRU: c_int = 0;
pub const ICSS_FW_RTU: c_int = 1;

pub const ICSSG_NUM_PA_STATS: c_int = 32;
pub const ICSSG_NUM_MIIG_STATS: c_int = 60;
// Number of ICSSG related stats

pub const ICSSG_NUM_STANDARD_STATS: c_int = 31;

pub const PRUETH_UNDIRECTED_PKT_DST_TAG: c_int = 0;

// Firmware status codes
pub const ICSS_HS_FW_READY: c_uint = 0x55555555;
pub const ICSS_HS_FW_DEAD: c_uint = 0xDEAD0000	/* lower 16 bits contain error code */;
// Firmware command codes
pub const ICSS_HS_CMD_BUSY: c_uint = 0x40000000;
pub const ICSS_HS_CMD_DONE: c_uint = 0x80000000;
pub const ICSS_HS_CMD_CANCEL: c_uint = 0x10000000;
// Firmware commands
pub const ICSS_CMD_SPAD: c_uint = 0x20;
pub const ICSS_CMD_RXTX: c_uint = 0x10;
pub const ICSS_CMD_ADD_FDB: c_uint = 0x1;
pub const ICSS_CMD_DEL_FDB: c_uint = 0x2;
pub const ICSS_CMD_SET_RUN: c_uint = 0x4;
pub const ICSS_CMD_GET_FDB_SLOT: c_uint = 0x5;
pub const ICSS_CMD_ENABLE_VLAN: c_uint = 0x5;
pub const ICSS_CMD_DISABLE_VLAN: c_uint = 0x6;
pub const ICSS_CMD_ADD_FILTER: c_uint = 0x7;
pub const ICSS_CMD_ADD_MAC: c_uint = 0x8;
// VLAN Filtering Related MACROs
pub const PRUETH_DFLT_VLAN_HSR: c_int = 1;
pub const PRUETH_DFLT_VLAN_SW: c_int = 1;
pub const PRUETH_DFLT_VLAN_MAC: c_int = 0;
pub const MAX_VLAN_ID: c_int = 256;
// In switch mode there are 3 real ports i.e. 3 mac addrs.
// however Linux sees only the host side port. The other 2 ports
// are the switch ports.
// In emac mode there are 2 real ports i.e. 2 mac addrs.
// Linux sees both the ports.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_port {
    PRUETH_PORT_HOST = 0,	/* host side port */
    PRUETH_PORT_MII0,	/* physical port RG/SG MII 0 */
    PRUETH_PORT_MII1,	/* physical port RG/SG MII 1 */
    PRUETH_PORT_INVALID,	/* Invalid prueth port */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_mac {
    PRUETH_MAC0 = 0,
    PRUETH_MAC1,
    PRUETH_NUM_MACS,
    PRUETH_MAC_INVALID,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_tx_chn {
    pub dma_dev: *mut device,
    pub napi_tx: napi_struct,
    pub desc_pool: *mut k3_cppi_desc_pool,
    pub tx_chn: *mut k3_udma_glue_tx_channel,
    pub emac: *mut prueth_emac,
    pub id: u32,
    pub descs_num: u32,
    pub irq: c_uint,
    pub name: [c_char; 32],
    pub tx_hrtimer: hrtimer,
    pub tx_pace_timeout_ns: c_ulong,
    pub xsk_pool: *mut xsk_buff_pool,
    pub irq_disabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_rx_chn {
    pub dev: *mut device,
    pub dma_dev: *mut device,
    pub desc_pool: *mut k3_cppi_desc_pool,
    pub rx_chn: *mut k3_udma_glue_rx_channel,
    pub descs_num: u32,
    pub /: *mut *mut unsigned int irq[ICSSG_MAX_RFLOWS]; / separate irq per flow,
    pub name: [c_char; 32],
    pub pg_pool: *mut page_pool,
    pub xdp_rxq: xdp_rxq_info,
    pub xsk_pool: *mut xsk_buff_pool,
    pub irq_disabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_swdata_type {
    PRUETH_SWDATA_INVALID = 0,
    PRUETH_SWDATA_SKB,
    PRUETH_SWDATA_PAGE,
    PRUETH_SWDATA_CMD,
    PRUETH_SWDATA_XDPF,
    PRUETH_SWDATA_XSK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum prueth_tx_buff_type {
    PRUETH_TX_BUFF_TYPE_XDP_TX,
    PRUETH_TX_BUFF_TYPE_XDP_NDO,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_swdata {
    pub type: prueth_swdata_type,
#[repr(C)]
#[derive(Copy, Clone)]
pub union prueth_data {
    pub skb: *mut sk_buff,
    pub page: *mut page,
    pub cmd: u32,
    pub xdpf: *mut xdp_frame,
    pub xdp: *mut xdp_buff,
    pub data: },
}

// There are 4 Tx DMA channels, but the highest priority is CH3 (thread 3)
// and lower three are lower priority channels or threads.
//
pub const PRUETH_MAX_TX_QUEUES: c_int = 4;

// XDP BPF state
pub const ICSSG_XDP_PASS: c_int = 0;

// Minimum coalesce time in usecs for both Tx and Rx
pub const ICSSG_MIN_COALESCE_USECS: c_int = 20;
// data for each emac port
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_emac {
    pub is_sr1: bool,
    pub prueth: *mut prueth,
    pub ndev: *mut net_device,
    pub mac_addr: [u8; 6],
    pub napi_rx: napi_struct,
    pub msg_enable: u32,
    pub link: c_int,
    pub speed: c_int,
    pub duplex: c_int,
    pub phy_id: *const c_char,
    pub phy_node: *mut device_node,
    pub phy_if: phy_interface_t,
    pub port_id: prueth_port,
    pub iep: *mut icss_iep,
    pub 1: unsigned int rx_ts_enabled :,
    pub 1: unsigned int tx_ts_enabled :,
    pub 1: unsigned int half_duplex :,
// DMA related
    pub tx_chns: [prueth_tx_chn; PRUETH_MAX_TX_QUEUES],
    pub tdown_complete: completion,
    pub tdown_cnt: core::sync::atomic::AtomicI32,
    pub rx_chns: prueth_rx_chn,
    pub rx_flow_id_base: c_int,
    pub tx_ch_num: c_int,
// SR1.0 Management channel
    pub rx_mgm_chn: prueth_rx_chn,
    pub rx_mgm_flow_id_base: c_int,
    pub /: *mut *mut spinlock_t lock; / serialize access,
// TX HW Timestamping
// TX TS cookie will be index to the tx_ts_skb array
    pub tx_ts_skb: [*mut sk_buff; PRUETH_MAX_TX_TS_REQUESTS],
    pub tx_ts_pending: core::sync::atomic::AtomicI32,
    pub tx_ts_irq: c_int,
    pub cmd_seq: u8,
// shutdown related
    pub cmd_data: [__le32; 4],
    pub cmd_complete: completion,
// Mutex to serialize access to firmware command interface
    pub cmd_lock: mutex,
    pub rx_mode_work: work_struct,
    pub dram: pruss_mem_region,
    pub offload_fwd_mark: bool,
    pub port_vlan: c_int,
    pub stats_work: delayed_work,
    pub stats: [u64; ICSSG_NUM_MIIG_STATS],
    pub pa_stats: [u64; ICSSG_NUM_PA_STATS],
// RX IRQ Coalescing Related
    pub rx_hrtimer: hrtimer,
    pub rx_pace_timeout_ns: c_ulong,
    pub vlan_mcast_list: [netdev_hw_addr_list; MAX_VLAN_ID],
    pub xdp_prog: *mut bpf_prog,
    pub xdpi: xdp_attachment_info,
    pub xsk_qid: c_int,
}

// The buf includes headroom compatible with both skb and xdpf

//
// struct prueth_pdata - PRUeth platform data
// @fdqring_mode: Free desc queue mode
// @quirk_10m_link_issue: 10M link detect errata
// @switch_mode: switch firmware support
// @banked_ms_ram: banked memory support
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth_pdata {
    pub fdqring_mode: k3_ring_mode,
    pub quirk_10m_link_issue:1: u32,
    pub switch_mode:1: u32,
    pub banked_ms_ram:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icssg_firmwares {
    pub pru: *const c_char,
    pub rtu: *const c_char,
    pub txpru: *const c_char,
}

//
// struct prueth - PRUeth structure
// @dev: device
// @pruss: pruss handle
// @pru: rproc instances of PRUs
// @rtu: rproc instances of RTUs
// @txpru: rproc instances of TX_PRUs
// @shram: PRUSS shared RAM region
// @sram_pool: MSMC RAM pool for buffers
// @msmcram: MSMC RAM region
// @eth_node: DT node for the port
// @emac: private EMAC data structure
// @registered_netdevs: list of registered netdevs
// @miig_rt: regmap to mii_g_rt block
// @mii_rt: regmap to mii_rt block
// @pa_stats: regmap to pa_stats block
// @pru_id: ID for each of the PRUs
// @pdev: pointer to ICSSG platform device
// @pdata: pointer to platform data for ICSSG driver
// @icssg_hwcmdseq: seq counter or HWQ messages
// @emacs_initialized: num of EMACs/ext ports that are up/running
// @iep0: pointer to IEP0 device
// @iep1: pointer to IEP1 device
// @vlan_tbl: VLAN-FID table pointer
// @hw_bridge_dev: pointer to HW bridge net device
// @hsr_dev: pointer to the HSR net device
// @hsr_prp_version: enum to store the protocol version of hsr master
// @br_members: bitmask of bridge member ports
// @hsr_members: bitmask of hsr member ports
// @prueth_netdevice_nb: netdevice notifier block
// @prueth_switchdev_nb: switchdev notifier block
// @prueth_switchdev_bl_nb: switchdev blocking notifier block
// @is_switch_mode: flag to indicate if device is in Switch mode
// @is_hsr_offload_mode: flag to indicate if device is in hsr offload mode
// @is_switchmode_supported: indicates platform support for switch mode
// @switch_id: ID for mapping switch ports to bridge
// @default_vlan: Default VLAN for host
// @icssg_emac_firmwares: Firmware names for EMAC mode, indexed per MAC
// @icssg_switch_firmwares: Firmware names for SWITCH mode, indexed per MAC
// @icssg_hsr_firmwares: Firmware names for HSR mode, indexed per MAC
// @icssg_prp_firmwares: Firmware names for PRP mode, indexed per MAC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct prueth {
    pub dev: *mut device,
    pub pruss: *mut pruss,
    pub pru: [*mut rproc; PRUSS_NUM_PRUS],
    pub rtu: [*mut rproc; PRUSS_NUM_PRUS],
    pub txpru: [*mut rproc; PRUSS_NUM_PRUS],
    pub shram: pruss_mem_region,
    pub sram_pool: *mut gen_pool,
    pub msmcram: pruss_mem_region,
    pub eth_node: [*mut device_node; PRUETH_NUM_MACS],
    pub emac: [*mut prueth_emac; PRUETH_NUM_MACS],
    pub registered_netdevs: [*mut net_device; PRUETH_NUM_MACS],
    pub miig_rt: *mut regmap,
    pub mii_rt: *mut regmap,
    pub pa_stats: *mut regmap,
    pub pru_id: [pruss_pru_id; PRUSS_NUM_PRUS],
    pub pdev: *mut platform_device,
    pub pdata: prueth_pdata,
    pub icssg_hwcmdseq: u8,
    pub emacs_initialized: c_int,
    pub iep0: *mut icss_iep,
    pub iep1: *mut icss_iep,
    pub vlan_tbl: *mut prueth_vlan_tbl,
    pub hw_bridge_dev: *mut net_device,
    pub hsr_dev: *mut net_device,
    pub hsr_prp_version: hsr_version,
    pub br_members: u8,
    pub hsr_members: u8,
    pub prueth_netdevice_nb: notifier_block,
    pub prueth_switchdev_nb: notifier_block,
    pub prueth_switchdev_bl_nb: notifier_block,
    pub is_switch_mode: bool,
    pub is_hsr_offload_mode: bool,
    pub is_switchmode_supported: bool,
    pub switch_id: [c_uchar; MAX_PHYS_ITEM_ID_LEN],
    pub default_vlan: c_int,
// @vtbl_lock: Lock for vtbl in shared memory
    pub vtbl_lock: spinlock_t,
// @stats_lock: Lock for reading icssg stats
    pub stats_lock: spinlock_t,
    pub icssg_emac_firmwares: [icssg_firmwares; PRUETH_NUM_MACS],
    pub icssg_switch_firmwares: [icssg_firmwares; PRUETH_NUM_MACS],
    pub icssg_hsr_firmwares: [icssg_firmwares; PRUETH_NUM_MACS],
    pub icssg_prp_firmwares: [icssg_firmwares; PRUETH_NUM_MACS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_tx_ts_response {
    pub reserved: [u32; 2],
    pub cookie: u32,
    pub lo_ts: u32,
    pub hi_ts: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct emac_tx_ts_response_sr1 {
    pub lo_ts: __le32,
    pub hi_ts: __le32,
    pub reserved: __le32,
    pub cookie: __le32,
}

// get PRUSS SLICE number from prueth_emac
// Classifier helpers
extern "C" {
    pub fn icssg_class_set_mac_addr(miig_rt: *mut regmap, slice: c_int, mac: *mut u8);
}
extern "C" {
    pub fn icssg_class_set_host_mac_addr(miig_rt: *mut regmap, mac: *const u8);
}
extern "C" {
    pub fn icssg_class_disable(miig_rt: *mut regmap, slice: c_int);
}
extern "C" {
    pub fn icssg_class_promiscuous_sr1(miig_rt: *mut regmap, slice: c_int);
}
extern "C" {
    pub fn icssg_ft1_set_mac_addr(miig_rt: *mut regmap, slice: c_int, mac_addr: *mut u8);
}
// config helpers
extern "C" {
    pub fn icssg_config_ipg(emac: *mut prueth_emac);
}
extern "C" {
    pub fn icssg_config_set_speed(emac: *mut prueth_emac);
}
extern "C" {
    pub fn icssg_config_half_duplex(emac: *mut prueth_emac);
}
extern "C" {
    pub fn icssg_init_emac_mode(prueth: *mut prueth);
}
extern "C" {
    pub fn icssg_init_fw_offload_mode(prueth: *mut prueth);
}
// Buffer queue helpers
extern "C" {
    pub fn icssg_queue_pop(prueth: *mut prueth, queue: u8) -> c_int;
}
extern "C" {
    pub fn icssg_queue_push(prueth: *mut prueth, queue: c_int, addr: u16);
}
extern "C" {
    pub fn icssg_queue_level(prueth: *mut prueth, queue: c_int) -> u32;
}
extern "C" {
    pub fn icssg_get_pvid(emac: *mut prueth_emac) -> u16;
}
extern "C" {
    pub fn icssg_set_pvid(prueth: *mut prueth, vid: u8, port: u8);
}
extern "C" {
    pub fn emac_fdb_flow_id_updated(emac: *mut prueth_emac) -> c_int;
}

extern "C" {
    pub fn icssg_stats_work_handler(work: *mut work_struct);
}
extern "C" {
    pub fn emac_update_hardware_stats(emac: *mut prueth_emac);
}
extern "C" {
    pub fn emac_get_stat_by_name(emac: *mut prueth_emac, stat_name: *mut c_char) -> c_int;
}
// Common functions
extern "C" {
    pub fn prueth_cleanup_tx_chns(emac: *mut prueth_emac);
}
extern "C" {
    pub fn prueth_ndev_del_tx_napi(emac: *mut prueth_emac, num: c_int);
}
extern "C" {
    pub fn prueth_ndev_add_tx_napi(emac: *mut prueth_emac) -> c_int;
}
extern "C" {
    pub fn prueth_init_tx_chns(emac: *mut prueth_emac) -> c_int;
}
extern "C" {
    pub fn prueth_rxbuf_total_len(len: c_uint) -> c_uint;
}
extern "C" {
    pub fn icssg_ndo_start_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx;
}
extern "C" {
    pub fn prueth_rx_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn prueth_cleanup_tx_ts(emac: *mut prueth_emac);
}
extern "C" {
    pub fn icssg_napi_rx_poll(napi_rx: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn icssg_ndo_tx_timeout(ndev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn prueth_node_port(eth_node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn prueth_node_mac(eth_node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn prueth_get_cores(prueth: *mut prueth, slice: c_int, is_sr1: bool) -> c_int;
}
extern "C" {
    pub fn prueth_put_cores(prueth: *mut prueth, slice: c_int);
}
// Revision specific helper
extern "C" {
    pub fn icssg_ts_to_ns(hi_sw: u32, hi: u32, lo: u32, cycle_time_ns: u32) -> u64;
}
extern "C" {
    pub fn prueth_rx_cleanup(data: *mut c_void, desc_dma: dma_addr_t);
}
extern "C" {
    pub fn prueth_tx_cleanup(data: *mut c_void, desc_dma: dma_addr_t);
}
extern "C" {
    pub fn prueth_xsk_wakeup(ndev: *mut net_device, qid: u32, flags: u32) -> c_int;
}
