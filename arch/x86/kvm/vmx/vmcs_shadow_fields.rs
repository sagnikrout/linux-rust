//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/vmx/vmcs_shadow_fields.h
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


//
// We do NOT shadow fields that are modified when L0
// traps and emulates any vmx instruction (e.g. VMPTRLD,
// VMXON...) executed by L1.
// For example, VM_INSTRUCTION_ERROR is read
// by L1 if a vmx instruction fails (part of the error path).
// Note the code assumes this logic. If for some reason
// we start shadowing these fields then we need to
// force a shadow sync when L0 emulates vmx instructions
// (e.g. force a sync if VM_INSTRUCTION_ERROR is modified
// by nested_vmx_failValid)
//
// When adding or removing fields here, note that shadowed
// fields must always be synced by prepare_vmcs02, not just
// prepare_vmcs02_rare.
//
// Keeping the fields ordered by size is an attempt at improving
// branch prediction in vmcs12_read_any and vmcs12_write_any.
//
// 16-bits
// 32-bits
// Natural width
// 64-bit

