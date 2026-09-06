//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/microchip/lan966x/lan966x_cbs.c
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

    int lan966x_cbs_add(struct lan966x_port *port,
    struct tc_cbs_qopt_offload *qopt)
    {
    struct lan966x *lan966x = port.lan966x;
    u32 cir, cbs;
    u8 se_idx;
// Check for invalid values
    if (qopt.idleslope <= 0 ||
    qopt.sendslope >= 0 ||
    qopt.locredit >= qopt.hicredit)
    return -EINVAL;
    se_idx = SE_IDX_QUEUE + port.chip_port * NUM_PRIO_QUEUES + qopt.queue;
    cir = qopt.idleslope;
    cbs = (qopt.idleslope - qopt.sendslope) *
    (qopt.hicredit - qopt.locredit) /
    -qopt.sendslope;
// Rate unit is 100 kbps
    cir = DIV_ROUND_UP(cir, 100);
// Avoid using zero rate
    cir = cir ?: 1;
// Burst unit is 4kB
    cbs = DIV_ROUND_UP(cbs, 4096);
// Avoid using zero burst
    cbs = cbs ?: 1;
// Check that actually the result can be written
    if (cir > GENMASK(15, 0) ||
    cbs > GENMASK(6, 0))
    return -EINVAL;
    lan_rmw(QSYS_SE_CFG_SE_AVB_ENA_SET(1) |
    QSYS_SE_CFG_SE_FRM_MODE_SET(1),
    QSYS_SE_CFG_SE_AVB_ENA |
    QSYS_SE_CFG_SE_FRM_MODE,
    lan966x, QSYS_SE_CFG(se_idx));
    lan_wr(QSYS_CIR_CFG_CIR_RATE_SET(cir) |
    QSYS_CIR_CFG_CIR_BURST_SET(cbs),
    lan966x, QSYS_CIR_CFG(se_idx));
    return 0;
    }
    int lan966x_cbs_del(struct lan966x_port *port,
    struct tc_cbs_qopt_offload *qopt)
    {
    struct lan966x *lan966x = port.lan966x;
    u8 se_idx;
    se_idx = SE_IDX_QUEUE + port.chip_port * NUM_PRIO_QUEUES + qopt.queue;
    lan_rmw(QSYS_SE_CFG_SE_AVB_ENA_SET(1) |
    QSYS_SE_CFG_SE_FRM_MODE_SET(0),
    QSYS_SE_CFG_SE_AVB_ENA |
    QSYS_SE_CFG_SE_FRM_MODE,
    lan966x, QSYS_SE_CFG(se_idx));
    lan_wr(QSYS_CIR_CFG_CIR_RATE_SET(0) |
    QSYS_CIR_CFG_CIR_BURST_SET(0),
    lan966x, QSYS_CIR_CFG(se_idx));
    return 0;
    }
