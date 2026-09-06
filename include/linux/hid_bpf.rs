//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hid_bpf.h
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


// SPDX-License-Identifier: GPL-2.0+

//
// The following is the user facing HID BPF API.
//
// Extra care should be taken when editing this part, as
// it might break existing out of the tree bpf programs.
//
// struct hid_bpf_ctx - User accessible data for all HID programs
//
// ``data`` is not directly accessible from the context. We need to issue
// a call to hid_bpf_get_data() in order to get a pointer to that field.
//
// @hid: the &struct hid_device representing the device itself
// @allocated_size: Allocated size of data.
//
// This is how much memory is available and can be requested
// by the HID program.
// Note that for ``HID_BPF_RDESC_FIXUP``, that memory is set to
// ``4096`` (4 KB)
// @size: Valid data in the data field.
//
// Programs can get the available valid size in data by fetching this field.
// Programs can also change this value by returning a positive number in the
// program.
// To discard the event, return a negative error code.
//
// ``size`` must always be less or equal than ``allocated_size`` (it is enforced
// once all BPF programs have been run).
// @retval: Return value of the previous program.
//
// ``hid`` and ``allocated_size`` are read-only, ``size`` and ``retval`` are read-write.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_bpf_ctx {
    pub hid: *mut hid_device,
    pub allocated_size: __u32,
    pub retval: __s32,
    pub size: __s32,
}

//
// Below is HID internal
//
pub const HID_BPF_MAX_PROGS_PER_DEV: c_int = 64;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_ops {
    pub data): *const *const *const *const hid_report (hid_get_report)(hid_report_enum report_enum, u8,
    pub from_bpf): u64 source, bool,
    pub from_bpf): u64 source, bool,
    pub lock_already_taken): bool from_bpf, bool,
    pub owner: *mut module,
    pub bus_type: *const bus_type,
}

//
// struct hid_bpf_ops - A BPF struct_ops of callbacks allowing to attach HID-BPF
// programs to a HID device
// @hid_id: the HID uniq ID to attach to. This is writeable before ``load()``, and
// cannot be changed after
// @flags: flags used while attaching the struct_ops to the device. Currently only
// available value is %0 or ``BPF_F_BEFORE``.
// Writeable only before ``load()``
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_bpf_ops {
// hid_id needs to stay first so we can easily change it
// from userspace.
//
    pub hid_id: c_int,
    pub flags: u32,
// private: do not show up in the docs
    pub list: list_head,
// public: rest should show up in the docs
//
// @hid_device_event: called whenever an event is coming in from the device
//
// It has the following arguments:
//
// ``ctx``: The HID-BPF context as &struct hid_bpf_ctx
//
// Return: %0 on success and keep processing; a positive
// value to change the incoming size buffer; a negative
// error code to interrupt the processing of this event
//
// Context: Interrupt context.
//
    pub source): u64,
//
// @hid_rdesc_fixup: called when the probe function parses the report descriptor
// of the HID device
//
// It has the following arguments:
//
// ``ctx``: The HID-BPF context as &struct hid_bpf_ctx
//
// Return: %0 on success and keep processing; a positive
// value to change the incoming size buffer; a negative
// error code to interrupt the processing of this device
//
    pub ctx): *mut *mut int (hid_rdesc_fixup)(struct hid_bpf_ctx,
//
// @hid_hw_request: called whenever a hid_hw_raw_request() call is emitted
// on the HID device
//
// It has the following arguments:
//
// ``ctx``: The HID-BPF context as &struct hid_bpf_ctx
//
// ``reportnum``: the report number, as in hid_hw_raw_request()
//
// ``rtype``: the report type (``HID_INPUT_REPORT``, ``HID_FEATURE_REPORT``,
// ``HID_OUTPUT_REPORT``)
//
// ``reqtype``: the request
//
// ``source``: a u64 referring to a uniq but identifiable source. If %0, the
// kernel itself emitted that call. For hidraw, ``source`` is set
// to the associated ``struct file *``.
//
// Return: %0 to keep processing the request by hid-core; any other value
// stops hid-core from processing that event. A positive value should be
// returned with the number of bytes returned in the incoming buffer; a
// negative error code interrupts the processing of this call.
//
    pub source): u64,
//
// @hid_hw_output_report: called whenever a hid_hw_output_report() call is emitted
// on the HID device
//
// It has the following arguments:
//
// ``ctx``: The HID-BPF context as &struct hid_bpf_ctx
//
// ``source``: a u64 referring to a uniq but identifiable source. If %0, the
// kernel itself emitted that call. For hidraw, ``source`` is set
// to the associated ``struct file *``.
//
// Return: %0 to keep processing the request by hid-core; any other value
// stops hid-core from processing that event. A positive value should be
// returned with the number of bytes written to the device; a negative error
// code interrupts the processing of this call.
//
    pub source): *mut *mut *mut int (hid_hw_output_report)(struct hid_bpf_ctx ctx, u64,
// private: do not show up in the docs
    pub hdev: *mut hid_device,
}

// stored in each device
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hid_bpf {
    pub type: *mut *mut *mut u8 device_data; / allocated when a bpf program of,
// SEC(f.../hid_bpf_device_event) has been attached
// to this HID device
//
    pub allocated_data: u32,
    pub /: *mut *mut bool destroyed; / prevents the assignment of any progs,
    pub rdesc_ops: *mut hid_bpf_ops,
    pub prog_list: list_head,
    pub /: *mut *mut mutex prog_list_lock; / protects prog_list update,
    pub /: *mut *mut srcu_srcu; / protects prog_list read-only access,
}

extern "C" {
    pub fn hid_bpf_connect_device(hdev: *mut hid_device) -> c_int;
}
extern "C" {
    pub fn hid_bpf_disconnect_device(hdev: *mut hid_device);
}
extern "C" {
    pub fn hid_bpf_destroy_device(hid: *mut hid_device);
}
extern "C" {
    pub fn hid_bpf_device_init(hid: *mut hid_device) -> c_int;
}

