//! Automatically rewritten from C to Rust
//! Source: arch/loongarch/power/suspend.c
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
// loongson-specific suspend support
//
// Author: Huacai Chen <chenhuacai@loongson.cn>
// Copyright (C) 2020-2022 Loongson Technology Corporation Limited
//

    u64 loongarch_suspend_addr;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct saved_registers {
    pub ecfg: u32,
    pub euen: u32,
    pub pwctl0: u32,
    pub pwctl1: u32,
    pub pgd: c_ulong,
    pub kpgd: c_ulong,
    pub pcpu_base: c_ulong,
}

    static struct saved_registers saved_regs;
#[no_mangle]
pub unsafe extern "C" fn loongarch_common_suspend() {
    void loongarch_common_suspend(void)
    {
    save_counter();
    saved_regs.pgd = csr_read(LOONGARCH_CSR_PGDL);
    saved_regs.kpgd = csr_read(LOONGARCH_CSR_PGDH);
    saved_regs.pwctl0 = csr_read32(LOONGARCH_CSR_PWCTL0);
    saved_regs.pwctl1 = csr_read32(LOONGARCH_CSR_PWCTL1);
    saved_regs.ecfg = csr_read32(LOONGARCH_CSR_ECFG);
    saved_regs.euen = csr_read32(LOONGARCH_CSR_EUEN);
    saved_regs.pcpu_base = csr_read(PERCPU_BASE_KS);
    loongarch_suspend_addr = loongson_sysconf.suspend_addr;
    }
#[no_mangle]
pub unsafe extern "C" fn loongarch_common_resume() {
    void loongarch_common_resume(void)
    {
    sync_counter();
    local_flush_tlb_all();
    csr_write(eentry, LOONGARCH_CSR_EENTRY);
    csr_write(eentry, LOONGARCH_CSR_MERRENTRY);
    csr_write(tlbrentry, LOONGARCH_CSR_TLBRENTRY);
    csr_write(saved_regs.pgd, LOONGARCH_CSR_PGDL);
    csr_write(saved_regs.kpgd, LOONGARCH_CSR_PGDH);
    csr_write32(saved_regs.pwctl0, LOONGARCH_CSR_PWCTL0);
    csr_write32(saved_regs.pwctl1, LOONGARCH_CSR_PWCTL1);
    csr_write32(saved_regs.ecfg, LOONGARCH_CSR_ECFG);
    csr_write32(saved_regs.euen, LOONGARCH_CSR_EUEN);
    csr_write(saved_regs.pcpu_base, PERCPU_BASE_KS);
    }
#[no_mangle]
pub unsafe extern "C" fn loongarch_acpi_suspend() -> c_int {
    int loongarch_acpi_suspend(void)
    {
    enable_gpe_wakeup();
    enable_pci_wakeup();
    loongarch_common_suspend();
// processor specific suspend
    loongarch_suspend_enter();
    loongarch_common_resume();
    return 0;
    }
