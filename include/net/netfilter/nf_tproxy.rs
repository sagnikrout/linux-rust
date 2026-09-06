//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/netfilter/nf_tproxy.h
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


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nf_tproxy_lookup_t {
    NF_TPROXY_LOOKUP_LISTENER,
    NF_TPROXY_LOOKUP_ESTABLISHED,
}

// assign a socket to the skb -- consumes sk
extern "C" {
    pub fn nf_tproxy_laddr4(skb: *mut sk_buff, user_laddr: __be32, daddr: __be32) -> __be32;
}
//
// nf_tproxy_handle_time_wait4 - handle IPv4 TCP TIME_WAIT reopen redirections
// @net:	The network namespace.
// @skb:	The skb being processed.
// @laddr:	IPv4 address to redirect to or zero.
// @lport:	TCP port to redirect to or zero.
// @sk:		The TIME_WAIT TCP socket found by the lookup.
//
// We have to handle SYN packets arriving to TIME_WAIT sockets
// differently: instead of reopening the connection we should rather
// redirect the new connection to the proxy if there's a listener
// socket present.
//
// nf_tproxy_handle_time_wait4() consumes the socket reference passed in.
//
// Returns: the listener socket if there's one, the TIME_WAIT socket if
// no such listener is found, or NULL if the TCP header is incomplete.
//
// This is used when the user wants to intercept a connection matching
// an explicit iptables rule. In this case the sockets are assumed
// matching in preference order:
//
// - match: if there's a fully established connection matching the
// _packet_ tuple, it is returned, assuming the redirection
// already took place and we process a packet belonging to an
// established connection
//
// - match: if there's a listening socket matching the redirection
// (e.g. on-port & on-ip of the connection), it is returned,
// regardless if it was bound to 0.0.0.0 or an explicit
// address. The reasoning is that if there's an explicit rule, it
// does not really matter if the listener is bound to an interface
// or to 0. The user already stated that he wants redirection
// (since he added the rule).
//
// Please note that there's an overlap between what a TPROXY target
// and a socket match will match. Normally if you have both rules the
// "socket" match will be the first one, effectively all packets
// belonging to established connections going through that one.
//
// nf_tproxy_handle_time_wait6 - handle IPv6 TCP TIME_WAIT reopen redirections
// @skb:	The skb being processed.
// @tproto:	Transport protocol.
// @thoff:	Transport protocol header offset.
// @net:	Network namespace.
// @laddr:	IPv6 address to redirect to.
// @lport:	TCP port to redirect to or zero.
// @sk:		The TIME_WAIT TCP socket found by the lookup.
//
// We have to handle SYN packets arriving to TIME_WAIT sockets
// differently: instead of reopening the connection we should rather
// redirect the new connection to the proxy if there's a listener
// socket present.
//
// nf_tproxy_handle_time_wait6() consumes the socket reference passed in.
//
// Returns: the listener socket if there's one, the TIME_WAIT socket if
// no such listener is found, or NULL if the TCP header is incomplete.
//
