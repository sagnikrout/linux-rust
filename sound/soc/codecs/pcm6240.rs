//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/pcm6240.h
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
// ALSA SoC Texas Instruments PCM6240 Family Audio ADC/DAC/Router
//
// Copyright (C) 2022 - 2024 Texas Instruments Incorporated
// https://www.ti.com
//
// The PCM6240 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// PCM6240 Family Audio chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcm_device {
    ADC3120,
    ADC5120,
    ADC6120,
    DIX4192,
    PCM1690,
    PCM3120,
    PCM3140,
    PCM5120,
    PCM5140,
    PCM6120,
    PCM6140,
    PCM6240,
    PCM6260,
    PCM9211,
    PCMD3140,
    PCMD3180,
    PCMD512X,
    TAA5212,
    TAA5412,
    TAD5212,
    TAD5412,
    MAX_DEVICE,
}

pub const PCMDEV_GENERIC_VOL_CTRL: c_uint = 0x0;
pub const PCMDEV_PCM1690_VOL_CTRL: c_uint = 0x1;
pub const PCMDEV_PCM1690_FINE_VOL_CTRL: c_uint = 0x2;
// Maximum number of I2C addresses
pub const PCMDEVICE_MAX_I2C_DEVICES: c_int = 4;
// Maximum number defined in REGBIN protocol
pub const PCMDEVICE_MAX_REGBIN_DEVICES: c_int = 8;
pub const PCMDEVICE_CONFIG_SUM: c_int = 64;
pub const PCMDEVICE_BIN_FILENAME_LEN: c_int = 64;

pub const PCMDEVICE_MAX_CHANNELS: c_int = 8;

// PAGE Control Register (available in page0 of each book)
pub const PCMDEVICE_PAGE_SELECT: c_uint = 0x00;

pub const PCM1690_REG_MODE_CTRL_DAMS_FINE_STEP: c_uint = 0x0;
pub const PCM1690_REG_MODE_CTRL_DAMS_WIDE_RANGE: c_uint = 0x80;

pub const PCM9211_REG_SW_CTRL_MRST: c_uint = 0x0;

pub const PCMDEVICE_CMD_SING_W: c_uint = 0x1;
pub const PCMDEVICE_CMD_BURST: c_uint = 0x2;
pub const PCMDEVICE_CMD_DELAY: c_uint = 0x3;
pub const PCMDEVICE_CMD_FIELD_W: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcmdevice_bin_blk_type {
    PCMDEVICE_BIN_BLK_COEFF = 1,
    PCMDEVICE_BIN_BLK_POST_POWER_UP,
    PCMDEVICE_BIN_BLK_PRE_SHUTDOWN,
    PCMDEVICE_BIN_BLK_PRE_POWER_UP,
    PCMDEVICE_BIN_BLK_POST_SHUTDOWN
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pcmdevice_fw_state {
    PCMDEVICE_FW_LOAD_OK = 0,
    PCMDEVICE_FW_LOAD_FAILED
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmdevice_regbin_hdr {
    pub img_sz: c_uint,
    pub checksum: c_uint,
    pub binary_version_num: c_uint,
    pub drv_fw_version: c_uint,
    pub timestamp: c_uint,
    pub plat_type: c_uchar,
    pub dev_family: c_uchar,
    pub reserve: c_uchar,
    pub ndev: c_uchar,
    pub devs: [c_uchar; PCMDEVICE_MAX_REGBIN_DEVICES],
    pub nconfig: c_uint,
    pub config_size: [c_uint; PCMDEVICE_CONFIG_SUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmdevice_block_data {
    pub dev_idx: c_uchar,
    pub block_type: c_uchar,
    pub yram_checksum: c_ushort,
    pub block_size: c_uint,
    pub n_subblks: c_uint,
    pub regdata: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmdevice_config_info {
    pub cfg_name: [c_char; 64],
    pub nblocks: c_uint,
    pub real_nblocks: c_uint,
    pub active_dev: c_uchar,
    pub __counted_by(nblocks): *mut *mut pcmdevice_block_data blk_data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmdevice_regbin {
    pub fw_hdr: pcmdevice_regbin_hdr,
    pub ncfgs: c_int,
    pub cfg_info: *mut pcmdevice_config_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmdevice_priv {
    pub component: *mut snd_soc_component,
    pub client: *mut i2c_client,
    pub dev: *mut device,
    pub codec_lock: mutex,
    pub hw_rst: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub regbin: pcmdevice_regbin,
    pub irq: c_int,
    pub addr: [c_uint; PCMDEVICE_MAX_I2C_DEVICES],
    pub chip_id: c_uint,
    pub cur_conf: c_int,
    pub fw_state: c_int,
    pub ndev: c_int,
    pub bin_name: [c_uchar; PCMDEVICE_BIN_FILENAME_LEN],
// used for kcontrol name
    pub upper_dev_name: [c_uchar; I2C_NAME_SIZE],
    pub dev_name: [c_uchar; I2C_NAME_SIZE],
}

// mixer control
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmdevice_mixer_control {
    pub max: c_int,
    pub reg: c_int,
    pub dev_no: c_uint,
    pub shift: c_uint,
    pub invert: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmdev_ctrl_info {
    pub gain: *const c_uint,
    pub pcmdev_ctrl: *const pcmdevice_mixer_control,
    pub ctrl_array_size: c_uint,
    pub get: *mut snd_kcontrol_get_t,
    pub put: *mut snd_kcontrol_put_t,
    pub pcmdev_ctrl_name_id: c_int,
}
