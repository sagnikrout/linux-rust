//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/tas2781.h
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
// ALSA SoC Texas Instruments TAS2563/TAS2781 Audio Smart Amplifier
//
// Copyright (C) 2022 - 2026 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2563/TAS2781 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// TAS2563/TAS2781 chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
// Author: Kevin Lu <kevin-lu@ti.com>
// Author: Baojun Xu <baojun.xu@ti.com>
//

// version number
pub const TAS2781_DRV_VER: c_int = 1;

pub const TAS2781_GLOBAL_ADDR: c_uint = 0x40;
pub const TAS2563_GLOBAL_ADDR: c_uint = 0x48;

pub const TASDEVICE_CRC8_POLYNOMIAL: c_uint = 0x4d;
// PAGE Control Register (available in page0 of each book)
pub const TASDEVICE_PAGE_SELECT: c_uint = 0x00;
pub const TASDEVICE_BOOKCTL_PAGE: c_uint = 0x00;
pub const TASDEVICE_BOOKCTL_REG: c_int = 127;

// Software Reset, compatble with new device (TAS5825).

// Checksum

// XM_340

// XM_341

// Volume control

// prm_Int_B0

// prm_Int_A1

// prm_TE_Beta

// prm_TE_Beta1

// prm_TE_1_Beta1

pub const TAS2781_TEST_PAGE_UNLOCK: c_uint = 0x0d;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum audio_device {
    TAS2020,
    TAS2118,
    TAS2120,
    TAS2320,
    TAS2563,
    TAS2568,
    TAS2570,
    TAS2572,
    TAS2573,
    TAS2574,
    TAS2781,
    TAS5802,
    TAS5806M,
    TAS5806MD,
    TAS5815,
    TAS5822,
    TAS5825,
    TAS5827,
    TAS5828,
    TAS5830,
    TAS5832,
    TAS_OTHERS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dspbin_type {
    TASDEV_BASIC,
    TASDEV_ALPHA,
    TASDEV_BETA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bulk_reg_val {
    pub reg: c_int,
    pub val: [c_uchar; 4],
    pub val_len: c_uchar,
    pub is_locked: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice {
    pub cali_data_backup: *mut bulk_reg_val,
    pub alp_cali_bckp: bulk_reg_val,
    pub cali_data_fmw: *mut tasdevice_fw,
    pub cali_specific: *mut c_void,
    pub dev_addr: c_uint,
    pub err_code: c_uint,
    pub cur_book: c_uchar,
    pub cur_prog: c_short,
    pub cur_conf: c_short,
    pub is_loading: bool,
    pub is_loaderr: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cali_reg {
    pub r0_reg: c_uint,
    pub r0_low_reg: c_uint,
    pub invr0_reg: c_uint,
    pub pow_reg: c_uint,
    pub tlimit_reg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct calidata {
    pub data: *mut c_uchar,
    pub total_sz: c_ulong,
    pub cali_reg_array: cali_reg,
    pub cali_dat_sz_per_dev: c_uint,
}

//
// To enable CONFIG_SND_SOC_TAS2781_ACOUST_I2C will create a bridge to the
// acoustic tuning tool which can tune the chips' acoustic effect. Due to the
// whole directly exposing the registers, there exist some potential risks. So
// this define is invisible in Kconfig, anyone who wants to use acoustic tool
// have to edit the source manually.
//

pub const TASDEV_DATA_PAYLOAD_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acoustic_data {
    pub len: c_uchar,
    pub id: c_uchar,
    pub addr: c_uchar,
    pub book: c_uchar,
    pub page: c_uchar,
    pub reg: c_uchar,
    pub data: [c_uchar; TASDEV_DATA_PAYLOAD_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_priv {
    pub tasdevice: [tasdevice; TASDEVICE_MAX_CHANNELS],
    pub rcabin: tasdevice_rca,
    pub cali_data: calidata,

    pub acou_data: acoustic_data,

    pub fmw: *mut tasdevice_fw,
    pub reset: *mut gpio_desc,
    pub codec_lock: mutex,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub cal_binaryname: [c_uchar; TASDEVICE_MAX_CHANNELS][64],
    pub crc8_lkp_tbl: [c_uchar; CRC8_TABLE_SIZE],
    pub coef_binaryname: [c_uchar; 64],
    pub rca_binaryname: [c_uchar; 64],
    pub dev_name: [c_uchar; 32],
    pub (*dvc_tlv_table)[4]: *const c_uchar,
    pub name_prefix: *const c_char,
    pub ndev: c_uchar,
    pub dspbin_typ: c_uint,
    pub magic_num: c_uint,
    pub chip_id: c_uint,
    pub sysclk: c_uint,
    pub speaker_id: c_int,
    pub irq: c_int,
    pub cur_prog: c_int,
    pub cur_conf: c_int,
    pub fw_state: c_int,
    pub index: c_int,
    pub client: *mut c_void,
    pub codec: *mut c_void,
    pub force_fwload_status: bool,
    pub playback_started: bool,
    pub isacpi: bool,
    pub isspi: bool,
    pub global_addr: c_uint,
    pub offset): *const *const firmware fmw, int,
    pub offset): *const *const firmware fmw, int,
    pub offset): *const *const firmware fmw, int,
    pub offset): *const *const firmware fmw, int,
    pub block): *mut tasdev_blk,
    pub book): unsigned short chn, int,
    pub value): c_uint,
    pub value): *mut unsigned short chn, unsigned int reg, unsigned int,
    pub n_length): c_uint,
}

extern "C" {
    pub fn tasdevice_remove(tas_priv: *mut tasdevice_priv);
}
