//! Automatically rewritten from C to Rust
//! Source: arch/x86/kernel/resource.c
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

    static void resource_clip(struct resource *res, resource_size_t start,
    resource_size_t end)
    {
    let mut low: resource_size_t = 0, high = 0;
    if (res.end < start || res.start > end)
    return;		/* no conflict */
    if (res.start < start)
    low = start - res.start;
    if (res.end > end)
    high = res.end - end;
// Keep the area above or below the conflict, whichever is larger
    if (low > high)
    res.end = start - 1;
    else
    res.start = end + 1;
    }
#[no_mangle]
unsafe extern "C" fn remove_e820_regions(avail: *mut resource) {
    static void remove_e820_regions(struct resource *avail)
    {
    int i;
    struct e820_entry *entry;
    u64 e820_start, e820_end;
    let mut orig: resource = *avail;
    if (!pci_use_e820)
    return;
    for (i = 0; i < e820_table.nr_entries; i++) {
    entry = &e820_table.entries[i];
    e820_start = entry.addr;
    e820_end = entry.addr + entry.size - 1;
    resource_clip(avail, e820_start, e820_end);
    if (orig.start != avail.start || orig.end != avail.end) {
    pr_info("resource: avoiding allocation from e820 entry [mem %#010Lx-%#010Lx]\n",
    e820_start, e820_end);
    if (avail.end > avail.start)
//
// Use %pa instead of %pR because "avail"
// is typically IORESOURCE_UNSET, so %pR
// shows the size instead of addresses.
//
    pr_info("resource: remaining [mem %pa-%pa] available\n",
    &avail.start, &avail.end);
    orig = *avail;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn arch_remove_reservations(avail: *mut resource) {
    void arch_remove_reservations(struct resource *avail)
    {
//
// Trim out BIOS area (high 2MB) and E820 regions. We do not remove
// the low 1MB unconditionally, as this area is needed for some ISA
// cards requiring a memory range.
//
    if (avail.flags & IORESOURCE_MEM) {
    resource_clip(avail, BIOS_ROM_BASE, BIOS_ROM_END);
    remove_e820_regions(avail);
    }
    }
