//! Automatically rewritten from C to Rust
//! Source: drivers/crypto/intel/qat/qat_common/adf_heartbeat_inject.c
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2023 Intel Corporation

pub const MAX_HB_TICKS: c_uint = 0xFFFFFFFF;
#[no_mangle]
unsafe extern "C" fn adf_hb_set_timer_to_max(accel_dev: *mut adf_accel_dev) -> c_int {
    static int adf_hb_set_timer_to_max(struct adf_accel_dev *accel_dev)
    {
    struct adf_hw_device_data *hw_data = accel_dev.hw_device;
    accel_dev.heartbeat.hb_timer = 0;
    if (hw_data.stop_timer)
    hw_data.stop_timer(accel_dev);
    return adf_send_admin_hb_timer(accel_dev, MAX_HB_TICKS);
    }
    static void adf_set_hb_counters_fail(struct adf_accel_dev *accel_dev, u32 ae,
    u32 thr)
    {
    struct hb_cnt_pair *stats = accel_dev.heartbeat.dma.virt_addr;
    struct adf_hw_device_data *hw_device = accel_dev.hw_device;
    let mut max_aes: usize = hw_device.get_num_aes(hw_device);
    let mut hb_ctrs: usize = hw_device.num_hb_ctrs;
    let mut thr_id: usize = ae * hb_ctrs + thr;
    let mut num_rsp: u16 = stats[thr_id].resp_heartbeat_cnt;
//
// Inject live.req != live.rsp and live.rsp == last.rsp
// to trigger the heartbeat error detection
//
    stats[thr_id].req_heartbeat_cnt++;
    stats += (max_aes * hb_ctrs);
    stats[thr_id].resp_heartbeat_cnt = num_rsp;
    }
#[no_mangle]
pub unsafe extern "C" fn adf_heartbeat_inject_error(accel_dev: *mut adf_accel_dev) -> c_int {
    int adf_heartbeat_inject_error(struct adf_accel_dev *accel_dev)
    {
    struct adf_hw_device_data *hw_device = accel_dev.hw_device;
    let mut max_aes: usize = hw_device.get_num_aes(hw_device);
    let mut hb_ctrs: usize = hw_device.num_hb_ctrs;
    u32 rand, rand_ae, rand_thr;
    unsigned long ae_mask;
    int ret;
    ae_mask = hw_device.ae_mask;
    do {
// Ensure we have a valid ae
    get_random_bytes(&rand, sizeof(rand));
    rand_ae = rand % max_aes;
    } while (!test_bit(rand_ae, &ae_mask));
    get_random_bytes(&rand, sizeof(rand));
    rand_thr = rand % hb_ctrs;
// Increase the heartbeat timer to prevent FW updating HB counters
    ret = adf_hb_set_timer_to_max(accel_dev);
    if (ret)
    return ret;
// Disable arbiter to stop processing any packet
    hw_device.exit_arb(accel_dev);
// Change HB counters memory to simulate a hang
    adf_set_hb_counters_fail(accel_dev, rand_ae, rand_thr);
    return 0;
    }
