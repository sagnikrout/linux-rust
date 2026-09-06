//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/kvm/irq.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// irq.h: in kernel interrupt controller related definitions
// Copyright (c) 2007, Intel Corporation.
//
// Authors:
// Yaozu (Eddie) Dong <Eddie.dong@intel.com>
//

pub const PIC_NUM_PINS: c_int = 16;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_kpic_state {
    pub /: *mut *mut u8 last_irr; / edge detection,
    pub /: *mut *mut u8 irr; / interrupt request register,
    pub /: *mut *mut u8 imr; / interrupt mask register,
    pub /: *mut *mut u8 isr; / interrupt service register,
    pub /: *mut *mut u8 priority_add; / highest irq priority,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub poll: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_auto_eoi: u8,
    pub special_fully_nested_mode: u8,
    pub /: *mut *mut u8 init4; / true if 4 byte init,
    pub /: *mut *mut u8 elcr; / PIIX edge/trigger selection,
    pub elcr_mask: u8,
    pub /: *mut *mut u8 isr_ack; / interrupt ack detection,
    pub pics_state: *mut kvm_pic,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pic {
    pub lock: spinlock_t,
    pub wakeup_needed: bool,
    pub pending_acks: unsigned,
    pub kvm: *mut kvm,
    pub /: *mut *mut kvm_kpic_state pics[2]; / 0 is master pic, 1 is slave pic,
    pub /: *mut *mut int output; / intr from master PIC,
    pub dev_master: kvm_io_device,
    pub dev_slave: kvm_io_device,
    pub dev_elcr: kvm_io_device,
    pub irq_states: [c_ulong; PIC_NUM_PINS],
}

extern "C" {
    pub fn kvm_pic_init(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_pic_destroy(kvm: *mut kvm);
}
extern "C" {
    pub fn kvm_pic_read_irq(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_pic_update_irq(s: *mut kvm_pic);
}
extern "C" {
    pub fn kvm_setup_default_ioapic_and_pic_routing(kvm: *mut kvm) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_get_irqchip(kvm: *mut kvm, chip: *mut kvm_irqchip) -> c_int;
}
extern "C" {
    pub fn kvm_vm_ioctl_set_irqchip(kvm: *mut kvm, chip: *mut kvm_irqchip) -> c_int;
}
// Matches smp_wmb() when setting irqchip_mode

extern "C" {
    pub fn irqchip_full(_arg: kvm) -> return;
}
// Matches smp_wmb() when setting irqchip_mode
extern "C" {
    pub fn kvm_cpu_has_injectable_intr(v: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_cpu_has_interrupt(vcpu: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_cpu_has_extint(v: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_cpu_get_extint(v: *mut kvm_vcpu) -> c_int;
}
extern "C" {
    pub fn kvm_cpu_get_interrupt(v: *mut kvm_vcpu) -> c_int;
}
//
// WARN if an IRQ was lost between detecting the IRQ and grabbing the
// IRQ for injection, unless it's possible the lost IRQ was due to one
// of the exceptional cases below.
//
// If the VM has an in-kernel PIC, the ExtINT handling that's routed
// through KVM's virtual PIC is tracked per-VM, not per-vCPU.  If
// another vCPU grabs the IRQ, or deasserts the interrupt (which is
// level-triggered), then it's both expected and "fine" for an IRQ
// seemingly be "lost" from this vCPU's perspective.
//
// Similarly, Xen's event channel isn't entirely within KVM's control,
// e.g. Xen emulation can be disabled entirely per-VM, or the guest
// can desassert an IRQ by writing to shared memory.
//
extern "C" {
    pub fn kvm_inject_pending_timer_irqs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_inject_apic_timer_irqs(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn kvm_apic_nmi_wd_deliver(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __kvm_migrate_apic_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __kvm_migrate_pit_timer(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn __kvm_migrate_timers(vcpu: *mut kvm_vcpu);
}
extern "C" {
    pub fn apic_has_pending_timer(vcpu: *mut kvm_vcpu) -> c_int;
}
