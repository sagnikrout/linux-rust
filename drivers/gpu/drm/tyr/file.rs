
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

// SPDX-License-Identifier: GPL-2.0 or MIT

use kernel::{
    drm::{
        self,
        Registered, //
    },
    prelude::*,
    uaccess::UserSlice,
    uapi, //
};

use crate::driver::{
    TyrDrmDevice,
    TyrDrmDriver,
    TyrDrmRegistrationData, //
};

#[pin_data]
pub(crate) struct TyrDrmFileData {}

/// Convenience type alias for our DRM `File` type
pub(crate) type TyrDrmFile = drm::file::File<TyrDrmFileData>;

impl drm::file::DriverFile for TyrDrmFileData {
    type Driver = TyrDrmDriver;

    fn open(_dev: &drm::Device<Self::Driver>) -> Result<Pin<KBox<Self>>> {
        KBox::try_pin_init(try_pin_init!(Self {}), GFP_KERNEL)
    }
}

impl TyrDrmFileData {
    pub(crate) fn dev_query(
        _ddev: &TyrDrmDevice<Registered>,
        reg_data: &TyrDrmRegistrationData<'_>,
        devquery: &mut uapi::drm_panthor_dev_query,
        _file: &TyrDrmFile,
    ) -> Result<u32> {
        if devquery.pointer == 0 {
            match devquery.type_ {
                uapi::drm_panthor_dev_query_type_DRM_PANTHOR_DEV_QUERY_GPU_INFO => {
                    devquery.size = core::mem::size_of_val(&reg_data.gpu_info) as u32;
                    Ok(0)
                }
                _ => Err(EINVAL),
            }
        } else {
            match devquery.type_ {
                uapi::drm_panthor_dev_query_type_DRM_PANTHOR_DEV_QUERY_GPU_INFO => {
                    let mut writer = UserSlice::new(
                        UserPtr::from_addr(devquery.pointer as usize),
                        devquery.size as usize,
                    )
                    .writer();

                    writer.write(&reg_data.gpu_info)?;

                    Ok(0)
                }
                _ => Err(EINVAL),
            }
        }
    }
}
