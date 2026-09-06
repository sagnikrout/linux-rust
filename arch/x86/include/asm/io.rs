//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/io.h
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
// This file contains the definitions for the x86 IO instructions
// inb/inw/inl/outb/outw/outl and the "string versions" of the same
// (insb/insw/insl/outsb/outsw/outsl). You can also use "pausing"
// versions of the single-IO instructions (inb_p/inw_p/..).
//
// This file is not meant to be obfuscating: it's just complicated
// to (a) handle it all in a way that makes gcc able to optimize it
// as well as possible and (b) trying to avoid writing the same thing
// over and over again with slight variations and possibly making a
// mistake somewhere.
//
// Thanks to James van Artsdalen for a better timing-fix than
// the two short jumps: using outb's to a nonexistent port seems
// to guarantee better timings even on fast machines.
//
// On the other hand, I'd like to be sure of a non-existent port:
// I feel a bit unsafe about using 0x80 (should be safe, though)
//
// Linus
//
// Bit simplified and optimized by Jan Hubicka
// Support of BIGMEM added by Gerhard Wichert, Siemens AG, July 1999.
//
// isa_memset_io, isa_memcpy_fromio, isa_memcpy_toio added,
// isa_read[wl] and isa_write[wl] fixed
// - Arnaldo Carvalho de Melo <acme@conectiva.com.br>
//

// Let people know that we have them

// Macro flag: #define ARCH_HAS_VALID_PHYS_ADDR_RANGE
extern "C" {
    pub fn valid_phys_addr_range(addr: phys_addr_t, size: usize) -> c_int;
}
extern "C" {
    pub fn valid_mmap_phys_addr_range(pfn: c_ulong, size: usize) -> c_int;
}
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
    pub fn __pa(_arg: address) -> return;
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
extern "C" {
    pub fn __va(_arg: address) -> return;
}

//
// ISA I/O bus memory addresses are 1:1 with the physical address.
// However, we truncate the address to unsigned int to avoid undesirable
// promotions in legacy drivers.
//

//
// The default ioremap() behavior is non-cached; if you need something
// else, you probably want one of the following.
//

//
// ioremap     -   map bus memory into CPU space
// @offset:    bus address of the memory
// @size:      size of the resource to map
//
// ioremap performs a platform specific sequence of operations to
// make bus memory CPU accessible via the readb/readw/readl/writeb
// writew/writel functions and the other mmio helpers. The returned
// address is not guaranteed to be usable directly as a virtual
// address.
//
// If the area you are trying to map is a PCI BAR you should have a
// look at pci_iomap().
//

extern "C" {
    pub fn iounmap(addr: *mut volatile void __iomem);
}

extern "C" {
    pub fn memcpy_fromio(: *mut c_void, : *const volatile void __iomem, _arg: usize);
}
extern "C" {
    pub fn memcpy_toio(: *mut volatile void __iomem, : *const c_void, _arg: usize);
}
extern "C" {
    pub fn memset_io(: *mut volatile void __iomem, _arg: c_int, _arg: usize);
}

//
// Commit 0f07496144c2 ("[PATCH] Add faster __iowrite32_copy routine for
// x86_64") says that circa 2006 rep movsl is noticeably faster than a copy
// loop.
//

//
// ISA space is 'always mapped' on a typical x86 system, no need to
// explicitly ioremap() it. The fact that the ISA IO space is mapped
// to PAGE_OFFSET is pure coincidence - it does not mean ISA values
// are physical addresses. The following constant pointer can be
// used as the IO-area pointer (it can be iounmapped as well, so the
// analogy with PCI is quite large):
//

extern "C" {
    pub fn native_io_delay();
}
extern "C" {
    pub fn io_delay_init();
}

// value = in##bwl(port);				\

extern "C" {
    pub fn unxlate_dev_mem_ptr(phys: phys_addr_t, addr: *mut c_void);
}

extern "C" {
    pub fn is_early_ioremap_ptep(ptep: *mut pte_t) -> bool;
}
pub const IO_SPACE_LIMIT: c_uint = 0xffff;

extern "C" {
    pub fn arch_phys_wc_index(handle: c_int) -> int __must_check;
}

extern "C" {
    pub fn arch_phys_wc_del(handle: c_int);
}

extern "C" {
    pub fn arch_io_reserve_memtype_wc(start: resource_size_t, size: resource_size_t) -> c_int;
}
extern "C" {
    pub fn arch_io_free_memtype_wc(start: resource_size_t, size: resource_size_t);
}

//
// iosubmit_cmds512 - copy data to single MMIO location, in 512-bit units
// @dst: destination, in MMIO space (must be 512-bit aligned)
// @src: source
// @count: number of 512 bits quantities to submit
//
// Submit data from kernel space to MMIO space, in units of 512 bits at a
// time.  Order of access is not guaranteed, nor is a memory barrier
// performed afterwards.
//
// Warning: Do not use this helper unless your driver has checked that the CPU
// instruction is supported on the platform.
//
