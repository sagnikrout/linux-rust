//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdca/sdca_device.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
// Copyright(c) 2024 Intel Corporation
//
// The MIPI SDCA specification is available for public downloads at
// https://www.mipi.org/mipi-sdca-v1-0-download
//

#[no_mangle]
pub unsafe extern "C" fn sdca_lookup_interface_revision(slave: *mut sdw_slave) {
    void sdca_lookup_interface_revision(struct sdw_slave *slave)
    {
    struct fwnode_handle *fwnode = slave.dev.fwnode;
//
// if this property is not present, then the sdca_interface_revision will
// remain zero, which will be considered as 'not defined' or 'invalid'.
//
    fwnode_property_read_u32(fwnode, "mipi-sdw-sdca-interface-revision",
    &slave.sdca_data.interface_revision);
    }
    EXPORT_SYMBOL_NS(sdca_lookup_interface_revision, "SND_SOC_SDCA");
#[no_mangle]
unsafe extern "C" fn devm_acpi_table_put(ptr: *mut c_void) {
    static void devm_acpi_table_put(void *ptr)
    {
    acpi_put_table((struct acpi_table_header *)ptr);
    }
#[no_mangle]
pub unsafe extern "C" fn sdca_lookup_swft(slave: *mut sdw_slave) {
    void sdca_lookup_swft(struct sdw_slave *slave)
    {
    acpi_status status;
    status = acpi_get_table(ACPI_SIG_SWFT, 0,
    (struct acpi_table_header **)&slave.sdca_data.swft);
    if (ACPI_FAILURE(status))
    dev_info(&slave.dev, "SWFT not available\n");
    else
    devm_add_action_or_reset(&slave.dev, devm_acpi_table_put,
    slave.sdca_data.swft);
    }
    EXPORT_SYMBOL_NS(sdca_lookup_swft, "SND_SOC_SDCA");
#[no_mangle]
unsafe extern "C" fn sdca_device_quirk_rt712_vb(slave: *mut sdw_slave) -> bool {
    static bool sdca_device_quirk_rt712_vb(struct sdw_slave *slave)
    {
    struct sdw_slave_id *id = &slave.id;
    int i;
//
// The RT712_VA relies on the v06r04 draft, and the
// RT712_VB on a more recent v08r01 draft.
//
    if (slave.sdca_data.interface_revision < 0x0801)
    return false;
    if (id.mfg_id != 0x025d)
    return false;
    if (id.part_id != 0x712 &&
    id.part_id != 0x713 &&
    id.part_id != 0x716 &&
    id.part_id != 0x717)
    return false;
    for (i = 0; i < slave.sdca_data.num_functions; i++) {
    if (slave.sdca_data.function[i].type == SDCA_FUNCTION_TYPE_SMART_MIC)
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sdca_device_quirk_skip_func_type_patching(slave: *mut sdw_slave) -> bool {
    static bool sdca_device_quirk_skip_func_type_patching(struct sdw_slave *slave)
    {
    const char *vendor, *sku;
    vendor = dmi_get_system_info(DMI_SYS_VENDOR);
    sku = dmi_get_system_info(DMI_PRODUCT_SKU);
    if (vendor && sku &&
    !strcmp(vendor, "Dell Inc.") &&
    (!strcmp(sku, "0C62") || !strcmp(sku, "0C63") || !strcmp(sku, "0C6B")) &&
    slave.sdca_data.interface_revision == 0x061c &&
    slave.id.mfg_id == 0x01fa && slave.id.part_id == 0x4243)
    return true;
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn sdca_device_quirk_match(slave: *mut sdw_slave, quirk: enum sdca_quirk) -> bool {
    bool sdca_device_quirk_match(struct sdw_slave *slave, enum sdca_quirk quirk)
    {
    switch (quirk) {
    case SDCA_QUIRKS_RT712_VB:
    return sdca_device_quirk_rt712_vb(slave);
    case SDCA_QUIRKS_SKIP_FUNC_TYPE_PATCHING:
    return sdca_device_quirk_skip_func_type_patching(slave);
    default:
    break;
    }
    return false;
    }
    EXPORT_SYMBOL_NS(sdca_device_quirk_match, "SND_SOC_SDCA");
