//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/input/touchscreen/goodix.h
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

// Register defines
pub const GOODIX_REG_MISCTL_DSP_CTL: c_uint = 0x4010;
pub const GOODIX_REG_MISCTL_SRAM_BANK: c_uint = 0x4048;
pub const GOODIX_REG_MISCTL_MEM_CD_EN: c_uint = 0x4049;
pub const GOODIX_REG_MISCTL_CACHE_EN: c_uint = 0x404B;
pub const GOODIX_REG_MISCTL_TMR0_EN: c_uint = 0x40B0;
pub const GOODIX_REG_MISCTL_SWRST: c_uint = 0x4180;
pub const GOODIX_REG_MISCTL_CPU_SWRST_PULSE: c_uint = 0x4184;
pub const GOODIX_REG_MISCTL_BOOTCTL: c_uint = 0x4190;
pub const GOODIX_REG_MISCTL_BOOT_OPT: c_uint = 0x4218;
pub const GOODIX_REG_MISCTL_BOOT_CTL: c_uint = 0x5094;
pub const GOODIX_REG_FW_SIG: c_uint = 0x8000;
pub const GOODIX_FW_SIG_LEN: c_int = 10;
pub const GOODIX_REG_MAIN_CLK: c_uint = 0x8020;
pub const GOODIX_MAIN_CLK_LEN: c_int = 6;
pub const GOODIX_REG_COMMAND: c_uint = 0x8040;
pub const GOODIX_CMD_SCREEN_OFF: c_uint = 0x05;
pub const GOODIX_REG_SW_WDT: c_uint = 0x8041;
pub const GOODIX_REG_REQUEST: c_uint = 0x8043;
pub const GOODIX_RQST_RESPONDED: c_uint = 0x00;
pub const GOODIX_RQST_CONFIG: c_uint = 0x01;
pub const GOODIX_RQST_BAK_REF: c_uint = 0x02;
pub const GOODIX_RQST_RESET: c_uint = 0x03;
pub const GOODIX_RQST_MAIN_CLOCK: c_uint = 0x04;
//
// Unknown request which gets send by the controller aprox.
// every 34 seconds once it is up and running.
//
pub const GOODIX_RQST_UNKNOWN: c_uint = 0x06;
pub const GOODIX_RQST_IDLE: c_uint = 0xFF;
pub const GOODIX_REG_STATUS: c_uint = 0x8044;
pub const GOODIX_GT1X_REG_CONFIG_DATA: c_uint = 0x8050;
pub const GOODIX_GT9X_REG_CONFIG_DATA: c_uint = 0x8047;
pub const GOODIX_REG_ID: c_uint = 0x8140;
pub const GOODIX_READ_COOR_ADDR: c_uint = 0x814E;
pub const GOODIX_REG_BAK_REF: c_uint = 0x99D0;
pub const GOODIX_ID_MAX_LEN: c_int = 4;
pub const GOODIX_CONFIG_MAX_LENGTH: c_int = 240;
pub const GOODIX_MAX_KEYS: c_int = 7;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum goodix_irq_pin_access_method {
    IRQ_PIN_ACCESS_NONE,
    IRQ_PIN_ACCESS_GPIO,
    IRQ_PIN_ACCESS_ACPI_GPIO,
    IRQ_PIN_ACCESS_ACPI_METHOD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_chip_data {
    pub config_addr: u16,
    pub config_len: c_int,
    pub len): *const *const *const *const int (check_config)(struct goodix_ts_data ts, u8 cfg, int,
    pub ts): *mut *mut void (calc_config_checksum)(struct goodix_ts_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct goodix_ts_data {
    pub client: *mut i2c_client,
    pub input_dev: *mut input_dev,
    pub input_pen: *mut input_dev,
    pub chip: *const goodix_chip_data,
    pub firmware_name: *const c_char,
    pub prop: touchscreen_properties,
    pub max_touch_num: c_uint,
    pub int_trigger_type: c_uint,
    pub avdd28: *mut regulator,
    pub vddio: *mut regulator,
    pub gpiod_int: *mut gpio_desc,
    pub gpiod_rst: *mut gpio_desc,
    pub gpio_count: c_int,
    pub gpio_int_idx: c_int,
    pub 1]: char id[GOODIX_ID_MAX_LEN +,
    pub cfg_name: [c_char; 64],
    pub version: u16,
    pub reset_controller_at_probe: bool,
    pub load_cfg_from_disk: bool,
    pub pen_input_registered: c_int,
    pub firmware_loading_complete: completion,
    pub irq_flags: c_ulong,
    pub irq_pin_access_method: goodix_irq_pin_access_method,
    pub contact_size: c_uint,
    pub config: [u8; GOODIX_CONFIG_MAX_LENGTH],
    pub keymap: [c_ushort; GOODIX_MAX_KEYS],
    pub main_clk: [u8; GOODIX_MAIN_CLK_LEN],
    pub bak_ref_len: c_int,
    pub bak_ref: *mut u8,
}

extern "C" {
    pub fn goodix_i2c_read(client: *mut i2c_client, reg: u16, buf: *mut u8, len: c_int) -> c_int;
}
extern "C" {
    pub fn goodix_i2c_write(client: *mut i2c_client, reg: u16, buf: *const u8, len: c_int) -> c_int;
}
extern "C" {
    pub fn goodix_i2c_write_u8(client: *mut i2c_client, reg: u16, value: u8) -> c_int;
}
extern "C" {
    pub fn goodix_send_cfg(ts: *mut goodix_ts_data, cfg: *const u8, len: c_int) -> c_int;
}
extern "C" {
    pub fn goodix_int_sync(ts: *mut goodix_ts_data) -> c_int;
}
extern "C" {
    pub fn goodix_reset_no_int_sync(ts: *mut goodix_ts_data) -> c_int;
}
extern "C" {
    pub fn goodix_firmware_check(ts: *mut goodix_ts_data) -> c_int;
}
extern "C" {
    pub fn goodix_handle_fw_request(ts: *mut goodix_ts_data) -> bool;
}
extern "C" {
    pub fn goodix_save_bak_ref(ts: *mut goodix_ts_data);
}
