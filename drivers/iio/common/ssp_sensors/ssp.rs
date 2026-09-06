//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/common/ssp_sensors/ssp.h
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
// Copyright (C) 2014, Samsung Electronics Co. Ltd. All Rights Reserved.
//

pub const SSP_DEVICE_ID: c_uint = 0x55;

pub const SSP_SW_RESET_TIME: c_int = 3000;
// Sensor polling in ms
pub const SSP_DEFAULT_POLLING_DELAY: c_int = 200;
pub const SSP_DEFAULT_RETRIES: c_int = 3;
pub const SSP_DATA_PACKET_SIZE: c_int = 960;
pub const SSP_HEADER_BUFFER_SIZE: c_int = 4;
// Firmware download STATE
pub const SSP_INVALID_REVISION: c_int = 99999;
pub const SSP_INVALID_REVISION2: c_uint = 0xffffff;
// AP -> SSP Instruction
pub const SSP_MSG2SSP_INST_BYPASS_SENSOR_ADD: c_uint = 0xa1;
pub const SSP_MSG2SSP_INST_BYPASS_SENSOR_RM: c_uint = 0xa2;
pub const SSP_MSG2SSP_INST_REMOVE_ALL: c_uint = 0xa3;
pub const SSP_MSG2SSP_INST_CHANGE_DELAY: c_uint = 0xa4;
pub const SSP_MSG2SSP_INST_LIBRARY_ADD: c_uint = 0xb1;
pub const SSP_MSG2SSP_INST_LIBRARY_REMOVE: c_uint = 0xb2;
pub const SSP_MSG2SSP_INST_LIB_NOTI: c_uint = 0xb4;
pub const SSP_MSG2SSP_INST_LIB_DATA: c_uint = 0xc1;
pub const SSP_MSG2SSP_AP_MCU_SET_GYRO_CAL: c_uint = 0xcd;
pub const SSP_MSG2SSP_AP_MCU_SET_ACCEL_CAL: c_uint = 0xce;
pub const SSP_MSG2SSP_AP_STATUS_SHUTDOWN: c_uint = 0xd0;
pub const SSP_MSG2SSP_AP_STATUS_WAKEUP: c_uint = 0xd1;
pub const SSP_MSG2SSP_AP_STATUS_SLEEP: c_uint = 0xd2;
pub const SSP_MSG2SSP_AP_STATUS_RESUME: c_uint = 0xd3;
pub const SSP_MSG2SSP_AP_STATUS_SUSPEND: c_uint = 0xd4;
pub const SSP_MSG2SSP_AP_STATUS_RESET: c_uint = 0xd5;
pub const SSP_MSG2SSP_AP_STATUS_POW_CONNECTED: c_uint = 0xd6;
pub const SSP_MSG2SSP_AP_STATUS_POW_DISCONNECTED: c_uint = 0xd7;
pub const SSP_MSG2SSP_AP_TEMPHUMIDITY_CAL_DONE: c_uint = 0xda;
pub const SSP_MSG2SSP_AP_MCU_SET_DUMPMODE: c_uint = 0xdb;
pub const SSP_MSG2SSP_AP_MCU_DUMP_CHECK: c_uint = 0xdc;
pub const SSP_MSG2SSP_AP_MCU_BATCH_FLUSH: c_uint = 0xdd;
pub const SSP_MSG2SSP_AP_MCU_BATCH_COUNT: c_uint = 0xdf;
pub const SSP_MSG2SSP_AP_WHOAMI: c_uint = 0x0f;
pub const SSP_MSG2SSP_AP_FIRMWARE_REV: c_uint = 0xf0;
pub const SSP_MSG2SSP_AP_SENSOR_FORMATION: c_uint = 0xf1;
pub const SSP_MSG2SSP_AP_SENSOR_PROXTHRESHOLD: c_uint = 0xf2;
pub const SSP_MSG2SSP_AP_SENSOR_BARCODE_EMUL: c_uint = 0xf3;
pub const SSP_MSG2SSP_AP_SENSOR_SCANNING: c_uint = 0xf4;
pub const SSP_MSG2SSP_AP_SET_MAGNETIC_HWOFFSET: c_uint = 0xf5;
pub const SSP_MSG2SSP_AP_GET_MAGNETIC_HWOFFSET: c_uint = 0xf6;
pub const SSP_MSG2SSP_AP_SENSOR_GESTURE_CURRENT: c_uint = 0xf7;
pub const SSP_MSG2SSP_AP_GET_THERM: c_uint = 0xf8;
pub const SSP_MSG2SSP_AP_GET_BIG_DATA: c_uint = 0xf9;
pub const SSP_MSG2SSP_AP_SET_BIG_DATA: c_uint = 0xfa;
pub const SSP_MSG2SSP_AP_START_BIG_DATA: c_uint = 0xfb;
pub const SSP_MSG2SSP_AP_SET_MAGNETIC_STATIC_MATRIX: c_uint = 0xfd;
pub const SSP_MSG2SSP_AP_SENSOR_TILT: c_uint = 0xea;
pub const SSP_MSG2SSP_AP_MCU_SET_TIME: c_uint = 0xfe;
pub const SSP_MSG2SSP_AP_MCU_GET_TIME: c_uint = 0xff;
pub const SSP_MSG2SSP_AP_FUSEROM: c_uint = 0x01;
// voice data
pub const SSP_TYPE_WAKE_UP_VOICE_SERVICE: c_uint = 0x01;
pub const SSP_TYPE_WAKE_UP_VOICE_SOUND_SOURCE_AM: c_uint = 0x01;
pub const SSP_TYPE_WAKE_UP_VOICE_SOUND_SOURCE_GRAMMER: c_uint = 0x02;
// Factory Test
pub const SSP_ACCELEROMETER_FACTORY: c_uint = 0x80;
pub const SSP_GYROSCOPE_FACTORY: c_uint = 0x81;
pub const SSP_GEOMAGNETIC_FACTORY: c_uint = 0x82;
pub const SSP_PRESSURE_FACTORY: c_uint = 0x85;
pub const SSP_GESTURE_FACTORY: c_uint = 0x86;
pub const SSP_TEMPHUMIDITY_CRC_FACTORY: c_uint = 0x88;
pub const SSP_GYROSCOPE_TEMP_FACTORY: c_uint = 0x8a;
pub const SSP_GYROSCOPE_DPS_FACTORY: c_uint = 0x8b;
pub const SSP_MCU_FACTORY: c_uint = 0x8c;
pub const SSP_MCU_SLEEP_FACTORY: c_uint = 0x8d;
// SSP -> AP ACK about write CMD
pub const SSP_MSG_ACK: c_uint = 0x80	/* ACK from SSP to AP */;
pub const SSP_MSG_NAK: c_uint = 0x70	/* NAK from SSP to AP */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_sensorhub_info {
    pub fw_name: *mut c_char,
    pub fw_crashed_name: *mut c_char,
    pub fw_rev: c_uint,
    pub mag_table: *const *const u8,
    pub mag_length: c_uint,
}

// ssp_msg options bit
pub const SSP_RW: c_int = 0;
pub const SSP_INDEX: c_int = 3;
pub const SSP_AP2HUB_READ: c_int = 0;
pub const SSP_AP2HUB_WRITE: c_int = 1;
pub const SSP_HUB2AP_WRITE: c_int = 2;
pub const SSP_AP2HUB_READY: c_int = 3;
pub const SSP_AP2HUB_RETURN: c_int = 4;
//
// struct ssp_data - ssp platformdata structure
// @spi:		spi device
// @sensorhub_info:	info about sensorhub board specific features
// @wdt_timer:		watchdog timer
// @work_wdt:		watchdog work
// @work_firmware:	firmware upgrade work queue
// @work_refresh:	refresh work queue for reset request from MCU
// @shut_down:		shut down flag
// @mcu_dump_mode:	mcu dump mode for debug
// @time_syncing:	time syncing indication flag
// @timestamp:		previous time in ns calculated for time syncing
// @check_status:	status table for each sensor
// @com_fail_cnt:	communication fail count
// @reset_cnt:		reset count
// @timeout_cnt:	timeout count
// @available_sensors:	available sensors seen by sensorhub (bit array)
// @cur_firm_rev:	cached current firmware revision
// @last_resume_state:	last AP resume/suspend state used to handle the PM
// state of ssp
// @last_ap_state:	(obsolete) sleep notification for MCU
// @sensor_enable:	sensor enable mask
// @delay_buf:		data acquisition intervals table
// @batch_latency_buf:	yet unknown but existing in communication protocol
// @batch_opt_buf:	yet unknown but existing in communication protocol
// @accel_position:	yet unknown but existing in communication protocol
// @mag_position:	yet unknown but existing in communication protocol
// @fw_dl_state:	firmware download state
// @comm_lock:		lock protecting the handshake
// @pending_lock:	lock protecting pending list and completion
// @mcu_reset_gpiod:	mcu reset line
// @ap_mcu_gpiod:	ap to mcu gpio line
// @mcu_ap_gpiod:	mcu to ap gpio line
// @pending_list:	pending list for messages queued to be sent/read
// @sensor_devs:	registered IIO devices table
// @enable_refcount:	enable reference count for wdt (watchdog timer)
// @header_buffer:	cache aligned buffer for packet header
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssp_data {
    pub spi: *mut spi_device,
    pub sensorhub_info: *const ssp_sensorhub_info,
    pub wdt_timer: timer_list,
    pub work_wdt: work_struct,
    pub work_refresh: delayed_work,
    pub shut_down: bool,
    pub mcu_dump_mode: bool,
    pub time_syncing: bool,
    pub timestamp: i64,
    pub check_status: [c_int; SSP_SENSOR_MAX],
    pub com_fail_cnt: c_uint,
    pub reset_cnt: c_uint,
    pub timeout_cnt: c_uint,
    pub available_sensors: c_uint,
    pub cur_firm_rev: c_uint,
    pub last_resume_state: c_char,
    pub last_ap_state: c_char,
    pub sensor_enable: c_uint,
    pub delay_buf: [u32; SSP_SENSOR_MAX],
    pub batch_latency_buf: [i32; SSP_SENSOR_MAX],
    pub batch_opt_buf: [i8; SSP_SENSOR_MAX],
    pub accel_position: c_int,
    pub mag_position: c_int,
    pub fw_dl_state: c_int,
    pub comm_lock: mutex,
    pub pending_lock: mutex,
    pub mcu_reset_gpiod: *mut gpio_desc,
    pub ap_mcu_gpiod: *mut gpio_desc,
    pub mcu_ap_gpiod: *mut gpio_desc,
    pub pending_list: list_head,
    pub sensor_devs: [*mut iio_dev; SSP_SENSOR_MAX],
    pub enable_refcount: core::sync::atomic::AtomicI32,
    pub __aligned(IIO_DMA_MINALIGN): __le16 header_buffer[SSP_HEADER_BUFFER_SIZE / sizeof(__le16)],
}

extern "C" {
    pub fn ssp_clean_pending_list(data: *mut ssp_data);
}
extern "C" {
    pub fn ssp_command(data: *mut ssp_data, command: c_char, arg: c_int) -> c_int;
}
extern "C" {
    pub fn ssp_irq_msg(data: *mut ssp_data) -> c_int;
}
extern "C" {
    pub fn ssp_get_chipid(data: *mut ssp_data) -> c_int;
}
extern "C" {
    pub fn ssp_set_magnetic_matrix(data: *mut ssp_data) -> c_int;
}
extern "C" {
    pub fn ssp_get_sensor_scanning_info(data: *mut ssp_data) -> c_uint;
}
extern "C" {
    pub fn ssp_get_firmware_rev(data: *mut ssp_data) -> c_uint;
}
extern "C" {
    pub fn ssp_queue_ssp_refresh_task(data: *mut ssp_data, delay: c_uint) -> c_int;
}
