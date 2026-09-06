//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_ets.c
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


// SPDX-License-Identifier: GPL-2.0+

#[no_mangle]
unsafe extern "C" fn lan966x_ets_hw_cost(w_min: u32, weight: u32) -> u32 {
    static u32 lan966x_ets_hw_cost(u32 w_min, u32 weight)
    {
    u32 res;
// Round half up: Multiply with 16 before division,
// add 8 and divide result with 16 again
//
    res = (((DWRR_COST_BIT_WIDTH << 4) * w_min / weight) + 8) >> 4;
    return max_t(u32, 1, res) - 1;
    }
    int lan966x_ets_add(struct lan966x_port *port,
    struct tc_ets_qopt_offload *qopt)
    {
    struct tc_ets_qopt_offload_replace_params *params;
    struct lan966x *lan966x = port.lan966x;
    let mut w_min: u32 = 100;
    let mut count: u8 = 0;
    u32 se_idx;
    u8 i;
// Check the input
    if (qopt.parent != TC_H_ROOT)
    return -EINVAL;
    params = &qopt.replace_params;
    if (params.bands != NUM_PRIO_QUEUES)
    return -EINVAL;
    for (i = 0; i < params.bands; ++i) {
// In the switch the DWRR is always on the lowest consecutive
// priorities. Due to this, the first priority must map to the
// first DWRR band.
//
    if (params.priomap[i] != (7 - i))
    return -EINVAL;
    if (params.quanta[i] && params.weights[i] == 0)
    return -EINVAL;
    }
    se_idx = SE_IDX_PORT + port.chip_port;
// Find minimum weight
    for (i = 0; i < params.bands; ++i) {
    if (params.quanta[i] == 0)
    continue;
    w_min = min(w_min, params.weights[i]);
    }
    for (i = 0; i < params.bands; ++i) {
    if (params.quanta[i] == 0)
    continue;
    ++count;
    lan_wr(lan966x_ets_hw_cost(w_min, params.weights[i]),
    lan966x, QSYS_SE_DWRR_CFG(se_idx, 7 - i));
    }
    lan_rmw(QSYS_SE_CFG_SE_DWRR_CNT_SET(count) |
    QSYS_SE_CFG_SE_RR_ENA_SET(0),
    QSYS_SE_CFG_SE_DWRR_CNT |
    QSYS_SE_CFG_SE_RR_ENA,
    lan966x, QSYS_SE_CFG(se_idx));
    return 0;
    }
    int lan966x_ets_del(struct lan966x_port *port,
    struct tc_ets_qopt_offload *qopt)
    {
    struct lan966x *lan966x = port.lan966x;
    u32 se_idx;
    int i;
    se_idx = SE_IDX_PORT + port.chip_port;
    for (i = 0; i < NUM_PRIO_QUEUES; ++i)
    lan_wr(0, lan966x, QSYS_SE_DWRR_CFG(se_idx, i));
    lan_rmw(QSYS_SE_CFG_SE_DWRR_CNT_SET(0) |
    QSYS_SE_CFG_SE_RR_ENA_SET(0),
    QSYS_SE_CFG_SE_DWRR_CNT |
    QSYS_SE_CFG_SE_RR_ENA,
    lan966x, QSYS_SE_CFG(se_idx));
    return 0;
    }
