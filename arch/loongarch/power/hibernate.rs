//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/power/hibernate.c
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

    static u32 saved_crmd;
    static u32 saved_prmd;
    static u32 saved_euen;
    static u32 saved_ecfg;
    static unsigned long saved_pcpu_base;
    struct pt_regs saved_regs;
#[no_mangle]
pub unsafe extern "C" fn save_processor_state() {
    void save_processor_state(void)
    {
    save_counter();
    saved_crmd = csr_read32(LOONGARCH_CSR_CRMD);
    saved_prmd = csr_read32(LOONGARCH_CSR_PRMD);
    saved_euen = csr_read32(LOONGARCH_CSR_EUEN);
    saved_ecfg = csr_read32(LOONGARCH_CSR_ECFG);
    saved_pcpu_base = csr_read(PERCPU_BASE_KS);
    if (is_fpu_owner())
    save_fp(current);
    }
#[no_mangle]
pub unsafe extern "C" fn restore_processor_state() {
    void restore_processor_state(void)
    {
    sync_counter();
    csr_write32(saved_crmd, LOONGARCH_CSR_CRMD);
    csr_write32(saved_prmd, LOONGARCH_CSR_PRMD);
    csr_write32(saved_euen, LOONGARCH_CSR_EUEN);
    csr_write32(saved_ecfg, LOONGARCH_CSR_ECFG);
    csr_write(saved_pcpu_base, PERCPU_BASE_KS);
    if (is_fpu_owner())
    restore_fp(current);
    }
#[no_mangle]
pub unsafe extern "C" fn pfn_is_nosave(pfn: c_ulong) -> c_int {
    int pfn_is_nosave(unsigned long pfn)
    {
    let mut nosave_begin_pfn: c_ulong = PFN_DOWN(__pa(&__nosave_begin));
    let mut nosave_end_pfn: c_ulong = PFN_UP(__pa(&__nosave_end));
    return	(pfn >= nosave_begin_pfn) && (pfn < nosave_end_pfn);
    }
    extern int swsusp_asm_suspend(void);
#[no_mangle]
pub unsafe extern "C" fn swsusp_arch_suspend() -> c_int {
    int swsusp_arch_suspend(void)
    {
    enable_pci_wakeup();
    return swsusp_asm_suspend();
    }
    extern int swsusp_asm_resume(void);
#[no_mangle]
pub unsafe extern "C" fn swsusp_arch_resume() -> c_int {
    int swsusp_arch_resume(void)
    {
// Avoid TLB mismatch during and after kernel resume
    local_flush_tlb_all();
    return swsusp_asm_resume();
    }
