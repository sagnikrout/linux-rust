//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/buffer_impl.h
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
// INDIO_BUFFER_FLAG_FIXED_WATERMARK - Watermark level of the buffer can not be
// configured. It has a fixed value which will be buffer specific.
//

//
// struct iio_buffer_access_funcs - access functions for buffers.
// @store_to:		actually store stuff to the buffer - must be safe to
// call from any context (e.g. must not sleep).
// @read:		try to get a specified number of bytes (must exist)
// @data_available:	indicates how much data is available for reading from
// the buffer.
// @remove_from:	remove scan from buffer. Drivers should calls this to
// remove a scan from a buffer.
// @write:		try to write a number of bytes
// @space_available:	returns the amount of bytes available in a buffer
// @request_update:	if a parameter change has been marked, update underlying
// storage.
// @set_bytes_per_datum:set number of bytes per datum
// @set_length:		set number of datums in buffer
// @enable:             called if the buffer is attached to a device and the
// device starts sampling. Calls are balanced with
// @disable.
// @disable:            called if the buffer is attached to a device and the
// device stops sampling. Calles are balanced with @enable.
// @release:		called when the last reference to the buffer is dropped,
// should free all resources allocated by the buffer.
// @attach_dmabuf:	called from userspace via ioctl to attach one external
// DMABUF.
// @detach_dmabuf:	called from userspace via ioctl to detach one previously
// attached DMABUF.
// @enqueue_dmabuf:	called from userspace via ioctl to queue this DMABUF
// object to this buffer. Requires a valid DMABUF fd, that
// was previouly attached to this buffer.
// @get_dma_dev:	called to get the DMA channel associated with this buffer.
// @lock_queue:		called when the core needs to lock the buffer queue;
// it is used when enqueueing DMABUF objects.
// @unlock_queue:       used to unlock a previously locked buffer queue
// @modes:		Supported operating modes by this buffer type
// @flags:		A bitmask combination of INDIO_BUFFER_FLAG_
//
// The purpose of this structure is to make the buffer element
// modular as event for a given driver, different usecases may require
// different buffer designs (space efficiency vs speed for example).
//
// It is worth noting that a given buffer implementation may only support a
// small proportion of these functions.  The core code 'should' cope fine with
// any of them not existing.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_buffer_access_funcs {
    pub data): *const *const *const int (store_to)(struct iio_buffer buffer, void,
    pub buf): *mut *mut *mut int (read)(struct iio_buffer buffer, size_t n, char __user,
    pub buffer): *mut *mut size_t (data_available)(struct iio_buffer,
    pub data): *mut *mut *mut int (remove_from)(struct iio_buffer buffer, void,
    pub buf): *const *const *const int (write)(struct iio_buffer buffer, size_t n, char __user,
    pub buffer): *mut *mut size_t (space_available)(struct iio_buffer,
    pub buffer): *mut *mut int (request_update)(struct iio_buffer,
    pub bpd): *mut *mut *mut int (set_bytes_per_datum)(struct iio_buffer buffer, size_t,
    pub length): *mut *mut *mut int (set_length)(struct iio_buffer buffer, unsigned int,
    pub indio_dev): *mut *mut *mut int (enable)(struct iio_buffer buffer, struct iio_dev,
    pub indio_dev): *mut *mut *mut int (disable)(struct iio_buffer buffer, struct iio_dev,
    pub buffer): *mut *mut void (release)(struct iio_buffer,
    pub attach): *mut dma_buf_attachment,
    pub block): *mut iio_dma_buffer_block,
    pub cyclic): size_t size, bool,
    pub buffer): *mut *mut *mut device  (get_dma_dev)(iio_buffer,
    pub buffer): *mut *mut void (lock_queue)(struct iio_buffer,
    pub buffer): *mut *mut void (unlock_queue)(struct iio_buffer,
    pub modes: c_uint,
    pub flags: c_uint,
}

//
// struct iio_buffer - general buffer structure
//
// Note that the internals of this structure should only be of interest to
// those writing new buffer implementations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_buffer {
// @length: Number of datums in buffer.
    pub length: c_uint,
// @flags: File ops flags including busy flag.
    pub flags: c_ulong,
// @bytes_per_datum: Size of individual datum including timestamp.
    pub bytes_per_datum: usize,
// @direction: Direction of the data stream (in/out).
    pub direction: iio_buffer_direction,
//
// @access: Buffer access functions associated with the
// implementation.
//
    pub access: *const iio_buffer_access_funcs,
// @scan_mask: Bitmask used in masking scan mode elements.
    pub scan_mask: *mut c_long,
// @demux_list: List of operations required to demux the scan.
    pub demux_list: list_head,
// @pollq: Wait queue to allow for polling on the buffer.
    pub pollq: wait_queue_head_t,
// @watermark: Number of datums to wait for poll/read.
    pub watermark: c_uint,
// private:
// @scan_timestamp: Does the scan mode include a timestamp.
    pub scan_timestamp: bool,
// @buffer_attr_list: List of buffer attributes.
    pub buffer_attr_list: list_head,
//
// @buffer_group: Attributes of the new buffer group.
// Includes scan elements attributes.
//
    pub buffer_group: attribute_group,
// @attrs: Standard attributes of the buffer.
    pub attrs: *const iio_dev_attr,
// @demux_bounce: Buffer for doing gather from incoming scan.
    pub demux_bounce: *mut c_void,
// @attached_entry: Entry in the devices list of buffers attached by the driver.
    pub attached_entry: list_head,
// @buffer_list: Entry in the devices list of current buffers.
    pub buffer_list: list_head,
// @ref: Reference count of the buffer.
    pub ref: kref,
// @dmabufs: List of DMABUF attachments
    pub /: *mut *mut list_head dmabufs; / P: dmabufs_mutex,
// @dmabufs_mutex: Protects dmabufs
    pub dmabufs_mutex: mutex,
}

//
// iio_update_buffers() - add or remove buffer from active list
// @indio_dev:		device to add buffer to
// @insert_buffer:	buffer to insert
// @remove_buffer:	buffer_to_remove
//
// Note this will tear down all the buffering and build it up again
//
// Returns: 0 on success or -errno on error
//
// iio_buffer_init() - Initialize the buffer structure
// @buffer:		buffer to be initialized
//
extern "C" {
    pub fn iio_buffer_init(buffer: *mut iio_buffer);
}
extern "C" {
    pub fn iio_buffer_put(buffer: *mut iio_buffer);
}
extern "C" {
    pub fn iio_buffer_signal_dmabuf_done(fence: *mut dma_fence, ret: c_int);
}

