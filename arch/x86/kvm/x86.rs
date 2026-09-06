//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/x86.h
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

pub const KVM_MAX_MCE_BANKS: c_int = 32;
extern "C" {
    pub fn kvm_x86_vendor_init(ops: *mut kvm_x86_init_ops) -> c_int;
}
extern "C" {
    pub fn kvm_x86_vendor_exit();
}
extern "C" {
    pub fn kvm_spurious_fault();
}

// Sanity check the size of the memslot hash tables.
//
// Assert that "struct kvm_{svm,vmx,tdx}" is an order-0 or order-1 allocation.
// Spilling over to an order-2 allocation isn't fundamentally problematic, but
// isn't expected to happen in the foreseeable future (O(years)).  Assert that
// the size is an order-0 allocation when ignoring the memslot hash tables, to
// help detect and debug unexpected size increases.
//

pub const KVM_DEFAULT_PLE_GAP: c_int = 128;
pub const KVM_VMX_DEFAULT_PLE_WINDOW: c_int = 4096;
pub const KVM_DEFAULT_PLE_WINDOW_GROW: c_int = 2;
pub const KVM_DEFAULT_PLE_WINDOW_SHRINK: c_int = 0;

pub const KVM_SVM_DEFAULT_PLE_WINDOW: c_int = 3000;
extern "C" {
    pub fn min(_arg: ret, _arg: (u64)max) -> return;
}
extern "C" {
    pub fn max(_arg: val, _arg: min) -> return;
}
extern "C" {
    pub fn kvm_service_local_tlb_flush_requests(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_check_nested_events(vcpu: *mut kvm_vcpu) -> c_int;
}
// Forcibly leave the nested mode in cases like a vCPU reset
//
// If IBRS is advertised to the vCPU, KVM must flush the indirect branch
// predictors when transitioning from L2 to L1, as L1 expects hardware (KVM in
// this case) to provide separate predictor modes.  Bare metal isolates the host
// from the guest, but doesn't isolate different guests from one another (in
// this case L1 and L2). The exception is if bare metal supports same mode IBRS,
// which offers protection within the same mode, and hence protects L1 from L2.
//
// Disallow modifying CPUID and feature MSRs, which affect the core virtual CPU
// model exposed to the guest and virtualized by KVM, if the vCPU has already
// run or is in guest mode (L2).  In both cases, KVM has already consumed the
// current virtual CPU model, and doesn't support "unwinding" to react to the
// new model.
//
// Note, the only way is_guest_mode() can be true with 'last_vmentry_cpu == -1'
// is if userspace sets CPUID and feature MSRs (to enable VMX/SVM), then sets
// nested state, and then attempts to set CPUID and/or feature MSRs *again*.
//
// WARN if a nested VM-Enter is pending completion, and userspace hasn't gained
// control since the nested VM-Enter was initiated (in which case, userspace
// may have modified vCPU state to induce an architecturally invalid VM-Exit).
//
// x86 MSRs which contain linear addresses, x86 hidden segment bases, and
// IDT/GDT bases have static canonicality checks, the size of which depends
// only on the CPU's support for 5-level paging, rather than on the state of
// CR4.LA57.  This applies to both WRMSR and to other instructions that set
// their values, e.g. SGDT.
//
// KVM passes through most of these MSRS and also doesn't intercept the
// instructions that set the hidden segment bases.
//
// Because of this, to be consistent with hardware, even if the guest doesn't
// have LA57 enabled in its CPUID, perform canonicality checks based on *host
// support for 5 level paging.
//
// Finally, instructions which are related to MMU invalidation of a given
// linear address, also have a similar static canonical check on address.
// This allows for example to invalidate 5-level addresses of a guest from a
// host which uses 4-level paging.
//
extern "C" {
    pub fn is_noncanonical_address(_arg: la, _arg: vcpu, _arg: X86EMUL_F_MSR) -> return;
}
extern "C" {
    pub fn is_noncanonical_address(_arg: la, _arg: vcpu, _arg: X86EMUL_F_DT_LOAD) -> return;
}
extern "C" {
    pub fn is_noncanonical_address(_arg: la, _arg: vcpu, _arg: X86EMUL_F_INVLPG) -> return;
}
//
// If this is a shadow nested page table, the "GVA" is
// actually a nGPA.
//
// Clear the mmio cache info for the given gva. If gva is MMIO_GVA_ANY, we
// clear all mmio cache info.
//

//
// Use a raw write to set the per-CPU flag, as KVM will ensure a flush
// even if preemption is currently enabled..  If the current vCPU task
// is migrated to a different CPU (or userspace runs the vCPU on a
// different task) before the next VM-Entry, then kvm_arch_vcpu_load()
// will request a flush on the new CPU.
//

extern "C" {
    pub fn kvm_vcpu_reset(vcpu: *mut kvm_vcpu, init_event: bool);
}
extern "C" {
    pub fn kvm_inject_realmode_interrupt(vcpu: *mut kvm_vcpu, irq: c_int, inc_eip: c_int);
}
extern "C" {
    pub fn get_kvmclock_ns(kvm: *mut kvm) -> u64;
}
extern "C" {
    pub fn kvm_get_wall_clock_epoch(kvm: *mut kvm) -> u64;
}
extern "C" {
    pub fn kvm_get_monotonic_and_clockread(kernel_ns: *mut i64, tsc_timestamp: *mut u64) -> bool;
}
extern "C" {
    pub fn kvm_guest_time_update(v: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_synchronize_tsc(vcpu: *mut kvm_vcpu, user_value: *mut u64);
}
extern "C" {
    pub fn kvm_scale_tsc(tsc: u64, ratio: u64) -> u64;
}
extern "C" {
    pub fn kvm_read_l1_tsc(vcpu: *mut kvm_vcpu, host_tsc: u64) -> u64;
}
extern "C" {
    pub fn kvm_calc_nested_tsc_offset(l1_offset: u64, l2_offset: u64, l2_multiplier: u64) -> u64;
}
extern "C" {
    pub fn kvm_calc_nested_tsc_multiplier(l1_multiplier: u64, l2_multiplier: u64) -> u64;
}
extern "C" {
    pub fn kvm_compute_l1_tsc_offset(vcpu: *mut kvm_vcpu, target_tsc: u64) -> u64;
}
extern "C" {
    pub fn kvm_vcpu_write_tsc_offset(vcpu: *mut kvm_vcpu, l1_offset: u64);
}
extern "C" {
    pub fn handle_ud(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_exception_payload_quirk(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_fixup_and_inject_pf_error(vcpu: *mut kvm_vcpu, gva: gva_t, error_code: u16);
}
//
// EMULTYPE_NO_DECODE - Set when re-emulating an instruction (after completing
// userspace I/O) to indicate that the emulation context
// should be reused as is, i.e. skip initialization of
// emulation context, instruction fetch and decode.
//
// EMULTYPE_TRAP_UD - Set when emulating an intercepted #UD from hardware.
// Indicates that only select instructions (tagged with
// EmulateOnUD) should be emulated (to minimize the emulator
// attack surface).  See also EMULTYPE_TRAP_UD_FORCED.
//
// EMULTYPE_SKIP - Set when emulating solely to skip an instruction, i.e. to
// decode the instruction length.  For use *only* by
// kvm_x86_ops.skip_emulated_instruction() implementations if
// EMULTYPE_COMPLETE_USER_EXIT is not set.
//
// EMULTYPE_ALLOW_RETRY_PF - Set when the emulator should resume the guest to
// retry native execution under certain conditions,
// Can only be set in conjunction with EMULTYPE_PF.
//
// EMULTYPE_TRAP_UD_FORCED - Set when emulating an intercepted #UD that was
// triggered by KVM's magic "force emulation" prefix,
// which is opt in via module param (off by default).
// Bypasses EmulateOnUD restriction despite emulating
// due to an intercepted #UD (see EMULTYPE_TRAP_UD).
// Used to test the full emulator from userspace.
//
// EMULTYPE_VMWARE_GP - Set when emulating an intercepted #GP for VMware
// backdoor emulation, which is opt in via module param.
// VMware backdoor emulation handles select instructions
// and reinjects the #GP for all other cases.
//
// EMULTYPE_PF - Set when an intercepted #PF triggers the emulation, in which case
// the CR2/GPA value pass on the stack is valid.
//
// EMULTYPE_COMPLETE_USER_EXIT - Set when the emulator should update interruptibility
// state and inject single-step #DBs after skipping
// an instruction (after completing userspace I/O).
//
// EMULTYPE_WRITE_PF_TO_SP - Set when emulating an intercepted page fault that
// is attempting to write a gfn that contains one or
// more of the PTEs used to translate the write itself,
// and the owning page table is being shadowed by KVM.
// If emulation of the faulting instruction fails and
// this flag is set, KVM will exit to userspace instead
// of retrying emulation as KVM cannot make forward
// progress.
//
// If emulation fails for a write to guest page tables,
// KVM unprotects (zaps) the shadow page for the target
// gfn and resumes the guest to retry the non-emulatable
// instruction (on hardware).  Unprotecting the gfn
// doesn't allow forward progress for a self-changing
// access because doing so also zaps the translation for
// the gfn, i.e. retrying the instruction will hit a
// !PRESENT fault, which results in a new shadow page
// and sends KVM back to square one.
//
// EMULTYPE_SKIP_SOFT_INT - Set in combination with EMULTYPE_SKIP to only skip
// an instruction if it could generate a given software
// interrupt, which must be encoded via
// EMULTYPE_SET_SOFT_INT_VECTOR().
//

extern "C" {
    pub fn kvm_emulate_instruction(vcpu: *mut kvm_vcpu, emulation_type: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_prepare_emulation_failure_exit(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_prepare_event_vectoring_exit(vcpu: *mut kvm_vcpu, gpa: gpa_t);
}
extern "C" {
    pub fn kvm_prepare_unexpected_reason_exit(vcpu: *mut kvm_vcpu, exit_reason: u64);
}
extern "C" {
    pub fn handle_fastpath_hlt(vcpu: *mut kvm_vcpu) -> fastpath_t;
}
extern "C" {
    pub fn handle_fastpath_invd(vcpu: *mut kvm_vcpu) -> fastpath_t;
}
extern "C" {
    pub fn kvm_emulate_as_nop(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_invd(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_mwait(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_handle_invalid_op(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_monitor(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_fast_pio(vcpu: *mut kvm_vcpu, size: c_int, port: c_ushort, in: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_cpuid(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_halt(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_halt_noskip(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_ap_reset_hold(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_wbinvd(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_deliver_sipi_vector(vcpu: *mut kvm_vcpu, vector: u8);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_task_switch_reason {
    TASK_SWITCH_CALL = 0,
    TASK_SWITCH_IRET = 1,
    TASK_SWITCH_JMP = 2,
    TASK_SWITCH_GATE = 3,
}

extern "C" {
    pub fn __kvm_set_xcr(vcpu: *mut kvm_vcpu, index: u32, xcr: u64) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_xsetbv(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_emulate_rdpmc(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_skip_emulated_instruction(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_complete_insn_gp(vcpu: *mut kvm_vcpu, err: c_int) -> c_int;
}
extern "C" {
    pub fn kvm_queue_exception(vcpu: *mut kvm_vcpu, nr: unsigned);
}
extern "C" {
    pub fn kvm_queue_exception_e(vcpu: *mut kvm_vcpu, nr: unsigned, error_code: u32);
}
extern "C" {
    pub fn kvm_queue_exception_p(vcpu: *mut kvm_vcpu, nr: unsigned, payload: c_ulong);
}
extern "C" {
    pub fn kvm_require_dr(vcpu: *mut kvm_vcpu, dr: c_int) -> bool;
}
extern "C" {
    pub fn kvm_inject_nmi(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_get_nr_pending_nmis(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn memslot_rmap_alloc(slot: *mut kvm_memory_slot, npages: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_is_reset_bsp(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_vcpu_is_bsp(vcpu: *mut kvm_vcpu) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_apicv_inhibit {

//
// INHIBITs that are relevant to both Intel's APICv and AMD's AVIC.
//

//
// APIC acceleration is disabled by a module parameter
// and/or not supported in hardware.
//
    APICV_INHIBIT_REASON_DISABLED,

//
// APIC acceleration is inhibited because AutoEOI feature is
// being used by a HyperV guest.
//
    APICV_INHIBIT_REASON_HYPERV,

//
// APIC acceleration is inhibited because the userspace didn't yet
// enable the kernel/split irqchip.
//
    APICV_INHIBIT_REASON_ABSENT,

// APIC acceleration is inhibited because KVM_GUESTDBG_BLOCKIRQ
// (out of band, debug measure of blocking all interrupts on this vCPU)
// was enabled, to avoid AVIC/APICv bypassing it.
//
    APICV_INHIBIT_REASON_BLOCKIRQ,

//
// APICv is disabled because not all vCPUs have a 1:1 mapping between
// APIC ID and vCPU, _and_ KVM is not applying its x2APIC hotplug hack.
//
    APICV_INHIBIT_REASON_PHYSICAL_ID_ALIASED,

//
// For simplicity, the APIC acceleration is inhibited
// first time either APIC ID or APIC base are changed by the guest
// from their reset values.
//
    APICV_INHIBIT_REASON_APIC_ID_MODIFIED,
    APICV_INHIBIT_REASON_APIC_BASE_MODIFIED,

//
// INHIBITs that are relevant only to the AMD's AVIC.
//

//
// AVIC is inhibited on a vCPU because it runs a nested guest.
//
// This is needed because unlike APICv, the peers of this vCPU
// cannot use the doorbell mechanism to signal interrupts via AVIC when
// a vCPU runs nested.
//
    APICV_INHIBIT_REASON_NESTED,

//
// On SVM, the wait for the IRQ window is implemented with pending vIRQ,
// which cannot be injected when the AVIC is enabled, thus AVIC
// is inhibited while KVM waits for IRQ window.
//
    APICV_INHIBIT_REASON_IRQWIN,

//
// PIT (i8254) 're-inject' mode, relies on EOI intercept,
// which AVIC doesn't support for edge triggered interrupts.
//
    APICV_INHIBIT_REASON_PIT_REINJ,

//
// AVIC is disabled because SEV doesn't support it.
//
    APICV_INHIBIT_REASON_SEV,

//
// AVIC is disabled because not all vCPUs with a valid LDR have a 1:1
// mapping between logical ID and vCPU.
//
    APICV_INHIBIT_REASON_LOGICAL_ID_ALIASED,

//
// AVIC is disabled because the vCPU's APIC ID is beyond the max
// supported by AVIC/x2AVIC, i.e. the vCPU is unaddressable.
//
    APICV_INHIBIT_REASON_PHYSICAL_ID_TOO_BIG,

    NR_APICV_INHIBIT_REASONS,
}

extern "C" {
    pub fn kvm_apicv_activated(kvm: *mut kvm) -> bool;
}
extern "C" {
    pub fn kvm_vcpu_apicv_activated(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn __kvm_vcpu_update_apicv(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_inc_or_dec_irq_window_inhibit(kvm: *mut kvm, inc: bool);
}
extern "C" {
    pub fn kvm_make_scan_ioapic_request(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_setup_xss_caps();
}
//
// Get a filtered version of KVM's supported XCR0 that strips out dynamic
// features for which the current process doesn't (yet) have permission to use.
// This is intended to be used only when enumerating support to userspace,
// e.g. in KVM_GET_SUPPORTED_CPUID and KVM_CAP_XSAVE2, it does NOT need to be
// used to check/restrict guest behavior as KVM rejects KVM_SET_CPUID{2} if
// userspace attempts to enable unpermitted features.
//
// Treat XTILE_CFG as unsupported if the current process isn't
// allowed to use XTILE_DATA, as attempting to set XTILE_CFG in
// XCR0 without setting XTILE_DATA is architecturally illegal.
//
// Same "calling convention" as do_div:
// - divide (n << 32) by base
// - put result in n
// - return remainder
//

// 0, 1, 4, 5, 6, 7 are valid values.
extern "C" {
    pub fn __kvm_pv_async_pf_enabled(_arg: vcpu->arch.apf.msr_en_val) -> return;
}
extern "C" {
    pub fn kvm_find_async_pf_gfn(vcpu: *mut kvm_vcpu, gfn: gfn_t) -> bool;
}
//
// Trigger machine check on the host. We assume all the MSRs are already set up
// by the CPU and that we still run on the same CPU as the MCE occurred on.
// We pass a fake environment to the machine check handler because we want
// the guest to be always treated like user space, no matter what context
// it used internally.
//

extern "C" {
    pub fn kvm_invalidate_pcid(vcpu: *mut kvm_vcpu, pcid: c_ulong);
}
extern "C" {
    pub fn kvm_handle_invpcid(vcpu: *mut kvm_vcpu, type: c_ulong, gva: gva_t) -> c_int;
}

extern "C" {
    pub fn kvm_emulate_hypercall(vcpu: *mut kvm_vcpu) -> c_int;
}
