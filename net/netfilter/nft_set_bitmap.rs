//! Automatically rewritten from C to Rust
//! Source: net/netfilter/nft_set_bitmap.c
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
// Copyright (c) 2017 Pablo Neira Ayuso <pablo@netfilter.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_bitmap_elem {
    pub priv: nft_elem_priv,
    pub head: list_head,
    pub ext: nft_set_ext,
}

// This bitmap uses two bits to represent one element. These two bits determine
// the element state in the current and the future generation.
//
// An element can be in three states. The generation cursor is represented using
// the ^ character, note that this cursor shifts on every successful transaction.
// If no transaction is going on, we observe all elements are in the following
// state:
//
// 11 = this element is active in the current generation. In case of no updates,
// ^    it stays active in the next generation.
// 00 = this element is inactive in the current generation. In case of no
// ^    updates, it stays inactive in the next generation.
//
// On transaction handling, we observe these two temporary states:
//
// 01 = this element is inactive in the current generation and it becomes active
// ^    in the next one. This happens when the element is inserted but commit
// path has not yet been executed yet, so activation is still pending. On
// transaction abortion, the element is removed.
// 10 = this element is active in the current generation and it becomes inactive
// ^    in the next one. This happens when the element is deactivated but commit
// path has not yet been executed yet, so removal is still pending. On
// transaction abortion, the next generation bit is reset to go back to
// restore its previous state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nft_bitmap {
    pub list: list_head,
    pub bitmap_size: u16,
    pub bitmap: [u8; ],
}

    static inline void nft_bitmap_location(const struct nft_set *set,
    const void *key,
    u32 *idx, u32 *off)
    {
    u32 k;
    if (set.klen == 2)
    k = *(u16 *)key;
    else
    k = *(u8 *)key;
    k <<= 1;
// idx = k / BITS_PER_BYTE;
// off = k % BITS_PER_BYTE;
    }
// Fetch the two bits that represent the element and check if it is active based
// on the generation mask.
//
    static inline bool
    nft_bitmap_active(const u8 *bitmap, u32 idx, u32 off, u8 genmask)
    {
    return (bitmap[idx] & (0x3 << off)) & (genmask << off);
    }
    INDIRECT_CALLABLE_SCOPE
    const struct nft_set_ext *
    nft_bitmap_lookup(const struct net *net, const struct nft_set *set,
    const u32 *key)
    {
    const struct nft_bitmap *priv = nft_set_priv(set);
    static const struct nft_set_ext found;
    let mut genmask: u8 = nft_genmask_cur(net);
    u32 idx, off;
    nft_bitmap_location(set, key, &idx, &off);
    if (nft_bitmap_active(priv.bitmap, idx, off, genmask))
    return &found;
    return core::ptr::null_mut();
    }
    static struct nft_bitmap_elem *
    nft_bitmap_elem_find(const struct net *net,
    const struct nft_set *set, struct nft_bitmap_elem *this,
    u8 genmask)
    {
    const struct nft_bitmap *priv = nft_set_priv(set);
    struct nft_bitmap_elem *be;
    list_for_each_entry_rcu(be, &priv.list, head,
    lockdep_is_held(&nft_pernet(net).commit_mutex)) {
    if (memcmp(nft_set_ext_key(&be.ext),
    nft_set_ext_key(&this.ext), set.klen) ||
    !nft_set_elem_active(&be.ext, genmask))
    continue;
    return be;
    }
    return core::ptr::null_mut();
    }
    static struct nft_elem_priv *
    nft_bitmap_get(const struct net *net, const struct nft_set *set,
    const struct nft_set_elem *elem, unsigned int flags)
    {
    const struct nft_bitmap *priv = nft_set_priv(set);
    let mut genmask: u8 = nft_genmask_cur(net);
    struct nft_bitmap_elem *be;
    list_for_each_entry_rcu(be, &priv.list, head) {
    if (memcmp(nft_set_ext_key(&be.ext), elem.key.val.data, set.klen) ||
    !nft_set_elem_active(&be.ext, genmask))
    continue;
    return &be.priv;
    }
    return ERR_PTR(-ENOENT);
    }
    static int nft_bitmap_insert(const struct net *net, const struct nft_set *set,
    const struct nft_set_elem *elem,
    struct nft_elem_priv **elem_priv)
    {
    struct nft_bitmap_elem *new = nft_elem_priv_cast(elem.priv), *be;
    struct nft_bitmap *priv = nft_set_priv(set);
    let mut genmask: u8 = nft_genmask_next(net);
    u32 idx, off;
    be = nft_bitmap_elem_find(net, set, new, genmask);
    if (be) {
// elem_priv = &be->priv;
    return -EEXIST;
    }
    nft_bitmap_location(set, nft_set_ext_key(&new.ext), &idx, &off);
// Enter 01 state.
    priv.bitmap[idx] |= (genmask << off);
    list_add_tail_rcu(&new.head, &priv.list);
    return 0;
    }
    static void nft_bitmap_remove(const struct net *net, const struct nft_set *set,
    struct nft_elem_priv *elem_priv)
    {
    struct nft_bitmap_elem *be = nft_elem_priv_cast(elem_priv);
    struct nft_bitmap *priv = nft_set_priv(set);
    let mut genmask: u8 = nft_genmask_next(net);
    u32 idx, off;
    nft_bitmap_location(set, nft_set_ext_key(&be.ext), &idx, &off);
// Enter 00 state.
    priv.bitmap[idx] &= ~(genmask << off);
    list_del_rcu(&be.head);
    }
    static void nft_bitmap_activate(const struct net *net,
    const struct nft_set *set,
    struct nft_elem_priv *elem_priv)
    {
    struct nft_bitmap_elem *be = nft_elem_priv_cast(elem_priv);
    struct nft_bitmap *priv = nft_set_priv(set);
    let mut genmask: u8 = nft_genmask_next(net);
    u32 idx, off;
    nft_bitmap_location(set, nft_set_ext_key(&be.ext), &idx, &off);
// Enter 11 state.
    priv.bitmap[idx] |= (genmask << off);
    nft_clear(net, &be.ext);
    }
    static void nft_bitmap_flush(const struct net *net,
    const struct nft_set *set,
    struct nft_elem_priv *elem_priv)
    {
    struct nft_bitmap_elem *be = nft_elem_priv_cast(elem_priv);
    struct nft_bitmap *priv = nft_set_priv(set);
    let mut genmask: u8 = nft_genmask_next(net);
    u32 idx, off;
    nft_bitmap_location(set, nft_set_ext_key(&be.ext), &idx, &off);
// Enter 10 state, similar to deactivation.
    priv.bitmap[idx] &= ~(genmask << off);
    nft_set_elem_change_active(net, set, &be.ext);
    }
    static struct nft_elem_priv *
    nft_bitmap_deactivate(const struct net *net, const struct nft_set *set,
    const struct nft_set_elem *elem)
    {
    struct nft_bitmap_elem *this = nft_elem_priv_cast(elem.priv), *be;
    struct nft_bitmap *priv = nft_set_priv(set);
    let mut genmask: u8 = nft_genmask_next(net);
    u32 idx, off;
    nft_bitmap_location(set, elem.key.val.data, &idx, &off);
    be = nft_bitmap_elem_find(net, set, this, genmask);
    if (!be)
    return core::ptr::null_mut();
// Enter 10 state.
    priv.bitmap[idx] &= ~(genmask << off);
    nft_set_elem_change_active(net, set, &be.ext);
    return &be.priv;
    }
    static void nft_bitmap_walk(const struct nft_ctx *ctx,
    struct nft_set *set,
    struct nft_set_iter *iter)
    {
    const struct nft_bitmap *priv = nft_set_priv(set);
    struct nft_bitmap_elem *be;
    list_for_each_entry_rcu(be, &priv.list, head,
    lockdep_is_held(&nft_pernet(ctx.net).commit_mutex)) {
    if (iter.count < iter.skip)
    goto cont;
    iter.err = iter.fn(ctx, set, iter, &be.priv);
    if (iter.err < 0)
    return;
    cont:
    iter.count++;
    }
    }
// The bitmap size is pow(2, key length in bits) / bits per byte. This is
// multiplied by two since each element takes two bits. For 8 bit keys, the
// bitmap consumes 66 bytes. For 16 bit keys, 16388 bytes.
//
#[no_mangle]
pub unsafe extern "C" fn nft_bitmap_size(klen: u32) -> u32 {
    static inline u32 nft_bitmap_size(u32 klen)
    {
    return ((2 << ((klen * BITS_PER_BYTE) - 1)) / BITS_PER_BYTE) << 1;
    }
#[no_mangle]
pub unsafe extern "C" fn nft_bitmap_total_size(klen: u32) -> u64 {
    static inline u64 nft_bitmap_total_size(u32 klen)
    {
    return sizeof(struct nft_bitmap) + nft_bitmap_size(klen);
    }
    static u64 nft_bitmap_privsize(const struct nlattr * const nla[],
    const struct nft_set_desc *desc)
    {
    let mut klen: u32 = ntohl(nla_get_be32(nla[NFTA_SET_KEY_LEN]));
    return nft_bitmap_total_size(klen);
    }
    static int nft_bitmap_init(const struct nft_set *set,
    const struct nft_set_desc *desc,
    const struct nlattr * const nla[])
    {
    struct nft_bitmap *priv = nft_set_priv(set);
    BUILD_BUG_ON(offsetof(struct nft_bitmap_elem, priv) != 0);
    INIT_LIST_HEAD(&priv.list);
    priv.bitmap_size = nft_bitmap_size(set.klen);
    return 0;
    }
    static void nft_bitmap_destroy(const struct nft_ctx *ctx,
    const struct nft_set *set)
    {
    struct nft_bitmap *priv = nft_set_priv(set);
    struct nft_bitmap_elem *be, *n;
    list_for_each_entry_safe(be, n, &priv.list, head)
    nf_tables_set_elem_destroy(ctx, set, &be.priv);
    }
    static bool nft_bitmap_estimate(const struct nft_set_desc *desc, u32 features,
    struct nft_set_estimate *est)
    {
// Make sure bitmaps we don't get bitmaps larger than 16 Kbytes.
    if (desc.klen > 2)
    return false;
#[no_mangle]
pub unsafe extern "C" fn if(_arg: desc->expr) -> else {
    else if (desc.expr)
    return false;
    est.size   = nft_bitmap_total_size(desc.klen);
    est.lookup = NFT_SET_CLASS_O_1;
    est.space  = NFT_SET_CLASS_O_1;
    return true;
    }
    const struct nft_set_type nft_set_bitmap_type = {
    .ops		= {
    .privsize	= nft_bitmap_privsize,
    .elemsize	= offsetof(struct nft_bitmap_elem, ext),
    .estimate	= nft_bitmap_estimate,
    .init		= nft_bitmap_init,
    .destroy	= nft_bitmap_destroy,
    .insert		= nft_bitmap_insert,
    .remove		= nft_bitmap_remove,
    .deactivate	= nft_bitmap_deactivate,
    .flush		= nft_bitmap_flush,
    .activate	= nft_bitmap_activate,
    .lookup		= nft_bitmap_lookup,
    .walk		= nft_bitmap_walk,
    .get		= nft_bitmap_get,
    },
    };
