//! Automatically rewritten from C to Rust
//! Source: lib/iomap.c
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
// Implement the default iomap interfaces
//
// (C) Copyright 2004 Linus Torvalds
//

//
// Read/write from/to an (offsettable) iomem cookie. It might be a PIO
// access or a MMIO access, these functions don't care. The info is
// encoded in the hardware mapping set up by the mapping functions
// (or the cookie itself, depending on implementation and hw).
//
// The generic routines don't assume any hardware mappings, and just
// encode the PIO/MMIO as part of the cookie. They coldly assume that
// the MMIO IO mappings are not in the low address range.
//
// Architectures for which this is not true can't use this generic
// implementation and should do their own copy.
//

//
// We encode the physical PIO addresses (0-0xffff) into the
// pointer by offsetting them with a constant (0x10000) and
// assuming that all the low addresses are always PIO. That means
// we can do some sanity checks on the low bits, and don't
// need to just take things for granted.
//
pub const PIO_OFFSET: c_uint = 0x10000UL;
pub const PIO_MASK: c_uint = 0x0ffffUL;
pub const PIO_RESERVED: c_uint = 0x40000UL;

#[no_mangle]
unsafe extern "C" fn bad_io_access(port: c_ulong, access: *const c_char) {
    static void bad_io_access(unsigned long port, const char *access)
    {
    let mut count: static int = 10;
    if (count) {
    count--;
    WARN(1, KERN_ERR "Bad IO access at port %#lx (%s)\n", port, access);
    }
    }
//
// Ugly macros are a way of life.
//

    unsigned long port = (unsigned long )addr;	\
    if (port >= PIO_RESERVED) {				\
    is_mmio;					\
    } else if (port > PIO_OFFSET) {				\
    port &= PIO_MASK;				\
    is_pio;						\
    } else							\
    bad_io_access(port, #is_pio );			\
    } while (0)

//
// Here and below, we apply __no_kmsan_checks to functions reading data from
// hardware, to ensure that KMSAN marks their return values as initialized.
//
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn ioread8(addr: *const void __iomem) -> c_uint {
    unsigned int ioread8(const void __iomem *addr)
    {
    IO_COND(addr, return inb(port), return readb(addr));
    return 0xff;
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn ioread16(addr: *const void __iomem) -> c_uint {
    unsigned int ioread16(const void __iomem *addr)
    {
    IO_COND(addr, return inw(port), return readw(addr));
    return 0xffff;
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn ioread16be(addr: *const void __iomem) -> c_uint {
    unsigned int ioread16be(const void __iomem *addr)
    {
    IO_COND(addr, return pio_read16be(port), return mmio_read16be(addr));
    return 0xffff;
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn ioread32(addr: *const void __iomem) -> c_uint {
    unsigned int ioread32(const void __iomem *addr)
    {
    IO_COND(addr, return inl(port), return readl(addr));
    return 0xffffffff;
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn ioread32be(addr: *const void __iomem) -> c_uint {
    unsigned int ioread32be(const void __iomem *addr)
    {
    IO_COND(addr, return pio_read32be(port), return mmio_read32be(addr));
    return 0xffffffff;
    }
    EXPORT_SYMBOL(ioread8);
    EXPORT_SYMBOL(ioread16);
    EXPORT_SYMBOL(ioread16be);
    EXPORT_SYMBOL(ioread32);
    EXPORT_SYMBOL(ioread32be);

#[no_mangle]
unsafe extern "C" fn pio_read64_lo_hi(port: c_ulong) -> u64 {
    static u64 pio_read64_lo_hi(unsigned long port)
    {
    u64 lo, hi;
    lo = inl(port);
    hi = inl(port + sizeof(u32));
    return lo | (hi << 32);
    }
#[no_mangle]
unsafe extern "C" fn pio_read64_hi_lo(port: c_ulong) -> u64 {
    static u64 pio_read64_hi_lo(unsigned long port)
    {
    u64 lo, hi;
    hi = inl(port + sizeof(u32));
    lo = inl(port);
    return lo | (hi << 32);
    }
#[no_mangle]
unsafe extern "C" fn pio_read64be_lo_hi(port: c_ulong) -> u64 {
    static u64 pio_read64be_lo_hi(unsigned long port)
    {
    u64 lo, hi;
    lo = pio_read32be(port + sizeof(u32));
    hi = pio_read32be(port);
    return lo | (hi << 32);
    }
#[no_mangle]
unsafe extern "C" fn pio_read64be_hi_lo(port: c_ulong) -> u64 {
    static u64 pio_read64be_hi_lo(unsigned long port)
    {
    u64 lo, hi;
    hi = pio_read32be(port);
    lo = pio_read32be(port + sizeof(u32));
    return lo | (hi << 32);
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn __ioread64_lo_hi(addr: *const void __iomem) -> u64 {
    u64 __ioread64_lo_hi(const void __iomem *addr)
    {
    IO_COND(addr, return pio_read64_lo_hi(port), return readq(addr));
    return 0xffffffffffffffffULL;
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn __ioread64_hi_lo(addr: *const void __iomem) -> u64 {
    u64 __ioread64_hi_lo(const void __iomem *addr)
    {
    IO_COND(addr, return pio_read64_hi_lo(port), return readq(addr));
    return 0xffffffffffffffffULL;
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn __ioread64be_lo_hi(addr: *const void __iomem) -> u64 {
    u64 __ioread64be_lo_hi(const void __iomem *addr)
    {
    IO_COND(addr, return pio_read64be_lo_hi(port),
    return mmio_read64be(addr));
    return 0xffffffffffffffffULL;
    }
    __no_kmsan_checks
#[no_mangle]
pub unsafe extern "C" fn __ioread64be_hi_lo(addr: *const void __iomem) -> u64 {
    u64 __ioread64be_hi_lo(const void __iomem *addr)
    {
    IO_COND(addr, return pio_read64be_hi_lo(port),
    return mmio_read64be(addr));
    return 0xffffffffffffffffULL;
    }
    EXPORT_SYMBOL(__ioread64_lo_hi);
    EXPORT_SYMBOL(__ioread64_hi_lo);
    EXPORT_SYMBOL(__ioread64be_lo_hi);
    EXPORT_SYMBOL(__ioread64be_hi_lo);

#[no_mangle]
pub unsafe extern "C" fn iowrite8(val: u8, addr: *mut void __iomem) {
    void iowrite8(u8 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, outb(val,port), writeb(val, addr));
    }
#[no_mangle]
pub unsafe extern "C" fn iowrite16(val: u16, addr: *mut void __iomem) {
    void iowrite16(u16 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, outw(val,port), writew(val, addr));
    }
#[no_mangle]
pub unsafe extern "C" fn iowrite16be(val: u16, addr: *mut void __iomem) {
    void iowrite16be(u16 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, pio_write16be(val,port), mmio_write16be(val, addr));
    }
#[no_mangle]
pub unsafe extern "C" fn iowrite32(val: u32, addr: *mut void __iomem) {
    void iowrite32(u32 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, outl(val,port), writel(val, addr));
    }
#[no_mangle]
pub unsafe extern "C" fn iowrite32be(val: u32, addr: *mut void __iomem) {
    void iowrite32be(u32 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, pio_write32be(val,port), mmio_write32be(val, addr));
    }
    EXPORT_SYMBOL(iowrite8);
    EXPORT_SYMBOL(iowrite16);
    EXPORT_SYMBOL(iowrite16be);
    EXPORT_SYMBOL(iowrite32);
    EXPORT_SYMBOL(iowrite32be);

#[no_mangle]
unsafe extern "C" fn pio_write64_lo_hi(val: u64, port: c_ulong) {
    static void pio_write64_lo_hi(u64 val, unsigned long port)
    {
    outl(val, port);
    outl(val >> 32, port + sizeof(u32));
    }
#[no_mangle]
unsafe extern "C" fn pio_write64_hi_lo(val: u64, port: c_ulong) {
    static void pio_write64_hi_lo(u64 val, unsigned long port)
    {
    outl(val >> 32, port + sizeof(u32));
    outl(val, port);
    }
#[no_mangle]
unsafe extern "C" fn pio_write64be_lo_hi(val: u64, port: c_ulong) {
    static void pio_write64be_lo_hi(u64 val, unsigned long port)
    {
    pio_write32be(val, port + sizeof(u32));
    pio_write32be(val >> 32, port);
    }
#[no_mangle]
unsafe extern "C" fn pio_write64be_hi_lo(val: u64, port: c_ulong) {
    static void pio_write64be_hi_lo(u64 val, unsigned long port)
    {
    pio_write32be(val >> 32, port);
    pio_write32be(val, port + sizeof(u32));
    }
#[no_mangle]
pub unsafe extern "C" fn __iowrite64_lo_hi(val: u64, addr: *mut void __iomem) {
    void __iowrite64_lo_hi(u64 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, pio_write64_lo_hi(val, port),
    writeq(val, addr));
    }
#[no_mangle]
pub unsafe extern "C" fn __iowrite64_hi_lo(val: u64, addr: *mut void __iomem) {
    void __iowrite64_hi_lo(u64 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, pio_write64_hi_lo(val, port),
    writeq(val, addr));
    }
#[no_mangle]
pub unsafe extern "C" fn __iowrite64be_lo_hi(val: u64, addr: *mut void __iomem) {
    void __iowrite64be_lo_hi(u64 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, pio_write64be_lo_hi(val, port),
    mmio_write64be(val, addr));
    }
#[no_mangle]
pub unsafe extern "C" fn __iowrite64be_hi_lo(val: u64, addr: *mut void __iomem) {
    void __iowrite64be_hi_lo(u64 val, void __iomem *addr)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(&val, sizeof(val));
    IO_COND(addr, pio_write64be_hi_lo(val, port),
    mmio_write64be(val, addr));
    }
    EXPORT_SYMBOL(__iowrite64_lo_hi);
    EXPORT_SYMBOL(__iowrite64_hi_lo);
    EXPORT_SYMBOL(__iowrite64be_lo_hi);
    EXPORT_SYMBOL(__iowrite64be_hi_lo);

//
// These are the "repeat MMIO read/write" functions.
// Note the "__raw" accesses, since we don't want to
// convert to CPU byte order. We write in "IO byte
// order" (we also don't have IO barriers).
//

#[no_mangle]
pub unsafe extern "C" fn mmio_insb(addr: *const void __iomem, dst: *mut u8, count: c_int) {
    static inline void mmio_insb(const void __iomem *addr, u8 *dst, int count)
    {
    while (--count >= 0) {
    let mut data: u8 = __raw_readb(addr);
// dst = data;
    dst++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_insw(addr: *const void __iomem, dst: *mut u16, count: c_int) {
    static inline void mmio_insw(const void __iomem *addr, u16 *dst, int count)
    {
    while (--count >= 0) {
    let mut data: u16 = __raw_readw(addr);
// dst = data;
    dst++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_insl(addr: *const void __iomem, dst: *mut u32, count: c_int) {
    static inline void mmio_insl(const void __iomem *addr, u32 *dst, int count)
    {
    while (--count >= 0) {
    let mut data: u32 = __raw_readl(addr);
// dst = data;
    dst++;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn mmio_outsb(addr: *mut void __iomem, src: *const u8, count: c_int) {
    static inline void mmio_outsb(void __iomem *addr, const u8 *src, int count)
    {
    while (--count >= 0) {
    __raw_writeb(*src, addr);
    src++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_outsw(addr: *mut void __iomem, src: *const u16, count: c_int) {
    static inline void mmio_outsw(void __iomem *addr, const u16 *src, int count)
    {
    while (--count >= 0) {
    __raw_writew(*src, addr);
    src++;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn mmio_outsl(addr: *mut void __iomem, src: *const u32, count: c_int) {
    static inline void mmio_outsl(void __iomem *addr, const u32 *src, int count)
    {
    while (--count >= 0) {
    __raw_writel(*src, addr);
    src++;
    }
    }

#[no_mangle]
pub unsafe extern "C" fn ioread8_rep(addr: *const void __iomem, dst: *mut c_void, count: c_ulong) {
    void ioread8_rep(const void __iomem *addr, void *dst, unsigned long count)
    {
    IO_COND(addr, insb(port,dst,count), mmio_insb(addr, dst, count));
// KMSAN must treat values read from devices as initialized.
    kmsan_unpoison_memory(dst, count);
    }
#[no_mangle]
pub unsafe extern "C" fn ioread16_rep(addr: *const void __iomem, dst: *mut c_void, count: c_ulong) {
    void ioread16_rep(const void __iomem *addr, void *dst, unsigned long count)
    {
    IO_COND(addr, insw(port,dst,count), mmio_insw(addr, dst, count));
// KMSAN must treat values read from devices as initialized.
    kmsan_unpoison_memory(dst, count * 2);
    }
#[no_mangle]
pub unsafe extern "C" fn ioread32_rep(addr: *const void __iomem, dst: *mut c_void, count: c_ulong) {
    void ioread32_rep(const void __iomem *addr, void *dst, unsigned long count)
    {
    IO_COND(addr, insl(port,dst,count), mmio_insl(addr, dst, count));
// KMSAN must treat values read from devices as initialized.
    kmsan_unpoison_memory(dst, count * 4);
    }
    EXPORT_SYMBOL(ioread8_rep);
    EXPORT_SYMBOL(ioread16_rep);
    EXPORT_SYMBOL(ioread32_rep);
#[no_mangle]
pub unsafe extern "C" fn iowrite8_rep(addr: *mut void __iomem, src: *const c_void, count: c_ulong) {
    void iowrite8_rep(void __iomem *addr, const void *src, unsigned long count)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(src, count);
    IO_COND(addr, outsb(port, src, count), mmio_outsb(addr, src, count));
    }
#[no_mangle]
pub unsafe extern "C" fn iowrite16_rep(addr: *mut void __iomem, src: *const c_void, count: c_ulong) {
    void iowrite16_rep(void __iomem *addr, const void *src, unsigned long count)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(src, count * 2);
    IO_COND(addr, outsw(port, src, count), mmio_outsw(addr, src, count));
    }
#[no_mangle]
pub unsafe extern "C" fn iowrite32_rep(addr: *mut void __iomem, src: *const c_void, count: c_ulong) {
    void iowrite32_rep(void __iomem *addr, const void *src, unsigned long count)
    {
// Make sure uninitialized memory isn't copied to devices.
    kmsan_check_memory(src, count * 4);
    IO_COND(addr, outsl(port, src,count), mmio_outsl(addr, src, count));
    }
    EXPORT_SYMBOL(iowrite8_rep);
    EXPORT_SYMBOL(iowrite16_rep);
    EXPORT_SYMBOL(iowrite32_rep);

// Create a virtual mapping cookie for an IO port range
    void __iomem *ioport_map(unsigned long port, unsigned int nr)
    {
    if (port > PIO_MASK)
    return core::ptr::null_mut();
    return (void __iomem *) (unsigned long) (port + PIO_OFFSET);
    }
#[no_mangle]
pub unsafe extern "C" fn ioport_unmap(addr: *mut void __iomem) {
    void ioport_unmap(void __iomem *addr)
    {
// Nothing to do
    }
    EXPORT_SYMBOL(ioport_map);
    EXPORT_SYMBOL(ioport_unmap);

// Hide the details if this is a MMIO or PIO address space and just do what
// you expect in the correct way.
#[no_mangle]
pub unsafe extern "C" fn pci_iounmap(dev: *mut pci_dev, addr: *mut *mut void __iomem) {
    void pci_iounmap(struct pci_dev *dev, void __iomem * addr)
    {
    IO_COND(addr, /* nothing */, iounmap(addr));
    }
    EXPORT_SYMBOL(pci_iounmap);
