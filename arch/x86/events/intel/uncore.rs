//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/events/intel/uncore.h
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

pub const UNCORE_PMU_NAME_LEN: c_int = 32;

pub const UNCORE_FIXED_EVENT: c_uint = 0xff;
pub const UNCORE_PMC_IDX_MAX_GENERIC: c_int = 8;
pub const UNCORE_PMC_IDX_MAX_FIXED: c_int = 1;
pub const UNCORE_PMC_IDX_MAX_FREERUNNING: c_int = 1;

pub const UNCORE_EXTRA_PCI_DEV: c_uint = 0xff;
pub const UNCORE_EXTRA_PCI_DEV_MAX: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_extra_dev {
    pub dev: [*mut pci_dev; UNCORE_EXTRA_PCI_DEV_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_discovery_domain {
// MSR address or PCI device used as the discovery base
    pub discovery_base: u32,
    pub base_is_pci: bool,
    pub ctl): *mut *mut int (global_init)(int die, u64,
// The units in the discovery table should be ignored.
    pub units_ignore: *mut c_int,
}

pub const UNCORE_DISCOVERY_DOMAINS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_plat_init {
    pub (*cpu_init)(void): *mut c_void,
    pub (*pci_init)(void): *mut c_int,
    pub (*mmio_init)(void): *mut c_void,
    pub domain: [uncore_discovery_domain; UNCORE_DISCOVERY_DOMAINS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_type {
    pub name: *const c_char,
    pub num_counters: c_int,
    pub num_boxes: c_int,
    pub perf_ctr_bits: c_int,
    pub fixed_ctr_bits: c_int,
    pub num_freerunning_types: c_int,
    pub type_id: c_int,
    pub perf_ctr: unsigned,
    pub event_ctl: unsigned,
    pub event_mask: unsigned,
    pub event_mask_ext: unsigned,
    pub fixed_ctr: unsigned,
    pub fixed_ctl: unsigned,
    pub box_ctl: unsigned,
    pub msr_offset: unsigned,
    pub mmio_offset: unsigned,
}

//
// Uncore PMU would store relevant platform topology configuration here
// to identify which platform component each PMON block of that type is
// supposed to monitor.
//
// Optional callbacks for managing mapping of Uncore units to PMONs
//
// Optional callbacks for extra uncore units cleanup
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_ops {
    pub ): *mut *mut int (init_box)(struct intel_uncore_box,
    pub ): *mut *mut void (exit_box)(struct intel_uncore_box,
    pub ): *mut *mut void (disable_box)(struct intel_uncore_box,
    pub ): *mut *mut void (enable_box)(struct intel_uncore_box,
    pub ): *mut *mut *mut void (disable_event)(struct intel_uncore_box , struct perf_event,
    pub ): *mut *mut *mut void (enable_event)(struct intel_uncore_box , struct perf_event,
    pub ): *mut *mut *mut u64 (read_counter)(struct intel_uncore_box , struct perf_event,
    pub ): *mut *mut *mut int (hw_config)(struct intel_uncore_box , struct perf_event,
    pub ): *mut perf_event,
    pub ): *mut *mut *mut void (put_constraint)(struct intel_uncore_box , struct perf_event,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_pmu {
    pub pmu: pmu,
    pub name: [c_char; UNCORE_PMU_NAME_LEN],
    pub pmu_idx: c_int,
    pub flags: c_ulong,
    pub activeboxes: core::sync::atomic::AtomicI32,
    pub cpu_mask: cpumask_t,
    pub type: *mut intel_uncore_type,
    pub boxes: *mut intel_uncore_box,
}

pub const PMU_REGISTERED_BIT: c_int = 0;
pub const PMU_BROKEN_BIT: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_extra_reg {
    pub lock: raw_spinlock_t,
    pub config2: u64 config, config1,,
    pub ref: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_box {
    pub /: *mut *mut int dieid; / Logical die ID,
    pub /: *mut *mut int n_active; / number of active events,
    pub n_events: c_int,
    pub /: *mut *mut int cpu; / cpu to collect events,
    pub flags: c_ulong,
    pub refcnt: core::sync::atomic::AtomicI32,
    pub events: [*mut perf_event; UNCORE_PMC_IDX_MAX],
    pub event_list: [*mut perf_event; UNCORE_PMC_IDX_MAX],
    pub event_constraint: [*mut event_constraint; UNCORE_PMC_IDX_MAX],
    pub active_mask: [c_ulong; BITS_TO_LONGS(UNCORE_PMC_IDX_MAX)],
    pub tags: [u64; UNCORE_PMC_IDX_MAX],
    pub pci_dev: *mut pci_dev,
    pub pmu: *mut intel_uncore_pmu,
    pub /: *mut *mut u64 hrtimer_duration; / hrtimer timeout for this box,
    pub hrtimer: hrtimer,
    pub list: list_head,
    pub active_list: list_head,
    pub io_addr: *mut void __iomem,
    pub shared_regs: [intel_uncore_extra_reg; ],
}

// CFL uncore 8th cbox MSRs
pub const CFL_UNC_CBO_7_PERFEVTSEL0: c_uint = 0xf70;
pub const CFL_UNC_CBO_7_PER_CTR0: c_uint = 0xf76;
pub const UNCORE_BOX_FLAG_INITIALIZED: c_int = 0;
// event config registers are 8-byte apart
pub const UNCORE_BOX_FLAG_CTL_OFFS8: c_int = 1;
// CFL 8th CBOX has different MSR space
pub const UNCORE_BOX_FLAG_CFL8_CBOX_MSR_OFFS: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_event_desc {
    pub attr: device_attribute,
    pub config: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct freerunning_counters {
    pub counter_base: c_uint,
    pub counter_offset: c_uint,
    pub box_offset: c_uint,
    pub num_counters: c_uint,
    pub bits: c_uint,
    pub box_offsets: *mut unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_iio_topology {
    pub pci_bus_no: c_int,
    pub segment: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uncore_upi_topology {
    pub die_to: c_int,
    pub pmu_idx_to: c_int,
    pub enabled: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_uncore_topology {
    pub pmu_idx: c_int,
    pub untyped: *mut c_void,
    pub iio: *mut uncore_iio_topology,
    pub upi: *mut uncore_upi_topology,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci2phy_map {
    pub list: list_head,
    pub segment: c_int,
    pub pbus_to_dieid: [c_int; 256],
}

extern "C" {
    pub fn uncore_pcibus_to_dieid(bus: *mut pci_bus) -> c_int;
}
extern "C" {
    pub fn uncore_die_to_segment(die: c_int) -> c_int;
}
extern "C" {
    pub fn uncore_device_to_die(dev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn uncore_die_to_cpu(die: c_int) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: dev_get_drvdata(dev), intel_uncore_pmu: struct, _arg: pmu) -> return;
}

//
// In the uncore document, there is no event-code assigned to free running
// counters. Some events need to be defined to indicate the free running
// counters. The events are encoded as event-code + umask-code.
//
// The event-code for all free running counters is 0xff, which is the same as
// the fixed counters.
//
// The umask-code is used to distinguish a fixed counter and a free running
// counter, and different types of free running counters.
// - For fixed counters, the umask-code is 0x0X.
// X indicates the index of the fixed counter, which starts from 0.
// - For free running counters, the umask-code uses the rest of the space.
// It would bare the format of 0xXY.
// X stands for the type of free running counters, which starts from 1.
// Y stands for the index of free running counters of same type, which
// starts from 0.
//
// For example, there are three types of IIO free running counters on Skylake
// server, IO CLOCKS counters, BANDWIDTH counters and UTILIZATION counters.
// The event-code for all the free running counters is 0xff.
// 'ioclk' is the first counter of IO CLOCKS. IO CLOCKS is the first type,
// which umask-code starts from 0x10.
// So 'ioclk' is encoded as event=0xff,umask=0x10
// 'bw_in_port2' is the third counter of BANDWIDTH counters. BANDWIDTH is
// the second type, which umask-code starts from 0x20.
// So 'bw_in_port2' is encoded as event=0xff,umask=0x22
//
pub const UNCORE_FREERUNNING_UMASK_START: c_uint = 0x10;
extern "C" {
    pub fn uncore_pci_fixed_ctl(_arg: box) -> return;
}
extern "C" {
    pub fn uncore_msr_fixed_ctl(_arg: box) -> return;
}
extern "C" {
    pub fn uncore_pci_fixed_ctr(_arg: box) -> return;
}
extern "C" {
    pub fn uncore_msr_fixed_ctr(_arg: box) -> return;
}
extern "C" {
    pub fn uncore_pci_event_ctl(_arg: box, _arg: idx) -> return;
}
extern "C" {
    pub fn uncore_msr_event_ctl(_arg: box, _arg: idx) -> return;
}
extern "C" {
    pub fn uncore_pci_perf_ctr(_arg: box, _arg: idx) -> return;
}
extern "C" {
    pub fn uncore_msr_perf_ctr(_arg: box, _arg: idx) -> return;
}
// Check and reject invalid config
extern "C" {
    pub fn container_of(_arg: event->pmu, intel_uncore_pmu: struct, _arg: pmu) -> return;
}
extern "C" {
    pub fn uncore_msr_read_counter(box: *mut intel_uncore_box, event: *mut perf_event) -> u64;
}
extern "C" {
    pub fn uncore_mmio_exit_box(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn uncore_pmu_start_hrtimer(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn uncore_pmu_cancel_hrtimer(box: *mut intel_uncore_box);
}
extern "C" {
    pub fn uncore_pmu_event_start(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn uncore_pmu_event_stop(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn uncore_pmu_event_add(event: *mut perf_event, flags: c_int) -> c_int;
}
extern "C" {
    pub fn uncore_pmu_event_del(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn uncore_pmu_event_read(event: *mut perf_event);
}
extern "C" {
    pub fn uncore_perf_event_update(box: *mut intel_uncore_box, event: *mut perf_event);
}
extern "C" {
    pub fn uncore_put_constraint(box: *mut intel_uncore_box, event: *mut perf_event);
}
extern "C" {
    pub fn uncore_shared_reg_config(box: *mut intel_uncore_box, idx: c_int) -> u64;
}
extern "C" {
    pub fn uncore_get_alias_name(pmu_name: *mut c_char, pmu: *mut intel_uncore_pmu);
}
// uncore_snb.c
extern "C" {
    pub fn snb_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn ivb_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn hsw_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn bdw_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn skl_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn snb_uncore_cpu_init();
}
extern "C" {
    pub fn nhm_uncore_cpu_init();
}
extern "C" {
    pub fn skl_uncore_cpu_init();
}
extern "C" {
    pub fn icl_uncore_cpu_init();
}
extern "C" {
    pub fn tgl_uncore_cpu_init();
}
extern "C" {
    pub fn adl_uncore_cpu_init();
}
extern "C" {
    pub fn lnl_uncore_cpu_init();
}
extern "C" {
    pub fn mtl_uncore_cpu_init();
}
extern "C" {
    pub fn ptl_uncore_cpu_init();
}
extern "C" {
    pub fn nvl_uncore_cpu_init();
}
extern "C" {
    pub fn tgl_uncore_mmio_init();
}
extern "C" {
    pub fn tgl_l_uncore_mmio_init();
}
extern "C" {
    pub fn adl_uncore_mmio_init();
}
extern "C" {
    pub fn lnl_uncore_mmio_init();
}
extern "C" {
    pub fn ptl_uncore_mmio_init();
}
extern "C" {
    pub fn snb_pci2phy_map_init(devid: c_int) -> c_int;
}
// uncore_snbep.c
extern "C" {
    pub fn snbep_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn snbep_uncore_cpu_init();
}
extern "C" {
    pub fn ivbep_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn ivbep_uncore_cpu_init();
}
extern "C" {
    pub fn hswep_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn hswep_uncore_cpu_init();
}
extern "C" {
    pub fn bdx_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn bdx_uncore_cpu_init();
}
extern "C" {
    pub fn knl_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn knl_uncore_cpu_init();
}
extern "C" {
    pub fn skx_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn skx_uncore_cpu_init();
}
extern "C" {
    pub fn snr_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn snr_uncore_cpu_init();
}
extern "C" {
    pub fn snr_uncore_mmio_init();
}
extern "C" {
    pub fn icx_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn icx_uncore_cpu_init();
}
extern "C" {
    pub fn icx_uncore_mmio_init();
}
extern "C" {
    pub fn spr_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn spr_uncore_cpu_init();
}
extern "C" {
    pub fn spr_uncore_mmio_init();
}
extern "C" {
    pub fn gnr_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn gnr_uncore_cpu_init();
}
extern "C" {
    pub fn gnr_uncore_mmio_init();
}
extern "C" {
    pub fn dmr_uncore_pci_init() -> c_int;
}
extern "C" {
    pub fn dmr_uncore_mmio_init();
}
// uncore_nhmex.c
extern "C" {
    pub fn nhmex_uncore_cpu_init();
}
