//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mxl692_defs.h
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
// Driver for the MaxLinear MxL69x family of combo tuners/demods
//
// Copyright (C) 2020 Brad Love <brad@nextdimension.cc>
//
// based on code:
// Copyright (c) 2016 MaxLinear, Inc. All rights reserved
// which was released under GPL V2
//
// Defines
//
pub const MXL_EAGLE_HOST_MSG_HEADER_SIZE: c_int = 8;
pub const MXL_EAGLE_FW_MAX_SIZE_IN_KB: c_int = 76;
pub const MXL_EAGLE_QAM_FFE_TAPS_LENGTH: c_int = 16;
pub const MXL_EAGLE_QAM_SPUR_TAPS_LENGTH: c_int = 32;
pub const MXL_EAGLE_QAM_DFE_TAPS_LENGTH: c_int = 72;
pub const MXL_EAGLE_ATSC_FFE_TAPS_LENGTH: c_int = 4096;
pub const MXL_EAGLE_ATSC_DFE_TAPS_LENGTH: c_int = 384;

pub const MXL_EAGLE_FW_LOAD_TIME: c_int = 50;
pub const MXL_EAGLE_FW_MAX_SIZE_IN_KB: c_int = 76;
pub const MXL_EAGLE_FW_HEADER_SIZE: c_int = 16;
pub const MXL_EAGLE_FW_SEGMENT_HEADER_SIZE: c_int = 8;
pub const MXL_EAGLE_MAX_I2C_PACKET_SIZE: c_int = 58;
pub const MXL_EAGLE_I2C_MHEADER_SIZE: c_int = 6;
pub const MXL_EAGLE_I2C_PHEADER_SIZE: c_int = 2;
// Enum of Eagle family devices
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_DEVICE_E {
    MXL_EAGLE_DEVICE_691 = 1,    /* Device Mxl691 */
    MXL_EAGLE_DEVICE_248 = 2,    /* Device Mxl248 */
    MXL_EAGLE_DEVICE_692 = 3,    /* Device Mxl692 */
    MXL_EAGLE_DEVICE_MAX,        /* No such device */
}

pub const VER_A: c_int = 1;
pub const VER_B: c_int = 1;
pub const VER_C: c_int = 1;
pub const VER_D: c_int = 3;
pub const VER_E: c_int = 6;
// Enum of Host to Eagle I2C protocol opcodes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_OPCODE_E {
// DEVICE
    MXL_EAGLE_OPCODE_DEVICE_DEMODULATOR_TYPE_SET,
    MXL_EAGLE_OPCODE_DEVICE_MPEG_OUT_PARAMS_SET,
    MXL_EAGLE_OPCODE_DEVICE_POWERMODE_SET,
    MXL_EAGLE_OPCODE_DEVICE_GPIO_DIRECTION_SET,
    MXL_EAGLE_OPCODE_DEVICE_GPO_LEVEL_SET,
    MXL_EAGLE_OPCODE_DEVICE_INTR_MASK_SET,
    MXL_EAGLE_OPCODE_DEVICE_IO_MUX_SET,
    MXL_EAGLE_OPCODE_DEVICE_VERSION_GET,
    MXL_EAGLE_OPCODE_DEVICE_STATUS_GET,
    MXL_EAGLE_OPCODE_DEVICE_GPI_LEVEL_GET,

// TUNER
    MXL_EAGLE_OPCODE_TUNER_CHANNEL_TUNE_SET,
    MXL_EAGLE_OPCODE_TUNER_LOCK_STATUS_GET,
    MXL_EAGLE_OPCODE_TUNER_AGC_STATUS_GET,

// ATSC
    MXL_EAGLE_OPCODE_ATSC_INIT_SET,
    MXL_EAGLE_OPCODE_ATSC_ACQUIRE_CARRIER_SET,
    MXL_EAGLE_OPCODE_ATSC_STATUS_GET,
    MXL_EAGLE_OPCODE_ATSC_ERROR_COUNTERS_GET,
    MXL_EAGLE_OPCODE_ATSC_EQUALIZER_FILTER_DFE_TAPS_GET,
    MXL_EAGLE_OPCODE_ATSC_EQUALIZER_FILTER_FFE_TAPS_GET,

// QAM
    MXL_EAGLE_OPCODE_QAM_PARAMS_SET,
    MXL_EAGLE_OPCODE_QAM_RESTART_SET,
    MXL_EAGLE_OPCODE_QAM_STATUS_GET,
    MXL_EAGLE_OPCODE_QAM_ERROR_COUNTERS_GET,
    MXL_EAGLE_OPCODE_QAM_CONSTELLATION_VALUE_GET,
    MXL_EAGLE_OPCODE_QAM_EQUALIZER_FILTER_FFE_GET,
    MXL_EAGLE_OPCODE_QAM_EQUALIZER_FILTER_SPUR_START_GET,
    MXL_EAGLE_OPCODE_QAM_EQUALIZER_FILTER_SPUR_END_GET,
    MXL_EAGLE_OPCODE_QAM_EQUALIZER_FILTER_DFE_TAPS_NUMBER_GET,
    MXL_EAGLE_OPCODE_QAM_EQUALIZER_FILTER_DFE_START_GET,
    MXL_EAGLE_OPCODE_QAM_EQUALIZER_FILTER_DFE_MIDDLE_GET,
    MXL_EAGLE_OPCODE_QAM_EQUALIZER_FILTER_DFE_END_GET,

// OOB
    MXL_EAGLE_OPCODE_OOB_PARAMS_SET,
    MXL_EAGLE_OPCODE_OOB_RESTART_SET,
    MXL_EAGLE_OPCODE_OOB_ERROR_COUNTERS_GET,
    MXL_EAGLE_OPCODE_OOB_STATUS_GET,

// SMA
    MXL_EAGLE_OPCODE_SMA_INIT_SET,
    MXL_EAGLE_OPCODE_SMA_PARAMS_SET,
    MXL_EAGLE_OPCODE_SMA_TRANSMIT_SET,
    MXL_EAGLE_OPCODE_SMA_RECEIVE_GET,

// DEBUG
    MXL_EAGLE_OPCODE_INTERNAL,

    MXL_EAGLE_OPCODE_MAX = 70,
}

// Enum of Host to Eagle I2C protocol opcodes
// DEVICE
// TUNER
// ATSC
// QAM
// OOB
// SMA
// DEBUG
// Enum of Callabck function types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_CB_TYPE_E {
    MXL_EAGLE_CB_FW_DOWNLOAD = 0,
}

// Enum of power supply types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_POWER_SUPPLY_SOURCE_E {
    MXL_EAGLE_POWER_SUPPLY_SOURCE_SINGLE,   /* Single supply of 3.3V */
    MXL_EAGLE_POWER_SUPPLY_SOURCE_DUAL,     /* Dual supply, 1.8V & 3.3V */
}

// Enum of I/O pad drive modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_IO_MUX_DRIVE_MODE_E {
    MXL_EAGLE_IO_MUX_DRIVE_MODE_1X,
    MXL_EAGLE_IO_MUX_DRIVE_MODE_2X,
    MXL_EAGLE_IO_MUX_DRIVE_MODE_3X,
    MXL_EAGLE_IO_MUX_DRIVE_MODE_4X,
    MXL_EAGLE_IO_MUX_DRIVE_MODE_5X,
    MXL_EAGLE_IO_MUX_DRIVE_MODE_6X,
    MXL_EAGLE_IO_MUX_DRIVE_MODE_7X,
    MXL_EAGLE_IO_MUX_DRIVE_MODE_8X,
}

// Enum of demodulator types. Used for selection of demodulator
// type in relevant devices, e.g. ATSC vs. QAM in Mxl691
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_DEMOD_TYPE_E {
    MXL_EAGLE_DEMOD_TYPE_QAM,    /* Mxl248 or Mxl692 */
    MXL_EAGLE_DEMOD_TYPE_OOB,    /* Mxl248 only */
    MXL_EAGLE_DEMOD_TYPE_ATSC    /* Mxl691 or Mxl692 */
}

// Enum of power modes. Used for initial
// activation, or for activating sleep mode
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_POWER_MODE_E {
    MXL_EAGLE_POWER_MODE_SLEEP,
    MXL_EAGLE_POWER_MODE_ACTIVE
}

// Enum of GPIOs, used in device GPIO APIs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_GPIO_NUMBER_E {
    MXL_EAGLE_GPIO_NUMBER_0,
    MXL_EAGLE_GPIO_NUMBER_1,
    MXL_EAGLE_GPIO_NUMBER_2,
    MXL_EAGLE_GPIO_NUMBER_3,
    MXL_EAGLE_GPIO_NUMBER_4,
    MXL_EAGLE_GPIO_NUMBER_5,
    MXL_EAGLE_GPIO_NUMBER_6
}

// Enum of GPIO directions, used in GPIO direction configuration API
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_GPIO_DIRECTION_E {
    MXL_EAGLE_GPIO_DIRECTION_INPUT,
    MXL_EAGLE_GPIO_DIRECTION_OUTPUT
}

// Enum of GPIO level, used in device GPIO APIs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_GPIO_LEVEL_E {
    MXL_EAGLE_GPIO_LEVEL_LOW,
    MXL_EAGLE_GPIO_LEVEL_HIGH,
}

// Enum of I/O Mux function, used in device I/O mux configuration API
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_IOMUX_FUNCTION_E {
    MXL_EAGLE_IOMUX_FUNC_FEC_LOCK,
    MXL_EAGLE_IOMUX_FUNC_MERR,
}

// Enum of MPEG Data format, used in MPEG and OOB output configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_MPEG_DATA_FORMAT_E {
    MXL_EAGLE_DATA_SERIAL_LSB_1ST = 0,
    MXL_EAGLE_DATA_SERIAL_MSB_1ST,

    MXL_EAGLE_DATA_SYNC_WIDTH_BIT = 0,
    MXL_EAGLE_DATA_SYNC_WIDTH_BYTE
}

// Enum of MPEG Clock format, used in MPEG and OOB output configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_MPEG_CLOCK_FORMAT_E {
    MXL_EAGLE_CLOCK_ACTIVE_HIGH = 0,
    MXL_EAGLE_CLOCK_ACTIVE_LOW,

    MXL_EAGLE_CLOCK_POSITIVE  = 0,
    MXL_EAGLE_CLOCK_NEGATIVE,

    MXL_EAGLE_CLOCK_IN_PHASE = 0,
    MXL_EAGLE_CLOCK_INVERTED,
}

// Enum of MPEG Clock speeds, used in MPEG output configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_MPEG_CLOCK_RATE_E {
    MXL_EAGLE_MPEG_CLOCK_54MHZ,
    MXL_EAGLE_MPEG_CLOCK_40_5MHZ,
    MXL_EAGLE_MPEG_CLOCK_27MHZ,
    MXL_EAGLE_MPEG_CLOCK_13_5MHZ,
}

// Enum of Interrupt mask bit, used in host interrupt configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_INTR_MASK_BITS_E {
    MXL_EAGLE_INTR_MASK_DEMOD = 0,
    MXL_EAGLE_INTR_MASK_SMA_RX = 1,
    MXL_EAGLE_INTR_MASK_WDOG = 31
}

// Enum of QAM Demodulator type, used in QAM configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_QAM_DEMOD_ANNEX_TYPE_E {
    MXL_EAGLE_QAM_DEMOD_ANNEX_B,    /* J.83B */
    MXL_EAGLE_QAM_DEMOD_ANNEX_A,    /* DVB-C */
}

// Enum of QAM Demodulator modulation, used in QAM configuration and status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_QAM_DEMOD_QAM_TYPE_E {
    MXL_EAGLE_QAM_DEMOD_QAM16,
    MXL_EAGLE_QAM_DEMOD_QAM64,
    MXL_EAGLE_QAM_DEMOD_QAM256,
    MXL_EAGLE_QAM_DEMOD_QAM1024,
    MXL_EAGLE_QAM_DEMOD_QAM32,
    MXL_EAGLE_QAM_DEMOD_QAM128,
    MXL_EAGLE_QAM_DEMOD_QPSK,
    MXL_EAGLE_QAM_DEMOD_AUTO,
}

// Enum of Demodulator IQ setup, used in QAM, OOB configuration and status
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_IQ_FLIP_E {
    MXL_EAGLE_DEMOD_IQ_NORMAL,
    MXL_EAGLE_DEMOD_IQ_FLIPPED,
    MXL_EAGLE_DEMOD_IQ_AUTO,
}

// Enum of OOB Demodulator symbol rates, used in OOB configuration
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_OOB_DEMOD_SYMB_RATE_E {
    MXL_EAGLE_OOB_DEMOD_SYMB_RATE_0_772MHZ,  /* ANSI/SCTE 55-2 0.772 MHz */
    MXL_EAGLE_OOB_DEMOD_SYMB_RATE_1_024MHZ,  /* ANSI/SCTE 55-1 1.024 MHz */
    MXL_EAGLE_OOB_DEMOD_SYMB_RATE_1_544MHZ,  /* ANSI/SCTE 55-2 1.544 MHz */
}

// Enum of tuner channel tuning mode
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_TUNER_CHANNEL_TUNE_MODE_E {
    MXL_EAGLE_TUNER_CHANNEL_TUNE_MODE_VIEW,    /* Normal "view" mode */
    MXL_EAGLE_TUNER_CHANNEL_TUNE_MODE_SCAN,    /* Fast "scan" mode */
}

// Enum of tuner bandwidth
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_TUNER_BW_E {
    MXL_EAGLE_TUNER_BW_6MHZ,
    MXL_EAGLE_TUNER_BW_7MHZ,
    MXL_EAGLE_TUNER_BW_8MHZ,
}

// Enum of tuner bandwidth
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_EAGLE_JUNCTION_TEMPERATURE_E {
    MXL_EAGLE_JUNCTION_TEMPERATURE_BELOW_0_CELSIUS          = 0,
    MXL_EAGLE_JUNCTION_TEMPERATURE_BETWEEN_0_TO_14_CELSIUS  = 1,
    MXL_EAGLE_JUNCTION_TEMPERATURE_BETWEEN_14_TO_28_CELSIUS = 3,
    MXL_EAGLE_JUNCTION_TEMPERATURE_BETWEEN_28_TO_42_CELSIUS = 2,
    MXL_EAGLE_JUNCTION_TEMPERATURE_BETWEEN_42_TO_57_CELSIUS = 6,
    MXL_EAGLE_JUNCTION_TEMPERATURE_BETWEEN_57_TO_71_CELSIUS = 7,
    MXL_EAGLE_JUNCTION_TEMPERATURE_BETWEEN_71_TO_85_CELSIUS = 5,
    MXL_EAGLE_JUNCTION_TEMPERATURE_ABOVE_85_CELSIUS         = 4,
}

// Struct passed in optional callback used during FW download
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_FW_DOWNLOAD_CB_PAYLOAD_T {
    pub total_len: u32,
    pub downloaded_len: u32,
}

// Struct used of I2C protocol between host and Eagle, internal use only
// Device version information struct
// Xtal configuration struct
// GPIO direction struct, internally used in GPIO configuration API
// GPO level struct, internally used in GPIO configuration API
// Device Status struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_DEV_STATUS_T {
    pub temperature: u8,
    pub demod_type: u8,
    pub power_mode: u8,
    pub cpu_utilization_percent: u8,
}

// Device interrupt configuration struct
// MPEG pad drive parameters, used on MPEG output configuration
// See MXL_EAGLE_IO_MUX_DRIVE_MODE_E
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_MPEG_PAD_DRIVE_T {
    pub pad_drv_mpeg_syn: u8,
    pub pad_drv_mpeg_dat: u8,
    pub pad_drv_mpeg_val: u8,
    pub pad_drv_mpeg_clk: u8,
}

// MPEGOUT parameter struct, used in MPEG output configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_MPEGOUT_PARAMS_T {
    pub mpeg_parallel: u8,
    pub msb_first: u8,
    pub /: *mut *mut u8 mpeg_sync_pulse_width; / See MXL_EAGLE_MPEG_DATA_FORMAT_E,
    pub mpeg_valid_pol: u8,
    pub mpeg_sync_pol: u8,
    pub mpeg_clk_pol: u8,
    pub mpeg3wire_mode_enable: u8,
    pub mpeg_clk_freq: u8,
    pub mpeg_pad_drv: MXL_EAGLE_MPEG_PAD_DRIVE_T,
}

// QAM Demodulator parameters struct, used in QAM params configuration
// QAM Demodulator status
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_QAM_DEMOD_STATUS_T {
    pub annex_type: u8,
    pub qam_type: u8,
    pub iq_flip: u8,
    pub interleaver_depth_i: u8,
    pub interleaver_depth_j: u8,
    pub qam_locked: u8,
    pub fec_locked: u8,
    pub mpeg_locked: u8,
    pub snr_db_tenths: u16,
    pub timing_offset: i16,
    pub carrier_offset_hz: i32,
}

// QAM Demodulator error counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_QAM_DEMOD_ERROR_COUNTERS_T {
    pub corrected_code_words: u32,
    pub uncorrected_code_words: u32,
    pub total_code_words_received: u32,
    pub corrected_bits: u32,
    pub error_mpeg_frames: u32,
    pub mpeg_frames_received: u32,
    pub erasures: u32,
}

// QAM Demodulator constellation point
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_QAM_DEMOD_CONSTELLATION_VAL_T {
    pub i_value: [i16; 12],
    pub q_value: [i16; 12],
}

// QAM Demodulator equalizer filter taps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_QAM_DEMOD_EQU_FILTER_T {
    pub ffe_taps: [i16; MXL_EAGLE_QAM_FFE_TAPS_LENGTH],
    pub spur_taps: [i16; MXL_EAGLE_QAM_SPUR_TAPS_LENGTH],
    pub dfe_taps: [i16; MXL_EAGLE_QAM_DFE_TAPS_LENGTH],
    pub ffe_leading_tap_index: u8,
    pub dfe_taps_number: u8,
}

// OOB Demodulator parameters struct, used in OOB params configuration
// OOB Demodulator error counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_OOB_DEMOD_ERROR_COUNTERS_T {
    pub corrected_packets: u32,
    pub uncorrected_packets: u32,
    pub total_packets_received: u32,
}

// OOB status
// ATSC Demodulator status
// ATSC Demodulator error counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_EAGLE_ATSC_DEMOD_ERROR_COUNTERS_T {
    pub error_packets: u32,
    pub total_packets: u32,
    pub error_bytes: u32,
}

// ATSC Demodulator equalizers filter taps
// Tuner AGC Status
// Tuner channel tune parameters
// Tuner channel lock indications
// Smart antenna parameters  used in Smart antenna params configuration
// Smart antenna message format
