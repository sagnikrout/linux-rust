//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qede/qede.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qede NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_stats_common {
    pub no_buff_discards: u64,
    pub packet_too_big_discard: u64,
    pub ttl0_discard: u64,
    pub rx_ucast_bytes: u64,
    pub rx_mcast_bytes: u64,
    pub rx_bcast_bytes: u64,
    pub rx_ucast_pkts: u64,
    pub rx_mcast_pkts: u64,
    pub rx_bcast_pkts: u64,
    pub mftag_filter_discards: u64,
    pub mac_filter_discards: u64,
    pub gft_filter_drop: u64,
    pub tx_ucast_bytes: u64,
    pub tx_mcast_bytes: u64,
    pub tx_bcast_bytes: u64,
    pub tx_ucast_pkts: u64,
    pub tx_mcast_pkts: u64,
    pub tx_bcast_pkts: u64,
    pub tx_err_drop_pkts: u64,
    pub coalesced_pkts: u64,
    pub coalesced_events: u64,
    pub coalesced_aborts_num: u64,
    pub non_coalesced_pkts: u64,
    pub coalesced_bytes: u64,
    pub link_change_count: u64,
    pub ptp_skip_txts: u64,
// port
    pub rx_64_byte_packets: u64,
    pub rx_65_to_127_byte_packets: u64,
    pub rx_128_to_255_byte_packets: u64,
    pub rx_256_to_511_byte_packets: u64,
    pub rx_512_to_1023_byte_packets: u64,
    pub rx_1024_to_1518_byte_packets: u64,
    pub rx_crc_errors: u64,
    pub rx_mac_crtl_frames: u64,
    pub rx_pause_frames: u64,
    pub rx_pfc_frames: u64,
    pub rx_align_errors: u64,
    pub rx_carrier_errors: u64,
    pub rx_oversize_packets: u64,
    pub rx_jabbers: u64,
    pub rx_undersize_packets: u64,
    pub rx_fragments: u64,
    pub tx_64_byte_packets: u64,
    pub tx_65_to_127_byte_packets: u64,
    pub tx_128_to_255_byte_packets: u64,
    pub tx_256_to_511_byte_packets: u64,
    pub tx_512_to_1023_byte_packets: u64,
    pub tx_1024_to_1518_byte_packets: u64,
    pub tx_pause_frames: u64,
    pub tx_pfc_frames: u64,
    pub brb_truncates: u64,
    pub brb_discards: u64,
    pub tx_mac_ctrl_frames: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_stats_bb {
    pub rx_1519_to_1522_byte_packets: u64,
    pub rx_1519_to_2047_byte_packets: u64,
    pub rx_2048_to_4095_byte_packets: u64,
    pub rx_4096_to_9216_byte_packets: u64,
    pub rx_9217_to_16383_byte_packets: u64,
    pub tx_1519_to_2047_byte_packets: u64,
    pub tx_2048_to_4095_byte_packets: u64,
    pub tx_4096_to_9216_byte_packets: u64,
    pub tx_9217_to_16383_byte_packets: u64,
    pub tx_lpi_entry_count: u64,
    pub tx_total_collisions: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_stats_ah {
    pub rx_1519_to_max_byte_packets: u64,
    pub tx_1519_to_max_byte_packets: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_stats {
    pub common: qede_stats_common,
    pub bb: qede_stats_bb,
    pub ah: qede_stats_ah,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_vlan {
    pub list: list_head,
    pub vid: u16,
    pub configured: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_rdma_dev {
    pub qedr_dev: *mut qedr_dev,
    pub entry: list_head,
    pub rdma_event_list: list_head,
    pub rdma_wq: *mut workqueue_struct,
    pub refcnt: kref,
    pub event_comp: completion,
    pub exp_recovery: bool,
}

pub const QEDE_RFS_MAX_FLTR: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qede_flags_bit {
    QEDE_FLAGS_IS_VF = 0,
    QEDE_FLAGS_LINK_REQUESTED,
    QEDE_FLAGS_PTP_TX_IN_PRORGESS,
    QEDE_FLAGS_TX_TIMESTAMPING_EN
}

pub const QEDE_DUMP_MAX_ARGS: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qede_dump_cmd {
    QEDE_DUMP_CMD_NONE = 0,
    QEDE_DUMP_CMD_NVM_CFG,
    QEDE_DUMP_CMD_GRCDUMP,
    QEDE_DUMP_CMD_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_dump_info {
    pub cmd: qede_dump_cmd,
    pub num_args: u8,
    pub args: [u32; QEDE_DUMP_MAX_ARGS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_coalesce {
    pub isvalid: bool,
    pub rxc: u16,
    pub txc: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_dev {
    pub cdev: *mut qed_dev,
    pub ndev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub devlink: *mut devlink,
    pub dp_module: u32,
    pub dp_level: u8,
    pub flags: c_ulong,

    pub ops: *const qed_eth_ops,
    pub ptp: *mut qede_ptp,
    pub ptp_skip_txts: u64,
    pub dev_info: qed_dev_eth_info,

    pub fp_array: *mut qede_fastpath,
    pub coal_entry: *mut qede_coalesce,
    pub req_num_tx: u8,
    pub fp_num_tx: u8,
    pub req_num_rx: u8,
    pub fp_num_rx: u8,
    pub req_queues: u16,
    pub num_queues: u16,
    pub total_xdp_queues: u16,

    pub int_info: qed_int_info,
// Smaller private variant of the RTNL lock
    pub qede_lock: mutex,
    pub /: *mut *mut u32 state; / Protected by qede_lock,
    pub rx_buf_size: u16,
    pub rx_copybreak: u32,
// L2 header size + 2*VLANs (8 bytes) + LLC SNAP (8 bytes)

// Max supported alignment is 256 (8 shift)
// minimal alignment shift 6 is optimal for 57xxx HW performance
//

// We assume skb_build() uses sizeof(struct skb_shared_info) bytes
// at the end of skb->data, to avoid wasting a full cache line.
// This reduces memory use (skb->truesize).
//

    pub stats: qede_stats,
// Bitfield to track initialized RSS params
    pub rss_params_inited: u32,
    pub rss_ind_table: [u16; 128],
    pub rss_key: [u32; 10],
    pub rss_caps: u8,
// Both must be a power of two
    pub q_num_rx_buffers: u16,
    pub q_num_tx_buffers: u16,
    pub gro_disable: bool,
    pub vlan_list: list_head,
    pub configured_vlans: u16,
    pub non_configured_vlans: u16,
    pub accept_any_vlan: bool,
    pub sp_task: delayed_work,
    pub sp_flags: c_ulong,
    pub vxlan_dst_port: u16,
    pub geneve_dst_port: u16,
    pub arfs: *mut qede_arfs,
    pub wol_enabled: bool,
    pub rdma_info: qede_rdma_dev,
    pub xdp_prog: *mut bpf_prog,
    pub last_err_type: qed_hw_err_type,
    pub err_flags: c_ulong,
pub const QEDE_ERR_IS_HANDLED: c_int = 31;
pub const QEDE_ERR_ATTN_CLR_EN: c_int = 0;
pub const QEDE_ERR_GET_DBG_INFO: c_int = 1;
pub const QEDE_ERR_IS_RECOVERABLE: c_int = 2;
pub const QEDE_ERR_WARN: c_int = 3;
    pub dump_info: qede_dump_info,
    pub periodic_task: delayed_work,
    pub stats_coal_ticks: c_ulong,
    pub stats_coal_usecs: u32,
    pub /: *mut *mut spinlock_t stats_lock; / lock for vport stats access,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum QEDE_STATE {
    QEDE_STATE_CLOSED,
    QEDE_STATE_OPEN,
    QEDE_STATE_RECOVERY,
}

pub const MAX_NUM_TC: c_int = 8;
pub const MAX_NUM_PRI: c_int = 8;
// The driver supports the new build_skb() API:
// RX ring buffer contains pointer to kmalloc() data only,
// skb are built only after the frame was DMA-ed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_rx_data {
    pub data: *mut page,
    pub mapping: dma_addr_t,
    pub page_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qede_agg_state {
    QEDE_AGG_STATE_NONE  = 0,
    QEDE_AGG_STATE_START = 1,
    QEDE_AGG_STATE_ERROR = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_agg_info {
// buffer is used to retain the Rx consumer descriptor when a TPA
// session starts. If the SKB allocation fails during TPA_START,
// we use this saved buffer to safely recycle the physical page
// back into the rx-bd-ring via qede_reuse_page(). We don't want
// to be in a state where allocation fails, as we can't reuse the
// consumer buffer in the rx-chain since FW may still be writing to it
// (since header needs to be modified for TPA).
// The second purpose is to keep a pointer to the bd buffer during
// aggregation.
//
    pub buffer: sw_rx_data,
    pub skb: *mut sk_buff,
// We need some structs from the start cookie until termination
    pub vlan_tag: u16,
    pub tpa_start_fail: bool,
    pub state: u8,
    pub frag_id: u8,
    pub tunnel_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_rx_queue {
    pub hw_cons_ptr: *mut __le16,
    pub hw_rxq_prod_addr: *mut void __iomem,
// Required for the allocation of replacement buffers
    pub dev: *mut device,
    pub xdp_prog: *mut bpf_prog,
    pub sw_rx_cons: u16,
    pub sw_rx_prod: u16,
    pub filled_buffers: u16,
    pub data_direction: u8,
    pub rxq_id: u8,
// Used once per each NAPI run
    pub num_rx_buffers: u16,
    pub rx_headroom: u16,
    pub rx_buf_size: u32,
    pub rx_buf_seg_size: u32,
    pub sw_rx_ring: *mut sw_rx_data,
    pub rx_bd_ring: qed_chain,
    pub ____cacheline_aligned: qed_chain rx_comp_ring,
// GRO
    pub tpa_info: [qede_agg_info; ETH_TPA_MAX_AGGS_NUM],
// Used once per each NAPI run
    pub rcv_pkts: u64,
    pub rx_hw_errors: u64,
    pub rx_alloc_errors: u64,
    pub rx_ip_frags: u64,
    pub xdp_no_pass: u64,
    pub handle: *mut c_void,
    pub xdp_rxq: xdp_rxq_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union db_prod {
    pub data: eth_db_data,
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_tx_bd {
    pub skb: *mut sk_buff,
    pub flags: u8,
// Set on the first BD descriptor when there is a split BD

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sw_tx_xdp {
    pub page: *mut page,
    pub xdpf: *mut xdp_frame,
    pub mapping: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_tx_queue {
    pub is_xdp: u8,
    pub is_legacy: bool,
    pub sw_tx_cons: u16,
    pub sw_tx_prod: u16,
    pub /: *mut *mut u16 num_tx_buffers; / Slowpath only,
    pub xmit_pkts: u64,
    pub stopped_cnt: u64,
    pub tx_mem_alloc_err: u64,
    pub hw_cons_ptr: *mut __le16,
// Needed for the mapping of packets
    pub dev: *mut device,
    pub doorbell_addr: *mut void __iomem,
    pub tx_db: db_prod,
// Spinlock for XDP queues in case of XDP_REDIRECT
    pub xdp_tx_lock: spinlock_t,
    pub /: *mut *mut int index; / Slowpath only,

// Regular Tx requires skb + metadata for release purpose,
// while XDP requires the pages and the mapped address.
//
    pub skbs: *mut sw_tx_bd,
    pub xdp: *mut sw_tx_xdp,
    pub sw_tx_ring: },
    pub tx_pbl: qed_chain,
// Slowpath; Should be kept in end [unless missing padding]
    pub handle: *mut c_void,
    pub cos: u16,
    pub ndev_txq_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_fastpath {
    pub edev: *mut qede_dev,
    pub type: u8,

    pub id: u8,
    pub xdp_xmit: u8,

    pub napi: napi_struct,
    pub sb_info: *mut qed_sb_info,
    pub rxq: *mut qede_rx_queue,
    pub txq: *mut qede_tx_queue,
    pub xdp_tx: *mut qede_tx_queue,
    pub 8]: char name[IFNAMSIZ +,
}

// Debug print definitions

pub const XMIT_PLAIN: c_int = 0;

pub const QEDE_SP_RECOVERY: c_int = 0;
pub const QEDE_SP_RX_MODE: c_int = 1;
pub const QEDE_SP_RSVD1: c_int = 2;
pub const QEDE_SP_RSVD2: c_int = 3;
pub const QEDE_SP_HW_ERR: c_int = 4;
pub const QEDE_SP_ARFS_CONFIG: c_int = 5;
pub const QEDE_SP_AER: c_int = 7;
pub const QEDE_SP_DISABLE: c_int = 8;

extern "C" {
    pub fn qede_process_arfs_filters(edev: *mut qede_dev, free_fltr: bool);
}
extern "C" {
    pub fn qede_poll_for_freeing_arfs_filters(edev: *mut qede_dev);
}
extern "C" {
    pub fn qede_arfs_filter_op(dev: *mut c_void, filter: *mut c_void, fw_rc: u8);
}
extern "C" {
    pub fn qede_free_arfs(edev: *mut qede_dev);
}
extern "C" {
    pub fn qede_alloc_arfs(edev: *mut qede_dev) -> c_int;
}
extern "C" {
    pub fn qede_add_cls_rule(edev: *mut qede_dev, info: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn qede_delete_flow_filter(edev: *mut qede_dev, cookie: u64) -> c_int;
}
extern "C" {
    pub fn qede_get_cls_rule_entry(edev: *mut qede_dev, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn qede_get_arfs_filter_count(edev: *mut qede_dev) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qede_reload_args {
    pub args): *mut *mut *mut void (func)(struct qede_dev edev, struct qede_reload_args,
    pub features: netdev_features_t,
    pub new_prog: *mut bpf_prog,
    pub mtu: u16,
    pub u: },
}

// Datapath functions definition
extern "C" {
    pub fn qede_start_xmit(skb: *mut sk_buff, ndev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn qede_alloc_rx_buffer(rxq: *mut qede_rx_queue, allow_lazy: bool) -> c_int;
}
extern "C" {
    pub fn qede_poll(napi: *mut napi_struct, budget: c_int) -> c_int;
}
extern "C" {
    pub fn qede_msix_fp_int(irq: c_int, fp_cookie: *mut c_void) -> irqreturn_t;
}
// Filtering function definitions
extern "C" {
    pub fn qede_force_mac(dev: *mut c_void, mac: *mut u8, forced: bool);
}
extern "C" {
    pub fn qede_udp_ports_update(dev: *mut c_void, vxlan_port: u16, geneve_port: u16);
}
extern "C" {
    pub fn qede_set_mac_addr(ndev: *mut net_device, p: *mut c_void) -> c_int;
}
extern "C" {
    pub fn qede_vlan_rx_add_vid(dev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn qede_vlan_rx_kill_vid(dev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn qede_vlan_mark_nonconfigured(edev: *mut qede_dev);
}
extern "C" {
    pub fn qede_configure_vlan_filters(edev: *mut qede_dev) -> c_int;
}
extern "C" {
    pub fn qede_set_features(dev: *mut net_device, features: netdev_features_t) -> c_int;
}
extern "C" {
    pub fn qede_set_rx_mode(ndev: *mut net_device);
}
extern "C" {
    pub fn qede_config_rx_mode(ndev: *mut net_device);
}
extern "C" {
    pub fn qede_xdp(dev: *mut net_device, xdp: *mut netdev_bpf) -> c_int;
}

extern "C" {
    pub fn qede_set_dcbnl_ops(ndev: *mut net_device);
}

extern "C" {
    pub fn qede_config_debug(debug: c_uint, p_dp_module: *mut u32, p_dp_level: *mut u8);
}
extern "C" {
    pub fn qede_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn qede_set_udp_tunnels(edev: *mut qede_dev);
}
extern "C" {
    pub fn qede_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn qede_fill_by_demand_stats(edev: *mut qede_dev);
}
extern "C" {
    pub fn __qede_lock(edev: *mut qede_dev);
}
extern "C" {
    pub fn __qede_unlock(edev: *mut qede_dev);
}
extern "C" {
    pub fn qede_has_rx_work(rxq: *mut qede_rx_queue) -> bool;
}
extern "C" {
    pub fn qede_txq_has_work(txq: *mut qede_tx_queue) -> c_int;
}
extern "C" {
    pub fn qede_recycle_rx_bd_ring(rxq: *mut qede_rx_queue, count: u8);
}
extern "C" {
    pub fn qede_update_rx_prod(edev: *mut qede_dev, rxq: *mut qede_rx_queue);
}
extern "C" {
    pub fn qede_forced_speed_maps_init();
}
pub const RX_RING_SIZE_POW: c_int = 13;

pub const NUM_RX_BDS_MIN: c_int = 128;
pub const NUM_RX_BDS_KDUMP_MIN: c_int = 63;

pub const TX_RING_SIZE_POW: c_int = 13;

pub const NUM_TX_BDS_MIN: c_int = 128;
pub const NUM_TX_BDS_KDUMP_MIN: c_int = 63;

pub const QEDE_MIN_PKT_LEN: c_int = 64;
pub const QEDE_RX_HDR_SIZE: c_int = 256;
pub const QEDE_MAX_JUMBO_PACKET_SIZE: c_int = 9600;

