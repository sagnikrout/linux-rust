//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/irq_vectors.h
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
// Linux IRQ vector layout.
//
// There are 256 IDT entries (per CPU - each entry is 8 bytes) which can
// be defined by Linux. They are used as a jump table by the CPU when a
// given vector is triggered - by a CPU-external, CPU-internal or
// software-triggered event.
//
// Linux sets the kernel code address each entry jumps to early during
// bootup, and never changes them. This is the general layout of the
// IDT entries:
//
// Vectors   0 ...  31 : system traps and exceptions - hardcoded events
// Vectors  32 ... 127 : device interrupts
// Vector  128         : legacy int80 syscall interface
// Vectors 129 ... FIRST_SYSTEM_VECTOR-1 : device interrupts
// Vectors FIRST_SYSTEM_VECTOR ... 255   : special interrupts
//
// 64-bit x86 has per CPU IDT tables, 32-bit has one shared IDT table.
//
// This file enumerates the exact layout of them:
//
// This is used as an interrupt vector when programming the APIC.
pub const NMI_VECTOR: c_uint = 0x02;
//
// IDT vectors usable for external interrupt sources start at 0x20.
// (0x80 is the syscall vector, 0x30-0x3f are for ISA)
//
pub const FIRST_EXTERNAL_VECTOR: c_uint = 0x20;
pub const IA32_SYSCALL_VECTOR: c_uint = 0x80;
//
// Vectors 0x30-0x3f are used for ISA interrupts.
// round up to the next 16-vector boundary
//

//
// Special IRQ vectors used by the SMP architecture, 0xf0-0xff
//
// some of the following vectors are 'rare', they are merged
// into a single vector (CALL_FUNCTION_VECTOR) to save vector space.
// TLB, reschedule and local APIC vectors are performance-critical.
//
pub const SPURIOUS_APIC_VECTOR: c_uint = 0xff;
//
// Sanity check
//

pub const ERROR_APIC_VECTOR: c_uint = 0xfe;
pub const RESCHEDULE_VECTOR: c_uint = 0xfd;
pub const CALL_FUNCTION_VECTOR: c_uint = 0xfc;
pub const CALL_FUNCTION_SINGLE_VECTOR: c_uint = 0xfb;
pub const THERMAL_APIC_VECTOR: c_uint = 0xfa;
pub const THRESHOLD_APIC_VECTOR: c_uint = 0xf9;
pub const REBOOT_VECTOR: c_uint = 0xf8;
//
// Generic system vector for platform specific use
//
pub const X86_PLATFORM_IPI_VECTOR: c_uint = 0xf7;
//
// IRQ work vector:
//
pub const IRQ_WORK_VECTOR: c_uint = 0xf6;
// IRQ vector for PMIs when running a guest with a mediated PMU.
pub const PERF_GUEST_MEDIATED_PMI_VECTOR: c_uint = 0xf5;
pub const DEFERRED_ERROR_VECTOR: c_uint = 0xf4;
// Vector on which hypervisor callbacks will be delivered
pub const HYPERVISOR_CALLBACK_VECTOR: c_uint = 0xf3;
// Vector for KVM to deliver posted interrupt IPI
pub const POSTED_INTR_VECTOR: c_uint = 0xf2;
pub const POSTED_INTR_WAKEUP_VECTOR: c_uint = 0xf1;
pub const POSTED_INTR_NESTED_VECTOR: c_uint = 0xf0;
pub const MANAGED_IRQ_SHUTDOWN_VECTOR: c_uint = 0xef;

pub const HYPERV_REENLIGHTENMENT_VECTOR: c_uint = 0xee;
pub const HYPERV_STIMER0_VECTOR: c_uint = 0xed;

pub const LOCAL_TIMER_VECTOR: c_uint = 0xec;
//
// Posted interrupt notification vector for all device MSIs delivered to
// the host kernel.
//
pub const POSTED_MSI_NOTIFICATION_VECTOR: c_uint = 0xeb;
pub const NR_VECTORS: c_int = 256;

//
// Size the maximum number of interrupts.
//
// If the irq_desc[] array has a sparse layout, we can size things
// generously - it scales up linearly with the maximum number of CPUs,
// and the maximum number of IO-APICs, whichever is higher.
//
// In other cases we size more conservatively, to not create too large
// static arrays.
//
pub const NR_IRQS_LEGACY: c_int = 16;

