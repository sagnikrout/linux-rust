//! Automatically rewritten from C to Rust
//! Source: drivers/iio/buffer/industrialio-buffer-cb.c
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
// The industrial I/O callback buffer
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_cb_buffer {
    pub buffer: iio_buffer,
// Must be safe to call from any context (e.g. must not sleep).
    pub private): *const *const *const int (cb)(void data, void,
    pub private: *mut c_void,
    pub channels: *mut iio_channel,
    pub indio_dev: *mut iio_dev,
}

    static struct iio_cb_buffer *buffer_to_cb_buffer(struct iio_buffer *buffer)
    {
    return container_of(buffer, struct iio_cb_buffer, buffer);
    }
#[no_mangle]
unsafe extern "C" fn iio_buffer_cb_store_to(buffer: *mut iio_buffer, data: *const c_void) -> c_int {
    static int iio_buffer_cb_store_to(struct iio_buffer *buffer, const void *data)
    {
    struct iio_cb_buffer *cb_buff = buffer_to_cb_buffer(buffer);
    return cb_buff.cb(data, cb_buff.private);
    }
#[no_mangle]
unsafe extern "C" fn iio_buffer_cb_release(buffer: *mut iio_buffer) {
    static void iio_buffer_cb_release(struct iio_buffer *buffer)
    {
    struct iio_cb_buffer *cb_buff = buffer_to_cb_buffer(buffer);
    bitmap_free(cb_buff.buffer.scan_mask);
    kfree(cb_buff);
    }
    static const struct iio_buffer_access_funcs iio_cb_access = {
    .store_to = &iio_buffer_cb_store_to,
    .release = &iio_buffer_cb_release,
    .modes = INDIO_BUFFER_SOFTWARE | INDIO_BUFFER_TRIGGERED,
    };
    struct iio_cb_buffer *iio_channel_get_all_cb(struct device *dev,
    int (*cb)(const void *data,
    void *private),
    void *private)
    {
    int ret;
    struct iio_cb_buffer *cb_buff;
    struct iio_channel *chan;
    if (!cb) {
    dev_err(dev, "Invalid arguments: A callback must be provided!\n");
    return ERR_PTR(-EINVAL);
    }
    cb_buff = kzalloc_obj(*cb_buff);
    if (cb_buff == core::ptr::null_mut())
    return ERR_PTR(-ENOMEM);
    iio_buffer_init(&cb_buff.buffer);
    cb_buff.private = private;
    cb_buff.cb = cb;
    cb_buff.buffer.access = &iio_cb_access;
    cb_buff.channels = iio_channel_get_all(dev);
    if (IS_ERR(cb_buff.channels)) {
    ret = PTR_ERR(cb_buff.channels);
    goto error_free_cb_buff;
    }
    cb_buff.indio_dev = cb_buff.channels[0].indio_dev;
    cb_buff.buffer.scan_mask = bitmap_zalloc(iio_get_masklength(cb_buff.indio_dev),
    GFP_KERNEL);
    if (cb_buff.buffer.scan_mask == core::ptr::null_mut()) {
    ret = -ENOMEM;
    goto error_release_channels;
    }
    chan = &cb_buff.channels[0];
    while (chan.indio_dev) {
    if (chan.indio_dev != cb_buff.indio_dev) {
    ret = -EINVAL;
    goto error_free_scan_mask;
    }
    set_bit(chan.channel.scan_index,
    cb_buff.buffer.scan_mask);
    chan++;
    }
    return cb_buff;
    error_free_scan_mask:
    bitmap_free(cb_buff.buffer.scan_mask);
    error_release_channels:
    iio_channel_release_all(cb_buff.channels);
    error_free_cb_buff:
    kfree(cb_buff);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL_GPL(iio_channel_get_all_cb);
    int iio_channel_cb_set_buffer_watermark(struct iio_cb_buffer *cb_buff,
    size_t watermark)
    {
    if (!watermark)
    return -EINVAL;
    cb_buff.buffer.watermark = watermark;
    return 0;
    }
    EXPORT_SYMBOL_GPL(iio_channel_cb_set_buffer_watermark);
#[no_mangle]
pub unsafe extern "C" fn iio_channel_start_all_cb(cb_buff: *mut iio_cb_buffer) -> c_int {
    int iio_channel_start_all_cb(struct iio_cb_buffer *cb_buff)
    {
    return iio_update_buffers(cb_buff.indio_dev, &cb_buff.buffer,
    core::ptr::null_mut());
    }
    EXPORT_SYMBOL_GPL(iio_channel_start_all_cb);
#[no_mangle]
pub unsafe extern "C" fn iio_channel_stop_all_cb(cb_buff: *mut iio_cb_buffer) {
    void iio_channel_stop_all_cb(struct iio_cb_buffer *cb_buff)
    {
    iio_update_buffers(cb_buff.indio_dev, core::ptr::null_mut(), &cb_buff.buffer);
    }
    EXPORT_SYMBOL_GPL(iio_channel_stop_all_cb);
#[no_mangle]
pub unsafe extern "C" fn iio_channel_release_all_cb(cb_buff: *mut iio_cb_buffer) {
    void iio_channel_release_all_cb(struct iio_cb_buffer *cb_buff)
    {
    iio_channel_release_all(cb_buff.channels);
    iio_buffer_put(&cb_buff.buffer);
    }
    EXPORT_SYMBOL_GPL(iio_channel_release_all_cb);
    struct iio_channel
// iio_channel_cb_get_channels(const struct iio_cb_buffer *cb_buffer)
    {
    return cb_buffer.channels;
    }
    EXPORT_SYMBOL_GPL(iio_channel_cb_get_channels);
    struct iio_dev
// iio_channel_cb_get_iio_dev(const struct iio_cb_buffer *cb_buffer)
    {
    return cb_buffer.indio_dev;
    }
    EXPORT_SYMBOL_GPL(iio_channel_cb_get_iio_dev);
    MODULE_AUTHOR("Jonathan Cameron <jic23@kernel.org>");
    MODULE_DESCRIPTION("Industrial I/O callback buffer");
    MODULE_LICENSE("GPL");
    MODULE_IMPORT_NS("IIO_CONSUMER");
