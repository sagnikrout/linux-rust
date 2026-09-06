//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acpi_bus.h
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
// acpi_bus.h - ACPI Bus Driver ($Revision: 22 $)
//
// Copyright (C) 2001, 2002 Andy Grover <andrew.grover@intel.com>
// Copyright (C) 2001, 2002 Paul Diefenbaugh <paul.s.diefenbaugh@intel.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_handle_list {
    pub count: u32,
    pub handles: *mut acpi_handle,
}

// acpi_utils.h
extern "C" {
    pub fn acpi_handle_list_free(list: *mut acpi_handle_list);
}
extern "C" {
    pub fn acpi_device_dep(target: acpi_handle, match: acpi_handle) -> bool;
}
extern "C" {
    pub fn acpi_has_method(handle: acpi_handle, name: *mut c_char) -> bool;
}
extern "C" {
    pub fn acpi_evaluate_ej0(handle: acpi_handle) -> acpi_status;
}
extern "C" {
    pub fn acpi_evaluate_lck(handle: acpi_handle, lock: c_int) -> acpi_status;
}
extern "C" {
    pub fn acpi_evaluate_reg(handle: acpi_handle, space_id: u8, function: u32) -> acpi_status;
}
extern "C" {
    pub fn acpi_ata_match(handle: acpi_handle) -> bool;
}
extern "C" {
    pub fn acpi_bay_match(handle: acpi_handle) -> bool;
}
extern "C" {
    pub fn acpi_dock_match(handle: acpi_handle) -> bool;
}
extern "C" {
    pub fn acpi_check_dsm(handle: acpi_handle, guid: *const guid_t, rev: u64, funcs: u64) -> bool;
}

extern "C" {
    pub fn acpi_dev_found(hid: *const c_char) -> bool;
}
extern "C" {
    pub fn acpi_dev_present(hid: *const c_char, uid: *const c_char, hrv: i64) -> bool;
}
extern "C" {
    pub fn acpi_reduced_hardware() -> bool;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_bus_device_type {
    ACPI_BUS_TYPE_DEVICE = 0,
    ACPI_BUS_TYPE_POWER,
    ACPI_BUS_TYPE_PROCESSOR,
    ACPI_BUS_TYPE_THERMAL,
    ACPI_BUS_TYPE_POWER_BUTTON,
    ACPI_BUS_TYPE_SLEEP_BUTTON,
    ACPI_BUS_TYPE_ECDT_EC,
    ACPI_BUS_DEVICE_TYPE_COUNT
}

//
// ACPI Scan Handler
// -----------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hotplug_profile {
    pub kobj: kobject,
    pub adev): *mut *mut int (scan_dependent)(struct acpi_device,
    pub adev): *mut *mut void (notify_online)(struct acpi_device,
    pub enabled:1: bool,
    pub demand_offline:1: bool,
}

extern "C" {
    pub fn container_of(_arg: kobj, acpi_hotplug_profile: struct, _arg: kobj) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_scan_handler {
    pub list_node: list_head,
    pub ids: *const acpi_device_id,
    pub matchid): *const *const *const bool (match)(char idstr, struct acpi_device_id,
    pub id): *const *const *const int (attach)(struct acpi_device dev, struct acpi_device_id,
    pub dev): *mut *mut void (detach)(struct acpi_device,
    pub dev): *mut *mut void (post_eject)(struct acpi_device,
    pub phys_dev): *mut *mut void (bind)(struct device,
    pub phys_dev): *mut *mut void (unbind)(struct device,
    pub hotplug: acpi_hotplug_profile,
}

//
// ACPI Hotplug Context
// --------------------
//
extern "C" {
    pub fn int(: *mut *mut acpi_hp_notify) (struct acpi_device, _arg: u32) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut acpi_hp_uevent) (struct acpi_device, _arg: u32) -> typedef;
}
extern "C" {
    pub fn void(: *mut *mut acpi_hp_fixup) (struct acpi_device) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hotplug_context {
    pub self: *mut acpi_device,
    pub notify: acpi_hp_notify,
    pub uevent: acpi_hp_uevent,
    pub fixup: acpi_hp_fixup,
}

//
// ACPI Device
// -----------
//
// Status (_STA)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_status {
    pub present:1: u32,
    pub enabled:1: u32,
    pub show_in_ui:1: u32,
    pub functional:1: u32,
    pub battery_present:1: u32,
    pub reserved:27: u32,
}

// Flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_flags {
    pub dynamic_status:1: u32,
    pub removable:1: u32,
    pub ejectable:1: u32,
    pub power_manageable:1: u32,
    pub initialized:1: u32,
    pub visited:1: u32,
    pub hotplug_notify:1: u32,
    pub is_dock_station:1: u32,
    pub of_compatible_ok:1: u32,
    pub coherent_dma:1: u32,
    pub cca_seen:1: u32,
    pub enumeration_by_parent:1: u32,
    pub honor_deps:1: u32,
    pub reserved:19: u32,
}

// File System
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_dir {
    pub entry: *mut proc_dir_entry,
}

// Plug and Play
pub type acpi_bus_address = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_hardware_id {
    pub list: list_head,
    pub id: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pnp_type {
    pub hardware_id:1: u32,
    pub bus_address:1: u32,
    pub platform_id:1: u32,
    pub backlight:1: u32,
    pub reserved:28: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_pnp {
    pub /: *mut *mut acpi_bus_id bus_id; / Object name,
    pub /: *mut *mut int instance_no; / Instance number of this object,
    pub /: *mut *mut acpi_pnp_type type; / ID type,
    pub /: *mut *mut acpi_bus_address bus_address; / _ADR,
    pub /: *mut *mut *mut char unique_id; / _UID,
    pub /: *mut *mut list_head ids; / _HID and _CIDs,
}

// Power Management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_power_flags {
    pub /: *mut *mut u32 explicit_get:1; / _PSC present?,
    pub /: *mut *mut u32 power_resources:1; / Power resources,
    pub /: *mut *mut u32 inrush_current:1; / Serialize Dx->D0,
    pub /: *mut *mut u32 power_removed:1; / Optimize Dx->D0,
    pub /: *mut *mut u32 ignore_parent:1; / Power is independent of parent power state,
    pub /: *mut *mut u32 dsw_present:1; / _DSW present?,
    pub reserved:26: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_power_state {
    pub /: *mut *mut list_head resources; / Power resources referenced,
    pub valid:1: u8,
    pub /: *mut *mut u8 explicit_set:1; / _PSx present?,
    pub reserved:6: u8,
    pub flags: },
    pub /: *mut *mut int power; / % Power (compared to D0),
    pub /: *mut *mut int latency; / Dx->D0 time (microseconds),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_power {
    pub /: *mut *mut int state; / Current state,
    pub flags: acpi_device_power_flags,
    pub /: *mut *mut acpi_device_power_state states[ACPI_D_STATE_COUNT]; / Power states (D0-D3Cold),
    pub /: *mut *mut u8 state_for_enumeration; / Deepest power state for enumeration,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_dep_data {
    pub node: list_head,
    pub supplier: acpi_handle,
    pub consumer: acpi_handle,
    pub honor_dep: bool,
    pub met: bool,
    pub free_when_met: bool,
}

// Performance Management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_perf_flags {
    pub reserved:8: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_perf_state {
    pub valid:1: u8,
    pub reserved:7: u8,
    pub flags: },
    pub /: *mut *mut u8 power; / % Power (compared to P0),
    pub /: *mut *mut u8 performance; / % Performance ( " ),
    pub /: *mut *mut int latency; / Px->P0 time (microseconds),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_perf {
    pub state: c_int,
    pub flags: acpi_device_perf_flags,
    pub state_count: c_int,
    pub states: *mut acpi_device_perf_state,
}

// Wakeup Management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_wakeup_flags {
    pub /: *mut *mut u8 valid:1; / Can successfully enable wakeup?,
    pub /: *mut *mut u8 notifier_present:1; / Wake-up notify handler has been installed,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_wakeup_context {
    pub func: Option<unsafe extern "C" fn()>,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_wakeup {
    pub gpe_device: acpi_handle,
    pub gpe_number: u64,
    pub sleep_state: u64,
    pub resources: list_head,
    pub flags: acpi_device_wakeup_flags,
    pub context: acpi_device_wakeup_context,
    pub ws: *mut wakeup_source,
    pub prepare_count: c_int,
    pub enable_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_physical_node {
    pub node: list_head,
    pub dev: *mut device,
    pub node_id: c_uint,
    pub put_online:1: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_properties {
    pub list: list_head,
    pub guid: *const guid_t,
    pub properties: *mut acpi_object,
    pub bufs: *mut c_void,
}

// ACPI Device Specific Data (_DSD)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_data {
    pub pointer: *const acpi_object,
    pub properties: list_head,
    pub of_compatible: *const acpi_object,
    pub subnodes: list_head,
}

pub const ACPI_DEVICE_SWNODE_ROOT: c_int = 0;
//
// The maximum expected number of CSI-2 data lanes.
//
// This number is not expected to ever have to be equal to or greater than the
// number of bits in an unsigned long variable, but if it needs to be increased
// above that limit, code will need to be adjusted accordingly.
//
pub const ACPI_DEVICE_CSI2_DATA_LANES: c_int = 8;
pub const ACPI_DEVICE_SWNODE_PORT_NAME_LENGTH: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_device_swnode_dev_props {
    ACPI_DEVICE_SWNODE_DEV_ROTATION,
    ACPI_DEVICE_SWNODE_DEV_CLOCK_FREQUENCY,
    ACPI_DEVICE_SWNODE_DEV_LED_MAX_MICROAMP,
    ACPI_DEVICE_SWNODE_DEV_FLASH_MAX_MICROAMP,
    ACPI_DEVICE_SWNODE_DEV_FLASH_MAX_TIMEOUT_US,
    ACPI_DEVICE_SWNODE_DEV_NUM_OF,
    ACPI_DEVICE_SWNODE_DEV_NUM_ENTRIES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_device_swnode_port_props {
    ACPI_DEVICE_SWNODE_PORT_REG,
    ACPI_DEVICE_SWNODE_PORT_NUM_OF,
    ACPI_DEVICE_SWNODE_PORT_NUM_ENTRIES
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_device_swnode_ep_props {
    ACPI_DEVICE_SWNODE_EP_REMOTE_EP,
    ACPI_DEVICE_SWNODE_EP_BUS_TYPE,
    ACPI_DEVICE_SWNODE_EP_REG,
    ACPI_DEVICE_SWNODE_EP_CLOCK_LANES,
    ACPI_DEVICE_SWNODE_EP_DATA_LANES,
    ACPI_DEVICE_SWNODE_EP_LANE_POLARITIES,
// TX only
    ACPI_DEVICE_SWNODE_EP_LINK_FREQUENCIES,
    ACPI_DEVICE_SWNODE_EP_NUM_OF,
    ACPI_DEVICE_SWNODE_EP_NUM_ENTRIES
}

//
// Each device has a root software node plus two times as many nodes as the
// number of CSI-2 ports.
//

//
// struct acpi_device_software_node_port - MIPI DisCo for Imaging CSI-2 port
// @port_name: Port name.
// @data_lanes: "data-lanes" property values.
// @lane_polarities: "lane-polarities" property values.
// @link_frequencies: "link_frequencies" property values.
// @port_nr: Port number.
// @crs_csi2_local: _CRS CSI2 record present (i.e. this is a transmitter one).
// @port_props: Port properties.
// @ep_props: Endpoint properties.
// @remote_ep: Reference to the remote endpoint.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_software_node_port {
    pub 1]: char port_name[ACPI_DEVICE_SWNODE_PORT_NAME_LENGTH +,
    pub data_lanes: [u32; ACPI_DEVICE_CSI2_DATA_LANES],
    pub /]: *mut *mut u32 lane_polarities[ACPI_DEVICE_CSI2_DATA_LANES + 1 / clock lane,
    pub link_frequencies: [u64; ACPI_DEVICE_CSI2_DATA_LANES],
    pub port_nr: c_uint,
    pub crs_csi2_local: bool,
    pub port_props: [property_entry; ACPI_DEVICE_SWNODE_PORT_NUM_ENTRIES],
    pub ep_props: [property_entry; ACPI_DEVICE_SWNODE_EP_NUM_ENTRIES],
    pub remote_ep: [software_node_ref_args; 1],
}

//
// struct acpi_device_software_nodes - Software nodes for an ACPI device
// @dev_props: Device properties.
// @nodes: Software nodes for root as well as ports and endpoints.
// @nodeptrs: Array of software node pointers, for (un)registering them.
// @ports: Information related to each port and endpoint within a port.
// @num_ports: The number of ports.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device_software_nodes {
    pub dev_props: [property_entry; ACPI_DEVICE_SWNODE_DEV_NUM_ENTRIES],
    pub nodes: *mut software_node,
    pub nodeptrs: *const software_node,
    pub ports: *mut acpi_device_software_node_port,
    pub num_ports: c_uint,
}

// Device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_device {
    pub pld_crc: u32,
    pub device_type: c_int,
    pub /: *mut *mut acpi_handle handle; / no handle for fixed hardware,
    pub fwnode: fwnode_handle,
    pub wakeup_list: list_head,
    pub del_list: list_head,
    pub status: acpi_device_status,
    pub flags: acpi_device_flags,
    pub pnp: acpi_device_pnp,
    pub power: acpi_device_power,
    pub wakeup: acpi_device_wakeup,
    pub performance: acpi_device_perf,
    pub dir: acpi_device_dir,
    pub data: acpi_device_data,
    pub handler: *mut acpi_scan_handler,
    pub hp: *mut acpi_hotplug_context,
    pub swnodes: *mut acpi_device_software_nodes,
    pub driver_gpios: *const acpi_gpio_mapping,
    pub driver_data: *mut c_void,
    pub dev: device,
    pub physical_node_count: c_uint,
    pub dep_unmet: c_uint,
    pub physical_node_list: list_head,
    pub physical_node_lock: mutex,
    pub ): *mut *mut void (remove)(struct acpi_device,
}

// Non-device subnode
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_data_node {
    pub sibling: list_head,
    pub name: *const c_char,
    pub handle: acpi_handle,
    pub fwnode: fwnode_handle,
    pub parent: *mut fwnode_handle,
    pub data: acpi_device_data,
    pub kobj: kobject,
    pub kobj_done: completion,
}

extern "C" {
    pub fn is_acpi_device_node(fwnode: *const fwnode_handle) -> bool;
}
extern "C" {
    pub fn is_acpi_data_node(fwnode: *const fwnode_handle) -> bool;
}

extern "C" {
    pub fn to_acpi_device(_arg: adev->dev.parent) -> return;
}
// ((u32 *)&adev->status) = sta;
// acpi_device.dev.bus == &acpi_bus_type
extern "C" {
    pub fn acpi_bus_for_each_dev(: *mut *mut int (fn)(struct device, ): *mut c_void, data: *mut c_void) -> c_int;
}
//
// Events
// ------
//
pub const MAX_ACPI_CLASS_NAME_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_bus_event {
    pub node: list_head,
    pub device_class: acpi_device_class,
    pub bus_id: acpi_bus_id,
    pub type: u32,
    pub data: u32,
}

extern "C" {
    pub fn acpi_bus_generate_netlink_event(char*: *const , char*: *const , _arg: u8, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_bus_private_data_handler(_arg: acpi_handle, : *mut c_void);
}
extern "C" {
    pub fn acpi_bus_get_private_data(_arg: acpi_handle, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn acpi_bus_attach_private_data(_arg: acpi_handle, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn acpi_bus_detach_private_data(_arg: acpi_handle);
}
extern "C" {
    pub fn register_acpi_notifier(: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn unregister_acpi_notifier(: *mut notifier_block) -> c_int;
}
//
// External Functions
//
extern "C" {
    pub fn acpi_bus_get_status(device: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn acpi_bus_set_power(handle: acpi_handle, state: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_device_set_power(device: *mut acpi_device, state: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_bus_init_power(device: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn acpi_device_fix_up_power(device: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn acpi_device_fix_up_power_extended(adev: *mut acpi_device);
}
extern "C" {
    pub fn acpi_device_fix_up_power_children(adev: *mut acpi_device);
}
extern "C" {
    pub fn acpi_bus_update_power(handle: acpi_handle, state_p: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_device_update_power(device: *mut acpi_device, state_p: *mut c_int) -> c_int;
}
extern "C" {
    pub fn acpi_bus_power_manageable(handle: acpi_handle) -> bool;
}
extern "C" {
    pub fn acpi_dev_power_up_children_with_adr(adev: *mut acpi_device);
}
extern "C" {
    pub fn acpi_dev_power_state_for_wake(adev: *mut acpi_device) -> u8;
}

extern "C" {
    pub fn acpi_bus_can_wakeup(handle: acpi_handle) -> bool;
}

extern "C" {
    pub fn acpi_scan_lock_acquire();
}
extern "C" {
    pub fn acpi_scan_lock_release();
}
extern "C" {
    pub fn acpi_lock_hp_context();
}
extern "C" {
    pub fn acpi_unlock_hp_context();
}
extern "C" {
    pub fn acpi_scan_add_handler(handler: *mut acpi_scan_handler) -> c_int;
}
extern "C" {
    pub fn acpi_bus_scan(handle: acpi_handle) -> c_int;
}
extern "C" {
    pub fn acpi_bus_trim(start: *mut acpi_device);
}
extern "C" {
    pub fn acpi_bus_get_ejd(handle: acpi_handle, ejd: *mut *mut acpi_handle) -> acpi_status;
}
//
// Bind physical devices with ACPI devices
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_bus_type {
    pub list: list_head,
    pub name: *const c_char,
    pub dev): *mut *mut bool (match)(struct device,
    pub ): *mut *mut *mut acpi_device  (find_companion)(device,
    pub ): *mut *mut void (setup)(struct device,
}

extern "C" {
    pub fn register_acpi_bus_type(: *mut acpi_bus_type) -> c_int;
}
extern "C" {
    pub fn unregister_acpi_bus_type(: *mut acpi_bus_type) -> c_int;
}
extern "C" {
    pub fn acpi_bind_one(dev: *mut device, adev: *mut acpi_device) -> c_int;
}
extern "C" {
    pub fn acpi_unbind_one(dev: *mut device) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acpi_bridge_type {
    ACPI_BRIDGE_TYPE_PCIE = 1,
    ACPI_BRIDGE_TYPE_CXL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pci_root {
    pub device: *mut *mut acpi_device,
    pub bus: *mut pci_bus,
    pub segment: u16,
    pub bridge_type: c_int,
    pub /: *mut *mut resource secondary; / downstream bus range,
    pub /: *mut *mut u32 osc_support_set; / _OSC state of support bits,
    pub /: *mut *mut u32 osc_control_set; / _OSC state of control bits,
    pub /: *mut *mut u32 osc_ext_support_set; / _OSC state of extended support bits,
    pub /: *mut *mut u32 osc_ext_control_set; / _OSC state of extended control bits,
    pub mcfg_addr: phys_addr_t,
}

// helper
extern "C" {
    pub fn acpi_dma_supported(adev: *const acpi_device) -> bool;
}
extern "C" {
    pub fn acpi_get_dma_attr(adev: *mut acpi_device) -> dev_dma_attr;
}
extern "C" {
    pub fn acpi_dma_get_range(dev: *mut device, map: *const bus_dma_region) -> c_int;
}
extern "C" {
    pub fn acpi_dma_configure_id(_arg: dev, _arg: attr, _arg: NULL) -> return;
}
extern "C" {
    pub fn acpi_is_root_bridge(_arg: acpi_handle) -> c_int;
}
extern "C" {
    pub fn acpi_enable_wakeup_device_power(dev: *mut acpi_device, state: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_disable_wakeup_device_power(dev: *mut acpi_device) -> c_int;
}

extern "C" {
    pub fn acpi_device_override_status(adev: *mut acpi_device, status: *mut c_ulonglong) -> bool;
}
extern "C" {
    pub fn acpi_quirk_skip_acpi_ac_and_battery() -> bool;
}
extern "C" {
    pub fn acpi_quirk_skip_serdev_enumeration(controller_parent: *mut device, skip: *mut bool) -> c_int;
}

// skip = false;

extern "C" {
    pub fn acpi_quirk_skip_i2c_client_enumeration(adev: *mut acpi_device) -> bool;
}
extern "C" {
    pub fn acpi_quirk_skip_gpio_event_handlers() -> bool;
}

extern "C" {
    pub fn acpi_pm_wakeup_event(dev: *mut device);
}
extern "C" {
    pub fn acpi_remove_pm_notifier(adev: *mut acpi_device) -> acpi_status;
}
extern "C" {
    pub fn acpi_pm_device_can_wakeup(dev: *mut device) -> bool;
}
extern "C" {
    pub fn acpi_pm_device_sleep_state(: *mut device, : *mut c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn acpi_pm_set_device_wakeup(dev: *mut device, enable: bool) -> c_int;
}

// p = ACPI_STATE_D0;

extern "C" {
    pub fn acpi_sleep_state_supported(sleep_state: u8) -> bool;
}

extern "C" {
    pub fn acpi_target_system_state() -> u32;
}

extern "C" {
    pub fn acpi_dev_uid_to_integer(adev: *mut acpi_device, integer: *mut u64) -> c_int;
}

//
// acpi_dev_uid_match - Match device by supplied UID
// @adev: ACPI device to match.
// @uid2: Unique ID of the device.
//
// Matches UID in @adev with given @uid2.
//
// Returns: %true if matches, %false otherwise.
//

// Treat @uid2 as a string for acpi string types */	\
// Treat as an integer otherwise */			\
//
// acpi_dev_hid_uid_match - Match device by supplied HID and UID
// @adev: ACPI device to match.
// @hid2: Hardware ID of the device.
// @uid2: Unique ID of the device, pass NULL to not check _UID.
//
// Matches HID and UID in @adev with given @hid2 and @uid2. Absence of @uid2
// will be treated as a match. If user wants to validate @uid2, it should be
// done before calling this function.
//
// Returns: %true if matches or @uid2 is NULL, %false otherwise.
//

// Distinguish integer 0 from NULL @uid2 */		\
extern "C" {
    pub fn acpi_dev_clear_dependencies(supplier: *mut acpi_device);
}
extern "C" {
    pub fn acpi_dev_ready_for_enumeration(device: *const acpi_device) -> bool;
}
//
// for_each_acpi_consumer_dev - iterate over the consumer ACPI devices for a
// given supplier
// @supplier: Pointer to the supplier's ACPI device
// @consumer: Pointer to &struct acpi_device to hold the consumer, initially NULL
//

//
// for_each_acpi_dev_match - iterate over ACPI devices that matching the criteria
// @adev: pointer to the matching ACPI device, NULL at the end of the loop
// @hid: Hardware ID of the device.
// @uid: Unique ID of the device, pass NULL to not check _UID
// @hrv: Hardware Revision of the device, pass -1 to not check _HRV
//
// The caller is responsible for invoking acpi_dev_put() on the returned device.
//

extern "C" {
    pub fn acpi_wait_for_acpi_ipmi() -> c_int;
}
extern "C" {
    pub fn acpi_scan_add_dep(handle: acpi_handle, dep_devices: *mut acpi_handle_list) -> c_int;
}
extern "C" {
    pub fn arch_acpi_add_auto_dep(handle: acpi_handle) -> u32;
}

