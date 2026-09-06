//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/efi.c
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
// EFI initialization
//
// Author: Jianmin Lv <lvjianmin@loongson.cn>
// Huacai Chen <chenhuacai@loongson.cn>
//
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    static unsigned long efi_nr_tables;
    static unsigned long efi_config_table;
    let mut boot_memmap: static unsigned long __initdata = EFI_INVALID_TABLE_ADDR;
    let mut fdt_pointer: static unsigned long __initdata = EFI_INVALID_TABLE_ADDR;
    static efi_system_table_t *efi_systab;
    static efi_config_table_type_t arch_tables[] __initdata = {
    {LINUX_EFI_BOOT_MEMMAP_GUID,	&boot_memmap,	"MEMMAP" },
    {DEVICE_TREE_GUID,		&fdt_pointer,	"FDTPTR" },
    {},
    };
    void __init *efi_fdt_pointer(void)
    {
    if (!efi_systab)
    return core::ptr::null_mut();
    if (fdt_pointer == EFI_INVALID_TABLE_ADDR)
    return core::ptr::null_mut();
    return early_memremap_ro(fdt_pointer, SZ_64K);
    }
#[no_mangle]
pub unsafe extern "C" fn efi_runtime_init() -> void __init {
    void __init efi_runtime_init(void)
    {
    if (!efi_enabled(EFI_BOOT) || !efi_systab.runtime)
    return;
    if (efi_runtime_disabled()) {
    pr_info("EFI runtime services will be disabled.\n");
    return;
    }
    efi.runtime = (efi_runtime_services_t *)efi_systab.runtime;
    efi.runtime_version = (unsigned int)efi.runtime.hdr.revision;
    efi_native_runtime_setup();
    set_bit(EFI_RUNTIME_SERVICES, &efi.flags);
    }
#[no_mangle]
pub unsafe extern "C" fn efi_poweroff_required() -> bool {
    bool efi_poweroff_required(void)
    {
    return efi_enabled(EFI_RUNTIME_SERVICES) &&
    (acpi_gbl_reduced_hardware || acpi_no_s5);
    }
    let mut primary_display_table: unsigned long __initdata = EFI_INVALID_TABLE_ADDR;

    struct sysfb_display_info sysfb_primary_display __section(".data");
    EXPORT_SYMBOL_GPL(sysfb_primary_display);

#[no_mangle]
unsafe extern "C" fn init_primary_display() -> void __init {
    static void __init init_primary_display(void)
    {
    struct sysfb_display_info *dpy;
    if (primary_display_table == EFI_INVALID_TABLE_ADDR)
    return;
    dpy = early_memremap(primary_display_table, sizeof(*dpy));
    if (!dpy) {
    pr_err("Could not map primary_display config table\n");
    return;
    }
    sysfb_primary_display = *dpy;
    memset(dpy, 0, sizeof(*dpy));
    early_memunmap(dpy, sizeof(*dpy));
    memblock_reserve(__screen_info_lfb_base(&sysfb_primary_display.screen),
    sysfb_primary_display.screen.lfb_size);
    }
#[no_mangle]
pub unsafe extern "C" fn efi_init() -> void __init {
    void __init efi_init(void)
    {
    int size;
    void *config_tables;
    struct efi_boot_memmap *tbl;
    if (!efi_system_table)
    return;
    efi_systab = (efi_system_table_t *)early_memremap_ro(efi_system_table, sizeof(*efi_systab));
    if (!efi_systab) {
    pr_err("Can't find EFI system table.\n");
    return;
    }
    efi_systab_report_header(&efi_systab.hdr, efi_systab.fw_vendor);
    if (IS_ENABLED(CONFIG_64BIT))
    set_bit(EFI_64BIT, &efi.flags);
    efi_nr_tables	 = efi_systab.nr_tables;
    efi_config_table = (unsigned long)efi_systab.tables;
    size = sizeof(efi_config_table_t);
    config_tables = early_memremap(efi_config_table, efi_nr_tables * size);
    efi_config_parse_tables(config_tables, efi_systab.nr_tables, arch_tables);
    early_memunmap(config_tables, efi_nr_tables * size);
    set_bit(EFI_CONFIG_TABLES, &efi.flags);
    if (IS_ENABLED(CONFIG_EFI_EARLYCON) || IS_ENABLED(CONFIG_SYSFB))
    init_primary_display();
    if (boot_memmap == EFI_INVALID_TABLE_ADDR)
    return;
    tbl = early_memremap_ro(boot_memmap, sizeof(*tbl));
    if (tbl) {
    struct efi_memory_map_data data;
    data.phys_map		= boot_memmap + sizeof(*tbl);
    data.size		= tbl.map_size;
    data.desc_size		= tbl.desc_size;
    data.desc_version	= tbl.desc_ver;
    if (efi_memmap_init_early(&data) < 0)
    panic("Unable to map EFI memory map.\n");
//
// Reserve the physical memory region occupied by the EFI
// memory map table (header + descriptors). This is crucial
// for kdump, as the kdump kernel relies on this original
// memmap passed by the bootloader. Without reservation,
// this region could be overwritten by the primary kernel.
// Also, set the EFI_PRESERVE_BS_REGIONS flag to indicate that
// critical boot services code/data regions like this are preserved.
//
    memblock_reserve((phys_addr_t)boot_memmap, sizeof(*tbl) + data.size);
    set_bit(EFI_PRESERVE_BS_REGIONS, &efi.flags);
    early_memunmap(tbl, sizeof(*tbl));
    }
    efi_esrt_init();
    }
