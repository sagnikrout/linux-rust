//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dmub/src/dmub_dcn35.h
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

// DCN35 register definitions.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn35_reg_offset {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn35_reg_shift {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn35_reg_mask {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn35_regs {
    pub offset: dmub_srv_dcn35_reg_offset,
    pub mask: dmub_srv_dcn35_reg_mask,
    pub shift: dmub_srv_dcn35_reg_shift,
}

// Hardware functions.
extern "C" {
    pub fn dmub_dcn35_init(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn35_reset(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn35_reset_release(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn35_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_set_inbox1_wptr(dmub: *mut dmub_srv, wptr_offset: u32);
}
extern "C" {
    pub fn dmub_dcn35_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_set_outbox1_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
}
extern "C" {
    pub fn dmub_dcn35_is_hw_init(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn35_is_supported(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn35_get_gpint_response(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_get_gpint_dataout(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params);
}
extern "C" {
    pub fn dmub_dcn35_skip_dmub_panel_power_sequence(dmub: *mut dmub_srv, skip: bool);
}
extern "C" {
    pub fn dmub_dcn35_get_fw_boot_status(dmub: *mut dmub_srv) -> dmub_fw_boot_status;
}
extern "C" {
    pub fn dmub_dcn35_get_fw_boot_option(dmub: *mut dmub_srv) -> dmub_fw_boot_options;
}
extern "C" {
    pub fn dmub_dcn35_get_outbox0_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_set_outbox0_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
}
extern "C" {
    pub fn dmub_dcn35_get_current_time(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_get_diagnostic_data(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn35_configure_dmub_in_system_memory(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn35_send_inbox0_cmd(dmub: *mut dmub_srv, data: dmub_inbox0_data_register);
}
extern "C" {
    pub fn dmub_dcn35_clear_inbox0_ack_register(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn35_read_inbox0_ack_register(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn35_should_detect(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn35_is_hw_powered_up(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_srv_dcn35_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context);
}
extern "C" {
    pub fn dmub_dcn35_get_preos_fw_info(dmub: *mut dmub_srv) -> bool;
}
