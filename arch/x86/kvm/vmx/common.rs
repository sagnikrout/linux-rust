//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/common.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub union vmx_exit_reason {
    pub 16: u32 basic :,
    pub 1: u32 reserved16 :,
    pub 1: u32 reserved17 :,
    pub 1: u32 reserved18 :,
    pub 1: u32 reserved19 :,
    pub 1: u32 reserved20 :,
    pub 1: u32 reserved21 :,
    pub 1: u32 reserved22 :,
    pub 1: u32 reserved23 :,
    pub 1: u32 reserved24 :,
    pub 1: u32 reserved25 :,
    pub 1: u32 bus_lock_detected :,
    pub 1: u32 enclave_mode :,
    pub 1: u32 smi_pending_mtf :,
    pub 1: u32 smi_from_vmx_root :,
    pub 1: u32 reserved30 :,
    pub 1: u32 failed_vmentry :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vcpu_vt {
// Posted interrupt descriptor
    pub pi_desc: pi_desc,
// Used if this vCPU is waiting for PI notification wakeup.
    pub pi_wakeup_list: list_head,
    pub exit_reason: vmx_exit_reason,
    pub exit_qualification: c_ulong,
    pub exit_intr_info: u32,
//
// If true, guest state has been loaded into hardware, and host state
// saved into vcpu_{vt,vmx,tdx}.  If false, host state is loaded into
// hardware.
//
    pub guest_state_loaded: bool,
    pub emulation_required: bool,

    pub msr_host_kernel_gs_base: u64,

}

extern "C" {
    pub fn is_td(_arg: vcpu->kvm) -> return;
}

// For TDX the direct mask is the shared mask.
// Is it a write fault?
// Is it a fetch fault?
// ept page table entry is present?
extern "C" {
    pub fn kvm_mmu_page_fault(_arg: vcpu, _arg: gpa, _arg: error_code, _arg: NULL, _arg: 0) -> return;
}

//
// The vector of the virtual has already been set in the PIR.
// Send a notification event to deliver the virtual interrupt
// unless the vCPU is the currently running vCPU, i.e. the
// event is being sent from a fastpath VM-Exit handler, in
// which case the PIR will be synced to the vIRR before
// re-entering the guest.
//
// When the target is not the running vCPU, the following
// possibilities emerge:
//
// Case 1: vCPU stays in non-root mode. Sending a notification
// event posts the interrupt to the vCPU.
//
// Case 2: vCPU exits to root mode and is still runnable. The
// PIR will be synced to the vIRR before re-entering the guest.
// Sending a notification event is ok as the host IRQ handler
// will ignore the spurious event.
//
// Case 3: vCPU exits to root mode and is blocked. vcpu_block()
// has already synced PIR to vIRR and never blocks the vCPU if
// the vIRR is not empty. Therefore, a blocked vCPU here does
// not wait for any requested interrupts in PIR, and sending a
// notification event also results in a benign, spurious event.
//

//
// The vCPU isn't in the guest; wake the vCPU in case it is blocking,
// otherwise do nothing as KVM will grab the highest priority pending
// IRQ via ->sync_pir_to_irr() in vcpu_enter_guest().
//
// Post an interrupt to a vCPU's PIR and trigger the vCPU to process the
// interrupt if necessary.
//
// If a previous notification has sent the IPI, nothing to do.
//
// The implied barrier in pi_test_and_set_on() pairs with the smp_mb_*()
// after setting vcpu->mode in vcpu_enter_guest(), thus the vCPU is
// guaranteed to see PID.ON=1 and sync the PIR to IRR if triggering a
// posted interrupt "fails" because vcpu->mode != IN_GUEST_MODE.
//
extern "C" {
    pub fn vmx_handle_nmi(vcpu: *mut kvm_vcpu) -> noinstr void;
}
