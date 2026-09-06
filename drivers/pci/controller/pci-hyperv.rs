//! Automatically rewritten from C to Rust
//! Source: drivers/pci/controller/pci-hyperv.c
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
// Copyright (c) Microsoft Corporation.
//
// Author:
// Jake Oshins <jakeo@microsoft.com>
//
// This driver acts as a paravirtual front-end for PCI Express root buses.
// When a PCI Express function (either an entire device or an SR-IOV
// Virtual Function) is being passed through to the VM, this driver exposes
// a new bus to the guest VM.  This is modeled as a root PCI bus because
// no bridges are being exposed to the VM.  In fact, with a "Generation 2"
// VM within Hyper-V, there may seem to be no PCI bus at all in the VM
// until a device as been exposed using this driver.
//
// Each root PCI bus has its own PCI domain, which is called "Segment" in
// the PCI Firmware Specifications.  Thus while each device passed through
// to the VM using this front-end will appear at "device 0", the domain will
// be unique.  Typically, each bus will have one PCI function on it, though
// this driver does support more than one.
//
// In order to map the interrupts from the device through to the guest VM,
// this driver also implements an IRQ Domain, which handles interrupts (either
// MSI or MSI-X) associated with the functions on the bus.  As interrupts are
// set up, torn down, or reaffined, this driver communicates with the
// underlying hypervisor to adjust the mappings in the I/O MMU so that each
// interrupt will be delivered to the correct virtual processor at the right
// vector.  This driver does not support level-triggered (line-based)
// interrupts, and will report that the Interrupt Line register in the
// function's configuration space is zero.
//
// The rest of this driver mostly maps PCI concepts onto underlying Hyper-V
// facilities.  For instance, the configuration space of a function exposed
// by Hyper-V is mapped into a single page of memory space, and the
// read and write handlers for config space must be aware of this mechanism.
// Similarly, device setup and teardown involves messages sent to and from
// the PCI back-end driver in Hyper-V.
//

//
// Protocol versions. The low word is the minor version, the high word the
// major version.
//

    enum pci_protocol_version_t {
    PCI_PROTOCOL_VERSION_1_1 = PCI_MAKE_VERSION(1, 1),	/* Win10 */
    PCI_PROTOCOL_VERSION_1_2 = PCI_MAKE_VERSION(1, 2),	/* RS1 */
    PCI_PROTOCOL_VERSION_1_3 = PCI_MAKE_VERSION(1, 3),	/* Vibranium */
    PCI_PROTOCOL_VERSION_1_4 = PCI_MAKE_VERSION(1, 4),	/* WS2022 */
    };

//
// Supported protocol versions in the order of probing - highest go
// first.
//
    static enum pci_protocol_version_t pci_protocol_versions[] = {
    PCI_PROTOCOL_VERSION_1_4,
    PCI_PROTOCOL_VERSION_1_3,
    PCI_PROTOCOL_VERSION_1_2,
    PCI_PROTOCOL_VERSION_1_1,
    };
pub const PCI_CONFIG_MMIO_LENGTH: c_uint = 0x2000;
pub const CFG_PAGE_OFFSET: c_uint = 0x1000;

pub const MAX_SUPPORTED_MSI_MESSAGES: c_uint = 0x400;
pub const STATUS_REVISION_MISMATCH: c_uint = 0xC0000059;
// space for 32bit serial number as string
pub const SLOT_NAME_SIZE: c_int = 11;
//
// Size of requestor for VMbus; the value is based on the observation
// that having more than one request outstanding is 'rare', and so 64
// should be generous in ensuring that we don't ever run out.
//
pub const HV_PCI_RQSTOR_SIZE: c_int = 64;
//
// Message Types
//
    enum pci_message_type {
//
// Version 1.1
//
    PCI_MESSAGE_BASE                = 0x42490000,
    PCI_BUS_RELATIONS               = PCI_MESSAGE_BASE + 0,
    PCI_QUERY_BUS_RELATIONS         = PCI_MESSAGE_BASE + 1,
    PCI_POWER_STATE_CHANGE          = PCI_MESSAGE_BASE + 4,
    PCI_QUERY_RESOURCE_REQUIREMENTS = PCI_MESSAGE_BASE + 5,
    PCI_QUERY_RESOURCE_RESOURCES    = PCI_MESSAGE_BASE + 6,
    PCI_BUS_D0ENTRY                 = PCI_MESSAGE_BASE + 7,
    PCI_BUS_D0EXIT                  = PCI_MESSAGE_BASE + 8,
    PCI_READ_BLOCK                  = PCI_MESSAGE_BASE + 9,
    PCI_WRITE_BLOCK                 = PCI_MESSAGE_BASE + 0xA,
    PCI_EJECT                       = PCI_MESSAGE_BASE + 0xB,
    PCI_QUERY_STOP                  = PCI_MESSAGE_BASE + 0xC,
    PCI_REENABLE                    = PCI_MESSAGE_BASE + 0xD,
    PCI_QUERY_STOP_FAILED           = PCI_MESSAGE_BASE + 0xE,
    PCI_EJECTION_COMPLETE           = PCI_MESSAGE_BASE + 0xF,
    PCI_RESOURCES_ASSIGNED          = PCI_MESSAGE_BASE + 0x10,
    PCI_RESOURCES_RELEASED          = PCI_MESSAGE_BASE + 0x11,
    PCI_INVALIDATE_BLOCK            = PCI_MESSAGE_BASE + 0x12,
    PCI_QUERY_PROTOCOL_VERSION      = PCI_MESSAGE_BASE + 0x13,
    PCI_CREATE_INTERRUPT_MESSAGE    = PCI_MESSAGE_BASE + 0x14,
    PCI_DELETE_INTERRUPT_MESSAGE    = PCI_MESSAGE_BASE + 0x15,
    PCI_RESOURCES_ASSIGNED2		= PCI_MESSAGE_BASE + 0x16,
    PCI_CREATE_INTERRUPT_MESSAGE2	= PCI_MESSAGE_BASE + 0x17,
    PCI_DELETE_INTERRUPT_MESSAGE2	= PCI_MESSAGE_BASE + 0x18, /* unused */
    PCI_BUS_RELATIONS2		= PCI_MESSAGE_BASE + 0x19,
    PCI_RESOURCES_ASSIGNED3         = PCI_MESSAGE_BASE + 0x1A,
    PCI_CREATE_INTERRUPT_MESSAGE3   = PCI_MESSAGE_BASE + 0x1B,
    PCI_MESSAGE_MAXIMUM
    };
//
// Structures defining the virtual PCI Express protocol.
//
    union pci_version {
    struct {
    u16 minor_version;
    u16 major_version;
    } parts;
    u32 version;
    } __packed;
//
// Function numbers are 8-bits wide on Express, as interpreted through ARI,
// which is all this driver does.  This representation is the one used in
// Windows, which is what is expected when sending this back and forth with
// the Hyper-V parent partition.
//
    union win_slot_encoding {
    struct {
    u32	dev:5;
    u32	func:3;
    u32	reserved:24;
    } bits;
    u32 slot;
    } __packed;
//
// Pretty much as defined in the PCI Specifications.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_function_description {
    pub /: *mut *mut u16 v_id; / vendor ID,
    pub /: *mut *mut u16 d_id; / device ID,
    pub rev: u8,
    pub prog_intf: u8,
    pub subclass: u8,
    pub base_class: u8,
    pub subsystem_id: u32,
    pub win_slot: union win_slot_encoding,
    pub /: *mut *mut u32 ser; / serial number,
    pub __packed: },
    enum pci_device_description_flags {
    HV_PCI_DEVICE_FLAG_NONE			= 0x0,
    HV_PCI_DEVICE_FLAG_NUMA_AFFINITY	= 0x1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_function_description2 {
    pub /: *mut *mut u16 v_id; / vendor ID,
    pub /: *mut *mut u16 d_id; / device ID,
    pub rev: u8,
    pub prog_intf: u8,
    pub subclass: u8,
    pub base_class: u8,
    pub subsystem_id: u32,
    pub win_slot: union win_slot_encoding,
    pub /: *mut *mut u32 ser; / serial number,
    pub flags: u32,
    pub virtual_numa_node: u16,
    pub reserved: u16,
    pub __packed: },
//
// struct hv_msi_desc
// @vector:		IDT entry
// @delivery_mode:	As defined in Intel's Programmer's
// Reference Manual, Volume 3, Chapter 8.
// @vector_count:	Number of contiguous entries in the
// Interrupt Descriptor Table that are
// occupied by this Message-Signaled
// Interrupt. For "MSI", as first defined
// in PCI 2.2, this can be between 1 and
// 32. For "MSI-X," as first defined in PCI
// 3.0, this must be 1, as each MSI-X table
// entry would have its own descriptor.
// @reserved:		Empty space
// @cpu_mask:		All the target virtual processors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_msi_desc {
    pub vector: u8,
    pub delivery_mode: u8,
    pub vector_count: u16,
    pub reserved: u32,
    pub cpu_mask: u64,
    pub __packed: },
//
// struct hv_msi_desc2 - 1.2 version of hv_msi_desc
// @vector:		IDT entry
// @delivery_mode:	As defined in Intel's Programmer's
// Reference Manual, Volume 3, Chapter 8.
// @vector_count:	Number of contiguous entries in the
// Interrupt Descriptor Table that are
// occupied by this Message-Signaled
// Interrupt. For "MSI", as first defined
// in PCI 2.2, this can be between 1 and
// 32. For "MSI-X," as first defined in PCI
// 3.0, this must be 1, as each MSI-X table
// entry would have its own descriptor.
// @processor_count:	number of bits enabled in array.
// @processor_array:	All the target virtual processors.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_msi_desc2 {
    pub vector: u8,
    pub delivery_mode: u8,
    pub vector_count: u16,
    pub processor_count: u16,
    pub processor_array: [u16; 32],
    pub __packed: },
//
// struct hv_msi_desc3 - 1.3 version of hv_msi_desc
// Everything is the same as in 'hv_msi_desc2' except that the size of the
// 'vector' field is larger to support bigger vector values. For ex: LPI
// vectors on ARM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_msi_desc3 {
    pub vector: u32,
    pub delivery_mode: u8,
    pub reserved: u8,
    pub vector_count: u16,
    pub processor_count: u16,
    pub processor_array: [u16; 32],
    pub __packed: },
//
// struct tran_int_desc
// @reserved:		unused, padding
// @vector_count:	same as in hv_msi_desc
// @data:		This is the "data payload" value that is
// written by the device when it generates
// a message-signaled interrupt, either MSI
// or MSI-X.
// @address:		This is the address to which the data
// payload is written on interrupt
// generation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tran_int_desc {
    pub reserved: u16,
    pub vector_count: u16,
    pub data: u32,
    pub address: u64,
    pub __packed: },
//
// A generic message format for virtual PCI.
// Specific message formats are defined later in the file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_message {
    pub type: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_child_message {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_incoming_message {
    pub hdr: vmpacket_descriptor,
    pub message_type: pci_message,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_response {
    pub hdr: vmpacket_descriptor,
    pub /: *mut *mut s32 status; / negative values are failures,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_packet {
    void (*completion_func)(void *context, struct pci_response *resp,
    pub resp_packet_size): c_int,
    pub compl_ctxt: *mut c_void,
}

//
// Specific message types supporting the PCI protocol.
//
// Version negotiation message. Sent from the guest to the host.
// The guest is free to try different versions until the host
// accepts the version.
//
// pci_version: The protocol version requested.
// is_last_attempt: If TRUE, this is the last version guest will request.
// reservedz: Reserved field, set to zero.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_version_request {
    pub message_type: pci_message,
    pub protocol_version: u32,
    pub __packed: },
//
// Bus D0 Entry.  This is sent from the guest to the host when the virtual
// bus (PCI Express port) is ready for action.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bus_d0_entry {
    pub message_type: pci_message,
    pub reserved: u32,
    pub mmio_base: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bus_relations {
    pub incoming: pci_incoming_message,
    pub device_count: u32,
    pub func: [pci_function_description; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_bus_relations2 {
    pub incoming: pci_incoming_message,
    pub device_count: u32,
    pub func: [pci_function_description2; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_q_res_req_response {
    pub hdr: vmpacket_descriptor,
    pub /: *mut *mut s32 status; / negative values are failures,
    pub probed_bar: [u32; PCI_STD_NUM_BARS],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_set_power {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub /: *mut *mut u32 power_state; / In Windows terms,
    pub reserved: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_set_power_response {
    pub hdr: vmpacket_descriptor,
    pub /: *mut *mut s32 status; / negative values are failures,
    pub wslot: union win_slot_encoding,
    pub /: *mut *mut u32 resultant_state; / In Windows terms,
    pub reserved: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_resources_assigned {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub /: *mut *mut u8 memory_range[0x14][6]; / not used here,
    pub msi_descriptors: u32,
    pub reserved: [u32; 4],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_resources_assigned2 {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub /: *mut *mut u8 memory_range[0x14][6]; / not used here,
    pub msi_descriptor_count: u32,
    pub reserved: [u8; 70],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_create_interrupt {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub int_desc: hv_msi_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_create_int_response {
    pub response: pci_response,
    pub reserved: u32,
    pub int_desc: tran_int_desc,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_create_interrupt2 {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub int_desc: hv_msi_desc2,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_create_interrupt3 {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub int_desc: hv_msi_desc3,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_delete_interrupt {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub int_desc: tran_int_desc,
    pub __packed: },
//
// Note: the VM must pass a valid block id, wslot and bytes_requested.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_read_block {
    pub message_type: pci_message,
    pub block_id: u32,
    pub wslot: union win_slot_encoding,
    pub bytes_requested: u32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_read_block_response {
    pub hdr: vmpacket_descriptor,
    pub status: u32,
    pub bytes: [u8; HV_CONFIG_BLOCK_SIZE_MAX],
    pub __packed: },
//
// Note: the VM must pass a valid block id, wslot and byte_count.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_write_block {
    pub message_type: pci_message,
    pub block_id: u32,
    pub wslot: union win_slot_encoding,
    pub byte_count: u32,
    pub bytes: [u8; HV_CONFIG_BLOCK_SIZE_MAX],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dev_inval_block {
    pub incoming: pci_incoming_message,
    pub wslot: union win_slot_encoding,
    pub block_mask: u64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_dev_incoming {
    pub incoming: pci_incoming_message,
    pub wslot: union win_slot_encoding,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_eject_response {
    pub message_type: pci_message,
    pub wslot: union win_slot_encoding,
    pub status: u32,
    pub __packed: },
    pub VMBUS_RING_SIZE(SZ_16K): static int pci_ring_size =,
//
// Driver specific state.
//
    enum hv_pcibus_state {
    hv_pcibus_init = 0,
    hv_pcibus_probed,
    hv_pcibus_installed,
    hv_pcibus_removing,
    hv_pcibus_maximum
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_pcibus_device {

    pub sysdata: pci_sysdata,

    pub sysdata: pci_config_window,

    pub bridge: *mut pci_host_bridge,
    pub fwnode: *mut fwnode_handle,
// Protocol version negotiated with the host
    pub protocol_version: enum pci_protocol_version_t,
    pub state_lock: mutex,
    pub state: enum hv_pcibus_state,
    pub hdev: *mut hv_device,
    pub low_mmio_space: resource_size_t,
    pub high_mmio_space: resource_size_t,
    pub mem_config: *mut resource,
    pub low_mmio_res: *mut resource,
    pub high_mmio_res: *mut resource,
    pub survey_event: *mut completion,
    pub /: *mut *mut spinlock_t config_lock; / Avoid two threads writing index page,
    pub /: *mut *mut spinlock_t device_list_lock; / Protect lists below,
    pub cfg_addr: *mut void __iomem,
    pub children: list_head,
    pub dr_list: list_head,
    pub irq_domain: *mut irq_domain,
    pub wq: *mut workqueue_struct,
// Highest slot of child device with resources allocated
    pub wslot_res_allocated: c_int,
    pub /: *mut *mut bool use_calls; / Use hypercalls to access mmio cfg space,
}

//
// Tracks "Device Relations" messages from the host, which must be both
// processed in order and deferred so that they don't run in the context
// of the incoming packet callback.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_dr_work {
    pub wrk: work_struct,
    pub bus: *mut hv_pcibus_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_pcidev_description {
    pub /: *mut *mut u16 v_id; / vendor ID,
    pub /: *mut *mut u16 d_id; / device ID,
    pub rev: u8,
    pub prog_intf: u8,
    pub subclass: u8,
    pub base_class: u8,
    pub subsystem_id: u32,
    pub win_slot: union win_slot_encoding,
    pub /: *mut *mut u32 ser; / serial number,
    pub flags: u32,
    pub virtual_numa_node: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_dr_state {
    pub list_entry: list_head,
    pub device_count: u32,
    pub __counted_by(device_count): hv_pcidev_description func[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_pci_dev {
// List protected by pci_rescan_remove_lock
    pub list_entry: list_head,
    pub refs: refcount_t,
    pub pci_slot: *mut pci_slot,
    pub desc: hv_pcidev_description,
    pub reported_missing: bool,
    pub hbus: *mut hv_pcibus_device,
    pub wrk: work_struct,
    pub block_mask): *mut *mut *mut void (block_invalidate)(void context, u64,
    pub invalidate_context: *mut c_void,
//
// What would be observed if one wrote 0xFFFFFFFF to a BAR and then
// read it back, for each of the BAR offsets within config space.
//
    pub probed_bar: [u32; PCI_STD_NUM_BARS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_pci_compl {
    pub host_event: completion,
    pub completion_status: i32,
}

    static void hv_pci_onchannelcallback(void *context);

#[no_mangle]
unsafe extern "C" fn hv_pci_irqchip_init() -> c_int {
    static int hv_pci_irqchip_init(void)
    {
    return 0;
    }
    static struct irq_domain *hv_pci_get_root_domain(void)
    {
    return x86_vector_domain;
    }
#[no_mangle]
unsafe extern "C" fn hv_msi_get_int_vector(data: *mut irq_data) -> c_uint {
    static unsigned int hv_msi_get_int_vector(struct irq_data *data)
    {
    struct irq_cfg *cfg = irqd_cfg(data);
    return cfg.vector;
    }

//
// hv_irq_retarget_interrupt() - "Unmask" the IRQ by setting its current
// affinity.
// @data:	Describes the IRQ
//
// Build new a destination for the MSI and make a hypercall to
// update the Interrupt Redirection Table. "Device Logical ID"
// is built out of this PCI bus's instance GUID and the function
// number of the device.
//
#[no_mangle]
unsafe extern "C" fn hv_irq_retarget_interrupt(data: *mut irq_data) {
    static void hv_irq_retarget_interrupt(struct irq_data *data)
    {
    struct msi_desc *msi_desc = irq_data_get_msi_desc(data);
    struct hv_retarget_device_interrupt *params;
    struct tran_int_desc *int_desc;
    struct hv_pcibus_device *hbus;
    const struct cpumask *dest;
    cpumask_var_t tmp;
    struct pci_bus *pbus;
    struct pci_dev *pdev;
    unsigned long flags;
    let mut var_size: u32 = 0;
    int cpu, nr_bank;
    u64 res;
    dest = irq_data_get_effective_affinity_mask(data);
    pdev = msi_desc_to_pci_dev(msi_desc);
    pbus = pdev.bus;
    hbus = container_of(pbus.sysdata, struct hv_pcibus_device, sysdata);
    int_desc = data.chip_data;
    if (!int_desc) {
    dev_warn(&hbus.hdev.device, "%s() can not unmask irq %u\n",
    __func__, data.irq);
    return;
    }
    local_irq_save(flags);
    params = *this_cpu_ptr(hyperv_pcpu_input_arg);
    memset(params, 0, sizeof(*params));
    params.partition_id = HV_PARTITION_ID_SELF;
    params.int_entry.source = HV_INTERRUPT_SOURCE_MSI;
    params.int_entry.msi_entry.address.as_uint32 = int_desc.address & 0xffffffff;
    params.int_entry.msi_entry.data.as_uint32 = int_desc.data;
    params.device_id = (hbus.hdev.dev_instance.b[5] << 24) |
    (hbus.hdev.dev_instance.b[4] << 16) |
    (hbus.hdev.dev_instance.b[7] << 8) |
    (hbus.hdev.dev_instance.b[6] & 0xf8) |
    PCI_FUNC(pdev.devfn);
    params.int_target.vector = hv_msi_get_int_vector(data);
    if (hbus.protocol_version >= PCI_PROTOCOL_VERSION_1_2) {
//
// PCI_PROTOCOL_VERSION_1_2 supports the VP_SET version of the
// HVCALL_RETARGET_INTERRUPT hypercall, which also coincides
// with >64 VP support.
// ms_hyperv.hints & HV_X64_EX_PROCESSOR_MASKS_RECOMMENDED
// is not sufficient for this hypercall.
//
    params.int_target.flags |=
    HV_DEVICE_INTERRUPT_TARGET_PROCESSOR_SET;
    if (!alloc_cpumask_var(&tmp, GFP_ATOMIC)) {
    res = 1;
    goto out;
    }
    cpumask_and(tmp, dest, cpu_online_mask);
    nr_bank = cpumask_to_vpset(&params.int_target.vp_set, tmp);
    free_cpumask_var(tmp);
    if (nr_bank <= 0) {
    res = 1;
    goto out;
    }
//
// var-sized hypercall, var-size starts after vp_mask (thus
// vp_set.format does not count, but vp_set.valid_bank_mask
// does).
//
    var_size = 1 + nr_bank;
    } else {
    for_each_cpu_and(cpu, dest, cpu_online_mask) {
    params.int_target.vp_mask |=
    (1ULL << hv_cpu_number_to_vp_number(cpu));
    }
    }
    res = hv_do_hypercall(HVCALL_RETARGET_INTERRUPT | (var_size << 17),
    params, core::ptr::null_mut());
    out:
    local_irq_restore(flags);
//
// During hibernation, when a CPU is offlined, the kernel tries
// to move the interrupt to the remaining CPUs that haven't
// been offlined yet. In this case, the below hv_do_hypercall()
// always fails since the vmbus channel has been closed:
// refer to cpu_disable_common() -> fixup_irqs() ->
// irq_migrate_all_off_this_cpu() -> migrate_one_irq().
//
// Suppress the error message for hibernation because the failure
// during hibernation does not matter (at this time all the devices
// have been frozen). Note: the correct affinity info is still updated
// into the irqdata data structure in migrate_one_irq() ->
// irq_do_set_affinity(), so later when the VM resumes,
// hv_pci_restore_msi_state() is able to correctly restore the
// interrupt with the correct affinity.
//
    if (!hv_result_success(res) && hbus.state != hv_pcibus_removing)
    dev_err(&hbus.hdev.device,
    "%s() failed: %#llx", __func__, res);
    }
#[no_mangle]
unsafe extern "C" fn hv_arch_irq_unmask(data: *mut irq_data) {
    static void hv_arch_irq_unmask(struct irq_data *data)
    {
    if (hv_root_partition())
//
// In case of the nested root partition, the nested hypervisor
// is taking care of interrupt remapping and thus the
// MAP_DEVICE_INTERRUPT hypercall is required instead of
// RETARGET_INTERRUPT.
//
    (void)hv_map_msi_interrupt(data, core::ptr::null_mut());
    else
    hv_irq_retarget_interrupt(data);
    }

//
// SPI vectors to use for vPCI; arch SPIs range is [32, 1019], but leaving a bit
// of room at the start to allow for SPIs to be specified through ACPI and
// starting with a power of two to satisfy power of 2 multi-MSI requirement.
//
pub const HV_PCI_MSI_SPI_START: c_int = 64;

pub const DELIVERY_MODE: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_pci_chip_data {
    pub HV_PCI_MSI_SPI_NR): DECLARE_BITMAP(spi_map,,
    pub map_lock: mutex,
}

// Hyper-V vPCI MSI GIC IRQ domain
    static struct irq_domain *hv_msi_gic_irq_domain;
// Hyper-V PCI MSI IRQ chip
    static struct irq_chip hv_arm64_msi_irq_chip = {
    .name = "MSI",
    .irq_set_affinity = irq_chip_set_affinity_parent,
    .irq_eoi = irq_chip_eoi_parent,
    .irq_mask = irq_chip_mask_parent,
    .irq_unmask = irq_chip_unmask_parent
    };
#[no_mangle]
unsafe extern "C" fn hv_msi_get_int_vector(irqd: *mut irq_data) -> c_uint {
    static unsigned int hv_msi_get_int_vector(struct irq_data *irqd)
    {
    return irqd.parent_data.hwirq;
    }
//
// @nr_bm_irqs:		Indicates the number of IRQs that were allocated from
// the bitmap.
// @nr_dom_irqs:	Indicates the number of IRQs that were allocated from
// the parent domain.
//
    static void hv_pci_vec_irq_free(struct irq_domain *domain,
    unsigned int virq,
    unsigned int nr_bm_irqs,
    unsigned int nr_dom_irqs)
    {
    struct hv_pci_chip_data *chip_data = domain.host_data;
    struct irq_data *d = irq_domain_get_irq_data(domain, virq);
    let mut first: c_int = d.hwirq - HV_PCI_MSI_SPI_START;
    int i;
    mutex_lock(&chip_data.map_lock);
    bitmap_release_region(chip_data.spi_map,
    first,
    get_count_order(nr_bm_irqs));
    mutex_unlock(&chip_data.map_lock);
    for (i = 0; i < nr_dom_irqs; i++) {
    if (i)
    d = irq_domain_get_irq_data(domain, virq + i);
    irq_domain_reset_irq_data(d);
    }
    irq_domain_free_irqs_parent(domain, virq, nr_dom_irqs);
    }
    static void hv_pci_vec_irq_domain_free(struct irq_domain *domain,
    unsigned int virq,
    unsigned int nr_irqs)
    {
    hv_pci_vec_irq_free(domain, virq, nr_irqs, nr_irqs);
    }
    static int hv_pci_vec_alloc_device_irq(struct irq_domain *domain,
    unsigned int nr_irqs,
    irq_hw_number_t *hwirq)
    {
    struct hv_pci_chip_data *chip_data = domain.host_data;
    int index;
// Find and allocate region from the SPI bitmap
    mutex_lock(&chip_data.map_lock);
    index = bitmap_find_free_region(chip_data.spi_map,
    HV_PCI_MSI_SPI_NR,
    get_count_order(nr_irqs));
    mutex_unlock(&chip_data.map_lock);
    if (index < 0)
    return -ENOSPC;
// hwirq = index + HV_PCI_MSI_SPI_START;
    return 0;
    }
    static int hv_pci_vec_irq_gic_domain_alloc(struct irq_domain *domain,
    unsigned int virq,
    irq_hw_number_t hwirq)
    {
    struct irq_fwspec fwspec;
    struct irq_data *d;
    int ret;
    fwspec.fwnode = domain.parent.fwnode;
    if (is_of_node(fwspec.fwnode)) {
// SPI lines for OF translations start at offset 32
    fwspec.param_count = 3;
    fwspec.param[0] = 0;
    fwspec.param[1] = hwirq - 32;
    fwspec.param[2] = IRQ_TYPE_EDGE_RISING;
    } else {
    fwspec.param_count = 2;
    fwspec.param[0] = hwirq;
    fwspec.param[1] = IRQ_TYPE_EDGE_RISING;
    }
    ret = irq_domain_alloc_irqs_parent(domain, virq, 1, &fwspec);
    if (ret)
    return ret;
//
// Since the interrupt specifier is not coming from ACPI or DT, the
// trigger type will need to be set explicitly. Otherwise, it will be
// set to whatever is in the GIC configuration.
//
    d = irq_domain_get_irq_data(domain.parent, virq);
    return d.chip.irq_set_type(d, IRQ_TYPE_EDGE_RISING);
    }
    static int hv_pci_vec_irq_domain_alloc(struct irq_domain *domain,
    unsigned int virq, unsigned int nr_irqs,
    void *args)
    {
    irq_hw_number_t hwirq;
    unsigned int i;
    int ret;
    ret = hv_pci_vec_alloc_device_irq(domain, nr_irqs, &hwirq);
    if (ret)
    return ret;
    for (i = 0; i < nr_irqs; i++) {
    ret = hv_pci_vec_irq_gic_domain_alloc(domain, virq + i,
    hwirq + i);
    if (ret) {
    hv_pci_vec_irq_free(domain, virq, nr_irqs, i);
    return ret;
    }
    irq_domain_set_hwirq_and_chip(domain, virq + i,
    hwirq + i,
    &hv_arm64_msi_irq_chip,
    domain.host_data);
    pr_debug("pID:%d vID:%u\n", (int)(hwirq + i), virq + i);
    }
    return 0;
    }
//
// Pick the first cpu as the irq affinity that can be temporarily used for
// composing MSI from the hypervisor. GIC will eventually set the right
// affinity for the irq and the 'unmask' will retarget the interrupt to that
// cpu.
//
    static int hv_pci_vec_irq_domain_activate(struct irq_domain *domain,
    struct irq_data *irqd, bool reserve)
    {
    let mut cpu: c_int = cpumask_first(cpu_present_mask);
    irq_data_update_effective_affinity(irqd, cpumask_of(cpu));
    return 0;
    }
    static const struct irq_domain_ops hv_pci_domain_ops = {
    .alloc	= hv_pci_vec_irq_domain_alloc,
    .free	= hv_pci_vec_irq_domain_free,
    .activate = hv_pci_vec_irq_domain_activate,
    };

    static struct irq_domain *hv_pci_of_irq_domain_parent(void)
    {
    struct device_node *parent;
    struct irq_domain *domain;
    parent = of_irq_find_parent(hv_get_vmbus_root_device().of_node);
    if (!parent)
    return core::ptr::null_mut();
    domain = irq_find_host(parent);
    of_node_put(parent);
    return domain;
    }

    static struct irq_domain *hv_pci_acpi_irq_domain_parent(void)
    {
    acpi_gsi_domain_disp_fn gsi_domain_disp_fn;
    gsi_domain_disp_fn = acpi_get_gsi_dispatcher();
    if (!gsi_domain_disp_fn)
    return core::ptr::null_mut();
    return irq_find_matching_fwnode(gsi_domain_disp_fn(0),
    DOMAIN_BUS_ANY);
    }

#[no_mangle]
unsafe extern "C" fn hv_pci_irqchip_init() -> c_int {
    static int hv_pci_irqchip_init(void)
    {
    static struct hv_pci_chip_data *chip_data;
    struct fwnode_handle *fn = core::ptr::null_mut();
    struct irq_domain *irq_domain_parent = core::ptr::null_mut();
    let mut ret: c_int = -ENOMEM;
    chip_data = kzalloc_obj(*chip_data);
    if (!chip_data)
    return ret;
    mutex_init(&chip_data.map_lock);
    fn = irq_domain_alloc_named_fwnode("hv_vpci_arm64");
    if (!fn)
    goto free_chip;
//
// IRQ domain once enabled, should not be removed since there is no
// way to ensure that all the corresponding devices are also gone and
// no interrupts will be generated.
//

    if (!acpi_disabled)
    irq_domain_parent = hv_pci_acpi_irq_domain_parent();

    if (!irq_domain_parent)
    irq_domain_parent = hv_pci_of_irq_domain_parent();

    if (!irq_domain_parent) {
    WARN_ONCE(1, "Invalid firmware configuration for VMBus interrupts\n");
    ret = -EINVAL;
    goto free_chip;
    }
    hv_msi_gic_irq_domain = irq_domain_create_hierarchy(irq_domain_parent, 0,
    HV_PCI_MSI_SPI_NR,
    fn, &hv_pci_domain_ops,
    chip_data);
    if (!hv_msi_gic_irq_domain) {
    pr_err("Failed to create Hyper-V arm64 vPCI MSI IRQ domain\n");
    goto free_chip;
    }
    return 0;
    free_chip:
    kfree(chip_data);
    if (fn)
    irq_domain_free_fwnode(fn);
    return ret;
    }
    static struct irq_domain *hv_pci_get_root_domain(void)
    {
    return hv_msi_gic_irq_domain;
    }
//
// SPIs are used for interrupts of PCI devices and SPIs is managed via GICD
// registers which Hyper-V already supports, so no hypercall needed.
//
    static void hv_arch_irq_unmask(struct irq_data *data) { }

//
// hv_pci_generic_compl() - Invoked for a completion packet
// @context:		Set up by the sender of the packet.
// @resp:		The response packet
// @resp_packet_size:	Size in bytes of the packet
//
// This function is used to trigger an event and report status
// for any message for which the completion packet contains a
// status and nothing else.
//
    static void hv_pci_generic_compl(void *context, struct pci_response *resp,
    int resp_packet_size)
    {
    struct hv_pci_compl *comp_pkt = context;
    comp_pkt.completion_status = resp.status;
    complete(&comp_pkt.host_event);
    }
    static struct hv_pci_dev *get_pcichild_wslot(struct hv_pcibus_device *hbus,
    u32 wslot);
#[no_mangle]
unsafe extern "C" fn get_pcichild(hpdev: *mut hv_pci_dev) {
    static void get_pcichild(struct hv_pci_dev *hpdev)
    {
    refcount_inc(&hpdev.refs);
    }
#[no_mangle]
unsafe extern "C" fn put_pcichild(hpdev: *mut hv_pci_dev) {
    static void put_pcichild(struct hv_pci_dev *hpdev)
    {
    if (refcount_dec_and_test(&hpdev.refs))
    kfree(hpdev);
    }
//
// There is no good way to get notified from vmbus_onoffer_rescind(),
// so let's use polling here, since this is not a hot path.
//
    static int wait_for_response(struct hv_device *hdev,
    struct completion *comp)
    {
    while (true) {
    if (hdev.channel.rescind) {
    dev_warn_once(&hdev.device, "The device is gone.\n");
    return -ENODEV;
    }
    if (wait_for_completion_timeout(comp, HZ / 10))
    break;
    }
    return 0;
    }
//
// devfn_to_wslot() - Convert from Linux PCI slot to Windows
// @devfn:	The Linux representation of PCI slot
//
// Windows uses a slightly different representation of PCI slot.
//
// Return: The Windows representation
//
#[no_mangle]
unsafe extern "C" fn devfn_to_wslot(devfn: c_int) -> u32 {
    static u32 devfn_to_wslot(int devfn)
    {
    union win_slot_encoding wslot;
    wslot.slot = 0;
    wslot.bits.dev = PCI_SLOT(devfn);
    wslot.bits.func = PCI_FUNC(devfn);
    return wslot.slot;
    }
//
// wslot_to_devfn() - Convert from Windows PCI slot to Linux
// @wslot:	The Windows representation of PCI slot
//
// Windows uses a slightly different representation of PCI slot.
//
// Return: The Linux representation
//
#[no_mangle]
unsafe extern "C" fn wslot_to_devfn(wslot: u32) -> c_int {
    static int wslot_to_devfn(u32 wslot)
    {
    union win_slot_encoding slot_no;
    slot_no.slot = wslot;
    return PCI_DEVFN(slot_no.bits.dev, slot_no.bits.func);
    }
#[no_mangle]
unsafe extern "C" fn hv_pci_read_mmio(dev: *mut device, gpa: phys_addr_t, size: c_int, val: *mut u32) {
    static void hv_pci_read_mmio(struct device *dev, phys_addr_t gpa, int size, u32 *val)
    {
    struct hv_mmio_read_input *in;
    struct hv_mmio_read_output *out;
    u64 ret;
//
// Must be called with interrupts disabled so it is safe
// to use the per-cpu input argument page.  Use it for
// both input and output.
//
    in = *this_cpu_ptr(hyperv_pcpu_input_arg);
    out = *this_cpu_ptr(hyperv_pcpu_input_arg) + sizeof(*in);
    in.gpa = gpa;
    in.size = size;
    ret = hv_do_hypercall(HVCALL_MMIO_READ, in, out);
    if (hv_result_success(ret)) {
    switch (size) {
    case 1:
// val = *(u8 *)(out->data);
    break;
    case 2:
// val = *(u16 *)(out->data);
    break;
    default:
// val = *(u32 *)(out->data);
    break;
    }
    } else
    dev_err(dev, "MMIO read hypercall error %llx addr %llx size %d\n",
    ret, gpa, size);
    }
#[no_mangle]
unsafe extern "C" fn hv_pci_write_mmio(dev: *mut device, gpa: phys_addr_t, size: c_int, val: u32) {
    static void hv_pci_write_mmio(struct device *dev, phys_addr_t gpa, int size, u32 val)
    {
    struct hv_mmio_write_input *in;
    u64 ret;
//
// Must be called with interrupts disabled so it is safe
// to use the per-cpu input argument memory.
//
    in = *this_cpu_ptr(hyperv_pcpu_input_arg);
    in.gpa = gpa;
    in.size = size;
    switch (size) {
    case 1:
// (u8 *)(in->data) = val;
    break;
    case 2:
// (u16 *)(in->data) = val;
    break;
    default:
// (u32 *)(in->data) = val;
    break;
    }
    ret = hv_do_hypercall(HVCALL_MMIO_WRITE, in, core::ptr::null_mut());
    if (!hv_result_success(ret))
    dev_err(dev, "MMIO write hypercall error %llx addr %llx size %d\n",
    ret, gpa, size);
    }
//
// PCI Configuration Space for these root PCI buses is implemented as a pair
// of pages in memory-mapped I/O space.  Writing to the first page chooses
// the PCI function being written or read.  Once the first page has been
// written to, the following page maps in the entire configuration space of
// the function.
//
// _hv_pcifront_read_config() - Internal PCI config read
// @hpdev:	The PCI driver's representation of the device
// @where:	Offset within config space
// @size:	Size of the transfer
// @val:	Pointer to the buffer receiving the data
//
    static void _hv_pcifront_read_config(struct hv_pci_dev *hpdev, int where,
    int size, u32 *val)
    {
    struct hv_pcibus_device *hbus = hpdev.hbus;
    struct device *dev = &hbus.hdev.device;
    let mut offset: c_int = where + CFG_PAGE_OFFSET;
    unsigned long flags;
//
// If the attempt is to read the IDs or the ROM BAR, simulate that.
//
    if (where + size <= PCI_COMMAND) {
    memcpy(val, ((u8 *)&hpdev.desc.v_id) + where, size);
    } else if (where >= PCI_CLASS_REVISION && where + size <=
    PCI_CACHE_LINE_SIZE) {
    memcpy(val, ((u8 *)&hpdev.desc.rev) + where -
    PCI_CLASS_REVISION, size);
    } else if (where >= PCI_SUBSYSTEM_VENDOR_ID && where + size <=
    PCI_ROM_ADDRESS) {
    memcpy(val, (u8 *)&hpdev.desc.subsystem_id + where -
    PCI_SUBSYSTEM_VENDOR_ID, size);
    } else if (where >= PCI_ROM_ADDRESS && where + size <=
    PCI_CAPABILITY_LIST) {
// ROM BARs are unimplemented
// val = 0;
    } else if ((where >= PCI_INTERRUPT_LINE && where + size <= PCI_INTERRUPT_PIN) ||
    (where >= PCI_INTERRUPT_PIN && where + size <= PCI_MIN_GNT)) {
//
// Interrupt Line and Interrupt PIN are hard-wired to zero
// because this front-end only supports message-signaled
// interrupts.
//
// val = 0;
    } else if (where + size <= CFG_PAGE_SIZE) {
    spin_lock_irqsave(&hbus.config_lock, flags);
    if (hbus.use_calls) {
    let mut addr: phys_addr_t = hbus.mem_config.start + offset;
    hv_pci_write_mmio(dev, hbus.mem_config.start, 4,
    hpdev.desc.win_slot.slot);
    hv_pci_read_mmio(dev, addr, size, val);
    } else {
    void __iomem *addr = hbus.cfg_addr + offset;
// Choose the function to be read. (See comment above)
    writel(hpdev.desc.win_slot.slot, hbus.cfg_addr);
// Make sure the function was chosen before reading.
    mb();
// Read from that function's config space.
    switch (size) {
    case 1:
// val = readb(addr);
    break;
    case 2:
// val = readw(addr);
    break;
    default:
// val = readl(addr);
    break;
    }
//
// Make sure the read was done before we release the
// spinlock allowing consecutive reads/writes.
//
    mb();
    }
    spin_unlock_irqrestore(&hbus.config_lock, flags);
    } else {
    dev_err(dev, "Attempt to read beyond a function's config space.\n");
    }
    }
#[no_mangle]
unsafe extern "C" fn hv_pcifront_get_vendor_id(hpdev: *mut hv_pci_dev) -> u16 {
    static u16 hv_pcifront_get_vendor_id(struct hv_pci_dev *hpdev)
    {
    struct hv_pcibus_device *hbus = hpdev.hbus;
    struct device *dev = &hbus.hdev.device;
    u32 val;
    u16 ret;
    unsigned long flags;
    spin_lock_irqsave(&hbus.config_lock, flags);
    if (hbus.use_calls) {
    phys_addr_t addr = hbus.mem_config.start +
    CFG_PAGE_OFFSET + PCI_VENDOR_ID;
    hv_pci_write_mmio(dev, hbus.mem_config.start, 4,
    hpdev.desc.win_slot.slot);
    hv_pci_read_mmio(dev, addr, 2, &val);
    ret = val;  /* Truncates to 16 bits */
    } else {
    void __iomem *addr = hbus.cfg_addr + CFG_PAGE_OFFSET +
    PCI_VENDOR_ID;
// Choose the function to be read. (See comment above)
    writel(hpdev.desc.win_slot.slot, hbus.cfg_addr);
// Make sure the function was chosen before we start reading.
    mb();
// Read from that function's config space.
    ret = readw(addr);
//
// mb() is not required here, because the
// spin_unlock_irqrestore() is a barrier.
//
    }
    spin_unlock_irqrestore(&hbus.config_lock, flags);
    return ret;
    }
//
// _hv_pcifront_write_config() - Internal PCI config write
// @hpdev:	The PCI driver's representation of the device
// @where:	Offset within config space
// @size:	Size of the transfer
// @val:	The data being transferred
//
    static void _hv_pcifront_write_config(struct hv_pci_dev *hpdev, int where,
    int size, u32 val)
    {
    struct hv_pcibus_device *hbus = hpdev.hbus;
    struct device *dev = &hbus.hdev.device;
    let mut offset: c_int = where + CFG_PAGE_OFFSET;
    unsigned long flags;
    if (where >= PCI_SUBSYSTEM_VENDOR_ID &&
    where + size <= PCI_CAPABILITY_LIST) {
// SSIDs and ROM BARs are read-only
    } else if (where >= PCI_COMMAND && where + size <= CFG_PAGE_SIZE) {
    spin_lock_irqsave(&hbus.config_lock, flags);
    if (hbus.use_calls) {
    let mut addr: phys_addr_t = hbus.mem_config.start + offset;
    hv_pci_write_mmio(dev, hbus.mem_config.start, 4,
    hpdev.desc.win_slot.slot);
    hv_pci_write_mmio(dev, addr, size, val);
    } else {
    void __iomem *addr = hbus.cfg_addr + offset;
// Choose the function to write. (See comment above)
    writel(hpdev.desc.win_slot.slot, hbus.cfg_addr);
// Make sure the function was chosen before writing.
    wmb();
// Write to that function's config space.
    switch (size) {
    case 1:
    writeb(val, addr);
    break;
    case 2:
    writew(val, addr);
    break;
    default:
    writel(val, addr);
    break;
    }
//
// Make sure the write was done before we release the
// spinlock allowing consecutive reads/writes.
//
    mb();
    }
    spin_unlock_irqrestore(&hbus.config_lock, flags);
    } else {
    dev_err(dev, "Attempt to write beyond a function's config space.\n");
    }
    }
//
// hv_pcifront_read_config() - Read configuration space
// @bus: PCI Bus structure
// @devfn: Device/function
// @where: Offset from base
// @size: Byte/word/dword
// @val: Value to be read
//
// Return: PCIBIOS_SUCCESSFUL on success
// PCIBIOS_DEVICE_NOT_FOUND on failure
//
    static int hv_pcifront_read_config(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 *val)
    {
    struct hv_pcibus_device *hbus =
    container_of(bus.sysdata, struct hv_pcibus_device, sysdata);
    struct hv_pci_dev *hpdev;
    hpdev = get_pcichild_wslot(hbus, devfn_to_wslot(devfn));
    if (!hpdev)
    return PCIBIOS_DEVICE_NOT_FOUND;
    _hv_pcifront_read_config(hpdev, where, size, val);
    put_pcichild(hpdev);
    return PCIBIOS_SUCCESSFUL;
    }
//
// hv_pcifront_write_config() - Write configuration space
// @bus: PCI Bus structure
// @devfn: Device/function
// @where: Offset from base
// @size: Byte/word/dword
// @val: Value to be written to device
//
// Return: PCIBIOS_SUCCESSFUL on success
// PCIBIOS_DEVICE_NOT_FOUND on failure
//
    static int hv_pcifront_write_config(struct pci_bus *bus, unsigned int devfn,
    int where, int size, u32 val)
    {
    struct hv_pcibus_device *hbus =
    container_of(bus.sysdata, struct hv_pcibus_device, sysdata);
    struct hv_pci_dev *hpdev;
    hpdev = get_pcichild_wslot(hbus, devfn_to_wslot(devfn));
    if (!hpdev)
    return PCIBIOS_DEVICE_NOT_FOUND;
    _hv_pcifront_write_config(hpdev, where, size, val);
    put_pcichild(hpdev);
    return PCIBIOS_SUCCESSFUL;
    }
// PCIe operations
    static struct pci_ops hv_pcifront_ops = {
    .read  = hv_pcifront_read_config,
    .write = hv_pcifront_write_config,
    };
//
// Paravirtual backchannel
//
// Hyper-V SR-IOV provides a backchannel mechanism in software for
// communication between a VF driver and a PF driver.  These
// "configuration blocks" are similar in concept to PCI configuration space,
// but instead of doing reads and writes in 32-bit chunks through a very slow
// path, packets of up to 128 bytes can be sent or received asynchronously.
//
// Nearly every SR-IOV device contains just such a communications channel in
// hardware, so using this one in software is usually optional.  Using the
// software channel, however, allows driver implementers to leverage software
// tools that fuzz the communications channel looking for vulnerabilities.
//
// The usage model for these packets puts the responsibility for reading or
// writing on the VF driver.  The VF driver sends a read or a write packet,
// indicating which "block" is being referred to by number.
//
// If the PF driver wishes to initiate communication, it can "invalidate" one or
// more of the first 64 blocks.  This invalidation is delivered via a callback
// supplied to the VF driver by this driver.
//
// No protocol is implied, except that supplied by the PF and VF drivers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hv_read_config_compl {
    pub comp_pkt: hv_pci_compl,
    pub buf: *mut c_void,
    pub len: c_uint,
    pub bytes_returned: c_uint,
}

//
// hv_pci_read_config_compl() - Invoked when a response packet
// for a read config block operation arrives.
// @context:		Identifies the read config operation
// @resp:		The response packet itself
// @resp_packet_size:	Size in bytes of the response packet
//
    static void hv_pci_read_config_compl(void *context, struct pci_response *resp,
    int resp_packet_size)
    {
    struct hv_read_config_compl *comp = context;
    struct pci_read_block_response *read_resp =
    (struct pci_read_block_response *)resp;
    unsigned int data_len, hdr_len;
    hdr_len = offsetof(struct pci_read_block_response, bytes);
    if (resp_packet_size < hdr_len) {
    comp.comp_pkt.completion_status = -1;
    goto out;
    }
    data_len = resp_packet_size - hdr_len;
    if (data_len > 0 && read_resp.status == 0) {
    comp.bytes_returned = min(comp.len, data_len);
    memcpy(comp.buf, read_resp.bytes, comp.bytes_returned);
    } else {
    comp.bytes_returned = 0;
    }
    comp.comp_pkt.completion_status = read_resp.status;
    out:
    complete(&comp.comp_pkt.host_event);
    }
//
// hv_read_config_block() - Sends a read config block request to
// the back-end driver running in the Hyper-V parent partition.
// @pdev:		The PCI driver's representation for this device.
// @buf:		Buffer into which the config block will be copied.
// @len:		Size in bytes of buf.
// @block_id:		Identifies the config block which has been requested.
// @bytes_returned:	Size which came back from the back-end driver.
//
// Return: 0 on success, -errno on failure
//
    static int hv_read_config_block(struct pci_dev *pdev, void *buf,
    unsigned int len, unsigned int block_id,
    unsigned int *bytes_returned)
    {
    struct hv_pcibus_device *hbus =
    container_of(pdev.bus.sysdata, struct hv_pcibus_device,
    sysdata);
    struct {
    struct pci_packet pkt;
    char buf[sizeof(struct pci_read_block)];
    } pkt;
    struct hv_read_config_compl comp_pkt;
    struct pci_read_block *read_blk;
    int ret;
    if (len == 0 || len > HV_CONFIG_BLOCK_SIZE_MAX)
    return -EINVAL;
    init_completion(&comp_pkt.comp_pkt.host_event);
    comp_pkt.buf = buf;
    comp_pkt.len = len;
    memset(&pkt, 0, sizeof(pkt));
    pkt.pkt.completion_func = hv_pci_read_config_compl;
    pkt.pkt.compl_ctxt = &comp_pkt;
    read_blk = (struct pci_read_block *)pkt.buf;
    read_blk.message_type.type = PCI_READ_BLOCK;
    read_blk.wslot.slot = devfn_to_wslot(pdev.devfn);
    read_blk.block_id = block_id;
    read_blk.bytes_requested = len;
    ret = vmbus_sendpacket(hbus.hdev.channel, read_blk,
    sizeof(*read_blk), (unsigned long)&pkt.pkt,
    VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (ret)
    return ret;
    ret = wait_for_response(hbus.hdev, &comp_pkt.comp_pkt.host_event);
    if (ret)
    return ret;
    if (comp_pkt.comp_pkt.completion_status != 0 ||
    comp_pkt.bytes_returned == 0) {
    dev_err(&hbus.hdev.device,
    "Read Config Block failed: 0x%x, bytes_returned=%d\n",
    comp_pkt.comp_pkt.completion_status,
    comp_pkt.bytes_returned);
    return -EIO;
    }
// bytes_returned = comp_pkt.bytes_returned;
    return 0;
    }
//
// hv_pci_write_config_compl() - Invoked when a response packet for a write
// config block operation arrives.
// @context:		Identifies the write config operation
// @resp:		The response packet itself
// @resp_packet_size:	Size in bytes of the response packet
//
    static void hv_pci_write_config_compl(void *context, struct pci_response *resp,
    int resp_packet_size)
    {
    struct hv_pci_compl *comp_pkt = context;
    comp_pkt.completion_status = resp.status;
    complete(&comp_pkt.host_event);
    }
//
// hv_write_config_block() - Sends a write config block request to the
// back-end driver running in the Hyper-V parent partition.
// @pdev:		The PCI driver's representation for this device.
// @buf:		Buffer from which the config block will	be copied.
// @len:		Size in bytes of buf.
// @block_id:		Identifies the config block which is being written.
//
// Return: 0 on success, -errno on failure
//
    static int hv_write_config_block(struct pci_dev *pdev, void *buf,
    unsigned int len, unsigned int block_id)
    {
    struct hv_pcibus_device *hbus =
    container_of(pdev.bus.sysdata, struct hv_pcibus_device,
    sysdata);
    struct {
    struct pci_packet pkt;
    char buf[sizeof(struct pci_write_block)];
    u32 reserved;
    } pkt;
    struct hv_pci_compl comp_pkt;
    struct pci_write_block *write_blk;
    u32 pkt_size;
    int ret;
    if (len == 0 || len > HV_CONFIG_BLOCK_SIZE_MAX)
    return -EINVAL;
    init_completion(&comp_pkt.host_event);
    memset(&pkt, 0, sizeof(pkt));
    pkt.pkt.completion_func = hv_pci_write_config_compl;
    pkt.pkt.compl_ctxt = &comp_pkt;
    write_blk = (struct pci_write_block *)pkt.buf;
    write_blk.message_type.type = PCI_WRITE_BLOCK;
    write_blk.wslot.slot = devfn_to_wslot(pdev.devfn);
    write_blk.block_id = block_id;
    write_blk.byte_count = len;
    memcpy(write_blk.bytes, buf, len);
    pkt_size = offsetof(struct pci_write_block, bytes) + len;
//
// This quirk is required on some hosts shipped around 2018, because
// these hosts don't check the pkt_size correctly (new hosts have been
// fixed since early 2019). The quirk is also safe on very old hosts
// and new hosts, because, on them, what really matters is the length
// specified in write_blk->byte_count.
//
    pkt_size += sizeof(pkt.reserved);
    ret = vmbus_sendpacket(hbus.hdev.channel, write_blk, pkt_size,
    (unsigned long)&pkt.pkt, VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (ret)
    return ret;
    ret = wait_for_response(hbus.hdev, &comp_pkt.host_event);
    if (ret)
    return ret;
    if (comp_pkt.completion_status != 0) {
    dev_err(&hbus.hdev.device,
    "Write Config Block failed: 0x%x\n",
    comp_pkt.completion_status);
    return -EIO;
    }
    return 0;
    }
//
// hv_register_block_invalidate() - Invoked when a config block invalidation
// arrives from the back-end driver.
// @pdev:		The PCI driver's representation for this device.
// @context:		Identifies the device.
// @block_invalidate:	Identifies all of the blocks being invalidated.
//
// Return: 0 on success, -errno on failure
//
    static int hv_register_block_invalidate(struct pci_dev *pdev, void *context,
    void (*block_invalidate)(void *context,
    u64 block_mask))
    {
    struct hv_pcibus_device *hbus =
    container_of(pdev.bus.sysdata, struct hv_pcibus_device,
    sysdata);
    struct hv_pci_dev *hpdev;
    hpdev = get_pcichild_wslot(hbus, devfn_to_wslot(pdev.devfn));
    if (!hpdev)
    return -ENODEV;
    hpdev.block_invalidate = block_invalidate;
    hpdev.invalidate_context = context;
    put_pcichild(hpdev);
    return 0;
    }
// Interrupt management hooks
    static void hv_int_desc_free(struct hv_pci_dev *hpdev,
    struct tran_int_desc *int_desc)
    {
    struct pci_delete_interrupt *int_pkt;
    struct {
    struct pci_packet pkt;
    u8 buffer[sizeof(struct pci_delete_interrupt)];
    } ctxt;
    if (!int_desc.vector_count) {
    kfree(int_desc);
    return;
    }
    memset(&ctxt, 0, sizeof(ctxt));
    int_pkt = (struct pci_delete_interrupt *)ctxt.buffer;
    int_pkt.message_type.type =
    PCI_DELETE_INTERRUPT_MESSAGE;
    int_pkt.wslot.slot = hpdev.desc.win_slot.slot;
    int_pkt.int_desc = *int_desc;
    vmbus_sendpacket(hpdev.hbus.hdev.channel, int_pkt, sizeof(*int_pkt),
    0, VM_PKT_DATA_INBAND, 0);
    kfree(int_desc);
    }
//
// hv_msi_free() - Free the MSI.
// @domain:	The interrupt domain pointer
// @irq:	Identifies the IRQ.
//
// The Hyper-V parent partition and hypervisor are tracking the
// messages that are in use, keeping the interrupt redirection
// table up to date.  This callback sends a message that frees
// the IRT entry and related tracking nonsense.
//
#[no_mangle]
unsafe extern "C" fn hv_msi_free(domain: *mut irq_domain, irq: c_uint) {
    static void hv_msi_free(struct irq_domain *domain, unsigned int irq)
    {
    struct hv_pcibus_device *hbus;
    struct hv_pci_dev *hpdev;
    struct pci_dev *pdev;
    struct tran_int_desc *int_desc;
    struct irq_data *irq_data = irq_domain_get_irq_data(domain, irq);
    struct msi_desc *msi = irq_data_get_msi_desc(irq_data);
    pdev = msi_desc_to_pci_dev(msi);
    hbus = domain.host_data;
    int_desc = irq_data_get_irq_chip_data(irq_data);
    if (!int_desc)
    return;
    irq_data.chip_data = core::ptr::null_mut();
    hpdev = get_pcichild_wslot(hbus, devfn_to_wslot(pdev.devfn));
    if (!hpdev) {
    kfree(int_desc);
    return;
    }
    hv_int_desc_free(hpdev, int_desc);
    put_pcichild(hpdev);
    }
#[no_mangle]
unsafe extern "C" fn hv_irq_mask(data: *mut irq_data) {
    static void hv_irq_mask(struct irq_data *data)
    {
    if (data.parent_data.chip.irq_mask)
    irq_chip_mask_parent(data);
    }
#[no_mangle]
unsafe extern "C" fn hv_irq_unmask(data: *mut irq_data) {
    static void hv_irq_unmask(struct irq_data *data)
    {
    hv_arch_irq_unmask(data);
    if (data.parent_data.chip.irq_unmask)
    irq_chip_unmask_parent(data);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compose_comp_ctxt {
    pub comp_pkt: hv_pci_compl,
    pub int_desc: tran_int_desc,
}

    static void hv_pci_compose_compl(void *context, struct pci_response *resp,
    int resp_packet_size)
    {
    struct compose_comp_ctxt *comp_pkt = context;
    struct pci_create_int_response *int_resp =
    (struct pci_create_int_response *)resp;
    if (resp_packet_size < sizeof(*int_resp)) {
    comp_pkt.comp_pkt.completion_status = -1;
    goto out;
    }
    comp_pkt.comp_pkt.completion_status = resp.status;
    comp_pkt.int_desc = int_resp.int_desc;
    out:
    complete(&comp_pkt.comp_pkt.host_event);
    }
    static u32 hv_compose_msi_req_v1(
    struct pci_create_interrupt *int_pkt,
    u32 slot, u8 vector, u16 vector_count)
    {
    int_pkt.message_type.type = PCI_CREATE_INTERRUPT_MESSAGE;
    int_pkt.wslot.slot = slot;
    int_pkt.int_desc.vector = vector;
    int_pkt.int_desc.vector_count = vector_count;
    int_pkt.int_desc.delivery_mode = DELIVERY_MODE;
//
// Create MSI w/ dummy vCPU set, overwritten by subsequent retarget in
// hv_irq_unmask().
//
    int_pkt.int_desc.cpu_mask = CPU_AFFINITY_ALL;
    return sizeof(*int_pkt);
    }
//
// The vCPU selected by hv_compose_multi_msi_req_get_cpu() and
// hv_compose_msi_req_get_cpu() is a "dummy" vCPU because the final vCPU to be
// interrupted is specified later in hv_irq_unmask() and communicated to Hyper-V
// via the HVCALL_RETARGET_INTERRUPT hypercall. But the choice of dummy vCPU is
// not irrelevant because Hyper-V chooses the physical CPU to handle the
// interrupts based on the vCPU specified in message sent to the vPCI VSP in
// hv_compose_msi_msg(). Hyper-V's choice of pCPU is not visible to the guest,
// but assigning too many vPCI device interrupts to the same pCPU can cause a
// performance bottleneck. So we spread out the dummy vCPUs to influence Hyper-V
// to spread out the pCPUs that it selects.
//
// For the single-MSI and MSI-X cases, it's OK for hv_compose_msi_req_get_cpu()
// to always return the same dummy vCPU, because a second call to
// hv_compose_msi_msg() contains the "real" vCPU, causing Hyper-V to choose a
// new pCPU for the interrupt. But for the multi-MSI case, the second call to
// hv_compose_msi_msg() exits without sending a message to the vPCI VSP, so the
// original dummy vCPU is used. This dummy vCPU must be round-robin'ed so that
// the pCPUs are spread out. All interrupts for a multi-MSI device end up using
// the same pCPU, even though the vCPUs will be spread out by later calls
// to hv_irq_unmask(), but that is the best we can do now.
//
// With Hyper-V in Nov 2022, the HVCALL_RETARGET_INTERRUPT hypercall does *not
// cause Hyper-V to reselect the pCPU based on the specified vCPU. Such an
// enhancement is planned for a future version. With that enhancement, the
// dummy vCPU selection won't matter, and interrupts for the same multi-MSI
// device will be spread across multiple pCPUs.
//
// Create MSI w/ dummy vCPU set targeting just one vCPU, overwritten
// by subsequent retarget in hv_irq_unmask().
//
#[no_mangle]
unsafe extern "C" fn hv_compose_msi_req_get_cpu(affinity: *const cpumask) -> c_int {
    static int hv_compose_msi_req_get_cpu(const struct cpumask *affinity)
    {
    return cpumask_first_and(affinity, cpu_online_mask);
    }
//
// Make sure the dummy vCPU values for multi-MSI don't all point to vCPU0.
//
#[no_mangle]
unsafe extern "C" fn hv_compose_multi_msi_req_get_cpu() -> c_int {
    static int hv_compose_multi_msi_req_get_cpu(void)
    {
    static DEFINE_SPINLOCK(multi_msi_cpu_lock);
// -1 means starting with CPU 0
    let mut cpu_next: static int = -1;
    unsigned long flags;
    int cpu;
    spin_lock_irqsave(&multi_msi_cpu_lock, flags);
    cpu_next = cpumask_next_wrap(cpu_next, cpu_online_mask);
    cpu = cpu_next;
    spin_unlock_irqrestore(&multi_msi_cpu_lock, flags);
    return cpu;
    }
    static u32 hv_compose_msi_req_v2(
    struct pci_create_interrupt2 *int_pkt, int cpu,
    u32 slot, u8 vector, u16 vector_count)
    {
    int_pkt.message_type.type = PCI_CREATE_INTERRUPT_MESSAGE2;
    int_pkt.wslot.slot = slot;
    int_pkt.int_desc.vector = vector;
    int_pkt.int_desc.vector_count = vector_count;
    int_pkt.int_desc.delivery_mode = DELIVERY_MODE;
    int_pkt.int_desc.processor_array[0] =
    hv_cpu_number_to_vp_number(cpu);
    int_pkt.int_desc.processor_count = 1;
    return sizeof(*int_pkt);
    }
    static u32 hv_compose_msi_req_v3(
    struct pci_create_interrupt3 *int_pkt, int cpu,
    u32 slot, u32 vector, u16 vector_count)
    {
    int_pkt.message_type.type = PCI_CREATE_INTERRUPT_MESSAGE3;
    int_pkt.wslot.slot = slot;
    int_pkt.int_desc.vector = vector;
    int_pkt.int_desc.reserved = 0;
    int_pkt.int_desc.vector_count = vector_count;
    int_pkt.int_desc.delivery_mode = DELIVERY_MODE;
    int_pkt.int_desc.processor_array[0] =
    hv_cpu_number_to_vp_number(cpu);
    int_pkt.int_desc.processor_count = 1;
    return sizeof(*int_pkt);
    }
//
// hv_compose_msi_msg() - Supplies a valid MSI address/data
// @data:	Everything about this MSI
// @msg:	Buffer that is filled in by this function
//
// This function unpacks the IRQ looking for target CPU set, IDT
// vector and mode and sends a message to the parent partition
// asking for a mapping for that tuple in this partition.  The
// response supplies a data value and address to which that data
// should be written to trigger that interrupt.
//
#[no_mangle]
unsafe extern "C" fn hv_compose_msi_msg(data: *mut irq_data, msg: *mut msi_msg) {
    static void hv_compose_msi_msg(struct irq_data *data, struct msi_msg *msg)
    {
    struct hv_pcibus_device *hbus;
    struct vmbus_channel *channel;
    struct hv_pci_dev *hpdev;
    struct pci_bus *pbus;
    struct pci_dev *pdev;
    const struct cpumask *dest;
    struct compose_comp_ctxt comp;
    struct tran_int_desc *int_desc;
    struct msi_desc *msi_desc;
//
// vector_count should be u16: see hv_msi_desc, hv_msi_desc2
// and hv_msi_desc3. vector must be u32: see hv_msi_desc3.
//
    u16 vector_count;
    u32 vector;
    struct {
    struct pci_packet pci_pkt;
    union {
    struct pci_create_interrupt v1;
    struct pci_create_interrupt2 v2;
    struct pci_create_interrupt3 v3;
    } int_pkts;
    } __packed ctxt;
    bool multi_msi;
    u64 trans_id;
    u32 size;
    int ret;
    int cpu;
    msi_desc  = irq_data_get_msi_desc(data);
    multi_msi = !msi_desc.pci.msi_attrib.is_msix &&
    msi_desc.nvec_used > 1;
// Reuse the previous allocation
    if (data.chip_data && multi_msi) {
    int_desc = data.chip_data;
    msg.address_hi = int_desc.address >> 32;
    msg.address_lo = int_desc.address & 0xffffffff;
    msg.data = int_desc.data;
    return;
    }
    pdev = msi_desc_to_pci_dev(msi_desc);
    dest = irq_data_get_effective_affinity_mask(data);
    pbus = pdev.bus;
    hbus = container_of(pbus.sysdata, struct hv_pcibus_device, sysdata);
    channel = hbus.hdev.channel;
    hpdev = get_pcichild_wslot(hbus, devfn_to_wslot(pdev.devfn));
    if (!hpdev)
    goto return_null_message;
// Free any previous message that might have already been composed.
    if (data.chip_data && !multi_msi) {
    int_desc = data.chip_data;
    data.chip_data = core::ptr::null_mut();
    hv_int_desc_free(hpdev, int_desc);
    }
    int_desc = kzalloc_obj(*int_desc, GFP_ATOMIC);
    if (!int_desc)
    goto drop_reference;
    if (multi_msi) {
//
// If this is not the first MSI of Multi MSI, we already have
// a mapping.  Can exit early.
//
    if (msi_desc.irq != data.irq) {
    data.chip_data = int_desc;
    int_desc.address = msi_desc.msg.address_lo |
    (u64)msi_desc.msg.address_hi << 32;
    int_desc.data = msi_desc.msg.data +
    (data.irq - msi_desc.irq);
    msg.address_hi = msi_desc.msg.address_hi;
    msg.address_lo = msi_desc.msg.address_lo;
    msg.data = int_desc.data;
    put_pcichild(hpdev);
    return;
    }
//
// The vector we select here is a dummy value.  The correct
// value gets sent to the hypervisor in unmask().  This needs
// to be aligned with the count, and also not zero.  Multi-msi
// is powers of 2 up to 32, so 32 will always work here.
//
    vector = 32;
    vector_count = msi_desc.nvec_used;
    cpu = hv_compose_multi_msi_req_get_cpu();
    } else {
    vector = hv_msi_get_int_vector(data);
    vector_count = 1;
    cpu = hv_compose_msi_req_get_cpu(dest);
    }
//
// hv_compose_msi_req_v1 and v2 are for x86 only, meaning 'vector'
// can't exceed u8. Cast 'vector' down to u8 for v1/v2 explicitly
// for better readability.
//
    memset(&ctxt, 0, sizeof(ctxt));
    init_completion(&comp.comp_pkt.host_event);
    ctxt.pci_pkt.completion_func = hv_pci_compose_compl;
    ctxt.pci_pkt.compl_ctxt = &comp;
    switch (hbus.protocol_version) {
    case PCI_PROTOCOL_VERSION_1_1:
    size = hv_compose_msi_req_v1(&ctxt.int_pkts.v1,
    hpdev.desc.win_slot.slot,
    (u8)vector,
    vector_count);
    break;
    case PCI_PROTOCOL_VERSION_1_2:
    case PCI_PROTOCOL_VERSION_1_3:
    size = hv_compose_msi_req_v2(&ctxt.int_pkts.v2,
    cpu,
    hpdev.desc.win_slot.slot,
    (u8)vector,
    vector_count);
    break;
    case PCI_PROTOCOL_VERSION_1_4:
    size = hv_compose_msi_req_v3(&ctxt.int_pkts.v3,
    cpu,
    hpdev.desc.win_slot.slot,
    vector,
    vector_count);
    break;
    default:
// As we only negotiate protocol versions known to this driver,
// this path should never hit. However, this is it not a hot
// path so we print a message to aid future updates.
//
    dev_err(&hbus.hdev.device,
    "Unexpected vPCI protocol, update driver.");
    goto free_int_desc;
    }
    ret = vmbus_sendpacket_getid(hpdev.hbus.hdev.channel, &ctxt.int_pkts,
    size, (unsigned long)&ctxt.pci_pkt,
    &trans_id, VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (ret) {
    dev_err(&hbus.hdev.device,
    "Sending request for interrupt failed: 0x%x",
    comp.comp_pkt.completion_status);
    goto free_int_desc;
    }
//
// Prevents hv_pci_onchannelcallback() from running concurrently
// in the tasklet.
//
    tasklet_disable_in_atomic(&channel.callback_event);
//
// Since this function is called with IRQ locks held, can't
// do normal wait for completion; instead poll.
//
    while (!try_wait_for_completion(&comp.comp_pkt.host_event)) {
    unsigned long flags;
// 0xFFFF means an invalid PCI VENDOR ID.
    if (hv_pcifront_get_vendor_id(hpdev) == 0xFFFF) {
    dev_err_once(&hbus.hdev.device,
    "the device has gone\n");
    goto enable_tasklet;
    }
//
// Make sure that the ring buffer data structure doesn't get
// freed while we dereference the ring buffer pointer.  Test
// for the channel's onchannel_callback being NULL within a
// sched_lock critical section.  See also the inline comments
// in vmbus_reset_channel_cb().
//
    spin_lock_irqsave(&channel.sched_lock, flags);
    if (unlikely(channel.onchannel_callback == core::ptr::null_mut())) {
    spin_unlock_irqrestore(&channel.sched_lock, flags);
    goto enable_tasklet;
    }
    hv_pci_onchannelcallback(hbus);
    spin_unlock_irqrestore(&channel.sched_lock, flags);
    udelay(100);
    }
    tasklet_enable(&channel.callback_event);
    if (comp.comp_pkt.completion_status < 0) {
    dev_err(&hbus.hdev.device,
    "Request for interrupt failed: 0x%x",
    comp.comp_pkt.completion_status);
    goto free_int_desc;
    }
//
// Record the assignment so that this can be unwound later. Using
// irq_set_chip_data() here would be appropriate, but the lock it takes
// is already held.
//
// int_desc = comp.int_desc;
    data.chip_data = int_desc;
// Pass up the result.
    msg.address_hi = comp.int_desc.address >> 32;
    msg.address_lo = comp.int_desc.address & 0xffffffff;
    msg.data = comp.int_desc.data;
    put_pcichild(hpdev);
    return;
    enable_tasklet:
    tasklet_enable(&channel.callback_event);
//
// The completion packet on the stack becomes invalid after 'return';
// remove the ID from the VMbus requestor if the identifier is still
// mapped to/associated with the packet.  (The identifier could have
// been 're-used', i.e., already removed and (re-)mapped.)
//
// Cf. hv_pci_onchannelcallback().
//
    vmbus_request_addr_match(channel, trans_id, (unsigned long)&ctxt.pci_pkt);
    free_int_desc:
    kfree(int_desc);
    drop_reference:
    put_pcichild(hpdev);
    return_null_message:
    msg.address_hi = 0;
    msg.address_lo = 0;
    msg.data = 0;
    }
    static bool hv_pcie_init_dev_msi_info(struct device *dev, struct irq_domain *domain,
    struct irq_domain *real_parent, struct msi_domain_info *info)
    {
    struct irq_chip *chip = info.chip;
    if (!msi_lib_init_dev_msi_info(dev, domain, real_parent, info))
    return false;
    info.ops.msi_prepare = hv_msi_prepare;
    chip.irq_set_affinity = irq_chip_set_affinity_parent;
    chip.irq_retrigger = irq_chip_retrigger_hierarchy;
    if (IS_ENABLED(CONFIG_X86))
    chip.flags |= IRQCHIP_MOVE_DEFERRED;
    return true;
    }

    MSI_FLAG_USE_DEF_CHIP_OPS		| \
    MSI_FLAG_PCI_MSI_MASK_PARENT)

    MSI_FLAG_PCI_MSIX			| \
    MSI_FLAG_PCI_MSIX_ALLOC_DYN	| \
    MSI_GENERIC_FLAGS_MASK)
    static const struct msi_parent_ops hv_pcie_msi_parent_ops = {
    .required_flags		= HV_PCIE_MSI_FLAGS_REQUIRED,
    .supported_flags	= HV_PCIE_MSI_FLAGS_SUPPORTED,
    .bus_select_token	= DOMAIN_BUS_PCI_MSI,
    .chip_flags		= HV_MSI_CHIP_FLAGS,
    .prefix			= "HV-",
    .init_dev_msi_info	= hv_pcie_init_dev_msi_info,
    };
// HW Interrupt Chip Descriptor
    static struct irq_chip hv_msi_irq_chip = {
    .name			= "Hyper-V PCIe MSI",
    .irq_compose_msi_msg	= hv_compose_msi_msg,
    .irq_set_affinity	= irq_chip_set_affinity_parent,
    .irq_ack		= irq_chip_ack_parent,
    .irq_eoi		= irq_chip_eoi_parent,
    .irq_mask		= hv_irq_mask,
    .irq_unmask		= hv_irq_unmask,
    };
    static int hv_pcie_domain_alloc(struct irq_domain *d, unsigned int virq, unsigned int nr_irqs,
    void *arg)
    {
//
// TODO: Allocating and populating struct tran_int_desc in hv_compose_msi_msg()
// should be moved here.
//
    int ret;
    ret = irq_domain_alloc_irqs_parent(d, virq, nr_irqs, arg);
    if (ret < 0)
    return ret;
    for (int i = 0; i < nr_irqs; i++) {
    irq_domain_set_hwirq_and_chip(d, virq + i, 0, &hv_msi_irq_chip, core::ptr::null_mut());
    if (IS_ENABLED(CONFIG_X86))
    __irq_set_handler(virq + i, handle_edge_irq, 0, "edge");
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv_pcie_domain_free(d: *mut irq_domain, virq: c_uint, nr_irqs: c_uint) {
    static void hv_pcie_domain_free(struct irq_domain *d, unsigned int virq, unsigned int nr_irqs)
    {
    for (int i = 0; i < nr_irqs; i++)
    hv_msi_free(d, virq + i);
    irq_domain_free_irqs_top(d, virq, nr_irqs);
    }
    static const struct irq_domain_ops hv_pcie_domain_ops = {
    .alloc	= hv_pcie_domain_alloc,
    .free	= hv_pcie_domain_free,
    };
//
// hv_pcie_init_irq_domain() - Initialize IRQ domain
// @hbus:	The root PCI bus
//
// This function creates an IRQ domain which will be used for
// interrupts from devices that have been passed through.  These
// devices only support MSI and MSI-X, not line-based interrupts
// or simulations of line-based interrupts through PCIe's
// fabric-layer messages.  Because interrupts are remapped, we
// can support multi-message MSI here.
//
// Return: '0' on success and error value on failure
//
#[no_mangle]
unsafe extern "C" fn hv_pcie_init_irq_domain(hbus: *mut hv_pcibus_device) -> c_int {
    static int hv_pcie_init_irq_domain(struct hv_pcibus_device *hbus)
    {
    struct irq_domain_info info = {
    .fwnode		= hbus.fwnode,
    .ops		= &hv_pcie_domain_ops,
    .host_data	= hbus,
    .parent		= hv_pci_get_root_domain(),
    };
    hbus.irq_domain = msi_create_parent_irq_domain(&info, &hv_pcie_msi_parent_ops);
    if (!hbus.irq_domain) {
    dev_err(&hbus.hdev.device,
    "Failed to build an MSI IRQ domain\n");
    return -ENODEV;
    }
    dev_set_msi_domain(&hbus.bridge.dev, hbus.irq_domain);
    return 0;
    }
//
// get_bar_size() - Get the address space consumed by a BAR
// @bar_val:	Value that a BAR returned after -1 was written
// to it.
//
// This function returns the size of the BAR, rounded up to 1
// page.  It has to be rounded up because the hypervisor's page
// table entry that maps the BAR into the VM can't specify an
// offset within a page.  The invariant is that the hypervisor
// must place any BARs of smaller than page length at the
// beginning of a page.
//
// Return:	Size in bytes of the consumed MMIO space.
//
#[no_mangle]
unsafe extern "C" fn get_bar_size(bar_val: u64) -> u64 {
    static u64 get_bar_size(u64 bar_val)
    {
    return round_up((1 + ~(bar_val & PCI_BASE_ADDRESS_MEM_MASK)),
    PAGE_SIZE);
    }
//
// survey_child_resources() - Total all MMIO requirements
// @hbus:	Root PCI bus, as understood by this driver
//
#[no_mangle]
unsafe extern "C" fn survey_child_resources(hbus: *mut hv_pcibus_device) {
    static void survey_child_resources(struct hv_pcibus_device *hbus)
    {
    struct hv_pci_dev *hpdev;
    let mut bar_size: resource_size_t = 0;
    unsigned long flags;
    struct completion *event;
    u64 bar_val;
    int i;
// If nobody is waiting on the answer, don't compute it.
    event = xchg(&hbus.survey_event, core::ptr::null_mut());
    if (!event)
    return;
// If the answer has already been computed, go with it.
    if (hbus.low_mmio_space || hbus.high_mmio_space) {
    complete(event);
    return;
    }
    spin_lock_irqsave(&hbus.device_list_lock, flags);
//
// Due to an interesting quirk of the PCI spec, all memory regions
// for a child device are a power of 2 in size and aligned in memory,
// so it's sufficient to just add them up without tracking alignment.
//
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    for (i = 0; i < PCI_STD_NUM_BARS; i++) {
    if (hpdev.probed_bar[i] & PCI_BASE_ADDRESS_SPACE_IO)
    dev_err(&hbus.hdev.device,
    "There's an I/O BAR in this list!\n");
    if (hpdev.probed_bar[i] != 0) {
//
// A probed BAR has all the upper bits set that
// can be changed.
//
    bar_val = hpdev.probed_bar[i];
    if (bar_val & PCI_BASE_ADDRESS_MEM_TYPE_64)
    bar_val |=
    ((u64)hpdev.probed_bar[++i] << 32);
    else
    bar_val |= 0xffffffff00000000ULL;
    bar_size = get_bar_size(bar_val);
    if (bar_val & PCI_BASE_ADDRESS_MEM_TYPE_64)
    hbus.high_mmio_space += bar_size;
    else
    hbus.low_mmio_space += bar_size;
    }
    }
    }
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    complete(event);
    }
//
// prepopulate_bars() - Fill in BARs with defaults
// @hbus:	Root PCI bus, as understood by this driver
//
// The core PCI driver code seems much, much happier if the BARs
// for a device have values upon first scan. So fill them in.
// The algorithm below works down from large sizes to small,
// attempting to pack the assignments optimally. The assumption,
// enforced in other parts of the code, is that the beginning of
// the memory-mapped I/O space will be aligned on the largest
// BAR size.
//
#[no_mangle]
unsafe extern "C" fn prepopulate_bars(hbus: *mut hv_pcibus_device) {
    static void prepopulate_bars(struct hv_pcibus_device *hbus)
    {
    let mut high_size: resource_size_t = 0;
    let mut low_size: resource_size_t = 0;
    let mut high_base: resource_size_t = 0;
    let mut low_base: resource_size_t = 0;
    resource_size_t bar_size;
    struct hv_pci_dev *hpdev;
    unsigned long flags;
    u64 bar_val;
    u32 command;
    bool high;
    int i;
    if (hbus.low_mmio_space) {
    low_size = 1ULL << (63 - __builtin_clzll(hbus.low_mmio_space));
    low_base = hbus.low_mmio_res.start;
    }
    if (hbus.high_mmio_space) {
    high_size = 1ULL <<
    (63 - __builtin_clzll(hbus.high_mmio_space));
    high_base = hbus.high_mmio_res.start;
    }
    spin_lock_irqsave(&hbus.device_list_lock, flags);
//
// Clear the memory enable bit, in case it's already set. This occurs
// in the suspend path of hibernation, where the device is suspended,
// resumed and suspended again: see hibernation_snapshot() and
// hibernation_platform_enter().
//
// If the memory enable bit is already set, Hyper-V silently ignores
// the below BAR updates, and the related PCI device driver can not
// work, because reading from the device register(s) always returns
// 0xFFFFFFFF (PCI_ERROR_RESPONSE).
//
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    _hv_pcifront_read_config(hpdev, PCI_COMMAND, 2, &command);
    command &= ~PCI_COMMAND_MEMORY;
    _hv_pcifront_write_config(hpdev, PCI_COMMAND, 2, command);
    }
// Pick addresses for the BARs.
    do {
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    for (i = 0; i < PCI_STD_NUM_BARS; i++) {
    bar_val = hpdev.probed_bar[i];
    if (bar_val == 0)
    continue;
    high = bar_val & PCI_BASE_ADDRESS_MEM_TYPE_64;
    if (high) {
    bar_val |=
    ((u64)hpdev.probed_bar[i + 1]
    << 32);
    } else {
    bar_val |= 0xffffffffULL << 32;
    }
    bar_size = get_bar_size(bar_val);
    if (high) {
    if (high_size != bar_size) {
    i++;
    continue;
    }
    _hv_pcifront_write_config(hpdev,
    PCI_BASE_ADDRESS_0 + (4 * i),
    4,
    (u32)(high_base & 0xffffff00));
    i++;
    _hv_pcifront_write_config(hpdev,
    PCI_BASE_ADDRESS_0 + (4 * i),
    4, (u32)(high_base >> 32));
    high_base += bar_size;
    } else {
    if (low_size != bar_size)
    continue;
    _hv_pcifront_write_config(hpdev,
    PCI_BASE_ADDRESS_0 + (4 * i),
    4,
    (u32)(low_base & 0xffffff00));
    low_base += bar_size;
    }
    }
    if (high_size <= 1 && low_size <= 1) {
//
// No need to set the PCI_COMMAND_MEMORY bit as
// the core PCI driver doesn't require the bit
// to be pre-set. Actually here we intentionally
// keep the bit off so that the PCI BAR probing
// in the core PCI driver doesn't cause Hyper-V
// to unnecessarily unmap/map the virtual BARs
// from/to the physical BARs multiple times.
// This reduces the VM boot time significantly
// if the BAR sizes are huge.
//
    break;
    }
    }
    high_size >>= 1;
    low_size >>= 1;
    }  while (high_size || low_size);
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    }
//
// Assign entries in sysfs pci slot directory.
//
// Note that this function does not need to lock the children list
// because it is called from pci_devices_present_work which
// is serialized with hv_eject_device_work because they are on the
// same ordered workqueue. Therefore hbus->children list will not change
// even when pci_create_slot sleeps.
//
#[no_mangle]
unsafe extern "C" fn hv_pci_assign_slots(hbus: *mut hv_pcibus_device) {
    static void hv_pci_assign_slots(struct hv_pcibus_device *hbus)
    {
    struct hv_pci_dev *hpdev;
    char name[SLOT_NAME_SIZE];
    int slot_nr;
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    if (hpdev.pci_slot)
    continue;
    slot_nr = PCI_SLOT(wslot_to_devfn(hpdev.desc.win_slot.slot));
    snprintf(name, SLOT_NAME_SIZE, "%u", hpdev.desc.ser);
    hpdev.pci_slot = pci_create_slot(hbus.bridge.bus, slot_nr,
    name, core::ptr::null_mut());
    if (IS_ERR(hpdev.pci_slot)) {
    pr_warn("pci_create slot %s failed\n", name);
    hpdev.pci_slot = core::ptr::null_mut();
    }
    }
    }
//
// Remove entries in sysfs pci slot directory.
//
#[no_mangle]
unsafe extern "C" fn hv_pci_remove_slots(hbus: *mut hv_pcibus_device) {
    static void hv_pci_remove_slots(struct hv_pcibus_device *hbus)
    {
    struct hv_pci_dev *hpdev;
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    if (!hpdev.pci_slot)
    continue;
    pci_destroy_slot(hpdev.pci_slot);
    hpdev.pci_slot = core::ptr::null_mut();
    }
    }
//
// Set NUMA node for the devices on the bus
//
#[no_mangle]
unsafe extern "C" fn hv_pci_assign_numa_node(hbus: *mut hv_pcibus_device) {
    static void hv_pci_assign_numa_node(struct hv_pcibus_device *hbus)
    {
    struct pci_dev *dev;
    struct pci_bus *bus = hbus.bridge.bus;
    struct hv_pci_dev *hv_dev;
    list_for_each_entry(dev, &bus.devices, bus_list) {
    hv_dev = get_pcichild_wslot(hbus, devfn_to_wslot(dev.devfn));
    if (!hv_dev)
    continue;
//
// If the Hyper-V host doesn't provide a NUMA node for the
// device, default to node 0. With NUMA_NO_NODE the kernel
// may spread work across NUMA nodes, which degrades
// performance on Hyper-V.
//
    set_dev_node(&dev.dev, 0);
    if (hv_dev.desc.flags & HV_PCI_DEVICE_FLAG_NUMA_AFFINITY &&
    hv_dev.desc.virtual_numa_node < num_possible_nodes())
//
// The kernel may boot with some NUMA nodes offline
// (e.g. in a KDUMP kernel) or with NUMA disabled via
// "numa=off". In those cases, adjust the host provided
// NUMA node to a valid NUMA node used by the kernel.
//
    set_dev_node(&dev.dev,
    numa_map_to_online_node(
    hv_dev.desc.virtual_numa_node));
    put_pcichild(hv_dev);
    }
    }
//
// create_root_hv_pci_bus() - Expose a new root PCI bus
// @hbus:	Root PCI bus, as understood by this driver
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn create_root_hv_pci_bus(hbus: *mut hv_pcibus_device) -> c_int {
    static int create_root_hv_pci_bus(struct hv_pcibus_device *hbus)
    {
    int error;
    struct pci_host_bridge *bridge = hbus.bridge;
    bridge.dev.parent = &hbus.hdev.device;
    bridge.sysdata = &hbus.sysdata;
    bridge.ops = &hv_pcifront_ops;
    error = pci_scan_root_bus_bridge(bridge);
    if (error)
    return error;
    pci_lock_rescan_remove();
    hv_pci_assign_numa_node(hbus);
    pci_bus_assign_resources(bridge.bus);
    hv_pci_assign_slots(hbus);
    pci_bus_add_devices(bridge.bus);
    pci_unlock_rescan_remove();
    hbus.state = hv_pcibus_installed;
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct q_res_req_compl {
    pub host_event: completion,
    pub hpdev: *mut hv_pci_dev,
}

//
// q_resource_requirements() - Query Resource Requirements
// @context:		The completion context.
// @resp:		The response that came from the host.
// @resp_packet_size:	The size in bytes of resp.
//
// This function is invoked on completion of a Query Resource
// Requirements packet.
//
    static void q_resource_requirements(void *context, struct pci_response *resp,
    int resp_packet_size)
    {
    struct q_res_req_compl *completion = context;
    struct pci_q_res_req_response *q_res_req =
    (struct pci_q_res_req_response *)resp;
    s32 status;
    int i;
    status = (resp_packet_size < sizeof(*q_res_req)) ? -1 : resp.status;
    if (status < 0) {
    dev_err(&completion.hpdev.hbus.hdev.device,
    "query resource requirements failed: %x\n",
    status);
    } else {
    for (i = 0; i < PCI_STD_NUM_BARS; i++) {
    completion.hpdev.probed_bar[i] =
    q_res_req.probed_bar[i];
    }
    }
    complete(&completion.host_event);
    }
//
// new_pcichild_device() - Create a new child device
// @hbus:	The internal struct tracking this root PCI bus.
// @desc:	The information supplied so far from the host
// about the device.
//
// This function creates the tracking structure for a new child
// device and kicks off the process of figuring out what it is.
//
// Return: Pointer to the new tracking struct
//
    static struct hv_pci_dev *new_pcichild_device(struct hv_pcibus_device *hbus,
    struct hv_pcidev_description *desc)
    {
    struct hv_pci_dev *hpdev;
    struct pci_child_message *res_req;
    struct q_res_req_compl comp_pkt;
    struct {
    struct pci_packet init_packet;
    u8 buffer[sizeof(struct pci_child_message)];
    } pkt;
    unsigned long flags;
    int ret;
    hpdev = kzalloc_obj(*hpdev);
    if (!hpdev)
    return core::ptr::null_mut();
    hpdev.hbus = hbus;
    memset(&pkt, 0, sizeof(pkt));
    init_completion(&comp_pkt.host_event);
    comp_pkt.hpdev = hpdev;
    pkt.init_packet.compl_ctxt = &comp_pkt;
    pkt.init_packet.completion_func = q_resource_requirements;
    res_req = (struct pci_child_message *)pkt.buffer;
    res_req.message_type.type = PCI_QUERY_RESOURCE_REQUIREMENTS;
    res_req.wslot.slot = desc.win_slot.slot;
    ret = vmbus_sendpacket(hbus.hdev.channel, res_req,
    sizeof(struct pci_child_message),
    (unsigned long)&pkt.init_packet,
    VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (ret)
    goto error;
    if (wait_for_response(hbus.hdev, &comp_pkt.host_event))
    goto error;
    hpdev.desc = *desc;
    refcount_set(&hpdev.refs, 1);
    get_pcichild(hpdev);
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    list_add_tail(&hpdev.list_entry, &hbus.children);
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    return hpdev;
    error:
    kfree(hpdev);
    return core::ptr::null_mut();
    }
//
// get_pcichild_wslot() - Find device from slot
// @hbus:	Root PCI bus, as understood by this driver
// @wslot:	Location on the bus
//
// This function looks up a PCI device and returns the internal
// representation of it.  It acquires a reference on it, so that
// the device won't be deleted while somebody is using it.  The
// caller is responsible for calling put_pcichild() to release
// this reference.
//
// Return:	Internal representation of a PCI device
//
    static struct hv_pci_dev *get_pcichild_wslot(struct hv_pcibus_device *hbus,
    u32 wslot)
    {
    unsigned long flags;
    struct hv_pci_dev *iter, *hpdev = core::ptr::null_mut();
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    list_for_each_entry(iter, &hbus.children, list_entry) {
    if (iter.desc.win_slot.slot == wslot) {
    hpdev = iter;
    get_pcichild(hpdev);
    break;
    }
    }
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    return hpdev;
    }
//
// pci_devices_present_work() - Handle new list of child devices
// @work:	Work struct embedded in struct hv_dr_work
//
// "Bus Relations" is the Windows term for "children of this
// bus."  The terminology is preserved here for people trying to
// debug the interaction between Hyper-V and Linux.  This
// function is called when the parent partition reports a list
// of functions that should be observed under this PCI Express
// port (bus).
//
// This function updates the list, and must tolerate being
// called multiple times with the same information.  The typical
// number of child devices is one, with very atypical cases
// involving three or four, so the algorithms used here can be
// simple and inefficient.
//
// It must also treat the omission of a previously observed device as
// notification that the device no longer exists.
//
// Note that this function is serialized with hv_eject_device_work(),
// because both are pushed to the ordered workqueue hbus->wq.
//
#[no_mangle]
unsafe extern "C" fn pci_devices_present_work(work: *mut work_struct) {
    static void pci_devices_present_work(struct work_struct *work)
    {
    u32 child_no;
    bool found;
    struct hv_pcidev_description *new_desc;
    struct hv_pci_dev *hpdev;
    struct hv_pcibus_device *hbus;
    struct list_head removed;
    struct hv_dr_work *dr_wrk;
    struct hv_dr_state *dr = core::ptr::null_mut();
    unsigned long flags;
    dr_wrk = container_of(work, struct hv_dr_work, wrk);
    hbus = dr_wrk.bus;
    kfree(dr_wrk);
    INIT_LIST_HEAD(&removed);
// Pull this off the queue and process it if it was the last one.
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    while (!list_empty(&hbus.dr_list)) {
    dr = list_first_entry(&hbus.dr_list, struct hv_dr_state,
    list_entry);
    list_del(&dr.list_entry);
// Throw this away if the list still has stuff in it.
    if (!list_empty(&hbus.dr_list)) {
    kfree(dr);
    continue;
    }
    }
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    if (!dr)
    return;
    mutex_lock(&hbus.state_lock);
// First, mark all existing children as reported missing.
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    hpdev.reported_missing = true;
    }
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
// Next, add back any reported devices.
    for (child_no = 0; child_no < dr.device_count; child_no++) {
    found = false;
    new_desc = &dr.func[child_no];
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    if ((hpdev.desc.win_slot.slot == new_desc.win_slot.slot) &&
    (hpdev.desc.v_id == new_desc.v_id) &&
    (hpdev.desc.d_id == new_desc.d_id) &&
    (hpdev.desc.ser == new_desc.ser)) {
    hpdev.reported_missing = false;
    found = true;
    }
    }
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    if (!found) {
    hpdev = new_pcichild_device(hbus, new_desc);
    if (!hpdev)
    dev_err(&hbus.hdev.device,
    "couldn't record a child device.\n");
    }
    }
// Move missing children to a list on the stack.
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    do {
    found = false;
    list_for_each_entry(hpdev, &hbus.children, list_entry) {
    if (hpdev.reported_missing) {
    found = true;
    put_pcichild(hpdev);
    list_move_tail(&hpdev.list_entry, &removed);
    break;
    }
    }
    } while (found);
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
// Delete everything that should no longer exist.
    while (!list_empty(&removed)) {
    hpdev = list_first_entry(&removed, struct hv_pci_dev,
    list_entry);
    list_del(&hpdev.list_entry);
    if (hpdev.pci_slot)
    pci_destroy_slot(hpdev.pci_slot);
    put_pcichild(hpdev);
    }
    switch (hbus.state) {
    case hv_pcibus_installed:
//
// Tell the core to rescan bus
// because there may have been changes.
//
    pci_lock_rescan_remove();
    pci_scan_child_bus(hbus.bridge.bus);
    hv_pci_assign_numa_node(hbus);
    hv_pci_assign_slots(hbus);
    pci_unlock_rescan_remove();
    break;
    case hv_pcibus_init:
    case hv_pcibus_probed:
    survey_child_resources(hbus);
    break;
    default:
    break;
    }
    mutex_unlock(&hbus.state_lock);
    kfree(dr);
    }
//
// hv_pci_start_relations_work() - Queue work to start device discovery
// @hbus:	Root PCI bus, as understood by this driver
// @dr:		The list of children returned from host
//
// Return:  0 on success, -errno on failure
//
    static int hv_pci_start_relations_work(struct hv_pcibus_device *hbus,
    struct hv_dr_state *dr)
    {
    struct hv_dr_work *dr_wrk;
    unsigned long flags;
    bool pending_dr;
    if (hbus.state == hv_pcibus_removing) {
    dev_info(&hbus.hdev.device,
    "PCI VMBus BUS_RELATIONS: ignored\n");
    return -ENOENT;
    }
    dr_wrk = kzalloc_obj(*dr_wrk, GFP_NOWAIT);
    if (!dr_wrk)
    return -ENOMEM;
    INIT_WORK(&dr_wrk.wrk, pci_devices_present_work);
    dr_wrk.bus = hbus;
    spin_lock_irqsave(&hbus.device_list_lock, flags);
//
// If pending_dr is true, we have already queued a work,
// which will see the new dr. Otherwise, we need to
// queue a new work.
//
    pending_dr = !list_empty(&hbus.dr_list);
    list_add_tail(&dr.list_entry, &hbus.dr_list);
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    if (pending_dr)
    kfree(dr_wrk);
    else
    queue_work(hbus.wq, &dr_wrk.wrk);
    return 0;
    }
//
// hv_pci_devices_present() - Handle list of new children
// @hbus:      Root PCI bus, as understood by this driver
// @relations: Packet from host listing children
//
// Process a new list of devices on the bus. The list of devices is
// discovered by VSP and sent to us via VSP message PCI_BUS_RELATIONS,
// whenever a new list of devices for this bus appears.
//
    static void hv_pci_devices_present(struct hv_pcibus_device *hbus,
    struct pci_bus_relations *relations)
    {
    struct hv_dr_state *dr;
    int i;
    dr = kzalloc_flex(*dr, func, relations.device_count, GFP_NOWAIT);
    if (!dr)
    return;
    dr.device_count = relations.device_count;
    for (i = 0; i < dr.device_count; i++) {
    dr.func[i].v_id = relations.func[i].v_id;
    dr.func[i].d_id = relations.func[i].d_id;
    dr.func[i].rev = relations.func[i].rev;
    dr.func[i].prog_intf = relations.func[i].prog_intf;
    dr.func[i].subclass = relations.func[i].subclass;
    dr.func[i].base_class = relations.func[i].base_class;
    dr.func[i].subsystem_id = relations.func[i].subsystem_id;
    dr.func[i].win_slot = relations.func[i].win_slot;
    dr.func[i].ser = relations.func[i].ser;
    }
    if (hv_pci_start_relations_work(hbus, dr))
    kfree(dr);
    }
//
// hv_pci_devices_present2() - Handle list of new children
// @hbus:	Root PCI bus, as understood by this driver
// @relations:	Packet from host listing children
//
// This function is the v2 version of hv_pci_devices_present()
//
    static void hv_pci_devices_present2(struct hv_pcibus_device *hbus,
    struct pci_bus_relations2 *relations)
    {
    struct hv_dr_state *dr;
    int i;
    dr = kzalloc_flex(*dr, func, relations.device_count, GFP_NOWAIT);
    if (!dr)
    return;
    dr.device_count = relations.device_count;
    for (i = 0; i < dr.device_count; i++) {
    dr.func[i].v_id = relations.func[i].v_id;
    dr.func[i].d_id = relations.func[i].d_id;
    dr.func[i].rev = relations.func[i].rev;
    dr.func[i].prog_intf = relations.func[i].prog_intf;
    dr.func[i].subclass = relations.func[i].subclass;
    dr.func[i].base_class = relations.func[i].base_class;
    dr.func[i].subsystem_id = relations.func[i].subsystem_id;
    dr.func[i].win_slot = relations.func[i].win_slot;
    dr.func[i].ser = relations.func[i].ser;
    dr.func[i].flags = relations.func[i].flags;
    dr.func[i].virtual_numa_node =
    relations.func[i].virtual_numa_node;
    }
    if (hv_pci_start_relations_work(hbus, dr))
    kfree(dr);
    }
//
// hv_eject_device_work() - Asynchronously handles ejection
// @work:	Work struct embedded in internal device struct
//
// This function handles ejecting a device.  Windows will
// attempt to gracefully eject a device, waiting 60 seconds to
// hear back from the guest OS that this completed successfully.
// If this timer expires, the device will be forcibly removed.
//
#[no_mangle]
unsafe extern "C" fn hv_eject_device_work(work: *mut work_struct) {
    static void hv_eject_device_work(struct work_struct *work)
    {
    struct pci_eject_response *ejct_pkt;
    struct hv_pcibus_device *hbus;
    struct hv_pci_dev *hpdev;
    struct pci_dev *pdev;
    unsigned long flags;
    int wslot;
    struct {
    struct pci_packet pkt;
    u8 buffer[sizeof(struct pci_eject_response)];
    } ctxt;
    hpdev = container_of(work, struct hv_pci_dev, wrk);
    hbus = hpdev.hbus;
    mutex_lock(&hbus.state_lock);
//
// Ejection can come before or after the PCI bus has been set up, so
// attempt to find it and tear down the bus state, if it exists.  This
// must be done without constructs like pci_domain_nr(hbus->bridge->bus)
// because hbus->bridge->bus may not exist yet.
//
    wslot = wslot_to_devfn(hpdev.desc.win_slot.slot);
    pdev = pci_get_domain_bus_and_slot(hbus.bridge.domain_nr, 0, wslot);
    if (pdev) {
    pci_lock_rescan_remove();
    pci_stop_and_remove_bus_device(pdev);
    pci_dev_put(pdev);
    pci_unlock_rescan_remove();
    }
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    list_del(&hpdev.list_entry);
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
    if (hpdev.pci_slot)
    pci_destroy_slot(hpdev.pci_slot);
    memset(&ctxt, 0, sizeof(ctxt));
    ejct_pkt = (struct pci_eject_response *)ctxt.buffer;
    ejct_pkt.message_type.type = PCI_EJECTION_COMPLETE;
    ejct_pkt.wslot.slot = hpdev.desc.win_slot.slot;
    vmbus_sendpacket(hbus.hdev.channel, ejct_pkt,
    sizeof(*ejct_pkt), 0,
    VM_PKT_DATA_INBAND, 0);
// For the get_pcichild() in hv_pci_eject_device()
    put_pcichild(hpdev);
// For the two refs got in new_pcichild_device()
    put_pcichild(hpdev);
    put_pcichild(hpdev);
// hpdev has been freed. Do not use it any more.
    mutex_unlock(&hbus.state_lock);
    }
//
// hv_pci_eject_device() - Handles device ejection
// @hpdev:	Internal device tracking struct
//
// This function is invoked when an ejection packet arrives.  It
// just schedules work so that we don't re-enter the packet
// delivery code handling the ejection.
//
#[no_mangle]
unsafe extern "C" fn hv_pci_eject_device(hpdev: *mut hv_pci_dev) {
    static void hv_pci_eject_device(struct hv_pci_dev *hpdev)
    {
    struct hv_pcibus_device *hbus = hpdev.hbus;
    struct hv_device *hdev = hbus.hdev;
    if (hbus.state == hv_pcibus_removing) {
    dev_info(&hdev.device, "PCI VMBus EJECT: ignored\n");
    return;
    }
    get_pcichild(hpdev);
    INIT_WORK(&hpdev.wrk, hv_eject_device_work);
    queue_work(hbus.wq, &hpdev.wrk);
    }
//
// hv_pci_onchannelcallback() - Handles incoming packets
// @context:	Internal bus tracking struct
//
// This function is invoked whenever the host sends a packet to
// this channel (which is private to this root PCI bus).
//
#[no_mangle]
unsafe extern "C" fn hv_pci_onchannelcallback(context: *mut c_void) {
    static void hv_pci_onchannelcallback(void *context)
    {
    let mut packet_size: c_int = 0x100;
    int ret;
    struct hv_pcibus_device *hbus = context;
    struct vmbus_channel *chan = hbus.hdev.channel;
    u32 bytes_recvd;
    u64 req_id, req_addr;
    struct vmpacket_descriptor *desc;
    unsigned char *buffer;
    let mut bufferlen: c_int = packet_size;
    struct pci_packet *comp_packet;
    struct pci_response *response;
    struct pci_incoming_message *new_message;
    struct pci_bus_relations *bus_rel;
    struct pci_bus_relations2 *bus_rel2;
    struct pci_dev_inval_block *inval;
    struct pci_dev_incoming *dev_message;
    struct hv_pci_dev *hpdev;
    unsigned long flags;
    buffer = kmalloc(bufferlen, GFP_ATOMIC);
    if (!buffer)
    return;
    while (1) {
    ret = vmbus_recvpacket_raw(chan, buffer, bufferlen,
    &bytes_recvd, &req_id);
    if (ret == -ENOBUFS) {
    kfree(buffer);
// Handle large packet
    bufferlen = bytes_recvd;
    buffer = kmalloc(bytes_recvd, GFP_ATOMIC);
    if (!buffer)
    return;
    continue;
    }
// Zero length indicates there are no more packets.
    if (ret || !bytes_recvd)
    break;
//
// All incoming packets must be at least as large as a
// response.
//
    if (bytes_recvd <= sizeof(struct pci_response))
    continue;
    desc = (struct vmpacket_descriptor *)buffer;
    switch (desc.type) {
    case VM_PKT_COMP:
    lock_requestor(chan, flags);
    req_addr = __vmbus_request_addr_match(chan, req_id,
    VMBUS_RQST_ADDR_ANY);
    if (req_addr == VMBUS_RQST_ERROR) {
    unlock_requestor(chan, flags);
    dev_err(&hbus.hdev.device,
    "Invalid transaction ID %llx\n",
    req_id);
    break;
    }
    comp_packet = (struct pci_packet *)req_addr;
    response = (struct pci_response *)buffer;
//
// Call ->completion_func() within the critical section to make
// sure that the packet pointer is still valid during the call:
// here 'valid' means that there's a task still waiting for the
// completion, and that the packet data is still on the waiting
// task's stack.  Cf. hv_compose_msi_msg().
//
    comp_packet.completion_func(comp_packet.compl_ctxt,
    response,
    bytes_recvd);
    unlock_requestor(chan, flags);
    break;
    case VM_PKT_DATA_INBAND:
    new_message = (struct pci_incoming_message *)buffer;
    switch (new_message.message_type.type) {
    case PCI_BUS_RELATIONS:
    bus_rel = (struct pci_bus_relations *)buffer;
    if (bytes_recvd < sizeof(*bus_rel) ||
    bytes_recvd <
    struct_size(bus_rel, func,
    bus_rel.device_count)) {
    dev_err(&hbus.hdev.device,
    "bus relations too small\n");
    break;
    }
    hv_pci_devices_present(hbus, bus_rel);
    break;
    case PCI_BUS_RELATIONS2:
    bus_rel2 = (struct pci_bus_relations2 *)buffer;
    if (bytes_recvd < sizeof(*bus_rel2) ||
    bytes_recvd <
    struct_size(bus_rel2, func,
    bus_rel2.device_count)) {
    dev_err(&hbus.hdev.device,
    "bus relations v2 too small\n");
    break;
    }
    hv_pci_devices_present2(hbus, bus_rel2);
    break;
    case PCI_EJECT:
    dev_message = (struct pci_dev_incoming *)buffer;
    if (bytes_recvd < sizeof(*dev_message)) {
    dev_err(&hbus.hdev.device,
    "eject message too small\n");
    break;
    }
    hpdev = get_pcichild_wslot(hbus,
    dev_message.wslot.slot);
    if (hpdev) {
    hv_pci_eject_device(hpdev);
    put_pcichild(hpdev);
    }
    break;
    case PCI_INVALIDATE_BLOCK:
    inval = (struct pci_dev_inval_block *)buffer;
    if (bytes_recvd < sizeof(*inval)) {
    dev_err(&hbus.hdev.device,
    "invalidate message too small\n");
    break;
    }
    hpdev = get_pcichild_wslot(hbus,
    inval.wslot.slot);
    if (hpdev) {
    if (hpdev.block_invalidate) {
    hpdev.block_invalidate(
    hpdev.invalidate_context,
    inval.block_mask);
    }
    put_pcichild(hpdev);
    }
    break;
    default:
    dev_warn(&hbus.hdev.device,
    "Unimplemented protocol message %x\n",
    new_message.message_type.type);
    break;
    }
    break;
    default:
    dev_err(&hbus.hdev.device,
    "unhandled packet type %d, tid %llx len %d\n",
    desc.type, req_id, bytes_recvd);
    break;
    }
    }
    kfree(buffer);
    }
//
// hv_pci_protocol_negotiation() - Set up protocol
// @hdev:		VMBus's tracking struct for this root PCI bus.
// @version:		Array of supported channel protocol versions in
// the order of probing - highest go first.
// @num_version:	Number of elements in the version array.
//
// This driver is intended to support running on Windows 10
// (server) and later versions. It will not run on earlier
// versions, as they assume that many of the operations which
// Linux needs accomplished with a spinlock held were done via
// asynchronous messaging via VMBus.  Windows 10 increases the
// surface area of PCI emulation so that these actions can take
// place by suspending a virtual processor for their duration.
//
// This function negotiates the channel protocol version,
// failing if the host doesn't support the necessary protocol
// level.
//
    static int hv_pci_protocol_negotiation(struct hv_device *hdev,
    enum pci_protocol_version_t version[],
    int num_version)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    struct pci_version_request *version_req;
    struct hv_pci_compl comp_pkt;
    struct pci_packet *pkt;
    int ret;
    int i;
//
// Initiate the handshake with the host and negotiate
// a version that the host can support. We start with the
// highest version number and go down if the host cannot
// support it.
//
    pkt = kzalloc(sizeof(*pkt) + sizeof(*version_req), GFP_KERNEL);
    if (!pkt)
    return -ENOMEM;
    init_completion(&comp_pkt.host_event);
    pkt.completion_func = hv_pci_generic_compl;
    pkt.compl_ctxt = &comp_pkt;
    version_req = (struct pci_version_request *)(pkt + 1);
    version_req.message_type.type = PCI_QUERY_PROTOCOL_VERSION;
    for (i = 0; i < num_version; i++) {
    version_req.protocol_version = version[i];
    ret = vmbus_sendpacket(hdev.channel, version_req,
    sizeof(struct pci_version_request),
    (unsigned long)pkt, VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (!ret)
    ret = wait_for_response(hdev, &comp_pkt.host_event);
    if (ret) {
    dev_err(&hdev.device,
    "PCI Pass-through VSP failed to request version: %d",
    ret);
    goto exit;
    }
    if (comp_pkt.completion_status >= 0) {
    hbus.protocol_version = version[i];
    dev_info(&hdev.device,
    "PCI VMBus probing: Using version %#x\n",
    hbus.protocol_version);
    goto exit;
    }
    if (comp_pkt.completion_status != STATUS_REVISION_MISMATCH) {
    dev_err(&hdev.device,
    "PCI Pass-through VSP failed version request: %#x",
    comp_pkt.completion_status);
    ret = -EPROTO;
    goto exit;
    }
    reinit_completion(&comp_pkt.host_event);
    }
    dev_err(&hdev.device,
    "PCI pass-through VSP failed to find supported version");
    ret = -EPROTO;
    exit:
    kfree(pkt);
    return ret;
    }
//
// hv_pci_free_bridge_windows() - Release memory regions for the
// bus
// @hbus:	Root PCI bus, as understood by this driver
//
#[no_mangle]
unsafe extern "C" fn hv_pci_free_bridge_windows(hbus: *mut hv_pcibus_device) {
    static void hv_pci_free_bridge_windows(struct hv_pcibus_device *hbus)
    {
//
// Set the resources back to the way they looked when they
// were allocated by setting IORESOURCE_BUSY again.
//
    if (hbus.low_mmio_space && hbus.low_mmio_res) {
    hbus.low_mmio_res.flags |= IORESOURCE_BUSY;
    vmbus_free_mmio(hbus.low_mmio_res.start,
    resource_size(hbus.low_mmio_res));
    }
    if (hbus.high_mmio_space && hbus.high_mmio_res) {
    hbus.high_mmio_res.flags |= IORESOURCE_BUSY;
    vmbus_free_mmio(hbus.high_mmio_res.start,
    resource_size(hbus.high_mmio_res));
    }
    }
//
// hv_pci_allocate_bridge_windows() - Allocate memory regions
// for the bus
// @hbus:	Root PCI bus, as understood by this driver
//
// This function calls vmbus_allocate_mmio(), which is itself a
// bit of a compromise.  Ideally, we might change the pnp layer
// in the kernel such that it comprehends either PCI devices
// which are "grandchildren of ACPI," with some intermediate bus
// node (in this case, VMBus) or change it such that it
// understands VMBus.  The pnp layer, however, has been declared
// deprecated, and not subject to change.
//
// The workaround, implemented here, is to ask VMBus to allocate
// MMIO space for this bus.  VMBus itself knows which ranges are
// appropriate by looking at its own ACPI objects.  Then, after
// these ranges are claimed, they're modified to look like they
// would have looked if the ACPI and pnp code had allocated
// bridge windows.  These descriptors have to exist in this form
// in order to satisfy the code which will get invoked when the
// endpoint PCI function driver calls request_mem_region() or
// request_mem_region_exclusive().
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn hv_pci_allocate_bridge_windows(hbus: *mut hv_pcibus_device) -> c_int {
    static int hv_pci_allocate_bridge_windows(struct hv_pcibus_device *hbus)
    {
    resource_size_t align;
    int ret;
    if (hbus.low_mmio_space) {
    align = 1ULL << (63 - __builtin_clzll(hbus.low_mmio_space));
    ret = vmbus_allocate_mmio(&hbus.low_mmio_res, hbus.hdev, 0,
    (u64)(u32)0xffffffff,
    hbus.low_mmio_space,
    align, false);
    if (ret) {
    dev_err(&hbus.hdev.device,
    "Need %#llx of low MMIO space. Consider reconfiguring the VM.\n",
    hbus.low_mmio_space);
    return ret;
    }
// Modify this resource to become a bridge window.
    hbus.low_mmio_res.flags |= IORESOURCE_WINDOW;
    hbus.low_mmio_res.flags &= ~IORESOURCE_BUSY;
    pci_add_resource(&hbus.bridge.windows, hbus.low_mmio_res);
    }
    if (hbus.high_mmio_space) {
    align = 1ULL << (63 - __builtin_clzll(hbus.high_mmio_space));
    ret = vmbus_allocate_mmio(&hbus.high_mmio_res, hbus.hdev,
    0x100000000, -1,
    hbus.high_mmio_space, align,
    false);
    if (ret) {
    dev_err(&hbus.hdev.device,
    "Need %#llx of high MMIO space. Consider reconfiguring the VM.\n",
    hbus.high_mmio_space);
    goto release_low_mmio;
    }
// Modify this resource to become a bridge window.
    hbus.high_mmio_res.flags |= IORESOURCE_WINDOW;
    hbus.high_mmio_res.flags &= ~IORESOURCE_BUSY;
    pci_add_resource(&hbus.bridge.windows, hbus.high_mmio_res);
    }
    return 0;
    release_low_mmio:
    if (hbus.low_mmio_res) {
    vmbus_free_mmio(hbus.low_mmio_res.start,
    resource_size(hbus.low_mmio_res));
    }
    return ret;
    }
//
// hv_allocate_config_window() - Find MMIO space for PCI Config
// @hbus:	Root PCI bus, as understood by this driver
//
// This function claims memory-mapped I/O space for accessing
// configuration space for the functions on this bus.
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn hv_allocate_config_window(hbus: *mut hv_pcibus_device) -> c_int {
    static int hv_allocate_config_window(struct hv_pcibus_device *hbus)
    {
    int ret;
//
// Set up a region of MMIO space to use for accessing configuration
// space.
//
    ret = vmbus_allocate_mmio(&hbus.mem_config, hbus.hdev, 0, -1,
    PCI_CONFIG_MMIO_LENGTH, 0x1000, false);
    if (ret)
    return ret;
//
// vmbus_allocate_mmio() gets used for allocating both device endpoint
// resource claims (those which cannot be overlapped) and the ranges
// which are valid for the children of this bus, which are intended
// to be overlapped by those children.  Set the flag on this claim
// meaning that this region can't be overlapped.
//
    hbus.mem_config.flags |= IORESOURCE_BUSY;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv_free_config_window(hbus: *mut hv_pcibus_device) {
    static void hv_free_config_window(struct hv_pcibus_device *hbus)
    {
    vmbus_free_mmio(hbus.mem_config.start, PCI_CONFIG_MMIO_LENGTH);
    }
    static int hv_pci_bus_exit(struct hv_device *hdev, bool keep_devs);
//
// hv_pci_enter_d0() - Bring the "bus" into the D0 power state
// @hdev:	VMBus's tracking struct for this root PCI bus
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn hv_pci_enter_d0(hdev: *mut hv_device) -> c_int {
    static int hv_pci_enter_d0(struct hv_device *hdev)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    struct pci_bus_d0_entry *d0_entry;
    struct hv_pci_compl comp_pkt;
    struct pci_packet *pkt;
    let mut retry: bool = true;
    int ret;
    enter_d0_retry:
//
// Tell the host that the bus is ready to use, and moved into the
// powered-on state.  This includes telling the host which region
// of memory-mapped I/O space has been chosen for configuration space
// access.
//
    pkt = kzalloc(sizeof(*pkt) + sizeof(*d0_entry), GFP_KERNEL);
    if (!pkt)
    return -ENOMEM;
    init_completion(&comp_pkt.host_event);
    pkt.completion_func = hv_pci_generic_compl;
    pkt.compl_ctxt = &comp_pkt;
    d0_entry = (struct pci_bus_d0_entry *)(pkt + 1);
    d0_entry.message_type.type = PCI_BUS_D0ENTRY;
    d0_entry.mmio_base = hbus.mem_config.start;
    ret = vmbus_sendpacket(hdev.channel, d0_entry, sizeof(*d0_entry),
    (unsigned long)pkt, VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (!ret)
    ret = wait_for_response(hdev, &comp_pkt.host_event);
    if (ret)
    goto exit;
//
// In certain case (Kdump) the pci device of interest was
// not cleanly shut down and resource is still held on host
// side, the host could return invalid device status.
// We need to explicitly request host to release the resource
// and try to enter D0 again.
//
    if (comp_pkt.completion_status < 0 && retry) {
    retry = false;
    dev_err(&hdev.device, "Retrying D0 Entry\n");
//
// Hv_pci_bus_exit() calls hv_send_resource_released()
// to free up resources of its child devices.
// In the kdump kernel we need to set the
// wslot_res_allocated to 255 so it scans all child
// devices to release resources allocated in the
// normal kernel before panic happened.
//
    hbus.wslot_res_allocated = 255;
    ret = hv_pci_bus_exit(hdev, true);
    if (ret == 0) {
    kfree(pkt);
    goto enter_d0_retry;
    }
    dev_err(&hdev.device,
    "Retrying D0 failed with ret %d\n", ret);
    }
    if (comp_pkt.completion_status < 0) {
    dev_err(&hdev.device,
    "PCI Pass-through VSP failed D0 Entry with status %x\n",
    comp_pkt.completion_status);
    ret = -EPROTO;
    goto exit;
    }
    ret = 0;
    exit:
    kfree(pkt);
    return ret;
    }
//
// hv_pci_query_relations() - Ask host to send list of child
// devices
// @hdev:	VMBus's tracking struct for this root PCI bus
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn hv_pci_query_relations(hdev: *mut hv_device) -> c_int {
    static int hv_pci_query_relations(struct hv_device *hdev)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    struct pci_message message;
    struct completion comp;
    int ret;
// Ask the host to send along the list of child devices
    init_completion(&comp);
    if (cmpxchg(&hbus.survey_event, core::ptr::null_mut(), &comp))
    return -ENOTEMPTY;
    memset(&message, 0, sizeof(message));
    message.type = PCI_QUERY_BUS_RELATIONS;
    ret = vmbus_sendpacket(hdev.channel, &message, sizeof(message),
    0, VM_PKT_DATA_INBAND, 0);
    if (!ret)
    ret = wait_for_response(hdev, &comp);
//
// In the case of fast device addition/removal, it's possible that
// vmbus_sendpacket() or wait_for_response() returns -ENODEV but we
// already got a PCI_BUS_RELATIONS* message from the host and the
// channel callback already scheduled a work to hbus->wq, which can be
// running pci_devices_present_work() -> survey_child_resources() ->
// complete(&hbus->survey_event), even after hv_pci_query_relations()
// exits and the stack variable 'comp' is no longer valid; as a result,
// a hang or a page fault may happen when the complete() calls
// raw_spin_lock_irqsave(). Flush hbus->wq before we exit from
// hv_pci_query_relations() to avoid the issues. Note: if 'ret' is
// -ENODEV, there can't be any more work item scheduled to hbus->wq
// after the flush_workqueue(): see vmbus_onoffer_rescind() ->
// vmbus_reset_channel_cb(), vmbus_rescind_cleanup() ->
// channel->rescind = true.
//
    flush_workqueue(hbus.wq);
    return ret;
    }
//
// hv_send_resources_allocated() - Report local resource choices
// @hdev:	VMBus's tracking struct for this root PCI bus
//
// The host OS is expecting to be sent a request as a message
// which contains all the resources that the device will use.
// The response contains those same resources, "translated"
// which is to say, the values which should be used by the
// hardware, when it delivers an interrupt.  (MMIO resources are
// used in local terms.)  This is nice for Windows, and lines up
// with the FDO/PDO split, which doesn't exist in Linux.  Linux
// is deeply expecting to scan an emulated PCI configuration
// space.  So this message is sent here only to drive the state
// machine on the host forward.
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn hv_send_resources_allocated(hdev: *mut hv_device) -> c_int {
    static int hv_send_resources_allocated(struct hv_device *hdev)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    struct pci_resources_assigned *res_assigned;
    struct pci_resources_assigned2 *res_assigned2;
    struct hv_pci_compl comp_pkt;
    struct hv_pci_dev *hpdev;
    struct pci_packet *pkt;
    size_t size_res;
    int wslot;
    int ret;
    size_res = (hbus.protocol_version < PCI_PROTOCOL_VERSION_1_2)
    ? sizeof(*res_assigned) : sizeof(*res_assigned2);
    pkt = kmalloc(sizeof(*pkt) + size_res, GFP_KERNEL);
    if (!pkt)
    return -ENOMEM;
    ret = 0;
    for (wslot = 0; wslot < 256; wslot++) {
    hpdev = get_pcichild_wslot(hbus, wslot);
    if (!hpdev)
    continue;
    memset(pkt, 0, sizeof(*pkt) + size_res);
    init_completion(&comp_pkt.host_event);
    pkt.completion_func = hv_pci_generic_compl;
    pkt.compl_ctxt = &comp_pkt;
    if (hbus.protocol_version < PCI_PROTOCOL_VERSION_1_2) {
    res_assigned =
    (struct pci_resources_assigned *)(pkt + 1);
    res_assigned.message_type.type =
    PCI_RESOURCES_ASSIGNED;
    res_assigned.wslot.slot = hpdev.desc.win_slot.slot;
    } else {
    res_assigned2 =
    (struct pci_resources_assigned2 *)(pkt + 1);
    res_assigned2.message_type.type =
    PCI_RESOURCES_ASSIGNED2;
    res_assigned2.wslot.slot = hpdev.desc.win_slot.slot;
    }
    put_pcichild(hpdev);
    ret = vmbus_sendpacket(hdev.channel, pkt + 1,
    size_res, (unsigned long)pkt,
    VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (!ret)
    ret = wait_for_response(hdev, &comp_pkt.host_event);
    if (ret)
    break;
    if (comp_pkt.completion_status < 0) {
    ret = -EPROTO;
    dev_err(&hdev.device,
    "resource allocated returned 0x%x",
    comp_pkt.completion_status);
    break;
    }
    hbus.wslot_res_allocated = wslot;
    }
    kfree(pkt);
    return ret;
    }
//
// hv_send_resources_released() - Report local resources
// released
// @hdev:	VMBus's tracking struct for this root PCI bus
//
// Return: 0 on success, -errno on failure
//
#[no_mangle]
unsafe extern "C" fn hv_send_resources_released(hdev: *mut hv_device) -> c_int {
    static int hv_send_resources_released(struct hv_device *hdev)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    struct pci_child_message pkt;
    struct hv_pci_dev *hpdev;
    int wslot;
    int ret;
    for (wslot = hbus.wslot_res_allocated; wslot >= 0; wslot--) {
    hpdev = get_pcichild_wslot(hbus, wslot);
    if (!hpdev)
    continue;
    memset(&pkt, 0, sizeof(pkt));
    pkt.message_type.type = PCI_RESOURCES_RELEASED;
    pkt.wslot.slot = hpdev.desc.win_slot.slot;
    put_pcichild(hpdev);
    ret = vmbus_sendpacket(hdev.channel, &pkt, sizeof(pkt), 0,
    VM_PKT_DATA_INBAND, 0);
    if (ret)
    return ret;
    hbus.wslot_res_allocated = wslot - 1;
    }
    hbus.wslot_res_allocated = -1;
    return 0;
    }
//
// hv_pci_probe() - New VMBus channel probe, for a root PCI bus
// @hdev:	VMBus's tracking struct for this root PCI bus
// @dev_id:	Identifies the device itself
//
// Return: 0 on success, -errno on failure
//
    static int hv_pci_probe(struct hv_device *hdev,
    const struct hv_vmbus_device_id *dev_id)
    {
    struct pci_host_bridge *bridge;
    struct hv_pcibus_device *hbus;
    int ret, dom;
    u16 dom_req;
    char *name;
    bridge = devm_pci_alloc_host_bridge(&hdev.device, 0);
    if (!bridge)
    return -ENOMEM;
    hbus = kzalloc_obj(*hbus);
    if (!hbus)
    return -ENOMEM;
    hbus.bridge = bridge;
    mutex_init(&hbus.state_lock);
    hbus.state = hv_pcibus_init;
    hbus.wslot_res_allocated = -1;
//
// The PCI bus "domain" is what is called "segment" in ACPI and other
// specs. Pull it from the instance ID, to get something usually
// unique. In rare cases of collision, we will find out another number
// not in use.
//
// Note that, since this code only runs in a Hyper-V VM, Hyper-V
// together with this guest driver can guarantee that (1) The only
// domain used by Gen1 VMs for something that looks like a physical
// PCI bus (which is actually emulated by the hypervisor) is domain 0.
// (2) There will be no overlap between domains (after fixing possible
// collisions) in the same VM.
//
// Because Gen1 VMs use domain 0, don't allow picking domain 0 here,
// even if bytes 4 and 5 of the instance GUID are both zero. For wider
// userspace compatibility, limit the domain ID to a 16-bit value.
//
    dom_req = hdev.dev_instance.b[5] << 8 | hdev.dev_instance.b[4];
    dom = pci_bus_find_emul_domain_nr(dom_req, 1, U16_MAX);
    if (dom < 0) {
    dev_err(&hdev.device,
    "Unable to use dom# 0x%x or other numbers", dom_req);
    ret = -EINVAL;
    goto free_bus;
    }
    if (dom != dom_req)
    dev_info(&hdev.device,
    "PCI dom# 0x%x has collision, using 0x%x",
    dom_req, dom);
    hbus.bridge.domain_nr = dom;

    hbus.sysdata.domain = dom;
    hbus.use_calls = !!(ms_hyperv.hints & HV_X64_USE_MMIO_HYPERCALLS);

//
// Set the PCI bus parent to be the corresponding VMbus
// device. Then the VMbus device will be assigned as the
// ACPI companion in pcibios_root_bridge_prepare() and
// pci_dma_configure() will propagate device coherence
// information to devices created on the bus.
//
    hbus.sysdata.parent = hdev.device.parent;
    hbus.use_calls = false;

    hbus.hdev = hdev;
    INIT_LIST_HEAD(&hbus.children);
    INIT_LIST_HEAD(&hbus.dr_list);
    spin_lock_init(&hbus.config_lock);
    spin_lock_init(&hbus.device_list_lock);
    hbus.wq = alloc_ordered_workqueue("hv_pci_%x", 0,
    hbus.bridge.domain_nr);
    if (!hbus.wq) {
    ret = -ENOMEM;
    goto free_bus;
    }
    hdev.channel.next_request_id_callback = vmbus_next_request_id;
    hdev.channel.request_addr_callback = vmbus_request_addr;
    hdev.channel.rqstor_size = HV_PCI_RQSTOR_SIZE;
    ret = vmbus_open(hdev.channel, pci_ring_size, pci_ring_size, core::ptr::null_mut(), 0,
    hv_pci_onchannelcallback, hbus);
    if (ret)
    goto destroy_wq;
    hv_set_drvdata(hdev, hbus);
    ret = hv_pci_protocol_negotiation(hdev, pci_protocol_versions,
    ARRAY_SIZE(pci_protocol_versions));
    if (ret)
    goto close;
    ret = hv_allocate_config_window(hbus);
    if (ret)
    goto close;
    hbus.cfg_addr = ioremap(hbus.mem_config.start,
    PCI_CONFIG_MMIO_LENGTH);
    if (!hbus.cfg_addr) {
    dev_err(&hdev.device,
    "Unable to map a virtual address for config space\n");
    ret = -ENOMEM;
    goto free_config;
    }
    name = kasprintf(GFP_KERNEL, "%pUL", &hdev.dev_instance);
    if (!name) {
    ret = -ENOMEM;
    goto unmap;
    }
    hbus.fwnode = irq_domain_alloc_named_fwnode(name);
    kfree(name);
    if (!hbus.fwnode) {
    ret = -ENOMEM;
    goto unmap;
    }
    ret = hv_pcie_init_irq_domain(hbus);
    if (ret)
    goto free_fwnode;
    ret = hv_pci_query_relations(hdev);
    if (ret)
    goto free_irq_domain;
    mutex_lock(&hbus.state_lock);
    ret = hv_pci_enter_d0(hdev);
    if (ret)
    goto release_state_lock;
    ret = hv_pci_allocate_bridge_windows(hbus);
    if (ret)
    goto exit_d0;
    ret = hv_send_resources_allocated(hdev);
    if (ret)
    goto free_windows;
    prepopulate_bars(hbus);
    hbus.state = hv_pcibus_probed;
    ret = create_root_hv_pci_bus(hbus);
    if (ret)
    goto free_windows;
    mutex_unlock(&hbus.state_lock);
    return 0;
    free_windows:
    hv_pci_free_bridge_windows(hbus);
    exit_d0:
    (void) hv_pci_bus_exit(hdev, true);
    release_state_lock:
    mutex_unlock(&hbus.state_lock);
    free_irq_domain:
    irq_domain_remove(hbus.irq_domain);
    free_fwnode:
    irq_domain_free_fwnode(hbus.fwnode);
    unmap:
    iounmap(hbus.cfg_addr);
    free_config:
    hv_free_config_window(hbus);
    close:
    vmbus_close(hdev.channel);
    destroy_wq:
    destroy_workqueue(hbus.wq);
    free_bus:
    kfree(hbus);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hv_pci_bus_exit(hdev: *mut hv_device, keep_devs: bool) -> c_int {
    static int hv_pci_bus_exit(struct hv_device *hdev, bool keep_devs)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    struct vmbus_channel *chan = hdev.channel;
    struct {
    struct pci_packet teardown_packet;
    u8 buffer[sizeof(struct pci_message)];
    } pkt;
    struct pci_message *msg;
    struct hv_pci_compl comp_pkt;
    struct hv_pci_dev *hpdev, *tmp;
    unsigned long flags;
    u64 trans_id;
    int ret;
//
// After the host sends the RESCIND_CHANNEL message, it doesn't
// access the per-channel ringbuffer any longer.
//
    if (chan.rescind)
    return 0;
    if (!keep_devs) {
    struct list_head removed;
// Move all present children to the list on stack
    INIT_LIST_HEAD(&removed);
    spin_lock_irqsave(&hbus.device_list_lock, flags);
    list_for_each_entry_safe(hpdev, tmp, &hbus.children, list_entry)
    list_move_tail(&hpdev.list_entry, &removed);
    spin_unlock_irqrestore(&hbus.device_list_lock, flags);
// Remove all children in the list
    list_for_each_entry_safe(hpdev, tmp, &removed, list_entry) {
    list_del(&hpdev.list_entry);
    if (hpdev.pci_slot)
    pci_destroy_slot(hpdev.pci_slot);
// For the two refs got in new_pcichild_device()
    put_pcichild(hpdev);
    put_pcichild(hpdev);
    }
    }
    ret = hv_send_resources_released(hdev);
    if (ret) {
    dev_err(&hdev.device,
    "Couldn't send resources released packet(s)\n");
    return ret;
    }
    memset(&pkt.teardown_packet, 0, sizeof(pkt.teardown_packet));
    init_completion(&comp_pkt.host_event);
    pkt.teardown_packet.completion_func = hv_pci_generic_compl;
    pkt.teardown_packet.compl_ctxt = &comp_pkt;
    msg = (struct pci_message *)pkt.buffer;
    msg.type = PCI_BUS_D0EXIT;
    ret = vmbus_sendpacket_getid(chan, msg, sizeof(*msg),
    (unsigned long)&pkt.teardown_packet,
    &trans_id, VM_PKT_DATA_INBAND,
    VMBUS_DATA_PACKET_FLAG_COMPLETION_REQUESTED);
    if (ret)
    return ret;
    if (wait_for_completion_timeout(&comp_pkt.host_event, 10 * HZ) == 0) {
//
// The completion packet on the stack becomes invalid after
// 'return'; remove the ID from the VMbus requestor if the
// identifier is still mapped to/associated with the packet.
//
// Cf. hv_pci_onchannelcallback().
//
    vmbus_request_addr_match(chan, trans_id,
    (unsigned long)&pkt.teardown_packet);
    return -ETIMEDOUT;
    }
    return 0;
    }
//
// hv_pci_remove() - Remove routine for this VMBus channel
// @hdev:	VMBus's tracking struct for this root PCI bus
//
#[no_mangle]
unsafe extern "C" fn hv_pci_remove(hdev: *mut hv_device) {
    static void hv_pci_remove(struct hv_device *hdev)
    {
    struct hv_pcibus_device *hbus;
    hbus = hv_get_drvdata(hdev);
    if (hbus.state == hv_pcibus_installed) {
    tasklet_disable(&hdev.channel.callback_event);
    hbus.state = hv_pcibus_removing;
    tasklet_enable(&hdev.channel.callback_event);
    destroy_workqueue(hbus.wq);
    hbus.wq = core::ptr::null_mut();
//
// At this point, no work is running or can be scheduled
// on hbus-wq. We can't race with hv_pci_devices_present()
// or hv_pci_eject_device(), it's safe to proceed.
//
// Remove the bus from PCI's point of view.
    pci_lock_rescan_remove();
    pci_stop_root_bus(hbus.bridge.bus);
    hv_pci_remove_slots(hbus);
    pci_remove_root_bus(hbus.bridge.bus);
    pci_unlock_rescan_remove();
    }
    hv_pci_bus_exit(hdev, false);
    vmbus_close(hdev.channel);
    iounmap(hbus.cfg_addr);
    hv_free_config_window(hbus);
    hv_pci_free_bridge_windows(hbus);
    irq_domain_remove(hbus.irq_domain);
    irq_domain_free_fwnode(hbus.fwnode);
    kfree(hbus);
    }
#[no_mangle]
unsafe extern "C" fn hv_pci_suspend(hdev: *mut hv_device) -> c_int {
    static int hv_pci_suspend(struct hv_device *hdev)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    enum hv_pcibus_state old_state;
    int ret;
//
// hv_pci_suspend() must make sure there are no pending work items
// before calling vmbus_close(), since it runs in a process context
// as a callback in dpm_suspend().  When it starts to run, the channel
// callback hv_pci_onchannelcallback(), which runs in a tasklet
// context, can be still running concurrently and scheduling new work
// items onto hbus->wq in hv_pci_devices_present() and
// hv_pci_eject_device(), and the work item handlers can access the
// vmbus channel, which can be being closed by hv_pci_suspend(), e.g.
// the work item handler pci_devices_present_work() ->
// new_pcichild_device() writes to the vmbus channel.
//
// To eliminate the race, hv_pci_suspend() disables the channel
// callback tasklet, sets hbus->state to hv_pcibus_removing, and
// re-enables the tasklet. This way, when hv_pci_suspend() proceeds,
// it knows that no new work item can be scheduled, and then it flushes
// hbus->wq and safely closes the vmbus channel.
//
    tasklet_disable(&hdev.channel.callback_event);
// Change the hbus state to prevent new work items.
    old_state = hbus.state;
    if (hbus.state == hv_pcibus_installed)
    hbus.state = hv_pcibus_removing;
    tasklet_enable(&hdev.channel.callback_event);
    if (old_state != hv_pcibus_installed)
    return -EINVAL;
    flush_workqueue(hbus.wq);
    ret = hv_pci_bus_exit(hdev, true);
    if (ret)
    return ret;
    vmbus_close(hdev.channel);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hv_pci_restore_msi_msg(pdev: *mut pci_dev, arg: *mut c_void) -> c_int {
    static int hv_pci_restore_msi_msg(struct pci_dev *pdev, void *arg)
    {
    struct irq_data *irq_data;
    struct msi_desc *entry;
    if (!pdev.msi_enabled && !pdev.msix_enabled)
    return 0;
    guard(msi_descs_lock)(&pdev.dev);
    msi_for_each_desc(entry, &pdev.dev, MSI_DESC_ASSOCIATED) {
    irq_data = irq_get_irq_data(entry.irq);
    if (WARN_ON_ONCE(!irq_data))
    return -EINVAL;
    hv_compose_msi_msg(irq_data, &entry.msg);
    }
    return 0;
    }
//
// Upon resume, pci_restore_msi_state() -> ... ->  __pci_write_msi_msg()
// directly writes the MSI/MSI-X registers via MMIO, but since Hyper-V
// doesn't trap and emulate the MMIO accesses, here hv_compose_msi_msg()
// must be used to ask Hyper-V to re-create the IOMMU Interrupt Remapping
// Table entries.
//
#[no_mangle]
unsafe extern "C" fn hv_pci_restore_msi_state(hbus: *mut hv_pcibus_device) {
    static void hv_pci_restore_msi_state(struct hv_pcibus_device *hbus)
    {
    pci_walk_bus(hbus.bridge.bus, hv_pci_restore_msi_msg, core::ptr::null_mut());
    }
#[no_mangle]
unsafe extern "C" fn hv_pci_resume(hdev: *mut hv_device) -> c_int {
    static int hv_pci_resume(struct hv_device *hdev)
    {
    struct hv_pcibus_device *hbus = hv_get_drvdata(hdev);
    enum pci_protocol_version_t version[1];
    int ret;
    hbus.state = hv_pcibus_init;
    hdev.channel.next_request_id_callback = vmbus_next_request_id;
    hdev.channel.request_addr_callback = vmbus_request_addr;
    hdev.channel.rqstor_size = HV_PCI_RQSTOR_SIZE;
    ret = vmbus_open(hdev.channel, pci_ring_size, pci_ring_size, core::ptr::null_mut(), 0,
    hv_pci_onchannelcallback, hbus);
    if (ret)
    return ret;
// Only use the version that was in use before hibernation.
    version[0] = hbus.protocol_version;
    ret = hv_pci_protocol_negotiation(hdev, version, 1);
    if (ret)
    goto out;
    ret = hv_pci_query_relations(hdev);
    if (ret)
    goto out;
    mutex_lock(&hbus.state_lock);
    ret = hv_pci_enter_d0(hdev);
    if (ret)
    goto release_state_lock;
    ret = hv_send_resources_allocated(hdev);
    if (ret)
    goto release_state_lock;
    prepopulate_bars(hbus);
    hv_pci_restore_msi_state(hbus);
    hbus.state = hv_pcibus_installed;
    mutex_unlock(&hbus.state_lock);
    return 0;
    release_state_lock:
    mutex_unlock(&hbus.state_lock);
    out:
    vmbus_close(hdev.channel);
    return ret;
    }
    static const struct hv_vmbus_device_id hv_pci_id_table[] = {
// PCI Pass-through Class ID
// 44C4F61D-4444-4400-9D52-802E27EDE19F
    { HV_PCIE_GUID, },
    { },
    };
    MODULE_DEVICE_TABLE(vmbus, hv_pci_id_table);
    static struct hv_driver hv_pci_drv = {
    .name		= "hv_pci",
    .id_table	= hv_pci_id_table,
    .probe		= hv_pci_probe,
    .remove		= hv_pci_remove,
    .suspend	= hv_pci_suspend,
    .resume		= hv_pci_resume,
    };
#[no_mangle]
unsafe extern "C" fn exit_hv_pci_drv() -> void __exit {
    static void __exit exit_hv_pci_drv(void)
    {
    vmbus_driver_unregister(&hv_pci_drv);
    hvpci_block_ops.read_block = core::ptr::null_mut();
    hvpci_block_ops.write_block = core::ptr::null_mut();
    hvpci_block_ops.reg_blk_invalidate = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn init_hv_pci_drv() -> int __init {
    static int __init init_hv_pci_drv(void)
    {
    int ret;
    if (!hv_is_hyperv_initialized())
    return -ENODEV;
    if (!hv_vmbus_exists())
    return -ENODEV;
    ret = hv_pci_irqchip_init();
    if (ret)
    return ret;
// Initialize PCI block r/w interface
    hvpci_block_ops.read_block = hv_read_config_block;
    hvpci_block_ops.write_block = hv_write_config_block;
    hvpci_block_ops.reg_blk_invalidate = hv_register_block_invalidate;
    return vmbus_driver_register(&hv_pci_drv);
    }
    module_init(init_hv_pci_drv);
    module_exit(exit_hv_pci_drv);
    MODULE_DESCRIPTION("Hyper-V PCI");
    MODULE_LICENSE("GPL v2");
