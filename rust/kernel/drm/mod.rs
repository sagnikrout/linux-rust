
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

// SPDX-License-Identifier: GPL-2.0 OR MIT

//! DRM subsystem abstractions.

pub mod device;
pub mod driver;
pub mod file;
pub mod gem;
pub mod gpuvm;
pub mod ioctl;

pub use self::device::Device;
pub use self::device::DeviceContext;
pub use self::device::Ioctl;
pub use self::device::Normal;
pub use self::device::Registered;
pub use self::device::RegistrationGuard;
pub use self::device::UnregisteredDevice;
pub use self::driver::Driver;
pub use self::driver::DriverInfo;
pub use self::driver::Registration;
pub use self::file::File;

pub(crate) mod private {
    pub trait Sealed {}
}
