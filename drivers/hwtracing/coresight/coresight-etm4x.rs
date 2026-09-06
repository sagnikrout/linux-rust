//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hwtracing/coresight/coresight-etm4x.h
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
// Copyright (c) 2014-2015, The Linux Foundation. All rights reserved.
//

//
// Device registers:
// 0x000 - 0x2FC: Trace		registers
// 0x300 - 0x314: Management	registers
// 0x318 - 0xEFC: Trace		registers
// 0xF00: Management		registers
// 0xFA0 - 0xFA4: Trace		registers
// 0xFA8 - 0xFFC: Management	registers
//
// Trace registers (0x000-0x2FC)
// Main control and configuration registers
pub const TRCPRGCTLR: c_uint = 0x004;
pub const TRCPROCSELR: c_uint = 0x008;
pub const TRCSTATR: c_uint = 0x00C;
pub const TRCCONFIGR: c_uint = 0x010;
pub const TRCAUXCTLR: c_uint = 0x018;
pub const TRCEVENTCTL0R: c_uint = 0x020;
pub const TRCEVENTCTL1R: c_uint = 0x024;
pub const TRCRSR: c_uint = 0x028;
pub const TRCSTALLCTLR: c_uint = 0x02C;
pub const TRCTSCTLR: c_uint = 0x030;
pub const TRCSYNCPR: c_uint = 0x034;
pub const TRCCCCTLR: c_uint = 0x038;
pub const TRCBBCTLR: c_uint = 0x03C;
pub const TRCTRACEIDR: c_uint = 0x040;
pub const TRCQCTLR: c_uint = 0x044;
// Filtering control registers
pub const TRCVICTLR: c_uint = 0x080;
pub const TRCVIIECTLR: c_uint = 0x084;
pub const TRCVISSCTLR: c_uint = 0x088;
pub const TRCVIPCSSCTLR: c_uint = 0x08C;
// Derived resources registers

pub const TRCSEQRSTEVR: c_uint = 0x118;
pub const TRCSEQSTR: c_uint = 0x11C;
pub const TRCEXTINSELR: c_uint = 0x120;

// ID registers
pub const TRCIDR8: c_uint = 0x180;
pub const TRCIDR9: c_uint = 0x184;
pub const TRCIDR10: c_uint = 0x188;
pub const TRCIDR11: c_uint = 0x18C;
pub const TRCIDR12: c_uint = 0x190;
pub const TRCIDR13: c_uint = 0x194;
pub const TRCIMSPEC0: c_uint = 0x1C0;

pub const TRCIDR0: c_uint = 0x1E0;
pub const TRCIDR1: c_uint = 0x1E4;
pub const TRCIDR2: c_uint = 0x1E8;
pub const TRCIDR3: c_uint = 0x1EC;
pub const TRCIDR4: c_uint = 0x1F0;
pub const TRCIDR5: c_uint = 0x1F4;
pub const TRCIDR6: c_uint = 0x1F8;
pub const TRCIDR7: c_uint = 0x1FC;
//
// Resource selection registers, n = 2-31.
// First pair (regs 0, 1) is always present and is reserved.
//

// Single-shot comparator registers, n = 0-7

// Management registers (0x300-0x314)
pub const TRCOSLAR: c_uint = 0x300;
pub const TRCOSLSR: c_uint = 0x304;
pub const TRCPDCR: c_uint = 0x310;
pub const TRCPDSR: c_uint = 0x314;
// Trace registers (0x318-0xEFC)
// Address Comparator registers n = 0-15

// ContextID/Virtual ContextID comparators, n = 0-7

pub const TRCCIDCCTLR0: c_uint = 0x680;
pub const TRCCIDCCTLR1: c_uint = 0x684;
pub const TRCVMIDCCTLR0: c_uint = 0x688;
pub const TRCVMIDCCTLR1: c_uint = 0x68C;
// Management register (0xF00)
// Integration control registers
pub const TRCITCTRL: c_uint = 0xF00;
// Trace registers (0xFA0-0xFA4)
// Claim tag registers
pub const TRCCLAIMSET: c_uint = 0xFA0;
pub const TRCCLAIMCLR: c_uint = 0xFA4;
// Management registers (0xFA8-0xFFC)
pub const TRCDEVAFF0: c_uint = 0xFA8;
pub const TRCDEVAFF1: c_uint = 0xFAC;
pub const TRCLAR: c_uint = 0xFB0;
pub const TRCLSR: c_uint = 0xFB4;
pub const TRCAUTHSTATUS: c_uint = 0xFB8;
pub const TRCDEVARCH: c_uint = 0xFBC;
pub const TRCDEVID: c_uint = 0xFC8;
pub const TRCDEVTYPE: c_uint = 0xFCC;
pub const TRCPIDR4: c_uint = 0xFD0;
pub const TRCPIDR5: c_uint = 0xFD4;
pub const TRCPIDR6: c_uint = 0xFD8;
pub const TRCPIDR7: c_uint = 0xFDC;
pub const TRCPIDR0: c_uint = 0xFE0;
pub const TRCPIDR1: c_uint = 0xFE4;
pub const TRCPIDR2: c_uint = 0xFE8;
pub const TRCPIDR3: c_uint = 0xFEC;
pub const TRCCIDR0: c_uint = 0xFF0;
pub const TRCCIDR1: c_uint = 0xFF4;
pub const TRCCIDR2: c_uint = 0xFF8;
pub const TRCCIDR3: c_uint = 0xFFC;

//
// Bit positions of registers that are defined above, in the sysreg.h style
// of _MASK for multi bit fields and BIT() for single bits.
//

//
// Utilities for programming EVENT resource selectors, e.g. TRCCNTCTLRn_RLDEVENT.
//
// Resource selectors have a common format across registers:
//
// 7     6  5  4     0
// +------+------+-------+
// | TYPE | RES0 |  SEL  |
// +------+------+-------+
//
// Where TYPE indicates whether the selector is for a single event or a pair.
// When TYPE is pair, SEL is 4 bits wide and using pair 0 is UNPREDICTABLE.
// Otherwise for single it's 5 bits wide.
//
extern "C" {
    pub fn FIELD_PREP(_arg: ETM4_RES_SEL_SINGLE_MASK, _arg: res_sel_idx) -> return;
}
//
// System instructions to access ETM registers.
// See ETMv4.4 spec ARM IHI0064F section 4.3.6 System instructions
//

// List of registers accessible via System instructions

// List of registers only accessible via memory-mapped interface

// ETE only supports system register access

// ETMv4 resources
pub const ETM_MAX_NR_PE: c_int = 8;
pub const ETMv4_MAX_CNTR: c_int = 4;
pub const ETM_MAX_SEQ_STATES: c_int = 4;
pub const ETM_MAX_SEQ_TRANSITIONS: c_int = 3;
pub const ETM_MAX_EXT_INP_SEL: c_int = 4;
pub const ETM_MAX_EXT_INP: c_int = 256;
pub const ETM_MAX_EXT_OUT: c_int = 4;
pub const ETM_MAX_SINGLE_ADDR_CMP: c_int = 16;

pub const ETM_MAX_DATA_VAL_CMP: c_int = 8;
pub const ETMv4_MAX_CTXID_CMP: c_int = 8;
pub const ETM_MAX_VMID_CMP: c_int = 8;
pub const ETM_MAX_PE_CMP: c_int = 8;
pub const ETM_MAX_RES_SEL: c_int = 32;
pub const ETM_MAX_SS_CMP: c_int = 8;
pub const ETMv4_SYNC_MASK: c_uint = 0x1F;
pub const ETM_CYC_THRESHOLD_MASK: c_uint = 0xFFF;
pub const ETM_CYC_THRESHOLD_DEFAULT: c_uint = 0x100;
pub const ETMv4_EVENT_MASK: c_uint = 0xFF;
pub const ETM_CNTR_MAX_VAL: c_uint = 0xFFFF;
pub const ETM_TRACEID_MASK: c_uint = 0x3f;
// ETMv4 programming modes

//
// TRCOSLSR.OSLM advertises the OS Lock model.
// OSLM[2:0] = TRCOSLSR[4:3,0]
//
// 0b000 - Trace OS Lock is not implemented.
// 0b010 - Trace OS Lock is implemented.
// 0b100 - Trace OS Lock is not implemented, unit is controlled by PE OS Lock.
//

//
// TRCDEVARCH Bit field definitions
// Bits[31:21]	- ARCHITECT = Always Arm Ltd.
// * Bits[31:28] = 0x4
// * Bits[27:21] = 0b0111011
// Bit[20]	- PRESENT,  Indicates the presence of this register.
//
// Bit[19:16]	- REVISION, Revision of the architecture.
//
// Bit[15:0]	- ARCHID, Identifies this component as an ETM
// * Bits[15:12] - architecture version of ETM
// *             = 4 for ETMv4
// * Bits[11:0] = 0xA13, architecture part number for ETM.
//

pub const ETM_DEVARCH_REVISION_SHIFT: c_int = 16;

pub const ETM_DEVARCH_ARCHID_ARCH_VER_SHIFT: c_int = 12;

pub const CS_DEVTYPE_PE_TRACE: c_uint = 0x00000013;
pub const TRCSTATR_IDLE_BIT: c_int = 0;
pub const TRCSTATR_PMSTABLE_BIT: c_int = 1;
pub const ETM_DEFAULT_ADDR_COMP: c_int = 0;

// PowerDown Control Register bits

pub const TRCACATR_EXLEVEL_SHIFT: c_int = 8;
//
// Exception level mask for Secure and Non-Secure ELs.
// ETM defines the bits for EL control (e.g, TRVICTLR, TRCACTRn).
// The Secure and Non-Secure ELs are always to gether.
// Non-secure EL3 is never implemented.
// We use the following generic mask as they appear in different
// registers and this can be shifted for the appropriate
// fields.
//

// access level controls in TRCACATRn
pub const TRCACATR_EXLEVEL_SHIFT: c_int = 8;
pub const ETM_TRCIDR1_ARCH_MAJOR_SHIFT: c_int = 8;

pub const ETM_TRCIDR1_ARCH_MINOR_SHIFT: c_int = 4;

pub const ETM_TRCIDR1_ARCH_ETMv4: c_uint = 0x4;
//
// Driver representation of the ETM architecture.
// The version of an ETM component can be detected from
//
// TRCDEVARCH	- CoreSight architected register
// - Bits[15:12] - Major version
// - Bits[19:16] - Minor version
//
// We must rely only on TRCDEVARCH for the version information. Even though,
// TRCIDR1 also provides the architecture version, it is a "Trace" register
// and as such must be accessed only with Trace power domain ON. This may
// not be available at probe time.
//
// Now to make certain decisions easier based on the version
// we use an internal representation of the version in the
// driver, as follows :
//
// ETM_ARCH_VERSION[7:0], where :
// Bits[7:4] - Major version
// Bits[3:0] - Minro version
//

// Interpretation of resource numbers change at ETM v4.3 architecture

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etm_impdef_type {
    ETM4_IMPDEF_HISI_CORE_COMMIT,
    ETM4_IMPDEF_FEATURE_MAX,
}

//
// struct etmv4_config - configuration information related to an ETMv4
// @mode:	Controls various modes supported by this ETM.
// @pe_sel:	Controls which PE to trace.
// @cfg:	Controls the tracing options.
// @eventctrl0: Controls the tracing of arbitrary events.
// @eventctrl1: Controls the behavior of the events that @event_ctrl0 selects.
// @stallctl:	If functionality that prevents trace unit buffer overflows
// is available.
// @ts_ctrl:	Controls the insertion of global timestamps in the
// trace streams.
// @syncfreq:	Controls how often trace synchronization requests occur.
// the TRCCCCTLR register.
// @ccctlr:	Sets the threshold value for cycle counting.
// @vinst_ctrl:	Controls instruction trace filtering.
// @viiectlr:	Set or read, the address range comparators.
// @vissctlr:	Set, or read, the single address comparators that control the
// ViewInst start-stop logic.
// @vipcssctlr:	Set, or read, which PE comparator inputs can control the
// ViewInst start-stop logic.
// @seq_idx:	Sequencor index selector.
// @seq_ctrl:	Control for the sequencer state transition control register.
// @seq_rst:	Moves the sequencer to state 0 when a programmed event occurs.
// @seq_state:	Set, or read the sequencer state.
// @cntr_idx:	Counter index seletor.
// @cntrldvr:	Sets or returns the reload count value for a counter.
// @cntr_ctrl:	Controls the operation of a counter.
// @cntr_val:	Sets or returns the value for a counter.
// @res_idx:	Resource index selector.
// @res_ctrl:	Controls the selection of the resources in the trace unit.
// @ss_idx:	Single-shot index selector.
// @ss_ctrl:	Controls the corresponding single-shot comparator resource.
// @ss_status:	The status of the corresponding single-shot comparator.
// @ss_pe_cmp:	Selects the PE comparator inputs for Single-shot control.
// @addr_idx:	Address comparator index selector.
// @addr_val:	Value for address comparator.
// @addr_acc:	Address comparator access type.
// @addr_type:	Current status of the comparator register.
// @ctxid_idx:	Context ID index selector.
// @ctxid_pid:	Value of the context ID comparator.
// @ctxid_mask0:Context ID comparator mask for comparator 0-3.
// @ctxid_mask1:Context ID comparator mask for comparator 4-7.
// @vmid_idx:	VM ID index selector.
// @vmid_val:	Value of the VM ID comparator.
// @vmid_mask0:	VM ID comparator mask for comparator 0-3.
// @vmid_mask1:	VM ID comparator mask for comparator 4-7.
// @ext_inp:	External input selection.
// @s_ex_level: Secure ELs where tracing is supported.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etmv4_config {
    pub mode: u64,
    pub pe_sel: u32,
    pub cfg: u32,
    pub eventctrl0: u32,
    pub eventctrl1: u32,
    pub stall_ctrl: u32,
    pub /: *mut *mut u32 ts_ctrl; / TRCTSCTLR,
    pub ccctlr: u32,
    pub bb_ctrl: u32,
    pub vinst_ctrl: u32,
    pub viiectlr: u32,
    pub vissctlr: u32,
    pub vipcssctlr: u32,
    pub seq_idx: u8,
    pub syncfreq: u8,
    pub seq_ctrl: [u32; ETM_MAX_SEQ_TRANSITIONS],
    pub seq_rst: u32,
    pub seq_state: u32,
    pub cntr_idx: u8,
    pub /: *mut *mut u32 cntrldvr[ETMv4_MAX_CNTR]; / TRCCNTRLDVRn,
    pub /: *mut *mut u32 cntr_ctrl[ETMv4_MAX_CNTR]; / TRCCNTCTLRn,
    pub /: *mut *mut u32 cntr_val[ETMv4_MAX_CNTR]; / TRCCNTVRn,
    pub res_idx: u8,
    pub /: *mut *mut u32 res_ctrl[ETM_MAX_RES_SEL]; / TRCRSCTLRn,
    pub ss_idx: u8,
    pub ss_ctrl: [u32; ETM_MAX_SS_CMP],
    pub ss_status: [u32; ETM_MAX_SS_CMP],
    pub ss_pe_cmp: [u32; ETM_MAX_SS_CMP],
    pub addr_idx: u8,
    pub addr_val: [u64; ETM_MAX_SINGLE_ADDR_CMP],
    pub addr_acc: [u64; ETM_MAX_SINGLE_ADDR_CMP],
    pub addr_type: [u8; ETM_MAX_SINGLE_ADDR_CMP],
    pub ctxid_idx: u8,
    pub ctxid_pid: [u64; ETMv4_MAX_CTXID_CMP],
    pub ctxid_mask0: u32,
    pub ctxid_mask1: u32,
    pub vmid_idx: u8,
    pub vmid_val: [u64; ETM_MAX_VMID_CMP],
    pub vmid_mask0: u32,
    pub vmid_mask1: u32,
    pub ext_inp: u32,
    pub s_ex_level: u8,
}

//
// struct etm4_save_state - state to be preserved when ETM is without power
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etmv4_save_state {
    pub trcprocselr: u32,
    pub trcconfigr: u32,
    pub trcauxctlr: u32,
    pub trceventctl0r: u32,
    pub trceventctl1r: u32,
    pub trcstallctlr: u32,
    pub trctsctlr: u32,
    pub trcsyncpr: u32,
    pub trcccctlr: u32,
    pub trcbbctlr: u32,
    pub trctraceidr: u32,
    pub trcqctlr: u32,
    pub trcvictlr: u32,
    pub trcviiectlr: u32,
    pub trcvissctlr: u32,
    pub trcvipcssctlr: u32,
    pub trcseqevr: [u32; ETM_MAX_SEQ_TRANSITIONS],
    pub trcseqrstevr: u32,
    pub trcseqstr: u32,
    pub trcextinselr: u32,
    pub trccntrldvr: [u32; ETMv4_MAX_CNTR],
    pub trccntctlr: [u32; ETMv4_MAX_CNTR],
    pub trccntvr: [u32; ETMv4_MAX_CNTR],
    pub trcrsctlr: [u32; ETM_MAX_RES_SEL],
    pub trcssccr: [u32; ETM_MAX_SS_CMP],
    pub trcsscsr: [u32; ETM_MAX_SS_CMP],
    pub trcsspcicr: [u32; ETM_MAX_SS_CMP],
    pub trcacvr: [u64; ETM_MAX_SINGLE_ADDR_CMP],
    pub trcacatr: [u64; ETM_MAX_SINGLE_ADDR_CMP],
    pub trccidcvr: [u64; ETMv4_MAX_CTXID_CMP],
    pub trcvmidcvr: [u64; ETM_MAX_VMID_CMP],
    pub trccidcctlr0: u32,
    pub trccidcctlr1: u32,
    pub trcvmidcctlr0: u32,
    pub trcvmidcctlr1: u32,
    pub trcclaimset: u32,
    pub trcpdcr: u32,
}

//
// struct etm4_drvdata - specifics associated to an ETM component
// @pclk:       APB clock if present, otherwise NULL
// @atclk:      Optional clock for the core parts of the ETMv4.
// @base:       Memory mapped base address for this component.
// @csdev:      Component vitals needed by the framework.
// @spinlock:   Only one at a time pls.
// @mode:	This tracer's mode, i.e sysFS, Perf or disabled.
// @cpu:        The cpu this component is affined to.
// @arch:       ETM architecture version.
// @nr_pe:	The number of processing entity available for tracing.
// @nr_pe_cmp:	The number of processing entity comparator inputs that are
// available for tracing.
// @nr_addr_cmp:Number of pairs of address comparators available
// as found in ETMIDR4 0-3.
// @nr_cntr:    Number of counters as found in ETMIDR5 bit 28-30.
// @nr_ext_inp: Number of external input.
// @numcidc:	Number of contextID comparators.
// @numvmidc:	Number of VMID comparators.
// @nrseqstate: The number of sequencer states that are implemented.
// @nr_seq_ctrls: The number of sequence state transition control registers.
// @nr_event:	Indicates how many events the trace unit support.
// @nr_resource:The number of resource selection pairs available for tracing.
// @nr_ss_cmp:	Number of single-shot comparator controls that are available.
// @trcid:	value of the current ID for this component.
// @trcid_size: Indicates the trace ID width.
// @ts_size:	Global timestamp size field.
// @ctxid_size:	Size of the context ID field to consider.
// @vmid_size:	Size of the VM ID comparator to consider.
// @ccsize:	Indicates the size of the cycle counter in bits.
// @ccitmin:	minimum value that can be programmed in
// @s_ex_level:	In secure state, indicates whether instruction tracing is
// supported for the corresponding Exception level.
// @ns_ex_level:In non-secure state, indicates whether instruction tracing is
// supported for the corresponding Exception level.
// @sticky_enable: true if ETM base configuration has been done.
// @boot_enable:True if we should start tracing at boot time.
// @os_unlock:  True if access to management registers is allowed.
// @instrp0:	Tracing of load and store instructions
// as P0 elements is supported.
// @q_filt:	Q element filtering support, if Q elements are supported.
// @trcbb:	Indicates if the trace unit supports branch broadcast tracing.
// @trccond:	If the trace unit supports conditional
// instruction tracing.
// @retstack:	Indicates if the implementation supports a return stack.
// @trccci:	Indicates if the trace unit supports cycle counting
// for instruction.
// @q_support:	Q element support characteristics.
// @trc_error:	Whether a trace unit can trace a system
// error exception.
// @syncpr:	Indicates if an implementation has a fixed
// synchronization period.
// @stall_ctrl:	Enables trace unit functionality that prevents trace
// unit buffer overflows.
// @sysstall:	Does the system support stall control of the PE?
// @nooverflow:	Indicate if overflow prevention is supported.
// @atbtrig:	If the implementation can support ATB triggers
// @lpoverride:	If the implementation can support low-power state over.
// @trfcr:	If the CPU supports FEAT_TRF, value of the TRFCR_ELx that
// allows tracing at all ELs. We don't want to compute this
// at runtime, due to the additional setting of TRFCR_CX when
// in EL2. Otherwise, 0.
// @config:	structure holding configuration parameters.
// @save_state:	State to be preserved across power loss
// @skip_power_up: Indicates if an implementation can skip powering up
// the trace unit.
// @paused:	Indicates if the trace unit is paused.
// @arch_features: Bitmap of arch features of etmv4 devices.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct etmv4_drvdata {
    pub pclk: *mut clk,
    pub atclk: *mut clk,
    pub base: *mut void __iomem,
    pub csdev: *mut coresight_device,
    pub spinlock: raw_spinlock_t,
    pub cpu: c_int,
    pub arch: u8,
    pub nr_pe: u8,
    pub nr_pe_cmp: u8,
    pub nr_addr_cmp: u8,
    pub nr_cntr: u8,
    pub nr_ext_inp: u8,
    pub numcidc: u8,
    pub numextinsel: u8,
    pub numvmidc: u8,
    pub nrseqstate: u8,
    pub nr_seq_ctrls: u8,
    pub nr_event: u8,
    pub nr_resource: u8,
    pub nr_ss_cmp: u8,
    pub trcid: u8,
    pub trcid_size: u8,
    pub ts_size: u8,
    pub ctxid_size: u8,
    pub vmid_size: u8,
    pub ccsize: u8,
    pub ccitmin: u16,
    pub s_ex_level: u8,
    pub ns_ex_level: u8,
    pub q_support: u8,
    pub os_lock_model: u8,
    pub 1: bool sticky_enable :,
    pub 1: bool boot_enable :,
    pub 1: bool os_unlock :,
    pub 1: bool instrp0 :,
    pub 1: bool q_filt :,
    pub 1: bool trcbb :,
    pub 1: bool trccond :,
    pub 1: bool retstack :,
    pub 1: bool trccci :,
    pub 1: bool trc_error :,
    pub 1: bool syncpr :,
    pub 1: bool stallctl :,
    pub 1: bool sysstall :,
    pub 1: bool nooverflow :,
    pub 1: bool atbtrig :,
    pub 1: bool lpoverride :,
    pub 1: bool skip_power_up :,
    pub 1: bool paused :,
    pub trfcr: u64,
    pub config: etmv4_config,
    pub save_state: *mut etmv4_save_state,
    pub ETM4_IMPDEF_FEATURE_MAX): DECLARE_BITMAP(arch_features,,
}

// Address comparator access types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etm_addr_acctype {
    TRCACATRn_TYPE_ADDR,
    TRCACATRn_TYPE_DATA_LOAD_ADDR,
    TRCACATRn_TYPE_DATA_STORE_ADDR,
    TRCACATRn_TYPE_DATA_LOAD_STORE_ADDR,
}

// Address comparator context types
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum etm_addr_ctxtype {
    ETM_CTX_NONE,
    ETM_CTX_CTXID,
    ETM_CTX_VMID,
    ETM_CTX_CTXID_VMID,
}

extern "C" {
    pub fn etm4_config_trace_mode(config: *mut etmv4_config);
}
extern "C" {
    pub fn etm4x_sysreg_read(offset: u32, _relaxed: bool, _64bit: bool) -> u64;
}
extern "C" {
    pub fn etm4x_sysreg_write(val: u64, offset: u32, _relaxed: bool, _64bit: bool);
}
extern "C" {
    pub fn etm4_release_trace_id(drvdata: *mut etmv4_drvdata);
}
