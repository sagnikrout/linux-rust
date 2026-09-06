//! Automatically rewritten from C to Rust
//! Source: net/xfrm/xfrm_ipcomp.c
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
// IP Payload Compression Protocol (IPComp) - RFC3173.
//
// Copyright (c) 2003 James Morris <jmorris@intercode.com.au>
// Copyright (c) 2003-2025 Herbert Xu <herbert@gondor.apana.org.au>
//
// Todo:
// - Tunable compression parameters.
// - Compression stats.
// - Adaptive compression.
//

pub const IPCOMP_SCRATCH_SIZE: c_int = 65400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipcomp_skb_cb {
    pub xfrm: xfrm_skb_cb,
    pub req: *mut acomp_req,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipcomp_data {
    pub threshold: u16,
    pub tfm: *mut crypto_acomp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipcomp_req_extra {
    pub x: *mut xfrm_state,
    pub sg: [scatterlist; ],
}

    static inline struct ipcomp_skb_cb *ipcomp_cb(struct sk_buff *skb)
    {
    struct ipcomp_skb_cb *cb = (void *)skb.cb;
    BUILD_BUG_ON(sizeof(*cb) > sizeof(skb.cb));
    return cb;
    }
#[no_mangle]
unsafe extern "C" fn ipcomp_post_acomp(skb: *mut sk_buff, err: c_int, hlen: c_int) -> c_int {
    static int ipcomp_post_acomp(struct sk_buff *skb, int err, int hlen)
    {
    struct acomp_req *req = ipcomp_cb(skb).req;
    struct ipcomp_req_extra *extra;
    struct scatterlist *dsg;
    int len, dlen;
    if (unlikely(!req))
    return err;
    extra = acomp_request_extra(req);
    dsg = extra.sg;
    if (unlikely(err))
    goto out_free_req;
    dlen = req.dlen;
    pskb_trim_unique(skb, 0);
    __skb_put(skb, hlen);
// Only update truesize on input.
    if (!hlen)
    skb.truesize += dlen;
    skb.data_len = dlen;
    skb.len += dlen;
    do {
    skb_frag_t *frag;
    struct page *page;
    frag = skb_shinfo(skb).frags + skb_shinfo(skb).nr_frags;
    page = sg_page(dsg);
    dsg = sg_next(dsg);
    len = PAGE_SIZE;
    if (dlen < len)
    len = dlen;
    skb_frag_fill_page_desc(frag, page, 0, len);
    skb_shinfo(skb).nr_frags++;
    } while ((dlen -= len));
    out_free_req:
    for (; dsg && sg_page(dsg); dsg = sg_next(dsg))
    __free_page(sg_page(dsg));
    acomp_request_free(req);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn ipcomp_input_done2(skb: *mut sk_buff, err: c_int) -> c_int {
    static int ipcomp_input_done2(struct sk_buff *skb, int err)
    {
    struct ip_comp_hdr *ipch = ip_comp_hdr(skb);
    let mut plen: c_int = skb.len;
    skb.transport_header = skb.network_header + sizeof(*ipch);
    return ipcomp_post_acomp(skb, err, 0) ?:
    skb.len < (plen + sizeof(ip_comp_hdr)) ? -EINVAL :
    ipch.nexthdr;
    }
#[no_mangle]
unsafe extern "C" fn ipcomp_input_done(data: *mut c_void, err: c_int) {
    static void ipcomp_input_done(void *data, int err)
    {
    struct sk_buff *skb = data;
    xfrm_input_resume(skb, ipcomp_input_done2(skb, err));
    }
    static struct acomp_req *ipcomp_setup_req(struct xfrm_state *x,
    struct sk_buff *skb, int minhead,
    int dlen)
    {
    let mut dnfrags: c_int = min(MAX_SKB_FRAGS, 16);
    struct ipcomp_data *ipcd = x.data;
    struct ipcomp_req_extra *extra;
    struct scatterlist *sg, *dsg;
    let mut plen: c_int = skb.len;
    struct crypto_acomp *tfm;
    struct acomp_req *req;
    int nfrags;
    int total;
    int err;
    int i;
    ipcomp_cb(skb).req = core::ptr::null_mut();
    do {
    struct sk_buff *trailer;
    if (skb.len > PAGE_SIZE) {
    if (skb_linearize_cow(skb))
    return ERR_PTR(-ENOMEM);
    nfrags = 1;
    break;
    }
    if (!skb_cloned(skb) && skb_headlen(skb) >= minhead) {
    if (!skb_is_nonlinear(skb)) {
    nfrags = 1;
    break;
    } else if (!skb_has_frag_list(skb)) {
    nfrags = skb_shinfo(skb).nr_frags;
    nfrags++;
    break;
    }
    }
    nfrags = skb_cow_data(skb, skb_headlen(skb) < minhead ?
    minhead - skb_headlen(skb) : 0,
    &trailer);
    if (nfrags < 0)
    return ERR_PTR(nfrags);
    } while (0);
    tfm = ipcd.tfm;
    req = acomp_request_alloc_extra(
    tfm, sizeof(*extra) + sizeof(*sg) * (nfrags + dnfrags),
    GFP_ATOMIC);
    ipcomp_cb(skb).req = req;
    if (!req)
    return ERR_PTR(-ENOMEM);
    extra = acomp_request_extra(req);
    extra.x = x;
    dsg = extra.sg;
    sg = dsg + dnfrags;
    sg_init_table(sg, nfrags);
    err = skb_to_sgvec(skb, sg, 0, plen);
    if (unlikely(err < 0))
    return ERR_PTR(err);
    sg_init_table(dsg, dnfrags);
    total = 0;
    for (i = 0; i < dnfrags && total < dlen; i++) {
    struct page *page;
    page = alloc_page(GFP_ATOMIC);
    if (!page)
    break;
    sg_set_page(dsg + i, page, PAGE_SIZE, 0);
    total += PAGE_SIZE;
    }
    if (!i)
    return ERR_PTR(-ENOMEM);
    sg_mark_end(dsg + i - 1);
    dlen = min(dlen, total);
    acomp_request_set_params(req, sg, dsg, plen, dlen);
    return req;
    }
#[no_mangle]
unsafe extern "C" fn ipcomp_decompress(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int ipcomp_decompress(struct xfrm_state *x, struct sk_buff *skb)
    {
    struct acomp_req *req;
    int err;
    req = ipcomp_setup_req(x, skb, 0, IPCOMP_SCRATCH_SIZE);
    err = PTR_ERR(req);
    if (IS_ERR(req))
    goto out;
    acomp_request_set_callback(req, 0, ipcomp_input_done, skb);
    err = crypto_acomp_decompress(req);
    if (err == -EINPROGRESS)
    return err;
    out:
    return ipcomp_input_done2(skb, err);
    }
#[no_mangle]
pub unsafe extern "C" fn ipcomp_input(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    int ipcomp_input(struct xfrm_state *x, struct sk_buff *skb)
    {
    struct ip_comp_hdr *ipch __maybe_unused;
    if (!pskb_may_pull(skb, sizeof(*ipch)))
    return -EINVAL;
    skb.ip_summed = CHECKSUM_NONE;
// Remove ipcomp header and decompress original payload
    __skb_pull(skb, sizeof(*ipch));
    return ipcomp_decompress(x, skb);
    }
    EXPORT_SYMBOL_GPL(ipcomp_input);
#[no_mangle]
unsafe extern "C" fn ipcomp_output_push(skb: *mut sk_buff) -> c_int {
    static int ipcomp_output_push(struct sk_buff *skb)
    {
    skb_push(skb, -skb_network_offset(skb));
    return 0;
    }
    static int ipcomp_output_done2(struct xfrm_state *x, struct sk_buff *skb,
    int err)
    {
    struct ip_comp_hdr *ipch;
    err = ipcomp_post_acomp(skb, err, sizeof(*ipch));
    if (err)
    goto out_ok;
// Install ipcomp header, convert into ipcomp datagram.
    ipch = ip_comp_hdr(skb);
    ipch.nexthdr = *skb_mac_header(skb);
    ipch.flags = 0;
    ipch.cpi = htons((u16 )ntohl(x.id.spi));
// skb_mac_header(skb) = IPPROTO_COMP;
    out_ok:
    return ipcomp_output_push(skb);
    }
#[no_mangle]
unsafe extern "C" fn ipcomp_output_done(data: *mut c_void, err: c_int) {
    static void ipcomp_output_done(void *data, int err)
    {
    struct ipcomp_req_extra *extra;
    struct sk_buff *skb = data;
    struct acomp_req *req;
    req = ipcomp_cb(skb).req;
    extra = acomp_request_extra(req);
    xfrm_output_resume(skb_to_full_sk(skb), skb,
    ipcomp_output_done2(extra.x, skb, err));
    }
#[no_mangle]
unsafe extern "C" fn ipcomp_compress(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    static int ipcomp_compress(struct xfrm_state *x, struct sk_buff *skb)
    {
    struct ip_comp_hdr *ipch __maybe_unused;
    struct acomp_req *req;
    int err;
    req = ipcomp_setup_req(x, skb, sizeof(*ipch),
    skb.len - sizeof(*ipch));
    err = PTR_ERR(req);
    if (IS_ERR(req))
    goto out;
    acomp_request_set_callback(req, 0, ipcomp_output_done, skb);
    err = crypto_acomp_compress(req);
    if (err == -EINPROGRESS)
    return err;
    out:
    return ipcomp_output_done2(x, skb, err);
    }
#[no_mangle]
pub unsafe extern "C" fn ipcomp_output(x: *mut xfrm_state, skb: *mut sk_buff) -> c_int {
    int ipcomp_output(struct xfrm_state *x, struct sk_buff *skb)
    {
    struct ipcomp_data *ipcd = x.data;
    if (skb.len < ipcd.threshold) {
// Don't bother compressing
    return ipcomp_output_push(skb);
    }
    return ipcomp_compress(x, skb);
    }
    EXPORT_SYMBOL_GPL(ipcomp_output);
#[no_mangle]
unsafe extern "C" fn ipcomp_free_data(ipcd: *mut ipcomp_data) {
    static void ipcomp_free_data(struct ipcomp_data *ipcd)
    {
    crypto_free_acomp(ipcd.tfm);
    }
#[no_mangle]
pub unsafe extern "C" fn ipcomp_destroy(x: *mut xfrm_state) {
    void ipcomp_destroy(struct xfrm_state *x)
    {
    struct ipcomp_data *ipcd = x.data;
    if (!ipcd)
    return;
    ipcomp_free_data(ipcd);
    kfree(ipcd);
    }
    EXPORT_SYMBOL_GPL(ipcomp_destroy);
#[no_mangle]
pub unsafe extern "C" fn ipcomp_init_state(x: *mut xfrm_state, extack: *mut netlink_ext_ack) -> c_int {
    int ipcomp_init_state(struct xfrm_state *x, struct netlink_ext_ack *extack)
    {
    int err;
    struct ipcomp_data *ipcd;
    struct xfrm_algo_desc *calg_desc;
    err = -EINVAL;
    if (!x.calg) {
    NL_SET_ERR_MSG(extack, "Missing required compression algorithm");
    goto out;
    }
    if (x.encap) {
    NL_SET_ERR_MSG(extack, "IPComp is not compatible with encapsulation");
    goto out;
    }
    err = -ENOMEM;
    ipcd = kzalloc_obj(*ipcd);
    if (!ipcd)
    goto out;
    ipcd.tfm = crypto_alloc_acomp(x.calg.alg_name, 0, 0);
    if (IS_ERR(ipcd.tfm))
    goto error;
    calg_desc = xfrm_calg_get_byname(x.calg.alg_name, 0);
    BUG_ON(!calg_desc);
    ipcd.threshold = calg_desc.uinfo.comp.threshold;
    x.data = ipcd;
    err = 0;
    out:
    return err;
    error:
    ipcomp_free_data(ipcd);
    kfree(ipcd);
    goto out;
    }
    EXPORT_SYMBOL_GPL(ipcomp_init_state);
    MODULE_LICENSE("GPL");
    MODULE_DESCRIPTION("IP Payload Compression Protocol (IPComp) - RFC3173");
    MODULE_AUTHOR("James Morris <jmorris@intercode.com.au>");
