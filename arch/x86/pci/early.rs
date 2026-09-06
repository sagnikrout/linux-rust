//! Automatically rewritten from C to Rust
//! Source: arch/x86/pci/early.c
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

// Direct PCI access. This is used for PCI accesses in early boot before
    the PCI subsystem works. */
#[no_mangle]
pub unsafe extern "C" fn read_pci_config(bus: u8, slot: u8, func: u8, offset: u8) -> u32 {
    u32 read_pci_config(u8 bus, u8 slot, u8 func, u8 offset)
    {
    u32 v;
    outl(0x80000000 | (bus<<16) | (slot<<11) | (func<<8) | offset, 0xcf8);
    v = inl(0xcfc);
    return v;
    }
#[no_mangle]
pub unsafe extern "C" fn read_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8) -> u8 {
    u8 read_pci_config_byte(u8 bus, u8 slot, u8 func, u8 offset)
    {
    u8 v;
    outl(0x80000000 | (bus<<16) | (slot<<11) | (func<<8) | offset, 0xcf8);
    v = inb(0xcfc + (offset&3));
    return v;
    }
#[no_mangle]
pub unsafe extern "C" fn read_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8) -> u16 {
    u16 read_pci_config_16(u8 bus, u8 slot, u8 func, u8 offset)
    {
    u16 v;
    outl(0x80000000 | (bus<<16) | (slot<<11) | (func<<8) | offset, 0xcf8);
    v = inw(0xcfc + (offset&2));
    return v;
    }
    void write_pci_config(u8 bus, u8 slot, u8 func, u8 offset,
    u32 val)
    {
    outl(0x80000000 | (bus<<16) | (slot<<11) | (func<<8) | offset, 0xcf8);
    outl(val, 0xcfc);
    }
#[no_mangle]
pub unsafe extern "C" fn write_pci_config_byte(bus: u8, slot: u8, func: u8, offset: u8, val: u8) {
    void write_pci_config_byte(u8 bus, u8 slot, u8 func, u8 offset, u8 val)
    {
    outl(0x80000000 | (bus<<16) | (slot<<11) | (func<<8) | offset, 0xcf8);
    outb(val, 0xcfc + (offset&3));
    }
#[no_mangle]
pub unsafe extern "C" fn write_pci_config_16(bus: u8, slot: u8, func: u8, offset: u8, val: u16) {
    void write_pci_config_16(u8 bus, u8 slot, u8 func, u8 offset, u16 val)
    {
    outl(0x80000000 | (bus<<16) | (slot<<11) | (func<<8) | offset, 0xcf8);
    outw(val, 0xcfc + (offset&2));
    }
#[no_mangle]
pub unsafe extern "C" fn early_pci_allowed() -> c_int {
    int early_pci_allowed(void)
    {
    return (pci_probe & (PCI_PROBE_CONF1|PCI_PROBE_NOEARLY)) ==
    PCI_PROBE_CONF1;
    }
