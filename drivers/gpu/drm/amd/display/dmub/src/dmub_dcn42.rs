//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dmub/src/dmub_dcn42.h
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


// SPDX-License-Identifier: MIT
//
// Copyright 2026 Advanced Micro Devices, Inc.
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

// DCN42 register definitions.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn42_reg_offset {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn42_reg_shift {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn42_reg_mask {

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_dcn42_regs {
    pub offset: dmub_srv_dcn42_reg_offset,
    pub mask: dmub_srv_dcn42_reg_mask,
    pub shift: dmub_srv_dcn42_reg_shift,
}

// Function declarations
// Initialization and configuration
extern "C" {
    pub fn dmub_srv_dcn42_regs_init(dmub: *mut dmub_srv, ctx: *mut dc_context);
}
extern "C" {
    pub fn dmub_dcn42_enable_dmub_boot_options(dmub: *mut dmub_srv, params: *const dmub_srv_hw_params);
}
extern "C" {
    pub fn dmub_dcn42_skip_dmub_panel_power_sequence(dmub: *mut dmub_srv, skip: bool);
}
extern "C" {
    pub fn dmub_dcn42_configure_dmub_in_system_memory(dmub: *mut dmub_srv);
}
// Reset and control
extern "C" {
    pub fn dmub_dcn42_reset(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn42_reset_release(dmub: *mut dmub_srv);
}
// Firmware loading
extern "C" {
    pub fn dmub_dcn42_backdoor_load(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window);
}
extern "C" {
    pub fn dmub_dcn42_backdoor_load_zfb_mode(dmub: *mut dmub_srv, cw0: *const dmub_window, cw1: *const dmub_window);
}
extern "C" {
    pub fn dmub_dcn42_setup_windows(dmub: *mut dmub_srv, cw2: *const dmub_window, cw3: *const dmub_window, cw4: *const dmub_window, cw5: *const dmub_window, cw6: *const dmub_window, region6: *const dmub_window);
}
// Mailbox operations - Inbox1
extern "C" {
    pub fn dmub_dcn42_setup_mailbox(dmub: *mut dmub_srv, inbox1: *const dmub_region);
}
extern "C" {
    pub fn dmub_dcn42_get_inbox1_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_get_inbox1_rptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_set_inbox1_wptr(dmub: *mut dmub_srv, wptr_offset: u32);
}
// Mailbox operations - Outbox1
extern "C" {
    pub fn dmub_dcn42_setup_out_mailbox(dmub: *mut dmub_srv, outbox1: *const dmub_region);
}
extern "C" {
    pub fn dmub_dcn42_get_outbox1_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_set_outbox1_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
}
// Mailbox operations - Outbox0
extern "C" {
    pub fn dmub_dcn42_setup_outbox0(dmub: *mut dmub_srv, outbox0: *const dmub_region);
}
extern "C" {
    pub fn dmub_dcn42_get_outbox0_wptr(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_set_outbox0_rptr(dmub: *mut dmub_srv, rptr_offset: u32);
}
// Mailbox operations - Inbox0
extern "C" {
    pub fn dmub_dcn42_send_inbox0_cmd(dmub: *mut dmub_srv, data: dmub_inbox0_data_register);
}
extern "C" {
    pub fn dmub_dcn42_clear_inbox0_ack_register(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn42_read_inbox0_ack_register(dmub: *mut dmub_srv) -> u32;
}
// REG Inbox0/Outbox0 operations
extern "C" {
    pub fn dmub_dcn42_send_reg_inbox0_cmd_msg(dmub: *mut dmub_srv, cmd: *mut dmub_rb_cmd);
}
extern "C" {
    pub fn dmub_dcn42_read_reg_inbox0_rsp_int_status(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_read_reg_inbox0_cmd_rsp(dmub: *mut dmub_srv, cmd: *mut dmub_rb_cmd);
}
extern "C" {
    pub fn dmub_dcn42_write_reg_inbox0_rsp_int_ack(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn42_clear_reg_inbox0_rsp_int_ack(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn42_enable_reg_inbox0_rsp_int(dmub: *mut dmub_srv, enable: bool);
}
extern "C" {
    pub fn dmub_dcn42_write_reg_outbox0_rdy_int_ack(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn42_read_reg_outbox0_msg(dmub: *mut dmub_srv, msg: *mut u32);
}
extern "C" {
    pub fn dmub_dcn42_write_reg_outbox0_rsp(dmub: *mut dmub_srv, rsp: *mut u32);
}
extern "C" {
    pub fn dmub_dcn42_read_reg_outbox0_rsp_int_status(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_enable_reg_outbox0_rdy_int(dmub: *mut dmub_srv, enable: bool);
}
extern "C" {
    pub fn dmub_dcn42_read_reg_outbox0_rdy_int_status(dmub: *mut dmub_srv) -> u32;
}
// GPINT operations
extern "C" {
    pub fn dmub_dcn42_set_gpint(dmub: *mut dmub_srv, reg: dmub_gpint_data_register);
}
extern "C" {
    pub fn dmub_dcn42_is_gpint_acked(dmub: *mut dmub_srv, reg: dmub_gpint_data_register) -> bool;
}
extern "C" {
    pub fn dmub_dcn42_get_gpint_response(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_get_gpint_dataout(dmub: *mut dmub_srv) -> u32;
}
// Status and detection
extern "C" {
    pub fn dmub_dcn42_is_hw_init(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn42_is_supported(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn42_is_hw_powered_up(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_dcn42_should_detect(dmub: *mut dmub_srv) -> bool;
}
// Firmware boot status and options
extern "C" {
    pub fn dmub_dcn42_get_fw_boot_status(dmub: *mut dmub_srv) -> dmub_fw_boot_status;
}
extern "C" {
    pub fn dmub_dcn42_get_fw_boot_option(dmub: *mut dmub_srv) -> dmub_fw_boot_options;
}
// Timing and diagnostics
extern "C" {
    pub fn dmub_dcn42_get_current_time(dmub: *mut dmub_srv) -> u32;
}
extern "C" {
    pub fn dmub_dcn42_get_diagnostic_data(dmub: *mut dmub_srv);
}
extern "C" {
    pub fn dmub_dcn42_get_preos_fw_info(dmub: *mut dmub_srv) -> bool;
}
