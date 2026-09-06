//! Automatically rewritten from C to Rust
//! Source: arch/x86/events/intel/uncore_nhmex.c
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
// Nehalem-EX/Westmere-EX uncore support

// NHM-EX event control
pub const NHMEX_PMON_CTL_EV_SEL_MASK: c_uint = 0x000000ff;
pub const NHMEX_PMON_CTL_UMASK_MASK: c_uint = 0x0000ff00;

pub const NHMEX_PMON_CTL_TRESH_MASK: c_uint = 0xff000000;

    NHMEX_PMON_CTL_UMASK_MASK | \
    NHMEX_PMON_CTL_EDGE_DET | \
    NHMEX_PMON_CTL_INVERT | \
    NHMEX_PMON_CTL_TRESH_MASK)
// NHM-EX Ubox
pub const NHMEX_U_MSR_PMON_GLOBAL_CTL: c_uint = 0xc00;
pub const NHMEX_U_MSR_PMON_CTR: c_uint = 0xc11;
pub const NHMEX_U_MSR_PMON_EV_SEL: c_uint = 0xc10;

pub const NHMEX_U_PMON_GLOBAL_PMI_CORE_SEL: c_uint = 0x0000001e;

    (NHMEX_PMON_CTL_EV_SEL_MASK |	\
    NHMEX_PMON_CTL_EDGE_DET)
// NHM-EX Cbox
pub const NHMEX_C0_MSR_PMON_GLOBAL_CTL: c_uint = 0xd00;
pub const NHMEX_C0_MSR_PMON_CTR0: c_uint = 0xd11;
pub const NHMEX_C0_MSR_PMON_EV_SEL0: c_uint = 0xd10;
pub const NHMEX_C_MSR_OFFSET: c_uint = 0x20;
// NHM-EX Bbox
pub const NHMEX_B0_MSR_PMON_GLOBAL_CTL: c_uint = 0xc20;
pub const NHMEX_B0_MSR_PMON_CTR0: c_uint = 0xc31;
pub const NHMEX_B0_MSR_PMON_CTL0: c_uint = 0xc30;
pub const NHMEX_B_MSR_OFFSET: c_uint = 0x40;
pub const NHMEX_B0_MSR_MATCH: c_uint = 0xe45;
pub const NHMEX_B0_MSR_MASK: c_uint = 0xe46;
pub const NHMEX_B1_MSR_MATCH: c_uint = 0xe4d;
pub const NHMEX_B1_MSR_MASK: c_uint = 0xe4e;

pub const NHMEX_B_PMON_CTL_EV_SEL_SHIFT: c_int = 1;

    (0x1f << NHMEX_B_PMON_CTL_EV_SEL_SHIFT)
pub const NHMEX_B_PMON_CTR_SHIFT: c_int = 6;

    (0x3 << NHMEX_B_PMON_CTR_SHIFT)

    (NHMEX_B_PMON_CTL_EV_SEL_MASK | \
    NHMEX_B_PMON_CTR_MASK)
// NHM-EX Sbox
pub const NHMEX_S0_MSR_PMON_GLOBAL_CTL: c_uint = 0xc40;
pub const NHMEX_S0_MSR_PMON_CTR0: c_uint = 0xc51;
pub const NHMEX_S0_MSR_PMON_CTL0: c_uint = 0xc50;
pub const NHMEX_S_MSR_OFFSET: c_uint = 0x80;
pub const NHMEX_S0_MSR_MM_CFG: c_uint = 0xe48;
pub const NHMEX_S0_MSR_MATCH: c_uint = 0xe49;
pub const NHMEX_S0_MSR_MASK: c_uint = 0xe4a;
pub const NHMEX_S1_MSR_MM_CFG: c_uint = 0xe58;
pub const NHMEX_S1_MSR_MATCH: c_uint = 0xe59;
pub const NHMEX_S1_MSR_MASK: c_uint = 0xe5a;

pub const NHMEX_S_EVENT_TO_R_PROG_EV: c_int = 0;
// NHM-EX Mbox
pub const NHMEX_M0_MSR_GLOBAL_CTL: c_uint = 0xca0;
pub const NHMEX_M0_MSR_PMU_DSP: c_uint = 0xca5;
pub const NHMEX_M0_MSR_PMU_ISS: c_uint = 0xca6;
pub const NHMEX_M0_MSR_PMU_MAP: c_uint = 0xca7;
pub const NHMEX_M0_MSR_PMU_MSC_THR: c_uint = 0xca8;
pub const NHMEX_M0_MSR_PMU_PGT: c_uint = 0xca9;
pub const NHMEX_M0_MSR_PMU_PLD: c_uint = 0xcaa;
pub const NHMEX_M0_MSR_PMU_ZDP_CTL_FVC: c_uint = 0xcab;
pub const NHMEX_M0_MSR_PMU_CTL0: c_uint = 0xcb0;
pub const NHMEX_M0_MSR_PMU_CNT0: c_uint = 0xcb1;
pub const NHMEX_M_MSR_OFFSET: c_uint = 0x40;
pub const NHMEX_M0_MSR_PMU_MM_CFG: c_uint = 0xe54;
pub const NHMEX_M1_MSR_PMU_MM_CFG: c_uint = 0xe5c;

pub const NHMEX_M_PMON_ADDR_MATCH_MASK: c_uint = 0x3ffffffffULL;
pub const NHMEX_M_PMON_ADDR_MASK_MASK: c_uint = 0x7ffffffULL;
pub const NHMEX_M_PMON_ADDR_MASK_SHIFT: c_int = 34;

pub const NHMEX_M_PMON_CTL_COUNT_MODE_SHIFT: c_int = 2;

    (0x3 << NHMEX_M_PMON_CTL_COUNT_MODE_SHIFT)
pub const NHMEX_M_PMON_CTL_STORAGE_MODE_SHIFT: c_int = 4;

    (0x3 << NHMEX_M_PMON_CTL_STORAGE_MODE_SHIFT)

pub const NHMEX_M_PMON_CTL_INC_SEL_SHIFT: c_int = 9;

    (0x1f << NHMEX_M_PMON_CTL_INC_SEL_SHIFT)
pub const NHMEX_M_PMON_CTL_SET_FLAG_SEL_SHIFT: c_int = 19;

    (0x7 << NHMEX_M_PMON_CTL_SET_FLAG_SEL_SHIFT)

    (NHMEX_M_PMON_CTL_COUNT_MODE_MASK |	\
    NHMEX_M_PMON_CTL_STORAGE_MODE_MASK |	\
    NHMEX_M_PMON_CTL_WRAP_MODE |		\
    NHMEX_M_PMON_CTL_FLAG_MODE |		\
    NHMEX_M_PMON_CTL_INC_SEL_MASK |	\
    NHMEX_M_PMON_CTL_SET_FLAG_SEL_MASK)

//
// use the 9~13 bits to select event If the 7th bit is not set,
// otherwise use the 19~21 bits to select event.
//

    NHMEX_M_PMON_CTL_FLAG_MODE)

    NHMEX_M_PMON_CTL_FLAG_MODE)

    NHMEX_M_PMON_CTL_FLAG_MODE)

    EVENT_EXTRA_REG(MBOX_INC_SEL(c), NHMEX_M0_MSR_PMU_##r, \
    MBOX_INC_SEL_MASK, (u64)-1, NHMEX_M_##r)

    EVENT_EXTRA_REG(MBOX_SET_FLAG_SEL(c), NHMEX_M0_MSR_PMU_##r, \
    MBOX_SET_FLAG_SEL_MASK, \
    (u64)-1, NHMEX_M_##r)
// NHM-EX Rbox
pub const NHMEX_R_MSR_GLOBAL_CTL: c_uint = 0xe00;
pub const NHMEX_R_MSR_PMON_CTL0: c_uint = 0xe10;
pub const NHMEX_R_MSR_PMON_CNT0: c_uint = 0xe11;
pub const NHMEX_R_MSR_OFFSET: c_uint = 0x20;

    ((n) < 4 ? (0xe0c + (n)) : (0xe2c + (n) - 4))

    (((n) < 4 ? 0 : 0x10) + (n) * 4)

    (0xe60 + NHMEX_R_MSR_PORTN_XBR_OFFSET(n))

    (NHMEX_R_MSR_PORTN_XBR_SET1_MM_CFG(n) + 1)

    (NHMEX_R_MSR_PORTN_XBR_SET1_MM_CFG(n) + 2)

    (0xe70 + NHMEX_R_MSR_PORTN_XBR_OFFSET(n))

    (NHMEX_R_MSR_PORTN_XBR_SET2_MM_CFG(n) + 1)

    (NHMEX_R_MSR_PORTN_XBR_SET2_MM_CFG(n) + 2)

pub const NHMEX_R_PMON_CTL_EV_SEL_SHIFT: c_int = 1;

    (0x1f << NHMEX_R_PMON_CTL_EV_SEL_SHIFT)

// NHM-EX Wbox
pub const NHMEX_W_MSR_GLOBAL_CTL: c_uint = 0xc80;
pub const NHMEX_W_MSR_PMON_CNT0: c_uint = 0xc90;
pub const NHMEX_W_MSR_PMON_EVT_SEL0: c_uint = 0xc91;
pub const NHMEX_W_MSR_PMON_FIXED_CTR: c_uint = 0x394;
pub const NHMEX_W_MSR_PMON_FIXED_CTL: c_uint = 0x395;

    ((1ULL << (n)) - 1)))
    DEFINE_UNCORE_FORMAT_ATTR(event, event, "config:0-7");
    DEFINE_UNCORE_FORMAT_ATTR(event5, event, "config:1-5");
    DEFINE_UNCORE_FORMAT_ATTR(umask, umask, "config:8-15");
    DEFINE_UNCORE_FORMAT_ATTR(edge, edge, "config:18");
    DEFINE_UNCORE_FORMAT_ATTR(inv, inv, "config:23");
    DEFINE_UNCORE_FORMAT_ATTR(thresh8, thresh, "config:24-31");
    DEFINE_UNCORE_FORMAT_ATTR(counter, counter, "config:6-7");
    DEFINE_UNCORE_FORMAT_ATTR(match, match, "config1:0-63");
    DEFINE_UNCORE_FORMAT_ATTR(mask, mask, "config2:0-63");
#[no_mangle]
unsafe extern "C" fn nhmex_uncore_msr_init_box(box: *mut intel_uncore_box) -> c_int {
    static int nhmex_uncore_msr_init_box(struct intel_uncore_box *box)
    {
    wrmsrq(NHMEX_U_MSR_PMON_GLOBAL_CTL, NHMEX_U_PMON_GLOBAL_EN_ALL);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_uncore_msr_exit_box(box: *mut intel_uncore_box) {
    static void nhmex_uncore_msr_exit_box(struct intel_uncore_box *box)
    {
    wrmsrq(NHMEX_U_MSR_PMON_GLOBAL_CTL, 0);
    }
#[no_mangle]
unsafe extern "C" fn nhmex_uncore_msr_disable_box(box: *mut intel_uncore_box) {
    static void nhmex_uncore_msr_disable_box(struct intel_uncore_box *box)
    {
    let mut msr: unsigned = uncore_msr_box_ctl(box);
    u64 config;
    if (msr) {
    rdmsrq(msr, config);
    config &= ~((1ULL << uncore_num_counters(box)) - 1);
// WBox has a fixed counter
    if (uncore_msr_fixed_ctl(box))
    config &= ~NHMEX_W_PMON_GLOBAL_FIXED_EN;
    wrmsrq(msr, config);
    }
    }
#[no_mangle]
unsafe extern "C" fn nhmex_uncore_msr_enable_box(box: *mut intel_uncore_box) {
    static void nhmex_uncore_msr_enable_box(struct intel_uncore_box *box)
    {
    let mut msr: unsigned = uncore_msr_box_ctl(box);
    u64 config;
    if (msr) {
    rdmsrq(msr, config);
    config |= (1ULL << uncore_num_counters(box)) - 1;
// WBox has a fixed counter
    if (uncore_msr_fixed_ctl(box))
    config |= NHMEX_W_PMON_GLOBAL_FIXED_EN;
    wrmsrq(msr, config);
    }
    }
#[no_mangle]
unsafe extern "C" fn nhmex_uncore_msr_disable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_uncore_msr_disable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    wrmsrq(event.hw.config_base, 0);
    }
#[no_mangle]
unsafe extern "C" fn nhmex_uncore_msr_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_uncore_msr_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    if (hwc.idx == UNCORE_PMC_IDX_FIXED)
    wrmsrq(hwc.config_base, NHMEX_PMON_CTL_EN_BIT0);
#[no_mangle]
pub unsafe extern "C" fn if(NHMEX_PMON_CTL_EN_BIT0: box->pmu->type->event_mask &) -> else {
    else if (box.pmu.type.event_mask & NHMEX_PMON_CTL_EN_BIT0)
    wrmsrq(hwc.config_base, hwc.config | NHMEX_PMON_CTL_EN_BIT22);
    else
    wrmsrq(hwc.config_base, hwc.config | NHMEX_PMON_CTL_EN_BIT0);
    }

    .init_box	= nhmex_uncore_msr_init_box,		\
    .exit_box	= nhmex_uncore_msr_exit_box,		\
    .disable_box	= nhmex_uncore_msr_disable_box,		\
    .enable_box	= nhmex_uncore_msr_enable_box,		\
    .disable_event	= nhmex_uncore_msr_disable_event,	\
    .read_counter	= uncore_msr_read_counter
    static struct intel_uncore_ops nhmex_uncore_ops = {
    NHMEX_UNCORE_OPS_COMMON_INIT(),
    .enable_event	= nhmex_uncore_msr_enable_event,
    };
    static struct attribute *nhmex_uncore_ubox_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_edge.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nhmex_uncore_ubox_format_group = {
    .name		= "format",
    .attrs		= nhmex_uncore_ubox_formats_attr,
    };
    static struct intel_uncore_type nhmex_uncore_ubox = {
    .name		= "ubox",
    .num_counters	= 1,
    .num_boxes	= 1,
    .perf_ctr_bits	= 48,
    .event_ctl	= NHMEX_U_MSR_PMON_EV_SEL,
    .perf_ctr	= NHMEX_U_MSR_PMON_CTR,
    .event_mask	= NHMEX_U_PMON_RAW_EVENT_MASK,
    .box_ctl	= NHMEX_U_MSR_PMON_GLOBAL_CTL,
    .ops		= &nhmex_uncore_ops,
    .format_group	= &nhmex_uncore_ubox_format_group
    };
    static struct attribute *nhmex_uncore_cbox_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_umask.attr,
    &format_attr_edge.attr,
    &format_attr_inv.attr,
    &format_attr_thresh8.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nhmex_uncore_cbox_format_group = {
    .name = "format",
    .attrs = nhmex_uncore_cbox_formats_attr,
    };
// msr offset for each instance of cbox
    static u64 nhmex_cbox_msr_offsets[] = {
    0x0, 0x80, 0x40, 0xc0, 0x20, 0xa0, 0x60, 0xe0, 0x240, 0x2c0,
    };
    static struct intel_uncore_type nhmex_uncore_cbox = {
    .name			= "cbox",
    .num_counters		= 6,
    .num_boxes		= 10,
    .perf_ctr_bits		= 48,
    .event_ctl		= NHMEX_C0_MSR_PMON_EV_SEL0,
    .perf_ctr		= NHMEX_C0_MSR_PMON_CTR0,
    .event_mask		= NHMEX_PMON_RAW_EVENT_MASK,
    .box_ctl		= NHMEX_C0_MSR_PMON_GLOBAL_CTL,
    .msr_offsets		= nhmex_cbox_msr_offsets,
    .pair_ctr_ctl		= 1,
    .ops			= &nhmex_uncore_ops,
    .format_group		= &nhmex_uncore_cbox_format_group
    };
    static struct uncore_event_desc nhmex_uncore_wbox_events[] = {
    INTEL_UNCORE_EVENT_DESC(clockticks, "event=0xff,umask=0"),
    { /* end: all zeroes */ },
    };
    static struct intel_uncore_type nhmex_uncore_wbox = {
    .name			= "wbox",
    .num_counters		= 4,
    .num_boxes		= 1,
    .perf_ctr_bits		= 48,
    .event_ctl		= NHMEX_W_MSR_PMON_CNT0,
    .perf_ctr		= NHMEX_W_MSR_PMON_EVT_SEL0,
    .fixed_ctr		= NHMEX_W_MSR_PMON_FIXED_CTR,
    .fixed_ctl		= NHMEX_W_MSR_PMON_FIXED_CTL,
    .event_mask		= NHMEX_PMON_RAW_EVENT_MASK,
    .box_ctl		= NHMEX_W_MSR_GLOBAL_CTL,
    .pair_ctr_ctl		= 1,
    .event_descs		= nhmex_uncore_wbox_events,
    .ops			= &nhmex_uncore_ops,
    .format_group		= &nhmex_uncore_cbox_format_group
    };
#[no_mangle]
unsafe extern "C" fn nhmex_bbox_hw_config(box: *mut intel_uncore_box, event: *mut perf_event) -> c_int {
    static int nhmex_bbox_hw_config(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    struct hw_perf_event_extra *reg2 = &hwc.branch_reg;
    int ctr, ev_sel;
    ctr = (hwc.config & NHMEX_B_PMON_CTR_MASK) >>
    NHMEX_B_PMON_CTR_SHIFT;
    ev_sel = (hwc.config & NHMEX_B_PMON_CTL_EV_SEL_MASK) >>
    NHMEX_B_PMON_CTL_EV_SEL_SHIFT;
// events that do not use the match/mask registers
    if ((ctr == 0 && ev_sel > 0x3) || (ctr == 1 && ev_sel > 0x6) ||
    (ctr == 2 && ev_sel != 0x4) || ctr == 3)
    return 0;
    if (box.pmu.pmu_idx == 0)
    reg1.reg = NHMEX_B0_MSR_MATCH;
    else
    reg1.reg = NHMEX_B1_MSR_MATCH;
    reg1.idx = 0;
    reg1.config = event.attr.config1;
    reg2.config = event.attr.config2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_bbox_msr_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_bbox_msr_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    struct hw_perf_event_extra *reg2 = &hwc.branch_reg;
    if (reg1.idx != EXTRA_REG_NONE) {
    wrmsrq(reg1.reg, reg1.config);
    wrmsrq(reg1.reg + 1, reg2.config);
    }
    wrmsrq(hwc.config_base, NHMEX_PMON_CTL_EN_BIT0 |
    (hwc.config & NHMEX_B_PMON_CTL_EV_SEL_MASK));
    }
//
// The Bbox has 4 counters, but each counter monitors different events.
// Use bits 6-7 in the event config to select counter.
//
    static struct event_constraint nhmex_uncore_bbox_constraints[] = {
    EVENT_CONSTRAINT(0 , 1, 0xc0),
    EVENT_CONSTRAINT(0x40, 2, 0xc0),
    EVENT_CONSTRAINT(0x80, 4, 0xc0),
    EVENT_CONSTRAINT(0xc0, 8, 0xc0),
    EVENT_CONSTRAINT_END,
    };
    static struct attribute *nhmex_uncore_bbox_formats_attr[] = {
    &format_attr_event5.attr,
    &format_attr_counter.attr,
    &format_attr_match.attr,
    &format_attr_mask.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nhmex_uncore_bbox_format_group = {
    .name = "format",
    .attrs = nhmex_uncore_bbox_formats_attr,
    };
    static struct intel_uncore_ops nhmex_uncore_bbox_ops = {
    NHMEX_UNCORE_OPS_COMMON_INIT(),
    .enable_event		= nhmex_bbox_msr_enable_event,
    .hw_config		= nhmex_bbox_hw_config,
    .get_constraint		= uncore_get_constraint,
    .put_constraint		= uncore_put_constraint,
    };
    static struct intel_uncore_type nhmex_uncore_bbox = {
    .name			= "bbox",
    .num_counters		= 4,
    .num_boxes		= 2,
    .perf_ctr_bits		= 48,
    .event_ctl		= NHMEX_B0_MSR_PMON_CTL0,
    .perf_ctr		= NHMEX_B0_MSR_PMON_CTR0,
    .event_mask		= NHMEX_B_PMON_RAW_EVENT_MASK,
    .box_ctl		= NHMEX_B0_MSR_PMON_GLOBAL_CTL,
    .msr_offset		= NHMEX_B_MSR_OFFSET,
    .pair_ctr_ctl		= 1,
    .num_shared_regs	= 1,
    .constraints		= nhmex_uncore_bbox_constraints,
    .ops			= &nhmex_uncore_bbox_ops,
    .format_group		= &nhmex_uncore_bbox_format_group
    };
#[no_mangle]
unsafe extern "C" fn nhmex_sbox_hw_config(box: *mut intel_uncore_box, event: *mut perf_event) -> c_int {
    static int nhmex_sbox_hw_config(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    struct hw_perf_event_extra *reg2 = &hwc.branch_reg;
// only TO_R_PROG_EV event uses the match/mask register
    if ((hwc.config & NHMEX_PMON_CTL_EV_SEL_MASK) !=
    NHMEX_S_EVENT_TO_R_PROG_EV)
    return 0;
    if (box.pmu.pmu_idx == 0)
    reg1.reg = NHMEX_S0_MSR_MM_CFG;
    else
    reg1.reg = NHMEX_S1_MSR_MM_CFG;
    reg1.idx = 0;
    reg1.config = event.attr.config1;
    reg2.config = event.attr.config2;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_sbox_msr_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_sbox_msr_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    struct hw_perf_event_extra *reg2 = &hwc.branch_reg;
    if (reg1.idx != EXTRA_REG_NONE) {
    wrmsrq(reg1.reg, 0);
    wrmsrq(reg1.reg + 1, reg1.config);
    wrmsrq(reg1.reg + 2, reg2.config);
    wrmsrq(reg1.reg, NHMEX_S_PMON_MM_CFG_EN);
    }
    wrmsrq(hwc.config_base, hwc.config | NHMEX_PMON_CTL_EN_BIT22);
    }
    static struct attribute *nhmex_uncore_sbox_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_umask.attr,
    &format_attr_edge.attr,
    &format_attr_inv.attr,
    &format_attr_thresh8.attr,
    &format_attr_match.attr,
    &format_attr_mask.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nhmex_uncore_sbox_format_group = {
    .name			= "format",
    .attrs			= nhmex_uncore_sbox_formats_attr,
    };
    static struct intel_uncore_ops nhmex_uncore_sbox_ops = {
    NHMEX_UNCORE_OPS_COMMON_INIT(),
    .enable_event		= nhmex_sbox_msr_enable_event,
    .hw_config		= nhmex_sbox_hw_config,
    .get_constraint		= uncore_get_constraint,
    .put_constraint		= uncore_put_constraint,
    };
    static struct intel_uncore_type nhmex_uncore_sbox = {
    .name			= "sbox",
    .num_counters		= 4,
    .num_boxes		= 2,
    .perf_ctr_bits		= 48,
    .event_ctl		= NHMEX_S0_MSR_PMON_CTL0,
    .perf_ctr		= NHMEX_S0_MSR_PMON_CTR0,
    .event_mask		= NHMEX_PMON_RAW_EVENT_MASK,
    .box_ctl		= NHMEX_S0_MSR_PMON_GLOBAL_CTL,
    .msr_offset		= NHMEX_S_MSR_OFFSET,
    .pair_ctr_ctl		= 1,
    .num_shared_regs	= 1,
    .ops			= &nhmex_uncore_sbox_ops,
    .format_group		= &nhmex_uncore_sbox_format_group
    };
    enum {
    EXTRA_REG_NHMEX_M_FILTER,
    EXTRA_REG_NHMEX_M_DSP,
    EXTRA_REG_NHMEX_M_ISS,
    EXTRA_REG_NHMEX_M_MAP,
    EXTRA_REG_NHMEX_M_MSC_THR,
    EXTRA_REG_NHMEX_M_PGT,
    EXTRA_REG_NHMEX_M_PLD,
    EXTRA_REG_NHMEX_M_ZDP_CTL_FVC,
    };
    static struct extra_reg nhmex_uncore_mbox_extra_regs[] = {
    MBOX_INC_SEL_EXTAR_REG(0x0, DSP),
    MBOX_INC_SEL_EXTAR_REG(0x4, MSC_THR),
    MBOX_INC_SEL_EXTAR_REG(0x5, MSC_THR),
    MBOX_INC_SEL_EXTAR_REG(0x9, ISS),
// event 0xa uses two extra registers
    MBOX_INC_SEL_EXTAR_REG(0xa, ISS),
    MBOX_INC_SEL_EXTAR_REG(0xa, PLD),
    MBOX_INC_SEL_EXTAR_REG(0xb, PLD),
// events 0xd ~ 0x10 use the same extra register
    MBOX_INC_SEL_EXTAR_REG(0xd, ZDP_CTL_FVC),
    MBOX_INC_SEL_EXTAR_REG(0xe, ZDP_CTL_FVC),
    MBOX_INC_SEL_EXTAR_REG(0xf, ZDP_CTL_FVC),
    MBOX_INC_SEL_EXTAR_REG(0x10, ZDP_CTL_FVC),
    MBOX_INC_SEL_EXTAR_REG(0x16, PGT),
    MBOX_SET_FLAG_SEL_EXTRA_REG(0x0, DSP),
    MBOX_SET_FLAG_SEL_EXTRA_REG(0x1, ISS),
    MBOX_SET_FLAG_SEL_EXTRA_REG(0x5, PGT),
    MBOX_SET_FLAG_SEL_EXTRA_REG(0x6, MAP),
    EVENT_EXTRA_END
    };
// Nehalem-EX or Westmere-EX ?
    static bool uncore_nhmex;
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_get_shared_reg(box: *mut intel_uncore_box, idx: c_int, config: u64) -> bool {
    static bool nhmex_mbox_get_shared_reg(struct intel_uncore_box *box, int idx, u64 config)
    {
    struct intel_uncore_extra_reg *er;
    unsigned long flags;
    let mut ret: bool = false;
    u64 mask;
    if (idx < EXTRA_REG_NHMEX_M_ZDP_CTL_FVC) {
    er = &box.shared_regs[idx];
    raw_spin_lock_irqsave(&er.lock, flags);
    if (!atomic_read(&er.ref) || er.config == config) {
    atomic_inc(&er.ref);
    er.config = config;
    ret = true;
    }
    raw_spin_unlock_irqrestore(&er.lock, flags);
    return ret;
    }
//
// The ZDP_CTL_FVC MSR has 4 fields which are used to control
// events 0xd ~ 0x10. Besides these 4 fields, there are additional
// fields which are shared.
//
    idx -= EXTRA_REG_NHMEX_M_ZDP_CTL_FVC;
    if (WARN_ON_ONCE(idx >= 4))
    return false;
// mask of the shared fields
    if (uncore_nhmex)
    mask = NHMEX_M_PMON_ZDP_CTL_FVC_MASK;
    else
    mask = WSMEX_M_PMON_ZDP_CTL_FVC_MASK;
    er = &box.shared_regs[EXTRA_REG_NHMEX_M_ZDP_CTL_FVC];
    raw_spin_lock_irqsave(&er.lock, flags);
// add mask of the non-shared field if it's in use
    if (__BITS_VALUE(atomic_read(&er.ref), idx, 8)) {
    if (uncore_nhmex)
    mask |= NHMEX_M_PMON_ZDP_CTL_FVC_EVENT_MASK(idx);
    else
    mask |= WSMEX_M_PMON_ZDP_CTL_FVC_EVENT_MASK(idx);
    }
    if (!atomic_read(&er.ref) || !((er.config ^ config) & mask)) {
    atomic_add(1 << (idx * 8), &er.ref);
    if (uncore_nhmex)
    mask = NHMEX_M_PMON_ZDP_CTL_FVC_MASK |
    NHMEX_M_PMON_ZDP_CTL_FVC_EVENT_MASK(idx);
    else
    mask = WSMEX_M_PMON_ZDP_CTL_FVC_MASK |
    WSMEX_M_PMON_ZDP_CTL_FVC_EVENT_MASK(idx);
    er.config &= ~mask;
    er.config |= (config & mask);
    ret = true;
    }
    raw_spin_unlock_irqrestore(&er.lock, flags);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_put_shared_reg(box: *mut intel_uncore_box, idx: c_int) {
    static void nhmex_mbox_put_shared_reg(struct intel_uncore_box *box, int idx)
    {
    struct intel_uncore_extra_reg *er;
    if (idx < EXTRA_REG_NHMEX_M_ZDP_CTL_FVC) {
    er = &box.shared_regs[idx];
    atomic_dec(&er.ref);
    return;
    }
    idx -= EXTRA_REG_NHMEX_M_ZDP_CTL_FVC;
    er = &box.shared_regs[EXTRA_REG_NHMEX_M_ZDP_CTL_FVC];
    atomic_sub(1 << (idx * 8), &er.ref);
    }
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_alter_er(event: *mut perf_event, new_idx: c_int, modify: bool) -> u64 {
    static u64 nhmex_mbox_alter_er(struct perf_event *event, int new_idx, bool modify)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    u64 idx, orig_idx = __BITS_VALUE(reg1.idx, 0, 8);
    let mut config: u64 = reg1.config;
// get the non-shared control bits and shift them
    idx = orig_idx - EXTRA_REG_NHMEX_M_ZDP_CTL_FVC;
    if (uncore_nhmex)
    config &= NHMEX_M_PMON_ZDP_CTL_FVC_EVENT_MASK(idx);
    else
    config &= WSMEX_M_PMON_ZDP_CTL_FVC_EVENT_MASK(idx);
    if (new_idx > orig_idx) {
    idx = new_idx - orig_idx;
    config <<= 3 * idx;
    } else {
    idx = orig_idx - new_idx;
    config >>= 3 * idx;
    }
// add the shared control bits back
    if (uncore_nhmex)
    config |= NHMEX_M_PMON_ZDP_CTL_FVC_MASK & reg1.config;
    else
    config |= WSMEX_M_PMON_ZDP_CTL_FVC_MASK & reg1.config;
    config |= NHMEX_M_PMON_ZDP_CTL_FVC_MASK & reg1.config;
    if (modify) {
// adjust the main event selector
    if (new_idx > orig_idx)
    hwc.config += idx << NHMEX_M_PMON_CTL_INC_SEL_SHIFT;
    else
    hwc.config -= idx << NHMEX_M_PMON_CTL_INC_SEL_SHIFT;
    reg1.config = config;
    reg1.idx = ~0xff | new_idx;
    }
    return config;
    }
    static struct event_constraint *
    nhmex_mbox_get_constraint(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event_extra *reg1 = &event.hw.extra_reg;
    struct hw_perf_event_extra *reg2 = &event.hw.branch_reg;
    int i, idx[2], alloc = 0;
    let mut config1: u64 = reg1.config;
    idx[0] = __BITS_VALUE(reg1.idx, 0, 8);
    idx[1] = __BITS_VALUE(reg1.idx, 1, 8);
    again:
    for (i = 0; i < 2; i++) {
    if (!uncore_box_is_fake(box) && (reg1.alloc & (0x1 << i)))
    idx[i] = 0xff;
    if (idx[i] == 0xff)
    continue;
    if (!nhmex_mbox_get_shared_reg(box, idx[i],
    __BITS_VALUE(config1, i, 32)))
    goto fail;
    alloc |= (0x1 << i);
    }
// for the match/mask registers
    if (reg2.idx != EXTRA_REG_NONE &&
    (uncore_box_is_fake(box) || !reg2.alloc) &&
    !nhmex_mbox_get_shared_reg(box, reg2.idx, reg2.config))
    goto fail;
//
// If it's a fake box -- as per validate_{group,event}() we
// shouldn't touch event state and we can avoid doing so
// since both will only call get_event_constraints() once
// on each event, this avoids the need for reg->alloc.
//
    if (!uncore_box_is_fake(box)) {
    if (idx[0] != 0xff && idx[0] != __BITS_VALUE(reg1.idx, 0, 8))
    nhmex_mbox_alter_er(event, idx[0], true);
    reg1.alloc |= alloc;
    if (reg2.idx != EXTRA_REG_NONE)
    reg2.alloc = 1;
    }
    return core::ptr::null_mut();
    fail:
    if (idx[0] != 0xff && !(alloc & 0x1) &&
    idx[0] >= EXTRA_REG_NHMEX_M_ZDP_CTL_FVC) {
//
// events 0xd ~ 0x10 are functional identical, but are
// controlled by different fields in the ZDP_CTL_FVC
// register. If we failed to take one field, try the
// rest 3 choices.
//
    BUG_ON(__BITS_VALUE(reg1.idx, 1, 8) != 0xff);
    idx[0] -= EXTRA_REG_NHMEX_M_ZDP_CTL_FVC;
    idx[0] = (idx[0] + 1) % 4;
    idx[0] += EXTRA_REG_NHMEX_M_ZDP_CTL_FVC;
    if (idx[0] != __BITS_VALUE(reg1.idx, 0, 8)) {
    config1 = nhmex_mbox_alter_er(event, idx[0], false);
    goto again;
    }
    }
    if (alloc & 0x1)
    nhmex_mbox_put_shared_reg(box, idx[0]);
    if (alloc & 0x2)
    nhmex_mbox_put_shared_reg(box, idx[1]);
    return &uncore_constraint_empty;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_put_constraint(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_mbox_put_constraint(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event_extra *reg1 = &event.hw.extra_reg;
    struct hw_perf_event_extra *reg2 = &event.hw.branch_reg;
    if (uncore_box_is_fake(box))
    return;
    if (reg1.alloc & 0x1)
    nhmex_mbox_put_shared_reg(box, __BITS_VALUE(reg1.idx, 0, 8));
    if (reg1.alloc & 0x2)
    nhmex_mbox_put_shared_reg(box, __BITS_VALUE(reg1.idx, 1, 8));
    reg1.alloc = 0;
    if (reg2.alloc) {
    nhmex_mbox_put_shared_reg(box, reg2.idx);
    reg2.alloc = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_extra_reg_idx(er: *mut extra_reg) -> c_int {
    static int nhmex_mbox_extra_reg_idx(struct extra_reg *er)
    {
    if (er.idx < EXTRA_REG_NHMEX_M_ZDP_CTL_FVC)
    return er.idx;
    return er.idx + (er.event >> NHMEX_M_PMON_CTL_INC_SEL_SHIFT) - 0xd;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_hw_config(box: *mut intel_uncore_box, event: *mut perf_event) -> c_int {
    static int nhmex_mbox_hw_config(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct intel_uncore_type *type = box.pmu.type;
    struct hw_perf_event_extra *reg1 = &event.hw.extra_reg;
    struct hw_perf_event_extra *reg2 = &event.hw.branch_reg;
    struct extra_reg *er;
    unsigned msr;
    let mut reg_idx: c_int = 0;
//
// The mbox events may require 2 extra MSRs at the most. But only
// the lower 32 bits in these MSRs are significant, so we can use
// config1 to pass two MSRs' config.
//
    for (er = nhmex_uncore_mbox_extra_regs; er.msr; er++) {
    if (er.event != (event.hw.config & er.config_mask))
    continue;
    if (event.attr.config1 & ~er.valid_mask)
    return -EINVAL;
    msr = er.msr + type.msr_offset * box.pmu.pmu_idx;
    if (WARN_ON_ONCE(msr >= 0xffff || er.idx >= 0xff))
    return -EINVAL;
// always use the 32~63 bits to pass the PLD config
    if (er.idx == EXTRA_REG_NHMEX_M_PLD)
    reg_idx = 1;
#[no_mangle]
pub unsafe extern "C" fn if(0): WARN_ON_ONCE(reg_idx >) -> else {
    else if (WARN_ON_ONCE(reg_idx > 0))
    return -EINVAL;
    reg1.idx &= ~(0xff << (reg_idx * 8));
    reg1.reg &= ~(0xffff << (reg_idx * 16));
    reg1.idx |= nhmex_mbox_extra_reg_idx(er) << (reg_idx * 8);
    reg1.reg |= msr << (reg_idx * 16);
    reg1.config = event.attr.config1;
    reg_idx++;
    }
//
// The mbox only provides ability to perform address matching
// for the PLD events.
//
    if (reg_idx == 2) {
    reg2.idx = EXTRA_REG_NHMEX_M_FILTER;
    if (event.attr.config2 & NHMEX_M_PMON_MM_CFG_EN)
    reg2.config = event.attr.config2;
    else
    reg2.config = ~0ULL;
    if (box.pmu.pmu_idx == 0)
    reg2.reg = NHMEX_M0_MSR_PMU_MM_CFG;
    else
    reg2.reg = NHMEX_M1_MSR_PMU_MM_CFG;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_shared_reg_config(box: *mut intel_uncore_box, idx: c_int) -> u64 {
    static u64 nhmex_mbox_shared_reg_config(struct intel_uncore_box *box, int idx)
    {
    struct intel_uncore_extra_reg *er;
    unsigned long flags;
    u64 config;
    if (idx < EXTRA_REG_NHMEX_M_ZDP_CTL_FVC)
    return box.shared_regs[idx].config;
    er = &box.shared_regs[EXTRA_REG_NHMEX_M_ZDP_CTL_FVC];
    raw_spin_lock_irqsave(&er.lock, flags);
    config = er.config;
    raw_spin_unlock_irqrestore(&er.lock, flags);
    return config;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_mbox_msr_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_mbox_msr_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    struct hw_perf_event_extra *reg2 = &hwc.branch_reg;
    int idx;
    idx = __BITS_VALUE(reg1.idx, 0, 8);
    if (idx != 0xff)
    wrmsrq(__BITS_VALUE(reg1.reg, 0, 16),
    nhmex_mbox_shared_reg_config(box, idx));
    idx = __BITS_VALUE(reg1.idx, 1, 8);
    if (idx != 0xff)
    wrmsrq(__BITS_VALUE(reg1.reg, 1, 16),
    nhmex_mbox_shared_reg_config(box, idx));
    if (reg2.idx != EXTRA_REG_NONE) {
    wrmsrq(reg2.reg, 0);
    if (reg2.config != ~0ULL) {
    wrmsrq(reg2.reg + 1,
    reg2.config & NHMEX_M_PMON_ADDR_MATCH_MASK);
    wrmsrq(reg2.reg + 2, NHMEX_M_PMON_ADDR_MASK_MASK &
    (reg2.config >> NHMEX_M_PMON_ADDR_MASK_SHIFT));
    wrmsrq(reg2.reg, NHMEX_M_PMON_MM_CFG_EN);
    }
    }
    wrmsrq(hwc.config_base, hwc.config | NHMEX_PMON_CTL_EN_BIT0);
    }
    DEFINE_UNCORE_FORMAT_ATTR(count_mode,		count_mode,	"config:2-3");
    DEFINE_UNCORE_FORMAT_ATTR(storage_mode,		storage_mode,	"config:4-5");
    DEFINE_UNCORE_FORMAT_ATTR(wrap_mode,		wrap_mode,	"config:6");
    DEFINE_UNCORE_FORMAT_ATTR(flag_mode,		flag_mode,	"config:7");
    DEFINE_UNCORE_FORMAT_ATTR(inc_sel,		inc_sel,	"config:9-13");
    DEFINE_UNCORE_FORMAT_ATTR(set_flag_sel,		set_flag_sel,	"config:19-21");
    DEFINE_UNCORE_FORMAT_ATTR(filter_cfg_en,	filter_cfg_en,	"config2:63");
    DEFINE_UNCORE_FORMAT_ATTR(filter_match,		filter_match,	"config2:0-33");
    DEFINE_UNCORE_FORMAT_ATTR(filter_mask,		filter_mask,	"config2:34-61");
    DEFINE_UNCORE_FORMAT_ATTR(dsp,			dsp,		"config1:0-31");
    DEFINE_UNCORE_FORMAT_ATTR(thr,			thr,		"config1:0-31");
    DEFINE_UNCORE_FORMAT_ATTR(fvc,			fvc,		"config1:0-31");
    DEFINE_UNCORE_FORMAT_ATTR(pgt,			pgt,		"config1:0-31");
    DEFINE_UNCORE_FORMAT_ATTR(map,			map,		"config1:0-31");
    DEFINE_UNCORE_FORMAT_ATTR(iss,			iss,		"config1:0-31");
    DEFINE_UNCORE_FORMAT_ATTR(pld,			pld,		"config1:32-63");
    static struct attribute *nhmex_uncore_mbox_formats_attr[] = {
    &format_attr_count_mode.attr,
    &format_attr_storage_mode.attr,
    &format_attr_wrap_mode.attr,
    &format_attr_flag_mode.attr,
    &format_attr_inc_sel.attr,
    &format_attr_set_flag_sel.attr,
    &format_attr_filter_cfg_en.attr,
    &format_attr_filter_match.attr,
    &format_attr_filter_mask.attr,
    &format_attr_dsp.attr,
    &format_attr_thr.attr,
    &format_attr_fvc.attr,
    &format_attr_pgt.attr,
    &format_attr_map.attr,
    &format_attr_iss.attr,
    &format_attr_pld.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nhmex_uncore_mbox_format_group = {
    .name		= "format",
    .attrs		= nhmex_uncore_mbox_formats_attr,
    };
    static struct uncore_event_desc nhmex_uncore_mbox_events[] = {
    INTEL_UNCORE_EVENT_DESC(bbox_cmds_read, "inc_sel=0xd,fvc=0x2800"),
    INTEL_UNCORE_EVENT_DESC(bbox_cmds_write, "inc_sel=0xd,fvc=0x2820"),
    { /* end: all zeroes */ },
    };
    static struct uncore_event_desc wsmex_uncore_mbox_events[] = {
    INTEL_UNCORE_EVENT_DESC(bbox_cmds_read, "inc_sel=0xd,fvc=0x5000"),
    INTEL_UNCORE_EVENT_DESC(bbox_cmds_write, "inc_sel=0xd,fvc=0x5040"),
    { /* end: all zeroes */ },
    };
    static struct intel_uncore_ops nhmex_uncore_mbox_ops = {
    NHMEX_UNCORE_OPS_COMMON_INIT(),
    .enable_event	= nhmex_mbox_msr_enable_event,
    .hw_config	= nhmex_mbox_hw_config,
    .get_constraint	= nhmex_mbox_get_constraint,
    .put_constraint	= nhmex_mbox_put_constraint,
    };
    static struct intel_uncore_type nhmex_uncore_mbox = {
    .name			= "mbox",
    .num_counters		= 6,
    .num_boxes		= 2,
    .perf_ctr_bits		= 48,
    .event_ctl		= NHMEX_M0_MSR_PMU_CTL0,
    .perf_ctr		= NHMEX_M0_MSR_PMU_CNT0,
    .event_mask		= NHMEX_M_PMON_RAW_EVENT_MASK,
    .box_ctl		= NHMEX_M0_MSR_GLOBAL_CTL,
    .msr_offset		= NHMEX_M_MSR_OFFSET,
    .pair_ctr_ctl		= 1,
    .num_shared_regs	= 8,
    .event_descs		= nhmex_uncore_mbox_events,
    .ops			= &nhmex_uncore_mbox_ops,
    .format_group		= &nhmex_uncore_mbox_format_group,
    };
#[no_mangle]
unsafe extern "C" fn nhmex_rbox_alter_er(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_rbox_alter_er(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
// adjust the main event selector and extra register index
    if (reg1.idx % 2) {
    reg1.idx--;
    hwc.config -= 1 << NHMEX_R_PMON_CTL_EV_SEL_SHIFT;
    } else {
    reg1.idx++;
    hwc.config += 1 << NHMEX_R_PMON_CTL_EV_SEL_SHIFT;
    }
// adjust extra register config
    switch (reg1.idx % 6) {
    case 2:
// shift the 8~15 bits to the 0~7 bits
    reg1.config >>= 8;
    break;
    case 3:
// shift the 0~7 bits to the 8~15 bits
    reg1.config <<= 8;
    break;
    }
    }
//
// Each rbox has 4 event set which monitor PQI port 0~3 or 4~7.
// An event set consists of 6 events, the 3rd and 4th events in
// an event set use the same extra register. So an event set uses
// 5 extra registers.
//
    static struct event_constraint *
    nhmex_rbox_get_constraint(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    struct hw_perf_event_extra *reg2 = &hwc.branch_reg;
    struct intel_uncore_extra_reg *er;
    unsigned long flags;
    int idx, er_idx;
    u64 config1;
    let mut ok: bool = false;
    if (!uncore_box_is_fake(box) && reg1.alloc)
    return core::ptr::null_mut();
    idx = reg1.idx % 6;
    config1 = reg1.config;
    again:
    er_idx = idx;
// the 3rd and 4th events use the same extra register
    if (er_idx > 2)
    er_idx--;
    er_idx += (reg1.idx / 6) * 5;
    er = &box.shared_regs[er_idx];
    raw_spin_lock_irqsave(&er.lock, flags);
    if (idx < 2) {
    if (!atomic_read(&er.ref) || er.config == reg1.config) {
    atomic_inc(&er.ref);
    er.config = reg1.config;
    ok = true;
    }
    } else if (idx == 2 || idx == 3) {
//
// these two events use different fields in a extra register,
// the 0~7 bits and the 8~15 bits respectively.
//
    let mut mask: u64 = 0xff << ((idx - 2) * 8);
    if (!__BITS_VALUE(atomic_read(&er.ref), idx - 2, 8) ||
    !((er.config ^ config1) & mask)) {
    atomic_add(1 << ((idx - 2) * 8), &er.ref);
    er.config &= ~mask;
    er.config |= config1 & mask;
    ok = true;
    }
    } else {
    if (!atomic_read(&er.ref) ||
    (er.config == (hwc.config >> 32) &&
    er.config1 == reg1.config &&
    er.config2 == reg2.config)) {
    atomic_inc(&er.ref);
    er.config = (hwc.config >> 32);
    er.config1 = reg1.config;
    er.config2 = reg2.config;
    ok = true;
    }
    }
    raw_spin_unlock_irqrestore(&er.lock, flags);
    if (!ok) {
//
// The Rbox events are always in pairs. The paired
// events are functional identical, but use different
// extra registers. If we failed to take an extra
// register, try the alternative.
//
    idx ^= 1;
    if (idx != reg1.idx % 6) {
    if (idx == 2)
    config1 >>= 8;
#[no_mangle]
pub unsafe extern "C" fn if(3: idx ==) -> else {
    else if (idx == 3)
    config1 <<= 8;
    goto again;
    }
    } else {
    if (!uncore_box_is_fake(box)) {
    if (idx != reg1.idx % 6)
    nhmex_rbox_alter_er(box, event);
    reg1.alloc = 1;
    }
    return core::ptr::null_mut();
    }
    return &uncore_constraint_empty;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_rbox_put_constraint(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_rbox_put_constraint(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct intel_uncore_extra_reg *er;
    struct hw_perf_event_extra *reg1 = &event.hw.extra_reg;
    int idx, er_idx;
    if (uncore_box_is_fake(box) || !reg1.alloc)
    return;
    idx = reg1.idx % 6;
    er_idx = idx;
    if (er_idx > 2)
    er_idx--;
    er_idx += (reg1.idx / 6) * 5;
    er = &box.shared_regs[er_idx];
    if (idx == 2 || idx == 3)
    atomic_sub(1 << ((idx - 2) * 8), &er.ref);
    else
    atomic_dec(&er.ref);
    reg1.alloc = 0;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_rbox_hw_config(box: *mut intel_uncore_box, event: *mut perf_event) -> c_int {
    static int nhmex_rbox_hw_config(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &event.hw.extra_reg;
    struct hw_perf_event_extra *reg2 = &event.hw.branch_reg;
    int idx;
    idx = (event.hw.config & NHMEX_R_PMON_CTL_EV_SEL_MASK) >>
    NHMEX_R_PMON_CTL_EV_SEL_SHIFT;
    if (idx >= 0x18)
    return -EINVAL;
    reg1.idx = idx;
    reg1.config = event.attr.config1;
    switch (idx % 6) {
    case 4:
    case 5:
    hwc.config |= event.attr.config & (~0ULL << 32);
    reg2.config = event.attr.config2;
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nhmex_rbox_msr_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhmex_rbox_msr_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    struct hw_perf_event_extra *reg1 = &hwc.extra_reg;
    struct hw_perf_event_extra *reg2 = &hwc.branch_reg;
    int idx, port;
    idx = reg1.idx;
    port = idx / 6 + box.pmu.pmu_idx * 4;
    switch (idx % 6) {
    case 0:
    wrmsrq(NHMEX_R_MSR_PORTN_IPERF_CFG0(port), reg1.config);
    break;
    case 1:
    wrmsrq(NHMEX_R_MSR_PORTN_IPERF_CFG1(port), reg1.config);
    break;
    case 2:
    case 3:
    wrmsrq(NHMEX_R_MSR_PORTN_QLX_CFG(port),
    uncore_shared_reg_config(box, 2 + (idx / 6) * 5));
    break;
    case 4:
    wrmsrq(NHMEX_R_MSR_PORTN_XBR_SET1_MM_CFG(port),
    hwc.config >> 32);
    wrmsrq(NHMEX_R_MSR_PORTN_XBR_SET1_MATCH(port), reg1.config);
    wrmsrq(NHMEX_R_MSR_PORTN_XBR_SET1_MASK(port), reg2.config);
    break;
    case 5:
    wrmsrq(NHMEX_R_MSR_PORTN_XBR_SET2_MM_CFG(port),
    hwc.config >> 32);
    wrmsrq(NHMEX_R_MSR_PORTN_XBR_SET2_MATCH(port), reg1.config);
    wrmsrq(NHMEX_R_MSR_PORTN_XBR_SET2_MASK(port), reg2.config);
    break;
    }
    wrmsrq(hwc.config_base, NHMEX_PMON_CTL_EN_BIT0 |
    (hwc.config & NHMEX_R_PMON_CTL_EV_SEL_MASK));
    }
    DEFINE_UNCORE_FORMAT_ATTR(xbr_mm_cfg, xbr_mm_cfg, "config:32-63");
    DEFINE_UNCORE_FORMAT_ATTR(xbr_match, xbr_match, "config1:0-63");
    DEFINE_UNCORE_FORMAT_ATTR(xbr_mask, xbr_mask, "config2:0-63");
    DEFINE_UNCORE_FORMAT_ATTR(qlx_cfg, qlx_cfg, "config1:0-15");
    DEFINE_UNCORE_FORMAT_ATTR(iperf_cfg, iperf_cfg, "config1:0-31");
    static struct attribute *nhmex_uncore_rbox_formats_attr[] = {
    &format_attr_event5.attr,
    &format_attr_xbr_mm_cfg.attr,
    &format_attr_xbr_match.attr,
    &format_attr_xbr_mask.attr,
    &format_attr_qlx_cfg.attr,
    &format_attr_iperf_cfg.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nhmex_uncore_rbox_format_group = {
    .name = "format",
    .attrs = nhmex_uncore_rbox_formats_attr,
    };
    static struct uncore_event_desc nhmex_uncore_rbox_events[] = {
    INTEL_UNCORE_EVENT_DESC(qpi0_flit_send,		"event=0x0,iperf_cfg=0x80000000"),
    INTEL_UNCORE_EVENT_DESC(qpi1_filt_send,		"event=0x6,iperf_cfg=0x80000000"),
    INTEL_UNCORE_EVENT_DESC(qpi0_idle_filt,		"event=0x0,iperf_cfg=0x40000000"),
    INTEL_UNCORE_EVENT_DESC(qpi1_idle_filt,		"event=0x6,iperf_cfg=0x40000000"),
    INTEL_UNCORE_EVENT_DESC(qpi0_date_response,	"event=0x0,iperf_cfg=0xc4"),
    INTEL_UNCORE_EVENT_DESC(qpi1_date_response,	"event=0x6,iperf_cfg=0xc4"),
    { /* end: all zeroes */ },
    };
    static struct intel_uncore_ops nhmex_uncore_rbox_ops = {
    NHMEX_UNCORE_OPS_COMMON_INIT(),
    .enable_event		= nhmex_rbox_msr_enable_event,
    .hw_config		= nhmex_rbox_hw_config,
    .get_constraint		= nhmex_rbox_get_constraint,
    .put_constraint		= nhmex_rbox_put_constraint,
    };
    static struct intel_uncore_type nhmex_uncore_rbox = {
    .name			= "rbox",
    .num_counters		= 8,
    .num_boxes		= 2,
    .perf_ctr_bits		= 48,
    .event_ctl		= NHMEX_R_MSR_PMON_CTL0,
    .perf_ctr		= NHMEX_R_MSR_PMON_CNT0,
    .event_mask		= NHMEX_R_PMON_RAW_EVENT_MASK,
    .box_ctl		= NHMEX_R_MSR_GLOBAL_CTL,
    .msr_offset		= NHMEX_R_MSR_OFFSET,
    .pair_ctr_ctl		= 1,
    .num_shared_regs	= 20,
    .event_descs		= nhmex_uncore_rbox_events,
    .ops			= &nhmex_uncore_rbox_ops,
    .format_group		= &nhmex_uncore_rbox_format_group
    };
    static struct intel_uncore_type *nhmex_msr_uncores[] = {
    &nhmex_uncore_ubox,
    &nhmex_uncore_cbox,
    &nhmex_uncore_bbox,
    &nhmex_uncore_sbox,
    &nhmex_uncore_mbox,
    &nhmex_uncore_rbox,
    &nhmex_uncore_wbox,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn nhmex_uncore_cpu_init() {
    void nhmex_uncore_cpu_init(void)
    {
    if (boot_cpu_data.x86_vfm == INTEL_NEHALEM_EX)
    uncore_nhmex = true;
    else
    nhmex_uncore_mbox.event_descs = wsmex_uncore_mbox_events;
    if (nhmex_uncore_cbox.num_boxes > topology_num_cores_per_package())
    nhmex_uncore_cbox.num_boxes = topology_num_cores_per_package();
    uncore_msr_uncores = nhmex_msr_uncores;
    }
// end of Nehalem-EX uncore support
