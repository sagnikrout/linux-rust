//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/reg.h
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
// Contains the definition of registers common to all PowerPC variants.
// If a register definition has been changed in a different PowerPC
// variant, we will case it in #ifndef XXX ... #endif, and have the
// number used in the Programming Environments Manual For 32-Bit
// Implementations of the PowerPC Architecture (a.k.a. Green Book) here.
//

// Pickup Book E specific registers.

// so tests for these bits fail on 32-bit
pub const MSR_SF: c_int = 0;
pub const MSR_HV: c_int = 0;
pub const MSR_S: c_int = 0;

//
// To be used in shared book E/book S, this avoids needing to worry about
// book S/book E in shared code
//

pub const MSR_SPE: c_int = 0;

// Server variant

// Default MSR for kernel mode.

pub const MSR_64BIT: c_int = 0;

// Condition Register related
pub const CR0_SHIFT: c_int = 28;
pub const CR0_MASK: c_uint = 0xF;

// Power Management - Processor Stop Status and Control Register Fields
pub const PSSCR_RL_MASK: c_uint = 0x0000000F /* Requested Level */;
pub const PSSCR_MTL_MASK: c_uint = 0x000000F0 /* Maximum Transition Level */;
pub const PSSCR_TR_MASK: c_uint = 0x00000300 /* Transition State */;
pub const PSSCR_PSLL_MASK: c_uint = 0x000F0000 /* Power-Saving Level Limit */;
pub const PSSCR_EC: c_uint = 0x00100000 /* Exit Criterion */;
pub const PSSCR_ESL: c_uint = 0x00200000 /* Enable State Loss */;
pub const PSSCR_SD: c_uint = 0x00400000 /* Status Disable */;
pub const PSSCR_PLS: c_uint = 0xf000000000000000 /* Power-saving Level Status */;
pub const PSSCR_PLS_SHIFT: c_int = 60;
pub const PSSCR_GUEST_VIS: c_uint = 0xf0000000000003ffUL /* Guest-visible PSSCR fields */;
pub const PSSCR_FAKE_SUSPEND: c_uint = 0x00000400 /* Fake-suspend bit (P9 DD2.2) */;

// Floating Point Status and Control Register (FPSCR) Fields
pub const FPSCR_FX: c_uint = 0x80000000	/* FPU exception summary */;
pub const FPSCR_FEX: c_uint = 0x40000000	/* FPU enabled exception summary */;
pub const FPSCR_VX: c_uint = 0x20000000	/* Invalid operation summary */;
pub const FPSCR_OX: c_uint = 0x10000000	/* Overflow exception summary */;
pub const FPSCR_UX: c_uint = 0x08000000	/* Underflow exception summary */;
pub const FPSCR_ZX: c_uint = 0x04000000	/* Zero-divide exception summary */;
pub const FPSCR_XX: c_uint = 0x02000000	/* Inexact exception summary */;
pub const FPSCR_VXSNAN: c_uint = 0x01000000	/* Invalid op for SNaN */;
pub const FPSCR_VXISI: c_uint = 0x00800000	/* Invalid op for Inv - Inv */;
pub const FPSCR_VXIDI: c_uint = 0x00400000	/* Invalid op for Inv / Inv */;
pub const FPSCR_VXZDZ: c_uint = 0x00200000	/* Invalid op for Zero / Zero */;
pub const FPSCR_VXIMZ: c_uint = 0x00100000	/* Invalid op for Inv * Zero */;
pub const FPSCR_VXVC: c_uint = 0x00080000	/* Invalid op for Compare */;
pub const FPSCR_FR: c_uint = 0x00040000	/* Fraction rounded */;
pub const FPSCR_FI: c_uint = 0x00020000	/* Fraction inexact */;
pub const FPSCR_FPRF: c_uint = 0x0001f000	/* FPU Result Flags */;
pub const FPSCR_FPCC: c_uint = 0x0000f000	/* FPU Condition Codes */;
pub const FPSCR_VXSOFT: c_uint = 0x00000400	/* Invalid op for software request */;
pub const FPSCR_VXSQRT: c_uint = 0x00000200	/* Invalid op for square root */;
pub const FPSCR_VXCVI: c_uint = 0x00000100	/* Invalid op for integer convert */;
pub const FPSCR_VE: c_uint = 0x00000080	/* Invalid op exception enable */;
pub const FPSCR_OE: c_uint = 0x00000040	/* IEEE overflow exception enable */;
pub const FPSCR_UE: c_uint = 0x00000020	/* IEEE underflow exception enable */;
pub const FPSCR_ZE: c_uint = 0x00000010	/* IEEE zero divide exception enable */;
pub const FPSCR_XE: c_uint = 0x00000008	/* FP inexact exception enable */;
pub const FPSCR_NI: c_uint = 0x00000004	/* FPU non IEEE-Mode */;
pub const FPSCR_RN: c_uint = 0x00000003	/* FPU rounding control */;
// Bit definitions for SPEFSCR.
pub const SPEFSCR_SOVH: c_uint = 0x80000000	/* Summary integer overflow high */;
pub const SPEFSCR_OVH: c_uint = 0x40000000	/* Integer overflow high */;
pub const SPEFSCR_FGH: c_uint = 0x20000000	/* Embedded FP guard bit high */;
pub const SPEFSCR_FXH: c_uint = 0x10000000	/* Embedded FP sticky bit high */;
pub const SPEFSCR_FINVH: c_uint = 0x08000000	/* Embedded FP invalid operation high */;
pub const SPEFSCR_FDBZH: c_uint = 0x04000000	/* Embedded FP div by zero high */;
pub const SPEFSCR_FUNFH: c_uint = 0x02000000	/* Embedded FP underflow high */;
pub const SPEFSCR_FOVFH: c_uint = 0x01000000	/* Embedded FP overflow high */;
pub const SPEFSCR_FINXS: c_uint = 0x00200000	/* Embedded FP inexact sticky */;
pub const SPEFSCR_FINVS: c_uint = 0x00100000	/* Embedded FP invalid op. sticky */;
pub const SPEFSCR_FDBZS: c_uint = 0x00080000	/* Embedded FP div by zero sticky */;
pub const SPEFSCR_FUNFS: c_uint = 0x00040000	/* Embedded FP underflow sticky */;
pub const SPEFSCR_FOVFS: c_uint = 0x00020000	/* Embedded FP overflow sticky */;
pub const SPEFSCR_MODE: c_uint = 0x00010000	/* Embedded FP mode */;
pub const SPEFSCR_SOV: c_uint = 0x00008000	/* Integer summary overflow */;
pub const SPEFSCR_OV: c_uint = 0x00004000	/* Integer overflow */;
pub const SPEFSCR_FG: c_uint = 0x00002000	/* Embedded FP guard bit */;
pub const SPEFSCR_FX: c_uint = 0x00001000	/* Embedded FP sticky bit */;
pub const SPEFSCR_FINV: c_uint = 0x00000800	/* Embedded FP invalid operation */;
pub const SPEFSCR_FDBZ: c_uint = 0x00000400	/* Embedded FP div by zero */;
pub const SPEFSCR_FUNF: c_uint = 0x00000200	/* Embedded FP underflow */;
pub const SPEFSCR_FOVF: c_uint = 0x00000100	/* Embedded FP overflow */;
pub const SPEFSCR_FINXE: c_uint = 0x00000040	/* Embedded FP inexact enable */;
pub const SPEFSCR_FINVE: c_uint = 0x00000020	/* Embedded FP invalid op. enable */;
pub const SPEFSCR_FDBZE: c_uint = 0x00000010	/* Embedded FP div by zero enable */;
pub const SPEFSCR_FUNFE: c_uint = 0x00000008	/* Embedded FP underflow enable */;
pub const SPEFSCR_FOVFE: c_uint = 0x00000004	/* Embedded FP overflow enable */;
pub const SPEFSCR_FRMC: c_uint = 0x00000003	/* Embedded FP rounding mode control */;
// Special Purpose Registers (SPRNs)
pub const SPRN_PID: c_uint = 0x030	/* Process ID */;

pub const SPRN_CTR: c_uint = 0x009	/* Count Register */;
pub const SPRN_DSCR: c_uint = 0x11;
pub const SPRN_CFAR: c_uint = 0x1c	/* Come From Address Register */;
pub const SPRN_AMR: c_uint = 0x1d	/* Authority Mask Register */;
pub const SPRN_UAMOR: c_uint = 0x9d	/* User Authority Mask Override Register */;
pub const SPRN_AMOR: c_uint = 0x15d	/* Authority Mask Override Register */;
pub const SPRN_ACOP: c_uint = 0x1F	/* Available Coprocessor Register */;
pub const SPRN_TFIAR: c_uint = 0x81	/* Transaction Failure Inst Addr   */;
pub const SPRN_TEXASR: c_uint = 0x82	/* Transaction EXception & Summary */;
pub const SPRN_TEXASRU: c_uint = 0x83	/* ''	   ''	   ''	 Upper 32  */;

pub const SPRN_TFHAR: c_uint = 0x80	/* Transaction Failure Handler Addr */;

pub const SPRN_CTRLF: c_uint = 0x088;
pub const SPRN_CTRLT: c_uint = 0x098;
pub const CTRL_CT: c_uint = 0xc0000000	/* current thread */;
pub const CTRL_CT0: c_uint = 0x80000000	/* thread 0 */;
pub const CTRL_CT1: c_uint = 0x40000000	/* thread 1 */;
pub const CTRL_TE: c_uint = 0x00c00000	/* thread enable */;
pub const CTRL_RUNLATCH: c_uint = 0x1;
pub const SPRN_DAWR0: c_uint = 0xB4;
pub const SPRN_DAWR1: c_uint = 0xB5;
pub const SPRN_RPR: c_uint = 0xBA	/* Relative Priority Register */;
pub const SPRN_CIABR: c_uint = 0xBB;
pub const CIABR_PRIV: c_uint = 0x3;
pub const CIABR_PRIV_USER: c_int = 1;
pub const CIABR_PRIV_SUPER: c_int = 2;
pub const CIABR_PRIV_HYPER: c_int = 3;
pub const SPRN_DAWRX0: c_uint = 0xBC;
pub const SPRN_DAWRX1: c_uint = 0xBD;

pub const SPRN_DABR: c_uint = 0x3F5	/* Data Address Breakpoint Register */;
pub const SPRN_DABR2: c_uint = 0x13D	/* e300 */;
pub const SPRN_DABRX: c_uint = 0x3F7	/* Data Address Breakpoint Register Extension */;

pub const SPRN_DAR: c_uint = 0x013	/* Data Address Register */;
pub const SPRN_DBCR: c_uint = 0x136	/* e300 Data Breakpoint Control Reg */;
pub const SPRN_DSISR: c_uint = 0x012	/* Data Storage Interrupt Status Register */;
pub const DSISR_BAD_DIRECT_ST: c_uint = 0x80000000 /* Obsolete: Direct store error */;
pub const DSISR_NOHPTE: c_uint = 0x40000000 /* no translation found */;
pub const DSISR_ATTR_CONFLICT: c_uint = 0x20000000 /* P9: Process vs. Partition attr */;
pub const DSISR_NOEXEC_OR_G: c_uint = 0x10000000 /* Alias of SRR1 bit, see below */;
pub const DSISR_PROTFAULT: c_uint = 0x08000000 /* protection fault */;
pub const DSISR_BADACCESS: c_uint = 0x04000000 /* bad access to CI or G */;
pub const DSISR_ISSTORE: c_uint = 0x02000000 /* access was a store */;
pub const DSISR_DABRMATCH: c_uint = 0x00400000 /* hit data breakpoint */;
pub const DSISR_NOSEGMENT: c_uint = 0x00200000 /* STAB miss (unsupported) */;
pub const DSISR_KEYFAULT: c_uint = 0x00200000 /* Storage Key fault */;
pub const DSISR_BAD_EXT_CTRL: c_uint = 0x00100000 /* Obsolete: External ctrl error */;
pub const DSISR_UNSUPP_MMU: c_uint = 0x00080000 /* P9: Unsupported MMU config */;
pub const DSISR_SET_RC: c_uint = 0x00040000 /* P9: Failed setting of R/C bits */;
pub const DSISR_PRTABLE_FAULT: c_uint = 0x00020000 /* P9: Fault on process table */;
pub const DSISR_ICSWX_NO_CT: c_uint = 0x00004000 /* P7: icswx unavailable cp type */;
pub const DSISR_BAD_COPYPASTE: c_uint = 0x00000008 /* P9: Copy/Paste on wrong memtype */;
pub const DSISR_BAD_AMO: c_uint = 0x00000004 /* P9: Incorrect AMO opcode */;
pub const DSISR_BAD_CI_LDST: c_uint = 0x00000002 /* P8: Bad HV CI load/store */;
//
// DSISR_NOEXEC_OR_G doesn't actually exist. This bit is always
// 0 on DSIs. However, on ISIs, the corresponding bit in SRR1
// indicates an attempt at executing from a no-execute PTE
// or segment or from a guarded page.
//
// We add a definition here for completeness as we alias
// DSISR and SRR1 in do_page_fault.
//
// DSISR bits that are treated as a fault. Any bit set
// here will skip hash_page, and cause do_page_fault to
// trigger a SIGBUS or SIGSEGV:
//

//
// These bits are equivalent in SRR1 and DSISR for 0x400
// instruction access interrupts on Book3S
//

pub const SPRN_TBRL: c_uint = 0x10C	/* Time Base Read Lower Register (user, R/O) */;
pub const SPRN_TBRU: c_uint = 0x10D	/* Time Base Read Upper Register (user, R/O) */;
pub const SPRN_CIR: c_uint = 0x11B	/* Chip Information Register (hyper, R/0) */;
pub const SPRN_TBWL: c_uint = 0x11C	/* Time Base Lower Register (super, R/W) */;
pub const SPRN_TBWU: c_uint = 0x11D	/* Time Base Upper Register (super, R/W) */;
pub const SPRN_TBU40: c_uint = 0x11E	/* Timebase upper 40 bits (hyper, R/W) */;
pub const SPRN_SPURR: c_uint = 0x134	/* Scaled PURR */;
pub const SPRN_HSPRG0: c_uint = 0x130	/* Hypervisor Scratch 0 */;
pub const SPRN_HSPRG1: c_uint = 0x131	/* Hypervisor Scratch 1 */;
pub const SPRN_HDSISR: c_uint = 0x132;
pub const SPRN_HDAR: c_uint = 0x133;
pub const SPRN_HDEC: c_uint = 0x136	/* Hypervisor Decrementer */;
pub const SPRN_HIOR: c_uint = 0x137	/* 970 Hypervisor interrupt offset */;
pub const SPRN_RMOR: c_uint = 0x138	/* Real mode offset register */;
pub const SPRN_HRMOR: c_uint = 0x139	/* Real mode offset register */;
pub const SPRN_HDEXCR_RO: c_uint = 0x1C7	/* Hypervisor DEXCR (non-privileged, readonly) */;
pub const SPRN_HASHKEYR: c_uint = 0x1D4	/* Non-privileged hashst/hashchk key register */;
pub const SPRN_HDEXCR: c_uint = 0x1D7	/* Hypervisor dynamic execution control register */;
pub const SPRN_DEXCR_RO: c_uint = 0x32C	/* DEXCR (non-privileged, readonly) */;
pub const SPRN_ASDR: c_uint = 0x330	/* Access segment descriptor register */;
pub const SPRN_DEXCR: c_uint = 0x33C	/* Dynamic execution control register */;
pub const DEXCR_PR_SBHE: c_uint = 0x80000000UL /* 0: Speculative Branch Hint Enable */;
pub const DEXCR_PR_IBRTPD: c_uint = 0x10000000UL /* 3: Indirect Branch Recurrent Target Prediction Disable */;
pub const DEXCR_PR_SRAPD: c_uint = 0x08000000UL /* 4: Subroutine Return Address Prediction Disable */;
pub const DEXCR_PR_NPHIE: c_uint = 0x04000000UL /* 5: Non-Privileged Hash Instruction Enable */;

pub const SPRN_IC: c_uint = 0x350	/* Virtual Instruction Count */;
pub const SPRN_VTB: c_uint = 0x351	/* Virtual Time Base */;
pub const SPRN_LDBAR: c_uint = 0x352	/* LD Base Address Register */;
pub const SPRN_PMICR: c_uint = 0x354   /* Power Management Idle Control Reg */;
pub const SPRN_PMSR: c_uint = 0x355   /* Power Management Status Reg */;
pub const SPRN_PMMAR: c_uint = 0x356	/* Power Management Memory Activity Register */;
pub const SPRN_PSSCR: c_uint = 0x357	/* Processor Stop Status and Control Register (ISA 3.0) */;
pub const SPRN_PSSCR_PR: c_uint = 0x337	/* PSSCR ISA 3.0, privileged mode access */;
pub const SPRN_TRIG2: c_uint = 0x372;
pub const SPRN_PMCR: c_uint = 0x374	/* Power Management Control Register */;
pub const SPRN_RWMR: c_uint = 0x375	/* Region-Weighting Mode Register */;
// HFSCR and FSCR bit numbers are the same

pub const SPRN_FSCR: c_uint = 0x099	/* Facility Status & Control Register */;

pub const SPRN_HFSCR: c_uint = 0xbe	/* HV=1 Facility Status & Control Register */;

pub const SPRN_TAR: c_uint = 0x32f	/* Target Address Register */;
pub const SPRN_LPCR: c_uint = 0x13E	/* LPAR Control Register */;

pub const LPCR_VC_SH: c_int = 61;
pub const LPCR_DPFD_SH: c_int = 52;

pub const LPCR_VRMASD_SH: c_int = 47;

pub const LPCR_RMLS: c_uint = 0x1C000000	/* Implementation dependent RMO limit sel */;
pub const LPCR_RMLS_SH: c_int = 26;

pub const LPCR_MER_SH: c_int = 11;

pub const LPCR_LPES: c_uint = 0x0000000c;

pub const LPCR_LPES_SH: c_int = 2;

pub const SPRN_LPID: c_uint = 0x13F	/* Logical Partition Identifier */;

pub const SPRN_HMER: c_uint = 0x150	/* Hypervisor maintenance exception reg */;

pub const SPRN_HMEER: c_uint = 0x151	/* Hyp maintenance exception enable reg */;
pub const SPRN_PCR: c_uint = 0x152	/* Processor compatibility register */;

//
// These bits are used in the function kvmppc_set_arch_compat() to specify and
// determine both the compatibility level which we want to emulate and the
// compatibility level which the host is capable of emulating.
//
pub const PCR_ARCH_31: c_uint = 0x20		/* Architecture 3.1 */;
pub const PCR_ARCH_300: c_uint = 0x10		/* Architecture 3.00 */;
pub const PCR_ARCH_207: c_uint = 0x8		/* Architecture 2.07 */;
pub const PCR_ARCH_206: c_uint = 0x4		/* Architecture 2.06 */;
pub const PCR_ARCH_205: c_uint = 0x2		/* Architecture 2.05 */;

pub const SPRN_HEIR: c_uint = 0x153	/* Hypervisor Emulated Instruction Register */;
pub const SPRN_TLBINDEXR: c_uint = 0x154	/* P7 TLB control register */;
pub const SPRN_TLBVPNR: c_uint = 0x155	/* P7 TLB control register */;
pub const SPRN_TLBRPNR: c_uint = 0x156	/* P7 TLB control register */;
pub const SPRN_TLBLPIDR: c_uint = 0x157	/* P7 TLB control register */;
pub const SPRN_DBAT0L: c_uint = 0x219	/* Data BAT 0 Lower Register */;
pub const SPRN_DBAT0U: c_uint = 0x218	/* Data BAT 0 Upper Register */;
pub const SPRN_DBAT1L: c_uint = 0x21B	/* Data BAT 1 Lower Register */;
pub const SPRN_DBAT1U: c_uint = 0x21A	/* Data BAT 1 Upper Register */;
pub const SPRN_DBAT2L: c_uint = 0x21D	/* Data BAT 2 Lower Register */;
pub const SPRN_DBAT2U: c_uint = 0x21C	/* Data BAT 2 Upper Register */;
pub const SPRN_DBAT3L: c_uint = 0x21F	/* Data BAT 3 Lower Register */;
pub const SPRN_DBAT3U: c_uint = 0x21E	/* Data BAT 3 Upper Register */;
pub const SPRN_DBAT4L: c_uint = 0x239	/* Data BAT 4 Lower Register */;
pub const SPRN_DBAT4U: c_uint = 0x238	/* Data BAT 4 Upper Register */;
pub const SPRN_DBAT5L: c_uint = 0x23B	/* Data BAT 5 Lower Register */;
pub const SPRN_DBAT5U: c_uint = 0x23A	/* Data BAT 5 Upper Register */;
pub const SPRN_DBAT6L: c_uint = 0x23D	/* Data BAT 6 Lower Register */;
pub const SPRN_DBAT6U: c_uint = 0x23C	/* Data BAT 6 Upper Register */;
pub const SPRN_DBAT7L: c_uint = 0x23F	/* Data BAT 7 Lower Register */;
pub const SPRN_DBAT7U: c_uint = 0x23E	/* Data BAT 7 Upper Register */;
pub const SPRN_PPR: c_uint = 0x380	/* SMT Thread status Register */;
pub const SPRN_TSCR: c_uint = 0x399	/* Thread Switch Control Register */;
pub const SPRN_DEC: c_uint = 0x016		/* Decrement Register */;
pub const SPRN_PIT: c_uint = 0x3DB		/* Programmable Interval Timer (BOOKE) */;
pub const SPRN_DER: c_uint = 0x095		/* Debug Enable Register */;
pub const DER_RSTE: c_uint = 0x40000000	/* Reset Interrupt */;
pub const DER_CHSTPE: c_uint = 0x20000000	/* Check Stop */;
pub const DER_MCIE: c_uint = 0x10000000	/* Machine Check Interrupt */;
pub const DER_EXTIE: c_uint = 0x02000000	/* External Interrupt */;
pub const DER_ALIE: c_uint = 0x01000000	/* Alignment Interrupt */;
pub const DER_PRIE: c_uint = 0x00800000	/* Program Interrupt */;
pub const DER_FPUVIE: c_uint = 0x00400000	/* FP Unavailable Interrupt */;
pub const DER_DECIE: c_uint = 0x00200000	/* Decrementer Interrupt */;
pub const DER_SYSIE: c_uint = 0x00040000	/* System Call Interrupt */;
pub const DER_TRE: c_uint = 0x00020000	/* Trace Interrupt */;
pub const DER_SEIE: c_uint = 0x00004000	/* FP SW Emulation Interrupt */;
pub const DER_ITLBMSE: c_uint = 0x00002000	/* Imp. Spec. Instruction TLB Miss */;
pub const DER_ITLBERE: c_uint = 0x00001000	/* Imp. Spec. Instruction TLB Error */;
pub const DER_DTLBMSE: c_uint = 0x00000800	/* Imp. Spec. Data TLB Miss */;
pub const DER_DTLBERE: c_uint = 0x00000400	/* Imp. Spec. Data TLB Error */;
pub const DER_LBRKE: c_uint = 0x00000008	/* Load/Store Breakpoint Interrupt */;
pub const DER_IBRKE: c_uint = 0x00000004	/* Instruction Breakpoint Interrupt */;
pub const DER_EBRKE: c_uint = 0x00000002	/* External Breakpoint Interrupt */;
pub const DER_DPIE: c_uint = 0x00000001	/* Dev. Port Nonmaskable Request */;
pub const SPRN_DMISS: c_uint = 0x3D0		/* Data TLB Miss Register */;
pub const SPRN_DHDES: c_uint = 0x0B1		/* Directed Hyp. Doorbell Exc. State */;
pub const SPRN_DPDES: c_uint = 0x0B0		/* Directed Priv. Doorbell Exc. State */;
pub const SPRN_EAR: c_uint = 0x11A		/* External Address Register */;
pub const SPRN_HASH1: c_uint = 0x3D2		/* Primary Hash Address Register */;
pub const SPRN_HASH2: c_uint = 0x3D3		/* Secondary Hash Address Register */;
pub const SPRN_HID0: c_uint = 0x3F0		/* Hardware Implementation Register 0 */;

// POWER8 HID0 bits

// POWER9 HID0 bits

pub const SPRN_HID1: c_uint = 0x3F1		/* Hardware Implementation Register 1 */;

pub const SPRN_HID2_750FX: c_uint = 0x3F8		/* IBM 750FX HID2 Register */;
pub const SPRN_HID2_GEKKO: c_uint = 0x398		/* Gekko HID2 Register */;
pub const SPRN_HID2_G2_LE: c_uint = 0x3F3		/* G2_LE HID2 Register */;

pub const SPRN_IABR: c_uint = 0x3F2	/* Instruction Address Breakpoint Register */;
pub const SPRN_IABR2: c_uint = 0x3FA		/* 83xx */;
pub const SPRN_IBCR: c_uint = 0x135		/* 83xx Insn Breakpoint Control Reg */;
pub const SPRN_IAMR: c_uint = 0x03D		/* Instr. Authority Mask Reg */;
pub const SPRN_HID4: c_uint = 0x3F4		/* 970 HID4 */;

pub const SPRN_HID4_GEKKO: c_uint = 0x3F3		/* Gekko HID4 */;
pub const SPRN_HID5: c_uint = 0x3F6		/* 970 HID5 */;
pub const SPRN_HID6: c_uint = 0x3F9	/* BE HID 6 */;

pub const SPRN_TSC_CELL: c_uint = 0x399	/* Thread switch control on Cell */;
pub const TSC_CELL_DEC_ENABLE_0: c_uint = 0x400000 /* Decrementer Interrupt */;
pub const TSC_CELL_DEC_ENABLE_1: c_uint = 0x200000 /* Decrementer Interrupt */;
pub const TSC_CELL_EE_ENABLE: c_uint = 0x100000 /* External Interrupt */;
pub const TSC_CELL_EE_BOOST: c_uint = 0x080000 /* External Interrupt Boost */;
pub const SPRN_TSC: c_uint = 0x3FD	/* Thread switch control on others */;
pub const SPRN_TST: c_uint = 0x3FC	/* Thread switch timeout on others */;

pub const SPRN_IAC1: c_uint = 0x3F4		/* Instruction Address Compare 1 */;
pub const SPRN_IAC2: c_uint = 0x3F5		/* Instruction Address Compare 2 */;

pub const SPRN_IBAT0L: c_uint = 0x211		/* Instruction BAT 0 Lower Register */;
pub const SPRN_IBAT0U: c_uint = 0x210		/* Instruction BAT 0 Upper Register */;
pub const SPRN_IBAT1L: c_uint = 0x213		/* Instruction BAT 1 Lower Register */;
pub const SPRN_IBAT1U: c_uint = 0x212		/* Instruction BAT 1 Upper Register */;
pub const SPRN_IBAT2L: c_uint = 0x215		/* Instruction BAT 2 Lower Register */;
pub const SPRN_IBAT2U: c_uint = 0x214		/* Instruction BAT 2 Upper Register */;
pub const SPRN_IBAT3L: c_uint = 0x217		/* Instruction BAT 3 Lower Register */;
pub const SPRN_IBAT3U: c_uint = 0x216		/* Instruction BAT 3 Upper Register */;
pub const SPRN_IBAT4L: c_uint = 0x231		/* Instruction BAT 4 Lower Register */;
pub const SPRN_IBAT4U: c_uint = 0x230		/* Instruction BAT 4 Upper Register */;
pub const SPRN_IBAT5L: c_uint = 0x233		/* Instruction BAT 5 Lower Register */;
pub const SPRN_IBAT5U: c_uint = 0x232		/* Instruction BAT 5 Upper Register */;
pub const SPRN_IBAT6L: c_uint = 0x235		/* Instruction BAT 6 Lower Register */;
pub const SPRN_IBAT6U: c_uint = 0x234		/* Instruction BAT 6 Upper Register */;
pub const SPRN_IBAT7L: c_uint = 0x237		/* Instruction BAT 7 Lower Register */;
pub const SPRN_IBAT7U: c_uint = 0x236		/* Instruction BAT 7 Upper Register */;
pub const SPRN_ICMP: c_uint = 0x3D5		/* Instruction TLB Compare Register */;
pub const SPRN_ICTC: c_uint = 0x3FB	/* Instruction Cache Throttling Control Reg */;

pub const SPRN_ICTRL: c_uint = 0x3F3	/* 1011 7450 icache and interrupt ctrl */;

pub const ICTRL_EICE: c_uint = 0x08000000	/* enable icache parity errs */;
pub const ICTRL_EDC: c_uint = 0x04000000	/* enable dcache parity errs */;
pub const ICTRL_EICP: c_uint = 0x00000100	/* enable icache par. check */;
pub const SPRN_IMISS: c_uint = 0x3D4		/* Instruction TLB Miss Register */;
pub const SPRN_IMMR: c_uint = 0x27E		/* Internal Memory Map Register */;
pub const SPRN_L2CR: c_uint = 0x3F9		/* Level 2 Cache Control Register */;
pub const SPRN_L2CR2: c_uint = 0x3f8;
pub const L2CR_L2E: c_uint = 0x80000000	/* L2 enable */;
pub const L2CR_L2PE: c_uint = 0x40000000	/* L2 parity enable */;
pub const L2CR_L2SIZ_MASK: c_uint = 0x30000000	/* L2 size mask */;
pub const L2CR_L2SIZ_256KB: c_uint = 0x10000000	/* L2 size 256KB */;
pub const L2CR_L2SIZ_512KB: c_uint = 0x20000000	/* L2 size 512KB */;
pub const L2CR_L2SIZ_1MB: c_uint = 0x30000000	/* L2 size 1MB */;
pub const L2CR_L2CLK_MASK: c_uint = 0x0e000000	/* L2 clock mask */;
pub const L2CR_L2CLK_DISABLED: c_uint = 0x00000000	/* L2 clock disabled */;
pub const L2CR_L2CLK_DIV1: c_uint = 0x02000000	/* L2 clock / 1 */;
pub const L2CR_L2CLK_DIV1_5: c_uint = 0x04000000	/* L2 clock / 1.5 */;
pub const L2CR_L2CLK_DIV2: c_uint = 0x08000000	/* L2 clock / 2 */;
pub const L2CR_L2CLK_DIV2_5: c_uint = 0x0a000000	/* L2 clock / 2.5 */;
pub const L2CR_L2CLK_DIV3: c_uint = 0x0c000000	/* L2 clock / 3 */;
pub const L2CR_L2RAM_MASK: c_uint = 0x01800000	/* L2 RAM type mask */;
pub const L2CR_L2RAM_FLOW: c_uint = 0x00000000	/* L2 RAM flow through */;
pub const L2CR_L2RAM_PIPE: c_uint = 0x01000000	/* L2 RAM pipelined */;
pub const L2CR_L2RAM_PIPE_LW: c_uint = 0x01800000	/* L2 RAM pipelined latewr */;
pub const L2CR_L2DO: c_uint = 0x00400000	/* L2 data only */;
pub const L2CR_L2I: c_uint = 0x00200000	/* L2 global invalidate */;
pub const L2CR_L2CTL: c_uint = 0x00100000	/* L2 RAM control */;
pub const L2CR_L2WT: c_uint = 0x00080000	/* L2 write-through */;
pub const L2CR_L2TS: c_uint = 0x00040000	/* L2 test support */;
pub const L2CR_L2OH_MASK: c_uint = 0x00030000	/* L2 output hold mask */;
pub const L2CR_L2OH_0_5: c_uint = 0x00000000	/* L2 output hold 0.5 ns */;
pub const L2CR_L2OH_1_0: c_uint = 0x00010000	/* L2 output hold 1.0 ns */;
pub const L2CR_L2SL: c_uint = 0x00008000	/* L2 DLL slow */;
pub const L2CR_L2DF: c_uint = 0x00004000	/* L2 differential clock */;
pub const L2CR_L2BYP: c_uint = 0x00002000	/* L2 DLL bypass */;
pub const L2CR_L2IP: c_uint = 0x00000001	/* L2 GI in progress */;
pub const L2CR_L2IO_745x: c_uint = 0x00100000	/* L2 instr. only (745x) */;
pub const L2CR_L2DO_745x: c_uint = 0x00010000	/* L2 data only (745x) */;
pub const L2CR_L2REP_745x: c_uint = 0x00001000	/* L2 repl. algorithm (745x) */;
pub const L2CR_L2HWF_745x: c_uint = 0x00000800	/* L2 hardware flush (745x) */;
pub const SPRN_L3CR: c_uint = 0x3FA	/* Level 3 Cache Control Register */;
pub const L3CR_L3E: c_uint = 0x80000000	/* L3 enable */;
pub const L3CR_L3PE: c_uint = 0x40000000	/* L3 data parity enable */;
pub const L3CR_L3APE: c_uint = 0x20000000	/* L3 addr parity enable */;
pub const L3CR_L3SIZ: c_uint = 0x10000000	/* L3 size */;
pub const L3CR_L3CLKEN: c_uint = 0x08000000	/* L3 clock enable */;
pub const L3CR_L3RES: c_uint = 0x04000000	/* L3 special reserved bit */;
pub const L3CR_L3CLKDIV: c_uint = 0x03800000	/* L3 clock divisor */;
pub const L3CR_L3IO: c_uint = 0x00400000	/* L3 instruction only */;
pub const L3CR_L3SPO: c_uint = 0x00040000	/* L3 sample point override */;
pub const L3CR_L3CKSP: c_uint = 0x00030000	/* L3 clock sample point */;
pub const L3CR_L3PSP: c_uint = 0x0000e000	/* L3 P-clock sample point */;
pub const L3CR_L3REP: c_uint = 0x00001000	/* L3 replacement algorithm */;
pub const L3CR_L3HWF: c_uint = 0x00000800	/* L3 hardware flush */;
pub const L3CR_L3I: c_uint = 0x00000400	/* L3 global invalidate */;
pub const L3CR_L3RT: c_uint = 0x00000300	/* L3 SRAM type */;
pub const L3CR_L3NIRCA: c_uint = 0x00000080	/* L3 non-integer ratio clock adj. */;
pub const L3CR_L3DO: c_uint = 0x00000040	/* L3 data only mode */;
pub const L3CR_PMEN: c_uint = 0x00000004	/* L3 private memory enable */;
pub const L3CR_PMSIZ: c_uint = 0x00000001	/* L3 private memory size */;
pub const SPRN_MSSCR0: c_uint = 0x3f6	/* Memory Subsystem Control Register 0 */;
pub const SPRN_MSSSR0: c_uint = 0x3f7	/* Memory Subsystem Status Register 1 */;
pub const SPRN_LDSTCR: c_uint = 0x3f8	/* Load/Store control register */;
pub const SPRN_LDSTDB: c_uint = 0x3f4	/* */;
pub const SPRN_LR: c_uint = 0x008	/* Link Register */;

pub const SPRN_PIR: c_uint = 0x3FF	/* Processor Identification Register */;

pub const SPRN_TIR: c_uint = 0x1BE	/* Thread Identification Register */;
pub const SPRN_PTCR: c_uint = 0x1D0	/* Partition table control Register */;
pub const SPRN_PSPB: c_uint = 0x09F	/* Problem State Priority Boost reg */;
pub const SPRN_PTEHI: c_uint = 0x3D5	/* 981 7450 PTE HI word (S/W TLB load) */;
pub const SPRN_PTELO: c_uint = 0x3D6	/* 982 7450 PTE LO word (S/W TLB load) */;
pub const SPRN_PURR: c_uint = 0x135	/* Processor Utilization of Resources Reg */;
pub const SPRN_PVR: c_uint = 0x11F	/* Processor Version Register */;
pub const SPRN_RPA: c_uint = 0x3D6	/* Required Physical Address Register */;
pub const SPRN_SDA: c_uint = 0x3BF	/* Sampled Data Address Register */;
pub const SPRN_SDR1: c_uint = 0x019	/* MMU Hash Base Register */;
pub const SPRN_ASR: c_uint = 0x118   /* Address Space Register */;
pub const SPRN_SIA: c_uint = 0x3BB	/* Sampled Instruction Address Register */;
pub const SPRN_SPRG0: c_uint = 0x110	/* Special Purpose Register General 0 */;
pub const SPRN_SPRG1: c_uint = 0x111	/* Special Purpose Register General 1 */;
pub const SPRN_SPRG2: c_uint = 0x112	/* Special Purpose Register General 2 */;
pub const SPRN_SPRG3: c_uint = 0x113	/* Special Purpose Register General 3 */;
pub const SPRN_USPRG3: c_uint = 0x103	/* SPRG3 userspace read */;
pub const SPRN_SPRG4: c_uint = 0x114	/* Special Purpose Register General 4 */;
pub const SPRN_USPRG4: c_uint = 0x104	/* SPRG4 userspace read */;
pub const SPRN_SPRG5: c_uint = 0x115	/* Special Purpose Register General 5 */;
pub const SPRN_USPRG5: c_uint = 0x105	/* SPRG5 userspace read */;
pub const SPRN_SPRG6: c_uint = 0x116	/* Special Purpose Register General 6 */;
pub const SPRN_USPRG6: c_uint = 0x106	/* SPRG6 userspace read */;
pub const SPRN_SPRG7: c_uint = 0x117	/* Special Purpose Register General 7 */;
pub const SPRN_USPRG7: c_uint = 0x107	/* SPRG7 userspace read */;
pub const SPRN_SRR0: c_uint = 0x01A	/* Save/Restore Register 0 */;
pub const SPRN_SRR1: c_uint = 0x01B	/* Save/Restore Register 1 */;

//
// Bits loaded from MSR upon interrupt.
// PPC (64-bit) bits 33-36,42-47 are interrupt dependent, the others are
// loaded from MSR. The exception is that SRESET and MCE do not always load
// bit 62 (RI) from MSR. Don't use PPC_BITMASK for this because 32-bit uses
// it.
//

pub const SRR1_ISI_NOPT: c_uint = 0x40000000 /* ISI: Not found in hash */;
pub const SRR1_ISI_N_G_OR_CIP: c_uint = 0x10000000 /* ISI: Access is no-exec or G or CI for a prefixed instruction */;
pub const SRR1_ISI_PROT: c_uint = 0x08000000 /* ISI: Other protection fault */;
pub const SRR1_WAKEMASK: c_uint = 0x00380000 /* reason for wakeup */;
pub const SRR1_WAKEMASK_P8: c_uint = 0x003c0000 /* reason for wakeup on POWER8 and 9 */;
pub const SRR1_WAKEMCE_RESVD: c_uint = 0x003c0000 /* Unused/reserved value used by MCE wakeup to indicate cause to idle wakeup handler */;
pub const SRR1_WAKESYSERR: c_uint = 0x00300000 /* System error */;
pub const SRR1_WAKEEE: c_uint = 0x00200000 /* External interrupt */;
pub const SRR1_WAKEHVI: c_uint = 0x00240000 /* Hypervisor Virtualization Interrupt (P9) */;
pub const SRR1_WAKEMT: c_uint = 0x00280000 /* mtctrl */;
pub const SRR1_WAKEHMI: c_uint = 0x00280000 /* Hypervisor maintenance */;
pub const SRR1_WAKEDEC: c_uint = 0x00180000 /* Decrementer interrupt */;
pub const SRR1_WAKEDBELL: c_uint = 0x00140000 /* Privileged doorbell on P8 */;
pub const SRR1_WAKETHERM: c_uint = 0x00100000 /* Thermal management interrupt */;
pub const SRR1_WAKERESET: c_uint = 0x00100000 /* System reset */;
pub const SRR1_WAKEHDBELL: c_uint = 0x000c0000 /* Hypervisor doorbell on P8 */;
pub const SRR1_WAKESTATE: c_uint = 0x00030000 /* Powersave exit mask [46:47] */;
pub const SRR1_WS_HVLOSS: c_uint = 0x00030000 /* HV resources not maintained */;
pub const SRR1_WS_GPRLOSS: c_uint = 0x00020000 /* GPRs not maintained */;
pub const SRR1_WS_NOLOSS: c_uint = 0x00010000 /* All resources maintained */;
pub const SRR1_PROGTM: c_uint = 0x00200000 /* TM Bad Thing */;
pub const SRR1_PROGFPE: c_uint = 0x00100000 /* Floating Point Enabled */;
pub const SRR1_PROGILL: c_uint = 0x00080000 /* Illegal instruction */;
pub const SRR1_PROGPRIV: c_uint = 0x00040000 /* Privileged instruction */;
pub const SRR1_PROGTRAP: c_uint = 0x00020000 /* Trap */;
pub const SRR1_PROGADDR: c_uint = 0x00010000 /* SRR0 contains subsequent addr */;
pub const SRR1_MCE_MCP: c_uint = 0x00080000 /* Machine check signal caused interrupt */;
pub const SRR1_BOUNDARY: c_uint = 0x10000000 /* Prefixed instruction crosses 64-byte boundary */;
pub const SRR1_PREFIXED: c_uint = 0x20000000 /* Exception caused by prefixed instruction */;
pub const SPRN_HSRR0: c_uint = 0x13A	/* Save/Restore Register 0 */;
pub const SPRN_HSRR1: c_uint = 0x13B	/* Save/Restore Register 1 */;
pub const HSRR1_DENORM: c_uint = 0x00100000 /* Denorm exception */;
pub const HSRR1_HISI_WRITE: c_uint = 0x00010000 /* HISI bcs couldn't update mem */;
pub const SPRN_TBCTL: c_uint = 0x35f	/* PA6T Timebase control register */;
pub const TBCTL_FREEZE: c_uint = 0x0000000000000000ull /* Freeze all tbs */;
pub const TBCTL_RESTART: c_uint = 0x0000000100000000ull /* Restart all tbs */;
pub const TBCTL_UPDATE_UPPER: c_uint = 0x0000000200000000ull /* Set upper 32 bits */;
pub const TBCTL_UPDATE_LOWER: c_uint = 0x0000000300000000ull /* Set lower 32 bits */;

pub const SPRN_SVR: c_uint = 0x11E	/* System Version Register */;

pub const SPRN_THRM1: c_uint = 0x3FC		/* Thermal Management Register 1 */;
// these bits were defined in inverted endian sense originally, ugh, confusing

pub const SPRN_THRM2: c_uint = 0x3FD		/* Thermal Management Register 2 */;
pub const SPRN_THRM3: c_uint = 0x3FE		/* Thermal Management Register 3 */;

pub const SPRN_TLBMISS: c_uint = 0x3D4		/* 980 7450 TLB Miss Register */;
pub const SPRN_UMMCR0: c_uint = 0x3A8	/* User Monitor Mode Control Register 0 */;
pub const SPRN_UMMCR1: c_uint = 0x3AC	/* User Monitor Mode Control Register 0 */;
pub const SPRN_UPMC1: c_uint = 0x3A9	/* User Performance Counter Register 1 */;
pub const SPRN_UPMC2: c_uint = 0x3AA	/* User Performance Counter Register 2 */;
pub const SPRN_UPMC3: c_uint = 0x3AD	/* User Performance Counter Register 3 */;
pub const SPRN_UPMC4: c_uint = 0x3AE	/* User Performance Counter Register 4 */;
pub const SPRN_USIA: c_uint = 0x3AB	/* User Sampled Instruction Address Register */;
pub const SPRN_VRSAVE: c_uint = 0x100	/* Vector Register Save Register */;
pub const SPRN_XER: c_uint = 0x001	/* Fixed Point Exception Register */;
pub const SPRN_MMCR0_GEKKO: c_uint = 0x3B8 /* Gekko Monitor Mode Control Register 0 */;
pub const SPRN_MMCR1_GEKKO: c_uint = 0x3BC /* Gekko Monitor Mode Control Register 1 */;
pub const SPRN_PMC1_GEKKO: c_uint = 0x3B9 /* Gekko Performance Monitor Control 1 */;
pub const SPRN_PMC2_GEKKO: c_uint = 0x3BA /* Gekko Performance Monitor Control 2 */;
pub const SPRN_PMC3_GEKKO: c_uint = 0x3BD /* Gekko Performance Monitor Control 3 */;
pub const SPRN_PMC4_GEKKO: c_uint = 0x3BE /* Gekko Performance Monitor Control 4 */;
pub const SPRN_WPAR_GEKKO: c_uint = 0x399 /* Gekko Write Pipe Address Register */;
pub const SPRN_SCOMC: c_uint = 0x114	/* SCOM Access Control */;
pub const SPRN_SCOMD: c_uint = 0x115	/* SCOM Access DATA */;
// Performance monitor SPRs

pub const SPRN_MMCR0: c_int = 795;
pub const MMCR0_FC: c_uint = 0x80000000UL /* freeze counters */;
pub const MMCR0_FCS: c_uint = 0x40000000UL /* freeze in supervisor state */;

pub const MMCR0_FCP: c_uint = 0x20000000UL /* freeze in problem state */;

pub const MMCR0_FCM1: c_uint = 0x10000000UL /* freeze counters while MSR mark = 1 */;
pub const MMCR0_FCM0: c_uint = 0x08000000UL /* freeze counters while MSR mark = 0 */;

pub const MMCR0_TBEE: c_uint = 0x00400000UL /* time base exception enable */;
pub const MMCR0_BHRBA: c_uint = 0x00200000UL /* BHRB Access allowed in userspace */;
pub const MMCR0_EBE: c_uint = 0x00100000UL /* Event based branch enable */;
pub const MMCR0_PMCC: c_uint = 0x000c0000UL /* PMC control */;

pub const MMCR0_PMCC_U6: c_uint = 0x00080000UL /* PMC1-6 are R/W by user (PR) */;
pub const MMCR0_PMC1CE: c_uint = 0x00008000UL /* PMC1 count enable*/;

pub const MMCR0_TRIGGER: c_uint = 0x00002000UL /* TRIGGER enable */;

// performance monitor alert has occurred, set to 0 after handling exception

pub const MMCR0_SHRFC: c_uint = 0x00000040UL /* SHRre freeze conditions between threads */;
pub const MMCR0_FC56: c_uint = 0x00000010UL /* freeze counters 5 and 6 */;
pub const MMCR0_FCTI: c_uint = 0x00000008UL /* freeze counters in tags inactive mode */;
pub const MMCR0_FCTA: c_uint = 0x00000004UL /* freeze counters in tags active mode */;
pub const MMCR0_FCWAIT: c_uint = 0x00000002UL /* freeze counter in WAIT state */;
pub const MMCR0_FCHV: c_uint = 0x00000001UL /* freeze conditions in hypervisor mode */;
pub const SPRN_MMCR1: c_int = 798;
pub const SPRN_MMCR2: c_int = 785;
pub const SPRN_MMCR3: c_int = 754;
pub const SPRN_UMMCR2: c_int = 769;
pub const SPRN_UMMCR3: c_int = 738;
pub const SPRN_MMCRA: c_uint = 0x312;
pub const MMCRA_SDSYNC: c_uint = 0x80000000UL /* SDAR synced with SIAR */;
pub const MMCRA_SDAR_DCACHE_MISS: c_uint = 0x40000000UL;
pub const MMCRA_SDAR_ERAT_MISS: c_uint = 0x20000000UL;
pub const MMCRA_SIHV: c_uint = 0x10000000UL /* state of MSR HV when SIAR set */;
pub const MMCRA_SIPR: c_uint = 0x08000000UL /* state of MSR PR when SIAR set */;
pub const MMCRA_SLOT: c_uint = 0x07000000UL /* SLOT bits (37-39) */;
pub const MMCRA_SLOT_SHIFT: c_int = 24;
pub const MMCRA_SAMPLE_ENABLE: c_uint = 0x00000001UL /* enable sampling */;

pub const POWER6_MMCRA_SDSYNC: c_uint = 0x0000080000000000ULL	/* SDAR/SIAR synced */;
pub const POWER6_MMCRA_SIHV: c_uint = 0x0000040000000000ULL;
pub const POWER6_MMCRA_SIPR: c_uint = 0x0000020000000000ULL;
pub const POWER6_MMCRA_THRM: c_uint = 0x00000020UL;
pub const POWER6_MMCRA_OTHER: c_uint = 0x0000000EUL;
pub const POWER7P_MMCRA_SIAR_VALID: c_uint = 0x10000000	/* P7+ SIAR contents valid */;
pub const POWER7P_MMCRA_SDAR_VALID: c_uint = 0x08000000	/* P7+ SDAR contents valid */;

pub const BESCR_GE: c_uint = 0x8000000000000000ULL /* Global Enable */;

pub const SPRN_PMC1: c_int = 787;
pub const SPRN_PMC2: c_int = 788;
pub const SPRN_PMC3: c_int = 789;
pub const SPRN_PMC4: c_int = 790;
pub const SPRN_PMC5: c_int = 791;
pub const SPRN_PMC6: c_int = 792;
pub const SPRN_PMC7: c_int = 793;
pub const SPRN_PMC8: c_int = 794;
pub const SPRN_SIER: c_int = 784;
pub const SIER_SIPR: c_uint = 0x2000000	/* Sampled MSR_PR */;
pub const SIER_SIHV: c_uint = 0x1000000	/* Sampled MSR_HV */;
pub const SIER_SIAR_VALID: c_uint = 0x0400000	/* SIAR contents valid */;
pub const SIER_SDAR_VALID: c_uint = 0x0200000	/* SDAR contents valid */;
pub const SPRN_SIER2: c_int = 752;
pub const SPRN_SIER3: c_int = 753;
pub const SPRN_USIER2: c_int = 736;
pub const SPRN_USIER3: c_int = 737;
pub const SPRN_SIAR: c_int = 796;
pub const SPRN_SDAR: c_int = 797;
pub const SPRN_TACR: c_int = 888;
pub const SPRN_TCSCR: c_int = 889;
pub const SPRN_CSIGR: c_int = 890;
pub const SPRN_SPMC1: c_int = 892;
pub const SPRN_SPMC2: c_int = 893;
// When EBB is enabled, some of MMCR0/MMCR2/SIER are user accessible

pub const MMCR2_USER_MASK: c_uint = 0x4020100804020000UL /* (FC1P|FC2P|FC3P|FC4P|FC5P|FC6P) */;
pub const SIER_USER_MASK: c_uint = 0x7fffffUL;
pub const SPRN_PA6T_MMCR0: c_int = 795;
pub const PA6T_MMCR0_EN0: c_uint = 0x0000000000000001UL;
pub const PA6T_MMCR0_EN1: c_uint = 0x0000000000000002UL;
pub const PA6T_MMCR0_EN2: c_uint = 0x0000000000000004UL;
pub const PA6T_MMCR0_EN3: c_uint = 0x0000000000000008UL;
pub const PA6T_MMCR0_EN4: c_uint = 0x0000000000000010UL;
pub const PA6T_MMCR0_EN5: c_uint = 0x0000000000000020UL;
pub const PA6T_MMCR0_SUPEN: c_uint = 0x0000000000000040UL;
pub const PA6T_MMCR0_PREN: c_uint = 0x0000000000000080UL;
pub const PA6T_MMCR0_HYPEN: c_uint = 0x0000000000000100UL;
pub const PA6T_MMCR0_FCM0: c_uint = 0x0000000000000200UL;
pub const PA6T_MMCR0_FCM1: c_uint = 0x0000000000000400UL;
pub const PA6T_MMCR0_INTGEN: c_uint = 0x0000000000000800UL;
pub const PA6T_MMCR0_INTEN0: c_uint = 0x0000000000001000UL;
pub const PA6T_MMCR0_INTEN1: c_uint = 0x0000000000002000UL;
pub const PA6T_MMCR0_INTEN2: c_uint = 0x0000000000004000UL;
pub const PA6T_MMCR0_INTEN3: c_uint = 0x0000000000008000UL;
pub const PA6T_MMCR0_INTEN4: c_uint = 0x0000000000010000UL;
pub const PA6T_MMCR0_INTEN5: c_uint = 0x0000000000020000UL;
pub const PA6T_MMCR0_DISCNT: c_uint = 0x0000000000040000UL;
pub const PA6T_MMCR0_UOP: c_uint = 0x0000000000080000UL;
pub const PA6T_MMCR0_TRG: c_uint = 0x0000000000100000UL;
pub const PA6T_MMCR0_TRGEN: c_uint = 0x0000000000200000UL;
pub const PA6T_MMCR0_TRGREG: c_uint = 0x0000000001600000UL;
pub const PA6T_MMCR0_SIARLOG: c_uint = 0x0000000002000000UL;
pub const PA6T_MMCR0_SDARLOG: c_uint = 0x0000000004000000UL;
pub const PA6T_MMCR0_PROEN: c_uint = 0x0000000008000000UL;
pub const PA6T_MMCR0_PROLOG: c_uint = 0x0000000010000000UL;
pub const PA6T_MMCR0_DAMEN2: c_uint = 0x0000000020000000UL;
pub const PA6T_MMCR0_DAMEN3: c_uint = 0x0000000040000000UL;
pub const PA6T_MMCR0_DAMEN4: c_uint = 0x0000000080000000UL;
pub const PA6T_MMCR0_DAMEN5: c_uint = 0x0000000100000000UL;
pub const PA6T_MMCR0_DAMSEL2: c_uint = 0x0000000200000000UL;
pub const PA6T_MMCR0_DAMSEL3: c_uint = 0x0000000400000000UL;
pub const PA6T_MMCR0_DAMSEL4: c_uint = 0x0000000800000000UL;
pub const PA6T_MMCR0_DAMSEL5: c_uint = 0x0000001000000000UL;
pub const PA6T_MMCR0_HANDDIS: c_uint = 0x0000002000000000UL;
pub const PA6T_MMCR0_PCTEN: c_uint = 0x0000004000000000UL;
pub const PA6T_MMCR0_SOCEN: c_uint = 0x0000008000000000UL;
pub const PA6T_MMCR0_SOCMOD: c_uint = 0x0000010000000000UL;
pub const SPRN_PA6T_MMCR1: c_int = 798;
pub const PA6T_MMCR1_ES2: c_uint = 0x00000000000000ffUL;
pub const PA6T_MMCR1_ES3: c_uint = 0x000000000000ff00UL;
pub const PA6T_MMCR1_ES4: c_uint = 0x0000000000ff0000UL;
pub const PA6T_MMCR1_ES5: c_uint = 0x00000000ff000000UL;

pub const SPRN_PA6T_UPMC2: c_int = 773;
pub const SPRN_PA6T_UPMC3: c_int = 774;
pub const SPRN_PA6T_UPMC4: c_int = 775;
pub const SPRN_PA6T_UPMC5: c_int = 776;

pub const SPRN_PA6T_PMC0: c_int = 787;
pub const SPRN_PA6T_PMC1: c_int = 788;
pub const SPRN_PA6T_PMC2: c_int = 789;
pub const SPRN_PA6T_PMC3: c_int = 790;
pub const SPRN_PA6T_PMC4: c_int = 791;
pub const SPRN_PA6T_PMC5: c_int = 792;

pub const SPRN_PA6T_IMA2: c_int = 882;
pub const SPRN_PA6T_IMA3: c_int = 883;
pub const SPRN_PA6T_IMA4: c_int = 884;
pub const SPRN_PA6T_IMA5: c_int = 885;
pub const SPRN_PA6T_IMA6: c_int = 886;
pub const SPRN_PA6T_IMA7: c_int = 887;
pub const SPRN_PA6T_IMA8: c_int = 888;
pub const SPRN_PA6T_IMA9: c_int = 889;

pub const MMCR0_FC: c_uint = 0x80000000UL /* freeze counters */;
pub const MMCR0_FCS: c_uint = 0x40000000UL /* freeze in supervisor state */;
pub const MMCR0_FCP: c_uint = 0x20000000UL /* freeze in problem state */;
pub const MMCR0_FCM1: c_uint = 0x10000000UL /* freeze counters while MSR mark = 1 */;
pub const MMCR0_FCM0: c_uint = 0x08000000UL /* freeze counters while MSR mark = 0 */;
pub const MMCR0_PMXE: c_uint = 0x04000000UL /* performance monitor exception enable */;
pub const MMCR0_FCECE: c_uint = 0x02000000UL /* freeze ctrs on enabled cond or event */;
pub const MMCR0_TBEE: c_uint = 0x00400000UL /* time base exception enable */;
pub const MMCR0_PMC1CE: c_uint = 0x00008000UL /* PMC1 count enable*/;
pub const MMCR0_PMCnCE: c_uint = 0x00004000UL /* count enable for all but PMC 1*/;
pub const MMCR0_TRIGGER: c_uint = 0x00002000UL /* TRIGGER enable */;
pub const MMCR0_PMC1SEL: c_uint = 0x00001fc0UL /* PMC 1 Event */;
pub const MMCR0_PMC2SEL: c_uint = 0x0000003fUL /* PMC 2 Event */;
pub const SPRN_MMCR1: c_int = 956;
pub const MMCR1_PMC3SEL: c_uint = 0xf8000000UL /* PMC 3 Event */;
pub const MMCR1_PMC4SEL: c_uint = 0x07c00000UL /* PMC 4 Event */;
pub const MMCR1_PMC5SEL: c_uint = 0x003e0000UL /* PMC 5 Event */;
pub const MMCR1_PMC6SEL: c_uint = 0x0001f800UL /* PMC 6 Event */;
pub const SPRN_MMCR2: c_int = 944;

// Bit definitions for MMCR0 and PMC1 / PMC2.

pub const MMCR0_PMC2_DCACHEMISS: c_uint = 0x6;
pub const MMCR0_PMC2_CYCLES: c_uint = 0x1;
pub const MMCR0_PMC2_ITLB: c_uint = 0x7;
pub const MMCR0_PMC2_LOADMISSTIME: c_uint = 0x5;

//
// SPRG usage:
//
// All 64-bit:
// - SPRG1 stores PACA pointer except 64-bit server in
// HV mode in which case it is HSPRG0
//
// 64-bit server:
// - SPRG0 scratch for TM recheckpoint/reclaim (reserved for HV on Power4)
// - SPRG2 scratch for exception vectors
// - SPRG3 CPU and NUMA node for VDSO getcpu (user visible)
// - HSPRG0 stores PACA in HV mode
// - HSPRG1 scratch for "HV" exceptions
//
// 64-bit embedded
// - SPRG0 generic exception scratch
// - SPRG2 TLB exception stack
// - SPRG3 critical exception scratch (user visible, sorry!)
// - SPRG4 unused (user visible)
// - SPRG6 TLB miss scratch (user visible, sorry !)
// - SPRG7 CPU and NUMA node for VDSO getcpu (user visible)
// - SPRG8 machine check exception scratch
// - SPRG9 debug exception scratch
//
// All 32-bit:
// - SPRG3 current thread_struct physical addr pointer
// (virtual on BookE, physical on others)
//
// 32-bit classic:
// - SPRG0 scratch for exception vectors
// - SPRG1 scratch for exception vectors
// - SPRG2 indicator that we are in RTAS
// - SPRG4 (603 only) pseudo TLB LRU data
//
// 32-bit 440 and FSL BookE:
// - SPRG0 scratch for exception vectors
// - SPRG1 scratch for exception vectors (*)
// - SPRG2 scratch for crit interrupts handler
// - SPRG4 scratch for exception vectors
// - SPRG5 scratch for exception vectors
// - SPRG6 scratch for machine check handler
// - SPRG7 scratch for exception vectors
// - SPRG9 scratch for debug vectors (e500 only)
//
// Additionally, BookE separates "read" and "write"
// of those registers. That allows to use the userspace
// readable variant for reads, which can avoid a fault
// with KVM type virtualization.
//
// 32-bit 8xx:
// - SPRG0 scratch for exception vectors
// - SPRG1 scratch for exception vectors
// - SPRG2 scratch for exception vectors
//

//
// An mtfsf instruction with the L bit set. On CPUs that support this a
// full 64bits of FPSCR is restored and on other CPUs the L bit is ignored.
//
// Until binutils gets the new form of mtfsf, hardwire the instruction.
//

// Processor Version Register (PVR) field extraction

//
// IBM has further subdivided the standard PowerPC 16-bit version and
// revision subfields of the PVR for the PowerPC 403s into the following:
//

// Processor Version Numbers
pub const PVR_403GA: c_uint = 0x00200000;
pub const PVR_403GB: c_uint = 0x00200100;
pub const PVR_403GC: c_uint = 0x00200200;
pub const PVR_403GCX: c_uint = 0x00201400;
pub const PVR_405GP: c_uint = 0x40110000;
pub const PVR_476: c_uint = 0x11a52000;
pub const PVR_476FPE: c_uint = 0x7ff50000;
pub const PVR_STB03XXX: c_uint = 0x40310000;
pub const PVR_NP405H: c_uint = 0x41410000;
pub const PVR_NP405L: c_uint = 0x41610000;
pub const PVR_601: c_uint = 0x00010000;
pub const PVR_602: c_uint = 0x00050000;
pub const PVR_603: c_uint = 0x00030000;
pub const PVR_603e: c_uint = 0x00060000;
pub const PVR_603ev: c_uint = 0x00070000;
pub const PVR_603r: c_uint = 0x00071000;
pub const PVR_604: c_uint = 0x00040000;
pub const PVR_604e: c_uint = 0x00090000;
pub const PVR_604r: c_uint = 0x000A0000;
pub const PVR_620: c_uint = 0x00140000;
pub const PVR_740: c_uint = 0x00080000;

pub const PVR_740P: c_uint = 0x10080000;

pub const PVR_7400: c_uint = 0x000C0000;
pub const PVR_7410: c_uint = 0x800C0000;
pub const PVR_7450: c_uint = 0x80000000;
pub const PVR_8540: c_uint = 0x80200000;
pub const PVR_8560: c_uint = 0x80200000;
pub const PVR_VER_E500V1: c_uint = 0x8020;
pub const PVR_VER_E500V2: c_uint = 0x8021;
pub const PVR_VER_E500MC: c_uint = 0x8023;
pub const PVR_VER_E5500: c_uint = 0x8024;
pub const PVR_VER_E6500: c_uint = 0x8040;
pub const PVR_VER_7450: c_uint = 0x8000;
pub const PVR_VER_7455: c_uint = 0x8001;
pub const PVR_VER_7447: c_uint = 0x8002;
pub const PVR_VER_7447A: c_uint = 0x8003;
pub const PVR_VER_7448: c_uint = 0x8004;
//
// For the 8xx processors, all of them report the same PVR family for
// the PowerPC core. The various versions of these processors must be
// differentiated by the version number in the Communication Processor
// Module (CPM).
//
pub const PVR_8xx: c_uint = 0x00500000;
pub const PVR_8240: c_uint = 0x00810100;
pub const PVR_8245: c_uint = 0x80811014;

// 476 Simulator seems to currently have the PVR of the 602...
pub const PVR_476_ISS: c_uint = 0x00052000;
// 64-bit processors
pub const PVR_NORTHSTAR: c_uint = 0x0033;
pub const PVR_PULSAR: c_uint = 0x0034;
pub const PVR_POWER4: c_uint = 0x0035;
pub const PVR_ICESTAR: c_uint = 0x0036;
pub const PVR_SSTAR: c_uint = 0x0037;
pub const PVR_POWER4p: c_uint = 0x0038;
pub const PVR_970: c_uint = 0x0039;
pub const PVR_POWER5: c_uint = 0x003A;
pub const PVR_POWER5p: c_uint = 0x003B;
pub const PVR_970FX: c_uint = 0x003C;
pub const PVR_POWER6: c_uint = 0x003E;
pub const PVR_POWER7: c_uint = 0x003F;
pub const PVR_630: c_uint = 0x0040;
pub const PVR_630p: c_uint = 0x0041;
pub const PVR_970MP: c_uint = 0x0044;
pub const PVR_970GX: c_uint = 0x0045;
pub const PVR_POWER7p: c_uint = 0x004A;
pub const PVR_POWER8E: c_uint = 0x004B;
pub const PVR_POWER8NVL: c_uint = 0x004C;
pub const PVR_POWER8: c_uint = 0x004D;
pub const PVR_HX_C2000: c_uint = 0x0066;
pub const PVR_POWER9: c_uint = 0x004E;
pub const PVR_POWER10: c_uint = 0x0080;
pub const PVR_POWER11: c_uint = 0x0082;
pub const PVR_POWER12: c_uint = 0x0083;
pub const PVR_BE: c_uint = 0x0070;
pub const PVR_PA6T: c_uint = 0x0090;
// "Logical" PVR values defined in PAPR, representing architecture levels
pub const PVR_ARCH_204: c_uint = 0x0f000001;
pub const PVR_ARCH_205: c_uint = 0x0f000002;
pub const PVR_ARCH_206: c_uint = 0x0f000003;
pub const PVR_ARCH_206p: c_uint = 0x0f100003;
pub const PVR_ARCH_207: c_uint = 0x0f000004;
pub const PVR_ARCH_300: c_uint = 0x0f000005;
pub const PVR_ARCH_31: c_uint = 0x0f000006;
pub const PVR_ARCH_31_P11: c_uint = 0x0f000007;
pub const PVR_ARCH_32: c_uint = 0x0f000008;
//
// Kernel-internal sentinel for invalid processor compatibility modes.
// PAPR specifies that the first byte of a valid logical PVR value is
// 0x0f. So 0xffffffff lies permanently outside the PAPR-defined range
// and is safe to repurpose. KVM stores it in vcpu->arch.arch_compat
// when userspace requests an unsupported compatibility mode (e.g.,
// Power11 PVR on a Power11 host booted in Power10 compat).
// kvmppc_sanity_check() detects this and prevents the vCPU from
// running with an unsupported arch_compat.
//
pub const PVR_ARCH_INVALID: c_uint = 0xffffffff;
// Macros for setting and retrieving special purpose registers

pub type ppc_inst_t = u32;

extern "C" {
    pub fn volatile("memory": "wrteei %0" : : "i" ((val & MSR_EE) ? 1 : 0) :) -> asm;
}
extern "C" {
    pub fn volatile("memory": "wrtee %0" : : "r" (val) :) -> asm;
}
extern "C" {
    pub fn msr_check_and_set(bits: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn __msr_check_and_clear(bits: c_ulong);
}

extern "C" {
    pub fn volatile(%0: "mfsr, 28): %1" : "=r" (val): "i" (idx >>) -> asm;
}
extern "C" {
    pub fn volatile(%0: "mfsrin, (idx): %1" : "=r" (val): "r") -> asm;
}
extern "C" {
    pub fn volatile(%1: "mtsr, (val): %0" : : "r", 28): "i" (idx >>) -> asm;
}
extern "C" {
    pub fn volatile(%0: "mtsrin, (val): %1" : : "r", (idx): "r") -> asm;
}

extern "C" {
    pub fn current_stack_frame() -> c_ulong;
}
extern "C" {
    pub fn asm(_arg: "r1") -> register unsigned long current_stack_pointer;
}
extern "C" {
    pub fn scom970_read(address: c_uint) -> c_ulong;
}
extern "C" {
    pub fn scom970_write(address: c_uint, value: c_ulong);
}
extern "C" {
    pub fn ppc_save_regs(regs: *mut pt_regs);
}

