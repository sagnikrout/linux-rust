//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/coco/sev/internal.h
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
pub const DR7_RESET_VALUE: c_uint = 0x400;
// #VC handler runtime per-CPU data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sev_es_runtime_data {
    pub ghcb_page: ghcb,
//
// Reserve one page per CPU as backup storage for the unencrypted GHCB.
// It is needed when an NMI happens while the #VC handler uses the real
// GHCB, and the NMI handler itself is causing another #VC exception. In
// that case the GHCB content of the first handler needs to be backed up
// and restored.
//
    pub backup_ghcb: ghcb,
//
// Mark the per-cpu GHCBs as in-use to detect nested #VC exceptions.
// There is no need for it to be atomic, because nothing is written to
// the GHCB between the read and the write of ghcb_active. So it is safe
// to use it when a nested #VC exception happens before the write.
//
// This is necessary for example in the #VC->NMI->#VC case when the NMI
// happens while the first #VC handler uses the GHCB. When the NMI code
// raises a second #VC handler it might overwrite the contents of the
// GHCB written by the first handler. To avoid this the content of the
// GHCB is saved and restored when the GHCB is detected to be in use
// already.
//
    pub ghcb_active: bool,
    pub backup_ghcb_active: bool,
//
// Cached DR7 value - write it on DR7 writes and return it on reads.
// That value will never make it to the real hardware DR7 as debugging
// is currently unsupported in SEV-ES guests.
//
    pub dr7: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ghcb_state {
    pub ghcb: *mut ghcb,
}

extern "C" {
    pub fn __sev_put_ghcb(state: *mut ghcb_state);
}
extern "C" {
    pub fn verify_exception_info(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result;
}
extern "C" {
    pub fn vc_forward_exception(ctxt: *mut es_em_ctxt);
}
extern "C" {
    pub fn svsm_pval_pages(desc: *mut snp_psc_desc);
}
extern "C" {
    pub fn svsm_perform_call_protocol(call: *mut svsm_call) -> c_int;
}
extern "C" {
    pub fn snp_svsm_vtpm_probe() -> bool;
}
extern "C" {
    pub fn kernel_exc_vmm_communication(regs: *mut pt_regs, error_code: c_ulong) -> noinstr void;
}
extern "C" {
    pub fn user_exc_vmm_communication(regs: *mut pt_regs, error_code: c_ulong) -> noinstr void;
}
extern "C" {
    pub fn native_rdmsrq(_arg: MSR_AMD64_SEV_ES_GHCB) -> return;
}
extern "C" {
    pub fn __vc_handle_msr(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt, write: bool) -> es_result;
}
extern "C" {
    pub fn get_hv_features() -> u64;
}
extern "C" {
    pub fn this_cpu_read(_arg: svsm_caa) -> return;
}
extern "C" {
    pub fn rip_rel_ptr(_arg: &boot_svsm_ca_page) -> return;
}
extern "C" {
    pub fn this_cpu_read(_arg: svsm_caa_pa) -> return;
}
