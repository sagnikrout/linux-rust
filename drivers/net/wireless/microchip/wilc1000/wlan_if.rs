//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/microchip/wilc1000/wlan_if.h
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
// Copyright (c) 2012 - 2018 Microchip Technology Inc., and its subsidiaries.
// All rights reserved.
//

pub const WILC_MAX_ASSOC_RESP_FRAME_SIZE: c_int = 512;
//
// Wlan Configuration ID
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bss_types {
    WILC_FW_BSS_TYPE_INFRA = 0,
    WILC_FW_BSS_TYPE_INDEPENDENT,
    WILC_FW_BSS_TYPE_AP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bus_acquire {
    WILC_BUS_ACQUIRE_ONLY = 0,
    WILC_BUS_ACQUIRE_AND_WAKEUP = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bus_release {
    WILC_BUS_RELEASE_ONLY = 0,
    WILC_BUS_RELEASE_ALLOW_SLEEP = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum authtype {
    WILC_FW_AUTH_OPEN_SYSTEM = 1,
    WILC_FW_AUTH_SHARED_KEY = 2,
    WILC_FW_AUTH_ANY = 3,
    WILC_FW_AUTH_IEEE8021 = 5,
    WILC_FW_AUTH_SAE = 7,
    WILC_FW_AUTH_IEE8021X_SHA256 = 9,
    WILC_FW_AUTH_OPEN_SYSTEM_SHA256 = 13
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mfptype {
    WILC_FW_MFP_NONE = 0x0,
    WILC_FW_MFP_OPTIONAL = 0x1,
    WILC_FW_MFP_REQUIRED = 0x2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum site_survey {
    WILC_FW_SITE_SURVEY_1CH = 0,
    WILC_FW_SITE_SURVEY_ALL_CH = 1,
    WILC_FW_SITE_SURVEY_OFF = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum wid_type {
    WID_CHAR		= 0,
    WID_SHORT		= 1,
    WID_INT			= 2,
    WID_STR			= 3,
    WID_BIN_DATA		= 4,
    WID_BIN			= 5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wid {
    pub id: u16,
    pub type: wid_type,
    pub size: i32,
    pub val: *mut i8,
}

//
// BSS Type
// -----------------------------------------------------------
// Configuration : Infrastructure   Independent   Access Point
// Values to set :         0               1            2
// -----------------------------------------------------------
//
// Transmit Rate
// -----------------------------------------------------------
// Configuration : 1  2  5.5  11  6  9  12  18  24  36  48  54
// Values to set : 1  2    5  11  6  9  12  18  24  36  48  54
// -----------------------------------------------------------
//
// Channel
// -----------------------------------------------------------
// Configuration(g) : 1  2  3  4  5  6  7  8  9 10 11 12 13 14
// Values to set    : 1  2  3  4  5  6  7  8  9 10 11 12 13 14
// -----------------------------------------------------------
//
// Preamble
// -----------------------------------------------------------
// Configuration :    short    long      Auto
// Values to set :       0       1         2
// -----------------------------------------------------------
//
// 11g operating mode (ignored if 11g not present)
// -----------------------------------------------------------
// Configuration :   HighPerf  Compat(RSet #1) Compat(RSet #2)
// Values to set :          1               2               3
// -----------------------------------------------------------
//
// Mac status (response only)
// -----------------------------------------------------------
// Configuration :   disconnect  connect
// Values to get :          0       1
// -----------------------------------------------------------
//
// Scan type
// -----------------------------------------------------------
// Configuration :   Passive Scanning   Active Scanning
// Values to set :                  0                 1
// -----------------------------------------------------------
//
// Key Id (WEP default key Id)
// -----------------------------------------------------------
// Configuration :   Any value between 0 to 3
// Values to set :   Same value. Default is 0
// -----------------------------------------------------------
//
// QoS Enable
// -----------------------------------------------------------
// Configuration :   QoS Disable   WMM Enable
// Values to set :   0             1
// -----------------------------------------------------------
//
// Power Management
// -----------------------------------------------------------
// Configuration : NO_POWERSAVE MIN_POWERSAVE MAX_POWERSAVE
// Values to set : 0            1             2
// -----------------------------------------------------------
//
// WEP/802 11I Configuration
// -----------------------------------------------------------
// Configuration:Disable WP40 WP104 WPA-AES WPA-TKIP RSN-AES RSN-TKIP
// Values (0x)  :   00     03   07     29       49       31      51
// Configuration:WPA-AES+TKIP RSN-AES+TKIP
// Values (0x)  :      69        71
// -----------------------------------------------------------
//
// WEP Configuration: Used in BSS STA mode only when WEP is enabled
// -----------------------------------------------------------
// Configuration : Open System Shared Key Any Type | 802.1x Auth
// Values (0x)   :    01             02         03 |    BIT2
// -----------------------------------------------------------
//
// Site Survey Type
// -----------------------------------------------------------
// Configuration       :  Values to set
// Survey 1 Channel    :  0
// survey all Channels :  1
// Disable Site Survey :  2
// -----------------------------------------------------------
//
// Listen Interval
// -----------------------------------------------------------
// Configuration :   Any value between 1 to 255
// Values to set :   Same value. Default is 3
// -----------------------------------------------------------
//
// DTIM Period
// -----------------------------------------------------------
// Configuration :   Any value between 1 to 255
// Values to set :   Same value. Default is 3
// -----------------------------------------------------------
//
// ACK Policy
// -----------------------------------------------------------
// Configuration :   Normal Ack            No Ack
// Values to set :       0                   1
// -----------------------------------------------------------
//
// Reset MAC (Set only)
// -----------------------------------------------------------
// Configuration :   Don't Reset	Reset	No Request
// Values to set :       0               1	    2
// -----------------------------------------------------------
//
// Broadcast SSID Option: Setting this will adhere to "" SSID element
// -----------------------------------------------------------
// Configuration :   Enable             Disable
// Values to set :   1                  0
// -----------------------------------------------------------
//
// Disconnect (Station)
// -----------------------------------------------------------
// Configuration :   Association ID
// Values to set :   Association ID
// -----------------------------------------------------------
//
// 11a Tx Power Level
// -----------------------------------------------------------
// Configuration : Sets TX Power (Higher the value greater the power)
// Values to set : Any value between 0 and 63 (inclusive Default 48)
// -----------------------------------------------------------
//
// Group Key Update Policy Selection
// -----------------------------------------------------------
// Configuration : Disabled timeBased packetBased timePacketBased
// Values to set :   1            2          3              4
// -----------------------------------------------------------
//
// Allow Short Slot
// -----------------------------------------------------------
// Configuration : Disallow Short Slot      Allow Short Slot
// (Enable Only Long Slot) (Enable Short Slot if applicable)
// Values to set :    0         1
// -----------------------------------------------------------
//
// 11b Tx Power Level
// -----------------------------------------------------------
// Configuration : Sets TX Power (Higher the value greater the power)
// Values to set : Any value between 0 and 63 (inclusive Default 48)
// -----------------------------------------------------------
//
// Scan Request
// -----------------------------------------------------------
// Configuration : Request default scan
// Values to set : 0
// -----------------------------------------------------------
//
// Rssi (get only)
// -----------------------------------------------------------
// Configuration :
// Values to get : Rssi value
// -----------------------------------------------------------
//
// Join Request
// -----------------------------------------------------------
// Configuration : Request to join
// Values to set : index of scan result
// -----------------------------------------------------------
//
// Enable User Control of TX Power
// -----------------------------------------------------------
// Configuration : Disable                  Enable
// Values to set :    0                       1
// -----------------------------------------------------------
//
// Enable Auto RX Sensitivity feature
// -----------------------------------------------------------
// Configuration : Disable                  Enable
// Values to set :    0                       1
// -----------------------------------------------------------
//
// Receive Buffer Based Ack
// -----------------------------------------------------------
// Configuration : Disable                  Enable
// Values to set :    0                       1
// -----------------------------------------------------------
//
// Scan Filter
// -----------------------------------------------------------
// Configuration : Class       No filter   AP only   Station Only
// Values to set :                0           1           2
// Configuration : Priority    High Rssi   Low Rssi     Detect
// Values to set :                0          0x4         0x0
// Configuration : Channel     filter off  filter on
// Values to set :                0          0x10
// -----------------------------------------------------------
//
// Link Loss Threshold (measure in the beacon period)
// -----------------------------------------------------------
// Configuration : Any value between 10 and 254(Set to 255 disable)
// Values to set : Same value. Default is 10
// -----------------------------------------------------------
//
// NMAC Character WID list
//
// Protection mode for MAC
// -----------------------------------------------------------
// Configuration :  Auto  No protection  ERP    HT    GF
// Values to set :  0     1              2      3     4
// -----------------------------------------------------------
//
// ERP Protection type for MAC
// -----------------------------------------------------------
// Configuration :  Self-CTS   RTS-CTS
// Values to set :  0          1
// -----------------------------------------------------------
//
// HT Option Enable
// -----------------------------------------------------------
// Configuration :   HT Enable          HT Disable
// Values to set :   1                  0
// -----------------------------------------------------------
//
// 11n Operating mode (Note that 11g operating mode will also be
// used in addition to this, if this is set to HT Mixed mode)
// -----------------------------------------------------------
// Configuration :  HT Mixed  HT Only-20MHz   HT Only-20/40MHz
// Values to set :     1         2               3
// -----------------------------------------------------------
//
// 11n OBSS non-HT STA Detection flag
// -----------------------------------------------------------
// Configuration :  Do not detect
// Values to set :  0
// Configuration :  Detect, do not protect or report
// Values to set :  1
// Configuration :  Detect, protect and do not report
// Values to set :  2
// Configuration :  Detect, protect and report to other BSS
// Values to set :  3
// -----------------------------------------------------------
//
// 11n HT Protection Type
// -----------------------------------------------------------
// Configuration :  RTS-CTS   First Frame Exchange at non-HT-rate
// Values to set :  0         1
// Configuration :  LSIG TXOP First Frame Exchange in Mixed Fmt
// Values to set :  2         3
// -----------------------------------------------------------
//
// 11n RIFS Protection Enable Flag
// -----------------------------------------------------------
// Configuration :  Disable    Enable
// Values to set :  0          1
// -----------------------------------------------------------
//
// SMPS Mode
// -----------------------------------------------------------
// Configuration :  Static   Dynamic   MIMO (Power Save Disabled)
// Values to set :  1        2         3
// -----------------------------------------------------------
//
// Current transmit MCS
// -----------------------------------------------------------
// Configuration :  MCS Index for data rate
// Values to set :  0 to 7
// -----------------------------------------------------------
//
// 11n Short GI Enable Flag
// -----------------------------------------------------------
// Configuration :  Disable    Enable
// Values to set :  0          1
// -----------------------------------------------------------
//
// 11n RIFS Enable Flag
// -----------------------------------------------------------
// Configuration :  Disable    Enable
// Values to set :  0          1
// -----------------------------------------------------------
//
// TX Abort Feature
// -----------------------------------------------------------
// Configuration :  Disable Self CTS    Enable Self CTS
// Values to set :             0                      1
// Configuration :  Disable TX Abort    Enable TX Abort
// Values to set :             2                      3
// Configuration :  Enable HW TX Abort Enable SW TX Abort
// Values to set :             4                      5
// -----------------------------------------------------------
//
// Immediate Block-Ack Support
// -----------------------------------------------------------
// Configuration : Disable                  Enable
// Values to set :    0                       1
// -----------------------------------------------------------
//
// TXOP Disable Flag
// -----------------------------------------------------------
// Configuration : Disable                  Enable
// Values to set :    1                        0
// -----------------------------------------------------------
//
// Custom Character WID list
// SCAN Complete notification WID
// EMAC Short WID list
// RTS Threshold
//
// -----------------------------------------------------------
// Configuration :   Any value between 256 to 2347
// Values to set :   Same value. Default is 2347
// -----------------------------------------------------------
//
// Fragmentation Threshold
// -----------------------------------------------------------
// Configuration :   Any value between 256 to 2346
// Values to set :   Same value. Default is 2346
// -----------------------------------------------------------
//
// NMAC Short WID list
// Custom Short WID list
// EMAC Integer WID list
// NMAC Integer WID list
// Custom Integer WID list
// EMAC String WID list
// NMAC String WID list
// Custom String WID list
// EMAC Binary WID list
// NMAC Binary WID list
// Miscellaneous WIDs
