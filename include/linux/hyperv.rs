//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hyperv.h
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
//
// Copyright (c) 2011, Microsoft Corporation.
//
// Authors:
// Haiyang Zhang <haiyangz@microsoft.com>
// Hank Janssen  <hjanssen@microsoft.com>
// K. Y. Srinivasan <kys@microsoft.com>
//

pub const MAX_PAGE_BUFFER_COUNT: c_int = 32;

//
// Types for GPADL, decides is how GPADL header is created.
//
// It doesn't make much difference between BUFFER and RING if PAGE_SIZE is the
// same as HV_HYP_PAGE_SIZE.
//
// If PAGE_SIZE is bigger than HV_HYP_PAGE_SIZE, the headers of ring buffers
// will be of PAGE_SIZE, however, only the first HV_HYP_PAGE will be put
// into gpadl, therefore the number for HV_HYP_PAGE and the indexes of each
// HV_HYP_PAGE will be different between different types of GPADL, for example
// if PAGE_SIZE is 64K:
//
// BUFFER:
//
// gva:    |--       64k      --|--       64k      --| ... |
// gpa:    | 4k | 4k | ... | 4k | 4k | 4k | ... | 4k |
// index:  0    1    2     15   16   17   18 .. 31   32 ...
// |    |    ...   |    |    |   ...    |   ...
// v    V          V    V    V          V
// gpadl:  | 4k | 4k | ... | 4k | 4k | 4k | ... | 4k | ... |
// index:  0    1    2 ... 15   16   17   18 .. 31   32 ...
//
// RING:
//
// | header  |           data           | header  |     data      |
// gva:    |-- 64k --|--       64k      --| ... |-- 64k --|-- 64k --| ... |
// gpa:    | 4k | .. | 4k | 4k | ... | 4k | ... | 4k | .. | 4k | .. | ... |
// index:  0    1    16   17   18    31   ...   n   n+1  n+16 ...         2n
// |         /    /          /          |         /
// |        /    /          /           |        /
// |       /    /   ...    /    ...     |       /      ...
// |      /    /          /             |      /
// |     /    /          /              |     /
// V    V    V          V               V    V               v
// gpadl:  | 4k | 4k |   ...    |    ...        | 4k | 4k |  ...     |
// index:  0    1    2   ...    16   ...       n-15 n-14 n-13  ...  2n-30
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_gpadl_type {
    HV_GPADL_BUFFER,
    HV_GPADL_RING,
    HV_GPADL_BUFFER_DECRYPTED
}

// Single-page buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_page_buffer {
    pub len: u32,
    pub offset: u32,
    pub pfn: u64,
}

// Multiple-page buffer
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_multipage_buffer {
// Length and Offset determines the # of pfns in the array
    pub len: u32,
    pub offset: u32,
    pub pfn_array: [u64; MAX_MULTIPAGE_BUFFER_COUNT],
}

//
// Multiple-page buffer array; the pfn array is variable size:
// The number of entries in the PFN array is determined by
// "len" and "offset".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_mpb_array {
// Length and Offset determines the # of pfns in the array
    pub len: u32,
    pub offset: u32,
    pub pfn_array: [u64; ],
}

// 0x18 includes the proprietary packet header

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_ring_buffer {
// Offset in bytes from the start of ring data below
    pub write_index: u32,
// Offset in bytes from the start of ring data below
    pub read_index: u32,
    pub interrupt_mask: u32,
//
// WS2012/Win8 and later versions of Hyper-V implement interrupt
// driven flow management. The feature bit feat_pending_send_sz
// is set by the host on the host->guest ring buffer, and by the
// guest on the guest->host ring buffer.
//
// The meaning of the feature bit is a bit complex in that it has
// semantics that apply to both ring buffers.  If the guest sets
// the feature bit in the guest->host ring buffer, the guest is
// telling the host that:
// 1) It will set the pending_send_sz field in the guest->host ring
// buffer when it is waiting for space to become available, and
// 2) It will read the pending_send_sz field in the host->guest
// ring buffer and interrupt the host when it frees enough space
//
// Similarly, if the host sets the feature bit in the host->guest
// ring buffer, the host is telling the guest that:
// 1) It will set the pending_send_sz field in the host->guest ring
// buffer when it is waiting for space to become available, and
// 2) It will read the pending_send_sz field in the guest->host
// ring buffer and interrupt the guest when it frees enough space
//
// If either the guest or host does not set the feature bit that it
// owns, that guest or host must do polling if it encounters a full
// ring buffer, and not signal the other end with an interrupt.
//
    pub pending_send_sz: u32,
    pub reserved1: [u32; 12],
    pub feat_pending_send_sz:1: u32,
}

// Pad it to PAGE_SIZE so that data starts on page boundary
//
// Ring data starts here + RingDataStartOffset
// !!! DO NOT place any fields below this !!!
//
// If the requested ring buffer size is at least 8 times the size of the
// header, steal space from the ring buffer for the header. Otherwise, add
// space for the header so that is doesn't take too much of the ring buffer
// space.
//
// The factor of 8 is somewhat arbitrary. The goal is to prevent adding a
// relatively small header (4 Kbytes on x86) to a large-ish power-of-2 ring
// buffer size (such as 128 Kbytes) and so end up making a nearly twice as
// large allocation that will be almost half wasted. As a contrasting example,
// on ARM64 with 64 Kbyte page size, we don't want to take 64 Kbytes for the
// header from a 128 Kbyte allocation, leaving only 64 Kbytes for the ring.
// In this latter case, we must add 64 Kbytes for the header and not worry
// about what's wasted.
//

// Calculate the proper size of a ringbuffer, it must be page-aligned

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_ring_buffer_info {
    pub ring_buffer: *mut hv_ring_buffer,
    pub /: *mut *mut u32 ring_size; / Include the shared header,
    pub ring_size_div10_reciprocal: reciprocal_value,
    pub ring_lock: spinlock_t,
    pub /: *mut *mut u32 ring_datasize; / < ring_size,
    pub priv_read_index: u32,
//
// The ring buffer mutex lock. This lock prevents the ring buffer from
// being freed while the ring buffer is being accessed.
//
    pub ring_buffer_mutex: mutex,
// Buffer that holds a copy of an incoming host packet
    pub pkt_buffer: *mut c_void,
    pub pkt_buffer_size: u32,
}

//
// VMBUS version is 32 bit entity broken up into
// two 16 bit quantities: major_number. minor_number.
//
// 0 . 13 (Windows Server 2008)
// 1 . 1  (Windows 7, WS2008 R2)
// 2 . 4  (Windows 8, WS2012)
// 3 . 0  (Windows 8.1, WS2012 R2)
// 4 . 0  (Windows 10)
// 4 . 1  (Windows 10 RS3)
// 5 . 0  (Newer Windows 10)
// 5 . 1  (Windows 10 RS4)
// 5 . 2  (Windows Server 2019, RS5)
// 5 . 3  (Windows Server 2022)
//
// The WS2008, WIN7, WIN8, and WIN8_1 versions are listed here for
// completeness but are no longer supported in the Linux kernel.
//

// Make maximum size of pipe payload of 16K

// Define PipeMode values.
pub const VMBUS_PIPE_TYPE_BYTE: c_uint = 0x00000000;
pub const VMBUS_PIPE_TYPE_MESSAGE: c_uint = 0x00000004;
// The size of the user defined data buffer for non-pipe offers.
pub const MAX_USER_DEFINED_BYTES: c_int = 120;
// The size of the user defined data buffer for pipe offers.
pub const MAX_PIPE_USER_DEFINED_BYTES: c_int = 116;
//
// At the center of the Channel Management library is the Channel Offer. This
// struct contains the fundamental information about an offer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_offer {
    pub if_type: guid_t,
    pub if_instance: guid_t,
//
// These two fields are not currently used.
//
    pub reserved1: u64,
    pub reserved2: u64,
    pub chn_flags: u16,
    pub /: *mut *mut *mut *mut u16 mmio_megabytes; / in bytes  1024  1024,
// Non-pipes: The user has MAX_USER_DEFINED_BYTES bytes.
    pub user_def: [c_uchar; MAX_USER_DEFINED_BYTES],
    pub std: },
//
// Pipes:
// The following structure is an integrated pipe protocol, which
// is implemented on top of standard user-defined data. Pipe
// clients have MAX_PIPE_USER_DEFINED_BYTES left for their own
// use.
//
    pub pipe_mode: u32,
    pub user_def: [c_uchar; MAX_PIPE_USER_DEFINED_BYTES],
    pub pipe: },
    pub u: },
//
// The sub_channel_index is defined in Win8: a value of zero means a
// primary channel and a value of non-zero means a sub-channel.
//
// Before Win8, the field is reserved, meaning it's always zero.
//
    pub sub_channel_index: u16,
    pub reserved3: u16,
    pub __packed: },
// Server Flags
pub const VMBUS_CHANNEL_ENUMERATE_DEVICE_INTERFACE: c_uint = 0x0001;
//
// This flag indicates that the channel is offered by the paravisor, and must
// use encrypted memory for the channel ring buffer.
//
pub const VMBUS_CHANNEL_CONFIDENTIAL_RING_BUFFER: c_uint = 0x0002;
//
// This flag indicates that the channel is offered by the paravisor, and must
// use encrypted memory for GPA direct packets and additional GPADLs.
//
pub const VMBUS_CHANNEL_CONFIDENTIAL_EXTERNAL_MEMORY: c_uint = 0x0004;
pub const VMBUS_CHANNEL_NAMED_PIPE_MODE: c_uint = 0x0010;
pub const VMBUS_CHANNEL_LOOPBACK_OFFER: c_uint = 0x0100;
pub const VMBUS_CHANNEL_PARENT_OFFER: c_uint = 0x0200;
pub const VMBUS_CHANNEL_REQUEST_MONITORED_NOTIFICATION: c_uint = 0x0400;
pub const VMBUS_CHANNEL_TLNPI_PROVIDER_OFFER: c_uint = 0x2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmpacket_descriptor {
    pub type: u16,
    pub offset8: u16,
    pub len8: u16,
    pub flags: u16,
    pub trans_id: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmpacket_header {
    pub prev_pkt_start_offset: u32,
    pub descriptor: vmpacket_descriptor,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmtransfer_page_range {
    pub byte_count: u32,
    pub byte_offset: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmtransfer_page_packet_header {
    pub d: vmpacket_descriptor,
    pub xfer_pageset_id: u16,
    pub sender_owns_set: u8,
    pub reserved: u8,
    pub range_cnt: u32,
    pub ranges: [vmtransfer_page_range; ],
    pub __packed: },
//
// This structure defines a range in guest physical space that can be made to
// look virtually contiguous.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpa_range {
    pub byte_count: u32,
    pub byte_offset: u32,
    pub pfn_array: [u64; ],
}

//
// This is the format for a GPA-Direct packet, which contains a set of GPA
// ranges, in addition to commands and/or data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmdata_gpa_direct {
    pub d: vmpacket_descriptor,
    pub reserved: u32,
    pub range_cnt: u32,
    pub range: [gpa_range; 1],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmbus_packet_type {
    VM_PKT_INVALID				= 0x0,
    VM_PKT_SYNCH				= 0x1,
    VM_PKT_ADD_XFER_PAGESET			= 0x2,
    VM_PKT_RM_XFER_PAGESET			= 0x3,
    VM_PKT_ESTABLISH_GPADL			= 0x4,
    VM_PKT_TEARDOWN_GPADL			= 0x5,
    VM_PKT_DATA_INBAND			= 0x6,
    VM_PKT_DATA_USING_XFER_PAGES		= 0x7,
    VM_PKT_DATA_USING_GPADL			= 0x8,
    VM_PKT_DATA_USING_GPA_DIRECT		= 0x9,
    VM_PKT_CANCEL_REQUEST			= 0xa,
    VM_PKT_COMP				= 0xb,
    VM_PKT_DATA_USING_ADDITIONAL_PKT	= 0xc,
    VM_PKT_ADDITIONAL_DATA			= 0xd
}

pub const VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED: c_int = 1;
// Version 1 messages
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmbus_channel_message_type {
    CHANNELMSG_INVALID			=  0,
    CHANNELMSG_OFFERCHANNEL		=  1,
    CHANNELMSG_RESCIND_CHANNELOFFER	=  2,
    CHANNELMSG_REQUESTOFFERS		=  3,
    CHANNELMSG_ALLOFFERS_DELIVERED	=  4,
    CHANNELMSG_OPENCHANNEL		=  5,
    CHANNELMSG_OPENCHANNEL_RESULT		=  6,
    CHANNELMSG_CLOSECHANNEL		=  7,
    CHANNELMSG_GPADL_HEADER		=  8,
    CHANNELMSG_GPADL_BODY			=  9,
    CHANNELMSG_GPADL_CREATED		= 10,
    CHANNELMSG_GPADL_TEARDOWN		= 11,
    CHANNELMSG_GPADL_TORNDOWN		= 12,
    CHANNELMSG_RELID_RELEASED		= 13,
    CHANNELMSG_INITIATE_CONTACT		= 14,
    CHANNELMSG_VERSION_RESPONSE		= 15,
    CHANNELMSG_UNLOAD			= 16,
    CHANNELMSG_UNLOAD_RESPONSE		= 17,
    CHANNELMSG_18				= 18,
    CHANNELMSG_19				= 19,
    CHANNELMSG_20				= 20,
    CHANNELMSG_TL_CONNECT_REQUEST		= 21,
    CHANNELMSG_MODIFYCHANNEL		= 22,
    CHANNELMSG_TL_CONNECT_RESULT		= 23,
    CHANNELMSG_MODIFYCHANNEL_RESPONSE	= 24,
    CHANNELMSG_COUNT
}

// Hyper-V supports about 2048 channels, and the RELIDs start with 1.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_message_header {
    pub msgtype: vmbus_channel_message_type,
    pub padding: u32,
    pub __packed: },
// Query VMBus Version parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_query_vmbus_version {
    pub header: vmbus_channel_message_header,
    pub version: u32,
    pub __packed: },
// VMBus Version Supported parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_version_supported {
    pub header: vmbus_channel_message_header,
    pub version_supported: u8,
    pub __packed: },
// Offer Channel parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_offer_channel {
    pub header: vmbus_channel_message_header,
    pub offer: vmbus_channel_offer,
    pub child_relid: u32,
    pub monitorid: u8,
//
// win7 and beyond splits this field into a bit field.
//
    pub monitor_allocated:1: u8,
    pub reserved:7: u8,
//
// These are new fields added in win7 and later.
// Do not access these fields without checking the
// negotiated protocol.
//
// If "is_dedicated_interrupt" is set, we must not set the
// associated bit in the channel bitmap while sending the
// interrupt to the host.
//
// connection_id is to be used in signaling the host.
//
    pub is_dedicated_interrupt:1: u16,
    pub reserved1:15: u16,
    pub connection_id: u32,
    pub __packed: },
// Rescind Offer parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_rescind_offer {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub __packed: },
//
// Request Offer -- no parameters, SynIC message contains the partition ID
// Set Snoop -- no parameters, SynIC message contains the partition ID
// Clear Snoop -- no parameters, SynIC message contains the partition ID
// All Offers Delivered -- no parameters, SynIC message contains the partition
// ID
// Flush Client -- no parameters, SynIC message contains the partition ID
//
// Open Channel parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_open_channel {
    pub header: vmbus_channel_message_header,
// Identifies the specific VMBus channel that is being opened.
    pub child_relid: u32,
// ID making a particular open request at a channel offer unique.
    pub openid: u32,
// GPADL for the channel's ring buffer.
    pub ringbuffer_gpadlhandle: u32,
//
// Starting with win8, this field will be used to specify
// the target virtual processor on which to deliver the interrupt for
// the host to guest communication.
// Prior to win8, incoming channel interrupts would only
// be delivered on cpu 0. Setting this value to 0 would
// preserve the earlier behavior.
//
    pub target_vp: u32,
//
// The upstream ring buffer begins at offset zero in the memory
// described by RingBufferGpadlHandle. The downstream ring buffer
// follows it at this offset (in pages).
//
    pub downstream_ringbuffer_pageoffset: u32,
// User-specific data to be passed along to the server endpoint.
    pub userdata: [c_uchar; MAX_USER_DEFINED_BYTES],
    pub __packed: },
// Open Channel Result parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_open_result {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub openid: u32,
    pub status: u32,
    pub __packed: },
// Modify Channel Result parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_modifychannel_response {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub status: u32,
    pub __packed: },
// Close channel parameters;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_close_channel {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub __packed: },
// Channel Message GPADL
pub const GPADL_TYPE_RING_BUFFER: c_int = 1;
pub const GPADL_TYPE_SERVER_SAVE_AREA: c_int = 2;
pub const GPADL_TYPE_TRANSACTION: c_int = 8;
//
// The number of PFNs in a GPADL message is defined by the number of
// pages that would be spanned by ByteCount and ByteOffset.  If the
// implied number of PFNs won't fit in this packet, there will be a
// follow-up packet that contains more.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_gpadl_header {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub gpadl: u32,
    pub range_buflen: u16,
    pub rangecount: u16,
    pub range: [gpa_range; ],
    pub __packed: },
// This is the followup packet that contains more PFNs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_gpadl_body {
    pub header: vmbus_channel_message_header,
    pub msgnumber: u32,
    pub gpadl: u32,
    pub pfn: [u64; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_gpadl_created {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub gpadl: u32,
    pub creation_status: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_gpadl_teardown {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub gpadl: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_gpadl_torndown {
    pub header: vmbus_channel_message_header,
    pub gpadl: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_relid_released {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub __packed: },
//
// Used by the paravisor only, means that the encrypted ring buffers and
// the encrypted external memory are supported
//
pub const VMBUS_FEATURE_FLAG_CONFIDENTIAL_CHANNELS: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_initiate_contact {
    pub header: vmbus_channel_message_header,
    pub vmbus_version_requested: u32,
    pub /: *mut *mut u32 target_vcpu; / The VCPU the host should respond to,
    pub interrupt_page: u64,
    pub msg_sint: u8,
    pub msg_vtl: u8,
    pub reserved: [u8; 2],
    pub /: *mut *mut u32 feature_flags; / VMBus version 6.0,
}

// Hyper-V socket: guest's connect()-ing to host
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_tl_connect_request {
    pub header: vmbus_channel_message_header,
    pub guest_endpoint_id: guid_t,
    pub host_service_id: guid_t,
    pub __packed: },
// Modify Channel parameters, cf. vmbus_send_modifychannel()
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_modifychannel {
    pub header: vmbus_channel_message_header,
    pub child_relid: u32,
    pub target_vp: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_version_response {
    pub header: vmbus_channel_message_header,
    pub version_supported: u8,
    pub connection_state: u8,
    pub padding: u16,
//
// On new hosts that support VMBus protocol 5.0, we must use
// VMBUS_MESSAGE_CONNECTION_ID_4 for the Initiate Contact Message,
// and for subsequent messages, we must use the Message Connection ID
// field in the host-returned Version Response Message.
//
// On old hosts, we should always use VMBUS_MESSAGE_CONNECTION_ID (1).
//
    pub msg_conn_id: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmbus_channel_state {
    CHANNEL_OFFER_STATE,
    CHANNEL_OPENING_STATE,
    CHANNEL_OPEN_STATE,
    CHANNEL_OPENED_STATE,
}

//
// Represents each channel msg on the vmbus connection This is a
// variable-size data structure depending on the msg type itself
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_msginfo {
// Bookkeeping stuff
    pub msglistentry: list_head,
// So far, this is only used to handle gpadl body message
    pub submsglist: list_head,
// Synchronize the request/response if needed
    pub waitevent: completion,
    pub waiting_channel: *mut vmbus_channel,
    pub version_supported: vmbus_channel_version_supported,
    pub open_result: vmbus_channel_open_result,
    pub gpadl_torndown: vmbus_channel_gpadl_torndown,
    pub gpadl_created: vmbus_channel_gpadl_created,
    pub version_response: vmbus_channel_version_response,
    pub modify_response: vmbus_channel_modifychannel_response,
    pub response: },
    pub msgsize: u32,
//
// The channel message that goes out on the "wire".
// It will contain at minimum the VMBUS_CHANNEL_MESSAGE_HEADER header
//
    pub msg: [c_uchar; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmbus_device_type {
    HV_IDE = 0,
    HV_SCSI,
    HV_FC,
    HV_NIC,
    HV_ND,
    HV_PCIE,
    HV_FB,
    HV_KBD,
    HV_MOUSE,
    HV_KVP,
    HV_TS,
    HV_HB,
    HV_SHUTDOWN,
    HV_FCOPY,
    HV_BACKUP,
    HV_DM,
    HV_UNKNOWN,
}

//
// Provides request ids for VMBus. Encapsulates guest memory
// addresses and stores the next available slot in req_arr
// to generate new ids in constant time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_requestor {
    pub req_arr: *mut u64,
    pub /: *mut *mut *mut unsigned long req_bitmap; / is a given slot available?,
    pub size: u32,
    pub next_request_id: u64,
    pub /: *mut *mut spinlock_t req_lock; / provides atomicity,
}

// NetVSC-specific

// StorVSC-specific

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_device {
// preferred ring buffer size in KB, 0 means no preferred size for this device
    pub pref_ring_size: usize,
    pub dev_type: u16,
    pub guid: guid_t,
    pub perf_device: bool,
    pub allowed_in_isolated: bool,
}

pub const VMBUS_DEFAULT_MAX_PKT_SIZE: c_int = 4096;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_gpadl {
    pub gpadl_handle: u32,
    pub size: u32,
    pub buffer: *mut c_void,
    pub decrypted: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel {
    pub listentry: list_head,
    pub device_obj: *mut hv_device,
    pub state: vmbus_channel_state,
    pub offermsg: vmbus_channel_offer_channel,
//
// These are based on the OfferMsg.MonitorId.
// Save it here for easy access.
//
    pub monitor_grp: u8,
    pub monitor_bit: u8,
    pub /: *mut *mut bool rescind; / got rescind msg,
    pub /: *mut *mut bool rescind_ref; / got rescind msg, got channel reference,
    pub rescind_event: completion,
    pub ringbuffer_gpadlhandle: vmbus_gpadl,
// Allocated memory for ring buffer
    pub ringbuffer_page: *mut page,
    pub ringbuffer_pagecount: u32,
    pub ringbuffer_send_offset: u32,
    pub /: *mut *mut hv_ring_buffer_info outbound; / send to parent,
    pub /: *mut *mut hv_ring_buffer_info inbound; / receive from parent,
    pub close_msg: vmbus_channel_close_channel,
// Statistics
    pub /: *mut *mut u64 interrupts; / Host to Guest interrupts,
    pub /: *mut *mut u64 sig_events; / Guest to Host events,
//
// Guest to host interrupts caused by the outbound ring buffer changing
// from empty to not empty.
//
    pub intr_out_empty: u64,
//
// Indicates that a full outbound ring buffer was encountered. The flag
// is set to true when a full outbound ring buffer is encountered and
// set to false when a write to the outbound ring buffer is completed.
//
    pub out_full_flag: bool,
// Channel callback's invoked in softirq context
    pub callback_event: tasklet_struct,
    pub context): *mut *mut void (onchannel_callback)(void,
    pub channel_callback_context: *mut c_void,
    pub new): u32 old, u32,
//
// Synchronize channel scheduling and channel removal; see the inline
// comments in vmbus_chan_sched() and vmbus_reset_channel_cb().
//
    pub sched_lock: spinlock_t,
//
// A channel can be marked for one of three modes of reading:
// BATCHED - callback called from taslket and should read
// channel until empty. Interrupts from the host
// are masked while read is in process (default).
// DIRECT - callback called from tasklet (softirq).
// ISR - callback called in interrupt context and must
// invoke its own deferred processing.
// Host interrupts are disabled and must be re-enabled
// when ring is empty.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_callback_mode {
    HV_CALL_BATCHED,
    HV_CALL_DIRECT,
    HV_CALL_ISR
    } callback_mode;

    bool is_dedicated_interrupt;
    u64 sig_event;

//
// Starting with win8, this field will be used to specify the
// target CPU on which to deliver the interrupt for the host
// to guest communication.
//
// Prior to win8, incoming channel interrupts would only be
// delivered on CPU 0. Setting this value to 0 would preserve
// the earlier behavior.
//
    u32 target_cpu;
//
// Support for sub-channels. For high performance devices,
// it will be useful to have multiple sub-channels to support
// a scalable communication infrastructure with the host.
// The support for sub-channels is implemented as an extension
// to the current infrastructure.
// The initial offer is considered the primary channel and this
// offer message will indicate if the host supports sub-channels.
// The guest is free to ask for sub-channels to be offered and can
// open these sub-channels as a normal "primary" channel. However,
// all sub-channels will have the same type and instance guids as the
// primary channel. Requests sent on a given channel will result in a
// response on the same channel.
//

//
// Sub-channel creation callback. This callback will be called in
// process context when a sub-channel offer is received from the host.
// The guest can open the sub-channel in the context of this callback.
//
    void (*sc_creation_callback)(struct vmbus_channel *new_sc);

//
// Channel rescind callback. Some channels (the hvsock ones), need to
// register a callback which is invoked in vmbus_onoffer_rescind().
//
    void (*chn_rescind_callback)(struct vmbus_channel *channel);

//
// All Sub-channels of a primary channel are linked here.
//
    struct list_head sc_list;
//
// The primary channel this sub-channel belongs to.
// This will be NULL for the primary channel.
//
    struct vmbus_channel *primary_channel;
//
// Support per-channel state for use by vmbus drivers.
//
    void *per_channel_state;

//
// Defer freeing channel until after all cpu's have
// gone through grace period.
//
    struct rcu_head rcu;

//
// For sysfs per-channel properties.
//
    struct kobject			kobj;

//
// For performance critical channels (storage, networking
// etc,), Hyper-V has a mechanism to enhance the throughput
// at the expense of latency:
// When the host is to be signaled, we just set a bit in a shared page
// and this bit will be inspected by the hypervisor within a certain
// window and if the bit is set, the host will be signaled. The window
// of time is the monitor latency - currently around 100 usecs. This
// mechanism improves throughput by:
//
// A) Making the host more efficient - each time it wakes up,
// potentially it will process more number of packets. The
// monitor latency allows a batch to build up.
// B) By deferring the hypercall to signal, we will also minimize
// the interrupts.
//
// Clearly, these optimizations improve throughput at the expense of
// latency. Furthermore, since the channel is shared for both
// control and data messages, control messages currently suffer
// unnecessary latency adversely impacting performance and boot
// time. To fix this issue, permit tagging the channel as being
// in "low latency" mode. In this mode, we will bypass the monitor
// mechanism.
//
    bool low_latency;

    bool probe_done;

//
// Cache the device ID here for easy access; this is useful, in
// particular, in situations where the channel's device_obj has
// not been allocated/initialized yet.
//
    u16 device_id;

//
// We must offload the handling of the primary/sub channels
// from the single-threaded vmbus_connection.work_queue to
// two different workqueue, otherwise we can block
// vmbus_connection.work_queue and hang: see vmbus_process_offer().
//
    struct work_struct add_channel_work;

//
// Guest to host interrupts caused by the inbound ring buffer changing
// from full to not full while a packet is waiting.
//
    u64 intr_in_full;

//
// The total number of write operations that encountered a full
// outbound ring buffer.
//
    u64 out_full_total;

//
// The number of write operations that were the first to encounter a
// full outbound ring buffer.
//
    u64 out_full_first;

// enabling/disabling fuzz testing on the channel (default is false)
    bool fuzz_testing_state;

//
// Interrupt delay will delay the guest from emptying the ring buffer
// for a specific amount of time. The delay is in microseconds and will
// be between 1 to a maximum of 1000, its default is 0 (no delay).
// The  Message delay will delay guest reading on a per message basis
// in microseconds between 1 to 1000 with the default being 0
// (no delay).
//
    u32 fuzz_testing_interrupt_delay;
    u32 fuzz_testing_message_delay;

// callback to generate a request ID from a request address
    u64 (*next_request_id_callback)(struct vmbus_channel *channel, u64 rqst_addr);
// callback to retrieve a request address from a request ID
    u64 (*request_addr_callback)(struct vmbus_channel *channel, u64 rqst_id);

// request/transaction ids for VMBus
    struct vmbus_requestor requestor;
    u32 rqstor_size;

// The max size of a packet on this channel
    u32 max_pkt_size;

// function to mmap ring buffer memory to the channel's sysfs ring attribute
    int (*mmap_prepare_ring_buffer)(struct vmbus_channel *channel, struct vm_area_desc *desc);

// boolean to control visibility of sysfs for ring buffer
    bool ring_sysfs_visible;
// The ring buffer is encrypted
    bool co_ring_buffer;
// The external memory is encrypted
    bool co_external_memory;
}

    pub \: *mut *mut vmbus_requestor rqstor = &(channel)->requestor;,
    pub \: spin_lock_irqsave(&rqstor->req_lock, flags);,
    pub &channel->requestor: *mut *mut vmbus_requestor rqstor =,
    pub flags): spin_unlock_irqrestore(&rqstor->req_lock,,
    pub rqst_addr): *mut *mut u64 vmbus_next_request_id(struct vmbus_channel channel, u64,
    pub rqst_addr): u64,
    pub rqst_addr): u64,
    pub trans_id): *mut *mut u64 vmbus_request_addr(struct vmbus_channel channel, u64,
    pub VMBUS_CHANNEL_CONFIDENTIAL_RING_BUFFER): return !!(o->offer.chn_flags &,
    pub VMBUS_CHANNEL_CONFIDENTIAL_EXTERNAL_MEMORY): return !!(o->offer.chn_flags &,
    pub VMBUS_CHANNEL_TLNPI_PROVIDER_OFFER): return !!(o->offer.chn_flags &,
    pub is_hvsock_offer(&c->offermsg): return,
    pub 0: return c->offermsg.offer.sub_channel_index !=,
    pub mode: c->callback_mode =,
    pub s: c->per_channel_state =,
    pub c->per_channel_state: return,
    pub flags: c_ulong,
    pub flags): spin_lock_irqsave(&c->outbound.ring_lock,,
    pub true: c->out_full_flag =,
    pub flags): spin_unlock_irqrestore(&c->outbound.ring_lock,,
    pub false: c->out_full_flag =,
    pub size: c->outbound.ring_buffer->pending_send_sz =,
    pub hdr): *mut void vmbus_onmessage(struct vmbus_channel_message_header,
    pub vmbus_request_offers(void): c_int,
//
// APIs for managing sub-channels.
//
    pub new_sc)): *mut *mut void (sc_cr_cb)(struct vmbus_channel,
    pub )): *mut *mut void (chn_rescind_cb)(struct vmbus_channel,
// The format must be the same as struct vmdata_gpa_direct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_packet_page_buffer {
    pub type: u16,
    pub dataoffset8: u16,
    pub length8: u16,
    pub flags: u16,
    pub transactionid: u64,
    pub reserved: u32,
    pub rangecount: u32,
    pub range: [hv_page_buffer; MAX_PAGE_BUFFER_COUNT],
    pub __packed: },
// The format must be the same as struct vmdata_gpa_direct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_packet_multipage_buffer {
    pub type: u16,
    pub dataoffset8: u16,
    pub length8: u16,
    pub flags: u16,
    pub transactionid: u64,
    pub reserved: u32,
    pub /: *mut *mut u32 rangecount; / Always 1 in this case,
    pub range: hv_multipage_buffer,
    pub __packed: },
// The format must be the same as struct vmdata_gpa_direct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_packet_mpb_array {
    pub type: u16,
    pub dataoffset8: u16,
    pub length8: u16,
    pub flags: u16,
    pub transactionid: u64,
    pub reserved: u32,
    pub /: *mut *mut u32 rangecount; / Always 1 in this case,
    pub range: hv_mpb_array,
    pub __packed: },
    pub recv_size): u32 send_size, u32,
    pub channel): *mut void vmbus_free_ring(struct vmbus_channel,
    pub context): *mut c_void,
    pub channel): *mut int vmbus_disconnect_ring(struct vmbus_channel,
    pub context): *mut c_void,
    pub channel): *mut extern void vmbus_close(struct vmbus_channel,
    pub flags): u32,
    pub flags): u32,
    pub requestid): u64,
    pub gpadl): *mut vmbus_gpadl,
    pub gpadl): *mut vmbus_gpadl,
    pub gpadl): *mut vmbus_gpadl,
    pub chunk_cnt_out): *mut u32,
    pub chunk_cnt): *mut *mut *mut *mut extern void vmbus_free_buffer(void addr, struct page chunks, u32,
    pub channel): *mut void vmbus_reset_channel_cb(struct vmbus_channel,
    pub requestid): *mut u64,
    pub requestid): *mut u64,
// Base driver object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_driver {
    pub name: *const c_char,
//
// A hvsock offer, which has a VMBUS_CHANNEL_TLNPI_PROVIDER_OFFER
// channel flag, actually doesn't mean a synthetic device because the
// offer's if_type/if_instance can change for every new hvsock
// connection.
//
// However, to facilitate the notification of new-offer/rescind-offer
// from vmbus driver to hvsock driver, we can handle hvsock offer as
// a special vmbus device, and hence we need the below flag to
// indicate if the driver is the hvsock driver or not: we need to
// specially treat the hvosck offer & driver in vmbus_match().
//
    pub hvsock: bool,
// the device type supported by this driver
    pub dev_type: guid_t,
    pub id_table: *const hv_vmbus_device_id,
    pub driver: device_driver,
// dynamic device GUID's
    pub lock: spinlock_t,
    pub list: list_head,
    pub dynids: },
    pub ): *const *const *const int (probe)(struct hv_device , struct hv_vmbus_device_id,
    pub dev): *mut *mut void (remove)(struct hv_device,
    pub ): *mut *mut void (shutdown)(struct hv_device,
    pub ): *mut *mut int (suspend)(struct hv_device,
    pub ): *mut *mut int (resume)(struct hv_device,
}

// Base device object
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_device {
// the device type id of this device
    pub dev_type: guid_t,
// the device instance id of this device
    pub dev_instance: guid_t,
    pub vendor_id: u16,
    pub device_id: u16,
    pub device: device,
    pub channel: *mut vmbus_channel,
    pub channels_kset: *mut kset,
    pub dma_parms: device_dma_parameters,
    pub dma_mask: u64,
// place holder to keep track of the dir for hv device in debugfs
    pub debug_dir: *mut dentry,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &dev->device) -> return;
}

extern "C" {
    pub fn hv_vmbus_exists() -> bool;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_ring_buffer_debug_info {
    pub current_interrupt_mask: u32,
    pub current_read_index: u32,
    pub current_write_index: u32,
    pub bytes_avail_toread: u32,
    pub bytes_avail_towrite: u32,
}

extern "C" {
    pub fn hv_ringbuffer_spinlock_busy(channel: *mut vmbus_channel) -> bool;
}
// Vmbus interface

extern "C" {
    pub fn vmbus_driver_unregister(hv_driver: *mut hv_driver);
}
extern "C" {
    pub fn vmbus_hvsock_device_unregister(channel: *mut vmbus_channel);
}
extern "C" {
    pub fn vmbus_free_mmio(start: resource_size_t, size: resource_size_t);
}
extern "C" {
    pub fn vmbus_initiate_unload(crash: bool);
}
extern "C" {
    pub fn vmbus_set_skip_unload(skip: bool);
}
//
// GUID definitions of various offer types - services offered to the guest.
//
// Network GUID
// {f8615163-df3e-46c5-913f-f2d2f965ed0e}
//

//
// IDE GUID
// {32412632-86cb-44a2-9b5c-50d1417354f5}
//

//
// SCSI GUID
// {ba6163d9-04a1-4d29-b605-72e2ffb1dc7f}
//

//
// Shutdown GUID
// {0e0b6031-5213-4934-818b-38d90ced39db}
//

//
// Time Synch GUID
// {9527E630-D0AE-497b-ADCE-E80AB0175CAF}
//

//
// Heartbeat GUID
// {57164f39-9115-4e78-ab55-382f3bd5422d}
//

//
// KVP GUID
// {a9a0f4e7-5a45-4d96-b827-8a841e8c03e6}
//

//
// Dynamic memory GUID
// {525074dc-8985-46e2-8057-a307dc18a502}
//

//
// Mouse GUID
// {cfa8b69e-5b4a-4cc0-b98b-8ba1a1f3f95a}
//

//
// Keyboard GUID
// {f912ad6d-2b17-48ea-bd65-f927a61c7684}
//

//
// VSS (Backup/Restore) GUID
//

//
// Synthetic Video GUID
// {DA0A7802-E377-4aac-8E77-0558EB1073F8}
//

//
// Synthetic FC GUID
// {2f9bcc4a-0069-4af3-b76b-6fd0be528cda}
//

//
// Guest File Copy Service
// {34D14BE3-DEE4-41c8-9AE7-6B174977C192}
//

//
// NetworkDirect. This is the guest RDMA service.
// {8c2eaf3d-32a7-4b09-ab99-bd1f1c86b501}
//

//
// PCI Express Pass Through
// {44C4F61D-4444-4400-9D52-802E27EDE19F}
//

//
// Linux doesn't support these 4 devices: the first two are for
// Automatic Virtual Machine Activation, the third is for
// Remote Desktop Virtualization, and the fourth is Initial
// Machine Configuration (IMC) used only by Windows guests.
// {f8e65716-3cb3-4a06-9a60-1889c5cccab5}
// {3375baf4-9e15-4b30-b765-67acb10d607b}
// {276aacf4-ac15-426c-98dd-7521ad3f01fe}
// {c376c1c3-d276-48d2-90a9-c04748072c60}
//

//
// Common header for Hyper-V ICs
//
pub const ICMSGTYPE_NEGOTIATE: c_int = 0;
pub const ICMSGTYPE_HEARTBEAT: c_int = 1;
pub const ICMSGTYPE_KVPEXCHANGE: c_int = 2;
pub const ICMSGTYPE_SHUTDOWN: c_int = 3;
pub const ICMSGTYPE_TIMESYNC: c_int = 4;
pub const ICMSGTYPE_VSS: c_int = 5;
pub const ICMSGTYPE_FCOPY: c_int = 7;
pub const ICMSGHDRFLAG_TRANSACTION: c_int = 1;
pub const ICMSGHDRFLAG_REQUEST: c_int = 2;
pub const ICMSGHDRFLAG_RESPONSE: c_int = 4;
//
// While we want to handle util services as regular devices,
// there is only one instance of each of these services; so
// we statically allocate the service specific state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_util_service {
    pub recv_buffer: *mut u8,
    pub channel: *mut c_void,
    pub ): *mut *mut void (util_cb)(void,
    pub ): *mut *mut int (util_init)(struct hv_util_service,
    pub (*util_init_transport)(void): *mut c_int,
    pub (*util_deinit)(void): *mut c_void,
    pub (*util_pre_suspend)(void): *mut c_int,
    pub (*util_pre_resume)(void): *mut c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbuspipe_hdr {
    pub flags: u32,
    pub msgsize: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ic_version {
    pub major: u16,
    pub minor: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmsg_hdr {
    pub icverframe: ic_version,
    pub icmsgtype: u16,
    pub icvermsg: ic_version,
    pub icmsgsize: u16,
    pub status: u32,
    pub ictransaction_id: u8,
    pub icflags: u8,
    pub reserved: [u8; 2],
    pub __packed: },
pub const IC_VERSION_NEGOTIATION_MAX_VER_COUNT: c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct icmsg_negotiate {
    pub icframe_vercnt: u16,
    pub icmsg_vercnt: u16,
    pub reserved: u32,
    pub /: *mut *mut ic_version icversion_data[]; / any size array,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct shutdown_msg_data {
    pub reason_code: u32,
    pub timeout_seconds: u32,
    pub flags: u32,
    pub display_message: [u8; 2048],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct heartbeat_msg_data {
    pub seq_num: u64,
    pub reserved: [u32; 8],
    pub __packed: },
// Time Sync IC defs
pub const ICTIMESYNCFLAG_PROBE: c_int = 0;
pub const ICTIMESYNCFLAG_SYNC: c_int = 1;
pub const ICTIMESYNCFLAG_SAMPLE: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ictimesync_data {
    pub parenttime: u64,
    pub childtime: u64,
    pub roundtriptime: u64,
    pub flags: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ictimesync_ref_data {
    pub parenttime: u64,
    pub vmreferencetime: u64,
    pub flags: u8,
    pub leapflags: c_char,
    pub stratum: c_char,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyperv_service_callback {
    pub msg_type: u8,
    pub log_msg: *mut c_char,
    pub data: guid_t,
    pub channel: *mut vmbus_channel,
    pub context): *mut *mut void (callback)(void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_dma_range {
    pub dma: dma_addr_t,
    pub mapping_size: u32,
}

pub const MAX_SRV_VER: c_uint = 0x7ffffff;
extern "C" {
    pub fn hv_process_channel_removal(channel: *mut vmbus_channel);
}
extern "C" {
    pub fn vmbus_setevent(channel: *mut vmbus_channel);
}
//
// Negotiated version with the Host.
//
extern "C" {
    pub fn vmbus_send_modifychannel(channel: *mut vmbus_channel, target_vp: u32) -> c_int;
}
extern "C" {
    pub fn vmbus_set_event(channel: *mut vmbus_channel);
}
extern "C" {
    pub fn vmbus_channel_set_cpu(channel: *mut vmbus_channel, target_cpu: u32) -> c_int;
}
// Get the start of the ring buffer.
//
// Mask off host interrupt callback notifications
//
// make sure mask update is not reordered
//
// Re-enable host callback and return number of outstanding bytes
//
// make sure mask update is not reordered
//
// Now check to see if the ring buffer is still empty.
// If it is not, we raced and we need to process new
// incoming messages.
//
extern "C" {
    pub fn hv_get_bytes_to_read(_arg: rbi) -> return;
}
//
// An API to support in-place processing of incoming VMBUS packets.
//
// Get data payload associated with descriptor
// Get data size associated with descriptor
// Get packet length associated with descriptor
extern "C" {
    pub fn hv_pkt_iter_close(channel: *mut vmbus_channel);
}

//
// Interface for passing data between SR-IOV PF and VF drivers. The VF driver
// sends requests to read and write blocks. Each block must be 128 bytes or
// smaller. Optionally, the VF driver can register a callback function which
// will be invoked when the host says that one or more of the first 64 block
// IDs is "invalid" which means that the VF driver should reread them.
//
pub const HV_CONFIG_BLOCK_SIZE_MAX: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hyperv_pci_block_ops {
    pub bytes_returned): *mut unsigned int block_id, unsigned int,
    pub block_id): c_uint,
    pub block_mask)): u64,
}

