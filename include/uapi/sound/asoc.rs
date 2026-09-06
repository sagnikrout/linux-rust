//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/sound/asoc.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// uapi/sound/asoc.h -- ALSA SoC Firmware Controls and DAPM
//
// Copyright (C) 2012 Texas Instruments Inc.
// Copyright (C) 2015 Intel Corporation.
//
// Simple file API to load FW that includes mixers, coefficients, DAPM graphs,
// algorithms, equalisers, DAIs, widgets etc.
//

//
// Maximum number of channels topology kcontrol can represent.
//
pub const SND_SOC_TPLG_MAX_CHAN: c_int = 8;
//
// Maximum number of PCM formats capability
//
pub const SND_SOC_TPLG_MAX_FORMATS: c_int = 16;
//
// Maximum number of PCM stream configs
//
pub const SND_SOC_TPLG_STREAM_CONFIG_MAX: c_int = 8;
//
// Maximum number of physical link's hardware configs
//
pub const SND_SOC_TPLG_HW_CONFIG_MAX: c_int = 8;
// individual kcontrol info types - can be mixed with other types
pub const SND_SOC_TPLG_CTL_VOLSW: c_int = 1;
pub const SND_SOC_TPLG_CTL_VOLSW_SX: c_int = 2;
pub const SND_SOC_TPLG_CTL_VOLSW_XR_SX: c_int = 3;
pub const SND_SOC_TPLG_CTL_ENUM: c_int = 4;
pub const SND_SOC_TPLG_CTL_BYTES: c_int = 5;
pub const SND_SOC_TPLG_CTL_ENUM_VALUE: c_int = 6;
pub const SND_SOC_TPLG_CTL_RANGE: c_int = 7;
pub const SND_SOC_TPLG_CTL_STROBE: c_int = 8;
// individual widget kcontrol info types - can be mixed with other types
pub const SND_SOC_TPLG_DAPM_CTL_VOLSW: c_int = 64;
pub const SND_SOC_TPLG_DAPM_CTL_ENUM_DOUBLE: c_int = 65;
pub const SND_SOC_TPLG_DAPM_CTL_ENUM_VIRT: c_int = 66;
pub const SND_SOC_TPLG_DAPM_CTL_ENUM_VALUE: c_int = 67;
pub const SND_SOC_TPLG_DAPM_CTL_PIN: c_int = 68;
// DAPM widget types - add new items to the end
pub const SND_SOC_TPLG_DAPM_INPUT: c_int = 0;
pub const SND_SOC_TPLG_DAPM_OUTPUT: c_int = 1;
pub const SND_SOC_TPLG_DAPM_MUX: c_int = 2;
pub const SND_SOC_TPLG_DAPM_MIXER: c_int = 3;
pub const SND_SOC_TPLG_DAPM_PGA: c_int = 4;
pub const SND_SOC_TPLG_DAPM_OUT_DRV: c_int = 5;
pub const SND_SOC_TPLG_DAPM_ADC: c_int = 6;
pub const SND_SOC_TPLG_DAPM_DAC: c_int = 7;
pub const SND_SOC_TPLG_DAPM_SWITCH: c_int = 8;
pub const SND_SOC_TPLG_DAPM_PRE: c_int = 9;
pub const SND_SOC_TPLG_DAPM_POST: c_int = 10;
pub const SND_SOC_TPLG_DAPM_AIF_IN: c_int = 11;
pub const SND_SOC_TPLG_DAPM_AIF_OUT: c_int = 12;
pub const SND_SOC_TPLG_DAPM_DAI_IN: c_int = 13;
pub const SND_SOC_TPLG_DAPM_DAI_OUT: c_int = 14;
pub const SND_SOC_TPLG_DAPM_DAI_LINK: c_int = 15;
pub const SND_SOC_TPLG_DAPM_BUFFER: c_int = 16;
pub const SND_SOC_TPLG_DAPM_SCHEDULER: c_int = 17;
pub const SND_SOC_TPLG_DAPM_EFFECT: c_int = 18;
pub const SND_SOC_TPLG_DAPM_SIGGEN: c_int = 19;
pub const SND_SOC_TPLG_DAPM_SRC: c_int = 20;
pub const SND_SOC_TPLG_DAPM_ASRC: c_int = 21;
pub const SND_SOC_TPLG_DAPM_ENCODER: c_int = 22;
pub const SND_SOC_TPLG_DAPM_DECODER: c_int = 23;

// Header magic number and string sizes
pub const SND_SOC_TPLG_MAGIC: c_uint = 0x41536F43 /* ASoC */;
// string sizes
pub const SND_SOC_TPLG_NUM_TEXTS: c_int = 16;
// ABI version
pub const SND_SOC_TPLG_ABI_VERSION: c_uint = 0x5	/* current version */;
pub const SND_SOC_TPLG_ABI_VERSION_MIN: c_uint = 0x5	/* oldest version supported */;
// Max size of TLV data
pub const SND_SOC_TPLG_TLV_SIZE: c_int = 32;
//
// File and Block header data types.
// Add new generic and vendor types to end of list.
// Generic types are handled by the core whilst vendors types are passed
// to the component drivers for handling.
//
pub const SND_SOC_TPLG_TYPE_MIXER: c_int = 1;
pub const SND_SOC_TPLG_TYPE_BYTES: c_int = 2;
pub const SND_SOC_TPLG_TYPE_ENUM: c_int = 3;
pub const SND_SOC_TPLG_TYPE_DAPM_GRAPH: c_int = 4;
pub const SND_SOC_TPLG_TYPE_DAPM_WIDGET: c_int = 5;
pub const SND_SOC_TPLG_TYPE_DAI_LINK: c_int = 6;
pub const SND_SOC_TPLG_TYPE_PCM: c_int = 7;
pub const SND_SOC_TPLG_TYPE_MANIFEST: c_int = 8;
pub const SND_SOC_TPLG_TYPE_CODEC_LINK: c_int = 9;
pub const SND_SOC_TPLG_TYPE_BACKEND_LINK: c_int = 10;
pub const SND_SOC_TPLG_TYPE_PDATA: c_int = 11;
pub const SND_SOC_TPLG_TYPE_DAI: c_int = 12;

// vendor block IDs - please add new vendor types to end
pub const SND_SOC_TPLG_TYPE_VENDOR_FW: c_int = 1000;
pub const SND_SOC_TPLG_TYPE_VENDOR_CONFIG: c_int = 1001;
pub const SND_SOC_TPLG_TYPE_VENDOR_COEFF: c_int = 1002;
pub const SND_SOC_TPLG_TYPEVENDOR_CODEC: c_int = 1003;
pub const SND_SOC_TPLG_STREAM_PLAYBACK: c_int = 0;
pub const SND_SOC_TPLG_STREAM_CAPTURE: c_int = 1;
// vendor tuple types
pub const SND_SOC_TPLG_TUPLE_TYPE_UUID: c_int = 0;
pub const SND_SOC_TPLG_TUPLE_TYPE_STRING: c_int = 1;
pub const SND_SOC_TPLG_TUPLE_TYPE_BOOL: c_int = 2;
pub const SND_SOC_TPLG_TUPLE_TYPE_BYTE: c_int = 3;
pub const SND_SOC_TPLG_TUPLE_TYPE_WORD: c_int = 4;
pub const SND_SOC_TPLG_TUPLE_TYPE_SHORT: c_int = 5;
// DAI flags

// DAI clock gating
pub const SND_SOC_TPLG_DAI_CLK_GATE_UNDEFINED: c_int = 0;
pub const SND_SOC_TPLG_DAI_CLK_GATE_GATED: c_int = 1;
pub const SND_SOC_TPLG_DAI_CLK_GATE_CONT: c_int = 2;
// DAI mclk_direction

// DAI physical PCM data formats.
// Add new formats to the end of the list.
//

// left and right justified also known as MSB and LSB respectively

// DAI link flags

// DAI topology BCLK parameter
// For the backwards capability, by default codec is bclk provider
//

// keep previous definitions for compatibility

// DAI topology FSYNC parameter
// For the backwards capability, by default codec is fsync provider
//

// keep previous definitions for compatibility

//
// Block Header.
// This header precedes all object and object arrays below.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_hdr {
    pub /: *mut *mut __le32 magic; / magic number,
    pub /: *mut *mut __le32 abi; / ABI version,
    pub /: *mut *mut __le32 version; / optional vendor specific version details,
    pub /: *mut *mut __le32 type; / SND_SOC_TPLG_TYPE_,
    pub /: *mut *mut __le32 size; / size of this structure,
    pub /: *mut *mut __le32 vendor_type; / optional vendor specific type info,
    pub /: *mut *mut __le32 payload_size; / data bytes, excluding this header,
    pub /: *mut *mut __le32 index; / identifier for block,
    pub /: *mut *mut __le32 count; / number of elements in block,
    pub __attribute__((packed)): },
// vendor tuple for uuid
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_vendor_uuid_elem {
    pub token: __le32,
    pub uuid: [c_char; 16],
    pub __attribute__((packed)): },
// vendor tuple for a bool/byte/short/word value
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_vendor_value_elem {
    pub token: __le32,
    pub value: __le32,
    pub __attribute__((packed)): },
// vendor tuple for string
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_vendor_string_elem {
    pub token: __le32,
    pub string: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_vendor_array {
    pub /: *mut *mut __le32 size; / size in bytes of the array, including all elements,
    pub /: *mut *mut __le32 type; / SND_SOC_TPLG_TUPLE_TYPE_,
    pub /: *mut *mut __le32 num_elems; / number of elements in array,
    pub uuid): __DECLARE_FLEX_ARRAY(struct snd_soc_tplg_vendor_uuid_elem,,
    pub value): __DECLARE_FLEX_ARRAY(struct snd_soc_tplg_vendor_value_elem,,
    pub string): __DECLARE_FLEX_ARRAY(struct snd_soc_tplg_vendor_string_elem,,
}

//
// Private data.
// All topology objects may have private data that can be used by the driver or
// firmware. Core will ignore this data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_private {
    pub /: *mut *mut __le32 size; / in bytes of private data,
    pub data): __DECLARE_FLEX_ARRAY(char,,
    pub array): __DECLARE_FLEX_ARRAY(struct snd_soc_tplg_vendor_array,,
}

//
// Kcontrol TLV data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_tlv_dbscale {
    pub min: __le32,
    pub step: __le32,
    pub mute: __le32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_ctl_tlv {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub /: *mut *mut *mut __le32 type; / SNDRV_CTL_TLVT_, type of TLV,
    pub data: [__le32; SND_SOC_TPLG_TLV_SIZE],
    pub scale: snd_soc_tplg_tlv_dbscale,
}

//
// Kcontrol channel data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_channel {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub reg: __le32,
    pub shift: __le32,
    pub /: *mut *mut __le32 id; / ID maps to Left, Right, LFE etc,
    pub __attribute__((packed)): },
//
// Genericl Operations IDs, for binding Kcontrol or Bytes ext ops
// Kcontrol ops need get/put/info.
// Bytes ext ops need get/put.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_io_ops {
    pub get: __le32,
    pub put: __le32,
    pub info: __le32,
    pub __attribute__((packed)): },
//
// kcontrol header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_ctl_hdr {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub type: __le32,
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub access: __le32,
    pub ops: snd_soc_tplg_io_ops,
    pub tlv: snd_soc_tplg_ctl_tlv,
    pub __attribute__((packed)): },
//
// Stream Capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_stream_caps {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub /: *mut *mut *mut __le64 formats; / supported formats SNDRV_PCM_FMTBIT_,
    pub /: *mut *mut *mut __le32 rates; / supported rates SNDRV_PCM_RATE_,
    pub /: *mut *mut __le32 rate_min; / min rate,
    pub /: *mut *mut __le32 rate_max; / max rate,
    pub /: *mut *mut __le32 channels_min; / min channels,
    pub /: *mut *mut __le32 channels_max; / max channels,
    pub /: *mut *mut __le32 periods_min; / min number of periods,
    pub /: *mut *mut __le32 periods_max; / max number of periods,
    pub /: *mut *mut __le32 period_size_min; / min period size bytes,
    pub /: *mut *mut __le32 period_size_max; / max period size bytes,
    pub /: *mut *mut __le32 buffer_size_min; / min buffer size bytes,
    pub /: *mut *mut __le32 buffer_size_max; / max buffer size bytes,
    pub /: *mut *mut __le32 sig_bits; / number of bits of content,
    pub __attribute__((packed)): },
//
// FE or BE Stream configuration supported by SW/FW
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_stream {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub /: *mut *mut char name[SNDRV_CTL_ELEM_ID_NAME_MAXLEN]; / Name of the stream,
    pub /: *mut *mut *mut __le64 format; / SNDRV_PCM_FMTBIT_,
    pub /: *mut *mut *mut __le32 rate; / SNDRV_PCM_RATE_,
    pub /: *mut *mut __le32 period_bytes; / size of period in bytes,
    pub /: *mut *mut __le32 buffer_bytes; / size of buffer in bytes,
    pub /: *mut *mut __le32 channels; / channels,
    pub __attribute__((packed)): },
//
// Describes a physical link's runtime supported hardware config,
// i.e. hardware audio formats.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_hw_config {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub /: *mut *mut __le32 id; / unique ID - - used to match,
    pub /: *mut *mut __le32 fmt; / SND_SOC_DAI_FORMAT_ format value,
    pub /: *mut *mut __u8 clock_gated; / SND_SOC_TPLG_DAI_CLK_GATE_ value,
    pub /: *mut *mut __u8 invert_bclk; / 1 for inverted BCLK, 0 for normal,
    pub /: *mut *mut __u8 invert_fsync; / 1 for inverted frame clock, 0 for normal,
    pub /: *mut *mut __u8 bclk_provider; / SND_SOC_TPLG_BCLK_ value,
    pub /: *mut *mut __u8 fsync_provider; / SND_SOC_TPLG_FSYNC_ value,
    pub /: *mut *mut __u8 mclk_direction; / SND_SOC_TPLG_MCLK_ value,
    pub /: *mut *mut __le16 reserved; / for 32bit alignment,
    pub /: *mut *mut __le32 mclk_rate; / MCLK or SYSCLK freqency in Hz,
    pub /: *mut *mut __le32 bclk_rate; / BCLK freqency in Hz,
    pub /: *mut *mut __le32 fsync_rate; / frame clock in Hz,
    pub /: *mut *mut __le32 tdm_slots; / number of TDM slots in use,
    pub /: *mut *mut __le32 tdm_slot_width; / width in bits for each slot,
    pub /: *mut *mut __le32 tx_slots; / bit mask for active Tx slots,
    pub /: *mut *mut __le32 rx_slots; / bit mask for active Rx slots,
    pub /: *mut *mut __le32 tx_channels; / number of Tx channels,
    pub /: *mut *mut __le32 tx_chanmap[SND_SOC_TPLG_MAX_CHAN]; / array of slot number,
    pub /: *mut *mut __le32 rx_channels; / number of Rx channels,
    pub /: *mut *mut __le32 rx_chanmap[SND_SOC_TPLG_MAX_CHAN]; / array of slot number,
    pub __attribute__((packed)): },
//
// Manifest. List totals for each payload type. Not used in parsing, but will
// be passed to the component driver before any other objects in order for any
// global component resource allocations.
//
// File block representation for manifest :-
// +-----------------------------------+----+
// | struct snd_soc_tplg_hdr           |  1 |
// +-----------------------------------+----+
// | struct snd_soc_tplg_manifest      |  1 |
// +-----------------------------------+----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_manifest {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub /: *mut *mut __le32 control_elems; / number of control elements,
    pub /: *mut *mut __le32 widget_elems; / number of widget elements,
    pub /: *mut *mut __le32 graph_elems; / number of graph elements,
    pub /: *mut *mut __le32 pcm_elems; / number of PCM elements,
    pub /: *mut *mut __le32 dai_link_elems; / number of DAI link elements,
    pub /: *mut *mut __le32 dai_elems; / number of physical DAI elements,
    pub /: *mut *mut __le32 reserved[20]; / reserved for new ABI element types,
    pub priv: snd_soc_tplg_private,
    pub __attribute__((packed)): },
//
// Mixer kcontrol.
//
// File block representation for mixer kcontrol :-
// +-----------------------------------+----+
// | struct snd_soc_tplg_hdr           |  1 |
// +-----------------------------------+----+
// | struct snd_soc_tplg_mixer_control |  N |
// +-----------------------------------+----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_mixer_control {
    pub hdr: snd_soc_tplg_ctl_hdr,
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub min: __le32,
    pub max: __le32,
    pub platform_max: __le32,
    pub invert: __le32,
    pub num_channels: __le32,
    pub channel: [snd_soc_tplg_channel; SND_SOC_TPLG_MAX_CHAN],
    pub priv: snd_soc_tplg_private,
    pub __attribute__((packed)): },
//
// Enumerated kcontrol
//
// File block representation for enum kcontrol :-
// +-----------------------------------+----+
// | struct snd_soc_tplg_hdr           |  1 |
// +-----------------------------------+----+
// | struct snd_soc_tplg_enum_control  |  N |
// +-----------------------------------+----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_enum_control {
    pub hdr: snd_soc_tplg_ctl_hdr,
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub num_channels: __le32,
    pub channel: [snd_soc_tplg_channel; SND_SOC_TPLG_MAX_CHAN],
    pub items: __le32,
    pub mask: __le32,
    pub count: __le32,
    pub texts: [c_char; SND_SOC_TPLG_NUM_TEXTS][SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub 4]: *mut *mut __le32 values[SND_SOC_TPLG_NUM_TEXTS  SNDRV_CTL_ELEM_ID_NAME_MAXLEN /,
    pub priv: snd_soc_tplg_private,
    pub __attribute__((packed)): },
//
// Bytes kcontrol
//
// File block representation for bytes kcontrol :-
// +-----------------------------------+----+
// | struct snd_soc_tplg_hdr           |  1 |
// +-----------------------------------+----+
// | struct snd_soc_tplg_bytes_control |  N |
// +-----------------------------------+----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_bytes_control {
    pub hdr: snd_soc_tplg_ctl_hdr,
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub max: __le32,
    pub mask: __le32,
    pub base: __le32,
    pub num_regs: __le32,
    pub ext_ops: snd_soc_tplg_io_ops,
    pub priv: snd_soc_tplg_private,
    pub __attribute__((packed)): },
//
// DAPM Graph Element
//
// File block representation for DAPM graph elements :-
// +-------------------------------------+----+
// | struct snd_soc_tplg_hdr             |  1 |
// +-------------------------------------+----+
// | struct snd_soc_tplg_dapm_graph_elem |  N |
// +-------------------------------------+----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_dapm_graph_elem {
    pub sink: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub control: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub source: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub __attribute__((packed)): },
//
// DAPM Widget.
//
// File block representation for DAPM widget :-
// +-------------------------------------+-----+
// | struct snd_soc_tplg_hdr             |  1  |
// +-------------------------------------+-----+
// | struct snd_soc_tplg_dapm_widget     |  N  |
// +-------------------------------------+-----+
// |   struct snd_soc_tplg_enum_control  | 0|1 |
// |   struct snd_soc_tplg_mixer_control | 0|N |
// +-------------------------------------+-----+
//
// Optional enum or mixer control can be appended to the end of each widget
// in the block.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_dapm_widget {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub /: *mut *mut __le32 id; / SND_SOC_DAPM_CTL,
    pub name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub sname: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub /: *mut *mut __le32 reg; / negative reg = no direct dapm,
    pub /: *mut *mut __le32 shift; / bits to shift,
    pub /: *mut *mut __le32 mask; / non-shifted mask,
    pub /: *mut *mut __le32 subseq; / sort within widget type,
    pub /: *mut *mut __le32 invert; / invert the power bit,
    pub /: *mut *mut __le32 ignore_suspend; / kept enabled over suspend,
    pub event_flags: __le16,
    pub event_type: __le16,
    pub num_kcontrols: __le32,
    pub priv: snd_soc_tplg_private,
//
// kcontrols that relate to this widget
// follow here after widget private data
//
    pub __attribute__((packed)): },
//
// Describes SW/FW specific features of PCM (FE DAI & DAI link).
//
// File block representation for PCM :-
// +-----------------------------------+-----+
// | struct snd_soc_tplg_hdr           |  1  |
// +-----------------------------------+-----+
// | struct snd_soc_tplg_pcm           |  N  |
// +-----------------------------------+-----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_pcm {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub pcm_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub dai_name: [c_char; SNDRV_CTL_ELEM_ID_NAME_MAXLEN],
    pub /: *mut *mut __le32 pcm_id; / unique ID - used to match with DAI link,
    pub /: *mut *mut __le32 dai_id; / unique ID - used to match,
    pub /: *mut *mut __le32 playback; / supports playback mode,
    pub /: *mut *mut __le32 capture; / supports capture mode,
    pub /: *mut *mut __le32 compress; / 1 = compressed; 0 = PCM,
    pub /: *mut *mut snd_soc_tplg_stream stream[SND_SOC_TPLG_STREAM_CONFIG_MAX]; / for DAI link,
    pub /: *mut *mut __le32 num_streams; / number of streams,
    pub /: *mut *mut snd_soc_tplg_stream_caps caps[2]; / playback and capture for DAI,
    pub /: *mut *mut __le32 flag_mask; / bitmask of flags to configure,
    pub /: *mut *mut *mut __le32 flags; / SND_SOC_TPLG_LNK_FLGBIT_ flag value,
    pub priv: snd_soc_tplg_private,
    pub __attribute__((packed)): },
//
// Describes the physical link runtime supported configs or params
//
// File block representation for physical link config :-
// +-----------------------------------+-----+
// | struct snd_soc_tplg_hdr           |  1  |
// +-----------------------------------+-----+
// | struct snd_soc_tplg_link_config   |  N  |
// +-----------------------------------+-----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_link_config {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub /: *mut *mut __le32 id; / unique ID - used to match,
    pub /: *mut *mut char name[SNDRV_CTL_ELEM_ID_NAME_MAXLEN]; / name - used to match,
    pub /: *mut *mut char stream_name[SNDRV_CTL_ELEM_ID_NAME_MAXLEN]; / stream name - used to match,
    pub /: *mut *mut snd_soc_tplg_stream stream[SND_SOC_TPLG_STREAM_CONFIG_MAX]; / supported configs playback and captrure,
    pub /: *mut *mut __le32 num_streams; / number of streams,
    pub /: *mut *mut snd_soc_tplg_hw_config hw_config[SND_SOC_TPLG_HW_CONFIG_MAX]; / hw configs,
    pub /: *mut *mut __le32 num_hw_configs; / number of hw configs,
    pub /: *mut *mut __le32 default_hw_config_id; / default hw config ID for init,
    pub /: *mut *mut __le32 flag_mask; / bitmask of flags to configure,
    pub /: *mut *mut *mut __le32 flags; / SND_SOC_TPLG_LNK_FLGBIT_ flag value,
    pub priv: snd_soc_tplg_private,
    pub __attribute__((packed)): },
//
// Describes SW/FW specific features of physical DAI.
// It can be used to configure backend DAIs for DPCM.
//
// File block representation for physical DAI :-
// +-----------------------------------+-----+
// | struct snd_soc_tplg_hdr           |  1  |
// +-----------------------------------+-----+
// | struct snd_soc_tplg_dai           |  N  |
// +-----------------------------------+-----+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_tplg_dai {
    pub /: *mut *mut __le32 size; / in bytes of this structure,
    pub /: *mut *mut char dai_name[SNDRV_CTL_ELEM_ID_NAME_MAXLEN]; / name - used to match,
    pub /: *mut *mut __le32 dai_id; / unique ID - used to match,
    pub /: *mut *mut __le32 playback; / supports playback mode,
    pub /: *mut *mut __le32 capture; / supports capture mode,
    pub /: *mut *mut snd_soc_tplg_stream_caps caps[2]; / playback and capture for DAI,
    pub /: *mut *mut __le32 flag_mask; / bitmask of flags to configure,
    pub /: *mut *mut *mut __le32 flags; / SND_SOC_TPLG_DAI_FLGBIT_,
    pub priv: snd_soc_tplg_private,
    pub __attribute__((packed)): },
