//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/devlink.h
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
// include/net/devlink.h - Network physical device Netlink interface
// Copyright (c) 2016 Mellanox Technologies. All rights reserved.
// Copyright (c) 2016 Jiri Pirko <jiri@mellanox.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_phys_attrs {
    pub group".: *mut *mut u32 port_number; / Same value as "split,
// A physical port which is visible to the user
// for a given port flavour.
//
    pub /: *mut *mut u32 split_subport_number; / If the port is split, this is the number of subport.,
}

//
// struct devlink_port_pci_pf_attrs - devlink port's PCI PF attributes
// @controller: Associated controller number
// @pf: associated PCI function number for the devlink port instance
// @external: when set, indicates if a port is for an external host controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_pci_pf_attrs {
    pub controller: u32,
    pub pf: u16,
    pub external:1: u8,
}

//
// struct devlink_port_pci_vf_attrs - devlink port's PCI VF attributes
// @controller: Associated controller number
// @pf: associated PCI function number for the devlink port instance
// @vf: associated PCI VF number of a PF for the devlink port instance;
// VF number starts from 0 for the first PCI virtual function
// @external: when set, indicates if a port is for an external host controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_pci_vf_attrs {
    pub controller: u32,
    pub pf: u16,
    pub vf: u16,
    pub external:1: u8,
}

//
// struct devlink_port_pci_sf_attrs - devlink port's PCI SF attributes
// @controller: Associated controller number
// @sf: associated SF number of a PF for the devlink port instance
// @pf: associated PCI function number for the devlink port instance
// @external: when set, indicates if a port is for an external host controller.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_pci_sf_attrs {
    pub controller: u32,
    pub sf: u32,
    pub pf: u16,
    pub external:1: u8,
}

//
// struct devlink_port_attrs - devlink port object
// @flavour: flavour of the port
// @split: indicates if this is split port
// @splittable: indicates if the port can be split.
// @no_phys_port_name: skip automatic phys_port_name generation; for
// compatibility only, newly added driver/port instance
// should never set this.
// @lanes: maximum number of lanes the port supports. 0 value is not passed to netlink.
// @switch_id: if the port is part of switch, this is buffer with ID, otherwise this is NULL
// @phys: physical port attributes
// @pci_pf: PCI PF port attributes
// @pci_vf: PCI VF port attributes
// @pci_sf: PCI SF port attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_attrs {
    pub lanes: u32,
    pub flavour: devlink_port_flavour,
    pub switch_id: netdev_phys_item_id,
    pub phys: devlink_port_phys_attrs,
    pub pci_pf: devlink_port_pci_pf_attrs,
    pub pci_vf: devlink_port_pci_vf_attrs,
    pub pci_sf: devlink_port_pci_sf_attrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_rate {
    pub list: list_head,
    pub type: devlink_rate_type,
    pub devlink: *mut devlink,
    pub priv: *mut c_void,
    pub tx_share: u64,
    pub tx_max: u64,
    pub parent: *mut devlink_rate,
    pub devlink_port: *mut devlink_port,
    pub name: *mut c_char,
    pub refcnt: refcount_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port {
    pub list: list_head,
    pub region_list: list_head,
    pub resource_list: list_head,
    pub devlink: *mut devlink,
    pub ops: *const devlink_port_ops,
    pub index: c_uint,
    pub type_eth/ib: *mut *mut spinlock_t type_lock; / Protects type and,
// structures consistency.
//
    pub type: devlink_port_type,
    pub desired_type: devlink_port_type,
    pub netdev: *mut net_device,
    pub ifindex: c_int,
    pub ifname: [c_char; IFNAMSIZ],
    pub type_eth: },
    pub ibdev: *mut ib_device,
    pub type_ib: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_new_attrs {
    pub flavour: devlink_port_flavour,
    pub port_index: c_uint,
    pub controller: u32,
    pub sfnum: u32,
    pub pfnum: u16,
}

//
// struct devlink_linecard_ops - Linecard operations
// @provision: callback to provision the linecard slot with certain
// type of linecard. As a result of this operation,
// driver is expected to eventually (could be after
// the function call returns) call one of:
// devlink_linecard_provision_set()
// devlink_linecard_provision_fail()
// @unprovision: callback to unprovision the linecard slot. As a result
// of this operation, driver is expected to eventually
// (could be after the function call returns) call
// devlink_linecard_provision_clear()
// devlink_linecard_provision_fail()
// @same_provision: callback to ask the driver if linecard is already
// provisioned in the same way user asks this linecard to be
// provisioned.
// @types_count: callback to get number of supported types
// @types_get: callback to get next type in list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_linecard_ops {
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub type_priv): *const *const char type, void,
    pub priv): *mut c_void,
    pub type_priv): *const c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_sb_pool_info {
    pub pool_type: devlink_sb_pool_type,
    pub size: u32,
    pub threshold_type: devlink_sb_threshold_type,
    pub cell_size: u32,
}

//
// struct devlink_dpipe_field - dpipe field object
// @name: field name
// @id: index inside the headers field array
// @bitwidth: bitwidth
// @mapping_type: mapping type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_field {
    pub name: *const c_char,
    pub id: c_uint,
    pub bitwidth: c_uint,
    pub mapping_type: devlink_dpipe_field_mapping_type,
}

//
// struct devlink_dpipe_header - dpipe header object
// @name: header name
// @id: index, global/local determined by global bit
// @fields: fields
// @fields_count: number of fields
// @global: indicates if header is shared like most protocol header
// or driver specific
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_header {
    pub name: *const c_char,
    pub id: c_uint,
    pub fields: *mut devlink_dpipe_field,
    pub fields_count: c_uint,
    pub global: bool,
}

//
// struct devlink_dpipe_match - represents match operation
// @type: type of match
// @header_index: header index (packets can have several headers of same
// type like in case of tunnels)
// @header: header
// @field_id: field index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_match {
    pub type: devlink_dpipe_match_type,
    pub header_index: c_uint,
    pub header: *mut devlink_dpipe_header,
    pub field_id: c_uint,
}

//
// struct devlink_dpipe_action - represents action operation
// @type: type of action
// @header_index: header index (packets can have several headers of same
// type like in case of tunnels)
// @header: header
// @field_id: field index
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_action {
    pub type: devlink_dpipe_action_type,
    pub header_index: c_uint,
    pub header: *mut devlink_dpipe_header,
    pub field_id: c_uint,
}

//
// struct devlink_dpipe_value - represents value of match/action
// @action: action
// @match: match
// @mapping_value: in case the field has some mapping this value
// specified the mapping value
// @mapping_valid: specify if mapping value is valid
// @value_size: value size
// @value: value
// @mask: bit mask
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_value {
    pub action: *mut devlink_dpipe_action,
    pub match: *mut devlink_dpipe_match,
}

//
// struct devlink_dpipe_entry - table entry object
// @index: index of the entry in the table
// @match_values: match values
// @match_values_count: count of matches tuples
// @action_values: actions values
// @action_values_count: count of actions values
// @counter: value of counter
// @counter_valid: Specify if value is valid from hardware
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_entry {
    pub index: u64,
    pub match_values: *mut devlink_dpipe_value,
    pub match_values_count: c_uint,
    pub action_values: *mut devlink_dpipe_value,
    pub action_values_count: c_uint,
    pub counter: u64,
    pub counter_valid: bool,
}

//
// struct devlink_dpipe_dump_ctx - context provided to driver in order
// to dump
// @info: info
// @cmd: devlink command
// @skb: skb
// @nest: top attribute
// @hdr: hdr
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_dump_ctx {
    pub info: *mut genl_info,
    pub cmd: devlink_command,
    pub skb: *mut sk_buff,
    pub nest: *mut nlattr,
    pub hdr: *mut c_void,
}

//
// struct devlink_dpipe_table - table object
// @priv: private
// @name: table name
// @counters_enabled: indicates if counters are active
// @counter_control_extern: indicates if counter control is in dpipe or
// external tool
// @resource_valid: Indicate that the resource id is valid
// @resource_id: relative resource this table is related to
// @resource_units: number of resource's unit consumed per table's entry
// @table_ops: table operations
// @rcu: rcu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_table {
    pub priv: *mut c_void,
// private:
    pub list: list_head,
// public:
    pub name: *const c_char,
    pub counters_enabled: bool,
    pub counter_control_extern: bool,
    pub resource_valid: bool,
    pub resource_id: u64,
    pub resource_units: u64,
    pub table_ops: *const devlink_dpipe_table_ops,
    pub rcu: rcu_head,
}

//
// struct devlink_dpipe_table_ops - dpipe_table ops
// @actions_dump: dumps all tables actions
// @matches_dump: dumps all tables matches
// @entries_dump: dumps all active entries in the table
// @counters_set_update:  when changing the counter status hardware sync
// maybe needed to allocate/free counter related
// resources
// @size_get: get size
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_table_ops {
    pub skb): *mut *mut *mut int (actions_dump)(void priv, struct sk_buff,
    pub skb): *mut *mut *mut int (matches_dump)(void priv, struct sk_buff,
    pub dump_ctx): *mut devlink_dpipe_dump_ctx,
    pub enable): *mut *mut *mut int (counters_set_update)(void priv, bool,
    pub priv): *mut *mut u64 (size_get)(void,
}

//
// struct devlink_dpipe_headers - dpipe headers
// @headers: header array can be shared (global bit) or driver specific
// @headers_count: count of headers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_dpipe_headers {
    pub headers: *mut devlink_dpipe_header,
    pub headers_count: c_uint,
}

//
// struct devlink_resource_size_params - resource's size parameters
// @size_min: minimum size which can be set
// @size_max: maximum size which can be set
// @size_granularity: size granularity
// @unit: resource's basic unit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_resource_size_params {
    pub size_min: u64,
    pub size_max: u64,
    pub size_granularity: u64,
    pub unit: devlink_resource_unit,
}

extern "C" {
    pub fn devlink_resource_occ_get_t(priv: *mut c_void) -> typedef u64;
}
pub const DEVLINK_RESOURCE_ID_PARENT_TOP: c_int = 0;

pub const __DEVLINK_PARAM_MAX_STRING_VALUE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devlink_param_type {
    DEVLINK_PARAM_TYPE_U8 = DEVLINK_VAR_ATTR_TYPE_U8,
    DEVLINK_PARAM_TYPE_U16 = DEVLINK_VAR_ATTR_TYPE_U16,
    DEVLINK_PARAM_TYPE_U32 = DEVLINK_VAR_ATTR_TYPE_U32,
    DEVLINK_PARAM_TYPE_U64 = DEVLINK_VAR_ATTR_TYPE_U64,
    DEVLINK_PARAM_TYPE_STRING = DEVLINK_VAR_ATTR_TYPE_STRING,
    DEVLINK_PARAM_TYPE_BOOL = DEVLINK_VAR_ATTR_TYPE_FLAG,
    DEVLINK_PARAM_TYPE_U64_ARRAY = DEVLINK_VAR_ATTR_TYPE_U64_ARRAY,
}

pub const __DEVLINK_PARAM_MAX_ARRAY_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_param_u64_array {
    pub size: u64,
    pub val: [u64; __DEVLINK_PARAM_MAX_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union devlink_param_value {
    pub vu8: u8,
    pub vu16: u16,
    pub vu32: u32,
    pub vu64: u64,
    pub vstr: [c_char; __DEVLINK_PARAM_MAX_STRING_VALUE],
    pub vbool: bool,
    pub u64arr: devlink_param_u64_array,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_param_gset_ctx {
    pub val: devlink_param_value,
    pub cmode: devlink_param_cmode,
}

//
// struct devlink_flash_notify - devlink dev flash notify data
// @status_msg: current status string
// @component: firmware component being updated
// @done: amount of work completed of total amount
// @total: amount of work expected to be done
// @timeout: expected max timeout in seconds
//
// These are values to be given to userland to be displayed in order
// to show current activity in a firmware update process.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_flash_notify {
    pub status_msg: *const c_char,
    pub component: *const c_char,
    pub done: c_ulong,
    pub total: c_ulong,
    pub timeout: c_ulong,
}

//
// struct devlink_param - devlink configuration parameter data
// @id: devlink parameter id number
// @name: name of the parameter
// @generic: indicates if the parameter is generic or driver specific
// @type: parameter type
// @supported_cmodes: bitmap of supported configuration modes
// @get: get parameter value, used for runtime and permanent
// configuration modes
// @set: set parameter value, used for runtime and permanent
// configuration modes
// @validate: validate input value is applicable (within value range, etc.)
// @get_default: get parameter default value, used for runtime and permanent
// configuration modes
// @reset_default: reset parameter to default value, used for runtime and permanent
// configuration modes
//
// This struct should be used by the driver to fill the data for
// a parameter it registers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_param {
    pub id: u32,
    pub name: *const c_char,
    pub generic: bool,
    pub type: devlink_param_type,
    pub supported_cmodes: c_ulong,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_param_item {
    pub list: list_head,
    pub param: *const devlink_param,
    pub driverinit_value: devlink_param_value,
    pub driverinit_value_valid: bool,
    pub reachable: *mut *mut devlink_param_value driverinit_value_new; / Not,
// until reload.
//
    pub driverinit_value_new_valid: bool,
    pub driverinit_default: devlink_param_value,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devlink_param_generic_id {
    DEVLINK_PARAM_GENERIC_ID_INT_ERR_RESET,
    DEVLINK_PARAM_GENERIC_ID_MAX_MACS,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_SRIOV,
    DEVLINK_PARAM_GENERIC_ID_REGION_SNAPSHOT,
    DEVLINK_PARAM_GENERIC_ID_IGNORE_ARI,
    DEVLINK_PARAM_GENERIC_ID_MSIX_VEC_PER_PF_MAX,
    DEVLINK_PARAM_GENERIC_ID_MSIX_VEC_PER_PF_MIN,
    DEVLINK_PARAM_GENERIC_ID_FW_LOAD_POLICY,
    DEVLINK_PARAM_GENERIC_ID_RESET_DEV_ON_DRV_PROBE,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_ROCE,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_REMOTE_DEV_RESET,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_ETH,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_RDMA,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_VNET,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_IWARP,
    DEVLINK_PARAM_GENERIC_ID_IO_EQ_SIZE,
    DEVLINK_PARAM_GENERIC_ID_EVENT_EQ_SIZE,
    DEVLINK_PARAM_GENERIC_ID_ENABLE_PHC,
    DEVLINK_PARAM_GENERIC_ID_CLOCK_ID,
    DEVLINK_PARAM_GENERIC_ID_TOTAL_VFS,
    DEVLINK_PARAM_GENERIC_ID_NUM_DOORBELLS,
    DEVLINK_PARAM_GENERIC_ID_MAX_MAC_PER_VF,
    DEVLINK_PARAM_GENERIC_ID_MAX_SFS,

// add new param generic ids above here
    __DEVLINK_PARAM_GENERIC_ID_MAX,
    DEVLINK_PARAM_GENERIC_ID_MAX = __DEVLINK_PARAM_GENERIC_ID_MAX - 1,
}

// Identifier of board design

// Revision of board design

// Maker of the board

// Part number of the board and its components

// Part number, identifier of asic design

// Revision of asic design

// Overall FW version

// Control processor FW version

// FW interface specification version

// Data path microcode controlling high-speed packet processing

// UNDI software version

// NCSI support/handler version

// FW parameter set id

// RoCE FW version

// Firmware bundle identifier

// Bootloader

//
// struct devlink_flash_update_params - Flash Update parameters
// @fw: pointer to the firmware data to update from
// @component: the flash component to update
// @overwrite_mask: which types of flash update are supported (may be %0)
//
// With the exception of fw, drivers must opt-in to parameters by
// setting the appropriate bit in the supported_flash_update_params field in
// their devlink_ops structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_flash_update_params {
    pub fw: *const firmware,
    pub component: *const c_char,
    pub overwrite_mask: u32,
}

//
// struct devlink_region_ops - Region operations
// @name: region name
// @destructor: callback used to free snapshot memory when deleting
// @snapshot: callback to request an immediate snapshot. On success,
// the data variable must be updated to point to the snapshot data.
// The function will be called while the devlink instance lock is
// held.
// @read: callback to directly read a portion of the region. On success,
// the data pointer will be updated with the contents of the
// requested portion of the region. The function will be called
// while the devlink instance lock is held.
// @priv: Pointer to driver private data for the region operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_region_ops {
    pub name: *const c_char,
    pub data): *const *const void (destructor)(void,
    pub data): *mut u8,
    pub data): *mut u64 offset, u32 size, u8,
    pub priv: *mut c_void,
}

//
// struct devlink_port_region_ops - Region operations for a port
// @name: region name
// @destructor: callback used to free snapshot memory when deleting
// @snapshot: callback to request an immediate snapshot. On success,
// the data variable must be updated to point to the snapshot data.
// The function will be called while the devlink instance lock is
// held.
// @read: callback to directly read a portion of the region. On success,
// the data pointer will be updated with the contents of the
// requested portion of the region. The function will be called
// while the devlink instance lock is held.
// @priv: Pointer to driver private data for the region operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_region_ops {
    pub name: *const c_char,
    pub data): *const *const void (destructor)(void,
    pub data): *mut u8,
    pub data): *mut u64 offset, u32 size, u8,
    pub priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devlink_health_reporter_state {
    DEVLINK_HEALTH_REPORTER_STATE_HEALTHY,
    DEVLINK_HEALTH_REPORTER_STATE_ERROR,
}

//
// struct devlink_health_reporter_ops - Reporter operations
// @name: reporter name
// @recover: callback to recover from reported error
// if priv_ctx is NULL, run a full recover
// @dump: callback to dump an object
// if priv_ctx is NULL, run a full dump
// @diagnose: callback to diagnose the current status
// @test: callback to trigger a test event
// @default_graceful_period: default min time (in msec)
// between recovery attempts
// @default_burst_period: default time (in msec) for
// error recoveries before starting the grace period
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_health_reporter_ops {
    pub name: *mut c_char,
    pub extack): *mut *mut void priv_ctx, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub default_graceful_period: u64,
    pub default_burst_period: u64,
}

//
// struct devlink_trap_metadata - Packet trap metadata.
// @trap_name: Trap name.
// @trap_group_name: Trap group name.
// @input_dev: Input netdevice.
// @dev_tracker: refcount tracker for @input_dev.
// @fa_cookie: Flow action user cookie.
// @trap_type: Trap type.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_trap_metadata {
    pub trap_name: *const c_char,
    pub trap_group_name: *const c_char,
    pub input_dev: *mut net_device,
    pub dev_tracker: netdevice_tracker,
    pub fa_cookie: *const flow_action_cookie,
    pub trap_type: devlink_trap_type,
}

//
// struct devlink_trap_policer - Immutable packet trap policer attributes.
// @id: Policer identifier.
// @init_rate: Initial rate in packets / sec.
// @init_burst: Initial burst size in packets.
// @max_rate: Maximum rate.
// @min_rate: Minimum rate.
// @max_burst: Maximum burst size.
// @min_burst: Minimum burst size.
//
// Describes immutable attributes of packet trap policers that drivers register
// with devlink.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_trap_policer {
    pub id: u32,
    pub init_rate: u64,
    pub init_burst: u64,
    pub max_rate: u64,
    pub min_rate: u64,
    pub max_burst: u64,
    pub min_burst: u64,
}

//
// struct devlink_trap_group - Immutable packet trap group attributes.
// @name: Trap group name.
// @id: Trap group identifier.
// @generic: Whether the trap group is generic or not.
// @init_policer_id: Initial policer identifier.
//
// Describes immutable attributes of packet trap groups that drivers register
// with devlink.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_trap_group {
    pub name: *const c_char,
    pub id: u16,
    pub generic: bool,
    pub init_policer_id: u32,
}

//
// struct devlink_trap - Immutable packet trap attributes.
// @type: Trap type.
// @init_action: Initial trap action.
// @generic: Whether the trap is generic or not.
// @id: Trap identifier.
// @name: Trap name.
// @init_group_id: Initial group identifier.
// @metadata_cap: Metadata types that can be provided by the trap.
//
// Describes immutable attributes of packet traps that drivers register with
// devlink.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_trap {
    pub type: devlink_trap_type,
    pub init_action: devlink_trap_action,
    pub generic: bool,
    pub id: u16,
    pub name: *const c_char,
    pub init_group_id: u16,
    pub metadata_cap: u32,
}

// All traps must be documented in
// Documentation/networking/devlink/devlink-trap.rst
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devlink_trap_generic_id {
    DEVLINK_TRAP_GENERIC_ID_SMAC_MC,
    DEVLINK_TRAP_GENERIC_ID_VLAN_TAG_MISMATCH,
    DEVLINK_TRAP_GENERIC_ID_INGRESS_VLAN_FILTER,
    DEVLINK_TRAP_GENERIC_ID_INGRESS_STP_FILTER,
    DEVLINK_TRAP_GENERIC_ID_EMPTY_TX_LIST,
    DEVLINK_TRAP_GENERIC_ID_PORT_LOOPBACK_FILTER,
    DEVLINK_TRAP_GENERIC_ID_BLACKHOLE_ROUTE,
    DEVLINK_TRAP_GENERIC_ID_TTL_ERROR,
    DEVLINK_TRAP_GENERIC_ID_TAIL_DROP,
    DEVLINK_TRAP_GENERIC_ID_NON_IP_PACKET,
    DEVLINK_TRAP_GENERIC_ID_UC_DIP_MC_DMAC,
    DEVLINK_TRAP_GENERIC_ID_DIP_LB,
    DEVLINK_TRAP_GENERIC_ID_SIP_MC,
    DEVLINK_TRAP_GENERIC_ID_SIP_LB,
    DEVLINK_TRAP_GENERIC_ID_CORRUPTED_IP_HDR,
    DEVLINK_TRAP_GENERIC_ID_IPV4_SIP_BC,
    DEVLINK_TRAP_GENERIC_ID_IPV6_MC_DIP_RESERVED_SCOPE,
    DEVLINK_TRAP_GENERIC_ID_IPV6_MC_DIP_INTERFACE_LOCAL_SCOPE,
    DEVLINK_TRAP_GENERIC_ID_MTU_ERROR,
    DEVLINK_TRAP_GENERIC_ID_UNRESOLVED_NEIGH,
    DEVLINK_TRAP_GENERIC_ID_RPF,
    DEVLINK_TRAP_GENERIC_ID_REJECT_ROUTE,
    DEVLINK_TRAP_GENERIC_ID_IPV4_LPM_UNICAST_MISS,
    DEVLINK_TRAP_GENERIC_ID_IPV6_LPM_UNICAST_MISS,
    DEVLINK_TRAP_GENERIC_ID_NON_ROUTABLE,
    DEVLINK_TRAP_GENERIC_ID_DECAP_ERROR,
    DEVLINK_TRAP_GENERIC_ID_OVERLAY_SMAC_MC,
    DEVLINK_TRAP_GENERIC_ID_INGRESS_FLOW_ACTION_DROP,
    DEVLINK_TRAP_GENERIC_ID_EGRESS_FLOW_ACTION_DROP,
    DEVLINK_TRAP_GENERIC_ID_STP,
    DEVLINK_TRAP_GENERIC_ID_LACP,
    DEVLINK_TRAP_GENERIC_ID_LLDP,
    DEVLINK_TRAP_GENERIC_ID_IGMP_QUERY,
    DEVLINK_TRAP_GENERIC_ID_IGMP_V1_REPORT,
    DEVLINK_TRAP_GENERIC_ID_IGMP_V2_REPORT,
    DEVLINK_TRAP_GENERIC_ID_IGMP_V3_REPORT,
    DEVLINK_TRAP_GENERIC_ID_IGMP_V2_LEAVE,
    DEVLINK_TRAP_GENERIC_ID_MLD_QUERY,
    DEVLINK_TRAP_GENERIC_ID_MLD_V1_REPORT,
    DEVLINK_TRAP_GENERIC_ID_MLD_V2_REPORT,
    DEVLINK_TRAP_GENERIC_ID_MLD_V1_DONE,
    DEVLINK_TRAP_GENERIC_ID_IPV4_DHCP,
    DEVLINK_TRAP_GENERIC_ID_IPV6_DHCP,
    DEVLINK_TRAP_GENERIC_ID_ARP_REQUEST,
    DEVLINK_TRAP_GENERIC_ID_ARP_RESPONSE,
    DEVLINK_TRAP_GENERIC_ID_ARP_OVERLAY,
    DEVLINK_TRAP_GENERIC_ID_IPV6_NEIGH_SOLICIT,
    DEVLINK_TRAP_GENERIC_ID_IPV6_NEIGH_ADVERT,
    DEVLINK_TRAP_GENERIC_ID_IPV4_BFD,
    DEVLINK_TRAP_GENERIC_ID_IPV6_BFD,
    DEVLINK_TRAP_GENERIC_ID_IPV4_OSPF,
    DEVLINK_TRAP_GENERIC_ID_IPV6_OSPF,
    DEVLINK_TRAP_GENERIC_ID_IPV4_BGP,
    DEVLINK_TRAP_GENERIC_ID_IPV6_BGP,
    DEVLINK_TRAP_GENERIC_ID_IPV4_VRRP,
    DEVLINK_TRAP_GENERIC_ID_IPV6_VRRP,
    DEVLINK_TRAP_GENERIC_ID_IPV4_PIM,
    DEVLINK_TRAP_GENERIC_ID_IPV6_PIM,
    DEVLINK_TRAP_GENERIC_ID_UC_LB,
    DEVLINK_TRAP_GENERIC_ID_LOCAL_ROUTE,
    DEVLINK_TRAP_GENERIC_ID_EXTERNAL_ROUTE,
    DEVLINK_TRAP_GENERIC_ID_IPV6_UC_DIP_LINK_LOCAL_SCOPE,
    DEVLINK_TRAP_GENERIC_ID_IPV6_DIP_ALL_NODES,
    DEVLINK_TRAP_GENERIC_ID_IPV6_DIP_ALL_ROUTERS,
    DEVLINK_TRAP_GENERIC_ID_IPV6_ROUTER_SOLICIT,
    DEVLINK_TRAP_GENERIC_ID_IPV6_ROUTER_ADVERT,
    DEVLINK_TRAP_GENERIC_ID_IPV6_REDIRECT,
    DEVLINK_TRAP_GENERIC_ID_IPV4_ROUTER_ALERT,
    DEVLINK_TRAP_GENERIC_ID_IPV6_ROUTER_ALERT,
    DEVLINK_TRAP_GENERIC_ID_PTP_EVENT,
    DEVLINK_TRAP_GENERIC_ID_PTP_GENERAL,
    DEVLINK_TRAP_GENERIC_ID_FLOW_ACTION_SAMPLE,
    DEVLINK_TRAP_GENERIC_ID_FLOW_ACTION_TRAP,
    DEVLINK_TRAP_GENERIC_ID_EARLY_DROP,
    DEVLINK_TRAP_GENERIC_ID_VXLAN_PARSING,
    DEVLINK_TRAP_GENERIC_ID_LLC_SNAP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_VLAN_PARSING,
    DEVLINK_TRAP_GENERIC_ID_PPPOE_PPP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_MPLS_PARSING,
    DEVLINK_TRAP_GENERIC_ID_ARP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_IP_1_PARSING,
    DEVLINK_TRAP_GENERIC_ID_IP_N_PARSING,
    DEVLINK_TRAP_GENERIC_ID_GRE_PARSING,
    DEVLINK_TRAP_GENERIC_ID_UDP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_TCP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_IPSEC_PARSING,
    DEVLINK_TRAP_GENERIC_ID_SCTP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_DCCP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_GTP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_ESP_PARSING,
    DEVLINK_TRAP_GENERIC_ID_BLACKHOLE_NEXTHOP,
    DEVLINK_TRAP_GENERIC_ID_DMAC_FILTER,
    DEVLINK_TRAP_GENERIC_ID_EAPOL,
    DEVLINK_TRAP_GENERIC_ID_LOCKED_PORT,

// Add new generic trap IDs above
    __DEVLINK_TRAP_GENERIC_ID_MAX,
    DEVLINK_TRAP_GENERIC_ID_MAX = __DEVLINK_TRAP_GENERIC_ID_MAX - 1,
}

// All trap groups must be documented in
// Documentation/networking/devlink/devlink-trap.rst
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devlink_trap_group_generic_id {
    DEVLINK_TRAP_GROUP_GENERIC_ID_L2_DROPS,
    DEVLINK_TRAP_GROUP_GENERIC_ID_L3_DROPS,
    DEVLINK_TRAP_GROUP_GENERIC_ID_L3_EXCEPTIONS,
    DEVLINK_TRAP_GROUP_GENERIC_ID_BUFFER_DROPS,
    DEVLINK_TRAP_GROUP_GENERIC_ID_TUNNEL_DROPS,
    DEVLINK_TRAP_GROUP_GENERIC_ID_ACL_DROPS,
    DEVLINK_TRAP_GROUP_GENERIC_ID_STP,
    DEVLINK_TRAP_GROUP_GENERIC_ID_LACP,
    DEVLINK_TRAP_GROUP_GENERIC_ID_LLDP,
    DEVLINK_TRAP_GROUP_GENERIC_ID_MC_SNOOPING,
    DEVLINK_TRAP_GROUP_GENERIC_ID_DHCP,
    DEVLINK_TRAP_GROUP_GENERIC_ID_NEIGH_DISCOVERY,
    DEVLINK_TRAP_GROUP_GENERIC_ID_BFD,
    DEVLINK_TRAP_GROUP_GENERIC_ID_OSPF,
    DEVLINK_TRAP_GROUP_GENERIC_ID_BGP,
    DEVLINK_TRAP_GROUP_GENERIC_ID_VRRP,
    DEVLINK_TRAP_GROUP_GENERIC_ID_PIM,
    DEVLINK_TRAP_GROUP_GENERIC_ID_UC_LB,
    DEVLINK_TRAP_GROUP_GENERIC_ID_LOCAL_DELIVERY,
    DEVLINK_TRAP_GROUP_GENERIC_ID_EXTERNAL_DELIVERY,
    DEVLINK_TRAP_GROUP_GENERIC_ID_IPV6,
    DEVLINK_TRAP_GROUP_GENERIC_ID_PTP_EVENT,
    DEVLINK_TRAP_GROUP_GENERIC_ID_PTP_GENERAL,
    DEVLINK_TRAP_GROUP_GENERIC_ID_ACL_SAMPLE,
    DEVLINK_TRAP_GROUP_GENERIC_ID_ACL_TRAP,
    DEVLINK_TRAP_GROUP_GENERIC_ID_PARSER_ERROR_DROPS,
    DEVLINK_TRAP_GROUP_GENERIC_ID_EAPOL,

// Add new generic trap group IDs above
    __DEVLINK_TRAP_GROUP_GENERIC_ID_MAX,
    DEVLINK_TRAP_GROUP_GENERIC_ID_MAX =
    __DEVLINK_TRAP_GROUP_GENERIC_ID_MAX - 1,
}

// device supports reload operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_ops {
//
// @supported_flash_update_params:
// mask of parameters supported by the driver's .flash_update
// implementation.
//
    pub supported_flash_update_params: u32,
    pub reload_actions: c_ulong,
    pub reload_limits: c_ulong,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub pool_info): *mut devlink_sb_pool_info,
    pub extack): *mut netlink_ext_ack,
    pub p_threshold): *mut u32,
    pub extack): *mut u32 threshold, struct netlink_ext_ack,
    pub p_threshold): *mut *mut u16 p_pool_index, u32,
    pub extack): *mut netlink_ext_ack,
    pub sb_index): c_uint,
    pub sb_index): c_uint,
    pub p_max): *mut *mut u32 p_cur, u32,
    pub p_max): *mut *mut u32 p_cur, u32,
    pub p_mode): *mut *mut *mut int (eswitch_mode_get)(struct devlink devlink, u16,
    pub extack): *mut netlink_ext_ack,
    pub p_inline_mode): *mut *mut *mut int (eswitch_inline_mode_get)(struct devlink devlink, u8,
    pub extack): *mut netlink_ext_ack,
    pub p_encap_mode): *mut devlink_eswitch_encap_mode,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
//
// @flash_update: Device flash update function
//
// Used to perform a flash update for the device. The set of
// parameters supported by the driver should be set in
// supported_flash_update_params.
//
    pub extack): *mut netlink_ext_ack,
//
// @trap_init: Trap initialization function.
//
// Should be used by device drivers to initialize the trap in the
// underlying device. Drivers should also store the provided trap
// context, so that they could efficiently pass it to
// devlink_trap_report() when the trap is triggered.
//
    pub trap_ctx): *const *const devlink_trap trap, void,
//
// @trap_fini: Trap de-initialization function.
//
// Should be used by device drivers to de-initialize the trap in the
// underlying device.
//
    pub trap_ctx): *const *const devlink_trap trap, void,
//
// @trap_action_set: Trap action set function.
//
    pub extack): *mut netlink_ext_ack,
//
// @trap_group_init: Trap group initialization function.
//
// Should be used by device drivers to initialize the trap group in the
// underlying device.
//
    pub group): *const devlink_trap_group,
//
// @trap_group_set: Trap group parameters set function.
//
// Note: @policer can be NULL when a policer is being unbound from
// @group.
//
    pub extack): *mut netlink_ext_ack,
//
// @trap_group_action_set: Trap group action set function.
//
// If this callback is populated, it will take precedence over looping
// over all traps in a group and calling .trap_action_set().
//
    pub extack): *mut netlink_ext_ack,
//
// @trap_drop_counter_get: Trap drop counter get function.
//
// Should be used by device drivers to report number of packets
// that have been dropped, and cannot be passed to the devlink
// subsystem by the underlying device.
//
    pub p_drops): *mut u64,
//
// @trap_policer_init: Trap policer initialization function.
//
// Should be used by device drivers to initialize the trap policer in
// the underlying device.
//
    pub policer): *const devlink_trap_policer,
//
// @trap_policer_fini: Trap policer de-initialization function.
//
// Should be used by device drivers to de-initialize the trap policer
// in the underlying device.
//
    pub policer): *const devlink_trap_policer,
//
// @trap_policer_set: Trap policer parameters set function.
//
    pub extack): *mut netlink_ext_ack,
//
// @trap_policer_counter_get: Trap policer counter get function.
//
// Should be used by device drivers to report number of packets dropped
// by the policer.
//
    pub p_drops): *mut u64,
//
// port_new() - Add a new port function of a specified flavor
// @devlink: Devlink instance
// @attrs: attributes of the new port
// @extack: extack for reporting error messages
// @devlink_port: pointer to store new devlink port pointer
//
// Devlink core will call this device driver function upon user request
// to create a new port function of a specified flavor and optional
// attributes
//
// Notes:
// - On success, drivers must register a port with devlink core
//
// Return: 0 on success, negative value otherwise.
//
    pub devlink_port): *mut devlink_port,
//
// Rate control callbacks.
//
    pub extack): *mut u64 tx_share, struct netlink_ext_ack,
    pub extack): *mut u64 tx_max, struct netlink_ext_ack,
    pub extack): *mut u32 tx_priority, struct netlink_ext_ack,
    pub extack): *mut u32 tx_weight, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut u64 tx_share, struct netlink_ext_ack,
    pub extack): *mut u64 tx_max, struct netlink_ext_ack,
    pub extack): *mut u32 tx_priority, struct netlink_ext_ack,
    pub extack): *mut u32 tx_weight, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
// Indicates if cross-device rate nodes are supported.
// This also requires a shared common ancestor object all devices that
// could share rate nodes are nested in.
// If enabled, rate operations may be called on an instance with only
// the common ancestor lock held and *without that instance lock held*.
// It is the driver's responsibility to ensure proper serialization
// with other operations.
//
    pub supported_cross_device_rate_nodes: bool,
//
// selftests_check() - queries if selftest is supported
// @devlink: devlink instance
// @id: test index
// @extack: extack for reporting error messages
//
// Return: true if test is supported by the driver
//
    pub extack): *mut netlink_ext_ack,
//
// selftest_run() - Runs a selftest
// @devlink: devlink instance
// @id: test index
// @extack: extack for reporting error messages
//
// Return: status of the test
//
    pub extack): *mut netlink_ext_ack,
}

// Devlink instance explicit locking
extern "C" {
    pub fn devl_lock(devlink: *mut devlink);
}
extern "C" {
    pub fn devl_trylock(devlink: *mut devlink) -> c_int;
}
extern "C" {
    pub fn devl_unlock(devlink: *mut devlink);
}
extern "C" {
    pub fn devl_assert_locked(devlink: *mut devlink);
}
extern "C" {
    pub fn devl_lock_is_held(devlink: *mut devlink) -> bool;
}
// This call is intended for software devices that can create
// devlink instances in other namespaces than init_net.
//
// Drivers that operate on real HW must use devlink_alloc() instead.
//
extern "C" {
    pub fn devlink_alloc_ns(_arg: ops, _arg: priv_size, _arg: &init_net, _arg: dev) -> return;
}
extern "C" {
    pub fn devl_register(devlink: *mut devlink) -> c_int;
}
extern "C" {
    pub fn devl_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_register(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_free(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_shd_put(devlink: *mut devlink);
}
//
// struct devlink_port_ops - Port operations
// @port_split: Callback used to split the port into multiple ones.
// @port_unsplit: Callback used to unsplit the port group back into
// a single port.
// @port_type_set: Callback used to set a type of a port.
// @port_del: Callback used to delete selected port along with related function.
// Devlink core calls this upon user request to delete
// a port previously created by devlink_ops->port_new().
// @port_fn_hw_addr_get: Callback used to set port function's hardware address.
// Should be used by device drivers to report
// the hardware address of a function managed
// by the devlink port.
// @port_fn_hw_addr_set: Callback used to set port function's hardware address.
// Should be used by device drivers to set the hardware
// address of a function managed by the devlink port.
// @port_fn_roce_get: Callback used to get port function's RoCE capability.
// Should be used by device drivers to report
// the current state of RoCE capability of a function
// managed by the devlink port.
// @port_fn_roce_set: Callback used to set port function's RoCE capability.
// Should be used by device drivers to enable/disable
// RoCE capability of a function managed
// by the devlink port.
// @port_fn_migratable_get: Callback used to get port function's migratable
// capability. Should be used by device drivers
// to report the current state of migratable capability
// of a function managed by the devlink port.
// @port_fn_migratable_set: Callback used to set port function's migratable
// capability. Should be used by device drivers
// to enable/disable migratable capability of
// a function managed by the devlink port.
// @port_fn_state_get: Callback used to get port function's state.
// Should be used by device drivers to report
// the current admin and operational state of a
// function managed by the devlink port.
// @port_fn_state_set: Callback used to get port function's state.
// Should be used by device drivers set
// the admin state of a function managed
// by the devlink port.
// @port_fn_ipsec_crypto_get: Callback used to get port function's ipsec_crypto
// capability. Should be used by device drivers
// to report the current state of ipsec_crypto
// capability of a function managed by the devlink
// port.
// @port_fn_ipsec_crypto_set: Callback used to set port function's ipsec_crypto
// capability. Should be used by device drivers to
// enable/disable ipsec_crypto capability of a
// function managed by the devlink port.
// @port_fn_ipsec_packet_get: Callback used to get port function's ipsec_packet
// capability. Should be used by device drivers
// to report the current state of ipsec_packet
// capability of a function managed by the devlink
// port.
// @port_fn_ipsec_packet_set: Callback used to set port function's ipsec_packet
// capability. Should be used by device drivers to
// enable/disable ipsec_packet capability of a
// function managed by the devlink port.
// @port_fn_max_io_eqs_get: Callback used to get port function's maximum number
// of event queues. Should be used by device drivers to
// report the maximum event queues of a function
// managed by the devlink port.
// @port_fn_max_io_eqs_set: Callback used to set port function's maximum number
// of event queues. Should be used by device drivers to
// configure maximum number of event queues
// of a function managed by the devlink port.
//
// Note: Driver should return -EOPNOTSUPP if it doesn't support
// port function (@port_fn_*) handling for a particular port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devlink_port_ops {
    pub extack): *mut unsigned int count, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub port_type): devlink_port_type,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut bool enable, struct netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
    pub extack): *mut netlink_ext_ack,
}

extern "C" {
    pub fn devlink_port_fini(devlink_port: *mut devlink_port);
}
extern "C" {
    pub fn devl_port_unregister(devlink_port: *mut devlink_port);
}
extern "C" {
    pub fn devlink_port_unregister(devlink_port: *mut devlink_port);
}
extern "C" {
    pub fn devlink_port_type_eth_set(devlink_port: *mut devlink_port);
}
extern "C" {
    pub fn devlink_port_type_clear(devlink_port: *mut devlink_port);
}
extern "C" {
    pub fn devl_rate_leaf_destroy(devlink_port: *mut devlink_port);
}
extern "C" {
    pub fn devl_rate_nodes_destroy(devlink: *mut devlink);
}
extern "C" {
    pub fn devl_linecard_destroy(linecard: *mut devlink_linecard);
}
extern "C" {
    pub fn devlink_linecard_provision_clear(linecard: *mut devlink_linecard);
}
extern "C" {
    pub fn devlink_linecard_provision_fail(linecard: *mut devlink_linecard);
}
extern "C" {
    pub fn devlink_linecard_activate(linecard: *mut devlink_linecard);
}
extern "C" {
    pub fn devlink_linecard_deactivate(linecard: *mut devlink_linecard);
}
extern "C" {
    pub fn devl_sb_unregister(devlink: *mut devlink, sb_index: c_uint);
}
extern "C" {
    pub fn devlink_sb_unregister(devlink: *mut devlink, sb_index: c_uint);
}
extern "C" {
    pub fn devl_dpipe_headers_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_dpipe_entry_ctx_prepare(dump_ctx: *mut devlink_dpipe_dump_ctx) -> c_int;
}
extern "C" {
    pub fn devlink_dpipe_entry_ctx_close(dump_ctx: *mut devlink_dpipe_dump_ctx) -> c_int;
}
extern "C" {
    pub fn devlink_dpipe_entry_clear(entry: *mut devlink_dpipe_entry);
}
extern "C" {
    pub fn devl_resources_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_resources_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn devl_port_resources_unregister(devlink_port: *mut devlink_port);
}
extern "C" {
    pub fn devl_param_value_changed(devlink: *mut devlink, param_id: u32);
}
extern "C" {
    pub fn devl_region_destroy(region: *mut devlink_region);
}
extern "C" {
    pub fn devlink_region_destroy(region: *mut devlink_region);
}
extern "C" {
    pub fn devlink_region_snapshot_id_get(devlink: *mut devlink, id: *mut u32) -> c_int;
}
extern "C" {
    pub fn devlink_region_snapshot_id_put(devlink: *mut devlink, id: u32);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum devlink_info_version_type {
    DEVLINK_INFO_VERSION_TYPE_NONE,
    DEVLINK_INFO_VERSION_TYPE_COMPONENT, /* May be used as flash update
// component by name.
//
}

extern "C" {
    pub fn devlink_fmsg_obj_nest_start(fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn devlink_fmsg_obj_nest_end(fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn devlink_fmsg_pair_nest_start(fmsg: *mut devlink_fmsg, name: *const c_char);
}
extern "C" {
    pub fn devlink_fmsg_pair_nest_end(fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn devlink_fmsg_arr_pair_nest_end(fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn devlink_fmsg_binary_pair_nest_end(fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn devlink_fmsg_u32_put(fmsg: *mut devlink_fmsg, value: u32);
}
extern "C" {
    pub fn devlink_fmsg_string_put(fmsg: *mut devlink_fmsg, value: *const c_char);
}
extern "C" {
    pub fn devlink_is_reload_failed(devlink: *const devlink) -> bool;
}

extern "C" {
    pub fn devlink_try_get(devlink: *mut devlink) -> *mut devlink __must_check;
}
extern "C" {
    pub fn devlink_put(devlink: *mut devlink);
}
extern "C" {
    pub fn devlink_compat_flash_update(devlink: *mut devlink, file_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn devlink_nl_port_handle_fill(msg: *mut sk_buff, devlink_port: *mut devlink_port) -> c_int;
}
extern "C" {
    pub fn devlink_nl_port_handle_size(devlink_port: *mut devlink_port) -> usize;
}
extern "C" {
    pub fn devlink_fmsg_dump_skb(fmsg: *mut devlink_fmsg, skb: *const sk_buff);
}

