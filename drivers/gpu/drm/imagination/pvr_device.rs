//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imagination/pvr_device.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
// Copyright (c) 2023 Imagination Technologies Ltd.

// Forward declaration from <linux/clk.h>.
// Forward declaration from <linux/firmware.h>.
// Forward declaration from <linux/pwrseq/consumer.h>

//
// struct pvr_gpu_id - Hardware GPU ID information for a PowerVR device
// @b: Branch ID.
// @v: Version ID.
// @n: Number of scalable units.
// @c: Config ID.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_gpu_id {
    pub c: u16 b, v, n,,
}

//
// struct pvr_fw_version - Firmware version information
// @major: Major version number.
// @minor: Minor version number.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_fw_version {
    pub minor: u16 major,,
}

//
// struct pvr_device_data - Platform specific data associated with a compatible string.
// @pwr_ops: Pointer to a structure with platform-specific power functions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_device_data {
    pub pwr_ops: *const pvr_power_sequence_ops,
}

//
// struct pvr_device - powervr-specific wrapper for &struct drm_device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_device {
//
// @base: The underlying &struct drm_device.
//
// Do not access this member directly, instead call
// from_pvr_device().
//
    pub base: drm_device,
// @gpu_id: GPU ID detected at runtime.
    pub gpu_id: pvr_gpu_id,
//
// @features: Hardware feature information.
//
// Do not access this member directly, instead use PVR_HAS_FEATURE()
// or PVR_FEATURE_VALUE() macros.
//
    pub features: pvr_device_features,
//
// @quirks: Hardware quirk information.
//
// Do not access this member directly, instead use PVR_HAS_QUIRK().
//
    pub quirks: pvr_device_quirks,
//
// @enhancements: Hardware enhancement information.
//
// Do not access this member directly, instead use
// PVR_HAS_ENHANCEMENT().
//
    pub enhancements: pvr_device_enhancements,
// @fw_version: Firmware version detected at runtime.
    pub fw_version: pvr_fw_version,
// @device_data: Pointer to platform-specific data.
    pub device_data: *const pvr_device_data,
// @regs_resource: Resource representing device control registers.
    pub regs_resource: *mut resource,
//
// @regs: Device control registers.
//
// These are mapped into memory when the device is initialized; that
// location is where this pointer points.
//
    pub regs: *mut void __iomem,
//
// @core_clk: General core clock.
//
// This is the primary clock used by the entire GPU core.
//
    pub core_clk: *mut clk,
//
// @sys_clk: Optional system bus clock.
//
// This may be used on some platforms to provide an independent clock to the SoC Interface
// (SOCIF). If present, this needs to be enabled/disabled together with @core_clk.
//
    pub sys_clk: *mut clk,
//
// @mem_clk: Optional memory clock.
//
// This may be used on some platforms to provide an independent clock to the Memory
// Interface (MEMIF). If present, this needs to be enabled/disabled together with @core_clk.
//
    pub mem_clk: *mut clk,
//
// @power: Optional power domain devices.
//
// On platforms with more than one power domain for the GPU, they are
// stored here in @domains, along with links between them in
// @domain_links. The size of @domain_links is one less than
// struct dev_pm_domain_list->num_pds in @domains.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_device_power {
    pub domains: *mut dev_pm_domain_list,
    pub domain_links: *mut device_link,
    pub power: },
//
// @reset: Optional reset line.
//
// This may be used on some platforms to provide a reset line that needs to be de-asserted
// after power-up procedure. It would also need to be asserted after the power-down
// procedure.
//
    pub reset: *mut reset_control,
// @pwrseq: Pointer to a power sequencer, if one is used.
    pub pwrseq: *mut pwrseq_desc,
// @irq: IRQ number.
    pub irq: c_int,
// @fwccb: Firmware CCB.
    pub fwccb: pvr_ccb,
//
// @kernel_vm_ctx: Virtual memory context used for kernel mappings.
//
// This is used for mappings in the firmware address region when a META firmware processor
// is in use.
//
// When a MIPS firmware processor is in use, this will be %NULL.
//
    pub kernel_vm_ctx: *mut pvr_vm_context,
// @fw_dev: Firmware related data.
    pub fw_dev: pvr_fw_device,
// @stream_musthave_quirks: Bit array of "must-have" quirks for stream commands.
    pub stream_musthave_quirks: [u32; PVR_STREAM_TYPE_MAX][PVR_STREAM_EXTHDR_TYPE_MAX],
//
// @mmu_flush_cache_flags: Records which MMU caches require flushing
// before submitting the next job.
//
    pub mmu_flush_cache_flags: core::sync::atomic::AtomicI32,
//
// @ctx_ids: Array of contexts belonging to this device. Array members
// are of type "struct pvr_context *".
//
// This array is used to allocate IDs used by the firmware.
//
    pub ctx_ids: xarray,
//
// @free_list_ids: Array of free lists belonging to this device. Array members
// are of type "struct pvr_free_list *".
//
// This array is used to allocate IDs used by the firmware.
//
    pub free_list_ids: xarray,
//
// @job_ids: Array of jobs belonging to this device. Array members
// are of type "struct pvr_job *".
//
    pub job_ids: xarray,
//
// @queues: Queue-related fields.
//
// @queues.active: Active queue list.
    pub active: list_head,
// @queues.idle: Idle queue list.
    pub idle: list_head,
// @queues.lock: Lock protecting access to the active/idle
// lists.
    pub lock: mutex,
    pub queues: },
//
// @watchdog: Watchdog for communications with firmware.
//
// @watchdog.work: Work item for watchdog callback.
    pub work: delayed_work,
//
// @watchdog.old_kccb_cmds_executed: KCCB command execution
// count at last watchdog poll.
//
    pub old_kccb_cmds_executed: u32,
//
// @watchdog.kccb_stall_count: Number of watchdog polls
// KCCB has been stalled for.
//
    pub kccb_stall_count: u32,
    pub watchdog: },
//
// @kccb: Circular buffer for communications with firmware.
//
// @kccb.ccb: Kernel CCB.
    pub ccb: pvr_ccb,
// @kccb.rtn_q: Waitqueue for KCCB command return waiters.
    pub rtn_q: wait_queue_head_t,
// @kccb.rtn_obj: Object representing KCCB return slots.
    pub rtn_obj: *mut pvr_fw_object,
//
// @kccb.rtn: Pointer to CPU mapping of KCCB return slots.
// Must be accessed by READ_ONCE()/WRITE_ONCE().
//
    pub rtn: *mut u32,
// @kccb.slot_count: Total number of KCCB slots available.
    pub slot_count: u32,
// @kccb.reserved_count: Number of KCCB slots reserved for
// future use.
    pub reserved_count: u32,
//
// @kccb.waiters: List of KCCB slot waiters.
//
    pub waiters: list_head,
// @kccb.fence_ctx: KCCB fence context.
// @kccb.fence_ctx.id: KCCB fence context ID
// allocated with dma_fence_context_alloc().
    pub id: u64,
// @kccb.fence_ctx.seqno: Sequence number incremented
// each time a fence is created.
    pub seqno: core::sync::atomic::AtomicI32,
//
// @kccb.fence_ctx.lock: Lock used to synchronize
// access to fences allocated by this context.
//
    pub lock: spinlock_t,
    pub fence_ctx: },
    pub kccb: },
//
// @lost: %true if the device has been lost.
//
// This variable is set if the device has become irretrievably unavailable, e.g. if the
// firmware processor has stopped responding and can not be revived via a hard reset.
//
    pub lost: bool,
//
// @reset_sem: Reset semaphore.
//
// GPU reset code will lock this for writing. Any code that submits commands to the firmware
// that isn't in an IRQ handler or on the scheduler workqueue must lock this for reading.
// Once this has been successfully locked, &pvr_dev->lost _must_ be checked, and -%EIO must
// be returned if it is set.
//
    pub reset_sem: rw_semaphore,
// @sched_wq: Workqueue for schedulers.
    pub sched_wq: *mut workqueue_struct,
//
// @ctx_list_lock: Lock to be held when accessing the context list in
// struct pvr_file.
//
    pub ctx_list_lock: spinlock_t,
// @has_safety_events: Whether this device can raise safety events.
    pub has_safety_events: bool,
}

//
// struct pvr_file - powervr-specific data to be assigned to &struct
// drm_file.driver_priv
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pvr_file {
//
// @file: A reference to the parent &struct drm_file.
//
// Do not access this member directly, instead call from_pvr_file().
//
    pub file: *mut drm_file,
//
// @pvr_dev: A reference to the powervr-specific wrapper for the
// associated device. Saves on repeated calls to to_pvr_device().
//
    pub pvr_dev: *mut pvr_device,
//
// @ctx_handles: Array of contexts belonging to this file. Array members
// are of type "struct pvr_context *".
//
// This array is used to allocate handles returned to userspace.
//
    pub ctx_handles: xarray,
//
// @free_list_handles: Array of free lists belonging to this file. Array
// members are of type "struct pvr_free_list *".
//
// This array is used to allocate handles returned to userspace.
//
    pub free_list_handles: xarray,
//
// @hwrt_handles: Array of HWRT datasets belonging to this file. Array
// members are of type "struct pvr_hwrt_dataset *".
//
// This array is used to allocate handles returned to userspace.
//
    pub hwrt_handles: xarray,
//
// @vm_ctx_handles: Array of VM contexts belonging to this file. Array
// members are of type "struct pvr_vm_context *".
//
// This array is used to allocate handles returned to userspace.
//
    pub vm_ctx_handles: xarray,
// @contexts: PVR context list.
    pub contexts: list_head,
}

//
// PVR_HAS_FEATURE() - Tests whether a PowerVR device has a given feature
// @pvr_dev: [IN] Target PowerVR device.
// @feature: [IN] Hardware feature name.
//
// Feature names are derived from those found in &struct pvr_device_features by
// dropping the 'has_' prefix, which is applied by this macro.
//
// Return:
// * true if the named feature is present in the hardware
// * false if the named feature is not present in the hardware
//

//
// PVR_FEATURE_VALUE() - Gets a PowerVR device feature value
// @pvr_dev: [IN] Target PowerVR device.
// @feature: [IN] Feature name.
// @value_out: [OUT] Feature value.
//
// This macro will get a feature value for those features that have values.
// If the feature is not present, nothing will be stored to @value_out.
//
// Feature names are derived from those found in &struct pvr_device_features by
// dropping the 'has_' prefix.
//
// Return:
// * 0 on success, or
// * -%EINVAL if the named feature is not present in the hardware
//

// (value_out) = _pvr_dev->features.feature; \
//
// PVR_HAS_QUIRK() - Tests whether a physical device has a given quirk
// @pvr_dev: [IN] Target PowerVR device.
// @quirk: [IN] Hardware quirk name.
//
// Quirk numbers are derived from those found in #pvr_device_quirks by
// dropping the 'has_brn' prefix, which is applied by this macro.
//
// Returns
// * true if the quirk is present in the hardware, or
// * false if the quirk is not present in the hardware.
//

//
// PVR_HAS_ENHANCEMENT() - Tests whether a physical device has a given
// enhancement
// @pvr_dev: [IN] Target PowerVR device.
// @enhancement: [IN] Hardware enhancement name.
//
// Enhancement numbers are derived from those found in #pvr_device_enhancements
// by dropping the 'has_ern' prefix, which is applied by this macro.
//
// Returns
// * true if the enhancement is present in the hardware, or
// * false if the enhancement is not present in the hardware.
//

//
// PVR_PACKED_BVNC() - Packs B, V, N and C values into a 64-bit unsigned integer
// @b: Branch ID.
// @v: Version ID.
// @n: Number of scalable units.
// @c: Config ID.
//
// The packed layout is as follows:
//
// +--------+--------+--------+-------+
// | 63..48 | 47..32 | 31..16 | 15..0 |
// +========+========+========+=======+
// | B      | V      | N      | C     |
// +--------+--------+--------+-------+
//
// pvr_gpu_id_to_packed_bvnc() should be used instead of this macro when a
// &struct pvr_gpu_id is available in order to ensure proper type checking.
//
// Return: Packed BVNC.
//
// clang-format off

// clang-format on
//
// pvr_gpu_id_to_packed_bvnc() - Packs B, V, N and C values into a 64-bit
// unsigned integer
// @gpu_id: GPU ID.
//
// The packed layout is as follows:
//
// +--------+--------+--------+-------+
// | 63..48 | 47..32 | 31..16 | 15..0 |
// +========+========+========+=======+
// | B      | V      | N      | C     |
// +--------+--------+--------+-------+
//
// This should be used in preference to PVR_PACKED_BVNC() when a &struct
// pvr_gpu_id is available in order to ensure proper type checking.
//
// Return: Packed BVNC.
//
extern "C" {
    pub fn PVR_PACKED_BVNC(_arg: gpu_id->b, _arg: gpu_id->v, _arg: gpu_id->n, _arg: gpu_id->c) -> return;
}
extern "C" {
    pub fn pvr_device_init(pvr_dev: *mut pvr_device) -> c_int;
}
extern "C" {
    pub fn pvr_device_fini(pvr_dev: *mut pvr_device);
}
extern "C" {
    pub fn pvr_device_reset(pvr_dev: *mut pvr_device);
}

//
// PVR_CR_FIELD_GET() - Extract a single field from a PowerVR control register
// @val: Value of the target register.
// @field: Field specifier, as defined in "pvr_rogue_cr_defs.h".
//
// Return: The extracted field.
//

//
// pvr_cr_read32() - Read a 32-bit register from a PowerVR device
// @pvr_dev: Target PowerVR device.
// @reg: Target register.
//
// Return: The value of the requested register.
//
extern "C" {
    pub fn ioread32(reg: pvr_dev->regs +) -> return;
}
//
// pvr_cr_read64() - Read a 64-bit register from a PowerVR device
// @pvr_dev: Target PowerVR device.
// @reg: Target register.
//
// Return: The value of the requested register.
//
extern "C" {
    pub fn ioread64(reg: pvr_dev->regs +) -> return;
}
//
// pvr_cr_write32() - Write to a 32-bit register in a PowerVR device
// @pvr_dev: Target PowerVR device.
// @reg: Target register.
// @val: Value to write.
//
// pvr_cr_write64() - Write to a 64-bit register in a PowerVR device
// @pvr_dev: Target PowerVR device.
// @reg: Target register.
// @val: Value to write.
//
// pvr_cr_poll_reg32() - Wait for a 32-bit register to match a given value by
// polling
// @pvr_dev: Target PowerVR device.
// @reg_addr: Address of register.
// @reg_value: Expected register value (after masking).
// @reg_mask: Mask of bits valid for comparison with @reg_value.
// @timeout_usec: Timeout length, in us.
//
// Returns:
// * 0 on success, or
// * -%ETIMEDOUT on timeout.
//
// pvr_cr_poll_reg64() - Wait for a 64-bit register to match a given value by
// polling
// @pvr_dev: Target PowerVR device.
// @reg_addr: Address of register.
// @reg_value: Expected register value (after masking).
// @reg_mask: Mask of bits valid for comparison with @reg_value.
// @timeout_usec: Timeout length, in us.
//
// Returns:
// * 0 on success, or
// * -%ETIMEDOUT on timeout.
//
// pvr_round_up_to_cacheline_size() - Round up a provided size to be cacheline
// aligned
// @pvr_dev: Target PowerVR device.
// @size: Initial size, in bytes.
//
// Returns:
// * Size aligned to cacheline size.
//
extern "C" {
    pub fn round_up(_arg: size, _arg: slc_cacheline_size_bytes) -> return;
}
//
// DOC: IOCTL validation helpers
//
// To validate the constraints imposed on IOCTL argument structs, a collection
// of macros and helper functions exist in ``pvr_device.h``.
//
// Of the current helpers, it should only be necessary to call
// PVR_IOCTL_UNION_PADDING_CHECK() directly. This macro should be used once in
// every code path which extracts a union member from a struct passed from
// userspace.
//
// pvr_ioctl_union_padding_check() - Validate that the implicit padding between
// the end of a union member and the end of the union itself is zeroed.
// @instance: Pointer to the instance of the struct to validate.
// @union_offset: Offset into the type of @instance of the target union. Must
// be 64-bit aligned.
// @union_size: Size of the target union in the type of @instance. Must be
// 64-bit aligned.
// @member_size: Size of the target member in the target union specified by
// @union_offset and @union_size. It is assumed that the offset of the target
// member is zero relative to @union_offset. Must be 64-bit aligned.
//
// You probably want to use PVR_IOCTL_UNION_PADDING_CHECK() instead of calling
// this function directly, since that macro abstracts away much of the setup,
// and also provides some static validation. See its docs for details.
//
// Return:
// * %true if every byte between the end of the used member of the union and
// the end of that union is zeroed, or
// * %false otherwise.
//
// void pointer arithmetic is technically illegal - cast to a byte
// pointer so this addition works safely.
//
extern "C" {
    pub fn mem_is_zero(_arg: padding_start, _arg: padding_size) -> return;
}
//
// PVR_STATIC_ASSERT_64BIT_ALIGNED() - Inline assertion for 64-bit alignment.
// @static_expr_: Target expression to evaluate.
//
// If @static_expr_ does not evaluate to a constant integer which would be a
// 64-bit aligned address (i.e. a multiple of 8), compilation will fail.
//
// Return:
// The value of @static_expr_.
//

//
// PVR_IOCTL_UNION_PADDING_CHECK() - Validate that the implicit padding between
// the end of a union member and the end of the union itself is zeroed.
// @struct_instance_: An expression which evaluates to a pointer to a UAPI data
// struct.
// @union_: The name of the union member of @struct_instance_ to check. If the
// union member is nested within the type of @struct_instance_, this may
// contain the member access operator (".").
// @member_: The name of the member of @union_ to assess.
//
// This is a wrapper around pvr_ioctl_union_padding_check() which performs
// alignment checks and simplifies things for the caller.
//
// Return:
// * %true if every byte in @struct_instance_ between the end of @member_ and
// the end of @union_ is zeroed, or
// * %false otherwise.
//

//
// These utility functions should more properly be placed in pvr_fw.h, but that
// would cause a dependency cycle between that header and this one. Since
// they're primarily used in pvr_device.c, let's put them in here for now.
//
