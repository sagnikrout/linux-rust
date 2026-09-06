//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/xen-netback/common.h
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


//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

pub type pending_ring_idx_t = c_uint;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pending_tx_info {
    pub /: *mut *mut xen_netif_tx_request req; / tx request,
    pub extra_count: c_uint,
// Callback data for released SKBs. The callback is always
// xenvif_zerocopy_callback, desc contains the pending_idx, which is
// also an index in pending_tx_info array. It is initialized in
// xenvif_alloc and it never changes.
// skb_shinfo(skb)->destructor_arg points to the first mapped slot's
// callback_struct in this array of struct pending_tx_info's, then ctx
// to the next, or NULL if there is no more slot for this skb.
// ubuf_to_vif is a helper which finds the struct xenvif from a pointer
// to this field.
//
    pub callback_struct: ubuf_info_msgzc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_rx_meta {
    pub id: c_int,
    pub size: c_int,
    pub gso_type: c_int,
    pub gso_size: c_int,
}

// Discriminate from any valid pending_idx value.
pub const INVALID_PENDING_IDX: c_uint = 0xFFFF;

// The maximum number of frags is derived from the size of a grant (same
// as a Xen page size for now).
//

// To avoid confusion, we define XEN_NETBK_LEGACY_SLOTS_MAX indicating
// the maximum slots a valid packet can use. Now this value is defined
// to be XEN_NETIF_NR_SLOTS_MIN, which is supposed to be supported by
// all backend.
//

// Queue name is interface name with "-qNNN" appended

// IRQ name is queue name with "-tx" or "-rx" appended

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_stats {
// Stats fields to be updated per-queue.
// A subset of struct net_device_stats that contains only the
// fields that are updated in netback.c for each queue.
//
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
// Additional stats used by xenvif
    pub rx_gso_checksum_fixup: c_ulong,
    pub tx_zerocopy_sent: c_ulong,
    pub tx_zerocopy_success: c_ulong,
    pub tx_zerocopy_fail: c_ulong,
    pub tx_frag_overflow: c_ulong,
}

pub const COPY_BATCH_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_copy_state {
    pub op: [gnttab_copy; COPY_BATCH_SIZE],
    pub idx: [RING_IDX; COPY_BATCH_SIZE],
    pub num: c_uint,
    pub completed: *mut sk_buff_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_queue {
    pub /: *mut *mut unsigned int id; / Queue ID, 0-based,
    pub /: *mut *mut char name[QUEUE_NAME_SIZE]; / DEVNAME-qN,
    pub /: *mut *mut *mut xenvif vif; / Parent VIF,
//
// TX/RX common EOI handling.
// When feature-split-event-channels = 0, interrupt handler sets
// NETBK_COMMON_EOI, otherwise NETBK_RX_EOI and NETBK_TX_EOI are set
// by the RX and TX interrupt handlers.
// RX and TX handler threads will issue an EOI when either
// NETBK_COMMON_EOI or their specific bits (NETBK_RX_EOI or
// NETBK_TX_EOI) are set and they will reset those bits.
//
    pub eoi_pending: core::sync::atomic::AtomicI32,
pub const NETBK_RX_EOI: c_uint = 0x01;
pub const NETBK_TX_EOI: c_uint = 0x02;
pub const NETBK_COMMON_EOI: c_uint = 0x04;
// Use NAPI for guest TX
    pub napi: napi_struct,
// When feature-split-event-channels = 0, tx_irq = rx_irq.
    pub tx_irq: c_uint,
// Only used when feature-split-event-channels = 1
    pub /: *mut *mut char tx_irq_name[IRQ_NAME_SIZE]; / DEVNAME-qN-tx,
    pub tx: xen_netif_tx_back_ring,
    pub tx_queue: sk_buff_head,
    pub mmap_pages: [*mut page; MAX_PENDING_REQS],
    pub pending_prod: pending_ring_idx_t,
    pub pending_cons: pending_ring_idx_t,
    pub pending_ring: [u16; MAX_PENDING_REQS],
    pub pending_tx_info: [pending_tx_info; MAX_PENDING_REQS],
    pub grant_tx_handle: [grant_handle_t; MAX_PENDING_REQS],
    pub MAX_PENDING_REQS]: *mut *mut gnttab_copy tx_copy_ops[2,
    pub tx_map_ops: [gnttab_map_grant_ref; MAX_PENDING_REQS],
    pub tx_unmap_ops: [gnttab_unmap_grant_ref; MAX_PENDING_REQS],
// passed to gnttab_[un]map_refs with pages under (un)mapping
    pub pages_to_map: [*mut page; MAX_PENDING_REQS],
    pub pages_to_unmap: [*mut page; MAX_PENDING_REQS],
// This prevents zerocopy callbacks  to race over dealloc_ring
    pub callback_lock: spinlock_t,
// This prevents dealloc thread and NAPI instance to race over response
// creation and pending_ring in xenvif_idx_release. In xenvif_tx_err
// it only protect response creation
//
    pub response_lock: spinlock_t,
    pub dealloc_prod: pending_ring_idx_t,
    pub dealloc_cons: pending_ring_idx_t,
    pub dealloc_ring: [u16; MAX_PENDING_REQS],
    pub dealloc_task: *mut task_struct,
    pub dealloc_wq: wait_queue_head_t,
    pub inflight_packets: core::sync::atomic::AtomicI32,
// Use kthread for guest RX
    pub task: *mut task_struct,
    pub wq: wait_queue_head_t,
// When feature-split-event-channels = 0, tx_irq = rx_irq.
    pub rx_irq: c_uint,
// Only used when feature-split-event-channels = 1
    pub /: *mut *mut char rx_irq_name[IRQ_NAME_SIZE]; / DEVNAME-qN-rx,
    pub rx: xen_netif_rx_back_ring,
    pub rx_queue: sk_buff_head,
    pub rx_queue_max: c_uint,
    pub rx_queue_len: c_uint,
    pub last_rx_time: c_ulong,
    pub rx_slots_needed: c_uint,
    pub stalled: bool,
    pub rx_copy: xenvif_copy_state,
// Transmit shaping: allow 'credit_bytes' every 'credit_usec'.
    pub credit_bytes: c_ulong,
    pub credit_usec: c_ulong,
    pub remaining_credit: c_ulong,
    pub credit_timeout: timer_list,
    pub credit_window_start: u64,
    pub rate_limited: bool,
// Statistics
    pub stats: xenvif_stats,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum state_bit_shift {
// This bit marks that the vif is connected
    VIF_STATUS_CONNECTED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_mcast_addr {
    pub entry: list_head,
    pub rcu: rcu_head,
    pub addr: [u8; 6],
}

pub const XEN_NETBK_MCAST_MAX: c_int = 64;
pub const XEN_NETBK_MAX_HASH_KEY_SIZE: c_int = 40;
pub const XEN_NETBK_MAX_HASH_MAPPING_SIZE: c_int = 128;
pub const XEN_NETBK_HASH_TAG_SIZE: c_int = 40;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_hash_cache_entry {
    pub link: list_head,
    pub rcu: rcu_head,
    pub tag: [u8; XEN_NETBK_HASH_TAG_SIZE],
    pub len: c_uint,
    pub val: u32,
    pub seq: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_hash_cache {
    pub lock: spinlock_t,
    pub list: list_head,
    pub count: c_uint,
    pub seq: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_hash {
    pub alg: c_uint,
    pub flags: u32,
    pub mapping_sel: bool,
    pub key: [u8; XEN_NETBK_MAX_HASH_KEY_SIZE],
    pub mapping: [u32; 2][XEN_NETBK_MAX_HASH_MAPPING_SIZE],
    pub size: c_uint,
    pub cache: xenvif_hash_cache,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct backend_info {
    pub dev: *mut xenbus_device,
    pub vif: *mut xenvif,
// This is the state that will be reflected in xenstore when any
// active hotplug script completes.
//
    pub state: xenbus_state,
    pub frontend_state: xenbus_state,
    pub hotplug_status_watch: xenbus_watch,
    pub have_hotplug_status_watch:1: u8,
    pub hotplug_script: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif {
// Unique identifier for this interface.
    pub domid: domid_t,
    pub handle: c_uint,
    pub fe_dev_addr: [u8; 6],
    pub fe_mcast_addr: list_head,
    pub fe_mcast_count: c_uint,
// Frontend feature information.
    pub gso_mask: c_int,
    pub can_sg:1: u8,
    pub ip_csum:1: u8,
    pub ipv6_csum:1: u8,
    pub multicast_control:1: u8,
// headroom requested by xen-netfront
    pub xdp_headroom: u16,
// Is this interface disabled? True when backend discovers
// frontend is rogue.
//
    pub disabled: bool,
    pub status: c_ulong,
    pub drain_timeout: c_ulong,
    pub stall_timeout: c_ulong,
// Queues
    pub queues: *mut xenvif_queue,
    pub /: *mut *mut unsigned int num_queues; / active queues, resource allocated,
    pub stalled_queues: c_uint,
    pub hash: xenvif_hash,
    pub credit_watch: xenbus_watch,
    pub mcast_ctrl_watch: xenbus_watch,
    pub be: *mut backend_info,
    pub lock: spinlock_t,

    pub xenvif_dbg_root: *mut dentry,

    pub ctrl: xen_netif_ctrl_back_ring,
    pub ctrl_irq: c_uint,
// Miscellaneous private stuff.
    pub dev: *mut net_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xenvif_rx_cb {
    pub expires: c_ulong,
    pub meta_slots_used: c_int,
}

extern "C" {
    pub fn to_xenbus_device(_arg: vif->dev->dev.parent) -> return;
}
extern "C" {
    pub fn xenvif_tx_credit_callback(t: *mut timer_list);
}
extern "C" {
    pub fn xenvif_init_queue(queue: *mut xenvif_queue) -> c_int;
}
extern "C" {
    pub fn xenvif_deinit_queue(queue: *mut xenvif_queue);
}
extern "C" {
    pub fn xenvif_disconnect_data(vif: *mut xenvif);
}
extern "C" {
    pub fn xenvif_disconnect_ctrl(vif: *mut xenvif);
}
extern "C" {
    pub fn xenvif_free(vif: *mut xenvif);
}
extern "C" {
    pub fn xenvif_xenbus_init() -> c_int;
}
extern "C" {
    pub fn xenvif_xenbus_fini();
}
// (Un)Map communication rings.
extern "C" {
    pub fn xenvif_unmap_frontend_data_rings(queue: *mut xenvif_queue);
}
// Check for SKBs from frontend and schedule backend processing
extern "C" {
    pub fn xenvif_napi_schedule_or_enable_events(queue: *mut xenvif_queue);
}
// Prevent the device from generating any further traffic.
extern "C" {
    pub fn xenvif_carrier_off(vif: *mut xenvif);
}
extern "C" {
    pub fn xenvif_tx_action(queue: *mut xenvif_queue, budget: c_int) -> c_int;
}
extern "C" {
    pub fn xenvif_kthread_guest_rx(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn xenvif_kick_thread(queue: *mut xenvif_queue);
}
extern "C" {
    pub fn xenvif_dealloc_kthread(data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn xenvif_ctrl_irq_fn(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn xenvif_have_rx_work(queue: *mut xenvif_queue, test_kthread: bool) -> bool;
}
extern "C" {
    pub fn xenvif_rx_queue_tail(queue: *mut xenvif_queue, skb: *mut sk_buff) -> bool;
}
extern "C" {
    pub fn xenvif_carrier_on(vif: *mut xenvif);
}
// Callbacks from stack when TX packet can be released
extern "C" {
    pub fn xenvif_interrupt(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}

extern "C" {
    pub fn xenvif_skb_zerocopy_complete(queue: *mut xenvif_queue);
}
// Multicast control
extern "C" {
    pub fn xenvif_mcast_match(vif: *mut xenvif, addr: *const u8) -> bool;
}
extern "C" {
    pub fn xenvif_mcast_addr_list_free(vif: *mut xenvif);
}
// Hash
extern "C" {
    pub fn xenvif_init_hash(vif: *mut xenvif);
}
extern "C" {
    pub fn xenvif_deinit_hash(vif: *mut xenvif);
}
extern "C" {
    pub fn xenvif_set_hash_alg(vif: *mut xenvif, alg: u32) -> u32;
}
extern "C" {
    pub fn xenvif_get_hash_flags(vif: *mut xenvif, flags: *mut u32) -> u32;
}
extern "C" {
    pub fn xenvif_set_hash_flags(vif: *mut xenvif, flags: u32) -> u32;
}
extern "C" {
    pub fn xenvif_set_hash_key(vif: *mut xenvif, gref: u32, len: u32) -> u32;
}
extern "C" {
    pub fn xenvif_set_hash_mapping_size(vif: *mut xenvif, size: u32) -> u32;
}
extern "C" {
    pub fn xenvif_set_skb_hash(vif: *mut xenvif, skb: *mut sk_buff);
}

extern "C" {
    pub fn xenvif_dump_hash_info(vif: *mut xenvif, m: *mut seq_file);
}

