//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/idpf/idpf.h
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
// Copyright (C) 2023 Intel Corporation
// Forward declaration

pub const IDPF_NO_FREE_SLOT: c_uint = 0xffff;
// Default Mailbox settings
pub const IDPF_NUM_FILTERS_PER_MSG: c_int = 20;

pub const IDPF_DFLT_MBX_Q_LEN: c_int = 64;
// maximum number of times to try before resetting mailbox
pub const IDPF_MB_MAX_ERR: c_int = 20;

pub const IDPF_WAIT_FOR_MARKER_TIMEO: c_int = 500;
pub const IDPF_MAX_WAIT: c_int = 500;
// available message levels

pub const IDPF_DIM_PROFILE_SLOTS: c_int = 5;

//
// struct idpf_mac_filter
// @list: list member field
// @macaddr: MAC address
// @remove: filter should be removed (virtchnl)
// @add: filter should be added (virtchnl)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_mac_filter {
    pub list: list_head,
    pub macaddr: [u8; ETH_ALEN],
    pub remove: bool,
    pub add: bool,
}

//
// enum idpf_state - State machine to handle bring up
// @__IDPF_VER_CHECK: Negotiate virtchnl version
// @__IDPF_GET_CAPS: Negotiate capabilities
// @__IDPF_INIT_SW: Init based on given capabilities
// @__IDPF_STATE_LAST: Must be last, used to determine size
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_state {
    __IDPF_VER_CHECK,
    __IDPF_GET_CAPS,
    __IDPF_INIT_SW,
    __IDPF_STATE_LAST,
}

//
// enum idpf_flags - Hard reset causes.
// @IDPF_HR_FUNC_RESET: Hard reset when TxRx timeout
// @IDPF_HR_DRV_LOAD: Set on driver load for a clean HW
// @IDPF_HR_RESET_IN_PROG: Reset in progress
// @IDPF_REMOVE_IN_PROG: Driver remove in progress
// @IDPF_MB_INTR_MODE: Mailbox in interrupt mode
// @IDPF_VC_CORE_INIT: virtchnl core has been init
// @IDPF_FLAGS_NBITS: Must be last
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_flags {
    IDPF_HR_FUNC_RESET,
    IDPF_HR_DRV_LOAD,
    IDPF_HR_RESET_IN_PROG,
    IDPF_REMOVE_IN_PROG,
    IDPF_MB_INTR_MODE,
    IDPF_VC_CORE_INIT,
    IDPF_FLAGS_NBITS,
}

//
// enum idpf_cap_field - Offsets into capabilities struct for specific caps
// @IDPF_BASE_CAPS: generic base capabilities
// @IDPF_CSUM_CAPS: checksum offload capabilities
// @IDPF_SEG_CAPS: segmentation offload capabilities
// @IDPF_RSS_CAPS: RSS offload capabilities
// @IDPF_HSPLIT_CAPS: Header split capabilities
// @IDPF_RSC_CAPS: RSC offload capabilities
// @IDPF_OTHER_CAPS: miscellaneous offloads
//
// Used when checking for a specific capability flag since different capability
// sets are not mutually exclusive numerically, the caller must specify which
// type of capability they are checking for.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_cap_field {
    IDPF_BASE_CAPS		= -1,
    IDPF_CSUM_CAPS		= offsetof(struct virtchnl2_get_capabilities,
    csum_caps),
    IDPF_SEG_CAPS		= offsetof(struct virtchnl2_get_capabilities,
    seg_caps),
    IDPF_RSS_CAPS		= offsetof(struct virtchnl2_get_capabilities,
    rss_caps),
    IDPF_HSPLIT_CAPS	= offsetof(struct virtchnl2_get_capabilities,
    hsplit_caps),
    IDPF_RSC_CAPS		= offsetof(struct virtchnl2_get_capabilities,
    rsc_caps),
    IDPF_OTHER_CAPS		= offsetof(struct virtchnl2_get_capabilities,
    other_caps),
}

//
// enum idpf_vport_state - Current vport state
// @IDPF_VPORT_UP: Vport is up
// @IDPF_VPORT_STATE_NBITS: Must be last, number of states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_vport_state {
    IDPF_VPORT_UP,
    IDPF_VPORT_STATE_NBITS
}

//
// struct idpf_netdev_priv - Struct to store vport back pointer
// @adapter: Adapter back pointer
// @vport: Vport back pointer
// @vport_id: Vport identifier
// @link_speed_mbps: Link speed in mbps
// @vport_idx: Relative vport index
// @max_tx_hdr_size: Max header length hardware can support
// @tx_max_bufs: Max buffers that can be transmitted with scatter-gather
// @state: See enum idpf_vport_state
// @netstats: Packet and byte stats
// @stats_lock: Lock to protect stats update
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_netdev_priv {
    pub adapter: *mut idpf_adapter,
    pub vport: *mut idpf_vport,
    pub vport_id: u32,
    pub link_speed_mbps: u32,
    pub vport_idx: u16,
    pub max_tx_hdr_size: u16,
    pub tx_max_bufs: u16,
    pub IDPF_VPORT_STATE_NBITS): DECLARE_BITMAP(state,,
    pub netstats: rtnl_link_stats64,
    pub stats_lock: spinlock_t,
}

//
// struct idpf_reset_reg - Reset register offsets/masks
// @rstat: Reset status register
// @rstat_m: Reset status mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_reset_reg {
    pub rstat: *mut void __iomem,
    pub rstat_m: u32,
}

//
// struct idpf_vport_max_q - Queue limits
// @max_rxq: Maximum number of RX queues supported
// @max_txq: Maixmum number of TX queues supported
// @max_bufq: In splitq, maximum number of buffer queues supported
// @max_complq: In splitq, maximum number of completion queues supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_vport_max_q {
    pub max_rxq: u16,
    pub max_txq: u16,
    pub max_bufq: u16,
    pub max_complq: u16,
}

//
// struct idpf_reg_ops - Device specific register operation function pointers
// @ctlq_reg_init: Mailbox control queue register initialization
// @intr_reg_init: Traffic interrupt register initialization
// @mb_intr_reg_init: Mailbox interrupt register initialization
// @reset_reg_init: Reset register initialization
// @trigger_reset: Trigger a reset to occur
// @ptp_reg_init: PTP register initialization
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_reg_ops {
    pub cctlq_info): *mut libie_ctlq_create_info,
    pub rsrc): *mut idpf_q_vec_rsrc,
    pub adapter): *mut *mut void (mb_intr_reg_init)(struct idpf_adapter,
    pub adapter): *mut *mut void (reset_reg_init)(struct idpf_adapter,
    pub trig_cause): idpf_flags,
    pub adapter): *const *const void (ptp_reg_init)(struct idpf_adapter,
}

pub const IDPF_MMIO_REG_NUM_STATIC: c_int = 2;
pub const IDPF_PF_MBX_REGION_SZ: c_int = 4096;
pub const IDPF_PF_RSTAT_REGION_SZ: c_int = 2048;
pub const IDPF_VF_MBX_REGION_SZ: c_int = 10240;
pub const IDPF_VF_RSTAT_REGION_SZ: c_int = 2048;
//
// struct idpf_dev_ops - Device specific operations
// @reg_ops: Register operations
// @idc_init: IDC initialization
// @static_reg_info: array of mailbox and rstat register info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_dev_ops {
    pub reg_ops: idpf_reg_ops,
    pub adapter): *mut *mut int (idc_init)(struct idpf_adapter,
// static_reg_info[0] is mailbox region, static_reg_info[1] is rstat
    pub static_reg_info: [resource; IDPF_MMIO_REG_NUM_STATIC],
}

//
// enum idpf_vport_reset_cause - Vport soft reset causes
// @IDPF_SR_Q_CHANGE: Soft reset queue change
// @IDPF_SR_Q_DESC_CHANGE: Soft reset descriptor change
// @IDPF_SR_MTU_CHANGE: Soft reset MTU change
// @IDPF_SR_RSC_CHANGE: Soft reset RSC change
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_vport_reset_cause {
    IDPF_SR_Q_CHANGE,
    IDPF_SR_Q_DESC_CHANGE,
    IDPF_SR_MTU_CHANGE,
    IDPF_SR_RSC_CHANGE,
}

//
// enum idpf_vport_flags - Vport flags
// @IDPF_VPORT_DEL_QUEUES: To send delete queues message
// @IDPF_VPORT_FLAGS_NBITS: Must be last
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_vport_flags {
    IDPF_VPORT_DEL_QUEUES,
    IDPF_VPORT_FLAGS_NBITS,
}

//
// struct idpf_tstamp_stats - Tx timestamp statistics
// @stats_sync: See struct u64_stats_sync
// @packets: Number of packets successfully timestamped by the hardware
// @discarded: Number of Tx skbs discarded due to cached PHC
// being too old to correctly extend timestamp
// @flushed: Number of Tx skbs flushed due to interface closed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_tstamp_stats {
    pub stats_sync: u64_stats_sync,
    pub packets: u64_stats_t,
    pub discarded: u64_stats_t,
    pub flushed: u64_stats_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_port_stats {
    pub stats_sync: u64_stats_sync,
    pub rx_hw_csum_err: u64_stats_t,
    pub rx_hsplit: u64_stats_t,
    pub rx_hsplit_hbo: u64_stats_t,
    pub rx_bad_descs: u64_stats_t,
    pub tx_linearize: u64_stats_t,
    pub tx_busy: u64_stats_t,
    pub tx_drops: u64_stats_t,
    pub tx_dma_map_errs: u64_stats_t,
    pub vport_stats: virtchnl2_vport_stats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_fsteer_fltr {
    pub list: list_head,
    pub fs: ethtool_rx_flow_spec,
}

//
// struct idpf_q_vec_rsrc - handle for queue and vector resources
// @dev: device pointer for DMA mapping
// @q_vectors: array of queue vectors
// @q_vector_idxs: starting index of queue vectors
// @num_q_vectors: number of IRQ vectors allocated
// @noirq_v_idx: ID of the NOIRQ vector
// @noirq_dyn_ctl_ena: value to write to the above to enable it
// @noirq_dyn_ctl: register to enable/disable the vector for NOIRQ queues
// @txq_grps: array of TX queue groups
// @txq_desc_count: TX queue descriptor count
// @complq_desc_count: completion queue descriptor count
// @txq_model: split queue or single queue queuing model
// @num_txq: number of allocated TX queues
// @num_complq: number of allocated completion queues
// @num_txq_grp: number of TX queue groups
// @xdp_txq_offset: index of the first XDPSQ (== number of regular SQs)
// @num_rxq_grp: number of RX queues in a group
// @rxq_model: splitq queue or single queue queuing model
// @rxq_grps: total number of RX groups. Number of groups * number of RX per
// group will yield total number of RX queues.
// @num_rxq: number of allocated RX queues
// @num_bufq: number of allocated buffer queues
// @rxq_desc_count: RX queue descriptor count. *MUST* have enough descriptors
// to complete all buffer descriptors for all buffer queues in
// the worst case.
// @bufq_desc_count: buffer queue descriptor count
// @num_bufqs_per_qgrp: buffer queues per RX queue in a given grouping
// @base_rxd: true if the driver should use base descriptors instead of flex
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_q_vec_rsrc {
    pub dev: *mut device,
    pub q_vectors: *mut idpf_q_vector,
    pub q_vector_idxs: *mut u16,
    pub num_q_vectors: u16,
    pub noirq_v_idx: u16,
    pub noirq_dyn_ctl_ena: u32,
    pub noirq_dyn_ctl: *mut void __iomem,
    pub txq_grps: *mut idpf_txq_group,
    pub txq_desc_count: u32,
    pub complq_desc_count: u32,
    pub txq_model: u32,
    pub num_txq: u16,
    pub num_complq: u16,
    pub num_txq_grp: u16,
    pub xdp_txq_offset: u16,
    pub num_rxq_grp: u16,
    pub rxq_model: u32,
    pub rxq_grps: *mut idpf_rxq_group,
    pub num_rxq: u16,
    pub num_bufq: u16,
    pub rxq_desc_count: u32,
    pub bufq_desc_count: [u32; IDPF_MAX_BUFQS_PER_RXQ_GRP],
    pub num_bufqs_per_qgrp: u8,
    pub base_rxd: bool,
}

//
// struct idpf_vport - Handle for netdevices and queue resources
// @dflt_qv_rsrc: contains default queue and vector resources
// @txqs: Used only in hotpath to get to the right queue very fast
// @num_txq: Number of allocated TX queues
// @num_xdp_txq: number of XDPSQs
// @xdpsq_share: whether XDPSQ sharing is enabled
// @xdp_prog: installed XDP program
// @vdev_info: IDC vport device info pointer
// @adapter: back pointer to associated adapter
// @netdev: Associated net_device. Each vport should have one and only one
// associated netdev.
// @flags: See enum idpf_vport_flags
// @compln_clean_budget: Work budget for completion clean
// @vport_id: Device given vport identifier
// @vport_type: Default SRIOV, SIOV, etc.
// @idx: Software index in adapter vports struct
// @max_mtu: device given max possible MTU
// @default_mac_addr: device will give a default MAC to use
// @rx_itr_profile: RX profiles for Dynamic Interrupt Moderation
// @tx_itr_profile: TX profiles for Dynamic Interrupt Moderation
// @port_stats: per port csum, header split, and other offload stats
// @default_vport: Use this vport if one isn't specified
// @crc_enable: Enable CRC insertion offload
// @link_up: True if link is up
// @tx_tstamp_caps: Capabilities negotiated for Tx timestamping
// @tstamp_config: The Tx tstamp config
// @tstamp_task: Tx timestamping task
// @tstamp_stats: Tx timestamping statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_vport {
    pub dflt_qv_rsrc: idpf_q_vec_rsrc,
    pub txqs: *mut idpf_tx_queue,
    pub num_txq: u16,
    pub num_xdp_txq: u16,
    pub xdpsq_share: bool,
    pub xdp_prog: *mut bpf_prog,
    pub vdev_info: *mut iidc_rdma_vport_dev_info,
    pub adapter: *mut idpf_adapter,
    pub netdev: *mut net_device,
    pub IDPF_VPORT_FLAGS_NBITS): DECLARE_BITMAP(flags,,
    pub compln_clean_budget: u32,
    pub vport_id: u32,
    pub vport_type: u16,
    pub idx: u16,
    pub max_mtu: u16,
    pub default_mac_addr: [u8; ETH_ALEN],
    pub rx_itr_profile: [u16; IDPF_DIM_PROFILE_SLOTS],
    pub tx_itr_profile: [u16; IDPF_DIM_PROFILE_SLOTS],
    pub port_stats: idpf_port_stats,
    pub default_vport: bool,
    pub crc_enable: bool,
    pub link_up: bool,
    pub tx_tstamp_caps: *mut idpf_ptp_vport_tx_tstamp_caps,
    pub tstamp_config: kernel_hwtstamp_config,
    pub tstamp_task: work_struct,
    pub tstamp_stats: idpf_tstamp_stats,
}

//
// enum idpf_user_flags
// @__IDPF_USER_FLAG_HSPLIT: header split state
// @__IDPF_PROMISC_UC: Unicast promiscuous mode
// @__IDPF_PROMISC_MC: Multicast promiscuous mode
// @__IDPF_USER_FLAGS_NBITS: Must be last
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_user_flags {
    __IDPF_USER_FLAG_HSPLIT = 0U,
    __IDPF_PROMISC_UC = 32,
    __IDPF_PROMISC_MC,

    __IDPF_USER_FLAGS_NBITS,
}

//
// struct idpf_rss_data - Associated RSS data
// @rss_key_size: Size of RSS hash key
// @rss_key: RSS hash key
// @rss_lut_size: Size of RSS lookup table
// @rss_lut: RSS lookup table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_rss_data {
    pub rss_key_size: u16,
    pub rss_key: *mut u8,
    pub rss_lut_size: u16,
    pub rss_lut: *mut u32,
}

//
// struct idpf_q_coalesce - User defined coalescing configuration values for
// a single queue.
// @tx_intr_mode: Dynamic TX ITR or not
// @rx_intr_mode: Dynamic RX ITR or not
// @tx_coalesce_usecs: TX interrupt throttling rate
// @rx_coalesce_usecs: RX interrupt throttling rate
//
// Used to restore user coalescing configuration after a reset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_q_coalesce {
    pub tx_intr_mode: u32,
    pub rx_intr_mode: u32,
    pub tx_coalesce_usecs: u32,
    pub rx_coalesce_usecs: u32,
}

//
// struct idpf_vport_user_config_data - User defined configuration values for
// each vport.
// @rss_data: See struct idpf_rss_data
// @q_coalesce: Array of per queue coalescing data
// @num_req_tx_qs: Number of user requested TX queues through ethtool
// @num_req_rx_qs: Number of user requested RX queues through ethtool
// @num_req_txq_desc: Number of user requested TX queue descriptors through
// ethtool
// @num_req_rxq_desc: Number of user requested RX queue descriptors through
// ethtool
// @xdp_prog: requested XDP program to install
// @user_flags: User toggled config flags
// @mac_filter_list: List of MAC filters
// @num_fsteer_fltrs: number of flow steering filters
// @flow_steer_list: list of flow steering filters
//
// Used to restore configuration after a reset as the vport will get wiped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_vport_user_config_data {
    pub rss_data: idpf_rss_data,
    pub q_coalesce: *mut idpf_q_coalesce,
    pub num_req_tx_qs: u16,
    pub num_req_rx_qs: u16,
    pub num_req_txq_desc: u32,
    pub num_req_rxq_desc: u32,
    pub xdp_prog: *mut bpf_prog,
    pub __IDPF_USER_FLAGS_NBITS): DECLARE_BITMAP(user_flags,,
    pub mac_filter_list: list_head,
    pub num_fsteer_fltrs: u32,
    pub flow_steer_list: list_head,
}

//
// enum idpf_vport_config_flags - Vport config flags
// @IDPF_VPORT_REG_NETDEV: Register netdev
// @IDPF_VPORT_UP_REQUESTED: Set if interface up is requested on core reset
// @IDPF_VPORT_CONFIG_FLAGS_NBITS: Must be last
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum idpf_vport_config_flags {
    IDPF_VPORT_REG_NETDEV,
    IDPF_VPORT_UP_REQUESTED,
    IDPF_VPORT_CONFIG_FLAGS_NBITS,
}

//
// struct idpf_avail_queue_info
// @avail_rxq: Available RX queues
// @avail_txq: Available TX queues
// @avail_bufq: Available buffer queues
// @avail_complq: Available completion queues
//
// Maintain total queues available after allocating max queues to each vport.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_avail_queue_info {
    pub avail_rxq: u16,
    pub avail_txq: u16,
    pub avail_bufq: u16,
    pub avail_complq: u16,
}

//
// struct idpf_vector_info - Utility structure to pass function arguments as a
// structure
// @num_req_vecs: Vectors required based on the number of queues updated by the
// user via ethtool
// @num_curr_vecs: Current number of vectors, must be >= @num_req_vecs
// @index: Relative starting index for vectors
// @default_vport: Vectors are for default vport
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_vector_info {
    pub num_req_vecs: u16,
    pub num_curr_vecs: u16,
    pub index: u16,
    pub default_vport: bool,
}

//
// struct idpf_vector_lifo - Stack to maintain vector indexes used for vector
// distribution algorithm
// @top: Points to stack top i.e. next available vector index
// @base: Always points to start of the free pool
// @size: Total size of the vector stack
// @vec_idx: Array to store all the vector indexes
//
// Vector stack maintains all the relative vector indexes at the *adapter
// level. This stack is divided into 2 parts, first one is called as 'default
// pool' and other one is called 'free pool'.  Vector distribution algorithm
// gives priority to default vports in a way that at least IDPF_MIN_Q_VEC
// vectors are allocated per default vport and the relative vector indexes for
// those are maintained in default pool. Free pool contains all the unallocated
// vector indexes which can be allocated on-demand basis. Mailbox vector index
// is maintained in the default pool of the stack.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_vector_lifo {
    pub top: u16,
    pub base: u16,
    pub size: u16,
    pub vec_idx: *mut u16,
}

//
// struct idpf_queue_id_reg_chunk - individual queue ID and register chunk
// @qtail_reg_start: queue tail register offset
// @qtail_reg_spacing: queue tail register spacing
// @type: queue type of the queues in the chunk
// @start_queue_id: starting queue ID in the chunk
// @num_queues: number of queues in the chunk
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_queue_id_reg_chunk {
    pub qtail_reg_start: u64,
    pub qtail_reg_spacing: u32,
    pub type: u32,
    pub start_queue_id: u32,
    pub num_queues: u32,
}

//
// struct idpf_queue_id_reg_info - queue ID and register chunk info received
// over the mailbox
// @num_chunks: number of chunks
// @queue_chunks: array of chunks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_queue_id_reg_info {
    pub num_chunks: u16,
    pub queue_chunks: *mut idpf_queue_id_reg_chunk,
}

//
// struct idpf_vport_config - Vport configuration data
// @user_config: see struct idpf_vport_user_config_data
// @max_q: Maximum possible queues
// @qid_reg_info: Struct to store the queue ID and register info
// @mac_filter_list_lock: Lock to protect mac filters
// @flow_steer_list_lock: Lock to protect fsteer filters
// @flags: See enum idpf_vport_config_flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_vport_config {
    pub user_config: idpf_vport_user_config_data,
    pub max_q: idpf_vport_max_q,
    pub qid_reg_info: idpf_queue_id_reg_info,
    pub mac_filter_list_lock: spinlock_t,
    pub flow_steer_list_lock: spinlock_t,
    pub IDPF_VPORT_CONFIG_FLAGS_NBITS): DECLARE_BITMAP(flags,,
}

// iter = (adapter)->max_vports ? *__##iter : NULL; \
// __##iter : NULL)
//
// struct idpf_adapter - Device data struct generated on probe
// @pdev: PCI device struct given on probe
// @virt_ver_maj: Virtchnl version major
// @virt_ver_min: Virtchnl version minor
// @msg_enable: Debug message level enabled
// @mb_wait_count: Number of times mailbox was attempted initialization
// @state: Init state machine
// @flags: See enum idpf_flags
// @reset_reg: See struct idpf_reset_reg
// @ctlq_ctx: controlq context
// @asq: Send control queue info
// @arq: Receive control queue info
// @xnm: Xn transaction manager
// @num_avail_msix: Available number of MSIX vectors
// @num_msix_entries: Number of entries in MSIX table
// @msix_entries: MSIX table
// @num_rdma_msix_entries: Available number of MSIX vectors for RDMA
// @rdma_msix_entries: RDMA MSIX table
// @req_vec_chunks: Requested vector chunk data
// @mb_vector: Mailbox vector data
// @vector_stack: Stack to store the msix vector indexes
// @irq_mb_handler: Handler for hard interrupt for mailbox
// @tx_timeout_count: Number of TX timeouts that have occurred
// @avail_queues: Device given queue limits
// @vports: Array to store vports created by the driver
// @netdevs: Associated Vport netdevs
// @vport_params_recvd: Vport params received
// @vport_ids: Array of device given vport identifiers
// @singleq_pt_lkup: Lookup table for singleq RX ptypes
// @splitq_pt_lkup: Lookup table for splitq RX ptypes
// @vport_config: Vport config parameters
// @max_vports: Maximum vports that can be allocated
// @num_alloc_vports: Current number of vports allocated
// @next_vport: Next free slot in pf->vport[] - 0-based!
// @init_task: Initialization task
// @init_wq: Workqueue for initialization task
// @serv_task: Periodically recurring maintenance task
// @serv_wq: Workqueue for service task
// @mbx_task: Task to handle mailbox interrupts
// @mbx_wq: Workqueue for mailbox responses
// @vc_event_task: Task to handle out of band virtchnl event notifications
// @vc_event_wq: Workqueue for virtchnl events
// @stats_task: Periodic statistics retrieval task
// @stats_wq: Workqueue for statistics task
// @caps: Negotiated capabilities with device
// @dev_ops: See idpf_dev_ops
// @cdev_info: IDC core device info pointer
// @num_vfs: Number of allocated VFs through sysfs. PF does not directly talk
// to VFs but is used to initialize them
// @crc_enable: Enable CRC insertion offload
// @req_tx_splitq: TX split or single queue model to request
// @req_rx_splitq: RX split or single queue model to request
// @vport_ctrl_lock: Lock to protect the vport control flow
// @vector_lock: Lock to protect vector distribution
// @queue_lock: Lock to protect queue distribution
// @vc_buf_lock: Lock to protect virtchnl buffer
// @ptp: Storage for PTP-related data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idpf_adapter {
    pub pdev: *mut pci_dev,
    pub virt_ver_maj: u32,
    pub virt_ver_min: u32,
    pub msg_enable: u32,
    pub mb_wait_count: u32,
    pub state: idpf_state,
    pub IDPF_FLAGS_NBITS): DECLARE_BITMAP(flags,,
    pub reset_reg: idpf_reset_reg,
    pub ctlq_ctx: libie_ctlq_ctx,
    pub asq: *mut libie_ctlq_info,
    pub arq: *mut libie_ctlq_info,
    pub xnm: *mut libie_ctlq_xn_manager,
    pub num_avail_msix: u16,
    pub num_msix_entries: u16,
    pub msix_entries: *mut msix_entry,
    pub num_rdma_msix_entries: u16,
    pub rdma_msix_entries: *mut msix_entry,
    pub req_vec_chunks: *mut virtchnl2_alloc_vectors,
    pub mb_vector: idpf_q_vector,
    pub vector_stack: idpf_vector_lifo,
    pub data): *mut *mut irqreturn_t (irq_mb_handler)(int irq, void,
    pub tx_timeout_count: u32,
    pub avail_queues: idpf_avail_queue_info,
    pub vports: *mut idpf_vport,
    pub netdevs: *mut net_device,
    pub vport_params_recvd: *mut virtchnl2_create_vport,
    pub vport_ids: *mut u32,
    pub singleq_pt_lkup: *mut libeth_rx_pt,
    pub splitq_pt_lkup: *mut libeth_rx_pt,
    pub vport_config: *mut idpf_vport_config,
    pub max_vports: u16,
    pub num_alloc_vports: u16,
    pub next_vport: u16,
    pub init_task: delayed_work,
    pub init_wq: *mut workqueue_struct,
    pub serv_task: delayed_work,
    pub serv_wq: *mut workqueue_struct,
    pub mbx_task: delayed_work,
    pub mbx_wq: *mut workqueue_struct,
    pub vc_event_task: delayed_work,
    pub vc_event_wq: *mut workqueue_struct,
    pub stats_task: delayed_work,
    pub stats_wq: *mut workqueue_struct,
    pub caps: virtchnl2_get_capabilities,
    pub dev_ops: idpf_dev_ops,
    pub cdev_info: *mut iidc_rdma_core_dev_info,
    pub num_vfs: c_int,
    pub crc_enable: bool,
    pub req_tx_splitq: bool,
    pub req_rx_splitq: bool,
    pub vport_ctrl_lock: mutex,
    pub vector_lock: mutex,
    pub queue_lock: mutex,
    pub vc_buf_lock: mutex,
    pub ptp: *mut idpf_ptp,
}

//
// idpf_is_queue_model_split - check if queue model is split
// @q_model: queue model single or split
//
// Returns true if queue model is split else false
//

//
// idpf_is_rdma_cap_ena - Determine if RDMA is supported
// @adapter: private data struct
//
// Return: true if RDMA capability is enabled, false otherwise
//
extern "C" {
    pub fn idpf_is_cap_ena(_arg: adapter, _arg: IDPF_OTHER_CAPS, _arg: VIRTCHNL2_CAP_RDMA) -> return;
}

//
// idpf_get_reserved_vecs - Get reserved vectors
// @adapter: private data struct
//
extern "C" {
    pub fn le16_to_cpu(_arg: adapter->caps.num_allocated_vectors) -> return;
}
//
// idpf_get_reserved_rdma_vecs - Get reserved RDMA vectors
// @adapter: private data struct
//
// Return: number of vectors reserved for RDMA
//
extern "C" {
    pub fn le16_to_cpu(_arg: adapter->caps.num_rdma_allocated_vectors) -> return;
}
//
// idpf_get_default_vports - Get default number of vports
// @adapter: private data struct
//
extern "C" {
    pub fn le16_to_cpu(_arg: adapter->caps.default_num_vports) -> return;
}
//
// idpf_get_max_vports - Get max number of vports
// @adapter: private data struct
//
extern "C" {
    pub fn le16_to_cpu(_arg: adapter->caps.max_vports) -> return;
}
//
// idpf_get_max_tx_bufs - Get max scatter-gather buffers supported by the device
// @adapter: private data struct
//
// idpf_get_min_tx_pkt_len - Get min packet length supported by the device
// @adapter: private data struct
//
// idpf_is_reset_detected - check if we were reset at some point
// @adapter: driver specific private structure
//
// Returns true if we are either in reset currently or were previously reset.
//
// idpf_is_reset_in_prog - check if reset is in progress
// @adapter: driver specific private structure
//
// Returns true if hard reset is in progress, false otherwise
//
// idpf_netdev_to_vport - get a vport handle from a netdev
// @netdev: network interface device structure
//
// idpf_netdev_to_adapter - Get adapter handle from a netdev
// @netdev: Network interface device structure
//
// idpf_is_feature_ena - Determine if a particular feature is enabled
// @vport: Vport to check
// @feature: Netdev flag to check
//
// Returns true or false if a particular feature is enabled.
//
// idpf_get_max_tx_hdr_size -- get the size of tx header
// @adapter: Driver specific private structure
//
extern "C" {
    pub fn le16_to_cpu(_arg: adapter->caps.max_tx_hdr_size) -> return;
}
//
// idpf_vport_ctrl_lock - Acquire the vport control lock
// @netdev: Network interface device structure
//
// This lock should be used by non-datapath code to protect against vport
// destruction.
//
// idpf_vport_ctrl_unlock - Release the vport control lock
// @netdev: Network interface device structure
//
extern "C" {
    pub fn mutex_is_locked(_arg: &np->adapter->vport_ctrl_lock) -> return;
}
extern "C" {
    pub fn idpf_statistics_task(work: *mut work_struct);
}
extern "C" {
    pub fn idpf_init_task(work: *mut work_struct);
}
extern "C" {
    pub fn idpf_service_task(work: *mut work_struct);
}
extern "C" {
    pub fn idpf_mbx_task(work: *mut work_struct);
}
extern "C" {
    pub fn idpf_vc_event_task(work: *mut work_struct);
}
extern "C" {
    pub fn idpf_dev_ops_init(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_vf_dev_ops_init(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_intr_req(adapter: *mut idpf_adapter) -> c_int;
}
extern "C" {
    pub fn idpf_mb_intr_rel_irq(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_intr_rel(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_get_max_tx_hdr_size(adapter: *mut idpf_adapter) -> u16;
}
extern "C" {
    pub fn idpf_deinit_task(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_set_ethtool_ops(netdev: *mut net_device);
}
extern "C" {
    pub fn idpf_sriov_configure(pdev: *mut pci_dev, num_vfs: c_int) -> c_int;
}
extern "C" {
    pub fn idpf_vport_get_hsplit(vport: *const idpf_vport) -> u8;
}
extern "C" {
    pub fn idpf_vport_set_hsplit(vport: *const idpf_vport, val: u8) -> bool;
}
extern "C" {
    pub fn idpf_idc_init(adapter: *mut idpf_adapter) -> c_int;
}
extern "C" {
    pub fn idpf_idc_deinit_core_aux_device(adapter: *mut idpf_adapter);
}
extern "C" {
    pub fn idpf_idc_deinit_vport_aux_device(vdev_info: *mut iidc_rdma_vport_dev_info);
}
extern "C" {
    pub fn idpf_idc_issue_reset_event(cdev_info: *mut iidc_rdma_core_dev_info);
}
