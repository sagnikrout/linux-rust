//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/cpu_entry_area.h
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

pub const VC_EXCEPTION_STKSZ: c_int = 0;

// Macro to enforce the same ordering and stack sizes

// The exception stacks' physical storage. No guard pages required
#[repr(C)]
#[derive(Copy, Clone)]
pub struct exception_stacks {
}

// The effective cpu entry area mapping with guard pages.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cea_exception_stacks {
}

//
// The exception stack ordering in [cea_]exception_stacks
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exception_stack_ordering {
    ESTACK_DF,
    ESTACK_NMI,
    ESTACK_DB,
    ESTACK_MCE,
    ESTACK_VC,
    ESTACK_VC2,
    N_EXCEPTION_STACKS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct doublefault_stack {
    pub long)]: unsigned long stack[(PAGE_SIZE - sizeof(struct x86_hw_tss)) / sizeof(unsigned,
    pub tss: x86_hw_tss,
    pub __aligned(PAGE_SIZE): },

//
// cpu_entry_area is a percpu region that contains things needed by the CPU
// and early entry/exit code.  Real types aren't used for all fields here
// to avoid circular header dependencies.
//
// Every field is a virtual alias of some other allocated backing store.
// There is no direct allocation of a struct cpu_entry_area.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_entry_area {
    pub gdt: [c_char; PAGE_SIZE],
//
// The GDT is just below entry_stack and thus serves (on x86_64) as
// a read-only guard page. On 32-bit the GDT must be writeable, so
// it needs an extra guard page.
//
    pub guard_entry_stack: [c_char; PAGE_SIZE],
    pub entry_stack_page: entry_stack_page,
    pub guard_doublefault_stack: [c_char; PAGE_SIZE],
    pub doublefault_stack: doublefault_stack,

//
// On x86_64, the TSS is mapped RO.  On x86_32, it's mapped RW because
// we need task switches to work, and task switches write to the TSS.
//
    pub tss: tss_struct,

//
// Exception stacks used for IST entries with guard pages.
//
    pub estacks: cea_exception_stacks,

//
// Per CPU debug store for Intel performance monitoring. Wastes a
// full page at the moment.
//
    pub cpu_debug_store: debug_store,
//
// The actual PEBS/BTS buffers must be mapped to user space
// Reserve enough fixmap PTEs.
//
    pub cpu_debug_buffers: debug_store_buffers,
}

extern "C" {
    pub fn setup_cpu_entry_areas();
}
extern "C" {
    pub fn cea_set_pte(cea_vaddr: *mut c_void, pa: phys_addr_t, flags: pgprot_t);
}

