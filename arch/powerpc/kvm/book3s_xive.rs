//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/kvm/book3s_xive.h
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
// Copyright 2017 Benjamin Herrenschmidt, IBM Corporation
//

//
// The XIVE Interrupt source numbers are within the range 0 to
// KVMPPC_XICS_NR_IRQS.
//
pub const KVMPPC_XIVE_FIRST_IRQ: c_int = 0;

//
// State for one guest irq source.
//
// For each guest source we allocate a HW interrupt in the XIVE
// which we use for all SW triggers. It will be unused for
// pass-through but it's easier to keep around as the same
// guest interrupt can alternatively be emulated or pass-through
// if a physical device is hot unplugged and replaced with an
// emulated one.
//
// This state structure is very similar to the XICS one with
// additional XIVE specific tracking.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_xive_irq_state {
    pub /: *mut *mut bool valid; / Interrupt entry is valid,
    pub /: *mut *mut u32 number; / Guest IRQ number,
    pub /: *mut *mut u32 ipi_number; / XIVE IPI HW number,
    pub /: *mut *mut xive_irq_data ipi_data; / XIVE IPI associated data,
    pub /: *mut *mut u32 pt_number; / XIVE Pass-through number if any,
    pub /: *mut *mut *mut xive_irq_data pt_data; / XIVE Pass-through associated data,
// Targetting as set by guest
    pub /: *mut *mut u8 guest_priority; / Guest set priority,
    pub /: *mut *mut u8 saved_priority; / Saved priority when masking,
// Actual targetting
    pub /: *mut *mut u32 act_server; / Actual server,
    pub /: *mut *mut u8 act_priority; / Actual priority,
// Various state bits
    pub /: *mut *mut bool in_eoi; / Synchronize with H_EOI,
    pub /: *mut *mut bool old_p; / P bit state when masking,
    pub /: *mut *mut bool old_q; / Q bit state when masking,
    pub /: *mut *mut bool lsi; / level-sensitive interrupt,
    pub /: *mut *mut bool asserted; / Only for emulated LSI: current state,
// Saved for migration state
    pub in_queue: bool,
    pub saved_p: bool,
    pub saved_q: bool,
    pub saved_scan_prio: u8,
// Xive native
    pub /: *mut *mut u32 eisn; / Guest Effective IRQ number,
}

// Select the "right" interrupt (IPI vs. passthrough)
// out_hw_irq = state->pt_number;
// out_xd = state->pt_data;
// out_hw_irq = state->ipi_number;
// out_xd = &state->ipi_data;
//
// This corresponds to an "ICS" in XICS terminology, we use it
// as a mean to break up source information into multiple structures.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_xive_src_block {
    pub lock: arch_spinlock_t,
    pub id: u16,
    pub irq_state: [kvmppc_xive_irq_state; KVMPPC_XICS_IRQ_PER_ICS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_xive_ops {
    pub guest_irq): *mut *mut *mut int (reset_mapped)(struct kvm kvm, unsigned long,
}

pub const KVMPPC_XIVE_FLAG_SINGLE_ESCALATION: c_uint = 0x1;
pub const KVMPPC_XIVE_FLAG_SAVE_RESTORE: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_xive {
    pub kvm: *mut kvm,
    pub dev: *mut kvm_device,
    pub dentry: *mut dentry,
// VP block associated with the VM
    pub vp_base: u32,
// Blocks of sources
    pub 1]: *mut *mut kvmppc_xive_src_block src_blocks[KVMPPC_XICS_MAX_ICS_ID +,
    pub max_sbid: u32,
//
// For state save, we lazily scan the queues on the first interrupt
// being migrated. We don't have a clean way to reset that flags
// so we keep track of the number of valid sources and how many of
// them were migrated so we can reset when all of them have been
// processed.
//
    pub src_count: u32,
    pub saved_src_count: u32,
//
// Some irqs are delayed on restore until the source is created,
// keep track here of how many of them
//
    pub delayed_irqs: u32,
// Which queues (priorities) are in use by the guest
    pub qmap: u8,
// Queue orders
    pub q_order: u32,
    pub q_page_order: u32,
// Flags
    pub flags: u8,
// Number of entries in the VP block
    pub nr_servers: u32,
    pub ops: *mut kvmppc_xive_ops,
    pub mapping: *mut address_space,
    pub mapping_lock: mutex,
    pub lock: mutex,
}

pub const KVMPPC_XIVE_Q_COUNT: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvmppc_xive_vcpu {
    pub xive: *mut kvmppc_xive,
    pub vcpu: *mut kvm_vcpu,
    pub valid: bool,
// Server number. This is the HW CPU ID from a guest perspective
    pub server_num: u32,
//
// HW VP corresponding to this VCPU. This is the base of the VP
// block plus the server number.
//
    pub vp_id: u32,
    pub vp_chip_id: u32,
    pub vp_cam: u32,
// IPI used for sending ... IPIs
    pub vp_ipi: u32,
    pub vp_ipi_data: xive_irq_data,
// Local emulation state
    pub /: *mut *mut uint8_t cppr; / guest CPPR,
    pub /: *mut *mut uint8_t hw_cppr;/ Hardware CPPR,
    pub mfrr: u8,
    pub pending: u8,
// Each VP has 8 queues though we only provision some
    pub queues: [xive_q; KVMPPC_XIVE_Q_COUNT],
    pub esc_virq: [u32; KVMPPC_XIVE_Q_COUNT],
    pub esc_virq_names: [*mut c_char; KVMPPC_XIVE_Q_COUNT],
// Stash a delayed irq on restore from migration (see set_icp)
    pub delayed_irq: u32,
// Stats
    pub stat_rm_h_xirr: u64,
    pub stat_rm_h_ipoll: u64,
    pub stat_rm_h_cppr: u64,
    pub stat_rm_h_eoi: u64,
    pub stat_rm_h_ipi: u64,
    pub stat_vm_h_xirr: u64,
    pub stat_vm_h_ipoll: u64,
    pub stat_vm_h_cppr: u64,
    pub stat_vm_h_eoi: u64,
    pub stat_vm_h_ipi: u64,
}

// source = src;
//
// When the XIVE resources are allocated at the HW level, the VP
// structures describing the vCPUs of a guest are distributed among
// the chips to optimize the PowerBUS usage. For best performance, the
// guest vCPUs can be pinned to match the VP structure distribution.
//
// Currently, the VP identifiers are deduced from the vCPU id using
// the kvmppc_pack_vcpu_id() routine which is not incorrect but not
// optimal either. It VSMT is used, the result is not continuous and
// the constraints on HW resources described above can not be met.
//
// Mapping between guest priorities and host priorities
// is as follow.
//
// Guest request for 0...6 are honored. Guest request for anything
// higher results in a priority of 6 being applied.
//
// Similar mapping is done for CPPR values
//
// idx = (*idx + 1) & msk;
//
// Common Xive routines for XICS-over-XIVE and XIVE native
//
extern "C" {
    pub fn kvmppc_xive_disable_vcpu_interrupts(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvmppc_xive_debug_show_queues(m: *mut seq_file, vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvmppc_xive_free_sources(sb: *mut kvmppc_xive_src_block);
}
extern "C" {
    pub fn kvmppc_xive_select_target(kvm: *mut kvm, server: *mut u32, prio: u8) -> c_int;
}
extern "C" {
    pub fn xive_cleanup_single_escalation(vcpu: *mut kvm_vcpu, irq: c_int);
}
extern "C" {
    pub fn kvmppc_xive_compute_vp_id(xive: *mut kvmppc_xive, cpu: u32, vp: *mut u32) -> c_int;
}
extern "C" {
    pub fn kvmppc_xive_set_nr_servers(xive: *mut kvmppc_xive, addr: u64) -> c_int;
}
extern "C" {
    pub fn kvmppc_xive_check_save_restore(vcpu: *mut kvm_vcpu) -> bool;
}

