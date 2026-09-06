//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/imc-pmu.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// IMC Nest Performance Monitor counter support.
//
// Copyright (C) 2017 Madhavan Srinivasan, IBM Corporation.
// (C) 2017 Anju T Sudhakar, IBM Corporation.
// (C) 2017 Hemant K Shaw, IBM Corporation.
//

//
// Compatibility macros for IMC devices
//

//
// LDBAR: Counter address and Enable/Disable macro.
// perf/imc-pmu.c has the LDBAR layout information.
//
pub const THREAD_IMC_LDBAR_MASK: c_uint = 0x0003ffffffffe000ULL;
pub const THREAD_IMC_ENABLE: c_uint = 0x8000000000000000ULL;
pub const TRACE_IMC_ENABLE: c_uint = 0x4000000000000000ULL;
//
// For debugfs interface for imc-mode and imc-command
//
pub const IMC_CNTL_BLK_OFFSET: c_uint = 0x3FC00;
pub const IMC_CNTL_BLK_CMD_OFFSET: c_int = 8;
pub const IMC_CNTL_BLK_MODE_OFFSET: c_int = 32;
//
// Structure to hold memory address information for imc units.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imc_mem_info {
    pub vbase: *mut u64,
    pub id: u32,
}

//
// Place holder for nest pmu events and values.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imc_events {
    pub value: u32,
    pub name: *mut c_char,
    pub unit: *mut c_char,
    pub scale: *mut c_char,
}

//
// Trace IMC hardware updates a 64bytes record on
// Core Performance Monitoring Counter (CPMC)
// overflow. Here is the layout for the trace imc record
//
// DW 0 : Timebase
// DW 1 : Program Counter
// DW 2 : PIDR information
// DW 3 : CPMC1
// DW 4 : CPMC2
// DW 5 : CPMC3
// Dw 6 : CPMC4
// DW 7 : Timebase
// .....
//
// The following is the data structure to hold trace imc data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct trace_imc_data {
    pub tb1: __be64,
    pub ip: __be64,
    pub val: __be64,
    pub cpmc1: __be64,
    pub cpmc2: __be64,
    pub cpmc3: __be64,
    pub cpmc4: __be64,
    pub tb2: __be64,
}

// Event attribute array index
pub const IMC_FORMAT_ATTR: c_int = 0;
pub const IMC_EVENT_ATTR: c_int = 1;
pub const IMC_CPUMASK_ATTR: c_int = 2;
pub const IMC_NULL_ATTR: c_int = 3;
// PMU Format attribute macros
pub const IMC_EVENT_OFFSET_MASK: c_uint = 0xffffffffULL;
//
// Macro to mask bits 0:21 of first double word(which is the timebase) to
// compare with 8th double word (timebase) of trace imc record data.
//
pub const IMC_TRACE_RECORD_TB1_MASK: c_uint = 0x3ffffffffffULL;
//
// Bit 0:1 in third DW of IMC trace record
// specifies the MSR[HV PR] values.
//

//
// Device tree parser code detects IMC pmu support and
// registers new IMC pmus. This structure will hold the
// pmu functions, events, counter memory information
// and attrs for each imc pmu and will be referenced at
// the time of pmu registration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imc_pmu {
    pub pmu: pmu,
    pub mem_info: *mut imc_mem_info,
    pub events: *mut imc_events,
//
// Attribute groups for the PMU. Slot 0 used for
// format attribute, slot 1 used for cpusmask attribute,
// slot 2 used for event attribute. Slot 3 keep as
// NULL.
//
    pub attr_groups: [*const attribute_group; 4],
    pub counter_mem_size: u32,
    pub domain: c_int,
//
// flag to notify whether the memory is mmaped
// or allocated by kernel.
//
    pub imc_counter_mmaped: bool,
}

//
// Structure to hold id, lock and reference count for the imc events which
// are inited.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imc_pmu_ref {
    pub lock: spinlock_t,
    pub id: c_uint,
    pub refc: c_int,
}

//
// In-Memory Collection Counters type.
// Data comes from Device tree.
// Three device type are supported.
//
// Domains for IMC PMUs
//
pub const IMC_DOMAIN_NEST: c_int = 1;
pub const IMC_DOMAIN_CORE: c_int = 2;
pub const IMC_DOMAIN_THREAD: c_int = 3;
// For trace-imc the domain is still thread but it operates in trace-mode
pub const IMC_DOMAIN_TRACE: c_int = 4;
extern "C" {
    pub fn thread_imc_disable();
}
extern "C" {
    pub fn get_max_nest_dev() -> c_int;
}
extern "C" {
    pub fn unregister_thread_imc();
}
