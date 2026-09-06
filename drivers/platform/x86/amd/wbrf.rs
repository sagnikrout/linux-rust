//! Automatically rewritten from C to Rust
//! Source: drivers/platform/x86/amd/wbrf.c
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
// Wifi Frequency Band Manage Interface
// Copyright (C) 2023 Advanced Micro Devices
//

//
// Functions bit vector for WBRF method
//
// Bit 0: WBRF supported.
// Bit 1: Function 1 (Add / Remove frequency) is supported.
// Bit 2: Function 2 (Get frequency list) is supported.
//
pub const WBRF_ENABLED: c_uint = 0x0;
pub const WBRF_RECORD: c_uint = 0x1;
pub const WBRF_RETRIEVE: c_uint = 0x2;
pub const WBRF_REVISION: c_uint = 0x1;
//
// The data structure used for WBRF_RETRIEVE is not naturally aligned.
// And unfortunately the design has been settled down.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amd_wbrf_ranges_out {
    pub num_of_ranges: u32,
    pub band_list: [freq_band_range; MAX_NUM_OF_WBRF_RANGES],
    pub __packed: },
    static const guid_t wifi_acpi_dsm_guid =
    GUID_INIT(0x7b7656cf, 0xdc3d, 0x4c1c,
    pub 0x70): 0x83, 0xe9, 0x66, 0xe7, 0x21, 0xde, 0x30,,
//
// Used to notify consumer (amdgpu driver currently) about
// the wifi frequency is change.
//
    pub BLOCKING_NOTIFIER_HEAD(wbrf_chain_head): static,
#[no_mangle]
unsafe extern "C" fn wbrf_record(adev: *mut acpi_device, action: u8, in: *mut wbrf_ranges_in_out) -> c_int {
    static int wbrf_record(struct acpi_device *adev, uint8_t action, struct wbrf_ranges_in_out *in)
    {
    pub argv4: union acpi_object,
    pub 0: u32 num_of_ranges =,
    pub num_of_elements: u32,
    pub 0: u32 arg_idx =,
    pub ret: c_int,
    pub i: u32,
    if (!in)
    pub -EINVAL: return,
    pub {: for (i = 0; i < ARRAY_SIZE(in->band_list); i++),
    if (in.band_list[i].start && in.band_list[i].end)
    }
//
// The num_of_ranges value in the "in" object supplied by
// the caller is required to be equal to the number of
// entries in the band_list array in there.
//
    if (num_of_ranges != in.num_of_ranges)
    pub -EINVAL: return,
//
// Every input frequency band comes with two end points(start/end)
// and each is accounted as an element. Meanwhile the range count
// and action type are accounted as an element each.
// So, the total element count = 2 * num_of_ranges + 1 + 1.
//
    pub 2: *mut *mut num_of_elements = 2  num_of_ranges +,
    union acpi_object *tmp __free(kfree) = kzalloc_objs(*tmp,
    if (!tmp)
    pub -ENOMEM: return,
    pub ACPI_TYPE_PACKAGE: argv4.package.type =,
    pub num_of_elements: argv4.package.count =,
    pub tmp: argv4.package.elements =,
// save the number of ranges
    pub ACPI_TYPE_INTEGER: tmp[0].integer.type =,
    pub num_of_ranges: tmp[0].integer.value =,
// save the action(WBRF_RECORD_ADD/REMOVE/RETRIEVE)
    pub ACPI_TYPE_INTEGER: tmp[1].integer.type =,
    pub action: tmp[1].integer.value =,
    pub 2: arg_idx =,
    pub {: for (i = 0; i < ARRAY_SIZE(in->band_list); i++),
    if (!in.band_list[i].start || !in.band_list[i].end)
    pub ACPI_TYPE_INTEGER: tmp[arg_idx].integer.type =,
    pub in->band_list[i].start: tmp[arg_idx++].integer.value =,
    pub ACPI_TYPE_INTEGER: tmp[arg_idx].integer.type =,
    pub in->band_list[i].end: tmp[arg_idx++].integer.value =,
    }
    union acpi_object *obj __free(kfree) =
    acpi_evaluate_dsm(adev.handle, &wifi_acpi_dsm_guid,
    pub &argv4): WBRF_REVISION, WBRF_RECORD,,
    if (!obj)
    pub -EINVAL: return,
    if (obj.type != ACPI_TYPE_INTEGER)
    pub -EINVAL: return,
    pub obj->integer.value: ret =,
    if (ret)
    pub -EINVAL: return,
    pub ret: return,
    }
//
// acpi_amd_wbrf_add_remove - add or remove the frequency band the device is using
//
// @dev: device pointer
// @action: remove or add the frequency band into bios
// @in: input structure containing the frequency band the device is using
//
// Broadcast to other consumers the frequency band the device starts
// to use. Underneath the surface the information is cached into an
// internal buffer first. Then a notification is sent to all those
// registered consumers. So then they can retrieve that buffer to
// know the latest active frequency bands. Consumers that haven't
// yet been registered can retrieve the information from the cache
// when they register.
//
// Return:
// 0 for success add/remove wifi frequency band.
// Returns a negative error code for failure.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_amd_wbrf_add_remove(dev: *mut device, action: u8, in: *mut wbrf_ranges_in_out) -> c_int {
    int acpi_amd_wbrf_add_remove(struct device *dev, uint8_t action, struct wbrf_ranges_in_out *in)
    {
    pub adev: *mut acpi_device,
    pub ret: c_int,
    pub ACPI_COMPANION(dev): adev =,
    if (!adev)
    pub -ENODEV: return,
    pub in): ret = wbrf_record(adev, action,,
    if (ret)
    pub ret: return,
    pub NULL): blocking_notifier_call_chain(&wbrf_chain_head, WBRF_CHANGED,,
    pub 0: return,
    }
//
// acpi_amd_wbrf_supported_producer - determine if the WBRF can be enabled
// for the device as a producer
//
// @dev: device pointer
//
// Check if the platform equipped with necessary implementations to
// support WBRF for the device as a producer.
//
// Return:
// true if WBRF is supported, otherwise returns false
//
#[no_mangle]
pub unsafe extern "C" fn acpi_amd_wbrf_supported_producer(dev: *mut device) -> bool {
    bool acpi_amd_wbrf_supported_producer(struct device *dev)
    {
    pub adev: *mut acpi_device,
    pub ACPI_COMPANION(dev): adev =,
    if (!adev)
    pub false: return,
    return acpi_check_dsm(adev.handle, &wifi_acpi_dsm_guid,
    pub BIT(WBRF_RECORD)): WBRF_REVISION,,
    }
//
// acpi_amd_wbrf_supported_consumer - determine if the WBRF can be enabled
// for the device as a consumer
//
// @dev: device pointer
//
// Determine if the platform equipped with necessary implementations to
// support WBRF for the device as a consumer.
//
// Return:
// true if WBRF is supported, otherwise returns false.
//
#[no_mangle]
pub unsafe extern "C" fn acpi_amd_wbrf_supported_consumer(dev: *mut device) -> bool {
    bool acpi_amd_wbrf_supported_consumer(struct device *dev)
    {
    pub adev: *mut acpi_device,
    pub ACPI_COMPANION(dev): adev =,
    if (!adev)
    pub false: return,
    return acpi_check_dsm(adev.handle, &wifi_acpi_dsm_guid,
    pub BIT(WBRF_RETRIEVE)): WBRF_REVISION,,
    }
//
// amd_wbrf_retrieve_freq_band - retrieve current active frequency bands
//
// @dev: device pointer
// @out: output structure containing all the active frequency bands
//
// Retrieve the current active frequency bands which were broadcasted
// by other producers. The consumer who calls this API should take
// proper actions if any of the frequency band may cause RFI with its
// own frequency band used.
//
// Return:
// 0 for getting wifi freq band successfully.
// Returns a negative error code for failure.
//
#[no_mangle]
pub unsafe extern "C" fn amd_wbrf_retrieve_freq_band(dev: *mut device, out: *mut wbrf_ranges_in_out) -> c_int {
    int amd_wbrf_retrieve_freq_band(struct device *dev, struct wbrf_ranges_in_out *out)
    {
    pub {0}: amd_wbrf_ranges_out acpi_out =,
    pub adev: *mut acpi_device,
    pub obj: *mut union acpi_object,
    pub param: union acpi_object,
    pub 0: int ret =,
    pub ACPI_COMPANION(dev): adev =,
    if (!adev)
    pub -ENODEV: return,
    pub ACPI_TYPE_STRING: param.type =,
    pub 0: param.string.length =,
    pub NULL: param.string.pointer =,
    obj = acpi_evaluate_dsm(adev.handle, &wifi_acpi_dsm_guid,
    pub &param): WBRF_REVISION, WBRF_RETRIEVE,,
    if (!obj)
    pub -EINVAL: return,
//
// The return buffer is with variable length and the format below:
// number_of_entries(1 DWORD):       Number of entries
// start_freq of 1st entry(1 QWORD): Start frequency of the 1st entry
// end_freq of 1st entry(1 QWORD):   End frequency of the 1st entry
// ...
// start_freq of the last entry(1 QWORD)
// end_freq of the last entry(1 QWORD)
//
// Thus the buffer length is determined by the number of entries.
// - For zero entry scenario, the buffer length will be 4 bytes.
// - For one entry scenario, the buffer length will be 20 bytes.
//
    if (obj.buffer.length > sizeof(acpi_out) || obj.buffer.length < 4) {
    pub information"): dev_err(dev, "Wrong sized WBRT,
    pub -EINVAL: ret =,
    pub out: goto,
    }
    pub obj->buffer.length): memcpy(&acpi_out, obj->buffer.pointer,,
    pub acpi_out.num_of_ranges: out->num_of_ranges =,
    pub sizeof(acpi_out.band_list)): memcpy(out->band_list, acpi_out.band_list,,
    out:
    pub ret: return,
    }
//
// amd_wbrf_register_notifier - register for notifications of frequency
// band update
//
// @nb: driver notifier block
//
// The consumer should register itself via this API so that it can get
// notified on the frequency band updates from other producers.
//
// Return:
// 0 for registering a consumer driver successfully.
// Returns a negative error code for failure.
//
#[no_mangle]
pub unsafe extern "C" fn amd_wbrf_register_notifier(nb: *mut notifier_block) -> c_int {
    int amd_wbrf_register_notifier(struct notifier_block *nb)
    {
    pub nb): return blocking_notifier_chain_register(&wbrf_chain_head,,
    }
//
// amd_wbrf_unregister_notifier - unregister for notifications of
// frequency band update
//
// @nb: driver notifier block
//
// The consumer should call this API when it is longer interested with
// the frequency band updates from other producers. Usually, this should
// be performed during driver cleanup.
//
// Return:
// 0 for unregistering a consumer driver.
// Returns a negative error code for failure.
//
#[no_mangle]
pub unsafe extern "C" fn amd_wbrf_unregister_notifier(nb: *mut notifier_block) -> c_int {
    int amd_wbrf_unregister_notifier(struct notifier_block *nb)
    {
    pub nb): return blocking_notifier_chain_unregister(&wbrf_chain_head,,
    }
