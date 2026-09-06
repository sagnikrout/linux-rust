//! Automatically rewritten from C Header to Rust Module
//! Source: tools/include/uapi/linux/kvm.h
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
// Userspace interface for /dev/kvm - kernel based virtual machine
//
// Note: you must update KVM_API_VERSION if you change this interface.
//

pub const KVM_API_VERSION: c_int = 12;
//
// Backwards-compatible definitions.
//
// for KVM_SET_USER_MEMORY_REGION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_userspace_memory_region {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub /: *mut *mut __u64 memory_size; / bytes,
    pub /: *mut *mut __u64 userspace_addr; / start of the userspace allocated memory,
}

// for KVM_SET_USER_MEMORY_REGION2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_userspace_memory_region2 {
    pub slot: __u32,
    pub flags: __u32,
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
    pub userspace_addr: __u64,
    pub guest_memfd_offset: __u64,
    pub guest_memfd: __u32,
    pub pad1: __u32,
    pub pad2: [__u64; 14],
}

//
// The bit 0 ~ bit 15 of kvm_userspace_memory_region::flags are visible for
// userspace, other bits are reserved for kvm internal use which are defined
// in include/linux/kvm_host.h.
//

// for KVM_IRQ_LINE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_level {
//
// ACPI gsi notion of irq.
// For IA-64 (APIC model) IOAPIC0: irq 0-23; IOAPIC1: irq 24-47..
// For X86 (standard AT mode) PIC0/1: irq 0-15. IOAPIC0: 0-23..
// For ARM: See Documentation/virt/kvm/api.rst
//
    pub irq: __u32,
    pub status: __s32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irqchip {
    pub chip_id: __u32,
    pub pad: __u32,
    pub /: *mut *mut char dummy[512]; / reserving space,

    pub pic: kvm_pic_state,

    pub ioapic: kvm_ioapic_state,

    pub chip: },
}

// for KVM_CREATE_PIT2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pit_config {
    pub flags: __u32,
    pub pad: [__u32; 15],
}

pub const KVM_PIT_SPEAKER_DUMMY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hyperv_exit {
pub const KVM_EXIT_HYPERV_SYNIC: c_int = 1;
pub const KVM_EXIT_HYPERV_HCALL: c_int = 2;
pub const KVM_EXIT_HYPERV_SYNDBG: c_int = 3;
    pub type: __u32,
    pub pad1: __u32,
    pub msr: __u32,
    pub pad2: __u32,
    pub control: __u64,
    pub evt_page: __u64,
    pub msg_page: __u64,
    pub synic: },
    pub input: __u64,
    pub result: __u64,
    pub params: [__u64; 2],
    pub hcall: },
    pub msr: __u32,
    pub pad2: __u32,
    pub control: __u64,
    pub status: __u64,
    pub send_page: __u64,
    pub recv_page: __u64,
    pub pending_page: __u64,
    pub syndbg: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xen_exit {
pub const KVM_EXIT_XEN_HCALL: c_int = 1;
    pub type: __u32,
    pub longmode: __u32,
    pub cpl: __u32,
    pub input: __u64,
    pub result: __u64,
    pub params: [__u64; 6],
    pub hcall: },
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_exit_snp_req_certs {
    pub gpa: __u64,
    pub npages: __u64,
    pub ret: __u64,
}

pub const KVM_S390_GET_SKEYS_NONE: c_int = 1;
pub const KVM_S390_SKEYS_MAX: c_int = 1048576;
pub const KVM_EXIT_UNKNOWN: c_int = 0;
pub const KVM_EXIT_EXCEPTION: c_int = 1;
pub const KVM_EXIT_IO: c_int = 2;
pub const KVM_EXIT_HYPERCALL: c_int = 3;
pub const KVM_EXIT_DEBUG: c_int = 4;
pub const KVM_EXIT_HLT: c_int = 5;
pub const KVM_EXIT_MMIO: c_int = 6;
pub const KVM_EXIT_IRQ_WINDOW_OPEN: c_int = 7;
pub const KVM_EXIT_SHUTDOWN: c_int = 8;
pub const KVM_EXIT_FAIL_ENTRY: c_int = 9;
pub const KVM_EXIT_INTR: c_int = 10;
pub const KVM_EXIT_SET_TPR: c_int = 11;
pub const KVM_EXIT_TPR_ACCESS: c_int = 12;
pub const KVM_EXIT_S390_SIEIC: c_int = 13;
pub const KVM_EXIT_S390_RESET: c_int = 14;

pub const KVM_EXIT_NMI: c_int = 16;
pub const KVM_EXIT_INTERNAL_ERROR: c_int = 17;
pub const KVM_EXIT_OSI: c_int = 18;
pub const KVM_EXIT_PAPR_HCALL: c_int = 19;
pub const KVM_EXIT_S390_UCONTROL: c_int = 20;
pub const KVM_EXIT_WATCHDOG: c_int = 21;
pub const KVM_EXIT_S390_TSCH: c_int = 22;
pub const KVM_EXIT_EPR: c_int = 23;
pub const KVM_EXIT_SYSTEM_EVENT: c_int = 24;
pub const KVM_EXIT_S390_STSI: c_int = 25;
pub const KVM_EXIT_IOAPIC_EOI: c_int = 26;
pub const KVM_EXIT_HYPERV: c_int = 27;
pub const KVM_EXIT_ARM_NISV: c_int = 28;
pub const KVM_EXIT_X86_RDMSR: c_int = 29;
pub const KVM_EXIT_X86_WRMSR: c_int = 30;
pub const KVM_EXIT_DIRTY_RING_FULL: c_int = 31;
pub const KVM_EXIT_AP_RESET_HOLD: c_int = 32;
pub const KVM_EXIT_X86_BUS_LOCK: c_int = 33;
pub const KVM_EXIT_XEN: c_int = 34;
pub const KVM_EXIT_RISCV_SBI: c_int = 35;
pub const KVM_EXIT_RISCV_CSR: c_int = 36;
pub const KVM_EXIT_NOTIFY: c_int = 37;
pub const KVM_EXIT_LOONGARCH_IOCSR: c_int = 38;
pub const KVM_EXIT_MEMORY_FAULT: c_int = 39;
pub const KVM_EXIT_TDX: c_int = 40;
pub const KVM_EXIT_ARM_SEA: c_int = 41;
pub const KVM_EXIT_ARM_LDST64B: c_int = 42;
pub const KVM_EXIT_SNP_REQ_CERTS: c_int = 43;
// For KVM_EXIT_INTERNAL_ERROR
// Emulate instruction failed.
pub const KVM_INTERNAL_ERROR_EMULATION: c_int = 1;
// Encounter unexpected simultaneous exceptions.
pub const KVM_INTERNAL_ERROR_SIMUL_EX: c_int = 2;
// Encounter unexpected vm-exit due to delivery event.
pub const KVM_INTERNAL_ERROR_DELIVERY_EV: c_int = 3;
// Encounter unexpected vm-exit reason
pub const KVM_INTERNAL_ERROR_UNEXPECTED_EXIT_REASON: c_int = 4;
// Flags that describe what fields in emulation_failure hold valid data.

//
// struct kvm_run can be modified by userspace at any time, so KVM must be
// careful to avoid TOCTOU bugs. In order to protect KVM, HINT_UNSAFE_IN_KVM()
// renames fields in struct kvm_run from <symbol> to <symbol>__unsafe when
// compiled into the kernel, ensuring that any use within KVM is obvious and
// gets extra scrutiny.
//

// for KVM_RUN, returned by mmap(vcpu_fd, offset=0)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_run {
// in
    pub request_interrupt_window: __u8,
    pub HINT_UNSAFE_IN_KVM(immediate_exit): __u8,
    pub padding1: [__u8; 6],
// out
    pub exit_reason: __u32,
    pub ready_for_interrupt_injection: __u8,
    pub if_flag: __u8,
    pub flags: __u16,
// in (pre_kvm_run), out (post_kvm_run)
    pub cr8: __u64,
    pub apic_base: __u64,

// the processor status word for s390
    pub /: *mut *mut __u64 psw_mask; / psw upper half,
    pub /: *mut *mut __u64 psw_addr; / psw lower half,

// KVM_EXIT_UNKNOWN
    pub hardware_exit_reason: __u64,
    pub hw: },
// KVM_EXIT_FAIL_ENTRY
    pub hardware_entry_failure_reason: __u64,
    pub cpu: __u32,
    pub fail_entry: },
// KVM_EXIT_EXCEPTION
    pub exception: __u32,
    pub error_code: __u32,
    pub ex: },
// KVM_EXIT_IO
pub const KVM_EXIT_IO_IN: c_int = 0;
pub const KVM_EXIT_IO_OUT: c_int = 1;
    pub direction: __u8,
    pub /: *mut *mut __u8 size; / bytes,
    pub port: __u16,
    pub count: __u32,
    pub /: *mut *mut __u64 data_offset; / relative to kvm_run start,
    pub io: },
// KVM_EXIT_DEBUG
    pub arch: kvm_debug_exit_arch,
    pub debug: },
// KVM_EXIT_MMIO
    pub phys_addr: __u64,
    pub data: [__u8; 8],
    pub len: __u32,
    pub is_write: __u8,
    pub mmio: },
// KVM_EXIT_LOONGARCH_IOCSR
    pub phys_addr: __u64,
    pub data: [__u8; 8],
    pub len: __u32,
    pub is_write: __u8,
    pub iocsr_io: },
// KVM_EXIT_HYPERCALL
    pub nr: __u64,
    pub args: [__u64; 6],
    pub ret: __u64,
    pub longmode: __u32,

    pub flags: __u64,
}

// KVM_EXIT_TPR_ACCESS
// KVM_EXIT_S390_SIEIC
// KVM_EXIT_S390_RESET
// KVM_EXIT_S390_UCONTROL
// KVM_EXIT_DCR (deprecated)
// KVM_EXIT_INTERNAL_ERROR
// Available with KVM_CAP_INTERNAL_ERROR_DATA:
//
// KVM_INTERNAL_ERROR_EMULATION
//
// "struct emulation_failure" is an overlay of "struct internal"
// that is used for the KVM_INTERNAL_ERROR_EMULATION sub-type of
// KVM_EXIT_INTERNAL_ERROR.  Note, unlike other internal error
// sub-types, this struct is ABI!  It also needs to be backwards
// compatible with "struct internal".  Take special care that
// "ndata" is correct, that new fields are enumerated in "flags",
// and that each flag enumerates fields that are 64-bit aligned
// and sized (so that ndata+internal.data[] is valid/accurate).
//
// Space beyond the defined fields may be used to store arbitrary
// debug information relating to the emulation failure. It is
// accounted for in "ndata" but the format is unspecified and is
// not represented in "flags". Any such information is *not* ABI!
//
// Arbitrary debug data may follow.
// KVM_EXIT_OSI
// KVM_EXIT_PAPR_HCALL
// KVM_EXIT_S390_TSCH
// KVM_EXIT_EPR
// KVM_EXIT_SYSTEM_EVENT
pub const KVM_SYSTEM_EVENT_SHUTDOWN: c_int = 1;
pub const KVM_SYSTEM_EVENT_RESET: c_int = 2;
pub const KVM_SYSTEM_EVENT_CRASH: c_int = 3;
pub const KVM_SYSTEM_EVENT_WAKEUP: c_int = 4;
pub const KVM_SYSTEM_EVENT_SUSPEND: c_int = 5;
pub const KVM_SYSTEM_EVENT_SEV_TERM: c_int = 6;
pub const KVM_SYSTEM_EVENT_TDX_FATAL: c_int = 7;

// KVM_EXIT_S390_STSI
// KVM_EXIT_IOAPIC_EOI
// KVM_EXIT_HYPERV
// KVM_EXIT_ARM_NISV / KVM_EXIT_ARM_LDST64B
// KVM_EXIT_X86_RDMSR / KVM_EXIT_X86_WRMSR

// KVM_EXIT_XEN
// KVM_EXIT_RISCV_SBI
// KVM_EXIT_RISCV_CSR
// KVM_EXIT_NOTIFY

// KVM_EXIT_MEMORY_FAULT

// KVM_EXIT_TDX
// KVM_EXIT_ARM_SEA

// KVM_EXIT_SNP_REQ_CERTS
// Fix the size of the union.
// 2048 is the size of the char array used to bound/pad the size
// of the union that holds sync regs.
//
pub const SYNC_REGS_SIZE_BYTES: c_int = 2048;
//
// shared registers between kvm and userspace.
// kvm_valid_regs specifies the register classes set by the host
// kvm_dirty_regs specified the register classes dirtied by userspace
// struct kvm_sync_regs is architecture specific, as well as the
// bits for kvm_valid_regs and kvm_dirty_regs
//
// for KVM_REGISTER_COALESCED_MMIO / KVM_UNREGISTER_COALESCED_MMIO
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_coalesced_mmio_zone {
    pub addr: __u64,
    pub size: __u32,
    pub pad: __u32,
    pub pio: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_coalesced_mmio {
    pub phys_addr: __u64,
    pub len: __u32,
    pub pad: __u32,
    pub pio: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_coalesced_mmio_ring {
    pub last: __u32 first,,
    pub coalesced_mmio): __DECLARE_FLEX_ARRAY(struct kvm_coalesced_mmio,,
}

// for KVM_TRANSLATE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_translation {
// in
    pub linear_address: __u64,
// out
    pub physical_address: __u64,
    pub valid: __u8,
    pub writeable: __u8,
    pub usermode: __u8,
    pub pad: [__u8; 5],
}

// for KVM_INTERRUPT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_interrupt {
// in
    pub irq: __u32,
}

// for KVM_GET_DIRTY_LOG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_dirty_log {
    pub slot: __u32,
    pub padding1: __u32,
    pub /: *mut *mut *mut void __user dirty_bitmap; / one bit per page,
    pub padding2: __u64,
}

// for KVM_CLEAR_DIRTY_LOG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_clear_dirty_log {
    pub slot: __u32,
    pub num_pages: __u32,
    pub first_page: __u64,
    pub /: *mut *mut *mut void __user dirty_bitmap; / one bit per page,
    pub padding2: __u64,
}

// for KVM_SET_SIGNAL_MASK
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_signal_mask {
    pub len: __u32,
    pub sigset): __DECLARE_FLEX_ARRAY(__u8,,
}

// for KVM_TPR_ACCESS_REPORTING
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_tpr_access_ctl {
    pub enabled: __u32,
    pub flags: __u32,
    pub reserved: [__u32; 8],
}

// for KVM_SET_VAPIC_ADDR
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vapic_addr {
    pub vapic_addr: __u64,
}

// for KVM_SET_MP_STATE
// not all states are valid on all architectures
pub const KVM_MP_STATE_RUNNABLE: c_int = 0;
pub const KVM_MP_STATE_UNINITIALIZED: c_int = 1;
pub const KVM_MP_STATE_INIT_RECEIVED: c_int = 2;
pub const KVM_MP_STATE_HALTED: c_int = 3;
pub const KVM_MP_STATE_SIPI_RECEIVED: c_int = 4;
pub const KVM_MP_STATE_STOPPED: c_int = 5;
pub const KVM_MP_STATE_CHECK_STOP: c_int = 6;
pub const KVM_MP_STATE_OPERATING: c_int = 7;
pub const KVM_MP_STATE_LOAD: c_int = 8;
pub const KVM_MP_STATE_AP_RESET_HOLD: c_int = 9;
pub const KVM_MP_STATE_SUSPENDED: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_mp_state {
    pub mp_state: __u32,
}

// for KVM_SET_GUEST_DEBUG
pub const KVM_GUESTDBG_ENABLE: c_uint = 0x00000001;
pub const KVM_GUESTDBG_SINGLESTEP: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug {
    pub control: __u32,
    pub pad: __u32,
    pub arch: kvm_guest_debug_arch,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ioeventfd {
    pub datamatch: __u64,
    pub /: *mut *mut __u64 addr; / legal pio/mmio address,
    pub /: *mut *mut __u32 len; / 1, 2, 4, or 8 bytes; or 0 to ignore length,
    pub fd: __s32,
    pub flags: __u32,
    pub pad: [__u8; 36],
}

// for KVM_ENABLE_CAP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_enable_cap {
// in
    pub cap: __u32,
    pub flags: __u32,
    pub args: [__u64; 4],
    pub pad: [__u8; 64],
}

pub const KVMIO: c_uint = 0xAE;
// machine type bits, to be used as argument to KVM_CREATE_VM
pub const KVM_VM_S390_UCONTROL: c_int = 1;
// on ppc, 0 indicate default, 1 should force HV and 2 PR
pub const KVM_VM_PPC_HV: c_int = 1;
pub const KVM_VM_PPC_PR: c_int = 2;
// on MIPS, 0 indicates auto, 1 forces VZ ASE, 2 forces trap & emulate
pub const KVM_VM_MIPS_AUTO: c_int = 0;
pub const KVM_VM_MIPS_VZ: c_int = 1;
pub const KVM_VM_MIPS_TE: c_int = 2;
pub const KVM_S390_SIE_PAGE_OFFSET: c_int = 1;
//
// On arm64, machine type can be used to request the physical
// address size for the VM. Bits[7-0] are reserved for the guest
// PA size shift (i.e, log2(PA_Size)). For backward compatibility,
// value 0 implies the default IPA size, 40bits.
//
pub const KVM_VM_TYPE_ARM_IPA_SIZE_MASK: c_uint = 0xffULL;

//
// ioctls for /dev/kvm fds:
//

//
// Check if a kvm extension is available.  Argument is extension number,
// return is 1 (yes) or 0 (no, sorry).
//

//
// Get size for mmap(vcpu_fd)
//

//
// Extension capability list.
//
pub const KVM_CAP_IRQCHIP: c_int = 0;
pub const KVM_CAP_HLT: c_int = 1;
pub const KVM_CAP_MMU_SHADOW_CACHE_CONTROL: c_int = 2;
pub const KVM_CAP_USER_MEMORY: c_int = 3;
pub const KVM_CAP_SET_TSS_ADDR: c_int = 4;
pub const KVM_CAP_VAPIC: c_int = 6;
pub const KVM_CAP_EXT_CPUID: c_int = 7;
pub const KVM_CAP_CLOCKSOURCE: c_int = 8;

pub const KVM_CAP_PIT: c_int = 11;
pub const KVM_CAP_NOP_IO_DELAY: c_int = 12;
pub const KVM_CAP_PV_MMU: c_int = 13;
pub const KVM_CAP_MP_STATE: c_int = 14;
pub const KVM_CAP_COALESCED_MMIO: c_int = 15;

pub const KVM_CAP_IOMMU: c_int = 18;
// Bug in KVM_SET_USER_MEMORY_REGION fixed:
pub const KVM_CAP_DESTROY_MEMORY_REGION_WORKS: c_int = 21;
pub const KVM_CAP_USER_NMI: c_int = 22;
pub const KVM_CAP_SET_GUEST_DEBUG: c_int = 23;

pub const KVM_CAP_REINJECT_CONTROL: c_int = 24;

pub const KVM_CAP_IRQ_ROUTING: c_int = 25;
pub const KVM_CAP_IRQ_INJECT_STATUS: c_int = 26;
pub const KVM_CAP_ASSIGN_DEV_IRQ: c_int = 29;
// Another bug in KVM_SET_USER_MEMORY_REGION fixed:
pub const KVM_CAP_JOIN_MEMORY_REGIONS_WORKS: c_int = 30;

pub const KVM_CAP_MCE: c_int = 31;

pub const KVM_CAP_IRQFD: c_int = 32;

pub const KVM_CAP_PIT2: c_int = 33;

pub const KVM_CAP_SET_BOOT_CPU_ID: c_int = 34;

pub const KVM_CAP_PIT_STATE2: c_int = 35;

pub const KVM_CAP_IOEVENTFD: c_int = 36;
pub const KVM_CAP_SET_IDENTITY_MAP_ADDR: c_int = 37;

pub const KVM_CAP_XEN_HVM: c_int = 38;

pub const KVM_CAP_ADJUST_CLOCK: c_int = 39;
pub const KVM_CAP_INTERNAL_ERROR_DATA: c_int = 40;

pub const KVM_CAP_VCPU_EVENTS: c_int = 41;

pub const KVM_CAP_S390_PSW: c_int = 42;
pub const KVM_CAP_PPC_SEGSTATE: c_int = 43;
pub const KVM_CAP_HYPERV: c_int = 44;
pub const KVM_CAP_HYPERV_VAPIC: c_int = 45;
pub const KVM_CAP_HYPERV_SPIN: c_int = 46;
pub const KVM_CAP_PCI_SEGMENT: c_int = 47;
pub const KVM_CAP_PPC_PAIRED_SINGLES: c_int = 48;
pub const KVM_CAP_INTR_SHADOW: c_int = 49;

pub const KVM_CAP_DEBUGREGS: c_int = 50;

pub const KVM_CAP_X86_ROBUST_SINGLESTEP: c_int = 51;
pub const KVM_CAP_PPC_OSI: c_int = 52;
pub const KVM_CAP_PPC_UNSET_IRQ: c_int = 53;
pub const KVM_CAP_ENABLE_CAP: c_int = 54;

pub const KVM_CAP_XSAVE: c_int = 55;

pub const KVM_CAP_XCRS: c_int = 56;

pub const KVM_CAP_PPC_GET_PVINFO: c_int = 57;
pub const KVM_CAP_PPC_IRQ_LEVEL: c_int = 58;
pub const KVM_CAP_ASYNC_PF: c_int = 59;
pub const KVM_CAP_TSC_CONTROL: c_int = 60;
pub const KVM_CAP_GET_TSC_KHZ: c_int = 61;
pub const KVM_CAP_PPC_BOOKE_SREGS: c_int = 62;
pub const KVM_CAP_SPAPR_TCE: c_int = 63;
pub const KVM_CAP_PPC_SMT: c_int = 64;
pub const KVM_CAP_PPC_RMA: c_int = 65;

pub const KVM_CAP_PPC_HIOR: c_int = 67;
pub const KVM_CAP_PPC_PAPR: c_int = 68;
pub const KVM_CAP_SW_TLB: c_int = 69;
pub const KVM_CAP_ONE_REG: c_int = 70;
pub const KVM_CAP_S390_GMAP: c_int = 71;
pub const KVM_CAP_TSC_DEADLINE_TIMER: c_int = 72;
pub const KVM_CAP_S390_UCONTROL: c_int = 73;
pub const KVM_CAP_SYNC_REGS: c_int = 74;
pub const KVM_CAP_PCI_2_3: c_int = 75;
pub const KVM_CAP_KVMCLOCK_CTRL: c_int = 76;
pub const KVM_CAP_SIGNAL_MSI: c_int = 77;
pub const KVM_CAP_PPC_GET_SMMU_INFO: c_int = 78;
pub const KVM_CAP_S390_COW: c_int = 79;
pub const KVM_CAP_PPC_ALLOC_HTAB: c_int = 80;
pub const KVM_CAP_READONLY_MEM: c_int = 81;
pub const KVM_CAP_IRQFD_RESAMPLE: c_int = 82;
pub const KVM_CAP_PPC_BOOKE_WATCHDOG: c_int = 83;
pub const KVM_CAP_PPC_HTAB_FD: c_int = 84;
pub const KVM_CAP_S390_CSS_SUPPORT: c_int = 85;
pub const KVM_CAP_PPC_EPR: c_int = 86;
pub const KVM_CAP_ARM_PSCI: c_int = 87;
pub const KVM_CAP_ARM_SET_DEVICE_ADDR: c_int = 88;
pub const KVM_CAP_DEVICE_CTRL: c_int = 89;
pub const KVM_CAP_IRQ_MPIC: c_int = 90;
pub const KVM_CAP_PPC_RTAS: c_int = 91;
pub const KVM_CAP_IRQ_XICS: c_int = 92;
pub const KVM_CAP_ARM_EL1_32BIT: c_int = 93;
pub const KVM_CAP_SPAPR_MULTITCE: c_int = 94;
pub const KVM_CAP_EXT_EMUL_CPUID: c_int = 95;
pub const KVM_CAP_HYPERV_TIME: c_int = 96;
pub const KVM_CAP_IOAPIC_POLARITY_IGNORED: c_int = 97;
pub const KVM_CAP_ENABLE_CAP_VM: c_int = 98;
pub const KVM_CAP_S390_IRQCHIP: c_int = 99;
pub const KVM_CAP_IOEVENTFD_NO_LENGTH: c_int = 100;
pub const KVM_CAP_VM_ATTRIBUTES: c_int = 101;
pub const KVM_CAP_ARM_PSCI_0_2: c_int = 102;
pub const KVM_CAP_PPC_FIXUP_HCALL: c_int = 103;
pub const KVM_CAP_PPC_ENABLE_HCALL: c_int = 104;
pub const KVM_CAP_CHECK_EXTENSION_VM: c_int = 105;
pub const KVM_CAP_S390_USER_SIGP: c_int = 106;
pub const KVM_CAP_S390_VECTOR_REGISTERS: c_int = 107;
pub const KVM_CAP_S390_MEM_OP: c_int = 108;
pub const KVM_CAP_S390_USER_STSI: c_int = 109;
pub const KVM_CAP_S390_SKEYS: c_int = 110;
pub const KVM_CAP_MIPS_FPU: c_int = 111;
pub const KVM_CAP_MIPS_MSA: c_int = 112;
pub const KVM_CAP_S390_INJECT_IRQ: c_int = 113;
pub const KVM_CAP_S390_IRQ_STATE: c_int = 114;
pub const KVM_CAP_PPC_HWRNG: c_int = 115;
pub const KVM_CAP_DISABLE_QUIRKS: c_int = 116;
pub const KVM_CAP_X86_SMM: c_int = 117;
pub const KVM_CAP_MULTI_ADDRESS_SPACE: c_int = 118;
pub const KVM_CAP_GUEST_DEBUG_HW_BPS: c_int = 119;
pub const KVM_CAP_GUEST_DEBUG_HW_WPS: c_int = 120;
pub const KVM_CAP_SPLIT_IRQCHIP: c_int = 121;
pub const KVM_CAP_IOEVENTFD_ANY_LENGTH: c_int = 122;
pub const KVM_CAP_HYPERV_SYNIC: c_int = 123;
pub const KVM_CAP_S390_RI: c_int = 124;
pub const KVM_CAP_SPAPR_TCE_64: c_int = 125;
pub const KVM_CAP_ARM_PMU_V3: c_int = 126;
pub const KVM_CAP_VCPU_ATTRIBUTES: c_int = 127;
pub const KVM_CAP_MAX_VCPU_ID: c_int = 128;
pub const KVM_CAP_X2APIC_API: c_int = 129;
pub const KVM_CAP_S390_USER_INSTR0: c_int = 130;
pub const KVM_CAP_MSI_DEVID: c_int = 131;
pub const KVM_CAP_PPC_HTM: c_int = 132;
pub const KVM_CAP_SPAPR_RESIZE_HPT: c_int = 133;
pub const KVM_CAP_PPC_MMU_RADIX: c_int = 134;
pub const KVM_CAP_PPC_MMU_HASH_V3: c_int = 135;
pub const KVM_CAP_IMMEDIATE_EXIT: c_int = 136;
pub const KVM_CAP_MIPS_VZ: c_int = 137;
pub const KVM_CAP_MIPS_TE: c_int = 138;
pub const KVM_CAP_MIPS_64BIT: c_int = 139;
pub const KVM_CAP_S390_GS: c_int = 140;
pub const KVM_CAP_S390_AIS: c_int = 141;
pub const KVM_CAP_SPAPR_TCE_VFIO: c_int = 142;
pub const KVM_CAP_X86_DISABLE_EXITS: c_int = 143;
pub const KVM_CAP_ARM_USER_IRQ: c_int = 144;
pub const KVM_CAP_S390_CMMA_MIGRATION: c_int = 145;
pub const KVM_CAP_PPC_FWNMI: c_int = 146;
pub const KVM_CAP_PPC_SMT_POSSIBLE: c_int = 147;
pub const KVM_CAP_HYPERV_SYNIC2: c_int = 148;
pub const KVM_CAP_HYPERV_VP_INDEX: c_int = 149;
pub const KVM_CAP_S390_AIS_MIGRATION: c_int = 150;
pub const KVM_CAP_PPC_GET_CPU_CHAR: c_int = 151;
pub const KVM_CAP_S390_BPB: c_int = 152;
pub const KVM_CAP_GET_MSR_FEATURES: c_int = 153;
pub const KVM_CAP_HYPERV_EVENTFD: c_int = 154;
pub const KVM_CAP_HYPERV_TLBFLUSH: c_int = 155;
pub const KVM_CAP_S390_HPAGE_1M: c_int = 156;
pub const KVM_CAP_NESTED_STATE: c_int = 157;
pub const KVM_CAP_ARM_INJECT_SERROR_ESR: c_int = 158;
pub const KVM_CAP_MSR_PLATFORM_INFO: c_int = 159;
pub const KVM_CAP_PPC_NESTED_HV: c_int = 160;
pub const KVM_CAP_HYPERV_SEND_IPI: c_int = 161;
pub const KVM_CAP_COALESCED_PIO: c_int = 162;
pub const KVM_CAP_HYPERV_ENLIGHTENED_VMCS: c_int = 163;
pub const KVM_CAP_EXCEPTION_PAYLOAD: c_int = 164;
pub const KVM_CAP_ARM_VM_IPA_SIZE: c_int = 165;

pub const KVM_CAP_HYPERV_CPUID: c_int = 167;
pub const KVM_CAP_MANUAL_DIRTY_LOG_PROTECT2: c_int = 168;
pub const KVM_CAP_PPC_IRQ_XIVE: c_int = 169;
pub const KVM_CAP_ARM_SVE: c_int = 170;
pub const KVM_CAP_ARM_PTRAUTH_ADDRESS: c_int = 171;
pub const KVM_CAP_ARM_PTRAUTH_GENERIC: c_int = 172;
pub const KVM_CAP_PMU_EVENT_FILTER: c_int = 173;
pub const KVM_CAP_ARM_IRQ_LINE_LAYOUT_2: c_int = 174;
pub const KVM_CAP_HYPERV_DIRECT_TLBFLUSH: c_int = 175;
pub const KVM_CAP_PPC_GUEST_DEBUG_SSTEP: c_int = 176;
pub const KVM_CAP_ARM_NISV_TO_USER: c_int = 177;
pub const KVM_CAP_ARM_INJECT_EXT_DABT: c_int = 178;
pub const KVM_CAP_S390_VCPU_RESETS: c_int = 179;
pub const KVM_CAP_S390_PROTECTED: c_int = 180;
pub const KVM_CAP_PPC_SECURE_GUEST: c_int = 181;
pub const KVM_CAP_HALT_POLL: c_int = 182;
pub const KVM_CAP_ASYNC_PF_INT: c_int = 183;
pub const KVM_CAP_LAST_CPU: c_int = 184;
pub const KVM_CAP_SMALLER_MAXPHYADDR: c_int = 185;
pub const KVM_CAP_S390_DIAG318: c_int = 186;
pub const KVM_CAP_STEAL_TIME: c_int = 187;
pub const KVM_CAP_X86_USER_SPACE_MSR: c_int = 188;
pub const KVM_CAP_X86_MSR_FILTER: c_int = 189;
pub const KVM_CAP_ENFORCE_PV_FEATURE_CPUID: c_int = 190;
pub const KVM_CAP_SYS_HYPERV_CPUID: c_int = 191;
pub const KVM_CAP_DIRTY_LOG_RING: c_int = 192;
pub const KVM_CAP_X86_BUS_LOCK_EXIT: c_int = 193;
pub const KVM_CAP_PPC_DAWR1: c_int = 194;
pub const KVM_CAP_SET_GUEST_DEBUG2: c_int = 195;
pub const KVM_CAP_SGX_ATTRIBUTE: c_int = 196;
pub const KVM_CAP_VM_COPY_ENC_CONTEXT_FROM: c_int = 197;
pub const KVM_CAP_PTP_KVM: c_int = 198;
pub const KVM_CAP_HYPERV_ENFORCE_CPUID: c_int = 199;
pub const KVM_CAP_SREGS2: c_int = 200;
pub const KVM_CAP_EXIT_HYPERCALL: c_int = 201;
pub const KVM_CAP_PPC_RPT_INVALIDATE: c_int = 202;
pub const KVM_CAP_BINARY_STATS_FD: c_int = 203;
pub const KVM_CAP_EXIT_ON_EMULATION_FAILURE: c_int = 204;
pub const KVM_CAP_ARM_MTE: c_int = 205;
pub const KVM_CAP_VM_MOVE_ENC_CONTEXT_FROM: c_int = 206;
pub const KVM_CAP_VM_GPA_BITS: c_int = 207;
pub const KVM_CAP_XSAVE2: c_int = 208;
pub const KVM_CAP_SYS_ATTRIBUTES: c_int = 209;
pub const KVM_CAP_PPC_AIL_MODE_3: c_int = 210;
pub const KVM_CAP_S390_MEM_OP_EXTENSION: c_int = 211;
pub const KVM_CAP_PMU_CAPABILITY: c_int = 212;
pub const KVM_CAP_DISABLE_QUIRKS2: c_int = 213;
pub const KVM_CAP_VM_TSC_CONTROL: c_int = 214;
pub const KVM_CAP_SYSTEM_EVENT_DATA: c_int = 215;
pub const KVM_CAP_ARM_SYSTEM_SUSPEND: c_int = 216;
pub const KVM_CAP_S390_PROTECTED_DUMP: c_int = 217;
pub const KVM_CAP_X86_TRIPLE_FAULT_EVENT: c_int = 218;
pub const KVM_CAP_X86_NOTIFY_VMEXIT: c_int = 219;
pub const KVM_CAP_VM_DISABLE_NX_HUGE_PAGES: c_int = 220;
pub const KVM_CAP_S390_ZPCI_OP: c_int = 221;
pub const KVM_CAP_S390_CPU_TOPOLOGY: c_int = 222;
pub const KVM_CAP_DIRTY_LOG_RING_ACQ_REL: c_int = 223;
pub const KVM_CAP_S390_PROTECTED_ASYNC_DISABLE: c_int = 224;
pub const KVM_CAP_DIRTY_LOG_RING_WITH_BITMAP: c_int = 225;
pub const KVM_CAP_PMU_EVENT_MASKED_EVENTS: c_int = 226;
pub const KVM_CAP_COUNTER_OFFSET: c_int = 227;
pub const KVM_CAP_ARM_EAGER_SPLIT_CHUNK_SIZE: c_int = 228;
pub const KVM_CAP_ARM_SUPPORTED_BLOCK_SIZES: c_int = 229;
pub const KVM_CAP_ARM_SUPPORTED_REG_MASK_RANGES: c_int = 230;
pub const KVM_CAP_USER_MEMORY2: c_int = 231;
pub const KVM_CAP_MEMORY_FAULT_INFO: c_int = 232;
pub const KVM_CAP_MEMORY_ATTRIBUTES: c_int = 233;
pub const KVM_CAP_GUEST_MEMFD: c_int = 234;
pub const KVM_CAP_VM_TYPES: c_int = 235;
pub const KVM_CAP_PRE_FAULT_MEMORY: c_int = 236;
pub const KVM_CAP_X86_APIC_BUS_CYCLES_NS: c_int = 237;
pub const KVM_CAP_X86_GUEST_MODE: c_int = 238;
pub const KVM_CAP_ARM_WRITABLE_IMP_ID_REGS: c_int = 239;
pub const KVM_CAP_ARM_EL2: c_int = 240;
pub const KVM_CAP_ARM_EL2_E2H0: c_int = 241;
pub const KVM_CAP_RISCV_MP_STATE_RESET: c_int = 242;
pub const KVM_CAP_ARM_CACHEABLE_PFNMAP_SUPPORTED: c_int = 243;
pub const KVM_CAP_GUEST_MEMFD_FLAGS: c_int = 244;
pub const KVM_CAP_ARM_SEA_TO_USER: c_int = 245;
pub const KVM_CAP_S390_USER_OPEREXEC: c_int = 246;
pub const KVM_CAP_S390_KEYOP: c_int = 247;
pub const KVM_CAP_S390_VSIE_ESAMODE: c_int = 248;
pub const KVM_CAP_S390_HPAGE_2G: c_int = 249;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_irqchip {
    pub irqchip: __u32,
    pub pin: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_msi {
    pub address_lo: __u32,
    pub address_hi: __u32,
    pub data: __u32,
    pub pad: __u32,
    pub devid: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_s390_adapter {
    pub ind_addr: __u64,
    pub summary_addr: __u64,
    pub ind_offset: __u64,
    pub summary_offset: __u32,
    pub adapter_id: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_hv_sint {
    pub vcpu: __u32,
    pub sint: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_xen_evtchn {
    pub port: __u32,
    pub vcpu: __u32,
    pub priority: __u32,
}

// gsi routing entry types
pub const KVM_IRQ_ROUTING_IRQCHIP: c_int = 1;
pub const KVM_IRQ_ROUTING_MSI: c_int = 2;
pub const KVM_IRQ_ROUTING_S390_ADAPTER: c_int = 3;
pub const KVM_IRQ_ROUTING_HV_SINT: c_int = 4;
pub const KVM_IRQ_ROUTING_XEN_EVTCHN: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing_entry {
    pub gsi: __u32,
    pub type: __u32,
    pub flags: __u32,
    pub pad: __u32,
    pub irqchip: kvm_irq_routing_irqchip,
    pub msi: kvm_irq_routing_msi,
    pub adapter: kvm_irq_routing_s390_adapter,
    pub hv_sint: kvm_irq_routing_hv_sint,
    pub xen_evtchn: kvm_irq_routing_xen_evtchn,
    pub pad: [__u32; 8],
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irq_routing {
    pub nr: __u32,
    pub flags: __u32,
    pub entries): __DECLARE_FLEX_ARRAY(struct kvm_irq_routing_entry,,
}

//
// Available with KVM_CAP_IRQFD_RESAMPLE
//
// KVM_IRQFD_FLAG_RESAMPLE indicates resamplefd is valid and specifies
// the irqfd to operate in resampling mode for level triggered interrupt
// emulation.  See Documentation/virt/kvm/api.rst.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_irqfd {
    pub fd: __u32,
    pub gsi: __u32,
    pub flags: __u32,
    pub resamplefd: __u32,
    pub pad: [__u8; 16],
}

// For KVM_CAP_ADJUST_CLOCK
// Do not use 1, KVM_CHECK_EXTENSION returned it before we had flags.
pub const KVM_CLOCK_TSC_STABLE: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_clock_data {
    pub clock: __u64,
    pub flags: __u32,
    pub pad0: __u32,
    pub realtime: __u64,
    pub host_tsc: __u64,
    pub pad: [__u32; 4],
}

// For KVM_CAP_SW_TLB
pub const KVM_MMU_FSL_BOOKE_NOHV: c_int = 0;
pub const KVM_MMU_FSL_BOOKE_HV: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_config_tlb {
    pub params: __u64,
    pub array: __u64,
    pub mmu_type: __u32,
    pub array_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_dirty_tlb {
    pub bitmap: __u64,
    pub num_dirty: __u32,
}

// Available with KVM_CAP_ONE_REG
pub const KVM_REG_ARCH_MASK: c_uint = 0xff00000000000000ULL;
pub const KVM_REG_GENERIC: c_uint = 0x0000000000000000ULL;
//
// Architecture specific registers are to be defined in arch headers and
// ORed with the arch identifier.
//
pub const KVM_REG_PPC: c_uint = 0x1000000000000000ULL;
pub const KVM_REG_X86: c_uint = 0x2000000000000000ULL;
pub const KVM_REG_IA64: c_uint = 0x3000000000000000ULL;
pub const KVM_REG_ARM: c_uint = 0x4000000000000000ULL;
pub const KVM_REG_S390: c_uint = 0x5000000000000000ULL;
pub const KVM_REG_ARM64: c_uint = 0x6000000000000000ULL;
pub const KVM_REG_MIPS: c_uint = 0x7000000000000000ULL;
pub const KVM_REG_RISCV: c_uint = 0x8000000000000000ULL;
pub const KVM_REG_LOONGARCH: c_uint = 0x9000000000000000ULL;
pub const KVM_REG_SIZE_SHIFT: c_int = 52;
pub const KVM_REG_SIZE_MASK: c_uint = 0x00f0000000000000ULL;

pub const KVM_REG_SIZE_U8: c_uint = 0x0000000000000000ULL;
pub const KVM_REG_SIZE_U16: c_uint = 0x0010000000000000ULL;
pub const KVM_REG_SIZE_U32: c_uint = 0x0020000000000000ULL;
pub const KVM_REG_SIZE_U64: c_uint = 0x0030000000000000ULL;
pub const KVM_REG_SIZE_U128: c_uint = 0x0040000000000000ULL;
pub const KVM_REG_SIZE_U256: c_uint = 0x0050000000000000ULL;
pub const KVM_REG_SIZE_U512: c_uint = 0x0060000000000000ULL;
pub const KVM_REG_SIZE_U1024: c_uint = 0x0070000000000000ULL;
pub const KVM_REG_SIZE_U2048: c_uint = 0x0080000000000000ULL;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_reg_list {
    pub /: *mut *mut __u64 n; / number of regs,
    pub reg): __DECLARE_FLEX_ARRAY(__u64,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_one_reg {
    pub id: __u64,
    pub addr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_msi {
    pub address_lo: __u32,
    pub address_hi: __u32,
    pub data: __u32,
    pub flags: __u32,
    pub devid: __u32,
    pub pad: [__u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_arm_device_addr {
    pub id: __u64,
    pub addr: __u64,
}

//
// Device control API, available with KVM_CAP_DEVICE_CTRL
//
pub const KVM_CREATE_DEVICE_TEST: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_create_device {
    pub /: *mut *mut __u32 type; / in: KVM_DEV_TYPE_xxx,
    pub /: *mut *mut __u32 fd; / out: device handle,
    pub /: *mut *mut __u32 flags; / in: KVM_CREATE_DEVICE_xxx,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_device_attr {
    pub /: *mut *mut __u32 flags; / no flags currently defined,
    pub /: *mut *mut __u32 group; / device-defined,
    pub /: *mut *mut __u64 attr; / group-defined,
    pub /: *mut *mut __u64 addr; / userspace address of attr data,
}

pub const KVM_DEV_VFIO_FILE: c_int = 1;
pub const KVM_DEV_VFIO_FILE_ADD: c_int = 1;
pub const KVM_DEV_VFIO_FILE_DEL: c_int = 2;
// KVM_DEV_VFIO_GROUP aliases are for compile time uapi compatibility

pub const KVM_DEV_VFIO_GROUP_SET_SPAPR_TCE: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_device_type {
    KVM_DEV_TYPE_FSL_MPIC_20	= 1,

    KVM_DEV_TYPE_FSL_MPIC_42,

    KVM_DEV_TYPE_XICS,

    KVM_DEV_TYPE_VFIO,

    KVM_DEV_TYPE_ARM_VGIC_V2,

    KVM_DEV_TYPE_FLIC,

    KVM_DEV_TYPE_ARM_VGIC_V3,

    KVM_DEV_TYPE_ARM_VGIC_ITS,

    KVM_DEV_TYPE_XIVE,

    KVM_DEV_TYPE_ARM_PV_TIME,

    KVM_DEV_TYPE_RISCV_AIA,

    KVM_DEV_TYPE_LOONGARCH_IPI,

    KVM_DEV_TYPE_LOONGARCH_EIOINTC,

    KVM_DEV_TYPE_LOONGARCH_PCHPIC,

    KVM_DEV_TYPE_LOONGARCH_DMSINTC,

    KVM_DEV_TYPE_ARM_VGIC_V5,

    KVM_DEV_TYPE_MAX,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vfio_spapr_tce {
    pub groupfd: __s32,
    pub tablefd: __s32,
}

pub const KVM_S390_KEYOP_ISKE: c_uint = 0x01;
pub const KVM_S390_KEYOP_RRBE: c_uint = 0x02;
pub const KVM_S390_KEYOP_SSKE: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_s390_keyop {
    pub guest_addr: __u64,
    pub key: __u8,
    pub operation: __u8,
    pub pad: [__u8; 6],
}

//
// KVM_CREATE_VCPU receives as a parameter the vcpu slot, and returns
// a vcpu fd.
//

// enable ucontrol for s390

// Device model IOC

// Available with KVM_CAP_PIT_STATE2

// Available with KVM_CAP_PPC_GET_PVINFO

// Available with KVM_CAP_TSC_CONTROL for a vCPU, or with
// KVM_CAP_VM_TSC_CONTROL to set defaults for a VM

// Available with KVM_CAP_SIGNAL_MSI

// Available with KVM_CAP_PPC_GET_SMMU_INFO

// Available with KVM_CAP_PPC_ALLOC_HTAB

// Available with KVM_CAP_RMA

// Available with KVM_CAP_PPC_HTAB_FD

// Available with KVM_CAP_ARM_SET_DEVICE_ADDR

// Available with KVM_CAP_PPC_RTAS

// Available with KVM_CAP_SPAPR_RESIZE_HPT

// Available with KVM_CAP_PPC_MMU_RADIX or KVM_CAP_PPC_MMU_HASH_V3

// Available with KVM_CAP_PPC_MMU_RADIX

// Available with KVM_CAP_PPC_GET_CPU_CHAR

// Available with KVM_CAP_PMU_EVENT_FILTER

// Available with KVM_CAP_COUNTER_OFFSET

// ioctl for vm fd

// ioctls for fds returned by KVM_CREATE_DEVICE

//
// ioctls for vcpu fds
//

// Available with KVM_CAP_VAPIC

// Available with KVM_CAP_VAPIC

// valid for virtual machine (for floating interrupt)_and_ vcpu

// store status for s390

// initial ipl psw for s390

// initial reset for s390

// Available with KVM_CAP_USER_NMI

// Available with KVM_CAP_SET_GUEST_DEBUG

// MCE for x86

// Available with KVM_CAP_VCPU_EVENTS

// Available with KVM_CAP_DEBUGREGS

//
// vcpu version available with KVM_CAP_ENABLE_CAP
// vm version available with KVM_CAP_ENABLE_CAP_VM
//

// Available with KVM_CAP_XSAVE

// Available with KVM_CAP_XCRS

// Available with KVM_CAP_SW_TLB

// Available with KVM_CAP_ONE_REG

// VM is being stopped by host

// Available with KVM_CAP_S390_MEM_OP

// Available with KVM_CAP_S390_SKEYS

// Available with KVM_CAP_S390_INJECT_IRQ

// Available with KVM_CAP_S390_IRQ_STATE

// Available with KVM_CAP_X86_SMM

// Available with KVM_CAP_S390_CMMA_MIGRATION

// Memory Encryption Commands

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_enc_region {
    pub addr: __u64,
    pub size: __u64,
}

// Available with KVM_CAP_HYPERV_EVENTFD

// Available with KVM_CAP_NESTED_STATE

// Available with KVM_CAP_MANUAL_DIRTY_LOG_PROTECT_2

// Available with KVM_CAP_HYPERV_CPUID (vcpu) / KVM_CAP_SYS_HYPERV_CPUID (system)

// Available with KVM_CAP_ARM_SVE

// Available with  KVM_CAP_S390_VCPU_RESETS

// Available with KVM_CAP_S390_PROTECTED

// Available with KVM_CAP_X86_MSR_FILTER

// Available with KVM_CAP_DIRTY_LOG_RING

// Per-VM Xen attributes

// Per-vCPU Xen attributes

// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND

//
// Arch needs to define the macro after implementing the dirty ring
// feature.  KVM_DIRTY_LOG_PAGE_OFFSET should be defined as the
// starting page offset of the dirty ring structures.
//

pub const KVM_DIRTY_LOG_PAGE_OFFSET: c_int = 0;

//
// KVM dirty GFN flags, defined as:
//
// |---------------+---------------+--------------|
// | bit 1 (reset) | bit 0 (dirty) | Status       |
// |---------------+---------------+--------------|
// |             0 |             0 | Invalid GFN  |
// |             0 |             1 | Dirty GFN    |
// |             1 |             X | GFN to reset |
// |---------------+---------------+--------------|
//
// Lifecycle of a dirty GFN goes like:
//
// dirtied         harvested        reset
// 00 -----------> 01 -------------> 1X -------+
// ^                                          |
// |                                          |
// +------------------------------------------+
//
// The userspace program is only responsible for the 01->1X state
// conversion after harvesting an entry.  Also, it must not skip any
// dirty bits, so that dirty bits are always harvested in sequence.
//

pub const KVM_DIRTY_GFN_F_MASK: c_uint = 0x3;
//
// KVM dirty rings should be mapped at KVM_DIRTY_LOG_PAGE_OFFSET of
// per-vcpu mmaped regions as an array of struct kvm_dirty_gfn.  The
// size of the gfn buffer is decided by the first argument when
// enabling KVM_CAP_DIRTY_LOG_RING.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_dirty_gfn {
    pub flags: __u32,
    pub slot: __u32,
    pub offset: __u64,
}

//
// struct kvm_stats_header - Header of per vm/vcpu binary statistics data.
// @flags: Some extra information for header, always 0 for now.
// @name_size: The size in bytes of the memory which contains statistics
// name string including trailing '\0'. The memory is allocated
// at the send of statistics descriptor.
// @num_desc: The number of statistics the vm or vcpu has.
// @id_offset: The offset of the vm/vcpu stats' id string in the file pointed
// by vm/vcpu stats fd.
// @desc_offset: The offset of the vm/vcpu stats' descriptor block in the file
// pointd by vm/vcpu stats fd.
// @data_offset: The offset of the vm/vcpu stats' data block in the file
// pointed by vm/vcpu stats fd.
//
// This is the header userspace needs to read from stats fd before any other
// readings. It is used by userspace to discover all the information about the
// vm/vcpu's binary statistics.
// Userspace reads this header from the start of the vm/vcpu's stats fd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_stats_header {
    pub flags: __u32,
    pub name_size: __u32,
    pub num_desc: __u32,
    pub id_offset: __u32,
    pub desc_offset: __u32,
    pub data_offset: __u32,
}

pub const KVM_STATS_TYPE_SHIFT: c_int = 0;

pub const KVM_STATS_UNIT_SHIFT: c_int = 4;

pub const KVM_STATS_BASE_SHIFT: c_int = 8;

//
// struct kvm_stats_desc - Descriptor of a KVM statistics.
// @flags: Annotations of the stats, like type, unit, etc.
// @exponent: Used together with @flags to determine the unit.
// @size: The number of data items for this stats.
// Every data item is of type __u64.
// @offset: The offset of the stats to the start of stat structure in
// structure kvm or kvm_vcpu.
// @bucket_size: A parameter value used for histogram stats. It is only used
// for linear histogram stats, specifying the size of the bucket;
// @name: The name string for the stats. Its size is indicated by the
// &kvm_stats_header->name_size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_stats_desc {
    pub flags: __u32,
    pub exponent: __s16,
    pub size: __u16,
    pub offset: __u32,
    pub bucket_size: __u32,
    pub name: [c_char; KVM_STATS_NAME_SIZE],
    pub name): __DECLARE_FLEX_ARRAY(char,,

}

// Available with KVM_CAP_XSAVE2

// Available with KVM_CAP_S390_PROTECTED_DUMP

// Available with KVM_CAP_X86_NOTIFY_VMEXIT

// Available with KVM_CAP_S390_ZPCI_OP

// Available with KVM_CAP_MEMORY_ATTRIBUTES

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_memory_attributes {
    pub address: __u64,
    pub size: __u64,
    pub attributes: __u64,
    pub flags: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_create_guest_memfd {
    pub size: __u64,
    pub flags: __u64,
    pub reserved: [__u64; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pre_fault_memory {
    pub gpa: __u64,
    pub size: __u64,
    pub flags: __u64,
    pub padding: [__u64; 5],
}
