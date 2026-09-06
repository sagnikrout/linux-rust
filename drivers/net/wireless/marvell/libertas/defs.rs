//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/marvell/libertas/defs.h
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
// This header file contains global constant/enum definitions,
// global variable declaration.
//

// Macro flag: #define DEBUG
// Macro flag: #define PROC_DEBUG

pub const LBS_DEB_ENTER: c_uint = 0x00000001;
pub const LBS_DEB_LEAVE: c_uint = 0x00000002;
pub const LBS_DEB_MAIN: c_uint = 0x00000004;
pub const LBS_DEB_NET: c_uint = 0x00000008;
pub const LBS_DEB_MESH: c_uint = 0x00000010;
pub const LBS_DEB_WEXT: c_uint = 0x00000020;
pub const LBS_DEB_IOCTL: c_uint = 0x00000040;
pub const LBS_DEB_SCAN: c_uint = 0x00000080;
pub const LBS_DEB_ASSOC: c_uint = 0x00000100;
pub const LBS_DEB_JOIN: c_uint = 0x00000200;
pub const LBS_DEB_11D: c_uint = 0x00000400;
pub const LBS_DEB_DEBUGFS: c_uint = 0x00000800;
pub const LBS_DEB_ETHTOOL: c_uint = 0x00001000;
pub const LBS_DEB_HOST: c_uint = 0x00002000;
pub const LBS_DEB_CMD: c_uint = 0x00004000;
pub const LBS_DEB_RX: c_uint = 0x00008000;
pub const LBS_DEB_TX: c_uint = 0x00010000;
pub const LBS_DEB_USB: c_uint = 0x00020000;
pub const LBS_DEB_CS: c_uint = 0x00040000;
pub const LBS_DEB_FW: c_uint = 0x00080000;
pub const LBS_DEB_THREAD: c_uint = 0x00100000;
pub const LBS_DEB_HEX: c_uint = 0x00200000;
pub const LBS_DEB_SDIO: c_uint = 0x00400000;
pub const LBS_DEB_SYSFS: c_uint = 0x00800000;
pub const LBS_DEB_SPI: c_uint = 0x01000000;
pub const LBS_DEB_CFG80211: c_uint = 0x02000000;

// Buffer Constants
// The size of SQ memory PPA, DPA are 8 DWORDs, that keep the physical
// addresses of TxPD buffers. Station has only 8 TxPD available, Whereas
// driver has more local TxPDs. Each TxPD on the host memory is associated
// with a Tx control node. The driver maintains 8 RxPD descriptors for
// station firmware to store Rx packet information.
//
// Current version of MAC has a 32x6 multicast address buffer.
//
// 802.11b can have up to  14 channels, the driver keeps the
// BSSID(MAC address) of each APs or Ad hoc stations it has sensed.
//
pub const MRVDRV_MAX_MULTICAST_LIST_SIZE: c_int = 32;
pub const LBS_NUM_CMD_BUFFERS: c_int = 10;

pub const MRVDRV_MAX_CHANNEL_SIZE: c_int = 14;
pub const MRVDRV_ASSOCIATION_TIME_OUT: c_int = 255;
pub const MRVDRV_SNAP_HEADER_LEN: c_int = 8;
pub const LBS_UPLD_SIZE: c_int = 2312;
pub const DEV_NAME_LEN: c_int = 32;
// Wake criteria for HOST_SLEEP_CFG command
pub const EHS_WAKE_ON_BROADCAST_DATA: c_uint = 0x0001;
pub const EHS_WAKE_ON_UNICAST_DATA: c_uint = 0x0002;
pub const EHS_WAKE_ON_MAC_EVENT: c_uint = 0x0004;
pub const EHS_WAKE_ON_MULTICAST_DATA: c_uint = 0x0008;
pub const EHS_REMOVE_WAKEUP: c_uint = 0xFFFFFFFF;
// Wake rules for Host_Sleep_CFG command
pub const WOL_RULE_NET_TYPE_INFRA_OR_IBSS: c_uint = 0x00;
pub const WOL_RULE_NET_TYPE_MESH: c_uint = 0x10;
pub const WOL_RULE_ADDR_TYPE_BCAST: c_uint = 0x01;
pub const WOL_RULE_ADDR_TYPE_MCAST: c_uint = 0x08;
pub const WOL_RULE_ADDR_TYPE_UCAST: c_uint = 0x02;
pub const WOL_RULE_OP_AND: c_uint = 0x01;
pub const WOL_RULE_OP_OR: c_uint = 0x02;
pub const WOL_RULE_OP_INVALID: c_uint = 0xFF;
pub const WOL_RESULT_VALID_CMD: c_int = 0;
pub const WOL_RESULT_NOSPC_ERR: c_int = 1;
pub const WOL_RESULT_EEXIST_ERR: c_int = 2;
// Misc constants
// This section defines 802.11 specific contants
pub const MRVDRV_MAX_BSS_DESCRIPTS: c_int = 16;
pub const MRVDRV_MAX_REGION_CODE: c_int = 6;
pub const MRVDRV_DEFAULT_LISTEN_INTERVAL: c_int = 10;
pub const MRVDRV_CHANNELS_PER_SCAN: c_int = 4;
pub const MRVDRV_MAX_CHANNELS_PER_SCAN: c_int = 14;
pub const MRVDRV_MIN_BEACON_INTERVAL: c_int = 20;
pub const MRVDRV_MAX_BEACON_INTERVAL: c_int = 1000;
pub const MRVDRV_BEACON_INTERVAL: c_int = 100;
pub const MARVELL_MESH_IE_LENGTH: c_int = 9;
//
// Values used to populate the struct mrvl_mesh_ie.  The only time you need this
// is when enabling the mesh using CMD_MESH_CONFIG.
//
pub const MARVELL_MESH_IE_TYPE: c_int = 4;
pub const MARVELL_MESH_IE_SUBTYPE: c_int = 0;
pub const MARVELL_MESH_IE_VERSION: c_int = 0;
pub const MARVELL_MESH_PROTO_ID_HWMP: c_int = 0;
pub const MARVELL_MESH_METRIC_ID: c_int = 0;
pub const MARVELL_MESH_CAPABILITY: c_int = 0;
// INT status Bit Definition
pub const MRVDRV_TX_DNLD_RDY: c_uint = 0x0001;
pub const MRVDRV_RX_UPLD_RDY: c_uint = 0x0002;
pub const MRVDRV_CMD_DNLD_RDY: c_uint = 0x0004;
pub const MRVDRV_CMD_UPLD_RDY: c_uint = 0x0008;
pub const MRVDRV_CARDEVENT: c_uint = 0x0010;
// Automatic TX control default levels
pub const POW_ADAPT_DEFAULT_P0: c_int = 13;
pub const POW_ADAPT_DEFAULT_P1: c_int = 15;
pub const POW_ADAPT_DEFAULT_P2: c_int = 18;
pub const TPC_DEFAULT_P0: c_int = 5;
pub const TPC_DEFAULT_P1: c_int = 10;
pub const TPC_DEFAULT_P2: c_int = 13;
// TxPD status
//
// Station firmware use TxPD status field to report final Tx transmit
// result, Bit masks are used to present combined situations.
//
pub const MRVDRV_TxPD_POWER_MGMT_NULL_PACKET: c_uint = 0x01;
pub const MRVDRV_TxPD_POWER_MGMT_LAST_PACKET: c_uint = 0x08;
// Tx mesh flag
//
// Currently we are using normal WDS flag as mesh flag.
// TODO: change to proper mesh flag when MAC understands it.
//

// Mesh interface ID
pub const MESH_IFACE_ID: c_uint = 0x0001;
// Mesh id should be in bits 14-13-12
pub const MESH_IFACE_BIT_OFFSET: c_uint = 0x000c;
// Mesh enable bit in FW capability

// FW definition from Marvell v4

// FW definition from Marvell v5

// FW definition from Marvell v10

// FW major revision definition

// RxPD status
pub const MRVDRV_RXPD_STATUS_OK: c_uint = 0x0001;
// RxPD status - Received packet types
// Rx mesh flag
//
// Currently we are using normal WDS flag as mesh flag.
// TODO: change to proper mesh flag when MAC understands it.
//

// RSSI-related defines
//
// RSSI constants are used to implement 802.11 RSSI threshold
// indication. if the Rx packet signal got too weak for 5 consecutive
// times, miniport driver (driver) will report this event to wrapper
//

// RTS/FRAG related defines
pub const MRVDRV_RTS_MIN_VALUE: c_int = 0;
pub const MRVDRV_RTS_MAX_VALUE: c_int = 2347;
pub const MRVDRV_FRAG_MIN_VALUE: c_int = 256;
pub const MRVDRV_FRAG_MAX_VALUE: c_int = 2346;
// This is for firmware specific length
pub const EXTRA_LEN: c_int = 36;

pub const KEY_LEN_WPA_AES: c_int = 16;
pub const KEY_LEN_WPA_TKIP: c_int = 32;
pub const KEY_LEN_WEP_104: c_int = 13;
pub const KEY_LEN_WEP_40: c_int = 5;
pub const RF_ANTENNA_1: c_uint = 0x1;
pub const RF_ANTENNA_2: c_uint = 0x2;
pub const RF_ANTENNA_AUTO: c_uint = 0xFFFF;

pub const MAX_RATES: c_int = 14;
pub const MAX_LEDS: c_int = 8;
// Global Variable Declaration
// ENUM definition
// SNRNF_TYPE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SNRNF_TYPE {
    TYPE_BEACON = 0,
    TYPE_RXPD,
    MAX_TYPE_B
}

// SNRNF_DATA
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SNRNF_DATA {
    TYPE_NOAVG = 0,
    TYPE_AVG,
    MAX_TYPE_AVG
}

// LBS_802_11_POWER_MODE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LBS_802_11_POWER_MODE {
    LBS802_11POWERMODECAM,
    LBS802_11POWERMODEMAX_PSP,
    LBS802_11POWERMODEFAST_PSP,
// not a real mode, defined as an upper bound
    LBS802_11POWEMODEMAX
}

// PS_STATE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PS_STATE {
    PS_STATE_FULL_POWER,
    PS_STATE_AWAKE,
    PS_STATE_PRE_SLEEP,
    PS_STATE_SLEEP
}

// DNLD_STATE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum DNLD_STATE {
    DNLD_RES_RECEIVED,
    DNLD_DATA_SENT,
    DNLD_CMD_SENT,
    DNLD_BOOTCMD_SENT,
}

// LBS_MEDIA_STATE
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LBS_MEDIA_STATE {
    LBS_CONNECTED,
    LBS_DISCONNECTED
}

// LBS_802_11_PRIVACY_FILTER
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum LBS_802_11_PRIVACY_FILTER {
    LBS802_11PRIVFILTERACCEPTALL,
    LBS802_11PRIVFILTER8021XWEP
}

// mv_ms_type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mv_ms_type {
    MVMS_DAT = 0,
    MVMS_CMD = 1,
    MVMS_TXDONE = 2,
    MVMS_EVENT
}

// KEY_TYPE_ID
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KEY_TYPE_ID {
    KEY_TYPE_ID_WEP = 0,
    KEY_TYPE_ID_TKIP,
    KEY_TYPE_ID_AES
}

// KEY_INFO_WPA (applies to both TKIP and AES/CCMP)
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum KEY_INFO_WPA {
    KEY_INFO_WPA_MCAST = 0x01,
    KEY_INFO_WPA_UNICAST = 0x02,
    KEY_INFO_WPA_ENABLED = 0x04
}

// Default values for fwt commands.
pub const FWT_DEFAULT_METRIC: c_int = 0;
pub const FWT_DEFAULT_DIR: c_int = 1;
// Default Rate, 11Mbps
pub const FWT_DEFAULT_RATE: c_int = 3;
pub const FWT_DEFAULT_SSN: c_uint = 0xffffffff;
pub const FWT_DEFAULT_DSN: c_int = 0;
pub const FWT_DEFAULT_HOPCOUNT: c_int = 0;
pub const FWT_DEFAULT_TTL: c_int = 0;
pub const FWT_DEFAULT_EXPIRATION: c_int = 0;
pub const FWT_DEFAULT_SLEEPMODE: c_int = 0;
pub const FWT_DEFAULT_SNR: c_int = 0;
