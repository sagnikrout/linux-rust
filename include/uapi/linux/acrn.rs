//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/acrn.h
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
// Userspace interface for /dev/acrn_hsm - ACRN Hypervisor Service Module
//
// This file can be used by applications that need to communicate with the HSM
// via the ioctl interface.
//
// Copyright (C) 2021 Intel Corporation. All rights reserved.
//

pub const ACRN_IO_REQUEST_MAX: c_int = 16;
pub const ACRN_IOREQ_STATE_PENDING: c_int = 0;
pub const ACRN_IOREQ_STATE_COMPLETE: c_int = 1;
pub const ACRN_IOREQ_STATE_PROCESSING: c_int = 2;
pub const ACRN_IOREQ_STATE_FREE: c_int = 3;
pub const ACRN_IOREQ_TYPE_PORTIO: c_int = 0;
pub const ACRN_IOREQ_TYPE_MMIO: c_int = 1;
pub const ACRN_IOREQ_TYPE_PCICFG: c_int = 2;
pub const ACRN_IOREQ_DIR_READ: c_int = 0;
pub const ACRN_IOREQ_DIR_WRITE: c_int = 1;
//
// struct acrn_mmio_request - Info of a MMIO I/O request
// @direction:	Access direction of this request (ACRN_IOREQ_DIR_*)
// @reserved:	Reserved for alignment and should be 0
// @address:	Access address of this MMIO I/O request
// @size:	Access size of this MMIO I/O request
// @value:	Read/write value of this MMIO I/O request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_mmio_request {
    pub direction: __u32,
    pub reserved: __u32,
    pub address: __u64,
    pub size: __u64,
    pub value: __u64,
}

//
// struct acrn_pio_request - Info of a PIO I/O request
// @direction:	Access direction of this request (ACRN_IOREQ_DIR_*)
// @reserved:	Reserved for alignment and should be 0
// @address:	Access address of this PIO I/O request
// @size:	Access size of this PIO I/O request
// @value:	Read/write value of this PIO I/O request
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_pio_request {
    pub direction: __u32,
    pub reserved: __u32,
    pub address: __u64,
    pub size: __u64,
    pub value: __u32,
}

//
// struct acrn_pci_request - Info of a PCI I/O request
// @direction:	Access direction of this request (ACRN_IOREQ_DIR_*)
// @reserved:	Reserved for alignment and should be 0
// @size:	Access size of this PCI I/O request
// @value:	Read/write value of this PIO I/O request
// @bus:	PCI bus value of this PCI I/O request
// @dev:	PCI device value of this PCI I/O request
// @func:	PCI function value of this PCI I/O request
// @reg:	PCI config space offset of this PCI I/O request
//
// Need keep same header layout with &struct acrn_pio_request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_pci_request {
    pub direction: __u32,
    pub reserved: [__u32; 3],
    pub size: __u64,
    pub value: __u32,
    pub bus: __u32,
    pub dev: __u32,
    pub func: __u32,
    pub reg: __u32,
}

//
// struct acrn_io_request - 256-byte ACRN I/O request
// @type:		Type of this request (ACRN_IOREQ_TYPE_*).
// @completion_polling:	Polling flag. Hypervisor will poll completion of the
// I/O request if this flag set.
// @reserved0:		Reserved fields.
// @reqs:		Union of different types of request. Byte offset: 64.
// @reqs.pio_request:	PIO request data of the I/O request.
// @reqs.pci_request:	PCI configuration space request data of the I/O request.
// @reqs.mmio_request:	MMIO request data of the I/O request.
// @reqs.data:		Raw data of the I/O request.
// @reserved1:		Reserved fields.
// @kernel_handled:	Flag indicates this request need be handled in kernel.
// @processed:		The status of this request (ACRN_IOREQ_STATE_*).
//
// The state transitions of ACRN I/O request:
//
// FREE -> PENDING -> PROCESSING -> COMPLETE -> FREE -> ...
//
// An I/O request in COMPLETE or FREE state is owned by the hypervisor. HSM and
// ACRN userspace are in charge of processing the others.
//
// On basis of the states illustrated above, a typical lifecycle of ACRN IO
// request would look like:
//
// Flow                 (assume the initial state is FREE)
// |
// |   Service VM vCPU 0     Service VM vCPU x      User vCPU y
// |
// |                                             hypervisor:
// |                                               fills in type, addr, etc.
// |                                               pauses the User VM vCPU y
// |                                               sets the state to PENDING (a)
// |                                               fires an upcall to Service VM
// |
// | HSM:
// |  scans for PENDING requests
// |  sets the states to PROCESSING (b)
// |  assigns the requests to clients (c)
// V
// |                     client:
// |                       scans for the assigned requests
// |                       handles the requests (d)
// |                     HSM:
// |                       sets states to COMPLETE
// |                       notifies the hypervisor
// |
// |                     hypervisor:
// |                       resumes User VM vCPU y (e)
// |
// |                                             hypervisor:
// |                                               post handling (f)
// V                                               sets states to FREE
//
// Note that the procedures (a) to (f) in the illustration above require to be
// strictly processed in the order.  One vCPU cannot trigger another request of
// I/O emulation before completing the previous one.
//
// Atomic and barriers are required when HSM and hypervisor accessing the state
// of &struct acrn_io_request.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_io_request {
    pub type: __u32,
    pub completion_polling: __u32,
    pub reserved0: [__u32; 14],
    pub pio_request: acrn_pio_request,
    pub pci_request: acrn_pci_request,
    pub mmio_request: acrn_mmio_request,
    pub data: [__u64; 8],
    pub reqs: },
    pub reserved1: __u32,
    pub kernel_handled: __u32,
    pub processed: __u32,
    pub __attribute__((aligned(256))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_io_request_buffer {
    pub req_slot: [acrn_io_request; ACRN_IO_REQUEST_MAX],
    pub reserved: [__u8; 4096],
}

//
// struct acrn_ioreq_notify - The structure of ioreq completion notification
// @vmid:	User VM ID
// @reserved:	Reserved and should be 0
// @vcpu:	vCPU ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_ioreq_notify {
    pub vmid: __u16,
    pub reserved: __u16,
    pub vcpu: __u32,
}

//
// struct acrn_vm_creation - Info to create a User VM
// @vmid:		User VM ID returned from the hypervisor
// @reserved0:		Reserved and must be 0
// @vcpu_num:		Number of vCPU in the VM. Return from hypervisor.
// @reserved1:		Reserved and must be 0
// @uuid:		Empty space never to be used again (used to be UUID of the VM)
// @vm_flag:		Flag of the VM creating. Pass to hypervisor directly.
// @ioreq_buf:		Service VM GPA of I/O request buffer. Pass to
// hypervisor directly.
// @cpu_affinity:	CPU affinity of the VM. Pass to hypervisor directly.
// It's a bitmap which indicates CPUs used by the VM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_vm_creation {
    pub vmid: __u16,
    pub reserved0: __u16,
    pub vcpu_num: __u16,
    pub reserved1: __u16,
    pub uuid: [__u8; 16],
    pub vm_flag: __u64,
    pub ioreq_buf: __u64,
    pub cpu_affinity: __u64,
}

//
// struct acrn_gp_regs - General registers of a User VM
// @rax:	Value of register RAX
// @rcx:	Value of register RCX
// @rdx:	Value of register RDX
// @rbx:	Value of register RBX
// @rsp:	Value of register RSP
// @rbp:	Value of register RBP
// @rsi:	Value of register RSI
// @rdi:	Value of register RDI
// @r8:		Value of register R8
// @r9:		Value of register R9
// @r10:	Value of register R10
// @r11:	Value of register R11
// @r12:	Value of register R12
// @r13:	Value of register R13
// @r14:	Value of register R14
// @r15:	Value of register R15
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_gp_regs {
    pub rax: __le64,
    pub rcx: __le64,
    pub rdx: __le64,
    pub rbx: __le64,
    pub rsp: __le64,
    pub rbp: __le64,
    pub rsi: __le64,
    pub rdi: __le64,
    pub r8: __le64,
    pub r9: __le64,
    pub r10: __le64,
    pub r11: __le64,
    pub r12: __le64,
    pub r13: __le64,
    pub r14: __le64,
    pub r15: __le64,
}

//
// struct acrn_descriptor_ptr - Segment descriptor table of a User VM.
// @limit:	Limit field.
// @base:	Base field.
// @reserved:	Reserved and must be 0.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_descriptor_ptr {
    pub limit: __le16,
    pub base: __le64,
    pub reserved: [__le16; 3],
// C attribute field omitted
//
// struct acrn_regs - Registers structure of a User VM
// @gprs:		General registers
// @gdt:		Global Descriptor Table
// @idt:		Interrupt Descriptor Table
// @rip:		Value of register RIP
// @cs_base:		Base of code segment selector
// @cr0:		Value of register CR0
// @cr4:		Value of register CR4
// @cr3:		Value of register CR3
// @ia32_efer:		Value of IA32_EFER MSR
// @rflags:		Value of regsiter RFLAGS
// @reserved_64:	Reserved and must be 0
// @cs_ar:		Attribute field of code segment selector
// @cs_limit:		Limit field of code segment selector
// @reserved_32:	Reserved and must be 0
// @cs_sel:		Value of code segment selector
// @ss_sel:		Value of stack segment selector
// @ds_sel:		Value of data segment selector
// @es_sel:		Value of extra segment selector
// @fs_sel:		Value of FS selector
// @gs_sel:		Value of GS selector
// @ldt_sel:		Value of LDT descriptor selector
// @tr_sel:		Value of TSS descriptor selector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_regs {
    pub gprs: acrn_gp_regs,
    pub gdt: acrn_descriptor_ptr,
    pub idt: acrn_descriptor_ptr,
    pub rip: __le64,
    pub cs_base: __le64,
    pub cr0: __le64,
    pub cr4: __le64,
    pub cr3: __le64,
    pub ia32_efer: __le64,
    pub rflags: __le64,
    pub reserved_64: [__le64; 4],
    pub cs_ar: __le32,
    pub cs_limit: __le32,
    pub reserved_32: [__le32; 3],
    pub cs_sel: __le16,
    pub ss_sel: __le16,
    pub ds_sel: __le16,
    pub es_sel: __le16,
    pub fs_sel: __le16,
    pub gs_sel: __le16,
    pub ldt_sel: __le16,
    pub tr_sel: __le16,
}

//
// struct acrn_vcpu_regs - Info of vCPU registers state
// @vcpu_id:	vCPU ID
// @reserved:	Reserved and must be 0
// @vcpu_regs:	vCPU registers state
//
// This structure will be passed to hypervisor directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_vcpu_regs {
    pub vcpu_id: __u16,
    pub reserved: [__u16; 3],
    pub vcpu_regs: acrn_regs,
}

pub const ACRN_MEM_ACCESS_RIGHT_MASK: c_uint = 0x00000007U;
pub const ACRN_MEM_ACCESS_READ: c_uint = 0x00000001U;
pub const ACRN_MEM_ACCESS_WRITE: c_uint = 0x00000002U;
pub const ACRN_MEM_ACCESS_EXEC: c_uint = 0x00000004U;

pub const ACRN_MEM_TYPE_MASK: c_uint = 0x000007C0U;
pub const ACRN_MEM_TYPE_WB: c_uint = 0x00000040U;
pub const ACRN_MEM_TYPE_WT: c_uint = 0x00000080U;
pub const ACRN_MEM_TYPE_UC: c_uint = 0x00000100U;
pub const ACRN_MEM_TYPE_WC: c_uint = 0x00000200U;
pub const ACRN_MEM_TYPE_WP: c_uint = 0x00000400U;
// Memory mapping types
pub const ACRN_MEMMAP_RAM: c_int = 0;
pub const ACRN_MEMMAP_MMIO: c_int = 1;
//
// struct acrn_vm_memmap - A EPT memory mapping info for a User VM.
// @type:		Type of the memory mapping (ACRM_MEMMAP_*).
// Pass to hypervisor directly.
// @attr:		Attribute of the memory mapping.
// Pass to hypervisor directly.
// @user_vm_pa:		Physical address of User VM.
// Pass to hypervisor directly.
// @service_vm_pa:	Physical address of Service VM.
// Pass to hypervisor directly.
// @vma_base:		VMA address of Service VM. Pass to hypervisor directly.
// @len:		Length of the memory mapping.
// Pass to hypervisor directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_vm_memmap {
    pub type: __u32,
    pub attr: __u32,
    pub user_vm_pa: __u64,
    pub service_vm_pa: __u64,
    pub vma_base: __u64,
}

// Type of interrupt of a passthrough device
pub const ACRN_PTDEV_IRQ_INTX: c_int = 0;
pub const ACRN_PTDEV_IRQ_MSI: c_int = 1;
pub const ACRN_PTDEV_IRQ_MSIX: c_int = 2;
//
// struct acrn_ptdev_irq - Interrupt data of a passthrough device.
// @type:		Type (ACRN_PTDEV_IRQ_*)
// @virt_bdf:		Virtual Bus/Device/Function
// @phys_bdf:		Physical Bus/Device/Function
// @intx:		Info of interrupt
// @intx.virt_pin:	Virtual IOAPIC pin
// @intx.phys_pin:	Physical IOAPIC pin
// @intx.is_pic_pin:	Is PIC pin or not
//
// This structure will be passed to hypervisor directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_ptdev_irq {
    pub type: __u32,
    pub virt_bdf: __u16,
    pub phys_bdf: __u16,
    pub virt_pin: __u32,
    pub phys_pin: __u32,
    pub is_pic_pin: __u32,
    pub intx: },
}

// Type of PCI device assignment

pub const ACRN_MMIODEV_RES_NUM: c_int = 3;
pub const ACRN_PCI_NUM_BARS: c_int = 6;
//
// struct acrn_pcidev - Info for assigning or de-assigning a PCI device
// @type:	Type of the assignment
// @virt_bdf:	Virtual Bus/Device/Function
// @phys_bdf:	Physical Bus/Device/Function
// @intr_line:	PCI interrupt line
// @intr_pin:	PCI interrupt pin
// @bar:	PCI BARs.
//
// This structure will be passed to hypervisor directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_pcidev {
    pub type: __u32,
    pub virt_bdf: __u16,
    pub phys_bdf: __u16,
    pub intr_line: __u8,
    pub intr_pin: __u8,
    pub bar: [__u32; ACRN_PCI_NUM_BARS],
}

//
// struct acrn_mmio_dev_res - MMIO device resource description
// @user_vm_pa:		Physical address of User VM of the MMIO region
// for the MMIO device.
// @service_vm_pa:	Physical address of Service VM of the MMIO
// region for the MMIO device.
// @size:		Size of the MMIO region for the MMIO device.
// @mem_type:		Memory type of the MMIO region for the MMIO
// device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_mmio_dev_res {
    pub user_vm_pa: __u64,
    pub service_vm_pa: __u64,
    pub size: __u64,
    pub mem_type: __u64,
}

//
// struct acrn_mmiodev - Info for assigning or de-assigning an MMIO device
// @name:	Name of the MMIO device.
// @res:	Array of MMIO device descriptions
//
// This structure will be passed to hypervisor directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_mmiodev {
    pub name: [__u8; 8],
    pub res: [acrn_mmio_dev_res; ACRN_MMIODEV_RES_NUM],
}

//
// struct acrn_vdev - Info for creating or destroying a virtual device
// @id:				Union of identifier of the virtual device
// @id.value:			Raw data of the identifier
// @id.fields.vendor:		Vendor id of the virtual PCI device
// @id.fields.device:		Device id of the virtual PCI device
// @id.fields.legacy_id:	ID of the virtual device if not a PCI device
// @slot:			Virtual Bus/Device/Function of the virtual
// device
// @io_base:			IO resource base address of the virtual device
// @io_size:			IO resource size of the virtual device
// @args:			Arguments for the virtual device creation
//
// The created virtual device can be a PCI device or a legacy device (e.g.
// a virtual UART controller) and it is emulated by the hypervisor. This
// structure will be passed to hypervisor directly.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_vdev {
//
// the identifier of the device, the low 32 bits represent the vendor
// id and device id of PCI device and the high 32 bits represent the
// device number of the legacy device
//
    pub value: __u64,
    pub vendor: __le16,
    pub device: __le16,
    pub legacy_id: __le32,
    pub fields: },
    pub id: },
    pub slot: __u64,
    pub io_addr: [__u32; ACRN_PCI_NUM_BARS],
    pub io_size: [__u32; ACRN_PCI_NUM_BARS],
    pub args: [__u8; 128],
}

//
// struct acrn_msi_entry - Info for injecting a MSI interrupt to a VM
// @msi_addr:	MSI addr[19:12] with dest vCPU ID
// @msi_data:	MSI data[7:0] with vector
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_msi_entry {
    pub msi_addr: __u64,
    pub msi_data: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_acpi_generic_address {
    pub space_id: __u8,
    pub bit_width: __u8,
    pub bit_offset: __u8,
    pub access_size: __u8,
    pub address: __u64,
// C attribute field omitted
//
// struct acrn_cstate_data - A C state package defined in ACPI
// @cx_reg:	Register of the C state object
// @type:	Type of the C state object
// @latency:	The worst-case latency to enter and exit this C state
// @power:	The average power consumption when in this C state
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_cstate_data {
    pub cx_reg: acrn_acpi_generic_address,
    pub type: __u8,
    pub latency: __u32,
    pub power: __u64,
}

//
// struct acrn_pstate_data - A P state package defined in ACPI
// @core_frequency:	CPU frequency (in MHz).
// @power:		Power dissipation (in milliwatts).
// @transition_latency:	The worst-case latency in microseconds that CPU is
// unavailable during a transition from any P state to
// this P state.
// @bus_master_latency:	The worst-case latency in microseconds that Bus Masters
// are prevented from accessing memory during a transition
// from any P state to this P state.
// @control:		The value to be written to Performance Control Register
// @status:		Transition status.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_pstate_data {
    pub core_frequency: __u64,
    pub power: __u64,
    pub transition_latency: __u64,
    pub bus_master_latency: __u64,
    pub control: __u64,
    pub status: __u64,
}

pub const PMCMD_TYPE_MASK: c_uint = 0x000000ff;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum acrn_pm_cmd_type {
    ACRN_PMCMD_GET_PX_CNT,
    ACRN_PMCMD_GET_PX_DATA,
    ACRN_PMCMD_GET_CX_CNT,
    ACRN_PMCMD_GET_CX_DATA,
}

pub const ACRN_IOEVENTFD_FLAG_PIO: c_uint = 0x01;
pub const ACRN_IOEVENTFD_FLAG_DATAMATCH: c_uint = 0x02;
pub const ACRN_IOEVENTFD_FLAG_DEASSIGN: c_uint = 0x04;
//
// struct acrn_ioeventfd - Data to operate a &struct hsm_ioeventfd
// @fd:		The fd of eventfd associated with a hsm_ioeventfd
// @flags:	Logical-OR of ACRN_IOEVENTFD_FLAG_
// @addr:	The start address of IO range of ioeventfd
// @len:	The length of IO range of ioeventfd
// @reserved:	Reserved and should be 0
// @data:	Data for data matching
//
// Without flag ACRN_IOEVENTFD_FLAG_DEASSIGN, ioctl ACRN_IOCTL_IOEVENTFD
// creates a &struct hsm_ioeventfd with properties originated from &struct
// acrn_ioeventfd. With flag ACRN_IOEVENTFD_FLAG_DEASSIGN, ioctl
// ACRN_IOCTL_IOEVENTFD destroys the &struct hsm_ioeventfd matching the fd.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_ioeventfd {
    pub fd: __u32,
    pub flags: __u32,
    pub addr: __u64,
    pub len: __u32,
    pub reserved: __u32,
    pub data: __u64,
}

pub const ACRN_IRQFD_FLAG_DEASSIGN: c_uint = 0x01;
//
// struct acrn_irqfd - Data to operate a &struct hsm_irqfd
// @fd:		The fd of eventfd associated with a hsm_irqfd
// @flags:	Logical-OR of ACRN_IRQFD_FLAG_
// @msi:	Info of MSI associated with the irqfd
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_irqfd {
    pub fd: __s32,
    pub flags: __u32,
    pub msi: acrn_msi_entry,
}

// The ioctl type, documented in ioctl-number.rst
pub const ACRN_IOCTL_TYPE: c_uint = 0xA2;
//
// Common IOCTL IDs definition for ACRN userspace
//

