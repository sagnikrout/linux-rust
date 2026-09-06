//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/radeon/radeon_mode.h
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

pub const RADEON_MAX_HPD_PINS: c_int = 7;
pub const RADEON_MAX_CRTCS: c_int = 6;
pub const RADEON_MAX_AFMT_BLOCKS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_rmx_type {
    RMX_OFF,
    RMX_FULL,
    RMX_CENTER,
    RMX_ASPECT
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_tv_std {
    TV_STD_NTSC,
    TV_STD_PAL,
    TV_STD_PAL_M,
    TV_STD_PAL_60,
    TV_STD_NTSC_J,
    TV_STD_SCART_PAL,
    TV_STD_SECAM,
    TV_STD_PAL_CN,
    TV_STD_PAL_N,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_underscan_type {
    UNDERSCAN_OFF,
    UNDERSCAN_ON,
    UNDERSCAN_AUTO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_hpd_id {
    RADEON_HPD_1 = 0,
    RADEON_HPD_2,
    RADEON_HPD_3,
    RADEON_HPD_4,
    RADEON_HPD_5,
    RADEON_HPD_6,
    RADEON_HPD_NONE = 0xff,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_output_csc {
    RADEON_OUTPUT_CSC_BYPASS = 0,
    RADEON_OUTPUT_CSC_TVRGB = 1,
    RADEON_OUTPUT_CSC_YCBCR601 = 2,
    RADEON_OUTPUT_CSC_YCBCR709 = 3,
}

pub const RADEON_MAX_I2C_BUS: c_int = 16;
// radeon gpio-based i2c
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
pub struct radeon_i2c_bus_rec {
    pub valid: bool,
// id used by atom
    pub i2c_id: u8,
// id used by atom
    pub hpd: radeon_hpd_id,
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_tmds_pll {
    pub freq: u32,
    pub value: u32,
}

pub const RADEON_MAX_BIOS_CONNECTOR: c_int = 16;
// pll flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_pll {
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
pub struct radeon_i2c_chan {
    pub adapter: i2c_adapter,
    pub dev: *mut drm_device,
    pub bit: i2c_algo_bit_data,
    pub rec: radeon_i2c_bus_rec,
    pub aux: drm_dp_aux,
    pub has_aux: bool,
    pub mutex: mutex,
}

// mostly for macs, but really any system without connector tables
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_connector_table {
    CT_NONE = 0,
    CT_GENERIC,
    CT_IBOOK,
    CT_POWERBOOK_EXTERNAL,
    CT_POWERBOOK_INTERNAL,
    CT_POWERBOOK_VGA,
    CT_MINI_EXTERNAL,
    CT_MINI_INTERNAL,
    CT_IMAC_G5_ISIGHT,
    CT_EMAC,
    CT_RN50_POWER,
    CT_MAC_X800,
    CT_MAC_G5_9600,
    CT_SAM440EP,
    CT_MAC_G4_SILVER
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_dvo_chip {
    DVO_SIL164,
    DVO_SIL1178,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_afmt {
    pub enabled: bool,
    pub offset: c_int,
    pub last_buffer_filled_status: bool,
    pub id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_mode_info {
    pub atom_context: *mut atom_context,
    pub atom_card_info: *mut card_info,
    pub connector_table: radeon_connector_table,
    pub mode_config_initialized: bool,
    pub crtcs: [*mut radeon_crtc; RADEON_MAX_CRTCS],
    pub afmt: [*mut radeon_afmt; RADEON_MAX_AFMT_BLOCKS],
// DVI-I properties
    pub coherent_mode_property: *mut drm_property,
// DAC enable load detect
    pub load_detect_property: *mut drm_property,
// TV standard
    pub tv_std_property: *mut drm_property,
// legacy TMDS PLL detect
    pub tmds_pll_property: *mut drm_property,
// underscan
    pub underscan_property: *mut drm_property,
    pub underscan_hborder_property: *mut drm_property,
    pub underscan_vborder_property: *mut drm_property,
// audio
    pub audio_property: *mut drm_property,
// FMT dithering
    pub dither_property: *mut drm_property,
// Output CSC
    pub output_csc_property: *mut drm_property,
// hardcoded DFP edid from BIOS
    pub bios_hardcoded_edid: *const drm_edid,
// firmware flags
    pub firmware_flags: u16,
// pointer to backlight encoder
    pub bl_encoder: *mut radeon_encoder,
// bitmask for active encoder frontends
    pub active_encoders: u32,
}

pub const RADEON_MAX_BL_LEVEL: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_backlight_privdata {
    pub encoder: *mut radeon_encoder,
    pub negative: u8,
}

pub const MAX_H_CODE_TIMING_LEN: c_int = 32;
pub const MAX_V_CODE_TIMING_LEN: c_int = 32;
// need to store these as reading
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_tv_regs {
    pub tv_uv_adr: u32,
    pub timing_cntl: u32,
    pub hrestart: u32,
    pub vrestart: u32,
    pub frestart: u32,
    pub h_code_timing: [u16; MAX_H_CODE_TIMING_LEN],
    pub v_code_timing: [u16; MAX_V_CODE_TIMING_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_atom_ss {
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_flip_status {
    RADEON_FLIP_NONE,
    RADEON_FLIP_PENDING,
    RADEON_FLIP_SUBMITTED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_crtc {
    pub base: drm_crtc,
    pub crtc_id: c_int,
    pub enabled: bool,
    pub can_tile: bool,
    pub cursor_out_of_bounds: bool,
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
    pub legacy_display_base_addr: u32,
    pub rmx_type: radeon_rmx_type,
    pub h_border: u8,
    pub v_border: u8,
    pub vsc: fixed20_12,
    pub hsc: fixed20_12,
    pub native_mode: drm_display_mode,
    pub pll_id: c_int,
// page flipping
    pub flip_queue: *mut workqueue_struct,
    pub flip_work: *mut radeon_flip_work,
    pub flip_status: radeon_flip_status,
// pll sharing
    pub ss: radeon_atom_ss,
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
    pub wm_low: u32,
    pub wm_high: u32,
    pub lb_vblank_lead_lines: u32,
    pub hw_mode: drm_display_mode,
    pub output_csc: radeon_output_csc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder_primary_dac {
// legacy primary dac
    pub ps2_pdac_adj: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder_lvds {
// legacy lvds
    pub panel_vcc_delay: u16,
    pub panel_pwr_delay: u8,
    pub panel_digon_delay: u8,
    pub panel_blon_delay: u8,
    pub panel_ref_divider: u16,
    pub panel_post_divider: u8,
    pub panel_fb_divider: u16,
    pub use_bios_dividers: bool,
    pub lvds_gen_cntl: u32,
// panel mode
    pub native_mode: drm_display_mode,
    pub bl_dev: *mut backlight_device,
    pub dpms_mode: c_int,
    pub backlight_level: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder_tv_dac {
// legacy tv dac
    pub ps2_tvdac_adj: u32,
    pub ntsc_tvdac_adj: u32,
    pub pal_tvdac_adj: u32,
    pub h_pos: c_int,
    pub v_pos: c_int,
    pub h_size: c_int,
    pub supported_tv_stds: c_int,
    pub tv_on: bool,
    pub tv_std: radeon_tv_std,
    pub tv: radeon_tv_regs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder_int_tmds {
// legacy int tmds
    pub tmds_pll: [radeon_tmds_pll; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder_ext_tmds {
// tmds over dvo
    pub i2c_bus: *mut radeon_i2c_chan,
    pub slave_addr: u8,
    pub dvo_chip: radeon_dvo_chip,
}

// spread spectrum
#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder_atom_dig {
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
    pub afmt: *mut radeon_afmt,
    pub pin: *mut r600_audio_pin,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder_atom_dac {
    pub tv_std: radeon_tv_std,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_encoder {
    pub base: drm_encoder,
    pub encoder_enum: u32,
    pub encoder_id: u32,
    pub devices: u32,
    pub active_device: u32,
    pub flags: u32,
    pub pixel_clock: u32,
    pub rmx_type: radeon_rmx_type,
    pub underscan_type: radeon_underscan_type,
    pub underscan_hborder: u32,
    pub underscan_vborder: u32,
    pub native_mode: drm_display_mode,
    pub enc_priv: *mut c_void,
    pub audio_polling_active: c_int,
    pub is_ext_encoder: bool,
    pub caps: u16,
    pub audio: *mut radeon_audio_funcs,
    pub output_csc: radeon_output_csc,
    pub can_mst: bool,
    pub offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_connector_atom_dig {
    pub igp_lane_info: u32,
// displayport
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub dp_sink_type: u8,
    pub dp_clock: c_int,
    pub dp_lane_count: c_int,
    pub edp_on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_gpio_rec {
    pub valid: bool,
    pub id: u8,
    pub reg: u32,
    pub mask: u32,
    pub shift: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_hpd {
    pub hpd: radeon_hpd_id,
    pub plugged_state: u8,
    pub gpio: radeon_gpio_rec,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_router {
    pub router_id: u32,
    pub i2c_info: radeon_i2c_bus_rec,
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
pub enum radeon_connector_audio {
    RADEON_AUDIO_DISABLE = 0,
    RADEON_AUDIO_ENABLE = 1,
    RADEON_AUDIO_AUTO = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum radeon_connector_dither {
    RADEON_FMT_DITHER_DISABLE = 0,
    RADEON_FMT_DITHER_ENABLE = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct radeon_connector {
    pub base: drm_connector,
    pub connector_id: u32,
    pub devices: u32,
    pub ddc_bus: *mut radeon_i2c_chan,
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
    pub hpd: radeon_hpd,
    pub router: radeon_router,
    pub router_bus: *mut radeon_i2c_chan,
    pub audio: radeon_connector_audio,
    pub dither: radeon_connector_dither,
    pub pixelclock_for_modeset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_clock_dividers {
    pub post_div: u32,

    pub 6: u32 reserved :,
    pub 12: u32 whole_fb_div :,
    pub 14: u32 frac_fb_div :,

    pub 14: u32 frac_fb_div :,
    pub 12: u32 whole_fb_div :,
    pub 6: u32 reserved :,

}

// added for CI
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mpll_param {

    pub 8: u32 reserved :,
    pub 12: u32 clkfrac :,
    pub 12: u32 clkf :,

    pub 12: u32 clkf :,
    pub 12: u32 clkfrac :,
    pub 8: u32 reserved :,

}

pub const MEM_TYPE_GDDR5: c_uint = 0x50;
pub const MEM_TYPE_GDDR4: c_uint = 0x40;
pub const MEM_TYPE_GDDR3: c_uint = 0x30;
pub const MEM_TYPE_DDR2: c_uint = 0x20;
pub const MEM_TYPE_GDDR1: c_uint = 0x10;
pub const MEM_TYPE_DDR3: c_uint = 0xb0;
pub const MEM_TYPE_MASK: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_memory_info {
    pub mem_vendor: u8,
    pub mem_type: u8,
}

pub const MAX_AC_TIMING_ENTRIES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_memory_clock_range_table {
    pub num_entries: u8,
    pub rsv: [u8; 3],
    pub mclk: [u32; MAX_AC_TIMING_ENTRIES],
}

pub const VBIOS_MC_REGISTER_ARRAY_SIZE: c_int = 32;
pub const VBIOS_MAX_AC_TIMING_ENTRIES: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mc_reg_entry {
    pub mclk_max: u32,
    pub mc_data: [u32; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mc_register_address {
    pub s1: u16,
    pub pre_reg_data: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_mc_reg_table {
    pub last: u8,
    pub num_entries: u8,
    pub mc_reg_table_entry: [atom_mc_reg_entry; VBIOS_MAX_AC_TIMING_ENTRIES],
    pub mc_reg_address: [atom_mc_register_address; VBIOS_MC_REGISTER_ARRAY_SIZE],
}

pub const MAX_VOLTAGE_ENTRIES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_voltage_table_entry {
    pub value: u16,
    pub smio_low: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct atom_voltage_table {
    pub count: u32,
    pub mask_low: u32,
    pub phase_delay: u32,
    pub entries: [atom_voltage_table_entry; MAX_VOLTAGE_ENTRIES],
}

// Driver internal use only flags of radeon_get_crtc_scanoutpos()

extern "C" {
    pub fn radeon_link_encoder_connector(dev: *mut drm_device);
}
extern "C" {
    pub fn radeon_encoder_get_dp_bridge_encoder_id(encoder: *mut drm_encoder) -> u16;
}
extern "C" {
    pub fn radeon_connector_encoder_get_dp_bridge_encoder_id(connector: *mut drm_connector) -> u16;
}
extern "C" {
    pub fn radeon_connector_is_dp12_capable(connector: *mut drm_connector) -> bool;
}
extern "C" {
    pub fn radeon_get_monitor_bpc(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn radeon_connector_hotplug(connector: *mut drm_connector);
}
extern "C" {
    pub fn radeon_dp_needs_link_train(radeon_connector: *mut radeon_connector) -> bool;
}
extern "C" {
    pub fn radeon_dp_getsinktype(radeon_connector: *mut radeon_connector) -> u8;
}
extern "C" {
    pub fn radeon_dp_getdpcd(radeon_connector: *mut radeon_connector) -> bool;
}
extern "C" {
    pub fn radeon_dp_aux_init(radeon_connector: *mut radeon_connector);
}
extern "C" {
    pub fn atombios_dig_encoder_setup(encoder: *mut drm_encoder, action: c_int, panel_mode: c_int);
}
extern "C" {
    pub fn atombios_dig_encoder_setup2(encoder: *mut drm_encoder, action: c_int, panel_mode: c_int, enc_override: c_int);
}
extern "C" {
    pub fn radeon_atom_encoder_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_atom_disp_eng_pll_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_atom_ext_encoder_setup_ddc(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn radeon_atom_copy_swap(dst: *mut u8, src: *mut u8, num_bytes: u8, to_le: bool);
}
extern "C" {
    pub fn radeon_i2c_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_i2c_fini(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_combios_i2c_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_atombios_i2c_init(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_i2c_destroy(i2c: *mut radeon_i2c_chan);
}
extern "C" {
    pub fn radeon_router_select_ddc_port(radeon_connector: *mut radeon_connector);
}
extern "C" {
    pub fn radeon_router_select_cd_port(radeon_connector: *mut radeon_connector);
}
extern "C" {
    pub fn radeon_ddc_probe(radeon_connector: *mut radeon_connector, use_aux: bool) -> bool;
}
extern "C" {
    pub fn radeon_setup_encoder_clones(dev: *mut drm_device);
}
extern "C" {
    pub fn atombios_dvo_setup(encoder: *mut drm_encoder, action: c_int);
}
extern "C" {
    pub fn atombios_digital_setup(encoder: *mut drm_encoder, action: c_int);
}
extern "C" {
    pub fn atombios_get_encoder_mode(encoder: *mut drm_encoder) -> c_int;
}
extern "C" {
    pub fn atombios_set_edp_panel_power(connector: *mut drm_connector, action: c_int) -> bool;
}
extern "C" {
    pub fn radeon_encoder_set_active_device(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn radeon_encoder_is_digital(encoder: *mut drm_encoder) -> bool;
}
extern "C" {
    pub fn radeon_crtc_load_lut(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn atombios_crtc_dpms(crtc: *mut drm_crtc, mode: c_int);
}
extern "C" {
    pub fn radeon_cursor_reset(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn radeon_combios_check_hardcoded_edid(rdev: *mut radeon_device) -> bool;
}
extern "C" {
    pub fn radeon_atom_get_clock_info(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn radeon_combios_get_clock_info(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn radeon_combios_external_tmds_setup(encoder: *mut drm_encoder) -> bool;
}
extern "C" {
    pub fn radeon_external_tmds_setup(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn radeon_combios_output_lock(encoder: *mut drm_encoder, lock: bool);
}
extern "C" {
    pub fn radeon_combios_initialize_bios_scratch_regs(dev: *mut drm_device);
}
extern "C" {
    pub fn radeon_atom_output_lock(encoder: *mut drm_encoder, lock: bool);
}
extern "C" {
    pub fn radeon_atom_initialize_bios_scratch_regs(dev: *mut drm_device);
}
extern "C" {
    pub fn radeon_save_bios_scratch_regs(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeon_restore_bios_scratch_regs(rdev: *mut radeon_device);
}
extern "C" {
    pub fn radeonfb_remove(dev: *mut drm_device, fb: *mut drm_framebuffer) -> c_int;
}
extern "C" {
    pub fn radeon_get_legacy_connector_info_from_bios(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn radeon_get_legacy_connector_info_from_table(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn radeon_get_clock_info(dev: *mut drm_device);
}
extern "C" {
    pub fn radeon_get_atom_connector_info_from_object_table(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn radeon_get_atom_connector_info_from_supported_devices_table(dev: *mut drm_device) -> bool;
}
extern "C" {
    pub fn radeon_enc_destroy(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn radeon_copy_fb(dev: *mut drm_device, dst_obj: *mut drm_gem_object);
}
extern "C" {
    pub fn radeon_combios_asic_init(dev: *mut drm_device);
}
extern "C" {
    pub fn atom_rv515_force_tv_scaler(rdev: *mut radeon_device, radeon_crtc: *mut radeon_crtc);
}
// legacy tv
// fmt blocks
extern "C" {
    pub fn avivo_program_fmt(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn dce3_program_fmt(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn dce4_program_fmt(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn dce8_program_fmt(encoder: *mut drm_encoder);
}
// fbdev layer

extern "C" {
    pub fn radeon_crtc_handle_vblank(rdev: *mut radeon_device, crtc_id: c_int);
}
extern "C" {
    pub fn radeon_crtc_handle_flip(rdev: *mut radeon_device, crtc_id: c_int);
}
extern "C" {
    pub fn radeon_atom_pick_dig_encoder(encoder: *mut drm_encoder, fe_idx: c_int) -> c_int;
}
extern "C" {
    pub fn radeon_atom_release_dig_encoder(rdev: *mut radeon_device, enc_idx: c_int);
}
