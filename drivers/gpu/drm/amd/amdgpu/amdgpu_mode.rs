//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/amdgpu/amdgpu_mode.h
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
// Copyright 2000 ATI Technologies Inc., Markham, Ontario, and
// VA Linux Systems Inc., Fremont, California.
// Copyright 2008 Red Hat Inc.
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
// Original Authors:
// Kevin E. Martin, Rickard E. Faith, Alan Hourihane
//
// Kernel port Author: Dave Airlie
//

pub const AMDGPU_MAX_HPD_PINS: c_int = 6;
pub const AMDGPU_MAX_CRTCS: c_int = 6;
pub const AMDGPU_MAX_PLANES: c_int = 6;
pub const AMDGPU_MAX_AFMT_BLOCKS: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_rmx_type {
    RMX_OFF,
    RMX_FULL,
    RMX_CENTER,
    RMX_ASPECT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_underscan_type {
    UNDERSCAN_OFF,
    UNDERSCAN_ON,
    UNDERSCAN_AUTO,
}

pub const AMDGPU_HPD_CONNECT_INT_DELAY_IN_MS: c_int = 50;
pub const AMDGPU_HPD_DISCONNECT_INT_DELAY_IN_MS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_hpd_id {
    AMDGPU_HPD_1 = 0,
    AMDGPU_HPD_2,
    AMDGPU_HPD_3,
    AMDGPU_HPD_4,
    AMDGPU_HPD_5,
    AMDGPU_HPD_6,
    AMDGPU_HPD_NONE = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_crtc_irq {
    AMDGPU_CRTC_IRQ_VBLANK1 = 0,
    AMDGPU_CRTC_IRQ_VBLANK2,
    AMDGPU_CRTC_IRQ_VBLANK3,
    AMDGPU_CRTC_IRQ_VBLANK4,
    AMDGPU_CRTC_IRQ_VBLANK5,
    AMDGPU_CRTC_IRQ_VBLANK6,
    AMDGPU_CRTC_IRQ_VLINE1,
    AMDGPU_CRTC_IRQ_VLINE2,
    AMDGPU_CRTC_IRQ_VLINE3,
    AMDGPU_CRTC_IRQ_VLINE4,
    AMDGPU_CRTC_IRQ_VLINE5,
    AMDGPU_CRTC_IRQ_VLINE6,
    AMDGPU_CRTC_IRQ_NONE = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_pageflip_irq {
    AMDGPU_PAGEFLIP_IRQ_D1 = 0,
    AMDGPU_PAGEFLIP_IRQ_D2,
    AMDGPU_PAGEFLIP_IRQ_D3,
    AMDGPU_PAGEFLIP_IRQ_D4,
    AMDGPU_PAGEFLIP_IRQ_D5,
    AMDGPU_PAGEFLIP_IRQ_D6,
    AMDGPU_PAGEFLIP_IRQ_NONE = 0xff
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_flip_status {
    AMDGPU_FLIP_NONE,
    AMDGPU_FLIP_PENDING,
    AMDGPU_FLIP_SUBMITTED
}

pub const AMDGPU_MAX_I2C_BUS: c_int = 16;
// amdgpu gpio-based i2c
// 1. "mask" reg and bits
// grabs the gpio pins for software use
// 0=not held  1=held
// 2. "a" reg and bits
// output pin value
// 0=low 1=high
// 3. "en" reg and bits
// sets the pin direction
// 0=input 1=output
// 4. "y" reg and bits
// input pin value
// 0=low 1=high
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_i2c_bus_rec {
    pub valid: bool,
// id used by atom
    pub i2c_id: u8,
// id used by atom
    pub hpd: amdgpu_hpd_id,
// can be used with hw i2c engine
    pub hw_capable: bool,
// uses multi-media i2c engine
    pub mm_i2c: bool,
// regs and bits
    pub mask_clk_reg: u32,
    pub mask_data_reg: u32,
    pub a_clk_reg: u32,
    pub a_data_reg: u32,
    pub en_clk_reg: u32,
    pub en_data_reg: u32,
    pub y_clk_reg: u32,
    pub y_data_reg: u32,
    pub mask_clk_mask: u32,
    pub mask_data_mask: u32,
    pub a_clk_mask: u32,
    pub a_data_mask: u32,
    pub en_clk_mask: u32,
    pub en_data_mask: u32,
    pub y_clk_mask: u32,
    pub y_data_mask: u32,
}

pub const AMDGPU_MAX_BIOS_CONNECTOR: c_int = 16;
// pll flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_pll {
// reference frequency
    pub reference_freq: u32,
// fixed dividers
    pub reference_div: u32,
    pub post_div: u32,
// pll in/out limits
    pub pll_in_min: u32,
    pub pll_in_max: u32,
    pub pll_out_min: u32,
    pub pll_out_max: u32,
    pub lcd_pll_out_min: u32,
    pub lcd_pll_out_max: u32,
    pub best_vco: u32,
// divider limits
    pub min_ref_div: u32,
    pub max_ref_div: u32,
    pub min_post_div: u32,
    pub max_post_div: u32,
    pub min_feedback_div: u32,
    pub max_feedback_div: u32,
    pub min_frac_feedback_div: u32,
    pub max_frac_feedback_div: u32,
// flags for the current clock
    pub flags: u32,
// pll id
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_i2c_chan {
    pub adapter: i2c_adapter,
    pub dev: *mut drm_device,
    pub bit: i2c_algo_bit_data,
    pub rec: amdgpu_i2c_bus_rec,
    pub aux: drm_dp_aux,
    pub has_aux: bool,
    pub mutex: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_afmt {
    pub enabled: bool,
    pub offset: c_int,
    pub last_buffer_filled_status: bool,
    pub id: c_int,
    pub pin: *mut amdgpu_audio_pin,
}

//
// Audio
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_audio_pin {
    pub channels: c_int,
    pub rate: c_int,
    pub bits_per_sample: c_int,
    pub status_bits: u8,
    pub category_code: u8,
    pub offset: u32,
    pub connected: bool,
    pub id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_audio {
    pub enabled: bool,
    pub pin: [amdgpu_audio_pin; AMDGPU_MAX_AFMT_BLOCKS],
    pub num_pins: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_display_funcs {
// display watermarks
    pub adev): *mut *mut void (bandwidth_update)(struct amdgpu_device,
// get frame count
    pub crtc): *mut *mut *mut u32 (vblank_get_counter)(struct amdgpu_device adev, int,
// set backlight level
    pub level): u8,
// get backlight level
    pub amdgpu_encoder): *mut *mut u8 (backlight_get_level)(struct amdgpu_encoder,
// hotplug detect
    pub hpd): *mut *mut *mut bool (hpd_sense)(struct amdgpu_device adev, enum amdgpu_hpd_id,
    pub hpd): amdgpu_hpd_id,
    pub adev): *mut *mut u32 (hpd_get_gpio_reg)(struct amdgpu_device,
// pageflipping
    pub async): int crtc_id, u64 crtc_base, bool,
    pub position): *mut *mut u32 vbl, u32,
// display topology setup
    pub caps): u16,
    pub router): *mut amdgpu_router,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_framebuffer {
    pub base: drm_framebuffer,
    pub tiling_flags: u64,
    pub tmz_surface: bool,
    pub gfx12_dcc: bool,
// caching for later use
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mode_info {
    pub atom_context: *mut atom_context,
    pub atom_card_info: *mut card_info,
    pub mode_config_initialized: bool,
    pub crtcs: [*mut amdgpu_crtc; AMDGPU_MAX_CRTCS],
    pub planes: [*mut drm_plane; AMDGPU_MAX_PLANES],
    pub afmt: [*mut amdgpu_afmt; AMDGPU_MAX_AFMT_BLOCKS],
// DVI-I properties
    pub coherent_mode_property: *mut drm_property,
// DAC enable load detect
    pub load_detect_property: *mut drm_property,
// underscan
    pub underscan_property: *mut drm_property,
    pub underscan_hborder_property: *mut drm_property,
    pub underscan_vborder_property: *mut drm_property,
// audio
    pub audio_property: *mut drm_property,
// FMT dithering
    pub dither_property: *mut drm_property,
// Adaptive Backlight Modulation (power feature)
    pub abm_level_property: *mut drm_property,
// hardcoded DFP edid from BIOS
    pub bios_hardcoded_edid: *const drm_edid,
// firmware flags
    pub firmware_flags: u32,
// pointer to backlight encoder
    pub bl_encoder: *mut amdgpu_encoder,
    pub /: *mut *mut u8 bl_level; / saved backlight level,
    pub /: *mut *mut amdgpu_audio audio; / audio stuff,
    pub /: *mut *mut int num_crtc; / number of crtcs,
    pub /: *mut *mut int num_hpd; / number of hpd pins,
    pub /: *mut *mut int num_dig; / number of dig blocks,
    pub /: *mut *mut bool gpu_vm_support; / supports display from GTT,
    pub disp_priority: c_int,
    pub funcs: *const amdgpu_display_funcs,
    pub plane_type: *const drm_plane_type,
// Driver-private color mgmt props
// @plane_degamma_lut_property: Plane property to set a degamma LUT to
// convert encoded values to light linear values before sampling or
// blending.
//
    pub plane_degamma_lut_property: *mut drm_property,
// @plane_degamma_lut_size_property: Plane property to define the max
// size of degamma LUT as supported by the driver (read-only).
//
    pub plane_degamma_lut_size_property: *mut drm_property,
//
// @plane_degamma_tf_property: Plane pre-defined transfer function to
// to go from scanout/encoded values to linear values.
//
    pub plane_degamma_tf_property: *mut drm_property,
//
// @plane_hdr_mult_property:
//
    pub plane_hdr_mult_property: *mut drm_property,
    pub plane_ctm_property: *mut drm_property,
//
// @plane_shaper_lut_property: Plane property to set pre-blending
// shaper LUT that converts color content before 3D LUT.
// If plane_shaper_tf_property != Identity TF, AMD color module will
// combine the user LUT values with pre-defined TF into the LUT
// parameters to be programmed.
//
    pub plane_shaper_lut_property: *mut drm_property,
//
// @plane_shaper_lut_size_property: Plane property for the size of
// pre-blending shaper LUT as supported by the driver (read-only).
//
    pub plane_shaper_lut_size_property: *mut drm_property,
//
// @plane_shaper_tf_property: Plane property to set a predefined
// transfer function for pre-blending shaper (before applying 3D LUT)
// with or without LUT. There is no shaper ROM, but we can use AMD
// color modules to program LUT parameters from predefined TF (or
// from a combination of pre-defined TF and the custom 1D LUT).
//
    pub plane_shaper_tf_property: *mut drm_property,
//
// @plane_lut3d_property: Plane property for color transformation using
// a 3D LUT (pre-blending), a three-dimensional array where each
// element is an RGB triplet. Each dimension has the size of
// lut3d_size. The array contains samples from the approximated
// function. On AMD, values between samples are estimated by
// tetrahedral interpolation. The array is accessed with three indices,
// one for each input dimension (color channel), blue being the
// outermost dimension, red the innermost.
//
    pub plane_lut3d_property: *mut drm_property,
//
// @plane_lut3d_size_property: Plane property to define the max size
// of 3D LUT as supported by the driver (read-only). The max size is
// the max size of one dimension and, therefore, the max number of
// entries for 3D LUT array is the 3D LUT size cubed.
//
    pub plane_lut3d_size_property: *mut drm_property,
//
// @plane_blend_lut_property: Plane property for output gamma before
// blending. Userspace set a blend LUT to convert colors after 3D LUT
// conversion. It works as a post-3DLUT 1D LUT. With shaper LUT, they
// are sandwiching 3D LUT with two 1D LUT. If plane_blend_tf_property
// != Identity TF, AMD color module will combine the user LUT values
// with pre-defined TF into the LUT parameters to be programmed.
//
    pub plane_blend_lut_property: *mut drm_property,
//
// @plane_blend_lut_size_property: Plane property to define the max
// size of blend LUT as supported by the driver (read-only).
//
    pub plane_blend_lut_size_property: *mut drm_property,
//
// @plane_blend_tf_property: Plane property to set a predefined
// transfer function for pre-blending blend/out_gamma (after applying
// 3D LUT) with or without LUT. There is no blend ROM, but we can use
// AMD color modules to program LUT parameters from predefined TF (or
// from a combination of pre-defined TF and the custom 1D LUT).
//
    pub plane_blend_tf_property: *mut drm_property,
// @regamma_tf_property: Transfer function for CRTC regamma
// (post-blending). Possible values are defined by `enum
// amdgpu_transfer_function`. There is no regamma ROM, but we can use
// AMD color modules to program LUT parameters from predefined TF (or
// from a combination of pre-defined TF and the custom 1D LUT).
//
    pub regamma_tf_property: *mut drm_property,
}

pub const AMDGPU_MAX_BL_LEVEL: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_backlight_privdata {
    pub encoder: *mut amdgpu_encoder,
    pub negative: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_atom_ss {
    pub percentage: u16,
    pub percentage_divider: u16,
    pub type: u8,
    pub step: u16,
    pub delay: u8,
    pub range: u8,
    pub refdiv: u8,
// asic_ss
    pub rate: u16,
    pub amount: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_crtc {
    pub base: drm_crtc,
    pub crtc_id: c_int,
    pub enabled: bool,
    pub can_tile: bool,
    pub crtc_offset: u32,
    pub cursor_bo: *mut drm_gem_object,
    pub cursor_addr: u64,
    pub cursor_x: c_int,
    pub cursor_y: c_int,
    pub cursor_hot_x: c_int,
    pub cursor_hot_y: c_int,
    pub cursor_width: c_int,
    pub cursor_height: c_int,
    pub max_cursor_width: c_int,
    pub max_cursor_height: c_int,
    pub rmx_type: amdgpu_rmx_type,
    pub h_border: u8,
    pub v_border: u8,
    pub vsc: fixed20_12,
    pub hsc: fixed20_12,
    pub native_mode: drm_display_mode,
    pub pll_id: u32,
// page flipping
    pub pflip_works: *mut amdgpu_flip_work,
    pub pflip_status: amdgpu_flip_status,
    pub deferred_flip_completion: c_int,
// parameters access from DM IRQ handler
    pub dm_irq_params: dm_irq_params,
// DM idle state manager
    pub ism: amdgpu_dm_ism,
// pll sharing
    pub ss: amdgpu_atom_ss,
    pub ss_enabled: bool,
    pub adjusted_clock: u32,
    pub bpc: c_int,
    pub pll_reference_div: u32,
    pub pll_post_div: u32,
    pub pll_flags: u32,
    pub encoder: *mut drm_encoder,
    pub connector: *mut drm_connector,
// for dpm
    pub line_time: u32,
    pub lb_vblank_lead_lines: u32,
    pub hw_mode: drm_display_mode,
    pub otg_inst: c_int,
    pub event: *mut drm_pending_vblank_event,
    pub wb_pending: bool,
    pub wb_frame_done: bool,
    pub wb_enabled: bool,
    pub wb_conn: *mut drm_writeback_connector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_encoder_atom_dig {
    pub linkb: bool,
// atom dig
    pub coherent_mode: bool,
    pub /: *mut *mut int dig_encoder; / -1 disabled, 0 DIGA, 1 DIGB, etc.,
// atom lvds/edp
    pub lcd_misc: u32,
    pub panel_pwr_delay: u16,
    pub lcd_ss_id: u32,
// panel mode
    pub native_mode: drm_display_mode,
    pub bl_dev: *mut backlight_device,
    pub dpms_mode: c_int,
    pub backlight_level: u8,
    pub panel_mode: c_int,
    pub afmt: *mut amdgpu_afmt,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_encoder {
    pub base: drm_encoder,
    pub encoder_enum: u32,
    pub encoder_id: u32,
    pub devices: u32,
    pub active_device: u32,
    pub flags: u32,
    pub pixel_clock: u32,
    pub rmx_type: amdgpu_rmx_type,
    pub underscan_type: amdgpu_underscan_type,
    pub underscan_hborder: u32,
    pub underscan_vborder: u32,
    pub native_mode: drm_display_mode,
    pub enc_priv: *mut c_void,
    pub audio_polling_active: c_int,
    pub is_ext_encoder: bool,
    pub caps: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_connector_atom_dig {
// displayport
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub downstream_ports: [u8; DP_MAX_DOWNSTREAM_PORTS],
    pub dp_sink_type: u8,
    pub dp_clock: c_int,
    pub dp_lane_count: c_int,
    pub edp_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_gpio_rec {
    pub valid: bool,
    pub id: u8,
    pub reg: u32,
    pub mask: u32,
    pub shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_hpd {
    pub hpd: amdgpu_hpd_id,
    pub plugged_state: u8,
    pub gpio: amdgpu_gpio_rec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_router {
    pub router_id: u32,
    pub i2c_info: amdgpu_i2c_bus_rec,
    pub i2c_addr: u8,
// i2c mux
    pub ddc_valid: bool,
    pub ddc_mux_type: u8,
    pub ddc_mux_control_pin: u8,
    pub ddc_mux_state: u8,
// clock/data mux
    pub cd_valid: bool,
    pub cd_mux_type: u8,
    pub cd_mux_control_pin: u8,
    pub cd_mux_state: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_connector_audio {
    AMDGPU_AUDIO_DISABLE = 0,
    AMDGPU_AUDIO_ENABLE = 1,
    AMDGPU_AUDIO_AUTO = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum amdgpu_connector_dither {
    AMDGPU_FMT_DITHER_DISABLE = 0,
    AMDGPU_FMT_DITHER_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_dm_dp_aux {
    pub aux: drm_dp_aux,
    pub ddc_service: *mut ddc_service,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_i2c_adapter {
    pub base: i2c_adapter,
    pub ddc_service: *mut ddc_service,
    pub oem: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_connector {
    pub base: drm_connector,
    pub connector_id: u32,
    pub devices: u32,
    pub ddc_bus: *mut amdgpu_i2c_chan,
// some systems have an hdmi and vga port with a shared ddc line
    pub shared_ddc: bool,
    pub use_digital: bool,
// we need to mind the EDID between detect
    pub edid: *const drm_edid,
    pub con_priv: *mut c_void,
    pub dac_load_detect: bool,
    pub /: *mut *mut bool detected_by_load; / if the connection status was determined by load,
    pub /: *mut *mut bool detected_hpd_without_ddc; / if an HPD signal was detected on DVI, but ddc probing failed,
    pub connector_object_id: u16,
    pub hpd: amdgpu_hpd,
    pub router: amdgpu_router,
    pub router_bus: *mut amdgpu_i2c_chan,
    pub audio: amdgpu_connector_audio,
    pub dither: amdgpu_connector_dither,
    pub pixelclock_for_modeset: unsigned,
}

// TODO: start to use this struct and remove same field from base one
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amdgpu_mst_connector {
    pub base: amdgpu_connector,
    pub mst_mgr: drm_dp_mst_topology_mgr,
    pub dm_dp_aux: amdgpu_dm_dp_aux,
    pub mst_output_port: *mut drm_dp_mst_port,
    pub mst_root: *mut amdgpu_connector,
    pub is_mst_connector: bool,
    pub mst_encoder: *mut amdgpu_encoder,
}

// Driver internal use only flags of amdgpu_display_get_crtc_scanoutpos()

extern "C" {
    pub fn amdgpu_link_encoder_connector(dev: *mut drm_device);
}
extern "C" {
    pub fn amdgpu_encoder_get_dp_bridge_encoder_id(encoder: *mut drm_encoder) -> u16;
}
extern "C" {
    pub fn amdgpu_encoder_set_active_device(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn amdgpufb_remove(dev: *mut drm_device, fb: *mut drm_framebuffer) -> c_int;
}
extern "C" {
    pub fn amdgpu_enc_destroy(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn amdgpu_copy_fb(dev: *mut drm_device, dst_obj: *mut drm_gem_object);
}
extern "C" {
    pub fn amdgpu_display_crtc_idx_to_irq_type(adev: *mut amdgpu_device, crtc: c_int) -> c_int;
}
// amdgpu_display.c
extern "C" {
    pub fn amdgpu_display_print_display_setup(dev: *mut drm_device);
}
extern "C" {
    pub fn amdgpu_display_modeset_create_props(adev: *mut amdgpu_device) -> c_int;
}
