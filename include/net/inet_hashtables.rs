//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/inet_hashtables.h
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
// operating system.  INET is implemented using the BSD Socket
// interface as the means of communication with the user level.
//
// Authors:	Lotsa people, from code originally in tcp
//

// This is for all connections with a full identity, no wildcards.
// The 'e' prefix stands for Establish, but we really put all sockets
// but LISTEN ones.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_ehash_bucket {
    pub chain: hlist_nulls_head,
}

// There are a few simple rules, which allow for local port reuse by
// an application.  In essence:
//
// 1) Sockets bound to different interfaces may share a local port.
// Failing that, goto test 2.
// 2) If all sockets have sk->sk_reuse set, and none of them are in
// TCP_LISTEN state, the port may be shared.
// Failing that, goto test 3.
// 3) If all sockets are bound to a specific inet_sk(sk)->rcv_saddr local
// address, and none of them are the same, the port may be
// shared.
// Failing this, the port cannot be shared.
//
// The interesting point, is test #2.  This is what an FTP server does
// all day.  To optimize this case we use a specific flag bit defined
// below.  As we add sockets to a bind bucket list, we perform a
// check of: (newsk->sk_reuse && (newsk->sk_state != TCP_LISTEN))
// As long as all sockets added to a bind bucket pass this test,
// the flag bit will be set.
// The resulting situation is that tcp_v[46]_verify_bind() can just check
// for this flag bit, if it is set and the socket trying to bind has
// sk->sk_reuse set, we don't even have to walk the owners list at all,
// we return that it is ok to bind this socket to the requested local port.
//
// Sounds like a lot of work, but it is worth it.  In a more naive
// implementation (ie. current FreeBSD etc.) the entire list of ports
// must be walked for each data port opened by an ftp server.  Needless
// to say, this does not scale at all.  With a couple thousand FTP
// users logged onto your box, isn't it nice to know that new data
// ports are created in O(1) time?  I thought so. ;-)	-DaveM
//
pub const FASTREUSEPORT_ANY: c_int = 1;
pub const FASTREUSEPORT_STRICT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_bind_bucket {
    pub ib_net: possible_net_t,
    pub l3mdev: c_int,
    pub port: c_ushort,
    pub fastreuse: signed char,
    pub fastreuseport: signed char,
    pub fastuid: kuid_t,

    pub fast_v6_rcv_saddr: in6_addr,

    pub fast_rcv_saddr: __be32,
    pub fast_sk_family: c_ushort,
    pub fast_ipv6_only: bool,
    pub node: hlist_node,
    pub bhash2: hlist_head,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_bind2_bucket {
    pub ib_net: possible_net_t,
    pub l3mdev: c_int,
    pub port: c_ushort,

    pub addr_type: c_ushort,
    pub v6_rcv_saddr: in6_addr,

    pub rcv_saddr: __be32,

// Node in the bhash2 inet_bind_hashbucket chain
    pub node: hlist_node,
    pub bhash_node: hlist_node,
// List of sockets hashed to this bucket
    pub owners: hlist_head,
    pub fastreuse: signed char,
    pub fastreuseport: signed char,
}

extern "C" {
    pub fn read_pnet(_arg: &ib->ib_net) -> return;
}
extern "C" {
    pub fn read_pnet(_arg: &ib->ib_net) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_bind_hashbucket {
    pub lock: spinlock_t,
    pub chain: hlist_head,
}

// Sockets can be hashed in established or listening table.
// We must use different 'nulls' end-of-chain value for all hash buckets :
// A socket might transition from ESTABLISH to LISTEN state without
// RCU grace period. A lookup in ehash table needs to handle this case.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_listen_hashbucket {
    pub lock: spinlock_t,
    pub nulls_head: hlist_nulls_head,
}

// This is for listening sockets, thus all sockets which possess wildcards.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_hashinfo {
// This is for sockets with full identity only.  Sockets here will
// always be without wildcards and will have the following invariant:
//
// TCP_ESTABLISHED <= sk->sk_state < TCP_CLOSE
//
    pub ehash: *mut inet_ehash_bucket,
    pub ehash_locks: *mut spinlock_t,
    pub ehash_mask: c_uint,
    pub ehash_locks_mask: c_uint,
// Ok, let's try this, I give up, we do need a local binding
// TCP hash as well as the others for fast bind/connect.
//
    pub bind_bucket_cachep: *mut kmem_cache,
// This bind table is hashed by local port
    pub bhash: *mut inet_bind_hashbucket,
    pub bind2_bucket_cachep: *mut kmem_cache,
// This bind table is hashed by local port and sk->sk_rcv_saddr (ipv4)
// or sk->sk_v6_rcv_saddr (ipv6). This 2nd bind table is used
// primarily for expediting bind conflict resolution.
//
    pub bhash2: *mut inet_bind_hashbucket,
    pub bhash_size: c_uint,
// The 2nd listener table hashed by local port and address
    pub lhash2_mask: c_uint,
    pub lhash2: *mut inet_listen_hashbucket,
    pub pernet: bool,
    pub ____cacheline_aligned_in_smp: },
    pub sock_net(sk)->ipv4.tcp_death_row.hashinfo: return,
    pub h->lhash2_mask]: return &h->lhash2[hash &,
    pub hashinfo->ehash_mask]: return &hashinfo->ehash[hash &,
    pub hashinfo->ehash_locks_mask]: return &hashinfo->ehash_locks[hash &,
    pub hashinfo): *mut int inet_ehash_locks_alloc(struct inet_hashinfo,
    pub NULL: hashinfo->ehash_locks =,
    pub ehash_entries): c_uint,
    pub hashinfo): *mut void inet_pernet_hashinfo_free(struct inet_hashinfo,
    pub l3mdev): unsigned short snum, int,
    pub tb): *mut void inet_bind_bucket_destroy(struct inet_bind_bucket,
    pub l3mdev): c_int,
    pub sk): *const sock,
    pub tb): *mut inet_bind2_bucket,
    pub sk): *const sock,
    pub sk): *const int l3mdev, struct sock,
    pub 1): return (lport + net_hash_mix(net)) & (bhash_size -,
    pub hash: u32,

    pub port): hash = ipv6_portaddr_hash(net, &sk->sk_v6_rcv_saddr,,

    pub port): hash = ipv4_portaddr_hash(net, sk->sk_rcv_saddr,,
    pub 1)]: return &hinfo->bhash2[hash & (hinfo->bhash_size -,

    pub false: return,
    pub true: return,

    pub htonl(INADDR_ANY): return sk->sk_rcv_saddr !=,
    pub port): *const *const *const inet_bhash2_addr_any_hashbucket(struct sock sk, struct net net, int,
// This should be called whenever a socket's sk_rcv_saddr (ipv4) or
// sk_v6_rcv_saddr (ipv6) changes after it has been binded. The socket's
// rcv_saddr field should already have been updated when this is called.
//
    pub family): *mut *mut *mut int inet_bhash2_update_saddr(struct sock sk, void saddr, int,
    pub sk): *mut void inet_bhash2_reset_saddr(struct sock,
    pub port): *mut *mut inet_bind2_bucket tb2, unsigned short,
// Caller must disable local BH processing.
    pub child): *const *const int __inet_inherit_port(struct sock sk, struct sock,
    pub sk): *mut void inet_put_port(struct sock,
    pub high_limit): c_ulong,
    pub found_dup_sk): *mut *mut *mut bool inet_ehash_insert(struct sock sk, struct sock osk, bool,
    pub found_dup_sk): *mut bool,
    pub sk): *mut int inet_hash(struct sock,
    pub sk): *mut void inet_unhash(struct sock,
    pub sdif): int dif, int,
    pub sdif): daddr, ntohs(dport), dif,,
// Socket demux engine toys.
// What happens here is ugly; there's a pair of adjacent fields in
    pub to: inet_sock; __be16 dport followed by __u16 num. We want,
//

    pub false: return,
// READ_ONCE() paired with WRITE_ONCE() in sock_bindtoindex_locked()
// Sockets in TCP_CLOSE state are _always_ taken out of the hash, so we need
// not check it for lookups anymore, thanks Alexey. -DaveM
//
    pub sdif): int dif, int,
    pub fport): __be32 faddr, __be16,
    pub inet_ehashfn: inet_ehashfn_t,
    pub udp_ehashfn): INDIRECT_CALLABLE_DECLARE(inet_ehashfn_t,
    pub ehashfn): *mut inet_ehashfn_t,
    pub ehashfn): *mut inet_ehashfn_t,
    pub 0): ntohs(dport), dif,,
    pub ntohs(dport): u16 hnum =,
    pub sk: *mut sock,
    pub sdif): daddr, hnum, dif,,
// refcounted = true;
    pub sk: return,
// refcounted = false;
    pub sdif): sport, daddr, hnum, dif,,
    pub sk: *mut sock,
    pub refcounted: bool,
    pub &refcounted): dport, dif, 0,,
    pub NULL: sk =,
    pub sk: return,
    pub reuse_sk: *mut *mut sock sk,,
    pub prefetched: bool,
    pub &prefetched): sk = skb_steal_sock(skb, refcounted,,
    pub NULL: return,
    pub sk: return,
    pub sk: return,
    pub sk: return,
    pub sk: return,
    pub sk: return,
// We've chosen a new reuseport sock which is never refcounted. This
// implies that sk also isn't refcounted.
//
    pub reuse_sk: return,
    pub skb_dst_dev_net_rcu(skb): *mut *mut net net =,
    pub ip_hdr(skb): *const *const iphdr iph =,
    pub sk: *mut sock,
    pub inet_ehashfn): refcounted,,
    pub NULL: return,
    pub sk: return,
    pub /: *mut *mut sk->sk_daddr = addr; / alias of inet_daddr,

    pub &sk->sk_v6_daddr): ipv6_addr_set_v4mapped(addr,,

    pub /: *mut *mut sk->sk_rcv_saddr = addr; / alias of inet_rcv_saddr,

    pub &sk->sk_v6_rcv_saddr): ipv6_addr_set_v4mapped(addr,,

    pub hash)): u32,
    pub sk): *mut sock,
