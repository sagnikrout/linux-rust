//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/nohash/mmu-e500.h
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
// Freescale Book-E/Book-3e (ISA 2.06+) MMU support
//
// Book-3e defined page sizes
pub const BOOK3E_PAGESZ_1K: c_int = 0;
pub const BOOK3E_PAGESZ_2K: c_int = 1;
pub const BOOK3E_PAGESZ_4K: c_int = 2;
pub const BOOK3E_PAGESZ_8K: c_int = 3;
pub const BOOK3E_PAGESZ_16K: c_int = 4;
pub const BOOK3E_PAGESZ_32K: c_int = 5;
pub const BOOK3E_PAGESZ_64K: c_int = 6;
pub const BOOK3E_PAGESZ_128K: c_int = 7;
pub const BOOK3E_PAGESZ_256K: c_int = 8;
pub const BOOK3E_PAGESZ_512K: c_int = 9;
pub const BOOK3E_PAGESZ_1M: c_int = 10;
pub const BOOK3E_PAGESZ_2M: c_int = 11;
pub const BOOK3E_PAGESZ_4M: c_int = 12;
pub const BOOK3E_PAGESZ_8M: c_int = 13;
pub const BOOK3E_PAGESZ_16M: c_int = 14;
pub const BOOK3E_PAGESZ_32M: c_int = 15;
pub const BOOK3E_PAGESZ_64M: c_int = 16;
pub const BOOK3E_PAGESZ_128M: c_int = 17;
pub const BOOK3E_PAGESZ_256M: c_int = 18;
pub const BOOK3E_PAGESZ_512M: c_int = 19;
pub const BOOK3E_PAGESZ_1GB: c_int = 20;
pub const BOOK3E_PAGESZ_2GB: c_int = 21;
pub const BOOK3E_PAGESZ_4GB: c_int = 22;
pub const BOOK3E_PAGESZ_8GB: c_int = 23;
pub const BOOK3E_PAGESZ_16GB: c_int = 24;
pub const BOOK3E_PAGESZ_32GB: c_int = 25;
pub const BOOK3E_PAGESZ_64GB: c_int = 26;
pub const BOOK3E_PAGESZ_128GB: c_int = 27;
pub const BOOK3E_PAGESZ_256GB: c_int = 28;
pub const BOOK3E_PAGESZ_512GB: c_int = 29;
pub const BOOK3E_PAGESZ_1TB: c_int = 30;
pub const BOOK3E_PAGESZ_2TB: c_int = 31;
// MAS registers bit definitions
pub const MAS0_TLBSEL_MASK: c_uint = 0x30000000;
pub const MAS0_TLBSEL_SHIFT: c_int = 28;

pub const MAS0_ESEL_MASK: c_uint = 0x0FFF0000;
pub const MAS0_ESEL_SHIFT: c_int = 16;

pub const MAS0_HES: c_uint = 0x00004000;
pub const MAS0_WQ_ALLWAYS: c_uint = 0x00000000;
pub const MAS0_WQ_COND: c_uint = 0x00001000;
pub const MAS0_WQ_CLR_RSRV: c_uint = 0x00002000;
pub const MAS1_VALID: c_uint = 0x80000000;
pub const MAS1_IPROT: c_uint = 0x40000000;

pub const MAS1_IND: c_uint = 0x00002000;
pub const MAS1_TS: c_uint = 0x00001000;
pub const MAS1_TSIZE_MASK: c_uint = 0x00000f80;
pub const MAS1_TSIZE_SHIFT: c_int = 7;

pub const MAS2_X0: c_uint = 0x00000040;
pub const MAS2_X1: c_uint = 0x00000020;
pub const MAS2_W: c_uint = 0x00000010;
pub const MAS2_I: c_uint = 0x00000008;
pub const MAS2_M: c_uint = 0x00000004;
pub const MAS2_G: c_uint = 0x00000002;
pub const MAS2_E: c_uint = 0x00000001;
pub const MAS2_WIMGE_MASK: c_uint = 0x0000001f;

pub const MAS3_RPN: c_uint = 0xFFFFF000;
pub const MAS3_U0: c_uint = 0x00000200;
pub const MAS3_U1: c_uint = 0x00000100;
pub const MAS3_U2: c_uint = 0x00000080;
pub const MAS3_U3: c_uint = 0x00000040;
pub const MAS3_UX: c_uint = 0x00000020;
pub const MAS3_SX: c_uint = 0x00000010;
pub const MAS3_UW: c_uint = 0x00000008;
pub const MAS3_SW: c_uint = 0x00000004;
pub const MAS3_UR: c_uint = 0x00000002;
pub const MAS3_SR: c_uint = 0x00000001;
pub const MAS3_BAP_MASK: c_uint = 0x0000003f;
pub const MAS3_SPSIZE: c_uint = 0x0000003e;
pub const MAS3_SPSIZE_SHIFT: c_int = 1;

pub const MAS4_INDD: c_uint = 0x00008000	/* Default IND */;

pub const MAS4_X0D: c_uint = 0x00000040;
pub const MAS4_X1D: c_uint = 0x00000020;
pub const MAS4_WD: c_uint = 0x00000010;
pub const MAS4_ID: c_uint = 0x00000008;
pub const MAS4_MD: c_uint = 0x00000004;
pub const MAS4_GD: c_uint = 0x00000002;
pub const MAS4_ED: c_uint = 0x00000001;
pub const MAS4_WIMGED_MASK: c_uint = 0x0000001f	/* Default WIMGE */;
pub const MAS4_WIMGED_SHIFT: c_int = 0;

pub const MAS4_ACMD: c_uint = 0x000000c0	/* Default ACM */;
pub const MAS4_ACMD_SHIFT: c_int = 6;
pub const MAS4_TSIZED_MASK: c_uint = 0x00000f80	/* Default TSIZE */;
pub const MAS4_TSIZED_SHIFT: c_int = 7;
pub const MAS5_SGS: c_uint = 0x80000000;
pub const MAS6_SPID0: c_uint = 0x3FFF0000;
pub const MAS6_SPID1: c_uint = 0x00007FFE;

pub const MAS6_SAS: c_uint = 0x00000001;

pub const MAS6_SIND: c_uint = 0x00000002	/* Indirect page */;
pub const MAS6_SIND_SHIFT: c_int = 1;
pub const MAS6_SPID_MASK: c_uint = 0x3fff0000;
pub const MAS6_SPID_SHIFT: c_int = 16;
pub const MAS6_ISIZE_MASK: c_uint = 0x00000f80;
pub const MAS6_ISIZE_SHIFT: c_int = 7;
pub const MAS7_RPN: c_uint = 0xFFFFFFFF;
pub const MAS8_TGS: c_uint = 0x80000000 /* Guest space */;
pub const MAS8_VF: c_uint = 0x40000000 /* Virtualization Fault */;
pub const MAS8_TLPID: c_uint = 0x000000ff;
// Bit definitions for MMUCFG
pub const MMUCFG_MAVN: c_uint = 0x00000003	/* MMU Architecture Version Number */;
pub const MMUCFG_MAVN_V1: c_uint = 0x00000000	/* v1.0 */;
pub const MMUCFG_MAVN_V2: c_uint = 0x00000001	/* v2.0 */;
pub const MMUCFG_NTLBS: c_uint = 0x0000000c	/* Number of TLBs */;
pub const MMUCFG_PIDSIZE: c_uint = 0x000007c0	/* PID Reg Size */;
pub const MMUCFG_TWC: c_uint = 0x00008000	/* TLB Write Conditional (v2.0) */;
pub const MMUCFG_LRAT: c_uint = 0x00010000	/* LRAT Supported (v2.0) */;
pub const MMUCFG_RASIZE: c_uint = 0x00fe0000	/* Real Addr Size */;
pub const MMUCFG_LPIDSIZE: c_uint = 0x0f000000	/* LPID Reg Size */;
// Bit definitions for MMUCSR0
pub const MMUCSR0_TLB1FI: c_uint = 0x00000002	/* TLB1 Flash invalidate */;
pub const MMUCSR0_TLB0FI: c_uint = 0x00000004	/* TLB0 Flash invalidate */;
pub const MMUCSR0_TLB2FI: c_uint = 0x00000040	/* TLB2 Flash invalidate */;
pub const MMUCSR0_TLB3FI: c_uint = 0x00000020	/* TLB3 Flash invalidate */;

pub const MMUCSR0_TLB0PS: c_uint = 0x00000780	/* TLB0 Page Size */;
pub const MMUCSR0_TLB1PS: c_uint = 0x00007800	/* TLB1 Page Size */;
pub const MMUCSR0_TLB2PS: c_uint = 0x00078000	/* TLB2 Page Size */;
pub const MMUCSR0_TLB3PS: c_uint = 0x00780000	/* TLB3 Page Size */;
// MMUCFG bits
pub const MMUCFG_MAVN_NASK: c_uint = 0x00000003;
pub const MMUCFG_MAVN_V1_0: c_uint = 0x00000000;
pub const MMUCFG_MAVN_V2_0: c_uint = 0x00000001;
pub const MMUCFG_NTLB_MASK: c_uint = 0x0000000c;
pub const MMUCFG_NTLB_SHIFT: c_int = 2;
pub const MMUCFG_PIDSIZE_MASK: c_uint = 0x000007c0;
pub const MMUCFG_PIDSIZE_SHIFT: c_int = 6;
pub const MMUCFG_TWC: c_uint = 0x00008000;
pub const MMUCFG_LRAT: c_uint = 0x00010000;
pub const MMUCFG_RASIZE_MASK: c_uint = 0x00fe0000;
pub const MMUCFG_RASIZE_SHIFT: c_int = 17;
pub const MMUCFG_LPIDSIZE_MASK: c_uint = 0x0f000000;
pub const MMUCFG_LPIDSIZE_SHIFT: c_int = 24;
// TLBnCFG encoding
pub const TLBnCFG_N_ENTRY: c_uint = 0x00000fff	/* number of entries */;
pub const TLBnCFG_HES: c_uint = 0x00002000	/* HW select supported */;
pub const TLBnCFG_IPROT: c_uint = 0x00008000	/* IPROT supported */;
pub const TLBnCFG_GTWE: c_uint = 0x00010000	/* Guest can write */;
pub const TLBnCFG_IND: c_uint = 0x00020000	/* IND entries supported */;
pub const TLBnCFG_PT: c_uint = 0x00040000	/* Can load from page table */;
pub const TLBnCFG_MINSIZE: c_uint = 0x00f00000	/* Minimum Page Size (v1.0) */;
pub const TLBnCFG_MINSIZE_SHIFT: c_int = 20;
pub const TLBnCFG_MAXSIZE: c_uint = 0x000f0000	/* Maximum Page Size (v1.0) */;
pub const TLBnCFG_MAXSIZE_SHIFT: c_int = 16;
pub const TLBnCFG_ASSOC: c_uint = 0xff000000	/* Associativity */;
pub const TLBnCFG_ASSOC_SHIFT: c_int = 24;
// TLBnPS encoding
pub const TLBnPS_4K: c_uint = 0x00000004;
pub const TLBnPS_8K: c_uint = 0x00000008;
pub const TLBnPS_16K: c_uint = 0x00000010;
pub const TLBnPS_32K: c_uint = 0x00000020;
pub const TLBnPS_64K: c_uint = 0x00000040;
pub const TLBnPS_128K: c_uint = 0x00000080;
pub const TLBnPS_256K: c_uint = 0x00000100;
pub const TLBnPS_512K: c_uint = 0x00000200;
pub const TLBnPS_1M: c_uint = 0x00000400;
pub const TLBnPS_2M: c_uint = 0x00000800;
pub const TLBnPS_4M: c_uint = 0x00001000;
pub const TLBnPS_8M: c_uint = 0x00002000;
pub const TLBnPS_16M: c_uint = 0x00004000;
pub const TLBnPS_32M: c_uint = 0x00008000;
pub const TLBnPS_64M: c_uint = 0x00010000;
pub const TLBnPS_128M: c_uint = 0x00020000;
pub const TLBnPS_256M: c_uint = 0x00040000;
pub const TLBnPS_512M: c_uint = 0x00080000;
pub const TLBnPS_1G: c_uint = 0x00100000;
pub const TLBnPS_2G: c_uint = 0x00200000;
pub const TLBnPS_4G: c_uint = 0x00400000;
pub const TLBnPS_8G: c_uint = 0x00800000;
pub const TLBnPS_16G: c_uint = 0x01000000;
pub const TLBnPS_32G: c_uint = 0x02000000;
pub const TLBnPS_64G: c_uint = 0x04000000;
pub const TLBnPS_128G: c_uint = 0x08000000;
pub const TLBnPS_256G: c_uint = 0x10000000;
// tlbilx action encoding
pub const TLBILX_T_ALL: c_int = 0;
pub const TLBILX_T_TID: c_int = 1;
pub const TLBILX_T_FULLMATCH: c_int = 3;
pub const TLBILX_T_CLASS0: c_int = 4;
pub const TLBILX_T_CLASS1: c_int = 5;
pub const TLBILX_T_CLASS2: c_int = 6;
pub const TLBILX_T_CLASS3: c_int = 7;
//
// The mapping only needs to be cache-coherent on SMP, except on
// Freescale e500mc derivatives where it's also needed for coherent DMA.
//

pub const MAS2_M_IF_NEEDED: c_int = 0;

// Page size definitions, common between 32 and 64-bit
//
// shift : is the "PAGE_SHIFT" value for that page size
//
pub const MMU_PAGE_SIZE_DIRECT: c_uint = 0x1	/* Supported as a direct size */;
pub const MMU_PAGE_SIZE_INDIRECT: c_uint = 0x2	/* Supported as an indirect size */;
// The page sizes use the same names as 64-bit hash but are
// constants
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlb_core_data {
//
// Per-core spinlock for e6500 TLB handlers (no tlbsrx.)
// Must be the first struct element.
//
    pub lock: u8,
// For software way selection, as on Freescale TLB1
    pub esel_first: u8 esel_next, esel_max,,
}

pub const PPC_HTW_NONE: c_int = 0;
pub const PPC_HTW_E6500: c_int = 1;
//
// 64-bit booke platforms don't load the tlb in the tlb miss handler code.
// HUGETLB_NEED_PRELOAD handles this - it causes huge_ptep_set_access_flags to
// return 1, indicating that the tlb requires preloading.
//
// Macro flag: #define HUGETLB_NEED_PRELOAD

pub const MAX_PHYSMEM_BITS: c_int = 44;

