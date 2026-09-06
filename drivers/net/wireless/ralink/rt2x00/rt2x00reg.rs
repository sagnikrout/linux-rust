//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ralink/rt2x00/rt2x00reg.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// RX crypto status
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rx_crypto {
    RX_CRYPTO_SUCCESS = 0,
    RX_CRYPTO_FAIL_ICV = 1,
    RX_CRYPTO_FAIL_MIC = 2,
    RX_CRYPTO_FAIL_KEY = 3,
}

//
// Antenna values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum antenna {
    ANTENNA_SW_DIVERSITY = 0,
    ANTENNA_A = 1,
    ANTENNA_B = 2,
    ANTENNA_HW_DIVERSITY = 3,
}

//
// Led mode values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum led_mode {
    LED_MODE_DEFAULT = 0,
    LED_MODE_TXRX_ACTIVITY = 1,
    LED_MODE_SIGNAL_STRENGTH = 2,
    LED_MODE_ASUS = 3,
    LED_MODE_ALPHA = 4,
}

//
// TSF sync values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tsf_sync {
    TSF_SYNC_NONE = 0,
    TSF_SYNC_INFRA = 1,
    TSF_SYNC_ADHOC = 2,
    TSF_SYNC_AP_NONE = 3,
}

//
// Device states
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dev_state {
    STATE_DEEP_SLEEP = 0,
    STATE_SLEEP = 1,
    STATE_STANDBY = 2,
    STATE_AWAKE = 3,

//
// Additional device states, these values are
// not strict since they are not directly passed
// into the device.
//
    STATE_RADIO_ON,
    STATE_RADIO_OFF,
    STATE_RADIO_IRQ_ON,
    STATE_RADIO_IRQ_OFF,
}

//
// IFS backoff values
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ifs {
    IFS_BACKOFF = 0,
    IFS_SIFS = 1,
    IFS_NEW_BACKOFF = 2,
    IFS_NONE = 3,
}

//
// IFS backoff values for HT devices
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum txop {
    TXOP_HTTXOP = 0,
    TXOP_PIFS = 1,
    TXOP_SIFS = 2,
    TXOP_BACKOFF = 3,
}

//
// Cipher types for hardware encryption
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cipher {
    CIPHER_NONE = 0,
    CIPHER_WEP64 = 1,
    CIPHER_WEP128 = 2,
    CIPHER_TKIP = 3,
    CIPHER_AES = 4,
//
// The following fields were added by rt61pci and rt73usb.
//
    CIPHER_CKIP64 = 5,
    CIPHER_CKIP128 = 6,
    CIPHER_TKIP_NO_MIC = 7, /* Don't send to device */

//
// Max cipher type.
// Note that CIPHER_NONE isn't counted, and CKIP64 and CKIP128
// are excluded due to limitations in mac80211.
//
    CIPHER_MAX = 4,
}

//
// Rate modulations
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rate_modulation {
    RATE_MODE_CCK = 0,
    RATE_MODE_OFDM = 1,
    RATE_MODE_HT_MIX = 2,
    RATE_MODE_HT_GREENFIELD = 3,
}

//
// Firmware validation error codes
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum firmware_errors {
    FW_OK,
    FW_BAD_CRC,
    FW_BAD_LENGTH,
    FW_BAD_VERSION,
}

//
// Register handlers.
// We store the position of a register field inside a field structure,
// This will simplify the process of setting and reading a certain field
// inside the register while making sure the process remains byte order safe.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_field8 {
    pub bit_offset: u8,
    pub bit_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_field16 {
    pub bit_offset: u16,
    pub bit_mask: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt2x00_field32 {
    pub bit_offset: u32,
    pub bit_mask: u32,
}

//
// Power of two check, this will check
// if the mask that has been given contains and contiguous set of bits.
// Note that we cannot use the is_power_of_2() function since this
// check must be done at compile-time.
//

//
// Macros to find first set bit in a variable.
// These macros behave the same as the __ffs() functions but
// the most important difference that this is done during
// compile-time rather then run-time.
//

//
// This macro will check the requirements for the FIELD{8,16,32} macros
// The mask should be a constant non-zero contiguous set of bits which
// does not exceed the given typelimit.
//

// (__reg) &= ~((__field).bit_mask);	\
// (__reg) |= ((__value) <<		\

