//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firewire/core.h
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

// -card
// This is the arbitrary value we use to indicate a mismatched gap count.
pub const GAP_COUNT_MISMATCHED: c_int = 0;

extern "C" {
    pub fn fw_err(card: *const fw_card, fmt: *const c_char, ...);
}
extern "C" {
    pub fn fw_notice(card: *const fw_card, fmt: *const c_char, ...);
}
// bitfields within the PHY registers
pub const PHY_LINK_ACTIVE: c_uint = 0x80;
pub const PHY_CONTENDER: c_uint = 0x40;
pub const PHY_BUS_RESET: c_uint = 0x40;
pub const PHY_EXTENDED_REGISTERS: c_uint = 0xe0;
pub const PHY_BUS_SHORT_RESET: c_uint = 0x40;
pub const PHY_INT_STATUS_BITS: c_uint = 0x3c;
pub const PHY_ENABLE_ACCEL: c_uint = 0x02;
pub const PHY_ENABLE_MULTI: c_uint = 0x01;
pub const PHY_PAGE_SELECT: c_uint = 0xe0;
pub const BANDWIDTH_AVAILABLE_INITIAL: c_int = 4915;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_card_driver {
//
// Enable the given card with the given initial config rom.
// This function is expected to activate the card, and either
// enable the PHY or set the link_on bit and initiate a bus
// reset.
//
    pub length): *const *const __be32 config_rom, size_t,
// After returning the call, any function is no longer triggered to handle hardware event.
    pub card): *mut *mut void (disable)(struct fw_card,
    pub address): *mut *mut *mut int (read_phy_reg)(struct fw_card card, int,
    pub set_bits): int clear_bits, int,
//
// Update the config rom for an enabled card.  This function
// should change the config rom that is presented on the bus
// and initiate a bus reset.
//
    pub length): *const *const __be32 config_rom, size_t,
    pub packet): *mut *mut *mut void (send_request)(struct fw_card card, struct fw_packet,
    pub packet): *mut *mut *mut void (send_response)(struct fw_card card, struct fw_packet,
// Calling cancel is valid once a packet has been submitted.
    pub packet): *mut *mut *mut int (cancel_packet)(struct fw_card card, struct fw_packet,
//
// Allow the specified node ID to do direct DMA out and in of
// host memory.  The card will disable this for all node when
// a bus reset happens, so driver need to re-enable this after
// bus reset.  Returns 0 on success, -ENODEV if the card
// doesn't support this, -ESTALE if the generation doesn't
// match.
//
    pub generation): int node_id, int,
    pub csr_offset): *mut *mut *mut u32 (read_csr)(struct fw_card card, int,
    pub value): *mut *mut *mut void (write_csr)(struct fw_card card, int csr_offset, u32,
    pub header_storage_size): usize,
    pub ctx): *mut *mut void (free_iso_context)(struct fw_iso_context,
    pub tags): s32 cycle, u32 sync, u32,
    pub channels): *mut *mut *mut int (set_iso_channels)(struct fw_iso_context ctx, u64,
    pub payload): c_ulong,
    pub ctx): *mut *mut void (flush_queue_iso)(struct fw_iso_context,
    pub ctx): *mut *mut int (flush_iso_completions)(struct fw_iso_context,
    pub ctx): *mut *mut int (stop_iso)(struct fw_iso_context,
}

extern "C" {
    pub fn fw_core_remove_card(card: *mut fw_card);
}
extern "C" {
    pub fn fw_compute_block_crc(block: *mut __be32) -> c_int;
}
extern "C" {
    pub fn fw_schedule_bm_work(card: *mut fw_card, delay: c_ulong);
}
// -cdev
extern "C" {
    pub fn fw_device_cdev_update(device: *mut fw_device);
}
extern "C" {
    pub fn fw_device_cdev_remove(device: *mut fw_device);
}
extern "C" {
    pub fn fw_cdev_handle_phy_packet(card: *mut fw_card, p: *mut fw_packet);
}
// -device
extern "C" {
    pub fn fw_device_set_broadcast_channel(dev: *mut device, gen: *mut c_void) -> c_int;
}
extern "C" {
    pub fn fw_node_event(card: *mut fw_card, node: *mut fw_node, event: c_int);
}
// -iso
extern "C" {
    pub fn fw_iso_buffer_alloc(buffer: *mut fw_iso_buffer, page_count: c_int) -> c_int;
}
extern "C" {
    pub fn fw_iso_buffer_lookup(buffer: *mut fw_iso_buffer, completed: dma_addr_t) -> usize;
}
// -topology
// The initial value of BUS_MANAGER_ID register, to express nothing registered.
pub const BUS_MANAGER_ID_NOT_REGISTERED: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_node {
    pub node_id: u16,
    pub color: u8,
    pub port_count: u8,
    pub link_on:1: u8,
    pub initiated_reset:1: u8,
    pub b_path:1: u8,
    pub /: *mut *mut u8 phy_speed:2; / As in the self ID packet.,
    pub the: *mut *mut u8 max_speed:2; / Minimum of all phy-speeds on the path from,
// local node to this node.
    pub /: *mut *mut u8 max_depth:4; / Maximum depth to any leaf node,
    pub /: *mut *mut u8 max_hops:4; / Max hops in this sub tree,
    pub kref: kref,
// For serializing node topology into a list.
    pub link: list_head,
// The device when already associated, else NULL.
    pub device: *mut fw_device,
    pub __counted_by(port_count): *mut *mut fw_node ports[],
}

extern "C" {
    pub fn fw_destroy_nodes(card: *mut fw_card);
}
//
// Check whether new_generation is the immediate successor of old_generation.
// Take counter roll-over at 255 (as per OHCI) into account.
//
// -transaction
pub const TCODE_LINK_INTERNAL: c_uint = 0xe;
pub const LOCAL_BUS: c_uint = 0xffc0;
// OHCI-1394's default upper bound for physical DMA: 4 GB

extern "C" {
    pub fn fw_core_handle_request(card: *mut fw_card, request: *mut fw_packet);
}
extern "C" {
    pub fn fw_core_handle_response(card: *mut fw_card, packet: *mut fw_packet);
}
extern "C" {
    pub fn fw_get_response_length(request: *mut fw_request) -> c_int;
}
extern "C" {
    pub fn fw_request_get(request: *mut fw_request);
}
extern "C" {
    pub fn fw_request_put(request: *mut fw_request);
}
extern "C" {
    pub fn fw_cancel_pending_transactions(card: *mut fw_card);
}
// Convert the value of IEEE 1394 CYCLE_TIME register to the format of timeStamp field in
// descriptors of 1394 OHCI.

