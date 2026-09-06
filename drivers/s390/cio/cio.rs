//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/s390/cio/cio.h
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
// path management control word
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pmcw {
    pub /: *mut *mut u32 intparm; / interruption parameter,
    pub /: *mut *mut u32 qf : 1; / qdio facility,
    pub 1: u32 w :,
    pub /: *mut *mut u32 isc : 3; / interruption subclass,
    pub /: *mut *mut u32 res5 : 3; / reserved zeros,
    pub /: *mut *mut u32 ena : 1; / enabled,
    pub /: *mut *mut u32 lm : 2; / limit mode,
    pub /: *mut *mut u32 mme : 2; / measurement-mode enable,
    pub /: *mut *mut u32 mp : 1; / multipath mode,
    pub /: *mut *mut u32 tf : 1; / timing facility,
    pub /: *mut *mut u32 dnv : 1; / device number valid,
    pub /: *mut *mut u32 dev : 16; / device number,
    pub /: *mut *mut u8 lpm; / logical path mask,
    pub /: *mut *mut u8 pnom; / path not operational mask,
    pub /: *mut *mut u8 lpum; / last path used mask,
    pub /: *mut *mut u8 pim; / path installed mask,
    pub /: *mut *mut u16 mbi; / measurement-block index,
    pub /: *mut *mut u8 pom; / path operational mask,
    pub /: *mut *mut u8 pam; / path available mask,
    pub /: *mut *mut u8 chpid[8]; / CHPID 0-7 (if available),
    pub /: *mut *mut u32 unused1 : 8; / reserved zeros,
    pub /: *mut *mut u32 st : 3; / subchannel type,
    pub /: *mut *mut u32 unused2 : 18; / reserved zeros,
    pub /: *mut *mut u32 mbfc : 1; / measurement block format control,
    pub /: *mut *mut u32 xmwme : 1; / extended measurement word mode enable,
    pub ...*/: *mut *mut u32 csense : 1; / concurrent sense; can be enabled,
// ... per MSCH, however, if facility
// ... is not installed, this results
// ... in an operand exception.
// C attribute field omitted
// Target SCHIB configuration.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct schib_config {
    pub mba: u64,
    pub intparm: u32,
    pub mbi: u16,
    pub isc:3: u32,
    pub ena:1: u32,
    pub mme:2: u32,
    pub mp:1: u32,
    pub csense:1: u32,
    pub mbfc:1: u32,
// C attribute field omitted
//
// subchannel information block
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct schib {
    pub /: *mut *mut pmcw pmcw; / path management control word,
    pub /: *mut *mut scsw scsw; / subchannel status word,
    pub /: *mut *mut __u64 mba; / measurement block address,
    pub /: *mut *mut __u8 mda[4]; / model dependent area,
    pub ((packed,aligned(4))): } __attribute__,
//
// When rescheduled, todo's with higher values will overwrite those
// with lower values.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sch_todo {
    SCH_TODO_NOTHING,
    SCH_TODO_EVAL,
    SCH_TODO_UNREG,
}

// subchannel data structure used by I/O subroutines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subchannel {
    pub schid: subchannel_id,
    pub /: *mut *mut spinlock_t lock; / subchannel lock,
    pub reg_mutex: mutex,
    pub /: *mut *mut } st; / subchannel type,
    pub /: *mut *mut __u8 vpm; / verified path mask,
    pub /: *mut *mut __u8 lpm; / logical path mask,
    pub /: *mut *mut __u8 opm; / operational path mask,
    pub /: *mut *mut schib schib; / subchannel information block,
    pub /: *mut *mut int isc; / desired interruption subclass,
    pub /: *mut *mut chsc_ssd_info ssd_info; / subchannel description,
    pub /: *mut *mut device dev; / entry in device tree,
    pub driver: *mut css_driver,
    pub todo: sch_todo,
    pub todo_work: work_struct,
    pub config: schib_config,
    pub dma_mask: u64,
// C attribute field omitted
    pub cio_irb): DECLARE_PER_CPU_ALIGNED(struct irb,,

    pub u32): *mut *mut extern int cio_enable_subchannel(struct subchannel ,,
    pub ): *mut extern int cio_disable_subchannel (struct subchannel,
    pub ): *mut extern int cio_cancel (struct subchannel,
    pub ): *mut extern int cio_clear (struct subchannel,
    pub ): *mut *mut extern int cio_cancel_halt_clear(struct subchannel , int,
    pub ): *mut extern int cio_resume (struct subchannel,
    pub ): *mut extern int cio_halt (struct subchannel,
    pub __u8): *mut *mut *mut extern int cio_start (struct subchannel , struct ccw1 ,,
    pub __u8): *mut *mut *mut extern int cio_start_key (struct subchannel , struct ccw1 , __u8,,
    pub int): *mut *mut extern int cio_set_options (struct subchannel ,,
    pub sch): *mut extern int cio_update_schib(struct subchannel,
    pub sch): *mut extern int cio_commit_config(struct subchannel,
    pub key): *mut *mut *mut int cio_tm_start_key(struct subchannel sch, struct tcw tcw, u8 lpm, u8,
    pub sch): *mut int cio_tm_intrg(struct subchannel,
    pub airq_init(void): extern int __init,
// Use with care.

    pub cio_probe_console(void): *mut extern struct subchannel,
    pub subchannel_id): extern int cio_is_console(struct,
    pub cio_register_early_subchannels(void): extern void,
    pub sch): *mut extern void cio_tsch(struct subchannel,

pub const cio_is_console(schid): c_int = 0;

