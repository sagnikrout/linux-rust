//! Automatically rewritten from C Header to Rust Module
//! Source: tools/arch/x86/include/uapi/asm/kvm.h
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
// KVM x86 specific structures and definitions
//

pub const KVM_PIO_PAGE_OFFSET: c_int = 1;
pub const KVM_COALESCED_MMIO_PAGE_OFFSET: c_int = 2;
pub const KVM_DIRTY_LOG_PAGE_OFFSET: c_int = 64;
pub const DE_VECTOR: c_int = 0;
pub const DB_VECTOR: c_int = 1;
pub const BP_VECTOR: c_int = 3;
pub const OF_VECTOR: c_int = 4;
pub const BR_VECTOR: c_int = 5;
pub const UD_VECTOR: c_int = 6;
pub const NM_VECTOR: c_int = 7;
pub const DF_VECTOR: c_int = 8;
pub const TS_VECTOR: c_int = 10;
pub const NP_VECTOR: c_int = 11;
pub const SS_VECTOR: c_int = 12;
pub const GP_VECTOR: c_int = 13;
pub const PF_VECTOR: c_int = 14;
pub const MF_VECTOR: c_int = 16;
pub const AC_VECTOR: c_int = 17;
pub const MC_VECTOR: c_int = 18;
pub const XM_VECTOR: c_int = 19;
pub const VE_VECTOR: c_int = 20;
pub const CP_VECTOR: c_int = 21;
pub const HV_VECTOR: c_int = 28;
pub const VC_VECTOR: c_int = 29;
pub const SX_VECTOR: c_int = 30;
// Select x86 specific features in <linux/kvm.h>
// Architectural interrupt line count.
pub const KVM_NR_INTERRUPTS: c_int = 256;
// for KVM_GET_IRQCHIP and KVM_SET_IRQCHIP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pic_state {
    pub /: *mut *mut __u8 last_irr; / edge detection,
    pub /: *mut *mut __u8 irr; / interrupt request register,
    pub /: *mut *mut __u8 imr; / interrupt mask register,
    pub /: *mut *mut __u8 isr; / interrupt service register,
    pub /: *mut *mut __u8 priority_add; / highest irq priority,
    pub irq_base: __u8,
    pub read_reg_select: __u8,
    pub poll: __u8,
    pub special_mask: __u8,
    pub init_state: __u8,
    pub auto_eoi: __u8,
    pub rotate_on_auto_eoi: __u8,
    pub special_fully_nested_mode: __u8,
    pub /: *mut *mut __u8 init4; / true if 4 byte init,
    pub /: *mut *mut __u8 elcr; / PIIX edge/trigger selection,
    pub elcr_mask: __u8,
}

pub const KVM_IOAPIC_NUM_PINS: c_int = 24;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_ioapic_state {
    pub base_address: __u64,
    pub ioregsel: __u32,
    pub id: __u32,
    pub irr: __u32,
    pub pad: __u32,
    pub bits: __u64,
    pub vector: __u8,
    pub delivery_mode:3: __u8,
    pub dest_mode:1: __u8,
    pub delivery_status:1: __u8,
    pub polarity:1: __u8,
    pub remote_irr:1: __u8,
    pub trig_mode:1: __u8,
    pub mask:1: __u8,
    pub reserve:7: __u8,
    pub reserved: [__u8; 4],
    pub dest_id: __u8,
    pub fields: },
    pub redirtbl: [}; KVM_IOAPIC_NUM_PINS],
}

pub const KVM_IRQCHIP_PIC_MASTER: c_int = 0;
pub const KVM_IRQCHIP_PIC_SLAVE: c_int = 1;
pub const KVM_IRQCHIP_IOAPIC: c_int = 2;
pub const KVM_NR_IRQCHIPS: c_int = 3;

// for KVM_GET_REGS and KVM_SET_REGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_regs {
// out (KVM_GET_REGS) / in (KVM_SET_REGS)
    pub rdx: __u64 rax, rbx, rcx,,
    pub rbp: __u64 rsi, rdi, rsp,,
    pub r11: __u64 r8, r9, r10,,
    pub r15: __u64 r12, r13, r14,,
    pub rflags: __u64 rip,,
}

// for KVM_GET_LAPIC and KVM_SET_LAPIC
pub const KVM_APIC_REG_SIZE: c_uint = 0x400;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_lapic_state {
    pub regs: [c_char; KVM_APIC_REG_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_segment {
    pub base: __u64,
    pub limit: __u32,
    pub selector: __u16,
    pub type: __u8,
    pub avl: __u8 present, dpl, db, s, l, g,,
    pub unusable: __u8,
    pub padding: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_dtable {
    pub base: __u64,
    pub limit: __u16,
    pub padding: [__u16; 3],
}

// for KVM_GET_SREGS and KVM_SET_SREGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs {
// out (KVM_GET_SREGS) / in (KVM_SET_SREGS)
    pub ss: kvm_segment cs, ds, es, fs, gs,,
    pub ldt: kvm_segment tr,,
    pub idt: kvm_dtable gdt,,
    pub cr8: __u64 cr0, cr2, cr3, cr4,,
    pub efer: __u64,
    pub apic_base: __u64,
    pub 64]: __u64 interrupt_bitmap[(KVM_NR_INTERRUPTS + 63) /,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sregs2 {
// out (KVM_GET_SREGS2) / in (KVM_SET_SREGS2)
    pub ss: kvm_segment cs, ds, es, fs, gs,,
    pub ldt: kvm_segment tr,,
    pub idt: kvm_dtable gdt,,
    pub cr8: __u64 cr0, cr2, cr3, cr4,,
    pub efer: __u64,
    pub apic_base: __u64,
    pub flags: __u64,
    pub pdptrs: [__u64; 4],
}

pub const KVM_SREGS2_FLAGS_PDPTRS_VALID: c_int = 1;
// for KVM_GET_FPU and KVM_SET_FPU
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_fpu {
    pub fpr: [__u8; 8][16],
    pub fcw: __u16,
    pub fsw: __u16,
    pub /: *mut *mut __u8 ftwx; / in fxsave format,
    pub pad1: __u8,
    pub last_opcode: __u16,
    pub last_ip: __u64,
    pub last_dp: __u64,
    pub xmm: [__u8; 16][16],
    pub mxcsr: __u32,
    pub pad2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_msr_entry {
    pub index: __u32,
    pub reserved: __u32,
    pub data: __u64,
}

// for KVM_GET_MSRS and KVM_SET_MSRS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_msrs {
    pub /: *mut *mut __u32 nmsrs; / number of msrs in entries,
    pub pad: __u32,
    pub entries): __DECLARE_FLEX_ARRAY(struct kvm_msr_entry,,
}

// for KVM_GET_MSR_INDEX_LIST
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_msr_list {
    pub /: *mut *mut __u32 nmsrs; / number of msrs in entries,
    pub indices): __DECLARE_FLEX_ARRAY(__u32,,
}

// Maximum size of any access bitmap in bytes
pub const KVM_MSR_FILTER_MAX_BITMAP_SIZE: c_uint = 0x600;
// for KVM_X86_SET_MSR_FILTER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_msr_filter_range {

    pub flags: __u32,
    pub /: *mut *mut __u32 nmsrs; / number of msrs in bitmap,
    pub /: *mut *mut __u32 base; / MSR index the bitmap starts at,
    pub /: *mut *mut *mut __u8 bitmap; / a 1 bit allows the operations in flags, 0 denies,
}

pub const KVM_MSR_FILTER_MAX_RANGES: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_msr_filter {

    pub flags: __u32,
    pub ranges: [kvm_msr_filter_range; KVM_MSR_FILTER_MAX_RANGES],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_cpuid_entry {
    pub function: __u32,
    pub eax: __u32,
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub padding: __u32,
}

// for KVM_SET_CPUID
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_cpuid {
    pub nent: __u32,
    pub padding: __u32,
    pub entries): __DECLARE_FLEX_ARRAY(struct kvm_cpuid_entry,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_cpuid_entry2 {
    pub function: __u32,
    pub index: __u32,
    pub flags: __u32,
    pub eax: __u32,
    pub ebx: __u32,
    pub ecx: __u32,
    pub edx: __u32,
    pub padding: [__u32; 3],
}

// for KVM_SET_CPUID2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_cpuid2 {
    pub nent: __u32,
    pub padding: __u32,
    pub entries): __DECLARE_FLEX_ARRAY(struct kvm_cpuid_entry2,,
}

// for KVM_GET_PIT and KVM_SET_PIT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pit_channel_state {
    pub /: *mut *mut __u32 count; / can be 65536,
    pub latched_count: __u16,
    pub count_latched: __u8,
    pub status_latched: __u8,
    pub status: __u8,
    pub read_state: __u8,
    pub write_state: __u8,
    pub write_latch: __u8,
    pub rw_mode: __u8,
    pub mode: __u8,
    pub bcd: __u8,
    pub gate: __u8,
    pub count_load_time: __s64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debug_exit_arch {
    pub exception: __u32,
    pub pad: __u32,
    pub pc: __u64,
    pub dr6: __u64,
    pub dr7: __u64,
}

pub const KVM_GUESTDBG_USE_SW_BP: c_uint = 0x00010000;
pub const KVM_GUESTDBG_USE_HW_BP: c_uint = 0x00020000;
pub const KVM_GUESTDBG_INJECT_DB: c_uint = 0x00040000;
pub const KVM_GUESTDBG_INJECT_BP: c_uint = 0x00080000;
pub const KVM_GUESTDBG_BLOCKIRQ: c_uint = 0x00100000;
// for KVM_SET_GUEST_DEBUG
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_guest_debug_arch {
    pub debugreg: [__u64; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pit_state {
    pub channels: [kvm_pit_channel_state; 3],
}

pub const KVM_PIT_FLAGS_HPET_LEGACY: c_uint = 0x00000001;
pub const KVM_PIT_FLAGS_SPEAKER_DATA_ON: c_uint = 0x00000002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pit_state2 {
    pub channels: [kvm_pit_channel_state; 3],
    pub flags: __u32,
    pub reserved: [__u32; 9],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_reinject_control {
    pub pit_reinject: __u8,
    pub reserved: [__u8; 31],
}

// When set in flags, include corresponding fields on KVM_SET_VCPU_EVENTS
pub const KVM_VCPUEVENT_VALID_NMI_PENDING: c_uint = 0x00000001;
pub const KVM_VCPUEVENT_VALID_SIPI_VECTOR: c_uint = 0x00000002;
pub const KVM_VCPUEVENT_VALID_SHADOW: c_uint = 0x00000004;
pub const KVM_VCPUEVENT_VALID_SMM: c_uint = 0x00000008;
pub const KVM_VCPUEVENT_VALID_PAYLOAD: c_uint = 0x00000010;
pub const KVM_VCPUEVENT_VALID_TRIPLE_FAULT: c_uint = 0x00000020;
// Interrupt shadow states
pub const KVM_X86_SHADOW_INT_MOV_SS: c_uint = 0x01;
pub const KVM_X86_SHADOW_INT_STI: c_uint = 0x02;
// for KVM_GET/SET_VCPU_EVENTS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vcpu_events {
    pub injected: __u8,
    pub nr: __u8,
    pub has_error_code: __u8,
    pub pending: __u8,
    pub error_code: __u32,
    pub exception: },
    pub injected: __u8,
    pub nr: __u8,
    pub soft: __u8,
    pub shadow: __u8,
    pub interrupt: },
    pub injected: __u8,
    pub pending: __u8,
    pub masked: __u8,
    pub pad: __u8,
    pub nmi: },
    pub sipi_vector: __u32,
    pub flags: __u32,
    pub smm: __u8,
    pub pending: __u8,
    pub smm_inside_nmi: __u8,
    pub latched_init: __u8,
    pub smi: },
    pub pending: __u8,
    pub triple_fault: },
    pub reserved: [__u8; 26],
    pub exception_has_payload: __u8,
    pub exception_payload: __u64,
}

// for KVM_GET/SET_DEBUGREGS
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_debugregs {
    pub db: [__u64; 4],
    pub dr6: __u64,
    pub dr7: __u64,
    pub flags: __u64,
    pub reserved: [__u64; 9],
}

// for KVM_CAP_XSAVE and KVM_CAP_XSAVE2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xsave {
//
// KVM_GET_XSAVE2 and KVM_SET_XSAVE write and read as many bytes
// as are returned by KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
// respectively, when invoked on the vm file descriptor.
//
// The size value returned by KVM_CHECK_EXTENSION(KVM_CAP_XSAVE2)
// will always be at least 4096. Currently, it is only greater
// than 4096 if a dynamic feature has been enabled with
// ``arch_prctl()``, but this may change in the future.
//
// The offsets of the state save areas in struct kvm_xsave follow
// the contents of CPUID leaf 0xD on the host.
//
    pub region: [__u32; 1024],
    pub extra): __DECLARE_FLEX_ARRAY(__u32,,
}

pub const KVM_MAX_XCRS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xcr {
    pub xcr: __u32,
    pub reserved: __u32,
    pub value: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xcrs {
    pub nr_xcrs: __u32,
    pub flags: __u32,
    pub xcrs: [kvm_xcr; KVM_MAX_XCRS],
    pub padding: [__u64; 16],
}

pub const KVM_X86_REG_TYPE_MSR: c_int = 2;
pub const KVM_X86_REG_TYPE_KVM: c_int = 3;

// KVM-defined registers starting from 0
pub const KVM_REG_GUEST_SSP: c_int = 0;

// kvm_sync_regs struct included by kvm_run struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sync_regs {
// Members of this structure are potentially malicious.
// Care must be taken by code reading, esp. interpreting,
// data fields from them inside KVM to prevent TOCTOU and
// double-fetch types of vulnerabilities.
//
    pub regs: kvm_regs,
    pub sregs: kvm_sregs,
    pub events: kvm_vcpu_events,
}

pub const KVM_STATE_NESTED_FORMAT_VMX: c_int = 0;
pub const KVM_STATE_NESTED_FORMAT_SVM: c_int = 1;
pub const KVM_STATE_NESTED_GUEST_MODE: c_uint = 0x00000001;
pub const KVM_STATE_NESTED_RUN_PENDING: c_uint = 0x00000002;
pub const KVM_STATE_NESTED_EVMCS: c_uint = 0x00000004;
pub const KVM_STATE_NESTED_MTF_PENDING: c_uint = 0x00000008;
pub const KVM_STATE_NESTED_GIF_SET: c_uint = 0x00000100;
pub const KVM_STATE_NESTED_SMM_GUEST_MODE: c_uint = 0x00000001;
pub const KVM_STATE_NESTED_SMM_VMXON: c_uint = 0x00000002;
pub const KVM_STATE_NESTED_VMX_VMCS_SIZE: c_uint = 0x1000;
pub const KVM_STATE_NESTED_SVM_VMCB_SIZE: c_uint = 0x1000;
pub const KVM_STATE_VMX_PREEMPTION_TIMER_DEADLINE: c_uint = 0x00000001;
// vendor-independent attributes for system fd (group 0)
pub const KVM_X86_GRP_SYSTEM: c_int = 0;

// vendor-specific groups and attributes for system fd
pub const KVM_X86_GRP_SEV: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vmx_nested_state_data {
    pub vmcs12: [__u8; KVM_STATE_NESTED_VMX_VMCS_SIZE],
    pub shadow_vmcs12: [__u8; KVM_STATE_NESTED_VMX_VMCS_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_vmx_nested_state_hdr {
    pub vmxon_pa: __u64,
    pub vmcs12_pa: __u64,
    pub flags: __u16,
    pub smm: },
    pub pad: __u16,
    pub flags: __u32,
    pub preemption_timer_deadline: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_svm_nested_state_data {
// Save area only used if KVM_STATE_NESTED_RUN_PENDING.
    pub vmcb12: [__u8; KVM_STATE_NESTED_SVM_VMCB_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_svm_nested_state_hdr {
    pub vmcb_pa: __u64,
    pub gpat: __u64,
}

// for KVM_CAP_NESTED_STATE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_nested_state {
    pub flags: __u16,
    pub format: __u16,
    pub size: __u32,
    pub vmx: kvm_vmx_nested_state_hdr,
    pub svm: kvm_svm_nested_state_hdr,
// Pad the header to 128 bytes.
    pub pad: [__u8; 120],
    pub hdr: },
//
// Define data region as 0 bytes to preserve backwards-compatability
// to old definition of kvm_nested_state in order to avoid changing
// KVM_{GET,PUT}_NESTED_STATE ioctl values.
//
    pub vmx): __DECLARE_FLEX_ARRAY(struct kvm_vmx_nested_state_data,,
    pub svm): __DECLARE_FLEX_ARRAY(struct kvm_svm_nested_state_data,,
    pub data: },
}

// for KVM_CAP_PMU_EVENT_FILTER
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_pmu_event_filter {
    pub action: __u32,
    pub nevents: __u32,
    pub fixed_counter_bitmap: __u32,
    pub flags: __u32,
    pub pad: [__u32; 4],
    pub events): __DECLARE_FLEX_ARRAY(__u64,,
}

pub const KVM_PMU_EVENT_ALLOW: c_int = 0;
pub const KVM_PMU_EVENT_DENY: c_int = 1;

// for KVM_CAP_MCE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_x86_mce {
    pub status: __u64,
    pub addr: __u64,
    pub misc: __u64,
    pub mcg_status: __u64,
    pub bank: __u8,
    pub pad1: [__u8; 7],
    pub pad2: [__u64; 3],
}

// for KVM_CAP_XEN_HVM

pub const KVM_XEN_MSR_MIN_INDEX: c_uint = 0x40000000u;
pub const KVM_XEN_MSR_MAX_INDEX: c_uint = 0x4fffffffu;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xen_hvm_config {
    pub flags: __u32,
    pub msr: __u32,
    pub blob_addr_32: __u64,
    pub blob_addr_64: __u64,
    pub blob_size_32: __u8,
    pub blob_size_64: __u8,
    pub pad2: [__u8; 30],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xen_hvm_attr {
    pub type: __u16,
    pub pad: [__u16; 3],
    pub long_mode: __u8,
    pub vector: __u8,
    pub runstate_update_flag: __u8,
    pub gfn: __u64,

    pub hva: __u64,
    pub shared_info: },
    pub send_port: __u32,
    pub /: *mut *mut __u32 type; / EVTCHNSTAT_ipi / EVTCHNSTAT_interdomain,
    pub flags: __u32,

//
// Events sent by the guest are either looped back to
// the guest itself (potentially on a different port#)
// or signalled via an eventfd.
//
    pub port: __u32,
    pub vcpu: __u32,
    pub priority: __u32,
    pub port: },
    pub /: *mut *mut __u32 port; / Zero for eventfd,
    pub fd: __s32,
    pub eventfd: },
    pub padding: [__u32; 4],
    pub deliver: },
    pub evtchn: },
    pub xen_version: __u32,
    pub pad: [__u64; 8],
    pub u: },
}

// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
pub const KVM_XEN_ATTR_TYPE_LONG_MODE: c_uint = 0x0;
pub const KVM_XEN_ATTR_TYPE_SHARED_INFO: c_uint = 0x1;
pub const KVM_XEN_ATTR_TYPE_UPCALL_VECTOR: c_uint = 0x2;
// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND
pub const KVM_XEN_ATTR_TYPE_EVTCHN: c_uint = 0x3;
pub const KVM_XEN_ATTR_TYPE_XEN_VERSION: c_uint = 0x4;
// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_RUNSTATE_UPDATE_FLAG
pub const KVM_XEN_ATTR_TYPE_RUNSTATE_UPDATE_FLAG: c_uint = 0x5;
// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA
pub const KVM_XEN_ATTR_TYPE_SHARED_INFO_HVA: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_xen_vcpu_attr {
    pub type: __u16,
    pub pad: [__u16; 3],
    pub gpa: __u64,

    pub hva: __u64,
    pub pad: [__u64; 8],
    pub state: __u64,
    pub state_entry_time: __u64,
    pub time_running: __u64,
    pub time_runnable: __u64,
    pub time_blocked: __u64,
    pub time_offline: __u64,
    pub runstate: },
    pub vcpu_id: __u32,
    pub port: __u32,
    pub priority: __u32,
    pub expires_ns: __u64,
    pub timer: },
    pub vector: __u8,
    pub u: },
}

// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO: c_uint = 0x0;
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_TIME_INFO: c_uint = 0x1;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADDR: c_uint = 0x2;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_CURRENT: c_uint = 0x3;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_DATA: c_uint = 0x4;
pub const KVM_XEN_VCPU_ATTR_TYPE_RUNSTATE_ADJUST: c_uint = 0x5;
// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_EVTCHN_SEND
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_ID: c_uint = 0x6;
pub const KVM_XEN_VCPU_ATTR_TYPE_TIMER: c_uint = 0x7;
pub const KVM_XEN_VCPU_ATTR_TYPE_UPCALL_VECTOR: c_uint = 0x8;
// Available with KVM_CAP_XEN_HVM / KVM_XEN_HVM_CONFIG_SHARED_INFO_HVA
pub const KVM_XEN_VCPU_ATTR_TYPE_VCPU_INFO_HVA: c_uint = 0x9;
// Secure Encrypted Virtualization command
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sev_cmd_id {
// Guest initialization commands
    KVM_SEV_INIT = 0,
    KVM_SEV_ES_INIT,
// Guest launch commands
    KVM_SEV_LAUNCH_START,
    KVM_SEV_LAUNCH_UPDATE_DATA,
    KVM_SEV_LAUNCH_UPDATE_VMSA,
    KVM_SEV_LAUNCH_SECRET,
    KVM_SEV_LAUNCH_MEASURE,
    KVM_SEV_LAUNCH_FINISH,
// Guest migration commands (outgoing)
    KVM_SEV_SEND_START,
    KVM_SEV_SEND_UPDATE_DATA,
    KVM_SEV_SEND_UPDATE_VMSA,
    KVM_SEV_SEND_FINISH,
// Guest migration commands (incoming)
    KVM_SEV_RECEIVE_START,
    KVM_SEV_RECEIVE_UPDATE_DATA,
    KVM_SEV_RECEIVE_UPDATE_VMSA,
    KVM_SEV_RECEIVE_FINISH,
// Guest status and debug commands
    KVM_SEV_GUEST_STATUS,
    KVM_SEV_DBG_DECRYPT,
    KVM_SEV_DBG_ENCRYPT,
// Guest certificates commands
    KVM_SEV_CERT_EXPORT,
// Attestation report
    KVM_SEV_GET_ATTESTATION_REPORT,
// Guest Migration Extension
    KVM_SEV_SEND_CANCEL,

// Second time is the charm; improved versions of the above ioctls.
    KVM_SEV_INIT2,

// SNP-specific commands
    KVM_SEV_SNP_LAUNCH_START = 100,
    KVM_SEV_SNP_LAUNCH_UPDATE,
    KVM_SEV_SNP_LAUNCH_FINISH,
    KVM_SEV_SNP_ENABLE_REQ_CERTS,

    KVM_SEV_NR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_cmd {
    pub id: __u32,
    pub pad0: __u32,
    pub data: __u64,
    pub error: __u32,
    pub sev_fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_init {
    pub vmsa_features: __u64,
    pub flags: __u32,
    pub ghcb_version: __u16,
    pub pad1: __u16,
    pub pad2: [__u32; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_launch_start {
    pub handle: __u32,
    pub policy: __u32,
    pub dh_uaddr: __u64,
    pub dh_len: __u32,
    pub pad0: __u32,
    pub session_uaddr: __u64,
    pub session_len: __u32,
    pub pad1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_launch_update_data {
    pub uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_launch_secret {
    pub hdr_uaddr: __u64,
    pub hdr_len: __u32,
    pub pad0: __u32,
    pub guest_uaddr: __u64,
    pub guest_len: __u32,
    pub pad1: __u32,
    pub trans_uaddr: __u64,
    pub trans_len: __u32,
    pub pad2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_launch_measure {
    pub uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_guest_status {
    pub handle: __u32,
    pub policy: __u32,
    pub state: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_dbg {
    pub src_uaddr: __u64,
    pub dst_uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_attestation_report {
    pub mnonce: [__u8; 16],
    pub uaddr: __u64,
    pub len: __u32,
    pub pad0: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_send_start {
    pub policy: __u32,
    pub pad0: __u32,
    pub pdh_cert_uaddr: __u64,
    pub pdh_cert_len: __u32,
    pub pad1: __u32,
    pub plat_certs_uaddr: __u64,
    pub plat_certs_len: __u32,
    pub pad2: __u32,
    pub amd_certs_uaddr: __u64,
    pub amd_certs_len: __u32,
    pub pad3: __u32,
    pub session_uaddr: __u64,
    pub session_len: __u32,
    pub pad4: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_send_update_data {
    pub hdr_uaddr: __u64,
    pub hdr_len: __u32,
    pub pad0: __u32,
    pub guest_uaddr: __u64,
    pub guest_len: __u32,
    pub pad1: __u32,
    pub trans_uaddr: __u64,
    pub trans_len: __u32,
    pub pad2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_receive_start {
    pub handle: __u32,
    pub policy: __u32,
    pub pdh_uaddr: __u64,
    pub pdh_len: __u32,
    pub pad0: __u32,
    pub session_uaddr: __u64,
    pub session_len: __u32,
    pub pad1: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_receive_update_data {
    pub hdr_uaddr: __u64,
    pub hdr_len: __u32,
    pub pad0: __u32,
    pub guest_uaddr: __u64,
    pub guest_len: __u32,
    pub pad1: __u32,
    pub trans_uaddr: __u64,
    pub trans_len: __u32,
    pub pad2: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_snp_launch_start {
    pub policy: __u64,
    pub gosvw: [__u8; 16],
    pub flags: __u16,
    pub pad0: [__u8; 6],
    pub pad1: [__u64; 4],
}

// Kept in sync with firmware values for simplicity.
pub const KVM_SEV_PAGE_TYPE_INVALID: c_uint = 0x0;
pub const KVM_SEV_SNP_PAGE_TYPE_NORMAL: c_uint = 0x1;
pub const KVM_SEV_SNP_PAGE_TYPE_ZERO: c_uint = 0x3;
pub const KVM_SEV_SNP_PAGE_TYPE_UNMEASURED: c_uint = 0x4;
pub const KVM_SEV_SNP_PAGE_TYPE_SECRETS: c_uint = 0x5;
pub const KVM_SEV_SNP_PAGE_TYPE_CPUID: c_uint = 0x6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_snp_launch_update {
    pub gfn_start: __u64,
    pub uaddr: __u64,
    pub len: __u64,
    pub type: __u8,
    pub pad0: __u8,
    pub flags: __u16,
    pub pad1: __u32,
    pub pad2: [__u64; 4],
}

pub const KVM_SEV_SNP_ID_BLOCK_SIZE: c_int = 96;
pub const KVM_SEV_SNP_ID_AUTH_SIZE: c_int = 4096;
pub const KVM_SEV_SNP_FINISH_DATA_SIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_sev_snp_launch_finish {
    pub id_block_uaddr: __u64,
    pub id_auth_uaddr: __u64,
    pub id_block_en: __u8,
    pub auth_key_en: __u8,
    pub vcek_disabled: __u8,
    pub host_data: [__u8; KVM_SEV_SNP_FINISH_DATA_SIZE],
    pub pad0: [__u8; 3],
    pub flags: __u16,
    pub pad1: [__u64; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_hyperv_eventfd {
    pub conn_id: __u32,
    pub fd: __s32,
    pub flags: __u32,
    pub padding: [__u32; 3],
}

pub const KVM_HYPERV_CONN_ID_MASK: c_uint = 0x00ffffff;

//
// Masked event layout.
// Bits   Description
// ----   -----------
// 7:0    event select (low bits)
// 15:8   umask match
// 31:16  unused
// 35:32  event select (high bits)
// 36:54  unused
// 55     exclude bit
// 63:56  umask mask
//

// for KVM_{GET,SET,HAS}_DEVICE_ATTR

// x86-specific KVM_EXIT_HYPERCALL flags.

pub const KVM_X86_DEFAULT_VM: c_int = 0;
pub const KVM_X86_SW_PROTECTED_VM: c_int = 1;
pub const KVM_X86_SEV_VM: c_int = 2;
pub const KVM_X86_SEV_ES_VM: c_int = 3;
pub const KVM_X86_SNP_VM: c_int = 4;
pub const KVM_X86_TDX_VM: c_int = 5;
// Trust Domain eXtension sub-ioctl() commands.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kvm_tdx_cmd_id {
    KVM_TDX_CAPABILITIES = 0,
    KVM_TDX_INIT_VM,
    KVM_TDX_INIT_VCPU,
    KVM_TDX_INIT_MEM_REGION,
    KVM_TDX_FINALIZE_VM,
    KVM_TDX_GET_CPUID,

    KVM_TDX_CMD_NR_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_tdx_cmd {
// enum kvm_tdx_cmd_id
    pub id: __u32,
// flags for sub-commend. If sub-command doesn't use this, set zero.
    pub flags: __u32,
//
// data for each sub-command. An immediate or a pointer to the actual
// data in process virtual address.  If sub-command doesn't use it,
// set zero.
//
    pub data: __u64,
//
// Auxiliary error code.  The sub-command may return TDX SEAMCALL
// status code in addition to -Exxx.
//
    pub hw_error: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_tdx_capabilities {
    pub supported_attrs: __u64,
    pub supported_xfam: __u64,
    pub kernel_tdvmcallinfo_1_r11: __u64,
    pub user_tdvmcallinfo_1_r11: __u64,
    pub kernel_tdvmcallinfo_1_r12: __u64,
    pub user_tdvmcallinfo_1_r12: __u64,
    pub reserved: [__u64; 250],
// Configurable CPUID bits for userspace
    pub cpuid: kvm_cpuid2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_tdx_init_vm {
    pub attributes: __u64,
    pub xfam: __u64,
    pub /: *mut *mut __u64 mrconfigid[6]; / sha384 digest,
    pub /: *mut *mut __u64 mrowner[6]; / sha384 digest,
    pub /: *mut *mut __u64 mrownerconfig[6]; / sha384 digest,
// The total space for TD_PARAMS before the CPUIDs is 256 bytes
    pub reserved: [__u64; 12],
//
// Call KVM_TDX_INIT_VM before vcpu creation, thus before
// KVM_SET_CPUID2.
// This configuration supersedes KVM_SET_CPUID2s for VCPUs because the
// TDX module directly virtualizes those CPUIDs without VMM.  The user
// space VMM, e.g. qemu, should make KVM_SET_CPUID2 consistent with
// those values.  If it doesn't, KVM may have wrong idea of vCPUIDs of
// the guest, and KVM may wrongly emulate CPUIDs or MSRs that the TDX
// module doesn't virtualize.
//
    pub cpuid: kvm_cpuid2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kvm_tdx_init_mem_region {
    pub source_addr: __u64,
    pub gpa: __u64,
    pub nr_pages: __u64,
}
