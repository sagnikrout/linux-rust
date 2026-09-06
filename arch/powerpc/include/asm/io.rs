//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/io.h
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
// Check of existence of legacy devices
extern "C" {
    pub fn check_legacy_ioport(base_port: c_ulong) -> c_int;
}
pub const I8042_DATA_REG: c_uint = 0x60;
pub const FDC_BASE: c_uint = 0x3f0;

//
// has legacy ISA devices ?
//

pub const SIO_CONFIG_RA: c_uint = 0x398;
pub const SIO_CONFIG_RD: c_uint = 0x399;
// 32 bits uses slightly different variables for the various IO
// bases. Most of this file only uses _IO_BASE though which we
// define properly based on the platform
//

pub const _ISA_MEM_BASE: c_int = 0;
pub const PCI_DRAM_OFFSET: c_int = 0;

pub const PCI_DRAM_OFFSET: c_int = 0;

// Boolean set by platform if PIO accesses are suppored while _IO_BASE
// is not set or addresses cannot be translated to MMIO. This is typically
// set when the platform supports "special" PIO accesses via a non memory
// mapped mechanism, and allows things like the early udbg UART code to
// function.
//

//
// Low level MMIO accessors
//
// This provides the non-bus specific accessors to MMIO. Those are PowerPC
// specific and thus shouldn't be used in generic code. The accessors
// provided here are:
//
// in_8, in_le16, in_be16, in_le32, in_be32, in_le64, in_be64
// out_8, out_le16, out_be16, out_le32, out_be32, out_le64, out_be64
// _insb, _insw, _insl, _outsb, _outsw, _outsl
//
// Those operate directly on a kernel virtual address. Note that the prototype
// for the out_* accessors has the arguments in opposite order from the usual
// linux PCI accessors. Unlike those, they take the address first and the value
// next.
//
// -mprefixed can generate offsets beyond range, fall back hack

// There is no asm instructions for 64 bits reverse loads and stores
extern "C" {
    pub fn swab64(_arg: in_be64(addr)) -> return;
}

// There is no asm instructions for 64 bits reverse loads and stores
extern "C" {
    pub fn swab64(_arg: in_le64(addr)) -> return;
}

//
// Low level IO stream instructions are defined out of line for now
//
extern "C" {
    pub fn _insb(addr: *const volatile u8 __iomem, buf: *mut c_void, count: c_long);
}
extern "C" {
    pub fn _outsb(addr: *mut volatile u8 __iomem, buf: *const c_void, count: c_long);
}
extern "C" {
    pub fn _insw(addr: *const volatile u16 __iomem, buf: *mut c_void, count: c_long);
}
extern "C" {
    pub fn _outsw(addr: *mut volatile u16 __iomem, buf: *const c_void, count: c_long);
}
extern "C" {
    pub fn _insl(addr: *const volatile u32 __iomem, buf: *mut c_void, count: c_long);
}
extern "C" {
    pub fn _outsl(addr: *mut volatile u32 __iomem, buf: *const c_void, count: c_long);
}
//
// memset_io, memcpy_toio, memcpy_fromio base implementations are out of line
//
extern "C" {
    pub fn _memset_io(addr: *mut volatile void __iomem, c: c_int, n: c_ulong);
}
//
// PCI and standard ISA accessors
//
// Those are globally defined linux accessors for devices on PCI or ISA
// busses. They follow the Linux defined semantics. The current implementation
// for PowerPC is as close as possible to the x86 version of these, and thus
// provides fairly heavy weight barriers for the non-raw versions
//
// In addition, they support a hook mechanism when CONFIG_PPC_INDIRECT_PIO
// is set allowing the platform to provide its own implementation of some
// of the accessors.
//
// Include the EEH definitions when EEH is enabled only so they don't get
// in the way when building for 32 bits
//

//
// Real mode versions of raw accessors. Those instructions are only supposed
// to be used in hypervisor real mode as per the architecture spec.
//

//
// PCI PIO and MMIO accessors.
//
// On 32 bits, PIO operations have a recovery mechanism in case they trigger
// machine checks (which they occasionally do when probing non existing
// IO ports on some platforms, like PowerMac and 8xx).
// I always found it to be of dubious reliability and I am tempted to get
// rid of it one of these days. So if you think it's important to keep it,
// please voice up asap. We never had it for 64 bits and I do not intend
// to port it over
//

// The "__do_*" operations below provide the actual "base" implementation
// for each of the defined accessors. Some of them use the out_* functions
// directly, some of them still use EEH, though we might change that in the
// future. Those macros below provide the necessary argument swapping and
// handling of the IO base for PIO.
//
// They are themselves used by the macros that define the actual accessors
// and can be used by the hooks if any.
//
// Note that PIO operations are always defined in terms of their corresonding
// MMIO operations. That allows platforms like iSeries who want to modify the
// behaviour of both to only hook on the MMIO version and get both. It's also
// possible to hook directly at the toplevel PIO operation if they have to
// be handled differently
//

extern "C" {
    pub fn __do_readb(_arg: addr) -> return;
}

extern "C" {
    pub fn __do_readw(_arg: addr) -> return;
}

extern "C" {
    pub fn __do_readl(_arg: addr) -> return;
}

extern "C" {
    pub fn __do_readw_be(_arg: addr) -> return;
}
extern "C" {
    pub fn __do_readl_be(_arg: addr) -> return;
}

extern "C" {
    pub fn __do_readq(_arg: addr) -> return;
}
extern "C" {
    pub fn __do_readq_be(_arg: addr) -> return;
}

// Structure containing all the hooks

// The inline wrappers

// Signal to asm-generic/io.h that we have implemented these.

//
// We don't do relaxed operations yet, at least not with this semantic
//

//
// Here comes the implementation of the IOMAP interfaces.
//
extern "C" {
    pub fn readw_be(_arg: addr) -> return;
}

extern "C" {
    pub fn readl_be(_arg: addr) -> return;
}

extern "C" {
    pub fn readq_be(_arg: addr) -> return;
}

extern "C" {
    pub fn pci_iounmap(dev: *mut pci_dev, addr: *mut void __iomem);
}

extern "C" {
    pub fn __volatile__("memory": "sync" : : :) -> __asm__;
}
// Enforce in-order execution of data I/O.
// No distinction between read/write on PPC; use eieio for all three.
// Those are fairly week though. They don't provide a barrier between
// MMIO and cacheable storage nor do they provide a barrier vs. locks,
// they only provide barriers between 2 __raw MMIO operations and
// possibly break write combining.
//

//
// output pause versions need a delay at least for the
// w83c105 ide controller in a p610.
//

//
// ioremap     -   map bus memory into CPU space
// @address:   bus address of the memory
// @size:      size of the resource to map
//
// ioremap performs a platform specific sequence of operations to
// make bus memory CPU accessible via the readb/readw/readl/writeb
// writew/writel functions and the other mmio helpers. The returned
// address is not guaranteed to be usable directly as a virtual
// address.
//
// We provide a few variations of it:
//
// * ioremap is the standard one and provides non-cacheable guarded mappings
// and can be hooked by the platform via ppc_md
//
// * ioremap_prot allows to specify the page flags as an argument and can
// also be hooked by the platform via ppc_md.
//
// * ioremap_wc enables write combining
//
// * ioremap_wt enables write through
//
// * ioremap_coherent maps coherent cached memory
//
// * iounmap undoes such a mapping and can be hooked
//
// * __ioremap_caller is the same as above but takes an explicit caller
// reference rather than using __builtin_return_address(0)
//

//
// When CONFIG_PPC_INDIRECT_PIO is set, we use the generic iomap implementation
// which needs some additional definitions here. They basically allow PIO
// space overall to be 1GB. This will work as long as we never try to use
// iomap to map MMIO below 1GB which should be fine on ppc64
//
pub const HAVE_ARCH_PIO_SIZE: c_int = 1;
pub const PIO_OFFSET: c_uint = 0x00000000UL;

//
// virt_to_phys	-	map virtual addresses to physical
// @address: address to remap
//
// The returned physical address is the physical (CPU) mapping for
// the memory address given. It is only valid to use this function on
// addresses directly mapped or allocated via kmalloc.
//
// This function does not give bus mappings for DMA transfers. In
// almost all conceivable cases a device driver should not be using
// this function
//
extern "C" {
    pub fn __pa(long)address: (unsigned) -> return;
}

//
// phys_to_virt	-	map physical address to virtual
// @address: address to remap
//
// The returned virtual address is a current CPU mapping for
// the memory address given. It is only valid to use this function on
// addresses that have a kernel mapping
//
// This function does not handle bus mappings for DMA transfers. In
// almost all conceivable cases a device driver should not be using
// this function
//

//
// 32 bits still uses virt_to_bus() for its implementation of DMA
// mappings se we have to keep it defined here. We also have some old
// drivers (shame shame shame) that use bus_to_virt() and haven't been
// fixed yet so I need to define it here.
//

extern "C" {
    pub fn __va(PCI_DRAM_OFFSET: address -) -> return;
}

// access ports

// Clear and set bits in one shot.  These macros can be used to clear and
// set multiple bits in a register using a single read-modify-write.  These
// macros can also be used to set a multiple-bit bit pattern using a mask,
// by specifying the mask in the 'clear' parameter and the new bit pattern
// in the 'set' parameter.
//

