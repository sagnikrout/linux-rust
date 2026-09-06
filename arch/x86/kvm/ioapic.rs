//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/ioapic.h
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

pub const IOAPIC_VERSION_ID: c_uint = 0x11	/* IOAPIC version */;
pub const IOAPIC_EDGE_TRIG: c_int = 0;
pub const IOAPIC_LEVEL_TRIG: c_int = 1;
pub const IOAPIC_DEFAULT_BASE_ADDRESS: c_uint = 0xfec00000;
pub const IOAPIC_MEM_LENGTH: c_uint = 0x100;
// Direct registers.
pub const IOAPIC_REG_SELECT: c_uint = 0x00;
pub const IOAPIC_REG_WINDOW: c_uint = 0x10;
// Indirect registers.
pub const IOAPIC_REG_APIC_ID: c_uint = 0x00	/* x86 IOAPIC only */;
pub const IOAPIC_REG_VERSION: c_uint = 0x01;
pub const IOAPIC_REG_ARB_ID: c_uint = 0x02	/* x86 IOAPIC only */;
// ioapic delivery mode
pub const IOAPIC_FIXED: c_uint = 0x0;
pub const IOAPIC_LOWEST_PRIORITY: c_uint = 0x1;
pub const IOAPIC_PMI: c_uint = 0x2;
pub const IOAPIC_NMI: c_uint = 0x4;
pub const IOAPIC_INIT: c_uint = 0x5;
pub const IOAPIC_EXTINT: c_uint = 0x7;
pub const RTC_GSI: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtc_status {
    pub pending_eoi: c_int,
// vcpu bitmap where IRQ has been sent
    pub KVM_MAX_VCPU_IDS): DECLARE_BITMAP(map,,
//
// Vector sent to a given vcpu, only valid when
// the vcpu's bit in map is set
//
    pub vectors: [u8; KVM_MAX_VCPU_IDS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union kvm_ioapic_redirect_entry {
    pub bits: u64,
    pub vector: u8,
    pub delivery_mode:3: u8,
    pub dest_mode:1: u8,
    pub delivery_status:1: u8,
    pub polarity:1: u8,
    pub remote_irr:1: u8,
    pub trig_mode:1: u8,
    pub mask:1: u8,
    pub reserve:7: u8,
    pub reserved: [u8; 4],
    pub dest_id: u8,
    pub fields: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ioapic {
    pub base_address: u64,
    pub ioregsel: u32,
    pub id: u32,
    pub irr: u32,
    pub pad: u32,
    pub redirtbl: [kvm_ioapic_redirect_entry; IOAPIC_NUM_PINS],
    pub irq_states: [c_ulong; IOAPIC_NUM_PINS],
    pub dev: kvm_io_device,
    pub kvm: *mut kvm,
    pub lock: spinlock_t,
    pub rtc_status: rtc_status,
    pub eoi_inject: delayed_work,
    pub irq_eoi: [u32; IOAPIC_NUM_PINS],
    pub irr_delivered: u32,
// reads protected by irq_srcu, writes by irq_lock
    pub mask_notifier_list: hlist_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_mask_notifier {
    pub masked): *mut *mut *mut void (func)(struct kvm_irq_mask_notifier kimn, bool,
    pub irq: c_int,
    pub link: hlist_node,
}

extern "C" {
    pub fn kvm_rtc_eoi_tracking_restore_one(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_ioapic_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_ioapic_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_get_ioapic(kvm: *mut kvm, state: *mut kvm_ioapic_state);
}
extern "C" {
    pub fn kvm_set_ioapic(kvm: *mut kvm, state: *mut kvm_ioapic_state);
}
// Logical OR for level trig interrupt

extern "C" {
    pub fn irqchip_full(_arg: kvm) -> return;
}
