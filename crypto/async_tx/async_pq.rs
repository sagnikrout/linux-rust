//! Automatically rewritten from C to Rust
//! Source: crypto/async_tx/async_pq.c
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
// Copyright(c) 2007 Yuri Tikhonov <yur@emcraft.com>
// Copyright(c) 2009 Intel Corporation
//

//
// struct pq_scribble_page - space to hold throwaway P or Q buffer for
// synchronous gen_syndrome
//
    static struct page *pq_scribble_page;
// the struct page *blocks[] parameter passed to async_gen_syndrome()
// and async_syndrome_val() contains the 'P' destination address at
// blocks[disks-2] and the 'Q' destination address at blocks[disks-1]
//
// note: these are macros as they are used as lvalues
//

pub const MAX_DISKS: c_int = 255;
//
// do_async_gen_syndrome - asynchronously calculate P and/or Q
//
    static __async_inline struct dma_async_tx_descriptor *
    do_async_gen_syndrome(struct dma_chan *chan,
    const unsigned char *scfs, int disks,
    struct dmaengine_unmap_data *unmap,
    enum dma_ctrl_flags dma_flags,
    struct async_submit_ctl *submit)
    {
    struct dma_async_tx_descriptor *tx = core::ptr::null_mut();
    struct dma_device *dma = chan.device;
    let mut flags_orig: enum async_tx_flags = submit.flags;
    let mut cb_fn_orig: dma_async_tx_callback = submit.cb_fn;
    let mut cb_param_orig: dma_async_tx_callback = submit.cb_param;
    let mut src_cnt: c_int = disks - 2;
    unsigned short pq_src_cnt;
    dma_addr_t dma_dest[2];
    let mut src_off: c_int = 0;
    while (src_cnt > 0) {
    submit.flags = flags_orig;
    pq_src_cnt = min(src_cnt, dma_maxpq(dma, dma_flags));
// if we are submitting additional pqs, leave the chain open,
// clear the callback parameters, and leave the destination
// buffers mapped
//
    if (src_cnt > pq_src_cnt) {
    submit.flags &= ~ASYNC_TX_ACK;
    submit.flags |= ASYNC_TX_FENCE;
    submit.cb_fn = core::ptr::null_mut();
    submit.cb_param = core::ptr::null_mut();
    } else {
    submit.cb_fn = cb_fn_orig;
    submit.cb_param = cb_param_orig;
    if (cb_fn_orig)
    dma_flags |= DMA_PREP_INTERRUPT;
    }
    if (submit.flags & ASYNC_TX_FENCE)
    dma_flags |= DMA_PREP_FENCE;
// Drivers force forward progress in case they can not provide
// a descriptor
//
    for (;;) {
    dma_dest[0] = unmap.addr[disks - 2];
    dma_dest[1] = unmap.addr[disks - 1];
    tx = dma.device_prep_dma_pq(chan, dma_dest,
    &unmap.addr[src_off],
    pq_src_cnt,
    &scfs[src_off], unmap.len,
    dma_flags);
    if (likely(tx))
    break;
    async_tx_quiesce(&submit.depend_tx);
    dma_async_issue_pending(chan);
    }
    dma_set_unmap(tx, unmap);
    async_tx_submit(chan, tx, submit);
    submit.depend_tx = tx;
// drop completed sources
    src_cnt -= pq_src_cnt;
    src_off += pq_src_cnt;
    dma_flags |= DMA_PREP_CONTINUE;
    }
    return tx;
    }
//
// do_sync_gen_syndrome - synchronously calculate a raid6 syndrome
//
    static void
    do_sync_gen_syndrome(struct page **blocks, unsigned int *offsets, int disks,
    size_t len, struct async_submit_ctl *submit)
    {
    void **srcs;
    int i;
    let mut start: c_int = -1, stop = disks - 3;
    if (submit.scribble)
    srcs = submit.scribble;
    else
    srcs = (void **) blocks;
    for (i = 0; i < disks; i++) {
    if (blocks[i] == core::ptr::null_mut()) {
    BUG_ON(i > disks - 3); /* P or Q can't be zero */
    srcs[i] = page_address(ZERO_PAGE(0));
    } else {
    srcs[i] = page_address(blocks[i]) + offsets[i];
    if (i < disks - 2) {
    stop = i;
    if (start == -1)
    start = i;
    }
    }
    }
    if (submit.flags & ASYNC_TX_PQ_XOR_DST) {
    BUG_ON(!raid6_can_xor_syndrome());
    if (start >= 0)
    raid6_xor_syndrome(disks, start, stop, len, srcs);
    } else
    raid6_gen_syndrome(disks, len, srcs);
    async_tx_sync_epilog(submit);
    }
    static inline bool
    is_dma_pq_aligned_offs(struct dma_device *dev, unsigned int *offs,
    int src_cnt, size_t len)
    {
    int i;
    for (i = 0; i < src_cnt; i++) {
    if (!is_dma_pq_aligned(dev, offs[i], 0, len))
    return false;
    }
    return true;
    }
//
// async_gen_syndrome - asynchronously calculate a raid6 syndrome
// @blocks: source blocks from idx 0..disks-3, P @ disks-2 and Q @ disks-1
// @offsets: offset array into each block (src and dest) to start transaction
// @disks: number of blocks (including missing P or Q, see below)
// @len: length of operation in bytes
// @submit: submission/completion modifiers
//
// General note: This routine assumes a field of GF(2^8) with a
// primitive polynomial of 0x11d and a generator of {02}.
//
// 'disks' note: callers can optionally omit either P or Q (but not
// both) from the calculation by setting blocks[disks-2] or
// blocks[disks-1] to NULL.  When P or Q is omitted 'len' must be <=
// PAGE_SIZE as a temporary buffer of this size is used in the
// synchronous path.  'disks' always accounts for both destination
// buffers.  If any source buffers (blocks[i] where i < disks - 2) are
// set to NULL those buffers will be replaced with the raid6_zero_page
// in the synchronous path and omitted in the hardware-asynchronous
// path.
//
    struct dma_async_tx_descriptor *
    async_gen_syndrome(struct page **blocks, unsigned int *offsets, int disks,
    size_t len, struct async_submit_ctl *submit)
    {
    let mut src_cnt: c_int = disks - 2;
    struct dma_chan *chan = async_tx_find_channel(submit, DMA_PQ,
    &P(blocks, disks), 2,
    blocks, src_cnt, len);
    struct dma_device *device = chan ? chan.device : core::ptr::null_mut();
    struct dmaengine_unmap_data *unmap = core::ptr::null_mut();
    BUG_ON(disks > MAX_DISKS || !(P(blocks, disks) || Q(blocks, disks)));
    if (device)
    unmap = dmaengine_get_unmap_data(device.dev, disks, GFP_NOWAIT);
// XORing P/Q is only implemented in software
    if (unmap && !(submit.flags & ASYNC_TX_PQ_XOR_DST) &&
    (src_cnt <= dma_maxpq(device, 0) ||
    dma_maxpq(device, DMA_PREP_CONTINUE) > 0) &&
    is_dma_pq_aligned_offs(device, offsets, disks, len)) {
    struct dma_async_tx_descriptor *tx;
    let mut dma_flags: enum dma_ctrl_flags = 0;
    unsigned char coefs[MAX_DISKS];
    int i, j;
// run the p+q asynchronously
    pr_debug("%s: (async) disks: %d len: %zu\n",
    __func__, disks, len);
// convert source addresses being careful to collapse 'empty'
// sources and update the coefficients accordingly
//
    unmap.len = len;
    for (i = 0, j = 0; i < src_cnt; i++) {
    if (blocks[i] == core::ptr::null_mut())
    continue;
    unmap.addr[j] = dma_map_page(device.dev, blocks[i],
    offsets[i], len, DMA_TO_DEVICE);
    coefs[j] = raid6_gfexp[i];
    unmap.to_cnt++;
    j++;
    }
//
// DMAs use destinations as sources,
// so use BIDIRECTIONAL mapping
//
    unmap.bidi_cnt++;
    if (P(blocks, disks))
    unmap.addr[j++] = dma_map_page(device.dev, P(blocks, disks),
    P(offsets, disks),
    len, DMA_BIDIRECTIONAL);
    else {
    unmap.addr[j++] = 0;
    dma_flags |= DMA_PREP_PQ_DISABLE_P;
    }
    unmap.bidi_cnt++;
    if (Q(blocks, disks))
    unmap.addr[j++] = dma_map_page(device.dev, Q(blocks, disks),
    Q(offsets, disks),
    len, DMA_BIDIRECTIONAL);
    else {
    unmap.addr[j++] = 0;
    dma_flags |= DMA_PREP_PQ_DISABLE_Q;
    }
    tx = do_async_gen_syndrome(chan, coefs, j, unmap, dma_flags, submit);
    dmaengine_unmap_put(unmap);
    return tx;
    }
    dmaengine_unmap_put(unmap);
// run the pq synchronously
    pr_debug("%s: (sync) disks: %d len: %zu\n", __func__, disks, len);
// wait for any prerequisite operations
    async_tx_quiesce(&submit.depend_tx);
    if (!P(blocks, disks)) {
    P(blocks, disks) = pq_scribble_page;
    P(offsets, disks) = 0;
    }
    if (!Q(blocks, disks)) {
    Q(blocks, disks) = pq_scribble_page;
    Q(offsets, disks) = 0;
    }
    do_sync_gen_syndrome(blocks, offsets, disks, len, submit);
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL_GPL(async_gen_syndrome);
    static inline struct dma_chan *
    pq_val_chan(struct async_submit_ctl *submit, struct page **blocks, int disks, size_t len)
    {

    return core::ptr::null_mut();

    return async_tx_find_channel(submit, DMA_PQ_VAL, core::ptr::null_mut(), 0,  blocks,
    disks, len);
    }
//
// async_syndrome_val - asynchronously validate a raid6 syndrome
// @blocks: source blocks from idx 0..disks-3, P @ disks-2 and Q @ disks-1
// @offsets: common offset into each block (src and dest) to start transaction
// @disks: number of blocks (including missing P or Q, see below)
// @len: length of operation in bytes
// @pqres: on val failure SUM_CHECK_P_RESULT and/or SUM_CHECK_Q_RESULT are set
// @spare: temporary result buffer for the synchronous case
// @s_off: spare buffer page offset
// @submit: submission / completion modifiers
//
// The same notes from async_gen_syndrome apply to the 'blocks',
// and 'disks' parameters of this routine.  The synchronous path
// requires a temporary result buffer and submit->scribble to be
// specified.
//
    struct dma_async_tx_descriptor *
    async_syndrome_val(struct page **blocks, unsigned int *offsets, int disks,
    size_t len, enum sum_check_flags *pqres, struct page *spare,
    unsigned int s_off, struct async_submit_ctl *submit)
    {
    struct dma_chan *chan = pq_val_chan(submit, blocks, disks, len);
    struct dma_device *device = chan ? chan.device : core::ptr::null_mut();
    struct dma_async_tx_descriptor *tx;
    unsigned char coefs[MAX_DISKS];
    let mut dma_flags: enum dma_ctrl_flags = submit.cb_fn ? DMA_PREP_INTERRUPT : 0;
    struct dmaengine_unmap_data *unmap = core::ptr::null_mut();
    BUG_ON(disks < 4 || disks > MAX_DISKS);
    if (device)
    unmap = dmaengine_get_unmap_data(device.dev, disks, GFP_NOWAIT);
    if (unmap && disks <= dma_maxpq(device, 0) &&
    is_dma_pq_aligned_offs(device, offsets, disks, len)) {
    struct device *dev = device.dev;
    dma_addr_t pq[2];
    int i, j = 0, src_cnt = 0;
    pr_debug("%s: (async) disks: %d len: %zu\n",
    __func__, disks, len);
    unmap.len = len;
    for (i = 0; i < disks-2; i++)
    if (likely(blocks[i])) {
    unmap.addr[j] = dma_map_page(dev, blocks[i],
    offsets[i], len,
    DMA_TO_DEVICE);
    coefs[j] = raid6_gfexp[i];
    unmap.to_cnt++;
    src_cnt++;
    j++;
    }
    if (!P(blocks, disks)) {
    pq[0] = 0;
    dma_flags |= DMA_PREP_PQ_DISABLE_P;
    } else {
    pq[0] = dma_map_page(dev, P(blocks, disks),
    P(offsets, disks), len,
    DMA_TO_DEVICE);
    unmap.addr[j++] = pq[0];
    unmap.to_cnt++;
    }
    if (!Q(blocks, disks)) {
    pq[1] = 0;
    dma_flags |= DMA_PREP_PQ_DISABLE_Q;
    } else {
    pq[1] = dma_map_page(dev, Q(blocks, disks),
    Q(offsets, disks), len,
    DMA_TO_DEVICE);
    unmap.addr[j++] = pq[1];
    unmap.to_cnt++;
    }
    if (submit.flags & ASYNC_TX_FENCE)
    dma_flags |= DMA_PREP_FENCE;
    for (;;) {
    tx = device.device_prep_dma_pq_val(chan, pq,
    unmap.addr,
    src_cnt,
    coefs,
    len, pqres,
    dma_flags);
    if (likely(tx))
    break;
    async_tx_quiesce(&submit.depend_tx);
    dma_async_issue_pending(chan);
    }
    dma_set_unmap(tx, unmap);
    async_tx_submit(chan, tx, submit);
    } else {
    struct page *p_src = P(blocks, disks);
    let mut p_off: c_uint = P(offsets, disks);
    struct page *q_src = Q(blocks, disks);
    let mut q_off: c_uint = Q(offsets, disks);
    let mut flags_orig: enum async_tx_flags = submit.flags;
    let mut cb_fn_orig: dma_async_tx_callback = submit.cb_fn;
    void *scribble = submit.scribble;
    void *cb_param_orig = submit.cb_param;
    void *p, *q, *s;
    pr_debug("%s: (sync) disks: %d len: %zu\n",
    __func__, disks, len);
// caller must provide a temporary result buffer and
// allow the input parameters to be preserved
//
    BUG_ON(!spare || !scribble);
// wait for any prerequisite operations
    async_tx_quiesce(&submit.depend_tx);
// recompute p and/or q into the temporary buffer and then
// check to see the result matches the current value
//
    tx = core::ptr::null_mut();
// pqres = 0;
    if (p_src) {
    init_async_submit(submit, ASYNC_TX_XOR_ZERO_DST, core::ptr::null_mut(),
    core::ptr::null_mut(), core::ptr::null_mut(), scribble);
    tx = async_xor_offs(spare, s_off,
    blocks, offsets, disks-2, len, submit);
    async_tx_quiesce(&tx);
    p = page_address(p_src) + p_off;
    s = page_address(spare) + s_off;
// pqres |= !!memcmp(p, s, len) << SUM_CHECK_P;
    }
    if (q_src) {
    P(blocks, disks) = core::ptr::null_mut();
    Q(blocks, disks) = spare;
    Q(offsets, disks) = s_off;
    init_async_submit(submit, 0, core::ptr::null_mut(), core::ptr::null_mut(), core::ptr::null_mut(), scribble);
    tx = async_gen_syndrome(blocks, offsets, disks,
    len, submit);
    async_tx_quiesce(&tx);
    q = page_address(q_src) + q_off;
    s = page_address(spare) + s_off;
// pqres |= !!memcmp(q, s, len) << SUM_CHECK_Q;
    }
// restore P, Q and submit
    P(blocks, disks) = p_src;
    P(offsets, disks) = p_off;
    Q(blocks, disks) = q_src;
    Q(offsets, disks) = q_off;
    submit.cb_fn = cb_fn_orig;
    submit.cb_param = cb_param_orig;
    submit.flags = flags_orig;
    async_tx_sync_epilog(submit);
    tx = core::ptr::null_mut();
    }
    dmaengine_unmap_put(unmap);
    return tx;
    }
    EXPORT_SYMBOL_GPL(async_syndrome_val);
#[no_mangle]
unsafe extern "C" fn async_pq_init() -> int __init {
    static int __init async_pq_init(void)
    {
    pq_scribble_page = alloc_page(GFP_KERNEL);
    if (pq_scribble_page)
    return 0;
    pr_err("%s: failed to allocate required spare page\n", __func__);
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn async_pq_exit() -> void __exit {
    static void __exit async_pq_exit(void)
    {
    __free_page(pq_scribble_page);
    }
    module_init(async_pq_init);
    module_exit(async_pq_exit);
    MODULE_DESCRIPTION("asynchronous raid6 syndrome generation/validation");
    MODULE_LICENSE("GPL");
