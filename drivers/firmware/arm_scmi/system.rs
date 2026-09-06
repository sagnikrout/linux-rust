//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/system.c
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
// System Control and Management Interface (SCMI) System Power Protocol
//
// Copyright (C) 2020-2022 ARM Ltd.
//

// Updated only after ALL the mandatory features for that version are merged
pub const SCMI_PROTOCOL_SUPPORTED_VERSION: c_uint = 0x20001;
pub const SCMI_SYSTEM_NUM_SOURCES: c_int = 1;
    enum scmi_system_protocol_cmd {
    SYSTEM_POWER_STATE_NOTIFY = 0x5,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_system_power_state_notify {
    pub notify_enable: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_system_power_state_notifier_payld {
    pub agent_id: __le32,
    pub flags: __le32,
    pub system_state: __le32,
    pub timeout: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_system_info {
    pub graceful_timeout_supported: bool,
    pub power_state_notify_cmd: bool,
}

    static bool scmi_system_notify_supported(const struct scmi_protocol_handle *ph,
    u8 evt_id, u32 src_id)
    {
    struct scmi_system_info *pinfo = ph.get_priv(ph);
    if (evt_id != SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER)
    return false;
    return pinfo.power_state_notify_cmd;
    }
    static int scmi_system_request_notify(const struct scmi_protocol_handle *ph,
    bool enable)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_system_power_state_notify *notify;
    ret = ph.xops.xfer_get_init(ph, SYSTEM_POWER_STATE_NOTIFY,
    sizeof(*notify), 0, &t);
    if (ret)
    return ret;
    notify = t.tx.buf;
    notify.notify_enable = enable ? cpu_to_le32(BIT(0)) : 0;
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_system_set_notify_enabled(const struct scmi_protocol_handle *ph,
    u8 evt_id, u32 src_id, bool enable)
    {
    int ret;
    ret = scmi_system_request_notify(ph, enable);
    if (ret)
    pr_debug("FAIL_ENABLE - evt[%X] - ret:%d\n", evt_id, ret);
    return ret;
    }
    static void *
    scmi_system_fill_custom_report(const struct scmi_protocol_handle *ph,
    u8 evt_id, ktime_t timestamp,
    const void *payld, size_t payld_sz,
    void *report, u32 *src_id)
    {
    size_t expected_sz;
    const struct scmi_system_power_state_notifier_payld *p = payld;
    struct scmi_system_power_state_notifier_report *r = report;
    struct scmi_system_info *pinfo = ph.get_priv(ph);
    expected_sz = pinfo.graceful_timeout_supported ?
    sizeof(*p) : sizeof(*p) - sizeof(__le32);
    if (evt_id != SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER ||
    payld_sz != expected_sz)
    return core::ptr::null_mut();
    r.timestamp = timestamp;
    r.agent_id = le32_to_cpu(p.agent_id);
    r.flags = le32_to_cpu(p.flags);
    r.system_state = le32_to_cpu(p.system_state);
    if (pinfo.graceful_timeout_supported &&
    r.system_state == SCMI_SYSTEM_SHUTDOWN &&
    SCMI_SYSPOWER_IS_REQUEST_GRACEFUL(r.flags))
    r.timeout = le32_to_cpu(p.timeout);
    else
    r.timeout = 0x00;
// src_id = 0;
    return r;
    }
    static const struct scmi_event system_events[] = {
    {
    .id = SCMI_EVENT_SYSTEM_POWER_STATE_NOTIFIER,
    .max_payld_sz =
    sizeof(struct scmi_system_power_state_notifier_payld),
    .max_report_sz =
    sizeof(struct scmi_system_power_state_notifier_report),
    },
    };
    static const struct scmi_event_ops system_event_ops = {
    .is_notify_supported = scmi_system_notify_supported,
    .set_notify_enabled = scmi_system_set_notify_enabled,
    .fill_custom_report = scmi_system_fill_custom_report,
    };
    static const struct scmi_protocol_events system_protocol_events = {
    .queue_sz = SCMI_PROTO_QUEUE_SZ,
    .ops = &system_event_ops,
    .evts = system_events,
    .num_events = ARRAY_SIZE(system_events),
    .num_sources = SCMI_SYSTEM_NUM_SOURCES,
    };
#[no_mangle]
unsafe extern "C" fn scmi_system_protocol_init(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_system_protocol_init(const struct scmi_protocol_handle *ph)
    {
    struct scmi_system_info *pinfo;
    dev_dbg(ph.dev, "System Power Version %d.%d\n",
    PROTOCOL_REV_MAJOR(ph.version), PROTOCOL_REV_MINOR(ph.version));
    pinfo = devm_kzalloc(ph.dev, sizeof(*pinfo), GFP_KERNEL);
    if (!pinfo)
    return -ENOMEM;
    if (PROTOCOL_REV_MAJOR(ph.version) >= 0x2)
    pinfo.graceful_timeout_supported = true;
    if (!ph.hops.protocol_msg_check(ph, SYSTEM_POWER_STATE_NOTIFY, core::ptr::null_mut()))
    pinfo.power_state_notify_cmd = true;
    return ph.set_priv(ph, pinfo);
    }
    static const struct scmi_protocol scmi_system = {
    .id = SCMI_PROTOCOL_SYSTEM,
    .owner = THIS_MODULE,
    .instance_init = &scmi_system_protocol_init,
    .ops = core::ptr::null_mut(),
    .events = &system_protocol_events,
    .supported_version = SCMI_PROTOCOL_SUPPORTED_VERSION,
    };
    DEFINE_SCMI_PROTOCOL_REGISTER_UNREGISTER(system, scmi_system)
