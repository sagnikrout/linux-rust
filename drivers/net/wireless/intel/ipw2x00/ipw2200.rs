//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/ipw2x00/ipw2200.h
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

// Macro flag: #define __ipw2200_h__

// Authentication  and Association States
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum connection_manager_assoc_states {
    CMAS_INIT = 0,
    CMAS_TX_AUTH_SEQ_1,
    CMAS_RX_AUTH_SEQ_2,
    CMAS_AUTH_SEQ_1_PASS,
    CMAS_AUTH_SEQ_1_FAIL,
    CMAS_TX_AUTH_SEQ_3,
    CMAS_RX_AUTH_SEQ_4,
    CMAS_AUTH_SEQ_2_PASS,
    CMAS_AUTH_SEQ_2_FAIL,
    CMAS_AUTHENTICATED,
    CMAS_TX_ASSOC,
    CMAS_RX_ASSOC_RESP,
    CMAS_ASSOCIATED,
    CMAS_LAST
}

pub const IPW_POWER_MODE_CAM: c_uint = 0x00	//(always on);
pub const IPW_POWER_INDEX_1: c_uint = 0x01;
pub const IPW_POWER_INDEX_2: c_uint = 0x02;
pub const IPW_POWER_INDEX_3: c_uint = 0x03;
pub const IPW_POWER_INDEX_4: c_uint = 0x04;
pub const IPW_POWER_INDEX_5: c_uint = 0x05;
pub const IPW_POWER_AC: c_uint = 0x06;
pub const IPW_POWER_BATTERY: c_uint = 0x07;
pub const IPW_POWER_LIMIT: c_uint = 0x07;
pub const IPW_POWER_MASK: c_uint = 0x0F;
pub const IPW_POWER_ENABLED: c_uint = 0x10;

pub const IPW_CMD_HOST_COMPLETE: c_int = 2;
pub const IPW_CMD_POWER_DOWN: c_int = 4;
pub const IPW_CMD_SYSTEM_CONFIG: c_int = 6;
pub const IPW_CMD_MULTICAST_ADDRESS: c_int = 7;
pub const IPW_CMD_SSID: c_int = 8;
pub const IPW_CMD_ADAPTER_ADDRESS: c_int = 11;
pub const IPW_CMD_PORT_TYPE: c_int = 12;
pub const IPW_CMD_RTS_THRESHOLD: c_int = 15;
pub const IPW_CMD_FRAG_THRESHOLD: c_int = 16;
pub const IPW_CMD_POWER_MODE: c_int = 17;
pub const IPW_CMD_WEP_KEY: c_int = 18;
pub const IPW_CMD_TGI_TX_KEY: c_int = 19;
pub const IPW_CMD_SCAN_REQUEST: c_int = 20;
pub const IPW_CMD_ASSOCIATE: c_int = 21;
pub const IPW_CMD_SUPPORTED_RATES: c_int = 22;
pub const IPW_CMD_SCAN_ABORT: c_int = 23;
pub const IPW_CMD_TX_FLUSH: c_int = 24;
pub const IPW_CMD_QOS_PARAMETERS: c_int = 25;
pub const IPW_CMD_SCAN_REQUEST_EXT: c_int = 26;
pub const IPW_CMD_DINO_CONFIG: c_int = 30;
pub const IPW_CMD_RSN_CAPABILITIES: c_int = 31;
pub const IPW_CMD_RX_KEY: c_int = 32;
pub const IPW_CMD_CARD_DISABLE: c_int = 33;
pub const IPW_CMD_SEED_NUMBER: c_int = 34;
pub const IPW_CMD_TX_POWER: c_int = 35;
pub const IPW_CMD_COUNTRY_INFO: c_int = 36;
pub const IPW_CMD_AIRONET_INFO: c_int = 37;
pub const IPW_CMD_AP_TX_POWER: c_int = 38;
pub const IPW_CMD_CCKM_INFO: c_int = 39;
pub const IPW_CMD_CCX_VER_INFO: c_int = 40;
pub const IPW_CMD_SET_CALIBRATION: c_int = 41;
pub const IPW_CMD_SENSITIVITY_CALIB: c_int = 42;
pub const IPW_CMD_RETRY_LIMIT: c_int = 51;
pub const IPW_CMD_IPW_PRE_POWER_DOWN: c_int = 58;
pub const IPW_CMD_VAP_BEACON_TEMPLATE: c_int = 60;
pub const IPW_CMD_VAP_DTIM_PERIOD: c_int = 61;
pub const IPW_CMD_EXT_SUPPORTED_RATES: c_int = 62;
pub const IPW_CMD_VAP_LOCAL_TX_PWR_CONSTRAINT: c_int = 63;
pub const IPW_CMD_VAP_QUIET_INTERVALS: c_int = 64;
pub const IPW_CMD_VAP_CHANNEL_SWITCH: c_int = 65;
pub const IPW_CMD_VAP_MANDATORY_CHANNELS: c_int = 66;
pub const IPW_CMD_VAP_CELL_PWR_LIMIT: c_int = 67;
pub const IPW_CMD_VAP_CF_PARAM_SET: c_int = 68;
pub const IPW_CMD_VAP_SET_BEACONING_STATE: c_int = 69;
pub const IPW_CMD_MEASUREMENT: c_int = 80;
pub const IPW_CMD_POWER_CAPABILITY: c_int = 81;
pub const IPW_CMD_SUPPORTED_CHANNELS: c_int = 82;
pub const IPW_CMD_TPC_REPORT: c_int = 83;
pub const IPW_CMD_WME_INFO: c_int = 84;
pub const IPW_CMD_PRODUCTION_COMMAND: c_int = 85;
pub const IPW_CMD_LINKSYS_EOU_INFO: c_int = 90;
pub const RFD_SIZE: c_int = 4;
pub const NUM_TFD_CHUNKS: c_int = 6;
pub const TX_QUEUE_SIZE: c_int = 32;
pub const RX_QUEUE_SIZE: c_int = 32;
pub const DINO_CMD_WEP_KEY: c_uint = 0x08;
pub const DINO_CMD_TX: c_uint = 0x0B;
pub const DCT_ANTENNA_A: c_uint = 0x01;
pub const DCT_ANTENNA_B: c_uint = 0x02;
pub const IPW_A_MODE: c_int = 0;
pub const IPW_B_MODE: c_int = 1;
pub const IPW_G_MODE: c_int = 2;
//
// TX Queue Flag Definitions
//
// tx wep key definition
pub const DCT_WEP_KEY_NOT_IMMIDIATE: c_uint = 0x00;
pub const DCT_WEP_KEY_64Bit: c_uint = 0x40;
pub const DCT_WEP_KEY_128Bit: c_uint = 0x80;
pub const DCT_WEP_KEY_128bitIV: c_uint = 0xC0;
pub const DCT_WEP_KEY_SIZE_MASK: c_uint = 0xC0;
pub const DCT_WEP_KEY_INDEX_MASK: c_uint = 0x0F;
pub const DCT_WEP_INDEX_USE_IMMEDIATE: c_uint = 0x20;
// abort attempt if mgmt frame is rx'd
pub const DCT_FLAG_ABORT_MGMT: c_uint = 0x01;
// require CTS
pub const DCT_FLAG_CTS_REQUIRED: c_uint = 0x02;
// use short preamble
pub const DCT_FLAG_LONG_PREAMBLE: c_uint = 0x00;
pub const DCT_FLAG_SHORT_PREAMBLE: c_uint = 0x04;
// RTS/CTS first
pub const DCT_FLAG_RTS_REQD: c_uint = 0x08;
// dont calculate duration field
pub const DCT_FLAG_DUR_SET: c_uint = 0x10;
// even if MAC WEP set (allows pre-encrypt)
pub const DCT_FLAG_NO_WEP: c_uint = 0x20;
// overwrite TSF field
pub const DCT_FLAG_TSF_REQD: c_uint = 0x40;
// ACK rx is expected to follow
pub const DCT_FLAG_ACK_REQD: c_uint = 0x80;
// TX flags extension
pub const DCT_FLAG_EXT_MODE_CCK: c_uint = 0x01;
pub const DCT_FLAG_EXT_MODE_OFDM: c_uint = 0x00;
pub const DCT_FLAG_EXT_SECURITY_WEP: c_uint = 0x00;

pub const DCT_FLAG_EXT_SECURITY_CKIP: c_uint = 0x04;
pub const DCT_FLAG_EXT_SECURITY_CCM: c_uint = 0x08;
pub const DCT_FLAG_EXT_SECURITY_TKIP: c_uint = 0x0C;
pub const DCT_FLAG_EXT_SECURITY_MASK: c_uint = 0x0C;
pub const DCT_FLAG_EXT_QOS_ENABLED: c_uint = 0x10;
pub const DCT_FLAG_EXT_HC_NO_SIFS_PIFS: c_uint = 0x00;
pub const DCT_FLAG_EXT_HC_SIFS: c_uint = 0x20;
pub const DCT_FLAG_EXT_HC_PIFS: c_uint = 0x40;
pub const TX_RX_TYPE_MASK: c_uint = 0xFF;
pub const TX_FRAME_TYPE: c_uint = 0x00;
pub const TX_HOST_COMMAND_TYPE: c_uint = 0x01;
pub const RX_FRAME_TYPE: c_uint = 0x09;
pub const RX_HOST_NOTIFICATION_TYPE: c_uint = 0x03;
pub const RX_HOST_CMD_RESPONSE_TYPE: c_uint = 0x04;
pub const RX_TX_FRAME_RESPONSE_TYPE: c_uint = 0x05;
pub const TFD_NEED_IRQ_MASK: c_uint = 0x04;
pub const HOST_CMD_DINO_CONFIG: c_int = 30;
pub const HOST_NOTIFICATION_STATUS_ASSOCIATED: c_int = 10;
pub const HOST_NOTIFICATION_STATUS_AUTHENTICATE: c_int = 11;
pub const HOST_NOTIFICATION_STATUS_SCAN_CHANNEL_RESULT: c_int = 12;
pub const HOST_NOTIFICATION_STATUS_SCAN_COMPLETED: c_int = 13;
pub const HOST_NOTIFICATION_STATUS_FRAG_LENGTH: c_int = 14;
pub const HOST_NOTIFICATION_STATUS_LINK_DETERIORATION: c_int = 15;
pub const HOST_NOTIFICATION_DINO_CONFIG_RESPONSE: c_int = 16;
pub const HOST_NOTIFICATION_STATUS_BEACON_STATE: c_int = 17;
pub const HOST_NOTIFICATION_STATUS_TGI_TX_KEY: c_int = 18;
pub const HOST_NOTIFICATION_TX_STATUS: c_int = 19;
pub const HOST_NOTIFICATION_CALIB_KEEP_RESULTS: c_int = 20;
pub const HOST_NOTIFICATION_MEASUREMENT_STARTED: c_int = 21;
pub const HOST_NOTIFICATION_MEASUREMENT_ENDED: c_int = 22;
pub const HOST_NOTIFICATION_CHANNEL_SWITCHED: c_int = 23;
pub const HOST_NOTIFICATION_RX_DURING_QUIET_PERIOD: c_int = 24;
pub const HOST_NOTIFICATION_NOISE_STATS: c_int = 25;
pub const HOST_NOTIFICATION_S36_MEASUREMENT_ACCEPTED: c_int = 30;
pub const HOST_NOTIFICATION_S36_MEASUREMENT_REFUSED: c_int = 31;
pub const HOST_NOTIFICATION_STATUS_BEACON_MISSING: c_int = 1;
pub const IPW_MB_SCAN_CANCEL_THRESHOLD: c_int = 3;
pub const IPW_MB_ROAMING_THRESHOLD_MIN: c_int = 1;
pub const IPW_MB_ROAMING_THRESHOLD_DEFAULT: c_int = 8;
pub const IPW_MB_ROAMING_THRESHOLD_MAX: c_int = 30;

pub const IPW_REAL_RATE_RX_PACKET_THRESHOLD: c_int = 300;
pub const MACADRR_BYTE_LEN: c_int = 6;
pub const DCR_TYPE_AP: c_uint = 0x01;
pub const DCR_TYPE_WLAP: c_uint = 0x02;
pub const DCR_TYPE_MU_ESS: c_uint = 0x03;
pub const DCR_TYPE_MU_IBSS: c_uint = 0x04;
pub const DCR_TYPE_MU_PIBSS: c_uint = 0x05;
pub const DCR_TYPE_SNIFFER: c_uint = 0x06;

// QoS  definitions
pub const CW_MIN_OFDM: c_int = 15;
pub const CW_MAX_OFDM: c_int = 1023;
pub const CW_MIN_CCK: c_int = 31;
pub const CW_MAX_CCK: c_int = 1023;

pub const QOS_TX0_ACM: c_int = 0;
pub const QOS_TX1_ACM: c_int = 0;
pub const QOS_TX2_ACM: c_int = 0;
pub const QOS_TX3_ACM: c_int = 0;
pub const QOS_TX0_TXOP_LIMIT_CCK: c_int = 0;
pub const QOS_TX1_TXOP_LIMIT_CCK: c_int = 0;

pub const QOS_TX0_TXOP_LIMIT_OFDM: c_int = 0;
pub const QOS_TX1_TXOP_LIMIT_OFDM: c_int = 0;

pub const DEF_TX0_AIFS: c_int = 0;
pub const DEF_TX1_AIFS: c_int = 0;
pub const DEF_TX2_AIFS: c_int = 0;
pub const DEF_TX3_AIFS: c_int = 0;
pub const DEF_TX0_ACM: c_int = 0;
pub const DEF_TX1_ACM: c_int = 0;
pub const DEF_TX2_ACM: c_int = 0;
pub const DEF_TX3_ACM: c_int = 0;
pub const DEF_TX0_TXOP_LIMIT_CCK: c_int = 0;
pub const DEF_TX1_TXOP_LIMIT_CCK: c_int = 0;
pub const DEF_TX2_TXOP_LIMIT_CCK: c_int = 0;
pub const DEF_TX3_TXOP_LIMIT_CCK: c_int = 0;
pub const DEF_TX0_TXOP_LIMIT_OFDM: c_int = 0;
pub const DEF_TX1_TXOP_LIMIT_OFDM: c_int = 0;
pub const DEF_TX2_TXOP_LIMIT_OFDM: c_int = 0;
pub const DEF_TX3_TXOP_LIMIT_OFDM: c_int = 0;
pub const QOS_QOS_SETS: c_int = 3;
pub const QOS_PARAM_SET_ACTIVE: c_int = 0;
pub const QOS_PARAM_SET_DEF_CCK: c_int = 1;
pub const QOS_PARAM_SET_DEF_OFDM: c_int = 2;

pub const IPW_TX_QUEUE_1: c_int = 1;
pub const IPW_TX_QUEUE_2: c_int = 2;
pub const IPW_TX_QUEUE_3: c_int = 3;
pub const IPW_TX_QUEUE_4: c_int = 4;
// QoS sturctures
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_qos_info {
    pub qos_enable: c_int,
    pub def_qos_parm_OFDM: *mut libipw_qos_parameters,
    pub def_qos_parm_CCK: *mut libipw_qos_parameters,
    pub burst_duration_CCK: u32,
    pub burst_duration_OFDM: u32,
    pub qos_no_ack_mask: u16,
    pub burst_enable: c_int,
}

//
// Generic queue structure
//
// Contains common data for Rx and Tx queues
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clx2_queue {
    pub /: *mut *mut *mut int n_bd; /< number of BDs in this queue,
    pub /: *mut *mut *mut int first_empty; /< 1-st empty entry (index),
    pub /: *mut *mut *mut int last_used; /< last used entry (index),
    pub /: *mut *mut *mut u32 reg_w; /< 'write' reg (queue head), addr in domain 1,
    pub /: *mut *mut *mut u32 reg_r; /< 'read' reg (queue tail), addr in domain 1,
    pub /: *mut *mut *mut dma_addr_t dma_addr; /< physical addr for BD's,
    pub /: *mut *mut *mut int low_mark; /< low watermark, resume queue if free space more than this,
    pub /: *mut *mut *mut int high_mark; /< high watermark, stop queue if free space less than this,
    pub /: *mut *mut } __packed; / XXX,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct machdr32 {
    pub frame_ctl: __le16,
    pub endians!: __le16 duration; // watch out for,
    pub addr1: [u8; MACADRR_BYTE_LEN],
    pub addr2: [u8; MACADRR_BYTE_LEN],
    pub addr3: [u8; MACADRR_BYTE_LEN],
    pub endians!: __le16 seq_ctrl; // more,
    pub addr4: [u8; MACADRR_BYTE_LEN],
    pub qos_ctrl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct machdr30 {
    pub frame_ctl: __le16,
    pub endians!: __le16 duration; // watch out for,
    pub addr1: [u8; MACADRR_BYTE_LEN],
    pub addr2: [u8; MACADRR_BYTE_LEN],
    pub addr3: [u8; MACADRR_BYTE_LEN],
    pub endians!: __le16 seq_ctrl; // more,
    pub addr4: [u8; MACADRR_BYTE_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct machdr26 {
    pub frame_ctl: __le16,
    pub endians!: __le16 duration; // watch out for,
    pub addr1: [u8; MACADRR_BYTE_LEN],
    pub addr2: [u8; MACADRR_BYTE_LEN],
    pub addr3: [u8; MACADRR_BYTE_LEN],
    pub endians!: __le16 seq_ctrl; // more,
    pub qos_ctrl: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct machdr24 {
    pub frame_ctl: __le16,
    pub endians!: __le16 duration; // watch out for,
    pub addr1: [u8; MACADRR_BYTE_LEN],
    pub addr2: [u8; MACADRR_BYTE_LEN],
    pub addr3: [u8; MACADRR_BYTE_LEN],
    pub endians!: __le16 seq_ctrl; // more,
    pub __packed: },
// TX TFD with 32 byte MAC Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_tfd_32 {
    pub 32: machdr32 mchdr; //,
    pub 8: __le32 uivplaceholder[2]; //,
    pub __packed: },
// TX TFD with 30 byte MAC Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_tfd_30 {
    pub 30: machdr30 mchdr; //,
    pub 2: u8 reserved[2]; //,
    pub 8: __le32 uivplaceholder[2]; //,
    pub __packed: },
// tx tfd with 26 byte mac header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_tfd_26 {
    pub 26: machdr26 mchdr; //,
    pub 2: u8 reserved1[2]; //,
    pub 8: __le32 uivplaceholder[2]; //,
    pub 4: u8 reserved2[4]; //,
    pub __packed: },
// tx tfd with 24 byte mac header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tx_tfd_24 {
    pub 24: machdr24 mchdr; //,
    pub 8: __le32 uivplaceholder[2]; //,
    pub 8: u8 reserved[8]; //,
    pub __packed: },
pub const DCT_WEP_KEY_FIELD_LENGTH: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tfd_command {
    pub index: u8,
    pub length: u8,
    pub reserved: __le16,
    pub payload: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tfd_data {
// Header
    pub work_area_ptr: __le32,
    pub /: *mut *mut u8 station_number; / 0 for BSS,
    pub reserved1: u8,
    pub reserved2: __le16,
// Tx Parameters
    pub cmd_id: u8,
    pub seq_num: u8,
    pub len: __le16,
    pub priority: u8,
    pub tx_flags: u8,
    pub tx_flags_ext: u8,
    pub key_index: u8,
    pub wepkey: [u8; DCT_WEP_KEY_FIELD_LENGTH],
    pub rate: u8,
    pub antenna: u8,
    pub next_packet_duration: __le16,
    pub next_frag_len: __le16,
    pub //////txop: __le16 back_off_counter;,
    pub retrylimit: u8,
    pub cwcurrent: __le16,
    pub reserved3: u8,
// 802.11 MAC Header
    pub tfd_24: tx_tfd_24,
    pub tfd_26: tx_tfd_26,
    pub tfd_30: tx_tfd_30,
    pub tfd_32: tx_tfd_32,
    pub tfd: },
// Payload DMA info
    pub num_chunks: __le32,
    pub chunk_ptr: [__le32; NUM_TFD_CHUNKS],
    pub chunk_len: [__le16; NUM_TFD_CHUNKS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct txrx_control_flags {
    pub message_type: u8,
    pub rx_seq_num: u8,
    pub control_bits: u8,
    pub reserved: u8,
    pub __packed: },
pub const TFD_SIZE: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tfd_frame {
    pub control_flags: txrx_control_flags,
    pub data: tfd_data,
    pub cmd: tfd_command,
    pub raw: [u8; TFD_CMD_IMMEDIATE_PAYLOAD_LENGTH],
    pub u: },
    pub __packed: },
    pub ): *const typedef void destructor_func(void,
//
// Tx Queue for DMA. Queue consists of circular buffer of
// BD's and required locking structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct clx2_tx_queue {
    pub q: clx2_queue,
    pub bd: *mut tfd_frame,
    pub txb: *mut libipw_txb,
}

//
// RX related structures and functions
//
pub const RX_FREE_BUFFERS: c_int = 32;
pub const RX_LOW_WATERMARK: c_int = 8;
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
pub struct ipw_cmd_stats {
    pub cmd_id: u8,
    pub seq_num: u8,
    pub good_sfd: __le16,
    pub bad_plcp: __le16,
    pub wrong_bssid: __le16,
    pub valid_mpdu: __le16,
    pub bad_mac_header: __le16,
    pub reserved_frame_types: __le16,
    pub rx_ina: __le16,
    pub bad_crc32: __le16,
    pub invalid_cts: __le16,
    pub invalid_acks: __le16,
    pub long_distance_ina_fina: __le16,
    pub dsp_silence_unreachable: __le16,
    pub accumulated_rssi: __le16,
    pub rx_ovfl_frame_tossed: __le16,
    pub rssi_silence_threshold: __le16,
    pub rx_ovfl_frame_supplied: __le16,
    pub last_rx_frame_signal: __le16,
    pub last_rx_frame_noise: __le16,
    pub rx_autodetec_no_ofdm: __le16,
    pub rx_autodetec_no_barker: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_channel_result {
    pub channel_num: u8,
    pub stats: ipw_cmd_stats,
    pub uReserved: u8,
    pub __packed: },
pub const SCAN_COMPLETED_STATUS_COMPLETE: c_int = 1;
pub const SCAN_COMPLETED_STATUS_ABORTED: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_scan_complete {
    pub scan_type: u8,
    pub num_channels: u8,
    pub status: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_frag_length {
    pub frag_length: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_beacon_state {
    pub state: __le32,
    pub number: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_tgi_tx_key {
    pub key_state: u8,
    pub security_type: u8,
    pub station_index: u8,
    pub reserved: u8,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_link_deterioration {
    pub stats: ipw_cmd_stats,
    pub rate: u8,
    pub modulation: u8,
    pub histogram: rate_histogram,
    pub /: *mut *mut u8 silence_notification_type; / SILENCE_OVER/UNDER_THRESH,
    pub silence_count: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_association {
    pub state: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_authenticate {
    pub state: u8,
    pub addr: machdr24,
    pub status: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_calibration {
    pub data: [u8; 104],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct notif_noise {
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rx_notification {
    pub reserved: [u8; 8],
    pub subtype: u8,
    pub flags: u8,
    pub size: __le16,
    pub assoc: notif_association,
    pub auth: notif_authenticate,
    pub channel_result: notif_channel_result,
    pub scan_complete: notif_scan_complete,
    pub frag_len: notif_frag_length,
    pub beacon_state: notif_beacon_state,
    pub tgi_tx_key: notif_tgi_tx_key,
    pub link_deterioration: notif_link_deterioration,
    pub calibration: notif_calibration,
    pub noise: notif_noise,
    pub raw): DECLARE_FLEX_ARRAY(u8,,
    pub u: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rx_frame {
    pub reserved1: __le32,
    pub OUR_TSF_IS_GREATER: u8 parent_tsf[4]; // fw_use[0] is boolean for,
    pub on.: u8 received_channel; // The channel that this frame was received,
// Note that for .11b this does not have to be
// the same as the channel that it was sent.
// Filled by LMAC
    pub frameStatus: u8,
    pub rate: u8,
    pub rssi: u8,
    pub agc: u8,
    pub rssi_dbm: u8,
    pub signal: __le16,
    pub noise: __le16,
    pub antennaAndPhy: u8,
    pub bg: u8 control; // control bit should be on in,
    pub rate: u8 rtscts_rate; // rate of rts or cts (in rts cts sequence,
// is identical)
    pub seen: u8 rtscts_seen; // 0x1 RTS seen ; 0x2 CTS,
    pub length: __le16,
    pub data: [u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rx_header {
    pub message_type: u8,
    pub rx_seq_num: u8,
    pub control_bits: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rx_packet {
    pub header: ipw_rx_header,
    pub frame: ipw_rx_frame,
    pub notification: ipw_rx_notification,
    pub u: },
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rx_mem_buffer {
    pub dma_addr: dma_addr_t,
    pub skb: *mut sk_buff,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rx_queue {
    pub RX_FREE_BUFFERS]: ipw_rx_mem_buffer pool[RX_QUEUE_SIZE +,
    pub queue: [*mut ipw_rx_mem_buffer; RX_QUEUE_SIZE],
    pub /: *mut *mut u32 processed; / Internal index to last handled Rx packet,
    pub /: *mut *mut u32 read; / Shared index to newest available Rx buffer,
    pub /: *mut *mut u32 write; / Shared index to oldest written Rx packet,
    pub /: *mut *mut u32 free_count; / Number of pre-allocated buffers in rx_free,
// Each of these lists is used as a FIFO for ipw_rx_mem_buffers
    pub /: *mut *mut list_head rx_free; / Own an SKBs,
    pub /: *mut *mut list_head rx_used; / No SKB allocated,
    pub lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alive_command_responce {
    pub alive_command: u8,
    pub sequence_number: u8,
    pub software_revision: __le16,
    pub device_identifier: u8,
    pub reserved1: [u8; 5],
    pub reserved2: __le16,
    pub reserved3: __le16,
    pub clock_settle_time: __le16,
    pub powerup_settle_time: __le16,
    pub reserved4: __le16,
    pub /: *mut *mut u8 time_stamp[5]; / month, day, year, hours, minutes,
    pub ucode_valid: u8,
    pub __packed: },
pub const IPW_MAX_RATES: c_int = 12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rates {
    pub num_rates: u8,
    pub rates: [u8; IPW_MAX_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct command_block {
    pub control: c_uint,
    pub source_addr: u32,
    pub dest_addr: u32,
    pub status: c_uint,
    pub __packed: },
pub const CB_NUMBER_OF_ELEMENTS_SMALL: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_image_desc {
    pub last_cb_index: c_ulong,
    pub current_cb_index: c_ulong,
    pub cb_list: [command_block; CB_NUMBER_OF_ELEMENTS_SMALL],
    pub v_addr: *mut c_void,
    pub p_addr: c_ulong,
    pub len: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_sys_config {
    pub bt_coexistence: u8,
    pub reserved1: u8,
    pub answer_broadcast_ssid_probe: u8,
    pub accept_all_data_frames: u8,
    pub accept_non_directed_frames: u8,
    pub exclude_unicast_unencrypted: u8,
    pub disable_unicast_decryption: u8,
    pub exclude_multicast_unencrypted: u8,
    pub disable_multicast_decryption: u8,
    pub antenna_diversity: u8,
    pub pass_crc_to_host: u8,
    pub dot11g_auto_detection: u8,
    pub enable_cts_to_self: u8,
    pub enable_multicast_filtering: u8,
    pub bt_coexist_collision_thr: u8,
    pub silence_threshold: u8,
    pub accept_all_mgmt_bcpr: u8,
    pub accept_all_mgmt_frames: u8,
    pub pass_noise_stats_to_host: u8,
    pub reserved3: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_multicast_addr {
    pub num_of_multicast_addresses: u8,
    pub reserved: [u8; 3],
    pub mac1: [u8; 6],
    pub mac2: [u8; 6],
    pub mac3: [u8; 6],
    pub mac4: [u8; 6],
    pub __packed: },
pub const DCW_WEP_KEY_INDEX_MASK: c_uint = 0x03	/* bits [0:1] */;
pub const DCW_WEP_KEY_SEC_TYPE_MASK: c_uint = 0x30	/* bits [4:5] */;
pub const DCW_WEP_KEY_SEC_TYPE_WEP: c_uint = 0x00;
pub const DCW_WEP_KEY_SEC_TYPE_CCM: c_uint = 0x20;
pub const DCW_WEP_KEY_SEC_TYPE_TKIP: c_uint = 0x30;
pub const DCW_WEP_KEY_INVALID_SIZE: c_uint = 0x00	/* 0 = Invalid key */;
pub const DCW_WEP_KEY64Bit_SIZE: c_uint = 0x05	/* 64-bit encryption */;
pub const DCW_WEP_KEY128Bit_SIZE: c_uint = 0x0D	/* 128-bit encryption */;
pub const DCW_CCM_KEY128Bit_SIZE: c_uint = 0x10	/* 128-bit key */;
// #define DCW_WEP_KEY128BitIV_SIZE      0x10    /* 128-bit key and 128-bit IV
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_wep_key {
    pub cmd_id: u8,
    pub seq_num: u8,
    pub key_index: u8,
    pub key_size: u8,
    pub key: [u8; 16],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_tgi_tx_key {
    pub key_id: u8,
    pub security_type: u8,
    pub station_index: u8,
    pub flags: u8,
    pub key: [u8; 16],
    pub tx_counter: [__le32; 2],
    pub __packed: },
pub const IPW_SCAN_CHANNELS: c_int = 54;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_scan_request {
    pub scan_type: u8,
    pub dwell_time: __le16,
    pub channels_list: [u8; IPW_SCAN_CHANNELS],
    pub channels_reserved: [u8; 3],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_scan_request_ext {
    pub full_scan_index: __le32,
    pub channels_list: [u8; IPW_SCAN_CHANNELS],
    pub 2]: u8 scan_type[IPW_SCAN_CHANNELS /,
    pub reserved: u8,
    pub dwell_time: [__le16; IPW_SCAN_TYPES],
    pub __packed: },
    pub 0x0F: return scan->scan_type[index / 2] &,
    pub 4: return (scan->scan_type[index / 2] & 0xF0) >>,
    pub 0x0F): (scan->scan_type[index / 2] & 0xF0) | (scan_type &,
    pub 4): ((scan_type & 0x0F) <<,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_associate {
    pub channel: u8,

    pub auth_key:4: u8 auth_type:4,,

    pub auth_type:4: u8 auth_key:4,,

    pub assoc_type: u8,
    pub reserved: u8,
    pub policy_support: __le16,
    pub preamble_length: u8,
    pub ieee_mode: u8,
    pub bssid: [u8; ETH_ALEN],
    pub assoc_tsf_msw: __le32,
    pub assoc_tsf_lsw: __le32,
    pub capability: __le16,
    pub listen_interval: __le16,
    pub beacon_interval: __le16,
    pub dest: [u8; ETH_ALEN],
    pub atim_window: __le16,
    pub smr: u8,
    pub reserved1: u8,
    pub reserved2: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_supported_rates {
    pub ieee_mode: u8,
    pub num_rates: u8,
    pub purpose: u8,
    pub reserved: u8,
    pub supported_rates: [u8; IPW_MAX_RATES],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rts_threshold {
    pub rts_threshold: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_frag_threshold {
    pub frag_threshold: __le16,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_retry_limit {
    pub short_retry_limit: u8,
    pub long_retry_limit: u8,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_dino_config {
    pub dino_config_addr: __le32,
    pub dino_config_size: __le16,
    pub dino_response: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_aironet_info {
    pub id: u8,
    pub length: u8,
    pub reserved: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rx_key {
    pub station_index: u8,
    pub key_type: u8,
    pub key_id: u8,
    pub key_flag: u8,
    pub key: [u8; 16],
    pub station_address: [u8; 6],
    pub key_index: u8,
    pub reserved: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_country_channel_info {
    pub first_channel: u8,
    pub no_channels: u8,
    pub max_tx_power: i8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_country_info {
    pub id: u8,
    pub length: u8,
    pub country_str: [u8; IEEE80211_COUNTRY_STRING_LEN],
    pub groups: [ipw_country_channel_info; 7],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_channel_tx_power {
    pub channel_number: u8,
    pub tx_power: i8,
    pub __packed: },

pub const MAX_A_CHANNELS: c_int = 37;
pub const MAX_B_CHANNELS: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_tx_power {
    pub num_channels: u8,
    pub ieee_mode: u8,
    pub channels_tx_power: [ipw_channel_tx_power; MAX_A_CHANNELS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rsn_capabilities {
    pub id: u8,
    pub length: u8,
    pub version: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_sensitivity_calib {
    pub beacon_rssi_raw: __le16,
    pub reserved: __le16,
    pub __packed: },
//
// Host command structure.
//
// On input, the following fields should be filled:
// - cmd
// - len
// - status_len
// - param (if needed)
//
// On output,
// - \a status contains status;
// - \a param filled with status parameters.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_cmd {
    pub /: *mut *mut *mut u32 cmd; /< Host command,
    pub /: *mut *mut *mut u32 status;/< Status,
    pub status_len: u32,
// < How many 32 bit parameters in the status
    pub /: *mut *mut *mut u32 len; /< incoming parameters length, bytes,
//
// command parameters.
// There should be enough space for incoming and
// outcoming parameters.
// Incoming parameters listed 1-st, followed by outcoming params.
// nParams=(len+3)/4+status_len
//
    pub param: [u32; ],
    pub __packed: },

pub const MAX_STATIONS: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_station_entry {
    pub mac_addr: [u8; ETH_ALEN],
    pub reserved: u8,
    pub support_mode: u8,
}

pub const AVG_ENTRIES: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct average {
    pub entries: [i16; AVG_ENTRIES],
    pub pos: u8,
    pub init: u8,
    pub sum: i32,
}

pub const MAX_SPEED_SCAN: c_int = 100;
pub const IPW_IBSS_MAC_HASH_SIZE: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_ibss_seq {
    pub mac: [u8; ETH_ALEN],
    pub seq_num: u16,
    pub frag_num: u16,
    pub packet_time: c_ulong,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_error_elem {
    pub desc: u32,
    pub time: u32,
    pub blink1: u32,
    pub blink2: u32,
    pub link1: u32,
    pub link2: u32,
    pub data: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_event {
    pub event: u32,
    pub time: u32,
    pub data: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_fw_error {
    pub jiffies: c_ulong,
    pub status: u32,
    pub config: u32,
    pub elem_len: u32,
    pub log_len: u32,
    pub log: *mut ipw_event,
    pub elem: [ipw_error_elem; ],
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipw_prom_filter {
    IPW_PROM_CTL_HEADER_ONLY = (1 << 0),
    IPW_PROM_MGMT_HEADER_ONLY = (1 << 1),
    IPW_PROM_DATA_HEADER_ONLY = (1 << 2),
    IPW_PROM_ALL_HEADER_ONLY = 0xf, /* bits 0..3 */
    IPW_PROM_NO_TX = (1 << 4),
    IPW_PROM_NO_RX = (1 << 5),
    IPW_PROM_NO_CTL = (1 << 6),
    IPW_PROM_NO_MGMT = (1 << 7),
    IPW_PROM_NO_DATA = (1 << 8),
}

    pub ipw_priv: struct,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_prom_priv {
    pub priv: *mut ipw_priv,
    pub ieee: *mut libipw_device,
    pub filter: ipw_prom_filter,
    pub tx_packets: c_int,
    pub rx_packets: c_int,
}

// Magic struct that slots into the radiotap header -- no reason
// to build this manually element by element, we can write it much
// more efficiently than we can parse it. ORDER MATTERS HERE
//
// When sent to us via the simulated Rx interface in sysfs, the entire
// structure is provided regardless of any bits unset.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_rt_hdr {
    pub rt_hdr: ieee80211_radiotap_header_fixed,
    pub /: *mut *mut *mut *mut u64 rt_tsf; / TSF / / XXX,
    pub /: *mut *mut u8 rt_flags; / radiotap packet flags,
    pub /: *mut *mut u8 rt_rate; / rate in 500kb/s,
    pub /: *mut *mut __le16 rt_channel; / channel in mhz,
    pub /: *mut *mut __le16 rt_chbitmask; / channel bitfield,
    pub /: *mut *mut s8 rt_dbmsignal; / signal in dbM, kluged to signed,
    pub rt_dbmnoise: i8,
    pub /: *mut *mut u8 rt_antenna; / antenna number,
    pub /: *mut *mut u8 payload[]; / payload...,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_priv {
// ieee device used by generic ieee processing code
    pub ieee: *mut libipw_device,
    pub lock: spinlock_t,
    pub irq_lock: spinlock_t,
    pub mutex: mutex,
// basic pci-network driver stuff
    pub pci_dev: *mut pci_dev,
    pub net_dev: *mut net_device,

// Promiscuous mode
    pub prom_priv: *mut ipw_prom_priv,
    pub prom_net_dev: *mut net_device,

// pci hardware address support
    pub hw_base: *mut void __iomem,
    pub hw_len: c_ulong,
    pub sram_desc: fw_image_desc,
// result of ucode download
    pub dino_alive: alive_command_responce,
    pub wait_command_queue: wait_queue_head_t,
    pub wait_state: wait_queue_head_t,
// Rx and Tx DMA processing queues
    pub rxq: *mut ipw_rx_queue,
    pub txq_cmd: clx2_tx_queue,
    pub txq: [clx2_tx_queue; 4],
    pub status: u32,
    pub config: u32,
    pub capability: u32,
    pub average_missed_beacons: average,
    pub exp_avg_rssi: i16,
    pub exp_avg_noise: i16,
    pub port_type: u32,
    pub /: *mut *mut *mut int rx_bufs_min; /< minimum number of bufs in Rx queue,
    pub /: *mut *mut *mut int rx_pend_max; /< maximum pending buffers for one IRQ,
    pub /: *mut *mut *mut u32 hcmd_seq; /< sequence number for hcmd,
    pub disassociate_threshold: u32,
    pub roaming_threshold: u32,
    pub assoc_request: ipw_associate,
    pub assoc_network: *mut libipw_network,
    pub ts_scan_abort: c_ulong,
    pub rates: ipw_supported_rates,
    pub /: *mut *mut *mut ipw_rates phy[3]; /< PHY restrictions, per band,
    pub /: *mut *mut *mut ipw_rates supp; /< software defined,
    pub /: *mut *mut *mut ipw_rates extended; /< use for corresp. IE, AP only,
    pub /: *mut *mut *mut notif_link_deterioration last_link_deterioration; / for statistics,
    pub /: *mut *mut *mut *mut ipw_cmd hcmd; /< host command currently executed,
    pub /: *mut *mut *mut wait_queue_head_t hcmd_wq; /< host command waits for execution,
    pub /: *mut *mut *mut u32 tsf_bcn[2]; /< TSF from latest beacon,
    pub /: *mut *mut *mut notif_calibration calib; /< last calibration,
// ordinal interface with firmware
    pub table0_addr: u32,
    pub table0_len: u32,
    pub table1_addr: u32,
    pub table1_len: u32,
    pub table2_addr: u32,
    pub table2_len: u32,
// context information
    pub essid: [u8; IW_ESSID_MAX_SIZE],
    pub essid_len: u8,
    pub nick: [u8; IW_ESSID_MAX_SIZE],
    pub rates_mask: u16,
    pub channel: u8,
    pub sys_config: ipw_sys_config,
    pub power_mode: u32,
    pub bssid: [u8; ETH_ALEN],
    pub rts_threshold: u16,
    pub mac_addr: [u8; ETH_ALEN],
    pub num_stations: u8,
    pub stations: [u8; MAX_STATIONS][ETH_ALEN],
    pub short_retry_limit: u8,
    pub long_retry_limit: u8,
    pub notif_missed_beacons: u32,
// Statistics and counters normalized with each association
    pub last_missed_beacons: u32,
    pub last_tx_packets: u32,
    pub last_rx_packets: u32,
    pub last_tx_failures: u32,
    pub last_rx_err: u32,
    pub last_rate: u32,
    pub missed_adhoc_beacons: u32,
    pub missed_beacons: u32,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub quality: u32,
    pub speed_scan: [u8; MAX_SPEED_SCAN],
    pub speed_scan_pos: u8,
    pub last_seq_num: u16,
    pub last_frag_num: u16,
    pub last_packet_time: c_ulong,
    pub ibss_mac_hash: [list_head; IPW_IBSS_MAC_HASH_SIZE],
// eeprom
    pub /: *mut *mut u8 eeprom[0x100]; / 256 bytes of eeprom,
    pub country: [u8; 4],
    pub eeprom_delay: c_int,
    pub wstats: iw_statistics,
    pub user_requested_scan: c_int,
    pub direct_scan_ssid: [u8; IW_ESSID_MAX_SIZE],
    pub direct_scan_ssid_len: u8,
    pub adhoc_check: delayed_work,
    pub associate: work_struct,
    pub disassociate: work_struct,
    pub system_config: work_struct,
    pub rx_replenish: work_struct,
    pub request_scan: delayed_work,
    pub request_direct_scan: delayed_work,
    pub request_passive_scan: delayed_work,
    pub scan_event: delayed_work,
    pub adapter_restart: work_struct,
    pub rf_kill: delayed_work,
    pub up: work_struct,
    pub down: work_struct,
    pub gather_stats: delayed_work,
    pub abort_scan: work_struct,
    pub roam: work_struct,
    pub scan_check: delayed_work,
    pub link_up: work_struct,
    pub link_down: work_struct,
    pub irq_tasklet: tasklet_struct,
// LED related variables and work_struct
    pub nic_type: u8,
    pub led_activity_on: u32,
    pub led_activity_off: u32,
    pub led_association_on: u32,
    pub led_association_off: u32,
    pub led_ofdm_on: u32,
    pub led_ofdm_off: u32,
    pub led_link_on: delayed_work,
    pub led_link_off: delayed_work,
    pub led_act_off: delayed_work,
    pub merge_networks: work_struct,
    pub cmdlog: *mut ipw_cmd_log,
    pub cmdlog_len: c_int,
    pub cmdlog_pos: c_int,
pub const IPW_2200BG: c_int = 1;
pub const IPW_2915ABG: c_int = 2;
    pub adapter: u8,
    pub tx_power: i8,
// Track time in suspend using CLOCK_BOOTTIME
    pub suspend_at: time64_t,
    pub suspend_time: time64_t,
    pub pm_state: [u32; 16],
    pub error: *mut ipw_fw_error,
// network state
// Used to pass the current INTA value from ISR to Tasklet
    pub isr_inta: u32,
// QoS
    pub qos_data: ipw_qos_info,
    pub qos_activate: work_struct,
//
// debugging info
    pub indirect_dword: u32,
    pub direct_dword: u32,
    pub indirect_byte: u32,
}

// debug macros
// Debug and printf string expansion helpers for printing bitfields

//
// To use the debug system;
//
// If you are defining a new debug classification, simply add it to the #define
// list here in the form of:
//
// #define IPW_DL_xxxx VALUE
//
// shifting value to the left one bit from the previous entry.  xxxx should be
// the name of the classification (for example, WEP)
//
// You then need to either add a IPW_xxxx_DEBUG() macro definition for your
// classification, or use IPW_DEBUG(IPW_DL_xxxx, ...) whenever you want
// to send output to that classification.
//
// To add your debug level to the list of levels seen when you perform
//
// % cat /proc/net/ipw/debug_level
//
// you simply need to add your entry to the ipw_debug_levels array.
//
// If you do not see debug_level in /proc/net/ipw then you do not have
// CONFIG_IPW2200_DEBUG defined in your kernel configuration
//

//
// Register bit definitions
//
pub const IPW_INTA_RW: c_uint = 0x00000008;
pub const IPW_INTA_MASK_R: c_uint = 0x0000000C;
pub const IPW_INDIRECT_ADDR: c_uint = 0x00000010;
pub const IPW_INDIRECT_DATA: c_uint = 0x00000014;
pub const IPW_AUTOINC_ADDR: c_uint = 0x00000018;
pub const IPW_AUTOINC_DATA: c_uint = 0x0000001C;
pub const IPW_RESET_REG: c_uint = 0x00000020;
pub const IPW_GP_CNTRL_RW: c_uint = 0x00000024;
pub const IPW_READ_INT_REGISTER: c_uint = 0xFF4;
pub const IPW_GP_CNTRL_BIT_INIT_DONE: c_uint = 0x00000004;
pub const IPW_REGISTER_DOMAIN1_END: c_uint = 0x00001000;
pub const IPW_SRAM_READ_INT_REGISTER: c_uint = 0x00000ff4;
pub const IPW_SHARED_LOWER_BOUND: c_uint = 0x00000200;
pub const IPW_INTERRUPT_AREA_LOWER_BOUND: c_uint = 0x00000f80;
pub const IPW_NIC_SRAM_LOWER_BOUND: c_uint = 0x00000000;
pub const IPW_NIC_SRAM_UPPER_BOUND: c_uint = 0x00030000;

pub const IPW_GP_CNTRL_BIT_CLOCK_READY: c_uint = 0x00000001;
pub const IPW_GP_CNTRL_BIT_HOST_ALLOWS_STANDBY: c_uint = 0x00000002;
//
// RESET Register Bit Indexes
//

pub const IPW_CSR_CIS_UPPER_BOUND: c_uint = 0x00000200;
pub const IPW_DOMAIN_0_END: c_uint = 0x1000;
pub const CLX_MEM_BAR_SIZE: c_uint = 0x1000;
// Dino/baseband control registers bits
pub const DINO_ENABLE_SYSTEM: c_uint = 0x80	/* 1 = baseband processor on, 0 = reset */;
pub const DINO_ENABLE_CS: c_uint = 0x40	/* 1 = enable ucode load */;
pub const DINO_RXFIFO_DATA: c_uint = 0x01	/* 1 = data available */;

pub const IPW_BASEBAND_POWER_DOWN: c_uint = 0x00000001;
pub const IPW_MEM_HALT_AND_RESET: c_uint = 0x003000e0;
// defgroup bits_halt_reset MEM_HALT_AND_RESET register bits
pub const IPW_BIT_HALT_RESET_ON: c_uint = 0x80000000;
pub const IPW_BIT_HALT_RESET_OFF: c_uint = 0x00000000;
pub const CB_LAST_VALID: c_uint = 0x20000000;
pub const CB_INT_ENABLED: c_uint = 0x40000000;
pub const CB_VALID: c_uint = 0x80000000;
pub const CB_SRC_LE: c_uint = 0x08000000;
pub const CB_DEST_LE: c_uint = 0x04000000;
pub const CB_SRC_AUTOINC: c_uint = 0x00800000;
pub const CB_SRC_IO_GATED: c_uint = 0x00400000;
pub const CB_DEST_AUTOINC: c_uint = 0x00080000;
pub const CB_SRC_SIZE_LONG: c_uint = 0x00200000;
pub const CB_DEST_SIZE_LONG: c_uint = 0x00020000;
// DMA DEFINES
pub const DMA_CONTROL_SMALL_CB_CONST_VALUE: c_uint = 0x00540000;
pub const DMA_CB_STOP_AND_ABORT: c_uint = 0x00000C00;
pub const DMA_CB_START: c_uint = 0x00000100;
pub const IPW_SHARED_SRAM_SIZE: c_uint = 0x00030000;
pub const IPW_SHARED_SRAM_DMA_CONTROL: c_uint = 0x00027000;
pub const CB_MAX_LENGTH: c_uint = 0x1FFF;
pub const IPW_HOST_EEPROM_DATA_SRAM_SIZE: c_uint = 0xA18;
pub const IPW_EEPROM_IMAGE_SIZE: c_uint = 0x100;
// DMA defs
pub const IPW_DMA_I_CURRENT_CB: c_uint = 0x003000D0;
pub const IPW_DMA_O_CURRENT_CB: c_uint = 0x003000D4;
pub const IPW_DMA_I_DMA_CONTROL: c_uint = 0x003000A4;
pub const IPW_DMA_I_CB_BASE: c_uint = 0x003000A0;
pub const IPW_TX_CMD_QUEUE_BD_BASE: c_uint = 0x00000200;
pub const IPW_TX_CMD_QUEUE_BD_SIZE: c_uint = 0x00000204;
pub const IPW_TX_QUEUE_0_BD_BASE: c_uint = 0x00000208;

pub const IPW_TX_QUEUE_1_BD_BASE: c_uint = 0x00000210;
pub const IPW_TX_QUEUE_1_BD_SIZE: c_uint = 0x00000214;
pub const IPW_TX_QUEUE_2_BD_BASE: c_uint = 0x00000218;

pub const IPW_TX_QUEUE_3_BD_BASE: c_uint = 0x00000220;
pub const IPW_TX_QUEUE_3_BD_SIZE: c_uint = 0x00000224;
pub const IPW_RX_BD_BASE: c_uint = 0x00000240;
pub const IPW_RX_BD_SIZE: c_uint = 0x00000244;
pub const IPW_RFDS_TABLE_LOWER: c_uint = 0x00000500;
pub const IPW_TX_CMD_QUEUE_READ_INDEX: c_uint = 0x00000280;
pub const IPW_TX_QUEUE_0_READ_INDEX: c_uint = 0x00000284;
pub const IPW_TX_QUEUE_1_READ_INDEX: c_uint = 0x00000288;

pub const IPW_TX_QUEUE_3_READ_INDEX: c_uint = 0x00000290;

//
// EEPROM Related Definitions
//

pub const MSB: c_int = 1;
pub const LSB: c_int = 0;

// EEPROM access by BYTE

// NIC type as found in the one byte EEPROM_NIC_TYPE offset
pub const EEPROM_NIC_TYPE_0: c_int = 0;
pub const EEPROM_NIC_TYPE_1: c_int = 1;
pub const EEPROM_NIC_TYPE_2: c_int = 2;
pub const EEPROM_NIC_TYPE_3: c_int = 3;
pub const EEPROM_NIC_TYPE_4: c_int = 4;
// Bluetooth Coexistence capabilities as found in EEPROM_SKU_CAPABILITY
pub const EEPROM_SKU_CAP_BT_CHANNEL_SIG: c_uint = 0x01	/* we can tell BT our channel # */;
pub const EEPROM_SKU_CAP_BT_PRIORITY: c_uint = 0x02	/* BT can take priority over us */;
pub const EEPROM_SKU_CAP_BT_OOB: c_uint = 0x04	/* we can signal BT out-of-band */;
pub const FW_MEM_REG_LOWER_BOUND: c_uint = 0x00300000;

pub const EEPROM_CMD_READ: c_uint = 0x2;
// Interrupts masks
pub const IPW_INTA_NONE: c_uint = 0x00000000;
pub const IPW_INTA_BIT_RX_TRANSFER: c_uint = 0x00000002;
pub const IPW_INTA_BIT_STATUS_CHANGE: c_uint = 0x00000010;
pub const IPW_INTA_BIT_BEACON_PERIOD_EXPIRED: c_uint = 0x00000020;
// Inta Bits for CF
pub const IPW_INTA_BIT_TX_CMD_QUEUE: c_uint = 0x00000800;
pub const IPW_INTA_BIT_TX_QUEUE_1: c_uint = 0x00001000;
pub const IPW_INTA_BIT_TX_QUEUE_2: c_uint = 0x00002000;
pub const IPW_INTA_BIT_TX_QUEUE_3: c_uint = 0x00004000;
pub const IPW_INTA_BIT_TX_QUEUE_4: c_uint = 0x00008000;
pub const IPW_INTA_BIT_SLAVE_MODE_HOST_CMD_DONE: c_uint = 0x00010000;
pub const IPW_INTA_BIT_PREPARE_FOR_POWER_DOWN: c_uint = 0x00100000;
pub const IPW_INTA_BIT_POWER_DOWN: c_uint = 0x00200000;
pub const IPW_INTA_BIT_FW_INITIALIZATION_DONE: c_uint = 0x01000000;
pub const IPW_INTA_BIT_FW_CARD_DISABLE_PHY_OFF_DONE: c_uint = 0x02000000;
pub const IPW_INTA_BIT_RF_KILL_DONE: c_uint = 0x04000000;
pub const IPW_INTA_BIT_FATAL_ERROR: c_uint = 0x40000000;
pub const IPW_INTA_BIT_PARITY_ERROR: c_uint = 0x80000000;
// Interrupts enabled at init time.

// FW event log definitions

// FW error log definitions

// TX power level (dbm)

pub const IPW_TX_POWER_MAX: c_int = 20;

pub const AUTH_OPEN: c_int = 0;
pub const AUTH_SHARED_KEY: c_int = 1;
pub const AUTH_LEAP: c_int = 2;
pub const AUTH_IGNORE: c_int = 3;
pub const HC_ASSOCIATE: c_int = 0;
pub const HC_REASSOCIATE: c_int = 1;
pub const HC_DISASSOCIATE: c_int = 2;
pub const HC_IBSS_START: c_int = 3;
pub const HC_IBSS_RECONF: c_int = 4;
pub const HC_DISASSOC_QUIET: c_int = 5;

pub const IPW_RATE_CAPABILITIES: c_int = 1;
pub const IPW_RATE_CONNECT: c_int = 0;
//
// Rate values and masks
//
pub const IPW_TX_RATE_1MB: c_uint = 0x0A;
pub const IPW_TX_RATE_2MB: c_uint = 0x14;
pub const IPW_TX_RATE_5MB: c_uint = 0x37;
pub const IPW_TX_RATE_6MB: c_uint = 0x0D;
pub const IPW_TX_RATE_9MB: c_uint = 0x0F;
pub const IPW_TX_RATE_11MB: c_uint = 0x6E;
pub const IPW_TX_RATE_12MB: c_uint = 0x05;
pub const IPW_TX_RATE_18MB: c_uint = 0x07;
pub const IPW_TX_RATE_24MB: c_uint = 0x09;
pub const IPW_TX_RATE_36MB: c_uint = 0x0B;
pub const IPW_TX_RATE_48MB: c_uint = 0x01;
pub const IPW_TX_RATE_54MB: c_uint = 0x03;
pub const IPW_ORD_TABLE_ID_MASK: c_uint = 0x0000FF00;
pub const IPW_ORD_TABLE_VALUE_MASK: c_uint = 0x000000FF;
pub const IPW_ORD_TABLE_0_MASK: c_uint = 0x0000F000;
pub const IPW_ORD_TABLE_1_MASK: c_uint = 0x0000F100;
pub const IPW_ORD_TABLE_2_MASK: c_uint = 0x0000F200;
pub const IPW_ORD_TABLE_3_MASK: c_uint = 0x0000F300;
pub const IPW_ORD_TABLE_4_MASK: c_uint = 0x0000F400;
pub const IPW_ORD_TABLE_5_MASK: c_uint = 0x0000F500;
pub const IPW_ORD_TABLE_6_MASK: c_uint = 0x0000F600;
pub const IPW_ORD_TABLE_7_MASK: c_uint = 0x0000F700;
//
// Table 0 Entries (all entries are 32 bits)
//
// Hole
pub const IPW_RSSI_TO_DBM: c_int = 112;
// Table 1 Entries
//
// Table 2 Entries
//
// FW_VERSION:    16 byte string
// FW_DATE:       16 byte string (only 14 bytes used)
// UCODE_VERSION: 4 byte version code
// UCODE_DATE:    5 bytes code code
// ADDAPTER_MAC:  6 byte MAC address
// RTC:           4 byte clock
//
// Table 3
// Table 4
// Table 5
// Table 6
// Table 7

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_fixed_rate {
    pub tx_rates: __le16,
    pub reserved: __le16,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host_cmd {
    pub cmd: u8,
    pub len: u8,
    pub reserved: u16,
    pub param: *const u32,
    pub /: *mut *mut } __packed; / XXX,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdlog_host_cmd {
    pub cmd: u8,
    pub len: u8,
    pub reserved: __le16,
    pub param: [c_char; 124],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipw_cmd_log {
    pub jiffies: c_ulong,
    pub retcode: c_int,
    pub cmd: cmdlog_host_cmd,
}

// SysConfig command parameters ...
// bt_coexistence param
pub const CFG_BT_COEXISTENCE_SIGNAL_CHNL: c_uint = 0x01	/* tell BT our chnl # */;
pub const CFG_BT_COEXISTENCE_DEFER: c_uint = 0x02	/* defer our Tx if BT traffic */;
pub const CFG_BT_COEXISTENCE_KILL: c_uint = 0x04	/* kill our Tx if BT traffic */;
pub const CFG_BT_COEXISTENCE_WME_OVER_BT: c_uint = 0x08	/* multimedia extensions */;
pub const CFG_BT_COEXISTENCE_OOB: c_uint = 0x10	/* signal BT via out-of-band */;
// clear-to-send to self param
pub const CFG_CTS_TO_ITSELF_ENABLED_MIN: c_uint = 0x00;
pub const CFG_CTS_TO_ITSELF_ENABLED_MAX: c_uint = 0x01;

// Antenna diversity param (h/w can select best antenna, based on signal)
pub const CFG_SYS_ANTENNA_BOTH: c_uint = 0x00	/* NIC selects best antenna */;
pub const CFG_SYS_ANTENNA_A: c_uint = 0x01	/* force antenna A */;
pub const CFG_SYS_ANTENNA_B: c_uint = 0x03	/* force antenna B */;
pub const CFG_SYS_ANTENNA_SLOW_DIV: c_uint = 0x02	/* consider background noise */;
pub const IPW_MAX_CONFIG_RETRIES: c_int = 10;
