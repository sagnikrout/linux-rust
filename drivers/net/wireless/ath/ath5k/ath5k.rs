//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/ath5k.h
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
// Copyright (c) 2004-2007 Reyk Floeter <reyk@openbsd.org>
// Copyright (c) 2006-2007 Nick Kossifidis <mickflemm@gmail.com>
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
// TODO: Clean up channel debugging (doesn't work anyway) and start
// working on reg. control code using all available eeprom information
// (rev. engineering needed)
pub const CHAN_DEBUG: c_int = 0;

// RX/TX descriptor hw structs
// TODO: Driver part should only see sw structs

// EEPROM structs/offsets
// TODO: Make a more generic struct (eg. add more stuff to ath5k_capabilities)
// and clean up common bits, then introduce set/get functions in eeprom.c

// PCI IDs
pub const PCI_DEVICE_ID_ATHEROS_AR5210: c_uint = 0x0007 /* AR5210 */;
pub const PCI_DEVICE_ID_ATHEROS_AR5311: c_uint = 0x0011 /* AR5311 */;
pub const PCI_DEVICE_ID_ATHEROS_AR5211: c_uint = 0x0012 /* AR5211 */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212: c_uint = 0x0013 /* AR5212 */;
pub const PCI_DEVICE_ID_3COM_3CRDAG675: c_uint = 0x0013 /* 3CRDAG675 (Atheros AR5212) */;
pub const PCI_DEVICE_ID_3COM_2_3CRPAG175: c_uint = 0x0013 /* 3CRPAG175 (Atheros AR5212) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5210_AP: c_uint = 0x0207 /* AR5210 (Early) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_IBM: c_uint = 0x1014 /* AR5212 (IBM MiniPCI) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5210_DEFAULT: c_uint = 0x1107 /* AR5210 (no eeprom) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_DEFAULT: c_uint = 0x1113 /* AR5212 (no eeprom) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5211_DEFAULT: c_uint = 0x1112 /* AR5211 (no eeprom) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_FPGA: c_uint = 0xf013 /* AR5212 (emulation board) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5211_LEGACY: c_uint = 0xff12 /* AR5211 (emulation board) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5211_FPGA11B: c_uint = 0xf11b /* AR5211 (emulation board) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5312_REV2: c_uint = 0x0052 /* AR5312 WMAC (AP31) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5312_REV7: c_uint = 0x0057 /* AR5312 WMAC (AP30-040) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5312_REV8: c_uint = 0x0058 /* AR5312 WMAC (AP43-030) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_0014: c_uint = 0x0014 /* AR5212 compatible */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_0015: c_uint = 0x0015 /* AR5212 compatible */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_0016: c_uint = 0x0016 /* AR5212 compatible */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_0017: c_uint = 0x0017 /* AR5212 compatible */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_0018: c_uint = 0x0018 /* AR5212 compatible */;
pub const PCI_DEVICE_ID_ATHEROS_AR5212_0019: c_uint = 0x0019 /* AR5212 compatible */;
pub const PCI_DEVICE_ID_ATHEROS_AR2413: c_uint = 0x001a /* AR2413 (Griffin-lite) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5413: c_uint = 0x001b /* AR5413 (Eagle) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5424: c_uint = 0x001c /* AR5424 (Condor PCI-E) */;
pub const PCI_DEVICE_ID_ATHEROS_AR5416: c_uint = 0x0023 /* AR5416 */;
pub const PCI_DEVICE_ID_ATHEROS_AR5418: c_uint = 0x0024 /* AR5418 */;
// \

//
// AR5K REGISTER ACCESS
//
// Some macros to read/write fields
// First shift, then mask

// First mask, then shift

// Some registers can hold multiple values of interest. For this
// reason when we want to write to these registers we must first
// retrieve the values which we do not want to clear (lets call this
// old_data) and then set the register with this and our new_value:
// ( old_data | new_value)

// Access QCU registers per queue

// Used while writing initvals

//
// Some tunable values (these should be changeable by the user)
// TODO: Make use of them and add more options OR use debug/configfs
//
pub const AR5K_TUNE_DMA_BEACON_RESP: c_int = 2;
pub const AR5K_TUNE_SW_BEACON_RESP: c_int = 10;
pub const AR5K_TUNE_ADDITIONAL_SWBA_BACKOFF: c_int = 0;
pub const AR5K_TUNE_MIN_TX_FIFO_THRES: c_int = 1;

pub const AR5K_TUNE_REGISTER_TIMEOUT: c_int = 20000;
// Register for RSSI threshold has a mask of 0xff, so 255 seems to
// be the max value.
pub const AR5K_TUNE_RSSI_THRES: c_int = 129;
// This must be set when setting the RSSI threshold otherwise it can
// prevent a reset. If AR5K_RSSI_THR is read after writing to it
// the BMISS_THRES will be seen as 0, seems hardware doesn't keep
// track of it. Max value depends on hardware. For AR5210 this is just 7.
// For AR5211+ this seems to be up to 255.
pub const AR5K_TUNE_BMISS_THRES: c_int = 7;
pub const AR5K_TUNE_REGISTER_DWELL_TIME: c_int = 20000;
pub const AR5K_TUNE_BEACON_INTERVAL: c_int = 100;
pub const AR5K_TUNE_AIFS: c_int = 2;
pub const AR5K_TUNE_AIFS_11B: c_int = 2;
pub const AR5K_TUNE_AIFS_XR: c_int = 0;
pub const AR5K_TUNE_CWMIN: c_int = 15;
pub const AR5K_TUNE_CWMIN_11B: c_int = 31;
pub const AR5K_TUNE_CWMIN_XR: c_int = 3;
pub const AR5K_TUNE_CWMAX: c_int = 1023;
pub const AR5K_TUNE_CWMAX_11B: c_int = 1023;
pub const AR5K_TUNE_CWMAX_XR: c_int = 7;

pub const AR5K_TUNE_MAX_TXPOWER: c_int = 63;
pub const AR5K_TUNE_DEFAULT_TXPOWER: c_int = 25;

pub const AR5K_INIT_CARR_SENSE_EN: c_int = 1;
// Swap RX/TX Descriptor for big endian archs

pub const AR5K_INIT_CFG: c_uint = 0x00000000;

// Initial values
pub const AR5K_INIT_CYCRSSI_THR1: c_int = 2;
// Tx retry limit defaults from standard
pub const AR5K_INIT_RETRY_SHORT: c_int = 7;
pub const AR5K_INIT_RETRY_LONG: c_int = 4;
// Slot time
pub const AR5K_INIT_SLOT_TIME_TURBO: c_int = 6;
pub const AR5K_INIT_SLOT_TIME_DEFAULT: c_int = 9;
pub const AR5K_INIT_SLOT_TIME_HALF_RATE: c_int = 13;
pub const AR5K_INIT_SLOT_TIME_QUARTER_RATE: c_int = 21;
pub const AR5K_INIT_SLOT_TIME_B: c_int = 20;
pub const AR5K_SLOT_TIME_MAX: c_uint = 0xffff;
// SIFS
pub const AR5K_INIT_SIFS_TURBO: c_int = 6;
pub const AR5K_INIT_SIFS_DEFAULT_BG: c_int = 10;
pub const AR5K_INIT_SIFS_DEFAULT_A: c_int = 16;
pub const AR5K_INIT_SIFS_HALF_RATE: c_int = 32;
pub const AR5K_INIT_SIFS_QUARTER_RATE: c_int = 64;
// Used to calculate tx time for non 5/10/40MHz
// operation
// It's preamble time + signal time (16 + 4)
pub const AR5K_INIT_OFDM_PREAMPLE_TIME: c_int = 20;
// Preamble time for 40MHz (turbo) operation (min ?)
pub const AR5K_INIT_OFDM_PREAMBLE_TIME_MIN: c_int = 14;
pub const AR5K_INIT_OFDM_SYMBOL_TIME: c_int = 4;
pub const AR5K_INIT_OFDM_PLCP_BITS: c_int = 22;
// Rx latency for 5 and 10MHz operation (max ?)
pub const AR5K_INIT_RX_LAT_MAX: c_int = 63;
// Tx latencies from initvals (5212 only but no problem
// because we only tweak them on 5212)
pub const AR5K_INIT_TX_LAT_A: c_int = 54;
pub const AR5K_INIT_TX_LAT_BG: c_int = 384;
// Tx latency for 40MHz (turbo) operation (min ?)
pub const AR5K_INIT_TX_LAT_MIN: c_int = 32;
// Default Tx/Rx latencies (same for 5211)
pub const AR5K_INIT_TX_LATENCY_5210: c_int = 54;
pub const AR5K_INIT_RX_LATENCY_5210: c_int = 29;
// Tx frame to Tx data start delay
pub const AR5K_INIT_TXF2TXD_START_DEFAULT: c_int = 14;
pub const AR5K_INIT_TXF2TXD_START_DELAY_10MHZ: c_int = 12;
pub const AR5K_INIT_TXF2TXD_START_DELAY_5MHZ: c_int = 13;
// We need to increase PHY switch and agc settling time
// on turbo mode
pub const AR5K_SWITCH_SETTLING: c_int = 5760;
pub const AR5K_SWITCH_SETTLING_TURBO: c_int = 7168;
pub const AR5K_AGC_SETTLING: c_int = 28;
// 38 on 5210 but shouldn't matter
pub const AR5K_AGC_SETTLING_TURBO: c_int = 37;
// \
// GENERIC CHIPSET DEFINITIONS
//
// enum ath5k_version - MAC Chips
// @AR5K_AR5210: AR5210 (Crete)
// @AR5K_AR5211: AR5211 (Oahu/Maui)
// @AR5K_AR5212: AR5212 (Venice) and newer
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_version {
    AR5K_AR5210	= 0,
    AR5K_AR5211	= 1,
    AR5K_AR5212	= 2,
}

//
// enum ath5k_radio - PHY Chips
// @AR5K_RF5110: RF5110 (Fez)
// @AR5K_RF5111: RF5111 (Sombrero)
// @AR5K_RF5112: RF2112/5112(A) (Derby/Derby2)
// @AR5K_RF2413: RF2413/2414 (Griffin/Griffin-Lite)
// @AR5K_RF5413: RF5413/5414/5424 (Eagle/Condor)
// @AR5K_RF2316: RF2315/2316 (Cobra SoC)
// @AR5K_RF2317: RF2317 (Spider SoC)
// @AR5K_RF2425: RF2425/2417 (Swan/Nalla)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_radio {
    AR5K_RF5110	= 0,
    AR5K_RF5111	= 1,
    AR5K_RF5112	= 2,
    AR5K_RF2413	= 3,
    AR5K_RF5413	= 4,
    AR5K_RF2316	= 5,
    AR5K_RF2317	= 6,
    AR5K_RF2425	= 7,
}

//
// Common silicon revision/version values
//
pub const AR5K_SREV_UNKNOWN: c_uint = 0xffff;
pub const AR5K_SREV_AR5210: c_uint = 0x00 /* Crete */;
pub const AR5K_SREV_AR5311: c_uint = 0x10 /* Maui 1 */;
pub const AR5K_SREV_AR5311A: c_uint = 0x20 /* Maui 2 */;
pub const AR5K_SREV_AR5311B: c_uint = 0x30 /* Spirit */;
pub const AR5K_SREV_AR5211: c_uint = 0x40 /* Oahu */;
pub const AR5K_SREV_AR5212: c_uint = 0x50 /* Venice */;
pub const AR5K_SREV_AR5312_R2: c_uint = 0x52 /* AP31 */;
pub const AR5K_SREV_AR5212_V4: c_uint = 0x54 /* ??? */;
pub const AR5K_SREV_AR5213: c_uint = 0x55 /* ??? */;
pub const AR5K_SREV_AR5312_R7: c_uint = 0x57 /* AP30 */;
pub const AR5K_SREV_AR2313_R8: c_uint = 0x58 /* AP43 */;
pub const AR5K_SREV_AR5213A: c_uint = 0x59 /* Hainan */;
pub const AR5K_SREV_AR2413: c_uint = 0x78 /* Griffin lite */;
pub const AR5K_SREV_AR2414: c_uint = 0x70 /* Griffin */;
pub const AR5K_SREV_AR2315_R6: c_uint = 0x86 /* AP51-Light */;
pub const AR5K_SREV_AR2315_R7: c_uint = 0x87 /* AP51-Full */;
pub const AR5K_SREV_AR5424: c_uint = 0x90 /* Condor */;
pub const AR5K_SREV_AR2317_R1: c_uint = 0x90 /* AP61-Light */;
pub const AR5K_SREV_AR2317_R2: c_uint = 0x91 /* AP61-Full */;
pub const AR5K_SREV_AR5413: c_uint = 0xa4 /* Eagle lite */;
pub const AR5K_SREV_AR5414: c_uint = 0xa0 /* Eagle */;
pub const AR5K_SREV_AR2415: c_uint = 0xb0 /* Talon */;
pub const AR5K_SREV_AR5416: c_uint = 0xc0 /* PCI-E */;
pub const AR5K_SREV_AR5418: c_uint = 0xca /* PCI-E */;
pub const AR5K_SREV_AR2425: c_uint = 0xe0 /* Swan */;
pub const AR5K_SREV_AR2417: c_uint = 0xf0 /* Nala */;
pub const AR5K_SREV_RAD_5110: c_uint = 0x00;
pub const AR5K_SREV_RAD_5111: c_uint = 0x10;
pub const AR5K_SREV_RAD_5111A: c_uint = 0x15;
pub const AR5K_SREV_RAD_2111: c_uint = 0x20;
pub const AR5K_SREV_RAD_5112: c_uint = 0x30;
pub const AR5K_SREV_RAD_5112A: c_uint = 0x35;
pub const AR5K_SREV_RAD_5112B: c_uint = 0x36;
pub const AR5K_SREV_RAD_2112: c_uint = 0x40;
pub const AR5K_SREV_RAD_2112A: c_uint = 0x45;
pub const AR5K_SREV_RAD_2112B: c_uint = 0x46;
pub const AR5K_SREV_RAD_2413: c_uint = 0x50;
pub const AR5K_SREV_RAD_5413: c_uint = 0x60;
pub const AR5K_SREV_RAD_2316: c_uint = 0x70 /* Cobra SoC */;
pub const AR5K_SREV_RAD_2317: c_uint = 0x80;
pub const AR5K_SREV_RAD_5424: c_uint = 0xa0 /* Mostly same as 5413 */;
pub const AR5K_SREV_RAD_2425: c_uint = 0xa2;
pub const AR5K_SREV_RAD_5133: c_uint = 0xc0;
pub const AR5K_SREV_PHY_5211: c_uint = 0x30;
pub const AR5K_SREV_PHY_5212: c_uint = 0x41;
pub const AR5K_SREV_PHY_5212A: c_uint = 0x42;
pub const AR5K_SREV_PHY_5212B: c_uint = 0x43;
pub const AR5K_SREV_PHY_2413: c_uint = 0x45;
pub const AR5K_SREV_PHY_5413: c_uint = 0x61;
pub const AR5K_SREV_PHY_2425: c_uint = 0x70;
// TODO add support to mac80211 for vendor-specific rates and modes
//
// DOC: Atheros XR
//
// Some of this information is based on Documentation from:
//
// http://madwifi-project.org/wiki/ChipsetFeatures/SuperAG
//
// Atheros' eXtended Range - range enhancing extension is a modulation scheme
// that is supposed to double the link distance between an Atheros XR-enabled
// client device with an Atheros XR-enabled access point. This is achieved
// by increasing the receiver sensitivity up to, -105dBm, which is about 20dB
// above what the 802.11 specifications demand. In addition, new (proprietary)
// data rates are introduced: 3, 2, 1, 0.5 and 0.25 MBit/s.
//
// Please note that can you either use XR or TURBO but you cannot use both,
// they are exclusive.
//
// Also note that we do not plan to support XR mode at least for now. You can
// get a mode similar to XR by using 5MHz bwmode.
//
// DOC: Atheros SuperAG
//
// In addition to XR we have another modulation scheme called TURBO mode
// that is supposed to provide a throughput transmission speed up to 40Mbit/s
// -60Mbit/s at a 108Mbit/s signaling rate achieved through the bonding of two
// 54Mbit/s 802.11g channels. To use this feature both ends must support it.
// There is also a distinction between "static" and "dynamic" turbo modes:
//
// - Static: is the dumb version: devices set to this mode stick to it until
// the mode is turned off.
//
// - Dynamic: is the intelligent version, the network decides itself if it
// is ok to use turbo. As soon as traffic is detected on adjacent channels
// (which would get used in turbo mode), or when a non-turbo station joins
// the network, turbo mode won't be used until the situation changes again.
// Dynamic mode is achieved by Atheros' Adaptive Radio (AR) feature which
// monitors the used radio band in order to decide whether turbo mode may
// be used or not.
//
// This article claims Super G sticks to bonding of channels 5 and 6 for
// USA:
//
// https://www.pcworld.com/article/id,113428-page,1/article.html
//
// The channel bonding seems to be driver specific though.
//
// In addition to TURBO modes we also have the following features for even
// greater speed-up:
//
// - Bursting: allows multiple frames to be sent at once, rather than pausing
// after each frame. Bursting is a standards-compliant feature that can be
// used with any Access Point.
//
// - Fast frames: increases the amount of information that can be sent per
// frame, also resulting in a reduction of transmission overhead. It is a
// proprietary feature that needs to be supported by the Access Point.
//
// - Compression: data frames are compressed in real time using a Lempel Ziv
// algorithm. This is done transparently. Once this feature is enabled,
// compression and decompression takes place inside the chipset, without
// putting additional load on the host CPU.
//
// As with XR we also don't plan to support SuperAG features for now. You can
// get a mode similar to TURBO by using 40MHz bwmode.
//
// enum ath5k_driver_mode - PHY operation mode
// @AR5K_MODE_11A: 802.11a
// @AR5K_MODE_11B: 802.11b
// @AR5K_MODE_11G: 801.11g
// @AR5K_MODE_MAX: Used for boundary checks
//
// Do not change the order here, we use these as
// array indices and it also maps EEPROM structures.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_driver_mode {
    AR5K_MODE_11A		=	0,
    AR5K_MODE_11B		=	1,
    AR5K_MODE_11G		=	2,
    AR5K_MODE_MAX		=	3
}

//
// enum ath5k_ant_mode - Antenna operation mode
// @AR5K_ANTMODE_DEFAULT: Default antenna setup
// @AR5K_ANTMODE_FIXED_A: Only antenna A is present
// @AR5K_ANTMODE_FIXED_B: Only antenna B is present
// @AR5K_ANTMODE_SINGLE_AP: STA locked on a single ap
// @AR5K_ANTMODE_SECTOR_AP: AP with tx antenna set on tx desc
// @AR5K_ANTMODE_SECTOR_STA: STA with tx antenna set on tx desc
// @AR5K_ANTMODE_DEBUG: Debug mode -A -> Rx, B-> Tx-
// @AR5K_ANTMODE_MAX: Used for boundary checks
//
// For more infos on antenna control check out phy.c
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_ant_mode {
    AR5K_ANTMODE_DEFAULT	= 0,
    AR5K_ANTMODE_FIXED_A	= 1,
    AR5K_ANTMODE_FIXED_B	= 2,
    AR5K_ANTMODE_SINGLE_AP	= 3,
    AR5K_ANTMODE_SECTOR_AP	= 4,
    AR5K_ANTMODE_SECTOR_STA	= 5,
    AR5K_ANTMODE_DEBUG	= 6,
    AR5K_ANTMODE_MAX,
}

//
// enum ath5k_bw_mode - Bandwidth operation mode
// @AR5K_BWMODE_DEFAULT: 20MHz, default operation
// @AR5K_BWMODE_5MHZ: Quarter rate
// @AR5K_BWMODE_10MHZ: Half rate
// @AR5K_BWMODE_40MHZ: Turbo
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_bw_mode {
    AR5K_BWMODE_DEFAULT	= 0,
    AR5K_BWMODE_5MHZ	= 1,
    AR5K_BWMODE_10MHZ	= 2,
    AR5K_BWMODE_40MHZ	= 3
}

// \
//
// struct ath5k_tx_status - TX Status descriptor
// @ts_seqnum: Sequence number
// @ts_tstamp: Timestamp
// @ts_status: Status code
// @ts_final_idx: Final transmission series index
// @ts_final_retry: Final retry count
// @ts_rssi: RSSI for received ACK
// @ts_shortretry: Short retry count
// @ts_virtcol: Virtual collision count
// @ts_antenna: Antenna used
//
// TX status descriptor gets filled by the hw
// on each transmission attempt.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_tx_status {
    pub ts_seqnum: u16,
    pub ts_tstamp: u16,
    pub ts_status: u8,
    pub ts_final_idx: u8,
    pub ts_final_retry: u8,
    pub ts_rssi: i8,
    pub ts_shortretry: u8,
    pub ts_virtcol: u8,
    pub ts_antenna: u8,
}

pub const AR5K_TXSTAT_ALTRATE: c_uint = 0x80;
pub const AR5K_TXERR_XRETRY: c_uint = 0x01;
pub const AR5K_TXERR_FILT: c_uint = 0x02;
pub const AR5K_TXERR_FIFO: c_uint = 0x04;
//
// enum ath5k_tx_queue - Queue types used to classify tx queues.
// @AR5K_TX_QUEUE_INACTIVE: q is unused -- see ath5k_hw_release_tx_queue
// @AR5K_TX_QUEUE_DATA: A normal data queue
// @AR5K_TX_QUEUE_BEACON: The beacon queue
// @AR5K_TX_QUEUE_CAB: The after-beacon queue
// @AR5K_TX_QUEUE_UAPSD: Unscheduled Automatic Power Save Delivery queue
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_tx_queue {
    AR5K_TX_QUEUE_INACTIVE = 0,
    AR5K_TX_QUEUE_DATA,
    AR5K_TX_QUEUE_BEACON,
    AR5K_TX_QUEUE_CAB,
    AR5K_TX_QUEUE_UAPSD,
}

pub const AR5K_NUM_TX_QUEUES: c_int = 10;
pub const AR5K_NUM_TX_QUEUES_NOQCU: c_int = 2;
//
// enum ath5k_tx_queue_subtype - Queue sub-types to classify normal data queues
// @AR5K_WME_AC_BK: Background traffic
// @AR5K_WME_AC_BE: Best-effort (normal) traffic
// @AR5K_WME_AC_VI: Video traffic
// @AR5K_WME_AC_VO: Voice traffic
//
// These are the 4 Access Categories as defined in
// WME spec. 0 is the lowest priority and 4 is the
// highest. Normal data that hasn't been classified
// goes to the Best Effort AC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_tx_queue_subtype {
    AR5K_WME_AC_BK = 0,
    AR5K_WME_AC_BE,
    AR5K_WME_AC_VI,
    AR5K_WME_AC_VO,
}

//
// enum ath5k_tx_queue_id - Queue ID numbers as returned by the hw functions
// @AR5K_TX_QUEUE_ID_NOQCU_DATA: Data queue on AR5210 (no QCU available)
// @AR5K_TX_QUEUE_ID_NOQCU_BEACON: Beacon queue on AR5210 (no QCU available)
// @AR5K_TX_QUEUE_ID_DATA_MIN: Data queue min index
// @AR5K_TX_QUEUE_ID_DATA_MAX: Data queue max index
// @AR5K_TX_QUEUE_ID_CAB: Content after beacon queue
// @AR5K_TX_QUEUE_ID_BEACON: Beacon queue
// @AR5K_TX_QUEUE_ID_UAPSD: Urgent Automatic Power Save Delivery,
//
// Each number represents a hw queue. If hw does not support hw queues
// (eg 5210) all data goes in one queue.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_tx_queue_id {
    AR5K_TX_QUEUE_ID_NOQCU_DATA	= 0,
    AR5K_TX_QUEUE_ID_NOQCU_BEACON	= 1,
    AR5K_TX_QUEUE_ID_DATA_MIN	= 0,
    AR5K_TX_QUEUE_ID_DATA_MAX	= 3,
    AR5K_TX_QUEUE_ID_UAPSD		= 7,
    AR5K_TX_QUEUE_ID_CAB		= 8,
    AR5K_TX_QUEUE_ID_BEACON		= 9,
}

//
// Flags to set hw queue's parameters...
//
pub const AR5K_TXQ_FLAG_TXOKINT_ENABLE: c_uint = 0x0001	/* Enable TXOK interrupt */;
pub const AR5K_TXQ_FLAG_TXERRINT_ENABLE: c_uint = 0x0002	/* Enable TXERR interrupt */;
pub const AR5K_TXQ_FLAG_TXEOLINT_ENABLE: c_uint = 0x0004	/* Enable TXEOL interrupt -not used- */;
pub const AR5K_TXQ_FLAG_TXDESCINT_ENABLE: c_uint = 0x0008	/* Enable TXDESC interrupt -not used- */;
pub const AR5K_TXQ_FLAG_TXURNINT_ENABLE: c_uint = 0x0010	/* Enable TXURN interrupt */;
pub const AR5K_TXQ_FLAG_CBRORNINT_ENABLE: c_uint = 0x0020	/* Enable CBRORN interrupt */;
pub const AR5K_TXQ_FLAG_CBRURNINT_ENABLE: c_uint = 0x0040	/* Enable CBRURN interrupt */;
pub const AR5K_TXQ_FLAG_QTRIGINT_ENABLE: c_uint = 0x0080	/* Enable QTRIG interrupt */;
pub const AR5K_TXQ_FLAG_TXNOFRMINT_ENABLE: c_uint = 0x0100	/* Enable TXNOFRM interrupt */;
pub const AR5K_TXQ_FLAG_BACKOFF_DISABLE: c_uint = 0x0200	/* Disable random post-backoff */;
pub const AR5K_TXQ_FLAG_RDYTIME_EXP_POLICY_ENABLE: c_uint = 0x0300	/* Enable ready time expiry policy (?)*/;
pub const AR5K_TXQ_FLAG_FRAG_BURST_BACKOFF_ENABLE: c_uint = 0x0800	/* Enable backoff while bursting */;
pub const AR5K_TXQ_FLAG_POST_FR_BKOFF_DIS: c_uint = 0x1000	/* Disable backoff while bursting */;
pub const AR5K_TXQ_FLAG_COMPRESSION_ENABLE: c_uint = 0x2000	/* Enable hw compression -not implemented-*/;
//
// struct ath5k_txq - Transmit queue state
// @qnum: Hardware q number
// @link: Link ptr in last TX desc
// @q: Transmit queue (&struct list_head)
// @lock: Lock on q and link
// @setup: Is the queue configured
// @txq_len:Number of queued buffers
// @txq_max: Max allowed num of queued buffers
// @txq_poll_mark: Used to check if queue got stuck
// @txq_stuck: Queue stuck counter
//
// One of these exists for each hardware transmit queue.
// Packets sent to us from above are assigned to queues based
// on their priority.  Not all devices support a complete set
// of hardware transmit queues. For those devices the array
// sc_ac2q will map multiple priorities to fewer hardware queues
// (typically all to one hardware queue).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_txq {
    pub qnum: c_uint,
    pub link: *mut u32,
    pub q: list_head,
    pub lock: spinlock_t,
    pub setup: bool,
    pub txq_len: c_int,
    pub txq_max: c_int,
    pub txq_poll_mark: bool,
    pub txq_stuck: c_uint,
}

//
// struct ath5k_txq_info - A struct to hold TX queue's parameters
// @tqi_type: One of enum ath5k_tx_queue
// @tqi_subtype: One of enum ath5k_tx_queue_subtype
// @tqi_flags: TX queue flags (see above)
// @tqi_aifs: Arbitrated Inter-frame Space
// @tqi_cw_min: Minimum Contention Window
// @tqi_cw_max: Maximum Contention Window
// @tqi_cbr_period: Constant bit rate period
// @tqi_ready_time: Time queue waits after an event when RDYTIME is enabled
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_txq_info {
    pub tqi_type: ath5k_tx_queue,
    pub tqi_subtype: ath5k_tx_queue_subtype,
    pub tqi_flags: u16,
    pub tqi_aifs: u8,
    pub tqi_cw_min: u16,
    pub tqi_cw_max: u16,
    pub tqi_cbr_period: u32,
    pub tqi_cbr_overflow_limit: u32,
    pub tqi_burst_time: u32,
    pub tqi_ready_time: u32,
}

//
// enum ath5k_pkt_type - Transmit packet types
// @AR5K_PKT_TYPE_NORMAL: Normal data
// @AR5K_PKT_TYPE_ATIM: ATIM
// @AR5K_PKT_TYPE_PSPOLL: PS-Poll
// @AR5K_PKT_TYPE_BEACON: Beacon
// @AR5K_PKT_TYPE_PROBE_RESP: Probe response
// @AR5K_PKT_TYPE_PIFS: PIFS
// Used on tx control descriptor
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_pkt_type {
    AR5K_PKT_TYPE_NORMAL		= 0,
    AR5K_PKT_TYPE_ATIM		= 1,
    AR5K_PKT_TYPE_PSPOLL		= 2,
    AR5K_PKT_TYPE_BEACON		= 3,
    AR5K_PKT_TYPE_PROBE_RESP	= 4,
    AR5K_PKT_TYPE_PIFS		= 5,
}

//
// TX power and TPC settings
//

// \
//
// struct ath5k_rx_status - RX Status descriptor
// @rs_datalen: Data length
// @rs_tstamp: Timestamp
// @rs_status: Status code
// @rs_phyerr: PHY error mask
// @rs_rssi: RSSI in 0.5dbm units
// @rs_keyix: Index to the key used for decrypting
// @rs_rate: Rate used to decode the frame
// @rs_antenna: Antenna used to receive the frame
// @rs_more: Indicates this is a frame fragment (Fast frames)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_rx_status {
    pub rs_datalen: u16,
    pub rs_tstamp: u16,
    pub rs_status: u8,
    pub rs_phyerr: u8,
    pub rs_rssi: i8,
    pub rs_keyix: u8,
    pub rs_rate: u8,
    pub rs_antenna: u8,
    pub rs_more: u8,
}

pub const AR5K_RXERR_CRC: c_uint = 0x01;
pub const AR5K_RXERR_PHY: c_uint = 0x02;
pub const AR5K_RXERR_FIFO: c_uint = 0x04;
pub const AR5K_RXERR_DECRYPT: c_uint = 0x08;
pub const AR5K_RXERR_MIC: c_uint = 0x10;

// \
pub const AR5K_BEACON_PERIOD: c_uint = 0x0000ffff;
pub const AR5K_BEACON_ENA: c_uint = 0x00800000 /*enable beacon xmit*/;
pub const AR5K_BEACON_RESET_TSF: c_uint = 0x01000000 /*force a TSF reset*/;
//
// TSF to TU conversion:
//
// TSF is a 64bit value in usec (microseconds).
// TU is a 32bit value and defined by IEEE802.11 (page 6) as "A measurement of
// time equal to 1024 usec", so it's roughly milliseconds (usec / 1024).
//

// \
//
// enum ath5k_rfgain - RF Gain optimization engine state
// @AR5K_RFGAIN_INACTIVE: Engine disabled
// @AR5K_RFGAIN_ACTIVE: Probe active
// @AR5K_RFGAIN_READ_REQUESTED: Probe requested
// @AR5K_RFGAIN_NEED_CHANGE: Gain_F needs change
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_rfgain {
    AR5K_RFGAIN_INACTIVE = 0,
    AR5K_RFGAIN_ACTIVE,
    AR5K_RFGAIN_READ_REQUESTED,
    AR5K_RFGAIN_NEED_CHANGE,
}

//
// struct ath5k_gain - RF Gain optimization engine state data
// @g_step_idx: Current step index
// @g_current: Current gain
// @g_target: Target gain
// @g_low: Low gain boundary
// @g_high: High gain boundary
// @g_f_corr: Gain_F correction
// @g_state: One of enum ath5k_rfgain
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_gain {
    pub g_step_idx: u8,
    pub g_current: u8,
    pub g_target: u8,
    pub g_low: u8,
    pub g_high: u8,
    pub g_f_corr: u8,
    pub g_state: u8,
}

// \
pub const AR5K_SLOT_TIME_9: c_int = 396;
pub const AR5K_SLOT_TIME_20: c_int = 880;
pub const AR5K_SLOT_TIME_MAX: c_uint = 0xffff;
//
// struct ath5k_athchan_2ghz - 2GHz to 5GHZ map for RF5111
// @a2_flags: Channel flags (internal)
// @a2_athchan: HW channel number (internal)
//
// This structure is used to map 2GHz channels to
// 5GHz Atheros channels on 2111 frequency converter
// that comes together with RF5111
// TODO: Clean up
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_athchan_2ghz {
    pub a2_flags: u32,
    pub a2_athchan: u16,
}

//
// enum ath5k_dmasize -  DMA size definitions (2^(n+2))
// @AR5K_DMASIZE_4B: 4Bytes
// @AR5K_DMASIZE_8B: 8Bytes
// @AR5K_DMASIZE_16B: 16Bytes
// @AR5K_DMASIZE_32B: 32Bytes
// @AR5K_DMASIZE_64B: 64Bytes (Default)
// @AR5K_DMASIZE_128B: 128Bytes
// @AR5K_DMASIZE_256B: 256Bytes
// @AR5K_DMASIZE_512B: 512Bytes
//
// These are used to set DMA burst size on hw
//
// Note: Some platforms can't handle more than 4Bytes
// be careful on embedded boards.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_dmasize {
    AR5K_DMASIZE_4B	= 0,
    AR5K_DMASIZE_8B,
    AR5K_DMASIZE_16B,
    AR5K_DMASIZE_32B,
    AR5K_DMASIZE_64B,
    AR5K_DMASIZE_128B,
    AR5K_DMASIZE_256B,
    AR5K_DMASIZE_512B
}

// \
//
// DOC: Rate codes
//
// Seems the ar5xxx hardware supports up to 32 rates, indexed by 1-32.
//
// The rate code is used to get the RX rate or set the TX rate on the
// hardware descriptors. It is also used for internal modulation control
// and settings.
//
// This is the hardware rate map we are aware of (html unfriendly):
//
// Rate code	Rate (Kbps)
// ---------	-----------
// 0x01		 3000 (XR)
// 0x02		 1000 (XR)
// 0x03		  250 (XR)
// 0x04 - 05	-Reserved-
// 0x06		 2000 (XR)
// 0x07		  500 (XR)
// 0x08		48000 (OFDM)
// 0x09		24000 (OFDM)
// 0x0A		12000 (OFDM)
// 0x0B		 6000 (OFDM)
// 0x0C		54000 (OFDM)
// 0x0D		36000 (OFDM)
// 0x0E		18000 (OFDM)
// 0x0F		 9000 (OFDM)
// 0x10 - 17	-Reserved-
// 0x18		11000L (CCK)
// 0x19		 5500L (CCK)
// 0x1A		 2000L (CCK)
// 0x1B		 1000L (CCK)
// 0x1C		11000S (CCK)
// 0x1D		 5500S (CCK)
// 0x1E		 2000S (CCK)
// 0x1F		-Reserved-
//
// "S" indicates CCK rates with short preamble and "L" with long preamble.
//
// AR5211 has different rate codes for CCK (802.11B) rates. It only uses the
// lowest 4 bits, so they are the same as above with a 0xF mask.
// (0xB, 0xA, 0x9 and 0x8 for 1M, 2M, 5.5M and 11M).
// We handle this in ath5k_setup_bands().
//
pub const AR5K_MAX_RATES: c_int = 32;
// B
pub const ATH5K_RATE_CODE_1M: c_uint = 0x1B;
pub const ATH5K_RATE_CODE_2M: c_uint = 0x1A;
pub const ATH5K_RATE_CODE_5_5M: c_uint = 0x19;
pub const ATH5K_RATE_CODE_11M: c_uint = 0x18;
// A and G
pub const ATH5K_RATE_CODE_6M: c_uint = 0x0B;
pub const ATH5K_RATE_CODE_9M: c_uint = 0x0F;
pub const ATH5K_RATE_CODE_12M: c_uint = 0x0A;
pub const ATH5K_RATE_CODE_18M: c_uint = 0x0E;
pub const ATH5K_RATE_CODE_24M: c_uint = 0x09;
pub const ATH5K_RATE_CODE_36M: c_uint = 0x0D;
pub const ATH5K_RATE_CODE_48M: c_uint = 0x08;
pub const ATH5K_RATE_CODE_54M: c_uint = 0x0C;
// Adding this flag to rate_code on B rates
// enables short preamble
pub const AR5K_SET_SHORT_PREAMBLE: c_uint = 0x04;
//
// Crypto definitions
//
pub const AR5K_KEYCACHE_SIZE: c_int = 8;
// \
//
// Misc definitions
//

//
// Hardware interrupt abstraction
//
// enum ath5k_int - Hardware interrupt masks helpers
// @AR5K_INT_RXOK: Frame successfully received
// @AR5K_INT_RXDESC: Request RX descriptor/Read RX descriptor
// @AR5K_INT_RXERR: Frame reception failed
// @AR5K_INT_RXNOFRM: No frame received within a specified time period
// @AR5K_INT_RXEOL: Reached "End Of List", means we need more RX descriptors
// @AR5K_INT_RXORN: Indicates we got RX FIFO overrun. Note that Rx overrun is
// not always fatal, on some chips we can continue operation
// without resetting the card, that's why %AR5K_INT_FATAL is not
// common for all chips.
// @AR5K_INT_RX_ALL: Mask to identify all RX related interrupts
//
// @AR5K_INT_TXOK: Frame transmission success
// @AR5K_INT_TXDESC: Request TX descriptor/Read TX status descriptor
// @AR5K_INT_TXERR: Frame transmission failure
// @AR5K_INT_TXEOL: Received End Of List for VEOL (Virtual End Of List). The
// Queue Control Unit (QCU) signals an EOL interrupt only if a
// descriptor's LinkPtr is NULL. For more details, refer to:
// "http://www.freepatentsonline.com/20030225739.html"
// @AR5K_INT_TXNOFRM: No frame was transmitted within a specified time period
// @AR5K_INT_TXURN: Indicates we got TX FIFO underrun. In such case we should
// increase the TX trigger threshold.
// @AR5K_INT_TX_ALL: Mask to identify all TX related interrupts
//
// @AR5K_INT_MIB: Indicates the either Management Information Base counters or
// one of the PHY error counters reached the maximum value and
// should be read and cleared.
// @AR5K_INT_SWI: Software triggered interrupt.
// @AR5K_INT_RXPHY: RX PHY Error
// @AR5K_INT_RXKCM: RX Key cache miss
// @AR5K_INT_SWBA: SoftWare Beacon Alert - indicates its time to send a
// beacon that must be handled in software. The alternative is if
// you have VEOL support, in that case you let the hardware deal
// with things.
// @AR5K_INT_BRSSI: Beacon received with an RSSI value below our threshold
// @AR5K_INT_BMISS: If in STA mode this indicates we have stopped seeing
// beacons from the AP have associated with, we should probably
// try to reassociate. When in IBSS mode this might mean we have
// not received any beacons from any local stations. Note that
// every station in an IBSS schedules to send beacons at the
// Target Beacon Transmission Time (TBTT) with a random backoff.
// @AR5K_INT_BNR: Beacon queue got triggered (DMA beacon alert) while empty.
// @AR5K_INT_TIM: Beacon with local station's TIM bit set
// @AR5K_INT_DTIM: Beacon with DTIM bit and zero DTIM count received
// @AR5K_INT_DTIM_SYNC: DTIM sync lost
// @AR5K_INT_GPIO: GPIO interrupt is used for RF Kill switches connected to
// our GPIO pins.
// @AR5K_INT_BCN_TIMEOUT: Beacon timeout, we waited after TBTT but got noting
// @AR5K_INT_CAB_TIMEOUT: We waited for CAB traffic after the beacon but got
// nothing or an incomplete CAB frame sequence.
// @AR5K_INT_QCBRORN: A queue got it's CBR counter expired
// @AR5K_INT_QCBRURN: A queue got triggered wile empty
// @AR5K_INT_QTRIG: A queue got triggered
//
// @AR5K_INT_FATAL: Fatal errors were encountered, typically caused by bus/DMA
// errors. Indicates we need to reset the card.
// @AR5K_INT_GLOBAL: Used to clear and set the IER
// @AR5K_INT_NOCARD: Signals the card has been removed
// @AR5K_INT_COMMON: Common interrupts shared among MACs with the same
// bit value
//
// These are mapped to take advantage of some common bits
// between the MACs, to be able to set intr properties
// easier. Some of them are not used yet inside hw.c. Most map
// to the respective hw interrupt value as they are common among different
// MACs.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_int {
    AR5K_INT_RXOK	= 0x00000001,
    AR5K_INT_RXDESC	= 0x00000002,
    AR5K_INT_RXERR	= 0x00000004,
    AR5K_INT_RXNOFRM = 0x00000008,
    AR5K_INT_RXEOL	= 0x00000010,
    AR5K_INT_RXORN	= 0x00000020,
    AR5K_INT_TXOK	= 0x00000040,
    AR5K_INT_TXDESC	= 0x00000080,
    AR5K_INT_TXERR	= 0x00000100,
    AR5K_INT_TXNOFRM = 0x00000200,
    AR5K_INT_TXEOL	= 0x00000400,
    AR5K_INT_TXURN	= 0x00000800,
    AR5K_INT_MIB	= 0x00001000,
    AR5K_INT_SWI	= 0x00002000,
    AR5K_INT_RXPHY	= 0x00004000,
    AR5K_INT_RXKCM	= 0x00008000,
    AR5K_INT_SWBA	= 0x00010000,
    AR5K_INT_BRSSI	= 0x00020000,
    AR5K_INT_BMISS	= 0x00040000,
    AR5K_INT_FATAL	= 0x00080000, /* Non common */
    AR5K_INT_BNR	= 0x00100000, /* Non common */
    AR5K_INT_TIM	= 0x00200000, /* Non common */
    AR5K_INT_DTIM	= 0x00400000, /* Non common */
    AR5K_INT_DTIM_SYNC =	0x00800000, /* Non common */
    AR5K_INT_GPIO	=	0x01000000,
    AR5K_INT_BCN_TIMEOUT =	0x02000000, /* Non common */
    AR5K_INT_CAB_TIMEOUT =	0x04000000, /* Non common */
    AR5K_INT_QCBRORN =	0x08000000, /* Non common */
    AR5K_INT_QCBRURN =	0x10000000, /* Non common */
    AR5K_INT_QTRIG	=	0x20000000, /* Non common */
    AR5K_INT_GLOBAL =	0x80000000,

    AR5K_INT_TX_ALL = AR5K_INT_TXOK
    | AR5K_INT_TXDESC
    | AR5K_INT_TXERR
    | AR5K_INT_TXNOFRM
    | AR5K_INT_TXEOL
    | AR5K_INT_TXURN,

    AR5K_INT_RX_ALL = AR5K_INT_RXOK
    | AR5K_INT_RXDESC
    | AR5K_INT_RXERR
    | AR5K_INT_RXNOFRM
    | AR5K_INT_RXEOL
    | AR5K_INT_RXORN,

    AR5K_INT_COMMON  = AR5K_INT_RXOK
    | AR5K_INT_RXDESC
    | AR5K_INT_RXERR
    | AR5K_INT_RXNOFRM
    | AR5K_INT_RXEOL
    | AR5K_INT_RXORN
    | AR5K_INT_TXOK
    | AR5K_INT_TXDESC
    | AR5K_INT_TXERR
    | AR5K_INT_TXNOFRM
    | AR5K_INT_TXEOL
    | AR5K_INT_TXURN
    | AR5K_INT_MIB
    | AR5K_INT_SWI
    | AR5K_INT_RXPHY
    | AR5K_INT_RXKCM
    | AR5K_INT_SWBA
    | AR5K_INT_BRSSI
    | AR5K_INT_BMISS
    | AR5K_INT_GPIO
    | AR5K_INT_GLOBAL,

    AR5K_INT_NOCARD	= 0xffffffff
}

//
// enum ath5k_calibration_mask - Mask which calibration is active at the moment
// @AR5K_CALIBRATION_FULL: Full calibration (AGC + SHORT)
// @AR5K_CALIBRATION_SHORT: Short calibration (NF + I/Q)
// @AR5K_CALIBRATION_NF: Noise Floor calibration
// @AR5K_CALIBRATION_ANI: Adaptive Noise Immunity
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_calibration_mask {
    AR5K_CALIBRATION_FULL = 0x01,
    AR5K_CALIBRATION_SHORT = 0x02,
    AR5K_CALIBRATION_NF = 0x04,
    AR5K_CALIBRATION_ANI = 0x08,
}

//
// enum ath5k_power_mode - Power management modes
// @AR5K_PM_UNDEFINED: Undefined
// @AR5K_PM_AUTO: Allow card to sleep if possible
// @AR5K_PM_AWAKE: Force card to wake up
// @AR5K_PM_FULL_SLEEP: Force card to full sleep (DANGEROUS)
// @AR5K_PM_NETWORK_SLEEP: Allow to sleep for a specified duration
//
// Currently only PM_AWAKE is used, FULL_SLEEP and NETWORK_SLEEP/AUTO
// are also known to have problems on some cards. This is not a big
// problem though because we can have almost the same effect as
// FULL_SLEEP by putting card on warm reset (it's almost powered down).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_power_mode {
    AR5K_PM_UNDEFINED = 0,
    AR5K_PM_AUTO,
    AR5K_PM_AWAKE,
    AR5K_PM_FULL_SLEEP,
    AR5K_PM_NETWORK_SLEEP,
}

//
// These match net80211 definitions (not used in
// mac80211).
// TODO: Clean this up
//

// GPIO-controlled software LED
pub const AR5K_SOFTLED_PIN: c_int = 0;
pub const AR5K_SOFTLED_ON: c_int = 0;
pub const AR5K_SOFTLED_OFF: c_int = 1;
// XXX: we *may* move cap_range stuff to struct wiphy
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_capabilities {
//
// Supported PHY modes
// (ie. AR5K_MODE_11A, AR5K_MODE_11B, ...)
//
    pub AR5K_MODE_MAX): DECLARE_BITMAP(cap_mode,,
//
// Frequency range (without regulation restrictions)
//
    pub range_2ghz_min: u16,
    pub range_2ghz_max: u16,
    pub range_5ghz_min: u16,
    pub range_5ghz_max: u16,
    pub cap_range: },
//
// Values stored in the EEPROM (some of them...)
//
    pub cap_eeprom: ath5k_eeprom_info,
//
// Queue information
//
    pub q_tx_num: u8,
    pub cap_queues: },
    pub cap_has_phyerr_counters: bool,
    pub cap_has_mrr_support: bool,
    pub cap_needs_2GHz_ovr: bool,
}

// size of noise floor history (keep it a power of two)
pub const ATH5K_NF_CAL_HIST_MAX: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_nfcal_hist {
    pub /: *mut *mut s16 index; / current index into nfval,
    pub /: *mut *mut s16 nfval[ATH5K_NF_CAL_HIST_MAX]; / last few noise floors,
}

pub const ATH5K_LED_MAX_NAME_LEN: c_int = 31;
//
// State for LED triggers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_led {
    pub /: *mut *mut char name[ATH5K_LED_MAX_NAME_LEN + 1]; / name of the LED in sysfs,
    pub /: *mut *mut *mut ath5k_hw ah; / driver state,
    pub /: *mut *mut led_classdev led_dev; / led classdev,
}

// Rfkill
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_rfkill {
// GPIO PIN for rfkill
    pub gpio: u16,
// polarity of rfkill GPIO PIN
    pub polarity: bool,
// RFKILL toggle tasklet
    pub toggleq: tasklet_struct,
}

// statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_statistics {
// antenna use
    pub /: *mut *mut unsigned int antenna_rx[5]; / frames count per antenna RX,
    pub /: *mut *mut unsigned int antenna_tx[5]; / frames count per antenna TX,
// frame errors
    pub /: *mut *mut unsigned int rx_all_count; / all RX frames, including errors,
    pub /: *mut *mut unsigned int tx_all_count; / all TX frames, including errors,
    pub pkts: *mut *mut unsigned int rx_bytes_count; / all RX bytes, including errored,
// and the MAC headers for each packet
//
    pub pkts: *mut *mut unsigned int tx_bytes_count; / all TX bytes, including errored,
// and the MAC headers and padding for
// each packet.
//
    pub rxerr_crc: c_uint,
    pub rxerr_phy: c_uint,
    pub rxerr_phy_code: [c_uint; 32],
    pub rxerr_fifo: c_uint,
    pub rxerr_decrypt: c_uint,
    pub rxerr_mic: c_uint,
    pub rxerr_proc: c_uint,
    pub rxerr_jumbo: c_uint,
    pub txerr_retry: c_uint,
    pub txerr_fifo: c_uint,
    pub txerr_filt: c_uint,
// MIB counters
    pub ack_fail: c_uint,
    pub rts_fail: c_uint,
    pub rts_ok: c_uint,
    pub fcs_error: c_uint,
    pub beacons: c_uint,
    pub mib_intr: c_uint,
    pub rxorn_intr: c_uint,
    pub rxeol_intr: c_uint,
}

//
// Misc defines
//
pub const AR5K_MAX_GPIO: c_int = 10;
pub const AR5K_MAX_RF_BANKS: c_int = 8;

// Driver state associated with an instance of a device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_hw {
    pub common: ath_common,
    pub pdev: *mut pci_dev,
    pub /: *mut *mut *mut device dev; / for dma mapping,
    pub irq: c_int,
    pub devid: u16,
    pub /: *mut *mut *mut void __iomem iobase; / address of the device,
    pub /: *mut *mut mutex lock; / dev-level lock,
    pub /: *mut *mut *mut ieee80211_hw hw; / IEEE 802.11 common,
    pub sbands: [ieee80211_supported_band; NUM_NL80211_BANDS],
    pub channels: [ieee80211_channel; ATH_CHAN_MAX],
    pub rates: [ieee80211_rate; NUM_NL80211_BANDS][AR5K_MAX_RATES],
    pub rate_idx: [i8; NUM_NL80211_BANDS][AR5K_MAX_RATES],
    pub opmode: nl80211_iftype,

    pub /: *mut *mut ath5k_dbg_info debug; / debug info,

    pub /: *mut *mut *mut ath5k_buf bufptr; / allocated buffer ptr,
    pub /: *mut *mut *mut ath5k_desc desc; / TX/RX descriptors,
    pub /: *mut *mut dma_addr_t desc_daddr; / DMA (physical) address,
    pub /: *mut *mut size_t desc_len; / size of TX/RX descriptors,
    pub 4): DECLARE_BITMAP(status,,

    pub /: *mut *mut *mut unsigned int filter_flags; / HW flags, AR5K_RX_FILTER_,
    pub /: *mut *mut *mut unsigned int fif_filter_flags; / Current FIF_ filter flags,
    pub /: *mut *mut *mut ieee80211_channel curchan; / current h/w channel,
    pub nvifs: u16,
    pub /: *mut *mut ath5k_int imask; / interrupt mask copy,
    pub irqlock: spinlock_t,
    pub /: *mut *mut bool rx_pending; / rx tasklet pending,
    pub /: *mut *mut bool tx_pending; / tx tasklet pending,
    pub bssidmask: [u8; ETH_ALEN],
    pub /: *mut *mut led_on; / pin setting for LED on,
    pub /: *mut *mut work_reset_work; / deferred chip reset,
    pub /: *mut *mut work_calib_work; / deferred phy calibration,
    pub /: *mut *mut list_head rxbuf; / receive buffer,
    pub rxbuflock: spinlock_t,
    pub /: *mut *mut *mut u32 rxlink; / link ptr in last RX desc,
    pub /: *mut *mut tasklet_rxtq; / rx intr tasklet,
    pub /: *mut *mut ath5k_led rx_led; / rx led,
    pub /: *mut *mut list_head txbuf; / transmit buffer,
    pub txbuflock: spinlock_t,
    pub /: *mut *mut unsigned int txbuf_len; / buf count in txbuf list,
    pub /: *mut *mut ath5k_txq txqs[AR5K_NUM_TX_QUEUES]; / tx queues,
    pub /: *mut *mut tasklet_txtq; / tx intr tasklet,
    pub /: *mut *mut ath5k_led tx_led; / tx led,
    pub rf_kill: ath5k_rfkill,
    pub /: *mut *mut spinlock_t block; / protects beacon,
    pub /: *mut *mut tasklet_beacontq; / beacon intr tasklet,
    pub /: *mut *mut list_head bcbuf; / beacon buffer,
    pub bslot: [*mut ieee80211_vif; ATH_BCBUF],
    pub num_ap_vifs: u16,
    pub num_adhoc_vifs: u16,
    pub num_mesh_vifs: u16,
    pub /: *mut *mut unsigned int nexttbtt; / next beacon time in TU,
    pub /: *mut *mut *mut ath5k_txq cabq; / content after beacon,
    pub /: *mut *mut bool assoc; / associate state,
    pub /: *mut *mut bool enable_beacon; / true if beacons are on,
    pub stats: ath5k_statistics,
    pub ani_state: ath5k_ani_state,
    pub /: *mut *mut tasklet_ani_tasklet; / ANI calibration,
    pub tx_complete_work: delayed_work,
    pub /: *mut *mut survey_info survey; / collected survey info,
    pub ah_imr: ath5k_int,
    pub ah_current_channel: *mut ieee80211_channel,
    pub ah_iq_cal_needed: bool,
    pub ah_single_chip: bool,
    pub ah_version: ath5k_version,
    pub ah_radio: ath5k_radio,
    pub ah_mac_srev: u32,
    pub ah_mac_version: u16,
    pub ah_phy_revision: u16,
    pub ah_radio_5ghz_revision: u16,
    pub ah_radio_2ghz_revision: u16,

    pub ah_retry_long: u8,
    pub ah_retry_short: u8,
    pub ah_use_32khz_clock: bool,
    pub ah_coverage_class: u8,
    pub ah_ack_bitrate_high: bool,
    pub ah_bwmode: u8,
    pub ah_short_slot: bool,
// Antenna Control
    pub ah_ant_ctl: [u32; AR5K_EEPROM_N_MODES][AR5K_ANT_MAX],
    pub ah_ant_mode: u8,
    pub ah_tx_ant: u8,
    pub ah_def_ant: u8,
    pub ah_capabilities: ath5k_capabilities,
    pub ah_txq: [ath5k_txq_info; AR5K_NUM_TX_QUEUES],
    pub ah_txq_status: u32,
    pub ah_txq_imr_txok: u32,
    pub ah_txq_imr_txerr: u32,
    pub ah_txq_imr_txurn: u32,
    pub ah_txq_imr_txdesc: u32,
    pub ah_txq_imr_txeol: u32,
    pub ah_txq_imr_cbrorn: u32,
    pub ah_txq_imr_cbrurn: u32,
    pub ah_txq_imr_qtrig: u32,
    pub ah_txq_imr_nofrm: u32,
    pub ah_txq_isr_txok_all: u32,
    pub ah_rf_banks: *mut u32,
    pub ah_rf_banks_size: usize,
    pub ah_rf_regs_count: usize,
    pub ah_gain: ath5k_gain,
    pub ah_offset: [u8; AR5K_MAX_RF_BANKS],
// Temporary tables used for interpolation
    pub 2]: *mut *mut u8 txp_pd_table[AR5K_EEPROM_POWER_TABLE_SIZE,
    pub txp_rates_power_table: [u16; AR5K_MAX_RATES],
    pub txp_min_idx: u8,
    pub txp_tpc: bool,
// Values in 0.25dB units
    pub txp_min_pwr: i16,
    pub txp_max_pwr: i16,
    pub txp_cur_pwr: i16,
// Values in 0.5dB units
    pub txp_offset: i16,
    pub txp_ofdm: i16,
    pub txp_cck_ofdm_gainf_delta: i16,
// Value in dB units
    pub txp_cck_ofdm_pwr_delta: i16,
    pub txp_setup: bool,
    pub /: *mut *mut int txp_requested; / Requested tx power in dBm,
    pub ah_txpower: },
    pub ah_nfcal_hist: ath5k_nfcal_hist,
// average beacon RSSI in our BSS (used by ANI)
    pub ah_beacon_rssi_avg: ewma_beacon_rssi,
// noise floor from last periodic calibration
    pub ah_noise_floor: i32,
// Calibration timestamp
    pub ah_cal_next_full: c_ulong,
    pub ah_cal_next_short: c_ulong,
    pub ah_cal_next_ani: c_ulong,
// Calibration mask
    pub ah_cal_mask: u8,
//
// Function pointers
//
    pub int): unsigned int, unsigned int, unsigned int, unsigned,
    pub ): *mut ath5k_tx_status,
    pub ): *mut ath5k_rx_status,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath_bus_ops {
    pub ath_bus_type: ath_bus_type,
    pub csz): *mut *mut *mut void (read_cachesize)(struct ath_common common, int,
    pub data): *mut *mut *mut bool (eeprom_read)(struct ath_common common, u32 off, u16,
    pub mac): *mut *mut *mut int (eeprom_read_mac)(struct ath5k_hw ah, u8,
}

//
// Prototypes
//
// Initialization and detach functions
extern "C" {
    pub fn ath5k_hw_init(ah: *mut ath5k_hw) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_deinit(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_sysfs_register(ah: *mut ath5k_hw) -> c_int;
}
extern "C" {
    pub fn ath5k_sysfs_unregister(ah: *mut ath5k_hw);
}
// Chip id helper functions
extern "C" {
    pub fn ath5k_hw_read_srev(ah: *mut ath5k_hw) -> c_int;
}
// LED functions
extern "C" {
    pub fn ath5k_init_leds(ah: *mut ath5k_hw) -> c_int;
}
extern "C" {
    pub fn ath5k_led_enable(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_led_off(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_unregister_leds(ah: *mut ath5k_hw);
}
// Reset Functions
extern "C" {
    pub fn ath5k_hw_nic_wakeup(ah: *mut ath5k_hw, channel: *mut ieee80211_channel) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_on_hold(ah: *mut ath5k_hw) -> c_int;
}
// Power management functions
// Clock rate related functions
extern "C" {
    pub fn ath5k_hw_htoclock(ah: *mut ath5k_hw, usec: c_uint) -> c_uint;
}
extern "C" {
    pub fn ath5k_hw_clocktoh(ah: *mut ath5k_hw, clock: c_uint) -> c_uint;
}
extern "C" {
    pub fn ath5k_hw_set_clockrate(ah: *mut ath5k_hw);
}
// DMA Related Functions
extern "C" {
    pub fn ath5k_hw_start_rx_dma(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_hw_get_rxdp(ah: *mut ath5k_hw) -> u32;
}
extern "C" {
    pub fn ath5k_hw_set_rxdp(ah: *mut ath5k_hw, phys_addr: u32) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_start_tx_dma(ah: *mut ath5k_hw, queue: c_uint) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_stop_beacon_queue(ah: *mut ath5k_hw, queue: c_uint) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_get_txdp(ah: *mut ath5k_hw, queue: c_uint) -> u32;
}
extern "C" {
    pub fn ath5k_hw_update_tx_triglevel(ah: *mut ath5k_hw, increase: bool) -> c_int;
}
// Interrupt handling
extern "C" {
    pub fn ath5k_hw_is_intr_pending(ah: *mut ath5k_hw) -> bool;
}
extern "C" {
    pub fn ath5k_hw_get_isr(ah: *mut ath5k_hw, interrupt_mask: *mut ath5k_int) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_set_imr(ah: *mut ath5k_hw, new_mask: ath5k_int) -> ath5k_int;
}
extern "C" {
    pub fn ath5k_hw_update_mib_counters(ah: *mut ath5k_hw);
}
// Init/Stop functions
extern "C" {
    pub fn ath5k_hw_dma_init(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_hw_dma_stop(ah: *mut ath5k_hw) -> c_int;
}
// EEPROM access functions
extern "C" {
    pub fn ath5k_eeprom_init(ah: *mut ath5k_hw) -> c_int;
}
extern "C" {
    pub fn ath5k_eeprom_detach(ah: *mut ath5k_hw);
}
// Protocol Control Unit Functions
// Helpers
extern "C" {
    pub fn ath5k_hw_get_default_slottime(ah: *mut ath5k_hw) -> c_uint;
}
extern "C" {
    pub fn ath5k_hw_get_default_sifs(ah: *mut ath5k_hw) -> c_uint;
}
extern "C" {
    pub fn ath5k_hw_set_opmode(ah: *mut ath5k_hw, opmode: nl80211_iftype) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_set_coverage_class(ah: *mut ath5k_hw, coverage_class: u8);
}
// RX filter control
extern "C" {
    pub fn ath5k_hw_set_lladdr(ah: *mut ath5k_hw, mac: *const u8) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_set_bssid(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_hw_set_bssid_mask(ah: *mut ath5k_hw, mask: *const u8);
}
extern "C" {
    pub fn ath5k_hw_set_mcast_filter(ah: *mut ath5k_hw, filter0: u32, filter1: u32);
}
extern "C" {
    pub fn ath5k_hw_get_rx_filter(ah: *mut ath5k_hw) -> u32;
}
extern "C" {
    pub fn ath5k_hw_set_rx_filter(ah: *mut ath5k_hw, filter: u32);
}
// Receive (DRU) start/stop functions
extern "C" {
    pub fn ath5k_hw_start_rx_pcu(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_hw_stop_rx_pcu(ah: *mut ath5k_hw);
}
// Beacon control functions
extern "C" {
    pub fn ath5k_hw_get_tsf64(ah: *mut ath5k_hw) -> u64;
}
extern "C" {
    pub fn ath5k_hw_set_tsf64(ah: *mut ath5k_hw, tsf64: u64);
}
extern "C" {
    pub fn ath5k_hw_reset_tsf(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_hw_check_beacon_timers(ah: *mut ath5k_hw, intval: c_int) -> bool;
}
// Init function
extern "C" {
    pub fn ath5k_hw_pcu_init(ah: *mut ath5k_hw, op_mode: nl80211_iftype);
}
// Queue Control Unit, DFS Control Unit Functions
extern "C" {
    pub fn ath5k_hw_num_tx_pending(ah: *mut ath5k_hw, queue: c_uint) -> u32;
}
extern "C" {
    pub fn ath5k_hw_release_tx_queue(ah: *mut ath5k_hw, queue: c_uint);
}
extern "C" {
    pub fn ath5k_hw_reset_tx_queue(ah: *mut ath5k_hw, queue: c_uint) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_set_ifs_intervals(ah: *mut ath5k_hw, slot_time: c_uint) -> c_int;
}
// Init function
extern "C" {
    pub fn ath5k_hw_init_queues(ah: *mut ath5k_hw) -> c_int;
}
// Hardware Descriptor Functions
extern "C" {
    pub fn ath5k_hw_init_desc_functions(ah: *mut ath5k_hw) -> c_int;
}
// GPIO Functions
extern "C" {
    pub fn ath5k_hw_set_ledstate(ah: *mut ath5k_hw, state: c_uint);
}
extern "C" {
    pub fn ath5k_hw_set_gpio_input(ah: *mut ath5k_hw, gpio: u32) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_set_gpio_output(ah: *mut ath5k_hw, gpio: u32) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_get_gpio(ah: *mut ath5k_hw, gpio: u32) -> u32;
}
extern "C" {
    pub fn ath5k_hw_set_gpio(ah: *mut ath5k_hw, gpio: u32, val: u32) -> c_int;
}
// RFkill Functions
extern "C" {
    pub fn ath5k_rfkill_hw_start(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_rfkill_hw_stop(ah: *mut ath5k_hw);
}
// Misc functions TODO: Cleanup
extern "C" {
    pub fn ath5k_hw_set_capabilities(ah: *mut ath5k_hw) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_enable_pspoll(ah: *mut ath5k_hw, bssid: *mut u8, assoc_id: u16) -> c_int;
}
extern "C" {
    pub fn ath5k_hw_disable_pspoll(ah: *mut ath5k_hw) -> c_int;
}
// Initial register settings functions
extern "C" {
    pub fn ath5k_hw_write_initvals(ah: *mut ath5k_hw, mode: u8, change_channel: bool) -> c_int;
}
// PHY functions
// Misc PHY functions
extern "C" {
    pub fn ath5k_hw_radio_revision(ah: *mut ath5k_hw, band: nl80211_band) -> u16;
}
extern "C" {
    pub fn ath5k_hw_phy_disable(ah: *mut ath5k_hw) -> c_int;
}
// Gain_F optimization
extern "C" {
    pub fn ath5k_hw_gainf_calibrate(ah: *mut ath5k_hw) -> ath5k_rfgain;
}
extern "C" {
    pub fn ath5k_hw_rfgain_opt_init(ah: *mut ath5k_hw) -> c_int;
}
// PHY/RF channel functions
extern "C" {
    pub fn ath5k_channel_ok(ah: *mut ath5k_hw, channel: *mut ieee80211_channel) -> bool;
}
// PHY calibration
extern "C" {
    pub fn ath5k_hw_init_nfcal_hist(ah: *mut ath5k_hw);
}
extern "C" {
    pub fn ath5k_hw_update_noise_floor(ah: *mut ath5k_hw);
}
// Spur mitigation
// Antenna control
extern "C" {
    pub fn ath5k_hw_set_antenna_mode(ah: *mut ath5k_hw, ant_mode: u8);
}
extern "C" {
    pub fn ath5k_hw_set_antenna_switch(ah: *mut ath5k_hw, ee_mode: u8);
}
// TX power setup
extern "C" {
    pub fn ath5k_hw_set_txpower_limit(ah: *mut ath5k_hw, txpower: u8) -> c_int;
}
// Init function
//
// Functions used internally
//

// On AR2315 and AR2317 the PCI clock domain registers
// are outside of the WMAC register space
extern "C" {
    pub fn ioread32(_arg: ath5k_ahb_reg(ah, _arg: reg)) -> return;
}

extern "C" {
    pub fn ioread32(reg: ah->iobase +) -> return;
}

