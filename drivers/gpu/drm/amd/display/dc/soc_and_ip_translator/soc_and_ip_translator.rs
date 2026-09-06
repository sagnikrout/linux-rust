//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/amd/display/dc/soc_and_ip_translator/soc_and_ip_translator.c
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


// SPDX-License-Identifier: MIT
//
// Copyright 2025 Advanced Micro Devices, Inc.

    static void dc_construct_soc_and_ip_translator(struct soc_and_ip_translator *soc_and_ip_translator,
    enum dce_version dc_version)
    {
    switch (dc_version) {
    case DCN_VERSION_4_01:
    dcn401_construct_soc_and_ip_translator(soc_and_ip_translator);
    break;
    case DCN_VERSION_4_2:
    dcn42_construct_soc_and_ip_translator(soc_and_ip_translator);
    break;
    case DCN_VERSION_4_2B:
    dcn42b_construct_soc_and_ip_translator(soc_and_ip_translator);
    break;
    case DCN_VERSION_6_0:
    dcn60_construct_soc_and_ip_translator(soc_and_ip_translator);
    break;
    default:
    break;
    }
    }
    struct soc_and_ip_translator *dc_create_soc_and_ip_translator(enum dce_version dc_version)
    {
    struct soc_and_ip_translator *soc_and_ip_translator;
    soc_and_ip_translator = kzalloc_obj(*soc_and_ip_translator);
    if (!soc_and_ip_translator)
    return core::ptr::null_mut();
    dc_construct_soc_and_ip_translator(soc_and_ip_translator, dc_version);
    return soc_and_ip_translator;
    }
#[no_mangle]
pub unsafe extern "C" fn dc_destroy_soc_and_ip_translator(soc_and_ip_translator: *mut soc_and_ip_translator) {
    void dc_destroy_soc_and_ip_translator(struct soc_and_ip_translator **soc_and_ip_translator)
    {
    kfree(*soc_and_ip_translator);
// soc_and_ip_translator = NULL;
    }
