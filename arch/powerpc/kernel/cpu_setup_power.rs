//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/cpu_setup_power.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2020, Jordan Niethe, IBM Corporation.
//
// This file contains low level CPU setup functions.
// Originally written in assembly by Benjamin Herrenschmidt & various other
// authors.
//

// Disable CPU_FTR_HVMODE and return false if MSR:HV is not set
#[no_mangle]
unsafe extern "C" fn init_hvmode_206(t: *mut cpu_spec) -> bool {
    static bool init_hvmode_206(struct cpu_spec *t)
    {
    u64 msr;
    msr = mfmsr();
    if (msr & MSR_HV)
    return true;
    t.cpu_features &= ~(CPU_FTR_HVMODE | CPU_FTR_P9_TM_HV_ASSIST);
    return false;
    }
#[no_mangle]
unsafe extern "C" fn init_LPCR_ISA300(lpcr: u64, lpes: u64) {
    static void init_LPCR_ISA300(u64 lpcr, u64 lpes)
    {
// POWER9 has no VRMASD
    lpcr |= (lpes << LPCR_LPES_SH) & LPCR_LPES;
    lpcr |= LPCR_PECE0|LPCR_PECE1|LPCR_PECE2;
    lpcr |= (4ull << LPCR_DPFD_SH) & LPCR_DPFD;
    lpcr &= ~LPCR_HDICE;	/* clear HDICE */
    lpcr |= (4ull << LPCR_VC_SH);
    mtspr(SPRN_LPCR, lpcr);
    isync();
    }
//
// Setup a sane LPCR:
// Called with initial LPCR and desired LPES 2-bit value
//
// LPES = 0b01 (HSRR0/1 used for 0x500)
// PECE = 0b111
// DPFD = 4
// HDICE = 0
// VC = 0b100 (VPM0=1, VPM1=0, ISL=0)
// VRMASD = 0b10000 (L=1, LP=00)
//
// Other bits untouched for now
//
#[no_mangle]
unsafe extern "C" fn init_LPCR_ISA206(lpcr: u64, lpes: u64) {
    static void init_LPCR_ISA206(u64 lpcr, u64 lpes)
    {
    lpcr |= (0x10ull << LPCR_VRMASD_SH) & LPCR_VRMASD;
    init_LPCR_ISA300(lpcr, lpes);
    }
#[no_mangle]
unsafe extern "C" fn init_FSCR() {
    static void init_FSCR(void)
    {
    u64 fscr;
    fscr = mfspr(SPRN_FSCR);
    fscr |= FSCR_TAR|FSCR_EBB;
    mtspr(SPRN_FSCR, fscr);
    }
#[no_mangle]
unsafe extern "C" fn init_FSCR_power9() {
    static void init_FSCR_power9(void)
    {
    u64 fscr;
    fscr = mfspr(SPRN_FSCR);
    fscr |= FSCR_SCV;
    mtspr(SPRN_FSCR, fscr);
    init_FSCR();
    }
#[no_mangle]
unsafe extern "C" fn init_FSCR_power10() {
    static void init_FSCR_power10(void)
    {
    u64 fscr;
    fscr = mfspr(SPRN_FSCR);
    fscr |= FSCR_PREFIX;
    mtspr(SPRN_FSCR, fscr);
    init_FSCR_power9();
    }
#[no_mangle]
unsafe extern "C" fn init_HFSCR() {
    static void init_HFSCR(void)
    {
    u64 hfscr;
    hfscr = mfspr(SPRN_HFSCR);
    hfscr |= HFSCR_TAR|HFSCR_TM|HFSCR_BHRB|HFSCR_PM|HFSCR_DSCR|\
    HFSCR_VECVSX|HFSCR_FP|HFSCR_EBB|HFSCR_MSGP;
    mtspr(SPRN_HFSCR, hfscr);
    }
#[no_mangle]
unsafe extern "C" fn init_PMU_HV() {
    static void init_PMU_HV(void)
    {
    mtspr(SPRN_MMCRC, 0);
    }
#[no_mangle]
unsafe extern "C" fn init_PMU_HV_ISA207() {
    static void init_PMU_HV_ISA207(void)
    {
    mtspr(SPRN_MMCRH, 0);
    }
#[no_mangle]
unsafe extern "C" fn init_PMU() {
    static void init_PMU(void)
    {
    mtspr(SPRN_MMCRA, 0);
    mtspr(SPRN_MMCR0, MMCR0_FC);
    mtspr(SPRN_MMCR1, 0);
    mtspr(SPRN_MMCR2, 0);
    }
#[no_mangle]
unsafe extern "C" fn init_PMU_ISA207() {
    static void init_PMU_ISA207(void)
    {
    mtspr(SPRN_MMCRS, 0);
    }
#[no_mangle]
unsafe extern "C" fn init_PMU_ISA31() {
    static void init_PMU_ISA31(void)
    {
    mtspr(SPRN_MMCR3, 0);
    mtspr(SPRN_MMCRA, MMCRA_BHRB_DISABLE);
    mtspr(SPRN_MMCR0, MMCR0_FC | MMCR0_PMCCEXT);
    }
#[no_mangle]
unsafe extern "C" fn init_DEXCR() {
    static void init_DEXCR(void)
    {
    mtspr(SPRN_DEXCR, DEXCR_INIT);
    mtspr(SPRN_HASHKEYR, 0);
    }
//
// Note that we can be called twice of pseudo-PVRs.
// The parameter offset is not used.
//
#[no_mangle]
pub unsafe extern "C" fn __setup_cpu_power7(offset: c_ulong, t: *mut cpu_spec) {
    void __setup_cpu_power7(unsigned long offset, struct cpu_spec *t)
    {
    if (!init_hvmode_206(t))
    return;
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR), LPCR_LPES1 >> LPCR_LPES_SH);
    }
#[no_mangle]
pub unsafe extern "C" fn __restore_cpu_power7() {
    void __restore_cpu_power7(void)
    {
    u64 msr;
    msr = mfmsr();
    if (!(msr & MSR_HV))
    return;
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR), LPCR_LPES1 >> LPCR_LPES_SH);
    }
#[no_mangle]
pub unsafe extern "C" fn __setup_cpu_power8(offset: c_ulong, t: *mut cpu_spec) {
    void __setup_cpu_power8(unsigned long offset, struct cpu_spec *t)
    {
    init_FSCR();
    init_PMU();
    init_PMU_ISA207();
    if (!init_hvmode_206(t))
    return;
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR) | LPCR_PECEDH, 0); /* LPES = 0 */
    init_HFSCR();
    init_PMU_HV();
    init_PMU_HV_ISA207();
    }
#[no_mangle]
pub unsafe extern "C" fn __restore_cpu_power8() {
    void __restore_cpu_power8(void)
    {
    u64 msr;
    init_FSCR();
    init_PMU();
    init_PMU_ISA207();
    msr = mfmsr();
    if (!(msr & MSR_HV))
    return;
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA206(mfspr(SPRN_LPCR) | LPCR_PECEDH, 0); /* LPES = 0 */
    init_HFSCR();
    init_PMU_HV();
    init_PMU_HV_ISA207();
    }
#[no_mangle]
pub unsafe extern "C" fn __setup_cpu_power9(offset: c_ulong, t: *mut cpu_spec) {
    void __setup_cpu_power9(unsigned long offset, struct cpu_spec *t)
    {
    init_FSCR_power9();
    init_PMU();
    if (!init_hvmode_206(t))
    return;
    mtspr(SPRN_PSSCR, 0);
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_PID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA300((mfspr(SPRN_LPCR) | LPCR_PECEDH | LPCR_PECE_HVEE |\
    LPCR_HVICE | LPCR_HEIC) & ~(LPCR_UPRT | LPCR_HR), 0);
    init_HFSCR();
    init_PMU_HV();
    }
#[no_mangle]
pub unsafe extern "C" fn __restore_cpu_power9() {
    void __restore_cpu_power9(void)
    {
    u64 msr;
    init_FSCR_power9();
    init_PMU();
    msr = mfmsr();
    if (!(msr & MSR_HV))
    return;
    mtspr(SPRN_PSSCR, 0);
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_PID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA300((mfspr(SPRN_LPCR) | LPCR_PECEDH | LPCR_PECE_HVEE |\
    LPCR_HVICE | LPCR_HEIC) & ~(LPCR_UPRT | LPCR_HR), 0);
    init_HFSCR();
    init_PMU_HV();
    }
#[no_mangle]
pub unsafe extern "C" fn __setup_cpu_power10(offset: c_ulong, t: *mut cpu_spec) {
    void __setup_cpu_power10(unsigned long offset, struct cpu_spec *t)
    {
    init_FSCR_power10();
    init_PMU();
    init_PMU_ISA31();
    init_DEXCR();
    if (!init_hvmode_206(t))
    return;
    mtspr(SPRN_PSSCR, 0);
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_PID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA300((mfspr(SPRN_LPCR) | LPCR_PECEDH | LPCR_PECE_HVEE |\
    LPCR_HVICE | LPCR_HEIC) & ~(LPCR_UPRT | LPCR_HR), 0);
    init_HFSCR();
    init_PMU_HV();
    }
#[no_mangle]
pub unsafe extern "C" fn __restore_cpu_power10() {
    void __restore_cpu_power10(void)
    {
    u64 msr;
    init_FSCR_power10();
    init_PMU();
    init_PMU_ISA31();
    init_DEXCR();
    msr = mfmsr();
    if (!(msr & MSR_HV))
    return;
    mtspr(SPRN_PSSCR, 0);
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_PID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA300((mfspr(SPRN_LPCR) | LPCR_PECEDH | LPCR_PECE_HVEE |\
    LPCR_HVICE | LPCR_HEIC) & ~(LPCR_UPRT | LPCR_HR), 0);
    init_HFSCR();
    init_PMU_HV();
    }
#[no_mangle]
pub unsafe extern "C" fn __setup_cpu_power12(offset: c_ulong, t: *mut cpu_spec) {
    void __setup_cpu_power12(unsigned long offset, struct cpu_spec *t)
    {
    init_FSCR_power10();
    init_PMU();
    init_PMU_ISA31();
    if (!init_hvmode_206(t))
    return;
    mtspr(SPRN_PSSCR, 0);
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_PID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA300((mfspr(SPRN_LPCR) | LPCR_PECEDH | LPCR_PECE_HVEE |\
    LPCR_HVICE | LPCR_HEIC) & ~(LPCR_UPRT | LPCR_HR), 0);
    init_HFSCR();
    init_PMU_HV();
    }
#[no_mangle]
pub unsafe extern "C" fn __restore_cpu_power12() {
    void __restore_cpu_power12(void)
    {
    u64 msr;
    init_FSCR_power10();
    init_PMU();
    init_PMU_ISA31();
    msr = mfmsr();
    if (!(msr & MSR_HV))
    return;
    mtspr(SPRN_PSSCR, 0);
    mtspr(SPRN_LPID, 0);
    mtspr(SPRN_PID, 0);
    mtspr(SPRN_AMOR, ~0);
    mtspr(SPRN_PCR, PCR_MASK);
    init_LPCR_ISA300((mfspr(SPRN_LPCR) | LPCR_PECEDH | LPCR_PECE_HVEE |\
    LPCR_HVICE | LPCR_HEIC) & ~(LPCR_UPRT | LPCR_HR), 0);
    init_HFSCR();
    init_PMU_HV();
    }
