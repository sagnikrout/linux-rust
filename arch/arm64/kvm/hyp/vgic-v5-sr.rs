//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/vgic-v5-sr.c
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
//
// Copyright (C) 2025, 2026 - Arm Ltd
//

#[no_mangle]
pub unsafe extern "C" fn __vgic_v5_save_apr(cpu_if: *mut vgic_v5_cpu_if) {
    void __vgic_v5_save_apr(struct vgic_v5_cpu_if *cpu_if)
    {
    cpu_if.vgic_apr = read_sysreg_s(SYS_ICH_APR_EL2);
    }
#[no_mangle]
unsafe extern "C" fn __vgic_v5_compat_mode_disable() {
    static void  __vgic_v5_compat_mode_disable(void)
    {
    sysreg_clear_set_s(SYS_ICH_VCTLR_EL2, ICH_VCTLR_EL2_V3, 0);
    isb();
    }
#[no_mangle]
pub unsafe extern "C" fn __vgic_v5_restore_vmcr_apr(cpu_if: *mut vgic_v5_cpu_if) {
    void __vgic_v5_restore_vmcr_apr(struct vgic_v5_cpu_if *cpu_if)
    {
    __vgic_v5_compat_mode_disable();
    write_sysreg_s(cpu_if.vgic_vmcr, SYS_ICH_VMCR_EL2);
    write_sysreg_s(cpu_if.vgic_apr, SYS_ICH_APR_EL2);
    }
#[no_mangle]
pub unsafe extern "C" fn __vgic_v5_save_ppi_state(cpu_if: *mut vgic_v5_cpu_if) {
    void __vgic_v5_save_ppi_state(struct vgic_v5_cpu_if *cpu_if)
    {
//
// The following code assumes that the bitmap storage that we have for
// PPIs is either 64 (architected PPIs, only).
//
    BUILD_BUG_ON(VGIC_V5_NR_PRIVATE_IRQS != 64);
    bitmap_write(host_data_ptr(vgic_v5_ppi_state).activer_exit,
    read_sysreg_s(SYS_ICH_PPI_ACTIVER0_EL2), 0, 64);
    bitmap_write(host_data_ptr(vgic_v5_ppi_state).pendr,
    read_sysreg_s(SYS_ICH_PPI_PENDR0_EL2), 0, 64);
    cpu_if.vgic_ppi_priorityr[0] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR0_EL2);
    cpu_if.vgic_ppi_priorityr[1] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR1_EL2);
    cpu_if.vgic_ppi_priorityr[2] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR2_EL2);
    cpu_if.vgic_ppi_priorityr[3] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR3_EL2);
    cpu_if.vgic_ppi_priorityr[4] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR4_EL2);
    cpu_if.vgic_ppi_priorityr[5] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR5_EL2);
    cpu_if.vgic_ppi_priorityr[6] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR6_EL2);
    cpu_if.vgic_ppi_priorityr[7] = read_sysreg_s(SYS_ICH_PPI_PRIORITYR7_EL2);
// Now that we are done, disable DVI
    write_sysreg_s(0, SYS_ICH_PPI_DVIR0_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_DVIR1_EL2);
    }
#[no_mangle]
pub unsafe extern "C" fn __vgic_v5_restore_ppi_state(cpu_if: *mut vgic_v5_cpu_if) {
    void __vgic_v5_restore_ppi_state(struct vgic_v5_cpu_if *cpu_if)
    {
    DECLARE_BITMAP(pendr, VGIC_V5_NR_PRIVATE_IRQS);
// Enable DVI so that the guest's interrupt config takes over
    write_sysreg_s(bitmap_read(cpu_if.vgic_ppi_dvir, 0, 64),
    SYS_ICH_PPI_DVIR0_EL2);
    write_sysreg_s(bitmap_read(cpu_if.vgic_ppi_activer, 0, 64),
    SYS_ICH_PPI_ACTIVER0_EL2);
    write_sysreg_s(bitmap_read(cpu_if.vgic_ppi_enabler, 0, 64),
    SYS_ICH_PPI_ENABLER0_EL2);
// Update the pending state of the NON-DVI'd PPIs, only
    bitmap_andnot(pendr, host_data_ptr(vgic_v5_ppi_state).pendr,
    cpu_if.vgic_ppi_dvir, VGIC_V5_NR_PRIVATE_IRQS);
    write_sysreg_s(bitmap_read(pendr, 0, 64), SYS_ICH_PPI_PENDR0_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[0],
    SYS_ICH_PPI_PRIORITYR0_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[1],
    SYS_ICH_PPI_PRIORITYR1_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[2],
    SYS_ICH_PPI_PRIORITYR2_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[3],
    SYS_ICH_PPI_PRIORITYR3_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[4],
    SYS_ICH_PPI_PRIORITYR4_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[5],
    SYS_ICH_PPI_PRIORITYR5_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[6],
    SYS_ICH_PPI_PRIORITYR6_EL2);
    write_sysreg_s(cpu_if.vgic_ppi_priorityr[7],
    SYS_ICH_PPI_PRIORITYR7_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_DVIR1_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_ACTIVER1_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_ENABLER1_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PENDR1_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR8_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR9_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR10_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR11_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR12_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR13_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR14_EL2);
    write_sysreg_s(0, SYS_ICH_PPI_PRIORITYR15_EL2);
    }
#[no_mangle]
pub unsafe extern "C" fn __vgic_v5_save_state(cpu_if: *mut vgic_v5_cpu_if) {
    void __vgic_v5_save_state(struct vgic_v5_cpu_if *cpu_if)
    {
    cpu_if.vgic_vmcr = read_sysreg_s(SYS_ICH_VMCR_EL2);
    cpu_if.vgic_icsr = read_sysreg_s(SYS_ICC_ICSR_EL1);
    }
#[no_mangle]
pub unsafe extern "C" fn __vgic_v5_restore_state(cpu_if: *mut vgic_v5_cpu_if) {
    void __vgic_v5_restore_state(struct vgic_v5_cpu_if *cpu_if)
    {
    write_sysreg_s(cpu_if.vgic_icsr, SYS_ICC_ICSR_EL1);
    }
