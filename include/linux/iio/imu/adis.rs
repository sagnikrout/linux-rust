//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/imu/adis.h
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
// Common library for ADIS16XXX devices
//
// Copyright 2012 Analog Devices Inc.
// Author: Lars-Peter Clausen <lars@metafoo.de>
//

pub const ADIS_PAGE_SIZE: c_uint = 0x80;
pub const ADIS_REG_PAGE_ID: c_uint = 0x00;
//
// struct adis_timeouts - ADIS chip variant timeouts
// @reset_ms - Wait time after rst pin goes inactive
// @sw_reset_ms - Wait time after sw reset command
// @self_test_ms - Wait time after self test command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis_timeout {
    pub reset_ms: u16,
    pub sw_reset_ms: u16,
    pub self_test_ms: u16,
}

//
// struct adis_data - ADIS chip variant specific data
// @read_delay: SPI delay for read operations in us
// @write_delay: SPI delay for write operations in us
// @cs_change_delay: SPI delay between CS changes in us
// @glob_cmd_reg: Register address of the GLOB_CMD register
// @msc_ctrl_reg: Register address of the MSC_CTRL register
// @diag_stat_reg: Register address of the DIAG_STAT register
// @diag_stat_size:	Length (in bytes) of the DIAG_STAT register. If 0 the
// default length is 2 bytes long.
// @prod_id_reg: Register address of the PROD_ID register
// @prod_id: Product ID code that should be expected when reading @prod_id_reg
// @self_test_mask: Bitmask of supported self-test operations
// @self_test_reg: Register address to request self test command
// @self_test_no_autoclear: True if device's self-test needs clear of ctrl reg
// @status_error_msgs: Array of error messages
// @status_error_mask: Bitmask of errors supported by the device
// @timeouts: Chip specific delays
// @enable_irq: Hook for ADIS devices that have a special IRQ enable/disable
// @unmasked_drdy: True for devices that cannot mask/unmask the data ready pin
// @has_paging: True if ADIS device has paged registers
// @has_fifo: True if ADIS device has a hardware FIFO
// @burst_reg_cmd:	Register command that triggers burst
// @burst_len:		Burst size in the SPI RX buffer. If @burst_max_len is defined,
// this should be the minimum size supported by the device.
// @burst_max_len:	Holds the maximum burst size when the device supports
// more than one burst mode with different sizes
// @burst_max_speed_hz:	Maximum spi speed that can be used in burst mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis_data {
    pub read_delay: c_uint,
    pub write_delay: c_uint,
    pub cs_change_delay: c_uint,
    pub glob_cmd_reg: c_uint,
    pub msc_ctrl_reg: c_uint,
    pub diag_stat_reg: c_uint,
    pub diag_stat_size: c_uint,
    pub prod_id_reg: c_uint,
    pub prod_id: c_uint,
    pub self_test_mask: c_uint,
    pub self_test_reg: c_uint,
    pub self_test_no_autoclear: bool,
    pub timeouts: *const adis_timeout,
    pub status_error_msgs: *const *const c_char,
    pub status_error_mask: c_uint,
    pub enable): *mut *mut *mut int (enable_irq)(struct adis adis, bool,
    pub unmasked_drdy: bool,
    pub has_paging: bool,
    pub has_fifo: bool,
    pub burst_reg_cmd: c_uint,
    pub burst_len: c_uint,
    pub burst_max_len: c_uint,
    pub burst_max_speed_hz: c_uint,
}

//
// struct adis_ops: Custom ops for adis devices.
// @write: Custom spi write implementation.
// @read: Custom spi read implementation.
// @reset: Custom sw reset implementation. The custom implementation does not
// need to sleep after the reset. It's done by the library already.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis_ops {
    pub size): c_uint,
    pub size): c_uint,
    pub adis): *mut *mut int (reset)(struct adis,
}

//
// struct adis - ADIS device instance data
// @spi: Reference to SPI device which owns this ADIS IIO device
// @trig: IIO trigger object data
// @data: ADIS chip variant specific data
// @burst_extra_len: Burst extra length. Should only be used by devices that can
// dynamically change their burst mode length.
// @ops: ops struct for custom read and write functions
// @state_lock: Lock used by the device to protect state
// @msg: SPI message object
// @xfer: SPI transfer objects to be used for a @msg
// @current_page: Some ADIS devices have registers, this selects current page
// @irq_flag: IRQ handling flags as passed to request_irq()
// @buffer: Data buffer for information read from the device
// @tx: DMA safe TX buffer for SPI transfers
// @rx: DMA safe RX buffer for SPI transfers
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adis {
    pub spi: *mut spi_device,
    pub trig: *mut iio_trigger,
    pub data: *const adis_data,
    pub burst_extra_len: c_uint,
    pub ops: *const adis_ops,
//
// The state_lock is meant to be used during operations that require
// a sequence of SPI R/W in order to protect the SPI transfer
// information (fields 'xfer', 'msg' & 'current_page') between
// potential concurrent accesses.
// This lock is used by all "adis_{functions}" that have to read/write
// registers. These functions also have unlocked variants
// (see "__adis_{functions}"), which don't hold this lock.
// This allows users of the ADIS library to group SPI R/W into
// the drivers, but they also must manage this lock themselves.
//
    pub state_lock: mutex,
    pub msg: spi_message,
    pub xfer: *mut spi_transfer,
    pub current_page: c_uint,
    pub irq_flag: c_ulong,
    pub buffer: *mut c_void,
    pub __aligned(IIO_DMA_MINALIGN): u8 tx[10],
    pub rx: [u8; 4],
}

extern "C" {
    pub fn __adis_reset(adis: *mut adis) -> c_int;
}
//
// adis_reset() - Reset the device
// @adis: The adis device
//
// Returns: %0 on success, a negative error code otherwise
//
extern "C" {
    pub fn __adis_reset(_arg: adis) -> return;
}
//
// __adis_write_reg_8() - Write single byte to a register (unlocked)
// @adis: The adis device
// @reg: The address of the register to be written
// @val: The value to write
//
// Returns: %0 on success, a negative error code otherwise
//
// __adis_write_reg_16() - Write 2 bytes to a pair of registers (unlocked)
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: Value to be written
//
// Returns: %0 on success, a negative error code otherwise
//
// __adis_write_reg_32() - write 4 bytes to four registers (unlocked)
// @adis: The adis device
// @reg: The address of the lower of the four register
// @val: Value to be written
//
// Returns: %0 on success, a negative error code otherwise
//
// __adis_read_reg_16() - read 2 bytes from a 16-bit register (unlocked)
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: The value read back from the device
//
// Returns: %0 on success, a negative error code otherwise
//
// val = tmp;
//
// __adis_read_reg_32() - read 4 bytes from a 32-bit register (unlocked)
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: The value read back from the device
//
// Returns: %0 on success, a negative error code otherwise
//
// val = tmp;
//
// adis_write_reg() - write N bytes to register
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: The value to write to device (up to 4 bytes)
// @size: The size of the @value (in bytes)
//
// Returns: %0 on success, a negative error code otherwise
//
// adis_read_reg() - read N bytes from register
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: The value read back from the device
// @size: The size of the @val buffer
//
// Returns: %0 on success, a negative error code otherwise
//
// adis_write_reg_8() - Write single byte to a register
// @adis: The adis device
// @reg: The address of the register to be written
// @val: The value to write
//
// Returns: %0 on success, a negative error code otherwise
//
extern "C" {
    pub fn adis_write_reg(_arg: adis, _arg: reg, _arg: val, _arg: 1) -> return;
}
//
// adis_write_reg_16() - Write 2 bytes to a pair of registers
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: Value to be written
//
// Returns: %0 on success, a negative error code otherwise
//
extern "C" {
    pub fn adis_write_reg(_arg: adis, _arg: reg, _arg: val, _arg: 2) -> return;
}
//
// adis_write_reg_32() - write 4 bytes to four registers
// @adis: The adis device
// @reg: The address of the lower of the four register
// @val: Value to be written
//
// Returns: %0 on success, a negative error code otherwise
//
extern "C" {
    pub fn adis_write_reg(_arg: adis, _arg: reg, _arg: val, _arg: 4) -> return;
}
//
// adis_read_reg_16() - read 2 bytes from a 16-bit register
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: The value read back from the device
//
// Returns: %0 on success, a negative error code otherwise
//
// val = tmp;
//
// adis_read_reg_32() - read 4 bytes from a 32-bit register
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @val: The value read back from the device
//
// Returns: %0 on success, a negative error code otherwise
//
// val = tmp;
//
// adis_update_bits_base() - ADIS Update bits function - Locked version
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @mask: Bitmask to change
// @val: Value to be written
// @size: Size of the register to update
//
// Updates the desired bits of @reg in accordance with @mask and @val.
//
// Returns: %0 on success, a negative error code otherwise
//
extern "C" {
    pub fn __adis_update_bits_base(_arg: adis, _arg: reg, _arg: mask, _arg: val, _arg: size) -> return;
}
//
// adis_update_bits() - Wrapper macro for adis_update_bits_base - Locked version
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @mask: Bitmask to change
// @val: Value to be written
//
// This macro evaluates the sizeof of @val at compile time and calls
// adis_update_bits_base() accordingly. Be aware that using MACROS/DEFINES for
// @val can lead to undesired behavior if the register to update is 16bit.
//

//
// adis_update_bits() - Wrapper macro for adis_update_bits_base
// @adis: The adis device
// @reg: The address of the lower of the two registers
// @mask: Bitmask to change
// @val: Value to be written
//
// This macro evaluates the sizeof of @val at compile time and calls
// adis_update_bits_base() accordingly. Be aware that using MACROS/DEFINES for
// @val can lead to undesired behavior if the register to update is 16bit.
//

extern "C" {
    pub fn __adis_check_status(adis: *mut adis) -> c_int;
}
extern "C" {
    pub fn __adis_initial_startup(adis: *mut adis) -> c_int;
}
extern "C" {
    pub fn __adis_enable_irq(adis: *mut adis, enable: bool) -> c_int;
}
extern "C" {
    pub fn __adis_enable_irq(_arg: adis, _arg: enable) -> return;
}
extern "C" {
    pub fn __adis_check_status(_arg: adis) -> return;
}

extern "C" {
    pub fn devm_adis_probe_trigger(adis: *mut adis, indio_dev: *mut iio_dev) -> c_int;
}

