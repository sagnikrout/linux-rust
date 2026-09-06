//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/core/dc_sink.c
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

//
// Private functions
//
#[no_mangle]
unsafe extern "C" fn dc_sink_construct(sink: *mut dc_sink, init_params: *const dc_sink_init_data) -> bool {
    static bool dc_sink_construct(struct dc_sink *sink, const struct dc_sink_init_data *init_params)
    {
    struct dc_link *link = init_params.link;
    if (!link)
    return false;
    sink.sink_signal = init_params.sink_signal;
    sink.link = link;
    sink.ctx = link.ctx;
    sink.dongle_max_pix_clk = init_params.dongle_max_pix_clk;
    sink.converter_disable_audio = init_params.converter_disable_audio;
    sink.dc_container_id = core::ptr::null_mut();
    sink.sink_id = init_params.link.ctx.dc_sink_id_count;
// increment dc_sink_id_count because we don't want two sinks with same ID
// unless they are actually the same
    init_params.link.ctx.dc_sink_id_count++;
    return true;
    }
//
// Public functions
//
#[no_mangle]
pub unsafe extern "C" fn dc_sink_retain(sink: *mut dc_sink) {
    void dc_sink_retain(struct dc_sink *sink)
    {
    kref_get(&sink.refcount);
    }
    EXPORT_IF_KUNIT(dc_sink_retain);
#[no_mangle]
unsafe extern "C" fn dc_sink_free(kref: *mut kref) {
    static void dc_sink_free(struct kref *kref)
    {
    struct dc_sink *sink = container_of(kref, struct dc_sink, refcount);
    kfree(sink.dc_container_id);
    kfree(sink);
    }
#[no_mangle]
pub unsafe extern "C" fn dc_sink_release(sink: *mut dc_sink) {
    void dc_sink_release(struct dc_sink *sink)
    {
    kref_put(&sink.refcount, dc_sink_free);
    }
    EXPORT_IF_KUNIT(dc_sink_release);
    struct dc_sink *dc_sink_create(const struct dc_sink_init_data *init_params)
    {
    struct dc_sink *sink = kzalloc_obj(*sink);
    if (core::ptr::null_mut() == sink)
    goto alloc_fail;
    if (false == dc_sink_construct(sink, init_params))
    goto construct_fail;
    kref_init(&sink.refcount);
    return sink;
    construct_fail:
    kfree(sink);
    alloc_fail:
    return core::ptr::null_mut();
    }
    EXPORT_IF_KUNIT(dc_sink_create);
//
// Protected functions - visible only inside of DC (not visible in DM)
//
