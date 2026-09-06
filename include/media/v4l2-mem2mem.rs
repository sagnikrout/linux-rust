//! Automatically rewritten from C Header to Rust Module
//! Source: include/media/v4l2-mem2mem.h
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
// Memory-to-memory device framework for Video for Linux 2.
//
// Helper functions for devices that use memory buffers for both source
// and destination.
//
// Copyright (c) 2009 Samsung Electronics Co., Ltd.
// Pawel Osciak, <pawel@osciak.com>
// Marek Szyprowski, <m.szyprowski@samsung.com>
//

//
// struct v4l2_m2m_ops - mem-to-mem device driver callbacks
// @device_run:	required. Begin the actual job (transaction) inside this
// callback.
// The job does NOT have to end before this callback returns
// (and it will be the usual case). When the job finishes,
// v4l2_m2m_job_finish() or v4l2_m2m_buf_done_and_job_finish()
// has to be called.
// @job_ready:	optional. Should return 0 if the driver does not have a job
// fully prepared to run yet (i.e. it will not be able to finish a
// transaction without sleeping). If not provided, it will be
// assumed that one source and one destination buffer are all
// that is required for the driver to perform one full transaction.
// This method may not sleep.
// @job_abort:	optional. Informs the driver that it has to abort the currently
// running transaction as soon as possible (i.e. as soon as it can
// stop the device safely; e.g. in the next interrupt handler),
// even if the transaction would not have been finished by then.
// After the driver performs the necessary steps, it has to call
// v4l2_m2m_job_finish() or v4l2_m2m_buf_done_and_job_finish() as
// if the transaction ended normally.
// This function does not have to (and will usually not) wait
// until the device enters a state when it can be stopped.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_m2m_ops {
    pub priv): *mut *mut void (device_run)(void,
    pub priv): *mut *mut int (job_ready)(void,
    pub priv): *mut *mut void (job_abort)(void,
}

//
// struct v4l2_m2m_queue_ctx - represents a queue for buffers ready to be
// processed
//
// @q:		pointer to struct &vb2_queue
// @rdy_queue:	List of V4L2 mem-to-mem queues
// @rdy_spinlock: spin lock to protect the struct usage
// @num_rdy:	number of buffers ready to be processed
// @buffered:	is the queue buffered?
//
// Queue for buffers ready to be processed as soon as this
// instance receives access to the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_m2m_queue_ctx {
    pub q: vb2_queue,
    pub rdy_queue: list_head,
    pub rdy_spinlock: spinlock_t,
    pub num_rdy: u8,
    pub buffered: bool,
}

//
// struct v4l2_m2m_ctx - Memory to memory context structure
//
// @q_lock: struct &mutex lock
// @new_frame: valid in the device_run callback: if true, then this
// starts a new frame; if false, then this is a new slice
// for an existing frame. This is always true unless
// V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF is set, which
// indicates slicing support.
// @is_draining: indicates device is in draining phase
// @last_src_buf: indicate the last source buffer for draining
// @next_buf_last: next capture queud buffer will be tagged as last
// @has_stopped: indicate the device has been stopped
// @ignore_cap_streaming: If true, job_ready can be called even if the CAPTURE
// queue is not streaming. This allows firmware to
// analyze the bitstream header which arrives on the
// OUTPUT queue. The driver must implement the job_ready
// callback correctly to make sure that the requirements
// for actual decoding are met.
// @m2m_dev: opaque pointer to the internal data to handle M2M context
// @cap_q_ctx: Capture (output to memory) queue context
// @out_q_ctx: Output (input from memory) queue context
// @queue: List of memory to memory contexts
// @job_flags: Job queue flags, used internally by v4l2-mem2mem.c:
// %TRANS_QUEUED, %TRANS_RUNNING and %TRANS_ABORT.
// @finished: Wait queue used to signalize when a job queue finished.
// @priv: Instance private data
//
// The memory to memory context is specific to a file handle, NOT to e.g.
// a device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_m2m_ctx {
// optional cap/out vb2 queues lock
    pub q_lock: *mut mutex,
    pub new_frame: bool,
    pub is_draining: bool,
    pub last_src_buf: *mut vb2_v4l2_buffer,
    pub next_buf_last: bool,
    pub has_stopped: bool,
    pub ignore_cap_streaming: bool,
// internal use only
    pub m2m_dev: *mut v4l2_m2m_dev,
    pub cap_q_ctx: v4l2_m2m_queue_ctx,
    pub out_q_ctx: v4l2_m2m_queue_ctx,
// For device job queue
    pub queue: list_head,
    pub job_flags: c_ulong,
    pub finished: wait_queue_head_t,
    pub priv: *mut c_void,
}

//
// struct v4l2_m2m_buffer - Memory to memory buffer
//
// @vb: pointer to struct &vb2_v4l2_buffer
// @list: list of m2m buffers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct v4l2_m2m_buffer {
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

//
// v4l2_m2m_get_curr_priv() - return driver private data for the currently
// running instance or NULL if no instance is running
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
//
// v4l2_m2m_get_vq() - return vb2_queue for the given type
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @type: type of the V4L2 buffer, as defined by enum &v4l2_buf_type
//
// This function returns the capture queue when @type is a capture type, and the
// output queue otherwise. It never returns a NULL pointer.
//
// v4l2_m2m_try_schedule() - check whether an instance is ready to be added to
// the pending job queue and add it if so.
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// There are three basic requirements an instance has to meet to be able to run:
// 1) at least one source buffer has to be queued,
// 2) at least one destination buffer has to be queued,
// 3) streaming has to be on.
//
// If a queue is buffered (for example a decoder hardware ringbuffer that has
// to be drained before doing streamoff), allow scheduling without v4l2 buffers
// on that queue.
//
// There may also be additional, custom requirements. In such case the driver
// should supply a custom callback (job_ready in v4l2_m2m_ops) that should
// return 1 if the instance is ready.
// An example of the above could be an instance that requires more than one
// src/dst buffer per transaction.
//
extern "C" {
    pub fn v4l2_m2m_try_schedule(m2m_ctx: *mut v4l2_m2m_ctx);
}
//
// v4l2_m2m_job_finish() - inform the framework that a job has been finished
// and have it clean up
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// Called by a driver to yield back the device after it has finished with it.
// Should be called as soon as possible after reaching a state which allows
// other instances to take control of the device.
//
// This function has to be called only after &v4l2_m2m_ops->device_run
// callback has been called on the driver.
//
// v4l2_m2m_buf_done_and_job_finish() - return source/destination buffers with
// state and inform the framework that a job has been finished and have it
// clean up
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @state: vb2 buffer state passed to v4l2_m2m_buf_done().
//
// Drivers that set V4L2_BUF_CAP_SUPPORTS_M2M_HOLD_CAPTURE_BUF must use this
// function instead of job_finish() to take held buffers into account. It is
// optional for other drivers.
//
// This function removes the source buffer from the ready list and returns
// it with the given state. The same is done for the destination buffer, unless
// it is marked 'held'. In that case the buffer is kept on the ready list.
//
// After that the job is finished (see job_finish()).
//
// This allows for multiple output buffers to be used to fill in a single
// capture buffer. This is typically used by stateless decoders where
// multiple e.g. H.264 slices contribute to a single decoded frame.
//
// v4l2_m2m_clear_state() - clear encoding/decoding state
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_mark_stopped() - set current encoding/decoding state as stopped
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_dst_buf_is_last() - return the current encoding/decoding session
// draining management state of next queued capture buffer
//
// This last capture buffer should be tagged with V4L2_BUF_FLAG_LAST to notify
// the end of the capture session.
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_has_stopped() - return the current encoding/decoding session
// stopped state
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_is_last_draining_src_buf() - return the output buffer draining
// state in the current encoding/decoding session
//
// This will identify the last output buffer queued before a session stop
// was required, leading to an actual encoding/decoding session stop state
// in the encoding/decoding process after being processed.
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @vbuf: pointer to struct &v4l2_buffer
//
// v4l2_m2m_last_buffer_done() - marks the buffer with LAST flag and DONE
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @vbuf: pointer to struct &v4l2_buffer
//
// v4l2_m2m_suspend() - stop new jobs from being run and wait for current job
// to finish
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
//
// Called by a driver in the suspend hook. Stop new jobs from being run, and
// wait for current running job to finish.
//
extern "C" {
    pub fn v4l2_m2m_suspend(m2m_dev: *mut v4l2_m2m_dev);
}
//
// v4l2_m2m_resume() - resume job running and try to run a queued job
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
//
// Called by a driver in the resume hook. This reverts the operation of
// v4l2_m2m_suspend() and allows job to be run. Also try to run a queued job if
// there is any.
//
extern "C" {
    pub fn v4l2_m2m_resume(m2m_dev: *mut v4l2_m2m_dev);
}
//
// v4l2_m2m_reqbufs() - multi-queue-aware REQBUFS multiplexer
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @reqbufs: pointer to struct &v4l2_requestbuffers
//
// v4l2_m2m_querybuf() - multi-queue-aware QUERYBUF multiplexer
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @buf: pointer to struct &v4l2_buffer
//
// See v4l2_m2m_mmap() documentation for details.
//
// v4l2_m2m_qbuf() - enqueue a source or destination buffer, depending on
// the type
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @buf: pointer to struct &v4l2_buffer
//
// v4l2_m2m_dqbuf() - dequeue a source or destination buffer, depending on
// the type
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @buf: pointer to struct &v4l2_buffer
//
// v4l2_m2m_prepare_buf() - prepare a source or destination buffer, depending on
// the type
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @buf: pointer to struct &v4l2_buffer
//
// v4l2_m2m_create_bufs() - create a source or destination buffer, depending
// on the type
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @create: pointer to struct &v4l2_create_buffers
//
// v4l2_m2m_expbuf() - export a source or destination buffer, depending on
// the type
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @eb: pointer to struct &v4l2_exportbuffer
//
// v4l2_m2m_streamon() - turn on streaming for a video queue
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @type: type of the V4L2 buffer, as defined by enum &v4l2_buf_type
//
// v4l2_m2m_streamoff() - turn off streaming for a video queue
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @type: type of the V4L2 buffer, as defined by enum &v4l2_buf_type
//
// v4l2_m2m_update_start_streaming_state() - update the encoding/decoding
// session state when a start of streaming of a video queue is requested
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @q: queue
//
// v4l2_m2m_update_stop_streaming_state() -  update the encoding/decoding
// session state when a stop of streaming of a video queue is requested
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @q: queue
//
// v4l2_m2m_encoder_cmd() - execute an encoder command
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @ec: pointer to the encoder command
//
// v4l2_m2m_decoder_cmd() - execute a decoder command
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @dc: pointer to the decoder command
//
// v4l2_m2m_poll() - poll replacement, for destination buffers only
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @wait: pointer to struct &poll_table_struct
//
// Call from the driver's poll() function. Will poll both queues. If a buffer
// is available to dequeue (with dqbuf) from the source queue, this will
// indicate that a non-blocking write can be performed, while read will be
// returned in case of the destination queue.
//
// v4l2_m2m_mmap() - source and destination queues-aware mmap multiplexer
//
// @file: pointer to struct &file
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @vma: pointer to struct &vm_area_struct
//
// Call from driver's mmap() function. Will handle mmap() for both queues
// seamlessly for the video buffer, which will receive normal per-queue offsets
// and proper vb2 queue pointers. The differentiation is made outside
// vb2 by adding a predefined offset to buffers from one of the queues
// and subtracting it before passing it back to vb2. Only drivers (and
// thus applications) receive modified offsets.
//

//
// v4l2_m2m_init() - initialize per-driver m2m data
//
// @m2m_ops: pointer to struct v4l2_m2m_ops
//
// Usually called from driver's ``probe()`` function.
//
// Return: returns an opaque pointer to the internal data to handle M2M context
//

extern "C" {
    pub fn v4l2_m2m_unregister_media_controller(m2m_dev: *mut v4l2_m2m_dev);
}

//
// v4l2_m2m_release() - cleans up and frees a m2m_dev structure
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
//
// Usually called from driver's ``remove()`` function.
//
extern "C" {
    pub fn v4l2_m2m_release(m2m_dev: *mut v4l2_m2m_dev);
}
//
// v4l2_m2m_get() - take a reference to the m2m_dev structure
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
//
// This is used to share the M2M device across multiple devices. This
// can be used to avoid scheduling two hardware nodes concurrently.
//
extern "C" {
    pub fn v4l2_m2m_get(m2m_dev: *mut v4l2_m2m_dev);
}
//
// v4l2_m2m_put() - remove a reference to the m2m_dev structure
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
//
// Once the M2M device has no more references, v4l2_m2m_release() will be
// called automatically. Users of this method should never call
// v4l2_m2m_release() directly. See v4l2_m2m_get() for more details.
//
extern "C" {
    pub fn v4l2_m2m_put(m2m_dev: *mut v4l2_m2m_dev);
}
//
// v4l2_m2m_ctx_init() - allocate and initialize a m2m context
//
// @m2m_dev: opaque pointer to the internal data to handle M2M context
// @drv_priv: driver's instance private data
// @queue_init: a callback for queue type-specific initialization function
// to be used for initializing vb2_queues
//
// Usually called from driver's ``open()`` function.
//
// v4l2_m2m_ctx_release() - release m2m context
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// Usually called from driver's release() function.
//
extern "C" {
    pub fn v4l2_m2m_ctx_release(m2m_ctx: *mut v4l2_m2m_ctx);
}
//
// v4l2_m2m_buf_queue() - add a buffer to the proper ready buffers list.
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @vbuf: pointer to struct &vb2_v4l2_buffer
//
// Call from vb2_queue_ops->ops->buf_queue, vb2_queue_ops callback.
//
// v4l2_m2m_num_src_bufs_ready() - return the number of source buffers ready for
// use
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_num_dst_bufs_ready() - return the number of destination buffers
// ready for use
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_next_buf() - return next buffer from the list of ready buffers
//
// @q_ctx: pointer to struct @v4l2_m2m_queue_ctx
//
// v4l2_m2m_next_src_buf() - return next source buffer from the list of ready
// buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
extern "C" {
    pub fn v4l2_m2m_next_buf(_arg: &m2m_ctx->out_q_ctx) -> return;
}
//
// v4l2_m2m_next_dst_buf() - return next destination buffer from the list of
// ready buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
extern "C" {
    pub fn v4l2_m2m_next_buf(_arg: &m2m_ctx->cap_q_ctx) -> return;
}
//
// v4l2_m2m_last_buf() - return last buffer from the list of ready buffers
//
// @q_ctx: pointer to struct @v4l2_m2m_queue_ctx
//
// v4l2_m2m_last_src_buf() - return last source buffer from the list of
// ready buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
extern "C" {
    pub fn v4l2_m2m_last_buf(_arg: &m2m_ctx->out_q_ctx) -> return;
}
//
// v4l2_m2m_last_dst_buf() - return last destination buffer from the list of
// ready buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
extern "C" {
    pub fn v4l2_m2m_last_buf(_arg: &m2m_ctx->cap_q_ctx) -> return;
}
//
// v4l2_m2m_for_each_dst_buf() - iterate over a list of destination ready
// buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @b: current buffer of type struct v4l2_m2m_buffer
//

//
// v4l2_m2m_for_each_src_buf() - iterate over a list of source ready buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @b: current buffer of type struct v4l2_m2m_buffer
//

//
// v4l2_m2m_for_each_dst_buf_safe() - iterate over a list of destination ready
// buffers safely
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @b: current buffer of type struct v4l2_m2m_buffer
// @n: used as temporary storage
//

//
// v4l2_m2m_for_each_src_buf_safe() - iterate over a list of source ready
// buffers safely
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @b: current buffer of type struct v4l2_m2m_buffer
// @n: used as temporary storage
//

//
// v4l2_m2m_get_src_vq() - return vb2_queue for source buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_get_dst_vq() - return vb2_queue for destination buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
// v4l2_m2m_buf_remove() - take off a buffer from the list of ready buffers and
// return it
//
// @q_ctx: pointer to struct @v4l2_m2m_queue_ctx
//
// v4l2_m2m_src_buf_remove() - take off a source buffer from the list of ready
// buffers and return it
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
extern "C" {
    pub fn v4l2_m2m_buf_remove(_arg: &m2m_ctx->out_q_ctx) -> return;
}
//
// v4l2_m2m_dst_buf_remove() - take off a destination buffer from the list of
// ready buffers and return it
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
//
extern "C" {
    pub fn v4l2_m2m_buf_remove(_arg: &m2m_ctx->cap_q_ctx) -> return;
}
//
// v4l2_m2m_buf_remove_by_buf() - take off exact buffer from the list of ready
// buffers
//
// @q_ctx: pointer to struct @v4l2_m2m_queue_ctx
// @vbuf: the buffer to be removed
//
// v4l2_m2m_src_buf_remove_by_buf() - take off exact source buffer from the list
// of ready buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @vbuf: the buffer to be removed
//
// v4l2_m2m_dst_buf_remove_by_buf() - take off exact destination buffer from the
// list of ready buffers
//
// @m2m_ctx: m2m context assigned to the instance given by struct &v4l2_m2m_ctx
// @vbuf: the buffer to be removed
//
extern "C" {
    pub fn v4l2_m2m_buf_remove_by_idx(_arg: &m2m_ctx->out_q_ctx, _arg: idx) -> return;
}
extern "C" {
    pub fn v4l2_m2m_buf_remove_by_idx(_arg: &m2m_ctx->cap_q_ctx, _arg: idx) -> return;
}
//
// v4l2_m2m_buf_copy_metadata() - copy buffer metadata from
// the output buffer to the capture buffer
//
// @out_vb: the output buffer that is the source of the metadata.
// @cap_vb: the capture buffer that will receive the metadata.
//
// This helper function copies the timestamp, timecode (if the TIMECODE
// buffer flag was set), field, and the TIMECODE and TSTAMP_SRC_MASK flags from
// @out_vb to @cap_vb.
//
// v4l2 request helper
extern "C" {
    pub fn v4l2_m2m_request_queue(req: *mut media_request);
}
// v4l2 ioctl helpers
extern "C" {
    pub fn v4l2_m2m_fop_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn v4l2_m2m_fop_poll(file: *mut file, wait: *mut poll_table) -> __poll_t;
}
