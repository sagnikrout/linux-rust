//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath5k/rfbuffer.h
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
// RF Buffer handling functions
//
// Copyright (c) 2009 Nick Kossifidis <mickflemm@gmail.com>
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
// DOC: RF Buffer registers
//
// There are some special registers on the RF chip
// that control various operation settings related mostly to
// the analog parts (channel, gain adjustment etc).
//
// We don't write on those registers directly but
// we send a data packet on the chip, using a special register,
// that holds all the settings we need. After we've sent the
// data packet, we write on another special register to notify hw
// to apply the settings. This is done so that control registers
// can be dynamically programmed during operation and the settings
// are applied faster on the hw.
//
// We call each data packet an "RF Bank" and all the data we write
// (all RF Banks) "RF Buffer". This file holds initial RF Buffer
// data for the different RF chips, and various info to match RF
// Buffer offsets with specific RF registers so that we can access
// them. We tweak these settings on rfregs_init function.
//
// Also check out reg.h and U.S. Patent 6677779 B1 (about buffer
// registers and control registers):
//
// https://www.google.com/patents?id=qNURAAAAEBAJ
//
// struct ath5k_ini_rfbuffer - Initial RF Buffer settings
// @rfb_bank: RF Bank number
// @rfb_ctrl_register: RF Buffer control register
// @rfb_mode_data: RF Buffer data for each mode
//
// Struct to hold default mode specific RF
// register values (RF Banks) for each chip.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_ini_rfbuffer {
    pub rfb_bank: u8,
    pub rfb_ctrl_register: u16,
    pub rfb_mode_data: [u32; 3],
}

//
// struct ath5k_rfb_field - An RF Buffer field (register/value)
// @len: Field length
// @pos: Offset on the raw packet
// @col: Used for shifting
//
// Struct to hold RF Buffer field
// infos used to access certain RF
// analog registers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_rfb_field {
    pub len: u8,
    pub pos: u16,
    pub col: u8,
}

//
// struct ath5k_rf_reg - RF analog register definition
// @bank: RF Buffer Bank number
// @index: Register's index on ath5k_rf_regx_idx
// @field: The &struct ath5k_rfb_field
//
// We use this struct to define the set of RF registers
// on each chip that we want to tweak. Some RF registers
// are common between different chip versions so this saves
// us space and complexity because we can refer to an rf
// register by it's index no matter what chip we work with
// as long as it has that register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath5k_rf_reg {
    pub bank: u8,
    pub index: u8,
    pub field: ath5k_rfb_field,
}

//
// enum ath5k_rf_regs_idx - Map RF registers to indexes
//
// We do this to handle common bits and make our
// life easier by using an index for each register
// instead of a full rfb_field
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ath5k_rf_regs_idx {
// BANK 2
    AR5K_RF_TURBO = 0,
// BANK 6
    AR5K_RF_OB_2GHZ,
    AR5K_RF_OB_5GHZ,
    AR5K_RF_DB_2GHZ,
    AR5K_RF_DB_5GHZ,
    AR5K_RF_FIXED_BIAS_A,
    AR5K_RF_FIXED_BIAS_B,
    AR5K_RF_PWD_XPD,
    AR5K_RF_XPD_SEL,
    AR5K_RF_XPD_GAIN,
    AR5K_RF_PD_GAIN_LO,
    AR5K_RF_PD_GAIN_HI,
    AR5K_RF_HIGH_VC_CP,
    AR5K_RF_MID_VC_CP,
    AR5K_RF_LOW_VC_CP,
    AR5K_RF_PUSH_UP,
    AR5K_RF_PAD2GND,
    AR5K_RF_XB2_LVL,
    AR5K_RF_XB5_LVL,
    AR5K_RF_PWD_ICLOBUF_2G,
    AR5K_RF_PWD_84,
    AR5K_RF_PWD_90,
    AR5K_RF_PWD_130,
    AR5K_RF_PWD_131,
    AR5K_RF_PWD_132,
    AR5K_RF_PWD_136,
    AR5K_RF_PWD_137,
    AR5K_RF_PWD_138,
    AR5K_RF_PWD_166,
    AR5K_RF_PWD_167,
    AR5K_RF_DERBY_CHAN_SEL_MODE,
// BANK 7
    AR5K_RF_GAIN_I,
    AR5K_RF_PLO_SEL,
    AR5K_RF_RFGAIN_SEL,
    AR5K_RF_RFGAIN_STEP,
    AR5K_RF_WAIT_S,
    AR5K_RF_WAIT_I,
    AR5K_RF_MAX_TIME,
    AR5K_RF_MIXVGA_OVR,
    AR5K_RF_MIXGAIN_OVR,
    AR5K_RF_MIXGAIN_STEP,
    AR5K_RF_PD_DELAY_A,
    AR5K_RF_PD_DELAY_B,
    AR5K_RF_PD_DELAY_XR,
    AR5K_RF_PD_PERIOD_A,
    AR5K_RF_PD_PERIOD_B,
    AR5K_RF_PD_PERIOD_XR,
}

// \
// RF5111 (Sombrero)
// BANK 2				len  pos col

// BANK 6				len  pos col

// Access to PWD registers

// BANK 7				len  pos col

// Only on AR5212 BaseBand and up

// Default mode specific settings
// BANK / C.R.     A/XR         B           G
// \
// RF5112/RF2112 (Derby)
// BANK 2 (Common)			len  pos col

// BANK 7 (Common)			len  pos col

// RFX112 (Derby 1)
// BANK 6				len  pos col

// Access to PWD registers

// Default mode specific settings
// BANK / C.R.     A/XR         B           G
// RFX112A (Derby 2)
// BANK 6				len  pos col

// Access to PWD registers

// Voltage regulators

// Power consumption

// Default mode specific settings
// BANK / C.R.     A/XR         B           G
// \
// RF2413 (Griffin)
// BANK 2				len  pos col

// BANK 6				len  pos col

// Default mode specific settings
// XXX: a/aTurbo ???
//
// BANK / C.R.     A/XR         B           G
// \
// RF2315/RF2316 (Cobra SoC)
// BANK 2				len  pos col

// BANK 6				len  pos col

// Default mode specific settings
// BANK / C.R.     A/XR         B           G
// \
// RF5413/RF5424 (Eagle/Condor)
// BANK 6				len  pos col

// Default mode specific settings
// BANK / C.R.     A/XR         B           G
// \
// RF2425/RF2417 (Swan/Nala)
// AR2317 (Spider SoC)
// BANK 2				len  pos col

// BANK 6				len  pos col

// Default mode specific settings
//
// BANK / C.R.     A/XR         B           G
//
// TODO: Handle the few differences with swan during
// bank modification and get rid of this
//
// BANK / C.R.     A/XR         B           G
//
// TODO: Handle the few differences with swan during
// bank modification and get rid of this
//
// BANK / C.R.     A/XR         B           G
