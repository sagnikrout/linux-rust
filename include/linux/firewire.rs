//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/firewire.h
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

pub const CSR_REGISTER_BASE: c_uint = 0xfffff0000000ULL;
// register offsets are relative to CSR_REGISTER_BASE
pub const CSR_STATE_CLEAR: c_uint = 0x0;
pub const CSR_STATE_SET: c_uint = 0x4;
pub const CSR_NODE_IDS: c_uint = 0x8;
pub const CSR_RESET_START: c_uint = 0xc;
pub const CSR_SPLIT_TIMEOUT_HI: c_uint = 0x18;
pub const CSR_SPLIT_TIMEOUT_LO: c_uint = 0x1c;
pub const CSR_CYCLE_TIME: c_uint = 0x200;
pub const CSR_BUS_TIME: c_uint = 0x204;
pub const CSR_BUSY_TIMEOUT: c_uint = 0x210;
pub const CSR_PRIORITY_BUDGET: c_uint = 0x218;
pub const CSR_BUS_MANAGER_ID: c_uint = 0x21c;
pub const CSR_BANDWIDTH_AVAILABLE: c_uint = 0x220;
pub const CSR_CHANNELS_AVAILABLE: c_uint = 0x224;
pub const CSR_CHANNELS_AVAILABLE_HI: c_uint = 0x224;
pub const CSR_CHANNELS_AVAILABLE_LO: c_uint = 0x228;
pub const CSR_MAINT_UTILITY: c_uint = 0x230;
pub const CSR_BROADCAST_CHANNEL: c_uint = 0x234;
pub const CSR_CONFIG_ROM: c_uint = 0x400;
pub const CSR_CONFIG_ROM_END: c_uint = 0x800;
pub const CSR_OMPR: c_uint = 0x900;

pub const CSR_IMPR: c_uint = 0x980;

pub const CSR_FCP_COMMAND: c_uint = 0xB00;
pub const CSR_FCP_RESPONSE: c_uint = 0xD00;
pub const CSR_FCP_END: c_uint = 0xF00;
pub const CSR_TOPOLOGY_MAP: c_uint = 0x1000;
pub const CSR_TOPOLOGY_MAP_END: c_uint = 0x1400;
pub const CSR_SPEED_MAP: c_uint = 0x2000;
pub const CSR_SPEED_MAP_END: c_uint = 0x3000;
pub const CSR_OFFSET: c_uint = 0x40;
pub const CSR_LEAF: c_uint = 0x80;
pub const CSR_DIRECTORY: c_uint = 0xc0;
pub const CSR_DESCRIPTOR: c_uint = 0x01;
pub const CSR_VENDOR: c_uint = 0x03;
pub const CSR_HARDWARE_VERSION: c_uint = 0x04;
pub const CSR_UNIT: c_uint = 0x11;
pub const CSR_SPECIFIER_ID: c_uint = 0x12;
pub const CSR_VERSION: c_uint = 0x13;
pub const CSR_DEPENDENT_INFO: c_uint = 0x14;
pub const CSR_MODEL: c_uint = 0x17;
pub const CSR_DIRECTORY_ID: c_uint = 0x20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_csr_iterator {
    pub p: *const u32,
    pub end: *const u32,
}

extern "C" {
    pub fn fw_csr_iterator_init(ci: *mut fw_csr_iterator, p: *const u32);
}
extern "C" {
    pub fn fw_csr_iterator_next(ci: *mut fw_csr_iterator, key: *mut c_int, value: *mut c_int) -> c_int;
}
extern "C" {
    pub fn fw_csr_string(directory: *const u32, key: c_int, buf: *mut c_char, size: usize) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_card {
    pub driver: *const fw_card_driver,
    pub device: *mut device,
    pub kref: kref,
    pub done: completion,
    pub node_id: c_int,
    pub generation: c_int,
    pub reset_jiffies: u64,
    pub current_tlabel: c_int,
    pub tlabel_mask: u64,
    pub list: list_head,
    pub lock: spinlock_t,
    pub transactions: },
    pub hi: u32,
    pub lo: u32,
    pub cycles: c_uint,
    pub jiffies: c_uint,
    pub lock: spinlock_t,
    pub split_timeout: },
    pub guid: c_ulonglong,
    pub max_receive: unsigned,
    pub link_speed: c_int,
    pub config_rom_generation: c_int,
    pub lock: spinlock_t,
    pub local_node: *mut fw_node,
    pub root_node: *mut fw_node,
    pub irm_node: *mut fw_node,
    pub /: *mut *mut u8 color; / must be u8 to match the definition in struct fw_node,
    pub gap_count: c_int,
    pub beta_repeaters_present: bool,
    pub index: c_int,
    pub link: list_head,
    pub /: *mut *mut delayed_work br_work; / bus reset job,
    pub br_short: bool,
    pub /: *mut *mut delayed_work bm_work; / bus manager job,
    pub bm_retries: c_int,
    pub bm_generation: c_int,
    pub bm_node_id: c_int,
    pub bm_abdicate: bool,
    pub /: *mut *mut bool priority_budget_implemented; / controller feature,
    pub /: *mut *mut bool broadcast_channel_auto_allocated; / controller feature,
    pub broadcast_channel_allocated: bool,
    pub broadcast_channel: u32,
    pub 4]: __be32 buffer[(CSR_TOPOLOGY_MAP_END - CSR_TOPOLOGY_MAP) /,
    pub lock: spinlock_t,
    pub topology_map: },
    pub maint_utility_register: __be32,
    pub isoc_wq: *mut workqueue_struct,
    pub async_wq: *mut workqueue_struct,
}

extern "C" {
    pub fn fw_card_release(kref: *mut kref);
}
extern "C" {
    pub fn fw_card_read_cycle_time(card: *mut fw_card, cycle_time: *mut u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_attribute_group {
    pub groups: [*mut attribute_group; 2],
    pub group: attribute_group,
    pub attrs: [*mut attribute; 13],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_device_quirk {
// See afa1282a35d3 ("firewire: core: check for 1394a compliant IRM, fix inaccessibility of Sony camcorder").
    FW_DEVICE_QUIRK_IRM_IS_1394_1995_ONLY = BIT(0),

// See a509e43ff338 ("firewire: core: fix unstable I/O with Canon camcorder").
    FW_DEVICE_QUIRK_IRM_IGNORES_BUS_MANAGER = BIT(1),

// MOTU Audio Express transfers acknowledge packet with 0x10 for pending state.
    FW_DEVICE_QUIRK_ACK_PACKET_WITH_INVALID_PENDING_CODE = BIT(2),

// TASCAM FW-1082/FW-1804/FW-1884 often freezes when receiving S400 packets.
    FW_DEVICE_QUIRK_UNSTABLE_AT_S400 = BIT(3),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_device_state {
    FW_DEVICE_INITIALIZING,
    FW_DEVICE_RUNNING,
    FW_DEVICE_GONE,
    FW_DEVICE_SHUTDOWN,
}

//
// Note, fw_device.generation always has to be read before fw_device.node_id.
// Use SMP memory barriers to ensure this.  Otherwise requests will be sent
// to an outdated node_id if the generation was updated in the meantime due
// to a bus reset.
//
// Likewise, fw-core will take care to update .node_id before .generation so
// that whenever fw_device.generation is current WRT the actual bus generation,
// fw_device.node_id is guaranteed to be current too.
//
// The same applies to fw_device.card->node_id vs. fw_device.generation.
//
// fw_device.config_rom and fw_device.config_rom_length may be accessed during
// the lifetime of any fw_unit belonging to the fw_device, before device_del()
// was called on the last fw_unit.  Alternatively, they may be accessed while
// holding fw_device_rwsem.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_device {
    pub state: core::sync::atomic::AtomicI32,
    pub node: *mut fw_node,
    pub node_id: c_int,
    pub generation: c_int,
    pub max_speed: unsigned,
    pub card: *mut fw_card,
    pub device: device,
// A set of enum fw_device_quirk.
    pub quirks: c_int,
    pub client_list_mutex: mutex,
    pub client_list: list_head,
    pub config_rom: *const u32,
    pub config_rom_length: usize,
    pub config_rom_retries: c_int,
    pub is_local:1: unsigned,
    pub max_rec:4: unsigned,
    pub cmc:1: unsigned,
    pub irmc:1: unsigned,
    pub bc_implemented:2: unsigned,
    pub workfn: work_func_t,
    pub work: delayed_work,
    pub attribute_group: fw_attribute_group,
}

extern "C" {
    pub fn fw_device_enable_phys_dma(device: *mut fw_device) -> c_int;
}
//
// fw_unit.directory must not be accessed after device_del(&fw_unit.device).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_unit {
    pub device: device,
    pub directory: *const u32,
    pub attribute_group: fw_attribute_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_driver {
    pub driver: device_driver,
    pub id): *const *const *const int (probe)(struct fw_unit unit, struct ieee1394_device_id,
// Called when the parent device sits through a bus reset.
    pub unit): *mut *mut void (update)(struct fw_unit,
    pub unit): *mut *mut void (remove)(struct fw_unit,
    pub id_table: *const ieee1394_device_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_transaction_callback {
    pub without_tstamp: fw_transaction_callback_t,
    pub with_tstamp: fw_transaction_callback_with_tstamp_t,
}

//
// This callback handles an inbound request subaction.  It is called in
// RCU read-side context, therefore must not sleep.
//
// The callback should not initiate outbound request subactions directly.
// Otherwise there is a danger of recursion of inbound and outbound
// transactions from and to the local node.
//
// The callback is responsible that fw_send_response() is called on the @request, except for FCP
// registers for which the core takes care of that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_packet {
    pub speed: c_int,
    pub generation: c_int,
    pub header: [u32; 4],
    pub header_length: usize,
    pub payload: *mut c_void,
    pub payload_length: usize,
    pub payload_bus: dma_addr_t,
    pub payload_mapped: bool,
    pub timestamp: u32,
//
// This callback is called when the packet transmission has completed.
// For successful transmission, the status code is the ack received
// from the destination.  Otherwise it is one of the juju-specific
// rcodes:  RCODE_SEND_ERROR, _CANCELLED, _BUSY, _GENERATION, _NO_ACK.
// The callback can be called from workqueue and thus must never block.
//
    pub callback: fw_packet_callback_t,
    pub ack: c_int,
    pub link: list_head,
    pub driver_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_transaction {
    pub /: *mut *mut int node_id; / The generation is implied; it is always the current.,
    pub tlabel: c_int,
    pub link: list_head,
    pub card: *mut fw_card,
    pub is_split_transaction: bool,
    pub split_timeout_timer: timer_list,
    pub split_timeout_cycle: u32,
    pub packet: fw_packet,
//
// The data passed to the callback is valid only during the
// callback.
//
    pub callback: fw_transaction_callback,
    pub with_tstamp: bool,
    pub callback_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_address_handler {
    pub offset: u64,
    pub length: u64,
    pub address_callback: fw_address_callback_t,
    pub callback_data: *mut c_void,
// Only for core functions.
    pub link: list_head,
    pub kref: kref,
    pub done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_address_region {
    pub start: u64,
    pub end: u64,
}

extern "C" {
    pub fn fw_core_remove_address_handler(handler: *mut fw_address_handler);
}
extern "C" {
    pub fn fw_get_request_speed(request: *mut fw_request) -> c_int;
}
extern "C" {
    pub fn fw_request_get_timestamp(request: *const fw_request) -> u32;
}
//
// fw_send_request() - submit a request packet for transmission to generate callback for response
// subaction without time stamp.
// @card:		interface to send the request at
// @t:			transaction instance to which the request belongs
// @tcode:		transaction code
// @destination_id:	destination node ID, consisting of bus_ID and phy_ID
// @generation:		bus generation in which request and response are valid
// @speed:		transmission speed
// @offset:		48bit wide offset into destination's address space
// @payload:		data payload for the request subaction
// @length:		length of the payload, in bytes
// @callback:		function to be called when the transaction is completed
// @callback_data:	data to be passed to the transaction completion callback
//
// A variation of __fw_send_request() to generate callback for response subaction without time
// stamp.
//
// The callback is invoked in the workqueue context in most cases. However, if an error is detected
// before queueing or the destination address refers to the local node, it is invoked in the
// current context instead.
//
// fw_send_request_with_tstamp() - submit a request packet for transmission to generate callback for
// response with time stamp.
// @card:		interface to send the request at
// @t:			transaction instance to which the request belongs
// @tcode:		transaction code
// @destination_id:	destination node ID, consisting of bus_ID and phy_ID
// @generation:		bus generation in which request and response are valid
// @speed:		transmission speed
// @offset:		48bit wide offset into destination's address space
// @payload:		data payload for the request subaction
// @length:		length of the payload, in bytes
// @callback:		function to be called when the transaction is completed
// @callback_data:	data to be passed to the transaction completion callback
//
// A variation of __fw_send_request() to generate callback for response subaction with time stamp.
//
// The callback is invoked in the workqueue context in most cases. However, if an error is detected
// before queueing or the destination address refers to the local node, it is invoked in the current
// context instead.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_descriptor {
    pub link: list_head,
    pub length: usize,
    pub immediate: u32,
    pub key: u32,
    pub data: *const u32,
}

extern "C" {
    pub fn fw_core_add_descriptor(desc: *mut fw_descriptor) -> c_int;
}
extern "C" {
    pub fn fw_core_remove_descriptor(desc: *mut fw_descriptor);
}
//
// The iso packet format allows for an immediate header/payload part
// stored in 'header' immediately after the packet info plus an
// indirect payload part that is pointer to by the 'payload' field.
// Applications can use one or the other or both to implement simple
// low-bandwidth streaming (e.g. audio) or more advanced
// scatter-gather streaming (e.g. assembling video frame automatically).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_iso_packet {
    pub /: *mut *mut u16 payload_length; / Length of indirect payload,
    pub /: *mut *mut u32 interrupt:1; / Generate interrupt on this packet,
    pub /: *mut *mut u32 skip:1; / tx: Set to not send packet at all,
// rx: Sync bit, wait for matching sy
    pub /: *mut *mut u32 tag:2; / tx: Tag in packet header,
    pub /: *mut *mut u32 sy:4; / tx: Sy in packet header,
    pub /: *mut *mut u32 header_length:8; / Size of immediate header,
    pub /: *mut *mut u32 header[]; / tx: Top of 1394 isoch. data_block,
}

pub const FW_ISO_CONTEXT_TRANSMIT: c_int = 0;
pub const FW_ISO_CONTEXT_RECEIVE: c_int = 1;
pub const FW_ISO_CONTEXT_RECEIVE_MULTICHANNEL: c_int = 2;
pub const FW_ISO_CONTEXT_MATCH_TAG0: c_int = 1;
pub const FW_ISO_CONTEXT_MATCH_TAG1: c_int = 2;
pub const FW_ISO_CONTEXT_MATCH_TAG2: c_int = 4;
pub const FW_ISO_CONTEXT_MATCH_TAG3: c_int = 8;
pub const FW_ISO_CONTEXT_MATCH_ALL_TAGS: c_int = 15;
//
// An iso buffer is just a set of pages mapped for DMA in the
// specified direction.  Since the pages are to be used for DMA, they
// are not mapped into the kernel virtual address space.  We store the
// DMA address in the page private. The helper function
// fw_iso_buffer_map() will map the pages into a given vma.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_iso_buffer {
    pub direction: dma_data_direction,
    pub pages: *mut page,
    pub dma_addrs: *mut dma_addr_t,
    pub page_count: c_int,
}

extern "C" {
    pub fn fw_iso_buffer_destroy(buffer: *mut fw_iso_buffer, card: *mut fw_card);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union fw_iso_callback {
    pub sc: fw_iso_callback_t,
    pub mc: fw_iso_mc_callback_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fw_iso_context_flag {
    FW_ISO_CONTEXT_FLAG_DROP_OVERFLOW_HEADERS = BIT(0),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_iso_context {
    pub card: *mut fw_card,
    pub work: work_struct,
    pub type: c_int,
    pub channel: c_int,
    pub speed: c_int,
    pub flags: c_int,
    pub header_size: usize,
    pub header_storage_size: usize,
    pub callback: fw_iso_callback,
    pub callback_data: *mut c_void,
}

extern "C" {
    pub fn fw_iso_context_set_channels(ctx: *mut fw_iso_context, channels: *mut u64) -> c_int;
}
extern "C" {
    pub fn fw_iso_context_queue_flush(ctx: *mut fw_iso_context);
}
extern "C" {
    pub fn fw_iso_context_flush_completions(ctx: *mut fw_iso_context) -> c_int;
}
//
// fw_iso_context_schedule_flush_completions() - schedule work item to process isochronous context.
// @ctx: the isochronous context
//
// Schedule a work item on workqueue to process the isochronous context. The registered callback
// function is called by the worker when a queued packet buffer with the interrupt flag is
// completed, either after transmission in the IT context or after being filled in the IR context.
// The callback function is also called when the header buffer in the context becomes full, If it
// is required to process the context in the current context, fw_iso_context_flush_completions() is
// available instead.
//
// Context: Any context.
//
extern "C" {
    pub fn fw_iso_context_stop(ctx: *mut fw_iso_context) -> c_int;
}
extern "C" {
    pub fn fw_iso_context_destroy(ctx: *mut fw_iso_context);
}
