//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cec-funcs.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause)
//
// cec - HDMI Consumer Electronics Control message functions
//
// Copyright 2016 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

// One Touch Play Feature
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// Routing Control Feature
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// orig_phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// new_phys_addr = (msg->msg[4] << 8) | msg->msg[5];
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// Standby Feature
// One Touch Record Feature
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_arib_data {
    pub transport_id: __u16,
    pub service_id: __u16,
    pub orig_network_id: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_atsc_data {
    pub transport_id: __u16,
    pub program_number: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_dvb_data {
    pub transport_id: __u16,
    pub service_id: __u16,
    pub orig_network_id: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_channel_data {
    pub channel_number_fmt: __u8,
    pub major: __u16,
    pub minor: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_digital_service_id {
    pub service_id_method: __u8,
    pub dig_bcast_system: __u8,
    pub arib: cec_op_arib_data,
    pub atsc: cec_op_atsc_data,
    pub dvb: cec_op_dvb_data,
    pub channel: cec_op_channel_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_record_src {
    pub type: __u8,
    pub digital: cec_op_digital_service_id,
    pub ana_bcast_type: __u8,
    pub ana_freq: __u16,
    pub bcast_system: __u8,
    pub analog: },
    pub plug: __u8,
    pub ext_plug: },
    pub phys_addr: __u16,
    pub ext_phys_addr: },
}

// msg++ = (digital->service_id_method << 7) | digital->dig_bcast_system;
// msg++ = (digital->channel.channel_number_fmt << 2) |
// msg++ = digital->channel.major & 0xff;
// msg++ = digital->channel.minor >> 8;
// msg++ = digital->channel.minor & 0xff;
// msg++ = 0;
// msg++ = digital->atsc.transport_id >> 8;
// msg++ = digital->atsc.transport_id & 0xff;
// msg++ = digital->atsc.program_number >> 8;
// msg++ = digital->atsc.program_number & 0xff;
// msg++ = 0;
// msg++ = digital->dvb.transport_id >> 8;
// msg++ = digital->dvb.transport_id & 0xff;
// msg++ = digital->dvb.service_id >> 8;
// msg++ = digital->dvb.service_id & 0xff;
// msg++ = digital->dvb.orig_network_id >> 8;
// msg++ = digital->dvb.orig_network_id & 0xff;
// rec_status = msg->msg[2];
// Timer Programming Feature
// timer_overlap_warning = msg->msg[2] >> 7;
// media_info = (msg->msg[2] >> 5) & 3;
// prog_info = msg->msg[2] & 0xf;
// prog_error = 0;
// prog_info = 0;
// prog_error = msg->msg[2] & 0xf;
// prog_info == CEC_OP_PROG_INFO_MIGHT_NOT_BE_ENOUGH_SPACE ||
// prog_error == CEC_OP_PROG_ERROR_DUPLICATE) {
// duration_hr = (msg->msg[3] >> 4) * 10 + (msg->msg[3] & 0xf);
// duration_min = (msg->msg[4] >> 4) * 10 + (msg->msg[4] & 0xf);
// duration_hr = *duration_min = 0;
// timer_cleared_status = msg->msg[2];
// Hours and minutes are in BCD format
// day = msg->msg[2];
// month = msg->msg[3];
// Hours and minutes are in BCD format
// start_hr = (msg->msg[4] >> 4) * 10 + (msg->msg[4] & 0xf);
// start_min = (msg->msg[5] >> 4) * 10 + (msg->msg[5] & 0xf);
// duration_hr = (msg->msg[6] >> 4) * 10 + (msg->msg[6] & 0xf);
// duration_min = (msg->msg[7] >> 4) * 10 + (msg->msg[7] & 0xf);
// recording_seq = msg->msg[8];
// ana_bcast_type = msg->msg[9];
// ana_freq = (msg->msg[10] << 8) | msg->msg[11];
// bcast_system = msg->msg[12];
// Hours and minutes are in BCD format
// day = msg->msg[2];
// month = msg->msg[3];
// Hours and minutes are in BCD format
// start_hr = (msg->msg[4] >> 4) * 10 + (msg->msg[4] & 0xf);
// start_min = (msg->msg[5] >> 4) * 10 + (msg->msg[5] & 0xf);
// duration_hr = (msg->msg[6] >> 4) * 10 + (msg->msg[6] & 0xf);
// duration_min = (msg->msg[7] >> 4) * 10 + (msg->msg[7] & 0xf);
// recording_seq = msg->msg[8];
// Hours and minutes are in BCD format
// day = msg->msg[2];
// month = msg->msg[3];
// Hours and minutes are in BCD format
// start_hr = (msg->msg[4] >> 4) * 10 + (msg->msg[4] & 0xf);
// start_min = (msg->msg[5] >> 4) * 10 + (msg->msg[5] & 0xf);
// duration_hr = (msg->msg[6] >> 4) * 10 + (msg->msg[6] & 0xf);
// duration_min = (msg->msg[7] >> 4) * 10 + (msg->msg[7] & 0xf);
// recording_seq = msg->msg[8];
// ext_src_spec = msg->msg[9];
// plug = msg->msg[10];
// phys_addr = (msg->msg[11] << 8) | msg->msg[12];
// Hours and minutes are in BCD format
// day = msg->msg[2];
// month = msg->msg[3];
// Hours and minutes are in BCD format
// start_hr = (msg->msg[4] >> 4) * 10 + (msg->msg[4] & 0xf);
// start_min = (msg->msg[5] >> 4) * 10 + (msg->msg[5] & 0xf);
// duration_hr = (msg->msg[6] >> 4) * 10 + (msg->msg[6] & 0xf);
// duration_min = (msg->msg[7] >> 4) * 10 + (msg->msg[7] & 0xf);
// recording_seq = msg->msg[8];
// ana_bcast_type = msg->msg[9];
// ana_freq = (msg->msg[10] << 8) | msg->msg[11];
// bcast_system = msg->msg[12];
// Hours and minutes are in BCD format
// day = msg->msg[2];
// month = msg->msg[3];
// Hours and minutes are in BCD format
// start_hr = (msg->msg[4] >> 4) * 10 + (msg->msg[4] & 0xf);
// start_min = (msg->msg[5] >> 4) * 10 + (msg->msg[5] & 0xf);
// duration_hr = (msg->msg[6] >> 4) * 10 + (msg->msg[6] & 0xf);
// duration_min = (msg->msg[7] >> 4) * 10 + (msg->msg[7] & 0xf);
// recording_seq = msg->msg[8];
// Hours and minutes are in BCD format
// day = msg->msg[2];
// month = msg->msg[3];
// Hours and minutes are in BCD format
// start_hr = (msg->msg[4] >> 4) * 10 + (msg->msg[4] & 0xf);
// start_min = (msg->msg[5] >> 4) * 10 + (msg->msg[5] & 0xf);
// duration_hr = (msg->msg[6] >> 4) * 10 + (msg->msg[6] & 0xf);
// duration_min = (msg->msg[7] >> 4) * 10 + (msg->msg[7] & 0xf);
// recording_seq = msg->msg[8];
// ext_src_spec = msg->msg[9];
// plug = msg->msg[10];
// phys_addr = (msg->msg[11] << 8) | msg->msg[12];
// System Information Feature
// cec_version = msg->msg[2];
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// prim_devtype = msg->msg[4];
//
// Assumes a single RC Profile byte and a single Device Features byte,
// i.e. no extended features are supported by this helper function.
//
// As of CEC 2.0 no extended features are defined, should those be added
// in the future, then this function needs to be adapted or a new function
// should be added.
//
// cec_version = msg->msg[2];
// all_device_types = msg->msg[3];
// rc_profile = p;
// dev_features = NULL;
// dev_features = p + 1;
// rc_profile = *dev_features = NULL;
// Deck Control Feature
// deck_control_mode = msg->msg[2];
// deck_info = msg->msg[2];
// status_req = msg->msg[2];
// play_mode = msg->msg[2];
// Tuner Control Feature
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_tuner_device_info {
    pub rec_flag: __u8,
    pub tuner_display_info: __u8,
    pub is_analog: __u8,
    pub digital: cec_op_digital_service_id,
    pub ana_bcast_type: __u8,
    pub ana_freq: __u16,
    pub bcast_system: __u8,
    pub analog: },
}

// status_req = msg->msg[2];
// ana_bcast_type = msg->msg[2];
// ana_freq = (msg->msg[3] << 8) | msg->msg[4];
// bcast_system = msg->msg[5];
// Vendor Specific Commands Feature
// vendor_id = (msg->msg[2] << 16) | (msg->msg[3] << 8) | msg->msg[4];
// size = msg->len - 2;
// size = 14;
// vendor_cmd = msg->msg + 2;
// size = msg->len - 5;
// size = 11;
// vendor_id = (msg->msg[2] << 16) | (msg->msg[3] << 8) | msg->msg[4];
// vendor_cmd = msg->msg + 5;
// size = msg->len - 2;
// size = 14;
// rc_code = msg->msg + 2;
// OSD Display Feature
// disp_ctl = msg->msg[2];
// Device OSD Transfer Feature
// Device Menu Control Feature
// menu_state = msg->msg[2];
// menu_req = msg->msg[2];
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cec_op_ui_command {
    pub ui_cmd: __u8,
    pub has_opt_arg: __u8,
    pub channel_identifier: cec_op_channel_data,
    pub ui_broadcast_type: __u8,
    pub ui_sound_presentation_control: __u8,
    pub play_mode: __u8,
    pub ui_function_media: __u8,
    pub ui_function_select_av_input: __u8,
    pub ui_function_select_audio_input: __u8,
}

// The optional operand is one byte for all these ui commands
// Remote Control Passthrough Feature
// Power Status Feature
// pwr_state = msg->msg[2];
// General Protocol Messages
// abort_msg = msg->msg[2];
// reason = msg->msg[3];
// This changes the current message into a feature abort message
// System Audio Control Feature
// aud_mute_status = msg->msg[2] >> 7;
// aud_vol_status = msg->msg[2] & 0x7f;
// sys_aud_status = msg->msg[2];
// phys_addr = 0xffff;
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// sys_aud_status = msg->msg[2];
// num_descriptors = (msg->len - 2) / 3;
// num_descriptors = 4;
// num_descriptors = msg->len - 2;
// num_descriptors = 4;
// audio_volume_level = msg->msg[2];
// Audio Rate Control Feature
// audio_rate = msg->msg[2];
// Audio Return Channel Control Feature
// Dynamic Audio Lipsync Feature
// Only for CEC 2.0 and up
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// video_latency = msg->msg[4];
// low_latency_mode = (msg->msg[5] >> 2) & 1;
// audio_out_compensated = msg->msg[5] & 3;
// audio_out_delay = msg->msg[6];
// audio_out_delay = 1;
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// Latency Indication Protocol Feature
// Only for CEC 2.0 and up
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// sqid = (msg->msg[2] << 24) | (msg->msg[3] << 16) |
// video_format = msg->msg[2];
// hdr_format = msg->msg[3];
// vrr_format = msg->msg[4];
// audio_format = msg->msg[5];
// audio_format_extension = msg->len > 6 ? msg->msg[6] : 0;
// video_latency = (msg->msg[2] << 8) | msg->msg[3];
// audio_latency = (msg->msg[4] << 8) | msg->msg[5];
// audio_format = msg->msg[2];
// audio_format_extension = msg->len > 3 ? msg->msg[3] : 0;
// audio_latency = (msg->msg[2] << 8) | msg->msg[3];
// video_format = msg->msg[2];
// hdr_format = msg->msg[3];
// vrr_format = msg->msg[4];
// video_latency = (msg->msg[2] << 8) | msg->msg[3];
// sqid = (msg->msg[2] << 24) | (msg->msg[3] << 16) |
// Capability Discovery and Control Feature
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// phys_addr1 = (msg->msg[5] << 8) | msg->msg[6];
// phys_addr2 = (msg->msg[7] << 8) | msg->msg[8];
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// target_phys_addr = (msg->msg[5] << 8) | msg->msg[6];
// hec_func_state = msg->msg[7] >> 6;
// host_func_state = (msg->msg[7] >> 4) & 3;
// enc_func_state = (msg->msg[7] >> 4) & 3;
// cdc_errcode = msg->msg[7] & 3;
// has_field = msg->len >= 10;
// hec_field = *has_field ? ((msg->msg[8] << 8) | msg->msg[9]) : 0;
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// phys_addr1 = (msg->msg[5] << 8) | msg->msg[6];
// phys_addr2 = (msg->msg[7] << 8) | msg->msg[8];
// hec_set_state = msg->msg[9];
// phys_addr3 = *phys_addr4 = *phys_addr5 = CEC_PHYS_ADDR_INVALID;
// phys_addr3 = (msg->msg[10] << 8) | msg->msg[11];
// phys_addr4 = (msg->msg[12] << 8) | msg->msg[13];
// phys_addr5 = (msg->msg[14] << 8) | msg->msg[15];
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// phys_addr1 = (msg->msg[5] << 8) | msg->msg[6];
// hec_set_state = msg->msg[7];
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// phys_addr1 = (msg->msg[5] << 8) | msg->msg[6];
// phys_addr2 = (msg->msg[7] << 8) | msg->msg[8];
// phys_addr3 = (msg->msg[9] << 8) | msg->msg[10];
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// input_port = msg->msg[5] >> 4;
// hpd_state = msg->msg[5] & 0xf;
// msg[2] and msg[3] (phys_addr) are filled in by the CEC framework
// phys_addr = (msg->msg[2] << 8) | msg->msg[3];
// hpd_state = msg->msg[5] >> 4;
// hpd_error = msg->msg[5] & 0xf;
