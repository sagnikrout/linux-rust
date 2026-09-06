//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/alignedmem.c
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
// efi_allocate_pages_aligned() - Allocate memory pages
// @size:	minimum number of bytes to allocate
// @addr:	On return the address of the first allocated page. The first
// allocated page has alignment EFI_ALLOC_ALIGN which is an
// architecture dependent multiple of the page size.
// @max:	the address that the last allocated memory page shall not
// exceed
// @align:	minimum alignment of the base of the allocation
// @memory_type: the type of memory to allocate
//
// Allocate pages as EFI_LOADER_DATA. The allocated pages are aligned according
// to @align, which should be >= EFI_ALLOC_ALIGN. The last allocated page will
// not exceed the address given by @max.
//
// Return:	status code
//
    efi_status_t efi_allocate_pages_aligned(unsigned long size, unsigned long *addr,
    unsigned long max, unsigned long align,
    int memory_type)
    {
    efi_physical_addr_t alloc_addr;
    efi_status_t status;
    int slack;
    max = min(max, EFI_ALLOC_LIMIT);
    if (align < EFI_ALLOC_ALIGN)
    align = EFI_ALLOC_ALIGN;
    alloc_addr = ALIGN_DOWN(max + 1, align) - 1;
    size = round_up(size, EFI_ALLOC_ALIGN);
    slack = align / EFI_PAGE_SIZE - 1;
    status = efi_bs_call(allocate_pages, EFI_ALLOCATE_MAX_ADDRESS,
    memory_type, size / EFI_PAGE_SIZE + slack,
    &alloc_addr);
    if (status != EFI_SUCCESS)
    return status;
// addr = ALIGN((unsigned long)alloc_addr, align);
    if (slack > 0) {
    let mut l: c_int = (alloc_addr & (align - 1)) / EFI_PAGE_SIZE;
    if (l) {
    efi_bs_call(free_pages, alloc_addr, slack - l + 1);
    slack = l - 1;
    }
    if (slack)
    efi_bs_call(free_pages, *addr + size, slack);
    }
    return EFI_SUCCESS;
    }
