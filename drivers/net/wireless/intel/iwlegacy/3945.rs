//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlegacy/3945.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright(c) 2003 - 2011 Intel Corporation. All rights reserved.
//
// Contact Information:
// Intel Linux Wireless <ilw@linux.intel.com>
// Intel Corporation, 5200 N.E. Elam Young Parkway, Hillsboro, OR 97124-6497
//

// Macro flag: #define __il_3945_h__

// Hardware specific file defines the PCI IDs table for that hardware module

// Highest firmware API version supported
pub const IL3945_UCODE_API_MAX: c_int = 2;
// Lowest firmware API version supported
pub const IL3945_UCODE_API_MIN: c_int = 1;

// Default noise level to report when noise measurement is not available.
// This may be because we're:
// 1)  Not associated (4965, no beacon stats being sent to driver)
// 2)  Scanning (noise measurement does not apply to associated channel)
// 3)  Receiving CCK (3945 delivers noise info only for OFDM frames)
// Use default noise value of -127 ... this is below the range of measurable
// Rx dBm for either 3945 or 4965, so it can indicate "unmeasurable" to user.
// Also, -127 works better than 0 when averaging frames with/without
// noise info (e.g. averaging might be done in app); measured dBm values are
// always negative ... using a negative value as the default keeps all
// averages within an s8's (used in some apps) range of negative values.

// Module parameters accessible from iwl-*.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_rate_scale_data {
    pub data: u64,
    pub success_counter: i32,
    pub success_ratio: i32,
    pub counter: i32,
    pub average_tpt: i32,
    pub stamp: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_rs_sta {
    pub lock: spinlock_t,
    pub il: *mut il_priv,
    pub expected_tpt: *mut i32,
    pub last_partial_flush: c_ulong,
    pub last_flush: c_ulong,
    pub flush_time: u32,
    pub last_tx_packets: u32,
    pub tx_packets: u32,
    pub tgg: u8,
    pub flush_pending: u8,
    pub start_rate: u8,
    pub rate_scale_flush: timer_list,
    pub win: [il3945_rate_scale_data; RATE_COUNT_3945],
// used to be in sta_info
    pub last_txrate_idx: c_int,
}

//
// The common struct MUST be first because it is shared between
// 3945 and 4965!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_sta_priv {
    pub common: il_station_priv_common,
    pub rs_sta: il3945_rs_sta,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum il3945_antenna {
    IL_ANTENNA_DIVERSITY,
    IL_ANTENNA_MAIN,
    IL_ANTENNA_AUX
}

//
// RTS threshold here is total size [2347] minus 4 FCS bytes
// Per spec:
// a value of 0 means RTS on all data/management packets
// a value > max MSDU size means no RTS
// else RTS for data/management frames where MPDU is larger
// than RTS value.
//

pub const IL_TX_FIFO_AC0: c_int = 0;
pub const IL_TX_FIFO_AC1: c_int = 1;
pub const IL_TX_FIFO_AC2: c_int = 2;
pub const IL_TX_FIFO_AC3: c_int = 3;
pub const IL_TX_FIFO_HCCA_1: c_int = 5;
pub const IL_TX_FIFO_HCCA_2: c_int = 6;
pub const IL_TX_FIFO_NONE: c_int = 7;
pub const IEEE80211_DATA_LEN: c_int = 2304;
pub const IEEE80211_4ADDR_LEN: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_frame {
    pub list: list_head,
// Must be last as it ends in a flexible-array member.
    pub frame: ieee80211_hdr,
    pub beacon: il3945_tx_beacon_cmd,
    pub raw: [u8; IEEE80211_FRAME_LEN],
    pub cmd: [u8; 360],
    pub u: },
}

pub const SUP_RATE_11A_MAX_NUM_CHANNELS: c_int = 8;
pub const SUP_RATE_11B_MAX_NUM_CHANNELS: c_int = 4;
pub const SUP_RATE_11G_MAX_NUM_CHANNELS: c_int = 12;
pub const IL_SUPPORTED_RATES_IE_LEN: c_int = 8;
pub const SCAN_INTERVAL: c_int = 100;
pub const MAX_TID_COUNT: c_int = 9;
pub const IL_INVALID_RATE: c_uint = 0xFF;

pub const STA_PS_STATUS_WAKE: c_int = 0;
pub const STA_PS_STATUS_SLEEP: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_ibss_seq {
    pub mac: [u8; ETH_ALEN],
    pub seq_num: u16,
    pub frag_num: u16,
    pub packet_time: c_ulong,
    pub list: list_head,
}

//
// Functions implemented in iwl3945-base.c which are forward declared here
// for use by iwl-*.c
//
extern "C" {
    pub fn il3945_rx_replenish(data: *mut c_void);
}
extern "C" {
    pub fn il3945_rx_queue_reset(il: *mut il_priv, rxq: *mut il_rx_queue);
}
extern "C" {
    pub fn il3945_dump_nic_error_log(il: *mut il_priv);
}
//
// Functions implemented in iwl-[34]*.c which are forward declared here
// for use by iwl3945-base.c
//
// NOTE:  The implementation of these functions are hardware specific
// which is why they are in the hardware specific files (vs. iwl-base.c)
//
// Naming convention --
// il3945_         <-- Its part of iwlwifi (should be changed to il3945_)
// il3945_hw_      <-- Hardware specific (implemented in iwl-XXXX.c by all HW)
// iwlXXXX_     <-- Hardware specific (implemented in iwl-XXXX.c for XXXX)
// il3945_bg_      <-- Called from work queue context
// il3945_mac_     <-- mac80211 callback
//
extern "C" {
    pub fn il3945_hw_handler_setup(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_hw_setup_deferred_work(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_hw_cancel_deferred_work(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_hw_rxq_stop(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_hw_set_hw_params(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_hw_nic_init(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_hw_nic_stop_master(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_hw_txq_ctx_free(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_hw_txq_ctx_stop(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_hw_nic_reset(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_hw_txq_free_tfd(il: *mut il_priv, txq: *mut il_tx_queue);
}
extern "C" {
    pub fn il3945_hw_get_temperature(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_hw_tx_queue_init(il: *mut il_priv, txq: *mut il_tx_queue) -> c_int;
}
extern "C" {
    pub fn il3945_hw_reg_send_txpower(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_hw_reg_set_txpower(il: *mut il_priv, power: i8) -> c_int;
}
extern "C" {
    pub fn il3945_hdl_stats(il: *mut il_priv, rxb: *mut il_rx_buf);
}
extern "C" {
    pub fn il3945_hdl_c_stats(il: *mut il_priv, rxb: *mut il_rx_buf);
}
extern "C" {
    pub fn il3945_disable_events(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_get_temperature(il: *const il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_post_associate(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_config_ap(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_commit_rxon(il: *mut il_priv) -> c_int;
}
//
// il3945_hw_find_station - Find station id for a given BSSID
// @bssid: MAC address of station ID to find
//
// NOTE:  This should not be hardware specific but the code has
// not yet been merged into a single common layer for managing the
// station tables.
//
extern "C" {
    pub fn il3945_hw_find_station(il: *mut il_priv, bssid: *const u8) -> u8;
}
extern "C" {
    pub fn il3945_get_antenna_flags(il: *const il_priv) -> __le32;
}
extern "C" {
    pub fn il3945_init_hw_rate_table(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_reg_txpower_periodic(il: *mut il_priv);
}
extern "C" {
    pub fn il3945_txpower_set_from_eeprom(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il3945_rs_next_rate(il: *mut il_priv, rate: c_int) -> c_int;
}
// scanning
extern "C" {
    pub fn il3945_request_scan(il: *mut il_priv, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn il3945_post_scan(il: *mut il_priv);
}
// rates
// RSSI to dBm
pub const IL39_RSSI_OFFSET: c_int = 95;
//
// EEPROM related constants, enums, and structures.
//

//
// Mapping of a Tx power level, at factory calibration temperature,
// to a radio/DSP gain table idx.
// One for each of 5 "sample" power levels in each band.
// v_det is measured at the factory, using the 3945's built-in power amplifier
// (PA) output voltage detector.  This same detector is used during Tx of
// long packets in normal operation to provide feedback as to proper output
// level.
// Data copied from EEPROM.
// DO NOT ALTER THIS STRUCTURE!!!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_eeprom_txpower_sample {
    pub /: *mut *mut u8 gain_idx; / idx into power (gain) setup table ...,
    pub /: *mut *mut s8 power; / ... for this pwr level for this chnl group,
    pub /: *mut *mut u16 v_det; / PA output voltage,
    pub __packed: },
//
// Mappings of Tx power levels -> nominal radio/DSP gain table idxes.
// One for each channel group (a.k.a. "band") (1 for BG, 4 for A).
// Tx power setup code interpolates between the 5 "sample" power levels
// to determine the nominal setup for a requested power level.
// Data copied from EEPROM.
// DO NOT ALTER THIS STRUCTURE!!!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_eeprom_txpower_group {
    pub /: *mut *mut il3945_eeprom_txpower_sample samples[5]; / 5 power levels,
    pub voltage->power: *mut *mut s32 a, b, c, d, e; / coefficients for,
// formula (signed)
    pub on: *mut *mut s32 Fa, Fb, Fc, Fd, Fe; / these modify coeffs based,
// frequency (signed)
    pub this: *mut *mut s8 saturation_power; / highest power possible by h/w in,
// band
    pub /: *mut *mut u8 group_channel; / "representative" channel # in this band,
    pub band: *mut *mut s16 temperature; / h/w temperature at factory calib this,
// (signed)
    pub __packed: },
//
// Temperature-based Tx-power compensation data, not band-specific.
// These coefficients are use to modify a/b/c/d/e coeffs based on
// difference between current temperature and factory calib temperature.
// Data copied from EEPROM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_eeprom_temperature_corr {
    pub Ta: u32,
    pub Tb: u32,
    pub Tc: u32,
    pub Td: u32,
    pub Te: u32,
    pub __packed: },
//
// EEPROM map
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_eeprom {
    pub reserved0: [u8; 16],
    pub /: *mut *mut u16 device_id; / abs.ofs: 16,
    pub reserved1: [u8; 2],
    pub /: *mut *mut u16 pmc; / abs.ofs: 20,
    pub reserved2: [u8; 20],
    pub /: *mut *mut u8 mac_address[6]; / abs.ofs: 42,
    pub reserved3: [u8; 58],
    pub /: *mut *mut u16 board_revision; / abs.ofs: 106,
    pub reserved4: [u8; 11],
    pub /: *mut *mut u8 board_pba_number[9]; / abs.ofs: 119,
    pub reserved5: [u8; 8],
    pub /: *mut *mut u16 version; / abs.ofs: 136,
    pub /: *mut *mut u8 sku_cap; / abs.ofs: 138,
    pub /: *mut *mut u8 leds_mode; / abs.ofs: 139,
    pub oem_mode: u16,
    pub /: *mut *mut u16 wowlan_mode; / abs.ofs: 142,
    pub /: *mut *mut u16 leds_time_interval; / abs.ofs: 144,
    pub /: *mut *mut u8 leds_off_time; / abs.ofs: 146,
    pub /: *mut *mut u8 leds_on_time; / abs.ofs: 147,
    pub /: *mut *mut u8 almgor_m_version; / abs.ofs: 148,
    pub /: *mut *mut u8 antenna_switch_type; / abs.ofs: 149,
    pub reserved6: [u8; 42],
    pub /: *mut *mut u8 sku_id[4]; / abs.ofs: 192,
//
// Per-channel regulatory data.
//
// Each channel that *might* be supported by 3945 has a fixed location
// in EEPROM containing EEPROM_CHANNEL_* usage flags (LSB) and max regulatory
// txpower (MSB).
//
// Entries immediately below are for 20 MHz channel width.
//
// 2.4 GHz channels 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
//
    pub /: *mut *mut u16 band_1_count; / abs.ofs: 196,
    pub /: *mut *mut il_eeprom_channel band_1_channels[14]; / abs.ofs: 198,
//
// 4.9 GHz channels 183, 184, 185, 187, 188, 189, 192, 196,
// 5.0 GHz channels 7, 8, 11, 12, 16
// (4915-5080MHz) (none of these is ever supported)
//
    pub /: *mut *mut u16 band_2_count; / abs.ofs: 226,
    pub /: *mut *mut il_eeprom_channel band_2_channels[13]; / abs.ofs: 228,
//
// 5.2 GHz channels 34, 36, 38, 40, 42, 44, 46, 48, 52, 56, 60, 64
// (5170-5320MHz)
//
    pub /: *mut *mut u16 band_3_count; / abs.ofs: 254,
    pub /: *mut *mut il_eeprom_channel band_3_channels[12]; / abs.ofs: 256,
//
// 5.5 GHz channels 100, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140
// (5500-5700MHz)
//
    pub /: *mut *mut u16 band_4_count; / abs.ofs: 280,
    pub /: *mut *mut il_eeprom_channel band_4_channels[11]; / abs.ofs: 282,
//
// 5.7 GHz channels 145, 149, 153, 157, 161, 165
// (5725-5825MHz)
//
    pub /: *mut *mut u16 band_5_count; / abs.ofs: 304,
    pub /: *mut *mut il_eeprom_channel band_5_channels[6]; / abs.ofs: 306,
    pub reserved9: [u8; 194],
//
// 3945 Txpower calibration data.
//
pub const IL_NUM_TX_CALIB_GROUPS: c_int = 5;
    pub groups: [il3945_eeprom_txpower_group; IL_NUM_TX_CALIB_GROUPS],
// abs.ofs: 512
    pub /: *mut *mut il3945_eeprom_temperature_corr corrections; / abs.ofs: 832,
    pub /: *mut *mut u8 reserved16[172]; / fill out to full 1024 byte block,
    pub __packed: },
pub const IL3945_EEPROM_IMG_SIZE: c_int = 1024;
// End of EEPROM

// 4 DATA + 1 CMD. There are 2 HCCA queues that are not used.
pub const IL39_NUM_QUEUES: c_int = 5;
pub const IL39_CMD_QUEUE_NUM: c_int = 4;
pub const IL_DEFAULT_TX_RETRY: c_int = 15;
//
pub const RFD_SIZE: c_int = 4;
pub const NUM_TFD_CHUNKS: c_int = 4;

// Sizes and addresses for instruction and data memory (SRAM) in
// 3945's embedded processor.  Driver access is via HBUS_TARG_MEM_* regs.

// Size of uCode instruction memory in bootstrap state machine

    pub IL39_RTC_DATA_UPPER_BOUND): addr <,
// Base physical address of il3945_shared is provided to FH39_TSSR_CBB_BASE
// and &il3945_shared.rx_read_ptr[0] is provided to FH39_RCSR_RPTR_ADDR(0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_shared {
    pub tx_base_ptr: [__le32; 8],
    pub __packed: },
//
// iwl3945 Flow Handler Definitions
//
// This I/O area is directly read/writable by driver (e.g. Linux uses writel())
// Addresses are offsets from device's PCI hardware base address.
//

// TFDB (Transmit Frame Buffer Descriptor)

// CBCC channel is [0,2]

// RCSR channel is [0,2]

// RSSR

// TCSR

// TSSR

// DBM

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_tfd_tb {
    pub addr: __le32,
    pub len: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_tfd {
    pub control_flags: __le32,
    pub tbs: [il3945_tfd_tb; 4],
    pub __pad: [u8; 28],
    pub __packed: },

    pub il3945_debugfs_ops: extern struct il_debugfs_ops,

