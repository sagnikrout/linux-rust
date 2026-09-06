//! Automatically rewritten from C to Rust
//! Source: net/ipv4/datagram.c
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
// common UDP/RAW code
// Linux INET implementation
//
// Authors:
// Hideaki YOSHIFUJI <yoshfuji@linux-ipv6.org>
//

#[no_mangle]
pub unsafe extern "C" fn __ip4_datagram_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int {
    int __ip4_datagram_connect(struct sock *sk, struct sockaddr_unsized *uaddr, int addr_len)
    {
    struct inet_sock *inet = inet_sk(sk);
    struct sockaddr_in *usin = (struct sockaddr_in *) uaddr;
    struct flowi4 *fl4;
    struct rtable *rt;
    __be32 saddr;
    int oif;
    int err;
    if (addr_len < sizeof(*usin))
    return -EINVAL;
    if (usin.sin_family != AF_INET)
    return -EAFNOSUPPORT;
    sk_dst_reset(sk);
    oif = sk.sk_bound_dev_if;
    saddr = inet.inet_saddr;
    if (ipv4_is_multicast(usin.sin_addr.s_addr)) {
    if (!oif || netif_index_is_l3_master(sock_net(sk), oif))
    oif = READ_ONCE(inet.mc_index);
    if (!saddr)
    saddr = READ_ONCE(inet.mc_addr);
    } else if (!oif) {
    oif = READ_ONCE(inet.uc_index);
    }
    fl4 = &inet.cork.fl.u.ip4;
    rt = ip_route_connect(fl4, usin.sin_addr.s_addr, saddr, oif,
    sk.sk_protocol, inet.inet_sport,
    usin.sin_port, sk);
    if (IS_ERR(rt)) {
    err = PTR_ERR(rt);
    if (err == -ENETUNREACH)
    IP_INC_STATS(sock_net(sk), IPSTATS_MIB_OUTNOROUTES);
    goto out;
    }
    if ((rt.rt_flags & RTCF_BROADCAST) && !sock_flag(sk, SOCK_BROADCAST)) {
    ip_rt_put(rt);
    err = -EACCES;
    goto out;
    }
// Update addresses before rehashing
    WRITE_ONCE(inet.inet_daddr, fl4.daddr);
    inet.inet_dport = usin.sin_port;
    if (!inet.inet_saddr)
    inet.inet_saddr = fl4.saddr;
    if (!inet.inet_rcv_saddr) {
    WRITE_ONCE(inet.inet_rcv_saddr, fl4.saddr);
    if (sk.sk_prot.rehash)
    sk.sk_prot.rehash(sk);
    }
    reuseport_has_conns_set(sk);
    sk.sk_state = TCP_ESTABLISHED;
    sk_set_txhash(sk);
    atomic_set(&inet.inet_id, get_random_u16());
    sk_dst_set(sk, &rt.dst);
    err = 0;
    out:
    return err;
    }
    EXPORT_SYMBOL(__ip4_datagram_connect);
#[no_mangle]
pub unsafe extern "C" fn ip4_datagram_connect(sk: *mut sock, uaddr: *mut sockaddr_unsized, addr_len: c_int) -> c_int {
    int ip4_datagram_connect(struct sock *sk, struct sockaddr_unsized *uaddr, int addr_len)
    {
    int res;
    lock_sock(sk);
    res = __ip4_datagram_connect(sk, uaddr, addr_len);
    release_sock(sk);
    return res;
    }
    EXPORT_SYMBOL(ip4_datagram_connect);
// Because UDP xmit path can manipulate sk_dst_cache without holding
// socket lock, we need to use sk_dst_set() here,
// even if we own the socket lock.
//
#[no_mangle]
pub unsafe extern "C" fn ip4_datagram_release_cb(sk: *mut sock) {
    void ip4_datagram_release_cb(struct sock *sk)
    {
    const struct inet_sock *inet = inet_sk(sk);
    struct dst_entry *dst;
    struct flowi4 fl4;
    struct rtable *rt;
    rcu_read_lock();
    dst = __sk_dst_get(sk);
    if (!dst || !READ_ONCE(dst.obsolete) || dst.ops.check(dst, 0)) {
    rcu_read_unlock();
    return;
    }
    inet_sk_init_flowi4(inet, &fl4);
    rt = ip_route_output_flow(sock_net(sk), &fl4, sk);
    dst = !IS_ERR(rt) ? &rt.dst : core::ptr::null_mut();
    sk_dst_set(sk, dst);
    rcu_read_unlock();
    }
    EXPORT_SYMBOL_GPL(ip4_datagram_release_cb);
