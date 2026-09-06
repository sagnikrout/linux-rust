//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/uapi/asm/bootparam.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// ram_size flags
pub const RAMDISK_IMAGE_START_MASK: c_uint = 0x07FF;
pub const RAMDISK_PROMPT_FLAG: c_uint = 0x8000;
pub const RAMDISK_LOAD_FLAG: c_uint = 0x4000;
// loadflags

// xloadflags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct setup_header {
    pub setup_sects: __u8,
    pub root_flags: __u16,
    pub syssize: __u32,
    pub ram_size: __u16,
    pub vid_mode: __u16,
    pub root_dev: __u16,
    pub boot_flag: __u16,
    pub jump: __u16,
    pub header: __u32,
    pub version: __u16,
    pub realmode_swtch: __u32,
    pub start_sys_seg: __u16,
    pub kernel_version: __u16,
    pub type_of_loader: __u8,
    pub loadflags: __u8,
    pub setup_move_size: __u16,
    pub code32_start: __u32,
    pub ramdisk_image: __u32,
    pub ramdisk_size: __u32,
    pub bootsect_kludge: __u32,
    pub heap_end_ptr: __u16,
    pub ext_loader_ver: __u8,
    pub ext_loader_type: __u8,
    pub cmd_line_ptr: __u32,
    pub initrd_addr_max: __u32,
    pub kernel_alignment: __u32,
    pub relocatable_kernel: __u8,
    pub min_alignment: __u8,
    pub xloadflags: __u16,
    pub cmdline_size: __u32,
    pub hardware_subarch: __u32,
    pub hardware_subarch_data: __u64,
    pub payload_offset: __u32,
    pub payload_length: __u32,
    pub setup_data: __u64,
    pub pref_address: __u64,
    pub init_size: __u32,
    pub handover_offset: __u32,
    pub kernel_info_offset: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_desc_table {
    pub length: __u16,
    pub table: [__u8; 14],
}

// Gleaned from OFW's set-parameters in cpu/x86/pc/linux.fth
#[repr(C)]
#[derive(Copy, Clone)]
pub struct olpc_ofw_header {
    pub /: *mut *mut __u32 ofw_magic; / OFW signature,
    pub ofw_version: __u32,
    pub /: *mut *mut __u32 cif_handler; / callback into OFW,
    pub irq_desc_table: __u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct efi_info {
    pub efi_loader_signature: __u32,
    pub efi_systab: __u32,
    pub efi_memdesc_size: __u32,
    pub efi_memdesc_version: __u32,
    pub efi_memmap: __u32,
    pub efi_memmap_size: __u32,
    pub efi_systab_hi: __u32,
    pub efi_memmap_hi: __u32,
}

//
// This is the maximum number of entries in struct boot_params::e820_table
// (the zeropage), which is part of the x86 boot protocol ABI:
//
pub const E820_MAX_ENTRIES_ZEROPAGE: c_int = 128;
//
// Smallest compatible version of jailhouse_setup_data required by this kernel.
//
pub const JAILHOUSE_SETUP_REQUIRED_VERSION: c_int = 1;
// The so-called "zeropage"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct boot_params {
    pub /: *mut *mut screen_info screen_info; / 0x000,
    pub /: *mut *mut apm_bios_info apm_bios_info; / 0x040,
    pub /: *mut *mut __u8 _pad2[4]; / 0x054,
    pub /: *mut *mut __u64 tboot_addr; / 0x058,
    pub /: *mut *mut ist_info ist_info; / 0x060,
    pub /: *mut *mut __u64 acpi_rsdp_addr; / 0x070,
    pub /: *mut *mut __u8 _pad3[8]; / 0x078,
    pub /: *mut *mut *mut *mut __u8 hd0_info[16]; / obsolete! / / 0x080,
    pub /: *mut *mut *mut *mut __u8 hd1_info[16]; / obsolete! / / 0x090,
    pub /: *mut *mut *mut *mut sys_desc_table sys_desc_table; / obsolete! / / 0x0a0,
    pub /: *mut *mut olpc_ofw_header olpc_ofw_header; / 0x0b0,
    pub /: *mut *mut __u32 ext_ramdisk_image; / 0x0c0,
    pub /: *mut *mut __u32 ext_ramdisk_size; / 0x0c4,
    pub /: *mut *mut __u32 ext_cmd_line_ptr; / 0x0c8,
    pub /: *mut *mut __u8 _pad4[112]; / 0x0cc,
    pub /: *mut *mut __u32 cc_blob_address; / 0x13c,
    pub /: *mut *mut edid_info edid_info; / 0x140,
    pub /: *mut *mut efi_info efi_info; / 0x1c0,
    pub /: *mut *mut __u32 alt_mem_k; / 0x1e0,
    pub /: *mut *mut *mut *mut __u32 scratch; / Scratch field! / / 0x1e4,
    pub /: *mut *mut __u8 e820_entries; / 0x1e8,
    pub /: *mut *mut __u8 eddbuf_entries; / 0x1e9,
    pub /: *mut *mut __u8 edd_mbr_sig_buf_entries; / 0x1ea,
    pub /: *mut *mut __u8 kbd_status; / 0x1eb,
    pub /: *mut *mut __u8 secure_boot; / 0x1ec,
    pub /: *mut *mut __u8 _pad5[2]; / 0x1ed,
//
// The sentinel is set to a nonzero value (0xff) in header.S.
//
// A bootloader is supposed to only take setup_header and put
// it into a clean boot_params buffer. If it turns out that
// it is clumsy or too generous with the buffer, it most
// probably will pick up the sentinel variable too. The fact
// that this variable then is still 0xff will let kernel
// know that some variables in boot_params are invalid and
// kernel should zero out certain portions of boot_params.
//
    pub /: *mut *mut __u8 sentinel; / 0x1ef,
    pub /: *mut *mut __u8 _pad6[1]; / 0x1f0,
    pub /: *mut *mut *mut *mut setup_header hdr; / setup header / / 0x1f1,
    pub setup_header)]: __u8 _pad7[0x290-0x1f1-sizeof(struct,
    pub /: *mut *mut __u32 edd_mbr_sig_buffer[EDD_MBR_SIG_MAX]; / 0x290,
    pub /: *mut *mut boot_e820_entry e820_table[E820_MAX_ENTRIES_ZEROPAGE]; / 0x2d0,
    pub /: *mut *mut __u8 _pad8[48]; / 0xcd0,
    pub /: *mut *mut edd_info eddbuf[EDDMAXNR]; / 0xd00,
    pub /: *mut *mut __u8 _pad9[276]; / 0xeec,
    pub __attribute__((packed)): },
//
// enum x86_hardware_subarch - x86 hardware subarchitecture
//
// The x86 hardware_subarch and hardware_subarch_data were added as of the x86
// boot protocol 2.07 to help distinguish and support custom x86 boot
// sequences. This enum represents accepted values for the x86
// hardware_subarch.  Custom x86 boot sequences (not X86_SUBARCH_PC) do not
// have or simply *cannot* make use of natural stubs like BIOS or EFI, the
// hardware_subarch can be used on the Linux entry path to revector to a
// subarchitecture stub when needed. This subarchitecture stub can be used to
// set up Linux boot parameters or for special care to account for nonstandard
// handling of page tables.
//
// These enums should only ever be used by x86 code, and the code that uses
// it should be well contained and compartmentalized.
//
// KVM and Xen HVM do not have a subarch as these are expected to follow
// standard x86 boot entries. If there is a genuine need for "hypervisor" type
// that should be considered separately in the future. Future guest types
// should seriously consider working with standard x86 boot stubs such as
// the BIOS or EFI boot stubs.
//
// WARNING: this enum is only used for legacy hacks, for platform features that
// are not easily enumerated or discoverable. You should not ever use
// this for new features.
//
// @X86_SUBARCH_PC: Should be used if the hardware is enumerable using standard
// PC mechanisms (PCI, ACPI) and doesn't need a special boot flow.
// @X86_SUBARCH_LGUEST: Used for x86 hypervisor demo, lguest, deprecated
// @X86_SUBARCH_XEN: Used for Xen guest types which follow the PV boot path,
// which start at asm startup_xen() entry point and later jump to the C
// xen_start_kernel() entry point. Both domU and dom0 type of guests are
// currently supported through this PV boot path.
// @X86_SUBARCH_INTEL_MID: Used for Intel MID (Mobile Internet Device) platform
// systems which do not have the PCI legacy interfaces.
// @X86_SUBARCH_CE4100: Used for Intel CE media processor (CE4100) SoC
// for settop boxes and media devices, the use of a subarch for CE4100
// is more of a hack...
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_hardware_subarch {
    X86_SUBARCH_PC = 0,
    X86_SUBARCH_LGUEST,
    X86_SUBARCH_XEN,
    X86_SUBARCH_INTEL_MID,
    X86_SUBARCH_CE4100,
    X86_NR_SUBARCHS,
}

