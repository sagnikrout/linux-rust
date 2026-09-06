//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/events/perf_event.h
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


//
// Performance events x86 architecture header
//
// Copyright (C) 2008 Linutronix GmbH, Thomas Gleixner <tglx@kernel.org>
// Copyright (C) 2008-2009 Red Hat, Inc., Ingo Molnar
// Copyright (C) 2009 Jaswinder Singh Rajput
// Copyright (C) 2009 Advanced Micro Devices, Inc., Robert Richter
// Copyright (C) 2008-2009 Red Hat, Inc., Peter Zijlstra
// Copyright (C) 2009 Intel Corporation, <markus.t.metzger@intel.com>
// Copyright (C) 2009 Google, Inc., Stephane Eranian
//
// For licencing details see kernel-base/COPYING
//

// To enable MSR tracing please use the generic trace points.
//
// |   NHM/WSM    |      SNB     |
// register -------------------------------
// |  HT  | no HT |  HT  | no HT |
// -----------------------------------------
// offcore  | core | core  | cpu  | core  |
// lbr_sel  | core | core  | cpu  | core  |
// ld_lat   | cpu  | core  | cpu  | core  |
// -----------------------------------------
//
// Given that there is a small number of shared regs,
// we can pre-allocate their slot in the per-cpu
// per-core reg tables.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum extra_reg_type {
    EXTRA_REG_NONE		= -1, /* not used */

    EXTRA_REG_RSP_0		= 0,  /* offcore_response_0 */
    EXTRA_REG_RSP_1		= 1,  /* offcore_response_1 */
    EXTRA_REG_LBR		= 2,  /* lbr_select */
    EXTRA_REG_LDLAT		= 3,  /* ld_lat_threshold */
    EXTRA_REG_FE		= 4,  /* fe_* */
    EXTRA_REG_SNOOP_0	= 5,  /* snoop response 0 */
    EXTRA_REG_SNOOP_1	= 6,  /* snoop response 1 */
    EXTRA_REG_OMR_0		= 7,  /* OMR 0 */
    EXTRA_REG_OMR_1		= 8,  /* OMR 1 */
    EXTRA_REG_OMR_2		= 9,  /* OMR 2 */
    EXTRA_REG_OMR_3		= 10,  /* OMR 3 */

    EXTRA_REG_MAX		      /* number of entries needed */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_constraint {
    pub idxmsk: [c_ulong; BITS_TO_LONGS(X86_PMC_IDX_MAX)],
    pub idxmsk64: u64,
}

//
// struct hw_perf_event.flags flags
//

extern "C" {
    pub fn is_metric_event(is_slots_event(event: event) ||) -> return;
}
extern "C" {
    pub fn is_x86_event(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn check_leader_group(_arg: event->group_leader, _arg: PERF_X86_EVENT_BRANCH_COUNTERS) -> return;
}
extern "C" {
    pub fn check_leader_group(_arg: event->group_leader, _arg: PERF_X86_EVENT_PEBS_CNTR) -> return;
}
extern "C" {
    pub fn check_leader_group(_arg: event->group_leader, _arg: PERF_X86_EVENT_ACR) -> return;
}
extern "C" {
    pub fn test_bit(_arg: hwc->idx, )&hwc->config1: *mut (unsigned long) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_nb {
    pub /: *mut *mut int nb_id; / NorthBridge id,
    pub /: *mut *mut int refcnt; / reference count,
    pub owners: [*mut perf_event; X86_PMC_IDX_MAX],
    pub event_constraints: [event_constraint; X86_PMC_IDX_MAX],
}

pub const PEBS_OUTPUT_OFFSET: c_int = 61;

//
// Flags PEBS can handle without an PMI.
//
// TID can only be handled by flushing at context switch.
// REGS_USER can be handled for events limited to ring 3.
//

// user space rdpmc control values
//
// Per register state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct er_account {
    pub /: *mut *mut raw_spinlock_t lock; / per-core: protect structure,
    pub /: *mut *mut u64 config; / extra MSR config,
    pub /: *mut *mut u64 reg; / extra MSR number,
    pub /: *mut *mut atomic_t ref; / reference count,
}

//
// Per core/cpu state
//
// Used to coordinate shared registers between HT threads or
// among events on a single PMU.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_shared_regs {
    pub regs: [er_account; EXTRA_REG_MAX],
    pub /: *mut *mut int refcnt; / per-core: #HT threads,
    pub /: *mut *mut unsigned core_id; / per-core: core id,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum intel_excl_state_type {
    INTEL_EXCL_UNUSED    = 0, /* counter is unused */
    INTEL_EXCL_SHARED    = 1, /* counter can be used by both threads */
    INTEL_EXCL_EXCLUSIVE = 2, /* counter can be used by one thread only */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_excl_states {
    pub state: [intel_excl_state_type; X86_PMC_IDX_MAX],
    pub /: *mut *mut bool sched_started; / true if scheduling has started,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intel_excl_cntrs {
    pub lock: raw_spinlock_t,
    pub states: [intel_excl_states; 2],
    pub has_exclusive: [u16; 2],
    pub exclusive_present: u32,
}

pub const MAX_LBR_ENTRIES: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_hw_events {
//
// Generic x86 PMC bits
//
    pub /: *mut *mut *mut perf_event events[X86_PMC_IDX_MAX]; / in counter order,
    pub active_mask: [c_ulong; BITS_TO_LONGS(X86_PMC_IDX_MAX)],
    pub dirty: [c_ulong; BITS_TO_LONGS(X86_PMC_IDX_MAX)],
    pub enabled: c_int,
    pub /: *mut *mut int n_events; / the # of events in the below arrays,
    pub arrays: *mut *mut int n_added; / the # last events in the below,
    pub arrays: *mut *mut int n_txn; / the # last events in the below,
    pub n_txn_pair: c_int,
    pub n_txn_metric: c_int,
    pub /: *mut *mut int assign[X86_PMC_IDX_MAX]; / event to counter assignment,
    pub tags: [u64; X86_PMC_IDX_MAX],
    pub /: *mut *mut *mut perf_event event_list[X86_PMC_IDX_MAX]; / in enabled order,
    pub event_constraint: [*mut event_constraint; X86_PMC_IDX_MAX],
    pub /: *mut *mut int n_excl; / the number of exclusive events,
    pub /: *mut *mut int n_late_setup; / the num of events needs late setup,
    pub txn_flags: c_uint,
    pub is_fake: c_int,
//
// Intel DebugStore bits
//
    pub ds: *mut debug_store,
    pub ds_bts_vaddr: *mut c_void,
// DS based PEBS or arch-PEBS buffer address
    pub pebs_vaddr: *mut c_void,
    pub pebs_enabled: u64,
    pub n_pebs: c_int,
    pub n_large_pebs: c_int,
    pub n_pebs_via_pt: c_int,
    pub pebs_output: c_int,
// Current super set of events hardware configuration
    pub pebs_data_cfg: u64,
    pub active_pebs_data_cfg: u64,
    pub pebs_record_size: c_int,
// Intel Fixed counter configuration
    pub fixed_ctrl_val: u64,
    pub active_fixed_ctrl_val: u64,
// Intel ACR/arch-PEBS configuration
    pub acr_cfg_b: [u64; X86_PMC_IDX_MAX],
    pub cfg_c_val: [u64; X86_PMC_IDX_MAX],
//
// Intel LBR bits
//
    pub lbr_users: c_int,
    pub lbr_pebs_users: c_int,
    pub lbr_stack: perf_branch_stack,
    pub lbr_entries: [perf_branch_entry; MAX_LBR_ENTRIES],
    pub /: *mut *mut u64 lbr_counters[MAX_LBR_ENTRIES]; / branch stack extra,
    pub lbr_sel: *mut er_account,
    pub lbr_ctl: *mut er_account,
}

//
// Intel host/guest exclude bits
//
// Intel checkpoint mask
//
// manage shared (per-core, per-cpu) registers
// used on Intel NHM/WSM/SNB
//
// manage exclusive counter access between hyperthread
//
// SKL TSX_FORCE_ABORT shadow
//
// Perf Metrics
//
// number of accepted metrics events
//
// AMD specific bits
//
// Inverted mask of bits to clear in the perf_ctr ctrl registers

//
// The constraint_match() function only works for 'simple' event codes
// and not for extended (AMD64_EVENTSEL_EVENT) events codes.
//

//
// The overlap flag marks event constraints with overlapping counter
// masks. This is the case if the counter mask of such an event is not
// a subset of any other counter mask of a constraint with an equal or
// higher weight, e.g.:
//
// c_overlaps = EVENT_CONSTRAINT_OVERLAP(0, 0x09, 0);
// c_another1 = EVENT_CONSTRAINT(0, 0x07, 0);
// c_another2 = EVENT_CONSTRAINT(0, 0x38, 0);
//
// The event scheduler may not select the correct counter in the first
// cycle because it needs to know which subsequent events will be
// scheduled. It may fail to schedule the events then. So we set the
// overlap flag for such constraints to give the scheduler a hint which
// events to select for counter rescheduling.
//
// Care must be taken as the rescheduling algorithm is O(n!) which
// will increase scheduling cycles for an over-committed system
// dramatically.  The number of such EVENT_CONSTRAINT_OVERLAP() macros
// and its counter masks must be kept at a minimum.
//

//
// Constraint on the Event code.
//

//
// Constraint on a range of Event codes
//

//
// Constraint on the Event code + UMask + fixed-mask
//
// filter mask to validate fixed counter events.
// the following filters disqualify for fixed counters:
// - inv
// - edge
// - cnt-mask
// - in_tx
// - in_tx_checkpointed
// The other filters are supported by fixed counters.
// The any-thread option is supported starting with v3.
//

//
// The special metric counters do not actually exist. They are calculated from
// the combination of the FxCtr3 + MSR_PERF_METRICS.
//
// The special metric counters are mapped to a dummy offset for the scheduler.
// The sharing between multiple users of the same metric without multiplexing
// is not allowed, even though the hardware supports that in principle.
//

//
// Constraint on the Event code + UMask
//

// Constraint on specific umask bit only + event

// Like UEVENT_CONSTRAINT, but match flags too

// Event constraint, but match on all event flags too.

// Check only flags, but allow all event/umask

// Check flags and event code, and set the HSW store flag

// Check flags and event code, and set the HSW load flag

// Check flags and event code/umask, and set the HSW store flag

// Check flags and event code/umask, and set the HSW load flag

// Check flags and event code/umask, and set the HSW N/A flag

//
// We define the end marker as having a weight of -1
// to enable blacklisting of events using a counter bitmask
// of zero and thus a weight of zero.
// The end marker has a weight that cannot possibly be
// obtained from counting the bits in the bitmask.
//

//
// Check for end marker with weight == -1
//

//
// Extra registers for specific events.
//
// Some events need large masks and require external MSRs.
// Those extra MSRs end up being shared for all events on
// a PMU and sometimes between PMU of sibling HT threads.
// In either case, the kernel needs to handle conflicting
// accesses to those extra, shared, regs. The data structure
// to manage those registers is stored in cpu_hw_event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extra_reg {
    pub event: c_uint,
    pub msr: c_uint,
    pub config_mask: u64,
    pub valid_mask: u64,
    pub /: *mut *mut int idx; / per_xxx->regs[] reg index,
    pub extra_msr_access: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_capabilities {
    pub lbr_format:6: u64,
    pub pebs_trap:1: u64,
    pub pebs_arch_reg:1: u64,
    pub pebs_format:4: u64,
    pub smm_freeze:1: u64,
//
// PMU supports separate counter range for writing
// values > 32bit.
//
    pub full_width_write:1: u64,
    pub pebs_baseline:1: u64,
    pub perf_metrics:1: u64,
    pub pebs_output_pt_available:1: u64,
    pub pebs_timing_info:1: u64,
    pub __reserved:1: u64,
    pub rdpmc_metrics_clear:1: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_pmu_quirk {
    pub next: *mut x86_pmu_quirk,
    pub (*func)(void): *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union x86_pmu_config {
    pub bits: },
    pub value: u64,
}

pub const PERF_PEBS_DATA_SOURCE_MAX: c_uint = 0x100;

pub const PERF_PEBS_DATA_SOURCE_GRT_MAX: c_uint = 0x10;

pub const X86_HYBRID_PMU_ATOM_IDX: c_int = 0;
pub const X86_HYBRID_PMU_CORE_IDX: c_int = 1;
pub const X86_HYBRID_PMU_TINY_IDX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hybrid_pmu_type {
    not_hybrid,
    hybrid_small		= BIT(X86_HYBRID_PMU_ATOM_IDX),
    hybrid_big		= BIT(X86_HYBRID_PMU_CORE_IDX),
    hybrid_tiny		= BIT(X86_HYBRID_PMU_TINY_IDX),

// The belows are only used for matching
    hybrid_big_small	= hybrid_big   | hybrid_small,
    hybrid_small_tiny	= hybrid_small | hybrid_tiny,
    hybrid_big_small_tiny	= hybrid_big   | hybrid_small_tiny,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arch_pebs_cap {
    pub caps: u64,
    pub counters: u64,
    pub pdists: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_hybrid_pmu {
    pub pmu: pmu,
    pub name: *const c_char,
    pub pmu_type: hybrid_pmu_type,
    pub supported_cpus: cpumask_t,
    pub intel_cap: perf_capabilities,
    pub intel_ctrl: u64,
    pub pebs_events_mask: u64,
    pub config_mask: u64,
    pub cntr_mask64: u64,
    pub cntr_mask: [c_ulong; BITS_TO_LONGS(X86_PMC_IDX_MAX)],
}

extern "C" {
    pub fn container_of(_arg: pmu, x86_hybrid_pmu: struct, _arg: pmu) -> return;
}

//
// struct x86_pmu - generic x86 pmu
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_pmu {
//
// Generic x86 PMC bits
//
    pub name: *const c_char,
    pub version: c_int,
    pub ): *mut *mut int (handle_irq)(struct pt_regs,
    pub (*disable_all)(void): *mut c_void,
    pub added): *mut *mut void (enable_all)(int,
    pub ): *mut *mut void (enable)(struct perf_event,
    pub ): *mut *mut void (disable)(struct perf_event,
    pub idx): *mut *mut *mut void (assign)(struct perf_event event, int,
    pub ): *mut *mut void (add)(struct perf_event,
    pub ): *mut *mut void (del)(struct perf_event,
    pub event): *mut *mut void (read)(struct perf_event,
    pub event): *mut *mut int (set_period)(struct perf_event,
    pub event): *mut *mut u64 (update)(struct perf_event,
    pub event): *mut *mut int (hw_config)(struct perf_event,
    pub assign): *mut *mut *mut int (schedule_events)(struct cpu_hw_events cpuc, int n, int,
    pub (*late_setup)(void): *mut c_void,
    pub event): *mut *mut void (pebs_enable)(struct perf_event,
    pub event): *mut *mut void (pebs_disable)(struct perf_event,
    pub (*pebs_enable_all)(void): *mut c_void,
    pub (*pebs_disable_all)(void): *mut c_void,
    pub eventsel: unsigned,
    pub perfctr: unsigned,
    pub fixedctr: unsigned,
    pub eventsel): *mut *mut int (addr_offset)(int index, bool,
    pub index): *mut *mut int (rdpmc_index)(int,
    pub (*event_map)(int): *mut u64,
    pub max_events: c_int,
    pub config_mask: u64,
    pub cntr_mask64: u64,
    pub cntr_mask: [c_ulong; BITS_TO_LONGS(X86_PMC_IDX_MAX)],
}

// PMI handler bits
//
// sysfs attrs
//
// CPU Hotplug hooks
//
// Intel Arch Perfmon v2+
//
// Intel DebugStore and PEBS bits
//
// Intel Architectural PEBS
//
// Intel LBR
//
// Intel Architectural LBR CPUID Enumeration
//
// Intel PT/LBR/BTS are exclusive
//
// Intel perf metrics
//
// AMD bits
//
// Extra registers for events
//
// Intel host/guest support (KVM)
//
// Check period value for PERF_EVENT_IOC_PERIOD ioctl.
//
// Hybrid support
//
// Most PMU capabilities are the same among different hybrid PMUs.
// The global x86_pmu saves the architecture capabilities, which
// are available for all PMUs. The hybrid_pmu only includes the
// unique capabilities.
//
extern "C" {
    pub fn intel_cpu_type((void: *mut *mut get_hybrid_cpu_type)) -> enum;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_perf_task_context_opt {
    pub lbr_callstack_users: c_int,
    pub lbr_stack_state: c_int,
    pub log_id: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_perf_task_context {
    pub lbr_sel: u64,
    pub tos: c_int,
    pub valid_lbrs: c_int,
    pub opt: x86_perf_task_context_opt,
    pub lbr: [lbr_entry; MAX_LBR_ENTRIES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_perf_task_context_arch_lbr {
    pub opt: x86_perf_task_context_opt,
    pub entries: [lbr_entry; ],
}

//
// Add padding to guarantee the 64-byte alignment of the state buffer.
//
// The structure is dynamically allocated. The size of the LBR state may vary
// based on the number of LBR registers.
//
// Do not put anything after the LBR state.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct x86_perf_task_context_arch_lbr_xsave {
    pub opt: x86_perf_task_context_opt,
    pub xsave: xregs_state,
    pub i387: fxregs_state,
    pub header: xstate_header,
    pub lbr: arch_lbr_state,
// C attribute field omitted
}

//
// x86_pmu flags
//
pub const PMU_FL_NO_HT_SHARING: c_uint = 0x1 /* no hyper-threading resource sharing */;
pub const PMU_FL_HAS_RSP_1: c_uint = 0x2 /* has 2 equivalent offcore_rsp regs   */;
pub const PMU_FL_EXCL_CNTRS: c_uint = 0x4 /* has exclusive counter requirements  */;
pub const PMU_FL_EXCL_ENABLED: c_uint = 0x8 /* exclusive counter active */;
pub const PMU_FL_PEBS_ALL: c_uint = 0x10 /* all events are valid PEBS events */;
pub const PMU_FL_TFA: c_uint = 0x20 /* deal with TSX force abort */;
pub const PMU_FL_PAIR: c_uint = 0x40 /* merge counters for large incr. events */;
pub const PMU_FL_INSTR_LATENCY: c_uint = 0x80 /* Support Instruction Latency in PEBS Memory Info Record */;
pub const PMU_FL_MEM_LOADS_AUX: c_uint = 0x100 /* Require an auxiliary event for the complete memory info */;
pub const PMU_FL_RETIRE_LATENCY: c_uint = 0x200 /* Support Retire Latency in PEBS */;
pub const PMU_FL_BR_CNTR: c_uint = 0x400 /* Support branch counter logging */;
pub const PMU_FL_DYN_CONSTRAINT: c_uint = 0x800 /* Needs dynamic constraint */;
pub const PMU_FL_HAS_OMR: c_uint = 0x1000 /* has 4 equivalent OMR regs */;

extern "C" {
    pub fn x86_perf_event_set_period(event: *mut perf_event) -> c_int;
}
//
// Generalized hw caching related hw_event table, filled
// in on a per model basis. A value of 0 means
// 'not supported', -1 means 'hw_event makes no sense on
// this CPU', any other value means the raw hw_event
// ID.
//

extern "C" {
    pub fn x86_perf_event_update(event: *mut perf_event) -> u64;
}
extern "C" {
    pub fn x86_perf_event_update(_arg: event) -> return;
}
extern "C" {
    pub fn x86_add_exclusive(what: c_uint) -> c_int;
}
extern "C" {
    pub fn x86_del_exclusive(what: c_uint);
}
extern "C" {
    pub fn x86_reserve_hardware() -> c_int;
}
extern "C" {
    pub fn x86_release_hardware();
}
extern "C" {
    pub fn x86_pmu_max_precise(pmu: *mut pmu) -> c_int;
}
extern "C" {
    pub fn hw_perf_lbr_event_destroy(event: *mut perf_event);
}
extern "C" {
    pub fn x86_setup_perfctr(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn x86_pmu_hw_config(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn x86_pmu_disable_all();
}
//
// Add enabled Merge event on next counter
// if large increment event being enabled on this counter
//
extern "C" {
    pub fn x86_pmu_enable_all(added: c_int);
}
extern "C" {
    pub fn x86_schedule_events(cpuc: *mut cpu_hw_events, n: c_int, assign: *mut c_int) -> c_int;
}
extern "C" {
    pub fn x86_pmu_stop(event: *mut perf_event, flags: c_int);
}
extern "C" {
    pub fn x86_pmu_enable_event(event: *mut perf_event);
}
extern "C" {
    pub fn x86_pmu_handle_irq(regs: *mut pt_regs) -> c_int;
}
extern "C" {
    pub fn x86_pmu_show_pmu_cap(pmu: *mut pmu);
}
extern "C" {
    pub fn hweight64(_arg: hybrid(pmu, _arg: cntr_mask64)) -> return;
}
extern "C" {
    pub fn fls64(_arg: hybrid(pmu, _arg: cntr_mask64)) -> return;
}
extern "C" {
    pub fn hweight64(_arg: hybrid(pmu, _arg: fixed_cntr_mask64)) -> return;
}
extern "C" {
    pub fn fls64(_arg: hybrid(pmu, _arg: fixed_cntr_mask64)) -> return;
}

//
// Not all PMUs provide the right context information to place the reported IP
// into full context. Specifically segment registers are typically not
// supplied.
//
// Assuming the address is a linear address (it is for IBS), we fake the CS and
// vm86 mode using the known zero-based code segment and 'fix up' the registers
// to reflect this.
//
// Intel PEBS/LBR appear to typically provide the effective address, nothing
// much we can do about that but pray and treat it like a linear address.
//
// x86control flow change classification
// x86control flow changes include branches, interrupts, traps, faults
//

extern "C" {
    pub fn common_branch_type(type: c_int) -> c_int;
}
extern "C" {
    pub fn branch_type(from: c_ulong, to: c_ulong, abort: c_int) -> c_int;
}
extern "C" {
    pub fn x86_event_sysfs_show(page: *mut c_char, config: u64, event: u64) -> isize;
}
extern "C" {
    pub fn intel_event_sysfs_show(page: *mut c_char, config: u64) -> isize;
}

extern "C" {
    pub fn amd_pmu_init() -> c_int;
}
extern "C" {
    pub fn amd_pmu_lbr_init() -> c_int;
}
extern "C" {
    pub fn amd_pmu_lbr_reset();
}
extern "C" {
    pub fn amd_pmu_lbr_read();
}
extern "C" {
    pub fn amd_pmu_lbr_add(event: *mut perf_event);
}
extern "C" {
    pub fn amd_pmu_lbr_del(event: *mut perf_event);
}
extern "C" {
    pub fn amd_pmu_lbr_enable_all();
}
extern "C" {
    pub fn amd_pmu_lbr_disable_all();
}
extern "C" {
    pub fn amd_pmu_lbr_hw_config(event: *mut perf_event) -> c_int;
}

pub const AMD_FAM19H_BRS_EVENT: c_uint = 0xc4 /* RETIRED_TAKEN_BRANCH_INSTRUCTIONS */;
extern "C" {
    pub fn amd_brs_init() -> c_int;
}
extern "C" {
    pub fn amd_brs_disable();
}
extern "C" {
    pub fn amd_brs_enable();
}
extern "C" {
    pub fn amd_brs_enable_all();
}
extern "C" {
    pub fn amd_brs_disable_all();
}
extern "C" {
    pub fn amd_brs_drain();
}
extern "C" {
    pub fn amd_brs_lopwr_init();
}
extern "C" {
    pub fn amd_brs_hw_config(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn amd_brs_reset();
}
//
// No need to reset BRS because it is reset
// on brs_enable() and it is saturating
//

//
// Only use BTS for fixed rate period==1 events.
//
// BTS doesn't virtualize.
//
extern "C" {
    pub fn intel_pmu_has_bts_period(_arg: event, _arg: hwc->sample_period) -> return;
}
extern "C" {
    pub fn intel_pmu_save_and_restart(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn intel_cpuc_prepare(cpuc: *mut cpu_hw_events, cpu: c_int) -> c_int;
}
extern "C" {
    pub fn intel_cpuc_finish(cpuc: *mut cpu_hw_events);
}
extern "C" {
    pub fn intel_pmu_init() -> c_int;
}
extern "C" {
    pub fn alloc_arch_pebs_buf_on_cpu(cpu: c_int) -> c_int;
}
extern "C" {
    pub fn release_arch_pebs_buf_on_cpu(cpu: c_int);
}
extern "C" {
    pub fn init_arch_pebs_on_cpu(cpu: c_int);
}
extern "C" {
    pub fn fini_arch_pebs_on_cpu(cpu: c_int);
}
extern "C" {
    pub fn init_debug_store_on_cpu(cpu: c_int);
}
extern "C" {
    pub fn fini_debug_store_on_cpu(cpu: c_int);
}
extern "C" {
    pub fn release_ds_buffers();
}
extern "C" {
    pub fn reserve_ds_buffers();
}
extern "C" {
    pub fn release_lbr_buffers();
}
extern "C" {
    pub fn reserve_lbr_buffers();
}
extern "C" {
    pub fn intel_pmu_enable_bts(config: u64);
}
extern "C" {
    pub fn intel_pmu_disable_bts();
}
extern "C" {
    pub fn intel_pmu_drain_bts_buffer() -> c_int;
}
extern "C" {
    pub fn intel_pmu_late_setup();
}
extern "C" {
    pub fn grt_latency_data(event: *mut perf_event, status: u64) -> u64;
}
extern "C" {
    pub fn cmt_latency_data(event: *mut perf_event, status: u64) -> u64;
}
extern "C" {
    pub fn lnl_latency_data(event: *mut perf_event, status: u64) -> u64;
}
extern "C" {
    pub fn arl_h_latency_data(event: *mut perf_event, status: u64) -> u64;
}
extern "C" {
    pub fn pnc_latency_data(event: *mut perf_event, status: u64) -> u64;
}
extern "C" {
    pub fn nvl_latency_data(event: *mut perf_event, status: u64) -> u64;
}
extern "C" {
    pub fn intel_pmu_pebs_add(event: *mut perf_event);
}
extern "C" {
    pub fn intel_pmu_pebs_del(event: *mut perf_event);
}
extern "C" {
    pub fn intel_pmu_pebs_enable(event: *mut perf_event);
}
extern "C" {
    pub fn intel_pmu_pebs_disable(event: *mut perf_event);
}
extern "C" {
    pub fn intel_pmu_pebs_enable_all();
}
extern "C" {
    pub fn intel_pmu_pebs_disable_all();
}
extern "C" {
    pub fn intel_pmu_pebs_sched_task(pmu_ctx: *mut perf_event_pmu_context, sched_in: bool);
}
extern "C" {
    pub fn intel_pmu_pebs_late_setup(cpuc: *mut cpu_hw_events);
}
extern "C" {
    pub fn intel_pmu_drain_pebs_buffer();
}
extern "C" {
    pub fn intel_pmu_store_pebs_lbrs(lbr: *mut lbr_entry);
}
extern "C" {
    pub fn intel_pebs_init();
}
extern "C" {
    pub fn lbr_from_signext_quirk_wr(val: u64) -> u64;
}
extern "C" {
    pub fn intel_pmu_lbr_reset();
}
extern "C" {
    pub fn intel_pmu_lbr_reset_32();
}
extern "C" {
    pub fn intel_pmu_lbr_reset_64();
}
extern "C" {
    pub fn intel_pmu_lbr_add(event: *mut perf_event);
}
extern "C" {
    pub fn intel_pmu_lbr_del(event: *mut perf_event);
}
extern "C" {
    pub fn intel_pmu_lbr_enable_all(pmi: bool);
}
extern "C" {
    pub fn intel_pmu_lbr_disable_all();
}
extern "C" {
    pub fn intel_pmu_lbr_read();
}
extern "C" {
    pub fn intel_pmu_lbr_read_32(cpuc: *mut cpu_hw_events);
}
extern "C" {
    pub fn intel_pmu_lbr_read_64(cpuc: *mut cpu_hw_events);
}
extern "C" {
    pub fn intel_pmu_lbr_save(ctx: *mut c_void);
}
extern "C" {
    pub fn intel_pmu_lbr_restore(ctx: *mut c_void);
}
extern "C" {
    pub fn intel_pmu_lbr_init_core();
}
extern "C" {
    pub fn intel_pmu_lbr_init_nhm();
}
extern "C" {
    pub fn intel_pmu_lbr_init_atom();
}
extern "C" {
    pub fn intel_pmu_lbr_init_slm();
}
extern "C" {
    pub fn intel_pmu_lbr_init_snb();
}
extern "C" {
    pub fn intel_pmu_lbr_init_hsw();
}
extern "C" {
    pub fn intel_pmu_lbr_init_skl();
}
extern "C" {
    pub fn intel_pmu_lbr_init_knl();
}
extern "C" {
    pub fn intel_pmu_lbr_init();
}
extern "C" {
    pub fn intel_pmu_arch_lbr_init();
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_nhm();
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_skl(pmem: bool);
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_adl();
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_grt();
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_mtl();
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_arl_h();
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_cmt();
}
extern "C" {
    pub fn intel_pmu_pebs_data_source_lnl();
}
extern "C" {
    pub fn intel_get_arch_pebs_data_config(event: *mut perf_event) -> u64;
}
extern "C" {
    pub fn intel_pmu_setup_lbr_filter(event: *mut perf_event) -> c_int;
}
extern "C" {
    pub fn intel_pt_interrupt();
}
extern "C" {
    pub fn intel_bts_interrupt() -> c_int;
}
extern "C" {
    pub fn intel_bts_enable_local();
}
extern "C" {
    pub fn intel_bts_disable_local();
}
extern "C" {
    pub fn p4_pmu_init() -> c_int;
}
extern "C" {
    pub fn p6_pmu_init() -> c_int;
}
extern "C" {
    pub fn knc_pmu_init() -> c_int;
}
extern "C" {
    pub fn fls(_arg: (u32)hybrid(pmu, _arg: pebs_events_mask)) -> return;
}

extern "C" {
    pub fn zhaoxin_pmu_init() -> c_int;
}

