//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/kasan/book3s_32.c
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
// Macro flag: #define DISABLE_BRANCH_PROFILING

#[no_mangle]
pub unsafe extern "C" fn kasan_init_region(start: *mut c_void, size: usize) -> int __init {
    int __init kasan_init_region(void *start, size_t size)
    {
    let mut k_start: c_ulong = (unsigned long)kasan_mem_to_shadow(start);
    let mut k_end: c_ulong = (unsigned long)kasan_mem_to_shadow(start + size);
    let mut k_nobat: c_ulong = k_start;
    unsigned long k_cur;
    phys_addr_t phys;
    int ret;
    while (k_nobat < k_end) {
    let mut k_size: c_uint = bat_block_size(k_nobat, k_end);
    let mut idx: c_int = find_free_bat();
    if (idx == -1)
    break;
    if (k_size < SZ_128K)
    break;
    phys = memblock_phys_alloc_range(k_size, k_size, 0,
    MEMBLOCK_ALLOC_ANYWHERE);
    if (!phys)
    break;
    setbat(idx, k_nobat, phys, k_size, PAGE_KERNEL);
    k_nobat += k_size;
    }
    if (k_nobat != k_start)
    update_bats();
    if (k_nobat < k_end) {
    phys = memblock_phys_alloc_range(k_end - k_nobat, PAGE_SIZE, 0,
    MEMBLOCK_ALLOC_ANYWHERE);
    if (!phys)
    return -ENOMEM;
    }
    ret = kasan_init_shadow_page_tables(k_start, k_end);
    if (ret)
    return ret;
    kasan_update_early_region(k_start, k_nobat, __pte(0));
    for (k_cur = k_nobat; k_cur < k_end; k_cur += PAGE_SIZE) {
    pmd_t *pmd = pmd_off_k(k_cur);
    let mut pte: pte_t = pfn_pte(PHYS_PFN(phys + k_cur - k_nobat), PAGE_KERNEL);
    __set_pte_at(&init_mm, k_cur, pte_offset_kernel(pmd, k_cur), pte, 0);
    }
    flush_tlb_kernel_range(k_start, k_end);
    memset(kasan_mem_to_shadow(start), 0, k_end - k_start);
    return 0;
    }
