//! Automatically rewritten from C Header to Rust Module
//! Source: include/kvm/arm_vgic.h
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
// Copyright (C) 2015, 2016 ARM Ltd.
//

pub const VGIC_V5_MAX_CPUS: c_int = 512;
pub const VGIC_V3_MAX_CPUS: c_int = 512;
pub const VGIC_V2_MAX_CPUS: c_int = 8;
pub const VGIC_NR_IRQS_LEGACY: c_int = 256;
pub const VGIC_NR_SGIS: c_int = 16;
pub const VGIC_NR_PPIS: c_int = 16;

pub const VGIC_MAX_SPI: c_int = 1019;
pub const VGIC_MAX_RESERVED: c_int = 1023;
pub const VGIC_MIN_LPI: c_int = 8192;

//
// GICv5 supports 128 PPIs, but only the first 64 are architected. We only
// support the timers and PMU in KVM, both of which are architected. Rather than
// handling twice the state, we instead opt to only support the architected set
// in KVM for now. At a future stage, this can be bumped up to 128, if required.
//
pub const VGIC_V5_NR_PRIVATE_IRQS: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vgic_type {
    VGIC_V2,		/* Good ol' GICv2 */
    VGIC_V3,		/* New fancy GICv3 */
    VGIC_V5,		/* Newer, fancier GICv5 */
}

// same for all guests, as depending only on the _host's_ GIC model
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_global {
// type of the host GIC
    pub type: vgic_type,
// Physical address of vgic virtual cpu interface
    pub vcpu_base: phys_addr_t,
// GICV mapping, kernel VA
    pub vcpu_base_va: *mut void __iomem,
// GICV mapping, HYP VA
    pub vcpu_hyp_va: *mut void __iomem,
// virtual control interface mapping, kernel VA
    pub vctrl_base: *mut void __iomem,
// virtual control interface mapping, HYP VA
    pub vctrl_hyp: *mut void __iomem,
// Physical CPU interface, kernel VA
    pub gicc_base: *mut void __iomem,
// Number of implemented list registers
    pub nr_lr: c_int,
// Maintenance IRQ number
    pub maint_irq: c_uint,
// maximum number of VCPUs allowed (GICv2 limits us to 8)
    pub max_gic_vcpus: c_int,
// Only needed for the legacy KVM_CREATE_IRQCHIP
    pub can_emulate_gicv2: bool,
// Hardware has GICv4?
    pub has_gicv4: bool,
    pub has_gicv4_1: bool,
// Pseudo GICv3 from outer space
    pub no_hw_deactivation: bool,
// GICv3 system register CPU interface
    pub gicv3_cpuif: static_key_false,
// GICv3 compat mode on a GICv5 host
    pub has_gcie_v3_compat: bool,
// GICv5 PPI capabilities
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(impl_ppi_mask,,
    pub vgic_v5_ppi_caps: },
}

pub const VGIC_V3_MAX_LRS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vgic_irq_config {
    VGIC_CONFIG_EDGE = 0,
    VGIC_CONFIG_LEVEL
}

//
// Per-irq ops overriding some common behavious.
//
// Always called in non-preemptible section and the functions can use
// kvm_arm_get_running_vcpu() to get the vcpu pointer for private IRQs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_ops {
// Per interrupt flags for special-cased interrupts
    pub (*get_flags)(void): *mut c_ulong,

//
// Callback function pointer to in-kernel devices that can tell us the
// state of the input level of mapped level-triggered IRQ faster than
// peaking into the physical GIC.
//
    pub vintid): *mut *mut bool (get_input_level)(int,
//
// Function pointer to override the queuing of an IRQ.
//
    pub __releases(&irq->irq_lock): unsigned long flags),
//
// Callback function pointer to either enable or disable direct
// injection for a mapped interrupt.
//
    pub direct): *mut *mut vgic_irq irq, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_irq {
    pub /: *mut *mut raw_spinlock_t irq_lock; / Protects the content of the struct,
    pub /: *mut *mut u32 intid; / Guest visible INTID,
    pub rcu: rcu_head,
    pub ap_list: list_head,
    pub VCPU: *mut *mut *mut kvm_vcpu vcpu; / SGIs and PPIs: The,
// SPIs and LPIs: The VCPU whose ap_list
// this is queued on.
//
    pub should: *mut *mut *mut kvm_vcpu target_vcpu; / The VCPU that this interrupt,
// be sent to, as a result of the
// targets reg (v2) or the
// affinity reg (v3).
//
    pub calculate: *mut *mut bool pending_latch:1; / The pending latch state used to,
// the pending state for both level
// and edge triggered IRQs.
    pub /: *mut *mut vgic_irq_config config:1; / Level or edge,
    pub /: *mut *mut bool line_level:1; / Level only,
    pub enabled:1: bool,
    pub active:1: bool,
    pub /: *mut *mut bool hw:1; / Tied to HW IRQ,
    pub /: *mut *mut bool on_lr:1; / Present in a CPU LR,
    pub /: *mut *mut refcount_t refcount; / Used for LPIs,
    pub /: *mut *mut u32 hwintid; / HW INTID number,
    pub /: *mut *mut unsigned int host_irq; / linux irq corresponding to hwintid,
    pub /: *mut *mut u8 targets; / GICv2 target VCPUs mask,
    pub /: *mut *mut u32 mpidr; / GICv3 target VCPU,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iodev_type {
    IODEV_CPUIF,
    IODEV_DIST,
    IODEV_REDIST,
    IODEV_ITS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_io_device {
    pub base_addr: gpa_t,
    pub redist_vcpu: *mut kvm_vcpu,
    pub its: *mut vgic_its,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_its {
// The base address of the ITS control register frame
    pub vgic_its_base: gpa_t,
    pub enabled: bool,
    pub iodev: vgic_io_device,
    pub dev: *mut kvm_device,
// These registers correspond to GITS_BASER{0,1}
    pub baser_device_table: u64,
    pub baser_coll_table: u64,
// Protects the command queue
    pub cmd_lock: mutex,
    pub cbaser: u64,
    pub creadr: u32,
    pub cwriter: u32,
// migration ABI revision in use
    pub abi_rev: u32,
// Protects the device and collection lists
    pub its_lock: mutex,
    pub device_list: list_head,
    pub collection_list: list_head,
//
// Caches the (device_id, event_id) -> vgic_irq translation for
// LPIs that are mapped and enabled.
//
    pub translation_cache: xarray,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_redist_region {
    pub index: u32,
    pub base: gpa_t,
    pub /: *mut *mut u32 count; / number of redistributors or 0 if single region,
    pub /: *mut *mut u32 free_index; / index of the next free redistributor,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_v5_vm {
//
// We only expose a subset of PPIs to the guest. This subset is a
// combination of the PPIs that are actually implemented and what we
// actually choose to expose.
//
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(vgic_ppi_mask,,
// A mask of the PPIs that are exposed for userspace to drive.
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(userspace_ppis,,
//
// The HMR itself is handled by the hardware, but we still need to have
// a mask that we can use when merging in pending state (only the state
// of Edge PPIs is merged back in from the guest an the HMR provides a
// convenient way to do that).
//
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(vgic_ppi_hmr,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_dist {
    pub in_kernel: bool,
    pub ready: bool,
    pub initialized: bool,
// vGIC model the kernel emulates for the guest (GICv2 or GICv3)
    pub vgic_model: u32,
// Implementation revision as reported in the GICD_IIDR
    pub implementation_rev: u32,

// Userspace can write to GICv2 IGROUPR
    pub v2_groups_user_writable: bool,
// Do injected MSIs require an additional device ID?
    pub msis_require_devid: bool,
    pub nr_spis: c_int,
// The GIC maintenance IRQ for nested hypervisors.
    pub mi_intid: u32,
// Track the number of in-flight active SPIs
    pub active_spis: core::sync::atomic::AtomicI32,
// base addresses in guest physical address space:
    pub /: *mut *mut gpa_t vgic_dist_base; / distributor,
// either a GICv2 CPU interface
    pub vgic_cpu_base: gpa_t,
// or a number of GICv3 redistributor regions
    pub rd_regions: list_head,
}

// distributor enabled
// Supports SGIs without active state
// Wants SGIs without active state
//
// Contains the attributes and gpa of the LPI configuration table.
// Since we report GICR_TYPER.CommonLPIAff as 0b00, we can share
// one address across all redistributors.
// GICv3 spec: IHI 0069E 6.1.1 "LPI Configuration tables"
//
// GICv4 ITS per-VM data, containing the IRQ domain, the VPE
// array, the property table pointer as well as allocation
// data. This essentially ties the Linux IRQ core and ITS
// together, and avoids leaking KVM's data structures anywhere
// else.
//
// GICv5 per-VM data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_v2_cpu_if {
    pub vgic_hcr: u32,
    pub vgic_vmcr: u32,
    pub vgic_apr: u32,
    pub vgic_lr: [u32; VGIC_V2_MAX_LRS],
    pub used_lrs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_v3_cpu_if {
    pub vgic_hcr: u32,
    pub vgic_vmcr: u32,
    pub /: *mut *mut u32 vgic_sre; / Restored only, change ignored,
    pub vgic_ap0r: [u32; 4],
    pub vgic_ap1r: [u32; 4],
    pub vgic_lr: [u64; VGIC_V3_MAX_LRS],
//
// GICv4 ITS per-VPE data, containing the doorbell IRQ, the
// pending table pointer, the its_vm pointer and a few other
// HW specific things. As for the its_vm structure, this is
// linking the Linux IRQ subsystem and the ITS together.
//
    pub its_vpe: its_vpe,
    pub used_lrs: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_v5_cpu_if {
    pub vgic_apr: u64,
    pub vgic_vmcr: u64,
// PPI register state
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(vgic_ppi_dvir,,
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(vgic_ppi_activer,,
    pub VGIC_V5_NR_PRIVATE_IRQS): DECLARE_BITMAP(vgic_ppi_enabler,,
// We have one byte (of which 5 bits are used) per PPI for priority
    pub 8]: u64 vgic_ppi_priorityr[VGIC_V5_NR_PRIVATE_IRQS /,
//
// The ICSR is re-used across host and guest, and hence it needs to be
// saved/restored. Only one copy is required as the host should block
// preemption between executing GIC CDRCFG and acccessing the
// ICC_ICSR_EL1. A guest, of course, can never guarantee this, and hence
// it is the hyp's responsibility to keep the state constistent.
//
    pub vgic_icsr: u64,
    pub gicv5_vpe: gicv5_vpe,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct vgic_cpu {
// CPU vif control registers for world switch
    pub vgic_v2: vgic_v2_cpu_if,
    pub vgic_v3: vgic_v3_cpu_if,
    pub vgic_v5: vgic_v5_cpu_if,
}

//
// List of IRQs that this VCPU should consider because they are either
// Active or Pending (hence the name; AP list), or because they recently
// were one of the two and need to be migrated off this list to another
// VCPU.
//
// Members below are used with GICv3 emulation only and represent
// parts of the redistributor.
//
// Contains the attributes and gpa of the LPI pending tables.
// GICR_CTLR.{ENABLE_LPIS,RWP}
// Cache guest priority bits
// Cache guest interrupt ID bits
extern "C" {
    pub fn kvm_set_legacy_vgic_v2_addr(kvm: *mut kvm, dev_addr: *mut kvm_arm_device_addr) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_early_init(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_vgic_vcpu_init(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_vcpu_nv_init(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_create(kvm: *mut kvm, type: u32) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_vgic_vcpu_destroy(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vgic_map_resources(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_finalize_idregs(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_vgic_hyp_init() -> c_int;
}
extern "C" {
    pub fn kvm_vgic_init_cpu_hardware();
}
extern "C" {
    pub fn kvm_vgic_clear_irq_ops(vcpu: *mut kvm_vcpu, vintid: u32);
}
extern "C" {
    pub fn kvm_vgic_unmap_phys_irq(vcpu: *mut kvm_vcpu, vintid: c_uint) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_get_map(vcpu: *mut kvm_vcpu, vintid: c_uint) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_map_is_active(vcpu: *mut kvm_vcpu, vintid: c_uint) -> bool;
}
extern "C" {
    pub fn kvm_vgic_vcpu_pending_irq(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_load(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vgic_put(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_get_eisr(vcpu: *mut kvm_vcpu) -> u16;
}
extern "C" {
    pub fn vgic_v3_get_elrsr(vcpu: *mut kvm_vcpu) -> u16;
}
extern "C" {
    pub fn vgic_v3_get_misr(vcpu: *mut kvm_vcpu) -> u64;
}

extern "C" {
    pub fn kvm_vcpu_has_pending_irqs(vcpu: *mut kvm_vcpu) -> bool;
}
extern "C" {
    pub fn kvm_vgic_sync_hwstate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vgic_flush_hwstate(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_vgic_reset_mapped_irq(vcpu: *mut kvm_vcpu, vintid: u32);
}
extern "C" {
    pub fn kvm_vgic_process_async_update(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v3_dispatch_sgi(vcpu: *mut kvm_vcpu, reg: u64, allow_group1: bool);
}
//
// kvm_vgic_get_max_vcpus - Get the maximum number of VCPUs allowed by HW
//
// The host's GIC naturally limits the maximum amount of VCPUs a guest
// can use.
//
// kvm_vgic_setup_default_irq_routing:
// Setup a default flat gsi routing table mapping all SPIs
//
extern "C" {
    pub fn kvm_vgic_setup_default_irq_routing(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_set_owner(vcpu: *mut kvm_vcpu, intid: c_uint, owner: *mut c_void) -> c_int;
}
extern "C" {
    pub fn kvm_vgic_v4_unset_forwarding(kvm: *mut kvm, host_irq: c_int);
}
extern "C" {
    pub fn vgic_v4_load(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vgic_v4_commit(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn vgic_v4_put(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn vgic_v5_finalize_ppi_state(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn vgic_v5_set_ppi_dvi(vcpu: *mut kvm_vcpu, irq: *mut vgic_irq, dvi: bool);
}
extern "C" {
    pub fn vgic_state_is_nested(vcpu: *mut kvm_vcpu) -> bool;
}
// CPU HP callbacks
extern "C" {
    pub fn kvm_vgic_cpu_up();
}
extern "C" {
    pub fn kvm_vgic_cpu_down();
}
