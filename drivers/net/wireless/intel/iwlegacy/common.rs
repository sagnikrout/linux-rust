//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlegacy/common.h
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

// Macro flag: #define __il_core_h__

pub const RX_QUEUE_SIZE: c_int = 256;
pub const RX_QUEUE_MASK: c_int = 255;
pub const RX_QUEUE_SIZE_LOG: c_int = 8;
//
// RX related structures and functions
//
pub const RX_FREE_BUFFERS: c_int = 64;
pub const RX_LOW_WATERMARK: c_int = 8;

// CT-KILL constants

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

//
// RTS threshold here is total size [2347] minus 4 FCS bytes
// Per spec:
// a value of 0 means RTS on all data/management packets
// a value > max MSDU size means no RTS
// else RTS for data/management frames where MPDU is larger
// than RTS value.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_rx_buf {
    pub page_dma: dma_addr_t,
    pub page: *mut page,
    pub list: list_head,
}

// defined below
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_cmd_meta {
// only for SYNC commands, iff the reply skb is wanted
    pub source: *mut il_host_cmd,
//
// only for ASYNC commands
// (which is somewhat stupid -- look at common.c for instance
// which duplicates a bunch of code because the callback isn't
// invoked for SYNC commands, if it were and its result passed
// through it would be simpler...)
//
    pub pkt): *mut il_rx_pkt,
// The CMD_SIZE_HUGE flag bit indicates that the command
// structure is stored at the end of the shared queue memory.
    pub flags: u32,
}

//
// Generic queue structure
//
// Contains common data for Rx and Tx queues
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_queue {
    pub /: *mut *mut int n_bd; / number of BDs in this queue,
    pub /: *mut *mut int write_ptr; / 1-st empty entry (idx) host_w,
    pub /: *mut *mut int read_ptr; / last used entry (idx) host_r,
// use for monitoring and recovering the stuck queue
    pub /: *mut *mut dma_addr_t dma_addr; / physical addr for BD's,
    pub /: *mut *mut int n_win; / safe queue win,
    pub id: u32,
    pub free: *mut *mut int low_mark; / low watermark, resume queue if,
// space more than this
    pub free: *mut *mut int high_mark; / high watermark, stop queue if,
// space less than this
}

//
// struct il_tx_queue - Tx Queue for DMA
// @q: generic Rx/Tx queue descriptor
// @bd: base of circular buffer of TFDs
// @cmd: array of command/TX buffer pointers
// @meta: array of meta data for each command/tx buffer
// @dma_addr_cmd: physical address of cmd/tx buffer array
// @skbs: array of per-TFD socket buffer pointers
// @time_stamp: time (in jiffies) of last read_ptr change
// @need_update: indicates need to update read/write idx
// @sched_retry: indicates queue is high-throughput aggregation (HT AGG) enabled
//
// A Tx queue consists of circular buffer of BDs (a.k.a. TFDs, transmit frame
// descriptors) and required locking structures.
//
pub const TFD_TX_CMD_SLOTS: c_int = 256;
pub const TFD_CMD_SLOTS: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_tx_queue {
    pub q: il_queue,
    pub tfds: *mut c_void,
    pub cmd: *mut il_device_cmd,
    pub meta: *mut il_cmd_meta,
    pub skbs: *mut sk_buff,
    pub time_stamp: c_ulong,
    pub need_update: u8,
    pub sched_retry: u8,
    pub active: u8,
    pub swq_id: u8,
}

//
// EEPROM access time values:
//
// Driver initiates EEPROM read by writing byte address << 1 to CSR_EEPROM_REG.
// Driver then polls CSR_EEPROM_REG for CSR_EEPROM_REG_READ_VALID_MSK (0x1).
// When polling, wait 10 uSec between polling loops, up to a maximum 5000 uSec.
// Driver reads 16-bit value from bits 31-16 of CSR_EEPROM_REG.
//

//
// Regulatory channel usage flags in EEPROM struct il4965_eeprom_channel.flags.
//
// IBSS and/or AP operation is allowed *only* on those channels with
// (VALID && IBSS && ACTIVE && !RADAR).  This restriction is in place because
// RADAR detection is not supported by the 4965 driver, but is a
// requirement for establishing a new network for legal operation on channels
// requiring RADAR detection or restricting ACTIVE scanning.
//
// NOTE:  "WIDE" flag does not indicate anything about "HT40" 40 MHz channels.
// It only indicates that 20 MHz channel use is supported; HT40 channel
// usage is indicated by a separate set of regulatory flags for each
// HT40 channel pair.
//
// NOTE:  Using a channel inappropriately will result in a uCode error!
//
pub const IL_NUM_TX_CALIB_GROUPS: c_int = 5;
// Bit 2 Reserved
// Bit 6 Reserved (was Narrow Channel)
// SKU Capabilities
// 3945 only

// *regulatory* channel data format in eeprom, one for each channel.
// There are separate entries for HT40 (40 MHz) vs. normal (20 MHz) channels.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_eeprom_channel {
    pub /: *mut *mut *mut u8 flags; / EEPROM_CHANNEL_ flags copied from EEPROM,
    pub /: *mut *mut s8 max_power_avg; / max power (dBm) on this chnl, limit 31,
    pub __packed: },
// 3945 Specific

// 4965 has two radio transmitters (and 3 radio receivers)

// 4965 has room for up to 8 sets of txpower calibration data

// 4965 factory calibration measures txpower gain settings for
// each of 3 target output levels

// 4965 Specific
// 4965 driver does not work with txpower calibration version < 5

// 2.4 GHz
    pub il_eeprom_band_1: [extern u8; 14],
//
// factory calibration data for one txpower level, on one channel,
// measured on one of the 2 tx chains (radio transmitter and associated
// antenna).  EEPROM contains:
//
// 1)  Temperature (degrees Celsius) of device when measurement was made.
//
// 2)  Gain table idx used to achieve the target measurement power.
// This refers to the "well-known" gain tables (see 4965.h).
//
// 3)  Actual measured output power, in half-dBm ("34" = 17 dBm).
//
// 4)  RF power amplifier detector level measurement (not used).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_eeprom_calib_measure {
    pub /: *mut *mut u8 temperature; / Device temperature (Celsius),
    pub /: *mut *mut u8 gain_idx; / Index into gain table,
    pub /: *mut *mut u8 actual_pow; / Measured RF output power, half-dBm,
    pub /: *mut *mut s8 pa_det; / Power amp detector level (not used),
    pub __packed: },
//
// measurement set for one channel.  EEPROM contains:
//
// 1)  Channel number measured
//
// 2)  Measurements for each of 3 power levels for each of 2 radio transmitters
// (a.k.a. "tx chains") (6 measurements altogether)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_eeprom_calib_ch_info {
    pub ch_num: u8,
    pub __packed: },
//
// txpower subband info.
//
// For each frequency subband, EEPROM contains the following:
//
// 1)  First and last channels within range of the subband.  "0" values
// indicate that this sample set is not being used.
//
// 2)  Sample measurement sets for 2 channels close to the range endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_eeprom_calib_subband_info {
    pub /: *mut *mut u8 ch_from; / channel number of lowest channel in subband,
    pub /: *mut *mut u8 ch_to; / channel number of highest channel in subband,
    pub ch1: il_eeprom_calib_ch_info,
    pub ch2: il_eeprom_calib_ch_info,
    pub __packed: },
//
// txpower calibration info.  EEPROM contains:
//
// 1)  Factory-measured saturation power levels (maximum levels at which
// tx power amplifier can output a signal without too much distortion).
// There is one level for 2.4 GHz band and one for 5 GHz band.  These
// values apply to all channels within each of the bands.
//
// 2)  Factory-measured power supply voltage level.  This is assumed to be
// constant (i.e. same value applies to all channels/bands) while the
// factory measurements are being made.
//
// 3)  Up to 8 sets of factory-measured txpower calibration values.
// These are for different frequency ranges, since txpower gain
// characteristics of the analog radio circuitry vary with frequency.
//
// Not all sets need to be filled with data;
// struct il_eeprom_calib_subband_info contains range of channels
// (0 if unused) for each set of data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_eeprom_calib_info {
    pub /: *mut *mut u8 saturation_power24; / half-dBm (e.g. "34" = 17 dBm),
    pub /: *mut *mut u8 saturation_power52; / half-dBm,
    pub /: *mut *mut __le16 voltage; / signed,
    pub band_info: [il_eeprom_calib_subband_info; EEPROM_TX_POWER_BANDS],
    pub __packed: },
// General

// The following masks are to be applied on EEPROM_RADIO_CONFIG

pub const EEPROM_3945_RF_CFG_TYPE_MAX: c_uint = 0x0;
pub const EEPROM_4965_RF_CFG_TYPE_MAX: c_uint = 0x1;
//
// Per-channel regulatory data.
//
// Each channel that *might* be supported by iwl has a fixed location
// in EEPROM containing EEPROM_CHANNEL_* usage flags (LSB) and max regulatory
// txpower (MSB).
//
// Entries immediately below are for 20 MHz channel width.  HT40 (40 MHz)
// channels (only for 4965, not supported by 3945) appear later in the EEPROM.
//
// 2.4 GHz channels 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14
//

//
// 4.9 GHz channels 183, 184, 185, 187, 188, 189, 192, 196,
// 5.0 GHz channels 7, 8, 11, 12, 16
// (4915-5080MHz) (none of these is ever supported)
//

//
// 5.2 GHz channels 34, 36, 38, 40, 42, 44, 46, 48, 52, 56, 60, 64
// (5170-5320MHz)
//

//
// 5.5 GHz channels 100, 104, 108, 112, 116, 120, 124, 128, 132, 136, 140
// (5500-5700MHz)
//

//
// 5.7 GHz channels 145, 149, 153, 157, 161, 165
// (5725-5825MHz)
//

//
// 2.4 GHz HT40 channels 1 (5), 2 (6), 3 (7), 4 (8), 5 (9), 6 (10), 7 (11)
//
// The channel listed is the center of the lower 20 MHz half of the channel.
// The overall center frequency is actually 2 channels (10 MHz) above that,
// and the upper half of each HT40 channel is centered 4 channels (20 MHz) away
// from the lower half; e.g. the upper half of HT40 channel 1 is channel 5,
// and the overall HT40 channel width centers on channel 3.
//
// NOTE:  The RXON command uses 20 MHz channel numbers to specify the
// control channel to which to tune.  RXON also specifies whether the
// control channel is the upper or lower half of a HT40 channel.
//
// NOTE:  4965 does not support HT40 channels on 2.4 GHz.
//

//
// 5.2 GHz HT40 channels 36 (40), 44 (48), 52 (56), 60 (64),
// 100 (104), 108 (112), 116 (120), 124 (128), 132 (136), 149 (153), 157 (161)
//

    pub il): *mut int il_eeprom_init(struct il_priv,
    pub il): *mut void il_eeprom_free(struct il_priv,
    pub offset): *const *const *const u8 il_eeprom_query_addr(struct il_priv il, size_t,
    pub offset): *const *const u16 il_eeprom_query16(struct il_priv il, size_t,
    pub il): *mut int il_init_channel_map(struct il_priv,
    pub il): *mut void il_free_channel_map(struct il_priv,
    pub channel): u16,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il4965_channel_tgd_info {
    pub type: u8,
    pub max_power: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il4965_channel_tgh_info {
    pub last_radar_time: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_clip_group {
// maximum power level to prevent clipping for each rate, derived by
// us from this band's saturation power in EEPROM
    pub clip_powers: [i8; IL_MAX_RATES],
}

// current Tx power values to use, one for each rate for each channel.
// requested power is limited by:
// -- regulatory EEPROM limits for this channel
// -- hardware capabilities (clip-powers)
// -- spectrum management
// -- user preference (e.g. iwconfig)
// when requested power is set, base power idx must also be set.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_channel_power_info {
    pub /: *mut *mut il3945_tx_power tpc; / actual radio and DSP gain settings,
    pub /: *mut *mut s8 power_table_idx; / actual (compenst'd) idx into gain table,
    pub /: *mut *mut s8 base_power_idx; / gain idx for power at factory temp.,
    pub /: *mut *mut s8 requested_power; / power (dBm) requested for this chnl/rate,
}

// current scan Tx power values to use, one for each scan rate for each
// channel.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_scan_power_info {
    pub /: *mut *mut il3945_tx_power tpc; / actual radio and DSP gain settings,
    pub /: *mut *mut s8 power_table_idx; / actual (compenst'd) idx into gain table,
    pub /: *mut *mut s8 requested_power; / scan pwr (dBm) requested for chnl/rate,
}

//
// One for each channel, holds all channel setup data
// Some of the fields (e.g. eeprom and flags/max_power_avg) are redundant
// with one another!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_channel_info {
    pub tgd: il4965_channel_tgd_info,
    pub tgh: il4965_channel_tgh_info,
    pub /: *mut *mut il_eeprom_channel eeprom; / EEPROM regulatory limit,
    pub for: *mut *mut il_eeprom_channel ht40_eeprom; / EEPROM regulatory limit,
// HT40 channel
    pub /: *mut *mut u8 channel; / channel number,
    pub /: *mut *mut u8 flags; / flags copied from EEPROM,
    pub /: *mut *mut s8 max_power_avg; / (dBm) regul. eeprom, normal Tx, any rate,
    pub /: *mut *mut s8 curr_txpow; / (dBm) regulatory/spectrum/user (not h/w) limit,
    pub /: *mut *mut s8 min_power; / always 0,
    pub /: *mut *mut s8 scan_power; / (dBm) regul. eeprom, direct scans, any rate,
    pub /: *mut *mut u8 group_idx; / 0-4, maps channel to group1/2/3/4/5,
    pub /: *mut *mut u8 band_idx; / 0-4, maps channel to band1/2/3/4/5,
    pub band: nl80211_band,
// HT40 channel info
    pub /: *mut *mut s8 ht40_max_power_avg; / (dBm) regul. eeprom, normal Tx, any rate,
    pub /: *mut *mut u8 ht40_flags; / flags copied from EEPROM,
    pub /: *mut *mut *mut u8 ht40_extension_channel; / HT_IE_EXT_CHANNEL_,
// Radio/DSP gain settings for each "normal" data Tx rate.
// These include, in addition to RF and DSP gain, a few fields for
// remembering/modifying gain settings (idxes).
    pub power_info: [il3945_channel_power_info; IL4965_MAX_RATE],
// Radio/DSP gain settings for each scan rate, for directed scans.
    pub scan_pwr_info: [il3945_scan_power_info; IL_NUM_SCAN_RATES],
}

pub const IL_TX_FIFO_BE: c_int = 1;

pub const IL_TX_FIFO_VO: c_int = 3;

// Minimum number of queues. MAX_NUM is defined in hw specific files.
// Set the minimum to accommodate the 4 standard TX queues, 1 command
// queue, 2 (unused) HCCA queues, and 4 HT queues (one for each AC)
pub const IL_MIN_NUM_QUEUES: c_int = 10;
pub const IL_DEFAULT_CMD_QUEUE_NUM: c_int = 4;
pub const IEEE80211_DATA_LEN: c_int = 2304;
pub const IEEE80211_4ADDR_LEN: c_int = 30;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_frame {
    pub list: list_head,
// Must be last as it ends in a flexible-array member.
    pub frame: ieee80211_hdr,
    pub beacon: il_tx_beacon_cmd,
    pub raw: [u8; IEEE80211_FRAME_LEN],
    pub cmd: [u8; 360],
    pub u: },
}

pub const DEF_CMD_PAYLOAD_SIZE: c_int = 320;
//
// struct il_device_cmd
//
// For allocation of the command and tx queues, this establishes the overall
// size of the largest command we send to uCode, except for a scan command
// (which is relatively huge; space is allocated separately).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_device_cmd {
    pub /: *mut *mut il_cmd_header hdr; / uCode API,
    pub flags: u32,
    pub val8: u8,
    pub val16: u16,
    pub val32: u32,
    pub tx: il_tx_cmd_hdr,
    pub payload: [u8; DEF_CMD_PAYLOAD_SIZE],
    pub cmd: } __packed,
    pub __packed: },

//
// struct il_device_cmd_huge
//
// For use when sending huge commands.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_device_cmd_huge {
    pub /: *mut *mut il_cmd_header hdr; / uCode API,
    pub il_cmd_header)]: u8 payload[IL_MAX_CMD_SIZE - sizeof(struct,
    pub cmd: } __packed,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_host_cmd {
    pub data: *const c_void,
    pub reply_page: c_ulong,
    pub pkt): *mut il_rx_pkt,
    pub flags: u32,
    pub len: u16,
    pub id: u8,
}

pub const SUP_RATE_11A_MAX_NUM_CHANNELS: c_int = 8;
pub const SUP_RATE_11B_MAX_NUM_CHANNELS: c_int = 4;
pub const SUP_RATE_11G_MAX_NUM_CHANNELS: c_int = 12;
//
// struct il_rx_queue - Rx queue
// @bd: driver's pointer to buffer of receive buffer descriptors (rbd)
// @bd_dma: bus address of buffer of receive buffer descriptors (rbd)
// @read: Shared idx to newest available Rx buffer
// @write: Shared idx to oldest written Rx packet
// @free_count: Number of pre-allocated buffers in rx_free
// @rx_free: list of free SKBs for use
// @rx_used: List of Rx buffers with no SKB
// @need_update: flag to indicate we need to update read/write idx
// @rb_stts: driver's pointer to receive buffer status
// @rb_stts_dma: bus address of receive buffer status
//
// NOTE:  rx_free and rx_used are used as a FIFO for il_rx_bufs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_rx_queue {
    pub bd: *mut __le32,
    pub bd_dma: dma_addr_t,
    pub RX_FREE_BUFFERS]: il_rx_buf pool[RX_QUEUE_SIZE +,
    pub queue: [*mut il_rx_buf; RX_QUEUE_SIZE],
    pub read: u32,
    pub write: u32,
    pub free_count: u32,
    pub write_actual: u32,
    pub rx_free: list_head,
    pub rx_used: list_head,
    pub need_update: c_int,
    pub rb_stts: *mut il_rb_status,
    pub rb_stts_dma: dma_addr_t,
    pub lock: spinlock_t,
}

pub const IL_SUPPORTED_RATES_IE_LEN: c_int = 8;
pub const MAX_TID_COUNT: c_int = 9;
pub const IL_INVALID_RATE: c_uint = 0xFF;

//
// struct il_ht_agg -- aggregation status while waiting for block-ack
// @txq_id: Tx queue used for Tx attempt
// @frame_count: # frames attempted by Tx command
// @wait_for_ba: Expect block-ack before next Tx reply
// @start_idx: Index of 1st Transmit Frame Descriptor (TFD) in Tx win
// @bitmap0: Low order bitmap, one bit for each frame pending ACK in Tx win
// @bitmap1: High order, one bit for each frame pending ACK in Tx win
// @rate_n_flags: Rate at which Tx was attempted
//
// If C_TX indicates that aggregation was attempted, driver must wait
// for block ack (N_COMPRESSED_BA).  This struct stores tx reply info
// until block ack arrives.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_ht_agg {
    pub txq_id: u16,
    pub frame_count: u16,
    pub wait_for_ba: u16,
    pub start_idx: u16,
    pub bitmap: u64,
    pub rate_n_flags: u32,
pub const IL_AGG_OFF: c_int = 0;
pub const IL_AGG_ON: c_int = 1;
pub const IL_EMPTYING_HW_QUEUE_ADDBA: c_int = 2;
pub const IL_EMPTYING_HW_QUEUE_DELBA: c_int = 3;
    pub state: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_tid_data {
    pub /: *mut *mut u16 seq_number; / 4965 only,
    pub tfds_in_queue: u16,
    pub agg: il_ht_agg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_hw_key {
    pub cipher: u32,
    pub keylen: c_int,
    pub keyidx: u8,
    pub key: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union il_ht_rate_supp {
    pub rates: u16,
    pub siso_rate: u8,
    pub mimo_rate: u8,
}

//
// Maximal MPDU density for TX aggregation
// 4 - 2us density
// 5 - 4us density
// 6 - 8us density
// 7 - 16us density
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_ht_config {
    pub single_chain_sufficient: bool,
    pub /: *mut *mut ieee80211_smps_mode smps; / current smps mode,
}

// QoS structures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_qos_info {
    pub qos_active: c_int,
    pub def_qos_parm: il_qosparam_cmd,
}

//
// Structure should be accessed with sta_lock held. When station addition
// is in progress (IL_STA_UCODE_INPROGRESS) it is possible to access only
// the commands (il_addsta_cmd and il_link_quality_cmd) without
// sta_lock held.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_station_entry {
    pub sta: il_addsta_cmd,
    pub tid: [il_tid_data; MAX_TID_COUNT],
    pub used: u8,
    pub keyinfo: il_hw_key,
    pub lq: *mut il_link_quality_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_station_priv_common {
    pub sta_id: u8,
}

//
// struct il_vif_priv - driver's ilate per-interface information
//
// When mac80211 allocates a virtual interface, it can allocate
// space for us to put data into.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_vif_priv {
    pub ibss_bssid_sta_id: u8,
}

// one for each uCode image (inst/data, boot/init/runtime)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_desc {
    pub /: *mut *mut *mut void v_addr; / access by driver,
    pub /: *mut *mut dma_addr_t p_addr; / access by card's busmaster DMA,
    pub /: *mut *mut u32 len; / bytes,
}

// uCode file layout
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_ucode_header {
    pub /: *mut *mut __le32 ver; / major/minor/API/serial,
    pub /: *mut *mut __le32 inst_size; / bytes of runtime code,
    pub /: *mut *mut __le32 data_size; / bytes of runtime data,
    pub /: *mut *mut __le32 init_size; / bytes of init code,
    pub /: *mut *mut __le32 init_data_size; / bytes of init data,
    pub /: *mut *mut __le32 boot_size; / bytes of bootstrap code,
    pub /: *mut *mut u8 data[0]; / in same order as sizes,
    pub v1: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il4965_ibss_seq {
    pub mac: [u8; ETH_ALEN],
    pub seq_num: u16,
    pub frag_num: u16,
    pub packet_time: c_ulong,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_sensitivity_ranges {
    pub min_nrg_cck: u16,
    pub max_nrg_cck: u16,
    pub nrg_th_cck: u16,
    pub nrg_th_ofdm: u16,
    pub auto_corr_min_ofdm: u16,
    pub auto_corr_min_ofdm_mrc: u16,
    pub auto_corr_min_ofdm_x1: u16,
    pub auto_corr_min_ofdm_mrc_x1: u16,
    pub auto_corr_max_ofdm: u16,
    pub auto_corr_max_ofdm_mrc: u16,
    pub auto_corr_max_ofdm_x1: u16,
    pub auto_corr_max_ofdm_mrc_x1: u16,
    pub auto_corr_max_cck: u16,
    pub auto_corr_max_cck_mrc: u16,
    pub auto_corr_min_cck: u16,
    pub auto_corr_min_cck_mrc: u16,
    pub barker_corr_th_min: u16,
    pub barker_corr_th_min_mrc: u16,
    pub nrg_th_cca: u16,
}

//
// struct il_hw_params
// @bcast_id: f/w broadcast station ID
// @max_txq_num: Max # Tx queues supported
// @dma_chnl_num: Number of Tx DMA/FIFO channels
// @scd_bc_tbls_size: size of scheduler byte count tables
// @tfd_size: TFD size
// @tx/rx_chains_num: Number of TX/RX chains
// @valid_tx/rx_ant: usable antennas
// @max_rxq_size: Max # Rx frames in Rx queue (must be power-of-2)
// @max_rxq_log: Log-base-2 of max_rxq_size
// @rx_page_order: Rx buffer page order
// @rx_wrt_ptr_reg: FH{39}_RSCSR_CHNL0_WPTR
// @max_stations:
// @ht40_channel: is 40MHz width possible in band 2.4
// BIT(NL80211_BAND_5GHZ) BIT(NL80211_BAND_5GHZ)
// @sw_crypto: 0 for hw, 1 for sw
// @max_xxx_size: for ucode uses
// @ct_kill_threshold: temperature threshold
// @beacon_time_tsf_bits: number of valid tsf bits for beacon time
// @struct il_sensitivity_ranges: range of sensitivity values
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_hw_params {
    pub bcast_id: u8,
    pub max_txq_num: u8,
    pub dma_chnl_num: u8,
    pub scd_bc_tbls_size: u16,
    pub tfd_size: u32,
    pub tx_chains_num: u8,
    pub rx_chains_num: u8,
    pub valid_tx_ant: u8,
    pub valid_rx_ant: u8,
    pub max_rxq_size: u16,
    pub max_rxq_log: u16,
    pub rx_page_order: u32,
    pub rx_wrt_ptr_reg: u32,
    pub max_stations: u8,
    pub ht40_channel: u8,
    pub /: *mut *mut u8 max_beacon_itrvl; / in 1024 ms,
    pub max_inst_size: u32,
    pub max_data_size: u32,
    pub max_bsm_size: u32,
    pub /: *mut *mut u32 ct_kill_threshold; / value in hw-dependent units,
    pub beacon_time_tsf_bits: u16,
    pub sens: *const il_sensitivity_ranges,
}

//
// Functions implemented in core module which are forward declared here
// for use by iwl-[4-5].c
//
// NOTE:  The implementation of these functions are not hardware specific
// which is why they are in the core module files.
//
// Naming convention --
// il_         <-- Is part of iwlwifi
// iwlXXXX_     <-- Hardware specific (implemented in iwl-XXXX.c for XXXX)
// il4965_bg_      <-- Called from work queue context
// il4965_mac_     <-- mac80211 callback
//
extern "C" {
    pub fn il4965_update_chain_flags(il: *mut il_priv);
}
extern "C" {
    pub fn il_queue_space(q: *const il_queue) -> c_int;
}
//
// This is for init calibration result and scan command which
// required buffer > TFD_MAX_PAYLOAD_SIZE,
// the big buffer at end of command array
//
// Otherwise, use normal size buffers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_dma_ptr {
    pub dma: dma_addr_t,
    pub addr: *mut c_void,
    pub size: usize,
}

pub const IL_OPERATION_MODE_AUTO: c_int = 0;
pub const IL_OPERATION_MODE_HT_ONLY: c_int = 1;
pub const IL_OPERATION_MODE_MIXED: c_int = 2;
pub const IL_OPERATION_MODE_20MHZ: c_int = 3;
pub const IL_TX_CRC_SIZE: c_int = 4;
pub const IL_TX_DELIMITER_SIZE: c_int = 4;

// Sensitivity and chain noise calibration
pub const INITIALIZATION_VALUE: c_uint = 0xFFFF;
pub const IL4965_CAL_NUM_BEACONS: c_int = 20;
pub const IL_CAL_NUM_BEACONS: c_int = 16;
pub const MAXIMUM_ALLOWED_PATHLOSS: c_int = 15;
pub const CHAIN_NOISE_MAX_DELTA_GAIN_CODE: c_int = 3;
pub const MAX_FA_OFDM: c_int = 50;
pub const MIN_FA_OFDM: c_int = 5;
pub const MAX_FA_CCK: c_int = 50;
pub const MIN_FA_CCK: c_int = 5;
pub const AUTO_CORR_STEP_OFDM: c_int = 1;
pub const AUTO_CORR_STEP_CCK: c_int = 3;
pub const AUTO_CORR_MAX_TH_CCK: c_int = 160;
pub const NRG_DIFF: c_int = 2;
pub const NRG_STEP_CCK: c_int = 2;
pub const NRG_MARGIN: c_int = 8;
pub const MAX_NUMBER_CCK_NO_FA: c_int = 100;

pub const CHAIN_A: c_int = 0;
pub const CHAIN_B: c_int = 1;
pub const CHAIN_C: c_int = 2;
pub const CHAIN_NOISE_DELTA_GAIN_INIT_VAL: c_int = 4;
pub const ALL_BAND_FILTER: c_uint = 0xFF00;
pub const IN_BAND_FILTER: c_uint = 0xFF;
pub const MIN_AVERAGE_NOISE_MAX_VALUE: c_uint = 0xFFFFFFFF;
pub const NRG_NUM_PREV_STAT_L: c_int = 20;
pub const NUM_RX_CHAINS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum il4965_false_alarm_state {
    IL_FA_TOO_MANY = 0,
    IL_FA_TOO_FEW = 1,
    IL_FA_GOOD_RANGE = 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum il4965_chain_noise_state {
    IL_CHAIN_NOISE_ALIVE = 0,	/* must be 0 */
    IL_CHAIN_NOISE_ACCUMULATE,
    IL_CHAIN_NOISE_CALIBRATED,
    IL_CHAIN_NOISE_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ucode_type {
    UCODE_NONE = 0,
    UCODE_INIT,
    UCODE_RT
}

// Sensitivity calib data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_sensitivity_data {
    pub auto_corr_ofdm: u32,
    pub auto_corr_ofdm_mrc: u32,
    pub auto_corr_ofdm_x1: u32,
    pub auto_corr_ofdm_mrc_x1: u32,
    pub auto_corr_cck: u32,
    pub auto_corr_cck_mrc: u32,
    pub last_bad_plcp_cnt_ofdm: u32,
    pub last_fa_cnt_ofdm: u32,
    pub last_bad_plcp_cnt_cck: u32,
    pub last_fa_cnt_cck: u32,
    pub nrg_curr_state: u32,
    pub nrg_prev_state: u32,
    pub nrg_value: [u32; 10],
    pub nrg_silence_rssi: [u8; NRG_NUM_PREV_STAT_L],
    pub nrg_silence_ref: u32,
    pub nrg_energy_idx: u32,
    pub nrg_silence_idx: u32,
    pub nrg_th_cck: u32,
    pub nrg_auto_corr_silence_diff: i32,
    pub num_in_cck_no_fa: u32,
    pub nrg_th_ofdm: u32,
    pub barker_corr_th_min: u16,
    pub barker_corr_th_min_mrc: u16,
    pub nrg_th_cca: u16,
}

// Chain noise (differential Rx gain) calib data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_chain_noise_data {
    pub active_chains: u32,
    pub chain_noise_a: u32,
    pub chain_noise_b: u32,
    pub chain_noise_c: u32,
    pub chain_signal_a: u32,
    pub chain_signal_b: u32,
    pub chain_signal_c: u32,
    pub beacon_count: u16,
    pub disconn_array: [u8; NUM_RX_CHAINS],
    pub delta_gain_code: [u8; NUM_RX_CHAINS],
    pub radio_write: u8,
    pub state: u8,
}

// interrupt stats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct isr_stats {
    pub hw: u32,
    pub sw: u32,
    pub err_code: u32,
    pub sch: u32,
    pub alive: u32,
    pub rfkill: u32,
    pub ctkill: u32,
    pub wakeup: u32,
    pub rx: u32,
    pub handlers: [u32; IL_CN_MAX],
    pub tx: u32,
    pub unhandled: u32,
}

// management stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum il_mgmt_stats {
    MANAGEMENT_ASSOC_REQ = 0,
    MANAGEMENT_ASSOC_RESP,
    MANAGEMENT_REASSOC_REQ,
    MANAGEMENT_REASSOC_RESP,
    MANAGEMENT_PROBE_REQ,
    MANAGEMENT_PROBE_RESP,
    MANAGEMENT_BEACON,
    MANAGEMENT_ATIM,
    MANAGEMENT_DISASSOC,
    MANAGEMENT_AUTH,
    MANAGEMENT_DEAUTH,
    MANAGEMENT_ACTION,
    MANAGEMENT_MAX,
}

// control stats
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum il_ctrl_stats {
    CONTROL_BACK_REQ = 0,
    CONTROL_BACK,
    CONTROL_PSPOLL,
    CONTROL_RTS,
    CONTROL_CTS,
    CONTROL_ACK,
    CONTROL_CFEND,
    CONTROL_CFENDACK,
    CONTROL_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct traffic_stats {
    pub mgmt: [u32; MANAGEMENT_MAX],
    pub ctrl: [u32; CONTROL_MAX],
    pub data_cnt: u32,
    pub data_bytes: u64,

}

//
// host interrupt timeout value
// used with setting interrupt coalescing timer
// the CSR_INT_COALESCING is an 8 bit register in 32-usec unit
//
// default interrupt coalescing timer is 64 x 32 = 2048 usecs
// default interrupt coalescing calibration timer is 16 x 32 = 512 usecs
//

// TX queue watchdog timeouts in mSecs

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_force_reset {
    pub reset_request_count: c_int,
    pub reset_success_count: c_int,
    pub reset_reject_count: c_int,
    pub reset_duration: c_ulong,
    pub last_force_reset_jiffies: c_ulong,
}

// extend beacon time format bit shifting
//
// for _3945 devices
// bits 31:24 - extended
// bits 23:0  - interval
//
pub const IL3945_EXT_BEACON_TIME_POS: c_int = 24;
//
// for _4965 devices
// bits 31:22 - extended
// bits 21:0  - interval
//
pub const IL4965_EXT_BEACON_TIME_POS: c_int = 22;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_rxon_context {
    pub vif: *mut ieee80211_vif,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_power_mgr {
    pub sleep_cmd: il_powertable_cmd,
    pub sleep_cmd_next: il_powertable_cmd,
    pub debug_sleep_level_override: c_int,
    pub pci_pm: bool,
    pub ps_disabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_priv {
    pub hw: *mut ieee80211_hw,
    pub ieee_channels: *mut ieee80211_channel,
    pub ieee_rates: *mut ieee80211_rate,
    pub cfg: *mut il_cfg,
    pub ops: *const il_ops,

    pub debugfs_ops: *const il_debugfs_ops,

// temporary frame storage list
    pub free_frames: list_head,
    pub frames_count: c_int,
    pub band: nl80211_band,
    pub alloc_rxb_page: c_int,
    pub rxb): *mut il_rx_buf,
    pub bands: [ieee80211_supported_band; NUM_NL80211_BANDS],
// spectrum measurement report caching
    pub measure_report: il_spectrum_notification,
    pub measurement_status: u8,
// ucode beacon time
    pub ucode_beacon_time: u32,
    pub missed_beacon_threshold: c_int,
// track IBSS manager (last beacon) status
    pub ibss_manager: u32,
// force reset
    pub force_reset: il_force_reset,
// we allocate array of il_channel_info for NIC's valid channels.
// Access via channel # using indirect idx array
    pub /: *mut *mut *mut il_channel_info channel_info; / channel info array,
    pub /: *mut *mut u8 channel_count; / # of channels,
// thermal calibration
    pub /: *mut *mut s32 temperature; / degrees Kelvin,
    pub last_temperature: i32,
// Scan related variables
    pub scan_start: c_ulong,
    pub scan_start_tsf: c_ulong,
    pub scan_cmd: *mut c_void,
    pub scan_band: nl80211_band,
    pub scan_request: *mut cfg80211_scan_request,
    pub scan_vif: *mut ieee80211_vif,
    pub scan_tx_ant: [u8; NUM_NL80211_BANDS],
    pub mgmt_tx_ant: u8,
// spinlock
    pub /: *mut *mut spinlock_t lock; / protect general shared data,
    pub /: *mut *mut spinlock_t hcmd_lock; / protect hcmd,
    pub /: *mut *mut spinlock_t reg_lock; / protect hw register access,
    pub mutex: mutex,
// basic pci-network driver stuff
    pub pci_dev: *mut pci_dev,
// pci hardware address support
    pub hw_base: *mut void __iomem,
    pub hw_rev: u32,
    pub hw_wa_rev: u32,
    pub rev_id: u8,
// command queue number
    pub cmd_queue: u8,
// max number of station keys
    pub sta_key_max_num: u8,
// EEPROM MAC addresses
    pub addresses: [mac_address; 1],
// uCode images, save to reload in case of failure
    pub /: *mut *mut int fw_idx; / firmware we're trying to load,
    pub of: *mut *mut u32 ucode_ver; / version of ucode, copy,
    pub /: *mut *mut fw_desc ucode_code; / runtime inst,
    pub /: *mut *mut fw_desc ucode_data; / runtime data original,
    pub /: *mut *mut fw_desc ucode_data_backup; / runtime data save/restore,
    pub /: *mut *mut fw_desc ucode_init; / initialization inst,
    pub /: *mut *mut fw_desc ucode_init_data; / initialization data,
    pub /: *mut *mut fw_desc ucode_boot; / bootstrap inst,
    pub ucode_type: ucode_type,
    pub /: *mut *mut u8 ucode_write_complete; / the image write is complete,
    pub firmware_name: [c_char; 25],
    pub vif: *mut ieee80211_vif,
    pub qos_data: il_qos_info,
    pub enabled: bool,
    pub is_40mhz: bool,
    pub non_gf_sta_present: bool,
    pub protection: u8,
    pub extension_chan_offset: u8,
    pub ht: },
//
// We declare this const so it can only be
// changed via explicit cast within the
// routines that actually update the physical
// hardware.
//
    pub active: il_rxon_cmd,
    pub staging: il_rxon_cmd,
    pub timing: il_rxon_time_cmd,
    pub switch_channel: __le16,
// 1st responses from initialize and runtime uCode images.
// _4965's initialize alive response contains some calibration data.
    pub card_alive_init: il_init_alive_resp,
    pub card_alive: il_alive_resp,
    pub active_rate: u16,
    pub start_calib: u8,
    pub sensitivity_data: il_sensitivity_data,
    pub chain_noise_data: il_chain_noise_data,
    pub sensitivity_tbl: [__le16; HD_TBL_SIZE],
    pub current_ht_config: il_ht_config,
// Rate scaling data
    pub retry_rate: u8,
    pub wait_command_queue: wait_queue_head_t,
    pub activity_timer_active: c_int,
// Rx and Tx DMA processing queues
    pub rxq: il_rx_queue,
    pub txq: *mut il_tx_queue,
    pub txq_ctx_active_msk: c_ulong,
    pub /: *mut *mut il_dma_ptr kw; / keep warm address,
    pub scd_bc_tbls: il_dma_ptr,
    pub /: *mut *mut u32 scd_base_addr; / scheduler sram base address,
    pub status: c_ulong,
// counts mgmt, ctl, and data packets
    pub tx_stats: traffic_stats,
    pub rx_stats: traffic_stats,
// counts interrupts
    pub isr_stats: isr_stats,
    pub power_data: il_power_mgr,
// context information
    pub /: *mut *mut u8 bssid[ETH_ALEN]; / used only on 3945 but filled by core,
// station table variables
// Note: if lock and sta_lock are needed, lock must be acquired first
    pub sta_lock: spinlock_t,
    pub num_stations: c_int,
    pub stations: [il_station_entry; IL_STATION_COUNT],
    pub ucode_key_table: c_ulong,
// queue refcounts
pub const IL_MAX_HW_QUEUES: c_int = 32;
    pub queue_stopped: [c_ulong; BITS_TO_LONGS(IL_MAX_HW_QUEUES)],
pub const IL_STOP_REASON_PASSIVE: c_int = 0;
    pub stop_reason: c_ulong,
// for each AC
    pub queue_stop_count: [core::sync::atomic::AtomicI32; 4],
// Indication if ieee80211_ops->open has been called
    pub is_open: u8,
    pub mac80211_registered: u8,
// eeprom -- this is in the card's little endian byte order
    pub eeprom: *mut u8,
    pub calib_info: *mut il_eeprom_calib_info,
    pub iw_mode: nl80211_iftype,
// Last Rx'd beacon timestamp
    pub timestamp: u64,

    pub shared_virt: *mut c_void,
    pub shared_phys: dma_addr_t,
    pub thermal_periodic: delayed_work,
    pub rfkill_poll: delayed_work,
    pub stats: il3945_notif_stats,

    pub accum_stats: il3945_notif_stats,
    pub delta_stats: il3945_notif_stats,
    pub max_delta: il3945_notif_stats,

    pub sta_supp_rates: u32,
    pub /: *mut *mut int last_rx_rssi; / From Rx packet stats,
// Rx'd packet timing information
    pub last_beacon_time: u32,
    pub last_tsf: u64,
//
// each calibration channel group in the
// EEPROM has a derived clip setting for
// each rate.
//
    pub clip_groups: [il3945_clip_group; 5],
    pub _3945: },

    pub last_phy_res: il_rx_phy_res,
    pub last_phy_res_valid: bool,
    pub ampdu_ref: u32,
    pub firmware_loading_complete: completion,
//
// chain noise reset and gain commands are the
// two extra calibration commands follows the standard
// phy calibration commands
//
    pub phy_calib_chain_noise_reset_cmd: u8,
    pub phy_calib_chain_noise_gain_cmd: u8,
    pub key_mapping_keys: u8,
    pub wep_keys: [il_wep_key; WEP_KEYS_MAX],
    pub stats: il_notif_stats,

    pub accum_stats: il_notif_stats,
    pub delta_stats: il_notif_stats,
    pub max_delta: il_notif_stats,

    pub _4965: },

}

// TX Power

// debugging info

// debugfs

extern "C" {
    pub fn il_is_associated(_arg: il) -> return;
}

pub const TIME_UNIT: c_int = 1024;
pub const IL_SKU_G: c_uint = 0x1;
pub const IL_SKU_A: c_uint = 0x2;
pub const IL_SKU_N: c_uint = 0x8;

// Size of one Rx buffer in host DRAM

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_debugfs_ops {
    pub ppos): *mut size_t count, loff_t,
    pub ppos): *mut size_t count, loff_t,
    pub ppos): *mut loff_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_ops {
// Handling TX
    pub byte_cnt): u16,
    pub pad): u16 len, u8 reset, u8,
    pub txq): *mut *mut *mut void (txq_free_tfd) (struct il_priv il, struct il_tx_queue,
    pub txq): *mut *mut *mut int (txq_init) (struct il_priv il, struct il_tx_queue,
// alive notification after init uCode load
    pub il): *mut *mut void (init_alive_start) (struct il_priv,
// check validity of rtc data address
    pub addr): *mut *mut int (is_valid_rtc_data_addr) (u32,
// 1st ucode load
    pub il): *mut *mut int (load_ucode) (struct il_priv,
    pub il): *mut *mut void (dump_nic_error_log) (struct il_priv,
    pub display): *mut *mut *mut *mut *mut int (dump_fh) (struct il_priv il, char buf, bool,
    pub ch_switch): *mut ieee80211_channel_switch,
// power management
    pub il): *mut *mut int (apm_init) (struct il_priv,
// tx power
    pub il): *mut *mut int (send_tx_power) (struct il_priv,
    pub il): *mut *mut void (update_chain_flags) (struct il_priv,
// eeprom operations
    pub il): *mut *mut int (eeprom_acquire_semaphore) (struct il_priv,
    pub il): *mut *mut void (eeprom_release_semaphore) (struct il_priv,
    pub il): *mut *mut int (rxon_assoc) (struct il_priv,
    pub il): *mut *mut int (commit_rxon) (struct il_priv,
    pub il): *mut *mut void (set_rxon_chain) (struct il_priv,
    pub len): *mut *mut u16(get_hcmd_size) (u8 cmd_id, u16,
    pub data): *const *const *const u16(build_addsta_hcmd) (struct il_addsta_cmd cmd, u8,
    pub vif): *mut *mut *mut int (request_scan) (struct il_priv il, struct ieee80211_vif,
    pub il): *mut *mut void (post_scan) (struct il_priv,
    pub il): *mut *mut void (post_associate) (struct il_priv,
    pub il): *mut *mut void (config_ap) (struct il_priv,
// station management
    pub il): *mut *mut int (update_bcast_stations) (struct il_priv,
    pub add): *mut *mut ieee80211_vif vif, bool,
    pub led_cmd): *mut *mut *mut int (send_led_cmd) (struct il_priv il, struct il_led_cmd,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_mod_params {
    pub /: *mut *mut int sw_crypto; / def: 0 = using hardware encryption,
    pub /: *mut *mut int disable_hw_scan; / def: 0 = use h/w scan,
    pub /: *mut *mut int num_of_queues; / def: HW dependent,
    pub /: *mut *mut int disable_11n; / def: 0 = 11n capabilities enabled,
    pub /: *mut *mut int amsdu_size_8K; / def: 0 = disable 8K amsdu size,
    pub /: *mut *mut int antenna; / def: 0 = both antennas (use diversity),
    pub /: *mut *mut int restart_fw; / def: 1 = restart firmware,
}

pub const IL_LED_SOLID: c_int = 11;

//
// LED mode
// IL_LED_DEFAULT:  use device default
// IL_LED_RF_STATE: turn LED on/off based on RF state
// LED ON  = RF ON
// LED OFF = RF OFF
// IL_LED_BLINK:    adjust led blink rate based on blink table
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum il_led_mode {
    IL_LED_DEFAULT,
    IL_LED_RF_STATE,
    IL_LED_BLINK,
}

extern "C" {
    pub fn il_leds_init(il: *mut il_priv);
}
extern "C" {
    pub fn il_leds_exit(il: *mut il_priv);
}
//
// struct il_cfg
// @fw_name_pre: Firmware filename prefix. The api version and extension
// (.ucode) will be added to filename before loading from disk. The
// filename is constructed as fw_name_pre<api>.ucode.
// @ucode_api_max: Highest version of uCode API supported by driver.
// @ucode_api_min: Lowest version of uCode API supported by driver.
// @scan_antennas: available antenna for scan operation
// @led_mode: 0=blinking, 1=On(RF On)/Off(RF Off)
//
// We enable the driver to be backward compatible wrt API version. The
// driver specifies which APIs it supports (with @ucode_api_max being the
// highest and @ucode_api_min the lowest). Firmware will only be loaded if
// it has a supported API version. The firmware's API version will be
// stored in @il_priv, enabling the driver to make runtime changes based
// on firmware version used.
//
// For example,
// if (IL_UCODE_API(il->ucode_ver) >= 2) {
// Driver interacts with Firmware API version >= 2.
// } else {
// Driver interacts with Firmware API version 1.
// }
//
// The ideal usage of this infrastructure is to treat a new ucode API
// release as a new hardware revision. That is, through utilizing the
// il_hcmd_utils_ops etc. we accommodate different command structures
// and flows between hardware versions as well as their API
// versions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_cfg {
// params specific to an individual device within a device family
    pub name: *const c_char,
    pub fw_name_pre: *const c_char,
    pub ucode_api_max: c_uint,
    pub ucode_api_min: c_uint,
    pub valid_tx_ant: u8,
    pub valid_rx_ant: u8,
    pub sku: c_uint,
    pub eeprom_ver: u16,
    pub eeprom_calib_ver: u16,
// module based parameters which can be set from modprobe cmd
    pub mod_params: *const il_mod_params,
// params not likely to change within a device family
    pub base_params: *mut il_base_params,
// params likely to change within a device family
    pub scan_rx_antennas: [u8; NUM_NL80211_BANDS],
    pub led_mode: il_led_mode,
    pub eeprom_size: c_int,
    pub /: *mut *mut int num_of_queues; / def: HW dependent,
    pub /: *mut *mut int num_of_ampdu_queues; / def: HW dependent,
// for il_apm_init()
    pub pll_cfg_val: u32,
    pub set_l0s: bool,
    pub use_bsm: bool,
    pub led_compensation: u16,
    pub chain_noise_num_beacons: c_int,
    pub wd_timeout: c_uint,
    pub temperature_kelvin: bool,
    pub ucode_tracing: bool,
    pub sensitivity_calib_by_driver: bool,
    pub chain_noise_calib_by_driver: bool,
    pub regulatory_bands: [u32; 7],
}

//
// L i b
//
extern "C" {
    pub fn il_mac_tx_last_beacon(hw: *mut ieee80211_hw) -> c_int;
}
extern "C" {
    pub fn il_set_rxon_hwcrypto(il: *mut il_priv, hw_decrypt: c_int);
}
extern "C" {
    pub fn il_check_rxon_cmd(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_full_rxon_required(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_set_rxon_channel(il: *mut il_priv, ch: *mut ieee80211_channel) -> c_int;
}
extern "C" {
    pub fn il_set_rxon_ht(il: *mut il_priv, ht_conf: *mut il_ht_config);
}
extern "C" {
    pub fn il_connection_init_rx_config(il: *mut il_priv);
}
extern "C" {
    pub fn il_set_rate(il: *mut il_priv);
}
extern "C" {
    pub fn il_irq_handle_error(il: *mut il_priv);
}
extern "C" {
    pub fn il_mac_add_interface(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif) -> c_int;
}
extern "C" {
    pub fn il_alloc_txq_mem(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_free_txq_mem(il: *mut il_priv);
}

extern "C" {
    pub fn il_update_stats(il: *mut il_priv, is_tx: bool, fc: __le16, len: u16);
}

//
// Handlers
//
extern "C" {
    pub fn il_hdl_pm_sleep(il: *mut il_priv, rxb: *mut il_rx_buf);
}
extern "C" {
    pub fn il_hdl_pm_debug_stats(il: *mut il_priv, rxb: *mut il_rx_buf);
}
extern "C" {
    pub fn il_hdl_error(il: *mut il_priv, rxb: *mut il_rx_buf);
}
extern "C" {
    pub fn il_hdl_csa(il: *mut il_priv, rxb: *mut il_rx_buf);
}
//
// RX
//
extern "C" {
    pub fn il_cmd_queue_unmap(il: *mut il_priv);
}
extern "C" {
    pub fn il_cmd_queue_free(il: *mut il_priv);
}
extern "C" {
    pub fn il_rx_queue_alloc(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_rx_queue_update_write_ptr(il: *mut il_priv, q: *mut il_rx_queue);
}
extern "C" {
    pub fn il_rx_queue_space(q: *const il_rx_queue) -> c_int;
}
extern "C" {
    pub fn il_tx_cmd_complete(il: *mut il_priv, rxb: *mut il_rx_buf);
}
extern "C" {
    pub fn il_hdl_spectrum_measurement(il: *mut il_priv, rxb: *mut il_rx_buf);
}
extern "C" {
    pub fn il_recover_from_stats(il: *mut il_priv, pkt: *mut il_rx_pkt);
}
extern "C" {
    pub fn il_chswitch_done(il: *mut il_priv, is_success: bool);
}
//
// TX
//
extern "C" {
    pub fn il_txq_update_write_ptr(il: *mut il_priv, txq: *mut il_tx_queue);
}
extern "C" {
    pub fn il_tx_queue_init(il: *mut il_priv, txq_id: u32) -> c_int;
}
extern "C" {
    pub fn il_tx_queue_reset(il: *mut il_priv, txq_id: u32);
}
extern "C" {
    pub fn il_tx_queue_unmap(il: *mut il_priv, txq_id: c_int);
}
extern "C" {
    pub fn il_tx_queue_free(il: *mut il_priv, txq_id: c_int);
}
extern "C" {
    pub fn il_setup_watchdog(il: *mut il_priv);
}
//
// TX power
//
extern "C" {
    pub fn il_set_tx_power(il: *mut il_priv, tx_power: i8, force: bool) -> c_int;
}
//
// Rate
//
extern "C" {
    pub fn il_get_lowest_plcp(il: *mut il_priv) -> u8;
}
//
// Scanning
//
extern "C" {
    pub fn il_init_scan_params(il: *mut il_priv);
}
extern "C" {
    pub fn il_scan_cancel(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_scan_cancel_timeout(il: *mut il_priv, ms: c_ulong) -> c_int;
}
extern "C" {
    pub fn il_force_scan_end(il: *mut il_priv);
}
extern "C" {
    pub fn il_internal_short_hw_scan(il: *mut il_priv);
}
extern "C" {
    pub fn il_force_reset(il: *mut il_priv, external: bool) -> c_int;
}
extern "C" {
    pub fn il_setup_rx_scan_handlers(il: *mut il_priv);
}
extern "C" {
    pub fn il_setup_scan_deferred_work(il: *mut il_priv);
}
extern "C" {
    pub fn il_cancel_scan_deferred_work(il: *mut il_priv);
}
// For faster active scanning, scan will move to the next channel if fewer than
// PLCP_QUIET_THRESH packets are heard on this channel within
// ACTIVE_QUIET_TIME after sending probe request.  This shortens the dwell
// time if it's a quiet channel (nothing responded to our probe, and there's
// no other traffic).
// Disable "quiet" feature by setting PLCP_QUIET_THRESH to 0.

//
// S e n d i n g     H o s t     C o m m a n d s
//
extern "C" {
    pub fn il_send_cmd_sync(il: *mut il_priv, cmd: *mut il_host_cmd) -> int __must_check;
}
extern "C" {
    pub fn il_send_cmd(il: *mut il_priv, cmd: *mut il_host_cmd) -> c_int;
}
extern "C" {
    pub fn il_enqueue_hcmd(il: *mut il_priv, cmd: *mut il_host_cmd) -> c_int;
}
//
// PCI
//
extern "C" {
    pub fn il_bg_watchdog(t: *mut timer_list);
}
extern "C" {
    pub fn il_usecs_to_beacons(il: *mut il_priv, usec: u32, beacon_interval: u32) -> u32;
}

//
// Error Handling Debugging
//
extern "C" {
    pub fn il4965_dump_nic_error_log(il: *mut il_priv);
}

extern "C" {
    pub fn il_print_rx_config_cmd(il: *mut il_priv);
}

extern "C" {
    pub fn il_clear_isr_stats(il: *mut il_priv);
}
//
// GEOS
//
extern "C" {
    pub fn il_init_geos(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_free_geos(il: *mut il_priv);
}
// DRIVER STATUS FUNCTIONS

// 1 is unused (used to be S_HCMD_SYNC_ACTIVE)
pub const S_INT_ENABLED: c_int = 2;
pub const S_RFKILL: c_int = 3;
pub const S_CT_KILL: c_int = 4;
pub const S_INIT: c_int = 5;
pub const S_ALIVE: c_int = 6;
pub const S_READY: c_int = 7;
pub const S_TEMPERATURE: c_int = 8;
pub const S_GEO_CONFIGURED: c_int = 9;
pub const S_EXIT_PENDING: c_int = 10;
pub const S_STATS: c_int = 12;
pub const S_SCANNING: c_int = 13;
pub const S_SCAN_ABORTING: c_int = 14;
pub const S_SCAN_HW: c_int = 15;
pub const S_POWER_PMI: c_int = 16;
pub const S_FW_ERROR: c_int = 17;
pub const S_CHANNEL_SWITCH_PENDING: c_int = 18;
// The adapter is 'ready' if READY and GEO_CONFIGURED bits are
// set but EXIT_PENDING is not
extern "C" {
    pub fn test_bit(_arg: S_ALIVE, _arg: &il->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: S_INIT, _arg: &il->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: S_RFKILL, _arg: &il->status) -> return;
}
extern "C" {
    pub fn test_bit(_arg: S_CT_KILL, _arg: &il->status) -> return;
}
extern "C" {
    pub fn il_is_ready(_arg: il) -> return;
}
extern "C" {
    pub fn il_send_bt_config(il: *mut il_priv);
}
extern "C" {
    pub fn il_send_stats_request(il: *mut il_priv, flags: u8, clear: bool) -> c_int;
}
extern "C" {
    pub fn il_apm_stop(il: *mut il_priv);
}
extern "C" {
    pub fn _il_apm_stop(il: *mut il_priv);
}
extern "C" {
    pub fn il_apm_init(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_send_rxon_timing(il: *mut il_priv) -> c_int;
}
// mac80211 handlers
extern "C" {
    pub fn il_mac_config(hw: *mut ieee80211_hw, radio_idx: c_int, changed: u32) -> c_int;
}
extern "C" {
    pub fn il_mac_reset_tsf(hw: *mut ieee80211_hw, vif: *mut ieee80211_vif);
}
extern "C" {
    pub fn il_isr(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn il_set_bit(p: *mut il_priv, r: u32, m: u32);
}
extern "C" {
    pub fn il_clear_bit(p: *mut il_priv, r: u32, m: u32);
}
extern "C" {
    pub fn _il_grab_nic_access(il: *mut il_priv) -> bool;
}
extern "C" {
    pub fn _il_poll_bit(il: *mut il_priv, addr: u32, bits: u32, mask: u32, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn il_poll_bit(il: *mut il_priv, addr: u32, mask: u32, timeout: c_int) -> c_int;
}
extern "C" {
    pub fn il_rd_prph(il: *mut il_priv, reg: u32) -> u32;
}
extern "C" {
    pub fn il_wr_prph(il: *mut il_priv, addr: u32, val: u32);
}
extern "C" {
    pub fn il_read_targ_mem(il: *mut il_priv, addr: u32) -> u32;
}
extern "C" {
    pub fn il_write_targ_mem(il: *mut il_priv, addr: u32, val: u32);
}
// Reclaim a command buffer only if this packet is a response
// to a (driver-originated) command. If the packet (e.g. Rx frame)
// originated from uCode, there is no command buffer to reclaim.
// Ucode should set SEQ_RX_FRAME bit if ucode-originated, but
// apparently a few don't get set; catch them here.
//

extern "C" {
    pub fn readl(ofs: il->hw_base +) -> return;
}
extern "C" {
    pub fn _il_rd(_arg: il, _arg: HBUS_TARG_PRPH_RDAT) -> return;
}
pub const HW_KEY_DYNAMIC: c_int = 0;
pub const HW_KEY_DEFAULT: c_int = 1;

extern "C" {
    pub fn il_restore_stations(il: *mut il_priv);
}
extern "C" {
    pub fn il_clear_ucode_stations(il: *mut il_priv);
}
extern "C" {
    pub fn il_dealloc_bcast_stations(il: *mut il_priv);
}
extern "C" {
    pub fn il_get_free_ucode_key_idx(il: *mut il_priv) -> c_int;
}
extern "C" {
    pub fn il_send_add_sta(il: *mut il_priv, sta: *mut il_addsta_cmd, flags: u8) -> c_int;
}
extern "C" {
    pub fn il_remove_station(il: *mut il_priv, sta_id: u8, addr: *const *const u8) -> c_int;
}
//
// il_clear_driver_stations - clear knowledge of all stations from driver
// @il: iwl il struct
//
// This is called during il_down() to make sure that in the case
// we're coming there from a hardware restart mac80211 will be
// able to reconfigure stations -- if we're getting there in the
// normal down flow then the stations will already be cleared.
//
// il_sta_id_or_broadcast - return sta_id or broadcast sta
// @il: iwl il
// @context: the current context
// @sta: mac80211 station
//
// In certain circumstances mac80211 passes a station pointer
// that may be %NULL, for example during TX or key setup. In
// that case, we need to use the broadcast station, so this
// inline wraps that pattern.
//
// mac80211 should not be passing a partially
// initialised station!
//
// il_queue_inc_wrap - increment queue idx, wrap back to beginning
// @idx -- current idx
// @n_bd -- total number of entries in queue (must be power of 2)
//
// il_queue_dec_wrap - decrement queue idx, wrap back to end
// @idx -- current idx
// @n_bd -- total number of entries in queue (must be power of 2)
//
// TODO: Move fw_desc functions to iwl-pci.ko
//
// we have 8 bits used like this:
//
// 7 6 5 4 3 2 1 0
// | | | | | | | |
// | | | | | | +-+-------- AC queue (0-3)
// | | | | | |
// | +-+-+-+-+------------ HW queue ID
// |
// +---------------------- unused
//

// disable interrupts from uCode/NIC to host
// acknowledge/clear/reset any interrupts still pending
// from uCode or flow handler (Rx/Tx DMA)
//
// il_beacon_time_mask_low - mask of lower 32 bit of beacon time
// @il -- pointer to il_priv data structure
// @tsf_bits -- number of bits need to shift for masking)
//
// il_beacon_time_mask_high - mask of higher 32 bit of beacon time
// @il -- pointer to il_priv data structure
// @tsf_bits -- number of bits need to shift for masking)
//
// struct il_rb_status - reseve buffer status host memory mapped FH registers
//
// @closed_rb_num [0:11] - Indicates the idx of the RB which was closed
// @closed_fr_num [0:11] - Indicates the idx of the RX Frame which was closed
// @finished_rb_num [0:11] - Indicates the idx of the current RB
// in which the last frame was written to
// @finished_fr_num [0:11] - Indicates the idx of the RX Frame
// which was transferred
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_rb_status {
    pub closed_rb_num: __le16,
    pub closed_fr_num: __le16,
    pub finished_rb_num: __le16,
    pub finished_fr_nam: __le16,
    pub /: *mut *mut __le32 __unused; / 3945 only,
    pub __packed: },
pub const TFD_QUEUE_SIZE_MAX: c_int = 256;
pub const TFD_QUEUE_SIZE_BC_DUP: c_int = 64;

pub const IL_NUM_OF_TBS: c_int = 20;
    pub 0xF: return (sizeof(addr) > sizeof(u32) ? (addr >> 16) >> 16 : 0) &,
//
// struct il_tfd_tb transmit buffer descriptor within transmit frame descriptor
//
// This structure contains dma address and length of transmission address
//
// @lo: low [31:0] portion of the dma address of TX buffer every even is
// unaligned on 16 bit boundary
// @hi_n_len: 0-3 [35:32] portion of dma
// 4-15 length of the tx buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_tfd_tb {
    pub lo: __le32,
    pub hi_n_len: __le16,
    pub __packed: },
//
// struct il_tfd
//
// Transmit Frame Descriptor (TFD)
//
// @ __reserved1[3] reserved
// @ num_tbs 0-4 number of active tbs
// 5   reserved
// 6-7 padding (not used)
// @ tbs[20]	transmit frame buffer descriptors
// @ __pad	padding
//
// Each Tx queue uses a circular buffer of 256 TFDs stored in host DRAM.
// Both driver and device share these circular buffers, each of which must be
// contiguous 256 TFDs x 128 bytes-per-TFD = 32 KBytes
//
// Driver must indicate the physical address of the base of each
// circular buffer via the FH49_MEM_CBBC_QUEUE registers.
//
// Each TFD contains pointer/size information for up to 20 data buffers
// in host DRAM.  These buffers collectively contain the (one) frame described
// by the TFD.  Each buffer must be a single contiguous block of memory within
// itself, but buffers may be scattered in host DRAM.  Each buffer has max size
// of (4K - 4).  The concatenates all of a TFD's buffers into a single
// Tx frame, up to 8 KBytes in size.
//
// A maximum of 255 (not 256!) TFDs may be on a queue waiting for Tx.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_tfd {
    pub __reserved1: [u8; 3],
    pub num_tbs: u8,
    pub tbs: [il_tfd_tb; IL_NUM_OF_TBS],
    pub __pad: __le32,
    pub __packed: },
// PCI registers
pub const PCI_CFG_RETRY_TIMEOUT: c_uint = 0x041;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_rate_info {
    pub /: *mut *mut u8 plcp; / uCode API: RATE_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_siso; / uCode API: RATE_SISO_6M_PLCP, etc.,
    pub /: *mut *mut u8 plcp_mimo2; / uCode API: RATE_MIMO2_6M_PLCP, etc.,
    pub /: *mut *mut u8 ieee; / MAC header: RATE_6M_IEEE, etc.,
    pub /: *mut *mut u8 prev_ieee; / previous rate in IEEE speeds,
    pub /: *mut *mut u8 next_ieee; / next rate in IEEE speeds,
    pub /: *mut *mut u8 prev_rs; / previous rate used in rs algo,
    pub /: *mut *mut u8 next_rs; / next rate used in rs algo,
    pub /: *mut *mut u8 prev_rs_tgg; / previous rate used in TGG rs algo,
    pub /: *mut *mut u8 next_rs_tgg; / next rate used in TGG rs algo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il3945_rate_info {
    pub /: *mut *mut u8 plcp; / uCode API: RATE_6M_PLCP, etc.,
    pub /: *mut *mut u8 ieee; / MAC header: RATE_6M_IEEE, etc.,
    pub /: *mut *mut u8 prev_ieee; / previous rate in IEEE speeds,
    pub /: *mut *mut u8 next_ieee; / next rate in IEEE speeds,
    pub /: *mut *mut u8 prev_rs; / previous rate used in rs algo,
    pub /: *mut *mut u8 next_rs; / next rate used in rs algo,
    pub /: *mut *mut u8 prev_rs_tgg; / previous rate used in TGG rs algo,
    pub /: *mut *mut u8 next_rs_tgg; / next rate used in TGG rs algo,
    pub /: *mut *mut u8 table_rs_idx; / idx in rate scale table cmd,
    pub /: *mut *mut u8 prev_table_rs; / prev in rate table cmd,
}

//
// These serve as idxes into
// struct il_rate_info il_rates[RATE_COUNT];
//
// #define vs. enum to keep from defaulting to 'large integer'

// uCode API values for legacy bit rates, both OFDM and CCK
// FIXME:RS:add RATE_LEGACY_INVM_PLCP = 0,
// uCode API values for OFDM high-throughput (HT) bit rates
// MAC header values for bit rates

pub const IL_MAX_RSSI_VAL: c_int = 0;
// These values specify how many Tx frame attempts before
// searching for a new modulation mode
pub const IL_LEGACY_FAILURE_LIMIT: c_int = 160;
pub const IL_LEGACY_SUCCESS_LIMIT: c_int = 480;
pub const IL_LEGACY_TBL_COUNT: c_int = 160;
pub const IL_NONE_LEGACY_FAILURE_LIMIT: c_int = 400;
pub const IL_NONE_LEGACY_SUCCESS_LIMIT: c_int = 4500;
pub const IL_NONE_LEGACY_TBL_COUNT: c_int = 1500;
// Success ratio (ACKed / attempted tx frames) values (perfect is 128 * 100)

// possible actions when in legacy mode
pub const IL_LEGACY_SWITCH_ANTENNA1: c_int = 0;
pub const IL_LEGACY_SWITCH_ANTENNA2: c_int = 1;
pub const IL_LEGACY_SWITCH_SISO: c_int = 2;
pub const IL_LEGACY_SWITCH_MIMO2_AB: c_int = 3;
pub const IL_LEGACY_SWITCH_MIMO2_AC: c_int = 4;
pub const IL_LEGACY_SWITCH_MIMO2_BC: c_int = 5;
// possible actions when in siso mode
pub const IL_SISO_SWITCH_ANTENNA1: c_int = 0;
pub const IL_SISO_SWITCH_ANTENNA2: c_int = 1;
pub const IL_SISO_SWITCH_MIMO2_AB: c_int = 2;
pub const IL_SISO_SWITCH_MIMO2_AC: c_int = 3;
pub const IL_SISO_SWITCH_MIMO2_BC: c_int = 4;
pub const IL_SISO_SWITCH_GI: c_int = 5;
// possible actions when in mimo mode
pub const IL_MIMO2_SWITCH_ANTENNA1: c_int = 0;
pub const IL_MIMO2_SWITCH_ANTENNA2: c_int = 1;
pub const IL_MIMO2_SWITCH_SISO_A: c_int = 2;
pub const IL_MIMO2_SWITCH_SISO_B: c_int = 3;
pub const IL_MIMO2_SWITCH_SISO_C: c_int = 4;
pub const IL_MIMO2_SWITCH_GI: c_int = 5;

// load per tid defines for A-MPDU activation
pub const IL_AGG_TPT_THREHOLD: c_int = 0;
pub const IL_AGG_LOAD_THRESHOLD: c_int = 10;
pub const IL_AGG_ALL_TID: c_uint = 0xff;

pub const TID_QUEUE_MAX_SIZE: c_int = 20;

pub const TID_MAX_LOAD_COUNT: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum il_table_type {
    LQ_NONE,
    LQ_G,			/* legacy types */
    LQ_A,
    LQ_SISO,		/* high-throughput types */
    LQ_MIMO2,
    LQ_MAX,
}

pub const ANT_NONE: c_uint = 0x0;

pub const IL_MAX_MCS_DISPLAY_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_rate_mcs_info {
    pub mbps: [c_char; IL_MAX_MCS_DISPLAY_SIZE],
    pub mcs: [c_char; IL_MAX_MCS_DISPLAY_SIZE],
}

//
// struct il_rate_scale_data -- tx success history for one rate
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_rate_scale_data {
    pub /: *mut *mut u64 data; / bitmap of successful frames,
    pub /: *mut *mut s32 success_counter; / number of frames successful,
    pub /: *mut *mut *mut s32 success_ratio; / per-cent  128,
    pub /: *mut *mut s32 counter; / number of frames attempted,
    pub /: *mut *mut *mut s32 average_tpt; / success ratio  expected throughput,
    pub stamp: c_ulong,
}

//
// struct il_scale_tbl_info -- tx params and success history for all rates
//
// There are two of these in struct il_lq_sta,
// one for "active", and one for "search".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_scale_tbl_info {
    pub lq_type: il_table_type,
    pub ant_type: u8,
    pub /: *mut *mut u8 is_SGI; / 1 = short guard interval,
    pub /: *mut *mut u8 is_ht40; / 1 = 40 MHz channel width,
    pub /: *mut *mut u8 is_dup; / 1 = duplicated data streams,
    pub /: *mut *mut *mut u8 action; / change modulation; IL_[LEGACY/SISO/MIMO]_SWITCH_,
    pub /: *mut *mut u8 max_search; / maximun number of tables we can search,
    pub /: *mut *mut *mut s32 expected_tpt; / throughput metrics; expected_tpt_G, etc.,
    pub /: *mut *mut u32 current_rate; / rate_n_flags, uCode API format,
    pub /: *mut *mut il_rate_scale_data win[RATE_COUNT]; / rate histories,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_traffic_load {
    pub /: *mut *mut unsigned long time_stamp; / age of the oldest stats,
    pub time: *mut *mut u32 packet_count[TID_QUEUE_MAX_SIZE]; / packet count in this,
// slice
    pub the: *mut *mut u32 total; / total num of packets during,
// last TID_MAX_TIME_DIFF
    pub has: *mut *mut u8 queue_count; / number of queues that,
// been used since the last cleanup
    pub /: *mut *mut u8 head; / start of the circular buffer,
}

//
// struct il_lq_sta -- driver's rate scaling ilate structure
//
// Pointer to this gets passed back and forth between driver and mac80211.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_lq_sta {
    pub /: *mut *mut u8 active_tbl; / idx of active table, range 0-1,
    pub /: *mut *mut u8 enable_counter; / indicates HT mode,
    pub /: *mut *mut u8 stay_in_tbl; / 1: disallow, 0: allow search for new mode,
    pub /: *mut *mut u8 search_better_tbl; / 1: currently trying alternate mode,
    pub last_tpt: i32,
// The following determine when to search for a new mode
    pub table_count_limit: u32,
    pub /: *mut *mut u32 max_failure_limit; / # failed frames before new search,
    pub /: *mut *mut u32 max_success_limit; / # successful frames before new search,
    pub table_count: u32,
    pub /: *mut *mut u32 total_failed; / total failed frames, any/all rates,
    pub /: *mut *mut u32 total_success; / total successful frames, any/all rates,
    pub /: *mut *mut u64 flush_timer; / time staying in mode before new search,
    pub /: *mut *mut u8 action_counter; / # mode-switch actions tried,
    pub is_green: u8,
    pub is_dup: u8,
    pub band: nl80211_band,
// The following are bitmaps of rates; RATE_6M_MASK, etc.
    pub supp_rates: u32,
    pub active_legacy_rate: u16,
    pub active_siso_rate: u16,
    pub active_mimo2_rate: u16,
    pub /: *mut *mut s8 max_rate_idx; / Max rate set by user,
    pub missed_rate_counter: u8,
    pub lq: il_link_quality_cmd,
    pub /: *mut *mut il_scale_tbl_info lq_info[LQ_SIZE]; / "active", "search",
    pub load: [il_traffic_load; TID_MAX_LOAD_COUNT],
    pub tx_agg_tid_en: u8,
    pub dbg_fixed_rate: u32,
    pub drv: *mut il_priv,
// used to be in sta_info
    pub last_txrate_idx: c_int,
// last tx rate_n_flags
    pub last_rate_n_flags: u32,
// packets destined for this STA are aggregated
    pub is_agg: u8,
}

//
// il_station_priv: Driver's ilate station information
//
// When mac80211 creates a station it reserves some space (hw->sta_data_size)
// in the structure for use by driver. This structure is places in that
// space.
//
// The common struct MUST be first because it is shared between
// 3945 and 4965!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct il_station_priv {
    pub common: il_station_priv_common,
    pub lq_sta: il_lq_sta,
    pub pending_frames: core::sync::atomic::AtomicI32,
    pub client: bool,
    pub asleep: bool,
}

//
// il3945_rate_scale_init - Initialize the rate scale table based on assoc info
//
// The specific throughput table used is based on the type of network
// the associated with, including A, B, G, and G w/ TGG protection
//
extern "C" {
    pub fn il3945_rate_scale_init(hw: *mut ieee80211_hw, sta_id: i32);
}
// Initialize station's rate scaling information after adding station
//
// il_rate_control_register - Register the rate control algorithm callbacks
//
// Since the rate control algorithm is hardware specific, there is no need
// or reason to place it as a stand alone module.  The driver can call
// il_rate_control_register in order to register the rate control callbacks
// with the mac80211 subsystem.  This should be performed prior to calling
// ieee80211_register_hw
//
extern "C" {
    pub fn il4965_rate_control_register() -> c_int;
}
extern "C" {
    pub fn il3945_rate_control_register() -> c_int;
}
//
// il_rate_control_unregister - Unregister the rate control callbacks
//
// This should be called after calling ieee80211_unregister_hw, but before
// the driver is unloaded.
//
extern "C" {
    pub fn il4965_rate_control_unregister();
}
extern "C" {
    pub fn il3945_rate_control_unregister();
}
extern "C" {
    pub fn il_power_update_mode(il: *mut il_priv, force: bool) -> c_int;
}
extern "C" {
    pub fn il_power_initialize(il: *mut il_priv);
}

//
// il_get_debug_level: Return active debug level for device
//
// Using sysfs it is possible to set per device debug level. This debug
// level will be used if set, otherwise the global debug level which can be
// set via module parameter is used.
//

extern "C" {
    pub fn il_dbgfs_register(il: *mut il_priv, name: *const c_char);
}
extern "C" {
    pub fn il_dbgfs_unregister(il: *mut il_priv);
}

//
// To use the debug system:
//
// If you are defining a new debug classification, simply add it to the #define
// list here in the form of
//
// #define IL_DL_xxxx VALUE
//
// where xxxx should be the name of the classification (for example, WEP).
//
// You then need to either add a IL_xxxx_DEBUG() macro definition for your
// classification, or use IL_DBG(IL_DL_xxxx, ...) whenever you want
// to send output to that classification.
//
// The active debug levels can be accessed via files
//
// /sys/module/iwl4965/parameters/debug
// /sys/module/iwl3945/parameters/debug
// /sys/class/net/wlan0/device/debug_level
//
// when CONFIG_IWLEGACY_DEBUG=y.
//
// 0x0000000F - 0x00000001

// 0x000000F0 - 0x00000010

// 0x00000F00 - 0x00000100

// 0x0000F000 - 0x00001000

// 0x000F0000 - 0x00010000

// 0x00F00000 - 0x00100000

// 0x0F000000 - 0x01000000

// 0xF0000000 - 0x10000000

