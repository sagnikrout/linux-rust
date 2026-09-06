//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_ppc.h
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
// Copyright IBM Corp. 2008
//
// Authors: Hollis Blanchard <hollisb@us.ibm.com>
//
// This file exists just so we can dereference kvm_vcpu, avoiding nested header
// dependencies.

//
// KVMPPC_INST_SW_BREAKPOINT is debug Instruction
// for supporting software breakpoint.
//
pub const KVMPPC_INST_SW_BREAKPOINT: c_uint = 0x00dddd00;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum emulation_result {
    EMULATE_DONE,         /* no further processing */
    EMULATE_DO_MMIO,      /* kvm_run filled with MMIO request */
    EMULATE_FAIL,         /* can't emulate this instruction */
    EMULATE_AGAIN,        /* something went wrong. go again */
    EMULATE_EXIT_USER,    /* emulation requires exit to user-space */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum instruction_fetch_type {
    INST_GENERIC,
    INST_SC,		/* system call */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xlate_instdata {
    XLATE_INST,		/* translate instruction address */
    XLATE_DATA		/* translate data address */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xlate_readwrite {
    XLATE_READ,		/* check for read permissions */
    XLATE_WRITE		/* check for write permissions */
}

extern "C" {
    pub fn kvmppc_vcpu_run(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn __kvmppc_vcpu_run(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_handler_highmem();
}
extern "C" {
    pub fn kvmppc_dump_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_emulate_instruction(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_emulate_loadstore(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_emulate_mmio(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_emulate_dec(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_get_dec(vcpu: *mut kvm_vcpu, tb: u64) -> u32;
}
extern "C" {
    pub fn kvmppc_decrementer_func(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_sanity_check(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_subarch_vcpu_init(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_subarch_vcpu_uninit(vcpu: *mut kvm_vcpu);
}
// Core-specific hooks
extern "C" {
    pub fn kvmppc_mmu_switch_pid(vcpu: *mut kvm_vcpu, pid: u32);
}
extern "C" {
    pub fn kvmppc_mmu_dtlb_index(vcpu: *mut kvm_vcpu, eaddr: gva_t) -> c_int;
}
extern "C" {
    pub fn kvmppc_mmu_itlb_index(vcpu: *mut kvm_vcpu, eaddr: gva_t) -> c_int;
}
extern "C" {
    pub fn kvmppc_mmu_dtlb_miss(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_mmu_itlb_miss(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_vcpu_create(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_core_vcpu_free(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_vcpu_setup(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_core_vcpu_load(vcpu: *mut kvm_vcpu, cpu: c_int);
}
extern "C" {
    pub fn kvmppc_core_vcpu_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_prepare_to_enter(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_core_pending_dec(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_core_queue_syscall(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_queue_dec(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_dequeue_dec(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_dequeue_external(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_queue_itlb_miss(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_flush_tlb(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_check_requests(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_booke_init() -> c_int;
}
extern "C" {
    pub fn kvmppc_booke_exit();
}
extern "C" {
    pub fn kvmppc_kvm_pv(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_map_magic(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_allocate_hpt(info: *mut kvm_hpt_info, order: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_set_hpt(kvm: *mut kvm, info: *mut kvm_hpt_info);
}
extern "C" {
    pub fn kvmppc_alloc_reset_hpt(kvm: *mut kvm, order: c_int) -> c_int;
}
extern "C" {
    pub fn kvmppc_free_hpt(info: *mut kvm_hpt_info);
}
extern "C" {
    pub fn kvmppc_rmap_reset(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmppc_pseries_do_hcall(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_switch_mmu_to_hpt(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvmppc_switch_mmu_to_radix(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvmppc_setup_partition_table(kvm: *mut kvm);
}

extern "C" {
    pub fn kvm_free_hpt_cma(page: *mut page, nr_pages: c_ulong);
}
extern "C" {
    pub fn kvmppc_core_init_vm(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvmppc_core_destroy_vm(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmppc_bookehv_init() -> c_int;
}
extern "C" {
    pub fn kvmppc_bookehv_exit();
}
extern "C" {
    pub fn kvmppc_prepare_to_enter(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_get_htab_fd(kvm: *mut kvm, : *mut kvm_get_htab_fd) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_ioctl_interrupt(vcpu: *mut kvm_vcpu, irq: *mut kvm_interrupt) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_rtas_define_token(kvm: *mut kvm, argp: *mut void __user) -> c_int;
}
extern "C" {
    pub fn kvmppc_rtas_hcall(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_rtas_tokens_free(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmppc_xics_int_on(kvm: *mut kvm, irq: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_xics_int_off(kvm: *mut kvm, irq: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_core_dequeue_debug(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_core_queue_debug(vcpu: *mut kvm_vcpu);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvmppc_one_reg {
    pub wval: u32,
    pub dval: u64,
    pub vval: vector128,
    pub vsxval: [u64; 2],
    pub vsx32val: [u32; 4],
    pub vsx16val: [u16; 8],
    pub vsx8val: [u8; 16],
    pub addr: u64,
    pub length: u64,
    pub vpaval: },
    pub xive_timaval: [u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_ops {
    pub owner: *mut module,
    pub sregs): *mut *mut *mut int (get_sregs)(struct kvm_vcpu vcpu, struct kvm_sregs,
    pub sregs): *mut *mut *mut int (set_sregs)(struct kvm_vcpu vcpu, struct kvm_sregs,
    pub val): *mut kvmppc_one_reg,
    pub val): *mut kvmppc_one_reg,
    pub cpu): *mut *mut *mut void (vcpu_load)(struct kvm_vcpu vcpu, int,
    pub vcpu): *mut *mut void (vcpu_put)(struct kvm_vcpu,
    pub srr1_flags): *mut *mut *mut void (inject_interrupt)(struct kvm_vcpu vcpu, int vec, u64,
    pub msr): *mut *mut *mut void (set_msr)(struct kvm_vcpu vcpu, u64,
    pub vcpu): *mut *mut int (vcpu_run)(struct kvm_vcpu,
    pub vcpu): *mut *mut int (vcpu_create)(struct kvm_vcpu,
    pub vcpu): *mut *mut void (vcpu_free)(struct kvm_vcpu,
    pub vcpu): *mut *mut int (check_requests)(struct kvm_vcpu,
    pub log): *mut *mut *mut int (get_dirty_log)(struct kvm kvm, struct kvm_dirty_log,
    pub memslot): *mut *mut *mut void (flush_memslot)(struct kvm kvm, struct kvm_memory_slot,
    pub change): kvm_mr_change,
    pub change): kvm_mr_change,
    pub range): *mut *mut *mut bool (unmap_gfn_range)(struct kvm kvm, struct kvm_gfn_range,
    pub range): *mut *mut *mut bool (age_gfn)(struct kvm kvm, struct kvm_gfn_range,
    pub range): *mut *mut *mut bool (test_age_gfn)(struct kvm kvm, struct kvm_gfn_range,
    pub slot): *mut *mut void (free_memslot)(struct kvm_memory_slot,
    pub kvm): *mut *mut int (init_vm)(struct kvm,
    pub kvm): *mut *mut void (destroy_vm)(struct kvm,
    pub info): *mut *mut *mut int (get_smmu_info)(struct kvm kvm, struct kvm_ppc_smmu_info,
    pub advance): *mut unsigned int inst, int,
    pub spr_val): *mut *mut *mut int (emulate_mtspr)(struct kvm_vcpu vcpu, int sprn, ulong,
    pub spr_val): *mut *mut *mut int (emulate_mfspr)(struct kvm_vcpu vcpu, int sprn, ulong,
    pub vcpu): *mut *mut void (fast_vcpu_kick)(struct kvm_vcpu,
    pub arg): c_ulong,
    pub hcall): *mut *mut int (hcall_implemented)(unsigned long,
    pub ): *mut irq_bypass_producer,
    pub ): *mut irq_bypass_producer,
    pub cfg): *mut *mut *mut int (configure_mmu)(struct kvm kvm, struct kvm_ppc_mmuv3_cfg,
    pub info): *mut *mut *mut int (get_rmmu_info)(struct kvm kvm, struct kvm_ppc_rmmu_info,
    pub flags): c_ulong,
    pub msr): *mut *mut *mut void (giveup_ext)(struct kvm_vcpu vcpu, ulong,
    pub kvm): *mut *mut int (enable_nested)(struct kvm,
    pub size): c_int,
    pub size): c_int,
    pub kvm): *mut *mut int (enable_svm)(struct kvm,
    pub kvm): *mut *mut int (svm_off)(struct kvm,
    pub kvm): *mut *mut int (enable_dawr1)(struct kvm,
    pub (*hash_v3_possible)(void): *mut bool,
    pub kvm): *mut *mut int (create_vm_debugfs)(struct kvm,
    pub debugfs_dentry): *mut *mut *mut int (create_vcpu_debugfs)(struct kvm_vcpu vcpu, struct dentry,
    pub host_caps): *mut *mut int (get_compat_caps)(struct kvm_ppc_compat_caps,
}

// Load the instruction manually if it failed to do so in the
// exit path
// Write fetch_failed unswapped if the fetch failed
// inst = ppc_inst(KVM_INST_FETCH_FAILED);

// Is this a prefixed instruction?
// inst = ppc_inst_prefix(prefix, suffix);

// inst = ppc_inst(fetched_inst);
extern "C" {
    pub fn kvmppc_hwrng_present() -> c_int;
}
//
// Cuts out inst bits with ordering according to spec.
// That means the leftmost bit is zero. All given bits are included.
//
// Replaces inst bits with ordering according to spec.
//

extern "C" {
    pub fn kvmppc_core_get_sregs(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> c_int;
}
extern "C" {
    pub fn kvmppc_core_set_sregs(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> c_int;
}
extern "C" {
    pub fn kvmppc_get_sregs_ivor(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> c_int;
}
extern "C" {
    pub fn kvmppc_set_sregs_ivor(vcpu: *mut kvm_vcpu, sregs: *mut kvm_sregs) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_ioctl_get_one_reg(vcpu: *mut kvm_vcpu, reg: *mut kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvm_vcpu_ioctl_set_one_reg(vcpu: *mut kvm_vcpu, reg: *mut kvm_one_reg) -> c_int;
}
extern "C" {
    pub fn kvmppc_get_one_reg(vcpu: *mut kvm_vcpu, id: u64, : *mut kvmppc_one_reg) -> c_int;
}
extern "C" {
    pub fn kvmppc_set_one_reg(vcpu: *mut kvm_vcpu, id: u64, : *mut kvmppc_one_reg) -> c_int;
}
extern "C" {
    pub fn kvmppc_set_pid(vcpu: *mut kvm_vcpu, pid: u32);
}

//
// To avoid the need to unnecessarily exit fully to the host kernel, an IPI to
// a CPU thread that's running/napping inside of a guest is by default regarded
// as a request to wake the CPU (if needed) and continue execution within the
// guest, potentially to process new state like externally-generated
// interrupts or IPIs sent from within the guest itself (e.g. H_PROD/H_IPI).
//
// To force an exit to the host kernel, kvmppc_set_host_ipi() must be called
// prior to issuing the IPI to set the corresponding 'host_ipi' flag in the
// target CPU's PACA. To avoid unnecessary exits to the host, this flag should
// be immediately cleared via kvmppc_clear_host_ipi() by the IPI handler on
// the receiving side prior to processing the IPI work.
//
// NOTE:
//
// We currently issue an smp_mb() at the beginning of kvmppc_set_host_ipi().
// This is to guard against sequences such as the following:
//
// CPU
// X: smp_muxed_ipi_set_message():
// X:   smp_mb()
// X:   message[RESCHEDULE] = 1
// X: doorbell_global_ipi(42):
// X:   kvmppc_set_host_ipi(42)
// X:   ppc_msgsnd_sync()/smp_mb()
// X:   ppc_msgsnd() -> 42
// 42: doorbell_exception(): // from CPU X
// 42:   ppc_msgsync()
// 105: smp_muxed_ipi_set_message():
// 105:   smb_mb()
// // STORE DEFERRED DUE TO RE-ORDERING
// --105:   message[CALL_FUNCTION] = 1
// | 105: doorbell_global_ipi(42):
// | 105:   kvmppc_set_host_ipi(42)
// |  42:   kvmppc_clear_host_ipi(42)
// |  42: smp_ipi_demux_relaxed()
// |  42: // returns to executing guest
// |      // RE-ORDERED STORE COMPLETES
// ->105:   message[CALL_FUNCTION] = 1
// 105:   ppc_msgsnd_sync()/smp_mb()
// 105:   ppc_msgsnd() -> 42
// 42: local_paca->kvm_hstate.host_ipi == 0 // IPI ignored
// 105: // hangs waiting on 42 to process messages/call_single_queue
//
// We also issue an smp_mb() at the end of kvmppc_clear_host_ipi(). This is
// to guard against sequences such as the following (as well as to create
// a read-side pairing with the barrier in kvmppc_set_host_ipi()):
//
// CPU
// X: smp_muxed_ipi_set_message():
// X:   smp_mb()
// X:   message[RESCHEDULE] = 1
// X: doorbell_global_ipi(42):
// X:   kvmppc_set_host_ipi(42)
// X:   ppc_msgsnd_sync()/smp_mb()
// X:   ppc_msgsnd() -> 42
// 42: doorbell_exception(): // from CPU X
// 42:   ppc_msgsync()
// // STORE DEFERRED DUE TO RE-ORDERING
// -- 42:   kvmppc_clear_host_ipi(42)
// |  42: smp_ipi_demux_relaxed()
// | 105: smp_muxed_ipi_set_message():
// | 105:   smb_mb()
// | 105:   message[CALL_FUNCTION] = 1
// | 105: doorbell_global_ipi(42):
// | 105:   kvmppc_set_host_ipi(42)
// |      // RE-ORDERED STORE COMPLETES
// -> 42:   kvmppc_clear_host_ipi(42)
// 42: // returns to executing guest
// 105:   ppc_msgsnd_sync()/smp_mb()
// 105:   ppc_msgsnd() -> 42
// 42: local_paca->kvm_hstate.host_ipi == 0 // IPI ignored
// 105: // hangs waiting on 42 to process messages/call_single_queue
//
// order stores of IPI messages vs. setting of host_ipi flag
//
// pairs with the barrier in kvmppc_clear_host_ipi()
//
// order clearing of host_ipi flag vs. processing of IPI messages
//
// pairs with the barrier in kvmppc_set_host_ipi()
//
extern "C" {
    pub fn kvm_hv_vm_activated();
}
extern "C" {
    pub fn kvm_hv_vm_deactivated();
}
extern "C" {
    pub fn kvm_hv_mode_active() -> bool;
}
extern "C" {
    pub fn kvmppc_check_need_tlb_flush(kvm: *mut kvm, pcpu: c_int);
}

extern "C" {
    pub fn kvmppc_alloc_host_rm_ops();
}
extern "C" {
    pub fn kvmppc_free_host_rm_ops();
}
extern "C" {
    pub fn kvmppc_free_pimap(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmppc_xics_rm_complete(vcpu: *mut kvm_vcpu, hcall: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_xics_free_icp(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_xics_hcall(vcpu: *mut kvm_vcpu, cmd: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_xive_xics_hcall(vcpu: *mut kvm_vcpu, req: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_xics_get_icp(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvmppc_xics_set_icp(vcpu: *mut kvm_vcpu, icpval: u64) -> c_int;
}
extern "C" {
    pub fn kvmppc_xics_ipi_action();
}

//
// Below the first "xive" is the "eXternal Interrupt Virtualization Engine"
// ie. P9 new interrupt controller, while the second "xive" is the legacy
// "eXternal Interrupt Vector Entry" which is the configuration of an
// interrupt on the "xics" interrupt controller on P8 and earlier. Those
// two function consume or produce a legacy "XIVE" state from the
// new "XIVE" interrupt controller.
//
extern "C" {
    pub fn kvmppc_xive_int_on(kvm: *mut kvm, irq: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_xive_int_off(kvm: *mut kvm, irq: u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_xive_cleanup_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_xive_get_icp(vcpu: *mut kvm_vcpu) -> u64;
}
extern "C" {
    pub fn kvmppc_xive_set_icp(vcpu: *mut kvm_vcpu, icpval: u64) -> c_int;
}
extern "C" {
    pub fn kvmppc_xive_push_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_xive_pull_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_xive_rearm_escalation(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvmppc_xive_native_cleanup_vcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_xive_native_supported() -> bool;
}

extern "C" {
    pub fn xive_enabled(cpu_has_feature(CPU_FTR_HVMODE: ) &&) -> return;
}

//
// Prototypes for functions called only from assembler code.
// Having prototypes reduces sparse errors.
//
extern "C" {
    pub fn kvmppc_rm_h_random(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvmhv_commence_exit(trap: c_int);
}
extern "C" {
    pub fn kvmppc_realmode_machine_check(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_subcore_enter_guest();
}
extern "C" {
    pub fn kvmppc_subcore_exit_guest();
}
extern "C" {
    pub fn kvmppc_realmode_hmi_handler() -> c_long;
}
extern "C" {
    pub fn kvmppc_p9_realmode_hmi_handler(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvmppc_h_bulk_remove(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvmppc_guest_entry_inject_int(vcpu: *mut kvm_vcpu);
}
//
// Host-side operations we want to set up while running in real
// mode in the guest operating on the xics.
// Currently only VCPU wakeup is supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union kvmppc_rm_state {
    pub raw: c_ulong,
    pub in_host: u32,
    pub rm_action: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_host_rm_core {
    pub rm_state: kvmppc_rm_state,
    pub rm_data: *mut c_void,
    pub pad: [c_char; 112],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_host_rm_ops {
    pub rm_core: *mut kvmppc_host_rm_core,
    pub vcpu): *mut *mut void (vcpu_kick)(struct kvm_vcpu,
}

extern "C" {
    pub fn mfspr(_arg: SPRN_GEPR) -> return;
}

extern "C" {
    pub fn kvmppc_mpic_set_epr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_mpic_disconnect_vcpu(opp: *mut openpic, vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvmppc_alloc_lpid() -> c_long;
}
extern "C" {
    pub fn kvmppc_free_lpid(lpid: c_long);
}
extern "C" {
    pub fn kvmppc_init_lpid(nr_lpids: c_ulong);
}
//
// We can only access pages that the kernel maps
// as memory. Bail out for unmapped ones.
//
// Clear i-cache for new pages
//
// Shared struct helpers. The shared struct can be little or big endian,
// depending on the guest endianness. So expose helpers to all of them.
//

// Only Book3S_64 PR supports bi-endian for now

// Book3s_64 HV on little endian is always little endian

extern "C" {
    pub fn be32_to_cpu(_arg: vcpu->arch.shared->sr[nr]) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: vcpu->arch.shared->sr[nr]) -> return;
}
//
// Please call after prepare_to_enter. This function puts the lazy ee and irq
// disabled tracking state back to normal mode, without actually enabling
// interrupts.
//

//
// To avoid races, the caller must have gone directly from having
// interrupts fully-enabled to hard-disabled.
//
// Only need to enable IRQs by hard enabling them after this

// Only need to enable IRQs by hard enabling them after this

extern "C" {
    pub fn xics_wake_cpu(cpu: c_int);
}
