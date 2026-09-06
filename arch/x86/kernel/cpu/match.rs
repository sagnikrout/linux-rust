//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/cpu/match.c
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
// x86_match_cpu - match current CPU against an array of x86_cpu_ids
// @match: Pointer to array of x86_cpu_ids. Last entry terminated with
// {}.
//
// Return the entry if the current CPU matches the entries in the
// passed x86_cpu_id match table. Otherwise NULL.  The match table
// contains vendor (X86_VENDOR_*), family, model and feature bits or
// respective wildcard entries.
//
// A typical table entry would be to match a specific CPU
//
// X86_MATCH_VFM_FEATURE(INTEL_BROADWELL, X86_FEATURE_ANY, NULL);
//
// Fields can be wildcarded with %X86_VENDOR_ANY, %X86_FAMILY_ANY,
// %X86_MODEL_ANY, %X86_FEATURE_ANY (except for vendor)
//
// asm/cpu_device_id.h contains a set of useful macros which are shortcuts
// for various common selections. The above can be shortened to:
//
// X86_MATCH_VFM(INTEL_BROADWELL, NULL);
//
// Arrays used to match for this should also be declared using
// MODULE_DEVICE_TABLE(x86cpu, ...)
//
// This always matches against the boot cpu, assuming models and features are
// consistent over all CPUs.
//
    const struct x86_cpu_id *x86_match_cpu(const struct x86_cpu_id *match)
    {
    const struct x86_cpu_id *m;
    struct cpuinfo_x86 *c = &boot_cpu_data;
    for (m = match; m.flags & X86_CPU_ID_FLAG_ENTRY_VALID; m++) {
    if (m.vendor != X86_VENDOR_ANY && c.x86_vendor != m.vendor)
    continue;
    if (m.family != X86_FAMILY_ANY && c.x86 != m.family)
    continue;
    if (m.model != X86_MODEL_ANY && c.x86_model != m.model)
    continue;
    if (m.steppings != X86_STEPPING_ANY &&
    !(BIT(c.x86_stepping) & m.steppings))
    continue;
    if (m.platform_mask != X86_PLATFORM_ANY &&
    !(BIT(c.intel_platform_id) & m.platform_mask))
    continue;
    if (m.feature != X86_FEATURE_ANY && !cpu_has(c, m.feature))
    continue;
    if (m.type != X86_CPU_TYPE_ANY && c.topo.cpu_type != m.type)
    continue;
    return m;
    }
    return core::ptr::null_mut();
    }
    EXPORT_SYMBOL(x86_match_cpu);
#[no_mangle]
pub unsafe extern "C" fn x86_match_min_microcode_rev(table: *const x86_cpu_id) -> bool {
    bool x86_match_min_microcode_rev(const struct x86_cpu_id *table)
    {
    const struct x86_cpu_id *res = x86_match_cpu(table);
    if (!res || res.driver_data > boot_cpu_data.microcode)
    return false;
    return true;
    }
    EXPORT_SYMBOL_GPL(x86_match_min_microcode_rev);
