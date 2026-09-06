//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/link_encoder.h
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
// Copyright 2017 Advanced Micro Devices, Inc.
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
// link_encoder.h
//
// Created on: Oct 6, 2015
// Author: yonsun
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encoder_init_data {
    pub channel: channel_id,
    pub connector: graphics_object_id,
    pub hpd_gpio: *mut gpio,
    pub hpd_source: hpd_source_id,
// TODO: in DAL2, here was pointer to EventManagerInterface
    pub encoder: graphics_object_id,
    pub analog_encoder: graphics_object_id,
    pub analog_engine: engine_id,
    pub ctx: *mut dc_context,
    pub transmitter: transmitter,
    pub hpd_active_high: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encoder_feature_support {
    pub IS_HBR2_CAPABLE:1: u32,
    pub IS_HBR3_CAPABLE:1: u32,
    pub IS_TPS3_CAPABLE:1: u32,
    pub IS_TPS4_CAPABLE:1: u32,
    pub HDMI_6GB_EN:1: u32,
    pub IS_DP2_CAPABLE:1: u32,
    pub IS_UHBR10_CAPABLE:1: u32,
    pub IS_UHBR13_5_CAPABLE:1: u32,
    pub IS_UHBR20_CAPABLE:1: u32,
    pub DP_IS_USB_C:1: u32,
    pub IS_HDMI_FRL_CAPABLE:1: u32,
    pub IS_FRL_8G_CAPABLE:1: u32,
    pub IS_FRL_10G_CAPABLE:1: u32,
    pub IS_FRL_12G_CAPABLE:1: u32,
    pub IS_FRL_16G_CAPABLE:1: u32,
    pub IS_FRL_20G_CAPABLE:1: u32,
    pub IS_FRL_24G_CAPABLE:1: u32,
    pub bits: },
    pub raw: u32,
    pub flags: },
    pub max_hdmi_deep_color: dc_color_depth,
    pub max_hdmi_pixel_clock: c_uint,
    pub hdmi_ycbcr420_supported: bool,
    pub dp_ycbcr420_supported: bool,
    pub fec_supported: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum phy_source_select {
    PHY_SOURCE_DIG,
    PHY_SOURCE_HPO
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_encoder {
    pub funcs: *const link_encoder_funcs,
    pub aux_channel_offset: i32,
    pub ctx: *mut dc_context,
    pub id: graphics_object_id,
    pub analog_id: graphics_object_id,
    pub connector: graphics_object_id,
    pub output_signals: u32,
    pub preferred_engine: engine_id,
    pub analog_engine: engine_id,
    pub features: encoder_feature_support,
    pub transmitter: transmitter,
    pub hpd_gpio: *mut gpio,
    pub hpd_source: hpd_source_id,
    pub usbc_combo_phy: bool,
    pub txffe_state: u8,
    pub hpd_active_high: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_enc_state {
    pub dphy_fec_en: u32,
    pub dphy_fec_ready_shadow: u32,
    pub dphy_fec_active_status: u32,
    pub dp_link_training_complete: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_txffe {
    pub amplitude: [u32; 4],
    pub pre_emphasis: [u32; 4],
    pub post_emphasis: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum encoder_type_select {
    ENCODER_TYPE_DIG = 0,
    ENCODER_TYPE_HDMI_FRL = 1,
    ENCODER_TYPE_DP_128B132B = 2
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_encoder_funcs {
    pub s): *mut *mut link_encoder enc, link_enc_state,
    pub stream): *const *const link_encoder enc, dc_stream_state,
    pub enc): *mut *mut void (hw_init)(struct link_encoder,
    pub signal): signal_type,
    pub pixel_clock): u32,
    pub clock_source): clock_source_id,
    pub clock_source): clock_source_id,
    pub pixel_clock): u32,
    pub pixel_clock): u32,
    pub signal): signal_type,
    pub lane_settings[LANE_COUNT_DP_MAX]): dc_lane_settings,
    pub para): *const encoder_set_dp_phy_pattern_param,
    pub table): *const link_mst_stream_allocation_table,
    pub exit_link_training_required): bool,
    pub sdp_transmit_line_num_deadline): c_uint,
    pub connect): bool,
    pub enc): *mut *mut void (enable_hpd)(struct link_encoder,
    pub enc): *mut *mut void (disable_hpd)(struct link_encoder,
    pub enc): *mut *mut bool (is_dig_enabled)(struct link_encoder,
    pub enc): *mut *mut unsigned int (get_dig_frontend)(struct link_encoder,
    pub enc): *mut *mut void (destroy)(struct link_encoder,
    pub enable): bool,
    pub ready): bool,
    pub enc): *mut *mut bool (fec_is_active)(struct link_encoder,
    pub enc): *mut *mut bool (is_in_alt_mode) (struct link_encoder,
    pub link_settings): *mut dc_link_settings,
    pub enc): *mut link_encoder,
    pub invert): bool,
    pub hpo_inst): u32,
    pub frl_link_rate): hdmi_frl_link_rate,
    pub frl_link_rate): hdmi_frl_link_rate,
    pub enc): *mut link_encoder,
    pub enc): *mut link_encoder,
    pub link_settings): *const dc_hdmi_frl_link_settings,
    pub lane_settings): *mut frl_txffe,
    pub lane_settings): *mut frl_txffe,
    pub hpo_inst): u32,
    pub fec_rdy): u8,
    pub digmode): u8,
    pub enc): *mut *mut bool (get_hpd_state)(struct link_encoder,
    pub delay_on_disconnect_in_ms): *mut *mut *mut bool (program_hpd_filter)(struct link_encoder enc, int delay_on_connect_in_ms, int,
    pub ri_pj_sw_mode): bool,
}

//
// Used to track assignments of links (display endpoints) to link encoders.
//
// Entry in link_enc_assignments table in struct resource_context.
// Entries only marked valid once encoder assigned to a link and invalidated once unassigned.
// Uses engine ID as identifier since PHY ID not relevant for USB4 DPIA endpoint.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct link_enc_assignment {
    pub valid: bool,
    pub ep_id: display_endpoint_id,
    pub eng_id: engine_id,
    pub stream: *mut dc_stream_state,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_enc_cfg_mode {
    LINK_ENC_CFG_STEADY, /* Normal operation - use current_state. */
    LINK_ENC_CFG_TRANSIENT /* During commit state - use state to be committed. */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_link_encoder {
    pub funcs: *const hpo_frl_link_encoder_funcs,
    pub ctx: *mut dc_context,
    pub inst: c_int,
}

//
// @hpo_frl_link_enc_state - FRL data from the device
//
// This struct is used to store FRL information retrieved from the hardware.
// This is used as a parameter for the read_state function.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_link_enc_state {
//
// @link_enc_enabled: 0 means disable and 1 enabled.
//
    pub link_enc_enabled: u32,
//
// @link_active:
//
// If link training is enable this field should be set to 1.
//
    pub link_active: u32,
//
// @lane_count: FRL lane count.
//
    pub lane_count: u32,
}

//
// @hpo_frl_link_encoder_funcs - FRL encoder functions
//
// DC handles FRL as an encoder; each ASIC may have some peculiarities in
// setting FRL. Thus, this struct, adds all the necessary callbacks that each
// DCN version must implement.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_link_encoder_funcs {
//
// @setup_link_encoder:
//
// This function is responsible for setup the ASIC to use FRL, i.e., it
// contains a register configuration. This function implementation
// expects the enablement of the link clock, lane count configuration,
// any reset/cleanup, and, finally, the enablement of the link.
//
    pub lane_count): c_int,
//
// @disable_link_encoder:
//
// Disable the FRL link. Note that this function must do the reverse of
// the setup_link_encoder.
//
    pub enc): *mut *mut void (disable_link_encoder)(struct hpo_frl_link_encoder,
//
// @set_hdmi_training_pattern:
//
// Register level configuration for each lane.
//
    pub lane3_pattern): u32,
//
// @get_hdmi_training_pattern:
//
// Retrieve from the registers each of the lane pattern configurations.
//
    pub lane3_pattern): *mut u32,
//
// @enable_frl_phy_output:
//
// Based on the parameters, this function should fill out the
// bp_transmitter_control struct and use it to enable the FRL PHY link
// via VBIOS.
//
    pub frl_link_rate): hdmi_frl_link_rate,
//
// @enable_output:
//
// Enable FRL by sending the enable packet training.
//
    pub enc): *mut *mut void (enable_output)(struct hpo_frl_link_encoder,
//
// @read_state:
//
// Get the FRL information from registers and fill it out in the
// hpo_frl_link_enc_state struct.
//
    pub state): *mut hpo_frl_link_enc_state,
//
// @destroy:
//
// Destroy encoder object.
//
    pub enc): *mut *mut void (destroy)(struct hpo_frl_link_encoder,
    pub enc): *mut hpo_frl_link_encoder,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp2_link_mode {
    DP2_LINK_TRAINING_TPS1,
    DP2_LINK_TRAINING_TPS2,
    DP2_LINK_ACTIVE,
    DP2_TEST_PATTERN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp2_phy_tp_select {
    DP_DPHY_TP_SELECT_TPS1,
    DP_DPHY_TP_SELECT_TPS2,
    DP_DPHY_TP_SELECT_PRBS,
    DP_DPHY_TP_SELECT_CUSTOM,
    DP_DPHY_TP_SELECT_SQUARE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp2_phy_tp_prbs {
    DP_DPHY_TP_PRBS7,
    DP_DPHY_TP_PRBS9,
    DP_DPHY_TP_PRBS11,
    DP_DPHY_TP_PRBS15,
    DP_DPHY_TP_PRBS23,
    DP_DPHY_TP_PRBS31
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_link_enc_state {
    pub link_enc_enabled: u32,
    pub link_mode: u32,
    pub lane_count: u32,
    pub slot_count: [u32; 4],
    pub stream_src: [u32; 4],
    pub vc_rate_x: [u32; 4],
    pub vc_rate_y: [u32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_link_encoder {
    pub funcs: *const hpo_dp_link_encoder_funcs,
    pub ctx: *mut dc_context,
    pub inst: c_int,
    pub preferred_engine: engine_id,
    pub transmitter: transmitter,
    pub hpd_source: hpd_source_id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_link_encoder_funcs {
    pub hpd_source): hpd_source_id,
    pub signal): signal_type,
    pub num_lanes): dc_lane_count,
    pub enc): *mut hpo_dp_link_encoder,
    pub tp_params): *mut encoder_set_dp_phy_pattern_param,
    pub table): *const link_mst_stream_allocation_table,
    pub avg_time_slots_per_mtp): fixed31_32,
    pub enc): *mut hpo_dp_link_encoder,
    pub state): *mut hpo_dp_link_enc_state,
    pub ffe_preset): u8,
}
