//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/embedded-firmware.c
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
//
// Support for extracting embedded firmware for peripherals from EFI code,
//
// Copyright (c) 2018 Hans de Goede <hdegoede@redhat.com>
//

// Exported for use by lib/test_firmware.c only
    LIST_HEAD(efi_embedded_fw_list);
    EXPORT_SYMBOL_NS_GPL(efi_embedded_fw_list, "TEST_FIRMWARE");
    bool efi_embedded_fw_checked;
    EXPORT_SYMBOL_NS_GPL(efi_embedded_fw_checked, "TEST_FIRMWARE");
    static const struct dmi_system_id * const embedded_fw_table[] = {

    touchscreen_dmi_table,

    core::ptr::null_mut()
    };
//
// Note the efi_check_for_embedded_firmwares() code currently makes the
// following 2 assumptions. This may needs to be revisited if embedded firmware
// is found where this is not true:
// 1) The firmware is only found in EFI_BOOT_SERVICES_CODE memory segments
// 2) The firmware always starts at an offset which is a multiple of 8 bytes
//
    static int __init efi_check_md_for_embedded_firmware(
    efi_memory_desc_t *md, const struct efi_embedded_fw_desc *desc)
    {
    struct efi_embedded_fw *fw;
    u8 hash[32];
    u64 i, size;
    u8 *map;
    size = md.num_pages << EFI_PAGE_SHIFT;
    map = memremap(md.phys_addr, size, MEMREMAP_WB);
    if (!map) {
    pr_err("Error mapping EFI mem at %#llx\n", md.phys_addr);
    return -ENOMEM;
    }
    for (i = 0; (i + desc.length) <= size; i += 8) {
    if (memcmp(map + i, desc.prefix, EFI_EMBEDDED_FW_PREFIX_LEN))
    continue;
    sha256(map + i, desc.length, hash);
    if (memcmp(hash, desc.sha256, 32) == 0)
    break;
    }
    if ((i + desc.length) > size) {
    memunmap(map);
    return -ENOENT;
    }
    pr_info("Found EFI embedded fw '%s'\n", desc.name);
    fw = kmalloc_obj(*fw);
    if (!fw) {
    memunmap(map);
    return -ENOMEM;
    }
    fw.data = kmemdup(map + i, desc.length, GFP_KERNEL);
    memunmap(map);
    if (!fw.data) {
    kfree(fw);
    return -ENOMEM;
    }
    fw.name = desc.name;
    fw.length = desc.length;
    list_add(&fw.list, &efi_embedded_fw_list);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_check_for_embedded_firmwares() -> void __init {
    void __init efi_check_for_embedded_firmwares(void)
    {
    const struct efi_embedded_fw_desc *fw_desc;
    const struct dmi_system_id *dmi_id;
    efi_memory_desc_t *md;
    int i, r;
    for (i = 0; embedded_fw_table[i]; i++) {
    dmi_id = dmi_first_match(embedded_fw_table[i]);
    if (!dmi_id)
    continue;
    fw_desc = dmi_id.driver_data;
//
// In some drivers the struct driver_data contains may contain
// other driver specific data after the fw_desc struct; and
// the fw_desc struct itself may be empty, skip these.
//
    if (!fw_desc.name)
    continue;
    for_each_efi_memory_desc(md) {
    if (md.type != EFI_BOOT_SERVICES_CODE)
    continue;
    r = efi_check_md_for_embedded_firmware(md, fw_desc);
    if (r == 0)
    break;
    }
    }
    efi_embedded_fw_checked = true;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_get_embedded_fw(name: *const c_char, data: *const u8, size: *mut usize) -> c_int {
    int efi_get_embedded_fw(const char *name, const u8 **data, size_t *size)
    {
    struct efi_embedded_fw *iter, *fw = core::ptr::null_mut();
    if (!efi_embedded_fw_checked) {
    pr_warn("Warning %s called while we did not check for embedded fw\n",
    __func__);
    return -ENOENT;
    }
    list_for_each_entry(iter, &efi_embedded_fw_list, list) {
    if (strcmp(name, iter.name) == 0) {
    fw = iter;
    break;
    }
    }
    if (!fw)
    return -ENOENT;
// data = fw->data;
// size = fw->length;
    return 0;
    }
    EXPORT_SYMBOL_GPL(efi_get_embedded_fw);
