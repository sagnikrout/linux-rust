//! Automatically rewritten from C Header to Rust Module
//! Source: include/xen/interface/hvm/params.h
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


// SPDX-License-Identifier: MIT

//
// Parameter space for HVMOP_{set,get}_param.
//
pub const HVM_PARAM_CALLBACK_IRQ: c_int = 0;
//
// How should CPU0 event-channel notifications be delivered?
//
// If val == 0 then CPU0 event-channel notifications are not delivered.
// If val != 0, val[63:56] encodes the type, as follows:
//
pub const HVM_PARAM_CALLBACK_TYPE_GSI: c_int = 0;
//
// val[55:0] is a delivery GSI.  GSI 0 cannot be used, as it aliases val == 0,
// and disables all notifications.
//
pub const HVM_PARAM_CALLBACK_TYPE_PCI_INTX: c_int = 1;
//
// val[55:0] is a delivery PCI INTx line:
// Domain = val[47:32], Bus = val[31:16] DevFn = val[15:8], IntX = val[1:0]
//

pub const HVM_PARAM_CALLBACK_TYPE_VECTOR: c_int = 2;
//
// val[7:0] is a vector number.  Check for XENFEAT_hvm_callback_vector to know
// if this delivery method is available.
//

pub const HVM_PARAM_CALLBACK_TYPE_PPI: c_int = 2;
//
// val[55:16] needs to be zero.
// val[15:8] is interrupt flag of the PPI used by event-channel:
// bit 8: the PPI is edge(1) or level(0) triggered
// bit 9: the PPI is active low(1) or high(0)
// val[7:0] is a PPI number used by event-channel.
// This is only used by ARM/ARM64 and masking/eoi the interrupt associated to
// the notification is handled by the interrupt controller.
//

pub const HVM_PARAM_STORE_PFN: c_int = 1;
pub const HVM_PARAM_STORE_EVTCHN: c_int = 2;
pub const HVM_PARAM_PAE_ENABLED: c_int = 4;
pub const HVM_PARAM_IOREQ_PFN: c_int = 5;
pub const HVM_PARAM_BUFIOREQ_PFN: c_int = 6;
//
// Set mode for virtual timers (currently x86 only):
// delay_for_missed_ticks (default):
// Do not advance a vcpu's time beyond the correct delivery time for
// interrupts that have been missed due to preemption. Deliver missed
// interrupts when the vcpu is rescheduled and advance the vcpu's virtual
// time stepwise for each one.
// no_delay_for_missed_ticks:
// As above, missed interrupts are delivered, but guest time always tracks
// wallclock (i.e., real) time while doing so.
// no_missed_ticks_pending:
// No missed interrupts are held pending. Instead, to ensure ticks are
// delivered at some non-zero rate, if we detect missed ticks then the
// internal tick alarm is not disabled if the VCPU is preempted during the
// next tick period.
// one_missed_tick_pending:
// Missed interrupts are collapsed together and delivered as one 'late tick'.
// Guest time always tracks wallclock (i.e., real) time.
//
pub const HVM_PARAM_TIMER_MODE: c_int = 10;
pub const HVMPTM_delay_for_missed_ticks: c_int = 0;
pub const HVMPTM_no_delay_for_missed_ticks: c_int = 1;
pub const HVMPTM_no_missed_ticks_pending: c_int = 2;
pub const HVMPTM_one_missed_tick_pending: c_int = 3;
// Boolean: Enable virtual HPET (high-precision event timer)? (x86-only)
pub const HVM_PARAM_HPET_ENABLED: c_int = 11;
// Identity-map page directory used by Intel EPT when CR0.PG=0.
pub const HVM_PARAM_IDENT_PT: c_int = 12;
// Device Model domain, defaults to 0.
pub const HVM_PARAM_DM_DOMAIN: c_int = 13;
// ACPI S state: currently support S0 and S3 on x86.
pub const HVM_PARAM_ACPI_S_STATE: c_int = 14;
// TSS used on Intel when CR0.PE=0.
pub const HVM_PARAM_VM86_TSS: c_int = 15;
// Boolean: Enable aligning all periodic vpts to reduce interrupts
pub const HVM_PARAM_VPT_ALIGN: c_int = 16;
// Console debug shared memory ring and event channel
pub const HVM_PARAM_CONSOLE_PFN: c_int = 17;
pub const HVM_PARAM_CONSOLE_EVTCHN: c_int = 18;
pub const HVM_NR_PARAMS: c_int = 19;
