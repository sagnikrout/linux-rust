//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/kvm/s390/s390.h
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
// definition for kvm on s390
//
// Copyright IBM Corp. 2008, 2020
//
// Author(s): Carsten Otte <cotte@de.ibm.com>
// Christian Borntraeger <borntraeger@de.ibm.com>
// Christian Ehrhardt <ehrhardt@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_s390_quad {
    pub sixteen: __uint128_t,
    pub eight: c_ulong,
    pub four: c_uint,
    pub two: c_ushort,
    pub one: c_uchar,
}

// Transactional Memory Execution related macros

pub const TDB_FORMAT1: c_int = 1;

extern "C" {
    pub fn kvm_s390_test_cpuflags(_arg: vcpu, _arg: CPUSTAT_STOPPED) -> return;
}
extern "C" {
    pub fn test_bit(_arg: vcpu->vcpu_idx, _arg: vcpu->kvm->arch.idle_mask) -> return;
}

extern "C" {
    pub fn test_bit(_arg: GMAP_FLAG_IS_UCONTROL, _arg: &kvm->arch.gmap->flags) -> return;
}

pub const GUEST_PREFIX_SHIFT: c_int = 12;
pub const GUEST_PREFIX_MASK_ZARCH: c_uint = 0x7fffe;
pub const GUEST_PREFIX_MASK_ESA: c_uint = 0x7ffff;
// ar = base2;
// The displacement is a 20bit _SIGNED_ value
// ar = base1;
// address1 = (base1 ? vcpu->run->s.regs.gprs[base1] : 0) + disp1;
// address2 = (base2 ? vcpu->run->s.regs.gprs[base2] : 0) + disp2;
// ar_b1 = base1;
// ar_b2 = base2;
// r1 = (vcpu->arch.sie_block->ipb & 0x00f00000) >> 20;
// r2 = (vcpu->arch.sie_block->ipb & 0x000f0000) >> 16;
// The displacement is a 20bit _SIGNED_ value
// ar = base2;
// Set the condition code in the guest program status word
// test availability of facility in a kvm instance
// ptr |= (0x80UL >> (nr & 7));
extern "C" {
    pub fn test_bit_inv(_arg: nr, _arg: kvm->arch.cpu_feat) -> return;
}
// are cpu states controlled by user space
// get the end gfn of the last (highest gfn) memslot
// implemented in pv.c
extern "C" {
    pub fn kvm_s390_pv_destroy_cpu(vcpu: *mut kvm_vcpu, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_create_cpu(vcpu: *mut kvm_vcpu, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_set_aside(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_deinit_aside_vm(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_deinit_cleanup_all(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_deinit_vm(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_init_vm(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_set_cpu_state(vcpu: *mut kvm_vcpu, state: u8) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_dump_cpu(vcpu: *mut kvm_vcpu, buff: *mut c_void, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_destroy_page(kvm: *mut kvm, gaddr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_convert_to_secure(kvm: *mut kvm, gaddr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_s390_pv_make_secure(kvm: *mut kvm, gaddr: c_ulong, uvcb: *mut c_void) -> c_int;
}
//
// __kvm_s390_pv_destroy_page() - Destroy a guest page.
// @page: the page to destroy
//
// An attempt will be made to destroy the given guest page. If the attempt
// fails, an attempt is made to export the page. If both attempts fail, an
// appropriate error is returned.
//
// Context: must be called holding the mm lock for gmap->mm
//
// Large folios cannot be secure. Small folio implies FW_LEVEL_PTE.
//
// Fault handlers can race; it is possible that two CPUs will fault
// on the same secure page. One CPU can destroy the page, reboot,
// re-enter secure mode and import it, while the second CPU was
// stuck at the beginning of the handler. At some point the second
// CPU will be able to progress, and it will not be able to destroy
// the page. In that case we do not want to terminate the process,
// we instead try to export the page.
//
// implemented in interrupt.c
extern "C" {
    pub fn kvm_s390_handle_wait(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_vcpu_wakeup(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_idle_wakeup(timer: *mut hrtimer) -> hrtimer_restart;
}
extern "C" {
    pub fn kvm_s390_deliver_pending_interrupts(vcpu: *mut kvm_vcpu) -> int __must_check;
}
extern "C" {
    pub fn kvm_s390_clear_local_irqs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_clear_float_irqs(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_inject_vcpu(_arg: vcpu, _arg: &irq) -> return;
}
extern "C" {
    pub fn kvm_s390_inject_vcpu(_arg: vcpu, _arg: &irq) -> return;
}
extern "C" {
    pub fn kvm_s390_mask_adapter(kvm: *mut kvm, id: c_uint, masked: bool) -> c_int;
}
// implemented in intercept.c
extern "C" {
    pub fn kvm_s390_get_ilen(vcpu: *mut kvm_vcpu) -> u8;
}
extern "C" {
    pub fn kvm_handle_sie_intercept(vcpu: *mut kvm_vcpu) -> c_int;
}
// don't inject PER events if we re-execute the instruction
extern "C" {
    pub fn handle_sthyi(vcpu: *mut kvm_vcpu) -> c_int;
}
// implemented in priv.c
extern "C" {
    pub fn is_valid_psw(psw: *mut psw_t) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_aa(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_b2(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_e3(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_e5(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_01(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_b9(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_lpsw(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_stctl(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_lctl(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_eb(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_skey_check_enable(vcpu: *mut kvm_vcpu) -> c_int;
}
// implemented in vsie.c
extern "C" {
    pub fn kvm_s390_handle_vsie(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_vsie_kick(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_vsie_gmap_notifier(gmap: *mut gmap, start: gpa_t, end: gpa_t);
}
extern "C" {
    pub fn kvm_s390_vsie_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_vsie_destroy(kvm: *mut kvm);
}
// implemented in sigp.c
extern "C" {
    pub fn kvm_s390_handle_sigp(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_sigp_pei(vcpu: *mut kvm_vcpu) -> c_int;
}
// implemented in s390.c
extern "C" {
    pub fn kvm_s390_try_set_tod_clock(kvm: *mut kvm, gtod: *const kvm_s390_vm_tod_clock) -> c_int;
}
extern "C" {
    pub fn kvm_s390_store_status_unloaded(vcpu: *mut kvm_vcpu, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_s390_vcpu_store_status(vcpu: *mut kvm_vcpu, addr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_s390_vcpu_start(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_vcpu_stop(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_vcpu_block(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_vcpu_unblock(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_vcpu_sie_inhibited(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn exit_sie(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_sync_request(req: c_int, vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_vcpu_setup_cmma(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_vcpu_unsetup_cmma(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_set_cpu_timer(vcpu: *mut kvm_vcpu, cputm: __u64);
}
extern "C" {
    pub fn kvm_s390_get_cpu_timer(vcpu: *mut kvm_vcpu) -> __u64;
}
extern "C" {
    pub fn kvm_s390_cpus_from_pv(kvm: *mut kvm, rc: *mut u16, rrc: *mut u16) -> c_int;
}
extern "C" {
    pub fn kvm_arch_setup_async_pf(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_s390_update_cmma_dirty(kvm: *mut kvm, old: *mut kvm_memory_slot);
}
extern "C" {
    pub fn kvm_s390_vm_stop_migration(kvm: *mut kvm) -> c_int;
}
// implemented in diag.c
extern "C" {
    pub fn kvm_s390_handle_diag(vcpu: *mut kvm_vcpu) -> c_int;
}
//
// kvm_s390_inject_prog_cond - conditionally inject a program check
// @vcpu: virtual cpu
// @rc: original return/error code
//
// This function is supposed to be used after regular guest access functions
// failed, to conditionally inject a program check to a vcpu. The typical
// pattern would look like
//
// rc = write_guest(vcpu, addr, data, len);
// if (rc)
// return kvm_s390_inject_prog_cond(vcpu, rc);
//
// A negative return code from guest access functions implies an internal error
// like e.g. out of memory. In these cases no program check should be injected
// to the guest.
// A positive value implies that an exception happened while accessing a guest's
// memory. In this case all data belonging to the corresponding program check
// has been stored in vcpu->arch.pgm and can be injected with
// kvm_s390_inject_prog_irq().
//
// Returns: - the original @rc value if @rc was negative (internal error)
// - zero if @rc was already zero
// - zero or error code from injecting if @rc was positive
// (program check injected to @vcpu)
//
extern "C" {
    pub fn kvm_s390_inject_prog_irq(_arg: vcpu, _arg: &vcpu->arch.pgm) -> return;
}
// implemented in interrupt.c
extern "C" {
    pub fn kvm_s390_vcpu_has_irq(vcpu: *mut kvm_vcpu, exclude_stop: c_int) -> c_int;
}
extern "C" {
    pub fn psw_extint_disabled(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_destroy_adapters(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_ext_call_pending(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_is_stop_irq_pending(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_is_restart_irq_pending(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_clear_stop_irq(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_gisa_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_gisa_clear(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_gisa_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_gisa_disable(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_gisa_enable(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_s390_gib_init(nisc: u8) -> int __init;
}
extern "C" {
    pub fn kvm_s390_gib_destroy();
}
extern "C" {
    pub fn kvm_s390_unmap_all_adapters(kvm: *mut kvm);
}
// implemented in guestdbg.c
extern "C" {
    pub fn kvm_s390_backup_guest_per_regs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_restore_guest_per_regs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_patch_guest_per_regs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_clear_bp_data(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_prepare_debug_exit(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_s390_handle_per_ifetch_icpt(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_s390_handle_per_event(vcpu: *mut kvm_vcpu) -> c_int;
}
//
// Without SIGP interpretation, only SRS interpretation (if available)
// might use the entries. By not setting the entries and keeping them
// invalid, hardware will not access them but intercept.
//
extern "C" {
    pub fn test_facility(TEID_FSI_STORE: 75) && (current->thread.gmap_teid.fsi ==) -> return;
}
//
// kvm_s390_vcpu_crypto_reset_all
//
// Reset the crypto attributes for each vcpu. This can be done while the vcpus
// are running as each vcpu will be removed from SIE before resetting the crypt
// attributes and restored to SIE afterward.
//
// Note: The kvm->lock must be held while calling this function
//
// @kvm: the KVM guest
//
extern "C" {
    pub fn kvm_s390_vcpu_crypto_reset_all(kvm: *mut kvm);
}
//
// kvm_s390_vcpu_pci_enable_interp
//
// Set the associated PCI attributes for each vcpu to allow for zPCI Load/Store
// interpretation as well as adapter interruption forwarding.
//
// @kvm: the KVM guest
//
extern "C" {
    pub fn kvm_s390_vcpu_pci_enable_interp(kvm: *mut kvm);
}
//
// diag9c_forwarding_hz
//
// Set the maximum number of diag9c forwarding per second
//
