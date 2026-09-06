//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/dsa.h
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
// include/net/dsa.h - Driver for Distributed Switch Architecture switch chips
// Copyright (c) 2008-2009 Marvell Semiconductor
//

pub const DSA_TAG_PROTO_NONE_VALUE: c_int = 0;
pub const DSA_TAG_PROTO_BRCM_VALUE: c_int = 1;
pub const DSA_TAG_PROTO_BRCM_PREPEND_VALUE: c_int = 2;
pub const DSA_TAG_PROTO_DSA_VALUE: c_int = 3;
pub const DSA_TAG_PROTO_EDSA_VALUE: c_int = 4;
pub const DSA_TAG_PROTO_GSWIP_VALUE: c_int = 5;
pub const DSA_TAG_PROTO_KSZ9477_VALUE: c_int = 6;
pub const DSA_TAG_PROTO_KSZ9893_VALUE: c_int = 7;
pub const DSA_TAG_PROTO_LAN9303_VALUE: c_int = 8;
pub const DSA_TAG_PROTO_MTK_VALUE: c_int = 9;
pub const DSA_TAG_PROTO_QCA_VALUE: c_int = 10;
pub const DSA_TAG_PROTO_TRAILER_VALUE: c_int = 11;
pub const DSA_TAG_PROTO_8021Q_VALUE: c_int = 12;
pub const DSA_TAG_PROTO_SJA1105_VALUE: c_int = 13;
pub const DSA_TAG_PROTO_KSZ8795_VALUE: c_int = 14;
pub const DSA_TAG_PROTO_OCELOT_VALUE: c_int = 15;
pub const DSA_TAG_PROTO_AR9331_VALUE: c_int = 16;
pub const DSA_TAG_PROTO_RTL4_A_VALUE: c_int = 17;
pub const DSA_TAG_PROTO_HELLCREEK_VALUE: c_int = 18;
pub const DSA_TAG_PROTO_XRS700X_VALUE: c_int = 19;
pub const DSA_TAG_PROTO_OCELOT_8021Q_VALUE: c_int = 20;
pub const DSA_TAG_PROTO_SEVILLE_VALUE: c_int = 21;
pub const DSA_TAG_PROTO_BRCM_LEGACY_VALUE: c_int = 22;
pub const DSA_TAG_PROTO_SJA1110_VALUE: c_int = 23;
pub const DSA_TAG_PROTO_RTL8_4_VALUE: c_int = 24;
pub const DSA_TAG_PROTO_RTL8_4T_VALUE: c_int = 25;
pub const DSA_TAG_PROTO_RZN1_A5PSW_VALUE: c_int = 26;
pub const DSA_TAG_PROTO_LAN937X_VALUE: c_int = 27;
pub const DSA_TAG_PROTO_VSC73XX_8021Q_VALUE: c_int = 28;
pub const DSA_TAG_PROTO_BRCM_LEGACY_FCS_VALUE: c_int = 29;
pub const DSA_TAG_PROTO_YT921X_VALUE: c_int = 30;
pub const DSA_TAG_PROTO_MXL_GSW1XX_VALUE: c_int = 31;
pub const DSA_TAG_PROTO_MXL862_VALUE: c_int = 32;
pub const DSA_TAG_PROTO_NETC_VALUE: c_int = 33;
pub const DSA_TAG_PROTO_KSZ8463_VALUE: c_int = 34;
pub const DSA_TAG_PROTO_MT7628_VALUE: c_int = 35;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsa_tag_protocol {
    DSA_TAG_PROTO_NONE		= DSA_TAG_PROTO_NONE_VALUE,
    DSA_TAG_PROTO_BRCM		= DSA_TAG_PROTO_BRCM_VALUE,
    DSA_TAG_PROTO_BRCM_LEGACY	= DSA_TAG_PROTO_BRCM_LEGACY_VALUE,
    DSA_TAG_PROTO_BRCM_LEGACY_FCS	= DSA_TAG_PROTO_BRCM_LEGACY_FCS_VALUE,
    DSA_TAG_PROTO_BRCM_PREPEND	= DSA_TAG_PROTO_BRCM_PREPEND_VALUE,
    DSA_TAG_PROTO_DSA		= DSA_TAG_PROTO_DSA_VALUE,
    DSA_TAG_PROTO_EDSA		= DSA_TAG_PROTO_EDSA_VALUE,
    DSA_TAG_PROTO_GSWIP		= DSA_TAG_PROTO_GSWIP_VALUE,
    DSA_TAG_PROTO_KSZ9477		= DSA_TAG_PROTO_KSZ9477_VALUE,
    DSA_TAG_PROTO_KSZ9893		= DSA_TAG_PROTO_KSZ9893_VALUE,
    DSA_TAG_PROTO_LAN9303		= DSA_TAG_PROTO_LAN9303_VALUE,
    DSA_TAG_PROTO_MTK		= DSA_TAG_PROTO_MTK_VALUE,
    DSA_TAG_PROTO_QCA		= DSA_TAG_PROTO_QCA_VALUE,
    DSA_TAG_PROTO_TRAILER		= DSA_TAG_PROTO_TRAILER_VALUE,
    DSA_TAG_PROTO_8021Q		= DSA_TAG_PROTO_8021Q_VALUE,
    DSA_TAG_PROTO_SJA1105		= DSA_TAG_PROTO_SJA1105_VALUE,
    DSA_TAG_PROTO_KSZ8795		= DSA_TAG_PROTO_KSZ8795_VALUE,
    DSA_TAG_PROTO_OCELOT		= DSA_TAG_PROTO_OCELOT_VALUE,
    DSA_TAG_PROTO_AR9331		= DSA_TAG_PROTO_AR9331_VALUE,
    DSA_TAG_PROTO_RTL4_A		= DSA_TAG_PROTO_RTL4_A_VALUE,
    DSA_TAG_PROTO_HELLCREEK		= DSA_TAG_PROTO_HELLCREEK_VALUE,
    DSA_TAG_PROTO_XRS700X		= DSA_TAG_PROTO_XRS700X_VALUE,
    DSA_TAG_PROTO_OCELOT_8021Q	= DSA_TAG_PROTO_OCELOT_8021Q_VALUE,
    DSA_TAG_PROTO_SEVILLE		= DSA_TAG_PROTO_SEVILLE_VALUE,
    DSA_TAG_PROTO_SJA1110		= DSA_TAG_PROTO_SJA1110_VALUE,
    DSA_TAG_PROTO_RTL8_4		= DSA_TAG_PROTO_RTL8_4_VALUE,
    DSA_TAG_PROTO_RTL8_4T		= DSA_TAG_PROTO_RTL8_4T_VALUE,
    DSA_TAG_PROTO_RZN1_A5PSW	= DSA_TAG_PROTO_RZN1_A5PSW_VALUE,
    DSA_TAG_PROTO_LAN937X		= DSA_TAG_PROTO_LAN937X_VALUE,
    DSA_TAG_PROTO_VSC73XX_8021Q	= DSA_TAG_PROTO_VSC73XX_8021Q_VALUE,
    DSA_TAG_PROTO_YT921X		= DSA_TAG_PROTO_YT921X_VALUE,
    DSA_TAG_PROTO_MXL_GSW1XX	= DSA_TAG_PROTO_MXL_GSW1XX_VALUE,
    DSA_TAG_PROTO_MXL862		= DSA_TAG_PROTO_MXL862_VALUE,
    DSA_TAG_PROTO_NETC		= DSA_TAG_PROTO_NETC_VALUE,
    DSA_TAG_PROTO_KSZ8463		= DSA_TAG_PROTO_KSZ8463_VALUE,
    DSA_TAG_PROTO_MT7628		= DSA_TAG_PROTO_MT7628_VALUE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_device_ops {
    pub dev): *mut *mut *mut *mut sk_buff (xmit)(sk_buff skb, net_device,
    pub dev): *mut *mut *mut *mut sk_buff (rcv)(sk_buff skb, net_device,
    pub offset): *mut c_int,
    pub ds): *mut *mut int (connect)(struct dsa_switch,
    pub ds): *mut *mut void (disconnect)(struct dsa_switch,
    pub needed_headroom: c_uint,
    pub needed_tailroom: c_uint,
    pub name: *const c_char,
    pub proto: dsa_tag_protocol,
// Some tagging protocols either mangle or shift the destination MAC
// address, in which case the DSA conduit would drop packets on ingress
// if what it understands out of the destination MAC address is not in
// its RX filter.
//
    pub promisc_on_conduit: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_lag {
    pub dev: *mut net_device,
    pub id: c_uint,
    pub fdb_lock: mutex,
    pub fdbs: list_head,
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_switch_tree {
    pub list: list_head,
// List of switch ports
    pub ports: list_head,
// Notifier chain for switch-wide events
    pub nh: raw_notifier_head,
// Tree identifier
    pub index: c_uint,
// Number of switches attached to this tree
    pub refcount: kref,
// Maps offloaded LAG netdevs to a zero-based linear ID for
// drivers that need it.
//
    pub lags: *mut dsa_lag,
// Tagging protocol operations
    pub tag_ops: *const dsa_device_ops,
// Default tagging protocol preferred by the switches in this
// tree.
//
    pub default_proto: dsa_tag_protocol,
// Has this tree been applied to the hardware?
    pub setup: bool,
//
// Configuration data for the platform device that owns
// this dsa switch tree instance.
//
    pub pd: *mut dsa_platform_data,
// List of DSA links composing the routing table
    pub rtable: list_head,
// Length of "lags" array
    pub lags_len: c_uint,
// Track the largest switch index within a tree
    pub last_switch: c_uint,
}

// LAG IDs are one-based, the dst->lags array is zero-based

// DSA LAG IDs are one-based, dst->lags is zero-based
// TC matchall action types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsa_port_mall_action_type {
    DSA_PORT_MALL_MIRROR,
    DSA_PORT_MALL_POLICER,
}

// TC mirroring entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_mall_mirror_tc_entry {
    pub to_local_port: u8,
    pub ingress: bool,
}

// TC matchall entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_mall_tc_entry {
    pub list: list_head,
    pub cookie: c_ulong,
    pub type: dsa_port_mall_action_type,
    pub mirror: dsa_mall_mirror_tc_entry,
    pub policer: flow_action_police,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_bridge {
    pub dev: *mut net_device,
    pub num: c_uint,
    pub tx_fwd_offload: bool,
    pub refcount: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_port {
// A CPU port is physically connected to a conduit device. A user port
// exposes a network device to user-space, called 'user' here.
//
    pub conduit: *mut net_device,
    pub user: *mut net_device,
}

// Copy of the tagging protocol operations, for quicker access
// in the data path. Valid only for the CPU ports.
//
// Copies for faster access in conduit receive hot path
// Warning: the following bit fields are not atomic, and updating them
// can only be done from code paths where concurrency is not possible
// (probe time or under rtnl_lock).
//
// Managed by DSA on user ports and by drivers on CPU and DSA ports
// conduit state bits, valid only on CPU ports
// Valid only on user ports
//
// Original copy of the conduit netdev ethtool_ops
//
// List of MAC addresses that must be forwarded on this port.
// These are only valid on CPU ports and DSA links.
//
// List of VLANs that CPU and DSA ports are members of.
// Access to this is serialized by the sleepable @vlans_lock.
//
// List of VLANs that user ports are members of.
// Access to this is serialized by netif_addr_lock_bh().
//
extern "C" {
    pub fn container_of(_arg: config, dsa_port: struct, _arg: pl_config) -> return;
}
// TODO: ideally DSA ports would have a single dp->link_dp member,
// and no dst->rtable nor this struct dsa_link would be needed,
// but this would require some more complex tree walking,
// so keep it stupid at the moment and list them all.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_link {
    pub dp: *mut dsa_port,
    pub link_dp: *mut dsa_port,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsa_db_type {
    DSA_DB_PORT,
    DSA_DB_LAG,
    DSA_DB_BRIDGE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_db {
    pub type: dsa_db_type,
    pub dp: *const dsa_port,
    pub lag: dsa_lag,
    pub bridge: dsa_bridge,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_mac_addr {
    pub addr: [c_uchar; ETH_ALEN],
    pub vid: u16,
    pub refcount: refcount_t,
    pub list: list_head,
    pub db: dsa_db,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_vlan {
    pub vid: u16,
    pub refcount: refcount_t,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_switch {
    pub dev: *mut device,
//
// Parent switch tree, and switch index.
//
    pub dst: *mut dsa_switch_tree,
    pub index: c_uint,
// Warning: the following bit fields are not atomic, and updating them
// can only be done from code paths where concurrency is not possible
// (probe time or under rtnl_lock).
//
    pub setup:1: u32,
// Disallow bridge core from requesting different VLAN awareness
// settings on ports if not hardware-supported
//
    pub vlan_filtering_is_global:1: u32,
// Keep VLAN filtering enabled on ports not offloading any upper
    pub needs_standalone_vlan_filtering:1: u32,
// Pass .port_vlan_add and .port_vlan_del to drivers even for bridges
// that have vlan_filtering=0. All drivers should ideally set this (and
// then the option would get removed), but it is unknown whether this
// would break things or not.
//
    pub configure_vlan_while_not_filtering:1: u32,
// Pop the default_pvid of VLAN-unaware bridge ports from tagged frames.
// DEPRECATED: Do NOT set this field in new drivers. Instead look at
// the dsa_software_vlan_untag() comments.
//
    pub untag_bridge_pvid:1: u32,
// Pop the default_pvid of VLAN-aware bridge ports from tagged frames.
// Useful if the switch cannot preserve the VLAN tag as seen on the
// wire for user port ingress, and chooses to send all frames as
// VLAN-tagged to the CPU, including those which were originally
// untagged.
//
    pub untag_vlan_aware_bridge_pvid:1: u32,
// Let DSA manage the FDB entries towards the
// CPU, based on the software bridge database.
//
    pub assisted_learning_on_cpu_port:1: u32,
// In case vlan_filtering_is_global is set, the VLAN awareness state
// should be retrieved from here and not from the per-port settings.
//
    pub vlan_filtering:1: u32,
// For switches that only have the MRU configurable. To ensure the
// configured MTU is not exceeded, normalization of MRU on all bridged
// interfaces is needed.
//
    pub mtu_enforcement_ingress:1: u32,
// Drivers that isolate the FDBs of multiple bridges must set this
// to true to receive the bridge as an argument in .port_fdb_{add,del}
// and .port_mdb_{add,del}. Otherwise, the bridge.num will always be
// passed as zero.
//
    pub fdb_isolation:1: u32,
// Drivers that have global DSCP mapping settings must set this to
// true to automatically apply the settings to all ports.
//
    pub dscp_prio_mapping_is_global:1: u32,
// Listener for switch fabric events
    pub nb: notifier_block,
//
// Give the switch driver somewhere to hang its private data
// structure.
//
    pub priv: *mut c_void,
    pub tagger_data: *mut c_void,
//
// Configuration data for this switch.
//
    pub cd: *mut dsa_chip_data,
//
// The switch operations.
//
    pub ops: *const dsa_switch_ops,
//
// Allow a DSA switch driver to override the phylink MAC ops
//
    pub phylink_mac_ops: *const phylink_mac_ops,
//
// User mii_bus and devices for the individual ports.
//
    pub phys_mii_mask: u32,
    pub user_mii_bus: *mut mii_bus,
// Ageing Time limits in msecs
    pub ageing_time_min: c_uint,
    pub ageing_time_max: c_uint,
// Storage for drivers using tag_8021q
    pub tag_8021q_ctx: *mut dsa_8021q_context,
// devlink used to represent this switch device
    pub devlink: *mut devlink,
// Number of switch port queues
    pub num_tx_queues: c_uint,
// Drivers that benefit from having an ID associated with each
// offloaded LAG should set this to the maximum number of
// supported IDs. DSA will then maintain a mapping of _at
// least_ these many IDs, accessible to drivers via
// dsa_lag_id().
//
    pub num_lag_ids: c_uint,
// Drivers that support bridge forwarding offload or FDB isolation
// should set this to the maximum number of bridges spanning the same
// switch tree (or all trees, in the case of cross-tree bridging
// support) that can be offloaded.
//
    pub max_num_bridges: c_uint,
    pub num_ports: c_uint,
}

// Return the local port used to reach an arbitrary switch device
// Return the local port used to reach an arbitrary switch port
extern "C" {
    pub fn dsa_routing_port(_arg: ds, _arg: device) -> return;
}
// Return the local port used to reach the dedicated CPU port
extern "C" {
    pub fn dsa_towards_port(_arg: ds, _arg: cpu_dp->ds->index, _arg: cpu_dp->index) -> return;
}
// Return true if this is the local port used to reach the CPU port
// Return true if this is a DSA port leading away from the CPU
extern "C" {
    pub fn dsa_is_dsa_port(_arg: ds, !dsa_is_upstream_port(ds: port) &&, _arg: port) -> return;
}
// Return the local port used to reach the CPU port
extern "C" {
    pub fn dsa_upstream_port(_arg: ds, _arg: dp->index) -> return;
}
// Return true if @upstream_ds is an upstream switch of @downstream_ds, meaning
// that the routing port from @downstream_ds to @upstream_ds is also the port
// which @downstream_ds uses to reach its dedicated CPU.
//
extern "C" {
    pub fn dsa_is_upstream_port(_arg: downstream_ds, _arg: routing_port) -> return;
}
extern "C" {
    pub fn dsa_port_lag_dev_get(_arg: dp->cpu_dp) -> return;
}
// Standalone ports are not in the same bridge with one another
// DSA ports connected to a bridge, and event was emitted
// for the bridge.
//
// Returns true if any port of this tree offloads the given net_device
// Returns true if any port of this tree offloads the given bridge

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_switch_ops {
//
// Tagging protocol helpers called for the CPU ports and DSA links.
// @get_tag_protocol retrieves the initial tagging protocol and is
// mandatory. Switches which can operate using multiple tagging
// protocols should implement @change_tag_protocol and report in
// @get_tag_protocol the tagger in current use.
//
    pub mprot): dsa_tag_protocol,
    pub proto): dsa_tag_protocol,
//
// Method for switch drivers to connect to the tagging protocol driver
// in current use. The switch driver can provide handlers for certain
// types of packets for switch management.
//
    pub proto): dsa_tag_protocol,
    pub extack): *mut netlink_ext_ack,
// Optional switch-wide initialization and destruction methods
    pub ds): *mut *mut int (setup)(struct dsa_switch,
    pub ds): *mut *mut void (teardown)(struct dsa_switch,
// Per-port initialization and destruction methods. Mandatory if the
// driver registers devlink port regions, optional otherwise.
//
    pub port): *mut *mut *mut int (port_setup)(struct dsa_switch ds, int,
    pub port): *mut *mut *mut void (port_teardown)(struct dsa_switch ds, int,
    pub port): *mut *mut *mut u32 (get_phy_flags)(struct dsa_switch ds, int,
//
// Access to the switch's PHY registers.
//
    pub regnum): *mut *mut *mut int (phy_read)(struct dsa_switch ds, int port, int,
    pub val): int regnum, u16,
//
// PHYLINK integration
//
    pub config): *mut phylink_config,
    pub state): *mut phylink_link_state,
//
// Port statistics counters.
//
    pub data): *mut u32 stringset, uint8_t,
    pub data): *mut int port, uint64_t,
    pub sset): *mut *mut *mut int (get_sset_count)(struct dsa_switch ds, int port, int,
    pub data): *mut int port, uint64_t,
    pub phy_stats): *mut ethtool_eth_phy_stats,
    pub mac_stats): *mut ethtool_eth_mac_stats,
    pub ctrl_stats): *mut ethtool_eth_ctrl_stats,
    pub ranges): *const ethtool_rmon_hist_range,
    pub ts_stats): *mut ethtool_ts_stats,
    pub s): *mut rtnl_link_stats64,
    pub pause_stats): *mut ethtool_pause_stats,
    pub data): *mut *mut ethtool_test etest, u64,
//
// ethtool Wake-on-LAN
//
    pub w): *mut ethtool_wolinfo,
    pub w): *mut ethtool_wolinfo,
//
// ethtool timestamp info
//
    pub ts): *mut kernel_ethtool_ts_info,
//
// ethtool MAC merge layer
//
    pub state): *mut ethtool_mm_state,
    pub extack): *mut netlink_ext_ack,
    pub stats): *mut ethtool_mm_stats,
//
// DCB ops
//
    pub port): *mut *mut *mut int (port_get_default_prio)(struct dsa_switch ds, int,
    pub prio): u8,
    pub dscp): *mut *mut *mut int (port_get_dscp_prio)(struct dsa_switch ds, int port, u8,
    pub prio): u8,
    pub prio): u8,
    pub nsel): *const *const u8 sel, int,
    pub nsel): *mut c_int,
//
// Suspend and resume
//
    pub ds): *mut *mut int (suspend)(struct dsa_switch,
    pub ds): *mut *mut int (resume)(struct dsa_switch,
//
// Port enable/disable
//
    pub phy): *mut phy_device,
    pub port): *mut *mut *mut void (port_disable)(struct dsa_switch ds, int,
//
// Notification for MAC address changes on user ports. Drivers can
// currently only veto operations. They should not use the method to
// program the hardware, since the operation is not rolled back in case
// of other errors.
//
    pub addr): *const c_uchar,
//
// Compatibility between device trees defining multiple CPU ports and
// drivers which are not OK to use by default the numerically smallest
// CPU port of a switch for its local ports. This can return NULL,
// meaning "don't know/don't care".
//
    pub ds): *mut *mut *mut dsa_port (preferred_default_local_cpu_port)(dsa_switch,
//
// Port's MAC EEE settings
//
    pub port): *mut *mut *mut bool (support_eee)(struct dsa_switch ds, int,
    pub e): *mut ethtool_keee,
// EEPROM access
    pub ds): *mut *mut int (get_eeprom_len)(struct dsa_switch,
    pub data): *mut *mut ethtool_eeprom eeprom, u8,
    pub data): *mut *mut ethtool_eeprom eeprom, u8,
//
// Register access.
//
    pub port): *mut *mut *mut int (get_regs_len)(struct dsa_switch ds, int,
    pub p): *mut *mut ethtool_regs regs, void,
//
// Upper device tracking.
//
    pub info): *mut netdev_notifier_changeupper_info,
//
// Bridge integration
//
    pub msecs): *mut *mut *mut int (set_ageing_time)(struct dsa_switch ds, unsigned int,
    pub extack): *mut netlink_ext_ack,
    pub bridge): dsa_bridge,
    pub state): u8,
    pub state): *const switchdev_mst_state,
    pub port): *mut *mut *mut void (port_fast_age)(struct dsa_switch ds, int,
    pub vid): *mut *mut *mut int (port_vlan_fast_age)(struct dsa_switch ds, int port, u16,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub mc): bool uc, bool,
//
// VLAN support
//
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub vlan): *const switchdev_obj_port_vlan,
    pub msti): *const switchdev_vlan_msti,
//
// Forwarding database
//
    pub db): dsa_db,
    pub db): dsa_db,
    pub data): *mut *mut dsa_fdb_dump_cb_t cb, void,
    pub db): dsa_db,
    pub db): dsa_db,
//
// Multicast database
//
    pub db): dsa_db,
    pub db): dsa_db,
//
// RXNFC
//
    pub rule_locs): *mut *mut ethtool_rxnfc nfc, u32,
    pub nfc): *mut ethtool_rxnfc,
//
// TC integration
//
    pub ingress): *mut *mut flow_cls_offload cls, bool,
    pub ingress): *mut *mut flow_cls_offload cls, bool,
    pub ingress): *mut *mut flow_cls_offload cls, bool,
    pub extack): *mut bool ingress, struct netlink_ext_ack,
    pub mirror): *mut dsa_mall_mirror_tc_entry,
    pub extack): *mut netlink_ext_ack,
    pub port): *mut *mut *mut void (port_policer_del)(struct dsa_switch ds, int,
    pub type_data): *mut tc_setup_type type, void,
//
// Cross-chip operations
//
    pub extack): *mut netlink_ext_ack,
    pub bridge): dsa_bridge,
    pub port): c_int,
    pub extack): *mut netlink_ext_ack,
    pub lag): int port, struct dsa_lag,
//
// PTP functionality
//
    pub config): *mut kernel_hwtstamp_config,
    pub extack): *mut netlink_ext_ack,
    pub skb): *mut sk_buff,
    pub type): *mut *mut sk_buff skb, unsigned int,
// Devlink parameters, etc
    pub ctx): *mut devlink_param_gset_ctx,
    pub ctx): *mut devlink_param_gset_ctx,
    pub extack): *mut netlink_ext_ack,
    pub pool_info): *mut devlink_sb_pool_info,
    pub extack): *mut netlink_ext_ack,
    pub p_threshold): *mut u32,
    pub extack): *mut netlink_ext_ack,
    pub p_threshold): *mut *mut u16 p_pool_index, u32,
    pub extack): *mut netlink_ext_ack,
    pub sb_index): c_uint,
    pub sb_index): c_uint,
    pub p_max): *mut *mut u32 p_cur, u32,
    pub p_max): *mut *mut u32 p_cur, u32,
//
// MTU change functionality. Switches can also adjust their MRU through
// this method. By MTU, one understands the SDU (L2 payload) length.
// If the switch needs to account for the DSA tag on the CPU port, this
// method needs to do so privately.
//
    pub new_mtu): c_int,
    pub port): *mut *mut *mut int (port_max_mtu)(struct dsa_switch ds, int,
//
// LAG integration
//
    pub port): *mut *mut *mut int (port_lag_change)(struct dsa_switch ds, int,
    pub extack): *mut netlink_ext_ack,
    pub lag): dsa_lag,
//
// HSR integration
//
    pub extack): *mut netlink_ext_ack,
    pub hsr): *mut net_device,
//
// MRP integration
//
    pub mrp): *const switchdev_obj_mrp,
    pub mrp): *const switchdev_obj_mrp,
    pub mrp): *const switchdev_obj_ring_role_mrp,
    pub mrp): *const switchdev_obj_ring_role_mrp,
//
// tag_8021q operations
//
    pub flags): u16,
    pub vid): *mut *mut *mut int (tag_8021q_vlan_del)(struct dsa_switch ds, int port, u16,
//
// DSA conduit tracking operations
//
    pub operational): bool,
}

extern "C" {
    pub fn dsa_devlink_resources_unregister(ds: *mut dsa_switch);
}
extern "C" {
    pub fn dsa_devlink_region_destroy(region: *mut devlink_region);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsa_devlink_priv {
    pub ds: *mut dsa_switch,
}

// Keep inline for faster access in hot path

// All DSA tags that push the EtherType to the right (basically all except tail
// tags, which don't break dissection) can be treated the same from the
// perspective of the flow dissector.
//
// We need to return:
// - offset: the (B - A) difference between:
// A. the position of the real EtherType and
// B. the current skb->data (aka ETH_HLEN bytes into the frame, aka 2 bytes
// after the normal EtherType was supposed to be)
// The offset in bytes is exactly equal to the tagger overhead (and half of
// that, in __be16 shorts).
//
// - proto: the value of the real EtherType.
//

// offset = tag_len;
// proto = ((__be16 *)skb->data)[(tag_len / 2) - 1];

extern "C" {
    pub fn dsa_unregister_switch(ds: *mut dsa_switch);
}
extern "C" {
    pub fn dsa_register_switch(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn dsa_switch_shutdown(ds: *mut dsa_switch);
}
extern "C" {
    pub fn dsa_flush_workqueue();
}

extern "C" {
    pub fn dsa_switch_suspend(ds: *mut dsa_switch) -> c_int;
}
extern "C" {
    pub fn dsa_switch_resume(ds: *mut dsa_switch) -> c_int;
}

extern "C" {
    pub fn dsa_user_dev_check(dev: *const net_device) -> bool;
}

extern "C" {
    pub fn dsa_enqueue_skb(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t;
}
extern "C" {
    pub fn dsa_port_phylink_mac_change(ds: *mut dsa_switch, port: c_int, up: bool);
}
extern "C" {
    pub fn dsa_supports_eee(ds: *mut dsa_switch, port: c_int) -> bool;
}
