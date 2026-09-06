//! Automatically rewritten from C to Rust
//! Source: net/ipv6/seg6_hmac.c
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
// SR-IPv6 implementation -- HMAC functions
//
// Author:
// David Lebrun <david.lebrun@uclouvain.be>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmac_storage {
    pub bh_lock: local_lock_t,
    pub hmac_ring: [c_char; SEG6_HMAC_RING_SIZE],
}

    static DEFINE_PER_CPU(struct hmac_storage, hmac_storage) = {
    .bh_lock = INIT_LOCAL_LOCK(bh_lock),
    };
#[no_mangle]
unsafe extern "C" fn seg6_hmac_cmpfn(arg: *mut rhashtable_compare_arg, obj: *const c_void) -> c_int {
    static int seg6_hmac_cmpfn(struct rhashtable_compare_arg *arg, const void *obj)
    {
    const struct seg6_hmac_info *hinfo = obj;
    return (hinfo.hmackeyid != *(__u32 *)arg.key);
    }
#[no_mangle]
pub unsafe extern "C" fn seg6_hinfo_release(hinfo: *mut seg6_hmac_info) {
    static inline void seg6_hinfo_release(struct seg6_hmac_info *hinfo)
    {
    kfree_rcu(hinfo, rcu);
    }
#[no_mangle]
unsafe extern "C" fn seg6_free_hi(ptr: *mut c_void, arg: *mut c_void) {
    static void seg6_free_hi(void *ptr, void *arg)
    {
    struct seg6_hmac_info *hinfo = (struct seg6_hmac_info *)ptr;
    if (hinfo)
    seg6_hinfo_release(hinfo);
    }
    static const struct rhashtable_params rht_params = {
    .head_offset		= offsetof(struct seg6_hmac_info, node),
    .key_offset		= offsetof(struct seg6_hmac_info, hmackeyid),
    .key_len		= sizeof(u32),
    .automatic_shrinking	= true,
    .obj_cmpfn		= seg6_hmac_cmpfn,
    };
    static struct sr6_tlv_hmac *seg6_get_tlv_hmac(struct ipv6_sr_hdr *srh)
    {
    struct sr6_tlv_hmac *tlv;
    if (srh.hdrlen < (srh.first_segment + 1) * 2 + 5)
    return core::ptr::null_mut();
    if (!sr_has_hmac(srh))
    return core::ptr::null_mut();
    tlv = (struct sr6_tlv_hmac *)
    ((char *)srh + ((srh.hdrlen + 1) << 3) - 40);
    if (tlv.tlvhdr.type != SR6_TLV_HMAC || tlv.tlvhdr.len != 38)
    return core::ptr::null_mut();
    return tlv;
    }
    int seg6_hmac_compute(struct seg6_hmac_info *hinfo, struct ipv6_sr_hdr *hdr,
    struct in6_addr *saddr, u8 *output)
    {
    let mut hmackeyid: __be32 = cpu_to_be32(hinfo.hmackeyid);
    int plen, i, ret = 0;
    char *ring, *off;
// saddr(16) + first_seg(1) + flags(1) + keyid(4) + seglist(16n)
    plen = 16 + 1 + 1 + 4 + (hdr.first_segment + 1) * 16;
// this limit allows for 14 segments
    if (plen >= SEG6_HMAC_RING_SIZE)
    return -EMSGSIZE;
// Let's build the HMAC text on the ring buffer. The text is composed
// as follows, in order:
//
// 1. Source IPv6 address (128 bits)
// 2. first_segment value (8 bits)
// 3. Flags (8 bits)
// 4. HMAC Key ID (32 bits)
// 5. All segments in the segments list (n * 128 bits)
//
    local_bh_disable();
    local_lock_nested_bh(&hmac_storage.bh_lock);
    ring = this_cpu_ptr(hmac_storage.hmac_ring);
    off = ring;
// source address
    memcpy(off, saddr, 16);
    off += 16;
// first_segment value
// off++ = hdr->first_segment;
// flags
// off++ = hdr->flags;
// HMAC Key ID
    memcpy(off, &hmackeyid, 4);
    off += 4;
// all segments in the list
    for (i = 0; i < hdr.first_segment + 1; i++) {
    memcpy(off, hdr.segments + i, 16);
    off += 16;
    }
    switch (hinfo.alg_id) {
    case SEG6_HMAC_ALGO_SHA1:
    hmac_sha1(&hinfo.key.sha1, ring, plen, output);
    static_assert(SEG6_HMAC_FIELD_LEN > SHA1_DIGEST_SIZE);
    memset(&output[SHA1_DIGEST_SIZE], 0,
    SEG6_HMAC_FIELD_LEN - SHA1_DIGEST_SIZE);
    break;
    case SEG6_HMAC_ALGO_SHA256:
    hmac_sha256(&hinfo.key.sha256, ring, plen, output);
    static_assert(SEG6_HMAC_FIELD_LEN == SHA256_DIGEST_SIZE);
    break;
    default:
    WARN_ON_ONCE(1);
    ret = -EINVAL;
    break;
    }
    local_unlock_nested_bh(&hmac_storage.bh_lock);
    local_bh_enable();
    return ret;
    }
    EXPORT_SYMBOL(seg6_hmac_compute);
// checks if an incoming SR-enabled packet's HMAC status matches
// the incoming policy.
//
// called with rcu_read_lock()
//
#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_validate_skb(skb: *mut sk_buff) -> bool {
    bool seg6_hmac_validate_skb(struct sk_buff *skb)
    {
    u8 hmac_output[SEG6_HMAC_FIELD_LEN];
    struct net *net = dev_net(skb.dev);
    struct seg6_hmac_info *hinfo;
    struct sr6_tlv_hmac *tlv;
    struct ipv6_sr_hdr *srh;
    struct inet6_dev *idev;
    int require_hmac;
    idev = __in6_dev_get(skb.dev);
    if (!idev)
    return false;
    srh = (struct ipv6_sr_hdr *)skb_transport_header(skb);
    tlv = seg6_get_tlv_hmac(srh);
    require_hmac = READ_ONCE(idev.cnf.seg6_require_hmac);
// mandatory check but no tlv
    if (require_hmac > 0 && !tlv)
    return false;
// no check
    if (require_hmac < 0)
    return true;
// check only if present
    if (require_hmac == 0 && !tlv)
    return true;
// now, seg6_require_hmac >= 0 && tlv
    hinfo = seg6_hmac_info_lookup(net, be32_to_cpu(tlv.hmackeyid));
    if (!hinfo)
    return false;
    if (seg6_hmac_compute(hinfo, srh, &ipv6_hdr(skb).saddr, hmac_output))
    return false;
    if (crypto_memneq(hmac_output, tlv.hmac, SEG6_HMAC_FIELD_LEN))
    return false;
    return true;
    }
    EXPORT_SYMBOL(seg6_hmac_validate_skb);
// called with rcu_read_lock()
    struct seg6_hmac_info *seg6_hmac_info_lookup(struct net *net, u32 key)
    {
    struct seg6_pernet_data *sdata = seg6_pernet(net);
    struct seg6_hmac_info *hinfo;
    hinfo = rhashtable_lookup_fast(&sdata.hmac_infos, &key, rht_params);
    return hinfo;
    }
    EXPORT_SYMBOL(seg6_hmac_info_lookup);
#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_info_add(net: *mut net, key: u32, hinfo: *mut seg6_hmac_info) -> c_int {
    int seg6_hmac_info_add(struct net *net, u32 key, struct seg6_hmac_info *hinfo)
    {
    struct seg6_pernet_data *sdata = seg6_pernet(net);
    int err;
    switch (hinfo.alg_id) {
    case SEG6_HMAC_ALGO_SHA1:
    hmac_sha1_preparekey(&hinfo.key.sha1,
    hinfo.secret, hinfo.slen);
    break;
    case SEG6_HMAC_ALGO_SHA256:
    hmac_sha256_preparekey(&hinfo.key.sha256,
    hinfo.secret, hinfo.slen);
    break;
    default:
    return -EINVAL;
    }
    err = rhashtable_lookup_insert_fast(&sdata.hmac_infos, &hinfo.node,
    rht_params);
    return err;
    }
    EXPORT_SYMBOL(seg6_hmac_info_add);
#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_info_del(net: *mut net, key: u32) -> c_int {
    int seg6_hmac_info_del(struct net *net, u32 key)
    {
    struct seg6_pernet_data *sdata = seg6_pernet(net);
    struct seg6_hmac_info *hinfo;
    let mut err: c_int = -ENOENT;
    hinfo = rhashtable_lookup_fast(&sdata.hmac_infos, &key, rht_params);
    if (!hinfo)
    goto out;
    err = rhashtable_remove_fast(&sdata.hmac_infos, &hinfo.node,
    rht_params);
    if (err)
    goto out;
    seg6_hinfo_release(hinfo);
    out:
    return err;
    }
    EXPORT_SYMBOL(seg6_hmac_info_del);
    int seg6_push_hmac(struct net *net, struct in6_addr *saddr,
    struct ipv6_sr_hdr *srh)
    {
    struct seg6_hmac_info *hinfo;
    struct sr6_tlv_hmac *tlv;
    let mut err: c_int = -ENOENT;
    tlv = seg6_get_tlv_hmac(srh);
    if (!tlv)
    return -EINVAL;
    rcu_read_lock();
    hinfo = seg6_hmac_info_lookup(net, be32_to_cpu(tlv.hmackeyid));
    if (!hinfo)
    goto out;
    memset(tlv.hmac, 0, SEG6_HMAC_FIELD_LEN);
    err = seg6_hmac_compute(hinfo, srh, saddr, tlv.hmac);
    out:
    rcu_read_unlock();
    return err;
    }
    EXPORT_SYMBOL(seg6_push_hmac);
#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_net_init(net: *mut net) -> int __net_init {
    int __net_init seg6_hmac_net_init(struct net *net)
    {
    struct seg6_pernet_data *sdata = seg6_pernet(net);
    return rhashtable_init(&sdata.hmac_infos, &rht_params);
    }
#[no_mangle]
pub unsafe extern "C" fn seg6_hmac_net_exit(net: *mut net) -> void __net_exit {
    void __net_exit seg6_hmac_net_exit(struct net *net)
    {
    struct seg6_pernet_data *sdata = seg6_pernet(net);
    rhashtable_free_and_destroy(&sdata.hmac_infos, seg6_free_hi, core::ptr::null_mut());
    }
    EXPORT_SYMBOL(seg6_hmac_net_exit);
