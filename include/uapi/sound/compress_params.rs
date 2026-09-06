//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/compress_params.h
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


// SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) AND MIT)
//
// compress_params.h - codec types and parameters for compressed data
// streaming interface
//
// Copyright (C) 2011 Intel Corporation
// Authors:	Pierre-Louis Bossart <pierre-louis.bossart@linux.intel.com>
// Vinod Koul <vinod.koul@linux.intel.com>
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// The definitions in this file are derived from the OpenMAX AL version 1.1
// and OpenMAX IL v 1.1.2 header files which contain the copyright notice below
// and are licensed under the MIT license.
//
// Copyright (c) 2007-2010 The Khronos Group Inc.
//

// AUDIO CODECS SUPPORTED
pub const MAX_NUM_CODECS: c_int = 32;
pub const MAX_NUM_CODEC_DESCRIPTORS: c_int = 32;
pub const MAX_NUM_BITRATES: c_int = 32;
pub const MAX_NUM_SAMPLE_RATES: c_int = 32;
// Codecs are listed linearly to allow for extensibility

//
// Profile and modes are listed with bit masks. This allows for a
// more compact representation of fields that will not evolve
// (in contrast to the list of codecs)
//

// MP3 modes are only useful for encoders

// AMR modes are only useful for encoders

// AMRWB modes are only useful for encoders

// AAC modes are required for encoders and decoders

// AAC formats are required for encoders and decoders

//
// Some implementations strip the ASF header and only send ASF packets
// to the DSP
//

//
// Define quality levels for FLAC encoders, from LEVEL0 (fast)
// to LEVEL8 (best)
//

// IEC61937 payloads without CUVP and preambles

// IEC61937 with S/PDIF preambles+CUVP bits in 32-bit containers

//
// IEC modes are mandatory for decoders. Format autodetection
// will only happen on the DSP side with mode 0. The PCM mode should
// not be used, the PCM codec should be used instead.
//

// <FIXME: multichannel encoders aren't supported for now. Would need
// VBR/CBR definitions

// Encoder options
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_enc_wma {
    pub /: *mut *mut __u32 super_block_align; / WMA Type-specific data,
}

//
// struct snd_enc_vorbis - Vorbis encoder parameters
// @quality: Sets encoding quality to n, between -1 (low) and 10 (high).
// In the default mode of operation, the quality level is 3.
// Normal quality range is 0 - 10.
// @managed: Boolean. Set  bitrate  management  mode. This turns off the
// normal VBR encoding, but allows hard or soft bitrate constraints to be
// enforced by the encoder. This mode can be slower, and may also be
// lower quality. It is primarily useful for streaming.
// @max_bit_rate: Enabled only if managed is TRUE
// @min_bit_rate: Enabled only if managed is TRUE
// @downmix: Boolean. Downmix input from stereo to mono (has no effect on
// non-stereo streams). Useful for lower-bitrate encoding.
//
// These options were extracted from the OpenMAX IL spec and Gstreamer vorbisenc
// properties
//
// For best quality users should specify VBR mode and set quality levels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_enc_vorbis {
    pub quality: __s32,
    pub managed: __u32,
    pub max_bit_rate: __u32,
    pub min_bit_rate: __u32,
    pub downmix: __u32,
// C attribute field omitted
//
// struct snd_enc_real - RealAudio encoder parameters
// @quant_bits: number of coupling quantization bits in the stream
// @start_region: coupling start region in the stream
// @num_regions: number of regions value
//
// These options were extracted from the OpenMAX IL spec
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_enc_real {
    pub quant_bits: __u32,
    pub start_region: __u32,
    pub num_regions: __u32,
// C attribute field omitted
//
// struct snd_enc_flac - FLAC encoder parameters
// @num: serial number, valid only for OGG formats
// needs to be set by application
// @gain: Add replay gain tags
//
// These options were extracted from the FLAC online documentation
// at http://flac.sourceforge.net/documentation_tools_flac.html
//
// To make the API simpler, it is assumed that the user will select quality
// profiles. Additional options that affect encoding quality and speed can
// be added at a later stage if needed.
//
// By default the Subset format is used by encoders.
//
// TAGS such as pictures, etc, cannot be handled by an offloaded encoder and are
// not supported in this API.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_enc_flac {
    pub num: __u32,
    pub gain: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_enc_generic {
    pub /: *mut *mut __u32 bw; / encoder bandwidth,
    pub /: *mut *mut __s32 reserved[15]; / Can be used for SND_AUDIOCODEC_BESPOKE,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dec_flac {
    pub sample_size: __u16,
    pub min_blk_size: __u16,
    pub max_blk_size: __u16,
    pub min_frame_size: __u16,
    pub max_frame_size: __u16,
    pub reserved: __u16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dec_wma {
    pub encoder_option: __u32,
    pub adv_encoder_option: __u32,
    pub adv_encoder_option2: __u32,
    pub reserved: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dec_alac {
    pub frame_length: __u32,
    pub compatible_version: __u8,
    pub pb: __u8,
    pub mb: __u8,
    pub kb: __u8,
    pub max_run: __u32,
    pub max_frame_bytes: __u32,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dec_ape {
    pub compatible_version: __u16,
    pub compression_level: __u16,
    pub format_flags: __u32,
    pub blocks_per_frame: __u32,
    pub final_frame_blocks: __u32,
    pub total_frames: __u32,
    pub seek_table_present: __u32,
// C attribute field omitted
//
// struct snd_dec_opus - Opus decoder parameters (raw opus packets)
// @version: Usually should be '1' but can be split into major (4 upper bits)
// and minor (4 lower bits) sub-fields.
// @num_channels: Number of output channels.
// @pre_skip: Number of samples to discard at 48 kHz.
// @sample_rate: Sample rate of original input.
// @output_gain: Gain to apply when decoding (in Q7.8 format).
// @mapping_family: Order and meaning of output channels. Only values 0 and 1
// are expected; values 2..255 are not recommended for playback.
//
// @chan_map: Optional channel mapping table. Describes mapping of opus streams
// to decoded channels. Fields:
// @chan_map.stream_count: Number of streams encoded in each Ogg packet.
// @chan_map.coupled_count: Number of streams whose decoders are used
// for two channels.
// @chan_map.channel_map: Which decoded channel to be used for each one.
// Supports only mapping families 0 and 1,
// max number of channels is 8.
//
// These options were extracted from RFC7845 Section 5.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dec_opus {
    pub version: __u8,
    pub num_channels: __u8,
    pub pre_skip: __u16,
    pub sample_rate: __u32,
    pub output_gain: __u16,
    pub mapping_family: __u8,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_dec_opus_ch_map {
    pub stream_count: __u8,
    pub coupled_count: __u8,
    pub channel_map: [__u8; 8],
    pub chan_map: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub union snd_codec_options {
    pub wma: snd_enc_wma,
    pub vorbis: snd_enc_vorbis,
    pub real: snd_enc_real,
    pub flac: snd_enc_flac,
    pub generic: snd_enc_generic,
    pub flac_d: snd_dec_flac,
    pub wma_d: snd_dec_wma,
    pub alac_d: snd_dec_alac,
    pub ape_d: snd_dec_ape,
    pub opus_d: snd_dec_opus,
    pub out_sample_rate: __u32,
    pub src_d: },
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_codec_desc_src {
    pub out_sample_rate_min: __u32,
    pub out_sample_rate_max: __u32,
// C attribute field omitted
// struct snd_codec_desc - description of codec capabilities
// @max_ch: Maximum number of audio channels
// @sample_rates: Sampling rates in Hz, use values like 48000 for this
// @num_sample_rates: Number of valid values in sample_rates array
// @bit_rate: Indexed array containing supported bit rates
// @num_bitrates: Number of valid values in bit_rate array
// @rate_control: value is specified by SND_RATECONTROLMODE defines.
// @profiles: Supported profiles. See SND_AUDIOPROFILE defines.
// @modes: Supported modes. See SND_AUDIOMODE defines
// @formats: Supported formats. See SND_AUDIOSTREAMFORMAT defines
// @min_buffer: Minimum buffer size handled by codec implementation
// @pcm_formats: Output (for decoders) or input (for encoders)
// PCM formats (required to accel mode, 0 for other modes)
// @u_space: union space (for codec dependent data)
// @reserved: reserved for future use
//
// This structure provides a scalar value for profiles, modes and stream
// format fields.
// If an implementation supports multiple combinations, they will be listed as
// codecs with different descriptors, for example there would be 2 descriptors
// for AAC-RAW and AAC-ADTS.
// This entails some redundancy but makes it easier to avoid invalid
// configurations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_codec_desc {
    pub max_ch: __u32,
    pub sample_rates: [__u32; MAX_NUM_SAMPLE_RATES],
    pub num_sample_rates: __u32,
    pub bit_rate: [__u32; MAX_NUM_BITRATES],
    pub num_bitrates: __u32,
    pub rate_control: __u32,
    pub profiles: __u32,
    pub modes: __u32,
    pub formats: __u32,
    pub min_buffer: __u32,
    pub pcm_formats: __u32,
    pub u_space: [__u32; 6],
    pub src: snd_codec_desc_src,
// C attribute field omitted
    pub reserved: [__u32; 8],
// C attribute field omitted
// struct snd_codec
// @id: Identifies the supported audio encoder/decoder.
// See SND_AUDIOCODEC macros.
// @ch_in: Number of input audio channels
// @ch_out: Number of output channels. In case of contradiction between
// this field and the channelMode field, the channelMode field
// overrides.
// @sample_rate: Audio sample rate of input data in Hz, use values like 48000
// for this.
// @bit_rate: Bitrate of encoded data. May be ignored by decoders
// @rate_control: Encoding rate control. See SND_RATECONTROLMODE defines.
// Encoders may rely on profiles for quality levels.
// May be ignored by decoders.
// @profile: Mandatory for encoders, can be mandatory for specific
// decoders as well. See SND_AUDIOPROFILE defines.
// @level: Supported level (Only used by WMA at the moment)
// @ch_mode: Channel mode for encoder. See SND_AUDIOCHANMODE defines
// @format: Format of encoded bistream. Mandatory when defined.
// See SND_AUDIOSTREAMFORMAT defines.
// @align: Block alignment in bytes of an audio sample.
// Only required for PCM or IEC formats.
// @options: encoder-specific settings
// @pcm_format: Output (for decoders) or input (for encoders)
// PCM formats (required to accel mode, 0 for other modes)
// @reserved: reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_codec {
    pub id: __u32,
    pub ch_in: __u32,
    pub ch_out: __u32,
    pub sample_rate: __u32,
    pub bit_rate: __u32,
    pub rate_control: __u32,
    pub profile: __u32,
    pub level: __u32,
    pub ch_mode: __u32,
    pub format: __u32,
    pub align: __u32,
    pub options: snd_codec_options,
    pub pcm_format: __u32,
    pub reserved: [__u32; 2],
// C attribute field omitted
