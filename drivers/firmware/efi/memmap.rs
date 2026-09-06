//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/memmap.c
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
// Common EFI memory map functions.
//

//
// __efi_memmap_init - Common code for mapping the EFI memory map
// @data: EFI memory map data
//
// This function takes care of figuring out which function to use to
// map the EFI memory map in efi.memmap based on how far into the boot
// we are.
//
// During bootup EFI_MEMMAP_LATE in data->flags should be clear since we
// only have access to the early_memremap*() functions as the vmalloc
// space isn't setup.  Once the kernel is fully booted we can fallback
// to the more robust memremap*() API.
//
// Returns: zero on success, a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn __efi_memmap_init(data: *mut efi_memory_map_data) -> int __init {
    int __init __efi_memmap_init(struct efi_memory_map_data *data)
    {
    struct efi_memory_map map;
    phys_addr_t phys_map;
    phys_map = data.phys_map;
    if (data.flags & EFI_MEMMAP_LATE)
    map.map = memremap(phys_map, data.size, MEMREMAP_WB);
    else
    map.map = early_memremap(phys_map, data.size);
    if (!map.map) {
    pr_err("Could not map the memory map! phys_map=%pa, size=0x%lx\n",
    &phys_map, data.size);
    return -ENOMEM;
    }
    map.phys_map = data.phys_map;
    map.nr_map = data.size / data.desc_size;
    map.map_end = map.map + data.size;
    map.desc_version = data.desc_version;
    map.desc_size = data.desc_size;
    map.flags = data.flags;
    set_bit(EFI_MEMMAP, &efi.flags);
    efi.memmap = map;
    return 0;
    }
//
// efi_memmap_init_early - Map the EFI memory map data structure
// @data: EFI memory map data
//
// Use early_memremap() to map the passed in EFI memory map and assign
// it to efi.memmap.
//
// Returns: zero on success, a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn efi_memmap_init_early(data: *mut efi_memory_map_data) -> int __init {
    int __init efi_memmap_init_early(struct efi_memory_map_data *data)
    {
// Cannot go backwards
    WARN_ON(efi.memmap.flags & EFI_MEMMAP_LATE);
    data.flags = 0;
    return __efi_memmap_init(data);
    }
#[no_mangle]
pub unsafe extern "C" fn efi_memmap_unmap() -> void __init {
    void __init efi_memmap_unmap(void)
    {
    if (!efi_enabled(EFI_MEMMAP))
    return;
    if (!(efi.memmap.flags & EFI_MEMMAP_LATE)) {
    unsigned long size;
    size = efi.memmap.desc_size * efi.memmap.nr_map;
    early_memunmap(efi.memmap.map, size);
    } else {
    memunmap(efi.memmap.map);
    }
    efi.memmap.map = core::ptr::null_mut();
    clear_bit(EFI_MEMMAP, &efi.flags);
    }
//
// efi_memmap_init_late - Map efi.memmap with memremap()
// @addr: Physical address of the new EFI memory map
// @size: Size in bytes of the new EFI memory map
//
// Setup a mapping of the EFI memory map using ioremap_cache(). This
// function should only be called once the vmalloc space has been
// setup and is therefore not suitable for calling during early EFI
// initialise, e.g. in efi_init(). Additionally, it expects
// efi_memmap_init_early() to have already been called.
//
// The reason there are two EFI memmap initialisation
// (efi_memmap_init_early() and this late version) is because the
// early EFI memmap should be explicitly unmapped once EFI
// initialisation is complete as the fixmap space used to map the EFI
// memmap (via early_memremap()) is a scarce resource.
//
// This late mapping is intended to persist for the duration of
// runtime so that things like efi_mem_desc_lookup() and
// efi_mem_attributes() always work.
//
// Returns: zero on success, a negative error code on failure.
//
#[no_mangle]
pub unsafe extern "C" fn efi_memmap_init_late(addr: phys_addr_t, size: c_ulong) -> int __init {
    int __init efi_memmap_init_late(phys_addr_t addr, unsigned long size)
    {
    struct efi_memory_map_data data = {
    .phys_map = addr,
    .size = size,
    .flags = EFI_MEMMAP_LATE,
    };
// Did we forget to unmap the early EFI memmap?
    WARN_ON(efi.memmap.map);
// Were we already called?
    WARN_ON(efi.memmap.flags & EFI_MEMMAP_LATE);
//
// It makes no sense to allow callers to register different
// values for the following fields. Copy them out of the
// existing early EFI memmap.
//
    data.desc_version = efi.memmap.desc_version;
    data.desc_size = efi.memmap.desc_size;
    return __efi_memmap_init(&data);
    }
