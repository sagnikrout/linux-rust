//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/delta/delta-mjpeg-fw.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) STMicroelectronics SA 2015
// Author: Hugues Fruchet <hugues.fruchet@st.com> for STMicroelectronics.
//
// struct jpeg_decoded_buffer_address_t
//
// defines the addresses where the decoded picture/additional
// info related to the block structures will be stored
//
// @display_luma_p:		address of the luma buffer
// @display_chroma_p:		address of the chroma buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decoded_buffer_address_t {
    pub luma_p: u32,
    pub chroma_p: u32,
}

//
// struct jpeg_display_buffer_address_t
//
// defines the addresses (used by the Display Reconstruction block)
// where the pictures to be displayed will be stored
//
// @struct_size:		size of the structure in bytes
// @display_luma_p:		address of the luma buffer
// @display_chroma_p:		address of the chroma buffer
// @display_decimated_luma_p:	address of the decimated luma buffer
// @display_decimated_chroma_p:	address of the decimated chroma buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_display_buffer_address_t {
    pub struct_size: u32,
    pub display_luma_p: u32,
    pub display_chroma_p: u32,
    pub display_decimated_luma_p: u32,
    pub display_decimated_chroma_p: u32,
}

//
// used for enabling main/aux outputs for both display &
// reference reconstruction blocks
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jpeg_rcn_ref_disp_enable_t {
// enable decimated (for display) reconstruction
    JPEG_DISP_AUX_EN = 0x00000010,
// enable main (for display) reconstruction
    JPEG_DISP_MAIN_EN = 0x00000020,
// enable both main & decimated (for display) reconstruction
    JPEG_DISP_AUX_MAIN_EN = 0x00000030,
// enable only reference output(ex. for trick modes)
    JPEG_REF_MAIN_EN = 0x00000100,
//
// enable reference output with decimated
// (for display) reconstruction
//
    JPEG_REF_MAIN_DISP_AUX_EN = 0x00000110,
//
// enable reference output with main
// (for display) reconstruction
//
    JPEG_REF_MAIN_DISP_MAIN_EN = 0x00000120,
//
// enable reference output with main & decimated
// (for display) reconstruction
//
    JPEG_REF_MAIN_DISP_MAIN_AUX_EN = 0x00000130
}

// identifies the horizontal decimation factor
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jpeg_horizontal_deci_factor_t {
// no resize
    JPEG_HDEC_1 = 0x00000000,
// Advanced H/2 resize using improved 8-tap filters
    JPEG_HDEC_ADVANCED_2 = 0x00000101,
// Advanced H/4 resize using improved 8-tap filters
    JPEG_HDEC_ADVANCED_4 = 0x00000102
}

// identifies the vertical decimation factor
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jpeg_vertical_deci_factor_t {
// no resize
    JPEG_VDEC_1 = 0x00000000,
// V/2 , progressive resize
    JPEG_VDEC_ADVANCED_2_PROG = 0x00000204,
// V/2 , interlaced resize
    JPEG_VDEC_ADVANCED_2_INT = 0x000000208
}

// status of the decoding process
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jpeg_decoding_error_t {
    JPEG_DECODER_NO_ERROR = 0,
    JPEG_DECODER_UNDEFINED_HUFF_TABLE = 1,
    JPEG_DECODER_UNSUPPORTED_MARKER = 2,
    JPEG_DECODER_UNABLE_ALLOCATE_MEMORY = 3,
    JPEG_DECODER_NON_SUPPORTED_SAMP_FACTORS = 4,
    JPEG_DECODER_BAD_PARAMETER = 5,
    JPEG_DECODER_DECODE_ERROR = 6,
    JPEG_DECODER_BAD_RESTART_MARKER = 7,
    JPEG_DECODER_UNSUPPORTED_COLORSPACE = 8,
    JPEG_DECODER_BAD_SOS_SPECTRAL = 9,
    JPEG_DECODER_BAD_SOS_SUCCESSIVE = 10,
    JPEG_DECODER_BAD_HEADER_LENGTH = 11,
    JPEG_DECODER_BAD_COUNT_VALUE = 12,
    JPEG_DECODER_BAD_DHT_MARKER = 13,
    JPEG_DECODER_BAD_INDEX_VALUE = 14,
    JPEG_DECODER_BAD_NUMBER_HUFFMAN_TABLES = 15,
    JPEG_DECODER_BAD_QUANT_TABLE_LENGTH = 16,
    JPEG_DECODER_BAD_NUMBER_QUANT_TABLES = 17,
    JPEG_DECODER_BAD_COMPONENT_COUNT = 18,
    JPEG_DECODER_DIVIDE_BY_ZERO_ERROR = 19,
    JPEG_DECODER_NOT_JPG_IMAGE = 20,
    JPEG_DECODER_UNSUPPORTED_ROTATION_ANGLE = 21,
    JPEG_DECODER_UNSUPPORTED_SCALING = 22,
    JPEG_DECODER_INSUFFICIENT_OUTPUTBUFFER_SIZE = 23,
    JPEG_DECODER_BAD_HWCFG_GP_VERSION_VALUE = 24,
    JPEG_DECODER_BAD_VALUE_FROM_RED = 25,
    JPEG_DECODER_BAD_SUBREGION_PARAMETERS = 26,
    JPEG_DECODER_PROGRESSIVE_DECODE_NOT_SUPPORTED = 27,
    JPEG_DECODER_ERROR_TASK_TIMEOUT = 28,
    JPEG_DECODER_ERROR_FEATURE_NOT_SUPPORTED = 29
}

// identifies the decoding mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jpeg_decoding_mode_t {
    JPEG_NORMAL_DECODE = 0,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum jpeg_additional_flags_t {
    JPEG_ADDITIONAL_FLAG_NONE = 0,
// request firmware to return values of the CEH registers
    JPEG_ADDITIONAL_FLAG_CEH = 1,
// output storage of auxiliary reconstruction in Raster format.
    JPEG_ADDITIONAL_FLAG_RASTER = 64,
// output storage of auxiliary reconstruction in 420MB format.
    JPEG_ADDITIONAL_FLAG_420MB = 128
}

//
// struct jpeg_video_decode_init_params_t - initialization command parameters
//
// @circular_buffer_begin_addr_p:	start address of fw circular buffer
// @circular_buffer_end_addr_p:		end address of fw circular buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_video_decode_init_params_t {
    pub circular_buffer_begin_addr_p: u32,
    pub circular_buffer_end_addr_p: u32,
    pub reserved: u32,
}

//
// struct jpeg_decode_params_t - decode command parameters
//
// @picture_start_addr_p:	start address of jpeg picture
// @picture_end_addr_p:		end address of jpeg picture
// @decoded_buffer_addr:	decoded picture buffer
// @display_buffer_addr:	display picture buffer
// @main_aux_enable:		enable main and/or aux outputs
// @horizontal_decimation_factor:horizontal decimation factor
// @vertical_decimation_factor:	vertical decimation factor
// @xvalue0:			the x(0) coordinate for subregion decoding
// @xvalue1:			the x(1) coordinate for subregion decoding
// @yvalue0:			the y(0) coordinate for subregion decoding
// @yvalue1:			the y(1) coordinate for subregion decoding
// @decoding_mode:		decoding mode
// @additional_flags:		additional flags
// @field_flag:			determines frame/field scan
// @is_jpeg_image:		1 = still jpeg, 0 = motion jpeg
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decode_params_t {
    pub picture_start_addr_p: u32,
    pub picture_end_addr_p: u32,
    pub decoded_buffer_addr: jpeg_decoded_buffer_address_t,
    pub display_buffer_addr: jpeg_display_buffer_address_t,
    pub main_aux_enable: jpeg_rcn_ref_disp_enable_t,
    pub horizontal_decimation_factor: jpeg_horizontal_deci_factor_t,
    pub vertical_decimation_factor: jpeg_vertical_deci_factor_t,
    pub xvalue0: u32,
    pub xvalue1: u32,
    pub yvalue0: u32,
    pub yvalue1: u32,
    pub decoding_mode: jpeg_decoding_mode_t,
    pub additional_flags: u32,
    pub field_flag: u32,
    pub reserved: u32,
    pub is_jpeg_image: u32,
}

//
// struct jpeg_decode_return_params_t
//
// status returned by firmware after decoding
//
// @decode_time_in_us:	decoding time in microseconds
// @pm_cycles:		profiling information
// @pm_dmiss:		profiling information
// @pm_imiss:		profiling information
// @pm_bundles:		profiling information
// @pm_pft:		profiling information
// @error_code:		status of the decoding process
// @ceh_registers:	array where values of the Contrast Enhancement
// Histogram (CEH) registers will be stored.
// ceh_registers[0] correspond to register MBE_CEH_0_7,
// ceh_registers[1] correspond to register MBE_CEH_8_15
// ceh_registers[2] correspond to register MBE_CEH_16_23
// Note that elements of this array will be updated only
// if additional_flags has JPEG_ADDITIONAL_FLAG_CEH set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jpeg_decode_return_params_t {
// profiling info
    pub decode_time_in_us: u32,
    pub pm_cycles: u32,
    pub pm_dmiss: u32,
    pub pm_imiss: u32,
    pub pm_bundles: u32,
    pub pm_pft: u32,
    pub error_code: jpeg_decoding_error_t,
    pub ceh_registers: [u32; 32],
}
