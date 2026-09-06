//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/soc-dai.h
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
// linux/sound/soc-dai.h -- ALSA SoC Layer
//
// Copyright:	2005-2008 Wolfson Microelectronics. PLC.
//
// Digital Audio Interface (DAI) API.
//

//
// DAI hardware audio formats.
//
// Describes the physical PCM data formating and clocking. Add new formats
// to the end.
//

// left and right justified also known as MSB and LSB respectively

// Describes the possible PCM format
//
// use SND_SOC_DAI_FORMAT_xx as eash shift.
// see
// snd_soc_runtime_get_dai_fmt()
//
pub const SND_SOC_POSSIBLE_DAIFMT_FORMAT_SHIFT: c_int = 0;

//
// DAI TDM slot idle modes
//
// Describes a CODEC/CPU's behaviour when not actively receiving or
// transmitting on a given TDM slot. NONE is undefined behaviour.
// Add new modes to the end.
//
pub const SND_SOC_DAI_TDM_IDLE_NONE: c_int = 0;
pub const SND_SOC_DAI_TDM_IDLE_OFF: c_int = 1;
pub const SND_SOC_DAI_TDM_IDLE_ZERO: c_int = 2;
pub const SND_SOC_DAI_TDM_IDLE_PULLDOWN: c_int = 3;
pub const SND_SOC_DAI_TDM_IDLE_HIZ: c_int = 4;
pub const SND_SOC_DAI_TDM_IDLE_PULLUP: c_int = 5;
pub const SND_SOC_DAI_TDM_IDLE_DRIVE_HIGH: c_int = 6;
//
// DAI Clock gating.
//
// DAI bit clocks can be gated (disabled) when the DAI is not
// sending or receiving PCM data in a frame. This can be used to save power.
//

// Describes the possible PCM format
//
// define GATED -> CONT. GATED will be selected if both are selected.
// see
// soc_dai_convert_possiblefmt_to_daifmt()
//
pub const SND_SOC_POSSIBLE_DAIFMT_CLOCK_SHIFT: c_int = 16;

//
// DAI hardware signal polarity.
//
// Specifies whether the DAI can also support inverted clocks for the specified
// format.
//
// BCLK:
// - "normal" polarity means signal is available at rising edge of BCLK
// - "inverted" polarity means signal is available at falling edge of BCLK
//
// FSYNC "normal" polarity depends on the frame format:
// - I2S: frame consists of left then right channel data. Left channel starts
// with falling FSYNC edge, right channel starts with rising FSYNC edge.
// - Left/Right Justified: frame consists of left then right channel data.
// Left channel starts with rising FSYNC edge, right channel starts with
// falling FSYNC edge.
// - DSP A/B: Frame starts with rising FSYNC edge.
// - AC97: Frame starts with rising FSYNC edge.
//
// "Negative" FSYNC polarity is the one opposite of "normal" polarity.
//

// Describes the possible PCM format
pub const SND_SOC_POSSIBLE_DAIFMT_INV_SHIFT: c_int = 32;

//
// DAI hardware clock providers/consumers
//
// This is wrt the codec, the inverse is true for the interface
// i.e. if the codec is clk and FRM provider then the interface is
// clk and frame consumer.
//

// when passed to set_fmt directly indicate if the device is provider or consumer

pub const SND_SOC_DAIFMT_FORMAT_MASK: c_uint = 0x000f;
pub const SND_SOC_DAIFMT_CLOCK_MASK: c_uint = 0x00f0;
pub const SND_SOC_DAIFMT_INV_MASK: c_uint = 0x0f00;
pub const SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint = 0xf000;

//
// Master Clock Directions
//
pub const SND_SOC_CLOCK_IN: c_int = 0;
pub const SND_SOC_CLOCK_OUT: c_int = 1;

// Digital Audio Interface clocking API.
extern "C" {
    pub fn snd_soc_dai_set_bclk_ratio(dai: *mut snd_soc_dai, ratio: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_soc_dai_set_bclk_clk(dai: *mut snd_soc_dai, bclk: *mut clk);
}
// Digital Audio interface formatting
extern "C" {
    pub fn snd_soc_dai_auto_select_format(rtd: *const snd_soc_pcm_runtime) -> c_uint;
}
extern "C" {
    pub fn snd_soc_dai_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
}
extern "C" {
    pub fn snd_soc_dai_set_tristate(dai: *mut snd_soc_dai, tristate: c_int) -> c_int;
}
// Digital Audio Interface mute
extern "C" {
    pub fn snd_soc_dai_mute_is_ctrled_at_trigger(dai: *mut snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn snd_soc_dai_is_dummy(dai: *const snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn snd_soc_dai_suspend(dai: *mut snd_soc_dai);
}
extern "C" {
    pub fn snd_soc_dai_resume(dai: *mut snd_soc_dai);
}
extern "C" {
    pub fn snd_soc_dai_compress_new(dai: *mut snd_soc_dai, rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
extern "C" {
    pub fn snd_soc_dai_stream_valid(dai: *const snd_soc_dai, stream: c_int) -> bool;
}
extern "C" {
    pub fn snd_soc_dai_active(dai: *const snd_soc_dai) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_dai_probe(rtd: *mut snd_soc_pcm_runtime, order: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_dai_remove(rtd: *mut snd_soc_pcm_runtime, order: c_int) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_dai_new(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
extern "C" {
    pub fn snd_soc_pcm_dai_prepare(substream: *mut snd_pcm_substream) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_ops {
// DAI driver callbacks
    pub dai): *mut *mut int (probe)(struct snd_soc_dai,
    pub dai): *mut *mut int (remove)(struct snd_soc_dai,
// compress dai
    pub rtd): *mut *mut int (compress_new)(struct snd_soc_pcm_runtime,
// Optional Callback used at pcm creation
    pub dai): *mut snd_soc_dai,
//
// DAI clocking configuration, all optional.
// Called by soc_card drivers, normally in their hw_params.
//
    pub dir): int clk_id, unsigned int freq, int,
    pub freq_out): unsigned int freq_in, unsigned int,
    pub div): *mut *mut *mut int (set_clkdiv)(struct snd_soc_dai dai, int div_id, int,
    pub ratio): *mut *mut *mut int (set_bclk_ratio)(struct snd_soc_dai dai, unsigned int,
//
// DAI format configuration
// Called by soc_card drivers, normally in their hw_params.
//
    pub fmt): *mut *mut *mut int (set_fmt)(struct snd_soc_dai dai, unsigned int,
    pub rx_mask): *mut *mut unsigned int tx_mask, unsigned int,
    pub slot_width): int slots, int,
    pub rx_mode): int tx_mode, int,
    pub rx_slot): *const unsigned int rx_num, unsigned int,
    pub rx_slot): *mut *mut unsigned int rx_num, unsigned int,
    pub tristate): *mut *mut *mut int (set_tristate)(struct snd_soc_dai dai, int,
    pub direction): *mut *mut void stream, int,
    pub direction): *mut *mut *mut *mut void (get_stream)(struct snd_soc_dai dai, int,
//
// DAI digital mute - optional.
// Called by soc-core to minimise any pops.
//
    pub stream): *mut *mut *mut int (mute_stream)(struct snd_soc_dai dai, int mute, int,
//
// ALSA PCM audio operations - all optional.
// Called by soc-core during audio PCM operations.
//
    pub ): *mut snd_soc_dai,
    pub ): *mut snd_soc_dai,
    pub ): *mut *mut snd_pcm_hw_params , snd_soc_dai,
    pub ): *mut snd_soc_dai,
    pub ): *mut snd_soc_dai,
//
// NOTE: Commands passed to the trigger function are not necessarily
// compatible with the current state of the dai. For example this
// sequence of commands is possible: START STOP STOP.
// So do not unconditionally use refcounting functions in the trigger
// function, e.g. clk_enable/disable.
//
    pub ): *mut snd_soc_dai,
//
// For hardware based FIFO caused delay reporting.
// Optional.
//
    pub ): *mut snd_soc_dai,
//
// Format list for auto selection.
// Format will be increased if priority format was
// not selected.
// see
// snd_soc_dai_get_fmt()
//
    pub auto_selectable_formats: *const u64,
    pub num_auto_selectable_formats: c_int,
// probe ordering - for components with runtime dependencies
    pub probe_order: c_int,
    pub remove_order: c_int,
// bit field
    pub no_capture_mute:1: c_uint,
    pub mute_unmute_on_trigger:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_cdai_ops {
//
// for compress ops
//
    pub ): *mut snd_soc_dai,
    pub ): *mut snd_soc_dai,
    pub ): *mut *mut snd_compr_params , snd_soc_dai,
    pub ): *mut *mut snd_codec , snd_soc_dai,
    pub ): *mut *mut snd_compr_metadata , snd_soc_dai,
    pub ): *mut *mut snd_compr_metadata , snd_soc_dai,
    pub ): *mut snd_soc_dai,
    pub dai): *mut snd_soc_dai,
    pub ): *mut snd_soc_dai,
}

//
// Digital Audio Interface Driver.
//
// Describes the Digital Audio Interface in terms of its ALSA, DAI and AC97
// operations and capabilities. Codec and platform drivers will register this
// structure for every DAI they have.
//
// This structure covers the clocking, formating and ALSA operations for each
// interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_driver {
// DAI description
    pub name: *const c_char,
    pub id: c_uint,
    pub base: c_uint,
    pub dobj: snd_soc_dobj,
    pub dai_args: *const of_phandle_args,
// ops
    pub ops: *const snd_soc_dai_ops,
    pub cops: *const snd_soc_cdai_ops,
// DAI capabilities
    pub capture: snd_soc_pcm_stream,
    pub playback: snd_soc_pcm_stream,
    pub symmetric_rate:1: c_uint,
    pub symmetric_channels:1: c_uint,
    pub symmetric_sample_bits:1: c_uint,
}

// for Playback/Capture
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai_stream {
    pub widget: *mut snd_soc_dapm_widget,
    pub /: *mut *mut unsigned int active; / usage count,
    pub /: *mut *mut unsigned int tdm_mask; / CODEC TDM slot masks and params (for fixup),
    pub /: *mut *mut *mut void dma_data; / DAI DMA data,
}

//
// Digital Audio Interface runtime data.
//
// Holds runtime data for a DAI.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_soc_dai {
    pub name: *const c_char,
    pub id: c_int,
    pub dev: *mut device,
// driver ops
    pub driver: *mut snd_soc_dai_driver,
// DAI runtime info
    pub 1]: snd_soc_dai_stream stream[SNDRV_PCM_STREAM_LAST +,
// Symmetry data - only valid if symmetry is being enforced
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
    pub symmetric_sample_bits: c_uint,
// shared BCLK clock for cross-DAI rate constraints
    pub bclk: *mut clk,
    pub /: *mut *mut *mut *mut unsigned int bclk_ratio; / BCLK = rate  bclk_ratio (0 = use channels  sample_bits),
// parent platform/codec
    pub component: *mut snd_soc_component,
    pub list: list_head,
// function mark
    pub mark_startup: *mut snd_pcm_substream,
    pub mark_hw_params: *mut snd_pcm_substream,
    pub mark_trigger: *mut snd_pcm_substream,
    pub mark_compr_startup: *mut snd_compr_stream,
// bit field
    pub probed:1: c_uint,
// DAI private data
    pub priv: *mut c_void,
}

// see snd_soc_dai_action() for setup
extern "C" {
    pub fn dev_get_drvdata(_arg: dai->dev) -> return;
}
//
// snd_soc_dai_set_stream() - Configures a DAI for stream operation
// @dai: DAI
// @stream: STREAM (opaque structure depending on DAI type)
// @direction: Stream direction(Playback/Capture)
// Some subsystems, such as SoundWire, don't have a notion of direction and we reuse
// the ASoC stream direction to configure sink/source ports.
// Playback maps to source ports and Capture for sink ports.
//
// This should be invoked with NULL to clear the stream set previously.
// Returns 0 on success, a negative error code otherwise.
//
// snd_soc_dai_get_stream() - Retrieves stream from DAI
// @dai: DAI
// @direction: Stream direction(Playback/Capture)
//
// This routine only retrieves that was previously configured
// with snd_soc_dai_get_stream()
//
// Returns pointer to stream or an ERR_PTR value, e.g.
// ERR_PTR(-ENOTSUPP) if callback is not supported;
//
extern "C" {
    pub fn ERR_PTR(_arg: -ENOTSUPP) -> return;
}
