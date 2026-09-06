//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qed/qed_eth_if.h
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
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

// 64 max queues * (1 rx + 4 tx-cos + 1 xdp)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_queue_start_common_params {
// Should always be relative to entity sending this.
    pub vport_id: u8,
    pub queue_id: u16,
// Relative, but relevant only for PFs
    pub stats_id: u8,
    pub p_sb: *mut qed_sb_info,
    pub sb_idx: u8,
    pub tc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_rxq_start_ret_params {
    pub p_prod: *mut void __iomem,
    pub p_handle: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_txq_start_ret_params {
    pub p_doorbell: *mut void __iomem,
    pub p_handle: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_filter_config_mode {
    QED_FILTER_CONFIG_MODE_DISABLE,
    QED_FILTER_CONFIG_MODE_5_TUPLE,
    QED_FILTER_CONFIG_MODE_L4_PORT,
    QED_FILTER_CONFIG_MODE_IP_DEST,
    QED_FILTER_CONFIG_MODE_IP_SRC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_ntuple_filter_params {
// Physically mapped address containing header of buffer to be used
// as filter.
//
    pub addr: dma_addr_t,
// Length of header in bytes
    pub length: u16,
// Relative queue-id to receive classified packet

    pub qid: u16,
// Identifier can either be according to vport-id or vfid
    pub b_is_vf: bool,
    pub vport_id: u8,
    pub vf_id: u8,
// true iff this filter is to be added. Else to be removed
    pub b_is_add: bool,
// If flow needs to be dropped
    pub b_is_drop: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_dev_eth_info {
    pub common: qed_dev_info,
    pub num_queues: u8,
    pub num_tc: u8,
    pub port_mac: [u8; ETH_ALEN],
    pub num_vlan_filters: u16,
    pub num_mac_filters: u16,
// Legacy VF - this affects the datapath, so qede has to know
    pub is_legacy: bool,
// Might depend on available resources [in case of VF]
    pub xdp_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_update_vport_rss_params {
    pub rss_ind_table: [*mut c_void; 128],
    pub rss_key: [u32; 10],
    pub rss_caps: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_update_vport_params {
    pub vport_id: u8,
    pub update_vport_active_flg: u8,
    pub vport_active_flg: u8,
    pub update_tx_switching_flg: u8,
    pub tx_switching_flg: u8,
    pub update_accept_any_vlan_flg: u8,
    pub accept_any_vlan: u8,
    pub update_rss_flg: u8,
    pub rss_params: qed_update_vport_rss_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_start_vport_params {
    pub remove_inner_vlan: bool,
    pub handle_ptp_pkts: bool,
    pub gro_enable: bool,
    pub drop_ttl0: bool,
    pub vport_id: u8,
    pub mtu: u16,
    pub clear_stats: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_filter_rx_mode_type {
    QED_FILTER_RX_MODE_TYPE_REGULAR,
    QED_FILTER_RX_MODE_TYPE_MULTI_PROMISC,
    QED_FILTER_RX_MODE_TYPE_PROMISC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_filter_xcast_params_type {
    QED_FILTER_XCAST_TYPE_ADD,
    QED_FILTER_XCAST_TYPE_DEL,
    QED_FILTER_XCAST_TYPE_REPLACE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_filter_ucast_params {
    pub type: qed_filter_xcast_params_type,
    pub vlan_valid: u8,
    pub vlan: u16,
    pub mac_valid: u8,
    pub mac: [c_uchar; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_filter_mcast_params {
    pub type: qed_filter_xcast_params_type,
    pub num: u8,
    pub mac: [c_uchar; 64][ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_filter_type {
    QED_FILTER_TYPE_UCAST,
    QED_FILTER_TYPE_MCAST,
    QED_FILTER_TYPE_RX_MODE,
    QED_MAX_FILTER_TYPES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_tunn_params {
    pub vxlan_port: u16,
    pub update_vxlan_port: u8,
    pub geneve_port: u16,
    pub update_geneve_port: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_cb_ops {
    pub common: qed_common_cb_ops,
    pub forced): *mut *mut *mut *mut void (force_mac) (void dev, u8 mac, bool,
    pub geneve_port): *mut *mut *mut void (ports_update)(void dev, u16 vxlan_port, u16,
}

pub const QED_MAX_PHC_DRIFT_PPB: c_int = 291666666;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ptp_filter_type {
    QED_PTP_FILTER_NONE,
    QED_PTP_FILTER_ALL,
    QED_PTP_FILTER_V1_L4_EVENT,
    QED_PTP_FILTER_V1_L4_GEN,
    QED_PTP_FILTER_V2_L4_EVENT,
    QED_PTP_FILTER_V2_L4_GEN,
    QED_PTP_FILTER_V2_L2_EVENT,
    QED_PTP_FILTER_V2_L2_GEN,
    QED_PTP_FILTER_V2_EVENT,
    QED_PTP_FILTER_V2_GEN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_ptp_hwtstamp_tx_type {
    QED_PTP_HWTSTAMP_TX_OFF,
    QED_PTP_HWTSTAMP_TX_ON,
}

// Prototype declaration of qed_eth_dcbnl_ops should match with the declaration
// of dcbnl_rtnl_ops structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_dcbnl_ops {
// IEEE 802.1Qaz std
    pub pfc): *mut *mut *mut int (ieee_getpfc)(struct qed_dev cdev, struct ieee_pfc,
    pub pfc): *mut *mut *mut int (ieee_setpfc)(struct qed_dev cdev, struct ieee_pfc,
    pub ets): *mut *mut *mut int (ieee_getets)(struct qed_dev cdev, struct ieee_ets,
    pub ets): *mut *mut *mut int (ieee_setets)(struct qed_dev cdev, struct ieee_ets,
    pub ets): *mut *mut *mut int (ieee_peer_getets)(struct qed_dev cdev, struct ieee_ets,
    pub pfc): *mut *mut *mut int (ieee_peer_getpfc)(struct qed_dev cdev, struct ieee_pfc,
    pub app): *mut *mut *mut int (ieee_getapp)(struct qed_dev cdev, struct dcb_app,
    pub app): *mut *mut *mut int (ieee_setapp)(struct qed_dev cdev, struct dcb_app,
// CEE std
    pub cdev): *mut *mut u8 (getstate)(struct qed_dev,
    pub state): *mut *mut *mut u8 (setstate)(struct qed_dev cdev, u8,
    pub up_map): *mut *mut *mut u8 pgid, u8 bw_pct, u8,
    pub bw_pct): *mut *mut *mut void (getpgbwgcfgtx)(struct qed_dev cdev, int pgid, u8,
    pub up_map): *mut *mut *mut u8 pgid, u8 bw_pct, u8,
    pub bw_pct): *mut *mut *mut void (getpgbwgcfgrx)(struct qed_dev cdev, int pgid, u8,
    pub setting): *mut *mut *mut void (getpfccfg)(struct qed_dev cdev, int prio, u8,
    pub setting): *mut *mut *mut void (setpfccfg)(struct qed_dev cdev, int prio, u8,
    pub cap): *mut *mut *mut u8 (getcap)(struct qed_dev cdev, int capid, u8,
    pub num): *mut *mut *mut int (getnumtcs)(struct qed_dev cdev, int tcid, u8,
    pub cdev): *mut *mut u8 (getpfcstate)(struct qed_dev,
    pub id): *mut *mut *mut int (getapp)(struct qed_dev cdev, u8 idtype, u16,
    pub flags): *mut *mut *mut u8 (getfeatcfg)(struct qed_dev cdev, int featid, u8,
// DCBX configuration
    pub cdev): *mut *mut u8 (getdcbx)(struct qed_dev,
    pub up_map): u8 pri_type, u8 pgid, u8 bw_pct, u8,
    pub up_map): u8 pri_type, u8 pgid, u8 bw_pct, u8,
    pub bw_pct): *mut *mut *mut void (setpgbwgcfgtx)(struct qed_dev cdev, int pgid, u8,
    pub bw_pct): *mut *mut *mut void (setpgbwgcfgrx)(struct qed_dev cdev, int pgid, u8,
    pub cdev): *mut *mut u8 (setall)(struct qed_dev,
    pub num): *mut *mut *mut int (setnumtcs)(struct qed_dev cdev, int tcid, u8,
    pub state): *mut *mut *mut void (setpfcstate)(struct qed_dev cdev, u8,
    pub up): *mut *mut *mut int (setapp)(struct qed_dev cdev, u8 idtype, u16 idval, u8,
    pub state): *mut *mut *mut u8 (setdcbx)(struct qed_dev cdev, u8,
    pub flags): *mut *mut *mut u8 (setfeatcfg)(struct qed_dev cdev, int featid, u8,
// Peer apps
    pub app_count): *mut u16,
    pub table): *mut *mut *mut int (peer_getapptable)(struct qed_dev cdev, struct dcb_app,
// CEE peer
    pub pfc): *mut *mut *mut int (cee_peer_getpfc)(struct qed_dev cdev, struct cee_pfc,
    pub pg): *mut *mut *mut int (cee_peer_getpg)(struct qed_dev cdev, struct cee_pg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_ptp_ops {
    pub qed_ptp_hwtstamp_tx_type): enum,
    pub ): *mut *mut *mut int (read_rx_ts)(struct qed_dev , u64,
    pub ): *mut *mut *mut int (read_tx_ts)(struct qed_dev , u64,
    pub ): *mut *mut *mut int (read_cc)(struct qed_dev , u64,
    pub ): *mut *mut int (disable)(struct qed_dev,
    pub s32): *mut *mut *mut int (adjfreq)(struct qed_dev ,,
    pub ): *mut *mut int (enable)(struct qed_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_eth_ops {
    pub common: *const qed_common_ops,

    pub iov: *const qed_iov_hv_ops,

    pub dcb: *const qed_eth_dcbnl_ops,

    pub ptp: *const qed_eth_ptp_ops,
    pub info): *mut qed_dev_eth_info,
    pub cookie): *mut c_void,
    pub mac): *mut *mut *mut bool(check_mac) (struct qed_dev cdev, u8,
    pub params): *mut qed_start_vport_params,
    pub vport_id): u8,
    pub params): *mut qed_update_vport_params,
    pub ret_params): *mut qed_rxq_start_ret_params,
    pub handle): *mut *mut *mut int (q_rx_stop)(struct qed_dev cdev, u8 rss_id, void,
    pub ret_params): *mut qed_txq_start_ret_params,
    pub handle): *mut *mut *mut int (q_tx_stop)(struct qed_dev cdev, u8 rss_id, void,
    pub type): qed_filter_rx_mode_type,
    pub params): *mut qed_filter_ucast_params,
    pub params): *mut qed_filter_mcast_params,
    pub cdev): *mut *mut int (fastpath_stop)(struct qed_dev,
    pub cqe): *mut eth_slow_path_rx_cqe,
    pub stats): *mut qed_eth_stats,
    pub params): *mut qed_tunn_params,
    pub params): *mut qed_ntuple_filter_params,
    pub mode): qed_filter_config_mode,
    pub handle): *mut *mut *mut *mut int (get_coalesce)(struct qed_dev cdev, u16 coal, void,
    pub mac): *const *const *const int (req_bulletin_update_mac)(struct qed_dev cdev, u8,
}

extern "C" {
    pub fn qed_put_eth_ops();
}
