//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/dvb-frontends/mxl5xx_defs.h
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
// Defines for the Maxlinear MX58x family of tuners/demods
//
// Copyright (C) 2014 Digital Devices GmbH
//
// based on code:
// Copyright (c) 2011-2013 MaxLinear, Inc. All rights reserved
// which was released under GPL V2
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_BOOL_E {
    MXL_DISABLE = 0,
    MXL_ENABLE  = 1,

    MXL_FALSE = 0,
    MXL_TRUE  = 1,

    MXL_INVALID = 0,
    MXL_VALID   = 1,

    MXL_NO      = 0,
    MXL_YES     = 1,

    MXL_OFF     = 0,
    MXL_ON      = 1
}

// Firmware-Host Command IDs
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_HOST_CMD_ID_E {
// --Device command IDs--
    MXL_HYDRA_DEV_NO_OP_CMD = 0, /* No OP */

    MXL_HYDRA_DEV_SET_POWER_MODE_CMD = 1,
    MXL_HYDRA_DEV_SET_OVERWRITE_DEF_CMD = 2,

// Host-used CMD, not used by firmware
    MXL_HYDRA_DEV_FIRMWARE_DOWNLOAD_CMD = 3,

// Additional CONTROL types from DTV
    MXL_HYDRA_DEV_SET_BROADCAST_PID_STB_ID_CMD = 4,
    MXL_HYDRA_DEV_GET_PMM_SLEEP_CMD = 5,

// --Tuner command IDs--
    MXL_HYDRA_TUNER_TUNE_CMD = 6,
    MXL_HYDRA_TUNER_GET_STATUS_CMD = 7,

// --Demod command IDs--
    MXL_HYDRA_DEMOD_SET_PARAM_CMD = 8,
    MXL_HYDRA_DEMOD_GET_STATUS_CMD = 9,

    MXL_HYDRA_DEMOD_RESET_FEC_COUNTER_CMD = 10,

    MXL_HYDRA_DEMOD_SET_PKT_NUM_CMD = 11,

    MXL_HYDRA_DEMOD_SET_IQ_SOURCE_CMD = 12,
    MXL_HYDRA_DEMOD_GET_IQ_DATA_CMD = 13,

    MXL_HYDRA_DEMOD_GET_M68HC05_VER_CMD = 14,

    MXL_HYDRA_DEMOD_SET_ERROR_COUNTER_MODE_CMD = 15,

// --- ABORT channel tune
    MXL_HYDRA_ABORT_TUNE_CMD = 16, /* Abort current tune command. */

// --SWM/FSK command IDs--
    MXL_HYDRA_FSK_RESET_CMD = 17,
    MXL_HYDRA_FSK_MSG_CMD = 18,
    MXL_HYDRA_FSK_SET_OP_MODE_CMD = 19,

// --DiSeqC command IDs--
    MXL_HYDRA_DISEQC_MSG_CMD = 20,
    MXL_HYDRA_DISEQC_COPY_MSG_TO_MAILBOX = 21,
    MXL_HYDRA_DISEQC_CFG_MSG_CMD = 22,

// --- FFT Debug Command IDs--
    MXL_HYDRA_REQ_FFT_SPECTRUM_CMD = 23,

// -- Demod scramblle code
    MXL_HYDRA_DEMOD_SCRAMBLE_CODE_CMD = 24,

// ---For host to know how many commands in total
    MXL_HYDRA_LAST_HOST_CMD = 25,

    MXL_HYDRA_DEMOD_INTR_TYPE_CMD = 47,
    MXL_HYDRA_DEV_INTR_CLEAR_CMD = 48,
    MXL_HYDRA_TUNER_SPECTRUM_REQ_CMD = 53,
    MXL_HYDRA_TUNER_ACTIVATE_CMD = 55,
    MXL_HYDRA_DEV_CFG_POWER_MODE_CMD = 56,
    MXL_HYDRA_DEV_XTAL_CAP_CMD = 57,
    MXL_HYDRA_DEV_CFG_SKU_CMD = 58,
    MXL_HYDRA_TUNER_SPECTRUM_MIN_GAIN_CMD = 59,
    MXL_HYDRA_DISEQC_CONT_TONE_CFG = 60,
    MXL_HYDRA_DEV_RF_WAKE_UP_CMD = 61,
    MXL_HYDRA_DEMOD_CFG_EQ_CTRL_PARAM_CMD = 62,
    MXL_HYDRA_DEMOD_FREQ_OFFSET_SEARCH_RANGE_CMD = 63,
    MXL_HYDRA_DEV_REQ_PWR_FROM_ADCRSSI_CMD = 64,

    MXL_XCPU_PID_FLT_CFG_CMD = 65,
    MXL_XCPU_SHMEM_TEST_CMD = 66,
    MXL_XCPU_ABORT_TUNE_CMD = 67,
    MXL_XCPU_CHAN_TUNE_CMD = 68,
    MXL_XCPU_FLT_BOND_HDRS_CMD = 69,

    MXL_HYDRA_DEV_BROADCAST_WAKE_UP_CMD = 70,
    MXL_HYDRA_FSK_CFG_FSK_FREQ_CMD = 71,
    MXL_HYDRA_FSK_POWER_DOWN_CMD = 72,
    MXL_XCPU_CLEAR_CB_STATS_CMD = 73,
    MXL_XCPU_CHAN_BOND_RESTART_CMD = 74
}

pub const MXL_HYDRA_OEM_MAX_BLOCK_WRITE_LENGTH: c_int = 248;

pub const MXL_HYDRA_CAP_MIN: c_int = 10;
pub const MXL_HYDRA_CAP_MAX: c_int = 33;
pub const MXL_HYDRA_PLID_REG_READ: c_uint = 0xFB   /* Read register PLID */;
pub const MXL_HYDRA_PLID_REG_WRITE: c_uint = 0xFC   /* Write register PLID */;
pub const MXL_HYDRA_PLID_CMD_READ: c_uint = 0xFD   /* Command Read PLID */;
pub const MXL_HYDRA_PLID_CMD_WRITE: c_uint = 0xFE   /* Command Write PLID */;

pub const MXL_HYDRA_SKU_ID_581: c_int = 0;
pub const MXL_HYDRA_SKU_ID_584: c_int = 1;
pub const MXL_HYDRA_SKU_ID_585: c_int = 2;
pub const MXL_HYDRA_SKU_ID_544: c_int = 3;
pub const MXL_HYDRA_SKU_ID_561: c_int = 4;
pub const MXL_HYDRA_SKU_ID_582: c_int = 5;
pub const MXL_HYDRA_SKU_ID_568: c_int = 6;
// macro for register write data buffer size
// (PLID + LEN (0xFF) + RegAddr + RegData)
//

// macro to extract a single byte from 4-byte(32-bit) data

pub const MAX_CMD_DATA: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MBIN_FILE_HEADER_T {
    pub id: u8,
    pub fmt_version: u8,
    pub header_len: u8,
    pub num_segments: u8,
    pub entry_address: [u8; 4],
    pub image_size24: [u8; 3],
    pub image_checksum: u8,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MBIN_FILE_T {
    pub header: MBIN_FILE_HEADER_T,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MBIN_SEGMENT_HEADER_T {
    pub id: u8,
    pub len24: [u8; 3],
    pub address: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MBIN_SEGMENT_T {
    pub header: MBIN_SEGMENT_HEADER_T,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_CMD_TYPE_E {

    do {								\
    cmd_buff[0] = ((req_type == MXL_CMD_WRITE) ? MXL_HYDRA_PLID_CMD_WRITE : MXL_HYDRA_PLID_CMD_READ); \
    cmd_buff[1] = (size > 251) ? 0xff : (u8) (size + 4);	\
    cmd_buff[2] = size;					\
    cmd_buff[3] = cmd_id;					\
    cmd_buff[4] = 0x00;					\
    cmd_buff[5] = 0x00;					\
    convert_endian(MXL_ENABLE_BIG_ENDIAN, size, (u8 *)data_ptr); \
    memcpy((void *)&cmd_buff[6], data_ptr, size);		\
    } while (0)

    struct MXL_REG_FIELD_T {
    u32 reg_addr;
    u8 lsb_pos;
    u8 num_of_bits;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_DEV_CMD_DATA_T {
    pub data_size: u32,
    pub data: [u8; MAX_CMD_DATA],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_SKU_TYPE_E {
    MXL_HYDRA_SKU_TYPE_MIN = 0x00,
    MXL_HYDRA_SKU_TYPE_581 = 0x00,
    MXL_HYDRA_SKU_TYPE_584 = 0x01,
    MXL_HYDRA_SKU_TYPE_585 = 0x02,
    MXL_HYDRA_SKU_TYPE_544 = 0x03,
    MXL_HYDRA_SKU_TYPE_561 = 0x04,
    MXL_HYDRA_SKU_TYPE_5XX = 0x05,
    MXL_HYDRA_SKU_TYPE_5YY = 0x06,
    MXL_HYDRA_SKU_TYPE_511 = 0x07,
    MXL_HYDRA_SKU_TYPE_561_DE = 0x08,
    MXL_HYDRA_SKU_TYPE_582 = 0x09,
    MXL_HYDRA_SKU_TYPE_541 = 0x0A,
    MXL_HYDRA_SKU_TYPE_568 = 0x0B,
    MXL_HYDRA_SKU_TYPE_542 = 0x0C,
    MXL_HYDRA_SKU_TYPE_MAX = 0x0D,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_SKU_COMMAND_T {
    pub sku_type: MXL_HYDRA_SKU_TYPE_E,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_DEMOD_ID_E {
    MXL_HYDRA_DEMOD_ID_0 = 0,
    MXL_HYDRA_DEMOD_ID_1,
    MXL_HYDRA_DEMOD_ID_2,
    MXL_HYDRA_DEMOD_ID_3,
    MXL_HYDRA_DEMOD_ID_4,
    MXL_HYDRA_DEMOD_ID_5,
    MXL_HYDRA_DEMOD_ID_6,
    MXL_HYDRA_DEMOD_ID_7,
    MXL_HYDRA_DEMOD_MAX
}

pub const MXL_DEMOD_SCRAMBLE_SEQ_LEN: c_int = 12;
pub const MAX_STEP_SIZE_24_XTAL_102_05_KHZ: c_int = 195;
pub const MAX_STEP_SIZE_24_XTAL_204_10_KHZ: c_int = 215;
pub const MAX_STEP_SIZE_24_XTAL_306_15_KHZ: c_int = 203;
pub const MAX_STEP_SIZE_24_XTAL_408_20_KHZ: c_int = 177;
pub const MAX_STEP_SIZE_27_XTAL_102_05_KHZ: c_int = 195;
pub const MAX_STEP_SIZE_27_XTAL_204_10_KHZ: c_int = 215;
pub const MAX_STEP_SIZE_27_XTAL_306_15_KHZ: c_int = 203;
pub const MAX_STEP_SIZE_27_XTAL_408_20_KHZ: c_int = 177;
pub const MXL_HYDRA_SPECTRUM_MIN_FREQ_KHZ: c_int = 300000;
pub const MXL_HYDRA_SPECTRUM_MAX_FREQ_KHZ: c_int = 2350000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_DEMOD_CHAN_PARAMS_OFFSET_E {
    DMD_STANDARD_ADDR = 0,
    DMD_SPECTRUM_INVERSION_ADDR,
    DMD_SPECTRUM_ROLL_OFF_ADDR,
    DMD_SYMBOL_RATE_ADDR,
    DMD_MODULATION_SCHEME_ADDR,
    DMD_FEC_CODE_RATE_ADDR,
    DMD_SNR_ADDR,
    DMD_FREQ_OFFSET_ADDR,
    DMD_CTL_FREQ_OFFSET_ADDR,
    DMD_STR_FREQ_OFFSET_ADDR,
    DMD_FTL_FREQ_OFFSET_ADDR,
    DMD_STR_NBC_SYNC_LOCK_ADDR,
    DMD_CYCLE_SLIP_COUNT_ADDR,
    DMD_DISPLAY_IQ_ADDR,
    DMD_DVBS2_CRC_ERRORS_ADDR,
    DMD_DVBS2_PER_COUNT_ADDR,
    DMD_DVBS2_PER_WINDOW_ADDR,
    DMD_DVBS_CORR_RS_ERRORS_ADDR,
    DMD_DVBS_UNCORR_RS_ERRORS_ADDR,
    DMD_DVBS_BER_COUNT_ADDR,
    DMD_DVBS_BER_WINDOW_ADDR,
    DMD_TUNER_ID_ADDR,
    DMD_DVBS2_PILOT_ON_OFF_ADDR,
    DMD_FREQ_SEARCH_RANGE_IN_KHZ_ADDR,

    MXL_DEMOD_CHAN_PARAMS_BUFF_SIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TUNER_ID_E {
    MXL_HYDRA_TUNER_ID_0 = 0,
    MXL_HYDRA_TUNER_ID_1,
    MXL_HYDRA_TUNER_ID_2,
    MXL_HYDRA_TUNER_ID_3,
    MXL_HYDRA_TUNER_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_BCAST_STD_E {
    MXL_HYDRA_DSS = 0,
    MXL_HYDRA_DVBS,
    MXL_HYDRA_DVBS2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_FEC_E {
    MXL_HYDRA_FEC_AUTO = 0,
    MXL_HYDRA_FEC_1_2,
    MXL_HYDRA_FEC_3_5,
    MXL_HYDRA_FEC_2_3,
    MXL_HYDRA_FEC_3_4,
    MXL_HYDRA_FEC_4_5,
    MXL_HYDRA_FEC_5_6,
    MXL_HYDRA_FEC_6_7,
    MXL_HYDRA_FEC_7_8,
    MXL_HYDRA_FEC_8_9,
    MXL_HYDRA_FEC_9_10,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_MODULATION_E {
    MXL_HYDRA_MOD_AUTO = 0,
    MXL_HYDRA_MOD_QPSK,
    MXL_HYDRA_MOD_8PSK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_SPECTRUM_E {
    MXL_HYDRA_SPECTRUM_AUTO = 0,
    MXL_HYDRA_SPECTRUM_INVERTED,
    MXL_HYDRA_SPECTRUM_NON_INVERTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_ROLLOFF_E {
    MXL_HYDRA_ROLLOFF_AUTO  = 0,
    MXL_HYDRA_ROLLOFF_0_20,
    MXL_HYDRA_ROLLOFF_0_25,
    MXL_HYDRA_ROLLOFF_0_35
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_PILOTS_E {
    MXL_HYDRA_PILOTS_OFF  = 0,
    MXL_HYDRA_PILOTS_ON,
    MXL_HYDRA_PILOTS_AUTO
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_CONSTELLATION_SRC_E {
    MXL_HYDRA_FORMATTER = 0,
    MXL_HYDRA_LEGACY_FEC,
    MXL_HYDRA_FREQ_RECOVERY,
    MXL_HYDRA_NBC,
    MXL_HYDRA_CTL,
    MXL_HYDRA_EQ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_LOCK_T {
    pub /: *mut *mut int agc_lock; / AGC lock info,
    pub /: *mut *mut int fec_lock; / Demod FEC block lock info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_STATUS_DVBS_T {
    pub /: *mut *mut u32 rs_errors; / RS decoder err counter,
    pub /: *mut *mut u32 ber_window; / Ber Windows,
    pub /: *mut *mut u32 ber_count; / BER count,
    pub /: *mut *mut u32 ber_window_iter1; / Ber Windows - post viterbi,
    pub /: *mut *mut u32 ber_count_iter1; / BER count - post viterbi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_STATUS_DSS_T {
    pub /: *mut *mut u32 rs_errors; / RS decoder err counter,
    pub /: *mut *mut u32 ber_window; / Ber Windows,
    pub /: *mut *mut u32 ber_count; / BER count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_STATUS_DVBS2_T {
    pub /: *mut *mut u32 crc_errors; / CRC error counter,
    pub /: *mut *mut u32 packet_error_count; / Number of packet errors,
    pub /: *mut *mut u32 total_packets; / Total packets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_STATUS_T {
    pub /: *mut *mut MXL_HYDRA_BCAST_STD_E standard_mask; / Standard DVB-S, DVB-S2 or DSS,
    pub /: *mut *mut MXL_HYDRA_DEMOD_STATUS_DVBS_T demod_status_dvbs; / DVB-S demod status,
    pub /: *mut *mut MXL_HYDRA_DEMOD_STATUS_DVBS2_T demod_status_dvbs2; / DVB-S2 demod status,
    pub /: *mut *mut MXL_HYDRA_DEMOD_STATUS_DSS_T demod_status_dss; / DSS demod status,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_SIG_OFFSET_INFO_T {
    pub /: *mut *mut s32 carrier_offset_in_hz; / CRL offset info,
    pub /: *mut *mut s32 symbol_offset_in_symbol; / SRL offset info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_SCRAMBLE_INFO_T {
    pub /: *mut *mut u8 scramble_sequence[MXL_DEMOD_SCRAMBLE_SEQ_LEN]; / scramble sequence,
    pub /: *mut *mut u32 scramble_code; / scramble gold code,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_SPECTRUM_STEP_SIZE_E {
    MXL_HYDRA_STEP_SIZE_24_XTAL_102_05KHZ, /* 102.05 KHz for 24 MHz XTAL */
    MXL_HYDRA_STEP_SIZE_24_XTAL_204_10KHZ, /* 204.10 KHz for 24 MHz XTAL */
    MXL_HYDRA_STEP_SIZE_24_XTAL_306_15KHZ, /* 306.15 KHz for 24 MHz XTAL */
    MXL_HYDRA_STEP_SIZE_24_XTAL_408_20KHZ, /* 408.20 KHz for 24 MHz XTAL */

    MXL_HYDRA_STEP_SIZE_27_XTAL_102_05KHZ, /* 102.05 KHz for 27 MHz XTAL */
    MXL_HYDRA_STEP_SIZE_27_XTAL_204_35KHZ, /* 204.35 KHz for 27 MHz XTAL */
    MXL_HYDRA_STEP_SIZE_27_XTAL_306_52KHZ, /* 306.52 KHz for 27 MHz XTAL */
    MXL_HYDRA_STEP_SIZE_27_XTAL_408_69KHZ, /* 408.69 KHz for 27 MHz XTAL */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_SPECTRUM_RESOLUTION_E {
    MXL_HYDRA_SPECTRUM_RESOLUTION_00_1_DB, /* 0.1 dB */
    MXL_HYDRA_SPECTRUM_RESOLUTION_01_0_DB, /* 1.0 dB */
    MXL_HYDRA_SPECTRUM_RESOLUTION_05_0_DB, /* 5.0 dB */
    MXL_HYDRA_SPECTRUM_RESOLUTION_10_0_DB, /* 10 dB */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_SPECTRUM_ERROR_CODE_E {
    MXL_SPECTRUM_NO_ERROR,
    MXL_SPECTRUM_INVALID_PARAMETER,
    MXL_SPECTRUM_INVALID_STEP_SIZE,
    MXL_SPECTRUM_BW_CANNOT_BE_COVERED,
    MXL_SPECTRUM_DEMOD_BUSY,
    MXL_SPECTRUM_TUNER_NOT_ENABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_SPECTRUM_REQ_T {
    pub /: *mut *mut u32 tuner_index; / TUNER Ctrl: one of MXL58x_TUNER_ID_E,
    pub /: *mut *mut u32 demod_index; / DEMOD Ctrl: one of MXL58x_DEMOD_ID_E,
    pub step_size_in_khz: MXL_HYDRA_SPECTRUM_STEP_SIZE_E,
    pub starting_freq_ink_hz: u32,
    pub total_steps: u32,
    pub spectrum_division: MXL_HYDRA_SPECTRUM_RESOLUTION_E,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_SEARCH_FREQ_OFFSET_TYPE_E {
    MXL_HYDRA_SEARCH_MAX_OFFSET = 0, /* DMD searches for max freq offset (i.e. 5MHz) */
    MXL_HYDRA_SEARCH_BW_PLUS_ROLLOFF, /* DMD searches for BW + ROLLOFF/2 */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL58X_CFG_FREQ_OFF_SEARCH_RANGE_T {
    pub demod_index: u32,
    pub search_type: MXL_HYDRA_SEARCH_FREQ_OFFSET_TYPE_E,
}

// there are two slices
// slice0 - TS0, TS1, TS2 & TS3
// slice1 - TS4, TS5, TS6 & TS7
//
pub const MXL_HYDRA_TS_SLICE_MAX: c_int = 2;
pub const MAX_FIXED_PID_NUM: c_int = 32;

pub const MXL_HYDRA_TS_FIXED_PID_FILT_SIZE: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_PID_BANK_TYPE_E {
    MXL_HYDRA_SOFTWARE_PID_BANK = 0,
    MXL_HYDRA_HARDWARE_PID_BANK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TS_MUX_MODE_E {
    MXL_HYDRA_TS_MUX_PID_REMAP = 0,
    MXL_HYDRA_TS_MUX_PREFIX_EXTRA_HEADER = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TS_MUX_TYPE_E {
    MXL_HYDRA_TS_MUX_DISABLE = 0, /* No Mux ( 1 TSIF to 1 TSIF) */
    MXL_HYDRA_TS_MUX_2_TO_1, /* Mux 2 TSIF to 1 TSIF */
    MXL_HYDRA_TS_MUX_4_TO_1, /* Mux 4 TSIF to 1 TSIF */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TS_GROUP_E {
    MXL_HYDRA_TS_GROUP_0_3 = 0, /* TS group 0 to 3 (TS0, TS1, TS2 & TS3) */
    MXL_HYDRA_TS_GROUP_4_7,     /* TS group 0 to 3 (TS4, TS5, TS6 & TS7) */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TS_PID_FLT_CTRL_E {
    MXL_HYDRA_TS_PIDS_ALLOW_ALL = 0, /* Allow all pids */
    MXL_HYDRA_TS_PIDS_DROP_ALL,	 /* Drop all pids */
    MXL_HYDRA_TS_INVALIDATE_PID_FILTER, /* Delete current PD filter in the device */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TS_PID_TYPE_E {
    MXL_HYDRA_TS_PID_FIXED = 0,
    MXL_HYDRA_TS_PID_REGULAR,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_TS_PID_T {
    pub /: *mut *mut u16 original_pid; / pid from TS,
    pub /: *mut *mut u16 remapped_pid; / remapped pid,
    pub /: *mut *mut MXL_BOOL_E enable; / enable or disable pid,
    pub /: *mut *mut MXL_BOOL_E allow_or_drop; / allow or drop pid,
    pub /: *mut *mut MXL_BOOL_E enable_pid_remap; / enable or disable pid remap,
    pub /: *mut *mut u8 bond_id; / Bond ID in A0 always 0 - Only for 568 Sku,
    pub /: *mut *mut u8 dest_id; / Output port ID for the PID - Only for 568 Sku,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_TS_MUX_PREFIX_HEADER_T {
    pub enable: MXL_BOOL_E,
    pub num_byte: u8,
    pub header: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_PID_FILTER_BANK_E {
    MXL_HYDRA_PID_BANK_A = 0,
    MXL_HYDRA_PID_BANK_B,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_MPEG_DATA_FMT_E {
    MXL_HYDRA_MPEG_SERIAL_MSB_1ST = 0,
    MXL_HYDRA_MPEG_SERIAL_LSB_1ST,

    MXL_HYDRA_MPEG_SYNC_WIDTH_BIT = 0,
    MXL_HYDRA_MPEG_SYNC_WIDTH_BYTE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_MPEG_MODE_E {
    MXL_HYDRA_MPEG_MODE_SERIAL_4_WIRE = 0, /* MPEG 4 Wire serial mode */
    MXL_HYDRA_MPEG_MODE_SERIAL_3_WIRE,     /* MPEG 3 Wire serial mode */
    MXL_HYDRA_MPEG_MODE_SERIAL_2_WIRE,     /* MPEG 2 Wire serial mode */
    MXL_HYDRA_MPEG_MODE_PARALLEL           /* MPEG parallel mode - valid only for MxL581 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_MPEG_CLK_TYPE_E {
    MXL_HYDRA_MPEG_CLK_CONTINUOUS = 0, /* Continuous MPEG clock */
    MXL_HYDRA_MPEG_CLK_GAPPED,         /* Gapped (gated) MPEG clock */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_MPEG_CLK_FMT_E {
    MXL_HYDRA_MPEG_ACTIVE_LOW = 0,
    MXL_HYDRA_MPEG_ACTIVE_HIGH,

    MXL_HYDRA_MPEG_CLK_NEGATIVE = 0,
    MXL_HYDRA_MPEG_CLK_POSITIVE,

    MXL_HYDRA_MPEG_CLK_IN_PHASE = 0,
    MXL_HYDRA_MPEG_CLK_INVERTED,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_MPEG_CLK_PHASE_E {
    MXL_HYDRA_MPEG_CLK_PHASE_SHIFT_0_DEG = 0,
    MXL_HYDRA_MPEG_CLK_PHASE_SHIFT_90_DEG,
    MXL_HYDRA_MPEG_CLK_PHASE_SHIFT_180_DEG,
    MXL_HYDRA_MPEG_CLK_PHASE_SHIFT_270_DEG
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_MPEG_ERR_INDICATION_E {
    MXL_HYDRA_MPEG_ERR_REPLACE_SYNC = 0,
    MXL_HYDRA_MPEG_ERR_REPLACE_VALID,
    MXL_HYDRA_MPEG_ERR_INDICATION_DISABLED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_MPEGOUT_PARAM_T {
    pub /: *mut *mut int enable; / Enable or Disable MPEG OUT,
    pub /: *mut *mut MXL_HYDRA_MPEG_CLK_TYPE_E mpeg_clk_type; / Continuous or gapped,
    pub /: *mut *mut MXL_HYDRA_MPEG_CLK_FMT_E mpeg_clk_pol; / MPEG Clk polarity,
    pub /: *mut *mut u8 max_mpeg_clk_rate; / Max MPEG Clk rate (0 - 104 MHz, 139 MHz),
    pub /: *mut *mut MXL_HYDRA_MPEG_CLK_PHASE_E mpeg_clk_phase; / MPEG Clk phase,
    pub /: *mut *mut MXL_HYDRA_MPEG_DATA_FMT_E lsb_or_msb_first; / LSB first or MSB first in TS transmission,
    pub /: *mut *mut MXL_HYDRA_MPEG_DATA_FMT_E mpeg_sync_pulse_width; / MPEG SYNC pulse width (1-bit or 1-byte),
    pub /: *mut *mut MXL_HYDRA_MPEG_CLK_FMT_E mpeg_valid_pol; / MPEG VALID polarity,
    pub /: *mut *mut MXL_HYDRA_MPEG_CLK_FMT_E mpeg_sync_pol; / MPEG SYNC polarity,
    pub /: *mut *mut MXL_HYDRA_MPEG_MODE_E mpeg_mode; / config 4/3/2-wire serial or parallel TS out,
    pub /: *mut *mut MXL_HYDRA_MPEG_ERR_INDICATION_E mpeg_error_indication; / Enable or Disable MPEG error indication,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_EXT_TS_IN_ID_E {
    MXL_HYDRA_EXT_TS_IN_0 = 0,
    MXL_HYDRA_EXT_TS_IN_1,
    MXL_HYDRA_EXT_TS_IN_2,
    MXL_HYDRA_EXT_TS_IN_3,
    MXL_HYDRA_EXT_TS_IN_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TS_OUT_ID_E {
    MXL_HYDRA_TS_OUT_0 = 0,
    MXL_HYDRA_TS_OUT_1,
    MXL_HYDRA_TS_OUT_2,
    MXL_HYDRA_TS_OUT_3,
    MXL_HYDRA_TS_OUT_4,
    MXL_HYDRA_TS_OUT_5,
    MXL_HYDRA_TS_OUT_6,
    MXL_HYDRA_TS_OUT_7,
    MXL_HYDRA_TS_OUT_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_TS_DRIVE_STRENGTH_E {
    MXL_HYDRA_TS_DRIVE_STRENGTH_1X = 0,
    MXL_HYDRA_TS_DRIVE_STRENGTH_2X,
    MXL_HYDRA_TS_DRIVE_STRENGTH_3X,
    MXL_HYDRA_TS_DRIVE_STRENGTH_4X,
    MXL_HYDRA_TS_DRIVE_STRENGTH_5X,
    MXL_HYDRA_TS_DRIVE_STRENGTH_6X,
    MXL_HYDRA_TS_DRIVE_STRENGTH_7X,
    MXL_HYDRA_TS_DRIVE_STRENGTH_8X
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_DEVICE_E {
    MXL_HYDRA_DEVICE_581 = 0,
    MXL_HYDRA_DEVICE_584,
    MXL_HYDRA_DEVICE_585,
    MXL_HYDRA_DEVICE_544,
    MXL_HYDRA_DEVICE_561,
    MXL_HYDRA_DEVICE_TEST,
    MXL_HYDRA_DEVICE_582,
    MXL_HYDRA_DEVICE_541,
    MXL_HYDRA_DEVICE_568,
    MXL_HYDRA_DEVICE_542,
    MXL_HYDRA_DEVICE_541S,
    MXL_HYDRA_DEVICE_561S,
    MXL_HYDRA_DEVICE_581S,
    MXL_HYDRA_DEVICE_MAX
}

// Demod IQ data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_IQ_SRC_T {
    pub demod_id: u32,
    pub Formatter: *mut *mut u32 source_of_iq; / == 0, it means I/Q comes from,
// == 1, Legacy FEC
// == 2, Frequency Recovery
// == 3, NBC
// == 4, CTL
// == 5, EQ
// == 6, FPGA
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_ABORT_TUNE_T {
    pub demod_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_TUNER_CMD {
    pub tuner_id: u8,
    pub enable: u8,
}

// Demod Para for Channel Tune
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_PARAM_T {
    pub tuner_index: u32,
    pub demod_index: u32,
    pub /: *mut *mut u32 frequency_in_hz; / Frequency,
    pub /: *mut *mut u32 standard; / one of MXL_HYDRA_BCAST_STD_E,
    pub /: *mut *mut u32 spectrum_inversion; / Input : Spectrum inversion.,
    pub /: *mut *mut u32 roll_off; / rollOff (alpha) factor,
    pub /: *mut *mut u32 symbol_rate_in_hz; / Symbol rate,
    pub /: *mut *mut u32 pilots; / TRUE = pilots enabled,
    pub /: *mut *mut u32 modulation_scheme; / Input : Modulation Scheme is one of MXL_HYDRA_MODULATION_E,
    pub /: *mut *mut u32 fec_code_rate; / Input : Forward error correction rate. Is one of MXL_HYDRA_FEC_E,
    pub /: *mut *mut u32 max_carrier_offset_in_mhz; / Maximum carrier freq offset in MHz. Same as freqSearchRangeKHz, but in unit of MHz.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DEMOD_SCRAMBLE_CODE_T {
    pub demod_index: u32,
    pub /: *mut *mut u8 scramble_sequence[12]; / scramble sequence,
    pub /: *mut *mut u32 scramble_code; / scramble gold code,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_INTR_CFG_T {
    pub intr_type: u32,
    pub intr_duration_in_nano_secs: u32,
    pub intr_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_POWER_MODE_CMD {
    pub /: *mut *mut u8 power_mode; / enumeration values are defined in MXL_HYDRA_PWR_MODE_E (device API.h),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_RF_WAKEUP_PARAM_T {
    pub /: *mut *mut u32 time_interval_in_seconds; / in seconds,
    pub tuner_index: u32,
    pub rssi_threshold: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_RF_WAKEUP_CFG_T {
    pub tuner_count: u32,
    pub params: MXL_HYDRA_RF_WAKEUP_PARAM_T,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_AUX_CTRL_MODE_E {
    MXL_HYDRA_AUX_CTRL_MODE_FSK = 0, /* Select FSK controller */
    MXL_HYDRA_AUX_CTRL_MODE_DISEQC,  /* Select DiSEqC controller */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_DISEQC_OPMODE_E {
    MXL_HYDRA_DISEQC_ENVELOPE_MODE = 0,
    MXL_HYDRA_DISEQC_TONE_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_DISEQC_VER_E {
    MXL_HYDRA_DISEQC_1_X = 0, /* Config DiSEqC 1.x mode */
    MXL_HYDRA_DISEQC_2_X, /* Config DiSEqC 2.x mode */
    MXL_HYDRA_DISEQC_DISABLE /* Disable DiSEqC */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_DISEQC_CARRIER_FREQ_E {
    MXL_HYDRA_DISEQC_CARRIER_FREQ_22KHZ = 0, /* DiSEqC signal frequency of 22 KHz */
    MXL_HYDRA_DISEQC_CARRIER_FREQ_33KHZ,     /* DiSEqC signal frequency of 33 KHz */
    MXL_HYDRA_DISEQC_CARRIER_FREQ_44KHZ      /* DiSEqC signal frequency of 44 KHz */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_DISEQC_ID_E {
    MXL_HYDRA_DISEQC_ID_0 = 0,
    MXL_HYDRA_DISEQC_ID_1,
    MXL_HYDRA_DISEQC_ID_2,
    MXL_HYDRA_DISEQC_ID_3
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MXL_HYDRA_FSK_OP_MODE_E {
    MXL_HYDRA_FSK_CFG_TYPE_39KPBS = 0, /* 39.0kbps */
    MXL_HYDRA_FSK_CFG_TYPE_39_017KPBS, /* 39.017kbps */
    MXL_HYDRA_FSK_CFG_TYPE_115_2KPBS   /* 115.2kbps */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL58X_DSQ_OP_MODE_T {
    pub /: *mut *mut u32 diseqc_id; / DSQ 0, 1, 2 or 3,
    pub /: *mut *mut u32 op_mode; / Envelope mode (0) or internal tone mode (1),
    pub /: *mut *mut u32 version; / 0: 1.0, 1: 1.1, 2: Disable,
    pub /: *mut *mut u32 center_freq; / 0: 22KHz, 1: 33KHz and 2: 44 KHz,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MXL_HYDRA_DISEQC_CFG_CONT_TONE_T {
    pub diseqc_id: u32,
    pub /: *mut *mut u32 cont_tone_flag; / 1: Enable , 0: Disable,
}
