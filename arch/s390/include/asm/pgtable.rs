//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/pgtable.h
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
// S390 version
// Copyright IBM Corp. 1999, 2000
// Author(s): Hartmut Penner (hp@de.ibm.com)
// Ulrich Weigand (weigand@de.ibm.com)
// Martin Schwidefsky (schwidefsky@de.ibm.com)
//
// Derived from "include/asm-i386/pgtable.h"
//

extern "C" {
    pub fn paging_init();
}
//
// The S390 doesn't have any external MMU info: the kernel page
// tables contain all the necessary information.
//

//
// ZERO_PAGE is a global shared page that is always zero; used
// for zero-mapped memory areas etc..
//

// TODO: s390 cannot support io_remap_pfn_range...

//
// The vmalloc and module area will always be on the topmost area of the
// kernel mapping. 512GB are reserved for vmalloc by default.
// At the top of the vmalloc area a 2GB area is reserved where modules
// will reside. That makes sure that inter module branches always
// happen without trampolines and in addition the placement within a
// 2GB frame is branch prediction unit friendly.
//

extern "C" {
    pub fn setup_protection_map();
}
//
// A 64 bit pagetable entry of S390 has following format:
// |			 PFRA			      |0IPC|  OS  |
// 0000000000111111111122222222223333333333444444444455555555556666
// 0123456789012345678901234567890123456789012345678901234567890123
//
// I Page-Invalid Bit:    Page is not available for address-translation
// P Page-Protection Bit: Store access not possible for page
// C Change-bit override: HW is not required to set change bit
//
// A 64 bit segmenttable entry of S390 has following format:
// |        P-table origin                              |      TT
// 0000000000111111111122222222223333333333444444444455555555556666
// 0123456789012345678901234567890123456789012345678901234567890123
//
// I Segment-Invalid Bit:    Segment is not available for address-translation
// C Common-Segment Bit:     Segment is not private (PoP 3-30)
// P Page-Protection Bit: Store access not possible for page
// TT Type 00
//
// A 64 bit region table entry of S390 has following format:
// |        S-table origin                             |   TF  TTTL
// 0000000000111111111122222222223333333333444444444455555555556666
// 0123456789012345678901234567890123456789012345678901234567890123
//
// I Segment-Invalid Bit:    Segment is not available for address-translation
// TT Type 01
// TF
// TL Table length
//
// The 64 bit regiontable origin of S390 has following format:
// |      region table origon                          |       DTTL
// 0000000000111111111122222222223333333333444444444455555555556666
// 0123456789012345678901234567890123456789012345678901234567890123
//
// X Space-Switch event:
// G Segment-Invalid Bit:
// P Private-Space Bit:
// S Storage-Alteration:
// R Real space
// TL Table-Length:
//
// A storage key has the following format:
// | ACC |F|R|C|0|
// 0   3 4 5 6 7
// ACC: access key
// F  : fetch protection bit
// R  : referenced bit
// C  : changed bit
//
// Hardware bits in the page table entry
pub const _PAGE_NOEXEC: c_uint = 0x100		/* HW no-execute bit  */;
pub const _PAGE_PROTECT: c_uint = 0x200		/* HW read-only bit  */;
pub const _PAGE_INVALID: c_uint = 0x400		/* HW invalid bit    */;
pub const _PAGE_LARGE: c_uint = 0x800		/* Bit to mark a large pte */;
// Software bits in the page table entry
pub const _PAGE_PRESENT: c_uint = 0x001		/* SW pte present bit */;
pub const _PAGE_YOUNG: c_uint = 0x004		/* SW pte young bit */;
pub const _PAGE_DIRTY: c_uint = 0x008		/* SW pte dirty bit */;
pub const _PAGE_READ: c_uint = 0x010		/* SW pte read bit */;
pub const _PAGE_WRITE: c_uint = 0x020		/* SW pte write bit */;
pub const _PAGE_SPECIAL: c_uint = 0x040		/* SW associated with special page */;
pub const _PAGE_UNUSED: c_uint = 0x080		/* SW bit for pgste usage state */;

pub const _PAGE_SOFT_DIRTY: c_uint = 0x002		/* SW pte soft dirty bit */;

pub const _PAGE_SOFT_DIRTY: c_uint = 0x000;

pub const _PAGE_SW_BITS: c_uint = 0xffUL		/* All SW bits */;

// Set of bits not changed in pte_modify

//
// Mask of bits that must not be changed with RDP. Allow only _PAGE_PROTECT
// HW bit and all SW bits.
//

//
// handle_pte_fault uses pte_present and pte_none to find out the pte type
// WITHOUT holding the page table lock. The _PAGE_PRESENT bit is used to
// distinguish present from not-present ptes. It is changed only with the page
// table lock held.
//
// The following table gives the different possible bit combinations for
// the pte hardware and software bits in the last 12 bits of a pte
// (. unassigned bit, x don't care, t swap type):
//
// 842100000000
// 000084210000
// 000000008421
// .IR.uswrdy.p
// empty			.10.00000000
// swap				.11..ttttt.0
// prot-none, clean, old	.11.xx0000.1
// prot-none, clean, young	.11.xx0001.1
// prot-none, dirty, old	.11.xx0010.1
// prot-none, dirty, young	.11.xx0011.1
// read-only, clean, old	.11.xx0100.1
// read-only, clean, young	.01.xx0101.1
// read-only, dirty, old	.11.xx0110.1
// read-only, dirty, young	.01.xx0111.1
// read-write, clean, old	.11.xx1100.1
// read-write, clean, young	.01.xx1101.1
// read-write, dirty, old	.10.xx1110.1
// read-write, dirty, young	.00.xx1111.1
// HW-bits: R read-only, I invalid
// SW-bits: p present, y young, d dirty, r read, w write, s special,
// u unused, l large
//
// pte_none    is true for the bit pattern .10.00000000, pte == 0x400
// pte_swap    is true for the bit pattern .11..ooooo.0, (pte & 0x201) == 0x200
// pte_present is true for the bit pattern .xx.xxxxxx.1, (pte & 0x001) == 0x001
//
// Bits in the segment/region table address-space-control-element

pub const _ASCE_PRIVATE_SPACE: c_uint = 0x100	/* private space control	    */;
pub const _ASCE_ALT_EVENT: c_uint = 0x80	/* storage alteration event control */;
pub const _ASCE_SPACE_SWITCH: c_uint = 0x40	/* space switch event		    */;
pub const _ASCE_REAL_SPACE: c_uint = 0x20	/* real space control		    */;
pub const _ASCE_TYPE_MASK: c_uint = 0x0c	/* asce table type mask		    */;
pub const _ASCE_TYPE_REGION1: c_uint = 0x0c	/* region first table type	    */;
pub const _ASCE_TYPE_REGION2: c_uint = 0x08	/* region second table type	    */;
pub const _ASCE_TYPE_REGION3: c_uint = 0x04	/* region third table type	    */;
pub const _ASCE_TYPE_SEGMENT: c_uint = 0x00	/* segment table type		    */;
pub const _ASCE_TABLE_LENGTH: c_uint = 0x03	/* region table length		    */;
// Bits in the region table entry

pub const _REGION_ENTRY_PROTECT: c_uint = 0x200	/* region protection bit	    */;
pub const _REGION_ENTRY_NOEXEC: c_uint = 0x100	/* region no-execute bit	    */;
pub const _REGION_ENTRY_OFFSET: c_uint = 0xc0	/* region table offset		    */;
pub const _REGION_ENTRY_INVALID: c_uint = 0x20	/* invalid region table entry	    */;
pub const _REGION_ENTRY_TYPE_MASK: c_uint = 0x0c	/* region table type mask	    */;
pub const _REGION_ENTRY_TYPE_R1: c_uint = 0x0c	/* region first table type	    */;
pub const _REGION_ENTRY_TYPE_R2: c_uint = 0x08	/* region second table type	    */;
pub const _REGION_ENTRY_TYPE_R3: c_uint = 0x04	/* region third table type	    */;
pub const _REGION_ENTRY_LENGTH: c_uint = 0x03	/* region third length		    */;

pub const _REGION3_ENTRY_HARDWARE_BITS: c_uint = 0xfffffffffffff6ffUL;
pub const _REGION3_ENTRY_HARDWARE_BITS_LARGE: c_uint = 0xffffffff8001073cUL;

pub const _REGION3_ENTRY_DIRTY: c_uint = 0x2000	/* SW region dirty bit */;
pub const _REGION3_ENTRY_YOUNG: c_uint = 0x1000	/* SW region young bit */;
pub const _REGION3_ENTRY_COMM: c_uint = 0x0010	/* Common-Region, marks swap entry */;
pub const _REGION3_ENTRY_LARGE: c_uint = 0x0400	/* RTTE-format control, large page  */;
pub const _REGION3_ENTRY_WRITE: c_uint = 0x8000	/* SW region write bit */;
pub const _REGION3_ENTRY_READ: c_uint = 0x4000	/* SW region read bit */;

pub const _REGION3_ENTRY_SOFT_DIRTY: c_uint = 0x0002 /* SW region soft dirty bit */;

pub const _REGION3_ENTRY_SOFT_DIRTY: c_uint = 0x0000 /* SW region soft dirty bit */;

pub const _REGION_ENTRY_BITS: c_uint = 0xfffffffffffff22fUL;
//
// SW region present bit. For non-leaf region-third-table entries, bits 62-63
// indicate the TABLE LENGTH and both must be set to 1. But such entries
// would always be considered as present, so it is safe to use bit 63 as
// PRESENT bit for PUD.
//
pub const _REGION3_ENTRY_PRESENT: c_uint = 0x0001;
// Bits in the segment table entry
pub const _SEGMENT_ENTRY_BITS: c_uint = 0xfffffffffffffe3fUL;
pub const _SEGMENT_ENTRY_HARDWARE_BITS: c_uint = 0xfffffffffffffe3cUL;
pub const _SEGMENT_ENTRY_HARDWARE_BITS_LARGE: c_uint = 0xfffffffffff1073cUL;

pub const _SEGMENT_ENTRY_PROTECT: c_uint = 0x200	/* segment protection bit	    */;
pub const _SEGMENT_ENTRY_NOEXEC: c_uint = 0x100	/* segment no-execute bit	    */;
pub const _SEGMENT_ENTRY_INVALID: c_uint = 0x20	/* invalid segment table entry	    */;
pub const _SEGMENT_ENTRY_TYPE_MASK: c_uint = 0x0c	/* segment table type mask	    */;

pub const _SEGMENT_ENTRY_DIRTY: c_uint = 0x2000	/* SW segment dirty bit */;
pub const _SEGMENT_ENTRY_YOUNG: c_uint = 0x1000	/* SW segment young bit */;
pub const _SEGMENT_ENTRY_COMM: c_uint = 0x0010	/* Common-Segment, marks swap entry */;
pub const _SEGMENT_ENTRY_LARGE: c_uint = 0x0400	/* STE-format control, large page */;
pub const _SEGMENT_ENTRY_WRITE: c_uint = 0x8000	/* SW segment write bit */;
pub const _SEGMENT_ENTRY_READ: c_uint = 0x4000	/* SW segment read bit */;

pub const _SEGMENT_ENTRY_SOFT_DIRTY: c_uint = 0x0002 /* SW segment soft dirty bit */;

pub const _SEGMENT_ENTRY_SOFT_DIRTY: c_uint = 0x0000 /* SW segment soft dirty bit */;

pub const _SEGMENT_ENTRY_PRESENT: c_uint = 0x0001	/* SW segment present bit */;
// Common bits in region and segment table entries, for swap entries
pub const _RST_ENTRY_COMM: c_uint = 0x0010	/* Common-Region/Segment, marks swap entry */;
pub const _RST_ENTRY_INVALID: c_uint = 0x0020	/* invalid region/segment table entry */;

pub const _REGION1_SHIFT: c_int = 53;
pub const _REGION2_SHIFT: c_int = 42;
pub const _REGION3_SHIFT: c_int = 31;
pub const _SEGMENT_SHIFT: c_int = 20;

//
// Segment table and region3 table entry encoding
// (R = read-only, I = invalid, y = young bit):
// dy..R...I...wr
// prot-none, clean, old	00..1...1...00
// prot-none, clean, young	01..1...1...00
// prot-none, dirty, old	10..1...1...00
// prot-none, dirty, young	11..1...1...00
// read-only, clean, old	00..1...1...01
// read-only, clean, young	01..1...0...01
// read-only, dirty, old	10..1...1...01
// read-only, dirty, young	11..1...0...01
// read-write, clean, old	00..1...1...11
// read-write, clean, young	01..1...0...11
// read-write, dirty, old	10..0...1...11
// read-write, dirty, young	11..0...0...11
// The segment table origin is used to distinguish empty (origin==0) from
// read-write, old segment table entries (origin!=0)
// HW-bits: R read-only, I invalid
// SW-bits: y young, d dirty, r read, w write
//
// A user page table pointer has the space-switch-event bit, the
// private-space-control bit and the storage-alteration-event-control
// bit set. A kernel page table pointer doesn't need them.
//

//
// Page protection definitions.
//

//
// Segment entry (large page) protection definitions.
//

//
// Region3 entry (large page) protection definitions.
//

extern "C" {
    pub fn __pte(~pgprot_val(prot): pte_val(pte) &) -> return;
}
extern "C" {
    pub fn __pte(pgprot_val(prot): pte_val(pte) |) -> return;
}
extern "C" {
    pub fn __pmd(~pgprot_val(prot): pmd_val(pmd) &) -> return;
}
extern "C" {
    pub fn __pmd(pgprot_val(prot): pmd_val(pmd) |) -> return;
}
extern "C" {
    pub fn __pud(~pgprot_val(prot): pud_val(pud) &) -> return;
}
extern "C" {
    pub fn __pud(pgprot_val(prot): pud_val(pud) |) -> return;
}
//
// As soon as the guest uses storage keys or enables PV, we deduplicate all
// mapped shared zeropages and prevent new shared zeropages from getting
// mapped.
//

//
// cspg() - Compare and Swap and Purge (CSPG)
// @ptr: Pointer to the value to be exchanged
// @old: The expected old value
// @new: The new value
//
// Return: True if compare and swap was successful, otherwise false.
//
pub const CRDTE_DTT_PAGE: c_uint = 0x00UL;
pub const CRDTE_DTT_SEGMENT: c_uint = 0x10UL;
pub const CRDTE_DTT_REGION3: c_uint = 0x14UL;
pub const CRDTE_DTT_REGION2: c_uint = 0x18UL;
pub const CRDTE_DTT_REGION1: c_uint = 0x1cUL;
//
// crdte() - Compare and Replace DAT Table Entry
// @old:     The expected old value
// @new:     The new value
// @table:   Pointer to the value to be exchanged
// @dtt:     Table type of the table to be exchanged
// @address: The address mapped by the entry to be replaced
// @asce:    The ASCE of this entry
//
// Return: True if compare and replace was successful, otherwise false.
//
// pgd/p4d/pud/pmd/pte query functions
//

// Bit pattern: (pte & 0x001) == 0x001
// Bit pattern: pte == 0x400
// Bit pattern: (pte & 0x201) == 0x200
extern "C" {
    pub fn pte_val(pte_val(b: a) ==) -> return;
}

extern "C" {
    pub fn pte_present(_PAGE_READ: pte) && !(pte_val(pte) &) -> return;
}
// pmd_leaf(pmd) implies pmd_present(pmd)
extern "C" {
    pub fn pmd_leaf(_SEGMENT_ENTRY_READ: pmd) && !(pmd_val(pmd) &) -> return;
}

extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_SWP_EXCLUSIVE)) -> return;
}
extern "C" {
    pub fn clear_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_SWP_EXCLUSIVE)) -> return;
}

extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_SOFT_DIRTY)) -> return;
}

extern "C" {
    pub fn clear_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_SOFT_DIRTY)) -> return;
}

extern "C" {
    pub fn set_pmd_bit(_arg: pmd, _arg: __pgprot(_SEGMENT_ENTRY_SOFT_DIRTY)) -> return;
}
extern "C" {
    pub fn clear_pmd_bit(_arg: pmd, _arg: __pgprot(_SEGMENT_ENTRY_SOFT_DIRTY)) -> return;
}

//
// query functions pte_write/pte_dirty/pte_young only work if
// pte_present() is true. Undefined behaviour if not..
//
// Extract the pgprot value from the given pte while at the same time making it
// usable for kernel address space mappings where fault driven dirty and
// young/old accounting is not supported, i.e _PAGE_PROTECT and _PAGE_INVALID
// must not be set.
//

extern "C" {
    pub fn __pgprot(_arg: pte_flags) -> return;
}
//
// pgd/pmd/pte modification functions
//

extern "C" {
    pub fn READ_ONCE(_arg: *mut ptep) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut pmdp) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut pudp) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut p4dp) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: *mut pgdp) -> return;
}
//
// The following pte modification functions only work if
// pte_present() is true. Undefined behaviour if not..
//
// newprot for PAGE_NONE, PAGE_RO, PAGE_RX, PAGE_RW and PAGE_RWX
// has the invalid bit set, clear it again for readable, young pages
//
// newprot for PAGE_RO, PAGE_RX, PAGE_RW and PAGE_RWX has the page
// protection bit set, clear it again for writable, dirty pages
//
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_PROTECT)) -> return;
}
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_PROTECT)) -> return;
}
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_INVALID)) -> return;
}
extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_SPECIAL)) -> return;
}

extern "C" {
    pub fn set_pte_bit(_arg: pte, _arg: __pgprot(_PAGE_LARGE)) -> return;
}

pub const IPTE_GLOBAL: c_int = 0;
pub const IPTE_LOCAL: c_int = 1;
pub const IPTE_NODAT: c_uint = 0x400;
pub const IPTE_GUEST_ASCE: c_uint = 0x800;
// Invalidation + TLB flush for the pte
// Invalidate ptes with options + TLB flush of the ptes
// Invalidate a range of ptes + TLB flush of the ptes
//
// This is hard to understand. ptep_get_and_clear and ptep_clear_flush
// both clear the TLB for the unmapped pte. The reason is that
// ptep_get_and_clear is used in common code (e.g. change_pte_range)
// to modify an active pte. The sequence is
// 1) ptep_get_and_clear
// 2) set_pte_at
// 3) flush_tlb_range
// On s390 the tlb needs to get flushed with the modification of the pte
// if the pte is active. The only way how this can be implemented is to
// have ptep_get_and_clear do the tlb flush. In exchange flush_tlb_range
// is a nop.
//
extern "C" {
    pub fn ptep_xchg_direct(: *mut mm_struct, long: unsigned, : *mut pte_t, _arg: pte_t) -> pte_t;
}
extern "C" {
    pub fn ptep_xchg_lazy(: *mut mm_struct, long: unsigned, : *mut pte_t, _arg: pte_t) -> pte_t;
}
extern "C" {
    pub fn pte_young(_arg: pte) -> return;
}
extern "C" {
    pub fn ptep_test_and_clear_young(_arg: vma, _arg: address, _arg: ptep) -> return;
}
// At this point the reference through the mapping is still present
extern "C" {
    pub fn ptep_modify_prot_start(: *mut vm_area_struct, long: unsigned, : *mut pte_t) -> pte_t;
}
// At this point the reference through the mapping is still present
//
// The batched pte unmap code uses ptep_get_and_clear_full to clear the
// ptes. Here an optimization is possible. tlb_gather_mmu flushes all
// tlbs of an mm if it can guarantee that the ptes of the mm_struct
// cannot be accessed while the batched unmap is running. In this case
// full==1 and a simple pte_clear is enough. See tlb.h.
//
// At this point the reference through the mapping is still present
//
// The notifier should have destroyed all protected vCPUs at
// this point, so the destroy should be successful.
//
// If something went wrong and the page could not be destroyed,
// or if this is not a mm teardown, the slower export is used
// as fallback instead. If even that fails, print a warning and
// leak the page, to avoid crashing the whole system.
//
// Check if PTEs only differ in _PAGE_PROTECT HW bit, but also allow SW PTE
// bits in the comparison. Those might change e.g. because of dirty and young
// tracking.
//
// Only allow changes from RO to RW
//
// RDP might not have propagated the PTE protection reset to all CPUs,
// so there could be spurious TLB protection faults.
// NOTE: This will also be called when a racing pagetable update on
// another thread already installed the correct PTE. Both cases cannot
// really be distinguished.
// Therefore, only do the local TLB flush when RDP can be used, and the
// PTE does not have _PAGE_PROTECT set, to avoid unnecessary overhead.
// A local RDP can be used to do the flush.
//

extern "C" {
    pub fn pgprot_writecombine(prot: pgprot_t) -> pgprot_t;
}

//
// Set multiple PTEs to consecutive pages with a single call.  All PTEs
// are within the same folio, PMD and VMA.
//

//
// Conversion functions: convert a page and protection to a page entry,
// and a page entry and page directory to the page they refer to.
//
extern "C" {
    pub fn pte_mkyoung(_arg: __pte) -> return;
}

//
// The pgd_offset function *always* adds the index for the top-level
// region/segment table. This is done to get a sequence like the
// following to work:
// pgdp = pgd_offset(current->mm, addr);
// pgd = READ_ONCE(*pgdp);
// p4dp = p4d_offset(&pgd, addr);
// ...
// The subsequent p4d_offset, pud_offset and pmd_offset functions
// only add an index if they dereferenced the pointer.
//
// Get the first entry of the top level table
// Pick up the shift from the table type of the first entry

extern "C" {
    pub fn p4d_offset_lockless(_arg: pgdp, _arg: *mut pgdp, _arg: address) -> return;
}

extern "C" {
    pub fn pud_offset_lockless(_arg: p4dp, _arg: *mut p4dp, _arg: address) -> return;
}

extern "C" {
    pub fn pmd_offset_lockless(_arg: pudp, _arg: *mut pudp, _arg: address) -> return;
}

extern "C" {
    pub fn set_pmd_bit(_arg: pmd, _arg: __pgprot(_SEGMENT_ENTRY_PROTECT)) -> return;
}
extern "C" {
    pub fn set_pmd_bit(_arg: pmd, _arg: __pgprot(_SEGMENT_ENTRY_PROTECT)) -> return;
}
extern "C" {
    pub fn set_pud_bit(_arg: pud, _arg: __pgprot(_REGION_ENTRY_PROTECT)) -> return;
}
extern "C" {
    pub fn set_pud_bit(_arg: pud, _arg: __pgprot(_REGION_ENTRY_PROTECT)) -> return;
}

//
// pgprot is PAGE_NONE, PAGE_RO, PAGE_RX, PAGE_RW or PAGE_RWX
// (see __Pxxx / __Sxxx). Convert to segment table entry format.
//
extern "C" {
    pub fn pgprot_val(_arg: SEGMENT_NONE) -> return;
}
extern "C" {
    pub fn pgprot_val(_arg: SEGMENT_RO) -> return;
}
extern "C" {
    pub fn pgprot_val(_arg: SEGMENT_RX) -> return;
}
extern "C" {
    pub fn pgprot_val(_arg: SEGMENT_RW) -> return;
}
extern "C" {
    pub fn pgprot_val(_arg: SEGMENT_RWX) -> return;
}
extern "C" {
    pub fn set_pmd_bit(_arg: pmd, _arg: __pgprot(_SEGMENT_ENTRY_INVALID)) -> return;
}
extern "C" {
    pub fn __pmd(massage_pgprot_pmd(pgprot): physpage +) -> return;
}

pub const IDTE_GLOBAL: c_int = 0;
pub const IDTE_LOCAL: c_int = 1;
pub const IDTE_PTOA: c_uint = 0x0800;
pub const IDTE_NODAT: c_uint = 0x1000;
pub const IDTE_GUEST_ASCE: c_uint = 0x2000;
// flush without guest asce
// flush with guest asce
// flush without guest asce
// flush with guest asce
extern "C" {
    pub fn pmdp_xchg_direct(: *mut mm_struct, long: unsigned, : *mut pmd_t, _arg: pmd_t) -> pmd_t;
}
extern "C" {
    pub fn pmdp_xchg_lazy(: *mut mm_struct, long: unsigned, : *mut pmd_t, _arg: pmd_t) -> pmd_t;
}
extern "C" {
    pub fn pudp_xchg_direct(: *mut mm_struct, long: unsigned, : *mut pud_t, _arg: pud_t) -> pud_t;
}

extern "C" {
    pub fn pgtable_trans_huge_withdraw(mm: *mut mm_struct, pmdp: *mut pmd_t) -> pgtable_t;
}
extern "C" {
    pub fn pmd_young(_arg: pmd) -> return;
}
extern "C" {
    pub fn pmdp_test_and_clear_young(_arg: vma, _arg: addr, _arg: pmdp) -> return;
}
extern "C" {
    pub fn set_pmd_bit(_arg: pmd, _arg: __pgprot(_SEGMENT_ENTRY_PROTECT)) -> return;
}
extern "C" {
    pub fn pmdp_huge_get_and_clear(_arg: vma->vm_mm, _arg: addr, _arg: pmdp) -> return;
}

extern "C" {
    pub fn pmdp_huge_get_and_clear(_arg: vma->vm_mm, _arg: address, _arg: pmdp) -> return;
}

extern "C" {
    pub fn pmd_leaf(_arg: pmd) -> return;
}

extern "C" {
    pub fn pte_present(_arg: pte) -> return;
}
extern "C" {
    pub fn pmd_leaf(_SEGMENT_ENTRY_READ: pmd) && (pmd_val(pmd) &) -> return;
}
extern "C" {
    pub fn pud_leaf(_arg: pud) -> return;
}

//
// 64 bit swap entry format:
// A page-table entry has some bits we have to treat in a special way.
// Bits 54 and 63 are used to indicate the page type. Bit 53 marks the pte
// as invalid.
// A swap pte is indicated by bit pattern (pte & 0x201) == 0x200
// |			  offset			|E11XX|type |S0|
// |0000000000111111111122222222223333333333444444444455|55555|55566|66|
// |0123456789012345678901234567890123456789012345678901|23456|78901|23|
//
// Bits 0-51 store the offset.
// Bit 52 (E) is used to remember PG_anon_exclusive.
// Bits 57-61 store the type.
// Bit 62 (S) is used for softdirty tracking.
// Bits 55 and 56 (X) are unused.
//

pub const __SWP_OFFSET_SHIFT: c_int = 12;

pub const __SWP_TYPE_SHIFT: c_int = 2;
extern "C" {
    pub fn __pte(_arg: pteval) -> return;
}

//
// 64 bit swap entry format for REGION3 and SEGMENT table entries (RSTE)
// Bits 59 and 63 are used to indicate the swap entry. Bit 58 marks the rste
// as invalid.
// A swap entry is indicated by bit pattern (rste & 0x011) == 0x010
// |			  offset			|Xtype |11TT|S0|
// |0000000000111111111122222222223333333333444444444455|555555|5566|66|
// |0123456789012345678901234567890123456789012345678901|234567|8901|23|
//
// Bits 0-51 store the offset.
// Bits 53-57 store the type.
// Bit 62 (S) is used for softdirty tracking.
// Bits 60-61 (TT) indicate the table type: 0x01 for REGION3 and 0x00 for SEGMENT.
// Bit 52 (X) is unused.
//

pub const __SWP_OFFSET_SHIFT_RSTE: c_int = 12;

pub const __SWP_TYPE_SHIFT_RSTE: c_int = 6;
//
// TT bits set to 0x00 == SEGMENT. For REGION3 entries, caller must add R3
// bits 0x01. See also __set_huge_pte_at().
//

//
// s390 has different layout for PTE and region / segment table entries (RSTE).
// This is also true for swap entries, and their swap type and offset encoding.
// For hugetlbfs PTE_MARKER support, s390 has internal __swp_type_rste() and
// __swp_offset_rste() helpers to correctly handle RSTE swap entries.
//
// But common swap code does not know about this difference, and only uses
// __swp_type(), __swp_offset() and __swp_entry() helpers for conversion between
// arch-dependent and arch-independent representation of swp_entry_t for all
// pagetable levels. On s390, those helpers only work for PTE swap entries.
//
// Therefore, implement __pmd_to_swp_entry() to build a fake PTE swap entry
// and return the arch-dependent representation of that. Correspondingly,
// implement __swp_entry_to_pmd() to convert that into a proper PMD swap
// entry again. With this, the arch-dependent swp_entry_t representation will
// always look like a PTE swap entry in common code.
//
// This is somewhat similar to fake PTEs in hugetlbfs code for s390, but only
// requires conversion of the swap type and offset, and not all the possible
// PTE bits.
//
extern "C" {
    pub fn __pte_to_swp_entry(_arg: pte) -> return;
}
extern "C" {
    pub fn vmem_add_mapping(start: c_ulong, size: c_ulong) -> c_int;
}
extern "C" {
    pub fn vmem_remove_mapping(start: c_ulong, size: c_ulong);
}
extern "C" {
    pub fn __vmem_map_4k_page(addr: c_ulong, phys: c_ulong, prot: pgprot_t, alloc: bool) -> c_int;
}
extern "C" {
    pub fn vmem_map_4k_page(addr: c_ulong, phys: c_ulong, prot: pgprot_t) -> c_int;
}
extern "C" {
    pub fn vmem_unmap_4k_page(addr: c_ulong);
}
// s390 has a private copy of get unmapped area to deal with cache synonyms
// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA
// Macro flag: #define HAVE_ARCH_UNMAPPED_AREA_TOPDOWN

