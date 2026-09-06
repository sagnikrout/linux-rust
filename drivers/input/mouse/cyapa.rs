//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/mouse/cyapa.h
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
// Cypress APA trackpad with I2C interface
//
// Author: Dudley Du <dudl@cypress.com>
//
// Copyright (C) 2014-2015 Cypress Semiconductor, Inc.
//
// This file is subject to the terms and conditions of the GNU General Public
// License.  See the file COPYING in the main directory of this archive for
// more details.
//

// APA trackpad firmware generation number.
pub const CYAPA_GEN_UNKNOWN: c_uint = 0x00   /* unknown protocol. */;
pub const CYAPA_GEN3: c_uint = 0x03   /* support MT-protocol B with tracking ID. */;
pub const CYAPA_GEN5: c_uint = 0x05   /* support TrueTouch GEN5 trackpad device. */;
pub const CYAPA_GEN6: c_uint = 0x06   /* support TrueTouch GEN6 trackpad device. */;

//
// Macros for SMBus communication
//
pub const SMBUS_READ: c_uint = 0x01;
pub const SMBUS_WRITE: c_uint = 0x00;

pub const SMBUS_BYTE_BLOCK_CMD_MASK: c_uint = 0x80;
pub const SMBUS_GROUP_BLOCK_CMD_MASK: c_uint = 0x40;
// Commands for read/write registers of Cypress trackpad
pub const CYAPA_CMD_SOFT_RESET: c_uint = 0x00;
pub const CYAPA_CMD_POWER_MODE: c_uint = 0x01;
pub const CYAPA_CMD_DEV_STATUS: c_uint = 0x02;
pub const CYAPA_CMD_GROUP_DATA: c_uint = 0x03;
pub const CYAPA_CMD_GROUP_CMD: c_uint = 0x04;
pub const CYAPA_CMD_GROUP_QUERY: c_uint = 0x05;
pub const CYAPA_CMD_BL_STATUS: c_uint = 0x06;
pub const CYAPA_CMD_BL_HEAD: c_uint = 0x07;
pub const CYAPA_CMD_BL_CMD: c_uint = 0x08;
pub const CYAPA_CMD_BL_DATA: c_uint = 0x09;
pub const CYAPA_CMD_BL_ALL: c_uint = 0x0a;
pub const CYAPA_CMD_BLK_PRODUCT_ID: c_uint = 0x0b;
pub const CYAPA_CMD_BLK_HEAD: c_uint = 0x0c;
pub const CYAPA_CMD_MAX_BASELINE: c_uint = 0x0d;
pub const CYAPA_CMD_MIN_BASELINE: c_uint = 0x0e;
pub const BL_HEAD_OFFSET: c_uint = 0x00;
pub const BL_DATA_OFFSET: c_uint = 0x10;

pub const CYAPA_REG_MAP_SIZE: c_int = 256;
//
// Gen3 Operational Device Status Register
//
// bit 7: Valid interrupt source
// bit 6 - 4: Reserved
// bit 3 - 2: Power status
// bit 1 - 0: Device status
//
pub const REG_OP_STATUS: c_uint = 0x00;
pub const OP_STATUS_SRC: c_uint = 0x80;
pub const OP_STATUS_POWER: c_uint = 0x0c;
pub const OP_STATUS_DEV: c_uint = 0x03;

//
// Operational Finger Count/Button Flags Register
//
// bit 7 - 4: Number of touched finger
// bit 3: Valid data
// bit 2: Middle Physical Button
// bit 1: Right Physical Button
// bit 0: Left physical Button
//
pub const REG_OP_DATA1: c_uint = 0x01;
pub const OP_DATA_VALID: c_uint = 0x08;
pub const OP_DATA_MIDDLE_BTN: c_uint = 0x04;
pub const OP_DATA_RIGHT_BTN: c_uint = 0x02;
pub const OP_DATA_LEFT_BTN: c_uint = 0x01;

//
// Write-only command file register used to issue commands and
// parameters to the bootloader.
// The default value read from it is always 0x00.
//
pub const REG_BL_FILE: c_uint = 0x00;
pub const BL_FILE: c_uint = 0x00;
//
// Bootloader Status Register
//
// bit 7: Busy
// bit 6 - 5: Reserved
// bit 4: Bootloader running
// bit 3 - 2: Reserved
// bit 1: Watchdog Reset
// bit 0: Checksum valid
//
pub const REG_BL_STATUS: c_uint = 0x01;
pub const BL_STATUS_REV_6_5: c_uint = 0x60;
pub const BL_STATUS_BUSY: c_uint = 0x80;
pub const BL_STATUS_RUNNING: c_uint = 0x10;
pub const BL_STATUS_REV_3_2: c_uint = 0x0c;
pub const BL_STATUS_WATCHDOG: c_uint = 0x02;
pub const BL_STATUS_CSUM_VALID: c_uint = 0x01;

//
// Bootloader Error Register
//
// bit 7: Invalid
// bit 6: Invalid security key
// bit 5: Bootloading
// bit 4: Command checksum
// bit 3: Flash protection error
// bit 2: Flash checksum error
// bit 1 - 0: Reserved
//
pub const REG_BL_ERROR: c_uint = 0x02;
pub const BL_ERROR_INVALID: c_uint = 0x80;
pub const BL_ERROR_INVALID_KEY: c_uint = 0x40;
pub const BL_ERROR_BOOTLOADING: c_uint = 0x20;
pub const BL_ERROR_CMD_CSUM: c_uint = 0x10;
pub const BL_ERROR_FLASH_PROT: c_uint = 0x08;
pub const BL_ERROR_FLASH_CSUM: c_uint = 0x04;
pub const BL_ERROR_RESERVED: c_uint = 0x03;
pub const BL_ERROR_NO_ERR_IDLE: c_uint = 0x00;

pub const CAPABILITY_BTN_SHIFT: c_int = 3;

pub const PWR_MODE_MASK: c_uint = 0xfc;

pub const PWR_STATUS_MASK: c_uint = 0x0c;

// Common macros for PIP interface.
pub const PIP_HID_DESCRIPTOR_ADDR: c_uint = 0x0001;
pub const PIP_REPORT_DESCRIPTOR_ADDR: c_uint = 0x0002;
pub const PIP_INPUT_REPORT_ADDR: c_uint = 0x0003;
pub const PIP_OUTPUT_REPORT_ADDR: c_uint = 0x0004;
pub const PIP_CMD_DATA_ADDR: c_uint = 0x0006;
pub const PIP_RETRIEVE_DATA_STRUCTURE: c_uint = 0x24;
pub const PIP_CMD_CALIBRATE: c_uint = 0x28;
pub const PIP_BL_CMD_VERIFY_APP_INTEGRITY: c_uint = 0x31;
pub const PIP_BL_CMD_GET_BL_INFO: c_uint = 0x38;
pub const PIP_BL_CMD_PROGRAM_VERIFY_ROW: c_uint = 0x39;
pub const PIP_BL_CMD_LAUNCH_APP: c_uint = 0x3b;
pub const PIP_BL_CMD_INITIATE_BL: c_uint = 0x48;
pub const PIP_INVALID_CMD: c_uint = 0xff;
pub const PIP_HID_DESCRIPTOR_SIZE: c_int = 32;
pub const PIP_HID_APP_REPORT_ID: c_uint = 0xf7;
pub const PIP_HID_BL_REPORT_ID: c_uint = 0xff;
pub const PIP_BL_CMD_REPORT_ID: c_uint = 0x40;
pub const PIP_BL_RESP_REPORT_ID: c_uint = 0x30;
pub const PIP_APP_CMD_REPORT_ID: c_uint = 0x2f;
pub const PIP_APP_RESP_REPORT_ID: c_uint = 0x1f;
pub const PIP_READ_SYS_INFO_CMD_LENGTH: c_int = 7;
pub const PIP_BL_READ_APP_INFO_CMD_LENGTH: c_int = 13;
pub const PIP_MIN_BL_CMD_LENGTH: c_int = 13;
pub const PIP_MIN_BL_RESP_LENGTH: c_int = 11;
pub const PIP_MIN_APP_CMD_LENGTH: c_int = 7;
pub const PIP_MIN_APP_RESP_LENGTH: c_int = 5;
pub const PIP_UNSUPPORTED_CMD_RESP_LENGTH: c_int = 6;
pub const PIP_READ_SYS_INFO_RESP_LENGTH: c_int = 71;
pub const PIP_BL_APP_INFO_RESP_LENGTH: c_int = 30;
pub const PIP_BL_GET_INFO_RESP_LENGTH: c_int = 19;
pub const PIP_BL_PLATFORM_VER_SHIFT: c_int = 4;
pub const PIP_BL_PLATFORM_VER_MASK: c_uint = 0x0f;
pub const PIP_PRODUCT_FAMILY_MASK: c_uint = 0xf000;
pub const PIP_PRODUCT_FAMILY_TRACKPAD: c_uint = 0x1000;
pub const PIP_DEEP_SLEEP_STATE_ON: c_uint = 0x00;
pub const PIP_DEEP_SLEEP_STATE_OFF: c_uint = 0x01;
pub const PIP_DEEP_SLEEP_STATE_MASK: c_uint = 0x03;
pub const PIP_APP_DEEP_SLEEP_REPORT_ID: c_uint = 0xf0;
pub const PIP_DEEP_SLEEP_RESP_LENGTH: c_int = 5;
pub const PIP_DEEP_SLEEP_OPCODE: c_uint = 0x08;
pub const PIP_DEEP_SLEEP_OPCODE_MASK: c_uint = 0x0f;
pub const PIP_RESP_LENGTH_OFFSET: c_int = 0;
pub const PIP_RESP_LENGTH_SIZE: c_int = 2;
pub const PIP_RESP_REPORT_ID_OFFSET: c_int = 2;
pub const PIP_RESP_RSVD_OFFSET: c_int = 3;
pub const PIP_RESP_RSVD_KEY: c_uint = 0x00;
pub const PIP_RESP_BL_SOP_OFFSET: c_int = 4;
pub const PIP_SOP_KEY: c_uint = 0x01  /* Start of Packet */;
pub const PIP_EOP_KEY: c_uint = 0x17  /* End of Packet */;
pub const PIP_RESP_APP_CMD_OFFSET: c_int = 4;

pub const PIP_RESP_STATUS_OFFSET: c_int = 5;

// Variables to record latest gen5 trackpad power states.
pub const UNINIT_SLEEP_TIME: c_uint = 0xffff;
pub const UNINIT_PWR_MODE: c_uint = 0xff;

// The touch.id is used as the MT slot id, thus max MT slot is 15
pub const CYAPA_MAX_MT_SLOTS: c_int = 15;
extern "C" {
    pub fn bool(: *mut *mut cb_sort)(struct cyapa, : *mut u8, _arg: c_int) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cyapa_pm_stage {
    CYAPA_PM_DEACTIVE,
    CYAPA_PM_ACTIVE,
    CYAPA_PM_SUSPEND,
    CYAPA_PM_RESUME,
    CYAPA_PM_RUNTIME_SUSPEND,
    CYAPA_PM_RUNTIME_RESUME,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyapa_dev_ops {
    pub ): *const *const *const int (check_fw)(struct cyapa , struct firmware,
    pub ): *mut *mut int (bl_enter)(struct cyapa,
    pub ): *mut *mut int (bl_activate)(struct cyapa,
    pub ): *const *const *const int (bl_initiate)(struct cyapa , struct firmware,
    pub ): *const *const *const int (update_fw)(struct cyapa , struct firmware,
    pub ): *mut *mut int (bl_deactivate)(struct cyapa,
    pub ): *mut *mut device_attribute , char,
    pub size_t): *const *const *const device_attribute , char ,,
    pub cyapa): *mut *mut int (initialize)(struct cyapa,
    pub len): *mut *mut *mut *mut int (state_parse)(struct cyapa cyapa, u8 reg_status, int,
    pub cyapa): *mut *mut int (operational_check)(struct cyapa,
    pub ): *mut *mut int (irq_handler)(struct cyapa,
    pub ): *mut *mut bool (irq_cmd_handler)(struct cyapa,
    pub cb_sort): *mut *mut *mut u8 , int ,,
    pub cyapa_pm_stage): *mut *mut *mut int (set_power_mode)(struct cyapa , u8, u16, enum,
    pub bool): *mut *mut *mut int (set_proximity)(struct cyapa ,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyapa_pip_cmd_states {
    pub cmd_lock: mutex,
    pub cmd_ready: completion,
    pub cmd_issued: core::sync::atomic::AtomicI32,
    pub in_progress_cmd: u8,
    pub is_irq_mode: bool,
    pub resp_sort_func: cb_sort,
    pub resp_data: *mut u8,
    pub resp_len: *mut c_int,
    pub pm_stage: cyapa_pm_stage,
    pub pm_stage_lock: mutex,
    pub irq_cmd_buf: [u8; CYAPA_REG_MAP_SIZE],
    pub empty_buf: [u8; CYAPA_REG_MAP_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union cyapa_cmd_states {
    pub pip: cyapa_pip_cmd_states,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cyapa_state {
    CYAPA_STATE_NO_DEVICE,
    CYAPA_STATE_BL_BUSY,
    CYAPA_STATE_BL_IDLE,
    CYAPA_STATE_BL_ACTIVE,
    CYAPA_STATE_OP,
    CYAPA_STATE_GEN5_BL,
    CYAPA_STATE_GEN5_APP,
    CYAPA_STATE_GEN6_BL,
    CYAPA_STATE_GEN6_APP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen6_interval_setting {
    pub active_interval: u16,
    pub lp1_interval: u16,
    pub lp2_interval: u16,
}

// The main device structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyapa {
    pub state: cyapa_state,
    pub status: [u8; BL_STATUS_SIZE],
    pub /: *mut *mut bool operational; / true: ready for data reporting; false: not.,
    pub vcc: *mut regulator,
    pub client: *mut i2c_client,
    pub input: *mut input_dev,
    pub /: *mut *mut char phys[32]; / Device physical location,
    pub /: *mut *mut bool irq_wake; / Irq wake is enabled,
    pub smbus: bool,
// power mode settings
    pub suspend_power_mode: u8,
    pub suspend_sleep_time: u16,
    pub runtime_suspend_power_mode: u8,
    pub runtime_suspend_sleep_time: u16,
    pub dev_pwr_mode: u8,
    pub dev_sleep_time: u16,
    pub gen6_interval_setting: gen6_interval_setting,
// Read from query data region.
    pub product_id: [c_char; 16],
    pub /: *mut *mut u8 platform_ver; / Platform version.,
    pub /: *mut *mut u8 fw_maj_ver; / Firmware major version.,
    pub /: *mut *mut u8 fw_min_ver; / Firmware minor version.,
    pub btn_capability: u8,
    pub gen: u8,
    pub max_abs_x: c_int,
    pub max_abs_y: c_int,
    pub physical_size_x: c_int,
    pub physical_size_y: c_int,
// Used in ttsp and truetouch based trackpad devices.
    pub /: *mut *mut u8 x_origin; / X Axis Origin: 0 = left side; 1 = right side.,
    pub /: *mut *mut u8 y_origin; / Y Axis Origin: 0 = top; 1 = bottom.,
    pub Axis*/: *mut *mut int electrodes_x; / Number of electrodes on the X,
    pub Axis*/: *mut *mut int electrodes_y; / Number of electrodes on the Y,
    pub /: *mut *mut int electrodes_rx; / Number of Rx electrodes,
    pub /: *mut *mut int aligned_electrodes_rx; / 4 aligned,
    pub max_z: c_int,
//
// Used to synchronize the access or update the device state.
// And since update firmware and read firmware image process will take
// quite long time, maybe more than 10 seconds, so use mutex_lock
// to sync and wait other interface and detecting are done or ready.
//
    pub state_sync_lock: mutex,
    pub ops: *const cyapa_dev_ops,
    pub cmd_states: cyapa_cmd_states,
}

extern "C" {
    pub fn cyapa_read_block(cyapa: *mut cyapa, cmd_idx: u8, values: *mut u8) -> isize;
}
extern "C" {
    pub fn cyapa_poll_state(cyapa: *mut cyapa, timeout: c_uint) -> c_int;
}
extern "C" {
    pub fn cyapa_sleep_time_to_pwr_cmd(sleep_time: u16) -> u8;
}
extern "C" {
    pub fn cyapa_pwr_cmd_to_sleep_time(pwr_mode: u8) -> u16;
}
extern "C" {
    pub fn cyapa_i2c_pip_read(cyapa: *mut cyapa, buf: *mut u8, size: usize) -> isize;
}
extern "C" {
    pub fn cyapa_i2c_pip_write(cyapa: *mut cyapa, buf: *mut u8, size: usize) -> isize;
}
extern "C" {
    pub fn cyapa_pip_state_parse(cyapa: *mut cyapa, reg_data: *mut u8, len: c_int) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_sort_system_info_data(cyapa: *mut cyapa, buf: *mut u8, len: c_int) -> bool;
}
extern "C" {
    pub fn cyapa_sort_tsg_pip_bl_resp_data(cyapa: *mut cyapa, data: *mut u8, len: c_int) -> bool;
}
extern "C" {
    pub fn cyapa_pip_deep_sleep(cyapa: *mut cyapa, state: u8) -> c_int;
}
extern "C" {
    pub fn cyapa_sort_tsg_pip_app_resp_data(cyapa: *mut cyapa, data: *mut u8, len: c_int) -> bool;
}
extern "C" {
    pub fn cyapa_pip_bl_exit(cyapa: *mut cyapa) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_bl_enter(cyapa: *mut cyapa) -> c_int;
}
extern "C" {
    pub fn cyapa_is_pip_bl_mode(cyapa: *mut cyapa) -> bool;
}
extern "C" {
    pub fn cyapa_is_pip_app_mode(cyapa: *mut cyapa) -> bool;
}
extern "C" {
    pub fn cyapa_pip_cmd_state_initialize(cyapa: *mut cyapa) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_resume_scanning(cyapa: *mut cyapa) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_suspend_scanning(cyapa: *mut cyapa) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_check_fw(cyapa: *mut cyapa, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_bl_initiate(cyapa: *mut cyapa, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_do_fw_update(cyapa: *mut cyapa, fw: *const firmware) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_bl_activate(cyapa: *mut cyapa) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_bl_deactivate(cyapa: *mut cyapa) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_set_proximity(cyapa: *mut cyapa, enable: bool) -> c_int;
}
extern "C" {
    pub fn cyapa_pip_irq_cmd_handler(cyapa: *mut cyapa) -> bool;
}
extern "C" {
    pub fn cyapa_pip_irq_handler(cyapa: *mut cyapa) -> c_int;
}
