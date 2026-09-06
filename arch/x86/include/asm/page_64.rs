//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/page_64.h
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

// duplicated to the one in bootmem.h
// use the carry flag to determine if x was < __START_KERNEL_map

extern "C" {
    pub fn __phys_addr(long: unsigned) -> c_ulong;
}

// only check upper bounds since lower bounds will trigger carry

extern "C" {
    pub fn __clear_pages_unrolled(page: *mut c_void);
}
//
// clear_pages() - clear a page range using a kernel virtual address.
// @addr: start address of kernel page range
// @npages: number of pages
//
// Switch between three implementations of page clearing based on CPU
// capabilities:
//
// - __clear_pages_unrolled(): the oldest, slowest and universally
// supported method. Zeroes via 8-byte MOV instructions unrolled 8x
// to write a 64-byte cacheline in each loop iteration.
//
// - "REP; STOSQ": really old CPUs had crummy REP implementations.
// Vendor CPU setup code sets 'REP_GOOD' on CPUs where REP can be
// trusted. The instruction writes 8-byte per REP iteration but
// CPUs can internally batch these together and do larger writes.
//
// - "REP; STOSB": used on CPUs with "enhanced REP MOVSB/STOSB",
// which enumerate 'ERMS' and provide an implementation which
// unlike "REP; STOSQ" above wasn't overly picky about alignment.
// The instruction writes 1-byte per REP iteration with CPUs
// internally batching these together into larger writes and is
// generally fastest of the three.
//
// Note that when running as a guest, features exposed by the CPU
// might be mediated by the hypervisor. So, the STOSQ variant might
// be in active use on some systems even when the hardware enumerates
// ERMS.
//
// Does absolutely no exception handling.
//
// Clean up KMSAN metadata for the pages being cleared. The assembly call
// below clobbers @addr, so perform unpoisoning before it.
//
// The inline asm embeds a CALL instruction and usually that is a no-no
// due to the compiler not knowing that and thus being unable to track
// callee-clobbered registers.
//
// In this case that is fine because the registers clobbered by
// __clear_pages_unrolled() are part of the inline asm register
// specification.
//

extern "C" {
    pub fn copy_page(to: *mut c_void, from: *mut c_void);
}
//
// User space process size.  This is the first address outside the user range.
// There are a few constraints that determine this:
//
// On Intel CPUs, if a SYSCALL instruction is at the highest canonical
// address, then that syscall will enter the kernel with a
// non-canonical return address, and SYSRET will explode dangerously.
// We avoid this particular problem by preventing anything
// from being mapped at the maximum canonical address.
//
// On AMD CPUs in the Ryzen family, there's a nasty bug in which the
// CPUs malfunction if they execute code from the highest canonical page.
// They'll speculate right off the end of the canonical space, and
// bad things happen.  This is worked around in the same way as the
// Intel problem.
//
// With page table isolation enabled, we map the LDT in ... [stay tuned]
//

