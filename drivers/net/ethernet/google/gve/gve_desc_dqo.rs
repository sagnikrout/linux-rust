//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/google/gve/gve_desc_dqo.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2015-2021 Google, Inc.
//
// GVE DQO Descriptor formats

pub const GVE_TX_MAX_HDR_SIZE_DQO: c_int = 255;
pub const GVE_TX_MIN_TSO_MSS_DQO: c_int = 88;

// Basic TX descriptor (DTYPE 0x0C)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_pkt_desc_dqo {
    pub buf_addr: __le64,
// Must be GVE_TX_PKT_DESC_DTYPE_DQO (0xc)
    pub 5: u8 dtype:,
// Denotes the last descriptor of a packet.
    pub 1: u8 end_of_packet:,
    pub 1: u8 checksum_offload_enable:,
// If set, will generate a descriptor completion for this descriptor.
    pub 1: u8 report_event:,
    pub reserved0: u8,
    pub reserved1: __le16,
// The TX completion associated with this packet will contain this tag.
//
    pub compl_tag: __le16,
    pub 14: u16 buf_size:,
    pub 2: u16 reserved2:,
    pub __packed: },
    pub 16): static_assert(sizeof(struct gve_tx_pkt_desc_dqo) ==,
pub const GVE_TX_PKT_DESC_DTYPE_DQO: c_uint = 0xc;

// Maximum number of data descriptors allowed per packet, or per-TSO segment.
pub const GVE_TX_MAX_DATA_DESCS: c_int = 10;
// Min gap between tail and head to avoid cacheline overlap
pub const GVE_TX_MIN_DESC_PREVENT_CACHE_OVERLAP: c_int = 4;
// "report_event" on TX packet descriptors may only be reported on the last
// descriptor of a TX packet, and they must be spaced apart with at least this
// value.
//
pub const GVE_TX_MIN_RE_INTERVAL: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_context_cmd_dtype {
    pub 5: u8 dtype:,
    pub 1: u8 tso:,
    pub 2: u8 reserved1:,
    pub reserved2: u8,
}

// TX Native TSO Context DTYPE (0x05)
//
// "flex" fields allow the driver to send additional packet context to HW.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_tso_context_desc_dqo {
// The L4 payload bytes that should be segmented.
    pub 24: u32 tso_total_len:,
    pub 8: u32 flex10:,
// Max segment size in TSO excluding headers.
    pub 14: u16 mss:,
    pub 2: u16 reserved:,
    pub /: *mut *mut u8 header_len; / Header length to use for TSO offload,
    pub flex11: u8,
    pub cmd_dtype: gve_tx_context_cmd_dtype,
    pub flex0: u8,
    pub flex5: u8,
    pub flex6: u8,
    pub flex7: u8,
    pub flex8: u8,
    pub flex9: u8,
    pub __packed: },
    pub 16): static_assert(sizeof(struct gve_tx_tso_context_desc_dqo) ==,
pub const GVE_TX_TSO_CTX_DESC_DTYPE_DQO: c_uint = 0x5;
// General context descriptor for sending metadata.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_general_context_desc_dqo {
    pub flex4: u8,
    pub flex5: u8,
    pub flex6: u8,
    pub flex7: u8,
    pub flex8: u8,
    pub flex9: u8,
    pub flex10: u8,
    pub flex11: u8,
    pub cmd_dtype: gve_tx_context_cmd_dtype,
    pub reserved: u16,
    pub flex0: u8,
    pub flex1: u8,
    pub flex2: u8,
    pub flex3: u8,
    pub __packed: },
    pub 16): static_assert(sizeof(struct gve_tx_general_context_desc_dqo) ==,
pub const GVE_TX_GENERAL_CTX_DESC_DTYPE_DQO: c_uint = 0x4;
// Logical structure of metadata which is packed into context descriptor flex
// fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_metadata_dqo {
    pub version: u8,
// If `skb->l4_hash` is set, this value should be
// derived from `skb->hash`.
//
// A zero value means no l4_hash was associated with the
// skb.
//
    pub 15: u16 path_hash:,
// Should be set to 1 if the flow associated with the
// skb had a rehash from the TCP stack.
//
    pub 1: u16 rehash_event:,
    pub __packed: },
    pub bytes: [u8; 12],
}

pub const GVE_TX_METADATA_VERSION_DQO: c_int = 0;
// TX completion descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_tx_compl_desc {
// For types 0-4 this is the TX queue ID associated with this
// completion.
//
    pub 11: u16 id:,
// See: GVE_COMPL_TYPE_DQO*
    pub 3: u16 type:,
    pub 1: u16 reserved0:,
// Flipped by HW to notify the descriptor is populated.
    pub 1: u16 generation:,
// For descriptor completions, this is the last index fetched
// by HW + 1.
//
    pub tx_head: __le16,
// For packet completions, this is the completion tag set on the
// TX packet descriptors.
//
    pub completion_tag: __le16,
}

pub const GVE_COMPL_TYPE_DQO_PKT: c_uint = 0x2 /* Packet completion */;
pub const GVE_COMPL_TYPE_DQO_DESC: c_uint = 0x4 /* Descriptor completion */;
pub const GVE_COMPL_TYPE_DQO_MISS: c_uint = 0x1 /* Miss path completion */;
pub const GVE_COMPL_TYPE_DQO_REINJECTION: c_uint = 0x3 /* Re-injection completion */;
// The most significant bit in the completion tag can change the completion
// type from packet completion to miss path completion.
//

// Descriptor to post buffers to HW on buffer queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_desc_dqo {
    pub /: *mut *mut __le16 buf_id; / ID returned in Rx completion descriptor,
    pub reserved0: __le16,
    pub reserved1: __le32,
    pub /: *mut *mut __le64 buf_addr; / DMA address of the buffer,
    pub header_buf_addr: __le64,
    pub reserved2: __le64,
    pub __packed: },
    pub 32): static_assert(sizeof(struct gve_rx_desc_dqo) ==,
// Descriptor for HW to notify SW of new packets received on RX queue.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gve_rx_compl_desc_dqo {
// Must be 1
    pub 4: u8 rxdid:,
    pub 4: u8 reserved0:,
// Packet originated from this system rather than the network.
    pub 1: u8 loopback:,
// Set when IPv6 packet contains a destination options header or routing
// header.
//
    pub 1: u8 ipv6_ex_add:,
// Invalid packet was received.
    pub 1: u8 rx_error:,
    pub 5: u8 reserved1:,
    pub 10: u16 packet_type:,
    pub 1: u16 ip_hdr_err:,
    pub 1: u16 udp_len_err:,
    pub 1: u16 raw_cs_invalid:,
    pub 3: u16 reserved2:,
    pub 14: u16 packet_len:,
// Flipped by HW to notify the descriptor is populated.
    pub 1: u16 generation:,
// Should be zero.
    pub 1: u16 buffer_queue_id:,
    pub 10: u16 header_len:,
    pub 1: u16 rsc:,
    pub 1: u16 split_header:,
    pub 4: u16 reserved3:,
    pub 1: u8 descriptor_done:,
    pub 1: u8 end_of_packet:,
    pub 1: u8 header_buffer_overflow:,
    pub 1: u8 l3_l4_processed:,
    pub 1: u8 csum_ip_err:,
    pub 1: u8 csum_l4_err:,
    pub 1: u8 csum_external_ip_err:,
    pub 1: u8 csum_external_udp_err:,
    pub status_error1: u8,
    pub reserved5: u8,
    pub ts_sub_nsecs_low: u8,
    pub /: *mut *mut __le16 buf_id; / Buffer ID which was sent on the buffer queue.,
// Packet checksum.
    pub raw_cs: __le16,
// Segment length for RSC packets.
    pub rsc_seg_len: __le16,
}

// Ringing the doorbell too often can hurt performance.
//
// HW requires this value to be at least 8.
//
pub const GVE_RX_BUF_THRESH_DQO: c_int = 32;
