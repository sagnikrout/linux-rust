//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/if_xdp.h
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
//
// if_xdp: XDP socket user-space interface
// Copyright(c) 2018 Intel Corporation.
//
// Author(s): Björn Töpel <bjorn.topel@intel.com>
// Magnus Karlsson <magnus.karlsson@intel.com>
//

// Options for the sxdp_flags field

// If this option is set, the driver might go sleep and in that case
// the XDP_RING_NEED_WAKEUP flag in the fill and/or Tx rings will be
// set. If it is set, the application need to explicitly wake up the
// driver with a poll() (Rx and Tx) or sendto() (Tx only). If you are
// running the driver and the application on the same core, you should
// use this option so that the kernel will yield to the user space
// application.
//

// By setting this option, userspace application indicates that it can
// handle multiple descriptors per packet thus enabling AF_XDP to split
// multi-buffer XDP frames into multiple Rx descriptors. Without this set
// such frames will be dropped.
//

// Flags for xsk_umem_config flags

// Force checksum calculation in software. Can be used for testing or
// working around potential HW issues. This option causes performance
// degradation and only works in XDP_COPY mode.
//

// Request to reserve tx_metadata_len bytes of per-chunk metadata.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_xdp {
    pub sxdp_family: __u16,
    pub sxdp_flags: __u16,
    pub sxdp_ifindex: __u32,
    pub sxdp_queue_id: __u32,
    pub sxdp_shared_umem_fd: __u32,
}

// XDP_RING flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_ring_offset {
    pub producer: __u64,
    pub consumer: __u64,
    pub desc: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_mmap_offsets {
    pub rx: xdp_ring_offset,
    pub tx: xdp_ring_offset,
    pub /: *mut *mut xdp_ring_offset fr; / Fill,
    pub /: *mut *mut xdp_ring_offset cr; / Completion,
}

// XDP socket options
pub const XDP_MMAP_OFFSETS: c_int = 1;
pub const XDP_RX_RING: c_int = 2;
pub const XDP_TX_RING: c_int = 3;
pub const XDP_UMEM_REG: c_int = 4;
pub const XDP_UMEM_FILL_RING: c_int = 5;
pub const XDP_UMEM_COMPLETION_RING: c_int = 6;
pub const XDP_STATISTICS: c_int = 7;
pub const XDP_OPTIONS: c_int = 8;
pub const XDP_MAX_TX_SKB_BUDGET: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_umem_reg {
    pub /: *mut *mut __u64 addr; / Start of packet data area,
    pub /: *mut *mut __u64 len; / Length of packet data area,
    pub chunk_size: __u32,
    pub headroom: __u32,
    pub flags: __u32,
    pub tx_metadata_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_statistics {
    pub /: *mut *mut __u64 rx_dropped; / Dropped for other reasons,
    pub /: *mut *mut __u64 rx_invalid_descs; / Dropped due to invalid descriptor,
    pub /: *mut *mut __u64 tx_invalid_descs; / Dropped due to invalid descriptor,
    pub /: *mut *mut __u64 rx_ring_full; / Dropped due to rx ring being full,
    pub /: *mut *mut __u64 rx_fill_ring_empty_descs; / Failed to retrieve item from fill ring,
    pub /: *mut *mut __u64 tx_ring_empty_descs; / Failed to retrieve item from tx ring,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_options {
    pub flags: __u32,
}

// Flags for the flags field of struct xdp_options

// Pgoff for mmaping the rings
pub const XDP_PGOFF_RX_RING: c_int = 0;
pub const XDP_PGOFF_TX_RING: c_uint = 0x80000000;
pub const XDP_UMEM_PGOFF_FILL_RING: c_uint = 0x100000000ULL;
pub const XDP_UMEM_PGOFF_COMPLETION_RING: c_uint = 0x180000000ULL;
// Masks for unaligned chunks mode
pub const XSK_UNALIGNED_BUF_OFFSET_SHIFT: c_int = 48;

// Request transmit timestamp. Upon completion, put it into tx_timestamp
// field of struct xsk_tx_metadata.
//

// Request transmit checksum offload. Checksum start position and offset
// are communicated via csum_start and csum_offset fields of struct
// xsk_tx_metadata.
//

// Request launch time hardware offload. The device will schedule the packet for
// transmission at a pre-determined time called launch time. The value of
// launch time is communicated via launch_time field of struct xsk_tx_metadata.
//

// AF_XDP offloads request. 'request' union member is consumed by the driver
// when the packet is being transmitted. 'completion' union member is
// filled by the driver when the transmit completion arrives.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_tx_metadata {
    pub flags: __u64,
// XDP_TXMD_FLAGS_CHECKSUM
// Offset from desc->addr where checksumming should start.
    pub csum_start: __u16,
// Offset from csum_start where checksum should be stored.
    pub csum_offset: __u16,
    pub reserved: __u32,
// XDP_TXMD_FLAGS_LAUNCH_TIME
// Launch time in nanosecond against the PTP HW Clock
    pub launch_time: __u64,
    pub request: },
// XDP_TXMD_FLAGS_TIMESTAMP
    pub tx_timestamp: __u64,
    pub completion: },
}

// Rx/Tx descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_desc {
    pub addr: __u64,
    pub len: __u32,
    pub options: __u32,
}

// UMEM descriptor is __u64
// Flag indicating that the packet continues with the buffer pointed out by the
// next frame in the ring. The end of the packet is signalled by setting this
// bit to zero. For single buffer packets, every descriptor has 'options' set
// to 0 and this maintains backward compatibility.
//

// TX packet carries valid metadata.

