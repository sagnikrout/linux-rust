//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_si_port_io.c
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

#[no_mangle]
unsafe extern "C" fn port_inb(io: *const si_sm_io, offset: c_uint) -> c_uchar {
    static unsigned char port_inb(const struct si_sm_io *io, unsigned int offset)
    {
    let mut addr: c_uint = io.addr_data;
    return inb(addr + (offset * io.regspacing));
    }
    static void port_outb(const struct si_sm_io *io, unsigned int offset,
    unsigned char b)
    {
    let mut addr: c_uint = io.addr_data;
    outb(b, addr + (offset * io.regspacing));
    }
#[no_mangle]
unsafe extern "C" fn port_inw(io: *const si_sm_io, offset: c_uint) -> c_uchar {
    static unsigned char port_inw(const struct si_sm_io *io, unsigned int offset)
    {
    let mut addr: c_uint = io.addr_data;
    return (inw(addr + (offset * io.regspacing)) >> io.regshift) & 0xff;
    }
    static void port_outw(const struct si_sm_io *io, unsigned int offset,
    unsigned char b)
    {
    let mut addr: c_uint = io.addr_data;
    outw(b << io.regshift, addr + (offset * io.regspacing));
    }
#[no_mangle]
unsafe extern "C" fn port_inl(io: *const si_sm_io, offset: c_uint) -> c_uchar {
    static unsigned char port_inl(const struct si_sm_io *io, unsigned int offset)
    {
    let mut addr: c_uint = io.addr_data;
    return (inl(addr + (offset * io.regspacing)) >> io.regshift) & 0xff;
    }
    static void port_outl(const struct si_sm_io *io, unsigned int offset,
    unsigned char b)
    {
    let mut addr: c_uint = io.addr_data;
    outl(b << io.regshift, addr+(offset * io.regspacing));
    }
#[no_mangle]
unsafe extern "C" fn port_cleanup(io: *mut si_sm_io) {
    static void port_cleanup(struct si_sm_io *io)
    {
    let mut addr: c_uint = io.addr_data;
    int          idx;
    if (addr) {
    for (idx = 0; idx < io.io_size; idx++)
    release_region(addr + idx * io.regspacing,
    io.regsize);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_port_setup(io: *mut si_sm_io) -> c_int {
    int ipmi_si_port_setup(struct si_sm_io *io)
    {
    let mut addr: c_uint = io.addr_data;
    int          idx;
    if (!addr)
    return -ENODEV;
//
// Figure out the actual inb/inw/inl/etc routine to use based
// upon the register size.
//
    switch (io.regsize) {
    case 1:
    io.inputb = port_inb;
    io.outputb = port_outb;
    break;
    case 2:
    io.inputb = port_inw;
    io.outputb = port_outw;
    break;
    case 4:
    io.inputb = port_inl;
    io.outputb = port_outl;
    break;
    default:
    dev_warn(io.dev, "Invalid register size: %d\n",
    io.regsize);
    return -EINVAL;
    }
//
// Some BIOSes reserve disjoint I/O regions in their ACPI
// tables.  This causes problems when trying to register the
// entire I/O region.  Therefore we must register each I/O
// port separately.
//
    for (idx = 0; idx < io.io_size; idx++) {
    if (request_region(addr + idx * io.regspacing,
    io.regsize, SI_DEVICE_NAME) == core::ptr::null_mut()) {
// Undo allocations
    while (idx--)
    release_region(addr + idx * io.regspacing,
    io.regsize);
    return -EIO;
    }
    }
    io.io_cleanup = port_cleanup;
    return 0;
    }
