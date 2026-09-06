//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/efi.h
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
// We map the EFI regions needed for runtime services non-contiguously,
// with preserved alignment on virtual addresses starting from -4G down
// for a total max space of 64G. This way, we provide for stable runtime
// services addresses across kernels so that a kexec'd kernel can still
// use them.
//
// This is the main reason why we're doing stable VA mappings for RT
// services.
//

//
// The EFI services are called through variadic functions in many cases. These
// functions are implemented in assembler and support only a fixed number of
// arguments. The macros below allows us to check at build time that we don't
// try to call them with too many arguments.
//
// __efi_nargs() will return the number of arguments if it is 7 or less, and
// cause a BUILD_BUG otherwise. The limitations of the C preprocessor make it
// impossible to calculate the exact number of arguments beyond some
// pre-defined limit. The maximum number of arguments currently supported by
// any of the thunks is 7, so this is good enough for now and can be extended
// in the obvious way if we ever need more.
//

//
// __efi_nargs_check(f, n, ...) will cause a BUILD_BUG if the ellipsis
// represents more than n arguments.
//

//
// The UEFI calling convention (UEFI spec 2.3.2 and 2.3.4) requires
// that FCW and MXCSR (64-bit) must be initialized prior to calling
// UEFI code.  (Oddly the spec does not require that the FPU stack
// be empty.)
//

extern "C" {
    pub fn __efi_call(fp: *mut c_void, ...) -> asmlinkage u64;
}

//
// CONFIG_KASAN may redefine memset to __memset.  __memset function is present
// only in kernel binary.  Since the EFI stub linked into a separate binary it
// doesn't have __memset().  So we should use standard memset from
// arch/x86/boot/compressed/string.c.  The same applies to memcpy and memmove.
//

extern "C" {
    pub fn efi_memblock_x86_reserve_range() -> int __init;
}
extern "C" {
    pub fn efi_print_memmap() -> void __init;
}
extern "C" {
    pub fn efi_map_region(md: *mut efi_memory_desc_t) -> void __init;
}
extern "C" {
    pub fn efi_map_region_fixed(md: *mut efi_memory_desc_t) -> void __init;
}
extern "C" {
    pub fn efi_sync_low_kernel_mappings();
}
extern "C" {
    pub fn efi_alloc_page_tables() -> int __init;
}
extern "C" {
    pub fn efi_setup_page_tables(pa_memmap: c_ulong, num_pages: unsigned) -> int __init;
}
extern "C" {
    pub fn efi_runtime_update_mappings() -> void __init;
}
extern "C" {
    pub fn efi_dump_pagetable() -> void __init;
}
extern "C" {
    pub fn efi_apply_memmap_quirks() -> void __init;
}
extern "C" {
    pub fn efi_reuse_config(tables: u64, nr_tables: c_int) -> int __init;
}
extern "C" {
    pub fn efi_delete_dummy_variable();
}
extern "C" {
    pub fn efi_unmap_boot_services();
}
extern "C" {
    pub fn arch_efi_call_virt_setup();
}
extern "C" {
    pub fn arch_efi_call_virt_teardown();
}

extern "C" {
    pub fn __efi64_thunk(_arg: u32, ...) -> u64;
}

extern "C" {
    pub fn IS_ENABLED(!efi_enabled(EFI_64BIT: CONFIG_X86_64) &&) -> return;
}
extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_EFI_MIXED) -> return;
}
extern "C" {
    pub fn parse_efi_setup(phys_addr: u64, data_len: u32);
}
extern "C" {
    pub fn efi_thunk_runtime_setup();
}
// arch specific definitions used by the stub code

// Macro flag: #define ARCH_HAS_EFISTUB_WRAPPERS
extern "C" {
    pub fn efi_is_64bit() -> return;
}

//
// The following macros allow translating arguments if necessary from native to
// mixed mode. The use case for this is to initialize the upper 32 bits of
// output parameters, and where the 32-bit method requires a 64-bit argument,
// which must be split up into two arguments to be thunked properly.
//
// As examples, the AllocatePool boot service returns the address of the
// allocation, but it will not set the high 32 bits of the address. To ensure
// that the full 64-bit address is initialized, we zero-init the address before
// calling the thunk.
//
// The FreePages boot service takes a 64-bit physical address even in 32-bit
// mode. For the thunk to work correctly, a native 64-bit call of
// free_pages(addr, size)
// must be translated to
// efi64_thunk(free_pages, addr & U32_MAX, addr >> 32, size)
// so that the two 32-bit halves of addr get pushed onto the stack separately.
//

// PCI I/O

// LoadFile

// Graphics Output Protocol

// TCG2 protocol

// DXE services

// file protocol

// file system protocol

// Memory Attribute Protocol

// EFI SMBIOS protocol

//
// The macros below handle the plumbing for the argument mapping. To add a
// mapping for a specific EFI method, simply define a macro
// __efi64_argmap_<method name>, following the examples above.
//

// Macro flag: #define __efi_eat(...)

// use rotate to move the value of bit #31 into position #63
extern "C" {
    pub fn ror64(_arg: rol32(status, _arg: 1), _arg: 1) -> return;
}
// The macro below handles dispatching via the thunk if needed

extern "C" {
    pub fn IS_ENABLED(_arg: CONFIG_X86_64) -> return;
}

extern "C" {
    pub fn efi_reboot_required() -> bool;
}
extern "C" {
    pub fn efi_is_table_address(phys_addr: c_ulong) -> bool;
}
extern "C" {
    pub fn efi_reserve_boot_services();
}

extern "C" {
    pub fn efi_memmap_install(data: *mut efi_memory_map_data) -> int __init;
}
extern "C" {
    pub fn __x86_efi_boot_mode() -> efi_secureboot_mode;
}

extern "C" {
    pub fn efi_get_runtime_map_size() -> c_int;
}
extern "C" {
    pub fn efi_get_runtime_map_desc_size() -> c_int;
}
extern "C" {
    pub fn efi_runtime_map_copy(buf: *mut c_void, bufsz: usize) -> c_int;
}

