//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/netronome/nfp/nfpcore/nfp_nsp_cmds.c
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017 Netronome Systems, Inc.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nsp_identify {
    pub version: [u8; 40],
    pub flags: u8,
    pub br_primary: u8,
    pub br_secondary: u8,
    pub br_nsp: u8,
    pub primary: __le16,
    pub secondary: __le16,
    pub nsp: __le16,
    pub reserved: [u8; 6],
    pub sensor_mask: __le64,
}

    struct nfp_nsp_identify *__nfp_nsp_identify(struct nfp_nsp *nsp)
    {
    struct nfp_nsp_identify *nspi = core::ptr::null_mut();
    struct nsp_identify *ni;
    int ret;
    if (nfp_nsp_get_abi_ver_minor(nsp) < 15)
    return core::ptr::null_mut();
    ni = kzalloc_obj(*ni);
    if (!ni)
    return core::ptr::null_mut();
    ret = nfp_nsp_read_identify(nsp, ni, sizeof(*ni));
    if (ret < 0) {
    nfp_err(nfp_nsp_cpp(nsp), "reading bsp version failed %d\n",
    ret);
    goto exit_free;
    }
    nspi = kzalloc_obj(*nspi);
    if (!nspi)
    goto exit_free;
    memcpy(nspi.version, ni.version, sizeof(nspi.version));
    nspi.version[sizeof(nspi.version) - 1] = '\0';
    nspi.flags = ni.flags;
    nspi.br_primary = ni.br_primary;
    nspi.br_secondary = ni.br_secondary;
    nspi.br_nsp = ni.br_nsp;
    nspi.primary = le16_to_cpu(ni.primary);
    nspi.secondary = le16_to_cpu(ni.secondary);
    nspi.nsp = le16_to_cpu(ni.nsp);
    nspi.sensor_mask = le64_to_cpu(ni.sensor_mask);
    exit_free:
    kfree(ni);
    return nspi;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nfp_sensors {
    pub chip_temp: __le32,
    pub assembly_power: __le32,
    pub assembly_12v_power: __le32,
    pub assembly_3v3_power: __le32,
}

    int nfp_hwmon_read_sensor(struct nfp_cpp *cpp, enum nfp_nsp_sensor_id id,
    long *val)
    {
    struct nfp_sensors s;
    struct nfp_nsp *nsp;
    int ret;
    nsp = nfp_nsp_open(cpp);
    if (IS_ERR(nsp))
    return PTR_ERR(nsp);
    ret = nfp_nsp_read_sensors(nsp, BIT(id), &s, sizeof(s));
    nfp_nsp_close(nsp);
    if (ret < 0)
    return ret;
    switch (id) {
    case NFP_SENSOR_CHIP_TEMPERATURE:
// val = le32_to_cpu(s.chip_temp);
    break;
    case NFP_SENSOR_ASSEMBLY_POWER:
// val = le32_to_cpu(s.assembly_power);
    break;
    case NFP_SENSOR_ASSEMBLY_12V_POWER:
// val = le32_to_cpu(s.assembly_12v_power);
    break;
    case NFP_SENSOR_ASSEMBLY_3V3_POWER:
// val = le32_to_cpu(s.assembly_3v3_power);
    break;
    default:
    return -EINVAL;
    }
    return 0;
    }
