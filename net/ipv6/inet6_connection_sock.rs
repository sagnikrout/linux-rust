//! Automatically rewritten from C to Rust
//! Source: net/ipv6/inet6_connection_sock.c
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
// INET        An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Support for INET6 connection oriented protocols.
//
// Authors:    See the TCPv6 sources
//

    struct dst_entry *inet6_csk_route_req(const struct sock *sk,
    struct dst_entry *dst,
    struct flowi6 *fl6,
    const struct request_sock *req,
    u8 proto)
    {
    const struct inet_request_sock *ireq = inet_rsk(req);
    const struct ipv6_pinfo *np = inet6_sk(sk);
    struct in6_addr *final_p, final;
    memset(fl6, 0, sizeof(*fl6));
    fl6.flowi6_proto = proto;
    fl6.daddr = ireq.ir_v6_rmt_addr;
    rcu_read_lock();
    final_p = fl6_update_dst(fl6, rcu_dereference(np.opt), &final);
    rcu_read_unlock();
    fl6.saddr = ireq.ir_v6_loc_addr;
    fl6.flowi6_oif = ireq.ir_iif;
    fl6.flowi6_mark = ireq.ir_mark;
    fl6.fl6_dport = ireq.ir_rmt_port;
    fl6.fl6_sport = htons(ireq.ir_num);
    fl6.flowi6_uid = sk_uid(sk);
    security_req_classify_flow(req, flowi6_to_flowi_common(fl6));
    ip6_ecmp_set_mp_hash(sock_net(sk), fl6, tcp_rsk(req).txhash);
    if (!dst) {
    dst = ip6_dst_lookup_flow(sock_net(sk), sk, fl6, final_p);
    if (IS_ERR(dst))
    return core::ptr::null_mut();
    }
    return dst;
    }
    struct dst_entry *inet6_csk_route_socket(struct sock *sk,
    struct flowi6 *fl6)
    {
    struct inet_sock *inet = inet_sk(sk);
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct in6_addr *final_p;
    struct dst_entry *dst;
    memset(fl6, 0, sizeof(*fl6));
    fl6.flowi6_proto = sk.sk_protocol;
    fl6.daddr = sk.sk_v6_daddr;
    fl6.saddr = np.saddr;
    fl6.flowlabel = np.flow_label;
    IP6_ECN_flow_xmit(sk, fl6.flowlabel);
    if (sk.sk_protocol == IPPROTO_TCP)
    ip6_ecmp_set_mp_hash(sock_net(sk), fl6, sk.sk_txhash);
    fl6.flowi6_oif = sk.sk_bound_dev_if;
    fl6.flowi6_mark = sk.sk_mark;
    fl6.fl6_sport = inet.inet_sport;
    fl6.fl6_dport = inet.inet_dport;
    fl6.flowi6_uid = sk_uid(sk);
    security_sk_classify_flow(sk, flowi6_to_flowi_common(fl6));
    rcu_read_lock();
    final_p = fl6_update_dst(fl6, rcu_dereference(np.opt), &np.final);
    rcu_read_unlock();
    dst = ip6_dst_lookup_flow(sock_net(sk), sk, fl6, final_p);
    if (!IS_ERR(dst))
    ip6_dst_store(sk, dst, false, false);
    return dst;
    }
#[no_mangle]
pub unsafe extern "C" fn inet6_csk_xmit(sk: *mut sock, skb: *mut sk_buff, fl_unused: *mut flowi) -> c_int {
    int inet6_csk_xmit(struct sock *sk, struct sk_buff *skb, struct flowi *fl_unused)
    {
    struct flowi6 *fl6 = &inet_sk(sk).cork.fl.u.ip6;
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct dst_entry *dst;
    int res;
    dst = __sk_dst_check(sk, np.dst_cookie);
    if (unlikely(!dst)) {
    dst = inet6_csk_route_socket(sk, fl6);
    if (IS_ERR(dst)) {
    WRITE_ONCE(sk.sk_err_soft, -PTR_ERR(dst));
    sk.sk_route_caps = 0;
    sk_skb_reason_drop(sk, skb,
    SKB_DROP_REASON_IP_OUTNOROUTES);
    return PTR_ERR(dst);
    }
// Restore final destination back after routing done
    fl6.daddr = sk.sk_v6_daddr;
    }
    rcu_read_lock();
    skb_dst_set_noref(skb, dst);
    res = ip6_xmit(sk, skb, fl6, sk.sk_mark, rcu_dereference(np.opt),
    np.tclass, READ_ONCE(sk.sk_priority));
    rcu_read_unlock();
    return res;
    }
    EXPORT_SYMBOL_GPL(inet6_csk_xmit);
