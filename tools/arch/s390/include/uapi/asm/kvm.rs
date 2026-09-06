//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/s390/include/uapi/asm/kvm.h
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
//
// KVM s390 specific structures and definitions
//
// Copyright IBM Corp. 2008, 2018
//
// Author(s): Carsten Otte <cotte@de.ibm.com>
// Christian Borntraeger <borntraeger@de.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_skeys {
    pub start_gfn: __u64,
    pub count: __u64,
    pub skeydata_addr: __u64,
    pub flags: __u32,
    pub reserved: [__u32; 9],
}

//
// kvm_s390_cmma_log - Used for CMMA migration.
//
// Used both for input and output.
//
// @start_gfn: Guest page number to start from.
// @count: Size of the result buffer.
// @flags: Control operation mode via KVM_S390_CMMA_* flags
// @remaining: Used with KVM_S390_GET_CMMA_BITS. Indicates how many dirty
// pages are still remaining.
// @mask: Used with KVM_S390_SET_CMMA_BITS. Bitmap of bits to actually set
// in the PGSTE.
// @values: Pointer to the values buffer.
//
// Used in KVM_S390_{G,S}ET_CMMA_BITS ioctls.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_cmma_log {
    pub start_gfn: __u64,
    pub count: __u32,
    pub flags: __u32,
    pub remaining: __u64,
    pub mask: __u64,
}

pub const KVM_S390_RESET_POR: c_int = 1;
pub const KVM_S390_RESET_CLEAR: c_int = 2;
pub const KVM_S390_RESET_SUBSYSTEM: c_int = 4;
pub const KVM_S390_RESET_CPU_INIT: c_int = 8;
pub const KVM_S390_RESET_IPL: c_int = 16;
// for KVM_S390_MEM_OP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_mem_op {
// in
    pub /: *mut *mut __u64 gaddr; / the guest address,
    pub /: *mut *mut __u64 flags; / flags,
    pub /: *mut *mut __u32 size; / amount of bytes,
    pub /: *mut *mut __u32 op; / type of operation,
    pub /: *mut *mut __u64 buf; / buffer in userspace,
    pub /: *mut *mut __u8 ar; / the access register number,
    pub /: *mut *mut __u8 key; / access key, ignored if flag unset,
    pub /: *mut *mut __u8 pad1[6]; / ignored,
    pub /: *mut *mut __u64 old_addr; / ignored if cmpxchg flag unset,
}

// types for kvm_s390_mem_op->op
pub const KVM_S390_MEMOP_LOGICAL_READ: c_int = 0;
pub const KVM_S390_MEMOP_LOGICAL_WRITE: c_int = 1;
pub const KVM_S390_MEMOP_SIDA_READ: c_int = 2;
pub const KVM_S390_MEMOP_SIDA_WRITE: c_int = 3;
pub const KVM_S390_MEMOP_ABSOLUTE_READ: c_int = 4;
pub const KVM_S390_MEMOP_ABSOLUTE_WRITE: c_int = 5;
pub const KVM_S390_MEMOP_ABSOLUTE_CMPXCHG: c_int = 6;
// flags for kvm_s390_mem_op->flags

// flags specifying extension support via KVM_CAP_S390_MEM_OP_EXTENSION

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_psw {
    pub mask: __u64,
    pub addr: __u64,
}

// valid values for type in kvm_s390_interrupt
pub const KVM_S390_SIGP_STOP: c_uint = 0xfffe0000u;
pub const KVM_S390_PROGRAM_INT: c_uint = 0xfffe0001u;
pub const KVM_S390_SIGP_SET_PREFIX: c_uint = 0xfffe0002u;
pub const KVM_S390_RESTART: c_uint = 0xfffe0003u;
pub const KVM_S390_INT_PFAULT_INIT: c_uint = 0xfffe0004u;
pub const KVM_S390_INT_PFAULT_DONE: c_uint = 0xfffe0005u;
pub const KVM_S390_MCHK: c_uint = 0xfffe1000u;
pub const KVM_S390_INT_CLOCK_COMP: c_uint = 0xffff1004u;
pub const KVM_S390_INT_CPU_TIMER: c_uint = 0xffff1005u;
pub const KVM_S390_INT_VIRTIO: c_uint = 0xffff2603u;
pub const KVM_S390_INT_SERVICE: c_uint = 0xffff2401u;
pub const KVM_S390_INT_EMERGENCY: c_uint = 0xffff1201u;
pub const KVM_S390_INT_EXTERNAL_CALL: c_uint = 0xffff1202u;
// Anything below 0xfffe0000u is taken by INT_IO

pub const KVM_S390_INT_IO_MIN: c_uint = 0x00000000u;
pub const KVM_S390_INT_IO_MAX: c_uint = 0xfffdffffu;
pub const KVM_S390_INT_IO_AI_MASK: c_uint = 0x04000000u;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_interrupt {
    pub type: __u32,
    pub parm: __u32,
    pub parm64: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_io_info {
    pub subchannel_id: __u16,
    pub subchannel_nr: __u16,
    pub io_int_parm: __u32,
    pub io_int_word: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ext_info {
    pub ext_params: __u32,
    pub pad: __u32,
    pub ext_params2: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pgm_info {
    pub trans_exc_code: __u64,
    pub mon_code: __u64,
    pub per_address: __u64,
    pub data_exc_code: __u32,
    pub code: __u16,
    pub mon_class_nr: __u16,
    pub per_code: __u8,
    pub per_atmid: __u8,
    pub exc_access_id: __u8,
    pub per_access_id: __u8,
    pub op_access_id: __u8,
pub const KVM_S390_PGM_FLAGS_ILC_VALID: c_uint = 0x01;
pub const KVM_S390_PGM_FLAGS_ILC_0: c_uint = 0x02;
pub const KVM_S390_PGM_FLAGS_ILC_1: c_uint = 0x04;
pub const KVM_S390_PGM_FLAGS_ILC_MASK: c_uint = 0x06;
pub const KVM_S390_PGM_FLAGS_NO_REWIND: c_uint = 0x08;
    pub flags: __u8,
    pub pad: [__u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_prefix_info {
    pub address: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_extcall_info {
    pub code: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_emerg_info {
    pub code: __u16,
}

pub const KVM_S390_STOP_FLAG_STORE_STATUS: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_stop_info {
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_mchk_info {
    pub cr14: __u64,
    pub mcic: __u64,
    pub failing_storage_address: __u64,
    pub ext_damage_code: __u32,
    pub pad: __u32,
    pub fixed_logout: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_irq {
    pub type: __u64,
    pub io: kvm_s390_io_info,
    pub ext: kvm_s390_ext_info,
    pub pgm: kvm_s390_pgm_info,
    pub emerg: kvm_s390_emerg_info,
    pub extcall: kvm_s390_extcall_info,
    pub prefix: kvm_s390_prefix_info,
    pub stop: kvm_s390_stop_info,
    pub mchk: kvm_s390_mchk_info,
    pub reserved: [c_char; 64],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_irq_state {
    pub buf: __u64,
    pub /: *mut *mut __u32 flags; / will stay unused for compatibility reasons,
    pub len: __u32,
    pub /: *mut *mut __u32 reserved[4]; / will stay unused for compatibility reasons,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ucas_mapping {
    pub user_addr: __u64,
    pub vcpu_addr: __u64,
    pub length: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_sec_parm {
    pub origin: __u64,
    pub length: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_unp {
    pub addr: __u64,
    pub size: __u64,
    pub tweak: __u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pv_cmd_dmp_id {
    KVM_PV_DUMP_INIT,
    KVM_PV_DUMP_CONFIG_STOR_STATE,
    KVM_PV_DUMP_COMPLETE,
    KVM_PV_DUMP_CPU,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_dmp {
    pub subcmd: __u64,
    pub buff_addr: __u64,
    pub buff_len: __u64,
    pub /: *mut *mut __u64 gaddr; / For dump storage state,
    pub reserved: [__u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pv_cmd_info_id {
    KVM_PV_INFO_VM,
    KVM_PV_INFO_DUMP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info_dump {
    pub dump_cpu_buffer_len: __u64,
    pub dump_config_mem_buffer_per_1m: __u64,
    pub dump_config_finalize_len: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info_vm {
    pub inst_calls_list: [__u64; 4],
    pub max_cpus: __u64,
    pub max_guests: __u64,
    pub max_guest_addr: __u64,
    pub feature_indication: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info_header {
    pub id: __u32,
    pub len_max: __u32,
    pub len_written: __u32,
    pub reserved: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_pv_info {
    pub header: kvm_s390_pv_info_header,
    pub dump: kvm_s390_pv_info_dump,
    pub vm: kvm_s390_pv_info_vm,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pv_cmd_id {
    KVM_PV_ENABLE,
    KVM_PV_DISABLE,
    KVM_PV_SET_SEC_PARMS,
    KVM_PV_UNPACK,
    KVM_PV_VERIFY,
    KVM_PV_PREP_RESET,
    KVM_PV_UNSHARE_ALL,
    KVM_PV_INFO,
    KVM_PV_DUMP,
    KVM_PV_ASYNC_CLEANUP_PREPARE,
    KVM_PV_ASYNC_CLEANUP_PERFORM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pv_cmd {
    pub /: *mut *mut __u32 cmd; / Command to be executed,
    pub /: *mut *mut __u16 rc; / Ultravisor return code,
    pub /: *mut *mut __u16 rrc; / Ultravisor return reason code,
    pub /: *mut *mut __u64 data; / Data or address,
    pub /: *mut *mut __u32 flags; / flags for future extensions. Must be 0 for now,
    pub reserved: [__u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_zpci_op {
// in
    pub /: *mut *mut __u32 fh; / target device,
    pub /: *mut *mut __u8 op; / operation to perform,
    pub pad: [__u8; 3],
// for KVM_S390_ZPCIOP_REG_AEN
    pub /: *mut *mut __u64 ibv; / Guest addr of interrupt bit vector,
    pub /: *mut *mut __u64 sb; / Guest addr of summary bit,
    pub flags: __u32,
    pub /: *mut *mut __u32 noi; / Number of interrupts,
    pub /: *mut *mut __u8 isc; / Guest interrupt subclass,
    pub /: *mut *mut __u8 sbo; / Offset of guest summary bit vector,
    pub pad: __u16,
    pub reg_aen: },
    pub reserved: [__u64; 8],
    pub u: },
}

// types for kvm_s390_zpci_op->op
pub const KVM_S390_ZPCIOP_REG_AEN: c_int = 0;
pub const KVM_S390_ZPCIOP_DEREG_AEN: c_int = 1;
// flags for kvm_s390_zpci_op->u.reg_aen.flags

// Device control API: s390-specific devices
pub const KVM_DEV_FLIC_GET_ALL_IRQS: c_int = 1;
pub const KVM_DEV_FLIC_ENQUEUE: c_int = 2;
pub const KVM_DEV_FLIC_CLEAR_IRQS: c_int = 3;
pub const KVM_DEV_FLIC_APF_ENABLE: c_int = 4;
pub const KVM_DEV_FLIC_APF_DISABLE_WAIT: c_int = 5;
pub const KVM_DEV_FLIC_ADAPTER_REGISTER: c_int = 6;
pub const KVM_DEV_FLIC_ADAPTER_MODIFY: c_int = 7;
pub const KVM_DEV_FLIC_CLEAR_IO_IRQ: c_int = 8;
pub const KVM_DEV_FLIC_AISM: c_int = 9;
pub const KVM_DEV_FLIC_AIRQ_INJECT: c_int = 10;
pub const KVM_DEV_FLIC_AISM_ALL: c_int = 11;
//
// We can have up to 4*64k pending subchannels + 8 adapter interrupts,
// as well as up  to ASYNC_PF_PER_VCPU*KVM_MAX_VCPUS pfault done interrupts.
// There are also sclp and machine checks. This gives us
// sizeof(kvm_s390_irq)*(4*65536+8+64*64+1+1) = 72 * 266250 = 19170000
// Lets round up to 8192 pages.
//
pub const KVM_S390_MAX_FLOAT_IRQS: c_int = 266250;
pub const KVM_S390_FLIC_MAX_BUFFER: c_uint = 0x2000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_io_adapter {
    pub id: __u32,
    pub isc: __u8,
    pub maskable: __u8,
    pub swap: __u8,
    pub flags: __u8,
}

pub const KVM_S390_ADAPTER_SUPPRESSIBLE: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ais_req {
    pub isc: __u8,
    pub mode: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_ais_all {
    pub simm: __u8,
    pub nimm: __u8,
}

pub const KVM_S390_IO_ADAPTER_MASK: c_int = 1;
pub const KVM_S390_IO_ADAPTER_MAP: c_int = 2;
pub const KVM_S390_IO_ADAPTER_UNMAP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_io_adapter_req {
    pub id: __u32,
    pub type: __u8,
    pub mask: __u8,
    pub pad0: __u16,
    pub addr: __u64,
}

// kvm attr_group  on vm fd
pub const KVM_S390_VM_MEM_CTRL: c_int = 0;
pub const KVM_S390_VM_TOD: c_int = 1;
pub const KVM_S390_VM_CRYPTO: c_int = 2;
pub const KVM_S390_VM_CPU_MODEL: c_int = 3;
pub const KVM_S390_VM_MIGRATION: c_int = 4;
pub const KVM_S390_VM_CPU_TOPOLOGY: c_int = 5;
// kvm attributes for mem_ctrl
pub const KVM_S390_VM_MEM_ENABLE_CMMA: c_int = 0;
pub const KVM_S390_VM_MEM_CLR_CMMA: c_int = 1;
pub const KVM_S390_VM_MEM_LIMIT_SIZE: c_int = 2;

// kvm attributes for KVM_S390_VM_TOD
pub const KVM_S390_VM_TOD_LOW: c_int = 0;
pub const KVM_S390_VM_TOD_HIGH: c_int = 1;
pub const KVM_S390_VM_TOD_EXT: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_tod_clock {
    pub epoch_idx: __u8,
    pub tod: __u64,
}

// kvm attributes for KVM_S390_VM_CPU_MODEL
// processor related attributes are r/w
pub const KVM_S390_VM_CPU_PROCESSOR: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_processor {
    pub cpuid: __u64,
    pub ibc: __u16,
    pub pad: [__u8; 6],
    pub fac_list: [__u64; 256],
}

// machine related attributes are r/o
pub const KVM_S390_VM_CPU_MACHINE: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_machine {
    pub cpuid: __u64,
    pub ibc: __u32,
    pub pad: [__u8; 4],
    pub fac_mask: [__u64; 256],
    pub fac_list: [__u64; 256],
}

pub const KVM_S390_VM_CPU_PROCESSOR_FEAT: c_int = 2;
pub const KVM_S390_VM_CPU_MACHINE_FEAT: c_int = 3;
pub const KVM_S390_VM_CPU_FEAT_NR_BITS: c_int = 1024;
pub const KVM_S390_VM_CPU_FEAT_ESOP: c_int = 0;
pub const KVM_S390_VM_CPU_FEAT_SIEF2: c_int = 1;
pub const KVM_S390_VM_CPU_FEAT_64BSCAO: c_int = 2;
pub const KVM_S390_VM_CPU_FEAT_SIIF: c_int = 3;
pub const KVM_S390_VM_CPU_FEAT_GPERE: c_int = 4;
pub const KVM_S390_VM_CPU_FEAT_GSLS: c_int = 5;
pub const KVM_S390_VM_CPU_FEAT_IB: c_int = 6;
pub const KVM_S390_VM_CPU_FEAT_CEI: c_int = 7;
pub const KVM_S390_VM_CPU_FEAT_IBS: c_int = 8;
pub const KVM_S390_VM_CPU_FEAT_SKEY: c_int = 9;
pub const KVM_S390_VM_CPU_FEAT_CMMA: c_int = 10;
pub const KVM_S390_VM_CPU_FEAT_PFMFI: c_int = 11;
pub const KVM_S390_VM_CPU_FEAT_SIGPIF: c_int = 12;
pub const KVM_S390_VM_CPU_FEAT_KSS: c_int = 13;
pub const KVM_S390_VM_CPU_FEAT_ASTFLEIE2: c_int = 14;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_feat {
    pub feat: [__u64; 16],
}

pub const KVM_S390_VM_CPU_PROCESSOR_SUBFUNC: c_int = 4;
pub const KVM_S390_VM_CPU_MACHINE_SUBFUNC: c_int = 5;
// for "test bit" instructions MSB 0 bit ordering, for "query" raw blocks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_subfunc {
    pub /: *mut *mut __u8 plo[32]; / always,
    pub /: *mut *mut __u8 ptff[16]; / with TOD-clock steering,
    pub /: *mut *mut __u8 kmac[16]; / with MSA,
    pub /: *mut *mut __u8 kmc[16]; / with MSA,
    pub /: *mut *mut __u8 km[16]; / with MSA,
    pub /: *mut *mut __u8 kimd[16]; / with MSA,
    pub /: *mut *mut __u8 klmd[16]; / with MSA,
    pub /: *mut *mut __u8 pckmo[16]; / with MSA3,
    pub /: *mut *mut __u8 kmctr[16]; / with MSA4,
    pub /: *mut *mut __u8 kmf[16]; / with MSA4,
    pub /: *mut *mut __u8 kmo[16]; / with MSA4,
    pub /: *mut *mut __u8 pcc[16]; / with MSA4,
    pub /: *mut *mut __u8 ppno[16]; / with MSA5,
    pub /: *mut *mut __u8 kma[16]; / with MSA8,
    pub /: *mut *mut __u8 kdsa[16]; / with MSA9,
    pub /: *mut *mut __u8 sortl[32]; / with STFLE.150,
    pub /: *mut *mut __u8 dfltcc[32]; / with STFLE.151,
    pub /: *mut *mut __u8 pfcr[16]; / with STFLE.201,
    pub reserved: [__u8; 1712],
}

pub const KVM_S390_VM_CPU_PROCESSOR_UV_FEAT_GUEST: c_int = 6;
pub const KVM_S390_VM_CPU_MACHINE_UV_FEAT_GUEST: c_int = 7;
pub const KVM_S390_VM_CPU_UV_FEAT_NR_BITS: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_vm_cpu_uv_feat {
    pub 4: __u64 :,
    pub /: *mut *mut __u64 ap : 1; / bit 4,
    pub /: *mut *mut __u64 ap_intr : 1; / bit 5,
    pub 58: __u64 :,
}

// kvm attributes for crypto
pub const KVM_S390_VM_CRYPTO_ENABLE_AES_KW: c_int = 0;
pub const KVM_S390_VM_CRYPTO_ENABLE_DEA_KW: c_int = 1;
pub const KVM_S390_VM_CRYPTO_DISABLE_AES_KW: c_int = 2;
pub const KVM_S390_VM_CRYPTO_DISABLE_DEA_KW: c_int = 3;
pub const KVM_S390_VM_CRYPTO_ENABLE_APIE: c_int = 4;
pub const KVM_S390_VM_CRYPTO_DISABLE_APIE: c_int = 5;
// kvm attributes for migration mode
pub const KVM_S390_VM_MIGRATION_STOP: c_int = 0;
pub const KVM_S390_VM_MIGRATION_START: c_int = 1;
pub const KVM_S390_VM_MIGRATION_STATUS: c_int = 2;
// for KVM_GET_REGS and KVM_SET_REGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
// general purpose regs for s390
    pub gprs: [__u64; 16],
}

// for KVM_GET_SREGS and KVM_SET_SREGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
    pub acrs: [__u32; 16],
    pub crs: [__u64; 16],
}

// for KVM_GET_FPU and KVM_SET_FPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
    pub fpc: __u32,
    pub fprs: [__u64; 16],
}

pub const KVM_GUESTDBG_USE_HW_BP: c_uint = 0x00010000;
pub const KVM_HW_BP: c_int = 1;
pub const KVM_HW_WP_WRITE: c_int = 2;
pub const KVM_SINGLESTEP: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
    pub addr: __u64,
    pub type: __u8,
    pub /: *mut *mut __u8 pad[7]; / Should be set to 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hw_breakpoint {
    pub addr: __u64,
    pub phys_addr: __u64,
    pub len: __u64,
    pub type: __u8,
    pub /: *mut *mut __u8 pad[7]; / Should be set to 0,
}

// for KVM_SET_GUEST_DEBUG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
    pub nr_hw_bp: __u32,
    pub /: *mut *mut __u32 pad; / Should be set to 0,
    pub hw_bp: *mut kvm_hw_breakpoint __user,
}

// for KVM_SYNC_PFAULT and KVM_REG_S390_PFTOKEN
pub const KVM_S390_PFAULT_TOKEN_INVALID: c_uint = 0xffffffffffffffffULL;

// length and alignment of the sdnx as a power of two
pub const SDNXC: c_int = 8;

// definition of registers in kvm_run
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {
    pub /: *mut *mut __u64 prefix; / prefix register,
    pub /: *mut *mut __u64 gprs[16]; / general purpose registers,
    pub /: *mut *mut __u32 acrs[16]; / access registers,
    pub /: *mut *mut __u64 crs[16]; / control registers,
    pub /: *mut *mut __u64 todpr; / tod programmable register [ARCH0],
    pub /: *mut *mut __u64 cputm; / cpu timer [ARCH0],
    pub /: *mut *mut __u64 ckc; / clock comparator [ARCH0],
    pub /: *mut *mut __u64 pp; / program parameter [ARCH0],
    pub /: *mut *mut __u64 gbea; / guest breaking-event address [ARCH0],
    pub /: *mut *mut __u64 pft; / pfault token [PFAULT],
    pub /: *mut *mut __u64 pfs; / pfault select [PFAULT],
    pub /: *mut *mut __u64 pfc; / pfault compare [PFAULT],
    pub /: *mut *mut __u64 vrs[32][2]; / vector registers (KVM_SYNC_VRS),
    pub /: *mut *mut __u64 fprs[16]; / fp registers (KVM_SYNC_FPRS),
}

