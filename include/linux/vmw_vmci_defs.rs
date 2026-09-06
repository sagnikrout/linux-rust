//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/vmw_vmci_defs.h
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
// VMware VMCI Driver
//
// Copyright (C) 2012 VMware, Inc. All rights reserved.
//

// Register offsets.
pub const VMCI_STATUS_ADDR: c_uint = 0x00;
pub const VMCI_CONTROL_ADDR: c_uint = 0x04;
pub const VMCI_ICR_ADDR: c_uint = 0x08;
pub const VMCI_IMR_ADDR: c_uint = 0x0c;
pub const VMCI_DATA_OUT_ADDR: c_uint = 0x10;
pub const VMCI_DATA_IN_ADDR: c_uint = 0x14;
pub const VMCI_CAPS_ADDR: c_uint = 0x18;
pub const VMCI_RESULT_LOW_ADDR: c_uint = 0x1c;
pub const VMCI_RESULT_HIGH_ADDR: c_uint = 0x20;
pub const VMCI_DATA_OUT_LOW_ADDR: c_uint = 0x24;
pub const VMCI_DATA_OUT_HIGH_ADDR: c_uint = 0x28;
pub const VMCI_DATA_IN_LOW_ADDR: c_uint = 0x2c;
pub const VMCI_DATA_IN_HIGH_ADDR: c_uint = 0x30;
pub const VMCI_GUEST_PAGE_SHIFT: c_uint = 0x34;
// Max number of devices.
pub const VMCI_MAX_DEVICES: c_int = 1;
// Status register bits.

// Control register bits.

// Capabilities register bits.

// Interrupt Cause register bits.

// Interrupt Mask register bits.

//
// Maximum MSI/MSI-X interrupt vectors in the device.
// If VMCI_CAPS_DMA_DATAGRAM is supported by the device,
// VMCI_MAX_INTRS_DMA_DATAGRAM vectors are available,
// otherwise only VMCI_MAX_INTRS_NOTIFICATION.
//
pub const VMCI_MAX_INTRS_NOTIFICATION: c_int = 2;
pub const VMCI_MAX_INTRS_DMA_DATAGRAM: c_int = 3;

//
// Supported interrupt vectors.  There is one for each ICR value above,
// but here they indicate the position in the vector array/message ID.
//
// A single VMCI device has an upper limit of 128MB on the amount of
// memory that can be used for queue pairs. Since each queue pair
// consists of at least two pages, the memory limit also dictates the
// number of queue pairs a guest can create.
//

//
// There can be at most PAGE_SIZE doorbells since there is one doorbell
// per byte in the doorbell bitmap page.
//

//
// Queues with pre-mapped data pages must be small, so that we don't pin
// too much kernel memory (especially on vmkernel).  We limit a queuepair to
// 32 KB, or 16 KB per queue for symmetrical pairs.
//

//
// The version of the VMCI device that supports MMIO access to registers
// requests 256KB for BAR1 whereas the version of VMCI that supports
// MSI/MSI-X only requests 8KB. The layout of the larger 256KB region is:
// - the first 128KB are used for MSI/MSI-X.
// - the following 64KB are used for MMIO register access.
// - the remaining 64KB are unused.
//

//
// For VMCI devices supporting the VMCI_CAPS_DMA_DATAGRAM capability, the
// sending and receiving of datagrams can be performed using DMA to/from
// a driver allocated buffer.
// Sending and receiving will be handled as follows:
// - when sending datagrams, the driver initializes the buffer where the
// data part will refer to the outgoing VMCI datagram, sets the busy flag
// to 1 and writes the address of the buffer to VMCI_DATA_OUT_HIGH_ADDR
// and VMCI_DATA_OUT_LOW_ADDR. Writing to VMCI_DATA_OUT_LOW_ADDR triggers
// the device processing of the buffer. When the device has processed the
// buffer, it will write the result value to the buffer and then clear the
// busy flag.
// - when receiving datagrams, the driver initializes the buffer where the
// data part will describe the receive buffer, clears the busy flag and
// writes the address of the buffer to VMCI_DATA_IN_HIGH_ADDR and
// VMCI_DATA_IN_LOW_ADDR. Writing to VMCI_DATA_IN_LOW_ADDR triggers the
// device processing of the buffer. The device will copy as many available
// datagrams into the buffer as possible, and then sets the busy flag.
// When the busy flag is set, the driver will process the datagrams in the
// buffer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_data_in_out_header {
    pub busy: u32,
    pub opcode: u32,
    pub size: u32,
    pub rsvd: u32,
    pub result: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_sg_elem {
    pub addr: u64,
    pub size: u64,
}

//
// We have a fixed set of resource IDs available in the VMX.
// This allows us to have a very simple implementation since we statically
// know how many will create datagram handles. If a new caller arrives and
// we have run out of slots we can manually increment the maximum size of
// available resource IDs.
//
// VMCI reserved hypervisor datagram resource IDs.
//
// VMCI_DATAGRAM_REQUEST_MAP and VMCI_DATAGRAM_REMOVE_MAP are
// obsoleted by the removal of VM to VM communication.
//
// VMCI_VSOCK_VMX_LOOKUP was assigned to 12 for Fusion 3.0/3.1,
// WS 7.0/7.1 and ESX 4.1
//
// struct vmci_handle - Ownership information structure
// @context:    The VMX context ID.
// @resource:   The resource ID (used for locating in resource hash).
//
// The vmci_handle structure is used to track resources used within
// vmw_vmci.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_handle {
    pub context: u32,
    pub resource: u32,
}

extern "C" {
    pub fn vmci_handle_is_equal(_arg: h, _arg: VMCI_INVALID_HANDLE) -> return;
}
//
// The below defines can be used to send anonymous requests.
// This also indicates that no response is expected.
//

// The lowest 16 context ids are reserved for internal use.

//
// Hypervisor context id, used for calling into hypervisor
// supplied services from the VM.
//
pub const VMCI_HYPERVISOR_CONTEXT_ID: c_int = 0;
//
// Well-known context id, a logical context that contains a set of
// well-known services. This context ID is now obsolete.
//
pub const VMCI_WELL_KNOWN_CONTEXT_ID: c_int = 1;
//
// Context ID used by host endpoints.
//
pub const VMCI_HOST_CONTEXT_ID: c_int = 2;

//
// The VMCI_CONTEXT_RESOURCE_ID is used together with vmci_make_handle to make
// handles that refer to a specific context.
//
pub const VMCI_CONTEXT_RESOURCE_ID: c_int = 0;
//
// VMCI error codes.
//
// VMCI clients should return error code within this range
// Internal error codes.
// VMCI reserved events.
// Only applicable to guest endpoints
// Applicable to guest and host
// Only applicable to guest endpoints
// Applicable to guest and host
//
// Applicable to VMX and vmk.  On vmk,
// this event has the Context payload type.
//
// Applicable to VMX and vmk.  Same as
// above for the payload type.
//
// Of the above events, a few are reserved for use in the VMX, and
// other endpoints (guest and host kernel) should not use them. For
// the rest of the events, we allow both host and guest endpoints to
// subscribe to them, to maintain the same API for host and guest
// endpoints.
//

// Reserved guest datagram resource ids.
pub const VMCI_EVENT_HANDLER: c_int = 0;
//
// VMCI coarse-grained privileges (per context or host
// process/endpoint. An entity with the restricted flag is only
// allowed to interact with the hypervisor and trusted entities.
//
// 0 through VMCI_RESERVED_RESOURCE_ID_MAX are reserved.
pub const VMCI_RESERVED_RESOURCE_ID_MAX: c_int = 1023;
//
// Driver version.
//
// Increment major version when you make an incompatible change.
// Compatibility goes both ways (old driver with new executable
// as well as new driver with old executable).
//
// Never change VMCI_VERSION_SHIFT_WIDTH
pub const VMCI_VERSION_SHIFT_WIDTH: c_int = 16;

//
// VMCI_VERSION is always the current version.  Subsequently listed
// versions are ways of detecting previous versions of the connecting
// application (i.e., VMX).
//
// VMCI_VERSION_NOVMVM: This version removed support for VM to VM
// communication.
//
// VMCI_VERSION_NOTIFY: This version introduced doorbell notification
// support.
//
// VMCI_VERSION_HOSTQP: This version introduced host end point support
// for hosted products.
//
// VMCI_VERSION_PREHOSTQP: This is the version prior to the adoption of
// support for host end-points.
//
// VMCI_VERSION_PREVERS2: This fictional version number is intended to
// represent the version of a VMX which doesn't call into the driver
// with ioctl VERSION2 and thus doesn't establish its version with the
// driver.
//

//
// The VMCI IOCTLs.  We use identity code 7, as noted in ioctl-number.rst,
// and we start at sequence 9f.  This gives us the same values that our
// shipping products use, starting at 1951, provided we leave out the
// direction and structure size.  Note that VMMon occupies the block
// following us, starting at 2001.
//

// IOCTL_VM_SOCKETS_GET_LOCAL_CID		_IO(7, 0xb9)

// IOCTL_VMMON_START				_IO(7, 0xd1)*/	/* 2001
//
// struct vmci_queue_header - VMCI Queue Header information.
//
// A Queue cannot stand by itself as designed.  Each Queue's header
// contains a pointer into itself (the producer_tail) and into its peer
// (consumer_head).  The reason for the separation is one of
// accessibility: Each end-point can modify two things: where the next
// location to enqueue is within its produce_q (producer_tail); and
// where the next dequeue location is in its consume_q (consumer_head).
//
// An end-point cannot modify the pointers of its peer (guest to
// guest; NOTE that in the host both queue headers are mapped r/w).
// But, each end-point needs read access to both Queue header
// structures in order to determine how much space is used (or left)
// in the Queue.  This is because for an end-point to know how full
// its produce_q is, it needs to use the consumer_head that points into
// the produce_q but -that- consumer_head is in the Queue header for
// that end-points consume_q.
//
// Thoroughly confused?  Sorry.
//
// producer_tail: the point to enqueue new entrants.  When you approach
// a line in a store, for example, you walk up to the tail.
//
// consumer_head: the point in the queue from which the next element is
// dequeued.  In other words, who is next in line is he who is at the
// head of the line.
//
// Also, producer_tail points to an empty byte in the Queue, whereas
// consumer_head points to a valid byte of data (unless producer_tail ==
// consumer_head in which case consumer_head does not point to a valid
// byte of data).
//
// For a queue of buffer 'size' bytes, the tail and head pointers will be in
// the range [0, size-1].
//
// If produce_q_header->producer_tail == consume_q_header->consumer_head
// then the produce_q is empty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_queue_header {
// All fields are 64bit and aligned.
    pub /: *mut *mut vmci_handle handle; / Identifier.,
    pub /: *mut *mut u64 producer_tail; / Offset in this queue.,
    pub /: *mut *mut u64 consumer_head; / Offset in peer queue.,
}

//
// struct vmci_datagram - Base struct for vmci datagrams.
// @dst:        A vmci_handle that tracks the destination of the datagram.
// @src:        A vmci_handle that tracks the source of the datagram.
// @payload_size:       The size of the payload.
//
// vmci_datagram structs are used when sending vmci datagrams.  They include
// the necessary source and destination information to properly route
// the information along with the size of the package.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_datagram {
    pub dst: vmci_handle,
    pub src: vmci_handle,
    pub payload_size: u64,
}

//
// Second flag is for creating a well-known handle instead of a per context
// handle.  Next flag is for deferring datagram delivery, so that the
// datagram callback is invoked in a delayed context (not interrupt context).
//
pub const VMCI_FLAG_DG_NONE: c_int = 0;

//
// Maximum supported size of a VMCI datagram for routable datagrams.
// Datagrams going to the hypervisor are allowed to be larger.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_payload_qp {
    pub /: *mut *mut vmci_handle handle; / queue_pair handle.,
    pub /: *mut *mut u32 peer_id; / Context id of attaching/detaching VM.,
    pub _pad: u32,
}

// Flags for VMCI queue_pair API.
// Fail alloc if QP not created by peer.
// Only allow attaches from local context.
// Host won't block when guest is quiesced.
// Pin data pages in ESX.  Used with NONBLOCK
// Update the following flag when adding new flags.
// Convenience flags
//
// We allow at least 1024 more event datagrams from the hypervisor past the
// normally allowed datagrams pending for a given context.  We define this
// limit on event datagrams from the hypervisor to guard against DoS attack
// from a malicious VM which could repeatedly attach to and detach from a queue
// pair, causing events to be queued at the destination VM.  However, the rate
// at which such events can be generated is small since it requires a VM exit
// and handling of queue pair attach/detach call at the hypervisor.  Event
// datagrams may be queued up at the destination VM if it has interrupts
// disabled or if it is not draining events for some other reason.  1024
// datagrams is a grossly conservative estimate of the time for which
// interrupts may be disabled in the destination VM, but at the same time does
// not exacerbate the memory pressure problem on the host by much (size of each
// event datagram is small).
//

//
// Struct used for querying, via VMCI_RESOURCES_QUERY, the availability of
// hypervisor resources.  Struct size is 16 bytes. All fields in struct are
// aligned to their natural alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_resource_query_hdr {
    pub hdr: vmci_datagram,
    pub num_resources: u32,
    pub _padding: u32,
}

//
// Convenience struct for negotiating vectors. Must match layout of
// VMCIResourceQueryHdr minus the struct vmci_datagram header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_resource_query_msg {
    pub num_resources: u32,
    pub _padding: u32,
    pub resources: [u32; 1],
}

//
// The maximum number of resources that can be queried using
// VMCI_RESOURCE_QUERY is 31, as the result is encoded in the lower 31
// bits of a positive return value. Negative values are reserved for
// errors.
//
pub const VMCI_RESOURCE_QUERY_MAX_NUM: c_int = 31;
// Maximum size for the VMCI_RESOURCE_QUERY request.

//
// Struct used for setting the notification bitmap.  All fields in
// struct are aligned to their natural alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_notify_bm_set_msg {
    pub hdr: vmci_datagram,
    pub bitmap_ppn32: u32,
    pub bitmap_ppn64: u64,
}

//
// Struct used for linking a doorbell handle with an index in the
// notify bitmap. All fields in struct are aligned to their natural
// alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_doorbell_link_msg {
    pub hdr: vmci_datagram,
    pub handle: vmci_handle,
    pub notify_idx: u64,
}

//
// Struct used for unlinking a doorbell handle from an index in the
// notify bitmap. All fields in struct are aligned to their natural
// alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_doorbell_unlink_msg {
    pub hdr: vmci_datagram,
    pub handle: vmci_handle,
}

//
// Struct used for generating a notification on a doorbell handle. All
// fields in struct are aligned to their natural alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_doorbell_notify_msg {
    pub hdr: vmci_datagram,
    pub handle: vmci_handle,
}

//
// This struct is used to contain data for events.  Size of this struct is a
// multiple of 8 bytes, and all fields are aligned to their natural alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_data {
    pub /: *mut *mut u32 event; / 4 bytes.,
    pub _pad: u32,
// Event payload is put here.
}

//
// Define the different VMCI_EVENT payload data types here.  All structs must
// be a multiple of 8 bytes, and fields must be aligned to their natural
// alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_payld_ctx {
    pub /: *mut *mut u32 context_id; / 4 bytes.,
    pub _pad: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_payld_qp {
    pub /: *mut *mut vmci_handle handle; / queue_pair handle.,
    pub /: *mut *mut u32 peer_id; / Context id of attaching/detaching VM.,
    pub _pad: u32,
}

//
// We define the following struct to get the size of the maximum event
// data the hypervisor may send to the guest.  If adding a new event
// payload type above, add it to the following struct too (inside the
// union).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_data_max {
    pub event_data: vmci_event_data,
    pub context_payload: vmci_event_payld_ctx,
    pub qp_payload: vmci_event_payld_qp,
    pub ev_data_payload: },
}

//
// Struct used for VMCI_EVENT_SUBSCRIBE/UNSUBSCRIBE and
// VMCI_EVENT_HANDLER messages.  Struct size is 32 bytes.  All fields
// in struct are aligned to their natural alignment.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_msg {
    pub hdr: vmci_datagram,
// Has event type and payload.
    pub event_data: vmci_event_data,
// Payload gets put here.
}

// Event with context payload.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_ctx {
    pub msg: vmci_event_msg,
    pub payload: vmci_event_payld_ctx,
}

// Event with QP payload.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_event_qp {
    pub msg: vmci_event_msg,
    pub payload: vmci_event_payld_qp,
}

//
// Structs used for queue_pair alloc and detach messages.  We align fields of
// these structs to 64bit boundaries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_qp_alloc_msg {
    pub hdr: vmci_datagram,
    pub handle: vmci_handle,
    pub peer: u32,
    pub flags: u32,
    pub produce_size: u64,
    pub consume_size: u64,
    pub num_ppns: u64,
// List of PPNs placed here.
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmci_qp_detach_msg {
    pub hdr: vmci_datagram,
    pub handle: vmci_handle,
}

// VMCI Doorbell API.

extern "C" {
    pub fn void(client_data: *mut *mut vmci_callback) (void) -> typedef;
}
//
// struct vmci_qp - A vmw_vmci queue pair handle.
//
// This structure is used as a handle to a queue pair created by
// VMCI.  It is intentionally left opaque to clients.
//
// Callback needed for correctly waiting on events.
// VMCI Event API.
//
// We use the following inline function to access the payload data
// associated with an event data.
//
// Helper to read a value from a head or tail pointer. For X86_32, the
// pointer is treated as a 32bit value, since the pointer value
// never exceeds a 32bit value in this case. Also, doing an
// atomic64_read on X86_32 uniprocessor systems may be implemented
// as a non locked cmpxchg8b, that may end up overwriting updates done
// by the VMCI device to the memory location. On 32bit SMP, the lock
// prefix will be used, so correctness isn't an issue, but using a
// 64bit operation still adds unnecessary overhead.
//
extern "C" {
    pub fn READ_ONCE()var: *mut *mut (unsigned long) -> return;
}
//
// Helper to set the value of a head or tail pointer. For X86_32, the
// pointer is treated as a 32bit value, since the pointer value
// never exceeds a 32bit value in this case. On 32bit SMP, using a
// locked cmpxchg8b adds unnecessary overhead.
//
// XXX buggered on big-endian
//
// Helper to add a given offset to a head or tail pointer. Wraps the
// value of the pointer around the max size of the queue.
//
// Helper routine to get the Producer Tail from the supplied queue.
//
extern "C" {
    pub fn vmci_q_read_pointer(_arg: &qh->producer_tail) -> return;
}
//
// Helper routine to get the Consumer Head from the supplied queue.
//
extern "C" {
    pub fn vmci_q_read_pointer(_arg: &qh->consumer_head) -> return;
}
//
// Helper routine to increment the Producer Tail.  Fundamentally,
// vmci_qp_add_pointer() is used to manipulate the tail itself.
//
// Helper routine to increment the Consumer Head.  Fundamentally,
// vmci_qp_add_pointer() is used to manipulate the head itself.
//
// Helper routine for getting the head and the tail pointer for a queue.
// Both the VMCIQueues are needed to get both the pointers for one queue.
//
// producer_tail = vmci_q_header_producer_tail(produce_q_header);
// consumer_head = vmci_q_header_consumer_head(consume_q_header);
//
// Finds available free space in a produce queue to enqueue more
// data or reports an error if queue pair corruption is detected.
//
// Deduct 1 to avoid tail becoming equal to head which causes
// ambiguity. If head and tail are equal it means that the
// queue is empty.
//
// vmci_q_header_free_space() does all the heavy lifting of
// determing the number of free bytes in a Queue.  This routine,
// then subtracts that size from the full size of the Queue so
// the caller knows how many bytes are ready to be dequeued.
// Results:
// On success, available data size in bytes (up to MAX_INT64).
// On failure, appropriate error code.
//
