//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/raspberrypi/vchiq_core.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2010-2012 Broadcom. All rights reserved.

// Do this so that we can test-build the code on non-rpi systems

// Macro flag: #define dsb(a)

pub const VCHIQ_SERVICE_HANDLE_INVALID: c_int = 0;
pub const VCHIQ_SLOT_SIZE: c_int = 4096;

// Macro flag: #define DEBUG_INITIALISE(local)
// Macro flag: #define DEBUG_TRACE(d)

// Macro flag: #define DEBUG_COUNT(d)

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vchiq_connstate {
    VCHIQ_CONNSTATE_DISCONNECTED,
    VCHIQ_CONNSTATE_CONNECTING,
    VCHIQ_CONNSTATE_CONNECTED,
    VCHIQ_CONNSTATE_PAUSING,
    VCHIQ_CONNSTATE_PAUSE_SENT,
    VCHIQ_CONNSTATE_PAUSED,
    VCHIQ_CONNSTATE_RESUMING,
    VCHIQ_CONNSTATE_PAUSE_TIMEOUT,
    VCHIQ_CONNSTATE_RESUME_TIMEOUT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vchiq_bulk_dir {
    VCHIQ_BULK_TRANSMIT,
    VCHIQ_BULK_RECEIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_bulk {
    pub mode: c_short,
    pub dir: c_short,
    pub cb_data: *mut c_void,
    pub cb_userdata: *mut void __user,
    pub waiter: *mut bulk_waiter,
    pub dma_addr: dma_addr_t,
    pub size: c_int,
    pub remote_data: *mut c_void,
    pub remote_size: c_int,
    pub actual: c_int,
    pub offset: *mut c_void,
    pub uoffset: *mut void __user,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_bulk_queue {
    pub /: *mut *mut int local_insert; / Where to insert the next local bulk,
    pub /: *mut *mut int remote_insert; / Where to insert the next remote bulk (master),
    pub /: *mut *mut int process; / Bulk to transfer next,
    pub /: *mut *mut int remote_notify; / Bulk to notify the remote client of next (mstr),
    pub /: *mut *mut int remove; / Bulk to notify the local client of, and remove, next,
    pub bulks: [vchiq_bulk; VCHIQ_NUM_SERVICE_BULKS],
}

//
// Remote events provide a way of presenting several virtual doorbells to a
// peer (ARM host to VPU) using only one physical doorbell. They can be thought
// of as a way for the peer to signal a semaphore, in this case implemented as
// a workqueue.
//
// Remote events remain signalled until acknowledged by the receiver, and they
// are non-counting. They are designed in such a way as to minimise the number
// of interrupts and avoid unnecessary waiting.
//
// A remote_event is as small data structures that live in shared memory. It
// comprises two booleans - armed and fired:
//
// The sender sets fired when they signal the receiver.
// If fired is set, the receiver has been signalled and need not wait.
// The receiver sets the armed field before they begin to wait.
// If armed is set, the receiver is waiting and wishes to be woken by interrupt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct remote_event {
    pub armed: c_int,
    pub fired: c_int,
    pub __unused: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_slot {
    pub data: [c_char; VCHIQ_SLOT_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_slot_info {
// Use two counters rather than one to avoid the need for a mutex.
    pub use_count: c_short,
    pub release_count: c_short,
}

//
// VCHIQ is a reliable connection-oriented datagram protocol.
//
// A VCHIQ service is equivalent to a TCP connection, except:
// + FOURCCs are used for the rendezvous, and port numbers are assigned at the
// time the connection is established.
// + There is less of a distinction between server and client sockets, the only
// difference being which end makes the first move.
// + For a multi-client server, the server creates new "listening" services as
// the existing one becomes connected - there is no need to specify the
// maximum number of clients up front.
// + Data transfer is reliable but packetized (messages have defined ends).
// + Messages can be either short (capable of fitting in a slot) and in-band,
// or copied between external buffers (bulk transfers).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_service {
    pub base: vchiq_service_base,
    pub handle: c_uint,
    pub ref_count: kref,
    pub rcu: rcu_head,
    pub srvstate: c_int,
    pub userdata): *mut *mut void (userdata_term)(void,
    pub localport: c_uint,
    pub remoteport: c_uint,
    pub public_fourcc: c_int,
    pub client_id: c_int,
    pub auto_close: c_char,
    pub sync: c_char,
    pub closing: c_char,
    pub trace: c_char,
    pub poll_flags: core::sync::atomic::AtomicI32,
    pub version: c_short,
    pub version_min: c_short,
    pub peer_version: c_short,
    pub state: *mut vchiq_state,
    pub instance: *mut vchiq_instance,
    pub service_use_count: c_int,
    pub bulk_tx: vchiq_bulk_queue,
    pub bulk_rx: vchiq_bulk_queue,
    pub remove_event: completion,
    pub bulk_remove_event: completion,
    pub bulk_mutex: mutex,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct service_stats_struct {
    pub quota_stalls: c_int,
    pub slot_stalls: c_int,
    pub bulk_stalls: c_int,
    pub error_count: c_int,
    pub ctrl_tx_count: c_int,
    pub ctrl_rx_count: c_int,
    pub bulk_tx_count: c_int,
    pub bulk_rx_count: c_int,
    pub bulk_aborted_count: c_int,
    pub ctrl_tx_bytes: u64,
    pub ctrl_rx_bytes: u64,
    pub bulk_tx_bytes: u64,
    pub bulk_rx_bytes: u64,
    pub stats: },
    pub msg_queue_read: c_int,
    pub msg_queue_write: c_int,
    pub msg_queue_pop: completion,
    pub msg_queue_push: completion,
    pub msg_queue: [*mut vchiq_header; VCHIQ_MAX_SLOTS],
}

//
// The quota information is outside struct vchiq_service so that it can
// be statically allocated, since for accounting reasons a service's slot
// usage is carried over between users of the same port number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_service_quota {
    pub slot_quota: c_ushort,
    pub slot_use_count: c_ushort,
    pub message_quota: c_ushort,
    pub message_use_count: c_ushort,
    pub quota_event: completion,
    pub previous_tx_index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_shared_state {
// A non-zero value here indicates that the content is valid.
    pub initialised: c_int,
// The first and last (inclusive) slots allocated to the owner.
    pub slot_first: c_int,
    pub slot_last: c_int,
// The slot allocated to synchronous messages from the owner.
    pub slot_sync: c_int,
//
// Signalling this event indicates that owner's slot handler thread
// should run.
//
    pub trigger: remote_event,
//
// Indicates the byte position within the stream where the next message
// will be written. The least significant bits are an index into the
// slot. The next bits are the index of the slot in slot_queue.
//
    pub tx_pos: c_int,
// This event should be signalled when a slot is recycled.
    pub recycle: remote_event,
// The slot_queue index where the next recycled slot will be written.
    pub slot_queue_recycle: c_int,
// This event should be signalled when a synchronous message is sent.
    pub sync_trigger: remote_event,
//
// This event should be signalled when a synchronous message has been
// released.
//
    pub sync_release: remote_event,
// A circular buffer of slot indexes.
    pub slot_queue: [c_int; VCHIQ_MAX_SLOTS_PER_SIDE],
// Debugging state
    pub debug: [c_int; DEBUG_MAX],
}

//
// vchiq_slot_zero describes the memory shared between the ARM host and the
// VideoCore VPU. The "master" and "slave" states are owned by the respective
// sides but visible to the other; the slots are shared, and the remaining
// fields are read-only.
//
// In the configuration used by this implementation, the memory is allocated
// by the host, the VPU is the master (the side which controls the DMA for bulk
// transfers), and the host is the slave.
//
// The ownership of slots changes with use:
// + When empty they are owned by the sender.
// + When partially filled they are shared with the receiver.
// + When completely full they are owned by the receiver.
// + When the receiver has finished processing the contents, they are recycled
// back to the sender.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_slot_zero {
    pub magic: c_int,
    pub version: c_short,
    pub version_min: c_short,
    pub slot_zero_size: c_int,
    pub slot_size: c_int,
    pub max_slots: c_int,
    pub max_slots_per_side: c_int,
    pub platform_data: [c_int; 2],
    pub master: vchiq_shared_state,
    pub slave: vchiq_shared_state,
    pub slots: [vchiq_slot_info; VCHIQ_MAX_SLOTS],
}

//
// This is the private runtime state used by each side. The same structure was
// originally used by both sides, but implementations have since diverged.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_state {
    pub dev: *mut device,
    pub id: c_int,
    pub initialised: c_int,
    pub conn_state: vchiq_connstate,
    pub version_common: c_short,
    pub local: *mut vchiq_shared_state,
    pub remote: *mut vchiq_shared_state,
    pub slot_data: *mut vchiq_slot,
    pub default_slot_quota: c_ushort,
    pub default_message_quota: c_ushort,
// Event indicating connect message received
    pub connect: completion,
// Mutex protecting services
    pub mutex: mutex,
    pub instance: *mut vchiq_instance,
// Processes all incoming messages which aren't synchronous
    pub slot_handler_thread: *mut task_struct,
//
// Slots which have been fully processed and released by the (peer)
// receiver are added to the receiver queue, which is asynchronously
// processed by the recycle thread.
//
    pub recycle_thread: *mut task_struct,
//
// Processes incoming synchronous messages
//
// The synchronous message channel is shared between all synchronous
// services, and provides a way for urgent messages to bypass
// potentially long queues of asynchronous messages in the normal slots.
//
// There can be only one outstanding synchronous message in
// each direction, and as a precious shared resource synchronous
// services should be used sparingly.
//
    pub sync_thread: *mut task_struct,
// Local implementation of the trigger remote event
    pub trigger_event: wait_queue_head_t,
// Local implementation of the recycle remote event
    pub recycle_event: wait_queue_head_t,
// Local implementation of the sync trigger remote event
    pub sync_trigger_event: wait_queue_head_t,
// Local implementation of the sync release remote event
    pub sync_release_event: wait_queue_head_t,
    pub tx_data: *mut c_char,
    pub rx_data: *mut c_char,
    pub rx_info: *mut vchiq_slot_info,
    pub slot_mutex: mutex,
    pub recycle_mutex: mutex,
    pub sync_mutex: mutex,
    pub msg_queue_spinlock: spinlock_t,
    pub bulk_waiter_spinlock: spinlock_t,
    pub quota_spinlock: spinlock_t,
//
// Indicates the byte position within the stream from where the next
// message will be read. The least significant bits are an index into
// the slot.The next bits are the index of the slot in
// remote->slot_queue.
//
    pub rx_pos: c_int,
//
// A cached copy of local->tx_pos. Only write to local->tx_pos, and read
// from remote->tx_pos.
//
    pub local_tx_pos: c_int,
// The slot_queue index of the slot to become available next.
    pub slot_queue_available: c_int,
// A flag to indicate if any poll has been requested
    pub poll_needed: c_int,
// Ths index of the previous slot used for data messages.
    pub previous_data_index: c_int,
// The number of slots occupied by data messages.
    pub data_use_count: c_ushort,
// The maximum number of slots to be occupied by data messages.
    pub data_quota: c_ushort,
// An array of bit sets indicating which services must be polled.
    pub poll_services: [core::sync::atomic::AtomicI32; BITSET_SIZE(VCHIQ_MAX_SERVICES)],
// The number of the first unused service
    pub unused_service: c_int,
// Signalled when a free slot becomes available.
    pub slot_available_event: completion,
// Signalled when a free data slot becomes available.
    pub data_quota_event: completion,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct state_stats_struct {
    pub slot_stalls: c_int,
    pub data_stalls: c_int,
    pub ctrl_tx_count: c_int,
    pub ctrl_rx_count: c_int,
    pub error_count: c_int,
    pub stats: },
    pub services: [*mut vchiq_service __rcu; VCHIQ_MAX_SERVICES],
    pub service_quotas: [vchiq_service_quota; VCHIQ_MAX_SERVICES],
    pub slot_info: [vchiq_slot_info; VCHIQ_MAX_SLOTS],
    pub platform_state: *mut opaque_platform_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pagelist {
    pub length: u32,
    pub type: u16,
    pub offset: u16,
    pub number: *mut *mut u32 addrs[1]; / N.B. 12 LSBs hold the,
// of following pages at consecutive
// addresses.
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_pagelist_info {
    pub pagelist: *mut pagelist,
    pub pagelist_buffer_size: usize,
    pub dma_addr: dma_addr_t,
    pub dma_dir: dma_data_direction,
    pub num_pages: c_uint,
    pub pages_need_release: c_uint,
    pub pages: *mut page,
    pub scatterlist: *mut scatterlist,
    pub scatterlist_mapped: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bulk_waiter {
    pub bulk: *mut vchiq_bulk,
    pub event: completion,
    pub actual: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vchiq_config {
    pub max_msg_size: c_uint,
    pub it: *mut *mut unsigned int bulk_threshold; / The message size above which,
// is better to use a bulk transfer
// (<= max_msg_size)
//
    pub max_outstanding_bulks: c_uint,
    pub max_services: c_uint,
    pub /: *mut *mut short version; / The version of VCHIQ,
    pub /: *mut *mut short version_min; / The minimum compatible version of VCHIQ,
}

extern "C" {
    pub fn vchiq_dump_platform_state(f: *mut seq_file);
}
extern "C" {
    pub fn vchiq_dump_platform_instances(state: *mut vchiq_state, f: *mut seq_file);
}
extern "C" {
    pub fn vchiq_dump_platform_service_state(f: *mut seq_file, service: *mut vchiq_service);
}
extern "C" {
    pub fn vchiq_use_service_internal(service: *mut vchiq_service) -> c_int;
}
extern "C" {
    pub fn vchiq_release_service_internal(service: *mut vchiq_service) -> c_int;
}
extern "C" {
    pub fn vchiq_on_remote_use(state: *mut vchiq_state);
}
extern "C" {
    pub fn vchiq_on_remote_release(state: *mut vchiq_state);
}
extern "C" {
    pub fn vchiq_platform_init_state(state: *mut vchiq_state) -> c_int;
}
extern "C" {
    pub fn vchiq_check_service(service: *mut vchiq_service) -> c_int;
}
extern "C" {
    pub fn vchiq_send_remote_use(state: *mut vchiq_state) -> c_int;
}
extern "C" {
    pub fn vchiq_send_remote_use_active(state: *mut vchiq_state) -> c_int;
}
extern "C" {
    pub fn vchiq_set_conn_state(state: *mut vchiq_state, newstate: vchiq_connstate);
}
extern "C" {
    pub fn vchiq_remove_service(instance: *mut vchiq_instance, service: c_uint) -> c_int;
}
extern "C" {
    pub fn vchiq_get_client_id(instance: *mut vchiq_instance, service: c_uint) -> c_int;
}
extern "C" {
    pub fn vchiq_get_config(config: *mut vchiq_config);
}
