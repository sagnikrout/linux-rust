//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/include/grph_object_ctrl_defs.h
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
// Copyright 2012-15 Advanced Micro Devices, Inc.
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
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

//
// #####################################################
//
// These defines shared between asic_control/bios_parser and other
// DAL components
//
// #####################################################
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum display_output_bit_depth {
    PANEL_UNDEFINE = 0,
    PANEL_6BIT_COLOR = 1,
    PANEL_8BIT_COLOR = 2,
    PANEL_10BIT_COLOR = 3,
    PANEL_12BIT_COLOR = 4,
    PANEL_16BIT_COLOR = 5,
}

// Device type as abstracted by ATOM BIOS
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dal_device_type {
    DEVICE_TYPE_UNKNOWN = 0,
    DEVICE_TYPE_LCD,
    DEVICE_TYPE_CRT,
    DEVICE_TYPE_DFP,
    DEVICE_TYPE_CV,
    DEVICE_TYPE_TV,
    DEVICE_TYPE_CF,
    DEVICE_TYPE_WIRELESS
}

// Device ID as abstracted by ATOM BIOS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_id {
    pub device_type:16: dal_device_type,
    pub /: *mut *mut uint32_t enum_id:16; / 1 based enum,
    pub raw_device_tag: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_i2c_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_info {
    pub clk_mask_register_index: u32,
    pub clk_en_register_index: u32,
    pub clk_y_register_index: u32,
    pub clk_a_register_index: u32,
    pub data_mask_register_index: u32,
    pub data_en_register_index: u32,
    pub data_y_register_index: u32,
    pub data_a_register_index: u32,
    pub clk_mask_shift: u32,
    pub clk_en_shift: u32,
    pub clk_y_shift: u32,
    pub clk_a_shift: u32,
    pub data_mask_shift: u32,
    pub data_en_shift: u32,
    pub data_y_shift: u32,
    pub data_a_shift: u32,
    pub gpio_info: },
    pub i2c_hw_assist: bool,
    pub i2c_line: u32,
    pub i2c_engine_id: u32,
    pub i2c_slave_address: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_hpd_info {
    pub hpd_int_gpio_uid: u8,
    pub hpd_active: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct connector_device_tag_info {
    pub acpi_device: u32,
    pub dev_id: device_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_timing {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct misc_info {
    pub HORIZONTAL_CUT_OFF:1: u32,
// 0=Active High, 1=Active Low
    pub H_SYNC_POLARITY:1: u32,
// 0=Active High, 1=Active Low
    pub V_SYNC_POLARITY:1: u32,
    pub VERTICAL_CUT_OFF:1: u32,
    pub H_REPLICATION_BY2:1: u32,
    pub V_REPLICATION_BY2:1: u32,
    pub COMPOSITE_SYNC:1: u32,
    pub INTERLACE:1: u32,
    pub DOUBLE_CLOCK:1: u32,
    pub RGB888:1: u32,
    pub GREY_LEVEL:2: u32,
    pub SPATIAL:1: u32,
    pub TEMPORAL:1: u32,
    pub API_ENABLED:1: u32,
    pub misc_info: },
    pub /: *mut *mut uint32_t pixel_clk; / in KHz,
    pub horizontal_addressable: u32,
    pub horizontal_blanking_time: u32,
    pub vertical_addressable: u32,
    pub vertical_blanking_time: u32,
    pub horizontal_sync_offset: u32,
    pub horizontal_sync_width: u32,
    pub vertical_sync_offset: u32,
    pub vertical_sync_width: u32,
    pub horizontal_border: u32,
    pub vertical_border: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct supported_refresh_rate {
    pub REFRESH_RATE_30HZ:1: u32,
    pub REFRESH_RATE_40HZ:1: u32,
    pub REFRESH_RATE_48HZ:1: u32,
    pub REFRESH_RATE_50HZ:1: u32,
    pub REFRESH_RATE_60HZ:1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct embedded_panel_info {
    pub lcd_timing: device_timing,
    pub ss_id: u32,
    pub supported_rr: supported_refresh_rate,
    pub drr_enabled: u32,
    pub min_drr_refresh_rate: u32,
    pub realtek_eDPToLVDS: bool,
    pub panel_width_mm: u16,
    pub panel_height_mm: u16,
    pub fake_edid_size: u16,
    pub fake_edid: *const u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_firmware_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pll_info {
    pub /: *mut *mut uint32_t crystal_frequency; / in KHz,
    pub /: *mut *mut uint32_t min_input_pxl_clk_pll_frequency; / in KHz,
    pub /: *mut *mut uint32_t max_input_pxl_clk_pll_frequency; / in KHz,
    pub /: *mut *mut uint32_t min_output_pxl_clk_pll_frequency; / in KHz,
    pub /: *mut *mut uint32_t max_output_pxl_clk_pll_frequency; / in KHz,
    pub pll_info: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct firmware_feature {
    pub memory_clk_ss_percentage: u32,
    pub engine_clk_ss_percentage: u32,
    pub feature: },
    pub /: *mut *mut uint32_t max_pixel_clock; / in KHz,
    pub /: *mut *mut uint32_t default_display_engine_pll_frequency; / in KHz,
    pub /: *mut *mut uint32_t external_clock_source_frequency_for_dp; / in KHz,
    pub /: *mut *mut uint32_t smu_gpu_pll_output_freq; / in KHz,
    pub min_allowed_bl_level: u8,
    pub remote_display_config: u8,
    pub /: *mut *mut uint32_t default_memory_clk; / in KHz,
    pub /: *mut *mut uint32_t default_engine_clk; / in KHz,
    pub /: *mut *mut uint32_t dp_phy_ref_clk; / in KHz - DCE12 only,
    pub /: *mut *mut uint32_t i2c_engine_ref_clk; / in KHz - DCE12 only,
    pub oem_i2c_present: bool,
    pub oem_i2c_obj_id: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dc_vram_info {
    pub num_chans: c_uint,
    pub dram_channel_width_bytes: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct step_and_delay_info {
    pub step: u32,
    pub delay: u32,
    pub recommended_ref_div: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct spread_spectrum_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spread_spectrum_type {
    pub CENTER_MODE:1: bool,
    pub EXTERNAL:1: bool,
    pub STEP_AND_DELAY_INFO:1: bool,
    pub type: },
// in unit of 0.01% (spreadPercentageDivider = 100),
    pub /: *mut otherwise in 0.001% units (spreadPercentageDivider = 1000);,
    pub spread_spectrum_percentage: u32,
    pub /: *mut *mut uint32_t spread_percentage_divider; / 100 or 1000,
    pub (HZ)*/: *mut *mut uint32_t spread_spectrum_range; / modulation freq,
    pub step_and_delay_info: step_and_delay_info,
// For mem/engine/uvd, Clock Out frequence (VCO ),
    pub /: *mut *mut uint32_t target_clock_range; / in KHz,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct graphics_object_encoder_cap_info {
    pub dp_hbr2_cap:1: u32,
    pub dp_hbr2_validated:1: u32,
//
// TODO: added MST and HDMI 6G capable flags
//
    pub reserved:15: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct din_connector_info {
    pub gpio_id: u32,
    pub gpio_tv_active_state: bool,
}

// Invalid channel mapping
//
// DDI PHY channel mapping reflecting XBAR setting
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union ddi_channel_mapping {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mapping {
    pub /: *mut *mut uint8_t lane0:2; / Mapping for lane 0,
    pub /: *mut *mut uint8_t lane1:2; / Mapping for lane 1,
    pub /: *mut *mut uint8_t lane2:2; / Mapping for lane 2,
    pub /: *mut *mut uint8_t lane3:2; / Mapping for lane 3,
    pub mapping: },
    pub raw: u8,
}

//
// Transmitter output configuration description
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transmitter_configuration_info {
// DDI PHY ID for the transmitter
    pub transmitter_phy_id: transmitter,
// DDI PHY channel mapping reflecting crossbar setting
    pub output_channel_mapping: ddi_channel_mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct transmitter_configuration {
// Configuration for the primary transmitter
    pub primary_transmitter_config: transmitter_configuration_info,
// Secondary transmitter configuration for Dual-link DVI
    pub secondary_transmitter_config: transmitter_configuration_info,
}

// These size should be sufficient to store info coming from BIOS
pub const NUMBER_OF_UCHAR_FOR_GUID: c_int = 16;
pub const MAX_NUMBER_OF_EXT_DISPLAY_PATH: c_int = 7;
pub const NUMBER_OF_CSR_M3_ARB: c_int = 10;
pub const NUMBER_OF_AVAILABLE_SCLK: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_reg_info {
    pub i2c_reg_index: c_uchar,
    pub i2c_reg_val: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ext_hdmi_settings {
    pub slv_addr: c_uchar,
    pub reg_num: c_uchar,
    pub reg_settings: [i2c_reg_info; 9],
    pub reg_num_6g: c_uchar,
    pub reg_settings_6g: [i2c_reg_info; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct edp_info {
    pub edp_backlight_pwm_hz: u16,
    pub edp_ss_percentage: u16,
    pub edp_ss_rate_10hz: u16,
    pub edp_pwr_on_off_delay: u8,
    pub edp_pwr_on_vary_bl_to_blon: u8,
    pub edp_pwr_down_bloff_to_vary_bloff: u8,
    pub edp_panel_bpc: u8,
    pub edp_bootup_bl_level: u8,
}

// V6
#[repr(C)]
#[derive(Copy, Clone)]
pub struct integrated_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct display_connection_info {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct external_display_path {
// A bit vector to show what devices are supported
    pub device_tag: u32,
// 16bit device ACPI id.
    pub device_acpi_enum: u32,
// A physical connector for displays to plug in,
    pub device_connector_id: graphics_object_id,
// An index into external AUX/DDC channel LUT
    pub ext_aux_ddc_lut_index: u8,
// An index into external HPD pin LUT
    pub ext_hpd_pin_lut_index: u8,
// external encoder object id
    pub ext_encoder_obj_id: graphics_object_id,
// XBAR mapping of the PHY channels
    pub channel_mapping: ddi_channel_mapping,
    pub caps: c_ushort,
    pub path: [}; MAX_NUMBER_OF_EXT_DISPLAY_PATH],
    pub gu_id: [u8; NUMBER_OF_UCHAR_FOR_GUID],
    pub checksum: u8,
    pub fixdpvoltageswing: u8,
    pub /: *mut *mut } ext_disp_conn_info; / exiting long long time,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct available_s_clk_list {
// Maximum clock supported with specified voltage index
    pub /: *mut *mut uint32_t supported_s_clk; / in KHz,
// The Voltage Index indicated by FUSE for specified SCLK
    pub voltage_index: u32,
// The Voltage ID indicated by FUSE for specified SCLK
    pub voltage_id: u32,
    pub avail_s_clk: [}; NUMBER_OF_AVAILABLE_SCLK],
    pub memory_type: u8,
    pub ma_channel_number: u8,
    pub /: *mut *mut uint32_t boot_up_engine_clock; / in KHz,
    pub /: *mut *mut uint32_t dentist_vco_freq; / in KHz,
    pub /: *mut *mut uint32_t boot_up_uma_clock; / in KHz,
    pub boot_up_req_display_vector: u32,
    pub other_display_misc: u32,
    pub gpu_cap_info: u32,
    pub sb_mmio_base_addr: u32,
    pub system_config: u32,
    pub cpu_cap_info: u32,
    pub max_nb_voltage: u32,
    pub min_nb_voltage: u32,
    pub boot_up_nb_voltage: u32,
    pub ext_disp_conn_info_offset: u32,
    pub csr_m3_arb_cntl_default: [u32; NUMBER_OF_CSR_M3_ARB],
    pub csr_m3_arb_cntl_uvd: [u32; NUMBER_OF_CSR_M3_ARB],
    pub csr_m3_arb_cntl_fs3d: [u32; NUMBER_OF_CSR_M3_ARB],
    pub gmc_restore_reset_time: u32,
    pub minimum_n_clk: u32,
    pub idle_n_clk: u32,
    pub ddr_dll_power_up_time: u32,
    pub ddr_pll_power_up_time: u32,
// start for V6
    pub pcie_clk_ss_type: u32,
    pub lvds_ss_percentage: u32,
    pub lvds_sspread_rate_in_10hz: u32,
    pub hdmi_ss_percentage: u32,
    pub hdmi_sspread_rate_in_10hz: u32,
    pub dvi_ss_percentage: u32,
    pub dvi_sspread_rate_in_10_hz: u32,
    pub sclk_dpm_boost_margin: u32,
    pub sclk_dpm_throttle_margin: u32,
    pub sclk_dpm_tdp_limit_pg: u32,
    pub sclk_dpm_tdp_limit_boost: u32,
    pub boost_engine_clock: u32,
    pub boost_vid_2bit: u32,
    pub enable_boost: u32,
    pub gnb_tdp_limit: u32,
// Start from V7
    pub max_lvds_pclk_freq_in_single_link: u32,
    pub lvds_misc: u32,
    pub lvds_pwr_on_seq_dig_on_to_de_in_4ms: u32,
    pub lvds_pwr_on_seq_de_to_vary_bl_in_4ms: u32,
    pub lvds_pwr_off_seq_vary_bl_to_de_in4ms: u32,
    pub lvds_pwr_off_seq_de_to_dig_on_in4ms: u32,
    pub lvds_off_to_on_delay_in_4ms: u32,
    pub lvds_pwr_on_seq_vary_bl_to_blon_in_4ms: u32,
    pub lvds_pwr_off_seq_blon_to_vary_bl_in_4ms: u32,
    pub lvds_reserved1: u32,
    pub lvds_bit_depth_control_val: u32,
// Start from V9
    pub dp0_ext_hdmi_slv_addr: c_uchar,
    pub dp0_ext_hdmi_reg_num: c_uchar,
    pub dp0_ext_hdmi_reg_settings: [i2c_reg_info; 9],
    pub dp0_ext_hdmi_6g_reg_num: c_uchar,
    pub dp0_ext_hdmi_6g_reg_settings: [i2c_reg_info; 3],
    pub dp1_ext_hdmi_slv_addr: c_uchar,
    pub dp1_ext_hdmi_reg_num: c_uchar,
    pub dp1_ext_hdmi_reg_settings: [i2c_reg_info; 9],
    pub dp1_ext_hdmi_6g_reg_num: c_uchar,
    pub dp1_ext_hdmi_6g_reg_settings: [i2c_reg_info; 3],
    pub dp2_ext_hdmi_slv_addr: c_uchar,
    pub dp2_ext_hdmi_reg_num: c_uchar,
    pub dp2_ext_hdmi_reg_settings: [i2c_reg_info; 9],
    pub dp2_ext_hdmi_6g_reg_num: c_uchar,
    pub dp2_ext_hdmi_6g_reg_settings: [i2c_reg_info; 3],
    pub dp3_ext_hdmi_slv_addr: c_uchar,
    pub dp3_ext_hdmi_reg_num: c_uchar,
    pub dp3_ext_hdmi_reg_settings: [i2c_reg_info; 9],
    pub dp3_ext_hdmi_6g_reg_num: c_uchar,
    pub dp3_ext_hdmi_6g_reg_settings: [i2c_reg_info; 3],
// V11
    pub dp_ss_control: u32,
// V2.1
    pub edp1_info: edp_info,
    pub edp2_info: edp_info,
    pub gpuclk_ss_percentage: u32,
    pub gpuclk_ss_type: u32,
}

//
// DFS-bypass flag
//
// Copy of SYS_INFO_GPUCAPS__ENABLE_DFS_BYPASS from atombios.h
#[repr(C)]
#[derive(Copy, Clone)]
pub struct panel_backlight_boundaries {
    pub min_signal_level: u32,
    pub max_signal_level: u32,
}
