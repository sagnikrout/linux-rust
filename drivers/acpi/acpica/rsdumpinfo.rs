//! Automatically rewritten from C to Rust
//! Source: drivers/acpi/acpica/rsdumpinfo.c
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
// Module Name: rsdumpinfo - Tables used to display resource descriptors.
//

    ACPI_MODULE_NAME("rsdumpinfo")

//
// Resource Descriptor info tables
//
// Note: The first table entry must be a Title or Literal and must contain
// the table length (number of table entries)
//
    struct acpi_rsdump_info acpi_rs_dump_irq[7] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_irq), "IRQ", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(irq.descriptor_length),
    "Descriptor Length", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(irq.triggering), "Triggering",
    acpi_gbl_he_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(irq.polarity), "Polarity",
    acpi_gbl_ll_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(irq.shareable), "Sharing",
    acpi_gbl_shr_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(irq.interrupt_count),
    "Interrupt Count", core::ptr::null_mut()},
    {ACPI_RSD_SHORTLIST, ACPI_RSD_OFFSET(irq.interrupts[0]),
    "Interrupt List", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_dma[6] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_dma), "DMA", core::ptr::null_mut()},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(dma.type), "Speed",
    acpi_gbl_typ_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(dma.bus_master), "Mastering",
    acpi_gbl_bm_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(dma.transfer), "Transfer Type",
    acpi_gbl_siz_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(dma.channel_count), "Channel Count",
    core::ptr::null_mut()},
    {ACPI_RSD_SHORTLIST, ACPI_RSD_OFFSET(dma.channels[0]), "Channel List",
    core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_start_dpf[4] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_start_dpf),
    "Start-Dependent-Functions", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(start_dpf.descriptor_length),
    "Descriptor Length", core::ptr::null_mut()},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(start_dpf.compatibility_priority),
    "Compatibility Priority", acpi_gbl_config_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(start_dpf.performance_robustness),
    "Performance/Robustness", acpi_gbl_config_decode}
    };
    struct acpi_rsdump_info acpi_rs_dump_end_dpf[1] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_end_dpf),
    "End-Dependent-Functions", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_io[6] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_io), "I/O", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(io.io_decode), "Address Decoding",
    acpi_gbl_io_decode},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(io.minimum), "Address Minimum", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(io.maximum), "Address Maximum", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(io.alignment), "Alignment", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(io.address_length), "Address Length",
    core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_fixed_io[3] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_fixed_io),
    "Fixed I/O", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(fixed_io.address), "Address", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(fixed_io.address_length),
    "Address Length", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_vendor[3] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_vendor),
    "Vendor Specific", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(vendor.byte_length), "Length", core::ptr::null_mut()},
    {ACPI_RSD_LONGLIST, ACPI_RSD_OFFSET(vendor.byte_data[0]), "Vendor Data",
    core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_end_tag[1] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_end_tag), "EndTag",
    core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_memory24[6] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_memory24),
    "24-Bit Memory Range", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(memory24.write_protect),
    "Write Protect", acpi_gbl_rw_decode},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(memory24.minimum), "Address Minimum",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(memory24.maximum), "Address Maximum",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(memory24.alignment), "Alignment",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(memory24.address_length),
    "Address Length", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_memory32[6] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_memory32),
    "32-Bit Memory Range", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(memory32.write_protect),
    "Write Protect", acpi_gbl_rw_decode},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(memory32.minimum), "Address Minimum",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(memory32.maximum), "Address Maximum",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(memory32.alignment), "Alignment",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(memory32.address_length),
    "Address Length", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_fixed_memory32[4] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_fixed_memory32),
    "32-Bit Fixed Memory Range", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(fixed_memory32.write_protect),
    "Write Protect", acpi_gbl_rw_decode},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(fixed_memory32.address), "Address",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(fixed_memory32.address_length),
    "Address Length", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_address16[8] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_address16),
    "16-Bit WORD Address Space", core::ptr::null_mut()},
    {ACPI_RSD_ADDRESS, 0, core::ptr::null_mut(), core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(address16.address.granularity),
    "Granularity", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(address16.address.minimum),
    "Address Minimum", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(address16.address.maximum),
    "Address Maximum", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(address16.address.translation_offset),
    "Translation Offset", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(address16.address.address_length),
    "Address Length", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(address16.resource_source), core::ptr::null_mut(), core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_address32[8] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_address32),
    "32-Bit DWORD Address Space", core::ptr::null_mut()},
    {ACPI_RSD_ADDRESS, 0, core::ptr::null_mut(), core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(address32.address.granularity),
    "Granularity", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(address32.address.minimum),
    "Address Minimum", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(address32.address.maximum),
    "Address Maximum", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(address32.address.translation_offset),
    "Translation Offset", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(address32.address.address_length),
    "Address Length", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(address32.resource_source), core::ptr::null_mut(), core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_address64[8] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_address64),
    "64-Bit QWORD Address Space", core::ptr::null_mut()},
    {ACPI_RSD_ADDRESS, 0, core::ptr::null_mut(), core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(address64.address.granularity),
    "Granularity", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(address64.address.minimum),
    "Address Minimum", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(address64.address.maximum),
    "Address Maximum", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(address64.address.translation_offset),
    "Translation Offset", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(address64.address.address_length),
    "Address Length", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(address64.resource_source), core::ptr::null_mut(), core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_ext_address64[8] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_ext_address64),
    "64-Bit Extended Address Space", core::ptr::null_mut()},
    {ACPI_RSD_ADDRESS, 0, core::ptr::null_mut(), core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(ext_address64.address.granularity),
    "Granularity", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(ext_address64.address.minimum),
    "Address Minimum", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(ext_address64.address.maximum),
    "Address Maximum", core::ptr::null_mut()},
    {ACPI_RSD_UINT64,
    ACPI_RSD_OFFSET(ext_address64.address.translation_offset),
    "Translation Offset", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(ext_address64.address.address_length),
    "Address Length", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(ext_address64.type_specific),
    "Type-Specific Attribute", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_ext_irq[8] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_ext_irq),
    "Extended IRQ", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(extended_irq.producer_consumer),
    "Type", acpi_gbl_consume_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(extended_irq.triggering),
    "Triggering", acpi_gbl_he_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(extended_irq.polarity), "Polarity",
    acpi_gbl_ll_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(extended_irq.shareable), "Sharing",
    acpi_gbl_shr_decode},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(extended_irq.resource_source), core::ptr::null_mut(),
    core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(extended_irq.interrupt_count),
    "Interrupt Count", core::ptr::null_mut()},
    {ACPI_RSD_DWORDLIST, ACPI_RSD_OFFSET(extended_irq.interrupts[0]),
    "Interrupt List", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_generic_reg[6] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_generic_reg),
    "Generic Register", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(generic_reg.space_id), "Space ID",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(generic_reg.bit_width), "Bit Width",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(generic_reg.bit_offset), "Bit Offset",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(generic_reg.access_size),
    "Access Size", core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_RSD_OFFSET(generic_reg.address), "Address", core::ptr::null_mut()}
    };
    struct acpi_rsdump_info acpi_rs_dump_gpio[16] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_gpio), "GPIO", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(gpio.revision_id), "RevisionId", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(gpio.connection_type),
    "ConnectionType", acpi_gbl_ct_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(gpio.producer_consumer),
    "ProducerConsumer", acpi_gbl_consume_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(gpio.pin_config), "PinConfig",
    acpi_gbl_ppc_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(gpio.shareable), "Sharing",
    acpi_gbl_shr_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(gpio.io_restriction),
    "IoRestriction", acpi_gbl_ior_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(gpio.triggering), "Triggering",
    acpi_gbl_he_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(gpio.polarity), "Polarity",
    acpi_gbl_ll_decode},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(gpio.drive_strength), "DriveStrength",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(gpio.debounce_timeout),
    "DebounceTimeout", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(gpio.resource_source),
    "ResourceSource", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(gpio.pin_table_length),
    "PinTableLength", core::ptr::null_mut()},
    {ACPI_RSD_WORDLIST, ACPI_RSD_OFFSET(gpio.pin_table), "PinTable", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(gpio.vendor_length), "VendorLength",
    core::ptr::null_mut()},
    {ACPI_RSD_SHORTLISTX, ACPI_RSD_OFFSET(gpio.vendor_data), "VendorData",
    core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_pin_function[10] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_pin_function),
    "PinFunction", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_function.revision_id),
    "RevisionId", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_function.pin_config), "PinConfig",
    acpi_gbl_ppc_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(pin_function.shareable), "Sharing",
    acpi_gbl_shr_decode},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_function.function_number),
    "FunctionNumber", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(pin_function.resource_source),
    "ResourceSource", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_function.pin_table_length),
    "PinTableLength", core::ptr::null_mut()},
    {ACPI_RSD_WORDLIST, ACPI_RSD_OFFSET(pin_function.pin_table), "PinTable",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_function.vendor_length),
    "VendorLength", core::ptr::null_mut()},
    {ACPI_RSD_SHORTLISTX, ACPI_RSD_OFFSET(pin_function.vendor_data),
    "VendorData", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_clock_input[7] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_clock_input),
    "ClockInput", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(clock_input.revision_id), "RevisionId",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(clock_input.frequency_numerator),
    "FrequencyNumerator", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(clock_input.frequency_divisor),
    "FrequencyDivisor", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(clock_input.scale), "Scale",
    acpi_gbl_clock_input_scale},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(clock_input.mode), "Mode",
    acpi_gbl_clock_input_mode},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(clock_input.resource_source),
    "ResourceSource", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_pin_config[11] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_pin_config),
    "PinConfig", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_config.revision_id), "RevisionId",
    core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(pin_config.producer_consumer),
    "ProducerConsumer", acpi_gbl_consume_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(pin_config.shareable), "Sharing",
    acpi_gbl_shr_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_config.pin_config_type),
    "PinConfigType", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(pin_config.pin_config_value),
    "PinConfigValue", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(pin_config.resource_source),
    "ResourceSource", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_config.pin_table_length),
    "PinTableLength", core::ptr::null_mut()},
    {ACPI_RSD_WORDLIST, ACPI_RSD_OFFSET(pin_config.pin_table), "PinTable",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_config.vendor_length),
    "VendorLength", core::ptr::null_mut()},
    {ACPI_RSD_SHORTLISTX, ACPI_RSD_OFFSET(pin_config.vendor_data),
    "VendorData", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_pin_group[8] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_pin_group),
    "PinGroup", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_group.revision_id), "RevisionId",
    core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(pin_group.producer_consumer),
    "ProducerConsumer", acpi_gbl_consume_decode},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_group.pin_table_length),
    "PinTableLength", core::ptr::null_mut()},
    {ACPI_RSD_WORDLIST, ACPI_RSD_OFFSET(pin_group.pin_table), "PinTable",
    core::ptr::null_mut()},
    {ACPI_RSD_LABEL, ACPI_RSD_OFFSET(pin_group.resource_label),
    "ResourceLabel", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_group.vendor_length),
    "VendorLength", core::ptr::null_mut()},
    {ACPI_RSD_SHORTLISTX, ACPI_RSD_OFFSET(pin_group.vendor_data),
    "VendorData", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_pin_group_function[9] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_pin_group_function),
    "PinGroupFunction", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_group_function.revision_id),
    "RevisionId", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG,
    ACPI_RSD_OFFSET(pin_group_function.producer_consumer),
    "ProducerConsumer", acpi_gbl_consume_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(pin_group_function.shareable),
    "Sharing", acpi_gbl_shr_decode},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_group_function.function_number),
    "FunctionNumber", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE_LABEL,
    ACPI_RSD_OFFSET(pin_group_function.resource_source_label),
    "ResourceSourceLabel", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(pin_group_function.resource_source),
    "ResourceSource", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_group_function.vendor_length),
    "VendorLength", core::ptr::null_mut()},
    {ACPI_RSD_SHORTLISTX, ACPI_RSD_OFFSET(pin_group_function.vendor_data),
    "VendorData", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_pin_group_config[10] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_pin_group_config),
    "PinGroupConfig", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_group_config.revision_id),
    "RevisionId", core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(pin_group_config.producer_consumer),
    "ProducerConsumer", acpi_gbl_consume_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(pin_group_config.shareable),
    "Sharing", acpi_gbl_shr_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(pin_group_config.pin_config_type),
    "PinConfigType", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(pin_group_config.pin_config_value),
    "PinConfigValue", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE_LABEL,
    ACPI_RSD_OFFSET(pin_group_config.resource_source_label),
    "ResourceSourceLabel", core::ptr::null_mut()},
    {ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(pin_group_config.resource_source),
    "ResourceSource", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(pin_group_config.vendor_length),
    "VendorLength", core::ptr::null_mut()},
    {ACPI_RSD_SHORTLISTX, ACPI_RSD_OFFSET(pin_group_config.vendor_data),
    "VendorData", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_fixed_dma[4] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_fixed_dma),
    "FixedDma", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(fixed_dma.request_lines),
    "RequestLines", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(fixed_dma.channels), "Channels",
    core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(fixed_dma.width), "TransferWidth",
    acpi_gbl_dts_decode},
    };

    {ACPI_RSD_UINT8,    ACPI_RSD_OFFSET (common_serial_bus.revision_id),    "RevisionId",               core::ptr::null_mut()}, \
    {ACPI_RSD_UINT8,    ACPI_RSD_OFFSET (common_serial_bus.type),           "Type",                     acpi_gbl_sbt_decode}, \
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET (common_serial_bus.producer_consumer), "ProducerConsumer",      acpi_gbl_consume_decode}, \
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET (common_serial_bus.slave_mode),     "SlaveMode",                acpi_gbl_sm_decode}, \
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET (common_serial_bus.connection_sharing),"ConnectionSharing",     acpi_gbl_shr_decode}, \
    {ACPI_RSD_UINT8,    ACPI_RSD_OFFSET (common_serial_bus.type_revision_id), "TypeRevisionId",         core::ptr::null_mut()}, \
    {ACPI_RSD_UINT16,   ACPI_RSD_OFFSET (common_serial_bus.type_data_length), "TypeDataLength",         core::ptr::null_mut()}, \
    {ACPI_RSD_SOURCE,   ACPI_RSD_OFFSET (common_serial_bus.resource_source), "ResourceSource",          core::ptr::null_mut()}, \
    {ACPI_RSD_UINT16,   ACPI_RSD_OFFSET (common_serial_bus.vendor_length),  "VendorLength",             core::ptr::null_mut()}, \
    {ACPI_RSD_SHORTLISTX,ACPI_RSD_OFFSET (common_serial_bus.vendor_data),   "VendorData",               core::ptr::null_mut()},
    struct acpi_rsdump_info acpi_rs_dump_common_serial_bus[11] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_common_serial_bus),
    "Common Serial Bus", core::ptr::null_mut()},
    ACPI_RS_DUMP_COMMON_SERIAL_BUS
    };
    struct acpi_rsdump_info acpi_rs_dump_csi2_serial_bus[11] = {
    { ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_csi2_serial_bus),
    "Camera Serial Bus", core::ptr::null_mut() },
    { ACPI_RSD_UINT8, ACPI_RSD_OFFSET(csi2_serial_bus.revision_id),
    "RevisionId", core::ptr::null_mut() },
    { ACPI_RSD_UINT8, ACPI_RSD_OFFSET(csi2_serial_bus.type), "Type",
    acpi_gbl_sbt_decode },
    { ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(csi2_serial_bus.producer_consumer),
    "ProducerConsumer", acpi_gbl_consume_decode },
    { ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(csi2_serial_bus.slave_mode),
    "SlaveMode", acpi_gbl_sm_decode },
    { ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(csi2_serial_bus.phy_type),
    "PhyType", acpi_gbl_phy_decode },
    { ACPI_RSD_6BITFLAG,
    ACPI_RSD_OFFSET(csi2_serial_bus.local_port_instance),
    "LocalPortInstance", core::ptr::null_mut() },
    { ACPI_RSD_UINT8, ACPI_RSD_OFFSET(csi2_serial_bus.type_revision_id),
    "TypeRevisionId", core::ptr::null_mut() },
    { ACPI_RSD_UINT16, ACPI_RSD_OFFSET(csi2_serial_bus.vendor_length),
    "VendorLength", core::ptr::null_mut() },
    { ACPI_RSD_SHORTLISTX, ACPI_RSD_OFFSET(csi2_serial_bus.vendor_data),
    "VendorData", core::ptr::null_mut() },
    { ACPI_RSD_SOURCE, ACPI_RSD_OFFSET(csi2_serial_bus.resource_source),
    "ResourceSource", core::ptr::null_mut() },
    };
    struct acpi_rsdump_info acpi_rs_dump_i2c_serial_bus[14] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_i2c_serial_bus),
    "I2C Serial Bus", core::ptr::null_mut()},
    ACPI_RS_DUMP_COMMON_SERIAL_BUS {ACPI_RSD_1BITFLAG,
    ACPI_RSD_OFFSET(i2c_serial_bus.
    access_mode),
    "AccessMode", acpi_gbl_am_decode},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(i2c_serial_bus.connection_speed),
    "ConnectionSpeed", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(i2c_serial_bus.slave_address),
    "SlaveAddress", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_spi_serial_bus[18] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_spi_serial_bus),
    "Spi Serial Bus", core::ptr::null_mut()},
    ACPI_RS_DUMP_COMMON_SERIAL_BUS {ACPI_RSD_1BITFLAG,
    ACPI_RSD_OFFSET(spi_serial_bus.
    wire_mode), "WireMode",
    acpi_gbl_wm_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(spi_serial_bus.device_polarity),
    "DevicePolarity", acpi_gbl_dp_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(spi_serial_bus.data_bit_length),
    "DataBitLength", core::ptr::null_mut()},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(spi_serial_bus.clock_phase),
    "ClockPhase", acpi_gbl_cph_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(spi_serial_bus.clock_polarity),
    "ClockPolarity", acpi_gbl_cpo_decode},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(spi_serial_bus.device_selection),
    "DeviceSelection", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(spi_serial_bus.connection_speed),
    "ConnectionSpeed", core::ptr::null_mut()},
    };
    struct acpi_rsdump_info acpi_rs_dump_uart_serial_bus[20] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_uart_serial_bus),
    "Uart Serial Bus", core::ptr::null_mut()},
    ACPI_RS_DUMP_COMMON_SERIAL_BUS {ACPI_RSD_2BITFLAG,
    ACPI_RSD_OFFSET(uart_serial_bus.
    flow_control),
    "FlowControl", acpi_gbl_fc_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(uart_serial_bus.stop_bits),
    "StopBits", acpi_gbl_sb_decode},
    {ACPI_RSD_3BITFLAG, ACPI_RSD_OFFSET(uart_serial_bus.data_bits),
    "DataBits", acpi_gbl_bpb_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(uart_serial_bus.endian), "Endian",
    acpi_gbl_ed_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(uart_serial_bus.parity), "Parity",
    acpi_gbl_pt_decode},
    {ACPI_RSD_UINT8, ACPI_RSD_OFFSET(uart_serial_bus.lines_enabled),
    "LinesEnabled", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(uart_serial_bus.rx_fifo_size),
    "RxFifoSize", core::ptr::null_mut()},
    {ACPI_RSD_UINT16, ACPI_RSD_OFFSET(uart_serial_bus.tx_fifo_size),
    "TxFifoSize", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_RSD_OFFSET(uart_serial_bus.default_baud_rate),
    "ConnectionSpeed", core::ptr::null_mut()},
    };
//
// Tables used for common address descriptor flag fields
//
    struct acpi_rsdump_info acpi_rs_dump_general_flags[5] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_general_flags), core::ptr::null_mut(),
    core::ptr::null_mut()},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.producer_consumer),
    "Consumer/Producer", acpi_gbl_consume_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.decode), "Address Decode",
    acpi_gbl_dec_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.min_address_fixed),
    "Min Relocatability", acpi_gbl_min_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.max_address_fixed),
    "Max Relocatability", acpi_gbl_max_decode}
    };
    struct acpi_rsdump_info acpi_rs_dump_memory_flags[5] = {
    {ACPI_RSD_LITERAL, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_memory_flags),
    "Resource Type", (void *)"Memory Range"},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.info.mem.write_protect),
    "Write Protect", acpi_gbl_rw_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(address.info.mem.caching),
    "Caching", acpi_gbl_mem_decode},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(address.info.mem.range_type),
    "Range Type", acpi_gbl_mtp_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.info.mem.translation),
    "Translation", acpi_gbl_ttp_decode}
    };
    struct acpi_rsdump_info acpi_rs_dump_io_flags[4] = {
    {ACPI_RSD_LITERAL, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_io_flags),
    "Resource Type", (void *)"I/O Range"},
    {ACPI_RSD_2BITFLAG, ACPI_RSD_OFFSET(address.info.io.range_type),
    "Range Type", acpi_gbl_rng_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.info.io.translation),
    "Translation", acpi_gbl_ttp_decode},
    {ACPI_RSD_1BITFLAG, ACPI_RSD_OFFSET(address.info.io.translation_type),
    "Translation Type", acpi_gbl_trs_decode}
    };
//
// Table used to dump _PRT contents
//
    struct acpi_rsdump_info acpi_rs_dump_prt[5] = {
    {ACPI_RSD_TITLE, ACPI_RSD_TABLE_SIZE(acpi_rs_dump_prt), core::ptr::null_mut(), core::ptr::null_mut()},
    {ACPI_RSD_UINT64, ACPI_PRT_OFFSET(address), "Address", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_PRT_OFFSET(pin), "Pin", core::ptr::null_mut()},
    {ACPI_RSD_STRING, ACPI_PRT_OFFSET(source[0]), "Source", core::ptr::null_mut()},
    {ACPI_RSD_UINT32, ACPI_PRT_OFFSET(source_index), "Source Index", core::ptr::null_mut()}
    };
