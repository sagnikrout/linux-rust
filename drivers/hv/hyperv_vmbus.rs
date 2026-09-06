//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hv/hyperv_vmbus.h
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

//
// Timeout for services such as KVP and fcopy.
//
pub const HV_UTIL_TIMEOUT: c_int = 30;
//
// Timeout for guest-host handshake for services.
//
pub const HV_UTIL_NEGO_TIMEOUT: c_int = 55;
extern "C" {
    pub fn vmbus_isr();
}
// Definitions for the monitored notification facility
#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_monitor_trigger_group {
    pub as_uint64: u64,
    pub pending: u32,
    pub armed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_monitor_parameter {
    pub connectionid: hv_connection_id,
    pub flagnumber: u16,
    pub rsvdz: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hv_monitor_trigger_state {
    pub asu32: u32,
    pub group_enable:4: u32,
    pub rsvdz:28: u32,
}

// struct hv_monitor_page Layout
// ------------------------------------------------------
// | 0   | TriggerState (4 bytes) | Rsvd1 (4 bytes)     |
// | 8   | TriggerGroup[0]                              |
// | 10  | TriggerGroup[1]                              |
// | 18  | TriggerGroup[2]                              |
// | 20  | TriggerGroup[3]                              |
// | 28  | Rsvd2[0]                                     |
// | 30  | Rsvd2[1]                                     |
// | 38  | Rsvd2[2]                                     |
// | 40  | NextCheckTime[0][0]    | NextCheckTime[0][1] |
// | ...                                                |
// | 240 | Latency[0][0..3]                             |
// | 340 | Rsvz3[0]                                     |
// | 440 | Parameter[0][0]                              |
// | 448 | Parameter[0][1]                              |
// | ...                                                |
// | 840 | Rsvd4[0]                                     |
// ------------------------------------------------------
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_monitor_page {
    pub trigger_state: hv_monitor_trigger_state,
    pub rsvdz1: u32,
    pub trigger_group: [hv_monitor_trigger_group; 4],
    pub rsvdz2: [u64; 3],
    pub next_checktime: [i32; 4][32],
    pub latency: [u16; 4][32],
    pub rsvdz3: [u64; 32],
    pub parameter: [hv_monitor_parameter; 4][32],
    pub rsvdz4: [u8; 1984],
}

// Definition of the hv_post_message hypercall input structure.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_input_post_message {
    pub connectionid: hv_connection_id,
    pub reserved: u32,
    pub message_type: u32,
    pub payload_size: u32,
    pub payload: [u64; HV_MESSAGE_PAYLOAD_QWORD_COUNT],
}

// VTL2 redirect connection ID for INITIATE_CONTACT.
//
// Per cpu state for channel handling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_per_cpu_context {
//
// SynIC pages for communicating with the host.
//
// These pages are accessible to the host partition and the hypervisor.
// They may be used for exchanging data with the host partition and the
// hypervisor even when they aren't trusted yet the guest partition
// must be prepared to handle the malicious behavior.
//
    pub hyp_synic_message_page: *mut c_void,
    pub hyp_synic_event_page: *mut c_void,
//
// SynIC pages for communicating with the paravisor.
//
// These pages may be accessed from within the guest partition only in
// CoCo VMs. Neither the host partition nor the hypervisor can access
// these pages in that case; they are used for exchanging data with the
// paravisor.
//
    pub para_synic_message_page: *mut c_void,
    pub para_synic_event_page: *mut c_void,
//
// The page is only used in hv_post_message() for a TDX VM (with the
// paravisor) to post a messages to Hyper-V: when such a VM calls
// HVCALL_POST_MESSAGE, it can't use the hyperv_pcpu_input_arg (which
// is encrypted in such a VM) as the hypercall input page, because
// the input page for HVCALL_POST_MESSAGE must be decrypted in such a
// VM, so post_msg_page (which is decrypted in hv_synic_alloc()) is
// introduced for this purpose. See hyperv_init() for more comments.
//
    pub post_msg_page: *mut c_void,
//
// Starting with win8, we can take channel interrupts on any CPU;
// we will manage the tasklet that handles events messages on a per CPU
// basis.
//
    pub msg_dpc: tasklet_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_context {
// We only support running on top of Hyper-V
// So at this point this really can only contain the Hyper-V ID
//
    pub guestid: u64,
    pub cpu_context: *mut hv_per_cpu_context __percpu,
//
// To manage allocations in a NUMA node.
// Array indexed by numa node ID.
//
    pub hv_numa_map: *mut cpumask,
}

// Hv Interface
extern "C" {
    pub fn hv_init() -> c_int;
}
extern "C" {
    pub fn hv_synic_alloc() -> c_int;
}
extern "C" {
    pub fn hv_synic_free();
}
extern "C" {
    pub fn hv_hyp_synic_enable_regs(cpu: c_uint);
}
extern "C" {
    pub fn hv_synic_init(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn hv_hyp_synic_disable_regs(cpu: c_uint);
}
extern "C" {
    pub fn hv_synic_cleanup(cpu: c_uint) -> c_int;
}
// Interface
extern "C" {
    pub fn hv_ringbuffer_pre_init(channel: *mut vmbus_channel);
}
extern "C" {
    pub fn hv_ringbuffer_cleanup(ring_info: *mut hv_ring_buffer_info);
}
//
// The Maximum number of channels (16384) is determined by the size of the
// interrupt page, which is HV_HYP_PAGE_SIZE. 1/2 of HV_HYP_PAGE_SIZE is to
// send endpoint interrupts, and the other is to receive endpoint interrupts.
//

// The value here must be in multiple of 32
pub const MAX_NUM_CHANNELS_SUPPORTED: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmbus_connect_state {
    DISCONNECTED,
    CONNECTING,
    CONNECTED,
    DISCONNECTING
}

//
// The CPU that Hyper-V will interrupt for VMBUS messages, such as
// CHANNELMSG_OFFERCHANNEL and CHANNELMSG_RESCIND_CHANNELOFFER.
//
pub const VMBUS_CONNECT_CPU: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_connection {
    pub msg_conn_id: u32,
    pub offer_in_progress: core::sync::atomic::AtomicI32,
    pub conn_state: vmbus_connect_state,
    pub next_gpadl_handle: core::sync::atomic::AtomicI32,
    pub unload_event: completion,
//
// Represents channel interrupts. Each bit position represents a
// channel.  When a channel sends an interrupt via VMBUS, it finds its
// bit in the sendInterruptPage, set it and calls Hv to generate a port
// event. The other end receives the port event and parse the
// recvInterruptPage to see which bit is set
//
    pub int_page: *mut c_void,
    pub send_int_page: *mut c_void,
    pub recv_int_page: *mut c_void,
//
// 2 pages - 1st page for parent->child notification and 2nd
// is child->parent notification
//
    pub monitor_pages: [*mut hv_monitor_page; 2],
    pub chn_msg_list: list_head,
    pub channelmsg_lock: spinlock_t,
// List of channels
    pub chn_list: list_head,
    pub channel_mutex: mutex,
// Array of channel pointers, indexed by relid
    pub channels: *mut vmbus_channel,
    pub relid_hiwater: u32,
//
// An offer message is handled first on the work_queue, and then
// is further handled on handle_primary_chan_wq or
// handle_sub_chan_wq.
//
    pub work_queue: *mut workqueue_struct,
    pub handle_primary_chan_wq: *mut workqueue_struct,
    pub handle_sub_chan_wq: *mut workqueue_struct,
    pub rescind_work_queue: *mut workqueue_struct,
//
// On suspension of the vmbus, the accumulated offer messages
// must be dropped.
//
    pub ignore_any_offer_msg: bool,
//
// The number of sub-channels and hv_sock channels that should be
// cleaned up upon suspend: sub-channels will be re-created upon
// resume, and hv_sock channels should not survive suspend.
//
    pub nr_chan_close_on_suspend: core::sync::atomic::AtomicI32,
//
// vmbus_bus_suspend() waits for "nr_chan_close_on_suspend" to
// drop to zero.
//
    pub ready_for_suspend_event: completion,
//
// Completed once the host has offered all boot-time channels.
// Note that some channels may still be under process on a workqueue.
//
    pub all_offers_delivered_event: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_msginfo {
// Bookkeeping stuff
    pub msglist_entry: list_head,
// The message itself
    pub msg: [c_uchar; ],
}

extern "C" {
    pub fn vmbus_negotiate_version(msginfo: *mut vmbus_channel_msginfo, version: u32) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmbus_message_handler_type {
// The related handler can sleep.
    VMHT_BLOCKING = 0,

// The related handler must NOT sleep.
    VMHT_NON_BLOCKING = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vmbus_channel_message_table_entry {
    pub message_type: vmbus_channel_message_type,
    pub handler_type: vmbus_message_handler_type,
    pub msg): *mut *mut void (message_handler)(struct vmbus_channel_message_header,
    pub min_payload_len: u32,
}

// General vmbus interface
extern "C" {
    pub fn vmbus_is_confidential() -> bool;
}

// Free the message slot and signal end-of-message if required
//
// On crash we're reading some other CPU's message page and we need
// to be careful: this other CPU may already had cleared the header
// and the host may already had delivered some other message there.
// In case we blindly write msg->header.message_type we're going
// to lose it. We can still lose a message of the same type but
// we count on the fact that there can only be one
// CHANNELMSG_UNLOAD_RESPONSE and we don't care about other messages
// on crash.
//
// The cmpxchg() above does an implicit memory barrier to
// ensure the write to MessageType (ie set to
// HVMSG_NONE) happens before we read the
// MessagePending and EOMing. Otherwise, the EOMing
// will not deliver any more messages since there is
// no empty slot
//
// This will cause message queue rescan to
// possibly deliver another msg from the
// hypervisor
//

extern "C" {
    pub fn vmbus_device_register(child_device_obj: *mut hv_device) -> c_int;
}
extern "C" {
    pub fn vmbus_device_unregister(device_obj: *mut hv_device);
}
extern "C" {
    pub fn vmbus_remove_channel_attr_group(channel: *mut vmbus_channel);
}
extern "C" {
    pub fn vmbus_channel_map_relid(channel: *mut vmbus_channel);
}
extern "C" {
    pub fn vmbus_channel_unmap_relid(channel: *mut vmbus_channel);
}
extern "C" {
    pub fn vmbus_free_channels();
}
// Connection interface
extern "C" {
    pub fn vmbus_connect() -> c_int;
}
extern "C" {
    pub fn vmbus_disconnect();
}
extern "C" {
    pub fn vmbus_post_msg(buffer: *mut c_void, buflen: usize, can_sleep: bool) -> c_int;
}
extern "C" {
    pub fn vmbus_on_event(data: c_ulong);
}
extern "C" {
    pub fn vmbus_on_msg_dpc(data: c_ulong);
}
extern "C" {
    pub fn hv_kvp_init(srv: *mut hv_util_service) -> c_int;
}
extern "C" {
    pub fn hv_kvp_init_transport() -> c_int;
}
extern "C" {
    pub fn hv_kvp_deinit();
}
extern "C" {
    pub fn hv_kvp_pre_suspend() -> c_int;
}
extern "C" {
    pub fn hv_kvp_pre_resume() -> c_int;
}
extern "C" {
    pub fn hv_kvp_onchannelcallback(context: *mut c_void);
}
extern "C" {
    pub fn hv_vss_init(srv: *mut hv_util_service) -> c_int;
}
extern "C" {
    pub fn hv_vss_init_transport() -> c_int;
}
extern "C" {
    pub fn hv_vss_deinit();
}
extern "C" {
    pub fn hv_vss_pre_suspend() -> c_int;
}
extern "C" {
    pub fn hv_vss_pre_resume() -> c_int;
}
extern "C" {
    pub fn hv_vss_onchannelcallback(context: *mut c_void);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hvutil_device_state {
    HVUTIL_DEVICE_INIT = 0,  /* driver is loaded, waiting for userspace */
    HVUTIL_READY,            /* userspace is registered */
    HVUTIL_HOSTMSG_RECEIVED, /* message from the host was received */
    HVUTIL_USERSPACE_REQ,    /* request to userspace was sent */
    HVUTIL_USERSPACE_RECV,   /* reply from userspace was received */
    HVUTIL_DEVICE_DYING,     /* driver unload is in progress */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum delay {
    INTERRUPT_DELAY = 0,
    MESSAGE_DELAY   = 1,
}

//
// List additions/deletions as well as updates of the target CPUs are
// protected by channel_mutex.
//

extern "C" {
    pub fn hv_debug_add_dev_dir(dev: *mut hv_device) -> c_int;
}
extern "C" {
    pub fn hv_debug_rm_dev_dir(dev: *mut hv_device);
}
extern "C" {
    pub fn hv_debug_rm_all_dir();
}
extern "C" {
    pub fn hv_debug_init() -> c_int;
}
extern "C" {
    pub fn hv_debug_delay_test(channel: *mut vmbus_channel, delay_type: delay);
}

// Create and remove sysfs entry for memory mapped ring buffers for a channel
extern "C" {
    pub fn hv_remove_ring_sysfs(channel: *mut vmbus_channel) -> c_int;
}
