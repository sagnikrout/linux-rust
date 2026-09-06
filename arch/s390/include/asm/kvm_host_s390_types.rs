//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/kvm_host_s390_types.h
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

pub const KVM_S390_BSCA_CPU_SLOTS: c_int = 64;
pub const KVM_S390_ESCA_CPU_SLOTS: c_int = 248;
pub const SCB_ALIGNMENT_SHIFT: c_int = 9;
pub const SIGP_CTRL_C: c_uint = 0x80;
pub const SIGP_CTRL_SCN_MASK: c_uint = 0x3f;
#[repr(C)]
#[derive(Copy, Clone)]
pub union bsca_sigp_ctrl {
    pub value: __u8,
    pub 1: __u8 c :,
    pub 1: __u8 r :,
    pub 6: __u8 scn :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union esca_sigp_ctrl {
    pub value: __u16,
    pub 1: __u8 c :,
    pub 7: __u8 reserved:,
    pub scn: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esca_entry {
    pub sigp_ctrl: esca_sigp_ctrl,
    pub reserved1: [__u16; 3],
    pub sda: __u64,
    pub reserved2: [__u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsca_entry {
    pub reserved0: __u8,
    pub sigp_ctrl: bsca_sigp_ctrl,
    pub reserved: [__u16; 3],
    pub sda: __u64,
    pub reserved2: [__u64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ipte_control {
    pub val: c_ulong,
    pub 1: unsigned long k :,
    pub 31: unsigned long kh :,
    pub 32: unsigned long kg :,
}

//
// Utility is defined as two bytes but having it four bytes wide
// generates more efficient code. Since the following bytes are
// reserved this makes no functional difference.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union sca_utility {
    pub val: __u32,
    pub 1: __u32 mtcr :,
    pub 31: __u32 :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bsca_block {
    pub ipte_control: ipte_control,
    pub reserved: [__u64; 5],
    pub mcn: __u64,
    pub utility: sca_utility,
    pub reserved2: [__u8; 4],
    pub cpu: [bsca_entry; KVM_S390_BSCA_CPU_SLOTS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct esca_block {
    pub ipte_control: ipte_control,
    pub reserved1: [__u64; 6],
    pub utility: sca_utility,
    pub reserved2: [__u8; 4],
    pub mcn: [__u64; 4],
    pub reserved3: [__u64; 20],
    pub cpu: [esca_entry; KVM_S390_ESCA_CPU_SLOTS],
}

//
// This struct is used to store some machine check info from lowcore
// for machine checks that happen while the guest is running.
// This info in host's lowcore might be overwritten by a second machine
// check from host when host is in the machine check's high-level handling.
// The size is 24 bytes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcck_volatile_info {
    pub mcic: __u64,
    pub failing_storage_address: __u64,
    pub ext_damage_code: __u32,
    pub reserved: __u32,
}

pub const SIDAD_SIZE_MASK: c_uint = 0xff;

pub const CPUSTAT_STOPPED: c_uint = 0x80000000;
pub const CPUSTAT_WAIT: c_uint = 0x10000000;
pub const CPUSTAT_ECALL_PEND: c_uint = 0x08000000;
pub const CPUSTAT_STOP_INT: c_uint = 0x04000000;
pub const CPUSTAT_IO_INT: c_uint = 0x02000000;
pub const CPUSTAT_EXT_INT: c_uint = 0x01000000;
pub const CPUSTAT_RUNNING: c_uint = 0x00800000;
pub const CPUSTAT_RETAINED: c_uint = 0x00400000;
pub const CPUSTAT_TIMING_SUB: c_uint = 0x00020000;
pub const CPUSTAT_SIE_SUB: c_uint = 0x00010000;
pub const CPUSTAT_RRF: c_uint = 0x00008000;
pub const CPUSTAT_SLSV: c_uint = 0x00004000;
pub const CPUSTAT_SLSR: c_uint = 0x00002000;
pub const CPUSTAT_ZARCH: c_uint = 0x00000800;
pub const CPUSTAT_MCDS: c_uint = 0x00000100;
pub const CPUSTAT_KSS: c_uint = 0x00000200;
pub const CPUSTAT_SM: c_uint = 0x00000080;
pub const CPUSTAT_IBS: c_uint = 0x00000040;
pub const CPUSTAT_GED2: c_uint = 0x00000010;
pub const CPUSTAT_G: c_uint = 0x00000008;
pub const CPUSTAT_GED: c_uint = 0x00000004;
pub const CPUSTAT_J: c_uint = 0x00000002;
pub const CPUSTAT_P: c_uint = 0x00000001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_sie_block {
    pub /: *mut *mut atomic_t cpuflags; / 0x0000,
    pub /: *mut *mut __u32 : 1; / 0x0004,
    pub 19: __u32 prefix :,
    pub 12: __u32 ibc :,
    pub /: *mut *mut __u8 reserved08[4]; / 0x0008,

    pub /: *mut *mut __u32 prog0c; / 0x000c,
    pub /: *mut *mut __u8 reserved10[16]; / 0x0010,
    pub pv_handle_cpu: __u64,
    pub pv_handle_config: __u64,
}

pub const LCTL_CR0: c_uint = 0x8000;
pub const LCTL_CR6: c_uint = 0x0200;
pub const LCTL_CR9: c_uint = 0x0040;
pub const LCTL_CR10: c_uint = 0x0020;
pub const LCTL_CR11: c_uint = 0x0010;
pub const LCTL_CR14: c_uint = 0x0002;
pub const ICTL_OPEREXC: c_uint = 0x80000000;
pub const ICTL_PINT: c_uint = 0x20000000;
pub const ICTL_LPSW: c_uint = 0x00400000;
pub const ICTL_STCTL: c_uint = 0x00040000;
pub const ICTL_ISKE: c_uint = 0x00004000;
pub const ICTL_SSKE: c_uint = 0x00002000;
pub const ICTL_RRBE: c_uint = 0x00001000;
pub const ICTL_TPROT: c_uint = 0x00000200;
pub const ECA_CEI: c_uint = 0x80000000;
pub const ECA_IB: c_uint = 0x40000000;
pub const ECA_SIGPI: c_uint = 0x10000000;
pub const ECA_MVPGI: c_uint = 0x01000000;
pub const ECA_AIV: c_uint = 0x00200000;
pub const ECA_VX: c_uint = 0x00020000;
pub const ECA_PROTEXCI: c_uint = 0x00002000;
pub const ECA_APIE: c_uint = 0x00000008;
pub const ECA_SII: c_uint = 0x00000001;
pub const ICPT_INST: c_uint = 0x04;
pub const ICPT_PROGI: c_uint = 0x08;
pub const ICPT_INSTPROGI: c_uint = 0x0C;
pub const ICPT_EXTREQ: c_uint = 0x10;
pub const ICPT_EXTINT: c_uint = 0x14;
pub const ICPT_IOREQ: c_uint = 0x18;
pub const ICPT_WAIT: c_uint = 0x1c;
pub const ICPT_VALIDITY: c_uint = 0x20;
pub const ICPT_STOP: c_uint = 0x28;
pub const ICPT_OPEREXC: c_uint = 0x2C;
pub const ICPT_PARTEXEC: c_uint = 0x38;
pub const ICPT_IOINST: c_uint = 0x40;
pub const ICPT_KSS: c_uint = 0x5c;
pub const ICPT_MCHKREQ: c_uint = 0x60;
pub const ICPT_INT_ENABLE: c_uint = 0x64;
pub const ICPT_PV_INSTR: c_uint = 0x68;
pub const ICPT_PV_NOTIFY: c_uint = 0x6c;
pub const ICPT_PV_PREF: c_uint = 0x70;
pub const IICTL_CODE_NONE: c_uint = 0x00;
pub const IICTL_CODE_MCHK: c_uint = 0x01;
pub const IICTL_CODE_EXT: c_uint = 0x02;
pub const IICTL_CODE_IO: c_uint = 0x03;
pub const IICTL_CODE_RESTART: c_uint = 0x04;
pub const IICTL_CODE_SPECIFICATION: c_uint = 0x10;
pub const IICTL_CODE_OPERAND: c_uint = 0x11;
pub const FPF_BPBC: c_uint = 0x20;
pub const ECB_GS: c_uint = 0x40;
pub const ECB_TE: c_uint = 0x10;
pub const ECB_SPECI: c_uint = 0x08;
pub const ECB_SRSI: c_uint = 0x04;
pub const ECB_HOSTPROTINT: c_uint = 0x02;
pub const ECB_PTF: c_uint = 0x01;
pub const ECB2_CMMA: c_uint = 0x80;
pub const ECB2_IEP: c_uint = 0x20;
pub const ECB2_PFMFI: c_uint = 0x08;
pub const ECB2_ESCA: c_uint = 0x04;
pub const ECB2_ZPCI_LSI: c_uint = 0x02;
pub const ECB3_AISI: c_uint = 0x20;
pub const ECB3_AISII: c_uint = 0x10;
pub const ECB3_DEA: c_uint = 0x08;
pub const ECB3_AES: c_uint = 0x04;
pub const ECB3_RI: c_uint = 0x01;

pub const GISA_FORMAT1: c_uint = 0x00000001;
pub const HPID_KVM: c_uint = 0x4;
pub const HPID_VSIE: c_uint = 0x5;
pub const CRYCB_FORMAT_MASK: c_uint = 0x00000003;
pub const CRYCB_FORMAT0: c_uint = 0x00000000;
pub const CRYCB_FORMAT1: c_uint = 0x00000001;
pub const CRYCB_FORMAT2: c_uint = 0x00000003;
pub const ECD_HOSTREGMGMT: c_uint = 0x20000000;
pub const ECD_MEF: c_uint = 0x08000000;
pub const ECD_ETOKENF: c_uint = 0x02000000;
pub const ECD_ECC: c_uint = 0x00200000;
pub const ECD_HMAC: c_uint = 0x00004000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_itdb {
    pub data: [__u8; 256],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sie_page {
    pub sie_block: kvm_s390_sie_block,
    pub /: *mut *mut mcck_volatile_info mcck_info; / 0x0200,
    pub /: *mut *mut __u8 reserved218[360]; / 0x0218,
    pub /: *mut *mut __u64 pv_grregs[16]; / 0x0380,
    pub /: *mut *mut __u8 reserved400[512]; / 0x0400,
    pub /: *mut *mut kvm_s390_itdb itdb; / 0x0600,
    pub /: *mut *mut __u8 reserved700[2304]; / 0x0700,
}
