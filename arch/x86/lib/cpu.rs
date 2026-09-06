//! Automatically rewritten from C to Rust
//! Source: arch/x86/lib/cpu.c
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


// SPDX-License-Identifier: GPL-2.0-only

#[no_mangle]
unsafe extern "C" fn __x86_family(base_fam: c_uint, ext_fam: c_uint) -> c_uint {
    static unsigned int __x86_family(unsigned int base_fam, unsigned int ext_fam)
    {
    if (base_fam == 0xf)
    base_fam += ext_fam;
    return base_fam;
    }
    static unsigned int __x86_model(unsigned int family, unsigned int base_model,
    unsigned int ext_model)
    {
    if (family >= 0x6)
    base_model |= ext_model << 4;
    return base_model;
    }
#[no_mangle]
pub unsafe extern "C" fn x86_family(sig: c_uint) -> c_uint {
    unsigned int x86_family(unsigned int sig)
    {
    return __x86_family((sig >> 8) & 0xf, (sig >> 20) & 0xff);
    }
    EXPORT_SYMBOL_GPL(x86_family);
#[no_mangle]
pub unsafe extern "C" fn x86_model(sig: c_uint) -> c_uint {
    unsigned int x86_model(unsigned int sig)
    {
    return __x86_model(x86_family(sig), (sig >> 4) & 0xf, (sig >> 16) & 0xf);
    }
    EXPORT_SYMBOL_GPL(x86_model);
#[no_mangle]
pub unsafe extern "C" fn x86_stepping(sig: c_uint) -> c_uint {
    unsigned int x86_stepping(unsigned int sig)
    {
    return sig & 0xf;
    }
    EXPORT_SYMBOL_GPL(x86_stepping);
#[no_mangle]
pub unsafe extern "C" fn cpuid_family(l: *const leaf_0x1_0) -> c_uint {
    unsigned int cpuid_family(const struct leaf_0x1_0 *l)
    {
    return __x86_family(l.base_family_id, l.ext_family);
    }
#[no_mangle]
pub unsafe extern "C" fn cpuid_model(l: *const leaf_0x1_0) -> c_uint {
    unsigned int cpuid_model(const struct leaf_0x1_0 *l)
    {
    return __x86_model(cpuid_family(l), l.base_model, l.ext_model);
    }
