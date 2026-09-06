//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/ipv6.h
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
// Linux INET6 implementation
//
// Authors:
// Pedro Roque		<roque@di.fc.ul.pt>
//

pub const SIN6_LEN_RFC2133: c_int = 24;
//
// NextHeader field of IPv6 header
//

pub const NEXTHDR_MAX: c_int = 255;
pub const IPV6_DEFAULT_HOPLIMIT: c_int = 64;
pub const IPV6_DEFAULT_MCASTHOPS: c_int = 1;
// Limits on Hop-by-Hop and Destination options.
//
// Per RFC8200 there is no limit on the maximum number or lengths of options in
// Hop-by-Hop or Destination options other then the packet must fit in an MTU.
// We allow configurable limits in order to mitigate potential denial of
// service attacks.
//
// There are three limits that may be set:
// - Limit the number of options in a Hop-by-Hop or Destination options
// extension header
// - Limit the byte length of a Hop-by-Hop or Destination options extension
// header
// - Disallow unknown options
//
// The limits are expressed in corresponding sysctls:
//
// ipv6.sysctl.max_dst_opts_cnt
// ipv6.sysctl.max_hbh_opts_cnt
// ipv6.sysctl.max_dst_opts_len
// ipv6.sysctl.max_hbh_opts_len
//
// max_*_opts_cnt is the number of TLVs that are allowed for Destination
// options or Hop-by-Hop options. If the number is less than zero then unknown
// TLVs are disallowed and the number of known options that are allowed is the
// absolute value. Setting the value to INT_MAX indicates no limit.
//
// max_*_opts_len is the length limit in bytes of a Destination or
// Hop-by-Hop options extension header. Setting the value to INT_MAX
// indicates no length limit.
//
// If a limit is exceeded when processing an extension header the packet is
// silently discarded.
//
// Default limits for Hop-by-Hop and Destination options
pub const IP6_DEFAULT_MAX_DST_OPTS_CNT: c_int = 8;
pub const IP6_DEFAULT_MAX_HBH_OPTS_CNT: c_int = 8;

// Hard limit on traversed IPv6 extension headers
pub const IP6_MAX_EXT_HDRS_CNT: c_int = 12;
//
// Addr type
//
// type	-	unicast | multicast
// scope	-	local	| site	    | global
// v4	-	compat
// v4mapped
// any
// loopback
//
pub const IPV6_ADDR_ANY: c_uint = 0x0000U;
pub const IPV6_ADDR_UNICAST: c_uint = 0x0001U;
pub const IPV6_ADDR_MULTICAST: c_uint = 0x0002U;
pub const IPV6_ADDR_LOOPBACK: c_uint = 0x0010U;
pub const IPV6_ADDR_LINKLOCAL: c_uint = 0x0020U;
pub const IPV6_ADDR_SITELOCAL: c_uint = 0x0040U;
pub const IPV6_ADDR_COMPATv4: c_uint = 0x0080U;
pub const IPV6_ADDR_SCOPE_MASK: c_uint = 0x00f0U;
pub const IPV6_ADDR_MAPPED: c_uint = 0x1000U;
//
// Addr scopes
//

pub const IPV6_ADDR_SCOPE_NODELOCAL: c_uint = 0x01;
pub const IPV6_ADDR_SCOPE_LINKLOCAL: c_uint = 0x02;
pub const IPV6_ADDR_SCOPE_SITELOCAL: c_uint = 0x05;
pub const IPV6_ADDR_SCOPE_ORGLOCAL: c_uint = 0x08;
pub const IPV6_ADDR_SCOPE_GLOBAL: c_uint = 0x0e;
//
// Addr flags
//

//
// fragmentation header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frag_hdr {
    pub nexthdr: __u8,
    pub reserved: __u8,
    pub frag_off: __be16,
    pub identification: __be32,
}

pub const IP6_MF: c_uint = 0x0001;
pub const IP6_OFFSET: c_uint = 0xFFF8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_fraglist_iter {
    pub tmp_hdr: *mut ipv6hdr,
    pub frag: *mut sk_buff,
    pub offset: c_int,
    pub hlen: c_uint,
    pub frag_id: __be32,
    pub nexthdr: u8,
}

extern "C" {
    pub fn ip6_fraglist_prepare(skb: *mut sk_buff, iter: *mut ip6_fraglist_iter);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_frag_state {
    pub prevhdr: *mut u8,
    pub hlen: c_uint,
    pub mtu: c_uint,
    pub left: c_uint,
    pub offset: c_int,
    pub ptr: c_int,
    pub hroom: c_int,
    pub troom: c_int,
    pub frag_id: __be32,
    pub nexthdr: u8,
}

// sysctls

// per device counters are atomic_long_t

// per device and per net counters are atomic_long_t

// MIBs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_ra_chain {
    pub next: *mut ip6_ra_chain,
    pub sk: *mut sock,
    pub sel: c_int,
    pub ): *mut *mut void (destructor)(struct sock,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_txoptions {
    pub refcnt: refcount_t,
// Length of this structure
    pub tot_len: c_int,
// length of extension headers
    pub /: *mut *mut __u16 opt_flen; / after fragment hdr,
    pub /: *mut *mut __u16 opt_nflen; / before fragment hdr,
    pub hopopt: *mut ipv6_opt_hdr,
    pub dst0opt: *mut ipv6_opt_hdr,
    pub /: *mut *mut *mut ipv6_rt_hdr srcrt; / Routing Header,
    pub dst1opt: *mut ipv6_opt_hdr,
    pub rcu: rcu_head,
// Option buffer, as read by IPV6_PKTOPTIONS, starts here.
}

// flowlabel_reflect sysctl values
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flowlabel_reflect {
    FLOWLABEL_REFLECT_ESTABLISHED		= 1,
    FLOWLABEL_REFLECT_TCP_RESET		= 2,
    FLOWLABEL_REFLECT_ICMPV6_ECHO_REPLIES	= 4,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip6_flowlabel {
    pub next: *mut ip6_flowlabel __rcu,
    pub label: __be32,
    pub users: core::sync::atomic::AtomicI32,
    pub dst: in6_addr,
    pub opt: *mut ipv6_txoptions,
    pub linger: c_ulong,
    pub rcu: rcu_head,
    pub share: u8,
    pub pid: *mut pid,
    pub uid: kuid_t,
    pub owner: },
    pub lastuse: c_ulong,
    pub expires: c_ulong,
    pub fl_net: *mut net,
}

pub const IPV6_TCLASS_SHIFT: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipv6_fl_socklist {
    pub next: *mut ipv6_fl_socklist __rcu,
    pub fl: *mut ip6_flowlabel,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipcm6_cookie {
    pub sockc: sockcm_cookie,
    pub hlimit: __s16,
    pub tclass: __s16,
    pub gso_size: __u16,
    pub dontfrag: __s8,
    pub opt: *mut ipv6_txoptions,
}

// ipc6 = (struct ipcm6_cookie) {

extern "C" {
    pub fn __fl6_sock_lookup(_arg: sk, ERR_PTR(-ENOENT: label) ? :) -> return;
}

extern "C" {
    pub fn fl6_free_socklist(sk: *mut sock);
}
extern "C" {
    pub fn ipv6_flowlabel_opt(sk: *mut sock, optval: sockptr_t, optlen: c_int) -> c_int;
}
extern "C" {
    pub fn ip6_flowlabel_init() -> c_int;
}
extern "C" {
    pub fn ip6_flowlabel_cleanup();
}
extern "C" {
    pub fn ip6_autoflowlabel(net: *mut net, sk: *const sock) -> bool;
}
extern "C" {
    pub fn ip6_ra_control(sk: *mut sock, sel: c_int) -> c_int;
}
extern "C" {
    pub fn ipv6_parse_hopopts(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __ipv6_fixup_options(_arg: opt_space, _arg: opt) -> return;
}
// If forwarding is enabled, RA are not accepted unless the special
// hybrid mode (accept_ra=2) is enabled.
//

extern "C" {
    pub fn __ipv6_addr_type(addr: *const in6_addr) -> c_int;
}
extern "C" {
    pub fn __ipv6_addr_src_scope(_arg: __ipv6_addr_type(addr)) -> return;
}
extern "C" {
    pub fn memcmp(_arg: a1, _arg: a2, in6_addr): sizeof(struct) -> return;
}

// caller must guarantee 0 <= plen <= 128

// ( u64 *)addr = (( u64)(wh) << 32 | ( u64)(wl));

// ( u64 *)addr = (( u64)(wl) << 32 | ( u64)(wh));

extern "C" {
    pub fn __ipv6_prefix_equal64_half(1: a1 +, 1: a2 +, 64: prefixlen -) -> return;
}
extern "C" {
    pub fn __ipv6_prefix_equal64_half(_arg: a1, _arg: a2, _arg: prefixlen) -> return;
}

// check complete u32 in prefix
// check incomplete u32 in prefix

// more secured version of ipv6_addr_hash()

//
// Note that we must  cast these to unsigned long to make sparse happy,
// since all of the endian-annotated types are fixed size regardless of arch.
//

// (unsigned long *)a |

extern "C" {
    pub fn ipv6_addr_v4mapped(ipv4_is_loopback(a->s6_addr32[3]: a) &&) -> return;
}
//
// Check for a RFC 4843 ORCHID address
// (Overlay Routable Cryptographic Hash Identifiers)
//
// find the first different bit between two addresses
// length of address must be a multiple of 32bits
//
// we should *never* get to this point since that
// would mean the addrs are equal
//
// However, we do get to it 8) And exactly, when
// addresses are equal 8)
//
// ip route add 1111::/128 via ...
// ip route add 1111::/64 via ...
// and we are here.
//
// Ideally, this function should stop comparison
// at prefix length. It does not, but it is still OK,
// if returned value is greater than prefix length.
// --ANK (980803)
//

extern "C" {
    pub fn __ipv6_addr_diff64(_arg: token1, _arg: token2, _arg: addrlen) -> return;
}

extern "C" {
    pub fn __ipv6_addr_diff32(_arg: token1, _arg: token2, _arg: addrlen) -> return;
}
extern "C" {
    pub fn __ipv6_addr_diff(_arg: a1, _arg: a2, in6_addr): sizeof(struct) -> return;
}
extern "C" {
    pub fn ipv6_proxy_select_ident(net: *mut net, skb: *mut sk_buff) -> __be32;
}
extern "C" {
    pub fn ip6_dst_hoplimit(dst: *mut dst_entry) -> c_int;
}
// copy IPv6 saddr & daddr to flow_keys, possibly using 64bit load/store
// Equivalent to :	flow->v6addrs.src = iph->saddr;
// flow->v6addrs.dst = iph->daddr;
//

// Sysctl settings for net ipv6.auto_flowlabels
pub const IP6_AUTO_FLOW_LABEL_OFF: c_int = 0;
pub const IP6_AUTO_FLOW_LABEL_OPTOUT: c_int = 1;
pub const IP6_AUTO_FLOW_LABEL_OPTIN: c_int = 2;
pub const IP6_AUTO_FLOW_LABEL_FORCED: c_int = 3;

// @flowlabel may include more than a flow label, eg, the traffic class.
// Here we want only the flow label value.
//
// Since this is being sent on the wire obfuscate hash a bit
// to minimize possibility that any useful information to an
// attacker is leaked. Only lower 20 bits are relevant.
//

extern "C" {
    pub fn READ_ONCE(_arg: net->ipv6.sysctl.multipath_hash_policy) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: net->ipv6.sysctl.multipath_hash_fields) -> return;
}

// Derive the IPv6 ECMP hash from txhash so a rehash may pick a different path;
// policy 0 only, and only when txhash is set.  >> 1 clears the top bit
// (fib6_select_path() uses mp_hash as a signed 31-bit value); ?: 1 keeps the
// result non-zero, since mp_hash 0 falls back to rt6_multipath_hash().
//
// Header manipulation
//
// (__be32 *)hdr = htonl(0x60000000 | (tclass << 20)) | flowlabel;
extern "C" {
    pub fn inet_dsfield_to_dscp(_arg: ip6_tclass(flowinfo)) -> return;
}
//
// Prototypes exported by ipv6
//
// rcv function (called from netdevice level)
//
extern "C" {
    pub fn ip6_rcv_finish(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
//
// upper-layer output functions
//
extern "C" {
    pub fn ip6_find_1stfragopt(skb: *mut sk_buff, nexthdr: *mut u8) -> c_int;
}
extern "C" {
    pub fn ip6_push_pending_frames(sk: *mut sock) -> c_int;
}
extern "C" {
    pub fn ip6_flush_pending_frames(sk: *mut sock);
}
extern "C" {
    pub fn ip6_send_skb(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __ip6_make_skb(_arg: sk, _arg: &sk->sk_write_queue, _arg: &inet_sk(sk)->cork) -> return;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EAFNOSUPPORT) -> return;
}

//
// skb processing functions
//
extern "C" {
    pub fn ip6_output(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip6_forward(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip6_input(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip6_mc_input(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn __ip6_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip6_local_out(net: *mut net, sk: *mut sock, skb: *mut sk_buff) -> c_int;
}
//
// Extension header (options) processing
//
extern "C" {
    pub fn ipv6_ext_hdr(nexthdr: u8) -> bool;
}
// find specified header and get offset to it
extern "C" {
    pub fn ipv6_find_tlv(skb: *const sk_buff, offset: c_int, type: c_int) -> c_int;
}
extern "C" {
    pub fn __fl6_update_dst(_arg: fl6, _arg: opt, _arg: orig) -> return;
}
//
// socket options (ipv6_sockglue.c)
//
extern "C" {
    pub fn ip6_datagram_connect(sk: *mut sock, addr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn ip6_datagram_dst_update(sk: *mut sock, fix_sk_saddr: bool) -> c_int;
}
extern "C" {
    pub fn ip6_datagram_release_cb(sk: *mut sock);
}
extern "C" {
    pub fn ipv6_recv_error(sk: *mut sock, msg: *mut msghdr, len: c_int) -> c_int;
}
extern "C" {
    pub fn ipv6_recv_rxpmtu(sk: *mut sock, msg: *mut msghdr, len: c_int) -> c_int;
}
extern "C" {
    pub fn ipv6_local_error(sk: *mut sock, err: c_int, fl6: *mut flowi6, info: u32);
}
extern "C" {
    pub fn ipv6_local_rxpmtu(sk: *mut sock, fl6: *mut flowi6, mtu: u32);
}
extern "C" {
    pub fn inet6_cleanup_sock(sk: *mut sock);
}
extern "C" {
    pub fn inet6_sock_destruct(sk: *mut sock);
}
extern "C" {
    pub fn inet6_release(sock: *mut socket) -> c_int;
}
extern "C" {
    pub fn inet6_bind(sock: *mut socket, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn inet6_bind_sk(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int;
}
extern "C" {
    pub fn inet6_ioctl(sock: *mut socket, cmd: c_uint, arg: c_ulong) -> c_int;
}
extern "C" {
    pub fn inet6_sendmsg(sock: *mut socket, msg: *mut msghdr, size: usize) -> c_int;
}
//
// reassembly.c
//

extern "C" {
    pub fn ac6_proc_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn ac6_proc_exit(net: *mut net);
}
extern "C" {
    pub fn raw6_proc_init() -> c_int;
}
extern "C" {
    pub fn raw6_proc_exit();
}
extern "C" {
    pub fn tcp6_proc_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn tcp6_proc_exit(net: *mut net);
}
extern "C" {
    pub fn udp6_proc_init(net: *mut net) -> c_int;
}
extern "C" {
    pub fn udp6_proc_exit(net: *mut net);
}
extern "C" {
    pub fn ipv6_misc_proc_init() -> c_int;
}
extern "C" {
    pub fn ipv6_misc_proc_exit();
}
extern "C" {
    pub fn snmp6_register_dev(idev: *mut inet6_dev) -> c_int;
}
extern "C" {
    pub fn snmp6_unregister_dev(idev: *mut inet6_dev) -> c_int;
}

extern "C" {
    pub fn ipv6_icmp_sysctl_table_size() -> usize;
}
extern "C" {
    pub fn ipv6_route_sysctl_table_size(net: *mut net) -> usize;
}
extern "C" {
    pub fn ipv6_sysctl_register() -> c_int;
}
extern "C" {
    pub fn ipv6_sysctl_unregister();
}

// check PUBLIC/TMP/PUBTMP_DEFAULT conflicts
// check HOME/COA conflicts
// check CGA/NONCGA conflicts
pub const IPV6_ADDR_WORDS: c_int = 4;
