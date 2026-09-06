//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/sparx5/sparx5_main.h
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


// SPDX-License-Identifier: GPL-2.0+
// Microchip Sparx5 Switch driver
//
// Copyright (c) 2021 Microchip Technology Inc. and its subsidiaries.
//

// Target chip type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum spx5_target_chiptype {
    SPX5_TARGET_CT_7546       = 0x7546,  /* SparX-5-64  Enterprise */
    SPX5_TARGET_CT_7549       = 0x7549,  /* SparX-5-90  Enterprise */
    SPX5_TARGET_CT_7552       = 0x7552,  /* SparX-5-128 Enterprise */
    SPX5_TARGET_CT_7556       = 0x7556,  /* SparX-5-160 Enterprise */
    SPX5_TARGET_CT_7558       = 0x7558,  /* SparX-5-200 Enterprise */
    SPX5_TARGET_CT_7546TSN    = 0x0546,  /* SparX-5-64i Industrial */
    SPX5_TARGET_CT_7549TSN    = 0x0549,  /* SparX-5-90i Industrial */
    SPX5_TARGET_CT_7552TSN    = 0x0552,  /* SparX-5-128i Industrial */
    SPX5_TARGET_CT_7556TSN    = 0x0556,  /* SparX-5-160i Industrial */
    SPX5_TARGET_CT_7558TSN    = 0x0558,  /* SparX-5-200i Industrial */
    SPX5_TARGET_CT_LAN9694    = 0x9694,  /* lan969x-40 */
    SPX5_TARGET_CT_LAN9691VAO = 0x9691,  /* lan969x-40-VAO */
    SPX5_TARGET_CT_LAN9694TSN = 0x9695,  /* lan969x-40-TSN */
    SPX5_TARGET_CT_LAN9694RED = 0x969A,  /* lan969x-40-RED */
    SPX5_TARGET_CT_LAN9696    = 0x9696,  /* lan969x-60 */
    SPX5_TARGET_CT_LAN9692VAO = 0x9692,  /* lan969x-65-VAO */
    SPX5_TARGET_CT_LAN9696TSN = 0x9697,  /* lan969x-60-TSN */
    SPX5_TARGET_CT_LAN9696RED = 0x969B,  /* lan969x-60-RED */
    SPX5_TARGET_CT_LAN9698    = 0x9698,  /* lan969x-100 */
    SPX5_TARGET_CT_LAN9693VAO = 0x9693,  /* lan969x-100-VAO */
    SPX5_TARGET_CT_LAN9698TSN = 0x9699,  /* lan969x-100-TSN */
    SPX5_TARGET_CT_LAN9698RED = 0x969C,  /* lan969x-100-RED */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_port_max_tags {
    SPX5_PORT_MAX_TAGS_NONE,  /* No extra tags allowed */
    SPX5_PORT_MAX_TAGS_ONE,   /* Single tag allowed */
    SPX5_PORT_MAX_TAGS_TWO    /* Single and double tag allowed */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_vlan_port_type {
    SPX5_VLAN_PORT_TYPE_UNAWARE, /* VLAN unaware port */
    SPX5_VLAN_PORT_TYPE_C,       /* C-port */
    SPX5_VLAN_PORT_TYPE_S,       /* S-port */
    SPX5_VLAN_PORT_TYPE_S_CUSTOM /* S-port using custom type */
}

// This is used in calendar configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_cal_bw {
    SPX5_CAL_SPEED_NONE = 0,
    SPX5_CAL_SPEED_1G   = 1,
    SPX5_CAL_SPEED_2G5  = 2,
    SPX5_CAL_SPEED_5G   = 3,
    SPX5_CAL_SPEED_10G  = 4,
    SPX5_CAL_SPEED_25G  = 5,
    SPX5_CAL_SPEED_0G5  = 6,
    SPX5_CAL_SPEED_12G5 = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_feature {
    SPX5_FEATURE_PSFP = BIT(0),
    SPX5_FEATURE_PTP  = BIT(1),
}

pub const SPX5_PORTS: c_int = 65;

pub const PGID_UC_FLOOD: c_int = 0;
pub const PGID_MC_FLOOD: c_int = 1;
pub const PGID_IPV4_MC_DATA: c_int = 2;
pub const PGID_IPV4_MC_CTRL: c_int = 3;
pub const PGID_IPV6_MC_DATA: c_int = 4;
pub const PGID_IPV6_MC_CTRL: c_int = 5;
pub const PGID_BCAST: c_int = 6;
pub const PGID_CPU: c_int = 7;
pub const PGID_MCAST_START: c_int = 8;
pub const PGID_TABLE_SIZE: c_int = 3290;

pub const NULL_VID: c_int = 0;

pub const XTR_QUEUE: c_int = 0;
pub const INJ_QUEUE: c_int = 0;
pub const FDMA_XTR_CHANNEL: c_int = 6;
pub const FDMA_INJ_CHANNEL: c_int = 0;
pub const FDMA_DCB_MAX: c_int = 64;
pub const FDMA_RX_DCB_MAX_DBS: c_int = 15;
pub const FDMA_TX_DCB_MAX_DBS: c_int = 1;
pub const SPARX5_PHC_COUNT: c_int = 3;
pub const SPARX5_PHC_PORT: c_int = 0;
pub const IFH_REW_OP_NOOP: c_uint = 0x0;
pub const IFH_REW_OP_ONE_STEP_PTP: c_uint = 0x3;
pub const IFH_REW_OP_TWO_STEP_PTP: c_uint = 0x4;
pub const IFH_PDU_TYPE_NONE: c_uint = 0x0;
pub const IFH_PDU_TYPE_PTP: c_uint = 0x5;
pub const IFH_PDU_TYPE_IPV4_UDP_PTP: c_uint = 0x6;
pub const IFH_PDU_TYPE_IPV6_UDP_PTP: c_uint = 0x7;
pub const SPX5_DSM_CAL_LEN: c_int = 64;
pub const SPX5_DSM_CAL_MAX_DEVS_PER_TAXI: c_int = 13;
pub const SPX5_DSM_CAL_EMPTY: c_uint = 0xFFFF;
pub const SPARX5_MAX_PTP_ID: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_calendar_data {
    pub schedule: [u32; SPX5_DSM_CAL_LEN],
    pub avg_dist: [u32; SPX5_DSM_CAL_MAX_DEVS_PER_TAXI],
    pub taxi_ports: [u32; SPX5_DSM_CAL_MAX_DEVS_PER_TAXI],
    pub taxi_speeds: [u32; SPX5_DSM_CAL_MAX_DEVS_PER_TAXI],
    pub dev_slots: [u32; SPX5_DSM_CAL_MAX_DEVS_PER_TAXI],
    pub new_slots: [u32; SPX5_DSM_CAL_LEN],
    pub temp_sched: [u32; SPX5_DSM_CAL_LEN],
    pub indices: [u32; SPX5_DSM_CAL_LEN],
    pub short_list: [u32; SPX5_DSM_CAL_LEN],
    pub long_list: [u32; SPX5_DSM_CAL_LEN],
}

// Frame DMA receive state:
// For each DB, there is a SKB, and the skb data pointer is mapped in
// the DB. Once a frame is received the skb is given to the upper layers
// and a new skb is added to the dcb.
// When the db_index reached FDMA_RX_DCB_MAX_DBS the DB is reused.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_rx {
    pub fdma: fdma,
    pub page_pool: *mut page_pool,
    pub skb: [*mut sk_buff; FDMA_DCB_MAX][FDMA_RX_DCB_MAX_DBS],
    pub page: [*mut page; FDMA_DCB_MAX][FDMA_RX_DCB_MAX_DBS],
}

// Used to store information about TX buffers.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_tx_buf {
    pub dev: *mut net_device,
    pub skb: *mut sk_buff,
    pub dma_addr: dma_addr_t,
    pub used: bool,
    pub ptp: bool,
}

// Frame DMA transmit state:
// DCBs are chained using the DCBs nextptr field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_tx {
    pub fdma: fdma,
    pub dbs: *mut sparx5_tx_buf,
    pub packets: u64,
    pub dropped: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port_config {
    pub portmode: phy_interface_t,
    pub bandwidth: u32,
    pub speed: c_int,
    pub duplex: c_int,
    pub media: phy_media,
    pub inband: bool,
    pub power_down: bool,
    pub autoneg: bool,
    pub serdes_reset: bool,
    pub pause: u32,
    pub pause_adv: u32,
    pub phy_mode: phy_interface_t,
    pub sd_sgpio: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_port {
    pub ndev: *mut net_device,
    pub sparx5: *mut sparx5,
    pub of_node: *mut device_node,
    pub serdes: *mut phy,
    pub conf: sparx5_port_config,
    pub phylink_config: phylink_config,
    pub phylink: *mut phylink,
    pub phylink_pcs: phylink_pcs,
    pub mirror_stats: flow_stats,
    pub portno: u16,
// Ingress default VLAN (pvid)
    pub pvid: u16,
// Egress default VLAN (vid)
    pub vid: u16,
    pub signd_internal: bool,
    pub signd_active_high: bool,
    pub signd_enable: bool,
    pub flow_control: bool,
    pub max_vlan_tags: sparx5_port_max_tags,
    pub vlan_type: sparx5_vlan_port_type,
    pub custom_etype: u32,
    pub vlan_aware: bool,
    pub inj_timer: hrtimer,
// ptp
    pub ptp_cmd: u8,
    pub ts_id: u16,
    pub tx_skbs: sk_buff_head,
    pub is_mrouter: bool,
    pub /: *mut *mut list_head tc_templates; / list of TC templates on this port,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_core_clockfreq {
    SPX5_CORE_CLOCK_DEFAULT,  /* Defaults to the highest supported frequency */
    SPX5_CORE_CLOCK_250MHZ,   /* 250MHZ core clock frequency */
    SPX5_CORE_CLOCK_328MHZ,   /* 328MHZ core clock frequency */
    SPX5_CORE_CLOCK_500MHZ,   /* 500MHZ core clock frequency */
    SPX5_CORE_CLOCK_625MHZ,   /* 625MHZ core clock frequency */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_phc {
    pub clock: *mut ptp_clock,
    pub info: ptp_clock_info,
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub sparx5: *mut sparx5,
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_skb_cb {
    pub rew_op: u8,
    pub pdu_type: u8,
    pub pdu_w16_offset: u8,
    pub ts_id: u16,
    pub jiffies: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_mdb_entry {
    pub list: list_head,
    pub SPX5_PORTS): DECLARE_BITMAP(port_mask,,
    pub addr: [c_uchar; ETH_ALEN],
    pub cpu_copy: bool,
    pub vid: u16,
    pub pgid_idx: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_mall_mirror_entry {
    pub idx: u32,
    pub port: *mut sparx5_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_mall_entry {
    pub list: list_head,
    pub port: *mut sparx5_port,
    pub cookie: c_ulong,
    pub type: flow_action_id,
    pub ingress: bool,
    pub mirror: sparx5_mall_mirror_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_regs {
    pub tsize: *const c_uint,
    pub gaddr: *const c_uint,
    pub gcnt: *const c_uint,
    pub gsize: *const c_uint,
    pub raddr: *const c_uint,
    pub rcnt: *const c_uint,
    pub fpos: *const c_uint,
    pub fsize: *const c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_consts {
    pub /: *mut *mut u32 n_ports; / Number of front ports,
    pub /: *mut *mut u32 n_ports_all; / Number of front ports + internal ports,
    pub /: *mut *mut u32 n_hsch_l1_elems; / Number of HSCH layer 1 elements,
    pub /: *mut *mut u32 n_hsch_queues; / Number of HSCH queues,
    pub /: *mut *mut u32 n_lb_groups; / Number of leacky bucket groupd,
    pub /: *mut *mut u32 n_pgids; / Number of PGID's,
    pub /: *mut *mut u32 n_sio_clks; / Number of serial IO clocks,
    pub /: *mut *mut u32 n_own_upsids; / Number of own UPSID's,
    pub /: *mut *mut u32 n_auto_cals; / Number of auto calendars,
    pub /: *mut *mut u32 n_filters; / Number of PSFP filters,
    pub /: *mut *mut u32 n_gates; / Number of PSFP gates,
    pub /: *mut *mut u32 n_sdlbs; / Number of service dual leaky buckets,
    pub /: *mut *mut u32 n_dsm_cal_taxis; / Number of DSM calendar taxis,
    pub /: *mut *mut u32 buf_size; / Amount of QLIM watermark memory,
    pub /: *mut *mut u32 qres_max_prio_idx; / Maximum QRES prio index,
    pub /: *mut *mut u32 qres_max_colour_idx; / Maximum QRES colour index,
    pub /: *mut *mut u32 tod_pin; / PTP TOD pin,
    pub vcaps_cfg: *const sparx5_vcap_inst,
    pub vcaps: *const vcap_info,
    pub vcap_stats: *const vcap_statistics,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_ops {
    pub portno): *mut *mut bool (is_port_2g5)(int,
    pub portno): *mut *mut bool (is_port_5g)(int,
    pub portno): *mut *mut bool (is_port_10g)(int,
    pub portno): *mut *mut bool (is_port_25g)(int,
    pub portno): *mut *mut bool (is_port_rgmii)(int,
    pub port): *mut *mut *mut u32 (get_port_dev_index)(struct sparx5 sparx5, int,
    pub port): *mut *mut *mut u32 (get_port_dev_bit)(struct sparx5 sparx5, int,
    pub grp): *mut *mut u32 (get_hsch_max_group_rate)(int,
    pub idx): *mut *mut *mut sparx5_sdlb_group (get_sdlb_group)(int,
    pub conf): *mut sparx5_port_config,
    pub args): *mut *mut irqreturn_t (ptp_irq_handler)(int irq, void,
    pub data): *mut sparx5_calendar_data,
    pub conf): *mut sparx5_port_config,
    pub sparx5): *mut *mut int (fdma_init)(struct sparx5,
    pub sparx5): *mut *mut int (fdma_deinit)(struct sparx5,
    pub weight): *mut *mut *mut int (fdma_poll)(struct napi_struct napi, int,
    pub dev): *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_main_io_resource {
    pub id: sparx5_target,
    pub offset: phys_addr_t,
    pub range: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_match_data {
    pub regs: *const sparx5_regs,
    pub consts: *const sparx5_consts,
    pub ops: *const sparx5_ops,
    pub iomap: *const sparx5_main_io_resource,
    pub ioranges: c_int,
    pub iomap_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5 {
    pub pdev: *mut platform_device,
    pub dev: *mut device,
    pub chip_id: u32,
    pub target_ct: spx5_target_chiptype,
    pub features: u32,
    pub regs: [*mut void __iomem; NUM_TARGETS],
    pub port_count: c_int,
    pub /: *mut *mut mutex lock; / MAC reg lock,
// port structures are in net device
    pub ports: [*mut sparx5_port; SPX5_PORTS],
    pub coreclock: sparx5_core_clockfreq,
// Statistics
    pub num_stats: u32,
    pub num_ethtool_stats: u32,
    pub stats_layout: *const *const c_char,
    pub stats: *mut u64,
// Workqueue for reading stats
    pub queue_stats_lock: mutex,
    pub stats_work: delayed_work,
    pub stats_queue: *mut workqueue_struct,
// Notifiers
    pub netdevice_nb: notifier_block,
    pub switchdev_nb: notifier_block,
    pub switchdev_blocking_nb: notifier_block,
// Switch state
    pub base_mac: [u8; ETH_ALEN],
// Associated bridge device (when bridged)
    pub hw_bridge_dev: *mut net_device,
// Bridged interfaces
    pub SPX5_PORTS): DECLARE_BITMAP(bridge_mask,,
    pub SPX5_PORTS): DECLARE_BITMAP(bridge_fwd_mask,,
    pub SPX5_PORTS): DECLARE_BITMAP(bridge_lrn_mask,,
    pub SPX5_PORTS): DECLARE_BITMAP(vlan_mask[VLAN_N_VID],,
// SW MAC table
    pub mact_entries: list_head,
// mac table list (mact_entries) mutex
    pub mact_lock: mutex,
// SW MDB table
    pub mdb_entries: list_head,
// mdb list mutex
    pub mdb_lock: mutex,
    pub mact_work: delayed_work,
    pub mact_queue: *mut workqueue_struct,
// Board specifics
    pub sd_sgpio_remapping: bool,
// Register based inj/xtr
    pub xtr_irq: c_int,
// Frame DMA
    pub fdma_irq: c_int,
    pub /: *mut *mut spinlock_t tx_lock; / lock for frame transmission,
    pub rx: sparx5_rx,
    pub tx: sparx5_tx,
// PTP
    pub ptp: bool,
    pub phc: [sparx5_phc; SPARX5_PHC_COUNT],
    pub /: *mut *mut spinlock_t ptp_clock_lock; / lock for phc,
    pub /: *mut *mut spinlock_t ptp_ts_id_lock; / lock for ts_id,
    pub /: *mut *mut mutex ptp_lock; / lock for ptp interface state,
    pub ptp_skbs: u16,
    pub ptp_irq: c_int,
// VCAP
    pub vcap_ctrl: *mut vcap_control,
// PGID allocation map
    pub pgid_map: [u8; PGID_TABLE_SIZE],
    pub mall_entries: list_head,
// Common root for debugfs
    pub debugfs_root: *mut dentry,
    pub data: *const sparx5_match_data,
}

// sparx5_main.c
extern "C" {
    pub fn is_sparx5(sparx5: *mut sparx5) -> bool;
}
extern "C" {
    pub fn sparx5_has_feature(sparx5: *mut sparx5, feature: sparx5_feature) -> bool;
}
// sparx5_switchdev.c
extern "C" {
    pub fn sparx5_register_notifier_blocks(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_unregister_notifier_blocks(sparx5: *mut sparx5);
}
// sparx5_packet.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frame_info {
    pub src_port: c_int,
    pub timestamp: u32,
}

extern "C" {
    pub fn sparx5_xtr_flush(sparx5: *mut sparx5, grp: u8);
}
extern "C" {
    pub fn sparx5_ifh_parse(sparx5: *mut sparx5, ifh: *mut u32, info: *mut frame_info);
}
extern "C" {
    pub fn sparx5_xtr_handler(irq: c_int, _priv: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn sparx5_port_xmit_impl(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn sparx5_manual_injection_mode(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_port_inj_timer_setup(port: *mut sparx5_port);
}
// sparx5_fdma.c
extern "C" {
    pub fn sparx5_fdma_init(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_fdma_deinit(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_fdma_start(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_fdma_stop(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_fdma_napi_callback(napi: *mut napi_struct, weight: c_int) -> c_int;
}
extern "C" {
    pub fn sparx5_fdma_handler(irq: c_int, args: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn sparx5_fdma_reload(sparx5: *mut sparx5, fdma: *mut fdma);
}
extern "C" {
    pub fn sparx5_fdma_injection_mode(sparx5: *mut sparx5);
}
// sparx5_mactable.c
extern "C" {
    pub fn sparx5_mc_sync(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn sparx5_mc_unsync(dev: *mut net_device, addr: *const c_uchar) -> c_int;
}
extern "C" {
    pub fn sparx5_set_ageing(sparx5: *mut sparx5, msecs: c_int);
}
extern "C" {
    pub fn sparx5_mact_init(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_mact_deinit(sparx5: *mut sparx5);
}
// sparx5_vlan.c
extern "C" {
    pub fn sparx5_pgid_update_mask(port: *mut sparx5_port, pgid: c_int, enable: bool);
}
extern "C" {
    pub fn sparx5_pgid_clear(spx5: *mut sparx5, pgid: c_int);
}
extern "C" {
    pub fn sparx5_pgid_read_mask(sparx5: *mut sparx5, pgid: c_int, portmask[3]: u32);
}
extern "C" {
    pub fn sparx5_update_fwd(sparx5: *mut sparx5);
}
extern "C" {
    pub fn sparx5_vlan_init(sparx5: *mut sparx5);
}
extern "C" {
    pub fn sparx5_vlan_port_setup(sparx5: *mut sparx5, portno: c_int);
}
extern "C" {
    pub fn sparx5_vlan_vid_del(port: *mut sparx5_port, vid: u16) -> c_int;
}
extern "C" {
    pub fn sparx5_vlan_port_apply(sparx5: *mut sparx5, port: *mut sparx5_port);
}
// sparx5_calendar.c
extern "C" {
    pub fn sparx5_calendar_init(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_cal_speed_to_value(speed: sparx5_cal_bw) -> u32;
}
extern "C" {
    pub fn sparx5_get_port_cal_speed(sparx5: *mut sparx5, portno: u32) -> sparx5_cal_bw;
}
// sparx5_ethtool.c
extern "C" {
    pub fn sparx5_get_stats64(ndev: *mut net_device, stats: *mut rtnl_link_stats64);
}
extern "C" {
    pub fn sparx5_stats_init(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_stats_deinit(sparx5: *mut sparx5);
}
// sparx5_dcb.c

extern "C" {
    pub fn sparx5_dcb_init(sparx5: *mut sparx5) -> c_int;
}

// sparx5_netdev.c
extern "C" {
    pub fn sparx5_set_port_ifh_rew_op(ifh_hdr: *mut c_void, rew_op: u32);
}
extern "C" {
    pub fn sparx5_set_port_ifh(sparx5: *mut sparx5, ifh_hdr: *mut c_void, portno: u16);
}
extern "C" {
    pub fn sparx5_netdevice_check(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn sparx5_register_netdevs(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_destroy_netdevs(sparx5: *mut sparx5);
}
extern "C" {
    pub fn sparx5_unregister_netdevs(sparx5: *mut sparx5);
}
// sparx5_ptp.c
extern "C" {
    pub fn sparx5_ptp_init(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_ptp_deinit(sparx5: *mut sparx5);
}
extern "C" {
    pub fn sparx5_ptp_irq_handler(irq: c_int, args: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn sparx5_ptp_gettime64(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int;
}
// sparx5_vcap_impl.c
extern "C" {
    pub fn sparx5_vcap_init(sparx5: *mut sparx5) -> c_int;
}
extern "C" {
    pub fn sparx5_vcap_deinit(sparx5: *mut sparx5);
}
// sparx5_pgid.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sparx5_pgid_type {
    SPX5_PGID_FREE,
    SPX5_PGID_RESERVED,
    SPX5_PGID_MULTICAST,
}

extern "C" {
    pub fn sparx5_pgid_init(spx5: *mut sparx5);
}
extern "C" {
    pub fn sparx5_pgid_alloc_mcast(spx5: *mut sparx5, idx: *mut u16) -> c_int;
}
extern "C" {
    pub fn sparx5_pgid_free(spx5: *mut sparx5, idx: u16) -> c_int;
}
extern "C" {
    pub fn sparx5_get_pgid(sparx5: *mut sparx5, pgid: c_int) -> c_int;
}
// sparx5_pool.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_pool_entry {
    pub ref_cnt: u16,
    pub /: *mut *mut u32 idx; / tc index,
}

extern "C" {
    pub fn sparx5_pool_idx_to_id(idx: u32) -> u32;
}
extern "C" {
    pub fn sparx5_pool_put(pool: *mut sparx5_pool_entry, size: c_int, id: u32) -> c_int;
}
extern "C" {
    pub fn sparx5_pool_get(pool: *mut sparx5_pool_entry, size: c_int, id: *mut u32) -> c_int;
}
// sparx5_port.c
extern "C" {
    pub fn sparx5_get_internal_port(sparx5: *mut sparx5, port: c_int) -> c_int;
}
// sparx5_sdlb.c
pub const SPX5_SDLB_PUP_TOKEN_DISABLE: c_uint = 0x1FFF;

pub const SPX5_SDLB_2CYCLES_TYPE2_THRES_OFFSET: c_int = 13;
pub const SPX5_SDLB_CNT: c_int = 4096;
pub const SPX5_SDLB_GROUP_CNT: c_int = 10;
pub const SPX5_CLK_PER_100PS_DEFAULT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_sdlb_group {
    pub max_rate: u64,
    pub min_burst: u32,
    pub frame_size: u32,
    pub pup_interval: u32,
    pub nsets: u32,
}

extern "C" {
    pub fn sparx5_sdlb_clk_hz_get(sparx5: *mut sparx5) -> u64;
}
extern "C" {
    pub fn sparx5_sdlb_group_get_by_rate(sparx5: *mut sparx5, rate: u32, burst: u32) -> c_int;
}
extern "C" {
    pub fn sparx5_sdlb_group_get_by_index(sparx5: *mut sparx5, idx: u32, group: *mut u32) -> c_int;
}
extern "C" {
    pub fn sparx5_sdlb_group_add(sparx5: *mut sparx5, group: u32, idx: u32) -> c_int;
}
extern "C" {
    pub fn sparx5_sdlb_group_del(sparx5: *mut sparx5, group: u32, idx: u32) -> c_int;
}
// sparx5_police.c
// More policer types will be added later
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_policer {
    pub type: u32,
    pub idx: u32,
    pub rate: u64,
    pub burst: u32,
    pub group: u32,
    pub event_mask: u8,
}

extern "C" {
    pub fn sparx5_policer_conf_set(sparx5: *mut sparx5, pol: *mut sparx5_policer) -> c_int;
}
// sparx5_psfp.c
pub const SPX5_PSFP_GCE_CNT: c_int = 4;
pub const SPX5_PSFP_SG_CNT: c_int = 1024;

pub const SPX5_PSFP_SG_CYCLE_TIME_DEFAULT: c_int = 1000000;
pub const SPX5_PSFP_SF_MAX_SDU: c_int = 16383;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_psfp_fm {
    pub pol: sparx5_policer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_psfp_gce {
    pub /: *mut *mut bool gate_state; / StreamGateState,
    pub /: *mut *mut u32 interval; / TimeInterval,
    pub /: *mut *mut u32 ipv; / InternalPriorityValue,
    pub /: *mut *mut u32 maxoctets; / IntervalOctetMax,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_psfp_sg {
    pub /: *mut *mut bool gate_state; / PSFPAdminGateStates,
    pub /: *mut *mut bool gate_enabled; / PSFPGateEnabled,
    pub /: *mut *mut u32 ipv; / PSFPAdminIPV,
    pub /: *mut *mut timespec64 basetime; / PSFPAdminBaseTime,
    pub /: *mut *mut u32 cycletime; / PSFPAdminCycleTime,
    pub /: *mut *mut u32 cycletimeext; / PSFPAdminCycleTimeExtension,
    pub /: *mut *mut u32 num_entries; / PSFPAdminControlListLength,
    pub gce: [sparx5_psfp_gce; SPX5_PSFP_GCE_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sparx5_psfp_sf {
    pub sblock_osize_ena: bool,
    pub sblock_osize: bool,
    pub max_sdu: u32,
    pub /: *mut *mut u32 sgid; / Gate id,
    pub /: *mut *mut u32 fmid; / Flow meter id,
}

extern "C" {
    pub fn sparx5_psfp_fm_del(sparx5: *mut sparx5, id: u32) -> c_int;
}
extern "C" {
    pub fn sparx5_psfp_sg_del(sparx5: *mut sparx5, id: u32) -> c_int;
}
extern "C" {
    pub fn sparx5_psfp_sf_del(sparx5: *mut sparx5, id: u32) -> c_int;
}
extern "C" {
    pub fn sparx5_psfp_isdx_get_sf(sparx5: *mut sparx5, isdx: u32) -> u32;
}
extern "C" {
    pub fn sparx5_psfp_isdx_get_fm(sparx5: *mut sparx5, isdx: u32) -> u32;
}
extern "C" {
    pub fn sparx5_psfp_sf_get_sg(sparx5: *mut sparx5, sfid: u32) -> u32;
}
extern "C" {
    pub fn sparx5_isdx_conf_set(sparx5: *mut sparx5, isdx: u32, sfid: u32, fmid: u32);
}
extern "C" {
    pub fn sparx5_psfp_init(sparx5: *mut sparx5);
}
// sparx5_qos.c
// sparx5_mirror.c
extern "C" {
    pub fn sparx5_mirror_add(entry: *mut sparx5_mall_entry) -> c_int;
}
extern "C" {
    pub fn sparx5_mirror_del(entry: *mut sparx5_mall_entry);
}
// Clock period in picoseconds
// Calculate raw offset
// Read, Write and modify registers content.
// The register definition macros start at the id
//
