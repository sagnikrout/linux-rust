//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/boot/compressed/misc.h
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
// Special hack: we have to be careful, because no indirections are allowed here,
// and paravirt_ops is a kind of one. As it will only run in baremetal anyway,
// we just keep it from happening. (This list needs to be extended when new
// paravirt and debugging variants are added.)
//

// cpu_feature_enabled() cannot be used this early
// Macro flag: #define USE_EARLY_PGTABLE_L5
//
// Boot stub deals with identity mappings, physical and virtual addresses are
// the same, so override these defines.
//
// <asm/page.h> will not define them if they are already defined.
//

// boot/compressed/vmlinux start and end markers
// misc.c
extern "C" {
    pub fn free(where: *mut c_void);
}
extern "C" {
    pub fn __putstr(s: *const c_char);
}
extern "C" {
    pub fn __puthex(value: c_ulong);
}
extern "C" {
    pub fn __putdec(value: c_ulong);
}

// cmdline.c
extern "C" {
    pub fn cmdline_find_option(option: *const c_char, buffer: *mut c_char, bufsize: c_int) -> c_int;
}
extern "C" {
    pub fn cmdline_find_option_bool(option: *const c_char) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mem_vector {
    pub start: u64,
    pub size: u64,
}

// kaslr.c

// cpuflags.c
extern "C" {
    pub fn has_cpuflag(flag: c_int) -> bool;
}

extern "C" {
    pub fn set_page_decrypted(address: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_page_encrypted(address: c_ulong) -> c_int;
}
extern "C" {
    pub fn set_page_non_present(address: c_ulong) -> c_int;
}

// early_serial_console.c
extern "C" {
    pub fn console_init();
}

extern "C" {
    pub fn sev_enable(bp: *mut boot_params);
}
extern "C" {
    pub fn snp_check_features();
}
extern "C" {
    pub fn sev_es_shutdown_ghcb();
}
extern "C" {
    pub fn sev_es_check_ghcb_fault(address: c_ulong) -> bool;
}
extern "C" {
    pub fn snp_set_page_private(paddr: c_ulong);
}
extern "C" {
    pub fn snp_set_page_shared(paddr: c_ulong);
}
extern "C" {
    pub fn sev_prep_identity_maps(top_level_pgt: c_ulong);
}
extern "C" {
    pub fn vc_decode_insn(ctxt: *mut es_em_ctxt) -> es_result;
}
extern "C" {
    pub fn insn_has_rep_prefix(insn: *mut insn) -> bool;
}
extern "C" {
    pub fn sev_insn_decode_init();
}
extern "C" {
    pub fn early_setup_ghcb() -> bool;
}

// acpi.c

extern "C" {
    pub fn get_rsdp_addr() -> acpi_physical_address;
}

extern "C" {
    pub fn count_immovable_mem_regions() -> c_int;
}

// ident_map_64.c
extern "C" {
    pub fn kernel_add_identity_map(start: c_ulong, end: c_ulong);
}
// Used by PAGE_KERN* macros:
// idt_64.c

extern "C" {
    pub fn cleanup_exception_handling();
}

// IDT Entry Points
extern "C" {
    pub fn boot_page_fault();
}
extern "C" {
    pub fn boot_nmi_trap();
}
extern "C" {
    pub fn boot_stage1_vc();
}
extern "C" {
    pub fn boot_stage2_vc();
}
extern "C" {
    pub fn sev_verify_cbit(cr3: c_ulong) -> c_ulong;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum efi_type {
    EFI_TYPE_64,
    EFI_TYPE_32,
    EFI_TYPE_NONE,
}

// helpers for early EFI config table access
extern "C" {
    pub fn efi_get_type(bp: *mut boot_params) -> efi_type;
}
extern "C" {
    pub fn efi_get_system_table(bp: *mut boot_params) -> c_ulong;
}

extern "C" {
    pub fn init_unaccepted_memory() -> bool;
}

// Defined in EFI stub
extern "C" {
    pub fn accept_memory(start: phys_addr_t, size: c_ulong);
}
