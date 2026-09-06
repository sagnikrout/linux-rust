//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/libeth/rx.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2024-2025 Intel Corporation

// Rx buffer management
// Space reserved in front of each frame

// Maximum headroom for worst-case calculations

// Link layer / L2 overhead: Ethernet, 2 VLAN tags (C + S), FCS

// Maximum supported L2-L4 header length

// Always use order-0 pages
pub const LIBETH_RX_PAGE_ORDER: c_int = 0;
// Pick a sane buffer stride and align to a cacheline boundary

// HW-writeable space in one buffer: truesize - headroom/tailroom, aligned

//
// struct libeth_fqe - structure representing an Rx buffer (fill queue element)
// @netmem: network memory reference holding the buffer
// @offset: offset from the page start (to the headroom)
// @truesize: total space occupied by the buffer (w/ headroom and tailroom)
//
// Depending on the MTU, API switches between one-page-per-frame and shared
// page model (to conserve memory on bigger-page platforms). In case of the
// former, @offset is always 0 and @truesize is always ```PAGE_SIZE```.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_fqe {
    pub netmem: netmem_ref,
    pub offset: u32,
    pub truesize: u32,
    pub __aligned_largest: },
//
// enum libeth_fqe_type - enum representing types of Rx buffers
// @LIBETH_FQE_MTU: buffer size is determined by MTU
// @LIBETH_FQE_SHORT: buffer size is smaller than MTU, for short frames
// @LIBETH_FQE_HDR: buffer size is ```LIBETH_MAX_HEAD```-sized, for headers
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum libeth_fqe_type {
    LIBETH_FQE_MTU		= 0U,
    LIBETH_FQE_SHORT,
    LIBETH_FQE_HDR,
}

//
// struct libeth_fq - structure representing a buffer (fill) queue
// @fp: hotpath part of the structure
// @pp: &page_pool for buffer management
// @fqes: array of Rx buffers
// @truesize: size to allocate per buffer, w/overhead
// @count: number of descriptors/buffers the queue has
// @type: type of the buffers this queue has
// @hsplit: flag whether header split is enabled
// @xdp: flag indicating whether XDP is enabled
// @no_napi: the queue is not a data queue and does not have NAPI
// @buf_len: HW-writeable length per each buffer
// @nid: ID of the closest NUMA node with memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_fq {
    pub pp: *mut page_pool,
    pub fqes: *mut libeth_fqe,
    pub truesize: u32,
    pub count: u32,
// Cold fields
    pub type:2: libeth_fqe_type,
    pub hsplit:1: bool,
    pub xdp:1: bool,
    pub no_napi:1: bool,
    pub buf_len: u32,
    pub nid: c_int,
}

extern "C" {
    pub fn libeth_rx_fq_create(fq: *mut libeth_fq, napi_dev: *mut c_void) -> c_int;
}
extern "C" {
    pub fn libeth_rx_fq_destroy(fq: *mut libeth_fq);
}
//
// libeth_rx_alloc - allocate a new Rx buffer
// @fq: fill queue to allocate for
// @i: index of the buffer within the queue
//
// Return: DMA address to be passed to HW for Rx on successful allocation,
// ```DMA_MAPPING_ERROR``` otherwise.
//
extern "C" {
    pub fn libeth_rx_recycle_slow(netmem: netmem_ref);
}
//
// libeth_rx_sync_for_cpu - synchronize or recycle buffer post DMA
// @fqe: buffer to process
// @len: frame length from the descriptor
//
// Process the buffer after it's written by HW. The regular path is to
// synchronize DMA for CPU, but in case of no data it will be immediately
// recycled back to its PP.
//
// Return: true when there's data to process, false otherwise.
//
// Very rare, but possible case. The most common reason:
// the last fragment contained FCS only, which was then
// stripped by the HW.
//
// Converting abstract packet type numbers into a software structure with
// the packet parameters to do O(1) lookup on Rx.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_rx_pt {
    pub outer_ip:2: u32,
    pub outer_frag:1: u32,
    pub tunnel_type:3: u32,
    pub tunnel_end_prot:2: u32,
    pub tunnel_end_frag:1: u32,
    pub inner_prot:3: u32,
    pub payload_layer:2: pkt_hash_types,
    pub pad:2: u32,
    pub hash_type:16: xdp_rss_hash_type,
}

//
// struct libeth_rx_csum - checksum offload bits decoded from the Rx descriptor
// @l3l4p: detectable L3 and L4 integrity check is processed by the hardware
// @ipe: IP checksum error
// @eipe: external (outermost) IP header (only for tunels)
// @eudpe: external (outermost) UDP checksum error (only for tunels)
// @ipv6exadd: IPv6 header with extension headers
// @l4e: L4 integrity error
// @pprs: set for packets that skip checksum calculation in the HW pre parser
// @nat: the packet is a UDP tunneled packet
// @raw_csum_valid: set if raw checksum is valid
// @pad: padding to naturally align raw_csum field
// @raw_csum: raw checksum
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_rx_csum {
    pub l3l4p:1: u32,
    pub ipe:1: u32,
    pub eipe:1: u32,
    pub eudpe:1: u32,
    pub ipv6exadd:1: u32,
    pub l4e:1: u32,
    pub pprs:1: u32,
    pub nat:1: u32,
    pub raw_csum_valid:1: u32,
    pub pad:7: u32,
    pub raw_csum:16: u32,
}

//
// struct libeth_rqe_info - receive queue element info
// @len: packet length
// @ptype: packet type based on types programmed into the device
// @eop: whether it's the last fragment of the packet
// @rxe: MAC errors: CRC, Alignment, Oversize, Undersizes, Length error
// @vlan: C-VLAN or S-VLAN tag depending on the VLAN offload configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libeth_rqe_info {
    pub len: u32,
    pub ptype:14: u32,
    pub eop:1: u32,
    pub rxe:1: u32,
    pub vlan:16: u32,
}

extern "C" {
    pub fn libeth_rx_pt_gen_hash_type(pt: *mut libeth_rx_pt);
}
//
// libeth_rx_pt_get_ip_ver - get IP version from a packet type structure
// @pt: packet type params
//
// Wrapper to compile out the IPv6 code from the drivers when not supported
// by the kernel.
//
// Return: @pt.outer_ip or stub for IPv6 when not compiled-in.
//

// libeth_has_*() can be used to quickly check whether the HW metadata is
// available to avoid further expensive processing such as descriptor reads.
// They already check for the corresponding netdev feature to be enabled,
// thus can be used as drop-in replacements.
//
// Non-zero _INNER* is only possible when _OUTER_IPV* is set,
// it is enough to check only for the L4 type.
//
// libeth_rx_pt_set_hash - fill in skb hash value basing on the PT
// @skb: skb to fill the hash in
// @hash: 32-bit hash value from the descriptor
// @pt: packet type
//
