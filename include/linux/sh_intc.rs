//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sh_intc.h
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

pub const INTC_NR_IRQS: c_int = 512;

pub const INTC_NR_IRQS: c_int = 1024;

//
// Convert back and forth between INTEVT and IRQ values.
//

pub type intc_enum = c_uchar;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_vect {
    pub enum_id: intc_enum,
    pub vect: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_group {
    pub enum_id: intc_enum,
    pub enum_ids: [intc_enum; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_subgroup {
    pub reg_width: unsigned long reg,,
    pub parent_id: intc_enum,
    pub enum_ids: [intc_enum; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_mask_reg {
    pub reg_width: unsigned long set_reg, clr_reg,,
    pub enum_ids: [intc_enum; 32],
    pub dist_reg: c_ulong,

    pub smp: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_prio_reg {
    pub field_width: unsigned long set_reg, clr_reg, reg_width,,
    pub enum_ids: [intc_enum; 16],
    pub smp: c_ulong,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_sense_reg {
    pub field_width: unsigned long reg, reg_width,,
    pub enum_ids: [intc_enum; 16],
}

// Macro flag: #define INTC_SMP_BALANCING(reg)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_hw_desc {
    pub vectors: *mut intc_vect,
    pub nr_vectors: c_uint,
    pub groups: *mut intc_group,
    pub nr_groups: c_uint,
    pub mask_regs: *mut intc_mask_reg,
    pub nr_mask_regs: c_uint,
    pub prio_regs: *mut intc_prio_reg,
    pub nr_prio_regs: c_uint,
    pub sense_regs: *mut intc_sense_reg,
    pub nr_sense_regs: c_uint,
    pub ack_regs: *mut intc_mask_reg,
    pub nr_ack_regs: c_uint,
    pub subgroups: *mut intc_subgroup,
    pub nr_subgroups: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct intc_desc {
    pub name: *mut c_char,
    pub resource: *mut resource,
    pub num_resources: c_uint,
    pub force_enable: intc_enum,
    pub force_disable: intc_enum,
    pub skip_syscore_suspend: bool,
    pub hw: intc_hw_desc,
}

extern "C" {
    pub fn register_intc_controller(desc: *mut intc_desc) -> c_int;
}
extern "C" {
    pub fn intc_set_priority(irq: c_uint, prio: c_uint) -> c_int;
}
extern "C" {
    pub fn intc_irq_lookup(chipname: *const c_char, enum_id: intc_enum) -> c_int;
}
extern "C" {
    pub fn intc_finalize();
}

extern "C" {
    pub fn register_intc_userimask(addr: c_ulong) -> c_int;
}

