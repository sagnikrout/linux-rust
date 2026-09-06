//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/hptiop.h
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
// HighPoint RR3xxx/4xxx controller driver for Linux
// Copyright (C) 2006-2015 HighPoint Technologies, Inc. All Rights Reserved.
//
// Please report bugs/comments/suggestions to linux@highpoint-tech.com
//
// For more information, visit http://www.highpoint-tech.com
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iopmu_itl {
    pub resrved0: [__le32; 4],
    pub inbound_msgaddr0: __le32,
    pub inbound_msgaddr1: __le32,
    pub outbound_msgaddr0: __le32,
    pub outbound_msgaddr1: __le32,
    pub inbound_doorbell: __le32,
    pub inbound_intstatus: __le32,
    pub inbound_intmask: __le32,
    pub outbound_doorbell: __le32,
    pub outbound_intstatus: __le32,
    pub outbound_intmask: __le32,
    pub reserved1: [__le32; 2],
    pub inbound_queue: __le32,
    pub outbound_queue: __le32,
}

pub const IOPMU_QUEUE_EMPTY: c_uint = 0xffffffff;
pub const IOPMU_QUEUE_MASK_HOST_BITS: c_uint = 0xf0000000;
pub const IOPMU_QUEUE_ADDR_HOST_BIT: c_uint = 0x80000000;
pub const IOPMU_QUEUE_REQUEST_SIZE_BIT: c_uint = 0x40000000;
pub const IOPMU_QUEUE_REQUEST_RESULT_BIT: c_uint = 0x40000000;
pub const IOPMU_OUTBOUND_INT_MSG0: c_int = 1;
pub const IOPMU_OUTBOUND_INT_MSG1: c_int = 2;
pub const IOPMU_OUTBOUND_INT_DOORBELL: c_int = 4;
pub const IOPMU_OUTBOUND_INT_POSTQUEUE: c_int = 8;
pub const IOPMU_OUTBOUND_INT_PCI: c_uint = 0x10;
pub const IOPMU_INBOUND_INT_MSG0: c_int = 1;
pub const IOPMU_INBOUND_INT_MSG1: c_int = 2;
pub const IOPMU_INBOUND_INT_DOORBELL: c_int = 4;
pub const IOPMU_INBOUND_INT_ERROR: c_int = 8;
pub const IOPMU_INBOUND_INT_POSTQUEUE: c_uint = 0x10;
pub const MVIOP_QUEUE_LEN: c_int = 512;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iopmu_mv {
    pub inbound_head: __le32,
    pub inbound_tail: __le32,
    pub outbound_head: __le32,
    pub outbound_tail: __le32,
    pub inbound_msg: __le32,
    pub outbound_msg: __le32,
    pub reserve: [__le32; 10],
    pub inbound_q: [__le64; MVIOP_QUEUE_LEN],
    pub outbound_q: [__le64; MVIOP_QUEUE_LEN],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iopmv_regs {
    pub 4]: __le32 reserved[0x20400 /,
    pub inbound_doorbell: __le32,
    pub inbound_intmask: __le32,
    pub outbound_doorbell: __le32,
    pub outbound_intmask: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iopmu_mvfrey {
    pub 4]: __le32 reserved0[(0x4000 - 0) /,
    pub inbound_base: __le32,
    pub inbound_base_high: __le32,
    pub 4]: __le32 reserved1[(0x4018 - 0x4008) /,
    pub inbound_write_ptr: __le32,
    pub 4]: __le32 reserved2[(0x402c - 0x401c) /,
    pub inbound_conf_ctl: __le32,
    pub 4]: __le32 reserved3[(0x4050 - 0x4030) /,
    pub outbound_base: __le32,
    pub outbound_base_high: __le32,
    pub outbound_shadow_base: __le32,
    pub outbound_shadow_base_high: __le32,
    pub 4]: __le32 reserved4[(0x4088 - 0x4060) /,
    pub isr_cause: __le32,
    pub isr_enable: __le32,
    pub 4]: __le32 reserved5[(0x1020c - 0x4090) /,
    pub pcie_f0_int_enable: __le32,
    pub 4]: __le32 reserved6[(0x10400 - 0x10210) /,
    pub f0_to_cpu_msg_a: __le32,
    pub 4]: __le32 reserved7[(0x10420 - 0x10404) /,
    pub cpu_to_f0_msg_a: __le32,
    pub 4]: __le32 reserved8[(0x10480 - 0x10424) /,
    pub f0_doorbell: __le32,
    pub f0_doorbell_enable: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvfrey_inlist_entry {
    pub addr: dma_addr_t,
    pub intrfc_len: __le32,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mvfrey_outlist_entry {
    pub val: __le32,
}

pub const MVIOP_MU_QUEUE_ADDR_HOST_BIT: c_int = 4;
pub const MVIOP_MU_QUEUE_ADDR_IOP_HIGH32: c_uint = 0xffffffff;
pub const MVIOP_MU_QUEUE_REQUEST_RESULT_BIT: c_int = 1;
pub const MVIOP_MU_QUEUE_REQUEST_RETURN_CONTEXT: c_int = 2;
pub const MVIOP_MU_INBOUND_INT_MSG: c_int = 1;
pub const MVIOP_MU_INBOUND_INT_POSTQUEUE: c_int = 2;
pub const MVIOP_MU_OUTBOUND_INT_MSG: c_int = 1;
pub const MVIOP_MU_OUTBOUND_INT_POSTQUEUE: c_int = 2;
pub const CL_POINTER_TOGGLE: c_uint = 0x00004000;
pub const CPU_TO_F0_DRBL_MSG_BIT: c_uint = 0x02000000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpt_iopmu_message {
// host-to-iop messages
    IOPMU_INBOUND_MSG0_NOP = 0,
    IOPMU_INBOUND_MSG0_RESET,
    IOPMU_INBOUND_MSG0_FLUSH,
    IOPMU_INBOUND_MSG0_SHUTDOWN,
    IOPMU_INBOUND_MSG0_STOP_BACKGROUND_TASK,
    IOPMU_INBOUND_MSG0_START_BACKGROUND_TASK,
    IOPMU_INBOUND_MSG0_RESET_COMM,
    IOPMU_INBOUND_MSG0_MAX = 0xff,
// iop-to-host messages
    IOPMU_OUTBOUND_MSG0_REGISTER_DEVICE_0 = 0x100,
    IOPMU_OUTBOUND_MSG0_REGISTER_DEVICE_MAX = 0x1ff,
    IOPMU_OUTBOUND_MSG0_UNREGISTER_DEVICE_0 = 0x200,
    IOPMU_OUTBOUND_MSG0_UNREGISTER_DEVICE_MAX = 0x2ff,
    IOPMU_OUTBOUND_MSG0_REVALIDATE_DEVICE_0 = 0x300,
    IOPMU_OUTBOUND_MSG0_REVALIDATE_DEVICE_MAX = 0x3ff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iop_request_header {
    pub size: __le32,
    pub type: __le32,
    pub flags: __le32,
    pub result: __le32,
    pub /: *mut *mut __le32 context; / host context,
    pub context_hi32: __le32,
}

pub const IOP_REQUEST_FLAG_SYNC_REQUEST: c_int = 1;
pub const IOP_REQUEST_FLAG_BIST_REQUEST: c_int = 2;
pub const IOP_REQUEST_FLAG_REMAPPED: c_int = 4;
pub const IOP_REQUEST_FLAG_OUTPUT_CONTEXT: c_int = 8;
pub const IOP_REQUEST_FLAG_ADDR_BITS: c_uint = 0x40 /* flags[31:16] is phy_addr[47:32] */;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpt_iop_request_type {
    IOP_REQUEST_TYPE_GET_CONFIG = 0,
    IOP_REQUEST_TYPE_SET_CONFIG,
    IOP_REQUEST_TYPE_BLOCK_COMMAND,
    IOP_REQUEST_TYPE_SCSI_COMMAND,
    IOP_REQUEST_TYPE_IOCTL_COMMAND,
    IOP_REQUEST_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hpt_iop_result_type {
    IOP_RESULT_PENDING = 0,
    IOP_RESULT_SUCCESS,
    IOP_RESULT_FAIL,
    IOP_RESULT_BUSY,
    IOP_RESULT_RESET,
    IOP_RESULT_INVALID_REQUEST,
    IOP_RESULT_BAD_TARGET,
    IOP_RESULT_CHECK_CONDITION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iop_request_get_config {
    pub header: hpt_iop_request_header,
    pub interface_version: __le32,
    pub firmware_version: __le32,
    pub max_requests: __le32,
    pub request_size: __le32,
    pub max_sg_count: __le32,
    pub data_transfer_length: __le32,
    pub alignment_mask: __le32,
    pub max_devices: __le32,
    pub sdram_size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iop_request_set_config {
    pub header: hpt_iop_request_header,
    pub iop_id: __le32,
    pub vbus_id: __le16,
    pub max_host_request_size: __le16,
    pub reserve: [__le32; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iopsg {
    pub size: __le32,
    pub /: *mut *mut __le32 eot; / non-zero: end of table,
    pub pci_address: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iop_request_block_command {
    pub header: hpt_iop_request_header,
    pub channel: u8,
    pub target: u8,
    pub lun: u8,
    pub pad1: u8,
    pub /: *mut *mut __le16 command; / IOP_BLOCK_COMMAND_{READ,WRITE},
    pub sectors: __le16,
    pub lba: __le64,
    pub sg_list: [hpt_iopsg; 1],
}

pub const IOP_BLOCK_COMMAND_READ: c_int = 1;
pub const IOP_BLOCK_COMMAND_WRITE: c_int = 2;
pub const IOP_BLOCK_COMMAND_VERIFY: c_int = 3;
pub const IOP_BLOCK_COMMAND_FLUSH: c_int = 4;
pub const IOP_BLOCK_COMMAND_SHUTDOWN: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iop_request_scsi_command {
    pub header: hpt_iop_request_header,
    pub channel: u8,
    pub target: u8,
    pub lun: u8,
    pub pad1: u8,
    pub cdb: [u8; 16],
    pub dataxfer_length: __le32,
    pub sg_list: [hpt_iopsg; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_iop_request_ioctl_command {
    pub header: hpt_iop_request_header,
    pub ioctl_code: __le32,
    pub inbuf_size: __le32,
    pub outbuf_size: __le32,
    pub bytes_returned: __le32,
    pub buf: [u8; ],
// out data should be put at buf[(inbuf_size+3)&~3]
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hptiop_request {
    pub next: *mut hptiop_request,
    pub req_virt: *mut c_void,
    pub req_shifted_phy: u32,
    pub scp: *mut scsi_cmnd,
    pub index: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_cmd_priv {
    pub mapped: c_int,
    pub sgcnt: c_int,
    pub dma_handle: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hptiop_family {
    UNKNOWN_BASED_IOP,
    INTEL_BASED_IOP,
    MV_BASED_IOP,
    MVFREY_BASED_IOP
    } ;

    struct hptiop_hba {
    struct hptiop_adapter_ops *ops;
    union {
    struct {
    struct hpt_iopmu_itl __iomem *iop;
    void __iomem *plx;
    } itl;
    struct {
    struct hpt_iopmv_regs *regs;
    struct hpt_iopmu_mv __iomem *mu;
    void *internal_req;
    dma_addr_t internal_req_phy;
    } mv;
    struct {
    struct hpt_iop_request_get_config __iomem *config;
    struct hpt_iopmu_mvfrey __iomem *mu;

    int internal_mem_size;
    struct hptiop_request internal_req;
    int list_count;
    struct mvfrey_inlist_entry *inlist;
    dma_addr_t inlist_phy;
    __le32 inlist_wptr;
    struct mvfrey_outlist_entry *outlist;
    dma_addr_t outlist_phy;
    __le32 *outlist_cptr; /* copy pointer shadow */
    dma_addr_t outlist_cptr_phy;
    __le32 outlist_rptr;
    } mvfrey;
    } u;

    struct Scsi_Host *host;
    struct pci_dev *pcidev;

// IOP config info
    u32     interface_version;
    u32     firmware_version;
    u32     sdram_size;
    u32     max_devices;
    u32     max_requests;
    u32     max_request_size;
    u32     max_sg_descriptors;

    u32     req_size; /* host-allocated request buffer size */

    u32     iopintf_v2: 1;
    u32     initialized: 1;
    u32     msg_done: 1;

    struct hptiop_request * req_list;
    struct hptiop_request reqs[HPTIOP_MAX_REQUESTS];

// used to free allocated dma area
    void        *dma_coherent[HPTIOP_MAX_REQUESTS];
    dma_addr_t  dma_coherent_handle[HPTIOP_MAX_REQUESTS];

    atomic_t    reset_count;
    atomic_t    resetting;

    wait_queue_head_t reset_wq;
    wait_queue_head_t ioctl_wq;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hpt_ioctl_k {
    pub hba: *mut *mut hptiop_hba,
    pub ioctl_code: u32,
    pub inbuf_size: u32,
    pub outbuf_size: u32,
    pub inbuf: *mut c_void,
    pub outbuf: *mut c_void,
    pub bytes_returned: *mut u32,
    pub ): *mut *mut void (done)(struct hpt_ioctl_k,
    pub /: *mut *mut int result; / HPT_IOCTL_RESULT_,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hptiop_adapter_ops {
    pub family: hptiop_family,
    pub millisec): *mut *mut *mut int (iop_wait_ready)(struct hptiop_hba hba, u32,
    pub hba): *mut *mut int (internal_memalloc)(struct hptiop_hba,
    pub hba): *mut *mut int (internal_memfree)(struct hptiop_hba,
    pub hba): *mut *mut int (map_pci_bar)(struct hptiop_hba,
    pub hba): *mut *mut void (unmap_pci_bar)(struct hptiop_hba,
    pub hba): *mut *mut void (enable_intr)(struct hptiop_hba,
    pub hba): *mut *mut void (disable_intr)(struct hptiop_hba,
    pub config): *mut hpt_iop_request_get_config,
    pub config): *mut hpt_iop_request_set_config,
    pub hba): *mut *mut int (iop_intr)(struct hptiop_hba,
    pub msg): *mut *mut *mut void (post_msg)(struct hptiop_hba hba, u32,
    pub _req): *mut *mut *mut void (post_req)(struct hptiop_hba hba, struct hptiop_request,
    pub hw_dma_bit_mask: c_int,
    pub hba): *mut *mut int (reset_comm)(struct hptiop_hba,
    pub host_phy_flag: __le64,
}

pub const HPT_IOCTL_RESULT_OK: c_int = 0;

