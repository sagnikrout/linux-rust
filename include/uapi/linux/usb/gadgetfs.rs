//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/gadgetfs.h
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
// Filesystem based user-mode API to USB Gadget controller hardware
//
// Other than ep0 operations, most things are done by read() and write()
// on endpoint files found in one directory.  They are configured by
// writing descriptors, and then may be used for normal stream style
// i/o requests.  When ep0 is configured, the device can enumerate;
// when it's closed, the device disconnects from usb.  Operations on
// ep0 require ioctl() operations.
//
// Configuration and device descriptors get written to /dev/gadget/$CHIP,
// which may then be used to read usb_gadgetfs_event structs.  The driver
// may activate endpoints as it handles SET_CONFIGURATION setup events,
// or earlier; writing endpoint descriptors to /dev/gadget/$ENDPOINT
// then performing data transfers by reading or writing.
//

//
// Events are delivered on the ep0 file descriptor, when the user mode driver
// reads from this file descriptor after writing the descriptors.  Don't
// stop polling this descriptor.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum usb_gadgetfs_event_type {
    GADGETFS_NOP = 0,

    GADGETFS_CONNECT,
    GADGETFS_DISCONNECT,
    GADGETFS_SETUP,
    GADGETFS_SUSPEND,
// and likely more !
}

// NOTE:  this structure must stay the same size and layout on
// both 32-bit and 64-bit kernels.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_gadgetfs_event {
// NOP, DISCONNECT, SUSPEND: nothing
// ... some hardware can't report disconnection
//
// CONNECT: just the speed
    pub speed: usb_device_speed,
// SETUP: packet; DATA phase i/o precedes next event
// (setup.bmRequestType & USB_DIR_IN) flags direction
// ... includes SET_CONFIGURATION, SET_INTERFACE
//
    pub setup: usb_ctrlrequest,
    pub u: },
    pub type: usb_gadgetfs_event_type,
}

// The 'g' code is also used by printer and hid gadget ioctl requests.
// Don't add any colliding codes to either driver, and keep
// them in unique ranges (size 0x20 for now).
//
// endpoint ioctls
// IN transfers may be reported to the gadget driver as complete
// when the fifo is loaded, before the host reads the data;
// OUT transfers may be reported to the host's "client" driver as
// complete when they're sitting in the FIFO unread.
// THIS returns how many bytes are "unclaimed" in the endpoint fifo
// (needed for precise fault handling, when the hardware allows it)
//

// discards any unclaimed data in the fifo.

// resets endpoint halt+toggle; used to implement set_interface.
// some hardware (like pxa2xx) can't support this.
//

