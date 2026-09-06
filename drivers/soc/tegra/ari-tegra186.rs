//! Automatically rewritten from C to Rust
//! Source: drivers/soc/tegra/ari-tegra186.c
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
//
// Copyright (c) 2021, NVIDIA CORPORATION.  All rights reserved.
//

pub const SMC_SIP_INVOKE_MCE: c_uint = 0xc2ffff00;
pub const MCE_SMC_READ_MCA: c_int = 12;
pub const MCA_ARI_CMD_RD_SERR: c_int = 1;
pub const MCA_ARI_RW_SUBIDX_STAT: c_int = 1;

pub const MCA_ARI_RW_SUBIDX_ADDR: c_int = 2;
pub const MCA_ARI_RW_SUBIDX_MSC1: c_int = 3;
pub const MCA_ARI_RW_SUBIDX_MSC2: c_int = 4;
    static const char * const bank_names[] = {
    "SYS:DPMU", "ROC:IOB", "ROC:MCB", "ROC:CCE", "ROC:CQX", "ROC:CTU",
    };
#[no_mangle]
unsafe extern "C" fn read_uncore_mca(cmd: u8, idx: u8, subidx: u8, inst: u8, data: *mut u64) {
    static void read_uncore_mca(u8 cmd, u8 idx, u8 subidx, u8 inst, u64 *data)
    {
    struct arm_smccc_res res;
    arm_smccc_smc(SMC_SIP_INVOKE_MCE | MCE_SMC_READ_MCA,
    ((u64)inst << 24) | ((u64)idx << 16) |
    ((u64)subidx << 8) | ((u64)cmd << 0),
    0, 0, 0, 0, 0, 0, &res);
// data = res.a2;
    }
    static int tegra186_ari_panic_handler(struct notifier_block *nb,
    unsigned long code, void *unused)
    {
    u64 status;
    int i;
    for (i = 0; i < ARRAY_SIZE(bank_names); i++) {
    read_uncore_mca(MCA_ARI_CMD_RD_SERR, i, MCA_ARI_RW_SUBIDX_STAT,
    0, &status);
    if (status & SERR_STATUS_VAL) {
    u64 addr, misc1, misc2;
    read_uncore_mca(MCA_ARI_CMD_RD_SERR, i,
    MCA_ARI_RW_SUBIDX_ADDR, 0, &addr);
    read_uncore_mca(MCA_ARI_CMD_RD_SERR, i,
    MCA_ARI_RW_SUBIDX_MSC1, 0, &misc1);
    read_uncore_mca(MCA_ARI_CMD_RD_SERR, i,
    MCA_ARI_RW_SUBIDX_MSC2, 0, &misc2);
    pr_crit("Machine Check Error in %s\n"
    "  status=0x%llx addr=0x%llx\n"
    "  msc1=0x%llx msc2=0x%llx\n",
    bank_names[i], status, addr, misc1, misc2);
    }
    }
    return NOTIFY_DONE;
    }
    static struct notifier_block tegra186_ari_panic_nb = {
    .notifier_call = tegra186_ari_panic_handler,
    };
#[no_mangle]
unsafe extern "C" fn tegra186_ari_init() -> int __init {
    static int __init tegra186_ari_init(void)
    {
    if (of_machine_is_compatible("nvidia,tegra186"))
    atomic_notifier_chain_register(&panic_notifier_list, &tegra186_ari_panic_nb);
    return 0;
    }
    early_initcall(tegra186_ari_init);
