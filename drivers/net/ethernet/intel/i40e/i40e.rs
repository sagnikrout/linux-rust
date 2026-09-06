//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/i40e/i40e.h
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
// Copyright(c) 2013 - 2021 Intel Corporation.

// Useful i40e defaults
pub const I40E_MAX_VEB: c_int = 16;
pub const I40E_MAX_NUM_DESCRIPTORS: c_int = 4096;
pub const I40E_MAX_NUM_DESCRIPTORS_XL710: c_int = 8160;

pub const I40E_DEFAULT_NUM_DESCRIPTORS: c_int = 512;
pub const I40E_REQ_DESCRIPTOR_MULTIPLE: c_int = 32;
pub const I40E_MIN_NUM_DESCRIPTORS: c_int = 64;
pub const I40E_MIN_MSIX: c_int = 2;

// max 16 qps

pub const I40E_DEFAULT_QUEUES_PER_VF: c_int = 4;
pub const I40E_MAX_VF_QUEUES: c_int = 16;

pub const I40E_FDIR_RING_COUNT: c_int = 32;
pub const I40E_MAX_AQ_BUF_SIZE: c_int = 4096;
pub const I40E_AQ_LEN: c_int = 256;
pub const I40E_MIN_ARQ_LEN: c_int = 1;
pub const I40E_MIN_ASQ_LEN: c_int = 2;

pub const I40E_MAX_USER_PRIORITY: c_int = 8;

pub const I40E_QUEUE_WAIT_RETRY_LIMIT: c_int = 10;

pub const I40E_OEM_EETRACK_ID: c_uint = 0xffffffff;

// BW rate limiting

// driver state flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_state {
    __I40E_TESTING,
    __I40E_CONFIG_BUSY,
    __I40E_CONFIG_DONE,
    __I40E_DOWN,
    __I40E_SERVICE_SCHED,
    __I40E_ADMINQ_EVENT_PENDING,
    __I40E_MDD_EVENT_PENDING,
    __I40E_MDD_VF_PRINT_PENDING,
    __I40E_VFLR_EVENT_PENDING,
    __I40E_RESET_RECOVERY_PENDING,
    __I40E_TIMEOUT_RECOVERY_PENDING,
    __I40E_MISC_IRQ_REQUESTED,
    __I40E_RESET_INTR_RECEIVED,
    __I40E_REINIT_REQUESTED,
    __I40E_PF_RESET_REQUESTED,
    __I40E_PF_RESET_AND_REBUILD_REQUESTED,
    __I40E_CORE_RESET_REQUESTED,
    __I40E_GLOBAL_RESET_REQUESTED,
    __I40E_EMP_RESET_INTR_RECEIVED,
    __I40E_SUSPENDED,
    __I40E_PTP_TX_IN_PROGRESS,
    __I40E_BAD_EEPROM,
    __I40E_DOWN_REQUESTED,
    __I40E_FD_FLUSH_REQUESTED,
    __I40E_FD_ATR_AUTO_DISABLED,
    __I40E_FD_SB_AUTO_DISABLED,
    __I40E_RESET_FAILED,
    __I40E_PORT_SUSPENDED,
    __I40E_VF_DISABLE,
    __I40E_MACVLAN_SYNC_PENDING,
    __I40E_TEMP_LINK_POLLING,
    __I40E_CLIENT_SERVICE_REQUESTED,
    __I40E_CLIENT_L2_CHANGE,
    __I40E_CLIENT_RESET,
    __I40E_VIRTCHNL_OP_PENDING,
    __I40E_RECOVERY_MODE,
    __I40E_VF_RESETS_DISABLED,	/* disable resets during i40e_remove */
    __I40E_IN_REMOVE,
    __I40E_VFS_RELEASING,
// This must be last as it determines the size of the BITMAP
    __I40E_STATE_SIZE__,
}

// VSI state flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_vsi_state {
    __I40E_VSI_DOWN,
    __I40E_VSI_NEEDS_RESTART,
    __I40E_VSI_SYNCING_FILTERS,
    __I40E_VSI_OVERFLOW_PROMISC,
    __I40E_VSI_REINIT_REQUESTED,
    __I40E_VSI_DOWN_REQUESTED,
    __I40E_VSI_RELEASING,
// This must be last as it determines the size of the BITMAP
    __I40E_VSI_STATE_SIZE__,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_pf_flags {
    I40E_FLAG_MSI_ENA,
    I40E_FLAG_MSIX_ENA,
    I40E_FLAG_RSS_ENA,
    I40E_FLAG_VMDQ_ENA,
    I40E_FLAG_SRIOV_ENA,
    I40E_FLAG_DCB_CAPABLE,
    I40E_FLAG_DCB_ENA,
    I40E_FLAG_FD_SB_ENA,
    I40E_FLAG_FD_ATR_ENA,
    I40E_FLAG_MFP_ENA,
    I40E_FLAG_HW_ATR_EVICT_ENA,
    I40E_FLAG_VEB_MODE_ENA,
    I40E_FLAG_VEB_STATS_ENA,
    I40E_FLAG_LINK_POLLING_ENA,
    I40E_FLAG_TRUE_PROMISC_ENA,
    I40E_FLAG_LEGACY_RX_ENA,
    I40E_FLAG_PTP_ENA,
    I40E_FLAG_IWARP_ENA,
    I40E_FLAG_LINK_DOWN_ON_CLOSE_ENA,
    I40E_FLAG_SOURCE_PRUNING_DIS,
    I40E_FLAG_TC_MQPRIO_ENA,
    I40E_FLAG_FD_SB_INACTIVE,
    I40E_FLAG_FD_SB_TO_CLOUD_FILTER,
    I40E_FLAG_FW_LLDP_DIS,
    I40E_FLAG_RS_FEC,
    I40E_FLAG_BASE_R_FEC,
// TOTAL_PORT_SHUTDOWN_ENA
// Allows to physically disable the link on the NIC's port.
// If enabled, (after link down request from the OS)
// no link, traffic or led activity is possible on that port.
//
// If I40E_FLAG_TOTAL_PORT_SHUTDOWN_ENA is set, the
// I40E_FLAG_LINK_DOWN_ON_CLOSE_ENA must be explicitly forced
// to true and cannot be disabled by system admin at that time.
// The functionalities are exclusive in terms of configuration, but
// they also have similar behavior (allowing to disable physical
// link of the port), with following differences:
// - LINK_DOWN_ON_CLOSE_ENA is configurable at host OS run-time and
// is supported by whole family of 7xx Intel Ethernet Controllers
// - TOTAL_PORT_SHUTDOWN_ENA may be enabled only before OS loads
// (in BIOS) only if motherboard's BIOS and NIC's FW has support of it
// - when LINK_DOWN_ON_CLOSE_ENABLED is used, the link is being brought
// down by sending phy_type=0 to NIC's FW
// - when TOTAL_PORT_SHUTDOWN_ENA is used, phy_type is not altered,
// instead the link is being brought down by clearing
// bit (I40E_AQ_PHY_ENABLE_LINK) in abilities field of
// i40e_aq_set_phy_config structure
//
    I40E_FLAG_TOTAL_PORT_SHUTDOWN_ENA,
    I40E_FLAG_VF_VLAN_PRUNING_ENA,
    I40E_FLAG_MDD_AUTO_RESET_VF,
    I40E_PF_FLAGS_NBITS,		/* must be last */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_interrupt_policy {
    I40E_INTERRUPT_BEST_CASE,
    I40E_INTERRUPT_MEDIUM,
    I40E_INTERRUPT_LOWEST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_lump_tracking {
    pub num_entries: u16,
    pub list: [u16; ],
pub const I40E_PILE_VALID_BIT: c_uint = 0x8000;

}

pub const I40E_DEFAULT_ATR_SAMPLE_RATE: c_int = 20;
pub const I40E_FDIR_MAX_RAW_PACKET_SIZE: c_int = 512;
pub const I40E_FDIR_BUFFER_FULL_MARGIN: c_int = 10;
pub const I40E_FDIR_BUFFER_HEAD_ROOM: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_fd_stat_idx {
    I40E_FD_STAT_ATR,
    I40E_FD_STAT_SB,
    I40E_FD_STAT_ATR_TUNNEL,
    I40E_FD_STAT_PF_COUNT
}

// The following structure contains the data parsed from the user-defined
// field of the ethtool_rx_flow_spec structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_rx_flow_userdef {
    pub flex_filter: bool,
    pub flex_word: u16,
    pub flex_offset: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_fdir_filter {
    pub fdir_node: hlist_node,
// filter ipnut set
    pub flow_type: u8,
    pub ipl4_proto: u8,
// TX packet view of src and dst
    pub dst_ip: __be32,
    pub src_ip: __be32,
    pub dst_ip6: [__be32; 4],
    pub src_ip6: [__be32; 4],
    pub src_port: __be16,
    pub dst_port: __be16,
    pub sctp_v_tag: __be32,
    pub vlan_etype: __be16,
    pub vlan_tag: __be16,
// Flexible data to match within the packet payload
    pub flex_word: __be16,
    pub flex_offset: u16,
    pub flex_filter: bool,
// filter control
    pub q_index: u16,
    pub flex_off: u8,
    pub pctype: u8,
    pub dest_vsi: u16,
    pub dest_ctl: u8,
    pub fd_status: u8,
    pub cnt_index: u16,
    pub fd_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_cloud_filter {
    pub cloud_node: hlist_node,
    pub cookie: c_ulong,
// cloud filter input set follows
    pub dst_mac: [u8; ETH_ALEN],
    pub src_mac: [u8; ETH_ALEN],
    pub vlan_id: __be16,
    pub /: *mut *mut u16 seid; / filter control,
    pub dst_port: __be16,
    pub src_port: __be16,
    pub tenant_id: u32,
    pub dst_ip: in_addr,
    pub src_ip: in_addr,
    pub v4: },
    pub dst_ip6: in6_addr,
    pub src_ip6: in6_addr,
    pub v6: },
    pub ip: },

    pub /: *mut *mut u16 n_proto; / Ethernet Protocol,
    pub /: *mut *mut u8 ip_proto; / IPPROTO value,
    pub flags: u8,
pub const I40E_CLOUD_TNL_TYPE_NONE: c_uint = 0xff;
    pub tunnel_type: u8,
}

pub const I40E_DCB_PRIO_TYPE_STRICT: c_int = 0;
pub const I40E_DCB_PRIO_TYPE_ETS: c_int = 1;
pub const I40E_DCB_STRICT_PRIO_CREDITS: c_int = 127;
// DCB per TC information data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_tc_info {
    pub /: *mut *mut u16 qoffset; / Queue offset from base queue,
    pub /: *mut *mut u16 qcount; / Total Queues,
    pub /: *mut *mut u8 netdev_tc; / Netdev TC index if netdev associated,
}

// TC configuration data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_tc_configuration {
    pub /: *mut *mut u8 numtc; / Total number of enabled TCs,
    pub /: *mut *mut u8 enabled_tc; / TC map,
    pub tc_info: [i40e_tc_info; I40E_MAX_TRAFFIC_CLASS],
}

pub const I40E_UDP_PORT_INDEX_UNUSED: c_int = 255;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_udp_port_config {
// AdminQ command interface expects port number in Host byte order
    pub port: u16,
    pub type: u8,
    pub filter_index: u8,
}

// macros related to FLX_PIT

pub const I40E_MAX_FLEX_SRC_OFFSET: c_uint = 0x1F;
// macros related to GLQF_ORT

pub const I40E_L3_GLQF_ORT_IDX: c_int = 34;
pub const I40E_L4_GLQF_ORT_IDX: c_int = 35;
// Flex PIT register index
pub const I40E_FLEX_PIT_IDX_START_L3: c_int = 3;
pub const I40E_FLEX_PIT_IDX_START_L4: c_int = 6;
pub const I40E_FLEX_PIT_TABLE_SIZE: c_int = 3;
pub const I40E_FLEX_DEST_UNUSED: c_int = 63;
pub const I40E_FLEX_INDEX_ENTRIES: c_int = 8;
// Flex MASK to disable all flexible entries

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_flex_pit {
    pub list: list_head,
    pub src_offset: u16,
    pub pit_index: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_fwd_adapter {
    pub netdev: *mut net_device,
    pub bit_no: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_channel {
    pub list: list_head,
    pub initialized: bool,
    pub type: u8,
    pub /: *mut *mut u16 vsi_number; / Assigned VSI number from AQ 'Add VSI' response,
    pub stat_counter_idx: u16,
    pub base_queue: u16,
    pub /: *mut *mut u16 num_queue_pairs; / Requested by user,
    pub seid: u16,
    pub enabled_tc: u8,
    pub info: i40e_aqc_vsi_properties_data,
    pub max_tx_rate: u64,
    pub fwd: *mut i40e_fwd_adapter,
// track this channel belongs to which VSI
    pub parent_vsi: *mut i40e_vsi,
}

// struct that defines the Ethernet device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_pf {
    pub pdev: *mut pci_dev,
    pub devlink_port: devlink_port,
    pub hw: i40e_hw,
    pub __I40E_STATE_SIZE__): DECLARE_BITMAP(state,,
    pub msix_entries: *mut msix_entry,
    pub /: *mut *mut u16 num_vmdq_vsis; / num vmdq vsis this PF has set up,
    pub /: *mut *mut u16 num_vmdq_qps; / num queue pairs per vmdq pool,
    pub /: *mut *mut u16 num_vmdq_msix; / num queue vectors per vmdq pool,
    pub /: *mut *mut u16 num_req_vfs; / num VFs requested for this PF,
    pub /: *mut *mut u16 num_vf_qps; / num queue pairs per VF,
    pub /: *mut *mut u16 num_lan_qps; / num lan queues this PF has set up,
    pub /: *mut *mut u16 num_lan_msix; / num queue vectors for the base PF vsi,
    pub /: *mut *mut u16 num_fdsb_msix; / num queue vectors for sideband Fdir,
    pub /: *mut *mut u16 num_iwarp_msix; / num of iwarp vectors for this PF,
    pub iwarp_base_vector: c_int,
    pub /: *mut *mut int queues_left; / queues left unclaimed,
    pub /: *mut *mut u16 alloc_rss_size; / allocated RSS queues,
    pub /: *mut *mut u16 rss_size_max; / HW defined max RSS queues,
    pub /: *mut *mut u16 fdir_pf_filter_count; / num of guaranteed filters for this PF,
    pub /: *mut *mut u16 num_alloc_vsi; / num VSIs this driver supports,
    pub wol_en: bool,
    pub fdir_filter_list: hlist_head,
    pub fdir_pf_active_filters: u16,
    pub fd_flush_timestamp: c_ulong,
    pub fd_flush_cnt: u32,
    pub fd_add_err: u32,
    pub fd_atr_cnt: u32,
// Book-keeping of side-band filter count per flow-type.
// This is used to detect and handle input set changes for
// respective flow-type.
//
    pub fd_tcp4_filter_cnt: u16,
    pub fd_udp4_filter_cnt: u16,
    pub fd_sctp4_filter_cnt: u16,
    pub fd_ip4_filter_cnt: u16,
    pub fd_tcp6_filter_cnt: u16,
    pub fd_udp6_filter_cnt: u16,
    pub fd_sctp6_filter_cnt: u16,
    pub fd_ip6_filter_cnt: u16,
// Flexible filter table values that need to be programmed into
// hardware, which expects L3 and L4 to be programmed separately. We
// need to ensure that the values are in ascended order and don't have
// duplicates, so we track each L3 and L4 values in separate lists.
//
    pub l3_flex_pit_list: list_head,
    pub l4_flex_pit_list: list_head,
    pub udp_tunnel_shared: udp_tunnel_nic_shared,
    pub udp_tunnel_nic: udp_tunnel_nic_info,
    pub cloud_filter_list: hlist_head,
    pub num_cloud_filters: u16,
    pub rx_itr_default: u16,
    pub tx_itr_default: u16,
    pub msg_enable: u32,
    pub int_name: [c_char; I40E_INT_NAME_STR_LEN],
    pub service_timer_period: c_ulong,
    pub service_timer_previous: c_ulong,
    pub service_timer: timer_list,
    pub service_task: work_struct,
    pub I40E_PF_FLAGS_NBITS): DECLARE_BITMAP(flags,,
    pub cinst: *mut i40e_client_instance,
    pub stat_offsets_loaded: bool,
    pub stats: i40e_hw_port_stats,
    pub stats_offsets: i40e_hw_port_stats,
    pub tx_timeout_count: u32,
    pub tx_timeout_recovery_level: u32,
    pub tx_timeout_last_recovery: c_ulong,
    pub hw_csum_rx_error: u32,
    pub led_status: u32,
    pub /: *mut *mut u16 corer_count; / Core reset count,
    pub /: *mut *mut u16 globr_count; / Global reset count,
    pub /: *mut *mut u16 empr_count; / EMP reset count,
    pub /: *mut *mut u16 pfr_count; / PF reset count,
    pub /: *mut *mut u16 sw_int_count; / SW interrupt count,
    pub link_down_events: u32,
    pub switch_mutex: mutex,
    pub /: *mut *mut u16 lan_vsi; / our default LAN VSI,
    pub /: *mut *mut u16 lan_veb; / initial relay, if exists,
pub const I40E_NO_VEB: c_uint = 0xffff;
pub const I40E_NO_VSI: c_uint = 0xffff;
    pub /: *mut *mut u16 next_vsi; / Next unallocated VSI - 0-based!,
    pub vsi: *mut i40e_vsi,
    pub veb: [*mut i40e_veb; I40E_MAX_VEB],
    pub qp_pile: *mut i40e_lump_tracking,
    pub irq_pile: *mut i40e_lump_tracking,
// switch config info
    pub main_vsi_seid: u16,
    pub mac_seid: u16,

    pub i40e_dbg_pf: *mut dentry,

    pub cur_promisc: bool,
// sr-iov config info
    pub vf: *mut i40e_vf,
    pub /: *mut *mut int num_alloc_vfs; / actual number of VFs allocated,
    pub vf_aq_requests: u32,
// If set to non-zero, the device uses this value
// as maximum number of MAC filters per VF.
//
    pub max_mac_per_vf: u32,
    pub /: *mut *mut u32 arq_overflows; / Not fatal, possibly indicative of problems,
    pub mdd_message_rate_limit: ratelimit_state,
// DCBx/DCBNL capability for PF that indicates
// whether DCBx is managed by firmware or host
// based agent (LLDPAD). Also, indicates what
// flavor of DCBx protocol (IEEE/CEE) is supported
// by the device. For now we're supporting IEEE
// mode only.
//
    pub dcbx_cap: u16,
    pub filter_settings: i40e_filter_control_settings,
    pub /: *mut *mut i40e_rx_pb_config pb_cfg; / Current Rx packet buffer config,
    pub tmp_cfg: i40e_dcbx_config,
// GPIO defines used by PTP
pub const I40E_SDP3_2: c_int = 18;
pub const I40E_SDP3_3: c_int = 19;
pub const I40E_GPIO_4: c_int = 20;
pub const I40E_LED2_0: c_int = 26;
pub const I40E_LED2_1: c_int = 27;
pub const I40E_LED3_0: c_int = 28;
pub const I40E_LED3_1: c_int = 29;

pub const I40E_PTP_2_SEC_DELAY: c_int = 2;
    pub ptp_clock: *mut ptp_clock,
    pub ptp_caps: ptp_clock_info,
    pub ptp_tx_skb: *mut sk_buff,
    pub ptp_tx_start: c_ulong,
    pub tstamp_config: kernel_hwtstamp_config,
    pub ptp_prev_hw_time: timespec64,
    pub ptp_extts0_work: work_struct,
    pub ptp_reset_start: ktime_t,
    pub /: *mut *mut mutex tmreg_lock; / Used to protect the SYSTIME registers.,
    pub ptp_adj_mult: u32,
    pub tx_hwtstamp_timeouts: u32,
    pub tx_hwtstamp_skipped: u32,
    pub rx_hwtstamp_cleared: u32,
    pub latch_event_flags: u32,
    pub /: *mut *mut spinlock_t ptp_rx_lock; / Used to protect Rx timestamp registers.,
    pub latch_events: [c_ulong; 4],
    pub ptp_tx: bool,
    pub ptp_rx: bool,
    pub ptp_pins: *mut i40e_ptp_pins_settings,
    pub /: *mut *mut u16 rss_table_size; / HW RSS table size,
    pub max_bw: u32,
    pub min_bw: u32,
    pub ioremap_len: u32,
    pub fd_inv: u32,
    pub phy_led_val: u16,
    pub last_sw_conf_flags: u16,
    pub last_sw_conf_valid_flags: u16,
// List to keep previous DDP profiles to be rolled back in the future
    pub ddp_old_prof: list_head,
}

//
// __i40e_pf_next_vsi - get next valid VSI
// @pf: pointer to the PF struct
// @idx: pointer to start position number
//
// Find and return next non-NULL VSI pointer in pf->vsi array and
// updates idx position. Returns NULL if no VSI is found.
//

//
// __i40e_pf_next_veb - get next valid VEB
// @pf: pointer to the PF struct
// @idx: pointer to start position number
//
// Find and return next non-NULL VEB pointer in pf->veb array and
// updates idx position. Returns NULL if no VEB is found.
//

//
// i40e_addr_to_hkey - Convert a 6-byte MAC Address to a u64 hash key
// @macaddr: the MAC Address as the base key
//
// Simply copies the address and returns it as a u64 for hashing
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i40e_filter_state {
    I40E_FILTER_INVALID = 0,	/* Invalid state */
    I40E_FILTER_NEW,		/* New, not sent to FW yet */
    I40E_FILTER_ACTIVE,		/* Added to switch by FW */
    I40E_FILTER_FAILED,		/* Rejected by FW */
    I40E_FILTER_REMOVE,		/* To be removed */
    I40E_FILTER_NEW_SYNC,		/* New, not sent yet, is in i40e_sync_vsi_filters() */
// There is no 'removed' state; the filter struct is freed
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_mac_filter {
    pub hlist: hlist_node,
    pub macaddr: [u8; ETH_ALEN],
    pub vlan: i16,
    pub state: i40e_filter_state,
}

// Wrapper structure to keep track of filters while we are preparing to send
// firmware commands. We cannot send firmware commands while holding a
// spinlock, since it might sleep. To avoid this, we wrap the added filters in
// a separate structure, which will track the state change and update the real
// filter while under lock. We can't simply hold the filters in a separate
// list, as this opens a window for a race condition when adding new MAC
// addresses to all VLANs, or when adding new VLANs to all MAC addresses.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_new_mac_filter {
    pub hlist: hlist_node,
    pub f: *mut i40e_mac_filter,
// Track future changes to state separately
    pub state: i40e_filter_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_veb {
    pub pf: *mut i40e_pf,
    pub idx: u16,
    pub seid: u16,
    pub uplink_seid: u16,
    pub /: *mut *mut u16 stats_idx; / index of VEB parent,
    pub enabled_tc: u8,
    pub /: *mut *mut u16 bridge_mode; / Bridge Mode (VEB/VEPA),
    pub bw_limit: u16,
    pub bw_max_quanta: u8,
    pub is_abs_credits: bool,
    pub bw_tc_share_credits: [u8; I40E_MAX_TRAFFIC_CLASS],
    pub bw_tc_limit_credits: [u16; I40E_MAX_TRAFFIC_CLASS],
    pub bw_tc_max_quanta: [u8; I40E_MAX_TRAFFIC_CLASS],
    pub kobj: *mut kobject,
    pub stat_offsets_loaded: bool,
    pub stats: i40e_eth_stats,
    pub stats_offsets: i40e_eth_stats,
    pub tc_stats: i40e_veb_tc_stats,
    pub tc_stats_offsets: i40e_veb_tc_stats,
}

// struct that defines a VSI, associated with a dev
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_vsi {
    pub netdev: *mut net_device,
    pub active_vlans: [c_ulong; BITS_TO_LONGS(VLAN_N_VID)],
    pub netdev_registered: bool,
    pub stat_offsets_loaded: bool,
    pub current_netdev_flags: u32,
    pub __I40E_VSI_STATE_SIZE__): DECLARE_BITMAP(state,,

    pub flags: c_ulong,
// Per VSI lock to protect elements/hash (MAC filter)
    pub mac_filter_hash_lock: spinlock_t,
// Fixed size hash table with 2^8 buckets for MAC filters
    pub 8): DECLARE_HASHTABLE(mac_filter_hash,,
    pub has_vlan_filter: bool,
// VSI stats
    pub net_stats: rtnl_link_stats64,
    pub net_stats_offsets: rtnl_link_stats64,
    pub eth_stats: i40e_eth_stats,
    pub eth_stats_offsets: i40e_eth_stats,
    pub tx_restart: u64,
    pub tx_busy: u64,
    pub tx_linearize: u64,
    pub tx_force_wb: u64,
    pub tx_stopped: u64,
    pub rx_buf_failed: u64,
    pub rx_page_failed: u64,
    pub rx_page_reuse: u64,
    pub rx_page_alloc: u64,
    pub rx_page_waive: u64,
    pub rx_page_busy: u64,
// These are containers of ring pointers, allocated at run-time
    pub rx_rings: *mut i40e_ring,
    pub tx_rings: *mut i40e_ring,
    pub /: *mut *mut *mut *mut i40e_ring xdp_rings; / XDP Tx rings,
    pub active_filters: u32,
    pub promisc_threshold: u32,
    pub work_limit: u16,
    pub /: *mut *mut u16 int_rate_limit; / value in usecs,
    pub /: *mut *mut u16 rss_table_size; / HW RSS table size,
    pub /: *mut *mut u16 rss_size; / Allocated RSS queues,
    pub /: *mut *mut *mut u8 rss_hkey_user; / User configured hash keys,
    pub /: *mut *mut *mut u8 rss_lut_user; / User configured lookup table entries,
    pub max_frame: u16,
    pub rx_buf_len: u16,
    pub xdp_prog: *mut bpf_prog,
// List of q_vectors allocated to this VSI
    pub q_vectors: *mut i40e_q_vector,
    pub num_q_vectors: c_int,
    pub base_vector: c_int,
    pub irqs_ready: bool,
    pub /: *mut *mut u16 seid; / HW index of this VSI (absolute index),
    pub /: *mut *mut u16 id; / VSI number,
    pub uplink_seid: u16,
    pub /: *mut *mut u16 base_queue; / vsi's first queue in hw array,
    pub /: *mut *mut u16 alloc_queue_pairs; / Allocated Tx/Rx queues,
    pub /: *mut *mut u16 req_queue_pairs; / User requested queue pairs,
    pub /: *mut *mut u16 num_queue_pairs; / Used tx and rx pairs,
    pub num_tx_desc: u16,
    pub num_rx_desc: u16,
    pub /: *mut *mut i40e_vsi_type type; / VSI type, e.g., LAN, FCoE, etc,
    pub /: *mut *mut s16 vf_id; / Virtual function ID for SRIOV VSIs,
    pub /: *mut *mut tc_mqprio_qopt_offload mqprio_qopt; / queue parameters,
    pub tc_config: i40e_tc_configuration,
    pub info: i40e_aqc_vsi_properties_data,
// VSI BW limit (absolute across all TCs)
    pub /: *mut *mut u16 bw_limit; / VSI BW Limit (0 = disabled),
    pub /: *mut *mut u8 bw_max_quanta; / Max Quanta when BW limit is enabled,
// Relative TC credits across VSIs
    pub bw_ets_share_credits: [u8; I40E_MAX_TRAFFIC_CLASS],
// TC BW limit credits within VSI
    pub bw_ets_limit_credits: [u16; I40E_MAX_TRAFFIC_CLASS],
// TC BW limit max quanta within VSI
    pub bw_ets_max_quanta: [u8; I40E_MAX_TRAFFIC_CLASS],
    pub /: *mut *mut *mut i40e_pf back; / Backreference to associated PF,
    pub /: *mut *mut u16 idx; / index in pf->vsi[],
    pub /: *mut *mut u16 veb_idx; / index of VEB parent,
    pub /: *mut *mut *mut kobject kobj; / sysfs object,
    pub /: *mut *mut bool current_isup; / Sync 'link up' logging,
    pub /: *mut *mut i40e_aq_link_speed current_speed; / Sync link speed logging,
// channel specific fields
    pub /: *mut *mut u16 cnt_q_avail; / num of queues available for channel usage,
    pub orig_rss_size: u16,
    pub current_rss_size: u16,
    pub reconfig_rss: bool,
    pub /: *mut *mut u16 next_base_queue; / next queue to be used for channel setup,
    pub ch_list: list_head,
    pub tc_seid_map: [u16; I40E_MAX_TRAFFIC_CLASS],
// macvlan fields

    pub I40E_MAX_MACVLANS): DECLARE_BITMAP(fwd_bitmask,,
    pub macvlan_list: list_head,
    pub macvlan_cnt: c_int,
    pub /: *mut *mut *mut void priv; / client driver data reference.,
// VSI specific handlers
    pub data): *mut *mut irqreturn_t (irq_handler)(int irq, void,
    pub /: *mut *mut *mut unsigned long af_xdp_zc_qps; / tracks AF_XDP ZC enabled qps,
    pub ____cacheline_internodealigned_in_smp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_netdev_priv {
    pub vsi: *mut i40e_vsi,
}

// struct that defines an interrupt vector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_q_vector {
    pub vsi: *mut i40e_vsi,
    pub /: *mut *mut u16 v_idx; / index in the vsi->q_vector array.,
    pub /: *mut *mut u16 reg_idx; / register index of the interrupt,
    pub napi: napi_struct,
    pub /: *mut *mut rcu_head rcu; / to avoid race with update stats on free,
    pub rx: i40e_ring_container,
    pub tx: i40e_ring_container,
    pub /: *mut *mut u8 itr_countdown; / when 0 should adjust adaptive ITR,
    pub /: *mut *mut u8 num_ringpairs; / total number of ring pairs in vector,
    pub affinity_mask: cpumask_t,
    pub affinity_notify: irq_affinity_notify,
    pub name: [c_char; I40E_INT_NAME_STR_LEN],
    pub arm_wb_state: bool,
    pub in_busy_poll: bool,
    pub /: *mut *mut int irq_num; / IRQ assigned to this q_vector,
    pub ____cacheline_internodealigned_in_smp: },
// lan device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i40e_device {
    pub list: list_head,
    pub pf: *mut i40e_pf,
}

//
// i40e_info_nvm_ver - format the NVM version string
// @hw: ptr to the hardware info
// @buf: string buffer to store
// @len: buffer size
//
// Formats NVM version string as:
// <gen>.<snap>.<release> when eetrackid == I40E_OEM_EETRACK_ID
// <nvm_major>.<nvm_minor> otherwise
//
// i40e_info_eetrack - format the EETrackID string
// @hw: ptr to the hardware info
// @buf: string buffer to store
// @len: buffer size
//
// Returns hexadecimally formated EETrackID if it is
// different from I40E_OEM_EETRACK_ID or empty string.
//
// i40e_info_civd_ver - format the NVM version strings
// @hw: ptr to the hardware info
// @buf: string buffer to store
// @len: buffer size
//
// Returns formated combo image version if adapter's EETrackID is
// different from I40E_OEM_EETRACK_ID or empty string.
//
// i40e_nvm_version_str - format the NVM version strings
// @hw: ptr to the hardware info
// @buf: string buffer to store
// @len: buffer size
//
// Get NVM version
// Append EETrackID if provided
// Append combo image version if provided
//
// i40e_netdev_to_pf: Retrieve the PF struct for given netdev
// @netdev: the corresponding netdev
//
// Return the PF struct for the given netdev
//
// i40e_get_fd_cnt_all - get the total FD filter space available
// @pf: pointer to the PF struct
//
// i40e_read_fd_input_set - reads value of flow director input set register
// @pf: pointer to the PF struct
// @addr: register addr
//
// This function reads value of flow director input set register
// specified by 'addr' (which is specific to flow-type)
//
// i40e_write_fd_input_set - writes value into flow director input set register
// @pf: pointer to the PF struct
// @addr: register addr
// @val: value to be written
//
// This function writes specified value to the register specified by 'addr'.
// This register is input set register based on flow-type.
//
// i40e_get_pf_count - get PCI PF count.
// @hw: pointer to a hw.
//
// Reports the function number of the highest PCI physical
// function plus 1 as it is loaded from the NVM.
//
// Return: PCI PF count.
//
// needed by i40e_ethtool.c
extern "C" {
    pub fn i40e_up(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_down(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_do_reset_safe(pf: *mut i40e_pf, reset_flags: u32);
}
extern "C" {
    pub fn i40e_do_reset(pf: *mut i40e_pf, reset_flags: u32, lock_acquired: bool);
}
extern "C" {
    pub fn i40e_config_rss(vsi: *mut i40e_vsi, seed: *mut u8, lut: *mut u8, lut_size: u16) -> c_int;
}
extern "C" {
    pub fn i40e_get_rss(vsi: *mut i40e_vsi, seed: *mut u8, lut: *mut u8, lut_size: u16) -> c_int;
}
//
// i40e_find_vsi_by_type - Find and return Flow Director VSI
// @pf: PF to search for VSI
// @type: Value indicating type of VSI we are looking for
//
extern "C" {
    pub fn i40e_update_stats(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_update_veb_stats(veb: *mut i40e_veb);
}
extern "C" {
    pub fn i40e_update_eth_stats(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_fdir_check_and_reenable(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_get_current_fd_count(pf: *mut i40e_pf) -> u32;
}
extern "C" {
    pub fn i40e_get_current_atr_cnt(pf: *mut i40e_pf) -> u32;
}
extern "C" {
    pub fn i40e_get_global_fd_count(pf: *mut i40e_pf) -> u32;
}
extern "C" {
    pub fn i40e_set_ntuple(pf: *mut i40e_pf, features: netdev_features_t) -> bool;
}
extern "C" {
    pub fn i40e_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn __i40e_del_filter(vsi: *mut i40e_vsi, f: *mut i40e_mac_filter);
}
extern "C" {
    pub fn i40e_sync_vsi_filters(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_vsi_release(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_service_event_schedule(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_control_wait_rx_q(pf: *mut i40e_pf, pf_q: c_int, enable: bool) -> c_int;
}
extern "C" {
    pub fn i40e_vsi_start_rings(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_vsi_stop_rings(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_vsi_stop_rings_no_wait(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_vsi_wait_queues_disabled(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_reconfig_rss_queues(pf: *mut i40e_pf, queue_count: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_veb_release(veb: *mut i40e_veb);
}
extern "C" {
    pub fn i40e_veb_config_tc(veb: *mut i40e_veb, enabled_tc: u8) -> c_int;
}
extern "C" {
    pub fn i40e_vsi_add_pvid(vsi: *mut i40e_vsi, vid: u16) -> c_int;
}
extern "C" {
    pub fn i40e_vsi_remove_pvid(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_vsi_reset_stats(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_pf_reset_stats(pf: *mut i40e_pf);
}

extern "C" {
    pub fn i40e_dbg_pf_init(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_dbg_pf_exit(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_dbg_init();
}
extern "C" {
    pub fn i40e_dbg_exit();
}

// needed by client drivers
extern "C" {
    pub fn i40e_lan_add_device(pf: *mut i40e_pf) -> c_int;
}
extern "C" {
    pub fn i40e_lan_del_device(pf: *mut i40e_pf) -> c_int;
}
extern "C" {
    pub fn i40e_client_subtask(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_notify_client_of_l2_param_changes(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_notify_client_of_netdev_close(pf: *mut i40e_pf, reset: bool);
}
extern "C" {
    pub fn i40e_notify_client_of_vf_enable(pf: *mut i40e_pf, num_vfs: u32);
}
extern "C" {
    pub fn i40e_notify_client_of_vf_reset(pf: *mut i40e_pf, vf_id: u32);
}
extern "C" {
    pub fn i40e_client_update_msix_info(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_vf_client_capable(pf: *mut i40e_pf, vf_id: u32) -> c_int;
}
//
// i40e_irq_dynamic_enable - Enable default interrupt generation settings
// @vsi: pointer to a vsi
// @vector: enable a particular Hw Interrupt vector, without base_vector
//
// skip the flush
extern "C" {
    pub fn i40e_irq_dynamic_disable_icr0(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_irq_dynamic_enable_icr0(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ioctl(netdev: *mut net_device, ifr: *mut ifreq, cmd: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_open(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn i40e_close(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn i40e_vsi_open(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_vlan_stripping_disable(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_add_vlan_all_mac(vsi: *mut i40e_vsi, vid: i16) -> c_int;
}
extern "C" {
    pub fn i40e_vsi_add_vlan(vsi: *mut i40e_vsi, vid: u16) -> c_int;
}
extern "C" {
    pub fn i40e_rm_vlan_all_mac(vsi: *mut i40e_vsi, vid: i16);
}
extern "C" {
    pub fn i40e_vsi_kill_vlan(vsi: *mut i40e_vsi, vid: u16);
}
extern "C" {
    pub fn i40e_del_mac_filter(vsi: *mut i40e_vsi, macaddr: *const u8) -> c_int;
}
extern "C" {
    pub fn i40e_is_vsi_in_vlan(vsi: *mut i40e_vsi) -> bool;
}
extern "C" {
    pub fn i40e_count_all_filters(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_count_active_filters(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_vlan_stripping_enable(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn test_bit(_arg: I40E_FLAG_FW_LLDP_DIS, _arg: pf->flags) -> return;
}

extern "C" {
    pub fn i40e_dcbnl_set_all(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_dcbnl_setup(vsi: *mut i40e_vsi);
}
extern "C" {
    pub fn i40e_hw_dcb_config(pf: *mut i40e_pf, new_cfg: *mut i40e_dcbx_config) -> c_int;
}
extern "C" {
    pub fn i40e_dcb_sw_default_config(pf: *mut i40e_pf) -> c_int;
}

extern "C" {
    pub fn i40e_ptp_rx_hang(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_tx_hang(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_tx_hwtstamp(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_rx_hwtstamp(pf: *mut i40e_pf, skb: *mut sk_buff, index: u8);
}
extern "C" {
    pub fn i40e_ptp_set_increment(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_save_hw_time(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_restore_hw_time(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_init(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_stop(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_ptp_alloc_pins(pf: *mut i40e_pf) -> c_int;
}
extern "C" {
    pub fn i40e_ptp_free_pins(pf: *mut i40e_pf);
}
extern "C" {
    pub fn i40e_update_adq_vsi_queues(vsi: *mut i40e_vsi, vsi_offset: c_int) -> c_int;
}
extern "C" {
    pub fn i40e_is_vsi_uplink_mode_veb(vsi: *mut i40e_vsi) -> c_int;
}
extern "C" {
    pub fn i40e_get_partition_bw_setting(pf: *mut i40e_pf) -> c_int;
}
extern "C" {
    pub fn i40e_set_partition_bw_setting(pf: *mut i40e_pf) -> c_int;
}
extern "C" {
    pub fn i40e_print_link_message(vsi: *mut i40e_vsi, isup: bool);
}
extern "C" {
    pub fn i40e_set_fec_in_flags(fec_cfg: u8, flags: *mut c_ulong);
}
extern "C" {
    pub fn i40e_create_queue_channel(vsi: *mut i40e_vsi, ch: *mut i40e_channel) -> c_int;
}
extern "C" {
    pub fn i40e_set_bw_limit(vsi: *mut i40e_vsi, seid: u16, max_tx_rate: u64) -> c_int;
}
//
// i40e_is_tc_mqprio_enabled - check if TC MQPRIO is enabled on PF
// @pf: pointer to a pf.
//
// Check and return state of flag I40E_FLAG_TC_MQPRIO.
//
// Return: true/false if I40E_FLAG_TC_MQPRIO is set or not
//
extern "C" {
    pub fn test_bit(_arg: I40E_FLAG_TC_MQPRIO_ENA, _arg: pf->flags) -> return;
}
//
// i40e_hw_to_pf - get pf pointer from the hardware structure
// @hw: pointer to the device HW structure
//
extern "C" {
    pub fn container_of(_arg: hw, i40e_pf: struct, _arg: hw) -> return;
}
//
// i40e_pf_get_vsi_by_seid - find VSI by SEID
// @pf: pointer to a PF
// @seid: SEID of the VSI
//
// i40e_pf_get_main_vsi - get pointer to main VSI
// @pf: pointer to a PF
//
// Return: pointer to main VSI or NULL if it does not exist
//
// i40e_pf_get_veb_by_seid - find VEB by SEID
// @pf: pointer to a PF
// @seid: SEID of the VSI
//
// i40e_pf_get_main_veb - get pointer to main VEB
// @pf: pointer to a PF
//
// Return: pointer to main VEB or NULL if it does not exist
//
