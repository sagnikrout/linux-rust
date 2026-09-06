//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/udp.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Definitions for the UDP module.
//
// Version:	@(#)udp.h	1.0.2	05/07/93
//
// Authors:	Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//
// Fixes:
// Alan Cox	: Turned on udp checksums. I don't want to
// chase 'memory corruption' bugs that aren't!
//

//
// struct udp_skb_cb  -  UDP private variables
//
// @header:      private variables used by IPv4/IPv6
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_skb_cb {
    pub h4: inet_skb_parm,

    pub h6: inet6_skb_parm,

    pub header: },
}

//
// struct udp_hslot - UDP hash slot used by udp_table.hash/hash4
//
// @head:	head of list of sockets
// @nulls_head:	head of list of sockets, only used by hash4
// @count:	number of sockets in 'head' list
// @lock:	spinlock protecting changes to head/count
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_hslot {
    pub head: hlist_head,
// hash4 uses hlist_nulls to avoid moving wrongly onto another
// hlist, because rehash() can happen with lookup().
//
    pub nulls_head: hlist_nulls_head,
}

//
// struct udp_hslot_main - UDP hash slot used by udp_table.hash2
//
// @hslot:	basic hash slot
// @hash4_cnt: number of sockets in hslot4 of the same
// (local port, local address)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_hslot_main {
    pub /: *mut *mut udp_hslot hslot; / must be the first member,

    pub hash4_cnt: u32,

    pub sizeof(long)): *mut *mut } __aligned(2,

//
// struct udp_table - UDP table
//
// @hash:	hash table, sockets are hashed on (local port)
// @hash2:	hash table, sockets are hashed on (local port, local address)
// @hash4:	hash table, connected sockets are hashed on
// (local port, local address, remote port, remote address)
// @mask:	number of slots in hash tables, minus 1
// @log:	log2(number of slots in hash table)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_table {
    pub hash: *mut udp_hslot,
    pub hash2: *mut udp_hslot_main,

    pub hash4: *mut udp_hslot,

    pub mask: c_uint,
    pub log: c_uint,
}

//
// For secondary hash, net_hash_mix() is performed before calling
// udp_hashslot2(), this explains difference with udp_hashslot()
//

// Must be called with table->hash2 initialized
extern "C" {
    pub fn sizeof(udp_hslot: struct) -> return;
}

// sysctl variables for udp
//
// Generic checksumming routines for UDP v4 and v6
//
extern "C" {
    pub fn __skb_checksum_complete(_arg: skb) -> return;
}
//
// udp_csum_outgoing  -  compute UDPv4/v6 checksum over fragments
// @sk: 	socket we are writing to
// @skb: 	sk_buff containing the filled-in UDP header
// (checksum field must be zeroed out)
//
extern "C" {
    pub fn csum_tcpudp_magic(_arg: saddr, _arg: daddr, _arg: len, _arg: IPPROTO_UDP, _arg: base) -> return;
}
extern "C" {
    pub fn udp_v6_early_demux(skb: *mut sk_buff);
}
extern "C" {
    pub fn udpv6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int;
}
// hash routines shared between UDPv4/6
extern "C" {
    pub fn udp_lib_unhash(sk: *mut sock);
}
extern "C" {
    pub fn udp_lib_rehash(sk: *mut sock, new_hash: u16, new_hash4: u16);
}
// hash4 routines shared between UDPv4/6

extern "C" {
    pub fn udp_lib_hash4(sk: *mut sock, hash: u16);
}
extern "C" {
    pub fn udp4_hash4(sk: *mut sock);
}

extern "C" {
    pub fn udp_flow_hashrnd() -> u32;
}
// Use default range
// Can't find a normal hash, caller has indicated an
// Ethernet packet so use that to compute a hash.
//
// Can't derive any sort of hash for the packet, set
// to some consistent random value.
//
// Since this is being sent on the wire obfuscate hash a bit
// to minimize possibility that any useful information to an
// attacker is leaked. Only upper 16 bits are relevant in the
// computation for 16 bit port value.
//
extern "C" {
    pub fn htons(_arg: reciprocal_scale(hash, min: max - min + 1) +) -> return;
}
extern "C" {
    pub fn sk_rmem_alloc_get(READ_ONCE(udp_sk(sk)->forward_deficit: sk) -) -> return;
}

extern "C" {
    pub fn inet_bound_dev_eq(_arg: true, _arg: bound_dev_if, _arg: dif, _arg: sdif) -> return;
}

// net/ipv4/udp.c
extern "C" {
    pub fn udp_destruct_common(sk: *mut sock);
}
extern "C" {
    pub fn skb_consume_udp(sk: *mut sock, skb: *mut sk_buff, len: c_int);
}
extern "C" {
    pub fn __udp_enqueue_schedule_skb(sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn udp_skb_destructor(sk: *mut sock, skb: *mut sk_buff);
}
extern "C" {
    pub fn __skb_recv_udp(_arg: sk, _arg: flags, _arg: &off, _arg: err) -> return;
}
extern "C" {
    pub fn udp_v4_early_demux(skb: *mut sk_buff) -> skb_drop_reason;
}
extern "C" {
    pub fn udp_sk_rx_dst_set(sk: *mut sock, dst: *mut dst_entry) -> bool;
}
extern "C" {
    pub fn udp_err(: *mut sk_buff, _arg: u32) -> c_int;
}
extern "C" {
    pub fn udp_abort(sk: *mut sock, err: c_int) -> c_int;
}
extern "C" {
    pub fn udp_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int;
}
extern "C" {
    pub fn udp_splice_eof(sock: *mut socket);
}
extern "C" {
    pub fn udp_push_pending_frames(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn udp_flush_pending_frames(sk: *mut sock);
}
extern "C" {
    pub fn udp_cmsg_send(sk: *mut sock, msg: *mut msghdr, gso_size: *mut u16) -> c_int;
}
extern "C" {
    pub fn udp4_hwcsum(skb: *mut sk_buff, src: __be32, dst: __be32);
}
extern "C" {
    pub fn udp_rcv(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn udp_ioctl(sk: *mut sock, cmd: c_int, karg: *mut c_int) -> c_int;
}
extern "C" {
    pub fn udp_pre_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn __udp_disconnect(sk: *mut sock, flags: c_int) -> c_int;
}
extern "C" {
    pub fn udp_disconnect(sk: *mut sock, flags: c_int) -> c_int;
}
extern "C" {
    pub fn udp_poll(file: *mut file, sock: *mut socket, wait: *mut poll_table) -> __poll_t;
}
extern "C" {
    pub fn udp_read_skb(sk: *mut sock, recv_actor: skb_read_actor_t) -> c_int;
}
// UDP uses skb->dev_scratch to cache as much information as possible and avoid
// possibly multiple cache miss on dequeue()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_dev_scratch {
// skb->truesize and the stateless bit are embedded in a single field;
// do not use a bitfield since the compiler emits better/smaller code
// this way
//
    pub _tsize_state: u32,

// len and the bit needed to compute skb_csum_unnecessary
// will be on cold cache lines at recvmsg time.
// skb->len can be stored on 16 bits since the udp header has been
// already validated and pulled.
//
    pub len: u16,
    pub is_linear: bool,
    pub csum_unnecessary: bool,

}

extern "C" {
    pub fn skb_csum_unnecessary(_arg: skb) -> return;
}

//
// SNMP statistics for UDP
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_seq_afinfo {
    pub family: sa_family_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct udp_iter_state {
    pub p: seq_net_private,
    pub bucket: c_int,
}

extern "C" {
    pub fn udp_seq_stop(seq: *mut seq_file, v: *mut c_void);
}
extern "C" {
    pub fn udp4_proc_init() -> c_int;
}
extern "C" {
    pub fn udp4_proc_exit();
}

extern "C" {
    pub fn udpv4_offload_init() -> c_int;
}
extern "C" {
    pub fn udp_init();
}
extern "C" {
    pub fn udp_encap_enable();
}
extern "C" {
    pub fn udp_encap_disable();
}

extern "C" {
    pub fn udpv6_encap_enable();
}

//
// Segmentation in UDP receive path is only for UDP GRO, drop udp
// fragmentation offload (UFO) packets.
//
// Avoid csum recalculation by skb_segment unless userspace explicitly
// asks for the final checksum values
//
// UDP segmentation expects packets of type CHECKSUM_PARTIAL or
// CHECKSUM_NONE in __udp_gso_segment. UDP GRO indeed builds partial
// packets in udp_gro_complete_segment. As does UDP GSO, verified by
// udp_send_skb. But when those packets are looped in dev_loopback_xmit
// their ip_summed CHECKSUM_NONE is changed to CHECKSUM_UNNECESSARY.
// Reset in this specific case, where PARTIAL is both correct and
// required.
//
// the GSO CB lays after the UDP one, no need to save and restore any
// CB fragment
//
// UDP packets generated with UDP_SEGMENT and traversing:
//
// UDP tunnel(xmit) -> veth (segmentation) -> veth (gro) -> UDP tunnel (rx)
//
// can reach an UDP socket with CHECKSUM_NONE, because
// __iptunnel_pull_header() converts CHECKSUM_PARTIAL into NONE.
// SKB_GSO_UDP_L4 or SKB_GSO_FRAGLIST packets with no UDP tunnel will
// have a valid checksum, as the GRO engine validates the UDP csum
// before the aggregation and nobody strips such info in between.
// Instead of adding another check in the tunnel fastpath, we can force
// a valid csum after the segmentation.
// Additionally fixup the UDP CB.
//

extern "C" {
    pub fn udp_bpf_update_proto(sk: *mut sock, psock: *mut sk_psock, restore: bool) -> c_int;
}

