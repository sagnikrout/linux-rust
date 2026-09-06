//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/mm/mmu_decl.h
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
// Declarations of procedures and variables shared between files
// in arch/ppc/mm/.
//
// Derived from arch/ppc/mm/init.c:
// Copyright (C) 1995-1996 Gary Thomas (gdt@linuxppc.org)
//
// Modifications by Paul Mackerras (PowerMac) (paulus@cs.anu.edu.au)
// and Cort Dougan (PReP) (cort@cs.nmt.edu)
// Copyright (C) 1996 Paul Mackerras
//
// Derived from "arch/i386/mm/init.c"
// Copyright (C) 1991, 1992, 1993, 1994  Linus Torvalds
//

//
// On 8xx, we directly inline tlbia
//

extern "C" {
    pub fn volatile("memory": "sync; tlbia; isync" : : :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "sync; tlbia; isync" : : :) -> asm;
}

extern "C" {
    pub fn _tlbil_all();
}
extern "C" {
    pub fn _tlbil_pid(pid: c_uint);
}

extern "C" {
    pub fn _tlbil_pid_noind(pid: c_uint);
}

//
// On 8xx, we directly inline tlbie, on others, it's extern
//

extern "C" {
    pub fn volatile("memory": "tlbie %0; sync" : : "r" (address) :) -> asm;
}

extern "C" {
    pub fn __tlbil_va(address: c_ulong, pid: c_uint);
}

extern "C" {
    pub fn print_system_hash_info();
}

extern "C" {
    pub fn mapin_ram();
}

// ...and now those things that may be slightly different between processor
// architectures.  -- Dan
//

extern "C" {
    pub fn MMU_init_hw();
}
extern "C" {
    pub fn MMU_init_hw_patch();
}
extern "C" {
    pub fn mmu_mapin_ram(base: c_ulong, top: c_ulong) -> c_ulong;
}

extern "C" {
    pub fn mmu_init_secondary(cpu: c_int);
}

extern "C" {
    pub fn adjust_total_lowmem();
}
extern "C" {
    pub fn switch_to_as1() -> c_int;
}
extern "C" {
    pub fn restore_to_as0(esel: c_int, offset: c_int, dt_ptr: *mut c_void, bootcpu: c_int);
}
extern "C" {
    pub fn create_kaslr_tlb_entry(entry: c_int, virt: c_ulong, phys: phys_addr_t);
}
extern "C" {
    pub fn reloc_kernel_entry(fdt: *mut c_void, addr: c_int);
}
extern "C" {
    pub fn relocate_init(dt_ptr: u64, start: phys_addr_t);
}

extern "C" {
    pub fn loadcam_entry(index: c_uint);
}
extern "C" {
    pub fn loadcam_multi(first_idx: c_int, num: c_int, tmp_idx: c_int);
}

extern "C" {
    pub fn kaslr_early_init(dt_ptr: *mut c_void, size: phys_addr_t);
}
extern "C" {
    pub fn kaslr_late_init();
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlbcam {
    pub MAS0: u32,
    pub MAS1: u32,
    pub MAS2: c_ulong,
    pub MAS3: u32,
    pub MAS7: u32,
}

pub const NUM_TLBCAMS: c_int = 64;

// 6xx have BATS
// PPC_85xx have TLBCAM
// 8xx have LTLB
extern "C" {
    pub fn v_block_mapped(va: c_ulong) -> phys_addr_t;
}
extern "C" {
    pub fn p_block_mapped(pa: phys_addr_t) -> c_ulong;
}

extern "C" {
    pub fn mmu_mark_initmem_nx() -> c_int;
}
extern "C" {
    pub fn mmu_mark_rodata_ro() -> c_int;
}

extern "C" {
    pub fn mmu_mapin_immr() -> void __init;
}

extern "C" {
    pub fn IS_ENABLED(debug_pagealloc_enabled(: CONFIG_KFENCE) ||) -> return;
}

extern "C" {
    pub fn hash__kernel_map_pages(page: *mut page, numpages: c_int, enable: c_int) -> c_int;
}
