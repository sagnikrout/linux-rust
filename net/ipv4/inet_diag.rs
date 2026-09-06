//! Automatically rewritten from C to Rust
//! Source: net/ipv4/inet_diag.c
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
// inet_diag.c	Module for monitoring INET transport protocols sockets.
//
// Authors:	Alexey Kuznetsov, <kuznet@ms2.inr.ac.ru>
//

    static const struct inet_diag_handler __rcu **inet_diag_table;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inet_diag_entry {
    pub saddr: *const __be32,
    pub daddr: *const __be32,
    pub sport: u16,
    pub dport: u16,
    pub family: u16,
    pub userlocks: u16,
    pub ifindex: u32,
    pub mark: u32,

    pub cgroup_id: u64,

}

    static const struct inet_diag_handler *inet_diag_lock_handler(int proto)
    {
    const struct inet_diag_handler *handler;
    if (proto < 0 || proto >= IPPROTO_MAX)
    return core::ptr::null_mut();
    if (!READ_ONCE(inet_diag_table[proto]))
    sock_load_diag_module(AF_INET, proto);
    rcu_read_lock();
    handler = rcu_dereference(inet_diag_table[proto]);
    if (handler && !try_module_get(handler.owner))
    handler = core::ptr::null_mut();
    rcu_read_unlock();
    return handler;
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_unlock_handler(handler: *const inet_diag_handler) {
    static void inet_diag_unlock_handler(const struct inet_diag_handler *handler)
    {
    module_put(handler.owner);
    }
#[no_mangle]
pub unsafe extern "C" fn inet_diag_msg_common_fill(r: *mut inet_diag_msg, sk: *mut sock) {
    void inet_diag_msg_common_fill(struct inet_diag_msg *r, struct sock *sk)
    {
    r.idiag_family = READ_ONCE(sk.sk_family);
    r.id.idiag_sport = htons(READ_ONCE(sk.sk_num));
    r.id.idiag_dport = READ_ONCE(sk.sk_dport);
    r.id.idiag_if = READ_ONCE(sk.sk_bound_dev_if);
    sock_diag_save_cookie(sk, r.id.idiag_cookie);

    if (r.idiag_family == AF_INET6) {
    data_race(*(struct in6_addr *)r.id.idiag_src = sk.sk_v6_rcv_saddr);
    data_race(*(struct in6_addr *)r.id.idiag_dst = sk.sk_v6_daddr);
    } else

    {
    memset(&r.id.idiag_src, 0, sizeof(r.id.idiag_src));
    memset(&r.id.idiag_dst, 0, sizeof(r.id.idiag_dst));
    r.id.idiag_src[0] = READ_ONCE(sk.sk_rcv_saddr);
    r.id.idiag_dst[0] = READ_ONCE(sk.sk_daddr);
    }
    }
    EXPORT_SYMBOL_GPL(inet_diag_msg_common_fill);
    int inet_diag_msg_attrs_fill(struct sock *sk, struct sk_buff *skb,
    struct inet_diag_msg *r, int ext,
    struct user_namespace *user_ns,
    bool net_admin)
    {
    const struct inet_sock *inet = inet_sk(sk);
    struct inet_diag_sockopt inet_sockopt;
    if (nla_put_u8(skb, INET_DIAG_SHUTDOWN, sk.sk_shutdown))
    goto errout;
// IPv6 dual-stack sockets use inet->tos for IPv4 connections,
// hence this needs to be included regardless of socket family.
//
    if (ext & (1 << (INET_DIAG_TOS - 1)))
    if (nla_put_u8(skb, INET_DIAG_TOS, READ_ONCE(inet.tos)) < 0)
    goto errout;

    if (r.idiag_family == AF_INET6) {
    if (ext & (1 << (INET_DIAG_TCLASS - 1)))
    if (nla_put_u8(skb, INET_DIAG_TCLASS,
    inet6_sk(sk).tclass) < 0)
    goto errout;
    if (((1 << sk.sk_state) & (TCPF_LISTEN | TCPF_CLOSE)) &&
    nla_put_u8(skb, INET_DIAG_SKV6ONLY, ipv6_only_sock(sk)))
    goto errout;
    }

    if (net_admin && nla_put_u32(skb, INET_DIAG_MARK, READ_ONCE(sk.sk_mark)))
    goto errout;
    if (ext & (1 << (INET_DIAG_CLASS_ID - 1)) ||
    ext & (1 << (INET_DIAG_TCLASS - 1))) {
    let mut classid: u32 = 0;

    classid = sock_cgroup_classid(&sk.sk_cgrp_data);

// Fallback to socket priority if class id isn't set.
// Classful qdiscs use it as direct reference to class.
// For cgroup2 classid is always zero.
//
    if (!classid)
    classid = READ_ONCE(sk.sk_priority);
    if (nla_put_u32(skb, INET_DIAG_CLASS_ID, classid))
    goto errout;
    }

    if (nla_put_u64_64bit(skb, INET_DIAG_CGROUP_ID,
    cgroup_id(sock_cgroup_ptr(&sk.sk_cgrp_data)),
    INET_DIAG_PAD))
    goto errout;

    r.idiag_uid = from_kuid_munged(user_ns, sk_uid(sk));
    r.idiag_inode = sock_i_ino(sk);
    memset(&inet_sockopt, 0, sizeof(inet_sockopt));
    inet_sockopt.recverr	= inet_test_bit(RECVERR, sk);
    inet_sockopt.is_icsk	= inet_test_bit(IS_ICSK, sk);
    inet_sockopt.freebind	= inet_test_bit(FREEBIND, sk);
    inet_sockopt.hdrincl	= inet_test_bit(HDRINCL, sk);
    inet_sockopt.mc_loop	= inet_test_bit(MC_LOOP, sk);
    inet_sockopt.transparent = inet_test_bit(TRANSPARENT, sk);
    inet_sockopt.mc_all	= inet_test_bit(MC_ALL, sk);
    inet_sockopt.nodefrag	= inet_test_bit(NODEFRAG, sk);
    inet_sockopt.bind_address_no_port = inet_test_bit(BIND_ADDRESS_NO_PORT, sk);
    inet_sockopt.recverr_rfc4884 = inet_test_bit(RECVERR_RFC4884, sk);
    inet_sockopt.defer_connect = inet_test_bit(DEFER_CONNECT, sk);
    if (nla_put(skb, INET_DIAG_SOCKOPT, sizeof(inet_sockopt),
    &inet_sockopt))
    goto errout;
    return 0;
    errout:
    return 1;
    }
    EXPORT_SYMBOL_GPL(inet_diag_msg_attrs_fill);
    static int inet_diag_parse_attrs(const struct nlmsghdr *nlh, int hdrlen,
    struct nlattr **req_nlas)
    {
    struct nlattr *nla;
    int remaining;
    nlmsg_for_each_attr(nla, nlh, hdrlen, remaining) {
    let mut type: c_int = nla_type(nla);
    if (type == INET_DIAG_REQ_PROTOCOL && nla_len(nla) != sizeof(u32))
    return -EINVAL;
    if (type < __INET_DIAG_REQ_MAX)
    req_nlas[type] = nla;
    }
    return 0;
    }
    static int inet_diag_get_protocol(const struct inet_diag_req_v2 *req,
    const struct inet_diag_dump_data *data)
    {
    if (data.req_nlas[INET_DIAG_REQ_PROTOCOL])
    return nla_get_u32(data.req_nlas[INET_DIAG_REQ_PROTOCOL]);
    return req.sdiag_protocol;
    }

    int inet_sk_diag_fill(struct sock *sk, struct inet_connection_sock *icsk,
    struct sk_buff *skb, struct netlink_callback *cb,
    const struct inet_diag_req_v2 *req,
    u16 nlmsg_flags, bool net_admin)
    {
    const struct tcp_congestion_ops *ca_ops;
    const struct inet_diag_handler *handler;
    struct inet_diag_dump_data *cb_data;
    let mut ext: c_int = req.idiag_ext;
    struct inet_diag_msg *r;
    struct nlmsghdr  *nlh;
    struct nlattr *attr;
    void *info = core::ptr::null_mut();
    u8 icsk_pending;
    int protocol;
    cb_data = cb.data;
    protocol = inet_diag_get_protocol(req, cb_data);
// inet_diag_lock_handler() made sure inet_diag_table[] is stable.
    handler = rcu_dereference_protected(inet_diag_table[protocol], 1);
    DEBUG_NET_WARN_ON_ONCE(!handler);
    if (!handler)
    return -ENXIO;
    nlh = nlmsg_put(skb, NETLINK_CB(cb.skb).portid, cb.nlh.nlmsg_seq,
    cb.nlh.nlmsg_type, sizeof(*r), nlmsg_flags);
    if (!nlh)
    return -EMSGSIZE;
    r = nlmsg_data(nlh);
    BUG_ON(!sk_fullsock(sk));
    inet_diag_msg_common_fill(r, sk);
    r.idiag_state = sk.sk_state;
    r.idiag_timer = IDIAG_TIMER_OFF;
    r.idiag_retrans = 0;
    r.idiag_expires = 0;
    if (inet_diag_msg_attrs_fill(sk, skb, r, ext,
    sk_user_ns(NETLINK_CB(cb.skb).sk),
    net_admin))
    goto errout;
    if (ext & (1 << (INET_DIAG_MEMINFO - 1))) {
    struct inet_diag_meminfo minfo = {
    .idiag_rmem = sk_rmem_alloc_get(sk),
    .idiag_wmem = READ_ONCE(sk.sk_wmem_queued),
    .idiag_fmem = READ_ONCE(sk.sk_forward_alloc),
    .idiag_tmem = sk_wmem_alloc_get(sk),
    };
    if (nla_put(skb, INET_DIAG_MEMINFO, sizeof(minfo), &minfo) < 0)
    goto errout;
    }
    if (ext & (1 << (INET_DIAG_SKMEMINFO - 1)))
    if (sock_diag_put_meminfo(sk, skb, INET_DIAG_SKMEMINFO))
    goto errout;
//
// RAW sockets might have user-defined protocols assigned,
// so report the one supplied on socket creation.
//
    if (sk.sk_type == SOCK_RAW) {
    if (nla_put_u8(skb, INET_DIAG_PROTOCOL, sk.sk_protocol))
    goto errout;
    }
    if (!icsk) {
    handler.idiag_get_info(sk, r, core::ptr::null_mut());
    goto out;
    }
    icsk_pending = smp_load_acquire(&icsk.icsk_pending);
    if (icsk_pending == ICSK_TIME_RETRANS ||
    icsk_pending == ICSK_TIME_REO_TIMEOUT ||
    icsk_pending == ICSK_TIME_LOSS_PROBE) {
    r.idiag_timer = IDIAG_TIMER_ON;
    r.idiag_retrans = READ_ONCE(icsk.icsk_retransmits);
    r.idiag_expires =
    jiffies_delta_to_msecs(tcp_timeout_expires(sk) - jiffies);
    } else if (icsk_pending == ICSK_TIME_PROBE0) {
    r.idiag_timer = IDIAG_TIMER_PROBE0;
    r.idiag_retrans = READ_ONCE(icsk.icsk_probes_out);
    r.idiag_expires =
    jiffies_delta_to_msecs(tcp_timeout_expires(sk) - jiffies);
    } else if (timer_pending(&icsk.icsk_keepalive_timer)) {
    r.idiag_timer = IDIAG_TIMER_KEEPALIVE;
    r.idiag_retrans = READ_ONCE(icsk.icsk_probes_out);
    r.idiag_expires =
    jiffies_delta_to_msecs(icsk.icsk_keepalive_timer.expires - jiffies);
    } else if ((READ_ONCE(icsk.icsk_ack.pending) & ICSK_ACK_TIMER) &&
    timer_pending(&icsk.icsk_delack_timer)) {
    r.idiag_timer = IDIAG_TIMER_DELACK;
    r.idiag_expires =
    jiffies_delta_to_msecs(icsk_delack_timeout(icsk) - jiffies);
    }
    if ((ext & (1 << (INET_DIAG_INFO - 1))) && handler.idiag_info_size) {
    attr = nla_reserve_64bit(skb, INET_DIAG_INFO,
    handler.idiag_info_size,
    INET_DIAG_PAD);
    if (!attr)
    goto errout;
    info = nla_data(attr);
    }
    if (ext & (1 << (INET_DIAG_CONG - 1))) {
    let mut err: c_int = 0;
    rcu_read_lock();
    ca_ops = READ_ONCE(icsk.icsk_ca_ops);
    if (ca_ops)
    err = nla_put_string(skb, INET_DIAG_CONG, ca_ops.name);
    rcu_read_unlock();
    if (err < 0)
    goto errout;
    }
    handler.idiag_get_info(sk, r, info);
    if (ext & (1 << (INET_DIAG_INFO - 1)) && handler.idiag_get_aux)
    if (handler.idiag_get_aux(sk, net_admin, skb) < 0)
    goto errout;
    if (sk.sk_state < TCP_TIME_WAIT) {
    union tcp_cc_info info;
    let mut sz: usize = 0;
    int attr;
    rcu_read_lock();
    ca_ops = READ_ONCE(icsk.icsk_ca_ops);
    if (ca_ops && ca_ops.get_info)
    sz = ca_ops.get_info(sk, ext, &attr, &info);
    rcu_read_unlock();
    if (sz && nla_put(skb, attr, sz, &info) < 0)
    goto errout;
    }
// Keep it at the end for potential retry with a larger skb,
// or else do best-effort fitting, which is only done for the
// first_nlmsg.
//
    if (cb_data.bpf_stg_diag) {
    let mut first_nlmsg: bool = ((unsigned char *)nlh == skb.data);
    unsigned int prev_min_dump_alloc;
    let mut total_nla_size: c_uint = 0;
    unsigned int msg_len;
    int err;
    msg_len = skb_tail_pointer(skb) - (unsigned char *)nlh;
    err = bpf_sk_storage_diag_put(cb_data.bpf_stg_diag, sk, skb,
    INET_DIAG_SK_BPF_STORAGES,
    &total_nla_size);
    if (!err)
    goto out;
    total_nla_size += msg_len;
    prev_min_dump_alloc = cb.min_dump_alloc;
    if (total_nla_size > prev_min_dump_alloc)
    cb.min_dump_alloc = min_t(u32, total_nla_size,
    MAX_DUMP_ALLOC_SIZE);
    if (!first_nlmsg)
    goto errout;
    if (cb.min_dump_alloc > prev_min_dump_alloc)
// Retry with pskb_expand_head() with
// __GFP_DIRECT_RECLAIM
//
    goto errout;
    WARN_ON_ONCE(total_nla_size <= prev_min_dump_alloc);
// Send what we have for this sk
// and move on to the next sk in the following
// dump()
//
    }
    out:
    nlmsg_end(skb, nlh);
    return 0;
    errout:
    nlmsg_cancel(skb, nlh);
    return -EMSGSIZE;
    }
    EXPORT_SYMBOL_GPL(inet_sk_diag_fill);
    static int inet_diag_cmd_exact(int cmd, struct sk_buff *in_skb,
    const struct nlmsghdr *nlh,
    int hdrlen,
    const struct inet_diag_req_v2 *req)
    {
    const struct inet_diag_handler *handler;
    struct inet_diag_dump_data dump_data;
    int err, protocol;
    memset(&dump_data, 0, sizeof(dump_data));
    err = inet_diag_parse_attrs(nlh, hdrlen, dump_data.req_nlas);
    if (err)
    return err;
    protocol = inet_diag_get_protocol(req, &dump_data);
    handler = inet_diag_lock_handler(protocol);
    if (!handler)
    return -ENOENT;
    if (cmd == SOCK_DIAG_BY_FAMILY) {
    struct netlink_callback cb = {
    .nlh = nlh,
    .skb = in_skb,
    .data = &dump_data,
    };
    err = handler.dump_one(&cb, req);
    } else if (cmd == SOCK_DESTROY && handler.destroy) {
    err = handler.destroy(in_skb, req);
    } else {
    err = -EOPNOTSUPP;
    }
    inet_diag_unlock_handler(handler);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn bitstring_match(a1: *const __be32, a2: *const __be32, bits: c_int) -> c_int {
    static int bitstring_match(const __be32 *a1, const __be32 *a2, int bits)
    {
    let mut words: c_int = bits >> 5;
    bits &= 0x1f;
    if (words) {
    if (memcmp(a1, a2, words << 2))
    return 0;
    }
    if (bits) {
    __be32 w1, w2;
    __be32 mask;
    w1 = a1[words];
    w2 = a2[words];
    mask = htonl((0xffffffff) << (32 - bits));
    if ((w1 ^ w2) & mask)
    return 0;
    }
    return 1;
    }
    static int inet_diag_bc_run(const struct nlattr *_bc,
    const struct inet_diag_entry *entry)
    {
    const void *bc = nla_data(_bc);
    let mut len: c_int = nla_len(_bc);
    while (len > 0) {
    let mut yes: c_int = 1;
    const struct inet_diag_bc_op *op = bc;
    switch (op.code) {
    case INET_DIAG_BC_NOP:
    break;
    case INET_DIAG_BC_JMP:
    yes = 0;
    break;
    case INET_DIAG_BC_S_EQ:
    yes = entry.sport == op[1].no;
    break;
    case INET_DIAG_BC_S_GE:
    yes = entry.sport >= op[1].no;
    break;
    case INET_DIAG_BC_S_LE:
    yes = entry.sport <= op[1].no;
    break;
    case INET_DIAG_BC_D_EQ:
    yes = entry.dport == op[1].no;
    break;
    case INET_DIAG_BC_D_GE:
    yes = entry.dport >= op[1].no;
    break;
    case INET_DIAG_BC_D_LE:
    yes = entry.dport <= op[1].no;
    break;
    case INET_DIAG_BC_AUTO:
    yes = !(entry.userlocks & SOCK_BINDPORT_LOCK);
    break;
    case INET_DIAG_BC_S_COND:
    case INET_DIAG_BC_D_COND: {
    const struct inet_diag_hostcond *cond;
    const __be32 *addr;
    cond = (const struct inet_diag_hostcond *)(op + 1);
    if (cond.port != -1 &&
    cond.port != (op.code == INET_DIAG_BC_S_COND ?
    entry.sport : entry.dport)) {
    yes = 0;
    break;
    }
    if (op.code == INET_DIAG_BC_S_COND)
    addr = entry.saddr;
    else
    addr = entry.daddr;
    if (cond.family != AF_UNSPEC &&
    cond.family != entry.family) {
    if (entry.family == AF_INET6 &&
    cond.family == AF_INET) {
    if (addr[0] == 0 && addr[1] == 0 &&
    addr[2] == htonl(0xffff) &&
    bitstring_match(addr + 3,
    cond.addr,
    cond.prefix_len))
    break;
    }
    yes = 0;
    break;
    }
    if (cond.prefix_len == 0)
    break;
    if (bitstring_match(addr, cond.addr,
    cond.prefix_len))
    break;
    yes = 0;
    break;
    }
    case INET_DIAG_BC_DEV_COND: {
    u32 ifindex;
    ifindex = *((const u32 *)(op + 1));
    if (ifindex != entry.ifindex)
    yes = 0;
    break;
    }
    case INET_DIAG_BC_MARK_COND: {
    struct inet_diag_markcond *cond;
    cond = (struct inet_diag_markcond *)(op + 1);
    if ((entry.mark & cond.mask) != cond.mark)
    yes = 0;
    break;
    }

    case INET_DIAG_BC_CGROUP_COND: {
    u64 cgroup_id;
    cgroup_id = get_unaligned((const u64 *)(op + 1));
    if (cgroup_id != entry.cgroup_id)
    yes = 0;
    break;
    }

    }
    if (yes) {
    len -= op.yes;
    bc += op.yes;
    } else {
    len -= op.no;
    bc += op.no;
    }
    }
    let mut len: return = = 0;
    }
// This helper is available for all sockets (ESTABLISH, TIMEWAIT, SYN_RECV)
//
    static void entry_fill_addrs(struct inet_diag_entry *entry,
    const struct sock *sk)
    {

    if (entry.family == AF_INET6) {
    entry.saddr = sk.sk_v6_rcv_saddr.s6_addr32;
    entry.daddr = sk.sk_v6_daddr.s6_addr32;
    } else

    {
    entry.saddr = &sk.sk_rcv_saddr;
    entry.daddr = &sk.sk_daddr;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn inet_diag_bc_sk(cb_data: *const inet_diag_dump_data, sk: *mut sock) -> c_int {
    int inet_diag_bc_sk(const struct inet_diag_dump_data *cb_data, struct sock *sk)
    {
    const struct nlattr *bc = cb_data.inet_diag_nla_bc;
    const struct inet_sock *inet = inet_sk(sk);
    struct inet_diag_entry entry;
    if (!bc)
    return 1;
    entry.family = READ_ONCE(sk.sk_family);
    entry_fill_addrs(&entry, sk);
    entry.sport = READ_ONCE(inet.inet_num);
    entry.dport = ntohs(READ_ONCE(inet.inet_dport));
    entry.ifindex = READ_ONCE(sk.sk_bound_dev_if);
    if (cb_data.userlocks_needed)
    entry.userlocks = sk_fullsock(sk) ? READ_ONCE(sk.sk_userlocks) : 0;
    if (cb_data.mark_needed) {
    if (sk_fullsock(sk))
    entry.mark = READ_ONCE(sk.sk_mark);
#[no_mangle]
pub unsafe extern "C" fn if(TCP_NEW_SYN_RECV: sk->sk_state ==) -> else {
    else if (sk.sk_state == TCP_NEW_SYN_RECV)
    entry.mark = inet_rsk(inet_reqsk(sk)).ir_mark;
#[no_mangle]
pub unsafe extern "C" fn if(TCP_TIME_WAIT: sk->sk_state ==) -> else {
    else if (sk.sk_state == TCP_TIME_WAIT)
    entry.mark = inet_twsk(sk).tw_mark;
    else
    entry.mark = 0;
    }

    if (cb_data.cgroup_needed)
    entry.cgroup_id = sk_fullsock(sk) ?
    cgroup_id(sock_cgroup_ptr(&sk.sk_cgrp_data)) : 0;

    return inet_diag_bc_run(bc, &entry);
    }
    EXPORT_SYMBOL_GPL(inet_diag_bc_sk);
#[no_mangle]
unsafe extern "C" fn valid_cc(bc: *const c_void, len: c_int, cc: c_int) -> c_int {
    static int valid_cc(const void *bc, int len, int cc)
    {
    while (len >= 0) {
    const struct inet_diag_bc_op *op = bc;
    if (cc > len)
    return 0;
    if (cc == len)
    return 1;
    if (op.yes < 4 || op.yes & 3)
    return 0;
    len -= op.yes;
    bc  += op.yes;
    }
    return 0;
    }
// data is u32 ifindex
    static bool valid_devcond(const struct inet_diag_bc_op *op, int len,
    int *min_len)
    {
// Check ifindex space.
// min_len += sizeof(u32);
    if (len < *min_len)
    return false;
    return true;
    }
// Validate an inet_diag_hostcond.
    static bool valid_hostcond(const struct inet_diag_bc_op *op, int len,
    int *min_len)
    {
    struct inet_diag_hostcond *cond;
    int addr_len;
// Check hostcond space.
// min_len += sizeof(struct inet_diag_hostcond);
    if (len < *min_len)
    return false;
    cond = (struct inet_diag_hostcond *)(op + 1);
// Check address family and address length.
    switch (cond.family) {
    case AF_UNSPEC:
    addr_len = 0;
    break;
    case AF_INET:
    addr_len = sizeof(struct in_addr);
    break;
    case AF_INET6:
    addr_len = sizeof(struct in6_addr);
    break;
    default:
    return false;
    }
// min_len += addr_len;
    if (len < *min_len)
    return false;
// Check prefix length (in bits) vs address length (in bytes).
    if (cond.prefix_len > 8 * addr_len)
    return false;
    return true;
    }
// Validate a port comparison operator.
    static bool valid_port_comparison(const struct inet_diag_bc_op *op,
    int len, int *min_len)
    {
// Port comparisons put the port in a follow-on inet_diag_bc_op.
// min_len += sizeof(struct inet_diag_bc_op);
    if (len < *min_len)
    return false;
    return true;
    }
    static bool valid_markcond(const struct inet_diag_bc_op *op, int len,
    int *min_len)
    {
// min_len += sizeof(struct inet_diag_markcond);
    return len >= *min_len;
    }

    static bool valid_cgroupcond(const struct inet_diag_bc_op *op, int len,
    int *min_len)
    {
// min_len += sizeof(u64);
    return len >= *min_len;
    }

    static int inet_diag_bc_audit(struct inet_diag_dump_data *cb_data,
    const struct sk_buff *skb)
    {
    const struct nlattr *attr = cb_data.inet_diag_nla_bc;
    const void *bytecode, *bc;
    int bytecode_len, len;
    bool net_admin;
    if (!attr)
    return 0;
    if (nla_len(attr) < sizeof(struct inet_diag_bc_op))
    return -EINVAL;
    net_admin = netlink_net_capable(skb, CAP_NET_ADMIN);
    bytecode = bc = nla_data(attr);
    len = bytecode_len = nla_len(attr);
    while (len > 0) {
    let mut min_len: c_int = sizeof(struct inet_diag_bc_op);
    const struct inet_diag_bc_op *op = bc;
    switch (op.code) {
    case INET_DIAG_BC_S_COND:
    case INET_DIAG_BC_D_COND:
    if (!valid_hostcond(bc, len, &min_len))
    return -EINVAL;
    break;
    case INET_DIAG_BC_DEV_COND:
    if (!valid_devcond(bc, len, &min_len))
    return -EINVAL;
    break;
    case INET_DIAG_BC_S_EQ:
    case INET_DIAG_BC_S_GE:
    case INET_DIAG_BC_S_LE:
    case INET_DIAG_BC_D_EQ:
    case INET_DIAG_BC_D_GE:
    case INET_DIAG_BC_D_LE:
    if (!valid_port_comparison(bc, len, &min_len))
    return -EINVAL;
    break;
    case INET_DIAG_BC_MARK_COND:
    if (!net_admin)
    return -EPERM;
    if (!valid_markcond(bc, len, &min_len))
    return -EINVAL;
    cb_data.mark_needed = true;
    break;

    case INET_DIAG_BC_CGROUP_COND:
    if (!valid_cgroupcond(bc, len, &min_len))
    return -EINVAL;
    cb_data.cgroup_needed = true;
    break;

    case INET_DIAG_BC_AUTO:
    cb_data.userlocks_needed = true;
    fallthrough;
    case INET_DIAG_BC_JMP:
    case INET_DIAG_BC_NOP:
    break;
    default:
    return -EINVAL;
    }
    if (op.code != INET_DIAG_BC_NOP) {
    if (op.no < min_len || op.no > len + 4 || op.no & 3)
    return -EINVAL;
    if (op.no < len &&
    !valid_cc(bytecode, bytecode_len, len - op.no))
    return -EINVAL;
    }
    if (op.yes < min_len || op.yes > len + 4 || op.yes & 3)
    return -EINVAL;
    bc  += op.yes;
    len -= op.yes;
    }
    let mut len: return = = 0 ? 0 : -EINVAL;
    }
    static int __inet_diag_dump(struct sk_buff *skb, struct netlink_callback *cb,
    const struct inet_diag_req_v2 *r)
    {
    struct inet_diag_dump_data *cb_data = cb.data;
    const struct inet_diag_handler *handler;
    u32 prev_min_dump_alloc;
    int protocol, err = 0;
    protocol = inet_diag_get_protocol(r, cb_data);
    again:
    prev_min_dump_alloc = cb.min_dump_alloc;
    handler = inet_diag_lock_handler(protocol);
    if (handler) {
    handler.dump(skb, cb, r);
    inet_diag_unlock_handler(handler);
    } else {
    err = -ENOENT;
    }
// The skb is not large enough to fit one sk info and
// inet_sk_diag_fill() has requested for a larger skb.
//
    if (!skb.len && cb.min_dump_alloc > prev_min_dump_alloc) {
    err = pskb_expand_head(skb, 0, cb.min_dump_alloc, GFP_KERNEL);
    if (!err)
    goto again;
    }
    return err ? : skb.len;
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_dump(skb: *mut sk_buff, cb: *mut netlink_callback) -> c_int {
    static int inet_diag_dump(struct sk_buff *skb, struct netlink_callback *cb)
    {
    return __inet_diag_dump(skb, cb, nlmsg_data(cb.nlh));
    }
#[no_mangle]
unsafe extern "C" fn __inet_diag_dump_start(cb: *mut netlink_callback, hdrlen: c_int) -> c_int {
    static int __inet_diag_dump_start(struct netlink_callback *cb, int hdrlen)
    {
    const struct nlmsghdr *nlh = cb.nlh;
    struct inet_diag_dump_data *cb_data;
    struct sk_buff *skb = cb.skb;
    struct nlattr *nla;
    int err;
    cb_data = kzalloc_obj(*cb_data);
    if (!cb_data)
    return -ENOMEM;
    err = inet_diag_parse_attrs(nlh, hdrlen, cb_data.req_nlas);
    if (err) {
    kfree(cb_data);
    return err;
    }
    err = inet_diag_bc_audit(cb_data, skb);
    if (err) {
    kfree(cb_data);
    return err;
    }
    nla = cb_data.inet_diag_nla_bpf_stgs;
    if (nla) {
    struct bpf_sk_storage_diag *bpf_stg_diag;
    bpf_stg_diag = bpf_sk_storage_diag_alloc(nla);
    if (IS_ERR(bpf_stg_diag)) {
    kfree(cb_data);
    return PTR_ERR(bpf_stg_diag);
    }
    cb_data.bpf_stg_diag = bpf_stg_diag;
    }
    cb.data = cb_data;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_dump_start(cb: *mut netlink_callback) -> c_int {
    static int inet_diag_dump_start(struct netlink_callback *cb)
    {
    return __inet_diag_dump_start(cb, sizeof(struct inet_diag_req_v2));
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_dump_start_compat(cb: *mut netlink_callback) -> c_int {
    static int inet_diag_dump_start_compat(struct netlink_callback *cb)
    {
    return __inet_diag_dump_start(cb, sizeof(struct inet_diag_req));
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_dump_done(cb: *mut netlink_callback) -> c_int {
    static int inet_diag_dump_done(struct netlink_callback *cb)
    {
    struct inet_diag_dump_data *cb_data = cb.data;
    bpf_sk_storage_diag_free(cb_data.bpf_stg_diag);
    kfree(cb.data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_type2proto(type: c_int) -> c_int {
    static int inet_diag_type2proto(int type)
    {
    switch (type) {
    case TCPDIAG_GETSOCK:
    return IPPROTO_TCP;
    default:
    return 0;
    }
    }
    static int inet_diag_dump_compat(struct sk_buff *skb,
    struct netlink_callback *cb)
    {
    struct inet_diag_req *rc = nlmsg_data(cb.nlh);
    struct inet_diag_req_v2 req;
    req.sdiag_family = AF_UNSPEC; /* compatibility */
    req.sdiag_protocol = inet_diag_type2proto(cb.nlh.nlmsg_type);
    req.idiag_ext = rc.idiag_ext;
    req.pad = 0;
    req.idiag_states = rc.idiag_states;
    req.id = rc.id;
    return __inet_diag_dump(skb, cb, &req);
    }
    static int inet_diag_get_exact_compat(struct sk_buff *in_skb,
    const struct nlmsghdr *nlh)
    {
    struct inet_diag_req *rc = nlmsg_data(nlh);
    struct inet_diag_req_v2 req;
    req.sdiag_family = rc.idiag_family;
    req.sdiag_protocol = inet_diag_type2proto(nlh.nlmsg_type);
    req.idiag_ext = rc.idiag_ext;
    req.pad = 0;
    req.idiag_states = rc.idiag_states;
    req.id = rc.id;
    return inet_diag_cmd_exact(SOCK_DIAG_BY_FAMILY, in_skb, nlh,
    sizeof(struct inet_diag_req), &req);
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_rcv_msg_compat(skb: *mut sk_buff, nlh: *mut nlmsghdr) -> c_int {
    static int inet_diag_rcv_msg_compat(struct sk_buff *skb, struct nlmsghdr *nlh)
    {
    let mut hdrlen: c_int = sizeof(struct inet_diag_req);
    struct net *net = sock_net(skb.sk);
    if (nlh.nlmsg_type >= INET_DIAG_GETSOCK_MAX ||
    nlmsg_len(nlh) < hdrlen)
    return -EINVAL;
    if (nlh.nlmsg_flags & NLM_F_DUMP) {
    struct netlink_dump_control c = {
    .start = inet_diag_dump_start_compat,
    .done = inet_diag_dump_done,
    .dump = inet_diag_dump_compat,
    };
    return netlink_dump_start(net.diag_nlsk, skb, nlh, &c);
    }
    return inet_diag_get_exact_compat(skb, nlh);
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_handler_cmd(skb: *mut sk_buff, h: *mut nlmsghdr) -> c_int {
    static int inet_diag_handler_cmd(struct sk_buff *skb, struct nlmsghdr *h)
    {
    let mut hdrlen: c_int = sizeof(struct inet_diag_req_v2);
    struct net *net = sock_net(skb.sk);
    if (nlmsg_len(h) < hdrlen)
    return -EINVAL;
    if (h.nlmsg_type == SOCK_DIAG_BY_FAMILY &&
    h.nlmsg_flags & NLM_F_DUMP) {
    struct netlink_dump_control c = {
    .start = inet_diag_dump_start,
    .done = inet_diag_dump_done,
    .dump = inet_diag_dump,
    };
    return netlink_dump_start(net.diag_nlsk, skb, h, &c);
    }
    return inet_diag_cmd_exact(h.nlmsg_type, skb, h, hdrlen,
    nlmsg_data(h));
    }
    static
#[no_mangle]
pub unsafe extern "C" fn inet_diag_handler_get_info(skb: *mut sk_buff, sk: *mut sock) -> c_int {
    int inet_diag_handler_get_info(struct sk_buff *skb, struct sock *sk)
    {
    const struct inet_diag_handler *handler;
    struct nlmsghdr *nlh;
    struct nlattr *attr;
    struct inet_diag_msg *r;
    void *info = core::ptr::null_mut();
    let mut err: c_int = 0;
    nlh = nlmsg_put(skb, 0, 0, SOCK_DIAG_BY_FAMILY, sizeof(*r), 0);
    if (!nlh)
    return -ENOMEM;
    r = nlmsg_data(nlh);
    memset(r, 0, sizeof(*r));
    inet_diag_msg_common_fill(r, sk);
    if (sk.sk_type == SOCK_DGRAM || sk.sk_type == SOCK_STREAM)
    r.id.idiag_sport = inet_sk(sk).inet_sport;
    r.idiag_state = sk.sk_state;
    if ((err = nla_put_u8(skb, INET_DIAG_PROTOCOL, sk.sk_protocol))) {
    nlmsg_cancel(skb, nlh);
    return err;
    }
    handler = inet_diag_lock_handler(sk.sk_protocol);
    if (!handler) {
    nlmsg_cancel(skb, nlh);
    return -ENOENT;
    }
    attr = handler.idiag_info_size
    ? nla_reserve_64bit(skb, INET_DIAG_INFO,
    handler.idiag_info_size,
    INET_DIAG_PAD)
    : core::ptr::null_mut();
    if (attr)
    info = nla_data(attr);
    handler.idiag_get_info(sk, r, info);
    inet_diag_unlock_handler(handler);
    nlmsg_end(skb, nlh);
    return 0;
    }
    static const struct sock_diag_handler inet_diag_handler = {
    .owner = THIS_MODULE,
    .family = AF_INET,
    .dump = inet_diag_handler_cmd,
    .get_info = inet_diag_handler_get_info,
    .destroy = inet_diag_handler_cmd,
    };
    static const struct sock_diag_handler inet6_diag_handler = {
    .owner = THIS_MODULE,
    .family = AF_INET6,
    .dump = inet_diag_handler_cmd,
    .get_info = inet_diag_handler_get_info,
    .destroy = inet_diag_handler_cmd,
    };
#[no_mangle]
pub unsafe extern "C" fn inet_diag_register(h: *const inet_diag_handler) -> c_int {
    int inet_diag_register(const struct inet_diag_handler *h)
    {
    let mut type: __u16 = h.idiag_type;
    if (type >= IPPROTO_MAX)
    return -EINVAL;
    return !cmpxchg((const struct inet_diag_handler **)&inet_diag_table[type],
    core::ptr::null_mut(), h) ? 0 : -EEXIST;
    }
    EXPORT_SYMBOL_GPL(inet_diag_register);
#[no_mangle]
pub unsafe extern "C" fn inet_diag_unregister(h: *const inet_diag_handler) {
    void inet_diag_unregister(const struct inet_diag_handler *h)
    {
    let mut type: __u16 = h.idiag_type;
    if (type >= IPPROTO_MAX)
    return;
    xchg((const struct inet_diag_handler **)&inet_diag_table[type],
    core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(inet_diag_unregister);
    static const struct sock_diag_inet_compat inet_diag_compat = {
    .owner	= THIS_MODULE,
    .fn	= inet_diag_rcv_msg_compat,
    };
#[no_mangle]
unsafe extern "C" fn inet_diag_init() -> int __init {
    static int __init inet_diag_init(void)
    {
    const int inet_diag_table_size = (IPPROTO_MAX *
    sizeof(struct inet_diag_handler *));
    let mut err: c_int = -ENOMEM;
    inet_diag_table = kzalloc(inet_diag_table_size, GFP_KERNEL);
    if (!inet_diag_table)
    goto out;
    err = sock_diag_register(&inet_diag_handler);
    if (err)
    goto out_free_nl;
    err = sock_diag_register(&inet6_diag_handler);
    if (err)
    goto out_free_inet;
    sock_diag_register_inet_compat(&inet_diag_compat);
    out:
    return err;
    out_free_inet:
    sock_diag_unregister(&inet_diag_handler);
    out_free_nl:
    kfree(inet_diag_table);
    goto out;
    }
#[no_mangle]
unsafe extern "C" fn inet_diag_exit() -> void __exit {
    static void __exit inet_diag_exit(void)
    {
    sock_diag_unregister(&inet6_diag_handler);
    sock_diag_unregister(&inet_diag_handler);
    sock_diag_unregister_inet_compat(&inet_diag_compat);
    kfree(inet_diag_table);
    }
    module_init(inet_diag_init);
    module_exit(inet_diag_exit);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("INET/INET6: socket monitoring via SOCK_DIAG");
    MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 2 /* AF_INET */);
    MODULE_ALIAS_NET_PF_PROTO_TYPE(PF_NETLINK, NETLINK_SOCK_DIAG, 10 /* AF_INET6 */);
