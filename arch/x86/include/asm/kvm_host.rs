//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/kvm_host.h
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
// Kernel-based Virtual Machine driver for Linux
//
// This header defines architecture specific interfaces, x86 version
//

//
// CONFIG_KVM_MAX_NR_VCPUS is defined iff CONFIG_KVM!=n, provide a dummy max if
// KVM is disabled (arbitrarily use the default from CONFIG_KVM_MAX_NR_VCPUS).
//

pub const KVM_MAX_VCPUS: c_int = 1024;

//
// In x86, the VCPU ID corresponds to the APIC ID, and APIC IDs
// might be larger than the actual number of VCPUs because the
// APIC ID encodes CPU topology information.
//
// In the worst case, we'll need less than one extra bit for the
// Core ID, and less than one extra bit for the Package (Die) ID,
// so ratio of 4 should be enough.
//
pub const KVM_VCPU_ID_RATIO: c_int = 4;

// memory slots that are not exposed to userspace
pub const KVM_INTERNAL_MEM_SLOTS: c_int = 3;
pub const KVM_HALT_POLL_NS_DEFAULT: c_int = 200000;

// x86-specific vcpu->requests bit members

// KVM Hugepage definitions for x86

pub const KVM_MAX_CPUID_ENTRIES: c_int = 256;
pub const KVM_NR_VAR_MTRR: c_int = 8;
pub const ASYNC_PF_PER_VCPU: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_reg {
    VCPU_REGS_RAX = __VCPU_REGS_RAX,
    VCPU_REGS_RCX = __VCPU_REGS_RCX,
    VCPU_REGS_RDX = __VCPU_REGS_RDX,
    VCPU_REGS_RBX = __VCPU_REGS_RBX,
    VCPU_REGS_RSP = __VCPU_REGS_RSP,
    VCPU_REGS_RBP = __VCPU_REGS_RBP,
    VCPU_REGS_RSI = __VCPU_REGS_RSI,
    VCPU_REGS_RDI = __VCPU_REGS_RDI,

    VCPU_REGS_R8  = 8,
    VCPU_REGS_R9,
    VCPU_REGS_R10,
    VCPU_REGS_R11,
    VCPU_REGS_R12,
    VCPU_REGS_R13,
    VCPU_REGS_R14,
    VCPU_REGS_R15,

    NR_VCPU_GENERAL_PURPOSE_REGS,

    VCPU_REG_RIP = NR_VCPU_GENERAL_PURPOSE_REGS,

    VCPU_REG_PDPTR,
    VCPU_REG_CR0,
//
// Alias AMD's ERAPS (not a real register) to CR3 so that common code
// can trigger emulation of the RAP (Return Address Predictor) with
// minimal support required in common code.  Piggyback CR3 as the RAP
// is cleared on writes to CR3, i.e. marking CR3 dirty will naturally
// mark ERAPS dirty as well.
//
    VCPU_REG_CR3,
    VCPU_REG_ERAPS = VCPU_REG_CR3,
    VCPU_REG_CR4,
    VCPU_REG_RFLAGS,
    VCPU_REG_SEGMENTS,
    VCPU_REG_EXIT_INFO_1,
    VCPU_REG_EXIT_INFO_2,

    NR_VCPU_TOTAL_REGS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum exit_fastpath_completion {
    EXIT_FASTPATH_NONE,
    EXIT_FASTPATH_REENTER_GUEST,
    EXIT_FASTPATH_EXIT_HANDLED,
    EXIT_FASTPATH_EXIT_USERSPACE,
}

pub type fastpath_t = exit_fastpath_completion;
pub const KVM_NR_DB_REGS: c_int = 4;

//
// IMPLICIT_ACCESS is a KVM-defined flag used to correctly perform SMAP checks
// when emulating instructions that triggers implicit access.
//

//
// PRIVATE_ACCESS is a KVM-defined flag us to indicate that a fault occurred
// when the guest was accessing private memory.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_caps {
// control of guest tsc rate supported?
    pub has_tsc_control: bool,
// maximum supported tsc_khz for guests
    pub max_guest_tsc_khz: u32,
// number of bits of the fractional part of the TSC scaling ratio
    pub tsc_scaling_ratio_frac_bits: u8,
// maximum allowed value of TSC scaling ratio
    pub max_tsc_scaling_ratio: u64,
// 1ull << kvm_caps.tsc_scaling_ratio_frac_bits
    pub default_tsc_scaling_ratio: u64,
// bus lock detection supported?
    pub has_bus_lock_exit: bool,
// notify VM exit supported?
    pub has_notify_vmexit: bool,
// bit mask of VM types
    pub supported_vm_types: u32,
    pub supported_mce_cap: u64,
    pub supported_xcr0: u64,
    pub supported_xss: u64,
    pub supported_perf_cap: u64,
    pub supported_efer_bits: u64,
    pub supported_quirks: u64,
    pub inapplicable_quirks: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_host_values {
//
// The host's raw MAXPHYADDR, i.e. the number of non-reserved physical
// address bits irrespective of features that repurpose legal bits,
// e.g. MKTME.
//
    pub maxphyaddr: u8,
    pub efer: u64,
    pub xcr0: u64,
    pub xss: u64,
    pub s_cet: u64,
    pub arch_capabilities: u64,
}

//
// kvm_mmu_page_role tracks the properties of a shadow page (where shadow page
// also includes TDP pages) to determine whether or not a page can be used in
// the given MMU context.  This is a subset of the overall kvm_cpu_role to
// minimize the size of kvm_memory_slot.arch.gfn_write_track, i.e. allows
// allocating 2 bytes per gfn instead of 4 bytes per gfn.
//
// Upper-level shadow pages having gptes are tracked for write-protection via
// gfn_write_track.  As above, gfn_write_track is a 16 bit counter, so KVM must
// not create more than 2^16-1 upper-level shadow pages at a single gfn,
// otherwise gfn_write_track will overflow and explosions will ensue.
//
// A unique shadow page (SP) for a gfn is created if and only if an existing SP
// cannot be reused.  The ability to reuse a SP is tracked by its role, which
// incorporates various mode bits and properties of the SP.  Roughly speaking,
// the number of unique SPs that can theoretically be created is 2^n, where n
// is the number of bits that are used to compute the role.
//
// But, even though there are 21 bits in the mask below, not all combinations
// of modes and flags are possible:
//
// - invalid shadow pages are not accounted, mirror pages are not shadowed,
// so the bits are effectively 19.
//
// - quadrant will only be used if has_4_byte_gpte=1 (non-PAE paging);
// execonly and ad_disabled are only used for nested EPT which has
// has_4_byte_gpte=0.  Therefore, 2 bits are always unused.
//
// - the 4 bits of level are effectively limited to the values 2/3/4/5,
// as 4k SPs are not tracked (allowed to go unsync).  In addition non-PAE
// paging has exactly one upper level, making level completely redundant
// when has_4_byte_gpte=1.
//
// - on top of this, smap_andnot_wp is only set if cr0_wp=0,
// therefore these two bits only give rise to 3 possibilities.
//
// Therefore, the maximum number of possible upper-level shadow pages for a
// single gfn is a bit less than 2^14.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_mmu_page_role {
    pub word: u32,
    pub level:4: unsigned,
    pub has_4_byte_gpte:1: unsigned,
    pub quadrant:2: unsigned,
    pub direct:1: unsigned,
    pub access:4: unsigned,
    pub invalid:1: unsigned,
    pub efer_nx:1: unsigned,
    pub cr0_wp:1: unsigned,
    pub smap_andnot_wp:1: unsigned,
    pub ad_disabled:1: unsigned,
    pub guest_mode:1: unsigned,
    pub passthrough:1: unsigned,
    pub is_mirror:1: unsigned,
//
// cr4_smep is also set for EPT MBEC.  Because it affects
// which pages are considered non-present (bit 10 additionally
// must be zero if MBEC is on) it has to be in the base role.
// It also has to be in the base role for AMD GMET because
// kernel-executable pages need to have U=0 with GMET enabled.
//
    pub cr4_smep:1: unsigned,
//
// This is left at the top of the word so that
// kvm_memslots_for_spte_role can extract it with a
// simple shift.  While there is room, give it a whole
// byte so it is also faster to load it from memory.
//
    pub smm:8: unsigned,
}

//
// kvm_mmu_extended_role complements kvm_mmu_page_role, tracking properties
// relevant to the current MMU configuration.   When loading CR0, CR4, or EFER,
// including on nested transitions, if nothing in the full role changes then
// MMU re-configuration can be skipped. @valid bit is set on first usage so we
// don't treat all-zero structure as valid data.
//
// The properties that are tracked in the extended role but not the page role
// are for things that either (a) do not affect the validity of the shadow page
// or (b) are indirectly reflected in the shadow page's role.  For example,
// CR4.PKE only affects permission checks for software walks of the guest page
// tables (because KVM doesn't support Protection Keys with shadow paging), and
// CR0.PG, CR4.PAE, and CR4.PSE are indirectly reflected in role.level.
//
// Note, SMAP is not redundant with smap_andnot_wp in the page role.  If
// CR0.WP=1, KVM can reuse shadow pages for the guest regardless of SMAP,
// but the MMU's permission checks for software walks need to be SMAP
// aware regardless of CR0.WP.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_mmu_extended_role {
    pub word: u32,
    pub valid:1: c_uint,
    pub execonly:1: c_uint,
    pub cr4_pse:1: c_uint,
    pub cr4_pke:1: c_uint,
    pub cr4_smap:1: c_uint,
    pub cr4_la57:1: c_uint,
    pub efer_lma:1: c_uint,
//
// True if either CR4.SMEP or EFER.NXE are set.  For AMD NPT
// this is the "real" host CR4.SMEP whereas cr4_smep is
// actually GMET.
//
    pub has_pferr_fetch:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_cpu_role {
    pub as_u64: u64,
    pub base: kvm_mmu_page_role,
    pub ext: kvm_mmu_extended_role,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_rmap_head {
    pub val: atomic_long_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pio_request {
    pub count: c_ulong,
    pub in: c_int,
    pub port: c_int,
    pub size: c_int,
}

pub const PT64_ROOT_MAX_LEVEL: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_page_format {
    pub rsvd_bits_mask: [u64; 2][PT64_ROOT_MAX_LEVEL],
    pub bad_mt_xwr: u64,
//
// The pkru_mask indicates if protection key checks are needed.  It
// consists of 16 domains indexed by page fault error code bits [4:1],
// with PFEC.RSVD replaced by ACC_USER_MASK from the page tables.
// Each domain has 2 bits which are ANDed with AD and WD from PKRU.
//
    pub pkru_mask: u32,
//
// Bitmap; bit set = permission fault
// Array index: page fault error code [4:1]
// Bit index: pte permissions in ACC_* format
//
    pub permissions: [u16; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu_root_info {
    pub pgd: gpa_t,
    pub hpa: hpa_t,
}

pub const KVM_MMU_NUM_PREV_ROOTS: c_int = 3;

// Macro flag: #define KVM_HAVE_MMU_RWLOCK
//
// x86 supports 4 paging modes (5-level 64-bit, 4-level 64-bit, 3-level 32-bit,
// and 2-level 32-bit).  The kvm_pagewalk structure abstracts the details of the
// current mmu mode.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pagewalk {
    pub vcpu): *mut *mut unsigned long (get_guest_pgd)(struct kvm_vcpu,
    pub index): *mut *mut *mut u64 (get_pdptr)(struct kvm_vcpu vcpu, int,
    pub from_hardware): bool,
    pub exception): *mut x86_exception,
    pub cpu_role: kvm_cpu_role,
    pub fmt: kvm_page_format,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mmu {
    pub fault): *mut *mut *mut int (page_fault)(struct kvm_vcpu vcpu, struct kvm_page_fault,
    pub i): *mut *mut kvm_mmu_page sp, int,
    pub w: *mut kvm_pagewalk,
    pub root: kvm_mmu_root_info,
    pub mirror_root_hpa: hpa_t,
    pub root_role: kvm_mmu_page_role,
    pub prev_roots: [kvm_mmu_root_info; KVM_MMU_NUM_PREV_ROOTS],
    pub pae_root: *mut u64,
    pub pml4_root: *mut u64,
    pub pml5_root: *mut u64,
//
// check zero bits on shadow page table entries, these
// bits include not only hardware reserved bits but also
// the bits spte never used.
//
    pub fmt: kvm_page_format,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pmc_type {
    KVM_PMC_GP = 0,
    KVM_PMC_FIXED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmc {
    pub type: pmc_type,
    pub idx: u8,
    pub is_paused: bool,
    pub intr: bool,
//
// Base value of the PMC counter, relative to the *consumed* count in
// the associated perf_event.  This value includes counter updates from
// the perf_event and emulated_count since the last time the counter
// was reprogrammed, but it is *not* the current value as seen by the
// guest or userspace.
//
// The count is relative to the associated perf_event so that KVM
// doesn't need to reprogram the perf_event every time the guest writes
// to the counter.
//
    pub counter: u64,
//
// PMC events triggered by KVM emulation that haven't been fully
// processed, i.e. haven't undergone overflow detection.
//
    pub emulated_counter: u64,
    pub eventsel: u64,
    pub eventsel_hw: u64,
    pub perf_event: *mut perf_event,
    pub vcpu: *mut kvm_vcpu,
//
// only for creating or reusing perf_event,
// eventsel value for general purpose counters,
// ctrl value for fixed counters.
//
    pub current_config: u64,
}

// More counters may conflict with other existing Architectural MSRs

pub const KVM_MAX_NR_INTEL_GP_COUNTERS: c_int = 8;
pub const KVM_MAX_NR_AMD_GP_COUNTERS: c_int = 6;

pub const KVM_MAX_NR_INTEL_FIXED_COUNTERS: c_int = 3;
pub const KVM_MAX_NR_AMD_FIXED_COUNTERS: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu {
    pub version: u8,
    pub nr_arch_gp_counters: unsigned,
    pub nr_arch_fixed_counters: unsigned,
    pub available_event_types: unsigned,
    pub fixed_ctr_ctrl: u64,
    pub fixed_ctr_ctrl_hw: u64,
    pub fixed_ctr_ctrl_rsvd: u64,
    pub global_ctrl: u64,
    pub global_status: u64,
    pub counter_bitmask: [u64; 2],
    pub global_ctrl_rsvd: u64,
    pub global_status_rsvd: u64,
    pub reserved_bits: u64,
    pub raw_event_mask: u64,
    pub gp_counters: [kvm_pmc; KVM_MAX_NR_GP_COUNTERS],
    pub fixed_counters: [kvm_pmc; KVM_MAX_NR_FIXED_COUNTERS],
//
// Overlay the bitmap with a 64-bit atomic so that all bits can be
// set in a single access, e.g. to reprogram all counters when the PMU
// filter changes.
//
    pub X86_PMC_IDX_MAX): DECLARE_BITMAP(reprogram_pmi,,
    pub __reprogram_pmi: core::sync::atomic::AtomicI64,
}

//
// If a guest counter is cross-mapped to host counter with different
// index, its PEBS capability will be temporarily disabled.
//
// The user should make sure that this mask is updated
// after disabling interrupts and before perf_guest_get_msrs();
//
// The gate to release perf_events not marked in
// pmc_in_use only once in a vcpu time slice.
//
// The total number of programmed perf_events and it helps to avoid
// redundant check before cleanup if guest don't use vPMU at all.
//
// Guest debug registers (DR0-3, DR6 and DR7) are saved/restored by
// hardware on exit from or enter to guest. KVM needn't switch them.
// DR0-3, DR6 and DR7 are set to their architectural INIT value on VM
// exit, host values need to be restored.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mtrr {
    pub 2]: *mut *mut u64 var[KVM_NR_VAR_MTRR,
    pub fixed_64k: u64,
    pub fixed_16k: [u64; 2],
    pub fixed_4k: [u64; 8],
    pub deftype: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hypervisor_cpuid {
    pub base: u32,
    pub limit: u32,
}

// Xen HVM per vcpu emulation context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_xen {
    pub hypercall_rip: u64,
    pub current_runstate: u32,
    pub upcall_vector: u8,
    pub vcpu_info_cache: gfn_to_pfn_cache,
    pub vcpu_time_info_cache: gfn_to_pfn_cache,
    pub runstate_cache: gfn_to_pfn_cache,
    pub runstate2_cache: gfn_to_pfn_cache,
    pub last_steal: u64,
    pub runstate_entry_time: u64,
    pub runstate_times: [u64; 4],
    pub evtchn_pending_sel: c_ulong,
    pub /: *mut *mut u32 vcpu_id; / The Xen / ACPI vCPU ID,
    pub timer_virq: u32,
    pub /: *mut *mut u64 timer_expires; / In guest epoch,
    pub timer_pending: core::sync::atomic::AtomicI32,
    pub timer: hrtimer,
    pub poll_evtchn: c_int,
    pub poll_timer: timer_list,
    pub cpuid: kvm_hypervisor_cpuid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_queued_exception {
    pub pending: bool,
    pub injected: bool,
    pub has_error_code: bool,
    pub vector: u8,
    pub error_code: u32,
    pub payload: c_ulong,
    pub has_payload: bool,
}

//
// Hardware-defined CPUID leafs that are either scattered by the kernel or are
// unknown to the kernel, but need to be directly used by KVM.  Note, these
// word values conflict with the kernel's "bug" caps, but KVM doesn't use those.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_only_cpuid_leafs {
    CPUID_12_EAX	 = NCAPINTS,
    CPUID_7_1_EDX,
    CPUID_8000_0007_EDX,
    CPUID_8000_0022_EAX,
    CPUID_7_2_EDX,
    CPUID_24_0_EBX,
    CPUID_8000_0021_ECX,
    CPUID_7_1_ECX,
    CPUID_1E_1_EAX,
    CPUID_24_1_ECX,
    NR_KVM_CPU_CAPS,

    NKVMCAPINTS = NR_KVM_CPU_CAPS - NCAPINTS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_arch {
//
// rip and regs accesses must go through
// kvm_{register,rip}_{read,write} functions.
//
    pub regs: [c_ulong; NR_VCPU_GENERAL_PURPOSE_REGS],
    pub rip: c_ulong,
    pub NR_VCPU_TOTAL_REGS): DECLARE_BITMAP(regs_avail,,
    pub NR_VCPU_TOTAL_REGS): DECLARE_BITMAP(regs_dirty,,
    pub cr0: c_ulong,
    pub cr0_guest_owned_bits: c_ulong,
    pub cr2: c_ulong,
    pub cr3: c_ulong,
    pub cr4: c_ulong,
    pub cr4_guest_owned_bits: c_ulong,
    pub cr4_guest_rsvd_bits: c_ulong,
    pub cr8: c_ulong,
    pub host_pkru: u32,
    pub pkru: u32,
    pub hflags: u32,
    pub efer: u64,
    pub host_debugctl: u64,
    pub apic_base: u64,
    pub /: *mut *mut *mut kvm_lapic apic; / kernel irqchip context,
    pub load_eoi_exitmap_pending: bool,
    pub 256): DECLARE_BITMAP(ioapic_handled_vectors,,
    pub apic_attention: c_ulong,
    pub apic_arb_prio: i32,
    pub mp_state: c_int,
    pub ia32_misc_enable_msr: u64,
    pub smbase: u64,
    pub smi_count: u64,
    pub at_instruction_boundary: bool,
    pub tpr_access_reporting: bool,
    pub xfd_no_write_intercept: bool,
    pub microcode_version: u64,
    pub arch_capabilities: u64,
    pub perf_capabilities: u64,
//
// Paging state of the vcpu
//
// If the vcpu runs in guest mode with two level paging this still saves
// the paging mode of the l1 guest. This context is always used to
// handle faults.
//
    pub mmu: *mut kvm_mmu,
// Non-nested MMU for L1
    pub root_mmu: kvm_mmu,
// L1 TDP when running nested
    pub guest_mmu: kvm_mmu,
    pub ngpa_walk: kvm_pagewalk,
//
// Pagewalk context used for gva_to_gpa translations.
//
    pub gva_walk: kvm_pagewalk,
    pub /: *mut *mut u64 pdptrs[4]; / pae,
    pub mmu_pte_list_desc_cache: kvm_mmu_memory_cache,
    pub mmu_shadow_page_cache: kvm_mmu_memory_cache,
    pub mmu_shadowed_info_cache: kvm_mmu_memory_cache,
    pub mmu_page_header_cache: kvm_mmu_memory_cache,
//
// This cache is to allocate external page table. E.g. private EPT used
// by the TDX module.
//
    pub mmu_external_spt_cache: kvm_mmu_memory_cache,
//
// QEMU userspace and the guest each have their own FPU state.
// In vcpu_run, we switch between the user and guest FPU contexts.
// While running a VCPU, the VCPU thread will have the guest FPU
// context.
//
// Note that while the PKRU state lives inside the fpu registers,
// it is switched out separately at VMENTER and VMEXIT time. The
// "guest_fpstate" state here contains the guest FPU context, with the
// host PRKU bits.
//
    pub guest_fpu: fpu_guest,
    pub xcr0: u64,
    pub guest_supported_xcr0: u64,
    pub ia32_xss: u64,
    pub guest_supported_xss: u64,
    pub pio: kvm_pio_request,
    pub pio_data: *mut c_void,
    pub sev_pio_data: *mut c_void,
    pub sev_pio_count: unsigned,
    pub event_exit_inst_len: u8,
    pub exception_from_userspace: bool,
// Exceptions to be injected to the guest.
    pub exception: kvm_queued_exception,
// Exception VM-Exits to be synthesized to L1.
    pub exception_vmexit: kvm_queued_exception,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_queued_interrupt {
    pub injected: bool,
    pub soft: bool,
    pub nr: u8,
    pub interrupt: },
    pub /: *mut *mut int halt_request; / real mode on Intel only,
    pub cpuid_nent: c_int,
    pub cpuid_entries: *mut kvm_cpuid_entry2,
    pub cpuid_dynamic_bits_dirty: bool,
    pub is_amd_compatible: bool,
//
// cpu_caps holds the effective guest capabilities, i.e. the features
// the vCPU is allowed to use.  Typically, but not always, features can
// be used by the guest if and only if both KVM and userspace want to
// expose the feature to the guest.
//
// A common exception is for virtualization holes, i.e. when KVM can't
// prevent the guest from using a feature, in which case the vCPU "has"
// the feature regardless of what KVM or userspace desires.
//
// Note, features that don't require KVM involvement in any way are
// NOT enforced/sanitized by KVM, i.e. are taken verbatim from the
// guest CPUID provided by userspace.
//
    pub cpu_caps: [u32; NR_KVM_CPU_CAPS],
    pub reserved_gpa_bits: u64,
    pub maxphyaddr: c_int,
// emulate context
    pub emulate_ctxt: *mut x86_emulate_ctxt,
    pub emulate_regs_need_sync_to_vcpu: bool,
    pub emulate_regs_need_sync_from_vcpu: bool,
    pub vcpu): *mut *mut int (complete_userspace_io)(struct kvm_vcpu,
    pub cui_linear_rip: c_ulong,
    pub cui_rdmsr_imm_reg: c_int,
    pub time: gpa_t,
    pub pvclock_tsc_shift: i8,
    pub pvclock_tsc_mul: u32,
    pub hw_tsc_khz: c_uint,
    pub pv_time: gfn_to_pfn_cache,
// set guest stopped flag in pvclock flags field
    pub pvclock_set_guest_stopped_request: bool,
    pub preempted: u8,
    pub msr_val: u64,
    pub last_steal: u64,
    pub cache: gfn_to_hva_cache,
    pub st: },
    pub l1_tsc_offset: u64,
    pub /: *mut *mut u64 tsc_offset; / current tsc offset,
    pub last_guest_tsc: u64,
    pub last_host_tsc: u64,
    pub tsc_offset_adjustment: u64,
    pub this_tsc_nsec: u64,
    pub this_tsc_write: u64,
    pub this_tsc_generation: u64,
    pub tsc_catchup: bool,
    pub tsc_always_catchup: bool,
    pub virtual_tsc_shift: i8,
    pub virtual_tsc_mult: u32,
    pub virtual_tsc_khz: u32,
    pub ia32_tsc_adjust_msr: i64,
    pub msr_ia32_power_ctl: u64,
    pub l1_tsc_scaling_ratio: u64,
    pub /: *mut *mut u64 tsc_scaling_ratio; / current scaling ratio,
    pub /: *mut *mut atomic_t nmi_queued; / unprocessed asynchronous NMIs,
// Number of NMIs pending injection, not including hardware vNMIs.
    pub nmi_pending: c_uint,
    pub /: *mut *mut bool nmi_injected; / Trying to inject an NMI this entry,
    pub /: *mut *mut bool smi_pending; / SMI queued after currently running handler,
    pub handling_intr_from_guest: u8,
    pub mtrr_state: kvm_mtrr,
    pub pat: u64,
    pub switch_db_regs: unsigned,
    pub db: [c_ulong; KVM_NR_DB_REGS],
    pub dr6: c_ulong,
    pub dr7: c_ulong,
    pub eff_db: [c_ulong; KVM_NR_DB_REGS],
    pub guest_debug_dr7: c_ulong,
    pub msr_platform_info: u64,
    pub msr_misc_features_enables: u64,
    pub mcg_cap: u64,
    pub mcg_status: u64,
    pub mcg_ctl: u64,
    pub mcg_ext_ctl: u64,
    pub mce_banks: *mut u64,
    pub mci_ctl2_banks: *mut u64,
// Cache MMIO info
    pub mmio_gva: u64,
    pub mmio_access: unsigned,
    pub mmio_gfn: gfn_t,
    pub mmio_gen: u64,
    pub pmu: kvm_pmu,
// used for guest single stepping over the given code position
    pub singlestep_rip: c_ulong,

    pub hyperv_enabled: bool,
    pub hyperv: *mut kvm_vcpu_hv,

    pub xen: kvm_vcpu_xen,

    pub wbinvd_dirty_mask: cpumask_var_t,
    pub last_retry_eip: c_ulong,
    pub last_retry_addr: c_ulong,
    pub halted: bool,
    pub gfns: [gfn_t; ASYNC_PF_PER_VCPU],
    pub data: gfn_to_hva_cache,
    pub /: *mut *mut u64 msr_en_val; / MSR_KVM_ASYNC_PF_EN,
    pub /: *mut *mut u64 msr_int_val; / MSR_KVM_ASYNC_PF_INT,
    pub vec: u16,
    pub id: u32,
    pub host_apf_flags: u32,
    pub pageready_pending: bool,
    pub apf: },
// OSVW MSRs (AMD only)
    pub length: u64,
    pub status: u64,
    pub osvw: },
    pub msr_val: u64,
    pub data: gfn_to_hva_cache,
    pub pv_eoi: },
    pub msr_kvm_poll_control: u64,
// pv related host specific info
    pub pv_unhalted: bool,
    pub pv: },
    pub pending_ioapic_eoi: c_int,
    pub pending_external_vector: c_int,
    pub highest_stale_pending_ioapic_eoi: c_int,
// be preempted when it's in kernel-mode(cpl=0)
    pub preempted_in_kernel: bool,
// Host CPU on which VM-entry was most recently attempted
    pub last_vmentry_cpu: c_int,
// AMD MSRC001_0015 Hardware Configuration
    pub msr_hwcr: u64,
// pv related cpuid info
//
// value of the eax register in the KVM_CPUID_FEATURES CPUID
// leaf.
//
    pub features: u32,
//
// indicates whether pv emulation should be disabled if features
// are not present in the guest's cpuid
//
    pub enforce: bool,
    pub pv_cpuid: },
// Protected Guests
    pub guest_state_protected: bool,
    pub guest_tsc_protected: bool,
//
// Set when PDPTS were loaded directly by the userspace without
// reading the guest memory
//
    pub pdptrs_from_userspace: bool,
//
// Set if an emulated nested VM-Enter to L2 is pending completion.  KVM
// must not synthesize a VM-Exit to L1 before entering L2, as VM-Exits
// can only occur at instruction boundaries.  The only exception is
// VMX's "notify" exits, which exist in large part to break the CPU out
// of infinite ucode loops, but can corrupt vCPU state in the process!
//
// For all intents and purposes, this is a boolean, but it's tracked as
// a u8 so that KVM can detect when userspace may have stuffed vCPU
// state and generated an architecturally-impossible VM-Exit.
//
pub const KVM_NESTED_RUN_PENDING: c_int = 1;
pub const KVM_NESTED_RUN_PENDING_UNTRUSTED: c_int = 2;
    pub nested_run_pending: u8,

    pub hv_root_tdp: hpa_t,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_lpage_info {
    pub disallow_lpage: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_memory_slot {
    pub rmap: [*mut kvm_rmap_head; KVM_NR_PAGE_SIZES],
    pub 1]: *mut *mut kvm_lpage_info lpage_info[KVM_NR_PAGE_SIZES -,
    pub gfn_write_track: *mut c_ushort,
}

// Hyper-V synthetic debugger (SynDbg)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hv_syndbg {
    pub control: u64,
    pub status: u64,
    pub send_page: u64,
    pub recv_page: u64,
    pub pending_page: u64,
    pub control: },
    pub options: u64,
}

// Current state of Hyper-V TSC page clocksource
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hv_tsc_page_status {
// TSC page was not set up or disabled
    HV_TSC_PAGE_UNSET = 0,
// TSC page MSR was written by the guest, update pending
    HV_TSC_PAGE_GUEST_CHANGED,
// TSC page update was triggered from the host side
    HV_TSC_PAGE_HOST_CHANGED,
// TSC page was properly set up and is currently active
    HV_TSC_PAGE_SET,
// TSC page was set up with an inaccessible GPA
    HV_TSC_PAGE_BROKEN,
}

// Hyper-V emulation context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hv {
    pub hv_lock: mutex,
    pub hv_guest_os_id: u64,
    pub hv_hypercall: u64,
    pub hv_tsc_page: u64,
    pub hv_tsc_page_status: hv_tsc_page_status,
// Hyper-v based guest crash (NT kernel bugcheck) parameters
    pub hv_crash_param: [u64; HV_X64_MSR_CRASH_PARAMS],
    pub hv_crash_ctl: u64,
    pub tsc_ref: ms_hyperv_tsc_page,
    pub conn_to_evt: idr,
    pub hv_reenlightenment_control: u64,
    pub hv_tsc_emulation_control: u64,
    pub hv_tsc_emulation_status: u64,
    pub hv_invtsc_control: u64,
// How many vCPUs have VP index != vCPU index
    pub num_mismatched_vp_indexes: core::sync::atomic::AtomicI32,
//
// How many SynICs use 'AutoEOI' feature
// (protected by arch.apicv_update_lock)
//
    pub synic_auto_eoi_used: c_uint,
    pub hv_syndbg: kvm_hv_syndbg,
    pub xsaves_xsavec_checked: bool,
}

// Xen emulation context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xen {
    pub xen_lock: mutex,
    pub xen_version: u32,
    pub long_mode: bool,
    pub runstate_update_flag: bool,
    pub upcall_vector: u8,
    pub shinfo_cache: gfn_to_pfn_cache,
    pub evtchn_ports: idr,
    pub poll_mask: [c_ulong; BITS_TO_LONGS(KVM_MAX_VCPUS)],
    pub hvm_config: kvm_xen_hvm_config,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_irqchip_mode {
    KVM_IRQCHIP_NONE,

    KVM_IRQCHIP_KERNEL,       /* created with KVM_CREATE_IRQCHIP */

    KVM_IRQCHIP_SPLIT,        /* created with KVM_CAP_SPLIT_IRQCHIP */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_suppress_eoi_broadcast_mode {
    KVM_SUPPRESS_EOI_BROADCAST_QUIRKED, /* Legacy behavior */
    KVM_SUPPRESS_EOI_BROADCAST_ENABLED, /* Enable Suppress EOI broadcast */
    KVM_SUPPRESS_EOI_BROADCAST_DISABLED /* Disable Suppress EOI broadcast */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_possible_nx_huge_pages {
//
// A list of kvm_mmu_page structs that, if zapped, could possibly be
// replaced by an NX huge page.  A shadow page is on this list if its
// existence disallows an NX huge page (nx_huge_page_disallowed is set)
// and there are no other conditions that prevent a huge page, e.g.
// the backing host page is huge, dirtly logging is not enabled for its
// memslot, etc...  Note, zapping shadow pages on this list doesn't
// guarantee an NX huge page will be created in its stead, e.g. if the
// guest attempts to execute from the region then KVM obviously can't
// create an NX huge page (without hanging the guest).
//
    pub pages: list_head,
    pub nr_pages: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_mmu_type {
    KVM_SHADOW_MMU,

    KVM_TDP_MMU,

    KVM_NR_MMU_TYPES,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch {
    pub n_requested_mmu_pages: c_ulong,
    pub n_max_mmu_pages: c_ulong,
    pub indirect_shadow_pages: c_uint,
    pub mmu_valid_gen: u8,
    pub vm_type: u8,
    pub has_private_mem: bool,
    pub has_protected_state: bool,
    pub has_protected_eoi: bool,
    pub has_protected_pmu: bool,
    pub pre_fault_allowed: bool,
    pub mmu_page_hash: *mut hlist_head,
    pub active_mmu_pages: list_head,
    pub possible_nx_huge_pages: [kvm_possible_nx_huge_pages; KVM_NR_MMU_TYPES],
    pub track_notifier_head: kvm_page_track_notifier_head,

//
// Protects marking pages unsync during page faults, as TDP MMU page
// faults only take mmu_lock for read.  For simplicity, the unsync
// pages lock is always taken when marking pages unsync regardless of
// whether mmu_lock is held for read or write.
//
    pub mmu_unsync_pages_lock: spinlock_t,
    pub shadow_mmio_value: u64,
    pub noncoherent_dma_count: core::sync::atomic::AtomicI32,
    pub nr_possible_bypass_irqs: c_ulong,

    pub vpic: *mut kvm_pic,
    pub vioapic: *mut kvm_ioapic,
    pub vpit: *mut kvm_pit,

    pub vapics_in_nmi_mode: core::sync::atomic::AtomicI32,
    pub apic_map_lock: mutex,
    pub apic_map: *mut kvm_apic_map __rcu,
    pub apic_map_dirty: core::sync::atomic::AtomicI32,
    pub apic_access_memslot_enabled: bool,
    pub apic_access_memslot_inhibited: bool,
//
// Force apicv_update_lock and apicv_nr_irq_window_req to reside in a
// dedicated cacheline.  They are write-mostly, whereas most everything
// else in kvm_arch is read-mostly.  Note that apicv_inhibit_reasons is
// read-mostly: toggling VM-wide inhibits is rare; _checking_ for
// inhibits is common.
//
// Protects apicv_inhibit_reasons and apicv_nr_irq_window_req (with an
// asterisk, see kvm_inc_or_dec_irq_window_inhibit() for details).
//
    pub apicv_update_lock: rw_semaphore,
    pub apicv_nr_irq_window_req: core::sync::atomic::AtomicI32,
    pub apicv_inhibit_reasons: c_ulong,
    pub wall_clock: gpa_t,
    pub disabled_exits: u64,
    pub kvmclock_offset: i64,
//
// This also protects nr_vcpus_matched_tsc which is read from a
// preemption-disabled region, so it must be a raw spinlock.
//
    pub tsc_write_lock: raw_spinlock_t,
    pub last_tsc_nsec: u64,
    pub last_tsc_write: u64,
    pub last_tsc_khz: u32,
    pub last_tsc_offset: u64,
    pub cur_tsc_nsec: u64,
    pub cur_tsc_write: u64,
    pub cur_tsc_offset: u64,
    pub cur_tsc_generation: u64,
    pub nr_vcpus_matched_tsc: c_int,
    pub default_tsc_khz: u32,
    pub user_set_tsc: bool,
    pub apic_bus_cycle_ns: u64,
    pub pvclock_sc: seqcount_raw_spinlock_t,
    pub use_master_clock: bool,
    pub master_kernel_ns: u64,
    pub master_cycle_now: u64,
    pub kvmclock_update_rs: ratelimit_state,

    pub hyperv: kvm_hv,

    pub xen: kvm_xen,

    pub backwards_tsc_observed: bool,
    pub boot_vcpu_runs_old_kvmclock: bool,
    pub bsp_vcpu_id: u32,
    pub disabled_quirks: u64,
    pub irqchip_mode: kvm_irqchip_mode,
    pub nr_reserved_ioapic_pins: u8,
    pub disabled_lapic_found: bool,
    pub x2apic_format: bool,
    pub x2apic_broadcast_quirk_disabled: bool,
    pub suppress_eoi_broadcast_mode: kvm_suppress_eoi_broadcast_mode,
    pub has_mapped_host_mmio: bool,
    pub guest_can_read_msr_platform_info: bool,
    pub exception_payload_enabled: bool,
    pub triple_fault_event: bool,
    pub bus_lock_detection_enabled: bool,
    pub enable_pmu: bool,
    pub created_mediated_pmu: bool,
    pub notify_window: u32,
    pub notify_vmexit_flags: u32,
//
// If exit_on_emulation_error is set, and the in-kernel instruction
// emulator fails to emulate an instruction, allow userspace
// the opportunity to look at it.
//
    pub exit_on_emulation_error: bool,
// Deflect RDMSR and WRMSR to user space when they trigger a #GP
    pub user_space_msr_mask: u32,
    pub msr_filter: *mut kvm_x86_msr_filter __rcu,
    pub hypercall_exit_enabled: u32,
// Guest can access the SGX PROVISIONKEY.
    pub sgx_provisioning_allowed: bool,
    pub pmu_event_filter: *mut kvm_x86_pmu_event_filter __rcu,
    pub nx_huge_page_recovery_thread: *mut vhost_task,
    pub nx_huge_page_last: u64,
    pub nx_once: once,

//
// The number of TDP MMU pages across all roots.  Used only to sanity
// check that KVM isn't leaking TDP MMU pages.
//
    pub tdp_mmu_pages: core::sync::atomic::AtomicI64,

//
// List of struct kvm_mmu_pages being used as roots.
// All struct kvm_mmu_pages in the list should have
// tdp_mmu_page set.
//
// For reads, this list is protected by:
// RCU alone or
// the MMU lock in read mode + RCU or
// the MMU lock in write mode
//
// For writes, this list is protected by tdp_mmu_pages_lock; see
// below for the details.
//
// Roots will remain in the list until their tdp_mmu_root_count
// drops to zero, at which point the thread that decremented the
// count to zero should removed the root from the list and clean
// it up, freeing the root after an RCU grace period.
//
    pub tdp_mmu_roots: list_head,
//
// Protects accesses to the following fields when the MMU lock
// is held in read mode:
// - tdp_mmu_roots (above)
// - the link field of kvm_mmu_page structs used by the TDP MMU
// - possible_nx_huge_pages[KVM_TDP_MMU];
// - the possible_nx_huge_page_link field of kvm_mmu_page structs used
// by the TDP MMU
// Because the lock is only taken within the MMU lock, strictly
// speaking it is redundant to acquire this lock when the thread
// holds the MMU lock in write mode.  However it often simplifies
// the code to do so.
//
    pub tdp_mmu_pages_lock: spinlock_t,

//
// If set, at least one shadow root has been allocated. This flag
// is used as one input when determining whether certain memslot
// related allocations are necessary.
//
    pub shadow_root_allocated: bool,

//
// If set, the VM has (or had) an external write tracking user, and
// thus all write tracking metadata has been allocated, even if KVM
// itself isn't using write tracking.
//
    pub external_write_tracking_enabled: bool,

    pub hv_root_tdp: hpa_t,
    pub hv_root_tdp_lock: spinlock_t,
    pub hv_pa_pg: *mut hv_partition_assist_pg,

//
// VM-scope maximum vCPU ID. Used to determine the size of structures
// that increase along with the maximum vCPU ID, in which case, using
// the global KVM_MAX_VCPU_IDS may lead to significant memory waste.
//
    pub max_vcpu_ids: u32,
    pub disable_nx_huge_pages: bool,
//
// Memory caches used to allocate shadow pages when performing eager
// page splitting. No need for a shadowed_info_cache since eager page
// splitting only allocates direct shadow pages.
//
// Protected by kvm->slots_lock.
//
    pub split_shadow_page_cache: kvm_mmu_memory_cache,
    pub split_page_header_cache: kvm_mmu_memory_cache,
//
// Memory cache used to allocate pte_list_desc structs while splitting
// huge pages. In the worst case, to split one huge page, 512
// pte_list_desc structs are needed to add each lower level leaf sptep
// to the rmap plus 1 to extend the parent_ptes rmap of the lower level
// page table.
//
// Protected by kvm->slots_lock.
//

    pub split_desc_cache: kvm_mmu_memory_cache,
    pub gfn_direct_bits: gfn_t,
//
// Size of the CPU's dirty log buffer, i.e. VMX's PML buffer. A Zero
// value indicates CPU dirty logging is unsupported or disabled in
// current VM.
//
    pub cpu_dirty_log_size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vm_stat {
    pub generic: kvm_vm_stat_generic,
    pub mmu_shadow_zapped: u64,
    pub mmu_pte_write: u64,
    pub mmu_pde_zapped: u64,
    pub mmu_flooded: u64,
    pub mmu_recycled: u64,
    pub mmu_cache_miss: u64,
    pub mmu_unsync: u64,
    pub mmu_shadow_pages: u64,
    pub pages_4k: core::sync::atomic::AtomicI64,
    pub pages_2m: core::sync::atomic::AtomicI64,
    pub pages_1g: core::sync::atomic::AtomicI64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub pf_taken: u64,
    pub pf_fixed: u64,
    pub pf_emulate: u64,
    pub pf_spurious: u64,
    pub pf_fast: u64,
    pub pf_mmio_spte_created: u64,
    pub pf_guest: u64,
    pub tlb_flush: u64,
    pub invlpg: u64,
    pub exits: u64,
    pub io_exits: u64,
    pub mmio_exits: u64,
    pub signal_exits: u64,
    pub irq_window_exits: u64,
    pub nmi_window_exits: u64,
    pub l1d_flush: u64,
    pub halt_exits: u64,
    pub request_irq_exits: u64,
    pub irq_exits: u64,
    pub host_state_reload: u64,
    pub fpu_reload: u64,
    pub insn_emulation: u64,
    pub insn_emulation_fail: u64,
    pub hypercalls: u64,
    pub irq_injections: u64,
    pub nmi_injections: u64,
    pub req_event: u64,
    pub nested_run: u64,
    pub directed_yield_attempted: u64,
    pub directed_yield_successful: u64,
    pub preemption_reported: u64,
    pub preemption_other: u64,
    pub guest_mode: u64,
    pub notify_window_exits: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msr_data {
    pub host_initiated: bool,
    pub index: u32,
    pub data: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_lapic_irq {
    pub vector: u32,
    pub delivery_mode: u16,
    pub dest_mode: u16,
    pub level: bool,
    pub trig_mode: u16,
    pub shorthand: u32,
    pub dest_id: u32,
    pub msi_redir_hint: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_x86_run_flags {
    KVM_RUN_FORCE_IMMEDIATE_EXIT	= BIT(0),
    KVM_RUN_LOAD_GUEST_DR6		= BIT(1),
    KVM_RUN_LOAD_DEBUGCTL		= BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_x86_ops {
    pub name: *const c_char,
    pub (*check_processor_compatibility)(void): *mut c_int,
    pub (*enable_virtualization_cpu)(void): *mut c_int,
    pub (*disable_virtualization_cpu)(void): *mut c_void,
    pub emergency_disable_virtualization_cpu: *mut cpu_emergency_virt_cb,
    pub (*hardware_unsetup)(void): *mut c_void,
    pub index): *mut *mut *mut bool (has_emulated_msr)(struct kvm kvm, u32,
    pub vcpu): *mut *mut void (vcpu_after_set_cpuid)(struct kvm_vcpu,
    pub vm_size: c_uint,
    pub kvm): *mut *mut int (vm_init)(struct kvm,
    pub kvm): *mut *mut void (vm_destroy)(struct kvm,
    pub kvm): *mut *mut void (vm_pre_destroy)(struct kvm,
// Create, but do not attach this VCPU
    pub kvm): *mut *mut int (vcpu_precreate)(struct kvm,
    pub vcpu): *mut *mut int (vcpu_create)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (vcpu_free)(struct kvm_vcpu,
    pub init_event): *mut *mut *mut void (vcpu_reset)(struct kvm_vcpu vcpu, bool,
    pub vcpu): *mut *mut void (prepare_switch_to_guest)(struct kvm_vcpu,
    pub cpu): *mut *mut *mut void (vcpu_load)(struct kvm_vcpu vcpu, int,
    pub vcpu): *mut *mut void (vcpu_put)(struct kvm_vcpu,
//
// Mask of DEBUGCTL bits that are owned by the host, i.e. that need to
// match the host's value even while the guest is active.
//
    pub HOST_OWNED_DEBUGCTL: u64,
    pub vcpu): *mut *mut void (update_exception_bitmap)(struct kvm_vcpu,
    pub msr): *mut *mut *mut int (get_msr)(struct kvm_vcpu vcpu, struct msr_data,
    pub msr): *mut *mut *mut int (set_msr)(struct kvm_vcpu vcpu, struct msr_data,
    pub seg): *mut *mut *mut u64 (get_segment_base)(struct kvm_vcpu vcpu, int,
    pub seg): *mut *mut kvm_segment var, int,
    pub vcpu): *mut *mut int (get_cpl)(struct kvm_vcpu,
    pub vcpu): *mut *mut int (get_cpl_no_cache)(struct kvm_vcpu,
    pub seg): *mut *mut kvm_segment var, int,
    pub l): *mut *mut *mut *mut void (get_cs_db_l_bits)(struct kvm_vcpu vcpu, int db, int,
    pub cr0): *mut *mut *mut bool (is_valid_cr0)(struct kvm_vcpu vcpu, unsigned long,
    pub cr0): *mut *mut *mut void (set_cr0)(struct kvm_vcpu vcpu, unsigned long,
    pub cr3): *mut *mut *mut void (post_set_cr3)(struct kvm_vcpu vcpu, unsigned long,
    pub cr4): *mut *mut *mut bool (is_valid_cr4)(struct kvm_vcpu vcpu, unsigned long,
    pub cr4): *mut *mut *mut void (set_cr4)(struct kvm_vcpu vcpu, unsigned long,
    pub efer): *mut *mut *mut int (set_efer)(struct kvm_vcpu vcpu, u64,
    pub dt): *mut *mut *mut void (get_idt)(struct kvm_vcpu vcpu, struct desc_ptr,
    pub dt): *mut *mut *mut void (set_idt)(struct kvm_vcpu vcpu, struct desc_ptr,
    pub dt): *mut *mut *mut void (get_gdt)(struct kvm_vcpu vcpu, struct desc_ptr,
    pub dt): *mut *mut *mut void (set_gdt)(struct kvm_vcpu vcpu, struct desc_ptr,
    pub vcpu): *mut *mut void (sync_dirty_debug_regs)(struct kvm_vcpu,
    pub value): *mut *mut *mut void (set_dr7)(struct kvm_vcpu vcpu, unsigned long,
    pub reg): *mut *mut *mut void (cache_reg)(struct kvm_vcpu vcpu, enum kvm_reg,
    pub vcpu): *mut *mut unsigned long (get_rflags)(struct kvm_vcpu,
    pub rflags): *mut *mut *mut void (set_rflags)(struct kvm_vcpu vcpu, unsigned long,
    pub vcpu): *mut *mut bool (get_if_flag)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (flush_tlb_all)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (flush_tlb_current)(struct kvm_vcpu,

    pub kvm): *mut *mut int (flush_remote_tlbs)(struct kvm,
    pub nr_pages): gfn_t,

//
// Flush any TLB entries associated with the given GVA.
// Does not need to flush GPA->HPA mappings.
// Can potentially get non-canonical addresses through INVLPGs, which
// the implementation may choose to ignore if appropriate.
//
    pub full): *mut *mut *mut void (flush_tlb_gva)(struct kvm_vcpu vcpu, gva_t addr, bool,
//
// Flush any TLB entries created by the guest.  Like tlb_flush_gva(),
// does not need to flush GPA->HPA mappings.
//
    pub vcpu): *mut *mut void (flush_tlb_guest)(struct kvm_vcpu,
    pub vcpu): *mut *mut bool (vcpu_needs_initialization)(struct kvm_vcpu,
    pub run_flags): u64,
    pub exit_fastpath): exit_fastpath_completion,
    pub vcpu): *mut *mut int (skip_emulated_instruction)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (update_emulated_instruction)(struct kvm_vcpu,
    pub vcpu): *mut *mut bool (unhandleable_emulation_required)(struct kvm_vcpu,
    pub mask): *mut *mut *mut void (set_interrupt_shadow)(struct kvm_vcpu vcpu, int,
    pub vcpu): *mut *mut u32 (get_interrupt_shadow)(struct kvm_vcpu,
    pub hypercall_addr): *mut c_uchar,
    pub reinjected): *mut *mut *mut void (inject_irq)(struct kvm_vcpu vcpu, bool,
    pub vcpu): *mut *mut void (inject_nmi)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (inject_exception)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (cancel_injection)(struct kvm_vcpu,
    pub for_injection): *mut *mut *mut int (interrupt_allowed)(struct kvm_vcpu vcpu, bool,
    pub for_injection): *mut *mut *mut int (nmi_allowed)(struct kvm_vcpu vcpu, bool,
    pub vcpu): *mut *mut bool (get_nmi_mask)(struct kvm_vcpu,
    pub masked): *mut *mut *mut void (set_nmi_mask)(struct kvm_vcpu vcpu, bool,
// Whether or not a virtual NMI is pending in hardware.
    pub vcpu): *mut *mut bool (is_vnmi_pending)(struct kvm_vcpu,
//
// Attempt to pend a virtual NMI in hardware.  Returns %true on success
// to allow using static_call_ret0 as the fallback.
//
    pub vcpu): *mut *mut bool (set_vnmi_pending)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (enable_nmi_window)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (enable_irq_window)(struct kvm_vcpu,
    pub irr): *mut *mut *mut void (update_cr8_intercept)(struct kvm_vcpu vcpu, int tpr, int,
    pub x2apic_icr_is_split: bool,
    pub required_apicv_inhibits: c_ulong,
    pub allow_apicv_in_x2apic_without_x2apic_virtualization: bool,
    pub vcpu): *mut *mut void (refresh_apicv_exec_ctrl)(struct kvm_vcpu,
    pub isr): *mut *mut *mut void (hwapic_isr_update)(struct kvm_vcpu vcpu, int,
    pub eoi_exit_bitmap): *mut *mut *mut void (load_eoi_exitmap)(struct kvm_vcpu vcpu, u64,
    pub vcpu): *mut *mut void (set_virtual_apic_mode)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (set_apic_access_page_addr)(struct kvm_vcpu,
    pub vector): int trig_mode, int,
    pub vcpu): *mut *mut int (sync_pir_to_irr)(struct kvm_vcpu,
    pub addr): *mut *mut *mut int (set_tss_addr)(struct kvm kvm, unsigned int,
    pub ident_addr): *mut *mut *mut int (set_identity_map_addr)(struct kvm kvm, u64,
    pub is_mmio): *mut *mut *mut u8 (get_mt_mask)(struct kvm_vcpu vcpu, gfn_t gfn, bool,
    pub kvm): *mut *mut bool (tdp_has_smep)(struct kvm,
    pub root_level): c_int,
// Update the external page table from spte getting set.
    pub level): u64 new_spte, enum pg_level,
// Update external page tables for page table about to be freed.
    pub sp): *mut *mut *mut void (free_external_spt)(struct kvm kvm, struct kvm_mmu_page,
    pub (*has_wbinvd_exit)(void): *mut bool,
    pub vcpu): *mut *mut u64 (get_l2_tsc_offset)(struct kvm_vcpu,
    pub vcpu): *mut *mut u64 (get_l2_tsc_multiplier)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (write_tsc_offset)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (write_tsc_multiplier)(struct kvm_vcpu,
//
// Retrieve somewhat arbitrary exit/entry information.  Intended to
// be used only from within tracepoints or error paths.
//
    pub error_code): *mut *mut u32 intr_info, u32,
    pub error_code): *mut *mut u32 intr_info, u32,
    pub exception): *mut x86_exception,
    pub vcpu): *mut *mut void (handle_exit_irqoff)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (update_cpu_dirty_logging)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (vcpu_blocking)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (vcpu_unblocking)(struct kvm_vcpu,
    pub vector): *mut *mut kvm_vcpu vcpu, u32,
    pub kvm): *mut *mut void (pi_start_bypass)(struct kvm,
    pub vcpu): *mut *mut void (apicv_pre_state_restore)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (apicv_post_state_restore)(struct kvm_vcpu,
    pub vcpu): *mut *mut bool (dy_apicv_has_pending_interrupt)(struct kvm_vcpu,
    pub vcpu): *mut *mut bool (protected_apic_has_interrupt)(struct kvm_vcpu,
    pub expired): *mut bool,
    pub vcpu): *mut *mut void (cancel_hv_timer)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (setup_mce)(struct kvm_vcpu,

    pub for_injection): *mut *mut *mut int (smi_allowed)(struct kvm_vcpu vcpu, bool,
    pub smram): *mut *mut *mut int (enter_smm)(struct kvm_vcpu vcpu, union kvm_smram,
    pub smram): *const *const *const int (leave_smm)(struct kvm_vcpu vcpu, union kvm_smram,
    pub vcpu): *mut *mut void (enable_smi_window)(struct kvm_vcpu,

    pub val): *mut *mut int (dev_get_attr)(u32 group, u64 attr, u64,
    pub argp): *mut *mut *mut int (mem_enc_ioctl)(struct kvm kvm, void __user,
    pub argp): *mut *mut *mut int (vcpu_mem_enc_ioctl)(struct kvm_vcpu vcpu, void __user,
    pub argp): *mut *mut *mut int (vcpu_mem_enc_unlocked_ioctl)(struct kvm_vcpu vcpu, void __user,
    pub argp): *mut *mut *mut int (mem_enc_register_region)(struct kvm kvm, struct kvm_enc_region,
    pub argp): *mut *mut *mut int (mem_enc_unregister_region)(struct kvm kvm, struct kvm_enc_region,
    pub source_fd): *mut *mut *mut int (vm_copy_enc_context_from)(struct kvm kvm, unsigned int,
    pub source_fd): *mut *mut *mut int (vm_move_enc_context_from)(struct kvm kvm, unsigned int,
    pub kvm): *mut *mut void (guest_memory_reclaimed)(struct kvm,
    pub vcpu): *mut *mut void (reload_vmsa)(struct kvm_vcpu,
    pub data): *mut *mut int (get_feature_msr)(u32 msr, u64,
    pub insn_len): *mut *mut void insn, int,
    pub vcpu): *mut *mut bool (apic_init_signal_blocked)(struct kvm_vcpu,
    pub vcpu): *mut *mut int (enable_l2_tlb_flush)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (migrate_timers)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (recalc_intercepts)(struct kvm_vcpu,
    pub err): *mut *mut *mut int (complete_emulated_msr)(struct kvm_vcpu vcpu, int,
    pub vector): *mut *mut *mut void (vcpu_deliver_sipi_vector)(struct kvm_vcpu vcpu, u8,
//
// Returns vCPU specific APICv inhibit reasons
//
    pub vcpu): *mut *mut unsigned long (vcpu_get_apicv_inhibit_reasons)(struct kvm_vcpu,
    pub flags): *mut *mut *mut gva_t (get_untagged_addr)(struct kvm_vcpu vcpu, gva_t gva, unsigned int,
    pub vcpu): *mut *mut *mut void (alloc_apic_backing_page)(struct kvm_vcpu,

    pub nr_pages): kvm_pfn_t,

    pub nr_pages): *mut *mut void (gmem_make_shared)(kvm_pfn_t pfn, kvm_pfn_t,

    pub range): *mut *mut *mut void (gmem_invalidate_range)(struct kvm kvm, struct kvm_gfn_range,

    pub is_private): *mut *mut *mut int (gmem_max_mapping_level)(struct kvm kvm, kvm_pfn_t pfn, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_x86_nested_ops {
    pub enabled: bool,
    pub vcpu): *mut *mut void (leave_nested)(struct kvm_vcpu,
    pub error_code): u32,
    pub vcpu): *mut *mut int (check_events)(struct kvm_vcpu,
    pub for_injection): *mut *mut *mut bool (has_events)(struct kvm_vcpu vcpu, bool,
    pub vcpu): *mut *mut void (triple_fault)(struct kvm_vcpu,
    pub user_data_size): unsigned,
    pub kvm_state): *mut kvm_nested_state,
    pub vcpu): *mut *mut bool (get_nested_state_pages)(struct kvm_vcpu,
    pub l2_gpa): *mut *mut *mut int (write_log_dirty)(struct kvm_vcpu vcpu, gpa_t,
    pub pte_access): u64,
    pub vmcs_version): *mut u16,
    pub vcpu): *mut *mut uint16_t (get_evmcs_version)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (hv_inject_synthetic_vmexit_post_tlb_flush)(struct kvm_vcpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_x86_init_ops {
    pub (*hardware_setup)(void): *mut c_int,
    pub (*handle_intel_pt_intr)(void): *mut c_uint,
    pub runtime_ops: *mut kvm_x86_ops,
    pub pmu_ops: *mut kvm_pmu_ops,
    pub nested_ops: *mut kvm_x86_nested_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_async_pf {
    pub token: u32,
    pub gfn: gfn_t,
    pub cr3: c_ulong,
    pub direct_map: bool,
    pub error_code: u64,
}

extern "C" {
    pub fn kvzalloc(_arg: kvm_x86_ops.vm_size, _arg: GFP_KERNEL_ACCOUNT) -> return;
}
extern "C" {
    pub fn kvm_arch_free_vm(kvm: *mut kvm);
}

extern "C" {
    pub fn kvm_x86_call(_arg: flush_remote_tlbs_range)(kvm, _arg: gfn, _arg: nr_pages) -> return;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_intr_type {
// Values are arbitrary, but must be non-zero.
    KVM_HANDLING_IRQ = 1,
    KVM_HANDLING_NMI,
}

// Enable perf NMI and timer modes to work, and minimise false positives.

// SMM is currently unsupported for guests with private memory.

extern "C" {
    pub fn kvm_arch_async_page_present_queued(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_arch_can_dequeue_async_page_present(vcpu: *mut kvm_vcpu) -> bool;
}
