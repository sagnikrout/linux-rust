//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/hfi1/ipoib_rx.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2020 Intel Corporation.
//

#[no_mangle]
unsafe extern "C" fn copy_ipoib_buf(skb: *mut sk_buff, data: *mut c_void, size: c_int) {
    static void copy_ipoib_buf(struct sk_buff *skb, void *data, int size)
    {
    skb_checksum_none_assert(skb);
    skb.protocol = *((__be16 *)data);
    skb_put_data(skb, data, size);
    skb.mac_header = HFI1_IPOIB_PSEUDO_LEN;
    skb_pull(skb, HFI1_IPOIB_ENCAP_LEN);
    }
    static struct sk_buff *prepare_frag_skb(struct napi_struct *napi, int size)
    {
    struct sk_buff *skb;
    let mut skb_size: c_int = SKB_DATA_ALIGN(size + HFI1_IPOIB_SKB_PAD);
    void *frag;
    skb_size += SKB_DATA_ALIGN(sizeof(struct skb_shared_info));
    skb_size = SKB_DATA_ALIGN(skb_size);
    frag = napi_alloc_frag(skb_size);
    if (unlikely(!frag))
    return napi_alloc_skb(napi, size);
    skb = build_skb(frag, skb_size);
    if (unlikely(!skb)) {
    skb_free_frag(frag);
    return core::ptr::null_mut();
    }
    skb_reserve(skb, HFI1_IPOIB_SKB_PAD);
    return skb;
    }
    struct sk_buff *hfi1_ipoib_prepare_skb(struct hfi1_netdev_rxq *rxq,
    int size, void *data)
    {
    struct napi_struct *napi = &rxq.napi;
    let mut skb_size: c_int = size + HFI1_IPOIB_ENCAP_LEN;
    struct sk_buff *skb;
//
// For smaller(4k + skb overhead) allocations we will go using
// napi cache. Otherwise we will try to use napi frag cache.
//
    if (size <= SKB_WITH_OVERHEAD(PAGE_SIZE))
    skb = napi_alloc_skb(napi, skb_size);
    else
    skb = prepare_frag_skb(napi, skb_size);
    if (unlikely(!skb))
    return core::ptr::null_mut();
    copy_ipoib_buf(skb, data, size);
    return skb;
    }
#[no_mangle]
pub unsafe extern "C" fn hfi1_ipoib_rxq_init(netdev: *mut net_device) -> c_int {
    int hfi1_ipoib_rxq_init(struct net_device *netdev)
    {
    struct hfi1_ipoib_dev_priv *ipoib_priv = hfi1_ipoib_priv(netdev);
    struct hfi1_devdata *dd = ipoib_priv.dd;
    int ret;
    ret = hfi1_netdev_rx_init(dd);
    if (ret)
    return ret;
    hfi1_init_aip_rsm(dd);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hfi1_ipoib_rxq_deinit(netdev: *mut net_device) {
    void hfi1_ipoib_rxq_deinit(struct net_device *netdev)
    {
    struct hfi1_ipoib_dev_priv *ipoib_priv = hfi1_ipoib_priv(netdev);
    struct hfi1_devdata *dd = ipoib_priv.dd;
    hfi1_deinit_aip_rsm(dd);
    hfi1_netdev_rx_destroy(dd);
    }
