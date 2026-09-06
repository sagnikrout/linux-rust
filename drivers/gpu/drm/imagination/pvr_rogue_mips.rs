//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_rogue_mips.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Utility defines for memory management.

// log2 page table sizes dependent on FW heap size and page size (for each OS).

// Maximum number of page table pages (both Host and MIPS pages).

// Total number of TLB entries.

// "Uncached" caching policy.

// "Write-back write-allocate" caching policy.

// "Write-through no write-allocate" caching policy.

// Cached policy used by MIPS in case of physical bus on 32 bit.

// Cached policy used by MIPS in case of physical bus on more than 32 bit.

// Total number of Remap entries.

// MIPS EntryLo/PTE format.

// Page Frame Number

// Mask used for the MIPS Page Table in case of physical bus on 32 bit.

// Mask used for the MIPS Page Table in case of physical bus on more than 32 bit.

// Remap Range Config Addr Out.
// These defines refer to the upper half of the Remap Range Config register.

//
// Pages to trampoline problematic physical addresses:
// - ROGUE_MIPSFW_BOOT_REMAP_PHYS_ADDR_IN : 0x1FC0_0000
// - ROGUE_MIPSFW_DATA_REMAP_PHYS_ADDR_IN : 0x1FC0_1000
// - ROGUE_MIPSFW_CODE_REMAP_PHYS_ADDR_IN : 0x1FC0_2000
// - (benign trampoline)               : 0x1FC0_3000
// that would otherwise be erroneously remapped by the MIPS wrapper.
// (see "Firmware virtual layout and remap configuration" section below)
//

// Firmware virtual layout and remap configuration.
//
// For each remap region we define:
// - the virtual base used by the Firmware to access code/data through that region
// - the microAptivAP physical address correspondent to the virtual base address,
// used as input address and remapped to the actual physical address
// - log2 of size of the region remapped by the MIPS wrapper, i.e. number of bits from
// the bottom of the base input address that survive onto the output address
// (this defines both the alignment and the maximum size of the remapped region)
// - one or more code/data segments within the remapped region.
//
// Boot remap setup.

// Data remap setup.

// Code remap setup.

// Permanent mappings setup.

// Bootloader configuration data.
//
// Bootloader configuration offset (where ROGUE_MIPSFW_BOOT_DATA lives)
// within the bootloader/NMI data page.
//

// NMI shared data.
// Base address of the shared data within the bootloader/NMI data page.

// Size used by Debug dump data.

// Offsets in the NMI shared area in 32-bit words.

// MIPS boot stage.

//
// MIPS private data in the bootloader data page.
// Memory below this offset is used by the FW only, no interface data allowed.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_mipsfw_boot_data {
    pub stack_phys_addr: u64,
    pub reg_base: u64,
    pub pt_phys_addr: [u64; ROGUE_MIPSFW_MAX_NUM_PAGETABLE_PAGES],
    pub pt_log2_page_size: u32,
    pub pt_num_pages: u32,
    pub reserved1: u32,
    pub reserved2: u32,
}

// Used for compatibility checks.

// microAptivAP cache line size.

//
// The SOCIF transactions are identified with the top 16 bits of the physical address emitted by
// the MIPS.
//

// Values to put in the MIPS selectors for performance counters.
// Icache accesses in COUNTER0.

// Icache misses in COUNTER1.

// Dcache accesses in COUNTER0.

// Dcache misses in COUNTER1.

// ITLB instruction accesses in COUNTER0.

// JTLB instruction accesses misses in COUNTER1.

// Instructions completed in COUNTER0.

// JTLB data misses in COUNTER1.

// Shift for the Event field in the MIPS perf ctrl registers.

// Additional flags for performance counters. See MIPS manual for further reference.

pub const ROGUE_MIPSFW_C0_NBHWIRQ: c_int = 8;
// Macros to decode C0_Cause register.

pub const ROGUE_MIPSFW_C0_CAUSE_EXCCODE_FWERROR: c_int = 9;
// Use only when Coprocessor Unusable exception.

// Macros to decode C0_Debug register.

// Macros to decode TLB entries.

// Page size in KB.

// Page size in KB.

// GET_PA uses a non-standard PFN mask for 36 bit addresses.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_mips_tlb_entry {
    pub tlb_page_mask: u32,
    pub tlb_hi: u32,
    pub tlb_lo0: u32,
    pub tlb_lo1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_mips_remap_entry {
    pub /: *mut *mut u32 remap_addr_in; / Always 4k aligned.,
    pub /: *mut *mut u32 remap_addr_out; / Always 4k aligned.,
    pub remap_region_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rogue_mips_state {
    pub /: *mut *mut u32 error_state; / This must come first in the structure.,
    pub error_epc: u32,
    pub status_register: u32,
    pub cause_register: u32,
    pub bad_register: u32,
    pub epc: u32,
    pub sp: u32,
    pub debug: u32,
    pub depc: u32,
    pub bad_instr: u32,
    pub unmapped_address: u32,
    pub tlb: [rogue_mips_tlb_entry; ROGUE_MIPSFW_NUMBER_OF_TLB_ENTRIES],
    pub remap: [rogue_mips_remap_entry; ROGUE_MIPSFW_NUMBER_OF_REMAP_ENTRIES],
}

