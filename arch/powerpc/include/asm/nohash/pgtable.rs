//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/pgtable.h
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
// _PAGE_CHG_MASK masks of bits that are to be preserved across
// pgprot changes.
//

// Permission masks used for kernel mappings

//
// PTE updates. This function is called whenever an existing
// valid PTE is updated. This does -not- include set_pte_at()
// which nowadays only sets a new PTE.
//
// Depending on the type of MMU, we may need to use atomic updates
// and the PTE may be either 32 or 64 bit wide. In the later case,
// when using atomic updates, only the low part of the PTE is
// accessed atomically.
//
// In addition, on 44x, we also maintain a global flag indicating
// that an executable user mapping was modified, which is needed
// to properly flush the virtually tagged instruction cache of
// those implementations.
//

// p = __pte(new);
// huge pages use the old page table lock

// Set the dirty and/or accessed bits atomically in a linux PTE

// Generic accessors to PTE bits

//
// write implies read, hence set both
//
extern "C" {
    pub fn __pte(_PAGE_RW: pte_val(pte) |) -> return;
}

extern "C" {
    pub fn __pte(_PAGE_DIRTY: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_ACCESSED: pte_val(pte) |) -> return;
}

extern "C" {
    pub fn __pte(~_PAGE_WRITE: pte_val(pte) &) -> return;
}

extern "C" {
    pub fn __pte(_PAGE_EXEC: pte_val(pte) |) -> return;
}

//
// Don't just check for any non zero bits in __PAGE_READ, since for book3e
// and PTE_64BIT, PAGE_KERNEL_X contains _PAGE_BAP_SR which is also in
// _PAGE_READ.  Need to explicitly match _PAGE_BAP_UR bit in that case too.
//

//
// We only find page table entry in the last level
// Hence no need for other accessors
//

//
// A read-only access is controlled by _PAGE_READ bit.
// We have _PAGE_READ set for WRITE
//
extern "C" {
    pub fn pte_present(!is_kernel_addr(addr: pte) &&) -> return;
}
// Conversion functions: convert a page and protection to a page entry,
// and a page entry and page directory to the page they refer to.
//
// Even if PTEs can be unsigned long long, a PFN is always an unsigned
// long for now.
//
// Generic modifiers for PTE bits
extern "C" {
    pub fn __pte(~_PAGE_EXEC: pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_DIRTY: pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_ACCESSED: pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_SPECIAL: pte_val(pte) |) -> return;
}

extern "C" {
    pub fn __pte(_arg: pte_val(pte)) -> return;
}

extern "C" {
    pub fn __pte(pgprot_val(newprot): (pte_val(pte) & _PAGE_CHG_MASK) |) -> return;
}
extern "C" {
    pub fn __pte(_PAGE_SWP_EXCLUSIVE: pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pte(~_PAGE_SWP_EXCLUSIVE: pte_val(pte) &) -> return;
}
// This low level function performs the actual PTE insertion
// Setting the PTE depends on the MMU type and other factors. It's
// an horrible mess that I'm not going to try to clean up now but
// I'm keeping it in one place rather than spread around
//
// Second case is 32-bit with 64-bit PTE.  In this case, we
// can just store as long as we do the two halves in the right order
// with a barrier in between.
// In the percpu case, we also fallback to the simple update
//
// Anything else just stores the PTE normally. That covers all 64-bit
// cases, and 32-bit non-hash with 32-bit PTEs.
//

// ptep = pte;

//
// With hardware tablewalk, a sync is needed to ensure that
// subsequent accesses see the PTE we just wrote.  Unlike userspace
// mappings, we can't tolerate spurious faults, so make sure
// the new PTE will be seen the first time.
//
// Macro to mark a page protection value as "uncacheable".
//

extern "C" {
    pub fn map_kernel_page(va: c_ulong, pa: phys_addr_t, prot: pgprot_t) -> c_int;
}
extern "C" {
    pub fn unmap_kernel_page(va: c_ulong);
}

