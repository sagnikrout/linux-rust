//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/hisilicon/hibmc/dp/dp_serdes.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2025 Hisilicon Limited.

#[no_mangle]
pub unsafe extern "C" fn hibmc_dp_serdes_set_tx_cfg(dp: *mut hibmc_dp_dev, train_set[HIBMC_DP_LANE_NUM_MAX]: u8) -> c_int {
    int hibmc_dp_serdes_set_tx_cfg(struct hibmc_dp_dev *dp, u8 train_set[HIBMC_DP_LANE_NUM_MAX])
    {
    static const u32 serdes_tx_cfg[4][4] = { {DP_SERDES_VOL0_PRE0, DP_SERDES_VOL0_PRE1,
    DP_SERDES_VOL0_PRE2, DP_SERDES_VOL0_PRE3},
    {DP_SERDES_VOL1_PRE0, DP_SERDES_VOL1_PRE1,
    DP_SERDES_VOL1_PRE2}, {DP_SERDES_VOL2_PRE0,
    DP_SERDES_VOL2_PRE1}, {DP_SERDES_VOL3_PRE0}};
    int cfg[2];
    int i;
    for (i = 0; i < HIBMC_DP_LANE_NUM_MAX; i++) {
    cfg[i] = serdes_tx_cfg[FIELD_GET(DP_TRAIN_VOLTAGE_SWING_MASK, train_set[i])]
    [FIELD_GET(DP_TRAIN_PRE_EMPHASIS_MASK, train_set[i])];
    if (!cfg[i])
    return -EINVAL;
// lane1 offset is 4
    writel(FIELD_PREP(HIBMC_DP_PMA_TXDEEMPH, cfg[i]),
    dp.serdes_base + HIBMC_DP_PMA_LANE0_OFFSET + i * 4);
    }
    usleep_range(300, 500);
    if (readl(dp.serdes_base + HIBMC_DP_LANE_STATUS_OFFSET) != DP_SERDES_DONE) {
    drm_dbg_dp(dp.dev, "dp serdes cfg failed\n");
    return -EAGAIN;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hibmc_dp_serdes_rate_switch(rate: u8, dp: *mut hibmc_dp_dev) -> c_int {
    int hibmc_dp_serdes_rate_switch(u8 rate, struct hibmc_dp_dev *dp)
    {
    writel(rate, dp.serdes_base + HIBMC_DP_LANE0_RATE_OFFSET);
    writel(rate, dp.serdes_base + HIBMC_DP_LANE1_RATE_OFFSET);
    usleep_range(300, 500);
    if (readl(dp.serdes_base + HIBMC_DP_LANE_STATUS_OFFSET) != DP_SERDES_DONE) {
    drm_dbg_dp(dp.dev, "dp serdes rate switching failed\n");
    return -EAGAIN;
    }
    if (rate < DP_SERDES_BW_8_1)
    drm_dbg_dp(dp.dev, "reducing serdes rate to :%d\n",
    rate ? rate * HIBMC_DP_LINK_RATE_CAL * 10 : 162);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn hibmc_dp_serdes_init(dp: *mut hibmc_dp_dev) -> c_int {
    int hibmc_dp_serdes_init(struct hibmc_dp_dev *dp)
    {
    dp.serdes_base = dp.base + HIBMC_DP_HOST_OFFSET;
    writel(FIELD_PREP(HIBMC_DP_PMA_TXDEEMPH, DP_SERDES_VOL0_PRE0),
    dp.serdes_base + HIBMC_DP_PMA_LANE0_OFFSET);
    writel(FIELD_PREP(HIBMC_DP_PMA_TXDEEMPH, DP_SERDES_VOL0_PRE0),
    dp.serdes_base + HIBMC_DP_PMA_LANE1_OFFSET);
    return hibmc_dp_serdes_rate_switch(DP_SERDES_BW_8_1, dp);
    }
