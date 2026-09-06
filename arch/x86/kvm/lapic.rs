//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/lapic.h
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

pub const KVM_APIC_INIT: c_int = 0;
pub const KVM_APIC_SIPI: c_int = 1;
pub const APIC_SHORT_MASK: c_uint = 0xc0000;
pub const APIC_DEST_NOSHORT: c_uint = 0x0;
pub const APIC_DEST_MASK: c_uint = 0x800;
pub const APIC_BUS_CYCLE_NS_DEFAULT: c_int = 1;
pub const APIC_BROADCAST: c_uint = 0xFF;
pub const X2APIC_BROADCAST: c_uint = 0xFFFFFFFFul;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lapic_mode {
    LAPIC_MODE_DISABLED = 0,
    LAPIC_MODE_INVALID = X2APIC_ENABLE,
    LAPIC_MODE_XAPIC = MSR_IA32_APICBASE_ENABLE,
    LAPIC_MODE_X2APIC = MSR_IA32_APICBASE_ENABLE | X2APIC_ENABLE,
}

//
// Track the mode of the optimized logical map, as the rules for decoding the
// destination vary per mode.  Enabling the optimized logical map requires all
// software-enabled local APIs to be in the same mode, each addressable APIC to
// be mapped to only one MDA, and each MDA to map to at most one APIC.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_apic_logical_mode {
// All local APICs are software disabled.
    KVM_APIC_MODE_SW_DISABLED,
// All software enabled local APICs in xAPIC cluster addressing mode.
    KVM_APIC_MODE_XAPIC_CLUSTER,
// All software enabled local APICs in xAPIC flat addressing mode.
    KVM_APIC_MODE_XAPIC_FLAT,
// All software enabled local APICs in x2APIC mode.
    KVM_APIC_MODE_X2APIC,
//
// Optimized map disabled, e.g. not all local APICs in the same logical
// mode, same logical ID assigned to multiple APICs, etc.
//
    KVM_APIC_MODE_MAP_DISABLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_apic_map {
    pub rcu: rcu_head,
    pub logical_mode: kvm_apic_logical_mode,
    pub max_apic_id: u32,
    pub xapic_flat_map: [*mut kvm_lapic; 8],
    pub xapic_cluster_map: [*mut kvm_lapic; 16][4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum lapic_lvt_entry {
    LVT_TIMER,
    LVT_THERMAL_MONITOR,
    LVT_PERFORMANCE_COUNTER,
    LVT_LINT0,
    LVT_LINT1,
    LVT_ERROR,
    LVT_CMCI,

    KVM_APIC_MAX_NR_LVT_ENTRIES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_timer {
    pub timer: hrtimer,
    pub /: *mut *mut s64 period; / unit: ns,
    pub target_expiration: ktime_t,
    pub timer_mode: u32,
    pub timer_mode_mask: u32,
    pub tscdeadline: u64,
    pub expired_tscdeadline: u64,
    pub timer_advance_ns: u32,
    pub /: *mut *mut atomic_t pending; / accumulated triggered timers,
    pub hv_timer_in_use: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_lapic {
    pub base_address: c_ulong,
    pub dev: kvm_io_device,
    pub lapic_timer: kvm_timer,
    pub divide_count: u32,
    pub vcpu: *mut kvm_vcpu,
    pub apicv_active: bool,
    pub sw_enabled: bool,
    pub irr_pending: bool,
    pub lvt0_in_nmi_mode: bool,
// Select registers in the vAPIC cannot be read/written.
    pub guest_apic_protected: bool,
// Number of bits set in ISR.
    pub isr_count: i16,
// The highest vector set in ISR; if -1 - invalid, must scan ISR.
    pub highest_isr_cache: c_int,
//
// APIC register page.  The layout matches the register layout seen by
// the guest 1:1, because it is accessed by the vmx microcode.
// Note: Only one register, the TPR, is used by the microcode.
//
    pub regs: *mut c_void,
    pub vapic_addr: gpa_t,
    pub vapic_cache: gfn_to_hva_cache,
    pub pending_events: c_ulong,
    pub sipi_vector: c_uint,
    pub nr_lvt_entries: c_int,
}

extern "C" {
    pub fn kvm_create_lapic(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_free_lapic(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_apic_has_interrupt(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_apic_ack_interrupt(vcpu: *mut kvm_vcpu, vector: c_int);
}
extern "C" {
    pub fn kvm_apic_accept_pic_intr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_apic_accept_events(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_lapic_reset(vcpu: *mut kvm_vcpu, init_event: bool);
}
extern "C" {
    pub fn kvm_lapic_get_cr8(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvm_lapic_set_tpr(vcpu: *mut kvm_vcpu, cr8: c_ulong);
}
extern "C" {
    pub fn kvm_lapic_update_cr8_intercept(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_lapic_set_eoi(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_apic_set_version(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_apic_after_set_mcg_cap(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_apic_clear_irr(vcpu: *mut kvm_vcpu, vec: c_int);
}
extern "C" {
    pub fn __kvm_apic_update_irr(pir: *mut c_ulong, regs: *mut c_void, max_irr: *mut c_int) -> bool;
}
extern "C" {
    pub fn kvm_apic_update_irr(vcpu: *mut kvm_vcpu, pir: *mut c_ulong, max_irr: *mut c_int) -> bool;
}
extern "C" {
    pub fn kvm_apic_update_ppr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_apic_local_deliver(apic: *mut kvm_lapic, lvt_type: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_apic_update_apicv(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_alloc_apic_access_page(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_inhibit_apic_access_page(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __kvm_irq_delivery_to_apic(_arg: kvm, _arg: src, _arg: irq, _arg: NULL) -> return;
}
extern "C" {
    pub fn kvm_apic_send_ipi(apic: *mut kvm_lapic, icr_low: u32, icr_high: u32);
}
extern "C" {
    pub fn kvm_apic_set_base(vcpu: *mut kvm_vcpu, value: u64, host_initiated: bool) -> c_int;
}
extern "C" {
    pub fn kvm_apic_get_state(vcpu: *mut kvm_vcpu, s: *mut kvm_lapic_state) -> c_int;
}
extern "C" {
    pub fn kvm_apic_set_state(vcpu: *mut kvm_vcpu, s: *mut kvm_lapic_state) -> c_int;
}
extern "C" {
    pub fn kvm_lapic_find_highest_irr(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_get_lapic_tscdeadline_msr(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvm_set_lapic_tscdeadline_msr(vcpu: *mut kvm_vcpu, data: u64);
}
extern "C" {
    pub fn kvm_apic_write_nodecode(vcpu: *mut kvm_vcpu, offset: u32);
}
extern "C" {
    pub fn kvm_apic_set_eoi_accelerated(vcpu: *mut kvm_vcpu, vector: c_int);
}
extern "C" {
    pub fn kvm_lapic_set_vapic_addr(vcpu: *mut kvm_vcpu, vapic_addr: gpa_t) -> c_int;
}
extern "C" {
    pub fn kvm_lapic_sync_from_vapic(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_lapic_sync_to_vapic(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_x2apic_icr_write_fast(apic: *mut kvm_lapic, data: u64) -> c_int;
}
extern "C" {
    pub fn kvm_x2apic_msr_write(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn kvm_x2apic_msr_read(vcpu: *mut kvm_vcpu, msr: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn kvm_hv_vapic_msr_write(vcpu: *mut kvm_vcpu, msr: u32, data: u64) -> c_int;
}
extern "C" {
    pub fn kvm_hv_vapic_msr_read(vcpu: *mut kvm_vcpu, msr: u32, data: *mut u64) -> c_int;
}
extern "C" {
    pub fn kvm_lapic_set_pv_eoi(vcpu: *mut kvm_vcpu, data: u64, len: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_lapic_exit();
}
extern "C" {
    pub fn kvm_x2apic_disable_read_intercept_reg_mask(vcpu: *mut kvm_vcpu) -> u64;
}
//
// irr_pending must be true if any interrupt is pending; set it after
// APIC_IRR to avoid race with apic_clear_irr
//
extern "C" {
    pub fn apic_get_reg(_arg: apic->regs, _arg: reg_off) -> return;
}
extern "C" {
    pub fn lapic_in_kernel(kvm_apic_hw_enabled(vcpu->arch.apic: vcpu) &&) -> return;
}
extern "C" {
    pub fn kvm_apic_present(kvm_apic_sw_enabled(vcpu->arch.apic: vcpu) &&) -> return;
}
extern "C" {
    pub fn lapic_in_kernel(test_bit(KVM_APIC_INIT: vcpu) &&, _arg: &vcpu->arch.apic->pending_events) -> return;
}
extern "C" {
    pub fn kvm_apic_pending_eoi(vcpu: *mut kvm_vcpu, vector: c_int) -> bool;
}
extern "C" {
    pub fn kvm_lapic_suppress_eoi_broadcast(apic: *mut kvm_lapic) -> bool;
}
extern "C" {
    pub fn kvm_wait_lapic_expire(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_lapic_switch_to_sw_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_lapic_switch_to_hv_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_lapic_expired_hv_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_lapic_hv_timer_in_use(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_lapic_restart_hv_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_apic_mode(_arg: vcpu->arch.apic_base) -> return;
}
