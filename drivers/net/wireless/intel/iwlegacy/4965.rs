//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlegacy/4965.h
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
// Copyright(c) 2008 - 2011 Intel Corporation. All rights reserved.
//
// Contact Information:
// Intel Linux Wireless <ilw@linux.intel.com>
// Intel Corporation, 5200 N.E. Elam Young Parkway, Hillsboro, OR 97124-6497
//

// Macro flag: #define __il_4965_h__
// configuration for the _4965 devices
// tx queue
// RXON
extern "C" {
    pub fn il4965_set_rxon_chain(il: *mut il_priv);
}
// uCode
extern "C" {
    pub fn il4965_verify_ucode(il: *mut il_priv) -> c_int;
}
// lib
extern "C" {
    pub fn il4965_check_abort_status(il: *mut il_priv, frame_count: u8, status: u32);
}
extern "C" {
    pub fn il4965_rx_queue_reset(il: *mut il_priv, rxq: *mut il_rx_queue);
}
extern "C" {
    pub fn il4965_rx_init(il: *mut il_priv, rxq: *mut il_rx_queue) -> c_int;
}
extern "C" {
    pub fn il4965_hw_nic_init(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il4965_dump_fh(il: *mut il_priv, buf: *mut c_char, display: bool) -> c_int;
}
extern "C" {
    pub fn il4965_nic_config(il: *mut il_priv);
}
// rx
extern "C" {
    pub fn il4965_rx_queue_restock(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_rx_replenish(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_rx_replenish_now(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_rx_queue_free(il: *mut il_priv, rxq: *mut il_rx_queue);
}
extern "C" {
    pub fn il4965_rxq_stop(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il4965_hwrate_to_mac80211_idx(rate_n_flags: u32, band: nl80211_band) -> c_int;
}
extern "C" {
    pub fn il4965_rx_handle(il: *mut il_priv);
}
// tx
extern "C" {
    pub fn il4965_hw_txq_free_tfd(il: *mut il_priv, txq: *mut il_tx_queue);
}
extern "C" {
    pub fn il4965_hw_tx_queue_init(il: *mut il_priv, txq: *mut il_tx_queue) -> c_int;
}
extern "C" {
    pub fn il4965_txq_check_empty(il: *mut il_priv, sta_id: c_int, tid: u8, txq_id: c_int) -> c_int;
}
extern "C" {
    pub fn il4965_tx_queue_reclaim(il: *mut il_priv, txq_id: c_int, idx: c_int) -> c_int;
}
extern "C" {
    pub fn il4965_hw_txq_ctx_free(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_txq_ctx_alloc(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il4965_txq_ctx_reset(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_txq_ctx_stop(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_txq_set_sched(il: *mut il_priv, mask: u32);
}
//
// Acquire il->lock before calling this function !
//
extern "C" {
    pub fn il4965_set_wr_ptrs(il: *mut il_priv, txq_id: c_int, idx: u32);
}
//
// il4965_tx_queue_set_status - (optionally) start Tx/Cmd queue
// @tx_fifo_id: Tx DMA/FIFO channel (range 0-7) that the queue will feed
// @scd_retry: (1) Indicates queue will be used in aggregation mode
//
// NOTE:  Acquire il->lock before calling this function !
//
// scan
extern "C" {
    pub fn il4965_request_scan(il: *mut il_priv, vif: *mut ieee80211_vif) -> c_int;
}
// station mgmt
// hcmd
extern "C" {
    pub fn il4965_send_beacon_cmd(il: *mut il_priv) -> c_int;
}

// station management
extern "C" {
    pub fn il4965_alloc_bcast_station(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il4965_add_bssid_station(il: *mut il_priv, addr: *const u8, sta_id_r: *mut u8) -> c_int;
}
extern "C" {
    pub fn il4965_restore_default_wep_keys(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il4965_sta_tx_modify_enable_tid(il: *mut il_priv, sta_id: c_int, tid: c_int) -> c_int;
}
extern "C" {
    pub fn il4965_sta_modify_sleep_tx_count(il: *mut il_priv, sta_id: c_int, cnt: c_int);
}
extern "C" {
    pub fn il4965_update_bcast_stations(il: *mut il_priv) -> c_int;
}
// rate
// eeprom
extern "C" {
    pub fn il4965_eeprom_get_mac(il: *const il_priv, mac: *mut *mut u8);
}
extern "C" {
    pub fn il4965_eeprom_acquire_semaphore(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il4965_eeprom_release_semaphore(il: *mut il_priv);
}
extern "C" {
    pub fn il4965_eeprom_check_version(il: *mut il_priv) -> c_int;
}
// mac80211 handlers (for 4965)
extern "C" {
    pub fn il4965_mac_start(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn il4965_mac_stop(hw: *mut ieee80211_hw, suspend: bool);
}
extern "C" {
    pub fn il4965_led_enable(il: *mut il_priv);
}
// EEPROM
pub const IL4965_EEPROM_IMG_SIZE: c_int = 1024;
//
// uCode queue management definitions ...
// The first queue used for block-ack aggregation is #7 (4965 only).
// All block-ack aggregation queues should map to Tx DMA/FIFO channel 7.
//
pub const IL49_FIRST_AMPDU_QUEUE: c_int = 7;
// Sizes and addresses for instruction and data memory (SRAM) in
// 4965's embedded processor.  Driver access is via HBUS_TARG_MEM_* regs.

// Size of uCode instruction memory in bootstrap state machine

// START TEMPERATURE
//
// 4965 temperature calculation.
//
// The driver must calculate the device temperature before calculating
// a txpower setting (amplifier gain is temperature dependent).  The
// calculation uses 4 measurements, 3 of which (R1, R2, R3) are calibration
// values used for the life of the driver, and one of which (R4) is the
// real-time temperature indicator.
//
// uCode provides all 4 values to the driver via the "initialize alive"
// notification (see struct il4965_init_alive_resp).  After the runtime uCode
// image loads, uCode updates the R4 value via stats notifications
// (see N_STATS), which occur after each received beacon
// when associated, or can be requested via C_STATS.
//
// NOTE:  uCode provides the R4 value as a 23-bit signed value.  Driver
// must sign-extend to 32 bits before applying formula below.
//
// Formula:
//
// degrees Kelvin = ((97 * 259 * (R4 - R2) / (R3 - R1)) / 100) + 8
//
// NOTE:  The basic formula is 259 * (R4-R2) / (R3-R1).  The 97/100 is
// an additional correction, which should be centered around 0 degrees
// Celsius (273 degrees Kelvin).  The 8 (3 percent of 273) compensates for
// centering the 97/100 correction around 0 degrees K.
//
// Add 273 to Kelvin value to find degrees Celsius, for comparing current
// temperature with factory-measured temperatures when calculating txpower
// settings.
//
pub const TEMPERATURE_CALIB_KELVIN_OFFSET: c_int = 8;
pub const TEMPERATURE_CALIB_A_VAL: c_int = 259;
// Limit range of calculated temperature to be between these Kelvin values

extern "C" {
    pub fn il4965_temperature_calib(il: *mut il_priv);
}
// END TEMPERATURE
// START TXPOWER
//
// 4965 txpower calculations rely on information from three sources:
//
// 1) EEPROM
// 2) "initialize" alive notification
// 3) stats notifications
//
// EEPROM data consists of:
//
// 1)  Regulatory information (max txpower and channel usage flags) is provided
// separately for each channel that can possibly supported by 4965.
// 40 MHz wide (.11n HT40) channels are listed separately from 20 MHz
// (legacy) channels.
//
// See struct il4965_eeprom_channel for format, and struct il4965_eeprom
// for locations in EEPROM.
//
// 2)  Factory txpower calibration information is provided separately for
// sub-bands of contiguous channels.  2.4GHz has just one sub-band,
// but 5 GHz has several sub-bands.
//
// In addition, per-band (2.4 and 5 Ghz) saturation txpowers are provided.
//
// See struct il4965_eeprom_calib_info (and the tree of structures
// contained within it) for format, and struct il4965_eeprom for
// locations in EEPROM.
//
// "Initialization alive" notification (see struct il4965_init_alive_resp)
// consists of:
//
// 1)  Temperature calculation parameters.
//
// 2)  Power supply voltage measurement.
//
// 3)  Tx gain compensation to balance 2 transmitters for MIMO use.
//
// Statistics notifications deliver:
//
// 1)  Current values for temperature param R4.
//
// To calculate a txpower setting for a given desired target txpower, channel,
// modulation bit rate, and transmitter chain (4965 has 2 transmitters to
// support MIMO and transmit diversity), driver must do the following:
//
// 1)  Compare desired txpower vs. (EEPROM) regulatory limit for this channel.
// Do not exceed regulatory limit; reduce target txpower if necessary.
//
// If setting up txpowers for MIMO rates (rate idxes 8-15, 24-31),
// 2 transmitters will be used simultaneously; driver must reduce the
// regulatory limit by 3 dB (half-power) for each transmitter, so the
// combined total output of the 2 transmitters is within regulatory limits.
//
// 2)  Compare target txpower vs. (EEPROM) saturation txpower *reduced by
// backoff for this bit rate*.  Do not exceed (saturation - backoff[rate]);
// reduce target txpower if necessary.
//
// Backoff values below are in 1/2 dB units (equivalent to steps in
// txpower gain tables):
//
// OFDM 6 - 36 MBit:  10 steps (5 dB)
// OFDM 48 MBit:      15 steps (7.5 dB)
// OFDM 54 MBit:      17 steps (8.5 dB)
// OFDM 60 MBit:      20 steps (10 dB)
// CCK all rates:     10 steps (5 dB)
//
// Backoff values apply to saturation txpower on a per-transmitter basis;
// when using MIMO (2 transmitters), each transmitter uses the same
// saturation level provided in EEPROM, and the same backoff values;
// no reduction (such as with regulatory txpower limits) is required.
//
// Saturation and Backoff values apply equally to 20 Mhz (legacy) channel
// widths and 40 Mhz (.11n HT40) channel widths; there is no separate
// factory measurement for ht40 channels.
//
// The result of this step is the final target txpower.  The rest of
// the steps figure out the proper settings for the device to achieve
// that target txpower.
//
// 3)  Determine (EEPROM) calibration sub band for the target channel, by
// comparing against first and last channels in each sub band
// (see struct il4965_eeprom_calib_subband_info).
//
// 4)  Linearly interpolate (EEPROM) factory calibration measurement sets,
// referencing the 2 factory-measured (sample) channels within the sub band.
//
// Interpolation is based on difference between target channel's frequency
// and the sample channels' frequencies.  Since channel numbers are based
// on frequency (5 MHz between each channel number), this is equivalent
// to interpolating based on channel number differences.
//
// Note that the sample channels may or may not be the channels at the
// edges of the sub band.  The target channel may be "outside" of the
// span of the sampled channels.
//
// Driver may choose the pair (for 2 Tx chains) of measurements (see
// struct il4965_eeprom_calib_ch_info) for which the actual measured
// txpower comes closest to the desired txpower.  Usually, though,
// the middle set of measurements is closest to the regulatory limits,
// and is therefore a good choice for all txpower calculations (this
// assumes that high accuracy is needed for maximizing legal txpower,
// while lower txpower configurations do not need as much accuracy).
//
// Driver should interpolate both members of the chosen measurement pair,
// i.e. for both Tx chains (radio transmitters), unless the driver knows
// that only one of the chains will be used (e.g. only one tx antenna
// connected, but this should be unusual).  The rate scaling algorithm
// switches antennas to find best performance, so both Tx chains will
// be used (although only one at a time) even for non-MIMO transmissions.
//
// Driver should interpolate factory values for temperature, gain table
// idx, and actual power.  The power amplifier detector values are
// not used by the driver.
//
// Sanity check:  If the target channel happens to be one of the sample
// channels, the results should agree with the sample channel's
// measurements!
//
// 5)  Find difference between desired txpower and (interpolated)
// factory-measured txpower.  Using (interpolated) factory gain table idx
// (shown elsewhere) as a starting point, adjust this idx lower to
// increase txpower, or higher to decrease txpower, until the target
// txpower is reached.  Each step in the gain table is 1/2 dB.
//
// For example, if factory measured txpower is 16 dBm, and target txpower
// is 13 dBm, add 6 steps to the factory gain idx to reduce txpower
// by 3 dB.
//
// 6)  Find difference between current device temperature and (interpolated)
// factory-measured temperature for sub-band.  Factory values are in
// degrees Celsius.  To calculate current temperature, see comments for
// "4965 temperature calculation".
//
// If current temperature is higher than factory temperature, driver must
// increase gain (lower gain table idx), and vice verse.
//
// Temperature affects gain differently for different channels:
//
// 2.4 GHz all channels:  3.5 degrees per half-dB step
// 5 GHz channels 34-43:  4.5 degrees per half-dB step
// 5 GHz channels >= 44:  4.0 degrees per half-dB step
//
// NOTE:  Temperature can increase rapidly when transmitting, especially
// with heavy traffic at high txpowers.  Driver should update
// temperature calculations often under these conditions to
// maintain strong txpower in the face of rising temperature.
//
// 7)  Find difference between current power supply voltage indicator
// (from "initialize alive") and factory-measured power supply voltage
// indicator (EEPROM).
//
// If the current voltage is higher (indicator is lower) than factory
// voltage, gain should be reduced (gain table idx increased) by:
//
// (eeprom - current) / 7
//
// If the current voltage is lower (indicator is higher) than factory
// voltage, gain should be increased (gain table idx decreased) by:
//
// 2 * (current - eeprom) / 7
//
// If number of idx steps in either direction turns out to be > 2,
// something is wrong ... just use 0.
//
// NOTE:  Voltage compensation is independent of band/channel.
//
// NOTE:  "Initialize" uCode measures current voltage, which is assumed
// to be constant after this initial measurement.  Voltage
// compensation for txpower (number of steps in gain table)
// may be calculated once and used until the next uCode bootload.
//
// 8)  If setting up txpowers for MIMO rates (rate idxes 8-15, 24-31),
// adjust txpower for each transmitter chain, so txpower is balanced
// between the two chains.  There are 5 pairs of tx_atten[group][chain]
// values in "initialize alive", one pair for each of 5 channel ranges:
//
// Group 0:  5 GHz channel 34-43
// Group 1:  5 GHz channel 44-70
// Group 2:  5 GHz channel 71-124
// Group 3:  5 GHz channel 125-200
// Group 4:  2.4 GHz all channels
//
// Add the tx_atten[group][chain] value to the idx for the target chain.
// The values are signed, but are in pairs of 0 and a non-negative number,
// so as to reduce gain (if necessary) of the "hotter" channel.  This
// avoids any need to double-check for regulatory compliance after
// this step.
//
// 9)  If setting up for a CCK rate, lower the gain by adding a CCK compensation
// value to the idx:
//
// Hardware rev B:  9 steps (4.5 dB)
// Hardware rev C:  5 steps (2.5 dB)
//
// Hardware rev for 4965 can be determined by reading CSR_HW_REV_WA_REG,
// bits [3:2], 1 = B, 2 = C.
//
// NOTE:  This compensation is in addition to any saturation backoff that
// might have been applied in an earlier step.
//
// 10) Select the gain table, based on band (2.4 vs 5 GHz).
//
// Limit the adjusted idx to stay within the table!
//
// 11) Read gain table entries for DSP and radio gain, place into appropriate
// location(s) in command (struct il4965_txpowertable_cmd).
//
// When MIMO is used (2 transmitters operating simultaneously), driver should
// limit each transmitter to deliver a max of 3 dB below the regulatory limit
// for the device.  That is, use half power for each transmitter, so total
// txpower is within regulatory limits.
//
// The value "6" represents number of steps in gain table to reduce power 3 dB.
// Each step is 1/2 dB.
//

//
// CCK gain compensation.
//
// When calculating txpowers for CCK, after making sure that the target power
// is within regulatory and saturation limits, driver must additionally
// back off gain by adding these values to the gain table idx.
//
// Hardware rev for 4965 can be determined by reading CSR_HW_REV_WA_REG,
// bits [3:2], 1 = B, 2 = C.
//

//
// 4965 power supply voltage compensation for txpower
//

//
// Gain tables.
//
// The following tables contain pair of values for setting txpower, i.e.
// gain settings for the output of the device's digital signal processor (DSP),
// and for the analog gain structure of the transmitter.
//
// Each entry in the gain tables represents a step of 1/2 dB.  Note that these
// are *relative* steps, not indications of absolute output power.  Output
// power varies with temperature, voltage, and channel frequency, and also
// requires consideration of average power (to satisfy regulatory constraints),
// and peak power (to avoid distortion of the output signal).
//
// Each entry contains two values:
// 1)  DSP gain (or sometimes called DSP attenuation).  This is a fine-grained
// linear value that multiplies the output of the digital signal processor,
// before being sent to the analog radio.
// 2)  Radio gain.  This sets the analog gain of the radio Tx path.
// It is a coarser setting, and behaves in a logarithmic (dB) fashion.
//
// EEPROM contains factory calibration data for txpower.  This maps actual
// measured txpower levels to gain settings in the "well known" tables
// below ("well-known" means here that both factory calibration *and* the
// driver work with the same table).
//
// There are separate tables for 2.4 GHz and 5 GHz bands.  The 5 GHz table
// has an extension (into negative idxes), in case the driver needs to
// boost power setting for high device temperatures (higher than would be
// present during factory calibration).  A 5 Ghz EEPROM idx of "40"
// corresponds to the 49th entry in the table used by the driver.
//

//
// 2.4 GHz gain table
//
// Index    Dsp gain   Radio gain
// 0        110         0x3f      (highest gain)
// 1        104         0x3f
// 2         98         0x3f
// 3        110         0x3e
// 4        104         0x3e
// 5         98         0x3e
// 6        110         0x3d
// 7        104         0x3d
// 8         98         0x3d
// 9        110         0x3c
// 10        104         0x3c
// 11         98         0x3c
// 12        110         0x3b
// 13        104         0x3b
// 14         98         0x3b
// 15        110         0x3a
// 16        104         0x3a
// 17         98         0x3a
// 18        110         0x39
// 19        104         0x39
// 20         98         0x39
// 21        110         0x38
// 22        104         0x38
// 23         98         0x38
// 24        110         0x37
// 25        104         0x37
// 26         98         0x37
// 27        110         0x36
// 28        104         0x36
// 29         98         0x36
// 30        110         0x35
// 31        104         0x35
// 32         98         0x35
// 33        110         0x34
// 34        104         0x34
// 35         98         0x34
// 36        110         0x33
// 37        104         0x33
// 38         98         0x33
// 39        110         0x32
// 40        104         0x32
// 41         98         0x32
// 42        110         0x31
// 43        104         0x31
// 44         98         0x31
// 45        110         0x30
// 46        104         0x30
// 47         98         0x30
// 48        110          0x6
// 49        104          0x6
// 50         98          0x6
// 51        110          0x5
// 52        104          0x5
// 53         98          0x5
// 54        110          0x4
// 55        104          0x4
// 56         98          0x4
// 57        110          0x3
// 58        104          0x3
// 59         98          0x3
// 60        110          0x2
// 61        104          0x2
// 62         98          0x2
// 63        110          0x1
// 64        104          0x1
// 65         98          0x1
// 66        110          0x0
// 67        104          0x0
// 68         98          0x0
// 69         97            0
// 70         96            0
// 71         95            0
// 72         94            0
// 73         93            0
// 74         92            0
// 75         91            0
// 76         90            0
// 77         89            0
// 78         88            0
// 79         87            0
// 80         86            0
// 81         85            0
// 82         84            0
// 83         83            0
// 84         82            0
// 85         81            0
// 86         80            0
// 87         79            0
// 88         78            0
// 89         77            0
// 90         76            0
// 91         75            0
// 92         74            0
// 93         73            0
// 94         72            0
// 95         71            0
// 96         70            0
// 97         69            0
// 98         68            0
//
// 5 GHz gain table
//
// Index    Dsp gain   Radio gain
// -9 	      123         0x3F      (highest gain)
// -8 	      117         0x3F
// -7        110         0x3F
// -6        104         0x3F
// -5         98         0x3F
// -4        110         0x3E
// -3        104         0x3E
// -2         98         0x3E
// -1        110         0x3D
// 0        104         0x3D
// 1         98         0x3D
// 2        110         0x3C
// 3        104         0x3C
// 4         98         0x3C
// 5        110         0x3B
// 6        104         0x3B
// 7         98         0x3B
// 8        110         0x3A
// 9        104         0x3A
// 10         98         0x3A
// 11        110         0x39
// 12        104         0x39
// 13         98         0x39
// 14        110         0x38
// 15        104         0x38
// 16         98         0x38
// 17        110         0x37
// 18        104         0x37
// 19         98         0x37
// 20        110         0x36
// 21        104         0x36
// 22         98         0x36
// 23        110         0x35
// 24        104         0x35
// 25         98         0x35
// 26        110         0x34
// 27        104         0x34
// 28         98         0x34
// 29        110         0x33
// 30        104         0x33
// 31         98         0x33
// 32        110         0x32
// 33        104         0x32
// 34         98         0x32
// 35        110         0x31
// 36        104         0x31
// 37         98         0x31
// 38        110         0x30
// 39        104         0x30
// 40         98         0x30
// 41        110         0x25
// 42        104         0x25
// 43         98         0x25
// 44        110         0x24
// 45        104         0x24
// 46         98         0x24
// 47        110         0x23
// 48        104         0x23
// 49         98         0x23
// 50        110         0x22
// 51        104         0x18
// 52         98         0x18
// 53        110         0x17
// 54        104         0x17
// 55         98         0x17
// 56        110         0x16
// 57        104         0x16
// 58         98         0x16
// 59        110         0x15
// 60        104         0x15
// 61         98         0x15
// 62        110         0x14
// 63        104         0x14
// 64         98         0x14
// 65        110         0x13
// 66        104         0x13
// 67         98         0x13
// 68        110         0x12
// 69        104         0x08
// 70         98         0x08
// 71        110         0x07
// 72        104         0x07
// 73         98         0x07
// 74        110         0x06
// 75        104         0x06
// 76         98         0x06
// 77        110         0x05
// 78        104         0x05
// 79         98         0x05
// 80        110         0x04
// 81        104         0x04
// 82         98         0x04
// 83        110         0x03
// 84        104         0x03
// 85         98         0x03
// 86        110         0x02
// 87        104         0x02
// 88         98         0x02
// 89        110         0x01
// 90        104         0x01
// 91         98         0x01
// 92        110         0x00
// 93        104         0x00
// 94         98         0x00
// 95         93         0x00
// 96         88         0x00
// 97         83         0x00
// 98         78         0x00
//
// Sanity checks and default values for EEPROM regulatory levels.
// If EEPROM values fall outside MIN/MAX range, use default values.
//
// Regulatory limits refer to the maximum average txpower allowed by
// regulatory agencies in the geographies in which the device is meant
// to be operated.  These limits are SKU-specific (i.e. geography-specific),
// and channel-specific; each channel has an individual regulatory limit
// listed in the EEPROM.
//
// Units are in half-dBm (i.e. "34" means 17 dBm).
//

//
// Sanity checks and default values for EEPROM saturation levels.
// If EEPROM values fall outside MIN/MAX range, use default values.
//
// Saturation is the highest level that the output power amplifier can produce
// without significant clipping distortion.  This is a "peak" power level.
// Different types of modulation (i.e. various "rates", and OFDM vs. CCK)
// require differing amounts of backoff, relative to their average power output,
// in order to avoid clipping distortion.
//
// Driver must make sure that it is violating neither the saturation limit,
// nor the regulatory limit, when calculating Tx power settings for various
// rates.
//
// Units are in half-dBm (i.e. "38" means 19 dBm).
//

//
// Channel groups used for Tx Attenuation calibration (MIMO tx channel balance)
// and thermal Txpower calibration.
//
// When calculating txpower, driver must compensate for current device
// temperature; higher temperature requires higher gain.  Driver must calculate
// current temperature (see "4965 temperature calculation"), then compare vs.
// factory calibration temperature in EEPROM; if current temperature is higher
// than factory temperature, driver must *increase* gain by proportions shown
// in table below.  If current temperature is lower than factory, driver must
// *decrease* gain.
//
// Different frequency ranges require different compensation, as shown below.
//
// Group 0, 5.2 GHz ch 34-43:  4.5 degrees per 1/2 dB.
pub const CALIB_IL_TX_ATTEN_GR1_FCH: c_int = 34;
pub const CALIB_IL_TX_ATTEN_GR1_LCH: c_int = 43;
// Group 1, 5.3 GHz ch 44-70:  4.0 degrees per 1/2 dB.
pub const CALIB_IL_TX_ATTEN_GR2_FCH: c_int = 44;
pub const CALIB_IL_TX_ATTEN_GR2_LCH: c_int = 70;
// Group 2, 5.5 GHz ch 71-124:  4.0 degrees per 1/2 dB.
pub const CALIB_IL_TX_ATTEN_GR3_FCH: c_int = 71;
pub const CALIB_IL_TX_ATTEN_GR3_LCH: c_int = 124;
// Group 3, 5.7 GHz ch 125-200:  4.0 degrees per 1/2 dB.
pub const CALIB_IL_TX_ATTEN_GR4_FCH: c_int = 125;
pub const CALIB_IL_TX_ATTEN_GR4_LCH: c_int = 200;
// Group 4, 2.4 GHz all channels:  3.5 degrees per 1/2 dB.
pub const CALIB_IL_TX_ATTEN_GR5_FCH: c_int = 1;
pub const CALIB_IL_TX_ATTEN_GR5_LCH: c_int = 20;
// END TXPOWER
//
// Tx/Rx Queues
//
// Most communication between driver and 4965 is via queues of data buffers.
// For example, all commands that the driver issues to device's embedded
// controller (uCode) are via the command queue (one of the Tx queues).  All
// uCode command responses/replies/notifications, including Rx frames, are
// conveyed from uCode to driver via the Rx queue.
//
// Most support for these queues, including handshake support, resides in
// structures in host DRAM, shared between the driver and the device.  When
// allocating this memory, the driver must make sure that data written by
// the host CPU updates DRAM immediately (and does not get "stuck" in CPU's
// cache memory), so DRAM and cache are consistent, and the device can
// immediately see changes made by the driver.
//
// 4965 supports up to 16 DRAM-based Tx queues, and services these queues via
// up to 7 DMA channels (FIFOs).  Each Tx queue is supported by a circular array
// in DRAM containing 256 Transmit Frame Descriptors (TFDs).
//
pub const IL49_NUM_FIFOS: c_int = 7;
pub const IL49_CMD_FIFO_NUM: c_int = 4;
pub const IL49_NUM_QUEUES: c_int = 16;
pub const IL49_NUM_AMPDU_QUEUES: c_int = 8;
//
// struct il4965_schedq_bc_tbl
//
// Byte Count table
//
// Each Tx queue uses a byte-count table containing 320 entries:
// one 16-bit entry for each of 256 TFDs, plus an additional 64 entries that
// duplicate the first 64 entries (to avoid wrap-around within a Tx win;
// max Tx win is 64 TFDs).
//
// When driver sets up a new TFD, it must also enter the total byte count
// of the frame to be transmitted into the corresponding entry in the byte
// count table for the chosen Tx queue.  If the TFD idx is 0-63, the driver
// must duplicate the byte count entry in corresponding idx 256-319.
//
// padding puts each byte count table on a 1024-byte boundary;
// 4965 assumes tables are separated by 1024 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il4965_scd_bc_tbl {
    pub tfd_offset: [__le16; TFD_QUEUE_BC_SIZE],
    pub sizeof(__le16)]: *mut *mut u8 pad[1024 - (TFD_QUEUE_BC_SIZE),
    pub __packed: },

// RSSI to dBm
pub const IL4965_RSSI_OFFSET: c_int = 44;
// PCI registers
pub const PCI_CFG_RETRY_TIMEOUT: c_uint = 0x041;
pub const IL4965_DEFAULT_TX_RETRY: c_int = 15;
// EEPROM
pub const IL4965_FIRST_AMPDU_QUEUE: c_int = 10;
// Calibration
    pub stat_resp): *mut *mut void il4965_chain_noise_calibration(struct il_priv il, void,
    pub resp): *mut *mut void il4965_sensitivity_calibration(struct il_priv il, void,
    pub il): *mut void il4965_init_sensitivity(struct il_priv,
    pub il): *mut void il4965_reset_run_time_calib(struct il_priv,
// Debug

    pub il4965_debugfs_ops: extern struct il_debugfs_ops,

//
// Flow Handler Definitions
//
// This I/O area is directly read/writable by driver (e.g. Linux uses writel())
// Addresses are offsets from device's PCI hardware base address.
//

//
// Keep-Warm (KW) buffer base address.
//
// Driver must allocate a 4KByte buffer that is used by 4965 for keeping the
// host DRAM powered on (via dummy accesses to DRAM) to maintain low-latency
// DRAM access when 4965 is Txing or Rxing.  The dummy accesses prevent host
// from going into a power-savings mode that would cause higher DRAM latency,
// and possible data over/under-runs, before all Tx/Rx is complete.
//
// Driver loads FH49_KW_MEM_ADDR_REG with the physical address (bits 35:4)
// of the buffer, which must be 4K aligned.  Once this is set up, the 4965
// automatically invokes keep-warm accesses when normal accesses might not
// be sufficient to maintain fast DRAM response.
//
// Bit fields:
// 31-0:  Keep-warm buffer physical base address [35:4], must be 4K aligned
//

//
// TFD Circular Buffers Base (CBBC) addresses
//
// 4965 has 16 base pointer registers, one for each of 16 host-DRAM-resident
// circular buffers (CBs/queues) containing Transmit Frame Descriptors (TFDs)
// (see struct il_tfd_frame).  These 16 pointer registers are offset by 0x04
// bytes from one another.  Each TFD circular buffer in DRAM must be 256-byte
// aligned (address bits 0-7 must be 0).
//
// Bit fields in each pointer register:
// 27-0: TFD CB physical base address [35:8], must be 256-byte aligned
//

// Find TFD CB base pointer for given queue (range 0-15).

//
// Rx SRAM Control and Status Registers (RSCSR)
//
// These registers provide handshake between driver and 4965 for the Rx queue
// (this queue handles *all* command responses, notifications, Rx data, etc.
// sent from 4965 uCode to host driver).  Unlike Tx, there is only one Rx
// queue, and only one Rx DMA/FIFO channel.  Also unlike Tx, which can
// concatenate up to 20 DRAM buffers to form a Tx frame, each Receive Buffer
// Descriptor (RBD) points to only one Rx Buffer (RB); there is a 1:1
// mapping between RBDs and RBs.
//
// Driver must allocate host DRAM memory for the following, and set the
// physical address of each into 4965 registers:
//
// 1)  Receive Buffer Descriptor (RBD) circular buffer (CB), typically with 256
// entries (although any power of 2, up to 4096, is selectable by driver).
// Each entry (1 dword) points to a receive buffer (RB) of consistent size
// (typically 4K, although 8K or 16K are also selectable by driver).
// Driver sets up RB size and number of RBDs in the CB via Rx config
// register FH49_MEM_RCSR_CHNL0_CONFIG_REG.
//
// Bit fields within one RBD:
// 27-0:  Receive Buffer physical address bits [35:8], 256-byte aligned
//
// Driver sets physical address [35:8] of base of RBD circular buffer
// into FH49_RSCSR_CHNL0_RBDCB_BASE_REG [27:0].
//
// 2)  Rx status buffer, 8 bytes, in which 4965 indicates which Rx Buffers
// (RBs) have been filled, via a "write pointer", actually the idx of
// the RB's corresponding RBD within the circular buffer.  Driver sets
// physical address [35:4] into FH49_RSCSR_CHNL0_STTS_WPTR_REG [31:0].
//
// Bit fields in lower dword of Rx status buffer (upper dword not used
// by driver; see struct il4965_shared, val0):
// 31-12:  Not used by driver
// 11- 0:  Index of last filled Rx buffer descriptor
// (4965 writes, driver reads this value)
//
// As the driver prepares Receive Buffers (RBs) for 4965 to fill, driver must
// enter pointers to these RBs into contiguous RBD circular buffer entries,
// and update the 4965's "write" idx register,
// FH49_RSCSR_CHNL0_RBDCB_WPTR_REG.
//
// This "write" idx corresponds to the *next* RBD that the driver will make
// available, i.e. one RBD past the tail of the ready-to-fill RBDs within
// the circular buffer.  This value should initially be 0 (before preparing any
// RBs), should be 8 after preparing the first 8 RBs (for example), and must
// wrap back to 0 at the end of the circular buffer (but don't wrap before
// "read" idx has advanced past 1!  See below).
// NOTE:  4965 EXPECTS THE WRITE IDX TO BE INCREMENTED IN MULTIPLES OF 8.
//
// As the 4965 fills RBs (referenced from contiguous RBDs within the circular
// buffer), it updates the Rx status buffer in host DRAM, 2) described above,
// to tell the driver the idx of the latest filled RBD.  The driver must
// read this "read" idx from DRAM after receiving an Rx interrupt from 4965.
//
// The driver must also internally keep track of a third idx, which is the
// next RBD to process.  When receiving an Rx interrupt, driver should process
// all filled but unprocessed RBs up to, but not including, the RB
// corresponding to the "read" idx.  For example, if "read" idx becomes "1",
// driver may process the RB pointed to by RBD 0.  Depending on volume of
// traffic, there may be many RBs to process.
//
// If read idx == write idx, 4965 thinks there is no room to put new data.
// Due to this, the maximum number of filled RBs is 255, instead of 256.  To
// be safe, make sure that there is a gap of at least 2 RBDs between "write"
// and "read" idxes; that is, make sure that there are no more than 254
// buffers waiting to be filled.
//

//
// Physical base address of 8-byte Rx Status buffer.
// Bit fields:
// 31-0: Rx status buffer physical base address [35:4], must 16-byte aligned.
//

//
// Physical base address of Rx Buffer Descriptor Circular Buffer.
// Bit fields:
// 27-0:  RBD CD physical base address [35:8], must be 256-byte aligned.
//

//
// Rx write pointer (idx, really!).
// Bit fields:
// 11-0:  Index of driver's most recent prepared-to-be-filled RBD, + 1.
// NOTE:  For 256-entry circular buffer, use only bits [7:0].
//

//
// Rx Config/Status Registers (RCSR)
// Rx Config Reg for channel 0 (only channel used)
//
// Driver must initialize FH49_MEM_RCSR_CHNL0_CONFIG_REG as follows for
// normal operation (see bit fields).
//
// Clearing FH49_MEM_RCSR_CHNL0_CONFIG_REG to 0 turns off Rx DMA.
// Driver should poll FH49_MEM_RSSR_RX_STATUS_REG	for
// FH49_RSSR_CHNL0_RX_STATUS_CHNL_IDLE (bit 24) before continuing.
//
// Bit fields:
// 31-30: Rx DMA channel enable: '00' off/pause, '01' pause at end of frame,
// '10' operate normally
// 29-24: reserved
// 23-20: # RBDs in circular buffer = 2^value; use "8" for 256 RBDs (normal),
// min "5" for 32 RBDs, max "12" for 4096 RBDs.
// 19-18: reserved
// 17-16: size of each receive buffer; '00' 4K (normal), '01' 8K,
// '10' 12K, '11' 16K.
// 15-14: reserved
// 13-12: IRQ destination; '00' none, '01' host driver (normal operation)
// 11- 4: timeout for closing Rx buffer and interrupting host (units 32 usec)
// typical value 0x10 (about 1/2 msec)
// 3- 0: reserved
//

//
// Rx Shared Status Registers (RSSR)
//
// After stopping Rx DMA channel (writing 0 to
// FH49_MEM_RCSR_CHNL0_CONFIG_REG), driver must poll
// FH49_MEM_RSSR_RX_STATUS_REG until Rx channel is idle.
//
// Bit fields:
// 24:  1 = Channel 0 is idle
//
// FH49_MEM_RSSR_SHARED_CTRL_REG and FH49_MEM_RSSR_RX_ENABLE_ERR_IRQ2DRV
// contain default values that should not be altered by the driver.
//

// Macro flag: #define FH49_MEM_RSSR_RX_ENABLE_ERR_IRQ2DRV\

pub const FH49_MEM_TFDIB_REG1_ADDR_BITSHIFT: c_int = 28;
// TFDB  Area - TFDs buffer table

//
// Transmit DMA Channel Control/Status Registers (TCSR)
//
// 4965 has one configuration register for each of 8 Tx DMA/FIFO channels
// supported in hardware (don't confuse these with the 16 Tx queues in DRAM,
// which feed the DMA/FIFO channels); config regs are separated by 0x20 bytes.
//
// To use a Tx DMA channel, driver must initialize its
// FH49_TCSR_CHNL_TX_CONFIG_REG(chnl) with:
//
// FH49_TCSR_TX_CONFIG_REG_VAL_DMA_CHNL_ENABLE |
// FH49_TCSR_TX_CONFIG_REG_VAL_DMA_CREDIT_ENABLE_VAL
//
// All other bits should be 0.
//
// Bit fields:
// 31-30: Tx DMA channel enable: '00' off/pause, '01' pause at end of frame,
// '10' operate normally
// 29- 4: Reserved, set to "0"
// 3: Enable internal DMA requests (1, normal operation), disable (0)
// 2- 0: Reserved, set to "0"
//

// Find Control/Status reg for given Tx DMA/FIFO channel

// TCSR: tx_config register values

//
// Tx Shared Status Registers (TSSR)
//
// After stopping Tx DMA channel (writing 0 to
// FH49_TCSR_CHNL_TX_CONFIG_REG(chnl)), driver must poll
// FH49_TSSR_TX_STATUS_REG until selected Tx channel is idle
// (channel's buffers empty | no pending requests).
//
// Bit fields:
// 31-24:  1 = Channel buffers empty (channel 7:0)
// 23-16:  1 = No pending requests (channel 7:0)
//

//
// Bit fields for TSSR(Tx Shared Status & Control) error status register:
// 31:  Indicates an address error when accessed to internal memory
// uCode/driver must write "1" in order to clear this flag
// 30:  Indicates that Host did not send the expected number of dwords to FH
// uCode/driver must write "1" in order to clear this flag
// 16-9:Each status bit is for one channel. Indicates that an (Error) ActDMA
// command was received from the scheduler while the TRB was already full
// with previous command
// uCode/driver must write "1" in order to clear this flag
// 7-0: Each status bit indicates a channel's TxCredit error. When an error
// bit is set, it indicates that the FH has received a full indication
// from the RTC TxFIFO and the current value of the TxCredit counter was
// not equal to zero. This mean that the credit mechanism was not
// synchronized to the TxFIFO status
// uCode/driver must write "1" in order to clear this flag
//

// Tx service channels

// Instruct FH to increment the retry count of a packet when
// it is brought from the memory to TX-FIFO
//

// Keep Warm Size
pub const IL_KW_SIZE: c_uint = 0x1000	/* 4k */;
