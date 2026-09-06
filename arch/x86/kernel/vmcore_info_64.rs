//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/vmcore_info_64.c
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
    let mut sme_mask: u64 = sme_me_mask;
    VMCOREINFO_NUMBER(phys_base);
    VMCOREINFO_SYMBOL(init_top_pgt);
    vmcoreinfo_append_str("NUMBER(pgtable_l5_enabled)=%d\n",
    pgtable_l5_enabled());

    VMCOREINFO_SYMBOL(node_data);
    VMCOREINFO_LENGTH(node_data, MAX_NUMNODES);

    vmcoreinfo_append_str("KERNELOFFSET=%lx\n", kaslr_offset());
    VMCOREINFO_NUMBER(KERNEL_IMAGE_SIZE);
    VMCOREINFO_NUMBER(sme_mask);
    }
