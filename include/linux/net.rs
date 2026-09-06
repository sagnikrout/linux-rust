//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/net.h
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
// NET		An implementation of the SOCKET network access protocol.
// This is the master header file for the Linux NET layer,
// or, in plain English: the networking handling part of the
// kernel.
//
// Version:	@(#)net.h	1.0.3	05/25/93
//
// Authors:	Orest Zborowski, <obz@Kodak.COM>
// Ross Biro
// Fred N. van Kempen, <waltje@uWalt.NL.Mugnet.ORG>
//

//
// struct sockopt - socket option value container
// @iter_in: iov_iter for reading optval with the content from the caller.
// Use copy_from_iter() given this iov direction is ITER_SOURCE
// @iter_out: iov_iter for protocols to update optval data to userspace
// Use _copy_to_iter() given iov direction is ITER_DEST
// @optlen: serves as both input (buffer size) and output (returned data size).
//
// Type-safe wrapper for socket option data that works with both
// user and kernel buffers.
//
// The optlen field allows callbacks to return a specific length value
// independent of the bytes written via copy_to_iter().
//
// Initialize a user-backed sockopt_t from the (optval, optlen) __user pair of
// a getsockopt() callback. Used by transitional __user getsockopt wrappers
// while the proto-layer callbacks are converted to take a sockopt_t; the
// caller writes opt->optlen back to the user optlen after the callback.
//
// Historically, SOCKWQ_ASYNC_NOSPACE & SOCKWQ_ASYNC_WAITDATA were located
// in sock->flags, but moved into sk->sk_wq->flags to be RCU protected.
// Eventually all flags will be in sk->sk_wq->flags.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum socket_flags {
    SOCKWQ_ASYNC_NOSPACE,
    SOCKWQ_ASYNC_WAITDATA,
    SOCK_NOSPACE,
    SOCK_SUPPORT_ZC,
    SOCK_CUSTOM_SOCKOPT,
}

//
// enum sock_type - Socket types
// @SOCK_STREAM: stream (connection) socket
// @SOCK_DGRAM: datagram (conn.less) socket
// @SOCK_RAW: raw socket
// @SOCK_RDM: reliably-delivered message
// @SOCK_SEQPACKET: sequential packet socket
// @SOCK_DCCP: Datagram Congestion Control Protocol socket
// @SOCK_PACKET: linux specific way of getting packets at the dev level.
// For writing rarp and other similar things on the user level.
//
// When adding some new socket type please
// grep ARCH_HAS_SOCKET_TYPE include/asm-* /socket.h, at least MIPS
// overrides this enum for binary compat reasons.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sock_type {
    SOCK_STREAM	= 1,
    SOCK_DGRAM	= 2,
    SOCK_RAW	= 3,
    SOCK_RDM	= 4,
    SOCK_SEQPACKET	= 5,
    SOCK_DCCP	= 6,
    SOCK_PACKET	= 10,
}

// Mask which covers at least up to SOCK_MASK-1.  The
// remaining bits are used as flags.
pub const SOCK_TYPE_MASK: c_uint = 0xf;
// Flags for socket, socketpair, accept4

//
// enum sock_shutdown_cmd - Shutdown types
// @SHUT_RD: shutdown receptions
// @SHUT_WR: shutdown transmissions
// @SHUT_RDWR: shutdown receptions/transmissions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sock_shutdown_cmd {
    SHUT_RD,
    SHUT_WR,
    SHUT_RDWR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_wq {
// Note: wait MUST be first field of socket_wq
    pub wait: wait_queue_head_t,
    pub fasync_list: *mut fasync_struct,
    pub /: *mut *mut unsigned long flags; / %SOCKWQ_ASYNC_NOSPACE, etc,
    pub rcu: rcu_head,
    pub ____cacheline_aligned_in_smp: },
//
// struct socket - general BSD socket
// @state: socket state (%SS_CONNECTED, etc)
// @type: socket type (%SOCK_STREAM, etc)
// @flags: socket flags (%SOCK_NOSPACE, etc)
// @ops: protocol specific socket operations
// @file: File back pointer for gc
// @sk: internal networking protocol agnostic socket representation
// @wq: wait queue for several uses
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket {
    pub state: socket_state,
    pub type: c_short,
    pub flags: c_ulong,
    pub file: *mut file,
    pub sk: *mut sock,
    pub /: *const *const *const proto_ops ops; / Might change with IPV6_ADDRFORM or MPTCP.,
    pub wq: socket_wq,
}

//
// "descriptor" for what we're up to with a read.
// This allows us to use the same read code yet
// have multiple different users of the data that
// we read from a file.
//
// The simplest case just copies the data to user
// mode.
//
extern "C" {
    pub fn int(: *mut *mut skb_read_actor_t)(struct sock, : *mut sk_buff) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct proto_ops {
    pub family: c_int,
    pub owner: *mut module,
    pub sock): *mut *mut int (release) (struct socket,
    pub sockaddr_len): c_int,
    pub flags): int sockaddr_len, int,
    pub sock2): *mut socket,
    pub arg): *mut proto_accept_arg,
    pub peer): c_int,
    pub wait): *mut poll_table_struct,
    pub arg): c_ulong,

    pub arg): c_ulong,

    pub time32): bool timeval, bool,
    pub len): *mut *mut *mut int (listen) (struct socket sock, int,
    pub flags): *mut *mut *mut int (shutdown) (struct socket sock, int,
    pub optlen): c_uint,
    pub optlen): *mut *mut int optname, char __user optval, int __user,
    pub opt): *mut int optname, sockopt_t,
    pub sock): *mut *mut *mut void (show_fdinfo)(struct seq_file m, struct socket,
    pub total_len): usize,
// Notes for implementing recvmsg:
// ===============================
// msg->msg_namelen should get updated by the recvmsg handlers
// iff msg_name != NULL. It is by default 0 to prevent
// returning uninitialized memory to user space.  The recvfrom
// handlers can assume that msg.msg_name is either NULL or has
// a minimum size of sizeof(struct sockaddr_storage).
//
    pub flags): size_t total_len, int,
    pub vma): *mut *mut vm_area_struct,
    pub flags): *mut *mut pipe_inode_info pipe, size_t len, unsigned int,
    pub sock): *mut *mut void (splice_eof)(struct socket,
    pub val): *mut *mut *mut int (set_peek_off)(struct sock sk, int,
    pub sock): *mut *mut int (peek_len)(struct socket,
// The following functions are called internally by kernel with
// sock lock already held.
//
    pub recv_actor): sk_read_actor_t,
// This is different from read_sock(), it reads an entire skb at a time.
    pub recv_actor): *mut *mut *mut int (read_skb)(struct sock sk, skb_read_actor_t,
    pub size): usize,
    pub val): *mut *mut *mut int (set_rcvlowat)(struct sock sk, int,
    pub val): *mut *mut *mut void (set_rcvbuf)(struct sock sk, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_proto_family {
    pub family: c_int,
    pub kern): int protocol, int,
    pub owner: *mut module,
}

extern "C" {
    pub fn sock_wake_async(sk_wq: *mut socket_wq, how: c_int, band: c_int) -> c_int;
}
extern "C" {
    pub fn sock_register(fam: *const net_proto_family) -> c_int;
}
extern "C" {
    pub fn sock_unregister(family: c_int);
}
extern "C" {
    pub fn sock_is_registered(family: c_int) -> bool;
}
extern "C" {
    pub fn sock_create(family: c_int, type: c_int, proto: c_int, res: *mut socket) -> c_int;
}
extern "C" {
    pub fn sock_create_kern(net: *mut net, family: c_int, type: c_int, proto: c_int, res: *mut socket) -> c_int;
}
extern "C" {
    pub fn sock_create_lite(family: c_int, type: c_int, proto: c_int, res: *mut socket) -> c_int;
}
extern "C" {
    pub fn sock_release(sock: *mut socket);
}
extern "C" {
    pub fn sock_sendmsg(sock: *mut socket, msg: *mut msghdr) -> c_int;
}
extern "C" {
    pub fn sock_recvmsg(sock: *mut socket, msg: *mut msghdr, flags: c_int) -> c_int;
}
extern "C" {
    pub fn sock_read_xattr(sock: *mut socket, name: *const c_char, value: *mut c_void, size: usize) -> c_int;
}

extern "C" {
    pub fn net_ratelimit() -> c_int;
}

//
// E.g. XFS meta- & log-data is in slab pages, or bcache meta
// data pages, or other high order pages allocated by
// __get_free_pages() without __GFP_COMP, which have a page_count
// of 0 and/or have PageSlab() set. We cannot use send_page for
// those, as that does get_page(); put_page(); and would cause
// either a VM_BUG directly, or __page_cache_release a page that
// would actually still be referenced by someone, leading to some
// obscure delayed Oops somewhere else.
//
// Check sendpage_ok on contiguous pages.
//
extern "C" {
    pub fn kernel_bind(sock: *mut socket, addr: *mut sockaddr_unsized, addrlen: c_int) -> c_int;
}
extern "C" {
    pub fn kernel_listen(sock: *mut socket, backlog: c_int) -> c_int;
}
extern "C" {
    pub fn kernel_accept(sock: *mut socket, newsock: *mut socket, flags: c_int) -> c_int;
}
extern "C" {
    pub fn kernel_getsockname(sock: *mut socket, addr: *mut sockaddr) -> c_int;
}
extern "C" {
    pub fn kernel_getpeername(sock: *mut socket, addr: *mut sockaddr) -> c_int;
}
extern "C" {
    pub fn kernel_sock_shutdown(sock: *mut socket, how: sock_shutdown_cmd) -> c_int;
}
// Routine returns the IP overhead imposed by a (caller-protected) socket.
extern "C" {
    pub fn kernel_sock_ip_overhead(sk: *mut sock) -> u32;
}

