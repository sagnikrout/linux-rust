//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/efi.h
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

extern "C" {
    pub fn efi_init();
}
extern "C" {
    pub fn efi_runtime_fixup_exception(regs: *mut pt_regs, msg: *const c_char) -> bool;
}

// Macro flag: #define efi_init()

extern "C" {
    pub fn efi_create_mapping(mm: *mut mm_struct, md: *mut efi_memory_desc_t) -> c_int;
}

extern "C" {
    pub fn __efi_rt_asm_wrapper(: *mut c_void, : *const c_char, ...) -> efi_status_t;
}
extern "C" {
    pub fn arch_efi_call_virt_setup();
}
extern "C" {
    pub fn arch_efi_call_virt_teardown();
}
//
// efi_rt_stack_top[-1] contains the value the stack pointer had before
// switching to the EFI runtime stack.
//

//
// Even when Linux uses IRQ priorities for IRQ disabling, EFI does not.
// And EFI shouldn't really play around with priority masking as it is not aware
// which priorities the OS has assigned to its interrupts.
//

// arch specific definitions used by the stub code
//
// In some configurations (e.g. VMAP_STACK && 64K pages), stacks built into the
// kernel need greater alignment than we require the segments to be padded to.
//

//
// On arm64, we have to ensure that the initrd ends up in the linear region,
// which is a 1 GB aligned region of size '1UL << (VA_BITS_MIN - 1)' that is
// guaranteed to cover the kernel Image.
//
// Since the EFI stub is part of the kernel Image, we can relax the
// usual requirements in Documentation/arch/arm64/booting.rst, which still
// apply to other bootloaders, and are required for some kernel
// configurations.
//
// Although relocatable kernels can fix up the misalignment with
// respect to MIN_KIMG_ALIGN, the resulting virtual text addresses are
// subtly out of sync with those recorded in the vmlinux when kaslr is
// disabled but the image required relocation anyway. Therefore retain
// 2M alignment if KASLR was explicitly disabled, even if it was not
// going to be activated to begin with.
//

extern "C" {
    pub fn primary_entry_offset() -> c_ulong;
}
//
// On ARM systems, virtually remapped UEFI runtime services are set up in two
// distinct stages:
// - The stub retrieves the final version of the memory map from UEFI, populates
// the virt_addr fields and calls the SetVirtualAddressMap() [SVAM] runtime
// service to communicate the new mapping to the firmware (Note that the new
// mapping is not live at this time)
// - During an early initcall(), the EFI system table is permanently remapped
// and the virtual remapping of the UEFI Runtime Services regions is loaded
// into a private set of page tables. If this all succeeds, the Runtime
// Services are enabled and the EFI_RUNTIME_SERVICES bit set.
//
// Update the current thread's saved ttbr0 since it is
// restored as part of a return from exception.
//
// Restore the current thread's saved ttbr0
// corresponding to its active_mm
//
extern "C" {
    pub fn efi_virtmap_load();
}
extern "C" {
    pub fn efi_virtmap_unload();
}
extern "C" {
    pub fn efi_handle_corrupted_x18(s: efi_status_t, f: *const c_char) -> efi_status_t;
}
extern "C" {
    pub fn efi_icache_sync(start: c_ulong, end: c_ulong);
}
