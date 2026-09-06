//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kernel/cpu/mce/internal.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum severity_level {
    MCE_NO_SEVERITY,
    MCE_DEFERRED_SEVERITY,
    MCE_UCNA_SEVERITY = MCE_DEFERRED_SEVERITY,
    MCE_KEEP_SEVERITY,
    MCE_SOME_SEVERITY,
    MCE_AO_SEVERITY,
    MCE_UC_SEVERITY,
    MCE_AR_SEVERITY,
    MCE_PANIC_SEVERITY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_evt_llist {
    pub llnode: llist_node,
    pub err: mce_hw_err,
}

extern "C" {
    pub fn mce_gen_pool_process(__unused: *mut work_struct);
}
extern "C" {
    pub fn mce_gen_pool_empty() -> bool;
}
extern "C" {
    pub fn mce_gen_pool_add(err: *mut mce_hw_err) -> bool;
}
extern "C" {
    pub fn mce_gen_pool_init() -> bool;
}
extern "C" {
    pub fn mce_severity(a: *mut mce, regs: *mut pt_regs, msg: *mut c_char, is_excp: bool) -> c_int;
}

extern "C" {
    pub fn mce_intel_handle_storm(bank: c_int, on: bool);
}
extern "C" {
    pub fn cmci_disable_bank(bank: c_int);
}
extern "C" {
    pub fn intel_init_cmci();
}
extern "C" {
    pub fn intel_init_lmce();
}
extern "C" {
    pub fn intel_clear_lmce();
}
extern "C" {
    pub fn intel_filter_mce(m: *mut mce) -> bool;
}
extern "C" {
    pub fn intel_mce_usable_address(m: *mut mce) -> bool;
}

extern "C" {
    pub fn mce_timer_kick(storm: bool);
}

extern "C" {
    pub fn cmci_storm_begin(bank: c_uint);
}
extern "C" {
    pub fn cmci_storm_end(bank: c_uint);
}
extern "C" {
    pub fn mce_track_storm(mce: *mut mce);
}
extern "C" {
    pub fn mce_inherit_storm(bank: c_uint);
}
extern "C" {
    pub fn mce_get_storm_mode() -> bool;
}
extern "C" {
    pub fn mce_set_storm_mode(storm: bool);
}
extern "C" {
    pub fn mce_get_apei_thr_limit() -> u32;
}

//
// history:		Bitmask tracking errors occurrence. Each set bit
// represents an error seen.
//
// timestamp:		Last time (in jiffies) that the bank was polled.
// in_storm_mode:	Is this bank in storm mode?
// poll_only:		Bank does not support CMCI, skip storm tracking.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct storm_bank {
    pub history: u64,
    pub timestamp: u64,
    pub in_storm_mode: bool,
    pub poll_only: bool,
}

// How many errors within the history buffer mark the start of a storm.
pub const STORM_BEGIN_THRESHOLD: c_int = 5;
//
// How many polls of machine check bank without an error before declaring
// the storm is over. Since it is tracked by the bitmasks in the history
// field of struct storm_bank the mask is 30 bits [0 ... 29].
//
pub const STORM_END_POLL_THRESHOLD: c_int = 29;
//
// banks:		per-cpu, per-bank details
// stormy_bank_count:	count of MC banks in storm state
// poll_mode:		CPU is in poll mode
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mca_storm_desc {
    pub banks: [storm_bank; MAX_NR_BANKS],
    pub stormy_bank_count: u8,
    pub poll_mode: bool,
}

extern "C" {
    pub fn apei_write_mce(m: *mut mce) -> c_int;
}
extern "C" {
    pub fn apei_read_mce(m: *mut mce, record_id: *mut u64) -> isize;
}
extern "C" {
    pub fn apei_check_mce() -> c_int;
}
extern "C" {
    pub fn apei_clear_mce(record_id: u64) -> c_int;
}

//
// We consider records to be equivalent if bank+status+addr+misc all match.
// This is only used when the system is going down because of a fatal error
// to avoid cluttering the console log with essentially repeated information.
// In normal processing all errors seen are logged.
//

extern "C" {
    pub fn mce_work_trigger();
}
extern "C" {
    pub fn mce_register_injector_chain(nb: *mut notifier_block);
}
extern "C" {
    pub fn mce_unregister_injector_chain(nb: *mut notifier_block);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mca_config {
// Proper #MC exception handler is set
    pub 58: __reserved :,
    pub dont_log_ce: bool,
    pub cmci_disabled: bool,
    pub ignore_ce: bool,
    pub print_all: bool,
    pub monarch_timeout: c_int,
    pub panic_timeout: c_int,
    pub rip_msr: u32,
    pub bootlog: i8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_vendor_flags {
//
// Indicates that overflow conditions are not fatal, when set.
//
// (AMD) SUCCOR stands for S/W UnCorrectable error COntainment and
// Recovery. It indicates support for data poisoning in HW and deferred
// error interrupts.
//
// (AMD) SMCA: This bit indicates support for Scalable MCA which expands
// the register space for each MCA bank and also increases number of
// banks. Also, to accommodate the new banks and registers, the MCA
// register space is moved to a new MSR range.
//
// Zen IFU quirk
// AMD-style error thresholding banks present.
// Pentium, family 5-style MCA
// Centaur Winchip C6-style MCA
// SandyBridge IFU quirk
// Skylake, Cascade Lake, Cooper Lake REP;MOVS* quirk
    pub 55: __reserved_0 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mce_bank {
// subevents to enable
    pub ctl: u64,
// initialise bank?
//
// (AMD) MCA_CONFIG[McaLsbInStatusSupported]: When set, this bit indicates
// the LSB field is found in MCA_STATUS and not in MCA_ADDR.
//
    pub 62: __reserved_1 :,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mca_msr {
    MCA_CTL,
    MCA_STATUS,
    MCA_ADDR,
    MCA_MISC,
}

// Decide whether to add MCE record to MCE event pool or filter it out.
extern "C" {
    pub fn filter_mce(m: *mut mce) -> bool;
}
extern "C" {
    pub fn mce_prep_record_common(m: *mut mce);
}
extern "C" {
    pub fn mce_prep_record_per_cpu(cpu: c_uint, m: *mut mce);
}

extern "C" {
    pub fn mce_threshold_create_device(cpu: c_uint);
}
extern "C" {
    pub fn mce_threshold_remove_device(cpu: c_uint);
}
extern "C" {
    pub fn mce_amd_handle_storm(bank: c_uint, on: bool);
}
extern "C" {
    pub fn amd_filter_mce(m: *mut mce) -> bool;
}
extern "C" {
    pub fn amd_mce_usable_address(m: *mut mce) -> bool;
}
extern "C" {
    pub fn amd_clear_bank(m: *mut mce);
}
//
// If MCA_CONFIG[McaLsbInStatusSupported] is set, extract ErrAddr in bits
// [56:0] of MCA_STATUS, else in bits [55:0] of MCA_ADDR.
//
extern "C" {
    pub fn smca_bsp_init();
}

extern "C" {
    pub fn intel_p5_mcheck_init(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn winchip_mcheck_init(c: *mut cpuinfo_x86);
}
extern "C" {
    pub fn pentium_machine_check(regs: *mut pt_regs) -> noinstr void;
}
extern "C" {
    pub fn winchip_machine_check(regs: *mut pt_regs) -> noinstr void;
}

extern "C" {
    pub fn mce_rdmsrq(msr: u32) -> noinstr u64;
}
extern "C" {
    pub fn mce_wrmsrq(msr: u32, v: u64) -> noinstr void;
}
extern "C" {
    pub fn void(_arg: *mut mc_poll_banks)(void) -> extern;
}
