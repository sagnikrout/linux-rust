//! Automatically rewritten from C Header to Rust Module
//! Source: tools/verification/rvgen/tests/golden/ha_percpu/ha_percpu.h
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
// Automatically generated C representation of ha_percpu automaton
// For further information about this format, see kernel documentation:
// Documentation/trace/rv/deterministic_automata.rst
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum states_ha_percpu {
    S0_ha_percpu,
    S1_ha_percpu,
    S2_ha_percpu,
    S3_ha_percpu,
    state_max_ha_percpu,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum events_ha_percpu {
    event0_ha_percpu,
    event1_ha_percpu,
    event2_ha_percpu,
    event_max_ha_percpu,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum envs_ha_percpu {
    clk_ha_percpu,
    env1_ha_percpu,
    env2_ha_percpu,
    env_max_ha_percpu,
    env_max_stored_ha_percpu = env1_ha_percpu,
}

// Macro flag: #define HA_CLK_NS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct automaton_ha_percpu {
    pub state_names: [*mut c_char; state_max_ha_percpu],
    pub event_names: [*mut c_char; event_max_ha_percpu],
    pub env_names: [*mut c_char; env_max_ha_percpu],
    pub function: [c_uchar; state_max_ha_percpu][event_max_ha_percpu],
    pub initial_state: c_uchar,
    pub final_states: [bool; state_max_ha_percpu],
}
