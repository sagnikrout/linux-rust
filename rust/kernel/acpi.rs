
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

//! Advanced Configuration and Power Interface abstractions.

use crate::{
    bindings,
    device_id::{RawDeviceId, RawDeviceIdIndex},
    prelude::*,
};

/// IdTable type for ACPI drivers.
pub type IdTable<T> = &'static dyn kernel::device_id::IdTable<DeviceId, T>;

/// An ACPI device id.
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct DeviceId(bindings::acpi_device_id);

// SAFETY: `DeviceId` is a `#[repr(transparent)]` wrapper of `acpi_device_id` and does not add
// additional invariants, so it's safe to transmute to `RawType`.
unsafe impl RawDeviceId for DeviceId {
    type RawType = bindings::acpi_device_id;
}

// SAFETY: `DRIVER_DATA_OFFSET` is the offset to the `driver_data` field.
unsafe impl RawDeviceIdIndex for DeviceId {
    const DRIVER_DATA_OFFSET: usize = core::mem::offset_of!(bindings::acpi_device_id, driver_data);
}

impl DeviceId {
    const ACPI_ID_LEN: usize = 16;

    /// Create a new device id from an ACPI 'id' string.
    #[inline(always)]
    pub const fn new(id: &'static CStr) -> Self {
        let src = id.to_bytes_with_nul();
        build_assert!(src.len() <= Self::ACPI_ID_LEN, "ID exceeds 16 bytes");
        let mut acpi: bindings::acpi_device_id = pin_init::zeroed();
        let mut i = 0;
        while i < src.len() {
            acpi.id[i] = src[i];
            i += 1;
        }

        Self(acpi)
    }
}

/// Create an ACPI `IdTable` with an "alias" for modpost.
#[macro_export]
macro_rules! acpi_device_table {
    ($($tt:tt)*) => {
        $crate::module_device_table!("acpi", $crate::acpi::DeviceId, $($tt)*);
    };
}
