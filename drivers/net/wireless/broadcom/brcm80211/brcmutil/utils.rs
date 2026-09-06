//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmutil/utils.c
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

    MODULE_AUTHOR("Broadcom Corporation");
    MODULE_DESCRIPTION("Broadcom 802.11n wireless LAN driver utilities.");
    MODULE_LICENSE("Dual BSD/GPL");
    struct sk_buff *brcmu_pkt_buf_get_skb(uint len)
    {
    struct sk_buff *skb;
    skb = dev_alloc_skb(len);
    if (skb) {
    skb_put(skb, len);
    skb.priority = 0;
    }
    return skb;
    }
    EXPORT_SYMBOL(brcmu_pkt_buf_get_skb);
// Free the driver packet. Free the tag if present
#[no_mangle]
pub unsafe extern "C" fn brcmu_pkt_buf_free_skb(skb: *mut sk_buff) {
    void brcmu_pkt_buf_free_skb(struct sk_buff *skb)
    {
    if (!skb)
    return;
    WARN_ON(skb.next);
    dev_kfree_skb_any(skb);
    }
    EXPORT_SYMBOL(brcmu_pkt_buf_free_skb);
//
// osl multiple-precedence packet queue
// hi_prec is always >= the number of the highest non-empty precedence
//
    struct sk_buff *brcmu_pktq_penq(struct pktq *pq, int prec,
    struct sk_buff *p)
    {
    struct sk_buff_head *q;
    if (pktq_full(pq) || pktq_pfull(pq, prec))
    return core::ptr::null_mut();
    q = &pq.q[prec].skblist;
    skb_queue_tail(q, p);
    pq.len++;
    if (pq.hi_prec < prec)
    pq.hi_prec = (u8) prec;
    return p;
    }
    EXPORT_SYMBOL(brcmu_pktq_penq);
    struct sk_buff *brcmu_pktq_penq_head(struct pktq *pq, int prec,
    struct sk_buff *p)
    {
    struct sk_buff_head *q;
    if (pktq_full(pq) || pktq_pfull(pq, prec))
    return core::ptr::null_mut();
    q = &pq.q[prec].skblist;
    skb_queue_head(q, p);
    pq.len++;
    if (pq.hi_prec < prec)
    pq.hi_prec = (u8) prec;
    return p;
    }
    EXPORT_SYMBOL(brcmu_pktq_penq_head);
    struct sk_buff *brcmu_pktq_pdeq(struct pktq *pq, int prec)
    {
    struct sk_buff_head *q;
    struct sk_buff *p;
    q = &pq.q[prec].skblist;
    p = skb_dequeue(q);
    if (p == core::ptr::null_mut())
    return core::ptr::null_mut();
    pq.len--;
    return p;
    }
    EXPORT_SYMBOL(brcmu_pktq_pdeq);
//
// precedence based dequeue with match function. Passing a NULL pointer
// for the match function parameter is considered to be a wildcard so
// any packet on the queue is returned. In that case it is no different
// from brcmu_pktq_pdeq() above.
//
    struct sk_buff *brcmu_pktq_pdeq_match(struct pktq *pq, int prec,
    bool (*match_fn)(struct sk_buff *skb,
    void *arg), void *arg)
    {
    struct sk_buff_head *q;
    struct sk_buff *p, *next;
    q = &pq.q[prec].skblist;
    skb_queue_walk_safe(q, p, next) {
    if (match_fn == core::ptr::null_mut() || match_fn(p, arg)) {
    skb_unlink(p, q);
    pq.len--;
    return p;
    }
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(brcmu_pktq_pdeq_match);
    struct sk_buff *brcmu_pktq_pdeq_tail(struct pktq *pq, int prec)
    {
    struct sk_buff_head *q;
    struct sk_buff *p;
    q = &pq.q[prec].skblist;
    p = skb_dequeue_tail(q);
    if (p == core::ptr::null_mut())
    return core::ptr::null_mut();
    pq.len--;
    return p;
    }
    EXPORT_SYMBOL(brcmu_pktq_pdeq_tail);
    void
    brcmu_pktq_pflush(struct pktq *pq, int prec, bool dir,
    bool (*fn)(struct sk_buff *, void *), void *arg)
    {
    struct sk_buff_head *q;
    struct sk_buff *p, *next;
    q = &pq.q[prec].skblist;
    skb_queue_walk_safe(q, p, next) {
    if (fn == core::ptr::null_mut() || (*fn) (p, arg)) {
    skb_unlink(p, q);
    brcmu_pkt_buf_free_skb(p);
    pq.len--;
    }
    }
    }
    EXPORT_SYMBOL(brcmu_pktq_pflush);
    void brcmu_pktq_flush(struct pktq *pq, bool dir,
    bool (*fn)(struct sk_buff *, void *), void *arg)
    {
    int prec;
    for (prec = 0; prec < pq.num_prec; prec++)
    brcmu_pktq_pflush(pq, prec, dir, fn, arg);
    }
    EXPORT_SYMBOL(brcmu_pktq_flush);
#[no_mangle]
pub unsafe extern "C" fn brcmu_pktq_init(pq: *mut pktq, num_prec: c_int, max_len: c_int) {
    void brcmu_pktq_init(struct pktq *pq, int num_prec, int max_len)
    {
    int prec;
// pq is variable size; only zero out what's requested
    memset(pq, 0,
    offsetof(struct pktq, q) + (sizeof(struct pktq_prec) * num_prec));
    pq.num_prec = (u16) num_prec;
    pq.max = (u16) max_len;
    for (prec = 0; prec < num_prec; prec++) {
    pq.q[prec].max = pq.max;
    skb_queue_head_init(&pq.q[prec].skblist);
    }
    }
    EXPORT_SYMBOL(brcmu_pktq_init);
    struct sk_buff *brcmu_pktq_peek_tail(struct pktq *pq, int *prec_out)
    {
    int prec;
    if (pktq_empty(pq))
    return core::ptr::null_mut();
    for (prec = 0; prec < pq.hi_prec; prec++)
    if (!skb_queue_empty(&pq.q[prec].skblist))
    break;
    if (prec_out)
// prec_out = prec;
    return skb_peek_tail(&pq.q[prec].skblist);
    }
    EXPORT_SYMBOL(brcmu_pktq_peek_tail);
// Return sum of lengths of a specific set of precedences
#[no_mangle]
pub unsafe extern "C" fn brcmu_pktq_mlen(pq: *mut pktq, prec_bmp: c_uint) -> c_int {
    int brcmu_pktq_mlen(struct pktq *pq, uint prec_bmp)
    {
    int prec, len;
    len = 0;
    for (prec = 0; prec <= pq.hi_prec; prec++)
    if (prec_bmp & (1 << prec))
    len += pq.q[prec].skblist.qlen;
    return len;
    }
    EXPORT_SYMBOL(brcmu_pktq_mlen);
// Priority dequeue from a specific set of precedences
    struct sk_buff *brcmu_pktq_mdeq(struct pktq *pq, uint prec_bmp,
    int *prec_out)
    {
    struct sk_buff_head *q;
    struct sk_buff *p;
    int prec;
    if (pktq_empty(pq))
    return core::ptr::null_mut();
    while ((prec = pq.hi_prec) > 0 &&
    skb_queue_empty(&pq.q[prec].skblist))
    pq.hi_prec--;
    while ((prec_bmp & (1 << prec)) == 0 ||
    skb_queue_empty(&pq.q[prec].skblist))
    if (prec-- == 0)
    return core::ptr::null_mut();
    q = &pq.q[prec].skblist;
    p = skb_dequeue(q);
    if (p == core::ptr::null_mut())
    return core::ptr::null_mut();
    pq.len--;
    if (prec_out)
// prec_out = prec;
    return p;
    }
    EXPORT_SYMBOL(brcmu_pktq_mdeq);
// Produce a human-readable string for boardrev
    char *brcmu_boardrev_str(u32 brev, char *buf)
    {
    char c;
    if (brev < 0x100) {
    snprintf(buf, BRCMU_BOARDREV_LEN, "%d.%d",
    (brev & 0xf0) >> 4, brev & 0xf);
    } else {
    c = (brev & 0xf000) == 0x1000 ? 'P' : 'A';
    snprintf(buf, BRCMU_BOARDREV_LEN, "%c%03x", c, brev & 0xfff);
    }
    return buf;
    }
    EXPORT_SYMBOL(brcmu_boardrev_str);
    char *brcmu_dotrev_str(u32 dotrev, char *buf)
    {
    u8 dotval[4];
    if (!dotrev) {
    snprintf(buf, BRCMU_DOTREV_LEN, "unknown");
    return buf;
    }
    dotval[0] = (dotrev >> 24) & 0xFF;
    dotval[1] = (dotrev >> 16) & 0xFF;
    dotval[2] = (dotrev >> 8) & 0xFF;
    dotval[3] = dotrev & 0xFF;
    if (dotval[3])
    snprintf(buf, BRCMU_DOTREV_LEN, "%d.%d.%d.%d", dotval[0],
    dotval[1], dotval[2], dotval[3]);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: dotval[2]) -> else {
    else if (dotval[2])
    snprintf(buf, BRCMU_DOTREV_LEN, "%d.%d.%d", dotval[0],
    dotval[1], dotval[2]);
    else
    snprintf(buf, BRCMU_DOTREV_LEN, "%d.%d", dotval[0],
    dotval[1]);
    return buf;
    }
    EXPORT_SYMBOL(brcmu_dotrev_str);

// pretty hex print a pkt buffer chain
#[no_mangle]
pub unsafe extern "C" fn brcmu_prpkt(msg: *const c_char, p0: *mut sk_buff) {
    void brcmu_prpkt(const char *msg, struct sk_buff *p0)
    {
    struct sk_buff *p;
    if (msg && (msg[0] != '\0'))
    pr_debug("%s:\n", msg);
    for (p = p0; p; p = p.next)
    print_hex_dump_bytes("", DUMP_PREFIX_OFFSET, p.data, p.len);
    }
    EXPORT_SYMBOL(brcmu_prpkt);
#[no_mangle]
pub unsafe extern "C" fn brcmu_dbg_hex_dump(data: *const c_void, size: usize, fmt: *const c_char, ...) {
    void brcmu_dbg_hex_dump(const void *data, size_t size, const char *fmt, ...)
    {
    struct va_format vaf;
    va_list args;
    va_start(args, fmt);
    vaf.fmt = fmt;
    vaf.va = &args;
    pr_debug("%pV", &vaf);
    va_end(args);
    print_hex_dump_bytes("", DUMP_PREFIX_OFFSET, data, size);
    }
    EXPORT_SYMBOL(brcmu_dbg_hex_dump);
