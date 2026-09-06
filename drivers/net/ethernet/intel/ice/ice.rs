//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice.h
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
// Copyright (c) 2018, Intel Corporation.

pub const ICE_BAR0: c_int = 0;
pub const ICE_REQ_DESC_MULTIPLE: c_int = 32;
pub const ICE_MIN_NUM_DESC: c_int = 64;
pub const ICE_MAX_NUM_DESC_E810: c_int = 8160;
pub const ICE_MAX_NUM_DESC_E830: c_int = 8096;

pub const ICE_DFLT_MIN_RX_DESC: c_int = 512;
pub const ICE_DFLT_NUM_TX_DESC: c_int = 256;
pub const ICE_DFLT_NUM_RX_DESC: c_int = 2048;

pub const ICE_AQ_LEN: c_int = 192;
pub const ICE_MBXSQ_LEN: c_int = 64;
pub const ICE_SBQ_LEN: c_int = 64;
pub const ICE_MIN_LAN_TXRX_MSIX: c_int = 1;
pub const ICE_MIN_LAN_OICR_MSIX: c_int = 1;

pub const ICE_FDIR_MSIX: c_int = 2;
pub const ICE_NO_VSI: c_uint = 0xffff;
pub const ICE_VSI_MAP_CONTIG: c_int = 0;
pub const ICE_VSI_MAP_SCATTER: c_int = 1;
pub const ICE_MAX_SCATTER_TXQS: c_int = 16;
pub const ICE_MAX_SCATTER_RXQS: c_int = 16;
pub const ICE_Q_WAIT_RETRY_LIMIT: c_int = 10;

pub const ICE_MAX_LG_RSS_QS: c_int = 256;
pub const ICE_INVAL_Q_INDEX: c_uint = 0xffff;

pub const ICE_CHNL_START_TC: c_int = 1;
pub const ICE_MAX_RESET_WAIT: c_int = 20;

pub const ICE_MAX_TSO_SIZE: c_int = 131072;

// Minimum BW limit is 500 Kbps for any scheduler node
pub const ICE_MIN_BW_LIMIT: c_int = 500;
// User can specify BW in either Kbit/Mbit/Gbit and OS converts it in bytes.
// use it to convert user specified BW limit into Kbps
//
pub const ICE_BW_KBPS_DIVISOR: c_int = 125;
// Default recipes have priority 4 and below, hence priority values between 5..7
// can be used as filter priority for advanced switch filter (advanced switch
// filters need new recipe to be created for specified extraction sequence
// because default recipe extraction sequence does not represent custom
// extraction)
//
pub const ICE_SWITCH_FLTR_PRIO_QUEUE: c_int = 7;
// prio 6 is reserved for future use (e.g. switch filter with L3 fields +
// (Optional: IP TOS/TTL) + L4 fields + (optionally: TCP fields such as
// SYN/FIN/RST))
//
pub const ICE_SWITCH_FLTR_PRIO_RSVD: c_int = 6;
pub const ICE_SWITCH_FLTR_PRIO_VSI: c_int = 5;

// Macro for each VSI in a PF

// Macros for each Tx/Xdp/Rx ring in a VSI

// Macros for each allocated Tx/Rx ring whether used or not in a VSI

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_feature {
    ICE_F_DSCP,
    ICE_F_PHY_RCLK,
    ICE_F_SMA_CTRL,
    ICE_F_CGU,
    ICE_F_GNSS,
    ICE_F_TXTIME,
    ICE_F_GCS,
    ICE_F_ROCE_LAG,
    ICE_F_SRIOV_LAG,
    ICE_F_SRIOV_AA_LAG,
    ICE_F_MBX_LIMIT,
    ICE_F_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_channel {
    pub list: list_head,
    pub type: u8,
    pub sw_id: u16,
    pub base_q: u16,
    pub num_rxq: u16,
    pub num_txq: u16,
    pub vsi_num: u16,
    pub ena_tc: u8,
    pub info: ice_aqc_vsi_props,
    pub max_tx_rate: u64,
    pub min_tx_rate: u64,
    pub num_sb_fltr: core::sync::atomic::AtomicI32,
    pub ch_vsi: *mut ice_vsi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_txq_meta {
    pub /: *mut *mut u32 q_teid; / Tx-scheduler element identifier,
    pub /: *mut *mut u16 q_id; / Entry in VSI's txq_map bitmap,
    pub /: *mut *mut u16 q_handle; / Relative index of Tx queue within TC,
    pub /: *mut *mut u16 vsi_idx; / VSI index that Tx queue belongs to,
    pub /: *mut *mut u8 tc; / TC number that Tx queue belongs to,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_info {
    pub qoffset: u16,
    pub qcount_tx: u16,
    pub qcount_rx: u16,
    pub netdev_tc: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_tc_cfg {
    pub /: *mut *mut u8 numtc; / Total number of enabled TCs,
    pub /: *mut *mut u16 ena_tc; / Tx map,
    pub tc_info: [ice_tc_info; ICE_MAX_TRAFFIC_CLASS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_qs_cfg {
    pub /: *mut *mut *mut mutex qs_mutex; / will be assigned to &pf->avail_q_mutex,
    pub pf_map: *mut c_ulong,
    pub pf_map_size: c_ulong,
    pub q_count: c_uint,
    pub scatter_count: c_uint,
    pub vsi_map: *mut u16,
    pub vsi_map_offset: u16,
    pub mapping_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_sw {
    pub pf: *mut ice_pf,
    pub /: *mut *mut u16 sw_id; / switch ID for this switch,
    pub /: *mut *mut u16 bridge_mode; / VEB/VEPA/Port Virtualizer,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_pf_state {
    ICE_TESTING,
    ICE_DOWN,
    ICE_NEEDS_RESTART,
    ICE_PREPARED_FOR_RESET,	/* set by driver when prepared */
    ICE_RESET_OICR_RECV,		/* set by driver after rcv reset OICR */
    ICE_PFR_REQ,		/* set by driver */
    ICE_CORER_REQ,		/* set by driver */
    ICE_GLOBR_REQ,		/* set by driver */
    ICE_CORER_RECV,		/* set by OICR handler */
    ICE_GLOBR_RECV,		/* set by OICR handler */
    ICE_EMPR_RECV,		/* set by OICR handler */
    ICE_SUSPENDED,		/* set on module remove path */
    ICE_RESET_FAILED,		/* set by reset/rebuild */
// When checking for the PF to be in a nominal operating state, the
// bits that are grouped at the beginning of the list need to be
// checked. Bits occurring before ICE_STATE_NOMINAL_CHECK_BITS will
// be checked. If you need to add a bit into consideration for nominal
// operating state, it must be added before
// ICE_STATE_NOMINAL_CHECK_BITS. Do not move this entry's position
// without appropriate consideration.
//
    ICE_STATE_NOMINAL_CHECK_BITS,
    ICE_ADMINQ_EVENT_PENDING,
    ICE_MAILBOXQ_EVENT_PENDING,
    ICE_SIDEBANDQ_EVENT_PENDING,
    ICE_MDD_EVENT_PENDING,
    ICE_VFLR_EVENT_PENDING,
    ICE_FLTR_OVERFLOW_PROMISC,
    ICE_VF_DIS,
    ICE_CFG_BUSY,
    ICE_SERVICE_SCHED,
    ICE_SERVICE_DIS,
    ICE_FD_FLUSH_REQ,
    ICE_OICR_INTR_DIS,		/* Global OICR interrupt disabled */
    ICE_MDD_VF_PRINT_PENDING,	/* set when MDD event handle */
    ICE_VF_RESETS_DISABLED,	/* disable resets during ice_remove */
    ICE_LINK_DEFAULT_OVERRIDE_PENDING,
    ICE_PHY_INIT_COMPLETE,
    ICE_FD_VF_FLUSH_CTX,		/* set at FD Rx IRQ or timeout */
    ICE_AUX_ERR_PENDING,
    ICE_STATE_NBITS		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_vsi_state {
    ICE_VSI_DOWN,
    ICE_VSI_NEEDS_RESTART,
    ICE_VSI_NETDEV_ALLOCD,
    ICE_VSI_NETDEV_REGISTERED,
    ICE_VSI_UMAC_FLTR_CHANGED,
    ICE_VSI_MMAC_FLTR_CHANGED,
    ICE_VSI_PROMISC_CHANGED,
    ICE_VSI_REBUILD_PENDING,
    ICE_VSI_STATE_NBITS		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsi_stats {
    pub /: *mut *mut *mut *mut ice_ring_stats tx_ring_stats; / Tx ring stats array,
    pub /: *mut *mut *mut *mut ice_ring_stats rx_ring_stats; / Rx ring stats array,
}

// struct that defines a VSI, associated with a dev
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_vsi {
    pub netdev: *mut net_device,
    pub /: *mut *mut *mut ice_sw vsw; / switch this VSI is on,
    pub /: *mut *mut *mut ice_pf back; / back pointer to PF,
    pub /: *mut *mut *mut *mut ice_rx_ring rx_rings; / Rx ring array,
    pub /: *mut *mut *mut *mut ice_tx_ring tx_rings; / Tx ring array,
    pub /: *mut *mut *mut *mut ice_q_vector q_vectors; / q_vector array,
    pub data): *mut *mut irqreturn_t (irq_handler)(int irq, void,
    pub tx_linearize: u64,
    pub ICE_VSI_STATE_NBITS): DECLARE_BITMAP(state,,
    pub current_netdev_flags: c_uint,
    pub tx_restart: u32,
    pub tx_busy: u32,
    pub rx_buf_failed: u32,
    pub rx_page_failed: u32,
    pub num_q_vectors: u16,
// tell if only dynamic irq allocation is allowed
    pub irq_dyn_alloc: bool,
    pub hsplit:1: bool,
    pub /: *mut *mut u16 vsi_num; / HW (absolute) index of this VSI,
    pub /: *mut *mut u16 idx; / software index in pf->vsi[],
    pub num_gfltr: u16,
    pub num_bfltr: u16,
// RSS config
    pub /: *mut *mut u16 rss_table_size; / HW RSS table size,
    pub /: *mut *mut u16 rss_size; / Allocated RSS queues,
    pub /: *mut *mut u8 rss_hfunc; / User configured hash type,
    pub /: *mut *mut *mut u8 rss_hkey_user; / User configured hash keys,
    pub /: *mut *mut *mut u8 rss_lut_user; / User configured lookup table entries,
    pub /: *mut *mut u8 rss_lut_type; / used to configure Get/Set RSS LUT AQ call,
// aRFS members only allocated for the PF VSI
pub const ICE_MAX_ARFS_LIST: c_int = 1024;

    pub arfs_fltr_list: *mut hlist_head,
    pub arfs_fltr_cntrs: *mut ice_arfs_active_fltr_cntrs,
    pub /: *mut *mut spinlock_t arfs_lock; / protects aRFS hash table and filter state,
    pub arfs_last_fltr_id: *mut core::sync::atomic::AtomicI32,
    pub max_frame: u16,
    pub /: *mut *mut ice_aqc_vsi_props info; / VSI properties,
    pub /: *mut *mut ice_vsi_vlan_info vlan_info; / vlan config to be restored,
// VSI stats
    pub net_stats: rtnl_link_stats64,
    pub net_stats_prev: rtnl_link_stats64,
    pub eth_stats: ice_eth_stats,
    pub eth_stats_prev: ice_eth_stats,
    pub /: *mut *mut list_head tmp_sync_list; / MAC filters to be synced,
    pub /: *mut *mut list_head tmp_unsync_list; / MAC filters to be unsynced,
    pub irqs_ready:1: u8,
    pub /: *mut *mut u8 current_isup:1; / Sync 'link up' logging,
    pub stat_offsets_loaded:1: u8,
    pub inner_vlan_ops: ice_vsi_vlan_ops,
    pub outer_vlan_ops: ice_vsi_vlan_ops,
    pub num_vlan: u16,
// queue information
    pub /: *mut *mut u8 tx_mapping_mode; / ICE_MAP_MODE_[CONTIG|SCATTER],
    pub /: *mut *mut u8 rx_mapping_mode; / ICE_MAP_MODE_[CONTIG|SCATTER],
    pub /: *mut *mut *mut u16 txq_map; / index in pf->avail_txqs,
    pub /: *mut *mut *mut u16 rxq_map; / index in pf->avail_rxqs,
    pub /: *mut *mut u16 alloc_txq; / Allocated Tx queues,
    pub /: *mut *mut u16 num_txq; / Used Tx queues,
    pub /: *mut *mut u16 alloc_rxq; / Allocated Rx queues,
    pub /: *mut *mut u16 num_rxq; / Used Rx queues,
    pub /: *mut *mut u16 req_txq; / User requested Tx queues,
    pub /: *mut *mut u16 req_rxq; / User requested Rx queues,
    pub num_rx_desc: u16,
    pub num_tx_desc: u16,
    pub tc_cfg: ice_tc_cfg,
    pub xdp_prog: *mut bpf_prog,
    pub /: *mut *mut *mut *mut ice_tx_ring xdp_rings; / XDP ring array,
    pub /: *mut *mut u16 num_xdp_txq; / Used XDP queues,
    pub /: *mut *mut u8 xdp_mapping_mode; / ICE_MAP_MODE_[CONTIG|SCATTER],
    pub xdp_state_lock: mutex,
    pub target_netdevs: *mut net_device,
    pub /: *mut *mut tc_mqprio_qopt_offload mqprio_qopt; / queue parameters,
// Channel Specific Fields
    pub tc_map_vsi: [*mut ice_vsi; ICE_CHNL_MAX_TC],
    pub cnt_q_avail: u16,
    pub /: *mut *mut u16 next_base_q; / next queue to be used for channel setup,
    pub ch_list: list_head,
    pub num_chnl_rxq: u16,
    pub num_chnl_txq: u16,
    pub ch_rss_size: u16,
    pub num_chnl_fltr: u16,
// store away rss size info before configuring ADQ channels so that,
// it can be used after tc-qdisc delete, to get back RSS setting as
// they were before
//
    pub orig_rss_size: u16,
// this keeps tracks of all enabled TC with and without DCB
// and inclusive of ADQ, vsi->mqprio_opt keeps track of queue
// information
//
    pub all_numtc: u8,
    pub all_enatc: u16,
// store away TC info, to be used for rebuild logic
    pub old_numtc: u8,
    pub old_ena_tc: u16,
// setup back reference, to which aggregator node this VSI
// corresponds to
//
    pub agg_node: *mut ice_agg_node,
    pub /: *mut *mut *mut ice_port_info port_info; / back pointer to port_info,
    pub /: *mut *mut *mut ice_channel ch; / VSI's channel structure, may be NULL,
// VF associated with this VSI, may be NULL
    pub vf: *mut ice_vf,
// SF associated with this VSI, may be NULL
    pub sf: *mut ice_dynamic_port,
}

// struct that defines an interrupt vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_q_vector {
    pub vsi: *mut ice_vsi,
    pub /: *mut *mut u16 v_idx; / index in the vsi->q_vector array.,
    pub /: *mut *mut u16 reg_idx; / PF relative register index,
    pub /: *mut *mut u8 num_ring_rx; / total number of Rx rings in vector,
    pub /: *mut *mut u8 num_ring_tx; / total number of Tx rings in vector,
    pub /: *mut *mut u8 wb_on_itr:1; / if true, WB on ITR is enabled,
// in usecs, need to use ice_intrl_to_usecs_reg() before writing this
// value to the device
//
    pub intrl: u8,
    pub napi: napi_struct,
    pub rx: ice_ring_container,
    pub tx: ice_ring_container,
    pub ch: *mut ice_channel,
    pub name: [c_char; ICE_INT_NAME_STR_LEN],
    pub /: *mut *mut u16 total_events; / net_dim(): number of interrupts processed,
    pub /: *mut *mut u16 vf_reg_idx; / VF relative register index,
    pub irq: msi_map,
    pub ____cacheline_internodealigned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_pf_flags {
    ICE_FLAG_FLTR_SYNC,
    ICE_FLAG_RDMA_ENA,
    ICE_FLAG_RSS_ENA,
    ICE_FLAG_SRIOV_ENA,
    ICE_FLAG_SRIOV_CAPABLE,
    ICE_FLAG_DCB_CAPABLE,
    ICE_FLAG_DCB_ENA,
    ICE_FLAG_FD_ENA,
    ICE_FLAG_PTP_SUPPORTED,		/* PTP is supported by NVM */
    ICE_FLAG_ADV_FEATURES,
    ICE_FLAG_TC_MQPRIO,		/* support for Multi queue TC */
    ICE_FLAG_CLS_FLOWER,
    ICE_FLAG_LINK_DOWN_ON_CLOSE_ENA,
    ICE_FLAG_TOTAL_PORT_SHUTDOWN_ENA,
    ICE_FLAG_NO_MEDIA,
    ICE_FLAG_FW_LLDP_AGENT,
    ICE_FLAG_MOD_POWER_UNSUPPORTED,
    ICE_FLAG_PHY_FW_LOAD_FAILED,
    ICE_FLAG_ETHTOOL_CTXT,		/* set when ethtool holds RTNL lock */
    ICE_FLAG_VF_TRUE_PROMISC_ENA,
    ICE_FLAG_MDD_AUTO_RESET_VF,
    ICE_FLAG_VF_VLAN_PRUNING,
    ICE_FLAG_LINK_LENIENT_MODE_ENA,
    ICE_FLAG_PLUG_AUX_DEV,
    ICE_FLAG_UNPLUG_AUX_DEV,
    ICE_FLAG_AUX_DEV_CREATED,
    ICE_FLAG_MTU_CHANGED,
    ICE_FLAG_GNSS,			/* GNSS successfully initialized */
    ICE_FLAG_DPLL,			/* SyncE/PTP dplls initialized */
    ICE_FLAG_LLDP_AQ_FLTR,
    ICE_PF_FLAGS_NBITS		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_misc_thread_tasks {
    ICE_MISC_THREAD_TX_TSTAMP,
    ICE_MISC_THREAD_NBITS		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_eswitch {
    pub uplink_vsi: *mut ice_vsi,
    pub br_offloads: *mut ice_esw_br_offloads,
    pub reprs: xarray,
    pub is_running: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_agg_node {
    pub agg_id: u32,
pub const ICE_MAX_VSIS_IN_AGG_NODE: c_int = 64;
    pub num_vsis: u32,
    pub valid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pf_msix {
    pub cur: u32,
    pub min: u32,
    pub max: u32,
    pub total: u32,
    pub rest: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_pf {
    pub pdev: *mut pci_dev,
    pub adapter: *mut ice_adapter,
    pub nvm_region: *mut devlink_region,
    pub sram_region: *mut devlink_region,
    pub devcaps_region: *mut devlink_region,
// devlink port data
    pub devlink_port: devlink_port,
// OS reserved IRQ details
    pub irq_tracker: ice_irq_tracker,
    pub virt_irq_tracker: ice_virt_irq_tracker,
    pub /: *mut *mut u16 ctrl_vsi_idx; / control VSI index in pf->vsi array,
    pub /: *mut *mut *mut *mut ice_vsi vsi; / VSIs created by the driver,
    pub vsi_stats: *mut ice_vsi_stats,
    pub /: *mut *mut *mut ice_sw first_sw; / first switch created by firmware,
    pub /: *mut *mut u16 eswitch_mode; / current mode of eswitch,
    pub ice_debugfs_pf: *mut dentry,
    pub vfs: ice_vfs,
    pub ICE_F_MAX): DECLARE_BITMAP(features,,
    pub ICE_STATE_NBITS): DECLARE_BITMAP(state,,
    pub ICE_PF_FLAGS_NBITS): DECLARE_BITMAP(flags,,
    pub ICE_MISC_THREAD_NBITS): DECLARE_BITMAP(misc_thread,,
    pub /: *mut *mut *mut unsigned long avail_txqs; / bitmap to track PF Tx queue usage,
    pub /: *mut *mut *mut unsigned long avail_rxqs; / bitmap to track PF Rx queue usage,
    pub /: *mut *mut *mut unsigned long txtime_txqs; / bitmap to track PF Tx Time queue,
    pub serv_tmr_period: c_ulong,
    pub serv_tmr_prev: c_ulong,
    pub serv_tmr: timer_list,
    pub serv_task: work_struct,
    pub /: *mut *mut mutex avail_q_mutex; / protects access to avail_[rx|tx]qs,
    pub /: *mut *mut mutex sw_mutex; / lock for protecting VSI alloc flow,
    pub /: *mut *mut mutex tc_mutex; / lock to protect TC changes,
    pub /: *mut *mut mutex adev_mutex; / lock to protect aux device access,
    pub /: *mut *mut mutex lag_mutex; / protect ice_lag in PF,
    pub msg_enable: u32,
    pub ptp: ice_ptp,
    pub gnss_serial: *mut gnss_serial,
    pub gnss_dev: *mut gnss_device,
    pub /: *mut *mut u16 num_rdma_msix; / Total MSIX vectors for RDMA driver,
// spinlock to protect the AdminQ wait list
    pub aq_wait_lock: spinlock_t,
    pub aq_wait_list: hlist_head,
    pub aq_wait_queue: wait_queue_head_t,
    pub fw_emp_reset_disabled: bool,
    pub reset_wait_queue: wait_queue_head_t,
    pub hw_csum_rx_error: u32,
    pub hw_rx_eipe_error: u32,
    pub oicr_err_reg: u32,
    pub /: *mut *mut msi_map oicr_irq; / Other interrupt cause MSIX vector,
    pub /: *mut *mut msi_map ll_ts_irq; / LL_TS interrupt MSIX vector,
    pub /: *mut *mut u16 max_pf_txqs; / Total Tx queues PF wide,
    pub /: *mut *mut u16 max_pf_rxqs; / Total Rx queues PF wide,
    pub msix: ice_pf_msix,
    pub /: *mut *mut u16 num_lan_tx; / num LAN Tx queues setup,
    pub /: *mut *mut u16 num_lan_rx; / num LAN Rx queues setup,
    pub /: *mut *mut u16 next_vsi; / Next free slot in pf->vsi[] - 0-based!,
    pub num_alloc_vsi: u16,
    pub /: *mut *mut u16 corer_count; / Core reset count,
    pub /: *mut *mut u16 globr_count; / Global reset count,
    pub /: *mut *mut u16 empr_count; / EMP reset count,
    pub /: *mut *mut u16 pfr_count; / PF reset count,
    pub link_down_events: u32,
    pub /: *mut *mut u8 wol_ena : 1; / software state of WoL,
    pub /: *mut *mut u32 wakeup_reason; / last wakeup reason,
    pub stats: ice_hw_port_stats,
    pub stats_prev: ice_hw_port_stats,
    pub hw: ice_hw,
    pub /: *mut *mut u8 stat_prev_loaded:1; / has previous stats been loaded,
    pub dcbx_cap: u16,
    pub tx_timeout_count: u32,
    pub tx_timeout_last_recovery: c_ulong,
    pub tx_timeout_recovery_level: u32,
    pub int_name: [c_char; ICE_INT_NAME_STR_LEN],
    pub int_name_ll_ts: [c_char; ICE_INT_NAME_STR_LEN],
    pub aux_idx: c_int,
    pub sw_int_count: u32,
// count of tc_flower filters specific to channel (aka where filter
// action is "hw_tc <tc_num>")
//
    pub num_dmac_chnl_fltrs: u16,
    pub tc_flower_fltr_list: hlist_head,
    pub supported_rxdids: u64,
    pub /: *mut *mut __le64 nvm_phy_type_lo; / NVM PHY type low,
    pub /: *mut *mut __le64 nvm_phy_type_hi; / NVM PHY type high,
    pub link_dflt_override: ice_link_default_override_tlv,
    pub /: *mut *mut *mut ice_lag lag; / Link Aggregation information,
    pub eswitch: ice_eswitch,
    pub br_port: *mut ice_esw_br_port,
    pub dyn_ports: xarray,
    pub sf_nums: xarray,
pub const ICE_INVALID_AGG_NODE_ID: c_int = 0;
pub const ICE_PF_AGG_NODE_ID_START: c_int = 1;
pub const ICE_MAX_PF_AGG_NODES: c_int = 32;
    pub pf_agg_node: [ice_agg_node; ICE_MAX_PF_AGG_NODES],
pub const ICE_VF_AGG_NODE_ID_START: c_int = 65;
pub const ICE_MAX_VF_AGG_NODES: c_int = 32;
    pub vf_agg_node: [ice_agg_node; ICE_MAX_VF_AGG_NODES],
    pub dplls: ice_dplls,
    pub hwmon_dev: *mut device,
    pub health_reporters: ice_health,
    pub cdev_info: *mut iidc_rdma_core_dev_info,
    pub num_quanta_prof_used: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_netdev_priv {
    pub vsi: *mut ice_vsi,
    pub repr: *mut ice_repr,
// indirect block callbacks on registered higher level devices
// (e.g. tunnel devices)
//
// tc_indr_block_cb_priv_list is used to look up indirect callback
// private data
//
    pub tc_indr_block_priv_list: list_head,
}

//
// ice_vector_ch_enabled
// @qv: pointer to q_vector, can be NULL
//
// This function returns true if vector is channel enabled otherwise false
//
// ice_ptp_pf_handles_tx_interrupt - Check if PF handles Tx interrupt
// @pf: Board private structure
//
// Return true if this PF should respond to the Tx timestamp interrupt
// indication in the miscellaneous OICR interrupt handler.
//
// ice_irq_dynamic_ena - Enable default interrupt generation settings
// @hw: pointer to HW struct
// @vsi: pointer to VSI struct, can be NULL
// @q_vector: pointer to q_vector, can be NULL
//
// clear the PBA here, as this function is meant to clean out all
// previous interrupts and enable the interrupt
//
// ice_netdev_to_pf - Retrieve the PF struct associated with a netdev
// @netdev: pointer to the netdev struct
//
// ice_is_txtime_ena - check if Tx Time is enabled on the Tx ring
// @ring: pointer to Tx ring
//
// Return: true if the Tx ring has Tx Time enabled, false otherwise.
//
extern "C" {
    pub fn test_bit(_arg: ring->q_index, _arg: pf->txtime_txqs) -> return;
}
//
// ice_is_txtime_cfg - check if Tx Time is configured on the Tx ring
// @ring: pointer to Tx ring
//
// Return: true if the Tx ring is configured for Tx ring, false otherwise.
//
extern "C" {
    pub fn test_bit(_arg: ICE_TX_RING_FLAGS_TXTIME, _arg: ring->flags) -> return;
}
//
// ice_get_xp_from_qid - get ZC XSK buffer pool bound to a queue ID
// @vsi: pointer to VSI
// @qid: index of a queue to look at XSK buff pool presence
//
// Return: A pointer to xsk_buff_pool structure if there is a buffer pool
// attached and configured as zero-copy, NULL otherwise.
//
// ice_rx_xsk_pool - assign XSK buff pool to Rx ring
// @ring: Rx ring to use
//
// Sets XSK buff pool pointer on Rx ring.
//
// ice_tx_xsk_pool - assign XSK buff pool to XDP ring
// @vsi: pointer to VSI
// @qid: index of a queue to look at XSK buff pool presence
//
// Sets XSK buff pool pointer on XDP ring.
//
// XDP ring is picked from Rx ring, whereas Rx ring is picked based on provided
// queue id. Reason for doing so is that queue vectors might have assigned more
// than one XDP ring, e.g. when user reduced the queue count on netdev; Rx ring
// carries a pointer to one of these XDP rings for its own purposes, such as
// handling XDP_TX action, therefore we can piggyback here on the
// rx_ring->xdp_ring assignment that was done during XDP rings initialization.
//
// ice_get_max_txq - return the maximum number of Tx queues for in a PF
// @pf: PF structure
//
// Return: maximum number of Tx queues
//
extern "C" {
    pub fn min(_arg: num_online_cpus(), _arg: pf->hw.func_caps.common_cap.num_txq) -> return;
}
//
// ice_get_max_rxq - return the maximum number of Rx queues for in a PF
// @pf: PF structure
//
// Return: maximum number of Rx queues
//
extern "C" {
    pub fn min(_arg: num_online_cpus(), _arg: pf->hw.func_caps.common_cap.num_rxq) -> return;
}
//
// ice_get_main_vsi - Get the PF VSI
// @pf: PF instance
//
// returns pf->vsi[0], which by definition is the PF VSI
//
// ice_get_netdev_priv_vsi - return VSI associated with netdev priv.
// @np: private netdev structure
//
// In case of port representor return source port VSI.
//
// ice_get_ctrl_vsi - Get the control VSI
// @pf: PF instance
//
// if pf->ctrl_vsi_idx is ICE_NO_VSI, control VSI was not set up
//
// ice_find_vsi - Find the VSI from VSI ID
// @pf: The PF pointer to search in
// @vsi_num: The VSI ID to search for
//
// ice_is_switchdev_running - check if switchdev is configured
// @pf: pointer to PF structure
//
// Returns true if eswitch mode is set to DEVLINK_ESWITCH_MODE_SWITCHDEV
// and switchdev is configured, false otherwise.
//
pub const ICE_FD_STAT_CTR_BLOCK_COUNT: c_int = 256;

pub const ICE_FD_STAT_CH: c_int = 1;

//
// ice_is_adq_active - any active ADQs
// @pf: pointer to PF
//
// This function returns true if there are any ADQs configured (which is
// determined by looking at VSI type (which should be VSI_PF), numtc, and
// TC_MQPRIO flag) otherwise return false
//
// is ADQ configured
extern "C" {
    pub fn ice_debugfs_pf_init(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_debugfs_pf_deinit(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_debugfs_init();
}
extern "C" {
    pub fn ice_debugfs_exit();
}
extern "C" {
    pub fn netif_is_ice(dev: *const net_device) -> bool;
}
extern "C" {
    pub fn ice_vsi_setup_tx_rings(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_setup_rx_rings(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_open_ctrl(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_open(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn ice_set_ethtool_repr_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn ice_set_ethtool_safe_mode_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn ice_set_ethtool_sf_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn ice_get_avail_txq_count(pf: *mut ice_pf) -> u16;
}
extern "C" {
    pub fn ice_get_avail_rxq_count(pf: *mut ice_pf) -> u16;
}
extern "C" {
    pub fn ice_vsi_recfg_qs(vsi: *mut ice_vsi, new_rx: c_int, new_tx: c_int, locked: bool) -> c_int;
}
extern "C" {
    pub fn ice_update_vsi_stats(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_update_pf_stats(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_up(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_down(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_down_up(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_vsi_cfg_lan(vsi: *mut ice_vsi) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_xdp_cfg {
    ICE_XDP_CFG_FULL,	/* Fully apply new config in .ndo_bpf() */
    ICE_XDP_CFG_PART,	/* Save/use part of config in VSI rebuild */
}

extern "C" {
    pub fn ice_vsi_determine_xdp_res(vsi: *mut ice_vsi) -> c_int;
}
extern "C" {
    pub fn ice_destroy_xdp_rings(vsi: *mut ice_vsi, cfg_type: ice_xdp_cfg) -> c_int;
}
extern "C" {
    pub fn ice_map_xdp_rings(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_get_rss(vsi: *mut ice_vsi, seed: *mut u8, lut: *mut u8, lut_size: u16) -> c_int;
}
extern "C" {
    pub fn ice_set_rss_lut(vsi: *mut ice_vsi, lut: *mut u8, lut_size: u16) -> c_int;
}
extern "C" {
    pub fn ice_get_rss_lut(vsi: *mut ice_vsi, lut: *mut u8, lut_size: u16) -> c_int;
}
extern "C" {
    pub fn ice_set_rss_key(vsi: *mut ice_vsi, seed: *mut u8) -> c_int;
}
extern "C" {
    pub fn ice_get_rss_key(vsi: *mut ice_vsi, seed: *mut u8) -> c_int;
}
extern "C" {
    pub fn ice_set_rss_hfunc(vsi: *mut ice_vsi, hfunc: u8) -> c_int;
}
extern "C" {
    pub fn ice_fill_rss_lut(lut: *mut u8, rss_table_size: u16, rss_size: u16);
}
extern "C" {
    pub fn ice_schedule_reset(pf: *mut ice_pf, reset: ice_reset_req) -> c_int;
}
extern "C" {
    pub fn ice_print_link_msg(vsi: *mut ice_vsi, isup: bool);
}
extern "C" {
    pub fn ice_plug_aux_dev(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_unplug_aux_dev(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_rdma_finalize_setup(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_init_rdma(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_deinit_rdma(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_is_wol_supported(hw: *mut ice_hw) -> bool;
}
extern "C" {
    pub fn ice_fdir_del_all_fltrs(vsi: *mut ice_vsi);
}
extern "C" {
    pub fn ice_vsi_manage_fdir(vsi: *mut ice_vsi, ena: bool);
}
extern "C" {
    pub fn ice_add_fdir_ethtool(vsi: *mut ice_vsi, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn ice_del_fdir_ethtool(vsi: *mut ice_vsi, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn ice_get_ethtool_fdir_entry(hw: *mut ice_hw, cmd: *mut ethtool_rxnfc) -> c_int;
}
extern "C" {
    pub fn ice_fdir_rem_adq_chnl(hw: *mut ice_hw, vsi_idx: u16);
}
extern "C" {
    pub fn ice_fdir_release_flows(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_fdir_replay_flows(hw: *mut ice_hw);
}
extern "C" {
    pub fn ice_fdir_replay_fltrs(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_fdir_create_dflt_rules(pf: *mut ice_pf) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ice_aq_task_state {
    ICE_AQ_TASK_NOT_PREPARED,
    ICE_AQ_TASK_WAITING,
    ICE_AQ_TASK_COMPLETE,
    ICE_AQ_TASK_CANCELED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ice_aq_task {
    pub entry: hlist_node,
    pub event: ice_rq_event_info,
    pub state: ice_aq_task_state,
    pub opcode: u16,
}

extern "C" {
    pub fn ice_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ice_open_internal(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ice_stop(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn ice_service_task_schedule(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_start_service_task(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_load(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_unload(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_adv_lnk_speed_maps_init();
}
extern "C" {
    pub fn ice_init_dev_hw(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_init_dev(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_deinit_dev(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_init_pf(pf: *mut ice_pf) -> c_int;
}
extern "C" {
    pub fn ice_deinit_pf(pf: *mut ice_pf);
}
extern "C" {
    pub fn ice_change_mtu(netdev: *mut net_device, new_mtu: c_int) -> c_int;
}
extern "C" {
    pub fn ice_tx_timeout(netdev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn ice_xdp(dev: *mut net_device, xdp: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn ice_set_netdev_features(netdev: *mut net_device);
}
extern "C" {
    pub fn ice_vlan_rx_add_vid(netdev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
extern "C" {
    pub fn ice_vlan_rx_kill_vid(netdev: *mut net_device, proto: __be16, vid: u16) -> c_int;
}
//
// ice_set_rdma_cap - enable RDMA support
// @pf: PF struct
//
// ice_clear_rdma_cap - disable RDMA support
// @pf: PF struct
//
// defer unplug to service task to avoid RTNL lock and
// clear PLUG bit so that pending plugs don't interfere
//
// ice_is_dual - Check if given config is multi-NAC
// @hw: pointer to HW structure
//
// Return: true if the device is running in mutli-NAC (Network
// Acceleration Complex) configuration variant, false otherwise
// (always false for non-E825 devices).
//
// ice_is_primary - Check if given device belongs to the primary complex
// @hw: pointer to HW structure
//
// Check if given PF/HW is running on primary complex in multi-NAC
// configuration.
//
// Return: true if the device is dual, false otherwise (always true
// for non-E825 devices).
//
// ice_pf_src_tmr_owned - Check if a primary timer is owned by PF
// @pf: pointer to PF structure
//
// Return: true if PF owns primary timer, false otherwise.
//
// ice_get_primary_hw - Get pointer to primary ice_hw structure
// @pf: pointer to PF structure
//
// Return: A pointer to ice_hw structure with access to timesync
// register space.
//
// ice_get_ctrl_pf - Get pointer to Control PF of the adapter
// @pf: pointer to the current PF structure
//
// Return: A pointer to ice_pf structure which is Control PF,
// NULL if it's not initialized yet.
//
