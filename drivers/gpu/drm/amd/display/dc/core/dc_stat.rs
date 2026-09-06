//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/core/dc_stat.c
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
// Copyright 2020 Advanced Micro Devices, Inc.
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
// DOC: DC STAT Interface
//
// These interfaces are called without acquiring DAL and DC locks.
// Hence, there is limitations on whese interfaces can access. Only
// variables exclusively defined for these interfaces can be modified.
//
// dc_stat_get_dmub_notification
//
// Calls dmub layer to retrieve dmub notification
//
// @dc: dc structure
// @notify: dmub notification structure
//
// Returns
// None
//
#[no_mangle]
pub unsafe extern "C" fn dc_stat_get_dmub_notification(dc: *const dc, notify: *mut dmub_notification) {
    void dc_stat_get_dmub_notification(const struct dc *dc, struct dmub_notification *notify)
    {
//
// This function is called without dal and dc locks, so
// we shall not modify any dc, dc_dmub_srv or dmub variables
// except variables exclusively accessed by this function
//
    struct dmub_srv *dmub = dc.ctx.dmub_srv.dmub;
    enum dmub_status status;
    status = dmub_srv_stat_get_notification(dmub, notify);
    ASSERT(status == DMUB_STATUS_OK);
// For HPD/HPD RX, convert dpia port index into link index
    if (notify.type == DMUB_NOTIFICATION_HPD ||
    notify.type == DMUB_NOTIFICATION_HPD_IRQ ||
    notify.type == DMUB_NOTIFICATION_AUX_REPLY ||
    notify.type == DMUB_NOTIFICATION_DPIA_NOTIFICATION ||
    notify.type == DMUB_NOTIFICATION_SET_CONFIG_REPLY) {
    notify.link_index =
    get_link_index_from_dpia_port_index(dc, notify.instance);
    }
    }
//
// dc_stat_get_dmub_dataout
//
// Calls dmub layer to retrieve dmub gpint dataout
//
// @dc: dc structure
// @dataout: dmub gpint dataout
//
// Returns
// None
//
#[no_mangle]
pub unsafe extern "C" fn dc_stat_get_dmub_dataout(dc: *const dc, dataout: *mut u32) {
    void dc_stat_get_dmub_dataout(const struct dc *dc, uint32_t *dataout)
    {
    struct dmub_srv *dmub = dc.ctx.dmub_srv.dmub;
    enum dmub_status status;
    status = dmub_srv_get_gpint_dataout(dmub, dataout);
    ASSERT(status == DMUB_STATUS_OK);
    }
