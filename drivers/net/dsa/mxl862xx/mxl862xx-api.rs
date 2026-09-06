//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/mxl862xx/mxl862xx-api.h
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


// SPDX-License-Identifier: GPL-2.0-or-later

//
// struct mdio_relay_data - relayed access to the switch internal MDIO bus
// @data: data to be read or written
// @phy: PHY index
// @mmd: MMD device
// @reg: register index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdio_relay_data {
    pub data: __le16,
    pub phy: u8,
    pub mmd: u8,
    pub reg: __le16,
    pub __packed: },
//
// struct mxl862xx_register_mod - Register access parameter to directly
// modify internal registers
// @addr: Register address offset for modification
// @data: Value to write to the register address
// @mask: Mask of bits to be modified (1 to modify, 0 to ignore)
//
// Used for direct register modification operations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_register_mod {
    pub addr: __le16,
    pub data: __le16,
    pub mask: __le16,
    pub __packed: },
//
// enum mxl862xx_mac_table_filter - Source/Destination MAC address filtering
//
// @MXL862XX_MAC_FILTER_NONE: no filter
// @MXL862XX_MAC_FILTER_SRC: source address filter
// @MXL862XX_MAC_FILTER_DEST: destination address filter
// @MXL862XX_MAC_FILTER_BOTH: both source and destination filter
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_mac_table_filter {
    MXL862XX_MAC_FILTER_NONE = 0,
    MXL862XX_MAC_FILTER_SRC = BIT(0),
    MXL862XX_MAC_FILTER_DEST = BIT(1),
    MXL862XX_MAC_FILTER_BOTH = BIT(0) | BIT(1),
}

// Set in port_id to use port_map[] as a portmap bitmap instead of a single
// port ID. When clear, port_id selects one port; when set, the firmware
// ignores the lower bits of port_id and writes port_map[] directly into
// the PCE bridge port map.
//

//
// struct mxl862xx_mac_table_add - MAC Table Entry to be added
// @fid: Filtering Identifier (FID) (not supported by all switches)
// @port_id: Ethernet Port number
// @port_map: Bridge Port Map
// @sub_if_id: Sub-Interface Identifier Destination
// @age_timer: Aging Time in seconds
// @vlan_id: STAG VLAN Id
// @static_entry: Static Entry (value will be aged out if not set to static)
// @traffic_class: Egress queue traffic class
// @mac: MAC Address to add to the table
// @filter_flag: See &enum mxl862xx_mac_table_filter
// @igmp_controlled: Packet is marked as IGMP controlled if destination MAC
// address matches MAC in this entry
// @associated_mac: Associated Mac address
// @tci: TCI for B-Step
// Bit [0:11] - VLAN ID
// Bit [12] - VLAN CFI/DEI
// Bit [13:15] - VLAN PRI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_mac_table_add {
    pub fid: __le16,
    pub port_id: __le32,
    pub port_map: [__le16; 8],
    pub sub_if_id: __le16,
    pub age_timer: __le32,
    pub vlan_id: __le16,
    pub static_entry: u8,
    pub traffic_class: u8,
    pub mac: [u8; ETH_ALEN],
    pub filter_flag: u8,
    pub igmp_controlled: u8,
    pub associated_mac: [u8; ETH_ALEN],
    pub tci: __le16,
    pub __packed: },
//
// struct mxl862xx_mac_table_remove - MAC Table Entry to be removed
// @fid: Filtering Identifier (FID)
// @mac: MAC Address to be removed from the table.
// @filter_flag: See &enum mxl862xx_mac_table_filter
// @tci: TCI for B-Step
// Bit [0:11] - VLAN ID
// Bit [12] - VLAN CFI/DEI
// Bit [13:15] - VLAN PRI
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_mac_table_remove {
    pub fid: __le16,
    pub mac: [u8; ETH_ALEN],
    pub filter_flag: u8,
    pub tci: __le16,
    pub __packed: },
//
// struct mxl862xx_mac_table_read - MAC Table Entry to be read
// @initial: Restart the get operation from the beginning of the table
// @last: Indicates that the read operation returned last entry
// @fid: Get the MAC table entry belonging to the given Filtering Identifier
// @port_id: The Bridge Port ID
// @port_map: Bridge Port Map
// @age_timer: Aging Time
// @vlan_id: STAG VLAN Id
// @static_entry: Indicates if this is a Static Entry
// @sub_if_id: Sub-Interface Identifier Destination
// @mac: MAC Address. Filled out by the switch API implementation.
// @filter_flag: See &enum mxl862xx_mac_table_filter
// @igmp_controlled: Packet is marked as IGMP controlled if destination MAC
// address matches the MAC in this entry
// @entry_changed: Indicate if the Entry has Changed
// @associated_mac: Associated MAC address
// @hit_status: MAC Table Hit Status Update
// @tci: TCI for B-Step
// Bit [0:11] - VLAN ID
// Bit [12] - VLAN CFI/DEI
// Bit [13:15] - VLAN PRI
// @first_bridge_port_id: The port this MAC address has first been learned.
// This is used for loop detection.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_mac_table_read {
    pub initial: u8,
    pub last: u8,
    pub fid: __le16,
    pub port_id: __le32,
    pub port_map: [__le16; 8],
    pub age_timer: __le32,
    pub vlan_id: __le16,
    pub static_entry: u8,
    pub sub_if_id: __le16,
    pub mac: [u8; ETH_ALEN],
    pub filter_flag: u8,
    pub igmp_controlled: u8,
    pub entry_changed: u8,
    pub associated_mac: [u8; ETH_ALEN],
    pub hit_status: u8,
    pub tci: __le16,
    pub first_bridge_port_id: __le16,
    pub __packed: },
//
// struct mxl862xx_mac_table_query - MAC Table Entry key-based lookup
// @mac: MAC Address to search for (input)
// @fid: Filtering Identifier (input)
// @found: Set by firmware: 1 if entry was found, 0 if not
// @port_id: Bridge Port ID (output; MSB set if portmap mode)
// @port_map: Bridge Port Map (output; valid for static entries)
// @sub_if_id: Sub-Interface Identifier Destination
// @age_timer: Aging Time
// @vlan_id: STAG VLAN Id
// @static_entry: Indicates if this is a Static Entry
// @filter_flag: See &enum mxl862xx_mac_table_filter (input+output)
// @igmp_controlled: IGMP controlled flag
// @entry_changed: Entry changed flag
// @associated_mac: Associated MAC address
// @hit_status: MAC Table Hit Status Update
// @tci: TCI (VLAN ID + CFI/DEI + PRI) (input)
// @first_bridge_port_id: First learned bridge port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_mac_table_query {
    pub mac: [u8; ETH_ALEN],
    pub fid: __le16,
    pub found: u8,
    pub port_id: __le32,
    pub port_map: [__le16; 8],
    pub sub_if_id: __le16,
    pub age_timer: __le32,
    pub vlan_id: __le16,
    pub static_entry: u8,
    pub filter_flag: u8,
    pub igmp_controlled: u8,
    pub entry_changed: u8,
    pub associated_mac: [u8; ETH_ALEN],
    pub hit_status: u8,
    pub tci: __le16,
    pub first_bridge_port_id: __le16,
    pub __packed: },
//
// enum mxl862xx_mac_clear_type - MAC table clear type
// @MXL862XX_MAC_CLEAR_PHY_PORT: clear dynamic entries based on port_id
// @MXL862XX_MAC_CLEAR_DYNAMIC: clear all dynamic entries
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_mac_clear_type {
    MXL862XX_MAC_CLEAR_PHY_PORT = 0,
    MXL862XX_MAC_CLEAR_DYNAMIC,
}

//
// struct mxl862xx_mac_table_clear - MAC table clear
// @type: see &enum mxl862xx_mac_clear_type
// @port_id: physical port id
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_mac_table_clear {
    pub type: u8,
    pub port_id: u8,
    pub __packed: },
//
// enum mxl862xx_age_timer - Aging Timer Value.
// @MXL862XX_AGETIMER_1_SEC: 1 second aging time
// @MXL862XX_AGETIMER_10_SEC: 10 seconds aging time
// @MXL862XX_AGETIMER_300_SEC: 300 seconds aging time
// @MXL862XX_AGETIMER_1_HOUR: 1 hour aging time
// @MXL862XX_AGETIMER_1_DAY: 24 hours aging time
// @MXL862XX_AGETIMER_CUSTOM: Custom aging time in seconds
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_age_timer {
    MXL862XX_AGETIMER_1_SEC = 1,
    MXL862XX_AGETIMER_10_SEC,
    MXL862XX_AGETIMER_300_SEC,
    MXL862XX_AGETIMER_1_HOUR,
    MXL862XX_AGETIMER_1_DAY,
    MXL862XX_AGETIMER_CUSTOM,
}

//
// struct mxl862xx_bridge_alloc - Bridge Allocation
// @bridge_id: If the bridge allocation is successful, a valid ID will be
// returned in this field. Otherwise, INVALID_HANDLE is
// returned. For bridge free, this field should contain a
// valid ID returned by the bridge allocation. ID 0 is not
// used for historic reasons.
//
// Used by MXL862XX_BRIDGE_ALLOC and MXL862XX_BRIDGE_FREE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_bridge_alloc {
    pub bridge_id: __le16,
}

//
// enum mxl862xx_bridge_config_mask - Bridge configuration mask
// @MXL862XX_BRIDGE_CONFIG_MASK_MAC_LEARNING_LIMIT:
// Mask for mac_learning_limit_enable and mac_learning_limit.
// @MXL862XX_BRIDGE_CONFIG_MASK_MAC_LEARNED_COUNT:
// Mask for mac_learning_count
// @MXL862XX_BRIDGE_CONFIG_MASK_MAC_DISCARD_COUNT:
// Mask for learning_discard_event
// @MXL862XX_BRIDGE_CONFIG_MASK_SUB_METER:
// Mask for sub_metering_enable and traffic_sub_meter_id
// @MXL862XX_BRIDGE_CONFIG_MASK_FORWARDING_MODE:
// Mask for forward_broadcast, forward_unknown_multicast_ip,
// forward_unknown_multicast_non_ip and forward_unknown_unicast.
// @MXL862XX_BRIDGE_CONFIG_MASK_ALL: Enable all
// @MXL862XX_BRIDGE_CONFIG_MASK_FORCE: Bypass any check for debug purpose
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_bridge_config_mask {
    MXL862XX_BRIDGE_CONFIG_MASK_MAC_LEARNING_LIMIT = BIT(0),
    MXL862XX_BRIDGE_CONFIG_MASK_MAC_LEARNED_COUNT = BIT(1),
    MXL862XX_BRIDGE_CONFIG_MASK_MAC_DISCARD_COUNT = BIT(2),
    MXL862XX_BRIDGE_CONFIG_MASK_SUB_METER = BIT(3),
    MXL862XX_BRIDGE_CONFIG_MASK_FORWARDING_MODE = BIT(4),
    MXL862XX_BRIDGE_CONFIG_MASK_ALL = 0x7FFFFFFF,
    MXL862XX_BRIDGE_CONFIG_MASK_FORCE = BIT(31)
}

//
// enum mxl862xx_bridge_port_egress_meter - Meters for egress traffic type
// @MXL862XX_BRIDGE_PORT_EGRESS_METER_BROADCAST:
// Index of broadcast traffic meter
// @MXL862XX_BRIDGE_PORT_EGRESS_METER_MULTICAST:
// Index of known multicast traffic meter
// @MXL862XX_BRIDGE_PORT_EGRESS_METER_UNKNOWN_MC_IP:
// Index of unknown multicast IP traffic meter
// @MXL862XX_BRIDGE_PORT_EGRESS_METER_UNKNOWN_MC_NON_IP:
// Index of unknown multicast non-IP traffic meter
// @MXL862XX_BRIDGE_PORT_EGRESS_METER_UNKNOWN_UC:
// Index of unknown unicast traffic meter
// @MXL862XX_BRIDGE_PORT_EGRESS_METER_OTHERS:
// Index of traffic meter for other types
// @MXL862XX_BRIDGE_PORT_EGRESS_METER_MAX: Number of index
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_bridge_port_egress_meter {
    MXL862XX_BRIDGE_PORT_EGRESS_METER_BROADCAST = 0,
    MXL862XX_BRIDGE_PORT_EGRESS_METER_MULTICAST,
    MXL862XX_BRIDGE_PORT_EGRESS_METER_UNKNOWN_MC_IP,
    MXL862XX_BRIDGE_PORT_EGRESS_METER_UNKNOWN_MC_NON_IP,
    MXL862XX_BRIDGE_PORT_EGRESS_METER_UNKNOWN_UC,
    MXL862XX_BRIDGE_PORT_EGRESS_METER_OTHERS,
    MXL862XX_BRIDGE_PORT_EGRESS_METER_MAX,
}

//
// struct mxl862xx_qos_meter_cfg - Rate meter configuration
// @enable: Enable/disable meter
// @meter_id: Meter ID (assigned by firmware on alloc)
// @meter_name: Meter name string
// @meter_type: Meter algorithm type (srTCM = 0, trTCM = 1)
// @cbs: Committed Burst Size (in bytes)
// @res1: Reserved
// @ebs: Excess Burst Size (in bytes)
// @res2: Reserved
// @rate: Committed Information Rate (in kbit/s)
// @pi_rate: Peak Information Rate (in kbit/s)
// @colour_blind_mode: Colour-blind mode enable
// @pkt_mode: Packet mode enable
// @local_overhd: Local overhead accounting enable
// @local_overhd_val: Local overhead accounting value
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_qos_meter_cfg {
    pub enable: u8,
    pub meter_id: __le16,
    pub meter_name: [c_char; 32],
    pub meter_type: __le32,
    pub cbs: __le32,
    pub res1: __le32,
    pub ebs: __le32,
    pub res2: __le32,
    pub rate: __le32,
    pub pi_rate: __le32,
    pub colour_blind_mode: u8,
    pub pkt_mode: u8,
    pub local_overhd: u8,
    pub local_overhd_val: __le16,
    pub __packed: },
//
// enum mxl862xx_bridge_forward_mode - Bridge forwarding type of packet
// @MXL862XX_BRIDGE_FORWARD_FLOOD: Packet is flooded to port members of
// ingress bridge port
// @MXL862XX_BRIDGE_FORWARD_DISCARD: Packet is discarded
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_bridge_forward_mode {
    MXL862XX_BRIDGE_FORWARD_FLOOD = 0,
    MXL862XX_BRIDGE_FORWARD_DISCARD,
}

//
// struct mxl862xx_bridge_config - Bridge Configuration
// @bridge_id: Bridge ID (FID)
// @mask: See &enum mxl862xx_bridge_config_mask
// @mac_learning_limit_enable: Enable MAC learning limitation
// @mac_learning_limit: Max number of MAC addresses that can be learned in
// this bridge (all bridge ports)
// @mac_learning_count: Number of MAC addresses learned from this bridge
// @learning_discard_event: Number of learning discard events due to
// hardware resource not available
// @sub_metering_enable: Traffic metering on type of traffic (such as
// broadcast, multicast, unknown unicast, etc) applies
// @traffic_sub_meter_id: Meter for bridge process with specific type (such
// as broadcast, multicast, unknown unicast, etc)
// @forward_broadcast: Forwarding mode of broadcast traffic. See
// &enum mxl862xx_bridge_forward_mode
// @forward_unknown_multicast_ip: Forwarding mode of unknown multicast IP
// traffic.
// See &enum mxl862xx_bridge_forward_mode
// @forward_unknown_multicast_non_ip: Forwarding mode of unknown multicast
// non-IP traffic.
// See &enum mxl862xx_bridge_forward_mode
// @forward_unknown_unicast: Forwarding mode of unknown unicast traffic. See
// &enum mxl862xx_bridge_forward_mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_bridge_config {
    pub bridge_id: __le16,
    pub /: *mut *mut __le32 mask; / enum mxl862xx_bridge_config_mask,
    pub mac_learning_limit_enable: u8,
    pub mac_learning_limit: __le16,
    pub mac_learning_count: __le16,
    pub learning_discard_event: __le32,
    pub sub_metering_enable: [u8; MXL862XX_BRIDGE_PORT_EGRESS_METER_MAX],
    pub traffic_sub_meter_id: [__le16; MXL862XX_BRIDGE_PORT_EGRESS_METER_MAX],
    pub /: *mut *mut __le32 forward_broadcast; / enum mxl862xx_bridge_forward_mode,
    pub /: *mut *mut __le32 forward_unknown_multicast_ip; / enum mxl862xx_bridge_forward_mode,
    pub /: *mut *mut __le32 forward_unknown_multicast_non_ip; / enum mxl862xx_bridge_forward_mode,
    pub /: *mut *mut __le32 forward_unknown_unicast; / enum mxl862xx_bridge_forward_mode,
    pub __packed: },
//
// struct mxl862xx_bridge_port_alloc - Bridge Port Allocation
// @bridge_port_id: If the bridge port allocation is successful, a valid ID
// will be returned in this field. Otherwise, INVALID_HANDLE
// is returned. For bridge port free, this field should
// contain a valid ID returned by the bridge port allocation.
//
// Used by MXL862XX_BRIDGE_PORT_ALLOC and MXL862XX_BRIDGE_PORT_FREE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_bridge_port_alloc {
    pub bridge_port_id: __le16,
}

//
// enum mxl862xx_bridge_port_config_mask - Bridge Port configuration mask
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_BRIDGE_ID:
// Mask for bridge_id
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_VLAN:
// Mask for ingress_extended_vlan_enable,
// ingress_extended_vlan_block_id and ingress_extended_vlan_block_size
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_VLAN:
// Mask for egress_extended_vlan_enable, egress_extended_vlan_block_id
// and egress_extended_vlan_block_size
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_MARKING:
// Mask for ingress_marking_mode
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_REMARKING:
// Mask for egress_remarking_mode
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_METER:
// Mask for ingress_metering_enable and ingress_traffic_meter_id
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_SUB_METER:
// Mask for egress_sub_metering_enable and egress_traffic_sub_meter_id
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_CTP_MAPPING:
// Mask for dest_logical_port_id, pmapper_enable, dest_sub_if_id_group,
// pmapper_mapping_mode, pmapper_id_valid and pmapper
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_BRIDGE_PORT_MAP:
// Mask for bridge_port_map
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_DEST_IP_LOOKUP:
// Mask for mc_dest_ip_lookup_disable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_SRC_IP_LOOKUP:
// Mask for mc_src_ip_lookup_enable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_DEST_MAC_LOOKUP:
// Mask for dest_mac_lookup_disable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_SRC_MAC_LEARNING:
// Mask for src_mac_learning_disable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_MAC_SPOOFING:
// Mask for mac_spoofing_detect_enable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_PORT_LOCK:
// Mask for port_lock_enable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_MAC_LEARNING_LIMIT:
// Mask for mac_learning_limit_enable and mac_learning_limit
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_MAC_LEARNED_COUNT:
// Mask for mac_learning_count
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_VLAN_FILTER:
// Mask for ingress_vlan_filter_enable, ingress_vlan_filter_block_id
// and ingress_vlan_filter_block_size
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_VLAN_FILTER1:
// Mask for bypass_egress_vlan_filter1, egress_vlan_filter1enable,
// egress_vlan_filter1block_id and egress_vlan_filter1block_size
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_VLAN_FILTER2:
// Mask for egress_vlan_filter2enable, egress_vlan_filter2block_id and
// egress_vlan_filter2block_size
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_VLAN_BASED_MAC_LEARNING:
// Mask for vlan_tag_selection, vlan_src_mac_priority_enable,
// vlan_src_mac_dei_enable, vlan_src_mac_vid_enable,
// vlan_dst_mac_priority_enable, vlan_dst_mac_dei_enable and
// vlan_dst_mac_vid_enable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_VLAN_BASED_MULTICAST_LOOKUP:
// Mask for vlan_multicast_priority_enable,
// vlan_multicast_dei_enable and vlan_multicast_vid_enable
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_LOOP_VIOLATION_COUNTER:
// Mask for loop_violation_count
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_ALL: Enable all
// @MXL862XX_BRIDGE_PORT_CONFIG_MASK_FORCE: Bypass any check for debug purpose
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_bridge_port_config_mask {
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_BRIDGE_ID = BIT(0),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_VLAN = BIT(1),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_VLAN = BIT(2),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_MARKING = BIT(3),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_REMARKING = BIT(4),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_METER = BIT(5),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_SUB_METER = BIT(6),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_CTP_MAPPING = BIT(7),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_BRIDGE_PORT_MAP = BIT(8),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_DEST_IP_LOOKUP = BIT(9),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_SRC_IP_LOOKUP = BIT(10),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_DEST_MAC_LOOKUP = BIT(11),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_MC_SRC_MAC_LEARNING = BIT(12),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_MAC_SPOOFING = BIT(13),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_PORT_LOCK = BIT(14),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_MAC_LEARNING_LIMIT = BIT(15),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_MAC_LEARNED_COUNT = BIT(16),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_INGRESS_VLAN_FILTER = BIT(17),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_VLAN_FILTER1 = BIT(18),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_EGRESS_VLAN_FILTER2 = BIT(19),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_VLAN_BASED_MAC_LEARNING = BIT(20),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_VLAN_BASED_MULTICAST_LOOKUP = BIT(21),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_LOOP_VIOLATION_COUNTER = BIT(22),
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_ALL = 0x7FFFFFFF,
    MXL862XX_BRIDGE_PORT_CONFIG_MASK_FORCE = BIT(31)
}

//
// enum mxl862xx_color_marking_mode - Color Marking Mode
// @MXL862XX_MARKING_ALL_GREEN: mark packets (except critical) to green
// @MXL862XX_MARKING_INTERNAL_MARKING: do not change color and priority
// @MXL862XX_MARKING_DEI: DEI mark mode
// @MXL862XX_MARKING_PCP_8P0D: PCP 8P0D mark mode
// @MXL862XX_MARKING_PCP_7P1D: PCP 7P1D mark mode
// @MXL862XX_MARKING_PCP_6P2D: PCP 6P2D mark mode
// @MXL862XX_MARKING_PCP_5P3D: PCP 5P3D mark mode
// @MXL862XX_MARKING_DSCP_AF: DSCP AF class
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_color_marking_mode {
    MXL862XX_MARKING_ALL_GREEN = 0,
    MXL862XX_MARKING_INTERNAL_MARKING,
    MXL862XX_MARKING_DEI,
    MXL862XX_MARKING_PCP_8P0D,
    MXL862XX_MARKING_PCP_7P1D,
    MXL862XX_MARKING_PCP_6P2D,
    MXL862XX_MARKING_PCP_5P3D,
    MXL862XX_MARKING_DSCP_AF,
}

//
// enum mxl862xx_color_remarking_mode - Color Remarking Mode
// @MXL862XX_REMARKING_NONE: values from last process stage
// @MXL862XX_REMARKING_DEI: DEI mark mode
// @MXL862XX_REMARKING_PCP_8P0D: PCP 8P0D mark mode
// @MXL862XX_REMARKING_PCP_7P1D: PCP 7P1D mark mode
// @MXL862XX_REMARKING_PCP_6P2D: PCP 6P2D mark mode
// @MXL862XX_REMARKING_PCP_5P3D: PCP 5P3D mark mode
// @MXL862XX_REMARKING_DSCP_AF: DSCP AF class
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_color_remarking_mode {
    MXL862XX_REMARKING_NONE = 0,
    MXL862XX_REMARKING_DEI = 2,
    MXL862XX_REMARKING_PCP_8P0D,
    MXL862XX_REMARKING_PCP_7P1D,
    MXL862XX_REMARKING_PCP_6P2D,
    MXL862XX_REMARKING_PCP_5P3D,
    MXL862XX_REMARKING_DSCP_AF,
}

//
// enum mxl862xx_pmapper_mapping_mode - P-mapper Mapping Mode
// @MXL862XX_PMAPPER_MAPPING_PCP: Use PCP for VLAN tagged packets to derive
// sub interface ID group
// @MXL862XX_PMAPPER_MAPPING_LAG: Use LAG Index for Pmapper access
// regardless of IP and VLAN packet
// @MXL862XX_PMAPPER_MAPPING_DSCP: Use DSCP for VLAN tagged IP packets to
// derive sub interface ID group
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_pmapper_mapping_mode {
    MXL862XX_PMAPPER_MAPPING_PCP = 0,
    MXL862XX_PMAPPER_MAPPING_LAG,
    MXL862XX_PMAPPER_MAPPING_DSCP,
}

//
// struct mxl862xx_pmapper - P-mapper Configuration
// @pmapper_id: Index of P-mapper (0-31)
// @dest_sub_if_id_group: Sub interface ID group. Entry 0 is for non-IP and
// non-VLAN tagged packets.
// Entries 1-8 are PCP mapping entries for VLAN tagged
// packets.
// Entries 9-72 are DSCP or LAG mapping entries.
//
// Used by CTP port config and bridge port config. In case of LAG, it is
// user's responsibility to provide the mapped entries in given P-mapper
// table. In other modes the entries are auto mapped from input packet.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_pmapper {
    pub pmapper_id: __le16,
    pub dest_sub_if_id_group: [u8; 73],
    pub __packed: },
//
// struct mxl862xx_bridge_port_config - Bridge Port Configuration
// @bridge_port_id: Bridge Port ID allocated by bridge port allocation
// @mask: See &enum mxl862xx_bridge_port_config_mask
// @bridge_id: Bridge ID (FID) to which this bridge port is associated
// @ingress_extended_vlan_enable: Enable extended VLAN processing for
// ingress traffic
// @ingress_extended_vlan_block_id: Extended VLAN block allocated for
// ingress traffic
// @ingress_extended_vlan_block_size: Extended VLAN block size for ingress
// traffic
// @egress_extended_vlan_enable: Enable extended VLAN processing for egress
// traffic
// @egress_extended_vlan_block_id: Extended VLAN block allocated for egress
// traffic
// @egress_extended_vlan_block_size: Extended VLAN block size for egress
// traffic
// @ingress_marking_mode: Ingress color marking mode. See
// &enum mxl862xx_color_marking_mode
// @egress_remarking_mode: Color remarking for egress traffic. See
// &enum mxl862xx_color_remarking_mode
// @ingress_metering_enable: Traffic metering on ingress traffic applies
// @ingress_traffic_meter_id: Meter for ingress Bridge Port process
// @egress_sub_metering_enable: Traffic metering on various types of egress
// traffic
// @egress_traffic_sub_meter_id: Meter for egress Bridge Port process with
// specific type
// @dest_logical_port_id: Destination logical port
// @pmapper_enable: Enable P-mapper
// @dest_sub_if_id_group: Destination sub interface ID group when
// pmapper_enable is false
// @pmapper_mapping_mode: P-mapper mapping mode. See
// &enum mxl862xx_pmapper_mapping_mode
// @pmapper_id_valid: When true, P-mapper is re-used; when false,
// allocation is handled by API
// @pmapper: P-mapper configuration used when pmapper_enable is true
// @bridge_port_map: Port map defining broadcast domain. Each bit
// represents one bridge port. Bridge port ID is
// index * 16 + bit offset.
// @mc_dest_ip_lookup_disable: Disable multicast IP destination table
// lookup
// @mc_src_ip_lookup_enable: Enable multicast IP source table lookup
// @dest_mac_lookup_disable: Disable destination MAC lookup; packet treated
// as unknown
// @src_mac_learning_disable: Disable source MAC address learning
// @mac_spoofing_detect_enable: Enable MAC spoofing detection
// @port_lock_enable: Enable port locking
// @mac_learning_limit_enable: Enable MAC learning limitation
// @mac_learning_limit: Maximum number of MAC addresses that can be learned
// from this bridge port
// @loop_violation_count: Number of loop violation events from this bridge
// port
// @mac_learning_count: Number of MAC addresses learned from this bridge
// port
// @ingress_vlan_filter_enable: Enable ingress VLAN filter
// @ingress_vlan_filter_block_id: VLAN filter block of ingress traffic
// @ingress_vlan_filter_block_size: VLAN filter block size for ingress
// traffic
// @bypass_egress_vlan_filter1: For ingress traffic, bypass VLAN filter 1
// at egress bridge port processing
// @egress_vlan_filter1enable: Enable egress VLAN filter 1
// @egress_vlan_filter1block_id: VLAN filter block 1 of egress traffic
// @egress_vlan_filter1block_size: VLAN filter block 1 size
// @egress_vlan_filter2enable: Enable egress VLAN filter 2
// @egress_vlan_filter2block_id: VLAN filter block 2 of egress traffic
// @egress_vlan_filter2block_size: VLAN filter block 2 size
// @vlan_tag_selection: VLAN tag selection for MAC address/multicast
// learning, lookup and filtering.
// 0 - Intermediate outer VLAN tag is used.
// 1 - Original outer VLAN tag is used.
// @vlan_src_mac_priority_enable: Enable VLAN Priority field for source MAC
// learning and filtering
// @vlan_src_mac_dei_enable: Enable VLAN DEI/CFI field for source MAC
// learning and filtering
// @vlan_src_mac_vid_enable: Enable VLAN ID field for source MAC learning
// and filtering
// @vlan_dst_mac_priority_enable: Enable VLAN Priority field for destination
// MAC lookup and filtering
// @vlan_dst_mac_dei_enable: Enable VLAN CFI/DEI field for destination MAC
// lookup and filtering
// @vlan_dst_mac_vid_enable: Enable VLAN ID field for destination MAC lookup
// and filtering
// @vlan_multicast_priority_enable: Enable VLAN Priority field for IP
// multicast lookup
// @vlan_multicast_dei_enable: Enable VLAN CFI/DEI field for IP multicast
// lookup
// @vlan_multicast_vid_enable: Enable VLAN ID field for IP multicast lookup
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_bridge_port_config {
    pub bridge_port_id: __le16,
    pub /: *mut *mut __le32 mask; / enum mxl862xx_bridge_port_config_mask,
    pub bridge_id: __le16,
    pub ingress_extended_vlan_enable: u8,
    pub ingress_extended_vlan_block_id: __le16,
    pub ingress_extended_vlan_block_size: __le16,
    pub egress_extended_vlan_enable: u8,
    pub egress_extended_vlan_block_id: __le16,
    pub egress_extended_vlan_block_size: __le16,
    pub /: *mut *mut __le32 ingress_marking_mode; / enum mxl862xx_color_marking_mode,
    pub /: *mut *mut __le32 egress_remarking_mode; / enum mxl862xx_color_remarking_mode,
    pub ingress_metering_enable: u8,
    pub ingress_traffic_meter_id: __le16,
    pub egress_sub_metering_enable: [u8; MXL862XX_BRIDGE_PORT_EGRESS_METER_MAX],
    pub egress_traffic_sub_meter_id: [__le16; MXL862XX_BRIDGE_PORT_EGRESS_METER_MAX],
    pub dest_logical_port_id: u8,
    pub pmapper_enable: u8,
    pub dest_sub_if_id_group: __le16,
    pub /: *mut *mut __le32 pmapper_mapping_mode; / enum mxl862xx_pmapper_mapping_mode,
    pub pmapper_id_valid: u8,
    pub pmapper: mxl862xx_pmapper,
    pub bridge_port_map: [__le16; 8],
    pub mc_dest_ip_lookup_disable: u8,
    pub mc_src_ip_lookup_enable: u8,
    pub dest_mac_lookup_disable: u8,
    pub src_mac_learning_disable: u8,
    pub mac_spoofing_detect_enable: u8,
    pub port_lock_enable: u8,
    pub mac_learning_limit_enable: u8,
    pub mac_learning_limit: __le16,
    pub loop_violation_count: __le16,
    pub mac_learning_count: __le16,
    pub ingress_vlan_filter_enable: u8,
    pub ingress_vlan_filter_block_id: __le16,
    pub ingress_vlan_filter_block_size: __le16,
    pub bypass_egress_vlan_filter1: u8,
    pub egress_vlan_filter1enable: u8,
    pub egress_vlan_filter1block_id: __le16,
    pub egress_vlan_filter1block_size: __le16,
    pub egress_vlan_filter2enable: u8,
    pub egress_vlan_filter2block_id: __le16,
    pub egress_vlan_filter2block_size: __le16,
    pub vlan_tag_selection: u8,
    pub vlan_src_mac_priority_enable: u8,
    pub vlan_src_mac_dei_enable: u8,
    pub vlan_src_mac_vid_enable: u8,
    pub vlan_dst_mac_priority_enable: u8,
    pub vlan_dst_mac_dei_enable: u8,
    pub vlan_dst_mac_vid_enable: u8,
    pub vlan_multicast_priority_enable: u8,
    pub vlan_multicast_dei_enable: u8,
    pub vlan_multicast_vid_enable: u8,
    pub __packed: },
//
// struct mxl862xx_cfg -  Global Switch configuration Attributes
// @mac_table_age_timer: See &enum mxl862xx_age_timer
// @age_timer: Custom MAC table aging timer in seconds
// @max_packet_len: Maximum Ethernet packet length
// @learning_limit_action: Automatic MAC address table learning limitation
// consecutive action
// @mac_locking_action: Accept or discard MAC port locking violation
// packets
// @mac_spoofing_action: Accept or discard MAC spoofing and port MAC locking
// violation packets
// @pause_mac_mode_src: Pause frame MAC source address mode
// @pause_mac_src: Pause frame MAC source address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_cfg {
    pub /: *mut *mut __le32 mac_table_age_timer; / enum mxl862xx_age_timer,
    pub age_timer: __le32,
    pub max_packet_len: __le16,
    pub learning_limit_action: u8,
    pub mac_locking_action: u8,
    pub mac_spoofing_action: u8,
    pub pause_mac_mode_src: u8,
    pub pause_mac_src: [u8; ETH_ALEN],
    pub __packed: },
//
// enum mxl862xx_extended_vlan_filter_type - Extended VLAN filter tag type
// @MXL862XX_EXTENDEDVLAN_FILTER_TYPE_NORMAL: Normal tagged
// @MXL862XX_EXTENDEDVLAN_FILTER_TYPE_NO_FILTER: No filter (wildcard)
// @MXL862XX_EXTENDEDVLAN_FILTER_TYPE_DEFAULT: Default entry
// @MXL862XX_EXTENDEDVLAN_FILTER_TYPE_NO_TAG: Untagged
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_filter_type {
    MXL862XX_EXTENDEDVLAN_FILTER_TYPE_NORMAL = 0,
    MXL862XX_EXTENDEDVLAN_FILTER_TYPE_NO_FILTER = 1,
    MXL862XX_EXTENDEDVLAN_FILTER_TYPE_DEFAULT = 2,
    MXL862XX_EXTENDEDVLAN_FILTER_TYPE_NO_TAG = 3,
}

//
// enum mxl862xx_extended_vlan_filter_tpid - Extended VLAN filter TPID
// @MXL862XX_EXTENDEDVLAN_FILTER_TPID_NO_FILTER: No TPID filter
// @MXL862XX_EXTENDEDVLAN_FILTER_TPID_8021Q: 802.1Q TPID
// @MXL862XX_EXTENDEDVLAN_FILTER_TPID_VTETYPE: VLAN type extension
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_filter_tpid {
    MXL862XX_EXTENDEDVLAN_FILTER_TPID_NO_FILTER = 0,
    MXL862XX_EXTENDEDVLAN_FILTER_TPID_8021Q = 1,
    MXL862XX_EXTENDEDVLAN_FILTER_TPID_VTETYPE = 2,
}

//
// enum mxl862xx_extended_vlan_filter_dei - Extended VLAN filter DEI
// @MXL862XX_EXTENDEDVLAN_FILTER_DEI_NO_FILTER: No DEI filter
// @MXL862XX_EXTENDEDVLAN_FILTER_DEI_0: DEI = 0
// @MXL862XX_EXTENDEDVLAN_FILTER_DEI_1: DEI = 1
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_filter_dei {
    MXL862XX_EXTENDEDVLAN_FILTER_DEI_NO_FILTER = 0,
    MXL862XX_EXTENDEDVLAN_FILTER_DEI_0 = 1,
    MXL862XX_EXTENDEDVLAN_FILTER_DEI_1 = 2,
}

//
// enum mxl862xx_extended_vlan_treatment_remove_tag - Tag removal action
// @MXL862XX_EXTENDEDVLAN_TREATMENT_NOT_REMOVE_TAG: Do not remove tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_REMOVE_1_TAG: Remove one tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_REMOVE_2_TAG: Remove two tags
// @MXL862XX_EXTENDEDVLAN_TREATMENT_DISCARD_UPSTREAM: Discard frame
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_treatment_remove_tag {
    MXL862XX_EXTENDEDVLAN_TREATMENT_NOT_REMOVE_TAG = 0,
    MXL862XX_EXTENDEDVLAN_TREATMENT_REMOVE_1_TAG = 1,
    MXL862XX_EXTENDEDVLAN_TREATMENT_REMOVE_2_TAG = 2,
    MXL862XX_EXTENDEDVLAN_TREATMENT_DISCARD_UPSTREAM = 3,
}

//
// enum mxl862xx_extended_vlan_treatment_priority - Treatment priority mode
// @MXL862XX_EXTENDEDVLAN_TREATMENT_PRIORITY_VAL: Use explicit value
// @MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_PRIORITY: Copy from inner tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_PRIORITY: Copy from outer tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_DSCP: Derive from DSCP
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_treatment_priority {
    MXL862XX_EXTENDEDVLAN_TREATMENT_PRIORITY_VAL = 0,
    MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_PRIORITY = 1,
    MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_PRIORITY = 2,
    MXL862XX_EXTENDEDVLAN_TREATMENT_DSCP = 3,
}

//
// enum mxl862xx_extended_vlan_treatment_vid - Treatment VID mode
// @MXL862XX_EXTENDEDVLAN_TREATMENT_VID_VAL: Use explicit VID value
// @MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_VID: Copy from inner tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_VID: Copy from outer tag
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_treatment_vid {
    MXL862XX_EXTENDEDVLAN_TREATMENT_VID_VAL = 0,
    MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_VID = 1,
    MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_VID = 2,
}

//
// enum mxl862xx_extended_vlan_treatment_tpid - Treatment TPID mode
// @MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_TPID: Copy from inner tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_TPID: Copy from outer tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_VTETYPE: Use VLAN type extension
// @MXL862XX_EXTENDEDVLAN_TREATMENT_8021Q: Use 802.1Q TPID
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_treatment_tpid {
    MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_TPID = 0,
    MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_TPID = 1,
    MXL862XX_EXTENDEDVLAN_TREATMENT_VTETYPE = 2,
    MXL862XX_EXTENDEDVLAN_TREATMENT_8021Q = 3,
}

//
// enum mxl862xx_extended_vlan_treatment_dei - Treatment DEI mode
// @MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_DEI: Copy from inner tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_DEI: Copy from outer tag
// @MXL862XX_EXTENDEDVLAN_TREATMENT_DEI_0: Set DEI to 0
// @MXL862XX_EXTENDEDVLAN_TREATMENT_DEI_1: Set DEI to 1
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_treatment_dei {
    MXL862XX_EXTENDEDVLAN_TREATMENT_INNER_DEI = 0,
    MXL862XX_EXTENDEDVLAN_TREATMENT_OUTER_DEI = 1,
    MXL862XX_EXTENDEDVLAN_TREATMENT_DEI_0 = 2,
    MXL862XX_EXTENDEDVLAN_TREATMENT_DEI_1 = 3,
}

//
// enum mxl862xx_extended_vlan_4_tpid_mode - 4-TPID mode selector
// @MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_1: VLAN TPID type 1
// @MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_2: VLAN TPID type 2
// @MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_3: VLAN TPID type 3
// @MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_4: VLAN TPID type 4
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_4_tpid_mode {
    MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_1 = 0,
    MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_2 = 1,
    MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_3 = 2,
    MXL862XX_EXTENDEDVLAN_TPID_VTETYPE_4 = 3,
}

//
// enum mxl862xx_extended_vlan_filter_ethertype - Filter EtherType match
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_NO_FILTER: No filter
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_IPOE: IPoE
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_PPPOE: PPPoE
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_ARP: ARP
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_IPV6IPOE: IPv6 IPoE
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_EAPOL: EAPOL
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_DHCPV4: DHCPv4
// @MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_DHCPV6: DHCPv6
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_extended_vlan_filter_ethertype {
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_NO_FILTER = 0,
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_IPOE = 1,
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_PPPOE = 2,
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_ARP = 3,
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_IPV6IPOE = 4,
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_EAPOL = 5,
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_DHCPV4 = 6,
    MXL862XX_EXTENDEDVLAN_FILTER_ETHERTYPE_DHCPV6 = 7,
}

//
// struct mxl862xx_extendedvlan_filter_vlan - Per-tag filter in Extended VLAN
// @type: Tag presence/type match (see &enum mxl862xx_extended_vlan_filter_type)
// @priority_enable: Enable PCP value matching
// @priority_val: PCP value to match
// @vid_enable: Enable VID matching
// @vid_val: VID value to match
// @tpid: TPID match mode (see &enum mxl862xx_extended_vlan_filter_tpid)
// @dei: DEI match mode (see &enum mxl862xx_extended_vlan_filter_dei)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_extendedvlan_filter_vlan {
    pub type: __le32,
    pub priority_enable: u8,
    pub priority_val: __le32,
    pub vid_enable: u8,
    pub vid_val: __le32,
    pub tpid: __le32,
    pub dei: __le32,
    pub __packed: },
//
// struct mxl862xx_extendedvlan_filter - Extended VLAN filter configuration
// @original_packet_filter_mode: If true, filter on original (pre-treatment)
// packet
// @filter_4_tpid_mode: 4-TPID mode (see &enum mxl862xx_extended_vlan_4_tpid_mode)
// @outer_vlan: Outer VLAN tag filter
// @inner_vlan: Inner VLAN tag filter
// @ether_type: EtherType filter (see
// &enum mxl862xx_extended_vlan_filter_ethertype)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_extendedvlan_filter {
    pub original_packet_filter_mode: u8,
    pub filter_4_tpid_mode: __le32,
    pub outer_vlan: mxl862xx_extendedvlan_filter_vlan,
    pub inner_vlan: mxl862xx_extendedvlan_filter_vlan,
    pub ether_type: __le32,
    pub __packed: },
//
// struct mxl862xx_extendedvlan_treatment_vlan - Per-tag treatment in
// Extended VLAN
// @priority_mode: Priority assignment mode
// (see &enum mxl862xx_extended_vlan_treatment_priority)
// @priority_val: Priority value (when mode is VAL)
// @vid_mode: VID assignment mode
// (see &enum mxl862xx_extended_vlan_treatment_vid)
// @vid_val: VID value (when mode is VAL)
// @tpid: TPID assignment mode
// (see &enum mxl862xx_extended_vlan_treatment_tpid)
// @dei: DEI assignment mode
// (see &enum mxl862xx_extended_vlan_treatment_dei)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_extendedvlan_treatment_vlan {
    pub priority_mode: __le32,
    pub priority_val: __le32,
    pub vid_mode: __le32,
    pub vid_val: __le32,
    pub tpid: __le32,
    pub dei: __le32,
    pub __packed: },
//
// struct mxl862xx_extendedvlan_treatment - Extended VLAN treatment
// @remove_tag: Tag removal action
// (see &enum mxl862xx_extended_vlan_treatment_remove_tag)
// @treatment_4_tpid_mode: 4-TPID treatment mode
// @add_outer_vlan: Add outer VLAN tag
// @outer_vlan: Outer VLAN tag treatment parameters
// @add_inner_vlan: Add inner VLAN tag
// @inner_vlan: Inner VLAN tag treatment parameters
// @reassign_bridge_port: Reassign to different bridge port
// @new_bridge_port_id: New bridge port ID
// @new_dscp_enable: Enable new DSCP assignment
// @new_dscp: New DSCP value
// @new_traffic_class_enable: Enable new traffic class assignment
// @new_traffic_class: New traffic class value
// @new_meter_enable: Enable new metering
// @s_new_traffic_meter_id: New traffic meter ID
// @dscp2pcp_map: DSCP to PCP mapping table (64 entries)
// @loopback_enable: Enable loopback
// @da_sa_swap_enable: Enable DA/SA swap
// @mirror_enable: Enable mirroring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_extendedvlan_treatment {
    pub remove_tag: __le32,
    pub treatment_4_tpid_mode: __le32,
    pub add_outer_vlan: u8,
    pub outer_vlan: mxl862xx_extendedvlan_treatment_vlan,
    pub add_inner_vlan: u8,
    pub inner_vlan: mxl862xx_extendedvlan_treatment_vlan,
    pub reassign_bridge_port: u8,
    pub new_bridge_port_id: __le16,
    pub new_dscp_enable: u8,
    pub new_dscp: __le16,
    pub new_traffic_class_enable: u8,
    pub new_traffic_class: u8,
    pub new_meter_enable: u8,
    pub s_new_traffic_meter_id: __le16,
    pub dscp2pcp_map: [u8; 64],
    pub loopback_enable: u8,
    pub da_sa_swap_enable: u8,
    pub mirror_enable: u8,
    pub __packed: },
//
// struct mxl862xx_extendedvlan_alloc - Extended VLAN block allocation
// @number_of_entries: Number of entries to allocate (input) / allocated
// (output)
// @extended_vlan_block_id: Block ID assigned by firmware (output on alloc,
// input on free)
//
// Used with %MXL862XX_EXTENDEDVLAN_ALLOC and %MXL862XX_EXTENDEDVLAN_FREE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_extendedvlan_alloc {
    pub number_of_entries: __le16,
    pub extended_vlan_block_id: __le16,
    pub __packed: },
//
// struct mxl862xx_extendedvlan_config - Extended VLAN entry configuration
// @extended_vlan_block_id: Block ID from allocation
// @entry_index: Entry index within the block
// @filter: Filter (match) configuration
// @treatment: Treatment (action) configuration
//
// Used with %MXL862XX_EXTENDEDVLAN_SET and %MXL862XX_EXTENDEDVLAN_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_extendedvlan_config {
    pub extended_vlan_block_id: __le16,
    pub entry_index: __le16,
    pub filter: mxl862xx_extendedvlan_filter,
    pub treatment: mxl862xx_extendedvlan_treatment,
    pub __packed: },
//
// enum mxl862xx_vlan_filter_tci_mask - VLAN Filter TCI mask
// @MXL862XX_VLAN_FILTER_TCI_MASK_VID: TCI mask for VLAN ID
// @MXL862XX_VLAN_FILTER_TCI_MASK_PCP: TCI mask for VLAN PCP
// @MXL862XX_VLAN_FILTER_TCI_MASK_TCI: TCI mask for VLAN TCI
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_vlan_filter_tci_mask {
    MXL862XX_VLAN_FILTER_TCI_MASK_VID = 0,
    MXL862XX_VLAN_FILTER_TCI_MASK_PCP = 1,
    MXL862XX_VLAN_FILTER_TCI_MASK_TCI = 2,
}

//
// struct mxl862xx_vlanfilter_alloc - VLAN Filter block allocation
// @number_of_entries: Number of entries to allocate (input) / allocated
// (output)
// @vlan_filter_block_id: Block ID assigned by firmware (output on alloc,
// input on free)
// @discard_untagged: Discard untagged packets
// @discard_unmatched_tagged: Discard tagged packets that do not match any
// entry in the block
// @use_default_port_vid: Use default port VLAN ID for filtering
//
// Used with %MXL862XX_VLANFILTER_ALLOC and %MXL862XX_VLANFILTER_FREE.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_vlanfilter_alloc {
    pub number_of_entries: __le16,
    pub vlan_filter_block_id: __le16,
    pub discard_untagged: u8,
    pub discard_unmatched_tagged: u8,
    pub use_default_port_vid: u8,
    pub __packed: },
//
// struct mxl862xx_vlanfilter_config - VLAN Filter entry configuration
// @vlan_filter_block_id: Block ID from allocation
// @entry_index: Entry index within the block
// @vlan_filter_mask: TCI field(s) to match (see
// &enum mxl862xx_vlan_filter_tci_mask)
// @val: TCI value(s) to match (VID, PCP, or full TCI depending on mask)
// @discard_matched: When true, discard frames matching this entry;
// when false, allow them
//
// Used with %MXL862XX_VLANFILTER_SET and %MXL862XX_VLANFILTER_GET.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_vlanfilter_config {
    pub vlan_filter_block_id: __le16,
    pub entry_index: __le16,
    pub /: *mut *mut __le32 vlan_filter_mask; / enum mxl862xx_vlan_filter_tci_mask,
    pub val: __le32,
    pub discard_matched: u8,
    pub __packed: },
//
// enum mxl862xx_ss_sp_tag_mask - Special tag valid field indicator bits
// @MXL862XX_SS_SP_TAG_MASK_RX: valid RX special tag mode
// @MXL862XX_SS_SP_TAG_MASK_TX: valid TX special tag mode
// @MXL862XX_SS_SP_TAG_MASK_RX_PEN: valid RX special tag info over preamble
// @MXL862XX_SS_SP_TAG_MASK_TX_PEN: valid TX special tag info over preamble
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_ss_sp_tag_mask {
    MXL862XX_SS_SP_TAG_MASK_RX = BIT(0),
    MXL862XX_SS_SP_TAG_MASK_TX = BIT(1),
    MXL862XX_SS_SP_TAG_MASK_RX_PEN = BIT(2),
    MXL862XX_SS_SP_TAG_MASK_TX_PEN = BIT(3),
}

//
// enum mxl862xx_ss_sp_tag_rx - RX special tag mode
// @MXL862XX_SS_SP_TAG_RX_NO_TAG_NO_INSERT: packet does NOT have special
// tag and special tag is NOT inserted
// @MXL862XX_SS_SP_TAG_RX_NO_TAG_INSERT: packet does NOT have special tag
// and special tag is inserted
// @MXL862XX_SS_SP_TAG_RX_TAG_NO_INSERT: packet has special tag and special
// tag is NOT inserted
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_ss_sp_tag_rx {
    MXL862XX_SS_SP_TAG_RX_NO_TAG_NO_INSERT = 0,
    MXL862XX_SS_SP_TAG_RX_NO_TAG_INSERT = 1,
    MXL862XX_SS_SP_TAG_RX_TAG_NO_INSERT = 2,
}

//
// enum mxl862xx_ss_sp_tag_tx - TX special tag mode
// @MXL862XX_SS_SP_TAG_TX_NO_TAG_NO_REMOVE: packet does NOT have special
// tag and special tag is NOT removed
// @MXL862XX_SS_SP_TAG_TX_TAG_REPLACE: packet has special tag and special
// tag is replaced
// @MXL862XX_SS_SP_TAG_TX_TAG_NO_REMOVE: packet has special tag and special
// tag is NOT removed
// @MXL862XX_SS_SP_TAG_TX_TAG_REMOVE: packet has special tag and special
// tag is removed
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_ss_sp_tag_tx {
    MXL862XX_SS_SP_TAG_TX_NO_TAG_NO_REMOVE = 0,
    MXL862XX_SS_SP_TAG_TX_TAG_REPLACE = 1,
    MXL862XX_SS_SP_TAG_TX_TAG_NO_REMOVE = 2,
    MXL862XX_SS_SP_TAG_TX_TAG_REMOVE = 3,
}

//
// enum mxl862xx_ss_sp_tag_rx_pen - RX special tag info over preamble
// @MXL862XX_SS_SP_TAG_RX_PEN_ALL_0: special tag info inserted from byte 2
// to 7 are all 0
// @MXL862XX_SS_SP_TAG_RX_PEN_BYTE_5_IS_16: special tag byte 5 is 16, other
// bytes from 2 to 7 are 0
// @MXL862XX_SS_SP_TAG_RX_PEN_BYTE_5_FROM_PREAMBLE: special tag byte 5 is
// from preamble field, others
// are 0
// @MXL862XX_SS_SP_TAG_RX_PEN_BYTE_2_TO_7_FROM_PREAMBLE: special tag byte 2
// to 7 are from preamble
// field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_ss_sp_tag_rx_pen {
    MXL862XX_SS_SP_TAG_RX_PEN_ALL_0 = 0,
    MXL862XX_SS_SP_TAG_RX_PEN_BYTE_5_IS_16 = 1,
    MXL862XX_SS_SP_TAG_RX_PEN_BYTE_5_FROM_PREAMBLE = 2,
    MXL862XX_SS_SP_TAG_RX_PEN_BYTE_2_TO_7_FROM_PREAMBLE = 3,
}

//
// struct mxl862xx_ss_sp_tag - Special tag port settings
// @pid: port ID (1~16)
// @mask: See &enum mxl862xx_ss_sp_tag_mask
// @rx: See &enum mxl862xx_ss_sp_tag_rx
// @tx: See &enum mxl862xx_ss_sp_tag_tx
// @rx_pen: See &enum mxl862xx_ss_sp_tag_rx_pen
// @tx_pen: TX special tag info over preamble
// 0 - disabled
// 1 - enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_ss_sp_tag {
    pub pid: u8,
    pub /: *mut *mut u8 mask; / enum mxl862xx_ss_sp_tag_mask,
    pub /: *mut *mut u8 rx; / enum mxl862xx_ss_sp_tag_rx,
    pub /: *mut *mut u8 tx; / enum mxl862xx_ss_sp_tag_tx,
    pub /: *mut *mut u8 rx_pen; / enum mxl862xx_ss_sp_tag_rx_pen,
    pub /: *mut *mut u8 tx_pen; / boolean,
    pub __packed: },
//
// enum mxl862xx_logical_port_mode - Logical port mode
// @MXL862XX_LOGICAL_PORT_8BIT_WLAN: WLAN with 8-bit station ID
// @MXL862XX_LOGICAL_PORT_9BIT_WLAN: WLAN with 9-bit station ID
// @MXL862XX_LOGICAL_PORT_ETHERNET: Ethernet port
// @MXL862XX_LOGICAL_PORT_OTHER: Others
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_logical_port_mode {
    MXL862XX_LOGICAL_PORT_8BIT_WLAN = 0,
    MXL862XX_LOGICAL_PORT_9BIT_WLAN,
    MXL862XX_LOGICAL_PORT_ETHERNET,
    MXL862XX_LOGICAL_PORT_OTHER = 0xFF,
}

//
// struct mxl862xx_ctp_port_assignment - CTP Port Assignment/association
// with logical port
// @logical_port_id: Logical Port Id. The valid range is hardware dependent
// @first_ctp_port_id: First CTP (Connectivity Termination Port) ID mapped
// to above logical port ID
// @number_of_ctp_port: Total number of CTP Ports mapped above logical port
// ID
// @mode: Logical port mode to define sub interface ID format. See
// &enum mxl862xx_logical_port_mode
// @bridge_port_id: Bridge Port ID (not FID). For allocation, each CTP
// allocated is mapped to the Bridge Port given by this field.
// The Bridge Port will be configured to use first CTP as
// egress CTP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_ctp_port_assignment {
    pub logical_port_id: u8,
    pub first_ctp_port_id: __le16,
    pub number_of_ctp_port: __le16,
    pub /: *mut *mut __le32 mode; / enum mxl862xx_logical_port_mode,
    pub bridge_port_id: __le16,
    pub __packed: },
//
// enum mxl862xx_stp_port_state - Spanning Tree Protocol port states
// @MXL862XX_STP_PORT_STATE_FORWARD: Forwarding state
// @MXL862XX_STP_PORT_STATE_DISABLE: Disabled/Discarding state
// @MXL862XX_STP_PORT_STATE_LEARNING: Learning state
// @MXL862XX_STP_PORT_STATE_BLOCKING: Blocking/Listening
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_stp_port_state {
    MXL862XX_STP_PORT_STATE_FORWARD = 0,
    MXL862XX_STP_PORT_STATE_DISABLE,
    MXL862XX_STP_PORT_STATE_LEARNING,
    MXL862XX_STP_PORT_STATE_BLOCKING,
}

//
// struct mxl862xx_stp_port_cfg - Configures the Spanning Tree Protocol state
// @port_id: Port number
// @fid: Filtering Identifier (FID)
// @port_state: See &enum mxl862xx_stp_port_state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_stp_port_cfg {
    pub port_id: __le16,
    pub fid: __le16,
    pub /: *mut *mut __le32 port_state; / enum mxl862xx_stp_port_state,
    pub __packed: },
//
// struct mxl862xx_sys_fw_image_version - Firmware version information
// @iv_major: firmware major version
// @iv_minor: firmware minor version
// @iv_revision: firmware revision
// @iv_build_num: firmware build number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_sys_fw_image_version {
    pub iv_major: u8,
    pub iv_minor: u8,
    pub iv_revision: __le16,
    pub iv_build_num: __le32,
    pub __packed: },
//
// enum mxl862xx_port_type - Port Type
// @MXL862XX_LOGICAL_PORT: Logical Port
// @MXL862XX_PHYSICAL_PORT: Physical Port
// @MXL862XX_CTP_PORT: Connectivity Termination Port (CTP)
// @MXL862XX_BRIDGE_PORT: Bridge Port
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_port_type {
    MXL862XX_LOGICAL_PORT = 0,
    MXL862XX_PHYSICAL_PORT,
    MXL862XX_CTP_PORT,
    MXL862XX_BRIDGE_PORT,
}

//
// enum mxl862xx_rmon_port_type - RMON counter table type
// @MXL862XX_RMON_CTP_PORT_RX: CTP RX counters
// @MXL862XX_RMON_CTP_PORT_TX: CTP TX counters
// @MXL862XX_RMON_BRIDGE_PORT_RX: Bridge port RX counters
// @MXL862XX_RMON_BRIDGE_PORT_TX: Bridge port TX counters
// @MXL862XX_RMON_CTP_PORT_PCE_BYPASS: CTP PCE bypass counters
// @MXL862XX_RMON_TFLOW_RX: TFLOW RX counters
// @MXL862XX_RMON_TFLOW_TX: TFLOW TX counters
// @MXL862XX_RMON_QMAP: QMAP counters
// @MXL862XX_RMON_METER: Meter counters
// @MXL862XX_RMON_PMAC: PMAC counters
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_rmon_port_type {
    MXL862XX_RMON_CTP_PORT_RX = 0,
    MXL862XX_RMON_CTP_PORT_TX,
    MXL862XX_RMON_BRIDGE_PORT_RX,
    MXL862XX_RMON_BRIDGE_PORT_TX,
    MXL862XX_RMON_CTP_PORT_PCE_BYPASS,
    MXL862XX_RMON_TFLOW_RX,
    MXL862XX_RMON_TFLOW_TX,
    MXL862XX_RMON_QMAP = 0x0e,
    MXL862XX_RMON_METER = 0x19,
    MXL862XX_RMON_PMAC = 0x1c,
}

//
// struct mxl862xx_rmon_port_cnt - RMON counters for a port
// @port_type: Port type for counter retrieval (see &enum mxl862xx_port_type)
// @port_id: Ethernet port number (zero-based)
// @sub_if_id_group: Sub-interface ID group
// @pce_bypass: Separate CTP Tx counters when PCE is bypassed
// @rx_extended_vlan_discard_pkts: Discarded at extended VLAN operation
// @mtu_exceed_discard_pkts: Discarded due to MTU exceeded
// @tx_under_size_good_pkts: Tx undersize (<64) packet count
// @tx_oversize_good_pkts: Tx oversize (>1518) packet count
// @rx_good_pkts: Received good packet count
// @rx_unicast_pkts: Received unicast packet count
// @rx_broadcast_pkts: Received broadcast packet count
// @rx_multicast_pkts: Received multicast packet count
// @rx_fcserror_pkts: Received FCS error packet count
// @rx_under_size_good_pkts: Received undersize good packet count
// @rx_oversize_good_pkts: Received oversize good packet count
// @rx_under_size_error_pkts: Received undersize error packet count
// @rx_good_pause_pkts: Received good pause packet count
// @rx_oversize_error_pkts: Received oversize error packet count
// @rx_align_error_pkts: Received alignment error packet count
// @rx_filtered_pkts: Filtered packet count
// @rx64byte_pkts: Received 64-byte packet count
// @rx127byte_pkts: Received 65-127 byte packet count
// @rx255byte_pkts: Received 128-255 byte packet count
// @rx511byte_pkts: Received 256-511 byte packet count
// @rx1023byte_pkts: Received 512-1023 byte packet count
// @rx_max_byte_pkts: Received 1024-max byte packet count
// @tx_good_pkts: Transmitted good packet count
// @tx_unicast_pkts: Transmitted unicast packet count
// @tx_broadcast_pkts: Transmitted broadcast packet count
// @tx_multicast_pkts: Transmitted multicast packet count
// @tx_single_coll_count: Transmit single collision count
// @tx_mult_coll_count: Transmit multiple collision count
// @tx_late_coll_count: Transmit late collision count
// @tx_excess_coll_count: Transmit excessive collision count
// @tx_coll_count: Transmit collision count
// @tx_pause_count: Transmit pause packet count
// @tx64byte_pkts: Transmitted 64-byte packet count
// @tx127byte_pkts: Transmitted 65-127 byte packet count
// @tx255byte_pkts: Transmitted 128-255 byte packet count
// @tx511byte_pkts: Transmitted 256-511 byte packet count
// @tx1023byte_pkts: Transmitted 512-1023 byte packet count
// @tx_max_byte_pkts: Transmitted 1024-max byte packet count
// @tx_dropped_pkts: Transmit dropped packet count
// @tx_acm_dropped_pkts: Transmit ACM dropped packet count
// @rx_dropped_pkts: Received dropped packet count
// @rx_good_bytes: Received good byte count (64-bit)
// @rx_bad_bytes: Received bad byte count (64-bit)
// @tx_good_bytes: Transmitted good byte count (64-bit)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_rmon_port_cnt {
    pub /: *mut *mut __le32 port_type; / enum mxl862xx_port_type,
    pub port_id: __le16,
    pub sub_if_id_group: __le16,
    pub pce_bypass: u8,
    pub rx_extended_vlan_discard_pkts: __le32,
    pub mtu_exceed_discard_pkts: __le32,
    pub tx_under_size_good_pkts: __le32,
    pub tx_oversize_good_pkts: __le32,
    pub rx_good_pkts: __le32,
    pub rx_unicast_pkts: __le32,
    pub rx_broadcast_pkts: __le32,
    pub rx_multicast_pkts: __le32,
    pub rx_fcserror_pkts: __le32,
    pub rx_under_size_good_pkts: __le32,
    pub rx_oversize_good_pkts: __le32,
    pub rx_under_size_error_pkts: __le32,
    pub rx_good_pause_pkts: __le32,
    pub rx_oversize_error_pkts: __le32,
    pub rx_align_error_pkts: __le32,
    pub rx_filtered_pkts: __le32,
    pub rx64byte_pkts: __le32,
    pub rx127byte_pkts: __le32,
    pub rx255byte_pkts: __le32,
    pub rx511byte_pkts: __le32,
    pub rx1023byte_pkts: __le32,
    pub rx_max_byte_pkts: __le32,
    pub tx_good_pkts: __le32,
    pub tx_unicast_pkts: __le32,
    pub tx_broadcast_pkts: __le32,
    pub tx_multicast_pkts: __le32,
    pub tx_single_coll_count: __le32,
    pub tx_mult_coll_count: __le32,
    pub tx_late_coll_count: __le32,
    pub tx_excess_coll_count: __le32,
    pub tx_coll_count: __le32,
    pub tx_pause_count: __le32,
    pub tx64byte_pkts: __le32,
    pub tx127byte_pkts: __le32,
    pub tx255byte_pkts: __le32,
    pub tx511byte_pkts: __le32,
    pub tx1023byte_pkts: __le32,
    pub tx_max_byte_pkts: __le32,
    pub tx_dropped_pkts: __le32,
    pub tx_acm_dropped_pkts: __le32,
    pub rx_dropped_pkts: __le32,
    pub rx_good_bytes: __le64,
    pub rx_bad_bytes: __le64,
    pub tx_good_bytes: __le64,
    pub __packed: },
// XPCS interface mode, MXL862XX_XPCS_*_INTERFACE field values
pub const MXL862XX_XPCS_IF_SGMII: c_int = 0;
pub const MXL862XX_XPCS_IF_1000BASEX: c_int = 1;
pub const MXL862XX_XPCS_IF_2500BASEX: c_int = 2;

pub const MXL862XX_XPCS_IF_10GBASER: c_int = 4;

pub const MXL862XX_XPCS_IF_5GBASER: c_int = 6;
pub const MXL862XX_XPCS_IF_QSGMII: c_int = 7;
// PCS negotiation mode, MXL862XX_XPCS_CFG_NEG_MODE field values

//
// PCS protocol role, MXL862XX_XPCS_CFG_ROLE field value. Selects the role
// the XPCS plays in protocols with an asymmetric AN code word (Cisco SGMII
// / QSGMII / USXGMII), driving VR_MII_AN_CTRL.TX_CONFIG: MAC means the
// local end receives the partner's AN word, PHY means it sources one.
// Ignored for symmetric protocols (1000BASE-X, 2500BASE-X, 10GBASE-R/KR).
//

// USXGMII lane mode, MXL862XX_XPCS_*_USX_LANE_MODE field values

//
// union mxl862xx_xpcs_an_word - XPCS AN code word, tagged by interface mode
// @cl37: 16-bit base-page word exchanged over the CL37 hardware AN path
// (SR_MII_AN_ADV on write, SR_MII_LP_BABL on read). Carries the
// 802.3 CL37 base page for 1000BASE-X/2500BASE-X and the Cisco
// SGMII config word for SGMII/QSGMII.
// @usx: USXGMII 16-bit AN code word, MDIO_USXGMII_* layout
// @cl73: CL73 48-bit base page (10GBASE-KR), three 16-bit registers per
// 802.3 Annex 28C
// @cl73.adv1: CL73 SR_AN_ADV1 / SR_AN_LP_ABL1
// @cl73.adv2: CL73 SR_AN_ADV2 / SR_AN_LP_ABL2
// @cl73.adv3: CL73 SR_AN_ADV3 / SR_AN_LP_ABL3
//
// The host picks the right member based on the interface field of the
// surrounding struct (and, for the asymmetric protocols, on the role).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union mxl862xx_xpcs_an_word {
    pub cl37: __le16,
    pub usx: __le16,
    pub adv1: __le16,
    pub adv2: __le16,
    pub adv3: __le16,
    pub cl73: },
    pub __packed: },
// PCS duplex mode, MXL862XX_XPCS_*_DUPLEX field values
pub const MXL862XX_XPCS_DUPLEX_HALF: c_int = 0;
pub const MXL862XX_XPCS_DUPLEX_FULL: c_int = 1;
//
// enum mxl862xx_xpcs_loopback_mode - XPCS loopback mode
// @MXL862XX_XPCS_LB_DISABLE: disable all loopback
// @MXL862XX_XPCS_LB_PCS_SERIAL: PCS TX-to-RX serial loopback
// @MXL862XX_XPCS_LB_PCS_PARALLEL: PCS RX-to-TX parallel loopback
// @MXL862XX_XPCS_LB_PMA_SERIAL: PMA TX-to-RX serial loopback
// @MXL862XX_XPCS_LB_PMA_PARALLEL: PMA RX-to-TX parallel loopback
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mxl862xx_xpcs_loopback_mode {
    MXL862XX_XPCS_LB_DISABLE = 0,
    MXL862XX_XPCS_LB_PCS_SERIAL = 1,
    MXL862XX_XPCS_LB_PCS_PARALLEL = 2,
    MXL862XX_XPCS_LB_PMA_SERIAL = 3,
    MXL862XX_XPCS_LB_PMA_PARALLEL = 4,
}

// Fields of mxl862xx_xpcs_pcs_cfg.mode

//
// struct mxl862xx_xpcs_pcs_cfg - PCS configuration parameters
// @mode: Packed interface and negotiation parameters, see
// MXL862XX_XPCS_CFG_*. port_id is the XPCS port index (0-3);
// interface is the PCS interface mode (MXL862XX_XPCS_IF_*);
// neg_mode is the negotiation mode (MXL862XX_XPCS_NEG_*);
// permit_pause allows pause to MAC; usx_lane_mode is the USXGMII
// lane mode (MXL862XX_XPCS_USX_*); role is the protocol role
// (MXL862XX_XPCS_ROLE_*); usx_subport is the sub-port (0-3) within
// the XPCS -- despite the name it also identifies the QSGMII
// sub-port -- used by the firmware to set MAC pause per sub-port
// and ignored for the XPCS-wide bringup, which is idempotent across
// slots.
// @advertising: AN code word the local end transmits. The active union
// member is selected by the interface field (and, for the
// asymmetric protocols, by role). Ignored when the local end
// does not transmit an AN word (role=MAC for SGMII/QSGMII
// USXGMII, 10GBASE-R, 5GBASE-R) or when neg_mode is not
// INBAND_AN_ON. Pass all-zero to keep the firmware default
// advertisement.
// @result: Firmware result. >0 means the host must follow with an AN
// restart, 0 means no host follow-up is needed, <0 is an errno.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_xpcs_pcs_cfg {
    pub mode: __le16,
    pub advertising: mxl862xx_xpcs_an_word,
    pub result: __le16,
    pub __packed: },
// Fields of mxl862xx_xpcs_pcs_state.mode

//
// struct mxl862xx_xpcs_pcs_state - PCS link state
// @mode: Packed input parameters and firmware status, see
// MXL862XX_XPCS_ST_*. The host writes port_id (XPCS port index 0-3),
// interface (MXL862XX_XPCS_IF_*), usx_lane_mode
// (MXL862XX_XPCS_USX_*) and usx_subport (0-3); the firmware fills in
// link, an_complete, duplex (MXL862XX_XPCS_DUPLEX_*), pcs_fault,
// pause (bit 0 symmetric, bit 1 asymmetric), lp_eee_cap and
// lp_eee_cs_cap.
// @speed: Resolved speed in Mbit/s (output)
// @lpa: Link partner ability word (output). Same union as
// &union mxl862xx_xpcs_an_word; the host picks the member based on
// the interface field.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_xpcs_pcs_state {
    pub mode: __le32,
    pub /: *mut *mut __le16 speed; / Mbit/s,
    pub lpa: mxl862xx_xpcs_an_word,
    pub __packed: },
//
// struct mxl862xx_xpcs_pcs_disable - PCS disable parameters
// @port_id: XPCS port index
// @__pad: padding
// @result: Firmware result. 0 on success, <0 on error.
//
// Asserts IDDQ + PHY + XPCS resets to power down the SERDES when the
// port is admin-down or no module is plugged in. The next PCS config
// implicitly powers it back up and reprograms the desired interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_xpcs_pcs_disable {
    pub port_id: u8,
    pub __pad: u8,
    pub result: __le16,
    pub __packed: },
// Fields of mxl862xx_xpcs_an_restart.mode

//
// struct mxl862xx_xpcs_an_restart - AN restart parameters
// @mode: Packed input parameters, see MXL862XX_XPCS_ANR_*. port_id is the
// XPCS port index (0-3); interface is the PCS interface mode
// (MXL862XX_XPCS_IF_*); usx_lane_mode is the USX lane mode
// (MXL862XX_XPCS_USX_*); usx_subport (0-3) selects the lane whose
// AN is restarted for QSGMII and QUSXGMII and is ignored by
// single-lane modes.
// @result: Firmware result. 0 on success, <0 on error.
//
// Restarts auto-negotiation on a single sub-port of the XPCS. The
// SERDES must already be configured.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_xpcs_an_restart {
    pub mode: __le16,
    pub result: __le16,
    pub __packed: },
// Fields of mxl862xx_xpcs_pcs_link_up.mode

//
// struct mxl862xx_xpcs_pcs_link_up - PCS link-up parameters
// @mode: Packed input parameters, see MXL862XX_XPCS_LU_*. port_id is the
// XPCS port index (0-3); interface is the PCS interface mode
// (MXL862XX_XPCS_IF_*); duplex is the duplex mode
// (MXL862XX_XPCS_DUPLEX_*); usx_lane_mode is the USX lane mode
// (USXGMII only, ignored otherwise, MXL862XX_XPCS_USX_*);
// usx_subport (0-3) selects the sub-port for QUSXGMII and QSGMII
// (despite the name) and is ignored otherwise.
// @speed: Resolved speed in Mbit/s
// @result: Firmware result. 0 on success, <0 is errno.
//
// Called once per link-up event after the host has resolved the
// line-side speed/duplex (from the PHY's read_status, from a preceding
// PCS get-state, or from a fixed-link description).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mxl862xx_xpcs_pcs_link_up {
    pub mode: __le16,
    pub /: *mut *mut __le16 speed; / Mbit/s,
    pub result: __le16,
    pub __packed: },
