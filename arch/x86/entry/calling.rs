//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/entry/calling.h
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

//
// 64-bit system call stack frame layout defines and helpers,
// for assembly code:
//
// We just clobbered the return address - use the IRET frame for unwinding:
//
// Sanitize registers of values that a speculation attack might
// otherwise want to exploit. The lower registers are likely clobbered
// well before they could be put to use in a speculative execution
// gadget.
//

//
// MITIGATION_PAGE_TABLE_ISOLATION PGDs are 8k.  Flip bit 12 to switch between the two
// halves:
//

// Clear PCID and "MITIGATION_PAGE_TABLE_ISOLATION bit", point CR3 at kernel pagetables:

//
// Test if the ASID needs a flush.
//
// Flush needed, clear the bit
// Flip the ASID to the user version
// Flip the PGD to the user version
//
// Test the user pagetable bit. If set, then the user page tables
// are active. If clear CR3 already has the kernel page table
// active.
//
// Restore CR3 from a kernel context. May restore a user CR3 value.
//
// If CR3 contained the kernel page tables at the paranoid exception
// entry, then there is nothing to restore as CR3 is not modified while
// handling the exception.
//
// Check if there's a pending flush for the user ASID we're
// about to set.
//

//
// IBRS kernel mitigation for Spectre_v2.
//
// Assumes full context is established (PUSH_REGS, CR3 and GS) and it clobbers
// the regs it uses (AX, CX, DX). Must be called before the first RET
// instruction (NOTE! UNTRAIN_RET includes a RET instruction)
//
// The optional argument is used to save/restore the current value,
// which is used on the paranoid paths.
//
// Assumes x86_spec_ctrl_{base,current} to have SPEC_CTRL_IBRS set.
//

//
// Similar to IBRS_ENTER, requires KERNEL GS,CR3 and clobbers (AX, CX, DX)
// regs. Must be called after the last RET.
//

//
// Mitigate Spectre v1 for conditional swapgs code paths.
//
// FENCE_SWAPGS_USER_ENTRY is used in the user entry swapgs code path, to
// prevent a speculative swapgs when coming from kernel space.
//
// FENCE_SWAPGS_KERNEL_ENTRY is used in the kernel entry non-swapgs code path,
// to prevent the swapgs from getting speculatively skipped when coming from
// user space.
//

//
// CPU/node NR is loaded from the limit (size) field of a special segment
// descriptor entry in GDT.
//
// Fetch the per-CPU GSBASE value for this processor and put it in @reg.
// We normally use %gs for accessing per-CPU data, but we are setting up
// %gs here and obviously can not use %gs itself to access per-CPU data.
//
// Do not use RDPID, because KVM loads guest's TSC_AUX on vm-entry and
// may not restore the host's value until the CPU returns to userspace.
// Thus the kernel would consume a guest's TSC_AUX if an NMI arrives
// while running KVM's run loop.
//

// rdi:	arg1 ... normal C conventions. rax is saved/restored.

// put return address in eax (arg1)
// Place EIP in the arg1
