//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tcp_ao.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union tcp_ao_addr {
    pub a4: in_addr,

    pub a6: in6_addr,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_hdr {
    pub kind: u8,
    pub length: u8,
    pub keyid: u8,
    pub rnext_keyid: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_counters {
    pub pkt_good: core::sync::atomic::AtomicI64,
    pub pkt_bad: core::sync::atomic::AtomicI64,
    pub key_not_found: core::sync::atomic::AtomicI64,
    pub ao_required: core::sync::atomic::AtomicI64,
    pub dropped_icmp: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tcp_ao_algo_id {
    TCP_AO_ALGO_HMAC_SHA1 = 1, /* specified by RFC 5926 */
    TCP_AO_ALGO_HMAC_SHA256, /* Linux extension */
    TCP_AO_ALGO_AES_128_CMAC, /* specified by RFC 5926 */
}

//
// This is the maximum untruncated MAC length, in bytes.  Note that the MACs
// actually get truncated to 20 or fewer bytes to fit in the TCP options space.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_key {
    pub node: hlist_node,
    pub addr: tcp_ao_addr,
    pub key: [u8; TCP_AO_MAXKEYLEN],
    pub algo: tcp_ao_algo_id,
    pub digest_size: c_uint,
    pub l3index: c_int,
    pub prefixlen: u8,
    pub family: u8,
    pub keylen: u8,
    pub keyflags: u8,
    pub sndid: u8,
    pub rcvid: u8,
    pub maclen: u8,
    pub rcu: rcu_head,
    pub pkt_good: core::sync::atomic::AtomicI64,
    pub pkt_bad: core::sync::atomic::AtomicI64,
    pub traffic_keys: [u8; ],
}

// Use tcp_ao_len_aligned() for TCP header calculations
extern "C" {
    pub fn tcp_ao_maclen(tcp_ao_hdr: key) + sizeof(struct) -> return;
}
extern "C" {
    pub fn round_up(_arg: tcp_ao_len(key), _arg: 4) -> return;
}
extern "C" {
    pub fn sizeof(1: tcp_ao_key) + (key->digest_size <<) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_ao_info {
// List of tcp_ao_key's
    pub head: hlist_head,
// current_key and rnext_key are maintained on sockets
// in TCP_AO_ESTABLISHED states.
// Their purpose is to cache keys on established connections,
// saving needless lookups. Never dereference any of them from
// listen sockets.
// ::current_key may change in RX to the key that was requested by
// the peer, please use READ_ONCE()/WRITE_ONCE() in order to avoid
// load/store tearing.
// Do the same for ::rnext_key, if you don't hold socket lock
// (it's changed only by userspace request in setsockopt()).
//
    pub current_key: *mut tcp_ao_key,
    pub rnext_key: *mut tcp_ao_key,
    pub counters: tcp_ao_counters,
    pub :30: __unused,
    pub lisn: __be32,
    pub risn: __be32,
// Sequence Number Extension (SNE) are upper 4 bytes for SEQ,
// that protect TCP-AO connection from replayed old TCP segments.
// See RFC5925 (6.2).
// In order to get correct SNE, there's a helper tcp_ao_compute_sne().
// It needs SEQ basis to understand whereabouts are lower SEQ numbers.
// According to that basis vector, it can provide incremented SNE
// when SEQ rolls over or provide decremented SNE when there's
// a retransmitted segment from before-rolling over.
// - for request sockets such basis is rcv_isn/snt_isn, which seems
// good enough as it's unexpected to receive 4 Gbytes on reqsk.
// - for full sockets the basis is rcv_nxt/snd_una. snd_una is
// taken instead of snd_nxt as currently it's easier to track
// in tcp_snd_una_update(), rather than updating SNE in all
// WRITE_ONCE(tp->snd_nxt, ...)
// - for time-wait sockets the basis is tw_rcv_nxt/tw_snd_nxt.
// tw_snd_nxt is not expected to change, while tw_rcv_nxt may.
//
    pub snd_sne: u32,
    pub rcv_sne: u32,
    pub /: *mut *mut refcount_t refcnt; / Protects twsk destruction,
    pub rcu: rcu_head,
}

// TCP-AO structures and functions

// TCP-AO structures and functions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp4_ao_context {
    pub saddr: __be32,
    pub daddr: __be32,
    pub sport: __be16,
    pub dport: __be16,
    pub sisn: __be32,
    pub disn: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp6_ao_context {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub sport: __be16,
    pub dport: __be16,
    pub sisn: __be32,
    pub disn: __be32,
}

// Established states are fast-path and there always is current_key/rnext_key

extern "C" {
    pub fn tcp_ao_destroy_sock(sk: *mut sock, twsk: bool);
}
extern "C" {
    pub fn tcp_ao_time_wait(tcptw: *mut tcp_timewait_sock, tp: *mut tcp_sock);
}
extern "C" {
    pub fn tcp_ao_ignore_icmp(sk: *const sock, family: c_int, type: c_int, code: c_int) -> bool;
}
extern "C" {
    pub fn tcp_ao_get_mkts(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t) -> c_int;
}
extern "C" {
    pub fn tcp_ao_get_sock_info(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t) -> c_int;
}
extern "C" {
    pub fn tcp_ao_get_repair(sk: *mut sock, optval: sockptr_t, optlen: sockptr_t) -> c_int;
}
extern "C" {
    pub fn tcp_ao_set_repair(sk: *mut sock, optval: sockptr_t, optlen: c_uint) -> c_int;
}
extern "C" {
    pub fn tcp_ao_compute_sne(next_sne: u32, next_seq: u32, seq: u32) -> u32;
}
// ipv4 specific functions
extern "C" {
    pub fn tcp_v4_parse_ao(sk: *mut sock, cmd: c_int, optval: sockptr_t, optlen: c_int) -> c_int;
}
// ipv6 specific functions
extern "C" {
    pub fn tcp_v6_parse_ao(sk: *mut sock, cmd: c_int, optval: sockptr_t, optlen: c_int) -> c_int;
}
extern "C" {
    pub fn tcp_ao_established(sk: *mut sock);
}
extern "C" {
    pub fn tcp_ao_finish_connect(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn tcp_ao_connect_init(sk: *mut sock);
}

// md5_hash = NULL;
// ao_hash = NULL;

