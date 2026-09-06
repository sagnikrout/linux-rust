//! Automatically rewritten from C to Rust
//! Source: crypto/async_tx/async_memcpy.c
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
// copy offload engine support
//
// Copyright © 2006, Intel Corporation.
//
// Dan Williams <dan.j.williams@intel.com>
//
// with architecture considerations by:
// Neil Brown <neilb@suse.de>
// Jeff Garzik <jeff@garzik.org>
//

//
// async_memcpy - attempt to copy memory with a dma engine.
// @dest: destination page
// @src: src page
// @dest_offset: offset into 'dest' to start transaction
// @src_offset: offset into 'src' to start transaction
// @len: length in bytes
// @submit: submission / completion modifiers
//
// honored flags: ASYNC_TX_ACK
//
    struct dma_async_tx_descriptor *
    async_memcpy(struct page *dest, struct page *src, unsigned int dest_offset,
    unsigned int src_offset, size_t len,
    struct async_submit_ctl *submit)
    {
    struct dma_chan *chan = async_tx_find_channel(submit, DMA_MEMCPY,
    &dest, 1, &src, 1, len);
    struct dma_device *device = chan ? chan.device : core::ptr::null_mut();
    struct dma_async_tx_descriptor *tx = core::ptr::null_mut();
    struct dmaengine_unmap_data *unmap = core::ptr::null_mut();
    if (device)
    unmap = dmaengine_get_unmap_data(device.dev, 2, GFP_NOWAIT);
    if (unmap && is_dma_copy_aligned(device, src_offset, dest_offset, len)) {
    let mut dma_prep_flags: c_ulong = 0;
    if (submit.cb_fn)
    dma_prep_flags |= DMA_PREP_INTERRUPT;
    if (submit.flags & ASYNC_TX_FENCE)
    dma_prep_flags |= DMA_PREP_FENCE;
    unmap.to_cnt = 1;
    unmap.addr[0] = dma_map_page(device.dev, src, src_offset, len,
    DMA_TO_DEVICE);
    unmap.from_cnt = 1;
    unmap.addr[1] = dma_map_page(device.dev, dest, dest_offset, len,
    DMA_FROM_DEVICE);
    unmap.len = len;
    tx = device.device_prep_dma_memcpy(chan, unmap.addr[1],
    unmap.addr[0], len,
    dma_prep_flags);
    }
    if (tx) {
    pr_debug("%s: (async) len: %zu\n", __func__, len);
    dma_set_unmap(tx, unmap);
    async_tx_submit(chan, tx, submit);
    } else {
    void *dest_buf, *src_buf;
    pr_debug("%s: (sync) len: %zu\n", __func__, len);
// wait for any prerequisite operations
    async_tx_quiesce(&submit.depend_tx);
    dest_buf = kmap_atomic(dest) + dest_offset;
    src_buf = kmap_atomic(src) + src_offset;
    memcpy(dest_buf, src_buf, len);
    kunmap_atomic(src_buf);
    kunmap_atomic(dest_buf);
    async_tx_sync_epilog(submit);
    }
    dmaengine_unmap_put(unmap);
    return tx;
    }
    EXPORT_SYMBOL_GPL(async_memcpy);
    MODULE_AUTHOR("Intel Corporation");
    MODULE_DESCRIPTION("asynchronous memcpy api");
    MODULE_LICENSE("GPL");
