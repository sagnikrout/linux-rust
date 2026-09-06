//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/test-drivers/vivid/vivid-core.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// vivid-core.h - core datastructures
//
// Copyright 2014 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

// The maximum number of inputs
pub const MAX_INPUTS: c_int = 16;
// The maximum number of outputs
pub const MAX_OUTPUTS: c_int = 16;
// The maximum number of video capture buffers
pub const MAX_VID_CAP_BUFFERS: c_int = 64;
// The maximum up or down scaling factor is 4
pub const MAX_ZOOM: c_int = 4;
// The maximum image width/height are set to 4K DMT
pub const MAX_WIDTH: c_int = 4096;
pub const MAX_HEIGHT: c_int = 2160;
// The minimum image width/height
pub const MIN_WIDTH: c_int = 16;

// Pixel Array control divider

// The data_offset of plane 0 for the multiplanar formats
pub const PLANE0_DATA_OFFSET: c_int = 128;
// The supported TV frequency range in MHz

// The number of samples returned in every SDR buffer
pub const SDR_CAP_SAMPLES_PER_BUF: c_uint = 0x4000;
// used by the threads to know when to resync internal counters

//
// Maximum number of HDMI inputs allowed by vivid, due to limitations
// of the Physical Address in the EDID and used by CEC we stop at 15
// inputs and outputs.
//
pub const MAX_HDMI_INPUTS: c_int = 15;
pub const MAX_HDMI_OUTPUTS: c_int = 15;
// Maximum number of S-Video inputs allowed by vivid
pub const MAX_SVID_INPUTS: c_int = 16;
// The maximum number of items in a menu control

// Number of fixed menu items in the 'Connected To' menu controls
pub const FIXED_MENU_ITEMS: c_int = 2;
// The maximum number of vivid devices

//
// NULL-terminated string array for the HDMI 'Connected To' menu controls
// with the list of possible HDMI outputs.
//
// The first two items are fixed ("TPG" and "None").
//
// Menu control skip mask of all HDMI outputs that are in use
//
// Bitmask of which vivid instances need to update any connected
// HDMI outputs.
//
// Spinlock for access to hdmi_to_output_menu_skip_mask and
// hdmi_input_update_outputs_mask.
//
// Workqueue that updates the menu controls whenever the HDMI menu skip mask
// changes.
//
// The HDMI menu control value (index in the menu list) maps to an HDMI
// output that is part of the given vivid_dev instance and has the given
// output index (as returned by VIDIOC_G_OUTPUT).
//
// NULL/0 if not available.
//
// NULL-terminated string array for the S-Video 'Connected To' menu controls
// with the list of possible S-Video outputs.
//
// The first two items are fixed ("TPG" and "None").
//
// Menu control skip mask of all S-Video outputs that are in use
// Spinlock for access to svid_to_output_menu_skip_mask
//
// Workqueue that updates the menu controls whenever the S-Video menu skip mask
// changes.
//
// The S-Video menu control value (index in the menu list) maps to an S-Video
// output that is part of the given vivid_dev instance and has the given
// output index (as returned by VIDIOC_G_OUTPUT).
//
// NULL/0 if not available.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vivid_fmt {
    pub /: *mut *mut u32 fourcc; / v4l2 format id,
    pub color_enc: tgp_color_enc,
    pub can_do_overlay: bool,
    pub vdownsampling: [u8; TPG_MAX_PLANES],
    pub alpha_mask: u32,
    pub planes: u8,
    pub buffers: u8,
    pub data_offset: [u32; TPG_MAX_PLANES],
    pub bit_depth: [u32; TPG_MAX_PLANES],
}

// buffer for one video frame
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vivid_buffer {
// common v4l buffer stuff -- must be first
    pub vb: vb2_v4l2_buffer,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vivid_input {
    WEBCAM,
    TV,
    SVID,
    HDMI,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vivid_signal_mode {
    CURRENT_DV_TIMINGS,
    CURRENT_STD = CURRENT_DV_TIMINGS,
    NO_SIGNAL,
    NO_LOCK,
    OUT_OF_RANGE,
    SELECTED_DV_TIMINGS,
    SELECTED_STD = SELECTED_DV_TIMINGS,
    CYCLE_DV_TIMINGS,
    CYCLE_STD = CYCLE_DV_TIMINGS,
    CUSTOM_DV_TIMINGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vivid_colorspace {
    VIVID_CS_170M,
    VIVID_CS_709,
    VIVID_CS_SRGB,
    VIVID_CS_OPRGB,
    VIVID_CS_2020,
    VIVID_CS_DCI_P3,
    VIVID_CS_240M,
    VIVID_CS_SYS_M,
    VIVID_CS_SYS_BG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vivid_cec_xfer {
    pub adap: *mut cec_adapter,
    pub msg: [u8; CEC_MAX_MSG_SIZE],
    pub len: u32,
    pub sft: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vivid_dev {
    pub inst: u8,
    pub v4l2_dev: v4l2_device,

    pub mdev: media_device,
    pub vid_cap_pad: media_pad,
    pub vid_out_pad: media_pad,
    pub vbi_cap_pad: media_pad,
    pub vbi_out_pad: media_pad,
    pub sdr_cap_pad: media_pad,
    pub meta_cap_pad: media_pad,
    pub meta_out_pad: media_pad,
    pub touch_cap_pad: media_pad,

    pub ctrl_hdl_user_gen: v4l2_ctrl_handler,
    pub ctrl_hdl_user_vid: v4l2_ctrl_handler,
    pub ctrl_hdl_user_aud: v4l2_ctrl_handler,
    pub ctrl_hdl_streaming: v4l2_ctrl_handler,
    pub ctrl_hdl_sdtv_cap: v4l2_ctrl_handler,
    pub ctrl_hdl_loop_cap: v4l2_ctrl_handler,
    pub ctrl_hdl_fb: v4l2_ctrl_handler,
    pub vid_cap_dev: video_device,
    pub ctrl_hdl_vid_cap: v4l2_ctrl_handler,
    pub vid_out_dev: video_device,
    pub ctrl_hdl_vid_out: v4l2_ctrl_handler,
    pub vbi_cap_dev: video_device,
    pub ctrl_hdl_vbi_cap: v4l2_ctrl_handler,
    pub vbi_out_dev: video_device,
    pub ctrl_hdl_vbi_out: v4l2_ctrl_handler,
    pub radio_rx_dev: video_device,
    pub ctrl_hdl_radio_rx: v4l2_ctrl_handler,
    pub radio_tx_dev: video_device,
    pub ctrl_hdl_radio_tx: v4l2_ctrl_handler,
    pub sdr_cap_dev: video_device,
    pub ctrl_hdl_sdr_cap: v4l2_ctrl_handler,
    pub meta_cap_dev: video_device,
    pub ctrl_hdl_meta_cap: v4l2_ctrl_handler,
    pub meta_out_dev: video_device,
    pub ctrl_hdl_meta_out: v4l2_ctrl_handler,
    pub touch_cap_dev: video_device,
    pub ctrl_hdl_touch_cap: v4l2_ctrl_handler,
    pub slock: spinlock_t,
    pub mutex: mutex,
    pub update_hdmi_ctrl_work: work_struct,
    pub update_svid_ctrl_work: work_struct,
// capabilities
    pub vid_cap_caps: u32,
    pub vid_out_caps: u32,
    pub vbi_cap_caps: u32,
    pub vbi_out_caps: u32,
    pub sdr_cap_caps: u32,
    pub radio_rx_caps: u32,
    pub radio_tx_caps: u32,
    pub meta_cap_caps: u32,
    pub meta_out_caps: u32,
    pub touch_cap_caps: u32,
// supported features
    pub multiplanar: bool,
    pub num_inputs: u8,
    pub num_hdmi_inputs: u8,
    pub num_svid_inputs: u8,
    pub input_type: [u8; MAX_INPUTS],
    pub input_name_counter: [u8; MAX_INPUTS],
    pub num_outputs: u8,
    pub num_hdmi_outputs: u8,
    pub output_type: [u8; MAX_OUTPUTS],
    pub output_name_counter: [u8; MAX_OUTPUTS],
    pub has_audio_inputs: bool,
    pub has_audio_outputs: bool,
    pub has_vid_cap: bool,
    pub has_vid_out: bool,
    pub has_vbi_cap: bool,
    pub has_raw_vbi_cap: bool,
    pub has_sliced_vbi_cap: bool,
    pub has_vbi_out: bool,
    pub has_raw_vbi_out: bool,
    pub has_sliced_vbi_out: bool,
    pub has_radio_rx: bool,
    pub has_radio_tx: bool,
    pub has_sdr_cap: bool,
    pub has_fb: bool,
    pub has_meta_cap: bool,
    pub has_meta_out: bool,
    pub has_tv_tuner: bool,
    pub has_touch_cap: bool,
// Output index (0-MAX_OUTPUTS) to vivid instance of connected input
    pub output_to_input_instance: [*mut vivid_dev; MAX_OUTPUTS],
// Output index (0-MAX_OUTPUTS) to input index (0-MAX_INPUTS) of connected input
    pub output_to_input_index: [u8; MAX_OUTPUTS],
// Output index (0-MAX_OUTPUTS) to HDMI or S-Video output index (0-MAX_HDMI/SVID_OUTPUTS)
    pub output_to_iface_index: [u8; MAX_OUTPUTS],
// ctrl_hdmi_to_output or ctrl_svid_to_output control value for each input
    pub input_is_connected_to_output: [i32; MAX_INPUTS],
// HDMI index (0-MAX_HDMI_OUTPUTS) to output index (0-MAX_OUTPUTS)
    pub hdmi_index_to_output_index: [u8; MAX_HDMI_OUTPUTS],
// HDMI index (0-MAX_HDMI_INPUTS) to input index (0-MAX_INPUTS)
    pub hdmi_index_to_input_index: [u8; MAX_HDMI_INPUTS],
// S-Video index (0-MAX_SVID_INPUTS) to input index (0-MAX_INPUTS)
    pub svid_index_to_input_index: [u8; MAX_SVID_INPUTS],
// controls
    pub brightness: *mut v4l2_ctrl,
    pub contrast: *mut v4l2_ctrl,
    pub saturation: *mut v4l2_ctrl,
    pub hue: *mut v4l2_ctrl,
// autogain/gain cluster
    pub autogain: *mut v4l2_ctrl,
    pub gain: *mut v4l2_ctrl,
}

// std_signal_mode/standard cluster
// dv_timings_signal_mode/timings cluster
// Framebuffer

// Error injection
// Input
// Output
// Output Overlay
// video capture
// thread for generating video capture stream
// Touch capture
// video output
// video loop precalculated rectangles
//
// Intersection between what the output side composes and the capture side
// crops. I.e., what actually needs to be copied from the output buffer to
// the capture buffer.
//
// The part of the output buffer that (after scaling) corresponds to loop_vid_copy.
// The part of the capture buffer that (after scaling) corresponds to loop_vid_copy.
//
// The intersection of the framebuffer, the overlay output window and
// loop_vid_copy. I.e., the part of the framebuffer that actually should be
// blended with the compose_out rectangle. This uses the framebuffer origin.
//
// The same as loop_fb_copy but with compose_out origin.
//
// The part of the capture buffer that (after scaling) corresponds
// to loop_vid_overlay.
//
// thread for generating video output stream
// SDR capture
// thread for generating SDR stream
// RDS generator
// Radio receiver
// Radio transmitter
// Shared between radio receiver and transmitter
// CEC
// CEC OSD String
extern "C" {
    pub fn vivid_is_tv_cap(vivid_is_svid_cap(dev: dev) ||) -> return;
}
