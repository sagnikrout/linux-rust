//! Automatically rewritten from C to Rust
//! Source: drivers/char/ipmi/ipmi_si_parisc.c
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

    static bool parisc_registered;
#[no_mangle]
unsafe extern "C" fn ipmi_parisc_probe(dev: *mut parisc_device) -> int __init {
    static int __init ipmi_parisc_probe(struct parisc_device *dev)
    {
    struct si_sm_io io;
    memset(&io, 0, sizeof(io));
    io.si_info	= &ipmi_kcs_si_info;
    io.addr_source	= SI_DEVICETREE;
    io.addr_space	= IPMI_MEM_ADDR_SPACE;
    io.addr_data	= dev.hpa.start;
    io.regsize	= 1;
    io.regspacing	= 1;
    io.regshift	= 0;
    io.irq		= 0; /* no interrupt */
    io.irq_setup	= core::ptr::null_mut();
    io.dev		= &dev.dev;
    dev_dbg(&dev.dev, "addr 0x%lx\n", io.addr_data);
    return ipmi_si_add_smi(&io);
    }
#[no_mangle]
unsafe extern "C" fn ipmi_parisc_remove(dev: *mut parisc_device) -> void __exit {
    static void __exit ipmi_parisc_remove(struct parisc_device *dev)
    {
    ipmi_si_remove_by_dev(&dev.dev);
    }
    static const struct parisc_device_id ipmi_parisc_tbl[] __initconst = {
    { HPHW_MC, HVERSION_REV_ANY_ID, 0x004, 0xC0 },
    { 0, }
    };
    MODULE_DEVICE_TABLE(parisc, ipmi_parisc_tbl);
    static struct parisc_driver ipmi_parisc_driver __refdata = {
    .name =		"ipmi",
    .id_table =	ipmi_parisc_tbl,
    .probe =	ipmi_parisc_probe,
    .remove =	__exit_p(ipmi_parisc_remove),
    };
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_parisc_init() {
    void ipmi_si_parisc_init(void)
    {
    register_parisc_driver(&ipmi_parisc_driver);
    parisc_registered = true;
    }
#[no_mangle]
pub unsafe extern "C" fn ipmi_si_parisc_shutdown() {
    void ipmi_si_parisc_shutdown(void)
    {
    if (parisc_registered)
    unregister_parisc_driver(&ipmi_parisc_driver);
    }
