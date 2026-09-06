//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_host.h
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
// Copyright IBM Corp. 2007
//
// Authors: Hollis Blanchard <hollisb@us.ibm.com>
//

//
// Limit the nested partition table to 4096 entries (because that's what
// hardware supports). Both guest and host use this value.
//
pub const KVM_MAX_NESTED_GUESTS_SHIFT: c_int = 12;

// These values are internal and can be increased later
pub const KVM_NR_IRQCHIPS: c_int = 1;
pub const KVM_IRQCHIP_NUM_PINS: c_int = 256;
// PPC-specific vcpu->requests bit members

pub const HPTEG_HASH_BITS_PTE: c_int = 13;
pub const HPTEG_HASH_BITS_PTE_LONG: c_int = 12;
pub const HPTEG_HASH_BITS_VPTE: c_int = 13;
pub const HPTEG_HASH_BITS_VPTE_LONG: c_int = 5;
pub const HPTEG_HASH_BITS_VPTE_64K: c_int = 11;

// Physical Address Mask - allowed range of real mode RAM access
pub const KVM_PAM: c_uint = 0x0fffffffffffffffULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vm_stat {
    pub generic: kvm_vm_stat_generic,
    pub num_2M_pages: u64,
    pub num_1G_pages: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_stat {
    pub generic: kvm_vcpu_stat_generic,
    pub sum_exits: u64,
    pub mmio_exits: u64,
    pub signal_exits: u64,
    pub light_exits: u64,
// Account for special types of light exits:
    pub itlb_real_miss_exits: u64,
    pub itlb_virt_miss_exits: u64,
    pub dtlb_real_miss_exits: u64,
    pub dtlb_virt_miss_exits: u64,
    pub syscall_exits: u64,
    pub isi_exits: u64,
    pub dsi_exits: u64,
    pub emulated_inst_exits: u64,
    pub dec_exits: u64,
    pub ext_intr_exits: u64,
    pub halt_successful_wait: u64,
    pub dbell_exits: u64,
    pub gdbell_exits: u64,
    pub ld: u64,
    pub st: u64,

    pub pf_storage: u64,
    pub pf_instruc: u64,
    pub sp_storage: u64,
    pub sp_instruc: u64,
    pub queue_intr: u64,
    pub ld_slow: u64,
    pub st_slow: u64,

    pub pthru_all: u64,
    pub pthru_host: u64,
    pub pthru_bad_aff: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_exit_types {
    MMIO_EXITS,
    SIGNAL_EXITS,
    ITLB_REAL_MISS_EXITS,
    ITLB_VIRT_MISS_EXITS,
    DTLB_REAL_MISS_EXITS,
    DTLB_VIRT_MISS_EXITS,
    SYSCALL_EXITS,
    ISI_EXITS,
    DSI_EXITS,
    EMULATED_INST_EXITS,
    EMULATED_MTMSRWE_EXITS,
    EMULATED_WRTEE_EXITS,
    EMULATED_MTSPR_EXITS,
    EMULATED_MFSPR_EXITS,
    EMULATED_MTMSR_EXITS,
    EMULATED_MFMSR_EXITS,
    EMULATED_TLBSX_EXITS,
    EMULATED_TLBWE_EXITS,
    EMULATED_RFI_EXITS,
    EMULATED_RFCI_EXITS,
    EMULATED_RFDI_EXITS,
    DEC_EXITS,
    EXT_INTR_EXITS,
    HALT_WAKEUP,
    USR_PR_INST,
    FP_UNAVAIL,
    DEBUG_EXITS,
    TIMEINGUEST,
    DBELL_EXITS,
    GDBELL_EXITS,
    __NUMBER_OF_KVM_EXIT_TYPES
}

// allow access to big endian 32bit upper/lower parts and 64bit var
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_exit_timing {
    pub tv64: u64,
    pub tbl: u32 tbu,,
    pub tv32: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_pginfo {
    pub pfn: c_ulong,
    pub refcnt: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_spapr_tce_iommu_table {
    pub rcu: rcu_head,
    pub next: list_head,
    pub tbl: *mut iommu_table,
    pub kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_spapr_tce_table {
    pub list: list_head,
    pub kvm: *mut kvm,
    pub liobn: u64,
    pub rcu: rcu_head,
    pub page_shift: u32,
    pub /: *mut *mut u64 offset; / in pages,
    pub /: *mut *mut u64 size; / window size in pages,
    pub iommu_tables: list_head,
    pub alloc_lock: mutex,
    pub pages: [*mut page; ],
}

// XICS components, defined in book3s_xics.c
// XIVE components, defined in book3s_xive.c
//
// The reverse mapping array has one entry for each HPTE,
// which stores the guest's view of the second word of the HPTE
// (including the guest physical address of the mapping),
// plus forward and backward pointers in a doubly-linked ring
// of HPTEs that map the same host page.  The pointers in this
// ring are 32-bit HPTE indexes, to save space.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct revmap_entry {
    pub guest_rpte: c_ulong,
    pub back: unsigned int forw,,
}

//
// The rmap array of size number of guest pages is allocated for each memslot.
// This array is used to store usage specific information about the guest page.
// Below are the encodings of the various possible usage types.
//
// Free bits which can be used to define a new usage
pub const KVMPPC_RMAP_TYPE_MASK: c_uint = 0xff00000000000000;
pub const KVMPPC_RMAP_NESTED: c_uint = 0xc000000000000000	/* Nested rmap array */;
pub const KVMPPC_RMAP_HPT: c_uint = 0x0100000000000000	/* HPT guest */;
//
// rmap usage definition for a hash page table (hpt) guest:
// 0x0000080000000000	Lock bit
// 0x0000018000000000	RC bits
// 0x0000000100000000	Present bit
// 0x00000000ffffffff	HPT index bits
// The bottom 32 bits are the index in the guest HPT of a HPTE that points to
// the page.
//
pub const KVMPPC_RMAP_LOCK_BIT: c_int = 43;
pub const KVMPPC_RMAP_RC_SHIFT: c_int = 32;

pub const KVMPPC_RMAP_PRESENT: c_uint = 0x100000000ul;
pub const KVMPPC_RMAP_INDEX: c_uint = 0xfffffffful;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch_memory_slot {

    pub rmap: *mut c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hpt_info {
// Host virtual (linear mapping) address of guest HPT
    pub virt: c_ulong,
// Array of reverse mapping entries for each guest HPTE
    pub rev: *mut revmap_entry,
// Guest HPT size is 2**(order) bytes
    pub order: u32,
// 1 if HPT allocated with CMA, 0 otherwise
    pub cma: c_int,
}

// Flag values for kvm_arch.secure_guest
pub const KVMPPC_SECURE_INIT_START: c_uint = 0x1 /* H_SVM_INIT_START has been called */;
pub const KVMPPC_SECURE_INIT_DONE: c_uint = 0x2 /* H_SVM_INIT_DONE completed */;
pub const KVMPPC_SECURE_INIT_ABORT: c_uint = 0x4 /* H_SVM_INIT_ABORT issued */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arch {
    pub lpid: u64,
    pub /: *mut *mut unsigned int smt_mode; / # vcpus per virtual core,
    pub /: *mut *mut unsigned int emul_smt_mode; / emualted SMT mode, on P9,

    pub tlb_sets: c_uint,
    pub hpt: kvm_hpt_info,
    pub mmio_update: core::sync::atomic::AtomicI64,
    pub host_lpid: c_uint,
    pub host_lpcr: c_ulong,
    pub sdr1: c_ulong,
    pub host_sdr1: c_ulong,
    pub lpcr: c_ulong,
    pub vrma_slb_v: c_ulong,
    pub mmu_ready: c_int,
    pub vcpus_running: core::sync::atomic::AtomicI32,
    pub online_vcores: u32,
    pub hpte_mod_interest: core::sync::atomic::AtomicI32,
    pub need_tlb_flush: cpumask_t,
    pub radix: u8,
    pub fwnmi_enabled: u8,
    pub secure_guest: u8,
    pub svm_enabled: u8,
    pub nested_enable: bool,
    pub dawr1_enabled: bool,
    pub pgtable: *mut pgd_t,
    pub process_table: u64,
    pub /: *mut *mut *mut kvm_resize_hpt resize_hpt; / protected by kvm->lock,

    pub hpt_mutex: mutex,

    pub spapr_tce_tables: list_head,
    pub rtas_tokens: list_head,
    pub rtas_token_lock: mutex,
    pub 1): DECLARE_BITMAP(enabled_hcalls, MAX_HCALL_OPCODE/4 +,

    pub mpic: *mut openpic,

    pub xics: *mut kvmppc_xics,
    pub xics_device: *mut kvmppc_xics,
    pub /: *mut *mut *mut kvmppc_xive xive; / Current XIVE device in use,
    pub native: *mut kvmppc_xive,
    pub xics_on_xive: *mut kvmppc_xive,
    pub xive_devices: },
    pub pimap: *mut kvmppc_passthru_irqmap,

    pub kvm_ops: *mut kvmppc_ops,

    pub uvmem_lock: mutex,
    pub uvmem_pfns: list_head,
    pub /: *mut *mut mutex mmu_setup_lock; / nests inside vcpu mutexes,
    pub l1_ptcr: u64,
    pub kvm_nested_guest_idr: idr,
// This array can grow quite large, keep it at the end
    pub vcores: [*mut kvmppc_vcore; KVM_MAX_VCORES],
}

// This bit is used when a vcore exit is triggered from outside the vcore
pub const VCORE_EXIT_REQ: c_uint = 0x10000;
//
// Values for vcore_state.
// Note that these are arranged such that lower values
// (< VCORE_SLEEPING) don't require stolen time accounting
// on load/unload, and higher values do.
//
pub const VCORE_INACTIVE: c_int = 0;
pub const VCORE_PREEMPT: c_int = 1;
pub const VCORE_PIGGYBACK: c_int = 2;
pub const VCORE_SLEEPING: c_int = 3;
pub const VCORE_RUNNING: c_int = 4;
pub const VCORE_EXITING: c_int = 5;
pub const VCORE_POLLING: c_int = 6;
//
// Struct used to manage memory for a virtual processor area
// registered by a PAPR guest.  There are three types of area
// that a guest can register.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_vpa {
    pub /: *mut *mut unsigned long gpa; / Current guest phys addr,
    pub /: *mut *mut *mut void pinned_addr; / Address in kernel linear mapping,
    pub /: *mut *mut *mut void pinned_end; / End of region,
    pub /: *mut *mut unsigned long next_gpa; / Guest phys addr for update,
    pub /: *mut *mut unsigned long len; / Number of bytes required,
    pub /: *mut *mut u8 update_pending; / 1 => update pinned_addr from next_gpa,
    pub /: *mut *mut bool dirty; / true => area has been modified by kernel,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_pte {
    pub eaddr: c_ulong,
    pub vpage: u64,
    pub raddr: c_ulong,
    pub 1: bool may_read :,
    pub 1: bool may_write :,
    pub 1: bool may_execute :,
    pub wimg: c_ulong,
    pub rc: c_ulong,
    pub /: *mut *mut u8 page_size; / MMU_PAGE_xxx,
    pub page_shift: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_mmu {
// book3s_64 only
    pub rs): *mut *mut *mut void (slbmte)(struct kvm_vcpu vcpu, u64 rb, u64,
    pub slb_nr): *mut *mut *mut u64 (slbmfee)(struct kvm_vcpu vcpu, u64,
    pub slb_nr): *mut *mut *mut u64 (slbmfev)(struct kvm_vcpu vcpu, u64,
    pub ret_slb): *mut *mut *mut int (slbfee)(struct kvm_vcpu vcpu, gva_t eaddr, ulong,
    pub slb_nr): *mut *mut *mut void (slbie)(struct kvm_vcpu vcpu, u64,
    pub vcpu): *mut *mut void (slbia)(struct kvm_vcpu,
// book3s
    pub value): *mut *mut *mut void (mtsrin)(struct kvm_vcpu vcpu, u32 srnum, ulong,
    pub srnum): *mut *mut *mut u32 (mfsrin)(struct kvm_vcpu vcpu, u32,
    pub iswrite): *mut *mut kvmppc_pte pte, bool data, bool,
    pub large): *mut *mut *mut void (tlbie)(struct kvm_vcpu vcpu, ulong addr, bool,
    pub vsid): *mut *mut *mut int (esid_to_vsid)(struct kvm_vcpu vcpu, ulong esid, u64,
    pub data): *mut *mut *mut u64 (ea_to_vp)(struct kvm_vcpu vcpu, gva_t eaddr, bool,
    pub vcpu): *mut *mut bool (is_dcbz32)(struct kvm_vcpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_slb {
    pub esid: u64,
    pub vsid: u64,
    pub orige: u64,
    pub origv: u64,
    pub 1: bool valid :,
    pub 1: bool Ks :,
    pub 1: bool Kp :,
    pub 1: bool nx :,
    pub /: *mut *mut bool large : 1; / PTEs are 16MB,
    pub /: *mut *mut bool tb : 1; / 1TB segment,
    pub 1: bool class :,
    pub /: *mut *mut u8 base_page_size; / MMU_PAGE_xxx,
}

// Struct used to accumulate timing information in HV real mode code
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmhv_tb_accumulator {
    pub /: *mut *mut *mut u64 seqcount; / used to synchronize access, also count  2,
    pub /: *mut *mut u64 tb_total; / total time in timebase ticks,
    pub /: *mut *mut u64 tb_min; / min time,
    pub /: *mut *mut u64 tb_max; / max time,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_irq_map {
    pub r_hwirq: u32,
    pub v_hwirq: u32,
    pub desc: *mut irq_desc,
}

pub const KVMPPC_PIRQ_MAPPED: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_passthru_irqmap {
    pub n_mapped: c_int,
    pub mapped: [kvmppc_irq_map; KVMPPC_PIRQ_MAPPED],
}

pub const KVMPPC_BOOKE_IAC_NUM: c_int = 2;
pub const KVMPPC_BOOKE_DAC_NUM: c_int = 2;

pub const KVMPPC_BOOKE_IAC_NUM: c_int = 4;
pub const KVMPPC_BOOKE_DAC_NUM: c_int = 2;

pub const KVMPPC_BOOKE_MAX_IAC: c_int = 4;
pub const KVMPPC_BOOKE_MAX_DAC: c_int = 2;
// KVMPPC_EPR_USER takes precedence over KVMPPC_EPR_KERNEL

pub const KVMPPC_IRQ_DEFAULT: c_int = 0;
pub const KVMPPC_IRQ_MPIC: c_int = 1;

pub const MMIO_HPTE_CACHE_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmio_hpte_cache_entry {
    pub hpte_v: c_ulong,
    pub hpte_r: c_ulong,
    pub rpte: c_ulong,
    pub pte_index: c_ulong,
    pub eaddr: c_ulong,
    pub slb_v: c_ulong,
    pub mmio_update: c_long,
    pub slb_base_pshift: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmio_hpte_cache {
    pub entry: [mmio_hpte_cache_entry; MMIO_HPTE_CACHE_SIZE],
    pub index: c_uint,
}

pub const KVMPPC_VSX_COPY_NONE: c_int = 0;
pub const KVMPPC_VSX_COPY_WORD: c_int = 1;
pub const KVMPPC_VSX_COPY_DWORD: c_int = 2;
pub const KVMPPC_VSX_COPY_DWORD_LOAD_DUMP: c_int = 3;
pub const KVMPPC_VSX_COPY_WORD_LOAD_DUMP: c_int = 4;
pub const KVMPPC_VMX_COPY_BYTE: c_int = 8;
pub const KVMPPC_VMX_COPY_HWORD: c_int = 9;
pub const KVMPPC_VMX_COPY_WORD: c_int = 10;
pub const KVMPPC_VMX_COPY_DWORD: c_int = 11;
// W0 and W1 of a XIVE thread management context
#[repr(C)]
#[derive(Copy, Clone)]
pub union xive_tma_w01 {
    pub nsr: u8,
    pub cppr: u8,
    pub ipb: u8,
    pub lsmfb: u8,
    pub ack: u8,
    pub inc: u8,
    pub age: u8,
    pub pipr: u8,
}

// Nestedv2 H_GUEST_RUN_VCPU configuration
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmhv_nestedv2_config {
    pub vcpu_run_output_cfg: kvmppc_gs_buff_info,
    pub vcpu_run_input_cfg: kvmppc_gs_buff_info,
    pub vcpu_run_output_size: u64,
}

// Nestedv2 L1<->L0 communication state
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmhv_nestedv2_io {
    pub cfg: kvmhv_nestedv2_config,
    pub vcpu_run_output: *mut kvmppc_gs_buff,
    pub vcpu_run_input: *mut kvmppc_gs_buff,
    pub vcpu_message: *mut kvmppc_gs_msg,
    pub vcore_message: *mut kvmppc_gs_msg,
    pub valids: kvmppc_gs_bitmap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_arch {
    pub host_stack: c_ulong,
    pub host_pid: u32,
    pub slb: [kvmppc_slb; 64],
    pub /: *mut *mut int slb_max; / 1 + index of last valid entry in slb[],
    pub /: *mut *mut int slb_nr; / total number of entries in SLB,
    pub mmu: kvmppc_mmu,
    pub book3s: *mut kvmppc_vcpu_book3s,

    pub shadow_vcpu: *mut kvmppc_book3s_shadow_vcpu,

//
// This is passed along to the HV via H_ENTER_NESTED. Align to
// prevent it crossing a real 4K page.
//
    pub __aligned(512): pt_regs regs,
    pub fp: thread_fp_state,
    pub evr: [c_ulong; 32],
    pub spefscr: c_ulong,
    pub host_spefscr: c_ulong,
    pub acc: u64,

    pub vr: thread_vr_state,

    pub host_mas4: u32,
    pub host_mas6: u32,
    pub shadow_epcr: u32,
    pub shadow_msrp: u32,
    pub eplc: u32,
    pub epsc: u32,
    pub oldpir: u32,

    pub epcr: u32,

// For Gekko paired singles
    pub qpr: [u32; 32],
    pub tar: c_ulong,

    pub hflags: c_ulong,
    pub guest_owned_ext: c_ulong,
    pub purr: c_ulong,
    pub spurr: c_ulong,
    pub ic: c_ulong,
    pub dscr: c_ulong,
    pub amr: c_ulong,
    pub uamor: c_ulong,
    pub iamr: c_ulong,
    pub ctrl: u32,
    pub dabrx: u32,
    pub dabr: c_ulong,
    pub dawr0: c_ulong,
    pub dawrx0: c_ulong,
    pub dawr1: c_ulong,
    pub dawrx1: c_ulong,
    pub dexcr: c_ulong,
    pub hashkeyr: c_ulong,
    pub hashpkeyr: c_ulong,
    pub ciabr: c_ulong,
    pub cfar: c_ulong,
    pub ppr: c_ulong,
    pub pspb: u32,
    pub load_ebb: u8,

    pub load_tm: u8,

    pub fscr: c_ulong,
    pub shadow_fscr: c_ulong,
    pub ebbhr: c_ulong,
    pub ebbrr: c_ulong,
    pub bescr: c_ulong,
    pub csigr: c_ulong,
    pub tacr: c_ulong,
    pub tcscr: c_ulong,
    pub acop: c_ulong,
    pub wort: c_ulong,
    pub tid: c_ulong,
    pub psscr: c_ulong,
    pub hfscr: c_ulong,
    pub shadow_srr1: c_ulong,

    pub /: *mut *mut u32 vrsave; / also USPRG0,
    pub mmucr: u32,
// shadow_msr is unused for BookE HV
    pub shadow_msr: c_ulong,
    pub csrr0: c_ulong,
    pub csrr1: c_ulong,
    pub dsrr0: c_ulong,
    pub dsrr1: c_ulong,
    pub mcsrr0: c_ulong,
    pub mcsrr1: c_ulong,
    pub mcsr: c_ulong,
    pub dec: c_ulong,

    pub decar: u32,

// Time base value when we entered the guest
    pub entry_tb: u64,
    pub entry_vtb: u64,
    pub entry_ic: u64,
    pub tcr: u32,
    pub /: *mut *mut ulong tsr; / we need to perform set/clr_bits() which requires ulong,
    pub ivor: [u32; 64],
    pub ivpr: c_ulong,
    pub pvr: u32,
    pub shadow_pid: u32,
    pub shadow_pid1: u32,
    pub pid: u32,
    pub swap_pid: u32,
    pub ccr0: u32,
    pub ccr1: u32,
    pub dbsr: u32,
    pub /: *mut *mut u64 mmcr[4]; / MMCR0, MMCR1, MMCR2, MMCR3,
    pub mmcra: u64,
    pub mmcrs: u64,
    pub pmc: [u32; 8],
    pub spmc: [u32; 2],
    pub siar: u64,
    pub sdar: u64,
    pub sier: [u64; 3],
    pub tfhar: u64,
    pub texasr: u64,
    pub tfiar: u64,
    pub orig_texasr: u64,
    pub cr_tm: u32,
    pub xer_tm: u64,
    pub lr_tm: u64,
    pub ctr_tm: u64,
    pub amr_tm: u64,
    pub ppr_tm: u64,
    pub dscr_tm: u64,
    pub tar_tm: u64,
    pub gpr_tm: [c_ulong; 32],
    pub fp_tm: thread_fp_state,
    pub vr_tm: thread_vr_state,
    pub /: *mut *mut u32 vrsave_tm; / also USPRG0,

    pub exit_timing_lock: mutex,
    pub timing_exit: kvmppc_exit_timing,
    pub timing_last_enter: kvmppc_exit_timing,
    pub last_exit_type: u32,
    pub timing_count_type: [u32; __NUMBER_OF_KVM_EXIT_TYPES],
    pub timing_sum_duration: [u64; __NUMBER_OF_KVM_EXIT_TYPES],
    pub timing_sum_quad_duration: [u64; __NUMBER_OF_KVM_EXIT_TYPES],
    pub timing_min_duration: [u64; __NUMBER_OF_KVM_EXIT_TYPES],
    pub timing_max_duration: [u64; __NUMBER_OF_KVM_EXIT_TYPES],
    pub timing_last_exit: u64,

    pub fault_dar: c_ulong,
    pub fault_dsisr: u32,
    pub intr_msr: c_ulong,
//
// POWER9 and later: fault_gpa contains the guest real address of page
// fault for a radix guest, or segment descriptor (equivalent to result
// from slbmfev of SLB entry that translated the EA) for hash guests.
//
    pub fault_gpa: c_ulong,

    pub fault_dear: c_ulong,
    pub fault_esr: c_ulong,
    pub queued_dear: c_ulong,
    pub queued_esr: c_ulong,
    pub wdt_lock: spinlock_t,
    pub wdt_timer: timer_list,
    pub tlbcfg: [u32; 4],
    pub tlbps: [u32; 4],
    pub mmucfg: u32,
    pub eptcfg: u32,
    pub epr: u32,
    pub sprg9: u64,
    pub pwrmgtcr0: u32,
    pub crit_save: u32,
// guest debug registers
    pub dbg_reg: debug_reg,

    pub paddr_accessed: gpa_t,
    pub vaddr_accessed: gva_t,
    pub pgdir: *mut pgd_t,
    pub /: *mut *mut u16 io_gpr; / GPR used as IO source/target,
    pub mmio_host_swabbed: u8,
    pub mmio_sign_extend: u8,
// conversion between single and double precision
    pub mmio_sp64_extend: u8,
//
// Number of simulations for vsx.
// If we use 2*8bytes to simulate 1*16bytes,
// then the number should be 2 and
// mmio_copy_type=KVMPPC_VSX_COPY_DWORD.
// If we use 4*4bytes to simulate 1*16bytes,
// the number should be 4 and
// mmio_vsx_copy_type=KVMPPC_VSX_COPY_WORD.
//
    pub mmio_vsx_copy_nums: u8,
    pub mmio_vsx_offset: u8,
    pub mmio_vmx_copy_nums: u8,
    pub mmio_vmx_offset: u8,
    pub mmio_copy_type: u8,
    pub osi_needed: u8,
    pub osi_enabled: u8,
    pub papr_enabled: u8,
    pub watchdog_enabled: u8,
    pub sane: u8,
    pub cpu_type: u8,
    pub hcall_needed: u8,
    pub /: *mut *mut u8 epr_flags; / KVMPPC_EPR_xxx,
    pub epr_needed: u8,
    pub /: *mut *mut u8 external_oneshot; / clear external irq after delivery,
    pub /: *mut *mut u32 cpr0_cfgaddr; / holds the last set cpr0_cfgaddr,
    pub dec_timer: hrtimer,
    pub dec_jiffies: u64,
    pub /: *mut *mut u64 dec_expires; / Relative to guest timebase.,
    pub pending_exceptions: c_ulong,
    pub ceded: u8,
    pub prodded: u8,
    pub doorbell_request: u8,
    pub /: *mut *mut u8 irq_pending; / Used by XIVE to signal pending guest irqs,
    pub last_inst: c_ulong,
    pub wait: rcuwait,
    pub waitp: *mut rcuwait,
    pub vcore: *mut kvmppc_vcore,
    pub ret: c_int,
    pub trap: c_int,
    pub state: c_int,
    pub ptid: c_int,
    pub thread_cpu: c_int,
    pub prev_cpu: c_int,
    pub timer_running: bool,
    pub cpu_run: wait_queue_head_t,
    pub /: *mut *mut machine_check_event mce_evt; / Valid if trap == 0x200,
    pub shared: *mut kvm_vcpu_arch_shared,

    pub shared_big_endian: bool,

    pub /: *mut *mut unsigned long magic_page_pa; / phys addr to map the magic page to,
    pub /: *mut *mut unsigned long magic_page_ea; / effect. addr to map the magic page to,
    pub disable_kernel_nx: bool,
    pub /: *mut *mut *mut int irq_type; / one of KVM_IRQ_,
    pub irq_cpu_id: c_int,
    pub /: *mut *mut *mut openpic mpic; / KVM_IRQ_MPIC,

    pub /: *mut *mut *mut kvmppc_icp icp; / XICS presentation controller,
    pub /: *mut *mut *mut kvmppc_xive_vcpu xive_vcpu; / XIVE virtual CPU data,
    pub /: *mut *mut __be32 xive_cam_word; / Cooked W2 in proper endian with valid bit,
    pub /: *mut *mut u8 xive_pushed; / Is the VP pushed on the physical CPU ?,
    pub /: *mut *mut u8 xive_esc_on; / Is the escalation irq enabled ?,
    pub /: *mut *mut xive_tma_w01 xive_saved_state; / W0..1 of XIVE thread state,
    pub /: *mut *mut u64 xive_esc_raddr; / Escalation interrupt ESB real addr,
    pub /: *mut *mut u64 xive_esc_vaddr; / Escalation interrupt ESB virt addr,

    pub shregs: kvm_vcpu_arch_shared,
    pub mmio_cache: mmio_hpte_cache,
    pub pgfault_addr: c_ulong,
    pub pgfault_index: c_long,
    pub pgfault_hpte: [c_ulong; 2],
    pub pgfault_cache: *mut mmio_hpte_cache_entry,
    pub run_task: *mut task_struct,
    pub vpa_update_lock: spinlock_t,
    pub vpa: kvmppc_vpa,
    pub dtl: kvmppc_vpa,
    pub dtl_ptr: *mut dtl_entry,
    pub dtl_index: c_ulong,
    pub stolen_logged: u64,
    pub slb_shadow: kvmppc_vpa,
    pub tbacct_lock: spinlock_t,
    pub busy_stolen: u64,
    pub busy_preempt: u64,
    pub emul_inst: u64,
    pub online: u32,
    pub /: *mut *mut u64 hfscr_permitted; / A mask of permitted HFSCR facilities,
// For support of nested guests
    pub nested: *mut kvm_nested_guest,
    pub /: *mut *mut u64 nested_hfscr; / HFSCR that the L1 requested for the nested guest,
    pub nested_vcpu_id: u32,
    pub nested_io_gpr: gpa_t,
// For nested APIv2 guests
    pub nestedv2_io: kvmhv_nestedv2_io,

    pub /: *mut *mut *mut kvmhv_tb_accumulator cur_activity; / What we're timing,
    pub /: *mut *mut u64 cur_tb_start; / when it started,

    pub vcpu_entry: kvmhv_tb_accumulator,
    pub vcpu_exit: kvmhv_tb_accumulator,
    pub in_guest: kvmhv_tb_accumulator,
    pub hcall: kvmhv_tb_accumulator,
    pub pg_fault: kvmhv_tb_accumulator,
    pub guest_entry: kvmhv_tb_accumulator,
    pub guest_exit: kvmhv_tb_accumulator,

    pub /: *mut *mut kvmhv_tb_accumulator rm_entry; / real-mode entry code,
    pub /: *mut *mut kvmhv_tb_accumulator rm_intr; / real-mode intr handling,
    pub /: *mut *mut kvmhv_tb_accumulator rm_exit; / real-mode exit code,
    pub /: *mut *mut kvmhv_tb_accumulator guest_time; / guest execution,
    pub /: *mut *mut kvmhv_tb_accumulator cede_time; / time napping inside guest,

    pub l1_to_l2_cs: u64,
    pub l2_to_l1_cs: u64,
    pub l2_runtime_agg: u64,

}

// Values for vcpu->arch.state
pub const KVMPPC_VCPU_NOTREADY: c_int = 0;
pub const KVMPPC_VCPU_RUNNABLE: c_int = 1;
pub const KVMPPC_VCPU_BUSY_IN_HOST: c_int = 2;
// Values for vcpu->arch.io_gpr
pub const KVM_MMIO_REG_MASK: c_uint = 0x003f;
pub const KVM_MMIO_REG_EXT_MASK: c_uint = 0xffc0;
pub const KVM_MMIO_REG_GPR: c_uint = 0x0000;
pub const KVM_MMIO_REG_FPR: c_uint = 0x0040;
pub const KVM_MMIO_REG_QPR: c_uint = 0x0080;
pub const KVM_MMIO_REG_FQPR: c_uint = 0x00c0;
pub const KVM_MMIO_REG_VSX: c_uint = 0x0100;
pub const KVM_MMIO_REG_VMX: c_uint = 0x0180;
pub const KVM_MMIO_REG_NESTED_GPR: c_uint = 0xffc0;
