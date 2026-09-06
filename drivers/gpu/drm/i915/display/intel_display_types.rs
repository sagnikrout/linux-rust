//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_types.h
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
// Copyright (c) 2006 Dave Airlie <airlied@linux.ie>
// Copyright (c) 2007-2008 Intel Corporation
// Jesse Barnes <jesse.barnes@intel.com>
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the next
// paragraph) shall be included in all copies or substantial portions of the
// Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

//
// Display related stuff
//
// these are outputs from the chip - integrated only
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_output_type {
    INTEL_OUTPUT_UNUSED = 0,
    INTEL_OUTPUT_ANALOG = 1,
    INTEL_OUTPUT_DVO = 2,
    INTEL_OUTPUT_SDVO = 3,
    INTEL_OUTPUT_LVDS = 4,
    INTEL_OUTPUT_TVOUT = 5,
    INTEL_OUTPUT_HDMI = 6,
    INTEL_OUTPUT_DP = 7,
    INTEL_OUTPUT_EDP = 8,
    INTEL_OUTPUT_DSI = 9,
    INTEL_OUTPUT_DDI = 10,
    INTEL_OUTPUT_DP_MST = 11,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_force_audio {
    HDMI_AUDIO_OFF_DVI = -2,	/* no aux data for HDMI-DVI converter */
    HDMI_AUDIO_OFF,			/* force turn off HDMI audio */
    HDMI_AUDIO_AUTO,		/* trust EDID */
    HDMI_AUDIO_ON,			/* force turn on HDMI audio */
}

// "Broadcast RGB" property
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_broadcast_rgb {
    INTEL_BROADCAST_RGB_AUTO,
    INTEL_BROADCAST_RGB_FULL,
    INTEL_BROADCAST_RGB_LIMITED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_fb_view {
//
// The remap information used in the remapped and rotated views to
// create the DMA scatter-gather list for each FB color plane. This sg
// list is created along with the view type (gtt.type) specific
// i915_vma object and contains the list of FB object pages (reordered
// in the rotated view) that are visible in the view.
// In the normal view the FB object's backing store sg list is used
// directly and hence the remap information here is not used.
//
    pub gtt: i915_gtt_view,
//
// The GTT view (gtt.type) specific information for each FB color
// plane. In the normal GTT view all formats (up to 4 color planes),
// in the rotated and remapped GTT view all no-CCS formats (up to 2
// color planes) are supported.
//
// The view information shared by all FB color planes in the FB,
// like dst x/y and src/dst width, is stored separately in
// intel_plane_state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i915_color_plane_view {
    pub offset: u32,
    pub y: unsigned int x,,
//
// Plane stride in:
// bytes for 0/180 degree rotation
// pixels for 90/270 degree rotation
//
    pub mapping_stride: c_uint,
    pub scanout_stride: c_uint,
    pub color_plane: [}; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_framebuffer {
    pub base: drm_framebuffer,
    pub frontbuffer: *mut intel_frontbuffer,
// Params to remap the FB pages and program the plane registers in each view.
    pub normal_view: intel_fb_view,
    pub rotated_view: intel_fb_view,
    pub remapped_view: intel_fb_view,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_hotplug_state {
    INTEL_HOTPLUG_UNCHANGED,
    INTEL_HOTPLUG_CHANGED,
    INTEL_HOTPLUG_RETRY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_encoder {
    pub base: drm_encoder,
    pub type: intel_output_type,
    pub port: port,
    pub cloneable: u16,
    pub pipe_mask: u8,
// Check and recover a bad link state.
    pub link_check_work: delayed_work,
    pub encoder): *mut *mut void (link_check)(struct intel_encoder,
    pub connector): *mut intel_connector,
    pub ): *mut drm_connector_state,
    pub ): *mut drm_connector_state,
    pub ): *mut drm_connector_state,
    pub ): *const drm_connector_state,
    pub ): *const drm_connector_state,
    pub ): *const drm_connector_state,
    pub ): *const drm_connector_state,
    pub ): *const drm_connector_state,
    pub ): *const drm_connector_state,
    pub ): *const drm_connector_state,
    pub conn_state): *const drm_connector_state,
    pub old_conn_state): *const drm_connector_state,
// Read out the current hw state of this connector, returning true if
// the encoder is active. If the encoder is enabled it also set the pipe
// it is connected to in the pipe parameter.
    pub pipe): *mut *mut *mut bool (get_hw_state)(struct intel_encoder , enum pipe,
// Reconstructs the equivalent mode flags for the current hardware
// state. This must be called _after_ display->get_pipe_config has
// pre-filled the pipe config. Note that intel_encoder->base.crtc must
// be set correctly before calling this function.
    pub pipe_config): *mut intel_crtc_state,
//
// Optional hook called during init/resume to sync any state
// stored in the encoder (eg. DP link parameters) wrt. the HW state.
//
    pub crtc_state): *const intel_crtc_state,
//
// Optional hook, returning true if this encoder allows a fastset
// during the initial commit, false otherwise.
//
    pub crtc_state): *mut intel_crtc_state,
//
// Acquires the power domains needed for an active encoder during
// hardware state readout.
//
    pub crtc_state): *mut intel_crtc_state,
//
// Called during system suspend after all pending requests for the
// encoder are flushed (for example for DP AUX transactions) and
// device interrupts are disabled.
// All modeset locks are held while the hook is called.
//
    pub ): *mut *mut void (suspend)(struct intel_encoder,
//
// Called without the modeset locks held after the suspend() hook for
// all encoders have been called.
//
    pub encoder): *mut *mut void (suspend_complete)(struct intel_encoder,
//
// Called during system reboot/shutdown after all the
// encoders have been disabled and suspended.
// All modeset locks are held while the hook is called.
//
    pub encoder): *mut *mut void (shutdown)(struct intel_encoder,
//
// Called without the modeset locks held after the shutdown() hook for
// all encoders have been called.
//
    pub encoder): *mut *mut void (shutdown_complete)(struct intel_encoder,
//
// Enable/disable the clock to the port.
//
    pub crtc_state): *const intel_crtc_state,
    pub encoder): *mut *mut void (disable_clock)(struct intel_encoder,
//
// Returns whether the port clock is enabled or not.
//
    pub encoder): *mut *mut bool (is_clock_enabled)(struct intel_encoder,
//
// Returns the PLL type the port uses.
//
    pub crtc_state): *const intel_crtc_state,
    pub n_entries): *mut c_int,
    pub crtc_state): *const intel_crtc_state,
    pub hpd_pin: hpd_pin,
    pub power_domain: intel_display_power_domain,
// VBT information for this encoder (may be NULL for older platforms)
    pub devdata: *const intel_bios_encoder_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_panel_bl_funcs {
// Connector and platform specific backlight functions
    pub pipe): *mut *mut *mut int (setup)(struct intel_connector connector, enum pipe,
    pub pipe): *mut *mut *mut u32 (get)(struct intel_connector connector, enum pipe,
    pub level): *const *const *const void (set)(struct drm_connector_state conn_state, u32,
    pub level): *const *const *const void (disable)(struct drm_connector_state conn_state, u32,
    pub level): *const *const drm_connector_state conn_state, u32,
    pub hz): *mut *mut *mut u32 (hz_to_pwm)(struct intel_connector connector, u32,
}

// in 100us units
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pps_delays {
    pub /: *mut *mut u16 power_up; / eDP: T1+T3, LVDS: T1+T2,
    pub /: *mut *mut u16 backlight_on; / eDP: T8, LVDS: T5,
    pub /: *mut *mut u16 backlight_off; / eDP: T9, LVDS: T6/TX,
    pub /: *mut *mut u16 power_down; / eDP: T10, LVDS: T3,
    pub /: *mut *mut u16 power_cycle; / eDP: T11+T12, LVDS: T7+T4,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drrs_type {
    DRRS_TYPE_NONE,
    DRRS_TYPE_STATIC,
    DRRS_TYPE_SEAMLESS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_vbt_panel_data {
    pub /: *mut *mut *mut drm_display_mode lfp_vbt_mode; / if any,
    pub /: *mut *mut *mut drm_display_mode sdvo_lvds_vbt_mode; / if any,
// Feature bits
    pub panel_type: c_int,
    pub lvds_dither:1: c_uint,
    pub /: *mut *mut unsigned int bios_lvds_val; / initial [PCH_]LVDS reg val in VBIOS,
    pub vrr: bool,
    pub seamless_drrs_min_refresh_rate: u8,
    pub drrs_type: drrs_type,
    pub max_link_rate: c_int,
    pub rate: c_int,
    pub lanes: c_int,
    pub preemphasis: c_int,
    pub vswing: c_int,
    pub bpp: c_int,
    pub pps: intel_pps_delays,
    pub drrs_msa_timing_delay: u8,
    pub low_vswing: bool,
    pub hobl: bool,
    pub dsc_disable: bool,
    pub pipe_joiner_enable: bool,
    pub edp: },
    pub enable: bool,
    pub full_link: bool,
    pub require_aux_wakeup: bool,
    pub idle_frames: c_int,
    pub tp1_wakeup_time_us: c_int,
    pub tp2_tp3_wakeup_time_us: c_int,
    pub psr2_tp2_tp3_wakeup_time_us: c_int,
    pub psr: },
    pub pwm_freq_hz: u16,
    pub brightness_precision_bits: u16,
    pub hdr_dpcd_refresh_timeout: u16,
    pub present: bool,
    pub active_low_pwm: bool,
    pub /: *mut *mut u8 min_brightness; / min_brightness/255 of max,
    pub /: *mut *mut s8 controller; / brightness controller number,
    pub type: intel_backlight_type,
    pub backlight: },
// MIPI DSI
    pub panel_id: u16,
    pub config: *mut mipi_config,
    pub pps: *mut mipi_pps_data,
    pub bl_ports: u16,
    pub cabc_ports: u16,
    pub seq_version: u8,
    pub size: u32,
    pub data: *mut u8,
    pub sequence: [*const u8; MIPI_SEQ_MAX],
    pub /: *mut *mut *mut u8 deassert_seq; / Used by fixup_mipi_sequences(),
    pub orientation: drm_panel_orientation,
    pub dsi: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_panel {
// Simple drm_panel
    pub base: *mut drm_panel,
// Fixed EDID for eDP and LVDS. May hold ERR_PTR for invalid EDID.
    pub fixed_edid: *const drm_edid,
    pub fixed_modes: list_head,
// backlight
    pub present: bool,
    pub level: u32,
    pub min: u32,
    pub max: u32,
    pub enabled: bool,
    pub /: *mut *mut bool combination_mode; / gen 2/4 only,
    pub active_low_pwm: bool,
    pub /: *mut *mut bool alternate_pwm_increment; / lpt+,
// PWM chip
    pub pwm_level_min: u32,
    pub pwm_level_max: u32,
    pub pwm_enabled: bool,
    pub /: *mut *mut bool util_pin_active_low; / bxt+,
    pub /: *mut *mut u8 controller; / bxt+ only,
    pub pwm: *mut pwm_device,
    pub pwm_state: pwm_state,
// DPCD backlight
    pub info: drm_edp_backlight_info,
    pub luminance_control_support: bool,
    pub vesa: },
    pub sdr_uses_aux: bool,
    pub supports_2084_decode: bool,
    pub supports_2020_gamut: bool,
    pub supports_segmented_backlight: bool,
    pub supports_sdp_colorimetry: bool,
    pub supports_tone_mapping: bool,
    pub intel_cap: },
    pub edp: },
    pub device: *mut backlight_device,
    pub funcs: *const intel_panel_bl_funcs,
    pub pwm_funcs: *const intel_panel_bl_funcs,
    pub enable): *mut *mut *mut void (power)(struct intel_connector , bool,
    pub backlight: },
    pub vbt: intel_vbt_panel_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_hdcp {
    pub shim: *const intel_hdcp_shim,
// Mutex for hdcp state of the connector
    pub mutex: mutex,
    pub value: u64,
    pub check_work: delayed_work,
    pub prop_work: work_struct,
// HDCP1.4 Encryption status
    pub hdcp_encrypted: bool,
// HDCP2.2 related definitions
// Flag indicates whether this connector supports HDCP2.2 or not.
    pub hdcp2_supported: bool,
// HDCP2.2 Encryption status
    pub hdcp2_encrypted: bool,
//
// Content Stream Type defined by content owner. TYPE0(0x0) content can
// flow in the link protected by HDCP2.2 or HDCP1.4, where as TYPE1(0x1)
// content can flow only through a link protected by HDCP2.2.
//
    pub content_type: u8,
    pub is_paired: bool,
    pub is_repeater: bool,
//
// Count of ReceiverID_List received. Initialized to 0 at AKE_INIT.
// Incremented after processing the RepeaterAuth_Send_ReceiverID_List.
// When it rolls over re-auth has to be triggered.
//
    pub seq_num_v: u32,
//
// Count of RepeaterAuth_Stream_Manage msg propagated.
// Initialized to 0 on AKE_INIT. Incremented after every successful
// transmission of RepeaterAuth_Stream_Manage message. When it rolls
// over re-Auth has to be triggered.
//
    pub seq_num_m: u32,
//
// Work queue to signal the CP_IRQ. Used for the waiters to read the
// available information from HDCP DP sink.
//
    pub cp_irq_queue: wait_queue_head_t,
    pub cp_irq_count: core::sync::atomic::AtomicI32,
    pub cp_irq_count_cached: c_int,
//
// HDCP register access for gen12+ need the transcoder associated.
// Transcoder attached to the connector could be changed at modeset.
// Hence caching the transcoder here.
//
    pub cpu_transcoder: transcoder,
// Only used for DP MST stream encryption
    pub stream_transcoder: transcoder,
// Used to force HDCP 1.4 bypassing HDCP 2.x
    pub force_hdcp14: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_panel_replay_dsc_support {
    INTEL_DP_PANEL_REPLAY_DSC_NOT_SUPPORTED,
    INTEL_DP_PANEL_REPLAY_DSC_FULL_FRAME_ONLY,
    INTEL_DP_PANEL_REPLAY_DSC_SELECTIVE_UPDATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_connector {
    pub base: drm_connector,
//
// The fixed encoder this connector is connected to.
//
    pub encoder: *mut intel_encoder,
// ACPI device id for ACPI and driver cooperation
    pub acpi_device_id: u32,
// Reads out the current hw, returning true if the connector is enabled
// and active (i.e. dpms ON state).
    pub ): *mut *mut bool (get_hw_state)(struct intel_connector,
//
// Optional hook called during init/resume to sync any state
// stored in the connector (eg. DSC state) wrt. the HW state.
//
    pub crtc_state): *const intel_crtc_state,
// Panel info for eDP and LVDS
    pub panel: intel_panel,
// Cached EDID for detect.
    pub detect_edid: *const drm_edid,
// Number of times hotplug detection was tried after an HPD interrupt
    pub hotplug_retries: c_int,
// since POLL and HPD connectors may use the same HPD line keep the native
    pub polled: u8,
    pub force_joined_pipes: c_int,
    pub dsc_decompression_aux: *mut drm_dp_aux,
    pub dsc_dpcd: [u8; DP_DSC_RECEIVER_CAP_SIZE],
    pub fec_capability: u8,
    pub dsc_hblank_expansion_quirk:1: u8,
    pub dsc_throughput_quirk:1: u8,
    pub dsc_decompression_enabled:1: u8,
    pub rgb_yuv444: c_int,
    pub yuv422_420: c_int,
    pub overall_throughput: },
    pub max_line_width: c_int,
    pub dsc_branch_caps: },
    pub dpcd: [u8; DP_PANEL_REPLAY_CAP_SIZE],
    pub support: bool,
    pub su_support: bool,
    pub dsc_support: intel_panel_replay_dsc_support,
    pub su_w_granularity: u16,
    pub su_y_granularity: u16,
    pub panel_replay_caps: },
    pub dpcd: [u8; EDP_PSR_RECEIVER_CAP_SIZE],
    pub intel_wa_dpcd: u8,
    pub support: bool,
    pub su_support: bool,
    pub su_w_granularity: u16,
    pub su_y_granularity: u16,
    pub sync_latency: u8,
    pub psr_caps: },
    pub dp: },
    pub port: *mut drm_dp_mst_port,
    pub dp: *mut intel_dp,
    pub mst: },
    pub force_bpp_x16: c_int,
    pub link: },
// Work struct to schedule a uevent on link train failure
    pub modeset_retry_work: work_struct,
    pub hdcp: intel_hdcp,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_digital_connector_state {
    pub base: drm_connector_state,
    pub force_audio: hdmi_force_audio,
    pub broadcast_rgb: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpll {
// given values
    pub n: c_int,
    pub m2: int m1,,
    pub p2: int p1,,
// derived values
    pub dot: c_int,
    pub vco: c_int,
    pub m: c_int,
    pub p: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_atomic_state {
    pub base: drm_atomic_commit,
    pub wakeref: *mut ref_tracker,
    pub global_objs: *mut intel_global_objs_state,
    pub num_global_objs: c_int,
// Internal commit, as opposed to userspace/client initiated one
    pub internal: bool,
    pub modeset: bool dpll_set,,
    pub dpll_state: [intel_dpll_state; I915_NUM_PLLS],
    pub inherited_dp_tunnels: *mut intel_dp_tunnel_inherited_state,
//
// Current watermarks can't be trusted during hardware readout, so
// don't bother calculating intermediate watermarks.
//
    pub skip_intermediate_wm: bool,
    pub rps_interactive: bool,
    pub cleanup_work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_plane_state {
    pub uapi: drm_plane_state,
//
// actual hardware state, the state we program to the hardware.
// The following members are used to verify the hardware state:
// During initial hw readout, they need to be copied from uapi.
//
    pub crtc: *mut drm_crtc,
    pub fb: *mut drm_framebuffer,
    pub alpha: u16,
    pub pixel_blend_mode: u16,
    pub rotation: c_uint,
    pub color_encoding: drm_color_encoding,
    pub color_range: drm_color_range,
    pub scaling_filter: drm_scaling_filter,
    pub lut_3d: *mut *mut *mut *mut drm_property_blob ctm, degamma_lut, gamma_lut,,
    pub hw: },
    pub ggtt_vma: *mut i915_vma,
    pub dpt_vma: *mut i915_vma,
    pub view: intel_fb_view,
// for legacy cursor fb unpin
    pub unpin_work: drm_vblank_work,
// fenced region ID (-1 if none)
    pub fence_id: i8,
// Plane pxp decryption state
    pub decrypt: bool,
// Plane state to display black pixels when pxp is borked
    pub force_black: bool,
// Acting as Y plane for another UV plane?
    pub is_y_plane: bool,
// plane control register
    pub ctl: u32,
// plane color control register
    pub color_ctl: u32,
// chroma upsampler control register
    pub cus_ctl: u32,
// surface address register
    pub surf: u32,
//
// scaler_id
// = -1 : not using a scaler
// >=  0 : using a scalers
//
// plane requiring a scaler:
// - During check_plane, its bit is set in
// crtc_state->scaler_state.scaler_users by calling helper function
// update_scaler_plane.
// - scaler_id indicates the scaler it got assigned.
//
// plane doesn't require a scaler:
// - this can happen when scaling is no more required or plane simply
// got disabled.
// - During check_plane, corresponding bit is reset in
// crtc_state->scaler_state.scaler_users by calling helper function
// update_scaler_plane.
//
    pub scaler_id: c_int,
//
// planar_linked_plane:
//
// ICL planar formats require 2 planes that are updated as pairs.
// This member is used to make sure the other plane is also updated
// when required, and for update_slave() to find the correct
// plane_state to pass as argument.
//
    pub planar_linked_plane: *mut intel_plane,
    pub ckey: drm_intel_sprite_colorkey,
    pub psr2_sel_fetch_area: drm_rect,
// Clear Color Value
    pub ccval: u64,
    pub no_fbc_reason: *const c_char,
    pub damage: drm_rect,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_initial_plane_config {
    pub fb: *mut drm_framebuffer,
    pub vma: *mut i915_vma,
    pub size: c_int,
    pub base: u32,
    pub rotation: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_scaler {
    pub mode: u32,
    pub in_use: bool,
    pub hscale: c_int,
    pub vscale: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_crtc_scaler_state {
pub const SKL_NUM_SCALERS: c_int = 2;
    pub scalers: [intel_scaler; SKL_NUM_SCALERS],
//
// scaler_users: keeps track of users requesting scalers on this crtc.
//
// If a bit is set, a user is using a scaler.
// Here user can be a plane or crtc as defined below:
// bits 0-30 - plane (bit position is index from drm_plane_index)
// bit 31    - crtc
//
// Instead of creating a new index to cover planes and crtc, using
// existing drm_plane_index for planes which is well less than 31
// planes and bit 31 for crtc. This should be fine to cover all
// our platforms.
//
// intel_atomic_setup_scalers will setup available scalers to users
// requesting scalers. It will gracefully fail if request exceeds
// availability.
//
pub const SKL_CRTC_INDEX: c_int = 31;
    pub scaler_users: unsigned,
// scaler used by crtc for panel fitting purpose
    pub scaler_id: c_int,
}

// {crtc,crtc_state}->mode_flags
// Flag to get scanline using frame time stamps

// Flag to use the scanline counter instead of the pixel counter

//
// TE0 or TE1 flag is set if the crtc has a DSI encoder which
// is operating in command mode.
// Flag to use TE from DSI0 instead of VBI in command mode
//

// Flag to use TE from DSI1 instead of VBI in command mode

// Flag to indicate mipi dsi periodic command mode where we do not get TE

// Do tricks to make vblank timestamps sane with VRR?

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_wm_level {
    pub enable: bool,
    pub pri_val: u32,
    pub spr_val: u32,
    pub cur_val: u32,
    pub fbc_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pipe_wm {
    pub wm: [intel_wm_level; 5],
    pub fbc_wm_enabled: bool,
    pub pipe_enabled: bool,
    pub sprites_enabled: bool,
    pub sprites_scaled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skl_wm_level {
    pub min_ddb_alloc: u16,
    pub /: *mut *mut u16 min_ddb_alloc_uv; / for pre-icl,
    pub blocks: u16,
    pub lines: u8,
    pub enable: bool,
    pub ignore_lines: bool,
    pub auto_min_alloc_wm_enable: bool,
    pub can_sagv: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skl_plane_wm {
    pub wm: [skl_wm_level; 8],
    pub trans_wm: skl_wm_level,
    pub wm0: skl_wm_level,
    pub trans_wm: skl_wm_level,
    pub sagv: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct skl_pipe_wm {
    pub planes: [skl_plane_wm; I915_MAX_PLANES],
    pub use_sagv_wm: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vlv_wm_level {
    VLV_WM_LEVEL_PM2,
    VLV_WM_LEVEL_PM5,
    VLV_WM_LEVEL_DDR_DVFS,
    NUM_VLV_WM_LEVELS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlv_wm_state {
    pub wm: [g4x_pipe_wm; NUM_VLV_WM_LEVELS],
    pub sr: [g4x_sr_wm; NUM_VLV_WM_LEVELS],
    pub num_levels: u8,
    pub cxsr: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vlv_fifo_state {
    pub plane: [u16; I915_MAX_PLANES],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum g4x_wm_level {
    G4X_WM_LEVEL_NORMAL,
    G4X_WM_LEVEL_SR,
    G4X_WM_LEVEL_HPLL,
    NUM_G4X_WM_LEVELS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct g4x_wm_state {
    pub wm: g4x_pipe_wm,
    pub sr: g4x_sr_wm,
    pub hpll: g4x_sr_wm,
    pub cxsr: bool,
    pub hpll_en: bool,
    pub fbc_en: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_crtc_wm_state {
//
// raw:
// The "raw" watermark values produced by the formula
// given the plane's current state. They do not consider
// how much FIFO is actually allocated for each plane.
//
// optimal:
// The "optimal" watermark values given the current
// state of the planes and the amount of FIFO
// allocated to each, ignoring any previous state
// of the planes.
//
// intermediate:
// The "intermediate" watermark values when transitioning
// between the old and new "optimal" values. Used when
// the watermark registers are single buffered and hence
// their state changes asynchronously with regards to the
// actual plane registers. These are essentially the
// worst case combination of the old and new "optimal"
// watermarks, which are therefore safe to use when the
// plane is in either its old or new state.
//
    pub intermediate: intel_pipe_wm,
    pub optimal: intel_pipe_wm,
    pub ilk: },
    pub raw: skl_pipe_wm,
// gen9+ only needs 1-step wm programming
    pub optimal: skl_pipe_wm,
    pub ddb: skl_ddb_entry,
//
// pre-icl: for packed/planar CbCr
// icl+: for everything
//
    pub plane_ddb: [skl_ddb_entry; I915_MAX_PLANES],
// pre-icl: for planar Y
    pub plane_ddb_y: [skl_ddb_entry; I915_MAX_PLANES],
//
// xe3: Minimum amount of display blocks and minimum
// sagv allocation required for async flip
//
    pub plane_min_ddb: [u16; I915_MAX_PLANES],
    pub plane_interim_ddb: [u16; I915_MAX_PLANES],
    pub skl: },
    pub /: *mut *mut g4x_pipe_wm raw[NUM_VLV_WM_LEVELS]; / not inverted,
    pub /: *mut *mut vlv_wm_state intermediate; / inverted,
    pub /: *mut *mut vlv_wm_state optimal; / inverted,
    pub fifo_state: vlv_fifo_state,
    pub vlv: },
    pub raw: [g4x_pipe_wm; NUM_G4X_WM_LEVELS],
    pub intermediate: g4x_wm_state,
    pub optimal: g4x_wm_state,
    pub g4x: },
}

//
// Platforms with two-step watermark programming will need to
// update watermark programming post-vblank to switch from the
// safe intermediate watermarks to the optimal final
// watermarks.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_output_format {
    INTEL_OUTPUT_FORMAT_RGB,
    INTEL_OUTPUT_FORMAT_YCBCR420,
    INTEL_OUTPUT_FORMAT_YCBCR444,
}

// Used by dp and fdi links
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_link_m_n {
    pub tu: u32,
    pub data_m: u32,
    pub data_n: u32,
    pub link_m: u32,
    pub link_n: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_csc_matrix {
    pub coeff: [u16; 9],
    pub preoff: [u16; 3],
    pub postoff: [u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scaler_filter_coeff {
    pub sign: u16,
    pub exp: u16,
    pub mantissa: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_casf {
pub const SCALER_FILTER_NUM_TAPS: c_int = 7;
    pub coeff: [scaler_filter_coeff; SCALER_FILTER_NUM_TAPS],
    pub strength: u8,
    pub win_size: u8,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_crtc_state {
//
// uapi (drm) state. This is the software state shown to userspace.
// In particular, the following members are used for bookkeeping:
// - crtc
// - state
// - *_changed
// - event
// - commit
// - mode_blob
//
    pub uapi: drm_crtc_state,
//
// actual hardware state, the state we program to the hardware.
// The following members are used to verify the hardware state:
// - enable
// - active
// - mode / pipe_mode / adjusted_mode
// - color property blobs.
//
// During initial hw readout, they need to be copied to uapi.
//
// Joiner will allow a transcoder mode that spans 2 pipes;
// Use the pipe_mode for calculations like watermarks, pipe
// scaler, and bandwidth.
//
// Use adjusted_mode for things that need to know the full
// mode on the transcoder, which spans all pipes.
//
    pub enable: bool active,,
// logical state of LUTs
    pub ctm: *mut *mut *mut drm_property_blob degamma_lut, gamma_lut,,
    pub adjusted_mode: drm_display_mode mode, pipe_mode,,
    pub background_color: u32,
    pub scaling_filter: drm_scaling_filter,
    pub sharpness_strength: u8,
    pub hw: },
// actual state of LUTs
    pub post_csc_lut: *mut *mut drm_property_blob pre_csc_lut,,
    pub output_csc: intel_csc_matrix csc,,
//
// quirks - bitfield with hw state readout quirks
//
// For various reasons the hw state readout code might not be able to
// completely faithfully read out the current state. These cases are
// tracked with quirk flags so that fastboot and state checker can act
// accordingly.
//

    pub quirks: c_ulong,
    pub /: *mut *mut unsigned fb_bits; / framebuffers to flip,
    pub /: *mut *mut bool update_pipe; / can a fast modeset be performed?,
    pub /: *mut *mut bool update_m_n; / update M/N seamlessly during fastset?,
    pub /: *mut *mut bool update_lrr; / update TRANS_VTOTAL/etc. during fastset?,
    pub disable_cxsr: bool,
    pub /: *mut *mut bool update_wm_pre, update_wm_post; / watermarks are updated,
    pub /: *mut *mut bool fifo_changed; / FIFO split is changed,
    pub preload_luts: bool,
    pub /: *mut *mut bool inherited; / state inherited from BIOS?,
// Ask the hardware to actually async flip?
    pub do_async_flip: bool,
// Pipe source size (ie. panel fitter input size)
// All planes will be positioned inside this space,
// and get clipped at the edges.
    pub pipe_src: drm_rect,
//
// Pipe pixel rate, adjusted for
// panel fitter/pipe scaler downscaling.
//
    pub pixel_rate: c_uint,
//
// Pipe pixel rate for CDCLK, adjusted for
// panel fitter/pipe scaler downscaling.
// CDCLK use cases need further adjustment.
//
    pub pixel_rate_cdclk: c_uint,
// Whether to set up the PCH/FDI. Note that we never allow sharing
// between pch encoders and cpu encoders.
    pub has_pch_encoder: bool,
// Are we sending infoframes on the attached port
    pub has_infoframe: bool,
// CPU Transcoder for the pipe. Currently this can only differ from the
// pipe on Haswell and later (where we have a special eDP transcoder)
// and Broxton (where we have special DSI transcoders).
    pub cpu_transcoder: transcoder,
//
// Use reduced/limited/broadcast rbg range, compressing from the full
// range fed into the crtcs.
//
    pub limited_color_range: bool,
// Bitmask of encoder types (enum intel_output_type)
// driven by the pipe.
//
    pub output_types: c_uint,
// Whether we should send NULL infoframes. Required for audio.
    pub has_hdmi_sink: bool,
// Audio enabled on this pipe. Only valid if either has_hdmi_sink or
// has_dp_encoder is set.
    pub has_audio: bool,
//
// Enable dithering, used when the selected pipe bpp doesn't match the
// plane bpp.
//
    pub dither: bool,
//
// Dither gets enabled for 18bpp which causes CRC mismatch errors for
// compliance video pattern tests.
// Disable dither only if it is a compliance test request for
// 18bpp.
//
    pub dither_force_disable: bool,
// Controls for the clock computation, to override various stages.
    pub clock_set: bool,
// SDVO TV has a bunch of special case. To make multifunction encoders
// work correctly, we need to track this at runtime.
    pub sdvo_tv_clock: bool,
//
// crtc bandwidth limit, don't increase pipe bpp or clock if not really
// required. This is set in the 2nd loop of calling encoder's
// ->compute_config if the first pick doesn't work out.
//
    pub bw_constrained: bool,
// Settings for the intel dpll used on pretty much everything but
// haswell.
    pub dpll: dpll,
// Selected dpll or NULL.
    pub intel_dpll: *mut intel_dpll,
// Actual register state of the dpll, for shared dpll cross-checking.
    pub dpll_hw_state: intel_dpll_hw_state,
//
// ICL reserved DPLLs for the CRTC/port. The active PLL is selected by
// setting shared_dpll and dpll_hw_state to one of these reserved ones.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icl_port_dpll {
    pub pll: *mut intel_dpll,
    pub hw_state: intel_dpll_hw_state,
    pub icl_port_dplls: [}; ICL_PORT_DPLL_COUNT],
// DSI PLL registers
    pub div: u32 ctrl,,
    pub dsi_pll: },
    pub /: *mut *mut int max_link_bpp_x16; / in 1/16 bpp units,
    pub /: *mut *mut int max_pipe_bpp; / in 1 bpp units,
    pub /: *mut *mut int pipe_bpp; / in 1 bpp units,
    pub min_hblank: c_int,
    pub dp_m_n: intel_link_m_n,
// m2_n2 for eDP downclock
    pub dp_m2_n2: intel_link_m_n,
    pub has_drrs: bool,
// PSR is supported but might not be enabled due the lack of enabled planes
    pub has_psr: bool,
    pub has_sel_update: bool,
    pub enable_psr2_sel_fetch: bool,
    pub enable_psr2_su_region_et: bool,
    pub req_psr2_sdp_prior_scanline: bool,
    pub has_panel_replay: bool,
    pub link_off_after_as_sdp_when_pr_active: bool,
    pub disable_as_sdp_when_pr_active: bool,
    pub wm_level_disabled: bool,
    pub pkg_c_latency_used: bool,
// Only used for state verification.
    pub panel_replay_dsc_support: intel_panel_replay_dsc_support,
    pub su_y_granularity: u16,
    pub active_non_psr_pipes: u8,
    pub entry_setup_frames: u8,
    pub no_psr_reason: *const c_char,
//
// Frequency the dpll for the port should run at. Differs from the
// adjusted dotclock e.g. for DP or 10/12bpc hdmi mode. This is also
// already multiplied by pixel_multiplier.
//
    pub port_clock: c_int,
// Used by SDVO (and if we ever fix it, HDMI).
    pub pixel_multiplier: unsigned,
// I915_MODE_FLAG_*
    pub mode_flags: u8,
    pub lane_count: u8,
//
// Used by platforms having DP/HDMI PHY with programmable lane
// latency optimization.
//
    pub lane_lat_optim_mask: u8,
// minimum acceptable voltage level
    pub min_voltage_level: u8,
// Panel fitter controls for gen2-gen4 + VLV
    pub control: u32,
    pub pgm_ratios: u32,
    pub lvds_border_bits: u32,
    pub gmch_pfit: },
// Panel fitter placement and size for Ironlake+
    pub casf: intel_casf,
    pub dst: drm_rect,
    pub enabled: bool,
    pub force_thru: bool,
    pub pch_pfit: },
// FDI configuration, only valid if has_pch_encoder is set.
    pub fdi_lanes: c_int,
    pub fdi_m_n: intel_link_m_n,
    pub ips_enabled: bool,
    pub crc_enabled: bool,
    pub double_wide: bool,
    pub scaler_state: intel_crtc_scaler_state,
// w/a for waiting 2 vblanks during crtc enable
    pub hsw_workaround_pipe: pipe,
    pub wm: intel_crtc_wm_state,
    pub min_cdclk: c_int,
    pub plane_min_cdclk: [c_int; I915_MAX_PLANES],
// for packed/planar CbCr
    pub data_rate: [u32; I915_MAX_PLANES],
// for planar Y
    pub data_rate_y: [u32; I915_MAX_PLANES],
// FIXME unify with data_rate[]?
    pub rel_data_rate: [u64; I915_MAX_PLANES],
    pub rel_data_rate_y: [u64; I915_MAX_PLANES],
// Gamma mode programmed on the pipe
    pub gamma_mode: u32,
// CSC mode programmed on the pipe
    pub csc_mode: u32,
// CHV CGM mode
    pub cgm_mode: u32,
}

// bitmask of logically enabled planes (enum plane_id)
// bitmask of actually visible planes (enum plane_id)
// bitmask of planes that will be updated during the commit
// bitmask of planes with async flip active
// HDMI scrambling status
// HDMI High TMDS char rate ratio
//
// Output format RGB/YCBCR etc., that is coming out
// at the end of the pipe.
//
// Sink output format RGB/YCBCR etc., that is going
// into the sink.
//
// enable pipe gamma?
// enable pipe csc?
// enable vlv/chv wgc csc?
// joiner pipe bitmask
// Display Stream compression state
// Only used for state computation, not read out from the HW.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dsc_slice_config {
    pub pipes_per_line: c_int,
    pub streams_per_pipe: c_int,
    pub slices_per_stream: c_int,
    pub slice_config: },
// Compressed Bpp in U6.4 format (first 4 bits for fractional part)
    pub compressed_bpp_x16: u16,
    pub config: drm_dsc_config,
    pub dsc: },
// DP tunnel used for BW allocation.
    pub dp_tunnel_ref: drm_dp_tunnel_ref,
// HSW+ linetime watermarks
    pub linetime: u16,
    pub ips_linetime: u16,
    pub enhanced_framing: bool,
//
// Forward Error Correction.
//
// Note: This will be false for 128b/132b, which will always have FEC
// enabled automatically.
//
    pub fec_enable: bool,
    pub sdp_split_enable: bool,
// Pointer to master transcoder in case of tiled displays
    pub master_transcoder: transcoder,
// Bitmask to indicate slaves attached
    pub sync_mode_slaves_mask: u16,
// Only valid on TGL+
    pub mst_master_transcoder: transcoder,
// For DSB based pipe updates
    pub dsb_commit: *mut *mut intel_dsb dsb_color,,
    pub use_dsb: bool,
    pub use_flipq: bool,
    pub psr2_man_track_ctl: u32,
    pub pipe_srcsz_early_tpt: u32,
    pub psr2_su_area: drm_rect,
// Variable Refresh Rate state
    pub in_range: bool enable,,
    pub pipeline_full: u8,
    pub guardband: u16 flipline, vmin, vmax,,
    pub vsync_start: u32 vsync_end,,
    pub enable: bool,
    pub vmax: u16 vmin,,
    pub slope: u16 guardband,,
    pub max_decrease: u16 max_increase,,
    pub vblank_target: u16,
    pub dc_balance: },
    pub vrr: },
// Content Match Refresh Rate state
    pub enable: bool,
    pub cmrr_m: u64 cmrr_n,,
    pub cmrr: },
// Stream Splitter for eDP MSO
    pub enable: bool,
    pub link_count: u8,
    pub pixel_overlap: u8,
    pub splitter: },
// for loading single buffered registers during vblank
    pub vblank_work: drm_vblank_work,
// LOBF flag
    pub has_lobf: bool,
// W2 window or 'set context latency' lines
    pub set_context_latency: u16,
    pub io_wake_lines: u8,
    pub fast_wake_lines: u8,
// LNL and beyond
    pub check_entry_lines: u8,
    pub aux_less_wake_lines: u8,
    pub silence_period_sym_clocks: u8,
    pub lfps_half_cycle_num_of_syms: u8,
    pub alpm_state: },
// to track changes in plane color blocks
    pub plane_color_changed: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_pipe_crc_source {
    INTEL_PIPE_CRC_SOURCE_NONE,
    INTEL_PIPE_CRC_SOURCE_PLANE1,
    INTEL_PIPE_CRC_SOURCE_PLANE2,
    INTEL_PIPE_CRC_SOURCE_PLANE3,
    INTEL_PIPE_CRC_SOURCE_PLANE4,
    INTEL_PIPE_CRC_SOURCE_PLANE5,
    INTEL_PIPE_CRC_SOURCE_PLANE6,
    INTEL_PIPE_CRC_SOURCE_PLANE7,
    INTEL_PIPE_CRC_SOURCE_PIPE,
// TV/DP on pre-gen5/vlv can't use the pipe source.
    INTEL_PIPE_CRC_SOURCE_TV,
    INTEL_PIPE_CRC_SOURCE_DP_B,
    INTEL_PIPE_CRC_SOURCE_DP_C,
    INTEL_PIPE_CRC_SOURCE_DP_D,
    INTEL_PIPE_CRC_SOURCE_AUTO,
    INTEL_PIPE_CRC_SOURCE_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drrs_refresh_rate {
    DRRS_REFRESH_RATE_HIGH,
    DRRS_REFRESH_RATE_LOW,
}

pub const INTEL_PIPE_CRC_ENTRIES_NR: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pipe_crc {
    pub lock: spinlock_t,
    pub skipped: c_int,
    pub source: intel_pipe_crc_source,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_flipq_id {
    INTEL_FLIPQ_PLANE_1,
    INTEL_FLIPQ_PLANE_2,
    INTEL_FLIPQ_PLANE_3,
    INTEL_FLIPQ_GENERAL,
    INTEL_FLIPQ_FAST,
    MAX_INTEL_FLIPQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_flipq {
    pub start_mmioaddr: u32,
    pub flipq_id: intel_flipq_id,
    pub tail: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_crtc {
    pub base: drm_crtc,
    pub pipe_head: list_head,
    pub pipe: pipe,
//
// Whether the crtc and the connected output pipeline is active. Implies
// that crtc->enabled is set, i.e. the current mode configuration has
// some outputs connected to this crtc.
//
    pub active: bool,
    pub plane_ids_mask: u8,
// I915_MODE_FLAG_*
    pub mode_flags: u8,
    pub vmax_vblank_start: u16,
    pub enabled_power_domains: intel_display_power_domain_set,
    pub hw_readout_power_domains: intel_display_power_domain_set,
    pub overlay: *mut intel_overlay,
    pub config: *mut intel_crtc_state,
// armed event for async flip
    pub flip_done_event: *mut drm_pending_vblank_event,
// armed event for DSB based updates
    pub dsb_event: *mut drm_pending_vblank_event,
// armed event for flip queue based updates
    pub flipq_event: *mut drm_pending_vblank_event,
// Access to these should be protected by display->irq.lock.
    pub cpu_fifo_underrun_disabled: bool,
    pub pch_fifo_underrun_disabled: bool,
    pub flipq: [intel_flipq; MAX_INTEL_FLIPQ],
// per-pipe watermark state
// watermarks currently being used
    pub ilk: intel_pipe_wm,
    pub vlv: vlv_wm_state,
    pub g4x: g4x_wm_state,
    pub active: },
    pub wm: },
    pub mutex: mutex,
    pub work: delayed_work,
    pub refresh_rate: drrs_refresh_rate,
    pub frontbuffer_bits: c_uint,
    pub busy_frontbuffer_bits: c_uint,
    pub cpu_transcoder: transcoder,
    pub m2_n2: intel_link_m_n m_n,,
    pub drrs: },
    pub flip_count: u64,
    pub dc_balance: },
    pub scanline_offset: c_int,
    pub start_vbl_count: unsigned,
    pub start_vbl_time: ktime_t,
    pub max_vbl: int min_vbl,,
    pub scanline_start: c_int,

    pub min: u64,
    pub max: u64,
    pub sum: u64,
    pub over: c_uint,
    pub /: *mut *mut unsigned int times[17]; / [1us, 16ms],
    pub vbl: },

    pub debug: },
// scalers available on this crtc
    pub num_scalers: c_int,
// for loading single buffered registers during vblank
    pub vblank_pm_qos: pm_qos_request,

    pub pipe_crc: intel_pipe_crc,

    pub vblank_psr_notify: bool,
    pub enabled: bool,
    pub cmtg: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_plane_error {
    pub surflive: u32 ctl, surf,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_plane {
    pub base: drm_plane,
    pub i9xx_plane: i9xx_plane_id,
    pub id: plane_id,
    pub pipe: pipe,
    pub need_async_flip_toggle_wa: bool,
    pub vtd_guard: u8,
    pub frontbuffer_bit: u32,
    pub size: u32 base, cntl,,
    pub cursor: },
    pub fbc: *mut intel_fbc,
//
// NOTE: Do not place new plane state fields here (e.g., when adding
// new plane properties).  New runtime state should now be placed in
// the intel_plane_state structure and accessed via plane_state.
//
    pub rotation): c_uint,
    pub rotation): c_uint,
    pub rotation): c_uint,
    pub color_plane): c_int,
    pub rotation): u64 modifier, unsigned int,
    pub modifier): *mut *mut bool (can_async_flip)(u64,
// Write all non-self arming plane registers
    pub plane_state): *const intel_plane_state,
// Write all self-arming plane registers
    pub plane_state): *const intel_plane_state,
// Disable the plane, must arm
    pub crtc_state): *const intel_crtc_state,
    pub error): *mut intel_plane_error,
    pub pipe): *mut *mut *mut bool (get_hw_state)(struct intel_plane plane, enum pipe,
    pub plane_state): *mut intel_plane_state,
    pub plane_state): *const *const u32 (surf_offset)(struct intel_plane_state,
    pub plane_state): *const intel_plane_state,
    pub async_flip): bool,
    pub plane): *mut *mut void (enable_flip_done)(struct intel_plane,
    pub plane): *mut *mut void (disable_flip_done)(struct intel_plane,
// For drm_panic
    pub plane): *mut *mut void (disable_tiling)(struct intel_plane,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_hdmi {
    pub hdmi_reg: intel_reg_t,
    pub type: drm_dp_dual_mode_type,
    pub max_tmds_clock: c_int,
    pub dp_dual_mode: },
    pub attached_connector: *mut intel_connector,
    pub cec_notifier: *mut cec_notifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_compliance_data {
    pub edid: c_ulong,
    pub video_pattern: u8,
    pub vdisplay: u16 hdisplay,,
    pub bpc: u8,
    pub phytest: drm_dp_phy_test_params,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_compliance {
    pub test_type: c_ulong,
    pub test_data: intel_dp_compliance_data,
    pub test_active: bool,
    pub test_link_rate: c_int,
    pub test_lane_count: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_pcon_frl {
    pub is_trained: bool,
    pub trained_rate_gbps: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_pps {
    pub panel_power_up_delay: c_int,
    pub panel_power_down_delay: c_int,
    pub panel_power_cycle_delay: c_int,
    pub backlight_on_delay: c_int,
    pub backlight_off_delay: c_int,
    pub panel_vdd_work: delayed_work,
    pub want_panel_vdd: bool,
    pub initializing: bool,
    pub last_power_on: c_ulong,
    pub last_backlight_off: c_ulong,
    pub panel_power_off_time: ktime_t,
    pub vdd_wakeref: *mut ref_tracker,
//
// Pipe whose power sequencer is currently locked into
// this port. Only relevant on VLV/CHV.
//
    pub vlv_pps_pipe: pipe,
//
// Power sequencer index. Only relevant on BXT+.
//
    pub pps_idx: c_int,
}

//
// Pipe currently driving the port. Used for preventing
// the use of the PPS for any pipe currentrly driving
// external DP as that will mess things up on VLV.
//
// Set if the sequencer may be reset due to a power transition,
// requiring a reinitialization. Only relevant on BXT+.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_psr {
// Mutex for PSR state of the transcoder
    pub lock: mutex,
pub const I915_PSR_DEBUG_MODE_MASK: c_uint = 0x0f;
pub const I915_PSR_DEBUG_DEFAULT: c_uint = 0x00;
pub const I915_PSR_DEBUG_DISABLE: c_uint = 0x01;
pub const I915_PSR_DEBUG_ENABLE: c_uint = 0x02;
pub const I915_PSR_DEBUG_FORCE_PSR1: c_uint = 0x03;
pub const I915_PSR_DEBUG_ENABLE_SEL_FETCH: c_uint = 0x4;
pub const I915_PSR_DEBUG_IRQ: c_uint = 0x10;
pub const I915_PSR_DEBUG_SU_REGION_ET_DISABLE: c_uint = 0x20;
pub const I915_PSR_DEBUG_PANEL_REPLAY_DISABLE: c_uint = 0x40;
    pub debug: u32,
    pub sink_support: bool,
    pub source_support: bool,
    pub enabled: bool,
    pub pause_counter: c_int,
    pub pipe: pipe,
    pub transcoder: transcoder,
    pub active: bool,
    pub work: work_struct,
    pub busy_frontbuffer_bits: c_uint,
    pub link_standby: bool,
    pub sel_update_enabled: bool,
    pub psr2_sel_fetch_enabled: bool,
    pub psr2_sel_fetch_cff_enabled: bool,
    pub su_region_et_enabled: bool,
    pub req_psr2_sdp_prior_scanline: bool,
    pub last_entry_attempt: ktime_t,
    pub last_exit: ktime_t,
    pub sink_not_reliable: bool,
    pub irq_aux_error: bool,
// DC3CO allowed used to control PSR configuration
    pub dc3co_allowed: bool,
// DC3CO disable work
    pub dc3co_work: delayed_work,
    pub su_w_granularity: u16,
    pub su_y_granularity: u16,
    pub source_panel_replay_support: bool,
    pub sink_panel_replay_support: bool,
    pub panel_replay_enabled: bool,
    pub dc3co_exit_delay: u32,
    pub entry_setup_frames: u8,
    pub io_wake_lines: u8,
    pub fast_wake_lines: u8,
    pub link_ok: bool,
    pub pkg_c_latency_used: bool,
    pub active_non_psr_pipes: u8,
    pub no_psr_reason: *const c_char,
    pub vblank_wakeref: *mut ref_tracker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_link_config {
    pub rate: c_int,
    pub lane_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp {
    pub output_reg: intel_reg_t,
    pub DP: u32,
    pub link_rate: c_int,
    pub lane_count: u8,
    pub sink_count: u8,
    pub downstream_port_changed: bool,
    pub needs_modeset_retry: bool,
    pub use_max_params: bool,
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub downstream_ports: [u8; DP_MAX_DOWNSTREAM_PORTS],
    pub edp_dpcd: [u8; EDP_DISPLAY_CTL_CAP_SIZE],
    pub lttpr_common_caps: [u8; DP_LTTPR_COMMON_CAP_SIZE],
    pub lttpr_phy_caps: [u8; DP_MAX_LTTPR_COUNT][DP_LTTPR_PHY_CAP_SIZE],
    pub pcon_dsc_dpcd: [u8; DP_PCON_DSC_ENCODER_CAP_SIZE],
// source rates
    pub num_source_rates: c_int,
    pub source_rates: *const c_int,
// sink rates as reported by DP_MAX_LINK_RATE/DP_SUPPORTED_LINK_RATES
    pub num_sink_rates: c_int,
    pub sink_rates: [c_int; DP_MAX_SUPPORTED_RATES],
    pub use_rate_select: bool,
// Max sink lane count as reported by DP_MAX_LANE_COUNT
    pub max_sink_lane_count: c_int,
// TODO: move the rest of link specific fields to here
    pub active: bool,
//
// Link parameters for which the MST topology was probed.
// Tracking these ensures that the MST path resources are
// re-enumerated whenever the link is retrained with new link
// parameters, as required by the DP standard.
//
    pub mst_probed_lane_count: c_int,
    pub mst_probed_rate: c_int,
    pub training: *mut intel_dp_link_training,
    pub caps: *mut intel_dp_link_caps,
    pub link: },
    pub reset_link_params: bool,
    pub mso_link_count: c_int,
    pub mso_pixel_overlap: c_int,
// sink or branch descriptor
    pub desc: drm_dp_desc,
    pub aux: drm_dp_aux,
    pub aux_busy_last_status: u32,
    pub train_set: [u8; 4],
    pub pps: intel_pps,
    pub is_mst: bool,
    pub mst_detect: drm_dp_mst_mode,
// connector directly attached - won't be use for modeset in mst world
    pub attached_connector: *mut intel_connector,
    pub as_sdp_supported: bool,
    pub as_sdp_v2_supported: bool,
    pub tunnel: *mut drm_dp_tunnel,
    pub tunnel_suspended:1: bool,
    pub disabled_uhbr_lane_mask: u8,
    pub stream_encoders: [*mut intel_dp_mst_encoder; I915_MAX_PIPES],
    pub mgr: drm_dp_mst_topology_mgr,
    pub active_streams: c_int,
    pub mst: },
    pub index): *mut *mut *mut u32 (get_aux_clock_divider)(struct intel_dp dp, int,
//
// This function returns the value we have to program the AUX_CTL
// register with to kick off an AUX transaction.
//
    pub aux_clock_divider): u32,
    pub dp): *mut *mut intel_reg_t (aux_ch_ctl_reg)(struct intel_dp,
    pub index): *mut *mut *mut intel_reg_t (aux_ch_data_reg)(struct intel_dp dp, int,
// This is called before a link training is starterd
    pub crtc_state): *const intel_crtc_state,
    pub dp_train_pat): u8,
    pub crtc_state): *const intel_crtc_state,
    pub intel_dp): *mut *mut u8 (preemph_max)(struct intel_dp,
    pub crtc_state): *const intel_crtc_state,
// Displayport compliance testing
    pub compliance: intel_dp_compliance,
// Downstream facing port caps
    pub max_tmds_clock: int min_tmds_clock,,
    pub max_dotclock: c_int,
    pub pcon_max_frl_bw: c_int,
    pub max_bpc: u8,
    pub ycbcr_444_to_420: bool,
    pub ycbcr420_passthrough: bool,
    pub rgb_to_ycbcr: bool,
    pub dfp: },
// To control wakeup latency, e.g. for irq-driven dp aux transfers.
    pub pm_qos: pm_qos_request,
// Display stream compression testing
    pub force_dsc_en: bool,
    pub force_dsc_output_format: c_int,
    pub force_dsc_fractional_bpp_en: bool,
    pub force_dsc_bpc: c_int,
    pub hobl_failed: bool,
    pub hobl_active: bool,
    pub frl: intel_dp_pcon_frl,
    pub psr: intel_psr,
// When we last wrote the OUI for eDP
    pub last_oui_write: c_ulong,
    pub oui_valid: bool,
    pub colorimetry_support: bool,
    pub transcoder: transcoder,
    pub lock: mutex,
    pub lobf_disable_debug: bool,
    pub sink_alpm_error: bool,
    pub alpm: },
    pub alpm_dpcd: u8,
    pub mask: c_ulong,
    pub quirks: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lspcon_vendor {
    LSPCON_VENDOR_MCA,
    LSPCON_VENDOR_PARADE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_lspcon {
    pub active: bool,
    pub hdr_supported: bool,
    pub mode: drm_lspcon_mode,
    pub vendor: lspcon_vendor,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_digital_port {
    pub base: intel_encoder,
    pub dp: intel_dp,
    pub hdmi: intel_hdmi,
    pub lspcon: intel_lspcon,
    pub bool): *mut *mut *mut irqreturn (hpd_pulse)(struct intel_digital_port ,,
    pub lane_reversal: bool,
    pub ddi_a_4_lanes: bool,
    pub release_cl2_override: bool,
    pub dedicated_external: bool,
    pub max_lanes: u8,
// Used for DP and ICL+ TypeC/DP and TypeC/HDMI ports.
    pub aux_ch: aux_ch,
    pub ddi_io_power_domain: intel_display_power_domain,
    pub ddi_io_wakeref: *mut ref_tracker,
    pub aux_wakeref: *mut ref_tracker,
    pub tc: *mut intel_tc_port,
// protects num_streams reference count, port_data and auth_status
    pub mutex: mutex,
// the number of pipes using HDCP signalling out of this port
    pub num_streams: c_uint,
// port HDCP auth status
    pub auth_status: bool,
// HDCP port data need to pass to security f/w
    pub port_data: hdcp_port_data,
// Whether the MST topology supports HDCP Type 1 Content
    pub mst_type1_capable: bool,
    pub hdcp: },
    pub len): *const *const void frame, ssize_t,
    pub len): *mut *mut void frame, ssize_t,
    pub conn_state): *const drm_connector_state,
    pub pipe_config): *const intel_crtc_state,
    pub encoder): *mut *mut bool (connected)(struct intel_encoder,
    pub dig_port): *mut *mut void (lock)(struct intel_digital_port,
    pub dig_port): *mut *mut void (unlock)(struct intel_digital_port,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_dp_mst_encoder {
    pub base: intel_encoder,
    pub pipe: pipe,
    pub primary: *mut intel_digital_port,
    pub connector: *mut intel_connector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_colorop {
    pub base: drm_colorop,
    pub id: intel_color_block,
}

extern "C" {
    pub fn enc_to_dig_port(_arg: intel_attached_encoder(connector)) -> return;
}
extern "C" {
    pub fn enc_to_intel_hdmi(_arg: intel_attached_encoder(connector)) -> return;
}
extern "C" {
    pub fn enc_to_intel_dp(_arg: intel_attached_encoder(connector)) -> return;
}
// Skip pure HDMI/DVI DDI encoders
extern "C" {
    pub fn intel_reg_valid(_arg: enc_to_intel_dp(encoder)->output_reg) -> return;
}
// See if the HDMI encoder is valid.
extern "C" {
    pub fn intel_reg_valid(_arg: enc_to_intel_hdmi(encoder)->hdmi_reg) -> return;
}
extern "C" {
    pub fn container_of(_arg: intel_dp, intel_digital_port: struct, _arg: dp) -> return;
}
extern "C" {
    pub fn container_of(_arg: intel_hdmi, intel_digital_port: struct, _arg: hdmi) -> return;
}
extern "C" {
    pub fn ERR_CAST(_arg: ret) -> return;
}
extern "C" {
    pub fn to_intel_plane_state(_arg: ret) -> return;
}
// intel_display.c
extern "C" {
    pub fn drm_atomic_crtc_needs_modeset(_arg: &crtc_state->uapi) -> return;
}
//
// Conversion functions/macros from various pointer types to struct
// intel_display pointer.
//

// Helper for generic association. Map types to conversion functions/macros.

// Convert various pointer types to struct intel_display pointer.

