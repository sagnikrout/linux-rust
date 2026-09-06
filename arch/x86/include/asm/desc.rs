//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/desc.h
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

// Set the ACCESS bit so it can be mapped RO
//
// Don't allow setting of the lm bit. It would confuse
// user_64bit_mode and would get overridden by sysret anyway.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdt_page {
    pub gdt: [desc_struct; GDT_ENTRIES],
    pub __attribute__((aligned(PAGE_SIZE))): },
    pub gdt_page): DECLARE_PER_CPU_PAGE_ALIGNED(struct gdt_page,,
// Provide the original GDT
    pub cpu).gdt: return per_cpu(gdt_page,,
// Provide the current original GDT
    pub this_cpu_ptr(&gdt_page)->gdt: return,
// Provide the fixmap address of the remapped GDT
    pub )&get_cpu_entry_area(cpu)->gdt: *mut return (struct desc_struct,
// Provide the current read-only GDT
    pub get_cpu_gdt_ro(smp_processor_id()): return,
// Provide the physical address of the GDT page.
    pub per_cpu_ptr_to_phys(get_cpu_gdt_rw(cpu)): return,
    pub func: gate->offset_low = (u16),
    pub 1: gate->bits.p =,
    pub dpl: gate->bits.dpl =,
    pub 0: gate->bits.zero =,
    pub type: gate->bits.type =,
    pub 16): gate->offset_middle = (u16) (func >>,

    pub __KERNEL_CS: gate->segment =,
    pub ist: gate->bits.ist =,
    pub 0: gate->reserved =,
    pub 32): gate->offset_high = (u32) (func >>,

    pub seg: gate->segment =,
    pub 0: gate->bits.ist =,

    pub ptr: *const *const u32 desc =,
    pub desc[1]): return !(desc[0] |,

    pub sizeof(*gate)): *mut memcpy(&idt[entry], gate,,
    pub 8): memcpy(&ldt[entry], desc,,
    pub size: c_uint,
    pub break: case DESC_TSS: size = sizeof(tss_desc);,
    pub break: case DESC_LDT: size = sizeof(ldt_desc);,
    pub break: *mut *mut default: size = sizeof(gdt);,
    pub size): memcpy(&gdt[entry], desc,,
    pub d: *mut *mut ldttss_desc desc =,
    pub sizeof(*desc)): *mut memset(desc, 0,,
    pub size: desc->limit0 = (u16),
    pub addr: desc->base0 = (u16),
    pub 0xFF: desc->base1 = (addr >> 16) &,
    pub type: desc->type =,
    pub 1: desc->p =,
    pub 0xF: desc->limit1 = (size >> 16) &,
    pub 0xFF: desc->base2 = (addr >> 24) &,

    pub 32): desc->base3 = (u32) (addr >>,

    pub get_cpu_gdt_rw(cpu): *mut *mut desc_d =,
    pub tss: tss_desc,
    pub DESC_TSS): write_gdt_entry(d, entry, &tss,,

    pub (0)): asm volatile("lldt %w0"::"q",
    pub smp_processor_id(): unsigned cpu =,
    pub ldt: ldt_desc,
    pub 1): *mut *mut entries  LDT_ENTRY_SIZE -,
    pub DESC_LDT): &ldt,,
    pub (GDT_ENTRY_LDT*8)): *mut asm volatile("lldt %w0"::"q",
    pub (*dtr)): *mut asm volatile("lgdt %0"::"m",
    pub (*dtr)): *mut asm volatile("lidt %0"::"m",
    pub (*dtr)): *mut asm volatile("sgdt %0":"=m",
    pub (*dtr)): *mut asm volatile("sidt %0":"=m",
}

//
// The LTR instruction marks the TSS GDT entry as busy. On 64-bit, the GDT is
// a read-only remapping. To prevent a page fault, the GDT is switched to the
// original writeable version when needed.
//

//
// If the current GDT is the read-only fixmap, swap to the original
// writeable version. Swap back at the end.
//
extern "C" {
    pub fn volatile((GDT_ENTRY_TSS*8): *mut "ltr %w0"::"q") -> asm;
}

extern "C" {
    pub fn volatile((GDT_ENTRY_TSS*8): *mut "ltr %w0"::"q") -> asm;
}

extern "C" {
    pub fn volatile((tr): "str %0":"=r") -> asm;
}
//
// LTR requires an available TSS, and the TSS is currently
// busy.  Make it be available so that LTR will work.
//
// Call this if you need the TSS limit to be correct, which should be the case
// if and only if you have TIF_IO_BITMAP set or you're switching to a task
// with TIF_IO_BITMAP set.
//
// If you do something evil that corrupts the cached TSS limit (I'm looking
// at you, VMX exits), call this function.
//
// The optimization here is that the TSS limit only matters for Linux if the
// IO bitmap is in use.  If the TSS limit gets forced to its minimum value,
// everything works except that IO bitmap will be ignored and all CPL 3 IO
// instructions will #GP, which is exactly what we want for normal tasks.
//
// This intentionally ignores lm, since 32-bit apps don't have that field.

// Lots of programs expect an all-zero user_desc to mean "no segment at all".

extern "C" {
    pub fn load_current_idt();
}
extern "C" {
    pub fn idt_setup_early_handler();
}
extern "C" {
    pub fn idt_setup_early_traps();
}
extern "C" {
    pub fn idt_setup_traps();
}
extern "C" {
    pub fn idt_setup_apic_and_irq_gates();
}
extern "C" {
    pub fn idt_is_f00f_address(address: c_ulong) -> bool;
}
extern "C" {
    pub fn idt_do_interrupt_irqoff(address: c_ulong);
}
extern "C" {
    pub fn idt_do_nmi_irqoff();
}
extern "C" {
    pub fn idt_entry_from_kvm(vector: c_uint);
}

extern "C" {
    pub fn idt_setup_early_pf();
}

extern "C" {
    pub fn idt_invalidate();
}
