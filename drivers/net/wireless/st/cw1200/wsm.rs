//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/st/cw1200/wsm.h
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
// WSM host interface (HI) interface for ST-Ericsson CW1200 mac80211 drivers
//
// Copyright (c) 2010, ST-Ericsson
// Author: Dmitry Tarnyagin <dmitry.tarnyagin@lockless.no>
//
// Based on CW1200 UMAC WSM API, which is
// Copyright (C) ST-Ericsson SA 2010
// Author: Stewart Mathers <stewart.mathers@stericsson.com>
//

// Macro flag: #define CW1200_WSM_H_INCLUDED

// Bands
// Radio band 2.412 -2.484 GHz.

// Radio band 4.9375-5.8250 GHz.

// Transmit rates
// 1   Mbps            ERP-DSSS

// 2   Mbps            ERP-DSSS

// 5.5 Mbps            ERP-CCK

// 11  Mbps            ERP-CCK

// 22  Mbps            ERP-PBCC (Not supported)
// #define WSM_TRANSMIT_RATE_22		(4)
// 33  Mbps            ERP-PBCC (Not supported)
// #define WSM_TRANSMIT_RATE_33		(5)
// 6   Mbps   (3 Mbps) ERP-OFDM, BPSK coding rate 1/2

// 9   Mbps (4.5 Mbps) ERP-OFDM, BPSK coding rate 3/4

// 12  Mbps  (6 Mbps)  ERP-OFDM, QPSK coding rate 1/2

// 18  Mbps  (9 Mbps)  ERP-OFDM, QPSK coding rate 3/4

// 24  Mbps (12 Mbps)  ERP-OFDM, 16QAM coding rate 1/2

// 36  Mbps (18 Mbps)  ERP-OFDM, 16QAM coding rate 3/4

// 48  Mbps (24 Mbps)  ERP-OFDM, 64QAM coding rate 1/2

// 54  Mbps (27 Mbps)  ERP-OFDM, 64QAM coding rate 3/4

// 6.5 Mbps            HT-OFDM, BPSK coding rate 1/2

// 13  Mbps            HT-OFDM, QPSK coding rate 1/2

// 19.5 Mbps           HT-OFDM, QPSK coding rate 3/4

// 26  Mbps            HT-OFDM, 16QAM coding rate 1/2

// 39  Mbps            HT-OFDM, 16QAM coding rate 3/4

// 52  Mbps            HT-OFDM, 64QAM coding rate 2/3

// 58.5 Mbps           HT-OFDM, 64QAM coding rate 3/4

// 65  Mbps            HT-OFDM, 64QAM coding rate 5/6

// Scan types
// Foreground scan

// Background scan

// Auto scan

// Scan flags
// Forced background scan means if the station cannot
// enter the power-save mode, it shall force to perform a
// background scan. Only valid when ScanType is
// background scan.

// The WLAN device scans one channel at a time so
// that disturbance to the data traffic is minimized.

// Preamble Type. Long if not set.

// 11n Tx Mode. Mixed if not set.

// Scan constraints
// Maximum number of channels to be scanned.

// The maximum number of SSIDs that the device can scan for.

// Power management modes
// 802.11 Active mode

// 802.11 PS mode

// Fast Power Save bit

// Dynamic aka Fast power save

// Undetermined
// Note : Undetermined status is reported when the
// NULL data frame used to advertise the PM mode to
// the AP at Pre or Post Background Scan is not Acknowledged

// Queue IDs
// best effort/legacy

// background

// video

// voice

// HT TX parameters
// Non-HT

// Mixed format

// Greenfield format

// STBC allowed

// EPTA prioirty flags for BT Coex
// default epta priority
pub const WSM_EPTA_PRIORITY_DEFAULT: c_int = 4;
// use for normal data
pub const WSM_EPTA_PRIORITY_DATA: c_int = 4;
// use for connect/disconnect/roaming
pub const WSM_EPTA_PRIORITY_MGT: c_int = 5;
// use for action frames
pub const WSM_EPTA_PRIORITY_ACTION: c_int = 5;
// use for AC_VI data
pub const WSM_EPTA_PRIORITY_VIDEO: c_int = 5;
// use for AC_VO data
pub const WSM_EPTA_PRIORITY_VOICE: c_int = 6;
// use for EAPOL exchange
pub const WSM_EPTA_PRIORITY_EAPOL: c_int = 7;
// TX status
// Frame was sent aggregated
// Only valid for WSM_SUCCESS status.

// Host should requeue this frame later.
// Valid only when status is WSM_REQUEUE.

// Normal Ack

// No Ack

// No explicit acknowledgement

// Block Ack
// Only valid for WSM_SUCCESS status.

// RX status
// Unencrypted

// WEP

// TKIP

// AES

// WAPI

// Macro to fetch encryption subfield.

// Frame was part of an aggregation

// Frame was first in the aggregation

// Frame was last in the aggregation

// Indicates a defragmented frame

// Indicates a Beacon frame

// Indicates STA bit beacon TIM field

// Indicates Beacon frame's virtual bitmap contains multicast bit

// Indicates frame contains a matching SSID

// Indicates frame contains a matching BSSI

// Indicates More bit set in Framectl field

// Indicates frame received during a measurement process

// Indicates frame received as an HT packet

// Indicates frame received with STBC

// Indicates Address 1 field matches dot11StationId

// Indicates Group address present in the Address 1 field

// Indicates Broadcast address present in the Address 1 field

// Indicates group key used with encrypted frames

// Macro to fetch encryption key index.

// Indicates TSF inclusion after 802.11 frame body

// Frame Control field starts at Frame offset + 2

// Join mode
// IBSS

// BSS

// PLCP preamble type
// For long preamble

// For short preamble (Long for 1Mbps)

// For short preamble (Long for 1 and 2Mbps)

// Join flags
// Unsynchronized

// The BSS owner is a P2P GO

// Force to join BSS with the BSSID and the
// SSID specified without waiting for beacons. The
// ProbeForJoin parameter is ignored.
//

// Give probe request/response higher
// priority over the BT traffic
//

// Issue immediate join confirmation and use
// join complete to notify about completion
//

// Key types

// Key indexes

// ACK policy

// Start modes

// SetAssociationMode MIB flags

// RcpiRssiThreshold MIB flags

// Update-ie constants

// WSM events
// Error

// BSS lost

// BSS regained

// Radar detected

// RCPI or RSSI threshold triggered

// BT inactive

// BT active

// MIB IDs
// 4.1  dot11StationId
pub const WSM_MIB_ID_DOT11_STATION_ID: c_uint = 0x0000;
// 4.2  dot11MaxtransmitMsduLifeTime
pub const WSM_MIB_ID_DOT11_MAX_TRANSMIT_LIFTIME: c_uint = 0x0001;
// 4.3  dot11MaxReceiveLifeTime
pub const WSM_MIB_ID_DOT11_MAX_RECEIVE_LIFETIME: c_uint = 0x0002;
// 4.4  dot11SlotTime
pub const WSM_MIB_ID_DOT11_SLOT_TIME: c_uint = 0x0003;
// 4.5  dot11GroupAddressesTable
pub const WSM_MIB_ID_DOT11_GROUP_ADDRESSES_TABLE: c_uint = 0x0004;
pub const WSM_MAX_GRP_ADDRTABLE_ENTRIES: c_int = 8;
// 4.6  dot11WepDefaultKeyId
pub const WSM_MIB_ID_DOT11_WEP_DEFAULT_KEY_ID: c_uint = 0x0005;
// 4.7  dot11CurrentTxPowerLevel
pub const WSM_MIB_ID_DOT11_CURRENT_TX_POWER_LEVEL: c_uint = 0x0006;
// 4.8  dot11RTSThreshold
pub const WSM_MIB_ID_DOT11_RTS_THRESHOLD: c_uint = 0x0007;
// 4.9  NonErpProtection
pub const WSM_MIB_ID_NON_ERP_PROTECTION: c_uint = 0x1000;
// 4.10 ArpIpAddressesTable
pub const WSM_MIB_ID_ARP_IP_ADDRESSES_TABLE: c_uint = 0x1001;
pub const WSM_MAX_ARP_IP_ADDRTABLE_ENTRIES: c_int = 1;
// 4.11 TemplateFrame
pub const WSM_MIB_ID_TEMPLATE_FRAME: c_uint = 0x1002;
// 4.12 RxFilter
pub const WSM_MIB_ID_RX_FILTER: c_uint = 0x1003;
// 4.13 BeaconFilterTable
pub const WSM_MIB_ID_BEACON_FILTER_TABLE: c_uint = 0x1004;
// 4.14 BeaconFilterEnable
pub const WSM_MIB_ID_BEACON_FILTER_ENABLE: c_uint = 0x1005;
// 4.15 OperationalPowerMode
pub const WSM_MIB_ID_OPERATIONAL_POWER_MODE: c_uint = 0x1006;
// 4.16 BeaconWakeUpPeriod
pub const WSM_MIB_ID_BEACON_WAKEUP_PERIOD: c_uint = 0x1007;
// 4.17 RcpiRssiThreshold
pub const WSM_MIB_ID_RCPI_RSSI_THRESHOLD: c_uint = 0x1009;
// 4.18 StatisticsTable
pub const WSM_MIB_ID_STATISTICS_TABLE: c_uint = 0x100A;
// 4.19 IbssPsConfig
pub const WSM_MIB_ID_IBSS_PS_CONFIG: c_uint = 0x100B;
// 4.20 CountersTable
pub const WSM_MIB_ID_COUNTERS_TABLE: c_uint = 0x100C;
// 4.21 BlockAckPolicy
pub const WSM_MIB_ID_BLOCK_ACK_POLICY: c_uint = 0x100E;
// 4.22 OverrideInternalTxRate
pub const WSM_MIB_ID_OVERRIDE_INTERNAL_TX_RATE: c_uint = 0x100F;
// 4.23 SetAssociationMode
pub const WSM_MIB_ID_SET_ASSOCIATION_MODE: c_uint = 0x1010;
// 4.24 UpdateEptaConfigData
pub const WSM_MIB_ID_UPDATE_EPTA_CONFIG_DATA: c_uint = 0x1011;
// 4.25 SelectCcaMethod
pub const WSM_MIB_ID_SELECT_CCA_METHOD: c_uint = 0x1012;
// 4.26 SetUpasdInformation
pub const WSM_MIB_ID_SET_UAPSD_INFORMATION: c_uint = 0x1013;
// 4.27 SetAutoCalibrationMode  WBF00004073
pub const WSM_MIB_ID_SET_AUTO_CALIBRATION_MODE: c_uint = 0x1015;
// 4.28 SetTxRateRetryPolicy
pub const WSM_MIB_ID_SET_TX_RATE_RETRY_POLICY: c_uint = 0x1016;
// 4.29 SetHostMessageTypeFilter
pub const WSM_MIB_ID_SET_HOST_MSG_TYPE_FILTER: c_uint = 0x1017;
// 4.30 P2PFindInfo
pub const WSM_MIB_ID_P2P_FIND_INFO: c_uint = 0x1018;
// 4.31 P2PPsModeInfo
pub const WSM_MIB_ID_P2P_PS_MODE_INFO: c_uint = 0x1019;
// 4.32 SetEtherTypeDataFrameFilter
pub const WSM_MIB_ID_SET_ETHERTYPE_DATAFRAME_FILTER: c_uint = 0x101A;
// 4.33 SetUDPPortDataFrameFilter
pub const WSM_MIB_ID_SET_UDPPORT_DATAFRAME_FILTER: c_uint = 0x101B;
// 4.34 SetMagicDataFrameFilter
pub const WSM_MIB_ID_SET_MAGIC_DATAFRAME_FILTER: c_uint = 0x101C;
// 4.35 P2PDeviceInfo
pub const WSM_MIB_ID_P2P_DEVICE_INFO: c_uint = 0x101D;
// 4.36 SetWCDMABand
pub const WSM_MIB_ID_SET_WCDMA_BAND: c_uint = 0x101E;
// 4.37 GroupTxSequenceCounter
pub const WSM_MIB_ID_GRP_SEQ_COUNTER: c_uint = 0x101F;
// 4.38 ProtectedMgmtPolicy
pub const WSM_MIB_ID_PROTECTED_MGMT_POLICY: c_uint = 0x1020;
// 4.39 SetHtProtection
pub const WSM_MIB_ID_SET_HT_PROTECTION: c_uint = 0x1021;
// 4.40 GPIO Command
pub const WSM_MIB_ID_GPIO_COMMAND: c_uint = 0x1022;
// 4.41 TSF Counter Value
pub const WSM_MIB_ID_TSF_COUNTER: c_uint = 0x1023;
// Test Purposes Only
pub const WSM_MIB_ID_BLOCK_ACK_INFO: c_uint = 0x100D;
// 4.42 UseMultiTxConfMessage
pub const WSM_MIB_USE_MULTI_TX_CONF: c_uint = 0x1024;
// 4.43 Keep-alive period
pub const WSM_MIB_ID_KEEP_ALIVE_PERIOD: c_uint = 0x1025;
// 4.44 Disable BSSID filter
pub const WSM_MIB_ID_DISABLE_BSSID_FILTER: c_uint = 0x1026;
// Frame template types

// Status
// The WSM firmware has completed a request
// successfully.

// This is a generic failure code if other error codes do
// not apply.

// A request contains one or more invalid parameters.

// The request cannot perform because the device is in
// an inappropriate mode.

// The frame received includes a decryption error.

// A MIC failure is detected in the received packets.

// The transmit request failed due to retry limit being
// exceeded.

// The transmit request failed due to MSDU life time
// being exceeded.

// The link to the AP is lost.

// No key was found for the encrypted frame

// Jammer was detected when transmitting this frame

// The message should be requeued later.
// This is applicable only to Transmit

// Advanced filtering options

// Actual header of WSM messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_hdr {
    pub len: __le16,
    pub id: __le16,
}

pub const MAX_BEACON_SKIP_TIME_MS: c_int = 1000;

// ********************************************************************
// WSM capability
pub const WSM_STARTUP_IND_ID: c_uint = 0x0801;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_startup_ind {
    pub input_buffers: u16,
    pub input_buffer_size: u16,
    pub status: u16,
    pub hw_id: u16,
    pub hw_subid: u16,
    pub fw_cap: u16,
    pub fw_type: u16,
    pub fw_api: u16,
    pub fw_build: u16,
    pub fw_ver: u16,
    pub fw_label: [c_char; 128],
    pub config: [u32; 4],
}

// ********************************************************************
// WSM commands
// 3.1
pub const WSM_CONFIGURATION_REQ_ID: c_uint = 0x0009;
pub const WSM_CONFIGURATION_RESP_ID: c_uint = 0x0409;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_tx_power_range {
    pub min_power_level: c_int,
    pub max_power_level: c_int,
    pub stepping: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_configuration {
// [in] */ u32 dot11MaxTransmitMsduLifeTime;
// [in] */ u32 dot11MaxReceiveLifeTime;
// [in] */ u32 dot11RtsThreshold;
// [in, out] */ u8 *dot11StationId;
// [in] */ const void *dpdData;
// [in] */ size_t dpdData_size;
// [out] */ u8 dot11FrequencyBandsSupported;
// [out] */ u32 supportedRateMask;
// [out] */ struct wsm_tx_power_range txPowerRange[2];
}

// 3.3
pub const WSM_RESET_REQ_ID: c_uint = 0x000A;
pub const WSM_RESET_RESP_ID: c_uint = 0x040A;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_reset {
// [in] */ int link_id;
// [in] */ bool reset_statistics;
}

extern "C" {
    pub fn wsm_reset(priv: *mut cw1200_common, arg: *const wsm_reset) -> c_int;
}
// 3.5
pub const WSM_READ_MIB_REQ_ID: c_uint = 0x0005;
pub const WSM_READ_MIB_RESP_ID: c_uint = 0x0405;
// 3.7
pub const WSM_WRITE_MIB_REQ_ID: c_uint = 0x0006;
pub const WSM_WRITE_MIB_RESP_ID: c_uint = 0x0406;
// 3.9
pub const WSM_START_SCAN_REQ_ID: c_uint = 0x0007;
pub const WSM_START_SCAN_RESP_ID: c_uint = 0x0407;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_ssid {
    pub ssid: [u8; 32],
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_scan_ch {
    pub number: u16,
    pub min_chan_time: u32,
    pub max_chan_time: u32,
    pub tx_power_level: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_scan {
// WSM_PHY_BAND_...
    pub band: u8,
// WSM_SCAN_TYPE_...
    pub type: u8,
// WSM_SCAN_FLAG_...
    pub flags: u8,
// WSM_TRANSMIT_RATE_...
    pub max_tx_rate: u8,
// Interval period in TUs that the device shall the re-
// execute the requested scan. Max value supported by the device
// is 256s.
    pub auto_scan_interval: u32,
// Number of probe requests (per SSID) sent to one (1)
// channel. Zero (0) means that none is send, which
// means that a passive scan is to be done. Value
// greater than zero (0) means that an active scan is to
// be done.
    pub num_probes: u32,
// Number of channels to be scanned.
// Maximum value is WSM_SCAN_MAX_NUM_OF_CHANNELS.
    pub num_channels: u8,
// Number of SSID provided in the scan command (this
// is zero (0) in broadcast scan)
// The maximum number of SSIDs is WSM_SCAN_MAX_NUM_OF_SSIDS.
    pub num_ssids: u8,
// The delay time (in microseconds) period
// before sending a probe-request.
    pub probe_delay: u8,
// SSIDs to be scanned [numOfSSIDs];
    pub ssids: *mut wsm_ssid,
// Channels to be scanned [numOfChannels];
    pub ch: *mut wsm_scan_ch,
}

extern "C" {
    pub fn wsm_scan(priv: *mut cw1200_common, arg: *const wsm_scan) -> c_int;
}
// 3.11
pub const WSM_STOP_SCAN_REQ_ID: c_uint = 0x0008;
pub const WSM_STOP_SCAN_RESP_ID: c_uint = 0x0408;
extern "C" {
    pub fn wsm_stop_scan(priv: *mut cw1200_common) -> c_int;
}
// 3.13
pub const WSM_SCAN_COMPLETE_IND_ID: c_uint = 0x0806;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_scan_complete {
// WSM_STATUS_...
    pub status: u32,
// WSM_PSM_...
    pub psm: u8,
// Number of channels that the scan operation completed.
    pub num_channels: u8,
}

// 3.14
pub const WSM_TX_CONFIRM_IND_ID: c_uint = 0x0404;
pub const WSM_MULTI_TX_CONFIRM_ID: c_uint = 0x041E;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_tx_confirm {
// Packet identifier used in wsm_tx.
    pub packet_id: u32,
// WSM_STATUS_...
    pub status: u32,
// WSM_TRANSMIT_RATE_...
    pub tx_rate: u8,
// The number of times the frame was transmitted
// without receiving an acknowledgement.
    pub ack_failures: u8,
// WSM_TX_STATUS_...
    pub flags: u16,
// The total time in microseconds that the frame spent in
// the WLAN device before transmission as completed.
    pub media_delay: u32,
// The total time in microseconds that the frame spent in
// the WLAN device before transmission was started.
    pub tx_queue_delay: u32,
}

// 3.15
// Note that ideology of wsm_tx struct is different against the rest of
// WSM API. wsm_hdr is /not/ a caller-adapted struct to be used as an input
// argument for WSM call, but a prepared bytestream to be sent to firmware.
// It is filled partly in cw1200_tx, partly in low-level WSM code.
// Please pay attention once again: ideology is different.
//
// Legend:
// - [in]: cw1200_tx must fill this field.
// - [wsm]: the field is filled by low-level WSM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_tx {
// common WSM header
    pub hdr: wsm_hdr,
// Packet identifier that meant to be used in completion.
    pub /: *mut *mut u32 packet_id; / Note this is actually a cookie,
// WSM_TRANSMIT_RATE_...
    pub max_tx_rate: u8,
// WSM_QUEUE_...
    pub queue_id: u8,
// True: another packet is pending on the host for transmission.
    pub more: u8,
// Bit 0 = 0 - Start expiry time from first Tx attempt (default)
// Bit 0 = 1 - Start expiry time from receipt of Tx Request
// Bits 3:1  - PTA Priority
// Bits 6:4  - Tx Rate Retry Policy
// Bit 7 - Reserved
    pub flags: u8,
// Should be 0.
    pub reserved: u32,
// The elapsed time in TUs, after the initial transmission
// of an MSDU, after which further attempts to transmit
// the MSDU shall be terminated. Overrides the global
// dot11MaxTransmitMsduLifeTime setting [optional]
// Device will set the default value if this is 0.
    pub expire_time: __le32,
// WSM_HT_TX_...
    pub ht_tx_parameters: __le32,
    pub __packed: },
// = sizeof(generic hi hdr) + sizeof(wsm hdr) + sizeof(alignment)

// 3.16
pub const WSM_RECEIVE_IND_ID: c_uint = 0x0804;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_rx {
// WSM_STATUS_...
    pub status: u32,
// Specifies the channel of the received packet.
    pub channel_number: u16,
// WSM_TRANSMIT_RATE_...
    pub rx_rate: u8,
// This value is expressed in signed Q8.0 format for
// RSSI and unsigned Q7.1 format for RCPI.
    pub rcpi_rssi: u8,
// WSM_RX_STATUS_...
    pub flags: u32,
}

// = sizeof(generic hi hdr) + sizeof(wsm hdr)

// 3.17
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_event {
// WSM_STATUS_...
// [out] */ u32 id;
// Indication parameters.
// For error indication, this shall be a 32-bit WSM status.
// For RCPI or RSSI indication, this should be an 8-bit
// RCPI or RSSI value.
// [out] */ u32 data;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cw1200_wsm_event {
    pub link: list_head,
    pub evt: wsm_event,
}

// 3.18 - 3.22
// Measurement. Skipped for now. Irrelevent.
// 3.23
pub const WSM_JOIN_REQ_ID: c_uint = 0x000B;
pub const WSM_JOIN_RESP_ID: c_uint = 0x040B;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_join {
// WSM_JOIN_MODE_...
    pub mode: u8,
// WSM_PHY_BAND_...
    pub band: u8,
// Specifies the channel number to join. The channel
// number will be mapped to an actual frequency
// according to the band
    pub channel_number: u16,
// Specifies the BSSID of the BSS or IBSS to be joined
// or the IBSS to be started.
    pub bssid: [u8; 6],
// ATIM window of IBSS
// When ATIM window is zero the initiated IBSS does
// not support power saving.
    pub atim_window: u16,
// WSM_JOIN_PREAMBLE_...
    pub preamble_type: u8,
// Specifies if a probe request should be send with the
// specified SSID when joining to the network.
    pub probe_for_join: u8,
// DTIM Period (In multiples of beacon interval)
    pub dtim_period: u8,
// WSM_JOIN_FLAGS_...
    pub flags: u8,
// Length of the SSID
    pub ssid_len: u32,
// Specifies the SSID of the IBSS to join or start
    pub ssid: [u8; 32],
// Specifies the time between TBTTs in TUs
    pub beacon_interval: u32,
// A bit mask that defines the BSS basic rate set.
    pub basic_rate_set: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_join_cnf {
    pub status: u32,
// Minimum transmission power level in units of 0.1dBm
    pub min_power_level: u32,
// Maximum transmission power level in units of 0.1dBm
    pub max_power_level: u32,
}

extern "C" {
    pub fn wsm_join(priv: *mut cw1200_common, arg: *mut wsm_join) -> c_int;
}
// 3.24
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_join_complete {
// WSM_STATUS_...
    pub status: u32,
}

// 3.25
pub const WSM_SET_PM_REQ_ID: c_uint = 0x0010;
pub const WSM_SET_PM_RESP_ID: c_uint = 0x0410;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_set_pm {
// WSM_PSM_...
    pub mode: u8,
// in unit of 500us; 0 to use default
    pub fast_psm_idle_period: u8,
// in unit of 500us; 0 to use default
    pub ap_psm_change_period: u8,
// in unit of 500us; 0 to disable auto-pspoll
    pub min_auto_pspoll_period: u8,
}

extern "C" {
    pub fn wsm_set_pm(priv: *mut cw1200_common, arg: *const wsm_set_pm) -> c_int;
}
// 3.27
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_set_pm_complete {
    pub /: *mut *mut u8 psm; / WSM_PSM_...,
}

// 3.28
pub const WSM_SET_BSS_PARAMS_REQ_ID: c_uint = 0x0011;
pub const WSM_SET_BSS_PARAMS_RESP_ID: c_uint = 0x0411;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_set_bss_params {
// This resets the beacon loss counters only
    pub reset_beacon_loss: u8,
// The number of lost consecutive beacons after which
// the WLAN device should indicate the BSS-Lost event
// to the WLAN host driver.
    pub beacon_lost_count: u8,
// The AID received during the association process.
    pub aid: u16,
// The operational rate set mask
    pub operational_rate_set: u32,
}

// 3.30
pub const WSM_ADD_KEY_REQ_ID: c_uint = 0x000C;
pub const WSM_ADD_KEY_RESP_ID: c_uint = 0x040C;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_add_key {
    pub /: *mut *mut u8 type; / WSM_KEY_TYPE_...,
    pub /: *mut *mut u8 index; / Key entry index: 0 -- WSM_KEY_MAX_INDEX,
    pub reserved: u16,
    pub /: *mut *mut u8 peer[6]; / MAC address of the peer station,
    pub reserved: u8,
    pub /: *mut *mut u8 keylen; / Key length in bytes,
    pub /: *mut *mut u8 keydata[16]; / Key data,
    pub wep_pairwise: } __packed,
    pub /: *mut *mut u8 keyid; / Unique per key identifier (0..3),
    pub /: *mut *mut u8 keylen; / Key length in bytes,
    pub reserved: u16,
    pub /: *mut *mut u8 keydata[16]; / Key data,
    pub wep_group: } __packed,
    pub /: *mut *mut u8 peer[6]; / MAC address of the peer station,
    pub reserved: u16,
    pub /: *mut *mut u8 keydata[16]; / TKIP key data,
    pub /: *mut *mut u8 rx_mic_key[8]; / Rx MIC key,
    pub /: *mut *mut u8 tx_mic_key[8]; / Tx MIC key,
    pub tkip_pairwise: } __packed,
    pub /: *mut *mut u8 keydata[16]; / TKIP key data,
    pub /: *mut *mut u8 rx_mic_key[8]; / Rx MIC key,
    pub /: *mut *mut u8 keyid; / Key ID,
    pub reserved: [u8; 3],
    pub /: *mut *mut u8 rx_seqnum[8]; / Receive Sequence Counter,
    pub tkip_group: } __packed,
    pub /: *mut *mut u8 peer[6]; / MAC address of the peer station,
    pub reserved: u16,
    pub /: *mut *mut u8 keydata[16]; / AES key data,
    pub aes_pairwise: } __packed,
    pub /: *mut *mut u8 keydata[16]; / AES key data,
    pub /: *mut *mut u8 keyid; / Key ID,
    pub reserved: [u8; 3],
    pub /: *mut *mut u8 rx_seqnum[8]; / Receive Sequence Counter,
    pub aes_group: } __packed,
    pub /: *mut *mut u8 peer[6]; / MAC address of the peer station,
    pub /: *mut *mut u8 keyid; / Key ID,
    pub reserved: u8,
    pub /: *mut *mut u8 keydata[16]; / WAPI key data,
    pub /: *mut *mut u8 mic_key[16]; / MIC key data,
    pub wapi_pairwise: } __packed,
    pub /: *mut *mut u8 keydata[16]; / WAPI key data,
    pub /: *mut *mut u8 mic_key[16]; / MIC key data,
    pub /: *mut *mut u8 keyid; / Key ID,
    pub reserved: [u8; 3],
    pub wapi_group: } __packed,
    pub __packed: },
    pub __packed: },
    pub arg): *const *const int wsm_add_key(struct cw1200_common priv, struct wsm_add_key,
// 3.32
pub const WSM_REMOVE_KEY_REQ_ID: c_uint = 0x000D;
pub const WSM_REMOVE_KEY_RESP_ID: c_uint = 0x040D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_remove_key {
    pub /: *mut *mut u8 index; / Key entry index : 0-10,
}

// 3.34
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_set_tx_queue_params {
// WSM_ACK_POLICY_...
    pub ackPolicy: u8,
// Medium Time of TSPEC (in 32us units) allowed per
// One Second Averaging Period for this queue.
    pub allowedMediumTime: u16,
// dot11MaxTransmitMsduLifetime to be used for the
// specified queue.
    pub maxTransmitLifetime: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_tx_queue_params {
// NOTE: index is a linux queue id.
    pub params: [wsm_set_tx_queue_params; 4],
}

// 3.36
pub const WSM_EDCA_PARAMS_REQ_ID: c_uint = 0x0013;
pub const WSM_EDCA_PARAMS_RESP_ID: c_uint = 0x0413;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_edca_queue_params {
// CWmin (in slots) for the access class.
    pub cwmin: u16,
// CWmax (in slots) for the access class.
    pub cwmax: u16,
// AIFS (in slots) for the access class.
    pub aifns: u16,
// TX OP Limit (in microseconds) for the access class.
    pub txop_limit: u16,
// dot11MaxReceiveLifetime to be used for the specified
// the access class. Overrides the global
// dot11MaxReceiveLifetime value
    pub max_rx_lifetime: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_edca_params {
// NOTE: index is a linux queue id.
    pub params: [wsm_edca_queue_params; 4],
    pub uapsd_enable: [bool; 4],
}

pub const TXOP_UNIT: c_int = 32;

// 3.38
// Set-System info. Skipped for now. Irrelevent.
// 3.40
pub const WSM_SWITCH_CHANNEL_REQ_ID: c_uint = 0x0016;
pub const WSM_SWITCH_CHANNEL_RESP_ID: c_uint = 0x0416;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_switch_channel {
// 1 - means the STA shall not transmit any further
// frames until the channel switch has completed
    pub mode: u8,
// Number of TBTTs until channel switch occurs.
// 0 - indicates switch shall occur at any time
// 1 - occurs immediately before the next TBTT
    pub switch_count: u8,
// The new channel number to switch to.
// Note this is defined as per section 2.7.
    pub channel_number: u16,
}

pub const WSM_START_REQ_ID: c_uint = 0x0017;
pub const WSM_START_RESP_ID: c_uint = 0x0417;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_start {
// WSM_START_MODE_...
// [in] */ u8 mode;
// WSM_PHY_BAND_...
// [in] */ u8 band;
// Channel number
// [in] */ u16 channel_number;
// Client Traffic window in units of TU
// Valid only when mode == ..._P2P
// [in] */ u32 ct_window;
// Interval between two consecutive
// beacon transmissions in TU.
// [in] */ u32 beacon_interval;
// DTIM period in terms of beacon intervals
// [in] */ u8 dtim_period;
// WSM_JOIN_PREAMBLE_...
// [in] */ u8 preamble;
// The delay time (in microseconds) period
// before sending a probe-request.
// [in] */ u8 probe_delay;
// Length of the SSID
// [in] */ u8 ssid_len;
// SSID of the BSS or P2P_GO to be started now.
// [in] */ u8 ssid[32];
// The basic supported rates for the MiniAP.
// [in] */ u32 basic_rate_set;
}

extern "C" {
    pub fn wsm_start(priv: *mut cw1200_common, arg: *const wsm_start) -> c_int;
}
pub const WSM_BEACON_TRANSMIT_REQ_ID: c_uint = 0x0018;
pub const WSM_BEACON_TRANSMIT_RESP_ID: c_uint = 0x0418;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_beacon_transmit {
// 1: enable; 0: disable
// [in] */ u8 enable_beaconing;
}

extern "C" {
    pub fn wsm_start_find(priv: *mut cw1200_common) -> c_int;
}
extern "C" {
    pub fn wsm_stop_find(priv: *mut cw1200_common) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_suspend_resume {
// See 3.52
// Link ID
// [out] */ int link_id;
// Stop sending further Tx requests down to device for this link
// [out] */ bool stop;
// Transmit multicast Frames
// [out] */ bool multicast;
// The AC on which Tx to be suspended /resumed.
// This is applicable only for U-APSD
// WSM_QUEUE_...
// [out] */ int queue;
}

// 3.54 Update-IE request.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_update_ie {
// WSM_UPDATE_IE_...
// [in] */ u16 what;
// [in] */ u16 count;
// [in] */ u8 *ies;
// [in] */ size_t length;
}

// 3.56
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_map_link {
// MAC address of the remote device
// [in] */ u8 mac_addr[6];
// [in] */ u8 link_id;
}

extern "C" {
    pub fn wsm_map_link(priv: *mut cw1200_common, arg: *const wsm_map_link) -> c_int;
}
// ********************************************************************
// MIB shortcats
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_rcpi_rssi_threshold {
    pub /: *mut *mut u8 rssiRcpiMode; / WSM_RCPI_RSSI_...,
    pub lowerThreshold: u8,
    pub upperThreshold: u8,
    pub rollingAverageCount: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_mib_counters_table {
    pub plcp_errors: __le32,
    pub fcs_errors: __le32,
    pub tx_packets: __le32,
    pub rx_packets: __le32,
    pub rx_packet_errors: __le32,
    pub rx_decryption_failures: __le32,
    pub rx_mic_failures: __le32,
    pub rx_no_key_failures: __le32,
    pub tx_multicast_frames: __le32,
    pub tx_frames_success: __le32,
    pub tx_frame_failures: __le32,
    pub tx_frames_retried: __le32,
    pub tx_frames_multi_retried: __le32,
    pub rx_frame_duplicates: __le32,
    pub rts_success: __le32,
    pub rts_failures: __le32,
    pub ack_failures: __le32,
    pub rx_multicast_frames: __le32,
    pub rx_frames_success: __le32,
    pub rx_cmac_icv_errors: __le32,
    pub rx_cmac_replays: __le32,
    pub rx_mgmt_ccmp_replays: __le32,
    pub __packed: },
    pub sizeof(*arg)): *mut arg,,
    pub ETH_ALEN): return wsm_read_mib(priv, WSM_MIB_ID_DOT11_STATION_ID, mac,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_rx_filter {
    pub promiscuous: bool,
    pub bssid: bool,
    pub fcs: bool,
    pub probeResponder: bool,
}

extern "C" {
    pub fn wsm_write_mib(_arg: priv, _arg: WSM_MIB_ID_RX_FILTER, _arg: &val, _arg: sizeof(val)) -> return;
}
extern "C" {
    pub fn wsm_set_probe_responder(priv: *mut cw1200_common, enable: bool) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_beacon_filter_table_entry {
    pub ie_id: u8,
    pub flags: u8,
    pub oui: [u8; 3],
    pub match_data: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_mib_beacon_filter_table {
    pub num: __le32,
    pub entry: [wsm_beacon_filter_table_entry; 10],
    pub __packed: },
    pub size): return wsm_write_mib(priv, WSM_MIB_ID_BEACON_FILTER_TABLE, ft,,

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_beacon_filter_control {
    pub enabled: c_int,
    pub bcn_count: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wsm_power_mode {
    wsm_power_mode_active = 0,
    wsm_power_mode_doze = 1,
    wsm_power_mode_quiescent = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_operational_mode {
    pub power_mode: wsm_power_mode,
    pub disable_more_flag_usage: c_int,
    pub perform_ant_diversity: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_template_frame {
    pub frame_type: u8,
    pub rate: u8,
    pub skb: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_protected_mgmt_policy {
    pub protectedMgmtEnable: bool,
    pub unprotectedMgmtFramesAllowed: bool,
    pub encryptionForAuthFrame: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_mib_block_ack_policy {
    pub tx_tid: u8,
    pub reserved1: u8,
    pub rx_tid: u8,
    pub reserved2: u8,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_mib_association_mode {
    pub /: *mut *mut u8 flags; / WSM_ASSOCIATION_MODE_...,
    pub /: *mut *mut u8 preamble; / WSM_JOIN_PREAMBLE_...,
    pub /: *mut *mut u8 greenfield; / 1 for greenfield,
    pub mpdu_start_spacing: u8,
    pub basic_rate_set: __le32,
    pub __packed: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_tx_rate_retry_policy {
    pub index: u8,
    pub short_retries: u8,
    pub long_retries: u8,
// BIT(2) - Terminate retries when Tx rate retry policy
// finishes.
// BIT(3) - Count initial frame transmission as part of
// rate retry counting but not as a retry
// attempt
//
    pub flags: u8,
    pub rate_recoveries: u8,
    pub reserved: [u8; 3],
    pub rate_count_indices: [__le32; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_set_tx_rate_retry_policy {
    pub num: u8,
    pub reserved: [u8; 3],
    pub tbl: [wsm_tx_rate_retry_policy; 8],
    pub __packed: },
    pub wsm_tx_rate_retry_policy): *mut *mut size_t size = 4 + arg->num  sizeof(struct,
// 4.32 SetEtherTypeDataFrameFilter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_ether_type_filter_hdr {
    pub /: *mut *mut u8 num; / Up to WSM_MAX_FILTER_ELEMENTS,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_ether_type_filter {
    pub /: *mut *mut u8 action; / WSM_FILTER_ACTION_XXX,
    pub reserved: u8,
    pub /: *mut *mut __le16 type; / Type of ethernet frame,
    pub __packed: },
    pub wsm_ether_type_filter): *mut *mut arg->num  sizeof(struct,
    pub size): arg,,
// 4.33 SetUDPPortDataFrameFilter
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_udp_port_filter_hdr {
    pub /: *mut *mut u8 num; / Up to WSM_MAX_FILTER_ELEMENTS,
    pub reserved: [u8; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_udp_port_filter {
    pub /: *mut *mut u8 action; / WSM_FILTER_ACTION_XXX,
    pub /: *mut *mut u8 type; / WSM_FILTER_PORT_TYPE_XXX,
    pub /: *mut *mut __le16 port; / Port number,
    pub __packed: },
    pub wsm_udp_port_filter): *mut *mut arg->num  sizeof(struct,
    pub size): arg,,
// Undocumented MIBs:
// 4.35 P2PDeviceInfo

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_p2p_device_type {
    pub category_id: __le16,
    pub oui: [u8; 4],
    pub subcategory_id: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_p2p_device_info {
    pub primaryDevice: wsm_p2p_device_type,
    pub reserved1: [u8; 3],
    pub devname_size: u8,
    pub local_devname: [u8; D11_MAX_SSID_LEN],
    pub reserved2: [u8; 3],
    pub num_secdev_supported: u8,
    pub secdevs: [wsm_p2p_device_type; ],
    pub __packed: },
// 4.36 SetWCDMABand - WO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_cdma_band {
    pub wcdma_band: u8,
    pub reserved: [u8; 3],
    pub __packed: },
// 4.37 GroupTxSequenceCounter - RO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_group_tx_seq {
    pub bits_47_16: __le32,
    pub bits_15_00: __le16,
    pub reserved: __le16,
    pub __packed: },
// 4.39 SetHtProtection - WO

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_ht_protection {
    pub flags: __le32,
    pub __packed: },
// 4.40 GPIO Command - R/W
pub const WSM_GPIO_COMMAND_SETUP: c_int = 0;
pub const WSM_GPIO_COMMAND_READ: c_int = 1;
pub const WSM_GPIO_COMMAND_WRITE: c_int = 2;
pub const WSM_GPIO_COMMAND_RESET: c_int = 3;
pub const WSM_GPIO_ALL_PINS: c_uint = 0xFF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_gpio_command {
    pub command: u8,
    pub pin: u8,
    pub config: __le16,
    pub __packed: },
// 4.41 TSFCounter - RO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_tsf_counter {
    pub tsf_counter: __le64,
    pub __packed: },
// 4.43 Keep alive period
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_keep_alive_period {
    pub period: __le16,
    pub reserved: [u8; 2],
    pub __packed: },
}

// BSSID filtering
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_set_bssid_filtering {
    pub filter: u8,
    pub reserved: [u8; 3],
    pub __packed: },
}

// Multicast filtering - 4.5
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_mib_multicast_filter {
    pub enable: __le32,
    pub num_addrs: __le32,
    pub macaddrs: [u8; WSM_MAX_GRP_ADDRTABLE_ENTRIES][ETH_ALEN],
    pub __packed: },
    pub sizeof(*fp)): *mut fp,,
// ARP IPv4 filtering - 4.10
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_mib_arp_ipv4_filter {
    pub enable: __le32,
    pub ipv4addrs: [__be32; WSM_MAX_ARP_IP_ADDRTABLE_ENTRIES],
    pub __packed: },
    pub sizeof(*fp)): *mut fp,,
// P2P Power Save Mode Info - 4.31
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_p2p_ps_modeinfo {
    pub opp_ps_ct_window: u8,
    pub count: u8,
    pub reserved: u8,
    pub dtim_count: u8,
    pub duration: __le32,
    pub interval: __le32,
    pub start_time: __le32,
    pub __packed: },
    pub sizeof(*mi)): *mut mi,,
    pub sizeof(*mi)): *mut mi,,
// UseMultiTxConfMessage
    pub 0: __le32 arg = enabled ? __cpu_to_le32(1) :,
    pub sizeof(arg)): &arg,,
// 4.26 SetUpasdInformation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_uapsd_info {
    pub uapsd_flags: __le16,
    pub min_auto_trigger_interval: __le16,
    pub max_auto_trigger_interval: __le16,
    pub auto_trigger_step: __le16,
}

// 4.22 OverrideInternalTxRate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_override_internal_txrate {
    pub internalTxRate: u8,
    pub nonErpInternalTxRate: u8,
    pub reserved: [u8; 2],
    pub __packed: },
    pub sizeof(*arg)): *mut arg,,
// ********************************************************************
// WSM TX port control
    pub priv): *mut void wsm_lock_tx(struct cw1200_common,
    pub priv): *mut void wsm_lock_tx_async(struct cw1200_common,
    pub priv): *mut bool wsm_flush_tx(struct cw1200_common,
    pub priv): *mut void wsm_unlock_tx(struct cw1200_common,
// ********************************************************************
// WSM / BH API
    pub len): *mut *mut *mut int wsm_handle_exception(struct cw1200_common priv, u8 data, size_t,
    pub skb_p): *mut sk_buff,
// ********************************************************************
// wsm_buf API
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_buf {
    pub begin: *mut u8,
    pub data: *mut u8,
    pub end: *mut u8,
}

extern "C" {
    pub fn wsm_buf_init(buf: *mut wsm_buf);
}
extern "C" {
    pub fn wsm_buf_deinit(buf: *mut wsm_buf);
}
// ********************************************************************
// wsm_cmd API
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wsm_cmd {
    pub /: *mut *mut spinlock_t lock; / Protect structure from multiple access,
    pub done: c_int,
    pub ptr: *mut u8,
    pub len: usize,
    pub arg: *mut c_void,
    pub ret: c_int,
    pub cmd: u16,
}

// ********************************************************************
// WSM TX buffer access
extern "C" {
    pub fn wsm_txed(priv: *mut cw1200_common, data: *mut u8);
}
// ********************************************************************
// Queue mapping: WSM <---> linux
// Linux: VO VI BE BK
// WSM:   BE BK VI VO
