//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/vmid.c
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
// Copyright (C) 2019 Western Digital Corporation or its affiliates.
//
// Authors:
// Anup Patel <anup.patel@wdc.com>
//

    let mut vmid_version: static unsigned long = 1;
    static unsigned long vmid_next;
    static unsigned long vmid_bits __ro_after_init;
    static DEFINE_SPINLOCK(vmid_lock);
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_gstage_vmid_detect() -> void __init {
    void __init kvm_riscv_gstage_vmid_detect(void)
    {
// Figure-out number of VMID bits in HW
    csr_write(CSR_HGATP, (kvm_riscv_gstage_mode(kvm_riscv_gstage_max_pgd_levels) <<
    HGATP_MODE_SHIFT) | HGATP_VMID);
    vmid_bits = csr_read(CSR_HGATP);
    vmid_bits = (vmid_bits & HGATP_VMID) >> HGATP_VMID_SHIFT;
    vmid_bits = fls_long(vmid_bits);
    csr_write(CSR_HGATP, 0);
// We polluted local TLB so flush all guest TLB
    kvm_riscv_local_hfence_gvma_all();
// We don't use VMID bits if they are not sufficient
    if ((1UL << vmid_bits) < num_possible_cpus())
    vmid_bits = 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_gstage_vmid_bits() -> c_ulong {
    unsigned long kvm_riscv_gstage_vmid_bits(void)
    {
    return vmid_bits;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_gstage_vmid_init(kvm: *mut kvm) -> c_int {
    int kvm_riscv_gstage_vmid_init(struct kvm *kvm)
    {
// Mark the initial VMID and VMID version invalid
    kvm.arch.vmid.vmid_version = 0;
    kvm.arch.vmid.vmid = 0;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_gstage_vmid_ver_changed(vmid: *mut kvm_vmid) -> bool {
    bool kvm_riscv_gstage_vmid_ver_changed(struct kvm_vmid *vmid)
    {
    if (!vmid_bits)
    return false;
    return unlikely(READ_ONCE(vmid.vmid_version) !=
    READ_ONCE(vmid_version));
    }
#[no_mangle]
unsafe extern "C" fn __local_hfence_gvma_all(info: *mut c_void) {
    static void __local_hfence_gvma_all(void *info)
    {
    kvm_riscv_local_hfence_gvma_all();
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_gstage_vmid_update(vcpu: *mut kvm_vcpu) {
    void kvm_riscv_gstage_vmid_update(struct kvm_vcpu *vcpu)
    {
    unsigned long i;
    struct kvm_vcpu *v;
    struct kvm_vmid *vmid = &vcpu.kvm.arch.vmid;
    if (!kvm_riscv_gstage_vmid_ver_changed(vmid))
    return;
    spin_lock(&vmid_lock);
//
// We need to re-check the vmid_version here to ensure that if
// another vcpu already allocated a valid vmid for this vm.
//
    if (!kvm_riscv_gstage_vmid_ver_changed(vmid)) {
    spin_unlock(&vmid_lock);
    return;
    }
// First user of a new VMID version?
    if (unlikely(vmid_next == 0)) {
    WRITE_ONCE(vmid_version, READ_ONCE(vmid_version) + 1);
    vmid_next = 1;
//
// We ran out of VMIDs so we increment vmid_version and
// start assigning VMIDs from 1.
//
// This also means existing VMIDs assignment to all Guest
// instances is invalid and we have force VMID re-assignement
// for all Guest instances. The Guest instances that were not
// running will automatically pick-up new VMIDs because will
// call kvm_riscv_gstage_vmid_update() whenever they enter
// in-kernel run loop. For Guest instances that are already
// running, we force VM exits on all host CPUs using IPI and
// flush all Guest TLBs.
//
    on_each_cpu_mask(cpu_online_mask, __local_hfence_gvma_all,
    core::ptr::null_mut(), 1);
    }
    vmid.vmid = vmid_next;
    vmid_next++;
    vmid_next &= (1 << vmid_bits) - 1;
    WRITE_ONCE(vmid.vmid_version, READ_ONCE(vmid_version));
    spin_unlock(&vmid_lock);
// Request G-stage page table update for all VCPUs
    kvm_for_each_vcpu(i, v, vcpu.kvm)
    kvm_make_request(KVM_REQ_UPDATE_HGATP, v);
    }
