//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/trace/rv/monitors/snep/snep.h
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
// Automatically generated C representation of snep automaton
// For further information about this format, see kernel documentation:
// Documentation/trace/rv/deterministic_automata.rst
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum states_snep {
    non_scheduling_context_snep,
    scheduling_contex_snep,
    state_max_snep,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum events_snep {
    preempt_disable_snep,
    preempt_enable_snep,
    schedule_entry_snep,
    schedule_exit_snep,
    event_max_snep,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct automaton_snep {
    pub state_names: [*mut c_char; state_max_snep],
    pub event_names: [*mut c_char; event_max_snep],
    pub function: [c_uchar; state_max_snep][event_max_snep],
    pub initial_state: c_uchar,
    pub final_states: [bool; state_max_snep],
}
