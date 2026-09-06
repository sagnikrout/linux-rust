//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dmub/dmub_srv.h
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
// Copyright 2019-2026 Advanced Micro Devices, Inc.
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
// DOC: DMUB interface and operation
//
// DMUB is the interface to the display DMCUB microcontroller on DCN hardware.
// It delegates hardware initialization and command submission to the
// microcontroller. DMUB is the shortname for DMCUB.
//
// This interface is not thread-safe. Ensure that all access to the interface
// is properly synchronized by the caller.
//
// Initialization and usage of the DMUB service should be done in the
// steps given below:
//
// 1. dmub_srv_create()
// 2. dmub_srv_has_hw_support()
// 3. dmub_srv_calc_region_info()
// 4. dmub_srv_hw_init()
//
// The call to dmub_srv_create() is required to use the server.
//
// The calls to dmub_srv_has_hw_support() and dmub_srv_calc_region_info()
// are helpers to query cache window size and allocate framebuffer(s)
// for the cache windows.
//
// The call to dmub_srv_hw_init() programs the DMCUB registers to prepare
// for command submission. Commands can be queued via dmub_srv_fb_cmd_queue()
// and executed via dmub_srv_fb_cmd_execute().
//
// If the queue is full the dmub_srv_wait_for_idle() call can be used to
// wait until the queue has been cleared.
//
// Destroying the DMUB service can be done by calling dmub_srv_destroy().
// This does not clear DMUB hardware state, only software state.
//
// The interface is intended to be standalone and should not depend on any
// other component within DAL.
//

pub const DMUB_PC_SNAPSHOT_COUNT: c_int = 10;
// Default tracebuffer size if meta is absent.

pub const PSP_HEADER_BYTES_256: c_uint = 0x100 // 256 bytes;
pub const PSP_FOOTER_BYTES_256: c_uint = 0x100 // 256 bytes;
// Forward declarations
// enum dmub_window_memory_type - memory location type specification for windows
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_window_memory_type {
    DMUB_WINDOW_MEMORY_TYPE_FB = 0,
    DMUB_WINDOW_MEMORY_TYPE_GART
}

// enum dmub_status - return code for dmcub functions
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_status {
    DMUB_STATUS_OK = 0,
    DMUB_STATUS_NO_CTX,
    DMUB_STATUS_QUEUE_FULL,
    DMUB_STATUS_TIMEOUT,
    DMUB_STATUS_INVALID,
    DMUB_STATUS_HW_FAILURE,
    DMUB_STATUS_POWER_STATE_D3
}

// enum dmub_asic - dmub asic identifier
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_asic {
    DMUB_ASIC_NONE = 0,
    DMUB_ASIC_DCN20,
    DMUB_ASIC_DCN21,
    DMUB_ASIC_DCN30,
    DMUB_ASIC_DCN301,
    DMUB_ASIC_DCN302,
    DMUB_ASIC_DCN303,
    DMUB_ASIC_DCN31,
    DMUB_ASIC_DCN31B,
    DMUB_ASIC_DCN314,
    DMUB_ASIC_DCN315,
    DMUB_ASIC_DCN316,
    DMUB_ASIC_DCN32,
    DMUB_ASIC_DCN321,
    DMUB_ASIC_DCN35,
    DMUB_ASIC_DCN351,
    DMUB_ASIC_DCN36,
    DMUB_ASIC_DCN401,
    DMUB_ASIC_DCN42,
    DMUB_ASIC_DCN42B,
    DMUB_ASIC_DCN60,

    DMUB_ASIC_MAX,
}

// enum dmub_window_id - dmub window identifier
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_window_id {
    DMUB_WINDOW_0_INST_CONST = 0,
    DMUB_WINDOW_1_STACK,
    DMUB_WINDOW_2_BSS_DATA,
    DMUB_WINDOW_3_VBIOS,
    DMUB_WINDOW_4_MAILBOX,
    DMUB_WINDOW_5_TRACEBUFF,
    DMUB_WINDOW_6_FW_STATE,
    DMUB_WINDOW_7_SCRATCH_MEM,
    DMUB_WINDOW_IB_MEM,
    DMUB_WINDOW_SHARED_STATE,
    DMUB_WINDOW_LSDMA_BUFFER,
    DMUB_WINDOW_CURSOR_OFFLOAD,
    DMUB_WINDOW_TOTAL,
}

// enum dmub_notification_type - dmub outbox notification identifier
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_notification_type {
    DMUB_NOTIFICATION_NO_DATA = 0,
    DMUB_NOTIFICATION_AUX_REPLY,
    DMUB_NOTIFICATION_HPD,
    DMUB_NOTIFICATION_HPD_IRQ,
    DMUB_NOTIFICATION_SET_CONFIG_REPLY,
    DMUB_NOTIFICATION_DPIA_NOTIFICATION,
    DMUB_NOTIFICATION_HPD_SENSE_NOTIFY,
    DMUB_NOTIFICATION_FUSED_IO,
    DMUB_NOTIFICATION_MAX
}

//
// DPIA NOTIFICATION Response Type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpia_notify_bw_alloc_status {

    DPIA_BW_REQ_FAILED = 0,
    DPIA_BW_REQ_SUCCESS,
    DPIA_EST_BW_CHANGED,
    DPIA_BW_ALLOC_CAPS_CHANGED
}

// enum dmub_memory_access_type - memory access method
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_memory_access_type {
    DMUB_MEMORY_ACCESS_DEFAULT,
    DMUB_MEMORY_ACCESS_CPU = DMUB_MEMORY_ACCESS_DEFAULT,
    DMUB_MEMORY_ACCESS_DMA
}

// enum dmub_power_state type - to track DC power state in dmub_srv
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_srv_power_state_type {
    DMUB_POWER_STATE_UNDEFINED = 0,
    DMUB_POWER_STATE_D0 = 1,
    DMUB_POWER_STATE_D3 = 8
}

// enum dmub_inbox_cmd_interface type - defines default interface for host->dmub commands
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dmub_inbox_cmd_interface_type {
    DMUB_CMD_INTERFACE_DEFAULT = 0,
    DMUB_CMD_INTERFACE_FB = 1,
    DMUB_CMD_INTERFACE_REG = 2,
}

//
// struct dmub_region - dmub hw memory region
// @base: base address for region, must be 256 byte aligned
// @top: top address for region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_region {
    pub base: u32,
    pub top: u32,
}

//
// struct dmub_window - dmub hw cache window
// @off: offset to the fb memory in gpu address space
// @r: region in uc address space for cache window
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_window {
    pub offset: dmub_addr,
    pub region: dmub_region,
}

//
// struct dmub_fb - defines a dmub framebuffer memory region
// @cpu_addr: cpu virtual address for the region, NULL if invalid
// @gpu_addr: gpu virtual address for the region, NULL if invalid
// @size: size of the region in bytes, zero if invalid
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_fb {
    pub cpu_addr: *mut c_void,
    pub gpu_addr: u64,
    pub size: u32,
}

//
// struct dmub_srv_region_params - params used for calculating dmub regions
// @inst_const_size: size of the fw inst const section
// @bss_data_size: size of the fw bss data section
// @vbios_size: size of the vbios data
// @fw_bss_data: raw firmware bss data section
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_region_params {
    pub inst_const_size: u32,
    pub bss_data_size: u32,
    pub vbios_size: u32,
    pub fw_inst_const: *const u8,
    pub fw_bss_data: *const u8,
    pub window_memory_type: *const dmub_window_memory_type,
    pub fw_info: *const dmub_fw_meta_info,
}

//
// struct dmub_srv_fw_meta_info_params - params used for fetching fw meta info from fw_image
// @inst_const_size: size of the fw inst const section
// @bss_data_size: size of the fw bss data section
// @fw_inst_const: raw firmware inst const section
// @fw_bss_data: raw firmware bss data section
// @custom_psp_footer_size: custom psp footer size to use when indexing for fw meta info
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_fw_meta_info_params {
    pub inst_const_size: u32,
    pub bss_data_size: u32,
    pub fw_inst_const: *const u8,
    pub fw_bss_data: *const u8,
    pub custom_psp_footer_size: u32,
}

//
// struct dmub_srv_region_info - output region info from the dmub service
// @fb_size: required minimum fb size for all regions, aligned to 4096 bytes
// @num_regions: number of regions used by the dmub service
// @regions: region info
//
// The regions are aligned such that they can be all placed within the
// same framebuffer but they can also be placed into different framebuffers.
//
// The size of each region can be calculated by the caller:
// size = reg.top - reg.base
//
// Care must be taken when performing custom allocations to ensure that each
// region base address is 256 byte aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_region_info {
    pub fb_size: u32,
    pub gart_size: u32,
    pub num_regions: u8,
    pub regions: [dmub_region; DMUB_WINDOW_TOTAL],
    pub verified_psp_footer_size: u32,
}

//
// struct dmub_srv_memory_params - parameters used for driver fb setup
// @region_info: region info calculated by dmub service
// @cpu_fb_addr: base cpu address for the framebuffer
// @cpu_inbox_addr: base cpu address for the gart
// @gpu_fb_addr: base gpu virtual address for the framebuffer
// @gpu_inbox_addr: base gpu virtual address for the gart
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_memory_params {
    pub region_info: *const dmub_srv_region_info,
    pub cpu_fb_addr: *mut c_void,
    pub cpu_gart_addr: *mut c_void,
    pub gpu_fb_addr: u64,
    pub gpu_gart_addr: u64,
    pub window_memory_type: *const dmub_window_memory_type,
}

//
// struct dmub_srv_fb_info - output fb info from the dmub service
// @num_fbs: number of required dmub framebuffers
// @fbs: fb data for each region
//
// Output from the dmub service helper that can be used by the
// driver to prepare dmub_fb that can be passed into the dmub
// hw init service.
//
// Assumes that all regions are within the same framebuffer
// and have been setup according to the region_info generated
// by the dmub service.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_fb_info {
    pub num_fb: u8,
    pub fb: [dmub_fb; DMUB_WINDOW_TOTAL],
}

//
// struct dmub_soc_fb_info - relevant addresses from the frame buffer
// @fb_base: base of the framebuffer aperture
// @fb_offset: offset of the framebuffer aperture
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_soc_fb_info {
    pub fb_base: u64,
    pub fb_offset: u64,
    pub alt_channel_region_size: [u32; 2],
    pub /: *mut *mut uint64_t alt_channel_region_base[2]; / mc address,
}

//
// struct dmub_srv_hw_params - params for dmub hardware initialization
// @fb: framebuffer info for each region
// @fb_base: base of the framebuffer aperture
// @fb_offset: offset of the framebuffer aperture
// @psp_version: psp version to pass for DMCU init
// @load_inst_const: true if DMUB should load inst const fw
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_hw_params {
    pub fb_info: *mut dmub_srv_fb_info,
    pub soc_fb_info: dmub_soc_fb_info,
    pub psp_version: u32,
    pub load_inst_const: bool,
    pub skip_panel_power_sequence: bool,
    pub disable_z10: bool,
    pub power_optimization: bool,
    pub dpia_supported: bool,
    pub disable_dpia: bool,
    pub usb4_cm_version: bool,
    pub fw_in_system_memory: bool,
    pub dpia_hpd_int_enable_supported: bool,
    pub disable_clock_gate: bool,
    pub disallow_dispclk_dppclk_ds: bool,
    pub ips_sequential_ono: bool,
    pub mem_access_type: dmub_memory_access_type,
    pub disable_ips: dmub_ips_disable_type,
    pub disallow_phy_access: bool,
    pub disable_sldo_opt: bool,
    pub enable_non_transparent_setconfig: bool,
    pub lower_hbr3_phy_ssc: bool,
    pub override_hbr3_pll_vco: bool,
    pub disable_dpia_bw_allocation: bool,
}

//
// struct dmub_srv_debug - Debug info for dmub_srv
// @timeout_occured: Indicates a timeout occured on any message from driver to dmub
// @timeout_cmd: first cmd sent from driver that timed out - subsequent timeouts are not stored
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_timeout_info {
    pub timeout_occured: bool,
    pub timeout_cmd: dmub_rb_cmd,
    pub timestamp: c_ulonglong,
}

//
// struct dmub_diagnostic_data - Diagnostic data retrieved from DMCUB for
// debugging purposes, including logging, crash analysis, etc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_diagnostic_data {
    pub dmcub_version: u32,
    pub scratch: [u32; 17],
    pub pc: [u32; DMUB_PC_SNAPSHOT_COUNT],
    pub undefined_address_fault_addr: u32,
    pub inst_fetch_fault_addr: u32,
    pub data_write_fault_addr: u32,
    pub inbox1_rptr: u32,
    pub inbox1_wptr: u32,
    pub inbox1_size: u32,
    pub inbox0_rptr: u32,
    pub inbox0_wptr: u32,
    pub inbox0_size: u32,
    pub outbox1_rptr: u32,
    pub outbox1_wptr: u32,
    pub outbox1_size: u32,
    pub gpint_datain0: u32,
    pub timeout_info: dmub_timeout_info,
    pub 1: uint8_t is_dmcub_enabled :,
    pub 1: uint8_t is_dmcub_soft_reset :,
    pub 1: uint8_t is_dmcub_secure_reset :,
    pub 1: uint8_t is_traceport_en :,
    pub 1: uint8_t is_cw0_enabled :,
    pub 1: uint8_t is_cw6_enabled :,
    pub 1: uint8_t is_pwait :,
}

//
// struct dmub_preos_info - preos fw info before loading post os fw.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_preos_info {
    pub fb_base: u64,
    pub fb_offset: u64,
    pub trace_buffer_phy_addr: u64,
    pub trace_buffer_size: u32,
    pub fw_version: u32,
    pub boot_status: u32,
    pub boot_options: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_inbox {
// generic status
    pub num_submitted: u64,
    pub num_reported: u64,
// frame buffer mailbox status
    pub rb: dmub_rb,
// register mailbox status
    pub is_pending: bool,
    pub is_multi_pending: bool,
}

//
// struct dmub_srv_base_funcs - Driver specific base callbacks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_base_funcs {
//
// @reg_read:
//
// Hook for reading a register.
//
// Return: The 32-bit register value from the given address.
//
    pub address): *mut *mut *mut uint32_t (reg_read)(void ctx, uint32_t,
//
// @reg_write:
//
// Hook for writing a value to the register specified by address.
//
    pub value): *mut *mut *mut void (reg_write)(void ctx, uint32_t address, uint32_t,
}

//
// struct dmub_srv_hw_funcs - hardware sequencer funcs for dmub
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_hw_funcs {
// private: internal use only
    pub dmub): *mut *mut void (init)(struct dmub_srv,
    pub dmub): *mut *mut void (reset)(struct dmub_srv,
    pub dmub): *mut *mut void (reset_release)(struct dmub_srv,
    pub cw1): *const dmub_window,
    pub cw1): *const dmub_window,
    pub region6): *const dmub_window,
    pub inbox1): *const dmub_region,
    pub dmub): *mut *mut uint32_t (get_inbox1_wptr)(struct dmub_srv,
    pub dmub): *mut *mut uint32_t (get_inbox1_rptr)(struct dmub_srv,
    pub wptr_offset): *mut *mut *mut void (set_inbox1_wptr)(struct dmub_srv dmub, uint32_t,
    pub outbox1): *const dmub_region,
    pub dmub): *mut *mut uint32_t (get_outbox1_wptr)(struct dmub_srv,
    pub rptr_offset): *mut *mut *mut void (set_outbox1_rptr)(struct dmub_srv dmub, uint32_t,
    pub outbox0): *const dmub_region,
    pub dmub): *mut *mut uint32_t (get_outbox0_wptr)(struct dmub_srv,
    pub rptr_offset): *mut *mut *mut void (set_outbox0_rptr)(struct dmub_srv dmub, uint32_t,
    pub dmub): *mut *mut uint32_t (emul_get_inbox1_rptr)(struct dmub_srv,
    pub dmub): *mut *mut uint32_t (emul_get_inbox1_wptr)(struct dmub_srv,
    pub wptr_offset): *mut *mut *mut void (emul_set_inbox1_wptr)(struct dmub_srv dmub, uint32_t,
    pub dmub): *mut *mut bool (is_supported)(struct dmub_srv,
    pub dmub): *mut *mut bool (is_psrsu_supported)(struct dmub_srv,
    pub dmub): *mut *mut bool (is_hw_init)(struct dmub_srv,
    pub dmub): *mut *mut bool (is_hw_powered_up)(struct dmub_srv,
    pub params): *const dmub_srv_hw_params,
    pub skip): *mut *mut *mut void (skip_dmub_panel_power_sequence)(struct dmub_srv dmub, bool,
    pub dmub): *mut *mut dmub_fw_boot_status (get_fw_status)(struct dmub_srv,
    pub dmub): *mut *mut dmub_fw_boot_options (get_fw_boot_option)(struct dmub_srv,
    pub reg): dmub_gpint_data_register,
    pub reg): dmub_gpint_data_register,
    pub dmub): *mut *mut uint32_t (get_gpint_response)(struct dmub_srv,
    pub dmub): *mut *mut uint32_t (get_gpint_dataout)(struct dmub_srv,
    pub dmub): *mut *mut void (configure_dmub_in_system_memory)(struct dmub_srv,
    pub dmub): *mut *mut void (clear_inbox0_ack_register)(struct dmub_srv,
    pub dmub): *mut *mut uint32_t (read_inbox0_ack_register)(struct dmub_srv,
    pub data): *mut *mut *mut void (send_inbox0_cmd)(struct dmub_srv dmub, union dmub_inbox0_data_register,
    pub dmub): *mut *mut uint32_t (get_current_time)(struct dmub_srv,
    pub dmub): *mut *mut void (get_diagnostic_data)(struct dmub_srv,
    pub dmub): *mut *mut bool (get_preos_fw_info)(struct dmub_srv,
    pub dmub): *mut *mut bool (should_detect)(struct dmub_srv,
    pub ctx): *mut *mut *mut void (init_reg_offsets)(struct dmub_srv dmub, struct dc_context,
    pub subvp_index): *const *const *const *const void (subvp_save_surf_addr)(struct dmub_srv dmub, struct dc_plane_address addr, uint8_t,
    pub cmd): *mut dmub_rb_cmd,
    pub dmub): *mut *mut uint32_t (read_reg_inbox0_rsp_int_status)(struct dmub_srv,
    pub cmd): *mut dmub_rb_cmd,
    pub dmub): *mut *mut void (write_reg_inbox0_rsp_int_ack)(struct dmub_srv,
    pub dmub): *mut *mut void (clear_reg_inbox0_rsp_int_ack)(struct dmub_srv,
    pub enable): *mut *mut *mut void (enable_reg_inbox0_rsp_int)(struct dmub_srv dmub, bool,
    pub dmub): *mut *mut uint32_t (read_reg_outbox0_rdy_int_status)(struct dmub_srv,
    pub dmub): *mut *mut void (write_reg_outbox0_rdy_int_ack)(struct dmub_srv,
    pub msg): *mut *mut *mut void (read_reg_outbox0_msg)(struct dmub_srv dmub, uint32_t,
    pub rsp): *mut *mut *mut void (write_reg_outbox0_rsp)(struct dmub_srv dmub, uint32_t,
    pub dmub): *mut *mut uint32_t (read_reg_outbox0_rsp_int_status)(struct dmub_srv,
    pub enable): *mut *mut *mut void (enable_reg_outbox0_rdy_int)(struct dmub_srv dmub, bool,
}

//
// struct dmub_srv_create_params - params for dmub service creation
// @base_funcs: driver supplied base routines
// @hw_funcs: optional overrides for hw funcs
// @user_ctx: context data for callback funcs
// @asic: driver supplied asic
// @fw_version: the current firmware version, if any
// @is_virtual: false for hw support only
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv_create_params {
    pub funcs: dmub_srv_base_funcs,
    pub hw_funcs: *mut dmub_srv_hw_funcs,
    pub user_ctx: *mut c_void,
    pub asic: dmub_asic,
    pub fw_version: u32,
    pub is_virtual: bool,
    pub inbox_type: dmub_inbox_cmd_interface_type,
}

//
// struct dmub_srv - software state for dmcub
// @asic: dmub asic identifier
// @user_ctx: user provided context for the dmub_srv
// @fw_version: the current firmware version, if any
// @is_virtual: false if hardware support only
// @shared_state: dmub shared state between firmware and driver
// @cursor_offload_v1: Cursor offload state
// @fw_state: dmub firmware state pointer (debug purpose only)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_srv {
    pub asic: dmub_asic,
    pub user_ctx: *mut c_void,
    pub fw_version: u32,
    pub is_virtual: bool,
    pub no_ext_reg_access: bool,
    pub scratch_mem_fb: dmub_fb,
    pub ib_mem_gart: dmub_fb,
    pub cursor_offload_fb: dmub_fb,
    pub fb_info: *const dmub_srv_fb_info,
    pub shared_state: *mut volatile struct dmub_shared_state_feature_block,
    pub cursor_offload_v1: *mut volatile struct dmub_cursor_offload_v1,
    pub fw_state: *const volatile struct dmub_fw_state,
// private: internal use only
    pub regs: *const dmub_srv_common_regs,
    pub regs_dcn31: *const dmub_srv_dcn31_regs,
    pub regs_dcn32: *mut dmub_srv_dcn32_regs,
    pub regs_dcn35: *mut dmub_srv_dcn35_regs,
    pub regs_dcn401: *const dmub_srv_dcn401_regs,
    pub regs_dcn42: *mut dmub_srv_dcn42_regs,
    pub regs_dcn60: *const dmub_srv_dcn60_regs,
    pub funcs: dmub_srv_base_funcs,
    pub hw_funcs: dmub_srv_hw_funcs,
    pub inbox1: dmub_srv_inbox,
    pub inbox1_last_wptr: u32,
    pub reg_inbox0: dmub_srv_inbox,
//
// outbox1_rb is accessed without locks (dal & dc)
// and to be used only in dmub_srv_stat_get_notification()
//
    pub outbox1_rb: dmub_rb,
    pub outbox0_rb: dmub_rb,
    pub sw_init: bool,
    pub hw_init: bool,
    pub dpia_supported: bool,
    pub soc_fb_info: dmub_soc_fb_info,
    pub psp_version: u32,
// Feature capabilities reported by fw
    pub meta_info: dmub_fw_meta_info,
    pub feature_caps: dmub_feature_caps,
    pub visual_confirm_color: dmub_visual_confirm_color,
    pub inbox_type: dmub_inbox_cmd_interface_type,
    pub power_state: dmub_srv_power_state_type,
    pub debug: dmub_diagnostic_data,
    pub lsdma_rb_fb: dmub_fb,
    pub preos_info: dmub_preos_info,
}

//
// struct dmub_notification - dmub notification data
// @type: dmub notification type
// @link_index: link index to identify aux connection
// @result: USB4 status returned from dmub
// @pending_notification: Indicates there are other pending notifications
// @aux_reply: aux reply
// @hpd_status: hpd status
// @bw_alloc_reply: BW Allocation reply from CM/DPIA
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_notification {
    pub type: dmub_notification_type,
    pub link_index: u8,
    pub result: u8,
// notify instance from DMUB
    pub instance: u8,
    pub pending_notification: bool,
    pub aux_reply: aux_reply_data,
    pub hpd_status: dp_hpd_status,
    pub sc_status: set_config_status,
    pub hpd_sense_notify: dmub_rb_cmd_hpd_sense_notify_data,
    pub fused_request: dmub_cmd_fused_request,
}

//
// DMUB firmware version helper macro - useful for checking if the version
// of a firmware to know if feature or functionality is supported or present.
//

//
// dmub_srv_create() - creates the DMUB service.
// @dmub: the dmub service
// @params: creation parameters for the service
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_destroy() - destroys the DMUB service.
// @dmub: the dmub service
//
extern "C" {
    pub fn dmub_srv_destroy(dmub: *mut dmub_srv);
}
//
// dmub_srv_calc_region_info() - retreives region info from the dmub service
// @dmub: the dmub service
// @params: parameters used to calculate region locations
// @info_out: the output region info from dmub
//
// Calculates the base and top address for all relevant dmub regions
// using the parameters given (if any).
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_calc_region_info() - retreives fb info from the dmub service
// @dmub: the dmub service
// @params: parameters used to calculate fb locations
// @info_out: the output fb info from dmub
//
// Calculates the base and top address for all relevant dmub regions
// using the parameters given (if any).
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_has_hw_support() - returns hw support state for dmcub
// @dmub: the dmub service
// @is_supported: hw support state
//
// Queries the hardware for DMCUB support and returns the result.
//
// Can be called before dmub_srv_hw_init().
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_is_hw_init() - returns hardware init state
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
extern "C" {
    pub fn dmub_srv_is_hw_init(dmub: *mut dmub_srv, is_hw_init: *mut bool) -> dmub_status;
}
//
// dmub_srv_hw_init() - initializes the underlying DMUB hardware
// @dmub: the dmub service
// @params: params for hardware initialization
//
// Resets the DMUB hardware and performs backdoor loading of the
// required cache regions based on the input framebuffer regions.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_NO_CTX - dmcub context not initialized
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_hw_reset() - puts the DMUB hardware in reset state if initialized
// @dmub: the dmub service
//
// Before destroying the DMUB service or releasing the backing framebuffer
// memory we'll need to put the DMCUB into reset first.
//
// A subsequent call to dmub_srv_hw_init() will re-enable the DMCUB.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
extern "C" {
    pub fn dmub_srv_hw_reset(dmub: *mut dmub_srv) -> dmub_status;
}
//
// dmub_srv_fb_cmd_queue() - queues a command to the DMUB
// @dmub: the dmub service
// @cmd: the command to queue
//
// Queues a command to the DMUB service but does not begin execution
// immediately.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_QUEUE_FULL - no remaining room in queue
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_fb_cmd_execute() - Executes a queued sequence to the dmub
// @dmub: the dmub service
//
// Begins execution of queued commands on the dmub.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
extern "C" {
    pub fn dmub_srv_fb_cmd_execute(dmub: *mut dmub_srv) -> dmub_status;
}
//
// dmub_srv_wait_for_hw_pwr_up() - Waits for firmware hardware power up is completed
// @dmub: the dmub service
// @timeout_us: the maximum number of microseconds to wait
//
// Waits until firmware hardware is powered up. The maximum
// wait time is given in microseconds to prevent spinning forever.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - timed out
// DMUB_STATUS_INVALID - unspecified error
//
extern "C" {
    pub fn dmub_srv_is_hw_pwr_up(dmub: *mut dmub_srv) -> bool;
}
//
// dmub_srv_wait_for_auto_load() - Waits for firmware auto load to complete
// @dmub: the dmub service
// @timeout_us: the maximum number of microseconds to wait
//
// Waits until firmware has been autoloaded by the DMCUB. The maximum
// wait time is given in microseconds to prevent spinning forever.
//
// On ASICs without firmware autoload support this function will return
// immediately.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - wait for phy init timed out
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_wait_for_phy_init() - Waits for DMUB PHY init to complete
// @dmub: the dmub service
// @timeout_us: the maximum number of microseconds to wait
//
// Waits until the PHY has been initialized by the DMUB. The maximum
// wait time is given in microseconds to prevent spinning forever.
//
// On ASICs without PHY init support this function will return
// immediately.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - wait for phy init timed out
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_wait_for_pending() - Re-entrant wait for messages currently pending
// @dmub: the dmub service
// @timeout_us: the maximum number of microseconds to wait
//
// Waits until the commands queued prior to this call are complete.
// If interfaces remain busy due to additional work being submitted
// concurrently, this function will not continue to wait.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - wait for buffer to flush timed out
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_wait_for_idle() - Waits for the DMUB to be idle
// @dmub: the dmub service
// @timeout_us: the maximum number of microseconds to wait
//
// Waits until the DMUB buffer is empty and all commands have
// finished processing. The maximum wait time is given in
// microseconds to prevent spinning forever.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - wait for buffer to flush timed out
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_send_gpint_command() - Sends a GPINT based command.
// @dmub: the dmub service
// @command_code: the command code to send
// @param: the command parameter to send
// @timeout_us: the maximum number of microseconds to wait
//
// Sends a command via the general purpose interrupt (GPINT).
// Waits for the number of microseconds specified by timeout_us
// for the command ACK before returning.
//
// Can be called after software initialization.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - wait for ACK timed out
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_get_gpint_response() - Queries the GPINT response.
// @dmub: the dmub service
// @response: the response for the last GPINT
//
// Returns the response code for the last GPINT interrupt.
//
// Can be called after software initialization.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_get_gpint_dataout() - Queries the GPINT DATAOUT.
// @dmub: the dmub service
// @dataout: the data for the GPINT DATAOUT
//
// Returns the response code for the last GPINT DATAOUT interrupt.
//
// Can be called after software initialization.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_flush_buffer_mem() - Read back entire frame buffer region.
// This ensures that the write from x86 has been flushed and will not
// hang the DMCUB.
// @dmub: the dmub service
// @fb: frame buffer to flush
//
// Can be called after software initialization.
//
extern "C" {
    pub fn dmub_srv_flush_buffer_mem(dmub: *mut dmub_srv, fb: *const dmub_fb);
}
//
// dmub_srv_get_fw_boot_status() - Returns the DMUB boot status bits.
//
// @dmub: the dmub service
// @status: out pointer for firmware status
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error, unsupported
//
extern "C" {
    pub fn dmub_srv_get_outbox0_msg(dmub: *mut dmub_srv, entry: *mut dmcub_trace_buf_entry) -> bool;
}
extern "C" {
    pub fn dmub_srv_get_diagnostic_data(dmub: *mut dmub_srv) -> bool;
}
extern "C" {
    pub fn dmub_srv_should_detect(dmub: *mut dmub_srv) -> bool;
}
//
// dmub_srv_send_inbox0_cmd() - Send command to DMUB using INBOX0
// @dmub: the dmub service
// @data: the data to be sent in the INBOX0 command
//
// Send command by writing directly to INBOX0 WPTR
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - hw_init false or hw function does not exist
//
extern "C" {
    pub fn dmub_srv_send_inbox0_cmd(dmub: *mut dmub_srv, data: dmub_inbox0_data_register) -> dmub_status;
}
//
// dmub_srv_wait_for_inbox0_ack() - wait for DMUB to ACK INBOX0 command
// @dmub: the dmub service
// @timeout_us: the maximum number of microseconds to wait
//
// Wait for DMUB to ACK the INBOX0 message
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - hw_init false or hw function does not exist
// DMUB_STATUS_TIMEOUT - wait for ack timed out
//
extern "C" {
    pub fn dmub_srv_wait_for_inbox0_ack(dmub: *mut dmub_srv, timeout_us: u32) -> dmub_status;
}
//
// dmub_srv_wait_for_inbox0_ack() - clear ACK register for INBOX0
// @dmub: the dmub service
//
// Clear ACK register for INBOX0
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - hw_init false or hw function does not exist
//
extern "C" {
    pub fn dmub_srv_clear_inbox0_ack(dmub: *mut dmub_srv) -> dmub_status;
}
//
// dmub_srv_subvp_save_surf_addr() - Save primary and meta address for subvp on each flip
// @dmub: The dmub service
// @addr: The surface address to be programmed on the current flip
// @subvp_index: Index of subvp pipe, indicates which subvp pipe the address should be saved for
//
// Function to save the surface flip addr into scratch registers. This is to fix a race condition
// between FW and driver reading / writing to the surface address at the same time. This is
// required because there is no EARLIEST_IN_USE_META.
//
// Return:
// void
//
extern "C" {
    pub fn dmub_srv_subvp_save_surf_addr(dmub: *mut dmub_srv, addr: *const dc_plane_address, subvp_index: u8);
}
//
// dmub_srv_set_power_state() - Track DC power state in dmub_srv
// @dmub: The dmub service
// @power_state: DC power state setting
//
// Store DC power state in dmub_srv.  If dmub_srv is in D3, then don't send messages to DMUB
//
// Return:
// void
//
extern "C" {
    pub fn dmub_srv_set_power_state(dmub: *mut dmub_srv, dmub_srv_power_state: dmub_srv_power_state_type);
}
//
// dmub_srv_reg_cmd_execute() - Executes provided command to the dmub
// @dmub: the dmub service
// @cmd: the command packet to be executed
//
// Executes a single command for the dmub.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
extern "C" {
    pub fn dmub_srv_reg_cmd_execute(dmub: *mut dmub_srv, cmd: *mut dmub_rb_cmd) -> dmub_status;
}
//
// dmub_srv_cmd_get_response() - Copies return data for command into buffer
// @dmub: the dmub service
// @cmd_rsp: response buffer
//
// Copies return data for command into buffer
//
// dmub_srv_sync_inboxes() - Sync inbox state
// @dmub: the dmub service
//
// Sync inbox state
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - unspecified error
//
extern "C" {
    pub fn dmub_srv_sync_inboxes(dmub: *mut dmub_srv) -> dmub_status;
}
//
// dmub_srv_wait_for_inbox_free() - Waits for space in the DMUB inbox to free up
// @dmub: the dmub service
// @timeout_us: the maximum number of microseconds to wait
// @num_free_required: number of free entries required
//
// Waits until the DMUB buffer is freed to the specified number.
// The maximum wait time is given in microseconds to prevent spinning
// forever.
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - wait for buffer to flush timed out
// DMUB_STATUS_INVALID - unspecified error
//
// dmub_srv_update_inbox_status() - Updates pending status for inbox & reg inbox0
// @dmub: the dmub service
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_TIMEOUT - wait for buffer to flush timed out
// DMUB_STATUS_HW_FAILURE - issue with HW programming
// DMUB_STATUS_INVALID - unspecified error
//
extern "C" {
    pub fn dmub_srv_update_inbox_status(dmub: *mut dmub_srv) -> dmub_status;
}
//
// dmub_srv_get_preos_info() - retrieves preos fw info
// @dmub: the dmub service
//
// Return:
// true - preos fw info retrieved successfully
// false - preos fw info not retrieved successfully
//
extern "C" {
    pub fn dmub_srv_get_preos_info(dmub: *mut dmub_srv) -> bool;
}
//
// dmub_srv_get_fw_meta_info_from_raw_fw() - Fetch firmware metadata info from raw firmware image
// @params: parameters for fetching firmware metadata info
// @fw_info_out: output buffer for firmware metadata info
//
// Return:
// DMUB_STATUS_OK - success
// DMUB_STATUS_INVALID - no FW meta info found
//
