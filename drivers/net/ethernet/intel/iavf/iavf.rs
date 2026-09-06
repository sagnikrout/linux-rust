//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/iavf/iavf.h
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
// Copyright(c) 2013 - 2018 Intel Corporation.

pub const DEFAULT_DEBUG_LEVEL_SHIFT: c_int = 3;

extern "C" {
    pub fn iavf_status_to_errno(status: iavf_status) -> c_int;
}
extern "C" {
    pub fn virtchnl_status_to_errno(v_status: virtchnl_status_code) -> c_int;
}
// VSI state flags shared with common code
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_vsi_state_t {
    __IAVF_VSI_DOWN,
// This must be last as it determines the size of the BITMAP
    __IAVF_VSI_STATE_SIZE__,
}

// dummy struct to make common code less painful
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_vsi {
    pub back: *mut iavf_adapter,
    pub netdev: *mut net_device,
    pub seid: u16,
    pub id: u16,
    pub __IAVF_VSI_STATE_SIZE__): DECLARE_BITMAP(state,,
    pub base_vector: c_int,
    pub qs_handle: u16,
}

// How many Rx Buffers do we bundle into one write to the hardware ?

pub const IAVF_DEFAULT_TXD: c_int = 512;
pub const IAVF_DEFAULT_RXD: c_int = 512;
pub const IAVF_MAX_TXD: c_int = 4096;
pub const IAVF_MIN_TXD: c_int = 64;
pub const IAVF_MAX_RXD: c_int = 4096;
pub const IAVF_MIN_RXD: c_int = 64;
pub const IAVF_REQ_DESCRIPTOR_MULTIPLE: c_int = 32;
pub const IAVF_MAX_AQ_BUF_SIZE: c_int = 4096;
pub const IAVF_AQ_LEN: c_int = 32;

pub const IAVF_MAX_REQ_QUEUES: c_int = 16;

pub const IAVF_MBPS_QUANTA: c_int = 50;

// MAX_MSIX_Q_VECTORS of these are allocated,
// but we only use one per queue-specific vector.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_q_vector {
    pub adapter: *mut iavf_adapter,
    pub vsi: *mut iavf_vsi,
    pub napi: napi_struct,
    pub rx: iavf_ring_container,
    pub tx: iavf_ring_container,
    pub ring_mask: u32,
    pub /: *mut *mut u8 itr_countdown; / when 0 should adjust adaptive ITR,
    pub /: *mut *mut u8 num_ringpairs; / total number of ring pairs in vector,
    pub /: *mut *mut u16 v_idx; / index in the vsi->q_vector array.,
    pub /: *mut *mut u16 reg_idx; / register index of the interrupt,
    pub 15]: char name[IFNAMSIZ +,
    pub arm_wb_state: bool,
}

// Helper macros to switch between ints/sec and what the register uses.
// And yes, it's the same math going both ways.  The lowest value
// supported by all of the iavf hardware is 8.
//

pub const OTHER_VECTOR: c_int = 1;

pub const MIN_MSIX_Q_VECTORS: c_int = 1;

pub const IAVF_QUEUE_END_OF_LIST: c_uint = 0x7FF;
pub const IAVF_FREE_VECTOR: c_uint = 0x7FFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_mac_filter {
    pub list: list_head,
    pub macaddr: [u8; ETH_ALEN],
    pub /: *mut *mut u8 is_new_mac:1; / filter is new, wait for PF decision,
    pub /: *mut *mut u8 remove:1; / filter needs to be removed,
    pub /: *mut *mut u8 add:1; / filter needs to be added,
    pub /: *mut *mut u8 is_primary:1; / filter is a default VF MAC,
    pub /: *mut *mut u8 add_handled:1; / received response for filter add,
    pub padding:3: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_vlan {
    pub vid: u16,
    pub tpid: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_vlan_state_t {
    IAVF_VLAN_INVALID,
    IAVF_VLAN_ADD,		/* filter needs to be added */
    IAVF_VLAN_ADDING,	/* ADD sent to PF, waiting for response */
    IAVF_VLAN_ACTIVE,	/* PF confirmed, filter is in HW */
    IAVF_VLAN_REMOVE,	/* filter queued for DEL from PF */
    IAVF_VLAN_REMOVING,	/* DEL sent to PF, waiting for response */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_vlan_filter {
    pub list: list_head,
    pub vlan: iavf_vlan,
    pub state: iavf_vlan_state_t,
}

pub const IAVF_MAX_TRAFFIC_CLASS: c_int = 4;
// State of traffic class creation
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_tc_state_t {
    __IAVF_TC_INVALID, /* no traffic class, default state */
    __IAVF_TC_RUNNING, /* traffic classes have been created */
}

// channel info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_channel_config {
    pub ch_info: [virtchnl_channel_info; IAVF_MAX_TRAFFIC_CLASS],
    pub state: iavf_tc_state_t,
    pub total_qps: u8,
}

// State of cloud filter
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_cloud_filter_state_t {
    __IAVF_CF_INVALID,	 /* cloud filter not added */
    __IAVF_CF_ADD_PENDING, /* cloud filter pending add by the PF */
    __IAVF_CF_DEL_PENDING, /* cloud filter pending del by the PF */
    __IAVF_CF_ACTIVE,	 /* cloud filter is active */
}

// Driver state. The order of these is important!
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_state_t {
    __IAVF_STARTUP,		/* driver loaded, probe complete */
    __IAVF_REMOVE,		/* driver is being unloaded */
    __IAVF_INIT_VERSION_CHECK,	/* aq msg sent, awaiting reply */
    __IAVF_INIT_GET_RESOURCES,	/* aq msg sent, awaiting reply */
    __IAVF_INIT_EXTENDED_CAPS,	/* process extended caps which require aq msg exchange */
    __IAVF_INIT_CONFIG_ADAPTER,
    __IAVF_INIT_SW,		/* got resources, setting up structs */
    __IAVF_INIT_FAILED,	/* init failed, restarting procedure */
    __IAVF_RESETTING,		/* in reset */
    __IAVF_COMM_FAILED,		/* communication with PF failed */
// Below here, watchdog is running
    __IAVF_DOWN,			/* ready, can be opened */
    __IAVF_DOWN_PENDING,		/* descending, waiting for watchdog */
    __IAVF_TESTING,		/* in ethtool self-test */
    __IAVF_RUNNING,		/* opened, working */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iavf_critical_section_t {
    __IAVF_IN_REMOVE_TASK,	/* device being removed */
}

pub const IAVF_CLOUD_FIELD_OMAC: c_uint = 0x01;
pub const IAVF_CLOUD_FIELD_IMAC: c_uint = 0x02;
pub const IAVF_CLOUD_FIELD_IVLAN: c_uint = 0x04;
pub const IAVF_CLOUD_FIELD_TEN_ID: c_uint = 0x08;
pub const IAVF_CLOUD_FIELD_IIP: c_uint = 0x10;

// bookkeeping of cloud filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_cloud_filter {
    pub state: iavf_cloud_filter_state_t,
    pub list: list_head,
    pub f: virtchnl_filter,
    pub cookie: c_ulong,
    pub /: *mut *mut bool del; / filter needs to be deleted,
    pub /: *mut *mut bool add; / filter needs to be added,
}

pub const IAVF_RESET_WAIT_MS: c_int = 10;
pub const IAVF_RESET_WAIT_DETECTED_COUNT: c_int = 500;
pub const IAVF_RESET_WAIT_COMPLETE_COUNT: c_int = 2000;
pub const IAVF_MAX_QOS_TC_NUM: c_int = 8;
pub const IAVF_DEFAULT_QUANTA_SIZE: c_int = 1024;
// board specific private data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iavf_adapter {
    pub wq: *mut workqueue_struct,
    pub reset_task: work_struct,
    pub adminq_task: work_struct,
    pub finish_config: work_struct,
    pub down_waitqueue: wait_queue_head_t,
    pub vc_waitqueue: wait_queue_head_t,
    pub q_vectors: *mut iavf_q_vector,
    pub vlan_filter_list: list_head,
    pub num_vlan_filters: c_int,
    pub mac_filter_list: list_head,
// Lock to protect accesses to MAC and VLAN lists
    pub mac_vlan_list_lock: spinlock_t,
    pub 9]: char misc_vector_name[IFNAMSIZ +,
    pub rxdid: u8,
    pub num_active_queues: c_int,
    pub num_req_queues: c_int,
// TX
    pub tx_rings: *mut iavf_ring,
    pub tx_timeout_count: u32,
    pub tx_desc_count: u32,
// RX
    pub rx_rings: *mut iavf_ring,
    pub hw_csum_rx_error: u64,
    pub rx_desc_count: u32,
    pub num_msix_vectors: c_int,
    pub msix_entries: *mut msix_entry,
    pub flags: u32,

// BIT(15) is free, was IAVF_FLAG_LEGACY_RX

// duplicates for common code
pub const IAVF_FLAG_DCB_ENABLED: c_int = 0;
// flags for admin queue service task
    pub aq_required: u64,

// Newer style, RSS done by the PF so we can ignore hardware vagaries.

// AQ messages that must be sent after IAVF_FLAG_AQ_GET_CONFIG, in
// order to negotiated extended capabilities.
//

// flags for processing extended capability messages during
// __IAVF_INIT_EXTENDED_CAPS. Each capability exchange requires
// both a SEND and a RECV step, which must be processed in sequence.
//
// During the __IAVF_INIT_EXTENDED_CAPS state, the driver will
// process one flag at a time during each state loop.
//
    pub extended_caps: u64,

// Lock to prevent possible clobbering of
// current_netdev_promisc_flags
//
    pub current_netdev_promisc_flags_lock: spinlock_t,
    pub current_netdev_promisc_flags: netdev_features_t,
// OS defined structs
    pub netdev: *mut net_device,
    pub pdev: *mut pci_dev,
    pub /: *mut *mut iavf_hw hw; / defined in iavf_type.h,
    pub state: iavf_state_t,
    pub last_state: iavf_state_t,
    pub crit_section: c_ulong,
    pub watchdog_task: delayed_work,
    pub link_up: bool,
    pub link_speed: virtchnl_link_speed,
// This is only populated if the VIRTCHNL_VF_CAP_ADV_LINK_SPEED is set
// in vf_res->vf_cap_flags. Use ADV_LINK_SUPPORT macro to determine if
// this field is valid. This field should be used going forward and the
// enum virtchnl_link_speed above should be considered the legacy way of
// storing/communicating link speeds.
//
    pub link_speed_mbps: u32,
    pub current_op: virtchnl_ops,
// RSS by the PF should be preferred over RSS via other methods.

    pub /: *mut *mut *mut virtchnl_vf_resource vf_res; / incl. all VSIs,
    pub /: *mut *mut *mut virtchnl_vsi_resource vsi_res; / our LAN VSI,
    pub pf_version: virtchnl_version_info,

    pub vlan_v2_caps: virtchnl_vlan_caps,
    pub supp_rxdids: u64,
    pub ptp: iavf_ptp,
    pub msg_enable: u16,
    pub current_stats: iavf_eth_stats,
    pub qos_caps: *mut virtchnl_qos_cap_list,
    pub vsi: iavf_vsi,
    pub aq_wait_count: u32,
// RSS stuff
    pub hfunc: virtchnl_rss_algorithm,
    pub rss_hashcfg: u64,
    pub rss_key_size: u16,
    pub rss_lut_size: u16,
    pub rss_key: *mut u8,
    pub rss_lut: *mut u8,
// ADQ related members
    pub ch_config: iavf_channel_config,
    pub num_tc: u8,
    pub cloud_filter_list: list_head,
// lock to protect access to the cloud filter list
    pub cloud_filter_list_lock: spinlock_t,
    pub num_cloud_filters: u16,
// snapshot of "num_active_queues" before setup_tc for qdisc add
// is invoked. This information is useful during qdisc del flow,
// to restore correct number of queues
//
    pub orig_num_active_queues: c_int,

    pub fdir_active_fltr: u16,
    pub raw_fdir_active_fltr: u16,
    pub fdir_list_head: list_head,
    pub /: *mut *mut spinlock_t fdir_fltr_lock; / protect the Flow Director filter list,
    pub adv_rss_list_head: list_head,
    pub /: *mut *mut spinlock_t adv_rss_lock; / protect the RSS management list,
}

// Must be called with fdir_fltr_lock lock held
// Ethtool Private Flags
// needed by iavf_ethtool.c
extern "C" {
    pub fn iavf_down(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_process_config(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_parse_vf_resource_msg(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_schedule_reset(adapter: *mut iavf_adapter, flags: u64);
}
extern "C" {
    pub fn iavf_schedule_aq_request(adapter: *mut iavf_adapter, flags: u64);
}
extern "C" {
    pub fn iavf_schedule_finish_config(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn iavf_free_all_tx_resources(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_free_all_rx_resources(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_send_api_ver(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_verify_api_ver(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_send_vf_config_msg(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_get_vf_config(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_get_vf_vlan_v2_caps(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_send_vf_offload_vlan_v2_msg(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_send_vf_supported_rxdids_msg(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_get_vf_supported_rxdids(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_send_vf_ptp_caps_msg(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_get_vf_ptp_caps(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_set_queue_vlan_tag_loc(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_get_num_vlans_added(adapter: *mut iavf_adapter) -> u16;
}
extern "C" {
    pub fn iavf_irq_enable(adapter: *mut iavf_adapter, flush: bool);
}
extern "C" {
    pub fn iavf_configure_queues(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_enable_queues(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_disable_queues(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_map_queues(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_add_ether_addrs(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_del_ether_addrs(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_add_vlans(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_del_vlans(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_set_promiscuous(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_promiscuous_mode_changed(adapter: *mut iavf_adapter) -> bool;
}
extern "C" {
    pub fn iavf_request_stats(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_request_reset(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_get_rss_hashcfg(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_set_rss_hashcfg(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_set_rss_key(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_set_rss_lut(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_set_rss_hfunc(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_enable_vlan_stripping(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_disable_vlan_stripping(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_config_rss(adapter: *mut iavf_adapter) -> c_int;
}
extern "C" {
    pub fn iavf_cfg_queues_bw(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_cfg_queues_quanta_size(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_get_qos_caps(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_enable_channels(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_disable_channels(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_add_cloud_filter(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_del_cloud_filter(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_enable_vlan_stripping_v2(adapter: *mut iavf_adapter, tpid: u16);
}
extern "C" {
    pub fn iavf_disable_vlan_stripping_v2(adapter: *mut iavf_adapter, tpid: u16);
}
extern "C" {
    pub fn iavf_enable_vlan_insertion_v2(adapter: *mut iavf_adapter, tpid: u16);
}
extern "C" {
    pub fn iavf_disable_vlan_insertion_v2(adapter: *mut iavf_adapter, tpid: u16);
}
extern "C" {
    pub fn iavf_add_fdir_filter(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_del_fdir_filter(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_add_adv_rss_cfg(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_del_adv_rss_cfg(adapter: *mut iavf_adapter);
}
extern "C" {
    pub fn iavf_reset_step(adapter: *mut iavf_adapter);
}
