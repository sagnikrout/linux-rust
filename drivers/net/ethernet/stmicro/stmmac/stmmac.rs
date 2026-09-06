//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/stmmac.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_resources {
    pub addr: *mut void __iomem,
    pub mac: [u8; ETH_ALEN],
    pub wol_irq: c_int,
    pub irq: c_int,
    pub sfty_irq: c_int,
    pub sfty_ce_irq: c_int,
    pub sfty_ue_irq: c_int,
    pub rx_irq: [c_int; MTL_MAX_RX_QUEUES],
    pub tx_irq: [c_int; MTL_MAX_TX_QUEUES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmmac_txbuf_type {
    STMMAC_TXBUF_T_SKB,
    STMMAC_TXBUF_T_XDP_TX,
    STMMAC_TXBUF_T_XDP_NDO,
    STMMAC_TXBUF_T_XSK_TX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_tx_info {
    pub xsk_meta: xsk_tx_metadata_compl,
    pub buf: dma_addr_t,
    pub len: unsigned,
    pub buf_type: stmmac_txbuf_type,
    pub map_as_page: bool,
    pub last_segment: bool,
    pub is_jumbo: bool,
}

// Frequently used values are kept adjacent for cache effect
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_tx_queue {
    pub tx_count_frames: u32,
    pub tbs: c_int,
    pub txtimer: hrtimer,
    pub queue_index: u32,
    pub priv_data: *mut stmmac_priv,
    pub ____cacheline_aligned_in_smp: *mut *mut dma_extended_desc dma_etx,
    pub dma_entx: *mut dma_edesc,
    pub dma_tx: *mut dma_desc,
    pub tx_skbuff: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_rx_buffer {
    pub page: *mut page,
    pub addr: dma_addr_t,
    pub page_offset: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_xdp_buff {
    pub xdp: xdp_buff,
    pub priv: *mut stmmac_priv,
    pub desc: *mut dma_desc,
    pub ndesc: *mut dma_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_metadata_request {
    pub priv: *mut stmmac_priv,
    pub tx_desc: *mut dma_desc,
    pub set_ic: *mut bool,
    pub edesc: *mut dma_edesc,
    pub tbs: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_xsk_tx_complete {
    pub priv: *mut stmmac_priv,
    pub desc: *mut dma_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_rx_queue {
    pub rx_count_frames: u32,
    pub queue_index: u32,
    pub xdp_rxq: xdp_rxq_info,
    pub xsk_pool: *mut xsk_buff_pool,
    pub page_pool: *mut page_pool,
    pub buf_pool: *mut stmmac_rx_buffer,
    pub priv_data: *mut stmmac_priv,
    pub dma_erx: *mut dma_extended_desc,
    pub ____cacheline_aligned_in_smp: *mut *mut dma_desc dma_rx,
    pub cur_rx: c_uint,
    pub dirty_rx: c_uint,
    pub buf_alloc_num: c_uint,
    pub napi_skb_frag_size: c_uint,
    pub dma_rx_phy: dma_addr_t,
    pub state_saved: c_uint,
    pub skb: *mut sk_buff,
    pub len: c_uint,
    pub error: c_uint,
    pub state: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_channel {
    pub ____cacheline_aligned_in_smp: napi_rx_napi,
    pub ____cacheline_aligned_in_smp: napi_tx_napi,
    pub ____cacheline_aligned_in_smp: napi_rxtx_napi,
    pub priv_data: *mut stmmac_priv,
    pub lock: spinlock_t,
    pub index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_fpe_cfg {
    pub mmsv: ethtool_mmsv,
    pub reg: *const stmmac_fpe_reg,
    pub /: *mut *mut u32 fpe_csr; / MAC_FPE_CTRL_STS reg cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_tc_entry {
    pub in_use: bool,
    pub in_hw: bool,
    pub is_last: bool,
    pub is_frag: bool,
    pub frag_ptr: *mut c_void,
    pub table_pos: c_uint,
    pub handle: u32,
    pub prio: u32,
    pub match_data: u32,
    pub match_en: u32,
    pub af:1: u8,
    pub rf:1: u8,
    pub im:1: u8,
    pub nc:1: u8,
    pub res1:4: u8,
    pub frame_offset: u8,
    pub ok_index: u8,
    pub dma_ch_no: u8,
    pub res2: u32,
    pub val: } __packed,
}

pub const STMMAC_PPS_MAX: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_pps_cfg {
    pub start: timespec64,
    pub period: timespec64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_rss {
    pub enable: c_int,
    pub key: [u8; STMMAC_RSS_HASH_KEY_SIZE],
    pub table: [u32; STMMAC_RSS_MAX_TABLE_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_flow_entry {
    pub cookie: c_ulong,
    pub action: c_ulong,
    pub ip_proto: u8,
    pub in_use: c_int,
    pub idx: c_int,
    pub is_l4: c_int,
}

// Rx Frame Steering
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmmac_rfs_type {
    STMMAC_RFS_T_VLAN,
    STMMAC_RFS_T_LLDP,
    STMMAC_RFS_T_1588,
    STMMAC_RFS_T_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_rfs_entry {
    pub cookie: c_ulong,
    pub etype: u16,
    pub in_use: c_int,
    pub type: c_int,
    pub tc: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_dma_conf {
    pub dma_buf_sz: c_uint,
// RX Queue
    pub rx_queue: [stmmac_rx_queue; MTL_MAX_RX_QUEUES],
    pub dma_rx_size: c_uint,
// TX Queue
    pub tx_queue: [stmmac_tx_queue; MTL_MAX_TX_QUEUES],
    pub dma_tx_size: c_uint,
}

pub const EST_GCL: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_est {
    pub enable: c_int,
    pub btr_reserve: [u32; 2],
    pub btr_offset: [u32; 2],
    pub btr: [u32; 2],
    pub ctr: [u32; 2],
    pub ter: u32,
    pub gcl_unaligned: [u32; EST_GCL],
    pub gcl: [u32; EST_GCL],
    pub gcl_size: u32,
    pub max_sdu: [u32; MTL_MAX_TX_QUEUES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_msi {
    pub sfty_ce_irq: c_int,
    pub sfty_ue_irq: c_int,
    pub rx_irq: [c_int; MTL_MAX_RX_QUEUES],
    pub tx_irq: [c_int; MTL_MAX_TX_QUEUES],
// irq name
    pub 9]: char int_name_mac[IFNAMSIZ +,
    pub 9]: char int_name_wol[IFNAMSIZ +,
    pub 9]: char int_name_lpi[IFNAMSIZ +,
    pub 10]: char int_name_sfty[IFNAMSIZ +,
    pub 10]: char int_name_sfty_ce[IFNAMSIZ +,
    pub 10]: char int_name_sfty_ue[IFNAMSIZ +,
    pub 14]: char int_name_rx_irq[MTL_MAX_RX_QUEUES][IFNAMSIZ +,
    pub 18]: char int_name_tx_irq[MTL_MAX_TX_QUEUES][IFNAMSIZ +,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stmmac_priv {
// Frequently used values are kept adjacent for cache effect
    pub tx_coal_frames: [u32; MTL_MAX_TX_QUEUES],
    pub tx_coal_timer: [u32; MTL_MAX_TX_QUEUES],
    pub rx_coal_frames: [u32; MTL_MAX_RX_QUEUES],
    pub hwts_tx_en: c_int,
    pub tx_path_in_lpi_mode: bool,
    pub sph_active: bool,
    pub sph_capable: bool,
    pub sarc_type: u32,
    pub rx_riwt: [u32; MTL_MAX_RX_QUEUES],
    pub hwts_rx_en: c_int,
    pub tsfupdt_coarse: bool,
    pub ioaddr: *mut void __iomem,
    pub dev: *mut net_device,
    pub device: *mut device,
    pub hw: *mut mac_device_info,
    pub priv): *mut *mut int (hwif_quirks)(struct stmmac_priv,
    pub lock: mutex,
    pub dma_conf: stmmac_dma_conf,
// Generic channel for NAPI
    pub channel: [stmmac_channel; STMMAC_CH_MAX],
    pub pause_time: c_uint,
    pub mii: *mut mii_bus,
    pub integrated_pcs: *mut stmmac_pcs,
    pub phylink_config: phylink_config,
    pub phylink: *mut phylink,
    pub ____cacheline_aligned_in_smp: stmmac_extra_stats xstats,
    pub sstats: stmmac_safety_stats,
    pub plat: *mut plat_stmmacenet_data,
// Protect est parameters
    pub est_lock: mutex,
    pub est: *mut stmmac_est,
    pub dma_cap: dma_features,
    pub mmc: stmmac_counters,
    pub hw_cap_support: c_int,
    pub synopsys_id: c_int,
    pub msg_enable: u32,
// Our MAC Wake-on-Lan options
    pub wolopts: c_int,
    pub wol_irq: c_int,
    pub gmii_address_bus_config: u32,
    pub eee_ctrl_timer: timer_list,
    pub tx_lpi_timer: u32,
    pub tx_lpi_clk_stop: bool,
    pub eee_enabled: bool,
    pub eee_active: bool,
    pub eee_sw_timer_en: bool,
    pub legacy_serdes_is_powered: bool,
// descriptor format:
// when clear: struct dma_desc or for tx TBS struct dma_edesc
// when set, struct dma_extended_desc
//
    pub extend_desc: bool,
// chain_mode: requested descriptor mode
    pub chain_mode: bool,
// descriptor_mode: actual descriptor mode,
// see STMMAC_CHAIN_MODE or STMMAC_RING_MODE
//
    pub descriptor_mode: u8,
    pub tstamp_config: kernel_hwtstamp_config,
    pub ptp_clock: *mut ptp_clock,
    pub ptp_clock_ops: ptp_clock_info,
    pub default_addend: c_uint,
    pub sub_second_inc: u32,
    pub systime_flags: u32,
    pub adv_ts: u32,
    pub use_riwt: c_int,
    pub irq_wake: c_int,
    pub ptp_lock: rwlock_t,
// Protects auxiliary snapshot registers from concurrent access.
    pub aux_ts_lock: mutex,
    pub tstamp_busy_wait: wait_queue_head_t,
    pub mmcaddr: *mut void __iomem,
    pub ptpaddr: *mut void __iomem,
    pub estaddr: *mut void __iomem,
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub num_double_vlans: c_uint,
    pub sfty_irq: c_int,
    pub msi: *mut stmmac_msi,

    pub dbgfs_dir: *mut dentry,

    pub state: c_ulong,
    pub wq: *mut workqueue_struct,
    pub service_task: work_struct,
// Frame Preemption feature (FPE)
    pub fpe_cfg: stmmac_fpe_cfg,
// TC Handling
    pub tc_entries_max: c_uint,
    pub tc_off_max: c_uint,
    pub tc_entries: *mut stmmac_tc_entry,
    pub flow_entries_max: c_uint,
    pub flow_entries: *mut stmmac_flow_entry,
    pub rfs_entries_max: [c_uint; STMMAC_RFS_T_MAX],
    pub rfs_entries_cnt: [c_uint; STMMAC_RFS_T_MAX],
    pub rfs_entries_total: c_uint,
    pub rfs_entries: *mut stmmac_rfs_entry,
// Pulse Per Second output
    pub pps: [stmmac_pps_cfg; STMMAC_PPS_MAX],
// Receive Side Scaling
    pub rss: stmmac_rss,
// XDP BPF Program
    pub af_xdp_zc_qps: *mut c_ulong,
    pub xdp_prog: *mut bpf_prog,
    pub devlink: *mut devlink,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stmmac_state {
    STMMAC_DOWN,
    STMMAC_RESET_REQUESTED,
    STMMAC_RESETING,
    STMMAC_SERVICE_SCHED,
}

extern "C" {
    pub fn stmmac_mdio_unregister(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn stmmac_mdio_register(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn stmmac_mdio_reset(mii: *mut mii_bus) -> c_int;
}
extern "C" {
    pub fn stmmac_mdio_lock(priv: *mut stmmac_priv);
}
extern "C" {
    pub fn stmmac_mdio_unlock(priv: *mut stmmac_priv);
}
extern "C" {
    pub fn stmmac_pcs_setup(ndev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn stmmac_pcs_clean(ndev: *mut net_device);
}
extern "C" {
    pub fn stmmac_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn stmmac_ptp_register(priv: *mut stmmac_priv);
}
extern "C" {
    pub fn stmmac_ptp_unregister(priv: *mut stmmac_priv);
}
extern "C" {
    pub fn stmmac_xdp_open(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn stmmac_xdp_release(dev: *mut net_device);
}
extern "C" {
    pub fn stmmac_get_phy_intf_sel(interface: phy_interface_t) -> c_int;
}
extern "C" {
    pub fn stmmac_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn stmmac_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn stmmac_dvr_remove(dev: *mut device);
}
extern "C" {
    pub fn stmmac_reinit_queues(dev: *mut net_device, rx_cnt: u8, tx_cnt: u8) -> c_int;
}
extern "C" {
    pub fn stmmac_reinit_ringparam(dev: *mut net_device, rx_size: u32, tx_size: u32) -> c_int;
}
extern "C" {
    pub fn stmmac_disable_rx_queue(priv: *mut stmmac_priv, queue: u32);
}
extern "C" {
    pub fn stmmac_enable_rx_queue(priv: *mut stmmac_priv, queue: u32);
}
extern "C" {
    pub fn stmmac_disable_tx_queue(priv: *mut stmmac_priv, queue: u32);
}
extern "C" {
    pub fn stmmac_enable_tx_queue(priv: *mut stmmac_priv, queue: u32);
}
extern "C" {
    pub fn stmmac_xsk_wakeup(dev: *mut net_device, queue: u32, flags: u32) -> c_int;
}

extern "C" {
    pub fn stmmac_selftest_get_strings(priv: *mut stmmac_priv, data: *mut u8);
}
extern "C" {
    pub fn stmmac_selftest_get_count(priv: *mut stmmac_priv) -> c_int;
}

// Not enabled

