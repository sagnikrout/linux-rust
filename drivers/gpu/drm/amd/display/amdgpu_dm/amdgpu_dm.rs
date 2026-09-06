//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/amdgpu_dm/amdgpu_dm.h
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
// Copyright (C) 2015-2020 Advanced Micro Devices, Inc. All rights reserved.
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
// This file contains the definition for amdgpu_display_manager
// and its API for amdgpu driver's use.
// This component provides all the display related functionality
// and this is the only component that calls DAL API.
// The API contained here intended for amdgpu driver use.
// The API that is called directly from KMS framework is located
// in amdgpu_dm_kms.h file
//
pub const AMDGPU_DM_MAX_CRTC: c_int = 6;
pub const AMDGPU_DM_MAX_NUM_EDP: c_int = 2;
pub const AMDGPU_DMUB_NOTIFICATION_MAX: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amd_vsdb_panel_type {
    AMD_VSDB_PANEL_TYPE_DEFAULT = 0,
    AMD_VSDB_PANEL_TYPE_MINILED,
    AMD_VSDB_PANEL_TYPE_OLED,
    AMD_VSDB_PANEL_TYPE_RESERVED,
}

//
// Maximum HDMI HPD debounce delay in milliseconds
//
pub const AMDGPU_DM_MAX_HDMI_HPD_DEBOUNCE_MS: c_int = 5000;
//

//

// Forward declarations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct common_irq_params {
    pub adev: *mut amdgpu_device,
    pub irq_src: dc_irq_source,
    pub previous_timestamp: core::sync::atomic::AtomicI64,
}

//
// struct dm_compressor_info - Buffer info used by frame buffer compression
// @cpu_addr: MMIO cpu addr
// @bo_ptr: Pointer to the buffer object
// @gpu_addr: MMIO gpu addr
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_compressor_info {
    pub cpu_addr: *mut c_void,
    pub bo_ptr: *mut amdgpu_bo,
    pub gpu_addr: u64,
}

//
// struct dm_boot_time_crc_info - Buffer info used by boot time CRC
// @cpu_addr: MMIO cpu addr
// @bo_ptr: Pointer to the buffer object
// @gpu_addr: MMIO gpu addr
// @size: Size of the buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_boot_time_crc_info {
    pub cpu_addr: *mut c_void,
    pub bo_ptr: *mut amdgpu_bo,
    pub gpu_addr: u64,
    pub size: u32,
}

extern "C" {
    pub fn void(adev: *mut *mut dmub_notify_interrupt_callback_t)(struct amdgpu_device, notify: *mut dmub_notification) -> typedef;
}
//
// struct dmub_hpd_work - Handle time consuming work in low priority outbox IRQ
//
// @handle_hpd_work: Work to be executed in a separate thread to handle hpd_low_irq
// @dmub_notify:  notification for callback function
// @adev: amdgpu_device pointer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmub_hpd_work {
    pub handle_hpd_work: work_struct,
    pub dmub_notify: *mut dmub_notification,
    pub adev: *mut amdgpu_device,
}

//
// struct vblank_control_work - Work data for vblank control
// @work: Kernel work data for the work event
// @dm: amdgpu display manager device
// @acrtc: amdgpu CRTC instance for which the event has occurred
// @stream: DC stream for which the event has occurred
// @enable: true if enabling vblank
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vblank_control_work {
    pub work: work_struct,
    pub dm: *mut amdgpu_display_manager,
    pub acrtc: *mut amdgpu_crtc,
    pub stream: *mut dc_stream_state,
    pub enable: bool,
}

//
// struct idle_workqueue - Work data for periodic action in idle
// @work: Kernel work data for the work event
// @dm: amdgpu display manager device
// @enable: true if idle worker is enabled
// @running: true if idle worker is running
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct idle_workqueue {
    pub work: work_struct,
    pub dm: *mut amdgpu_display_manager,
    pub enable: bool,
    pub running: bool,
}

//
// struct vupdate_offload_work - Work data for offloading task from vupdate handler
// @work: Kernel work data for the work event
// @adev: amdgpu_device back pointer
// @stream: DC stream associated with the crtc
// @adjust: DC CRTC timing adjust to be applied to the crtc
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vupdate_offload_work {
    pub work: work_struct,
    pub adev: *mut amdgpu_device,
    pub stream: *mut dc_stream_state,
    pub adjust: *mut dc_crtc_timing_adjust,
}

pub const MAX_LUMINANCE_DATA_POINTS: c_int = 99;
//
// struct amdgpu_dm_luminance_data - Custom luminance data
// @luminance: Luminance in percent
// @input_signal: Input signal in range 0-255
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_luminance_data {
    pub luminance: u8,
    pub input_signal: u8,
    pub __packed: },
//
// struct amdgpu_dm_backlight_caps - Information about backlight
//
// Describe the backlight support for ACPI or eDP AUX.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_backlight_caps {
//
// @ext_caps: Keep the data struct with all the information about the
// display support for HDR.
//
    pub ext_caps: *mut dpcd_sink_ext_caps,
//
// @aux_min_input_signal: Min brightness value supported by the display
//
    pub aux_min_input_signal: u32,
//
// @aux_max_input_signal: Max brightness value supported by the display
// in nits.
//
    pub aux_max_input_signal: u32,
//
// @min_input_signal: minimum possible input in range 0-255.
//
    pub min_input_signal: c_int,
//
// @max_input_signal: maximum possible input in range 0-255.
//
    pub max_input_signal: c_int,
//
// @caps_valid: true if these values are from the ACPI interface.
//
    pub caps_valid: bool,
//
// @aux_support: Describes if the display supports AUX backlight.
//
    pub aux_support: bool,
//
// @brightness_mask: After deriving brightness, OR it with this mask.
// Workaround for panels with issues with certain brightness values.
//
    pub brightness_mask: u32,
//
// @ac_level: the default brightness if booted on AC
//
    pub ac_level: u8,
//
// @dc_level: the default brightness if booted on DC
//
    pub dc_level: u8,
//
// @data_points: the number of custom luminance data points
//
    pub data_points: u8,
//
// @luminance_data: custom luminance data
//
    pub luminance_data: [amdgpu_dm_luminance_data; MAX_LUMINANCE_DATA_POINTS],
}

//
// struct dal_allocation - Tracks mapped FB memory for SMU communication
// @list: list of dal allocations
// @bo: GPU buffer object
// @cpu_ptr: CPU virtual address of the GPU buffer object
// @gpu_addr: GPU virtual address of the GPU buffer object
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dal_allocation {
    pub list: list_head,
    pub bo: *mut amdgpu_bo,
    pub cpu_ptr: *mut c_void,
    pub gpu_addr: u64,
}

//
// struct hpd_rx_irq_offload_work_queue - Work queue to handle hpd_rx_irq
// offload work
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpd_rx_irq_offload_work_queue {
//
// @wq: workqueue structure to queue offload work.
//
    pub wq: *mut workqueue_struct,
//
// @offload_lock: To protect fields of offload work queue.
//
    pub offload_lock: spinlock_t,
//
// @is_handling_link_loss: Used to prevent inserting link loss event when
// we're handling link loss
//
    pub is_handling_link_loss: bool,
//
// @is_handling_mst_msg_rdy_event: Used to prevent inserting mst message
// ready event when we're already handling mst message ready event
//
    pub is_handling_mst_msg_rdy_event: bool,
//
// @aconnector: The aconnector that this work queue is attached to
//
    pub aconnector: *mut amdgpu_dm_connector,
}

//
// struct hpd_rx_irq_offload_work - hpd_rx_irq offload work structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpd_rx_irq_offload_work {
//
// @work: offload work
//
    pub work: work_struct,
//
// @data: reference irq data which is used while handling offload work
//
    pub data: hpd_irq_data,
//
// @offload_wq: offload work queue that this work is queued to
//
    pub offload_wq: *mut hpd_rx_irq_offload_work_queue,
//
// @adev: amdgpu_device pointer
//
    pub adev: *mut amdgpu_device,
}

//
// struct amdgpu_display_manager - Central amdgpu display manager device
//
// @dc: Display Core control structure
// @adev: AMDGPU base driver structure
// @ddev: DRM base driver structure
// @display_indexes_num: Max number of display streams supported
// @irq_handler_list_table_lock: Synchronizes access to IRQ tables
// @irq_wq: Dedicated high-priority unbound workqueue for deferred IRQ work
// @vmin_vmax_wq: Dedicated unbound workqueue for deferred vmin/vmax updates
// @backlight_dev: Backlight control device
// @backlight_link: Link on which to control backlight
// @backlight_caps: Capabilities of the backlight device
// @freesync_module: Module handling freesync calculations
// @hdcp_workqueue: AMDGPU content protection queue
// @fw_dmcu: Reference to DMCU firmware
// @dmcu_fw_version: Version of the DMCU firmware
// @soc_bounding_box: SOC bounding box values provided by gpu_info FW
// @cached_state: Caches device atomic state for suspend/resume
// @cached_dc_state: Cached state of content streams
// @compressor: Frame buffer compression buffer. See &struct dm_compressor_info
// @force_timing_sync: set via debugfs. When set, indicates that all connected
// displays will be forced to synchronize.
// @dmcub_trace_event_en: enable dmcub trace events
// @dmub_outbox_params: DMUB Outbox parameters
// @num_of_edps: number of backlight eDPs
// @disable_hpd_irq: disables all HPD and HPD RX interrupt handling in the
// driver when true
// @dmub_aux_transfer_done: struct completion used to indicate when DMUB
// transfers are done
// @delayed_hpd_wq: work queue used to delay DMUB HPD work
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_display_manager {
    pub dc: *mut dc,
//
// @dmub_srv:
//
// DMUB service, used for controlling the DMUB on hardware
// that supports it. The pointer to the dmub_srv will be
// NULL on hardware that does not support it.
//
    pub dmub_srv: *mut dmub_srv,
//
// @dmub_notify:
//
// Notification from DMUB.
//
    pub dmub_notify: *mut dmub_notification,
//
// @dmub_callback:
//
// Callback functions to handle notification from DMUB.
//
    pub dmub_callback: [dmub_notify_interrupt_callback_t; AMDGPU_DMUB_NOTIFICATION_MAX],
//
// @dmub_thread_offload:
//
// Flag to indicate if callback is offload.
//
    pub dmub_thread_offload: [bool; AMDGPU_DMUB_NOTIFICATION_MAX],
//
// @dmub_fb_info:
//
// Framebuffer regions for the DMUB.
//
    pub dmub_fb_info: *mut dmub_srv_fb_info,
//
// @dmub_fw:
//
// DMUB firmware, required on hardware that has DMUB support.
//
    pub dmub_fw: *const firmware,
//
// @dmub_bo:
//
// Buffer object for the DMUB.
//
    pub dmub_bo: *mut amdgpu_bo,
//
// @dmub_bo_gpu_addr:
//
// GPU virtual address for the DMUB buffer object.
//
    pub dmub_bo_gpu_addr: u64,
//
// @dmub_bo_cpu_addr:
//
// CPU address for the DMUB buffer object.
//
    pub dmub_bo_cpu_addr: *mut c_void,
//
// @dmcub_fw_version:
//
// DMCUB firmware version.
//
    pub dmcub_fw_version: u32,
//
// @fw_inst_size:
//
// Size of the firmware instruction buffer.
//
    pub fw_inst_size: u32,
//
// @cgs_device:
//
// The Common Graphics Services device. It provides an interface for
// accessing registers.
//
    pub cgs_device: *mut cgs_device,
    pub adev: *mut amdgpu_device,
    pub ddev: *mut drm_device,
    pub display_indexes_num: u16,
//
// @atomic_obj:
//
// In combination with &dm_atomic_state it helps manage
// global atomic state that doesn't map cleanly into existing
// drm resources, like &dc_context.
//
    pub atomic_obj: drm_private_obj,
//
// @dc_lock:
//
// Guards access to DC functions that can issue register write
// sequences.
//
    pub dc_lock: mutex,
//
// @dmub_lock:
//
// Guards access to DMUB command submission.
//
    pub dmub_lock: spinlock_t,
//
// @audio_lock:
//
// Guards access to audio instance changes.
//
    pub audio_lock: mutex,
//
// @audio_component:
//
// Used to notify ELD changes to sound driver.
//
    pub audio_component: *mut drm_audio_component,
//
// @audio_registered:
//
// True if the audio component has been registered
// successfully, false otherwise.
//
    pub audio_registered: bool,
//
// @irq_handler_list_low_tab:
//
// Low priority IRQ handler table.
//
// It is a n*m table consisting of n IRQ sources, and m handlers per IRQ
// source. Low priority IRQ handlers are deferred to a workqueue to be
// processed. Hence, they can sleep.
//
// Note that handlers are called in the same order as they were
// registered (FIFO).
//
    pub irq_handler_list_low_tab: [list_head; DAL_IRQ_SOURCES_NUMBER],
//
// @irq_handler_list_high_tab:
//
// High priority IRQ handler table.
//
// It is a n*m table, same as &irq_handler_list_low_tab. However,
// handlers in this table are not deferred and are called immediately.
//
    pub irq_handler_list_high_tab: [list_head; DAL_IRQ_SOURCES_NUMBER],
//
// @pflip_params:
//
// Page flip IRQ parameters, passed to registered handlers when
// triggered.
//
    pub 1]: pflip_params[DC_IRQ_SOURCE_PFLIP_LAST - DC_IRQ_SOURCE_PFLIP_FIRST +,
//
// @vblank_params:
//
// Vertical blanking IRQ parameters, passed to registered handlers when
// triggered.
//
    pub 1]: vblank_params[DC_IRQ_SOURCE_VBLANK6 - DC_IRQ_SOURCE_VBLANK1 +,
//
// @vline0_params:
//
// OTG vertical interrupt0 IRQ parameters, passed to registered
// handlers when triggered.
//
    pub 1]: vline0_params[DC_IRQ_SOURCE_DC6_VLINE0 - DC_IRQ_SOURCE_DC1_VLINE0 +,
//
// @vupdate_params:
//
// Vertical update IRQ parameters, passed to registered handlers when
// triggered.
//
    pub 1]: vupdate_params[DC_IRQ_SOURCE_VUPDATE6 - DC_IRQ_SOURCE_VUPDATE1 +,
//
// @dmub_trace_params:
//
// DMUB trace event IRQ parameters, passed to registered handlers when
// triggered.
//
    pub irq_handler_list_table_lock: spinlock_t,
    pub irq_wq: *mut workqueue_struct,
    pub vmin_vmax_wq: *mut workqueue_struct,
    pub backlight_dev: [*mut backlight_device; AMDGPU_DM_MAX_NUM_EDP],
    pub backlight_link: [*const dc_link; AMDGPU_DM_MAX_NUM_EDP],
    pub num_of_edps: u8,
    pub backlight_caps: [amdgpu_dm_backlight_caps; AMDGPU_DM_MAX_NUM_EDP],
    pub freesync_module: *mut mod_freesync,
    pub power_module: *mut mod_power,
    pub hdcp_workqueue: *mut hdcp_workqueue,
//
// @vblank_control_workqueue:
//
// Deferred work for vblank control events.
//
    pub vblank_control_workqueue: *mut workqueue_struct,
//
// @idle_workqueue:
//
// Periodic work for idle events.
//
    pub idle_workqueue: *mut idle_workqueue,
    pub cached_state: *mut drm_atomic_commit,
    pub cached_dc_state: *mut dc_state,
    pub compressor: dm_compressor_info,
    pub fw_dmcu: *const firmware,
    pub dmcu_fw_version: u32,
//
// @soc_bounding_box:
//
// gpu_info FW provided soc bounding box struct or 0 if not
// available in FW
//
    pub soc_bounding_box: *const gpu_info_soc_bounding_box_v1_0,
//
// @active_vblank_irq_count:
//
// number of currently active vblank irqs
//
    pub active_vblank_irq_count: u32,

//
// @secure_display_ctx:
//
// Store secure display relevant info. e.g. the ROI information
// , the work_struct to command dmub, etc.
//
    pub secure_display_ctx: secure_display_context,

//
// @hpd_rx_offload_wq:
//
// Work queue to offload works of hpd_rx_irq
//
    pub hpd_rx_offload_wq: *mut hpd_rx_irq_offload_work_queue,
//
// @mst_encoders:
//
// fake encoders used for DP MST.
//
    pub mst_encoders: [amdgpu_encoder; AMDGPU_DM_MAX_CRTC],
    pub force_timing_sync: bool,
    pub disable_hpd_irq: bool,
    pub dmcub_trace_event_en: bool,
//
// @da_list:
//
// DAL fb memory allocation list, for communication with SMU.
//
    pub da_list: list_head,
    pub dmub_aux_transfer_done: completion,
    pub delayed_hpd_wq: *mut workqueue_struct,
//
// @brightness:
//
// cached backlight values.
//
    pub brightness: [u32; AMDGPU_DM_MAX_NUM_EDP],
//
// @actual_brightness:
//
// last successfully applied backlight values.
//
    pub actual_brightness: [u32; AMDGPU_DM_MAX_NUM_EDP],
//
// @aux_hpd_discon_quirk:
//
// quirk for hpd discon while aux is on-going.
// occurred on certain intel platform
//
    pub aux_hpd_discon_quirk: bool,
//
// @edp0_on_dp1_quirk:
//
// quirk for platforms that put edp0 on DP1.
//
    pub edp0_on_dp1_quirk: bool,
//
// @dpia_aux_lock:
//
// Guards access to DPIA AUX
//
    pub dpia_aux_lock: mutex,
//
// @bb_from_dmub:
//
// Bounding box data read from dmub during early initialization for DCN4+
// Data is stored as a byte array that should be casted to the appropriate bb struct
//
    pub bb_from_dmub: *mut c_void,
//
// @i2c_devres_group:
//
// Devres group for DM i2c adapter lifetime management.
//
    pub i2c_devres_group: *mut c_void,
//
// @oem_i2c:
//
// OEM i2c bus
//
    pub oem_i2c: *mut amdgpu_i2c_adapter,
//
// @fused_io:
//
// dmub fused io interface
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fused_io_sync {
    pub replied: completion,
    pub here: char reply_data[0x40]; // Cannot include dmub_cmd,
    pub fused_io: [}; 8],
//
// @hdmi_frl_status_polling_work:
//
// workqueue for 200ms frl status polling
//
    pub hdmi_frl_status_polling_wq: *mut workqueue_struct,
    pub hdmi_frl_status_polling_work: delayed_work,
    pub hdmi_frl_status_polling_delay_ms: c_uint,
//
// @dm_boot_time_crc_info:
//
// Buffer info for the boot time crc.
//
    pub boot_time_crc_info: dm_boot_time_crc_info,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dsc_clock_force_state {
    DSC_CLK_FORCE_DEFAULT = 0,
    DSC_CLK_FORCE_ENABLE,
    DSC_CLK_FORCE_DISABLE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_preferred_settings {
    pub dsc_force_enable: dsc_clock_force_state,
    pub dsc_num_slices_v: u32,
    pub dsc_num_slices_h: u32,
    pub dsc_bits_per_pixel: u32,
    pub dsc_force_disable_passthrough: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mst_progress_status {
    MST_STATUS_DEFAULT = 0,
    MST_PROBE = BIT(0),
    MST_REMOTE_EDID = BIT(1),
    MST_ALLOCATE_NEW_PAYLOAD = BIT(2),
    MST_CLEAR_ALLOCATED_PAYLOAD = BIT(3),
}

//
// struct amdgpu_hdmi_vsdb_info - Keep track of the VSDB info
//
// AMDGPU supports FreeSync over HDMI by using the VSDB section, and this
// struct is useful to keep track of the display-specific information about
// FreeSync.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_hdmi_vsdb_info {
//
// @amd_vsdb_version: Vendor Specific Data Block Version, should be
// used to determine which Vendor Specific InfoFrame (VSIF) to send.
//
    pub amd_vsdb_version: c_uint,
//
// @freesync_supported: FreeSync Supported.
//
    pub freesync_supported: bool,
//
// @min_refresh_rate_hz: FreeSync Minimum Refresh Rate in Hz.
//
    pub min_refresh_rate_hz: c_uint,
//
// @max_refresh_rate_hz: FreeSync Maximum Refresh Rate in Hz
//
    pub max_refresh_rate_hz: c_uint,
//
// @freesync_mccs_vcp_code: MCCS VCP code for freesync state
//
    pub freesync_mccs_vcp_code: c_uint,
//
// @replay_mode: Replay supported
//
    pub replay_mode: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_connector {
    pub base: drm_connector,
    pub connector_id: u32,
    pub bl_idx: c_int,
    pub notifier: *mut cec_notifier,
// we need to mind the EDID between detect
    pub drm_edid: *const drm_edid,
// shared with amdgpu
    pub hpd: amdgpu_hpd,
// number of modes generated from EDID at 'dc_sink'
    pub num_modes: c_int,
// The 'old' sink - before an HPD.
// The 'current' sink is in dc_link->sink.
    pub dc_sink: *mut dc_sink,
    pub dc_link: *mut dc_link,
//
// @dc_em_sink: Reference to the emulated (virtual) sink.
//
    pub dc_em_sink: *mut dc_sink,
// DM only
    pub mst_mgr: drm_dp_mst_topology_mgr,
    pub dm_dp_aux: amdgpu_dm_dp_aux,
    pub mst_output_port: *mut drm_dp_mst_port,
    pub mst_root: *mut amdgpu_dm_connector,
    pub dsc_aux: *mut drm_dp_aux,
    pub mst_local_bw: u32,
    pub vc_full_pbn: u16,
    pub handle_mst_msg_ready: mutex,
// branch device specific data
    pub branch_ieee_oui: u32,
// TODO see if we can merge with ddc_bus or make a dm_connector
    pub i2c: *mut amdgpu_i2c_adapter,
// Monitor range limits
//
// @min_vfreq: Minimal frequency supported by the display in Hz. This
// value is set to zero when there is no FreeSync support.
//
    pub min_vfreq: c_int,
//
// @max_vfreq: Maximum frequency supported by the display in Hz. This
// value is set to zero when there is no FreeSync support.
//
    pub max_vfreq: c_int,
// Audio instance - protected by audio_lock.
    pub audio_inst: c_int,
    pub hpd_lock: mutex,
    pub fake_enable: bool,
    pub force_yuv_pixel_format: u8,
    pub dsc_settings: dsc_preferred_settings,
    pub psr_caps: psr_caps,
    pub mst_downstream_port_present: dp_downstream_port_present,
// Cached display modes
    pub freesync_vid_base: drm_display_mode,
    pub sr_skip_count: c_int,
    pub disallow_edp_enter_psr: bool,
    pub disallow_edp_enter_replay: bool,
    pub mst_downstream_port_caps: dwnstream_portxcaps,
// Record progress status of mst
    pub mst_status: u8,
// Automated testing
    pub timing_changed: bool,
    pub timing_requested: *mut dc_crtc_timing,
// Adaptive Sync
    pub pack_sdp_v1_3: bool,
    pub as_type: adaptive_sync_type,
    pub vsdb_info: amdgpu_hdmi_vsdb_info,
// HDMI HPD debounce support
    pub hdmi_hpd_debounce_delay_ms: c_uint,
    pub hdmi_hpd_debounce_work: delayed_work,
    pub hdmi_prev_sink: *mut dc_sink,
// HDMI compliance automation
    pub hdmi_comp_auto: bool,
}

// status |= flags;
// status &= ~flags;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_wb_connector {
    pub base: drm_writeback_connector,
    pub link: *mut dc_link,
}

// enum amdgpu_transfer_function: pre-defined transfer function supported by AMD.
//
// It includes standardized transfer functions and pure power functions. The
// transfer function coefficients are available at modules/color/color_gamma.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_transfer_function {
    AMDGPU_TRANSFER_FUNCTION_DEFAULT,
    AMDGPU_TRANSFER_FUNCTION_SRGB_EOTF,
    AMDGPU_TRANSFER_FUNCTION_BT709_INV_OETF,
    AMDGPU_TRANSFER_FUNCTION_PQ_EOTF,
    AMDGPU_TRANSFER_FUNCTION_IDENTITY,
    AMDGPU_TRANSFER_FUNCTION_GAMMA22_EOTF,
    AMDGPU_TRANSFER_FUNCTION_GAMMA24_EOTF,
    AMDGPU_TRANSFER_FUNCTION_GAMMA26_EOTF,
    AMDGPU_TRANSFER_FUNCTION_SRGB_INV_EOTF,
    AMDGPU_TRANSFER_FUNCTION_BT709_OETF,
    AMDGPU_TRANSFER_FUNCTION_PQ_INV_EOTF,
    AMDGPU_TRANSFER_FUNCTION_GAMMA22_INV_EOTF,
    AMDGPU_TRANSFER_FUNCTION_GAMMA24_INV_EOTF,
    AMDGPU_TRANSFER_FUNCTION_GAMMA26_INV_EOTF,
    AMDGPU_TRANSFER_FUNCTION_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_plane_state {
    pub base: drm_plane_state,
    pub dc_state: *mut dc_plane_state,
// Plane color mgmt
//
// @degamma_lut:
//
// 1D LUT for mapping framebuffer/plane pixel data before sampling or
// blending operations. It's usually applied to linearize input space.
// The blob (if not NULL) is an array of &struct drm_color_lut.
//
    pub degamma_lut: *mut drm_property_blob,
//
// @degamma_tf:
//
// Predefined transfer function to tell DC driver the input space to
// linearize.
//
    pub degamma_tf: amdgpu_transfer_function,
//
// @hdr_mult:
//
// Multiplier to 'gain' the plane.  When PQ is decoded using the fixed
// func transfer function to the internal FP16 fb, 1.0 -> 80 nits (on
// AMD at least). When sRGB is decoded, 1.0 -> 1.0, obviously.
// Therefore, 1.0 multiplier = 80 nits for SDR content.  So if you
// want, 203 nits for SDR content, pass in (203.0 / 80.0).  Format is
// S31.32 sign-magnitude.
//
// HDR multiplier can wide range beyond [0.0, 1.0]. This means that PQ
// TF is needed for any subsequent linear-to-non-linear transforms.
//
    pub hdr_mult: __u64,
//
// @ctm:
//
// Color transformation matrix. The blob (if not NULL) is a &struct
// drm_color_ctm_3x4.
//
    pub ctm: *mut drm_property_blob,
//
// @shaper_lut: shaper lookup table blob. The blob (if not NULL) is an
// array of &struct drm_color_lut.
//
    pub shaper_lut: *mut drm_property_blob,
//
// @shaper_tf:
//
// Predefined transfer function to delinearize color space.
//
    pub shaper_tf: amdgpu_transfer_function,
//
// @lut3d: 3D lookup table blob. The blob (if not NULL) is an array of
// &struct drm_color_lut.
//
    pub lut3d: *mut drm_property_blob,
//
// @blend_lut: blend lut lookup table blob. The blob (if not NULL) is an
// array of &struct drm_color_lut.
//
    pub blend_lut: *mut drm_property_blob,
//
// @blend_tf:
//
// Pre-defined transfer function for converting plane pixel data before
// applying blend LUT.
//
    pub blend_tf: amdgpu_transfer_function,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_dm_cursor_mode {
    DM_CURSOR_NATIVE_MODE = 0,
    DM_CURSOR_OVERLAY_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_crtc_state {
    pub base: drm_crtc_state,
    pub stream: *mut dc_stream_state,
    pub cm_has_degamma: bool,
    pub cm_is_degamma_srgb: bool,
    pub mpo_requested: bool,
    pub update_type: c_int,
    pub active_planes: c_int,
    pub crc_skip_count: c_int,
    pub freesync_vrr_info_changed: bool,
    pub mode_changed_independent_from_dsc: bool,
    pub dsc_force_changed: bool,
    pub vrr_supported: bool,
    pub freesync_config: mod_freesync_config,
    pub vrr_infopacket: dc_info_packet,
    pub abm_level: c_int,
//
// @regamma_tf:
//
// Pre-defined transfer function for converting internal FB -> wire
// encoding.
//
    pub regamma_tf: amdgpu_transfer_function,
    pub cursor_mode: amdgpu_dm_cursor_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_atomic_state {
    pub base: drm_private_state,
    pub context: *mut dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dm_connector_state {
    pub base: drm_connector_state,
    pub scaling: amdgpu_rmx_type,
    pub underscan_vborder: u8,
    pub underscan_hborder: u8,
    pub underscan_enable: bool,
    pub freesync_capable: bool,
    pub update_hdcp: bool,
    pub abm_sysfs_forbidden: bool,
    pub abm_level: u8,
    pub vcpi_slots: c_int,
    pub pbn: u64,
}

// Macro flag: #define to_dm_connector_state(x)\

extern "C" {
    pub fn amdgpu_dm_trigger_timing_sync(dev: *mut drm_device);
}
// 3D LUT max size is 17x17x17 (4913 entries)
pub const MAX_COLOR_3DLUT_SIZE: c_int = 17;
pub const MAX_COLOR_3DLUT_BITDEPTH: c_int = 12;
// 1D LUT size
pub const MAX_COLOR_LUT_ENTRIES: c_int = 4096;
// Legacy gamm LUT users such as X doesn't like large LUT sizes
pub const MAX_COLOR_LEGACY_LUT_ENTRIES: c_int = 256;
extern "C" {
    pub fn amdgpu_dm_init_color_mod();
}
extern "C" {
    pub fn amdgpu_dm_create_color_properties(adev: *mut amdgpu_device) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_verify_lut_sizes(crtc_state: *const drm_crtc_state) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_update_crtc_color_mgmt(crtc: *mut dm_crtc_state) -> c_int;
}
extern "C" {
    pub fn amdgpu_dm_is_headless(adev: *mut amdgpu_device) -> bool;
}
extern "C" {
    pub fn amdgpu_dm_crtc_complete_writeback(acrtc: *mut amdgpu_crtc) -> bool;
}
extern "C" {
    pub fn retrieve_dmi_info(dm: *mut amdgpu_display_manager);
}
extern "C" {
    pub fn dm_should_disable_stutter(pdev: *mut pci_dev) -> bool;
}
extern "C" {
    pub fn amdgpu_dm_emulated_link_detect(link: *mut dc_link);
}
//
// Use the uniqueness of the plane's (zpos, drm obj ID) combination to iterate
// by descending zpos, as read from the new plane state. This is the same
// ordering as defined by drm_atomic_normalize_zpos().
//

extern "C" {
    pub fn dm_is_idle(ip_block: *mut amdgpu_ip_block) -> bool;
}
extern "C" {
    pub fn dm_wait_for_idle(ip_block: *mut amdgpu_ip_block) -> c_int;
}
extern "C" {
    pub fn dm_soft_reset(ip_block: *mut amdgpu_ip_block) -> c_int;
}
extern "C" {
    pub fn dm_bandwidth_update(adev: *mut amdgpu_device);
}
extern "C" {
    pub fn dm_vblank_get_counter(adev: *mut amdgpu_device, crtc: c_int) -> u32;
}
extern "C" {
    pub fn dm_plane_layer_index_cmp(a: *const c_void, b: *const c_void) -> c_int;
}
extern "C" {
    pub fn modereset_required(crtc_state: *mut drm_crtc_state) -> bool;
}
extern "C" {
    pub fn set_multisync_trigger_params(stream: *mut dc_stream_state);
}
extern "C" {
    pub fn set_master_stream(stream_set[]: *mut dc_stream_state, stream_count: c_int);
}
extern "C" {
    pub fn dm_enable_per_frame_crtc_master_sync(context: *mut dc_state);
}

