//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/elf.h
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

// 32-bit ELF base types.
pub type Elf32_Addr = __u32;
pub type Elf32_Half = __u16;
pub type Elf32_Off = __u32;
pub type Elf32_Sword = __s32;
pub type Elf32_Word = __u32;
pub type Elf32_Versym = __u16;
// 64-bit ELF base types.
pub type Elf64_Addr = __u64;
pub type Elf64_Half = __u16;
pub type Elf64_SHalf = __s16;
pub type Elf64_Off = __u64;
pub type Elf64_Sword = __s32;
pub type Elf64_Word = __u32;
pub type Elf64_Xword = __u64;
pub type Elf64_Sxword = __s64;
pub type Elf64_Versym = __u16;
// These constants are for the segment types stored in the image headers
pub const PT_NULL: c_int = 0;
pub const PT_LOAD: c_int = 1;
pub const PT_DYNAMIC: c_int = 2;
pub const PT_INTERP: c_int = 3;
pub const PT_NOTE: c_int = 4;
pub const PT_SHLIB: c_int = 5;
pub const PT_PHDR: c_int = 6;

pub const PT_LOOS: c_uint = 0x60000000      /* OS-specific */;
pub const PT_HIOS: c_uint = 0x6fffffff      /* OS-specific */;
pub const PT_LOPROC: c_uint = 0x70000000;
pub const PT_HIPROC: c_uint = 0x7fffffff;

// ARM MTE memory tag segment type

//
// Extended Numbering
//
// If the real number of program header table entries is larger than
// or equal to PN_XNUM(0xffff), it is set to sh_info field of the
// section header at index 0, and PN_XNUM is set to e_phnum
// field. Otherwise, the section header at index 0 is zero
// initialized, if it exists.
//
// Specifications are available in:
//
// - Oracle: Linker and Libraries.
// Part No: 817–1984–19, August 2011.
// https://docs.oracle.com/cd/E18752_01/pdf/817-1984.pdf
//
// - System V ABI AMD64 Architecture Processor Supplement
// Draft Version 0.99.4,
// January 13, 2010.
// http://www.cs.washington.edu/education/courses/cse351/12wi/supp-docs/abi.pdf
//
pub const PN_XNUM: c_uint = 0xffff;
// These constants define the different elf file types
pub const ET_NONE: c_int = 0;
pub const ET_REL: c_int = 1;
pub const ET_EXEC: c_int = 2;
pub const ET_DYN: c_int = 3;
pub const ET_CORE: c_int = 4;
pub const ET_LOPROC: c_uint = 0xff00;
pub const ET_HIPROC: c_uint = 0xffff;
// This is the info that is needed to parse the dynamic section of the file
pub const DT_NULL: c_int = 0;
pub const DT_NEEDED: c_int = 1;
pub const DT_PLTRELSZ: c_int = 2;
pub const DT_PLTGOT: c_int = 3;
pub const DT_HASH: c_int = 4;
pub const DT_STRTAB: c_int = 5;
pub const DT_SYMTAB: c_int = 6;
pub const DT_RELA: c_int = 7;
pub const DT_RELASZ: c_int = 8;
pub const DT_RELAENT: c_int = 9;
pub const DT_STRSZ: c_int = 10;
pub const DT_SYMENT: c_int = 11;
pub const DT_INIT: c_int = 12;
pub const DT_FINI: c_int = 13;
pub const DT_SONAME: c_int = 14;
pub const DT_RPATH: c_int = 15;
pub const DT_SYMBOLIC: c_int = 16;
pub const DT_REL: c_int = 17;
pub const DT_RELSZ: c_int = 18;
pub const DT_RELENT: c_int = 19;
pub const DT_PLTREL: c_int = 20;
pub const DT_DEBUG: c_int = 21;
pub const DT_TEXTREL: c_int = 22;
pub const DT_JMPREL: c_int = 23;
pub const DT_ENCODING: c_int = 32;
pub const OLD_DT_LOOS: c_uint = 0x60000000;
pub const DT_LOOS: c_uint = 0x6000000d;
pub const DT_HIOS: c_uint = 0x6ffff000;
pub const DT_VALRNGLO: c_uint = 0x6ffffd00;
pub const DT_VALRNGHI: c_uint = 0x6ffffdff;
pub const DT_ADDRRNGLO: c_uint = 0x6ffffe00;
pub const DT_GNU_HASH: c_uint = 0x6ffffef5;
pub const DT_ADDRRNGHI: c_uint = 0x6ffffeff;
pub const DT_VERSYM: c_uint = 0x6ffffff0;
pub const DT_RELACOUNT: c_uint = 0x6ffffff9;
pub const DT_RELCOUNT: c_uint = 0x6ffffffa;
pub const DT_FLAGS_1: c_uint = 0x6ffffffb;
pub const DT_VERDEF: c_uint = 0x6ffffffc;
pub const DT_VERDEFNUM: c_uint = 0x6ffffffd;
pub const DT_VERNEED: c_uint = 0x6ffffffe;
pub const DT_VERNEEDNUM: c_uint = 0x6fffffff;
pub const OLD_DT_HIOS: c_uint = 0x6fffffff;
pub const DT_LOPROC: c_uint = 0x70000000;
pub const DT_HIPROC: c_uint = 0x7fffffff;
// This info is needed when parsing the symbol table
pub const STB_LOCAL: c_int = 0;
pub const STB_GLOBAL: c_int = 1;
pub const STB_WEAK: c_int = 2;
pub const STN_UNDEF: c_int = 0;
pub const STT_NOTYPE: c_int = 0;
pub const STT_OBJECT: c_int = 1;
pub const STT_FUNC: c_int = 2;
pub const STT_SECTION: c_int = 3;
pub const STT_FILE: c_int = 4;
pub const STT_COMMON: c_int = 5;
pub const STT_TLS: c_int = 6;
pub const VER_FLG_BASE: c_uint = 0x1;
pub const VER_FLG_WEAK: c_uint = 0x2;

// The following are used with relocations

pub const EI_NIDENT: c_int = 16;
// These constants define the permissions on sections in the program
pub const PF_R: c_uint = 0x4;
pub const PF_W: c_uint = 0x2;
pub const PF_X: c_uint = 0x1;
// sh_type
pub const SHT_NULL: c_int = 0;
pub const SHT_PROGBITS: c_int = 1;
pub const SHT_SYMTAB: c_int = 2;
pub const SHT_STRTAB: c_int = 3;
pub const SHT_RELA: c_int = 4;
pub const SHT_HASH: c_int = 5;
pub const SHT_DYNAMIC: c_int = 6;
pub const SHT_NOTE: c_int = 7;
pub const SHT_NOBITS: c_int = 8;
pub const SHT_REL: c_int = 9;
pub const SHT_SHLIB: c_int = 10;
pub const SHT_DYNSYM: c_int = 11;
pub const SHT_NUM: c_int = 12;
pub const SHT_LOPROC: c_uint = 0x70000000;
pub const SHT_HIPROC: c_uint = 0x7fffffff;
pub const SHT_LOUSER: c_uint = 0x80000000;
pub const SHT_HIUSER: c_uint = 0xffffffff;
// sh_flags
pub const SHF_WRITE: c_uint = 0x1;
pub const SHF_ALLOC: c_uint = 0x2;
pub const SHF_EXECINSTR: c_uint = 0x4;
pub const SHF_MERGE: c_uint = 0x10;
pub const SHF_STRINGS: c_uint = 0x20;
pub const SHF_INFO_LINK: c_uint = 0x40;
pub const SHF_LINK_ORDER: c_uint = 0x80;
pub const SHF_OS_NONCONFORMING: c_uint = 0x100;
pub const SHF_GROUP: c_uint = 0x200;
pub const SHF_TLS: c_uint = 0x400;
pub const SHF_RELA_LIVEPATCH: c_uint = 0x00100000;
pub const SHF_RO_AFTER_INIT: c_uint = 0x00200000;
pub const SHF_ORDERED: c_uint = 0x04000000;
pub const SHF_EXCLUDE: c_uint = 0x08000000;
pub const SHF_MASKOS: c_uint = 0x0ff00000;
pub const SHF_MASKPROC: c_uint = 0xf0000000;
// special section indexes
pub const SHN_UNDEF: c_int = 0;
pub const SHN_LORESERVE: c_uint = 0xff00;
pub const SHN_LOPROC: c_uint = 0xff00;
pub const SHN_HIPROC: c_uint = 0xff1f;
pub const SHN_LIVEPATCH: c_uint = 0xff20;
pub const SHN_ABS: c_uint = 0xfff1;
pub const SHN_COMMON: c_uint = 0xfff2;
pub const SHN_HIRESERVE: c_uint = 0xffff;

pub const EI_MAG1: c_int = 1;
pub const EI_MAG2: c_int = 2;
pub const EI_MAG3: c_int = 3;
pub const EI_CLASS: c_int = 4;
pub const EI_DATA: c_int = 5;
pub const EI_VERSION: c_int = 6;
pub const EI_OSABI: c_int = 7;
pub const EI_PAD: c_int = 8;
pub const ELFMAG0: c_uint = 0x7f		/* EI_MAG */;

pub const SELFMAG: c_int = 4;

pub const ELFCLASS32: c_int = 1;
pub const ELFCLASS64: c_int = 2;
pub const ELFCLASSNUM: c_int = 3;

pub const ELFDATA2LSB: c_int = 1;
pub const ELFDATA2MSB: c_int = 2;

pub const EV_CURRENT: c_int = 1;
pub const EV_NUM: c_int = 2;
pub const ELFOSABI_NONE: c_int = 0;
pub const ELFOSABI_LINUX: c_int = 3;

// Note definitions: NN_ defines names. NT_ defines types.

pub const NT_GNU_PROPERTY_TYPE_0: c_int = 5;
//
// Notes used in ET_CORE. Architectures export some of the arch register sets
// using the corresponding note types via the PTRACE_GETREGSET and
// PTRACE_SETREGSET requests.
//

pub const NT_PRSTATUS: c_int = 1;

pub const NT_PRFPREG: c_int = 2;

pub const NT_PRPSINFO: c_int = 3;

pub const NT_TASKSTRUCT: c_int = 4;

pub const NT_AUXV: c_int = 6;
//
// Note to userspace developers: size of NT_SIGINFO note may increase
// in the future to accomodate more fields, don't assume it is fixed!
//

pub const NT_SIGINFO: c_uint = 0x53494749;

pub const NT_FILE: c_uint = 0x46494c45;

pub const NT_PRXFPREG: c_uint = 0x46e62b7f      /* copied from gdb5.1/include/elf/common.h */;

pub const NT_PPC_VMX: c_uint = 0x100		/* PowerPC Altivec/VMX registers */;

pub const NT_PPC_SPE: c_uint = 0x101		/* PowerPC SPE/EVR registers */;

pub const NT_PPC_VSX: c_uint = 0x102		/* PowerPC VSX registers */;

pub const NT_PPC_TAR: c_uint = 0x103		/* Target Address Register */;

pub const NT_PPC_PPR: c_uint = 0x104		/* Program Priority Register */;

pub const NT_PPC_DSCR: c_uint = 0x105		/* Data Stream Control Register */;

pub const NT_PPC_EBB: c_uint = 0x106		/* Event Based Branch Registers */;

pub const NT_PPC_PMU: c_uint = 0x107		/* Performance Monitor Registers */;

pub const NT_PPC_TM_CGPR: c_uint = 0x108		/* TM checkpointed GPR Registers */;

pub const NT_PPC_TM_CFPR: c_uint = 0x109		/* TM checkpointed FPR Registers */;

pub const NT_PPC_TM_CVMX: c_uint = 0x10a		/* TM checkpointed VMX Registers */;

pub const NT_PPC_TM_CVSX: c_uint = 0x10b		/* TM checkpointed VSX Registers */;

pub const NT_PPC_TM_SPR: c_uint = 0x10c		/* TM Special Purpose Registers */;

pub const NT_PPC_TM_CTAR: c_uint = 0x10d		/* TM checkpointed Target Address Register */;

pub const NT_PPC_TM_CPPR: c_uint = 0x10e		/* TM checkpointed Program Priority Register */;

pub const NT_PPC_TM_CDSCR: c_uint = 0x10f		/* TM checkpointed Data Stream Control Register */;

pub const NT_PPC_PKEY: c_uint = 0x110		/* Memory Protection Keys registers */;

pub const NT_PPC_DEXCR: c_uint = 0x111		/* PowerPC DEXCR registers */;

pub const NT_PPC_HASHKEYR: c_uint = 0x112		/* PowerPC HASHKEYR register */;

pub const NT_386_TLS: c_uint = 0x200		/* i386 TLS slots (struct user_desc) */;

pub const NT_386_IOPERM: c_uint = 0x201		/* x86 io permission bitmap (1=deny) */;

pub const NT_X86_XSTATE: c_uint = 0x202		/* x86 extended state using xsave */;
// Old binutils treats 0x203 as a CET state

pub const NT_X86_SHSTK: c_uint = 0x204		/* x86 SHSTK state */;

pub const NT_X86_XSAVE_LAYOUT: c_uint = 0x205	/* XSAVE layout description */;

pub const NT_S390_HIGH_GPRS: c_uint = 0x300	/* s390 upper register halves */;

pub const NT_S390_TIMER: c_uint = 0x301		/* s390 timer register */;

pub const NT_S390_TODCMP: c_uint = 0x302		/* s390 TOD clock comparator register */;

pub const NT_S390_TODPREG: c_uint = 0x303		/* s390 TOD programmable register */;

pub const NT_S390_CTRS: c_uint = 0x304		/* s390 control registers */;

pub const NT_S390_PREFIX: c_uint = 0x305		/* s390 prefix register */;

pub const NT_S390_LAST_BREAK: c_uint = 0x306	/* s390 breaking event address */;

pub const NT_S390_SYSTEM_CALL: c_uint = 0x307	/* s390 system call restart data */;

pub const NT_S390_TDB: c_uint = 0x308		/* s390 transaction diagnostic block */;

pub const NT_S390_VXRS_LOW: c_uint = 0x309	/* s390 vector registers 0-15 upper half */;

pub const NT_S390_VXRS_HIGH: c_uint = 0x30a	/* s390 vector registers 16-31 */;

pub const NT_S390_GS_CB: c_uint = 0x30b		/* s390 guarded storage registers */;

pub const NT_S390_GS_BC: c_uint = 0x30c		/* s390 guarded storage broadcast control block */;

pub const NT_S390_RI_CB: c_uint = 0x30d		/* s390 runtime instrumentation */;

pub const NT_S390_PV_CPU_DATA: c_uint = 0x30e	/* s390 protvirt cpu dump data */;

pub const NT_ARM_VFP: c_uint = 0x400		/* ARM VFP/NEON registers */;

pub const NT_ARM_TLS: c_uint = 0x401		/* ARM TLS register */;

pub const NT_ARM_HW_BREAK: c_uint = 0x402		/* ARM hardware breakpoint registers */;

pub const NT_ARM_HW_WATCH: c_uint = 0x403		/* ARM hardware watchpoint registers */;

pub const NT_ARM_SYSTEM_CALL: c_uint = 0x404	/* ARM system call number */;

pub const NT_ARM_SVE: c_uint = 0x405		/* ARM Scalable Vector Extension registers */;

pub const NT_ARM_PAC_MASK: c_uint = 0x406	/* ARM pointer authentication code masks */;

pub const NT_ARM_PACA_KEYS: c_uint = 0x407	/* ARM pointer authentication address keys */;

pub const NT_ARM_PACG_KEYS: c_uint = 0x408	/* ARM pointer authentication generic key */;

pub const NT_ARM_TAGGED_ADDR_CTRL: c_uint = 0x409	/* arm64 tagged address control (prctl()) */;

pub const NT_ARM_PAC_ENABLED_KEYS: c_uint = 0x40a	/* arm64 ptr auth enabled keys (prctl()) */;

pub const NT_ARM_SSVE: c_uint = 0x40b		/* ARM Streaming SVE registers */;

pub const NT_ARM_ZA: c_uint = 0x40c		/* ARM SME ZA registers */;

pub const NT_ARM_ZT: c_uint = 0x40d		/* ARM SME ZT registers */;

pub const NT_ARM_FPMR: c_uint = 0x40e		/* ARM floating point mode register */;

pub const NT_ARM_POE: c_uint = 0x40f		/* ARM POE registers */;

pub const NT_ARM_GCS: c_uint = 0x410		/* ARM GCS state */;

pub const NT_ARC_V2: c_uint = 0x600		/* ARCv2 accumulator/extra registers */;

pub const NT_VMCOREDD: c_uint = 0x700		/* Vmcore Device Dump Note */;

pub const NT_MIPS_DSP: c_uint = 0x800		/* MIPS DSP ASE registers */;

pub const NT_MIPS_FP_MODE: c_uint = 0x801		/* MIPS floating-point mode */;

pub const NT_MIPS_MSA: c_uint = 0x802		/* MIPS SIMD registers */;

pub const NT_RISCV_CSR: c_uint = 0x900		/* RISC-V Control and Status Registers */;

pub const NT_RISCV_VECTOR: c_uint = 0x901		/* RISC-V vector registers */;

pub const NT_RISCV_TAGGED_ADDR_CTRL: c_uint = 0x902	/* RISC-V tagged address control (prctl()) */;

pub const NT_RISCV_USER_CFI: c_uint = 0x903		/* RISC-V shadow stack state */;

pub const NT_LOONGARCH_CPUCFG: c_uint = 0xa00	/* LoongArch CPU config registers */;

pub const NT_LOONGARCH_CSR: c_uint = 0xa01	/* LoongArch control and status registers */;

pub const NT_LOONGARCH_LSX: c_uint = 0xa02	/* LoongArch Loongson SIMD Extension registers */;

pub const NT_LOONGARCH_LASX: c_uint = 0xa03	/* LoongArch Loongson Advanced SIMD Extension registers */;

pub const NT_LOONGARCH_LBT: c_uint = 0xa04	/* LoongArch Loongson Binary Translation registers */;

pub const NT_LOONGARCH_HW_BREAK: c_uint = 0xa05   /* LoongArch hardware breakpoint registers */;

pub const NT_LOONGARCH_HW_WATCH: c_uint = 0xa06   /* LoongArch hardware watchpoint registers */;
// Note header in a PT_NOTE section
// .note.gnu.property types for EM_AARCH64:
pub const GNU_PROPERTY_AARCH64_FEATURE_1_AND: c_uint = 0xc0000000;
// Bits for GNU_PROPERTY_AARCH64_FEATURE_1_BTI

