//! Automatically rewritten from C to Rust
//! Source: drivers/perf/hisilicon/hns3_pmu.c
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
// This driver adds support for HNS3 PMU iEP device. Related perf events are
// bandwidth, latency, packet rate, interrupt rate etc.
//
// Copyright (C) 2022 HiSilicon Limited
//

// registers offset address
pub const HNS3_PMU_REG_GLOBAL_CTRL: c_uint = 0x0000;
pub const HNS3_PMU_REG_CLOCK_FREQ: c_uint = 0x0020;
pub const HNS3_PMU_REG_BDF: c_uint = 0x0fe0;
pub const HNS3_PMU_REG_VERSION: c_uint = 0x0fe4;
pub const HNS3_PMU_REG_DEVICE_ID: c_uint = 0x0fe8;
pub const HNS3_PMU_REG_EVENT_OFFSET: c_uint = 0x1000;
pub const HNS3_PMU_REG_EVENT_SIZE: c_uint = 0x1000;
pub const HNS3_PMU_REG_EVENT_CTRL_LOW: c_uint = 0x00;
pub const HNS3_PMU_REG_EVENT_CTRL_HIGH: c_uint = 0x04;
pub const HNS3_PMU_REG_EVENT_INTR_STATUS: c_uint = 0x08;
pub const HNS3_PMU_REG_EVENT_INTR_MASK: c_uint = 0x0c;
pub const HNS3_PMU_REG_EVENT_COUNTER: c_uint = 0x10;
pub const HNS3_PMU_REG_EVENT_EXT_COUNTER: c_uint = 0x18;
pub const HNS3_PMU_REG_EVENT_QID_CTRL: c_uint = 0x28;
pub const HNS3_PMU_REG_EVENT_QID_PARA: c_uint = 0x2c;

pub const HNS3_PMU_FILTER_ALL_TC: c_uint = 0xf;
pub const HNS3_PMU_FILTER_ALL_QUEUE: c_uint = 0xffff;
pub const HNS3_PMU_CTRL_SUBEVENT_S: c_int = 4;
pub const HNS3_PMU_CTRL_FILTER_MODE_S: c_int = 24;

pub const HNS3_PMU_QID_PARA_FUNC_S: c_int = 0;
pub const HNS3_PMU_QID_PARA_QUEUE_S: c_int = 16;

pub const HNS3_PMU_MAX_HW_EVENTS: c_int = 8;
//
// Each hardware event contains two registers (counter and ext_counter) for
// bandwidth, packet rate, latency and interrupt rate. These two registers will
// be triggered to run at the same when a hardware event is enabled. The meaning
// of counter and ext_counter of different event type are different, their
// meaning show as follow:
//
// +----------------+------------------+---------------+
// |   event type   |     counter      |  ext_counter  |
// +----------------+------------------+---------------+
// | bandwidth      | byte number      | cycle number  |
// +----------------+------------------+---------------+
// | packet rate    | packet number    | cycle number  |
// +----------------+------------------+---------------+
// | latency        | cycle number     | packet number |
// +----------------+------------------+---------------+
// | interrupt rate | interrupt number | cycle number  |
// +----------------+------------------+---------------+
//
// The cycle number indicates increment of counter of hardware timer, the
// frequency of hardware timer can be read from hw_clk_freq file.
//
// Performance of each hardware event is calculated by: counter / ext_counter.
//
// Since processing of data is preferred to be done in userspace, we expose
// ext_counter as a separate event for userspace and use bit 16 to indicate it.
// For example, event 0x00001 and 0x10001 are actually one event for hardware
// because bit 0-15 are same. If the bit 16 of one event is 0 means to read
// counter register, otherwise means to read ext_counter register.
//
// bandwidth events
pub const HNS3_PMU_EVT_BW_SSU_EGU_BYTE_NUM: c_uint = 0x00001;
pub const HNS3_PMU_EVT_BW_SSU_EGU_TIME: c_uint = 0x10001;
pub const HNS3_PMU_EVT_BW_SSU_RPU_BYTE_NUM: c_uint = 0x00002;
pub const HNS3_PMU_EVT_BW_SSU_RPU_TIME: c_uint = 0x10002;
pub const HNS3_PMU_EVT_BW_SSU_ROCE_BYTE_NUM: c_uint = 0x00003;
pub const HNS3_PMU_EVT_BW_SSU_ROCE_TIME: c_uint = 0x10003;
pub const HNS3_PMU_EVT_BW_ROCE_SSU_BYTE_NUM: c_uint = 0x00004;
pub const HNS3_PMU_EVT_BW_ROCE_SSU_TIME: c_uint = 0x10004;
pub const HNS3_PMU_EVT_BW_TPU_SSU_BYTE_NUM: c_uint = 0x00005;
pub const HNS3_PMU_EVT_BW_TPU_SSU_TIME: c_uint = 0x10005;
pub const HNS3_PMU_EVT_BW_RPU_RCBRX_BYTE_NUM: c_uint = 0x00006;
pub const HNS3_PMU_EVT_BW_RPU_RCBRX_TIME: c_uint = 0x10006;
pub const HNS3_PMU_EVT_BW_RCBTX_TXSCH_BYTE_NUM: c_uint = 0x00008;
pub const HNS3_PMU_EVT_BW_RCBTX_TXSCH_TIME: c_uint = 0x10008;
pub const HNS3_PMU_EVT_BW_WR_FBD_BYTE_NUM: c_uint = 0x00009;
pub const HNS3_PMU_EVT_BW_WR_FBD_TIME: c_uint = 0x10009;
pub const HNS3_PMU_EVT_BW_WR_EBD_BYTE_NUM: c_uint = 0x0000a;
pub const HNS3_PMU_EVT_BW_WR_EBD_TIME: c_uint = 0x1000a;
pub const HNS3_PMU_EVT_BW_RD_FBD_BYTE_NUM: c_uint = 0x0000b;
pub const HNS3_PMU_EVT_BW_RD_FBD_TIME: c_uint = 0x1000b;
pub const HNS3_PMU_EVT_BW_RD_EBD_BYTE_NUM: c_uint = 0x0000c;
pub const HNS3_PMU_EVT_BW_RD_EBD_TIME: c_uint = 0x1000c;
pub const HNS3_PMU_EVT_BW_RD_PAY_M0_BYTE_NUM: c_uint = 0x0000d;
pub const HNS3_PMU_EVT_BW_RD_PAY_M0_TIME: c_uint = 0x1000d;
pub const HNS3_PMU_EVT_BW_RD_PAY_M1_BYTE_NUM: c_uint = 0x0000e;
pub const HNS3_PMU_EVT_BW_RD_PAY_M1_TIME: c_uint = 0x1000e;
pub const HNS3_PMU_EVT_BW_WR_PAY_M0_BYTE_NUM: c_uint = 0x0000f;
pub const HNS3_PMU_EVT_BW_WR_PAY_M0_TIME: c_uint = 0x1000f;
pub const HNS3_PMU_EVT_BW_WR_PAY_M1_BYTE_NUM: c_uint = 0x00010;
pub const HNS3_PMU_EVT_BW_WR_PAY_M1_TIME: c_uint = 0x10010;
// packet rate events
pub const HNS3_PMU_EVT_PPS_IGU_SSU_PACKET_NUM: c_uint = 0x00100;
pub const HNS3_PMU_EVT_PPS_IGU_SSU_TIME: c_uint = 0x10100;
pub const HNS3_PMU_EVT_PPS_SSU_EGU_PACKET_NUM: c_uint = 0x00101;
pub const HNS3_PMU_EVT_PPS_SSU_EGU_TIME: c_uint = 0x10101;
pub const HNS3_PMU_EVT_PPS_SSU_RPU_PACKET_NUM: c_uint = 0x00102;
pub const HNS3_PMU_EVT_PPS_SSU_RPU_TIME: c_uint = 0x10102;
pub const HNS3_PMU_EVT_PPS_SSU_ROCE_PACKET_NUM: c_uint = 0x00103;
pub const HNS3_PMU_EVT_PPS_SSU_ROCE_TIME: c_uint = 0x10103;
pub const HNS3_PMU_EVT_PPS_ROCE_SSU_PACKET_NUM: c_uint = 0x00104;
pub const HNS3_PMU_EVT_PPS_ROCE_SSU_TIME: c_uint = 0x10104;
pub const HNS3_PMU_EVT_PPS_TPU_SSU_PACKET_NUM: c_uint = 0x00105;
pub const HNS3_PMU_EVT_PPS_TPU_SSU_TIME: c_uint = 0x10105;
pub const HNS3_PMU_EVT_PPS_RPU_RCBRX_PACKET_NUM: c_uint = 0x00106;
pub const HNS3_PMU_EVT_PPS_RPU_RCBRX_TIME: c_uint = 0x10106;
pub const HNS3_PMU_EVT_PPS_RCBTX_TPU_PACKET_NUM: c_uint = 0x00107;
pub const HNS3_PMU_EVT_PPS_RCBTX_TPU_TIME: c_uint = 0x10107;
pub const HNS3_PMU_EVT_PPS_RCBTX_TXSCH_PACKET_NUM: c_uint = 0x00108;
pub const HNS3_PMU_EVT_PPS_RCBTX_TXSCH_TIME: c_uint = 0x10108;
pub const HNS3_PMU_EVT_PPS_WR_FBD_PACKET_NUM: c_uint = 0x00109;
pub const HNS3_PMU_EVT_PPS_WR_FBD_TIME: c_uint = 0x10109;
pub const HNS3_PMU_EVT_PPS_WR_EBD_PACKET_NUM: c_uint = 0x0010a;
pub const HNS3_PMU_EVT_PPS_WR_EBD_TIME: c_uint = 0x1010a;
pub const HNS3_PMU_EVT_PPS_RD_FBD_PACKET_NUM: c_uint = 0x0010b;
pub const HNS3_PMU_EVT_PPS_RD_FBD_TIME: c_uint = 0x1010b;
pub const HNS3_PMU_EVT_PPS_RD_EBD_PACKET_NUM: c_uint = 0x0010c;
pub const HNS3_PMU_EVT_PPS_RD_EBD_TIME: c_uint = 0x1010c;
pub const HNS3_PMU_EVT_PPS_RD_PAY_M0_PACKET_NUM: c_uint = 0x0010d;
pub const HNS3_PMU_EVT_PPS_RD_PAY_M0_TIME: c_uint = 0x1010d;
pub const HNS3_PMU_EVT_PPS_RD_PAY_M1_PACKET_NUM: c_uint = 0x0010e;
pub const HNS3_PMU_EVT_PPS_RD_PAY_M1_TIME: c_uint = 0x1010e;
pub const HNS3_PMU_EVT_PPS_WR_PAY_M0_PACKET_NUM: c_uint = 0x0010f;
pub const HNS3_PMU_EVT_PPS_WR_PAY_M0_TIME: c_uint = 0x1010f;
pub const HNS3_PMU_EVT_PPS_WR_PAY_M1_PACKET_NUM: c_uint = 0x00110;
pub const HNS3_PMU_EVT_PPS_WR_PAY_M1_TIME: c_uint = 0x10110;
pub const HNS3_PMU_EVT_PPS_NICROH_TX_PRE_PACKET_NUM: c_uint = 0x00111;
pub const HNS3_PMU_EVT_PPS_NICROH_TX_PRE_TIME: c_uint = 0x10111;
pub const HNS3_PMU_EVT_PPS_NICROH_RX_PRE_PACKET_NUM: c_uint = 0x00112;
pub const HNS3_PMU_EVT_PPS_NICROH_RX_PRE_TIME: c_uint = 0x10112;
// latency events
pub const HNS3_PMU_EVT_DLY_TX_PUSH_TIME: c_uint = 0x00202;
pub const HNS3_PMU_EVT_DLY_TX_PUSH_PACKET_NUM: c_uint = 0x10202;
pub const HNS3_PMU_EVT_DLY_TX_TIME: c_uint = 0x00204;
pub const HNS3_PMU_EVT_DLY_TX_PACKET_NUM: c_uint = 0x10204;
pub const HNS3_PMU_EVT_DLY_SSU_TX_NIC_TIME: c_uint = 0x00206;
pub const HNS3_PMU_EVT_DLY_SSU_TX_NIC_PACKET_NUM: c_uint = 0x10206;
pub const HNS3_PMU_EVT_DLY_SSU_TX_ROCE_TIME: c_uint = 0x00207;
pub const HNS3_PMU_EVT_DLY_SSU_TX_ROCE_PACKET_NUM: c_uint = 0x10207;
pub const HNS3_PMU_EVT_DLY_SSU_RX_NIC_TIME: c_uint = 0x00208;
pub const HNS3_PMU_EVT_DLY_SSU_RX_NIC_PACKET_NUM: c_uint = 0x10208;
pub const HNS3_PMU_EVT_DLY_SSU_RX_ROCE_TIME: c_uint = 0x00209;
pub const HNS3_PMU_EVT_DLY_SSU_RX_ROCE_PACKET_NUM: c_uint = 0x10209;
pub const HNS3_PMU_EVT_DLY_RPU_TIME: c_uint = 0x0020e;
pub const HNS3_PMU_EVT_DLY_RPU_PACKET_NUM: c_uint = 0x1020e;
pub const HNS3_PMU_EVT_DLY_TPU_TIME: c_uint = 0x0020f;
pub const HNS3_PMU_EVT_DLY_TPU_PACKET_NUM: c_uint = 0x1020f;
pub const HNS3_PMU_EVT_DLY_RPE_TIME: c_uint = 0x00210;
pub const HNS3_PMU_EVT_DLY_RPE_PACKET_NUM: c_uint = 0x10210;
pub const HNS3_PMU_EVT_DLY_TPE_TIME: c_uint = 0x00211;
pub const HNS3_PMU_EVT_DLY_TPE_PACKET_NUM: c_uint = 0x10211;
pub const HNS3_PMU_EVT_DLY_TPE_PUSH_TIME: c_uint = 0x00212;
pub const HNS3_PMU_EVT_DLY_TPE_PUSH_PACKET_NUM: c_uint = 0x10212;
pub const HNS3_PMU_EVT_DLY_WR_FBD_TIME: c_uint = 0x00213;
pub const HNS3_PMU_EVT_DLY_WR_FBD_PACKET_NUM: c_uint = 0x10213;
pub const HNS3_PMU_EVT_DLY_WR_EBD_TIME: c_uint = 0x00214;
pub const HNS3_PMU_EVT_DLY_WR_EBD_PACKET_NUM: c_uint = 0x10214;
pub const HNS3_PMU_EVT_DLY_RD_FBD_TIME: c_uint = 0x00215;
pub const HNS3_PMU_EVT_DLY_RD_FBD_PACKET_NUM: c_uint = 0x10215;
pub const HNS3_PMU_EVT_DLY_RD_EBD_TIME: c_uint = 0x00216;
pub const HNS3_PMU_EVT_DLY_RD_EBD_PACKET_NUM: c_uint = 0x10216;
pub const HNS3_PMU_EVT_DLY_RD_PAY_M0_TIME: c_uint = 0x00217;
pub const HNS3_PMU_EVT_DLY_RD_PAY_M0_PACKET_NUM: c_uint = 0x10217;
pub const HNS3_PMU_EVT_DLY_RD_PAY_M1_TIME: c_uint = 0x00218;
pub const HNS3_PMU_EVT_DLY_RD_PAY_M1_PACKET_NUM: c_uint = 0x10218;
pub const HNS3_PMU_EVT_DLY_WR_PAY_M0_TIME: c_uint = 0x00219;
pub const HNS3_PMU_EVT_DLY_WR_PAY_M0_PACKET_NUM: c_uint = 0x10219;
pub const HNS3_PMU_EVT_DLY_WR_PAY_M1_TIME: c_uint = 0x0021a;
pub const HNS3_PMU_EVT_DLY_WR_PAY_M1_PACKET_NUM: c_uint = 0x1021a;
pub const HNS3_PMU_EVT_DLY_MSIX_WRITE_TIME: c_uint = 0x0021c;
pub const HNS3_PMU_EVT_DLY_MSIX_WRITE_PACKET_NUM: c_uint = 0x1021c;
// interrupt rate events
pub const HNS3_PMU_EVT_PPS_MSIX_NIC_INTR_NUM: c_uint = 0x00300;
pub const HNS3_PMU_EVT_PPS_MSIX_NIC_TIME: c_uint = 0x10300;
// filter mode supported by each bandwidth event
pub const HNS3_PMU_FILTER_BW_SSU_EGU: c_uint = 0x07;
pub const HNS3_PMU_FILTER_BW_SSU_RPU: c_uint = 0x1f;
pub const HNS3_PMU_FILTER_BW_SSU_ROCE: c_uint = 0x0f;
pub const HNS3_PMU_FILTER_BW_ROCE_SSU: c_uint = 0x0f;
pub const HNS3_PMU_FILTER_BW_TPU_SSU: c_uint = 0x1f;
pub const HNS3_PMU_FILTER_BW_RPU_RCBRX: c_uint = 0x11;
pub const HNS3_PMU_FILTER_BW_RCBTX_TXSCH: c_uint = 0x11;
pub const HNS3_PMU_FILTER_BW_WR_FBD: c_uint = 0x1b;
pub const HNS3_PMU_FILTER_BW_WR_EBD: c_uint = 0x11;
pub const HNS3_PMU_FILTER_BW_RD_FBD: c_uint = 0x01;
pub const HNS3_PMU_FILTER_BW_RD_EBD: c_uint = 0x1b;
pub const HNS3_PMU_FILTER_BW_RD_PAY_M0: c_uint = 0x01;
pub const HNS3_PMU_FILTER_BW_RD_PAY_M1: c_uint = 0x01;
pub const HNS3_PMU_FILTER_BW_WR_PAY_M0: c_uint = 0x01;
pub const HNS3_PMU_FILTER_BW_WR_PAY_M1: c_uint = 0x01;
// filter mode supported by each packet rate event
pub const HNS3_PMU_FILTER_PPS_IGU_SSU: c_uint = 0x07;
pub const HNS3_PMU_FILTER_PPS_SSU_EGU: c_uint = 0x07;
pub const HNS3_PMU_FILTER_PPS_SSU_RPU: c_uint = 0x1f;
pub const HNS3_PMU_FILTER_PPS_SSU_ROCE: c_uint = 0x0f;
pub const HNS3_PMU_FILTER_PPS_ROCE_SSU: c_uint = 0x0f;
pub const HNS3_PMU_FILTER_PPS_TPU_SSU: c_uint = 0x1f;
pub const HNS3_PMU_FILTER_PPS_RPU_RCBRX: c_uint = 0x11;
pub const HNS3_PMU_FILTER_PPS_RCBTX_TPU: c_uint = 0x1f;
pub const HNS3_PMU_FILTER_PPS_RCBTX_TXSCH: c_uint = 0x11;
pub const HNS3_PMU_FILTER_PPS_WR_FBD: c_uint = 0x1b;
pub const HNS3_PMU_FILTER_PPS_WR_EBD: c_uint = 0x11;
pub const HNS3_PMU_FILTER_PPS_RD_FBD: c_uint = 0x01;
pub const HNS3_PMU_FILTER_PPS_RD_EBD: c_uint = 0x1b;
pub const HNS3_PMU_FILTER_PPS_RD_PAY_M0: c_uint = 0x01;
pub const HNS3_PMU_FILTER_PPS_RD_PAY_M1: c_uint = 0x01;
pub const HNS3_PMU_FILTER_PPS_WR_PAY_M0: c_uint = 0x01;
pub const HNS3_PMU_FILTER_PPS_WR_PAY_M1: c_uint = 0x01;
pub const HNS3_PMU_FILTER_PPS_NICROH_TX_PRE: c_uint = 0x01;
pub const HNS3_PMU_FILTER_PPS_NICROH_RX_PRE: c_uint = 0x01;
// filter mode supported by each latency event
pub const HNS3_PMU_FILTER_DLY_TX_PUSH: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_TX: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_SSU_TX_NIC: c_uint = 0x07;
pub const HNS3_PMU_FILTER_DLY_SSU_TX_ROCE: c_uint = 0x07;
pub const HNS3_PMU_FILTER_DLY_SSU_RX_NIC: c_uint = 0x07;
pub const HNS3_PMU_FILTER_DLY_SSU_RX_ROCE: c_uint = 0x07;
pub const HNS3_PMU_FILTER_DLY_RPU: c_uint = 0x11;
pub const HNS3_PMU_FILTER_DLY_TPU: c_uint = 0x1f;
pub const HNS3_PMU_FILTER_DLY_RPE: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_TPE: c_uint = 0x0b;
pub const HNS3_PMU_FILTER_DLY_TPE_PUSH: c_uint = 0x1b;
pub const HNS3_PMU_FILTER_DLY_WR_FBD: c_uint = 0x1b;
pub const HNS3_PMU_FILTER_DLY_WR_EBD: c_uint = 0x11;
pub const HNS3_PMU_FILTER_DLY_RD_FBD: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_RD_EBD: c_uint = 0x1b;
pub const HNS3_PMU_FILTER_DLY_RD_PAY_M0: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_RD_PAY_M1: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_WR_PAY_M0: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_WR_PAY_M1: c_uint = 0x01;
pub const HNS3_PMU_FILTER_DLY_MSIX_WRITE: c_uint = 0x01;
// filter mode supported by each interrupt rate event
pub const HNS3_PMU_FILTER_INTR_MSIX_NIC: c_uint = 0x01;
    enum hns3_pmu_hw_filter_mode {
    HNS3_PMU_HW_FILTER_GLOBAL,
    HNS3_PMU_HW_FILTER_PORT,
    HNS3_PMU_HW_FILTER_PORT_TC,
    HNS3_PMU_HW_FILTER_FUNC,
    HNS3_PMU_HW_FILTER_FUNC_QUEUE,
    HNS3_PMU_HW_FILTER_FUNC_INTR,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_pmu_event_attr {
    pub event: u32,
    pub filter_support: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hns3_pmu {
    pub hw_events: [*mut perf_event; HNS3_PMU_MAX_HW_EVENTS],
    pub node: hlist_node,
    pub pdev: *mut pci_dev,
    pub pmu: pmu,
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub on_cpu: c_int,
    pub identifier: u32,
    pub /: *mut *mut u32 hw_clk_freq; / hardware clock frequency of PMU,
// maximum and minimum bdf allowed by PMU
    pub bdf_min: u16,
    pub bdf_max: u16,
}

    static inline u64 hns3_pmu_get_##_name(struct perf_event *event) \
    {                                                                \
    return FIELD_GET(GENMASK_ULL(_end, _start),              \
    event.attr._config);                   \
    }
    HNS3_PMU_FILTER_ATTR(subevent, config, 0, 7);
    HNS3_PMU_FILTER_ATTR(event_type, config, 8, 15);
    HNS3_PMU_FILTER_ATTR(ext_counter_used, config, 16, 16);
    HNS3_PMU_FILTER_ATTR(port, config1, 0, 3);
    HNS3_PMU_FILTER_ATTR(tc, config1, 4, 7);
    HNS3_PMU_FILTER_ATTR(bdf, config1, 8, 23);
    HNS3_PMU_FILTER_ATTR(queue, config1, 24, 39);
    HNS3_PMU_FILTER_ATTR(intr, config1, 40, 51);
    HNS3_PMU_FILTER_ATTR(global, config1, 52, 52);

    HNS3_PMU_EVT_BW_##_name##_BYTE_NUM,				\
    HNS3_PMU_FILTER_BW_##_name})

    HNS3_PMU_EVT_BW_##_name##_TIME,					\
    HNS3_PMU_FILTER_BW_##_name})

    HNS3_PMU_EVT_PPS_##_name##_PACKET_NUM,				\
    HNS3_PMU_FILTER_PPS_##_name})

    HNS3_PMU_EVT_PPS_##_name##_TIME,				\
    HNS3_PMU_FILTER_PPS_##_name})

    HNS3_PMU_EVT_DLY_##_name##_TIME,				\
    HNS3_PMU_FILTER_DLY_##_name})

    HNS3_PMU_EVT_DLY_##_name##_PACKET_NUM,				\
    HNS3_PMU_FILTER_DLY_##_name})

    HNS3_PMU_EVT_PPS_##_name##_INTR_NUM,				\
    HNS3_PMU_FILTER_INTR_##_name})

    HNS3_PMU_EVT_PPS_##_name##_TIME,				\
    HNS3_PMU_FILTER_INTR_##_name})
    static ssize_t hns3_pmu_event_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct hns3_pmu_event_attr *event;
    struct dev_ext_attribute *eattr;
    eattr = container_of(attr, struct dev_ext_attribute, attr);
    event = eattr.var;
    return sysfs_emit(buf, "config=0x%x\n", event.event);
    }
    static ssize_t hns3_pmu_filter_mode_show(struct device *dev,
    struct device_attribute *attr,
    char *buf)
    {
    struct hns3_pmu_event_attr *event;
    struct dev_ext_attribute *eattr;
    int len;
    eattr = container_of(attr, struct dev_ext_attribute, attr);
    event = eattr.var;
    len = sysfs_emit_at(buf, 0, "filter mode supported: ");
    if (event.filter_support & HNS3_PMU_FILTER_SUPPORT_GLOBAL)
    len += sysfs_emit_at(buf, len, "global ");
    if (event.filter_support & HNS3_PMU_FILTER_SUPPORT_PORT)
    len += sysfs_emit_at(buf, len, "port ");
    if (event.filter_support & HNS3_PMU_FILTER_SUPPORT_PORT_TC)
    len += sysfs_emit_at(buf, len, "port-tc ");
    if (event.filter_support & HNS3_PMU_FILTER_SUPPORT_FUNC)
    len += sysfs_emit_at(buf, len, "func ");
    if (event.filter_support & HNS3_PMU_FILTER_SUPPORT_FUNC_QUEUE)
    len += sysfs_emit_at(buf, len, "func-queue ");
    if (event.filter_support & HNS3_PMU_FILTER_SUPPORT_FUNC_INTR)
    len += sysfs_emit_at(buf, len, "func-intr ");
    len += sysfs_emit_at(buf, len, "\n");
    return len;
    }

    (&((struct dev_ext_attribute[]) {				\
    { __ATTR(_name, 0444, _func, core::ptr::null_mut()), (void *)_config }	\
    })[0].attr.attr)

    HNS3_PMU_ATTR(_name, device_show_string, _format)

    HNS3_PMU_ATTR(_name, hns3_pmu_event_show, (void *)_event)

    HNS3_PMU_ATTR(_name, hns3_pmu_filter_mode_show, (void *)_event)

    HNS3_PMU_EVENT_ATTR(_name##_byte_num, HNS3_BW_EVT_BYTE_NUM(_macro)), \
    HNS3_PMU_EVENT_ATTR(_name##_time, HNS3_BW_EVT_TIME(_macro))

    HNS3_PMU_EVENT_ATTR(_name##_packet_num, HNS3_PPS_EVT_PACKET_NUM(_macro)), \
    HNS3_PMU_EVENT_ATTR(_name##_time, HNS3_PPS_EVT_TIME(_macro))

    HNS3_PMU_EVENT_ATTR(_name##_time, HNS3_DLY_EVT_TIME(_macro)), \
    HNS3_PMU_EVENT_ATTR(_name##_packet_num, HNS3_DLY_EVT_PACKET_NUM(_macro))

    HNS3_PMU_EVENT_ATTR(_name##_intr_num, HNS3_INTR_EVT_INTR_NUM(_macro)), \
    HNS3_PMU_EVENT_ATTR(_name##_time, HNS3_INTR_EVT_TIME(_macro))

    HNS3_PMU_FLT_MODE_ATTR(_name##_byte_num, HNS3_BW_EVT_BYTE_NUM(_macro)), \
    HNS3_PMU_FLT_MODE_ATTR(_name##_time, HNS3_BW_EVT_TIME(_macro))

    HNS3_PMU_FLT_MODE_ATTR(_name##_packet_num, HNS3_PPS_EVT_PACKET_NUM(_macro)), \
    HNS3_PMU_FLT_MODE_ATTR(_name##_time, HNS3_PPS_EVT_TIME(_macro))

    HNS3_PMU_FLT_MODE_ATTR(_name##_time, HNS3_DLY_EVT_TIME(_macro)), \
    HNS3_PMU_FLT_MODE_ATTR(_name##_packet_num, HNS3_DLY_EVT_PACKET_NUM(_macro))

    HNS3_PMU_FLT_MODE_ATTR(_name##_intr_num, HNS3_INTR_EVT_INTR_NUM(_macro)), \
    HNS3_PMU_FLT_MODE_ATTR(_name##_time, HNS3_INTR_EVT_TIME(_macro))
    static u8 hns3_pmu_hw_filter_modes[] = {
    HNS3_PMU_HW_FILTER_GLOBAL,
    HNS3_PMU_HW_FILTER_PORT,
    HNS3_PMU_HW_FILTER_PORT_TC,
    HNS3_PMU_HW_FILTER_FUNC,
    HNS3_PMU_HW_FILTER_FUNC_QUEUE,
    HNS3_PMU_HW_FILTER_FUNC_INTR,
    };

    ((_hwc).addr_filters = (void *)&hns3_pmu_hw_filter_modes[(_mode)])
    static ssize_t identifier_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(dev_get_drvdata(dev));
    return sysfs_emit(buf, "0x%x\n", hns3_pmu.identifier);
    }
    static DEVICE_ATTR_RO(identifier);
    static ssize_t cpumask_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(dev_get_drvdata(dev));
    return sysfs_emit(buf, "%d\n", hns3_pmu.on_cpu);
    }
    static DEVICE_ATTR_RO(cpumask);
    static ssize_t bdf_min_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(dev_get_drvdata(dev));
    let mut bdf: u16 = hns3_pmu.bdf_min;
    return sysfs_emit(buf, "%02x:%02x.%x\n", PCI_BUS_NUM(bdf),
    PCI_SLOT(bdf), PCI_FUNC(bdf));
    }
    static DEVICE_ATTR_RO(bdf_min);
    static ssize_t bdf_max_show(struct device *dev, struct device_attribute *attr,
    char *buf)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(dev_get_drvdata(dev));
    let mut bdf: u16 = hns3_pmu.bdf_max;
    return sysfs_emit(buf, "%02x:%02x.%x\n", PCI_BUS_NUM(bdf),
    PCI_SLOT(bdf), PCI_FUNC(bdf));
    }
    static DEVICE_ATTR_RO(bdf_max);
    static ssize_t hw_clk_freq_show(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(dev_get_drvdata(dev));
    return sysfs_emit(buf, "%u\n", hns3_pmu.hw_clk_freq);
    }
    static DEVICE_ATTR_RO(hw_clk_freq);
    static struct attribute *hns3_pmu_events_attr[] = {
// bandwidth events
    HNS3_PMU_BW_EVT_PAIR(bw_ssu_egu, SSU_EGU),
    HNS3_PMU_BW_EVT_PAIR(bw_ssu_rpu, SSU_RPU),
    HNS3_PMU_BW_EVT_PAIR(bw_ssu_roce, SSU_ROCE),
    HNS3_PMU_BW_EVT_PAIR(bw_roce_ssu, ROCE_SSU),
    HNS3_PMU_BW_EVT_PAIR(bw_tpu_ssu, TPU_SSU),
    HNS3_PMU_BW_EVT_PAIR(bw_rpu_rcbrx, RPU_RCBRX),
    HNS3_PMU_BW_EVT_PAIR(bw_rcbtx_txsch, RCBTX_TXSCH),
    HNS3_PMU_BW_EVT_PAIR(bw_wr_fbd, WR_FBD),
    HNS3_PMU_BW_EVT_PAIR(bw_wr_ebd, WR_EBD),
    HNS3_PMU_BW_EVT_PAIR(bw_rd_fbd, RD_FBD),
    HNS3_PMU_BW_EVT_PAIR(bw_rd_ebd, RD_EBD),
    HNS3_PMU_BW_EVT_PAIR(bw_rd_pay_m0, RD_PAY_M0),
    HNS3_PMU_BW_EVT_PAIR(bw_rd_pay_m1, RD_PAY_M1),
    HNS3_PMU_BW_EVT_PAIR(bw_wr_pay_m0, WR_PAY_M0),
    HNS3_PMU_BW_EVT_PAIR(bw_wr_pay_m1, WR_PAY_M1),
// packet rate events
    HNS3_PMU_PPS_EVT_PAIR(pps_igu_ssu, IGU_SSU),
    HNS3_PMU_PPS_EVT_PAIR(pps_ssu_egu, SSU_EGU),
    HNS3_PMU_PPS_EVT_PAIR(pps_ssu_rpu, SSU_RPU),
    HNS3_PMU_PPS_EVT_PAIR(pps_ssu_roce, SSU_ROCE),
    HNS3_PMU_PPS_EVT_PAIR(pps_roce_ssu, ROCE_SSU),
    HNS3_PMU_PPS_EVT_PAIR(pps_tpu_ssu, TPU_SSU),
    HNS3_PMU_PPS_EVT_PAIR(pps_rpu_rcbrx, RPU_RCBRX),
    HNS3_PMU_PPS_EVT_PAIR(pps_rcbtx_tpu, RCBTX_TPU),
    HNS3_PMU_PPS_EVT_PAIR(pps_rcbtx_txsch, RCBTX_TXSCH),
    HNS3_PMU_PPS_EVT_PAIR(pps_wr_fbd, WR_FBD),
    HNS3_PMU_PPS_EVT_PAIR(pps_wr_ebd, WR_EBD),
    HNS3_PMU_PPS_EVT_PAIR(pps_rd_fbd, RD_FBD),
    HNS3_PMU_PPS_EVT_PAIR(pps_rd_ebd, RD_EBD),
    HNS3_PMU_PPS_EVT_PAIR(pps_rd_pay_m0, RD_PAY_M0),
    HNS3_PMU_PPS_EVT_PAIR(pps_rd_pay_m1, RD_PAY_M1),
    HNS3_PMU_PPS_EVT_PAIR(pps_wr_pay_m0, WR_PAY_M0),
    HNS3_PMU_PPS_EVT_PAIR(pps_wr_pay_m1, WR_PAY_M1),
    HNS3_PMU_PPS_EVT_PAIR(pps_intr_nicroh_tx_pre, NICROH_TX_PRE),
    HNS3_PMU_PPS_EVT_PAIR(pps_intr_nicroh_rx_pre, NICROH_RX_PRE),
// latency events
    HNS3_PMU_DLY_EVT_PAIR(dly_tx_push_to_mac, TX_PUSH),
    HNS3_PMU_DLY_EVT_PAIR(dly_tx_normal_to_mac, TX),
    HNS3_PMU_DLY_EVT_PAIR(dly_ssu_tx_th_nic, SSU_TX_NIC),
    HNS3_PMU_DLY_EVT_PAIR(dly_ssu_tx_th_roce, SSU_TX_ROCE),
    HNS3_PMU_DLY_EVT_PAIR(dly_ssu_rx_th_nic, SSU_RX_NIC),
    HNS3_PMU_DLY_EVT_PAIR(dly_ssu_rx_th_roce, SSU_RX_ROCE),
    HNS3_PMU_DLY_EVT_PAIR(dly_rpu, RPU),
    HNS3_PMU_DLY_EVT_PAIR(dly_tpu, TPU),
    HNS3_PMU_DLY_EVT_PAIR(dly_rpe, RPE),
    HNS3_PMU_DLY_EVT_PAIR(dly_tpe_normal, TPE),
    HNS3_PMU_DLY_EVT_PAIR(dly_tpe_push, TPE_PUSH),
    HNS3_PMU_DLY_EVT_PAIR(dly_wr_fbd, WR_FBD),
    HNS3_PMU_DLY_EVT_PAIR(dly_wr_ebd, WR_EBD),
    HNS3_PMU_DLY_EVT_PAIR(dly_rd_fbd, RD_FBD),
    HNS3_PMU_DLY_EVT_PAIR(dly_rd_ebd, RD_EBD),
    HNS3_PMU_DLY_EVT_PAIR(dly_rd_pay_m0, RD_PAY_M0),
    HNS3_PMU_DLY_EVT_PAIR(dly_rd_pay_m1, RD_PAY_M1),
    HNS3_PMU_DLY_EVT_PAIR(dly_wr_pay_m0, WR_PAY_M0),
    HNS3_PMU_DLY_EVT_PAIR(dly_wr_pay_m1, WR_PAY_M1),
    HNS3_PMU_DLY_EVT_PAIR(dly_msix_write, MSIX_WRITE),
// interrupt rate events
    HNS3_PMU_INTR_EVT_PAIR(pps_intr_msix_nic, MSIX_NIC),
    core::ptr::null_mut()
    };
    static struct attribute *hns3_pmu_filter_mode_attr[] = {
// bandwidth events
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_ssu_egu, SSU_EGU),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_ssu_rpu, SSU_RPU),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_ssu_roce, SSU_ROCE),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_roce_ssu, ROCE_SSU),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_tpu_ssu, TPU_SSU),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_rpu_rcbrx, RPU_RCBRX),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_rcbtx_txsch, RCBTX_TXSCH),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_wr_fbd, WR_FBD),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_wr_ebd, WR_EBD),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_rd_fbd, RD_FBD),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_rd_ebd, RD_EBD),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_rd_pay_m0, RD_PAY_M0),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_rd_pay_m1, RD_PAY_M1),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_wr_pay_m0, WR_PAY_M0),
    HNS3_PMU_BW_FLT_MODE_PAIR(bw_wr_pay_m1, WR_PAY_M1),
// packet rate events
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_igu_ssu, IGU_SSU),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_ssu_egu, SSU_EGU),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_ssu_rpu, SSU_RPU),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_ssu_roce, SSU_ROCE),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_roce_ssu, ROCE_SSU),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_tpu_ssu, TPU_SSU),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_rpu_rcbrx, RPU_RCBRX),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_rcbtx_tpu, RCBTX_TPU),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_rcbtx_txsch, RCBTX_TXSCH),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_wr_fbd, WR_FBD),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_wr_ebd, WR_EBD),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_rd_fbd, RD_FBD),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_rd_ebd, RD_EBD),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_rd_pay_m0, RD_PAY_M0),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_rd_pay_m1, RD_PAY_M1),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_wr_pay_m0, WR_PAY_M0),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_wr_pay_m1, WR_PAY_M1),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_intr_nicroh_tx_pre, NICROH_TX_PRE),
    HNS3_PMU_PPS_FLT_MODE_PAIR(pps_intr_nicroh_rx_pre, NICROH_RX_PRE),
// latency events
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_tx_push_to_mac, TX_PUSH),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_tx_normal_to_mac, TX),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_ssu_tx_th_nic, SSU_TX_NIC),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_ssu_tx_th_roce, SSU_TX_ROCE),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_ssu_rx_th_nic, SSU_RX_NIC),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_ssu_rx_th_roce, SSU_RX_ROCE),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_rpu, RPU),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_tpu, TPU),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_rpe, RPE),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_tpe_normal, TPE),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_tpe_push, TPE_PUSH),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_wr_fbd, WR_FBD),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_wr_ebd, WR_EBD),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_rd_fbd, RD_FBD),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_rd_ebd, RD_EBD),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_rd_pay_m0, RD_PAY_M0),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_rd_pay_m1, RD_PAY_M1),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_wr_pay_m0, WR_PAY_M0),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_wr_pay_m1, WR_PAY_M1),
    HNS3_PMU_DLY_FLT_MODE_PAIR(dly_msix_write, MSIX_WRITE),
// interrupt rate events
    HNS3_PMU_INTR_FLT_MODE_PAIR(pps_intr_msix_nic, MSIX_NIC),
    core::ptr::null_mut()
    };
    static struct attribute_group hns3_pmu_events_group = {
    .name = "events",
    .attrs = hns3_pmu_events_attr,
    };
    static struct attribute_group hns3_pmu_filter_mode_group = {
    .name = "filtermode",
    .attrs = hns3_pmu_filter_mode_attr,
    };
    static struct attribute *hns3_pmu_format_attr[] = {
    HNS3_PMU_FORMAT_ATTR(subevent, "config:0-7"),
    HNS3_PMU_FORMAT_ATTR(event_type, "config:8-15"),
    HNS3_PMU_FORMAT_ATTR(ext_counter_used, "config:16"),
    HNS3_PMU_FORMAT_ATTR(port, "config1:0-3"),
    HNS3_PMU_FORMAT_ATTR(tc, "config1:4-7"),
    HNS3_PMU_FORMAT_ATTR(bdf, "config1:8-23"),
    HNS3_PMU_FORMAT_ATTR(queue, "config1:24-39"),
    HNS3_PMU_FORMAT_ATTR(intr, "config1:40-51"),
    HNS3_PMU_FORMAT_ATTR(global, "config1:52"),
    core::ptr::null_mut()
    };
    static struct attribute_group hns3_pmu_format_group = {
    .name = "format",
    .attrs = hns3_pmu_format_attr,
    };
    static struct attribute *hns3_pmu_cpumask_attrs[] = {
    &dev_attr_cpumask.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group hns3_pmu_cpumask_attr_group = {
    .attrs = hns3_pmu_cpumask_attrs,
    };
    static struct attribute *hns3_pmu_identifier_attrs[] = {
    &dev_attr_identifier.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group hns3_pmu_identifier_attr_group = {
    .attrs = hns3_pmu_identifier_attrs,
    };
    static struct attribute *hns3_pmu_bdf_range_attrs[] = {
    &dev_attr_bdf_min.attr,
    &dev_attr_bdf_max.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group hns3_pmu_bdf_range_attr_group = {
    .attrs = hns3_pmu_bdf_range_attrs,
    };
    static struct attribute *hns3_pmu_hw_clk_freq_attrs[] = {
    &dev_attr_hw_clk_freq.attr,
    core::ptr::null_mut()
    };
    static struct attribute_group hns3_pmu_hw_clk_freq_attr_group = {
    .attrs = hns3_pmu_hw_clk_freq_attrs,
    };
    static const struct attribute_group *hns3_pmu_attr_groups[] = {
    &hns3_pmu_events_group,
    &hns3_pmu_filter_mode_group,
    &hns3_pmu_format_group,
    &hns3_pmu_cpumask_attr_group,
    &hns3_pmu_identifier_attr_group,
    &hns3_pmu_bdf_range_attr_group,
    &hns3_pmu_hw_clk_freq_attr_group,
    core::ptr::null_mut()
    };
#[no_mangle]
unsafe extern "C" fn hns3_pmu_get_event(event: *mut perf_event) -> u32 {
    static u32 hns3_pmu_get_event(struct perf_event *event)
    {
    return hns3_pmu_get_ext_counter_used(event) << 16 |
    hns3_pmu_get_event_type(event) << 8 |
    hns3_pmu_get_subevent(event);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_get_real_event(event: *mut perf_event) -> u32 {
    static u32 hns3_pmu_get_real_event(struct perf_event *event)
    {
    return hns3_pmu_get_event_type(event) << 8 |
    hns3_pmu_get_subevent(event);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_get_offset(offset: u32, idx: u32) -> u32 {
    static u32 hns3_pmu_get_offset(u32 offset, u32 idx)
    {
    return offset + HNS3_PMU_REG_EVENT_OFFSET +
    HNS3_PMU_REG_EVENT_SIZE * idx;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_readl(hns3_pmu: *mut hns3_pmu, reg_offset: u32, idx: u32) -> u32 {
    static u32 hns3_pmu_readl(struct hns3_pmu *hns3_pmu, u32 reg_offset, u32 idx)
    {
    let mut offset: u32 = hns3_pmu_get_offset(reg_offset, idx);
    return readl(hns3_pmu.base + offset);
    }
    static void hns3_pmu_writel(struct hns3_pmu *hns3_pmu, u32 reg_offset, u32 idx,
    u32 val)
    {
    let mut offset: u32 = hns3_pmu_get_offset(reg_offset, idx);
    writel(val, hns3_pmu.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_readq(hns3_pmu: *mut hns3_pmu, reg_offset: u32, idx: u32) -> u64 {
    static u64 hns3_pmu_readq(struct hns3_pmu *hns3_pmu, u32 reg_offset, u32 idx)
    {
    let mut offset: u32 = hns3_pmu_get_offset(reg_offset, idx);
    return readq(hns3_pmu.base + offset);
    }
    static void hns3_pmu_writeq(struct hns3_pmu *hns3_pmu, u32 reg_offset, u32 idx,
    u64 val)
    {
    let mut offset: u32 = hns3_pmu_get_offset(reg_offset, idx);
    writeq(val, hns3_pmu.base + offset);
    }
    static bool hns3_pmu_cmp_event(struct perf_event *target,
    struct perf_event *event)
    {
    return hns3_pmu_get_real_event(target) == hns3_pmu_get_real_event(event);
    }
    static int hns3_pmu_find_related_event_idx(struct hns3_pmu *hns3_pmu,
    struct perf_event *event)
    {
    struct perf_event *sibling;
    let mut hw_event_used: c_int = 0;
    int idx;
    for (idx = 0; idx < HNS3_PMU_MAX_HW_EVENTS; idx++) {
    sibling = hns3_pmu.hw_events[idx];
    if (!sibling)
    continue;
    hw_event_used++;
    if (!hns3_pmu_cmp_event(sibling, event))
    continue;
// Related events is used in group
    if (sibling.group_leader == event.group_leader)
    return idx;
    }
// No related event and all hardware events are used up
    if (hw_event_used >= HNS3_PMU_MAX_HW_EVENTS)
    return -EBUSY;
// No related event and there is extra hardware events can be use
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_get_event_idx(hns3_pmu: *mut hns3_pmu) -> c_int {
    static int hns3_pmu_get_event_idx(struct hns3_pmu *hns3_pmu)
    {
    int idx;
    for (idx = 0; idx < HNS3_PMU_MAX_HW_EVENTS; idx++) {
    if (!hns3_pmu.hw_events[idx])
    return idx;
    }
    return -EBUSY;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_valid_bdf(hns3_pmu: *mut hns3_pmu, bdf: u16) -> bool {
    static bool hns3_pmu_valid_bdf(struct hns3_pmu *hns3_pmu, u16 bdf)
    {
    struct pci_dev *pdev;
    if (bdf < hns3_pmu.bdf_min || bdf > hns3_pmu.bdf_max) {
    pci_err(hns3_pmu.pdev, "Invalid EP device: %#x!\n", bdf);
    return false;
    }
    pdev = pci_get_domain_bus_and_slot(pci_domain_nr(hns3_pmu.pdev.bus),
    PCI_BUS_NUM(bdf),
    GET_PCI_DEVFN(bdf));
    if (!pdev) {
    pci_err(hns3_pmu.pdev, "Nonexistent EP device: %#x!\n", bdf);
    return false;
    }
    pci_dev_put(pdev);
    return true;
    }
    static void hns3_pmu_set_qid_para(struct hns3_pmu *hns3_pmu, u32 idx, u16 bdf,
    u16 queue)
    {
    u32 val;
    val = GET_PCI_DEVFN(bdf);
    val |= (u32)queue << HNS3_PMU_QID_PARA_QUEUE_S;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_QID_PARA, idx, val);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_qid_req_start(hns3_pmu: *mut hns3_pmu, idx: u32) -> bool {
    static bool hns3_pmu_qid_req_start(struct hns3_pmu *hns3_pmu, u32 idx)
    {
    let mut queue_id_valid: bool = false;
    u32 reg_qid_ctrl, val;
    int err;
// enable queue id request
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_QID_CTRL, idx,
    HNS3_PMU_QID_CTRL_REQ_ENABLE);
    reg_qid_ctrl = hns3_pmu_get_offset(HNS3_PMU_REG_EVENT_QID_CTRL, idx);
    err = readl_poll_timeout(hns3_pmu.base + reg_qid_ctrl, val,
    val & HNS3_PMU_QID_CTRL_DONE, 1, 1000);
    if (err == -ETIMEDOUT) {
    pci_err(hns3_pmu.pdev, "QID request timeout!\n");
    goto out;
    }
    queue_id_valid = !(val & HNS3_PMU_QID_CTRL_MISS);
    out:
// disable qid request and clear status
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_QID_CTRL, idx, 0);
    return queue_id_valid;
    }
    static bool hns3_pmu_valid_queue(struct hns3_pmu *hns3_pmu, u32 idx, u16 bdf,
    u16 queue)
    {
    hns3_pmu_set_qid_para(hns3_pmu, idx, bdf, queue);
    return hns3_pmu_qid_req_start(hns3_pmu, idx);
    }
    static struct hns3_pmu_event_attr *hns3_pmu_get_pmu_event(u32 event)
    {
    struct hns3_pmu_event_attr *pmu_event;
    struct dev_ext_attribute *eattr;
    struct device_attribute *dattr;
    struct attribute *attr;
    u32 i;
    for (i = 0; i < ARRAY_SIZE(hns3_pmu_events_attr) - 1; i++) {
    attr = hns3_pmu_events_attr[i];
    dattr = container_of(attr, struct device_attribute, attr);
    eattr = container_of(dattr, struct dev_ext_attribute, attr);
    pmu_event = eattr.var;
    if (event == pmu_event.event)
    return pmu_event;
    }
    return core::ptr::null_mut();
    }
    static int hns3_pmu_set_func_mode(struct perf_event *event,
    struct hns3_pmu *hns3_pmu)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut bdf: u16 = hns3_pmu_get_bdf(event);
    if (!hns3_pmu_valid_bdf(hns3_pmu, bdf))
    return -ENOENT;
    HNS3_PMU_SET_HW_FILTER(hwc, HNS3_PMU_HW_FILTER_FUNC);
    return 0;
    }
    static int hns3_pmu_set_func_queue_mode(struct perf_event *event,
    struct hns3_pmu *hns3_pmu)
    {
    let mut queue_id: u16 = hns3_pmu_get_queue(event);
    struct hw_perf_event *hwc = &event.hw;
    let mut bdf: u16 = hns3_pmu_get_bdf(event);
    if (!hns3_pmu_valid_bdf(hns3_pmu, bdf))
    return -ENOENT;
    if (!hns3_pmu_valid_queue(hns3_pmu, hwc.idx, bdf, queue_id)) {
    pci_err(hns3_pmu.pdev, "Invalid queue: %u\n", queue_id);
    return -ENOENT;
    }
    HNS3_PMU_SET_HW_FILTER(hwc, HNS3_PMU_HW_FILTER_FUNC_QUEUE);
    return 0;
    }
    static bool
    hns3_pmu_is_enabled_global_mode(struct perf_event *event,
    struct hns3_pmu_event_attr *pmu_event)
    {
    let mut global: u8 = hns3_pmu_get_global(event);
    if (!(pmu_event.filter_support & HNS3_PMU_FILTER_SUPPORT_GLOBAL))
    return false;
    return global;
    }
    static bool hns3_pmu_is_enabled_func_mode(struct perf_event *event,
    struct hns3_pmu_event_attr *pmu_event)
    {
    let mut queue_id: u16 = hns3_pmu_get_queue(event);
    let mut bdf: u16 = hns3_pmu_get_bdf(event);
    if (!(pmu_event.filter_support & HNS3_PMU_FILTER_SUPPORT_FUNC))
    return false;
#[no_mangle]
pub unsafe extern "C" fn if(HNS3_PMU_FILTER_ALL_QUEUE: queue_id !=) -> else {
    else if (queue_id != HNS3_PMU_FILTER_ALL_QUEUE)
    return false;
    return bdf;
    }
    static bool
    hns3_pmu_is_enabled_func_queue_mode(struct perf_event *event,
    struct hns3_pmu_event_attr *pmu_event)
    {
    let mut queue_id: u16 = hns3_pmu_get_queue(event);
    let mut bdf: u16 = hns3_pmu_get_bdf(event);
    if (!(pmu_event.filter_support & HNS3_PMU_FILTER_SUPPORT_FUNC_QUEUE))
    return false;
#[no_mangle]
pub unsafe extern "C" fn if(HNS3_PMU_FILTER_ALL_QUEUE: queue_id ==) -> else {
    else if (queue_id == HNS3_PMU_FILTER_ALL_QUEUE)
    return false;
    return bdf;
    }
    static bool hns3_pmu_is_enabled_port_mode(struct perf_event *event,
    struct hns3_pmu_event_attr *pmu_event)
    {
    let mut tc_id: u8 = hns3_pmu_get_tc(event);
    if (!(pmu_event.filter_support & HNS3_PMU_FILTER_SUPPORT_PORT))
    return false;
    let mut tc_id: return = = HNS3_PMU_FILTER_ALL_TC;
    }
    static bool
    hns3_pmu_is_enabled_port_tc_mode(struct perf_event *event,
    struct hns3_pmu_event_attr *pmu_event)
    {
    let mut tc_id: u8 = hns3_pmu_get_tc(event);
    if (!(pmu_event.filter_support & HNS3_PMU_FILTER_SUPPORT_PORT_TC))
    return false;
    return tc_id != HNS3_PMU_FILTER_ALL_TC;
    }
    static bool
    hns3_pmu_is_enabled_func_intr_mode(struct perf_event *event,
    struct hns3_pmu *hns3_pmu,
    struct hns3_pmu_event_attr *pmu_event)
    {
    let mut bdf: u16 = hns3_pmu_get_bdf(event);
    if (!(pmu_event.filter_support & HNS3_PMU_FILTER_SUPPORT_FUNC_INTR))
    return false;
    return hns3_pmu_valid_bdf(hns3_pmu, bdf);
    }
    static int hns3_pmu_select_filter_mode(struct perf_event *event,
    struct hns3_pmu *hns3_pmu)
    {
    let mut event_id: u32 = hns3_pmu_get_event(event);
    struct hw_perf_event *hwc = &event.hw;
    struct hns3_pmu_event_attr *pmu_event;
    pmu_event = hns3_pmu_get_pmu_event(event_id);
    if (!pmu_event) {
    pci_err(hns3_pmu.pdev, "Invalid pmu event\n");
    return -ENOENT;
    }
    if (hns3_pmu_is_enabled_global_mode(event, pmu_event)) {
    HNS3_PMU_SET_HW_FILTER(hwc, HNS3_PMU_HW_FILTER_GLOBAL);
    return 0;
    }
    if (hns3_pmu_is_enabled_func_mode(event, pmu_event))
    return hns3_pmu_set_func_mode(event, hns3_pmu);
    if (hns3_pmu_is_enabled_func_queue_mode(event, pmu_event))
    return hns3_pmu_set_func_queue_mode(event, hns3_pmu);
    if (hns3_pmu_is_enabled_port_mode(event, pmu_event)) {
    HNS3_PMU_SET_HW_FILTER(hwc, HNS3_PMU_HW_FILTER_PORT);
    return 0;
    }
    if (hns3_pmu_is_enabled_port_tc_mode(event, pmu_event)) {
    HNS3_PMU_SET_HW_FILTER(hwc, HNS3_PMU_HW_FILTER_PORT_TC);
    return 0;
    }
    if (hns3_pmu_is_enabled_func_intr_mode(event, hns3_pmu, pmu_event)) {
    HNS3_PMU_SET_HW_FILTER(hwc, HNS3_PMU_HW_FILTER_FUNC_INTR);
    return 0;
    }
    return -ENOENT;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_validate_event_group(event: *mut perf_event) -> bool {
    static bool hns3_pmu_validate_event_group(struct perf_event *event)
    {
    struct perf_event *sibling, *leader = event.group_leader;
    struct perf_event *event_group[HNS3_PMU_MAX_HW_EVENTS];
    let mut counters: c_int = 1;
    int num;
    event_group[0] = leader;
    if (!is_software_event(leader)) {
    if (leader.pmu != event.pmu)
    return false;
    if (leader != event && !hns3_pmu_cmp_event(leader, event))
    event_group[counters++] = event;
    }
    for_each_sibling_event(sibling, event.group_leader) {
    if (is_software_event(sibling))
    continue;
    if (sibling.pmu != event.pmu)
    return false;
    for (num = 0; num < counters; num++) {
//
// If we find a related event, then it's a valid group
// since we don't need to allocate a new counter for it.
//
    if (hns3_pmu_cmp_event(event_group[num], sibling))
    break;
    }
//
// Otherwise it's a new event but if there's no available counter,
// fail the check since we cannot schedule all the events in
// the group simultaneously.
//
    if (num == HNS3_PMU_MAX_HW_EVENTS)
    return false;
    if (num == counters)
    event_group[counters++] = sibling;
    }
    return true;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_get_filter_condition(event: *mut perf_event) -> u32 {
    static u32 hns3_pmu_get_filter_condition(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    let mut intr_id: u16 = hns3_pmu_get_intr(event);
    let mut port_id: u8 = hns3_pmu_get_port(event);
    let mut bdf: u16 = hns3_pmu_get_bdf(event);
    let mut tc_id: u8 = hns3_pmu_get_tc(event);
    u8 filter_mode;
    filter_mode = *(u8 *)hwc.addr_filters;
    switch (filter_mode) {
    case HNS3_PMU_HW_FILTER_PORT:
    return FILTER_CONDITION_PORT(port_id);
    case HNS3_PMU_HW_FILTER_PORT_TC:
    return FILTER_CONDITION_PORT_TC(port_id, tc_id);
    case HNS3_PMU_HW_FILTER_FUNC:
    case HNS3_PMU_HW_FILTER_FUNC_QUEUE:
    return GET_PCI_DEVFN(bdf);
    case HNS3_PMU_HW_FILTER_FUNC_INTR:
    return FILTER_CONDITION_FUNC_INTR(GET_PCI_DEVFN(bdf), intr_id);
    default:
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_config_filter(event: *mut perf_event) {
    static void hns3_pmu_config_filter(struct perf_event *event)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    let mut event_type: u8 = hns3_pmu_get_event_type(event);
    let mut subevent_id: u8 = hns3_pmu_get_subevent(event);
    let mut queue_id: u16 = hns3_pmu_get_queue(event);
    struct hw_perf_event *hwc = &event.hw;
    let mut filter_mode: u8 = *(u8 *)hwc.addr_filters;
    let mut bdf: u16 = hns3_pmu_get_bdf(event);
    let mut idx: u32 = hwc.idx;
    u32 val;
    val = event_type;
    val |= subevent_id << HNS3_PMU_CTRL_SUBEVENT_S;
    val |= filter_mode << HNS3_PMU_CTRL_FILTER_MODE_S;
    val |= HNS3_PMU_EVENT_OVERFLOW_RESTART;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx, val);
    val = hns3_pmu_get_filter_condition(event);
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_HIGH, idx, val);
    if (filter_mode == HNS3_PMU_HW_FILTER_FUNC_QUEUE)
    hns3_pmu_set_qid_para(hns3_pmu, idx, bdf, queue_id);
    }
    static void hns3_pmu_enable_counter(struct hns3_pmu *hns3_pmu,
    struct hw_perf_event *hwc)
    {
    let mut idx: u32 = hwc.idx;
    u32 val;
    val = hns3_pmu_readl(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx);
    val |= HNS3_PMU_EVENT_EN;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx, val);
    }
    static void hns3_pmu_disable_counter(struct hns3_pmu *hns3_pmu,
    struct hw_perf_event *hwc)
    {
    let mut idx: u32 = hwc.idx;
    u32 val;
    val = hns3_pmu_readl(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx);
    val &= ~HNS3_PMU_EVENT_EN;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx, val);
    }
    static void hns3_pmu_enable_intr(struct hns3_pmu *hns3_pmu,
    struct hw_perf_event *hwc)
    {
    let mut idx: u32 = hwc.idx;
    u32 val;
    val = hns3_pmu_readl(hns3_pmu, HNS3_PMU_REG_EVENT_INTR_MASK, idx);
    val &= ~HNS3_PMU_INTR_MASK_OVERFLOW;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_INTR_MASK, idx, val);
    }
    static void hns3_pmu_disable_intr(struct hns3_pmu *hns3_pmu,
    struct hw_perf_event *hwc)
    {
    let mut idx: u32 = hwc.idx;
    u32 val;
    val = hns3_pmu_readl(hns3_pmu, HNS3_PMU_REG_EVENT_INTR_MASK, idx);
    val |= HNS3_PMU_INTR_MASK_OVERFLOW;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_INTR_MASK, idx, val);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_clear_intr_status(hns3_pmu: *mut hns3_pmu, idx: u32) {
    static void hns3_pmu_clear_intr_status(struct hns3_pmu *hns3_pmu, u32 idx)
    {
    u32 val;
    val = hns3_pmu_readl(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx);
    val |= HNS3_PMU_EVENT_STATUS_RESET;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx, val);
    val = hns3_pmu_readl(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx);
    val &= ~HNS3_PMU_EVENT_STATUS_RESET;
    hns3_pmu_writel(hns3_pmu, HNS3_PMU_REG_EVENT_CTRL_LOW, idx, val);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_read_counter(event: *mut perf_event) -> u64 {
    static u64 hns3_pmu_read_counter(struct perf_event *event)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    return hns3_pmu_readq(hns3_pmu, event.hw.event_base, event.hw.idx);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_write_counter(event: *mut perf_event, value: u64) {
    static void hns3_pmu_write_counter(struct perf_event *event, u64 value)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    let mut idx: u32 = event.hw.idx;
    hns3_pmu_writeq(hns3_pmu, HNS3_PMU_REG_EVENT_COUNTER, idx, value);
    hns3_pmu_writeq(hns3_pmu, HNS3_PMU_REG_EVENT_EXT_COUNTER, idx, value);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_init_counter(event: *mut perf_event) {
    static void hns3_pmu_init_counter(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    local64_set(&hwc.prev_count, 0);
    hns3_pmu_write_counter(event, 0);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_event_init(event: *mut perf_event) -> c_int {
    static int hns3_pmu_event_init(struct perf_event *event)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    int idx;
    int ret;
    if (event.attr.type != event.pmu.type)
    return -ENOENT;
// Sampling is not supported
    if (is_sampling_event(event) || event.attach_state & PERF_ATTACH_TASK)
    return -EOPNOTSUPP;
    event.cpu = hns3_pmu.on_cpu;
    idx = hns3_pmu_get_event_idx(hns3_pmu);
    if (idx < 0) {
    pci_err(hns3_pmu.pdev, "Up to %u events are supported!\n",
    HNS3_PMU_MAX_HW_EVENTS);
    return -EBUSY;
    }
    hwc.idx = idx;
    ret = hns3_pmu_select_filter_mode(event, hns3_pmu);
    if (ret) {
    pci_err(hns3_pmu.pdev, "Invalid filter, ret = %d.\n", ret);
    return ret;
    }
    if (!hns3_pmu_validate_event_group(event)) {
    pci_err(hns3_pmu.pdev, "Invalid event group.\n");
    return -EINVAL;
    }
    if (hns3_pmu_get_ext_counter_used(event))
    hwc.event_base = HNS3_PMU_REG_EVENT_EXT_COUNTER;
    else
    hwc.event_base = HNS3_PMU_REG_EVENT_COUNTER;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_read(event: *mut perf_event) {
    static void hns3_pmu_read(struct perf_event *event)
    {
    struct hw_perf_event *hwc = &event.hw;
    u64 new_cnt, prev_cnt, delta;
    do {
    prev_cnt = local64_read(&hwc.prev_count);
    new_cnt = hns3_pmu_read_counter(event);
    } while (local64_cmpxchg(&hwc.prev_count, prev_cnt, new_cnt) !=
    prev_cnt);
    delta = new_cnt - prev_cnt;
    local64_add(delta, &event.count);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_start(event: *mut perf_event, flags: c_int) {
    static void hns3_pmu_start(struct perf_event *event, int flags)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    if (WARN_ON_ONCE(!(hwc.state & PERF_HES_STOPPED)))
    return;
    WARN_ON_ONCE(!(hwc.state & PERF_HES_UPTODATE));
    hwc.state = 0;
    hns3_pmu_config_filter(event);
    hns3_pmu_init_counter(event);
    hns3_pmu_enable_intr(hns3_pmu, hwc);
    hns3_pmu_enable_counter(hns3_pmu, hwc);
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_stop(event: *mut perf_event, flags: c_int) {
    static void hns3_pmu_stop(struct perf_event *event, int flags)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    hns3_pmu_disable_counter(hns3_pmu, hwc);
    hns3_pmu_disable_intr(hns3_pmu, hwc);
    WARN_ON_ONCE(hwc.state & PERF_HES_STOPPED);
    hwc.state |= PERF_HES_STOPPED;
    if (hwc.state & PERF_HES_UPTODATE)
    return;
// Read hardware counter and update the perf counter statistics
    hns3_pmu_read(event);
    hwc.state |= PERF_HES_UPTODATE;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_add(event: *mut perf_event, flags: c_int) -> c_int {
    static int hns3_pmu_add(struct perf_event *event, int flags)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    int idx;
    hwc.state = PERF_HES_STOPPED | PERF_HES_UPTODATE;
// Check all working events to find a related event.
    idx = hns3_pmu_find_related_event_idx(hns3_pmu, event);
    if (idx < 0 && idx != -ENOENT)
    return idx;
// Current event shares an enabled hardware event with related event
    if (idx >= 0 && idx < HNS3_PMU_MAX_HW_EVENTS) {
    hwc.idx = idx;
    goto start_count;
    }
    idx = hns3_pmu_get_event_idx(hns3_pmu);
    if (idx < 0)
    return idx;
    hwc.idx = idx;
    hns3_pmu.hw_events[idx] = event;
    start_count:
    if (flags & PERF_EF_START)
    hns3_pmu_start(event, PERF_EF_RELOAD);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_del(event: *mut perf_event, flags: c_int) {
    static void hns3_pmu_del(struct perf_event *event, int flags)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(event.pmu);
    struct hw_perf_event *hwc = &event.hw;
    hns3_pmu_stop(event, PERF_EF_UPDATE);
    hns3_pmu.hw_events[hwc.idx] = core::ptr::null_mut();
    perf_event_update_userpage(event);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_enable(pmu: *mut pmu) {
    static void hns3_pmu_enable(struct pmu *pmu)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(pmu);
    u32 val;
    val = readl(hns3_pmu.base + HNS3_PMU_REG_GLOBAL_CTRL);
    val |= HNS3_PMU_GLOBAL_START;
    writel(val, hns3_pmu.base + HNS3_PMU_REG_GLOBAL_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_disable(pmu: *mut pmu) {
    static void hns3_pmu_disable(struct pmu *pmu)
    {
    struct hns3_pmu *hns3_pmu = to_hns3_pmu(pmu);
    u32 val;
    val = readl(hns3_pmu.base + HNS3_PMU_REG_GLOBAL_CTRL);
    val &= ~HNS3_PMU_GLOBAL_START;
    writel(val, hns3_pmu.base + HNS3_PMU_REG_GLOBAL_CTRL);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_alloc_pmu(pdev: *mut pci_dev, hns3_pmu: *mut hns3_pmu) -> c_int {
    static int hns3_pmu_alloc_pmu(struct pci_dev *pdev, struct hns3_pmu *hns3_pmu)
    {
    u16 device_id;
    char *name;
    u32 val;
    hns3_pmu.base = pcim_iomap_table(pdev)[BAR_2];
    if (!hns3_pmu.base) {
    pci_err(pdev, "ioremap failed\n");
    return -ENOMEM;
    }
    hns3_pmu.hw_clk_freq = readl(hns3_pmu.base + HNS3_PMU_REG_CLOCK_FREQ);
    val = readl(hns3_pmu.base + HNS3_PMU_REG_BDF);
    hns3_pmu.bdf_min = val & 0xffff;
    hns3_pmu.bdf_max = val >> 16;
    val = readl(hns3_pmu.base + HNS3_PMU_REG_DEVICE_ID);
    device_id = val & 0xffff;
    name = devm_kasprintf(&pdev.dev, GFP_KERNEL, "hns3_pmu_sicl_%u", device_id);
    if (!name)
    return -ENOMEM;
    hns3_pmu.pdev = pdev;
    hns3_pmu.on_cpu = -1;
    hns3_pmu.identifier = readl(hns3_pmu.base + HNS3_PMU_REG_VERSION);
    hns3_pmu.pmu = (struct pmu) {
    .name		= name,
    .module		= THIS_MODULE,
    .parent		= &pdev.dev,
    .event_init	= hns3_pmu_event_init,
    .pmu_enable	= hns3_pmu_enable,
    .pmu_disable	= hns3_pmu_disable,
    .add		= hns3_pmu_add,
    .del		= hns3_pmu_del,
    .start		= hns3_pmu_start,
    .stop		= hns3_pmu_stop,
    .read		= hns3_pmu_read,
    .task_ctx_nr	= perf_invalid_context,
    .attr_groups	= hns3_pmu_attr_groups,
    .capabilities	= PERF_PMU_CAP_NO_EXCLUDE,
    };
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t hns3_pmu_irq(int irq, void *data)
    {
    struct hns3_pmu *hns3_pmu = data;
    u32 intr_status, idx;
    for (idx = 0; idx < HNS3_PMU_MAX_HW_EVENTS; idx++) {
    intr_status = hns3_pmu_readl(hns3_pmu,
    HNS3_PMU_REG_EVENT_INTR_STATUS,
    idx);
//
// As each counter will restart from 0 when it is overflowed,
// extra processing is no need, just clear interrupt status.
//
    if (intr_status)
    hns3_pmu_clear_intr_status(hns3_pmu, idx);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_online_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int hns3_pmu_online_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct hns3_pmu *hns3_pmu;
    hns3_pmu = hlist_entry_safe(node, struct hns3_pmu, node);
    if (!hns3_pmu)
    return -ENODEV;
    if (hns3_pmu.on_cpu == -1) {
    hns3_pmu.on_cpu = cpu;
    irq_set_affinity(hns3_pmu.irq, cpumask_of(cpu));
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_offline_cpu(cpu: c_uint, node: *mut hlist_node) -> c_int {
    static int hns3_pmu_offline_cpu(unsigned int cpu, struct hlist_node *node)
    {
    struct hns3_pmu *hns3_pmu;
    unsigned int target;
    hns3_pmu = hlist_entry_safe(node, struct hns3_pmu, node);
    if (!hns3_pmu)
    return -ENODEV;
// Nothing to do if this CPU doesn't own the PMU
    if (hns3_pmu.on_cpu != cpu)
    return 0;
// Choose a new CPU from all online cpus
    target = cpumask_any_but(cpu_online_mask, cpu);
    if (target >= nr_cpu_ids)
    return 0;
    perf_pmu_migrate_context(&hns3_pmu.pmu, cpu, target);
    hns3_pmu.on_cpu = target;
    irq_set_affinity(hns3_pmu.irq, cpumask_of(target));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_free_irq(data: *mut c_void) {
    static void hns3_pmu_free_irq(void *data)
    {
    struct pci_dev *pdev = data;
    pci_free_irq_vectors(pdev);
    }
    static int hns3_pmu_irq_register(struct pci_dev *pdev,
    struct hns3_pmu *hns3_pmu)
    {
    int irq, ret;
    ret = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_MSI);
    if (ret < 0) {
    pci_err(pdev, "failed to enable MSI vectors, ret = %d.\n", ret);
    return ret;
    }
    ret = devm_add_action_or_reset(&pdev.dev, hns3_pmu_free_irq, pdev);
    if (ret) {
    pci_err(pdev, "failed to add free irq action, ret = %d.\n", ret);
    return ret;
    }
    irq = pci_irq_vector(pdev, 0);
    ret = devm_request_irq(&pdev.dev, irq, hns3_pmu_irq, 0,
    hns3_pmu.pmu.name, hns3_pmu);
    if (ret) {
    pci_err(pdev, "failed to register irq, ret = %d.\n", ret);
    return ret;
    }
    hns3_pmu.irq = irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_init_pmu(pdev: *mut pci_dev, hns3_pmu: *mut hns3_pmu) -> c_int {
    static int hns3_pmu_init_pmu(struct pci_dev *pdev, struct hns3_pmu *hns3_pmu)
    {
    int ret;
    ret = hns3_pmu_alloc_pmu(pdev, hns3_pmu);
    if (ret)
    return ret;
    ret = hns3_pmu_irq_register(pdev, hns3_pmu);
    if (ret)
    return ret;
    ret = cpuhp_state_add_instance(CPUHP_AP_PERF_ARM_HNS3_PMU_ONLINE,
    &hns3_pmu.node);
    if (ret) {
    pci_err(pdev, "failed to register hotplug, ret = %d.\n", ret);
    return ret;
    }
    ret = perf_pmu_register(&hns3_pmu.pmu, hns3_pmu.pmu.name, -1);
    if (ret) {
    pci_err(pdev, "failed to register perf PMU, ret = %d.\n", ret);
    cpuhp_state_remove_instance_nocalls(CPUHP_AP_PERF_ARM_HNS3_PMU_ONLINE,
    &hns3_pmu.node);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_uninit_pmu(pdev: *mut pci_dev) {
    static void hns3_pmu_uninit_pmu(struct pci_dev *pdev)
    {
    struct hns3_pmu *hns3_pmu = pci_get_drvdata(pdev);
    perf_pmu_unregister(&hns3_pmu.pmu);
    cpuhp_state_remove_instance_nocalls(CPUHP_AP_PERF_ARM_HNS3_PMU_ONLINE,
    &hns3_pmu.node);
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_init_dev(pdev: *mut pci_dev) -> c_int {
    static int hns3_pmu_init_dev(struct pci_dev *pdev)
    {
    int ret;
    ret = pcim_enable_device(pdev);
    if (ret) {
    pci_err(pdev, "failed to enable pci device, ret = %d.\n", ret);
    return ret;
    }
    ret = pcim_iomap_regions(pdev, BIT(BAR_2), "hns3_pmu");
    if (ret < 0) {
    pci_err(pdev, "failed to request pci region, ret = %d.\n", ret);
    return ret;
    }
    pci_set_master(pdev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int hns3_pmu_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct hns3_pmu *hns3_pmu;
    int ret;
    hns3_pmu = devm_kzalloc(&pdev.dev, sizeof(*hns3_pmu), GFP_KERNEL);
    if (!hns3_pmu)
    return -ENOMEM;
    ret = hns3_pmu_init_dev(pdev);
    if (ret)
    return ret;
    ret = hns3_pmu_init_pmu(pdev, hns3_pmu);
    if (ret) {
    pci_clear_master(pdev);
    return ret;
    }
    pci_set_drvdata(pdev, hns3_pmu);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn hns3_pmu_remove(pdev: *mut pci_dev) {
    static void hns3_pmu_remove(struct pci_dev *pdev)
    {
    hns3_pmu_uninit_pmu(pdev);
    pci_clear_master(pdev);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    }
    static const struct pci_device_id hns3_pmu_ids[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_HUAWEI, 0xa22b) },
    { 0, }
    };
    MODULE_DEVICE_TABLE(pci, hns3_pmu_ids);
    static struct pci_driver hns3_pmu_driver = {
    .name = "hns3_pmu",
    .id_table = hns3_pmu_ids,
    .probe = hns3_pmu_probe,
    .remove = hns3_pmu_remove,
    };
#[no_mangle]
unsafe extern "C" fn hns3_pmu_module_init() -> int __init {
    static int __init hns3_pmu_module_init(void)
    {
    int ret;
    ret = cpuhp_setup_state_multi(CPUHP_AP_PERF_ARM_HNS3_PMU_ONLINE,
    "AP_PERF_ARM_HNS3_PMU_ONLINE",
    hns3_pmu_online_cpu,
    hns3_pmu_offline_cpu);
    if (ret) {
    pr_err("failed to setup HNS3 PMU hotplug, ret = %d.\n", ret);
    return ret;
    }
    ret = pci_register_driver(&hns3_pmu_driver);
    if (ret) {
    pr_err("failed to register pci driver, ret = %d.\n", ret);
    cpuhp_remove_multi_state(CPUHP_AP_PERF_ARM_HNS3_PMU_ONLINE);
    }
    return ret;
    }
    module_init(hns3_pmu_module_init);
#[no_mangle]
unsafe extern "C" fn hns3_pmu_module_exit() -> void __exit {
    static void __exit hns3_pmu_module_exit(void)
    {
    pci_unregister_driver(&hns3_pmu_driver);
    cpuhp_remove_multi_state(CPUHP_AP_PERF_ARM_HNS3_PMU_ONLINE);
    }
    module_exit(hns3_pmu_module_exit);
    MODULE_DESCRIPTION("HNS3 PMU driver");
    MODULE_LICENSE("GPL v2");
