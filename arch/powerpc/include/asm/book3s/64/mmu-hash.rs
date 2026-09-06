//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/book3s/64/mmu-hash.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// PowerPC64 memory management structures
//
// Dave Engebretsen & Mike Corrigan <{engebret|mikejc}@us.ibm.com>
// PPC64 rework.
//

//
// This is necessary to get the definition of PGTABLE_RANGE which we
// need for various slices related matters. Note that this isn't the
// complete pgtable.h but only a portion of it.
//

//
// SLB
//
pub const SLB_NUM_BOLTED: c_int = 2;
pub const SLB_CACHE_ENTRIES: c_int = 8;
pub const SLB_MIN_SIZE: c_int = 32;
// Bits in the SLB ESID word

// Bits in the SLB VSID word
pub const SLB_VSID_SHIFT: c_int = 12;

pub const SLB_VSID_SHIFT_1T: c_int = 24;
pub const SLB_VSID_SSIZE_SHIFT: c_int = 62;

pub const SLBIE_SSIZE_SHIFT: c_int = 25;
//
// Hash table
//
pub const HPTES_PER_GROUP: c_int = 8;
pub const HPTE_V_SSIZE_SHIFT: c_int = 62;
pub const HPTE_V_AVPN_SHIFT: c_int = 7;

//
// ISA 3.0 has a different HPTE format.
//
pub const HPTE_R_3_0_SSIZE_SHIFT: c_int = 58;

pub const HPTE_R_RPN_SHIFT: c_int = 12;

// Values for PP (assumes Ks=0, Kp=1)

// Fields for tlbiel instruction in architecture 2.06
pub const TLBIEL_INVAL_SEL_MASK: c_uint = 0xc00	/* invalidation selector */;
pub const TLBIEL_INVAL_PAGE: c_uint = 0x000	/* invalidate a single page */;
pub const TLBIEL_INVAL_SET_LPID: c_uint = 0x800	/* invalidate a set for current LPID */;
pub const TLBIEL_INVAL_SET: c_uint = 0xc00	/* invalidate a set for all LPIDs */;
pub const TLBIEL_INVAL_SET_MASK: c_uint = 0xfff000	/* set number to inval. */;
pub const TLBIEL_INVAL_SET_SHIFT: c_int = 12;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmu_hash_ops {
    pub local): int ssize, int,
    pub flags): int ssize, unsigned long,
    pub ssize): int psize, int,
    pub ssize): c_int,
    pub hpte_group): *mut *mut long (hpte_remove)(unsigned long,
    pub ssize): int psize, int,
    pub local): *mut *mut void (flush_hash_range)(unsigned long number, int,
    pub local): int psize, int ssize, int,
    pub shift): *mut *mut int (resize_hpt)(unsigned long,
//
// Special for kexec.
// To be called in real mode with interrupts disabled. No locks are
// taken as such, concurrent access on pre POWER5 hardware could result
// in a deadlock.
// The linear mapping is destroyed as well.
//
    pub (*hpte_clear_all)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_pte {
    pub v: __be64,
    pub r: __be64,
}

//
// Segment sizes.
// These are the values used by hardware in the B field of
// SLB entries and the first dword of MMU hashtable entries.
// The B field is 2 bits; the values 2 and 3 are unused and reserved.
//
pub const MMU_SEGSIZE_256M: c_int = 0;
pub const MMU_SEGSIZE_1T: c_int = 1;
//
// encode page number shift.
// in order to fit the 78 bit va in a 64 bit variable we shift the va by
// 12 bits. This enable us to address upto 76 bit va.
// For hpt hash from a va we can ignore the page size bits of va and for
// hpte encoding we ignore up to 23 bits of va. So ignoring lower 12 bits ensure
// we work in all cases including 4k page size.
//
pub const VPN_SHIFT: c_int = 12;
//
// HPTE Large Page (LP) details
//
pub const LP_SHIFT: c_int = 12;
pub const LP_BITS: c_int = 8;

//
// This array is indexed by the LP field of the HPTE second dword.
// Since this field may contain some RPN bits, some entries are
// replicated so that we get the same value irrespective of RPN.
// The top 4 bits are the page size index (MMU_PAGE_*) for the
// actual page size, the bottom 4 bits are the base page size.
//
// Look at the 8 bit LP value
extern "C" {
    pub fn __hpte_page_size(_arg: h, _arg: l, _arg: 0) -> return;
}
extern "C" {
    pub fn __hpte_page_size(_arg: h, _arg: l, _arg: 1) -> return;
}
//
// The current system page and segment sizes
//
// If the processor supports 64k normal pages but not 64k cache
// inhibited pages, we have to be prepared to switch processes
// to use 4k pages when they create cache-inhibited mappings.
// If this is the case, mmu_ci_restrictions will be set to 1.
//
// This computes the AVPN and B fields of the first dword of a HPTE,
// for use when we want to match an existing PTE.  The bottom 7 bits
// of the returned value are zero.
//
// The AVA field omits the low-order 23 bits of the 78 bits VA.
// These bits are not needed in the PTE, because the
// low-order b of these bits are part of the byte offset
// into the virtual page and, if b < 23, the high-order
// 23-b of these bits are always used in selecting the
// PTEGs to be searched
//
// ISA v3.0 defines a new HPTE format, which differs from the old
// format in having smaller AVPN and ARPN fields, and the B field
// in the second dword instead of the first.
//
// trim AVPN, drop B
// move B field from 1st to 2nd dword, trim ARPN
// insert B field
// clear out B field
//
// This function sets the AVPN and L fields of the HPTE  appropriately
// using the base page size and actual page size.
//
// This function sets the ARPN, and LP fields of the HPTE appropriately
// for the page size. We assume the pa is already "clean" that is properly
// aligned for the requested page size
//
// A 4K page needs no special encoding
//
// Build a VPN_SHIFT bit shifted va given VSID, EA and segment size.
//
// This hashes a virtual address
//
// VPN_SHIFT can be atmost 12
pub const HPTE_LOCAL_UPDATE: c_uint = 0x1;
pub const HPTE_NOHPTE_UPDATE: c_uint = 0x2;
pub const HPTE_USE_KERNEL_KEY: c_uint = 0x4;
extern "C" {
    pub fn hash_page_do_lazy_icache(pp: c_uint, pte: pte_t, trap: c_int) -> c_uint;
}
extern "C" {
    pub fn low_hash_fault(regs: *mut pt_regs, address: c_ulong, rc: c_int);
}
extern "C" {
    pub fn __hash_page(trap: c_ulong, ea: c_ulong, dsisr: c_ulong, msr: c_ulong) -> c_int;
}

extern "C" {
    pub fn pseries_add_gpage(addr: u64, page_size: u64, number_of_pages: c_ulong);
}
extern "C" {
    pub fn demote_segment_4k(mm: *mut mm_struct, addr: c_ulong);
}
extern "C" {
    pub fn hash__setup_new_exec();
}

extern "C" {
    pub fn hpte_init_pseries();
}

extern "C" {
    pub fn hpte_init_native();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slb_entry {
    pub esid: u64,
    pub vsid: u64,
}

extern "C" {
    pub fn slb_initialize();
}
extern "C" {
    pub fn slb_flush_and_restore_bolted();
}
extern "C" {
    pub fn slb_flush_all_realmode();
}
extern "C" {
    pub fn __slb_restore_bolted_realmode();
}
extern "C" {
    pub fn slb_restore_bolted_realmode();
}
extern "C" {
    pub fn slb_save_contents(slb_ptr: *mut slb_entry);
}
extern "C" {
    pub fn slb_dump_contents(slb_ptr: *mut slb_entry);
}
extern "C" {
    pub fn slb_vmalloc_update();
}

extern "C" {
    pub fn slb_set_size(size: u16);
}

//
// VSID allocation (256MB segment)
//
// We first generate a 37-bit "proto-VSID". Proto-VSIDs are generated
// from mmu context id and effective segment id of the address.
//
// For user processes max context id is limited to MAX_USER_CONTEXT.
// more details in get_user_context
//
// For kernel space get_kernel_context
//
// The proto-VSIDs are then scrambled into real VSIDs with the
// multiplicative hash:
//
// VSID = (proto-VSID * VSID_MULTIPLIER) % VSID_MODULUS
//
// VSID_MULTIPLIER is prime, so in particular it is
// co-prime to VSID_MODULUS, making this a 1:1 scrambling function.
// Because the modulus is 2^n-1 we can compute it efficiently without
// a divide or extra multiply (see below). The scramble function gives
// robust scattering in the hash table (at least based on some initial
// results).
//
// We use VSID 0 to indicate an invalid VSID. The means we can't use context id
// 0, because a context id of 0 and an EA of 0 gives a proto-VSID of 0, which
// will produce a VSID of 0.
//
// We also need to avoid the last segment of the last context, because that
// would give a protovsid of 0x1fffffffff. That will result in a VSID 0
// because of the modulo operation in vsid scramble.
//
// Max Va bits we support as of now is 68 bits. We want 19 bit
// context ID.
// Restrictions:
// GPU has restrictions of not able to access beyond 128TB
// (47 bit effective address). We also cannot do more than 20bit PID.
// For p4 and p5 which can only do 65 bit VA, we restrict our CONTEXT_BITS
// to 16 bits (ie, we can only have 2^16 pids at the same time).
//
pub const VA_BITS: c_int = 68;
pub const CONTEXT_BITS: c_int = 19;

//
// Now certain config support MAX_PHYSMEM more than 512TB. Hence we will need
// to use more than one context for linear mapping the kernel.
// For vmalloc and memmap, we use just one context with 512TB. With 64 byte
// struct page size, we need ony 32 TB in memmap for 2PB (51 bits (MAX_PHYSMEM_BITS)).
//

pub const MAX_KERNEL_CTX_CNT: c_int = 1;

pub const MAX_VMALLOC_CTX_CNT: c_int = 1;
pub const MAX_IO_CTX_CNT: c_int = 1;
pub const MAX_VMEMMAP_CTX_CNT: c_int = 1;
//
// 256MB segment
// The proto-VSID space has 2^(CONTEX_BITS + ESID_BITS) - 1 segments
// available for user + kernel mapping. VSID 0 is reserved as invalid, contexts
// 1-4 are used for kernel mapping. Each segment contains 2^28 bytes. Each
// context maps 2^49 bytes (512TB).
//
// We also need to avoid the last segment of the last context, because that
// would give a protovsid of 0x1fffffffff. That will result in a VSID 0
// because of the modulo operation in vsid scramble.
//

// The + 2 accounts for INVALID_REGION and 1 more to avoid overlap with kernel

//
// For platforms that support on 65bit VA we limit the context bits
//

//
// This should be computed such that protovosid * vsid_mulitplier
// doesn't overflow 64 bits. The vsid_mutliplier should also be
// co-prime to vsid_modulus. We also need to make sure that number
// of bits in multiplied result (dividend) is less than twice the number of
// protovsid bits for our modulus optmization to work.
//
// The below table shows the current values used.
// |-------+------------+----------------------+------------+-------------------|
// |       | Prime Bits | proto VSID_BITS_65VA | Total Bits | 2* prot VSID_BITS |
// |-------+------------+----------------------+------------+-------------------|
// | 1T    |         24 |                   25 |         49 |                50 |
// |-------+------------+----------------------+------------+-------------------|
// | 256MB |         24 |                   37 |         61 |                74 |
// |-------+------------+----------------------+------------+-------------------|
//
// |-------+------------+----------------------+------------+--------------------|
// |       | Prime Bits | proto VSID_BITS_68VA | Total Bits | 2* proto VSID_BITS |
// |-------+------------+----------------------+------------+--------------------|
// | 1T    |         24 |                   28 |         52 |                 56 |
// |-------+------------+----------------------+------------+--------------------|
// | 256MB |         24 |                   40 |         64 |                 80 |
// |-------+------------+----------------------+------------+--------------------|
//

//
// Modular multiplicative inverse of VSID_MULTIPLIER under modulo VSID_MODULUS
//

// 1TB VSID reserved for VRMA
pub const VRMA_VSID: c_uint = 0x1ffffffUL;

// 4 bits per slice and we have one slice per 1TB

//
// For the sub-page protection option, we extend the PGD with one of
// these.  Basically we have a 3-level tree, with the top level being
// the protptrs array.  To optimize speed and memory consumption when
// only addresses < 4GB are being protected, pointers to the first
// four pages of sub-page protection words are stored in the low_prot
// array.
// Each page of sub-page protection words protects 1GB (4 bytes
// protects 64k).  For the 3-level tree, each page of pointers then
// protects 8TB.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subpage_prot_table {
    pub /: *mut *mut unsigned long maxaddr; / only addresses < this are protected,
    pub 43)]: *mut *mut *mut unsigned int protptrs[(TASK_SIZE_USER64 >>,
    pub low_prot: [*mut c_uint; 4],
}

extern "C" {
    pub fn subpage_prot_free(mm: *mut mm_struct);
}

//
// One bit per slice. We have lower slices which cover 256MB segments
// upto 4G range. That gets us 16 low slices. For the rest we track slices
// in 1TB size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slice_mask {
    pub low_slices: u64,
    pub SLICE_NUM_HIGH): DECLARE_BITMAP(high_slices,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hash_mm_context {
    pub /: *mut *mut u16 user_psize; / page size index,
// SLB page size encodings
    pub low_slices_psize: [c_uchar; LOW_SLICE_ARRAY_SZ],
    pub high_slices_psize: [c_uchar; SLICE_ARRAY_SIZE],
    pub slb_addr_limit: c_ulong,

    pub mask_64k: slice_mask,

    pub mask_4k: slice_mask,

    pub mask_16m: slice_mask,
    pub mask_16g: slice_mask,

    pub spt: *mut subpage_prot_table,

}

//
// The code below is equivalent to this function for arguments
// < 2^VSID_BITS, which is all this should ever be called
// with.  However gcc is not clever enough to compute the
// modulus (2^n-1) without a second multiply.
//

// simplified form avoiding mod operation

//
// We have same multipler for both 256 and 1T segements now
//

// Returns the segment size indicator for a user address
// Use 1T segments if possible for addresses >= 1T
//
// Bad address. We return VSID 0 for that
//
extern "C" {
    pub fn vsid_scramble(_arg: protovsid, _arg: VSID_MULTIPLIER_256M, _arg: vsid_bits) -> return;
}
// 1T segment
extern "C" {
    pub fn vsid_scramble(_arg: protovsid, _arg: VSID_MULTIPLIER_1T, _arg: vsid_bits) -> return;
}
//
// For kernel space, we use context ids as
// below. Range is 512TB per context.
//
// 0x00001 -  [ 0xc000000000000000 - 0xc001ffffffffffff]
// 0x00002 -  [ 0xc002000000000000 - 0xc003ffffffffffff]
// 0x00003 -  [ 0xc004000000000000 - 0xc005ffffffffffff]
// 0x00004 -  [ 0xc006000000000000 - 0xc007ffffffffffff]
//
// vmap, IO, vmemap
//
// 0x00005 -  [ 0xc008000000000000 - 0xc009ffffffffffff]
// 0x00006 -  [ 0xc00a000000000000 - 0xc00bffffffffffff]
// 0x00007 -  [ 0xc00c000000000000 - 0xc00dffffffffffff]
//
// Depending on Kernel config, kernel region can have one context
// or more.
//
// We already verified ea to be not beyond the addr limit.
//
// This is only valid for addresses >= PAGE_OFFSET
//
extern "C" {
    pub fn get_vsid(_arg: context, _arg: ea, _arg: ssize) -> return;
}
extern "C" {
    pub fn htab_shift_for_mem_size(mem_size: c_ulong) -> unsigned;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slb_index {
    LINEAR_INDEX	= 0, /* Kernel linear map  (0xc000000000000000) */
    KSTACK_INDEX	= 1, /* Kernel stack map */
}

extern "C" {
    pub fn __mk_vsid_data(_arg: get_kernel_vsid(ea, _arg: ssize), _arg: ssize, _arg: flags) -> return;
}

