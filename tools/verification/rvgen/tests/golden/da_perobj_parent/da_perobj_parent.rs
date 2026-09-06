//! Automatically rewritten from C Header to Rust Module
//! Source: tools/verification/rvgen/tests/golden/da_perobj_parent/da_perobj_parent.h
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
// Automatically generated C representation of da_perobj_parent automaton
// For further information about this format, see kernel documentation:
// Documentation/trace/rv/deterministic_automata.rst
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum states_da_perobj_parent {
    state_a_da_perobj_parent,
    state_b_da_perobj_parent,
    state_c_da_perobj_parent,
    state_max_da_perobj_parent,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum events_da_perobj_parent {
    event_1_da_perobj_parent,
    event_2_da_perobj_parent,
    event_3_da_perobj_parent,
    event_max_da_perobj_parent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct automaton_da_perobj_parent {
    pub state_names: [*mut c_char; state_max_da_perobj_parent],
    pub event_names: [*mut c_char; event_max_da_perobj_parent],
    pub function: [c_uchar; state_max_da_perobj_parent][event_max_da_perobj_parent],
    pub initial_state: c_uchar,
    pub final_states: [bool; state_max_da_perobj_parent],
}
