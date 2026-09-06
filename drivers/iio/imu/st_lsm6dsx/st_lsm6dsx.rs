//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/imu/st_lsm6dsx/st_lsm6dsx.h
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
// STMicroelectronics st_lsm6dsx sensor driver
//
// Copyright 2016 STMicroelectronics Inc.
//
// Lorenzo Bianconi <lorenzo.bianconi@st.com>
// Denis Ciocca <denis.ciocca@st.com>
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_lsm6dsx_hw_id {
    ST_LSM6DS3_ID = 1,
    ST_LSM6DS3H_ID,
    ST_LSM6DSL_ID,
    ST_LSM6DSM_ID,
    ST_ISM330DLC_ID,
    ST_LSM6DSO_ID,
    ST_ASM330LHH_ID,
    ST_LSM6DSOX_ID,
    ST_LSM6DSR_ID,
    ST_LSM6DS3TRC_ID,
    ST_ISM330DHCX_ID,
    ST_LSM9DS1_ID,
    ST_LSM6DS0_ID,
    ST_LSM6DSRX_ID,
    ST_LSM6DST_ID,
    ST_LSM6DSOP_ID,
    ST_ASM330LHHX_ID,
    ST_LSM6DSTX_ID,
    ST_LSM6DSV_ID,
    ST_LSM6DSV16X_ID,
    ST_LSM6DSO16IS_ID,
    ST_ISM330IS_ID,
    ST_ASM330LHB_ID,
    ST_ASM330LHHXG1_ID,
    ST_LSM6DSX_MAX_ID,
}

pub const ST_LSM6DSX_BUFF_SIZE: c_int = 512;
pub const ST_LSM6DSX_CHAN_SIZE: c_int = 2;
pub const ST_LSM6DSX_SAMPLE_SIZE: c_int = 6;
pub const ST_LSM6DSX_TAG_SIZE: c_int = 1;

// ST_LSM6DSX_TAGGED_SAMPLE_SIZE)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_reg {
    pub addr: u8,
    pub mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_odr {
    pub milli_hz: u32,
    pub val: u8,
}

pub const ST_LSM6DSX_ODR_LIST_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_odr_table_entry {
    pub reg: st_lsm6dsx_reg,
    pub odr_avl: [st_lsm6dsx_odr; ST_LSM6DSX_ODR_LIST_SIZE],
    pub odr_len: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_samples_to_discard {
    pub milli_hz: u32,
    pub samples: u16,
    pub val: [}; ST_LSM6DSX_ODR_LIST_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_fs {
    pub gain: u32,
    pub val: u8,
}

pub const ST_LSM6DSX_FS_LIST_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_fs_table_entry {
    pub reg: st_lsm6dsx_reg,
    pub fs_avl: [st_lsm6dsx_fs; ST_LSM6DSX_FS_LIST_SIZE],
    pub fs_len: c_int,
}

//
// struct st_lsm6dsx_fifo_ops - ST IMU FIFO settings
// @update_fifo: Update FIFO configuration callback.
// @read_fifo: Read FIFO callback.
// @fifo_th: FIFO threshold register info (addr + mask).
// @fifo_diff: FIFO diff status register info (addr + mask).
// @max_size: Sensor max fifo length in FIFO words.
// @th_wl: FIFO threshold word length.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_fifo_ops {
    pub enable): *mut *mut *mut int (update_fifo)(struct st_lsm6dsx_sensor sensor, bool,
    pub hw): *mut *mut int (read_fifo)(struct st_lsm6dsx_hw,
    pub addr: u8,
    pub mask: u16,
    pub fifo_th: },
    pub addr: u8,
    pub mask: u16,
    pub fifo_diff: },
    pub max_size: u16,
    pub th_wl: u8,
}

//
// struct st_lsm6dsx_hw_ts_settings - ST IMU hw timer settings
// @timer_en: Hw timer enable register info (addr + mask).
// @hr_timer: Hw timer resolution register info (addr + mask).
// @fifo_en: Hw timer FIFO enable register info (addr + mask).
// @decimator: Hw timer FIFO decimator register info (addr + mask).
// @freq_fine: Difference in % of ODR with respect to the typical.
// @ts_sensitivity: Nominal timestamp sensitivity.
// @ts_trim_coeff: Coefficient for calculating the calibrated timestamp gain.
// This coefficient comes into play when linearizing the formula
// used to calculate the calibrated timestamp (please see the
// relevant formula in the AN for the specific IMU).
// For example, in the case of LSM6DSO we have:
//
// 1 / (1 + x) ~= 1 - x (Taylor’s Series)
// ttrim[s] = 1 / (40000 * (1 + 0.0015 * val)) (from AN5192)
// ttrim[ns] ~= 25000 - 37.5 * val
// ttrim[ns] ~= 25000 - (37500 * val) / 1000
//
// so, replacing ts_sensitivity = 25000 and
// ts_trim_coeff = 37500
//
// ttrim[ns] ~= ts_sensitivity - (ts_trim_coeff * val) / 1000
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_hw_ts_settings {
    pub timer_en: st_lsm6dsx_reg,
    pub hr_timer: st_lsm6dsx_reg,
    pub fifo_en: st_lsm6dsx_reg,
    pub decimator: st_lsm6dsx_reg,
    pub freq_fine: u8,
    pub ts_sensitivity: u16,
    pub ts_trim_coeff: u16,
}

//
// struct st_lsm6dsx_shub_settings - ST IMU hw i2c controller settings
// @page_mux: register page mux info (addr + mask).
// @master_en: master config register info (addr + mask).
// @pullup_en: i2c controller pull-up register info (addr + mask).
// @aux_sens: aux sensor register info (addr + mask).
// @wr_once: write_once register info (addr + mask).
// @emb_func:  embedded function register info (addr + mask).
// @num_ext_dev: max number of slave devices.
// @shub_out: sensor hub first output register info.
// @slv0_addr: slave0 address in secondary page.
// @dw_slv0_addr: slave0 write register address in secondary page.
// @batch_en: Enable/disable FIFO batching.
// @pause: controller pause value.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_shub_settings {
    pub page_mux: st_lsm6dsx_reg,
    pub sec_page: bool,
    pub addr: u8,
    pub mask: u8,
    pub master_en: },
    pub sec_page: bool,
    pub addr: u8,
    pub mask: u8,
    pub pullup_en: },
    pub aux_sens: st_lsm6dsx_reg,
    pub wr_once: st_lsm6dsx_reg,
    pub emb_func: st_lsm6dsx_reg,
    pub num_ext_dev: u8,
    pub sec_page: bool,
    pub addr: u8,
    pub shub_out: },
    pub slv0_addr: u8,
    pub dw_slv0_addr: u8,
    pub batch_en: u8,
    pub pause: u8,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_lsm6dsx_event_id {
    ST_LSM6DSX_EVENT_WAKEUP,
    ST_LSM6DSX_EVENT_TAP,
    ST_LSM6DSX_EVENT_MAX
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_event_src {
    pub value: st_lsm6dsx_reg,
    pub x_value: st_lsm6dsx_reg,
    pub y_value: st_lsm6dsx_reg,
    pub z_value: st_lsm6dsx_reg,
    pub enable_mask: u8,
    pub enable_axis_reg: u8,
    pub enable_x_mask: u8,
    pub enable_y_mask: u8,
    pub enable_z_mask: u8,
    pub status: st_lsm6dsx_reg,
    pub status_x_mask: u8,
    pub status_y_mask: u8,
    pub status_z_mask: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_event_settings {
    pub enable_reg: st_lsm6dsx_reg,
    pub sources: [st_lsm6dsx_event_src; ST_LSM6DSX_EVENT_MAX],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_lsm6dsx_sensor_id {
    ST_LSM6DSX_ID_GYRO,
    ST_LSM6DSX_ID_ACC,
    ST_LSM6DSX_ID_EXT0,
    ST_LSM6DSX_ID_EXT1,
    ST_LSM6DSX_ID_EXT2,
    ST_LSM6DSX_ID_FUSION,
    ST_LSM6DSX_ID_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_lsm6dsx_ext_sensor_id {
    ST_LSM6DSX_ID_MAGN,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_fusion_settings {
    pub chan: *const iio_chan_spec,
    pub chan_len: c_int,
    pub odr_reg: st_lsm6dsx_reg,
    pub odr_hz: [c_int; ST_LSM6DSX_ODR_LIST_SIZE],
    pub odr_len: c_int,
    pub fifo_enable: st_lsm6dsx_reg,
    pub page_mux: st_lsm6dsx_reg,
    pub enable: st_lsm6dsx_reg,
}

//
// struct st_lsm6dsx_ext_dev_settings - i2c controller slave settings
// @i2c_addr: I2c slave address list.
// @wai: Wai address info.
// @id: external sensor id.
// @odr_table: Output data rate of the sensor [Hz].
// @fs_table: Configured sensor sensitivity table depending on full scale.
// @temp_comp: Temperature compensation register info (addr + mask).
// @pwr_table: Power on register info (addr + mask).
// @off_canc: Offset cancellation register info (addr + mask).
// @bdu: Block data update register info (addr + mask).
// @out: Output register info.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_ext_dev_settings {
    pub i2c_addr: [u8; 2],
    pub addr: u8,
    pub val: u8,
    pub wai: },
    pub id: st_lsm6dsx_ext_sensor_id,
    pub odr_table: st_lsm6dsx_odr_table_entry,
    pub fs_table: st_lsm6dsx_fs_table_entry,
    pub temp_comp: st_lsm6dsx_reg,
    pub reg: st_lsm6dsx_reg,
    pub off_val: u8,
    pub on_val: u8,
    pub pwr_table: },
    pub off_canc: st_lsm6dsx_reg,
    pub bdu: st_lsm6dsx_reg,
    pub addr: u8,
    pub len: u8,
    pub out: },
}

//
// struct st_lsm6dsx_settings - ST IMU sensor settings
// @reset: register address for reset.
// @boot: register address for boot.
// @bdu: register address for Block Data Update.
// @id: List of hw id/device name supported by the driver configuration.
// @channels: IIO channels supported by the device.
// @irq_config: interrupts related registers.
// @drdy_mask: register info for data-ready mask (addr + mask).
// @odr_table: Hw sensors odr table (Hz + val).
// @samples_to_discard: Number of samples to discard for filters settling time.
// @fs_table: Hw sensors gain table (gain + val).
// @decimator: List of decimator register info (addr + mask).
// @batch: List of FIFO batching register info (addr + mask).
// @fifo_ops: Sensor hw FIFO parameters.
// @ts_settings: Hw timer related settings.
// @shub_settings: i2c controller related settings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_settings {
    pub reset: st_lsm6dsx_reg,
    pub boot: st_lsm6dsx_reg,
    pub bdu: st_lsm6dsx_reg,
    pub hw_id: st_lsm6dsx_hw_id,
    pub name: *const c_char,
    pub wai: u8,
    pub id: [}; ST_LSM6DSX_MAX_ID],
    pub chan: *const iio_chan_spec,
    pub len: c_int,
    pub channels: [}; 2],
    pub irq1: st_lsm6dsx_reg,
    pub irq2: st_lsm6dsx_reg,
    pub irq1_func: u8,
    pub irq2_func: u8,
    pub lir: st_lsm6dsx_reg,
    pub clear_on_read: st_lsm6dsx_reg,
    pub hla: st_lsm6dsx_reg,
    pub od: st_lsm6dsx_reg,
    pub irq_config: },
    pub drdy_mask: st_lsm6dsx_reg,
    pub odr_table: [st_lsm6dsx_odr_table_entry; 2],
    pub samples_to_discard: [st_lsm6dsx_samples_to_discard; 2],
    pub fs_table: [st_lsm6dsx_fs_table_entry; 2],
    pub decimator: [st_lsm6dsx_reg; ST_LSM6DSX_ID_MAX],
    pub batch: [st_lsm6dsx_reg; 2],
    pub fifo_ops: st_lsm6dsx_fifo_ops,
    pub ts_settings: st_lsm6dsx_hw_ts_settings,
    pub shub_settings: st_lsm6dsx_shub_settings,
    pub event_settings: st_lsm6dsx_event_settings,
    pub fusion_settings: st_lsm6dsx_fusion_settings,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum st_lsm6dsx_fifo_mode {
    ST_LSM6DSX_FIFO_BYPASS = 0x0,
    ST_LSM6DSX_FIFO_CONT = 0x6,
}

//
// struct st_lsm6dsx_sensor - ST IMU sensor instance
// @name: Sensor name.
// @id: Sensor identifier.
// @hw: Pointer to instance of struct st_lsm6dsx_hw.
// @gain: Configured sensor sensitivity.
// @odr: Output data rate of the sensor [mHz].
// hwfifo_odr_mHz: Batch data rate for hardware FIFO [mHz]
// @samples_to_discard: Number of samples to discard for filters settling time.
// @watermark: Sensor watermark level.
// @decimator: Sensor decimation factor.
// @sip: Number of samples in a given pattern.
// @ts_ref: Sensor timestamp reference for hw one.
// @ext_info: Sensor settings if it is connected to i2c controller
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_sensor {
    pub name: [c_char; 32],
    pub id: st_lsm6dsx_sensor_id,
    pub hw: *mut st_lsm6dsx_hw,
    pub gain: u32,
    pub odr: u32,
    pub hwfifo_odr_mHz: u32,
    pub samples_to_discard: u16,
    pub watermark: u16,
    pub decimator: u8,
    pub sip: u8,
    pub ts_ref: i64,
    pub settings: *const st_lsm6dsx_ext_dev_settings,
    pub slv_odr: u32,
    pub addr: u8,
    pub ext_info: },
}

//
// struct st_lsm6dsx_hw - ST IMU MEMS hw instance
// @dev: Pointer to instance of struct device (I2C or SPI).
// @regmap: Register map of the device.
// @irq: Device interrupt line (I2C or SPI).
// @fifo_lock: Mutex to prevent concurrent access to the hw FIFO.
// @conf_lock: Mutex to prevent concurrent FIFO configuration update.
// @page_lock: Mutex to prevent concurrent memory page configuration.
// @suspend_mask: Suspended sensor bitmask.
// @enable_mask: Enabled sensor bitmask.
// @fifo_mask: Enabled hw FIFO bitmask.
// @ts_gain: Hw timestamp rate after internal calibration.
// @ts_sip: Total number of timestamp samples in a given pattern.
// @sip: Total number of samples (acc/gyro/ts) in a given pattern.
// @buff: Device read buffer.
// @irq_routing: pointer to interrupt routing configuration.
// @enable_event: enabled event bitmask.
// @iio_devs: Pointers to acc/gyro iio_dev instances.
// @settings: Pointer to the specific sensor settings in use.
// @orientation: sensor chip orientation relative to main hardware.
// @scan: Temporary buffers used to align data before iio_push_to_buffers()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct st_lsm6dsx_hw {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub irq: c_int,
    pub fifo_lock: mutex,
    pub conf_lock: mutex,
    pub page_lock: mutex,
    pub suspend_mask: u8,
    pub enable_mask: u8,
    pub fifo_mask: u8,
    pub ts_gain: i64,
    pub ts_sip: u8,
    pub sip: u8,
    pub irq_routing: u8,
    pub enable_event: [u8; ST_LSM6DSX_EVENT_MAX],
    pub buff: *mut u8,
    pub iio_devs: [*mut iio_dev; ST_LSM6DSX_ID_MAX],
    pub settings: *const st_lsm6dsx_settings,
    pub orientation: iio_mount_matrix,
// Ensure natural alignment of buffer elements
    pub channels: [__le16; 3],
    pub ts: aligned_s64,
    pub scan: [}; ST_LSM6DSX_ID_MAX],
}

extern "C" {
    pub fn st_lsm6dsx_fifo_setup(hw: *mut st_lsm6dsx_hw) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_set_watermark(iio_dev: *mut iio_dev, val: c_uint) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_update_fifo(sensor: *mut st_lsm6dsx_sensor, enable: bool) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_flush_fifo(hw: *mut st_lsm6dsx_hw) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_resume_fifo(hw: *mut st_lsm6dsx_hw) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_read_fifo(hw: *mut st_lsm6dsx_hw) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_read_tagged_fifo(hw: *mut st_lsm6dsx_hw) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_check_odr(sensor: *mut st_lsm6dsx_sensor, odr: u32, val: *mut u8) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_shub_probe(hw: *mut st_lsm6dsx_hw, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_shub_set_enable(sensor: *mut st_lsm6dsx_sensor, enable: bool) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_shub_read_output(hw: *mut st_lsm6dsx_hw, data: *mut u8, len: c_int) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_fusion_probe(hw: *mut st_lsm6dsx_hw, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_fusion_set_enable(sensor: *mut st_lsm6dsx_sensor, enable: bool) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_fusion_set_odr(sensor: *mut st_lsm6dsx_sensor, enable: bool) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_set_page(hw: *mut st_lsm6dsx_hw, enable: bool) -> c_int;
}
extern "C" {
    pub fn st_lsm6dsx_shub_set_enable(_arg: sensor, _arg: enable) -> return;
}
extern "C" {
    pub fn st_lsm6dsx_fusion_set_enable(_arg: sensor, _arg: enable) -> return;
}
extern "C" {
    pub fn st_lsm6dsx_sensor_set_enable(_arg: sensor, _arg: enable) -> return;
}
