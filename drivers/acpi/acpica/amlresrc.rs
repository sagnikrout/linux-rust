//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/acpi/acpica/amlresrc.h
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
// Module Name: amlresrc.h - AML resource descriptors
//
// Copyright (C) 2000 - 2026, Intel Corp.
//
// acpisrc:struct_defs -- for acpisrc conversion
//
// Resource descriptor tags, as defined in the ACPI specification.
// Used to symbolically reference fields within a descriptor.
//

// Default sizes for "small" resource descriptors
pub const ASL_RDESC_IRQ_SIZE: c_uint = 0x02;
pub const ASL_RDESC_DMA_SIZE: c_uint = 0x02;
pub const ASL_RDESC_ST_DEPEND_SIZE: c_uint = 0x00;
pub const ASL_RDESC_END_DEPEND_SIZE: c_uint = 0x00;
pub const ASL_RDESC_IO_SIZE: c_uint = 0x07;
pub const ASL_RDESC_FIXED_IO_SIZE: c_uint = 0x03;
pub const ASL_RDESC_FIXED_DMA_SIZE: c_uint = 0x05;
pub const ASL_RDESC_END_TAG_SIZE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct asl_resource_node {
    pub buffer_length: u32,
    pub buffer: *mut c_void,
    pub next: *mut asl_resource_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct asl_resource_info {
    pub /: *mut *mut *mut acpi_parse_object descriptor_type_op; / Resource descriptor parse node,
    pub /: *mut *mut *mut acpi_parse_object mapping_op; / Used for mapfile support,
    pub /: *mut *mut u32 current_byte_offset; / Offset in resource template,
}

// Macros used to generate AML resource length fields

//
// Resource descriptors defined in the ACPI specification.
//
// Packing/alignment must be BYTE because these descriptors
// are used to overlay the raw AML byte stream.
//

//
// SMALL descriptors
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_small_header {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_irq {
    pub irq_mask: AML_RESOURCE_SMALL_HEADER_COMMON u16,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_irq_noflags {
    pub irq_mask: AML_RESOURCE_SMALL_HEADER_COMMON u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_dma {
    pub dma_channel_mask: AML_RESOURCE_SMALL_HEADER_COMMON u8,
    pub flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_start_dependent {
    pub flags: AML_RESOURCE_SMALL_HEADER_COMMON u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_start_dependent_noprio {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_end_dependent {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_io {
    pub flags: AML_RESOURCE_SMALL_HEADER_COMMON u8,
    pub minimum: u16,
    pub maximum: u16,
    pub alignment: u8,
    pub address_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_fixed_io {
    pub address: AML_RESOURCE_SMALL_HEADER_COMMON u16,
    pub address_length: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_vendor_small {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_end_tag {
    pub checksum: AML_RESOURCE_SMALL_HEADER_COMMON u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_fixed_dma {
    pub request_lines: AML_RESOURCE_SMALL_HEADER_COMMON u16,
    pub channels: u16,
    pub width: u8,
}

//
// LARGE descriptors
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_large_header {
// General Flags for address space resource descriptors
pub const ACPI_RESOURCE_FLAG_DEC: c_int = 2;
pub const ACPI_RESOURCE_FLAG_MIF: c_int = 4;
pub const ACPI_RESOURCE_FLAG_MAF: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_memory24 {
    pub flags: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub minimum: u16,
    pub maximum: u16,
    pub alignment: u16,
    pub address_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_vendor_large {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_memory32 {
    pub flags: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub minimum: u32,
    pub maximum: u32,
    pub alignment: u32,
    pub address_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_fixed_memory32 {
    pub flags: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub address: u32,
    pub address_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_address {
    pub AML_RESOURCE_ADDRESS_COMMON}: AML_RESOURCE_LARGE_HEADER_COMMON,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_extended_address64 {
    pub revision_ID: AML_RESOURCE_ADDRESS_COMMON u8,
    pub reserved: u8,
    pub granularity: u64,
    pub minimum: u64,
    pub maximum: u64,
    pub translation_offset: u64,
    pub address_length: u64,
    pub type_specific: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_address64 {
    pub granularity: AML_RESOURCE_ADDRESS_COMMON u64,
    pub minimum: u64,
    pub maximum: u64,
    pub translation_offset: u64,
    pub address_length: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_address32 {
    pub granularity: AML_RESOURCE_ADDRESS_COMMON u32,
    pub minimum: u32,
    pub maximum: u32,
    pub translation_offset: u32,
    pub address_length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_address16 {
    pub granularity: AML_RESOURCE_ADDRESS_COMMON u16,
    pub minimum: u16,
    pub maximum: u16,
    pub translation_offset: u16,
    pub address_length: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_extended_irq {
    pub flags: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub interrupt_count: u8,
    pub interrupt: u32,
    pub interrupts): ACPI_FLEX_ARRAY(u32,,
}

// res_source_index, res_source optional fields follow
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_generic_register {
    pub address_space_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub /: *mut *mut u8 access_size; / ACPI 3.0, was previously Reserved,
    pub address: u64,
}

// Common descriptor for gpio_int and gpio_io (ACPI 5.0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_gpio {
    pub revision_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub connection_type: u8,
    pub flags: u16,
    pub int_flags: u16,
    pub pin_config: u8,
    pub drive_strength: u16,
    pub debounce_timeout: u16,
    pub pin_table_offset: u16,
    pub res_source_index: u8,
    pub res_source_offset: u16,
    pub vendor_offset: u16,
    pub vendor_length: u16,
//
// Optional fields follow immediately:
// 1) PIN list (Words)
// 2) Resource Source String
// 3) Vendor Data bytes
//
}

// Values for connection_type above
pub const AML_RESOURCE_GPIO_TYPE_INT: c_int = 0;
pub const AML_RESOURCE_GPIO_TYPE_IO: c_int = 1;
pub const AML_RESOURCE_MAX_GPIOTYPE: c_int = 1;
// Common preamble for all serial descriptors (ACPI 5.0)

// Values for the type field above
pub const AML_RESOURCE_I2C_SERIALBUSTYPE: c_int = 1;
pub const AML_RESOURCE_SPI_SERIALBUSTYPE: c_int = 2;
pub const AML_RESOURCE_UART_SERIALBUSTYPE: c_int = 3;
pub const AML_RESOURCE_CSI2_SERIALBUSTYPE: c_int = 4;
pub const AML_RESOURCE_MAX_SERIALBUSTYPE: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_common_serialbus {
    pub AML_RESOURCE_SERIAL_COMMON}: AML_RESOURCE_LARGE_HEADER_COMMON,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_csi2_serialbus {
//
// Optional fields follow immediately:
// 1) Vendor Data bytes
// 2) Resource Source String
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_i2c_serialbus {
    pub connection_speed: AML_RESOURCE_SERIAL_COMMON u32,
    pub slave_address: u16,
//
// Optional fields follow immediately:
// 1) Vendor Data bytes
// 2) Resource Source String
//
}

pub const AML_RESOURCE_I2C_MIN_DATA_LEN: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_spi_serialbus {
    pub connection_speed: AML_RESOURCE_SERIAL_COMMON u32,
    pub data_bit_length: u8,
    pub clock_phase: u8,
    pub clock_polarity: u8,
    pub device_selection: u16,
//
// Optional fields follow immediately:
// 1) Vendor Data bytes
// 2) Resource Source String
//
}

pub const AML_RESOURCE_SPI_MIN_DATA_LEN: c_int = 9;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_uart_serialbus {
    pub default_baud_rate: AML_RESOURCE_SERIAL_COMMON u32,
    pub rx_fifo_size: u16,
    pub tx_fifo_size: u16,
    pub parity: u8,
    pub lines_enabled: u8,
//
// Optional fields follow immediately:
// 1) Vendor Data bytes
// 2) Resource Source String
//
}

pub const AML_RESOURCE_UART_MIN_DATA_LEN: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_pin_function {
    pub revision_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub flags: u16,
    pub pin_config: u8,
    pub function_number: u16,
    pub pin_table_offset: u16,
    pub res_source_index: u8,
    pub res_source_offset: u16,
    pub vendor_offset: u16,
    pub vendor_length: u16,
//
// Optional fields follow immediately:
// 1) PIN list (Words)
// 2) Resource Source String
// 3) Vendor Data bytes
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_pin_config {
    pub revision_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub flags: u16,
    pub pin_config_type: u8,
    pub pin_config_value: u32,
    pub pin_table_offset: u16,
    pub res_source_index: u8,
    pub res_source_offset: u16,
    pub vendor_offset: u16,
    pub vendor_length: u16,
//
// Optional fields follow immediately:
// 1) PIN list (Words)
// 2) Resource Source String
// 3) Vendor Data bytes
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_clock_input {
    pub revision_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub flags: u16,
    pub frequency_divisor: u16,
    pub frequency_numerator: u32,
//
// Optional fields follow immediately:
// 1) Resource Source index
// 2) Resource Source String
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_pin_group {
    pub revision_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub flags: u16,
    pub pin_table_offset: u16,
    pub label_offset: u16,
    pub vendor_offset: u16,
    pub vendor_length: u16,
//
// Optional fields follow immediately:
// 1) PIN list (Words)
// 2) Resource Label String
// 3) Vendor Data bytes
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_pin_group_function {
    pub revision_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub flags: u16,
    pub function_number: u16,
    pub res_source_index: u8,
    pub res_source_offset: u16,
    pub res_source_label_offset: u16,
    pub vendor_offset: u16,
    pub vendor_length: u16,
//
// Optional fields follow immediately:
// 1) Resource Source String
// 2) Resource Source Label String
// 3) Vendor Data bytes
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aml_resource_pin_group_config {
    pub revision_id: AML_RESOURCE_LARGE_HEADER_COMMON u8,
    pub flags: u16,
    pub pin_config_type: u8,
    pub pin_config_value: u32,
    pub res_source_index: u8,
    pub res_source_offset: u16,
    pub res_source_label_offset: u16,
    pub vendor_offset: u16,
    pub vendor_length: u16,
//
// Optional fields follow immediately:
// 1) Resource Source String
// 2) Resource Source Label String
// 3) Vendor Data bytes
//
}

// Union of all resource descriptors, so we can allocate the worst case
#[repr(C)]
#[derive(Copy, Clone)]
pub union aml_resource {
// Descriptor headers
    pub descriptor_type: u8,
    pub small_header: aml_resource_small_header,
    pub large_header: aml_resource_large_header,
// Small resource descriptors
    pub irq: aml_resource_irq,
    pub dma: aml_resource_dma,
    pub start_dpf: aml_resource_start_dependent,
    pub end_dpf: aml_resource_end_dependent,
    pub io: aml_resource_io,
    pub fixed_io: aml_resource_fixed_io,
    pub fixed_dma: aml_resource_fixed_dma,
    pub vendor_small: aml_resource_vendor_small,
    pub end_tag: aml_resource_end_tag,
// Large resource descriptors
    pub memory24: aml_resource_memory24,
    pub generic_reg: aml_resource_generic_register,
    pub vendor_large: aml_resource_vendor_large,
    pub memory32: aml_resource_memory32,
    pub fixed_memory32: aml_resource_fixed_memory32,
    pub address16: aml_resource_address16,
    pub address32: aml_resource_address32,
    pub address64: aml_resource_address64,
    pub ext_address64: aml_resource_extended_address64,
    pub extended_irq: aml_resource_extended_irq,
    pub gpio: aml_resource_gpio,
    pub i2c_serial_bus: aml_resource_i2c_serialbus,
    pub spi_serial_bus: aml_resource_spi_serialbus,
    pub uart_serial_bus: aml_resource_uart_serialbus,
    pub csi2_serial_bus: aml_resource_csi2_serialbus,
    pub common_serial_bus: aml_resource_common_serialbus,
    pub pin_function: aml_resource_pin_function,
    pub pin_config: aml_resource_pin_config,
    pub pin_group: aml_resource_pin_group,
    pub pin_group_function: aml_resource_pin_group_function,
    pub pin_group_config: aml_resource_pin_group_config,
    pub clock_input: aml_resource_clock_input,
// Utility overlays
    pub address: aml_resource_address,
    pub dword_item: u32,
    pub word_item: u16,
    pub byte_item: u8,
}

// restore default alignment

// Interfaces used by both the disassembler and compiler
