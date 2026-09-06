//! Automatically rewritten from C to Rust
//! Source: arch/x86/events/intel/uncore_snb.c
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
// Nehalem/SandBridge/Haswell/Broadwell/Skylake uncore support

// Uncore IMC PCI IDs
pub const PCI_DEVICE_ID_INTEL_SNB_IMC: c_uint = 0x0100;
pub const PCI_DEVICE_ID_INTEL_IVB_IMC: c_uint = 0x0154;
pub const PCI_DEVICE_ID_INTEL_IVB_E3_IMC: c_uint = 0x0150;
pub const PCI_DEVICE_ID_INTEL_HSW_IMC: c_uint = 0x0c00;
pub const PCI_DEVICE_ID_INTEL_HSW_U_IMC: c_uint = 0x0a04;
pub const PCI_DEVICE_ID_INTEL_BDW_IMC: c_uint = 0x1604;
pub const PCI_DEVICE_ID_INTEL_SKL_U_IMC: c_uint = 0x1904;
pub const PCI_DEVICE_ID_INTEL_SKL_Y_IMC: c_uint = 0x190c;
pub const PCI_DEVICE_ID_INTEL_SKL_HD_IMC: c_uint = 0x1900;
pub const PCI_DEVICE_ID_INTEL_SKL_HQ_IMC: c_uint = 0x1910;
pub const PCI_DEVICE_ID_INTEL_SKL_SD_IMC: c_uint = 0x190f;
pub const PCI_DEVICE_ID_INTEL_SKL_SQ_IMC: c_uint = 0x191f;
pub const PCI_DEVICE_ID_INTEL_SKL_E3_IMC: c_uint = 0x1918;
pub const PCI_DEVICE_ID_INTEL_KBL_Y_IMC: c_uint = 0x590c;
pub const PCI_DEVICE_ID_INTEL_KBL_U_IMC: c_uint = 0x5904;
pub const PCI_DEVICE_ID_INTEL_KBL_UQ_IMC: c_uint = 0x5914;
pub const PCI_DEVICE_ID_INTEL_KBL_SD_IMC: c_uint = 0x590f;
pub const PCI_DEVICE_ID_INTEL_KBL_SQ_IMC: c_uint = 0x591f;
pub const PCI_DEVICE_ID_INTEL_KBL_HQ_IMC: c_uint = 0x5910;
pub const PCI_DEVICE_ID_INTEL_KBL_WQ_IMC: c_uint = 0x5918;
pub const PCI_DEVICE_ID_INTEL_CFL_2U_IMC: c_uint = 0x3ecc;
pub const PCI_DEVICE_ID_INTEL_CFL_4U_IMC: c_uint = 0x3ed0;
pub const PCI_DEVICE_ID_INTEL_CFL_4H_IMC: c_uint = 0x3e10;
pub const PCI_DEVICE_ID_INTEL_CFL_6H_IMC: c_uint = 0x3ec4;
pub const PCI_DEVICE_ID_INTEL_CFL_2S_D_IMC: c_uint = 0x3e0f;
pub const PCI_DEVICE_ID_INTEL_CFL_4S_D_IMC: c_uint = 0x3e1f;
pub const PCI_DEVICE_ID_INTEL_CFL_6S_D_IMC: c_uint = 0x3ec2;
pub const PCI_DEVICE_ID_INTEL_CFL_8S_D_IMC: c_uint = 0x3e30;
pub const PCI_DEVICE_ID_INTEL_CFL_4S_W_IMC: c_uint = 0x3e18;
pub const PCI_DEVICE_ID_INTEL_CFL_6S_W_IMC: c_uint = 0x3ec6;
pub const PCI_DEVICE_ID_INTEL_CFL_8S_W_IMC: c_uint = 0x3e31;
pub const PCI_DEVICE_ID_INTEL_CFL_4S_S_IMC: c_uint = 0x3e33;
pub const PCI_DEVICE_ID_INTEL_CFL_6S_S_IMC: c_uint = 0x3eca;
pub const PCI_DEVICE_ID_INTEL_CFL_8S_S_IMC: c_uint = 0x3e32;
pub const PCI_DEVICE_ID_INTEL_AML_YD_IMC: c_uint = 0x590c;
pub const PCI_DEVICE_ID_INTEL_AML_YQ_IMC: c_uint = 0x590d;
pub const PCI_DEVICE_ID_INTEL_WHL_UQ_IMC: c_uint = 0x3ed0;
pub const PCI_DEVICE_ID_INTEL_WHL_4_UQ_IMC: c_uint = 0x3e34;
pub const PCI_DEVICE_ID_INTEL_WHL_UD_IMC: c_uint = 0x3e35;
pub const PCI_DEVICE_ID_INTEL_CML_H1_IMC: c_uint = 0x9b44;
pub const PCI_DEVICE_ID_INTEL_CML_H2_IMC: c_uint = 0x9b54;
pub const PCI_DEVICE_ID_INTEL_CML_H3_IMC: c_uint = 0x9b64;
pub const PCI_DEVICE_ID_INTEL_CML_U1_IMC: c_uint = 0x9b51;
pub const PCI_DEVICE_ID_INTEL_CML_U2_IMC: c_uint = 0x9b61;
pub const PCI_DEVICE_ID_INTEL_CML_U3_IMC: c_uint = 0x9b71;
pub const PCI_DEVICE_ID_INTEL_CML_S1_IMC: c_uint = 0x9b33;
pub const PCI_DEVICE_ID_INTEL_CML_S2_IMC: c_uint = 0x9b43;
pub const PCI_DEVICE_ID_INTEL_CML_S3_IMC: c_uint = 0x9b53;
pub const PCI_DEVICE_ID_INTEL_CML_S4_IMC: c_uint = 0x9b63;
pub const PCI_DEVICE_ID_INTEL_CML_S5_IMC: c_uint = 0x9b73;
pub const PCI_DEVICE_ID_INTEL_ICL_U_IMC: c_uint = 0x8a02;
pub const PCI_DEVICE_ID_INTEL_ICL_U2_IMC: c_uint = 0x8a12;
pub const PCI_DEVICE_ID_INTEL_TGL_U1_IMC: c_uint = 0x9a02;
pub const PCI_DEVICE_ID_INTEL_TGL_U2_IMC: c_uint = 0x9a04;
pub const PCI_DEVICE_ID_INTEL_TGL_U3_IMC: c_uint = 0x9a12;
pub const PCI_DEVICE_ID_INTEL_TGL_U4_IMC: c_uint = 0x9a14;
pub const PCI_DEVICE_ID_INTEL_TGL_H_IMC: c_uint = 0x9a36;
pub const PCI_DEVICE_ID_INTEL_RKL_1_IMC: c_uint = 0x4c43;
pub const PCI_DEVICE_ID_INTEL_RKL_2_IMC: c_uint = 0x4c53;
pub const PCI_DEVICE_ID_INTEL_ADL_1_IMC: c_uint = 0x4660;
pub const PCI_DEVICE_ID_INTEL_ADL_2_IMC: c_uint = 0x4641;
pub const PCI_DEVICE_ID_INTEL_ADL_3_IMC: c_uint = 0x4601;
pub const PCI_DEVICE_ID_INTEL_ADL_4_IMC: c_uint = 0x4602;
pub const PCI_DEVICE_ID_INTEL_ADL_5_IMC: c_uint = 0x4609;
pub const PCI_DEVICE_ID_INTEL_ADL_6_IMC: c_uint = 0x460a;
pub const PCI_DEVICE_ID_INTEL_ADL_7_IMC: c_uint = 0x4621;
pub const PCI_DEVICE_ID_INTEL_ADL_8_IMC: c_uint = 0x4623;
pub const PCI_DEVICE_ID_INTEL_ADL_9_IMC: c_uint = 0x4629;
pub const PCI_DEVICE_ID_INTEL_ADL_10_IMC: c_uint = 0x4637;
pub const PCI_DEVICE_ID_INTEL_ADL_11_IMC: c_uint = 0x463b;
pub const PCI_DEVICE_ID_INTEL_ADL_12_IMC: c_uint = 0x4648;
pub const PCI_DEVICE_ID_INTEL_ADL_13_IMC: c_uint = 0x4649;
pub const PCI_DEVICE_ID_INTEL_ADL_14_IMC: c_uint = 0x4650;
pub const PCI_DEVICE_ID_INTEL_ADL_15_IMC: c_uint = 0x4668;
pub const PCI_DEVICE_ID_INTEL_ADL_16_IMC: c_uint = 0x4670;
pub const PCI_DEVICE_ID_INTEL_ADL_17_IMC: c_uint = 0x4614;
pub const PCI_DEVICE_ID_INTEL_ADL_18_IMC: c_uint = 0x4617;
pub const PCI_DEVICE_ID_INTEL_ADL_19_IMC: c_uint = 0x4618;
pub const PCI_DEVICE_ID_INTEL_ADL_20_IMC: c_uint = 0x461B;
pub const PCI_DEVICE_ID_INTEL_ADL_21_IMC: c_uint = 0x461C;
pub const PCI_DEVICE_ID_INTEL_RPL_1_IMC: c_uint = 0xA700;
pub const PCI_DEVICE_ID_INTEL_RPL_2_IMC: c_uint = 0xA702;
pub const PCI_DEVICE_ID_INTEL_RPL_3_IMC: c_uint = 0xA706;
pub const PCI_DEVICE_ID_INTEL_RPL_4_IMC: c_uint = 0xA709;
pub const PCI_DEVICE_ID_INTEL_RPL_5_IMC: c_uint = 0xA701;
pub const PCI_DEVICE_ID_INTEL_RPL_6_IMC: c_uint = 0xA703;
pub const PCI_DEVICE_ID_INTEL_RPL_7_IMC: c_uint = 0xA704;
pub const PCI_DEVICE_ID_INTEL_RPL_8_IMC: c_uint = 0xA705;
pub const PCI_DEVICE_ID_INTEL_RPL_9_IMC: c_uint = 0xA706;
pub const PCI_DEVICE_ID_INTEL_RPL_10_IMC: c_uint = 0xA707;
pub const PCI_DEVICE_ID_INTEL_RPL_11_IMC: c_uint = 0xA708;
pub const PCI_DEVICE_ID_INTEL_RPL_12_IMC: c_uint = 0xA709;
pub const PCI_DEVICE_ID_INTEL_RPL_13_IMC: c_uint = 0xA70a;
pub const PCI_DEVICE_ID_INTEL_RPL_14_IMC: c_uint = 0xA70b;
pub const PCI_DEVICE_ID_INTEL_RPL_15_IMC: c_uint = 0xA715;
pub const PCI_DEVICE_ID_INTEL_RPL_16_IMC: c_uint = 0xA716;
pub const PCI_DEVICE_ID_INTEL_RPL_17_IMC: c_uint = 0xA717;
pub const PCI_DEVICE_ID_INTEL_RPL_18_IMC: c_uint = 0xA718;
pub const PCI_DEVICE_ID_INTEL_RPL_19_IMC: c_uint = 0xA719;
pub const PCI_DEVICE_ID_INTEL_RPL_20_IMC: c_uint = 0xA71A;
pub const PCI_DEVICE_ID_INTEL_RPL_21_IMC: c_uint = 0xA71B;
pub const PCI_DEVICE_ID_INTEL_RPL_22_IMC: c_uint = 0xA71C;
pub const PCI_DEVICE_ID_INTEL_RPL_23_IMC: c_uint = 0xA728;
pub const PCI_DEVICE_ID_INTEL_RPL_24_IMC: c_uint = 0xA729;
pub const PCI_DEVICE_ID_INTEL_RPL_25_IMC: c_uint = 0xA72A;
pub const PCI_DEVICE_ID_INTEL_MTL_1_IMC: c_uint = 0x7d00;
pub const PCI_DEVICE_ID_INTEL_MTL_2_IMC: c_uint = 0x7d01;
pub const PCI_DEVICE_ID_INTEL_MTL_3_IMC: c_uint = 0x7d02;
pub const PCI_DEVICE_ID_INTEL_MTL_4_IMC: c_uint = 0x7d05;
pub const PCI_DEVICE_ID_INTEL_MTL_5_IMC: c_uint = 0x7d10;
pub const PCI_DEVICE_ID_INTEL_MTL_6_IMC: c_uint = 0x7d14;
pub const PCI_DEVICE_ID_INTEL_MTL_7_IMC: c_uint = 0x7d15;
pub const PCI_DEVICE_ID_INTEL_MTL_8_IMC: c_uint = 0x7d16;
pub const PCI_DEVICE_ID_INTEL_MTL_9_IMC: c_uint = 0x7d21;
pub const PCI_DEVICE_ID_INTEL_MTL_10_IMC: c_uint = 0x7d22;
pub const PCI_DEVICE_ID_INTEL_MTL_11_IMC: c_uint = 0x7d23;
pub const PCI_DEVICE_ID_INTEL_MTL_12_IMC: c_uint = 0x7d24;
pub const PCI_DEVICE_ID_INTEL_MTL_13_IMC: c_uint = 0x7d28;

    {									\
    PCI_DEVICE(PCI_VENDOR_ID_INTEL, PCI_DEVICE_ID_INTEL_##a##_IMC),	\
    .driver_data = UNCORE_PCI_DEV_DATA(SNB_PCI_UNCORE_IMC, 0),	\
    }
// SNB event control
pub const SNB_UNC_CTL_EV_SEL_MASK: c_uint = 0x000000ff;
pub const SNB_UNC_CTL_UMASK_MASK: c_uint = 0x0000ff00;

pub const SNB_UNC_CTL_CMASK_MASK: c_uint = 0x1f000000;
pub const NHM_UNC_CTL_CMASK_MASK: c_uint = 0xff000000;

    SNB_UNC_CTL_UMASK_MASK | \
    SNB_UNC_CTL_EDGE_DET | \
    SNB_UNC_CTL_INVERT | \
    SNB_UNC_CTL_CMASK_MASK)

    SNB_UNC_CTL_UMASK_MASK | \
    SNB_UNC_CTL_EDGE_DET | \
    SNB_UNC_CTL_INVERT | \
    NHM_UNC_CTL_CMASK_MASK)
// SNB global control register
pub const SNB_UNC_PERF_GLOBAL_CTL: c_uint = 0x391;
pub const SNB_UNC_FIXED_CTR_CTRL: c_uint = 0x394;
pub const SNB_UNC_FIXED_CTR: c_uint = 0x395;
// SNB uncore global control

// SNB Cbo register
pub const SNB_UNC_CBO_0_PERFEVTSEL0: c_uint = 0x700;
pub const SNB_UNC_CBO_0_PER_CTR0: c_uint = 0x706;
pub const SNB_UNC_CBO_MSR_OFFSET: c_uint = 0x10;
// SNB ARB register
pub const SNB_UNC_ARB_PER_CTR0: c_uint = 0x3b0;
pub const SNB_UNC_ARB_PERFEVTSEL0: c_uint = 0x3b2;
pub const SNB_UNC_ARB_MSR_OFFSET: c_uint = 0x10;
// NHM global control register
pub const NHM_UNC_PERF_GLOBAL_CTL: c_uint = 0x391;
pub const NHM_UNC_FIXED_CTR: c_uint = 0x394;
pub const NHM_UNC_FIXED_CTR_CTRL: c_uint = 0x395;
// NHM uncore global control

// NHM uncore register
pub const NHM_UNC_PERFEVTSEL0: c_uint = 0x3c0;
pub const NHM_UNC_UNCORE_PMC0: c_uint = 0x3b0;
// SKL uncore global control
pub const SKL_UNC_PERF_GLOBAL_CTL: c_uint = 0xe01;

// ICL Cbo register
pub const ICL_UNC_CBO_CONFIG: c_uint = 0x396;
pub const ICL_UNC_NUM_CBO_MASK: c_uint = 0xf;
pub const ICL_UNC_CBO_0_PER_CTR0: c_uint = 0x702;
pub const ICL_UNC_CBO_MSR_OFFSET: c_uint = 0x8;
// ICL ARB register
pub const ICL_UNC_ARB_PER_CTR: c_uint = 0x3b1;
pub const ICL_UNC_ARB_PERFEVTSEL: c_uint = 0x3b3;
// ADL uncore global control
pub const ADL_UNC_PERF_GLOBAL_CTL: c_uint = 0x2ff0;
pub const ADL_UNC_FIXED_CTR_CTRL: c_uint = 0x2fde;
pub const ADL_UNC_FIXED_CTR: c_uint = 0x2fdf;
// ADL Cbo register
pub const ADL_UNC_CBO_0_PER_CTR0: c_uint = 0x2002;
pub const ADL_UNC_CBO_0_PERFEVTSEL0: c_uint = 0x2000;
pub const ADL_UNC_CTL_THRESHOLD: c_uint = 0x3f000000;

    SNB_UNC_CTL_UMASK_MASK | \
    SNB_UNC_CTL_EDGE_DET | \
    SNB_UNC_CTL_INVERT | \
    ADL_UNC_CTL_THRESHOLD)
// ADL ARB register
pub const ADL_UNC_ARB_PER_CTR0: c_uint = 0x2FD2;
pub const ADL_UNC_ARB_PERFEVTSEL0: c_uint = 0x2FD0;
pub const ADL_UNC_ARB_MSR_OFFSET: c_uint = 0x8;
// MTL Cbo register
pub const MTL_UNC_CBO_0_PER_CTR0: c_uint = 0x2448;
pub const MTL_UNC_CBO_0_PERFEVTSEL0: c_uint = 0x2442;
// MTL HAC_ARB register
pub const MTL_UNC_HAC_ARB_CTR: c_uint = 0x2018;
pub const MTL_UNC_HAC_ARB_CTRL: c_uint = 0x2012;
// MTL ARB register
pub const MTL_UNC_ARB_CTR: c_uint = 0x2418;
pub const MTL_UNC_ARB_CTRL: c_uint = 0x2412;
// MTL cNCU register
pub const MTL_UNC_CNCU_FIXED_CTR: c_uint = 0x2408;
pub const MTL_UNC_CNCU_FIXED_CTRL: c_uint = 0x2402;
pub const MTL_UNC_CNCU_BOX_CTL: c_uint = 0x240e;
// MTL sNCU register
pub const MTL_UNC_SNCU_FIXED_CTR: c_uint = 0x2008;
pub const MTL_UNC_SNCU_FIXED_CTRL: c_uint = 0x2002;
pub const MTL_UNC_SNCU_BOX_CTL: c_uint = 0x200e;
// MTL HAC_CBO register
pub const MTL_UNC_HBO_CTR: c_uint = 0x2048;
pub const MTL_UNC_HBO_CTRL: c_uint = 0x2042;
// PTL Low Power Bridge register
pub const PTL_UNC_IA_CORE_BRIDGE_PER_CTR0: c_uint = 0x2028;
pub const PTL_UNC_IA_CORE_BRIDGE_PERFEVTSEL0: c_uint = 0x2022;
// PTL Santa register
pub const PTL_UNC_SANTA_CTR0: c_uint = 0x2418;
pub const PTL_UNC_SANTA_CTRL0: c_uint = 0x2412;
// PTL cNCU register
pub const PTL_UNC_CNCU_MSR_OFFSET: c_uint = 0x140;
// NVL cNCU register
pub const NVL_UNC_CNCU_BOX_CTL: c_uint = 0x202e;
pub const NVL_UNC_CNCU_FIXED_CTR: c_uint = 0x2028;
pub const NVL_UNC_CNCU_FIXED_CTRL: c_uint = 0x2022;
// NVL SANTA register
pub const NVL_UNC_SANTA_CTR0: c_uint = 0x2048;
pub const NVL_UNC_SANTA_CTRL0: c_uint = 0x2042;
// NVL CBOX register
pub const NVL_UNC_CBOX_PER_CTR0: c_uint = 0x2108;
pub const NVL_UNC_CBOX_PERFEVTSEL0: c_uint = 0x2102;
    DEFINE_UNCORE_FORMAT_ATTR(event, event, "config:0-7");
    DEFINE_UNCORE_FORMAT_ATTR(umask, umask, "config:8-15");
    DEFINE_UNCORE_FORMAT_ATTR(chmask, chmask, "config:8-11");
    DEFINE_UNCORE_FORMAT_ATTR(edge, edge, "config:18");
    DEFINE_UNCORE_FORMAT_ATTR(inv, inv, "config:23");
    DEFINE_UNCORE_FORMAT_ATTR(cmask5, cmask, "config:24-28");
    DEFINE_UNCORE_FORMAT_ATTR(cmask8, cmask, "config:24-31");
    DEFINE_UNCORE_FORMAT_ATTR(threshold, threshold, "config:24-29");
    DEFINE_UNCORE_FORMAT_ATTR(threshold2, threshold, "config:24-31");
// Sandy Bridge uncore support
#[no_mangle]
unsafe extern "C" fn snb_uncore_msr_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void snb_uncore_msr_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    if (hwc.idx < UNCORE_PMC_IDX_FIXED)
    wrmsrq(hwc.config_base, hwc.config | SNB_UNC_CTL_EN);
    else
    wrmsrq(hwc.config_base, SNB_UNC_CTL_EN);
    }
#[no_mangle]
unsafe extern "C" fn snb_uncore_msr_disable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void snb_uncore_msr_disable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    wrmsrq(event.hw.config_base, 0);
    }
#[no_mangle]
unsafe extern "C" fn snb_uncore_msr_init_box(box: *mut intel_uncore_box) -> c_int {
    static int snb_uncore_msr_init_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0) {
    wrmsrq(SNB_UNC_PERF_GLOBAL_CTL,
    SNB_UNC_GLOBAL_CTL_EN | SNB_UNC_GLOBAL_CTL_CORE_ALL);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snb_uncore_msr_enable_box(box: *mut intel_uncore_box) {
    static void snb_uncore_msr_enable_box(struct intel_uncore_box *box)
    {
    wrmsrq(SNB_UNC_PERF_GLOBAL_CTL,
    SNB_UNC_GLOBAL_CTL_EN | SNB_UNC_GLOBAL_CTL_CORE_ALL);
    }
#[no_mangle]
unsafe extern "C" fn snb_uncore_msr_exit_box(box: *mut intel_uncore_box) {
    static void snb_uncore_msr_exit_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0)
    wrmsrq(SNB_UNC_PERF_GLOBAL_CTL, 0);
    }
    static struct uncore_event_desc snb_uncore_events[] = {
    INTEL_UNCORE_EVENT_DESC(clockticks, "event=0xff,umask=0x00"),
    { /* end: all zeroes */ },
    };
    static struct attribute *snb_uncore_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_umask.attr,
    &format_attr_edge.attr,
    &format_attr_inv.attr,
    &format_attr_cmask5.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group snb_uncore_format_group = {
    .name		= "format",
    .attrs		= snb_uncore_formats_attr,
    };
    static struct intel_uncore_ops snb_uncore_msr_ops = {
    .init_box	= snb_uncore_msr_init_box,
    .enable_box	= snb_uncore_msr_enable_box,
    .exit_box	= snb_uncore_msr_exit_box,
    .disable_event	= snb_uncore_msr_disable_event,
    .enable_event	= snb_uncore_msr_enable_event,
    .read_counter	= uncore_msr_read_counter,
    };
    static struct event_constraint snb_uncore_arb_constraints[] = {
    UNCORE_EVENT_CONSTRAINT(0x80, 0x1),
    UNCORE_EVENT_CONSTRAINT(0x83, 0x1),
    EVENT_CONSTRAINT_END
    };
    static struct intel_uncore_type snb_uncore_cbox = {
    .name		= "cbox",
    .num_counters   = 2,
    .num_boxes	= 4,
    .perf_ctr_bits	= 44,
    .fixed_ctr_bits	= 48,
    .perf_ctr	= SNB_UNC_CBO_0_PER_CTR0,
    .event_ctl	= SNB_UNC_CBO_0_PERFEVTSEL0,
    .fixed_ctr	= SNB_UNC_FIXED_CTR,
    .fixed_ctl	= SNB_UNC_FIXED_CTR_CTRL,
    .single_fixed	= 1,
    .event_mask	= SNB_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_CBO_MSR_OFFSET,
    .ops		= &snb_uncore_msr_ops,
    .format_group	= &snb_uncore_format_group,
    .event_descs	= snb_uncore_events,
    };
    static struct intel_uncore_type snb_uncore_arb = {
    .name		= "arb",
    .num_counters   = 2,
    .num_boxes	= 1,
    .perf_ctr_bits	= 44,
    .perf_ctr	= SNB_UNC_ARB_PER_CTR0,
    .event_ctl	= SNB_UNC_ARB_PERFEVTSEL0,
    .event_mask	= SNB_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_ARB_MSR_OFFSET,
    .constraints	= snb_uncore_arb_constraints,
    .ops		= &snb_uncore_msr_ops,
    .format_group	= &snb_uncore_format_group,
    };
    static struct intel_uncore_type *snb_msr_uncores[] = {
    &snb_uncore_cbox,
    &snb_uncore_arb,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn snb_uncore_cpu_init() {
    void snb_uncore_cpu_init(void)
    {
    uncore_msr_uncores = snb_msr_uncores;
    if (snb_uncore_cbox.num_boxes > topology_num_cores_per_package())
    snb_uncore_cbox.num_boxes = topology_num_cores_per_package();
    }
#[no_mangle]
unsafe extern "C" fn skl_uncore_msr_init_box(box: *mut intel_uncore_box) -> c_int {
    static int skl_uncore_msr_init_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0) {
    wrmsrq(SKL_UNC_PERF_GLOBAL_CTL,
    SNB_UNC_GLOBAL_CTL_EN | SKL_UNC_GLOBAL_CTL_CORE_ALL);
    }
// The 8th CBOX has different MSR space
    if (box.pmu.pmu_idx == 7)
    __set_bit(UNCORE_BOX_FLAG_CFL8_CBOX_MSR_OFFS, &box.flags);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn skl_uncore_msr_enable_box(box: *mut intel_uncore_box) {
    static void skl_uncore_msr_enable_box(struct intel_uncore_box *box)
    {
    wrmsrq(SKL_UNC_PERF_GLOBAL_CTL,
    SNB_UNC_GLOBAL_CTL_EN | SKL_UNC_GLOBAL_CTL_CORE_ALL);
    }
#[no_mangle]
unsafe extern "C" fn skl_uncore_msr_exit_box(box: *mut intel_uncore_box) {
    static void skl_uncore_msr_exit_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0)
    wrmsrq(SKL_UNC_PERF_GLOBAL_CTL, 0);
    }
    static struct intel_uncore_ops skl_uncore_msr_ops = {
    .init_box	= skl_uncore_msr_init_box,
    .enable_box	= skl_uncore_msr_enable_box,
    .exit_box	= skl_uncore_msr_exit_box,
    .disable_event	= snb_uncore_msr_disable_event,
    .enable_event	= snb_uncore_msr_enable_event,
    .read_counter	= uncore_msr_read_counter,
    };
    static struct intel_uncore_type skl_uncore_cbox = {
    .name		= "cbox",
    .num_counters   = 4,
    .num_boxes	= 8,
    .perf_ctr_bits	= 44,
    .fixed_ctr_bits	= 48,
    .perf_ctr	= SNB_UNC_CBO_0_PER_CTR0,
    .event_ctl	= SNB_UNC_CBO_0_PERFEVTSEL0,
    .fixed_ctr	= SNB_UNC_FIXED_CTR,
    .fixed_ctl	= SNB_UNC_FIXED_CTR_CTRL,
    .single_fixed	= 1,
    .event_mask	= SNB_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_CBO_MSR_OFFSET,
    .ops		= &skl_uncore_msr_ops,
    .format_group	= &snb_uncore_format_group,
    .event_descs	= snb_uncore_events,
    };
    static struct intel_uncore_type *skl_msr_uncores[] = {
    &skl_uncore_cbox,
    &snb_uncore_arb,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn skl_uncore_cpu_init() {
    void skl_uncore_cpu_init(void)
    {
    uncore_msr_uncores = skl_msr_uncores;
    if (skl_uncore_cbox.num_boxes > topology_num_cores_per_package())
    skl_uncore_cbox.num_boxes = topology_num_cores_per_package();
    snb_uncore_arb.ops = &skl_uncore_msr_ops;
    }
    static struct intel_uncore_ops icl_uncore_msr_ops = {
    .disable_event	= snb_uncore_msr_disable_event,
    .enable_event	= snb_uncore_msr_enable_event,
    .read_counter	= uncore_msr_read_counter,
    };
    static struct intel_uncore_type icl_uncore_cbox = {
    .name		= "cbox",
    .num_counters   = 2,
    .perf_ctr_bits	= 44,
    .perf_ctr	= ICL_UNC_CBO_0_PER_CTR0,
    .event_ctl	= SNB_UNC_CBO_0_PERFEVTSEL0,
    .event_mask	= SNB_UNC_RAW_EVENT_MASK,
    .msr_offset	= ICL_UNC_CBO_MSR_OFFSET,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &snb_uncore_format_group,
    };
    static struct uncore_event_desc icl_uncore_events[] = {
    INTEL_UNCORE_EVENT_DESC(clockticks, "event=0xff"),
    { /* end: all zeroes */ },
    };
    static struct attribute *icl_uncore_clock_formats_attr[] = {
    &format_attr_event.attr,
    core::ptr::null_mut(),
    };
    static struct attribute_group icl_uncore_clock_format_group = {
    .name = "format",
    .attrs = icl_uncore_clock_formats_attr,
    };
    static struct intel_uncore_type icl_uncore_clockbox = {
    .name		= "clock",
    .num_counters	= 1,
    .num_boxes	= 1,
    .fixed_ctr_bits	= 48,
    .fixed_ctr	= SNB_UNC_FIXED_CTR,
    .fixed_ctl	= SNB_UNC_FIXED_CTR_CTRL,
    .single_fixed	= 1,
    .event_mask	= SNB_UNC_CTL_EV_SEL_MASK,
    .format_group	= &icl_uncore_clock_format_group,
    .ops		= &icl_uncore_msr_ops,
    .event_descs	= icl_uncore_events,
    };
    static struct intel_uncore_type icl_uncore_arb = {
    .name		= "arb",
    .num_counters   = 1,
    .num_boxes	= 1,
    .perf_ctr_bits	= 44,
    .perf_ctr	= ICL_UNC_ARB_PER_CTR,
    .event_ctl	= ICL_UNC_ARB_PERFEVTSEL,
    .event_mask	= SNB_UNC_RAW_EVENT_MASK,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &snb_uncore_format_group,
    };
    static struct intel_uncore_type *icl_msr_uncores[] = {
    &icl_uncore_cbox,
    &icl_uncore_arb,
    &icl_uncore_clockbox,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn icl_get_cbox_num() -> c_int {
    static int icl_get_cbox_num(void)
    {
    u64 num_boxes;
    rdmsrq(ICL_UNC_CBO_CONFIG, num_boxes);
    return num_boxes & ICL_UNC_NUM_CBO_MASK;
    }
#[no_mangle]
pub unsafe extern "C" fn icl_uncore_cpu_init() {
    void icl_uncore_cpu_init(void)
    {
    uncore_msr_uncores = icl_msr_uncores;
    icl_uncore_cbox.num_boxes = icl_get_cbox_num();
    }
    static struct intel_uncore_type *tgl_msr_uncores[] = {
    &icl_uncore_cbox,
    &snb_uncore_arb,
    &icl_uncore_clockbox,
    core::ptr::null_mut(),
    };
#[no_mangle]
unsafe extern "C" fn rkl_uncore_msr_init_box(box: *mut intel_uncore_box) -> c_int {
    static int rkl_uncore_msr_init_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0)
    wrmsrq(SKL_UNC_PERF_GLOBAL_CTL, SNB_UNC_GLOBAL_CTL_EN);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn tgl_uncore_cpu_init() {
    void tgl_uncore_cpu_init(void)
    {
    uncore_msr_uncores = tgl_msr_uncores;
    icl_uncore_cbox.num_boxes = icl_get_cbox_num();
    icl_uncore_cbox.ops = &skl_uncore_msr_ops;
    icl_uncore_clockbox.ops = &skl_uncore_msr_ops;
    snb_uncore_arb.ops = &skl_uncore_msr_ops;
    skl_uncore_msr_ops.init_box = rkl_uncore_msr_init_box;
    }
#[no_mangle]
unsafe extern "C" fn adl_uncore_msr_enable_box(box: *mut intel_uncore_box) {
    static void adl_uncore_msr_enable_box(struct intel_uncore_box *box)
    {
    wrmsrq(ADL_UNC_PERF_GLOBAL_CTL, SNB_UNC_GLOBAL_CTL_EN);
    }
#[no_mangle]
unsafe extern "C" fn adl_uncore_msr_disable_box(box: *mut intel_uncore_box) {
    static void adl_uncore_msr_disable_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0)
    wrmsrq(ADL_UNC_PERF_GLOBAL_CTL, 0);
    }
#[no_mangle]
unsafe extern "C" fn adl_uncore_msr_exit_box(box: *mut intel_uncore_box) {
    static void adl_uncore_msr_exit_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0)
    wrmsrq(ADL_UNC_PERF_GLOBAL_CTL, 0);
    }
    static struct intel_uncore_ops adl_uncore_msr_ops = {
    .enable_box	= adl_uncore_msr_enable_box,
    .disable_box	= adl_uncore_msr_disable_box,
    .exit_box	= adl_uncore_msr_exit_box,
    .disable_event	= snb_uncore_msr_disable_event,
    .enable_event	= snb_uncore_msr_enable_event,
    .read_counter	= uncore_msr_read_counter,
    };
    static struct attribute *adl_uncore_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_umask.attr,
    &format_attr_edge.attr,
    &format_attr_inv.attr,
    &format_attr_threshold.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group adl_uncore_format_group = {
    .name		= "format",
    .attrs		= adl_uncore_formats_attr,
    };
    static struct intel_uncore_type adl_uncore_cbox = {
    .name		= "cbox",
    .num_counters   = 2,
    .perf_ctr_bits	= 44,
    .perf_ctr	= ADL_UNC_CBO_0_PER_CTR0,
    .event_ctl	= ADL_UNC_CBO_0_PERFEVTSEL0,
    .event_mask	= ADL_UNC_RAW_EVENT_MASK,
    .msr_offset	= ICL_UNC_CBO_MSR_OFFSET,
    .ops		= &adl_uncore_msr_ops,
    .format_group	= &adl_uncore_format_group,
    };
    static struct intel_uncore_type adl_uncore_arb = {
    .name		= "arb",
    .num_counters   = 2,
    .num_boxes	= 2,
    .perf_ctr_bits	= 44,
    .perf_ctr	= ADL_UNC_ARB_PER_CTR0,
    .event_ctl	= ADL_UNC_ARB_PERFEVTSEL0,
    .event_mask	= SNB_UNC_RAW_EVENT_MASK,
    .msr_offset	= ADL_UNC_ARB_MSR_OFFSET,
    .constraints	= snb_uncore_arb_constraints,
    .ops		= &adl_uncore_msr_ops,
    .format_group	= &snb_uncore_format_group,
    };
    static struct intel_uncore_type adl_uncore_clockbox = {
    .name		= "clock",
    .num_counters	= 1,
    .num_boxes	= 1,
    .fixed_ctr_bits	= 48,
    .fixed_ctr	= ADL_UNC_FIXED_CTR,
    .fixed_ctl	= ADL_UNC_FIXED_CTR_CTRL,
    .single_fixed	= 1,
    .event_mask	= SNB_UNC_CTL_EV_SEL_MASK,
    .format_group	= &icl_uncore_clock_format_group,
    .ops		= &adl_uncore_msr_ops,
    .event_descs	= icl_uncore_events,
    };
    static struct intel_uncore_type *adl_msr_uncores[] = {
    &adl_uncore_cbox,
    &adl_uncore_arb,
    &adl_uncore_clockbox,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn adl_uncore_cpu_init() {
    void adl_uncore_cpu_init(void)
    {
    adl_uncore_cbox.num_boxes = icl_get_cbox_num();
    uncore_msr_uncores = adl_msr_uncores;
    }
    static struct intel_uncore_type mtl_uncore_cbox = {
    .name		= "cbox",
    .num_counters   = 2,
    .perf_ctr_bits	= 48,
    .perf_ctr	= MTL_UNC_CBO_0_PER_CTR0,
    .event_ctl	= MTL_UNC_CBO_0_PERFEVTSEL0,
    .event_mask	= ADL_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_CBO_MSR_OFFSET,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &adl_uncore_format_group,
    };
    static struct intel_uncore_type mtl_uncore_hac_arb = {
    .name		= "hac_arb",
    .num_counters   = 2,
    .num_boxes	= 2,
    .perf_ctr_bits	= 48,
    .perf_ctr	= MTL_UNC_HAC_ARB_CTR,
    .event_ctl	= MTL_UNC_HAC_ARB_CTRL,
    .event_mask	= ADL_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_CBO_MSR_OFFSET,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &adl_uncore_format_group,
    };
    static struct intel_uncore_type mtl_uncore_arb = {
    .name		= "arb",
    .num_counters   = 2,
    .num_boxes	= 2,
    .perf_ctr_bits	= 48,
    .perf_ctr	= MTL_UNC_ARB_CTR,
    .event_ctl	= MTL_UNC_ARB_CTRL,
    .event_mask	= ADL_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_CBO_MSR_OFFSET,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &adl_uncore_format_group,
    };
    static struct intel_uncore_type mtl_uncore_hac_cbox = {
    .name		= "hac_cbox",
    .num_counters   = 2,
    .num_boxes	= 2,
    .perf_ctr_bits	= 48,
    .perf_ctr	= MTL_UNC_HBO_CTR,
    .event_ctl	= MTL_UNC_HBO_CTRL,
    .event_mask	= ADL_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_CBO_MSR_OFFSET,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &adl_uncore_format_group,
    };
#[no_mangle]
unsafe extern "C" fn mtl_uncore_msr_init_box(box: *mut intel_uncore_box) -> c_int {
    static int mtl_uncore_msr_init_box(struct intel_uncore_box *box)
    {
    wrmsrq(uncore_msr_box_ctl(box), SNB_UNC_GLOBAL_CTL_EN);
    return 0;
    }
    static struct intel_uncore_ops mtl_uncore_msr_ops = {
    .init_box	= mtl_uncore_msr_init_box,
    .disable_event	= snb_uncore_msr_disable_event,
    .enable_event	= snb_uncore_msr_enable_event,
    .read_counter	= uncore_msr_read_counter,
    };
    static struct intel_uncore_type mtl_uncore_cncu = {
    .name		= "cncu",
    .num_counters   = 1,
    .num_boxes	= 1,
    .box_ctl	= MTL_UNC_CNCU_BOX_CTL,
    .fixed_ctr_bits = 48,
    .fixed_ctr	= MTL_UNC_CNCU_FIXED_CTR,
    .fixed_ctl	= MTL_UNC_CNCU_FIXED_CTRL,
    .single_fixed	= 1,
    .event_mask	= SNB_UNC_CTL_EV_SEL_MASK,
    .format_group	= &icl_uncore_clock_format_group,
    .ops		= &mtl_uncore_msr_ops,
    .event_descs	= icl_uncore_events,
    };
    static struct intel_uncore_type mtl_uncore_sncu = {
    .name		= "sncu",
    .num_counters   = 1,
    .num_boxes	= 1,
    .box_ctl	= MTL_UNC_SNCU_BOX_CTL,
    .fixed_ctr_bits	= 48,
    .fixed_ctr	= MTL_UNC_SNCU_FIXED_CTR,
    .fixed_ctl	= MTL_UNC_SNCU_FIXED_CTRL,
    .single_fixed	= 1,
    .event_mask	= SNB_UNC_CTL_EV_SEL_MASK,
    .format_group	= &icl_uncore_clock_format_group,
    .ops		= &mtl_uncore_msr_ops,
    .event_descs	= icl_uncore_events,
    };
    static struct intel_uncore_type *mtl_msr_uncores[] = {
    &mtl_uncore_cbox,
    &mtl_uncore_hac_arb,
    &mtl_uncore_arb,
    &mtl_uncore_hac_cbox,
    &mtl_uncore_cncu,
    &mtl_uncore_sncu,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn mtl_uncore_cpu_init() {
    void mtl_uncore_cpu_init(void)
    {
    mtl_uncore_cbox.num_boxes = icl_get_cbox_num();
    uncore_msr_uncores = mtl_msr_uncores;
    }
    static struct intel_uncore_type *lnl_msr_uncores[] = {
    &mtl_uncore_cbox,
    &mtl_uncore_arb,
    core::ptr::null_mut()
    };
pub const LNL_UNC_MSR_GLOBAL_CTL: c_uint = 0x240e;
#[no_mangle]
unsafe extern "C" fn lnl_uncore_msr_init_box(box: *mut intel_uncore_box) -> c_int {
    static int lnl_uncore_msr_init_box(struct intel_uncore_box *box)
    {
    if (box.pmu.pmu_idx == 0)
    wrmsrq(LNL_UNC_MSR_GLOBAL_CTL, SNB_UNC_GLOBAL_CTL_EN);
    return 0;
    }
    static struct intel_uncore_ops lnl_uncore_msr_ops = {
    .init_box	= lnl_uncore_msr_init_box,
    .disable_event	= snb_uncore_msr_disable_event,
    .enable_event	= snb_uncore_msr_enable_event,
    .read_counter	= uncore_msr_read_counter,
    };
#[no_mangle]
pub unsafe extern "C" fn lnl_uncore_cpu_init() {
    void lnl_uncore_cpu_init(void)
    {
    mtl_uncore_cbox.num_boxes = 4;
    mtl_uncore_cbox.ops = &lnl_uncore_msr_ops;
    uncore_msr_uncores = lnl_msr_uncores;
    }
    enum {
    SNB_PCI_UNCORE_IMC,
    };
    static struct uncore_event_desc snb_uncore_imc_events[] = {
    INTEL_UNCORE_EVENT_DESC(data_reads,  "event=0x01"),
    INTEL_UNCORE_EVENT_DESC(data_reads.scale, "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(data_reads.unit, "MiB"),
    INTEL_UNCORE_EVENT_DESC(data_writes, "event=0x02"),
    INTEL_UNCORE_EVENT_DESC(data_writes.scale, "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(data_writes.unit, "MiB"),
    INTEL_UNCORE_EVENT_DESC(gt_requests, "event=0x03"),
    INTEL_UNCORE_EVENT_DESC(gt_requests.scale, "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(gt_requests.unit, "MiB"),
    INTEL_UNCORE_EVENT_DESC(ia_requests, "event=0x04"),
    INTEL_UNCORE_EVENT_DESC(ia_requests.scale, "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(ia_requests.unit, "MiB"),
    INTEL_UNCORE_EVENT_DESC(io_requests, "event=0x05"),
    INTEL_UNCORE_EVENT_DESC(io_requests.scale, "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(io_requests.unit, "MiB"),
    { /* end: all zeroes */ },
    };
pub const SNB_UNCORE_PCI_IMC_EVENT_MASK: c_uint = 0xff;
pub const SNB_UNCORE_PCI_IMC_BAR_OFFSET: c_uint = 0x48;
// page size multiple covering all config regs
pub const SNB_UNCORE_PCI_IMC_MAP_SIZE: c_uint = 0x6000;
pub const SNB_UNCORE_PCI_IMC_DATA_READS: c_uint = 0x1;
pub const SNB_UNCORE_PCI_IMC_DATA_READS_BASE: c_uint = 0x5050;
pub const SNB_UNCORE_PCI_IMC_DATA_WRITES: c_uint = 0x2;
pub const SNB_UNCORE_PCI_IMC_DATA_WRITES_BASE: c_uint = 0x5054;

// BW break down- legacy counters
pub const SNB_UNCORE_PCI_IMC_GT_REQUESTS: c_uint = 0x3;
pub const SNB_UNCORE_PCI_IMC_GT_REQUESTS_BASE: c_uint = 0x5040;
pub const SNB_UNCORE_PCI_IMC_IA_REQUESTS: c_uint = 0x4;
pub const SNB_UNCORE_PCI_IMC_IA_REQUESTS_BASE: c_uint = 0x5044;
pub const SNB_UNCORE_PCI_IMC_IO_REQUESTS: c_uint = 0x5;
pub const SNB_UNCORE_PCI_IMC_IO_REQUESTS_BASE: c_uint = 0x5048;
    enum perf_snb_uncore_imc_freerunning_types {
    SNB_PCI_UNCORE_IMC_DATA_READS		= 0,
    SNB_PCI_UNCORE_IMC_DATA_WRITES,
    SNB_PCI_UNCORE_IMC_GT_REQUESTS,
    SNB_PCI_UNCORE_IMC_IA_REQUESTS,
    SNB_PCI_UNCORE_IMC_IO_REQUESTS,
    SNB_PCI_UNCORE_IMC_FREERUNNING_TYPE_MAX,
    };
    static struct freerunning_counters snb_uncore_imc_freerunning[] = {
    [SNB_PCI_UNCORE_IMC_DATA_READS]		= { SNB_UNCORE_PCI_IMC_DATA_READS_BASE,
    0x0, 0x0, 1, 32 },
    [SNB_PCI_UNCORE_IMC_DATA_WRITES]	= { SNB_UNCORE_PCI_IMC_DATA_WRITES_BASE,
    0x0, 0x0, 1, 32 },
    [SNB_PCI_UNCORE_IMC_GT_REQUESTS]	= { SNB_UNCORE_PCI_IMC_GT_REQUESTS_BASE,
    0x0, 0x0, 1, 32 },
    [SNB_PCI_UNCORE_IMC_IA_REQUESTS]	= { SNB_UNCORE_PCI_IMC_IA_REQUESTS_BASE,
    0x0, 0x0, 1, 32 },
    [SNB_PCI_UNCORE_IMC_IO_REQUESTS]	= { SNB_UNCORE_PCI_IMC_IO_REQUESTS_BASE,
    0x0, 0x0, 1, 32 },
    };
    static struct attribute *snb_uncore_imc_formats_attr[] = {
    &format_attr_event.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group snb_uncore_imc_format_group = {
    .name = "format",
    .attrs = snb_uncore_imc_formats_attr,
    };
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_init_box(box: *mut intel_uncore_box) -> c_int {
    static int snb_uncore_imc_init_box(struct intel_uncore_box *box)
    {
    struct intel_uncore_type *type = box.pmu.type;
    struct pci_dev *pdev = box.pci_dev;
    let mut where: c_int = SNB_UNCORE_PCI_IMC_BAR_OFFSET;
    resource_size_t addr;
    u32 pci_dword;
    pci_read_config_dword(pdev, where, &pci_dword);
    addr = pci_dword;

    pci_read_config_dword(pdev, where + 4, &pci_dword);
    addr |= ((resource_size_t)pci_dword << 32);

    addr &= ~(PAGE_SIZE - 1);
    box.io_addr = ioremap(addr, type.mmio_map_size);
    if (!box.io_addr) {
    pr_warn("perf uncore: Failed to ioremap for %s.\n", type.name);
    return -ENOMEM;
    }
    box.hrtimer_duration = UNCORE_SNB_IMC_HRTIMER_INTERVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_enable_box(box: *mut intel_uncore_box) {
    static void snb_uncore_imc_enable_box(struct intel_uncore_box *box)
    {}
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_disable_box(box: *mut intel_uncore_box) {
    static void snb_uncore_imc_disable_box(struct intel_uncore_box *box)
    {}
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void snb_uncore_imc_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {}
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_disable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void snb_uncore_imc_disable_event(struct intel_uncore_box *box, struct perf_event *event)
    {}
//
// Keep the custom event_init() function compatible with old event
// encoding for free running counters.
//
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_event_init(event: *mut perf_event) -> c_int {
    static int snb_uncore_imc_event_init(struct perf_event *event)
    {
    struct intel_uncore_pmu *pmu;
    struct intel_uncore_box *box;
    struct hw_perf_event *hwc = &event.hw;
    let mut cfg: u64 = event.attr.config & SNB_UNCORE_PCI_IMC_EVENT_MASK;
    int idx, base;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
    pmu = uncore_event_to_pmu(event);
// no device found for this pmu
    if (!uncore_pmu_available(pmu))
    return -ENOENT;
// Sampling not supported yet
    if (hwc.sample_period)
    return -EINVAL;
// unsupported modes and filters
    if (event.attr.sample_period) /* no sampling */
    return -EINVAL;
//
// Place all uncore events for a particular physical package
// onto a single cpu
//
    if (event.cpu < 0)
    return -EINVAL;
// check only supported bits are set
    if (event.attr.config & ~SNB_UNCORE_PCI_IMC_EVENT_MASK)
    return -EINVAL;
    box = uncore_pmu_to_box(pmu, event.cpu);
    if (!box || box.cpu < 0)
    return -EINVAL;
    event.cpu = box.cpu;
    event.pmu_private = box;
    event.event_caps |= PERF_EV_CAP_READ_ACTIVE_PKG;
    event.hw.idx = -1;
    event.hw.last_tag = ~0ULL;
    event.hw.extra_reg.idx = EXTRA_REG_NONE;
    event.hw.branch_reg.idx = EXTRA_REG_NONE;
//
// check event is known (whitelist, determines counter)
//
    switch (cfg) {
    case SNB_UNCORE_PCI_IMC_DATA_READS:
    base = SNB_UNCORE_PCI_IMC_DATA_READS_BASE;
    idx = UNCORE_PMC_IDX_FREERUNNING;
    break;
    case SNB_UNCORE_PCI_IMC_DATA_WRITES:
    base = SNB_UNCORE_PCI_IMC_DATA_WRITES_BASE;
    idx = UNCORE_PMC_IDX_FREERUNNING;
    break;
    case SNB_UNCORE_PCI_IMC_GT_REQUESTS:
    base = SNB_UNCORE_PCI_IMC_GT_REQUESTS_BASE;
    idx = UNCORE_PMC_IDX_FREERUNNING;
    break;
    case SNB_UNCORE_PCI_IMC_IA_REQUESTS:
    base = SNB_UNCORE_PCI_IMC_IA_REQUESTS_BASE;
    idx = UNCORE_PMC_IDX_FREERUNNING;
    break;
    case SNB_UNCORE_PCI_IMC_IO_REQUESTS:
    base = SNB_UNCORE_PCI_IMC_IO_REQUESTS_BASE;
    idx = UNCORE_PMC_IDX_FREERUNNING;
    break;
    default:
    return -EINVAL;
    }
// must be done before validate_group
    event.hw.event_base = base;
    event.hw.idx = idx;
// Convert to standard encoding format for freerunning counters
    event.hw.config = ((cfg - 1) << 8) | 0x10ff;
// no group validation needed, we have free running counters
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_hw_config(box: *mut intel_uncore_box, event: *mut perf_event) -> c_int {
    static int snb_uncore_imc_hw_config(struct intel_uncore_box *box, struct perf_event *event)
    {
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snb_pci2phy_map_init(devid: c_int) -> c_int {
    int snb_pci2phy_map_init(int devid)
    {
    struct pci_dev *dev = core::ptr::null_mut();
    struct pci2phy_map *map;
    int bus, segment;
    dev = pci_get_device(PCI_VENDOR_ID_INTEL, devid, dev);
    if (!dev)
    return -ENOTTY;
    bus = dev.bus.number;
    segment = pci_domain_nr(dev.bus);
    raw_spin_lock(&pci2phy_map_lock);
    map = __find_pci2phy_map(segment);
    if (!map) {
    raw_spin_unlock(&pci2phy_map_lock);
    pci_dev_put(dev);
    return -ENOMEM;
    }
    map.pbus_to_dieid[bus] = 0;
    raw_spin_unlock(&pci2phy_map_lock);
    pci_dev_put(dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn snb_uncore_imc_read_counter(box: *mut intel_uncore_box, event: *mut perf_event) -> u64 {
    static u64 snb_uncore_imc_read_counter(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
//
// SNB IMC counters are 32-bit and are laid out back to back
// in MMIO space. Therefore we must use a 32-bit accessor function
// using readq() from uncore_mmio_read_counter() causes problems
// because it is reading 64-bit at a time. This is okay for the
// uncore_perf_event_update() function because it drops the upper
// 32-bits but not okay for plain uncore_read_counter() as invoked
// in uncore_pmu_event_start().
//
    return (u64)readl(box.io_addr + hwc.event_base);
    }
    static struct pmu snb_uncore_imc_pmu = {
    .task_ctx_nr	= perf_invalid_context,
    .event_init	= snb_uncore_imc_event_init,
    .add		= uncore_pmu_event_add,
    .del		= uncore_pmu_event_del,
    .start		= uncore_pmu_event_start,
    .stop		= uncore_pmu_event_stop,
    .read		= uncore_pmu_event_read,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE,
    };
    static struct intel_uncore_ops snb_uncore_imc_ops = {
    .init_box	= snb_uncore_imc_init_box,
    .exit_box	= uncore_mmio_exit_box,
    .enable_box	= snb_uncore_imc_enable_box,
    .disable_box	= snb_uncore_imc_disable_box,
    .disable_event	= snb_uncore_imc_disable_event,
    .enable_event	= snb_uncore_imc_enable_event,
    .hw_config	= snb_uncore_imc_hw_config,
    .read_counter	= snb_uncore_imc_read_counter,
    };
    static struct intel_uncore_type snb_uncore_imc = {
    .name		= "imc",
    .num_counters   = 5,
    .num_boxes	= 1,
    .num_freerunning_types	= SNB_PCI_UNCORE_IMC_FREERUNNING_TYPE_MAX,
    .mmio_map_size	= SNB_UNCORE_PCI_IMC_MAP_SIZE,
    .freerunning	= snb_uncore_imc_freerunning,
    .event_descs	= snb_uncore_imc_events,
    .format_group	= &snb_uncore_imc_format_group,
    .ops		= &snb_uncore_imc_ops,
    .pmu		= &snb_uncore_imc_pmu,
    };
    static struct intel_uncore_type *snb_pci_uncores[] = {
    [SNB_PCI_UNCORE_IMC]	= &snb_uncore_imc,
    core::ptr::null_mut(),
    };
    static const struct pci_device_id snb_uncore_pci_ids[] = {
    IMC_UNCORE_DEV(SNB),
    { /* end: all zeroes */ },
    };
    static const struct pci_device_id ivb_uncore_pci_ids[] = {
    IMC_UNCORE_DEV(IVB),
    IMC_UNCORE_DEV(IVB_E3),
    { /* end: all zeroes */ },
    };
    static const struct pci_device_id hsw_uncore_pci_ids[] = {
    IMC_UNCORE_DEV(HSW),
    IMC_UNCORE_DEV(HSW_U),
    { /* end: all zeroes */ },
    };
    static const struct pci_device_id bdw_uncore_pci_ids[] = {
    IMC_UNCORE_DEV(BDW),
    { /* end: all zeroes */ },
    };
    static const struct pci_device_id skl_uncore_pci_ids[] = {
    IMC_UNCORE_DEV(SKL_Y),
    IMC_UNCORE_DEV(SKL_U),
    IMC_UNCORE_DEV(SKL_HD),
    IMC_UNCORE_DEV(SKL_HQ),
    IMC_UNCORE_DEV(SKL_SD),
    IMC_UNCORE_DEV(SKL_SQ),
    IMC_UNCORE_DEV(SKL_E3),
    IMC_UNCORE_DEV(KBL_Y),
    IMC_UNCORE_DEV(KBL_U),
    IMC_UNCORE_DEV(KBL_UQ),
    IMC_UNCORE_DEV(KBL_SD),
    IMC_UNCORE_DEV(KBL_SQ),
    IMC_UNCORE_DEV(KBL_HQ),
    IMC_UNCORE_DEV(KBL_WQ),
    IMC_UNCORE_DEV(CFL_2U),
    IMC_UNCORE_DEV(CFL_4U),
    IMC_UNCORE_DEV(CFL_4H),
    IMC_UNCORE_DEV(CFL_6H),
    IMC_UNCORE_DEV(CFL_2S_D),
    IMC_UNCORE_DEV(CFL_4S_D),
    IMC_UNCORE_DEV(CFL_6S_D),
    IMC_UNCORE_DEV(CFL_8S_D),
    IMC_UNCORE_DEV(CFL_4S_W),
    IMC_UNCORE_DEV(CFL_6S_W),
    IMC_UNCORE_DEV(CFL_8S_W),
    IMC_UNCORE_DEV(CFL_4S_S),
    IMC_UNCORE_DEV(CFL_6S_S),
    IMC_UNCORE_DEV(CFL_8S_S),
    IMC_UNCORE_DEV(AML_YD),
    IMC_UNCORE_DEV(AML_YQ),
    IMC_UNCORE_DEV(WHL_UQ),
    IMC_UNCORE_DEV(WHL_4_UQ),
    IMC_UNCORE_DEV(WHL_UD),
    IMC_UNCORE_DEV(CML_H1),
    IMC_UNCORE_DEV(CML_H2),
    IMC_UNCORE_DEV(CML_H3),
    IMC_UNCORE_DEV(CML_U1),
    IMC_UNCORE_DEV(CML_U2),
    IMC_UNCORE_DEV(CML_U3),
    IMC_UNCORE_DEV(CML_S1),
    IMC_UNCORE_DEV(CML_S2),
    IMC_UNCORE_DEV(CML_S3),
    IMC_UNCORE_DEV(CML_S4),
    IMC_UNCORE_DEV(CML_S5),
    { /* end: all zeroes */ },
    };
    static const struct pci_device_id icl_uncore_pci_ids[] = {
    IMC_UNCORE_DEV(ICL_U),
    IMC_UNCORE_DEV(ICL_U2),
    IMC_UNCORE_DEV(RKL_1),
    IMC_UNCORE_DEV(RKL_2),
    { /* end: all zeroes */ },
    };
    static struct pci_driver snb_uncore_pci_driver = {
    .name		= "snb_uncore",
    .id_table	= snb_uncore_pci_ids,
    };
    static struct pci_driver ivb_uncore_pci_driver = {
    .name		= "ivb_uncore",
    .id_table	= ivb_uncore_pci_ids,
    };
    static struct pci_driver hsw_uncore_pci_driver = {
    .name		= "hsw_uncore",
    .id_table	= hsw_uncore_pci_ids,
    };
    static struct pci_driver bdw_uncore_pci_driver = {
    .name		= "bdw_uncore",
    .id_table	= bdw_uncore_pci_ids,
    };
    static struct pci_driver skl_uncore_pci_driver = {
    .name		= "skl_uncore",
    .id_table	= skl_uncore_pci_ids,
    };
    static struct pci_driver icl_uncore_pci_driver = {
    .name		= "icl_uncore",
    .id_table	= icl_uncore_pci_ids,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imc_uncore_pci_dev {
    pub pci_id: __u32,
    pub driver: *mut pci_driver,
}

    { .pci_id = PCI_DEVICE_ID_INTEL_##a, .driver = (d) }
    static const struct imc_uncore_pci_dev desktop_imc_pci_ids[] = {
    IMC_DEV(SNB_IMC, &snb_uncore_pci_driver),
    IMC_DEV(IVB_IMC, &ivb_uncore_pci_driver),    /* 3rd Gen Core processor */
    IMC_DEV(IVB_E3_IMC, &ivb_uncore_pci_driver), /* Xeon E3-1200 v2/3rd Gen Core processor */
    IMC_DEV(HSW_IMC, &hsw_uncore_pci_driver),    /* 4th Gen Core Processor */
    IMC_DEV(HSW_U_IMC, &hsw_uncore_pci_driver),  /* 4th Gen Core ULT Mobile Processor */
    IMC_DEV(BDW_IMC, &bdw_uncore_pci_driver),    /* 5th Gen Core U */
    IMC_DEV(SKL_Y_IMC, &skl_uncore_pci_driver),  /* 6th Gen Core Y */
    IMC_DEV(SKL_U_IMC, &skl_uncore_pci_driver),  /* 6th Gen Core U */
    IMC_DEV(SKL_HD_IMC, &skl_uncore_pci_driver),  /* 6th Gen Core H Dual Core */
    IMC_DEV(SKL_HQ_IMC, &skl_uncore_pci_driver),  /* 6th Gen Core H Quad Core */
    IMC_DEV(SKL_SD_IMC, &skl_uncore_pci_driver),  /* 6th Gen Core S Dual Core */
    IMC_DEV(SKL_SQ_IMC, &skl_uncore_pci_driver),  /* 6th Gen Core S Quad Core */
    IMC_DEV(SKL_E3_IMC, &skl_uncore_pci_driver),  /* Xeon E3 V5 Gen Core processor */
    IMC_DEV(KBL_Y_IMC, &skl_uncore_pci_driver),  /* 7th Gen Core Y */
    IMC_DEV(KBL_U_IMC, &skl_uncore_pci_driver),  /* 7th Gen Core U */
    IMC_DEV(KBL_UQ_IMC, &skl_uncore_pci_driver),  /* 7th Gen Core U Quad Core */
    IMC_DEV(KBL_SD_IMC, &skl_uncore_pci_driver),  /* 7th Gen Core S Dual Core */
    IMC_DEV(KBL_SQ_IMC, &skl_uncore_pci_driver),  /* 7th Gen Core S Quad Core */
    IMC_DEV(KBL_HQ_IMC, &skl_uncore_pci_driver),  /* 7th Gen Core H Quad Core */
    IMC_DEV(KBL_WQ_IMC, &skl_uncore_pci_driver),  /* 7th Gen Core S 4 cores Work Station */
    IMC_DEV(CFL_2U_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core U 2 Cores */
    IMC_DEV(CFL_4U_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core U 4 Cores */
    IMC_DEV(CFL_4H_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core H 4 Cores */
    IMC_DEV(CFL_6H_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core H 6 Cores */
    IMC_DEV(CFL_2S_D_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 2 Cores Desktop */
    IMC_DEV(CFL_4S_D_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 4 Cores Desktop */
    IMC_DEV(CFL_6S_D_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 6 Cores Desktop */
    IMC_DEV(CFL_8S_D_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 8 Cores Desktop */
    IMC_DEV(CFL_4S_W_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 4 Cores Work Station */
    IMC_DEV(CFL_6S_W_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 6 Cores Work Station */
    IMC_DEV(CFL_8S_W_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 8 Cores Work Station */
    IMC_DEV(CFL_4S_S_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 4 Cores Server */
    IMC_DEV(CFL_6S_S_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 6 Cores Server */
    IMC_DEV(CFL_8S_S_IMC, &skl_uncore_pci_driver),  /* 8th Gen Core S 8 Cores Server */
    IMC_DEV(AML_YD_IMC, &skl_uncore_pci_driver),	/* 8th Gen Core Y Mobile Dual Core */
    IMC_DEV(AML_YQ_IMC, &skl_uncore_pci_driver),	/* 8th Gen Core Y Mobile Quad Core */
    IMC_DEV(WHL_UQ_IMC, &skl_uncore_pci_driver),	/* 8th Gen Core U Mobile Quad Core */
    IMC_DEV(WHL_4_UQ_IMC, &skl_uncore_pci_driver),	/* 8th Gen Core U Mobile Quad Core */
    IMC_DEV(WHL_UD_IMC, &skl_uncore_pci_driver),	/* 8th Gen Core U Mobile Dual Core */
    IMC_DEV(CML_H1_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_H2_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_H3_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_U1_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_U2_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_U3_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_S1_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_S2_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_S3_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_S4_IMC, &skl_uncore_pci_driver),
    IMC_DEV(CML_S5_IMC, &skl_uncore_pci_driver),
    IMC_DEV(ICL_U_IMC, &icl_uncore_pci_driver),	/* 10th Gen Core Mobile */
    IMC_DEV(ICL_U2_IMC, &icl_uncore_pci_driver),	/* 10th Gen Core Mobile */
    IMC_DEV(RKL_1_IMC, &icl_uncore_pci_driver),
    IMC_DEV(RKL_2_IMC, &icl_uncore_pci_driver),
    {  /* end marker */ }
    };

    for (x = (t); (x).pci_id; x++)
    static struct pci_driver *imc_uncore_find_dev(void)
    {
    const struct imc_uncore_pci_dev *p;
    int ret;
    for_each_imc_pci_id(p, desktop_imc_pci_ids) {
    ret = snb_pci2phy_map_init(p.pci_id);
    if (ret == 0)
    return p.driver;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn imc_uncore_pci_init() -> c_int {
    static int imc_uncore_pci_init(void)
    {
    struct pci_driver *imc_drv = imc_uncore_find_dev();
    if (!imc_drv)
    return -ENODEV;
    uncore_pci_uncores = snb_pci_uncores;
    uncore_pci_driver = imc_drv;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn snb_uncore_pci_init() -> c_int {
    int snb_uncore_pci_init(void)
    {
    return imc_uncore_pci_init();
    }
#[no_mangle]
pub unsafe extern "C" fn ivb_uncore_pci_init() -> c_int {
    int ivb_uncore_pci_init(void)
    {
    return imc_uncore_pci_init();
    }
#[no_mangle]
pub unsafe extern "C" fn hsw_uncore_pci_init() -> c_int {
    int hsw_uncore_pci_init(void)
    {
    return imc_uncore_pci_init();
    }
#[no_mangle]
pub unsafe extern "C" fn bdw_uncore_pci_init() -> c_int {
    int bdw_uncore_pci_init(void)
    {
    return imc_uncore_pci_init();
    }
#[no_mangle]
pub unsafe extern "C" fn skl_uncore_pci_init() -> c_int {
    int skl_uncore_pci_init(void)
    {
    return imc_uncore_pci_init();
    }
// end of Sandy Bridge uncore support
// Nehalem uncore support
#[no_mangle]
unsafe extern "C" fn nhm_uncore_msr_disable_box(box: *mut intel_uncore_box) {
    static void nhm_uncore_msr_disable_box(struct intel_uncore_box *box)
    {
    wrmsrq(NHM_UNC_PERF_GLOBAL_CTL, 0);
    }
#[no_mangle]
unsafe extern "C" fn nhm_uncore_msr_enable_box(box: *mut intel_uncore_box) {
    static void nhm_uncore_msr_enable_box(struct intel_uncore_box *box)
    {
    wrmsrq(NHM_UNC_PERF_GLOBAL_CTL, NHM_UNC_GLOBAL_CTL_EN_PC_ALL | NHM_UNC_GLOBAL_CTL_EN_FC);
    }
#[no_mangle]
unsafe extern "C" fn nhm_uncore_msr_enable_event(box: *mut intel_uncore_box, event: *mut perf_event) {
    static void nhm_uncore_msr_enable_event(struct intel_uncore_box *box, struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    if (hwc.idx < UNCORE_PMC_IDX_FIXED)
    wrmsrq(hwc.config_base, hwc.config | SNB_UNC_CTL_EN);
    else
    wrmsrq(hwc.config_base, NHM_UNC_FIXED_CTR_CTL_EN);
    }
    static struct attribute *nhm_uncore_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_umask.attr,
    &format_attr_edge.attr,
    &format_attr_inv.attr,
    &format_attr_cmask8.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group nhm_uncore_format_group = {
    .name = "format",
    .attrs = nhm_uncore_formats_attr,
    };
    static struct uncore_event_desc nhm_uncore_events[] = {
    INTEL_UNCORE_EVENT_DESC(clockticks,                "event=0xff,umask=0x00"),
    INTEL_UNCORE_EVENT_DESC(qmc_writes_full_any,       "event=0x2f,umask=0x0f"),
    INTEL_UNCORE_EVENT_DESC(qmc_normal_reads_any,      "event=0x2c,umask=0x0f"),
    INTEL_UNCORE_EVENT_DESC(qhl_request_ioh_reads,     "event=0x20,umask=0x01"),
    INTEL_UNCORE_EVENT_DESC(qhl_request_ioh_writes,    "event=0x20,umask=0x02"),
    INTEL_UNCORE_EVENT_DESC(qhl_request_remote_reads,  "event=0x20,umask=0x04"),
    INTEL_UNCORE_EVENT_DESC(qhl_request_remote_writes, "event=0x20,umask=0x08"),
    INTEL_UNCORE_EVENT_DESC(qhl_request_local_reads,   "event=0x20,umask=0x10"),
    INTEL_UNCORE_EVENT_DESC(qhl_request_local_writes,  "event=0x20,umask=0x20"),
    { /* end: all zeroes */ },
    };
    static struct intel_uncore_ops nhm_uncore_msr_ops = {
    .disable_box	= nhm_uncore_msr_disable_box,
    .enable_box	= nhm_uncore_msr_enable_box,
    .disable_event	= snb_uncore_msr_disable_event,
    .enable_event	= nhm_uncore_msr_enable_event,
    .read_counter	= uncore_msr_read_counter,
    };
    static struct intel_uncore_type nhm_uncore = {
    .name		= "",
    .num_counters   = 8,
    .num_boxes	= 1,
    .perf_ctr_bits	= 48,
    .fixed_ctr_bits	= 48,
    .event_ctl	= NHM_UNC_PERFEVTSEL0,
    .perf_ctr	= NHM_UNC_UNCORE_PMC0,
    .fixed_ctr	= NHM_UNC_FIXED_CTR,
    .fixed_ctl	= NHM_UNC_FIXED_CTR_CTRL,
    .event_mask	= NHM_UNC_RAW_EVENT_MASK,
    .event_descs	= nhm_uncore_events,
    .ops		= &nhm_uncore_msr_ops,
    .format_group	= &nhm_uncore_format_group,
    };
    static struct intel_uncore_type *nhm_msr_uncores[] = {
    &nhm_uncore,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn nhm_uncore_cpu_init() {
    void nhm_uncore_cpu_init(void)
    {
    uncore_msr_uncores = nhm_msr_uncores;
    }
// end of Nehalem uncore support
// Tiger Lake MMIO uncore support
    static const struct pci_device_id tgl_uncore_pci_ids[] = {
    IMC_UNCORE_DEV(TGL_U1),
    IMC_UNCORE_DEV(TGL_U2),
    IMC_UNCORE_DEV(TGL_U3),
    IMC_UNCORE_DEV(TGL_U4),
    IMC_UNCORE_DEV(TGL_H),
    IMC_UNCORE_DEV(ADL_1),
    IMC_UNCORE_DEV(ADL_2),
    IMC_UNCORE_DEV(ADL_3),
    IMC_UNCORE_DEV(ADL_4),
    IMC_UNCORE_DEV(ADL_5),
    IMC_UNCORE_DEV(ADL_6),
    IMC_UNCORE_DEV(ADL_7),
    IMC_UNCORE_DEV(ADL_8),
    IMC_UNCORE_DEV(ADL_9),
    IMC_UNCORE_DEV(ADL_10),
    IMC_UNCORE_DEV(ADL_11),
    IMC_UNCORE_DEV(ADL_12),
    IMC_UNCORE_DEV(ADL_13),
    IMC_UNCORE_DEV(ADL_14),
    IMC_UNCORE_DEV(ADL_15),
    IMC_UNCORE_DEV(ADL_16),
    IMC_UNCORE_DEV(ADL_17),
    IMC_UNCORE_DEV(ADL_18),
    IMC_UNCORE_DEV(ADL_19),
    IMC_UNCORE_DEV(ADL_20),
    IMC_UNCORE_DEV(ADL_21),
    IMC_UNCORE_DEV(RPL_1),
    IMC_UNCORE_DEV(RPL_2),
    IMC_UNCORE_DEV(RPL_3),
    IMC_UNCORE_DEV(RPL_4),
    IMC_UNCORE_DEV(RPL_5),
    IMC_UNCORE_DEV(RPL_6),
    IMC_UNCORE_DEV(RPL_7),
    IMC_UNCORE_DEV(RPL_8),
    IMC_UNCORE_DEV(RPL_9),
    IMC_UNCORE_DEV(RPL_10),
    IMC_UNCORE_DEV(RPL_11),
    IMC_UNCORE_DEV(RPL_12),
    IMC_UNCORE_DEV(RPL_13),
    IMC_UNCORE_DEV(RPL_14),
    IMC_UNCORE_DEV(RPL_15),
    IMC_UNCORE_DEV(RPL_16),
    IMC_UNCORE_DEV(RPL_17),
    IMC_UNCORE_DEV(RPL_18),
    IMC_UNCORE_DEV(RPL_19),
    IMC_UNCORE_DEV(RPL_20),
    IMC_UNCORE_DEV(RPL_21),
    IMC_UNCORE_DEV(RPL_22),
    IMC_UNCORE_DEV(RPL_23),
    IMC_UNCORE_DEV(RPL_24),
    IMC_UNCORE_DEV(RPL_25),
    IMC_UNCORE_DEV(MTL_1),
    IMC_UNCORE_DEV(MTL_2),
    IMC_UNCORE_DEV(MTL_3),
    IMC_UNCORE_DEV(MTL_4),
    IMC_UNCORE_DEV(MTL_5),
    IMC_UNCORE_DEV(MTL_6),
    IMC_UNCORE_DEV(MTL_7),
    IMC_UNCORE_DEV(MTL_8),
    IMC_UNCORE_DEV(MTL_9),
    IMC_UNCORE_DEV(MTL_10),
    IMC_UNCORE_DEV(MTL_11),
    IMC_UNCORE_DEV(MTL_12),
    IMC_UNCORE_DEV(MTL_13),
    { /* end: all zeroes */ }
    };
    enum perf_tgl_uncore_imc_freerunning_types {
    TGL_MMIO_UNCORE_IMC_DATA_TOTAL,
    TGL_MMIO_UNCORE_IMC_DATA_READ,
    TGL_MMIO_UNCORE_IMC_DATA_WRITE,
    TGL_MMIO_UNCORE_IMC_FREERUNNING_TYPE_MAX
    };
    static struct freerunning_counters tgl_l_uncore_imc_freerunning[] = {
    [TGL_MMIO_UNCORE_IMC_DATA_TOTAL]	= { 0x5040, 0x0, 0x0, 1, 64 },
    [TGL_MMIO_UNCORE_IMC_DATA_READ]		= { 0x5058, 0x0, 0x0, 1, 64 },
    [TGL_MMIO_UNCORE_IMC_DATA_WRITE]	= { 0x50A0, 0x0, 0x0, 1, 64 },
    };
    static struct freerunning_counters tgl_uncore_imc_freerunning[] = {
    [TGL_MMIO_UNCORE_IMC_DATA_TOTAL]	= { 0xd840, 0x0, 0x0, 1, 64 },
    [TGL_MMIO_UNCORE_IMC_DATA_READ]		= { 0xd858, 0x0, 0x0, 1, 64 },
    [TGL_MMIO_UNCORE_IMC_DATA_WRITE]	= { 0xd8A0, 0x0, 0x0, 1, 64 },
    };
    static struct uncore_event_desc tgl_uncore_imc_events[] = {
    INTEL_UNCORE_EVENT_DESC(data_total,         "event=0xff,umask=0x10"),
    INTEL_UNCORE_EVENT_DESC(data_total.scale,   "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(data_total.unit,    "MiB"),
    INTEL_UNCORE_EVENT_DESC(data_read,         "event=0xff,umask=0x20"),
    INTEL_UNCORE_EVENT_DESC(data_read.scale,   "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(data_read.unit,    "MiB"),
    INTEL_UNCORE_EVENT_DESC(data_write,        "event=0xff,umask=0x30"),
    INTEL_UNCORE_EVENT_DESC(data_write.scale,  "6.103515625e-5"),
    INTEL_UNCORE_EVENT_DESC(data_write.unit,   "MiB"),
    { /* end: all zeroes */ }
    };
    static struct pci_dev *tgl_uncore_get_mc_dev(void)
    {
    const struct pci_device_id *ids = tgl_uncore_pci_ids;
    struct pci_dev *mc_dev = core::ptr::null_mut();
    while (ids && ids.vendor) {
    mc_dev = pci_get_device(PCI_VENDOR_ID_INTEL, ids.device, core::ptr::null_mut());
    if (mc_dev)
    return mc_dev;
    ids++;
    }
// Just try to grab 00:00.0 device
    if (!mc_dev)
    mc_dev = pci_get_domain_bus_and_slot(0, 0, PCI_DEVFN(0, 0));
    return mc_dev;
    }
pub const TGL_UNCORE_MMIO_IMC_MEM_OFFSET: c_uint = 0x10000;
pub const TGL_UNCORE_PCI_IMC_MAP_SIZE: c_uint = 0xe000;
    static int
    uncore_get_box_mmio_addr(struct intel_uncore_box *box,
    unsigned int base_offset,
    int bar_offset, int step)
    {
    struct pci_dev *pdev = tgl_uncore_get_mc_dev();
    struct intel_uncore_pmu *pmu = box.pmu;
    struct intel_uncore_type *type = pmu.type;
    resource_size_t addr;
    let mut ret: c_int = 0;
    u32 bar;
    if (!pdev) {
    pr_warn("perf uncore: Cannot find matched IMC device.\n");
    return -ENODEV;
    }
    pci_read_config_dword(pdev, bar_offset, &bar);
    if (!(bar & BIT(0))) {
    pr_warn("perf uncore: BAR 0x%x is disabled. Failed to map %s counters.\n",
    bar_offset, type.name);
    ret = -ENODEV;
    goto out;
    }
    bar &= ~BIT(0);
    addr = (resource_size_t)(bar + step * pmu.pmu_idx);

    pci_read_config_dword(pdev, bar_offset + 4, &bar);
    addr |= ((resource_size_t)bar << 32);

    addr += base_offset;
    box.io_addr = ioremap(addr, type.mmio_map_size);
    if (!box.io_addr) {
    ret = -ENOMEM;
    pr_warn("perf uncore: Failed to ioremap for %s.\n", type.name);
    }
    out:
    pci_dev_put(pdev);
    return ret;
    }
    static int __uncore_imc_init_box(struct intel_uncore_box *box,
    unsigned int base_offset)
    {
    return uncore_get_box_mmio_addr(box, base_offset,
    SNB_UNCORE_PCI_IMC_BAR_OFFSET,
    TGL_UNCORE_MMIO_IMC_MEM_OFFSET);
    }
#[no_mangle]
unsafe extern "C" fn tgl_uncore_imc_freerunning_init_box(box: *mut intel_uncore_box) -> c_int {
    static int tgl_uncore_imc_freerunning_init_box(struct intel_uncore_box *box)
    {
    return __uncore_imc_init_box(box, 0);
    }
    static struct intel_uncore_ops tgl_uncore_imc_freerunning_ops = {
    .init_box	= tgl_uncore_imc_freerunning_init_box,
    .exit_box	= uncore_mmio_exit_box,
    .read_counter	= uncore_mmio_read_counter,
    .hw_config	= uncore_freerunning_hw_config,
    };
    static struct attribute *tgl_uncore_imc_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_umask.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group tgl_uncore_imc_format_group = {
    .name = "format",
    .attrs = tgl_uncore_imc_formats_attr,
    };
    static struct intel_uncore_type tgl_uncore_imc_free_running = {
    .name			= "imc_free_running",
    .num_counters		= 3,
    .num_boxes		= 2,
    .num_freerunning_types	= TGL_MMIO_UNCORE_IMC_FREERUNNING_TYPE_MAX,
    .mmio_map_size		= TGL_UNCORE_PCI_IMC_MAP_SIZE,
    .freerunning		= tgl_uncore_imc_freerunning,
    .ops			= &tgl_uncore_imc_freerunning_ops,
    .event_descs		= tgl_uncore_imc_events,
    .format_group		= &tgl_uncore_imc_format_group,
    };
    static struct intel_uncore_type *tgl_mmio_uncores[] = {
    &tgl_uncore_imc_free_running,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn tgl_l_uncore_mmio_init() {
    void tgl_l_uncore_mmio_init(void)
    {
    tgl_uncore_imc_free_running.freerunning = tgl_l_uncore_imc_freerunning;
    uncore_mmio_uncores = tgl_mmio_uncores;
    }
#[no_mangle]
pub unsafe extern "C" fn tgl_uncore_mmio_init() {
    void tgl_uncore_mmio_init(void)
    {
    uncore_mmio_uncores = tgl_mmio_uncores;
    }
// end of Tiger Lake MMIO uncore support
// Alder Lake MMIO uncore support
pub const ADL_UNCORE_IMC_BASE: c_uint = 0xd900;
pub const ADL_UNCORE_IMC_MAP_SIZE: c_uint = 0x200;
pub const ADL_UNCORE_IMC_CTR: c_uint = 0xe8;
pub const ADL_UNCORE_IMC_CTRL: c_uint = 0xd0;
pub const ADL_UNCORE_IMC_GLOBAL_CTL: c_uint = 0xc0;
pub const ADL_UNCORE_IMC_BOX_CTL: c_uint = 0xc4;
pub const ADL_UNCORE_IMC_FREERUNNING_BASE: c_uint = 0xd800;
pub const ADL_UNCORE_IMC_FREERUNNING_MAP_SIZE: c_uint = 0x100;

    ADL_UNCORE_IMC_CTL_RST_CTRS)
#[no_mangle]
unsafe extern "C" fn adl_uncore_imc_init_box(box: *mut intel_uncore_box) -> c_int {
    static int adl_uncore_imc_init_box(struct intel_uncore_box *box)
    {
    let mut ret: c_int = __uncore_imc_init_box(box, ADL_UNCORE_IMC_BASE);
// The global control in MC1 can control both MCs.
    if (!ret && (box.pmu.pmu_idx == 1))
    writel(ADL_UNCORE_IMC_CTL_INT, box.io_addr + ADL_UNCORE_IMC_GLOBAL_CTL);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn adl_uncore_mmio_disable_box(box: *mut intel_uncore_box) {
    static void adl_uncore_mmio_disable_box(struct intel_uncore_box *box)
    {
    if (!box.io_addr)
    return;
    writel(ADL_UNCORE_IMC_CTL_FRZ, box.io_addr + uncore_mmio_box_ctl(box));
    }
#[no_mangle]
unsafe extern "C" fn adl_uncore_mmio_enable_box(box: *mut intel_uncore_box) {
    static void adl_uncore_mmio_enable_box(struct intel_uncore_box *box)
    {
    if (!box.io_addr)
    return;
    writel(0, box.io_addr + uncore_mmio_box_ctl(box));
    }

    .exit_box	= uncore_mmio_exit_box,		\
    .disable_box	= adl_uncore_mmio_disable_box,	\
    .enable_box	= adl_uncore_mmio_enable_box,	\
    .disable_event	= intel_generic_uncore_mmio_disable_event,	\
    .enable_event	= intel_generic_uncore_mmio_enable_event,	\
    .read_counter	= uncore_mmio_read_counter,
    static struct intel_uncore_ops adl_uncore_mmio_ops = {
    .init_box	= adl_uncore_imc_init_box,
    MMIO_UNCORE_COMMON_OPS()
    };
pub const ADL_UNC_CTL_CHMASK_MASK: c_uint = 0x00000f00;

    ADL_UNC_CTL_CHMASK_MASK | \
    SNB_UNC_CTL_EDGE_DET)
    static struct attribute *adl_uncore_imc_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_chmask.attr,
    &format_attr_edge.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group adl_uncore_imc_format_group = {
    .name		= "format",
    .attrs		= adl_uncore_imc_formats_attr,
    };
    static struct intel_uncore_type adl_uncore_imc = {
    .name		= "imc",
    .num_counters   = 5,
    .num_boxes	= 2,
    .perf_ctr_bits	= 64,
    .perf_ctr	= ADL_UNCORE_IMC_CTR,
    .event_ctl	= ADL_UNCORE_IMC_CTRL,
    .event_mask	= ADL_UNC_IMC_EVENT_MASK,
    .box_ctl	= ADL_UNCORE_IMC_BOX_CTL,
    .mmio_offset	= 0,
    .mmio_map_size	= ADL_UNCORE_IMC_MAP_SIZE,
    .ops		= &adl_uncore_mmio_ops,
    .format_group	= &adl_uncore_imc_format_group,
    };
    enum perf_adl_uncore_imc_freerunning_types {
    ADL_MMIO_UNCORE_IMC_DATA_TOTAL,
    ADL_MMIO_UNCORE_IMC_DATA_READ,
    ADL_MMIO_UNCORE_IMC_DATA_WRITE,
    ADL_MMIO_UNCORE_IMC_FREERUNNING_TYPE_MAX
    };
    static struct freerunning_counters adl_uncore_imc_freerunning[] = {
    [ADL_MMIO_UNCORE_IMC_DATA_TOTAL]	= { 0x40, 0x0, 0x0, 1, 64 },
    [ADL_MMIO_UNCORE_IMC_DATA_READ]		= { 0x58, 0x0, 0x0, 1, 64 },
    [ADL_MMIO_UNCORE_IMC_DATA_WRITE]	= { 0xA0, 0x0, 0x0, 1, 64 },
    };
#[no_mangle]
unsafe extern "C" fn adl_uncore_imc_freerunning_init_box(box: *mut intel_uncore_box) -> c_int {
    static int adl_uncore_imc_freerunning_init_box(struct intel_uncore_box *box)
    {
    return __uncore_imc_init_box(box, ADL_UNCORE_IMC_FREERUNNING_BASE);
    }
    static struct intel_uncore_ops adl_uncore_imc_freerunning_ops = {
    .init_box	= adl_uncore_imc_freerunning_init_box,
    .exit_box	= uncore_mmio_exit_box,
    .read_counter	= uncore_mmio_read_counter,
    .hw_config	= uncore_freerunning_hw_config,
    };
    static struct intel_uncore_type adl_uncore_imc_free_running = {
    .name			= "imc_free_running",
    .num_counters		= 3,
    .num_boxes		= 2,
    .num_freerunning_types	= ADL_MMIO_UNCORE_IMC_FREERUNNING_TYPE_MAX,
    .mmio_map_size		= ADL_UNCORE_IMC_FREERUNNING_MAP_SIZE,
    .freerunning		= adl_uncore_imc_freerunning,
    .ops			= &adl_uncore_imc_freerunning_ops,
    .event_descs		= tgl_uncore_imc_events,
    .format_group		= &tgl_uncore_imc_format_group,
    };
    static struct intel_uncore_type *adl_mmio_uncores[] = {
    &adl_uncore_imc,
    &adl_uncore_imc_free_running,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn adl_uncore_mmio_init() {
    void adl_uncore_mmio_init(void)
    {
    uncore_mmio_uncores = adl_mmio_uncores;
    }
// end of Alder Lake MMIO uncore support
// Lunar Lake MMIO uncore support
pub const LNL_UNCORE_PCI_SAFBAR_OFFSET: c_uint = 0x68;
pub const LNL_UNCORE_MAP_SIZE: c_uint = 0x1000;
pub const LNL_UNCORE_SNCU_BASE: c_uint = 0xE4B000;
pub const LNL_UNCORE_SNCU_CTR: c_uint = 0x390;
pub const LNL_UNCORE_SNCU_CTRL: c_uint = 0x398;
pub const LNL_UNCORE_SNCU_BOX_CTL: c_uint = 0x380;
pub const LNL_UNCORE_GLOBAL_CTL: c_uint = 0x700;
pub const LNL_UNCORE_HBO_BASE: c_uint = 0xE54000;

pub const LNL_UNCORE_HBO_CTR: c_uint = 0x570;
pub const LNL_UNCORE_HBO_CTRL: c_uint = 0x550;
pub const LNL_UNCORE_HBO_BOX_CTL: c_uint = 0x548;
pub const LNL_UNC_CTL_THRESHOLD: c_uint = 0xff000000;

    SNB_UNC_CTL_UMASK_MASK | \
    SNB_UNC_CTL_EDGE_DET | \
    SNB_UNC_CTL_INVERT | \
    LNL_UNC_CTL_THRESHOLD)
    static struct attribute *lnl_uncore_formats_attr[] = {
    &format_attr_event.attr,
    &format_attr_umask.attr,
    &format_attr_edge.attr,
    &format_attr_inv.attr,
    &format_attr_threshold2.attr,
    core::ptr::null_mut()
    };
    static const struct attribute_group lnl_uncore_format_group = {
    .name		= "format",
    .attrs		= lnl_uncore_formats_attr,
    };
#[no_mangle]
unsafe extern "C" fn lnl_uncore_hbo_init_box(box: *mut intel_uncore_box) -> c_int {
    static int lnl_uncore_hbo_init_box(struct intel_uncore_box *box)
    {
    return uncore_get_box_mmio_addr(box, LNL_UNCORE_HBO_BASE,
    LNL_UNCORE_PCI_SAFBAR_OFFSET,
    LNL_UNCORE_HBO_OFFSET);
    }
    static struct intel_uncore_ops lnl_uncore_hbo_ops = {
    .init_box	= lnl_uncore_hbo_init_box,
    MMIO_UNCORE_COMMON_OPS()
    };
    static struct intel_uncore_type lnl_uncore_hbo = {
    .name		= "hbo",
    .num_counters   = 4,
    .num_boxes	= 2,
    .perf_ctr_bits	= 64,
    .perf_ctr	= LNL_UNCORE_HBO_CTR,
    .event_ctl	= LNL_UNCORE_HBO_CTRL,
    .event_mask	= LNL_UNC_RAW_EVENT_MASK,
    .box_ctl	= LNL_UNCORE_HBO_BOX_CTL,
    .mmio_map_size	= LNL_UNCORE_MAP_SIZE,
    .ops		= &lnl_uncore_hbo_ops,
    .format_group	= &lnl_uncore_format_group,
    };
#[no_mangle]
unsafe extern "C" fn lnl_uncore_sncu_init_box(box: *mut intel_uncore_box) -> c_int {
    static int lnl_uncore_sncu_init_box(struct intel_uncore_box *box)
    {
    int ret = uncore_get_box_mmio_addr(box, LNL_UNCORE_SNCU_BASE,
    LNL_UNCORE_PCI_SAFBAR_OFFSET,
    0);
    if (!ret)
    writel(ADL_UNCORE_IMC_CTL_INT, box.io_addr + LNL_UNCORE_GLOBAL_CTL);
    return ret;
    }
    static struct intel_uncore_ops lnl_uncore_sncu_ops = {
    .init_box	= lnl_uncore_sncu_init_box,
    MMIO_UNCORE_COMMON_OPS()
    };
    static struct intel_uncore_type lnl_uncore_sncu = {
    .name		= "sncu",
    .num_counters   = 2,
    .num_boxes	= 1,
    .perf_ctr_bits	= 64,
    .perf_ctr	= LNL_UNCORE_SNCU_CTR,
    .event_ctl	= LNL_UNCORE_SNCU_CTRL,
    .event_mask	= LNL_UNC_RAW_EVENT_MASK,
    .box_ctl	= LNL_UNCORE_SNCU_BOX_CTL,
    .mmio_map_size	= LNL_UNCORE_MAP_SIZE,
    .ops		= &lnl_uncore_sncu_ops,
    .format_group	= &lnl_uncore_format_group,
    };
    static struct intel_uncore_type *lnl_mmio_uncores[] = {
    &adl_uncore_imc,
    &lnl_uncore_hbo,
    &lnl_uncore_sncu,
    &adl_uncore_imc_free_running,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn lnl_uncore_mmio_init() {
    void lnl_uncore_mmio_init(void)
    {
    uncore_mmio_uncores = lnl_mmio_uncores;
    }
// end of Lunar Lake MMIO uncore support
// Panther Lake uncore support
pub const UNCORE_PTL_MAX_NUM_UNCORE_TYPES: c_int = 42;
pub const UNCORE_PTL_TYPE_IMC: c_int = 6;
pub const UNCORE_PTL_TYPE_SNCU: c_int = 34;
pub const UNCORE_PTL_TYPE_HBO: c_int = 41;
pub const PTL_UNCORE_GLOBAL_CTL_OFFSET: c_uint = 0x380;
    static struct intel_uncore_type ptl_uncore_imc = {
    .name			= "imc",
    .mmio_map_size		= 0xf00,
    };
#[no_mangle]
unsafe extern "C" fn ptl_uncore_sncu_init_box(box: *mut intel_uncore_box) -> c_int {
    static int ptl_uncore_sncu_init_box(struct intel_uncore_box *box)
    {
    let mut ret: c_int = intel_generic_uncore_mmio_init_box(box);
// Clear the global freeze bit
    if (box.io_addr)
    writel(0, box.io_addr + PTL_UNCORE_GLOBAL_CTL_OFFSET);
    return ret;
    }
    static struct intel_uncore_ops ptl_uncore_sncu_ops = {
    .init_box		= ptl_uncore_sncu_init_box,
    .exit_box		= uncore_mmio_exit_box,
    .disable_box		= intel_generic_uncore_mmio_disable_box,
    .enable_box		= intel_generic_uncore_mmio_enable_box,
    .disable_event		= intel_generic_uncore_mmio_disable_event,
    .enable_event		= intel_generic_uncore_mmio_enable_event,
    .read_counter		= uncore_mmio_read_counter,
    };
    static struct intel_uncore_type ptl_uncore_sncu = {
    .name			= "sncu",
    .ops			= &ptl_uncore_sncu_ops,
    .mmio_map_size		= 0xf00,
    };
    static struct intel_uncore_type ptl_uncore_hbo = {
    .name			= "hbo",
    .mmio_map_size		= 0xf00,
    };
    static struct intel_uncore_type *ptl_uncores[UNCORE_PTL_MAX_NUM_UNCORE_TYPES] = {
    [UNCORE_PTL_TYPE_IMC] = &ptl_uncore_imc,
    [UNCORE_PTL_TYPE_SNCU] = &ptl_uncore_sncu,
    [UNCORE_PTL_TYPE_HBO] = &ptl_uncore_hbo,
    };
pub const UNCORE_PTL_MMIO_EXTRA_UNCORES: c_int = 1;
    static struct intel_uncore_type *ptl_mmio_extra_uncores[UNCORE_PTL_MMIO_EXTRA_UNCORES] = {
    &adl_uncore_imc_free_running,
    };
#[no_mangle]
pub unsafe extern "C" fn ptl_uncore_mmio_init() {
    void ptl_uncore_mmio_init(void)
    {
    uncore_mmio_uncores = uncore_get_uncores(UNCORE_ACCESS_MMIO,
    UNCORE_PTL_MMIO_EXTRA_UNCORES,
    ptl_mmio_extra_uncores,
    UNCORE_PTL_MAX_NUM_UNCORE_TYPES,
    ptl_uncores);
    }
    static struct intel_uncore_type ptl_uncore_ia_core_bridge = {
    .name		= "ia_core_bridge",
    .num_counters   = 2,
    .num_boxes	= 1,
    .perf_ctr_bits	= 48,
    .perf_ctr	= PTL_UNC_IA_CORE_BRIDGE_PER_CTR0,
    .event_ctl	= PTL_UNC_IA_CORE_BRIDGE_PERFEVTSEL0,
    .event_mask	= ADL_UNC_RAW_EVENT_MASK,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &adl_uncore_format_group,
    };
    static struct intel_uncore_type ptl_uncore_santa = {
    .name		= "santa",
    .num_counters   = 2,
    .num_boxes	= 2,
    .perf_ctr_bits	= 48,
    .perf_ctr	= PTL_UNC_SANTA_CTR0,
    .event_ctl	= PTL_UNC_SANTA_CTRL0,
    .event_mask	= ADL_UNC_RAW_EVENT_MASK,
    .msr_offset	= SNB_UNC_CBO_MSR_OFFSET,
    .ops		= &icl_uncore_msr_ops,
    .format_group	= &adl_uncore_format_group,
    };
    static struct intel_uncore_type *ptl_msr_uncores[] = {
    &mtl_uncore_cbox,
    &ptl_uncore_ia_core_bridge,
    &ptl_uncore_santa,
    &mtl_uncore_cncu,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn ptl_uncore_cpu_init() {
    void ptl_uncore_cpu_init(void)
    {
    mtl_uncore_cbox.num_boxes = 6;
    mtl_uncore_cbox.ops = &lnl_uncore_msr_ops;
    mtl_uncore_cncu.num_counters = 2;
    mtl_uncore_cncu.num_boxes = 2;
    mtl_uncore_cncu.msr_offset = PTL_UNC_CNCU_MSR_OFFSET;
    mtl_uncore_cncu.single_fixed = 0;
    uncore_msr_uncores = ptl_msr_uncores;
    }
// end of Panther Lake uncore support
// Nova Lake uncore support
    static struct intel_uncore_type *nvl_msr_uncores[] = {
    &mtl_uncore_cbox,
    &ptl_uncore_santa,
    &mtl_uncore_cncu,
    core::ptr::null_mut()
    };
#[no_mangle]
pub unsafe extern "C" fn nvl_uncore_cpu_init() {
    void nvl_uncore_cpu_init(void)
    {
    mtl_uncore_cbox.num_boxes = 12;
    mtl_uncore_cbox.perf_ctr = NVL_UNC_CBOX_PER_CTR0;
    mtl_uncore_cbox.event_ctl = NVL_UNC_CBOX_PERFEVTSEL0;
    ptl_uncore_santa.perf_ctr = NVL_UNC_SANTA_CTR0;
    ptl_uncore_santa.event_ctl = NVL_UNC_SANTA_CTRL0;
    mtl_uncore_cncu.box_ctl = NVL_UNC_CNCU_BOX_CTL;
    mtl_uncore_cncu.fixed_ctr = NVL_UNC_CNCU_FIXED_CTR;
    mtl_uncore_cncu.fixed_ctl = NVL_UNC_CNCU_FIXED_CTRL;
    uncore_msr_uncores = nvl_msr_uncores;
    }
// end of Nova Lake uncore support
