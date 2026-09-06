//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/thunderbolt.h
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
// Thunderbolt service API
//
// Copyright (C) 2014 Andreas Noever <andreas.noever@gmail.com>
// Copyright (C) 2017, Intel Corporation
// Authors: Michael Jamet <michael.jamet@intel.com>
// Mika Westerberg <mika.westerberg@linux.intel.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_cfg_pkg_type {
    TB_CFG_PKG_READ = 1,
    TB_CFG_PKG_WRITE = 2,
    TB_CFG_PKG_ERROR = 3,
    TB_CFG_PKG_NOTIFY_ACK = 4,
    TB_CFG_PKG_EVENT = 5,
    TB_CFG_PKG_XDOMAIN_REQ = 6,
    TB_CFG_PKG_XDOMAIN_RESP = 7,
    TB_CFG_PKG_OVERRIDE = 8,
    TB_CFG_PKG_RESET = 9,
    TB_CFG_PKG_ICM_EVENT = 10,
    TB_CFG_PKG_ICM_CMD = 11,
    TB_CFG_PKG_ICM_RESP = 12,
}

//
// enum tb_security_level - Thunderbolt security level
// @TB_SECURITY_NONE: No security, legacy mode
// @TB_SECURITY_USER: User approval required at minimum
// @TB_SECURITY_SECURE: One time saved key required at minimum
// @TB_SECURITY_DPONLY: Only tunnel Display port (and USB)
// @TB_SECURITY_USBONLY: Only tunnel USB controller of the connected
// Thunderbolt dock (and Display Port). All PCIe
// links downstream of the dock are removed.
// @TB_SECURITY_NOPCIE: For USB4 systems this level is used when the
// PCIe tunneling is disabled from the BIOS.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_security_level {
    TB_SECURITY_NONE,
    TB_SECURITY_USER,
    TB_SECURITY_SECURE,
    TB_SECURITY_DPONLY,
    TB_SECURITY_USBONLY,
    TB_SECURITY_NOPCIE,
}

//
// struct tb - main thunderbolt bus structure
// @dev: Domain device
// @lock: Big lock. Must be held when accessing any struct
// tb_switch / struct tb_port.
// @nhi: Pointer to the NHI structure
// @ctl: Control channel for this domain
// @wq: Ordered workqueue for all domain specific work
// @root_switch: Root switch of this domain
// @cm_ops: Connection manager specific operations vector
// @index: Linux assigned domain number
// @security_level: Current security level
// @nboot_acl: Number of boot ACLs the domain supports
// @privdata: Private connection manager specific data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb {
    pub dev: device,
    pub lock: mutex,
    pub nhi: *mut tb_nhi,
    pub ctl: *mut tb_ctl,
    pub wq: *mut workqueue_struct,
    pub root_switch: *mut tb_switch,
    pub cm_ops: *const tb_cm_ops,
    pub index: c_int,
    pub security_level: tb_security_level,
    pub nboot_acl: usize,
    pub privdata: [c_ulong; ],
}

pub const TB_LINKS_PER_PHY_PORT: c_int = 2;
//
// struct tb_property_dir - XDomain property directory
// @uuid: Directory UUID or %NULL if root directory
// @properties: List of properties in this directory
//
// User needs to provide serialization if needed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_property_dir {
    pub uuid: *const uuid_t,
    pub properties: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_property_type {
    TB_PROPERTY_TYPE_UNKNOWN = 0x00,
    TB_PROPERTY_TYPE_DIRECTORY = 0x44,
    TB_PROPERTY_TYPE_DATA = 0x64,
    TB_PROPERTY_TYPE_TEXT = 0x74,
    TB_PROPERTY_TYPE_VALUE = 0x76,
}

pub const TB_PROPERTY_KEY_SIZE: c_int = 8;
//
// struct tb_property - XDomain property
// @list: Used to link properties together in a directory
// @key: Key for the property (always terminated).
// @type: Type of the property
// @length: Length of the property data in dwords
// @value: Property value
//
// Users use @type to determine which field in @value is filled.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_property {
    pub list: list_head,
    pub 1]: char key[TB_PROPERTY_KEY_SIZE +,
    pub type: tb_property_type,
    pub length: usize,
    pub dir: *mut tb_property_dir,
    pub data: *mut u8,
    pub text: *mut c_char,
    pub immediate: u32,
    pub value: },
}

extern "C" {
    pub fn tb_property_free_dir(dir: *mut tb_property_dir);
}
extern "C" {
    pub fn tb_property_remove(tb_property: *mut tb_property);
}

extern "C" {
    pub fn tb_register_property_dir(key: *const c_char, dir: *mut tb_property_dir) -> c_int;
}
extern "C" {
    pub fn tb_unregister_property_dir(key: *const c_char, dir: *mut tb_property_dir);
}
//
// enum tb_link_width - Thunderbolt/USB4 link width
// @TB_LINK_WIDTH_SINGLE: Single lane link
// @TB_LINK_WIDTH_DUAL: Dual lane symmetric link
// @TB_LINK_WIDTH_ASYM_TX: Dual lane asymmetric Gen 4 link with 3 transmitters
// @TB_LINK_WIDTH_ASYM_RX: Dual lane asymmetric Gen 4 link with 3 receivers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tb_link_width {
    TB_LINK_WIDTH_SINGLE = BIT(0),
    TB_LINK_WIDTH_DUAL = BIT(1),
    TB_LINK_WIDTH_ASYM_TX = BIT(2),
    TB_LINK_WIDTH_ASYM_RX = BIT(3),
}

//
// struct tb_xdomain - Cross-domain (XDomain) connection
// @dev: XDomain device
// @tb: Pointer to the domain
// @remote_uuid: UUID of the remote domain (host)
// @local_uuid: Cached local UUID
// @route: Route string the other domain can be reached
// @vendor: Vendor ID of the remote domain
// @device: Device ID of the demote domain
// @local_max_hopid: Maximum input HopID of this host
// @remote_max_hopid: Maximum input HopID of the remote host
// @lock: Lock to serialize access to the following fields of this structure
// @vendor_name: Name of the vendor (or %NULL if not known)
// @device_name: Name of the device (or %NULL if not known)
// @link_speed: Speed of the link in Gb/s
// @link_width: Width of the downstream facing link
// @link_usb4: Downstream link is USB4
// @is_unplugged: The XDomain is unplugged
// @removing: Set by tb_xdomain_remove() under @lock to prevent
// concurrent delayed work queueing
// @needs_uuid: If the XDomain does not have @remote_uuid it will be
// queried first
// @service_ids: Used to generate IDs for the services
// @in_hopids: Input HopIDs for DMA tunneling
// @out_hopids: Output HopIDs for DMA tunneling
// @local_property_block: Local block of properties
// @local_property_block_gen: Generation of @local_property_block
// @local_property_block_len: Length of the @local_property_block in dwords
// @remote_properties: Properties exported by the remote domain
// @remote_property_block_gen: Generation of @remote_properties
// @state: Next XDomain discovery state to run
// @state_work: Work used to run the next state
// @state_retries: Number of retries remain for the state
// @properties_changed_work: Work used to notify the remote domain that
// our properties have changed
// @properties_changed_retries: Number of times left to send properties
// changed notification
// @bonding_possible: True if lane bonding is possible on local side
// @target_link_width: Target link width from the remote host
// @ntunnels: Keeps track of how many tunnels go through this XDomain
// @link: Root switch link the remote domain is connected (ICM only)
// @depth: Depth in the chain the remote domain is connected (ICM only)
//
// This structure represents connection across two domains (hosts).
// Each XDomain contains zero or more services which are exposed as
// &struct tb_service objects.
//
// Service drivers may access this structure if they need to enumerate
// non-standard properties but they need hold @lock when doing so
// because properties can be changed asynchronously in response to
// changes in the remote domain.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_xdomain {
    pub dev: device,
    pub tb: *mut tb,
    pub remote_uuid: *mut uuid_t,
    pub local_uuid: *const uuid_t,
    pub route: u64,
    pub vendor: u16,
    pub device: u16,
    pub local_max_hopid: c_uint,
    pub remote_max_hopid: c_uint,
    pub lock: mutex,
    pub vendor_name: *const c_char,
    pub device_name: *const c_char,
    pub link_speed: c_uint,
    pub link_width: tb_link_width,
    pub link_usb4: bool,
    pub is_unplugged: bool,
    pub removing: bool,
    pub needs_uuid: bool,
    pub service_ids: ida,
    pub in_hopids: ida,
    pub out_hopids: ida,
    pub local_property_block: *mut u32,
    pub local_property_block_gen: u32,
    pub local_property_block_len: u32,
    pub remote_properties: *mut tb_property_dir,
    pub remote_property_block_gen: u32,
    pub state: c_int,
    pub state_work: delayed_work,
    pub state_retries: c_int,
    pub properties_changed_work: delayed_work,
    pub properties_changed_retries: c_int,
    pub bonding_possible: bool,
    pub target_link_width: u8,
    pub ntunnels: core::sync::atomic::AtomicI32,
    pub link: u8,
    pub depth: u8,
}

extern "C" {
    pub fn tb_xdomain_lane_bonding_enable(xd: *mut tb_xdomain) -> c_int;
}
extern "C" {
    pub fn tb_xdomain_lane_bonding_disable(xd: *mut tb_xdomain);
}
extern "C" {
    pub fn tb_xdomain_alloc_in_hopid(xd: *mut tb_xdomain, hopid: c_int) -> c_int;
}
extern "C" {
    pub fn tb_xdomain_release_in_hopid(xd: *mut tb_xdomain, hopid: c_int);
}
extern "C" {
    pub fn tb_xdomain_alloc_out_hopid(xd: *mut tb_xdomain, hopid: c_int) -> c_int;
}
extern "C" {
    pub fn tb_xdomain_release_out_hopid(xd: *mut tb_xdomain, hopid: c_int);
}
extern "C" {
    pub fn tb_xdomain_disable_paths(_arg: xd, _arg: -1, _arg: -1, _arg: -1, _arg: -1) -> return;
}
extern "C" {
    pub fn container_of(_arg: dev, tb_xdomain: struct, _arg: dev) -> return;
}
//
// struct tb_protocol_handler - Protocol specific handler
// @uuid: XDomain messages with this UUID are dispatched to this handler
// @callback: Callback called with the XDomain message. Returning %1
// here tells the XDomain core that the message was handled
// by this handler and should not be forwared to other
// handlers.
// @data: Data passed with the callback
// @list: Handlers are linked using this
//
// Thunderbolt services can hook into incoming XDomain requests by
// registering protocol handler. Only limitation is that the XDomain
// discovery protocol UUID cannot be registered since it is handled by
// the core XDomain code.
//
// The @callback must check that the message is really directed to the
// service the driver implements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_protocol_handler {
    pub uuid: *const uuid_t,
    pub data): *const *const *const int (callback)(void buf, size_t size, void,
    pub data: *mut c_void,
    pub list: list_head,
}

extern "C" {
    pub fn tb_register_protocol_handler(handler: *mut tb_protocol_handler) -> c_int;
}
extern "C" {
    pub fn tb_unregister_protocol_handler(handler: *mut tb_protocol_handler);
}
//
// struct tb_service - Thunderbolt service
// @dev: XDomain device
// @id: ID of the service (shown in sysfs)
// @key: Protocol key from the properties directory
// @prtcid: Protocol ID from the properties directory
// @prtcvers: Protocol version from the properties directory
// @prtcrevs: Protocol software revision from the properties directory
// @prtcstns: Protocol settings mask from the properties directory
// @lock: Protects this structure
// @local_properties: Properties owned by the service driver
// @remote_properties: Properties read from the remote service. These
// are read-only.
// @debugfs_dir: Pointer to the service debugfs directory. Always created
// when debugfs is enabled. Can be used by service drivers to
// add their own entries under the service.
//
// Each domain exposes set of services it supports as collection of
// properties. For each service there will be one corresponding
// &struct tb_service. Service drivers are bound to these.
//
// Service drivers can add their own dynamic properties to
// @local_properties but whenever they do so @lock must be held.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_service {
    pub dev: device,
    pub id: c_int,
    pub key: *const c_char,
    pub prtcid: u32,
    pub prtcvers: u32,
    pub prtcrevs: u32,
    pub prtcstns: u32,
    pub lock: mutex,
    pub local_properties: *mut tb_property_dir,
    pub remote_properties: *mut tb_property_dir,
    pub debugfs_dir: *mut dentry,
}

extern "C" {
    pub fn container_of(_arg: dev, tb_service: struct, _arg: dev) -> return;
}
//
// struct tb_service_driver - Thunderbolt service driver
// @driver: Driver structure
// @probe: Called when the driver is probed
// @remove: Called when the driver is removed (optional)
// @shutdown: Called at shutdown time to stop the service (optional)
// @id_table: Table of service identifiers the driver supports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_service_driver {
    pub driver: device_driver,
    pub svc): *mut *mut int (probe)(struct tb_service,
    pub svc): *mut *mut void (remove)(struct tb_service,
    pub svc): *mut *mut void (shutdown)(struct tb_service,
    pub id_table: *const tb_service_id,
}

extern "C" {
    pub fn tb_register_service_driver(drv: *mut tb_service_driver) -> c_int;
}
extern "C" {
    pub fn tb_unregister_service_driver(drv: *mut tb_service_driver);
}
extern "C" {
    pub fn dev_get_drvdata(_arg: &svc->dev) -> return;
}
extern "C" {
    pub fn tb_to_xdomain(_arg: svc->dev.parent) -> return;
}
extern "C" {
    pub fn tb_service_properties_changed(svc: *mut tb_service);
}
//
// struct tb_nhi - thunderbolt native host interface
// @lock: Must be held during ring creation/destruction. Is acquired by
// interrupt_work when dispatching interrupts to individual rings.
// @dev: Device associated with this NHI instance
// @ops: NHI specific optional ops
// @iobase: MMIO space of the NHI
// @tx_rings: All Tx rings available on this host controller
// @rx_rings: All Rx rings available on this host controller
// @going_away: The host controller device is about to disappear so when
// this flag is set, avoid touching the hardware anymore.
// @iommu_dma_protection: An IOMMU will isolate external-facing ports.
// @interrupt_work: Work scheduled to handle ring interrupt when no
// MSI-X is used.
// @hop_count: Number of rings (end point hops) supported by NHI.
// @quirks: NHI specific quirks if any
// @domain_released: Completed when domain has been fully released
// @host_reset: Host router was reset on driver load, or forced on system
// shutdown/reboot. When set, tb_stop() asserts DPR on connected
// downstream ports to signal disconnect before tearing down the
// router tree. Only Thunderbolt 3 devices are reset; USB4
// routers are skipped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_nhi {
    pub lock: spinlock_t,
    pub dev: *mut device,
    pub ops: *const tb_nhi_ops,
    pub iobase: *mut void __iomem,
    pub tx_rings: *mut tb_ring,
    pub rx_rings: *mut tb_ring,
    pub going_away: bool,
    pub iommu_dma_protection: bool,
    pub interrupt_work: work_struct,
    pub hop_count: u32,
    pub quirks: c_ulong,
    pub domain_released: completion,
    pub host_reset: bool,
}

//
// struct tb_ring - thunderbolt TX or RX ring associated with a NHI
// @lock: Lock serializing actions to this ring. Must be acquired after
// nhi->lock.
// @nhi: Pointer to the native host controller interface
// @size: Size of the ring
// @hop: Hop (DMA channel) associated with this ring
// @head: Head of the ring (write next descriptor here)
// @tail: Tail of the ring (complete next descriptor here)
// @descriptors: Allocated descriptors for this ring
// @descriptors_dma: DMA address of descriptors for this ring
// @queue: Queue holding frames to be transferred over this ring
// @in_flight: Queue holding frames that are currently in flight
// @work: Interrupt work structure
// @is_tx: Is the ring Tx or Rx
// @running: Is the ring running
// @irq: MSI-X irq number if the ring uses MSI-X. %0 otherwise.
// @vector: MSI-X vector number the ring uses (only set if @irq is > 0)
// @flags: Ring specific flags
// @e2e_tx_hop: Transmit HopID when E2E is enabled. Only applicable to
// RX ring. For TX ring this should be set to %0.
// @sof_mask: Bit mask used to detect start of frame PDF
// @eof_mask: Bit mask used to detect end of frame PDF
// @start_poll: Called when ring interrupt is triggered to start
// polling. Passing %NULL keeps the ring in interrupt mode.
// @poll_data: Data passed to @start_poll
// @interval_nsec: Interval counter if interrupt throttling is to be
// used with this ring (in ns)
// @wait: Used to signal that the ring may be empty now
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tb_ring {
    pub lock: spinlock_t,
    pub nhi: *mut tb_nhi,
    pub size: c_int,
    pub hop: c_int,
    pub head: c_int,
    pub tail: c_int,
    pub descriptors: *mut ring_desc,
    pub descriptors_dma: dma_addr_t,
    pub queue: list_head,
    pub in_flight: list_head,
    pub work: work_struct,
    pub is_tx:1: bool,
    pub running:1: bool,
    pub irq: c_int,
    pub vector: u8,
    pub flags: c_uint,
    pub e2e_tx_hop: c_int,
    pub sof_mask: u16,
    pub eof_mask: u16,
    pub data): *mut *mut void (start_poll)(void,
    pub poll_data: *mut c_void,
    pub interval_nsec: c_uint,
    pub wait: wait_queue_head_t,
}

// Leave ring interrupt enabled on suspend

// Configure the ring to be in frame mode

// Enable end-to-end flow control

// Do not enable interrupt for the ring

extern "C" {
    pub fn void(: *mut *mut ring_cb)(struct tb_ring, : *mut ring_frame, canceled: bool) -> typedef;
}
//
// enum ring_desc_flags - Flags for DMA ring descriptor
// @RING_DESC_ISOCH: Enable isonchronous DMA (Tx only)
// @RING_DESC_CRC_ERROR: In frame mode CRC check failed for the frame (Rx only)
// @RING_DESC_COMPLETED: Descriptor completed (set by NHI)
// @RING_DESC_POSTED: Always set this
// @RING_DESC_BUFFER_OVERRUN: RX buffer overrun
// @RING_DESC_INTERRUPT: Request an interrupt on completion
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ring_desc_flags {
    RING_DESC_ISOCH = 0x1,
    RING_DESC_CRC_ERROR = 0x1,
    RING_DESC_COMPLETED = 0x2,
    RING_DESC_POSTED = 0x4,
    RING_DESC_BUFFER_OVERRUN = 0x04,
    RING_DESC_INTERRUPT = 0x8,
}

//
// struct ring_frame - For use with ring_rx/ring_tx
// @buffer_phy: DMA mapped address of the frame
// @callback: Callback called when the frame is finished (optional)
// @list: Frame is linked to a queue using this
// @size: Size of the frame in bytes (%0 means %4096)
// @flags: Flags for the frame (see &enum ring_desc_flags)
// @eof: End of frame protocol defined field
// @sof: Start of frame protocol defined field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_frame {
    pub buffer_phy: dma_addr_t,
    pub callback: ring_cb,
    pub list: list_head,
    pub size:12: u32,
    pub flags:12: u32,
    pub eof:4: u32,
    pub sof:4: u32,
}

// Minimum size for ring_rx
pub const TB_FRAME_SIZE: c_int = 256;
pub const TB_MAX_FRAME_SIZE: c_int = 4096;
extern "C" {
    pub fn tb_ring_start(ring: *mut tb_ring);
}
extern "C" {
    pub fn tb_ring_flush(ring: *mut tb_ring, timeout_msec: c_uint) -> bool;
}
extern "C" {
    pub fn tb_ring_stop(ring: *mut tb_ring);
}
extern "C" {
    pub fn tb_ring_free(ring: *mut tb_ring);
}
extern "C" {
    pub fn __tb_ring_enqueue(ring: *mut tb_ring, frame: *mut ring_frame) -> c_int;
}
//
// tb_ring_rx() - enqueue a frame on an RX ring
// @ring: Ring to enqueue the frame
// @frame: Frame to enqueue
//
// @frame->buffer, @frame->buffer_phy have to be set. The buffer must
// contain at least %TB_FRAME_SIZE bytes.
//
// @frame->callback will be invoked with @frame->size, @frame->flags,
// @frame->eof, @frame->sof set once the frame has been received.
//
// If ring_stop() is called after the packet has been enqueued
// @frame->callback will be called with canceled set to true.
//
// Return: %-ESHUTDOWN if ring_stop() has been called, %0 otherwise.
//
extern "C" {
    pub fn __tb_ring_enqueue(_arg: ring, _arg: frame) -> return;
}
//
// tb_ring_tx() - enqueue a frame on an TX ring
// @ring: Ring the enqueue the frame
// @frame: Frame to enqueue
//
// @frame->buffer, @frame->buffer_phy, @frame->size, @frame->eof and
// @frame->sof have to be set.
//
// @frame->callback will be invoked with once the frame has been transmitted.
//
// If ring_stop() is called after the packet has been enqueued @frame->callback
// will be called with canceled set to true.
//
// Return: %-ESHUTDOWN if ring_stop has been called, %0 otherwise.
//
extern "C" {
    pub fn __tb_ring_enqueue(_arg: ring, _arg: frame) -> return;
}
// Used only when the ring is in polling mode
extern "C" {
    pub fn tb_ring_poll_complete(ring: *mut tb_ring);
}
extern "C" {
    pub fn tb_ring_throttling(ring: *mut tb_ring, interval_nsec: c_uint) -> c_int;
}
//
// tb_ring_dma_device() - Return device used for DMA mapping
// @ring: Ring whose DMA device is retrieved
//
// Use this function when you are mapping DMA for buffers that are
// passed to the ring for sending/receiving.
//
// Return: Pointer to device used for DMA mapping.
//

extern "C" {
    pub fn tb_configfs_register_group(group: *mut config_group) -> c_int;
}
extern "C" {
    pub fn tb_configfs_unregister_group(group: *mut config_group);
}

