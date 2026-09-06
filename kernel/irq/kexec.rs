//! Automatically rewritten from C to Rust
//! Source: kernel/irq/kexec.c
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

#[no_mangle]
pub unsafe extern "C" fn machine_kexec_mask_interrupts() {
    void machine_kexec_mask_interrupts(void)
    {
    struct irq_desc *desc;
    unsigned int i;
    for_each_irq_desc(i, desc) {
    struct irq_chip *chip;
    let mut check_eoi: c_int = 1;
    chip = irq_desc_get_chip(desc);
    if (!chip || !irqd_is_started(&desc.irq_data))
    continue;
    if (IS_ENABLED(CONFIG_GENERIC_IRQ_KEXEC_CLEAR_VM_FORWARD)) {
//
// First try to remove the active state from an interrupt which is forwarded
// to a VM. If the interrupt is not forwarded, try to EOI the interrupt.
//
    check_eoi = irq_set_irqchip_state(i, IRQCHIP_STATE_ACTIVE, false);
    }
    if (check_eoi && chip.irq_eoi && irqd_irq_inprogress(&desc.irq_data))
    chip.irq_eoi(&desc.irq_data);
    irq_shutdown(desc);
    }
    }
