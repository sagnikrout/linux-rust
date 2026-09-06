//! Automatically rewritten from C to Rust
//! Source: arch/arm64/kvm/hyp/vgic-v2-cpuif-proxy.c
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
// Copyright (C) 2012-2015 - ARM Ltd
// Author: Marc Zyngier <marc.zyngier@arm.com>
//

#[no_mangle]
unsafe extern "C" fn __is_be(vcpu: *mut kvm_vcpu) -> bool {
    static bool __is_be(struct kvm_vcpu *vcpu)
    {
    if (vcpu_mode_is_32bit(vcpu))
    return !!(read_sysreg_el2(SYS_SPSR) & PSR_AA32_E_BIT);
    return !!(read_sysreg_el1(SYS_SCTLR) & SCTLR_ELx_EE);
    }
//
// __vgic_v2_perform_cpuif_access -- perform a GICV access on behalf of the
// guest.
//
// @vcpu: the offending vcpu
//
// Returns:
// 1: GICV access successfully performed
// 0: Not a GICV access
// -1: Illegal GICV access successfully performed
//
#[no_mangle]
pub unsafe extern "C" fn __vgic_v2_perform_cpuif_access(vcpu: *mut kvm_vcpu) -> c_int {
    int __vgic_v2_perform_cpuif_access(struct kvm_vcpu *vcpu)
    {
    struct kvm *kvm = kern_hyp_va(vcpu.kvm);
    struct vgic_dist *vgic = &kvm.arch.vgic;
    phys_addr_t fault_ipa;
    void __iomem *addr;
    int rd;
// Build the full address
    fault_ipa  = kvm_vcpu_get_fault_ipa(vcpu);
    fault_ipa |= FAR_TO_FIPA_OFFSET(kvm_vcpu_get_hfar(vcpu));
// If not for GICV, move on
    if (fault_ipa <  vgic.vgic_cpu_base ||
    fault_ipa >= (vgic.vgic_cpu_base + KVM_VGIC_V2_CPU_SIZE))
    return 0;
// Reject anything but a 32bit access
    if (kvm_vcpu_dabt_get_as(vcpu) != sizeof(u32)) {
    __kvm_skip_instr(vcpu);
    return -1;
    }
// Not aligned? Don't bother
    if (fault_ipa & 3) {
    __kvm_skip_instr(vcpu);
    return -1;
    }
// Handle deactivation as a normal exit
    if ((fault_ipa - vgic.vgic_cpu_base) >= GIC_CPU_DEACTIVATE)
    return 0;
    rd = kvm_vcpu_dabt_get_rd(vcpu);
    addr  = kvm_vgic_global_state.vcpu_hyp_va;
    addr += fault_ipa - vgic.vgic_cpu_base;
    if (kvm_vcpu_dabt_iswrite(vcpu)) {
    let mut data: u32 = vcpu_get_reg(vcpu, rd);
    if (__is_be(vcpu)) {
// guest pre-swabbed data, undo this for writel()
    data = __kvm_swab32(data);
    }
    writel_relaxed(data, addr);
    } else {
    let mut data: u32 = readl_relaxed(addr);
    if (__is_be(vcpu)) {
// guest expects swabbed data
    data = __kvm_swab32(data);
    }
    vcpu_set_reg(vcpu, rd, data);
    }
    __kvm_skip_instr(vcpu);
    return 1;
    }
