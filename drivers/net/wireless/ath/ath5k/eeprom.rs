//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/eeprom.h
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
// Copyright (c) 2004-2008 Reyk Floeter <reyk@openbsd.org>
// Copyright (c) 2006-2008 Nick Kossifidis <mickflemm@gmail.com>
//
// Permission to use, copy, modify, and distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
// ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
// ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
// OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//
// Common ar5xxx EEPROM data offsets (set these on AR5K_EEPROM_BASE)
//
pub const AR5K_EEPROM_PCIE_OFFSET: c_uint = 0x02	/* Contains offset to PCI-E infos */;
pub const AR5K_EEPROM_PCIE_SERDES_SECTION: c_uint = 0x40	/* PCIE_OFFSET points here when;
// SERDES infos are present
pub const AR5K_EEPROM_MAGIC: c_uint = 0x003d	/* EEPROM Magic number */;
pub const AR5K_EEPROM_MAGIC_VALUE: c_uint = 0x5aa5	/* Default - found on EEPROM */;
pub const AR5K_EEPROM_IS_HB63: c_uint = 0x000b	/* Talon detect */;
pub const AR5K_EEPROM_RFKILL: c_uint = 0x0f;
pub const AR5K_EEPROM_RFKILL_GPIO_SEL: c_uint = 0x0000001c;
pub const AR5K_EEPROM_RFKILL_GPIO_SEL_S: c_int = 2;
pub const AR5K_EEPROM_RFKILL_POLARITY: c_uint = 0x00000002;
pub const AR5K_EEPROM_RFKILL_POLARITY_S: c_int = 1;
pub const AR5K_EEPROM_REG_DOMAIN: c_uint = 0x00bf	/* EEPROM regdom */;
// FLASH(EEPROM) Defines for AR531X chips
pub const AR5K_EEPROM_SIZE_LOWER: c_uint = 0x1b /* size info -- lower */;
pub const AR5K_EEPROM_SIZE_UPPER: c_uint = 0x1c /* size info -- upper */;
pub const AR5K_EEPROM_SIZE_UPPER_MASK: c_uint = 0xfff0;
pub const AR5K_EEPROM_SIZE_UPPER_SHIFT: c_int = 4;
pub const AR5K_EEPROM_SIZE_ENDLOC_SHIFT: c_int = 12;
pub const AR5K_EEPROM_CHECKSUM: c_uint = 0x00c0	/* EEPROM checksum */;
pub const AR5K_EEPROM_INFO_BASE: c_uint = 0x00c0	/* EEPROM header */;

pub const AR5K_EEPROM_INFO_CKSUM: c_uint = 0xffff;

pub const AR5K_EEPROM_VERSION_3_0: c_uint = 0x3000	/* No idea what's going on before this version */;
pub const AR5K_EEPROM_VERSION_3_1: c_uint = 0x3001	/* ob/db values for 2GHz (ar5211_rfregs) */;
pub const AR5K_EEPROM_VERSION_3_2: c_uint = 0x3002	/* different frequency representation (eeprom_bin2freq) */;
pub const AR5K_EEPROM_VERSION_3_3: c_uint = 0x3003	/* offsets changed, has 32 CTLs (see below) and ee_false_detect (eeprom_read_modes) */;
pub const AR5K_EEPROM_VERSION_3_4: c_uint = 0x3004	/* has ee_i_gain, ee_cck_ofdm_power_delta (eeprom_read_modes) */;
pub const AR5K_EEPROM_VERSION_4_0: c_uint = 0x4000	/* has ee_misc, ee_cal_pier, ee_turbo_max_power and ee_xr_power (eeprom_init) */;
pub const AR5K_EEPROM_VERSION_4_1: c_uint = 0x4001	/* has ee_margin_tx_rx (eeprom_init) */;
pub const AR5K_EEPROM_VERSION_4_2: c_uint = 0x4002	/* has ee_cck_ofdm_gain_delta (eeprom_init) */;
pub const AR5K_EEPROM_VERSION_4_3: c_uint = 0x4003	/* power calibration changes */;
pub const AR5K_EEPROM_VERSION_4_4: c_uint = 0x4004;
pub const AR5K_EEPROM_VERSION_4_5: c_uint = 0x4005;
pub const AR5K_EEPROM_VERSION_4_6: c_uint = 0x4006	/* has ee_scaled_cck_delta */;
pub const AR5K_EEPROM_VERSION_4_7: c_uint = 0x3007	/* 4007 ? */;
pub const AR5K_EEPROM_VERSION_4_9: c_uint = 0x4009	/* EAR futureproofing */;
pub const AR5K_EEPROM_VERSION_5_0: c_uint = 0x5000	/* Has 2413 PDADC calibration etc */;
pub const AR5K_EEPROM_VERSION_5_1: c_uint = 0x5001	/* Has capability values */;
pub const AR5K_EEPROM_VERSION_5_3: c_uint = 0x5003	/* Has spur mitigation tables */;
pub const AR5K_EEPROM_MODE_11A: c_int = 0;
pub const AR5K_EEPROM_MODE_11B: c_int = 1;
pub const AR5K_EEPROM_MODE_11G: c_int = 2;

// Newer EEPROMs are using a different offset

// Misc values available since EEPROM 4.0

// calibration settings

pub const AR5K_EEPROM_GROUP1_OFFSET: c_uint = 0x0;
pub const AR5K_EEPROM_GROUP2_OFFSET: c_uint = 0x5;
pub const AR5K_EEPROM_GROUP3_OFFSET: c_uint = 0x37;
pub const AR5K_EEPROM_GROUP4_OFFSET: c_uint = 0x46;
pub const AR5K_EEPROM_GROUP5_OFFSET: c_uint = 0x55;
pub const AR5K_EEPROM_GROUP6_OFFSET: c_uint = 0x65;
pub const AR5K_EEPROM_GROUP7_OFFSET: c_uint = 0x69;
pub const AR5K_EEPROM_GROUP8_OFFSET: c_uint = 0x6f;

// [3.1 - 3.3]
pub const AR5K_EEPROM_OBDB0_2GHZ: c_uint = 0x00ec;
pub const AR5K_EEPROM_OBDB1_2GHZ: c_uint = 0x00ed;
pub const AR5K_EEPROM_PROTECT: c_uint = 0x003f	/* EEPROM protect status */;
pub const AR5K_EEPROM_PROTECT_RD_0_31: c_uint = 0x0001	/* Read protection bit for offsets 0x0 - 0x1f */;
pub const AR5K_EEPROM_PROTECT_WR_0_31: c_uint = 0x0002	/* Write protection bit for offsets 0x0 - 0x1f */;
pub const AR5K_EEPROM_PROTECT_RD_32_63: c_uint = 0x0004	/* 0x20 - 0x3f */;
pub const AR5K_EEPROM_PROTECT_WR_32_63: c_uint = 0x0008;
pub const AR5K_EEPROM_PROTECT_RD_64_127: c_uint = 0x0010	/* 0x40 - 0x7f */;
pub const AR5K_EEPROM_PROTECT_WR_64_127: c_uint = 0x0020;
pub const AR5K_EEPROM_PROTECT_RD_128_191: c_uint = 0x0040	/* 0x80 - 0xbf (regdom) */;
pub const AR5K_EEPROM_PROTECT_WR_128_191: c_uint = 0x0080;
pub const AR5K_EEPROM_PROTECT_RD_192_207: c_uint = 0x0100	/* 0xc0 - 0xcf */;
pub const AR5K_EEPROM_PROTECT_WR_192_207: c_uint = 0x0200;
pub const AR5K_EEPROM_PROTECT_RD_208_223: c_uint = 0x0400	/* 0xd0 - 0xdf */;
pub const AR5K_EEPROM_PROTECT_WR_208_223: c_uint = 0x0800;
pub const AR5K_EEPROM_PROTECT_RD_224_239: c_uint = 0x1000	/* 0xe0 - 0xef */;
pub const AR5K_EEPROM_PROTECT_WR_224_239: c_uint = 0x2000;
pub const AR5K_EEPROM_PROTECT_RD_240_255: c_uint = 0x4000	/* 0xf0 - 0xff */;
pub const AR5K_EEPROM_PROTECT_WR_240_255: c_uint = 0x8000;
// Some EEPROM defines
pub const AR5K_EEPROM_EEP_SCALE: c_int = 100;
pub const AR5K_EEPROM_EEP_DELTA: c_int = 10;
pub const AR5K_EEPROM_N_MODES: c_int = 3;
pub const AR5K_EEPROM_N_5GHZ_CHAN: c_int = 10;
pub const AR5K_EEPROM_N_5GHZ_RATE_CHAN: c_int = 8;
pub const AR5K_EEPROM_N_2GHZ_CHAN: c_int = 3;
pub const AR5K_EEPROM_N_2GHZ_CHAN_2413: c_int = 4;
pub const AR5K_EEPROM_N_2GHZ_CHAN_MAX: c_int = 4;
pub const AR5K_EEPROM_MAX_CHAN: c_int = 10;
pub const AR5K_EEPROM_N_PWR_POINTS_5111: c_int = 11;
pub const AR5K_EEPROM_N_PCDAC: c_int = 11;
pub const AR5K_EEPROM_N_PHASE_CAL: c_int = 5;
pub const AR5K_EEPROM_N_TEST_FREQ: c_int = 8;
pub const AR5K_EEPROM_N_EDGES: c_int = 8;
pub const AR5K_EEPROM_N_INTERCEPTS: c_int = 11;

pub const AR5K_EEPROM_PCDAC_M: c_uint = 0x3f;
pub const AR5K_EEPROM_PCDAC_START: c_int = 1;
pub const AR5K_EEPROM_PCDAC_STOP: c_int = 63;
pub const AR5K_EEPROM_PCDAC_STEP: c_int = 1;
pub const AR5K_EEPROM_NON_EDGE_M: c_uint = 0x40;
pub const AR5K_EEPROM_CHANNEL_POWER: c_int = 8;
pub const AR5K_EEPROM_N_OBDB: c_int = 4;
pub const AR5K_EEPROM_OBDB_DIS: c_uint = 0xffff;
pub const AR5K_EEPROM_CHANNEL_DIS: c_uint = 0xff;

pub const AR5K_EEPROM_MAX_CTLS: c_int = 32;
pub const AR5K_EEPROM_N_PD_CURVES: c_int = 4;
pub const AR5K_EEPROM_N_XPD0_POINTS: c_int = 4;
pub const AR5K_EEPROM_N_XPD3_POINTS: c_int = 3;
pub const AR5K_EEPROM_N_PD_GAINS: c_int = 4;
pub const AR5K_EEPROM_N_PD_POINTS: c_int = 5;
pub const AR5K_EEPROM_N_INTERCEPT_10_2GHZ: c_int = 35;
pub const AR5K_EEPROM_N_INTERCEPT_10_5GHZ: c_int = 55;
pub const AR5K_EEPROM_POWER_M: c_uint = 0x3f;
pub const AR5K_EEPROM_POWER_MIN: c_int = 0;
pub const AR5K_EEPROM_POWER_MAX: c_int = 3150;
pub const AR5K_EEPROM_POWER_STEP: c_int = 50;
pub const AR5K_EEPROM_POWER_TABLE_SIZE: c_int = 64;
pub const AR5K_EEPROM_N_POWER_LOC_11B: c_int = 4;
pub const AR5K_EEPROM_N_POWER_LOC_11G: c_int = 6;
pub const AR5K_EEPROM_I_GAIN: c_int = 10;
pub const AR5K_EEPROM_CCK_OFDM_DELTA: c_int = 15;
pub const AR5K_EEPROM_N_IQ_CAL: c_int = 2;
// 5GHz/2GHz
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_eeprom_freq_bands {
    AR5K_EEPROM_BAND_5GHZ = 0,
    AR5K_EEPROM_BAND_2GHZ = 1,
    AR5K_EEPROM_N_FREQ_BANDS,
}

// Spur chans per freq band
pub const AR5K_EEPROM_N_SPUR_CHANS: c_int = 5;
// fbin value for chan 2464 x2
pub const AR5K_EEPROM_5413_SPUR_CHAN_1: c_int = 1640;
// fbin value for chan 2420 x2
pub const AR5K_EEPROM_5413_SPUR_CHAN_2: c_int = 1200;
pub const AR5K_EEPROM_SPUR_CHAN_MASK: c_uint = 0x3FFF;
pub const AR5K_EEPROM_NO_SPUR: c_uint = 0x8000;
pub const AR5K_SPUR_CHAN_WIDTH: c_int = 87;
pub const AR5K_SPUR_SYMBOL_WIDTH_BASE_100Hz: c_int = 3125;
pub const AR5K_SPUR_SYMBOL_WIDTH_TURBO_100Hz: c_int = 6250;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_ant_table {
    AR5K_ANT_CTL		= 0,	/* Idle switch table settings */
    AR5K_ANT_SWTABLE_A	= 1,	/* Switch table for antenna A */
    AR5K_ANT_SWTABLE_B	= 2,	/* Switch table for antenna B */
    AR5K_ANT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_ctl_mode {
    AR5K_CTL_11A = 0,
    AR5K_CTL_11B = 1,
    AR5K_CTL_11G = 2,
    AR5K_CTL_TURBO = 3,
    AR5K_CTL_TURBOG = 4,
    AR5K_CTL_2GHT20 = 5,
    AR5K_CTL_5GHT20 = 6,
    AR5K_CTL_2GHT40 = 7,
    AR5K_CTL_5GHT40 = 8,
    AR5K_CTL_MODE_M = 15,
}

// Per channel calibration data, used for power table setup
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_chan_pcal_info_rf5111 {
// Power levels in half dBm units
// for one power curve.
    pub pwr: [u8; AR5K_EEPROM_N_PWR_POINTS_5111],
// PCDAC table steps
// for the above values
    pub pcdac: [u8; AR5K_EEPROM_N_PWR_POINTS_5111],
// Starting PCDAC step
    pub pcdac_min: u8,
// Final PCDAC step
    pub pcdac_max: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_chan_pcal_info_rf5112 {
// Power levels in quarter dBm units
// for lower (0) and higher (3)
// level curves in 0.25dB units
    pub pwr_x0: [i8; AR5K_EEPROM_N_XPD0_POINTS],
    pub pwr_x3: [i8; AR5K_EEPROM_N_XPD3_POINTS],
// PCDAC table steps
// for the above values
    pub pcdac_x0: [u8; AR5K_EEPROM_N_XPD0_POINTS],
    pub pcdac_x3: [u8; AR5K_EEPROM_N_XPD3_POINTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_chan_pcal_info_rf2413 {
// Starting pwr/pddac values
    pub pwr_i: [i8; AR5K_EEPROM_N_PD_GAINS],
    pub pddac_i: [u8; AR5K_EEPROM_N_PD_GAINS],
// (pwr,pddac) points
// power levels in 0.5dB units
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_powertable_type {
    AR5K_PWRTABLE_PWR_TO_PCDAC = 0,
    AR5K_PWRTABLE_LINEAR_PCDAC = 1,
    AR5K_PWRTABLE_PWR_TO_PDADC = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_pdgain_info {
    pub pd_points: u8,
    pub pd_step: *mut u8,
// Power values are in
// 0.25dB units
    pub pd_pwr: *mut i16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_chan_pcal_info {
// Frequency
    pub freq: u16,
// Tx power boundaries
    pub max_pwr: i16,
    pub min_pwr: i16,
    pub rf5111_info: ath5k_chan_pcal_info_rf5111,
    pub rf5112_info: ath5k_chan_pcal_info_rf5112,
    pub rf2413_info: ath5k_chan_pcal_info_rf2413,
}

// Raw values used by phy code
// Curves are stored in order from lower
// gain to higher gain (max txpower -> min txpower)
// Per rate calibration data for each mode,
// used for rate power table setup.
// Note: Values in 0.5dB units
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_rate_pcal_info {
    pub /: *mut *mut u16 freq; / Frequency,
// Power level for 6-24Mbit/s rates or
// 1Mb rate
    pub target_power_6to24: u16,
// Power level for 36Mbit rate or
// 2Mb rate
    pub target_power_36: u16,
// Power level for 48Mbit rate or
// 5.5Mbit rate
    pub target_power_48: u16,
// Power level for 54Mbit rate or
// 11Mbit rate
    pub target_power_54: u16,
}

// Power edges for conformance test limits
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_edge_power {
    pub freq: u16,
    pub /: *mut *mut u16 edge; / in half dBm,
    pub flag: bool,
}

//
// struct ath5k_eeprom_info - EEPROM calibration data
//
// @ee_regdomain: ath/regd.c takes care of COUNTRY_ERD and WORLDWIDE_ROAMING
// flags
// @ee_ant_gain: Antenna gain in 0.5dB steps signed [5211 only?]
// @ee_cck_ofdm_gain_delta: difference in gainF to output the same power for
// OFDM and CCK packets
// @ee_cck_ofdm_power_delta: power difference between OFDM (6Mbps) and CCK
// (11Mbps) rate in G mode. 0.1dB steps
// @ee_scaled_cck_delta: for Japan Channel 14: 0.1dB resolution
//
// @ee_i_cal: Initial I coefficient to correct I/Q mismatch in the receive path
// @ee_q_cal: Initial Q coefficient to correct I/Q mismatch in the receive path
// @ee_fixed_bias: use ee_ob and ee_db settings or use automatic control
// @ee_switch_settling: RX/TX Switch settling time
// @ee_atn_tx_rx: Difference in attenuation between TX and RX in 1dB steps
// @ee_ant_control: Antenna Control Settings
// @ee_ob: Bias current for Output stage of PA
// B/G mode: Index [0] is used for AR2112/5112, otherwise [1]
// A mode: [0] 5.15-5.25 [1] 5.25-5.50 [2] 5.50-5.70 [3] 5.70-5.85 GHz
// @ee_db: Bias current for Output stage of PA. see @ee_ob
// @ee_tx_end2xlna_enable: Time difference from when BB finishes sending a frame
// to when the external LNA is activated
// @ee_tx_end2xpa_disable: Time difference from when BB finishes sending a frame
// to when the external PA switch is deactivated
// @ee_tx_frm2xpa_enable: Time difference from when MAC sends frame to when
// external PA switch is activated
// @ee_thr_62: Clear Channel Assessment (CCA) sensitivity
// (IEEE802.11a section 17.3.10.5 )
// @ee_xlna_gain: Total gain of the LNA (information only)
// @ee_xpd: Use external (1) or internal power detector
// @ee_x_gain: Gain for external power detector output (differences in EEMAP
// versions!)
// @ee_i_gain: Initial gain value after reset
// @ee_margin_tx_rx: Margin in dB when final attenuation stage should be used
//
// @ee_false_detect: Backoff in Sensitivity (dB) on channels with spur signals
// @ee_noise_floor_thr: Noise floor threshold in 1dB steps
// @ee_adc_desired_size: Desired amplitude for ADC, used by AGC; in 0.5 dB steps
// @ee_pga_desired_size: Desired output of PGA (for BB gain) in 0.5 dB steps
// @ee_pd_gain_overlap: PD ADC curves need to overlap in 0.5dB steps (ee_map>=2)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_eeprom_info {
// Header information
    pub ee_magic: u16,
    pub ee_protect: u16,
    pub ee_regdomain: u16,
    pub ee_version: u16,
    pub ee_header: u16,
    pub ee_ant_gain: u16,
    pub ee_rfkill_pin: u8,
    pub ee_rfkill_pol: bool,
    pub ee_is_hb63: bool,
    pub ee_serdes: bool,
    pub ee_misc0: u16,
    pub ee_misc1: u16,
    pub ee_misc2: u16,
    pub ee_misc3: u16,
    pub ee_misc4: u16,
    pub ee_misc5: u16,
    pub ee_misc6: u16,
    pub ee_cck_ofdm_gain_delta: u16,
    pub ee_cck_ofdm_power_delta: u16,
    pub ee_scaled_cck_delta: u16,
// RF Calibration settings (reset, rfregs)
    pub ee_i_cal: [u16; AR5K_EEPROM_N_MODES],
    pub ee_q_cal: [u16; AR5K_EEPROM_N_MODES],
    pub ee_fixed_bias: [u16; AR5K_EEPROM_N_MODES],
    pub ee_turbo_max_power: [u16; AR5K_EEPROM_N_MODES],
    pub ee_xr_power: [u16; AR5K_EEPROM_N_MODES],
    pub ee_switch_settling: [u16; AR5K_EEPROM_N_MODES],
    pub ee_atn_tx_rx: [u16; AR5K_EEPROM_N_MODES],
    pub ee_ant_control: [u16; AR5K_EEPROM_N_MODES][AR5K_EEPROM_N_PCDAC],
    pub ee_ob: [u16; AR5K_EEPROM_N_MODES][AR5K_EEPROM_N_OBDB],
    pub ee_db: [u16; AR5K_EEPROM_N_MODES][AR5K_EEPROM_N_OBDB],
    pub ee_tx_end2xlna_enable: [u16; AR5K_EEPROM_N_MODES],
    pub ee_tx_end2xpa_disable: [u16; AR5K_EEPROM_N_MODES],
    pub ee_tx_frm2xpa_enable: [u16; AR5K_EEPROM_N_MODES],
    pub ee_thr_62: [u16; AR5K_EEPROM_N_MODES],
    pub ee_xlna_gain: [u16; AR5K_EEPROM_N_MODES],
    pub ee_xpd: [u16; AR5K_EEPROM_N_MODES],
    pub ee_x_gain: [u16; AR5K_EEPROM_N_MODES],
    pub ee_i_gain: [u16; AR5K_EEPROM_N_MODES],
    pub ee_margin_tx_rx: [u16; AR5K_EEPROM_N_MODES],
    pub ee_switch_settling_turbo: [u16; AR5K_EEPROM_N_MODES],
    pub ee_margin_tx_rx_turbo: [u16; AR5K_EEPROM_N_MODES],
    pub ee_atn_tx_rx_turbo: [u16; AR5K_EEPROM_N_MODES],
// Power calibration data
    pub ee_false_detect: [u16; AR5K_EEPROM_N_MODES],
// Number of pd gain curves per mode
    pub ee_pd_gains: [u8; AR5K_EEPROM_N_MODES],
// Back mapping pdcurve number -> pdcurve index in pd->pd_curves
    pub ee_pdc_to_idx: [u8; AR5K_EEPROM_N_MODES][AR5K_EEPROM_N_PD_GAINS],
    pub ee_n_piers: [u8; AR5K_EEPROM_N_MODES],
    pub ee_pwr_cal_a: [ath5k_chan_pcal_info; AR5K_EEPROM_N_5GHZ_CHAN],
    pub ee_pwr_cal_b: [ath5k_chan_pcal_info; AR5K_EEPROM_N_2GHZ_CHAN_MAX],
    pub ee_pwr_cal_g: [ath5k_chan_pcal_info; AR5K_EEPROM_N_2GHZ_CHAN_MAX],
// Per rate target power levels
    pub ee_rate_target_pwr_num: [u8; AR5K_EEPROM_N_MODES],
    pub ee_rate_tpwr_a: [ath5k_rate_pcal_info; AR5K_EEPROM_N_5GHZ_CHAN],
    pub ee_rate_tpwr_b: [ath5k_rate_pcal_info; AR5K_EEPROM_N_2GHZ_CHAN_MAX],
    pub ee_rate_tpwr_g: [ath5k_rate_pcal_info; AR5K_EEPROM_N_2GHZ_CHAN_MAX],
// Conformance test limits (Unused)
    pub ee_ctls: u8,
    pub ee_ctl: [u8; AR5K_EEPROM_MAX_CTLS],
    pub AR5K_EEPROM_MAX_CTLS]: *mut *mut ath5k_edge_power ee_ctl_pwr[AR5K_EEPROM_N_EDGES,
// Noise Floor Calibration settings
    pub ee_noise_floor_thr: [i16; AR5K_EEPROM_N_MODES],
    pub ee_adc_desired_size: [i8; AR5K_EEPROM_N_MODES],
    pub ee_pga_desired_size: [i8; AR5K_EEPROM_N_MODES],
    pub ee_adc_desired_size_turbo: [i8; AR5K_EEPROM_N_MODES],
    pub ee_pga_desired_size_turbo: [i8; AR5K_EEPROM_N_MODES],
    pub ee_pd_gain_overlap: i8,
// Spur mitigation data (fbin values for spur channels)
    pub ee_spur_chans: [u16; AR5K_EEPROM_N_SPUR_CHANS][AR5K_EEPROM_N_FREQ_BANDS],
}
