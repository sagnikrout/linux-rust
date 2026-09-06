//! Automatically rewritten from C to Rust
//! Source: rust/helpers/mm.c
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

#[no_mangle]
pub unsafe extern "C" fn rust_helper_mmgrab(mm: *mut mm_struct) -> __rust_helper void {
    __rust_helper void rust_helper_mmgrab(struct mm_struct *mm)
    {
    mmgrab(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mmdrop(mm: *mut mm_struct) -> __rust_helper void {
    __rust_helper void rust_helper_mmdrop(struct mm_struct *mm)
    {
    mmdrop(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mmget(mm: *mut mm_struct) -> __rust_helper void {
    __rust_helper void rust_helper_mmget(struct mm_struct *mm)
    {
    mmget(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mmget_not_zero(mm: *mut mm_struct) -> __rust_helper bool {
    __rust_helper bool rust_helper_mmget_not_zero(struct mm_struct *mm)
    {
    return mmget_not_zero(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mmap_read_lock(mm: *mut mm_struct) -> __rust_helper void {
    __rust_helper void rust_helper_mmap_read_lock(struct mm_struct *mm)
    {
    mmap_read_lock(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mmap_read_trylock(mm: *mut mm_struct) -> __rust_helper bool {
    __rust_helper bool rust_helper_mmap_read_trylock(struct mm_struct *mm)
    {
    return mmap_read_trylock(mm);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_mmap_read_unlock(mm: *mut mm_struct) -> __rust_helper void {
    __rust_helper void rust_helper_mmap_read_unlock(struct mm_struct *mm)
    {
    mmap_read_unlock(mm);
    }
    __rust_helper struct vm_area_struct *
    rust_helper_vma_lookup(struct mm_struct *mm, unsigned long addr)
    {
    return vma_lookup(mm, addr);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_vma_end_read(vma: *mut vm_area_struct) -> __rust_helper void {
    __rust_helper void rust_helper_vma_end_read(struct vm_area_struct *vma)
    {
    vma_end_read(vma);
    }
