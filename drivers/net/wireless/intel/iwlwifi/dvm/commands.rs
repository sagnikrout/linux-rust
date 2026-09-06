//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/dvm/commands.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2005-2014, 2023-2025 Intel Corporation
//
// Please use this file (commands.h) only for uCode API definitions.
// Please use iwl-xxxx-hw.h for hardware-related definitions.
// Please use dev.h for driver implementation definitions.
//

// Macro flag: #define __iwl_commands_h__

// RXON and QOS commands
// Multi-Station support
// Security
// RX, TX, LEDs
// WiMAX coexistence
// Calibration
// 802.11h related
// Power Management
// Scan commands and notifications
// IBSS/AP commands
// Miscellaneous commands
// Bluetooth device coexistence config command
// Statistics
// RF-KILL commands and notifications
// Missed beacons notification
// BT Coex
// PAN commands
//
// Minimum number of queues. MAX_NUM is defined in hw specific files.
// Set the minimum to accommodate
// - 4 standard TX queues
// - the command queue
// - 4 PAN TX queues
// - the PAN multicast queue, and
// - the AUX (TX during scan dwell) queue.
//
pub const IWL_MIN_NUM_QUEUES: c_int = 11;
//
// Command queue depends on iPAN support.
//
pub const IWL_DEFAULT_CMD_QUEUE_NUM: c_int = 4;
pub const IWL_IPAN_CMD_QUEUE_NUM: c_int = 9;

pub const IWL_TX_FIFO_BE: c_int = 1;

pub const IWL_TX_FIFO_VO: c_int = 3;

pub const IWL_TX_FIFO_BE_IPAN: c_int = 4;

pub const IWL_TX_FIFO_VO_IPAN: c_int = 5;
// re-uses the VO FIFO, uCode will properly flush/schedule
pub const IWL_TX_FIFO_AUX: c_int = 5;
pub const IWL_TX_FIFO_UNUSED: c_int = 255;
pub const IWLAGN_CMD_FIFO_NUM: c_int = 7;
//
// This queue number is required for proper operation
// because the ucode will stop/start the scheduler as
// required.
//
pub const IWL_IPAN_MCAST_QUEUE: c_int = 8;
//
// (0)
// Commonly used structures and definitions:
// Command header, rate_n_flags, txpower
//
// iwlagn rate_n_flags bit fields
//
// rate_n_flags format is used in following iwlagn commands:
// REPLY_RX (response only)
// REPLY_RX_MPDU (response only)
// REPLY_TX (both command and response)
// REPLY_TX_LINK_QUALITY_CMD
//
// High-throughput (HT) rate format for bits 7:0 (bit 8 must be "1"):
// 2-0:  0)   6 Mbps
// 1)  12 Mbps
// 2)  18 Mbps
// 3)  24 Mbps
// 4)  36 Mbps
// 5)  48 Mbps
// 6)  54 Mbps
// 7)  60 Mbps
//
// 4-3:  0)  Single stream (SISO)
// 1)  Dual stream (MIMO)
// 2)  Triple stream (MIMO)
//
// 5:  Value of 0x20 in bits 7:0 indicates 6 Mbps HT40 duplicate data
//
// Legacy OFDM rate format for bits 7:0 (bit 8 must be "0", bit 9 "0"):
// 3-0:  0xD)   6 Mbps
// 0xF)   9 Mbps
// 0x5)  12 Mbps
// 0x7)  18 Mbps
// 0x9)  24 Mbps
// 0xB)  36 Mbps
// 0x1)  48 Mbps
// 0x3)  54 Mbps
//
// Legacy CCK rate format for bits 7:0 (bit 8 must be "0", bit 9 "1"):
// 6-0:   10)  1 Mbps
// 20)  2 Mbps
// 55)  5.5 Mbps
// 110)  11 Mbps
//
pub const RATE_MCS_CODE_MSK: c_uint = 0x7;
pub const RATE_MCS_SPATIAL_POS: c_int = 3;
pub const RATE_MCS_SPATIAL_MSK: c_uint = 0x18;
pub const RATE_MCS_HT_DUP_POS: c_int = 5;
pub const RATE_MCS_HT_DUP_MSK: c_uint = 0x20;
// Both legacy and HT use bits 7:0 as the CCK/OFDM rate or HT MCS
pub const RATE_MCS_RATE_MSK: c_uint = 0xff;
// Bit 8: (1) HT format, (0) legacy format in bits 7:0
pub const RATE_MCS_FLAGS_POS: c_int = 8;
pub const RATE_MCS_HT_POS: c_int = 8;
pub const RATE_MCS_HT_MSK: c_uint = 0x100;
// Bit 9: (1) CCK, (0) OFDM.  HT (bit 8) must be "0" for this bit to be valid
pub const RATE_MCS_CCK_POS: c_int = 9;
pub const RATE_MCS_CCK_MSK: c_uint = 0x200;
// Bit 10: (1) Use Green Field preamble
pub const RATE_MCS_GF_POS: c_int = 10;
pub const RATE_MCS_GF_MSK: c_uint = 0x400;
// Bit 11: (1) Use 40Mhz HT40 chnl width, (0) use 20 MHz legacy chnl width
pub const RATE_MCS_HT40_POS: c_int = 11;
pub const RATE_MCS_HT40_MSK: c_uint = 0x800;
// Bit 12: (1) Duplicate data on both 20MHz chnls. HT40 (bit 11) must be set.
pub const RATE_MCS_DUP_POS: c_int = 12;
pub const RATE_MCS_DUP_MSK: c_uint = 0x1000;
// Bit 13: (1) Short guard interval (0.4 usec), (0) normal GI (0.8 usec)
pub const RATE_MCS_SGI_POS: c_int = 13;
pub const RATE_MCS_SGI_MSK: c_uint = 0x2000;
//
// rate_n_flags Tx antenna masks
// bit14:16
//
pub const RATE_MCS_ANT_POS: c_int = 14;
pub const RATE_MCS_ANT_A_MSK: c_uint = 0x04000;
pub const RATE_MCS_ANT_B_MSK: c_uint = 0x08000;
pub const RATE_MCS_ANT_C_MSK: c_uint = 0x10000;

pub const RATE_ANT_NUM: c_int = 3;
pub const POWER_TABLE_NUM_ENTRIES: c_int = 33;
pub const POWER_TABLE_NUM_HT_OFDM_ENTRIES: c_int = 32;
pub const POWER_TABLE_CCK_ENTRY: c_int = 32;
pub const IWL_PWR_NUM_HT_OFDM_ENTRIES: c_int = 24;
pub const IWL_PWR_CCK_ENTRIES: c_int = 2;
//
// struct tx_power_dual_stream
//
// Table entries in REPLY_TX_PWR_TABLE_CMD, REPLY_CHANNEL_SWITCH
//
// Same format as iwl_tx_power_dual_stream, but __le32
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_power_dual_stream {
    pub dw: __le32,
    pub __packed: },
//
// Command REPLY_TX_POWER_DBM_CMD = 0x98
// struct iwlagn_tx_power_dbm_cmd
//
pub const IWLAGN_TX_POWER_AUTO: c_uint = 0x7f;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_tx_power_dbm_cmd {
    pub /: *mut *mut s8 global_lmt; /in half-dBm (e.g. 30 = 15 dBm),
    pub flags: u8,
    pub /: *mut *mut s8 srv_chan_lmt; /in half-dBm (e.g. 30 = 15 dBm),
    pub reserved: u8,
    pub __packed: },
//
// Command TX_ANT_CONFIGURATION_CMD = 0x98
// This command is used to configure valid Tx antenna.
// By default uCode concludes the valid antenna according to the radio flavor.
// This command enables the driver to override/modify this conclusion.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tx_ant_config_cmd {
    pub valid: __le32,
    pub __packed: },
//
// (0a)
// Alive and Error Commands & Responses:
//

//
// REPLY_ALIVE = 0x1 (response only, not a command)
//
// uCode issues this "alive" notification once the runtime image is ready
// to receive commands from the driver.  This is the *second* "alive"
// notification that the driver will receive after rebooting uCode;
// this "alive" is indicated by subtype field != 9.
//
// See comments documenting "BSM" (bootstrap state machine).
//
// This response includes two pointers to structures within the device's
// data SRAM (access via HBUS_TARG_MEM_* regs) that are useful for debugging:
//
// 1)  log_event_table_ptr indicates base of the event log.  This traces
// a 256-entry history of uCode execution within a circular buffer.
// Its header format is:
//
// __le32 log_size;     log capacity (in number of entries)
// __le32 type;         (1) timestamp with each entry, (0) no timestamp
// __le32 wraps;        # times uCode has wrapped to top of circular buffer
// __le32 write_index;  next circular buffer entry that uCode would fill
//
// The header is followed by the circular buffer of log entries.  Entries
// with timestamps have the following format:
//
// __le32 event_id;     range 0 - 1500
// __le32 timestamp;    low 32 bits of TSF (of network, if associated)
// __le32 data;         event_id-specific data value
//
// Entries without timestamps contain only event_id and data.
//
// 2)  error_event_table_ptr indicates base of the error log.  This contains
// information about any uCode error that occurs.  For agn, the format
// of the error log is defined by struct iwl_error_event_table.
//
// The Linux driver can print both logs to the system log when a uCode error
// occurs.
//
// Note: This structure is read from the device with IO accesses,
// and the reading already does the endian conversion. As it is
// read with u32-sized accesses, any members with a different size
// need to be ordered correctly though!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_error_event_table {
    pub /: *mut *mut u32 valid; / (nonzero) valid, (0) log is empty,
    pub /: *mut *mut u32 error_id; / type of error,
    pub /: *mut *mut u32 pc; / program counter,
    pub /: *mut *mut u32 blink1; / branch link,
    pub /: *mut *mut u32 blink2; / branch link,
    pub /: *mut *mut u32 ilink1; / interrupt link,
    pub /: *mut *mut u32 ilink2; / interrupt link,
    pub /: *mut *mut u32 data1; / error-specific data,
    pub /: *mut *mut u32 data2; / error-specific data,
    pub /: *mut *mut u32 line; / source code line of error,
    pub /: *mut *mut u32 bcon_time; / beacon timer,
    pub /: *mut *mut u32 tsf_low; / network timestamp function timer,
    pub /: *mut *mut u32 tsf_hi; / network timestamp function timer,
    pub /: *mut *mut u32 gp1; / GP1 timer register,
    pub /: *mut *mut u32 gp2; / GP2 timer register,
    pub /: *mut *mut u32 gp3; / GP3 timer register,
    pub /: *mut *mut u32 ucode_ver; / uCode version,
    pub /: *mut *mut u32 hw_ver; / HW Silicon version,
    pub /: *mut *mut u32 brd_ver; / HW board version,
    pub /: *mut *mut u32 log_pc; / log program counter,
    pub /: *mut *mut u32 frame_ptr; / frame pointer,
    pub /: *mut *mut u32 stack_ptr; / stack pointer,
    pub /: *mut *mut u32 hcmd; / last host command header,
    pub LMPM_NIC_ISR0:: *mut *mut u32 isr0; / isr status register,
// rxtx_flag
    pub LMPM_NIC_ISR1:: *mut *mut u32 isr1; / isr status register,
// host_flag
    pub LMPM_NIC_ISR2:: *mut *mut u32 isr2; / isr status register,
// enc_flag
    pub LMPM_NIC_ISR3:: *mut *mut u32 isr3; / isr status register,
// time_flag
    pub LMPM_NIC_ISR4:: *mut *mut u32 isr4; / isr status register,
// wico interrupt
    pub /: *mut *mut u32 isr_pref; / isr status register LMPM_NIC_PREF_STAT,
    pub /: *mut *mut u32 wait_event; / wait event() caller address,
    pub /: *mut *mut u32 l2p_control; / L2pControlField,
    pub /: *mut *mut u32 l2p_duration; / L2pDurationField,
    pub /: *mut *mut u32 l2p_mhvalid; / L2pMhValidBits,
    pub /: *mut *mut u32 l2p_addr_match; / L2pAddrMatchStat,
    pub on: *mut *mut u32 lmpm_pmg_sel; / indicate which clocks are turned,
// (LMPM_PMG_SEL)
    pub the: *mut *mut u32 u_timestamp; / indicate when the date and time of,
// compilation
    pub /: *mut *mut u32 flow_handler; / FH read/write pointers, RX credit,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_alive_resp {
    pub ucode_minor: u8,
    pub ucode_major: u8,
    pub reserved1: __le16,
    pub sw_rev: [u8; 8],
    pub ver_type: u8,
    pub /: *mut *mut u8 ver_subtype; / not "9" for runtime alive,
    pub reserved2: __le16,
    pub /: *mut *mut __le32 log_event_table_ptr; / SRAM address for event log,
    pub /: *mut *mut __le32 error_event_table_ptr; / SRAM address for error log,
    pub timestamp: __le32,
    pub is_valid: __le32,
    pub __packed: },
//
// REPLY_ERROR = 0x2 (response only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_error_resp {
    pub error_type: __le32,
    pub cmd_id: u8,
    pub reserved1: u8,
    pub bad_cmd_seq_num: __le16,
    pub error_info: __le32,
    pub timestamp: __le64,
    pub __packed: },
//
// (1)
// RXON Commands & Responses:
//
// Rx config defines & structure
//
// rx_config device types
}

// rx_config flags
// band & modulation selection

// auto detection enable

// TGg protection when tx

// cck short slot & preamble

// antenna selection

// radar detection enable

// rx response to host with 8-byte TSF
// (according to ON_AIR deassertion)

// HT flags

// channel mode

// CTS to self (if spec allows) flag

// rx_config filter flags
// accept all data frames

// pass control & management to host

// accept multi-cast

// don't decrypt uni-cast frames

// don't decrypt multi-cast frames

// STA is associated

// transfer to host non bssid beacons in associated state

//
// REPLY_RXON = 0x10 (command, has simple generic response)
//
// RXON tunes the radio tuner to a service channel, and sets up a number
// of parameters that are used primarily for Rx, but also for Tx operations.
//
// NOTE:  When tuning to a new channel, driver must set the
// RXON_FILTER_ASSOC_MSK to 0.  This will clear station-dependent
// info within the device, including the station tables, tx retry
// rate tables, and txpower tables.  Driver must build a new station
// table and txpower table before transmitting anything on the RXON
// channel.
//
// NOTE:  All RXONs wipe clean the internal txpower table.  Driver must
// issue a new REPLY_TX_PWR_TABLE_CMD after each REPLY_RXON (0x10),
// regardless of whether RXON_FILTER_ASSOC_MSK is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rxon_cmd {
    pub node_addr: [u8; 6],
    pub reserved1: __le16,
    pub bssid_addr: [u8; 6],
    pub reserved2: __le16,
    pub wlap_bssid_addr: [u8; 6],
    pub reserved3: __le16,
    pub dev_type: u8,
    pub air_propagation: u8,
    pub rx_chain: __le16,
    pub ofdm_basic_rates: u8,
    pub cck_basic_rates: u8,
    pub assoc_id: __le16,
    pub flags: __le32,
    pub filter_flags: __le32,
    pub channel: __le16,
    pub ofdm_ht_single_stream_basic_rates: u8,
    pub ofdm_ht_dual_stream_basic_rates: u8,
    pub ofdm_ht_triple_stream_basic_rates: u8,
    pub reserved5: u8,
    pub acquisition_data: __le16,
    pub reserved6: __le16,
    pub __packed: },
//
// REPLY_RXON_ASSOC = 0x11 (command, has simple generic response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rxon_assoc_cmd {
    pub flags: __le32,
    pub filter_flags: __le32,
    pub ofdm_basic_rates: u8,
    pub cck_basic_rates: u8,
    pub reserved1: __le16,
    pub ofdm_ht_single_stream_basic_rates: u8,
    pub ofdm_ht_dual_stream_basic_rates: u8,
    pub ofdm_ht_triple_stream_basic_rates: u8,
    pub reserved2: u8,
    pub rx_chain_select_flags: __le16,
    pub acquisition_data: __le16,
    pub reserved3: __le32,
    pub __packed: },
pub const IWL_CONN_MAX_LISTEN_INTERVAL: c_int = 10;

//
// REPLY_RXON_TIMING = 0x14 (command, has simple generic response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rxon_time_cmd {
    pub timestamp: __le64,
    pub beacon_interval: __le16,
    pub atim_window: __le16,
    pub beacon_init_val: __le32,
    pub listen_interval: __le16,
    pub dtim_period: u8,
    pub delta_cp_bss_tbtts: u8,
    pub __packed: },
//
// REPLY_CHANNEL_SWITCH = 0x72 (command, has simple generic response)
//
// struct iwl5000_channel_switch_cmd - channel switch command (5000 series)
// @band: 0- 5.2GHz, 1- 2.4GHz
// @expect_beacon: 0- resume transmits after channel switch
// 1- wait for beacon to resume transmits
// @channel: new channel number
// @rxon_flags: Rx on flags
// @rxon_filter_flags: filtering parameters
// @switch_time: switch time in extended beacon format
// @reserved: reserved bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl5000_channel_switch_cmd {
    pub band: u8,
    pub expect_beacon: u8,
    pub channel: __le16,
    pub rxon_flags: __le32,
    pub rxon_filter_flags: __le32,
    pub switch_time: __le32,
    pub IWL_PWR_CCK_ENTRIES]: __le32 reserved[2][IWL_PWR_NUM_HT_OFDM_ENTRIES +,
    pub __packed: },
//
// struct iwl6000_channel_switch_cmd - channel switch command (6000 series)
// @band: 0- 5.2GHz, 1- 2.4GHz
// @expect_beacon: 0- resume transmits after channel switch
// 1- wait for beacon to resume transmits
// @channel: new channel number
// @rxon_flags: Rx on flags
// @rxon_filter_flags: filtering parameters
// @switch_time: switch time in extended beacon format
// @reserved: reserved bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl6000_channel_switch_cmd {
    pub band: u8,
    pub expect_beacon: u8,
    pub channel: __le16,
    pub rxon_flags: __le32,
    pub rxon_filter_flags: __le32,
    pub switch_time: __le32,
    pub IWL_PWR_CCK_ENTRIES]: __le32 reserved[3][IWL_PWR_NUM_HT_OFDM_ENTRIES +,
    pub __packed: },
//
// CHANNEL_SWITCH_NOTIFICATION = 0x73 (notification only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_csa_notification {
    pub band: __le16,
    pub channel: __le16,
    pub /: *mut *mut __le32 status; / 0 - OK, 1 - fail,
    pub __packed: },
//
// (2)
// Quality-of-Service (QOS) Commands & Responses:
//
// struct iwl_ac_qos -- QOS timing params for REPLY_QOS_PARAM
// One for each of 4 EDCA access categories in struct iwl_qosparam_cmd
//
// @cw_min: Contention window, start value in numbers of slots.
// Should be a power-of-2, minus 1.  Device's default is 0x0f.
// @cw_max: Contention window, max value in numbers of slots.
// Should be a power-of-2, minus 1.  Device's default is 0x3f.
// @aifsn:  Number of slots in Arbitration Interframe Space (before
// performing random backoff timing prior to Tx).  Device default 1.
// @edca_txop:  Length of Tx opportunity, in uSecs.  Device default is 0.
// @reserved1: reserved for alignment
//
// Device will automatically increase contention window by (2*CW) + 1 for each
// transmission retry.  Device uses cw_max as a bit mask, ANDed with new CW
// value, to cap the CW value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ac_qos {
    pub cw_min: __le16,
    pub cw_max: __le16,
    pub aifsn: u8,
    pub reserved1: u8,
    pub edca_txop: __le16,
    pub __packed: },
// QoS flags defines

// Number of Access Categories (AC) (EDCA), queues 0..3
pub const AC_NUM: c_int = 4;
//
// REPLY_QOS_PARAM = 0x13 (command, has simple generic response)
//
// This command sets up timings for each of the 4 prioritized EDCA Tx FIFOs
// 0: Background, 1: Best Effort, 2: Video, 3: Voice.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_qosparam_cmd {
    pub qos_flags: __le32,
    pub ac: [iwl_ac_qos; AC_NUM],
    pub __packed: },
//
// (3)
// Add/Modify Stations Commands & Responses:
//
// Multi station support
//
// Special, dedicated locations within device's station table
pub const IWL_AP_ID: c_int = 0;
pub const IWL_AP_ID_PAN: c_int = 1;
pub const IWL_STA_ID: c_int = 2;
pub const IWLAGN_PAN_BCAST_ID: c_int = 14;
pub const IWLAGN_BROADCAST_ID: c_int = 15;
pub const IWLAGN_STATION_COUNT: c_int = 16;

// Use in mode field.  1: modify existing entry, 0: add new station entry
pub const STA_CONTROL_MODIFY_MSK: c_uint = 0x01;
// key flags __le16

pub const STA_KEY_FLG_KEYID_POS: c_int = 8;

// wep key is either from global key (0) or from station info array (1)

// wep key in STA: 5-bytes (0) or 13-bytes (1)

pub const STA_KEY_MAX_NUM: c_int = 8;
pub const STA_KEY_MAX_NUM_PAN: c_int = 16;
// must not match WEP_INVALID_OFFSET
pub const IWLAGN_HW_KEY_DEFAULT: c_uint = 0xfe;
// Flags indicate whether to modify vs. don't change various station params
pub const STA_MODIFY_KEY_MASK: c_uint = 0x01;
pub const STA_MODIFY_TID_DISABLE_TX: c_uint = 0x02;
pub const STA_MODIFY_TX_RATE_MSK: c_uint = 0x04;
pub const STA_MODIFY_ADDBA_TID_MSK: c_uint = 0x08;
pub const STA_MODIFY_DELBA_TID_MSK: c_uint = 0x10;
pub const STA_MODIFY_SLEEP_TX_COUNT_MSK: c_uint = 0x20;
// agn
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_keyinfo {
    pub key_flags: __le16,
    pub /: *mut *mut u8 tkip_rx_tsc_byte2; / TSC[2] for key mix ph1 detection,
    pub reserved1: u8,
    pub /: *mut *mut __le16 tkip_rx_ttak[5]; / 10-byte unicast TKIP TTAK,
    pub key_offset: u8,
    pub reserved2: u8,
    pub /: *mut *mut u8 key[16]; / 16-byte unicast decryption key,
    pub tx_secur_seq_cnt: __le64,
    pub hw_tkip_mic_rx_key: __le64,
    pub hw_tkip_mic_tx_key: __le64,
    pub __packed: },
//
// struct sta_id_modify - station modify command
// @addr: station's MAC address
// @reserved1: reserved for alignment
// @sta_id: index of station in uCode's station table
// @modify_mask: STA_MODIFY_*, 1: modify, 0: don't change
// @reserved2: reserved for alignment
//
// Driver selects unused table index when adding new station,
// or the index to a pre-existing station entry when modifying that station.
// Some indexes have special purposes (IWL_AP_ID, index 0, is for AP).
//
// modify_mask flags select which parameters to modify vs. leave alone.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta_id_modify {
    pub addr: [u8; ETH_ALEN],
    pub reserved1: __le16,
    pub sta_id: u8,
    pub modify_mask: u8,
    pub reserved2: __le16,
    pub __packed: },
//
// REPLY_ADD_STA = 0x18 (command)
//
// The device contains an internal table of per-station information,
// with info on security keys, aggregation parameters, and Tx rates for
// initial Tx attempt and any retries (agn devices uses
// REPLY_TX_LINK_QUALITY_CMD,
//
// REPLY_ADD_STA sets up the table entry for one station, either creating
// a new entry, or modifying a pre-existing one.
//
// NOTE:  RXON command (without "associated" bit set) wipes the station table
// clean.  Moving into RF_KILL state does this also.  Driver must set up
// new station table before transmitting anything on the RXON channel
// (except active scans or active measurements; those commands carry
// their own txpower/rate setup data).
//
// When getting started on a new channel, driver must set up the
// IWL_BROADCAST_ID entry (last entry in the table).  For a client
// station in a BSS, once an AP is selected, driver sets up the AP STA
// in the IWL_AP_ID entry (1st entry in the table).  BROADCAST and AP
// are all that are needed for a BSS client station.  If the device is
// used as AP, or in an IBSS network, driver must set up station table
// entries for all STAs in network, starting with index IWL_STA_ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_addsta_cmd {
    pub /: *mut *mut u8 mode; / 1: modify existing, 0: add new station,
    pub reserved: [u8; 3],
    pub sta: sta_id_modify,
    pub key: iwl_keyinfo,
    pub /: *mut *mut *mut __le32 station_flags; / STA_FLG_,
    pub /: *mut *mut *mut __le32 station_flags_msk; / STA_FLG_,
// bit field to disable (1) or enable (0) Tx for Traffic ID (TID)
// corresponding to bit (e.g. bit 5 controls TID 5).
// Set modify_mask bit STA_MODIFY_TID_DISABLE_TX to use this field.
    pub tid_disable_tx: __le16,
    pub legacy_reserved: __le16,
// TID for which to add block-ack support.
// Set modify_mask bit STA_MODIFY_ADDBA_TID_MSK to use this field.
    pub add_immediate_ba_tid: u8,
// TID for which to remove block-ack support.
// Set modify_mask bit STA_MODIFY_DELBA_TID_MSK to use this field.
    pub remove_immediate_ba_tid: u8,
// Starting Sequence Number for added block-ack support.
// Set modify_mask bit STA_MODIFY_ADDBA_TID_MSK to use this field.
    pub add_immediate_ba_ssn: __le16,
//
// Number of packets OK to transmit to station even though
// it is asleep -- used to synchronise PS-poll and u-APSD
// responses while ucode keeps track of STA sleep state.
//
    pub sleep_tx_count: __le16,
    pub reserved2: __le16,
    pub __packed: },
pub const ADD_STA_SUCCESS_MSK: c_uint = 0x1;
pub const ADD_STA_NO_ROOM_IN_TABLE: c_uint = 0x2;
pub const ADD_STA_NO_BLOCK_ACK_RESOURCE: c_uint = 0x4;
pub const ADD_STA_MODIFY_NON_EXIST_STA: c_uint = 0x8;
//
// REPLY_ADD_STA = 0x18 (response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_add_sta_resp {
    pub /: *mut *mut *mut u8 status; / ADD_STA_,
    pub __packed: },
pub const REM_STA_SUCCESS_MSK: c_uint = 0x1;
//
// REPLY_REM_STA = 0x19 (response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rem_sta_resp {
    pub status: u8,
    pub __packed: },
//
// REPLY_REM_STA = 0x19 (command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rem_sta_cmd {
    pub /: *mut *mut u8 num_sta; / number of removed stations,
    pub reserved: [u8; 3],
    pub /: *mut *mut u8 addr[ETH_ALEN]; / MAC addr of the first station,
    pub reserved2: [u8; 2],
    pub __packed: },
// WiFi queues mask

// PAN queues mask

pub const IWL_AGG_TX_QUEUE_MSK: c_uint = 0xffc00;

//
// REPLY_TXFIFO_FLUSH = 0x1e(command and response)
//
// When using full FIFO flush this command checks the scheduler HW block WR/RD
// pointers to check if all the frames were transferred by DMA into the
// relevant TX FIFO queue. Only when the DMA is finished and the queue is
// empty the command can finish.
// This command is used to flush the TXFIFO from transmit commands, it may
// operate on single or multiple queues, the command queue can't be flushed by
// this command. The command response is returned when all the queue flush
// operations are done. Each TX command flushed return response with the FLUSH
// status set in the TX response status. When FIFO flush operation is used,
// the flush operation ends when both the scheduler DMA done and TXFIFO empty
// are set.
//
// @queue_control: bit mask for which queues to flush
// @flush_control: flush controls
// 0: Dump single MSDU
// 1: Dump multiple MSDU according to PS, INVALID STA, TTL, TID disable.
// 2: Dump all FIFO
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_txfifo_flush_cmd_v3 {
    pub queue_control: __le32,
    pub flush_control: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_txfifo_flush_cmd_v2 {
    pub queue_control: __le16,
    pub flush_control: __le16,
    pub __packed: },
//
// REPLY_WEP_KEY = 0x20
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wep_key {
    pub key_index: u8,
    pub key_offset: u8,
    pub reserved1: [u8; 2],
    pub key_size: u8,
    pub reserved2: [u8; 3],
    pub key: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wep_cmd {
    pub num_keys: u8,
    pub global_key_type: u8,
    pub flags: u8,
    pub reserved: u8,
    pub key: [iwl_wep_key; ],
    pub __packed: },
pub const WEP_KEY_WEP_TYPE: c_int = 1;
pub const WEP_KEYS_MAX: c_int = 4;
pub const WEP_INVALID_OFFSET: c_uint = 0xff;
pub const WEP_KEY_LEN_64: c_int = 5;
pub const WEP_KEY_LEN_128: c_int = 13;
//
// (4)
// Rx Responses:
//

pub const RX_RES_PHY_FLAGS_ANTENNA_MSK: c_uint = 0x70;
pub const RX_RES_PHY_FLAGS_ANTENNA_POS: c_int = 4;

pub const IWLAGN_RX_RES_PHY_CNT: c_int = 8;
pub const IWLAGN_RX_RES_AGC_IDX: c_int = 1;
pub const IWLAGN_RX_RES_RSSI_AB_IDX: c_int = 2;
pub const IWLAGN_RX_RES_RSSI_C_IDX: c_int = 3;
pub const IWLAGN_OFDM_AGC_MSK: c_uint = 0xfe00;
pub const IWLAGN_OFDM_AGC_BIT_POS: c_int = 9;
pub const IWLAGN_OFDM_RSSI_INBAND_A_BITMSK: c_uint = 0x00ff;
pub const IWLAGN_OFDM_RSSI_ALLBAND_A_BITMSK: c_uint = 0xff00;
pub const IWLAGN_OFDM_RSSI_A_BIT_POS: c_int = 0;
pub const IWLAGN_OFDM_RSSI_INBAND_B_BITMSK: c_uint = 0xff0000;
pub const IWLAGN_OFDM_RSSI_ALLBAND_B_BITMSK: c_uint = 0xff000000;
pub const IWLAGN_OFDM_RSSI_B_BIT_POS: c_int = 16;
pub const IWLAGN_OFDM_RSSI_INBAND_C_BITMSK: c_uint = 0x00ff;
pub const IWLAGN_OFDM_RSSI_ALLBAND_C_BITMSK: c_uint = 0xff00;
pub const IWLAGN_OFDM_RSSI_C_BIT_POS: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_non_cfg_phy {
    pub /: *mut *mut __le32 non_cfg_phy[IWLAGN_RX_RES_PHY_CNT]; / up to 8 phy entries,
    pub __packed: },
//
// REPLY_RX = 0xc3 (response only, not a command)
// Used only for legacy (non 11n) frames.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_phy_res {
    pub /: *mut *mut u8 non_cfg_phy_cnt; / non configurable DSP phy data byte count,
    pub /: *mut *mut u8 cfg_phy_cnt; / configurable DSP phy data byte count,
    pub /: *mut *mut u8 stat_id; / configurable DSP phy data set ID,
    pub reserved1: u8,
    pub /: *mut *mut __le64 timestamp; / TSF at on air rise,
    pub /: *mut *mut __le32 beacon_time_stamp; / beacon at on-air rise,
    pub /: *mut *mut __le16 phy_flags; / general phy flags: band, modulation, ...,
    pub /: *mut *mut __le16 channel; / channel number,
    pub /: *mut *mut u8 non_cfg_phy_buf[32]; / for various implementations of non_cfg_phy,
    pub /: *mut *mut *mut __le32 rate_n_flags; / RATE_MCS_,
    pub /: *mut *mut __le16 byte_count; / frame's byte-count,
    pub /: *mut *mut __le16 frame_time; / frame's time on the air,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_rx_mpdu_res_start {
    pub byte_count: __le16,
    pub reserved: __le16,
    pub __packed: },
//
// (5)
// Tx Commands & Responses:
//
// Driver must place each REPLY_TX command into one of the prioritized Tx
// queues in host DRAM, shared between driver and device (see comments for
// SCD registers and Tx/Rx Queues).  When the device's Tx scheduler and uCode
// are preparing to transmit, the device pulls the Tx command over the PCI
// bus via one of the device's Tx DMA channels, to fill an internal FIFO
// from which data will be transmitted.
//
// uCode handles all timing and protocol related to control frames
// (RTS/CTS/ACK), based on flags in the Tx command.  uCode and Tx scheduler
// handle reception of block-acks; uCode updates the host driver via
// REPLY_COMPRESSED_BA.
//
// uCode handles retrying Tx when an ACK is expected but not received.
// This includes trying lower data rates than the one requested in the Tx
// command, as set up by the REPLY_TX_LINK_QUALITY_CMD (agn).
//
// Driver sets up transmit power for various rates via REPLY_TX_PWR_TABLE_CMD.
// This command must be executed after every RXON command, before Tx can occur.
//
// REPLY_TX Tx flags field
//
// 1: Use RTS/CTS protocol or CTS-to-self if spec allows it
// before this frame. if CTS-to-self required check
// RXON_FLG_SELF_CTS_EN status.
//

// 1: Expect ACK from receiving station
// 0: Don't expect ACK (MAC header's duration field s/b 0)
// Set this for unicast frames, but not broadcast/multicast.

// For agn devices:
// 1: Use rate scale table (see REPLY_TX_LINK_QUALITY_CMD).
// Tx command's initial_rate_index indicates first rate to try;
// uCode walks through table for additional Tx attempts.
// 0: Use Tx rate/MCS from Tx command's rate_n_flags field.
// This rate will be used for all Tx attempts; it will not be scaled.

// 1: Expect immediate block-ack.
// Set when Txing a block-ack request frame.  Also set TX_CMD_FLG_ACK_MSK.

// Tx antenna selection field; reserved (0) for agn devices.

// 1: Ignore Bluetooth priority for this frame.
// 0: Delay Tx until Bluetooth device is done (normal usage).

// 1: uCode overrides sequence control field in MAC header.
// 0: Driver provides sequence control field in MAC header.
// Set this for management frames, non-QOS data frames, non-unicast frames,
// and also in Tx command embedded in REPLY_SCAN_CMD for active scans.

// 1: This frame is non-last MPDU; more fragments are coming.
// 0: Last fragment, or not using fragmentation.

// 1: uCode calculates and inserts Timestamp Function (TSF) in outgoing frame.
// 0: No TSF required in outgoing frame.
// Set this for transmitting beacons and probe responses.

// 1: Driver inserted 2 bytes pad after the MAC header, for (required) dword
// alignment of frame's payload data field.
// 0: No pad
// Set this for MAC headers with 26 or 30 bytes, i.e. those with QOS or ADDR4
// field (but not both).  Driver must align frame data (i.e. data following
// MAC header) to DWORD boundary.

// accelerate aggregation support
// 0 - no CCMP encryption; 1 - CCMP encryption

// HCCA-AP - disable duration overwriting.

//
// TX command security control
//
pub const TX_CMD_SEC_WEP: c_uint = 0x01;
pub const TX_CMD_SEC_CCM: c_uint = 0x02;
pub const TX_CMD_SEC_TKIP: c_uint = 0x03;
pub const TX_CMD_SEC_MSK: c_uint = 0x03;
pub const TX_CMD_SEC_SHIFT: c_int = 6;
pub const TX_CMD_SEC_KEY128: c_uint = 0x08;
//
// REPLY_TX = 0x1c (command)
//
// Used for managing Tx retries when expecting block-acks.
// Driver should set these fields to 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_dram_scratch {
    pub /: *mut *mut u8 try_cnt; / Tx attempts,
    pub /: *mut *mut u8 bt_kill_cnt; / Tx attempts blocked by Bluetooth device,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tx_cmd {
// New members MUST be added within the __struct_group() macro below.
//
// MPDU byte count:
// MAC header (24/26/30/32 bytes) + 2 bytes pad if 26/30 header size,
// + 8 byte IV for CCM or TKIP (not used for WEP)
// + Data payload
// + 8-byte MIC (not used for CCM/WEP)
// NOTE:  Does not include Tx command bytes, post-MAC pad bytes,
// MIC (CCM) 8 bytes, ICV (WEP/TKIP/CKIP) 4 bytes, CRC 4 bytes.i
// Range: 14-2342 bytes.
//
    pub len: __le16,
//
// MPDU or MSDU byte count for next frame.
// Used for fragmentation and bursting, but not 11n aggregation.
// Same as "len", but for next frame.  Set to 0 if not applicable.
//
    pub next_frame_len: __le16,
    pub /: *mut *mut *mut __le32 tx_flags; / TX_CMD_FLG_,
// uCode may modify this field of the Tx command (in host DRAM!).
// Driver must also set dram_lsb_ptr and dram_msb_ptr in this cmd.
    pub scratch: iwl_dram_scratch,
// Rate for *all* Tx attempts, if TX_CMD_FLG_STA_RATE_MSK is cleared.
    pub /: *mut *mut *mut __le32 rate_n_flags; / RATE_MCS_,
// Index of destination station in uCode's station table
    pub sta_id: u8,
// Type of security encryption:  CCM or TKIP
    pub /: *mut *mut *mut u8 sec_ctl; / TX_CMD_SEC_,
//
// Index into rate table (see REPLY_TX_LINK_QUALITY_CMD) for initial
// Tx attempt, if TX_CMD_FLG_STA_RATE_MSK is set.  Normally "0" for
// data frames, this field may be used to selectively reduce initial
// rate (via non-0 value) for special frames (e.g. management), while
// still supporting rate scaling for all frames.
//
    pub initial_rate_index: u8,
    pub reserved: u8,
    pub key: [u8; 16],
    pub next_frame_flags: __le16,
    pub reserved2: __le16,
    pub life_time: __le32,
    pub attempt: __le32,
    pub stop_time: },
// Host DRAM physical address pointer to "scratch" in this command.
// Must be dword aligned.  "0" in dram_lsb_ptr disables usage.
    pub dram_lsb_ptr: __le32,
    pub dram_msb_ptr: u8,
    pub /: *mut *mut u8 rts_retry_limit; /byte 50,
    pub /: *mut *mut u8 data_retry_limit; /byte 51,
    pub tid_tspec: u8,
    pub pm_frame_timeout: __le16,
    pub attempt_duration: __le16,
    pub timeout: },
//
// Duration of EDCA burst Tx Opportunity, in 32-usec units.
// Set this if txop time is not specified by HCCA protocol (e.g. by AP).
//
    pub driver_txop: __le16,
//
// MAC header goes here, followed by 2 bytes padding if MAC header
// length is 26 or 30 bytes, followed by payload data
//
    pub hdr: [ieee80211_hdr; ],
    pub __packed: },
    pub __struct_group()"): "struct member likely outside of,
//
// TX command response is sent after *agn* transmission attempts.
//
// both postpone and abort status are expected behavior from uCode. there is
// no special operation required from driver; except for RFKILL_FLUSH,
// which required tx flush host command to flush all the tx frames in queues
//
// postpone TX
// abort TX
}

pub const TX_PACKET_MODE_REGULAR: c_uint = 0x0000;
pub const TX_PACKET_MODE_BURST_SEQ: c_uint = 0x0100;
pub const TX_PACKET_MODE_BURST_FIRST: c_uint = 0x0200;
//
// TX aggregation status
//
pub const AGG_TX_STATUS_MSK: c_uint = 0x00000fff	/* bits 0:11 */;
pub const AGG_TX_TRY_MSK: c_uint = 0x0000f000	/* bits 12:15 */;
pub const AGG_TX_TRY_POS: c_int = 12;

// # tx attempts for first frame in aggregation
pub const AGG_TX_STATE_TRY_CNT_POS: c_int = 12;
pub const AGG_TX_STATE_TRY_CNT_MSK: c_uint = 0xf000;
// Command ID and sequence number of Tx command for this frame
pub const AGG_TX_STATE_SEQ_NUM_POS: c_int = 16;
pub const AGG_TX_STATE_SEQ_NUM_MSK: c_uint = 0xffff0000;
//
// REPLY_TX = 0x1c (response)
//
// This response may be in one of two slightly different formats, indicated
// by the frame_count field:
//
// 1)  No aggregation (frame_count == 1).  This reports Tx results for
// a single frame.  Multiple attempts, at various bit rates, may have
// been made for this frame.
//
// 2)  Aggregation (frame_count > 1).  This reports Tx results for
// 2 or more frames that used block-acknowledge.  All frames were
// transmitted at same rate.  Rate scaling may have been used if first
// frame in this new agg block failed in previous agg block(s).
//
// Note that, for aggregation, ACK (block-ack) status is not delivered here;
// block-ack has not been received by the time the agn device records
// this status.
// This status relates to reasons the tx might have been blocked or aborted
// within the sending station (this agn device), rather than whether it was
// received successfully by the destination station.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct agg_tx_status {
    pub status: __le16,
    pub sequence: __le16,
    pub __packed: },
// refer to ra_tid
pub const IWLAGN_TX_RES_TID_POS: c_int = 0;
pub const IWLAGN_TX_RES_TID_MSK: c_uint = 0x0f;
pub const IWLAGN_TX_RES_RA_POS: c_int = 4;
pub const IWLAGN_TX_RES_RA_MSK: c_uint = 0xf0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_tx_resp {
    pub /: *mut *mut u8 frame_count; / 1 no aggregation, >1 aggregation,
    pub /: *mut *mut u8 bt_kill_count; / # blocked by bluetooth (unused for agg),
    pub /: *mut *mut u8 failure_rts; / # failures due to unsuccessful RTS,
    pub /: *mut *mut u8 failure_frame; / # failures due to no ACK (unused for agg),
// For non-agg:  Rate at which frame was successful.
// For agg:  Rate at which all frames were transmitted.
    pub /: *mut *mut *mut __le32 rate_n_flags; / RATE_MCS_,
// For non-agg:  RTS + CTS + frame tx attempts time + ACK.
// For agg:  RTS + CTS + aggregation tx time + block-ack time.
    pub /: *mut *mut __le16 wireless_media_time; / uSecs,
    pub /: *mut *mut u8 pa_status; / RF power amplifier measurement (not used),
    pub pa_integ_res_a: [u8; 3],
    pub pa_integ_res_b: [u8; 3],
    pub pa_integ_res_C: [u8; 3],
    pub tfd_info: __le32,
    pub seq_ctl: __le16,
    pub byte_cnt: __le16,
    pub tlc_info: u8,
    pub /: *mut *mut u8 ra_tid; / tid (0:3), sta_id (4:7),
    pub frame_ctrl: __le16,
//
// For non-agg:  frame status TX_STATUS_
// For agg:  status of 1st frame, AGG_TX_STATE_*; other frame status
// fields follow this one, up to frame_count.
// Bit fields:
// 11- 0:  AGG_TX_STATE_* status code
// 15-12:  Retry count for 1st frame in aggregation (retries
// occur if tx failed for this frame when it was a
// member of a previous aggregation block).  If rate
// scaling is used, retry count indicates the rate
// table entry used for all frames in the new agg.
// 31-16:  Sequence # for this frame's Tx cmd (not SSN!)
//
    pub -: *mut *mut agg_tx_status status; / TX status (in aggregation,
// status of 1st frame)
    pub __packed: },
//
// REPLY_COMPRESSED_BA = 0xc5 (response only, not a command)
//
// Reports Block-Acknowledge from recipient station
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_compressed_ba_resp {
    pub sta_addr_lo32: __le32,
    pub sta_addr_hi16: __le16,
    pub reserved: __le16,
// Index of recipient (BA-sending) station in uCode's station table
    pub sta_id: u8,
    pub tid: u8,
    pub seq_ctl: __le16,
    pub bitmap: __le64,
    pub scd_flow: __le16,
    pub scd_ssn: __le16,
    pub /: *mut *mut u8 txed; / number of frames sent,
    pub /: *mut *mut u8 txed_2_done; / number of frames acked,
    pub reserved1: __le16,
    pub __packed: },
//
// REPLY_TX_PWR_TABLE_CMD = 0x97 (command, has simple generic response)
//
// RS_NEW_API: only TLC_RTS remains and moved to bit 0

// # of EDCA prioritized tx fifos

// # entries in rate scale table to support Tx retries
pub const LINK_QUAL_MAX_RETRY_NUM: c_int = 16;
// Tx antenna selection values

//
// struct iwl_link_qual_general_params
//
// Used in REPLY_TX_LINK_QUALITY_CMD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_link_qual_general_params {
    pub flags: u8,
// No entries at or above this (driver chosen) index contain MIMO
    pub mimo_delimiter: u8,
// Best single antenna to use for single stream (legacy, SISO).
    pub /: *mut *mut *mut u8 single_stream_ant_msk; / LINK_QUAL_ANT_,
// Best antennas to use for MIMO
    pub /: *mut *mut *mut u8 dual_stream_ant_msk; / LINK_QUAL_ANT_,
//
// If driver needs to use different initial rates for different
// EDCA QOS access categories (as implemented by tx fifos 0-3),
// this table will set that up, by indicating the indexes in the
// rs_table[LINK_QUAL_MAX_RETRY_NUM] rate table at which to start.
// Otherwise, driver should set all entries to 0.
//
// Entry usage:
// 0 = Background, 1 = Best Effort (normal), 2 = Video, 3 = Voice
// TX FIFOs above 3 use same value (typically 0) as TX FIFO 3.
//
    pub start_rate_index: [u8; LINK_QUAL_AC_NUM],
    pub __packed: },

//
// struct iwl_link_qual_agg_params
//
// Used in REPLY_TX_LINK_QUALITY_CMD
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_link_qual_agg_params {
//
// Maximum number of uSec in aggregation.
// default set to 4000 (4 milliseconds) if not configured in .cfg
//
    pub agg_time_limit: __le16,
//
// Number of Tx retries allowed for a frame, before that frame will
// no longer be considered for the start of an aggregation sequence
// (scheduler will then try to tx it as single frame).
// Driver should set this to 3.
//
    pub agg_dis_start_th: u8,
//
// Maximum number of frames in aggregation.
// 0 = no limit (default).  1 = no aggregation.
// Other values = max # frames in aggregation.
//
    pub agg_frame_cnt_limit: u8,
    pub reserved: __le32,
    pub __packed: },
//
// REPLY_TX_LINK_QUALITY_CMD = 0x4e (command, has simple generic response)
//
// For agn devices
//
// Each station in the agn device's internal station table has its own table
// of 16
// Tx rates and modulation modes (e.g. legacy/SISO/MIMO) for retrying Tx when
// an ACK is not received.  This command replaces the entire table for
// one station.
//
// NOTE:  Station must already be in agn device's station table.
// Use REPLY_ADD_STA.
//
// The rate scaling procedures described below work well.  Of course, other
// procedures are possible, and may work better for particular environments.
//
// FILLING THE RATE TABLE
//
// Given a particular initial rate and mode, as determined by the rate
// scaling algorithm described below, the Linux driver uses the following
// formula to fill the rs_table[LINK_QUAL_MAX_RETRY_NUM] rate table in the
// Link Quality command:
//
// 1)  If using High-throughput (HT) (SISO or MIMO) initial rate:
// a) Use this same initial rate for first 3 entries.
// b) Find next lower available rate using same mode (SISO or MIMO),
// use for next 3 entries.  If no lower rate available, switch to
// legacy mode (no HT40 channel, no MIMO, no short guard interval).
// c) If using MIMO, set command's mimo_delimiter to number of entries
// using MIMO (3 or 6).
// d) After trying 2 HT rates, switch to legacy mode (no HT40 channel,
// no MIMO, no short guard interval), at the next lower bit rate
// (e.g. if second HT bit rate was 54, try 48 legacy), and follow
// legacy procedure for remaining table entries.
//
// 2)  If using legacy initial rate:
// a) Use the initial rate for only one entry.
// b) For each following entry, reduce the rate to next lower available
// rate, until reaching the lowest available rate.
// c) When reducing rate, also switch antenna selection.
// d) Once lowest available rate is reached, repeat this rate until
// rate table is filled (16 entries), switching antenna each entry.
//
// ACCUMULATING HISTORY
//
// The rate scaling algorithm for agn devices, as implemented in Linux driver,
// uses two sets of frame Tx success history:  One for the current/active
// modulation mode, and one for a speculative/search mode that is being
// attempted. If the speculative mode turns out to be more effective (i.e.
// actual transfer rate is better), then the driver continues to use the
// speculative mode as the new current active mode.
//
// Each history set contains, separately for each possible rate, data for a
// sliding window of the 62 most recent tx attempts at that rate.  The data
// includes a shifting bitmap of success(1)/failure(0), and sums of successful
// and attempted frames, from which the driver can additionally calculate a
// success ratio (success / attempted) and number of failures
// (attempted - success), and control the size of the window (attempted).
// The driver uses the bit map to remove successes from the success sum, as
// the oldest tx attempts fall out of the window.
//
// When the agn device makes multiple tx attempts for a given frame, each
// attempt might be at a different rate, and have different modulation
// characteristics (e.g. antenna, fat channel, short guard interval), as set
// up in the rate scaling table in the Link Quality command.  The driver must
// determine which rate table entry was used for each tx attempt, to determine
// which rate-specific history to update, and record only those attempts that
// match the modulation characteristics of the history set.
//
// When using block-ack (aggregation), all frames are transmitted at the same
// rate, since there is no per-attempt acknowledgment from the destination
// station.  The Tx response struct iwl_tx_resp indicates the Tx rate in
// rate_n_flags field.  After receiving a block-ack, the driver can update
// history for the entire block all at once.
//
// FINDING BEST STARTING RATE:
//
// When working with a selected initial modulation mode (see below), the
// driver attempts to find a best initial rate.  The initial rate is the
// first entry in the Link Quality command's rate table.
//
// 1)  Calculate actual throughput (success ratio * expected throughput, see
// table below) for current initial rate.  Do this only if enough frames
// have been attempted to make the value meaningful:  at least 6 failed
// tx attempts, or at least 8 successes.  If not enough, don't try rate
// scaling yet.
//
// 2)  Find available rates adjacent to current initial rate.  Available means:
// a)  supported by hardware &&
// b)  supported by association &&
// c)  within any constraints selected by user
//
// 3)  Gather measured throughputs for adjacent rates.  These might not have
// enough history to calculate a throughput.  That's okay, we might try
// using one of them anyway!
//
// 4)  Try decreasing rate if, for current rate:
// a)  success ratio is < 15% ||
// b)  lower adjacent rate has better measured throughput ||
// c)  higher adjacent rate has worse throughput, and lower is unmeasured
//
// As a sanity check, if decrease was determined above, leave rate
// unchanged if:
// a)  lower rate unavailable
// b)  success ratio at current rate > 85% (very good)
// c)  current measured throughput is better than expected throughput
// of lower rate (under perfect 100% tx conditions, see table below)
//
// 5)  Try increasing rate if, for current rate:
// a)  success ratio is < 15% ||
// b)  both adjacent rates' throughputs are unmeasured (try it!) ||
// b)  higher adjacent rate has better measured throughput ||
// c)  lower adjacent rate has worse throughput, and higher is unmeasured
//
// As a sanity check, if increase was determined above, leave rate
// unchanged if:
// a)  success ratio at current rate < 70%.  This is not particularly
// good performance; higher rate is sure to have poorer success.
//
// 6)  Re-evaluate the rate after each tx frame.  If working with block-
// acknowledge, history and statistics may be calculated for the entire
// block (including prior history that fits within the history windows),
// before re-evaluation.
//
// FINDING BEST STARTING MODULATION MODE:
//
// After working with a modulation mode for a "while" (and doing rate scaling),
// the driver searches for a new initial mode in an attempt to improve
// throughput.  The "while" is measured by numbers of attempted frames:
//
// For legacy mode, search for new mode after:
// 480 successful frames, or 160 failed frames
// For high-throughput modes (SISO or MIMO), search for new mode after:
// 4500 successful frames, or 400 failed frames
//
// Mode switch possibilities are (3 for each mode):
//
// For legacy:
// Change antenna, try SISO (if HT association), try MIMO (if HT association)
// For SISO:
// Change antenna, try MIMO, try shortened guard interval (SGI)
// For MIMO:
// Try SISO antenna A, SISO antenna B, try shortened guard interval (SGI)
//
// When trying a new mode, use the same bit rate as the old/current mode when
// trying antenna switches and shortened guard interval.  When switching to
// SISO from MIMO or legacy, or to MIMO from SISO or legacy, use a rate
// for which the expected throughput (under perfect conditions) is about the
// same or slightly better than the actual measured throughput delivered by
// the old/current mode.
//
// Actual throughput can be estimated by multiplying the expected throughput
// by the success ratio (successful / attempted tx frames).  Frame size is
// not considered in this calculation; it assumes that frame size will average
// out to be fairly consistent over several samples.  The following are
// metric values for expected throughput assuming 100% success ratio.
// Only G band has support for CCK rates:
//
// RATE:  1    2    5   11    6   9   12   18   24   36   48   54   60
//
// G:  7   13   35   58   40  57   72   98  121  154  177  186  186
// A:  0    0    0    0   40  57   72   98  121  154  177  186  186
// SISO 20MHz:  0    0    0    0   42  42   76  102  124  159  183  193  202
// SGI SISO 20MHz:  0    0    0    0   46  46   82  110  132  168  192  202  211
// MIMO 20MHz:  0    0    0    0   74  74  123  155  179  214  236  244  251
// SGI MIMO 20MHz:  0    0    0    0   81  81  131  164  188  222  243  251  257
// SISO 40MHz:  0    0    0    0   77  77  127  160  184  220  242  250  257
// SGI SISO 40MHz:  0    0    0    0   83  83  135  169  193  229  250  257  264
// MIMO 40MHz:  0    0    0    0  123 123  182  214  235  264  279  285  289
// SGI MIMO 40MHz:  0    0    0    0  131 131  191  222  242  270  284  289  293
//
// After the new mode has been tried for a short while (minimum of 6 failed
// frames or 8 successful frames), compare success ratio and actual throughput
// estimate of the new mode with the old.  If either is better with the new
// mode, continue to use the new mode.
//
// Continue comparing modes until all 3 possibilities have been tried.
// If moving from legacy to HT, try all 3 possibilities from the new HT
// mode.  After trying all 3, a best mode is found.  Continue to use this mode
// for the longer "while" described above (e.g. 480 successful frames for
// legacy), and then repeat the search process.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_link_quality_cmd {
// Index of destination/recipient station in uCode's station table
    pub sta_id: u8,
    pub reserved1: u8,
    pub /: *mut *mut __le16 control; / not used,
    pub general_params: iwl_link_qual_general_params,
    pub agg_params: iwl_link_qual_agg_params,
//
// Rate info; when using rate-scaling, Tx command's initial_rate_index
// specifies 1st Tx rate attempted, via index into this table.
// agn devices works its way through table when retrying Tx.
//
    pub /: *mut *mut *mut *mut __le32 rate_n_flags; / RATE_MCS_, IWL_RATE_,
    pub rs_table: [}; LINK_QUAL_MAX_RETRY_NUM],
    pub reserved2: __le32,
    pub __packed: },
//
// BT configuration enable flags:
// bit 0 - 1: BT channel announcement enabled
// 0: disable
// bit 1 - 1: priority of BT device enabled
// 0: disable
// bit 2 - 1: BT 2 wire support enabled
// 0: disable
//

pub const BT_DURATION_LIMIT_DEF: c_int = 625;
pub const BT_DURATION_LIMIT_MAX: c_int = 1250;
pub const BT_DURATION_LIMIT_MIN: c_int = 625;
pub const BT_ON_THRESHOLD_DEF: c_int = 4;
pub const BT_ON_THRESHOLD_MAX: c_int = 1000;
pub const BT_ON_THRESHOLD_MIN: c_int = 1;
pub const BT_FRAG_THRESHOLD_DEF: c_int = 0;
pub const BT_FRAG_THRESHOLD_MAX: c_int = 0;
pub const BT_FRAG_THRESHOLD_MIN: c_int = 0;
pub const BT_AGG_THRESHOLD_DEF: c_int = 1200;
pub const BT_AGG_THRESHOLD_MAX: c_int = 8000;
pub const BT_AGG_THRESHOLD_MIN: c_int = 400;
//
// REPLY_BT_CONFIG = 0x9b (command, has simple generic response)
//
// agn devices support hardware handshake with Bluetooth device on
// same platform.  Bluetooth device alerts wireless device when it will Tx;
// wireless device can delay or kill its own Tx to accommodate.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_cmd {
    pub flags: u8,
    pub lead_time: u8,
    pub max_kill: u8,
    pub reserved: u8,
    pub kill_ack_mask: __le32,
    pub kill_cts_mask: __le32,
    pub __packed: },

pub const IWLAGN_BT_FLAG_COEX_MODE_SHIFT: c_int = 3;
pub const IWLAGN_BT_FLAG_COEX_MODE_DISABLED: c_int = 0;
pub const IWLAGN_BT_FLAG_COEX_MODE_LEGACY_2W: c_int = 1;
pub const IWLAGN_BT_FLAG_COEX_MODE_3W: c_int = 2;
pub const IWLAGN_BT_FLAG_COEX_MODE_4W: c_int = 3;

// Disable Sync PSPoll on SCO/eSCO

pub const IWLAGN_BT_PRIO_BOOST_MAX: c_uint = 0xFF;
pub const IWLAGN_BT_PRIO_BOOST_MIN: c_uint = 0x00;
pub const IWLAGN_BT_PRIO_BOOST_DEFAULT: c_uint = 0xF0;
pub const IWLAGN_BT_PRIO_BOOST_DEFAULT32: c_uint = 0xF0F0F0F0;
pub const IWLAGN_BT_MAX_KILL_DEFAULT: c_int = 5;
pub const IWLAGN_BT3_T7_DEFAULT: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_kill_idx {
    IWL_BT_KILL_DEFAULT = 0,
    IWL_BT_KILL_OVERRIDE = 1,
    IWL_BT_KILL_REDUCE = 2,
}

pub const IWLAGN_BT3_PRIO_SAMPLE_DEFAULT: c_int = 2;
pub const IWLAGN_BT3_T2_DEFAULT: c_uint = 0xc;

pub const IWLAGN_BT_DECISION_LUT_SIZE: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_basic_bt_cmd {
    pub flags: u8,
    pub /: *mut *mut u8 ledtime; / unused,
    pub max_kill: u8,
    pub bt3_timer_t7_value: u8,
    pub kill_ack_mask: __le32,
    pub kill_cts_mask: __le32,
    pub bt3_prio_sample_time: u8,
    pub bt3_timer_t2_value: u8,
    pub /: *mut *mut __le16 bt4_reaction_time; / unused,
    pub bt3_lookup_table: [__le32; IWLAGN_BT_DECISION_LUT_SIZE],
//
// bit 0: use reduced tx power for control frame
// bit 1 - 7: reserved
//
    pub reduce_txpower: u8,
    pub reserved: u8,
    pub valid: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_cmd_v1 {
    pub basic: iwl_basic_bt_cmd,
    pub prio_boost: u8,
//
// set IWLAGN_BT_VALID_BOOST to "1" in "valid" bitmask
// if configure the following patterns
//
    pub /: *mut *mut u8 tx_prio_boost; / SW boost of WiFi tx priority,
    pub /: *mut *mut __le16 rx_prio_boost; / SW boost of WiFi rx priority,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_cmd_v2 {
    pub basic: iwl_basic_bt_cmd,
    pub prio_boost: __le32,
//
// set IWLAGN_BT_VALID_BOOST to "1" in "valid" bitmask
// if configure the following patterns
//
    pub reserved: u8,
    pub /: *mut *mut u8 tx_prio_boost; / SW boost of WiFi tx priority,
    pub /: *mut *mut __le16 rx_prio_boost; / SW boost of WiFi rx priority,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_bt_sco_cmd {
    pub flags: __le32,
}

//
// (6)
// Spectrum Management (802.11h) Commands, Responses, Notifications:
//
// Spectrum Management
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_measure_channel {
    pub beacon: *mut *mut __le32 duration; / measurement duration in extended,
// format
    pub /: *mut *mut u8 channel; / channel to measure,
    pub /: *mut *mut u8 type; / see enum iwl_measure_type,
    pub reserved: __le16,
    pub __packed: },
//
// REPLY_SPECTRUM_MEASUREMENT_CMD = 0x74 (command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_spectrum_cmd {
    pub /: *mut *mut __le16 len; / number of bytes starting from token,
    pub /: *mut *mut u8 token; / token id,
    pub /: *mut *mut u8 id; / measurement id -- 0 or 1,
    pub /: *mut *mut u8 origin; / 0 = TGh, 1 = other, 2 = TGk,
    pub /: *mut *mut u8 periodic; / 1 = periodic,
    pub path_loss_timeout: __le16,
    pub /: *mut *mut __le32 start_time; / start time in extended beacon format,
    pub reserved2: __le32,
    pub /: *mut *mut __le32 flags; / rxon flags,
    pub /: *mut *mut __le32 filter_flags; / rxon filter flags,
    pub /: *mut *mut __le16 channel_count; / minimum 1, maximum 10,
    pub reserved3: __le16,
    pub channels: [iwl_measure_channel; 10],
    pub __packed: },
//
// REPLY_SPECTRUM_MEASUREMENT_CMD = 0x74 (response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_spectrum_resp {
    pub token: u8,
    pub /: *mut *mut u8 id; / id of the prior command replaced, or 0xff,
    pub handled: *mut *mut __le16 status; / 0 - command will be,
// 1 - cannot handle (conflicts with another
// measurement)
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_measurement_state {
    IWL_MEASUREMENT_START = 0,
    IWL_MEASUREMENT_STOP = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_measurement_status {
    IWL_MEASUREMENT_OK = 0,
    IWL_MEASUREMENT_CONCURRENT = 1,
    IWL_MEASUREMENT_CSA_CONFLICT = 2,
    IWL_MEASUREMENT_TGH_CONFLICT = 3,
// 4-5 reserved
    IWL_MEASUREMENT_STOPPED = 6,
    IWL_MEASUREMENT_TIMEOUT = 7,
    IWL_MEASUREMENT_PERIODIC_FAILED = 8,
}

pub const NUM_ELEMENTS_IN_HISTOGRAM: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_measurement_histogram {
    pub /: *mut *mut __le32 ofdm[NUM_ELEMENTS_IN_HISTOGRAM]; / in 0.8usec counts,
    pub /: *mut *mut __le32 cck[NUM_ELEMENTS_IN_HISTOGRAM]; / in 1usec counts,
    pub __packed: },
// clear channel availability counters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_measurement_cca_counters {
    pub ofdm: __le32,
    pub cck: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_measure_type {
    IWL_MEASURE_BASIC = (1 << 0),
    IWL_MEASURE_CHANNEL_LOAD = (1 << 1),
    IWL_MEASURE_HISTOGRAM_RPI = (1 << 2),
    IWL_MEASURE_HISTOGRAM_NOISE = (1 << 3),
    IWL_MEASURE_FRAME = (1 << 4),
// bits 5:6 are reserved
    IWL_MEASURE_IDLE = (1 << 7),
}

//
// SPECTRUM_MEASURE_NOTIFICATION = 0x75 (notification only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_spectrum_notification {
    pub /: *mut *mut u8 id; / measurement id -- 0 or 1,
    pub token: u8,
    pub /: *mut *mut u8 channel_index; / index in measurement channel list,
    pub /: *mut *mut u8 state; / 0 - start, 1 - stop,
    pub /: *mut *mut __le32 start_time; / lower 32-bits of TSF,
    pub /: *mut *mut u8 band; / 0 - 5.2GHz, 1 - 2.4GHz,
    pub channel: u8,
    pub /: *mut *mut u8 type; / see enum iwl_measurement_type,
    pub reserved1: u8,
// NOTE:  cca_ofdm, cca_cck, basic_type, and histogram are only
// valid if applicable for measurement type requested.
    pub /: *mut *mut __le32 cca_ofdm; / cca fraction time in 40Mhz clock periods,
    pub /: *mut *mut __le32 cca_cck; / cca fraction time in 44Mhz clock periods,
    pub /: *mut *mut __le32 cca_time; / channel load time in usecs,
    pub -: *mut *mut u8 basic_type; / 0 - bss, 1 - ofdm preamble, 2,
// unidentified
    pub reserved2: [u8; 3],
    pub histogram: iwl_measurement_histogram,
    pub /: *mut *mut __le32 stop_time; / lower 32-bits of TSF,
    pub /: *mut *mut __le32 status; / see iwl_measurement_status,
    pub __packed: },
//
// (7)
// Power Management Commands, Responses, Notifications:
//
// struct iwl_powertable_cmd - Power Table Command
// @flags: See below:
//
// POWER_TABLE_CMD = 0x77 (command, has simple generic response)
//
// PM allow:
// bit 0 - '0' Driver not allow power management
// '1' Driver allow PM (use rest of parameters)
//
// uCode send sleep notifications:
// bit 1 - '0' Don't send sleep notification
// '1' send sleep notification (SEND_PM_NOTIFICATION)
//
// Sleep over DTIM
// bit 2 - '0' PM have to walk up every DTIM
// '1' PM could sleep over DTIM till listen Interval.
//
// PCI power managed
// bit 3 - '0' (PCI_CFG_LINK_CTRL & 0x1)
// '1' !(PCI_CFG_LINK_CTRL & 0x1)
//
// Fast PD
// bit 4 - '1' Put radio to sleep when receiving frame for others
//
// Force sleep Modes
// bit 31/30- '00' use both mac/xtal sleeps
// '01' force Mac sleep
// '10' force xtal sleep
// '11' Illegal set
//
// NOTE: if sleep_interval[SLEEP_INTRVL_TABLE_SIZE-1] > DTIM period then
// ucode assume sleep over DTIM is allowed and we don't need to wake up
// for every DTIM.
//
pub const IWL_POWER_VEC_SIZE: c_int = 5;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_powertable_cmd {
    pub flags: __le16,
    pub keep_alive_seconds: u8,
    pub debug_flags: u8,
    pub rx_data_timeout: __le32,
    pub tx_data_timeout: __le32,
    pub sleep_interval: [__le32; IWL_POWER_VEC_SIZE],
    pub keep_alive_beacons: __le32,
    pub __packed: },
//
// PM_SLEEP_NOTIFICATION = 0x7A (notification only, not a command)
// all devices identical.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sleep_notification {
    pub pm_sleep_mode: u8,
    pub pm_wakeup_src: u8,
    pub reserved: __le16,
    pub sleep_time: __le32,
    pub tsf_low: __le32,
    pub bcon_timer: __le32,
    pub __packed: },
// Sleep states.  all devices identical.
// 3 reserved
}

//
// REPLY_CARD_STATE_CMD = 0xa0 (command, has simple generic response)
//
pub const CARD_STATE_CMD_DISABLE: c_uint = 0x00	/* Put card to sleep */;
pub const CARD_STATE_CMD_ENABLE: c_uint = 0x01	/* Wake up card */;
pub const CARD_STATE_CMD_HALT: c_uint = 0x02	/* Power down permanently */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_card_state_cmd {
    pub /: *mut *mut *mut __le32 status; / CARD_STATE_CMD_ request new power state,
    pub __packed: },
//
// CARD_STATE_NOTIFICATION = 0xa1 (notification only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_card_state_notif {
    pub flags: __le32,
    pub __packed: },
pub const HW_CARD_DISABLED: c_uint = 0x01;
pub const SW_CARD_DISABLED: c_uint = 0x02;
pub const CT_CARD_DISABLED: c_uint = 0x04;
pub const RXON_CARD_DISABLED: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ct_kill_config {
    pub reserved: __le32,
    pub critical_temperature_M: __le32,
    pub critical_temperature_R: __le32,
    pub __packed: },
// 1000, and 6x00
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ct_kill_throttling_config {
    pub critical_temperature_exit: __le32,
    pub reserved: __le32,
    pub critical_temperature_enter: __le32,
    pub __packed: },
//
// (8)
// Scan Commands, Responses, Notifications:
//

//
// struct iwl_scan_channel - entry in REPLY_SCAN_CMD channel table
//
// One for each channel in the scan list.
// Each channel can independently select:
// 1)  SSID for directed active scans
// 2)  Txpower setting (for rate specified within Tx command)
// 3)  How long to stay on-channel (behavior may be modified by quiet_time,
// quiet_plcp_th, good_CRC_th)
//
// To avoid uCode errors, make sure the following are true (see comments
// under struct iwl_scan_cmd about max_out_time and quiet_time):
// 1)  If using passive_dwell (i.e. passive_dwell != 0):
// active_dwell <= passive_dwell (< max_out_time if max_out_time != 0)
// 2)  quiet_time <= active_dwell
// 3)  If restricting off-channel time (i.e. max_out_time !=0):
// passive_dwell < max_out_time
// active_dwell < max_out_time
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_channel {
//
// type is defined as:
// 0:0 1 = active, 0 = passive
// 1:20 SSID direct bit map; if a bit is set, then corresponding
// SSID IE is transmitted in probe request.
// 21:31 reserved
//
    pub type: __le32,
    pub /: *mut *mut __le16 channel; / band is selected by iwl_scan_cmd "flags" field,
    pub /: *mut *mut u8 tx_gain; / gain for analog radio,
    pub /: *mut *mut u8 dsp_atten; / gain for DSP,
    pub /: *mut *mut __le16 active_dwell; / in 1024-uSec TU (time units), typ 5-50,
    pub /: *mut *mut __le16 passive_dwell; / in 1024-uSec TU (time units), typ 20-500,
    pub __packed: },
// set number of direct probes __le32 type

//
// struct iwl_ssid_ie - directed scan network information element
//
// Up to 20 of these may appear in REPLY_SCAN_CMD,
// selected by "type" bit field in struct iwl_scan_channel;
// each channel may select different ssids from among the 20 entries.
// SSID IEs get transmitted in reverse order of entry.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_ssid_ie {
    pub id: u8,
    pub len: u8,
    pub ssid: [u8; 32],
    pub __packed: },
pub const PROBE_OPTION_MAX: c_int = 20;

pub const IWL_GOOD_CRC_TH_DISABLED: c_int = 0;

pub const IWL_MAX_CMD_SIZE: c_int = 4096;
//
// REPLY_SCAN_CMD = 0x80 (command)
//
// The hardware scan command is very powerful; the driver can set it up to
// maintain (relatively) normal network traffic while doing a scan in the
// background.  The max_out_time and suspend_time control the ratio of how
// long the device stays on an associated network channel ("service channel")
// vs. how long it's away from the service channel, i.e. tuned to other channels
// for scanning.
//
// max_out_time is the max time off-channel (in usec), and suspend_time
// is how long (in "extended beacon" format) that the scan is "suspended"
// after returning to the service channel.  That is, suspend_time is the
// time that we stay on the service channel, doing normal work, between
// scan segments.  The driver may set these parameters differently to support
// scanning when associated vs. not associated, and light vs. heavy traffic
// loads when associated.
//
// After receiving this command, the device's scan engine does the following;
//
// 1)  Sends SCAN_START notification to driver
// 2)  Checks to see if it has time to do scan for one channel
// 3)  Sends NULL packet, with power-save (PS) bit set to 1,
// to tell AP that we're going off-channel
// 4)  Tunes to first channel in scan list, does active or passive scan
// 5)  Sends SCAN_RESULT notification to driver
// 6)  Checks to see if it has time to do scan on *next* channel in list
// 7)  Repeats 4-6 until it no longer has time to scan the next channel
// before max_out_time expires
// 8)  Returns to service channel
// 9)  Sends NULL packet with PS=0 to tell AP that we're back
// 10) Stays on service channel until suspend_time expires
// 11) Repeats entire process 2-10 until list is complete
// 12) Sends SCAN_COMPLETE notification
//
// For fast, efficient scans, the scan command also has support for staying on
// a channel for just a short time, if doing active scanning and getting no
// responses to the transmitted probe request.  This time is controlled by
// quiet_time, and the number of received packets below which a channel is
// considered "quiet" is controlled by quiet_plcp_threshold.
//
// For active scanning on channels that have regulatory restrictions against
// blindly transmitting, the scan can listen before transmitting, to make sure
// that there is already legitimate activity on the channel.  If enough
// packets are cleanly received on the channel (controlled by good_CRC_th,
// typical value 1), the scan engine starts transmitting probe requests.
//
// Driver must use separate scan commands for 2.4 vs. 5 GHz bands.
//
// To avoid uCode errors, see timing restrictions described under
// struct iwl_scan_channel.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_scan_flags {
// BIT(0) currently unused
    IWL_SCAN_FLAGS_ACTION_FRAME_TX	= BIT(1),
// bits 2-7 reserved
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scan_cmd {
    pub len: __le16,
    pub /: *mut *mut u8 scan_flags; / scan flags: see enum iwl_scan_flags,
    pub /: *mut *mut u8 channel_count; / # channels in channel list,
    pub channel: *mut *mut __le16 quiet_time; / dwell only this # millisecs on quiet,
// (only for active scan)
    pub /: *mut *mut __le16 quiet_plcp_th; / quiet chnl is < this # pkts (typ. 1),
    pub /: *mut *mut __le16 good_CRC_th; / passive -> active promotion threshold,
    pub /: *mut *mut *mut __le16 rx_chain; / RXON_RX_CHAIN_,
    pub (service): *mut *mut __le32 max_out_time; / max usec to be away from associated,
// channel
    pub beacon: *mut *mut __le32 suspend_time; / pause scan this long (in "extended,
// format") when returning to service chnl:
//
    pub /: *mut *mut *mut __le32 flags; / RXON_FLG_,
    pub /: *mut *mut *mut __le32 filter_flags; / RXON_FILTER_,
// For active scans (set to all-0s for passive scans).
// Does not include payload.  Must specify Tx rate; no rate scaling.
    pub tx_cmd: iwl_tx_cmd_hdr,
// For directed active scans (set to all-0s otherwise)
    pub direct_scan: [iwl_ssid_ie; PROBE_OPTION_MAX],
//
// Probe request frame, followed by channel list.
//
// Size of probe request frame is specified by byte count in tx_cmd.
// Channel list follows immediately after probe request frame.
// Number of channels in list is specified by channel_count.
// Each channel in list is of type:
//
// struct iwl_scan_channel channels[0];
//
// NOTE:  Only one band of channels can be scanned per pass.  You
// must not mix 2.4GHz channels and 5.2GHz channels, and you must wait
// for one scan to complete (i.e. receive SCAN_COMPLETE_NOTIFICATION)
// before requesting another scan.
//
    pub data: [u8; ],
    pub __packed: },
// Can abort will notify by complete notification with abort status.

// complete notification statuses
pub const ABORT_STATUS: c_uint = 0x2;
//
// REPLY_SCAN_CMD = 0x80 (response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scanreq_notification {
    pub /: *mut *mut __le32 status; / 1: okay, 2: cannot fulfill request,
    pub __packed: },
//
// SCAN_START_NOTIFICATION = 0x82 (notification only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scanstart_notification {
    pub tsf_low: __le32,
    pub tsf_high: __le32,
    pub beacon_timer: __le32,
    pub channel: u8,
    pub band: u8,
    pub reserved: [u8; 2],
    pub status: __le32,
    pub __packed: },
pub const SCAN_OWNER_STATUS: c_uint = 0x1;
pub const MEASURE_OWNER_STATUS: c_uint = 0x2;
pub const IWL_PROBE_STATUS_OK: c_int = 0;

// error statuses combined with TX_FAILED

//
// SCAN_RESULTS_NOTIFICATION = 0x83 (notification only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scanresults_notification {
    pub channel: u8,
    pub band: u8,
    pub probe_status: u8,
    pub /: *mut *mut u8 num_probe_not_sent; / not enough time to send,
    pub tsf_low: __le32,
    pub tsf_high: __le32,
    pub statistics: [__le32; NUMBER_OF_STATISTICS],
    pub __packed: },
//
// SCAN_COMPLETE_NOTIFICATION = 0x84 (notification only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_scancomplete_notification {
    pub scanned_channels: u8,
    pub status: u8,
    pub /: *mut *mut u8 bt_status; / BT On/Off status,
    pub last_channel: u8,
    pub tsf_low: __le32,
    pub tsf_high: __le32,
    pub __packed: },
//
// (9)
// IBSS/AP Commands and Notifications:
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ibss_manager {
    IWL_NOT_IBSS_MANAGER = 0,
    IWL_IBSS_MANAGER = 1,
}

//
// BEACON_NOTIFICATION = 0x90 (notification only, not a command)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_beacon_notif {
    pub beacon_notify_hdr: iwlagn_tx_resp,
    pub low_tsf: __le32,
    pub high_tsf: __le32,
    pub ibss_mgr_status: __le32,
    pub __packed: },
//
// REPLY_TX_BEACON = 0x91 (command, has simple generic response)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_tx_beacon_cmd {
    pub tx: iwl_tx_cmd_hdr,
    pub tim_idx: __le16,
    pub tim_size: u8,
    pub reserved1: u8,
    pub /: *mut *mut ieee80211_hdr frame[]; / beacon frame,
    pub __packed: },
//
// (10)
// Statistics Commands and Notifications:
//
pub const IWL_TEMP_CONVERT: c_int = 260;
pub const SUP_RATE_11A_MAX_NUM_CHANNELS: c_int = 8;
pub const SUP_RATE_11B_MAX_NUM_CHANNELS: c_int = 4;
pub const SUP_RATE_11G_MAX_NUM_CHANNELS: c_int = 12;
// Used for passing to driver number of successes and failures per rate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rate_histogram {
    pub a: [__le32; SUP_RATE_11A_MAX_NUM_CHANNELS],
    pub b: [__le32; SUP_RATE_11B_MAX_NUM_CHANNELS],
    pub g: [__le32; SUP_RATE_11G_MAX_NUM_CHANNELS],
    pub success: },
    pub a: [__le32; SUP_RATE_11A_MAX_NUM_CHANNELS],
    pub b: [__le32; SUP_RATE_11B_MAX_NUM_CHANNELS],
    pub g: [__le32; SUP_RATE_11G_MAX_NUM_CHANNELS],
    pub failed: },
    pub __packed: },
// statistics command response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_dbg {
    pub burst_check: __le32,
    pub burst_count: __le32,
    pub wait_for_silence_timeout_cnt: __le32,
    pub reserved: [__le32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_rx_phy {
    pub ina_cnt: __le32,
    pub fina_cnt: __le32,
    pub plcp_err: __le32,
    pub crc32_err: __le32,
    pub overrun_err: __le32,
    pub early_overrun_err: __le32,
    pub crc32_good: __le32,
    pub false_alarm_cnt: __le32,
    pub fina_sync_err_cnt: __le32,
    pub sfd_timeout: __le32,
    pub fina_timeout: __le32,
    pub unresponded_rts: __le32,
    pub rxe_frame_limit_overrun: __le32,
    pub sent_ack_cnt: __le32,
    pub sent_cts_cnt: __le32,
    pub sent_ba_rsp_cnt: __le32,
    pub dsp_self_kill: __le32,
    pub mh_format_err: __le32,
    pub re_acq_main_rssi_sum: __le32,
    pub reserved3: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_rx_ht_phy {
    pub plcp_err: __le32,
    pub overrun_err: __le32,
    pub early_overrun_err: __le32,
    pub crc32_good: __le32,
    pub crc32_err: __le32,
    pub mh_format_err: __le32,
    pub agg_crc32_good: __le32,
    pub agg_mpdu_cnt: __le32,
    pub agg_cnt: __le32,
    pub unsupport_mcs: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_rx_non_phy {
    pub /: *mut *mut __le32 bogus_cts; / CTS received when not expecting CTS,
    pub /: *mut *mut __le32 bogus_ack; / ACK received when not expecting ACK,
    pub that: *mut *mut __le32 non_bssid_frames; / number of frames with BSSID,
// doesn't belong to the STA BSSID
    pub the: *mut *mut __le32 filtered_frames; / count frames that were dumped in,
// filtering process
    pub on: *mut *mut __le32 non_channel_beacons; / beacons with our bss id but not,
// our serving channel
    pub our: *mut *mut __le32 channel_beacons; / beacons with our bss id and in,
// serving channel
    pub /: *mut *mut __le32 num_missed_bcon; / number of missed beacons,
    pub the: *mut *mut __le32 adc_rx_saturation_time; / count in 0.8us units the time,
// ADC was in saturation
    pub searched: *mut *mut __le32 ina_detection_search_time;/ total time (in 0.8us),
// for INA
    pub /: *mut *mut __le32 beacon_silence_rssi_a; / RSSI silence after beacon frame,
    pub /: *mut *mut __le32 beacon_silence_rssi_b; / RSSI silence after beacon frame,
    pub /: *mut *mut __le32 beacon_silence_rssi_c; / RSSI silence after beacon frame,
    pub data: *mut *mut __le32 interference_data_flag; / flag for interference,
// availability. 1 when data is
// available.
    pub /: *mut *mut __le32 channel_load; / counts RX Enable time in uSec,
    pub OFDM: *mut *mut __le32 dsp_false_alarms; / DSP false alarm (both,
// and CCK) counter
    pub beacon_rssi_a: __le32,
    pub beacon_rssi_b: __le32,
    pub beacon_rssi_c: __le32,
    pub beacon_energy_a: __le32,
    pub beacon_energy_b: __le32,
    pub beacon_energy_c: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_rx_non_phy_bt {
    pub common: statistics_rx_non_phy,
// additional stats for bt
    pub num_bt_kills: __le32,
    pub reserved: [__le32; 2],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_rx {
    pub ofdm: statistics_rx_phy,
    pub cck: statistics_rx_phy,
    pub general: statistics_rx_non_phy,
    pub ofdm_ht: statistics_rx_ht_phy,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_rx_bt {
    pub ofdm: statistics_rx_phy,
    pub cck: statistics_rx_phy,
    pub general: statistics_rx_non_phy_bt,
    pub ofdm_ht: statistics_rx_ht_phy,
    pub __packed: },
//
// struct statistics_tx_power - current tx power
//
// @ant_a: current tx power on chain a in 1/2 dB step
// @ant_b: current tx power on chain b in 1/2 dB step
// @ant_c: current tx power on chain c in 1/2 dB step
// @reserved: reserved for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_tx_power {
    pub ant_a: u8,
    pub ant_b: u8,
    pub ant_c: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_tx_non_phy_agg {
    pub ba_timeout: __le32,
    pub ba_reschedule_frames: __le32,
    pub scd_query_agg_frame_cnt: __le32,
    pub scd_query_no_agg: __le32,
    pub scd_query_agg: __le32,
    pub scd_query_mismatch: __le32,
    pub frame_not_ready: __le32,
    pub underrun: __le32,
    pub bt_prio_kill: __le32,
    pub rx_ba_rsp_cnt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_tx {
    pub preamble_cnt: __le32,
    pub rx_detected_cnt: __le32,
    pub bt_prio_defer_cnt: __le32,
    pub bt_prio_kill_cnt: __le32,
    pub few_bytes_cnt: __le32,
    pub cts_timeout: __le32,
    pub ack_timeout: __le32,
    pub expected_ack_cnt: __le32,
    pub actual_ack_cnt: __le32,
    pub dump_msdu_cnt: __le32,
    pub burst_abort_next_frame_mismatch_cnt: __le32,
    pub burst_abort_missing_next_frame_cnt: __le32,
    pub cts_timeout_collision: __le32,
    pub ack_or_ba_timeout_collision: __le32,
    pub agg: statistics_tx_non_phy_agg,
//
// "tx_power" are optional parameters provided by uCode,
// 6000 series is the only device provide the information,
// Those are reserved fields for all the other devices
//
    pub tx_power: statistics_tx_power,
    pub reserved1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_div {
    pub tx_on_a: __le32,
    pub tx_on_b: __le32,
    pub exec_time: __le32,
    pub probe_time: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_general_common {
    pub /: *mut *mut __le32 temperature; / radio temperature,
    pub /: *mut *mut __le32 temperature_m; / radio voltage,
    pub dbg: statistics_dbg,
    pub sleep_time: __le32,
    pub slots_out: __le32,
    pub slots_idle: __le32,
    pub ttl_timestamp: __le32,
    pub div: statistics_div,
    pub rx_enable_counter: __le32,
//
// num_of_sos_states:
// count the number of times we have to re-tune
// in order to get out of bad PHY status
//
    pub num_of_sos_states: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_bt_activity {
// Tx statistics
    pub hi_priority_tx_req_cnt: __le32,
    pub hi_priority_tx_denied_cnt: __le32,
    pub lo_priority_tx_req_cnt: __le32,
    pub lo_priority_tx_denied_cnt: __le32,
// Rx statistics
    pub hi_priority_rx_req_cnt: __le32,
    pub hi_priority_rx_denied_cnt: __le32,
    pub lo_priority_rx_req_cnt: __le32,
    pub lo_priority_rx_denied_cnt: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_general {
    pub common: statistics_general_common,
    pub reserved2: __le32,
    pub reserved3: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct statistics_general_bt {
    pub common: statistics_general_common,
    pub activity: statistics_bt_activity,
    pub reserved2: __le32,
    pub reserved3: __le32,
    pub __packed: },

//
// REPLY_STATISTICS_CMD = 0x9c,
// all devices identical.
//
// This command triggers an immediate response containing uCode statistics.
// The response is in the same format as STATISTICS_NOTIFICATION 0x9d, below.
//
// If the CLEAR_STATS configuration flag is set, uCode will clear its
// internal copy of the statistics (counters) after issuing the response.
// This flag does not affect STATISTICS_NOTIFICATIONs after beacons (see below).
//
// If the DISABLE_NOTIF configuration flag is set, uCode will not issue
// STATISTICS_NOTIFICATIONs after received beacons (see below).  This flag
// does not affect the response to the REPLY_STATISTICS_CMD 0x9c itself.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_statistics_cmd {
    pub /: *mut *mut *mut __le32 configuration_flags; / IWL_STATS_CONF_,
    pub __packed: },
//
// STATISTICS_NOTIFICATION = 0x9d (notification only, not a command)
//
// By default, uCode issues this notification after receiving a beacon
// while associated.  To disable this behavior, set DISABLE_NOTIF flag in the
// REPLY_STATISTICS_CMD 0x9c, above.
//
// Statistics counters continue to increment beacon after beacon, but are
// cleared when changing channels or when driver issues REPLY_STATISTICS_CMD
// 0x9c with CLEAR_STATS bit set (see above).
//
// uCode also issues this notification during scans.  uCode clears statistics
// appropriately so that each notification contains statistics for only the
// one channel that has just been scanned.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_notif_statistics {
    pub flag: __le32,
    pub rx: statistics_rx,
    pub tx: statistics_tx,
    pub general: statistics_general,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_notif_statistics {
    pub flag: __le32,
    pub rx: statistics_rx_bt,
    pub tx: statistics_tx,
    pub general: statistics_general_bt,
    pub __packed: },
//
// MISSED_BEACONS_NOTIFICATION = 0xa2 (notification only, not a command)
//
// uCode send MISSED_BEACONS_NOTIFICATION to driver when detect beacon missed
// in regardless of how many missed beacons, which mean when driver receive the
// notification, inside the command, it can find all the beacons information
// which include number of total missed beacons, number of consecutive missed
// beacons, number of beacons received and number of beacons expected to
// receive.
//
// If uCode detected consecutive_missed_beacons > 5, it will reset the radio
// in order to bring the radio/PHY back to working state; which has no relation
// to when driver will perform sensitivity calibration.
//
// Driver should set it own missed_beacon_threshold to decide when to perform
// sensitivity calibration based on number of consecutive missed beacons in
// order to improve overall performance, especially in noisy environment.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_missed_beacon_notif {
    pub consecutive_missed_beacons: __le32,
    pub total_missed_becons: __le32,
    pub num_expected_beacons: __le32,
    pub num_recvd_beacons: __le32,
    pub __packed: },
//
// (11)
// Rx Calibration Commands:
//
// With the uCode used for open source drivers, most Tx calibration (except
// for Tx Power) and most Rx calibration is done by uCode during the
// "initialize" phase of uCode boot.  Driver must calibrate only:
//
// 1)  Tx power (depends on temperature), described elsewhere
// 2)  Receiver gain balance (optimize MIMO, and detect disconnected antennas)
// 3)  Receiver sensitivity (to optimize signal detection)
//
// SENSITIVITY_CMD = 0xa8 (command, has simple generic response)
//
// This command sets up the Rx signal detector for a sensitivity level that
// is high enough to lock onto all signals within the associated network,
// but low enough to ignore signals that are below a certain threshold, so as
// not to have too many "false alarms".  False alarms are signals that the
// Rx DSP tries to lock onto, but then discards after determining that they
// are noise.
//
// The optimum number of false alarms is between 5 and 50 per 200 TUs
// (200 * 1024 uSecs, i.e. 204.8 milliseconds) of actual Rx time (i.e.
// time listening, not transmitting).  Driver must adjust sensitivity so that
// the ratio of actual false alarms to actual Rx time falls within this range.
//
// While associated, uCode delivers STATISTICS_NOTIFICATIONs after each
// received beacon.  These provide information to the driver to analyze the
// sensitivity.  Don't analyze statistics that come in from scanning, or any
// other non-associated-network source.  Pertinent statistics include:
//
// From "general" statistics (struct statistics_rx_non_phy):
//
// (beacon_energy_[abc] & 0x0FF00) >> 8 (unsigned, higher value is lower level)
// Measure of energy of desired signal.  Used for establishing a level
// below which the device does not detect signals.
//
// (beacon_silence_rssi_[abc] & 0x0FF00) >> 8 (unsigned, units in dB)
// Measure of background noise in silent period after beacon.
//
// channel_load
// uSecs of actual Rx time during beacon period (varies according to
// how much time was spent transmitting).
//
// From "cck" and "ofdm" statistics (struct statistics_rx_phy), separately:
//
// false_alarm_cnt
// Signal locks abandoned early (before phy-level header).
//
// plcp_err
// Signal locks abandoned late (during phy-level header).
//
// NOTE:  Both false_alarm_cnt and plcp_err increment monotonically from
// beacon to beacon, i.e. each value is an accumulation of all errors
// before and including the latest beacon.  Values will wrap around to 0
// after counting up to 2^32 - 1.  Driver must differentiate vs.
// previous beacon's values to determine # false alarms in the current
// beacon period.
//
// Total number of false alarms = false_alarms + plcp_errs
//
// For OFDM, adjust the following table entries in struct iwl_sensitivity_cmd
// (notice that the start points for OFDM are at or close to settings for
// maximum sensitivity):
//
// START  /  MIN  /  MAX
// HD_AUTO_CORR32_X1_TH_ADD_MIN_INDEX          90   /   85  /  120
// HD_AUTO_CORR32_X1_TH_ADD_MIN_MRC_INDEX     170   /  170  /  210
// HD_AUTO_CORR32_X4_TH_ADD_MIN_INDEX         105   /  105  /  140
// HD_AUTO_CORR32_X4_TH_ADD_MIN_MRC_INDEX     220   /  220  /  270
//
// If actual rate of OFDM false alarms (+ plcp_errors) is too high
// (greater than 50 for each 204.8 msecs listening), reduce sensitivity
// by *adding* 1 to all 4 of the table entries above, up to the max for
// each entry.  Conversely, if false alarm rate is too low (less than 5
// for each 204.8 msecs listening), *subtract* 1 from each entry to
// increase sensitivity.
//
// For CCK sensitivity, keep track of the following:
//
// 1).  20-beacon history of maximum background noise, indicated by
// (beacon_silence_rssi_[abc] & 0x0FF00), units in dB, across the
// 3 receivers.  For any given beacon, the "silence reference" is
// the maximum of last 60 samples (20 beacons * 3 receivers).
//
// 2).  10-beacon history of strongest signal level, as indicated
// by (beacon_energy_[abc] & 0x0FF00) >> 8, across the 3 receivers,
// i.e. the strength of the signal through the best receiver at the
// moment.  These measurements are "upside down", with lower values
// for stronger signals, so max energy will be *minimum* value.
//
// Then for any given beacon, the driver must determine the *weakest
// of the strongest signals; this is the minimum level that needs to be
// successfully detected, when using the best receiver at the moment.
// "Max cck energy" is the maximum (higher value means lower energy!)
// of the last 10 minima.  Once this is determined, driver must add
// a little margin by adding "6" to it.
//
// 3).  Number of consecutive beacon periods with too few false alarms.
// Reset this to 0 at the first beacon period that falls within the
// "good" range (5 to 50 false alarms per 204.8 milliseconds rx).
//
// Then, adjust the following CCK table entries in struct iwl_sensitivity_cmd
// (notice that the start points for CCK are at maximum sensitivity):
//
// START  /  MIN  /  MAX
// HD_AUTO_CORR40_X4_TH_ADD_MIN_INDEX         125   /  125  /  200
// HD_AUTO_CORR40_X4_TH_ADD_MIN_MRC_INDEX     200   /  200  /  400
// HD_MIN_ENERGY_CCK_DET_INDEX                100   /    0  /  100
//
// If actual rate of CCK false alarms (+ plcp_errors) is too high
// (greater than 50 for each 204.8 msecs listening), method for reducing
// sensitivity is:
//
// 1)  *Add* 3 to value in HD_AUTO_CORR40_X4_TH_ADD_MIN_MRC_INDEX,
// up to max 400.
//
// 2)  If current value in HD_AUTO_CORR40_X4_TH_ADD_MIN_INDEX is < 160,
// sensitivity has been reduced a significant amount; bring it up to
// a moderate 161.  Otherwise, *add* 3, up to max 200.
//
// 3)  a)  If current value in HD_AUTO_CORR40_X4_TH_ADD_MIN_INDEX is > 160,
// sensitivity has been reduced only a moderate or small amount;
// *subtract* 2 from value in HD_MIN_ENERGY_CCK_DET_INDEX,
// down to min 0.  Otherwise (if gain has been significantly reduced),
// don't change the HD_MIN_ENERGY_CCK_DET_INDEX value.
//
// b)  Save a snapshot of the "silence reference".
//
// If actual rate of CCK false alarms (+ plcp_errors) is too low
// (less than 5 for each 204.8 msecs listening), method for increasing
// sensitivity is used only if:
//
// 1a)  Previous beacon did not have too many false alarms
// 1b)  AND difference between previous "silence reference" and current
// "silence reference" (prev - current) is 2 or more,
// OR 2)  100 or more consecutive beacon periods have had rate of
// less than 5 false alarms per 204.8 milliseconds rx time.
//
// Method for increasing sensitivity:
//
// 1)  *Subtract* 3 from value in HD_AUTO_CORR40_X4_TH_ADD_MIN_INDEX,
// down to min 125.
//
// 2)  *Subtract* 3 from value in HD_AUTO_CORR40_X4_TH_ADD_MIN_MRC_INDEX,
// down to min 200.
//
// 3)  *Add* 2 to value in HD_MIN_ENERGY_CCK_DET_INDEX, up to max 100.
//
// If actual rate of CCK false alarms (+ plcp_errors) is within good range
// (between 5 and 50 for each 204.8 msecs listening):
//
// 1)  Save a snapshot of the silence reference.
//
// 2)  If previous beacon had too many CCK false alarms (+ plcp_errors),
// give some extra margin to energy threshold by *subtracting* 8
// from value in HD_MIN_ENERGY_CCK_DET_INDEX.
//
// For all cases (too few, too many, good range), make sure that the CCK
// detection threshold (energy) is below the energy level for robust
// detection over the past 10 beacon periods, the "Max cck energy".
// Lower values mean higher energy; this means making sure that the value
// in HD_MIN_ENERGY_CCK_DET_INDEX is at or *above* "Max cck energy".
//
// Table entries in SENSITIVITY_CMD (struct iwl_sensitivity_cmd)
//

//
// Additional table entries in enhance SENSITIVITY_CMD
//

// number of entries for enhanced tbl

// number of additional entries for enhanced tbl

// Control field in struct iwl_sensitivity_cmd

//
// struct iwl_sensitivity_cmd - sensitivity configuration command
// @control:  (1) updates working table, (0) updates default table
// @table:  energy threshold values, use HD_* as index into table
//
// Always use "1" in "control" to update uCode's working table and DSP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_sensitivity_cmd {
    pub /: *mut *mut __le16 control; / always use "1",
    pub /: *mut *mut *mut __le16 table[HD_TABLE_SIZE]; / use HD_ as index,
    pub __packed: },
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_enhance_sensitivity_cmd {
    pub /: *mut *mut __le16 control; / always use "1",
    pub /: *mut *mut *mut __le16 enhance_table[ENHANCE_HD_TABLE_SIZE]; / use HD_ as index,
    pub __packed: },
//
// REPLY_PHY_CALIBRATION_CMD = 0xb0 (command, has simple generic response)
//
// This command sets the relative gains of agn device's 3 radio receiver chains.
//
// After the first association, driver should accumulate signal and noise
// statistics from the STATISTICS_NOTIFICATIONs that follow the first 20
// beacons from the associated network (don't collect statistics that come
// in from scanning, or any other non-network source).
//
// DISCONNECTED ANTENNA:
//
// Driver should determine which antennas are actually connected, by comparing
// average beacon signal levels for the 3 Rx chains.  Accumulate (add) the
// following values over 20 beacons, one accumulator for each of the chains
// a/b/c, from struct statistics_rx_non_phy:
//
// beacon_rssi_[abc] & 0x0FF (unsigned, units in dB)
//
// Find the strongest signal from among a/b/c.  Compare the other two to the
// strongest.  If any signal is more than 15 dB (times 20, unless you
// divide the accumulated values by 20) below the strongest, the driver
// considers that antenna to be disconnected, and should not try to use that
// antenna/chain for Rx or Tx.  If both A and B seem to be disconnected,
// driver should declare the stronger one as connected, and attempt to use it
// (A and B are the only 2 Tx chains!).
//
// RX BALANCE:
//
// Driver should balance the 3 receivers (but just the ones that are connected
// to antennas, see above) for gain, by comparing the average signal levels
// detected during the silence after each beacon (background noise).
// Accumulate (add) the following values over 20 beacons, one accumulator for
// each of the chains a/b/c, from struct statistics_rx_non_phy:
//
// beacon_silence_rssi_[abc] & 0x0FF (unsigned, units in dB)
//
// Find the weakest background noise level from among a/b/c.  This Rx chain
// will be the reference, with 0 gain adjustment.  Attenuate other channels by
// finding noise difference:
//
// (accum_noise[i] - accum_noise[reference]) / 30
//
// The "30" adjusts the dB in the 20 accumulated samples to units of 1.5 dB.
// For use in diff_gain_[abc] fields of struct iwl_calibration_cmd, the
// driver should limit the difference results to a range of 0-3 (0-4.5 dB),
// and set bit 2 to indicate "reduce gain".  The value for the reference
// (weakest) chain should be "0".
//
// diff_gain_[abc] bit fields:
// 2: (1) reduce gain, (0) increase gain
// 1-0: amount of gain, units of 1.5 dB
//
// Phy calibration command for series
}

// This enum defines the bitmap of various calibrations to enable in both
// init ucode and runtime ucode through CALIBRATION_CFG_CMD.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_ucode_calib_cfg {
    IWL_CALIB_CFG_RX_BB_IDX			= BIT(0),
    IWL_CALIB_CFG_DC_IDX			= BIT(1),
    IWL_CALIB_CFG_LO_IDX			= BIT(2),
    IWL_CALIB_CFG_TX_IQ_IDX			= BIT(3),
    IWL_CALIB_CFG_RX_IQ_IDX			= BIT(4),
    IWL_CALIB_CFG_NOISE_IDX			= BIT(5),
    IWL_CALIB_CFG_CRYSTAL_IDX		= BIT(6),
    IWL_CALIB_CFG_TEMPERATURE_IDX		= BIT(7),
    IWL_CALIB_CFG_PAPD_IDX			= BIT(8),
    IWL_CALIB_CFG_SENSITIVITY_IDX		= BIT(9),
    IWL_CALIB_CFG_TX_PWR_IDX		= BIT(10),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_cfg_elmnt_s {
    pub is_enable: __le32,
    pub start: __le32,
    pub send_res: __le32,
    pub apply_res: __le32,
    pub reserved: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_cfg_status_s {
    pub once: iwl_calib_cfg_elmnt_s,
    pub perd: iwl_calib_cfg_elmnt_s,
    pub flags: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_cfg_cmd {
    pub ucd_calib_cfg: iwl_calib_cfg_status_s,
    pub drv_calib_cfg: iwl_calib_cfg_status_s,
    pub reserved1: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_hdr {
    pub op_code: u8,
    pub first_group: u8,
    pub groups_num: u8,
    pub data_valid: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_cmd {
    pub hdr: iwl_calib_hdr,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_xtal_freq_cmd {
    pub hdr: iwl_calib_hdr,
    pub cap_pin1: u8,
    pub cap_pin2: u8,
    pub pad: [u8; 2],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_temperature_offset_cmd {
    pub hdr: iwl_calib_hdr,
    pub radio_sensor_offset: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_temperature_offset_v2_cmd {
    pub hdr: iwl_calib_hdr,
    pub radio_sensor_offset_high: __le16,
    pub radio_sensor_offset_low: __le16,
    pub burntVoltageRef: __le16,
    pub reserved: __le16,
    pub __packed: },
// IWL_PHY_CALIBRATE_CHAIN_NOISE_RESET_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_chain_noise_reset_cmd {
    pub hdr: iwl_calib_hdr,
    pub data: [u8; ],
}

// IWL_PHY_CALIBRATE_CHAIN_NOISE_GAIN_CMD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_calib_chain_noise_gain_cmd {
    pub hdr: iwl_calib_hdr,
    pub delta_gain_1: u8,
    pub delta_gain_2: u8,
    pub pad: [u8; 2],
    pub __packed: },
//
// (12)
// Miscellaneous Commands:
//
// LEDs Command & Response
// REPLY_LEDS_CMD = 0x48 (command, has simple generic response)
//
// For each of 3 possible LEDs (Activity/Link/Tech, selected by "id" field),
// this command turns it on or off, or sets up a periodic blinking cycle.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_led_cmd {
    pub /: *mut *mut __le32 interval; / "interval" in uSec,
    pub /: *mut *mut u8 id; / 1: Activity, 2: Link, 3: Tech,
    pub blinking: *mut *mut u8 off; / # intervals off while,
// "0", with >0 "on" value, turns LED on
    pub blinking: *mut *mut u8 on; / # intervals on while,
// "0", regardless of "off", turns LED off
    pub reserved: u8,
    pub __packed: },
//
// station priority table entries
// also used as potential "events" value for both
// COEX_MEDIUM_NOTIFICATION and COEX_EVENT_CMD
//
// COEX events entry flag masks
// RP - Requested Priority
// WP - Win Medium Priority: priority assigned when the contention has been won
//

pub const COEX_CU_UNASSOC_IDLE_RP: c_int = 4;
pub const COEX_CU_UNASSOC_MANUAL_SCAN_RP: c_int = 4;
pub const COEX_CU_UNASSOC_AUTO_SCAN_RP: c_int = 4;
pub const COEX_CU_CALIBRATION_RP: c_int = 4;
pub const COEX_CU_PERIODIC_CALIBRATION_RP: c_int = 4;
pub const COEX_CU_CONNECTION_ESTAB_RP: c_int = 4;
pub const COEX_CU_ASSOCIATED_IDLE_RP: c_int = 4;
pub const COEX_CU_ASSOC_MANUAL_SCAN_RP: c_int = 4;
pub const COEX_CU_ASSOC_AUTO_SCAN_RP: c_int = 4;
pub const COEX_CU_ASSOC_ACTIVE_LEVEL_RP: c_int = 4;
pub const COEX_CU_RF_ON_RP: c_int = 6;
pub const COEX_CU_RF_OFF_RP: c_int = 4;
pub const COEX_CU_STAND_ALONE_DEBUG_RP: c_int = 6;
pub const COEX_CU_IPAN_ASSOC_LEVEL_RP: c_int = 4;
pub const COEX_CU_RSRVD1_RP: c_int = 4;
pub const COEX_CU_RSRVD2_RP: c_int = 4;
pub const COEX_CU_UNASSOC_IDLE_WP: c_int = 3;
pub const COEX_CU_UNASSOC_MANUAL_SCAN_WP: c_int = 3;
pub const COEX_CU_UNASSOC_AUTO_SCAN_WP: c_int = 3;
pub const COEX_CU_CALIBRATION_WP: c_int = 3;
pub const COEX_CU_PERIODIC_CALIBRATION_WP: c_int = 3;
pub const COEX_CU_CONNECTION_ESTAB_WP: c_int = 3;
pub const COEX_CU_ASSOCIATED_IDLE_WP: c_int = 3;
pub const COEX_CU_ASSOC_MANUAL_SCAN_WP: c_int = 3;
pub const COEX_CU_ASSOC_AUTO_SCAN_WP: c_int = 3;
pub const COEX_CU_ASSOC_ACTIVE_LEVEL_WP: c_int = 3;
pub const COEX_CU_RF_ON_WP: c_int = 3;
pub const COEX_CU_RF_OFF_WP: c_int = 3;
pub const COEX_CU_STAND_ALONE_DEBUG_WP: c_int = 6;
pub const COEX_CU_IPAN_ASSOC_LEVEL_WP: c_int = 3;
pub const COEX_CU_RSRVD1_WP: c_int = 3;
pub const COEX_CU_RSRVD2_WP: c_int = 3;
pub const COEX_UNASSOC_IDLE_FLAGS: c_int = 0;

pub const COEX_PERIODIC_CALIBRATION_FLAGS: c_int = 0;
//
// COEX_CONNECTION_ESTAB:
// we need DELAY_MEDIUM_FREE_NTFY to let WiMAX disconnect from network.
//

pub const COEX_ASSOCIATED_IDLE_FLAGS: c_int = 0;

pub const COEX_ASSOC_ACTIVE_LEVEL_FLAGS: c_int = 0;
pub const COEX_RF_ON_FLAGS: c_int = 0;
pub const COEX_RF_OFF_FLAGS: c_int = 0;

pub const COEX_RSRVD1_FLAGS: c_int = 0;
pub const COEX_RSRVD2_FLAGS: c_int = 0;
//
// COEX_CU_RF_ON is the event wrapping all radio ownership.
// We need DELAY_MEDIUM_FREE_NTFY to let WiMAX disconnect from network.
//

// un-association part
// calibration
// connection
// association part
// RF ON/OFF
// IPAN
// reserved
}

//
// Coexistence WIFI/WIMAX  Command
// COEX_PRIORITY_TABLE_CMD = 0x5a
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wimax_coex_event_entry {
    pub request_prio: u8,
    pub win_medium_prio: u8,
    pub reserved: u8,
    pub flags: u8,
    pub __packed: },
// COEX flag masks
// Station table is valid

// UnMask wake up src at unassociated sleep

// UnMask wake up src at associated sleep

// Enable CoEx feature.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wimax_coex_cmd {
    pub flags: u8,
    pub reserved: [u8; 3],
    pub sta_prio: [iwl_wimax_coex_event_entry; COEX_NUM_OF_EVENTS],
    pub __packed: },
//
// Coexistence MEDIUM NOTIFICATION
// COEX_MEDIUM_NOTIFICATION = 0x5b
//
// notification from uCode to host to indicate medium changes
//
// status field
// bit 0 - 2: medium status
// bit 3: medium change indication
// bit 4 - 31: reserved
//
// status option values, (0 - 2 bits)

// send notification status (1 bit)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_coex_medium_notification {
    pub status: __le32,
    pub events: __le32,
    pub __packed: },
//
// Coexistence EVENT  Command
// COEX_EVENT_CMD = 0x5c
//
// send from host to uCode for coex event request.
//
// flags options

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_coex_event_cmd {
    pub flags: u8,
    pub event: u8,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_coex_event_resp {
    pub status: __le32,
    pub __packed: },
//
// Bluetooth Coexistence commands
//
// BT Status notification
// REPLY_BT_COEX_PROFILE_NOTIF = 0xce
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwl_bt_coex_profile_traffic_load {
    IWL_BT_COEX_TRAFFIC_LOAD_NONE = 	0,
    IWL_BT_COEX_TRAFFIC_LOAD_LOW =		1,
    IWL_BT_COEX_TRAFFIC_LOAD_HIGH = 	2,
    IWL_BT_COEX_TRAFFIC_LOAD_CONTINUOUS =	3,
//
// There are no more even though below is a u8, the
// indication from the BT device only has two bits.
//
}

pub const BT_SESSION_ACTIVITY_1_UART_MSG: c_uint = 0x1;
pub const BT_SESSION_ACTIVITY_2_UART_MSG: c_uint = 0x2;
// BT UART message - Share Part (BT -> WiFi)

// BT Session Activity 2 UART message (BT -> WiFi)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_uart_msg {
    pub header: u8,
    pub frame1: u8,
    pub frame2: u8,
    pub frame3: u8,
    pub frame4: u8,
    pub frame5: u8,
    pub frame6: u8,
    pub frame7: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_profile_notif {
    pub last_bt_uart_msg: iwl_bt_uart_msg,
    pub /: *mut *mut u8 bt_status; / 0 - off, 1 - on,
    pub /: *mut *mut u8 bt_traffic_load; / 0 .. 3?,
    pub /: *mut *mut u8 bt_ci_compliance; / 0 - not complied, 1 - complied,
    pub reserved: u8,
    pub __packed: },
pub const IWL_BT_COEX_PRIO_TBL_SHARED_ANTENNA_POS: c_int = 0;
pub const IWL_BT_COEX_PRIO_TBL_SHARED_ANTENNA_MSK: c_uint = 0x1;
pub const IWL_BT_COEX_PRIO_TBL_PRIO_POS: c_int = 1;
pub const IWL_BT_COEX_PRIO_TBL_PRIO_MASK: c_uint = 0x0e;
pub const IWL_BT_COEX_PRIO_TBL_RESERVED_POS: c_int = 4;
pub const IWL_BT_COEX_PRIO_TBL_RESERVED_MASK: c_uint = 0xf0;
pub const IWL_BT_COEX_PRIO_TBL_PRIO_SHIFT: c_int = 1;
//
// BT Coexistence Priority table
// REPLY_BT_COEX_PRIO_TABLE = 0xcc
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_coex_prio_table_events {
    BT_COEX_PRIO_TBL_EVT_INIT_CALIB1 = 0,
    BT_COEX_PRIO_TBL_EVT_INIT_CALIB2 = 1,
    BT_COEX_PRIO_TBL_EVT_PERIODIC_CALIB_LOW1 = 2,
    BT_COEX_PRIO_TBL_EVT_PERIODIC_CALIB_LOW2 = 3, /* DC calib */
    BT_COEX_PRIO_TBL_EVT_PERIODIC_CALIB_HIGH1 = 4,
    BT_COEX_PRIO_TBL_EVT_PERIODIC_CALIB_HIGH2 = 5,
    BT_COEX_PRIO_TBL_EVT_DTIM = 6,
    BT_COEX_PRIO_TBL_EVT_SCAN52 = 7,
    BT_COEX_PRIO_TBL_EVT_SCAN24 = 8,
    BT_COEX_PRIO_TBL_EVT_RESERVED0 = 9,
    BT_COEX_PRIO_TBL_EVT_RESERVED1 = 10,
    BT_COEX_PRIO_TBL_EVT_RESERVED2 = 11,
    BT_COEX_PRIO_TBL_EVT_RESERVED3 = 12,
    BT_COEX_PRIO_TBL_EVT_RESERVED4 = 13,
    BT_COEX_PRIO_TBL_EVT_RESERVED5 = 14,
    BT_COEX_PRIO_TBL_EVT_RESERVED6 = 15,
// BT_COEX_PRIO_TBL_EVT_MAX should always be last
    BT_COEX_PRIO_TBL_EVT_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bt_coex_prio_table_priorities {
    BT_COEX_PRIO_TBL_DISABLED = 0,
    BT_COEX_PRIO_TBL_PRIO_LOW = 1,
    BT_COEX_PRIO_TBL_PRIO_HIGH = 2,
    BT_COEX_PRIO_TBL_PRIO_BYPASS = 3,
    BT_COEX_PRIO_TBL_PRIO_COEX_OFF = 4,
    BT_COEX_PRIO_TBL_PRIO_COEX_ON = 5,
    BT_COEX_PRIO_TBL_PRIO_RSRVD1 = 6,
    BT_COEX_PRIO_TBL_PRIO_RSRVD2 = 7,
    BT_COEX_PRIO_TBL_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_prio_table_cmd {
    pub prio_tbl: [u8; BT_COEX_PRIO_TBL_EVT_MAX],
    pub __packed: },
pub const IWL_BT_COEX_ENV_CLOSE: c_int = 0;
pub const IWL_BT_COEX_ENV_OPEN: c_int = 1;
//
// BT Protection Envelope
// REPLY_BT_COEX_PROT_ENV = 0xcd
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_bt_coex_prot_env_cmd {
    pub /: *mut *mut u8 action; / 0 = closed, 1 = open,
    pub /: *mut *mut u8 type; / 0 .. 15,
    pub reserved: [u8; 2],
    pub __packed: },
//
// REPLY_D3_CONFIG
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwlagn_d3_wakeup_filters {
    IWLAGN_D3_WAKEUP_RFKILL		= BIT(0),
    IWLAGN_D3_WAKEUP_SYSASSERT	= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_d3_config_cmd {
    pub min_sleep_time: __le32,
    pub wakeup_flags: __le32,
    pub __packed: },
//
// REPLY_WOWLAN_PATTERNS
//
pub const IWLAGN_WOWLAN_MIN_PATTERN_LEN: c_int = 16;
pub const IWLAGN_WOWLAN_MAX_PATTERN_LEN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_wowlan_pattern {
    pub 8]: u8 mask[IWLAGN_WOWLAN_MAX_PATTERN_LEN /,
    pub pattern: [u8; IWLAGN_WOWLAN_MAX_PATTERN_LEN],
    pub mask_size: u8,
    pub pattern_size: u8,
    pub reserved: __le16,
    pub __packed: },
pub const IWLAGN_WOWLAN_MAX_PATTERNS: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_wowlan_patterns_cmd {
    pub n_patterns: __le32,
    pub patterns: [iwlagn_wowlan_pattern; ],
    pub __packed: },
//
// REPLY_WOWLAN_WAKEUP_FILTER
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iwlagn_wowlan_wakeup_filters {
    IWLAGN_WOWLAN_WAKEUP_MAGIC_PACKET	= BIT(0),
    IWLAGN_WOWLAN_WAKEUP_PATTERN_MATCH	= BIT(1),
    IWLAGN_WOWLAN_WAKEUP_BEACON_MISS	= BIT(2),
    IWLAGN_WOWLAN_WAKEUP_LINK_CHANGE	= BIT(3),
    IWLAGN_WOWLAN_WAKEUP_GTK_REKEY_FAIL	= BIT(4),
    IWLAGN_WOWLAN_WAKEUP_EAP_IDENT_REQ	= BIT(5),
    IWLAGN_WOWLAN_WAKEUP_4WAY_HANDSHAKE	= BIT(6),
    IWLAGN_WOWLAN_WAKEUP_ALWAYS		= BIT(7),
    IWLAGN_WOWLAN_WAKEUP_ENABLE_NET_DETECT	= BIT(8),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_wowlan_wakeup_filter_cmd {
    pub enabled: __le32,
    pub non_qos_seq: __le16,
    pub reserved: __le16,
    pub qos_seq: [__le16; 8],
}

//
// REPLY_WOWLAN_TSC_RSC_PARAMS
//
pub const IWLAGN_NUM_RSC: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tkip_sc {
    pub iv16: __le16,
    pub pad: __le16,
    pub iv32: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_tkip_rsc_tsc {
    pub unicast_rsc: [tkip_sc; IWLAGN_NUM_RSC],
    pub multicast_rsc: [tkip_sc; IWLAGN_NUM_RSC],
    pub tsc: tkip_sc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aes_sc {
    pub pn: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_aes_rsc_tsc {
    pub unicast_rsc: [aes_sc; IWLAGN_NUM_RSC],
    pub multicast_rsc: [aes_sc; IWLAGN_NUM_RSC],
    pub tsc: aes_sc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub union iwlagn_all_tsc_rsc {
    pub tkip: iwlagn_tkip_rsc_tsc,
    pub aes: iwlagn_aes_rsc_tsc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_wowlan_rsc_tsc_params_cmd {
    pub all_tsc_rsc: iwlagn_all_tsc_rsc,
    pub __packed: },
//
// REPLY_WOWLAN_TKIP_PARAMS
//
pub const IWLAGN_MIC_KEY_SIZE: c_int = 8;
pub const IWLAGN_P1K_SIZE: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_mic_keys {
    pub tx: [u8; IWLAGN_MIC_KEY_SIZE],
    pub rx_unicast: [u8; IWLAGN_MIC_KEY_SIZE],
    pub rx_mcast: [u8; IWLAGN_MIC_KEY_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_p1k_cache {
    pub p1k: [__le16; IWLAGN_P1K_SIZE],
    pub __packed: },
pub const IWLAGN_NUM_RX_P1K_CACHE: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_wowlan_tkip_params_cmd {
    pub mic_keys: iwlagn_mic_keys,
    pub tx: iwlagn_p1k_cache,
    pub rx_uni: [iwlagn_p1k_cache; IWLAGN_NUM_RX_P1K_CACHE],
    pub rx_multi: [iwlagn_p1k_cache; IWLAGN_NUM_RX_P1K_CACHE],
    pub __packed: },
//
// REPLY_WOWLAN_KEK_KCK_MATERIAL
//
pub const IWLAGN_KCK_MAX_SIZE: c_int = 32;
pub const IWLAGN_KEK_MAX_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_wowlan_kek_kck_material_cmd {
    pub kck: [u8; IWLAGN_KCK_MAX_SIZE],
    pub kek: [u8; IWLAGN_KEK_MAX_SIZE],
    pub kck_len: __le16,
    pub kek_len: __le16,
    pub replay_ctr: __le64,
    pub __packed: },
pub const RF_KILL_INDICATOR_FOR_WOWLAN: c_uint = 0x87;
//
// REPLY_WOWLAN_GET_STATUS = 0xe5
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwlagn_wowlan_status {
    pub replay_ctr: __le64,
    pub rekey_status: __le32,
    pub wakeup_reason: __le32,
    pub pattern_number: u8,
    pub reserved1: u8,
    pub qos_seq_ctr: [__le16; 8],
    pub non_qos_seq_ctr: __le16,
    pub reserved2: __le16,
    pub tsc_rsc: iwlagn_all_tsc_rsc,
    pub reserved3: __le16,
    pub __packed: },
//
// REPLY_WIPAN_PARAMS = 0xb2 (Commands and Notification)
//
// Minimum slot time in TU
//
pub const IWL_MIN_SLOT_TIME: c_int = 20;
//
// struct iwl_wipan_slot - WiPAN slot configuration
// @width: Time in TU
// @type:
// 0 - BSS
// 1 - PAN
// @reserved: reserved for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wipan_slot {
    pub width: __le16,
    pub type: u8,
    pub reserved: u8,
    pub __packed: },

//
// struct iwl_wipan_params_cmd - WiPAN parameters
// @flags:
// bit0: reserved
// bit1: CP leave channel with CTS
// bit2: CP leave channel qith Quiet
// bit3: slotted mode
// 1 - work in slotted mode
// 0 - work in non slotted mode
// bit4: filter beacon notification
// bit5: full tx slotted mode. if this flag is set,
// uCode will perform leaving channel methods in context switch
// also when working in same channel mode
// @num_slots: 1 - 10
// @slots: per-slot data
// @reserved: reserved for alignment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wipan_params_cmd {
    pub flags: __le16,
    pub reserved: u8,
    pub num_slots: u8,
    pub slots: [iwl_wipan_slot; 10],
    pub __packed: },
//
// REPLY_WIPAN_P2P_CHANNEL_SWITCH = 0xb9
//
// TODO: Figure out what this is used for,
// it can only switch between 2.4 GHz
// channels!!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wipan_p2p_channel_switch_cmd {
    pub channel: __le16,
    pub reserved: __le16,
}

//
// REPLY_WIPAN_NOA_NOTIFICATION = 0xbc
//
// This is used by the device to notify us of the
// NoA schedule it determined so we can forward it
// to userspace for inclusion in probe responses.
//
// In beacons, the NoA schedule is simply appended
// to the frame we give the device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wipan_noa_descriptor {
    pub count: u8,
    pub duration: __le32,
    pub interval: __le32,
    pub starttime: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wipan_noa_attribute {
    pub id: u8,
    pub length: __le16,
    pub index: u8,
    pub ct_window: u8,
    pub descr1: iwl_wipan_noa_descriptor descr0,,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_wipan_noa_notification {
    pub noa_active: u32,
    pub noa_attribute: iwl_wipan_noa_attribute,
    pub __packed: },
