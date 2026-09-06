//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nf_conntrack_labels.c
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
//
// test/set flag bits stored in conntrack extension area.
//
// (C) 2013 Astaro GmbH & Co KG
//

#[no_mangle]
unsafe extern "C" fn replace_u32(address: *mut u32, mask: u32, new: u32) -> c_int {
    static int replace_u32(u32 *address, u32 mask, u32 new)
    {
    u32 old, tmp;
    do {
    old = *address;
    tmp = (old & mask) ^ new;
    if (old == tmp)
    return 0;
    } while (cmpxchg(address, old, tmp) != old);
    return 1;
    }
    int nf_connlabels_replace(struct nf_conn *ct,
    const u32 *data,
    const u32 *mask, unsigned int words32)
    {
    struct nf_conn_labels *labels;
    unsigned int size, i;
    let mut changed: c_int = 0;
    u32 *dst;
    labels = nf_ct_labels_find(ct);
    if (!labels)
    return -ENOSPC;
    size = sizeof(labels.bits);
    if (size < (words32 * sizeof(u32)))
    words32 = size / sizeof(u32);
    dst = (u32 *) labels.bits;
    for (i = 0; i < words32; i++)
    changed |= replace_u32(&dst[i], mask ? ~mask[i] : 0, data[i]);
    size /= sizeof(u32);
    for (i = words32; i < size; i++) /* pad */
    replace_u32(&dst[i], 0, 0);
    if (changed)
    nf_conntrack_event_cache(IPCT_LABEL, ct);
    return 0;
    }
    EXPORT_SYMBOL_GPL(nf_connlabels_replace);
#[no_mangle]
pub unsafe extern "C" fn nf_connlabels_get(net: *mut net, bits: c_uint) -> c_int {
    int nf_connlabels_get(struct net *net, unsigned int bits)
    {
    int v;
    if (BIT_WORD(bits) >= NF_CT_LABELS_MAX_SIZE / sizeof(long))
    return -ERANGE;
    BUILD_BUG_ON(NF_CT_LABELS_MAX_SIZE / sizeof(long) >= U8_MAX);
    v = atomic_inc_return_relaxed(&net.ct.labels_used);
    WARN_ON_ONCE(v <= 0);
    return 0;
    }
    EXPORT_SYMBOL_GPL(nf_connlabels_get);
#[no_mangle]
pub unsafe extern "C" fn nf_connlabels_put(net: *mut net) {
    void nf_connlabels_put(struct net *net)
    {
    let mut v: c_int = atomic_dec_return_relaxed(&net.ct.labels_used);
    WARN_ON_ONCE(v < 0);
    }
    EXPORT_SYMBOL_GPL(nf_connlabels_put);
