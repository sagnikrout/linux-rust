//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/mpic.h
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
// Global registers
//
pub const MPIC_GREG_BASE: c_uint = 0x01000;
pub const MPIC_GREG_FEATURE_0: c_uint = 0x00000;
pub const MPIC_GREG_FEATURE_LAST_SRC_MASK: c_uint = 0x07ff0000;
pub const MPIC_GREG_FEATURE_LAST_SRC_SHIFT: c_int = 16;
pub const MPIC_GREG_FEATURE_LAST_CPU_MASK: c_uint = 0x00001f00;
pub const MPIC_GREG_FEATURE_LAST_CPU_SHIFT: c_int = 8;
pub const MPIC_GREG_FEATURE_VERSION_MASK: c_uint = 0xff;
pub const MPIC_GREG_FEATURE_1: c_uint = 0x00010;
pub const MPIC_GREG_GLOBAL_CONF_0: c_uint = 0x00020;
pub const MPIC_GREG_GCONF_RESET: c_uint = 0x80000000;
// On the FSL mpic implementations the Mode field is expand to be
// 2 bits wide:
// 0b00 = pass through (interrupts routed to IRQ0)
// 0b01 = Mixed mode
// 0b10 = reserved
// 0b11 = External proxy / coreint
//
pub const MPIC_GREG_GCONF_COREINT: c_uint = 0x60000000;
pub const MPIC_GREG_GCONF_8259_PTHROU_DIS: c_uint = 0x20000000;
pub const MPIC_GREG_GCONF_NO_BIAS: c_uint = 0x10000000;
pub const MPIC_GREG_GCONF_BASE_MASK: c_uint = 0x000fffff;
pub const MPIC_GREG_GCONF_MCK: c_uint = 0x08000000;
pub const MPIC_GREG_GLOBAL_CONF_1: c_uint = 0x00030;
pub const MPIC_GREG_VENDOR_0: c_uint = 0x00040;
pub const MPIC_GREG_VENDOR_1: c_uint = 0x00050;
pub const MPIC_GREG_VENDOR_2: c_uint = 0x00060;
pub const MPIC_GREG_VENDOR_3: c_uint = 0x00070;
pub const MPIC_GREG_VENDOR_ID: c_uint = 0x00080;
pub const MPIC_GREG_VENDOR_ID_STEPPING_MASK: c_uint = 0x00ff0000;
pub const MPIC_GREG_VENDOR_ID_STEPPING_SHIFT: c_int = 16;
pub const MPIC_GREG_VENDOR_ID_DEVICE_ID_MASK: c_uint = 0x0000ff00;
pub const MPIC_GREG_VENDOR_ID_DEVICE_ID_SHIFT: c_int = 8;
pub const MPIC_GREG_VENDOR_ID_VENDOR_ID_MASK: c_uint = 0x000000ff;
pub const MPIC_GREG_PROCESSOR_INIT: c_uint = 0x00090;
pub const MPIC_GREG_IPI_VECTOR_PRI_0: c_uint = 0x000a0;
pub const MPIC_GREG_IPI_VECTOR_PRI_1: c_uint = 0x000b0;
pub const MPIC_GREG_IPI_VECTOR_PRI_2: c_uint = 0x000c0;
pub const MPIC_GREG_IPI_VECTOR_PRI_3: c_uint = 0x000d0;
pub const MPIC_GREG_IPI_STRIDE: c_uint = 0x10;
pub const MPIC_GREG_SPURIOUS: c_uint = 0x000e0;
pub const MPIC_GREG_TIMER_FREQ: c_uint = 0x000f0;
//
// Timer registers
//
pub const MPIC_TIMER_BASE: c_uint = 0x01100;
pub const MPIC_TIMER_STRIDE: c_uint = 0x40;
pub const MPIC_TIMER_GROUP_STRIDE: c_uint = 0x1000;
pub const MPIC_TIMER_CURRENT_CNT: c_uint = 0x00000;
pub const MPIC_TIMER_BASE_CNT: c_uint = 0x00010;
pub const MPIC_TIMER_VECTOR_PRI: c_uint = 0x00020;
pub const MPIC_TIMER_DESTINATION: c_uint = 0x00030;
//
// Per-Processor registers
//
pub const MPIC_CPU_THISBASE: c_uint = 0x00000;
pub const MPIC_CPU_BASE: c_uint = 0x20000;
pub const MPIC_CPU_STRIDE: c_uint = 0x01000;
pub const MPIC_CPU_IPI_DISPATCH_0: c_uint = 0x00040;
pub const MPIC_CPU_IPI_DISPATCH_1: c_uint = 0x00050;
pub const MPIC_CPU_IPI_DISPATCH_2: c_uint = 0x00060;
pub const MPIC_CPU_IPI_DISPATCH_3: c_uint = 0x00070;
pub const MPIC_CPU_IPI_DISPATCH_STRIDE: c_uint = 0x00010;
pub const MPIC_CPU_CURRENT_TASK_PRI: c_uint = 0x00080;
pub const MPIC_CPU_TASKPRI_MASK: c_uint = 0x0000000f;
pub const MPIC_CPU_WHOAMI: c_uint = 0x00090;
pub const MPIC_CPU_WHOAMI_MASK: c_uint = 0x0000001f;
pub const MPIC_CPU_INTACK: c_uint = 0x000a0;
pub const MPIC_CPU_EOI: c_uint = 0x000b0;
pub const MPIC_CPU_MCACK: c_uint = 0x000c0;
//
// Per-source registers
//
pub const MPIC_IRQ_BASE: c_uint = 0x10000;
pub const MPIC_IRQ_STRIDE: c_uint = 0x00020;
pub const MPIC_IRQ_VECTOR_PRI: c_uint = 0x00000;
pub const MPIC_VECPRI_MASK: c_uint = 0x80000000;
pub const MPIC_VECPRI_ACTIVITY: c_uint = 0x40000000	/* Read Only */;
pub const MPIC_VECPRI_PRIORITY_MASK: c_uint = 0x000f0000;
pub const MPIC_VECPRI_PRIORITY_SHIFT: c_int = 16;
pub const MPIC_VECPRI_VECTOR_MASK: c_uint = 0x000007ff;
pub const MPIC_VECPRI_POLARITY_POSITIVE: c_uint = 0x00800000;
pub const MPIC_VECPRI_POLARITY_NEGATIVE: c_uint = 0x00000000;
pub const MPIC_VECPRI_POLARITY_MASK: c_uint = 0x00800000;
pub const MPIC_VECPRI_SENSE_LEVEL: c_uint = 0x00400000;
pub const MPIC_VECPRI_SENSE_EDGE: c_uint = 0x00000000;
pub const MPIC_VECPRI_SENSE_MASK: c_uint = 0x00400000;
pub const MPIC_IRQ_DESTINATION: c_uint = 0x00010;
pub const MPIC_FSL_BRR1: c_uint = 0x00000;
pub const MPIC_FSL_BRR1_VER: c_uint = 0x0000ffff;
pub const MPIC_MAX_IRQ_SOURCES: c_int = 2048;
pub const MPIC_MAX_CPUS: c_int = 32;
pub const MPIC_MAX_ISU: c_int = 32;
pub const MPIC_MAX_ERR: c_int = 32;
pub const MPIC_FSL_ERR_INT: c_int = 16;
//
// Tsi108 implementation of MPIC has many differences from the original one
//
// Global registers
//
pub const TSI108_GREG_BASE: c_uint = 0x00000;
pub const TSI108_GREG_FEATURE_0: c_uint = 0x00000;
pub const TSI108_GREG_GLOBAL_CONF_0: c_uint = 0x00004;
pub const TSI108_GREG_VENDOR_ID: c_uint = 0x0000c;
pub const TSI108_GREG_IPI_VECTOR_PRI_0: c_uint = 0x00204		/* Doorbell 0 */;
pub const TSI108_GREG_IPI_STRIDE: c_uint = 0x0c;
pub const TSI108_GREG_SPURIOUS: c_uint = 0x00010;
pub const TSI108_GREG_TIMER_FREQ: c_uint = 0x00014;
//
// Timer registers
//
pub const TSI108_TIMER_BASE: c_uint = 0x0030;
pub const TSI108_TIMER_STRIDE: c_uint = 0x10;
pub const TSI108_TIMER_CURRENT_CNT: c_uint = 0x00000;
pub const TSI108_TIMER_BASE_CNT: c_uint = 0x00004;
pub const TSI108_TIMER_VECTOR_PRI: c_uint = 0x00008;
pub const TSI108_TIMER_DESTINATION: c_uint = 0x0000c;
//
// Per-Processor registers
//
pub const TSI108_CPU_BASE: c_uint = 0x00300;
pub const TSI108_CPU_STRIDE: c_uint = 0x00040;
pub const TSI108_CPU_IPI_DISPATCH_0: c_uint = 0x00200;
pub const TSI108_CPU_IPI_DISPATCH_STRIDE: c_uint = 0x00000;
pub const TSI108_CPU_CURRENT_TASK_PRI: c_uint = 0x00000;
pub const TSI108_CPU_WHOAMI: c_uint = 0xffffffff;
pub const TSI108_CPU_INTACK: c_uint = 0x00004;
pub const TSI108_CPU_EOI: c_uint = 0x00008;
pub const TSI108_CPU_MCACK: c_uint = 0x00004 /* Doesn't really exist here */;
//
// Per-source registers
//
pub const TSI108_IRQ_BASE: c_uint = 0x00100;
pub const TSI108_IRQ_STRIDE: c_uint = 0x00008;
pub const TSI108_IRQ_VECTOR_PRI: c_uint = 0x00000;
pub const TSI108_VECPRI_VECTOR_MASK: c_uint = 0x000000ff;
pub const TSI108_VECPRI_POLARITY_POSITIVE: c_uint = 0x01000000;
pub const TSI108_VECPRI_POLARITY_NEGATIVE: c_uint = 0x00000000;
pub const TSI108_VECPRI_SENSE_LEVEL: c_uint = 0x02000000;
pub const TSI108_VECPRI_SENSE_EDGE: c_uint = 0x00000000;
pub const TSI108_VECPRI_POLARITY_MASK: c_uint = 0x01000000;
pub const TSI108_VECPRI_SENSE_MASK: c_uint = 0x02000000;
pub const TSI108_IRQ_DESTINATION: c_uint = 0x00004;
// weird mpic register indices and mask bits in the HW info array

// Fixup table entry

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mpic_reg_type {
    mpic_access_mmio_le,
    mpic_access_mmio_be,

    mpic_access_dcr

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpic_reg_bank {
    pub base: *mut u32 __iomem,

    pub dhost: dcr_host_t,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mpic_irq_save {

    pub fixup_data: u32,

}

// The instance data of a given MPIC
// The OpenFirmware dt node for this MPIC
// The remapper for this MPIC
// The "linux" controller struct

// Flags
// How many irq sources in a given ISU
// Number of sources
// vector numbers used for internal sources (ipi/timers)
// vector numbers used for FSL MPIC error interrupts
// Spurious vector to program into unused sources

// The fixup table

// Register access method
// The physical base address of the MPIC
// The various ioremap'ed bases
// ioremap'ed base for error interrupt registers
// Protected sources

// Pointer to HW info array

// link

//
// MPIC flags (passed to mpic_alloc)
//
// The top 4 bits contain an MPIC bhw id that is used to index the
// register offsets and some masks when CONFIG_MPIC_WEIRD is set.
// Note setting any ID (leaving those bits to 0) means standard MPIC
//
// This is a secondary ("chained") controller; it only uses the CPU0
// registers.  Primary controllers have IPIs and affinity control.
//
pub const MPIC_SECONDARY: c_uint = 0x00000001;
// Set this for a big-endian MPIC
pub const MPIC_BIG_ENDIAN: c_uint = 0x00000002;
// Broken U3 MPIC
pub const MPIC_U3_HT_IRQS: c_uint = 0x00000004;
// Broken IPI registers (autodetected)
pub const MPIC_BROKEN_IPI: c_uint = 0x00000008;
// Spurious vector requires EOI
pub const MPIC_SPV_EOI: c_uint = 0x00000020;
// No passthrough disable
pub const MPIC_NO_PTHROU_DIS: c_uint = 0x00000040;
// DCR based MPIC
pub const MPIC_USES_DCR: c_uint = 0x00000080;
// MPIC has 11-bit vector fields (or larger)
pub const MPIC_LARGE_VECTORS: c_uint = 0x00000100;
// Enable delivery of prio 15 interrupts as MCK instead of EE
pub const MPIC_ENABLE_MCK: c_uint = 0x00000200;
// Disable bias among target selection, spread interrupts evenly
pub const MPIC_NO_BIAS: c_uint = 0x00000400;
// Destination only supports a single CPU at a time
pub const MPIC_SINGLE_DEST_CPU: c_uint = 0x00001000;
// Enable CoreInt delivery of interrupts
pub const MPIC_ENABLE_COREINT: c_uint = 0x00002000;
// Do not reset the MPIC during initialization
pub const MPIC_NO_RESET: c_uint = 0x00004000;
// Freescale MPIC (compatible includes "fsl,mpic")
pub const MPIC_FSL: c_uint = 0x00008000;
// Freescale MPIC supports EIMR (error interrupt mask register).
// This flag is set for MPIC version >= 4.1 (version determined
// from the BRR1 register).
//
pub const MPIC_FSL_HAS_EIMR: c_uint = 0x00010000;
// MPIC HW modification ID
pub const MPIC_REGSET_MASK: c_uint = 0xf0000000;

// Get the version of primary MPIC

extern "C" {
    pub fn fsl_mpic_primary_get_version() -> u32;
}

// Allocate the controller structure and setup the linux irq descs
// for the range if interrupts passed in. No HW initialization is
// actually performed.
//
// @phys_addr:	physial base address of the MPIC
// @flags:	flags, see constants above
// @isu_size:	number of interrupts in an ISU. Use 0 to use a
// standard ISU-less setup (aka powermac)
// @irq_offset: first irq number to assign to this mpic
// @irq_count:  number of irqs to use with this mpic IRQ sources. Pass 0
// to match the number of sources
// @ipi_offset: first irq number to assign to this mpic IPI sources,
// used only on primary mpic
// @senses:	array of sense values
// @senses_num: number of entries in the array
//
// Note about the sense array. If none is passed, all interrupts are
// setup to be level negative unless MPIC_U3_HT_IRQS is set in which
// case they are edge positive (and the array is ignored anyway).
// The values in the array start at the first source of the MPIC,
// that is senses[0] correspond to linux irq "irq_offset".
//
// Assign ISUs, to call before mpic_init()
//
// @mpic:	controller structure as returned by mpic_alloc()
// @isu_num:	ISU number
// @phys_addr:	physical address of the ISU
//
// Initialize the controller. After this has been called, none of the above
// should be called again for this mpic
//
extern "C" {
    pub fn mpic_init(mpic: *mut mpic);
}
//
// All of the following functions must only be used after the
// ISUs have been assigned and the controller fully initialized
// with mpic_init()
//
// Change the priority of an interrupt. Default is 8 for irqs and
// 10 for IPIs. You can call this on both IPIs and IRQ numbers, but the
// IPI number is then the offset'ed (linux irq number mapped to the IPI)
//
extern "C" {
    pub fn mpic_irq_set_priority(irq: c_uint, pri: c_uint);
}
// Setup a non-boot CPU
extern "C" {
    pub fn mpic_setup_this_cpu();
}
// Clean up for kexec (or cpu offline or ...)
extern "C" {
    pub fn mpic_teardown_this_cpu(secondary: c_int);
}
// Get the current cpu priority for this cpu (0..15)
extern "C" {
    pub fn mpic_cpu_get_priority() -> c_int;
}
// Set the current cpu priority for this cpu
extern "C" {
    pub fn mpic_cpu_set_priority(prio: c_int);
}
// Request IPIs on primary mpic
extern "C" {
    pub fn mpic_request_ipis() -> void __init;
}
// Send a message (IPI) to a given target (cpu number or MSG_*)
extern "C" {
    pub fn smp_mpic_message_pass(target: c_int, msg: c_int);
}
// Unmask a specific virq
extern "C" {
    pub fn mpic_unmask_irq(d: *mut irq_data);
}
// Mask a specific virq
extern "C" {
    pub fn mpic_mask_irq(d: *mut irq_data);
}
// EOI a specific virq
extern "C" {
    pub fn mpic_end_irq(d: *mut irq_data);
}
// Fetch interrupt from a given mpic
extern "C" {
    pub fn mpic_get_one_irq(mpic: *mut mpic) -> c_uint;
}
// This one gets from the primary mpic
extern "C" {
    pub fn mpic_get_irq() -> c_uint;
}
// This one gets from the primary mpic via CoreInt
extern "C" {
    pub fn mpic_get_coreint_irq() -> c_uint;
}
// Fetch Machine Check interrupt from primary mpic
extern "C" {
    pub fn mpic_get_mcirq() -> c_uint;
}

