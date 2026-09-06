//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sched/isolation.h
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


#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hk_type {
// Inverse of boot-time isolcpus= argument
    HK_TYPE_DOMAIN_BOOT,
//
// Same as HK_TYPE_DOMAIN_BOOT but also includes the
// inverse of cpuset isolated partitions. As such it
// is always a subset of HK_TYPE_DOMAIN_BOOT.
//
    HK_TYPE_DOMAIN,
// Inverse of boot-time isolcpus=managed_irq argument
    HK_TYPE_MANAGED_IRQ,
// Inverse of boot-time nohz_full= or isolcpus=nohz arguments
    HK_TYPE_KERNEL_NOISE,
    HK_TYPE_MAX,

//
// HK_TYPE_KTHREAD is now an alias of HK_TYPE_DOMAIN
//
    HK_TYPE_KTHREAD = HK_TYPE_DOMAIN,

//
// The following housekeeping types are only set by the nohz_full
// boot commandline option. So they can share the same value.
//
    HK_TYPE_TICK    = HK_TYPE_KERNEL_NOISE,
    HK_TYPE_TIMER   = HK_TYPE_KERNEL_NOISE,
    HK_TYPE_RCU     = HK_TYPE_KERNEL_NOISE,
    HK_TYPE_MISC    = HK_TYPE_KERNEL_NOISE,
    HK_TYPE_WQ      = HK_TYPE_KERNEL_NOISE,
}

extern "C" {
    pub fn housekeeping_any_cpu(type: hk_type) -> c_int;
}
extern "C" {
    pub fn housekeeping_enabled(type: hk_type) -> bool;
}
extern "C" {
    pub fn housekeeping_affine(t: *mut task_struct, type: hk_type);
}
extern "C" {
    pub fn housekeeping_test_cpu(cpu: c_int, type: hk_type) -> bool;
}
extern "C" {
    pub fn housekeeping_update(isol_mask: *mut cpumask) -> c_int;
}
extern "C" {
    pub fn housekeeping_init() -> void __init;
}

extern "C" {
    pub fn smp_processor_id() -> return;
}

extern "C" {
    pub fn housekeeping_test_cpu(_arg: cpu, _arg: type) -> return;
}

