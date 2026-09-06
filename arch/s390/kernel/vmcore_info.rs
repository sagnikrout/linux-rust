//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/vmcore_info.c
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
pub unsafe extern "C" fn arch_crash_save_vmcoreinfo() {
    void arch_crash_save_vmcoreinfo(void)
    {
    struct lowcore *abs_lc;
    VMCOREINFO_SYMBOL(lowcore_ptr);
    VMCOREINFO_SYMBOL(high_memory);
    VMCOREINFO_LENGTH(lowcore_ptr, NR_CPUS);
    vmcoreinfo_append_str("SAMODE31=%lx\n", (unsigned long)__samode31);
    vmcoreinfo_append_str("EAMODE31=%lx\n", (unsigned long)__eamode31);
    vmcoreinfo_append_str("IDENTITYBASE=%lx\n", __identity_base);
    vmcoreinfo_append_str("KERNELOFFSET=%lx\n", kaslr_offset());
    vmcoreinfo_append_str("KERNELOFFPHYS=%lx\n", __kaslr_offset_phys);
    abs_lc = get_abs_lowcore();
    abs_lc.vmcore_info = paddr_vmcoreinfo_note();
    put_abs_lowcore(abs_lc);
    }
