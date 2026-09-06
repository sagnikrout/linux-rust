//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_bios_types.h
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
// Copyright 2016 Advanced Micro Devices, Inc.
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
// Interface file for VBIOS implementations.
//
// The default implementation is inside DC.
// Display Manager (which instantiates DC) has the option to supply it's own
// (external to DC) implementation of VBIOS, which will be called by DC, using
// this interface.
// (The intended use is Diagnostics, but other uses may appear.)
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_vbios_funcs {
    pub bios): *mut *mut uint8_t (get_connectors_number)(struct dc_bios,
    pub connector_index): u8,
    pub src_object_id): *mut graphics_object_id,
    pub info): *mut graphics_object_i2c_info,
    pub info): *mut graphics_object_hpd_info,
    pub info): *mut connector_device_tag_info,
    pub ss_info): *mut spread_spectrum_info,
    pub signal): as_signal_type,
    pub info): *mut embedded_panel_info,
    pub info): *mut gpio_pin_info,
    pub info): *mut bp_encoder_cap_info,
    pub bios): *mut dc_bios,
    pub state): bool,
    pub id): device_id,
// COMMANDS
    pub bp_params): *mut bp_crtc_source_select,
    pub cntl): *mut bp_encoder_control,
    pub cntl): *mut bp_external_encoder_control,
    pub ext_enc_id): graphics_object_id,
    pub cntl): *mut bp_transmitter_control,
    pub enable): bool,
    pub bp_params): *mut bp_adjust_pixel_clock_parameters,
    pub bp_params): *mut bp_pixel_clock_parameters,
    pub bp_params): *mut bp_set_dce_clock_parameters,
    pub enable): bool,
    pub bp_params): *mut bp_hw_crtc_timing_parameters,
    pub bp_params): *mut bp_pixel_clock_parameters,
    pub action): bp_pipe_control_action,
    pub dcb): *mut *mut void (bios_parser_destroy)(struct dc_bios,
    pub board_layout_info): *mut board_layout_info,
    pub dst): *mut c_void,
    pub dcb): *mut dc_bios,
    pub bypass_panel_control_wait): u8,
    pub soc_bb_info): *mut bp_soc_bb_info,
    pub info): *mut bp_disp_connector_caps_info,
    pub dce_caps): *mut u8,
    pub dce_caps): *mut u8,
    pub info): *mut bp_connector_speed_cap_info,
    pub info): *mut graphics_object_i2c_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_registers {
    pub BIOS_SCRATCH_0: u32,
    pub BIOS_SCRATCH_3: u32,
    pub BIOS_SCRATCH_6: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_bios {
    pub funcs: *const dc_vbios_funcs,
    pub bios: *mut u8,
    pub bios_size: u32,
    pub bios_local_image: *mut u8,
    pub ctx: *mut dc_context,
    pub regs: *const bios_registers,
    pub integrated_info: *mut integrated_info,
    pub fw_info: dc_firmware_info,
    pub fw_info_valid: bool,
    pub vram_info: dc_vram_info,
    pub bb_info: bp_soc_bb_info,
    pub golden_table: dc_golden_table,
}
