//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/interface_64.h
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
// 64-bit segment selectors
// These flat segments are in the Xen-private section of every GDT. Since these
// are also present in the initial GDT, many OSes will be able to avoid
// installing their own GDT.
//
pub const FLAT_RING3_CS32: c_uint = 0xe023  /* GDT index 260 */;
pub const FLAT_RING3_CS64: c_uint = 0xe033  /* GDT index 261 */;
pub const FLAT_RING3_DS32: c_uint = 0xe02b  /* GDT index 262 */;
pub const FLAT_RING3_DS64: c_uint = 0x0000  /* NULL selector */;
pub const FLAT_RING3_SS32: c_uint = 0xe02b  /* GDT index 262 */;
pub const FLAT_RING3_SS64: c_uint = 0xe02b  /* GDT index 262 */;

pub const __HYPERVISOR_VIRT_START: c_uint = 0xFFFF800000000000;
pub const __HYPERVISOR_VIRT_END: c_uint = 0xFFFF880000000000;
pub const __MACH2PHYS_VIRT_START: c_uint = 0xFFFF800000000000;
pub const __MACH2PHYS_VIRT_END: c_uint = 0xFFFF804000000000;
pub const __MACH2PHYS_SHIFT: c_int = 3;
//
// int HYPERVISOR_set_segment_base(unsigned int which, unsigned long base)
// @which == SEGBASE_*  ;  @base == 64-bit base address
// Returns 0 on success.
//
pub const SEGBASE_FS: c_int = 0;
pub const SEGBASE_GS_USER: c_int = 1;
pub const SEGBASE_GS_KERNEL: c_int = 2;

//
// int HYPERVISOR_iret(void)
// All arguments are on the kernel stack, in the following format.
// Never returns if successful. Current kernel context is lost.
// The saved CS is mapped as follows:
// RING0 -> RING3 kernel mode.
// RING1 -> RING3 kernel mode.
// RING2 -> RING3 kernel mode.
// RING3 -> RING3 user mode.
// However RING0 indicates that the guest kernel should return to itself
// directly with
// orb   $3,1*8(%rsp)
// iretq
// If flags contains VGCF_in_syscall:
// Restore RAX, RIP, RFLAGS, RSP.
// Discard R11, RCX, CS, SS.
// Otherwise:
// Restore RAX, R11, RCX, CS:RIP, RFLAGS, SS:RSP.
// All other registers are saved on hypercall entry and restored to user.
//
// Guest exited in SYSCALL context? Return to guest with SYSRET?
pub const _VGCF_in_syscall: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iret_context {
// Top of stack (%rsp at point of hypercall).
    pub ss: uint64_t rax, r11, rcx, flags, rip, cs, rflags, rsp,,
// Bottom of iret stack frame.
}

// Anonymous union includes both 32- and 64-bit names (e.g., eax/rax).

// Non-gcc sources must always use the proper 64-bit name (e.g., rax).

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_user_regs {
    pub r15: u64,
    pub r14: u64,
    pub r13: u64,
    pub r12: u64,
    pub r11: u64,
    pub r10: u64,
    pub r9: u64,
    pub r8: u64,
    pub /: *mut *mut uint32_t error_code; / private,
    pub /: *mut *mut uint32_t entry_vector; / private,
    pub _pad0: [uint16_t cs,; 1],
    pub saved_upcall_mask: u8,
    pub _pad1: [u8; 3],
    pub /: *mut *mut __DECL_REG(flags); / rflags.IF == !saved_upcall_mask,
    pub _pad2: [uint16_t ss,; 3],
    pub _pad3: [uint16_t es,; 3],
    pub _pad4: [uint16_t ds,; 3],
    pub /: *mut *mut uint16_t fs, _pad5[3]; / Non-zero => takes precedence over fs_base.,
    pub /: *mut *mut uint16_t gs, _pad6[3]; / Non-zero => takes precedence over gs_base_usr.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_vcpu_info {
    pub cr2: c_ulong,
    pub /: *mut *mut unsigned long pad; / sizeof(vcpu_info_t) == 64,
}

pub type xen_callback_t = c_ulong;

