//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kernel/vmcore_info.c
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
// Copyright (C) Linaro.
// Copyright (C) Huawei Futurewei Technologies.
//

    static inline u64 get_tcr_el1_t1sz(void);
#[no_mangle]
pub unsafe extern "C" fn get_tcr_el1_t1sz() -> u64 {
    static inline u64 get_tcr_el1_t1sz(void)
    {
    return (read_sysreg(tcr_el1) & TCR_EL1_T1SZ_MASK) >> TCR_EL1_T1SZ_SHIFT;
    }
#[no_mangle]
pub unsafe extern "C" fn arch_crash_save_vmcoreinfo() {
    void arch_crash_save_vmcoreinfo(void)
    {
    VMCOREINFO_NUMBER(VA_BITS);
// Please note VMCOREINFO_NUMBER() uses "%d", not "%x"
    vmcoreinfo_append_str("NUMBER(MODULES_VADDR)=0x%lx\n", MODULES_VADDR);
    vmcoreinfo_append_str("NUMBER(MODULES_END)=0x%lx\n", MODULES_END);
    vmcoreinfo_append_str("NUMBER(VMALLOC_END)=0x%lx\n", VMALLOC_END);
    vmcoreinfo_append_str("NUMBER(VMEMMAP_START)=0x%lx\n", VMEMMAP_START);
    vmcoreinfo_append_str("NUMBER(VMEMMAP_END)=0x%lx\n", VMEMMAP_END);
    vmcoreinfo_append_str("NUMBER(kimage_voffset)=0x%llx\n",
    kimage_voffset);
    vmcoreinfo_append_str("NUMBER(PHYS_OFFSET)=0x%llx\n",
    PHYS_OFFSET);
    vmcoreinfo_append_str("NUMBER(TCR_EL1_T1SZ)=0x%llx\n",
    get_tcr_el1_t1sz());
    vmcoreinfo_append_str("KERNELOFFSET=%lx\n", kaslr_offset());
    vmcoreinfo_append_str("NUMBER(KERNELPACMASK)=0x%llx\n",
    system_supports_address_auth() ?
    ptrauth_kernel_pac_mask() : 0);
    }
