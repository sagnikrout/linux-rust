//! Automatically rewritten from C to Rust
//! Source: net/ipv6/ping.c
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
// "Ping" sockets
//
// Based on ipv4/ping.c code.
//
// Authors:	Lorenzo Colitti (IPv6 support)
// Vasiliy Kulikov / Openwall (IPv4 implementation, for Linux 2.6),
// Pavel Kankovsky (IPv4 implementation, for Linux 2.4.32)
//

// Compatibility glue so we can support IPv6 when it's compiled as a module
#[no_mangle]
unsafe extern "C" fn dummy_ipv6_recv_error(sk: *mut sock, msg: *mut msghdr, len: c_int) -> c_int {
    static int dummy_ipv6_recv_error(struct sock *sk, struct msghdr *msg, int len)
    {
    return -EAFNOSUPPORT;
    }
    static void dummy_ip6_datagram_recv_ctl(struct sock *sk, struct msghdr *msg,
    struct sk_buff *skb)
    {
    }
#[no_mangle]
unsafe extern "C" fn dummy_icmpv6_err_convert(type: u8, code: u8, err: *mut c_int) -> c_int {
    static int dummy_icmpv6_err_convert(u8 type, u8 code, int *err)
    {
    return -EAFNOSUPPORT;
    }
    static void dummy_ipv6_icmp_error(struct sock *sk, struct sk_buff *skb, int err,
    __be16 port, u32 info, u8 *payload) {}
    static int dummy_ipv6_chk_addr(struct net *net, const struct in6_addr *addr,
    const struct net_device *dev, int strict)
    {
    return 0;
    }
    static int ping_v6_pre_connect(struct sock *sk, struct sockaddr_unsized *uaddr,
    int addr_len)
    {
// This check is replicated from __ip6_datagram_connect() and
// intended to prevent BPF program called below from accessing
// bytes that are out of the bound specified by user in addr_len.
//
    if (addr_len < SIN6_LEN_RFC2133)
    return -EINVAL;
    return BPF_CGROUP_RUN_PROG_INET6_CONNECT_LOCK(sk, uaddr, &addr_len);
    }
#[no_mangle]
unsafe extern "C" fn ping_v6_sendmsg(sk: *mut sock, msg: *mut msghdr, len: usize) -> c_int {
    static int ping_v6_sendmsg(struct sock *sk, struct msghdr *msg, size_t len)
    {
    struct inet_sock *inet = inet_sk(sk);
    struct ipv6_pinfo *np = inet6_sk(sk);
    struct icmp6hdr user_icmph;
    int addr_type;
    struct in6_addr *daddr;
    let mut oif: c_int = 0;
    struct flowi6 fl6;
    int err;
    struct dst_entry *dst;
    struct rt6_info *rt;
    struct pingfakehdr pfh;
    struct ipcm6_cookie ipc6;
    err = ping_common_sendmsg(AF_INET6, msg, len, &user_icmph,
    sizeof(user_icmph));
    if (err)
    return err;
    memset(&fl6, 0, sizeof(fl6));
    if (msg.msg_name) {
    DECLARE_SOCKADDR(struct sockaddr_in6 *, u, msg.msg_name);
    if (msg.msg_namelen < sizeof(*u))
    return -EINVAL;
    if (u.sin6_family != AF_INET6) {
    return -EAFNOSUPPORT;
    }
    daddr = &(u.sin6_addr);
    if (inet6_test_bit(SNDFLOW, sk))
    fl6.flowlabel = u.sin6_flowinfo & IPV6_FLOWINFO_MASK;
    if (__ipv6_addr_needs_scope_id(ipv6_addr_type(daddr)))
    oif = u.sin6_scope_id;
    } else {
    if (sk.sk_state != TCP_ESTABLISHED)
    return -EDESTADDRREQ;
    daddr = &sk.sk_v6_daddr;
    fl6.flowlabel = np.flow_label;
    }
    if (!oif)
    oif = sk.sk_bound_dev_if;
    if (!oif)
    oif = np.sticky_pktinfo.ipi6_ifindex;
    if (!oif && ipv6_addr_is_multicast(daddr))
    oif = READ_ONCE(np.mcast_oif);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !oif) -> else {
    else if (!oif)
    oif = READ_ONCE(np.ucast_oif);
    addr_type = ipv6_addr_type(daddr);
    if ((__ipv6_addr_needs_scope_id(addr_type) && !oif) ||
    (addr_type & IPV6_ADDR_MAPPED) ||
    (oif && sk.sk_bound_dev_if && oif != sk.sk_bound_dev_if &&
    l3mdev_master_ifindex_by_index(sock_net(sk), oif) != sk.sk_bound_dev_if))
    return -EINVAL;
    ipcm6_init_sk(&ipc6, sk);
    fl6.flowi6_oif = oif;
    if (msg.msg_controllen) {
    let mut opt: ipv6_txoptions = {};
    opt.tot_len = sizeof(opt);
    ipc6.opt = &opt;
    err = ip6_datagram_send_ctl(sock_net(sk), sk, msg, &fl6, &ipc6);
    if (err < 0)
    return err;
// Changes to txoptions and flow info are not implemented, yet.
// Drop the options.
//
    ipc6.opt = core::ptr::null_mut();
    }
    fl6.flowi6_proto = IPPROTO_ICMPV6;
    fl6.saddr = np.saddr;
    fl6.daddr = *daddr;
    fl6.flowi6_mark = ipc6.sockc.mark;
    fl6.flowi6_uid = sk_uid(sk);
    fl6.fl6_icmp_type = user_icmph.icmp6_type;
    fl6.fl6_icmp_code = user_icmph.icmp6_code;
    security_sk_classify_flow(sk, flowi6_to_flowi_common(&fl6));
    fl6.flowlabel = ip6_make_flowinfo(ipc6.tclass, fl6.flowlabel);
    dst = ip6_sk_dst_lookup_flow(sk, &fl6, daddr, false);
    if (IS_ERR(dst))
    return PTR_ERR(dst);
    rt = dst_rt6_info(dst);
    if (!fl6.flowi6_oif && ipv6_addr_is_multicast(&fl6.daddr))
    fl6.flowi6_oif = READ_ONCE(np.mcast_oif);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !fl6.flowi6_oif) -> else {
    else if (!fl6.flowi6_oif)
    fl6.flowi6_oif = READ_ONCE(np.ucast_oif);
    pfh.icmph.type = user_icmph.icmp6_type;
    pfh.icmph.code = user_icmph.icmp6_code;
    pfh.icmph.checksum = 0;
    pfh.icmph.un.echo.id = inet.inet_sport;
    pfh.icmph.un.echo.sequence = user_icmph.icmp6_sequence;
    pfh.msg = msg;
    pfh.wcheck = 0;
    pfh.family = AF_INET6;
    if (ipc6.hlimit < 0)
    ipc6.hlimit = ip6_sk_dst_hoplimit(np, &fl6, dst);
    lock_sock(sk);
    err = ip6_append_data(sk, ping_getfrag, &pfh, len,
    sizeof(struct icmp6hdr), &ipc6, &fl6, rt,
    MSG_DONTWAIT);
    if (err) {
    ICMP6_INC_STATS(sock_net(sk), rt.rt6i_idev,
    ICMP6_MIB_OUTERRORS);
    ip6_flush_pending_frames(sk);
    } else {
    icmpv6_push_pending_frames(sk, &fl6,
    (struct icmp6hdr *)&pfh.icmph, len);
    }
    release_sock(sk);
    dst_release(dst);
    if (err)
    return err;
    return len;
    }
    struct proto pingv6_prot = {
    .name =		"PINGv6",
    .owner =	THIS_MODULE,
    .init =		ping_init_sock,
    .close =	ping_close,
    .pre_connect =	ping_v6_pre_connect,
    .connect =	ip6_datagram_connect_v6_only,
    .disconnect =	__udp_disconnect,
    .setsockopt =	ipv6_setsockopt,
    .getsockopt =	ipv6_getsockopt,
    .sendmsg =	ping_v6_sendmsg,
    .recvmsg =	ping_recvmsg,
    .bind =		ping_bind,
    .backlog_rcv =	ping_queue_rcv_skb,
    .unhash =	ping_unhash,
    .get_port =	ping_get_port,
    .put_port =	ping_unhash,
    .obj_size =	sizeof(struct raw6_sock),
    .ipv6_pinfo_offset = offsetof(struct raw6_sock, inet6),
    };
    EXPORT_SYMBOL_GPL(pingv6_prot);
    static struct inet_protosw pingv6_protosw = {
    .type =      SOCK_DGRAM,
    .protocol =  IPPROTO_ICMPV6,
    .prot =      &pingv6_prot,
    .ops =       &inet6_sockraw_ops,
    .flags =     INET_PROTOSW_REUSE,
    };

    static void *ping_v6_seq_start(struct seq_file *seq, loff_t *pos)
    {
    return ping_seq_start(seq, pos, AF_INET6);
    }
#[no_mangle]
unsafe extern "C" fn ping_v6_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int {
    static int ping_v6_seq_show(struct seq_file *seq, void *v)
    {
    if (v == SEQ_START_TOKEN) {
    seq_puts(seq, IPV6_SEQ_DGRAM_HEADER);
    } else {
    let mut bucket: c_int = ((struct ping_iter_state *) seq.private).bucket;
    struct inet_sock *inet = inet_sk((struct sock *)v);
    let mut srcp: __u16 = ntohs(inet.inet_sport);
    let mut destp: __u16 = ntohs(inet.inet_dport);
    ip6_dgram_sock_seq_show(seq, v, srcp, destp, bucket);
    }
    return 0;
    }
    static const struct seq_operations ping_v6_seq_ops = {
    .start		= ping_v6_seq_start,
    .show		= ping_v6_seq_show,
    .next		= ping_seq_next,
    .stop		= ping_seq_stop,
    };
#[no_mangle]
unsafe extern "C" fn ping_v6_proc_init_net(net: *mut net) -> int __net_init {
    static int __net_init ping_v6_proc_init_net(struct net *net)
    {
    if (!proc_create_net("icmp6", 0444, net.proc_net, &ping_v6_seq_ops,
    sizeof(struct ping_iter_state)))
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ping_v6_proc_exit_net(net: *mut net) -> void __net_exit {
    static void __net_exit ping_v6_proc_exit_net(struct net *net)
    {
    remove_proc_entry("icmp6", net.proc_net);
    }
    static struct pernet_operations ping_v6_net_ops = {
    .init = ping_v6_proc_init_net,
    .exit = ping_v6_proc_exit_net,
    };

#[no_mangle]
pub unsafe extern "C" fn pingv6_init() -> int __init {
    int __init pingv6_init(void)
    {

    let mut ret: c_int = register_pernet_subsys(&ping_v6_net_ops);
    if (ret)
    return ret;

    pingv6_ops.ipv6_recv_error = ipv6_recv_error;
    pingv6_ops.ip6_datagram_recv_common_ctl = ip6_datagram_recv_common_ctl;
    pingv6_ops.ip6_datagram_recv_specific_ctl =
    ip6_datagram_recv_specific_ctl;
    pingv6_ops.icmpv6_err_convert = icmpv6_err_convert;
    pingv6_ops.ipv6_icmp_error = ipv6_icmp_error;
    pingv6_ops.ipv6_chk_addr = ipv6_chk_addr;
    return inet6_register_protosw(&pingv6_protosw);
    }
// This never gets called because it's not possible to unload the ipv6 module,
// but just in case.
//
#[no_mangle]
pub unsafe extern "C" fn pingv6_exit() {
    void pingv6_exit(void)
    {
    pingv6_ops.ipv6_recv_error = dummy_ipv6_recv_error;
    pingv6_ops.ip6_datagram_recv_common_ctl = dummy_ip6_datagram_recv_ctl;
    pingv6_ops.ip6_datagram_recv_specific_ctl = dummy_ip6_datagram_recv_ctl;
    pingv6_ops.icmpv6_err_convert = dummy_icmpv6_err_convert;
    pingv6_ops.ipv6_icmp_error = dummy_ipv6_icmp_error;
    pingv6_ops.ipv6_chk_addr = dummy_ipv6_chk_addr;

    unregister_pernet_subsys(&ping_v6_net_ops);

    inet6_unregister_protosw(&pingv6_protosw);
    }
