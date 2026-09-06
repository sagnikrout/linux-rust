//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dmub/src/dmub_dcn32.h
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
// Copyright 2022 Advanced Micro Devices, Inc.
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

// DCN32 register definitions.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn32_reg_offset {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn32_reg_shift {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn32_reg_mask {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn32_regs {
    pub offset: dmub_srv_dcn32_reg_offset,
    pub mask: dmub_srv_dcn32_reg_mask,
    pub shift: dmub_srv_dcn32_reg_shift,
}

extern "C" {
    pub fn dmub_dcn32_reset(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn32_reset_release(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn32_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_set_inbox1_wptr(dmub: *mut dmub_srv, wptr_offset: u32);
}
extern "C" {
    pub fn dmub_dcn32_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_set_outbox1_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
}
extern "C" {
    pub fn dmub_dcn32_is_hw_init(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn32_is_supported(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn32_get_gpint_response(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_get_gpint_dataout(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params);
}
extern "C" {
    pub fn dmub_dcn32_skip_dmub_panel_power_sequence(dmub: *mut dmub_srv, skip: bool);
}
extern "C" {
    pub fn dmub_dcn32_get_fw_boot_status(dmub: *mut dmub_srv) -> dmub_fw_boot_status;
}
extern "C" {
    pub fn dmub_dcn32_get_outbox0_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_set_outbox0_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
}
extern "C" {
    pub fn dmub_dcn32_get_current_time(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_get_diagnostic_data(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn32_configure_dmub_in_system_memory(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn32_send_inbox0_cmd(dmub: *mut dmub_srv, data: dmub_inbox0_data_register);
}
extern "C" {
    pub fn dmub_dcn32_clear_inbox0_ack_register(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn32_read_inbox0_ack_register(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn32_save_surf_addr(dmub: *mut dmub_srv, addr: *const dc_plane_address, subvp_index: u8);
}
extern "C" {
    pub fn dmub_srv_dcn32_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context);
}
