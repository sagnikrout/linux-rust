//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kernel/vmcore_info.c
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
pub unsafe extern "C" fn get_satp_value() -> u64 {
    static inline u64 get_satp_value(void)
    {
    return csr_read(CSR_SATP);
    }
#[no_mangle]
pub unsafe extern "C" fn arch_crash_save_vmcoreinfo() {
    void arch_crash_save_vmcoreinfo(void)
    {
    VMCOREINFO_NUMBER(phys_ram_base);
    vmcoreinfo_append_str("NUMBER(PAGE_OFFSET)=0x%lx\n", PAGE_OFFSET);
    vmcoreinfo_append_str("NUMBER(VMALLOC_END)=0x%lx\n", VMALLOC_END);

    VMCOREINFO_NUMBER(VA_BITS);
    vmcoreinfo_append_str("NUMBER(VMEMMAP_START)=0x%lx\n", VMEMMAP_START);
    vmcoreinfo_append_str("NUMBER(VMEMMAP_END)=0x%lx\n", VMEMMAP_END);

    vmcoreinfo_append_str("NUMBER(MODULES_VADDR)=0x%lx\n", MODULES_VADDR);
    vmcoreinfo_append_str("NUMBER(MODULES_END)=0x%lx\n", MODULES_END);

    vmcoreinfo_append_str("NUMBER(KERNEL_LINK_ADDR)=0x%lx\n", KERNEL_LINK_ADDR);
    vmcoreinfo_append_str("NUMBER(va_kernel_pa_offset)=0x%lx\n",
    kernel_map.va_kernel_pa_offset);
    vmcoreinfo_append_str("KERNELOFFSET=%lx\n", kaslr_offset());
    vmcoreinfo_append_str("NUMBER(satp)=0x%llx\n", get_satp_value());
    }
