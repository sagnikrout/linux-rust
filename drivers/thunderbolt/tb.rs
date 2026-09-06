//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thunderbolt/tb.h
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
// Thunderbolt driver - bus logic (NHI independent)
//
// Copyright (c) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2018, Intel Corporation
//

// Keep link controller awake during update

// Disable CLx if not supported

// Need to keep power on while USB4 port is in redrive mode

//
// struct tb_nvm - Structure holding NVM information
// @dev: Owner of the NVM
// @major: Major version number of the active NVM portion
// @minor: Minor version number of the active NVM portion
// @id: Identifier used with both NVM portions
// @active: Active portion NVMem device
// @active_size: Size in bytes of the active NVM
// @non_active: Non-active portion NVMem device
// @buf: Buffer where the NVM image is stored before it is written to
// the actual NVM flash device
// @buf_data_start: Where the actual image starts after skipping
// possible headers
// @buf_data_size: Number of bytes actually consumed by the new NVM
// image
// @authenticating: The device is authenticating the new NVM
// @flushed: The image has been flushed to the storage area
// @vops: Router vendor specific NVM operations (optional)
//
// The user of this structure needs to handle serialization of possible
// concurrent access.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_nvm {
    pub dev: *mut device,
    pub major: u32,
    pub minor: u32,
    pub id: c_int,
    pub active: *mut nvmem_device,
    pub active_size: usize,
    pub non_active: *mut nvmem_device,
    pub buf: *mut c_void,
    pub buf_data_start: *mut c_void,
    pub buf_data_size: usize,
    pub authenticating: bool,
    pub flushed: bool,
    pub vops: *const tb_nvm_vendor_ops,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_nvm_write_ops {
    WRITE_AND_AUTHENTICATE = 1,
    WRITE_ONLY = 2,
    AUTHENTICATE_ONLY = 3,
}

pub const TB_SWITCH_KEY_SIZE: c_int = 32;
pub const TB_SWITCH_MAX_DEPTH: c_int = 6;
pub const USB4_SWITCH_MAX_DEPTH: c_int = 5;
//
// enum tb_switch_tmu_mode - TMU mode
// @TB_SWITCH_TMU_MODE_OFF: TMU is off
// @TB_SWITCH_TMU_MODE_LOWRES: Uni-directional, normal mode
// @TB_SWITCH_TMU_MODE_HIFI_UNI: Uni-directional, HiFi mode
// @TB_SWITCH_TMU_MODE_HIFI_BI: Bi-directional, HiFi mode
// @TB_SWITCH_TMU_MODE_MEDRES_ENHANCED_UNI: Enhanced Uni-directional, MedRes mode
//
// Ordering is based on TMU accuracy level (highest last).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_switch_tmu_mode {
    TB_SWITCH_TMU_MODE_OFF,
    TB_SWITCH_TMU_MODE_LOWRES,
    TB_SWITCH_TMU_MODE_HIFI_UNI,
    TB_SWITCH_TMU_MODE_HIFI_BI,
    TB_SWITCH_TMU_MODE_MEDRES_ENHANCED_UNI,
}

//
// struct tb_switch_tmu - Structure holding router TMU configuration
// @cap: Offset to the TMU capability (%0 if not found)
// @has_ucap: Does the switch support uni-directional mode
// @mode: TMU mode related to the upstream router. Reflects the HW
// setting. Don't care for host router.
// @mode_request: TMU mode requested to set. Related to upstream router.
// Don't care for host router.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_switch_tmu {
    pub cap: c_int,
    pub has_ucap: bool,
    pub mode: tb_switch_tmu_mode,
    pub mode_request: tb_switch_tmu_mode,
}

//
// struct tb_switch - a thunderbolt switch
// @dev: Device for the switch
// @config: Switch configuration
// @ports: Ports in this switch
// @dma_port: If the switch has port supporting DMA configuration based
// mailbox this will hold the pointer to that (%NULL
// otherwise). If set it also means the switch has
// upgradeable NVM.
// @tmu: The switch TMU configuration
// @tb: Pointer to the domain the switch belongs to
// @uid: Unique ID of the switch
// @uuid: UUID of the switch (or %NULL if not supported)
// @vendor: Vendor ID of the switch
// @device: Device ID of the switch
// @vendor_name: Name of the vendor (or %NULL if not known)
// @device_name: Name of the device (or %NULL if not known)
// @link_speed: Speed of the link in Gb/s
// @link_width: Width of the upstream facing link
// @preferred_link_width: Router preferred link width (only set for Gen 4 links)
// @link_usb4: Upstream link is USB4
// @generation: Switch Thunderbolt generation
// @cap_plug_events: Offset to the plug events capability (%0 if not found)
// @cap_vsec_tmu: Offset to the TMU vendor specific capability (%0 if not found)
// @cap_lc: Offset to the link controller capability (%0 if not found)
// @cap_lp: Offset to the low power (CLx for TBT) capability (%0 if not found)
// @is_unplugged: The switch is going away
// @drom: DROM of the switch (%NULL if not found)
// @nvm: Pointer to the NVM if the switch has one (%NULL otherwise)
// @no_nvm_upgrade: Prevent NVM upgrade of this switch
// @safe_mode: The switch is in safe-mode
// @boot: Whether the switch was already authorized on boot or not
// @rpm: The switch supports runtime PM
// @authorized: Whether the switch is authorized by user or policy
// @security_level: Switch supported security level
// @debugfs_dir: Pointer to the debugfs structure
// @key: Contains the key used to challenge the device or %NULL if not
// supported. Size of the key is %TB_SWITCH_KEY_SIZE.
// @connection_id: Connection ID used with ICM messaging
// @connection_key: Connection key used with ICM messaging
// @link: Root switch link this switch is connected (ICM only)
// @depth: Depth in the chain this switch is connected (ICM only)
// @rpm_complete: Completion used to wait for runtime resume to
// complete (ICM only)
// @quirks: Quirks used for this Thunderbolt switch
// @credit_allocation: Are the below buffer allocation parameters valid
// @max_usb3_credits: Router preferred number of buffers for USB 3.x
// @min_dp_aux_credits: Router preferred minimum number of buffers for DP AUX
// @min_dp_main_credits: Router preferred minimum number of buffers for DP MAIN
// @max_pcie_credits: Router preferred number of buffers for PCIe
// @max_dma_credits: Router preferred number of buffers for DMA/P2P
// @clx: CLx states on the upstream link of the router
// @drom_blob: DROM debugfs blob wrapper
//
// When the switch is being added or removed to the domain (other
// switches) you need to have domain lock held.
//
// In USB4 terminology this structure represents a router.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_switch {
    pub dev: device,
    pub config: tb_regs_switch_header,
    pub ports: *mut tb_port,
    pub dma_port: *mut tb_dma_port,
    pub tmu: tb_switch_tmu,
    pub tb: *mut tb,
    pub uid: u64,
    pub uuid: *mut uuid_t,
    pub vendor: u16,
    pub device: u16,
    pub vendor_name: *const c_char,
    pub device_name: *const c_char,
    pub link_speed: c_uint,
    pub link_width: tb_link_width,
    pub preferred_link_width: tb_link_width,
    pub link_usb4: bool,
    pub generation: c_uint,
    pub cap_plug_events: c_int,
    pub cap_vsec_tmu: c_int,
    pub cap_lc: c_int,
    pub cap_lp: c_int,
    pub is_unplugged: bool,
    pub drom: *mut u8,
    pub nvm: *mut tb_nvm,
    pub no_nvm_upgrade: bool,
    pub safe_mode: bool,
    pub boot: bool,
    pub rpm: bool,
    pub authorized: c_uint,
    pub security_level: tb_security_level,
    pub debugfs_dir: *mut dentry,
    pub key: *mut u8,
    pub connection_id: u8,
    pub connection_key: u8,
    pub link: u8,
    pub depth: u8,
    pub rpm_complete: completion,
    pub quirks: c_ulong,
    pub credit_allocation: bool,
    pub max_usb3_credits: c_uint,
    pub min_dp_aux_credits: c_uint,
    pub min_dp_main_credits: c_uint,
    pub max_pcie_credits: c_uint,
    pub max_dma_credits: c_uint,
    pub clx: c_uint,

    pub drom_blob: debugfs_blob_wrapper,

}

//
// struct tb_bandwidth_group - Bandwidth management group
// @tb: Pointer to the domain the group belongs to
// @index: Index of the group (aka Group_ID). Valid values %1-%7
// @ports: DP IN adapters belonging to this group are linked here
// @reserved: Bandwidth released by one tunnel in the group, available
// to others. This is reported as part of estimated_bw for
// the group.
// @release_work: Worker to release the @reserved if it is not used by
// any of the tunnels.
//
// Any tunnel that requires isochronous bandwidth (that's DP for now) is
// attached to a bandwidth group. All tunnels going through the same
// USB4 links share the same group and can dynamically distribute the
// bandwidth within the group.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_bandwidth_group {
    pub tb: *mut tb,
    pub index: c_int,
    pub ports: list_head,
    pub reserved: c_int,
    pub release_work: delayed_work,
}

//
// struct tb_port - a thunderbolt port, part of a tb_switch
// @config: Cached port configuration read from registers
// @sw: Switch the port belongs to
// @remote: Remote port (%NULL if not connected)
// @xdomain: Remote host (%NULL if not connected)
// @cap_phy: Offset, zero if not found
// @cap_tmu: Offset of the adapter specific TMU capability (%0 if not present)
// @cap_adap: Offset of the adapter specific capability (%0 if not present)
// @cap_usb4: Offset to the USB4 port capability (%0 if not present)
// @usb4: Pointer to the USB4 port structure (only if @cap_usb4 is != %0)
// @port: Port number on switch
// @disabled: Disabled by eeprom or enabled but not implemented
// @bonded: true if the port is bonded (two lanes combined as one)
// @dual_link_port: If the switch is connected using two ports, points
// to the other port.
// @link_nr: Is this primary or secondary port on the dual_link.
// @in_hopids: Currently allocated input HopIDs
// @out_hopids: Currently allocated output HopIDs
// @list: Used to link ports to DP resources list
// @total_credits: Total number of buffers available for this port
// @ctl_credits: Buffers reserved for control path
// @dma_credits: Number of credits allocated for DMA tunneling for all
// DMA paths through this port.
// @group: Bandwidth allocation group the adapter is assigned to. Only
// used for DP IN adapters for now.
// @group_list: The adapter is linked to the group's list of ports through this
// @max_bw: Maximum possible bandwidth through this adapter if set to
// non-zero.
// @redrive: For DP IN, if true the adapter is in redrive mode.
//
// In USB4 terminology this structure represents an adapter (protocol or
// lane adapter).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_port {
    pub config: tb_regs_port_header,
    pub sw: *mut tb_switch,
    pub remote: *mut tb_port,
    pub xdomain: *mut tb_xdomain,
    pub cap_phy: c_int,
    pub cap_tmu: c_int,
    pub cap_adap: c_int,
    pub cap_usb4: c_int,
    pub usb4: *mut usb4_port,
    pub port: u8,
    pub disabled: bool,
    pub bonded: bool,
    pub dual_link_port: *mut tb_port,
    pub link_nr:1: u8,
    pub in_hopids: ida,
    pub out_hopids: ida,
    pub list: list_head,
    pub total_credits: c_uint,
    pub ctl_credits: c_uint,
    pub dma_credits: c_uint,
    pub group: *mut tb_bandwidth_group,
    pub group_list: list_head,
    pub max_bw: c_uint,
    pub redrive: bool,
}

//
// struct usb4_port - USB4 port device
// @dev: Device for the port
// @port: Pointer to the lane 0 adapter
// @can_offline: Does the port have necessary platform support to move
// it into offline mode and back
// @offline: The port is currently in offline mode
// @margining: Pointer to margining structure if enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb4_port {
    pub dev: device,
    pub port: *mut tb_port,
    pub can_offline: bool,
    pub offline: bool,

    pub margining: *mut tb_margining,

}

//
// struct tb_retimer - Thunderbolt retimer
// @dev: Device for the retimer
// @tb: Pointer to the domain the retimer belongs to
// @index: Retimer index facing the router USB4 port
// @vendor: Vendor ID of the retimer
// @device: Device ID of the retimer
// @port: Pointer to the lane 0 adapter
// @nvm: Pointer to the NVM if the retimer has one (%NULL otherwise)
// @no_nvm_upgrade: Prevent NVM upgrade of this retimer
// @auth_status: Status of last NVM authentication
// @margining: Pointer to margining structure if enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_retimer {
    pub dev: device,
    pub tb: *mut tb,
    pub index: u8,
    pub vendor: u32,
    pub device: u32,
    pub port: *mut tb_port,
    pub nvm: *mut tb_nvm,
    pub no_nvm_upgrade: bool,
    pub auth_status: u32,

    pub margining: *mut tb_margining,

}

//
// struct tb_path_hop - routing information for a tb_path
// @in_port: Ingress port of a switch
// @out_port: Egress port of a switch where the packet is routed out
// (must be on the same switch as @in_port)
// @in_hop_index: HopID where the path configuration entry is placed in
// the path config space of @in_port.
// @in_counter_index: Used counter index (not used in the driver
// currently, %-1 to disable)
// @next_hop_index: HopID of the packet when it is routed out from @out_port
// @initial_credits: Number of initial flow control credits allocated for
// the path
// @nfc_credits: Number of non-flow controlled buffers allocated for the
// @in_port.
// @pm_support: Set path PM packet support bit to 1 (for USB4 v2 routers)
//
// Hop configuration is always done on the IN port of a switch.
// in_port and out_port have to be on the same switch. Packets arriving on
// in_port with "hop" = in_hop_index will get routed to through out_port. The
// next hop to take (on out_port->remote) is determined by
// next_hop_index. When routing packet to another switch (out->remote is
// set) the @next_hop_index must match the @in_hop_index of that next
// hop to make routing possible.
//
// in_counter_index is the index of a counter (in TB_CFG_COUNTERS) on the in
// port.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_path_hop {
    pub in_port: *mut tb_port,
    pub out_port: *mut tb_port,
    pub in_hop_index: c_int,
    pub in_counter_index: c_int,
    pub next_hop_index: c_int,
    pub initial_credits: c_uint,
    pub nfc_credits: c_uint,
    pub pm_support: bool,
}

//
// enum tb_path_port - path options mask
// @TB_PATH_NONE: Do not activate on any hop on path
// @TB_PATH_SOURCE: Activate on the first hop (out of src)
// @TB_PATH_INTERNAL: Activate on the intermediate hops (not the first/last)
// @TB_PATH_DESTINATION: Activate on the last hop (into dst)
// @TB_PATH_ALL: Activate on all hops on the path
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_path_port {
    TB_PATH_NONE = 0,
    TB_PATH_SOURCE = 1,
    TB_PATH_INTERNAL = 2,
    TB_PATH_DESTINATION = 4,
    TB_PATH_ALL = 7,
}

//
// struct tb_path - a unidirectional path between two ports
// @tb: Pointer to the domain structure
// @name: Name of the path (used for debugging)
// @ingress_shared_buffer: Shared buffering used for ingress ports on the path
// @egress_shared_buffer: Shared buffering used for egress ports on the path
// @ingress_fc_enable: Flow control for ingress ports on the path
// @egress_fc_enable: Flow control for egress ports on the path
// @priority: Priority group if the path
// @weight: Weight of the path inside the priority group
// @drop_packages: Drop packages from queue tail or head
// @activated: Is the path active
// @clear_fc: Clear all flow control from the path config space entries
// when deactivating this path
// @path_length: How many hops the path uses
// @alloc_hopid: Does this path consume port HopID
// @hops: Path hops
//
// A path consists of a number of hops (see &struct tb_path_hop). To
// establish a PCIe tunnel two paths have to be created between the two
// PCIe ports.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_path {
    pub tb: *mut tb,
    pub name: *const c_char,
    pub ingress_shared_buffer: tb_path_port,
    pub egress_shared_buffer: tb_path_port,
    pub ingress_fc_enable: tb_path_port,
    pub egress_fc_enable: tb_path_port,
    pub priority:3: c_uint,
    pub weight:4: c_int,
    pub drop_packages: bool,
    pub activated: bool,
    pub clear_fc: bool,
    pub path_length: c_int,
    pub alloc_hopid: bool,
    pub __counted_by(path_length): tb_path_hop hops[],
}

// HopIDs 0-7 are reserved by the Thunderbolt protocol
pub const TB_PATH_MIN_HOPID: c_int = 8;
//
// Support paths from the farthest (depth 6) router to the host and back
// to the same level (not necessarily to the same router).
//

// Possible wake types

// CL states

//
// struct tb_cm_ops - Connection manager specific operations vector
// @driver_ready: Called right after control channel is started. Used by
// ICM to send driver ready message to the firmware.
// @start: Starts the domain
// @stop: Stops the domain
// @deinit: Perform any cleanup after the domain is stopped but before
// it is unregistered. Called without @tb->lock taken. Optional.
// @suspend_noirq: Connection manager specific suspend_noirq
// @resume_noirq: Connection manager specific resume_noirq
// @suspend: Connection manager specific suspend
// @freeze_noirq: Connection manager specific freeze_noirq
// @thaw_noirq: Connection manager specific thaw_noirq
// @complete: Connection manager specific complete
// @runtime_suspend: Connection manager specific runtime_suspend
// @runtime_resume: Connection manager specific runtime_resume
// @runtime_suspend_switch: Runtime suspend a switch
// @runtime_resume_switch: Runtime resume a switch
// @handle_event: Handle thunderbolt event
// @get_boot_acl: Get boot ACL list
// @set_boot_acl: Set boot ACL list
// @disapprove_switch: Disapprove switch (disconnect PCIe tunnel)
// @approve_switch: Approve switch
// @add_switch_key: Add key to switch
// @challenge_switch_key: Challenge switch using key
// @disconnect_pcie_paths: Disconnects PCIe paths before NVM update
// @approve_xdomain_paths: Approve (establish) XDomain DMA paths
// @disconnect_xdomain_paths: Disconnect XDomain DMA paths
// @usb4_switch_op: Optional proxy for USB4 router operations. If set
// this will be called whenever USB4 router operation is
// performed. If this returns %-EOPNOTSUPP then the
// native USB4 router operation is called.
// @usb4_switch_nvm_authenticate_status: Optional callback that the CM
// implementation can use to return
// status of USB4 NVM_AUTH router
// operation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_cm_ops {
    pub tb): *mut *mut int (driver_ready)(struct tb,
    pub reset): *mut *mut *mut int (start)(struct tb tb, bool,
    pub tb): *mut *mut void (stop)(struct tb,
    pub tb): *mut *mut void (deinit)(struct tb,
    pub tb): *mut *mut int (suspend_noirq)(struct tb,
    pub tb): *mut *mut int (resume_noirq)(struct tb,
    pub tb): *mut *mut int (suspend)(struct tb,
    pub tb): *mut *mut int (freeze_noirq)(struct tb,
    pub tb): *mut *mut int (thaw_noirq)(struct tb,
    pub tb): *mut *mut void (complete)(struct tb,
    pub tb): *mut *mut int (runtime_suspend)(struct tb,
    pub tb): *mut *mut int (runtime_resume)(struct tb,
    pub sw): *mut *mut int (runtime_suspend_switch)(struct tb_switch,
    pub sw): *mut *mut int (runtime_resume_switch)(struct tb_switch,
    pub size): *const *const void buf, size_t,
    pub nuuids): *mut *mut *mut *mut int (get_boot_acl)(struct tb tb, uuid_t uuids, size_t,
    pub nuuids): *const *const *const *const int (set_boot_acl)(struct tb tb, uuid_t uuids, size_t,
    pub sw): *mut *mut *mut int (disapprove_switch)(struct tb tb, struct tb_switch,
    pub sw): *mut *mut *mut int (approve_switch)(struct tb tb, struct tb_switch,
    pub sw): *mut *mut *mut int (add_switch_key)(struct tb tb, struct tb_switch,
    pub response): *const *const u8 challenge, u8,
    pub tb): *mut *mut int (disconnect_pcie_paths)(struct tb,
    pub receive_ring): int receive_path, int,
    pub receive_ring): int receive_path, int,
    pub rx_data_len): *mut *mut void rx_data, size_t,
    pub status): *mut u32,
}

// helper functions & macros
//
// tb_upstream_port() - return the upstream port of a switch
// @sw: Router
//
// Every switch has an upstream port (for the root switch it is the NHI).
//
// During switch alloc/init tb_upstream_port()->remote may be NULL, even for
// non root switches (on the NHI port remote is always NULL).
//
// Return: Pointer to &struct tb_port.
//
// tb_is_upstream_port() - Is the port upstream facing
// @port: Port to check
//
// Return: %true if @port is upstream facing port. In case of dual link
// ports, both return %true.
//
// tb_port_has_remote() - Does the port have switch connected downstream
// @port: Port to check
//
// Return: %true only when the port is primary port and has remote set.
//

extern "C" {
    pub fn tb_domain_init() -> c_int;
}
extern "C" {
    pub fn tb_domain_exit();
}
extern "C" {
    pub fn tb_xdomain_init() -> c_int;
}
extern "C" {
    pub fn tb_xdomain_exit();
}
extern "C" {
    pub fn tb_domain_add(tb: *mut tb, reset: bool) -> c_int;
}
extern "C" {
    pub fn tb_domain_remove(tb: *mut tb);
}
extern "C" {
    pub fn tb_domain_suspend_noirq(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_resume_noirq(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_suspend(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_freeze_noirq(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_thaw_noirq(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_complete(tb: *mut tb);
}
extern "C" {
    pub fn tb_domain_runtime_suspend(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_runtime_resume(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_disapprove_switch(tb: *mut tb, sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_domain_approve_switch(tb: *mut tb, sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_domain_approve_switch_key(tb: *mut tb, sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_domain_challenge_switch_key(tb: *mut tb, sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_domain_disconnect_pcie_paths(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_disconnect_all_paths(tb: *mut tb) -> c_int;
}
extern "C" {
    pub fn tb_domain_unregister_unplugged_xdomains(tb: *mut tb) -> c_int;
}
//
// tb_domain_event() - Notify userspace about an event in domain
// @tb: Domain where event occurred
// @envp: Array of uevent environment strings (can be %NULL)
//
// This function provides a way to notify userspace about any events
// that take place in the domain.
//
extern "C" {
    pub fn tb_nvm_read_version(nvm: *mut tb_nvm) -> c_int;
}
extern "C" {
    pub fn tb_nvm_validate(nvm: *mut tb_nvm) -> c_int;
}
extern "C" {
    pub fn tb_nvm_write_headers(nvm: *mut tb_nvm) -> c_int;
}
extern "C" {
    pub fn tb_nvm_add_active(nvm: *mut tb_nvm, reg_read: nvmem_reg_read_t) -> c_int;
}
extern "C" {
    pub fn tb_nvm_add_non_active(nvm: *mut tb_nvm, reg_write: nvmem_reg_write_t) -> c_int;
}
extern "C" {
    pub fn tb_nvm_free(nvm: *mut tb_nvm);
}
extern "C" {
    pub fn tb_nvm_exit();
}
extern "C" {
    pub fn int(: *mut *mut read_block_fn)(void, int: unsigned, : *mut c_void, _arg: usize) -> typedef;
}
extern "C" {
    pub fn int(: *mut *mut write_block_fn)(void, int: unsigned, : *const c_void, _arg: usize) -> typedef;
}
extern "C" {
    pub fn tb_switch_configure(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_configuration_valid(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_add(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_remove(sw: *mut tb_switch);
}
extern "C" {
    pub fn tb_switch_suspend(sw: *mut tb_switch, runtime: bool);
}
extern "C" {
    pub fn tb_switch_resume(sw: *mut tb_switch, runtime: bool) -> c_int;
}
extern "C" {
    pub fn tb_switch_reset(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_sw_set_unplugged(sw: *mut tb_switch);
}
//
// tb_switch_for_each_port() - Iterate over each switch port
// @sw: Switch whose ports to iterate
// @p: Port used as iterator
//
// Iterates over each switch port skipping the control port (port %0).
//

extern "C" {
    pub fn container_of(_arg: dev, tb_switch: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn tb_to_switch(_arg: sw->dev.parent) -> return;
}
//
// tb_switch_downstream_port() - Return downstream facing port of parent router
// @sw: Device router pointer
//
// Call only for device routers.
//
// Return: Pointer to &struct tb_port or %NULL in case of failure.
//
extern "C" {
    pub fn tb_port_at(_arg: tb_route(sw), _arg: tb_switch_parent(sw)) -> return;
}
//
// tb_switch_depth() - Returns depth of the connected router
// @sw: Router
//
// Return: Router depth level as a number.
//
// tb_switch_is_icm() - Is the switch handled by ICM firmware
// @sw: Switch to check
//
// In case there is a need to differentiate whether ICM firmware or SW CM
// is handling @sw this function can be called. It is valid to call this
// after tb_switch_alloc() and tb_switch_configure() has been called
// (latter only for SW CM case).
//
// Return: %true if switch is handled by ICM, %false if handled by
// software CM.
//
extern "C" {
    pub fn tb_switch_set_link_width(sw: *mut tb_switch, width: tb_link_width) -> c_int;
}
extern "C" {
    pub fn tb_switch_configure_link(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_unconfigure_link(sw: *mut tb_switch);
}
extern "C" {
    pub fn tb_switch_query_dp_resource(sw: *mut tb_switch, in: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_switch_alloc_dp_resource(sw: *mut tb_switch, in: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_switch_dealloc_dp_resource(sw: *mut tb_switch, in: *mut tb_port);
}
extern "C" {
    pub fn tb_switch_tmu_init(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_tmu_post_time(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_tmu_disable(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_tmu_enable(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_tmu_configure(sw: *mut tb_switch, mode: tb_switch_tmu_mode) -> c_int;
}
//
// tb_switch_tmu_is_configured() - Is given TMU mode configured
// @sw: Router whose mode to check
// @mode: Mode to check
//
// Checks if given router TMU mode is configured to @mode. Note the
// router TMU might not be enabled to this mode.
//
// Return: %true if TMU mode is equal to @mode, %false otherwise.
//
// tb_switch_tmu_is_enabled() - Checks if the specified TMU mode is enabled
// @sw: Router whose TMU mode to check
//
// Return: %true if hardware TMU configuration matches the requested
// configuration (and is not %TB_SWITCH_TMU_MODE_OFF), %false otherwise.
//
extern "C" {
    pub fn tb_port_clx_is_enabled(port: *mut tb_port, clx: c_uint) -> bool;
}
extern "C" {
    pub fn tb_switch_clx_init(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_clx_enable(sw: *mut tb_switch, clx: c_uint) -> c_int;
}
extern "C" {
    pub fn tb_switch_clx_disable(sw: *mut tb_switch) -> c_int;
}
//
// tb_switch_clx_is_enabled() - Checks if the CLx is enabled
// @sw: Router to check for the CLx
// @clx: The CLx states to check for
//
// Checks if the specified CLx is enabled on the router upstream link.
//
// Not applicable for a host router.
//
// Return: %true if any of the given states is enabled, %false otherwise.
//
extern "C" {
    pub fn tb_switch_pcie_l1_enable(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_xhci_connect(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_switch_xhci_disconnect(sw: *mut tb_switch);
}
extern "C" {
    pub fn tb_port_state(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_wait_for_port(port: *mut tb_port, wait_if_unplugged: bool) -> c_int;
}
extern "C" {
    pub fn tb_port_add_nfc_credits(port: *mut tb_port, credits: c_int) -> c_int;
}
extern "C" {
    pub fn tb_port_clear_counter(port: *mut tb_port, counter: c_int) -> c_int;
}
extern "C" {
    pub fn tb_port_unlock(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_enable(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_disable(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_reset(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_alloc_in_hopid(port: *mut tb_port, hopid: c_int, max_hopid: c_int) -> c_int;
}
extern "C" {
    pub fn tb_port_release_in_hopid(port: *mut tb_port, hopid: c_int);
}
extern "C" {
    pub fn tb_port_alloc_out_hopid(port: *mut tb_port, hopid: c_int, max_hopid: c_int) -> c_int;
}
extern "C" {
    pub fn tb_port_release_out_hopid(port: *mut tb_port, hopid: c_int);
}
//
// tb_port_path_direction_downstream() - Checks if path is directed downstream
// @src: Source adapter
// @dst: Destination adapter
//
// Return: %true only if the specified path from source adapter (@src)
// to destination adapter (@dst) is directed downstream.
//
// tb_for_each_port_on_path() - Iterate over each port on path
// @src: Source port
// @dst: Destination port
// @p: Port used as iterator
//
// Walks over each port on path from @src to @dst.
//

//
// tb_for_each_upstream_port_on_path() - Iterate over each upstream port on path
// @src: Source port
// @dst: Destination port
// @p: Port used as iterator
//
// Walks over each upstream lane adapter on path from @src to @dst.
//

extern "C" {
    pub fn tb_port_get_link_speed(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_get_link_generation(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_get_link_width(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_width_supported(port: *mut tb_port, width: c_uint) -> bool;
}
extern "C" {
    pub fn tb_port_set_link_width(port: *mut tb_port, width: tb_link_width) -> c_int;
}
extern "C" {
    pub fn tb_port_lane_bonding_enable(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_port_lane_bonding_disable(port: *mut tb_port);
}
extern "C" {
    pub fn tb_port_update_credits(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_switch_find_vse_cap(sw: *mut tb_switch, vsec: tb_switch_vse_cap) -> c_int;
}
extern "C" {
    pub fn tb_switch_find_cap(sw: *mut tb_switch, cap: tb_switch_cap) -> c_int;
}
extern "C" {
    pub fn tb_switch_next_cap(sw: *mut tb_switch, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn tb_port_find_cap(port: *mut tb_port, cap: tb_port_cap) -> c_int;
}
extern "C" {
    pub fn tb_port_next_cap(port: *mut tb_port, offset: c_uint) -> c_int;
}
extern "C" {
    pub fn tb_port_is_enabled(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_usb3_port_is_enabled(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_usb3_port_enable(port: *mut tb_port, enable: bool) -> c_int;
}
extern "C" {
    pub fn tb_pci_port_is_enabled(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_pci_port_enable(port: *mut tb_port, enable: bool) -> c_int;
}
extern "C" {
    pub fn tb_dp_port_hpd_is_active(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_dp_port_hpd_clear(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_dp_port_is_enabled(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_dp_port_enable(port: *mut tb_port, enable: bool) -> c_int;
}
extern "C" {
    pub fn tb_path_free(path: *mut tb_path);
}
extern "C" {
    pub fn tb_path_activate(path: *mut tb_path) -> c_int;
}
extern "C" {
    pub fn tb_path_deactivate(path: *mut tb_path);
}
extern "C" {
    pub fn tb_path_deactivate_hop(port: *mut tb_port, hop_index: c_int) -> c_int;
}
extern "C" {
    pub fn tb_path_is_invalid(path: *mut tb_path) -> bool;
}
//
// tb_path_for_each_hop() - Iterate over each hop on path
// @path: Path whose hops to iterate
// @hop: Hop used as iterator
//
// Iterates over each hop on path.
//

extern "C" {
    pub fn tb_drom_read(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_drom_read_uid_only(sw: *mut tb_switch, uid: *mut u64) -> c_int;
}
extern "C" {
    pub fn tb_lc_read_uuid(sw: *mut tb_switch, uuid: *mut u32) -> c_int;
}
extern "C" {
    pub fn tb_lc_reset_port(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_lc_configure_port(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_lc_unconfigure_port(port: *mut tb_port);
}
extern "C" {
    pub fn tb_lc_configure_xdomain(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_lc_unconfigure_xdomain(port: *mut tb_port);
}
extern "C" {
    pub fn tb_lc_start_lane_initialization(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_lc_is_clx_supported(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_lc_is_usb_plugged(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_lc_is_xhci_connected(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_lc_xhci_connect(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_lc_xhci_disconnect(port: *mut tb_port);
}
extern "C" {
    pub fn tb_lc_set_wake(sw: *mut tb_switch, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn tb_lc_set_sleep(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn tb_lc_lane_bonding_possible(sw: *mut tb_switch) -> bool;
}
extern "C" {
    pub fn tb_lc_dp_sink_query(sw: *mut tb_switch, in: *mut tb_port) -> bool;
}
extern "C" {
    pub fn tb_lc_dp_sink_alloc(sw: *mut tb_switch, in: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_lc_dp_sink_dealloc(sw: *mut tb_switch, in: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_lc_force_power(sw: *mut tb_switch) -> c_int;
}
//
// tb_downstream_route() - get route to downstream switch
// @port: Port to check
//
// Port must not be the upstream port (otherwise a loop is created).
//
// Return: Route to the switch behind @port.
//
extern "C" {
    pub fn tb_is_xdomain_enabled() -> bool;
}
extern "C" {
    pub fn tb_xdomain_add(xd: *mut tb_xdomain);
}
extern "C" {
    pub fn tb_xdomain_remove(xd: *mut tb_xdomain);
}
extern "C" {
    pub fn tb_xdomain_unregister(xd: *mut tb_xdomain);
}
extern "C" {
    pub fn tb_to_switch(_arg: xd->dev.parent) -> return;
}
//
// tb_xdomain_downstream_port() - Return downstream facing port of parent router
// @xd: Xdomain pointer
//
// Return: Pointer to &struct tb_port or %NULL in case of failure.
//
extern "C" {
    pub fn tb_port_at(_arg: xd->route, _arg: tb_xdomain_parent(xd)) -> return;
}
extern "C" {
    pub fn tb_retimer_scan(port: *mut tb_port, add: bool) -> c_int;
}
extern "C" {
    pub fn tb_retimer_remove_all(port: *mut tb_port);
}
extern "C" {
    pub fn container_of(_arg: dev, tb_retimer: struct, _arg: dev) -> return;
}
//
// usb4_switch_version() - Returns USB4 version of the router
// @sw: Router to check
//
// Return: Major version of USB4 router (%1 for v1, %2 for v2 and so
// on). Can be called to pre-USB4 router too and in that case returns %0.
//
extern "C" {
    pub fn FIELD_GET(_arg: USB4_VERSION_MAJOR_MASK, _arg: sw->config.thunderbolt_version) -> return;
}
//
// tb_switch_is_usb4() - Is the switch USB4 compliant
// @sw: Switch to check
//
// Return: %true if the @sw is USB4 compliant router, %false otherwise.
//
extern "C" {
    pub fn usb4_switch_check_wakes(sw: *mut tb_switch);
}
extern "C" {
    pub fn usb4_switch_setup(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn usb4_switch_configuration_valid(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn usb4_switch_read_uid(sw: *mut tb_switch, uid: *mut u64) -> c_int;
}
extern "C" {
    pub fn usb4_switch_lane_bonding_possible(sw: *mut tb_switch) -> bool;
}
extern "C" {
    pub fn usb4_switch_set_wake(sw: *mut tb_switch, flags: c_uint, runtime: bool) -> c_int;
}
extern "C" {
    pub fn usb4_switch_set_sleep(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn usb4_switch_nvm_sector_size(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn usb4_switch_nvm_set_offset(sw: *mut tb_switch, address: c_uint) -> c_int;
}
extern "C" {
    pub fn usb4_switch_nvm_authenticate(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn usb4_switch_nvm_authenticate_status(sw: *mut tb_switch, status: *mut u32) -> c_int;
}
extern "C" {
    pub fn usb4_switch_credits_init(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn usb4_switch_query_dp_resource(sw: *mut tb_switch, in: *mut tb_port) -> bool;
}
extern "C" {
    pub fn usb4_switch_alloc_dp_resource(sw: *mut tb_switch, in: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_switch_dealloc_dp_resource(sw: *mut tb_switch, in: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_switch_add_ports(sw: *mut tb_switch) -> c_int;
}
extern "C" {
    pub fn usb4_switch_remove_ports(sw: *mut tb_switch);
}
extern "C" {
    pub fn usb4_port_unlock(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_hotplug_enable(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_reset(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_configure(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_unconfigure(port: *mut tb_port);
}
extern "C" {
    pub fn usb4_port_configure_xdomain(port: *mut tb_port, xd: *mut tb_xdomain) -> c_int;
}
extern "C" {
    pub fn usb4_port_unconfigure_xdomain(port: *mut tb_port);
}
extern "C" {
    pub fn usb4_port_router_offline(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_router_online(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_enumerate_retimers(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_clx_supported(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn usb4_port_asym_supported(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn usb4_port_asym_set_link_width(port: *mut tb_port, width: tb_link_width) -> c_int;
}
extern "C" {
    pub fn usb4_port_asym_start(port: *mut tb_port) -> c_int;
}
//
// enum usb4_sb_target - Sideband transaction target
// @USB4_SB_TARGET_ROUTER: Target is the router itself
// @USB4_SB_TARGET_PARTNER: Target is partner
// @USB4_SB_TARGET_RETIMER: Target is retimer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb4_sb_target {
    USB4_SB_TARGET_ROUTER,
    USB4_SB_TARGET_PARTNER,
    USB4_SB_TARGET_RETIMER,
}

//
// enum usb4_margin_sw_error_counter - Software margining error counter operation
// @USB4_MARGIN_SW_ERROR_COUNTER_NOP: No change in counter setup
// @USB4_MARGIN_SW_ERROR_COUNTER_CLEAR: Set the error counter to 0, enable counter
// @USB4_MARGIN_SW_ERROR_COUNTER_START: Start counter, count from last value
// @USB4_MARGIN_SW_ERROR_COUNTER_STOP: Stop counter, do not clear value
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb4_margin_sw_error_counter {
    USB4_MARGIN_SW_ERROR_COUNTER_NOP,
    USB4_MARGIN_SW_ERROR_COUNTER_CLEAR,
    USB4_MARGIN_SW_ERROR_COUNTER_START,
    USB4_MARGIN_SW_ERROR_COUNTER_STOP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb4_margining_lane {
    USB4_MARGINING_LANE_RX0 = 0,
    USB4_MARGINING_LANE_RX1 = 1,
    USB4_MARGINING_LANE_RX2 = 2,
    USB4_MARGINING_LANE_ALL = 7,
}

//
// struct usb4_port_margining_params - USB4 margining parameters
// @error_counter: Error counter operation for software margining
// @ber_level: Current BER level contour value
// @lanes: Lanes to enable for the margining operation
// @voltage_time_offset: Offset for voltage / time for software margining
// @optional_voltage_offset_range: Enable optional extended voltage range
// @right_high: %false if left/low margin test is performed, %true if right/high
// @upper_eye: %true if margin test is done on upper eye, %false if done on
// lower eye
// @time: %true if time margining is used instead of voltage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb4_port_margining_params {
    pub error_counter: usb4_margin_sw_error_counter,
    pub ber_level: u32,
    pub lanes: usb4_margining_lane,
    pub voltage_time_offset: u32,
    pub optional_voltage_offset_range: bool,
    pub right_high: bool,
    pub upper_eye: bool,
    pub time: bool,
}

extern "C" {
    pub fn usb4_port_retimer_set_inbound_sbtx(port: *mut tb_port, index: u8) -> c_int;
}
extern "C" {
    pub fn usb4_port_retimer_unset_inbound_sbtx(port: *mut tb_port, index: u8) -> c_int;
}
extern "C" {
    pub fn usb4_port_retimer_is_last(port: *mut tb_port, index: u8) -> c_int;
}
extern "C" {
    pub fn usb4_port_retimer_is_cable(port: *mut tb_port, index: u8) -> c_int;
}
extern "C" {
    pub fn usb4_port_retimer_nvm_sector_size(port: *mut tb_port, index: u8) -> c_int;
}
extern "C" {
    pub fn usb4_port_retimer_nvm_authenticate(port: *mut tb_port, index: u8) -> c_int;
}
extern "C" {
    pub fn usb4_usb3_port_max_link_rate(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_set_cm_id(port: *mut tb_port, cm_id: c_int) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_bandwidth_mode_supported(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn usb4_dp_port_bandwidth_mode_enabled(port: *mut tb_port) -> bool;
}
extern "C" {
    pub fn usb4_dp_port_group_id(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_set_group_id(port: *mut tb_port, group_id: c_int) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_nrd(port: *mut tb_port, rate: *mut c_int, lanes: *mut c_int) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_set_nrd(port: *mut tb_port, rate: c_int, lanes: c_int) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_granularity(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_set_granularity(port: *mut tb_port, granularity: c_int) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_set_estimated_bandwidth(port: *mut tb_port, bw: c_int) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_allocated_bandwidth(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_allocate_bandwidth(port: *mut tb_port, bw: c_int) -> c_int;
}
extern "C" {
    pub fn usb4_dp_port_requested_bandwidth(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn usb4_pci_port_set_ext_encapsulation(port: *mut tb_port, enable: bool) -> c_int;
}
extern "C" {
    pub fn usb4_pci_port_ltssm_state(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: dev, usb4_port: struct, _arg: dev) -> return;
}
extern "C" {
    pub fn usb4_port_device_remove(usb4: *mut usb4_port);
}
extern "C" {
    pub fn usb4_port_device_resume(usb4: *mut usb4_port) -> c_int;
}
extern "C" {
    pub fn usb4_port_index(sw: *const tb_switch, port: *const tb_port) -> c_int;
}
extern "C" {
    pub fn tb_check_quirks(sw: *mut tb_switch);
}

extern "C" {
    pub fn tb_acpi_add_links(nhi: *mut tb_nhi) -> bool;
}
extern "C" {
    pub fn tb_acpi_is_native() -> bool;
}
extern "C" {
    pub fn tb_acpi_may_tunnel_usb3() -> bool;
}
extern "C" {
    pub fn tb_acpi_may_tunnel_dp() -> bool;
}
extern "C" {
    pub fn tb_acpi_may_tunnel_pcie() -> bool;
}
extern "C" {
    pub fn tb_acpi_is_xdomain_allowed() -> bool;
}
extern "C" {
    pub fn tb_acpi_init() -> c_int;
}
extern "C" {
    pub fn tb_acpi_exit();
}
extern "C" {
    pub fn tb_acpi_power_on_retimers(port: *mut tb_port) -> c_int;
}
extern "C" {
    pub fn tb_acpi_power_off_retimers(port: *mut tb_port) -> c_int;
}

extern "C" {
    pub fn tb_debugfs_init();
}
extern "C" {
    pub fn tb_debugfs_exit();
}
extern "C" {
    pub fn tb_switch_debugfs_init(sw: *mut tb_switch);
}
extern "C" {
    pub fn tb_switch_debugfs_remove(sw: *mut tb_switch);
}
extern "C" {
    pub fn tb_xdomain_debugfs_init(xd: *mut tb_xdomain);
}
extern "C" {
    pub fn tb_xdomain_debugfs_remove(xd: *mut tb_xdomain);
}
extern "C" {
    pub fn tb_service_debugfs_init(svc: *mut tb_service);
}
extern "C" {
    pub fn tb_service_debugfs_remove(svc: *mut tb_service);
}
extern "C" {
    pub fn tb_retimer_debugfs_init(rt: *mut tb_retimer);
}
extern "C" {
    pub fn tb_retimer_debugfs_remove(rt: *mut tb_retimer);
}

extern "C" {
    pub fn tb_configfs_init() -> c_int;
}
extern "C" {
    pub fn tb_configfs_exit();
}

