//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/cs46xx/cs46xx_dsp_task_types.h
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
// The driver for the Cirrus Logic's Sound Fusion CS46XX based soundcards
// Copyright (c) by Jaroslav Kysela <perex@perex.cz>
//
// NOTE: comments are copy/paste from cwcemb80.lst
// provided by Tom Woller at Cirrus (my only
// documentation about the SP OS running inside
// the DSP)
//

//
pub const HFG_FIRST_EXECUTE_MODE: c_uint = 0x0001;
pub const HFG_FIRST_EXECUTE_MODE_BIT: c_int = 0;
pub const HFG_CONTEXT_SWITCH_MODE: c_uint = 0x0002;
pub const HFG_CONTEXT_SWITCH_MODE_BIT: c_int = 1;

pub const MAX_MG_STACK_SIZE: c_int = 16;
pub const MAX_BG_STACK_SIZE: c_int = 9;
pub const MAX_HFG_STACK_SIZE: c_int = 4;

// Minimal context save area for Hyper Forground
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_hf_save_area {
    pub r10_save: u32,
    pub r54_save: u32,
    pub r98_save: u32,
    pub r32_save: u32,
    pub r76_save: u32,
    pub rsd2_save: u32,
// saved as part of HFG context
}

// Task link data structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_tree_link {
// Pointer to sibling task control block
// Pointer to child task control block
// Pointer to code entry point
// Pointer to local data
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_task_tree_data {
// Initial tock count; controls task tree execution rate
// Tock down counter
// Add to ActiveCount when TockCountLimit reached:
// Number of pending activations for task tree
// BitNumber to enable modification of correct bit in ActiveTaskFlags
// Pointer to OS location for indicating current activity on task level
// Data structure for controlling movement of memory blocks:-
// Data structure for controlling synchronous link update
// Save area for remainder of full context.
// Address of start of local stack for data storage
}

// These data items have the same relative locations to those
// used for this data in the SPOS control block for SPOS 1.0
// This structure contains extra storage for the task tree
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_task_tree_context_block {
// Up to 10 values are saved onto the stack.  8 for the task tree, 1 for
    pub saverfe: u32,
// Value may be overwritten by stack save algorithm.
    pub /: *mut *mut u32 saverba; / (HFG),
    pub saverdc: u32,
    pub /: *mut *mut u32 savers_config_23; / (HFG),
    pub /: *mut *mut u32 savers_DMA23; / (HFG),
    pub saversa0: u32,
    pub saversi0: u32,
    pub saversa1: u32,
    pub saversi1: u32,
    pub saversa3: u32,
    pub saversd0: u32,
    pub saversd1: u32,
    pub saversd3: u32,
    pub savers_config01: u32,
    pub savers_DMA01: u32,
    pub saveacc0hl: u32,
    pub saveacc1hl: u32,
    pub saveacc0xacc1x: u32,
    pub saveacc2hl: u32,
    pub saveacc3hl: u32,
    pub saveacc2xacc3x: u32,
    pub saveaux0hl: u32,
    pub saveaux1hl: u32,
    pub saveaux0xaux1x: u32,
    pub saveaux2hl: u32,
    pub saveaux3hl: u32,
    pub saveaux2xaux3x: u32,
    pub savershouthl: u32,
    pub savershoutxmacmode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dsp_task_tree_control_block {
    pub context: dsp_hf_save_area,
    pub links: dsp_tree_link,
    pub data: dsp_task_tree_data,
    pub context_blk: dsp_task_tree_context_block,
    pub int_timer: dsp_interval_timer_data,
}
