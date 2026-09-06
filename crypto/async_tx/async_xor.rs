//! Automatically rewritten from C to Rust
//! Source: crypto/async_tx/async_xor.c
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
// xor offload engine api
//
// Copyright © 2006, Intel Corporation.
//
// Dan Williams <dan.j.williams@intel.com>
//
// with architecture considerations by:
// Neil Brown <neilb@suse.de>
// Jeff Garzik <jeff@garzik.org>
//

// do_async_xor - dma map the pages and perform the xor with an engine
    static __async_inline struct dma_async_tx_descriptor *
    do_async_xor(struct dma_chan *chan, struct dmaengine_unmap_data *unmap,
    struct async_submit_ctl *submit)
    {
    struct dma_device *dma = chan.device;
    struct dma_async_tx_descriptor *tx = core::ptr::null_mut();
    let mut cb_fn_orig: dma_async_tx_callback = submit.cb_fn;
    void *cb_param_orig = submit.cb_param;
    let mut flags_orig: enum async_tx_flags = submit.flags;
    let mut dma_flags: enum dma_ctrl_flags = 0;
    let mut src_cnt: c_int = unmap.to_cnt;
    int xor_src_cnt;
    let mut dma_dest: dma_addr_t = unmap.addr[unmap.to_cnt];
    dma_addr_t *src_list = unmap.addr;
    while (src_cnt) {
    dma_addr_t tmp;
    submit.flags = flags_orig;
    xor_src_cnt = min(src_cnt, (int)dma.max_xor);
// if we are submitting additional xors, leave the chain open
// and clear the callback parameters
//
    if (src_cnt > xor_src_cnt) {
    submit.flags &= ~ASYNC_TX_ACK;
    submit.flags |= ASYNC_TX_FENCE;
    submit.cb_fn = core::ptr::null_mut();
    submit.cb_param = core::ptr::null_mut();
    } else {
    submit.cb_fn = cb_fn_orig;
    submit.cb_param = cb_param_orig;
    }
    if (submit.cb_fn)
    dma_flags |= DMA_PREP_INTERRUPT;
    if (submit.flags & ASYNC_TX_FENCE)
    dma_flags |= DMA_PREP_FENCE;
// Drivers force forward progress in case they can not provide a
// descriptor
//
    tmp = src_list[0];
    if (src_list > unmap.addr)
    src_list[0] = dma_dest;
    tx = dma.device_prep_dma_xor(chan, dma_dest, src_list,
    xor_src_cnt, unmap.len,
    dma_flags);
    if (unlikely(!tx))
    async_tx_quiesce(&submit.depend_tx);
// spin wait for the preceding transactions to complete
    while (unlikely(!tx)) {
    dma_async_issue_pending(chan);
    tx = dma.device_prep_dma_xor(chan, dma_dest,
    src_list,
    xor_src_cnt, unmap.len,
    dma_flags);
    }
    src_list[0] = tmp;
    dma_set_unmap(tx, unmap);
    async_tx_submit(chan, tx, submit);
    submit.depend_tx = tx;
    if (src_cnt > xor_src_cnt) {
// drop completed sources
    src_cnt -= xor_src_cnt;
// use the intermediate result a source
    src_cnt++;
    src_list += xor_src_cnt - 1;
    } else
    break;
    }
    return tx;
    }
    static void
    do_sync_xor_offs(struct page *dest, unsigned int offset,
    struct page **src_list, unsigned int *src_offs,
    int src_cnt, size_t len, struct async_submit_ctl *submit)
    {
    int i;
    let mut xor_src_cnt: c_int = 0;
    void *dest_buf;
    void **srcs;
    if (submit.scribble)
    srcs = submit.scribble;
    else
    srcs = (void **) src_list;
// convert to buffer pointers
    for (i = 0; i < src_cnt; i++)
    if (src_list[i])
    srcs[xor_src_cnt++] = page_address(src_list[i]) +
    (src_offs ? src_offs[i] : offset);
// set destination address
    dest_buf = page_address(dest) + offset;
    if (submit.flags & ASYNC_TX_XOR_ZERO_DST)
    memset(dest_buf, 0, len);
    xor_gen(dest_buf, srcs, xor_src_cnt, len);
    async_tx_sync_epilog(submit);
    }
    static inline bool
    dma_xor_aligned_offsets(struct dma_device *device, unsigned int offset,
    unsigned int *src_offs, int src_cnt, int len)
    {
    int i;
    if (!is_dma_xor_aligned(device, offset, 0, len))
    return false;
    if (!src_offs)
    return true;
    for (i = 0; i < src_cnt; i++) {
    if (!is_dma_xor_aligned(device, src_offs[i], 0, len))
    return false;
    }
    return true;
    }
//
// async_xor_offs - attempt to xor a set of blocks with a dma engine.
// @dest: destination page
// @offset: dst offset to start transaction
// @src_list: array of source pages
// @src_offs: array of source pages offset, NULL means common src/dst offset
// @src_cnt: number of source pages
// @len: length in bytes
// @submit: submission / completion modifiers
//
// honored flags: ASYNC_TX_ACK, ASYNC_TX_XOR_ZERO_DST, ASYNC_TX_XOR_DROP_DST
//
// xor_gen always uses the dest as a source so the ASYNC_TX_XOR_ZERO_DST flag
// must be set to not include dest data in the calculation.  The assumption with
// dma engines is that they only use the destination buffer as a source when it
// is explicitly specified in the source list.
//
// src_list note: if the dest is also a source it must be at index zero.
// The contents of this array will be overwritten if a scribble region
// is not specified.
//
    struct dma_async_tx_descriptor *
    async_xor_offs(struct page *dest, unsigned int offset,
    struct page **src_list, unsigned int *src_offs,
    int src_cnt, size_t len, struct async_submit_ctl *submit)
    {
    struct dma_chan *chan = async_tx_find_channel(submit, DMA_XOR,
    &dest, 1, src_list,
    src_cnt, len);
    struct dma_device *device = chan ? chan.device : core::ptr::null_mut();
    struct dmaengine_unmap_data *unmap = core::ptr::null_mut();
    BUG_ON(src_cnt <= 1);
    if (device)
    unmap = dmaengine_get_unmap_data(device.dev, src_cnt+1, GFP_NOWAIT);
    if (unmap && dma_xor_aligned_offsets(device, offset,
    src_offs, src_cnt, len)) {
    struct dma_async_tx_descriptor *tx;
    int i, j;
// run the xor asynchronously
    pr_debug("%s (async): len: %zu\n", __func__, len);
    unmap.len = len;
    for (i = 0, j = 0; i < src_cnt; i++) {
    if (!src_list[i])
    continue;
    unmap.to_cnt++;
    unmap.addr[j++] = dma_map_page(device.dev, src_list[i],
    src_offs ? src_offs[i] : offset,
    len, DMA_TO_DEVICE);
    }
// map it bidirectional as it may be re-used as a source
    unmap.addr[j] = dma_map_page(device.dev, dest, offset, len,
    DMA_BIDIRECTIONAL);
    unmap.bidi_cnt = 1;
    tx = do_async_xor(chan, unmap, submit);
    dmaengine_unmap_put(unmap);
    return tx;
    } else {
    dmaengine_unmap_put(unmap);
// run the xor synchronously
    pr_debug("%s (sync): len: %zu\n", __func__, len);
    WARN_ONCE(chan, "%s: no space for dma address conversion\n",
    __func__);
// in the sync case the dest is an implied source
// (assumes the dest is the first source)
//
    if (submit.flags & ASYNC_TX_XOR_DROP_DST) {
    src_cnt--;
    src_list++;
    if (src_offs)
    src_offs++;
    }
// wait for any prerequisite operations
    async_tx_quiesce(&submit.depend_tx);
    do_sync_xor_offs(dest, offset, src_list, src_offs,
    src_cnt, len, submit);
    return core::ptr::null_mut();
    }
    }
    EXPORT_SYMBOL_GPL(async_xor_offs);
//
// async_xor - attempt to xor a set of blocks with a dma engine.
// @dest: destination page
// @src_list: array of source pages
// @offset: common src/dst offset to start transaction
// @src_cnt: number of source pages
// @len: length in bytes
// @submit: submission / completion modifiers
//
// honored flags: ASYNC_TX_ACK, ASYNC_TX_XOR_ZERO_DST, ASYNC_TX_XOR_DROP_DST
//
// xor_gen always uses the dest as a source so the ASYNC_TX_XOR_ZERO_DST flag
// must be set to not include dest data in the calculation.  The assumption with
// dma engines is that they only use the destination buffer as a source when it
// is explicitly specified in the source list.
//
// src_list note: if the dest is also a source it must be at index zero.
// The contents of this array will be overwritten if a scribble region
// is not specified.
//
    struct dma_async_tx_descriptor *
    async_xor(struct page *dest, struct page **src_list, unsigned int offset,
    int src_cnt, size_t len, struct async_submit_ctl *submit)
    {
    return async_xor_offs(dest, offset, src_list, core::ptr::null_mut(),
    src_cnt, len, submit);
    }
    EXPORT_SYMBOL_GPL(async_xor);
#[no_mangle]
unsafe extern "C" fn page_is_zero(p: *mut page, offset: c_uint, len: usize) -> c_int {
    static int page_is_zero(struct page *p, unsigned int offset, size_t len)
    {
    return !memchr_inv(page_address(p) + offset, 0, len);
    }
    static inline struct dma_chan *
    xor_val_chan(struct async_submit_ctl *submit, struct page *dest,
    struct page **src_list, int src_cnt, size_t len)
    {

    return core::ptr::null_mut();

    return async_tx_find_channel(submit, DMA_XOR_VAL, &dest, 1, src_list,
    src_cnt, len);
    }
//
// async_xor_val_offs - attempt a xor parity check with a dma engine.
// @dest: destination page used if the xor is performed synchronously
// @offset: des offset in pages to start transaction
// @src_list: array of source pages
// @src_offs: array of source pages offset, NULL means common src/det offset
// @src_cnt: number of source pages
// @len: length in bytes
// @result: 0 if sum == 0 else non-zero
// @submit: submission / completion modifiers
//
// honored flags: ASYNC_TX_ACK
//
// src_list note: if the dest is also a source it must be at index zero.
// The contents of this array will be overwritten if a scribble region
// is not specified.
//
    struct dma_async_tx_descriptor *
    async_xor_val_offs(struct page *dest, unsigned int offset,
    struct page **src_list, unsigned int *src_offs,
    int src_cnt, size_t len, enum sum_check_flags *result,
    struct async_submit_ctl *submit)
    {
    struct dma_chan *chan = xor_val_chan(submit, dest, src_list, src_cnt, len);
    struct dma_device *device = chan ? chan.device : core::ptr::null_mut();
    struct dma_async_tx_descriptor *tx = core::ptr::null_mut();
    struct dmaengine_unmap_data *unmap = core::ptr::null_mut();
    BUG_ON(src_cnt <= 1);
    if (device)
    unmap = dmaengine_get_unmap_data(device.dev, src_cnt, GFP_NOWAIT);
    if (unmap && src_cnt <= device.max_xor &&
    dma_xor_aligned_offsets(device, offset, src_offs, src_cnt, len)) {
    let mut dma_prep_flags: c_ulong = 0;
    int i;
    pr_debug("%s: (async) len: %zu\n", __func__, len);
    if (submit.cb_fn)
    dma_prep_flags |= DMA_PREP_INTERRUPT;
    if (submit.flags & ASYNC_TX_FENCE)
    dma_prep_flags |= DMA_PREP_FENCE;
    for (i = 0; i < src_cnt; i++) {
    unmap.addr[i] = dma_map_page(device.dev, src_list[i],
    src_offs ? src_offs[i] : offset,
    len, DMA_TO_DEVICE);
    unmap.to_cnt++;
    }
    unmap.len = len;
    tx = device.device_prep_dma_xor_val(chan, unmap.addr, src_cnt,
    len, result,
    dma_prep_flags);
    if (unlikely(!tx)) {
    async_tx_quiesce(&submit.depend_tx);
    while (!tx) {
    dma_async_issue_pending(chan);
    tx = device.device_prep_dma_xor_val(chan,
    unmap.addr, src_cnt, len, result,
    dma_prep_flags);
    }
    }
    dma_set_unmap(tx, unmap);
    async_tx_submit(chan, tx, submit);
    } else {
    let mut flags_orig: enum async_tx_flags = submit.flags;
    pr_debug("%s: (sync) len: %zu\n", __func__, len);
    WARN_ONCE(device && src_cnt <= device.max_xor,
    "%s: no space for dma address conversion\n",
    __func__);
    submit.flags |= ASYNC_TX_XOR_DROP_DST;
    submit.flags &= ~ASYNC_TX_ACK;
    tx = async_xor_offs(dest, offset, src_list, src_offs,
    src_cnt, len, submit);
    async_tx_quiesce(&tx);
// result = !page_is_zero(dest, offset, len) << SUM_CHECK_P;
    async_tx_sync_epilog(submit);
    submit.flags = flags_orig;
    }
    dmaengine_unmap_put(unmap);
    return tx;
    }
    EXPORT_SYMBOL_GPL(async_xor_val_offs);
    MODULE_AUTHOR("Intel Corporation");
    MODULE_DESCRIPTION("asynchronous xor/xor-zero-sum api");
    MODULE_LICENSE("GPL");
