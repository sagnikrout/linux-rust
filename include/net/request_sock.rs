//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/request_sock.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// NET		Generic infrastructure for Network protocols.
//
// Definitions for request_sock
//
// Authors:	Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//
// From code originally in include/net/tcp.h
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_sock_ops {
    pub family: c_int,
    pub obj_size: c_uint,
    pub slab: *mut kmem_cache,
    pub slab_name: *mut c_char,
    pub req): *mut request_sock,
    pub reason): sk_rst_reason,
    pub req): *mut *mut void (destructor)(struct request_sock,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct saved_syn {
    pub mac_hdrlen: u32,
    pub network_hdrlen: u32,
    pub tcp_hdrlen: u32,
    pub data: [u8; ],
}

// struct request_sock - mini sock to represent a connection request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_sock {
    pub __req_common: sock_common,

    pub dl_next: *mut request_sock,
    pub mss: u16,
    pub /: *mut *mut u8 num_retrans; / number of retransmits,
    pub if: *mut *mut u8 syncookie:1; / True,
// 1) tcpopts needs to be encoded in
// TS of SYN+ACK
// 2) ACK is validated by BPF kfunc.
//
    pub /: *mut *mut u8 num_timeout:7; / number of timeouts,
    pub ts_recent: u32,
    pub rsk_timer: timer_list,
    pub rsk_ops: *const request_sock_ops,
    pub sk: *mut sock,
    pub saved_syn: *mut saved_syn,
    pub secid: u32,
    pub peer_secid: u32,
    pub timeout: u32,
}

//
// skb_steal_sock - steal a socket from an sk_buff
// @skb: sk_buff to steal the socket from
// @refcounted: is set to true if the socket is reference-counted
// @prefetched: is set to true if the socket was assigned from bpf
//
// prefetched = false;
// refcounted = false;
// prefetched = skb_sk_is_prefetched(skb);

// refcounted = false;

// refcounted = sk_is_refcounted(sk);
// refcounted = true;
extern "C" {
    pub fn __reqsk_free(req: *mut request_sock);
}
//
// For a TCP Fast Open listener -
// lock - protects the access to all the reqsk, which is co-owned by
// the listener and the child socket.
// qlen - pending TFO requests (still in TCP_SYN_RECV).
// max_qlen - max TFO reqs allowed before TFO is disabled.
//
// XXX (TFO) - ideally these fields can be made as part of "listen_sock"
// structure above. But there is some implementation difficulty due to
// listen_sock being part of request_sock_queue hence will be freed when
// a listener is stopped. But TFO related fields may continue to be
// accessed even after a listener is closed, until its sk_refcnt drops
// to 0 implying no more outstanding TFO reqs. One solution is to keep
// listen_opt around until	sk_refcnt drops to 0. But there is some other
// complexity that needs to be resolved. E.g., a listener can be disabled
// temporarily through shutdown()->tcp_disconnect(), and re-enabled later.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fastopen_queue {
    pub /: *mut *mut *mut request_sock rskq_rst_head; / Keep track of past TFO,
    pub RST.: *mut *mut *mut request_sock rskq_rst_tail; / requests that caused,
// This is part of the defense
// against spoofing attack.
//
    pub lock: spinlock_t,
    pub /: *mut *mut int qlen; / # of pending (TCP_SYN_RECV) reqs,
    pub /: *mut *mut int max_qlen; / != 0 iff TFO is currently enabled,
    pub /: *mut *mut *mut tcp_fastopen_context __rcu ctx; / cipher context for cookie,
}

// struct request_sock_queue - queue of request_socks
//
// @rskq_accept_head - FIFO head of established children
// @rskq_accept_tail - FIFO tail of established children
// @rskq_defer_accept - User waits for some data after accept()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_sock_queue {
    pub rskq_lock: spinlock_t,
    pub rskq_defer_accept: u8,
    pub synflood_warned: u8,
    pub qlen: core::sync::atomic::AtomicI32,
    pub young: core::sync::atomic::AtomicI32,
    pub rskq_accept_head: *mut request_sock,
    pub rskq_accept_tail: *mut request_sock,
    pub determine: *mut *mut fastopen_queue fastopenq; / Check max_qlen != 0 to,
// if TFO is enabled.
//
}

extern "C" {
    pub fn atomic_read(_arg: &queue->qlen) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &queue->young) -> return;
}
// RFC 7323 2.3 Using the Window Scale Option
// The window field (SEG.WND) of every outgoing segment, with the
// exception of <SYN> segments, MUST be right-shifted by
// Rcv.Wind.Shift bits.
//
// This means the SEG.WND carried in SYNACK can not exceed 65535.
// We use this property to harden TCP stack while in NEW_SYN_RECV state.
//
extern "C" {
    pub fn min(_arg: req->rsk_rcv_wnd, _arg: 65535U) -> return;
}
