//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/virtio_vsock.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_vsock_skb_cb {
    pub reply: bool,
    pub tap_delivered: bool,
    pub offset: u32,
}

extern "C" {
    pub fn __virtio_vsock_alloc_skb_with_frags(_arg: size, _arg: 0, _arg: mask) -> return;
}
extern "C" {
    pub fn virtio_vsock_alloc_linear_skb(_arg: size, _arg: mask) -> return;
}
// Dimension the RX SKB so that the entire thing fits exactly into
// a single 4KiB page. This avoids wasting memory due to alloc_skb()
// rounding up to the next page order and also means that we
// don't leave higher-order pages sitting around in the RX queue.
//

pub const VIRTIO_VSOCK_MAX_BUF_SIZE: c_uint = 0xFFFFFFFFUL;

// Per-socket state (accessed via vsk->trans)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_vsock_sock {
    pub vsk: *mut vsock_sock,
    pub tx_lock: spinlock_t,
    pub rx_lock: spinlock_t,
// Protected by tx_lock
    pub tx_cnt: u32,
    pub peer_fwd_cnt: u32,
    pub peer_buf_alloc: u32,
    pub bytes_unsent: usize,
// Protected by rx_lock
    pub fwd_cnt: u32,
    pub last_fwd_cnt: u32,
    pub rx_bytes: u32,
    pub buf_alloc: u32,
    pub buf_used: u32,
    pub rx_queue: sk_buff_head,
    pub msg_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_vsock_pkt_info {
    pub remote_port: u32 remote_cid,,
    pub vsk: *mut vsock_sock,
    pub msg: *mut msghdr,
    pub net: *mut net,
    pub pkt_len: u32,
    pub type: u16,
    pub op: u16,
    pub flags: u32,
    pub reply: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_transport {
// This must be the first field
    pub transport: vsock_transport,
// Takes ownership of the packet
    pub net): *mut *mut *mut int (send_pkt)(struct sk_buff skb, struct net,
// Used in MSG_ZEROCOPY mode. Checks, that provided data
// (number of buffers) could be transmitted with zerocopy
// mode. If this callback is not implemented for the current
// transport - this means that this transport doesn't need
// extra checks and can perform zerocopy transmission by
// default.
//
    pub bufs_num): *mut *mut bool (can_msgzerocopy)(int,
}

extern "C" {
    pub fn virtio_transport_stream_has_data(vsk: *mut vsock_sock) -> i64;
}
extern "C" {
    pub fn virtio_transport_stream_has_space(vsk: *mut vsock_sock) -> i64;
}
extern "C" {
    pub fn virtio_transport_seqpacket_has_data(vsk: *mut vsock_sock) -> u32;
}
extern "C" {
    pub fn virtio_transport_unsent_bytes(vsk: *mut vsock_sock) -> isize;
}
extern "C" {
    pub fn virtio_transport_notify_buffer_size(vsk: *mut vsock_sock, val: *mut u64);
}
extern "C" {
    pub fn virtio_transport_stream_rcvhiwat(vsk: *mut vsock_sock) -> u64;
}
extern "C" {
    pub fn virtio_transport_stream_is_active(vsk: *mut vsock_sock) -> bool;
}
extern "C" {
    pub fn virtio_transport_stream_allow(vsk: *mut vsock_sock, cid: u32, port: u32) -> bool;
}
extern "C" {
    pub fn virtio_transport_dgram_allow(vsk: *mut vsock_sock, cid: u32, port: u32) -> bool;
}
extern "C" {
    pub fn virtio_transport_connect(vsk: *mut vsock_sock) -> c_int;
}
extern "C" {
    pub fn virtio_transport_shutdown(vsk: *mut vsock_sock, mode: c_int) -> c_int;
}
extern "C" {
    pub fn virtio_transport_release(vsk: *mut vsock_sock);
}
extern "C" {
    pub fn virtio_transport_destruct(vsk: *mut vsock_sock);
}
extern "C" {
    pub fn virtio_transport_inc_tx_pkt(vvs: *mut virtio_vsock_sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn virtio_transport_get_credit(vvs: *mut virtio_vsock_sock, wanted: u32) -> u32;
}
extern "C" {
    pub fn virtio_transport_put_credit(vvs: *mut virtio_vsock_sock, credit: u32);
}
extern "C" {
    pub fn virtio_transport_deliver_tap_pkt(skb: *mut sk_buff);
}
extern "C" {
    pub fn virtio_transport_purge_skbs(vsk: *mut c_void, list: *mut sk_buff_head) -> c_int;
}
extern "C" {
    pub fn virtio_transport_read_skb(vsk: *mut vsock_sock, read_actor: skb_read_actor_t) -> c_int;
}
extern "C" {
    pub fn virtio_transport_notify_set_rcvlowat(vsk: *mut vsock_sock, val: c_int) -> c_int;
}
