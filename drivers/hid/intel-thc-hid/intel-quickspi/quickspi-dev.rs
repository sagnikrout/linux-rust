//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-thc-hid/intel-quickspi/quickspi-dev.h
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

pub const PCI_DEVICE_ID_INTEL_THC_MTL_DEVICE_ID_SPI_PORT1: c_uint = 0x7E49;
pub const PCI_DEVICE_ID_INTEL_THC_MTL_DEVICE_ID_SPI_PORT2: c_uint = 0x7E4B;
pub const PCI_DEVICE_ID_INTEL_THC_LNL_DEVICE_ID_SPI_PORT1: c_uint = 0xA849;
pub const PCI_DEVICE_ID_INTEL_THC_LNL_DEVICE_ID_SPI_PORT2: c_uint = 0xA84B;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_H_DEVICE_ID_SPI_PORT1: c_uint = 0xE349;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_H_DEVICE_ID_SPI_PORT2: c_uint = 0xE34B;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_U_DEVICE_ID_SPI_PORT1: c_uint = 0xE449;
pub const PCI_DEVICE_ID_INTEL_THC_PTL_U_DEVICE_ID_SPI_PORT2: c_uint = 0xE44B;
pub const PCI_DEVICE_ID_INTEL_THC_WCL_DEVICE_ID_SPI_PORT1: c_uint = 0x4D49;
pub const PCI_DEVICE_ID_INTEL_THC_WCL_DEVICE_ID_SPI_PORT2: c_uint = 0x4D4B;
pub const PCI_DEVICE_ID_INTEL_THC_ARL_DEVICE_ID_SPI_PORT1: c_uint = 0x7749;
pub const PCI_DEVICE_ID_INTEL_THC_ARL_DEVICE_ID_SPI_PORT2: c_uint = 0x774B;
pub const PCI_DEVICE_ID_INTEL_THC_NVL_H_DEVICE_ID_SPI_PORT1: c_uint = 0xD349;
pub const PCI_DEVICE_ID_INTEL_THC_NVL_H_DEVICE_ID_SPI_PORT2: c_uint = 0xD34B;
// HIDSPI special ACPI parameters DSM methods
pub const ACPI_QUICKSPI_REVISION_NUM: c_int = 2;
pub const ACPI_QUICKSPI_FUNC_NUM_INPUT_REP_HDR_ADDR: c_int = 1;
pub const ACPI_QUICKSPI_FUNC_NUM_INPUT_REP_BDY_ADDR: c_int = 2;
pub const ACPI_QUICKSPI_FUNC_NUM_OUTPUT_REP_ADDR: c_int = 3;
pub const ACPI_QUICKSPI_FUNC_NUM_READ_OPCODE: c_int = 4;
pub const ACPI_QUICKSPI_FUNC_NUM_WRITE_OPCODE: c_int = 5;
pub const ACPI_QUICKSPI_FUNC_NUM_IO_MODE: c_int = 6;
// QickSPI device special ACPI parameters DSM methods
pub const ACPI_QUICKSPI_FUNC_NUM_CONNECTION_SPEED: c_int = 1;
pub const ACPI_QUICKSPI_FUNC_NUM_LIMIT_PACKET_SIZE: c_int = 2;
pub const ACPI_QUICKSPI_FUNC_NUM_PERFORMANCE_LIMIT: c_int = 3;
// Platform special ACPI parameters DSM methods
pub const ACPI_QUICKSPI_FUNC_NUM_ACTIVE_LTR: c_int = 1;
pub const ACPI_QUICKSPI_FUNC_NUM_LP_LTR: c_int = 2;

// Packet size value, the unit is 16 bytes
pub const DEFAULT_MIN_PACKET_SIZE_VALUE: c_int = 4;
pub const MAX_PACKET_SIZE_VALUE_MTL: c_int = 128;
pub const MAX_PACKET_SIZE_VALUE_LNL: c_int = 256;
//
// THC uses runtime auto suspend to dynamically switch between THC active LTR
// and low power LTR to save CPU power.
// Default value is 5000ms, that means if no touch event in this time, THC will
// change to low power LTR mode.
//
pub const DEFAULT_AUTO_SUSPEND_DELAY_MS: c_int = 5000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum quickspi_dev_state {
    QUICKSPI_NONE,
    QUICKSPI_INITIATED,
    QUICKSPI_RESETING,
    QUICKSPI_RESET,
    QUICKSPI_ENABLED,
    QUICKSPI_DISABLED,
}

//
// struct quickspi_driver_data - Driver specific data for quickspi device
// @max_packet_size_value: identify max packet size, unit is 16 bytes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quickspi_driver_data {
    pub max_packet_size_value: u32,
}

//
// struct quickspi_device -  THC QuickSpi device struct
// @dev: point to kernel device
// @pdev: point to PCI device
// @thc_hw: point to THC device
// @hid_dev: point to hid device
// @acpi_dev: point to ACPI device
// @driver_data: point to quickspi specific driver data
// @state: THC SPI device state
// @mem_addr: MMIO memory address
// @dev_desc: device descriptor for HIDSPI protocol
// @input_report_hdr_addr: device input report header address
// @input_report_bdy_addr: device input report body address
// @output_report_bdy_addr: device output report address
// @spi_freq_val: device supported max SPI frequnecy, in Hz
// @spi_read_io_mode: device supported SPI read io mode
// @spi_write_io_mode: device supported SPI write io mode
// @spi_read_opcode: device read opcode
// @spi_write_opcode: device write opcode
// @limit_packet_size: 1 - limit read/write packet to 64Bytes
// 0 - device no packet size limiation for read/write
// @performance_limit: delay time, in ms.
// if device has performance limitation, must give a delay
// before write operation after a read operation.
// @active_ltr_val: THC active LTR value
// @low_power_ltr_val: THC low power LTR value
// @report_descriptor: store a copy of device report descriptor
// @input_buf: store a copy of latest input report data
// @report_buf: store a copy of latest input/output report packet from set/get feature
// @report_len: the length of input/output report packet
// @reset_ack_wq: workqueue for waiting reset response from device
// @reset_ack: indicate reset response received or not
// @nondma_int_received_wq: workqueue for waiting THC non-DMA interrupt
// @nondma_int_received: indicate THC non-DMA interrupt received or not
// @report_desc_got_wq: workqueue for waiting device report descriptor
// @report_desc_got: indicate device report descritor received or not
// @set_power_on_wq: workqueue for waiting set power on response from device
// @set_power_on: indicate set power on response received or not
// @get_feature_cmpl_wq: workqueue for waiting get feature response from device
// @get_feature_cmpl: indicate get feature received or not
// @set_feature_cmpl_wq: workqueue for waiting set feature to device
// @set_feature_cmpl: indicate set feature send complete or not
// @recover_work: Work structure for recovery
// @recovery_disabled: Whether recovery work is blocked during teardown
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct quickspi_device {
    pub dev: *mut device,
    pub pdev: *mut pci_dev,
    pub thc_hw: *mut thc_device,
    pub hid_dev: *mut hid_device,
    pub acpi_dev: *mut acpi_device,
    pub driver_data: *mut quickspi_driver_data,
    pub state: quickspi_dev_state,
    pub mem_addr: *mut void __iomem,
    pub dev_desc: hidspi_dev_descriptor,
    pub input_report_hdr_addr: u32,
    pub input_report_bdy_addr: u32,
    pub output_report_addr: u32,
    pub spi_freq_val: u32,
    pub spi_read_io_mode: u32,
    pub spi_write_io_mode: u32,
    pub spi_read_opcode: u32,
    pub spi_write_opcode: u32,
    pub limit_packet_size: u32,
    pub spi_packet_size: u32,
    pub performance_limit: u32,
    pub active_ltr_val: u32,
    pub low_power_ltr_val: u32,
    pub report_descriptor: *mut u8,
    pub input_buf: *mut u8,
    pub report_buf: *mut u8,
    pub report_buf_size: u32,
    pub report_len: u32,
    pub reset_ack_wq: wait_queue_head_t,
    pub reset_ack: bool,
    pub nondma_int_received_wq: wait_queue_head_t,
    pub nondma_int_received: bool,
    pub report_desc_got_wq: wait_queue_head_t,
    pub report_desc_got: bool,
    pub get_report_cmpl_wq: wait_queue_head_t,
    pub get_report_cmpl: bool,
    pub set_report_cmpl_wq: wait_queue_head_t,
    pub set_report_cmpl: bool,
    pub recover_work: work_struct,
    pub recovery_disabled: bool,
}
