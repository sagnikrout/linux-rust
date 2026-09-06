//! Automatically rewritten from C to Rust
//! Source: drivers/iio/buffer/industrialio-hw-consumer.c
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright 2017 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

//
// struct iio_hw_consumer - IIO hw consumer block
// @buffers: hardware buffers list head.
// @channels: IIO provider channels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_hw_consumer {
    pub buffers: list_head,
    pub channels: *mut iio_channel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_consumer_buffer {
    pub head: list_head,
    pub indio_dev: *mut iio_dev,
    pub buffer: iio_buffer,
}

    static struct hw_consumer_buffer *iio_buffer_to_hw_consumer_buffer(
    struct iio_buffer *buffer)
    {
    return container_of(buffer, struct hw_consumer_buffer, buffer);
    }
#[no_mangle]
unsafe extern "C" fn iio_hw_buf_release(buffer: *mut iio_buffer) {
    static void iio_hw_buf_release(struct iio_buffer *buffer)
    {
    struct hw_consumer_buffer *hw_buf =
    iio_buffer_to_hw_consumer_buffer(buffer);
    bitmap_free(buffer.scan_mask);
    kfree(hw_buf);
    }
    static const struct iio_buffer_access_funcs iio_hw_buf_access = {
    .release = &iio_hw_buf_release,
    .modes = INDIO_BUFFER_HARDWARE,
    };
    static struct hw_consumer_buffer *iio_hw_consumer_get_buffer(
    struct iio_hw_consumer *hwc, struct iio_dev *indio_dev)
    {
    struct hw_consumer_buffer *buf;
    list_for_each_entry(buf, &hwc.buffers, head) {
    if (buf.indio_dev == indio_dev)
    return buf;
    }
    buf = kzalloc_obj(*buf);
    if (!buf)
    return core::ptr::null_mut();
    buf.buffer.access = &iio_hw_buf_access;
    buf.indio_dev = indio_dev;
    buf.buffer.scan_mask = bitmap_zalloc(iio_get_masklength(indio_dev),
    GFP_KERNEL);
    if (!buf.buffer.scan_mask) {
    kfree(buf);
    return core::ptr::null_mut();
    }
    iio_buffer_init(&buf.buffer);
    list_add_tail(&buf.head, &hwc.buffers);
    return buf;
    }
//
// iio_hw_consumer_alloc() - Allocate IIO hardware consumer
// @dev: Pointer to consumer device.
//
// Returns a valid iio_hw_consumer on success or a ERR_PTR() on failure.
//
    struct iio_hw_consumer *iio_hw_consumer_alloc(struct device *dev)
    {
    struct hw_consumer_buffer *buf, *tmp;
    struct iio_hw_consumer *hwc;
    struct iio_channel *chan;
    int ret;
    hwc = kzalloc_obj(*hwc);
    if (!hwc)
    return ERR_PTR(-ENOMEM);
    INIT_LIST_HEAD(&hwc.buffers);
    hwc.channels = iio_channel_get_all(dev);
    if (IS_ERR(hwc.channels)) {
    ret = PTR_ERR(hwc.channels);
    goto err_free_hwc;
    }
    chan = &hwc.channels[0];
    while (chan.indio_dev) {
    buf = iio_hw_consumer_get_buffer(hwc, chan.indio_dev);
    if (!buf) {
    ret = -ENOMEM;
    goto err_put_buffers;
    }
    set_bit(chan.channel.scan_index, buf.buffer.scan_mask);
    chan++;
    }
    return hwc;
    err_put_buffers:
    list_for_each_entry_safe(buf, tmp, &hwc.buffers, head)
    iio_buffer_put(&buf.buffer);
    iio_channel_release_all(hwc.channels);
    err_free_hwc:
    kfree(hwc);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(iio_hw_consumer_alloc);
//
// iio_hw_consumer_free() - Free IIO hardware consumer
// @hwc: hw consumer to free.
//
#[no_mangle]
pub unsafe extern "C" fn iio_hw_consumer_free(hwc: *mut iio_hw_consumer) {
    void iio_hw_consumer_free(struct iio_hw_consumer *hwc)
    {
    struct hw_consumer_buffer *buf, *n;
    iio_channel_release_all(hwc.channels);
    list_for_each_entry_safe(buf, n, &hwc.buffers, head)
    iio_buffer_put(&buf.buffer);
    kfree(hwc);
    }
    EXPORT_SYMBOL_GPL(iio_hw_consumer_free);
#[no_mangle]
unsafe extern "C" fn devm_iio_hw_consumer_release(iio_hwc: *mut c_void) {
    static void devm_iio_hw_consumer_release(void *iio_hwc)
    {
    iio_hw_consumer_free(iio_hwc);
    }
//
// devm_iio_hw_consumer_alloc - Resource-managed iio_hw_consumer_alloc()
// @dev: Pointer to consumer device.
//
// Managed iio_hw_consumer_alloc. iio_hw_consumer allocated with this function
// is automatically freed on driver detach.
//
// returns pointer to allocated iio_hw_consumer on success, NULL on failure.
//
    struct iio_hw_consumer *devm_iio_hw_consumer_alloc(struct device *dev)
    {
    struct iio_hw_consumer *iio_hwc;
    int ret;
    iio_hwc = iio_hw_consumer_alloc(dev);
    if (IS_ERR(iio_hwc))
    return iio_hwc;
    ret = devm_add_action_or_reset(dev, devm_iio_hw_consumer_release,
    iio_hwc);
    if (ret)
    return ERR_PTR(ret);
    return iio_hwc;
    }
    EXPORT_SYMBOL_GPL(devm_iio_hw_consumer_alloc);
//
// iio_hw_consumer_enable() - Enable IIO hardware consumer
// @hwc: iio_hw_consumer to enable.
//
// Returns 0 on success.
//
#[no_mangle]
pub unsafe extern "C" fn iio_hw_consumer_enable(hwc: *mut iio_hw_consumer) -> c_int {
    int iio_hw_consumer_enable(struct iio_hw_consumer *hwc)
    {
    struct hw_consumer_buffer *buf;
    int ret;
    list_for_each_entry(buf, &hwc.buffers, head) {
    ret = iio_update_buffers(buf.indio_dev, &buf.buffer, core::ptr::null_mut());
    if (ret)
    goto err_disable_buffers;
    }
    return 0;
    err_disable_buffers:
    list_for_each_entry_continue_reverse(buf, &hwc.buffers, head)
    iio_update_buffers(buf.indio_dev, core::ptr::null_mut(), &buf.buffer);
    return ret;
    }
    EXPORT_SYMBOL_GPL(iio_hw_consumer_enable);
//
// iio_hw_consumer_disable() - Disable IIO hardware consumer
// @hwc: iio_hw_consumer to disable.
//
#[no_mangle]
pub unsafe extern "C" fn iio_hw_consumer_disable(hwc: *mut iio_hw_consumer) {
    void iio_hw_consumer_disable(struct iio_hw_consumer *hwc)
    {
    struct hw_consumer_buffer *buf;
    list_for_each_entry(buf, &hwc.buffers, head)
    iio_update_buffers(buf.indio_dev, core::ptr::null_mut(), &buf.buffer);
    }
    EXPORT_SYMBOL_GPL(iio_hw_consumer_disable);
    MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
    MODULE_DESCRIPTION("Hardware consumer buffer the IIO framework");
    MODULE_LICENSE("GPL v2");
    MODULE_IMPORT_NS("IIO_CONSUMER");
