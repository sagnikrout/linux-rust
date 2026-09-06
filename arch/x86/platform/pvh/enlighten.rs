//! Automatically rewritten from C to Rust
//! Source: arch/x86/platform/pvh/enlighten.c
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
// PVH variables.
//
// pvh_bootparams and pvh_start_info need to live in a data segment since
// they are used after startup_{32|64}, which clear .bss, are invoked.
//
    struct boot_params __initdata pvh_bootparams;
    struct hvm_start_info __initdata pvh_start_info;
    let mut pvh_start_info_sz: unsigned int __initconst = sizeof(pvh_start_info);
//
// Xen guests are able to obtain the memory map from the hypervisor via the
// HYPERVISOR_memory_op hypercall.
// If we are trying to boot a Xen PVH guest, it is expected that the kernel
// will have been configured to provide an override for this routine to do
// just that.
//
#[no_mangle]
pub unsafe extern "C" fn mem_map_via_hcall(__maybe_unused: *mut *mut boot_params ptr) -> void __init __weak {
    void __init __weak mem_map_via_hcall(struct boot_params *ptr __maybe_unused)
    {
    xen_raw_printk("Error: Could not find memory map\n");
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn init_pvh_bootparams(xen_guest: bool) -> void __init {
    static void __init init_pvh_bootparams(bool xen_guest)
    {
    if ((pvh_start_info.version > 0) && (pvh_start_info.memmap_entries)) {
    struct hvm_memmap_table_entry *ep;
    int i;
    ep = __va(pvh_start_info.memmap_paddr);
    pvh_bootparams.e820_entries = pvh_start_info.memmap_entries;
    for (i = 0; i < pvh_bootparams.e820_entries ; i++, ep++) {
    pvh_bootparams.e820_table[i].addr = ep.addr;
    pvh_bootparams.e820_table[i].size = ep.size;
    pvh_bootparams.e820_table[i].type = ep.type;
    }
    } else if (xen_guest) {
    mem_map_via_hcall(&pvh_bootparams);
    } else {
// Non-xen guests are not supported by version 0
    BUG();
    }
    if (pvh_bootparams.e820_entries < E820_MAX_ENTRIES_ZEROPAGE - 1) {
    pvh_bootparams.e820_table[pvh_bootparams.e820_entries].addr =
    ISA_START_ADDRESS;
    pvh_bootparams.e820_table[pvh_bootparams.e820_entries].size =
    ISA_END_ADDRESS - ISA_START_ADDRESS;
    pvh_bootparams.e820_table[pvh_bootparams.e820_entries].type =
    E820_TYPE_RESERVED;
    pvh_bootparams.e820_entries++;
    } else
    xen_raw_printk("Warning: Can fit ISA range into e820\n");
    pvh_bootparams.hdr.cmd_line_ptr =
    pvh_start_info.cmdline_paddr;
// The first module is always ramdisk.
    if (pvh_start_info.nr_modules) {
    struct hvm_modlist_entry *modaddr =
    __va(pvh_start_info.modlist_paddr);
    pvh_bootparams.hdr.ramdisk_image = modaddr.paddr;
    pvh_bootparams.hdr.ramdisk_size = modaddr.size;
    }
//
// See Documentation/arch/x86/boot.rst.
//
// Version 2.12 supports Xen entry point but we will use default x86/PC
// environment (i.e. hardware_subarch 0).
//
    pvh_bootparams.hdr.version = (2 << 8) | 12;
    pvh_bootparams.hdr.type_of_loader = ((xen_guest ? 0x9 : 0xb) << 4) | 0;
    pvh_bootparams.acpi_rsdp_addr = pvh_start_info.rsdp_paddr;
    }
//
// If we are trying to boot a Xen PVH guest, it is expected that the kernel
// will have been configured to provide the required override for this routine.
//
#[no_mangle]
pub unsafe extern "C" fn xen_pvh_init(boot_params: *mut boot_params) -> void __init __weak {
    void __init __weak xen_pvh_init(struct boot_params *boot_params)
    {
    xen_raw_printk("Error: Missing xen PVH initialization\n");
    BUG();
    }
#[no_mangle]
unsafe extern "C" fn hypervisor_specific_init(xen_guest: bool) -> void __init {
    static void __init hypervisor_specific_init(bool xen_guest)
    {
    if (xen_guest)
    xen_pvh_init(&pvh_bootparams);
    }
//
// This routine (and those that it might call) should not use
// anything that lives in .bss since that segment will be cleared later.
//
#[no_mangle]
pub unsafe extern "C" fn xen_prepare_pvh() -> void __init {
    void __init xen_prepare_pvh(void)
    {
    let mut msr: u32 = xen_cpuid_base();
    let mut xen_guest: bool = !!msr;
    if (pvh_start_info.magic != XEN_HVM_START_MAGIC_VALUE) {
    xen_raw_printk("Error: Unexpected magic value (0x%08x)\n",
    pvh_start_info.magic);
    BUG();
    }
//
// This must not compile to "call memset" because memset() may be
// instrumented.
//
    __builtin_memset(&pvh_bootparams, 0, sizeof(pvh_bootparams));
    hypervisor_specific_init(xen_guest);
    init_pvh_bootparams(xen_guest);
    }
