//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kexec.h
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

// Size of each exception handler referenced by the IDT

//
// KEXEC_SOURCE_MEMORY_LIMIT maximum page get_free_page can return.
// I.e. Maximum page that is mapped directly into kernel memory,
// and kmap is not required.
//
// So far x86_64 is limited to 40 physical address bits.
//

// Maximum physical address we can use pages from

// Maximum address we can reach in physical address mode

// Maximum address we can use for the control code buffer

// The native architecture

// We can also handle crash dumps from 64 bit kernel.

// Maximum physical address we can use pages from

// Maximum address we can reach in physical address mode

// Maximum address we can use for the control pages

// The native architecture

//
// This function is responsible for capturing register states if coming
// via panic otherwise just fix up the ss and sp if coming via kernel
// mode exception.
//
extern "C" {
    pub fn volatile(": "mov %%" _ASM_BX, "=m"(newregs->bx): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(": "mov %%" _ASM_CX, "=m"(newregs->cx): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(": "mov %%" _ASM_DX, "=m"(newregs->dx): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(": "mov %%" _ASM_SI, "=m"(newregs->si): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(": "mov %%" _ASM_DI, "=m"(newregs->di): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(": "mov %%" _ASM_BP, "=m"(newregs->bp): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(": "mov %%" _ASM_AX, "=m"(newregs->ax): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(": "mov %%" _ASM_SP, "=m"(newregs->sp): %0" :) -> asm;
}

extern "C" {
    pub fn volatile(%%r8: "mov, "=m"(newregs->r8): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%r9: "mov, "=m"(newregs->r9): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%r10: "mov, "=m"(newregs->r10): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%r11: "mov, "=m"(newregs->r11): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%r12: "mov, "=m"(newregs->r12): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%r13: "mov, "=m"(newregs->r13): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%r14: "mov, "=m"(newregs->r14): %0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%r15: "mov, "=m"(newregs->r15): %0" :) -> asm;
}

extern "C" {
    pub fn volatile(%%ss: "mov, "=a"(newregs->ss): %k0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%cs: "mov, "=a"(newregs->cs): %k0" :) -> asm;
}

extern "C" {
    pub fn volatile(%%ds: "mov, "=a"(newregs->ds): %k0" :) -> asm;
}
extern "C" {
    pub fn volatile(%%es: "mov, "=a"(newregs->es): %k0" :) -> asm;
}

// Macro flag: #define ARCH_HAS_KIMAGE_ARCH

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kimage_arch {
    pub pgd: *mut pgd_t,

    pub pmd0: *mut pmd_t,
    pub pmd1: *mut pmd_t,

    pub pte0: *mut pte_t,
    pub pte1: *mut pte_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kimage_arch {
//
// This is a kimage control page, as it must not overlap with either
// source or destination address ranges.
//
    pub pgd: *mut pgd_t,
//
// The virtual mapping of the control code page itself is used only
// during the transition, while the current kernel's pages are all
// in place. Thus the intermediate page table pages used to map it
// are not control pages, but instead just normal pages obtained
// with get_zeroed_page(). And have to be tracked (below) so that
// they can be freed.
//
    pub p4d: *mut p4d_t,
    pub pud: *mut pud_t,
    pub pmd: *mut pmd_t,
    pub pte: *mut pte_t,
}

//
// Number of elements and order of elements in this structure should match
// with the ones in arch/x86/purgatory/entry64.S. If you make a change here
// make an appropriate change in purgatory too.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kexec_entry64_regs {
    pub rax: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbx: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
}

extern "C" {
    pub fn arch_kexec_pre_free_pages(vaddr: *mut c_void, pages: c_uint);
}

extern "C" {
    pub fn arch_kexec_protect_crashkres();
}

extern "C" {
    pub fn arch_kexec_unprotect_crashkres();
}

extern "C" {
    pub fn arch_kimage_file_post_load_cleanup(image: *mut kimage) -> c_int;
}

extern "C" {
    pub fn kdump_nmi_shootdown_cpus();
}

extern "C" {
    pub fn arch_crash_handle_hotplug_event(image: *mut kimage, arg: *mut c_void);
}

extern "C" {
    pub fn arch_crash_hotplug_support(image: *mut kimage, kexec_flags: c_ulong) -> c_int;
}

extern "C" {
    pub fn arch_crash_get_elfcorehdr_size() -> c_uint;
}

