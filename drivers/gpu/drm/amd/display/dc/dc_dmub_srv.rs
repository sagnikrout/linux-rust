//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_dmub_srv.h
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
// Copyright 2018 Advanced Micro Devices, Inc.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dmub_srv {
    pub dmub: *mut dmub_srv,
    pub ctx: *mut dc_context,
    pub dm: *mut c_void,
    pub idle_exit_counter: i32,
    pub driver_signals: dmub_shared_state_ips_driver_signals,
    pub idle_allowed: bool,
    pub needs_idle_wake: bool,
    pub cursor_offload_enabled: bool,
}

extern "C" {
    pub fn dc_dmub_srv_wait_for_pending(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_optimized_init_done(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_cmd_run(dc_dmub_srv: *mut dc_dmub_srv, cmd: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_cmd_run_list(dc_dmub_srv: *mut dc_dmub_srv, count: c_uint, cmd_list: *mut dmub_rb_cmd, wait_type: dm_dmub_wait_type) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_is_restore_required(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_get_dmub_outbox0_msg(dc: *const dc, entry: *mut dmcub_trace_buf_entry) -> bool;
}
extern "C" {
    pub fn dc_dmub_trace_event_control(dc: *mut dc, enable: bool);
}
extern "C" {
    pub fn dc_dmub_srv_drr_update_cmd(dc: *mut dc, tg_inst: u32, vtotal_min: u32, vtotal_max: u32);
}
extern "C" {
    pub fn dc_dmub_srv_set_drr_manual_trigger_cmd(dc: *mut dc, tg_inst: u32);
}
extern "C" {
    pub fn dc_dmub_srv_p_state_delegate(dc: *mut dc, enable_pstate: bool, context: *mut dc_state) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_query_caps_cmd(dc_dmub_srv: *mut dc_dmub_srv);
}
extern "C" {
    pub fn dc_dmub_srv_get_visual_confirm_color_cmd(dc: *mut dc, pipe_ctx: *mut pipe_ctx);
}
extern "C" {
    pub fn dc_dmub_srv_clear_inbox0_ack(dmub_srv: *mut dc_dmub_srv);
}
extern "C" {
    pub fn dc_dmub_srv_wait_for_inbox0_ack(dmub_srv: *mut dc_dmub_srv);
}
extern "C" {
    pub fn dc_dmub_srv_send_inbox0_cmd(dmub_srv: *mut dc_dmub_srv, data: dmub_inbox0_data_register);
}
extern "C" {
    pub fn dc_dmub_srv_get_diagnostic_data(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
}
extern "C" {
    pub fn dc_dmub_setup_subvp_dmub_command(dc: *mut dc, context: *mut dc_state, enable: bool);
}
extern "C" {
    pub fn dc_dmub_srv_log_diagnostic_data(dc_dmub_srv: *mut dc_dmub_srv);
}
extern "C" {
    pub fn dc_send_update_cursor_info_to_dmu(pCtx: *mut pipe_ctx, pipe_idx: u8);
}
extern "C" {
    pub fn dc_dmub_check_min_version(srv: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_enable_dpia_trace(dc: *const dc);
}
extern "C" {
    pub fn dc_dmub_srv_subvp_save_surf_addr(dc_dmub_srv: *const dc_dmub_srv, addr: *const dc_plane_address, subvp_index: u8);
}
extern "C" {
    pub fn dc_dmub_srv_is_hw_pwr_up(dc_dmub_srv: *mut dc_dmub_srv, wait: bool) -> bool;
}
extern "C" {
    pub fn dc_dmub_srv_apply_idle_power_optimizations(dc: *const dc, allow_idle: bool);
}
//
// dc_dmub_srv_set_power_state() - Sets the power state for DMUB service.
//
// Controls whether messaging the DMCUB or interfacing with it via HW register
// interaction is permittable.
//
// @dc_dmub_srv - The DC DMUB service pointer
// @power_state - the DC power state
//
extern "C" {
    pub fn dc_dmub_srv_set_power_state(dc_dmub_srv: *mut dc_dmub_srv, power_state: dc_acpi_cm_power_state);
}
//
// dc_dmub_srv_notify_fw_dc_power_state() - Notifies firmware of the DC power state.
//
// Differs from dc_dmub_srv_set_power_state in that it needs to access HW in order
// to message DMCUB of the state transition. Should come after the D0 exit and
// before D3 set power state.
//
// @dc_dmub_srv - The DC DMUB service pointer
// @power_state - the DC power state
//
// @dc_dmub_srv_should_detect() - Checks if link detection is required.
//
// While in idle power states we may need driver to manually redetect in
// the case of a missing hotplug. Should be called from a polling timer.
//
// Return: true if redetection is required.
//
extern "C" {
    pub fn dc_dmub_srv_should_detect(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
}
//
// dc_wake_and_execute_dmub_cmd() - Wrapper for DMUB command execution.
//
// Refer to dc_wake_and_execute_dmub_cmd_list() for usage and limitations,
// This function is a convenience wrapper for a single command execution.
//
// @ctx: DC context
// @cmd: The command to send/receive
// @wait_type: The wait behavior for the execution
//
// Return: true on command submission success, false otherwise
//
// dc_wake_and_execute_dmub_cmd_list() - Wrapper for DMUB command list execution.
//
// If the DMCUB hardware was asleep then it wakes the DMUB before
// executing the command and attempts to re-enter if the command
// submission was successful.
//
// This should be the preferred command submission interface provided
// the DC lock is acquired.
//
// Entry/exit out of idle power optimizations would need to be
// manually performed otherwise through dc_allow_idle_optimizations().
//
// @ctx: DC context
// @count: Number of commands to send/receive
// @cmd: Array of commands to send
// @wait_type: The wait behavior for the execution
//
// Return: true on command submission success, false otherwise
//
// dc_wake_and_execute_gpint()
//
// @ctx: DC context
// @command_code: The command ID to send to DMCUB
// @param: The parameter to message DMCUB
// @response: Optional response out value - may be NULL.
// @wait_type: The wait behavior for the execution
//
extern "C" {
    pub fn dmub_lsdma_init(dc_dmub_srv: *mut dc_dmub_srv) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_linear_copy_params {
    pub src_lo: u32,
    pub src_hi: u32,
    pub dst_lo: u32,
    pub dst_hi: u32,
    pub 30: uint32_t count :,
    pub 2: uint32_t read_compress :,
    pub 4: uint32_t tmz :,
    pub 3: uint32_t cache_policy_src :,
    pub 3: uint32_t cache_policy_dst :,
    pub 6: uint32_t data_format :,
    pub 3: uint32_t num_type :,
    pub 2: uint32_t write_compress :,
    pub 2: uint32_t max_com :,
    pub 1: uint32_t max_uncom :,
    pub 8: uint32_t reserved0 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_linear_sub_window_copy_params {
    pub src_lo: u32,
    pub src_hi: u32,
    pub dst_lo: u32,
    pub dst_hi: u32,
    pub 16: uint32_t src_x :,
    pub 16: uint32_t src_y :,
    pub 16: uint32_t dst_x :,
    pub 16: uint32_t dst_y :,
    pub 16: uint32_t rect_x :,
    pub 16: uint32_t rect_y :,
    pub 16: uint32_t src_pitch :,
    pub 16: uint32_t dst_pitch :,
    pub src_slice_pitch: u32,
    pub dst_slice_pitch: u32,
    pub 4: uint32_t tmz :,
    pub 3: uint32_t element_size :,
    pub 3: uint32_t src_cache_policy :,
    pub 3: uint32_t dst_cache_policy :,
    pub 6: uint32_t data_format :,
    pub 3: uint32_t num_type :,
    pub 2: uint32_t read_compress :,
    pub 2: uint32_t write_compress :,
    pub 2: uint32_t max_com :,
    pub 1: uint32_t max_uncom :,
    pub 3: uint32_t reserved0 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lsdma_send_tiled_to_tiled_copy_command_params {
    pub src_addr: u64,
    pub dst_addr: u64,
    pub 16: uint32_t src_x :,
    pub 16: uint32_t src_y :,
    pub 16: uint32_t dst_x :,
    pub 16: uint32_t dst_y :,
    pub 16: uint32_t src_width :,
    pub 16: uint32_t dst_width :,
    pub 16: uint32_t rect_x :,
    pub 16: uint32_t rect_y :,
    pub 16: uint32_t src_height :,
    pub 16: uint32_t dst_height :,
    pub 6: uint32_t data_format :,
    pub 5: uint32_t swizzle_mode :,
    pub 3: uint32_t element_size :,
    pub 1: uint32_t dcc :,
    pub 4: uint32_t tmz :,
    pub 2: uint32_t read_compress :,
    pub 2: uint32_t write_compress :,
    pub 2: uint32_t max_com :,
    pub 1: uint32_t max_uncom :,
    pub 3: uint32_t src_cache_policy :,
    pub 3: uint32_t dst_cache_policy :,
}

extern "C" {
    pub fn dmub_lsdma_send_poll_reg_write_command(dc_dmub_srv: *mut dc_dmub_srv, reg_addr: u32, reg_data: u32) -> bool;
}
//
// struct ips_residency_info - struct containing info from dmub_ips_residency_stats
//
// @ips_mode: The mode of IPS that the follow stats appertain to
// @residency_percent: The percentage of time spent in given IPS mode in millipercent
// @entry_counter: The number of entries made in to this IPS state
// @total_active_time_us: uint32_t array of length 2 representing time in the given IPS mode
// in microseconds. Index 0 is lower 32 bits, index 1 is upper 32 bits.
// @total_inactive_time_us: uint32_t array of length 2 representing time outside the given IPS mode
// in microseconds. Index 0 is lower 32 bits, index 1 is upper 32 bits.
// @histogram: Histogram of given IPS state durations - bucket definitions in dmub_ips.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ips_residency_info {
    pub ips_mode: ips_residency_mode,
    pub residency_percent: c_uint,
    pub entry_counter: c_uint,
    pub total_active_time_us: [c_uint; 2],
    pub total_inactive_time_us: [c_uint; 2],
    pub histogram: [c_uint; 16],
}

extern "C" {
    pub fn dc_dmub_srv_ips_residency_cntl(ctx: *const dc_context, panel_inst: u8, start_measurement: bool) -> bool;
}
//
// dc_dmub_srv_cursor_offload_init() - Enables or disables cursor offloading for a stream.
//
// @dc: pointer to DC object
//
extern "C" {
    pub fn dc_dmub_srv_cursor_offload_init(dc: *mut dc);
}
//
// dc_dmub_srv_control_cursor_offload() - Enables or disables cursor offloading for a stream.
//
// @dc: pointer to DC object
// @context: the DC context to reference for pipe allocations
// @stream: the stream to control
// @enable: true to enable cursor offload, false to disable
//
// dc_dmub_srv_program_cursor_now() - Requests immediate cursor programming for a given pipe.
//
// @dc: pointer to DC object
// @pipe: top-most pipe for a stream.
//
extern "C" {
    pub fn dc_dmub_srv_program_cursor_now(dc: *mut dc, pipe: *const pipe_ctx);
}
//
// dc_dmub_srv_is_cursor_offload_enabled() - Checks if cursor offload is supported.
//
// @dc: pointer to DC object
//
// Return: true if cursor offload is supported, false otherwise
//
extern "C" {
    pub fn dc_dmub_srv_is_cursor_offload_enabled(dc: *const dc) -> bool;
}
//
// dc_dmub_srv_boot_time_crc_init() - Initializes DMUB boot time CRC.
//
// @dc - pointer to DC object
// @gpu_addr - address for the boot time CRC buffer
// @size - size of the boot time CRC buffer
//
extern "C" {
    pub fn dc_dmub_srv_boot_time_crc_init(dc: *const dc, gpu_addr: u64, size: u32);
}
//
// dc_dmub_srv_release_hw() - Notifies DMUB service that HW access is no longer required.
//
// @dc - pointer to DC object
//
extern "C" {
    pub fn dc_dmub_srv_release_hw(dc: *const dc);
}
//
// dc_dmub_srv_log_preos_dmcub_info() - Logs preos dmcub fw info.
//
// @dc - pointer to DC object
//
extern "C" {
    pub fn dc_dmub_srv_log_preos_dmcub_info(dc_dmub_srv: *mut dc_dmub_srv);
}
//
// dc_dmub_srv_ihc_set_dig_hdcp_interrupt_dest() - Configure IHC interrupt destination.
// @dc_dmub_srv: DMUB service handle
// @dig_id: DIG engine ID (0-3)
// @to_dmu: Route interrupt to DMU (true) or not (false)
//
// Sends command to DMUB firmware to configure IHC interrupt routing
// for HDCP I2C transfer requests.
//
// Return: true on success, false on failure
//
// dc_dmub_srv_get_fams2_debug_meta() - Queries for FAMS2 debug metadata from DMUB and logs it.
//
// @dc_dmub_srv - pointer to DMUB service object
//
extern "C" {
    pub fn dc_dmub_srv_get_fams2_debug_meta(dc_dmub_srv: *mut dc_dmub_srv);
}
