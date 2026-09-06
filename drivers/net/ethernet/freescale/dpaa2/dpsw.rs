//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/freescale/dpaa2/dpsw.h
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
//
// Copyright 2014-2016 Freescale Semiconductor Inc.
// Copyright 2017-2021 NXP
//
// Data Path L2-Switch API
// Contains API for handling DPSW topology and functionality
//
// DPSW general definitions
pub const DPSW_MAX_PRIORITIES: c_int = 8;
pub const DPSW_MAX_IF: c_int = 64;
pub const DPSW_MAX_LAG_IFS: c_int = 8;
extern "C" {
    pub fn dpsw_open(mc_io: *mut fsl_mc_io, cmd_flags: u32, dpsw_id: c_int, token: *mut u16) -> c_int;
}
extern "C" {
    pub fn dpsw_close(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
// DPSW options
//
// DPSW_OPT_FLOODING_DIS - Flooding was disabled at device create
//
pub const DPSW_OPT_FLOODING_DIS: c_uint = 0x0000000000000001ULL;
//
// DPSW_OPT_MULTICAST_DIS - Multicast was disabled at device create
//
pub const DPSW_OPT_MULTICAST_DIS: c_uint = 0x0000000000000004ULL;
//
// DPSW_OPT_CTRL_IF_DIS - Control interface support is disabled
//
pub const DPSW_OPT_CTRL_IF_DIS: c_uint = 0x0000000000000010ULL;
//
// enum dpsw_component_type - component type of a bridge
// @DPSW_COMPONENT_TYPE_C_VLAN: A C-VLAN component of an
// enterprise VLAN bridge or of a Provider Bridge used
// to process C-tagged frames
// @DPSW_COMPONENT_TYPE_S_VLAN: An S-VLAN component of a
// Provider Bridge
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_component_type {
    DPSW_COMPONENT_TYPE_C_VLAN = 0,
    DPSW_COMPONENT_TYPE_S_VLAN
}

//
// enum dpsw_flooding_cfg - flooding configuration requested
// @DPSW_FLOODING_PER_VLAN: Flooding replicators are allocated per VLAN and
// interfaces present in each of them can be configured using
// dpsw_vlan_add_if_flooding()/dpsw_vlan_remove_if_flooding().
// This is the default configuration.
//
// @DPSW_FLOODING_PER_FDB: Flooding replicators are allocated per FDB and
// interfaces present in each of them can be configured using
// dpsw_set_egress_flood().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_flooding_cfg {
    DPSW_FLOODING_PER_VLAN = 0,
    DPSW_FLOODING_PER_FDB,
}

//
// enum dpsw_broadcast_cfg - broadcast configuration requested
// @DPSW_BROADCAST_PER_OBJECT: There is only one broadcast replicator per DPSW
// object. This is the default configuration.
// @DPSW_BROADCAST_PER_FDB: Broadcast replicators are allocated per FDB and
// interfaces present in each of them can be configured using
// dpsw_set_egress_flood().
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_broadcast_cfg {
    DPSW_BROADCAST_PER_OBJECT = 0,
    DPSW_BROADCAST_PER_FDB,
}

extern "C" {
    pub fn dpsw_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpsw_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpsw_reset(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
// DPSW IRQ Index and Events
pub const DPSW_IRQ_INDEX_IF: c_uint = 0x0000;
pub const DPSW_IRQ_INDEX_L2SW: c_uint = 0x0001;
//
// DPSW_IRQ_EVENT_LINK_CHANGED - Indicates that the link state changed
//
pub const DPSW_IRQ_EVENT_LINK_CHANGED: c_uint = 0x0001;
//
// DPSW_IRQ_EVENT_ENDPOINT_CHANGED - Indicates a change in endpoint
//
pub const DPSW_IRQ_EVENT_ENDPOINT_CHANGED: c_uint = 0x0002;
//
// struct dpsw_irq_cfg - IRQ configuration
// @addr:	Address that must be written to signal a message-based interrupt
// @val:	Value to write into irq_addr address
// @irq_num: A user defined number associated with this IRQ
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_irq_cfg {
    pub addr: u64,
    pub val: u32,
    pub irq_num: c_int,
}

//
// struct dpsw_attr - Structure representing DPSW attributes
// @id: DPSW object ID
// @options: Enable/Disable DPSW features
// @max_vlans: Maximum Number of VLANs
// @max_meters_per_if:  Number of meters per interface
// @max_fdbs: Maximum Number of FDBs
// @max_fdb_entries: Number of FDB entries for default FDB table;
// 0 - indicates default 1024 entries.
// @fdb_aging_time: Default FDB aging time for default FDB table;
// 0 - indicates default 300 seconds
// @max_fdb_mc_groups: Number of multicast groups in each FDB table;
// 0 - indicates default 32
// @mem_size: DPSW frame storage memory size
// @num_ifs: Number of interfaces
// @num_vlans: Current number of VLANs
// @num_fdbs: Current number of FDBs
// @component_type: Component type of this bridge
// @flooding_cfg: Flooding configuration (PER_VLAN - default, PER_FDB)
// @broadcast_cfg: Broadcast configuration (PER_OBJECT - default, PER_FDB)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_attr {
    pub id: c_int,
    pub options: u64,
    pub max_vlans: u16,
    pub max_meters_per_if: u8,
    pub max_fdbs: u8,
    pub max_fdb_entries: u16,
    pub fdb_aging_time: u16,
    pub max_fdb_mc_groups: u16,
    pub num_ifs: u16,
    pub mem_size: u16,
    pub num_vlans: u16,
    pub num_fdbs: u8,
    pub component_type: dpsw_component_type,
    pub flooding_cfg: dpsw_flooding_cfg,
    pub broadcast_cfg: dpsw_broadcast_cfg,
}

//
// struct dpsw_ctrl_if_attr - Control interface attributes
// @rx_fqid:		Receive FQID
// @rx_err_fqid:	Receive error FQID
// @tx_err_conf_fqid:	Transmit error and confirmation FQID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_ctrl_if_attr {
    pub rx_fqid: u32,
    pub rx_err_fqid: u32,
    pub tx_err_conf_fqid: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_queue_type {
    DPSW_QUEUE_RX,
    DPSW_QUEUE_TX_ERR_CONF,
    DPSW_QUEUE_RX_ERR,
}

pub const DPSW_MAX_DPBP: c_int = 8;
//
// struct dpsw_ctrl_if_pools_cfg - Control interface buffer pools configuration
// @num_dpbp: Number of DPBPs
// @pools: Array of buffer pools parameters; The number of valid entries
// must match 'num_dpbp' value
// @pools.dpbp_id: DPBP object ID
// @pools.buffer_size: Buffer size
// @pools.backup_pool: Backup pool
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_ctrl_if_pools_cfg {
    pub num_dpbp: u8,
    pub dpbp_id: c_int,
    pub buffer_size: u16,
    pub backup_pool: c_int,
    pub pools: [}; DPSW_MAX_DPBP],
}

pub const DPSW_CTRL_IF_QUEUE_OPT_USER_CTX: c_uint = 0x00000001;
pub const DPSW_CTRL_IF_QUEUE_OPT_DEST: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_ctrl_if_dest {
    DPSW_CTRL_IF_DEST_NONE = 0,
    DPSW_CTRL_IF_DEST_DPIO = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_ctrl_if_dest_cfg {
    pub dest_type: dpsw_ctrl_if_dest,
    pub dest_id: c_int,
    pub priority: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_ctrl_if_queue_cfg {
    pub options: u32,
    pub user_ctx: u64,
    pub dest_cfg: dpsw_ctrl_if_dest_cfg,
}

extern "C" {
    pub fn dpsw_ctrl_if_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
extern "C" {
    pub fn dpsw_ctrl_if_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16) -> c_int;
}
//
// enum dpsw_action - Action selection for special/control frames
// @DPSW_ACTION_DROP: Drop frame
// @DPSW_ACTION_REDIRECT: Redirect frame to control port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_action {
    DPSW_ACTION_DROP = 0,
    DPSW_ACTION_REDIRECT = 1
}

pub const DPSW_LINK_OPT_AUTONEG: c_uint = 0x0000000000000001ULL;
pub const DPSW_LINK_OPT_HALF_DUPLEX: c_uint = 0x0000000000000002ULL;
pub const DPSW_LINK_OPT_PAUSE: c_uint = 0x0000000000000004ULL;
pub const DPSW_LINK_OPT_ASYM_PAUSE: c_uint = 0x0000000000000008ULL;
//
// struct dpsw_link_cfg - Structure representing DPSW link configuration
// @rate: Rate
// @options: Mask of available options; use 'DPSW_LINK_OPT_<X>' values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_link_cfg {
    pub rate: u32,
    pub options: u64,
}

//
// struct dpsw_link_state - Structure representing DPSW link state
// @rate: Rate
// @options: Mask of available options; use 'DPSW_LINK_OPT_<X>' values
// @up: 0 - covers two cases: down and disconnected, 1 - up
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_link_state {
    pub rate: u32,
    pub options: u64,
    pub up: u8,
}

//
// struct dpsw_tci_cfg - Tag Control Information (TCI) configuration
// @pcp: Priority Code Point (PCP): a 3-bit field which refers
// to the IEEE 802.1p priority
// @dei: Drop Eligible Indicator (DEI): a 1-bit field. May be used
// separately or in conjunction with PCP to indicate frames
// eligible to be dropped in the presence of congestion
// @vlan_id: VLAN Identifier (VID): a 12-bit field specifying the VLAN
// to which the frame belongs. The hexadecimal values
// of 0x000 and 0xFFF are reserved;
// all other values may be used as VLAN identifiers,
// allowing up to 4,094 VLANs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_tci_cfg {
    pub pcp: u8,
    pub dei: u8,
    pub vlan_id: u16,
}

//
// enum dpsw_stp_state - Spanning Tree Protocol (STP) states
// @DPSW_STP_STATE_DISABLED: Disabled state
// @DPSW_STP_STATE_LISTENING: Listening state
// @DPSW_STP_STATE_LEARNING: Learning state
// @DPSW_STP_STATE_FORWARDING: Forwarding state
// @DPSW_STP_STATE_BLOCKING: Blocking state
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_stp_state {
    DPSW_STP_STATE_DISABLED = 0,
    DPSW_STP_STATE_LISTENING = 1,
    DPSW_STP_STATE_LEARNING = 2,
    DPSW_STP_STATE_FORWARDING = 3,
    DPSW_STP_STATE_BLOCKING = 0
}

//
// struct dpsw_stp_cfg - Spanning Tree Protocol (STP) Configuration
// @vlan_id: VLAN ID STP state
// @state: STP state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_stp_cfg {
    pub vlan_id: u16,
    pub state: dpsw_stp_state,
}

//
// enum dpsw_accepted_frames - Types of frames to accept
// @DPSW_ADMIT_ALL: The device accepts VLAN tagged, untagged and
// priority tagged frames
// @DPSW_ADMIT_ONLY_VLAN_TAGGED: The device discards untagged frames or
// Priority-Tagged frames received on this interface.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_accepted_frames {
    DPSW_ADMIT_ALL = 1,
    DPSW_ADMIT_ONLY_VLAN_TAGGED = 3
}

//
// enum dpsw_counter  - Counters types
// @DPSW_CNT_ING_FRAME: Counts ingress frames
// @DPSW_CNT_ING_BYTE: Counts ingress bytes
// @DPSW_CNT_ING_FLTR_FRAME: Counts filtered ingress frames
// @DPSW_CNT_ING_FRAME_DISCARD: Counts discarded ingress frame
// @DPSW_CNT_ING_MCAST_FRAME: Counts ingress multicast frames
// @DPSW_CNT_ING_MCAST_BYTE: Counts ingress multicast bytes
// @DPSW_CNT_ING_BCAST_FRAME: Counts ingress broadcast frames
// @DPSW_CNT_ING_BCAST_BYTES: Counts ingress broadcast bytes
// @DPSW_CNT_EGR_FRAME: Counts egress frames
// @DPSW_CNT_EGR_BYTE: Counts egress bytes
// @DPSW_CNT_EGR_FRAME_DISCARD: Counts discarded egress frames
// @DPSW_CNT_EGR_STP_FRAME_DISCARD: Counts egress STP discarded frames
// @DPSW_CNT_ING_NO_BUFF_DISCARD: Counts ingress no buffer discarded frames
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_counter {
    DPSW_CNT_ING_FRAME = 0x0,
    DPSW_CNT_ING_BYTE = 0x1,
    DPSW_CNT_ING_FLTR_FRAME = 0x2,
    DPSW_CNT_ING_FRAME_DISCARD = 0x3,
    DPSW_CNT_ING_MCAST_FRAME = 0x4,
    DPSW_CNT_ING_MCAST_BYTE = 0x5,
    DPSW_CNT_ING_BCAST_FRAME = 0x6,
    DPSW_CNT_ING_BCAST_BYTES = 0x7,
    DPSW_CNT_EGR_FRAME = 0x8,
    DPSW_CNT_EGR_BYTE = 0x9,
    DPSW_CNT_EGR_FRAME_DISCARD = 0xa,
    DPSW_CNT_EGR_STP_FRAME_DISCARD = 0xb,
    DPSW_CNT_ING_NO_BUFF_DISCARD = 0xc,
}

extern "C" {
    pub fn dpsw_if_enable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, if_id: u16) -> c_int;
}
extern "C" {
    pub fn dpsw_if_disable(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, if_id: u16) -> c_int;
}
//
// struct dpsw_if_attr - Structure representing DPSW interface attributes
// @num_tcs: Number of traffic classes
// @rate: Transmit rate in bits per second
// @options: Interface configuration options (bitmap)
// @enabled: Indicates if interface is enabled
// @accept_all_vlan: The device discards/accepts incoming frames
// for VLANs that do not include this interface
// @admit_untagged: When set to 'DPSW_ADMIT_ONLY_VLAN_TAGGED', the device
// discards untagged frames or priority-tagged frames received on
// this interface;
// When set to 'DPSW_ADMIT_ALL', untagged frames or priority-
// tagged frames received on this interface are accepted
// @qdid: control frames transmit qdid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_if_attr {
    pub num_tcs: u8,
    pub rate: u32,
    pub options: u32,
    pub enabled: c_int,
    pub accept_all_vlan: c_int,
    pub admit_untagged: dpsw_accepted_frames,
    pub qdid: u16,
}

//
// struct dpsw_vlan_cfg - VLAN Configuration
// @fdb_id: Forwarding Data Base
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_vlan_cfg {
    pub fdb_id: u16,
}

pub const DPSW_VLAN_ADD_IF_OPT_FDB_ID: c_uint = 0x0001;
//
// struct dpsw_vlan_if_cfg - Set of VLAN Interfaces
// @num_ifs: The number of interfaces that are assigned to the egress
// list for this VLAN
// @if_id: The set of interfaces that are
// assigned to the egress list for this VLAN
// @options: Options map for this command (DPSW_VLAN_ADD_IF_OPT_FDB_ID)
// @fdb_id: FDB id to be used by this VLAN on these specific interfaces
// (taken into account only if the DPSW_VLAN_ADD_IF_OPT_FDB_ID is
// specified in the options field)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_vlan_if_cfg {
    pub num_ifs: u16,
    pub options: u16,
    pub if_id: [u16; DPSW_MAX_IF],
    pub fdb_id: u16,
}

//
// enum dpsw_fdb_entry_type - FDB Entry type - Static/Dynamic
// @DPSW_FDB_ENTRY_STATIC: Static entry
// @DPSW_FDB_ENTRY_DINAMIC: Dynamic entry
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_fdb_entry_type {
    DPSW_FDB_ENTRY_STATIC = 0,
    DPSW_FDB_ENTRY_DINAMIC = 1
}

//
// struct dpsw_fdb_unicast_cfg - Unicast entry configuration
// @type: Select static or dynamic entry
// @mac_addr: MAC address
// @if_egress: Egress interface ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_fdb_unicast_cfg {
    pub type: dpsw_fdb_entry_type,
    pub mac_addr: [u8; 6],
    pub if_egress: u16,
}

//
// struct fdb_dump_entry - fdb snapshot entry
// @mac_addr: MAC address
// @type: bit0 - DINAMIC(1)/STATIC(0), bit1 - UNICAST(1)/MULTICAST(0)
// @if_info: unicast - egress interface, multicast - number of egress interfaces
// @if_mask: multicast - egress interface mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fdb_dump_entry {
    pub mac_addr: [u8; 6],
    pub type: u8,
    pub if_info: u8,
    pub if_mask: [u8; 8],
}

//
// struct dpsw_fdb_multicast_cfg - Multi-cast entry configuration
// @type: Select static or dynamic entry
// @mac_addr: MAC address
// @num_ifs: Number of external and internal interfaces
// @if_id: Egress interface IDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_fdb_multicast_cfg {
    pub type: dpsw_fdb_entry_type,
    pub mac_addr: [u8; 6],
    pub num_ifs: u16,
    pub if_id: [u16; DPSW_MAX_IF],
}

//
// enum dpsw_learning_mode - Auto-learning modes
// @DPSW_LEARNING_MODE_DIS: Disable Auto-learning
// @DPSW_LEARNING_MODE_HW: Enable HW auto-Learning
// @DPSW_LEARNING_MODE_NON_SECURE: Enable None secure learning by CPU
// @DPSW_LEARNING_MODE_SECURE: Enable secure learning by CPU
//
// NONE - SECURE LEARNING
// SMAC found	DMAC found	CTLU Action
// v		v	Forward frame to
// 1.  DMAC destination
// -		v	Forward frame to
// 1.  DMAC destination
// 2.  Control interface
// v		-	Forward frame to
// 1.  Flooding list of interfaces
// -		-	Forward frame to
// 1.  Flooding list of interfaces
// 2.  Control interface
// SECURE LEARING
// SMAC found	DMAC found	CTLU Action
// v		v		Forward frame to
// 1.  DMAC destination
// -		v		Forward frame to
// 1.  Control interface
// v		-		Forward frame to
// 1.  Flooding list of interfaces
// -		-		Forward frame to
// 1.  Control interface
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_learning_mode {
    DPSW_LEARNING_MODE_DIS = 0,
    DPSW_LEARNING_MODE_HW = 1,
    DPSW_LEARNING_MODE_NON_SECURE = 2,
    DPSW_LEARNING_MODE_SECURE = 3
}

//
// struct dpsw_fdb_attr - FDB Attributes
// @max_fdb_entries: Number of FDB entries
// @fdb_ageing_time: Ageing time in seconds
// @learning_mode: Learning mode
// @num_fdb_mc_groups: Current number of multicast groups
// @max_fdb_mc_groups: Maximum number of multicast groups
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_fdb_attr {
    pub max_fdb_entries: u16,
    pub fdb_ageing_time: u16,
    pub learning_mode: dpsw_learning_mode,
    pub num_fdb_mc_groups: u16,
    pub max_fdb_mc_groups: u16,
}

//
// struct dpsw_fdb_cfg  - FDB Configuration
// @num_fdb_entries: Number of FDB entries
// @fdb_ageing_time: Ageing time in seconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_fdb_cfg {
    pub num_fdb_entries: u16,
    pub fdb_ageing_time: u16,
}

extern "C" {
    pub fn dpsw_fdb_remove(mc_io: *mut fsl_mc_io, cmd_flags: u32, token: u16, fdb_id: u16) -> c_int;
}
//
// enum dpsw_flood_type - Define the flood type of a DPSW object
// @DPSW_BROADCAST: Broadcast flooding
// @DPSW_FLOODING: Unknown flooding
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_flood_type {
    DPSW_BROADCAST = 0,
    DPSW_FLOODING,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_egress_flood_cfg {
    pub fdb_id: u16,
    pub flood_type: dpsw_flood_type,
    pub num_ifs: u16,
    pub if_id: [u16; DPSW_MAX_IF],
}

//
// struct dpsw_acl_cfg - ACL Configuration
// @max_entries: Number of ACL rules
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_acl_cfg {
    pub max_entries: u16,
}

//
// struct dpsw_acl_if_cfg - List of interfaces to associate with an ACL table
// @num_ifs: Number of interfaces
// @if_id: List of interfaces
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_acl_if_cfg {
    pub num_ifs: u16,
    pub if_id: [u16; DPSW_MAX_IF],
}

//
// struct dpsw_acl_fields - ACL fields.
// @l2_dest_mac: Destination MAC address: BPDU, Multicast, Broadcast, Unicast,
// slow protocols, MVRP, STP
// @l2_source_mac: Source MAC address
// @l2_tpid: Layer 2 (Ethernet) protocol type, used to identify the following
// protocols: MPLS, PTP, PFC, ARP, Jumbo frames, LLDP, IEEE802.1ae,
// Q-in-Q, IPv4, IPv6, PPPoE
// @l2_pcp_dei: indicate which protocol is encapsulated in the payload
// @l2_vlan_id: layer 2 VLAN ID
// @l2_ether_type: layer 2 Ethernet type
// @l3_dscp: Layer 3 differentiated services code point
// @l3_protocol: Tells the Network layer at the destination host, to which
// Protocol this packet belongs to. The following protocol are
// supported: ICMP, IGMP, IPv4 (encapsulation), TCP, IPv6
// (encapsulation), GRE, PTP
// @l3_source_ip: Source IPv4 IP
// @l3_dest_ip: Destination IPv4 IP
// @l4_source_port: Source TCP/UDP Port
// @l4_dest_port: Destination TCP/UDP Port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_acl_fields {
    pub l2_dest_mac: [u8; 6],
    pub l2_source_mac: [u8; 6],
    pub l2_tpid: u16,
    pub l2_pcp_dei: u8,
    pub l2_vlan_id: u16,
    pub l2_ether_type: u16,
    pub l3_dscp: u8,
    pub l3_protocol: u8,
    pub l3_source_ip: u32,
    pub l3_dest_ip: u32,
    pub l4_source_port: u16,
    pub l4_dest_port: u16,
}

//
// struct dpsw_acl_key - ACL key
// @match: Match fields
// @mask: Mask: b'1 - valid, b'0 don't care
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_acl_key {
    pub match: dpsw_acl_fields,
    pub mask: dpsw_acl_fields,
}

//
// enum dpsw_acl_action - action to be run on the ACL rule match
// @DPSW_ACL_ACTION_DROP: Drop frame
// @DPSW_ACL_ACTION_REDIRECT: Redirect to certain port
// @DPSW_ACL_ACTION_ACCEPT: Accept frame
// @DPSW_ACL_ACTION_REDIRECT_TO_CTRL_IF: Redirect to control interface
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_acl_action {
    DPSW_ACL_ACTION_DROP,
    DPSW_ACL_ACTION_REDIRECT,
    DPSW_ACL_ACTION_ACCEPT,
    DPSW_ACL_ACTION_REDIRECT_TO_CTRL_IF
}

//
// struct dpsw_acl_result - ACL action
// @action: Action should be taken when	ACL entry hit
// @if_id:  Interface IDs to redirect frame. Valid only if redirect selected for
// action
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_acl_result {
    pub action: dpsw_acl_action,
    pub if_id: u16,
}

//
// struct dpsw_acl_entry_cfg - ACL entry
// @key_iova: I/O virtual address of DMA-able memory filled with key after call
// to dpsw_acl_prepare_entry_cfg()
// @result: Required action when entry hit occurs
// @precedence: Precedence inside ACL 0 is lowest; This priority can not change
// during the lifetime of a Policy. It is user responsibility to
// space the priorities according to consequent rule additions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_acl_entry_cfg {
    pub key_iova: u64,
    pub result: dpsw_acl_result,
    pub precedence: c_int,
}

//
// enum dpsw_reflection_filter - Filter type for frames to be reflected
// @DPSW_REFLECTION_FILTER_INGRESS_ALL: Reflect all frames
// @DPSW_REFLECTION_FILTER_INGRESS_VLAN: Reflect only frames that belong to
// the particular VLAN defined by vid parameter
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpsw_reflection_filter {
    DPSW_REFLECTION_FILTER_INGRESS_ALL = 0,
    DPSW_REFLECTION_FILTER_INGRESS_VLAN = 1
}

//
// struct dpsw_reflection_cfg - Structure representing the mirroring config
// @filter: Filter type for frames to be mirrored
// @vlan_id: VLAN ID to mirror; valid only when the type is DPSW_INGRESS_VLAN
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_reflection_cfg {
    pub filter: dpsw_reflection_filter,
    pub vlan_id: u16,
}

// Link Aggregation Group configuration
pub const DPSW_LAG_SET_PHASE_APPLY: c_int = 0;
pub const DPSW_LAG_SET_PHASE_CHECK: c_int = 1;
//
// struct dpsw_lag_cfg - Configuration structure for a LAG group
// @group_id: Link aggregation group ID. Valid values are in the
// [1, DPSW_MAX_LAG_IFS] range.
// @num_ifs: Number of interfaces in this LAG group, valid range is
// [0, DPSW_MAX_LAG_IFS].
// @if_id: Array containing the interface IDs of the ports part of a LAG group
// @phase: Use DPSW_LAG_SET_PHASE_APPLY for LAG configuration processing or
// DPSW_LAG_SET_PHASE_CHECK for LAG configuration validation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpsw_lag_cfg {
    pub group_id: u8,
    pub num_ifs: u8,
    pub if_id: [u8; DPSW_MAX_LAG_IFS],
    pub phase: u8,
}
