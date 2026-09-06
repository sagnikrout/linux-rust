//! Automatically rewritten from C to Rust
//! Source: net/netfilter/ipvs/ip_vs_pe_sip.c
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


// SPDX-License-Identifier: GPL-2.0-only

    static const char *ip_vs_dbg_callid(char *buf, size_t buf_len,
    const char *callid, size_t callid_len,
    int *idx)
    {
    let mut max_len: usize = 64;
    let mut len: usize = min3(max_len, callid_len, buf_len - *idx - 1);
    memcpy(buf + *idx, callid, len);
    buf[*idx+len] = '\0';
// idx += len + 1;
    return buf + *idx - len;
    }

    ip_vs_dbg_callid(ip_vs_dbg_buf, sizeof(ip_vs_dbg_buf),		\
    callid, len, &ip_vs_dbg_idx)

    static int get_callid(const char *dptr, unsigned int dataoff,
    unsigned int datalen,
    unsigned int *matchoff, unsigned int *matchlen)
    {
// Find callid
    while (1) {
    int ret = ct_sip_get_header(core::ptr::null_mut(), dptr, dataoff, datalen,
    SIP_HDR_CALL_ID, matchoff,
    matchlen);
    if (ret > 0)
    break;
    if (!ret)
    return -EINVAL;
    dataoff += *matchoff;
    }
// Too large is useless
    if (*matchlen > IP_VS_PEDATA_MAXLEN)
    return -EINVAL;
// SIP headers are always followed by a line terminator
    if (*matchoff + *matchlen == datalen)
    return -EINVAL;
// RFC 2543 allows lines to be terminated with CR, LF or CRLF,
// RFC 3261 allows only CRLF, we support both.
    if (*(dptr + *matchoff + *matchlen) != '\r' &&
// (dptr + *matchoff + *matchlen) != '\n')
    return -EINVAL;
    IP_VS_DBG_BUF(9, "SIP callid %s (%d bytes)\n",
    IP_VS_DEBUG_CALLID(dptr + *matchoff, *matchlen),
// matchlen);
    return 0;
    }
    static int
    ip_vs_sip_fill_param(struct ip_vs_conn_param *p, struct sk_buff *skb)
    {
    struct ip_vs_iphdr iph;
    unsigned int dataoff, datalen, matchoff, matchlen;
    const char *dptr;
    int retc;
    retc = ip_vs_fill_iph_skb(p.af, skb, false, &iph);
// Only useful with UDP
    if (!retc || iph.protocol != IPPROTO_UDP)
    return -EINVAL;
// todo: IPv6 fragments:
// I think this only should be done for the first fragment. /HS
//
    dataoff = iph.len + sizeof(struct udphdr);
    if (dataoff >= skb.len)
    return -EINVAL;
    retc = skb_linearize(skb);
    if (retc < 0)
    return retc;
    dptr = skb.data + dataoff;
    datalen = skb.len - dataoff;
    if (get_callid(dptr, 0, datalen, &matchoff, &matchlen))
    return -EINVAL;
// N.B: pe_data is only set on success,
// this allows fallback to the default persistence logic on failure
//
    p.pe_data = kmemdup(dptr + matchoff, matchlen, GFP_ATOMIC);
    if (!p.pe_data)
    return -ENOMEM;
    p.pe_data_len = matchlen;
    return 0;
    }
    static bool ip_vs_sip_ct_match(const struct ip_vs_conn_param *p,
    struct ip_vs_conn *ct)
    {
    let mut ret: bool = false;
    if (ct.af == p.af &&
    ip_vs_addr_equal(p.af, p.caddr, &ct.caddr) &&
// protocol should only be IPPROTO_IP if
// d_addr is a fwmark
    ip_vs_addr_equal(p.protocol == IPPROTO_IP ? AF_UNSPEC : p.af,
    p.vaddr, &ct.vaddr) &&
    ct.vport == p.vport &&
    ct.flags & IP_VS_CONN_F_TEMPLATE &&
    ct.protocol == p.protocol &&
    ct.pe_data && ct.pe_data_len == p.pe_data_len &&
    !memcmp(ct.pe_data, p.pe_data, p.pe_data_len))
    ret = true;
    IP_VS_DBG_BUF(9, "SIP template match %s %s.%s:%d %s\n",
    ip_vs_proto_name(p.protocol),
    IP_VS_DEBUG_CALLID(p.pe_data, p.pe_data_len),
    IP_VS_DBG_ADDR(p.af, p.vaddr), ntohs(p.vport),
    ret ? "hit" : "not hit");
    return ret;
    }
    static u32 ip_vs_sip_hashkey_raw(const struct ip_vs_conn_param *p,
    struct ip_vs_rht *t, bool inverse)
    {
    return jhash(p.pe_data, p.pe_data_len, (u32)t.hash_key.key[0]);
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_sip_show_pe_data(cp: *const ip_vs_conn, buf: *mut c_char) -> c_int {
    static int ip_vs_sip_show_pe_data(const struct ip_vs_conn *cp, char *buf)
    {
    memcpy(buf, cp.pe_data, cp.pe_data_len);
    return cp.pe_data_len;
    }
    static struct ip_vs_conn *
    ip_vs_sip_conn_out(struct ip_vs_service *svc,
    struct ip_vs_dest *dest,
    struct sk_buff *skb,
    const struct ip_vs_iphdr *iph,
    __be16 dport,
    __be16 cport)
    {
    if (likely(iph.protocol == IPPROTO_UDP))
    return ip_vs_new_conn_out(svc, dest, skb, iph, dport, cport);
// currently no need to handle other than UDP
    return core::ptr::null_mut();
    }
    static struct ip_vs_pe ip_vs_sip_pe =
    {
    .name =			"sip",
    .refcnt =		ATOMIC_INIT(0),
    .module =		THIS_MODULE,
    .n_list =		LIST_HEAD_INIT(ip_vs_sip_pe.n_list),
    .fill_param =		ip_vs_sip_fill_param,
    .ct_match =		ip_vs_sip_ct_match,
    .hashkey_raw =		ip_vs_sip_hashkey_raw,
    .show_pe_data =		ip_vs_sip_show_pe_data,
    .conn_out =		ip_vs_sip_conn_out,
    };
#[no_mangle]
unsafe extern "C" fn ip_vs_sip_init() -> int __init {
    static int __init ip_vs_sip_init(void)
    {
    return register_ip_vs_pe(&ip_vs_sip_pe);
    }
#[no_mangle]
unsafe extern "C" fn ip_vs_sip_cleanup() -> void __exit {
    static void __exit ip_vs_sip_cleanup(void)
    {
    unregister_ip_vs_pe(&ip_vs_sip_pe);
    synchronize_rcu();
    }
    module_init(ip_vs_sip_init);
    module_exit(ip_vs_sip_cleanup);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("ipvs sip helper");
