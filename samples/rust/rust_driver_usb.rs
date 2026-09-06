
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
// SPDX-FileCopyrightText: Copyright (C) 2025 Collabora Ltd.

//! Rust USB driver sample.

use kernel::{
    device::{
        self,
        Core, //
    },
    prelude::*,
    sync::aref::ARef,
    usb, //
};

struct SampleDriver {
    _intf: ARef<usb::Interface>,
}

kernel::usb_device_table!(
    USB_TABLE,
    <SampleDriver as usb::Driver>::IdInfo,
    [(usb::DeviceId::from_id(0x1234, 0x5678), ()),]
);

impl usb::Driver for SampleDriver {
    type IdInfo = ();
    type Data<'bound> = Self;
    const ID_TABLE: usb::IdTable<Self::IdInfo> = &USB_TABLE;

    fn probe<'bound>(
        intf: &'bound usb::Interface<Core<'_>>,
        _id: &usb::DeviceId,
        _info: Option<&'bound Self::IdInfo>,
    ) -> impl PinInit<Self, Error> + 'bound {
        let dev: &device::Device<Core<'_>> = intf.as_ref();
        dev_info!(dev, "Rust USB driver sample probed\n");

        Ok(Self { _intf: intf.into() })
    }

    fn disconnect<'bound>(intf: &'bound usb::Interface<Core<'_>>, _data: Pin<&Self>) {
        let dev: &device::Device<Core<'_>> = intf.as_ref();
        dev_info!(dev, "Rust USB driver sample disconnected\n");
    }
}

kernel::module_usb_driver! {
    type: SampleDriver,
    name: "rust_driver_usb",
    authors: ["Daniel Almeida"],
    description: "Rust USB driver sample",
    license: "GPL v2",
}
