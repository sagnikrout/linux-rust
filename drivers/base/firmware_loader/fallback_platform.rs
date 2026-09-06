//! Automatically rewritten from C to Rust
//! Source: drivers/base/firmware_loader/fallback_platform.c
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

#[no_mangle]
pub unsafe extern "C" fn firmware_fallback_platform(fw_priv: *mut fw_priv) -> c_int {
    int firmware_fallback_platform(struct fw_priv *fw_priv)
    {
    const u8 *data;
    size_t size;
    int rc;
    if (!(fw_priv.opt_flags & FW_OPT_FALLBACK_PLATFORM))
    return -ENOENT;
    rc = security_kernel_load_data(LOADING_FIRMWARE, true);
    if (rc)
    return rc;
    rc = efi_get_embedded_fw(fw_priv.fw_name, &data, &size);
    if (rc)
    return rc; /* rc == -ENOENT when the fw was not found */
    if (fw_priv.data && size > fw_priv.allocated_size)
    return -ENOMEM;
    rc = security_kernel_post_load_data((u8 *)data, size, LOADING_FIRMWARE,
    "platform");
    if (rc)
    return rc;
    if (!fw_priv.data)
    fw_priv.data = vmalloc(size);
    if (!fw_priv.data)
    return -ENOMEM;
    memcpy(fw_priv.data, data, size);
    fw_priv.size = size;
    fw_state_done(fw_priv);
    return 0;
    }
