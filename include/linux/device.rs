//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device.h
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
// device.h - generic, centralized driver model
//
// Copyright (c) 2001-2003 Patrick Mochel <mochel@osdl.org>
// Copyright (c) 2004-2009 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (c) 2008-2009 Novell Inc.
//
// See Documentation/driver-api/driver-model/ for more information.
//

//
// struct subsys_interface - interfaces to device functions
// @name:       name of the device function
// @subsys:     subsystem of the devices to attach to
// @node:       the list of functions registered at the subsystem
// @add_dev:    device hookup to device function handler
// @remove_dev: device hookup to device function handler
//
// Simple interfaces attached to a subsystem. Multiple interfaces can
// attach to a subsystem and its devices. Unlike drivers, they do not
// exclusively claim or control devices. Interfaces usually represent
// a specific functionality of a subsystem/class of devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subsys_interface {
    pub name: *const c_char,
    pub subsys: *const bus_type,
    pub node: list_head,
    pub sif): *mut *mut *mut int (add_dev)(struct device dev, struct subsys_interface,
    pub sif): *mut *mut *mut void (remove_dev)(struct device dev, struct subsys_interface,
}

extern "C" {
    pub fn subsys_interface_register(sif: *mut subsys_interface) -> c_int;
}
extern "C" {
    pub fn subsys_interface_unregister(sif: *mut subsys_interface);
}
//
// The type of device, "struct device" is embedded in. A class
// or bus can contain devices of different types
// like "partitions" and "disks", "mouse" and "event".
// This identifies the device type and carries type-specific
// information, equivalent to the kobj_type of a kobject.
// If "name" is specified, the uevent will contain it in
// the DEVTYPE variable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_type {
    pub name: *const c_char,
    pub groups: *const attribute_group,
    pub env): *const *const *const int (uevent)(struct device dev, struct kobj_uevent_env,
    pub gid): *mut *mut kuid_t uid, kgid_t,
    pub dev): *mut *mut void (release)(struct device,
    pub pm: *const dev_pm_ops,
}

//
// struct device_attribute - Interface for exporting device attributes.
// @attr: sysfs attribute definition.
// @show: Show handler.
// @show_const: Show handler (read-only).
// @store: Store handler.
// @store_const: Store handler (read-only).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_attribute {
    pub attr: attribute,
    pub buf): *mut c_char,
    pub buf): *mut c_char,
    pub count): *const *const char buf, size_t,
    pub count): *const *const char buf, size_t,
}

//
// struct dev_ext_attribute - Exported device attribute with extra context.
// @attr: Exported device attribute.
// @var: Pointer to context.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_ext_attribute {
    pub attr: device_attribute,
    pub var: *mut c_void,
}

//
// DEVICE_ATTR - Define a device attribute.
// @_name: Attribute name.
// @_mode: File mode.
// @_show: Show handler. Optional, but mandatory if attribute is readable.
// @_store: Store handler. Optional, but mandatory if attribute is writable.
//
// Convenience macro for defining a struct device_attribute.
//
// For example, ``DEVICE_ATTR(foo, 0644, foo_show, foo_store);`` expands to:
//
// .. code-block:: c
//
// struct device_attribute dev_attr_foo = {
// .attr	= { .name = "foo", .mode = 0644 },
// .show	= foo_show,
// .store	= foo_store,
// };
//

//
// DEVICE_ATTR_RW - Define a read-write device attribute.
// @_name: Attribute name.
//
// Like DEVICE_ATTR(), but @_mode is 0644, @_show is <_name>_show,
// and @_store is <_name>_store.
//

//
// DEVICE_ATTR_ADMIN_RW - Define an admin-only read-write device attribute.
// @_name: Attribute name.
//
// Like DEVICE_ATTR_RW(), but @_mode is 0600.
//

//
// DEVICE_ATTR_RW_NAMED - Define a read-write device attribute with a sysfs name
// that differs from the function name.
// @_name: Attribute function preface
// @_attrname: Attribute name as it wil be exposed in the sysfs.
//
// Like DEVICE_ATTR_RW(), but allows for reusing names under separate paths in
// the same driver.
//

//
// DEVICE_ATTR_RO - Define a readable device attribute.
// @_name: Attribute name.
//
// Like DEVICE_ATTR(), but @_mode is 0444 and @_show is <_name>_show.
//

//
// DEVICE_ATTR_ADMIN_RO - Define an admin-only readable device attribute.
// @_name: Attribute name.
//
// Like DEVICE_ATTR_RO(), but @_mode is 0400.
//

//
// DEVICE_ATTR_RO_NAMED - Define a read-only device attribute with a sysfs name
// that differs from the function name.
// @_name: Attribute function preface
// @_attrname: Attribute name as it wil be exposed in the sysfs.
//
// Like DEVICE_ATTR_RO(), but allows for reusing names under separate paths in
// the same driver.
//

//
// DEVICE_ATTR_WO - Define an admin-only writable device attribute.
// @_name: Attribute name.
//
// Like DEVICE_ATTR(), but @_mode is 0200 and @_store is <_name>_store.
//

//
// DEVICE_ATTR_WO_NAMED - Define a read-only device attribute with a sysfs name
// that differs from the function name.
// @_name: Attribute function preface
// @_attrname: Attribute name as it wil be exposed in the sysfs.
//
// Like DEVICE_ATTR_WO(), but allows for reusing names under separate paths in
// the same driver.
//

//
// DEVICE_ULONG_ATTR - Define a device attribute backed by an unsigned long.
// @_name: Attribute name.
// @_mode: File mode.
// @_var: Identifier of unsigned long.
//
// Like DEVICE_ATTR(), but @_show and @_store are automatically provided
// such that reads and writes to the attribute from userspace affect @_var.
//

//
// DEVICE_INT_ATTR - Define a device attribute backed by an int.
// @_name: Attribute name.
// @_mode: File mode.
// @_var: Identifier of int.
//
// Like DEVICE_ULONG_ATTR(), but @_var is an int.
//

//
// DEVICE_BOOL_ATTR - Define a device attribute backed by a bool.
// @_name: Attribute name.
// @_mode: File mode.
// @_var: Identifier of bool.
//
// Like DEVICE_ULONG_ATTR(), but @_var is a bool.
//

//
// DEVICE_STRING_ATTR_RO - Define a device attribute backed by a r/o string.
// @_name: Attribute name.
// @_mode: File mode.
// @_var: Identifier of string.
//
// Like DEVICE_ULONG_ATTR(), but @_var is a string. Because the length of the
// string allocation is unknown, the attribute must be read-only.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_dma_parameters {
//
// a low level driver may set these to teach IOMMU code about
// sg limitations.
//
    pub max_segment_size: c_uint,
    pub min_align_mask: c_uint,
    pub segment_boundary_mask: c_ulong,
}

//
// enum device_link_state - Device link states.
// @DL_STATE_NONE: The presence of the drivers is not being tracked.
// @DL_STATE_DORMANT: None of the supplier/consumer drivers is present.
// @DL_STATE_AVAILABLE: The supplier driver is present, but the consumer is not.
// @DL_STATE_CONSUMER_PROBE: The consumer is probing (supplier driver present).
// @DL_STATE_ACTIVE: Both the supplier and consumer drivers are present.
// @DL_STATE_SUPPLIER_UNBIND: The supplier driver is unbinding.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum device_link_state {
    DL_STATE_NONE = -1,
    DL_STATE_DORMANT = 0,
    DL_STATE_AVAILABLE,
    DL_STATE_CONSUMER_PROBE,
    DL_STATE_ACTIVE,
    DL_STATE_SUPPLIER_UNBIND,
}

//
// Device link flags.
//
// STATELESS: The core will not remove this link automatically.
// AUTOREMOVE_CONSUMER: Remove the link automatically on consumer driver unbind.
// PM_RUNTIME: If set, the runtime PM framework will use this link.
// RPM_ACTIVE: Run pm_runtime_get_sync() on the supplier during link creation.
// AUTOREMOVE_SUPPLIER: Remove the link automatically on supplier driver unbind.
// AUTOPROBE_CONSUMER: Probe consumer driver automatically after supplier binds.
// MANAGED: The core tracks presence of supplier/consumer drivers (internal).
// SYNC_STATE_ONLY: Link only affects sync_state() behavior.
// INFERRED: Inferred from data (eg: firmware) and not from driver actions.
//

//
// enum dl_dev_state - Device driver presence tracking information.
// @DL_DEV_NO_DRIVER: There is no driver attached to the device.
// @DL_DEV_PROBING: A driver is probing.
// @DL_DEV_DRIVER_BOUND: The driver has been bound to the device.
// @DL_DEV_UNBINDING: The driver is unbinding from the device.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dl_dev_state {
    DL_DEV_NO_DRIVER = 0,
    DL_DEV_PROBING,
    DL_DEV_DRIVER_BOUND,
    DL_DEV_UNBINDING,
}

//
// enum device_removable - Whether the device is removable. The criteria for a
// device to be classified as removable is determined by its subsystem or bus.
// @DEVICE_REMOVABLE_NOT_SUPPORTED: This attribute is not supported for this
// device (default).
// @DEVICE_REMOVABLE_UNKNOWN:  Device location is Unknown.
// @DEVICE_FIXED: Device is not removable by the user.
// @DEVICE_REMOVABLE: Device is removable by the user.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum device_removable {
    DEVICE_REMOVABLE_NOT_SUPPORTED = 0, /* must be 0 */
    DEVICE_REMOVABLE_UNKNOWN,
    DEVICE_FIXED,
    DEVICE_REMOVABLE,
}

//
// struct dev_links_info - Device data related to device links.
// @suppliers: List of links to supplier devices.
// @consumers: List of links to consumer devices.
// @defer_sync: Hook to global list of devices that have deferred sync_state.
// @status: Driver status information.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_links_info {
    pub suppliers: list_head,
    pub consumers: list_head,
    pub defer_sync: list_head,
    pub status: dl_dev_state,
}

//
// struct dev_msi_info - Device data related to MSI
// @domain:	The MSI interrupt domain associated to the device
// @data:	Pointer to MSI device data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_msi_info {

    pub domain: *mut irq_domain,
    pub data: *mut msi_device_data,

}

//
// enum device_physical_location_panel - Describes which panel surface of the
// system's housing the device connection point resides on.
// @DEVICE_PANEL_TOP: Device connection point is on the top panel.
// @DEVICE_PANEL_BOTTOM: Device connection point is on the bottom panel.
// @DEVICE_PANEL_LEFT: Device connection point is on the left panel.
// @DEVICE_PANEL_RIGHT: Device connection point is on the right panel.
// @DEVICE_PANEL_FRONT: Device connection point is on the front panel.
// @DEVICE_PANEL_BACK: Device connection point is on the back panel.
// @DEVICE_PANEL_UNKNOWN: The panel with device connection point is unknown.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum device_physical_location_panel {
    DEVICE_PANEL_TOP,
    DEVICE_PANEL_BOTTOM,
    DEVICE_PANEL_LEFT,
    DEVICE_PANEL_RIGHT,
    DEVICE_PANEL_FRONT,
    DEVICE_PANEL_BACK,
    DEVICE_PANEL_UNKNOWN,
}

//
// enum device_physical_location_vertical_position - Describes vertical
// position of the device connection point on the panel surface.
// @DEVICE_VERT_POS_UPPER: Device connection point is at upper part of panel.
// @DEVICE_VERT_POS_CENTER: Device connection point is at center part of panel.
// @DEVICE_VERT_POS_LOWER: Device connection point is at lower part of panel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum device_physical_location_vertical_position {
    DEVICE_VERT_POS_UPPER,
    DEVICE_VERT_POS_CENTER,
    DEVICE_VERT_POS_LOWER,
}

//
// enum device_physical_location_horizontal_position - Describes horizontal
// position of the device connection point on the panel surface.
// @DEVICE_HORI_POS_LEFT: Device connection point is at left part of panel.
// @DEVICE_HORI_POS_CENTER: Device connection point is at center part of panel.
// @DEVICE_HORI_POS_RIGHT: Device connection point is at right part of panel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum device_physical_location_horizontal_position {
    DEVICE_HORI_POS_LEFT,
    DEVICE_HORI_POS_CENTER,
    DEVICE_HORI_POS_RIGHT,
}

//
// struct device_physical_location - Device data related to physical location
// of the device connection point.
// @panel: Panel surface of the system's housing that the device connection
// point resides on.
// @vertical_position: Vertical position of the device connection point within
// the panel.
// @horizontal_position: Horizontal position of the device connection point
// within the panel.
// @dock: Set if the device connection point resides in a docking station or
// port replicator.
// @lid: Set if this device connection point resides on the lid of laptop
// system.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_physical_location {
    pub panel: device_physical_location_panel,
    pub vertical_position: device_physical_location_vertical_position,
    pub horizontal_position: device_physical_location_horizontal_position,
    pub dock: bool,
    pub lid: bool,
}

//
// enum struct_device_flags - Flags in struct device
//
// Each flag should have a set of accessor functions created via
// __create_dev_flag_accessors() for each access.
//
// @DEV_FLAG_READY_TO_PROBE: If set then device_add() has finished enough
// initialization that probe could be called.
// @DEV_FLAG_CAN_MATCH: The device has matched with a driver at least once or it
// is in a bus (like AMBA) which can't check for matching drivers
// until other devices probe successfully.
// @DEV_FLAG_DMA_IOMMU: Device is using default IOMMU implementation for DMA and
// doesn't rely on dma_ops structure.
// @DEV_FLAG_DMA_SKIP_SYNC: DMA sync operations can be skipped for coherent
// buffers.
// @DEV_FLAG_DMA_OPS_BYPASS: If set then the dma_ops are bypassed for the
// streaming DMA operations (->map_* / ->unmap_* / ->sync_*), and
// optional (if the coherent mask is large enough) also for dma
// allocations. This flag is managed by the dma ops instance from
// ->dma_supported.
// @DEV_FLAG_STATE_SYNCED: The hardware state of this device has been synced to
// match the software state of this device by calling the
// driver/bus sync_state() callback.
// @DEV_FLAG_DMA_COHERENT: This particular device is dma coherent, even if the
// architecture supports non-coherent devices.
// @DEV_FLAG_OF_NODE_REUSED: Set if the device-tree node is shared with an
// ancestor device.
// @DEV_FLAG_OFFLINE_DISABLED: If set, the device is permanently online.
// @DEV_FLAG_OFFLINE: Set after successful invocation of bus type's .offline().
// @DEV_FLAG_COUNT: Number of defined struct_device_flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum struct_device_flags {
    DEV_FLAG_READY_TO_PROBE = 0,
    DEV_FLAG_CAN_MATCH = 1,
    DEV_FLAG_DMA_IOMMU = 2,
    DEV_FLAG_DMA_SKIP_SYNC = 3,
    DEV_FLAG_DMA_OPS_BYPASS = 4,
    DEV_FLAG_STATE_SYNCED = 5,
    DEV_FLAG_DMA_COHERENT = 6,
    DEV_FLAG_OF_NODE_REUSED = 7,
    DEV_FLAG_OFFLINE_DISABLED = 8,
    DEV_FLAG_OFFLINE = 9,

    DEV_FLAG_COUNT
}

//
// struct device - The basic device structure
// @parent:	The device's "parent" device, the device to which it is attached.
// In most cases, a parent device is some sort of bus or host
// controller. If parent is NULL, the device, is a top-level device,
// which is not usually what you want.
// @p:		Holds the private data of the driver core portions of the device.
// See the comment of the struct device_private for detail.
// @kobj:	A top-level, abstract class from which other classes are derived.
// @init_name:	Initial name of the device.
// @type:	The type of device.
// This identifies the device type and carries type-specific
// information.
// @mutex:	Mutex to synchronize calls to its driver.
// @bus:	Type of bus device is on.
// @driver:	Which driver has allocated this
// @platform_data: Platform data specific to the device.
// Example: For devices on custom boards, as typical of embedded
// and SOC based hardware, Linux often uses platform_data to point
// to board-specific structures describing devices and how they
// are wired.  That can include what ports are available, chip
// variants, which GPIO pins act in what additional roles, and so
// on.  This shrinks the "Board Support Packages" (BSPs) and
// minimizes board-specific #ifdefs in drivers.
// @driver_data: Private pointer for driver specific info.
// @driver_override: Driver name to force a match.  Do not touch directly; use
// device_set_driver_override() instead.
// @links:	Links to suppliers and consumers of this device.
// @power:	For device power management.
// See Documentation/driver-api/pm/devices.rst for details.
// @pm_domain:	Provide callbacks that are executed during system suspend,
// hibernation, system resume and during runtime PM transitions
// along with subsystem-level and driver-level callbacks.
// @em_pd:	device's energy model performance domain
// @pins:	For device pin management.
// See Documentation/driver-api/pin-control.rst for details.
// @msi:	MSI related data
// @numa_node:	NUMA node this device is close to.
// @dma_ops:    DMA mapping operations for this device.
// @dma_mask:	Dma mask (if dma'ble device).
// @coherent_dma_mask: Like dma_mask, but for alloc_coherent mapping as not all
// hardware supports 64-bit addresses for consistent allocations
// such descriptors.
// @bus_dma_limit: Limit of an upstream bridge or bus which imposes a smaller
// DMA limit than the device itself supports.
// @dma_range_map: map for DMA memory ranges relative to that of RAM
// @dma_parms:	A low level driver may set these to teach IOMMU code about
// segment limitations.
// @dma_pools:	Dma pools (if dma'ble device).
// @dma_mem:	Internal for coherent mem override.
// @cma_area:	Contiguous memory area for dma allocations
// @dma_io_tlb_mem: Software IO TLB allocator.  Not for driver use.
// @dma_io_tlb_pools:	List of transient swiotlb memory pools.
// @dma_io_tlb_lock:	Protects changes to the list of active pools.
// @dma_uses_io_tlb: %true if device has used the software IO TLB.
// @archdata:	For arch-specific additions.
// @of_node:	Associated device tree node.
// @fwnode:	Associated device node supplied by platform firmware.
// @devt:	For creating the sysfs "dev".
// @id:		device instance
// @devres_lock: Spinlock to protect the resource of the device.
// @devres_head: The resources list of the device.
// @class:	The class of the device.
// @groups:	Optional attribute groups.
// @release:	Callback to free the device after all references have
// gone away. This should be set by the allocator of the
// device (i.e. the bus driver that discovered the device).
// @iommu_group: IOMMU group the device belongs to.
// @iommu:	Per device generic IOMMU runtime data
// @physical_location: Describes physical location of the device connection
// point in the system housing.
// @removable:  Whether the device can be removed from the system. This
// should be set by the subsystem / bus driver that discovered
// the device.
// @flags:	DEV_FLAG_XXX flags. Use atomic bitfield operations to modify.
//
// At the lowest level, every device in a Linux system is represented by an
// instance of struct device. The device structure contains the information
// that the device model core needs to model the system. Most subsystems,
// however, track additional information about the devices they host. As a
// result, it is rare for devices to be represented by bare device structures;
// instead, that structure, like kobject structures, is usually embedded within
// a higher-level representation of the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device {
    pub kobj: kobject,
    pub parent: *mut device,
    pub p: *mut device_private,
    pub /: *const *const *const char init_name; / initial name of the device,
    pub type: *const device_type,
    pub /: *const *const *const bus_type bus; / type of bus device is on,
    pub this: *mut *mut *mut device_driver driver; / which driver has allocated,
    pub device: *mut *mut *mut void platform_data; / Platform specific data,,
    pub with: *mut *mut *mut void driver_data; / Driver data, set and get,
    pub name: *const c_char,
    pub lock: spinlock_t,
    pub driver_override: },
    pub to: *mut *mut mutex mutex; / mutex to synchronize calls,
// its driver.
//
    pub links: dev_links_info,
    pub power: dev_pm_info,
    pub pm_domain: *mut dev_pm_domain,

    pub em_pd: *mut em_perf_domain,

    pub pins: *mut dev_pin_info,

    pub msi: dev_msi_info,

    pub dma_ops: *const dma_map_ops,

    pub /: *mut *mut *mut u64 dma_mask; / dma mask (if dma'able device),
    pub for: *mut *mut u64 coherent_dma_mask;/ Like dma_mask, but,
    pub /: *mut *mut u64 bus_dma_limit; / upstream dma constraint,
    pub dma_range_map: *const bus_dma_region,
    pub dma_parms: *mut device_dma_parameters,
    pub /: *mut *mut list_head dma_pools; / dma pools (if dma'ble),

    pub mem: *mut *mut *mut dma_coherent_mem dma_mem; / internal for coherent,

    pub dma: *mut *mut *mut cma cma_area; / contiguous memory area for,

    pub dma_io_tlb_mem: *mut io_tlb_mem,

    pub dma_io_tlb_pools: list_head,
    pub dma_io_tlb_lock: spinlock_t,
    pub dma_uses_io_tlb: bool,

// arch specific additions
    pub archdata: dev_archdata,
    pub /: *mut *mut *mut device_node of_node; / associated device tree node,
    pub /: *mut *mut *mut fwnode_handle fwnode; / firmware device node,

    pub /: *mut *mut int numa_node; / NUMA node this device is close to,

    pub /: *mut *mut dev_t devt; / dev_t, creates the sysfs "dev",
    pub /: *mut *mut u32 id; / device instance,
    pub devres_lock: spinlock_t,
    pub devres_head: list_head,
    pub class: *const class,
    pub /: *const *const *const *const attribute_group groups; / optional groups,
    pub dev): *mut *mut void (release)(struct device,
    pub iommu_group: *mut iommu_group,
    pub iommu: *mut dev_iommu,
    pub physical_location: *mut device_physical_location,
    pub removable: device_removable,
    pub DEV_FLAG_COUNT): DECLARE_BITMAP(flags,,
}

//
// struct device_link - Device link representation.
// @supplier: The device on the supplier end of the link.
// @s_node: Hook to the supplier device's list of links to consumers.
// @consumer: The device on the consumer end of the link.
// @c_node: Hook to the consumer device's list of links to suppliers.
// @link_dev: device used to expose link details in sysfs
// @status: The state of the link (with respect to the presence of drivers).
// @flags: Link flags.
// @rpm_active: Whether or not the consumer device is runtime-PM-active.
// @kref: Count repeated addition of the same link.
// @rm_work: Work structure used for removing the link.
// @supplier_preactivated: Supplier has been made active before consumer probe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_link {
    pub supplier: *mut device,
    pub s_node: list_head,
    pub consumer: *mut device,
    pub c_node: list_head,
    pub link_dev: device,
    pub status: device_link_state,
    pub flags: u32,
    pub rpm_active: refcount_t,
    pub kref: kref,
    pub rm_work: work_struct,
    pub /: *mut *mut bool supplier_preactivated; / Owned by consumer probe.,
}

extern "C" {
    pub fn __device_set_driver_override(dev: *mut device, s: *const c_char, len: usize) -> c_int;
}
//
// device_set_driver_override() - Helper to set or clear driver override.
// @dev: Device to change
// @s: NUL-terminated string, new driver name to force a match, pass empty
// string to clear it ("" or "\n", where the latter is only for sysfs
// interface).
//
// Helper to set or clear driver override of a device.
//
// Returns: 0 on success or a negative error code on failure.
//
extern "C" {
    pub fn __device_set_driver_override(_arg: dev, _arg: s, 0: s ? strlen(s) :) -> return;
}
//
// device_has_driver_override() - Check if a driver override has been set.
// @dev: device to check
//
// Returns true if a driver override has been set for this device.
//
// device_match_driver_override() - Match a driver against the device's driver_override.
// @dev: device to check
// @drv: driver to match against
//
// Returns > 0 if a driver override is set and matches the given driver, 0 if a
// driver override is set but does not match, or < 0 if a driver override is not
// set at all.
//
// device_iommu_mapped - Returns true when the device DMA is translated
// by an IOMMU
// @dev: Device to perform the check on
//
// Get the wakeup routines, which depend on struct device

//
// dev_name - Return a device's name.
// @dev: Device with name to get.
// Return: The kobject name of the device, or its initial name if unavailable.
//
// Use the init name until the kobject becomes available
extern "C" {
    pub fn kobject_name(_arg: &dev->kobj) -> return;
}
//
// dev_bus_name - Return a device's bus/class name, if at all possible
// @dev: struct device to get the bus/class name of
//
// Will return the name of the bus/class the device is attached to.  If it is
// not attached to a bus/class, an empty string will be returned.
//

//
// dev_pm_set_strict_midlayer - Update the device's power.strict_midlayer flag
// @dev: Target device.
// @val: New flag value.
//
// When set, power.strict_midlayer means that the middle layer power management
// code (typically, a bus type or a PM domain) does not expect its runtime PM
// suspend callback to be invoked at all during system-wide PM transitions and
// it does not expect its runtime PM resume callback to be invoked at any point
// when runtime PM is disabled for the device during system-wide PM transitions.
//

extern "C" {
    pub fn mutex_lock_interruptible(_arg: &dev->mutex) -> return;
}
extern "C" {
    pub fn mutex_trylock(_arg: &dev->mutex) -> return;
}
//
// High level routines for use by the bus drivers
//
extern "C" {
    pub fn device_register(dev: *mut device) -> int __must_check;
}
extern "C" {
    pub fn device_unregister(dev: *mut device);
}
extern "C" {
    pub fn device_initialize(dev: *mut device);
}
extern "C" {
    pub fn device_add(dev: *mut device) -> int __must_check;
}
extern "C" {
    pub fn device_del(dev: *mut device);
}
//
// device_find_child_by_name - device iterator for locating a child device.
// @parent: parent struct device
// @name: name of the child device
//
// This is similar to the device_find_child() function above, but it
// returns a reference to a device that has the name @name.
//
// NOTE: you will need to drop the reference with put_device() after use.
//
extern "C" {
    pub fn device_find_child(_arg: parent, _arg: name, _arg: device_match_name) -> return;
}
//
// device_find_any_child - device iterator for locating a child device, if any.
// @parent: parent struct device
//
// This is similar to the device_find_child() function above, but it
// returns a reference to a child device, if any.
//
// NOTE: you will need to drop the reference with put_device() after use.
//
extern "C" {
    pub fn device_find_child(_arg: parent, _arg: NULL, _arg: device_match_any) -> return;
}
extern "C" {
    pub fn device_rename(dev: *mut device, new_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn device_change_owner(dev: *mut device, kuid: kuid_t, kgid: kgid_t) -> c_int;
}

//
// device_lock_set_class - Specify a temporary lock class while a device
// is attached to a driver
// @dev: device to modify
// @key: lock class key data
//
// This must be called with the device_lock() already held, for example
// from driver ->probe(). Take care to only override the default
// lockdep_no_validate class.
//

//
// device_lock_reset_class - Return a device to the default lockdep novalidate state
// @dev: device to modify
//
// This must be called with the device_lock() already held, for example
// from driver ->remove().
//

extern "C" {
    pub fn lock_device_hotplug();
}
extern "C" {
    pub fn unlock_device_hotplug();
}
extern "C" {
    pub fn lock_device_hotplug_sysfs() -> c_int;
}
extern "C" {
    pub fn device_offline(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn device_online(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn set_primary_fwnode(dev: *mut device, fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn set_secondary_fwnode(dev: *mut device, fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn device_set_node(dev: *mut device, fwnode: *mut fwnode_handle);
}
extern "C" {
    pub fn device_add_of_node(dev: *mut device, of_node: *mut device_node) -> c_int;
}
extern "C" {
    pub fn device_remove_of_node(dev: *mut device);
}
extern "C" {
    pub fn device_set_of_node_from_dev(dev: *mut device, dev2: *const device);
}
//
// Root device objects for grouping under /sys/devices
//
// This is a macro to avoid include problems with THIS_MODULE

extern "C" {
    pub fn root_device_unregister(root: *mut device);
}
//
// Manual binding of a device to driver. See drivers/base/bus.c
// for information on use.
//
extern "C" {
    pub fn device_bind_driver(dev: *mut device) -> int __must_check;
}
extern "C" {
    pub fn device_release_driver(dev: *mut device);
}
extern "C" {
    pub fn device_attach(dev: *mut device) -> int  __must_check;
}
extern "C" {
    pub fn driver_attach(drv: *const device_driver) -> int __must_check;
}
extern "C" {
    pub fn device_initial_probe(dev: *mut device);
}
extern "C" {
    pub fn device_reprobe(dev: *mut device) -> int __must_check;
}
extern "C" {
    pub fn device_is_bound(dev: *mut device) -> bool;
}
//
// Easy functions for dynamically creating devices on the fly
//
extern "C" {
    pub fn device_destroy(cls: *const class, devt: dev_t);
}
extern "C" {
    pub fn device_add_groups(_arg: dev, _arg: groups) -> return;
}
//
// get_device - atomically increment the reference count for the device.
//
extern "C" {
    pub fn put_device(dev: *mut device);
}
extern "C" {
    pub fn kill_device(dev: *mut device) -> bool;
}

extern "C" {
    pub fn devtmpfs_mount() -> c_int;
}

// drivers/base/power/shutdown.c
extern "C" {
    pub fn device_shutdown();
}
// debugging and troubleshooting/diagnostic helpers.
// Device links interface.
extern "C" {
    pub fn device_link_del(link: *mut device_link);
}
extern "C" {
    pub fn device_link_remove(consumer: *mut c_void, supplier: *mut device);
}
extern "C" {
    pub fn device_links_supplier_sync_state_pause();
}
extern "C" {
    pub fn device_links_supplier_sync_state_resume();
}
extern "C" {
    pub fn device_link_wait_removal();
}
// Create alias, so I can be autoloaded.

