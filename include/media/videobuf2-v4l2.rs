//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/videobuf2-v4l2.h
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


//
// videobuf2-v4l2.h - V4L2 driver helper framework
//
// Copyright (C) 2010 Samsung Electronics
//
// Author: Pawel Osciak <pawel@osciak.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

//
// struct vb2_v4l2_buffer - video buffer information for v4l2.
//
// @vb2_buf:	embedded struct &vb2_buffer.
// @flags:	buffer informational flags.
// @field:	field order of the image in the buffer, as defined by
// &enum v4l2_field.
// @timecode:	frame timecode.
// @sequence:	sequence count of this frame.
// @request_fd:	the request_fd associated with this buffer
// @is_held:	if true, then this capture buffer was held
// @planes:	plane information (userptr/fd, length, bytesused, data_offset).
//
// Should contain enough information to be able to cover all the fields
// of &struct v4l2_buffer at ``videodev2.h``.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vb2_v4l2_buffer {
    pub vb2_buf: vb2_buffer,
    pub flags: __u32,
    pub field: __u32,
    pub timecode: v4l2_timecode,
    pub sequence: __u32,
    pub request_fd: __s32,
    pub is_held: bool,
    pub planes: [vb2_plane; VB2_MAX_PLANES],
}

// VB2 V4L2 flags as set in vb2_queue.subsystem_flags

//
// to_vb2_v4l2_buffer() - cast struct vb2_buffer * to struct vb2_v4l2_buffer
//

//
// vb2_find_buffer() - Find a buffer with given timestamp
//
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @timestamp:	the timestamp to find.
//
// Returns the buffer with the given @timestamp, or NULL if not found.
//
// vb2_querybuf() - Query video buffer information
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @b:		buffer structure passed from userspace to
// &v4l2_ioctl_ops->vidioc_querybuf handler in driver
//
// Should be called from vidioc_querybuf ioctl handler in driver.
// This function will verify the passed v4l2_buffer structure and fill the
// relevant information for the userspace.
//
// The return values from this function are intended to be directly returned
// from vidioc_querybuf handler in driver.
//
extern "C" {
    pub fn vb2_querybuf(q: *mut vb2_queue, b: *mut v4l2_buffer) -> c_int;
}
//
// vb2_reqbufs() - Wrapper for vb2_core_reqbufs() that also verifies
// the memory and type values.
//
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @req:	&struct v4l2_requestbuffers passed from userspace to
// &v4l2_ioctl_ops->vidioc_reqbufs handler in driver.
//
extern "C" {
    pub fn vb2_reqbufs(q: *mut vb2_queue, req: *mut v4l2_requestbuffers) -> c_int;
}
//
// vb2_create_bufs() - Wrapper for vb2_core_create_bufs() that also verifies
// the memory and type values.
//
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @create:	creation parameters, passed from userspace to
// &v4l2_ioctl_ops->vidioc_create_bufs handler in driver
//
extern "C" {
    pub fn vb2_create_bufs(q: *mut vb2_queue, create: *mut v4l2_create_buffers) -> c_int;
}
//
// vb2_prepare_buf() - Pass ownership of a buffer from userspace to the kernel
//
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @mdev:	pointer to &struct media_device, may be NULL.
// @b:		buffer structure passed from userspace to
// &v4l2_ioctl_ops->vidioc_prepare_buf handler in driver
//
// Should be called from &v4l2_ioctl_ops->vidioc_prepare_buf ioctl handler
// of a driver.
//
// This function:
//
// #) verifies the passed buffer,
// #) calls &vb2_ops->buf_prepare callback in the driver (if provided),
// in which driver-specific buffer initialization can be performed.
// #) if @b->request_fd is non-zero and @mdev->ops->req_queue is set,
// then bind the prepared buffer to the request.
//
// The return values from this function are intended to be directly returned
// from &v4l2_ioctl_ops->vidioc_prepare_buf handler in driver.
//
// vb2_qbuf() - Queue a buffer from userspace
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @mdev:	pointer to &struct media_device, may be NULL.
// @b:		buffer structure passed from userspace to
// &v4l2_ioctl_ops->vidioc_qbuf handler in driver
//
// Should be called from &v4l2_ioctl_ops->vidioc_qbuf handler of a driver.
//
// This function:
//
// #) verifies the passed buffer;
// #) if @b->request_fd is non-zero and @mdev->ops->req_queue is set,
// then bind the buffer to the request.
// #) if necessary, calls &vb2_ops->buf_prepare callback in the driver
// (if provided), in which driver-specific buffer initialization can
// be performed;
// #) if streaming is on, queues the buffer in driver by the means of
// &vb2_ops->buf_queue callback for processing.
//
// The return values from this function are intended to be directly returned
// from &v4l2_ioctl_ops->vidioc_qbuf handler in driver.
//
// vb2_expbuf() - Export a buffer as a file descriptor
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @eb:		export buffer structure passed from userspace to
// &v4l2_ioctl_ops->vidioc_expbuf handler in driver
//
// The return values from this function are intended to be directly returned
// from &v4l2_ioctl_ops->vidioc_expbuf handler in driver.
//
extern "C" {
    pub fn vb2_expbuf(q: *mut vb2_queue, eb: *mut v4l2_exportbuffer) -> c_int;
}
//
// vb2_dqbuf() - Dequeue a buffer to the userspace
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @b:		buffer structure passed from userspace to
// &v4l2_ioctl_ops->vidioc_dqbuf handler in driver
// @nonblocking: if true, this call will not sleep waiting for a buffer if no
// buffers ready for dequeuing are present. Normally the driver
// would be passing (&file->f_flags & %O_NONBLOCK) here
//
// Should be called from &v4l2_ioctl_ops->vidioc_dqbuf ioctl handler
// of a driver.
//
// This function:
//
// #) verifies the passed buffer;
// #) calls &vb2_ops->buf_finish callback in the driver (if provided), in which
// driver can perform any additional operations that may be required before
// returning the buffer to userspace, such as cache sync;
// #) the buffer struct members are filled with relevant information for
// the userspace.
//
// The return values from this function are intended to be directly returned
// from &v4l2_ioctl_ops->vidioc_dqbuf handler in driver.
//
extern "C" {
    pub fn vb2_dqbuf(q: *mut vb2_queue, b: *mut v4l2_buffer, nonblocking: bool) -> c_int;
}
//
// vb2_streamon - start streaming
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @type:	type argument passed from userspace to vidioc_streamon handler,
// as defined by &enum v4l2_buf_type.
//
// Should be called from &v4l2_ioctl_ops->vidioc_streamon handler of a driver.
//
// This function:
//
// 1) verifies current state
// 2) passes any previously queued buffers to the driver and starts streaming
//
// The return values from this function are intended to be directly returned
// from &v4l2_ioctl_ops->vidioc_streamon handler in the driver.
//
extern "C" {
    pub fn vb2_streamon(q: *mut vb2_queue, type: v4l2_buf_type) -> c_int;
}
//
// vb2_streamoff - stop streaming
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @type:	type argument passed from userspace to vidioc_streamoff handler
//
// Should be called from vidioc_streamoff handler of a driver.
//
// This function:
//
// #) verifies current state,
// #) stop streaming and dequeues any queued buffers, including those previously
// passed to the driver (after waiting for the driver to finish).
//
// This call can be used for pausing playback.
// The return values from this function are intended to be directly returned
// from vidioc_streamoff handler in the driver
//
extern "C" {
    pub fn vb2_streamoff(q: *mut vb2_queue, type: v4l2_buf_type) -> c_int;
}
//
// vb2_queue_init() - initialize a videobuf2 queue
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
//
// The vb2_queue structure should be allocated by the driver. The driver is
// responsible of clearing it's content and setting initial values for some
// required entries before calling this function.
// q->ops, q->mem_ops, q->type and q->io_modes are mandatory. Please refer
// to the struct vb2_queue description in include/media/videobuf2-core.h
// for more information.
//
extern "C" {
    pub fn vb2_queue_init(q: *mut vb2_queue) -> int __must_check;
}
//
// vb2_queue_init_name() - initialize a videobuf2 queue with a name
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @name:	the queue name
//
// This function initializes the vb2_queue exactly like vb2_queue_init(),
// and additionally sets the queue name. The queue name is used for logging
// purpose, and should uniquely identify the queue within the context of the
// device it belongs to. This is useful to attribute kernel log messages to the
// right queue for m2m devices or other devices that handle multiple queues.
//
extern "C" {
    pub fn vb2_queue_init_name(q: *mut vb2_queue, name: *const c_char) -> int __must_check;
}
//
// vb2_queue_release() - stop streaming, release the queue and free memory
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
//
// This function stops streaming and performs necessary clean ups, including
// freeing video buffer memory. The driver is responsible for freeing
// the vb2_queue structure itself.
//
extern "C" {
    pub fn vb2_queue_release(q: *mut vb2_queue);
}
//
// vb2_queue_change_type() - change the type of an inactive vb2_queue
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @type:	the type to change to (V4L2_BUF_TYPE_VIDEO_*)
//
// This function changes the type of the vb2_queue. This is only possible
// if the queue is not busy (i.e. no buffers have been allocated).
//
// vb2_queue_change_type() can be used to support multiple buffer types using
// the same queue. The driver can implement v4l2_ioctl_ops.vidioc_reqbufs and
// v4l2_ioctl_ops.vidioc_create_bufs functions and call vb2_queue_change_type()
// before calling vb2_ioctl_reqbufs() or vb2_ioctl_create_bufs(), and thus
// "lock" the buffer type until the buffers have been released.
//
extern "C" {
    pub fn vb2_queue_change_type(q: *mut vb2_queue, type: c_uint) -> c_int;
}
//
// vb2_poll() - implements poll userspace operation
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @file:	file argument passed to the poll file operation handler
// @wait:	wait argument passed to the poll file operation handler
//
// This function implements poll file operation handler for a driver.
// For CAPTURE queues, if a buffer is ready to be dequeued, the userspace will
// be informed that the file descriptor of a video device is available for
// reading.
// For OUTPUT queues, if a buffer is ready to be dequeued, the file descriptor
// will be reported as available for writing.
//
// If the driver uses struct v4l2_fh, then vb2_poll() will also check for any
// pending events.
//
// The return values from this function are intended to be directly returned
// from poll handler in driver.
//
extern "C" {
    pub fn vb2_poll(q: *mut vb2_queue, file: *mut file, wait: *mut poll_table) -> __poll_t;
}
//
// The following functions are not part of the vb2 core API, but are simple
// helper functions that you can use in your struct v4l2_file_operations,
// struct v4l2_ioctl_ops and struct vb2_ops. They will serialize if vb2_queue->lock
// or video_device->lock is set, and they will set and test the queue owner
// (vb2_queue->owner) to check if the calling filehandle is permitted to do the
// queuing operation.
//
// vb2_queue_is_busy() - check if the queue is busy
// @q:		pointer to &struct vb2_queue with videobuf2 queue.
// @file:	file through which the vb2 queue access is performed
//
// The queue is considered busy if it has an owner and the owner is not the
// @file.
//
// Queue ownership is acquired and checked by some of the v4l2_ioctl_ops helpers
// below. Drivers can also use this function directly when they need to
// open-code ioctl handlers, for instance to add additional checks between the
// queue ownership test and the call to the corresponding vb2 operation.
//
// struct v4l2_ioctl_ops helpers
extern "C" {
    pub fn vb2_ioctl_querybuf(file: *mut file, priv: *mut c_void, p: *mut v4l2_buffer) -> c_int;
}
extern "C" {
    pub fn vb2_ioctl_qbuf(file: *mut file, priv: *mut c_void, p: *mut v4l2_buffer) -> c_int;
}
extern "C" {
    pub fn vb2_ioctl_dqbuf(file: *mut file, priv: *mut c_void, p: *mut v4l2_buffer) -> c_int;
}
extern "C" {
    pub fn vb2_ioctl_streamon(file: *mut file, priv: *mut c_void, i: v4l2_buf_type) -> c_int;
}
extern "C" {
    pub fn vb2_ioctl_streamoff(file: *mut file, priv: *mut c_void, i: v4l2_buf_type) -> c_int;
}
// struct v4l2_file_operations helpers
extern "C" {
    pub fn vb2_fop_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn vb2_fop_release(file: *mut file) -> c_int;
}
extern "C" {
    pub fn _vb2_fop_release(file: *mut file, lock: *mut mutex) -> c_int;
}
extern "C" {
    pub fn vb2_fop_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;
}

//
// vb2_video_unregister_device - unregister the video device and release queue
//
// @vdev: pointer to &struct video_device
//
// If the driver uses vb2_fop_release()/_vb2_fop_release(), then it should use
// vb2_video_unregister_device() instead of video_unregister_device().
//
// This function will call video_unregister_device() and then release the
// vb2_queue if streaming is in progress. This will stop streaming and
// this will simplify the unbind sequence since after this call all subdevs
// will have stopped streaming as well.
//
extern "C" {
    pub fn vb2_video_unregister_device(vdev: *mut video_device);
}
extern "C" {
    pub fn vb2_request_validate(req: *mut media_request) -> c_int;
}
extern "C" {
    pub fn vb2_request_queue(req: *mut media_request);
}
