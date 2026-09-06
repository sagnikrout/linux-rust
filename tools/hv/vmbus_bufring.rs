//! Automatically rewritten from C Header to Rust Module
//! Source: tools/hv/vmbus_bufring.h
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


// SPDX-License-Identifier: BSD-3-Clause

pub const ICMSGHDRFLAG_TRANSACTION: c_int = 1;
pub const ICMSGHDRFLAG_REQUEST: c_int = 2;
pub const ICMSGHDRFLAG_RESPONSE: c_int = 4;
pub const IC_VERSION_NEGOTIATION_MAX_VER_COUNT: c_int = 100;

//
// Channel packets
//
// Channel packet flags
pub const VMBUS_CHANPKT_TYPE_INBAND: c_uint = 0x0006;
pub const VMBUS_CHANPKT_TYPE_RXBUF: c_uint = 0x0007;
pub const VMBUS_CHANPKT_TYPE_GPA: c_uint = 0x0009;
pub const VMBUS_CHANPKT_TYPE_COMP: c_uint = 0x000b;
pub const VMBUS_CHANPKT_FLAG_NONE: c_int = 0;
pub const VMBUS_CHANPKT_FLAG_RC: c_uint = 0x0001  /* report completion */;
pub const VMBUS_CHANPKT_SIZE_SHIFT: c_int = 3;

//
// Buffer ring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_bufring {
    pub windex: volatile uint32_t,
    pub rindex: volatile uint32_t,
//
// Interrupt mask {0,1}
//
// For TX bufring, host set this to 1, when it is processing
// the TX bufring, so that we can safely skip the TX event
// notification to host.
//
// For RX bufring, once this is set to 1 by us, host will not
// further dispatch interrupts to us, even if there are data
// pending on the RX bufring.  This effectively disables the
// interrupt of the channel to which this RX bufring is attached.
//
    pub imask: volatile uint32_t,
//
// Win8 uses some of the reserved bits to implement
// interrupt driven flow management. On the send side
// we can request that the receiver interrupt the sender
// when the ring transitions from being full to being able
// to handle a message of size "pending_send_sz".
//
// Add necessary state for this enhancement.
//
    pub pending_send: volatile uint32_t,
    pub reserved1: [u32; 12],
    pub feat_pending_send_sz:1: u32,
}

// Pad it to rte_mem_page_size() so that data starts on page boundary
//
// Ring data starts here + RingDataStartOffset
// !!! DO NOT place any fields below this !!!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_br {
    pub vbr: *mut vmbus_bufring,
    pub dsize: u32,
    pub /: *mut *mut uint32_t windex; / next available location,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_chanpkt_hdr {
    pub /: *mut *mut uint16_t type; / VMBUS_CHANPKT_TYPE_,
    pub /: *mut *mut uint16_t hlen; / header len, in 8 bytes,
    pub /: *mut *mut uint16_t tlen; / total len, in 8 bytes,
    pub /: *mut *mut uint16_t flags; / VMBUS_CHANPKT_FLAG_,
    pub xactid: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_chanpkt {
    pub hdr: vmbus_chanpkt_hdr,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbuspipe_hdr {
    pub flags: c_uint,
    pub msgsize: c_uint,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ic_version {
    pub major: c_ushort,
    pub minor: c_ushort,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmsg_negotiate {
    pub icframe_vercnt: c_ushort,
    pub icmsg_vercnt: c_ushort,
    pub reserved: c_uint,
    pub /: *mut *mut ic_version icversion_data[]; / any size array,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmsg_hdr {
    pub icverframe: ic_version,
    pub icmsgtype: c_ushort,
    pub icvermsg: ic_version,
    pub icmsgsize: c_ushort,
    pub status: c_uint,
    pub ictransaction_id: c_uchar,
    pub icflags: c_uchar,
    pub reserved: [c_uchar; 2],
    pub __packed: },
    pub len): *mut *mut *mut int rte_vmbus_chan_recv_raw(struct vmbus_br rxbr, void data, uint32_t,
    pub flags): uint32_t dlen, uint32_t,
    pub blen): *mut *mut *mut void vmbus_br_setup(struct vmbus_br br, void buf, unsigned int,
    pub size): *mut *mut *mut void vmbus_uio_map(int fd, int,
// Amount of space available for write
    pub br->vbr->rindex: uint32_t rindex =,
    pub rindex): return br->dsize - (windex -,
    pub windex: return rindex -,
    pub br->vbr->windex): return br->dsize - vmbus_br_availwrite(br,,
