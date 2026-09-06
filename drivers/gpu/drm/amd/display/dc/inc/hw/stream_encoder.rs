//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/inc/hw/stream_encoder.h
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
// stream_encoder.h
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_pixel_encoding_type {
    DP_PIXEL_ENCODING_TYPE_RGB444		= 0x00000000,
    DP_PIXEL_ENCODING_TYPE_YCBCR422		= 0x00000001,
    DP_PIXEL_ENCODING_TYPE_YCBCR444		= 0x00000002,
    DP_PIXEL_ENCODING_TYPE_RGB_WIDE_GAMUT	= 0x00000003,
    DP_PIXEL_ENCODING_TYPE_Y_ONLY		= 0x00000004,
    DP_PIXEL_ENCODING_TYPE_YCBCR420		= 0x00000005
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dp_component_depth {
    DP_COMPONENT_PIXEL_DEPTH_6BPC		= 0x00000000,
    DP_COMPONENT_PIXEL_DEPTH_8BPC		= 0x00000001,
    DP_COMPONENT_PIXEL_DEPTH_10BPC		= 0x00000002,
    DP_COMPONENT_PIXEL_DEPTH_12BPC		= 0x00000003,
    DP_COMPONENT_PIXEL_DEPTH_16BPC		= 0x00000004
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct audio_clock_info {
// pixel clock frequency
    pub pixel_clock_in_10khz: u32,
// N - 32KHz audio
    pub n_32khz: u32,
// CTS - 32KHz audio
    pub cts_32khz: u32,
    pub n_44khz: u32,
    pub cts_44khz: u32,
    pub n_48khz: u32,
    pub cts_48khz: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dynamic_metadata_mode {
    dmdata_dp,
    dmdata_hdmi,
    dmdata_dolby_vision
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct frl_audio_clock_info {
    pub frl_character_clock_kHz: u32,
    pub n_32khz: u32,
    pub cts_32khz: u32,
    pub n_44khz: u32,
    pub cts_44khz: u32,
    pub n_48khz: u32,
    pub cts_48khz: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_sdp_line_num {
// Adaptive Sync SDP
    pub adaptive_sync_line_num_valid: bool,
    pub adaptive_sync_line_num: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encoder_info_frame {
// auxiliary video information
    pub avi: dc_info_packet,
    pub gamut: dc_info_packet,
    pub vendor: dc_info_packet,
    pub hfvsif: dc_info_packet,
    pub vtem: dc_info_packet,
// source product description
    pub spd: dc_info_packet,
// video stream configuration
    pub vsc: dc_info_packet,
// HDR Static MetaData
    pub hdrsmd: dc_info_packet,
// Adaptive Sync SDP
    pub adaptive_sync: dc_info_packet,
    pub sdp_line_num: enc_sdp_line_num,
    pub firmware_controlled_hdr_info_packet: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encoder_unblank_param {
    pub link_settings: dc_link_settings,
    pub timing: dc_crtc_timing,
    pub opp_cnt: c_int,
    pub pix_per_cycle: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct encoder_set_dp_phy_pattern_param {
    pub dp_phy_pattern: dp_test_pattern,
    pub custom_pattern: *const u8,
    pub custom_pattern_size: u32,
    pub dp_panel_mode: dp_panel_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_encoder {
    pub funcs: *const stream_encoder_funcs,
    pub ctx: *mut dc_context,
    pub bp: *mut dc_bios,
    pub id: engine_id,
    pub stream_enc_inst: u32,
    pub vpg: *mut vpg,
    pub afmt: *mut afmt,
    pub apg: *mut apg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct enc_state {
    pub state.: uint32_t dsc_mode; // DISABLED 0; 1 or 2 indicate enabled,
    pub dsc_slice_width: u32,
    pub sec_gsp_pps_line_num: u32,
    pub vbid6_line_reference: u32,
    pub vbid6_line_num: u32,
    pub sec_gsp_pps_enable: u32,
    pub sec_stream_enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stream_encoder_funcs {
    pub enable_sdp_splitting): u32,
    pub enable_audio): bool,
    pub is_dual_link): bool,
    pub crtc_timing): *mut dc_crtc_timing,
    pub avg_time_slots_per_mtp): fixed31_32,
    pub info_frame): *const encoder_info_frame,
    pub enc): *mut stream_encoder,
    pub info_frame): *mut encoder_info_frame,
    pub info_frame): *const encoder_info_frame,
    pub sdp_message_size): c_uint,
    pub enc): *mut stream_encoder,
    pub enc): *mut stream_encoder,
    pub param): *const encoder_unblank_param,
    pub mute): *mut *mut stream_encoder enc, bool,
    pub info): *mut audio_info,
    pub enc): *mut stream_encoder,
    pub enc): *mut stream_encoder,
    pub audio_crtc_info): *mut audio_crtc_info,
    pub enc): *mut stream_encoder,
    pub enable): bool,
    pub enable): *mut *mut stream_encoder enc, bool,
    pub tg_inst): c_int,
    pub enable): bool,
    pub enc): *mut stream_encoder,
    pub enc): *mut stream_encoder,
    pub depth): *mut dc_color_depth,
    pub s): *mut *mut *mut void (enc_read_state)(struct stream_encoder enc, struct enc_state,
    pub dsc_slice_width): u32,
    pub immediate_update): bool,
    pub dmdata_mode): dynamic_metadata_mode,
//
// @dp_set_odm_combine: Sets up DP stream encoder for ODM.
//
    pub odm_combine): bool,
    pub enc): *mut stream_encoder,
    pub pix_per_container): *mut *mut stream_encoder enc, unsigned int,
    pub enc): *mut *mut void (enable_fifo)(struct stream_encoder,
    pub enc): *mut *mut void (disable_fifo)(struct stream_encoder,
    pub enc): *mut *mut bool (is_fifo_enabled)(struct stream_encoder,
    pub link_enc_inst): *mut *mut *mut void (map_stream_to_link)(struct stream_encoder enc, uint32_t stream_enc_inst, uint32_t,
    pub enc): *mut *mut uint32_t (get_pixels_per_cycle)(struct stream_encoder,
}

//
// @hpo_frl_stream_encoder_state - Stream encoder parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_stream_encoder_state {
    pub stream_enc_enabled: u32,
    pub otg_inst: u32,
    pub color_depth: u32,
    pub num_odm_segments: u32,
    pub h_active: u32,
    pub h_blank: u32,
    pub borrow_mode: u32,
    pub pixel_format: dc_pixel_encoding,
}

//
// @hpo_frl_stream_encoder - Encoder stream instance
//
// This struct keeps the reference to the struct with the FRL stream encoder
// callbacks. Additionally, it has other references that simplify the stream
// configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_stream_encoder {
//
// @funcs: callback functions for using FRL stream encoder.
//
    pub funcs: *const hpo_frl_stream_encoder_funcs,
//
// @stream_enc_inst: Instance ID reference.
//
    pub stream_enc_inst: u32,
//
// @ctx: DC context.
//
    pub ctx: *mut dc_context,
//
// @bp: Bios parser reference.
//
    pub bp: *mut dc_bios,
//
// @id: ID to the Engine object type.
//
    pub id: engine_id,
//
// @afmt:
//
// Audio Formatter (AFMT) reference used for select the correct audio
// reference.
//
    pub afmt: *mut afmt,
//
// @vpg:
//
// The VBI Packet Generator (VPG) reference which is used for
// generating the HDMI data island packet headers for ISRC1 and generic
// packages.
//
    pub vpg: *mut vpg,
//
// @afmt:
//
// Audio Pattern Generator (APG) reference used for select the correct audio
// reference.
//
    pub apg: *mut apg,
}

//
// @hpo_frl_stream_encoder_funcs - FRL stream encoder functions callbacks
//
// DC must set up the FRL encoder and the stream encoder; however, each ASIC
// may have some quirks in setting FRL. Thus, this struct, adds all the encoder
// stream interfaces for setup the FRL stream encoder.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_frl_stream_encoder_funcs {
    pub dsc_packed_pps): *mut u8,
//
// @hdmi_frl_enable:
//
// This callback initializes a new FRL stream by enabling HDMI Dispclk,
// SOCCLK, and HDMI stream clock. Additionally, this callback is also
// used to initialize some debug options, such as the CRC validation.
// Finally, it must setup the OTG instance.
//
    pub otg_inst): c_int,
//
// @hdmi_frl_unblank:
//
// Ensure that the FIFO video stream is active, reset the necessary
// FIFO registers, enable HDMI tribyte encoder, and finally, adjust the
// clock ramp registers for FIFO. Notice that all the configuration
// made by this function it is at the register level.
//
    pub otg_inst): c_int,
//
// @hdmi_frl_blank:
//
// This callback is a register-level configuration that must disable
// the clock ramp adjuster FIFO, disable the HDMI Tribyte encoder, and
// disable the stream clocks (dispclk, socclk, and hdmistreamclk).
//
    pub enc): *mut *mut void (hdmi_frl_blank)(struct hpo_frl_stream_encoder,
//
// @hdmi_frl_fifo_odm_enabled:
//
// This callback checks if the FIFO ODM Combine mode is enabled
//
    pub enc): *mut *mut bool (hdmi_frl_fifo_odm_enabled)(struct hpo_frl_stream_encoder,
//
// @hdmi_frl_set_stream_attribute:
//
// This callback should be invoked only after the link is trained. The
// implementation should configure the pixel encode, color depth, ODM
// mode, configure horizontal blank/active size, configure borrow
// parameters, enable general control packet, enable/setup audio, and
// AVMute.
//
    pub odm_combine_num_segments): c_int,
//
// @update_hdmi_info_packets:
//
// Update the HDMI packet control option.
//
    pub info_frame): *const encoder_info_frame,
//
// @stop_hdmi_info_packets:
//
// Update HDMI info packet (avi, vendoer, gamut, spd, hdrsmd, hfvsif,
// vtem, etc).
//
    pub enc): *mut *mut void (stop_hdmi_info_packets)(struct hpo_frl_stream_encoder,
//
// @audio_mute_control:
//
// Just mute the audio.
//
    pub mute): bool,
//
// @hdmi_audio_setup:
//
// Setup HDMI audio based on the Azila info.
//
    pub audio_crtc_info): *mut audio_crtc_info,
//
// @hdmi_audio_disable:
//
// Disable audio.
//
    pub enc): *mut *mut void (hdmi_audio_disable)(struct hpo_frl_stream_encoder,
//
// @set_avmute:
//
// Disable AVmute at the register-level.
//
    pub enable): *mut *mut *mut void (set_avmute)(struct hpo_frl_stream_encoder enc, bool,
//
// @validate_hdmi_frl_output:
//
// Validate FRL inputs, DSC, audio parameters, FRL capacity, and borrow
// parameters.
//
    pub dsc_max_rate): c_uint,
//
// @read_state:
//
// Fill out the hpo_frl_stream_encoder_state with the info retrieved
// from ASIC registers.
//
    pub state): *mut hpo_frl_stream_encoder_state,
//
// @set_dynamic_metadata:
//
// Metadata configuration that sets:
// - Using Enfine or disable DME.
// - HUBP setup for the physical instance that has the DME enabled.
// - Metadata packet type.
// - Ensure OTG master update locks in the changing DME configuration.
//
    pub dmdata_mode): dynamic_metadata_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_stream_encoder_state {
    pub stream_enc_enabled: u32,
    pub vid_stream_enabled: u32,
    pub otg_inst: u32,
    pub pixel_encoding: u32,
    pub component_depth: u32,
    pub compressed_format: u32,
    pub sdp_enabled: u32,
    pub mapped_to_link_enc: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_stream_encoder {
    pub funcs: *const hpo_dp_stream_encoder_funcs,
    pub ctx: *mut dc_context,
    pub bp: *mut dc_bios,
    pub inst: u32,
    pub id: engine_id,
    pub vpg: *mut vpg,
    pub apg: *mut apg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpo_dp_stream_encoder_funcs {
    pub enc): *mut hpo_dp_stream_encoder,
    pub stream_source): u32,
    pub enc): *mut hpo_dp_stream_encoder,
    pub enc): *mut hpo_dp_stream_encoder,
    pub double_buffer_en): bool,
    pub info_frame): *mut encoder_info_frame,
    pub info_frame): *const encoder_info_frame,
    pub enc): *mut hpo_dp_stream_encoder,
    pub immediate_update): bool,
    pub link_enc_inst): u32,
    pub info): *mut audio_info,
    pub enc): *mut hpo_dp_stream_encoder,
    pub enc): *mut hpo_dp_stream_encoder,
    pub state): *mut hpo_dp_stream_encoder_state,
    pub width): u16,
}
