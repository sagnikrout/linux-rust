//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_book3s.h
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
// Copyright SUSE Linux Products GmbH 2009
//
// Authors: Alexander Graf <agraf@suse.de>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_bat {
    pub raw: u64,
    pub bepi: u32,
    pub bepi_mask: u32,
    pub brpn: u32,
    pub wimg: u8,
    pub pp: u8,
    pub 1: bool vs :,
    pub 1: bool vp :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_sid_map {
    pub guest_vsid: u64,
    pub guest_esid: u64,
    pub host_vsid: u64,
    pub 1: bool valid :,
}

pub const SID_MAP_BITS: c_int = 9;

pub const SID_CONTEXTS: c_int = 1;

pub const SID_CONTEXTS: c_int = 128;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpte_cache {
    pub list_pte: hlist_node,
    pub list_pte_long: hlist_node,
    pub list_vpte: hlist_node,
    pub list_vpte_long: hlist_node,

    pub list_vpte_64k: hlist_node,

    pub rcu_head: rcu_head,
    pub host_vpn: u64,
    pub pfn: u64,
    pub slot: c_ulong,
    pub pte: kvmppc_pte,
    pub pagesize: c_int,
}

//
// Struct for a virtual core.
// Note: entry_exit_map combines a bitmap of threads that have entered
// in the bottom 8 bits and a bitmap of threads that have exited in the
// next 8 bits.  This is so that we can atomically set the entry bit
// iff the exit map is 0 without taking a lock.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_vcore {
    pub n_runnable: c_int,
    pub num_threads: c_int,
    pub entry_exit_map: c_int,
    pub napping_threads: c_int,
    pub first_vcpuid: c_int,
    pub pcpu: u16,
    pub last_cpu: u16,
    pub vcore_state: u8,
    pub in_guest: u8,
    pub runnable_threads: [*mut kvm_vcpu; MAX_SMT_THREADS],
    pub preempt_list: list_head,
    pub lock: spinlock_t,
    pub wait: rcuwait,
    pub /: *mut *mut spinlock_t stoltb_lock; / protects stolen_tb and preempt_tb,
    pub stolen_tb: u64,
    pub preempt_tb: u64,
    pub runner: *mut kvm_vcpu,
    pub kvm: *mut kvm,
    pub /: *mut *mut u64 tb_offset; / guest timebase - host timebase,
    pub /: *mut *mut u64 tb_offset_applied; / timebase offset currently in force,
    pub lpcr: c_ulong,
    pub arch_compat: u32,
    pub pcr: c_ulong,
    pub /: *mut *mut ulong dpdes; / doorbell state (POWER8),
    pub /: *mut *mut ulong vtb; / virtual timebase,
    pub conferring_threads: c_ulong,
    pub halt_poll_ns: c_uint,
    pub online_count: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_vcpu_book3s {
    pub sid_map: [kvmppc_sid_map; SID_MAP_NUM],
    pub esid: u64,
    pub vsid: u64,
    pub slb_shadow: [}; 64],
    pub slb_shadow_max: u8,
    pub ibat: [kvmppc_bat; 8],
    pub dbat: [kvmppc_bat; 8],
    pub hid: [u64; 6],
    pub gqr: [u64; 8],
    pub sdr1: u64,
    pub hior: u64,
    pub msr_mask: u64,
    pub vtb: u64,
    pub vsid_pool: [u32; VSID_POOL_SIZE],
    pub vsid_next: u32,

    pub proto_vsid_first: u64,
    pub proto_vsid_max: u64,
    pub proto_vsid_next: u64,
    pub context_id: [c_int; SID_CONTEXTS],
    pub /: *mut *mut bool hior_explicit; / HIOR is set by ioctl, not PVR,
    pub hpte_hash_pte: [hlist_head; HPTEG_HASH_NUM_PTE],
    pub hpte_hash_pte_long: [hlist_head; HPTEG_HASH_NUM_PTE_LONG],
    pub hpte_hash_vpte: [hlist_head; HPTEG_HASH_NUM_VPTE],
    pub hpte_hash_vpte_long: [hlist_head; HPTEG_HASH_NUM_VPTE_LONG],    pub hpte_hash_vpte_64k: [hlist_head; HPTEG_HASH_NUM_VPTE_64K],
    pub hpte_cache_count: c_int,
    pub mmu_lock: spinlock_t,
}

pub const VSID_REAL: c_uint = 0x07ffffffffc00000ULL;
pub const VSID_BAT: c_uint = 0x07ffffffffb00000ULL;
pub const VSID_64K: c_uint = 0x0800000000000000ULL;
pub const VSID_1T: c_uint = 0x1000000000000000ULL;
pub const VSID_REAL_DR: c_uint = 0x2000000000000000ULL;
pub const VSID_REAL_IR: c_uint = 0x4000000000000000ULL;
pub const VSID_PR: c_uint = 0x8000000000000000ULL;
extern "C" {
    pub fn kvmppc_mmu_pte_flush(vcpu: *mut kvm_vcpu, ea: c_ulong, ea_mask: c_ulong);
}
extern "C" {
    pub fn kvmppc_mmu_pte_vflush(vcpu: *mut kvm_vcpu, vp: u64, vp_mask: u64);
}
extern "C" {
    pub fn kvmppc_mmu_pte_pflush(vcpu: *mut kvm_vcpu, pa_start: c_ulong, pa_end: c_ulong);
}
extern "C" {
    pub fn kvmppc_set_msr(vcpu: *mut kvm_vcpu, new_msr: u64);
}
extern "C" {
    pub fn kvmppc_mmu_book3s_64_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_mmu_book3s_32_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_mmu_book3s_hv_init(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_mmu_unmap_page(vcpu: *mut kvm_vcpu, pte: *mut kvmppc_pte);
}
extern "C" {
    pub fn kvmppc_mmu_map_segment(vcpu: *mut kvm_vcpu, eaddr: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmppc_mmu_flush_segment(vcpu: *mut kvm_vcpu, eaddr: c_ulong, seg_size: c_ulong);
}
extern "C" {
    pub fn kvmppc_mmu_flush_segments(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_mmu_hpte_cache_map(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache);
}
extern "C" {
    pub fn kvmppc_mmu_hpte_cache_free(pte: *mut hpte_cache);
}
extern "C" {
    pub fn kvmppc_mmu_hpte_destroy(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_mmu_hpte_init(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_mmu_invalidate_pte(vcpu: *mut kvm_vcpu, pte: *mut hpte_cache);
}
extern "C" {
    pub fn kvmppc_mmu_hpte_sysinit() -> c_int;
}
extern "C" {
    pub fn kvmppc_mmu_hpte_sysexit();
}
extern "C" {
    pub fn kvmppc_mmu_hv_init() -> c_int;
}
extern "C" {
    pub fn kvmppc_book3s_hcall_implemented(kvm: *mut kvm, hc: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmppc_init_vm_radix(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvmppc_free_radix(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmppc_radix_init() -> c_int;
}
extern "C" {
    pub fn kvmppc_radix_exit();
}
extern "C" {
    pub fn kvmhv_get_rmmu_info(kvm: *mut kvm, info: *mut kvm_ppc_rmmu_info) -> c_int;
}
// XXX remove this export when load_last_inst() is generic
extern "C" {
    pub fn kvmppc_ld(vcpu: *mut kvm_vcpu, eaddr: *mut c_ulong, size: c_int, ptr: *mut c_void, data: bool) -> c_int;
}
extern "C" {
    pub fn kvmppc_book3s_queue_irqprio(vcpu: *mut kvm_vcpu, vec: c_uint);
}
extern "C" {
    pub fn kvmppc_inject_interrupt(vcpu: *mut kvm_vcpu, vec: c_int, flags: u64);
}
extern "C" {
    pub fn kvmppc_trigger_fac_interrupt(vcpu: *mut kvm_vcpu, fac: c_ulong);
}
extern "C" {
    pub fn kvmppc_giveup_ext(vcpu: *mut kvm_vcpu, msr: c_ulong);
}
extern "C" {
    pub fn kvmppc_emulate_paired_single(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_set_fscr(vcpu: *mut kvm_vcpu, fscr: u64);
}
extern "C" {
    pub fn kvmhv_p9_tm_emulation_early(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmhv_p9_tm_emulation(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmhv_emulate_tm_rollback(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_entry_trampoline();
}
extern "C" {
    pub fn kvmppc_hv_entry_trampoline();
}
extern "C" {
    pub fn kvmppc_alignment_dsisr(vcpu: *mut kvm_vcpu, inst: c_uint) -> u32;
}
extern "C" {
    pub fn kvmppc_alignment_dar(vcpu: *mut kvm_vcpu, inst: c_uint) -> c_ulong;
}
extern "C" {
    pub fn kvmppc_h_pr(vcpu: *mut kvm_vcpu, cmd: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmppc_pr_init_default_hcalls(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmppc_hcall_impl_pr(cmd: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmppc_hcall_impl_hv_realmode(cmd: c_ulong) -> c_int;
}
extern "C" {
    pub fn kvmppc_copy_to_svcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_copy_from_svcpu(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_read_intr() -> c_long;
}
extern "C" {
    pub fn kvmppc_set_msr_hv(vcpu: *mut kvm_vcpu, msr: u64);
}
extern "C" {
    pub fn kvmppc_inject_interrupt_hv(vcpu: *mut kvm_vcpu, vec: c_int, srr1_flags: u64);
}

extern "C" {
    pub fn kvmppc_save_tm_pr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_restore_tm_pr(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_save_tm_sprs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_restore_tm_sprs(vcpu: *mut kvm_vcpu);
}

extern "C" {
    pub fn kvmhv_nested_init() -> c_long;
}
extern "C" {
    pub fn kvmhv_nested_exit();
}
extern "C" {
    pub fn kvmhv_vm_nested_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmhv_set_partition_table(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvmhv_copy_tofrom_guest_nested(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvmhv_flush_lpid(lpid: u64);
}
extern "C" {
    pub fn kvmhv_set_ptbl_entry(lpid: u64, dw0: u64, dw1: u64);
}
extern "C" {
    pub fn kvmhv_release_all_nested(kvm: *mut kvm);
}
extern "C" {
    pub fn kvmhv_enter_nested_guest(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvmhv_do_nested_tlbie(vcpu: *mut kvm_vcpu) -> c_long;
}
extern "C" {
    pub fn kvmhv_save_hv_regs(vcpu: *mut kvm_vcpu, hr: *mut hv_guest_state);
}
extern "C" {
    pub fn kvmhv_nested_page_fault(vcpu: *mut kvm_vcpu) -> long int;
}
extern "C" {
    pub fn kvmppc_giveup_fac(vcpu: *mut kvm_vcpu, fac: c_ulong);
}

extern "C" {
    pub fn static_branch_unlikely(_arg: &__kvmhv_is_nestedv2) -> return;
}

extern "C" {
    pub fn __kvmhv_nestedv2_reload_ptregs(vcpu: *mut kvm_vcpu, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn __kvmhv_nestedv2_mark_dirty_ptregs(vcpu: *mut kvm_vcpu, regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn __kvmhv_nestedv2_mark_dirty(vcpu: *mut kvm_vcpu, iden: u16) -> c_int;
}
extern "C" {
    pub fn __kvmhv_nestedv2_cached_reload(vcpu: *mut kvm_vcpu, iden: u16) -> c_int;
}
extern "C" {
    pub fn __kvmhv_nestedv2_reload_ptregs(_arg: vcpu, _arg: regs) -> return;
}
extern "C" {
    pub fn __kvmhv_nestedv2_mark_dirty_ptregs(_arg: vcpu, _arg: regs) -> return;
}
extern "C" {
    pub fn __kvmhv_nestedv2_mark_dirty(_arg: vcpu, _arg: iden) -> return;
}
extern "C" {
    pub fn __kvmhv_nestedv2_cached_reload(_arg: vcpu, _arg: iden) -> return;
}
// Also add subarch specific defines

extern "C" {
    pub fn kvmppc_get_msr(vcpu: *mut kvm_vcpu) -> u64;
}

// v =  vcpu->arch.vr.vr[i];

// Expiry time of vcpu DEC relative to host TB
extern "C" {
    pub fn kvmppc_get_dec_expires(kvmppc_get_tb_offset(vcpu: vcpu) -) -> return;
}
extern "C" {
    pub fn is_kvmppc_hv_enabled(kvm: *mut kvm) -> bool;
}
// Only PR KVM supports the magic page
extern "C" {
    pub fn kvmppc_h_logical_ci_load(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_h_logical_ci_store(vcpu: *mut kvm_vcpu) -> c_int;
}
// Magic register values loaded into r3 and r4 before the 'sc' assembly
// instruction for the OSI hypercalls
pub const OSI_SC_MAGIC_R3: c_uint = 0x113724FA;
pub const OSI_SC_MAGIC_R4: c_uint = 0x77810F9B;
pub const INS_DCBZ: c_uint = 0x7c0007ec;
// TO = 31 for unconditional trap
pub const INS_TW: c_uint = 0x7fe00008;
pub const SPLIT_HACK_MASK: c_uint = 0xff000000;
pub const SPLIT_HACK_OFFS: c_uint = 0xfb000000;
//
// This packs a VCPU ID from the [0..KVM_MAX_VCPU_IDS) space down to the
// [0..KVM_MAX_VCPUS) space, using knowledge of the guest's core stride
// (but not its actual threading mode, which is not available) to avoid
// collisions.
//
// The implementation leaves VCPU IDs from the range [0..KVM_MAX_VCPUS) (block
// 0) unchanged: if the guest is filling each VCORE completely then it will be
// using consecutive IDs and it will fill the space without any packing.
//
// For higher VCPU IDs, the packed ID is based on the VCPU ID modulo
// KVM_MAX_VCPUS (effectively masking off the top bits) and then an offset is
// added to avoid collisions.
//
// VCPU IDs in the range [KVM_MAX_VCPUS..(KVM_MAX_VCPUS*2)) (block 1) are only
// possible if the guest is leaving at least 1/2 of each VCORE empty, so IDs
// can be safely packed into the second half of each VCORE by adding an offset
// of (stride / 2).
//
// Similarly, if VCPU IDs in the range [(KVM_MAX_VCPUS*2)..(KVM_MAX_VCPUS*4))
// (blocks 2 and 3) are seen, the guest must be leaving at least 3/4 of each
// VCORE empty so packed IDs can be offset by (stride / 4) and (stride * 3 / 4).
//
// Finally, VCPU IDs from blocks 5..7 will only be seen if the guest is using a
// stride of 8 and 1 thread per core so the remaining offsets of 1, 5, 3 and 7
// must be free to use.
//
// (The offsets for each block are stored in block_offsets[], indexed by the
// block number if the stride is 8. For cases where the guest's stride is less
// than 8, we can re-use the block_offsets array by multiplying the block
// number by (MAX_SMT_THREADS / stride) to reach the correct entry.)
//
