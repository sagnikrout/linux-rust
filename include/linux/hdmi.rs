//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hdmi.h
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
// Copyright (C) 2012 Avionic Design GmbH
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sub license,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial portions
// of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
// THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_packet_type {
    HDMI_PACKET_TYPE_NULL = 0x00,
    HDMI_PACKET_TYPE_AUDIO_CLOCK_REGEN = 0x01,
    HDMI_PACKET_TYPE_AUDIO_SAMPLE = 0x02,
    HDMI_PACKET_TYPE_GENERAL_CONTROL = 0x03,
    HDMI_PACKET_TYPE_ACP = 0x04,
    HDMI_PACKET_TYPE_ISRC1 = 0x05,
    HDMI_PACKET_TYPE_ISRC2 = 0x06,
    HDMI_PACKET_TYPE_ONE_BIT_AUDIO_SAMPLE = 0x07,
    HDMI_PACKET_TYPE_DST_AUDIO = 0x08,
    HDMI_PACKET_TYPE_HBR_AUDIO_STREAM = 0x09,
    HDMI_PACKET_TYPE_GAMUT_METADATA = 0x0a,
// + enum hdmi_infoframe_type
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_infoframe_type {
    HDMI_INFOFRAME_TYPE_VENDOR = 0x81,
    HDMI_INFOFRAME_TYPE_AVI = 0x82,
    HDMI_INFOFRAME_TYPE_SPD = 0x83,
    HDMI_INFOFRAME_TYPE_AUDIO = 0x84,
    HDMI_INFOFRAME_TYPE_DRM = 0x87,
}

// HDMI spec maximum TMDS character rates, in Hz
pub const HDMI_TMDS_CHAR_RATE_MIN_HZ: c_int = 25000000;
pub const HDMI_1_0_TMDS_CHAR_RATE_MAX_HZ: c_int = 165000000;
pub const HDMI_1_3_TMDS_CHAR_RATE_MAX_HZ: c_int = 340000000;
pub const HDMI_2_0_TMDS_CHAR_RATE_MAX_HZ: c_int = 600000000;
pub const HDMI_IEEE_OUI: c_uint = 0x000c03;
pub const HDMI_FORUM_IEEE_OUI: c_uint = 0xc45dd8;
pub const HDMI_INFOFRAME_HEADER_SIZE: c_int = 4;
pub const HDMI_AVI_INFOFRAME_SIZE: c_int = 13;
pub const HDMI_SPD_INFOFRAME_SIZE: c_int = 25;
pub const HDMI_AUDIO_INFOFRAME_SIZE: c_int = 10;
pub const HDMI_DRM_INFOFRAME_SIZE: c_int = 26;
pub const HDMI_VENDOR_INFOFRAME_SIZE: c_int = 4;
//
// HDMI 1.3a table 5-14 states that the largest InfoFrame_length is 27,
// not including the packet header or checksum byte. We include the
// checksum byte in HDMI_INFOFRAME_HEADER_SIZE, so this should allow
// HDMI_INFOFRAME_SIZE(MAX) to be the largest buffer we could ever need
// for any HDMI infoframe.
//
pub const HDMI_MAX_INFOFRAME_SIZE: c_int = 27;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_any_infoframe {
    pub type: hdmi_infoframe_type,
    pub version: c_uchar,
    pub length: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_colorspace {
    HDMI_COLORSPACE_RGB,
    HDMI_COLORSPACE_YUV422,
    HDMI_COLORSPACE_YUV444,
    HDMI_COLORSPACE_YUV420,
    HDMI_COLORSPACE_RESERVED4,
    HDMI_COLORSPACE_RESERVED5,
    HDMI_COLORSPACE_RESERVED6,
    HDMI_COLORSPACE_IDO_DEFINED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_scan_mode {
    HDMI_SCAN_MODE_NONE,
    HDMI_SCAN_MODE_OVERSCAN,
    HDMI_SCAN_MODE_UNDERSCAN,
    HDMI_SCAN_MODE_RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_colorimetry {
    HDMI_COLORIMETRY_NONE,
    HDMI_COLORIMETRY_ITU_601,
    HDMI_COLORIMETRY_ITU_709,
    HDMI_COLORIMETRY_EXTENDED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_picture_aspect {
    HDMI_PICTURE_ASPECT_NONE,
    HDMI_PICTURE_ASPECT_4_3,
    HDMI_PICTURE_ASPECT_16_9,
    HDMI_PICTURE_ASPECT_64_27,
    HDMI_PICTURE_ASPECT_256_135,
    HDMI_PICTURE_ASPECT_RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_active_aspect {
    HDMI_ACTIVE_ASPECT_16_9_TOP = 2,
    HDMI_ACTIVE_ASPECT_14_9_TOP = 3,
    HDMI_ACTIVE_ASPECT_16_9_CENTER = 4,
    HDMI_ACTIVE_ASPECT_PICTURE = 8,
    HDMI_ACTIVE_ASPECT_4_3 = 9,
    HDMI_ACTIVE_ASPECT_16_9 = 10,
    HDMI_ACTIVE_ASPECT_14_9 = 11,
    HDMI_ACTIVE_ASPECT_4_3_SP_14_9 = 13,
    HDMI_ACTIVE_ASPECT_16_9_SP_14_9 = 14,
    HDMI_ACTIVE_ASPECT_16_9_SP_4_3 = 15,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_extended_colorimetry {
    HDMI_EXTENDED_COLORIMETRY_XV_YCC_601,
    HDMI_EXTENDED_COLORIMETRY_XV_YCC_709,
    HDMI_EXTENDED_COLORIMETRY_S_YCC_601,
    HDMI_EXTENDED_COLORIMETRY_OPYCC_601,
    HDMI_EXTENDED_COLORIMETRY_OPRGB,

// The following EC values are only defined in CEA-861-F.
    HDMI_EXTENDED_COLORIMETRY_BT2020_CONST_LUM,
    HDMI_EXTENDED_COLORIMETRY_BT2020,
    HDMI_EXTENDED_COLORIMETRY_RESERVED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_quantization_range {
    HDMI_QUANTIZATION_RANGE_DEFAULT,
    HDMI_QUANTIZATION_RANGE_LIMITED,
    HDMI_QUANTIZATION_RANGE_FULL,
    HDMI_QUANTIZATION_RANGE_RESERVED,
}

// non-uniform picture scaling
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_nups {
    HDMI_NUPS_UNKNOWN,
    HDMI_NUPS_HORIZONTAL,
    HDMI_NUPS_VERTICAL,
    HDMI_NUPS_BOTH,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_ycc_quantization_range {
    HDMI_YCC_QUANTIZATION_RANGE_LIMITED,
    HDMI_YCC_QUANTIZATION_RANGE_FULL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_content_type {
    HDMI_CONTENT_TYPE_GRAPHICS,
    HDMI_CONTENT_TYPE_PHOTO,
    HDMI_CONTENT_TYPE_CINEMA,
    HDMI_CONTENT_TYPE_GAME,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_metadata_type {
    HDMI_STATIC_METADATA_TYPE1 = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_eotf {
    HDMI_EOTF_TRADITIONAL_GAMMA_SDR,
    HDMI_EOTF_TRADITIONAL_GAMMA_HDR,
    HDMI_EOTF_SMPTE_ST2084,
    HDMI_EOTF_BT_2100_HLG,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_avi_infoframe {
    pub type: hdmi_infoframe_type,
    pub version: c_uchar,
    pub length: c_uchar,
    pub itc: bool,
    pub pixel_repeat: c_uchar,
    pub colorspace: hdmi_colorspace,
    pub scan_mode: hdmi_scan_mode,
    pub colorimetry: hdmi_colorimetry,
    pub picture_aspect: hdmi_picture_aspect,
    pub active_aspect: hdmi_active_aspect,
    pub extended_colorimetry: hdmi_extended_colorimetry,
    pub quantization_range: hdmi_quantization_range,
    pub nups: hdmi_nups,
    pub video_code: c_uchar,
    pub ycc_quantization_range: hdmi_ycc_quantization_range,
    pub content_type: hdmi_content_type,
    pub top_bar: c_ushort,
    pub bottom_bar: c_ushort,
    pub left_bar: c_ushort,
    pub right_bar: c_ushort,
}

// DRM Infoframe as per CTA 861.G spec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_drm_infoframe {
    pub type: hdmi_infoframe_type,
    pub version: c_uchar,
    pub length: c_uchar,
    pub eotf: hdmi_eotf,
    pub metadata_type: hdmi_metadata_type,
    pub y: u16 x,,
    pub display_primaries: [}; 3],
    pub y: u16 x,,
    pub white_point: },
    pub max_display_mastering_luminance: u16,
    pub min_display_mastering_luminance: u16,
    pub max_cll: u16,
    pub max_fall: u16,
}

extern "C" {
    pub fn hdmi_avi_infoframe_init(frame: *mut hdmi_avi_infoframe);
}
extern "C" {
    pub fn hdmi_avi_infoframe_check(frame: *mut hdmi_avi_infoframe) -> c_int;
}
extern "C" {
    pub fn hdmi_drm_infoframe_init(frame: *mut hdmi_drm_infoframe) -> c_int;
}
extern "C" {
    pub fn hdmi_drm_infoframe_check(frame: *mut hdmi_drm_infoframe) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_spd_sdi {
    HDMI_SPD_SDI_UNKNOWN,
    HDMI_SPD_SDI_DSTB,
    HDMI_SPD_SDI_DVDP,
    HDMI_SPD_SDI_DVHS,
    HDMI_SPD_SDI_HDDVR,
    HDMI_SPD_SDI_DVC,
    HDMI_SPD_SDI_DSC,
    HDMI_SPD_SDI_VCD,
    HDMI_SPD_SDI_GAME,
    HDMI_SPD_SDI_PC,
    HDMI_SPD_SDI_BD,
    HDMI_SPD_SDI_SACD,
    HDMI_SPD_SDI_HDDVD,
    HDMI_SPD_SDI_PMP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_spd_infoframe {
    pub type: hdmi_infoframe_type,
    pub version: c_uchar,
    pub length: c_uchar,
    pub vendor: [c_char; 8],
    pub product: [c_char; 16],
    pub sdi: hdmi_spd_sdi,
}

extern "C" {
    pub fn hdmi_spd_infoframe_check(frame: *mut hdmi_spd_infoframe) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_coding_type {
    HDMI_AUDIO_CODING_TYPE_STREAM,
    HDMI_AUDIO_CODING_TYPE_PCM,
    HDMI_AUDIO_CODING_TYPE_AC3,
    HDMI_AUDIO_CODING_TYPE_MPEG1,
    HDMI_AUDIO_CODING_TYPE_MP3,
    HDMI_AUDIO_CODING_TYPE_MPEG2,
    HDMI_AUDIO_CODING_TYPE_AAC_LC,
    HDMI_AUDIO_CODING_TYPE_DTS,
    HDMI_AUDIO_CODING_TYPE_ATRAC,
    HDMI_AUDIO_CODING_TYPE_DSD,
    HDMI_AUDIO_CODING_TYPE_EAC3,
    HDMI_AUDIO_CODING_TYPE_DTS_HD,
    HDMI_AUDIO_CODING_TYPE_MLP,
    HDMI_AUDIO_CODING_TYPE_DST,
    HDMI_AUDIO_CODING_TYPE_WMA_PRO,
    HDMI_AUDIO_CODING_TYPE_CXT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_sample_size {
    HDMI_AUDIO_SAMPLE_SIZE_STREAM,
    HDMI_AUDIO_SAMPLE_SIZE_16,
    HDMI_AUDIO_SAMPLE_SIZE_20,
    HDMI_AUDIO_SAMPLE_SIZE_24,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_sample_frequency {
    HDMI_AUDIO_SAMPLE_FREQUENCY_STREAM,
    HDMI_AUDIO_SAMPLE_FREQUENCY_32000,
    HDMI_AUDIO_SAMPLE_FREQUENCY_44100,
    HDMI_AUDIO_SAMPLE_FREQUENCY_48000,
    HDMI_AUDIO_SAMPLE_FREQUENCY_88200,
    HDMI_AUDIO_SAMPLE_FREQUENCY_96000,
    HDMI_AUDIO_SAMPLE_FREQUENCY_176400,
    HDMI_AUDIO_SAMPLE_FREQUENCY_192000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_audio_coding_type_ext {
// Refer to Audio Coding Type (CT) field in Data Byte 1
    HDMI_AUDIO_CODING_TYPE_EXT_CT,

//
// The next three CXT values are defined in CEA-861-E only.
// They do not exist in older versions, and in CEA-861-F they are
// defined as 'Not in use'.
//
    HDMI_AUDIO_CODING_TYPE_EXT_HE_AAC,
    HDMI_AUDIO_CODING_TYPE_EXT_HE_AAC_V2,
    HDMI_AUDIO_CODING_TYPE_EXT_MPEG_SURROUND,

// The following CXT values are only defined in CEA-861-F.
    HDMI_AUDIO_CODING_TYPE_EXT_MPEG4_HE_AAC,
    HDMI_AUDIO_CODING_TYPE_EXT_MPEG4_HE_AAC_V2,
    HDMI_AUDIO_CODING_TYPE_EXT_MPEG4_AAC_LC,
    HDMI_AUDIO_CODING_TYPE_EXT_DRA,
    HDMI_AUDIO_CODING_TYPE_EXT_MPEG4_HE_AAC_SURROUND,
    HDMI_AUDIO_CODING_TYPE_EXT_MPEG4_AAC_LC_SURROUND = 10,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_audio_infoframe {
    pub type: hdmi_infoframe_type,
    pub version: c_uchar,
    pub length: c_uchar,
    pub channels: c_uchar,
    pub coding_type: hdmi_audio_coding_type,
    pub sample_size: hdmi_audio_sample_size,
    pub sample_frequency: hdmi_audio_sample_frequency,
    pub coding_type_ext: hdmi_audio_coding_type_ext,
    pub channel_allocation: c_uchar,
    pub level_shift_value: c_uchar,
    pub downmix_inhibit: bool,
}

extern "C" {
    pub fn hdmi_audio_infoframe_init(frame: *mut hdmi_audio_infoframe) -> c_int;
}
extern "C" {
    pub fn hdmi_audio_infoframe_check(frame: *const hdmi_audio_infoframe) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hdmi_3d_structure {
    HDMI_3D_STRUCTURE_INVALID = -1,
    HDMI_3D_STRUCTURE_FRAME_PACKING = 0,
    HDMI_3D_STRUCTURE_FIELD_ALTERNATIVE,
    HDMI_3D_STRUCTURE_LINE_ALTERNATIVE,
    HDMI_3D_STRUCTURE_SIDE_BY_SIDE_FULL,
    HDMI_3D_STRUCTURE_L_DEPTH,
    HDMI_3D_STRUCTURE_L_DEPTH_GFX_GFX_DEPTH,
    HDMI_3D_STRUCTURE_TOP_AND_BOTTOM,
    HDMI_3D_STRUCTURE_SIDE_BY_SIDE_HALF = 8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdmi_vendor_infoframe {
    pub type: hdmi_infoframe_type,
    pub version: c_uchar,
    pub length: c_uchar,
    pub oui: c_uint,
    pub vic: u8,
    pub s3d_struct: hdmi_3d_structure,
    pub s3d_ext_data: c_uint,
}

// HDR Metadata as per 861.G spec
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_static_metadata {
    pub eotf: __u8,
    pub metadata_type: __u8,
    pub max_cll: __u16,
    pub max_fall: __u16,
    pub min_cll: __u16,
}

//
// struct hdr_sink_metadata - HDR sink metadata
//
// Metadata Information read from Sink's EDID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_sink_metadata {
//
// @metadata_type: Static_Metadata_Descriptor_ID.
//
    pub metadata_type: __u32,
//
// @hdmi_type1: HDR Metadata Infoframe.
//
    pub hdmi_type1: hdr_static_metadata,
}

extern "C" {
    pub fn hdmi_vendor_infoframe_init(frame: *mut hdmi_vendor_infoframe) -> c_int;
}
extern "C" {
    pub fn hdmi_vendor_infoframe_check(frame: *mut hdmi_vendor_infoframe) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_vendor_any_infoframe {
    pub type: hdmi_infoframe_type,
    pub version: c_uchar,
    pub length: c_uchar,
    pub oui: c_uint,
    pub any: },
    pub hdmi: hdmi_vendor_infoframe,
}

//
// union hdmi_infoframe - overall union of all abstract infoframe representations
// @any: generic infoframe
// @avi: avi infoframe
// @spd: spd infoframe
// @vendor: union of all vendor infoframes
// @audio: audio infoframe
// @drm: Dynamic Range and Mastering infoframe
//
// This is used by the generic pack function. This works since all infoframes
// have the same header which also indicates which type of infoframe should be
// packed.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union hdmi_infoframe {
    pub any: hdmi_any_infoframe,
    pub avi: hdmi_avi_infoframe,
    pub spd: hdmi_spd_infoframe,
    pub vendor: hdmi_vendor_any_infoframe,
    pub audio: hdmi_audio_infoframe,
    pub drm: hdmi_drm_infoframe,
}
