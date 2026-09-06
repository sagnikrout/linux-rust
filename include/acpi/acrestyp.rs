//! Automatically rewritten from C Header to Rust Module
//! Source: include/acpi/acrestyp.h
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


// SPDX-License-Identifier: BSD-3-Clause OR GPL-2.0
//
// Name: acrestyp.h - Defines, types, and structures for resource descriptors
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// Definitions for Resource Attributes
//
// Memory Attributes
//

// ! [Begin] no source code translation
//
// IO Attributes
// The ISA IO ranges are:     n000-n0FFh,  n400-n4FFh, n800-n8FFh, nC00-nCFFh.
// The non-ISA IO ranges are: n100-n3FFh,  n500-n7FFh, n900-nBFFh, nCD0-nFFFh.
//
// ! [End] no source code translation !

// Type of translation - 1=Sparse, 0=Dense

//
// IO Port Descriptor Decode
//

//
// Interrupt attributes - used in multiple descriptors
//
// Triggering

// Polarity

// Sharing

// Wake

//
// DMA Attributes
//

//
// Start Dependent Functions Priority definitions
//

//
// 16, 32 and 64-bit Address Descriptor resource types
//

// Producer/Consumer

//
// If possible, pack the following structures to byte alignment
//

// UUID data structures for use in vendor-defined resource descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_uuid {
    pub data: [u8; ACPI_UUID_LENGTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_vendor_uuid {
    pub subtype: u8,
    pub data: [u8; ACPI_UUID_LENGTH],
}

//
// Structures used to describe device resources
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_irq {
    pub descriptor_length: u8,
    pub triggering: u8,
    pub polarity: u8,
    pub shareable: u8,
    pub wake_capable: u8,
    pub interrupt_count: u8,
    pub interrupt: u8,
    pub interrupts): ACPI_FLEX_ARRAY(u8,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_dma {
    pub type: u8,
    pub bus_master: u8,
    pub transfer: u8,
    pub channel_count: u8,
    pub channel: u8,
    pub channels): ACPI_FLEX_ARRAY(u8,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_start_dependent {
    pub descriptor_length: u8,
    pub compatibility_priority: u8,
    pub performance_robustness: u8,
}

//
// The END_DEPENDENT_FUNCTIONS_RESOURCE struct is not
// needed because it has no fields
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_io {
    pub io_decode: u8,
    pub alignment: u8,
    pub address_length: u8,
    pub minimum: u16,
    pub maximum: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_fixed_io {
    pub address: u16,
    pub address_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_fixed_dma {
    pub request_lines: u16,
    pub channels: u16,
    pub width: u8,
}

// Values for Width field above
pub const ACPI_DMA_WIDTH8: c_int = 0;
pub const ACPI_DMA_WIDTH16: c_int = 1;
pub const ACPI_DMA_WIDTH32: c_int = 2;
pub const ACPI_DMA_WIDTH64: c_int = 3;
pub const ACPI_DMA_WIDTH128: c_int = 4;
pub const ACPI_DMA_WIDTH256: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_vendor {
    pub byte_length: u16,
    pub byte_data: [u8; ],
}

// Vendor resource with UUID info (introduced in ACPI 3.0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_vendor_typed {
    pub byte_length: u16,
    pub uuid_subtype: u8,
    pub uuid: [u8; ACPI_UUID_LENGTH],
    pub byte_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_end_tag {
    pub checksum: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_memory24 {
    pub write_protect: u8,
    pub minimum: u16,
    pub maximum: u16,
    pub alignment: u16,
    pub address_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_memory32 {
    pub write_protect: u8,
    pub minimum: u32,
    pub maximum: u32,
    pub alignment: u32,
    pub address_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_fixed_memory32 {
    pub write_protect: u8,
    pub address: u32,
    pub address_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_memory_attribute {
    pub write_protect: u8,
    pub caching: u8,
    pub range_type: u8,
    pub translation: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_io_attribute {
    pub range_type: u8,
    pub translation: u8,
    pub translation_type: u8,
    pub reserved1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_resource_attribute {
    pub mem: acpi_memory_attribute,
    pub io: acpi_io_attribute,
// Used for the *word_space macros
    pub type_specific: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_label {
    pub string_length: u16,
    pub string_ptr: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_source {
    pub index: u8,
    pub string_length: u16,
    pub string_ptr: *mut c_char,
}

// Fields common to all address descriptors, 16/32/64 bit

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_address16_attribute {
    pub granularity: u16,
    pub minimum: u16,
    pub maximum: u16,
    pub translation_offset: u16,
    pub address_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_address32_attribute {
    pub granularity: u32,
    pub minimum: u32,
    pub maximum: u32,
    pub translation_offset: u32,
    pub address_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_address64_attribute {
    pub granularity: u64,
    pub minimum: u64,
    pub maximum: u64,
    pub translation_offset: u64,
    pub address_length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_address {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_address16 {
    pub address: ACPI_RESOURCE_ADDRESS_COMMON struct acpi_address16_attribute,
    pub resource_source: acpi_resource_source,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_address32 {
    pub address: ACPI_RESOURCE_ADDRESS_COMMON struct acpi_address32_attribute,
    pub resource_source: acpi_resource_source,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_address64 {
    pub address: ACPI_RESOURCE_ADDRESS_COMMON struct acpi_address64_attribute,
    pub resource_source: acpi_resource_source,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_extended_address64 {
    pub revision_ID: ACPI_RESOURCE_ADDRESS_COMMON u8,
    pub address: acpi_address64_attribute,
    pub type_specific: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_extended_irq {
    pub producer_consumer: u8,
    pub triggering: u8,
    pub polarity: u8,
    pub shareable: u8,
    pub wake_capable: u8,
    pub interrupt_count: u8,
    pub resource_source: acpi_resource_source,
    pub interrupt: u32,
    pub interrupts): ACPI_FLEX_ARRAY(u32,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_generic_register {
    pub space_id: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_gpio {
    pub revision_id: u8,
    pub connection_type: u8,
    pub /: *mut *mut u8 producer_consumer; / For values, see Producer/Consumer above,
    pub pin_config: u8,
    pub /: *mut *mut u8 shareable; / For values, see Interrupt Attributes above,
    pub /: *mut *mut u8 wake_capable; / For values, see Interrupt Attributes above,
    pub io_restriction: u8,
    pub /: *mut *mut u8 triggering; / For values, see Interrupt Attributes above,
    pub /: *mut *mut u8 polarity; / For values, see Interrupt Attributes above,
    pub drive_strength: u16,
    pub debounce_timeout: u16,
    pub pin_table_length: u16,
    pub vendor_length: u16,
    pub resource_source: acpi_resource_source,
    pub pin_table: *mut u16,
    pub vendor_data: *mut u8,
}

// Values for GPIO connection_type field above
pub const ACPI_RESOURCE_GPIO_TYPE_INT: c_int = 0;
pub const ACPI_RESOURCE_GPIO_TYPE_IO: c_int = 1;
// Values for pin_config field above
pub const ACPI_PIN_CONFIG_DEFAULT: c_int = 0;
pub const ACPI_PIN_CONFIG_PULLUP: c_int = 1;
pub const ACPI_PIN_CONFIG_PULLDOWN: c_int = 2;
pub const ACPI_PIN_CONFIG_NOPULL: c_int = 3;
// Values for io_restriction field above
pub const ACPI_IO_RESTRICT_NONE: c_int = 0;
pub const ACPI_IO_RESTRICT_INPUT: c_int = 1;
pub const ACPI_IO_RESTRICT_OUTPUT: c_int = 2;
pub const ACPI_IO_RESTRICT_NONE_PRESERVE: c_int = 3;
// Common structure for I2C, SPI, UART, CSI2 serial descriptors

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_common_serialbus {
// Values for the Type field above
pub const ACPI_RESOURCE_SERIAL_TYPE_I2C: c_int = 1;
pub const ACPI_RESOURCE_SERIAL_TYPE_SPI: c_int = 2;
pub const ACPI_RESOURCE_SERIAL_TYPE_UART: c_int = 3;
pub const ACPI_RESOURCE_SERIAL_TYPE_CSI2: c_int = 4;
// Values for slave_mode field above
pub const ACPI_CONTROLLER_INITIATED: c_int = 0;
pub const ACPI_DEVICE_INITIATED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_i2c_serialbus {
    pub access_mode: ACPI_RESOURCE_SERIAL_COMMON u8,
    pub slave_address: u16,
    pub connection_speed: u32,
    pub lvr: u8,
}

// Values for access_mode field above
pub const ACPI_I2C_7BIT_MODE: c_int = 0;
pub const ACPI_I2C_10BIT_MODE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_spi_serialbus {
    pub wire_mode: ACPI_RESOURCE_SERIAL_COMMON u8,
    pub device_polarity: u8,
    pub data_bit_length: u8,
    pub clock_phase: u8,
    pub clock_polarity: u8,
    pub device_selection: u16,
    pub connection_speed: u32,
}

// Values for wire_mode field above
pub const ACPI_SPI_4WIRE_MODE: c_int = 0;
pub const ACPI_SPI_3WIRE_MODE: c_int = 1;
// Values for device_polarity field above
pub const ACPI_SPI_ACTIVE_LOW: c_int = 0;
pub const ACPI_SPI_ACTIVE_HIGH: c_int = 1;
// Values for clock_phase field above
pub const ACPI_SPI_FIRST_PHASE: c_int = 0;
pub const ACPI_SPI_SECOND_PHASE: c_int = 1;
// Values for clock_polarity field above
pub const ACPI_SPI_START_LOW: c_int = 0;
pub const ACPI_SPI_START_HIGH: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_uart_serialbus {
    pub endian: ACPI_RESOURCE_SERIAL_COMMON u8,
    pub data_bits: u8,
    pub stop_bits: u8,
    pub flow_control: u8,
    pub parity: u8,
    pub lines_enabled: u8,
    pub rx_fifo_size: u16,
    pub tx_fifo_size: u16,
    pub default_baud_rate: u32,
}

// Values for Endian field above
pub const ACPI_UART_LITTLE_ENDIAN: c_int = 0;
pub const ACPI_UART_BIG_ENDIAN: c_int = 1;
// Values for data_bits field above
pub const ACPI_UART_5_DATA_BITS: c_int = 0;
pub const ACPI_UART_6_DATA_BITS: c_int = 1;
pub const ACPI_UART_7_DATA_BITS: c_int = 2;
pub const ACPI_UART_8_DATA_BITS: c_int = 3;
pub const ACPI_UART_9_DATA_BITS: c_int = 4;
// Values for stop_bits field above
pub const ACPI_UART_NO_STOP_BITS: c_int = 0;
pub const ACPI_UART_1_STOP_BIT: c_int = 1;
pub const ACPI_UART_1P5_STOP_BITS: c_int = 2;
pub const ACPI_UART_2_STOP_BITS: c_int = 3;
// Values for flow_control field above
pub const ACPI_UART_FLOW_CONTROL_NONE: c_int = 0;
pub const ACPI_UART_FLOW_CONTROL_HW: c_int = 1;
pub const ACPI_UART_FLOW_CONTROL_XON_XOFF: c_int = 2;
// Values for Parity field above
pub const ACPI_UART_PARITY_NONE: c_int = 0;
pub const ACPI_UART_PARITY_EVEN: c_int = 1;
pub const ACPI_UART_PARITY_ODD: c_int = 2;
pub const ACPI_UART_PARITY_MARK: c_int = 3;
pub const ACPI_UART_PARITY_SPACE: c_int = 4;
// Values for lines_enabled bitfield above

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_csi2_serialbus {
    pub local_port_instance: ACPI_RESOURCE_SERIAL_COMMON u8,
    pub phy_type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_pin_function {
    pub revision_id: u8,
    pub pin_config: u8,
    pub /: *mut *mut u8 shareable; / For values, see Interrupt Attributes above,
    pub function_number: u16,
    pub pin_table_length: u16,
    pub vendor_length: u16,
    pub resource_source: acpi_resource_source,
    pub pin_table: *mut u16,
    pub vendor_data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_pin_config {
    pub revision_id: u8,
    pub /: *mut *mut u8 producer_consumer; / For values, see Producer/Consumer above,
    pub /: *mut *mut u8 shareable; / For values, see Interrupt Attributes above,
    pub pin_config_type: u8,
    pub pin_config_value: u32,
    pub pin_table_length: u16,
    pub vendor_length: u16,
    pub resource_source: acpi_resource_source,
    pub pin_table: *mut u16,
    pub vendor_data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_clock_input {
    pub revision_id: u8,
    pub mode: u8,
    pub scale: u8,
    pub frequency_divisor: u16,
    pub frequency_numerator: u32,
    pub resource_source: acpi_resource_source,
}

// Values for pin_config_type field above
pub const ACPI_PIN_CONFIG_DEFAULT: c_int = 0;
pub const ACPI_PIN_CONFIG_BIAS_PULL_UP: c_int = 1;
pub const ACPI_PIN_CONFIG_BIAS_PULL_DOWN: c_int = 2;
pub const ACPI_PIN_CONFIG_BIAS_DEFAULT: c_int = 3;
pub const ACPI_PIN_CONFIG_BIAS_DISABLE: c_int = 4;
pub const ACPI_PIN_CONFIG_BIAS_HIGH_IMPEDANCE: c_int = 5;
pub const ACPI_PIN_CONFIG_BIAS_BUS_HOLD: c_int = 6;
pub const ACPI_PIN_CONFIG_DRIVE_OPEN_DRAIN: c_int = 7;
pub const ACPI_PIN_CONFIG_DRIVE_OPEN_SOURCE: c_int = 8;
pub const ACPI_PIN_CONFIG_DRIVE_PUSH_PULL: c_int = 9;
pub const ACPI_PIN_CONFIG_DRIVE_STRENGTH: c_int = 10;
pub const ACPI_PIN_CONFIG_SLEW_RATE: c_int = 11;
pub const ACPI_PIN_CONFIG_INPUT_DEBOUNCE: c_int = 12;
pub const ACPI_PIN_CONFIG_INPUT_SCHMITT_TRIGGER: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_pin_group {
    pub revision_id: u8,
    pub /: *mut *mut u8 producer_consumer; / For values, see Producer/Consumer above,
    pub pin_table_length: u16,
    pub vendor_length: u16,
    pub pin_table: *mut u16,
    pub resource_label: acpi_resource_label,
    pub vendor_data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_pin_group_function {
    pub revision_id: u8,
    pub /: *mut *mut u8 producer_consumer; / For values, see Producer/Consumer above,
    pub /: *mut *mut u8 shareable; / For values, see Interrupt Attributes above,
    pub function_number: u16,
    pub vendor_length: u16,
    pub resource_source: acpi_resource_source,
    pub resource_source_label: acpi_resource_label,
    pub vendor_data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource_pin_group_config {
    pub revision_id: u8,
    pub /: *mut *mut u8 producer_consumer; / For values, see Producer/Consumer above,
    pub /: *mut *mut u8 shareable; / For values, see Interrupt Attributes above,
    pub /: *mut *mut u8 pin_config_type; / For values, see pin_config_type above,
    pub pin_config_value: u32,
    pub vendor_length: u16,
    pub resource_source: acpi_resource_source,
    pub resource_source_label: acpi_resource_label,
    pub vendor_data: *mut u8,
}

// ACPI_RESOURCE_TYPEs
pub const ACPI_RESOURCE_TYPE_IRQ: c_int = 0;
pub const ACPI_RESOURCE_TYPE_DMA: c_int = 1;
pub const ACPI_RESOURCE_TYPE_START_DEPENDENT: c_int = 2;
pub const ACPI_RESOURCE_TYPE_END_DEPENDENT: c_int = 3;
pub const ACPI_RESOURCE_TYPE_IO: c_int = 4;
pub const ACPI_RESOURCE_TYPE_FIXED_IO: c_int = 5;
pub const ACPI_RESOURCE_TYPE_VENDOR: c_int = 6;
pub const ACPI_RESOURCE_TYPE_END_TAG: c_int = 7;
pub const ACPI_RESOURCE_TYPE_MEMORY24: c_int = 8;
pub const ACPI_RESOURCE_TYPE_MEMORY32: c_int = 9;
pub const ACPI_RESOURCE_TYPE_FIXED_MEMORY32: c_int = 10;
pub const ACPI_RESOURCE_TYPE_ADDRESS16: c_int = 11;
pub const ACPI_RESOURCE_TYPE_ADDRESS32: c_int = 12;
pub const ACPI_RESOURCE_TYPE_ADDRESS64: c_int = 13;

pub const ACPI_RESOURCE_TYPE_EXTENDED_IRQ: c_int = 15;
pub const ACPI_RESOURCE_TYPE_GENERIC_REGISTER: c_int = 16;

pub const ACPI_RESOURCE_TYPE_MAX: c_int = 25;
// Master union for resource descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub union acpi_resource_data {
    pub irq: acpi_resource_irq,
    pub dma: acpi_resource_dma,
    pub start_dpf: acpi_resource_start_dependent,
    pub io: acpi_resource_io,
    pub fixed_io: acpi_resource_fixed_io,
    pub fixed_dma: acpi_resource_fixed_dma,
    pub vendor: acpi_resource_vendor,
    pub vendor_typed: acpi_resource_vendor_typed,
    pub end_tag: acpi_resource_end_tag,
    pub memory24: acpi_resource_memory24,
    pub memory32: acpi_resource_memory32,
    pub fixed_memory32: acpi_resource_fixed_memory32,
    pub address16: acpi_resource_address16,
    pub address32: acpi_resource_address32,
    pub address64: acpi_resource_address64,
    pub ext_address64: acpi_resource_extended_address64,
    pub extended_irq: acpi_resource_extended_irq,
    pub generic_reg: acpi_resource_generic_register,
    pub gpio: acpi_resource_gpio,
    pub i2c_serial_bus: acpi_resource_i2c_serialbus,
    pub spi_serial_bus: acpi_resource_spi_serialbus,
    pub uart_serial_bus: acpi_resource_uart_serialbus,
    pub csi2_serial_bus: acpi_resource_csi2_serialbus,
    pub common_serial_bus: acpi_resource_common_serialbus,
    pub pin_function: acpi_resource_pin_function,
    pub pin_config: acpi_resource_pin_config,
    pub pin_group: acpi_resource_pin_group,
    pub pin_group_function: acpi_resource_pin_group_function,
    pub pin_group_config: acpi_resource_pin_group_config,
    pub clock_input: acpi_resource_clock_input,
// Common fields
    pub /: *mut *mut acpi_resource_address address; / Common 16/32/64 address fields,
}

// Common resource header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_resource {
    pub type: u32,
    pub length: u32,
    pub data: acpi_resource_data,
}

// restore default alignment

// Macro for walking resource templates with multiple descriptors

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_pci_routing_table {
    pub length: u32,
    pub pin: u32,
    pub /: *mut *mut u64 address; / here for 64-bit alignment,
    pub source_index: u32,
    pub /: *mut *mut char pad[4]; / pad to 64 bits so sizeof() works in all cases,
    pub source): ACPI_FLEX_ARRAY(char,,
}
