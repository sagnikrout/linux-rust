//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/sma1307.h
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
// sma1307.h -- sma1307 ALSA SoC Audio driver
//
// Copyright 2024 Iron Device Corporation
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License version 2 as
// published by the Free Software Foundation.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sma1307_fault {
    SMA1307_FAULT_OT1,
    SMA1307_FAULT_OT2,
    SMA1307_FAULT_UVLO,
    SMA1307_FAULT_OVP_BST,
    SMA1307_FAULT_OCP_SPK,
    SMA1307_FAULT_OCP_BST,
    SMA1307_FAULT_CLK
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sma1307_mode {
    SMA1307_MONO_MODE,
    SMA1307_LEFT_MODE,
    SMA1307_RIGHT_MODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sma1307_sdo_mode {
    SMA1307_OUT_DATA_ONE_48K,
    SMA1307_OUT_DATA_TWO_48K,
    SMA1307_OUT_DATA_TWO_24K,
    SMA1307_OUT_CLK_PLL,
    SMA1307_OUT_CLK_OSC
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sma1307_sdo_source {
    SMA1307_OUT_DISABLE,
    SMA1307_OUT_FORMAT_C,
    SMA1307_OUT_MIXER_OUT,
    SMA1307_OUT_AFTER_DSP,
    SMA1307_OUT_VRMS2_AVG,
    SMA1307_OUT_BATTERY,
    SMA1307_OUT_TEMP,
    SMA1307_OUT_AFTER_DELAY
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sma1307_setting_file {
    pub status: bool,
    pub header: *mut c_char,
    pub def: *mut c_int,
    pub mode_set: [*mut c_int; 5],
    pub checksum: c_int,
    pub num_mode: c_int,
    pub header_size: usize,
    pub def_size: usize,
    pub mode_size: usize,
}

pub const SMA1307_I2C_ADDR_00: c_uint = 0x1e;
pub const SMA1307_I2C_ADDR_01: c_uint = 0x3e;
pub const SMA1307_I2C_ADDR_10: c_uint = 0x5e;
pub const SMA1307_I2C_ADDR_11: c_uint = 0x7e;

pub const SMA1307_EXTERNAL_CLOCK_19_2: c_uint = 0x00;
pub const SMA1307_EXTERNAL_CLOCK_24_576: c_uint = 0x01;
pub const SMA1307_PLL_CLKIN_MCLK: c_uint = 0x02;
pub const SMA1307_PLL_CLKIN_BCLK: c_uint = 0x03;
pub const SMA1307_OFFSET_DEFAULT_MODE: c_uint = 0x00;
pub const SMA1307_OFFSET_BURNING_MODE: c_uint = 0x01;
pub const SMA1307_SETTING_HEADER_SIZE: c_uint = 0x08;
pub const SMA1307_SETTING_DEFAULT_SIZE: c_uint = 0xC0;
pub const SMA1307_DEFAULT_SET: c_uint = 0x00;
pub const SMA1307_BINARY_FILE_SET: c_uint = 0x01;
// Controls Name

// DAPM Name

//
// SMA1307 Register Definition
//
// SMA1307 Register Addresses
pub const SMA1307_00_SYSTEM_CTRL: c_uint = 0x00;
pub const SMA1307_01_INPUT_CTRL1: c_uint = 0x01;
pub const SMA1307_02_BROWN_OUT_PROT1: c_uint = 0x02;
pub const SMA1307_03_BROWN_OUT_PROT2: c_uint = 0x03;
pub const SMA1307_04_BROWN_OUT_PROT3: c_uint = 0x04;
pub const SMA1307_05_BROWN_OUT_PROT8: c_uint = 0x05;
pub const SMA1307_06_BROWN_OUT_PROT9: c_uint = 0x06;
pub const SMA1307_07_BROWN_OUT_PROT10: c_uint = 0x07;
pub const SMA1307_08_BROWN_OUT_PROT11: c_uint = 0x08;
pub const SMA1307_09_OUTPUT_CTRL: c_uint = 0x09;
pub const SMA1307_0A_SPK_VOL: c_uint = 0x0A;
pub const SMA1307_0B_BST_TEST: c_uint = 0x0B;
pub const SMA1307_0C_BOOST_CTRL8: c_uint = 0x0C;
pub const SMA1307_0D_SPK_TEST: c_uint = 0x0D;
pub const SMA1307_0E_MUTE_VOL_CTRL: c_uint = 0x0E;
pub const SMA1307_0F_VBAT_TEMP_SENSING: c_uint = 0x0F;
pub const SMA1307_10_SYSTEM_CTRL1: c_uint = 0x10;
pub const SMA1307_11_SYSTEM_CTRL2: c_uint = 0x11;
pub const SMA1307_12_SYSTEM_CTRL3: c_uint = 0x12;
pub const SMA1307_13_DELAY: c_uint = 0x13;
pub const SMA1307_14_MODULATOR: c_uint = 0x14;
pub const SMA1307_15_BASS_SPK1: c_uint = 0x15;
pub const SMA1307_16_BASS_SPK2: c_uint = 0x16;
pub const SMA1307_17_BASS_SPK3: c_uint = 0x17;
pub const SMA1307_18_BASS_SPK4: c_uint = 0x18;
pub const SMA1307_19_BASS_SPK5: c_uint = 0x19;
pub const SMA1307_1A_BASS_SPK6: c_uint = 0x1A;
pub const SMA1307_1B_BASS_SPK7: c_uint = 0x1B;
pub const SMA1307_1C_BROWN_OUT_PROT20: c_uint = 0x1C;
pub const SMA1307_1D_BROWN_OUT_PROT0: c_uint = 0x1D;
pub const SMA1307_1E_TONE_GENERATOR: c_uint = 0x1E;
pub const SMA1307_1F_TONE_FINE_VOLUME: c_uint = 0x1F;
pub const SMA1307_22_COMP_HYS_SEL: c_uint = 0x22;
pub const SMA1307_23_COMPLIM1: c_uint = 0x23;
pub const SMA1307_24_COMPLIM2: c_uint = 0x24;
pub const SMA1307_25_COMPLIM3: c_uint = 0x25;
pub const SMA1307_26_COMPLIM4: c_uint = 0x26;
pub const SMA1307_27_BROWN_OUT_PROT4: c_uint = 0x27;
pub const SMA1307_28_BROWN_OUT_PROT5: c_uint = 0x28;
pub const SMA1307_29_BROWN_OUT_PROT12: c_uint = 0x29;
pub const SMA1307_2A_BROWN_OUT_PROT13: c_uint = 0x2A;
pub const SMA1307_2B_BROWN_OUT_PROT14: c_uint = 0x2B;
pub const SMA1307_2C_BROWN_OUT_PROT15: c_uint = 0x2C;
pub const SMA1307_2D_BROWN_OUT_PROT6: c_uint = 0x2D;
pub const SMA1307_2E_BROWN_OUT_PROT7: c_uint = 0x2E;
pub const SMA1307_2F_BROWN_OUT_PROT16: c_uint = 0x2F;
pub const SMA1307_30_BROWN_OUT_PROT17: c_uint = 0x30;
pub const SMA1307_31_BROWN_OUT_PROT18: c_uint = 0x31;
pub const SMA1307_32_BROWN_OUT_PROT19: c_uint = 0x32;
pub const SMA1307_34_OCP_SPK: c_uint = 0x34;
pub const SMA1307_35_FDPEC_CTRL0: c_uint = 0x35;
pub const SMA1307_36_PROTECTION: c_uint = 0x36;
pub const SMA1307_37_SLOPECTRL: c_uint = 0x37;
pub const SMA1307_38_POWER_METER: c_uint = 0x38;
pub const SMA1307_39_PMT_NZ_VAL: c_uint = 0x39;
pub const SMA1307_3B_TEST1: c_uint = 0x3B;
pub const SMA1307_3C_TEST2: c_uint = 0x3C;
pub const SMA1307_3D_TEST3: c_uint = 0x3D;
pub const SMA1307_3E_IDLE_MODE_CTRL: c_uint = 0x3E;
pub const SMA1307_3F_ATEST2: c_uint = 0x3F;
pub const SMA1307_8B_PLL_POST_N: c_uint = 0x8B;
pub const SMA1307_8C_PLL_N: c_uint = 0x8C;
pub const SMA1307_8D_PLL_A_SETTING: c_uint = 0x8D;
pub const SMA1307_8E_PLL_P_CP: c_uint = 0x8E;
pub const SMA1307_8F_ANALOG_TEST: c_uint = 0x8F;
pub const SMA1307_90_CRESTLIM1: c_uint = 0x90;
pub const SMA1307_91_CRESTLIM2: c_uint = 0x91;
pub const SMA1307_92_FDPEC_CTRL1: c_uint = 0x92;
pub const SMA1307_93_INT_CTRL: c_uint = 0x93;
pub const SMA1307_94_BOOST_CTRL9: c_uint = 0x94;
pub const SMA1307_95_BOOST_CTRL10: c_uint = 0x95;
pub const SMA1307_96_BOOST_CTRL11: c_uint = 0x96;
pub const SMA1307_97_OTP_TRM0: c_uint = 0x97;
pub const SMA1307_98_OTP_TRM1: c_uint = 0x98;
pub const SMA1307_99_OTP_TRM2: c_uint = 0x99;
pub const SMA1307_9A_OTP_TRM3: c_uint = 0x9A;
pub const SMA1307_A0_PAD_CTRL0: c_uint = 0xA0;
pub const SMA1307_A1_PAD_CTRL1: c_uint = 0xA1;
pub const SMA1307_A2_TOP_MAN1: c_uint = 0xA2;
pub const SMA1307_A3_TOP_MAN2: c_uint = 0xA3;
pub const SMA1307_A4_TOP_MAN3: c_uint = 0xA4;
pub const SMA1307_A5_TDM1: c_uint = 0xA5;
pub const SMA1307_A6_TDM2: c_uint = 0xA6;
pub const SMA1307_A7_CLK_MON: c_uint = 0xA7;
pub const SMA1307_A8_BOOST_CTRL1: c_uint = 0xA8;
pub const SMA1307_A9_BOOST_CTRL2: c_uint = 0xA9;
pub const SMA1307_AA_BOOST_CTRL3: c_uint = 0xAA;
pub const SMA1307_AB_BOOST_CTRL4: c_uint = 0xAB;
pub const SMA1307_AC_BOOST_CTRL5: c_uint = 0xAC;
pub const SMA1307_AD_BOOST_CTRL6: c_uint = 0xAD;
pub const SMA1307_AE_BOOST_CTRL7: c_uint = 0xAE;
pub const SMA1307_AF_LPF: c_uint = 0xAF;
pub const SMA1307_B0_RMS_TC1: c_uint = 0xB0;
pub const SMA1307_B1_RMS_TC2: c_uint = 0xB1;
pub const SMA1307_B2_AVG_TC1: c_uint = 0xB2;
pub const SMA1307_B3_AVG_TC2: c_uint = 0xB3;
pub const SMA1307_B4_PRVALUE1: c_uint = 0xB4;
pub const SMA1307_B5_PRVALUE2: c_uint = 0xB5;
pub const SMA1307_B8_SPK_NG_CTRL1: c_uint = 0xB8;
pub const SMA1307_B9_SPK_NG_CTRL2: c_uint = 0xB9;
pub const SMA1307_BA_DGC1: c_uint = 0xBA;
pub const SMA1307_BB_DGC2: c_uint = 0xBB;
pub const SMA1307_BC_DGC3: c_uint = 0xBC;
pub const SMA1307_BD_MCBS_CTRL1: c_uint = 0xBD;
pub const SMA1307_BE_MCBS_CTRL2: c_uint = 0xBE;
// Status Register Read Only
pub const SMA1307_F5_READY_FOR_V_SAR: c_uint = 0xF5;
pub const SMA1307_F7_READY_FOR_T_SAR: c_uint = 0xF7;
pub const SMA1307_F8_STATUS_T1: c_uint = 0xF8;
pub const SMA1307_F9_STATUS_T2: c_uint = 0xF9;
pub const SMA1307_FA_STATUS1: c_uint = 0xFA;
pub const SMA1307_FB_STATUS2: c_uint = 0xFB;
pub const SMA1307_FC_STATUS3: c_uint = 0xFC;
pub const SMA1307_FD_STATUS4: c_uint = 0xFD;
pub const SMA1307_FE_STATUS5: c_uint = 0xFE;
pub const SMA1307_FF_DEVICE_INDEX: c_uint = 0xFF;
// SMA1307 Registers Bit Fields
// Power On/Off

pub const SMA1307_POWER_OFF: c_int = 0;

// Reset

// Left Polarity

pub const SMA1307_LOW_FIRST_CH: c_int = 0;

// SCK Falling/Rising

pub const SMA1307_SCK_FALLING_EDGE: c_int = 0;

// SPK Mute

pub const SMA1307_SPK_UNMUTE: c_int = 0;

// SPK Mode

pub const SMA1307_SPK_OFF: c_int = 0;

// Mono Mix

pub const SMA1307_MONOMIX_OFF: c_int = 0;

// LR Data Swap

pub const SMA1307_LR_DATA_SW_NORMAL: c_int = 0;

// PLL On/Off

pub const SMA1307_PLL_ON: c_int = 0;

// Input Format

pub const SMA1307_STANDARD_I2S: c_int = 0;

// Controller / Device Setting

pub const SMA1307_DEVICE_MODE: c_int = 0;

// Port Config

pub const SMA1307_INPUT_PORT_ONLY: c_int = 0;

// SDO Output

pub const SMA1307_LOGIC_OUTPUT: c_int = 0;

pub const SMA1307_SDO_DATA: c_int = 0;

// SDO Output2

pub const SMA1307_ONE_SDO_PER_CH: c_int = 0;

// SDO Output3

pub const SMA1307_SDO_OUTPUT3_DIS: c_int = 0;

// SDO OUT1 Select

pub const SMA1307_SDO1_DISABLE: c_int = 0;

// SDO OUT0 Select

pub const SMA1307_SDO0_DISABLE: c_int = 0;

// INTERRUPT Operation

pub const SMA1307_INT_CLEAR_AUTO: c_int = 0;

// INTERRUPT CLEAR

pub const SMA1307_INT_READY: c_int = 0;

// INTERRUPT Disable

pub const SMA1307_NORMAL_INT: c_int = 0;

// Interface Control

pub const SMA1307_SCK_64FS: c_int = 0;

pub const SMA1307_DATA_24BIT: c_int = 0;

pub const SMA1307_TDM_TX_MONO: c_int = 0;

pub const SMA1307_TDM_SLOT0_RX_POS_0: c_int = 0;

pub const SMA1307_TDM_SLOT1_RX_POS_0: c_int = 0;

// TDM2 FORMAT : 0xA6

pub const SMA1307_TDM_DL_16: c_int = 0;

pub const SMA1307_TDM_N_SLOT_4: c_int = 0;

pub const SMA1307_TDM_SLOT0_TX_POS_0: c_int = 0;

pub const SMA1307_TDM_SLOT1_TX_POS_0: c_int = 0;

// OTP STATUS

pub const SMA1307_OTP_STAT_0: c_int = 0;

// STATUS

pub const SMA1307_REV_NUM_REV0: c_int = 0;

