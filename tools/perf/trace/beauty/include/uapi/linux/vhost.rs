//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/trace/beauty/include/uapi/linux/vhost.h
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
// Userspace interface for in-kernel virtio accelerators.
// vhost is used to reduce the number of system calls involved in virtio.
//
// Existing virtio net code is used in the guest without modification.
//
// This header includes interface used by userspace hypervisor for
// device configuration.
//

// ioctls
pub const VHOST_VIRTIO: c_uint = 0xAF;
// Features bitmask for forward compatibility.  Transport bits are used for
// vhost specific features.

// Set current process as the (exclusive) owner of this file descriptor.  This
// must be called before any other vhost command.  Further calls to
// VHOST_SET_OWNER fail until VHOST_RESET_OWNER is called.

// Give up ownership, and reset the device to default values.
// Allows subsequent call to VHOST_SET_OWNER to succeed.

// Set up/modify memory layout

// Write logging setup.
// Memory writes can optionally be logged by setting bit at an offset
// (calculated from the physical address) from specified log base.
// The bit is set using an atomic 32 bit operation.
// Set base address for logging.

// Specify an eventfd file descriptor to signal on log write.

// By default, a device gets one vhost_worker that its virtqueues share. This
// command allows the owner of the device to create an additional vhost_worker
// for the device. It can later be bound to 1 or more of its virtqueues using
// the VHOST_ATTACH_VRING_WORKER command.
//
// This must be called after VHOST_SET_OWNER and the caller must be the owner
// of the device. The new thread will inherit caller's cgroups and namespaces,
// and will share the caller's memory space. The new thread will also be
// counted against the caller's RLIMIT_NPROC value.
//
// The worker's ID used in other commands will be returned in
// vhost_worker_state.
//

// Free a worker created with VHOST_NEW_WORKER if it's not attached to any
// virtqueue. If userspace is not able to call this for workers its created,
// the kernel will free all the device's workers when the device is closed.
//

// Ring setup.
// Set number of descriptors in ring. This parameter can not
// be modified while ring is running (bound to a device).

// Set addresses for the ring.

// Base value where queue looks for available descriptors

// Get accessor: reads index, writes value in num

// Set the vring byte order in num. Valid values are VHOST_VRING_LITTLE_ENDIAN
// or VHOST_VRING_BIG_ENDIAN (other values return -EINVAL).
// The byte order cannot be changed while the device is active: trying to do so
// returns -EBUSY.
// This is a legacy only API that is simply ignored when VIRTIO_F_VERSION_1 is
// set.
// Not all kernel configurations support this ioctl, but all configurations that
// support SET also support GET.
//
pub const VHOST_VRING_LITTLE_ENDIAN: c_int = 0;
pub const VHOST_VRING_BIG_ENDIAN: c_int = 1;

// Attach a vhost_worker created with VHOST_NEW_WORKER to one of the device's
// virtqueues.
//
// This will replace the virtqueue's existing worker. If the replaced worker
// is no longer attached to any virtqueues, it can be freed with
// VHOST_FREE_WORKER.
//

// Return the vring worker's ID

// The following ioctls use eventfd file descriptors to signal and poll
// for events.
// Set eventfd to poll for added buffers

// Set eventfd to signal when buffers have beed used

// Set eventfd to signal an error

// Set busy loop timeout (in us)

// Get busy loop timeout (in us)

// Set or get vhost backend capability

// VHOST_NET specific defines
// Attach virtio net ring to a raw socket, or tap device.
// The socket must be already bound to an ethernet device, this device will be
// used for transmit.  Pass fd -1 to unbind from the socket and the transmit
// device.  This can be used to stop the ring (e.g. for migration).

// VHOST_SCSI specific defines

// Changing this breaks userspace.

// Set and get the events missed flag

// VHOST_VSOCK specific defines

// VHOST_VDPA specific defines
// Get the device id. The device ids follow the same definition of
// the device id defined in virtio-spec.
//

// Get and set the status. The status bits follow the same definition
// of the device status defined in virtio-spec.
//

// Get and set the device config. The device config follows the same
// definition of the device config defined in virtio-spec.
//

// Enable/disable the ring.

// Get the max ring size.

// Set event fd for config interrupt

// Get the valid iova range

// Get the config size

// Get the number of address spaces.

// Get the group for a virtqueue: read index, write group in num,
// The virtqueue index is stored in the index field of
// vhost_vring_state. The group for this specific virtqueue is
// returned via num field of vhost_vring_state.
//

// Set the ASID for a virtqueue group. The group index is stored in
// the index field of vhost_vring_state, the ASID associated with this
// group is stored at num field of vhost_vring_state.
//

// Suspend a device so it does not process virtqueue requests anymore
//
// After the return of ioctl the device must preserve all the necessary state
// (the virtqueue vring base plus the possible device specific states) that is
// required for restoring in the future. The device must not change its
// configuration after that point.
//

// Resume a device so it can resume processing virtqueue requests
//
// After the return of this ioctl the device will have restored all the
// necessary states and it is fully operational to continue processing the
// virtqueue descriptors.
//

// Get the group for the descriptor table including driver & device areas
// of a virtqueue: read index, write group in num.
// The virtqueue index is stored in the index field of vhost_vring_state.
// The group ID of the descriptor table for this specific virtqueue
// is returned via num field of vhost_vring_state.
//

// Get the count of all virtqueues

// Get the number of virtqueue groups.

// Get the queue size of a specific virtqueue.
// userspace set the vring index in vhost_vring_state.index
// kernel set the queue size in vhost_vring_state.num
//

// Extended features manipulation

// fork_owner values for vhost
pub const VHOST_FORK_OWNER_KTHREAD: c_int = 0;
pub const VHOST_FORK_OWNER_TASK: c_int = 1;
//
// VHOST_SET_FORK_FROM_OWNER - Set the fork_owner flag for the vhost device,
// This ioctl must called before VHOST_SET_OWNER.
// Only available when CONFIG_VHOST_ENABLE_FORK_OWNER_CONTROL=y
//
// @param fork_owner: An 8-bit value that determines the vhost thread mode
//
// When fork_owner is set to VHOST_FORK_OWNER_TASK(default value):
// - Vhost will create vhost worker as tasks forked from the owner,
// inheriting all of the owner's attributes.
//
// When fork_owner is set to VHOST_FORK_OWNER_KTHREAD:
// - Vhost will create vhost workers as kernel threads.
//

//
// VHOST_GET_FORK_OWNER - Get the current fork_owner flag for the vhost device.
// Only available when CONFIG_VHOST_ENABLE_FORK_OWNER_CONTROL=y
//
// @return: An 8-bit value indicating the current thread mode.
//

