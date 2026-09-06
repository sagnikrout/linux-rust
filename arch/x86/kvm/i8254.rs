//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/i8254.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kpit_channel_state {
    pub /: *mut *mut u32 count; / can be 65536,
    pub latched_count: u16,
    pub count_latched: u8,
    pub status_latched: u8,
    pub status: u8,
    pub read_state: u8,
    pub write_state: u8,
    pub write_latch: u8,
    pub rw_mode: u8,
    pub mode: u8,
    pub /: *mut *mut u8 bcd; / not supported,
    pub /: *mut *mut u8 gate; / timer start,
    pub count_load_time: ktime_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kpit_state {
// All members before "struct mutex lock" are protected by the lock.
    pub channels: [kvm_kpit_channel_state; 3],
    pub flags: u32,
    pub is_periodic: bool,
    pub /: *mut *mut s64 period; / unit: ns,
    pub timer: hrtimer,
    pub lock: mutex,
    pub reinject: core::sync::atomic::AtomicI32,
    pub /: *mut *mut atomic_t pending; / accumulated triggered timers,
    pub irq_ack: core::sync::atomic::AtomicI32,
    pub irq_ack_notifier: kvm_irq_ack_notifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pit {
    pub dev: kvm_io_device,
    pub speaker_dev: kvm_io_device,
    pub kvm: *mut kvm,
    pub pit_state: kvm_kpit_state,
    pub mask_notifier: kvm_irq_mask_notifier,
    pub worker: *mut kthread_worker,
    pub expired: kthread_work,
}

pub const KVM_PIT_BASE_ADDRESS: c_uint = 0x40;
pub const KVM_SPEAKER_BASE_ADDRESS: c_uint = 0x61;
pub const KVM_PIT_MEM_LENGTH: c_int = 4;
pub const KVM_PIT_FREQ: c_int = 1193181;

pub const KVM_PIT_CHANNEL_MASK: c_uint = 0x3;
extern "C" {
    pub fn kvm_vm_ioctl_get_pit(kvm: *mut kvm, ps: *mut kvm_pit_state) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_set_pit(kvm: *mut kvm, ps: *mut kvm_pit_state) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_get_pit2(kvm: *mut kvm, ps: *mut kvm_pit_state2) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_set_pit2(kvm: *mut kvm, ps: *mut kvm_pit_state2) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_reinject(kvm: *mut kvm, control: *mut kvm_reinject_control) -> c_int;
}
extern "C" {
    pub fn kvm_free_pit(kvm: *mut kvm);
}

