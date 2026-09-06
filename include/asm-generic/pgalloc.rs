//! Automatically rewritten from C Header to Rust Module
//! Source: include/asm-generic/pgalloc.h
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
// __pte_alloc_one_kernel - allocate memory for a PTE-level kernel page table
// @mm: the mm_struct of the current context
//
// This function is intended for architectures that need
// anything beyond simple page allocation.
//
// Return: pointer to the allocated memory or %NULL on error
//
extern "C" {
    pub fn ptdesc_address(_arg: ptdesc) -> return;
}

//
// pte_alloc_one_kernel - allocate memory for a PTE-level kernel page table
// @mm: the mm_struct of the current context
//
// Return: pointer to the allocated memory or %NULL on error
//
extern "C" {
    pub fn __pte_alloc_one_kernel_noprof(_arg: mm) -> return;
}

//
// pte_free_kernel - free PTE-level kernel page table memory
// @mm: the mm_struct of the current context
// @pte: pointer to the memory containing the page table
//
// __pte_alloc_one - allocate memory for a PTE-level user page table
// @mm: the mm_struct of the current context
// @gfp: GFP flags to use for the allocation
//
// Allocate memory for a page table and ptdesc and runs pagetable_pte_ctor().
//
// This function is intended for architectures that need
// anything beyond simple page allocation or must have custom GFP flags.
//
// Return: `struct page` referencing the ptdesc or %NULL on error
//
extern "C" {
    pub fn ptdesc_page(_arg: ptdesc) -> return;
}

//
// pte_alloc_one - allocate a page for PTE-level user page table
// @mm: the mm_struct of the current context
//
// Allocate memory for a page table and ptdesc and runs pagetable_pte_ctor().
//
// Return: `struct page` referencing the ptdesc or %NULL on error
//
extern "C" {
    pub fn __pte_alloc_one_noprof(_arg: mm, _arg: GFP_PGTABLE_USER) -> return;
}

//
// Should really implement gc for free page table pages. This could be
// done with a reference count in struct page.
//
// pte_free - free PTE-level user page table memory
// @mm: the mm_struct of the current context
// @pte_page: the `struct page` referencing the ptdesc
//

//
// pmd_alloc_one - allocate memory for a PMD-level page table
// @mm: the mm_struct of the current context
//
// Allocate memory for a page table and ptdesc and runs pagetable_pmd_ctor().
//
// Allocations use %GFP_PGTABLE_USER in user context and
// %GFP_PGTABLE_KERNEL in kernel context.
//
// Return: pointer to the allocated memory or %NULL on error
//
extern "C" {
    pub fn ptdesc_address(_arg: ptdesc) -> return;
}

extern "C" {
    pub fn ptdesc_address(_arg: ptdesc) -> return;
}

//
// pud_alloc_one - allocate memory for a PUD-level page table
// @mm: the mm_struct of the current context
//
// Allocate memory for a page table using %GFP_PGTABLE_USER for user context
// and %GFP_PGTABLE_KERNEL for kernel context.
//
// Return: pointer to the allocated memory or %NULL on error
//
extern "C" {
    pub fn __pud_alloc_one_noprof(_arg: mm, _arg: addr) -> return;
}

extern "C" {
    pub fn ptdesc_address(_arg: ptdesc) -> return;
}

extern "C" {
    pub fn __p4d_alloc_one_noprof(_arg: mm, _arg: addr) -> return;
}

extern "C" {
    pub fn ptdesc_address(_arg: ptdesc) -> return;
}

