//! Automatically rewritten from C to Rust
//! Source: drivers/virtio/virtio_dma_buf.c
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
// dma-bufs for virtio exported objects
//
// Copyright (C) 2020 Google, Inc.
//

//
// virtio_dma_buf_export - Creates a new dma-buf for a virtio exported object
// @exp_info: [in] see dma_buf_export(). ops MUST refer to a dma_buf_ops
// struct embedded in a virtio_dma_buf_ops.
//
// This wraps dma_buf_export() to allow virtio drivers to create a dma-buf
// for a virtio exported object that can be queried by other virtio drivers
// for the object's UUID.
//
    struct dma_buf *virtio_dma_buf_export
    (const struct dma_buf_export_info *exp_info)
    {
    const struct virtio_dma_buf_ops *virtio_ops =
    container_of(exp_info.ops,
    const struct virtio_dma_buf_ops, ops);
    if (!exp_info.ops ||
    exp_info.ops.attach != &virtio_dma_buf_attach ||
    !virtio_ops.get_uuid) {
    return ERR_PTR(-EINVAL);
    }
    return dma_buf_export(exp_info);
    }
    EXPORT_SYMBOL(virtio_dma_buf_export);
//
// virtio_dma_buf_attach - mandatory attach callback for virtio dma-bufs
// @dma_buf: [in] buffer to attach
// @attach: [in] attachment structure
//
    int virtio_dma_buf_attach(struct dma_buf *dma_buf,
    struct dma_buf_attachment *attach)
    {
    int ret;
    const struct virtio_dma_buf_ops *ops =
    container_of(dma_buf.ops,
    const struct virtio_dma_buf_ops, ops);
    if (ops.device_attach) {
    ret = ops.device_attach(dma_buf, attach);
    if (ret)
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL(virtio_dma_buf_attach);
//
// is_virtio_dma_buf - returns true if the given dma-buf is a virtio dma-buf
// @dma_buf: buffer to query
//
#[no_mangle]
pub unsafe extern "C" fn is_virtio_dma_buf(dma_buf: *mut dma_buf) -> bool {
    bool is_virtio_dma_buf(struct dma_buf *dma_buf)
    {
    return dma_buf.ops.attach == &virtio_dma_buf_attach;
    }
    EXPORT_SYMBOL(is_virtio_dma_buf);
//
// virtio_dma_buf_get_uuid - gets a virtio dma-buf's exported object's uuid
// @dma_buf: [in] buffer to query
// @uuid: [out] the uuid
//
// Returns: 0 on success, negative on failure.
//
    int virtio_dma_buf_get_uuid(struct dma_buf *dma_buf,
    uuid_t *uuid)
    {
    const struct virtio_dma_buf_ops *ops =
    container_of(dma_buf.ops,
    const struct virtio_dma_buf_ops, ops);
    if (!is_virtio_dma_buf(dma_buf))
    return -EINVAL;
    return ops.get_uuid(dma_buf, uuid);
    }
    EXPORT_SYMBOL(virtio_dma_buf_get_uuid);
    MODULE_DESCRIPTION("dma-bufs for virtio exported objects");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("DMA_BUF");
