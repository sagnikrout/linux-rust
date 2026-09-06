//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sof/dai-intel.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
//
// This file is provided under a dual BSD/GPLv2 license.  When using or
// redistributing this file, you may do so under either license.
//
// Copyright(c) 2018 Intel Corporation
//

// ssc1: TINTE

// ssc1: PINTE

// ssc2: SMTATF

// ssc2: MMRATF

// ssc2: PSPSTWFDFD

// ssc2: PSPSRWFDFD

// ssc1: LBM

// here is the possibility to define others aux macros
pub const SOF_DAI_INTEL_SSP_FRAME_PULSE_WIDTH_MAX: c_int = 38;
pub const SOF_DAI_INTEL_SSP_SLOT_PADDING_MAX: c_int = 31;
// SSP clocks control settings
//
// Macros for clks_control field in sof_ipc_dai_ssp_params struct.
//
// mclk 0 disable

// mclk 1 disable

// mclk keep active

// bclk keep active

// fs keep active

// bclk idle

// mclk early start

// bclk early start

// mclk always on

// DMIC max. four controllers for eight microphone channels
pub const SOF_DAI_INTEL_DMIC_NUM_CTRL: c_int = 4;
// SSP Configuration Request - SOF_IPC_DAI_SSP_CONFIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_ssp_params {
    pub hdr: sof_ipc_hdr,
    pub reserved1: u16,
    pub mclk_id: u16,
    pub /: *mut *mut uint32_t mclk_rate; / mclk frequency in Hz,
    pub /: *mut *mut uint32_t fsync_rate; / fsync frequency in Hz,
    pub /: *mut *mut uint32_t bclk_rate; / bclk frequency in Hz,
// TDM
    pub tdm_slots: u32,
    pub rx_slots: u32,
    pub tx_slots: u32,
// data
    pub sample_valid_bits: u32,
    pub tdm_slot_width: u16,
    pub /: *mut *mut uint16_t reserved2; / alignment,
// MCLK
    pub mclk_direction: u32,
    pub frame_pulse_width: u16,
    pub tdm_per_slot_padding_flag: u16,
    pub clks_control: u32,
    pub quirks: u32,
    pub BCLK: *mut *mut uint32_t bclk_delay; / guaranteed time (ms) for which,
// will be driven, before sending data
//
    pub __packed: },
// HDA Configuration Request - SOF_IPC_DAI_HDA_CONFIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_hda_params {
    pub hdr: sof_ipc_hdr,
    pub link_dma_ch: u32,
    pub rate: u32,
    pub channels: u32,
    pub __packed: },
// ALH Configuration Request - SOF_IPC_DAI_ALH_CONFIG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_alh_params {
    pub hdr: sof_ipc_hdr,
    pub stream_id: u32,
    pub rate: u32,
    pub channels: u32,
// reserved for future use
    pub reserved: [u32; 13],
    pub __packed: },
// DMIC Configuration Request - SOF_IPC_DAI_DMIC_CONFIG
// This struct is defined per 2ch PDM controller available in the platform.
// Normally it is sufficient to set the used microphone specific enables to 1
// and keep other parameters as zero. The customizations are:
//
// 1. If a device mixes different microphones types with different polarity
// and/or the absolute polarity matters the PCM signal from a microphone
// can be inverted with the controls.
//
// 2. If the microphones in a stereo pair do not appear in captured stream
// in desired order due to board schematics choises they can be swapped with
// the clk_edge parameter.
//
// 3. If PDM bit errors are seen in capture (poor quality) the skew parameter
// that delays the sampling time of data by half cycles of DMIC source clock
// can be tried for improvement. However there is no guarantee for this to fix
// data integrity problems.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_dmic_pdm_ctrl {
    pub hdr: sof_ipc_hdr,
    pub /: *mut *mut *mut uint16_t id; /< PDM controller ID,
    pub 1)*/: *mut *mut *mut uint16_t enable_mic_a; /< Use A (left) channel mic (0 or,
    pub 1)*/: *mut *mut *mut uint16_t enable_mic_b; /< Use B (right) channel mic (0 or,
    pub /: *mut *mut *mut uint16_t polarity_mic_a; /< Optionally invert mic A signal (0 or 1),
    pub /: *mut *mut *mut uint16_t polarity_mic_b; /< Optionally invert mic B signal (0 or 1),
    pub /: *mut *mut *mut uint16_t clk_edge; /< Optionally swap data clock edge (0 or 1),
    pub /: *mut *mut *mut uint16_t skew; /< Adjust PDM data sampling vs. clock (0..15),
    pub /: *mut *mut *mut uint16_t reserved[3]; /< Make sure the total size is 4 bytes aligned,
    pub __packed: },
// This struct contains the global settings for all 2ch PDM controllers. The
// version number used in configuration data is checked vs. version used by
// device driver src/drivers/dmic.c need to match. It is incremented from
// initial value 1 if updates done for the to driver would alter the operation
// of the microphone.
//
// Note: The microphone clock (pdmclk_min, pdmclk_max, duty_min, duty_max)
// parameters need to be set as defined in microphone data sheet. E.g. clock
// range 1.0 - 3.2 MHz is usually supported microphones. Some microphones are
// multi-mode capable and there may be denied mic clock frequencies between
// the modes. In such case set the clock range limits of the desired mode to
// avoid the driver to set clock to an illegal rate.
//
// The duty cycle could be set to 48-52% if not known. Generally these
// parameters can be altered within data sheet specified limits to match
// required audio application performance power.
//
// The microphone clock needs to be usually about 50-80 times the used audio
// sample rate. With highest sample rates above 48 kHz this can relaxed
// somewhat.
//
// The parameter wake_up_time describes how long time the microphone needs
// for the data line to produce valid output from mic clock start. The driver
// will mute the captured audio for the given time. The min_clock_on_time
// parameter is used to prevent too short clock bursts to happen. The driver
// will keep the clock active after capture stop if this time is not yet
// met. The unit for both is microseconds (us). Exceed of 100 ms will be
// treated as an error.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_ipc_dai_dmic_params {
    pub hdr: sof_ipc_hdr,
    pub /: *mut *mut *mut uint32_t driver_ipc_version; /< Version (1..N),
    pub /: *mut *mut *mut uint32_t pdmclk_min; /< Minimum microphone clock in Hz (100000..N),
    pub /: *mut *mut *mut uint32_t pdmclk_max; /< Maximum microphone clock in Hz (min...N),
    pub /: *mut *mut *mut uint32_t fifo_fs; /< FIFO sample rate in Hz (8000..96000),
    pub /: *mut *mut *mut uint32_t reserved_1; /< Reserved,
    pub /: *mut *mut *mut uint16_t fifo_bits; /< FIFO word length (16 or 32),
    pub /: *mut *mut *mut uint16_t fifo_bits_b; /< Deprecated since firmware ABI 3.0.1,
    pub /: *mut *mut *mut uint16_t duty_min; /< Min. mic clock duty cycle in % (20..80),
    pub /: *mut *mut *mut uint16_t duty_max; /< Max. mic clock duty cycle in % (min..80),
    pub /: *mut *mut *mut uint32_t num_pdm_active; /< Number of active pdm controllers.,
// < Range is 1..SOF_DAI_INTEL_DMIC_NUM_CTRL
    pub /: *mut *mut *mut uint32_t wake_up_time; /< Time from clock start to data (us),
    pub /: *mut *mut *mut uint32_t min_clock_on_time; /< Min. time that clk is kept on (us),
    pub /: *mut *mut *mut uint32_t unmute_ramp_time; /< Length of logarithmic gain ramp (ms),
// reserved for future use
    pub reserved: [u32; 5],
// < PDM controllers configuration
    pub pdm: [sof_ipc_dai_dmic_pdm_ctrl; SOF_DAI_INTEL_DMIC_NUM_CTRL],
    pub __packed: },
