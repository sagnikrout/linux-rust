//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/loongarch.c
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
// Author: Yun Liu <liuyun@loongson.cn>
// Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    typedef void __noreturn (*kernel_entry_t)(bool efi, unsigned long cmdline,
    unsigned long systab);
#[no_mangle]
pub unsafe extern "C" fn check_platform_features() -> efi_status_t {
    efi_status_t check_platform_features(void)
    {
    return EFI_SUCCESS;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_cache_sync_image(image_base: c_ulong, alloc_size: c_ulong) {
    void efi_cache_sync_image(unsigned long image_base, unsigned long alloc_size)
    {
    asm volatile ("ibar 0" ::: "memory");
    }
#[no_mangle]
pub unsafe extern "C" fn efi_get_kimg_kaslr_address() -> c_ulong {
    unsigned long efi_get_kimg_kaslr_address(void)
    {
    let mut random_offset: c_uint = 0;

    if (!efi_nokaslr) {
    efi_get_random_bytes(sizeof(random_offset), (u8 *)&random_offset);
    random_offset ^= (random_get_entropy() << 16);
    random_offset &= (CONFIG_RANDOMIZE_BASE_MAX_OFFSET - 1);
    random_offset = ALIGN(random_offset + SZ_64K, SZ_64K);
    }

    return PHYSADDR(VMLINUX_LOAD_ADDRESS) + random_offset;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exit_boot_struct {
    pub runtime_map: *mut efi_memory_desc_t,
    pub runtime_entry_count: c_int,
}

#[no_mangle]
unsafe extern "C" fn exit_boot_func(map: *mut efi_boot_memmap, priv: *mut c_void) -> efi_status_t {
    static efi_status_t exit_boot_func(struct efi_boot_memmap *map, void *priv)
    {
    struct exit_boot_struct *p = priv;
//
// Update the memory map with virtual addresses. The function will also
// populate @runtime_map with copies of just the EFI_MEMORY_RUNTIME
// entries so that we can pass it straight to SetVirtualAddressMap()
//
    efi_get_virtmap(map.map, map.map_size, map.desc_size,
    p.runtime_map, &p.runtime_entry_count);
    return EFI_SUCCESS;
    }
    unsigned long __weak kernel_entry_address(unsigned long kernel_addr,
    efi_loaded_image_t *image)
    {
    return *(unsigned long *)(kernel_addr + 8) - PHYSADDR(VMLINUX_LOAD_ADDRESS) + kernel_addr;
    }
    efi_status_t efi_boot_kernel(void *handle, efi_loaded_image_t *image,
    unsigned long kernel_addr, char *cmdline_ptr)
    {
    kernel_entry_t real_kernel_entry;
    struct exit_boot_struct priv;
    unsigned long desc_size;
    efi_status_t status;
    u32 desc_ver;
    status = efi_alloc_virtmap(&priv.runtime_map, &desc_size, &desc_ver);
    if (status != EFI_SUCCESS) {
    efi_err("Unable to retrieve UEFI memory map.\n");
    return status;
    }
    efi_info("Exiting boot services\n");
    efi_novamap = false;
    status = efi_exit_boot_services(handle, &priv, exit_boot_func);
    if (status != EFI_SUCCESS)
    return status;
// Install the new virtual address map
    efi_rt_call(set_virtual_address_map,
    priv.runtime_entry_count * desc_size, desc_size,
    desc_ver, priv.runtime_map);
// Config Direct Mapping
    csr_write(CSR_DMW0_INIT, LOONGARCH_CSR_DMWIN0);
    csr_write(CSR_DMW1_INIT, LOONGARCH_CSR_DMWIN1);
    csr_write(CSR_DMW2_INIT, LOONGARCH_CSR_DMWIN2);
    csr_write(CSR_DMW3_INIT, LOONGARCH_CSR_DMWIN3);
    real_kernel_entry = (void *)kernel_entry_address(kernel_addr, image);
    real_kernel_entry(true, (unsigned long)cmdline_ptr,
    (unsigned long)efi_system_table);
    }
