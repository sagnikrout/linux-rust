//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_packet.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_pkt {
    pub spkt_family: c_ushort,
    pub spkt_device: [c_uchar; 14],
    pub spkt_protocol: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_ll {
    pub sll_family: c_ushort,
    pub sll_protocol: __be16,
    pub sll_ifindex: c_int,
    pub sll_hatype: c_ushort,
    pub sll_pkttype: c_uchar,
    pub sll_halen: c_uchar,
    pub sll_addr: [c_uchar; 8],
}

// Packet types

// Unused, PACKET_FASTROUTE and PACKET_LOOPBACK are invisible to user space

// Packet socket options
pub const PACKET_ADD_MEMBERSHIP: c_int = 1;
pub const PACKET_DROP_MEMBERSHIP: c_int = 2;
pub const PACKET_RECV_OUTPUT: c_int = 3;
// Value 4 is still used by obsolete turbo-packet.
pub const PACKET_RX_RING: c_int = 5;
pub const PACKET_STATISTICS: c_int = 6;
pub const PACKET_COPY_THRESH: c_int = 7;
pub const PACKET_AUXDATA: c_int = 8;
pub const PACKET_ORIGDEV: c_int = 9;
pub const PACKET_VERSION: c_int = 10;
pub const PACKET_HDRLEN: c_int = 11;
pub const PACKET_RESERVE: c_int = 12;
pub const PACKET_TX_RING: c_int = 13;
pub const PACKET_LOSS: c_int = 14;
pub const PACKET_VNET_HDR: c_int = 15;
pub const PACKET_TX_TIMESTAMP: c_int = 16;
pub const PACKET_TIMESTAMP: c_int = 17;
pub const PACKET_FANOUT: c_int = 18;
pub const PACKET_TX_HAS_OFF: c_int = 19;
pub const PACKET_QDISC_BYPASS: c_int = 20;
pub const PACKET_ROLLOVER_STATS: c_int = 21;
pub const PACKET_FANOUT_DATA: c_int = 22;
pub const PACKET_IGNORE_OUTGOING: c_int = 23;
pub const PACKET_VNET_HDR_SZ: c_int = 24;
pub const PACKET_FANOUT_HASH: c_int = 0;
pub const PACKET_FANOUT_LB: c_int = 1;
pub const PACKET_FANOUT_CPU: c_int = 2;
pub const PACKET_FANOUT_ROLLOVER: c_int = 3;
pub const PACKET_FANOUT_RND: c_int = 4;
pub const PACKET_FANOUT_QM: c_int = 5;
pub const PACKET_FANOUT_CBPF: c_int = 6;
pub const PACKET_FANOUT_EBPF: c_int = 7;
pub const PACKET_FANOUT_FLAG_ROLLOVER: c_uint = 0x1000;
pub const PACKET_FANOUT_FLAG_UNIQUEID: c_uint = 0x2000;
pub const PACKET_FANOUT_FLAG_IGNORE_OUTGOING: c_uint = 0x4000;
pub const PACKET_FANOUT_FLAG_DEFRAG: c_uint = 0x8000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_stats {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_stats_v3 {
    pub tp_packets: c_uint,
    pub tp_drops: c_uint,
    pub tp_freeze_q_cnt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_rollover_stats {
    pub tp_all: __aligned_u64,
    pub tp_huge: __aligned_u64,
    pub tp_failed: __aligned_u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_stats_u {
    pub stats1: tpacket_stats,
    pub stats3: tpacket_stats_v3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_auxdata {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u16,
}

// Rx ring - header status
pub const TP_STATUS_KERNEL: c_int = 0;

// Tx ring - header status
pub const TP_STATUS_AVAILABLE: c_int = 0;

// Rx and Tx ring - header status

// Rx ring - feature request bits
pub const TP_FT_REQ_FILL_RXHASH: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_hdr {
    pub tp_status: c_ulong,
    pub tp_len: c_uint,
    pub tp_snaplen: c_uint,
    pub tp_mac: c_ushort,
    pub tp_net: c_ushort,
    pub tp_sec: c_uint,
    pub tp_usec: c_uint,
}

pub const TPACKET_ALIGNMENT: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket2_hdr {
    pub tp_status: __u32,
    pub tp_len: __u32,
    pub tp_snaplen: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_vlan_tci: __u16,
    pub tp_vlan_tpid: __u16,
    pub tp_padding: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_hdr_variant1 {
    pub tp_rxhash: __u32,
    pub tp_vlan_tci: __u32,
    pub tp_vlan_tpid: __u16,
    pub tp_padding: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket3_hdr {
    pub tp_next_offset: __u32,
    pub tp_sec: __u32,
    pub tp_nsec: __u32,
    pub tp_snaplen: __u32,
    pub tp_len: __u32,
    pub tp_status: __u32,
    pub tp_mac: __u16,
    pub tp_net: __u16,
// pkt_hdr variants
    pub hv1: tpacket_hdr_variant1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_bd_ts {
    pub ts_sec: c_uint,
    pub ts_usec: c_uint,
    pub ts_nsec: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_hdr_v1 {
    pub block_status: __u32,
    pub num_pkts: __u32,
    pub offset_to_first_pkt: __u32,
// Number of valid bytes (including padding)
// blk_len <= tp_block_size
//
    pub blk_len: __u32,
//
// Quite a few uses of sequence number:
// 1. Make sure cache flush etc worked.
// Well, one can argue - why not use the increasing ts below?
// But look at 2. below first.
// 2. When you pass around blocks to other user space decoders,
// you can see which blk[s] is[are] outstanding etc.
// 3. Validate kernel code.
//
    pub seq_num: __aligned_u64,
//
// ts_last_pkt:
//
// Case 1.	Block has 'N'(N >=1) packets and TMO'd(timed out)
// ts_last_pkt == 'time-stamp of last packet' and NOT the
// time when the timer fired and the block was closed.
// By providing the ts of the last packet we can absolutely
// guarantee that time-stamp wise, the first packet in the
// next block will never precede the last packet of the
// previous block.
// Case 2.	Block has zero packets and TMO'd
// ts_last_pkt = time when the timer fired and the block
// was closed.
// Case 3.	Block has 'N' packets and NO TMO.
// ts_last_pkt = time-stamp of the last pkt in the block.
//
// ts_first_pkt:
// Is always the time-stamp when the block was opened.
// Case a)	ZERO packets
// No packets to deal with but at least you know
// the time-interval of this block.
// Case b) Non-zero packets
// Use the ts of the first packet in the block.
//
    pub ts_last_pkt: tpacket_bd_ts ts_first_pkt,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_bd_header_u {
    pub bh1: tpacket_hdr_v1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_block_desc {
    pub version: __u32,
    pub offset_to_priv: __u32,
    pub hdr: tpacket_bd_header_u,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tpacket_versions {
    TPACKET_V1,
    TPACKET_V2,
    TPACKET_V3
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_req {
    pub /: *mut *mut unsigned int tp_block_size; / Minimal size of contiguous block,
    pub /: *mut *mut unsigned int tp_block_nr; / Number of blocks,
    pub /: *mut *mut unsigned int tp_frame_size; / Size of frame,
    pub /: *mut *mut unsigned int tp_frame_nr; / Total number of frames,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tpacket_req3 {
    pub /: *mut *mut unsigned int tp_block_size; / Minimal size of contiguous block,
    pub /: *mut *mut unsigned int tp_block_nr; / Number of blocks,
    pub /: *mut *mut unsigned int tp_frame_size; / Size of frame,
    pub /: *mut *mut unsigned int tp_frame_nr; / Total number of frames,
    pub /: *mut *mut unsigned int tp_retire_blk_tov; / timeout in msecs,
    pub /: *mut *mut unsigned int tp_sizeof_priv; / offset to private data area,
    pub tp_feature_req_word: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union tpacket_req_u {
    pub req: tpacket_req,
    pub req3: tpacket_req3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct packet_mreq {
    pub mr_ifindex: c_int,
    pub mr_type: c_ushort,
    pub mr_alen: c_ushort,
    pub mr_address: [c_uchar; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fanout_args {

    pub id: __u16,
    pub type_flags: __u16,

    pub type_flags: __u16,
    pub id: __u16,

    pub max_num_members: __u32,
}

pub const PACKET_MR_MULTICAST: c_int = 0;
pub const PACKET_MR_PROMISC: c_int = 1;
pub const PACKET_MR_ALLMULTI: c_int = 2;
pub const PACKET_MR_UNICAST: c_int = 3;
