//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mana/gdma.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
// Copyright (c) 2021, Microsoft Corporation.

pub const GDMA_STATUS_MORE_ENTRIES: c_uint = 0x00000105;
pub const GDMA_STATUS_CMD_UNSUPPORTED: c_uint = 0xffffffff;
// Structures labeled with "HW DATA" are exchanged with the hardware. All of
// them are naturally aligned and hence don't need __packed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_request_type {
    GDMA_VERIFY_VF_DRIVER_VERSION	= 1,
    GDMA_QUERY_MAX_RESOURCES	= 2,
    GDMA_LIST_DEVICES		= 3,
    GDMA_REGISTER_DEVICE		= 4,
    GDMA_DEREGISTER_DEVICE		= 5,
    GDMA_GENERATE_TEST_EQE		= 10,
    GDMA_CREATE_QUEUE		= 12,
    GDMA_DISABLE_QUEUE		= 13,
    GDMA_ALLOCATE_RESOURCE_RANGE	= 22,
    GDMA_DESTROY_RESOURCE_RANGE	= 24,
    GDMA_CREATE_DMA_REGION		= 25,
    GDMA_DMA_REGION_ADD_PAGES	= 26,
    GDMA_DESTROY_DMA_REGION		= 27,
    GDMA_CREATE_PD			= 29,
    GDMA_DESTROY_PD			= 30,
    GDMA_CREATE_MR			= 31,
    GDMA_DESTROY_MR			= 32,
    GDMA_QUERY_HWC_TIMEOUT		= 84, /* 0x54 */
    GDMA_ALLOC_DM			= 96, /* 0x60 */
    GDMA_DESTROY_DM			= 97, /* 0x61 */
}

pub const GDMA_RESOURCE_DOORBELL_PAGE: c_int = 27;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_queue_type {
    GDMA_INVALID_QUEUE,
    GDMA_SQ,
    GDMA_RQ,
    GDMA_CQ,
    GDMA_EQ,
    GDMA_DIM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_work_request_flags {
    GDMA_WR_NONE			= 0,
    GDMA_WR_OOB_IN_SGL		= BIT(0),
    GDMA_WR_PAD_BY_SGE0		= BIT(1),
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_eqe_type {
    GDMA_EQE_COMPLETION		= 3,
    GDMA_EQE_TEST_EVENT		= 64,
    GDMA_EQE_HWC_INIT_EQ_ID_DB	= 129,
    GDMA_EQE_HWC_INIT_DATA		= 130,
    GDMA_EQE_HWC_INIT_DONE		= 131,
    GDMA_EQE_HWC_FPGA_RECONFIG	= 132,
    GDMA_EQE_HWC_SOC_RECONFIG_DATA	= 133,
    GDMA_EQE_HWC_SOC_SERVICE	= 134,
    GDMA_EQE_HWC_RESET_REQUEST	= 135,
    GDMA_EQE_RNIC_QP_FATAL		= 176,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_service_type {
    GDMA_SERVICE_TYPE_NONE		= 0,
    GDMA_SERVICE_TYPE_RDMA_SUSPEND	= 1,
    GDMA_SERVICE_TYPE_RDMA_RESUME	= 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_service_work {
    pub work: work_struct,
    pub gdma_dev: *mut gdma_dev,
    pub event: gdma_service_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_resource {
// Protect the bitmap
    pub lock: spinlock_t,
// The bitmap size in bits.
    pub size: u32,
// The bitmap tracks the resources.
    pub map: *mut c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union gdma_doorbell_entry {
    pub as_uint64: u64,
    pub 24: u64 id :,
    pub 8: u64 reserved :,
    pub 31: u64 tail_ptr :,
    pub 1: u64 arm :,
    pub cq: },
    pub 24: u64 id :,
    pub 8: u64 wqe_cnt :,
    pub 32: u64 tail_ptr :,
    pub rq: },
    pub 24: u64 id :,
    pub 8: u64 reserved :,
    pub 32: u64 tail_ptr :,
    pub sq: },
    pub 16: u64 id :,
    pub 16: u64 reserved :,
    pub 31: u64 tail_ptr :,
    pub 1: u64 arm :,
    pub eq: },
    pub 24: u64 id :,
    pub 8: u64 reserved :,
    pub 10: u64 mod_usec :,
    pub 5: u64 reserve1 :,
    pub 1: u64 mod_usec_vld :,
    pub 8: u64 mod_comps :,
    pub 7: u64 reserve2 :,
    pub 1: u64 mod_comps_vld:,
    pub dim: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_msg_hdr {
    pub hdr_type: u32,
    pub msg_type: u32,
    pub msg_version: u16,
    pub hwc_msg_id: u16,
    pub msg_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_dev_id {
    pub type: u16,
    pub instance: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_req_hdr {
    pub req: gdma_msg_hdr,
    pub /: *mut *mut gdma_msg_hdr resp; / The expected response,
    pub dev_id: gdma_dev_id,
    pub activity_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_resp_hdr {
    pub response: gdma_msg_hdr,
    pub dev_id: gdma_dev_id,
    pub activity_id: u32,
    pub status: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_general_req {
    pub hdr: gdma_req_hdr,
}

pub const GDMA_MESSAGE_V1: c_int = 1;
pub const GDMA_MESSAGE_V2: c_int = 2;
pub const GDMA_MESSAGE_V3: c_int = 3;
pub const GDMA_MESSAGE_V4: c_int = 4;
pub const GDMA_MESSAGE_V5: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_general_resp {
    pub hdr: gdma_resp_hdr,
}

pub const GDMA_STANDARD_HEADER_TYPE: c_int = 0;
// The 16-byte struct is part of the GDMA work queue entry (WQE).
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_sge {
    pub address: u64,
    pub mem_key: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_wqe_request {
    pub sgl: *mut gdma_sge,
    pub num_sge: u32,
    pub inline_oob_size: u32,
    pub inline_oob_data: *const c_void,
    pub flags: u32,
    pub client_data_unit: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_page_type {
    GDMA_PAGE_TYPE_4K,
}

pub const GDMA_INVALID_DMA_REGION: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_serv_work {
    pub serv_work: work_struct,
    pub pdev: *mut pci_dev,
    pub type: gdma_eqe_type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_mem_info {
    pub dev: *mut device,
    pub dma_handle: dma_addr_t,
    pub virt_addr: *mut c_void,
    pub length: u64,
// Scattered fallback: when @nr_pages > 0 the ring is that many
// PAGE_SIZE coherent allocations in @pages_va/@pages_dma, not
// @virt_addr/@dma_handle.
//
    pub pages_va: *mut c_void,
    pub pages_dma: *mut dma_addr_t,
    pub nr_pages: c_uint,
// Allocated by the PF driver
    pub dma_region_handle: u64,
}

pub const REGISTER_ATB_MST_MKEY_LOWER_SIZE: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_dev {
    pub gdma_context: *mut gdma_context,
    pub dev_id: gdma_dev_id,
    pub pdid: u32,
    pub doorbell: u32,
    pub gpa_mkey: u32,
// GDMA driver specific pointer
    pub driver_data: *mut c_void,
    pub adev: *mut auxiliary_device,
    pub is_suspended: bool,
    pub rdma_teardown: bool,
}

// MANA_PAGE_SIZE is the DMA unit
pub const MANA_PAGE_SHIFT: c_int = 12;

// Required by HW

pub const GDMA_CQE_SIZE: c_int = 64;
pub const GDMA_EQE_SIZE: c_int = 16;
pub const GDMA_MAX_SQE_SIZE: c_int = 512;
pub const GDMA_MAX_RQE_SIZE: c_int = 256;
pub const GDMA_COMP_DATA_SIZE: c_uint = 0x3C;
pub const GDMA_EVENT_DATA_SIZE: c_uint = 0xC;
// The WQE size must be a multiple of the Basic Unit, which is 32 bytes.
pub const GDMA_WQE_BU_SIZE: c_int = 32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_comp {
    pub 4]: u32 cqe_data[GDMA_COMP_DATA_SIZE /,
    pub wq_num: u32,
    pub is_sq: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_event {
    pub 4]: u32 details[GDMA_EVENT_DATA_SIZE /,
    pub type: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mana_eq {
    pub eq: *mut gdma_queue,
    pub mana_eq_debugfs: *mut dentry,
}

extern "C" {
    pub fn gdma_cq_callback(context: *mut c_void, q: *mut gdma_queue) -> typedef void;
}
// The 'head' is the producer index. For SQ/RQ, when the driver posts a WQE
// (Note: the WQE size must be a multiple of the 32-byte Basic Unit), the
// driver increases the 'head' in BUs rather than in bytes, and notifies
// the HW of the updated head. For EQ/CQ, the driver uses the 'head' to track
// the HW head, and increases the 'head' by 1 for every processed EQE/CQE.
//
// The 'tail' is the consumer index for SQ/RQ. After the CQE of the SQ/RQ is
// processed, the driver increases the 'tail' to indicate that WQEs have
// been consumed by the HW, so the driver can post new WQEs into the SQ/RQ.
//
// The driver doesn't use the 'tail' for EQ/CQ, because the driver ensures
// that the EQ/CQ is big enough so they can't overflow, and the driver uses
// the owner bits mechanism to detect if the queue has become empty.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_queue {
    pub gdma_dev: *mut gdma_dev,
    pub type: gdma_queue_type,
    pub id: u32,
    pub mem_info: gdma_mem_info,
    pub queue_mem_ptr: *mut c_void,
    pub queue_size: u32,
    pub monitor_avl_buf: bool,
    pub head: u32,
    pub tail: u32,
    pub entry: list_head,
// Extra fields specific to EQ/CQ.
    pub disable_needed: bool,
    pub callback: *mut gdma_eq_callback,
    pub context: *mut c_void,
    pub msix_index: c_uint,
    pub irq: c_uint,
    pub log2_throttle_limit: u32,
    pub eq: },
    pub callback: *mut gdma_cq_callback,
    pub context: *mut c_void,
    pub /: *mut *mut *mut gdma_queue parent; / For CQ/EQ relationship,
    pub cq: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_queue_spec {
    pub type: gdma_queue_type,
    pub monitor_avl_buf: bool,
    pub queue_size: c_uint,
// Extra fields specific to EQ/CQ.
    pub callback: *mut gdma_eq_callback,
    pub context: *mut c_void,
    pub log2_throttle_limit: c_ulong,
    pub msix_index: c_uint,
    pub eq: },
    pub callback: *mut gdma_cq_callback,
    pub context: *mut c_void,
    pub parent_eq: *mut gdma_queue,
    pub cq: },
}

pub const MANA_IRQ_NAME_SZ: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_irq_context {
    pub arg): *mut *mut void (handler)(void,
// Protect the eq_list
    pub lock: spinlock_t,
    pub eq_list: list_head,
    pub name: [c_char; MANA_IRQ_NAME_SZ],
    pub msi: c_uint,
    pub irq: c_uint,
    pub refcount: refcount_t,
    pub bitmap_refs: c_uint,
    pub dyn_msix: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_context_flags {
    GC_PROBE_SUCCEEDED	= 0,
    GC_IN_SERVICE		= 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_context {
    pub dev: *mut device,
    pub mana_pci_debugfs: *mut dentry,
// Hardware max number of queues
    pub max_num_queues: c_uint,
// Per-vPort max number of queues
    pub max_num_queues_vport: c_uint,
    pub max_num_msix: c_uint,
    pub num_msix_usable: c_uint,
    pub irq_contexts: xarray,
// L2 MTU
    pub adapter_mtu: u16,
// NIC supports CQE x8 coalescing
    pub cqe8_coalescing_sup: bool,
// This maps a CQ index to the queue structure.
    pub max_num_cqs: c_uint,
    pub cq_table: *mut gdma_queue,
// Protect eq_test_event and test_event_eq_id
    pub eq_test_event_mutex: mutex,
    pub eq_test_event: completion,
    pub test_event_eq_id: u32,
    pub is_pf: bool,
    pub is_pf2: bool,
    pub bar0_pa: phys_addr_t,
    pub bar0_va: *mut void __iomem,
    pub bar0_size: resource_size_t,
    pub shm_base: *mut void __iomem,
    pub db_page_base: *mut void __iomem,
    pub phys_db_page_base: phys_addr_t,
    pub db_page_off: u64,
    pub db_page_size: u64,
    pub numa_node: c_int,
// Shared memory chanenl (used to bootstrap HWC)
    pub shm_channel: shm_channel,
// Hardware communication channel (HWC)
    pub hwc: gdma_dev,
// Azure network adapter
    pub mana: gdma_dev,
// Azure RDMA adapter
    pub mana_ib: gdma_dev,
    pub pf_cap_flags1: u64,
    pub gdma_protocol_ver: u64,
    pub service_wq: *mut workqueue_struct,
    pub flags: c_ulong,
// Protect access to GIC context
    pub gic_mutex: mutex,
// Indicate if this device is sharing MSI for EQs on MANA
    pub msi_sharing: bool,
// Bitmap tracks where MSI is allocated when it is not shared for EQs
    pub msi_bitmap: *mut c_ulong,
}

extern "C" {
    pub fn mana_gd_wq_avail_space(wq: *mut gdma_queue) -> u32;
}
extern "C" {
    pub fn mana_gd_test_eq(gc: *mut gdma_context, eq: *mut gdma_queue) -> c_int;
}
extern "C" {
    pub fn mana_gd_destroy_queue(gc: *mut gdma_context, queue: *mut gdma_queue);
}
extern "C" {
    pub fn mana_gd_poll_cq(cq: *mut gdma_queue, comp: *mut gdma_comp, num_cqe: c_int) -> c_int;
}
extern "C" {
    pub fn mana_gd_ring_cq(cq: *mut gdma_queue, arm_bit: u8);
}
extern "C" {
    pub fn mana_schedule_serv_work(gc: *mut gdma_context, type: gdma_eqe_type) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_wqe {
    pub :24: u32 reserved,
    pub :8: u32 last_vbytes,
    pub flags: u32,
    pub :8: u32 num_sge,
    pub inline_oob_size_div4:3: u32,
    pub :1: u32 client_oob_in_sgl,
    pub :4: u32 reserved1,
    pub :14: u32 client_data_unit,
    pub :2: u32 reserved2,
}

pub const INLINE_OOB_SMALL_SIZE: c_int = 8;
pub const INLINE_OOB_LARGE_SIZE: c_int = 24;
pub const MANA_MAX_TX_WQE_SGL_ENTRIES: c_int = 30;
pub const MAX_TX_WQE_SIZE: c_int = 512;
pub const MAX_RX_WQE_SIZE: c_int = 256;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_cqe {
    pub 4]: u32 cqe_data[GDMA_COMP_DATA_SIZE /,
    pub as_uint32: u32,
    pub 24: u32 wq_num :,
    pub 1: u32 is_sq :,
    pub 4: u32 reserved :,
    pub 3: u32 owner_bits :,
}

pub const GDMA_CQE_OWNER_BITS: c_int = 3;

pub const SET_ARM_BIT: c_int = 1;
pub const GDMA_EQE_OWNER_BITS: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub union gdma_eqe_info {
    pub as_uint32: u32,
    pub 8: u32 type :,
    pub 8: u32 reserved1 :,
    pub 2: u32 client_id :,
    pub 11: u32 reserved2 :,
    pub 3: u32 owner_bits :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_eqe {
    pub 4]: u32 details[GDMA_EVENT_DATA_SIZE /,
    pub eqe_info: u32,
}

pub const GDMA_REG_DB_PAGE_OFFSET: c_int = 8;
pub const GDMA_REG_DB_PAGE_SIZE: c_uint = 0x10;
pub const GDMA_REG_SHM_OFFSET: c_uint = 0x18;
pub const GDMA_PF_REG_DB_PAGE_SIZE: c_uint = 0xD0;
pub const GDMA_PF_REG_DB_PAGE_OFF: c_uint = 0xC8;
pub const GDMA_PF_REG_SHM_OFF: c_uint = 0x70;
pub const GDMA_SRIOV_REG_CFG_BASE_OFF: c_uint = 0x108;
pub const MANA_PF_DEVICE_ID: c_uint = 0x00B9;
pub const MANA_PF2_DEVICE_ID: c_uint = 0x00C1;
pub const MANA_VF_DEVICE_ID: c_uint = 0x00BA;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_posted_wqe_info {
    pub wqe_size_in_bu: u32,
}

// GDMA_GENERATE_TEST_EQE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_generate_test_event_req {
    pub hdr: gdma_req_hdr,
    pub queue_index: u32,
}

// GDMA_VERIFY_VF_DRIVER_VERSION

// Advertise to the NIC firmware: the NAPI work_done variable race is fixed,
// so the driver is able to reliably support features like busy_poll.
//

// Driver can handle holes (zeros) in the device list

// Driver supports dynamic MSI-X vector allocation

// Driver can self reset on EQE notification

// Driver can self reset on FPGA Reconfig EQE notification

// Driver detects stalled send queues and recovers them

// Driver supports separate EQ/MSIs for each vPort

// Driver supports linearizing the skb when num_sge exceeds hardware limit

// Driver can send HWC periodically to query stats

// Driver can handle hardware recovery events during probe

// Driver supports self recovery on Hardware Channel timeouts

// Driver supports dynamic interrupt moderation - DIM

// Driver supports non-contiguous queue buffers

pub const GDMA_DRV_CAP_FLAGS2: c_int = 0;
pub const GDMA_DRV_CAP_FLAGS3: c_int = 0;
pub const GDMA_DRV_CAP_FLAGS4: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_verify_ver_req {
    pub hdr: gdma_req_hdr,
// Mandatory fields required for protocol establishment
    pub protocol_ver_min: u64,
    pub protocol_ver_max: u64,
// Gdma Driver Capability Flags
    pub gd_drv_cap_flags1: u64,
    pub gd_drv_cap_flags2: u64,
    pub gd_drv_cap_flags3: u64,
    pub gd_drv_cap_flags4: u64,
// Advisory fields
    pub drv_ver: u64,
    pub /: *mut *mut u32 os_type; / Linux = 0x10; Windows = 0x20; Other = 0x30,
    pub reserved: u32,
    pub os_ver_major: u32,
    pub os_ver_minor: u32,
    pub os_ver_build: u32,
    pub os_ver_platform: u32,
    pub reserved_2: u64,
    pub os_ver_str1: [u8; 128],
    pub os_ver_str2: [u8; 128],
    pub os_ver_str3: [u8; 128],
    pub os_ver_str4: [u8; 128],
}

// HW supports dynamic interrupt moderation - DIM

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_verify_ver_resp {
    pub hdr: gdma_resp_hdr,
    pub gdma_protocol_ver: u64,
    pub pf_cap_flags1: u64,
    pub pf_cap_flags2: u64,
    pub pf_cap_flags3: u64,
    pub pf_cap_flags4: u64,
}

// GDMA_QUERY_MAX_RESOURCES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_query_max_resources_resp {
    pub hdr: gdma_resp_hdr,
    pub status: u32,
    pub max_sq: u32,
    pub max_rq: u32,
    pub max_cq: u32,
    pub max_eq: u32,
    pub max_db: u32,
    pub max_mst: u32,
    pub max_cq_mod_ctx: u32,
    pub max_mod_cq: u32,
    pub max_msix: u32,
}

// GDMA_LIST_DEVICES
pub const GDMA_DEV_LIST_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_list_devices_resp {
    pub hdr: gdma_resp_hdr,
    pub num_of_devs: u32,
    pub reserved: u32,
    pub devs: [gdma_dev_id; GDMA_DEV_LIST_SIZE],
}

// GDMA_REGISTER_DEVICE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_register_device_resp {
    pub hdr: gdma_resp_hdr,
    pub pdid: u32,
    pub gpa_mkey: u32,
    pub db_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_allocate_resource_range_req {
    pub hdr: gdma_req_hdr,
    pub resource_type: u32,
    pub num_resources: u32,
    pub alignment: u32,
    pub allocated_resources: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_allocate_resource_range_resp {
    pub hdr: gdma_resp_hdr,
    pub allocated_resources: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_resource_range_req {
    pub hdr: gdma_req_hdr,
    pub resource_type: u32,
    pub num_resources: u32,
    pub allocated_resources: u32,
}

// GDMA_CREATE_QUEUE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_queue_req {
    pub hdr: gdma_req_hdr,
    pub type: u32,
    pub reserved1: u32,
    pub pdid: u32,
    pub doolbell_id: u32,
    pub gdma_region: u64,
    pub reserved2: u32,
    pub queue_size: u32,
    pub log2_throttle_limit: u32,
    pub eq_pci_msix_index: u32,
    pub cq_mod_ctx_id: u32,
    pub cq_parent_eq_id: u32,
    pub rq_drop_on_overrun: u8,
    pub rq_err_on_wqe_overflow: u8,
    pub rq_chain_rec_wqes: u8,
    pub sq_hw_db: u8,
    pub reserved3: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_queue_resp {
    pub hdr: gdma_resp_hdr,
    pub queue_index: u32,
}

// GDMA_DISABLE_QUEUE
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_disable_queue_req {
    pub hdr: gdma_req_hdr,
    pub type: u32,
    pub queue_index: u32,
    pub alloc_res_id_on_creation: u32,
}

// GDMA_QUERY_HWC_TIMEOUT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_query_hwc_timeout_req {
    pub hdr: gdma_req_hdr,
    pub timeout_ms: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_query_hwc_timeout_resp {
    pub hdr: gdma_resp_hdr,
    pub timeout_ms: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_mr_access_flags {
    GDMA_ACCESS_FLAG_LOCAL_READ = BIT_ULL(0),
    GDMA_ACCESS_FLAG_LOCAL_WRITE = BIT_ULL(1),
    GDMA_ACCESS_FLAG_REMOTE_READ = BIT_ULL(2),
    GDMA_ACCESS_FLAG_REMOTE_WRITE = BIT_ULL(3),
    GDMA_ACCESS_FLAG_REMOTE_ATOMIC = BIT_ULL(4),
    GDMA_ACCESS_FLAG_BIND_MW = BIT_ULL(5),
}

// GDMA_CREATE_DMA_REGION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_dma_region_req {
    pub hdr: gdma_req_hdr,
// The total size of the DMA region
    pub length: u64,
// The offset in the first page
    pub offset_in_page: u32,
// enum gdma_page_type
    pub gdma_page_type: u32,
// The total number of pages
    pub page_count: u32,
// If page_addr_list_len is smaller than page_count,
// the remaining page addresses will be added via the
// message GDMA_DMA_REGION_ADD_PAGES.
//
    pub page_addr_list_len: u32,
    pub page_addr_list: [u64; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_dma_region_resp {
    pub hdr: gdma_resp_hdr,
    pub dma_region_handle: u64,
}

// GDMA_DMA_REGION_ADD_PAGES
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_dma_region_add_pages_req {
    pub hdr: gdma_req_hdr,
    pub dma_region_handle: u64,
    pub page_addr_list_len: u32,
    pub reserved3: u32,
    pub page_addr_list: [u64; ],
}

// GDMA_DESTROY_DMA_REGION
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_dma_region_req {
    pub hdr: gdma_req_hdr,
    pub dma_region_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_pd_flags {
    GDMA_PD_FLAG_ALLOW_GPA_MR = BIT(0),
    GDMA_PD_FLAG_SHORT_PDN = BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_pd_req {
    pub hdr: gdma_req_hdr,
    pub flags: gdma_pd_flags,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_pd_resp {
    pub hdr: gdma_resp_hdr,
    pub pd_handle: u64,
    pub pd_id: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_pd_req {
    pub hdr: gdma_req_hdr,
    pub pd_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_pd_resp {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gdma_mr_type {
//
// Guest Physical Address - MRs of this type allow access
// to any DMA-mapped memory using bus-logical address
//
    GDMA_MR_TYPE_GPA = 1,
// Guest Virtual Address - MRs of this type allow access
// to memory mapped by PTEs associated with this MR using a virtual
// address that is set up in the MST
//
    GDMA_MR_TYPE_GVA = 2,
// Guest zero-based address MRs
    GDMA_MR_TYPE_ZBVA = 4,
// Device address MRs
    GDMA_MR_TYPE_DM = 5,
// Memory Window type 1
    GDMA_MR_TYPE_MW1 = 6,
// Memory Window type 2
    GDMA_MR_TYPE_MW2 = 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_mr_params {
    pub pd_handle: u64,
    pub mr_type: gdma_mr_type,
    pub dma_region_handle: u64,
    pub virtual_address: u64,
    pub access_flags: gdma_mr_access_flags,
    pub gva: },
    pub dma_region_handle: u64,
    pub access_flags: gdma_mr_access_flags,
    pub zbva: },
    pub dm_handle: u64,
    pub offset: u64,
    pub length: u64,
    pub access_flags: gdma_mr_access_flags,
    pub da: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_mr_request {
    pub hdr: gdma_req_hdr,
    pub pd_handle: u64,
    pub mr_type: gdma_mr_type,
    pub reserved_1: u32,
    pub dma_region_handle: u64,
    pub virtual_address: u64,
    pub access_flags: gdma_mr_access_flags,
    pub gva: } __packed,
    pub dma_region_handle: u64,
    pub access_flags: gdma_mr_access_flags,
    pub zbva: } __packed,
    pub dm_handle: u64,
    pub offset: u64,
    pub access_flags: gdma_mr_access_flags,
    pub da: } __packed,
    pub __packed: },
    pub reserved_2: u32,
    pub length: u64,
    pub da_ext: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_create_mr_response {
    pub hdr: gdma_resp_hdr,
    pub mr_handle: u64,
    pub lkey: u32,
    pub rkey: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_mr_request {
    pub hdr: gdma_req_hdr,
    pub mr_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_mr_response {
    pub hdr: gdma_resp_hdr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_alloc_dm_req {
    pub hdr: gdma_req_hdr,
    pub length: u64,
    pub alignment: u32,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_alloc_dm_resp {
    pub hdr: gdma_resp_hdr,
    pub dm_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_dm_req {
    pub hdr: gdma_req_hdr,
    pub dm_handle: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gdma_destroy_dm_resp {
    pub hdr: gdma_resp_hdr,
}

extern "C" {
    pub fn mana_gd_verify_vf_version(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn mana_gd_register_device(gd: *mut gdma_dev) -> c_int;
}
extern "C" {
    pub fn mana_gd_deregister_device(gd: *mut gdma_dev) -> c_int;
}
extern "C" {
    pub fn mana_gd_alloc_res_map(res_avail: u32, r: *mut gdma_resource) -> c_int;
}
extern "C" {
    pub fn mana_gd_free_res_map(r: *mut gdma_resource);
}
extern "C" {
    pub fn mana_gd_free_memory(gmi: *mut gdma_mem_info);
}
extern "C" {
    pub fn mana_gd_destroy_dma_region(gc: *mut gdma_context, dma_region_handle: u64) -> c_int;
}
extern "C" {
    pub fn mana_register_debugfs();
}
extern "C" {
    pub fn mana_unregister_debugfs();
}
extern "C" {
    pub fn mana_rdma_service_event(gc: *mut gdma_context, event: gdma_service_type) -> c_int;
}
extern "C" {
    pub fn mana_gd_suspend(pdev: *mut pci_dev, state: pm_message_t) -> c_int;
}
extern "C" {
    pub fn mana_gd_resume(pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn mana_need_log(gc: *mut gdma_context, err: c_int) -> bool;
}
extern "C" {
    pub fn mana_gd_put_gic(gc: *mut gdma_context, use_msi_bitmap: bool, msi: c_int);
}
