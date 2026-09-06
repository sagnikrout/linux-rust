//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/mce.h
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
// Machine Check support for x86
//
// MCG_CAP register defines
pub const MCG_BANKCNT_MASK: c_uint = 0xff         /* Number of Banks */;

pub const MCG_EXT_CNT_MASK: c_uint = 0xff0000     /* Number of Extended registers */;
pub const MCG_EXT_CNT_SHIFT: c_int = 16;

// MCG_STATUS register defines

// MCG_EXT_CTL register defines

// MCi_STATUS register defines

// AMD-specific bits

//
// McaX field if set indicates a given bank supports MCA extensions:
// - Deferred error interrupt type is specifiable by bank.
// - MCx_MISC0[BlkPtr] field indicates presence of extended MISC registers,
// But should not be used to determine MSR numbers.
// - TCC bit is present in MCx_STATUS.
//
pub const MCI_CONFIG_MCAX: c_uint = 0x1;

pub const MCI_IPID_MCATYPE: c_uint = 0xFFFF0000;
pub const MCI_IPID_HWID: c_uint = 0xFFF;
//
// Note that the full MCACOD field of IA32_MCi_STATUS MSR is
// bits 15:0.  But bit 12 is the 'F' bit, defined for corrected
// errors to indicate that errors are being filtered by hardware.
// We should mask out bit 12 when looking for specific signatures
// of uncorrected errors - so the F bit is deliberately skipped
// in this #define.
//
pub const MCACOD: c_uint = 0xefff     /* MCA Error Code */;
// Architecturally defined codes from SDM Vol. 3B Chapter 15
pub const MCACOD_SCRUB: c_uint = 0x00C0	/* 0xC0-0xCF Memory Scrubbing */;
pub const MCACOD_SCRUBMSK: c_uint = 0xeff0	/* Skip bit 12 ('F' bit) */;
pub const MCACOD_L3WB: c_uint = 0x017A	/* L3 Explicit Writeback */;
pub const MCACOD_DATA: c_uint = 0x0134	/* Data Load */;
pub const MCACOD_INSTR: c_uint = 0x0150	/* Instruction Fetch */;
// MCi_MISC register defines

// MCi_ADDR register defines

// CTL2 register defines

pub const MCI_CTL2_CMCI_THRESHOLD_MASK: c_uint = 0x7fffULL;
pub const MCJ_CTX_MASK: c_int = 3;

pub const MCJ_CTX_PROCESS: c_uint = 0x1  /* inject context: process */;
pub const MCJ_CTX_IRQ: c_uint = 0x2  /* inject context: IRQ */;
pub const MCJ_NMI_BROADCAST: c_uint = 0x4  /* do NMI broadcasting */;
pub const MCJ_EXCEPTION: c_uint = 0x8  /* raise as exception */;
pub const MCJ_IRQ_BROADCAST: c_uint = 0x10 /* do IRQ broadcasting */;

// AMD Scalable MCA
pub const MSR_AMD64_SMCA_MC0_CTL: c_uint = 0xc0002000;
pub const MSR_AMD64_SMCA_MC0_STATUS: c_uint = 0xc0002001;
pub const MSR_AMD64_SMCA_MC0_ADDR: c_uint = 0xc0002002;
pub const MSR_AMD64_SMCA_MC0_MISC0: c_uint = 0xc0002003;
pub const MSR_AMD64_SMCA_MC0_CONFIG: c_uint = 0xc0002004;
pub const MSR_AMD64_SMCA_MC0_IPID: c_uint = 0xc0002005;
pub const MSR_AMD64_SMCA_MC0_SYND: c_uint = 0xc0002006;
pub const MSR_AMD64_SMCA_MC0_DESTAT: c_uint = 0xc0002008;
pub const MSR_AMD64_SMCA_MC0_DEADDR: c_uint = 0xc0002009;
pub const MSR_AMD64_SMCA_MC0_MISC1: c_uint = 0xc000200a;
// Registers MISC2 to MISC4 are at offsets B to D.
pub const MSR_AMD64_SMCA_MC0_SYND1: c_uint = 0xc000200e;
pub const MSR_AMD64_SMCA_MC0_SYND2: c_uint = 0xc000200f;

// mce.kflags flag bits for logging etc.

//
// Indicates an MCE which has happened in kernel space but from
// which the kernel can recover simply by executing fixup_exception()
// so that an error is returned to the caller of the function that
// hit the machine check.
//

//
// Indicates an MCE that happened in kernel space while copying data
// from user. In this case fixup_exception() gets the kernel to the
// error exit for the copy function. Machine check handler can then
// treat it like a fault taken in user mode.
//

//
// Indicates that handler should check and clear Deferred error registers
// rather than common ones.
//

//
// This structure contains all data related to the MCE log.  Also
// carries a signature to make it easier to find from external
// debugging tools.  Each entry is only valid when its finished flag
// is set.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_log_buffer {
    pub /: *mut *mut char signature[12]; / "MACHINECHECK",
    pub /: *mut *mut unsigned len; / = elements in .mce_entry[],
    pub next: unsigned,
    pub flags: unsigned,
    pub /: *mut *mut unsigned recordlen; / length of struct mce,
    pub entry: [mce; ],
}

// Highest last
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mce_notifier_prios {
    MCE_PRIO_LOWEST,
    MCE_PRIO_MCELOG,
    MCE_PRIO_EDAC,
    MCE_PRIO_NFIT,
    MCE_PRIO_EXTLOG,
    MCE_PRIO_UC,
    MCE_PRIO_EARLY,
    MCE_PRIO_CEC,
    MCE_PRIO_HIGHEST = MCE_PRIO_CEC
}

//
// struct mce_hw_err - Hardware Error Record.
// @m:		Machine Check record.
// @vendor:	Vendor-specific error information.
//
// Vendor-specific fields should not be added to struct mce. Instead, vendors
// should export their vendor-specific data through their structure in the
// vendor union below.
//
// AMD's vendor data is parsed by error decoding tools for supplemental error
// information. Thus, current offsets of existing fields must be maintained.
// Only add new fields at the end of AMD's vendor structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_hw_err {
    pub m: mce,
#[repr(C)]
#[derive(Copy, Clone)]
pub union vendor_info {
    pub /: *mut *mut u64 synd1; / MCA_SYND1 MSR,
    pub /: *mut *mut u64 synd2; / MCA_SYND2 MSR,
    pub amd: },
    pub vendor: },
}

extern "C" {
    pub fn mce_register_decode_chain(nb: *mut notifier_block);
}
extern "C" {
    pub fn mce_unregister_decode_chain(nb: *mut notifier_block);
}

extern "C" {
    pub fn enable_copy_mc_fragile();
}
extern "C" {
    pub fn copy_mc_fragile(dst: *mut c_void, src: *const c_void, cnt: unsigned) -> unsigned long __must_check;
}

extern "C" {
    pub fn mcheck_init() -> c_int;
}
extern "C" {
    pub fn mca_bsp_init(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn mcheck_cpu_init(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn mcheck_cpu_clear(c: *mut cpuinfo_x86);
}

extern "C" {
    pub fn mce_prep_record(err: *mut mce_hw_err);
}
extern "C" {
    pub fn mce_log(err: *mut mce_hw_err);
}
// Maximum number of MCA banks per CPU.
pub const MAX_NR_BANKS: c_int = 64;

extern "C" {
    pub fn mce_intel_feature_init(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn mce_intel_feature_clear(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn cmci_clear();
}
extern "C" {
    pub fn cmci_reenable();
}
extern "C" {
    pub fn cmci_rediscover();
}
extern "C" {
    pub fn cmci_recheck();
}

extern "C" {
    pub fn mce_available(c: *mut cpuinfo_x86) -> bool;
}
extern "C" {
    pub fn mce_is_memory_error(m: *mut mce) -> bool;
}
extern "C" {
    pub fn mce_is_correctable(m: *mut mce) -> bool;
}
extern "C" {
    pub fn mce_usable_address(m: *mut mce) -> bool;
}
extern "C" {
    pub fn DECLARE_BITMAP(_arg: mce_banks_t, _arg: MAX_NR_BANKS) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mcp_flags {
    MCP_TIMESTAMP	= BIT(0),	/* log time stamp */
    MCP_UC		= BIT(1),	/* log uncorrected errors */
    MCP_QUEUE_LOG	= BIT(2),	/* only queue to genpool */
}

extern "C" {
    pub fn machine_check_poll(flags: mcp_flags, b: *mut mce_banks_t);
}
// Disable CMCI/polling for MCA bank claimed by firmware
extern "C" {
    pub fn mce_disable_bank(bank: c_int);
}

extern "C" {
    pub fn mce_save_apei_thr_limit(thr_limit: u32);
}

//
// Exception handler
//
extern "C" {
    pub fn do_machine_check(pt_regs: *mut pt_regs);
}
//
// Threshold handler
//
extern "C" {
    pub fn void(_arg: *mut mce_threshold_vector)(void) -> extern;
}
// Deferred error interrupt handler
extern "C" {
    pub fn void(_arg: *mut deferred_error_int_vector)(void) -> extern;
}
//
// Used by APEI to report memory error via /dev/mcelog
//
// Enumerate new IP types and HWID values in AMD processors which support
// Scalable MCA.
//

//
// These may be used by multiple smca_hwid_mcatypes.
//
// Keep in alphanumeric order, numerals before letters.
// Exception: Keep "V2, etc." with their originals.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smca_bank_types {
    SMCA_CS,	/* Coherent Station */
    SMCA_CS_V2,
    SMCA_DACC_BE,	/* Data Acceleration Back-end */
    SMCA_DACC_FE,	/* Data Acceleration Front-end */
    SMCA_DE,	/* Decoder Unit */
    SMCA_EDDR5CMN,	/* eDDR5 CMN */
    SMCA_EX,	/* Execution Unit */
    SMCA_FP,	/* Floating Point */
    SMCA_GMI_PCS,	/* GMI PCS Unit */
    SMCA_GMI_PHY,	/* GMI PHY Unit */
    SMCA_IF,	/* Instruction Fetch */
    SMCA_L2_CACHE,	/* L2 Cache */
    SMCA_L3_CACHE,	/* L3 Cache */
    SMCA_LS,	/* Load Store */
    SMCA_LS_V2,
    SMCA_MA_LLC,	/* Memory Attached Last Level Cache */
    SMCA_MP5,	/* Microprocessor 5 Unit */
    SMCA_MPART,	/* AMD Root of Trust Microprocessor */
    SMCA_MPASP,	/* AMD Secure Processor */
    SMCA_MPASP_V2,
    SMCA_MPDACC,	/* MP for Data Acceleration */
    SMCA_MPDMA,	/* MPDMA Unit */
    SMCA_MPM,	/* Microprocessor Manageability Core */
    SMCA_MPRAS,	/* MP for RAS */
    SMCA_NBIF,	/* NBIF Unit */
    SMCA_NBIO,	/* Northbridge IO Unit */
    SMCA_PB,	/* Parameter Block */
    SMCA_PCIE,	/* PCI Express Unit */
    SMCA_PCIE_V2,
    SMCA_PCIE_PL,	/* PCIe Link */
    SMCA_PIE,	/* Power, Interrupts, etc. */
    SMCA_PSP,	/* Platform Security Processor */
    SMCA_PSP_V2,
    SMCA_RESERVED,	/* Reserved */
    SMCA_SATA,	/* SATA Unit */
    SMCA_SHUB,	/* System HUB Unit */
    SMCA_SMU,	/* System Management Unit */
    SMCA_SMU_V2,
    SMCA_SSBDCI,	/* Die to Die Interconnect */
    SMCA_UMC,	/* Unified Memory Controller */
    SMCA_UMC_V2,
    SMCA_USB,	/* USB Unit */
    SMCA_USR_CP,	/* Ultra Short Reach Control Plane Controller */
    SMCA_USR_DP,	/* Ultra Short Reach Data Plane Controller */
    SMCA_WAFL_PHY,	/* WAFL PHY Unit */
    SMCA_XGMI_PCS,	/* xGMI PCS Unit */
    SMCA_XGMI_PHY,	/* xGMI PHY Unit */
    N_SMCA_BANK_TYPES
}

extern "C" {
    pub fn amd_mce_is_memory_error(m: *mut mce) -> bool;
}
extern "C" {
    pub fn mce_amd_feature_init(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn smca_get_bank_type(cpu: c_uint, bank: c_uint) -> smca_bank_types;
}

extern "C" {
    pub fn copy_mc_fragile_handle_tail(to: *mut c_char, from: *mut c_char, len: unsigned) -> c_ulong;
}
