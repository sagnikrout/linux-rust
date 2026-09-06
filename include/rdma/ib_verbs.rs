//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_verbs.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2004 Mellanox Technologies Ltd.  All rights reserved.
// Copyright (c) 2004 Infinicon Corporation.  All rights reserved.
// Copyright (c) 2004, 2020 Intel Corporation.  All rights reserved.
// Copyright (c) 2004 Topspin Corporation.  All rights reserved.
// Copyright (c) 2004 Voltaire Corporation.  All rights reserved.
// Copyright (c) 2005 Sun Microsystems, Inc. All rights reserved.
// Copyright (c) 2005, 2006, 2007 Cisco Systems.  All rights reserved.
//

extern "C" {
    pub fn ibdev_emerg(ibdev: *const ib_device, format: *const c_char, ...);
}
extern "C" {
    pub fn ibdev_alert(ibdev: *const ib_device, format: *const c_char, ...);
}
extern "C" {
    pub fn ibdev_crit(ibdev: *const ib_device, format: *const c_char, ...);
}
extern "C" {
    pub fn ibdev_err(ibdev: *const ib_device, format: *const c_char, ...);
}
extern "C" {
    pub fn ibdev_warn(ibdev: *const ib_device, format: *const c_char, ...);
}
extern "C" {
    pub fn ibdev_notice(ibdev: *const ib_device, format: *const c_char, ...);
}
extern "C" {
    pub fn ibdev_info(ibdev: *const ib_device, format: *const c_char, ...);
}

// descriptor check is first to prevent flooding with "callbacks suppressed"

#[repr(C)]
#[derive(Copy, Clone)]
pub union ib_gid {
    pub raw: [u8; 16],
    pub subnet_prefix: __be64,
    pub interface_id: __be64,
    pub global: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_gid_type {
    IB_GID_TYPE_IB = IB_UVERBS_GID_TYPE_IB,
    IB_GID_TYPE_ROCE = IB_UVERBS_GID_TYPE_ROCE_V1,
    IB_GID_TYPE_ROCE_UDP_ENCAP = IB_UVERBS_GID_TYPE_ROCE_V2,
    IB_GID_TYPE_SIZE
}

pub const ROCE_V2_UDP_DPORT: c_int = 4791;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_gid_attr {
    pub ndev: *mut net_device __rcu,
    pub device: *mut ib_device,
    pub gid: ib_gid,
    pub gid_type: ib_gid_type,
    pub index: u16,
    pub port_num: u32,
}

// set the local administered indication
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_transport_type {
    RDMA_TRANSPORT_IB,
    RDMA_TRANSPORT_IWARP,
    RDMA_TRANSPORT_USNIC,
    RDMA_TRANSPORT_USNIC_UDP,
    RDMA_TRANSPORT_UNSPECIFIED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_protocol_type {
    RDMA_PROTOCOL_IB,
    RDMA_PROTOCOL_IBOE,
    RDMA_PROTOCOL_IWARP,
    RDMA_PROTOCOL_USNIC_UDP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_network_type {
    RDMA_NETWORK_IB,
    RDMA_NETWORK_ROCE_V1,
    RDMA_NETWORK_IPV4,
    RDMA_NETWORK_IPV6
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_link_layer {
    IB_LINK_LAYER_UNSPECIFIED,
    IB_LINK_LAYER_INFINIBAND,
    IB_LINK_LAYER_ETHERNET,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_device_cap_flags {
    IB_DEVICE_RESIZE_MAX_WR = IB_UVERBS_DEVICE_RESIZE_MAX_WR,
    IB_DEVICE_BAD_PKEY_CNTR = IB_UVERBS_DEVICE_BAD_PKEY_CNTR,
    IB_DEVICE_BAD_QKEY_CNTR = IB_UVERBS_DEVICE_BAD_QKEY_CNTR,
    IB_DEVICE_RAW_MULTI = IB_UVERBS_DEVICE_RAW_MULTI,
    IB_DEVICE_AUTO_PATH_MIG = IB_UVERBS_DEVICE_AUTO_PATH_MIG,
    IB_DEVICE_CHANGE_PHY_PORT = IB_UVERBS_DEVICE_CHANGE_PHY_PORT,
    IB_DEVICE_UD_AV_PORT_ENFORCE = IB_UVERBS_DEVICE_UD_AV_PORT_ENFORCE,
    IB_DEVICE_CURR_QP_STATE_MOD = IB_UVERBS_DEVICE_CURR_QP_STATE_MOD,
    IB_DEVICE_SHUTDOWN_PORT = IB_UVERBS_DEVICE_SHUTDOWN_PORT,
// IB_DEVICE_INIT_TYPE = IB_UVERBS_DEVICE_INIT_TYPE, (not in use)
    IB_DEVICE_PORT_ACTIVE_EVENT = IB_UVERBS_DEVICE_PORT_ACTIVE_EVENT,
    IB_DEVICE_SYS_IMAGE_GUID = IB_UVERBS_DEVICE_SYS_IMAGE_GUID,
    IB_DEVICE_RC_RNR_NAK_GEN = IB_UVERBS_DEVICE_RC_RNR_NAK_GEN,
    IB_DEVICE_SRQ_RESIZE = IB_UVERBS_DEVICE_SRQ_RESIZE,
    IB_DEVICE_N_NOTIFY_CQ = IB_UVERBS_DEVICE_N_NOTIFY_CQ,

// Reserved, old SEND_W_INV = 1 << 16,
    IB_DEVICE_MEM_WINDOW = IB_UVERBS_DEVICE_MEM_WINDOW,
//
// Devices should set IB_DEVICE_UD_IP_SUM if they support
// insertion of UDP and TCP checksum on outgoing UD IPoIB
// messages and can verify the validity of checksum for
// incoming messages.  Setting this flag implies that the
// IPoIB driver may set NETIF_F_IP_CSUM for datagram mode.
//
    IB_DEVICE_UD_IP_CSUM = IB_UVERBS_DEVICE_UD_IP_CSUM,
    IB_DEVICE_XRC = IB_UVERBS_DEVICE_XRC,

//
// This device supports the IB "base memory management extension",
// which includes support for fast registrations (IB_WR_REG_MR,
// IB_WR_LOCAL_INV and IB_WR_SEND_WITH_INV verbs).  This flag should
// also be set by any iWarp device which must support FRs to comply
// to the iWarp verbs spec.  iWarp devices also support the
// IB_WR_RDMA_READ_WITH_INV verb for RDMA READs that invalidate the
// stag.
//
    IB_DEVICE_MEM_MGT_EXTENSIONS = IB_UVERBS_DEVICE_MEM_MGT_EXTENSIONS,
    IB_DEVICE_MEM_WINDOW_TYPE_2A = IB_UVERBS_DEVICE_MEM_WINDOW_TYPE_2A,
    IB_DEVICE_MEM_WINDOW_TYPE_2B = IB_UVERBS_DEVICE_MEM_WINDOW_TYPE_2B,
    IB_DEVICE_RC_IP_CSUM = IB_UVERBS_DEVICE_RC_IP_CSUM,
// Deprecated. Please use IB_RAW_PACKET_CAP_IP_CSUM.
    IB_DEVICE_RAW_IP_CSUM = IB_UVERBS_DEVICE_RAW_IP_CSUM,
    IB_DEVICE_MANAGED_FLOW_STEERING =
    IB_UVERBS_DEVICE_MANAGED_FLOW_STEERING,
// Deprecated. Please use IB_RAW_PACKET_CAP_SCATTER_FCS.
    IB_DEVICE_RAW_SCATTER_FCS = IB_UVERBS_DEVICE_RAW_SCATTER_FCS,
// The device supports padding incoming writes to cacheline.
    IB_DEVICE_PCI_WRITE_END_PADDING =
    IB_UVERBS_DEVICE_PCI_WRITE_END_PADDING,
// Placement type attributes
    IB_DEVICE_FLUSH_GLOBAL = IB_UVERBS_DEVICE_FLUSH_GLOBAL,
    IB_DEVICE_FLUSH_PERSISTENT = IB_UVERBS_DEVICE_FLUSH_PERSISTENT,
    IB_DEVICE_ATOMIC_WRITE = IB_UVERBS_DEVICE_ATOMIC_WRITE,
    IB_DEVICE_CC_DMA_BOUNCE = IB_UVERBS_DEVICE_CC_DMA_BOUNCE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_kernel_cap_flags {
//
// This device supports a per-device lkey or stag that can be
// used without performing a memory registration for the local
// memory.  Note that ULPs should never check this flag, but
// instead of use the local_dma_lkey flag in the ib_pd structure,
// which will always contain a usable lkey.
//
    IBK_LOCAL_DMA_LKEY = 1 << 0,
// IB_QP_CREATE_INTEGRITY_EN is supported to implement T10-PI
    IBK_INTEGRITY_HANDOVER = 1 << 1,
// IB_ACCESS_ON_DEMAND is supported during reg_user_mr()
    IBK_ON_DEMAND_PAGING = 1 << 2,
// IB_MR_TYPE_SG_GAPS is supported
    IBK_SG_GAPS_REG = 1 << 3,
// Driver supports RDMA_NLDEV_CMD_DELLINK
    IBK_ALLOW_USER_UNREG = 1 << 4,

// ipoib will use IB_QP_CREATE_BLOCK_MULTICAST_LOOPBACK
    IBK_BLOCK_MULTICAST_LOOPBACK = 1 << 5,
// iopib will use IB_QP_CREATE_IPOIB_UD_LSO for its QPs
    IBK_UD_TSO = 1 << 6,
// iopib will use the device ops:
// get_vf_config
// get_vf_guid
// get_vf_stats
// set_vf_guid
// set_vf_link_state
//
    IBK_VIRTUAL_FUNCTION = 1 << 7,
// ipoib will use IB_QP_CREATE_NETDEV_USE for its QPs
    IBK_RDMA_NETDEV_OPA = 1 << 8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_atomic_cap {
    IB_ATOMIC_NONE,
    IB_ATOMIC_HCA,
    IB_ATOMIC_GLOB
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_odp_general_cap_bits {
    IB_ODP_SUPPORT		= IB_UVERBS_ODP_SUPPORT,
    IB_ODP_SUPPORT_IMPLICIT = IB_UVERBS_ODP_SUPPORT_IMPLICIT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_odp_transport_cap_bits {
    IB_ODP_SUPPORT_SEND	= IB_UVERBS_ODP_SUPPORT_SEND,
    IB_ODP_SUPPORT_RECV	= IB_UVERBS_ODP_SUPPORT_RECV,
    IB_ODP_SUPPORT_WRITE	= IB_UVERBS_ODP_SUPPORT_WRITE,
    IB_ODP_SUPPORT_READ	= IB_UVERBS_ODP_SUPPORT_READ,
    IB_ODP_SUPPORT_ATOMIC	= IB_UVERBS_ODP_SUPPORT_ATOMIC,
    IB_ODP_SUPPORT_SRQ_RECV	= IB_UVERBS_ODP_SUPPORT_SRQ_RECV,
    IB_ODP_SUPPORT_FLUSH	= IB_UVERBS_ODP_SUPPORT_FLUSH,
    IB_ODP_SUPPORT_ATOMIC_WRITE	= IB_UVERBS_ODP_SUPPORT_ATOMIC_WRITE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_odp_caps {
    pub general_caps: u64,
    pub rc_odp_caps: u32,
    pub uc_odp_caps: u32,
    pub ud_odp_caps: u32,
    pub xrc_odp_caps: u32,
    pub per_transport_caps: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rss_caps {
// Corresponding bit will be set if qp type from
// 'enum ib_qp_type' is supported, e.g.
// supported_qpts |= 1 << IB_QPT_UD
//
    pub supported_qpts: u32,
    pub max_rwq_indirection_tables: u32,
    pub max_rwq_indirection_table_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_tm_cap_flags {
// Support tag matching with rendezvous offload for RC transport
    IB_TM_CAP_RNDV_RC = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_tm_caps {
// Max size of RNDV header
    pub max_rndv_hdr_size: u32,
// Max number of entries in tag matching list
    pub max_num_tags: u32,
// From enum ib_tm_cap_flags
    pub flags: u32,
// Max number of outstanding list operations
    pub max_ops: u32,
// Max number of SGE in tag matching entry
    pub max_sge: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cq_init_attr {
    pub cqe: c_uint,
    pub comp_vector: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cq_attr_mask {
    IB_CQ_MODERATE = 1 << 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cq_caps {
    pub max_cq_moderation_count: u16,
    pub max_cq_moderation_period: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_mr_attr {
    pub length: u64,
    pub offset: u64,
    pub access_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm_alloc_attr {
    pub length: u64,
    pub alignment: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_device_attr {
    pub fw_ver: u64,
    pub sys_image_guid: __be64,
    pub max_mr_size: u64,
    pub page_size_cap: u64,
    pub vendor_id: u32,
    pub vendor_part_id: u32,
    pub hw_ver: u32,
    pub max_qp: u32,
    pub max_qp_wr: u32,
    pub device_cap_flags: u64,
    pub kernel_cap_flags: u64,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_sge_rd: u32,
    pub max_cq: u32,
    pub max_cqe: u32,
    pub max_mr: u32,
    pub max_pd: u32,
    pub max_qp_rd_atom: u32,
    pub max_ee_rd_atom: u32,
    pub max_res_rd_atom: u32,
    pub max_qp_init_rd_atom: u32,
    pub max_ee_init_rd_atom: u32,
    pub atomic_cap: ib_atomic_cap,
    pub masked_atomic_cap: ib_atomic_cap,
    pub max_ee: u32,
    pub max_rdd: u32,
    pub max_mw: u32,
    pub max_raw_ipv6_qp: u32,
    pub max_raw_ethy_qp: u32,
    pub max_mcast_grp: u32,
    pub max_mcast_qp_attach: u32,
    pub max_total_mcast_qp_attach: u32,
    pub max_ah: u32,
    pub max_srq: u32,
    pub max_srq_wr: u32,
    pub max_srq_sge: u32,
    pub max_fast_reg_page_list_len: c_uint,
    pub max_pi_fast_reg_page_list_len: c_uint,
    pub max_pkeys: u16,
    pub local_ca_ack_delay: u8,
    pub sig_prot_cap: c_int,
    pub sig_guard_cap: c_int,
    pub odp_caps: ib_odp_caps,
    pub timestamp_mask: u64,
    pub /: *mut *mut uint64_t hca_core_clock; / in KHZ,
    pub rss_caps: ib_rss_caps,
    pub max_wq_type_rq: u32,
    pub /: *mut *mut u32 raw_packet_caps; / Use ib_raw_packet_caps enum,
    pub tm_caps: ib_tm_caps,
    pub cq_caps: ib_cq_caps,
    pub max_dm_size: u64,
// Max entries for sgl for optimized performance per READ
    pub max_sgl_rd: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mtu {
    IB_MTU_256  = 1,
    IB_MTU_512  = 2,
    IB_MTU_1024 = 3,
    IB_MTU_2048 = 4,
    IB_MTU_4096 = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum opa_mtu {
    OPA_MTU_8192 = 6,
    OPA_MTU_10240 = 7
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_port_state {
    IB_PORT_NOP		= 0,
    IB_PORT_DOWN		= 1,
    IB_PORT_INIT		= 2,
    IB_PORT_ARMED		= 3,
    IB_PORT_ACTIVE		= 4,
    IB_PORT_ACTIVE_DEFER	= 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_port_phys_state {
    IB_PORT_PHYS_STATE_SLEEP = 1,
    IB_PORT_PHYS_STATE_POLLING = 2,
    IB_PORT_PHYS_STATE_DISABLED = 3,
    IB_PORT_PHYS_STATE_PORT_CONFIGURATION_TRAINING = 4,
    IB_PORT_PHYS_STATE_LINK_UP = 5,
    IB_PORT_PHYS_STATE_LINK_ERROR_RECOVERY = 6,
    IB_PORT_PHYS_STATE_PHY_TEST = 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_port_width {
    IB_WIDTH_1X	= 1,
    IB_WIDTH_2X	= 16,
    IB_WIDTH_4X	= 2,
    IB_WIDTH_8X	= 4,
    IB_WIDTH_12X	= 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_port_speed {
    IB_SPEED_SDR	= 1,
    IB_SPEED_DDR	= 2,
    IB_SPEED_QDR	= 4,
    IB_SPEED_FDR10	= 8,
    IB_SPEED_FDR	= 16,
    IB_SPEED_EDR	= 32,
    IB_SPEED_HDR	= 64,
    IB_SPEED_NDR	= 128,
    IB_SPEED_XDR	= 256,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_stat_flag {
    IB_STAT_FLAG_OPTIONAL = 1 << 0,
}

//
// struct rdma_stat_desc - description of one rdma stat/counter
// @name: The name of the counter
// @flags: Flags of the counter; For example, IB_STAT_FLAG_OPTIONAL
// @priv: Driver private information; Core code should not use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_stat_desc {
    pub name: *const c_char,
    pub flags: c_uint,
    pub priv: *const c_void,
}

//
// struct rdma_hw_stats - collection of hardware stats and their management
// @lock: Mutex to protect parallel write access to lifespan and values
// of counters, which are 64bits and not guaranteed to be written
// atomicaly on 32bits systems.
// @timestamp: Used by the core code to track when the last update was
// @lifespan: Used by the core code to determine how old the counters
// should be before being updated again.  Stored in jiffies, defaults
// to 10 milliseconds, drivers can override the default be specifying
// their own value during their allocation routine.
// @descs: Array of pointers to static descriptors used for the counters
// in directory.
// @is_disabled: A bitmap to indicate each counter is currently disabled
// or not.
// @num_counters: How many hardware counters there are.  If name is
// shorter than this number, a kernel oops will result.  Driver authors
// are encouraged to leave BUILD_BUG_ON(ARRAY_SIZE(@name) < num_counters)
// in their code to prevent this.
// @value: Array of u64 counters that are accessed by the sysfs code and
// filled in by the drivers get_stats routine
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_hw_stats {
    pub /: *mut *mut mutex lock; / Protect lifespan and values[],
    pub timestamp: c_ulong,
    pub lifespan: c_ulong,
    pub descs: *const rdma_stat_desc,
    pub is_disabled: *mut c_ulong,
    pub num_counters: c_int,
    pub __counted_by(num_counters): u64 value[],
}

pub const RDMA_HW_STATS_DEFAULT_LIFESPAN: c_int = 10;
extern "C" {
    pub fn rdma_free_hw_stats_struct(stats: *mut rdma_hw_stats);
}
// Define bits for the various functionality this port needs to be supported by
// the core.
//
// Management                           0x00000FFF
pub const RDMA_CORE_CAP_IB_MAD: c_uint = 0x00000001;
pub const RDMA_CORE_CAP_IB_SMI: c_uint = 0x00000002;
pub const RDMA_CORE_CAP_IB_CM: c_uint = 0x00000004;
pub const RDMA_CORE_CAP_IW_CM: c_uint = 0x00000008;
pub const RDMA_CORE_CAP_IB_SA: c_uint = 0x00000010;
pub const RDMA_CORE_CAP_OPA_MAD: c_uint = 0x00000020;
// Address format                       0x000FF000
pub const RDMA_CORE_CAP_AF_IB: c_uint = 0x00001000;
pub const RDMA_CORE_CAP_ETH_AH: c_uint = 0x00002000;
pub const RDMA_CORE_CAP_OPA_AH: c_uint = 0x00004000;
pub const RDMA_CORE_CAP_IB_GRH_REQUIRED: c_uint = 0x00008000;
// Protocol                             0xFFF00000
pub const RDMA_CORE_CAP_PROT_IB: c_uint = 0x00100000;
pub const RDMA_CORE_CAP_PROT_ROCE: c_uint = 0x00200000;
pub const RDMA_CORE_CAP_PROT_IWARP: c_uint = 0x00400000;
pub const RDMA_CORE_CAP_PROT_ROCE_UDP_ENCAP: c_uint = 0x00800000;
pub const RDMA_CORE_CAP_PROT_RAW_PACKET: c_uint = 0x01000000;
pub const RDMA_CORE_CAP_PROT_USNIC: c_uint = 0x02000000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_attr {
    pub subnet_prefix: u64,
    pub state: ib_port_state,
    pub max_mtu: ib_mtu,
    pub active_mtu: ib_mtu,
    pub phys_mtu: u32,
    pub gid_tbl_len: c_int,
    pub ip_gids:1: c_uint,
// This is the value from PortInfo CapabilityMask, defined by IBA
    pub port_cap_flags: u32,
    pub max_msg_sz: u32,
    pub bad_pkey_cntr: u32,
    pub qkey_viol_cntr: u32,
    pub pkey_tbl_len: u16,
    pub sm_lid: u32,
    pub lid: u32,
    pub lmc: u8,
    pub max_vl_num: u8,
    pub sm_sl: u8,
    pub subnet_timeout: u8,
    pub init_type_reply: u8,
    pub active_width: u8,
    pub active_speed: u16,
    pub phys_state: u8,
    pub port_cap_flags2: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_device_modify_flags {
    IB_DEVICE_MODIFY_SYS_IMAGE_GUID	= 1 << 0,
    IB_DEVICE_MODIFY_NODE_DESC	= 1 << 1
}

pub const IB_DEVICE_NODE_DESC_MAX: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_device_modify {
    pub sys_image_guid: u64,
    pub node_desc: [c_char; IB_DEVICE_NODE_DESC_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_port_modify_flags {
    IB_PORT_SHUTDOWN		= 1,
    IB_PORT_INIT_TYPE		= (1<<2),
    IB_PORT_RESET_QKEY_CNTR		= (1<<3),
    IB_PORT_OPA_MASK_CHG		= (1<<4)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_modify {
    pub set_port_cap_mask: u32,
    pub clr_port_cap_mask: u32,
    pub init_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_event_type {
    IB_EVENT_CQ_ERR,
    IB_EVENT_QP_FATAL,
    IB_EVENT_QP_REQ_ERR,
    IB_EVENT_QP_ACCESS_ERR,
    IB_EVENT_COMM_EST,
    IB_EVENT_SQ_DRAINED,
    IB_EVENT_PATH_MIG,
    IB_EVENT_PATH_MIG_ERR,
    IB_EVENT_DEVICE_FATAL,
    IB_EVENT_PORT_ACTIVE,
    IB_EVENT_PORT_ERR,
    IB_EVENT_LID_CHANGE,
    IB_EVENT_PKEY_CHANGE,
    IB_EVENT_SM_CHANGE,
    IB_EVENT_SRQ_ERR,
    IB_EVENT_SRQ_LIMIT_REACHED,
    IB_EVENT_QP_LAST_WQE_REACHED,
    IB_EVENT_CLIENT_REREGISTER,
    IB_EVENT_GID_CHANGE,
    IB_EVENT_WQ_FATAL,
    IB_EVENT_DEVICE_SPEED_CHANGE,
}

extern "C" {
    pub fn ib_event_msg(event: ib_event_type) -> *const char __attribute_const__;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_event {
    pub device: *mut ib_device,
    pub cq: *mut ib_cq,
    pub qp: *mut ib_qp,
    pub srq: *mut ib_srq,
    pub wq: *mut ib_wq,
    pub port_num: u32,
    pub element: },
    pub event: ib_event_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_event_handler {
    pub device: *mut ib_device,
    pub ): *mut *mut *mut void (handler)(struct ib_event_handler , struct ib_event,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_global_route {
    pub sgid_attr: *const ib_gid_attr,
    pub dgid: ib_gid,
    pub flow_label: u32,
    pub sgid_index: u8,
    pub hop_limit: u8,
    pub traffic_class: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_grh {
    pub version_tclass_flow: __be32,
    pub paylen: __be16,
    pub next_hdr: u8,
    pub hop_limit: u8,
    pub sgid: ib_gid,
    pub dgid: ib_gid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rdma_network_hdr {
    pub ibgrh: ib_grh,
// The IB spec states that if it's IPv4, the header
// is located in the last 20 bytes of the header.
//
    pub reserved: [u8; 20],
    pub roce4grh: iphdr,
}

pub const IB_QPN_MASK: c_uint = 0xFFFFFF;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_ah_flags {
    IB_AH_GRH	= 1
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_rate {
    IB_RATE_PORT_CURRENT = 0,
    IB_RATE_2_5_GBPS = 2,
    IB_RATE_5_GBPS   = 5,
    IB_RATE_10_GBPS  = 3,
    IB_RATE_20_GBPS  = 6,
    IB_RATE_30_GBPS  = 4,
    IB_RATE_40_GBPS  = 7,
    IB_RATE_60_GBPS  = 8,
    IB_RATE_80_GBPS  = 9,
    IB_RATE_120_GBPS = 10,
    IB_RATE_14_GBPS  = 11,
    IB_RATE_56_GBPS  = 12,
    IB_RATE_112_GBPS = 13,
    IB_RATE_168_GBPS = 14,
    IB_RATE_25_GBPS  = 15,
    IB_RATE_100_GBPS = 16,
    IB_RATE_200_GBPS = 17,
    IB_RATE_300_GBPS = 18,
    IB_RATE_28_GBPS  = 19,
    IB_RATE_50_GBPS  = 20,
    IB_RATE_400_GBPS = 21,
    IB_RATE_600_GBPS = 22,
    IB_RATE_800_GBPS = 23,
    IB_RATE_1600_GBPS = 25,
}

//
// ib_rate_to_mult - Convert the IB rate enum to a multiple of the
// base rate of 2.5 Gbit/sec.  For example, IB_RATE_5_GBPS will be
// converted to 2, since 5 Gbit/sec is 2 * 2.5 Gbit/sec.
// @rate: rate to convert.
//
extern "C" {
    pub fn ib_rate_to_mult(rate: ib_rate) -> __attribute_const__ int;
}
//
// ib_rate_to_mbps - Convert the IB rate enum to Mbps.
// For example, IB_RATE_2_5_GBPS will be converted to 2500.
// @rate: rate to convert.
//
extern "C" {
    pub fn ib_rate_to_mbps(rate: ib_rate) -> __attribute_const__ int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_speed_info {
    pub str: *const c_char,
    pub /: *mut *mut int rate; / in deci-Gb/sec (100 MBps units),
}

//
// ib_port_attr_to_speed_info - Convert port attributes to speed information
// @attr: Port attributes containing active_speed and active_width
// @speed_info: Speed information to return
//
// Returns 0 on success, -EINVAL on error.
//
// enum ib_mr_type - memory region type
// @IB_MR_TYPE_MEM_REG:       memory region that is used for
// normal registration
// @IB_MR_TYPE_SG_GAPS:       memory region that is capable to
// register any arbitrary sg lists (without
// the normal mr constraints - see
// ib_map_mr_sg)
// @IB_MR_TYPE_DM:            memory region that is used for device
// memory registration
// @IB_MR_TYPE_USER:          memory region that is used for the user-space
// application
// @IB_MR_TYPE_DMA:           memory region that is used for DMA operations
// without address translations (VA=PA)
// @IB_MR_TYPE_INTEGRITY:     memory region that is used for
// data integrity operations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mr_type {
    IB_MR_TYPE_MEM_REG,
    IB_MR_TYPE_SG_GAPS,
    IB_MR_TYPE_DM,
    IB_MR_TYPE_USER,
    IB_MR_TYPE_DMA,
    IB_MR_TYPE_INTEGRITY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mr_status_check {
    IB_MR_CHECK_SIG_STATUS = 1,
}

//
// struct ib_mr_status - Memory region status container
//
// @fail_status: Bitmask of MR checks status. For each
// failed check a corresponding status bit is set.
// @sig_err: Additional info for IB_MR_CEHCK_SIG_STATUS
// failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mr_status {
    pub fail_status: u32,
    pub sig_err: ib_sig_err,
}

//
// mult_to_ib_rate - Convert a multiple of 2.5 Gbit/sec to an IB rate
// enum.
// @mult: multiple to convert.
//
extern "C" {
    pub fn mult_to_ib_rate(mult: c_int) -> __attribute_const__ enum ib_rate;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ah_init_attr {
    pub ah_attr: *mut rdma_ah_attr,
    pub flags: u32,
    pub xmit_slave: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_ah_attr_type {
    RDMA_AH_ATTR_TYPE_UNDEFINED,
    RDMA_AH_ATTR_TYPE_IB,
    RDMA_AH_ATTR_TYPE_ROCE,
    RDMA_AH_ATTR_TYPE_OPA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ah_attr {
    pub dlid: u16,
    pub src_path_bits: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct roce_ah_attr {
    pub dmac: [u8; ETH_ALEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct opa_ah_attr {
    pub dlid: u32,
    pub src_path_bits: u8,
    pub make_grd: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_ah_attr {
    pub grh: ib_global_route,
    pub sl: u8,
    pub static_rate: u8,
    pub port_num: u32,
    pub ah_flags: u8,
    pub type: rdma_ah_attr_type,
    pub ib: ib_ah_attr,
    pub roce: roce_ah_attr,
    pub opa: opa_ah_attr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wc_status {
    IB_WC_SUCCESS,
    IB_WC_LOC_LEN_ERR,
    IB_WC_LOC_QP_OP_ERR,
    IB_WC_LOC_EEC_OP_ERR,
    IB_WC_LOC_PROT_ERR,
    IB_WC_WR_FLUSH_ERR,
    IB_WC_MW_BIND_ERR,
    IB_WC_BAD_RESP_ERR,
    IB_WC_LOC_ACCESS_ERR,
    IB_WC_REM_INV_REQ_ERR,
    IB_WC_REM_ACCESS_ERR,
    IB_WC_REM_OP_ERR,
    IB_WC_RETRY_EXC_ERR,
    IB_WC_RNR_RETRY_EXC_ERR,
    IB_WC_LOC_RDD_VIOL_ERR,
    IB_WC_REM_INV_RD_REQ_ERR,
    IB_WC_REM_ABORT_ERR,
    IB_WC_INV_EECN_ERR,
    IB_WC_INV_EEC_STATE_ERR,
    IB_WC_FATAL_ERR,
    IB_WC_RESP_TIMEOUT_ERR,
    IB_WC_GENERAL_ERR
}

extern "C" {
    pub fn ib_wc_status_msg(status: ib_wc_status) -> *const char __attribute_const__;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wc_opcode {
    IB_WC_SEND = IB_UVERBS_WC_SEND,
    IB_WC_RDMA_WRITE = IB_UVERBS_WC_RDMA_WRITE,
    IB_WC_RDMA_READ = IB_UVERBS_WC_RDMA_READ,
    IB_WC_COMP_SWAP = IB_UVERBS_WC_COMP_SWAP,
    IB_WC_FETCH_ADD = IB_UVERBS_WC_FETCH_ADD,
    IB_WC_BIND_MW = IB_UVERBS_WC_BIND_MW,
    IB_WC_LOCAL_INV = IB_UVERBS_WC_LOCAL_INV,
    IB_WC_LSO = IB_UVERBS_WC_TSO,
    IB_WC_ATOMIC_WRITE = IB_UVERBS_WC_ATOMIC_WRITE,
    IB_WC_REG_MR,
    IB_WC_MASKED_COMP_SWAP,
    IB_WC_MASKED_FETCH_ADD,
    IB_WC_FLUSH = IB_UVERBS_WC_FLUSH,
//
// Set value of IB_WC_RECV so consumers can test if a completion is a
// receive by testing (opcode & IB_WC_RECV).
//
    IB_WC_RECV			= 1 << 7,
    IB_WC_RECV_RDMA_WITH_IMM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wc_flags {
    IB_WC_GRH		= 1,
    IB_WC_WITH_IMM		= (1<<1),
    IB_WC_WITH_INVALIDATE	= (1<<2),
    IB_WC_IP_CSUM_OK	= (1<<3),
    IB_WC_WITH_SMAC		= (1<<4),
    IB_WC_WITH_VLAN		= (1<<5),
    IB_WC_WITH_NETWORK_HDR_TYPE	= (1<<6),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_wc {
    pub wr_id: u64,
    pub wr_cqe: *mut ib_cqe,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_cq_notify_flags {
    IB_CQ_SOLICITED			= 1 << 0,
    IB_CQ_NEXT_COMP			= 1 << 1,
    IB_CQ_SOLICITED_MASK		= IB_CQ_SOLICITED | IB_CQ_NEXT_COMP,
    IB_CQ_REPORT_MISSED_EVENTS	= 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_srq_type {
    IB_SRQT_BASIC = IB_UVERBS_SRQT_BASIC,
    IB_SRQT_XRC = IB_UVERBS_SRQT_XRC,
    IB_SRQT_TM = IB_UVERBS_SRQT_TM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_srq_attr_mask {
    IB_SRQ_MAX_WR	= 1 << 0,
    IB_SRQ_LIMIT	= 1 << 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_srq_attr {
    pub max_wr: u32,
    pub max_sge: u32,
    pub srq_limit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_srq_init_attr {
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub srq_context: *mut c_void,
    pub attr: ib_srq_attr,
    pub srq_type: ib_srq_type,
    pub cq: *mut ib_cq,
    pub xrcd: *mut ib_xrcd,
    pub xrc: },
    pub max_num_tags: u32,
    pub tag_matching: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_qp_cap {
    pub max_send_wr: u32,
    pub max_recv_wr: u32,
    pub max_send_sge: u32,
    pub max_recv_sge: u32,
    pub max_inline_data: u32,
//
// Maximum number of rdma_rw_ctx structures in flight at a time.
// ib_create_qp() will calculate the right amount of needed WRs
// and MRs based on this.
//
    pub max_rdma_ctxs: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_sig_type {
    IB_SIGNAL_ALL_WR,
    IB_SIGNAL_REQ_WR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_qp_type {
//
// IB_QPT_SMI and IB_QPT_GSI have to be the first two entries
// here (and in that order) since the MAD layer uses them as
// indices into a 2-entry table.
//
    IB_QPT_SMI,
    IB_QPT_GSI,

    IB_QPT_RC = IB_UVERBS_QPT_RC,
    IB_QPT_UC = IB_UVERBS_QPT_UC,
    IB_QPT_UD = IB_UVERBS_QPT_UD,
    IB_QPT_RAW_IPV6,
    IB_QPT_RAW_ETHERTYPE,
    IB_QPT_RAW_PACKET = IB_UVERBS_QPT_RAW_PACKET,
    IB_QPT_XRC_INI = IB_UVERBS_QPT_XRC_INI,
    IB_QPT_XRC_TGT = IB_UVERBS_QPT_XRC_TGT,
    IB_QPT_MAX,
    IB_QPT_DRIVER = IB_UVERBS_QPT_DRIVER,
// Reserve a range for qp types internal to the low level driver.
// These qp types will not be visible at the IB core layer, so the
// IB_QPT_MAX usages should not be affected in the core layer
//
    IB_QPT_RESERVED1 = 0x1000,
    IB_QPT_RESERVED2,
    IB_QPT_RESERVED3,
    IB_QPT_RESERVED4,
    IB_QPT_RESERVED5,
    IB_QPT_RESERVED6,
    IB_QPT_RESERVED7,
    IB_QPT_RESERVED8,
    IB_QPT_RESERVED9,
    IB_QPT_RESERVED10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_qp_create_flags {
    IB_QP_CREATE_IPOIB_UD_LSO		= 1 << 0,
    IB_QP_CREATE_BLOCK_MULTICAST_LOOPBACK	=
    IB_UVERBS_QP_CREATE_BLOCK_MULTICAST_LOOPBACK,
    IB_QP_CREATE_CROSS_CHANNEL              = 1 << 2,
    IB_QP_CREATE_MANAGED_SEND               = 1 << 3,
    IB_QP_CREATE_MANAGED_RECV               = 1 << 4,
    IB_QP_CREATE_NETIF_QP			= 1 << 5,
    IB_QP_CREATE_INTEGRITY_EN		= 1 << 6,
    IB_QP_CREATE_NETDEV_USE			= 1 << 7,
    IB_QP_CREATE_SCATTER_FCS		=
    IB_UVERBS_QP_CREATE_SCATTER_FCS,
    IB_QP_CREATE_CVLAN_STRIPPING		=
    IB_UVERBS_QP_CREATE_CVLAN_STRIPPING,
    IB_QP_CREATE_SOURCE_QPN			= 1 << 10,
    IB_QP_CREATE_PCI_WRITE_END_PADDING	=
    IB_UVERBS_QP_CREATE_PCI_WRITE_END_PADDING,
// reserve bits 26-31 for low level drivers' internal use
    IB_QP_CREATE_RESERVED_START		= 1 << 26,
    IB_QP_CREATE_RESERVED_END		= 1 << 31,
}

//
// Note: users may not call ib_close_qp or ib_destroy_qp from the event_handler
// callback to destroy the passed in QP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_qp_init_attr {
// This callback occurs in workqueue context
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub qp_context: *mut c_void,
    pub send_cq: *mut ib_cq,
    pub recv_cq: *mut ib_cq,
    pub srq: *mut ib_srq,
    pub /: *mut *mut *mut ib_xrcd xrcd; / XRC TGT QPs only,
    pub cap: ib_qp_cap,
    pub sq_sig_type: ib_sig_type,
    pub qp_type: ib_qp_type,
    pub create_flags: u32,
//
// Only needed for special QP types, or when using the RW API.
//
    pub port_num: u32,
    pub rwq_ind_tbl: *mut ib_rwq_ind_table,
    pub source_qpn: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_qp_open_attr {
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub qp_context: *mut c_void,
    pub qp_num: u32,
    pub qp_type: ib_qp_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_rnr_timeout {
    IB_RNR_TIMER_655_36 =  0,
    IB_RNR_TIMER_000_01 =  1,
    IB_RNR_TIMER_000_02 =  2,
    IB_RNR_TIMER_000_03 =  3,
    IB_RNR_TIMER_000_04 =  4,
    IB_RNR_TIMER_000_06 =  5,
    IB_RNR_TIMER_000_08 =  6,
    IB_RNR_TIMER_000_12 =  7,
    IB_RNR_TIMER_000_16 =  8,
    IB_RNR_TIMER_000_24 =  9,
    IB_RNR_TIMER_000_32 = 10,
    IB_RNR_TIMER_000_48 = 11,
    IB_RNR_TIMER_000_64 = 12,
    IB_RNR_TIMER_000_96 = 13,
    IB_RNR_TIMER_001_28 = 14,
    IB_RNR_TIMER_001_92 = 15,
    IB_RNR_TIMER_002_56 = 16,
    IB_RNR_TIMER_003_84 = 17,
    IB_RNR_TIMER_005_12 = 18,
    IB_RNR_TIMER_007_68 = 19,
    IB_RNR_TIMER_010_24 = 20,
    IB_RNR_TIMER_015_36 = 21,
    IB_RNR_TIMER_020_48 = 22,
    IB_RNR_TIMER_030_72 = 23,
    IB_RNR_TIMER_040_96 = 24,
    IB_RNR_TIMER_061_44 = 25,
    IB_RNR_TIMER_081_92 = 26,
    IB_RNR_TIMER_122_88 = 27,
    IB_RNR_TIMER_163_84 = 28,
    IB_RNR_TIMER_245_76 = 29,
    IB_RNR_TIMER_327_68 = 30,
    IB_RNR_TIMER_491_52 = 31
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_qp_attr_mask {
    IB_QP_STATE			= 1,
    IB_QP_CUR_STATE			= (1<<1),
    IB_QP_EN_SQD_ASYNC_NOTIFY	= (1<<2),
    IB_QP_ACCESS_FLAGS		= (1<<3),
    IB_QP_PKEY_INDEX		= (1<<4),
    IB_QP_PORT			= (1<<5),
    IB_QP_QKEY			= (1<<6),
    IB_QP_AV			= (1<<7),
    IB_QP_PATH_MTU			= (1<<8),
    IB_QP_TIMEOUT			= (1<<9),
    IB_QP_RETRY_CNT			= (1<<10),
    IB_QP_RNR_RETRY			= (1<<11),
    IB_QP_RQ_PSN			= (1<<12),
    IB_QP_MAX_QP_RD_ATOMIC		= (1<<13),
    IB_QP_ALT_PATH			= (1<<14),
    IB_QP_MIN_RNR_TIMER		= (1<<15),
    IB_QP_SQ_PSN			= (1<<16),
    IB_QP_MAX_DEST_RD_ATOMIC	= (1<<17),
    IB_QP_PATH_MIG_STATE		= (1<<18),
    IB_QP_CAP			= (1<<19),
    IB_QP_DEST_QPN			= (1<<20),
    IB_QP_RESERVED1			= (1<<21),
    IB_QP_RESERVED2			= (1<<22),
    IB_QP_RESERVED3			= (1<<23),
    IB_QP_RESERVED4			= (1<<24),
    IB_QP_RATE_LIMIT		= (1<<25),

    IB_QP_ATTR_STANDARD_BITS = GENMASK(20, 0),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_qp_state {
    IB_QPS_RESET,
    IB_QPS_INIT,
    IB_QPS_RTR,
    IB_QPS_RTS,
    IB_QPS_SQD,
    IB_QPS_SQE,
    IB_QPS_ERR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mig_state {
    IB_MIG_MIGRATED,
    IB_MIG_REARM,
    IB_MIG_ARMED
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mw_type {
    IB_MW_TYPE_1 = 1,
    IB_MW_TYPE_2 = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_qp_attr {
    pub qp_state: ib_qp_state,
    pub cur_qp_state: ib_qp_state,
    pub path_mtu: ib_mtu,
    pub path_mig_state: ib_mig_state,
    pub qkey: u32,
    pub rq_psn: u32,
    pub sq_psn: u32,
    pub dest_qp_num: u32,
    pub qp_access_flags: c_int,
    pub cap: ib_qp_cap,
    pub ah_attr: rdma_ah_attr,
    pub alt_ah_attr: rdma_ah_attr,
    pub pkey_index: u16,
    pub alt_pkey_index: u16,
    pub en_sqd_async_notify: u8,
    pub sq_draining: u8,
    pub max_rd_atomic: u8,
    pub max_dest_rd_atomic: u8,
    pub min_rnr_timer: u8,
    pub port_num: u32,
    pub timeout: u8,
    pub retry_cnt: u8,
    pub rnr_retry: u8,
    pub alt_port_num: u32,
    pub alt_timeout: u8,
    pub rate_limit: u32,
    pub xmit_slave: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wr_opcode {
// These are shared with userspace
    IB_WR_RDMA_WRITE = IB_UVERBS_WR_RDMA_WRITE,
    IB_WR_RDMA_WRITE_WITH_IMM = IB_UVERBS_WR_RDMA_WRITE_WITH_IMM,
    IB_WR_SEND = IB_UVERBS_WR_SEND,
    IB_WR_SEND_WITH_IMM = IB_UVERBS_WR_SEND_WITH_IMM,
    IB_WR_RDMA_READ = IB_UVERBS_WR_RDMA_READ,
    IB_WR_ATOMIC_CMP_AND_SWP = IB_UVERBS_WR_ATOMIC_CMP_AND_SWP,
    IB_WR_ATOMIC_FETCH_AND_ADD = IB_UVERBS_WR_ATOMIC_FETCH_AND_ADD,
    IB_WR_BIND_MW = IB_UVERBS_WR_BIND_MW,
    IB_WR_LSO = IB_UVERBS_WR_TSO,
    IB_WR_SEND_WITH_INV = IB_UVERBS_WR_SEND_WITH_INV,
    IB_WR_RDMA_READ_WITH_INV = IB_UVERBS_WR_RDMA_READ_WITH_INV,
    IB_WR_LOCAL_INV = IB_UVERBS_WR_LOCAL_INV,
    IB_WR_MASKED_ATOMIC_CMP_AND_SWP =
    IB_UVERBS_WR_MASKED_ATOMIC_CMP_AND_SWP,
    IB_WR_MASKED_ATOMIC_FETCH_AND_ADD =
    IB_UVERBS_WR_MASKED_ATOMIC_FETCH_AND_ADD,
    IB_WR_FLUSH = IB_UVERBS_WR_FLUSH,
    IB_WR_ATOMIC_WRITE = IB_UVERBS_WR_ATOMIC_WRITE,

// These are kernel only and can not be issued by userspace
    IB_WR_REG_MR = 0x20,
    IB_WR_REG_MR_INTEGRITY,

// reserve values for low level drivers' internal use.
// These values will not be used at all in the ib core layer.
//
    IB_WR_RESERVED1 = 0xf0,
    IB_WR_RESERVED2,
    IB_WR_RESERVED3,
    IB_WR_RESERVED4,
    IB_WR_RESERVED5,
    IB_WR_RESERVED6,
    IB_WR_RESERVED7,
    IB_WR_RESERVED8,
    IB_WR_RESERVED9,
    IB_WR_RESERVED10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_send_flags {
    IB_SEND_FENCE		= 1,
    IB_SEND_SIGNALED	= (1<<1),
    IB_SEND_SOLICITED	= (1<<2),
    IB_SEND_INLINE		= (1<<3),
    IB_SEND_IP_CSUM		= (1<<4),

// reserve bits 26-31 for low level drivers' internal use
    IB_SEND_RESERVED_START	= (1 << 26),
    IB_SEND_RESERVED_END	= (1 << 31),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_sge {
    pub addr: u64,
    pub length: u32,
    pub lkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cqe {
    pub wc): *mut *mut *mut void (done)(struct ib_cq cq, struct ib_wc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_send_wr {
    pub next: *mut ib_send_wr,
    pub wr_id: u64,
    pub wr_cqe: *mut ib_cqe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rdma_wr {
    pub wr: ib_send_wr,
    pub remote_addr: u64,
    pub rkey: u32,
}

extern "C" {
    pub fn container_of(_arg: wr, ib_rdma_wr: struct, _arg: wr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_atomic_wr {
    pub wr: ib_send_wr,
    pub remote_addr: u64,
    pub compare_add: u64,
    pub swap: u64,
    pub compare_add_mask: u64,
    pub swap_mask: u64,
    pub rkey: u32,
}

extern "C" {
    pub fn container_of(_arg: wr, ib_atomic_wr: struct, _arg: wr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ud_wr {
    pub wr: ib_send_wr,
    pub ah: *mut ib_ah,
    pub header: *mut c_void,
    pub hlen: c_int,
    pub mss: c_int,
    pub remote_qpn: u32,
    pub remote_qkey: u32,
    pub /: *mut *mut u16 pkey_index; / valid for GSI only,
    pub /: *mut *mut u32 port_num; / valid for DR SMPs on switch only,
}

extern "C" {
    pub fn container_of(_arg: wr, ib_ud_wr: struct, _arg: wr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_reg_wr {
    pub wr: ib_send_wr,
    pub mr: *mut ib_mr,
    pub key: u32,
    pub access: c_int,
}

extern "C" {
    pub fn container_of(_arg: wr, ib_reg_wr: struct, _arg: wr) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_recv_wr {
    pub next: *mut ib_recv_wr,
    pub wr_id: u64,
    pub wr_cqe: *mut ib_cqe,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_access_flags {
    IB_ACCESS_LOCAL_WRITE = IB_UVERBS_ACCESS_LOCAL_WRITE,
    IB_ACCESS_REMOTE_WRITE = IB_UVERBS_ACCESS_REMOTE_WRITE,
    IB_ACCESS_REMOTE_READ = IB_UVERBS_ACCESS_REMOTE_READ,
    IB_ACCESS_REMOTE_ATOMIC = IB_UVERBS_ACCESS_REMOTE_ATOMIC,
    IB_ACCESS_MW_BIND = IB_UVERBS_ACCESS_MW_BIND,
    IB_ZERO_BASED = IB_UVERBS_ACCESS_ZERO_BASED,
    IB_ACCESS_ON_DEMAND = IB_UVERBS_ACCESS_ON_DEMAND,
    IB_ACCESS_HUGETLB = IB_UVERBS_ACCESS_HUGETLB,
    IB_ACCESS_RELAXED_ORDERING = IB_UVERBS_ACCESS_RELAXED_ORDERING,
    IB_ACCESS_FLUSH_GLOBAL = IB_UVERBS_ACCESS_FLUSH_GLOBAL,
    IB_ACCESS_FLUSH_PERSISTENT = IB_UVERBS_ACCESS_FLUSH_PERSISTENT,

    IB_ACCESS_OPTIONAL = IB_UVERBS_ACCESS_OPTIONAL_RANGE,
    IB_ACCESS_SUPPORTED =
    ((IB_ACCESS_FLUSH_PERSISTENT << 1) - 1) | IB_ACCESS_OPTIONAL,
}

//
// XXX: these are apparently used for ->rereg_user_mr, no idea why they
// are hidden here instead of a uapi header!
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mr_rereg_flags {
    IB_MR_REREG_TRANS	= 1,
    IB_MR_REREG_PD		= (1<<1),
    IB_MR_REREG_ACCESS	= (1<<2),
    IB_MR_REREG_SUPPORTED	= ((IB_MR_REREG_ACCESS << 1) - 1)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_remove_reason {
//
// Userspace requested uobject deletion or initial try
// to remove uobject via cleanup. Call could fail
//
    RDMA_REMOVE_DESTROY,
// Context deletion. This call should delete the actual object itself
    RDMA_REMOVE_CLOSE,
// Driver is being hot-unplugged. This call should delete the actual object itself
    RDMA_REMOVE_DRIVER_REMOVE,
// uobj is being cleaned-up before being committed
    RDMA_REMOVE_ABORT,
// The driver failed to destroy the uobject and is being disconnected
    RDMA_REMOVE_DRIVER_FAILURE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rdmacg_object {

    pub /: *mut *mut *mut rdma_cgroup cg; / owner rdma cgroup,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ucontext {
    pub device: *mut ib_device,
    pub ufile: *mut ib_uverbs_file,
    pub cg_obj: ib_rdmacg_object,
    pub enabled_caps: u64,
//
// Implementation details of the RDMA core, don't use in drivers:
//
    pub res: rdma_restrack_entry,
    pub mmap_xa: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_uobject {
    pub /: *mut *mut u64 user_handle; / handle given to us by userspace,
// ufile & ucontext owning this object
    pub ufile: *mut ib_uverbs_file,
// FIXME, save memory: ufile->context == context
    pub /: *mut *mut *mut ib_ucontext context; / associated user context,
    pub /: *mut *mut *mut void object; / containing object,
    pub /: *mut *mut list_head list; / link to context's list,
    pub /: *mut *mut ib_rdmacg_object cg_obj; / rdmacg object,
    pub /: *mut *mut int id; / index into kernel idr,
    pub ref: kref,
    pub /: *mut *mut atomic_t usecnt; / protects exclusive access,
    pub /: *mut *mut rcu_head rcu; / kfree_rcu() overhead,
    pub uapi_object: *const uverbs_api_object,
}

//
// struct ib_udata - Driver request/response data from userspace
// @inbuf: Pointer to request data from userspace
// @outbuf: Pointer to response buffer in userspace
// @inlen: Length of request data
// @outlen: Length of response buffer
//
// struct ib_udata is used to hold the driver data request and response
// structures defined in the uapi. They follow these rules for forwards and
// backwards compatibility:
//
// 1) Userspace can provide a longer request so long as the trailing part the
// kernel doesn't understand is all zeros.
//
// This provides a degree of safety if userspace wrongly tries to use a new
// feature the kernel does not understand with some non-zero value.
//
// It allows a simpler rdma-core implementation because the library can
// simply always use the latest structs for the request, even if they are
// bigger. It simply has to avoid using the new members if they are not
// supported/required.
//
// 2) Userspace can provide a shorter request; the kernel will zero-pad it out
// to fill the storage. The newer kernel should understand that older
// userspace will provide 0 to new fields. The kernel has three options to
// enable new request fields:
//
// - Input comp_mask that says the field is supported
// - Look for non-zero values
// - Check if the udata->inlen size covers the field
//
// This also corrects any bugs related to not filling in request structures
// as the new helper always fully writes to the struct.
//
// 3) Userspace can provide a shorter or longer response struct. If shorter,
// the kernel reply is truncated. The kernel should be designed to not write
// to new reply fields unless userspace has affirmatively requested them.
//
// If the user buffer is longer, the kernel will zero-fill it.
//
// Userspace has three options to enable new response fields:
//
// - Output comp_mask that says the field is supported
// - Look for non-zero values
// - Infer the output must be valid because the request contents demand it
// and old kernels will fail the request
//
// The following helper functions implement these semantics:
//
// ib_copy_validate_udata_in() - Checks the minimum length, and zero trailing::
//
// struct driver_create_cq_req req;
// int err;
//
// err = ib_copy_validate_udata_in(udata, req, end_member);
// if (err)
// return err;
//
// The third argument specifies the last member of the struct in the first
// kernel version that introduced it, establishing the minimum required size.
//
// ib_copy_validate_udata_in_cm() - The above but also validate a
// comp_mask member only has supported bits set::
//
// err = ib_copy_validate_udata_in_cm(udata, req, first_version_last_member,
// DRIVER_CREATE_CQ_MASK_FEATURE_A |
// DRIVER_CREATE_CQ_MASK_FEATURE_B);
//
// ib_respond_udata() - Implements the response rules::
//
// struct driver_create_cq_resp resp = {};
//
// resp.some_field = value;
// return ib_respond_udata(udata, resp);
//
// ib_is_udata_in_empty() - Used instead of ib_copy_validate_udata_in() if the
// driver does not have a request structure::
//
// ret = ib_is_udata_in_empty(udata);
// if (ret)
// return ret;
//
// Similarly ib_respond_empty_udata() is used instead of ib_respond_udata() if
// the driver does not have a response structure::
//
// return ib_respond_empty_udata(udata);
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_udata {
    pub inbuf: *const void __user,
    pub outbuf: *mut void __user,
    pub inlen: usize,
    pub outlen: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_pd {
    pub local_dma_lkey: u32,
    pub flags: u32,
    pub device: *mut ib_device,
    pub uobject: *mut ib_uobject,
    pub /: *mut *mut atomic_t usecnt; / count all resources,
    pub unsafe_global_rkey: u32,
//
// Implementation details of the RDMA core, don't use in drivers:
//
    pub __internal_mr: *mut ib_mr,
    pub res: rdma_restrack_entry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_xrcd {
    pub device: *mut ib_device,
    pub /: *mut *mut atomic_t usecnt; / count all exposed resources,
    pub inode: *mut inode,
    pub tgt_qps_rwsem: rw_semaphore,
    pub tgt_qps: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ah {
    pub device: *mut ib_device,
    pub pd: *mut ib_pd,
    pub uobject: *mut ib_uobject,
    pub sgid_attr: *const ib_gid_attr,
    pub type: rdma_ah_attr_type,
}

extern "C" {
    pub fn void(cq: *mut *mut ib_comp_handler)(struct ib_cq, cq_context: *mut c_void) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_poll_context {
    IB_POLL_SOFTIRQ,	   /* poll from softirq context */
    IB_POLL_WORKQUEUE,	   /* poll from workqueue */
    IB_POLL_UNBOUND_WORKQUEUE, /* poll from unbound workqueue */
    IB_POLL_LAST_POOL_TYPE = IB_POLL_UNBOUND_WORKQUEUE,

    IB_POLL_DIRECT,		   /* caller context, no hw completions */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_cq {
    pub device: *mut ib_device,
    pub uobject: *mut ib_ucq_object,
    pub comp_handler: ib_comp_handler,
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub cq_context: *mut c_void,
    pub cqe: c_int,
    pub cqe_used: c_uint,
    pub /: *mut *mut atomic_t usecnt; / count number of work queues,
    pub poll_ctx: ib_poll_context,
    pub wc: *mut ib_wc,
    pub pool_entry: list_head,
    pub iop: irq_poll,
    pub work: work_struct,
}

// updated only by trace points
//
// Implementation details of the RDMA core, don't use in drivers:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_qp_attach_comp_cntr_op {
    IB_QP_ATTACH_COMP_CNTR_OP_SEND = IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_SEND,
    IB_QP_ATTACH_COMP_CNTR_OP_RECV = IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_RECV,
    IB_QP_ATTACH_COMP_CNTR_OP_RDMA_READ = IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_RDMA_READ,
    IB_QP_ATTACH_COMP_CNTR_OP_REMOTE_RDMA_READ = IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_REMOTE_RDMA_READ,
    IB_QP_ATTACH_COMP_CNTR_OP_RDMA_WRITE = IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_RDMA_WRITE,
    IB_QP_ATTACH_COMP_CNTR_OP_REMOTE_RDMA_WRITE = IB_UVERBS_QP_ATTACH_COMP_CNTR_OP_REMOTE_RDMA_WRITE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_comp_cntr_caps {
    pub max_value: u64,
    pub max_counters: u32,
    pub /: *mut *mut u32 supported_qp_attach_ops; / Bitmask of enum ib_qp_attach_comp_cntr_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_comp_cntr {
    pub device: *mut ib_device,
    pub uobject: *mut ib_uobject,
    pub usecnt: core::sync::atomic::AtomicI32,
    pub res: rdma_restrack_entry,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_comp_cntr_entry {
    IB_COMP_CNTR_ENTRY_COMP = IB_UVERBS_COMP_CNTR_ENTRY_COMP,
    IB_COMP_CNTR_ENTRY_ERR = IB_UVERBS_COMP_CNTR_ENTRY_ERR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_comp_cntr_modify_op {
    IB_COMP_CNTR_MODIFY_OP_SET = IB_UVERBS_COMP_CNTR_MODIFY_OP_SET,
    IB_COMP_CNTR_MODIFY_OP_INC = IB_UVERBS_COMP_CNTR_MODIFY_OP_INC,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_qp_attach_comp_cntr_attr {
    pub /: *mut *mut u32 op_mask; / Bitmask of enum ib_qp_attach_comp_cntr_op,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_srq {
    pub device: *mut ib_device,
    pub pd: *mut ib_pd,
    pub uobject: *mut ib_usrq_object,
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub srq_context: *mut c_void,
    pub srq_type: ib_srq_type,
    pub usecnt: core::sync::atomic::AtomicI32,
    pub cq: *mut ib_cq,
    pub xrcd: *mut ib_xrcd,
    pub srq_num: u32,
    pub xrc: },
}

//
// Implementation details of the RDMA core, don't use in drivers:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_raw_packet_caps {
//
// Strip cvlan from incoming packet and report it in the matching work
// completion is supported.
//
    IB_RAW_PACKET_CAP_CVLAN_STRIPPING =
    IB_UVERBS_RAW_PACKET_CAP_CVLAN_STRIPPING,
//
// Scatter FCS field of an incoming packet to host memory is supported.
//
    IB_RAW_PACKET_CAP_SCATTER_FCS = IB_UVERBS_RAW_PACKET_CAP_SCATTER_FCS,
// Checksum offloads are supported (for both send and receive).
    IB_RAW_PACKET_CAP_IP_CSUM = IB_UVERBS_RAW_PACKET_CAP_IP_CSUM,
//
// When a packet is received for an RQ with no receive WQEs, the
// packet processing is delayed.
//
    IB_RAW_PACKET_CAP_DELAY_DROP = IB_UVERBS_RAW_PACKET_CAP_DELAY_DROP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wq_type {
    IB_WQT_RQ = IB_UVERBS_WQT_RQ,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wq_state {
    IB_WQS_RESET,
    IB_WQS_RDY,
    IB_WQS_ERR
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_wq {
    pub device: *mut ib_device,
    pub uobject: *mut ib_uwq_object,
    pub wq_context: *mut c_void,
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub pd: *mut ib_pd,
    pub cq: *mut ib_cq,
    pub wq_num: u32,
    pub state: ib_wq_state,
    pub wq_type: ib_wq_type,
    pub usecnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wq_flags {
    IB_WQ_FLAGS_CVLAN_STRIPPING	= IB_UVERBS_WQ_FLAGS_CVLAN_STRIPPING,
    IB_WQ_FLAGS_SCATTER_FCS		= IB_UVERBS_WQ_FLAGS_SCATTER_FCS,
    IB_WQ_FLAGS_DELAY_DROP		= IB_UVERBS_WQ_FLAGS_DELAY_DROP,
    IB_WQ_FLAGS_PCI_WRITE_END_PADDING =
    IB_UVERBS_WQ_FLAGS_PCI_WRITE_END_PADDING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_wq_init_attr {
    pub wq_context: *mut c_void,
    pub wq_type: ib_wq_type,
    pub max_wr: u32,
    pub max_sge: u32,
    pub cq: *mut ib_cq,
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub /: *mut *mut u32 create_flags; / Use enum ib_wq_flags,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_wq_attr_mask {
    IB_WQ_STATE		= 1 << 0,
    IB_WQ_CUR_STATE		= 1 << 1,
    IB_WQ_FLAGS		= 1 << 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_wq_attr {
    pub wq_state: ib_wq_state,
    pub curr_wq_state: ib_wq_state,
    pub /: *mut *mut u32 flags; / Use enum ib_wq_flags,
    pub /: *mut *mut u32 flags_mask; / Use enum ib_wq_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rwq_ind_table {
    pub device: *mut ib_device,
    pub uobject: *mut ib_uobject,
    pub usecnt: core::sync::atomic::AtomicI32,
    pub ind_tbl_num: u32,
    pub log_ind_tbl_size: u32,
    pub ind_tbl: *mut ib_wq,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_rwq_ind_table_init_attr {
    pub log_ind_tbl_size: u32,
// Each entry is a pointer to Receive Work Queue
    pub ind_tbl: *mut ib_wq,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_pkey_state {
    IB_PORT_PKEY_NOT_VALID = 0,
    IB_PORT_PKEY_VALID = 1,
    IB_PORT_PKEY_LISTED = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_pkey {
    pub state: port_pkey_state,
    pub pkey_index: u16,
    pub port_num: u32,
    pub qp_list: list_head,
    pub to_error_list: list_head,
    pub sec: *mut ib_qp_security,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ports_pkeys {
    pub main: ib_port_pkey,
    pub alt: ib_port_pkey,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_qp_security {
    pub qp: *mut ib_qp,
    pub dev: *mut ib_device,
// Hold this mutex when changing port and pkey settings.
    pub mutex: mutex,
    pub ports_pkeys: *mut ib_ports_pkeys,
// A list of all open shared QP handles.  Required to enforce security
// properly for all users of a shared QP.
//
    pub shared_qp_list: list_head,
    pub security: *mut c_void,
    pub destroying: bool,
    pub error_list_count: core::sync::atomic::AtomicI32,
    pub error_complete: completion,
    pub error_comps_pending: c_int,
}

//
// @max_write_sge: Maximum SGE elements per RDMA WRITE request.
// @max_read_sge:  Maximum SGE elements per RDMA READ request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_qp {
    pub device: *mut ib_device,
    pub pd: *mut ib_pd,
    pub send_cq: *mut ib_cq,
    pub recv_cq: *mut ib_cq,
    pub mr_lock: spinlock_t,
    pub mrs_used: c_int,
    pub rdma_mrs: list_head,
    pub sig_mrs: list_head,
    pub srq: *mut ib_srq,
    pub srq_completion: completion,
    pub /: *mut *mut *mut ib_xrcd xrcd; / XRC TGT QPs only,
    pub xrcd_list: list_head,
    pub /: *mut *mut xarray comp_cntrs; / op_mask -> comp_cntr,
    pub comp_cntr_op_mask: u32,
// count times opened, mcast attaches, flow attaches
    pub usecnt: core::sync::atomic::AtomicI32,
    pub open_list: list_head,
    pub real_qp: *mut ib_qp,
    pub uobject: *mut ib_uqp_object,
    pub ): *mut *mut *mut void (event_handler)(struct ib_event , void,
    pub ): *mut *mut *mut void (registered_event_handler)(struct ib_event , void,
    pub qp_context: *mut c_void,
// sgid_attrs associated with the AV's
    pub av_sgid_attr: *const ib_gid_attr,
    pub alt_path_sgid_attr: *const ib_gid_attr,
    pub qp_num: u32,
    pub max_write_sge: u32,
    pub max_read_sge: u32,
    pub qp_type: ib_qp_type,
    pub rwq_ind_tbl: *mut ib_rwq_ind_table,
    pub qp_sec: *mut ib_qp_security,
    pub port: u32,
    pub integrity_en: bool,
//
// Implementation details of the RDMA core, don't use in drivers:
//
    pub res: rdma_restrack_entry,
// The counter the qp is bind to
    pub counter: *mut rdma_counter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dm {
    pub device: *mut ib_device,
    pub length: u32,
    pub flags: u32,
    pub uobject: *mut ib_uobject,
    pub usecnt: core::sync::atomic::AtomicI32,
}

// bit values to mark existence of ib_dmah fields
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_dmah {
    pub device: *mut ib_device,
    pub uobject: *mut ib_uobject,
//
// Implementation details of the RDMA core, don't use in drivers:
//
    pub res: rdma_restrack_entry,
    pub cpu_id: u32,
    pub mem_type: tph_mem_type,
    pub usecnt: core::sync::atomic::AtomicI32,
    pub ph: u8,
    pub /: *mut *mut u8 valid_fields; / use IB_DMAH_XXX_EXISTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mr {
    pub device: *mut ib_device,
//
// Due to IB_MR_REREG_PD pd is not a fixed pointer and can change. For a
// user MR, this value should only be read from a system call that holds
// the uobject lock, or the driver should disable in-place REREG_PD.
//
    pub pd: *mut ib_pd,
    pub lkey: u32,
    pub rkey: u32,
    pub iova: u64,
    pub length: u64,
    pub page_size: c_uint,
    pub type: ib_mr_type,
    pub need_inval: bool,
    pub /: *mut *mut *mut ib_uobject uobject; / user,
    pub /: *mut *mut list_head qp_entry; / FR,
}

//
// Implementation details of the RDMA core, don't use in drivers:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mw {
    pub device: *mut ib_device,
    pub pd: *mut ib_pd,
    pub uobject: *mut ib_uobject,
    pub rkey: u32,
    pub type: ib_mw_type,
}

// Supported steering options
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_flow_attr_type {
// steering according to rule specifications
    IB_FLOW_ATTR_NORMAL		= 0x0,
// default unicast and multicast rule -
// receive all Eth traffic which isn't steered to any QP
//
    IB_FLOW_ATTR_ALL_DEFAULT	= 0x1,
// default multicast rule -
// receive all Eth multicast traffic which isn't steered to any QP
//
    IB_FLOW_ATTR_MC_DEFAULT		= 0x2,
// sniffer rule - receive all port traffic
    IB_FLOW_ATTR_SNIFFER		= 0x3
}

// Supported steering header types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_flow_spec_type {
// L2 headers
    IB_FLOW_SPEC_ETH		= 0x20,
    IB_FLOW_SPEC_IB			= 0x22,
// L3 header
    IB_FLOW_SPEC_IPV4		= 0x30,
    IB_FLOW_SPEC_IPV6		= 0x31,
    IB_FLOW_SPEC_ESP                = 0x34,
// L4 headers
    IB_FLOW_SPEC_TCP		= 0x40,
    IB_FLOW_SPEC_UDP		= 0x41,
    IB_FLOW_SPEC_VXLAN_TUNNEL	= 0x50,
    IB_FLOW_SPEC_GRE		= 0x51,
    IB_FLOW_SPEC_MPLS		= 0x60,
    IB_FLOW_SPEC_INNER		= 0x100,
// Actions
    IB_FLOW_SPEC_ACTION_TAG         = 0x1000,
    IB_FLOW_SPEC_ACTION_DROP        = 0x1001,
    IB_FLOW_SPEC_ACTION_HANDLE	= 0x1002,
    IB_FLOW_SPEC_ACTION_COUNT       = 0x1003,
}

pub const IB_FLOW_SPEC_LAYER_MASK: c_uint = 0xF0;
pub const IB_FLOW_SPEC_SUPPORT_LAYERS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_flow_flags {
    IB_FLOW_ATTR_FLAGS_DONT_TRAP = 1UL << 1, /* Continue match, no steal */
    IB_FLOW_ATTR_FLAGS_EGRESS = 1UL << 2, /* Egress flow */
    IB_FLOW_ATTR_FLAGS_RESERVED  = 1UL << 3  /* Must be last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_eth_filter {
    pub dst_mac: [u8; 6],
    pub src_mac: [u8; 6],
    pub ether_type: __be16,
    pub vlan_tag: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_eth {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_eth_filter,
    pub mask: ib_flow_eth_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_ib_filter {
    pub dlid: __be16,
    pub sl: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_ib {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_ib_filter,
    pub mask: ib_flow_ib_filter,
}

// IPv4 header flags
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_ipv4_flags {
    IB_IPV4_DONT_FRAG = 0x2, /* Don't enable packet fragmentation */
    IB_IPV4_MORE_FRAG = 0X4  /* For All fragmented packets except the
    last have this flag set */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_ipv4_filter {
    pub src_ip: __be32,
    pub dst_ip: __be32,
    pub proto: u8,
    pub tos: u8,
    pub ttl: u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_ipv4 {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_ipv4_filter,
    pub mask: ib_flow_ipv4_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_ipv6_filter {
    pub src_ip: [u8; 16],
    pub dst_ip: [u8; 16],
    pub flow_label: __be32,
    pub next_hdr: u8,
    pub traffic_class: u8,
    pub hop_limit: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_ipv6 {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_ipv6_filter,
    pub mask: ib_flow_ipv6_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_tcp_udp_filter {
    pub dst_port: __be16,
    pub src_port: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_tcp_udp {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_tcp_udp_filter,
    pub mask: ib_flow_tcp_udp_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_tunnel_filter {
    pub tunnel_id: __be32,
}

// ib_flow_spec_tunnel describes the Vxlan tunnel
// the tunnel_id from val has the vni value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_tunnel {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_tunnel_filter,
    pub mask: ib_flow_tunnel_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_esp_filter {
    pub spi: __be32,
    pub seq: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_esp {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_esp_filter,
    pub mask: ib_flow_esp_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_gre_filter {
    pub c_ks_res0_ver: __be16,
    pub protocol: __be16,
    pub key: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_gre {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_gre_filter,
    pub mask: ib_flow_gre_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_mpls_filter {
    pub tag: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_mpls {
    pub type: u32,
    pub size: u16,
    pub val: ib_flow_mpls_filter,
    pub mask: ib_flow_mpls_filter,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_action_tag {
    pub type: ib_flow_spec_type,
    pub size: u16,
    pub tag_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_action_drop {
    pub type: ib_flow_spec_type,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_action_handle {
    pub type: ib_flow_spec_type,
    pub size: u16,
    pub act: *mut ib_flow_action,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_counters_description {
    IB_COUNTER_PACKETS,
    IB_COUNTER_BYTES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_action_count {
    pub type: ib_flow_spec_type,
    pub size: u16,
    pub counters: *mut ib_counters,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ib_flow_spec {
    pub type: u32,
    pub size: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_attr {
    pub type: ib_flow_attr_type,
    pub size: u16,
    pub priority: u16,
    pub flags: u32,
    pub num_of_specs: u8,
    pub port: u32,
    pub flows: [ib_flow_spec; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow {
    pub qp: *mut ib_qp,
    pub device: *mut ib_device,
    pub uobject: *mut ib_uobject,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_flow_action_type {
    IB_FLOW_ACTION_UNSPECIFIED,
    IB_FLOW_ACTION_ESP = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_action_attrs_esp_keymats {
    pub protocol: ib_uverbs_flow_action_esp_keymat,
    pub aes_gcm: ib_uverbs_flow_action_esp_keymat_aes_gcm,
    pub keymat: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_action_attrs_esp_replays {
    pub protocol: ib_uverbs_flow_action_esp_replay,
    pub bmp: ib_uverbs_flow_action_esp_replay_bmp,
    pub replay: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_flow_action_attrs_esp_flags {
// All user-space flags at the top: Use enum ib_uverbs_flow_action_esp_flags
// This is done in order to share the same flags between user-space and
// kernel and spare an unnecessary translation.
//

// Kernel flags
    IB_FLOW_ACTION_ESP_FLAGS_ESN_TRIGGERED	= 1ULL << 32,
    IB_FLOW_ACTION_ESP_FLAGS_MOD_ESP_ATTRS	= 1ULL << 33,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_spec_list {
    pub next: *mut ib_flow_spec_list,
    pub spec: ib_flow_spec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_action_attrs_esp {
    pub keymat: *mut ib_flow_action_attrs_esp_keymats,
    pub replay: *mut ib_flow_action_attrs_esp_replays,
    pub encap: *mut ib_flow_spec_list,
// Used only if IB_FLOW_ACTION_ESP_FLAGS_ESN_TRIGGERED is enabled.
// Value of 0 is a valid value.
//
    pub esn: u32,
    pub spi: u32,
    pub seq: u32,
    pub tfc_pad: u32,
// Use enum ib_flow_action_attrs_esp_flags
    pub flags: u64,
    pub hard_limit_pkts: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_flow_action {
    pub device: *mut ib_device,
    pub uobject: *mut ib_uobject,
    pub type: ib_flow_action_type,
    pub usecnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_process_mad_flags {
    IB_MAD_IGNORE_MKEY	= 1,
    IB_MAD_IGNORE_BKEY	= 2,
    IB_MAD_IGNORE_ALL	= IB_MAD_IGNORE_MKEY | IB_MAD_IGNORE_BKEY
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_mad_result {
    IB_MAD_RESULT_FAILURE  = 0,      /* (!SUCCESS is the important flag) */
    IB_MAD_RESULT_SUCCESS  = 1 << 0, /* MAD was successfully processed   */
    IB_MAD_RESULT_REPLY    = 1 << 1, /* Reply packet needs to be sent    */
    IB_MAD_RESULT_CONSUMED = 1 << 2  /* Packet consumed: stop processing */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_cache {
    pub subnet_prefix: u64,
    pub pkey: *mut ib_pkey_cache,
    pub gid: *mut ib_gid_table,
    pub lmc: u8,
    pub port_state: ib_port_state,
    pub last_port_state: ib_port_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_immutable {
    pub pkey_tbl_len: c_int,
    pub gid_tbl_len: c_int,
    pub core_cap_flags: u32,
    pub max_mad_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_port_data {
    pub ib_dev: *mut ib_device,
    pub immutable: ib_port_immutable,
    pub pkey_list_lock: spinlock_t,
    pub netdev_lock: spinlock_t,
    pub pkey_list: list_head,
    pub cache: ib_port_cache,
    pub netdev: *mut net_device __rcu,
    pub netdev_tracker: netdevice_tracker,
    pub ndev_hash_link: hlist_node,
    pub port_counter: rdma_port_counter,
    pub sysfs: *mut ib_port,
}

// rdma netdev type - specifies protocol type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_netdev_t {
    RDMA_NETDEV_IPOIB,
}

//
// struct rdma_netdev - rdma netdev
// For cases where netstack interfacing is required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_netdev {
    pub clnt_priv: *mut c_void,
    pub hca: *mut ib_device,
    pub port_num: u32,
    pub mtu: c_int,
    pub netdev): *mut *mut void (free_rdma_netdev)(struct net_device,
// control functions
    pub id): *mut *mut *mut void (set_id)(struct net_device netdev, int,
// send packet
    pub dqpn): *mut *mut ib_ah address, u32,
// multicast
    pub qkey): int set_qkey, u32,
    pub mlid): *mut *mut ib_gid gid, u16,
// timeout
    pub txqueue): *mut *mut *mut void (tx_timeout)(struct net_device dev, unsigned int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_netdev_alloc_params {
    pub sizeof_priv: usize,
    pub txqs: c_uint,
    pub rxqs: c_uint,
    pub param: *mut c_void,
    pub param): *mut *mut net_device netdev, void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_odp_counters {
    pub faults: core::sync::atomic::AtomicI64,
    pub faults_handled: core::sync::atomic::AtomicI64,
    pub invalidations: core::sync::atomic::AtomicI64,
    pub invalidations_handled: core::sync::atomic::AtomicI64,
    pub prefetch: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_counters {
    pub device: *mut ib_device,
    pub uobject: *mut ib_uobject,
// num of objects attached
    pub usecnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_counters_read_attr {
    pub counters_buff: *mut u64,
    pub ncounters: u32,
    pub /: *mut *mut u32 flags; / use enum ib_read_counters_flags,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdma_user_mmap_entry {
    pub ref: kref,
    pub ucontext: *mut ib_ucontext,
    pub start_pgoff: c_ulong,
    pub npages: usize,
    pub driver_removed: bool,
// protects access to dmabufs
    pub dmabufs_lock: mutex,
    pub dmabufs: list_head,
}

// Return the offset (in bytes) the user should pass to libc's mmap()
//
// struct ib_device_ops - InfiniBand device operations
// This structure defines all the InfiniBand device operations, providers will
// need to define the supported operations, otherwise they will be set to null.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_device_ops {
    pub owner: *mut module,
    pub driver_id: rdma_driver_id,
    pub uverbs_abi_ver: u32,
    pub uverbs_no_driver_id_binding:1: c_uint,
//
// Indicates the driver checks every op accepting a udata for the
// correct size on input and always handles the output using the udata
// helpers.
//
    pub uverbs_robust_udata:1: c_uint,
//
// NOTE: New drivers should not make use of device_group; instead new
// device parameter should be exposed via netlink command. This
// mechanism exists only for existing drivers.
//
    pub device_group: *const attribute_group,
    pub port_groups: *const attribute_group,
    pub bad_send_wr): *const ib_send_wr,
    pub bad_recv_wr): *const ib_recv_wr,
    pub qp): *mut *mut void (drain_rq)(struct ib_qp,
    pub qp): *mut *mut void (drain_sq)(struct ib_qp,
    pub wc): *mut *mut *mut int (poll_cq)(struct ib_cq cq, int num_entries, struct ib_wc,
    pub wc_cnt): *mut *mut *mut int (peek_cq)(struct ib_cq cq, int,
    pub flags): *mut *mut *mut int (req_notify_cq)(struct ib_cq cq, enum ib_cq_notify_flags,
    pub bad_recv_wr): *const ib_recv_wr,
    pub out_mad_pkey_index): *mut *mut size_t out_mad_size, u16,
    pub udata): *mut ib_udata,
    pub device_modify): *mut ib_device_modify,
    pub str): *mut *mut *mut void (get_dev_fw_str)(struct ib_device device, char,
    pub port_attr): *mut ib_port_attr,
    pub speed): *mut u64,
    pub port_modify): *mut ib_port_modify,
//
// The following mandatory functions are used only at device
// registration.  Keep functions such as these at the end of this
// structure to avoid cache line misses when accessing struct ib_device
// in fast paths.
//
    pub immutable): *mut ib_port_immutable,
    pub port_num): u32,
//
// When calling get_netdev, the HW vendor's driver should return the
// net device of device @device at port @port_num or NULL if such
// a net device doesn't exist. The vendor driver should call dev_hold
// on this net device. The HW vendor's device driver must guarantee
// that this function returns NULL before the net device has finished
// NETDEV_UNREGISTER state.
//
    pub port_num): u32,
//
// rdma netdev operation
//
// Driver implementing alloc_rdma_netdev or rdma_netdev_get_params
// must return -EOPNOTSUPP if it doesn't support the specified type.
//
    pub )): *mut *mut void (setup)(struct net_device,
    pub params): *mut rdma_netdev_alloc_params,
//
// query_gid should be return GID value for @device, when @port_num
// link layer is either IB or iWarp. It is no-op if @port_num port
// is RoCE link layer.
//
    pub gid): *mut ib_gid,
//
// When calling add_gid, the HW vendor's driver should add the gid
// of device of port at gid index available at @attr. Meta-info of
// that gid (for example, the network device related to this gid) is
// available at @attr. @context allows the HW vendor driver to store
// extra information together with a GID entry. The HW vendor driver may
// allocate memory to contain this information and store it in @context
// when a new GID entry is written to. Params are consistent until the
// next call of add_gid or delete_gid. The function should return 0 on
// success or error otherwise. The function could be called
// concurrently for different ports. This function is only called when
// roce_gid_table is used.
//
    pub context): *const *const *const int (add_gid)(struct ib_gid_attr attr, void,
//
// When calling del_gid, the HW vendor's driver should delete the
// gid of device @device at gid index gid_index of port port_num
// available in @attr.
// Upon the deletion of a GID entry, the HW vendor must free any
// allocated memory. The caller will clear @context afterwards.
// This function is only called when roce_gid_table is used.
//
    pub context): *const *const *const int (del_gid)(struct ib_gid_attr attr, void,
    pub pkey): *mut u16,
    pub udata): *mut ib_udata,
    pub context): *mut *mut void (dealloc_ucontext)(struct ib_ucontext,
    pub vma): *mut *mut *mut int (mmap)(struct ib_ucontext context, struct vm_area_struct,
//
// This will be called once refcount of an entry in mmap_xa reaches
// zero. The type of the memory that was mapped may differ between
// entries and is opaque to the rdma_user_mmap interface.
// Therefore needs to be implemented by the driver in mmap_free.
//
    pub entry): *mut *mut void (mmap_free)(struct rdma_user_mmap_entry,
    pub provider): *mut p2pdma_provider,
    pub pg_off): off_t,
    pub ibcontext): *mut *mut void (disassociate_ucontext)(struct ib_ucontext,
    pub udata): *mut *mut *mut int (alloc_pd)(struct ib_pd pd, struct ib_udata,
    pub udata): *mut *mut *mut int (dealloc_pd)(struct ib_pd pd, struct ib_udata,
    pub udata): *mut ib_udata,
    pub udata): *mut ib_udata,
    pub ah_attr): *mut *mut *mut int (modify_ah)(struct ib_ah ah, struct rdma_ah_attr,
    pub ah_attr): *mut *mut *mut int (query_ah)(struct ib_ah ah, struct rdma_ah_attr,
    pub flags): *mut *mut *mut int (destroy_ah)(struct ib_ah ah, u32,
    pub udata): *mut ib_udata,
    pub udata): *mut ib_udata,
    pub srq_attr): *mut *mut *mut int (query_srq)(struct ib_srq srq, struct ib_srq_attr,
    pub udata): *mut *mut *mut int (destroy_srq)(struct ib_srq srq, struct ib_udata,
    pub udata): *mut ib_udata,
    pub udata): *mut int qp_attr_mask, struct ib_udata,
    pub attr): *mut ib_qp_attach_comp_cntr_attr,
    pub qp_init_attr): *mut int qp_attr_mask, struct ib_qp_init_attr,
    pub udata): *mut *mut *mut int (destroy_qp)(struct ib_qp qp, struct ib_udata,
    pub attrs): *mut uverbs_attr_bundle,
    pub attrs): *mut uverbs_attr_bundle,
    pub cq_period): *mut *mut *mut int (modify_cq)(struct ib_cq cq, u16 cq_count, u16,
    pub udata): *mut *mut *mut int (destroy_cq)(struct ib_cq cq, struct ib_udata,
    pub udata): *mut ib_udata,
//
// pre_destroy_cq - Prevent a cq from generating any new work
// completions, but not free any kernel resources
//
    pub cq): *mut *mut int (pre_destroy_cq)(struct ib_cq,
//
// post_destroy_cq - Free all kernel resources
//
    pub cq): *mut *mut void (post_destroy_cq)(struct ib_cq,
    pub attrs): *mut uverbs_attr_bundle,
    pub cc): *mut *mut int (destroy_comp_cntr)(struct ib_comp_cntr,
    pub value): ib_comp_cntr_modify_op op, u64,
    pub value): *mut *mut *mut int (read_comp_cntr)(struct ib_comp_cntr cc, enum ib_comp_cntr_entry entry, u64,
    pub attrs): *mut uverbs_attr_bundle,
    pub mr_access_flags): *mut *mut *mut *mut ib_mr (get_dma_mr)(ib_pd pd, int,
    pub udata): *mut ib_udata,
    pub attrs): *mut uverbs_attr_bundle,
    pub udata): *mut ib_udata,
    pub udata): *mut *mut *mut int (dereg_mr)(struct ib_mr mr, struct ib_udata,
    pub max_num_sg): u32,
    pub max_num_meta_sg): u32,
    pub attrs): *mut uverbs_attr_bundle,
//
// Kernel users should universally support relaxed ordering (RO), as
// they are designed to read data only after observing the CQE and use
// the DMA API correctly.
//
// Some drivers implicitly enable RO if platform supports it.
//
    pub sg_offset): *mut c_uint,
    pub mr_status): *mut ib_mr_status,
    pub udata): *mut *mut *mut int (alloc_mw)(struct ib_mw mw, struct ib_udata,
    pub mw): *mut *mut int (dealloc_mw)(struct ib_mw,
    pub lid): *mut *mut *mut *mut int (attach_mcast)(struct ib_qp qp, union ib_gid gid, u16,
    pub lid): *mut *mut *mut *mut int (detach_mcast)(struct ib_qp qp, union ib_gid gid, u16,
    pub udata): *mut *mut *mut int (alloc_xrcd)(struct ib_xrcd xrcd, struct ib_udata,
    pub udata): *mut *mut *mut int (dealloc_xrcd)(struct ib_xrcd xrcd, struct ib_udata,
    pub udata): *mut ib_udata,
    pub flow_id): *mut *mut int (destroy_flow)(struct ib_flow,
    pub action): *mut *mut int (destroy_flow_action)(struct ib_flow_action,
    pub state): c_int,
    pub ivf): *mut ifla_vf_info,
    pub stats): *mut ifla_vf_stats,
    pub port_guid): *mut ifla_vf_guid,
    pub type): c_int,
    pub udata): *mut ib_udata,
    pub udata): *mut *mut *mut int (destroy_wq)(struct ib_wq wq, struct ib_udata,
    pub udata): *mut u32 wq_attr_mask, struct ib_udata,
    pub udata): *mut ib_udata,
    pub wq_ind_table): *mut *mut int (destroy_rwq_ind_table)(struct ib_rwq_ind_table,
    pub attrs): *mut uverbs_attr_bundle,
    pub attrs): *mut *mut *mut int (dealloc_dm)(struct ib_dm dm, struct uverbs_attr_bundle,
    pub attrs): *mut uverbs_attr_bundle,
    pub attrs): *mut *mut *mut int (dealloc_dmah)(struct ib_dmah dmah, struct uverbs_attr_bundle,
    pub attrs): *mut uverbs_attr_bundle,
    pub attrs): *mut uverbs_attr_bundle,
    pub counters): *mut *mut int (destroy_counters)(struct ib_counters,
    pub attrs): *mut uverbs_attr_bundle,
    pub meta_sg_offset): *mut c_uint,
//
// alloc_hw_[device,port]_stats - Allocate a struct rdma_hw_stats and
// fill in the driver initialized data.  The struct is kfree()'ed by
// the sysfs core when the device is removed.  A lifespan of -1 in the
// return struct tells the core to set a default lifespan.
//
    pub device): *mut *mut *mut rdma_hw_stats (alloc_hw_device_stats)(ib_device,
    pub port_num): u32,
//
// get_hw_stats - Fill in the counter value(s) in the stats struct.
// @index - The index in the value array we wish to have updated, or
// num_counters if we want all stats updated
// Return codes -
// < 0 - Error, no counters updated
// index - Updated the single counter pointed to by index
// num_counters - Updated all counters (will reset the timestamp
// and prevent further calls for lifespan milliseconds)
// Drivers are allowed to update all counters in leiu of just the
// one given in index at their option
//
    pub index): *mut *mut rdma_hw_stats stats, u32 port, int,
//
// modify_hw_stat - Modify the counter configuration
// @enable: true/false when enable/disable a counter
// Return codes - 0 on success or error code otherwise.
//
    pub enable): unsigned int counter_index, bool,
//
// Allows rdma drivers to add their own restrack attributes.
//
    pub ibmr): *mut *mut *mut int (fill_res_mr_entry)(struct sk_buff msg, struct ib_mr,
    pub ibmr): *mut *mut *mut int (fill_res_mr_entry_raw)(struct sk_buff msg, struct ib_mr,
    pub ibcq): *mut *mut *mut int (fill_res_cq_entry)(struct sk_buff msg, struct ib_cq,
    pub ibcq): *mut *mut *mut int (fill_res_cq_entry_raw)(struct sk_buff msg, struct ib_cq,
    pub ibqp): *mut *mut *mut int (fill_res_qp_entry)(struct sk_buff msg, struct ib_qp,
    pub ibqp): *mut *mut *mut int (fill_res_qp_entry_raw)(struct sk_buff msg, struct ib_qp,
    pub id): *mut *mut *mut int (fill_res_cm_id_entry)(struct sk_buff msg, struct rdma_cm_id,
    pub ib_srq): *mut *mut *mut int (fill_res_srq_entry)(struct sk_buff msg, struct ib_srq,
    pub ib_srq): *mut *mut *mut int (fill_res_srq_entry_raw)(struct sk_buff msg, struct ib_srq,
// Device lifecycle callbacks
//
// Called after the device becomes registered, before clients are
// attached
//
    pub dev): *mut *mut int (enable_driver)(struct ib_device,
//
// This is called as part of ib_dealloc_device().
//
    pub dev): *mut *mut void (dealloc_driver)(struct ib_device,
// iWarp CM callbacks
    pub qp): *mut *mut void (iw_add_ref)(struct ib_qp,
    pub qp): *mut *mut void (iw_rem_ref)(struct ib_qp,
    pub qpn): *mut *mut *mut *mut ib_qp (iw_get_qp)(ib_device device, int,
    pub conn_param): *mut iw_cm_conn_param,
    pub conn_param): *mut iw_cm_conn_param,
    pub pdata_len): u8,
    pub backlog): *mut *mut *mut int (iw_create_listen)(struct iw_cm_id cm_id, int,
    pub cm_id): *mut *mut int (iw_destroy_listen)(struct iw_cm_id,
//
// counter_bind_qp - Bind a QP to a counter.
// @counter - The counter to be bound. If counter->id is zero then
// the driver needs to allocate a new counter and set counter->id
//
    pub port): u32,
//
// counter_unbind_qp - Unbind the qp from the dynamically-allocated
// counter and bind it onto the default one
//
    pub port): *mut *mut *mut int (counter_unbind_qp)(struct ib_qp qp, u32,
//
// counter_dealloc -De-allocate the hw counter
//
    pub counter): *mut *mut int (counter_dealloc)(struct rdma_counter,
//
// counter_alloc_stats - Allocate a struct rdma_hw_stats and fill in
// the driver initialized data.
//
    pub counter): *mut rdma_counter,
//
// counter_update_stats - Query the stats value of this counter
//
    pub counter): *mut *mut int (counter_update_stats)(struct rdma_counter,
//
// counter_init - Initialize the driver specific rdma counter struct.
//
    pub counter): *mut *mut void (counter_init)(struct rdma_counter,
//
// Allows rdma drivers to add their own restrack attributes
// dumped via 'rdma stat' iproute2 command.
//
    pub ibmr): *mut *mut *mut int (fill_stat_mr_entry)(struct sk_buff msg, struct ib_mr,
// query driver for its ucontext properties
    pub attrs): *mut uverbs_attr_bundle,
//
// Provide NUMA node. This API exists for rdmavt/hfi1 only.
// Everyone else relies on Linux memory management model.
//
    pub dev): *mut *mut int (get_numa_node)(struct ib_device,
//
// add_sub_dev - Add a sub IB device
//
    pub name): *const c_char,
//
// del_sub_dev - Delete a sub IB device
//
    pub sub_dev): *mut *mut void (del_sub_dev)(struct ib_device,
//
// ufile_cleanup - Attempt to cleanup ubojects HW resources inside
// the ufile.
//
    pub ufile): *mut *mut void (ufile_hw_cleanup)(struct ib_uverbs_file,
//
// report_port_event - Drivers need to implement this if they have
// some private stuff to handle when link status changes.
//
    pub event): *mut *mut net_device ndev, unsigned long,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_core_device {
// device must be the first element in structure until,
// union of ib_core_device and device exists in ib_device.
//
    pub dev: device,
    pub rdma_net: possible_net_t,
    pub ports_kobj: *mut kobject,
    pub port_list: list_head,
    pub /: *mut *mut *mut ib_device owner; / reach back to owner ib_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_device {
// Do not access @dma_device directly from ULP nor from HW drivers.
    pub dma_device: *mut device,
    pub ops: ib_device_ops,
    pub name: [c_char; IB_DEVICE_NAME_MAX],
    pub rcu_head: rcu_head,
    pub event_handler_list: list_head,
// Protects event_handler_list
    pub event_handler_rwsem: rw_semaphore,
// Protects QP's event_handler calls and open_qp list
    pub qp_open_list_lock: spinlock_t,
    pub client_data_rwsem: rw_semaphore,
    pub client_data: xarray,
    pub unregistration_lock: mutex,
// Synchronize GID, Pkey cache entries, subnet prefix, LMC
    pub cache_lock: rwlock_t,
//
// port_data is indexed by port number
//
    pub port_data: *mut ib_port_data,
    pub num_comp_vectors: c_int,
    pub dev: device,
    pub coredev: ib_core_device,
}

// First group is for device attributes,
// Second group is for driver provided attributes (optional).
// Third group is for the hw_stats
// It is a NULL terminated array.
//
// Indicates kernel verbs support, should not be used in drivers
// CQ adaptive moderation (RDMA DIM)
// CoCo guest with DMA bounce buffering required

//
// Positive refcount indicates that the device is currently
// registered and cannot be unregistered.
//
// Protects compat_devs xarray modifications
// Maintains compat devices for each net namespace
// Used by iWarp CM
// A parent device has a list of sub-devices
// A sub device has a type and a parent
extern "C" {
    pub fn kzalloc_node(_arg: size, _arg: gfp, _arg: dev->ops.get_numa_node(dev)) -> return;
}
extern "C" {
    pub fn kzalloc(_arg: size, _arg: gfp) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_client {
    pub name: *const c_char,
    pub ibdev): *mut *mut int (add)(struct ib_device,
    pub client_data): *mut *mut *mut void (remove)(struct ib_device , void,
    pub client_data): *mut *mut *mut void (rename)(struct ib_device dev, void,
    pub res): *mut ib_client_nl_info,
    pub res): *mut *mut int (get_global_nl_info)(struct ib_client_nl_info,
// Returns the net_dev belonging to this ib_client and matching the
// given parameters.
// @dev:	 An RDMA device that the net_dev use for communication.
// @port:	 A physical port number on the RDMA device.
// @pkey:	 P_Key that the net_dev uses if applicable.
// @gid:	 A GID that the net_dev uses to communicate.
// @addr:	 An IP address the net_dev is configured with.
// @client_data: The device's client data set by ib_set_client_data().
//
// An ib_client that implements a net_dev on top of RDMA devices
// (such as IP over IB) should implement this callback, allowing the
// rdma_cm module to find the right net_dev for a given request.
//
// The caller is responsible for calling dev_put on the returned
// netdev.
    pub client_data): *mut c_void,
    pub uses: refcount_t,
    pub uses_zero: completion,
    pub client_id: u32,
// kverbs are not required by the client
    pub no_kverbs_req:1: u8,
}

extern "C" {
    pub fn ib_dealloc_device(device: *mut ib_device);
}
extern "C" {
    pub fn ib_get_device_fw_str(device: *mut ib_device, str: *mut c_char);
}
extern "C" {
    pub fn ib_unregister_device(device: *mut ib_device);
}
extern "C" {
    pub fn ib_unregister_driver(driver_id: rdma_driver_id);
}
extern "C" {
    pub fn ib_unregister_device_and_put(device: *mut ib_device);
}
extern "C" {
    pub fn ib_unregister_device_queued(ib_dev: *mut ib_device);
}
extern "C" {
    pub fn ib_register_client(client: *mut ib_client) -> c_int;
}
extern "C" {
    pub fn ib_unregister_client(client: *mut ib_client);
}
//
// ib_get_client_data - Get IB client context
// @device:Device to get context for
// @client:Client to get context for
//
// ib_get_client_data() returns the client context data set with
// ib_set_client_data(). This can only be called while the client is
// registered to the device, once the ib_client remove() callback returns this
// cannot be called.
//
extern "C" {
    pub fn xa_load(_arg: &device->client_data, _arg: client->client_id) -> return;
}

extern "C" {
    pub fn rdma_user_mmap_disassociate(device: *mut ib_device);
}
extern "C" {
    pub fn rdma_user_mmap_entry_put(entry: *mut rdma_user_mmap_entry);
}
extern "C" {
    pub fn rdma_user_mmap_entry_remove(entry: *mut rdma_user_mmap_entry);
}

extern "C" {
    pub fn ib_is_buffer_cleared(offset: udata->inbuf +, _arg: len) -> return;
}
//
// ib_modify_qp_is_ok - Check that the supplied attribute mask
// contains all required attributes and no attributes not allowed for
// the given QP state transition.
// @cur_state: Current QP state
// @next_state: Next QP state
// @type: QP type
// @mask: Mask of supplied QP attributes
//
// This function is a helper function that a low-level driver's
// modify_qp method can use to validate the consumer's input.  It
// checks that cur_state and next_state are valid QP states, that a
// transition from cur_state to next_state is allowed by the IB spec,
// and that the attribute mask supplied is allowed for the transition.
//
extern "C" {
    pub fn ib_register_event_handler(event_handler: *mut ib_event_handler);
}
extern "C" {
    pub fn ib_unregister_event_handler(event_handler: *mut ib_event_handler);
}
extern "C" {
    pub fn ib_dispatch_event(event: *const ib_event);
}
//
// rdma_cap_ib_switch - Check if the device is IB switch
// @device: Device to check
//
// Device driver is responsible for setting is_switch bit on
// in ib_device structure at init time.
//
// Return: true if the device is IB switch.
//
// rdma_start_port - Return the first valid port number for the device
// specified
//
// @device: Device to be checked
//
// Return start port number
//
// rdma_for_each_port - Iterate over all valid port numbers of the IB device
// @device: The struct ib_device * to iterate over
// @iter: The unsigned int to store the port number
//

//
// rdma_end_port - Return the last valid port number for the device
// specified
//
// @device: Device to be checked
//
// Return last port number
//
// rdma_cap_ib_mad - Check if the port of a device supports Infiniband
// Management Datagrams.
// @device: Device to check
// @port_num: Port number to check
//
// Management Datagrams (MAD) are a required part of the InfiniBand
// specification and are supported on all InfiniBand devices.  A slightly
// extended version are also supported on OPA interfaces.
//
// Return: true if the port supports sending/receiving of MAD packets.
//
// rdma_cap_opa_mad - Check if the port of device provides support for OPA
// Management Datagrams.
// @device: Device to check
// @port_num: Port number to check
//
// Intel OmniPath devices extend and/or replace the InfiniBand Management
// datagrams with their own versions.  These OPA MADs share many but not all of
// the characteristics of InfiniBand MADs.
//
// OPA MADs differ in the following ways:
//
// 1) MADs are variable size up to 2K
// IBTA defined MADs remain fixed at 256 bytes
// 2) OPA SMPs must carry valid PKeys
// 3) OPA SMP packets are a different format
//
// Return: true if the port supports OPA MAD packet formats.
//
// rdma_cap_ib_smi - Check if the port of a device provides an Infiniband
// Subnet Management Agent (SMA) on the Subnet Management Interface (SMI).
// @device: Device to check
// @port_num: Port number to check
//
// Each InfiniBand node is required to provide a Subnet Management Agent
// that the subnet manager can access.  Prior to the fabric being fully
// configured by the subnet manager, the SMA is accessed via a well known
// interface called the Subnet Management Interface (SMI).  This interface
// uses directed route packets to communicate with the SM to get around the
// chicken and egg problem of the SM needing to know what's on the fabric
// in order to configure the fabric, and needing to configure the fabric in
// order to send packets to the devices on the fabric.  These directed
// route packets do not need the fabric fully configured in order to reach
// their destination.  The SMI is the only method allowed to send
// directed route packets on an InfiniBand fabric.
//
// Return: true if the port provides an SMI.
//
// rdma_cap_ib_cm - Check if the port of device has the capability Infiniband
// Communication Manager.
// @device: Device to check
// @port_num: Port number to check
//
// The InfiniBand Communication Manager is one of many pre-defined General
// Service Agents (GSA) that are accessed via the General Service
// Interface (GSI).  It's role is to facilitate establishment of connections
// between nodes as well as other management related tasks for established
// connections.
//
// Return: true if the port supports an IB CM (this does not guarantee that
// a CM is actually running however).
//
// rdma_cap_iw_cm - Check if the port of device has the capability IWARP
// Communication Manager.
// @device: Device to check
// @port_num: Port number to check
//
// Similar to above, but specific to iWARP connections which have a different
// managment protocol than InfiniBand.
//
// Return: true if the port supports an iWARP CM (this does not guarantee that
// a CM is actually running however).
//
// rdma_cap_ib_sa - Check if the port of device has the capability Infiniband
// Subnet Administration.
// @device: Device to check
// @port_num: Port number to check
//
// An InfiniBand Subnet Administration (SA) service is a pre-defined General
// Service Agent (GSA) provided by the Subnet Manager (SM).  On InfiniBand
// fabrics, devices should resolve routes to other hosts by contacting the
// SA to query the proper route.
//
// Return: true if the port should act as a client to the fabric Subnet
// Administration interface.  This does not imply that the SA service is
// running locally.
//
// rdma_cap_ib_mcast - Check if the port of device has the capability Infiniband
// Multicast.
// @device: Device to check
// @port_num: Port number to check
//
// InfiniBand multicast registration is more complex than normal IPv4 or
// IPv6 multicast registration.  Each Host Channel Adapter must register
// with the Subnet Manager when it wishes to join a multicast group.  It
// should do so only once regardless of how many queue pairs it subscribes
// to this group.  And it should leave the group only after all queue pairs
// attached to the group have been detached.
//
// Return: true if the port must undertake the additional adminstrative
// overhead of registering/unregistering with the SM and tracking of the
// total number of queue pairs attached to the multicast group.
//
extern "C" {
    pub fn rdma_cap_ib_sa(_arg: device, _arg: port_num) -> return;
}
//
// rdma_cap_af_ib - Check if the port of device has the capability
// Native Infiniband Address.
// @device: Device to check
// @port_num: Port number to check
//
// InfiniBand addressing uses a port's GUID + Subnet Prefix to make a default
// GID.  RoCE uses a different mechanism, but still generates a GID via
// a prescribed mechanism and port specific data.
//
// Return: true if the port uses a GID address to identify devices on the
// network.
//
// rdma_cap_eth_ah - Check if the port of device has the capability
// Ethernet Address Handle.
// @device: Device to check
// @port_num: Port number to check
//
// RoCE is InfiniBand over Ethernet, and it uses a well defined technique
// to fabricate GIDs over Ethernet/IP specific addresses native to the
// port.  Normally, packet headers are generated by the sending host
// adapter, but when sending connectionless datagrams, we must manually
// inject the proper headers for the fabric we are communicating over.
//
// Return: true if we are running as a RoCE port and must force the
// addition of a Global Route Header built from our Ethernet Address
// Handle into our header list for connectionless packets.
//
// rdma_cap_opa_ah - Check if the port of device supports
// OPA Address handles
// @device: Device to check
// @port_num: Port number to check
//
// Return: true if we are running on an OPA device which supports
// the extended OPA addressing.
//
// rdma_max_mad_size - Return the max MAD size required by this RDMA Port.
//
// @device: Device
// @port_num: Port number
//
// This MAD size includes the MAD headers and MAD payload.  No other headers
// are included.
//
// Return the max MAD size required by the Port.  Will return 0 if the port
// does not support MADs
//
// rdma_cap_roce_gid_table - Check if the port of device uses roce_gid_table
// @device: Device to check
// @port_num: Port number to check
//
// RoCE GID table mechanism manages the various GIDs for a device.
//
// NOTE: if allocating the port's GID table has failed, this call will still
// return true, but any RoCE GID table API will fail.
//
// Return: true if the port uses RoCE GID table mechanism in order to manage
// its GIDs.
//
// Check if the device supports READ W/ INVALIDATE.
//
// iWarp drivers must support READ W/ INVALIDATE.  No other protocol
// has support for it yet.
//
extern "C" {
    pub fn rdma_protocol_iwarp(_arg: dev, _arg: port_num) -> return;
}
//
// rdma_core_cap_opa_port - Return whether the RDMA Port is OPA or not.
// @device: Device
// @port_num: 1 based Port number
//
// Return true if port is an Intel OPA port , false if not
//
// rdma_mtu_enum_to_int - Return the mtu of the port as an integer value.
// @device: Device
// @port: Port number
// @mtu: enum value of MTU
//
// Return the MTU size supported by the port as an integer value. Will return
// -1 if enum value of mtu is not supported.
//
extern "C" {
    pub fn opa_mtu_enum_to_int(opa_mtu)mtu: (enum) -> return;
}
extern "C" {
    pub fn ib_mtu_enum_to_int(ib_mtu)mtu: (enum) -> return;
}
//
// rdma_mtu_from_attr - Return the mtu of the port from the port attribute.
// @device: Device
// @port: Port number
// @attr: port attribute
//
// Return the MTU size supported by the port as an integer value.
//
extern "C" {
    pub fn ib_mtu_enum_to_int(_arg: attr->max_mtu) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ib_pd_flags {
//
// Create a memory registration for all memory in the system and place
// the rkey for it into pd->unsafe_global_rkey.  This can be used by
// ULPs to avoid the overhead of dynamic MRs.
//
// This flag is generally considered unsafe and must only be used in
// extremly trusted environments.  Every use of it will log a warning
// in the kernel log.
//
    IB_PD_UNSAFE_GLOBAL_RKEY	= 0x01,
}

//
// ib_alloc_pd - Allocates an unused protection domain.
// @device: The device on which to allocate the protection domain.
// @flags: protection domain flags
//
// A protection domain object provides an association between QPs, shared
// receive queues, address handles, memory regions, and memory windows.
//
// Every PD has a local_dma_lkey which can be used as the lkey value for local
// memory operations.
//

extern "C" {
    pub fn ib_dealloc_pd_user(pd: *mut ib_pd, udata: *mut ib_udata) -> c_int;
}
//
// ib_dealloc_pd - Deallocate kernel PD
// @pd: The protection domain
//
// NOTE: for user PD use ib_dealloc_pd_user with valid udata!
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_create_ah_flags {
// In a sleepable context
    RDMA_CREATE_AH_SLEEPABLE = BIT(0),
}

//
// rdma_create_ah - Creates an address handle for the given address vector.
// @pd: The protection domain associated with the address handle.
// @ah_attr: The attributes of the address vector.
// @flags: Create address handle flags (see enum rdma_create_ah_flags).
//
// The address handle is used to reference a local or global destination
// in all UD QP post sends.
//
// rdma_create_user_ah - Creates an address handle for the given address vector.
// It resolves destination mac address for ah attribute of RoCE type.
// @pd: The protection domain associated with the address handle.
// @ah_attr: The attributes of the address vector.
// @udata: pointer to user's input output buffer information need by
// provider driver.
//
// It returns 0 on success and returns appropriate error code on error.
// The address handle is used to reference a local or global destination
// in all UD QP post sends.
//
// ib_get_gids_from_rdma_hdr - Get sgid and dgid from GRH or IPv4 header
// work completion.
// @hdr: the L3 header to parse
// @net_type: type of header to parse
// @sgid: place to store source gid
// @dgid: place to store destination gid
//
// ib_get_rdma_header_version - Get the header version
// @hdr: the L3 header to parse
//
extern "C" {
    pub fn ib_get_rdma_header_version(hdr: *const rdma_network_hdr) -> c_int;
}
//
// ib_init_ah_attr_from_wc - Initializes address handle attributes from a
// work completion.
// @device: Device on which the received message arrived.
// @port_num: Port on which the received message arrived.
// @wc: Work completion associated with the received message.
// @grh: References the received global route header.  This parameter is
// ignored unless the work completion indicates that the GRH is valid.
// @ah_attr: Returned attributes that can be used when creating an address
// handle for replying to the message.
// When ib_init_ah_attr_from_wc() returns success,
// (a) for IB link layer it optionally contains a reference to SGID attribute
// when GRH is present for IB link layer.
// (b) for RoCE link layer it contains a reference to SGID attribute.
// User must invoke rdma_cleanup_ah_attr_gid_attr() to release reference to SGID
// attributes which are initialized using ib_init_ah_attr_from_wc().
//
// ib_create_ah_from_wc - Creates an address handle associated with the
// sender of the specified work completion.
// @pd: The protection domain associated with the address handle.
// @wc: Work completion information associated with a received message.
// @grh: References the received global route header.  This parameter is
// ignored unless the work completion indicates that the GRH is valid.
// @port_num: The outbound port number to associate with the address.
//
// The address handle is used to reference a local or global destination
// in all UD QP post sends.
//
// rdma_modify_ah - Modifies the address vector associated with an address
// handle.
// @ah: The address handle to modify.
// @ah_attr: The new address vector attributes to associate with the
// address handle.
//
extern "C" {
    pub fn rdma_modify_ah(ah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
//
// rdma_query_ah - Queries the address vector associated with an address
// handle.
// @ah: The address handle to query.
// @ah_attr: The address vector attributes associated with the address
// handle.
//
extern "C" {
    pub fn rdma_query_ah(ah: *mut ib_ah, ah_attr: *mut rdma_ah_attr) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_destroy_ah_flags {
// In a sleepable context
    RDMA_DESTROY_AH_SLEEPABLE = BIT(0),
}

//
// rdma_destroy_ah_user - Destroys an address handle.
// @ah: The address handle to destroy.
// @flags: Destroy address handle flags (see enum rdma_destroy_ah_flags).
// @udata: Valid user data or NULL for kernel objects
//
extern "C" {
    pub fn rdma_destroy_ah_user(ah: *mut ib_ah, flags: u32, udata: *mut ib_udata) -> c_int;
}
//
// rdma_destroy_ah - Destroys an kernel address handle.
// @ah: The address handle to destroy.
// @flags: Destroy address handle flags (see enum rdma_destroy_ah_flags).
//
// NOTE: for user ah use rdma_destroy_ah_user with valid udata!
//
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ib_create_srq_user(_arg: pd, _arg: srq_init_attr, _arg: NULL, _arg: NULL) -> return;
}
//
// ib_modify_srq - Modifies the attributes for the specified SRQ.
// @srq: The SRQ to modify.
// @srq_attr: On input, specifies the SRQ attributes to modify.  On output,
// the current values of selected SRQ attributes are returned.
// @srq_attr_mask: A bit-mask used to specify which attributes of the SRQ
// are being modified.
//
// The mask may contain IB_SRQ_MAX_WR to resize the SRQ and/or
// IB_SRQ_LIMIT to set the SRQ's limit and request notification when
// the number of receives queued drops below the limit.
//
// ib_query_srq - Returns the attribute list and current values for the
// specified SRQ.
// @srq: The SRQ to query.
// @srq_attr: The attributes of the specified SRQ.
//
// ib_destroy_srq_user - Destroys the specified SRQ.
// @srq: The SRQ to destroy.
// @udata: Valid user data or NULL for kernel objects
//
extern "C" {
    pub fn ib_destroy_srq_user(srq: *mut ib_srq, udata: *mut ib_udata) -> c_int;
}
//
// ib_destroy_srq - Destroys the specified kernel SRQ.
// @srq: The SRQ to destroy.
//
// NOTE: for user srq use ib_destroy_srq_user with valid udata!
//
// ib_post_srq_recv - Posts a list of work requests to the specified SRQ.
// @srq: The SRQ to post the work request on.
// @recv_wr: A list of work requests to post on the receive queue.
// @bad_recv_wr: On an immediate failure, this parameter will reference
// the work request that failed to be posted on the QP.
//
// ib_create_qp - Creates a kernel QP associated with the specific protection
// domain.
// @pd: The protection domain associated with the QP.
// @init_attr: A list of initial attributes required to create the
// QP.  If QP creation succeeds, then the attributes are updated to
// the actual capabilities of the created QP.
//
extern "C" {
    pub fn ib_create_qp_kernel(_arg: pd, _arg: init_attr, _arg: KBUILD_MODNAME) -> return;
}
//
// ib_modify_qp_with_udata - Modifies the attributes for the specified QP.
// @qp: The QP to modify.
// @attr: On input, specifies the QP attributes to modify.  On output,
// the current values of selected QP attributes are returned.
// @attr_mask: A bit-mask used to specify which attributes of the QP
// are being modified.
// @udata: pointer to user's input output buffer information
// are being modified.
// It returns 0 on success and returns appropriate error code on error.
//
// ib_modify_qp - Modifies the attributes for the specified QP and then
// transitions the QP to the given state.
// @qp: The QP to modify.
// @qp_attr: On input, specifies the QP attributes to modify.  On output,
// the current values of selected QP attributes are returned.
// @qp_attr_mask: A bit-mask used to specify which attributes of the QP
// are being modified.
//
// ib_query_qp - Returns the attribute list and current values for the
// specified QP.
// @qp: The QP to query.
// @qp_attr: The attributes of the specified QP.
// @qp_attr_mask: A bit-mask used to select specific attributes to query.
// @qp_init_attr: Additional attributes of the selected QP.
//
// The qp_attr_mask may be used to limit the query to gathering only the
// selected attributes.
//
// ib_destroy_qp - Destroys the specified QP.
// @qp: The QP to destroy.
// @udata: Valid udata or NULL for kernel objects
//
extern "C" {
    pub fn ib_destroy_qp_user(qp: *mut ib_qp, udata: *mut ib_udata) -> c_int;
}
//
// ib_destroy_qp - Destroys the specified kernel QP.
// @qp: The QP to destroy.
//
// NOTE: for user qp use ib_destroy_qp_user with valid udata!
//
extern "C" {
    pub fn ib_destroy_qp_user(_arg: qp, _arg: NULL) -> return;
}
//
// ib_open_qp - Obtain a reference to an existing sharable QP.
// @xrcd: XRC domain
// @qp_open_attr: Attributes identifying the QP to open.
//
// Returns a reference to a sharable QP.
//
// ib_close_qp - Release an external reference to a QP.
// @qp: The QP handle to release
//
// The opened QP handle is released by the caller.  The underlying
// shared QP is not destroyed until all internal references are released.
//
extern "C" {
    pub fn ib_close_qp(qp: *mut ib_qp) -> c_int;
}
//
// ib_post_send - Posts a list of work requests to the send queue of
// the specified QP.
// @qp: The QP to post the work request on.
// @send_wr: A list of work requests to post on the send queue.
// @bad_send_wr: On an immediate failure, this parameter will reference
// the work request that failed to be posted on the QP.
//
// While IBA Vol. 1 section 11.4.1.1 specifies that if an immediate
// error is returned, the QP state shall not be affected,
// ib_post_send() will return an immediate error after queueing any
// earlier work requests in the list.
//
// ib_post_recv - Posts a list of work requests to the receive queue of
// the specified QP.
// @qp: The QP to post the work request on.
// @recv_wr: A list of work requests to post on the receive queue.
// @bad_recv_wr: On an immediate failure, this parameter will reference
// the work request that failed to be posted on the QP.
//
// ib_alloc_cq_any: Allocate kernel CQ
// @dev: The IB device
// @private: Private data attached to the CQE
// @nr_cqe: Number of CQEs in the CQ
// @poll_ctx: Context used for polling the CQ
//
extern "C" {
    pub fn ib_free_cq(cq: *mut ib_cq);
}
extern "C" {
    pub fn ib_process_cq_direct(cq: *mut ib_cq, budget: c_int) -> c_int;
}
//
// ib_create_cq - Creates a CQ on the specified device.
// @device: The device on which to create the CQ.
// @comp_handler: A user-specified callback that is invoked when a
// completion event occurs on the CQ.
// @event_handler: A user-specified callback that is invoked when an
// asynchronous event not associated with a completion occurs on the CQ.
// @cq_context: Context associated with the CQ returned to the user via
// the associated completion and event handlers.
// @cq_attr: The attributes the CQ should be created upon.
//
// Users can examine the cq structure to determine the actual CQ size.
//

//
// rdma_set_cq_moderation - Modifies moderation params of the CQ
// @cq: The CQ to modify.
// @cq_count: number of CQEs that will trigger an event
// @cq_period: max period of time in usec before triggering an event
//
extern "C" {
    pub fn rdma_set_cq_moderation(cq: *mut ib_cq, cq_count: u16, cq_period: u16) -> c_int;
}
//
// ib_destroy_cq_user - Destroys the specified CQ.
// @cq: The CQ to destroy.
// @udata: Valid user data or NULL for kernel objects
//
extern "C" {
    pub fn ib_destroy_cq_user(cq: *mut ib_cq, udata: *mut ib_udata) -> c_int;
}
//
// ib_destroy_cq - Destroys the specified kernel CQ.
// @cq: The CQ to destroy.
//
// NOTE: for user cq use ib_destroy_cq_user with valid udata!
//
// ib_poll_cq - poll a CQ for completion(s)
// @cq:the CQ being polled
// @num_entries:maximum number of completions to return
// @wc:array of at least @num_entries &struct ib_wc where completions
// will be returned
//
// Poll a CQ for (possibly multiple) completions.  If the return value
// is < 0, an error occurred.  If the return value is >= 0, it is the
// number of completions returned.  If the return value is
// non-negative and < num_entries, then the CQ was emptied.
//
// ib_req_notify_cq - Request completion notification on a CQ.
// @cq: The CQ to generate an event for.
// @flags:
// Must contain exactly one of %IB_CQ_SOLICITED or %IB_CQ_NEXT_COMP
// to request an event on the next solicited event or next work
// completion at any type, respectively. %IB_CQ_REPORT_MISSED_EVENTS
// may also be |ed in to request a hint about missed events, as
// described below.
//
// Return Value:
// < 0 means an error occurred while requesting notification
// == 0 means notification was requested successfully, and if
// IB_CQ_REPORT_MISSED_EVENTS was passed in, then no events
// were missed and it is safe to wait for another event.  In
// this case is it guaranteed that any work completions added
// to the CQ since the last CQ poll will trigger a completion
// notification event.
// > 0 is only returned if IB_CQ_REPORT_MISSED_EVENTS was passed
// in.  It means that the consumer must poll the CQ again to
// make sure it is empty to avoid missing an event because of a
// race between requesting notification and an entry being
// added to the CQ.  This return value means it is possible
// (but not guaranteed) that a work completion has been added
// to the CQ since the last poll without triggering a
// completion notification event.
//
extern "C" {
    pub fn ib_cq_pool_put(cq: *mut ib_cq, nr_cqe: c_uint);
}
//
// Drivers that don't need a DMA mapping at the RDMA layer, set dma_device to
// NULL. This causes the ib_dma* helpers to just stash the kernel virtual
// address into the dma address.
//
// Check if a IB device's underlying DMA mapping supports P2PDMA transfers.
//
extern "C" {
    pub fn dma_pci_p2pdma_supported(_arg: dev->dma_device) -> return;
}
//
// ib_virt_dma_to_ptr - Convert a dma_addr to a kernel pointer
// @dma_addr: The DMA address
//
// Used by ib_uses_virt_dma() devices to get back to the kernel pointer after
// going through the dma_addr marshalling.
//
// virt_dma mode maps the kvs's directly into the dma addr
//
// ib_virt_dma_to_page - Convert a dma_addr to a struct page
// @dma_addr: The DMA address
//
// Used by ib_uses_virt_dma() device to get back to the struct page after going
// through the dma_addr marshalling.
//
extern "C" {
    pub fn virt_to_page(_arg: ib_virt_dma_to_ptr(dma_addr)) -> return;
}
//
// ib_dma_mapping_error - check a DMA addr for error
// @dev: The device for which the dma_addr was created
// @dma_addr: The DMA address to check
//
extern "C" {
    pub fn dma_mapping_error(_arg: dev->dma_device, _arg: dma_addr) -> return;
}
//
// ib_dma_map_single - Map a kernel virtual address to DMA address
// @dev: The device for which the dma_addr is to be created
// @cpu_addr: The kernel virtual address
// @size: The size of the region in bytes
// @direction: The direction of the DMA
//
extern "C" {
    pub fn dma_map_single(_arg: dev->dma_device, _arg: cpu_addr, _arg: size, _arg: direction) -> return;
}
//
// ib_dma_unmap_single - Destroy a mapping created by ib_dma_map_single()
// @dev: The device for which the DMA address was created
// @addr: The DMA address
// @size: The size of the region in bytes
// @direction: The direction of the DMA
//
// ib_dma_map_page - Map a physical page to DMA address
// @dev: The device for which the dma_addr is to be created
// @page: The page to be mapped
// @offset: The offset within the page
// @size: The size of the region in bytes
// @direction: The direction of the DMA
//
extern "C" {
    pub fn dma_map_page(_arg: dev->dma_device, _arg: page, _arg: offset, _arg: size, _arg: direction) -> return;
}
//
// ib_dma_unmap_page - Destroy a mapping created by ib_dma_map_page()
// @dev: The device for which the DMA address was created
// @addr: The DMA address
// @size: The size of the region in bytes
// @direction: The direction of the DMA
//
// ib_dma_map_bvec - Map a bio_vec to DMA address
// @dev: The device for which the dma_addr is to be created
// @bvec: The bio_vec to map
// @direction: The direction of the DMA
//
// Returns a DMA address for the bio_vec. The caller must check the
// result with ib_dma_mapping_error() before use; a failed mapping
// must not be passed to ib_dma_unmap_bvec().
//
// For software RDMA devices (rxe, siw), returns a virtual address
// and no actual DMA mapping occurs.
//
// ib_dma_unmap_bvec - Unmap a bio_vec DMA mapping
// @dev: The device for which the DMA address was created
// @addr: The DMA address returned by ib_dma_map_bvec()
// @size: The size of the region in bytes
// @direction: The direction of the DMA
//
// Releases a DMA mapping created by ib_dma_map_bvec(). For software
// RDMA devices this is a no-op since no actual mapping occurred.
//
extern "C" {
    pub fn ib_dma_virt_map_sg(dev: *mut ib_device, sg: *mut scatterlist, nents: c_int) -> c_int;
}
extern "C" {
    pub fn ib_dma_virt_map_sg(_arg: dev, _arg: sg, _arg: nents) -> return;
}
//
// ib_dma_map_sgtable_attrs - Map a scatter/gather table to DMA addresses
// @dev: The device for which the DMA addresses are to be created
// @sgt: The sg_table object describing the buffer
// @direction: The direction of the DMA
// @dma_attrs: Optional DMA attributes for the map operation
//
extern "C" {
    pub fn dma_map_sgtable(_arg: dev->dma_device, _arg: sgt, _arg: direction, _arg: dma_attrs) -> return;
}
//
// ib_dma_map_sg - Map a scatter/gather list to DMA addresses
// @dev: The device for which the DMA addresses are to be created
// @sg: The array of scatter/gather entries
// @nents: The number of scatter/gather entries
// @direction: The direction of the DMA
//
extern "C" {
    pub fn ib_dma_map_sg_attrs(_arg: dev, _arg: sg, _arg: nents, _arg: direction, _arg: 0) -> return;
}
//
// ib_dma_unmap_sg - Unmap a scatter/gather list of DMA addresses
// @dev: The device for which the DMA addresses were created
// @sg: The array of scatter/gather entries
// @nents: The number of scatter/gather entries
// @direction: The direction of the DMA
//
// ib_dma_max_seg_size - Return the size limit of a single DMA transfer
// @dev: The device to query
//
// The returned value represents a size in bytes.
//
extern "C" {
    pub fn dma_get_max_seg_size(_arg: dev->dma_device) -> return;
}
//
// ib_dma_sync_single_for_cpu - Prepare DMA region to be accessed by CPU
// @dev: The device for which the DMA address was created
// @addr: The DMA address
// @size: The size of the region in bytes
// @dir: The direction of the DMA
//
// ib_dma_sync_single_for_device - Prepare DMA region to be accessed by device
// @dev: The device for which the DMA address was created
// @addr: The DMA address
// @size: The size of the region in bytes
// @dir: The direction of the DMA
//
// ib_reg_user_mr - register a memory region for virtual addresses from kernel
// space. This function should be called when 'current' is the owning MM.
//
// ib_advise_mr -  give an advice about an address range in a memory region
//
// ib_dereg_mr_user - Deregisters a memory region and removes it from the
// HCA translation table.
// @mr: The memory region to deregister.
// @udata: Valid user data or NULL for kernel object
//
// This function can fail, if the memory region has memory windows bound to it.
//
extern "C" {
    pub fn ib_dereg_mr_user(mr: *mut ib_mr, udata: *mut ib_udata) -> c_int;
}
//
// ib_dereg_mr - Deregisters a kernel memory region and removes it from the
// HCA translation table.
// @mr: The memory region to deregister.
//
// This function can fail, if the memory region has memory windows bound to it.
//
// NOTE: for user mr use ib_dereg_mr_user with valid udata!
//
extern "C" {
    pub fn ib_dereg_mr_user(_arg: mr, _arg: NULL) -> return;
}
//
// ib_update_fast_reg_key - updates the key portion of the fast_reg MR
// R_Key and L_Key.
// @mr: struct ib_mr pointer to be updated.
// @newkey: new key to be used.
//
// ib_inc_rkey - increments the key portion of the given rkey. Can be used
// for calculating a new rkey for type 2 memory windows.
// @rkey: the rkey to increment.
//
// ib_attach_mcast - Attaches the specified QP to a multicast group.
// @qp: QP to attach to the multicast group.  The QP must be type
// IB_QPT_UD.
// @gid: Multicast group GID.
// @lid: Multicast group LID in host byte order.
//
// In order to send and receive multicast packets, subnet
// administration must have created the multicast group and configured
// the fabric appropriately.  The port associated with the specified
// QP must also be a member of the multicast group.
//
extern "C" {
    pub fn ib_attach_mcast(qp: *mut ib_qp, gid: *mut ib_gid, lid: u16) -> c_int;
}
//
// ib_detach_mcast - Detaches the specified QP from a multicast group.
// @qp: QP to detach from the multicast group.
// @gid: Multicast group GID.
// @lid: Multicast group LID in host byte order.
//
extern "C" {
    pub fn ib_detach_mcast(qp: *mut ib_qp, gid: *mut ib_gid, lid: u16) -> c_int;
}
extern "C" {
    pub fn ib_dealloc_xrcd_user(xrcd: *mut ib_xrcd, udata: *mut ib_udata) -> c_int;
}
//
// Local write permission is required if remote write or
// remote atomic permission is also requested.
//
// We have writable memory backing the MR if any of the following
// access flags are set.  "Local write" and "remote write" obviously
// require write access.  "Remote atomic" can do things like fetch and
// add, which will modify memory, and "MW bind" can change permissions
// by binding a window.
//
// ib_check_mr_status: lightweight check of MR status.
// This routine may provide status checks on a selected
// ib_mr. first use is for signature status check.
//
// @mr: A memory region.
// @check_mask: Bitmask of which checks to perform from
// ib_mr_status_check enumeration.
// @mr_status: The container of relevant status checks.
// failed checks will be indicated in the status bitmask
// and the relevant info shall be in the error item.
//
// ib_device_try_get: Hold a registration lock
// @dev: The device to lock
//
// A device under an active registration lock cannot become unregistered. It
// is only possible to obtain a registration lock on a device that is fully
// registered, otherwise this function returns false.
//
// The registration lock is only necessary for actions which require the
// device to still be registered. Uses that only require the device pointer to
// be valid should use get_device(&ibdev->dev) to hold the memory.
//
extern "C" {
    pub fn refcount_inc_not_zero(_arg: &dev->refcount) -> return;
}
extern "C" {
    pub fn ib_device_put(device: *mut ib_device);
}
extern "C" {
    pub fn ib_destroy_wq_user(wq: *mut ib_wq, udata: *mut ib_udata) -> c_int;
}
extern "C" {
    pub fn ib_drain_rq(qp: *mut ib_qp);
}
extern "C" {
    pub fn ib_drain_sq(qp: *mut ib_qp);
}
extern "C" {
    pub fn ib_drain_qp(qp: *mut ib_qp);
}
// rdma_ah_read_grh(const struct rdma_ah_attr *attr)
// To retrieve and modify the grh
// rdma_ah_retrieve_grh(struct rdma_ah_attr *attr)
extern "C" {
    pub fn rdma_destroy_ah_attr(ah_attr: *mut rdma_ah_attr);
}
extern "C" {
    pub fn rdma_move_ah_attr(dest: *mut rdma_ah_attr, src: *mut rdma_ah_attr);
}
//
// rdma_ah_find_type - Return address handle type.
//
// @dev: Device to be checked
// @port_num: Port number
//
// ib_lid_cpu16 - Return lid in 16bit CPU encoding.
// In the current implementation the only way to
// get the 32bit lid is from other sources for OPA.
// For IB, lids will always be 16bits so cast the
// value accordingly.
//
// @lid: A 32bit LID
//
// ib_lid_be16 - Return lid in 16bit BE encoding.
//
// @lid: A 32bit LID
//
extern "C" {
    pub fn cpu_to_be16(_arg: (u16)lid) -> return;
}
//
// rdma_roce_rescan_device - Rescan all of the network devices in the system
// and add their gids, as needed, to the relevant RoCE devices.
//
// @ibdev:         the rdma device
//
extern "C" {
    pub fn rdma_roce_rescan_device(ibdev: *mut ib_device);
}
extern "C" {
    pub fn rdma_roce_rescan_port(ib_dev: *mut ib_device, port: u32);
}

extern "C" {
    pub fn uverbs_destroy_def_handler(attrs: *mut uverbs_attr_bundle) -> c_int;
}
extern "C" {
    pub fn rdma_uattrs_has_raw_cap(attrs: *const uverbs_attr_bundle) -> bool;
}

//
// rdma_device_to_ibdev - Get ib_device pointer from device pointer
//
// @device:	device pointer for which ib_device pointer to retrieve
//
// rdma_device_to_ibdev() retrieves ib_device pointer from device.
//
// ibdev_to_node - return the NUMA node for a given ib_device
// @ibdev:	device to get the NUMA node for.
//
extern "C" {
    pub fn dev_to_node(_arg: parent) -> return;
}
//
// rdma_device_to_drv_device - Helper macro to reach back to driver's
// ib_device holder structure from device pointer.
//
// NOTE: New drivers should not make use of this API; This API is only for
// existing drivers who have exposed sysfs entries using
// ops->device_group.
//

extern "C" {
    pub fn rdma_dev_has_raw_cap(dev: *const ib_device) -> bool;
}
extern "C" {
    pub fn read_pnet(_arg: &device->coredev.rdma_net) -> return;
}

//
// rdma_flow_label_to_udp_sport - generate a RoCE v2 UDP src port value based
// on the flow_label
// @fl: flow_label value
//
// This function will convert the 20 bit flow_label input to a valid RoCE v2
// UDP src port 14 bit value. All RoCE V2 drivers should use this same
// convention.
//
// rdma_calc_flow_label - generate a RDMA symmetric flow label value based on
// local and remote qpn values
//
// This function folded the multiplication results of two qpns, 24 bit each,
// fields, and converts it to a 20 bit results.
//
// This function will create symmetric flow_label value based on the local
// and remote qpn values. this will allow both the requester and responder
// to calculate the same flow_label for a given connection.
//
// This helper function should be used by driver in case the upper layer
// provide a zero flow_label value. This is to improve entropy of RDMA
// traffic in the network.
//
// rdma_get_udp_sport - Calculate and set UDP source port based on the flow
// label. If flow label is not defined in GRH then
// calculate it based on lqpn/rqpn.
//
// @fl:                 flow label from GRH
// @lqpn:               local qp number
// @rqpn:               remote qp number
//
extern "C" {
    pub fn rdma_flow_label_to_udp_sport(_arg: fl) -> return;
}
// ib_add_sub_device - Add a sub IB device on an existing one
//
// @parent: The IB device that needs to add a sub device
// @type: The type of the new sub device
// @name: The name of the new sub device
//
// Return 0 on success, an error code otherwise
//
// ib_del_sub_device_and_put - Delect an IB sub device while holding a 'get'
//
// @sub: The sub device that is going to be deleted
//
// Return 0 on success, an error code otherwise
//
extern "C" {
    pub fn ib_del_sub_device_and_put(sub: *mut ib_device) -> c_int;
}
