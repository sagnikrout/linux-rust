//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_main.h
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

pub const TABLE_UPDATE_SLEEP_US: c_int = 10;
pub const TABLE_UPDATE_TIMEOUT_US: c_int = 100000;
pub const READL_SLEEP_US: c_int = 10;
pub const READL_TIMEOUT_US: c_int = 100000000;
pub const LAN966X_BUFFER_CELL_SZ: c_int = 64;

pub const LAN966X_BUFFER_MIN_SZ: c_int = 60;

pub const PGID_AGGR: c_int = 64;
pub const PGID_SRC: c_int = 80;
pub const PGID_ENTRIES: c_int = 89;
pub const UNAWARE_PVID: c_int = 0;
pub const HOST_PVID: c_int = 4095;
// Reserved amount for (SRC, PRIO) at index 8*SRC + PRIO
pub const QSYS_Q_RSRV: c_int = 95;
pub const NUM_PHYS_PORTS: c_int = 8;
pub const CPU_PORT: c_int = 8;
pub const NUM_PRIO_QUEUES: c_int = 8;
// Reserved PGIDs

// Non-reserved PGIDs, used for general purpose

pub const LAN966X_SPEED_NONE: c_int = 0;
pub const LAN966X_SPEED_2500: c_int = 1;
pub const LAN966X_SPEED_1000: c_int = 1;
pub const LAN966X_SPEED_100: c_int = 2;
pub const LAN966X_SPEED_10: c_int = 3;
pub const LAN966X_PHC_COUNT: c_int = 3;
pub const LAN966X_PHC_PORT: c_int = 0;
pub const LAN966X_PHC_PINS_NUM: c_int = 7;
pub const IFH_REW_OP_NOOP: c_uint = 0x0;
pub const IFH_REW_OP_ONE_STEP_PTP: c_uint = 0x3;
pub const IFH_REW_OP_TWO_STEP_PTP: c_uint = 0x4;
pub const IFH_PDU_TYPE_NONE: c_int = 0;
pub const IFH_PDU_TYPE_IPV4: c_int = 7;
pub const IFH_PDU_TYPE_IPV6: c_int = 8;
pub const FDMA_RX_DCB_MAX_DBS: c_int = 1;
pub const FDMA_TX_DCB_MAX_DBS: c_int = 1;
pub const FDMA_XTR_CHANNEL: c_int = 6;
pub const FDMA_INJ_CHANNEL: c_int = 0;
pub const FDMA_DCB_MAX: c_int = 512;

pub const LAN966X_PORT_QOS_PCP_COUNT: c_int = 8;
pub const LAN966X_PORT_QOS_DEI_COUNT: c_int = 8;

pub const LAN966X_PORT_QOS_DSCP_COUNT: c_int = 64;
// Port PCP rewrite mode
pub const LAN966X_PORT_REW_TAG_CTRL_CLASSIFIED: c_int = 0;
pub const LAN966X_PORT_REW_TAG_CTRL_MAPPED: c_int = 2;
// Port DSCP rewrite mode
pub const LAN966X_PORT_REW_DSCP_FRAME: c_int = 0;
pub const LAN966X_PORT_REW_DSCP_ANALIZER: c_int = 1;
pub const LAN966X_PORT_QOS_REWR_DSCP_ALL: c_int = 3;
// MAC table entry types.
// ENTRYTYPE_NORMAL is subject to aging.
// ENTRYTYPE_LOCKED is not subject to aging.
// ENTRYTYPE_MACv4 is not subject to aging. For IPv4 multicast.
// ENTRYTYPE_MACv6 is not subject to aging. For IPv6 multicast.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum macaccess_entry_type {
    ENTRYTYPE_NORMAL = 0,
    ENTRYTYPE_LOCKED,
    ENTRYTYPE_MACV4,
    ENTRYTYPE_MACV6,
}

// FDMA return action codes for checking if the frame is valid
// FDMA_PASS, frame is valid and can be used
// FDMA_ERROR, something went wrong, stop getting more frames
// FDMA_DROP, frame is dropped, but continue to get more frames
// FDMA_TX, frame is given to TX, but continue to get more frames
// FDMA_REDIRECT, frame is given to TX, but continue to get more frames
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lan966x_fdma_action {
    FDMA_PASS = 0,
    FDMA_ERROR,
    FDMA_DROP,
    FDMA_TX,
    FDMA_REDIRECT,
}

// Controls how PORT_MASK is applied
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LAN966X_PORT_MASK_MODE {
    LAN966X_PMM_NO_ACTION,
    LAN966X_PMM_REPLACE,
    LAN966X_PMM_FORWARDING,
    LAN966X_PMM_REDIRECT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is2_port_sel_ipv6 {
    VCAP_IS2_PS_IPV6_TCPUDP_OTHER,
    VCAP_IS2_PS_IPV6_STD,
    VCAP_IS2_PS_IPV6_IP4_TCPUDP_IP4_OTHER,
    VCAP_IS2_PS_IPV6_MAC_ETYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is1_port_sel_other {
    VCAP_IS1_PS_OTHER_NORMAL,
    VCAP_IS1_PS_OTHER_7TUPLE,
    VCAP_IS1_PS_OTHER_DBL_VID,
    VCAP_IS1_PS_OTHER_DMAC_VID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is1_port_sel_ipv4 {
    VCAP_IS1_PS_IPV4_NORMAL,
    VCAP_IS1_PS_IPV4_7TUPLE,
    VCAP_IS1_PS_IPV4_5TUPLE_IP4,
    VCAP_IS1_PS_IPV4_DBL_VID,
    VCAP_IS1_PS_IPV4_DMAC_VID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is1_port_sel_ipv6 {
    VCAP_IS1_PS_IPV6_NORMAL,
    VCAP_IS1_PS_IPV6_7TUPLE,
    VCAP_IS1_PS_IPV6_5TUPLE_IP4,
    VCAP_IS1_PS_IPV6_NORMAL_IP6,
    VCAP_IS1_PS_IPV6_5TUPLE_IP6,
    VCAP_IS1_PS_IPV6_DBL_VID,
    VCAP_IS1_PS_IPV6_DMAC_VID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vcap_is1_port_sel_rt {
    VCAP_IS1_PS_RT_NORMAL,
    VCAP_IS1_PS_RT_7TUPLE,
    VCAP_IS1_PS_RT_DBL_VID,
    VCAP_IS1_PS_RT_DMAC_VID,
    VCAP_IS1_PS_RT_FOLLOW_OTHER = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_rx {
    pub lan966x: *mut lan966x,
    pub fdma: fdma,
// For each DB, there is a page
    pub page: [*mut page; FDMA_DCB_MAX][FDMA_RX_DCB_MAX_DBS],
// Represents the page order that is used to allocate the pages for the
// RX buffers. This value is calculated based on max MTU of the devices.
//
    pub page_order: u8,
// Represents the max size frame that it can receive to the CPU. This
// includes the IFH + VLAN tags + frame + skb_shared_info
//
    pub max_mtu: u32,
    pub page_pool: *mut page_pool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_tx_dcb_buf {
    pub dma_addr: dma_addr_t,
    pub dev: *mut net_device,
    pub skb: *mut sk_buff,
    pub xdpf: *mut xdp_frame,
    pub page: *mut page,
    pub data: },
    pub len: u32,
    pub 1: u32 used :,
    pub 1: u32 ptp :,
    pub 1: u32 use_skb :,
    pub 1: u32 xdp_ndo :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_tx {
    pub lan966x: *mut lan966x,
    pub fdma: fdma,
// Array of dcbs that are given to the HW
    pub dcbs_buf: *mut lan966x_tx_dcb_buf,
    pub activated: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_stat_layout {
    pub offset: u32,
    pub name: [c_char; ETH_GSTRING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_phc {
    pub clock: *mut ptp_clock,
    pub info: ptp_clock_info,
    pub pins: [ptp_pin_desc; LAN966X_PHC_PINS_NUM],
    pub hwtstamp_config: kernel_hwtstamp_config,
    pub lan966x: *mut lan966x,
    pub index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_skb_cb {
    pub rew_op: u8,
    pub pdu_type: u8,
    pub ts_id: u16,
    pub jiffies: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x {
    pub dev: *mut device,
    pub num_phys_ports: u8,
    pub ports: *mut lan966x_port,
    pub regs: [*mut void __iomem; NUM_TARGETS],
    pub shared_queue_sz: c_int,
    pub base_mac: [u8; ETH_ALEN],
    pub /: *mut *mut spinlock_t tx_lock; / lock for frame transmission,
    pub bridge: *mut net_device,
    pub bridge_mask: u16,
    pub bridge_fwd_mask: u16,
    pub mac_entries: list_head,
    pub /: *mut *mut spinlock_t mac_lock; / lock for mac_entries list,
    pub vlan_mask: [u16; VLAN_N_VID],
    pub VLAN_N_VID): DECLARE_BITMAP(cpu_vlan_mask,,
// stats
    pub stats_layout: *const lan966x_stat_layout,
    pub num_stats: u32,
// lock for reading stats
    pub stats_lock: spinlock_t,
    pub stats: *mut u64,
    pub stats_work: delayed_work,
    pub stats_queue: *mut workqueue_struct,
// interrupts
    pub xtr_irq: c_int,
    pub ana_irq: c_int,
    pub ptp_irq: c_int,
    pub fdma_irq: c_int,
    pub ptp_ext_irq: c_int,
// worqueue for fdb
    pub fdb_work: *mut workqueue_struct,
    pub fdb_entries: list_head,
// mdb
    pub mdb_entries: list_head,
    pub pgid_entries: list_head,
// ptp
    pub ptp: bool,
    pub phc: [lan966x_phc; LAN966X_PHC_COUNT],
    pub /: *mut *mut spinlock_t ptp_clock_lock; / lock for phc,
    pub /: *mut *mut spinlock_t ptp_ts_id_lock; / lock for ts_id,
    pub /: *mut *mut mutex ptp_lock; / lock for ptp interface state,
    pub ptp_skbs: u16,
// fdma
    pub fdma: bool,
    pub fdma_ndev: *mut net_device,
    pub rx: lan966x_rx,
    pub tx: lan966x_tx,
    pub napi: napi_struct,
// Mirror
    pub mirror_monitor: *mut lan966x_port,
    pub mirror_mask: [u32; 2],
    pub mirror_count: u32,
// vcap
    pub vcap_ctrl: *mut vcap_control,
// debugfs
    pub debugfs_root: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port_config {
    pub portmode: phy_interface_t,
    pub advertising: *const c_ulong,
    pub speed: c_int,
    pub duplex: c_int,
    pub pause: u32,
    pub inband: bool,
    pub autoneg: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port_tc {
    pub ingress_shared_block: bool,
    pub police_id: c_ulong,
    pub ingress_mirror_id: c_ulong,
    pub egress_mirror_id: c_ulong,
    pub police_stat: flow_stats,
    pub mirror_stat: flow_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port_qos_pcp {
    pub map: [u8; LAN966X_PORT_QOS_PCP_DEI_COUNT],
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port_qos_dscp {
    pub map: [u8; LAN966X_PORT_QOS_DSCP_COUNT],
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port_qos_pcp_rewr {
    pub map: [u16; NUM_PRIO_QUEUES],
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port_qos_dscp_rewr {
    pub map: [u16; LAN966X_PORT_QOS_DSCP_COUNT],
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port_qos {
    pub pcp: lan966x_port_qos_pcp,
    pub dscp: lan966x_port_qos_dscp,
    pub pcp_rewr: lan966x_port_qos_pcp_rewr,
    pub dscp_rewr: lan966x_port_qos_dscp_rewr,
    pub default_prio: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lan966x_port {
    pub dev: *mut net_device,
    pub lan966x: *mut lan966x,
    pub chip_port: u8,
    pub pvid: u16,
    pub vid: u16,
    pub vlan_aware: bool,
    pub learn_ena: bool,
    pub mcast_ena: bool,
    pub phylink_config: phylink_config,
    pub phylink_pcs: phylink_pcs,
    pub config: lan966x_port_config,
    pub phylink: *mut phylink,
    pub serdes: *mut phy,
    pub fwnode: *mut fwnode_handle,
    pub ptp_tx_cmd: u8,
    pub ptp_rx_cmd: bool,
    pub ts_id: u16,
    pub tx_skbs: sk_buff_head,
    pub bond: *mut net_device,
    pub lag_tx_active: bool,
    pub hash_type: netdev_lag_hash,
    pub tc: lan966x_port_tc,
    pub xdp_prog: *mut bpf_prog,
    pub xdp_rxq: xdp_rxq_info,
}

extern "C" {
    pub fn lan966x_netdevice_check(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn lan966x_register_notifier_blocks();
}
extern "C" {
    pub fn lan966x_unregister_notifier_blocks();
}
extern "C" {
    pub fn lan966x_hw_offload(lan966x: *mut lan966x, port: u32, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn lan966x_ifh_get_src_port(ifh: *mut c_void, src_port: *mut u64);
}
extern "C" {
    pub fn lan966x_ifh_get_timestamp(ifh: *mut c_void, timestamp: *mut u64);
}
extern "C" {
    pub fn lan966x_ifh_set_bypass(ifh: *mut c_void, bypass: u64);
}
extern "C" {
    pub fn lan966x_ifh_set_port(ifh: *mut c_void, bypass: u64);
}
extern "C" {
    pub fn lan966x_stats_init(lan966x: *mut lan966x) -> c_int;
}
extern "C" {
    pub fn lan966x_port_config_down(port: *mut lan966x_port);
}
extern "C" {
    pub fn lan966x_port_config_up(port: *mut lan966x_port);
}
extern "C" {
    pub fn lan966x_port_init(port: *mut lan966x_port);
}
extern "C" {
    pub fn lan966x_mac_cpu_learn(lan966x: *mut lan966x, addr: *const c_char, vid: u16) -> c_int;
}
extern "C" {
    pub fn lan966x_mac_cpu_forget(lan966x: *mut lan966x, addr: *const c_char, vid: u16) -> c_int;
}
extern "C" {
    pub fn lan966x_mac_init(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_mac_purge_entries(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_mac_irq_handler(lan966x: *mut lan966x) -> irqreturn_t;
}
extern "C" {
    pub fn lan966x_vlan_init(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_vlan_port_apply(port: *mut lan966x_port);
}
extern "C" {
    pub fn lan966x_vlan_cpu_member_cpu_vlan_mask(lan966x: *mut lan966x, vid: u16) -> bool;
}
extern "C" {
    pub fn lan966x_vlan_port_rew_host(port: *mut lan966x_port);
}
extern "C" {
    pub fn lan966x_vlan_port_del_vlan(port: *mut lan966x_port, vid: u16);
}
extern "C" {
    pub fn lan966x_vlan_cpu_add_vlan(lan966x: *mut lan966x, vid: u16);
}
extern "C" {
    pub fn lan966x_vlan_cpu_del_vlan(lan966x: *mut lan966x, vid: u16);
}
extern "C" {
    pub fn lan966x_fdb_write_entries(lan966x: *mut lan966x, vid: u16);
}
extern "C" {
    pub fn lan966x_fdb_erase_entries(lan966x: *mut lan966x, vid: u16);
}
extern "C" {
    pub fn lan966x_fdb_init(lan966x: *mut lan966x) -> c_int;
}
extern "C" {
    pub fn lan966x_fdb_deinit(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_fdb_flush_workqueue(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_mdb_init(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_mdb_deinit(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_mdb_erase_entries(lan966x: *mut lan966x, vid: u16);
}
extern "C" {
    pub fn lan966x_mdb_write_entries(lan966x: *mut lan966x, vid: u16);
}
extern "C" {
    pub fn lan966x_mdb_clear_entries(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_mdb_restore_entries(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_ptp_init(lan966x: *mut lan966x) -> c_int;
}
extern "C" {
    pub fn lan966x_ptp_deinit(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_ptp_irq_handler(irq: c_int, args: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lan966x_ptp_ext_irq_handler(irq: c_int, args: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lan966x_ptp_get_period_ps() -> u32;
}
extern "C" {
    pub fn lan966x_ptp_gettime64(ptp: *mut ptp_clock_info, ts: *mut timespec64) -> c_int;
}
extern "C" {
    pub fn lan966x_ptp_del_traps(port: *mut lan966x_port) -> c_int;
}
extern "C" {
    pub fn lan966x_fdma_xmit(skb: *mut sk_buff, ifh: *mut __be32, dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn lan966x_fdma_xmit_xdpf(port: *mut lan966x_port, ptr: *mut c_void, len: u32) -> c_int;
}
extern "C" {
    pub fn lan966x_fdma_change_mtu(lan966x: *mut lan966x) -> c_int;
}
extern "C" {
    pub fn lan966x_fdma_netdev_init(lan966x: *mut lan966x, dev: *mut net_device);
}
extern "C" {
    pub fn lan966x_fdma_netdev_deinit(lan966x: *mut lan966x, dev: *mut net_device);
}
extern "C" {
    pub fn lan966x_fdma_init(lan966x: *mut lan966x) -> c_int;
}
extern "C" {
    pub fn lan966x_fdma_deinit(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_fdma_irq_handler(irq: c_int, args: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn lan966x_fdma_reload_page_pool(lan966x: *mut lan966x) -> c_int;
}
extern "C" {
    pub fn lan966x_lag_port_leave(port: *mut lan966x_port, bond: *mut net_device);
}
extern "C" {
    pub fn lan966x_lag_first_port(lag: *mut net_device, dev: *mut net_device) -> bool;
}
extern "C" {
    pub fn lan966x_lag_get_mask(lan966x: *mut lan966x, bond: *mut net_device) -> u32;
}
extern "C" {
    pub fn lan966x_port_stp_state_set(port: *mut lan966x_port, state: u8);
}
extern "C" {
    pub fn lan966x_update_fwd_mask(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_mqprio_add(port: *mut lan966x_port, num_tc: u8) -> c_int;
}
extern "C" {
    pub fn lan966x_mqprio_del(port: *mut lan966x_port) -> c_int;
}
extern "C" {
    pub fn lan966x_taprio_init(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_taprio_deinit(lan966x: *mut lan966x);
}
extern "C" {
    pub fn lan966x_taprio_del(port: *mut lan966x_port) -> c_int;
}
extern "C" {
    pub fn lan966x_taprio_speed_set(port: *mut lan966x_port, speed: c_int) -> c_int;
}
extern "C" {
    pub fn lan966x_xdp_port_init(port: *mut lan966x_port) -> c_int;
}
extern "C" {
    pub fn lan966x_xdp_port_deinit(port: *mut lan966x_port);
}
extern "C" {
    pub fn lan966x_xdp(dev: *mut net_device, xdp: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn lan966x_xdp_present(lan966x: *mut lan966x) -> bool;
}
extern "C" {
    pub fn lan966x_vcap_init(lan966x: *mut lan966x) -> c_int;
}
extern "C" {
    pub fn lan966x_vcap_deinit(lan966x: *mut lan966x);
}

extern "C" {
    pub fn lan966x_dcb_init(lan966x: *mut lan966x);
}

