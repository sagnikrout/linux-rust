//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pe.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright 2011 Red Hat, Inc.
// All rights reserved.
//
// Author(s): Peter Jones <pjones@redhat.com>
//

//
// Starting from version v3.0, the major version field should be interpreted as
// a bit mask of features supported by the kernel's EFI stub:
// - 0x1: initrd loading from the LINUX_EFI_INITRD_MEDIA_GUID device path,
// - 0x2: initrd loading using the initrd= command line option, where the file
// may be specified using device path notation, and is not required to
// reside on the same volume as the loaded kernel image.
//
// The recommended way of loading and starting v1.0 or later kernels is to use
// the LoadImage() and StartImage() EFI boot services, and expose the initrd
// via the LINUX_EFI_INITRD_MEDIA_GUID device path.
//
// Versions older than v1.0 may support initrd loading via the image load
// options (using initrd=, limited to the volume from which the kernel itself
// was loaded), or only via arch specific means (bootparams, DT, etc).
//
// The minor version field must remain 0x0.
// (https://lore.kernel.org/all/efd6f2d4-547c-1378-1faa-53c044dbd297@gmail.com/)
//
pub const LINUX_EFISTUB_MAJOR_VERSION: c_uint = 0x3;
pub const LINUX_EFISTUB_MINOR_VERSION: c_uint = 0x0;
//
// LINUX_PE_MAGIC appears at offset 0x38 into the MS-DOS header of EFI bootable
// Linux kernel images that target the architecture as specified by the PE/COFF
// header machine type field.
//
pub const LINUX_PE_MAGIC: c_uint = 0x818223cd;
pub const IMAGE_DOS_SIGNATURE: c_uint = 0x5a4d /* "MZ" */;
pub const IMAGE_NT_SIGNATURE: c_uint = 0x00004550 /* "PE\0\0" */;
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: c_uint = 0x0107 /* ROM image (for R3000/R4000/R10000/ALPHA), without MZ and PE\0\0 sign */;
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: c_uint = 0x010b /* PE32 executable image */;
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: c_uint = 0x020b /* PE32+ executable image */;
// machine type
pub const IMAGE_FILE_MACHINE_UNKNOWN: c_uint = 0x0000 /* Unknown architecture */;
pub const IMAGE_FILE_MACHINE_TARGET_HOST: c_uint = 0x0001 /* Interacts with the host and not a WOW64 guest (not for file image) */;
pub const IMAGE_FILE_MACHINE_ALPHA_OLD: c_uint = 0x0183 /* DEC Alpha AXP 32-bit (old images) */;
pub const IMAGE_FILE_MACHINE_ALPHA: c_uint = 0x0184 /* DEC Alpha AXP 32-bit */;
pub const IMAGE_FILE_MACHINE_ALPHA64: c_uint = 0x0284 /* DEC Alpha AXP 64-bit (with 8kB page size) */;

pub const IMAGE_FILE_MACHINE_AM33: c_uint = 0x01d3 /* Matsushita AM33, now Panasonic MN103 */;
pub const IMAGE_FILE_MACHINE_AMD64: c_uint = 0x8664 /* AMD64 (x64) */;
pub const IMAGE_FILE_MACHINE_ARM: c_uint = 0x01c0 /* ARM Little-Endian (ARMv4) */;
pub const IMAGE_FILE_MACHINE_THUMB: c_uint = 0x01c2 /* ARM Thumb Little-Endian (ARMv4T) */;
pub const IMAGE_FILE_MACHINE_ARMNT: c_uint = 0x01c4 /* ARM Thumb-2 Little-Endian (ARMv7) */;

pub const IMAGE_FILE_MACHINE_ARM64: c_uint = 0xaa64 /* ARM64 Little-Endian (Classic ABI) */;
pub const IMAGE_FILE_MACHINE_ARM64EC: c_uint = 0xa641 /* ARM64 Little-Endian (Emulation Compatible ABI for AMD64) */;
pub const IMAGE_FILE_MACHINE_ARM64X: c_uint = 0xa64e /* ARM64 Little-Endian (fat binary with both Classic ABI and EC ABI code) */;
pub const IMAGE_FILE_MACHINE_CEE: c_uint = 0xc0ee /* COM+ Execution Engine (CLR pure MSIL object files) */;
pub const IMAGE_FILE_MACHINE_CEF: c_uint = 0x0cef /* Windows CE 3.0 Common Executable Format (CEF bytecode) */;
pub const IMAGE_FILE_MACHINE_CHPE_X86: c_uint = 0x3a64 /* ARM64 Little-Endian (Compiled Hybrid PE ABI for I386) */;

pub const IMAGE_FILE_MACHINE_EBC: c_uint = 0x0ebc /* EFI/UEFI Byte Code */;
pub const IMAGE_FILE_MACHINE_I386: c_uint = 0x014c /* Intel 386 (x86) */;
pub const IMAGE_FILE_MACHINE_I860: c_uint = 0x014d /* Intel 860 (N10) */;
pub const IMAGE_FILE_MACHINE_IA64: c_uint = 0x0200 /* Intel IA-64 (with 8kB page size) */;
pub const IMAGE_FILE_MACHINE_LOONGARCH32: c_uint = 0x6232 /* LoongArch 32-bit processor family */;
pub const IMAGE_FILE_MACHINE_LOONGARCH64: c_uint = 0x6264 /* LoongArch 64-bit processor family */;
pub const IMAGE_FILE_MACHINE_M32R: c_uint = 0x9041 /* Mitsubishi M32R 32-bit Little-Endian */;
pub const IMAGE_FILE_MACHINE_M68K: c_uint = 0x0268 /* Motorola 68000 series */;
pub const IMAGE_FILE_MACHINE_MIPS16: c_uint = 0x0266 /* MIPS III with MIPS16 ASE Little-Endian */;
pub const IMAGE_FILE_MACHINE_MIPSFPU: c_uint = 0x0366 /* MIPS III with FPU Little-Endian */;
pub const IMAGE_FILE_MACHINE_MIPSFPU16: c_uint = 0x0466 /* MIPS III with MIPS16 ASE and FPU Little-Endian */;
pub const IMAGE_FILE_MACHINE_MPPC_601: c_uint = 0x0601 /* PowerPC 32-bit Big-Endian */;
pub const IMAGE_FILE_MACHINE_OMNI: c_uint = 0xace1 /* Microsoft OMNI VM (omniprox.dll) */;
pub const IMAGE_FILE_MACHINE_PARISC: c_uint = 0x0290 /* HP PA-RISC */;
pub const IMAGE_FILE_MACHINE_POWERPC: c_uint = 0x01f0 /* PowerPC 32-bit Little-Endian */;
pub const IMAGE_FILE_MACHINE_POWERPCFP: c_uint = 0x01f1 /* PowerPC 32-bit with FPU Little-Endian */;
pub const IMAGE_FILE_MACHINE_POWERPCBE: c_uint = 0x01f2 /* PowerPC 64-bit Big-Endian */;
pub const IMAGE_FILE_MACHINE_R3000: c_uint = 0x0162 /* MIPS I Little-Endian */;
pub const IMAGE_FILE_MACHINE_R3000_BE: c_uint = 0x0160 /* MIPS I Big-Endian */;
pub const IMAGE_FILE_MACHINE_R4000: c_uint = 0x0166 /* MIPS III Little-Endian (with 1kB or 4kB page size) */;
pub const IMAGE_FILE_MACHINE_R10000: c_uint = 0x0168 /* MIPS IV Little-Endian */;
pub const IMAGE_FILE_MACHINE_RISCV32: c_uint = 0x5032 /* RISC-V 32-bit address space */;
pub const IMAGE_FILE_MACHINE_RISCV64: c_uint = 0x5064 /* RISC-V 64-bit address space */;
pub const IMAGE_FILE_MACHINE_RISCV128: c_uint = 0x5128 /* RISC-V 128-bit address space */;
pub const IMAGE_FILE_MACHINE_SH3: c_uint = 0x01a2 /* Hitachi SH-3 32-bit Little-Endian (with 1kB page size) */;
pub const IMAGE_FILE_MACHINE_SH3DSP: c_uint = 0x01a3 /* Hitachi SH-3 DSP 32-bit (with 1kB page size) */;
pub const IMAGE_FILE_MACHINE_SH3E: c_uint = 0x01a4 /* Hitachi SH-3E Little-Endian (with 1kB page size) */;
pub const IMAGE_FILE_MACHINE_SH4: c_uint = 0x01a6 /* Hitachi SH-4 32-bit Little-Endian (with 1kB page size) */;
pub const IMAGE_FILE_MACHINE_SH5: c_uint = 0x01a8 /* Hitachi SH-5 64-bit */;
pub const IMAGE_FILE_MACHINE_TAHOE: c_uint = 0x07cc /* Intel EM machine */;
pub const IMAGE_FILE_MACHINE_TRICORE: c_uint = 0x0520 /* Infineon AUDO 32-bit */;
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: c_uint = 0x0169 /* MIPS Windows CE v2 Little-Endian */;
// flags
pub const IMAGE_FILE_RELOCS_STRIPPED: c_uint = 0x0001 /* Relocation info stripped from file */;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: c_uint = 0x0002 /* File is executable (i.e. no unresolved external references) */;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: c_uint = 0x0004 /* Line nunbers stripped from file */;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: c_uint = 0x0008 /* Local symbols stripped from file */;
pub const IMAGE_FILE_AGGRESSIVE_WS_TRIM: c_uint = 0x0010 /* Aggressively trim working set */;
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: c_uint = 0x0020 /* App can handle >2gb addresses (image can be loaded at address above 2GB) */;
pub const IMAGE_FILE_16BIT_MACHINE: c_uint = 0x0040 /* 16 bit word machine */;
pub const IMAGE_FILE_BYTES_REVERSED_LO: c_uint = 0x0080 /* Bytes of machine word are reversed (should be set together with IMAGE_FILE_BYTES_REVERSED_HI) */;
pub const IMAGE_FILE_32BIT_MACHINE: c_uint = 0x0100 /* 32 bit word machine */;
pub const IMAGE_FILE_DEBUG_STRIPPED: c_uint = 0x0200 /* Debugging info stripped from file in .DBG file */;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: c_uint = 0x0400 /* If Image is on removable media, copy and run from the swap file */;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: c_uint = 0x0800 /* If Image is on Net, copy and run from the swap file */;
pub const IMAGE_FILE_SYSTEM: c_uint = 0x1000 /* System kernel-mode file (can't be loaded in user-mode) */;
pub const IMAGE_FILE_DLL: c_uint = 0x2000 /* File is a DLL */;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: c_uint = 0x4000 /* File should only be run on a UP (uniprocessor) machine */;
pub const IMAGE_FILE_BYTES_REVERSED_HI: c_uint = 0x8000 /* Bytes of machine word are reversed (should be set together with IMAGE_FILE_BYTES_REVERSED_LO) */;
// subsys

pub const IMAGE_SUBSYSTEM_RESERVED_6: c_int = 6;

pub const IMAGE_SUBSYSTEM_RESERVED_15: c_int = 15;

// dll_flags
pub const IMAGE_LIBRARY_PROCESS_INIT: c_uint = 0x0001 /* DLL initialization function called just after process initialization */;
pub const IMAGE_LIBRARY_PROCESS_TERM: c_uint = 0x0002 /* DLL initialization function called just before process termination */;
pub const IMAGE_LIBRARY_THREAD_INIT: c_uint = 0x0004 /* DLL initialization function called just after thread initialization */;
pub const IMAGE_LIBRARY_THREAD_TERM: c_uint = 0x0008 /* DLL initialization function called just before thread initialization */;
pub const IMAGE_DLLCHARACTERISTICS_RESERVED_4: c_uint = 0x0010;
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: c_uint = 0x0020 /* ASLR with 64 bit address space (image can be loaded at address above 4GB) */;
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: c_uint = 0x0040 /* The DLL can be relocated at load time */;
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: c_uint = 0x0080 /* Code integrity checks are forced */;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: c_uint = 0x0100 /* Image is compatible with data execution prevention */;
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: c_uint = 0x0200 /* Image is isolation aware, but should not be isolated (prevents loading of manifest file) */;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: c_uint = 0x0400 /* Image does not use SEH, no SE handler may reside in this image */;
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: c_uint = 0x0800 /* Do not bind the image */;
pub const IMAGE_DLLCHARACTERISTICS_X86_THUNK: c_uint = 0x1000 /* Image is a Wx86 Thunk DLL (for non-x86/risc DLL files) */;
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: c_uint = 0x1000 /* Image should execute in an AppContainer (for EXE Metro Apps in Windows 8) */;
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: c_uint = 0x2000 /* A WDM driver */;
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: c_uint = 0x4000 /* Image supports Control Flow Guard */;
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: c_uint = 0x8000 /* The image is terminal server (Remote Desktop Services) aware */;
// IMAGE_DEBUG_TYPE_EX_DLLCHARACTERISTICS flags
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT: c_uint = 0x0001 /* Image is Control-flow Enforcement Technology Shadow Stack compatible */;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT_STRICT_MODE: c_uint = 0x0002 /* CET is enforced in strict mode */;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_SET_CONTEXT_IP_VALIDATION_RELAXED_MODE: c_uint = 0x0004 /* Relaxed mode for Context IP Validation under CET is allowed */;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_DYNAMIC_APIS_ALLOW_IN_PROC: c_uint = 0x0008 /* Use of dynamic APIs is restricted to processes only */;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_1: c_uint = 0x0010;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_2: c_uint = 0x0020;
pub const IMAGE_DLLCHARACTERISTICS_EX_FORWARD_CFI_COMPAT: c_uint = 0x0040 /* All branch targets in all image code sections are annotated with forward-edge control flow integrity guard instructions */;
pub const IMAGE_DLLCHARACTERISTICS_EX_HOTPATCH_COMPATIBLE: c_uint = 0x0080 /* Image can be modified while in use, hotpatch-compatible */;
// section_header flags
pub const IMAGE_SCN_SCALE_INDEX: c_uint = 0x00000001 /* address of tls index is scaled = multiplied by 4 (for .tls section on MIPS only) */;
pub const IMAGE_SCN_TYPE_NO_LOAD: c_uint = 0x00000002 /* reserved */;
pub const IMAGE_SCN_TYPE_GROUPED: c_uint = 0x00000004 /* obsolete (used for 16-bit offset code) */;
pub const IMAGE_SCN_TYPE_NO_PAD: c_uint = 0x00000008 /* .o only - don't pad - obsolete (same as IMAGE_SCN_ALIGN_1BYTES) */;
pub const IMAGE_SCN_TYPE_COPY: c_uint = 0x00000010 /* reserved */;
pub const IMAGE_SCN_CNT_CODE: c_uint = 0x00000020 /* .text */;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: c_uint = 0x00000040 /* .data */;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: c_uint = 0x00000080 /* .bss */;
pub const IMAGE_SCN_LNK_OTHER: c_uint = 0x00000100 /* .o only - other type than code, data or info */;
pub const IMAGE_SCN_LNK_INFO: c_uint = 0x00000200 /* .o only - .drectve comments */;
pub const IMAGE_SCN_LNK_OVERLAY: c_uint = 0x00000400 /* section contains overlay */;
pub const IMAGE_SCN_LNK_REMOVE: c_uint = 0x00000800 /* .o only - scn to be rm'd*/;
pub const IMAGE_SCN_LNK_COMDAT: c_uint = 0x00001000 /* .o only - COMDAT data */;
pub const IMAGE_SCN_RESERVED_13: c_uint = 0x00002000 /* spec omits this */;
pub const IMAGE_SCN_MEM_PROTECTED: c_uint = 0x00004000 /* section is memory protected (for M68K) */;
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: c_uint = 0x00004000 /* reset speculative exceptions handling bits in the TLB entries (for non-M68K) */;
pub const IMAGE_SCN_MEM_FARDATA: c_uint = 0x00008000 /* section uses FAR_EXTERNAL relocations (for M68K) */;
pub const IMAGE_SCN_GPREL: c_uint = 0x00008000 /* global pointer referenced data (for non-M68K) */;
pub const IMAGE_SCN_MEM_SYSHEAP: c_uint = 0x00010000 /* use system heap (for M68K) */;
pub const IMAGE_SCN_MEM_PURGEABLE: c_uint = 0x00020000 /* section can be released from RAM (for M68K) */;
pub const IMAGE_SCN_MEM_16BIT: c_uint = 0x00020000 /* section is 16-bit (for non-M68K where it makes sense: I386, THUMB, MIPS16, MIPSFPU16, ...) */;
pub const IMAGE_SCN_MEM_LOCKED: c_uint = 0x00040000 /* prevent the section from being moved (for M68K and .o I386) */;
pub const IMAGE_SCN_MEM_PRELOAD: c_uint = 0x00080000 /* section is preload to RAM (for M68K and .o I386) */;
// and here they just stuck a 1-byte integer in the middle of a bitfield
pub const IMAGE_SCN_ALIGN_1BYTES: c_uint = 0x00100000 /* .o only - it does what it says on the box */;
pub const IMAGE_SCN_ALIGN_2BYTES: c_uint = 0x00200000;
pub const IMAGE_SCN_ALIGN_4BYTES: c_uint = 0x00300000;
pub const IMAGE_SCN_ALIGN_8BYTES: c_uint = 0x00400000;
pub const IMAGE_SCN_ALIGN_16BYTES: c_uint = 0x00500000;
pub const IMAGE_SCN_ALIGN_32BYTES: c_uint = 0x00600000;
pub const IMAGE_SCN_ALIGN_64BYTES: c_uint = 0x00700000;
pub const IMAGE_SCN_ALIGN_128BYTES: c_uint = 0x00800000;
pub const IMAGE_SCN_ALIGN_256BYTES: c_uint = 0x00900000;
pub const IMAGE_SCN_ALIGN_512BYTES: c_uint = 0x00a00000;
pub const IMAGE_SCN_ALIGN_1024BYTES: c_uint = 0x00b00000;
pub const IMAGE_SCN_ALIGN_2048BYTES: c_uint = 0x00c00000;
pub const IMAGE_SCN_ALIGN_4096BYTES: c_uint = 0x00d00000;
pub const IMAGE_SCN_ALIGN_8192BYTES: c_uint = 0x00e00000;
pub const IMAGE_SCN_ALIGN_RESERVED: c_uint = 0x00f00000;
pub const IMAGE_SCN_ALIGN_MASK: c_uint = 0x00f00000;
pub const IMAGE_SCN_LNK_NRELOC_OVFL: c_uint = 0x01000000 /* .o only - extended relocations */;
pub const IMAGE_SCN_MEM_DISCARDABLE: c_uint = 0x02000000 /* scn can be discarded */;
pub const IMAGE_SCN_MEM_NOT_CACHED: c_uint = 0x04000000 /* cannot be cached */;
pub const IMAGE_SCN_MEM_NOT_PAGED: c_uint = 0x08000000 /* not pageable */;
pub const IMAGE_SCN_MEM_SHARED: c_uint = 0x10000000 /* can be shared */;
pub const IMAGE_SCN_MEM_EXECUTE: c_uint = 0x20000000 /* can be executed as code */;
pub const IMAGE_SCN_MEM_READ: c_uint = 0x40000000 /* readable */;
pub const IMAGE_SCN_MEM_WRITE: c_uint = 0x80000000 /* writeable */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mz_hdr {
    pub /: *mut *mut uint16_t magic; / MZ_MAGIC,
    pub /: *mut *mut uint16_t lbsize; / size of last used block,
    pub /: *mut *mut uint16_t blocks; / pages in file, 0x3,
    pub /: *mut *mut uint16_t relocs; / relocations,
    pub /: *mut *mut uint16_t hdrsize; / header size in "paragraphs",
    pub /: *mut *mut uint16_t min_extra_pps; / .bss,
    pub /: *mut *mut uint16_t max_extra_pps; / runtime limit for the arena size,
    pub /: *mut *mut uint16_t ss; / relative stack segment,
    pub /: *mut *mut uint16_t sp; / initial %sp register,
    pub /: *mut *mut uint16_t checksum; / word checksum,
    pub /: *mut *mut uint16_t ip; / initial %ip register,
    pub /: *mut *mut uint16_t cs; / initial %cs relative to load segment,
    pub /: *mut *mut uint16_t reloc_table_offset; / offset of the first relocation,
    pub /: *mut *mut uint16_t overlay_num; / overlay number. set to 0.,
    pub /: *mut *mut uint16_t reserved0[4]; / reserved,
    pub /: *mut *mut uint16_t oem_id; / oem identifier,
    pub /: *mut *mut uint16_t oem_info; / oem specific,
    pub /: *mut *mut uint16_t reserved1[10]; / reserved,
    pub /: *mut *mut uint32_t peaddr; / address of pe header,
    pub /: *mut *mut char message[]; / message to print,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mz_reloc {
    pub offset: u16,
    pub segment: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pe_hdr {
    pub /: *mut *mut uint32_t magic; / PE magic,
    pub /: *mut *mut uint16_t machine; / machine type,
    pub /: *mut *mut uint16_t sections; / number of sections,
    pub /: *mut *mut uint32_t timestamp; / time_t,
    pub /: *mut *mut uint32_t symbol_table; / symbol table offset,
    pub /: *mut *mut uint32_t symbols; / number of symbols,
    pub /: *mut *mut uint16_t opt_hdr_size; / size of optional header,
    pub /: *mut *mut uint16_t flags; / flags,
}

// the fact that pe32 isn't padded where pe32+ is 64-bit means union won't
// work right.  vomit.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pe32_opt_hdr {
// "standard" header
    pub /: *mut *mut uint16_t magic; / file type,
    pub /: *mut *mut uint8_t ld_major; / linker major version,
    pub /: *mut *mut uint8_t ld_minor; / linker minor version,
    pub /: *mut *mut uint32_t text_size; / size of text section(s),
    pub /: *mut *mut uint32_t data_size; / size of data section(s),
    pub /: *mut *mut uint32_t bss_size; / size of bss section(s),
    pub /: *mut *mut uint32_t entry_point; / file offset of entry point,
    pub /: *mut *mut uint32_t code_base; / relative code addr in ram,
    pub /: *mut *mut uint32_t data_base; / relative data addr in ram,
// "windows" header
    pub /: *mut *mut uint32_t image_base; / preferred load address,
    pub /: *mut *mut uint32_t section_align; / alignment in bytes,
    pub /: *mut *mut uint32_t file_align; / file alignment in bytes,
    pub /: *mut *mut uint16_t os_major; / major OS version,
    pub /: *mut *mut uint16_t os_minor; / minor OS version,
    pub /: *mut *mut uint16_t image_major; / major image version,
    pub /: *mut *mut uint16_t image_minor; / minor image version,
    pub /: *mut *mut uint16_t subsys_major; / major subsystem version,
    pub /: *mut *mut uint16_t subsys_minor; / minor subsystem version,
    pub /: *mut *mut uint32_t win32_version; / win32 version reported at runtime,
    pub /: *mut *mut uint32_t image_size; / image size,
    pub to: *mut *mut uint32_t header_size; / header size rounded up,
    pub /: *mut *mut uint32_t csum; / checksum,
    pub /: *mut *mut uint16_t subsys; / subsystem,
    pub /: *mut *mut uint16_t dll_flags; / more flags!,
    pub /: *mut *mut uint32_t stack_size_req;/ amt of stack requested,
    pub /: *mut *mut uint32_t stack_size; / amt of stack required,
    pub /: *mut *mut uint32_t heap_size_req; / amt of heap requested,
    pub /: *mut *mut uint32_t heap_size; / amt of heap required,
    pub /: *mut *mut uint32_t loader_flags; / loader flags,
    pub /: *mut *mut uint32_t data_dirs; / number of data dir entries,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pe32plus_opt_hdr {
    pub /: *mut *mut uint16_t magic; / file type,
    pub /: *mut *mut uint8_t ld_major; / linker major version,
    pub /: *mut *mut uint8_t ld_minor; / linker minor version,
    pub /: *mut *mut uint32_t text_size; / size of text section(s),
    pub /: *mut *mut uint32_t data_size; / size of data section(s),
    pub /: *mut *mut uint32_t bss_size; / size of bss section(s),
    pub /: *mut *mut uint32_t entry_point; / file offset of entry point,
    pub /: *mut *mut uint32_t code_base; / relative code addr in ram,
// "windows" header
    pub /: *mut *mut uint64_t image_base; / preferred load address,
    pub /: *mut *mut uint32_t section_align; / alignment in bytes,
    pub /: *mut *mut uint32_t file_align; / file alignment in bytes,
    pub /: *mut *mut uint16_t os_major; / major OS version,
    pub /: *mut *mut uint16_t os_minor; / minor OS version,
    pub /: *mut *mut uint16_t image_major; / major image version,
    pub /: *mut *mut uint16_t image_minor; / minor image version,
    pub /: *mut *mut uint16_t subsys_major; / major subsystem version,
    pub /: *mut *mut uint16_t subsys_minor; / minor subsystem version,
    pub /: *mut *mut uint32_t win32_version; / win32 version reported at runtime,
    pub /: *mut *mut uint32_t image_size; / image size,
    pub to: *mut *mut uint32_t header_size; / header size rounded up,
    pub /: *mut *mut uint32_t csum; / checksum,
    pub /: *mut *mut uint16_t subsys; / subsystem,
    pub /: *mut *mut uint16_t dll_flags; / more flags!,
    pub /: *mut *mut uint64_t stack_size_req;/ amt of stack requested,
    pub /: *mut *mut uint64_t stack_size; / amt of stack required,
    pub /: *mut *mut uint64_t heap_size_req; / amt of heap requested,
    pub /: *mut *mut uint64_t heap_size; / amt of heap required,
    pub /: *mut *mut uint32_t loader_flags; / loader flags,
    pub /: *mut *mut uint32_t data_dirs; / number of data dir entries,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_dirent {
    pub /: *mut *mut uint32_t virtual_address; / relative to load address,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data_directory {
    pub /: *mut *mut data_dirent exports; / .edata,
    pub /: *mut *mut data_dirent imports; / .idata,
    pub /: *mut *mut data_dirent resources; / .rsrc,
    pub /: *mut *mut data_dirent exceptions; / .pdata,
    pub /: *mut *mut data_dirent certs; / certs,
    pub /: *mut *mut data_dirent base_relocations; / .reloc,
    pub /: *mut *mut data_dirent debug; / .debug,
    pub /: *mut *mut data_dirent arch; / reservered,
    pub /: *mut *mut data_dirent global_ptr; / global pointer reg. Size=0,
    pub /: *mut *mut data_dirent tls; / .tls,
    pub /: *mut *mut data_dirent load_config; / load configuration structure,
    pub /: *mut *mut data_dirent bound_imports; / bound import table,
    pub /: *mut *mut data_dirent import_addrs; / import address table,
    pub /: *mut *mut data_dirent delay_imports; / delay-load import table,
    pub /: *mut *mut data_dirent clr_runtime_hdr; / .cor (clr/.net executables),
    pub reserved: data_dirent,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct section_header {
    pub /: *mut *mut char name[8]; / name or "/12\0" string tbl offset,
    pub /: *mut *mut uint32_t virtual_size; / size of loaded section in ram,
    pub /: *mut *mut uint32_t virtual_address; / relative virtual address,
    pub /: *mut *mut uint32_t raw_data_size; / size of the section,
    pub /: *mut *mut uint32_t data_addr; / file pointer to first page of sec,
    pub /: *mut *mut uint32_t relocs; / file pointer to relocation entries,
    pub /: *mut *mut uint32_t line_numbers; / line numbers!,
    pub /: *mut *mut uint16_t num_relocs; / number of relocations,
    pub /: *mut *mut uint16_t num_lin_numbers; / srsly.,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x64_coff_reloc_type {
    IMAGE_REL_AMD64_ABSOLUTE = 0,
    IMAGE_REL_AMD64_ADDR64,
    IMAGE_REL_AMD64_ADDR32,
    IMAGE_REL_AMD64_ADDR32N,
    IMAGE_REL_AMD64_REL32,
    IMAGE_REL_AMD64_REL32_1,
    IMAGE_REL_AMD64_REL32_2,
    IMAGE_REL_AMD64_REL32_3,
    IMAGE_REL_AMD64_REL32_4,
    IMAGE_REL_AMD64_REL32_5,
    IMAGE_REL_AMD64_SECTION,
    IMAGE_REL_AMD64_SECREL,
    IMAGE_REL_AMD64_SECREL7,
    IMAGE_REL_AMD64_TOKEN,
    IMAGE_REL_AMD64_SREL32,
    IMAGE_REL_AMD64_PAIR,
    IMAGE_REL_AMD64_SSPAN32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arm_coff_reloc_type {
    IMAGE_REL_ARM_ABSOLUTE,
    IMAGE_REL_ARM_ADDR32,
    IMAGE_REL_ARM_ADDR32N,
    IMAGE_REL_ARM_BRANCH2,
    IMAGE_REL_ARM_BRANCH1,
    IMAGE_REL_ARM_SECTION,
    IMAGE_REL_ARM_SECREL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sh_coff_reloc_type {
    IMAGE_REL_SH3_ABSOLUTE,
    IMAGE_REL_SH3_DIRECT16,
    IMAGE_REL_SH3_DIRECT32,
    IMAGE_REL_SH3_DIRECT8,
    IMAGE_REL_SH3_DIRECT8_WORD,
    IMAGE_REL_SH3_DIRECT8_LONG,
    IMAGE_REL_SH3_DIRECT4,
    IMAGE_REL_SH3_DIRECT4_WORD,
    IMAGE_REL_SH3_DIRECT4_LONG,
    IMAGE_REL_SH3_PCREL8_WORD,
    IMAGE_REL_SH3_PCREL8_LONG,
    IMAGE_REL_SH3_PCREL12_WORD,
    IMAGE_REL_SH3_STARTOF_SECTION,
    IMAGE_REL_SH3_SIZEOF_SECTION,
    IMAGE_REL_SH3_SECTION,
    IMAGE_REL_SH3_SECREL,
    IMAGE_REL_SH3_DIRECT32_NB,
    IMAGE_REL_SH3_GPREL4_LONG,
    IMAGE_REL_SH3_TOKEN,
    IMAGE_REL_SHM_PCRELPT,
    IMAGE_REL_SHM_REFLO,
    IMAGE_REL_SHM_REFHALF,
    IMAGE_REL_SHM_RELLO,
    IMAGE_REL_SHM_RELHALF,
    IMAGE_REL_SHM_PAIR,
    IMAGE_REL_SHM_NOMODE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ppc_coff_reloc_type {
    IMAGE_REL_PPC_ABSOLUTE,
    IMAGE_REL_PPC_ADDR64,
    IMAGE_REL_PPC_ADDR32,
    IMAGE_REL_PPC_ADDR24,
    IMAGE_REL_PPC_ADDR16,
    IMAGE_REL_PPC_ADDR14,
    IMAGE_REL_PPC_REL24,
    IMAGE_REL_PPC_REL14,
    IMAGE_REL_PPC_ADDR32N,
    IMAGE_REL_PPC_SECREL,
    IMAGE_REL_PPC_SECTION,
    IMAGE_REL_PPC_SECREL16,
    IMAGE_REL_PPC_REFHI,
    IMAGE_REL_PPC_REFLO,
    IMAGE_REL_PPC_PAIR,
    IMAGE_REL_PPC_SECRELLO,
    IMAGE_REL_PPC_GPREL,
    IMAGE_REL_PPC_TOKEN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum x86_coff_reloc_type {
    IMAGE_REL_I386_ABSOLUTE,
    IMAGE_REL_I386_DIR16,
    IMAGE_REL_I386_REL16,
    IMAGE_REL_I386_DIR32,
    IMAGE_REL_I386_DIR32NB,
    IMAGE_REL_I386_SEG12,
    IMAGE_REL_I386_SECTION,
    IMAGE_REL_I386_SECREL,
    IMAGE_REL_I386_TOKEN,
    IMAGE_REL_I386_SECREL7,
    IMAGE_REL_I386_REL32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ia64_coff_reloc_type {
    IMAGE_REL_IA64_ABSOLUTE,
    IMAGE_REL_IA64_IMM14,
    IMAGE_REL_IA64_IMM22,
    IMAGE_REL_IA64_IMM64,
    IMAGE_REL_IA64_DIR32,
    IMAGE_REL_IA64_DIR64,
    IMAGE_REL_IA64_PCREL21B,
    IMAGE_REL_IA64_PCREL21M,
    IMAGE_REL_IA64_PCREL21F,
    IMAGE_REL_IA64_GPREL22,
    IMAGE_REL_IA64_LTOFF22,
    IMAGE_REL_IA64_SECTION,
    IMAGE_REL_IA64_SECREL22,
    IMAGE_REL_IA64_SECREL64I,
    IMAGE_REL_IA64_SECREL32,
    IMAGE_REL_IA64_DIR32NB,
    IMAGE_REL_IA64_SREL14,
    IMAGE_REL_IA64_SREL22,
    IMAGE_REL_IA64_SREL32,
    IMAGE_REL_IA64_UREL32,
    IMAGE_REL_IA64_PCREL60X,
    IMAGE_REL_IA64_PCREL60B,
    IMAGE_REL_IA64_PCREL60F,
    IMAGE_REL_IA64_PCREL60I,
    IMAGE_REL_IA64_PCREL60M,
    IMAGE_REL_IA64_IMMGPREL6,
    IMAGE_REL_IA64_TOKEN,
    IMAGE_REL_IA64_GPREL32,
    IMAGE_REL_IA64_ADDEND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coff_reloc {
    pub virtual_address: u32,
    pub symbol_table_index: u32,
    pub x64_type: x64_coff_reloc_type,
    pub arm_type: arm_coff_reloc_type,
    pub sh_type: sh_coff_reloc_type,
    pub ppc_type: ppc_coff_reloc_type,
    pub x86_type: x86_coff_reloc_type,
    pub ia64_type: ia64_coff_reloc_type,
    pub data: u16,
}

//
// Definitions for the contents of the certs data block
//
pub const WIN_CERT_TYPE_PKCS_SIGNED_DATA: c_uint = 0x0002;
pub const WIN_CERT_TYPE_EFI_OKCS115: c_uint = 0x0EF0;
pub const WIN_CERT_TYPE_EFI_GUID: c_uint = 0x0EF1;
pub const WIN_CERT_REVISION_1_0: c_uint = 0x0100;
pub const WIN_CERT_REVISION_2_0: c_uint = 0x0200;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct win_certificate {
    pub length: u32,
    pub revision: u16,
    pub cert_type: u16,
}

