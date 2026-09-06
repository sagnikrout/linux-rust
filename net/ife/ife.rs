//! Automatically rewritten from C to Rust
//! Source: net/ife/ife.c
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


//
// net/ife/ife.c - Inter-FE protocol based on ForCES WG InterFE LFB
// Copyright (c) 2015 Jamal Hadi Salim <jhs@mojatatu.com>
// Copyright (c) 2017 Yotam Gigi <yotamg@mellanox.com>
//
// Refer to: draft-ietf-forces-interfelfb-03 and netdev01 paper:
// "Distributing Linux Traffic Control Classifier-Action Subsystem"
// Authors: Jamal Hadi Salim and Damascene M. Joachimpillai
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifeheadr {
    pub metalen: __be16,
    pub tlv_data: [u8; ],
}

    void *ife_encode(struct sk_buff *skb, u16 metalen)
    {
// OUTERHDR:TOTMETALEN:{TLVHDR:Metadatum:TLVHDR..}:ORIGDATA
// where ORIGDATA = original ethernet header ...
//
    let mut hdrm: c_int = metalen + IFE_METAHDRLEN;
    let mut total_push: c_int = hdrm + ETH_HLEN;
    struct ifeheadr *ifehdr;
    struct ethhdr *iethh;	/* inner ether header */
    let mut skboff: c_int = 0;
    int err;
    err = skb_cow_head(skb, total_push);
    if (unlikely(err))
    return core::ptr::null_mut();
    iethh = (struct ethhdr *) skb.data;
    __skb_push(skb, total_push);
    memcpy(skb.data, iethh, ETH_HLEN);
    skb_reset_mac_header(skb);
    skboff += ETH_HLEN;
// total metadata length
    ifehdr = (struct ifeheadr *) (skb.data + skboff);
    metalen += IFE_METAHDRLEN;
    ifehdr.metalen = htons(metalen);
    return ifehdr.tlv_data;
    }
    EXPORT_SYMBOL_GPL(ife_encode);
    void *ife_decode(struct sk_buff *skb, u16 *metalen)
    {
    struct ifeheadr *ifehdr;
    int total_pull;
    u16 ifehdrln;
    if (!pskb_may_pull(skb, ETH_HLEN + IFE_METAHDRLEN))
    return core::ptr::null_mut();
    ifehdr = (struct ifeheadr *)(skb.data + ETH_HLEN);
    ifehdrln = ntohs(ifehdr.metalen);
    total_pull = ETH_HLEN + ifehdrln;
    if (unlikely(ifehdrln < 2))
    return core::ptr::null_mut();
    if (unlikely(!pskb_may_pull(skb, total_pull + ETH_HLEN)))
    return core::ptr::null_mut();
    ifehdr = (struct ifeheadr *)(skb.data + ETH_HLEN);
    skb_set_mac_header(skb, total_pull);
    __skb_pull(skb, total_pull);
// metalen = ifehdrln - IFE_METAHDRLEN;
    return &ifehdr.tlv_data;
    }
    EXPORT_SYMBOL_GPL(ife_decode);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct meta_tlvhdr {
    pub type: __be16,
    pub len: __be16,
}

    static bool __ife_tlv_meta_valid(const unsigned char *skbdata,
    const unsigned char *ifehdr_end)
    {
    const struct meta_tlvhdr *tlv;
    u16 tlvlen;
    if (unlikely(skbdata + sizeof(*tlv) > ifehdr_end))
    return false;
    tlv = (const struct meta_tlvhdr *)skbdata;
    tlvlen = ntohs(tlv.len);
// tlv length field is inc header, check on minimum
    if (tlvlen < NLA_HDRLEN)
    return false;
// overflow by NLA_ALIGN check
    if (NLA_ALIGN(tlvlen) < tlvlen)
    return false;
    if (unlikely(skbdata + NLA_ALIGN(tlvlen) > ifehdr_end))
    return false;
    return true;
    }
// Caller takes care of presenting data in network order
//
    void *ife_tlv_meta_decode(void *skbdata, const void *ifehdr_end, u16 *attrtype,
    u16 *dlen, u16 *totlen)
    {
    struct meta_tlvhdr *tlv;
    if (!__ife_tlv_meta_valid(skbdata, ifehdr_end))
    return core::ptr::null_mut();
    tlv = (struct meta_tlvhdr *)skbdata;
// dlen = ntohs(tlv->len) - NLA_HDRLEN;
// attrtype = ntohs(tlv->type);
    if (totlen)
// totlen = nla_total_size(*dlen);
    return skbdata + sizeof(struct meta_tlvhdr);
    }
    EXPORT_SYMBOL_GPL(ife_tlv_meta_decode);
    void *ife_tlv_meta_next(void *skbdata)
    {
    struct meta_tlvhdr *tlv = (struct meta_tlvhdr *) skbdata;
    let mut tlvlen: u16 = ntohs(tlv.len);
    tlvlen = NLA_ALIGN(tlvlen);
    return skbdata + tlvlen;
    }
    EXPORT_SYMBOL_GPL(ife_tlv_meta_next);
// Caller takes care of presenting data in network order
//
#[no_mangle]
pub unsafe extern "C" fn ife_tlv_meta_encode(skbdata: *mut c_void, attrtype: u16, dlen: u16, dval: *const c_void) -> c_int {
    int ife_tlv_meta_encode(void *skbdata, u16 attrtype, u16 dlen, const void *dval)
    {
    __be32 *tlv = (__be32 *) (skbdata);
    u16 totlen = nla_total_size(dlen);	/*alignment + hdr */
    char *dptr = (char *) tlv + NLA_HDRLEN;
    let mut htlv: u32 = attrtype << 16 | (dlen + NLA_HDRLEN);
// tlv = htonl(htlv);
    memset(dptr, 0, totlen - NLA_HDRLEN);
    memcpy(dptr, dval, dlen);
    return totlen;
    }
    EXPORT_SYMBOL_GPL(ife_tlv_meta_encode);
    MODULE_AUTHOR("Jamal Hadi Salim <jhs@mojatatu.com>");
    MODULE_AUTHOR("Yotam Gigi <yotam.gi@gmail.com>");
    MODULE_DESCRIPTION("Inter-FE LFB action");
    MODULE_LICENSE("GPL");
