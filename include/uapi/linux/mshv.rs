//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mshv.h
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
// Userspace interfaces for /dev/mshv* devices and derived fds
//
// This file is divided into sections containing data structures and IOCTLs for
// a particular set of related devices or derived file descriptors.
//
// The IOCTL definitions are at the end of each section. They are grouped by
// device/fd, so that new IOCTLs can easily be added with a monotonically
// increasing number.
//

pub const MSHV_IOCTL: c_uint = 0xB8;
//
// Entry point to main VMM APIs: /dev/mshv
//

//
// struct mshv_create_partition - arguments for MSHV_CREATE_PARTITION
// @pt_flags: Bitmask of 1 << MSHV_PT_BIT_
// @pt_isolation: MSHV_PT_ISOLATION_
//
// This is the initial/v1 version for backward compatibility.
//
// Returns a file descriptor to act as a handle to a guest partition.
// At this point the partition is not yet initialized in the hypervisor.
// Some operations must be done with the partition in this state, e.g. setting
// so-called "early" partition properties. The partition can then be
// initialized with MSHV_INITIALIZE_PARTITION.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_create_partition {
    pub pt_flags: __u64,
    pub pt_isolation: __u64,
}

pub const MSHV_NUM_CPU_FEATURES_BANKS: c_int = 2;
//
// struct mshv_create_partition_v2
//
// This is extended version of the above initial MSHV_CREATE_PARTITION
// ioctl and allows for following additional parameters:
//
// @pt_num_cpu_fbanks: Must be set to MSHV_NUM_CPU_FEATURES_BANKS.
// @pt_cpu_fbanks: Disabled processor feature banks array.
// @pt_disabled_xsave: Disabled xsave feature bits.
//
// pt_cpu_fbanks and pt_disabled_xsave are passed through as-is to the create
// partition hypercall.
//
// Returns : same as above original mshv_create_partition
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_create_partition_v2 {
    pub pt_flags: __u64,
    pub pt_isolation: __u64,
    pub pt_num_cpu_fbanks: __u16,
    pub /: *mut *mut __u8 pt_rsvd[6]; / MBZ,
    pub pt_cpu_fbanks: [__u64; MSHV_NUM_CPU_FEATURES_BANKS],
    pub /: *mut *mut __u64 pt_rsvd1[2]; / MBZ,

    pub pt_disabled_xsave: __u64,

    pub /: *mut *mut __u64 pt_rsvd2; / MBZ,

    pub __packed: },
// /dev/mshv

//
// Child partition APIs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_create_vp {
    pub vp_index: __u32,
}

// The hypervisor's "native" page size
pub const MSHV_HV_PAGE_SIZE: c_uint = 0x1000;
//
// struct mshv_user_mem_region - arguments for MSHV_SET_GUEST_MEMORY
// @size: Size of the memory region (bytes). Must be aligned to
// MSHV_HV_PAGE_SIZE
// @guest_pfn: Base guest page number to map
// @userspace_addr: Base address of userspace memory. Must be aligned to
// MSHV_HV_PAGE_SIZE
// @flags: Bitmask of 1 << MSHV_SET_MEM_BIT_*. If (1 << MSHV_SET_MEM_BIT_UNMAP)
// is set, ignore other bits.
// @rsvd: MBZ
//
// Map or unmap a region of userspace memory to Guest Physical Addresses (GPA).
// Mappings can't overlap in GPA space.
// To unmap, these fields must match an existing mapping.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_user_mem_region {
    pub size: __u64,
    pub guest_pfn: __u64,
    pub userspace_addr: __u64,
    pub flags: __u8,
    pub rsvd: [__u8; 7],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_user_irqfd {
    pub fd: __s32,
    pub resamplefd: __s32,
    pub gsi: __u32,
    pub flags: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_user_ioeventfd {
    pub datamatch: __u64,
    pub /: *mut *mut __u64 addr; / legal pio/mmio address,
    pub /: *mut *mut __u32 len; / 1, 2, 4, or 8 bytes,
    pub fd: __s32,
    pub flags: __u32,
    pub rsvd: [__u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_user_irq_entry {
    pub gsi: __u32,
    pub address_lo: __u32,
    pub address_hi: __u32,
    pub data: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_user_irq_table {
    pub nr: __u32,
    pub /: *mut *mut __u32 rsvd; / MBZ,
    pub entries: [mshv_user_irq_entry; ],
}

//
// struct mshv_gpap_access_bitmap - arguments for MSHV_GET_GPAP_ACCESS_BITMAP
// @access_type: MSHV_GPAP_ACCESS_TYPE_* - The type of access to record in the
// bitmap
// @access_op: MSHV_GPAP_ACCESS_OP_* - Allows an optional clear or set of all
// the access states in the range, after retrieving the current
// states.
// @rsvd: MBZ
// @page_count: Number of pages
// @gpap_base: Base gpa page number
// @bitmap_ptr: Output buffer for bitmap, at least (page_count + 7) / 8 bytes
//
// Retrieve a bitmap of either ACCESSED or DIRTY bits for a given range of guest
// memory, and optionally clear or set the bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_gpap_access_bitmap {
    pub access_type: __u8,
    pub access_op: __u8,
    pub rsvd: [__u8; 6],
    pub page_count: __u64,
    pub gpap_base: __u64,
    pub bitmap_ptr: __u64,
}

//
// struct mshv_root_hvcall - arguments for MSHV_ROOT_HVCALL
// @code: Hypercall code (HVCALL_*)
// @reps: in: Rep count ('repcount')
// out: Reps completed ('repcomp'). MBZ unless rep hvcall
// @in_sz: Size of input incl rep data. <= MSHV_HV_PAGE_SIZE
// @out_sz: Size of output buffer. <= MSHV_HV_PAGE_SIZE. MBZ if out_ptr is 0
// @status: in: MBZ
// out: HV_STATUS_* from hypercall
// @rsvd: MBZ
// @in_ptr: Input data buffer (struct hv_input_*). If used with partition or
// vp fd, partition id field is populated by kernel.
// @out_ptr: Output data buffer (optional)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_root_hvcall {
    pub code: __u16,
    pub reps: __u16,
    pub in_sz: __u16,
    pub out_sz: __u16,
    pub status: __u16,
    pub rsvd: [__u8; 6],
    pub in_ptr: __u64,
    pub out_ptr: __u64,
}

// Partition fds created with MSHV_CREATE_PARTITION

// Generic hypercall

//
// VP APIs for child partitions
//
pub const MSHV_RUN_VP_BUF_SZ: c_int = 256;
//
// VP state pages may be mapped to userspace via mmap().
// To specify which state page, use MSHV_VP_MMAP_OFFSET_ values multiplied by
// the system page size.
// e.g.
// long page_size = sysconf(_SC_PAGE_SIZE);
// void *reg_page = mmap(NULL, MSHV_HV_PAGE_SIZE, PROT_READ|PROT_WRITE,
// MAP_SHARED, vp_fd,
// MSHV_VP_MMAP_OFFSET_REGISTERS * page_size);
//
// struct mshv_run_vp - argument for MSHV_RUN_VP
// @msg_buf: On success, the intercept message is copied here. It can be
// interpreted using the relevant hypervisor definitions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_run_vp {
    pub msg_buf: [__u8; MSHV_RUN_VP_BUF_SZ],
}

//
// struct mshv_get_set_vp_state - arguments for MSHV_[GET,SET]_VP_STATE
// @type: MSHV_VP_STATE_
// @rsvd: MBZ
// @buf_sz: in: 4k page-aligned size of buffer
// out: Actual size of data (on EINVAL, check this to see if buffer
// was too small)
// @buf_ptr: 4k page-aligned data buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_get_set_vp_state {
    pub type: __u8,
    pub rsvd: [__u8; 3],
    pub buf_sz: __u32,
    pub buf_ptr: __u64,
}

// VP fds created with MSHV_CREATE_VP

//
// Generic hypercall
// Defined above in partition IOCTLs, avoid redefining it here
// #define MSHV_ROOT_HVCALL			_IOWR(MSHV_IOCTL, 0x07, struct mshv_root_hvcall)
//
// Structure definitions, macros and IOCTLs for mshv_vtl
pub const MSHV_CAP_CORE_API_STABLE: c_uint = 0x0;
pub const MSHV_CAP_REGISTER_PAGE: c_uint = 0x1;
pub const MSHV_CAP_VTL_RETURN_ACTION: c_uint = 0x2;
pub const MSHV_CAP_DR6_SHARED: c_uint = 0x3;
pub const MSHV_MAX_RUN_MSG_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vp_registers {
    pub /: *mut *mut __u32 count; / supports only 1 register at a time,
    pub /: *mut *mut __u32 reserved; / Reserved for alignment or future use,
    pub /: *mut *mut __u64 regs_ptr; / pointer to struct hv_register_assoc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_set_eventfd {
    pub fd: __s32,
    pub flag: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_signal_event {
    pub connection_id: __u32,
    pub flag: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_sint_post_msg {
    pub message_type: __u64,
    pub connection_id: __u32,
    pub /: *mut *mut __u32 payload_size; / Must not exceed HV_MESSAGE_PAYLOAD_BYTE_COUNT,
    pub /: *mut *mut __u64 payload_ptr; / pointer to message payload (bytes),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_ram_disposition {
    pub start_pfn: __u64,
    pub /: *mut *mut __u64 last_pfn; / last_pfn is excluded from the range [start_pfn, last_pfn),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_set_poll_file {
    pub cpu: __u32,
    pub fd: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_hvcall_setup {
    pub /: *mut *mut __u64 bitmap_array_size; / stores number of bytes,
    pub allow_bitmap_ptr: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_vtl_hvcall {
    pub /: *mut *mut __u64 control; / Hypercall control code,
    pub /: *mut *mut __u64 input_size; / Size of the input data,
    pub /: *mut *mut __u64 input_ptr; / Pointer to the input struct,
    pub /: *mut *mut __u64 status; / Status of the hypercall (output),
    pub /: *mut *mut __u64 output_size; / Size of the output data,
    pub /: *mut *mut __u64 output_ptr; / Pointer to the output struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mshv_sint_mask {
    pub mask: __u8,
    pub reserved: [__u8; 7],
}

// /dev/mshv device IOCTL

// vtl device

// VMBus device IOCTLs

// hv_hvcall device

