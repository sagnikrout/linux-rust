//! Automatically rewritten from C to Rust
//! Source: drivers/perf/riscv_pmu_legacy.c
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
//
// RISC-V performance counter support.
//
// Copyright (C) 2021 Western Digital Corporation or its affiliates.
//
// This implementation is based on old RISC-V perf and ARM perf event code
// which are in turn based on sparc64 and x86 code.
//

pub const RISCV_PMU_LEGACY_CYCLE: c_int = 0;
pub const RISCV_PMU_LEGACY_INSTRET: c_int = 2;
    static bool pmu_init_done;
#[no_mangle]
unsafe extern "C" fn pmu_legacy_ctr_get_idx(event: *mut perf_event) -> c_int {
    static int pmu_legacy_ctr_get_idx(struct perf_event *event)
    {
    struct perf_event_attr *attr = &event.attr;
    if (event.attr.type != PERF_TYPE_HARDWARE)
    return -ENOENT;
    if (attr.config == PERF_COUNT_HW_CPU_CYCLES)
    return RISCV_PMU_LEGACY_CYCLE;
#[no_mangle]
pub unsafe extern "C" fn if(PERF_COUNT_HW_INSTRUCTIONS: attr->config ==) -> else {
    else if (attr.config == PERF_COUNT_HW_INSTRUCTIONS)
    return RISCV_PMU_LEGACY_INSTRET;
    else
    return -ENOENT;
    }
// For legacy config & counter index are same
#[no_mangle]
unsafe extern "C" fn pmu_legacy_event_map(event: *mut perf_event, config: *mut u64) -> c_int {
    static int pmu_legacy_event_map(struct perf_event *event, u64 *config)
    {
    return pmu_legacy_ctr_get_idx(event);
    }
// cycle & instret are always 64 bit, one bit less according to SBI spec
#[no_mangle]
unsafe extern "C" fn pmu_legacy_ctr_get_width(idx: c_int) -> c_int {
    static int pmu_legacy_ctr_get_width(int idx)
    {
    return 63;
    }
#[no_mangle]
unsafe extern "C" fn pmu_legacy_read_ctr(event: *mut perf_event) -> u64 {
    static u64 pmu_legacy_read_ctr(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut idx: c_int = hwc.idx;
    u64 val;
    if (idx == RISCV_PMU_LEGACY_CYCLE) {
    val = riscv_pmu_ctr_read_csr(CSR_CYCLE);
    if (IS_ENABLED(CONFIG_32BIT))
    val = (u64)riscv_pmu_ctr_read_csr(CSR_CYCLEH) << 32 | val;
    } else if (idx == RISCV_PMU_LEGACY_INSTRET) {
    val = riscv_pmu_ctr_read_csr(CSR_INSTRET);
    if (IS_ENABLED(CONFIG_32BIT))
    val = ((u64)riscv_pmu_ctr_read_csr(CSR_INSTRETH)) << 32 | val;
    } else
    return 0;
    return val;
    }
#[no_mangle]
unsafe extern "C" fn pmu_legacy_ctr_start(event: *mut perf_event, ival: u64) {
    static void pmu_legacy_ctr_start(struct perf_event *event, u64 ival)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut initial_val: u64 = pmu_legacy_read_ctr(event);
//
// The legacy method doesn't really have a start/stop method.
// It also can not update the counter with a initial value.
// But we still need to set the prev_count so that read() can compute
// the delta. Just use the current counter value to set the prev_count.
//
    local64_set(&hwc.prev_count, initial_val);
    }
#[no_mangle]
unsafe extern "C" fn pmu_legacy_csr_index(event: *mut perf_event) -> u8 {
    static uint8_t pmu_legacy_csr_index(struct perf_event *event)
    {
    return event.hw.idx;
    }
#[no_mangle]
unsafe extern "C" fn pmu_legacy_event_mapped(event: *mut perf_event, mm: *mut mm_struct) {
    static void pmu_legacy_event_mapped(struct perf_event *event, struct mm_struct *mm)
    {
    if (event.attr.config != PERF_COUNT_HW_CPU_CYCLES &&
    event.attr.config != PERF_COUNT_HW_INSTRUCTIONS)
    return;
    event.hw.flags |= PERF_EVENT_FLAG_USER_READ_CNT;
    }
#[no_mangle]
unsafe extern "C" fn pmu_legacy_event_unmapped(event: *mut perf_event, mm: *mut mm_struct) {
    static void pmu_legacy_event_unmapped(struct perf_event *event, struct mm_struct *mm)
    {
    if (event.attr.config != PERF_COUNT_HW_CPU_CYCLES &&
    event.attr.config != PERF_COUNT_HW_INSTRUCTIONS)
    return;
    event.hw.flags &= ~PERF_EVENT_FLAG_USER_READ_CNT;
    }
//
// This is just a simple implementation to allow legacy implementations
// compatible with new RISC-V PMU driver framework.
// This driver only allows reading two counters i.e CYCLE & INSTRET.
// However, it can not start or stop the counter. Thus, it is not very useful
// will be removed in future.
//
#[no_mangle]
unsafe extern "C" fn pmu_legacy_init(pmu: *mut riscv_pmu) {
    static void pmu_legacy_init(struct riscv_pmu *pmu)
    {
    pr_info("Legacy PMU implementation is available\n");
    pmu.cmask = BIT(RISCV_PMU_LEGACY_CYCLE) |
    BIT(RISCV_PMU_LEGACY_INSTRET);
    pmu.ctr_start = pmu_legacy_ctr_start;
    pmu.ctr_stop = core::ptr::null_mut();
    pmu.event_map = pmu_legacy_event_map;
    pmu.ctr_get_idx = pmu_legacy_ctr_get_idx;
    pmu.ctr_get_width = pmu_legacy_ctr_get_width;
    pmu.ctr_clear_idx = core::ptr::null_mut();
    pmu.ctr_read = pmu_legacy_read_ctr;
    pmu.event_mapped = pmu_legacy_event_mapped;
    pmu.event_unmapped = pmu_legacy_event_unmapped;
    pmu.csr_index = pmu_legacy_csr_index;
    pmu.pmu.capabilities |= PERF_PMU_CAP_NO_INTERRUPT;
    pmu.pmu.capabilities |= PERF_PMU_CAP_NO_EXCLUDE;
    perf_pmu_register(&pmu.pmu, "cpu", PERF_TYPE_RAW);
    }
#[no_mangle]
unsafe extern "C" fn pmu_legacy_device_probe(pdev: *mut platform_device) -> c_int {
    static int pmu_legacy_device_probe(struct platform_device *pdev)
    {
    struct riscv_pmu *pmu = core::ptr::null_mut();
    pmu = riscv_pmu_alloc();
    if (!pmu)
    return -ENOMEM;
    pmu.pmu.parent = &pdev.dev;
    pmu_legacy_init(pmu);
    return 0;
    }
    static struct platform_driver pmu_legacy_driver = {
    .probe		= pmu_legacy_device_probe,
    .driver		= {
    .name	= RISCV_PMU_LEGACY_PDEV_NAME,
    },
    };
#[no_mangle]
unsafe extern "C" fn riscv_pmu_legacy_devinit() -> int __init {
    static int __init riscv_pmu_legacy_devinit(void)
    {
    int ret;
    struct platform_device *pdev;
    if (likely(pmu_init_done))
    return 0;
    ret = platform_driver_register(&pmu_legacy_driver);
    if (ret)
    return ret;
    pdev = platform_device_register_simple(RISCV_PMU_LEGACY_PDEV_NAME, -1, core::ptr::null_mut(), 0);
    if (IS_ERR(pdev)) {
    platform_driver_unregister(&pmu_legacy_driver);
    return PTR_ERR(pdev);
    }
    return ret;
    }
    late_initcall(riscv_pmu_legacy_devinit);
#[no_mangle]
pub unsafe extern "C" fn riscv_pmu_legacy_skip_init() {
    void riscv_pmu_legacy_skip_init(void)
    {
    pmu_init_done = true;
    }
