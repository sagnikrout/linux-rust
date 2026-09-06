//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_pci.h
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


//
// Virtio PCI driver
//
// This module allows virtio devices to be used over a virtual PCI device.
// This can be used with QEMU based VMMs like KVM or Xen.
//
// Copyright IBM Corp. 2007
//
// Authors:
// Anthony Liguori  <aliguori@us.ibm.com>
//
// This header is BSD licensed so anyone can use the definitions to implement
// compatible drivers/servers.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
// notice, this list of conditions and the following disclaimer in the
// documentation and/or other materials provided with the distribution.
// 3. Neither the name of IBM nor the names of its contributors
// may be used to endorse or promote products derived from this software
// without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS ``AS IS'' AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED.  IN NO EVENT SHALL IBM OR CONTRIBUTORS BE LIABLE
// FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
// OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
// HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
// LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
// OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
// SUCH DAMAGE.
//

// A 32-bit r/o bitmask of the features supported by the host
pub const VIRTIO_PCI_HOST_FEATURES: c_int = 0;
// A 32-bit r/w bitmask of features activated by the guest
pub const VIRTIO_PCI_GUEST_FEATURES: c_int = 4;
// A 32-bit r/w PFN for the currently selected queue
pub const VIRTIO_PCI_QUEUE_PFN: c_int = 8;
// A 16-bit r/o queue size for the currently selected queue
pub const VIRTIO_PCI_QUEUE_NUM: c_int = 12;
// A 16-bit r/w queue selector
pub const VIRTIO_PCI_QUEUE_SEL: c_int = 14;
// A 16-bit r/w queue notifier
pub const VIRTIO_PCI_QUEUE_NOTIFY: c_int = 16;
// An 8-bit device status register.
pub const VIRTIO_PCI_STATUS: c_int = 18;
// An 8-bit r/o interrupt status register.  Reading the value will return the
// current contents of the ISR and will also clear it.  This is effectively
// a read-and-acknowledge.
pub const VIRTIO_PCI_ISR: c_int = 19;
// MSI-X registers: only enabled if MSI-X is enabled.
// A 16-bit vector for configuration changes.
pub const VIRTIO_MSI_CONFIG_VECTOR: c_int = 20;
// A 16-bit vector for selected queue notifications.
pub const VIRTIO_MSI_QUEUE_VECTOR: c_int = 22;
// The remaining space is defined by each driver as the per-driver
// configuration space

// Deprecated: please use VIRTIO_PCI_CONFIG_OFF instead

// Virtio ABI version, this must match exactly
pub const VIRTIO_PCI_ABI_VERSION: c_int = 0;
// How many bits to shift physical queue address written to QUEUE_PFN.
// 12 is historical, and due to x86 page size.
pub const VIRTIO_PCI_QUEUE_ADDR_SHIFT: c_int = 12;
// The alignment to use between consumer and producer parts of vring.
// x86 pagesize again.
pub const VIRTIO_PCI_VRING_ALIGN: c_int = 4096;

// The bit of the ISR which indicates a device configuration change.
pub const VIRTIO_PCI_ISR_CONFIG: c_uint = 0x2;
// Vector value used to disable MSI for queue
pub const VIRTIO_MSI_NO_VECTOR: c_uint = 0xffff;

// IDs for different capabilities.  Must all exist.
// Common configuration
pub const VIRTIO_PCI_CAP_COMMON_CFG: c_int = 1;
// Notifications
pub const VIRTIO_PCI_CAP_NOTIFY_CFG: c_int = 2;
// ISR access
pub const VIRTIO_PCI_CAP_ISR_CFG: c_int = 3;
// Device specific configuration
pub const VIRTIO_PCI_CAP_DEVICE_CFG: c_int = 4;
// PCI configuration access
pub const VIRTIO_PCI_CAP_PCI_CFG: c_int = 5;
// Additional shared memory capability
pub const VIRTIO_PCI_CAP_SHARED_MEMORY_CFG: c_int = 8;
// PCI vendor data configuration
pub const VIRTIO_PCI_CAP_VENDOR_CFG: c_int = 9;
// This is the PCI capability header:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_cap {
    pub /: *mut *mut __u8 cap_vndr; / Generic PCI field: PCI_CAP_ID_VNDR,
    pub /: *mut *mut __u8 cap_next; / Generic PCI field: next ptr.,
    pub /: *mut *mut __u8 cap_len; / Generic PCI field: capability length,
    pub /: *mut *mut __u8 cfg_type; / Identifies the structure.,
    pub /: *mut *mut __u8 bar; / Where to find it.,
    pub /: *mut *mut __u8 id; / Multiple capabilities of the same type,
    pub /: *mut *mut __u8 padding[2]; / Pad to full dword.,
    pub /: *mut *mut __le32 offset; / Offset within bar.,
    pub /: *mut *mut __le32 length; / Length of the structure, in bytes.,
}

// This is the PCI vendor data capability header:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_vndr_data {
    pub /: *mut *mut __u8 cap_vndr; / Generic PCI field: PCI_CAP_ID_VNDR,
    pub /: *mut *mut __u8 cap_next; / Generic PCI field: next ptr.,
    pub /: *mut *mut __u8 cap_len; / Generic PCI field: capability length,
    pub /: *mut *mut __u8 cfg_type; / Identifies the structure.,
    pub /: *mut *mut __u16 vendor_id; / Identifies the vendor-specific format.,
// For Vendor Definition
// Pads structure to a multiple of 4 bytes
// Reads must not have side effects
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_cap64 {
    pub cap: virtio_pci_cap,
    pub /: *mut *mut __le32 offset_hi; / Most sig 32 bits of offset,
    pub /: *mut *mut __le32 length_hi; / Most sig 32 bits of length,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_notify_cap {
    pub cap: virtio_pci_cap,
    pub /: *mut *mut __le32 notify_off_multiplier; / Multiplier for queue_notify_off.,
}

// Fields in VIRTIO_PCI_CAP_COMMON_CFG:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_common_cfg {
// About the whole device.
    pub /: *mut *mut __le32 device_feature_select; / read-write,
    pub /: *mut *mut __le32 device_feature; / read-only,
    pub /: *mut *mut __le32 guest_feature_select; / read-write,
    pub /: *mut *mut __le32 guest_feature; / read-write,
    pub /: *mut *mut __le16 msix_config; / read-write,
    pub /: *mut *mut __le16 num_queues; / read-only,
    pub /: *mut *mut __u8 device_status; / read-write,
    pub /: *mut *mut __u8 config_generation; / read-only,
// About a specific virtqueue.
    pub /: *mut *mut __le16 queue_select; / read-write,
    pub /: *mut *mut __le16 queue_size; / read-write, power of 2.,
    pub /: *mut *mut __le16 queue_msix_vector; / read-write,
    pub /: *mut *mut __le16 queue_enable; / read-write,
    pub /: *mut *mut __le16 queue_notify_off; / read-only,
    pub /: *mut *mut __le32 queue_desc_lo; / read-write,
    pub /: *mut *mut __le32 queue_desc_hi; / read-write,
    pub /: *mut *mut __le32 queue_avail_lo; / read-write,
    pub /: *mut *mut __le32 queue_avail_hi; / read-write,
    pub /: *mut *mut __le32 queue_used_lo; / read-write,
    pub /: *mut *mut __le32 queue_used_hi; / read-write,
}

//
// Warning: do not use sizeof on this: use offsetofend for
// specific fields you need.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_modern_common_cfg {
    pub cfg: virtio_pci_common_cfg,
    pub /: *mut *mut __le16 queue_notify_data; / read-write,
    pub /: *mut *mut __le16 queue_reset; / read-write,
    pub /: *mut *mut __le16 admin_queue_index; / read-only,
    pub /: *mut *mut __le16 admin_queue_num; / read-only,
}

// Fields in VIRTIO_PCI_CAP_PCI_CFG:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_pci_cfg_cap {
    pub cap: virtio_pci_cap,
    pub /: *mut *mut __u8 pci_cfg_data[4]; / Data for BAR access.,
}

// Macro versions of offsets for the Old Timers!
pub const VIRTIO_PCI_CAP_VNDR: c_int = 0;
pub const VIRTIO_PCI_CAP_NEXT: c_int = 1;
pub const VIRTIO_PCI_CAP_LEN: c_int = 2;
pub const VIRTIO_PCI_CAP_CFG_TYPE: c_int = 3;
pub const VIRTIO_PCI_CAP_BAR: c_int = 4;
pub const VIRTIO_PCI_CAP_OFFSET: c_int = 8;
pub const VIRTIO_PCI_CAP_LENGTH: c_int = 12;
pub const VIRTIO_PCI_NOTIFY_CAP_MULT: c_int = 16;
pub const VIRTIO_PCI_COMMON_DFSELECT: c_int = 0;
pub const VIRTIO_PCI_COMMON_DF: c_int = 4;
pub const VIRTIO_PCI_COMMON_GFSELECT: c_int = 8;
pub const VIRTIO_PCI_COMMON_GF: c_int = 12;
pub const VIRTIO_PCI_COMMON_MSIX: c_int = 16;
pub const VIRTIO_PCI_COMMON_NUMQ: c_int = 18;
pub const VIRTIO_PCI_COMMON_STATUS: c_int = 20;
pub const VIRTIO_PCI_COMMON_CFGGENERATION: c_int = 21;
pub const VIRTIO_PCI_COMMON_Q_SELECT: c_int = 22;
pub const VIRTIO_PCI_COMMON_Q_SIZE: c_int = 24;
pub const VIRTIO_PCI_COMMON_Q_MSIX: c_int = 26;
pub const VIRTIO_PCI_COMMON_Q_ENABLE: c_int = 28;
pub const VIRTIO_PCI_COMMON_Q_NOFF: c_int = 30;
pub const VIRTIO_PCI_COMMON_Q_DESCLO: c_int = 32;
pub const VIRTIO_PCI_COMMON_Q_DESCHI: c_int = 36;
pub const VIRTIO_PCI_COMMON_Q_AVAILLO: c_int = 40;
pub const VIRTIO_PCI_COMMON_Q_AVAILHI: c_int = 44;
pub const VIRTIO_PCI_COMMON_Q_USEDLO: c_int = 48;
pub const VIRTIO_PCI_COMMON_Q_USEDHI: c_int = 52;
pub const VIRTIO_PCI_COMMON_Q_NDATA: c_int = 56;
pub const VIRTIO_PCI_COMMON_Q_RESET: c_int = 58;
pub const VIRTIO_PCI_COMMON_ADM_Q_IDX: c_int = 60;
pub const VIRTIO_PCI_COMMON_ADM_Q_NUM: c_int = 62;

// Admin command status.
pub const VIRTIO_ADMIN_STATUS_OK: c_int = 0;
// Admin command opcode.
pub const VIRTIO_ADMIN_CMD_LIST_QUERY: c_uint = 0x0;
pub const VIRTIO_ADMIN_CMD_LIST_USE: c_uint = 0x1;
// Admin command group type.
pub const VIRTIO_ADMIN_GROUP_TYPE_SELF: c_uint = 0x0;
pub const VIRTIO_ADMIN_GROUP_TYPE_SRIOV: c_uint = 0x1;
// Transitional device admin command.
pub const VIRTIO_ADMIN_CMD_LEGACY_COMMON_CFG_WRITE: c_uint = 0x2;
pub const VIRTIO_ADMIN_CMD_LEGACY_COMMON_CFG_READ: c_uint = 0x3;
pub const VIRTIO_ADMIN_CMD_LEGACY_DEV_CFG_WRITE: c_uint = 0x4;
pub const VIRTIO_ADMIN_CMD_LEGACY_DEV_CFG_READ: c_uint = 0x5;
pub const VIRTIO_ADMIN_CMD_LEGACY_NOTIFY_INFO: c_uint = 0x6;
// Device parts access commands.
pub const VIRTIO_ADMIN_CMD_CAP_ID_LIST_QUERY: c_uint = 0x7;
pub const VIRTIO_ADMIN_CMD_DEVICE_CAP_GET: c_uint = 0x8;
pub const VIRTIO_ADMIN_CMD_DRIVER_CAP_SET: c_uint = 0x9;
pub const VIRTIO_ADMIN_CMD_RESOURCE_OBJ_CREATE: c_uint = 0xa;
pub const VIRTIO_ADMIN_CMD_RESOURCE_OBJ_DESTROY: c_uint = 0xd;
pub const VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_GET: c_uint = 0xe;
pub const VIRTIO_ADMIN_CMD_DEV_PARTS_GET: c_uint = 0xf;
pub const VIRTIO_ADMIN_CMD_DEV_PARTS_SET: c_uint = 0x10;
pub const VIRTIO_ADMIN_CMD_DEV_MODE_SET: c_uint = 0x11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_hdr {
    pub opcode: __le16,
//
// 1 - SR-IOV
// 2-65535 - reserved
//
    pub group_type: __le16,
// Unused, reserved for future extensions.
    pub reserved1: [__u8; 12],
    pub group_member_id: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_status {
    pub status: __le16,
    pub status_qualifier: __le16,
// Unused, reserved for future extensions.
    pub reserved2: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_legacy_wr_data {
    pub /: *mut *mut __u8 offset; / Starting offset of the register(s) to write.,
    pub reserved: [__u8; 7],
    pub registers: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_legacy_rd_data {
    pub /: *mut *mut __u8 offset; / Starting offset of the register(s) to read.,
}

pub const VIRTIO_ADMIN_CMD_NOTIFY_INFO_FLAGS_END: c_int = 0;
pub const VIRTIO_ADMIN_CMD_NOTIFY_INFO_FLAGS_OWNER_DEV: c_uint = 0x1;
pub const VIRTIO_ADMIN_CMD_NOTIFY_INFO_FLAGS_OWNER_MEM: c_uint = 0x2;
pub const VIRTIO_ADMIN_CMD_MAX_NOTIFY_INFO: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_notify_info_data {
    pub /: *mut *mut __u8 flags; / 0 = end of list, 1 = owner device, 2 = member device,
    pub /: *mut *mut __u8 bar; / BAR of the member or the owner device,
    pub padding: [__u8; 6],
    pub /: *mut *mut __le64 offset; / Offset within bar.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_notify_info_result {
    pub entries: [virtio_admin_cmd_notify_info_data; VIRTIO_ADMIN_CMD_MAX_NOTIFY_INFO],
}

pub const VIRTIO_DEV_PARTS_CAP: c_uint = 0x0000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_dev_parts_cap {
    pub get_parts_resource_objects_limit: __u8,
    pub set_parts_resource_objects_limit: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_query_cap_id_result {
    pub supported_caps: [__le64; MAX_CAP_ID],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_cap_get_data {
    pub id: __le16,
    pub reserved: [__u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_cap_set_data {
    pub id: __le16,
    pub reserved: [__u8; 6],
    pub cap_specific_data: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_resource_obj_cmd_hdr {
    pub type: __le16,
    pub reserved: [__u8; 2],
    pub /: *mut *mut __le32 id; / Indicates unique resource object id per resource object type,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_resource_obj_create_data {
    pub hdr: virtio_admin_cmd_resource_obj_cmd_hdr,
    pub flags: __le64,
    pub resource_obj_specific_data: [__u8; ],
}

pub const VIRTIO_RESOURCE_OBJ_DEV_PARTS: c_int = 0;
pub const VIRTIO_RESOURCE_OBJ_DEV_PARTS_TYPE_GET: c_int = 0;
pub const VIRTIO_RESOURCE_OBJ_DEV_PARTS_TYPE_SET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_resource_obj_dev_parts {
    pub type: __u8,
    pub reserved: [__u8; 7],
}

pub const VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_TYPE_SIZE: c_int = 0;
pub const VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_TYPE_COUNT: c_int = 1;
pub const VIRTIO_ADMIN_CMD_DEV_PARTS_METADATA_TYPE_LIST: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_dev_parts_metadata_data {
    pub hdr: virtio_admin_cmd_resource_obj_cmd_hdr,
    pub type: __u8,
    pub reserved: [__u8; 7],
}

pub const VIRTIO_DEV_PART_F_OPTIONAL: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_dev_part_hdr {
    pub part_type: __le16,
    pub flags: __u8,
    pub reserved: __u8,
    pub offset: __le32,
    pub reserved: __le32,
    pub pci_common_cfg: },
    pub index: __le16,
    pub reserved: [__u8; 6],
    pub vq_index: },
    pub selector: },
    pub length: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_dev_part {
    pub hdr: virtio_dev_part_hdr,
    pub value: [__u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_dev_parts_metadata_result {
    pub size: __le32,
    pub reserved: __le32,
    pub parts_size: },
    pub count: __le32,
    pub reserved: __le32,
    pub hdr_list_count: },
    pub count: __le32,
    pub reserved: __le32,
    pub hdrs: [virtio_dev_part_hdr; ],
    pub hdr_list: },
}

pub const VIRTIO_ADMIN_CMD_DEV_PARTS_GET_TYPE_SELECTED: c_int = 0;
pub const VIRTIO_ADMIN_CMD_DEV_PARTS_GET_TYPE_ALL: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_dev_parts_get_data {
    pub hdr: virtio_admin_cmd_resource_obj_cmd_hdr,
    pub type: __u8,
    pub reserved: [__u8; 7],
    pub hdr_list: [virtio_dev_part_hdr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_dev_parts_set_data {
    pub hdr: virtio_admin_cmd_resource_obj_cmd_hdr,
    pub parts: [virtio_dev_part; ],
}

pub const VIRTIO_ADMIN_CMD_DEV_MODE_F_STOPPED: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_admin_cmd_dev_mode_set_data {
    pub flags: __u8,
}
