//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/kernel/mem.c
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
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

#[no_mangle]
pub unsafe extern "C" fn memblock_init() -> void __init {
    void __init memblock_init(void)
    {
    u32 mem_type;
    u64 mem_start, mem_size;
    efi_memory_desc_t *md;
// Parse memory information
    for_each_efi_memory_desc(md) {
    mem_type = md.type;
    mem_start = md.phys_addr;
    mem_size = md.num_pages << EFI_PAGE_SHIFT;
    switch (mem_type) {
    case EFI_LOADER_CODE:
    case EFI_LOADER_DATA:
    case EFI_BOOT_SERVICES_CODE:
    case EFI_BOOT_SERVICES_DATA:
    case EFI_PERSISTENT_MEMORY:
    case EFI_CONVENTIONAL_MEMORY:
    memblock_add(mem_start, mem_size);
    break;
    case EFI_PAL_CODE:
    case EFI_UNUSABLE_MEMORY:
    case EFI_ACPI_RECLAIM_MEMORY:
    memblock_add(mem_start, mem_size);
    fallthrough;
    case EFI_RESERVED_TYPE:
    case EFI_RUNTIME_SERVICES_CODE:
    case EFI_RUNTIME_SERVICES_DATA:
    case EFI_MEMORY_MAPPED_IO:
    case EFI_MEMORY_MAPPED_IO_PORT_SPACE:
    memblock_reserve(mem_start, mem_size);
    break;
    }
    }
    max_pfn = PFN_DOWN(memblock_end_of_DRAM());
    max_low_pfn = min(PFN_DOWN(HIGHMEM_START), max_pfn);
    memblock_set_current_limit(PFN_PHYS(max_low_pfn));
// Reserve the first 2MB
    memblock_reserve(PHYS_OFFSET, 0x200000);
// Reserve the kernel text/data/bss
    memblock_reserve(__pa_symbol(&_text),
    __pa_symbol(&_end) - __pa_symbol(&_text));
    memblock_set_node(0, PHYS_ADDR_MAX, &memblock.memory, 0);
    memblock_set_node(0, PHYS_ADDR_MAX, &memblock.reserved, 0);
    }
