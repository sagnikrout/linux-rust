//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/iscsi_ibft_find.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2007-2010 Red Hat, Inc.
// by Peter Jones <pjones@redhat.com>
// Copyright 2007 IBM, Inc.
// by Konrad Rzeszutek <konradr@linux.vnet.ibm.com>
// Copyright 2008
// by Konrad Rzeszutek <ketuzsezr@darnok.org>
//
// This code finds the iSCSI Boot Format Table.
//

//
// Physical location of iSCSI Boot Format Table.
//
    phys_addr_t ibft_phys_addr;
    EXPORT_SYMBOL_GPL(ibft_phys_addr);
    static const struct {
    char *sign;
    } ibft_signs[] = {
    { "iBFT" },
    { "BIFT" },	/* Broadcom iSCSI Offload */
    };
pub const IBFT_SIGN_LEN: c_int = 4;
pub const VGA_MEM: c_uint = 0xA0000 /* VGA buffer */;
pub const VGA_SIZE: c_uint = 0x20000 /* 128kB */;
//
// Routine used to find and reserve the iSCSI Boot Format Table
//
#[no_mangle]
pub unsafe extern "C" fn reserve_ibft_region() -> void __init {
    void __init reserve_ibft_region(void)
    {
    unsigned long pos, virt_pos = 0;
    let mut len: c_uint = 0;
    void *virt = core::ptr::null_mut();
    int i;
    ibft_phys_addr = 0;
// iBFT 1.03 section 1.4.3.1 mandates that UEFI machines will
// only use ACPI for this
//
    if (efi_enabled(EFI_BOOT))
    return;
    for (pos = IBFT_START; pos < IBFT_END; pos += 16) {
// The table can't be inside the VGA BIOS reserved space,
// so skip that area
    if (pos == VGA_MEM)
    pos += VGA_SIZE;
// Map page by page
    if (offset_in_page(pos) == 0) {
    if (virt)
    early_memunmap(virt, PAGE_SIZE);
    virt = early_memremap_ro(pos, PAGE_SIZE);
    virt_pos = pos;
    }
    for (i = 0; i < ARRAY_SIZE(ibft_signs); i++) {
    if (memcmp(virt + (pos - virt_pos), ibft_signs[i].sign,
    IBFT_SIGN_LEN) == 0) {
    unsigned long *addr =
    (unsigned long *)(virt + pos - virt_pos + 4);
    len = *addr;
// if the length of the table extends past 1M,
// the table cannot be valid.
    if (pos + len <= (IBFT_END-1)) {
    ibft_phys_addr = pos;
    memblock_reserve(ibft_phys_addr, PAGE_ALIGN(len));
    pr_info("iBFT found at %pa.\n", &ibft_phys_addr);
    goto out;
    }
    }
    }
    }
    out:
    early_memunmap(virt, PAGE_SIZE);
    }
