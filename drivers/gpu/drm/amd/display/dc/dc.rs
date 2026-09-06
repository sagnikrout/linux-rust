//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc.h
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

// forward declaration

//
// MAX_SURFACES - representative of the upper bound of surfaces that can be piped to a single CRTC
//
pub const MAX_SURFACES: c_int = 4;
//
// MAX_PLANES - representative of the upper bound of planes that are supported by the HW
//
pub const MAX_PLANES: c_int = 6;
pub const MAX_STREAMS: c_int = 6;
pub const MIN_VIEWPORT_SIZE: c_int = 12;
pub const MAX_NUM_EDP: c_int = 2;
pub const MAX_SUPPORTED_FORMATS: c_int = 7;
pub const MAX_HOST_ROUTERS_NUM: c_int = 3;
pub const MAX_DPIA_PER_HOST_ROUTER: c_int = 3;

pub const NUM_FAST_FLIPS_TO_STEADY_STATE: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_cap_chk_intermediates_fixed31_32 {
    pub c_frl_sb: c_int,
    pub overhead_sb: fixed31_32,
    pub overhead_rs: fixed31_32,
    pub overhead_map: fixed31_32,
    pub overhead_min: fixed31_32,
    pub overhead_max: fixed31_32,
    pub f_pixel_clock_max: fixed31_32,
    pub t_line: fixed31_32,
    pub r_bit_min: fixed31_32,
    pub r_frl_char_min: fixed31_32,
    pub c_frl_line: fixed31_32,
    pub ap: fixed31_32,
    pub r_ap: fixed31_32,
    pub avg_audio_packets_line: fixed31_32,
    pub margin: fixed31_32,
    pub audio_packets_line: c_int,
    pub blank_audio_min: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_cap_chk_params_fixed31_32 {
    pub lanes: c_int,
    pub /: *mut *mut fixed31_32 f_pixel_clock_nominal; / Pixel Clock rate (Hz),
    pub /: *mut *mut fixed31_32 r_bit_nominal; / FRL bitrate (bps),
    pub audio_packet_type: c_int,
    pub /: *mut *mut fixed31_32 f_audio; / Audio rate (Hz),
    pub /: *mut *mut int h_active; / Active pixels per line,
    pub /: *mut *mut int h_blank; / Blanking pixels per line,
    pub /: *mut *mut int bpc; / Bits per component,
    pub /: *mut *mut int vic; / Video Identification Code,
    pub pixel_encoding: hdmi_frl_pixel_encoding,
    pub /: *mut *mut bool compressed; / set to true if DSC is enabled,
    pub /: *mut *mut bool bypass_hc_target_calc; / debug only,
    pub /: *mut *mut bool allow_all_bpp; / dsc_all_bpp,
// DSC parameters
    pub slices: c_int,
    pub slice_width: c_int,
    pub bpp_target: fixed31_32,
    pub layout: c_int,
    pub /: *mut *mut int acat; / not supported,
// outputs
    pub borrow_params: frl_dml_borrow_params,
    pub average_tribyte_rate: c_int,
}

// Display Core Interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_versions {
    pub dc_ver: *const c_char,
    pub dmcu_version: dmcu_version,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_protocol_version {
    DP_VERSION_1_4 = 0,
    DP_VERSION_2_1,
    DP_VERSION_UNKNOWN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_plane_type {
    DC_PLANE_TYPE_INVALID,
    DC_PLANE_TYPE_DCE_RGB,
    DC_PLANE_TYPE_DCE_UNDERLAY,
    DC_PLANE_TYPE_DCN_UNIVERSAL,
}

// Sizes defined as multiples of 64KB
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum det_size {
    DET_SIZE_DEFAULT = 0,
    DET_SIZE_192KB = 3,
    DET_SIZE_256KB = 4,
    DET_SIZE_320KB = 5,
    DET_SIZE_384KB = 6
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_cap {
    pub type: dc_plane_type,
    pub 1: uint32_t per_pixel_alpha :,
    pub 1: uint32_t argb8888 :,
    pub 1: uint32_t nv12 :,
    pub 1: uint32_t fp16 :,
    pub 1: uint32_t p010 :,
    pub 1: uint32_t ayuv :,
    pub 8bpc: uint32_t yuy2 : 1; // Packed 422,
    pub 10bpc: uint32_t y210 : 1; // Packed 422,
    pub 12bpc: uint32_t y212 : 1; // Packed 422,
    pub 8bpc: uint32_t p208 : 1; // Planar 422,
    pub 10bpc: uint32_t p210 : 1; // Planar 422,
    pub 12bpc: uint32_t p212 : 1; // Planar 422,
// Not all caps will be used/supported
    pub pixel_format_support: },
// max upscaling factor x1000
// upscaling factors are always >= 1
// for example, 1080p -> 8K is 4.0, or 4000 raw value
    pub argb8888: u32,
    pub nv12: u32,
    pub fp16: u32,
    pub max_upscale_factor: },
// max downscale factor x1000
// downscale factors are always <= 1
// for example, 8K -> 1080p is 0.25, or 250 raw value
    pub argb8888: u32,
    pub nv12: u32,
    pub fp16: u32,
    pub max_downscale_factor: },
// minimal width/height
    pub min_width: u32,
    pub min_height: u32,
}

//
// DOC: color-management-caps
//
// **Color management caps (DPP and MPC)
//
// Modules/color calculates various color operations which are translated to
// abstracted HW. DCE 5-12 had almost no important changes, but starting with
// DCN1, every new generation comes with fairly major differences in color
// pipeline. Therefore, we abstract color pipe capabilities so modules/DM can
// decide mapping to HW block based on logical capabilities.
//
// struct rom_curve_caps - predefined transfer function caps for degamma and regamma
// @srgb: RGB color space transfer func
// @bt2020: BT.2020 transfer func
// @gamma2_2: standard gamma
// @pq: perceptual quantizer transfer function
// @hlg: hybrid log–gamma transfer function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rom_curve_caps {
    pub 1: uint16_t srgb :,
    pub 1: uint16_t bt2020 :,
    pub 1: uint16_t gamma2_2 :,
    pub 1: uint16_t pq :,
    pub 1: uint16_t hlg :,
}

//
// struct dpp_color_caps - color pipeline capabilities for display pipe and
// plane blocks
//
// @dcn_arch: all DCE generations treated the same
// @input_lut_shared: shared with DGAM. Input LUT is different than most LUTs,
// just plain 256-entry lookup
// @icsc: input color space conversion
// @dgam_ram: programmable degamma LUT
// @post_csc: post color space conversion, before gamut remap
// @gamma_corr: degamma correction
// @hw_3d_lut: 3D LUT support. It implies a shaper LUT before. It may be shared
// with MPC by setting mpc:shared_3d_lut flag
// @ogam_ram: programmable out/blend gamma LUT
// @ocsc: output color space conversion
// @dgam_rom_for_yuv: pre-defined degamma LUT for YUV planes
// @upsp_pre_scaler: Ability to upsample 420/422 before scaling
// @dgam_rom_caps: pre-definied curve caps for degamma 1D LUT
// @ogam_rom_caps: pre-definied curve caps for regamma 1D LUT
//
// Note: hdr_mult and gamut remap (CTM) are always available in DPP (in that order)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpp_color_caps {
    pub 1: uint16_t dcn_arch :,
    pub 1: uint16_t input_lut_shared :,
    pub 1: uint16_t icsc :,
    pub 1: uint16_t dgam_ram :,
    pub 1: uint16_t post_csc :,
    pub 1: uint16_t gamma_corr :,
    pub 1: uint16_t hw_3d_lut :,
    pub 1: uint16_t ogam_ram :,
    pub 1: uint16_t ocsc :,
    pub 1: uint16_t dgam_rom_for_yuv :,
    pub 1: uint16_t upsp_pre_scaler :,
    pub dgam_rom_caps: rom_curve_caps,
    pub ogam_rom_caps: rom_curve_caps,
}

// Below structure is to describe the HW support for mem layout, extend support
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lut3d_caps {
    pub /: *mut *mut uint32_t dma_3d_lut : 1; /< DMA mode support for 3D LUT,
    pub 1: uint32_t swizzle_3d_rgb :,
    pub 1: uint32_t swizzle_3d_bgr :,
    pub 1: uint32_t linear_1d :,
    pub mem_layout_support: },
    pub 1: uint32_t unorm_12msb :,
    pub 1: uint32_t unorm_12lsb :,
    pub 1: uint32_t float_fp1_5_10 :,
    pub mem_format_support: },
    pub 1: uint32_t order_rgba :,
    pub 1: uint32_t order_bgra :,
    pub mem_pixel_order_support: },
// < size options are 9, 17, 33, 45, 65
    pub /: *mut *mut uint32_t dim_9 : 1; / 3D LUT support for 9x9x9,
    pub /: *mut *mut uint32_t dim_17 : 1; / 3D LUT support for 17x17x17,
    pub /: *mut *mut uint32_t dim_33 : 1; / 3D LUT support for 33x33x33,
    pub /: *mut *mut uint32_t dim_45 : 1; / 3D LUT support for 45x45x45,
    pub /: *mut *mut uint32_t dim_65 : 1; / 3D LUT support for 65x65x65,
    pub lut_dim_caps: },
}

//
// struct mpc_color_caps - color pipeline capabilities for multiple pipe and
// plane combined blocks
//
// @gamut_remap: color transformation matrix
// @ogam_ram: programmable out gamma LUT
// @ocsc: output color space conversion matrix
// @num_3dluts: MPC 3D LUT; always assumes a preceding shaper LUT
// @num_rmcm_3dluts: number of RMCM 3D LUTS; always assumes a preceding shaper LUT
// @shared_3d_lut: shared 3D LUT flag. Can be either DPP or MPC, but single
// instance
// @ogam_rom_caps: pre-definied curve caps for regamma 1D LUT
// @mcm_3d_lut_caps: HW support cap for MCM LUT memory
// @rmcm_3d_lut_caps: HW support cap for RMCM LUT memory
// @preblend: whether color manager supports preblend with MPC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpc_color_caps {
    pub 1: uint16_t gamut_remap :,
    pub 1: uint16_t ogam_ram :,
    pub 1: uint16_t ocsc :,
    pub 3: uint16_t num_3dluts :,
    pub 3: uint16_t num_rmcm_3dluts :,
    pub shared_3d_lut:1: u16,
    pub ogam_rom_caps: rom_curve_caps,
    pub mcm_3d_lut_caps: lut3d_caps,
    pub rmcm_3d_lut_caps: lut3d_caps,
    pub preblend: bool,
    pub max_gamut_remap_coeff: fixed31_32,
}

//
// struct dc_color_caps - color pipes capabilities for DPP and MPC hw blocks
// @dpp: color pipes caps for DPP
// @mpc: color pipes caps for MPC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_color_caps {
    pub dpp: dpp_color_caps,
    pub mpc: mpc_color_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dmub_caps {
    pub psr: bool,
    pub mclk_sw: bool,
    pub subvp_psr: bool,
    pub gecc_enable: bool,
    pub fams_ver: u8,
    pub aux_backlight_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_scl_caps {
    pub sharpener_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_check_config {
//
// max video plane width that can be safely assumed to be always
// supported by single DPP pipe.
//
    pub max_optimizable_video_width: c_uint,
    pub enable_legacy_fast_update: bool,
    pub deferred_transition_state: bool,
    pub transition_countdown_to_steady_state: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_caps {
    pub max_streams: u32,
    pub max_links: u32,
    pub max_audios: u32,
    pub max_slave_planes: u32,
    pub max_slave_yuv_planes: u32,
    pub max_slave_rgb_planes: u32,
    pub max_planes: u32,
    pub max_downscale_ratio: u32,
    pub i2c_speed_in_khz: u32,
    pub i2c_speed_in_khz_hdcp: u32,
    pub dmdata_alloc_size: u32,
    pub max_cursor_size: c_uint,
    pub max_buffered_cursor_size: c_uint,
    pub max_video_width: c_uint,
    pub min_horizontal_blanking_period: c_uint,
    pub linear_pitch_alignment: c_int,
    pub dcc_const_color: bool,
    pub dynamic_audio: bool,
    pub is_apu: bool,
    pub dual_link_dvi: bool,
    pub post_blend_color_processing: bool,
    pub force_dp_tps4_for_cp2520: bool,
    pub disable_dp_clk_share: bool,
    pub psp_setup_panel_mode: bool,
    pub extended_aux_timeout_support: bool,
    pub dmcub_support: bool,
    pub zstate_support: bool,
    pub ips_support: bool,
    pub ips_v2_support: bool,
    pub num_of_internal_disp: u32,
    pub max_dp_protocol_version: dp_protocol_version,
    pub hdmi_hpo: bool,
    pub mall_size_per_mem_channel: c_uint,
    pub mall_size_total: c_uint,
    pub cursor_cache_size: c_uint,
    pub planes: [dc_plane_cap; MAX_PLANES],
    pub color: dc_color_caps,
    pub dmub_caps: dc_dmub_caps,
    pub dp_hpo: bool,
    pub dp_hdmi21_pcon_support: bool,
    pub edp_dsc_support: bool,
    pub vbios_lttpr_aware: bool,
    pub vbios_lttpr_enable: bool,
    pub fused_io_supported: bool,
    pub max_otg_num: u32,
    pub max_cab_allocation_bytes: u32,
    pub cache_line_size: u32,
    pub cache_num_ways: u32,
    pub subvp_fw_processing_delay_us: u16,
    pub subvp_drr_max_vblank_margin_us: u8,
    pub subvp_prefetch_end_to_mall_start_us: u16,
    pub height: uint8_t subvp_swath_height_margin_lines; // subvp start line must be aligned to 2 x swath,
    pub subvp_pstate_allow_width_us: u16,
    pub subvp_vertical_int_margin_us: u16,
    pub seamless_odm: bool,
    pub max_v_total: u32,
    pub vtotal_limited_by_fp2: bool,
    pub max_disp_clock_khz_at_vmin: u32,
    pub subvp_drr_vblank_start_margin_us: u8,
    pub cursor_not_scaled: bool,
    pub dcmode_power_limits_present: bool,
    pub sequential_ono: bool,
// Conservative limit for DCC cases which require ODM4:1 to support
    pub dcc_plane_width_limit: u32,
    pub scl_caps: dc_scl_caps,
    pub num_of_host_routers: u8,
    pub num_of_dpias_per_host_router: u8,
// limit of the ODM only, could be limited by other factors (like pipe count)
    pub max_odm_combine_factor: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_bug_wa {
    pub no_connect_phy_config: bool,
    pub dedcn20_305_wa: bool,
    pub skip_clock_update: bool,
    pub lt_early_cr_pattern: bool,
    pub 1: uint8_t uclk :,
    pub 1: uint8_t fclk :,
    pub 1: uint8_t dcfclk :,
    pub 1: uint8_t dcfclk_ds:,
    pub clock_update_disable_mask: },
    pub skip_psr_ips_crtc_disable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dcc_surface_param {
    pub surface_size: dc_size,
    pub format: surface_pixel_format,
    pub plane0_pitch: c_uint,
    pub plane1_size: dc_size,
    pub plane1_pitch: c_uint,
    pub swizzle_mode: swizzle_mode_values,
    pub swizzle_mode_addr3: swizzle_mode_addr3_values,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_dcc_setting {
    pub max_compressed_blk_size: c_uint,
    pub max_uncompressed_blk_size: c_uint,
    pub independent_64b_blks: bool,
// These bitfields to be used starting with DCN 3.0
    pub case): uint32_t dcc_256_64_64 : 1;//available in ASICs before DCN 3.0 (the worst compression,
    pub 3.0: uint32_t dcc_128_128_uncontrained : 1; //available in ASICs before DCN,
    pub 3.0: uint32_t dcc_256_128_128 : 1; //available starting with DCN,
    pub case): uint32_t dcc_256_256_unconstrained : 1; //available in ASICs before DCN 3.0 (the best compression,
    pub case): uint32_t dcc_256_256 : 1; //available in ASICs starting with DCN 4.0x (the best compression,
    pub 4.0x: uint32_t dcc_256_128 : 1; //available in ASICs starting with DCN,
    pub case): uint32_t dcc_256_64 : 1; //available in ASICs starting with DCN 4.0x (the worst compression,
    pub dcc_controls: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_surface_dcc_cap {
    pub rgb: dc_dcc_setting,
    pub grph: },
    pub luma: dc_dcc_setting,
    pub chroma: dc_dcc_setting,
    pub video: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_static_screen_params {
    pub force_trigger: bool,
    pub cursor_update: bool,
    pub surface_update: bool,
    pub overlay_update: bool,
    pub triggers: },
    pub num_frames: c_uint,
}

// Surface update type is used by dc_update_surfaces_and_stream
// The update type is determined at the very beginning of the function based
// on parameters passed in and decides how much programming (or updating) is
// going to be done during the call.
//
// UPDATE_TYPE_FAST is used for really fast updates that do not require much
// logical calculations or hardware register programming. This update MUST be
// ISR safe on windows. Currently fast update will only be used to flip surface
// address.
//
// UPDATE_TYPE_MED is used for slower updates which require significant hw
// re-programming however do not affect bandwidth consumption or clock
// requirements. At present, this is the level at which front end updates
// that do not require us to run bw_calcs happen. These are in/out transfer func
// updates, viewport offset changes, recout size changes and pixel depth changes.
// This update can be done at ISR, but we want to minimize how often this happens.
//
// UPDATE_TYPE_FULL is slow. Really slow. This requires us to recalculate our
// bandwidth and clocks, possibly rearrange some pipes and reprogram anything front
// end related. Any time viewport dimensions, recout dimensions, scaling ratios or
// gamma need to be adjusted or pipe needs to be turned on (or disconnected) we do
// a full update. This cannot be done at ISR level and should be a rare event.
// Unless someone is stress testing mpo enter/exit, playing with colour or adjusting
// underscan we don't expect to see this call at all.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_update_type {
    UPDATE_TYPE_FAST, /* super fast, safe to execute in isr */
    UPDATE_TYPE_MED,  /* ISR safe, most of programming needed, no bw/clk change*/
    UPDATE_TYPE_FULL, /* may need to shuffle resources */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_lock_descriptor {
    LOCK_DESCRIPTOR_NONE = 0x0,
    LOCK_DESCRIPTOR_STREAM = 0x1,
    LOCK_DESCRIPTOR_LINK = 0x2,
    LOCK_DESCRIPTOR_GLOBAL = 0x4,
    LOCK_DESCRIPTOR_PROBE = 0x8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_update_descriptor {
    pub update_type: dc_update_type,
    pub lock_descriptor: dc_lock_descriptor,
}

// Forward declaration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cap_funcs {
    pub output): *mut dc_surface_dcc_cap,
    pub context): *mut *mut *mut bool (get_subvp_en)(struct dc dc, struct dc_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union allow_lttpr_non_transparent_mode {
    pub 1: bool DP1_4A :,
    pub 1: bool DP2_0 :,
    pub bits: },
    pub raw: c_uchar,
}

// Structure to hold configuration flags set by dm at dc creation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_config {
    pub gpu_vm_support: bool,
    pub disable_disp_pll_sharing: bool,
    pub fbc_support: bool,
    pub disable_fractional_pwm: bool,
    pub allow_seamless_boot_optimization: bool,
    pub seamless_boot_edp_requested: bool,
    pub edp_not_connected: bool,
    pub edp_no_power_sequencing: bool,
    pub force_enum_edp: bool,
    pub forced_clocks: bool,
    pub allow_lttpr_non_transparent_mode: allow_lttpr_non_transparent_mode,
    pub multi_mon_pp_mclk_switch: bool,
    pub disable_dmcu: bool,
    pub allow_4to1MPC: bool,
    pub enable_windowed_mpo_odm: bool,
    pub CP2520: bool forceHBR2CP2520; // Used for switching between test patterns TPS4 and,
    pub allow_edp_hotplug_detection: u32,
    pub skip_riommu_prefetch_wa: bool,
    pub clamp_min_dcfclk: bool,
    pub vblank_alignment_dto_params: u64,
    pub vblank_alignment_max_frame_time_diff: u8,
    pub is_asymmetric_memory: bool,
    pub is_single_rank_dimm: bool,
    pub is_vmin_only_asic: bool,
    pub use_spl: bool,
    pub prefer_easf: bool,
    pub use_pipe_ctx_sync_logic: bool,
    pub smart_mux_version: c_int,
    pub ignore_dpref_ss: bool,
    pub enable_mipi_converter_optimization: bool,
    pub enable_frl: bool,
    pub force_hdmi21_frl_enc_enable: bool,
    pub use_default_clock_table: bool,
    pub force_bios_enable_lttpr: bool,
    pub force_bios_fixed_vs: u8,
    pub sdpif_request_limit_words_per_umc: c_uint,
    pub dc_mode_clk_limit_support: bool,
    pub EnableMinDispClkODM: bool,
    pub enable_auto_dpm_test_logs: bool,
    pub disable_ips: c_uint,
    pub disable_ips_rcg: c_uint,
    pub disable_ips_in_vpb: c_uint,
    pub disable_ips_in_dpms_off: bool,
    pub usb4_bw_alloc_support: bool,
    pub allow_0_dtb_clk: bool,
    pub use_assr_psp_message: bool,
    pub support_edp0_on_dp1: bool,
    pub enable_fpo_flicker_detection: c_uint,
    pub disable_hbr_audio_dp2: bool,
    pub consolidated_dpia_dp_lt: bool,
    pub set_pipe_unlock_order: bool,
    pub enable_dpia_pre_training: bool,
    pub unify_link_enc_assignment: bool,
    pub enable_cursor_offload: bool,
    pub dp_connector_no_native_i2c: bool,
    pub link_index_with_no_ddc: c_uint,
    pub frame_update_cmd_version2: bool,
    pub dcn_sharpness_range: spl_sharpness_range,
    pub dcn_override_sharpness_range: spl_sharpness_range,
    pub no_native422_support: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum visual_confirm {
    VISUAL_CONFIRM_DISABLE = 0,
    VISUAL_CONFIRM_SURFACE = 1,
    VISUAL_CONFIRM_HDR = 2,
    VISUAL_CONFIRM_MPCTREE = 4,
    VISUAL_CONFIRM_PSR = 5,
    VISUAL_CONFIRM_SWAPCHAIN = 6,
    VISUAL_CONFIRM_FAMS = 7,
    VISUAL_CONFIRM_SWIZZLE = 9,
    VISUAL_CONFIRM_SMARTMUX_DGPU = 10,
    VISUAL_CONFIRM_REPLAY = 12,
    VISUAL_CONFIRM_SUBVP = 14,
    VISUAL_CONFIRM_ABM = 15,
    VISUAL_CONFIRM_MCLK_SWITCH = 16,
    VISUAL_CONFIRM_FAMS2 = 19,
    VISUAL_CONFIRM_HW_CURSOR = 20,
    VISUAL_CONFIRM_VABC = 21,
    VISUAL_CONFIRM_DCC = 22,
    VISUAL_CONFIRM_BOOSTED_REFRESH_RATE = 23,
    VISUAL_CONFIRM_EXPLICIT = 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_psr_power_opts {
    psr_power_opt_invalid = 0x0,
    psr_power_opt_smu_opt_static_screen = 0x1,
    psr_power_opt_z10_static_screen = 0x10,
    psr_power_opt_ds_disable_allow = 0x100,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dml_hostvm_override_opts {
    DML_HOSTVM_NO_OVERRIDE = 0x0,
    DML_HOSTVM_OVERRIDE_FALSE = 0x1,
    DML_HOSTVM_OVERRIDE_TRUE = 0x2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_replay_power_opts {
    replay_power_opt_invalid		= 0x0,
    replay_power_opt_smu_opt_static_screen	= 0x1,
    replay_power_opt_z10_static_screen	= 0x10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcc_option {
    DCC_ENABLE = 0,
    DCC_DISABLE = 1,
    DCC_HALF_REQ_DISALBE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum in_game_fams_config {
    INGAME_FAMS_SINGLE_DISP_ENABLE, // enable in-game fams
    INGAME_FAMS_DISABLE, // disable in-game fams
    INGAME_FAMS_MULTI_DISP_ENABLE, //enable in-game fams for multi-display
    INGAME_FAMS_MULTI_DISP_CLAMPED_ONLY, //enable in-game fams for multi-display only for clamped RR strategies
}

//
// enum pipe_split_policy - Pipe split strategy supported by DCN
//
// This enum is used to define the pipe split policy supported by DCN. By
// default, DC favors MPC_SPLIT_DYNAMIC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pipe_split_policy {
//
// @MPC_SPLIT_DYNAMIC: DC will automatically decide how to split the
// pipe in order to bring the best trade-off between performance and
// power consumption. This is the recommended option.
//
    MPC_SPLIT_DYNAMIC = 0,

//
// @MPC_SPLIT_AVOID: Avoid pipe split, which means that DC will not
// try any sort of split optimization.
//
    MPC_SPLIT_AVOID = 1,

//
// @MPC_SPLIT_AVOID_MULT_DISP: With this option, DC will only try to
// optimize the pipe utilization when using a single display; if the
// user connects to a second display, DC will avoid pipe split.
//
    MPC_SPLIT_AVOID_MULT_DISP = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wm_report_mode {
    WM_REPORT_DEFAULT = 0,
    WM_REPORT_OVERRIDE = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dtm_pstate {
    dtm_level_p0 = 0,/*highest voltage*/
    dtm_level_p1,
    dtm_level_p2,
    dtm_level_p3,
    dtm_level_p4,/*when active_display_count = 0*/
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn_pwr_state {
    DCN_PWR_STATE_UNKNOWN = -1,
    DCN_PWR_STATE_MISSION_MODE = 0,
    DCN_PWR_STATE_LOW_POWER = 3,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dcn_zstate_support_state {
    DCN_ZSTATE_SUPPORT_UNKNOWN,
    DCN_ZSTATE_SUPPORT_ALLOW,
    DCN_ZSTATE_SUPPORT_ALLOW_Z8_ONLY,
    DCN_ZSTATE_SUPPORT_ALLOW_Z8_Z10_ONLY,
    DCN_ZSTATE_SUPPORT_ALLOW_Z10_ONLY,
    DCN_ZSTATE_SUPPORT_DISALLOW,
}

//
// struct dc_clocks - DC pipe clocks
//
// For any clocks that may differ per pipe only the max is stored in this
// structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_clocks {
    pub dispclk_khz: c_int,
    pub actual_dispclk_khz: c_int,
    pub dppclk_khz: c_int,
    pub actual_dppclk_khz: c_int,
    pub disp_dpp_voltage_level_khz: c_int,
    pub dcfclk_khz: c_int,
    pub socclk_khz: c_int,
    pub dcfclk_deep_sleep_khz: c_int,
    pub fclk_khz: c_int,
    pub phyclk_khz: c_int,
    pub dramclk_khz: c_int,
    pub p_state_change_support: bool,
    pub zstate_support: dcn_zstate_support_state,
    pub dtbclk_en: bool,
    pub ref_dtbclk_khz: c_int,
    pub fclk_p_state_change_support: bool,
    pub pwr_state: dcn_pwr_state,
//
// Elements below are not compared for the purposes of
// optimization required
//
// @cstate_allow
//
// DCN's DF C-state vote as last successfully acknowledged by PMFW.
// false = DCN does NOT permit DF C-state;
// true = DCN permits DF C-state;
//
    pub cstate_allow: bool,
    pub prev_p_state_change_support: bool,
    pub fclk_prev_p_state_change_support: bool,
    pub num_ways: c_int,
    pub host_router_bw_kbps: [c_int; MAX_HOST_ROUTERS_NUM],
//
// @fw_based_mclk_switching
//
// DC has a mechanism that leverage the variable refresh rate to switch
// memory clock in cases that we have a large latency to achieve the
// memory clock change and a short vblank window. DC has some
// requirements to enable this feature, and this field describes if the
// system support or not such a feature.
//
    pub fw_based_mclk_switching: bool,
    pub fw_based_mclk_switching_shut_down: bool,
    pub alt_ch_pstate_switch: bool,
    pub prev_num_ways: c_int,
    pub dtm_level: dtm_pstate,
    pub max_supported_dppclk_khz: c_int,
    pub max_supported_dispclk_khz: c_int,
    pub dppclk_khz*/: *mut *mut int bw_dppclk_khz; /a copy of,
    pub bw_dispclk_khz: c_int,
    pub idle_dramclk_khz: c_int,
    pub idle_fclk_khz: c_int,
    pub subvp_prefetch_dramclk_khz: c_int,
    pub subvp_prefetch_fclk_khz: c_int,
// deprecated: use _KBps variants — will be removed after DML update
    pub utm_urgent_bandwidth_lb_Kbps: c_uint,
    pub utm_nominal_bandwidth_lb_Kbps: c_uint,
    pub utm_urgent_bandwidth_lb_KBps: c_uint,
    pub utm_nominal_bandwidth_lb_KBps: c_uint,
    pub utm_latency_ub_index: c_uint,
    pub utm_lsdma_bandwidth_lb_KBps: c_uint,
    pub utm_nominal_max_latency_ub_ns: c_uint,
    pub utm_nominal_avg_latency_ub_ns: c_uint,
// deprecated: use _KBps variant — will be removed after DML update
    pub required_avg_active_bandwidth_Kbps: c_uint,
    pub required_avg_active_bandwidth_KBps: c_uint,
// Stutter efficiency is technically not clock values
// but stored here so the values are part of the update_clocks call similar to num_ways
// Efficiencies are stored as percentage (0-100)
//
    pub //LP1: uint8_t base_efficiency;,
    pub //LP2: uint8_t low_power_efficiency;,
    pub z8_stutter_efficiency: u8,
    pub z8_stutter_period: c_int,
    pub stutter_efficiency: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_bw_validation_profile {
    pub enable: bool,
    pub total_ticks: c_ulonglong,
    pub voltage_level_ticks: c_ulonglong,
    pub watermark_ticks: c_ulonglong,
    pub rq_dlg_ticks: c_ulonglong,
    pub total_count: c_ulonglong,
    pub skip_fast_count: c_ulonglong,
    pub skip_pass_count: c_ulonglong,
    pub skip_fail_count: c_ulonglong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union mem_low_power_enable_options {
    pub 1: bool vga:,
    pub 1: bool i2c:,
    pub 1: bool dmcu:,
    pub 1: bool dscl:,
    pub 1: bool cm:,
    pub 1: bool mpc:,
    pub 1: bool optc:,
    pub 1: bool vpg:,
    pub 1: bool afmt:,
    pub bits: },
    pub u32All: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union root_clock_optimization_options {
    pub 1: bool dpp:,
    pub 1: bool dsc:,
    pub 1: bool hdmistream:,
    pub 1: bool hdmichar:,
    pub 1: bool dpstream:,
    pub 1: bool symclk32_se:,
    pub 1: bool symclk32_le:,
    pub 1: bool symclk_fe:,
    pub 1: bool physymclk:,
    pub 1: bool dpiasymclk:,
    pub 22: uint32_t reserved:,
    pub bits: },
    pub u32All: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union fine_grain_clock_gating_enable_options {
    pub /: *mut *mut bool dccg_global_fgcg_rep : 1; / Global fine grain clock gating of repeaters,
    pub /: *mut *mut bool dchub : 1; / Display controller hub,
    pub 1: bool dchubbub :,
    pub /: *mut *mut bool dpp : 1; / Display pipes and planes,
    pub /: *mut *mut bool opp : 1; / Output pixel processing,
    pub /: *mut *mut bool optc : 1; / Output pipe timing combiner,
    pub /: *mut *mut bool dio : 1; / Display output,
    pub /: *mut *mut bool dwb : 1; / Display writeback,
    pub /: *mut *mut bool mmhubbub : 1; / Multimedia hub,
    pub /: *mut *mut bool dmu : 1; / Display core management unit,
    pub /: *mut *mut bool az : 1; / Azalia,
    pub 1: bool dchvm :,
    pub /: *mut *mut bool dsc : 1; / Display stream compression,
    pub 19: uint32_t reserved :,
    pub bits: },
    pub u32All: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pg_hw_pipe_resources {
    PG_HUBP = 0,
    PG_DPP,
    PG_DSC,
    PG_MPCC,
    PG_OPP,
    PG_OPTC,
    PG_DPSTREAM,
    PG_HDMISTREAM,
    PG_PHYSYMCLK,
    PG_HW_PIPE_RESOURCES_NUM_ELEMENT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pg_hw_resources {
    PG_DCCG = 0,
    PG_DCIO,
    PG_DIO,
    PG_DCHUBBUB,
    PG_DCHVM,
    PG_DWB,
    PG_HPO,
    PG_DCOH,
    PG_HW_RESOURCES_NUM_ELEMENT
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pg_block_update {
    pub pg_pipe_res_update: [bool; PG_HW_PIPE_RESOURCES_NUM_ELEMENT][MAX_PIPES],
    pub pg_res_update: [bool; PG_HW_RESOURCES_NUM_ELEMENT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dpia_debug_options {
    pub /: *mut *mut uint32_t disable_dpia:1; / bit 0,
    pub /: *mut *mut uint32_t force_non_lttpr:1; / bit 1,
    pub /: *mut *mut uint32_t extend_aux_rd_interval:1; / bit 2,
    pub /: *mut *mut uint32_t disable_mst_dsc_work_around:1; / bit 3,
    pub /: *mut *mut uint32_t enable_force_tbt3_work_around:1; / bit 4,
    pub /: *mut *mut uint32_t disable_usb4_pm_support:1; / bit 5,
    pub /: *mut *mut uint32_t enable_usb4_bw_zero_alloc_patch:1; / bit 6,
    pub reserved:25: u32,
    pub bits: },
    pub raw: u32,
}

// AUX wake work around options
// 0: enable/disable work around
// 1: use default timeout LINK_AUX_WAKE_TIMEOUT_MS
// 15-2: reserved
// 31-16: timeout in ms
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union aux_wake_wa_options {
    pub 1: uint32_t enable_wa :,
    pub 1: uint32_t use_default_timeout :,
    pub 14: uint32_t rsvd:,
    pub 16: uint32_t timeout_ms :,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_debug_data {
    pub ltFailCount: u32,
    pub i2cErrorCount: u32,
    pub auxErrorCount: u32,
    pub topology_history: pipe_topology_history,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_phy_addr_space_config {
    pub start_addr: u64,
    pub end_addr: u64,
    pub fb_top: u64,
    pub fb_offset: u64,
    pub fb_base: u64,
    pub agp_top: u64,
    pub agp_bot: u64,
    pub agp_base: u64,
    pub system_aperture: },
    pub page_table_start_addr: u64,
    pub page_table_end_addr: u64,
    pub page_table_base_addr: u64,
    pub base_addr_is_mc_addr: bool,
    pub gart_config: },
    pub valid: bool,
    pub is_hvm_enabled: bool,
    pub page_table_default_page_addr: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_virtual_addr_space_config {
    pub page_table_base_addr: u64,
    pub page_table_start_addr: u64,
    pub page_table_end_addr: u64,
    pub page_table_block_size_in_bytes: u32,
    pub invalid: uint8_t page_table_depth; // 1 = 1 level, 2 = 2 level, etc. 0 =,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_bounding_box_overrides {
    pub sr_exit_time_ns: c_uint,
    pub sr_enter_plus_exit_time_ns: c_uint,
    pub sr_exit_z8_time_ns: c_uint,
    pub sr_enter_plus_exit_z8_time_ns: c_uint,
    pub urgent_latency_ns: c_uint,
    pub percent_of_ideal_drambw: c_uint,
    pub dram_clock_change_latency_ns: c_uint,
    pub dummy_clock_change_latency_ns: c_uint,
    pub fclk_clock_change_latency_ns: c_uint,
// This forces a hard min on the DCFCLK we use
// for DML.  Unlike the debug option for forcing
// DCFCLK, this override affects watermark calculations
//
    pub min_dcfclk_mhz: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_qos_info {
    pub qos_bandwidth_lb_in_mbps: u32,
    pub calculated_avg_bw_in_mbps: u32,
    pub qos_max_latency_ub_in_ns: u32,
    pub qos_avg_latency_ub_in_ns: u32,
    pub dcn_bandwidth_ub_in_mbps: u32,
    pub qos_max_bw_budget_in_mbps: u32,
}

//
// struct dc_debug_options - DC debug struct
//
// This struct provides a simple mechanism for developers to change some
// configurations, enable/disable features, and activate extra debug options.
// This can be very handy to narrow down whether some specific feature is
// causing an issue or not.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_debug_options {
    pub disable_dsc: bool,
    pub visual_confirm: visual_confirm,
    pub visual_confirm_rect_height: c_uint,
    pub sanity_checks: bool,
    pub max_disp_clk: bool,
    pub surface_trace: bool,
    pub clock_trace: bool,
    pub validation_trace: bool,
    pub bandwidth_calcs_trace: bool,
    pub max_downscale_src_width: c_int,
// stutter efficiency related
    pub disable_stutter: bool,
    pub use_max_lb: bool,
    pub disable_dcc: dcc_option,
//
// @pipe_split_policy: Define which pipe split policy is used by the
// display core.
//
    pub pipe_split_policy: pipe_split_policy,
    pub force_single_disp_pipe_split: bool,
    pub voltage_align_fclk: bool,
    pub disable_min_fclk: bool,
    pub hdcp_lc_force_fw_enable: bool,
    pub hdcp_lc_enable_sw_fallback: bool,
    pub disable_dfs_bypass: bool,
    pub disable_dpp_power_gate: bool,
    pub disable_hubp_power_gate: bool,
    pub disable_dsc_power_gate: bool,
    pub disable_optc_power_gate: bool,
    pub disable_hpo_power_gate: bool,
    pub disable_io_clk_power_gate: bool,
    pub disable_mem_power_gate: bool,
    pub disable_dio_power_gate: bool,
    pub dsc_min_slice_height_override: c_uint,
    pub dsc_bpp_increment_div: c_uint,
    pub disable_pplib_wm_range: bool,
    pub pplib_wm_report_mode: wm_report_mode,
    pub min_disp_clk_khz: c_uint,
    pub min_dpp_clk_khz: c_uint,
    pub min_dram_clk_khz: c_uint,
    pub sr_exit_time_dpm0_ns: c_uint,
    pub sr_enter_plus_exit_time_dpm0_ns: c_uint,
    pub sr_exit_time_ns: c_uint,
    pub sr_enter_plus_exit_time_ns: c_uint,
    pub sr_exit_z8_time_ns: c_uint,
    pub sr_enter_plus_exit_z8_time_ns: c_uint,
    pub urgent_latency_ns: c_uint,
    pub underflow_assert_delay_us: u32,
    pub percent_of_ideal_drambw: c_uint,
    pub dram_clock_change_latency_ns: c_uint,
    pub optimized_watermark: bool,
    pub always_scale: c_int,
    pub disable_pplib_clock_request: bool,
    pub disable_clock_gate: bool,
    pub disable_mem_low_power: bool,
    pub pstate_enabled: bool,
    pub disable_dmcu: bool,
    pub force_abm_enable: bool,
    pub disable_stereo_support: bool,
    pub vsr_support: bool,
    pub performance_trace: bool,
    pub az_endpoint_mute_only: bool,
    pub always_use_regamma: bool,
    pub recovery_enabled: bool,
    pub avoid_vbios_exec_table: bool,
    pub scl_reset_length10: bool,
    pub hdmi20_disable: bool,
    pub skip_detection_link_training: bool,
    pub edid_read_retry_times: u32,
    pub inst: uint8_t force_odm_combine; //bit vector based on otg,
    pub seamless_boot_odm_combine: u8,
    pub inst: uint8_t force_odm_combine_4to1; //bit vector based on otg,
    pub minimum_z8_residency_time: c_uint,
    pub minimum_z10_residency_time: c_uint,
    pub disable_z9_mpc: bool,
    pub force_fclk_khz: c_uint,
    pub enable_tri_buf: bool,
    pub ips_disallow_entry: bool,
    pub disable_idle_power_optimizations: bool,
    pub mall_size_override: c_uint,
    pub mall_additional_timer_percent: c_uint,
    pub mall_error_as_fatal: bool,
    pub /: *mut *mut bool dmub_command_table; / for testing only,
    pub bw_val_profile: dc_bw_validation_profile,
    pub disable_fec: bool,
    pub disable_48mhz_pwrdwn: bool,
// This forces a hard min on the DCFCLK requested to SMU/PP
// watermarks are not affected.
//
    pub force_min_dcfclk_mhz: c_uint,
    pub dwb_fi_phase: c_int,
    pub disable_timing_sync: bool,
    pub cm_in_bypass: bool,
    pub change.*/: *mut *mut int force_clock_mode;/every mode,
    pub disable_dram_clock_change_vactive_support: bool,
    pub validate_dml_output: bool,
    pub enable_dmcub_surface_flip: bool,
    pub usbc_combo_phy_reset_wa: bool,
    pub force_fva: bool,
    pub max_frl_rate: c_int,
    pub force_frl_rate: c_uint,
    pub ignore_ffe: bool,
    pub select_ffe: c_uint,
    pub limit_ffe: c_uint,
    pub force_frl_always: bool,
    pub force_frl_dsc: bool,
    pub force_frl_max: bool,
    pub apply_vsdb_rcc_wa: bool,
    pub enable_hdmi_idcc: bool,
    pub enable_dram_clock_change_one_display_vactive: bool,
// TODO - remove once tested
    pub legacy_dp2_lt: bool,
    pub set_mst_en_for_sst: bool,
    pub disable_uhbr: bool,
    pub force_dp2_lt_fallback_method: bool,
    pub ignore_cable_id: bool,
    pub enable_mem_low_power: mem_low_power_enable_options,
    pub root_clock_optimization: root_clock_optimization_options,
    pub enable_fine_grain_clock_gating: fine_grain_clock_gating_enable_options,
    pub hpo_optimization: bool,
    pub force_vblank_alignment: bool,
// Enable dmub aux for legacy ddc
    pub enable_dmub_aux_for_legacy_ddc: bool,
    pub disable_fams: bool,
    pub disable_fams_gaming: in_game_fams_config,
// FEC/PSR1 sequence enable delay in 100us
    pub fec_enable_delay_in100us: u8,
    pub enable_driver_sequence_debug: bool,
    pub crb_alloc_policy: det_size,
    pub crb_alloc_policy_min_disp_count: c_uint,
    pub disable_z10: bool,
    pub enable_z9_disable_interface: bool,
    pub psr_skip_crtc_disable: bool,
    pub ips_skip_crtc_disable_mask: u32,
    pub dpia_debug: dpia_debug_options,
    pub disable_fixed_vs_aux_timeout_wa: bool,
    pub fixed_vs_aux_delay_config_wa: u32,
    pub force_disable_subvp: bool,
    pub force_subvp_mclk_switch: bool,
    pub allow_sw_cursor_fallback: bool,
    pub force_subvp_num_ways: c_uint,
    pub force_mall_ss_num_ways: c_uint,
    pub alloc_extra_way_for_cursor: bool,
    pub subvp_extra_lines: u32,
    pub disable_force_pstate_allow_on_hw_release: bool,
    pub force_usr_allow: bool,
// uses value at boot and disables switch
    pub disable_dtb_ref_clk_switch: bool,
    pub extended_blank_optimization: bool,
    pub aux_wake_wa: aux_wake_wa_options,
    pub mst_start_top_delay: u32,
    pub psr_power_use_phy_fsm: u8,
    pub dml_hostvm_override: dml_hostvm_override_opts,
    pub dml_disallow_alternate_prefetch_modes: bool,
    pub use_legacy_soc_bb_mechanism: bool,
    pub exit_idle_opt_for_cursor_updates: bool,
    pub using_dml2: bool,
    pub enable_single_display_2to1_odm_policy: bool,
    pub enable_double_buffered_dsc_pg_support: bool,
    pub enable_dp_dig_pixel_rate_div_policy: bool,
    pub using_dml21: bool,
    pub lttpr_mode_override: lttpr_mode,
    pub dsc_delay_factor_wa_x1000: c_uint,
    pub min_prefetch_in_strobe_ns: c_uint,
    pub disable_unbounded_requesting: bool,
    pub dig_fifo_off_in_blank: bool,
    pub override_dispclk_programming: bool,
    pub otg_crc_db: bool,
    pub disallow_dispclk_dppclk_ds: bool,
    pub disable_fpo_optimizations: bool,
    pub support_eDP1_5: bool,
    pub fpo_vactive_margin_us: u32,
    pub disable_fpo_vactive: bool,
    pub disable_boot_optimizations: bool,
    pub override_odm_optimization: bool,
    pub minimize_dispclk_using_odm: bool,
    pub disable_subvp_high_refresh: bool,
    pub disable_dp_plus_plus_wa: bool,
    pub fpo_vactive_min_active_margin_us: u32,
    pub fpo_vactive_max_blank_us: u32,
    pub enable_hpo_pg_support: bool,
    pub disable_dc_mode_overwrite: bool,
    pub replay_skip_crtc_disabled: bool,
    pub it*/: *mut *mut bool ignore_pg;/do nothing, let pmfw control,
    pub psp_disabled_wa: bool,
    pub ips2_eval_delay_us: c_uint,
    pub ips2_entry_delay_us: c_uint,
    pub optimize_ips_handshake: bool,
    pub disable_dmub_reallow_idle: bool,
    pub disable_timeout: bool,
    pub disable_extblankadj: bool,
    pub enable_idle_reg_checks: bool,
    pub static_screen_wait_frames: c_uint,
    pub pwm_freq: u32,
    pub force_chroma_subsampling_1tap: bool,
    pub dcc_meta_propagation_delay_us: c_uint,
    pub disable_422_left_edge_pixel: bool,
    pub dml21_force_pstate_method: bool,
    pub dml21_force_pstate_method_values: [u32; MAX_PIPES],
    pub dml21_disable_pstate_method_mask: u32,
    pub fams_version: fw_assisted_mclk_switch_version,
    pub fams2_config: dmub_fams2_global_feature_config,
    pub force_cositing: c_uint,
    pub disable_spl: c_uint,
    pub force_easf: c_uint,
    pub force_sharpness: c_uint,
    pub force_sharpness_level: c_uint,
    pub force_lls: c_uint,
    pub notify_dpia_hr_bw: bool,
    pub enable_ips_visual_confirm: bool,
    pub sharpen_policy: c_uint,
    pub scale_to_sharpness_policy: c_uint,
    pub enable_oled_edp_power_up_opt: c_uint,
    pub enable_hblank_borrow: bool,
    pub force_subvp_df_throttle: bool,
    pub acpi_transition_bitmasks: [u32; MAX_PIPES],
    pub enable_pg_cntl_debug_logs: bool,
    pub auxless_alpm_lfps_setup_ns: c_uint,
    pub auxless_alpm_lfps_period_ns: c_uint,
    pub auxless_alpm_lfps_silence_ns: c_uint,
    pub auxless_alpm_lfps_t1t2_us: c_uint,
    pub auxless_alpm_lfps_t1t2_offset_us: c_short,
    pub disable_stutter_for_wm_program: bool,
    pub enable_block_sequence_programming: bool,
    pub custom_psp_footer_size: u32,
    pub disable_deferred_minimal_transitions: bool,
    pub num_fast_flips_to_steady_state_override: c_uint,
    pub enable_dmu_recovery: bool,
    pub force_vmin_threshold: c_uint,
    pub enable_otg_frame_sync_pwa: bool,
    pub min_deep_sleep_dcfclk_khz: c_uint,
    pub force_odm2to1_for_edp_pixclk_mhz: c_uint,
    pub enable_replay_esd_recovery: bool,
    pub iommu_mismatch_temp_wka: u8,
    pub disable_dynamic_expansion_for_test_pattern: bool,
    pub dml21_custom_derate_num_dpms: u32,
    pub dml21_custom_derate_at_dpm: [u32; DML2_MAX_NUM_DPM_LVL],
}

// Generic structure that can be used to query properties of DC. More fields
// can be added as required.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_current_properties {
    pub cursor_size_limit: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum frame_buffer_mode {
    FRAME_BUFFER_MODE_LOCAL_ONLY = 0,
    FRAME_BUFFER_MODE_ZFB_ONLY,
    FRAME_BUFFER_MODE_MIXED_ZFB_AND_LOCAL,
    } ;

    struct dchub_init_data {
    int64_t zfb_phys_addr_base;
    int64_t zfb_mc_base_addr;
    uint64_t zfb_size_in_byte;
    enum frame_buffer_mode fb_mode;
    bool dchub_initialzied;
    bool dchub_info_valid;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_init_data {
    pub asic_id: hw_asic_id,
    pub /: *mut *mut *mut void driver; / ctx,
    pub cgs_device: *mut cgs_device,
    pub bb_overrides: dc_bounding_box_overrides,
    pub num_virtual_links: c_int,
//
// If 'vbios_override' not NULL, it will be called instead
// of the real VBIOS. Intended use is Diagnostics on FPGA.
//
    pub vbios_override: *mut dc_bios,
    pub dce_environment: dce_environment,
    pub dmub_if: *mut dmub_offload_funcs,
    pub flags: dc_config,
    pub log_mask: u64,
    pub vendor_signature: dpcd_vendor_signature,
    pub force_smu_not_present: bool,
//
// IP offset for run time initializaion of register addresses
//
// DCN3.5+ will fail dc_create() if these fields are null for them. They are
// applicable starting with DCN32/321 and are not used for ASICs upstreamed
// before them.
//
    pub dcn_reg_offsets: *mut u32,
    pub nbio_reg_offsets: *mut u32,
    pub clk_reg_offsets: *mut u32,
    pub bb_from_dmub: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_callback_init {
    pub cp_psp: cp_psp,
}

extern "C" {
    pub fn dc_hardware_init(dc: *mut dc);
}
extern "C" {
    pub fn dc_get_vmid_use_vector(dc: *mut dc) -> c_int;
}
extern "C" {
    pub fn dc_setup_vm_context(dc: *mut dc, va_config: *mut dc_virtual_addr_space_config, vmid: c_int);
}
// Returns the number of vmids supported
extern "C" {
    pub fn dc_setup_system_context(dc: *mut dc, pa_config: *mut dc_phy_addr_space_config) -> c_uint;
}
extern "C" {
    pub fn dc_deinit_callbacks(dc: *mut dc);
}
extern "C" {
    pub fn dc_destroy(dc: *mut dc);
}
// Surface Interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_hdr_static_metadata {
// display chromaticities and white point in units of 0.00001
    pub chromaticity_green_x: c_uint,
    pub chromaticity_green_y: c_uint,
    pub chromaticity_blue_x: c_uint,
    pub chromaticity_blue_y: c_uint,
    pub chromaticity_red_x: c_uint,
    pub chromaticity_red_y: c_uint,
    pub chromaticity_white_point_x: c_uint,
    pub chromaticity_white_point_y: c_uint,
    pub min_luminance: u32,
    pub max_luminance: u32,
    pub maximum_content_light_level: u32,
    pub maximum_frame_average_light_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_transfer_func_type {
    TF_TYPE_PREDEFINED,
    TF_TYPE_DISTRIBUTED_POINTS,
    TF_TYPE_BYPASS,
    TF_TYPE_HWPWL
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_transfer_func_distributed_points {
    pub red: [fixed31_32; TRANSFER_FUNC_POINTS],
    pub green: [fixed31_32; TRANSFER_FUNC_POINTS],
    pub blue: [fixed31_32; TRANSFER_FUNC_POINTS],
    pub end_exponent: u16,
    pub x_point_at_y1_red: u16,
    pub x_point_at_y1_green: u16,
    pub x_point_at_y1_blue: u16,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_transfer_func_predefined {
    TRANSFER_FUNCTION_SRGB,
    TRANSFER_FUNCTION_BT709,
    TRANSFER_FUNCTION_PQ,
    TRANSFER_FUNCTION_LINEAR,
    TRANSFER_FUNCTION_UNITY,
    TRANSFER_FUNCTION_HLG,
    TRANSFER_FUNCTION_HLG12,
    TRANSFER_FUNCTION_GAMMA22,
    TRANSFER_FUNCTION_GAMMA24,
    TRANSFER_FUNCTION_GAMMA26
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_transfer_func {
    pub refcount: kref,
    pub type: dc_transfer_func_type,
    pub tf: dc_transfer_func_predefined,
// FP16 1.0 reference level in nits, default is 80 nits, only for PQ
    pub sdr_ref_white_level: u32,
    pub hdr_multiplier: fixed31_32,
    pub pwl: pwl_params,
    pub tf_pts: dc_transfer_func_distributed_points,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union dc_3dlut_state {
    pub /: *mut *mut uint32_t initialized:1; /if 3dlut is went through color module for initialization,
    pub valid*/: *mut *mut uint32_t rmu_idx_valid:1; /if mux settings are,
    pub use*/: *mut *mut uint32_t rmu_mux_num:3; /index of mux to,
    pub mpcc3*/: *mut *mut uint32_t mpc_rmu0_mux:4; /select mpcc on mux, one of the following : mpcc0, mpcc1, mpcc2,,
    pub mpc_rmu1_mux:4: u32,
    pub mpc_rmu2_mux:4: u32,
    pub reserved:15: u32,
    pub bits: },
    pub raw: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lut_rgb {
    pub b: u16,
    pub g: u16,
    pub r: u16,
    pub padding: u16,
}

// this structure maps directly to how the lut will read it from memory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lut_mem_mapping {
// NATIVE MODE 1, 2
// RGB layout          [b][g][r]      //red  is 128 byte aligned
// BGR layout          [r][g][b]      //blue is 128 byte aligned
    pub rgb_17c: [lut_rgb; 17][17][MATRIX_17C__DIM_128_ALIGNED_LEN],
    pub rgb_33c: [lut_rgb; 33][33][MATRIX_33C__DIM_128_ALIGNED_LEN],
// TRANSFORMED
    pub linear_rgb: [*mut u16; (33*33*33*4/128+1)*128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_rmcm_3dlut {
    pub isInUse: bool,
    pub stream: *const dc_stream_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_3dlut {
    pub refcount: kref,
    pub lut_3d: tetrahedral_params,
    pub state: dc_3dlut_state,
}

// 3DLUT DMA (Fast Load) params
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_3dlut_dma {
    pub addr: dc_plane_address,
    pub swizzle: dc_cm_lut_swizzle,
    pub format: dc_cm_lut_pixel_format,
    pub /: *mut *mut uint16_t bias; / FP1.5.10,
    pub /: *mut *mut uint16_t scale; / FP1.5.10,
    pub size: dc_cm_lut_size,
}

// color manager
#[repr(C)]
#[derive(Copy, Clone)]
pub union dc_plane_cm_flags {
    pub all: c_uint,
    pub 1: unsigned int shaper_enable :,
    pub 1: unsigned int lut3d_enable :,
    pub 1: unsigned int blend_enable :,
// whether legacy (lut3d_func) or DMA is valid
    pub 1: unsigned int lut3d_dma_enable :,
// RMCM lut to be used instead of MCM
    pub 1: unsigned int rmcm_enable :,
    pub 27: unsigned int reserved:,
    pub bits: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_cm {
    pub refcount: kref,
    pub shaper_func: dc_transfer_func,
    pub lut3d_func: dc_3dlut,
    pub lut3d_dma: dc_3dlut_dma,
}

//
// This structure is filled in by dc_surface_get_status and contains
// the last requested address and the currently active address so the called
// can determine if there are any outstanding flips
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_status {
    pub requested_address: dc_plane_address,
    pub current_address: dc_plane_address,
    pub is_flip_pending: bool,
    pub is_right_eye: bool,
    pub cm_hist: cm_hist,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_update_bits {
    pub addr_update:1: u32,
    pub dcc_change:1: u32,
    pub color_space_change:1: u32,
    pub horizontal_mirror_change:1: u32,
    pub per_pixel_alpha_change:1: u32,
    pub global_alpha_change:1: u32,
    pub hdr_mult:1: u32,
    pub rotation_change:1: u32,
    pub swizzle_change:1: u32,
    pub scaling_change:1: u32,
    pub position_change:1: u32,
    pub in_transfer_func_change:1: u32,
    pub input_csc_change:1: u32,
    pub coeff_reduction_change:1: u32,
    pub pixel_format_change:1: u32,
    pub plane_size_change:1: u32,
    pub gamut_remap_change:1: u32,
    pub cursor_csc_color_matrix_change:1: u32,
    pub new_plane:1: u32,
    pub bpp_change:1: u32,
    pub gamma_change:1: u32,
    pub bandwidth_change:1: u32,
    pub clock_change:1: u32,
    pub stereo_format_change:1: u32,
    pub lut_3d:1: u32,
    pub tmz_changed:1: u32,
    pub full_update:1: u32,
    pub sdr_white_level_nits:1: u32,
    pub cm_hist_change:1: u32,
// NOTE: When adding a new field, also update:
// - dc_pipe_update_bits_set_full()
// - dc_pipe_update_bits_is_any_set()
//
}

// memset ensures padding bits are zeroed
extern "C" {
    pub fn dc_check_address_only_update(update_bits: pipe_update_bits) -> bool;
}
pub const DC_REMOVE_PLANE_POINTERS: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_state {
    pub address: dc_plane_address,
    pub time: dc_plane_flip_time,
    pub triplebuffer_flips: bool,
    pub scaling_quality: scaling_taps,
    pub src_rect: rect,
    pub dst_rect: rect,
    pub clip_rect: rect,
    pub plane_size: plane_size,
    pub tiling_info: dc_tiling_info,
    pub dcc: dc_plane_dcc_param,
    pub gamma_correction: dc_gamma,
    pub in_transfer_func: dc_transfer_func,
    pub bias_and_scale: dc_bias_and_scale,
    pub input_csc_color_matrix: dc_csc_transform,
    pub coeff_reduction_factor: fixed31_32,
    pub hdr_mult: fixed31_32,
    pub gamut_remap_matrix: colorspace_transform,
    pub color_space: dc_color_space,

    pub lut_bank_a: bool,
    pub hdr_static_ctx: dc_hdr_static_metadata,
    pub lut3d_func: dc_3dlut,
    pub in_shaper_func: dc_transfer_func,
    pub blend_tf: dc_transfer_func,
    pub mcm_shaper_3dlut_setting: dc_cm2_shaper_3dlut_setting,
    pub mcm_lut1d_enable: bool,
    pub mcm_luts: dc_cm2_func_luts,
    pub mcm_location: mpcc_movable_cm_location,

    pub cm: dc_plane_cm,
    pub gamcor_tf: *mut dc_transfer_func,
    pub format: surface_pixel_format,
    pub rotation: dc_rotation_angle,
    pub stereo_format: plane_stereo_format,
    pub is_tiling_rotated: bool,
    pub per_pixel_alpha: bool,
    pub pre_multiplied_alpha: bool,
    pub global_alpha: bool,
    pub global_alpha_value: c_int,
    pub visible: bool,
    pub flip_immediate: bool,
    pub horizontal_mirror: bool,
    pub layer_index: c_uint,
    pub update_bits: pipe_update_bits,
    pub flip_int_enabled: bool,
    pub skip_manual_trigger: bool,
// private to DC core
    pub status: dc_plane_status,
    pub ctx: *mut dc_context,
// HACK: Workaround for forcing full reprogramming under some conditions
    pub force_full_update: bool,
    pub instead: bool is_phantom; // TODO: Change mall_stream_config into mall_plane_config,
// private to dc_surface.c
    pub irq_source: dc_irq_source,
    pub refcount: kref,
    pub visual_confirm_color: tg_color,
    pub is_statically_allocated: bool,
    pub cositing: chroma_cositing,
    pub cursor_csc_color_matrix: dc_csc_transform,
    pub adaptive_sharpness_en: bool,
    pub adaptive_sharpness_policy: c_int,
    pub sharpness_level: c_uint,
    pub scaling_linearity: dc_scaling_linearity,
    pub sdr_white_level_nits: c_uint,
    pub cm_hist_control: cm_hist_control,
    pub sharpness_range: spl_sharpness_range,
    pub sharpness_source: sharpness_range_source,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_plane_info {
    pub plane_size: plane_size,
    pub tiling_info: dc_tiling_info,
    pub dcc: dc_plane_dcc_param,
    pub format: surface_pixel_format,
    pub rotation: dc_rotation_angle,
    pub stereo_format: plane_stereo_format,
    pub color_space: dc_color_space,
    pub horizontal_mirror: bool,
    pub visible: bool,
    pub per_pixel_alpha: bool,
    pub pre_multiplied_alpha: bool,
    pub global_alpha: bool,
    pub global_alpha_value: c_int,
    pub input_csc_enabled: bool,
    pub layer_index: c_uint,
    pub cositing: chroma_cositing,
    pub scaling_linearity: dc_scaling_linearity,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_scratch_space {
// used to temporarily backup plane states of a stream during
// dc update. The reason is that plane states are overwritten
// with surface updates in dc update. Once they are overwritten
// current state is no longer valid. We want to temporarily
// store current value in plane states so we can still recover
// a valid current state during dc update.
//
    pub plane_states: [dc_plane_state; MAX_SURFACES],
    pub stream_state: dc_stream_state,
}

//
// A link contains one or more sinks and their connected status.
// The currently active signal type (HDMI, DP-SST, DP-MST) is also reported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_link {
    pub remote_sinks: [*mut dc_sink; MAX_SINKS_PER_LINK],
    pub sink_count: c_uint,
    pub local_sink: *mut dc_sink,
    pub link_index: c_uint,
    pub type: dc_connection_type,
    pub connector_signal: signal_type,
    pub irq_source_hpd: dc_irq_source,
    pub /: *mut *mut dc_irq_source irq_source_hpd_rx;/ aka DP Short Pulse,
    pub /: *mut *mut dc_irq_source irq_source_read_request;/ Read Request,
    pub is_hpd_filter_disabled: bool,
    pub dp_ss_off: bool,
//
// @link_state_valid:
//
// If there is no link and local sink, this variable should be set to
// false. Otherwise, it should be set to true; usually, the function
// core_link_enable_stream sets this field to true.
//
    pub link_state_valid: bool,
    pub aux_access_disabled: bool,
    pub sync_lt_in_progress: bool,
    pub skip_stream_reenable: bool,
    pub is_internal_display: bool,
// @todo Rename. Flag an endpoint as having a programmable mapping to a DIG encoder.
    pub is_dig_mapping_flexible: bool,
    pub /: *mut *mut bool hpd_status; / HPD status of link without physical HPD pin.,
    pub /: *mut *mut bool is_hpd_pending; / Indicates a new received hpd,
// USB4 DPIA links skip verifying link cap, instead performing the fallback method
// for every link training. This is incompatible with DP LL compliance automation,
// which expects the same link settings to be used every retry on a link loss.
// This flag is used to skip the fallback when link loss occurs during automation.
//
    pub skip_fallback_on_link_loss: bool,
    pub edp_sink_present: bool,
    pub dp_trace: dp_trace,
    pub is_link_locked: volatile bool,
// caps is the same as reported_link_cap. link_traing use
// reported_link_cap. Will clean up.  TODO
//
    pub reported_link_cap: dc_link_settings,
    pub verified_link_cap: dc_link_settings,
    pub cur_link_settings: dc_link_settings,
    pub cur_lane_setting: [dc_lane_settings; LANE_COUNT_DP_MAX],
    pub preferred_link_setting: dc_link_settings,
// preferred_training_settings are override values that
// come from DM. DM is responsible for the memory
// management of the override pointers.
//
    pub preferred_training_settings: dc_link_training_overrides,
    pub preferred_hdmi_frl_settings: dc_hdmi_frl_link_training_overrides,
    pub audio_test_data: dp_audio_test_data,
// On ASICs with dp_connector_no_native_i2c cap set and no_ddc_pin cap
// set by IFWI, link aux_hw_inst is used in aux layer functions instead
// of ddc_pin to know which aux instance is associated with link.
//
    pub no_ddc_pin: bool,
// When set, forces all native I2C communication on this DP connector
// to use the I2C-over-AUX protocol instead of native I2C signaling.
//
    pub force_to_use_aux: bool,
    pub aux_hw_inst: gpio_ddc_line,
    pub ddc_hw_inst: gpio_ddc_line,
    pub hpd_src: u8,
    pub link_enc_hw_inst: u8,
// DIG link encoder ID. Used as index in link encoder resource pool.
// For links with fixed mapping to DIG, this is not changed after dc_link
// object creation.
//
    pub eng_id: engine_id,
    pub dpia_preferred_eng_id: engine_id,
    pub test_pattern_enabled: bool,
// Pending/Current test pattern are only used to perform and track
// FIXED_VS retimer test pattern/lane adjustment override state.
// Pending allows link HWSS to differentiate PHY vs non-PHY pattern,
// to perform specific lane adjust overrides before setting certain
// PHY test patterns. In cases when lane adjust and set test pattern
// calls are not performed atomically (i.e. performing link training),
// pending_test_pattern will be invalid or contain a non-PHY test pattern
// and current_test_pattern will contain required context for any future
// set pattern/set lane adjust to transition between override state(s).
//
    pub current_test_pattern: dp_test_pattern,
    pub pending_test_pattern: dp_test_pattern,
    pub compliance_test_state: compliance_test_state,
    pub priv: *mut c_void,
    pub ddc: *mut ddc_service,
    pub panel_mode: dp_panel_mode,
    pub aux_mode: bool,
// Private to DC core
    pub dc: *const dc,
    pub ctx: *mut dc_context,
    pub panel_cntl: *mut panel_cntl,
    pub link_enc: *mut link_encoder,
    pub hpo_frl_link_enc: *mut hpo_frl_link_encoder,
    pub frl_reported_link_cap: dc_hdmi_frl_link_settings,
    pub frl_verified_link_cap: dc_hdmi_frl_link_settings,
    pub frl_link_settings: dc_hdmi_frl_link_settings,
    pub frl_flags: dc_hdmi_frl_flags,
    pub hdmi_cable_id: hdmi_idcc_cable_id,
    pub link_id: graphics_object_id,
// External encoder eg. NUTMEG or TRAVIS used on CIK APUs.
    pub ext_enc_id: graphics_object_id,
// Endpoint type distinguishes display endpoints which do not have entries
// in the BIOS connector table from those that do. Helps when tracking link
// encoder to display endpoint assignments.
//
    pub ep_type: display_endpoint_type,
    pub ddi_channel_mapping: ddi_channel_mapping,
    pub device_tag: connector_device_tag_info,
    pub dpcd_caps: dpcd_caps,
    pub dongle_max_pix_clk: u32,
    pub chip_caps: c_ushort,
    pub dpcd_sink_count: c_uint,
    pub hdcp_caps: hdcp_caps,
    pub edp_revision: edp_revision,
    pub dpcd_sink_ext_caps: dpcd_sink_ext_caps,
    pub psr_settings: psr_settings,
    pub replay_settings: replay_settings,
// Drive settings read from integrated info table
    pub bios_forced_drive_settings: dc_lane_settings,
// Vendor specific LTTPR workaround variables
    pub vendor_specific_lttpr_link_rate_wa: u8,
    pub apply_vendor_specific_lttpr_link_rate_wa: bool,
// MST record stream using this link
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_flags {
    pub dp_keep_receiver_powered: bool,
    pub dp_skip_DID2: bool,
    pub dp_skip_reset_segment: bool,
    pub dp_skip_fs_144hz: bool,
// Some DP bridges don't work with RBR and must use HBR.
    pub dp_skip_rbr: bool,
    pub dp_mot_reset_segment: bool,
// Some USB4 docks do not handle turning off MST DSC once it has been enabled.
    pub dpia_mst_dsc_always_on: bool,
// Forced DPIA into TBT3 compatibility mode.
    pub dpia_forced_tbt3_mode: bool,
    pub dongle_mode_timing_override: bool,
    pub blank_stream_on_ocs_change: bool,
    pub read_dpcd204h_on_irq_hpd: bool,
    pub force_dp_ffe_preset: bool,
    pub skip_phy_ssc_reduction: bool,
    pub wa_flags: },
    pub forced_dp_ffe_preset: dc_dp_ffe_preset,
    pub mst_stream_alloc_table: link_mst_stream_allocation_table,
    pub link_status: dc_link_status,
    pub dprx_states: dprx_states,
    pub fec_state: dc_link_fec_state,
    pub is_dds: bool,
    pub is_display_mux_present: bool,
    pub forcibly: bool link_powered_externally; // Used to bypass hardware sequencing delays when panel is powered down,
    pub panel_config: dc_panel_config,
    pub panel_type: dc_panel_type,
    pub phy_state: phy_state,
    pub phy_transition_bitmask: u32,
// BW ALLOCATON USB4 ONLY
    pub dpia_bw_alloc_config: dc_dpia_bw_alloc,
    pub skip_implict_edp_power_control: bool,
    pub forced_psr_active: bool,
    pub backlight_control_type: backlight_control_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc {
    pub debug: dc_debug_options,
    pub versions: dc_versions,
    pub caps: dc_caps,
    pub check_config: dc_check_config,
    pub cap_funcs: dc_cap_funcs,
    pub config: dc_config,
    pub bb_overrides: dc_bounding_box_overrides,
    pub work_arounds: dc_bug_wa,
    pub ctx: *mut dc_context,
    pub vm_pa_config: dc_phy_addr_space_config,
    pub link_count: u8,
    pub links: [*mut dc_link; MAX_LINKS],
    pub lowest_dpia_link_index: u8,
    pub link_srv: *mut link_service,
    pub current_state: *mut dc_state,
    pub res_pool: *mut resource_pool,
//
// @update_scratch_pool: Per-commit scratch buffers for dc_update_state.
//
    pub 1]: *mut *mut dc_update_scratch_space update_scratch_pool[MAX_STREAMS +,
    pub 1]: bool update_scratch_in_use[MAX_STREAMS +,
    pub clk_mgr: *mut clk_mgr,
// Display Engine Clock levels
    pub sclk_lvls: dm_pp_clock_levels,
// Inputs into BW and WM calculations.
    pub bw_dceip: *mut bw_calcs_dceip,
    pub bw_vbios: *mut bw_calcs_vbios,
    pub dcn_soc: *mut dcn_soc_bounding_box,
    pub dcn_ip: *mut dcn_ip_params,
    pub dml: display_mode_lib,
// HW functions
    pub hwss: hw_sequencer_funcs,
    pub hwseq: *mut dce_hwseq,
// Require to optimize clocks and bandwidth for added/removed planes
    pub optimized_required: bool,
    pub idle_optimizations_allowed: bool,
    pub enable_c20_dtm_b0: bool,
// Require to maintain clocks and bandwidth for UEFI enabled HW
// For eDP to know the switching state of SmartMux
    pub is_switch_in_progress_orig: bool,
    pub is_switch_in_progress_dest: bool,
// FBC compressor
    pub fbc_compressor: *mut compressor,
    pub debug_data: dc_debug_data,
    pub vendor_signature: dpcd_vendor_signature,
    pub build_id: *const c_char,
    pub vm_helper: *mut vm_helper,
    pub dcn_reg_offsets: *mut u32,
    pub nbio_reg_offsets: *mut u32,
    pub clk_reg_offsets: *mut u32,
// Scratch memory
//
// For matching clock_limits table in driver with table
// from PMFW.
//
    pub clock_limits: [_vcs_dpi_voltage_scaling_st; DC__VOLTAGE_STATES],
    pub update_bw_bounding_box: },
    pub current_state: dc_scratch_space,
    pub new_state: dc_scratch_space,
    pub stack: dc_stream_state temp_stream; // Used so we don't need to allocate stream on the,
    pub temp_link: dc_link,
    pub /: *mut *mut bool pipes_to_unlock_first[MAX_PIPES]; / Any of the pipes indicated here should be unlocked first,
    pub scratch: },
    pub dml2_options: dml2_configuration_options,
    pub dml2_dc_power_options: dml2_configuration_options,
    pub power_state: dc_acpi_cm_power_state,
    pub soc_and_ip_translator: *mut soc_and_ip_translator,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_scaling_info {
    pub src_rect: rect,
    pub dst_rect: rect,
    pub clip_rect: rect,
    pub scaling_quality: scaling_taps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_fast_update {
    pub flip_addr: *const dc_flip_addrs,
    pub gamma: *const dc_gamma,
    pub gamut_remap_matrix: *const colorspace_transform,
    pub input_csc_color_matrix: *const dc_csc_transform,
    pub coeff_reduction_factor: *const fixed31_32,
    pub out_transfer_func: *mut dc_transfer_func,
    pub output_csc_transform: *mut dc_csc_transform,
    pub cursor_csc_color_matrix: *const dc_csc_transform,
    pub cm_hist_control: *mut cm_hist_control,
// stream-level fast updates
    pub gamut_remap: *const colorspace_transform,
    pub cursor_attributes: *const dc_cursor_attributes,
    pub cursor_position: *const dc_cursor_position,
    pub periodic_interrupt: *const periodic_interrupt_config,
    pub dither_option: *const dc_dither_option,
    pub vrr_infopacket: *mut dc_info_packet,
    pub vsc_infopacket: *mut dc_info_packet,
    pub vsp_infopacket: *mut dc_info_packet,
    pub hfvsif_infopacket: *mut dc_info_packet,
    pub vtem_infopacket: *mut dc_info_packet,
    pub adaptive_sync_infopacket: *mut dc_info_packet,
    pub avi_infopacket: *mut dc_info_packet,
    pub hdr_static_metadata: *mut dc_info_packet,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_surface_update {
    pub surface: *mut dc_plane_state,
// isr safe update parameters.  null means no updates
    pub flip_addr: *const dc_flip_addrs,
    pub plane_info: *const dc_plane_info,
    pub scaling_info: *const dc_scaling_info,
    pub hdr_mult: fixed31_32,
// following updates require alloc/sleep/spin that is not isr safe,
// null means no updates
//
    pub gamma: *const dc_gamma,
    pub in_transfer_func: *const dc_transfer_func,
    pub input_csc_color_matrix: *const dc_csc_transform,
    pub coeff_reduction_factor: *const fixed31_32,
    pub gamut_remap_matrix: *const colorspace_transform,
    pub cm: *const dc_plane_cm,
    pub cursor_csc_color_matrix: *const dc_csc_transform,
    pub sdr_white_level_nits: c_uint,
    pub bias_and_scale: dc_bias_and_scale,
    pub cm_hist_control: *mut cm_hist_control,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_state_update {
    pub stream: *mut dc_stream_state,
    pub stream_update: *mut dc_stream_update,
    pub surface_updates: *mut dc_surface_update,
    pub surface_count: c_int,
    pub probe_updates: *const dc_probe_updates,
}

//
// dc_check_state_update() - Classify an update without committing it.
// @check_config: DC check configuration
// @updates:      root update object to classify
//
// Return: descriptor indicating update type and required lock scope.
//
// dc_update_state - Commit an absolute dc_state_update.
// @dc:      DC structure
// @updates: root update object carrying stream, plane, and probe updates
//
// Return: true on success, false on failure.
//
extern "C" {
    pub fn dc_update_state(dc: *mut dc, updates: *const dc_state_update) -> bool;
}
//
// dc_update_state_init - Acquire and initialise a commit scratch buffer.
// @dc:      DC structure
// @updates: update descriptor; validated before the slot is acquired
//
// Return: a scratch slot on success, NULL if validation fails or the pool
// is exhausted. The slot must be released via dc_update_state_cleanup() on
// success, or automatically by dc_update_state_prepare() on failure.
//
// dc_update_state_prepare - Prepare the commit under the global lock.
// @scratch: commit scratch from dc_update_state_init()
//
// On failure the scratch slot is released and false is returned; the caller
// must not call execute or cleanup.
//
extern "C" {
    pub fn dc_update_state_prepare(scratch: *mut dc_update_scratch_space) -> bool;
}
//
// dc_update_state_execute - Program hardware; called without the global lock.
// @scratch: commit scratch from dc_update_state_init()
//
extern "C" {
    pub fn dc_update_state_execute(scratch: *const dc_update_scratch_space);
}
//
// dc_update_state_cleanup - Finalise the commit and release the scratch slot.
// @scratch: commit scratch from dc_update_state_init()
//
// Must be called with the global lock held. Returns true if the caller must
// loop back to prepare (SEAMLESS continuation).
//
extern "C" {
    pub fn dc_update_state_cleanup(scratch: *mut dc_update_scratch_space) -> bool;
}
//
// struct dc_probe_latencies - min/max/avg memory latency in ns.
// @max_latency_ns: maximum latency in nanoseconds
// @avg_latency_ns: average latency in nanoseconds
// @min_latency_ns: minimum latency in nanoseconds
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_probe_latencies {
    pub max_latency_ns: u32,
    pub avg_latency_ns: u32,
    pub min_latency_ns: u32,
}

//
// struct dc_probe_status - results for a probe.
// @valid: true if a measurement was latched.
// @type: type of the probe that produced this result.
// @u.bandwidth_mbps:         peak BW in Mbps (DC_PROBE_PEAK_MEM_BW).
// @u.latency:                min/max/avg memory latency in ns (DC_PROBE_MEM_LATENCY),
// stored as struct dc_probe_latencies.
// @u.urgent_assertion_count: number of urgent assertion events (DC_PROBE_URGENT_ASSERTION_COUNT).
// @u.prefetch_data_size:     total prefetch data in bytes (DC_PROBE_PREFETCH_DATA_SIZE).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_probe_status {
    pub valid: bool,
    pub type: dc_probe_type,
    pub bandwidth_mbps: u32,
    pub latency: dc_probe_latencies,
    pub urgent_assertion_count: u32,
    pub prefetch_data_size: u32,
    pub u: },
}

//
// enum dc_get_status_type - Bitmask selecting which status classes to populate.
// @DC_GET_STATUS_STREAM: populate stream_status fields in dc_state_status
// @DC_GET_STATUS_PROBE:  populate probe_status fields in dc_state_status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dc_get_status_type {
    DC_GET_STATUS_STREAM = (1u << 0),
    DC_GET_STATUS_PROBE  = (1u << 1),
}

//
// struct dc_get_status_options - Input selector for dc_state_get_status.
// @state:  source state to read status from
// @types:  OR of dc_get_status_type values selecting classes to populate
// @stream: optional stream filter for DC_GET_STATUS_STREAM. NULL means
// populate status for all streams in the state
// @probe:  optional probe filter for DC_GET_STATUS_PROBE. NULL means
// populate status for all probes in the state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_get_status_options {
    pub state: *mut dc_state,
    pub types: u32,
    pub stream: *const dc_stream_state,
    pub probe: *const dc_probe_state,
}

//
// struct dc_state_status - Output-only status object from dc_state_get_status.
// @stream_count: number of valid entries in stream_status (DC_GET_STATUS_STREAM)
// @stream_status: pointers to live per-stream status entries
// @probe_count: number of valid entries in probe_status (DC_GET_STATUS_PROBE)
// @probe_status: pointers to live per-probe status entries
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_state_status {
    pub stream_count: c_int,
    pub stream_status: [*mut dc_stream_status; MAX_STREAMS],
    pub probe_count: c_int,
    pub probe_status: [*mut dc_probe_status; MAX_PROBES],
}

//
// dc_state_get_status - Unified status readback for dc_state.
// @status:  output object populated according to options->types
// @options: selects the source state, status classes to fill, and filters
//
// Return: DC_OK on success, DC_ERROR_UNEXPECTED if state is NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_underflow_debug_data {
    pub hubbub_reg_state: *mut dcn_hubbub_reg_state,
    pub hubp_reg_state: [*mut dcn_hubp_reg_state; MAX_PIPES],
    pub dpp_reg_state: [*mut dcn_dpp_reg_state; MAX_PIPES],
    pub mpc_reg_state: [*mut dcn_mpc_reg_state; MAX_PIPES],
    pub opp_reg_state: [*mut dcn_opp_reg_state; MAX_PIPES],
    pub dsc_reg_state: [*mut dcn_dsc_reg_state; MAX_PIPES],
    pub optc_reg_state: [*mut dcn_optc_reg_state; MAX_PIPES],
    pub dccg_reg_state: [*mut dcn_dccg_reg_state; MAX_PIPES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_features {
    pub ips: bool,
    pub rcg: bool,
    pub replay: bool,
    pub dds: bool,
    pub sprs: bool,
    pub psr: bool,
    pub fams: bool,
    pub mpo: bool,
    pub uclk_p_state: bool,
}

//
// Create a new surface with default parameters;
//
extern "C" {
    pub fn dc_gamma_retain(dc_gamma: *mut dc_gamma);
}
extern "C" {
    pub fn dc_gamma_release(dc_gamma: *mut dc_gamma);
}
extern "C" {
    pub fn dc_transfer_func_retain(dc_tf: *mut dc_transfer_func);
}
extern "C" {
    pub fn dc_transfer_func_release(dc_tf: *mut dc_transfer_func);
}
extern "C" {
    pub fn dc_3dlut_func_release(lut: *mut dc_3dlut);
}
extern "C" {
    pub fn dc_3dlut_func_retain(lut: *mut dc_3dlut);
}
extern "C" {
    pub fn dc_plane_cm_release(cm: *mut dc_plane_cm);
}
extern "C" {
    pub fn dc_plane_cm_retain(cm: *mut dc_plane_cm);
}
//
// dc_get_default_tiling_info() - Retrieve an ASIC-appropriate default tiling
// description for (typically) linear surfaces.
//
// This is used by OS/DM paths that need a valid, fully-initialized tiling
// description without hardcoding gfx-version specifics in the caller.
//
extern "C" {
    pub fn dc_get_default_tiling_info(dc: *const dc, tiling_info: *mut dc_tiling_info);
}
//
// struct dc_validation_stream - Per-stream surface/stream association for validation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_validation_stream {
//
// @stream: Stream state properties
//
    pub stream: *mut dc_stream_state,
//
// @plane_states: Surface state
//
    pub plane_states: [*mut dc_plane_state; MAX_SURFACES],
//
// @plane_count: Total of active planes
//
    pub plane_count: u8,
}

//
// struct dc_validation_set - Root validation input grouping all streams for a commit
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_validation_set {
//
// @streams: Per-stream entries (stream + its planes)
//
    pub streams: [dc_validation_stream; MAX_STREAMS],
//
// @stream_count: Number of active entries in @streams
//
    pub stream_count: u8,
//
// @probes: Global probe descriptors to validate alongside the streams
//
    pub probes: [dc_probe_state; MAX_PROBES],
//
// @probe_count: Number of active entries in @probes
//
    pub probe_count: u8,
}

extern "C" {
    pub fn dc_validate_plane(dc: *mut dc, plane_state: *const dc_plane_state) -> dc_status;
}
extern "C" {
    pub fn dc_resource_is_dsc_encoding_supported(dc: *const dc) -> bool;
}
extern "C" {
    pub fn fast_nonaddr_updates_exist(fast_update: *mut dc_fast_update, surface_count: c_int) -> bool;
}
//
// Set up streams and links associated to drive sinks
// The streams parameter is an absolute set of all active streams.
//
// After this call:
// Phy, Encoder, Timing Generator are programmed and enabled.
// New streams are enabled with blank stream; no memory read.
//
extern "C" {
    pub fn dc_commit_streams(dc: *mut dc, params: *mut dc_commit_streams_params) -> dc_status;
}
extern "C" {
    pub fn dc_get_opp_for_plane(dc: *mut dc, plane: *mut dc_plane_state) -> u32;
}
extern "C" {
    pub fn dc_set_disable_128b_132b_stream_overhead(disable: bool);
}
// The function returns minimum bandwidth required to drive a given timing
// return - minimum required timing bandwidth in kbps.
//
// Link Interfaces
// Return an enumerated dc_link.
// dc_link order is constant and determined at
// boot time.  They cannot be created or destroyed.
// Use dc_get_caps() to get number of links.
//
// Return instance id of the edp link. Inst 0 is primary edp link.
// Return an array of link pointers to edp links.
// The function initiates detection handshake over the given link. It first
// determines if there are display connections over the link. If so it initiates
// detection protocols supported by the connected receiver device. The function
// contains protocol specific handshake sequences which are sometimes mandatory
// to establish a proper connection between TX and RX. So it is always
// recommended to call this function as the first link operation upon HPD event
// or power up event. Upon completion, the function will update link structure
// in place based on latest RX capabilities. The function may also cause dpms
// to be reset to off for all currently enabled streams to the link. It is DM's
// responsibility to serialize detection and DPMS updates.
//
// @reason - Indicate which event triggers this detection. dc may customize
// detection flow depending on the triggering events.
// return false - if detection is not fully completed. This could happen when
// there is an unrecoverable error during detection or detection is partially
// completed (detection has been delegated to dm mst manager ie.
// link->connection_type == dc_connection_mst_branch when returning false).
// return true - detection is completed, link has been fully updated with latest
// detection result.
//
extern "C" {
    pub fn dc_link_detect(link: *mut dc_link, reason: dc_detect_reason) -> bool;
}
// When link connection type is dc_connection_mst_branch, remote sink can be
// added to the link. The interface creates a remote sink and associates it with
// current link. The sink will be retained by link until remove remote sink is
// called.
//
// @dc_link - link the remote sink will be added to.
// @edid - byte array of EDID raw data.
// @len - size of the edid in byte
// @init_data -
//
// Remove remote sink from a link with dc_connection_mst_branch connection type.
// @link - link the sink should be removed from
// @sink - sink to be removed.
//
// Enable HPD interrupt handler for a given link
extern "C" {
    pub fn dc_link_enable_hpd(link: *const dc_link);
}
// Disable HPD interrupt handler for a given link
extern "C" {
    pub fn dc_link_disable_hpd(link: *const dc_link);
}
// determine if there is a sink connected to the link
//
// @type - dc_connection_single if connected, dc_connection_none otherwise.
// return - false if an unexpected error occurs, true otherwise.
//
// NOTE: This function doesn't detect downstream sink connections i.e
// dc_connection_mst_branch, dc_connection_sst_branch. In this case, it will
// return dc_connection_single if the branch device is connected despite of
// downstream sink's connection status.
//
// query current hpd pin value
// return - true HPD is asserted (HPD high), false otherwise (HPD low)
//
extern "C" {
    pub fn dc_link_get_hpd_state(link: *mut dc_link) -> bool;
}
// Getter for cached link status from given link
// enable/disable hardware HPD filter.
//
// @link - The link the HPD pin is associated with.
// @enable = true - enable hardware HPD filter. HPD event will only queued to irq
// handler once after no HPD change has been detected within dc default HPD
// filtering interval since last HPD event. i.e if display keeps toggling hpd
// pulses within default HPD interval, no HPD event will be received until HPD
// toggles have stopped. Then HPD event will be queued to irq handler once after
// dc default HPD filtering interval since last HPD event.
//
// @enable = false - disable hardware HPD filter. HPD event will be queued
// immediately to irq handler after no HPD change has been detected within
// IRQ_HPD (aka HPD short pulse) interval (i.e 2ms).
//
extern "C" {
    pub fn dc_link_enable_hpd_filter(link: *mut dc_link, enable: bool);
}
// submit i2c read/write payloads through ddc channel
// @link_index - index to a link with ddc in i2c mode
// @cmd - i2c command structure
// return - true if success, false otherwise.
//
// submit i2c read/write payloads through oem channel
// @link_index - index to a link with ddc in i2c mode
// @cmd - i2c command structure
// return - true if success, false otherwise.
//
// Attempt to transfer the given aux payload. This function does not perform
// retries or handle error states. The reply is returned in the payload->reply
// and the result through operation_result. Returns the number of bytes
// transferred,or -1 on a failure.
//
// return true if the connected receiver supports the hdcp version
extern "C" {
    pub fn dc_link_is_hdcp14(link: *mut dc_link, signal: signal_type) -> bool;
}
extern "C" {
    pub fn dc_link_is_hdcp22(link: *mut dc_link, signal: signal_type) -> bool;
}
// Notify DC about DP RX Interrupt (aka DP IRQ_HPD).
//
// TODO - When defer_handling is true the function will have a different purpose.
// It no longer does complete hpd rx irq handling. We should create a separate
// interface specifically for this case.
//
// Return:
// true - Downstream port status changed. DM should call DC to do the
// detection.
// false - no change in Downstream port status. No further action required
// from DM.
//
// handle DP specs define test automation sequence
extern "C" {
    pub fn dc_link_dp_handle_automated_test(link: *mut dc_link);
}
// handle DP Link loss sequence and try to recover RX link loss with best
// effort
//
extern "C" {
    pub fn dc_link_dp_handle_link_loss(link: *mut dc_link);
}
// Determine if hpd rx irq should be handled or ignored
// return true - hpd rx irq should be handled.
// return false - it is safe to ignore hpd rx irq event
//
extern "C" {
    pub fn dc_link_dp_allow_hpd_rx_irq(link: *const dc_link) -> bool;
}
// Determine if link loss is indicated with a given hpd_irq_dpcd_data.
// @link - link the hpd irq data associated with
// @hpd_irq_dpcd_data - input hpd irq data
// return - true if hpd irq data indicates a link lost
//
// Read hpd rx irq data from a given link
// @link - link where the hpd irq data should be read from
// @irq_data - output hpd irq data
// return - DC_OK if hpd irq data is read successfully, otherwise hpd irq data
// read has failed.
//
extern "C" {
    pub fn dc_link_frl_poll_status_flag(link: *mut dc_link) -> bool;
}
// The function clears recorded DP RX states in the link. DM should call this
// function when it is resuming from S3 power state to previously connected links.
//
// TODO - in the future we should consider to expand link resume interface to
// support clearing previous rx states. So we don't have to rely on dm to call
// this interface explicitly.
//
extern "C" {
    pub fn dc_link_clear_dprx_states(link: *mut dc_link);
}
// Destruct the mst topology of the link and reset the allocated payload table
//
// NOTE: this should only be called if DM chooses not to call dc_link_detect but
// still wants to reset MST topology on an unplug event
extern "C" {
    pub fn dc_link_reset_cur_dp_mst_topology(link: *mut dc_link) -> bool;
}
// The function calculates effective DP link bandwidth when a given link is
// using the given link settings.
//
// return - total effective link bandwidth in kbps.
//
// The function returns effective HDMI FRL bandwidth given link rate.
// return - total effective link bandwidth in kbps.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dp_audio_bandwidth_params {
    pub crtc_timing: *const dc_crtc_timing,
    pub link_encoding: dp_link_encoding,
    pub channel_count: u32,
    pub sample_rate_hz: u32,
}

// The function calculates the minimum size of hblank (in bytes) needed to
// support the specified channel count and sample rate combination, given the
// link encoding and timing to be used. This calculation is not supported
// for 8b/10b SST.
//
// return - min hblank size in bytes, 0 if 8b/10b SST.
//
// The function takes a snapshot of current link resource allocation state
// @dc: pointer to dc of the dm calling this
// @map: a dc link resource snapshot defined internally to dc.
//
// DM needs to capture a snapshot of current link resource allocation mapping
// and store it in its persistent storage.
//
// Some of the link resource is using first come first serve policy.
// The allocation mapping depends on original hotplug order. This information
// is lost after driver is loaded next time. The snapshot is used in order to
// restore link resource to its previous state so user will get consistent
// link capability allocation across reboot.
//
extern "C" {
    pub fn dc_get_cur_link_res_map(dc: *const dc, map: *mut u32);
}
// This function restores link resource allocation state from a snapshot
// @dc: pointer to dc of the dm calling this
// @map: a dc link resource snapshot defined internally to dc.
//
// DM needs to call this function after initial link detection on boot and
// before first commit streams to restore link resource allocation state
// from previous boot session.
//
// Some of the link resource is using first come first serve policy.
// The allocation mapping depends on original hotplug order. This information
// is lost after driver is loaded next time. The snapshot is used in order to
// restore link resource to its previous state so user will get consistent
// link capability allocation across reboot.
//
extern "C" {
    pub fn dc_restore_link_res_map(dc: *const dc, map: *mut u32);
}
extern "C" {
    pub fn dc_link_wait_for_unlocked(link: *mut dc_link);
}
// TODO: this is not meant to be exposed to DM. Should switch to stream update
// interface i.e stream_update->dsc_config
//
extern "C" {
    pub fn dc_link_update_dsc_config(pipe_ctx: *mut pipe_ctx) -> bool;
}
// translate a raw link rate data to bandwidth in kbps
extern "C" {
    pub fn dc_link_bw_kbps_from_raw_frl_link_rate_data(dc: *const dc, bw: u8) -> u32;
}
// determine the optimal bandwidth given link and required bw.
// @link - current detected link
// @req_bw - requested bandwidth in kbps
// @link_settings - returned most optimal link settings that can fit the
// requested bandwidth
// return - false if link can't support requested bandwidth, true if link
// settings is found.
//
// return the max dp link settings can be driven by the link without considering
// connected RX device and its capability
//
// determine when the link is driving MST mode, what DP link channel coding
// format will be used. The decision will remain unchanged until next HPD event.
//
// @link -  a link with DP RX connection
// return - if stream is committed to this link with MST signal type, type of
// channel coding format dc will choose.
//
// get max dp link settings the link can enable with all things considered. (i.e
// TX/RX/Cable capabilities and dp override policies.
//
// @link - a link with DP RX connection
// return - max dp link settings the link can enable.
//
// Get the highest encoding format that the link supports; highest meaning the
// encoding format which supports the maximum bandwidth.
//
// @link - a link with DP RX connection
// return - highest encoding format link supports.
//
extern "C" {
    pub fn dc_link_get_highest_encoding_format(link: *const dc_link) -> dc_link_encoding_format;
}
// get max frl link settings the link can enable with all things considered.
// (i.e TX/RX capabilities and link verification result.
//
// @link - a link with FRL RX connection
// return - max frl link settings the link can enable.
//
// Check if a RX (ex. DP sink, MST hub, passive or active dongle) is connected
// to a link with dp connector signal type.
// @link - a link with dp connector signal type
// return - true if connected, false otherwise
//
extern "C" {
    pub fn dc_link_is_dp_sink_present(link: *mut dc_link) -> bool;
}
// Force DP lane settings update to main-link video signal and notify the change
// to DP RX via DPCD. This is a debug interface used for video signal integrity
// tuning purpose. The interface assumes link has already been enabled with DP
// signal.
//
// @lt_settings - a container structure with desired hw_lane_settings
//
// Enable a test pattern in Link or PHY layer in an active link for compliance
// test or debugging purpose. The test pattern will remain until next un-plug.
//
// @link - active link with DP signal output enabled.
// @test_pattern - desired test pattern to output.
// NOTE: set to DP_TEST_PATTERN_VIDEO_MODE to disable previous test pattern.
// @test_pattern_color_space - for video test pattern choose a desired color
// space.
// @p_link_settings - For PHY pattern choose a desired link settings
// @p_custom_pattern - some test pattern will require a custom input to
// customize some pattern details. Otherwise keep it to NULL.
// @cust_pattern_size - size of the custom pattern input.
//
// Force DP link settings to always use a specific value until reboot to a
// specific link. If link has already been enabled, the interface will also
// switch to desired link settings immediately. This is a debug interface to
// generic dp issue trouble shooting.
//
// Force FRL link settings to always use a specific value until reboot to a
// specific link. If link has already been enabled, the interface will also
// switch to desired link settings immediately. This is a debug interface to
// generic FRL issue trouble shooting.
//
// Force DP link to customize a specific link training behavior by overriding to
// standard DP specs defined protocol. This is a debug interface to trouble shoot
// display specific link training issues or apply some display specific
// workaround in link training.
//
// @link_settings - if not NULL, force preferred link settings to the link.
// @lt_override - a set of override pointers. If any pointer is none NULL, dc
// will apply this particular override in future link training. If NULL is
// passed in, dc resets previous overrides.
// NOTE: DM must keep the memory from override pointers until DM resets preferred
// training settings.
//
// return - true if FEC is supported with connected DP RX, false otherwise
extern "C" {
    pub fn dc_link_is_fec_supported(link: *const dc_link) -> bool;
}
// query FEC enablement policy to determine if FEC will be enabled by dc during
// link enablement.
// return - true if FEC should be enabled, false otherwise.
//
extern "C" {
    pub fn dc_link_should_enable_fec(link: *const dc_link) -> bool;
}
// determine lttpr mode the current link should be enabled with a specific link
// settings.
//
// Force DP RX to update its power state.
// NOTE: this interface doesn't update dp main-link. Calling this function will
// cause DP TX main-link and DP RX power states out of sync. DM has to restore
// RX power state back upon finish DM specific execution requiring DP RX in a
// specific power state.
// @on - true to set DP RX in D0 power state, false to set DP RX in D3 power
// state.
//
extern "C" {
    pub fn dc_link_dp_receiver_power_ctrl(link: *mut dc_link, on: bool);
}
// Force link to read base dp receiver caps from dpcd 000h - 00Fh and overwrite
// current value read from extended receiver cap from 02200h - 0220Fh.
// Some DP RX has problems of providing accurate DP receiver caps from extended
// field, this interface is a workaround to revert link back to use base caps.
//
// Set backlight level of an embedded panel (eDP, LVDS).
// backlight_pwm_u16_16 is unsigned 32 bit with 16 bit integer
// and 16 bit fractional, where 1.0 is max backlight value.
//
// Set/get nits-based backlight level of an embedded panel (eDP, LVDS).
extern "C" {
    pub fn dc_link_get_backlight_level(dc_link: *const dc_link) -> c_int;
}
extern "C" {
    pub fn dc_link_get_target_backlight_pwm(link: *const dc_link) -> c_int;
}
extern "C" {
    pub fn dc_link_get_psr_state(dc_link: *const dc_link, state: *mut dc_psr_state) -> bool;
}
//
// Communicate with DMUB to allow or disallow Panel Replay on the specified link:
//
// @link: pointer to the dc_link struct instance
// @enable: enable(active) or disable(inactive) replay
// @wait: state transition need to wait the active set completed.
// @force_static: force disable(inactive) the replay
// @power_opts: set power optimazation parameters to DMUB.
//
// return: allow Replay active will return true, else will return false.
//
extern "C" {
    pub fn dc_link_get_replay_state(dc_link: *const dc_link, state: *mut u64) -> bool;
}
//
// Enable or disable Panel Replay on the specified link:
//
// @link: pointer to the dc_link struct instance
// @enable: enable or disable Panel Replay
//
// return: true if successful, false otherwise
//
extern "C" {
    pub fn dc_link_set_pr_enable(link: *mut dc_link, enable: bool) -> bool;
}
//
// Update Panel Replay state parameters:
//
// @link: pointer to the dc_link struct instance
// @update_state_data: pointer to state update data structure
//
// return: true if successful, false otherwise
//
// Send general command to Panel Replay firmware:
//
// @link: pointer to the dc_link struct instance
// @general_cmd_data: pointer to general command data structure
//
// return: true if successful, false otherwise
//
// Measure Panel Replay residency for the given link.
// mode: PR_RESIDENCY_MODE_PHY, PR_RESIDENCY_MODE_ALPM, or
// PR_RESIDENCY_MODE_ENABLEMENT_PERIOD
//
// Get Panel Replay state:
//
// @link: pointer to the dc_link struct instance
// @state: pointer to store the Panel Replay state
//
// return: true if successful, false otherwise
//
extern "C" {
    pub fn dc_link_get_pr_state(link: *const dc_link, state: *mut u64) -> bool;
}
// On eDP links this function call will stall until T12 has elapsed.
// If the panel is not in power off state, this function will return
// immediately.
//
extern "C" {
    pub fn dc_link_wait_for_t12(link: *mut dc_link) -> bool;
}
// Determine if dp trace has been initialized to reflect upto date result
// return - true if trace is initialized and has valid data. False dp trace
// doesn't have valid result.
//
extern "C" {
    pub fn dc_dp_trace_is_initialized(link: *mut dc_link) -> bool;
}
// Query a dp trace flag to indicate if the current dp trace data has been
// logged before
//
// Set dp trace flag to indicate whether DM has already logged the current dp
// trace data. DM can set is_logged to true upon logging and check
// dc_dp_trace_is_logged before logging to avoid logging the same result twice.
//
// Obtain driver time stamp for last dp link training end. The time stamp is
// formatted based on dm_get_timestamp DM function.
// @in_detection - true to get link training end time stamp of last link
// training in detection sequence. false to get link training end time stamp
// of last link training in commit (dpms) sequence
//
// Get how many link training attempts dc has done with latest sequence.
// @in_detection - true to get link training count of last link
// training in detection sequence. false to get link training count of last link
// training in commit (dpms) sequence
//
// Get how many link loss has happened since last link training attempts
extern "C" {
    pub fn dc_dp_trace_get_link_loss_count(link: *mut dc_link) -> c_uint;
}
//
// USB4 DPIA BW ALLOCATION PUBLIC FUNCTIONS
//
// Send a request from DP-Tx requesting to allocate BW remotely after
// allocating it locally. This will get processed by CM and a CB function
// will be called.
//
// @link: pointer to the dc_link struct instance
// @req_bw: The requested bw in Kbyte to allocated
//
// return: none
//
extern "C" {
    pub fn dc_link_set_usb4_req_bw_req(link: *mut dc_link, req_bw: c_int);
}
//
// Handle the USB4 BW Allocation related functionality here:
// Plug => Try to allocate max bw from timing parameters supported by the sink
// Unplug => de-allocate bw
//
// @link: pointer to the dc_link struct instance
// @peak_bw: Peak bw used by the link/sink
//
// Calculates the DP tunneling bandwidth required for the stream timing
// and aggregates the stream bandwidth for the respective DP tunneling link
//
// return: dc_status
//
extern "C" {
    pub fn dc_link_validate_dp_tunneling_bandwidth(dc: *const dc, new_ctx: *const dc_state) -> dc_status;
}
//
// Get if ALPM is supported by the link
//
// Sink Interfaces - A sink corresponds to a display output device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_container_id {
// 128bit GUID in binary form
    pub guid: [c_uchar; 16],
// 8 byte port ID -> ELD.PortID
    pub portId: [c_uint; 2],
// 128bit GUID in binary formufacturer name -> ELD.ManufacturerName
    pub manufacturerName: c_ushort,
// 2 byte product code -> ELD.ProductCode
    pub productCode: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_sink_dsc_caps {
// 'true' if these are virtual DPCD's DSC caps (immediately upstream of sink in MST topology),
// 'false' if they are sink's DSC caps
    pub is_virtual_dpcd_dsc: bool,
// 'true' if MST topology supports DSC passthrough for sink
// 'false' if MST topology does not support DSC passthrough
    pub is_dsc_passthrough_supported: bool,
    pub dsc_dec_caps: dsc_dec_dpcd_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_sink_hblank_expansion_caps {
// 'true' if these are virtual DPCD's HBlank expansion caps (immediately upstream of sink in MST topology),
// 'false' if they are sink's HBlank expansion caps
    pub is_virtual_dpcd_hblank_expansion: bool,
    pub dpcd_caps: hblank_expansion_dpcd_caps,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_sink_fec_caps {
    pub is_rx_fec_supported: bool,
    pub is_topology_fec_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scdc_caps {
    pub manufacturer_OUI: hdmi_scdc_manufacturer_OUI_data,
    pub device_id: hdmi_scdc_device_id_data,
}

//
// The sink structure contains EDID and other display device properties
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_sink {
    pub sink_signal: signal_type,
    pub /: *mut *mut dc_edid dc_edid; / raw edid,
    pub /: *mut *mut dc_edid_caps edid_caps; / parse display caps,
    pub dc_container_id: *mut dc_container_id,
    pub dongle_max_pix_clk: u32,
    pub priv: *mut c_void,
    pub features_3d: [stereo_3d_features; TIMING_3D_FORMAT_MAX],
    pub converter_disable_audio: bool,
    pub mccs_caps: mccs_caps,
    pub scdc_caps: scdc_caps,
    pub dsc_caps: dc_sink_dsc_caps,
    pub fec_caps: dc_sink_fec_caps,
    pub hblank_expansion_caps: dc_sink_hblank_expansion_caps,
    pub is_vsc_sdp_colorimetry_supported: bool,
// private to DC core
    pub link: *mut dc_link,
    pub ctx: *mut dc_context,
    pub sink_id: u32,
// private to dc_sink.c
// refcount must be the last member in dc_sink, since we want the
// sink structure to be logically cloneable up to (but not including)
// refcount
    pub refcount: kref,
}

extern "C" {
    pub fn dc_sink_retain(sink: *mut dc_sink);
}
extern "C" {
    pub fn dc_sink_release(sink: *mut dc_sink);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_sink_init_data {
    pub sink_signal: signal_type,
    pub link: *mut dc_link,
    pub dongle_max_pix_clk: u32,
    pub converter_disable_audio: bool,
}

// Newer interfaces
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_cursor {
    pub address: dc_plane_address,
    pub attributes: dc_cursor_attributes,
}

// Interrupt interfaces
extern "C" {
    pub fn dc_interrupt_set(dc: *mut dc, src: dc_irq_source, enable: bool) -> bool;
}
extern "C" {
    pub fn dc_interrupt_ack(dc: *mut dc, src: dc_irq_source);
}
extern "C" {
    pub fn dc_get_flip_pending_on_otg(dc: *mut dc, otg_inst: c_int) -> bool;
}
extern "C" {
    pub fn dc_notify_vsync_int_state(dc: *mut dc, stream: *mut dc_stream_state, enable: bool);
}
// Power Interfaces
extern "C" {
    pub fn dc_resume(dc: *mut dc);
}
extern "C" {
    pub fn dc_power_down_on_boot(dc: *mut dc);
}
extern "C" {
    pub fn dc_disable_dangling_timing_generators(dc: *mut dc);
}
//
// HDCP Interfaces
//
extern "C" {
    pub fn dc_is_dmcu_initialized(dc: *mut dc) -> bool;
}
extern "C" {
    pub fn dc_set_clock(dc: *mut dc, clock_type: dc_clock_type, clk_khz: u32, stepping: u32) -> dc_status;
}
extern "C" {
    pub fn dc_get_clock(dc: *mut dc, clock_type: dc_clock_type, clock_cfg: *mut dc_clock_config);
}

extern "C" {
    pub fn dc_allow_idle_optimizations_internal(dc: *mut dc, allow: bool, caller_name: *const c_char);
}
extern "C" {
    pub fn dc_exit_ips_for_hw_access_internal(dc: *mut dc, caller_name: *const c_char);
}
extern "C" {
    pub fn dc_dmub_is_ips_idle_state(dc: *mut dc) -> bool;
}
// set min and max memory clock to lowest and highest DPM level, respectively
extern "C" {
    pub fn dc_unlock_memory_clock_frequency(dc: *mut dc);
}
// set min memory clock to the min required for current mode, max to maxDPM
extern "C" {
    pub fn dc_lock_memory_clock_frequency(dc: *mut dc);
}
// set soft max for memclk, to be used for AC/DC switching clock limitations
extern "C" {
    pub fn dc_enable_dcmode_clk_limit(dc: *mut dc, enable: bool);
}
// cleanup on driver unload
extern "C" {
    pub fn dc_hardware_release(dc: *mut dc);
}
// disables fw based mclk switch
extern "C" {
    pub fn dc_mclk_switch_using_fw_based_vblank_stretch_shut_down(dc: *mut dc);
}
extern "C" {
    pub fn dc_set_psr_allow_active(dc: *mut dc, enable: bool) -> bool;
}
extern "C" {
    pub fn dc_set_replay_allow_active(dc: *mut dc, active: bool) -> bool;
}
extern "C" {
    pub fn dc_set_ips_disable(dc: *mut dc, disable_ips: c_uint) -> bool;
}
extern "C" {
    pub fn dc_z10_restore(dc: *const dc);
}
extern "C" {
    pub fn dc_z10_save_init(dc: *mut dc);
}
extern "C" {
    pub fn dc_is_dmub_outbox_supported(dc: *mut dc) -> bool;
}
extern "C" {
    pub fn dc_enable_dmub_notifications(dc: *mut dc) -> bool;
}
extern "C" {
    pub fn dc_enable_dmub_outbox(dc: *mut dc);
}
//
// smart power OLED Interfaces
//
extern "C" {
    pub fn dc_smart_power_oled_get_max_cll(link: *const dc_link, pCurrent_MaxCLL: *mut c_uint) -> bool;
}
// Get dc link index from dpia port index
extern "C" {
    pub fn dc_process_dmub_dpia_set_tps_notification(dc: *const dc, link_index: u32, tps: u8);
}
extern "C" {
    pub fn dc_print_dmub_diagnostic_data(dc: *const dc);
}
extern "C" {
    pub fn dc_query_current_properties(dc: *mut dc, properties: *mut dc_current_properties);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_power_profile {
    pub /: *mut *mut int power_level; / Lower is better,
}

extern "C" {
    pub fn dc_get_power_profile_for_dc_state(context: *const dc_state) -> dc_power_profile;
}
extern "C" {
    pub fn dc_get_det_buffer_size_from_state(context: *const dc_state) -> c_uint;
}
extern "C" {
    pub fn dc_get_host_router_index(link: *const dc_link, host_router_index: *mut c_uint) -> bool;
}
extern "C" {
    pub fn dc_log_preos_dmcub_info(dc: *const dc);
}
// DSC Interfaces

// Disable acc mode Interfaces
extern "C" {
    pub fn dc_disable_accelerated_mode(dc: *mut dc);
}
extern "C" {
    pub fn dc_is_cursor_limit_pending(dc: *mut dc) -> bool;
}
extern "C" {
    pub fn dc_can_clear_cursor_limit(dc: *const dc) -> bool;
}
//
// dc_get_underflow_debug_data_for_otg() - Retrieve underflow debug data.
//
// @dc: Pointer to the display core context.
// @primary_otg_inst: Instance index of the primary OTG that underflowed.
// @out_data: Pointer to a dc_underflow_debug_data struct to be filled with debug information.
//
// This function collects and logs underflow-related HW states when underflow happens,
// including OTG underflow status, current read positions, frame count, and per-HUBP debug data.
// The results are stored in the provided out_data structure for further analysis or logging.
//
extern "C" {
    pub fn dc_get_underflow_debug_data_for_otg(dc: *mut dc, primary_otg_inst: c_uint, out_data: *mut dc_underflow_debug_data);
}
extern "C" {
    pub fn dc_get_power_feature_status(dc: *mut dc, primary_otg_inst: c_uint, out_data: *mut power_features);
}
//
// Software state variables used to program register fields across the display pipeline
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_register_software_state {
// HUBP register programming variables for each pipe
    pub valid_plane_state: bool,
    pub valid_stream: bool,
    pub min_dc_gfx_version9: bool,
    pub /: *mut *mut uint32_t vtg_sel; / DCHUBP_CNTL->HUBP_VTG_SEL from pipe_ctx->stream_res.tg->inst,
    pub /: *mut *mut uint32_t hubp_clock_enable; / HUBP_CLK_CNTL->HUBP_CLOCK_ENABLE from power management,
    pub /: *mut *mut uint32_t surface_pixel_format; / DCSURF_SURFACE_CONFIG->SURFACE_PIXEL_FORMAT from plane_state->format,
    pub /: *mut *mut uint32_t rotation_angle; / DCSURF_SURFACE_CONFIG->ROTATION_ANGLE from plane_state->rotation,
    pub /: *mut *mut uint32_t h_mirror_en; / DCSURF_SURFACE_CONFIG->H_MIRROR_EN from plane_state->horizontal_mirror,
    pub /: *mut *mut uint32_t surface_dcc_en; / DCSURF_SURFACE_CONTROL->PRIMARY_SURFACE_DCC_EN from dcc->enable,
    pub /: *mut *mut uint32_t surface_size_width; / HUBP_SIZE->SURFACE_SIZE_WIDTH from plane_size.surface_size.width,
    pub /: *mut *mut uint32_t surface_size_height; / HUBP_SIZE->SURFACE_SIZE_HEIGHT from plane_size.surface_size.height,
    pub /: *mut *mut uint32_t pri_viewport_width; / DCSURF_PRI_VIEWPORT_DIMENSION->PRI_VIEWPORT_WIDTH from scaler_data.viewport.width,
    pub /: *mut *mut uint32_t pri_viewport_height; / DCSURF_PRI_VIEWPORT_DIMENSION->PRI_VIEWPORT_HEIGHT from scaler_data.viewport.height,
    pub /: *mut *mut uint32_t pri_viewport_x_start; / DCSURF_PRI_VIEWPORT_START->PRI_VIEWPORT_X_START from scaler_data.viewport.x,
    pub /: *mut *mut uint32_t pri_viewport_y_start; / DCSURF_PRI_VIEWPORT_START->PRI_VIEWPORT_Y_START from scaler_data.viewport.y,
    pub /: *mut *mut uint32_t cursor_enable; / CURSOR_CONTROL->CURSOR_ENABLE from cursor_attributes.enable,
    pub /: *mut *mut uint32_t cursor_width; / CURSOR_SETTINGS->CURSOR_WIDTH from cursor_position.width,
    pub /: *mut *mut uint32_t cursor_height; / CURSOR_SETTINGS->CURSOR_HEIGHT from cursor_position.height,
// Additional DCC configuration
    pub /: *mut *mut uint32_t surface_dcc_ind_64b_blk; / DCSURF_SURFACE_CONTROL->PRIMARY_SURFACE_DCC_IND_64B_BLK from dcc.independent_64b_blks,
    pub /: *mut *mut uint32_t surface_dcc_ind_128b_blk; / DCSURF_SURFACE_CONTROL->PRIMARY_SURFACE_DCC_IND_128B_BLK from dcc.independent_128b_blks,
// Surface pitch configuration
    pub /: *mut *mut uint32_t surface_pitch; / DCSURF_SURFACE_PITCH->PITCH from plane_size.surface_pitch,
    pub /: *mut *mut uint32_t meta_pitch; / DCSURF_SURFACE_PITCH->META_PITCH from dcc.meta_pitch,
    pub /: *mut *mut uint32_t chroma_pitch; / DCSURF_SURFACE_PITCH_C->PITCH_C from plane_size.chroma_pitch,
    pub /: *mut *mut uint32_t meta_pitch_c; / DCSURF_SURFACE_PITCH_C->META_PITCH_C from dcc.meta_pitch_c,
// Surface addresses
    pub /: *mut *mut uint32_t primary_surface_address_low; / DCSURF_PRIMARY_SURFACE_ADDRESS->PRIMARY_SURFACE_ADDRESS from address.grph.addr.low_part,
    pub /: *mut *mut uint32_t primary_surface_address_high; / DCSURF_PRIMARY_SURFACE_ADDRESS_HIGH->PRIMARY_SURFACE_ADDRESS_HIGH from address.grph.addr.high_part,
    pub /: *mut *mut uint32_t primary_meta_surface_address_low; / DCSURF_PRIMARY_META_SURFACE_ADDRESS->PRIMARY_META_SURFACE_ADDRESS from address.grph.meta_addr.low_part,
    pub /: *mut *mut uint32_t primary_meta_surface_address_high; / DCSURF_PRIMARY_META_SURFACE_ADDRESS_HIGH->PRIMARY_META_SURFACE_ADDRESS_HIGH from address.grph.meta_addr.high_part,
// TMZ configuration
    pub /: *mut *mut uint32_t primary_surface_tmz; / DCSURF_SURFACE_CONTROL->PRIMARY_SURFACE_TMZ from address.tmz_surface,
    pub /: *mut *mut uint32_t primary_meta_surface_tmz; / DCSURF_SURFACE_CONTROL->PRIMARY_META_SURFACE_TMZ from address.tmz_surface,
// Tiling configuration
    pub /: *mut *mut uint32_t sw_mode; / DCSURF_TILING_CONFIG->SW_MODE from tiling_info.gfx9.swizzle,
    pub /: *mut *mut uint32_t num_pipes; / DCSURF_ADDR_CONFIG->NUM_PIPES from tiling_info.gfx9.num_pipes,
    pub /: *mut *mut uint32_t num_banks; / DCSURF_ADDR_CONFIG->NUM_BANKS from tiling_info.gfx9.num_banks,
    pub /: *mut *mut uint32_t pipe_interleave; / DCSURF_ADDR_CONFIG->PIPE_INTERLEAVE from tiling_info.gfx9.pipe_interleave,
    pub /: *mut *mut uint32_t num_shader_engines; / DCSURF_ADDR_CONFIG->NUM_SE from tiling_info.gfx9.num_shader_engines,
    pub /: *mut *mut uint32_t num_rb_per_se; / DCSURF_ADDR_CONFIG->NUM_RB_PER_SE from tiling_info.gfx9.num_rb_per_se,
    pub /: *mut *mut uint32_t num_pkrs; / DCSURF_ADDR_CONFIG->NUM_PKRS from tiling_info.gfx9.num_pkrs,
// DML Request Size Configuration - Luma
    pub /: *mut *mut uint32_t rq_chunk_size; / DCHUBP_REQ_SIZE_CONFIG->CHUNK_SIZE from rq_regs.rq_regs_l.chunk_size,
    pub /: *mut *mut uint32_t rq_min_chunk_size; / DCHUBP_REQ_SIZE_CONFIG->MIN_CHUNK_SIZE from rq_regs.rq_regs_l.min_chunk_size,
    pub /: *mut *mut uint32_t rq_meta_chunk_size; / DCHUBP_REQ_SIZE_CONFIG->META_CHUNK_SIZE from rq_regs.rq_regs_l.meta_chunk_size,
    pub /: *mut *mut uint32_t rq_min_meta_chunk_size; / DCHUBP_REQ_SIZE_CONFIG->MIN_META_CHUNK_SIZE from rq_regs.rq_regs_l.min_meta_chunk_size,
    pub /: *mut *mut uint32_t rq_dpte_group_size; / DCHUBP_REQ_SIZE_CONFIG->DPTE_GROUP_SIZE from rq_regs.rq_regs_l.dpte_group_size,
    pub /: *mut *mut uint32_t rq_mpte_group_size; / DCHUBP_REQ_SIZE_CONFIG->MPTE_GROUP_SIZE from rq_regs.rq_regs_l.mpte_group_size,
    pub /: *mut *mut uint32_t rq_swath_height_l; / DCHUBP_REQ_SIZE_CONFIG->SWATH_HEIGHT_L from rq_regs.rq_regs_l.swath_height,
    pub /: *mut *mut uint32_t rq_pte_row_height_l; / DCHUBP_REQ_SIZE_CONFIG->PTE_ROW_HEIGHT_L from rq_regs.rq_regs_l.pte_row_height,
// DML Request Size Configuration - Chroma
    pub /: *mut *mut uint32_t rq_chunk_size_c; / DCHUBP_REQ_SIZE_CONFIG_C->CHUNK_SIZE_C from rq_regs.rq_regs_c.chunk_size,
    pub /: *mut *mut uint32_t rq_min_chunk_size_c; / DCHUBP_REQ_SIZE_CONFIG_C->MIN_CHUNK_SIZE_C from rq_regs.rq_regs_c.min_chunk_size,
    pub /: *mut *mut uint32_t rq_meta_chunk_size_c; / DCHUBP_REQ_SIZE_CONFIG_C->META_CHUNK_SIZE_C from rq_regs.rq_regs_c.meta_chunk_size,
    pub /: *mut *mut uint32_t rq_min_meta_chunk_size_c; / DCHUBP_REQ_SIZE_CONFIG_C->MIN_META_CHUNK_SIZE_C from rq_regs.rq_regs_c.min_meta_chunk_size,
    pub /: *mut *mut uint32_t rq_dpte_group_size_c; / DCHUBP_REQ_SIZE_CONFIG_C->DPTE_GROUP_SIZE_C from rq_regs.rq_regs_c.dpte_group_size,
    pub /: *mut *mut uint32_t rq_mpte_group_size_c; / DCHUBP_REQ_SIZE_CONFIG_C->MPTE_GROUP_SIZE_C from rq_regs.rq_regs_c.mpte_group_size,
    pub /: *mut *mut uint32_t rq_swath_height_c; / DCHUBP_REQ_SIZE_CONFIG_C->SWATH_HEIGHT_C from rq_regs.rq_regs_c.swath_height,
    pub /: *mut *mut uint32_t rq_pte_row_height_c; / DCHUBP_REQ_SIZE_CONFIG_C->PTE_ROW_HEIGHT_C from rq_regs.rq_regs_c.pte_row_height,
// DML Expansion Modes
    pub /: *mut *mut uint32_t drq_expansion_mode; / DCN_EXPANSION_MODE->DRQ_EXPANSION_MODE from rq_regs.drq_expansion_mode,
    pub /: *mut *mut uint32_t prq_expansion_mode; / DCN_EXPANSION_MODE->PRQ_EXPANSION_MODE from rq_regs.prq_expansion_mode,
    pub /: *mut *mut uint32_t mrq_expansion_mode; / DCN_EXPANSION_MODE->MRQ_EXPANSION_MODE from rq_regs.mrq_expansion_mode,
    pub /: *mut *mut uint32_t crq_expansion_mode; / DCN_EXPANSION_MODE->CRQ_EXPANSION_MODE from rq_regs.crq_expansion_mode,
// DML DLG parameters - nominal
    pub /: *mut *mut uint32_t dst_y_per_vm_vblank; / NOM_PARAMETERS_0->DST_Y_PER_VM_VBLANK from dlg_regs.dst_y_per_vm_vblank,
    pub /: *mut *mut uint32_t dst_y_per_row_vblank; / NOM_PARAMETERS_0->DST_Y_PER_ROW_VBLANK from dlg_regs.dst_y_per_row_vblank,
    pub /: *mut *mut uint32_t dst_y_per_vm_flip; / NOM_PARAMETERS_1->DST_Y_PER_VM_FLIP from dlg_regs.dst_y_per_vm_flip,
    pub /: *mut *mut uint32_t dst_y_per_row_flip; / NOM_PARAMETERS_1->DST_Y_PER_ROW_FLIP from dlg_regs.dst_y_per_row_flip,
// DML prefetch settings
    pub /: *mut *mut uint32_t dst_y_prefetch; / PREFETCH_SETTINS->DST_Y_PREFETCH from dlg_regs.dst_y_prefetch,
    pub /: *mut *mut uint32_t vratio_prefetch; / PREFETCH_SETTINS->VRATIO_PREFETCH from dlg_regs.vratio_prefetch,
    pub /: *mut *mut uint32_t vratio_prefetch_c; / PREFETCH_SETTINS_C->VRATIO_PREFETCH_C from dlg_regs.vratio_prefetch_c,
// TTU parameters
    pub /: *mut *mut uint32_t qos_level_low_wm; / TTU_CNTL1->QoSLevelLowWaterMark from ttu_regs.qos_level_low_wm,
    pub /: *mut *mut uint32_t qos_level_high_wm; / TTU_CNTL1->QoSLevelHighWaterMark from ttu_regs.qos_level_high_wm,
    pub /: *mut *mut uint32_t qos_level_flip; / TTU_CNTL2->QoS_LEVEL_FLIP_L from ttu_regs.qos_level_flip,
    pub /: *mut *mut uint32_t min_ttu_vblank; / DCN_GLOBAL_TTU_CNTL->MIN_TTU_VBLANK from ttu_regs.min_ttu_vblank,
    pub hubp: [}; MAX_PIPES],
// HUBBUB register programming variables
// Individual DET buffer control per pipe - software state that programs DET registers
    pub /: *mut *mut uint32_t det0_size; / DCHUBBUB_DET0_CTRL->DET0_SIZE from hubbub->funcs->program_det_size(hubbub, 0, det_buffer_size_kb),
    pub /: *mut *mut uint32_t det1_size; / DCHUBBUB_DET1_CTRL->DET1_SIZE from hubbub->funcs->program_det_size(hubbub, 1, det_buffer_size_kb),
    pub /: *mut *mut uint32_t det2_size; / DCHUBBUB_DET2_CTRL->DET2_SIZE from hubbub->funcs->program_det_size(hubbub, 2, det_buffer_size_kb),
    pub /: *mut *mut uint32_t det3_size; / DCHUBBUB_DET3_CTRL->DET3_SIZE from hubbub->funcs->program_det_size(hubbub, 3, det_buffer_size_kb),
// Compression buffer control - software state that programs COMPBUF registers
    pub /: *mut *mut uint32_t compbuf_size; / DCHUBBUB_COMPBUF_CTRL->COMPBUF_SIZE from hubbub->funcs->program_compbuf_size(hubbub, compbuf_size_kb, safe_to_increase),
    pub /: *mut *mut uint32_t compbuf_reserved_space_64b; / COMPBUF_RESERVED_SPACE->COMPBUF_RESERVED_SPACE_64B from hubbub2->pixel_chunk_size / 32,
    pub /: *mut *mut uint32_t compbuf_reserved_space_zs; / COMPBUF_RESERVED_SPACE->COMPBUF_RESERVED_SPACE_ZS from hubbub2->pixel_chunk_size / 128,
    pub hubbub: },
// DPP register programming variables for each pipe (simplified for available fields)
    pub /: *mut *mut uint32_t dpp_clock_enable; / DPP_CONTROL->DPP_CLOCK_ENABLE from dppclk_enable,
// Recout (Rectangle of Interest) configuration
    pub /: *mut *mut uint32_t recout_start_x; / RECOUT_START->RECOUT_START_X from pipe_ctx->plane_res.scl_data.recout.x,
    pub /: *mut *mut uint32_t recout_start_y; / RECOUT_START->RECOUT_START_Y from pipe_ctx->plane_res.scl_data.recout.y,
    pub /: *mut *mut uint32_t recout_width; / RECOUT_SIZE->RECOUT_WIDTH from pipe_ctx->plane_res.scl_data.recout.width,
    pub /: *mut *mut uint32_t recout_height; / RECOUT_SIZE->RECOUT_HEIGHT from pipe_ctx->plane_res.scl_data.recout.height,
// MPC (Multiple Pipe/Plane Combiner) size configuration
    pub /: *mut *mut uint32_t mpc_width; / MPC_SIZE->MPC_WIDTH from pipe_ctx->plane_res.scl_data.h_active,
    pub /: *mut *mut uint32_t mpc_height; / MPC_SIZE->MPC_HEIGHT from pipe_ctx->plane_res.scl_data.v_active,
// DSCL mode configuration
    pub /: *mut *mut uint32_t dscl_mode; / SCL_MODE->DSCL_MODE from pipe_ctx->plane_res.scl_data.dscl_prog_data.dscl_mode,
// Scaler ratios (simplified to integer parts)
    pub /: *mut *mut uint32_t horz_ratio_int; / SCL_HORZ_FILTER_SCALE_RATIO->SCL_H_SCALE_RATIO integer part from ratios.horz,
    pub /: *mut *mut uint32_t vert_ratio_int; / SCL_VERT_FILTER_SCALE_RATIO->SCL_V_SCALE_RATIO integer part from ratios.vert,
// Basic scaler taps
    pub /: *mut *mut uint32_t h_taps; / SCL_TAP_CONTROL->SCL_H_NUM_TAPS from taps.h_taps,
    pub /: *mut *mut uint32_t v_taps; / SCL_TAP_CONTROL->SCL_V_NUM_TAPS from taps.v_taps,
    pub dpp: [}; MAX_PIPES],
// DCCG register programming variables
// Core Display Clock Control
    pub /: *mut *mut uint32_t dispclk_khz; / DENTIST_DISPCLK_CNTL->DENTIST_DISPCLK_WDIVIDER from clk_mgr.dispclk_khz,
    pub /: *mut *mut uint32_t dc_mem_global_pwr_req_dis; / DC_MEM_GLOBAL_PWR_REQ_CNTL->DC_MEM_GLOBAL_PWR_REQ_DIS from memory power management settings,
// DPP Clock Control - 4 fields per pipe
    pub /: *mut *mut uint32_t dppclk_khz[MAX_PIPES]; / DPPCLK_CTRL->DPPCLK_R_GATE_DISABLE from dpp_clocks[pipe],
    pub /: *mut *mut uint32_t dppclk_enable[MAX_PIPES]; / DPPCLK_CTRL->DPPCLK0_EN,DPPCLK1_EN,DPPCLK2_EN,DPPCLK3_EN from dccg31_update_dpp_dto(),
    pub /: *mut *mut uint32_t dppclk_dto_enable[MAX_PIPES]; / DPPCLK_DTO_CTRL->DPPCLK_DTO_ENABLE from dccg->dpp_clock_gated[dpp_inst] state,
    pub /: *mut *mut uint32_t dppclk_dto_phase[MAX_PIPES]; / DPPCLK0_DTO_PARAM->DPPCLK0_DTO_PHASE from phase calculation req_dppclk/ref_dppclk,
    pub /: *mut *mut uint32_t dppclk_dto_modulo[MAX_PIPES]; / DPPCLK0_DTO_PARAM->DPPCLK0_DTO_MODULO from modulo = 0xff,
// DSC Clock Control - 4 fields per DSC resource
    pub /: *mut *mut uint32_t dscclk_khz[MAX_PIPES]; / DSCCLK_DTO_CTRL->DSCCLK_DTO_ENABLE from dsc_clocks,
    pub /: *mut *mut uint32_t dscclk_dto_enable[MAX_PIPES]; / DSCCLK_DTO_CTRL->DSCCLK0_DTO_ENABLE,DSCCLK1_DTO_ENABLE,DSCCLK2_DTO_ENABLE,DSCCLK3_DTO_ENABLE,
    pub /: *mut *mut uint32_t dscclk_dto_phase[MAX_PIPES]; / DSCCLK0_DTO_PARAM->DSCCLK0_DTO_PHASE from dccg31_enable_dscclk(),
    pub /: *mut *mut uint32_t dscclk_dto_modulo[MAX_PIPES]; / DSCCLK0_DTO_PARAM->DSCCLK0_DTO_MODULO from dccg31_enable_dscclk(),
// Pixel Clock Control - per pipe
    pub /: *mut *mut uint32_t pixclk_khz[MAX_PIPES]; / PIXCLK_RESYNC_CNTL->PIXCLK_RESYNC_ENABLE from stream.timing.pix_clk_100hz,
    pub /: *mut *mut uint32_t otg_pixel_rate_div[MAX_PIPES]; / OTG_PIXEL_RATE_DIV->OTG_PIXEL_RATE_DIV from OTG pixel rate divider control,
    pub /: *mut *mut uint32_t dtbclk_dto_enable[MAX_PIPES]; / OTG0_PIXEL_RATE_CNTL->DTBCLK_DTO_ENABLE from dccg31_set_dtbclk_dto(),
    pub /: *mut *mut uint32_t pipe_dto_src_sel[MAX_PIPES]; / OTG0_PIXEL_RATE_CNTL->PIPE_DTO_SRC_SEL from dccg31_set_dtbclk_dto() source selection,
    pub /: *mut *mut uint32_t dtbclk_dto_div[MAX_PIPES]; / OTG0_PIXEL_RATE_CNTL->DTBCLK_DTO_DIV from dtbdto_div calculation,
    pub /: *mut *mut uint32_t otg_add_pixel[MAX_PIPES]; / OTG0_PIXEL_RATE_CNTL->OTG_ADD_PIXEL from dccg31_otg_add_pixel(),
    pub /: *mut *mut uint32_t otg_drop_pixel[MAX_PIPES]; / OTG0_PIXEL_RATE_CNTL->OTG_DROP_PIXEL from dccg31_otg_drop_pixel(),
// DTBCLK DTO Control - 4 DTOs
    pub /: *mut *mut uint32_t dtbclk_dto_modulo[4]; / DTBCLK_DTO0_MODULO->DTBCLK_DTO0_MODULO from dccg31_set_dtbclk_dto() modulo calculation,
    pub /: *mut *mut uint32_t dtbclk_dto_phase[4]; / DTBCLK_DTO0_PHASE->DTBCLK_DTO0_PHASE from phase calculation pixclk_khz/ref_dtbclk_khz,
    pub /: *mut *mut uint32_t dtbclk_dto_dbuf_en; / DTBCLK_DTO_DBUF_EN->DTBCLK DTO data buffer enable,
// DP Stream Clock Control - 4 pipes
    pub /: *mut *mut uint32_t dpstreamclk_enable[MAX_PIPES]; / DPSTREAMCLK_CNTL->DPSTREAMCLK_PIPE0_EN,DPSTREAMCLK_PIPE1_EN,DPSTREAMCLK_PIPE2_EN,DPSTREAMCLK_PIPE3_EN,
    pub /: *mut *mut uint32_t dp_dto_modulo[4]; / DP_DTO0_MODULO->DP_DTO0_MODULO from DP stream DTO programming,
    pub /: *mut *mut uint32_t dp_dto_phase[4]; / DP_DTO0_PHASE->DP_DTO0_PHASE from DP stream DTO programming,
    pub /: *mut *mut uint32_t dp_dto_dbuf_en; / DP_DTO_DBUF_EN->DP DTO data buffer enable,
// PHY Symbol Clock Control - 5 PHYs (A,B,C,D,E)
    pub /: *mut *mut uint32_t phy_symclk_force_en[5]; / PHYASYMCLK_CLOCK_CNTL->PHYASYMCLK_FORCE_EN from dccg31_set_physymclk() force_enable,
    pub /: *mut *mut uint32_t phy_symclk_force_src_sel[5]; / PHYASYMCLK_CLOCK_CNTL->PHYASYMCLK_FORCE_SRC_SEL from dccg31_set_physymclk() clk_src,
    pub /: *mut *mut uint32_t phy_symclk_gate_disable[5]; / DCCG_GATE_DISABLE_CNTL2->PHYASYMCLK_GATE_DISABLE from debug.root_clock_optimization.bits.physymclk,
// SYMCLK32 SE Control - 4 instances
    pub /: *mut *mut uint32_t symclk32_se_src_sel[4]; / SYMCLK32_SE_CNTL->SYMCLK32_SE0_SRC_SEL from dccg31_enable_symclk32_se() with get_phy_mux_symclk() mapping,
    pub /: *mut *mut uint32_t symclk32_se_enable[4]; / SYMCLK32_SE_CNTL->SYMCLK32_SE0_EN from dccg31_enable_symclk32_se() enable,
    pub /: *mut *mut uint32_t symclk32_se_gate_disable[4]; / DCCG_GATE_DISABLE_CNTL3->SYMCLK32_SE0_GATE_DISABLE from debug.root_clock_optimization.bits.symclk32_se,
// SYMCLK32 LE Control - 2 instances
    pub /: *mut *mut uint32_t symclk32_le_src_sel[2]; / SYMCLK32_LE_CNTL->SYMCLK32_LE0_SRC_SEL from dccg31_enable_symclk32_le() phyd32clk source,
    pub /: *mut *mut uint32_t symclk32_le_enable[2]; / SYMCLK32_LE_CNTL->SYMCLK32_LE0_EN from dccg31_enable_symclk32_le() enable,
    pub /: *mut *mut uint32_t symclk32_le_gate_disable[2]; / DCCG_GATE_DISABLE_CNTL3->SYMCLK32_LE0_GATE_DISABLE from debug.root_clock_optimization.bits.symclk32_le,
// HDMI Clock Control
    pub /: *mut *mut uint32_t hdmicharclk_enable; / HDMICHARCLK0_CLOCK_CNTL->HDMICHARCLK0_EN from dccg31_enable_hdmicharclk(),
    pub /: *mut *mut uint32_t hdmicharclk_src_sel; / HDMICHARCLK0_CLOCK_CNTL->HDMICHARCLK0_SRC_SEL from dccg31_enable_hdmicharclk() phypll_inst source,
    pub /: *mut *mut uint32_t hdmistreamclk_src_sel; / HDMISTREAMCLK_CNTL->HDMISTREAMCLK0_SRC_SEL from dccg31_set_hdmistreamclk() src selection,
    pub /: *mut *mut uint32_t hdmistreamclk_dto_force_dis; / HDMISTREAMCLK_CNTL->HDMISTREAMCLK0_DTO_FORCE_DIS from dccg31_set_hdmistreamclk() DTO force bypass,
    pub /: *mut *mut uint32_t hdmistreamclk_dto_phase; / HDMISTREAMCLK0_DTO_PARAM->HDMISTREAMCLK0_DTO_PHASE from dccg31_disable_hdmistreamclk(),
    pub /: *mut *mut uint32_t hdmistreamclk_dto_modulo; / HDMISTREAMCLK0_DTO_PARAM->HDMISTREAMCLK0_DTO_MODULO from dccg31_disable_hdmistreamclk(),
// DPIA Clock Control
    pub /: *mut *mut uint32_t dpiaclk_540m_dto_modulo; / DPIACLK_540M_DTO_MODULO->DPIA 540MHz DTO modulo,
    pub /: *mut *mut uint32_t dpiaclk_540m_dto_phase; / DPIACLK_540M_DTO_PHASE->DPIA 540MHz DTO phase,
    pub /: *mut *mut uint32_t dpiaclk_810m_dto_modulo; / DPIACLK_810M_DTO_MODULO->DPIA 810MHz DTO modulo,
    pub /: *mut *mut uint32_t dpiaclk_810m_dto_phase; / DPIACLK_810M_DTO_PHASE->DPIA 810MHz DTO phase,
    pub /: *mut *mut uint32_t dpiaclk_dto_cntl; / DPIACLK_DTO_CNTL->DPIA clock DTO control,
    pub /: *mut *mut uint32_t dpiasymclk_cntl; / DPIASYMCLK_CNTL->DPIA symbol clock control,
// Clock Gating Control
    pub /: *mut *mut uint32_t dccg_gate_disable_cntl; / DCCG_GATE_DISABLE_CNTL->Clock gate disable control from dccg31_init(),
    pub /: *mut *mut uint32_t dpstreamclk_gate_disable; / DCCG_GATE_DISABLE_CNTL3->DPSTREAMCLK_GATE_DISABLE from debug.root_clock_optimization.bits.dpstream,
    pub /: *mut *mut uint32_t dpstreamclk_root_gate_disable; / DCCG_GATE_DISABLE_CNTL3->DPSTREAMCLK_ROOT_GATE_DISABLE from debug.root_clock_optimization.bits.dpstream,
// VSync Control
    pub /: *mut *mut uint32_t vsync_cnt_ctrl; / DCCG_VSYNC_CNT_CTRL->VSync counter control,
    pub /: *mut *mut uint32_t vsync_cnt_int_ctrl; / DCCG_VSYNC_CNT_INT_CTRL->VSync counter interrupt control,
    pub /: *mut *mut uint32_t vsync_otg_latch_value[6]; / DCCG_VSYNC_OTG0_LATCH_VALUE->OTG0 VSync latch value (for OTG0-5),
// Time Base Control
    pub /: *mut *mut uint32_t microsecond_time_base_div; / MICROSECOND_TIME_BASE_DIV->Microsecond time base divider,
    pub /: *mut *mut uint32_t millisecond_time_base_div; / MILLISECOND_TIME_BASE_DIV->Millisecond time base divider,
    pub dccg: },
// DSC essential configuration for underflow analysis
// DSC active state - critical for bandwidth analysis
    pub /: *mut *mut uint32_t dsc_clock_enable; / DSC enabled - affects bandwidth requirements,
// DSC configuration affecting bandwidth and timing
    pub /: *mut *mut uint32_t dsc_num_slices_h; / Horizontal slice count - affects throughput,
    pub /: *mut *mut uint32_t dsc_num_slices_v; / Vertical slice count - affects throughput,
    pub /: *mut *mut uint32_t dsc_bits_per_pixel; / Compression ratio - affects bandwidth,
// OPP integration - affects pipeline flow
    pub /: *mut *mut uint32_t dscrm_dsc_forward_enable; / DSC forwarding to OPP enabled,
    pub /: *mut *mut uint32_t dscrm_dsc_opp_pipe_source; / Which OPP receives DSC output,
    pub dsc: [}; MAX_PIPES],
// MPC register programming variables
// MPCC blending tree and mode control
    pub /: *mut *mut uint32_t mpcc_mode[MAX_PIPES]; / MPCC_CONTROL->MPCC_MODE from blend_cfg.blend_mode,
    pub /: *mut *mut uint32_t mpcc_alpha_blend_mode[MAX_PIPES]; / MPCC_CONTROL->MPCC_ALPHA_BLND_MODE from blend_cfg.alpha_mode,
    pub /: *mut *mut uint32_t mpcc_alpha_multiplied_mode[MAX_PIPES]; / MPCC_CONTROL->MPCC_ALPHA_MULTIPLIED_MODE from blend_cfg.pre_multiplied_alpha,
    pub /: *mut *mut uint32_t mpcc_blnd_active_overlap_only[MAX_PIPES]; / MPCC_CONTROL->MPCC_BLND_ACTIVE_OVERLAP_ONLY from blend_cfg.overlap_only,
    pub /: *mut *mut uint32_t mpcc_global_alpha[MAX_PIPES]; / MPCC_CONTROL->MPCC_GLOBAL_ALPHA from blend_cfg.global_alpha,
    pub /: *mut *mut uint32_t mpcc_global_gain[MAX_PIPES]; / MPCC_CONTROL->MPCC_GLOBAL_GAIN from blend_cfg.global_gain,
    pub /: *mut *mut uint32_t mpcc_bg_bpc[MAX_PIPES]; / MPCC_CONTROL->MPCC_BG_BPC from background color depth,
    pub /: *mut *mut uint32_t mpcc_bot_gain_mode[MAX_PIPES]; / MPCC_CONTROL->MPCC_BOT_GAIN_MODE from bottom layer gain control,
// MPCC blending tree connections
    pub /: *mut *mut uint32_t mpcc_bot_sel[MAX_PIPES]; / MPCC_BOT_SEL->MPCC_BOT_SEL from mpcc_state->bot_sel,
    pub /: *mut *mut uint32_t mpcc_top_sel[MAX_PIPES]; / MPCC_TOP_SEL->MPCC_TOP_SEL from mpcc_state->dpp_id,
// MPCC output gamma control
    pub /: *mut *mut uint32_t mpcc_ogam_mode[MAX_PIPES]; / MPCC_OGAM_CONTROL->MPCC_OGAM_MODE from output gamma mode,
    pub /: *mut *mut uint32_t mpcc_ogam_select[MAX_PIPES]; / MPCC_OGAM_CONTROL->MPCC_OGAM_SELECT from gamma LUT bank selection,
    pub /: *mut *mut uint32_t mpcc_ogam_pwl_disable[MAX_PIPES]; / MPCC_OGAM_CONTROL->MPCC_OGAM_PWL_DISABLE from PWL control,
// MPCC pipe assignment and status
    pub /: *mut *mut uint32_t mpcc_opp_id[MAX_PIPES]; / MPCC_OPP_ID->MPCC_OPP_ID from mpcc_state->opp_id,
    pub /: *mut *mut uint32_t mpcc_idle[MAX_PIPES]; / MPCC_STATUS->MPCC_IDLE from mpcc idle status,
    pub /: *mut *mut uint32_t mpcc_busy[MAX_PIPES]; / MPCC_STATUS->MPCC_BUSY from mpcc busy status,
// MPC output processing
    pub /: *mut *mut uint32_t mpc_out_csc_mode; / MPC_OUT_CSC_COEF->MPC_OUT_CSC_MODE from output_csc,
    pub /: *mut *mut uint32_t mpc_out_gamma_mode; / MPC_OUT_GAMMA_LUT->MPC_OUT_GAMMA_MODE from output_gamma,
    pub mpc: },
// OPP register programming variables for each pipe
// Display Pattern Generator (DPG) Control - 19 fields from DPG_CONTROL register
    pub /: *mut *mut uint32_t dpg_enable; / DPG_CONTROL->DPG_EN from test_pattern parameter (enable/disable),
// Format Control (FMT) - 18 fields from FMT_CONTROL register
    pub /: *mut *mut uint32_t fmt_pixel_encoding; / FMT_CONTROL->FMT_PIXEL_ENCODING from clamping->pixel_encoding,
    pub /: *mut *mut uint32_t fmt_subsampling_mode; / FMT_CONTROL->FMT_SUBSAMPLING_MODE from force_chroma_subsampling_1tap,
    pub /: *mut *mut uint32_t fmt_cbcr_bit_reduction_bypass; / FMT_CONTROL->FMT_CBCR_BIT_REDUCTION_BYPASS from pixel_encoding bypass control,
    pub /: *mut *mut uint32_t fmt_stereosync_override; / FMT_CONTROL->FMT_STEREOSYNC_OVERRIDE from stereo timing override,
    pub /: *mut *mut uint32_t fmt_spatial_dither_frame_counter_max; / FMT_CONTROL->FMT_SPATIAL_DITHER_FRAME_COUNTER_MAX from fmt_bit_depth->flags,
    pub /: *mut *mut uint32_t fmt_spatial_dither_frame_counter_bit_swap; / FMT_CONTROL->FMT_SPATIAL_DITHER_FRAME_COUNTER_BIT_SWAP from dither control,
    pub /: *mut *mut uint32_t fmt_truncate_enable; / FMT_CONTROL->FMT_TRUNCATE_EN from fmt_bit_depth->flags.TRUNCATE_ENABLED,
    pub /: *mut *mut uint32_t fmt_truncate_depth; / FMT_CONTROL->FMT_TRUNCATE_DEPTH from fmt_bit_depth->flags.TRUNCATE_DEPTH,
    pub /: *mut *mut uint32_t fmt_truncate_mode; / FMT_CONTROL->FMT_TRUNCATE_MODE from fmt_bit_depth->flags.TRUNCATE_MODE,
    pub /: *mut *mut uint32_t fmt_spatial_dither_enable; / FMT_CONTROL->FMT_SPATIAL_DITHER_EN from fmt_bit_depth->flags.SPATIAL_DITHER_ENABLED,
    pub /: *mut *mut uint32_t fmt_spatial_dither_mode; / FMT_CONTROL->FMT_SPATIAL_DITHER_MODE from fmt_bit_depth->flags.SPATIAL_DITHER_MODE,
    pub /: *mut *mut uint32_t fmt_spatial_dither_depth; / FMT_CONTROL->FMT_SPATIAL_DITHER_DEPTH from fmt_bit_depth->flags.SPATIAL_DITHER_DEPTH,
    pub /: *mut *mut uint32_t fmt_temporal_dither_enable; / FMT_CONTROL->FMT_TEMPORAL_DITHER_EN from fmt_bit_depth->flags.TEMPORAL_DITHER_ENABLED,
    pub /: *mut *mut uint32_t fmt_clamp_data_enable; / FMT_CONTROL->FMT_CLAMP_DATA_EN from clamping->clamping_range enable,
    pub /: *mut *mut uint32_t fmt_clamp_color_format; / FMT_CONTROL->FMT_CLAMP_COLOR_FORMAT from clamping->color_format,
    pub /: *mut *mut uint32_t fmt_dynamic_exp_enable; / FMT_CONTROL->FMT_DYNAMIC_EXP_EN from color_sp/color_dpth/signal,
    pub /: *mut *mut uint32_t fmt_dynamic_exp_mode; / FMT_CONTROL->FMT_DYNAMIC_EXP_MODE from color space mode mapping,
    pub /: *mut *mut uint32_t fmt_bit_depth_control; / Legacy field - kept for compatibility,
// OPP Pipe Control - 1 field from OPP_PIPE_CONTROL register
    pub /: *mut *mut uint32_t opp_pipe_clock_enable; / OPP_PIPE_CONTROL->OPP_PIPE_CLOCK_EN from enable parameter (bool),
// OPP CRC Control - 3 fields from OPP_PIPE_CRC_CONTROL register
    pub /: *mut *mut uint32_t opp_crc_enable; / OPP_PIPE_CRC_CONTROL->CRC_EN from CRC enable control,
    pub /: *mut *mut uint32_t opp_crc_select_source; / OPP_PIPE_CRC_CONTROL->CRC_SELECT_SOURCE from CRC source selection,
    pub /: *mut *mut uint32_t opp_crc_stereo_cont; / OPP_PIPE_CRC_CONTROL->CRC_STEREO_CONT from stereo continuous CRC,
// Output Buffer (OPPBUF) Control - 6 fields from OPPBUF_CONTROL register
    pub /: *mut *mut uint32_t oppbuf_active_width; / OPPBUF_CONTROL->OPPBUF_ACTIVE_WIDTH from oppbuf_params->active_width,
    pub /: *mut *mut uint32_t oppbuf_pixel_repetition; / OPPBUF_CONTROL->OPPBUF_PIXEL_REPETITION from oppbuf_params->pixel_repetition,
    pub /: *mut *mut uint32_t oppbuf_display_segmentation; / OPPBUF_CONTROL->OPPBUF_DISPLAY_SEGMENTATION from oppbuf_params->mso_segmentation,
    pub /: *mut *mut uint32_t oppbuf_overlap_pixel_num; / OPPBUF_CONTROL->OPPBUF_OVERLAP_PIXEL_NUM from oppbuf_params->mso_overlap_pixel_num,
    pub /: *mut *mut uint32_t oppbuf_3d_vact_space1_size; / OPPBUF_CONTROL->OPPBUF_3D_VACT_SPACE1_SIZE from 3D timing space1_size,
    pub /: *mut *mut uint32_t oppbuf_3d_vact_space2_size; / OPPBUF_CONTROL->OPPBUF_3D_VACT_SPACE2_SIZE from 3D timing space2_size,
// DSC Forward Config - 3 fields from DSCRM_DSC_FORWARD_CONFIG register
    pub /: *mut *mut uint32_t dscrm_dsc_forward_enable; / DSCRM_DSC_FORWARD_CONFIG->DSCRM_DSC_FORWARD_EN from DSC forward enable control,
    pub /: *mut *mut uint32_t dscrm_dsc_opp_pipe_source; / DSCRM_DSC_FORWARD_CONFIG->DSCRM_DSC_OPP_PIPE_SOURCE from opp_pipe parameter,
    pub /: *mut *mut uint32_t dscrm_dsc_forward_enable_status; / DSCRM_DSC_FORWARD_CONFIG->DSCRM_DSC_FORWARD_EN_STATUS from DSC forward status (read-only),
    pub opp: [}; MAX_PIPES],
// OPTC register programming variables for each pipe
    pub otg_master_inst: u32,
// OTG_CONTROL register - 5 fields for OTG control
    pub /: *mut *mut uint32_t otg_master_enable; / OTG_CONTROL->OTG_MASTER_EN from timing enable/disable control,
    pub /: *mut *mut uint32_t otg_disable_point_cntl; / OTG_CONTROL->OTG_DISABLE_POINT_CNTL from disable timing control,
    pub /: *mut *mut uint32_t otg_start_point_cntl; / OTG_CONTROL->OTG_START_POINT_CNTL from start timing control,
    pub /: *mut *mut uint32_t otg_field_number_cntl; / OTG_CONTROL->OTG_FIELD_NUMBER_CNTL from interlace field control,
    pub /: *mut *mut uint32_t otg_out_mux; / OTG_CONTROL->OTG_OUT_MUX from output mux selection,
// OTG Horizontal Timing - 7 fields
    pub /: *mut *mut uint32_t otg_h_total; / OTG_H_TOTAL->OTG_H_TOTAL from dc_crtc_timing->h_total,
    pub /: *mut *mut uint32_t otg_h_blank_start; / OTG_H_BLANK_START_END->OTG_H_BLANK_START from dc_crtc_timing->h_front_porch,
    pub /: *mut *mut uint32_t otg_h_blank_end; / OTG_H_BLANK_START_END->OTG_H_BLANK_END from dc_crtc_timing->h_addressable_video_pixel_width,
    pub /: *mut *mut uint32_t otg_h_sync_start; / OTG_H_SYNC_A->OTG_H_SYNC_A_START from dc_crtc_timing->h_sync_width,
    pub /: *mut *mut uint32_t otg_h_sync_end; / OTG_H_SYNC_A->OTG_H_SYNC_A_END from calculated sync end position,
    pub /: *mut *mut uint32_t otg_h_sync_polarity; / OTG_H_SYNC_A_CNTL->OTG_H_SYNC_A_POL from dc_crtc_timing->flags.HSYNC_POSITIVE_POLARITY,
    pub /: *mut *mut uint32_t otg_h_timing_div_mode; / OTG_H_TIMING_CNTL->OTG_H_TIMING_DIV_MODE from horizontal timing division mode,
// OTG Vertical Timing - 7 fields
    pub /: *mut *mut uint32_t otg_v_total; / OTG_V_TOTAL->OTG_V_TOTAL from dc_crtc_timing->v_total,
    pub /: *mut *mut uint32_t otg_v_blank_start; / OTG_V_BLANK_START_END->OTG_V_BLANK_START from dc_crtc_timing->v_front_porch,
    pub /: *mut *mut uint32_t otg_v_blank_end; / OTG_V_BLANK_START_END->OTG_V_BLANK_END from dc_crtc_timing->v_addressable_video_line_width,
    pub /: *mut *mut uint32_t otg_v_sync_start; / OTG_V_SYNC_A->OTG_V_SYNC_A_START from dc_crtc_timing->v_sync_width,
    pub /: *mut *mut uint32_t otg_v_sync_end; / OTG_V_SYNC_A->OTG_V_SYNC_A_END from calculated sync end position,
    pub /: *mut *mut uint32_t otg_v_sync_polarity; / OTG_V_SYNC_A_CNTL->OTG_V_SYNC_A_POL from dc_crtc_timing->flags.VSYNC_POSITIVE_POLARITY,
    pub /: *mut *mut uint32_t otg_v_sync_mode; / OTG_V_SYNC_A_CNTL->OTG_V_SYNC_MODE from sync mode selection,
// OTG DRR (Dynamic Refresh Rate) Control - 8 fields
    pub /: *mut *mut uint32_t otg_v_total_max; / OTG_V_TOTAL_MAX->OTG_V_TOTAL_MAX from drr_params->vertical_total_max,
    pub /: *mut *mut uint32_t otg_v_total_min; / OTG_V_TOTAL_MIN->OTG_V_TOTAL_MIN from drr_params->vertical_total_min,
    pub /: *mut *mut uint32_t otg_v_total_mid; / OTG_V_TOTAL_MID->OTG_V_TOTAL_MID from drr_params->vertical_total_mid,
    pub /: *mut *mut uint32_t otg_v_total_max_sel; / OTG_V_TOTAL_CONTROL->OTG_V_TOTAL_MAX_SEL from DRR max selection enable,
    pub /: *mut *mut uint32_t otg_v_total_min_sel; / OTG_V_TOTAL_CONTROL->OTG_V_TOTAL_MIN_SEL from DRR min selection enable,
    pub /: *mut *mut uint32_t otg_vtotal_mid_replacing_max_en; / OTG_V_TOTAL_CONTROL->OTG_VTOTAL_MID_REPLACING_MAX_EN from DRR mid-frame enable,
    pub /: *mut *mut uint32_t otg_vtotal_mid_frame_num; / OTG_V_TOTAL_CONTROL->OTG_VTOTAL_MID_FRAME_NUM from drr_params->vertical_total_mid_frame_num,
    pub /: *mut *mut uint32_t otg_set_v_total_min_mask; / OTG_V_TOTAL_CONTROL->OTG_SET_V_TOTAL_MIN_MASK from DRR trigger mask,
    pub /: *mut *mut uint32_t otg_force_lock_on_event; / OTG_V_TOTAL_CONTROL->OTG_FORCE_LOCK_ON_EVENT from DRR force lock control,
// OPTC Data Source and ODM - 6 fields
    pub /: *mut *mut uint32_t optc_seg0_src_sel; / OPTC_DATA_SOURCE_SELECT->OPTC_SEG0_SRC_SEL from opp_id[0] ODM segment 0 source,
    pub /: *mut *mut uint32_t optc_seg1_src_sel; / OPTC_DATA_SOURCE_SELECT->OPTC_SEG1_SRC_SEL from opp_id[1] ODM segment 1 source,
    pub /: *mut *mut uint32_t optc_seg2_src_sel; / OPTC_DATA_SOURCE_SELECT->OPTC_SEG2_SRC_SEL from opp_id[2] ODM segment 2 source,
    pub /: *mut *mut uint32_t optc_seg3_src_sel; / OPTC_DATA_SOURCE_SELECT->OPTC_SEG3_SRC_SEL from opp_id[3] ODM segment 3 source,
    pub /: *mut *mut uint32_t optc_num_of_input_segment; / OPTC_DATA_SOURCE_SELECT->OPTC_NUM_OF_INPUT_SEGMENT from opp_cnt-1 number of input segments,
    pub /: *mut *mut uint32_t optc_mem_sel; / OPTC_MEMORY_CONFIG->OPTC_MEM_SEL from memory_mask ODM memory selection,
// OPTC Data Format and DSC - 4 fields
    pub /: *mut *mut uint32_t optc_data_format; / OPTC_DATA_FORMAT_CONTROL->OPTC_DATA_FORMAT from data format selection,
    pub /: *mut *mut uint32_t optc_dsc_mode; / OPTC_DATA_FORMAT_CONTROL->OPTC_DSC_MODE from dsc_mode parameter,
    pub /: *mut *mut uint32_t optc_dsc_bytes_per_pixel; / OPTC_BYTES_PER_PIXEL->OPTC_DSC_BYTES_PER_PIXEL from dsc_bytes_per_pixel parameter,
    pub /: *mut *mut uint32_t optc_segment_width; / OPTC_WIDTH_CONTROL->OPTC_SEGMENT_WIDTH from segment_width parameter,
    pub /: *mut *mut uint32_t optc_dsc_slice_width; / OPTC_WIDTH_CONTROL->OPTC_DSC_SLICE_WIDTH from dsc_slice_width parameter,
// OPTC Clock and Underflow Control - 4 fields
    pub /: *mut *mut uint32_t optc_input_pix_clk_en; / OPTC_INPUT_CLOCK_CONTROL->OPTC_INPUT_PIX_CLK_EN from pixel clock enable,
    pub /: *mut *mut uint32_t optc_underflow_occurred_status; / OPTC_INPUT_GLOBAL_CONTROL->OPTC_UNDERFLOW_OCCURRED_STATUS from underflow status (read-only),
    pub /: *mut *mut uint32_t optc_underflow_clear; / OPTC_INPUT_GLOBAL_CONTROL->OPTC_UNDERFLOW_CLEAR from underflow clear control,
    pub /: *mut *mut uint32_t otg_clock_enable; / OTG_CLOCK_CONTROL->OTG_CLOCK_EN from OTG clock enable,
    pub /: *mut *mut uint32_t otg_clock_gate_dis; / OTG_CLOCK_CONTROL->OTG_CLOCK_GATE_DIS from clock gate disable,
// OTG Stereo and 3D Control - 6 fields
    pub /: *mut *mut uint32_t otg_stereo_enable; / OTG_STEREO_CONTROL->OTG_STEREO_EN from stereo enable control,
    pub /: *mut *mut uint32_t otg_stereo_sync_output_line_num; / OTG_STEREO_CONTROL->OTG_STEREO_SYNC_OUTPUT_LINE_NUM from timing->stereo_3d_format line num,
    pub /: *mut *mut uint32_t otg_stereo_sync_output_polarity; / OTG_STEREO_CONTROL->OTG_STEREO_SYNC_OUTPUT_POLARITY from stereo polarity control,
    pub /: *mut *mut uint32_t otg_3d_structure_en; / OTG_3D_STRUCTURE_CONTROL->OTG_3D_STRUCTURE_EN from 3D structure enable,
    pub /: *mut *mut uint32_t otg_3d_structure_v_update_mode; / OTG_3D_STRUCTURE_CONTROL->OTG_3D_STRUCTURE_V_UPDATE_MODE from 3D vertical update mode,
    pub /: *mut *mut uint32_t otg_3d_structure_stereo_sel_ovr; / OTG_3D_STRUCTURE_CONTROL->OTG_3D_STRUCTURE_STEREO_SEL_OVR from 3D stereo selection override,
    pub /: *mut *mut uint32_t otg_interlace_enable; / OTG_INTERLACE_CONTROL->OTG_INTERLACE_ENABLE from dc_crtc_timing->flags.INTERLACE,
// OTG GSL (Global Sync Lock) Control - 5 fields
    pub /: *mut *mut uint32_t otg_gsl0_en; / OTG_GSL_CONTROL->OTG_GSL0_EN from GSL group 0 enable,
    pub /: *mut *mut uint32_t otg_gsl1_en; / OTG_GSL_CONTROL->OTG_GSL1_EN from GSL group 1 enable,
    pub /: *mut *mut uint32_t otg_gsl2_en; / OTG_GSL_CONTROL->OTG_GSL2_EN from GSL group 2 enable,
    pub /: *mut *mut uint32_t otg_gsl_master_en; / OTG_GSL_CONTROL->OTG_GSL_MASTER_EN from GSL master enable,
    pub /: *mut *mut uint32_t otg_gsl_master_mode; / OTG_GSL_CONTROL->OTG_GSL_MASTER_MODE from gsl_params->gsl_master mode,
// OTG DRR Advanced Control - 4 fields
    pub /: *mut *mut uint32_t otg_v_total_last_used_by_drr; / OTG_DRR_CONTROL->OTG_V_TOTAL_LAST_USED_BY_DRR from last used DRR V_TOTAL (read-only),
    pub /: *mut *mut uint32_t otg_drr_trigger_window_start_x; / OTG_DRR_TRIGGER_WINDOW->OTG_DRR_TRIGGER_WINDOW_START_X from window_start parameter,
    pub /: *mut *mut uint32_t otg_drr_trigger_window_end_x; / OTG_DRR_TRIGGER_WINDOW->OTG_DRR_TRIGGER_WINDOW_END_X from window_end parameter,
    pub /: *mut *mut uint32_t otg_drr_v_total_change_limit; / OTG_DRR_V_TOTAL_CHANGE->OTG_DRR_V_TOTAL_CHANGE_LIMIT from limit parameter,
// OTG DSC Position Control - 2 fields
    pub /: *mut *mut uint32_t otg_dsc_start_position_x; / OTG_DSC_START_POSITION->OTG_DSC_START_POSITION_X from DSC start X position,
    pub /: *mut *mut uint32_t otg_dsc_start_position_line_num; / OTG_DSC_START_POSITION->OTG_DSC_START_POSITION_LINE_NUM from DSC start line number,
// OTG Double Buffer Control - 2 fields
    pub /: *mut *mut uint32_t otg_drr_timing_dbuf_update_mode; / OTG_DOUBLE_BUFFER_CONTROL->OTG_DRR_TIMING_DBUF_UPDATE_MODE from DRR double buffer mode,
    pub /: *mut *mut uint32_t otg_blank_data_double_buffer_en; / OTG_DOUBLE_BUFFER_CONTROL->OTG_BLANK_DATA_DOUBLE_BUFFER_EN from blank data double buffer enable,
// OTG Vertical Interrupts - 6 fields
    pub /: *mut *mut uint32_t otg_vertical_interrupt0_int_enable; / OTG_VERTICAL_INTERRUPT0_CONTROL->OTG_VERTICAL_INTERRUPT0_INT_ENABLE from interrupt 0 enable,
    pub /: *mut *mut uint32_t otg_vertical_interrupt0_line_start; / OTG_VERTICAL_INTERRUPT0_POSITION->OTG_VERTICAL_INTERRUPT0_LINE_START from start_line parameter,
    pub /: *mut *mut uint32_t otg_vertical_interrupt1_int_enable; / OTG_VERTICAL_INTERRUPT1_CONTROL->OTG_VERTICAL_INTERRUPT1_INT_ENABLE from interrupt 1 enable,
    pub /: *mut *mut uint32_t otg_vertical_interrupt1_line_start; / OTG_VERTICAL_INTERRUPT1_POSITION->OTG_VERTICAL_INTERRUPT1_LINE_START from start_line parameter,
    pub /: *mut *mut uint32_t otg_vertical_interrupt2_int_enable; / OTG_VERTICAL_INTERRUPT2_CONTROL->OTG_VERTICAL_INTERRUPT2_INT_ENABLE from interrupt 2 enable,
    pub /: *mut *mut uint32_t otg_vertical_interrupt2_line_start; / OTG_VERTICAL_INTERRUPT2_POSITION->OTG_VERTICAL_INTERRUPT2_LINE_START from start_line parameter,
// OTG Global Sync Parameters - 6 fields
    pub /: *mut *mut uint32_t otg_vready_offset; / OTG_VREADY_PARAM->OTG_VREADY_OFFSET from vready_offset parameter,
    pub /: *mut *mut uint32_t otg_vstartup_start; / OTG_VSTARTUP_PARAM->OTG_VSTARTUP_START from vstartup_start parameter,
    pub /: *mut *mut uint32_t otg_vupdate_offset; / OTG_VUPDATE_PARAM->OTG_VUPDATE_OFFSET from vupdate_offset parameter,
    pub /: *mut *mut uint32_t otg_vupdate_width; / OTG_VUPDATE_PARAM->OTG_VUPDATE_WIDTH from vupdate_width parameter,
    pub /: *mut *mut uint32_t master_update_lock_vupdate_keepout_start_offset; / OTG_VUPDATE_KEEPOUT->MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_START_OFFSET from pstate_keepout start,
    pub /: *mut *mut uint32_t master_update_lock_vupdate_keepout_end_offset; / OTG_VUPDATE_KEEPOUT->MASTER_UPDATE_LOCK_VUPDATE_KEEPOUT_END_OFFSET from pstate_keepout end,
// OTG Manual Trigger Control - 11 fields
    pub /: *mut *mut uint32_t otg_triga_source_select; / OTG_TRIGA_CNTL->OTG_TRIGA_SOURCE_SELECT from trigger A source selection,
    pub /: *mut *mut uint32_t otg_triga_source_pipe_select; / OTG_TRIGA_CNTL->OTG_TRIGA_SOURCE_PIPE_SELECT from trigger A pipe selection,
    pub /: *mut *mut uint32_t otg_triga_rising_edge_detect_cntl; / OTG_TRIGA_CNTL->OTG_TRIGA_RISING_EDGE_DETECT_CNTL from trigger A rising edge detect,
    pub /: *mut *mut uint32_t otg_triga_falling_edge_detect_cntl; / OTG_TRIGA_CNTL->OTG_TRIGA_FALLING_EDGE_DETECT_CNTL from trigger A falling edge detect,
    pub /: *mut *mut uint32_t otg_triga_polarity_select; / OTG_TRIGA_CNTL->OTG_TRIGA_POLARITY_SELECT from trigger A polarity selection,
    pub /: *mut *mut uint32_t otg_triga_frequency_select; / OTG_TRIGA_CNTL->OTG_TRIGA_FREQUENCY_SELECT from trigger A frequency selection,
    pub /: *mut *mut uint32_t otg_triga_delay; / OTG_TRIGA_CNTL->OTG_TRIGA_DELAY from trigger A delay,
    pub /: *mut *mut uint32_t otg_triga_clear; / OTG_TRIGA_CNTL->OTG_TRIGA_CLEAR from trigger A clear,
    pub /: *mut *mut uint32_t otg_triga_manual_trig; / OTG_TRIGA_MANUAL_TRIG->OTG_TRIGA_MANUAL_TRIG from manual trigger A,
    pub /: *mut *mut uint32_t otg_trigb_source_select; / OTG_TRIGB_CNTL->OTG_TRIGB_SOURCE_SELECT from trigger B source selection,
    pub /: *mut *mut uint32_t otg_trigb_polarity_select; / OTG_TRIGB_CNTL->OTG_TRIGB_POLARITY_SELECT from trigger B polarity selection,
    pub /: *mut *mut uint32_t otg_trigb_manual_trig; / OTG_TRIGB_MANUAL_TRIG->OTG_TRIGB_MANUAL_TRIG from manual trigger B,
// OTG Static Screen and Update Control - 6 fields
    pub /: *mut *mut uint32_t otg_static_screen_event_mask; / OTG_STATIC_SCREEN_CONTROL->OTG_STATIC_SCREEN_EVENT_MASK from event_triggers parameter,
    pub /: *mut *mut uint32_t otg_static_screen_frame_count; / OTG_STATIC_SCREEN_CONTROL->OTG_STATIC_SCREEN_FRAME_COUNT from num_frames parameter,
    pub /: *mut *mut uint32_t master_update_lock; / OTG_MASTER_UPDATE_LOCK->MASTER_UPDATE_LOCK from update lock control,
    pub /: *mut *mut uint32_t master_update_mode; / OTG_MASTER_UPDATE_MODE->MASTER_UPDATE_MODE from update mode selection,
    pub /: *mut *mut uint32_t otg_force_count_now_mode; / OTG_FORCE_COUNT_NOW_CNTL->OTG_FORCE_COUNT_NOW_MODE from force count mode,
    pub /: *mut *mut uint32_t otg_force_count_now_clear; / OTG_FORCE_COUNT_NOW_CNTL->OTG_FORCE_COUNT_NOW_CLEAR from force count clear,
// VTG Control - 3 fields
    pub /: *mut *mut uint32_t vtg0_enable; / CONTROL->VTG0_ENABLE from VTG enable control,
    pub /: *mut *mut uint32_t vtg0_fp2; / CONTROL->VTG0_FP2 from VTG front porch 2,
    pub /: *mut *mut uint32_t vtg0_vcount_init; / CONTROL->VTG0_VCOUNT_INIT from VTG vertical count init,
// OTG Status (Read-Only) - 12 fields
    pub /: *mut *mut uint32_t otg_v_blank; / OTG_STATUS->OTG_V_BLANK from vertical blank status (read-only),
    pub /: *mut *mut uint32_t otg_v_active_disp; / OTG_STATUS->OTG_V_ACTIVE_DISP from vertical active display (read-only),
    pub /: *mut *mut uint32_t otg_frame_count; / OTG_STATUS_FRAME_COUNT->OTG_FRAME_COUNT from frame count (read-only),
    pub /: *mut *mut uint32_t otg_horz_count; / OTG_STATUS_POSITION->OTG_HORZ_COUNT from horizontal position (read-only),
    pub /: *mut *mut uint32_t otg_vert_count; / OTG_STATUS_POSITION->OTG_VERT_COUNT from vertical position (read-only),
    pub /: *mut *mut uint32_t otg_horz_count_hv; / OTG_STATUS_HV_COUNT->OTG_HORZ_COUNT from horizontal count (read-only),
    pub /: *mut *mut uint32_t otg_vert_count_nom; / OTG_STATUS_HV_COUNT->OTG_VERT_COUNT_NOM from vertical count nominal (read-only),
    pub /: *mut *mut uint32_t otg_flip_pending; / OTG_PIPE_UPDATE_STATUS->OTG_FLIP_PENDING from flip pending status (read-only),
    pub /: *mut *mut uint32_t otg_dc_reg_update_pending; / OTG_PIPE_UPDATE_STATUS->OTG_DC_REG_UPDATE_PENDING from DC register update pending (read-only),
    pub /: *mut *mut uint32_t otg_cursor_update_pending; / OTG_PIPE_UPDATE_STATUS->OTG_CURSOR_UPDATE_PENDING from cursor update pending (read-only),
    pub /: *mut *mut uint32_t otg_vupdate_keepout_status; / OTG_PIPE_UPDATE_STATUS->OTG_VUPDATE_KEEPOUT_STATUS from VUPDATE keepout status (read-only),
    pub optc: [}; MAX_PIPES],
// Metadata
    pub active_pipe_count: u32,
    pub active_stream_count: u32,
    pub state_valid: bool,
}

//
// dc_capture_register_software_state() - Capture software state for register programming
// @dc: DC context containing current display configuration
// @state: Pointer to dc_register_software_state structure to populate
//
// Extracts all software state variables that are used to program hardware register
// fields across the display driver pipeline. This provides a complete snapshot
// of the software configuration that drives hardware register programming.
//
// The function traverses the DC context and extracts values from:
// - Stream configurations (timing, format, DSC settings)
// - Plane states (surface format, rotation, scaling, cursor)
// - Pipe contexts (resource allocation, blending, viewport)
// - Clock manager (display clocks, DPP clocks, pixel clocks)
// - Resource context (DET buffer allocation, ODM configuration)
//
// This is essential for underflow debugging as it captures the exact software
// state that determines how registers are programmed, allowing analysis of
// whether underflow is caused by incorrect register programming or timing issues.
//
// Return: true if state was successfully captured, false on error
//
extern "C" {
    pub fn dc_capture_register_software_state(dc: *mut dc, state: *mut dc_register_software_state) -> bool;
}
//
// dc_get_qos_info() - Retrieve Quality of Service (QoS) information from display core
// @dc: DC context containing current display configuration
// @info: Pointer to dc_qos_info structure to populate with QoS metrics
//
// This function retrieves QoS metrics from the display core that can be used by
// benchmark tools to analyze display system performance. The function may take
// several milliseconds to execute due to hardware measurement requirements.
//
// QoS information includes:
// - Bandwidth bounds (lower limits in Mbps)
// - Latency bounds (upper limits in nanoseconds)
// - Hardware-measured bandwidth metrics (peak/average in Mbps)
// - Hardware-measured latency metrics (maximum/average in nanoseconds)
//
// The function will populate the provided dc_qos_info structure with current
// QoS measurements. If hardware measurement functions are not available for
// the current DCN version, the function returns false with zero'd info structure.
//
// Return: true if QoS information was successfully retrieved, false if measurement
// functions are unavailable or hardware measurements cannot be performed
//
extern "C" {
    pub fn dc_get_qos_info(dc: *mut dc, info: *mut dc_qos_info) -> bool;
}
//
// dc_override_memory_bandwidth_request - Override the DCN nominal memory
// bandwidth request sent to PMFW, independent of the current display mode.
// For debug use only.
// @dc: DC instance
// @bw_mbps: requested bandwidth in MB/s; 0 clears the override
//
// Return: capped bandwidth value actually applied (MB/s)
//
