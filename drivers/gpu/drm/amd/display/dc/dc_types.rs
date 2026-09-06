//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_types.h
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
// Copyright 2012-2026 Advanced Micro Devices, Inc.
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
// AND EdidUtility only needs a portion
// of this file, including the rest only
// causes additional issues.
//

// forward declarations
//
// Environment definitions
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dce_environment {
    DCE_ENV_PRODUCTION_DRV = 0,
// Emulation on FPGA, in "Maximus" System.
// This environment enforces that *only* DC registers accessed.
// (access to non-DC registers will hang FPGA)
    DCE_ENV_FPGA_MAXIMUS,
// Emulation on real HW or on FPGA. Used by Diagnostics, enforces
// requirements of Diagnostics team.
    DCE_ENV_DIAG,
//
// Guest VM system, DC HW may exist but is not virtualized and
// should not be used.  SW support for VDI only.
//
    DCE_ENV_VIRTUAL_HW
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_perf_trace {
    pub read_count: c_ulong,
    pub write_count: c_ulong,
    pub last_entry_read: c_ulong,
    pub last_entry_write: c_ulong,
}

pub const NUM_PIXEL_FORMATS: c_int = 10;
pub const DTBCLK_LIMIT: c_int = 2920;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tiling_mode {
    TILING_MODE_INVALID,
    TILING_MODE_LINEAR,
    TILING_MODE_TILED,
    TILING_MODE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum view_3d_format {
    VIEW_3D_FORMAT_NONE = 0,
    VIEW_3D_FORMAT_FRAME_SEQUENTIAL,
    VIEW_3D_FORMAT_SIDE_BY_SIDE,
    VIEW_3D_FORMAT_TOP_AND_BOTTOM,
    VIEW_3D_FORMAT_COUNT,
    VIEW_3D_FORMAT_FIRST = VIEW_3D_FORMAT_FRAME_SEQUENTIAL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum plane_stereo_format {
    PLANE_STEREO_FORMAT_NONE = 0,
    PLANE_STEREO_FORMAT_SIDE_BY_SIDE = 1,
    PLANE_STEREO_FORMAT_TOP_AND_BOTTOM = 2,
    PLANE_STEREO_FORMAT_FRAME_ALTERNATE = 3,
    PLANE_STEREO_FORMAT_ROW_INTERLEAVED = 5,
    PLANE_STEREO_FORMAT_COLUMN_INTERLEAVED = 6,
    PLANE_STEREO_FORMAT_CHECKER_BOARD = 7
}

// TODO: Find way to calculate number of bits
// Please increase if pixel_format enum increases
// num  from  PIXEL_FORMAT_INDEX8 to PIXEL_FORMAT_444BPP32
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_edid_connector_type {
    DC_EDID_CONNECTOR_UNKNOWN = 0,
    DC_EDID_CONNECTOR_ANALOG = 1,
    DC_EDID_CONNECTOR_DIGITAL = 10,
    DC_EDID_CONNECTOR_DVI = 11,
    DC_EDID_CONNECTOR_HDMIA = 12,
    DC_EDID_CONNECTOR_MDDI = 14,
    DC_EDID_CONNECTOR_DISPLAYPORT = 15
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_edid_status {
    EDID_OK,
    EDID_BAD_INPUT,
    EDID_NO_RESPONSE,
    EDID_BAD_CHECKSUM,
    EDID_THE_SAME,
    EDID_FALL_BACK,
    EDID_PARTIAL_VALID,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum act_return_status {
    ACT_SUCCESS,
    ACT_LINK_LOST,
    ACT_FAILED
}

// audio capability from EDID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cea_audio_mode {
    pub [6:3]*/: *mut *mut uint8_t format_code; / ucData[0],
    pub [2:0]*/: *mut *mut uint8_t channel_count; / ucData[0],
    pub ucData[1]*/: *mut *mut uint8_t sample_rate; /,
    pub LPCM*/: *mut *mut uint8_t sample_size; / for,
// for Audio Formats 2-8 (Max bit rate divided by 8 kHz)
    pub max_bit_rate: u8,
    pub 9-15*/: *mut *mut uint8_t audio_codec_vendor_specific; / for Audio Formats,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_edid {
    pub length: u32,
    pub raw_edid: [u8; DC_MAX_EDID_BUFFER_SIZE],
}

// When speaker location data block is not available, DEFAULT_SPEAKER_LOCATION
// is used. In this case we assume speaker location are: front left, front
// right and front center.
pub const DEFAULT_SPEAKER_LOCATION: c_int = 5;
pub const DC_MAX_AUDIO_DESC_COUNT: c_int = 16;
pub const AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_panel_patch {
    pub dppowerup_delay: c_uint,
    pub extra_t12_ms: c_uint,
    pub extra_delay_backlight_off: c_uint,
    pub extra_t7_ms: c_uint,
    pub skip_scdc_overwrite: c_uint,
    pub delay_ignore_msa: c_uint,
    pub disable_fec: c_uint,
    pub extra_t3_ms: c_uint,
    pub max_dsc_target_bpp_limit: c_uint,
    pub embedded_tiled_slave: c_uint,
    pub disable_fams: c_uint,
    pub hdmi_spe_handling: c_uint,
    pub block_420_Freesync: c_uint,
    pub block_10g: c_uint,
    pub hdmi_comp_manual: c_uint,
    pub hdmi_comp_auto: c_uint,
    pub force_frl: c_uint,
    pub vsdb_rcc_wa: c_uint,
    pub delay_hdmi_link_training: c_uint,
    pub skip_frl_pre_training: c_uint,
    pub skip_avmute: c_uint,
    pub skip_audio_sab_check: c_uint,
    pub mst_start_top_delay: c_uint,
    pub remove_sink_ext_caps: c_uint,
    pub disable_second_tile: bool,
    pub disable_colorimetry: c_uint,
    pub blankstream_before_otg_off: u8,
    pub oled_optimize_display_on: bool,
    pub force_mst_blocked_discovery: c_uint,
    pub wait_after_dpcd_poweroff_ms: c_uint,
}

//
// struct dc_edid_caps - Capabilities read from EDID.
// @analog: Whether the monitor is analog. Used by DVI-I handling.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_edid_caps {
// sink identification
    pub manufacturer_id: u16,
    pub product_id: u16,
    pub serial_number: u32,
    pub manufacture_week: u8,
    pub manufacture_year: u8,
    pub display_name: [u8; AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS],
// audio caps
    pub speaker_flags: u8,
    pub audio_mode_count: u32,
    pub audio_modes: [dc_cea_audio_mode; DC_MAX_AUDIO_DESC_COUNT],
    pub audio_latency: u32,
    pub video_latency: u32,
    pub freesync_vcp_code: c_uchar,
    pub qs_bit: u8,
    pub qy_bit: u8,
    pub max_tmds_clk_mhz: u32,
// HDMI 2.0 caps
    pub lte_340mcsc_scramble: bool,
    pub edid_hdmi: bool,
    pub hdr_supported: bool,
    pub rr_capable: bool,
    pub scdc_present: bool,
    pub analog: bool,
// HDMI 2.1 caps
    pub max_frl_rate: u8,
    pub frl_dsc_support: bool,
    pub frl_dsc_10bpc: bool,
    pub frl_dsc_12bpc: bool,
    pub frl_dsc_all_bpp: bool,
    pub frl_dsc_native_420: bool,
    pub frl_dsc_max_slices: u8,
    pub frl_dsc_max_frl_rate: u8,
    pub frl_dsc_total_chunk_kbytes: u8,
    pub panel_patch: dc_panel_patch,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_mode_flags {
// note: part of refresh rate flag
    pub :1: uint32_t INTERLACE,
// native display timing
    pub :1: uint32_t NATIVE,
// preferred is the recommended mode, one per display
    pub :1: uint32_t PREFERRED,
// true if this mode should use reduced blanking timings
// _not_ related to the Reduced Blanking adjustment
    pub :1: uint32_t REDUCED_BLANKING,
// note: part of refreshrate flag
    pub :1: uint32_t VIDEO_OPTIMIZED_RATE,
// should be reported to upper layers as mode_flags
    pub :1: uint32_t PACKED_PIXEL_FORMAT,
// < preferred view
    pub :1: uint32_t PREFERRED_VIEW,
// this timing should be used only in tiled mode
    pub :1: uint32_t TILED_MODE,
    pub :1: uint32_t DSE_MODE,
// Refresh rate divider when Miracast sink is using a
    pub MIRACAST_REFRESH_DIVIDER: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_timing_source {
    TIMING_SOURCE_UNDEFINED,

// explicitly specifed by user, most important
    TIMING_SOURCE_USER_FORCED,
    TIMING_SOURCE_USER_OVERRIDE,
    TIMING_SOURCE_CUSTOM,
    TIMING_SOURCE_EXPLICIT,

// explicitly specified by the display device, more important
    TIMING_SOURCE_EDID_CEA_SVD_3D,
    TIMING_SOURCE_EDID_CEA_SVD_PREFERRED,
    TIMING_SOURCE_EDID_CEA_SVD_420,
    TIMING_SOURCE_EDID_DETAILED,
    TIMING_SOURCE_EDID_ESTABLISHED,
    TIMING_SOURCE_EDID_STANDARD,
    TIMING_SOURCE_EDID_CEA_SVD,
    TIMING_SOURCE_EDID_CVT_3BYTE,
    TIMING_SOURCE_EDID_4BYTE,
    TIMING_SOURCE_EDID_CEA_DISPLAYID_VTDB,
    TIMING_SOURCE_EDID_CEA_RID,
    TIMING_SOURCE_EDID_DISPLAYID_TYPE5,
    TIMING_SOURCE_VBIOS,
    TIMING_SOURCE_CV,
    TIMING_SOURCE_TV,
    TIMING_SOURCE_HDMI_VIC,
    TIMING_SOURCE_CEA_VIC,

// implicitly specified by display device, still safe but less important
    TIMING_SOURCE_DEFAULT,

// only used for custom base modes
    TIMING_SOURCE_CUSTOM_BASE,

// these timing might not work, least important
    TIMING_SOURCE_RANGELIMIT,
    TIMING_SOURCE_OS_FORCED,
    TIMING_SOURCE_IMPLICIT,

// only used by default mode list
    TIMING_SOURCE_BASICMODE,

    TIMING_SOURCE_COUNT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stereo_3d_features {
    pub supported: bool,
    pub allTimings: bool,
    pub cloneMode: bool,
    pub scaling: bool,
    pub singleFrameSWPacked: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_timing_support_method {
    TIMING_SUPPORT_METHOD_UNDEFINED,
    TIMING_SUPPORT_METHOD_EXPLICIT,
    TIMING_SUPPORT_METHOD_IMPLICIT,
    TIMING_SUPPORT_METHOD_NATIVE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_mode_info {
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub field_rate: u32,
// Vertical refresh rate for progressive modes.
// Field rate for interlaced modes.
    pub timing_standard: dc_timing_standard,
    pub timing_source: dc_timing_source,
    pub flags: dc_mode_flags,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_power_state {
    DC_POWER_STATE_ON = 1,
    DC_POWER_STATE_STANDBY,
    DC_POWER_STATE_SUSPEND,
    DC_POWER_STATE_OFF
}

// DC PowerStates
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_video_power_state {
    DC_VIDEO_POWER_UNSPECIFIED = 0,
    DC_VIDEO_POWER_ON = 1,
    DC_VIDEO_POWER_STANDBY,
    DC_VIDEO_POWER_SUSPEND,
    DC_VIDEO_POWER_OFF,
    DC_VIDEO_POWER_HIBERNATE,
    DC_VIDEO_POWER_SHUTDOWN,
    DC_VIDEO_POWER_ULPS,	/* BACO or Ultra-Light-Power-State */
    DC_VIDEO_POWER_AFTER_RESET,
    DC_VIDEO_POWER_MAXIMUM
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_acpi_cm_power_state {
    DC_ACPI_CM_POWER_STATE_D0 = 1,
    DC_ACPI_CM_POWER_STATE_D1 = 2,
    DC_ACPI_CM_POWER_STATE_D2 = 4,
    DC_ACPI_CM_POWER_STATE_D3 = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_connection_type {
    dc_connection_none,
    dc_connection_single,
    dc_connection_mst_branch,
    dc_connection_sst_branch,
    dc_connection_analog_load
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_csc_adjustments {
    pub contrast: fixed31_32,
    pub saturation: fixed31_32,
    pub brightness: fixed31_32,
    pub hue: fixed31_32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_scaling_linearity {
    DC_SCALING_LINEARITY_LINEAR,
    DC_SCALING_LINEARITY_SOURCE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_blending_linearity {
    DC_BLENDING_LINEARITY_LINEAR,
    DC_BLENDING_LINEARITY_SOURCE,
}

// Scaling format
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scaling_transformation {
    SCALING_TRANSFORMATION_UNINITIALIZED,
    SCALING_TRANSFORMATION_IDENTITY = 0x0001,
    SCALING_TRANSFORMATION_CENTER_TIMING = 0x0002,
    SCALING_TRANSFORMATION_FULL_SCREEN_SCALE = 0x0004,
    SCALING_TRANSFORMATION_PRESERVE_ASPECT_RATIO_SCALE = 0x0008,
    SCALING_TRANSFORMATION_DAL_DECIDE = 0x0010,
    SCALING_TRANSFORMATION_INVALID = 0x80000000,

// Flag the first and last
    SCALING_TRANSFORMATION_BEGING = SCALING_TRANSFORMATION_IDENTITY,
    SCALING_TRANSFORMATION_END =
    SCALING_TRANSFORMATION_PRESERVE_ASPECT_RATIO_SCALE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum display_content_type {
    DISPLAY_CONTENT_TYPE_NO_DATA = 0,
    DISPLAY_CONTENT_TYPE_GRAPHICS = 1,
    DISPLAY_CONTENT_TYPE_PHOTO = 2,
    DISPLAY_CONTENT_TYPE_CINEMA = 4,
    DISPLAY_CONTENT_TYPE_GAME = 8
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cm_gamut_adjust_type {
    CM_GAMUT_ADJUST_TYPE_BYPASS = 0,
    CM_GAMUT_ADJUST_TYPE_HW, /* without adjustments */
    CM_GAMUT_ADJUST_TYPE_SW /* use adjustments */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cm_grph_csc_adjustment {
    pub temperature_matrix: [fixed31_32; 12],
    pub gamut_adjust_type: cm_gamut_adjust_type,
    pub gamut_coef_format: cm_gamut_coef_format,
}

// writeback
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dwb_stereo_params {
    pub /: *mut *mut bool stereo_enabled; / false: normal mode, true: 3D stereo,
    pub /: *mut *mut dwb_stereo_type stereo_type; / indicates stereo format,
    pub /: *mut *mut bool stereo_polarity; / indicates left eye or right eye comes first in stereo mode,
    pub /: *mut *mut dwb_stereo_eye_select stereo_eye_select; / indicate which eye should be captured,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dwb_cnv_params {
    pub /: *mut *mut unsigned int src_width; / input active width,
    pub /: *mut *mut unsigned int src_height; / input active height (half-active height in interlaced mode),
    pub /: *mut *mut unsigned int crop_width; / cropped window width at cnv output,
    pub /: *mut *mut bool crop_en; / window cropping enable in cnv,
    pub /: *mut *mut unsigned int crop_height; / cropped window height at cnv output,
    pub /: *mut *mut unsigned int crop_x; / cropped window start x value at cnv output,
    pub /: *mut *mut unsigned int crop_y; / cropped window start y value at cnv output,
    pub /: *mut *mut dwb_cnv_out_bpc cnv_out_bpc; / cnv output pixel depth - 8bpc or 10bpc,
    pub /: *mut *mut dwb_out_format fc_out_format; / dwb output pixel format - 2101010 or 16161616 and ARGB or RGBA,
    pub /: *mut *mut dwb_out_denorm out_denorm_mode;/ dwb output denormalization mode,
    pub /: *mut *mut unsigned int out_max_pix_val;/ pixel values greater than out_max_pix_val are clamped to out_max_pix_val,
    pub /: *mut *mut unsigned int out_min_pix_val;/ pixel values less than out_min_pix_val are clamped to out_min_pix_val,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dwb_params {
    pub /: *mut *mut unsigned int dwbscl_black_color; / must be in FP1.5.10,
    pub /: *mut *mut unsigned int hdr_mult; / must be in FP1.6.12,
    pub csc_params: cm_grph_csc_adjustment,
    pub stereo_params: dwb_stereo_params,
    pub /: *mut *mut dc_dwb_cnv_params cnv_params; / CNV source size and cropping window parameters,
    pub /: *mut *mut unsigned int dest_width; / Destination width,
    pub /: *mut *mut unsigned int dest_height; / Destination height,
    pub /: *mut *mut dwb_scaler_mode out_format; / default = YUV420 - TODO: limit this to 0 and 1 on dcn3,
    pub /: *mut *mut dwb_output_depth output_depth; / output pixel depth - 8bpc or 10bpc,
    pub /: *mut *mut dwb_capture_rate capture_rate; / controls the frame capture rate,
    pub /: *mut *mut scaling_taps scaler_taps; / Scaling taps,
    pub subsample_position: dwb_subsample_position,
    pub out_transfer_func: *const dc_transfer_func,
}

// audio
#[repr(C)]
#[derive(Copy, Clone)]
pub union audio_sample_rates {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sample_rates {
    pub RATE_32:1: u8,
    pub RATE_44_1:1: u8,
    pub RATE_48:1: u8,
    pub RATE_88_2:1: u8,
    pub RATE_96:1: u8,
    pub RATE_176_4:1: u8,
    pub RATE_192:1: u8,
    pub rate: },
    pub all: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_speaker_flags {
    pub FL_FR:1: u32,
    pub LFE:1: u32,
    pub FC:1: u32,
    pub RL_RR:1: u32,
    pub RC:1: u32,
    pub FLC_FRC:1: u32,
    pub RLC_RRC:1: u32,
    pub SUPPORT_AI:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_speaker_info {
    pub ALLSPEAKERS:7: u32,
    pub SUPPORT_AI:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_info_flags {
    pub speaker_flags: audio_speaker_flags,
    pub info: audio_speaker_info,
    pub all: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_format_code {
    AUDIO_FORMAT_CODE_FIRST = 1,
    AUDIO_FORMAT_CODE_LINEARPCM = AUDIO_FORMAT_CODE_FIRST,

    AUDIO_FORMAT_CODE_AC3,
// Layers 1 & 2
    AUDIO_FORMAT_CODE_MPEG1,
// MPEG1 Layer 3
    AUDIO_FORMAT_CODE_MP3,
// multichannel
    AUDIO_FORMAT_CODE_MPEG2,
    AUDIO_FORMAT_CODE_AAC,
    AUDIO_FORMAT_CODE_DTS,
    AUDIO_FORMAT_CODE_ATRAC,
    AUDIO_FORMAT_CODE_1BITAUDIO,
    AUDIO_FORMAT_CODE_DOLBYDIGITALPLUS,
    AUDIO_FORMAT_CODE_DTS_HD,
    AUDIO_FORMAT_CODE_MAT_MLP,
    AUDIO_FORMAT_CODE_DST,
    AUDIO_FORMAT_CODE_WMAPRO,
    AUDIO_FORMAT_CODE_LAST,
    AUDIO_FORMAT_CODE_COUNT =
    AUDIO_FORMAT_CODE_LAST - AUDIO_FORMAT_CODE_FIRST
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_mode {
// ucData[0] [6:3]
    pub format_code: audio_format_code,
// ucData[0] [2:0]
    pub channel_count: u8,
// ucData[1]
    pub sample_rates: audio_sample_rates,
// for LPCM
    pub sample_size: u8,
// for Audio Formats 2-8 (Max bit rate divided by 8 kHz)
    pub max_bit_rate: u8,
// for Audio Formats 9-15
    pub vendor_specific: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_info {
    pub flags: audio_info_flags,
    pub video_latency: u32,
    pub audio_latency: u32,
    pub display_index: u32,
    pub display_name: [u8; AUDIO_INFO_DISPLAY_NAME_SIZE_IN_CHARS],
    pub manufacture_id: u32,
    pub product_id: u32,
// PortID used for ContainerID when defined
    pub port_id: [u32; 2],
    pub mode_count: u32,
// this field must be last in this struct
    pub modes: [audio_mode; DC_MAX_AUDIO_DESC_COUNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_check {
    pub audio_packet_type: c_uint,
    pub max_audiosample_rate: c_uint,
    pub max_channel_count: c_uint,
    pub acat: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_infoframe_type {
    DC_HDMI_INFOFRAME_TYPE_VENDOR = 0x81,
    DC_HDMI_INFOFRAME_TYPE_AVI = 0x82,
    DC_HDMI_INFOFRAME_TYPE_SPD = 0x83,
    DC_HDMI_INFOFRAME_TYPE_AUDIO = 0x84,
    DC_DP_INFOFRAME_TYPE_PPS = 0x10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_info_packet {
    pub valid: bool,
    pub hb0: u8,
    pub hb1: u8,
    pub hb2: u8,
    pub hb3: u8,
    pub sb: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_info_packet_128 {
    pub valid: bool,
    pub hb0: u8,
    pub hb1: u8,
    pub hb2: u8,
    pub hb3: u8,
    pub sb: [u8; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_edid_read_policy {
    pub max_retry_count: u32,
    pub delay_time_ms: u32,
    pub ignore_checksum: u32,
}

pub const DC_PLANE_UPDATE_TIMES_MAX: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_flip_time {
    pub time_elapsed_in_us: [c_uint; DC_PLANE_UPDATE_TIMES_MAX],
    pub index: c_uint,
    pub prev_update_time_in_us: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_alpm_mode {
    DC_ALPM_AUXWAKE = 0,
    DC_ALPM_AUXLESS = 1,
    DC_ALPM_UNSUPPORTED = 0xF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_psr_state {
    PSR_STATE0 = 0x0,
    PSR_STATE1,
    PSR_STATE1a,
    PSR_STATE2,
    PSR_STATE2a,
    PSR_STATE2b,
    PSR_STATE3,
    PSR_STATE3Init,
    PSR_STATE4,
    PSR_STATE4a,
    PSR_STATE4b,
    PSR_STATE4c,
    PSR_STATE4d,
    PSR_STATE4_FULL_FRAME,
    PSR_STATE4a_FULL_FRAME,
    PSR_STATE4b_FULL_FRAME,
    PSR_STATE4c_FULL_FRAME,
    PSR_STATE4_FULL_FRAME_POWERUP,
    PSR_STATE4_FULL_FRAME_HW_LOCK,
    PSR_STATE5,
    PSR_STATE5a,
    PSR_STATE5b,
    PSR_STATE5c,
    PSR_STATE_HWLOCK_MGR,
    PSR_STATE_POLLVUPDATE,
    PSR_STATE_RELEASE_HWLOCK_MGR_FULL_FRAME,
    PSR_STATE_INVALID = 0xFF
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psr_config {
    pub psr_version: c_uchar,
    pub psr_rfb_setup_time: c_uint,
    pub psr_exit_link_training_required: bool,
    pub psr_frame_capture_indication_req: bool,
    pub psr_sdp_transmit_line_num_deadline: c_uint,
    pub allow_smu_optimizations: bool,
    pub allow_multi_disp_optimizations: bool,
// Panel self refresh 2 selective update granularity required
    pub su_granularity_required: bool,
// psr2 selective update y granularity capability
    pub su_y_granularity: u8,
    pub line_time_in_us: c_uint,
    pub rate_control_caps: u8,
    pub dsc_slice_height: u16,
    pub os_request_force_ffu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dmcu_psr_level {
    pub SKIP_CRC:1: c_uint,
    pub SKIP_DP_VID_STREAM_DISABLE:1: c_uint,
    pub SKIP_PHY_POWER_DOWN:1: c_uint,
    pub SKIP_AUX_ACK_CHECK:1: c_uint,
    pub SKIP_CRTC_DISABLE:1: c_uint,
    pub SKIP_AUX_RFB_CAPTURE_CHECK:1: c_uint,
    pub SKIP_SMU_NOTIFICATION:1: c_uint,
    pub SKIP_AUTO_STATE_ADVANCE:1: c_uint,
    pub DISABLE_PSR_ENTRY_ABORT:1: c_uint,
    pub SKIP_SINGLE_OTG_DISABLE:1: c_uint,
    pub DISABLE_ALPM:1: c_uint,
    pub ALPM_DEFAULT_PD_MODE:1: c_uint,
    pub RESERVED:20: c_uint,
    pub bits: },
    pub u32all: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum physical_phy_id {
    PHYLD_0,
    PHYLD_1,
    PHYLD_2,
    PHYLD_3,
    PHYLD_4,
    PHYLD_5,
    PHYLD_6,
    PHYLD_7,
    PHYLD_8,
    PHYLD_9,
    PHYLD_COUNT,
    PHYLD_UNKNOWN = (-1L)
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_type {
    PHY_TYPE_UNKNOWN  = 1,
    PHY_TYPE_PCIE_PHY = 2,
    PHY_TYPE_UNIPHY = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct psr_context {
// ddc line
    pub channel: channel_id,
// Transmitter id
    pub transmitterId: transmitter,
// Engine Id is used for Dig Be source select
    pub engineId: engine_id,
// Controller Id used for Dig Fe source select
    pub controllerId: controller_id,
// Pcie or Uniphy
    pub phyType: phy_type,
// Physical PHY Id used by SMU interpretation
    pub smuPhyId: physical_phy_id,
// Vertical total pixels from crtc timing.
// This is used for static screen detection.
// ie. If we want to detect half a frame,
// we use this to determine the hyst lines.
//
    pub crtcTimingVerticalTotal: c_uint,
// PSR supported from panel capabilities and
// current display configuration
//
    pub psrSupportedDisplayConfig: bool,
// Whether fast link training is supported by the panel
    pub psrExitLinkTrainingRequired: bool,
// If RFB setup time is greater than the total VBLANK time,
// it is not possible for the sink to capture the video frame
// in the same frame the SDP is sent. In this case,
// the frame capture indication bit should be set and an extra
// static frame should be transmitted to the sink.
//
    pub psrFrameCaptureIndicationReq: bool,
// Set the last possible line SDP may be transmitted without violating
// the RFB setup time or entering the active video frame.
//
    pub sdpTransmitLineNumDeadline: c_uint,
// The VSync rate in Hz used to calculate the
// step size for smooth brightness feature
//
    pub vsync_rate_hz: c_uint,
    pub skipPsrWaitForPllLock: c_uint,
    pub numberOfControllers: c_uint,
// Unused, for future use. To indicate that first changed frame from
// state3 shouldn't result in psr_inactive, but rather to perform
// an automatic single frame rfb_update.
//
    pub rfb_update_auto_en: bool,
// Number of frame before entering static screen
    pub timehyst_frames: c_uint,
// Partial frames before entering static screen
    pub hyst_lines: c_uint,
// # of repeated AUX transaction attempts to make before
// indicating failure to the driver
//
    pub aux_repeats: c_uint,
// Controls hw blocks to power down during PSR active state
    pub psr_level: dmcu_psr_level,
// Controls additional delay after remote frame capture before
// continuing powerd own
//
    pub frame_delay: c_uint,
    pub allow_smu_optimizations: bool,
    pub allow_multi_disp_optimizations: bool,
// Panel self refresh 2 selective update granularity required
    pub su_granularity_required: bool,
// psr2 selective update y granularity capability
    pub su_y_granularity: u8,
    pub line_time_in_us: c_uint,
    pub rate_control_caps: u8,
    pub dsc_slice_height: u16,
    pub os_request_force_ffu: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct colorspace_transform {
    pub matrix: [fixed31_32; 12],
    pub enable_remap: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum i2c_mot_mode {
    I2C_MOT_UNDEF,
    I2C_MOT_TRUE,
    I2C_MOT_FALSE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AsicStateEx {
    pub memoryClock: c_uint,
    pub displayClock: c_uint,
    pub engineClock: c_uint,
    pub maxSupportedDppClock: c_uint,
    pub dppClock: c_uint,
    pub socClock: c_uint,
    pub dcfClockDeepSleep: c_uint,
    pub fClock: c_uint,
    pub phyClock: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_clock_type {
    DC_CLOCK_TYPE_DISPCLK = 0,
    DC_CLOCK_TYPE_DPPCLK        = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_clock_config {
    pub max_clock_khz: u32,
    pub min_clock_khz: u32,
    pub bw_requirequired_clock_khz: u32,
    pub use*/: *mut *mut uint32_t current_clock_khz;/current clock in,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hubp_dmdata_mode {
    DMDATA_SW_MODE,
    DMDATA_HW_MODE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dmdata_attributes {
// Specifies whether dynamic meta data will be updated by software
// or has to be fetched by hardware (DMA mode)
//
    pub dmdata_mode: hubp_dmdata_mode,
// Specifies if current dynamic meta data is to be used only for the current frame
    pub dmdata_repeat: bool,
// Specifies the size of Dynamic Metadata surface in byte.  Size of 0 means no Dynamic metadata is fetched
    pub dmdata_size: u32,
// Specifies if a new dynamic meta data should be fetched for an upcoming frame
    pub dmdata_updated: bool,
// If hardware mode is used, the base address where DMDATA surface is located
    pub address: PHYSICAL_ADDRESS_LOC,
// Specifies whether QOS level will be provided by TTU or it will come from DMDATA_QOS_LEVEL
    pub dmdata_qos_mode: bool,
// If qos_mode = 1, this is the QOS value to be used:
    pub dmdata_qos_level: u32,
// Specifies the value in unit of REFCLK cycles to be added to the
// current time to produce the Amortized deadline for Dynamic Metadata chunk request
//
    pub dmdata_dl_delta: u32,
// An unbounded array of uint32s, represents software dmdata to be loaded
    pub dmdata_sw_data: *mut u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw_asic_id {
    pub chip_id: u32,
    pub chip_family: u32,
    pub pci_revision_id: u32,
    pub hw_internal_rev: u32,
    pub vram_type: u32,
    pub vram_width: u32,
    pub feature_flags: u32,
    pub fake_paths_num: u32,
    pub atombios_base_address: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_context {
    pub dc: *mut dc,
    pub /: *mut *mut *mut void driver_context; / e.g. amdgpu_device,
    pub logger: *mut dal_logger,
    pub perf_trace: *mut dc_perf_trace,
    pub cgs_device: *mut c_void,
    pub dce_environment: dce_environment,
    pub asic_id: hw_asic_id,
// todo: below should probably move to dc.  to facilitate removal
// of AS we will store these here
//
    pub dce_version: dce_version,
    pub dc_bios: *mut dc_bios,
    pub created_bios: bool,
    pub gpio_service: *mut gpio_service,
    pub dc_sink_id_count: u32,
    pub dc_stream_id_count: u32,
    pub dc_edp_id_count: u32,
    pub fbc_gpu_addr: u64,
    pub dmub_srv: *mut dc_dmub_srv,
    pub cp_psp: cp_psp,
    pub dcn_reg_offsets: *mut u32,
    pub nbio_reg_offsets: *mut u32,
    pub clk_reg_offsets: *mut u32,
}

// DSC DPCD capabilities
#[repr(C)]
#[derive(Copy, Clone)]
pub union dsc_slice_caps1 {
    pub 1: uint8_t NUM_SLICES_1 :,
    pub 1: uint8_t NUM_SLICES_2 :,
    pub 1: uint8_t RESERVED :,
    pub 1: uint8_t NUM_SLICES_4 :,
    pub 1: uint8_t NUM_SLICES_6 :,
    pub 1: uint8_t NUM_SLICES_8 :,
    pub 1: uint8_t NUM_SLICES_10 :,
    pub 1: uint8_t NUM_SLICES_12 :,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dsc_slice_caps2 {
    pub 1: uint8_t NUM_SLICES_16 :,
    pub 1: uint8_t NUM_SLICES_20 :,
    pub 1: uint8_t NUM_SLICES_24 :,
    pub 5: uint8_t RESERVED :,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dsc_color_formats {
    pub 1: uint8_t RGB :,
    pub 1: uint8_t YCBCR_444 :,
    pub 1: uint8_t YCBCR_SIMPLE_422 :,
    pub 1: uint8_t YCBCR_NATIVE_422 :,
    pub 1: uint8_t YCBCR_NATIVE_420 :,
    pub 3: uint8_t RESERVED :,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dsc_color_depth {
    pub 1: uint8_t RESERVED1 :,
    pub 1: uint8_t COLOR_DEPTH_8_BPC :,
    pub 1: uint8_t COLOR_DEPTH_10_BPC :,
    pub 1: uint8_t COLOR_DEPTH_12_BPC :,
    pub 3: uint8_t RESERVED2 :,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc_dec_dpcd_caps {
    pub is_dsc_supported: bool,
    pub dsc_version: u8,
    pub /: *mut *mut int32_t rc_buffer_size; / DSC RC buffer block size in bytes,
    pub slice_caps1: dsc_slice_caps1,
    pub slice_caps2: dsc_slice_caps2,
    pub lb_bit_depth: i32,
    pub is_block_pred_supported: bool,
    pub /: *mut *mut uint32_t edp_max_bits_per_pixel; / Valid only in eDP,
    pub color_formats: dsc_color_formats,
    pub color_depth: dsc_color_depth,
    pub /: *mut *mut int32_t throughput_mode_0_mps; / In MPs,
    pub /: *mut *mut int32_t throughput_mode_1_mps; / In MPs,
    pub max_slice_width: i32,
    pub /: *mut *mut uint32_t bpp_increment_div; / bpp increment divisor, e.g. if 16, it's 1/16th of a bit,
// Extended DSC caps
    pub /: *mut *mut uint32_t branch_overall_throughput_0_mps; / In MPs,
    pub /: *mut *mut uint32_t branch_overall_throughput_1_mps; / In MPs,
    pub branch_max_line_width: u32,
    pub /: *mut *mut bool is_frl; / Decoded format,
    pub is_vic_all_bpp: bool,
    pub total_chunk_kbytes: u32,
    pub /: *mut *mut bool is_dp; / Decoded format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hblank_expansion_dpcd_caps {
    pub expansion_supported: bool,
    pub reduction_supported: bool,
    pub pixels*/: *mut *mut bool buffer_unit_bytes; / True: buffer size in bytes. False: buffer size in,
    pub lane*/: *mut *mut bool buffer_per_port; / True: buffer size per port. False: buffer size per,
    pub /: *mut *mut uint32_t buffer_size; / Add 1 to value and multiply by 32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_golden_table {
    pub dc_golden_table_ver: u16,
    pub aux_dphy_rx_control0_val: u32,
    pub aux_dphy_tx_control_val: u32,
    pub aux_dphy_rx_control1_val: u32,
    pub dc_gpio_aux_ctrl_0_val: u32,
    pub dc_gpio_aux_ctrl_1_val: u32,
    pub dc_gpio_aux_ctrl_2_val: u32,
    pub dc_gpio_aux_ctrl_3_val: u32,
    pub dc_gpio_aux_ctrl_4_val: u32,
    pub dc_gpio_aux_ctrl_5_val: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_gpu_mem_alloc_type {
    DC_MEM_ALLOC_TYPE_GART,
    DC_MEM_ALLOC_TYPE_FRAME_BUFFER,
    DC_MEM_ALLOC_TYPE_INVISIBLE_FRAME_BUFFER,
    DC_MEM_ALLOC_TYPE_AGP
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_link_encoding_format {
    DC_LINK_ENCODING_UNSPECIFIED = 0,
    DC_LINK_ENCODING_DP_8b_10b,
    DC_LINK_ENCODING_DP_128b_132b,
    DC_LINK_ENCODING_HDMI_TMDS,
    DC_LINK_ENCODING_HDMI_FRL
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_psr_version {
    DC_PSR_VERSION_1			= 0,
    DC_PSR_VERSION_SU_1			= 1,
    DC_PSR_VERSION_UNSUPPORTED		= 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_replay_version {
    DC_FREESYNC_REPLAY = 0,
    DC_VESA_PANEL_REPLAY = 1,
    DC_REPLAY_VERSION_UNSUPPORTED = 0XFF,
}

// Possible values of display_endpoint_id.endpoint
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum display_endpoint_type {
    DISPLAY_ENDPOINT_PHY = 0, /* Physical connector. */
    DISPLAY_ENDPOINT_USB4_DPIA, /* USB4 DisplayPort tunnel. */
    DISPLAY_ENDPOINT_UNKNOWN = -1
}

// Extends graphics_object_id with an additional member 'ep_type' for
// distinguishing between physical endpoints (with entries in BIOS connector table) and
// logical endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_endpoint_id {
    pub link_id: graphics_object_id,
    pub ep_type: display_endpoint_type,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_panel_type {
    PANEL_TYPE_NONE = 0, // UNKONWN, not determined yet
    PANEL_TYPE_LCD = 1,
    PANEL_TYPE_OLED = 2,
    PANEL_TYPE_MINILED = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum backlight_control_type {
    BACKLIGHT_CONTROL_PWM = 0,
    BACKLIGHT_CONTROL_VESA_AUX = 1,
    BACKLIGHT_CONTROL_AMD_AUX = 2,
}

pub const MAX_CRC_WINDOW_NUM: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct otg_phy_mux {
    pub phy_output_num: u8,
    pub otg_output_num: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct crc_window {
    pub rect: rect,
    pub enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_detect_reason {
    DETECT_REASON_BOOT,
    DETECT_REASON_RESUMEFROMS3S4,
    DETECT_REASON_HPD,
    DETECT_REASON_HPDRX,
    DETECT_REASON_FALLBACK,
    DETECT_REASON_RETRAIN,
    DETECT_REASON_TDR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_link_status {
    pub link_active: bool,
    pub dpcd_caps: *mut dpcd_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdcp_rx_caps {
    pub version: u8,
    pub reserved: u8,
    pub 1: uint8_t repeater :,
    pub 1: uint8_t hdcp_capable :,
    pub 6: uint8_t reserved :,
    pub byte0: },
    pub fields: },
    pub raw: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union hdcp_bcaps {
    pub HDCP_CAPABLE:1: u8,
    pub REPEATER:1: u8,
    pub RESERVED:6: u8,
    pub bits: },
    pub raw: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdcp_caps {
    pub rx_caps: hdcp_rx_caps,
    pub bcaps: hdcp_bcaps,
}

// DP MST stream allocation (payload bandwidth number)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_mst_stream_allocation {
// DIG front
    pub stream_enc: *const stream_encoder,
// HPO DP Stream Encoder
    pub hpo_dp_stream_enc: *const hpo_dp_stream_encoder,
// associate DRM payload table with DC stream encoder
    pub vcp_id: u8,
// number of slots required for the DP stream in transport packet
    pub slot_count: u8,
}

pub const MAX_CONTROLLER_NUM: c_int = 6;
// DP MST stream allocation table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_mst_stream_allocation_table {
// number of DP video streams
    pub stream_count: c_int,
// array of stream allocations
    pub stream_allocations: [link_mst_stream_allocation; MAX_CONTROLLER_NUM],
}

// PSR feature flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psr_settings {
    pub sink: bool psr_feature_enabled; // PSR is supported by,
    pub active: bool psr_allow_active; // PSR is currently,
    pub DPCD: dc_psr_version psr_version; // Internal PSR version, determined based on,
    pub sink: bool psr_vtotal_control_support; // Vtotal control is supported by,
    pub PSR-SU: unsigned long long psr_dirty_rects_change_timestamp_ns; // for delay of enabling,
// These parameters are calculated in Driver,
// based on display timing and Sink capabilities.
// If VBLANK region is too small and Sink takes a long time
// to set up RFB, it may take an extra frame to enter PSR state.
//
    pub psr_frame_capture_indication_req: bool,
    pub psr_sdp_transmit_line_num_deadline: c_uint,
    pub force_ffu_mode: u8,
    pub psr_power_opt: c_uint,
//
// Some panels cannot handle idle pattern during PSR entry.
// To power down phy before disable stream to avoid sending
// idle pattern.
//
    pub power_down_phy_before_disable_stream: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_coasting_vtotal_type {
    PR_COASTING_TYPE_NOM = 0,
    PR_COASTING_TYPE_STATIC,
    PR_COASTING_TYPE_FULL_SCREEN_VIDEO,
    PR_COASTING_TYPE_TEST_HARNESS,
    PR_COASTING_TYPE_VIDEO_CONFERENCING_V2,
    PR_COASTING_TYPE_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_link_off_frame_count_level {
    PR_LINK_OFF_FRAME_COUNT_FAIL = 0x0,
    PR_LINK_OFF_FRAME_COUNT_GOOD = 0x2,
    PR_LINK_OFF_FRAME_COUNT_BEST = 0x6,
}

//
// This is general Interface for Replay to
// set an 32 bit variable to dmub
// The Message_type indicates which variable
// passed to DMUB.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum replay_FW_Message_type {
    Replay_Msg_Not_Support = -1,
    Replay_Set_Timing_Sync_Supported,
    Replay_Set_Residency_Frameupdate_Timer,
    Replay_Set_Pseudo_VTotal,
    Replay_Disabled_Adaptive_Sync_SDP,
    Replay_Set_General_Cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union replay_error_status {
    pub :1: unsigned int STATE_TRANSITION_ERROR,
    pub :1: unsigned int LINK_CRC_ERROR,
    pub :1: unsigned int DESYNC_ERROR,
    pub :1: unsigned int RESERVED_3,
    pub :1: unsigned int LOW_RR_INCORRECT_VTOTAL,
    pub :1: unsigned int NO_DOUBLED_RR,
    pub :2: unsigned int RESERVED_6_7,
    pub bits: },
    pub raw: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union replay_low_refresh_rate_enable_options {
// BIT[0-3]: Replay Low Hz Support control
    pub :1: unsigned int ENABLE_LOW_RR_SUPPORT,
    pub :1: unsigned int SKIP_ASIC_CHECK,
    pub :2: unsigned int RESERVED_2_3,
// BIT[4-15]: Replay Low Hz Enable Scenarios
    pub :1: unsigned int ENABLE_STATIC_SCREEN,
    pub :1: unsigned int ENABLE_FULL_SCREEN_VIDEO,
    pub :1: unsigned int ENABLE_GENERAL_UI,
    pub :9: unsigned int RESERVED_7_15,
// BIT[16-31]: Replay Low Hz Enable Check
    pub :1: unsigned int ENABLE_STATIC_FLICKER_CHECK,
    pub :15: unsigned int RESERVED_17_31,
    pub bits: },
    pub raw: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union replay_optimization {
// BIT[0-1]: Replay Teams Optimization
    pub :1: unsigned int TEAMS_OPTIMIZATION_VER_1,
    pub :1: unsigned int TEAMS_OPTIMIZATION_VER_2,
// BIT[2]: Replay Live Capture with CVT
    pub :1: unsigned int LIVE_CAPTURE_WITH_CVT,
    pub :1: unsigned int RESERVED_3,
    pub bits: },
    pub raw: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct replay_config {
// Replay version
    pub replay_version: dc_replay_version,
// Replay feature is supported
    pub replay_supported: bool,
// Replay caps support DPCD & EDID caps
    pub replay_cap_support: bool,
// Power opt flags that are supported
    pub replay_power_opt_supported: c_uint,
// SMU optimization is supported
    pub replay_smu_opt_supported: bool,
// Replay enablement option
    pub replay_enable_option: c_uint,
// Replay debug flags
    pub debug_flags: u32,
// Replay sync is supported
    pub replay_timing_sync_supported: bool,
// Replay Disable desync error check.
    pub force_disable_desync_error_check: bool,
// Replay Received Desync Error HPD.
    pub received_desync_error_hpd: bool,
// Replay feature is supported long vblank
    pub replay_support_fast_resync_in_ultra_sleep_mode: bool,
// Replay error status
    pub replay_error_status: replay_error_status,
// Replay Low Hz enable Options
    pub low_rr_enable_options: replay_low_refresh_rate_enable_options,
// Replay coasting vtotal is within low refresh rate range.
    pub low_rr_activated: bool,
// Replay low refresh rate supported
    pub low_rr_supported: bool,
// Replay Video Conferencing Optimization Enabled
    pub replay_video_conferencing_optimization_enabled: bool,
// Replay alpm mode
    pub alpm_mode: dc_alpm_mode,
// Replay full screen only
    pub os_request_force_ffu: bool,
// Replay optimization
    pub replay_optimization: replay_optimization,
// Replay sub feature Frame Skipping is supported
    pub frame_skip_supported: bool,
// Replay Received Frame Skipping Error HPD.
    pub received_frame_skipping_error_hpd: bool,
// Live capture with CVT is activated
    pub live_capture_with_cvt_activated: bool,
}

// Replay feature flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct replay_settings {
// Replay configuration
    pub config: replay_config,
// Replay feature is ready for activating
    pub replay_feature_enabled: bool,
// Replay is currently active
    pub replay_allow_active: bool,
// Replay is currently active
    pub replay_allow_long_vblank: bool,
// Power opt flags that are activated currently
    pub replay_power_opt_active: c_uint,
// SMU optimization is enabled
    pub replay_smu_opt_enable: bool,
// Current Coasting vtotal
    pub coasting_vtotal: u32,
// Coasting vtotal table
    pub coasting_vtotal_table: [u32; PR_COASTING_TYPE_NUM],
// Defer Update Coasting vtotal table
    pub defer_update_coasting_vtotal_table: [u32; PR_COASTING_TYPE_NUM],
// Skip frame number table
    pub frame_skip_number_table: [u32; PR_COASTING_TYPE_NUM],
// Defer skip frame number table
    pub defer_frame_skip_number_table: [u32; PR_COASTING_TYPE_NUM],
// Maximum link off frame count
    pub link_off_frame_count: u32,
// Replay pseudo vtotal for low refresh rate
    pub low_rr_full_screen_video_pseudo_vtotal: u16,
// Replay last pseudo vtotal set to DMUB
    pub last_pseudo_vtotal: u16,
// Replay desync error
    pub replay_desync_error_fail_count: u32,
// The frame skip number dal send to DMUB
    pub frame_skip_number: u16,
// Current Panel Replay events
    pub replay_events: u32,
}

// To split out "global" and "per-panel" config settings.
// Add a struct dc_panel_config under dc_link
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_panel_config {
// extra panel power sequence parameters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pps {
    pub extra_t3_ms: c_uint,
    pub extra_t7_ms: c_uint,
    pub extra_delay_backlight_off: c_uint,
    pub extra_post_t7_ms: c_uint,
    pub extra_pre_t11_ms: c_uint,
    pub extra_t12_ms: c_uint,
    pub extra_post_OUI_ms: c_uint,
    pub pps: },
// nit brightness
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nits_brightness {
    pub /: *mut *mut unsigned int peak; / nits,
    pub /: *mut *mut unsigned int max_avg; / nits,
    pub /: *mut *mut unsigned int min; / 1/10000 nits,
    pub max_nonboost_brightness_millinits: c_uint,
    pub min_brightness_millinits: c_uint,
    pub nits_brightness: },
// PSR/Replay
#[repr(C)]
#[derive(Copy, Clone)]
pub struct psr {
    pub disable_psr: bool,
    pub disallow_psrsu: bool,
    pub disallow_replay: bool,
    pub rc_disable: bool,
    pub rc_allow_static_screen: bool,
    pub rc_allow_fullscreen_VPB: bool,
    pub read_psrcap_again: bool,
    pub replay_enable_option: c_uint,
    pub enable_frame_skipping: bool,
    pub enable_teams_optimization: bool,
    pub psr: },
// ABM
#[repr(C)]
#[derive(Copy, Clone)]
pub struct varib {
    pub varibright_feature_enable: c_uint,
    pub def_varibright_level: c_uint,
    pub abm_config_setting: c_uint,
    pub varib: },
// edp DSC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsc {
    pub disable_dsc_edp: bool,
    pub force_dsc_edp_policy: c_uint,
    pub dsc: },
// eDP ILR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ilr {
    pub /: *mut *mut bool optimize_edp_link_rate; / eDP ILR,
    pub ilr: },
// CACP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cacp {
    pub cacp_supported: c_uint,
    pub cacp_control_mode: c_uint,
    pub strscl_valid: c_uint,
    pub strscl_sdr: [c_uint; 4],
    pub strscl_hdr: [c_uint; 4],
    pub cacp: },
// Adaptive VariBright
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adaptive_vb {
    pub disable_adaptive_vb: bool,
    pub 0xDCAA6414: unsigned int default_abm_vb_levels; // default value =,
    pub default_cacp_vb_levels: c_uint,
    pub 0xB4805A40: unsigned int default_abm_vb_hdr_levels; // default value =,
    pub default_cacp_vb_hdr_levels: c_uint,
    pub 0x23210012: unsigned int abm_scaling_factors; // default value =,
    pub cacp_scaling_factors: c_uint,
    pub 0x0A141E: unsigned int battery_life_configures; // default value =,
    pub 0x6A4F7244: unsigned int abm_backlight_adaptive_pwl_1; // default value =,
    pub 0x4C615659: unsigned int abm_backlight_adaptive_pwl_2; // default value =,
    pub 0x0064: unsigned int abm_backlight_adaptive_pwl_3; // default value =,
    pub cacp_backlight_adaptive_pwl_1: c_uint,
    pub cacp_backlight_adaptive_pwl_2: c_uint,
    pub cacp_backlight_adaptive_pwl_3: c_uint,
    pub adaptive_vb: },
// Ramless Idle Opt
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rio {
    pub disable_rio: bool,
    pub rio: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mccs_caps {
    pub freesync_supported: bool,
}

pub const MAX_SINKS_PER_LINK: c_int = 4;
//
// USB4 DPIA BW ALLOCATION STRUCTS
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dpia_bw_alloc {
    pub sinks: int remote_sink_req_bw[MAX_SINKS_PER_LINK]; // BW requested by remote,
    pub already: int link_verified_bw; // The Verified BW that link can allocated and use that has been verified,
    pub require/support: int link_max_bw; // The Max BW that link can,
    pub DPIA: int allocated_bw; // The Actual Allocated BW for this,
    pub DPIA: int estimated_bw; // The estimated available BW for this,
    pub Granularity: int bw_granularity; // BW,
    pub tunneling: int dp_overhead; // DP overhead in dp,
    pub CM: bool bw_alloc_enabled; // The BW Alloc Mode Support is turned ON for all 3: DP-Tx & Dpia &,
    pub count: uint8_t nrd_max_lane_count; // Non-reduced max lane,
    pub rate: uint8_t nrd_max_link_rate; // Non-reduced max link,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_hpd_enable_select {
    HPD_EN_FOR_ALL_EDP = 0,
    HPD_EN_FOR_PRIMARY_EDP_ONLY,
    HPD_EN_FOR_SECONDARY_EDP_ONLY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm_lut_swizzle {
    CM_LUT_3D_SWIZZLE_LINEAR_RGB,
    CM_LUT_3D_SWIZZLE_LINEAR_BGR,
    CM_LUT_1D_PACKED_LINEAR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm_lut_pixel_format {
    CM_LUT_PIXEL_FORMAT_RGBA16161616_UNORM_12MSB,
    CM_LUT_PIXEL_FORMAT_BGRA16161616_UNORM_12MSB,
    CM_LUT_PIXEL_FORMAT_RGBA16161616_UNORM_12LSB,
    CM_LUT_PIXEL_FORMAT_BGRA16161616_UNORM_12LSB,
    CM_LUT_PIXEL_FORMAT_RGBA16161616_FLOAT_FP1_5_10,
    CM_LUT_PIXEL_FORMAT_BGRA16161616_FLOAT_FP1_5_10
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm_lut_size {
    CM_LUT_SIZE_NONE,
    CM_LUT_SIZE_999,
    CM_LUT_SIZE_171717,
    CM_LUT_SIZE_333333,
    CM_LUT_SIZE_454545,
    CM_LUT_SIZE_656565,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm2_shaper_3dlut_setting {
    DC_CM2_SHAPER_3DLUT_SETTING_BYPASS_ALL,
    DC_CM2_SHAPER_3DLUT_SETTING_ENABLE_SHAPER,
// Bypassing Shaper will always bypass 3DLUT
    DC_CM2_SHAPER_3DLUT_SETTING_ENABLE_SHAPER_3DLUT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm2_gpu_mem_layout {
    DC_CM2_GPU_MEM_LAYOUT_3D_SWIZZLE_LINEAR_RGB,
    DC_CM2_GPU_MEM_LAYOUT_3D_SWIZZLE_LINEAR_BGR,
    DC_CM2_GPU_MEM_LAYOUT_1D_PACKED_LINEAR
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm2_gpu_mem_pixel_component_order {
    DC_CM2_GPU_MEM_PIXEL_COMPONENT_ORDER_RGBA,
    DC_CM2_GPU_MEM_PIXEL_COMPONENT_ORDER_BGRA
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm2_gpu_mem_format {
    DC_CM2_GPU_MEM_FORMAT_16161616_UNORM_12MSB,
    DC_CM2_GPU_MEM_FORMAT_16161616_UNORM_12LSB,
    DC_CM2_GPU_MEM_FORMAT_16161616_FLOAT_FP1_5_10
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm2_gpu_mem_size {
    DC_CM2_GPU_MEM_SIZE_171717,
    DC_CM2_GPU_MEM_SIZE_333333,
    DC_CM2_GPU_MEM_SIZE_454545,
    DC_CM2_GPU_MEM_SIZE_656565,
    DC_CM2_GPU_MEM_SIZE_TRANSFORMED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cm2_gpu_mem_format_parameters {
    pub format: dc_cm2_gpu_mem_format,
// bias & scale for float only
    pub bias: u16,
    pub scale: u16,
    pub float_params: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cm2_gpu_mem_parameters {
    pub addr: dc_plane_address,
    pub layout: dc_cm2_gpu_mem_layout,
    pub format_params: dc_cm2_gpu_mem_format_parameters,
    pub component_order: dc_cm2_gpu_mem_pixel_component_order,
    pub size: dc_cm2_gpu_mem_size,
    pub bit_depth: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_cm2_transfer_func_source {
    DC_CM2_TRANSFER_FUNC_SOURCE_SYSMEM,
    DC_CM2_TRANSFER_FUNC_SOURCE_VIDMEM
}

//
// All pointers in this struct must remain valid for as long as the 3DLUTs are used
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cm2_func_luts {
    pub shaper: *const dc_transfer_func,
    pub lut3d_src: dc_cm2_transfer_func_source,
    pub lut3d_func: *const dc_3dlut,
    pub gpu_mem_params: dc_cm2_gpu_mem_parameters,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mall_stream_type {
    SUBVP_NONE, // subvp not in use
    SUBVP_MAIN, // subvp in use, this stream is main stream
    SUBVP_PHANTOM, // subvp in use, this stream is a phantom stream
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_power_source_type {
    DC_POWER_SOURCE_AC, // wall power
    DC_POWER_SOURCE_DC, // battery power
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_state_create_params {
    pub power_source: dc_power_source_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_commit_streams_params {
    pub streams: *mut dc_stream_state,
    pub stream_count: u8,
    pub power_source: dc_power_source_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct set_backlight_level_params {
// backlight in pwm
    pub backlight_pwm_u16_16: u32,
// brightness ramping
    pub frame_ramp: u32,
// backlight control type
// 0: PWM backlight control
// 1: VESA AUX backlight control
// 2: AMD AUX backlight control
//
    pub control_type: backlight_control_type,
// backlight in millinits
    pub backlight_millinits: u32,
// transition time in ms
    pub transition_time_in_ms: u32,
// minimum luminance in nits
    pub min_luminance: u32,
// maximum luminance in nits
    pub max_luminance: u32,
// minimum backlight in pwm
    pub min_backlight_pwm: u32,
// maximum backlight in pwm
    pub max_backlight_pwm: u32,
// AUX HW instance
    pub aux_inst: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_validate_mode {
// validate the mode and program HW
    DC_VALIDATE_MODE_AND_PROGRAMMING = 0,
// only validate the mode
    DC_VALIDATE_MODE_ONLY = 1,
// validate the mode and get the max state (voltage level)
    DC_VALIDATE_MODE_AND_STATE_INDEX = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_validation_dpia_set {
    pub link: *const dc_link,
    pub tunnel_settings: *const dc_tunnel_settings,
    pub required_bw: u32,
}
