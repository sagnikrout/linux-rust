//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/xen/page.h
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

// Xen machine address
// Xen pseudo-physical address

// MACHINE <-> PHYSICAL CONVERSION MACROS

extern "C" {
    pub fn xen_alloc_p2m_entry(pfn: c_ulong) -> c_int;
}
extern "C" {
    pub fn get_phys_to_machine(pfn: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn set_phys_to_machine(pfn: c_ulong, mfn: c_ulong) -> bool;
}
extern "C" {
    pub fn __set_phys_to_machine(pfn: c_ulong, mfn: c_ulong) -> bool;
}

//
// Helper functions to write or read unsigned long values to/from
// memory, when the access may fault.
//
// val = rval;

//
// When to use pfn_to_mfn(), __pfn_to_mfn() or get_phys_to_machine():
// - pfn_to_mfn() returns either INVALID_P2M_ENTRY or the mfn. No indicator
// bits (identity or foreign) are set.
// - __pfn_to_mfn() returns the found entry of the p2m table. A possibly set
// identity or foreign indicator will be still set. __pfn_to_mfn() is
// encapsulating get_phys_to_machine() which is called in special cases only.
// - get_phys_to_machine() is to be called by __pfn_to_mfn() only in special
// cases needing an extended handling.
//
extern "C" {
    pub fn get_phys_to_machine(_arg: pfn) -> return;
}
extern "C" {
    pub fn IDENTITY_FRAME(_arg: pfn) -> return;
}
extern "C" {
    pub fn get_phys_to_machine(_arg: pfn) -> return;
}

//
// Some x86 code are still using pfn_to_mfn instead of
// pfn_to_mfn. This will have to be removed when we figured
// out which call.
//
// The array access can fail (e.g., device space beyond end of RAM).
// In such cases it doesn't matter what we return (we return garbage),
// but we must handle the fault without crashing!
//
// Some x86 code are still using mfn_to_pfn instead of
// gfn_to_pfn. This will have to be removed when we figure
// out which call.
//
// pfn is ~0 if there are no entries in the m2p for mfn or the
// entry doesn't map back to the mfn.
//
extern "C" {
    pub fn XMADDR(offset: PFN_PHYS(pfn_to_mfn(PFN_DOWN(phys.paddr))) |) -> return;
}
extern "C" {
    pub fn XPADDR(offset: PFN_PHYS(mfn_to_pfn(PFN_DOWN(machine.maddr))) |) -> return;
}
// Pseudo-physical <-> Guest conversion
extern "C" {
    pub fn pfn_to_mfn(_arg: pfn) -> return;
}
extern "C" {
    pub fn mfn_to_pfn(_arg: gfn) -> return;
}
// Pseudo-physical <-> Bus conversion

//
// We detect special mappings in one of two ways:
// 1. If the MFN is an I/O page then Xen will set the m2p entry
// to be outside our maximum possible pseudophys range.
// 2. If the MFN belongs to a different domain then we will certainly
// not have MFN in our p2m table. Conversely, if the page is ours,
// then we'll have p2m(m2p(MFN))==MFN.
// If we detect a special mapping then it doesn't have a 'struct page'.
// We force !pfn_valid() by returning an out-of-range pointer.
//
// NB. These checks require that, for any MFN that is not in our reservation,
// there is no PFN such that p2m(PFN) == MFN. Otherwise we can get confused if
// we are foreign-mapping the MFN, and the other domain as m2p(MFN) == PFN.
// Yikes! Various places must poke in INVALID_P2M_ENTRY for safety.
//
// NB2. When deliberately mapping foreign pages into the p2m table, you *must
// use FOREIGN_FRAME(). This will cause pte_pfn() to choke on it, as we
// require. In all the cases we care about, the FOREIGN_FRAME bit is
// masked (e.g., pfn_to_mfn()) so behaviour there is correct.
//
// VIRT <-> MACHINE conversion

extern "C" {
    pub fn PFN_DOWN(_arg: __pa(v)) -> return;
}

// VIRT <-> GUEST conversion

extern "C" {
    pub fn arbitrary_virt_to_machine(address: *mut c_void) -> xmaddr_t;
}
extern "C" {
    pub fn arbitrary_virt_to_mfn(vaddr: *mut c_void) -> c_ulong;
}
extern "C" {
    pub fn make_lowmem_page_readonly(vaddr: *mut c_void);
}
extern "C" {
    pub fn make_lowmem_page_readwrite(vaddr: *mut c_void);
}
