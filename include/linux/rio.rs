//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rio.h
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
// RapidIO interconnect services
// (RapidIO Interconnect Specification, http://www.rapidio.org)
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//

pub const RIO_INVALID_DESTID: c_uint = 0xffff;
pub const RIO_MAX_MPORTS: c_int = 8;
pub const RIO_MAX_MPORT_RESOURCES: c_int = 16;
pub const RIO_MAX_DEV_RESOURCES: c_int = 16;
pub const RIO_MAX_MPORT_NAME: c_int = 40;
pub const RIO_GLOBAL_TABLE: c_uint = 0xff	/* Indicates access of a switch's;
pub const RIO_INVALID_ROUTE: c_uint = 0xff	/* Indicates that a route table;

pub const RIO_MAX_MBOX: c_int = 4;
pub const RIO_MAX_MSG_SIZE: c_uint = 0x1000;
//
// Error values that may be returned by RIO functions.
//
pub const RIO_SUCCESSFUL: c_uint = 0x00;
pub const RIO_BAD_SIZE: c_uint = 0x81;
//
// For RIO devices, the region numbers are assigned this way:
//
// 0	RapidIO outbound doorbells
// 1-15	RapidIO memory regions
//
// For RIO master ports, the region number are assigned this way:
//
// 0	RapidIO inbound doorbells
// 1	RapidIO inbound mailboxes
// 2	RapidIO outbound mailboxes
//
pub const RIO_DOORBELL_RESOURCE: c_int = 0;
pub const RIO_INB_MBOX_RESOURCE: c_int = 1;
pub const RIO_OUTB_MBOX_RESOURCE: c_int = 2;
pub const RIO_PW_MSG_SIZE: c_int = 64;
//
// A component tag value (stored in the component tag CSR) is used as device's
// unique identifier assigned during enumeration. Besides being used for
// identifying switches (which do not have device ID register), it also is used
// by error management notification and therefore has to be assigned
// to endpoints as well.
//
pub const RIO_CTAG_RESRVD: c_uint = 0xfffe0000 /* Reserved */;
pub const RIO_CTAG_UDEVID: c_uint = 0x0001ffff /* Unique device identifier */;
//
// struct rio_switch - RIO switch info
// @node: Node in global list of switches
// @route_table: Copy of switch routing table
// @port_ok: Status of each port (one bit per port) - OK=1 or UNINIT=0
// @ops: pointer to switch-specific operations
// @lock: lock to serialize operations updates
// @nextdev: Array of per-port pointers to the next attached device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_switch {
    pub node: list_head,
    pub route_table: *mut u8,
    pub port_ok: u32,
    pub ops: *mut rio_switch_ops,
    pub lock: spinlock_t,
    pub nextdev: [*mut rio_dev; ],
}

//
// struct rio_switch_ops - Per-switch operations
// @owner: The module owner of this structure
// @add_entry: Callback for switch-specific route add function
// @get_entry: Callback for switch-specific route get function
// @clr_table: Callback for switch-specific clear route table function
// @set_domain: Callback for switch-specific domain setting function
// @get_domain: Callback for switch-specific domain get function
// @em_init: Callback for switch-specific error management init function
// @em_handle: Callback for switch-specific error management handler function
//
// Defines the operations that are necessary to initialize/control
// a particular RIO switch device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_switch_ops {
    pub owner: *mut module,
    pub route_port): u16 table, u16 route_destid, u8,
    pub route_port): *mut u16 table, u16 route_destid, u8,
    pub table): u16,
    pub sw_domain): u8,
    pub sw_domain): *mut u8,
    pub dev): *mut *mut int (em_init) (struct rio_dev,
    pub swport): *mut *mut *mut int (em_handle) (struct rio_dev dev, u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_device_state {
    RIO_DEVICE_INITIALIZING,
    RIO_DEVICE_RUNNING,
    RIO_DEVICE_GONE,
    RIO_DEVICE_SHUTDOWN,
}

//
// struct rio_dev - RIO device info
// @global_list: Node in list of all RIO devices
// @net_list: Node in list of RIO devices in a network
// @net: Network this device is a part of
// @do_enum: Enumeration flag
// @did: Device ID
// @vid: Vendor ID
// @device_rev: Device revision
// @asm_did: Assembly device ID
// @asm_vid: Assembly vendor ID
// @asm_rev: Assembly revision
// @efptr: Extended feature pointer
// @pef: Processing element features
// @swpinfo: Switch port info
// @src_ops: Source operation capabilities
// @dst_ops: Destination operation capabilities
// @comp_tag: RIO component tag
// @phys_efptr: RIO device extended features pointer
// @phys_rmap: LP-Serial Register Map Type (1 or 2)
// @em_efptr: RIO Error Management features pointer
// @dma_mask: Mask of bits of RIO address this device implements
// @driver: Driver claiming this device
// @dev: Device model device
// @riores: RIO resources this device owns
// @pwcback: port-write callback function for this device
// @destid: Network destination ID (or associated destid for switch)
// @hopcount: Hopcount to this device
// @prev: Previous RIO device connected to the current one
// @state: device state
// @rswitch: struct rio_switch (if valid for this device)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dev {
    pub /: *mut *mut list_head global_list; / node in list of all RIO devices,
    pub /: *mut *mut list_head net_list; / node in per net list,
    pub /: *mut *mut *mut rio_net net; / RIO net this device resides in,
    pub do_enum: bool,
    pub did: u16,
    pub vid: u16,
    pub device_rev: u32,
    pub asm_did: u16,
    pub asm_vid: u16,
    pub asm_rev: u16,
    pub efptr: u16,
    pub pef: u32,
    pub swpinfo: u32,
    pub src_ops: u32,
    pub dst_ops: u32,
    pub comp_tag: u32,
    pub phys_efptr: u32,
    pub phys_rmap: u32,
    pub em_efptr: u32,
    pub dma_mask: u64,
    pub /: *mut *mut *mut rio_driver driver; / RIO driver claiming this device,
    pub /: *mut *mut device dev; / LDM device structure,
    pub riores: [resource; RIO_MAX_DEV_RESOURCES],
    pub step): *mut *mut *mut *mut int (pwcback) (struct rio_dev rdev, union rio_pw_msg msg, int,
    pub destid: u16,
    pub hopcount: u8,
    pub prev: *mut rio_dev,
    pub state: core::sync::atomic::AtomicI32,
    pub /: *mut *mut rio_switch rswitch[]; / RIO switch info,
}

//
// struct rio_msg - RIO message event
// @res: Mailbox resource
// @mcback: Message event callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_msg {
    pub res: *mut resource,
    pub slot): *mut *mut *mut *mut void (mcback) (struct rio_mport  mport, void dev_id, int mbox, int,
}

//
// struct rio_dbell - RIO doorbell event
// @node: Node in list of doorbell events
// @res: Doorbell resource
// @dinb: Doorbell event callback
// @dev_id: Device specific pointer to pass on event
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dbell {
    pub node: list_head,
    pub res: *mut resource,
    pub info): *mut *mut *mut *mut void (dinb) (struct rio_mport mport, void dev_id, u16 src, u16 dst, u16,
    pub dev_id: *mut c_void,
}

//
// struct rio_mport - RIO master port info
// @dbells: List of doorbell events
// @pwrites: List of portwrite events
// @node: Node in global list of master ports
// @nnode: Node in network list of master ports
// @net: RIO net this mport is attached to
// @lock: lock to synchronize lists manipulations
// @iores: I/O mem resource that this master port interface owns
// @riores: RIO resources that this master port interfaces owns
// @inb_msg: RIO inbound message event descriptors
// @outb_msg: RIO outbound message event descriptors
// @host_deviceid: Host device ID associated with this master port
// @ops: configuration space functions
// @id: Port ID, unique among all ports
// @index: Port index, unique among all port interfaces of the same type
// @sys_size: RapidIO common transport system size
// @phys_efptr: RIO port extended features pointer
// @phys_rmap: LP-Serial EFB Register Mapping type (1 or 2).
// @name: Port name string
// @dev: device structure associated with an mport
// @priv: Master port private data
// @dma: DMA device associated with mport
// @nscan: RapidIO network enumeration/discovery operations
// @state: mport device state
// @pwe_refcnt: port-write enable ref counter to track enable/disable requests
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_mport {
    pub /: *mut *mut list_head dbells; / list of doorbell events,
    pub /: *mut *mut list_head pwrites; / list of portwrite events,
    pub /: *mut *mut list_head node; / node in global list of ports,
    pub /: *mut *mut list_head nnode; / node in net list of ports,
    pub /: *mut *mut *mut rio_net net; / RIO net this mport is attached to,
    pub lock: mutex,
    pub iores: resource,
    pub riores: [resource; RIO_MAX_MPORT_RESOURCES],
    pub inb_msg: [rio_msg; RIO_MAX_MBOX],
    pub outb_msg: [rio_msg; RIO_MAX_MBOX],
    pub /: *mut *mut int host_deviceid; / Host device ID,
    pub /: *mut *mut *mut rio_ops ops; / low-level architecture-dependent routines,
    pub /: *mut *mut unsigned char id; / port ID, unique among all ports,
    pub port: *mut *mut unsigned char index; / port index, unique among all,
    pub size.: *mut *mut unsigned int sys_size; / RapidIO common transport system,
// 0 - Small size. 256 devices.
// 1 - Large size, 65536 devices.
//
    pub phys_efptr: u32,
    pub phys_rmap: u32,
    pub name: [c_uchar; RIO_MAX_MPORT_NAME],
    pub dev: device,
    pub /: *mut *mut *mut void priv; / Master port private data,

    pub dma: dma_device,

    pub nscan: *mut rio_scan,
    pub state: core::sync::atomic::AtomicI32,
    pub pwe_refcnt: c_uint,
}

//
// Enumeration/discovery control flags
//
pub const RIO_SCAN_ENUM_NO_WAIT: c_uint = 0x00000001 /* Do not wait for enum completed */;
//
// struct rio_net - RIO network info
// @node: Node in global list of RIO networks
// @devices: List of devices in this network
// @switches: List of switches in this network
// @mports: List of master ports accessing this network
// @hport: Default port for accessing this network
// @id: RIO network ID
// @dev: Device object
// @enum_data: private data specific to a network enumerator
// @release: enumerator-specific release callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_net {
    pub /: *mut *mut list_head node; / node in list of networks,
    pub /: *mut *mut list_head devices; / list of devices in this net,
    pub /: *mut *mut list_head switches; / list of switches in this net,
    pub /: *mut *mut list_head mports; / list of ports accessing net,
    pub /: *mut *mut *mut rio_mport hport; / primary port for accessing net,
    pub /: *mut *mut unsigned char id; / RIO network ID,
    pub dev: device,
    pub /: *mut *mut *mut void enum_data; / private data for enumerator of the network,
    pub net): *mut *mut void (release)(struct rio_net,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_link_speed {
    RIO_LINK_DOWN = 0, /* SRIO Link not initialized */
    RIO_LINK_125 = 1, /* 1.25 GBaud  */
    RIO_LINK_250 = 2, /* 2.5 GBaud   */
    RIO_LINK_312 = 3, /* 3.125 GBaud */
    RIO_LINK_500 = 4, /* 5.0 GBaud   */
    RIO_LINK_625 = 5  /* 6.25 GBaud  */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_link_width {
    RIO_LINK_1X  = 0,
    RIO_LINK_1XR = 1,
    RIO_LINK_2X  = 3,
    RIO_LINK_4X  = 2,
    RIO_LINK_8X  = 4,
    RIO_LINK_16X = 5
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_mport_flags {
    RIO_MPORT_DMA	 = (1 << 0), /* supports DMA data transfers */
    RIO_MPORT_DMA_SG = (1 << 1), /* DMA supports HW SG mode */
    RIO_MPORT_IBSG	 = (1 << 2), /* inbound mapping supports SG */
}

//
// struct rio_mport_attr - RIO mport device attributes
// @flags: mport device capability flags
// @link_speed: SRIO link speed value (as defined by RapidIO specification)
// @link_width:	SRIO link width value (as defined by RapidIO specification)
// @dma_max_sge: number of SG list entries that can be handled by DMA channel(s)
// @dma_max_size: max number of bytes in single DMA transfer (SG entry)
// @dma_align: alignment shift for DMA operations (as for other DMA operations)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_mport_attr {
    pub flags: c_int,
    pub link_speed: c_int,
    pub link_width: c_int,
// DMA capability info: valid only if RIO_MPORT_DMA flag is set
    pub dma_max_sge: c_int,
    pub dma_max_size: c_int,
    pub dma_align: c_int,
}

// Low-level architecture-dependent routines
//
// struct rio_ops - Low-level RIO configuration space operations
// @lcread: Callback to perform local (master port) read of config space.
// @lcwrite: Callback to perform local (master port) write of config space.
// @cread: Callback to perform network read of config space.
// @cwrite: Callback to perform network write of config space.
// @dsend: Callback to send a doorbell message.
// @pwenable: Callback to enable/disable port-write message handling.
// @open_outb_mbox: Callback to initialize outbound mailbox.
// @close_outb_mbox: Callback to shut down outbound mailbox.
// @open_inb_mbox: Callback to initialize inbound mailbox.
// @close_inb_mbox: Callback to	shut down inbound mailbox.
// @add_outb_message: Callback to add a message to an outbound mailbox queue.
// @add_inb_buffer: Callback to	add a buffer to an inbound mailbox queue.
// @get_inb_message: Callback to get a message from an inbound mailbox queue.
// @map_inb: Callback to map RapidIO address region into local memory space.
// @unmap_inb: Callback to unmap RapidIO address region mapped with map_inb().
// @query_mport: Callback to query mport device attributes.
// @map_outb: Callback to map outbound address region into local memory space.
// @unmap_outb: Callback to unmap outbound RapidIO address region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_ops {
    pub data): *mut u32,
    pub data): u32,
    pub data): *mut u8 hopcount, u32 offset, int len, u32,
    pub data): u8 hopcount, u32 offset, int len, u32,
    pub data): *mut *mut *mut int (dsend) (struct rio_mport mport, int index, u16 destid, u16,
    pub enable): *mut *mut *mut int (pwenable) (struct rio_mport mport, int,
    pub entries): int mbox, int,
    pub mbox): *mut *mut *mut void (close_outb_mbox)(struct rio_mport mport, int,
    pub entries): int mbox, int,
    pub mbox): *mut *mut *mut void (close_inb_mbox)(struct rio_mport mport, int,
    pub len): *mut *mut int mbox, void buffer, size_t,
    pub buf): *mut *mut *mut int (add_inb_buffer)(struct rio_mport mport, int mbox, void,
    pub mbox): *mut *mut *mut *mut void (get_inb_message)(struct rio_mport mport, int,
    pub flags): u64 rstart, u64 size, u32,
    pub lstart): *mut *mut *mut void (unmap_inb)(struct rio_mport mport, dma_addr_t,
    pub attr): *mut rio_mport_attr,
    pub laddr): *mut u32 size, u32 flags, dma_addr_t,
    pub rstart): *mut *mut *mut void (unmap_outb)(struct rio_mport mport, u16 destid, u64,
}

pub const RIO_RESOURCE_MEM: c_uint = 0x00000100;
pub const RIO_RESOURCE_DOORBELL: c_uint = 0x00000200;
pub const RIO_RESOURCE_MAILBOX: c_uint = 0x00000400;
pub const RIO_RESOURCE_CACHEABLE: c_uint = 0x00010000;
pub const RIO_RESOURCE_PCI: c_uint = 0x00020000;
pub const RIO_RESOURCE_BUSY: c_uint = 0x80000000;
//
// struct rio_driver - RIO driver info
// @node: Node in list of drivers
// @name: RIO driver name
// @id_table: RIO device ids to be associated with this driver
// @probe: RIO device inserted
// @remove: RIO device removed
// @shutdown: shutdown notification callback
// @suspend: RIO device suspended
// @resume: RIO device awakened
// @enable_wake: RIO device enable wake event
// @driver: LDM driver struct
//
// Provides info on a RIO device driver for insertion/removal and
// power management purposes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_driver {
    pub node: list_head,
    pub name: *mut c_char,
    pub id_table: *const rio_device_id,
    pub id): *const *const *const *const int (probe) (struct rio_dev  dev, struct rio_device_id,
    pub dev): *mut *mut *mut void (remove) (struct rio_dev,
    pub dev): *mut *mut void (shutdown)(struct rio_dev,
    pub state): *mut *mut *mut int (suspend) (struct rio_dev  dev, u32,
    pub dev): *mut *mut *mut int (resume) (struct rio_dev,
    pub enable): *mut *mut *mut int (enable_wake) (struct rio_dev  dev, u32 state, int,
    pub driver: device_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union rio_pw_msg {
    pub /: *mut *mut u32 comptag; / Component Tag CSR,
    pub /: *mut *mut u32 errdetect; / Port N Error Detect CSR,
    pub /: *mut *mut u32 is_port; / Implementation specific + PortID,
    pub /: *mut *mut u32 ltlerrdet; / LTL Error Detect CSR,
    pub padding: [u32; 12],
    pub em: },
    pub raw: [u32; RIO_PW_MSG_SIZE/sizeof(u32)],
}

//
// enum rio_write_type - RIO write transaction types used in DMA transfers
//
// Note: RapidIO specification defines write (NWRITE) and
// write-with-response (NWRITE_R) data transfer operations.
// Existing DMA controllers that service RapidIO may use one of these operations
// for entire data transfer or their combination with only the last data packet
// requires response.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rio_write_type {
    RDW_DEFAULT,		/* default method used by DMA driver */
    RDW_ALL_NWRITE,		/* all packets use NWRITE */
    RDW_ALL_NWRITE_R,	/* all packets use NWRITE_R */
    RDW_LAST_NWRITE_R,	/* last packet uses NWRITE_R, others - NWRITE */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dma_ext {
    pub destid: u16,
    pub /: *mut *mut u64 rio_addr; / low 64-bits of 66-bit RapidIO address,
    pub /: *mut *mut u8 rio_addr_u; / upper 2-bits of 66-bit RapidIO address,
    pub /: *mut *mut rio_write_type wr_type; / preferred RIO write operation type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_dma_data {
// Local data (as scatterlist)
    pub /: *mut *mut *mut scatterlist sg; / I/O scatter list,
    pub /: *mut *mut unsigned int sg_len; / size of scatter list,
// Remote device address (flat buffer)
    pub /: *mut *mut u64 rio_addr; / low 64-bits of 66-bit RapidIO address,
    pub /: *mut *mut u8 rio_addr_u; / upper 2-bits of 66-bit RapidIO address,
    pub /: *mut *mut rio_write_type wr_type; / preferred RIO write operation type,
}

extern "C" {
    pub fn container_of(_arg: ddev, rio_mport: struct, _arg: dma) -> return;
}

//
// struct rio_scan - RIO enumeration and discovery operations
// @owner: The module owner of this structure
// @enumerate: Callback to perform RapidIO fabric enumeration.
// @discover: Callback to perform RapidIO fabric discovery.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_scan {
    pub owner: *mut module,
    pub flags): *mut *mut *mut int (enumerate)(struct rio_mport mport, u32,
    pub flags): *mut *mut *mut int (discover)(struct rio_mport mport, u32,
}

//
// struct rio_scan_node - list node to register RapidIO enumeration and
// discovery methods with RapidIO core.
// @mport_id: ID of an mport (net) serviced by this enumerator
// @node: node in global list of registered enumerators
// @ops: RIO enumeration and discovery operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio_scan_node {
    pub mport_id: c_int,
    pub node: list_head,
    pub ops: *mut rio_scan,
}

// Architecture and hardware-specific functions
extern "C" {
    pub fn rio_mport_initialize(: *mut rio_mport) -> c_int;
}
extern "C" {
    pub fn rio_register_mport(: *mut rio_mport) -> c_int;
}
extern "C" {
    pub fn rio_unregister_mport(: *mut rio_mport) -> c_int;
}
extern "C" {
    pub fn rio_open_inb_mbox(: *mut rio_mport, : *mut c_void, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn rio_close_inb_mbox(: *mut rio_mport, _arg: c_int);
}
extern "C" {
    pub fn rio_open_outb_mbox(: *mut rio_mport, : *mut c_void, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn rio_close_outb_mbox(: *mut rio_mport, _arg: c_int);
}
