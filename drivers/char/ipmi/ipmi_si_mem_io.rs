//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_si_mem_io.c
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


// SPDX-License-Identifier: GPL-2.0+

    static unsigned char intf_mem_inb(const struct si_sm_io *io,
    unsigned int offset)
    {
    return readb((io.addr)+(offset * io.regspacing));
    }
    static void intf_mem_outb(const struct si_sm_io *io, unsigned int offset,
    unsigned char b)
    {
    writeb(b, (io.addr)+(offset * io.regspacing));
    }
    static unsigned char intf_mem_inw(const struct si_sm_io *io,
    unsigned int offset)
    {
    return (readw((io.addr)+(offset * io.regspacing)) >> io.regshift)
    & 0xff;
    }
    static void intf_mem_outw(const struct si_sm_io *io, unsigned int offset,
    unsigned char b)
    {
    writeb(b << io.regshift, (io.addr)+(offset * io.regspacing));
    }
    static unsigned char intf_mem_inl(const struct si_sm_io *io,
    unsigned int offset)
    {
    return (readl((io.addr)+(offset * io.regspacing)) >> io.regshift)
    & 0xff;
    }
    static void intf_mem_outl(const struct si_sm_io *io, unsigned int offset,
    unsigned char b)
    {
    writel(b << io.regshift, (io.addr)+(offset * io.regspacing));
    }

#[no_mangle]
unsafe extern "C" fn mem_inq(io: *const si_sm_io, offset: c_uint) -> c_uchar {
    static unsigned char mem_inq(const struct si_sm_io *io, unsigned int offset)
    {
    return (readq((io.addr)+(offset * io.regspacing)) >> io.regshift)
    & 0xff;
    }
    static void mem_outq(const struct si_sm_io *io, unsigned int offset,
    unsigned char b)
    {
    writeq((u64)b << io.regshift, (io.addr)+(offset * io.regspacing));
    }

#[no_mangle]
unsafe extern "C" fn mem_region_cleanup(io: *mut si_sm_io, num: c_int) {
    static void mem_region_cleanup(struct si_sm_io *io, int num)
    {
    let mut addr: c_ulong = io.addr_data;
    int idx;
    for (idx = 0; idx < num; idx++)
    release_mem_region(addr + idx * io.regspacing,
    io.regsize);
    }
#[no_mangle]
unsafe extern "C" fn mem_cleanup(io: *mut si_sm_io) {
    static void mem_cleanup(struct si_sm_io *io)
    {
    if (io.addr) {
    iounmap(io.addr);
    mem_region_cleanup(io, io.io_size);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_mem_setup(io: *mut si_sm_io) -> c_int {
    int ipmi_si_mem_setup(struct si_sm_io *io)
    {
    let mut addr: c_ulong = io.addr_data;
    int           mapsize, idx;
    if (!addr)
    return -ENODEV;
//
// Figure out the actual readb/readw/readl/etc routine to use based
// upon the register size.
//
    switch (io.regsize) {
    case 1:
    io.inputb = intf_mem_inb;
    io.outputb = intf_mem_outb;
    break;
    case 2:
    io.inputb = intf_mem_inw;
    io.outputb = intf_mem_outw;
    break;
    case 4:
    io.inputb = intf_mem_inl;
    io.outputb = intf_mem_outl;
    break;

    case 8:
    io.inputb = mem_inq;
    io.outputb = mem_outq;
    break;

    default:
    dev_warn(io.dev, "Invalid register size: %d\n",
    io.regsize);
    return -EINVAL;
    }
//
// Some BIOSes reserve disjoint memory regions in their ACPI
// tables.  This causes problems when trying to request the
// entire region.  Therefore we must request each register
// separately.
//
    for (idx = 0; idx < io.io_size; idx++) {
    if (request_mem_region(addr + idx * io.regspacing,
    io.regsize, SI_DEVICE_NAME) == core::ptr::null_mut()) {
// Undo allocations
    mem_region_cleanup(io, idx);
    return -EIO;
    }
    }
//
// Calculate the total amount of memory to claim.  This is an
// unusual looking calculation, but it avoids claiming any
// more memory than it has to.  It will claim everything
// between the first address to the end of the last full
// register.
//
    mapsize = ((io.io_size * io.regspacing)
    - (io.regspacing - io.regsize));
    io.addr = ioremap(addr, mapsize);
    if (io.addr == core::ptr::null_mut()) {
    mem_region_cleanup(io, io.io_size);
    return -EIO;
    }
    io.io_cleanup = mem_cleanup;
    return 0;
    }
