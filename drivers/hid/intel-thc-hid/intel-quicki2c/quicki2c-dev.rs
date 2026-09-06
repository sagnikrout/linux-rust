//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-thc-hid/intel-quicki2c/quicki2c-dev.h
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
// Copyright (c) 2024 Intel Corporation

pub const PCI_DEVICE_ID_INTEL_THC_LNL_DEVICE_ID_I2C_PORT1: c_uint = 0xA848;
pub const PCI_DEVICE_ID_INTEL_THC_LNL_DEVICE_ID_I2C_PORT2: c_uint = 0xA84A;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_H_DEVICE_ID_I2C_PORT1: c_uint = 0xE348;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_H_DEVICE_ID_I2C_PORT2: c_uint = 0xE34A;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_U_DEVICE_ID_I2C_PORT1: c_uint = 0xE448;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_U_DEVICE_ID_I2C_PORT2: c_uint = 0xE44A;
pub const PCI_DEVICE_ID_INTEL_THC_WCL_DEVICE_ID_I2C_PORT1: c_uint = 0x4D48;
pub const PCI_DEVICE_ID_INTEL_THC_WCL_DEVICE_ID_I2C_PORT2: c_uint = 0x4D4A;
pub const PCI_DEVICE_ID_INTEL_THC_NVL_H_DEVICE_ID_I2C_PORT1: c_uint = 0xD348;
pub const PCI_DEVICE_ID_INTEL_THC_NVL_H_DEVICE_ID_I2C_PORT2: c_uint = 0xD34A;
// Packet size value, the unit is 16 bytes
pub const MAX_PACKET_SIZE_VALUE_LNL: c_int = 256;
// HIDI2C special ACPI parameters DSD name

// HIDI2C special ACPI parameters DSM methods
pub const QUICKI2C_ACPI_REVISION_NUM: c_int = 1;
pub const QUICKI2C_ACPI_FUNC_NUM_HID_DESC_ADDR: c_int = 1;
pub const QUICKI2C_ACPI_FUNC_NUM_ACTIVE_LTR_VAL: c_int = 1;
pub const QUICKI2C_ACPI_FUNC_NUM_LP_LTR_VAL: c_int = 2;
pub const QUICKI2C_SUBIP_STANDARD_MODE_MAX_SPEED: c_int = 100000;
pub const QUICKI2C_SUBIP_FAST_MODE_MAX_SPEED: c_int = 400000;
pub const QUICKI2C_SUBIP_FASTPLUS_MODE_MAX_SPEED: c_int = 1000000;
pub const QUICKI2C_SUBIP_HIGH_SPEED_MODE_MAX_SPEED: c_int = 3400000;
pub const QUICKI2C_DEFAULT_ACTIVE_LTR_VALUE: c_int = 5;
pub const QUICKI2C_DEFAULT_LP_LTR_VALUE: c_int = 500;
pub const QUICKI2C_RPM_TIMEOUT_MS: c_int = 500;
// PTL Max packet size detection capability is 255 Bytes
pub const MAX_RX_DETECT_SIZE_PTL: c_int = 255;
// NVL Max packet size detection capability is 64K Bytes
pub const MAX_RX_DETECT_SIZE_NVL: c_int = 65535;
// Max interrupt delay capability is 2.56ms
pub const MAX_RX_INTERRUPT_DELAY: c_int = 256;
// Default interrupt delay is 1ms, suitable for most devices

//
// THC uses runtime auto suspend to dynamically switch between THC active LTR
// and low power LTR to save CPU power.
// Default value is 5000ms, that means if no touch event in this time, THC will
// change to low power LTR mode.
//
pub const DEFAULT_AUTO_SUSPEND_DELAY_MS: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum quicki2c_dev_state {
    QUICKI2C_NONE,
    QUICKI2C_RESETING,
    QUICKI2C_RESETED,
    QUICKI2C_INITED,
    QUICKI2C_ENABLED,
    QUICKI2C_DISABLED,
}

//
// struct quicki2c_subip_acpi_parameter - QuickI2C ACPI DSD parameters
// @device_address: I2C device slave address
// @connection_speed: I2C device expected connection speed
// @addressing_mode: I2C device slave address mode, 7bit or 10bit
//
// Those properties get from QUICKI2C_ACPI_METHOD_NAME_ICRS method, used for
// Bus parameter.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quicki2c_subip_acpi_parameter {
    pub device_address: u16,
    pub connection_speed: u64,
    pub addressing_mode: u8,
    pub reserved: u8,
    pub __packed: },
//
// struct quicki2c_subip_acpi_config - QuickI2C ACPI DSD parameters
// @SMHX: Standard Mode (100 kbit/s) Serial Clock Line HIGH Period
// @SMLX: Standard Mode (100 kbit/s) Serial Clock Line LOW Period
// @SMTD: Standard Mode (100 kbit/s) Serial Data Line Transmit Hold Period
// @SMRD: Standard Mode (100 kbit/s) Serial Data Receive Hold Period
// @FMHX: Fast Mode (400 kbit/s) Serial Clock Line HIGH Period
// @FMLX: Fast Mode (400 kbit/s) Serial Clock Line LOW Period
// @FMTD: Fast Mode (400 kbit/s) Serial Data Line Transmit Hold Period
// @FMRD: Fast Mode (400 kbit/s) Serial Data Line Receive Hold Period
// @FMSL: Maximum length (in ic_clk_cycles) of suppressed spikes
// in Standard Mode, Fast Mode and Fast Mode Plus
// @FPHX: Fast Mode Plus (1Mbit/sec) Serial Clock Line HIGH Period
// @FPLX: Fast Mode Plus (1Mbit/sec) Serial Clock Line LOW Period
// @FPTD: Fast Mode Plus (1Mbit/sec) Serial Data Line Transmit HOLD Period
// @FPRD: Fast Mode Plus (1Mbit/sec) Serial Data Line Receive HOLD Period
// @HMHX: High Speed Mode Plus (3.4Mbits/sec) Serial Clock Line HIGH Period
// @HMLX: High Speed Mode Plus (3.4Mbits/sec) Serial Clock Line LOW Period
// @HMTD: High Speed Mode Plus (3.4Mbits/sec) Serial Data Line Transmit HOLD Period
// @HMRD: High Speed Mode Plus (3.4Mbits/sec) Serial Data Line Receive HOLD Period
// @HMSL: Maximum length (in ic_clk_cycles) of suppressed spikes in High Speed Mode
// @FSEN: Maximum Frame Size Feature Enable Control
// @FSVL: Maximum Frame Size Value (unit in Bytes)
// @INDE: Interrupt Delay Feature Enable Control
// @INDV: Interrupt Delay Value (unit in 10 us)
//
// Those properties get from QUICKI2C_ACPI_METHOD_NAME_ISUB method, used for
// I2C timing configure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quicki2c_subip_acpi_config {
    pub SMHX: u64,
    pub SMLX: u64,
    pub SMTD: u64,
    pub SMRD: u64,
    pub FMHX: u64,
    pub FMLX: u64,
    pub FMTD: u64,
    pub FMRD: u64,
    pub FMSL: u64,
    pub FPHX: u64,
    pub FPLX: u64,
    pub FPTD: u64,
    pub FPRD: u64,
    pub HMHX: u64,
    pub HMLX: u64,
    pub HMTD: u64,
    pub HMRD: u64,
    pub HMSL: u64,
    pub FSEN: u64,
    pub FSVL: u64,
    pub INDE: u64,
    pub INDV: u64,
    pub reserved: u8,
}

//
// struct quicki2c_ddata - Driver specific data for quicki2c device
// @max_detect_size: Identify max packet size detect for rx
// @interrupt_delay: Identify max interrupt detect delay for rx
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quicki2c_ddata {
    pub max_detect_size: u32,
    pub max_interrupt_delay: u32,
}

//
// struct quicki2c_device -  THC QuickI2C device struct
// @dev: Point to kernel device
// @pdev: Point to PCI device
// @thc_hw: Point to THC device
// @hid_dev: Point to HID device
// @acpi_dev: Point to ACPI device
// @ddata: Point to QuickI2C platform specific driver data
// @state: THC I2C device state
// @mem_addr: MMIO memory address
// @dev_desc: Device descriptor for HIDI2C protocol
// @i2c_config: I2C bus configuration
// @hid_desc_addr: Register address for retrieve HID device descriptor
// @active_ltr_val: THC active LTR value
// @low_power_ltr_val: THC low power LTR value
// @report_descriptor: Store a copy of device report descriptor
// @input_buf: Store a copy of latest input report data
// @report_buf: Store a copy of latest input/output report packet from set/get feature
// @report_len: The length of input/output report packet
// @reset_ack_wq: Workqueue for waiting reset response from device
// @reset_ack: Indicate reset response received or not
// @i2c_max_frame_size_enable: Indicate max frame size feature enabled or not
// @i2c_max_frame_size: Max RX frame size (unit in Bytes)
// @i2c_int_delay_enable: Indicate interrupt delay feature enabled or not
// @i2c_int_delay: Interrupt detection delay value (unit in 10 us)
// @recover_work: Work structure for recovery
// @recovery_disabled: Whether recovery work is blocked during teardown
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quicki2c_device {
    pub dev: *mut device,
    pub pdev: *mut pci_dev,
    pub thc_hw: *mut thc_device,
    pub hid_dev: *mut hid_device,
    pub acpi_dev: *mut acpi_device,
    pub ddata: *const quicki2c_ddata,
    pub state: quicki2c_dev_state,
    pub mem_addr: *mut void __iomem,
    pub dev_desc: hidi2c_dev_descriptor,
    pub i2c_config: thc_i2c_config,
    pub hid_desc_addr: u16,
    pub active_ltr_val: u32,
    pub low_power_ltr_val: u32,
    pub report_descriptor: *mut u8,
    pub input_buf: *mut u8,
    pub report_buf: *mut u8,
    pub report_len: usize,
    pub reset_ack_wq: wait_queue_head_t,
    pub reset_ack: bool,
    pub i2c_max_frame_size_enable: u32,
    pub i2c_max_frame_size: u32,
    pub i2c_int_delay_enable: u32,
    pub i2c_int_delay: u32,
    pub recover_work: work_struct,
    pub recovery_disabled: bool,
}
