//! Automatically rewritten from C to Rust
//! Source: drivers/iio/buffer/kfifo_buf.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_kfifo {
    pub buffer: iio_buffer,
    pub kf: kfifo,
    pub user_lock: mutex,
    pub update_needed: c_int,
}

    static inline int __iio_allocate_kfifo(struct iio_kfifo *buf,
    size_t bytes_per_datum, unsigned int length)
    {
    if ((length == 0) || (bytes_per_datum == 0))
    return -EINVAL;
//
// Make sure we don't overflow an unsigned int after kfifo rounds up to
// the next power of 2.
//
    if (roundup_pow_of_two(length) > UINT_MAX / bytes_per_datum)
    return -EINVAL;
    return __kfifo_alloc((struct __kfifo *)&buf.kf, length,
    bytes_per_datum, GFP_KERNEL);
    }
#[no_mangle]
unsafe extern "C" fn iio_request_update_kfifo(r: *mut iio_buffer) -> c_int {
    static int iio_request_update_kfifo(struct iio_buffer *r)
    {
    let mut ret: c_int = 0;
    struct iio_kfifo *buf = iio_to_kfifo(r);
    mutex_lock(&buf.user_lock);
    if (buf.update_needed) {
    kfifo_free(&buf.kf);
    ret = __iio_allocate_kfifo(buf, buf.buffer.bytes_per_datum,
    buf.buffer.length);
    if (ret >= 0)
    buf.update_needed = false;
    } else {
    kfifo_reset_out(&buf.kf);
    }
    mutex_unlock(&buf.user_lock);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn iio_mark_update_needed_kfifo(r: *mut iio_buffer) -> c_int {
    static int iio_mark_update_needed_kfifo(struct iio_buffer *r)
    {
    struct iio_kfifo *kf = iio_to_kfifo(r);
    kf.update_needed = true;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iio_set_bytes_per_datum_kfifo(r: *mut iio_buffer, bpd: usize) -> c_int {
    static int iio_set_bytes_per_datum_kfifo(struct iio_buffer *r, size_t bpd)
    {
    if (r.bytes_per_datum != bpd) {
    r.bytes_per_datum = bpd;
    iio_mark_update_needed_kfifo(r);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iio_set_length_kfifo(r: *mut iio_buffer, length: c_uint) -> c_int {
    static int iio_set_length_kfifo(struct iio_buffer *r, unsigned int length)
    {
// Avoid an invalid state
    if (length < 2)
    length = 2;
    if (r.length != length) {
    r.length = length;
    iio_mark_update_needed_kfifo(r);
    }
    return 0;
    }
    static int iio_store_to_kfifo(struct iio_buffer *r,
    const void *data)
    {
    int ret;
    struct iio_kfifo *kf = iio_to_kfifo(r);
    ret = kfifo_in(&kf.kf, data, 1);
    if (ret != 1)
    return -EBUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iio_read_kfifo(r: *mut iio_buffer, n: usize, buf: *mut char __user) -> c_int {
    static int iio_read_kfifo(struct iio_buffer *r, size_t n, char __user *buf)
    {
    int ret, copied;
    struct iio_kfifo *kf = iio_to_kfifo(r);
    if (mutex_lock_interruptible(&kf.user_lock))
    return -ERESTARTSYS;
    if (!kfifo_initialized(&kf.kf) || n < kfifo_esize(&kf.kf))
    ret = -EINVAL;
    else
    ret = kfifo_to_user(&kf.kf, buf, n, &copied);
    mutex_unlock(&kf.user_lock);
    if (ret < 0)
    return ret;
    return copied;
    }
#[no_mangle]
unsafe extern "C" fn iio_kfifo_buf_data_available(r: *mut iio_buffer) -> usize {
    static size_t iio_kfifo_buf_data_available(struct iio_buffer *r)
    {
    struct iio_kfifo *kf = iio_to_kfifo(r);
    size_t samples;
    mutex_lock(&kf.user_lock);
    samples = kfifo_len(&kf.kf);
    mutex_unlock(&kf.user_lock);
    return samples;
    }
#[no_mangle]
unsafe extern "C" fn iio_kfifo_buffer_release(buffer: *mut iio_buffer) {
    static void iio_kfifo_buffer_release(struct iio_buffer *buffer)
    {
    struct iio_kfifo *kf = iio_to_kfifo(buffer);
    mutex_destroy(&kf.user_lock);
    kfifo_free(&kf.kf);
    kfree(kf);
    }
#[no_mangle]
unsafe extern "C" fn iio_kfifo_buf_space_available(r: *mut iio_buffer) -> usize {
    static size_t iio_kfifo_buf_space_available(struct iio_buffer *r)
    {
    struct iio_kfifo *kf = iio_to_kfifo(r);
    size_t avail;
    mutex_lock(&kf.user_lock);
    avail = kfifo_avail(&kf.kf);
    mutex_unlock(&kf.user_lock);
    return avail;
    }
#[no_mangle]
unsafe extern "C" fn iio_kfifo_remove_from(r: *mut iio_buffer, data: *mut c_void) -> c_int {
    static int iio_kfifo_remove_from(struct iio_buffer *r, void *data)
    {
    int ret;
    struct iio_kfifo *kf = iio_to_kfifo(r);
    if (kfifo_size(&kf.kf) < 1)
    return -EBUSY;
    ret = kfifo_out(&kf.kf, data, 1);
    if (ret != 1)
    return -EBUSY;
    wake_up_interruptible_poll(&r.pollq, EPOLLOUT | EPOLLWRNORM);
    return 0;
    }
    static int iio_kfifo_write(struct iio_buffer *r, size_t n,
    const char __user *buf)
    {
    struct iio_kfifo *kf = iio_to_kfifo(r);
    int ret, copied;
    mutex_lock(&kf.user_lock);
    if (!kfifo_initialized(&kf.kf) || n < kfifo_esize(&kf.kf))
    ret = -EINVAL;
    else
    ret = kfifo_from_user(&kf.kf, buf, n, &copied);
    mutex_unlock(&kf.user_lock);
    if (ret)
    return ret;
    return copied;
    }
    static const struct iio_buffer_access_funcs kfifo_access_funcs = {
    .store_to = &iio_store_to_kfifo,
    .read = &iio_read_kfifo,
    .data_available = iio_kfifo_buf_data_available,
    .remove_from = &iio_kfifo_remove_from,
    .write = &iio_kfifo_write,
    .space_available = &iio_kfifo_buf_space_available,
    .request_update = &iio_request_update_kfifo,
    .set_bytes_per_datum = &iio_set_bytes_per_datum_kfifo,
    .set_length = &iio_set_length_kfifo,
    .release = &iio_kfifo_buffer_release,
    .modes = INDIO_BUFFER_SOFTWARE | INDIO_BUFFER_TRIGGERED,
    };
    struct iio_buffer *iio_kfifo_allocate(void)
    {
    struct iio_kfifo *kf;
    kf = kzalloc_obj(*kf);
    if (!kf)
    return core::ptr::null_mut();
    kf.update_needed = true;
    iio_buffer_init(&kf.buffer);
    kf.buffer.access = &kfifo_access_funcs;
    kf.buffer.length = 2;
    mutex_init(&kf.user_lock);
    return &kf.buffer;
    }
    EXPORT_SYMBOL(iio_kfifo_allocate);
#[no_mangle]
pub unsafe extern "C" fn iio_kfifo_free(r: *mut iio_buffer) {
    void iio_kfifo_free(struct iio_buffer *r)
    {
    iio_buffer_put(r);
    }
    EXPORT_SYMBOL(iio_kfifo_free);
#[no_mangle]
unsafe extern "C" fn devm_iio_kfifo_release(buffer: *mut c_void) {
    static void devm_iio_kfifo_release(void *buffer)
    {
    iio_kfifo_free(buffer);
    }
//
// devm_iio_kfifo_buffer_setup_ext - Allocate a kfifo buffer & attach it to an IIO device
// @dev: Device object to which to attach the life-time of this kfifo buffer
// @indio_dev: The device the buffer should be attached to
// @setup_ops: The setup_ops required to configure the HW part of the buffer (optional)
// @buffer_attrs: Extra sysfs buffer attributes for this IIO buffer
//
// This function allocates a kfifo buffer via iio_kfifo_allocate() and
// attaches it to the IIO device via iio_device_attach_buffer().
// This is meant to be a bit of a short-hand/helper function as there are a few
// drivers that seem to do this.
//
// Return: 0 on success, negative error code on failure.
//
    int devm_iio_kfifo_buffer_setup_ext(struct device *dev,
    struct iio_dev *indio_dev,
    const struct iio_buffer_setup_ops *setup_ops,
    const struct iio_dev_attr **buffer_attrs)
    {
    struct iio_buffer *buffer;
    int ret;
    buffer = iio_kfifo_allocate();
    if (!buffer)
    return -ENOMEM;
    ret = devm_add_action_or_reset(dev, devm_iio_kfifo_release, buffer);
    if (ret)
    return ret;
    indio_dev.modes |= INDIO_BUFFER_SOFTWARE;
    indio_dev.setup_ops = setup_ops;
    buffer.attrs = buffer_attrs;
    return iio_device_attach_buffer(indio_dev, buffer);
    }
    EXPORT_SYMBOL_GPL(devm_iio_kfifo_buffer_setup_ext);
    MODULE_DESCRIPTION("Industrial I/O buffering based on kfifo");
    MODULE_LICENSE("GPL");
