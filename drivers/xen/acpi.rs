//! Automatically rewritten from C to Rust
//! Source: drivers/xen/acpi.c
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
// acpi.c
// acpi file for domain 0 kernel
//
// Copyright (c) 2011 Konrad Rzeszutek Wilk <konrad.wilk@oracle.com>
// Copyright (c) 2011 Yu Ke ke.yu@intel.com
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License version 2
// as published by the Free Software Foundation; or, when distributed
// separately from the Linux kernel or incorporated into other
// software packages, subject to the following license:
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this source file (the "Software"), to deal in the Software without
// restriction, including without limitation the rights to use, copy, modify,
// merge, publish, distribute, sublicense, and/or sell copies of the Software,
// and to permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
// IN THE SOFTWARE.
//

    static int xen_acpi_notify_hypervisor_state(u8 sleep_state,
    u32 val_a, u32 val_b,
    bool extended)
    {
    let mut bits: c_uint = extended ? 8 : 16;
    struct xen_platform_op op = {
    .cmd = XENPF_enter_acpi_sleep,
    .interface_version = XENPF_INTERFACE_VERSION,
    .u.enter_acpi_sleep = {
    .val_a = (u16)val_a,
    .val_b = (u16)val_b,
    .sleep_state = sleep_state,
    .flags = extended ? XENPF_ACPI_SLEEP_EXTENDED : 0,
    },
    };
    if (WARN((val_a & (~0 << bits)) || (val_b & (~0 << bits)),
    "Using more than %u bits of sleep control values %#x/%#x!"
    "Email xen-devel@lists.xen.org - Thank you.\n", \
    bits, val_a, val_b))
    return -1;
    HYPERVISOR_platform_op(&op);
    return 1;
    }
    int xen_acpi_notify_hypervisor_sleep(u8 sleep_state,
    u32 pm1a_cnt, u32 pm1b_cnt)
    {
    return xen_acpi_notify_hypervisor_state(sleep_state, pm1a_cnt,
    pm1b_cnt, false);
    }
    int xen_acpi_notify_hypervisor_extended_sleep(u8 sleep_state,
    u32 val_a, u32 val_b)
    {
    return xen_acpi_notify_hypervisor_state(sleep_state, val_a,
    val_b, true);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_prt_entry {
    pub id: acpi_pci_id,
    pub pin: u8,
    pub link: acpi_handle,
    pub index: u32,
}

    int xen_acpi_get_gsi_info(struct pci_dev *dev,
    int *gsi_out,
    int *trigger_out,
    int *polarity_out)
    {
    u32 gsi;
    u8 pin;
    struct acpi_prt_entry *entry;
    let mut trigger: c_int = ACPI_LEVEL_SENSITIVE;
    int ret, polarity = acpi_irq_model == ACPI_IRQ_MODEL_GIC ?
    ACPI_ACTIVE_HIGH : ACPI_ACTIVE_LOW;
    if (!dev || !gsi_out || !trigger_out || !polarity_out)
    return -EINVAL;
    pin = dev.pin;
    if (!pin)
    return -EINVAL;
    entry = acpi_pci_irq_lookup(dev, pin);
    if (entry) {
    ret = 0;
    if (entry.link)
    ret = acpi_pci_link_allocate_irq(entry.link,
    entry.index,
    &trigger, &polarity,
    core::ptr::null_mut(), &gsi);
    else
    gsi = entry.index;
    } else
    ret = -ENODEV;
    if (ret < 0)
    return -EINVAL;
// gsi_out = gsi;
// trigger_out = trigger;
// polarity_out = polarity;
    return 0;
    }
    EXPORT_SYMBOL_GPL(xen_acpi_get_gsi_info);
    static get_gsi_from_sbdf_t get_gsi_from_sbdf;
    static DEFINE_RWLOCK(get_gsi_from_sbdf_lock);
#[no_mangle]
pub unsafe extern "C" fn xen_acpi_register_get_gsi_func(func: get_gsi_from_sbdf_t) {
    void xen_acpi_register_get_gsi_func(get_gsi_from_sbdf_t func)
    {
    write_lock(&get_gsi_from_sbdf_lock);
    get_gsi_from_sbdf = func;
    write_unlock(&get_gsi_from_sbdf_lock);
    }
    EXPORT_SYMBOL_GPL(xen_acpi_register_get_gsi_func);
#[no_mangle]
pub unsafe extern "C" fn xen_acpi_get_gsi_from_sbdf(sbdf: u32) -> c_int {
    int xen_acpi_get_gsi_from_sbdf(u32 sbdf)
    {
    let mut ret: c_int = -EOPNOTSUPP;
    read_lock(&get_gsi_from_sbdf_lock);
    if (get_gsi_from_sbdf)
    ret = get_gsi_from_sbdf(sbdf);
    read_unlock(&get_gsi_from_sbdf_lock);
    return ret;
    }
    EXPORT_SYMBOL_GPL(xen_acpi_get_gsi_from_sbdf);
