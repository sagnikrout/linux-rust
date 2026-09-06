//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/power.c
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
// System Control and Management Interface (SCMI) Power Protocol
//
// Copyright (C) 2018-2022 ARM Ltd.
//

// Updated only after ALL the mandatory features for that version are merged
pub const SCMI_PROTOCOL_SUPPORTED_VERSION: c_uint = 0x30001;
    enum scmi_power_protocol_cmd {
    POWER_DOMAIN_ATTRIBUTES = 0x3,
    POWER_STATE_SET = 0x4,
    POWER_STATE_GET = 0x5,
    POWER_STATE_NOTIFY = 0x6,
    POWER_DOMAIN_NAME_GET = 0x8,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_power_attributes {
    pub num_domains: __le16,
    pub reserved: __le16,
    pub stats_addr_low: __le32,
    pub stats_addr_high: __le32,
    pub stats_size: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_power_domain_attributes {
    pub flags: __le32,
    pub name: [u8; SCMI_SHORT_NAME_MAX_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_power_set_state {
    pub flags: __le32,

    pub domain: __le32,
    pub state: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_power_state_notify {
    pub domain: __le32,
    pub notify_enable: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_power_state_notify_payld {
    pub agent_id: __le32,
    pub domain_id: __le32,
    pub power_state: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct power_dom_info {
    pub state_set_sync: bool,
    pub state_set_async: bool,
    pub state_set_notify: bool,
    pub name: [c_char; SCMI_MAX_STR_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_power_info {
    pub notify_state_change_cmd: bool,
    pub num_domains: c_int,
    pub stats_addr: u64,
    pub stats_size: u32,
    pub dom_info: *mut power_dom_info,
}

    static int scmi_power_attributes_get(const struct scmi_protocol_handle *ph,
    struct scmi_power_info *pi)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_msg_resp_power_attributes *attr;
    ret = ph.xops.xfer_get_init(ph, PROTOCOL_ATTRIBUTES,
    0, sizeof(*attr), &t);
    if (ret)
    return ret;
    attr = t.rx.buf;
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    pi.num_domains = le16_to_cpu(attr.num_domains);
    pi.stats_addr = le32_to_cpu(attr.stats_addr_low) |
    (u64)le32_to_cpu(attr.stats_addr_high) << 32;
    pi.stats_size = le32_to_cpu(attr.stats_size);
    }
    ph.xops.xfer_put(ph, t);
    if (!ret)
    if (!ph.hops.protocol_msg_check(ph, POWER_STATE_NOTIFY, core::ptr::null_mut()))
    pi.notify_state_change_cmd = true;
    return ret;
    }
    static int
    scmi_power_domain_attributes_get(const struct scmi_protocol_handle *ph,
    u32 domain, struct power_dom_info *dom_info,
    bool notify_state_change_cmd)
    {
    int ret;
    u32 flags;
    struct scmi_xfer *t;
    struct scmi_msg_resp_power_domain_attributes *attr;
    ret = ph.xops.xfer_get_init(ph, POWER_DOMAIN_ATTRIBUTES,
    sizeof(domain), sizeof(*attr), &t);
    if (ret)
    return ret;
    put_unaligned_le32(domain, t.tx.buf);
    attr = t.rx.buf;
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    flags = le32_to_cpu(attr.flags);
    if (notify_state_change_cmd)
    dom_info.state_set_notify =
    SUPPORTS_STATE_SET_NOTIFY(flags);
    dom_info.state_set_async = SUPPORTS_STATE_SET_ASYNC(flags);
    dom_info.state_set_sync = SUPPORTS_STATE_SET_SYNC(flags);
    strscpy(dom_info.name, attr.name, SCMI_SHORT_NAME_MAX_SIZE);
    }
    ph.xops.xfer_put(ph, t);
//
// If supported overwrite short name with the extended one;
// on error just carry on and use already provided short name.
//
    if (!ret && PROTOCOL_REV_MAJOR(ph.version) >= 0x3 &&
    SUPPORTS_EXTENDED_NAMES(flags)) {
    ph.hops.extended_name_get(ph, POWER_DOMAIN_NAME_GET,
    domain, core::ptr::null_mut(), dom_info.name,
    SCMI_MAX_STR_SIZE);
    }
    return ret;
    }
    static int scmi_power_state_set(const struct scmi_protocol_handle *ph,
    u32 domain, u32 state)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_power_set_state *st;
    ret = ph.xops.xfer_get_init(ph, POWER_STATE_SET, sizeof(*st), 0, &t);
    if (ret)
    return ret;
    st = t.tx.buf;
    st.flags = cpu_to_le32(0);
    st.domain = cpu_to_le32(domain);
    st.state = cpu_to_le32(state);
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_power_state_get(const struct scmi_protocol_handle *ph,
    u32 domain, u32 *state)
    {
    int ret;
    struct scmi_xfer *t;
    ret = ph.xops.xfer_get_init(ph, POWER_STATE_GET, sizeof(u32), sizeof(u32), &t);
    if (ret)
    return ret;
    put_unaligned_le32(domain, t.tx.buf);
    ret = ph.xops.do_xfer(ph, t);
    if (!ret)
// state = get_unaligned_le32(t->rx.buf);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scmi_power_num_domains_get(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_power_num_domains_get(const struct scmi_protocol_handle *ph)
    {
    struct scmi_power_info *pi = ph.get_priv(ph);
    return pi.num_domains;
    }
    static const char *
    scmi_power_name_get(const struct scmi_protocol_handle *ph,
    u32 domain)
    {
    struct scmi_power_info *pi = ph.get_priv(ph);
    struct power_dom_info *dom;
    if (domain >= pi.num_domains)
    return "unknown";
    dom = pi.dom_info + domain;
    return dom.name;
    }
    static const struct scmi_power_proto_ops power_proto_ops = {
    .num_domains_get = scmi_power_num_domains_get,
    .name_get = scmi_power_name_get,
    .state_set = scmi_power_state_set,
    .state_get = scmi_power_state_get,
    };
    static int scmi_power_request_notify(const struct scmi_protocol_handle *ph,
    u32 domain, bool enable)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_power_state_notify *notify;
    ret = ph.xops.xfer_get_init(ph, POWER_STATE_NOTIFY,
    sizeof(*notify), 0, &t);
    if (ret)
    return ret;
    notify = t.tx.buf;
    notify.domain = cpu_to_le32(domain);
    notify.notify_enable = enable ? cpu_to_le32(BIT(0)) : 0;
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static bool scmi_power_notify_supported(const struct scmi_protocol_handle *ph,
    u8 evt_id, u32 src_id)
    {
    struct power_dom_info *dom;
    struct scmi_power_info *pinfo = ph.get_priv(ph);
    if (evt_id != SCMI_EVENT_POWER_STATE_CHANGED ||
    src_id >= pinfo.num_domains)
    return false;
    dom = pinfo.dom_info + src_id;
    return dom.state_set_notify;
    }
    static int scmi_power_set_notify_enabled(const struct scmi_protocol_handle *ph,
    u8 evt_id, u32 src_id, bool enable)
    {
    int ret;
    ret = scmi_power_request_notify(ph, src_id, enable);
    if (ret)
    pr_debug("FAIL_ENABLE - evt[%X] dom[%d] - ret:%d\n",
    evt_id, src_id, ret);
    return ret;
    }
    static void *
    scmi_power_fill_custom_report(const struct scmi_protocol_handle *ph,
    u8 evt_id, ktime_t timestamp,
    const void *payld, size_t payld_sz,
    void *report, u32 *src_id)
    {
    const struct scmi_power_state_notify_payld *p = payld;
    struct scmi_power_state_changed_report *r = report;
    if (evt_id != SCMI_EVENT_POWER_STATE_CHANGED || sizeof(*p) != payld_sz)
    return core::ptr::null_mut();
    r.timestamp = timestamp;
    r.agent_id = le32_to_cpu(p.agent_id);
    r.domain_id = le32_to_cpu(p.domain_id);
    r.power_state = le32_to_cpu(p.power_state);
// src_id = r->domain_id;
    return r;
    }
#[no_mangle]
unsafe extern "C" fn scmi_power_get_num_sources(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_power_get_num_sources(const struct scmi_protocol_handle *ph)
    {
    struct scmi_power_info *pinfo = ph.get_priv(ph);
    if (!pinfo)
    return -EINVAL;
    return pinfo.num_domains;
    }
    static const struct scmi_event power_events[] = {
    {
    .id = SCMI_EVENT_POWER_STATE_CHANGED,
    .max_payld_sz = sizeof(struct scmi_power_state_notify_payld),
    .max_report_sz =
    sizeof(struct scmi_power_state_changed_report),
    },
    };
    static const struct scmi_event_ops power_event_ops = {
    .is_notify_supported = scmi_power_notify_supported,
    .get_num_sources = scmi_power_get_num_sources,
    .set_notify_enabled = scmi_power_set_notify_enabled,
    .fill_custom_report = scmi_power_fill_custom_report,
    };
    static const struct scmi_protocol_events power_protocol_events = {
    .queue_sz = SCMI_PROTO_QUEUE_SZ,
    .ops = &power_event_ops,
    .evts = power_events,
    .num_events = ARRAY_SIZE(power_events),
    };
#[no_mangle]
unsafe extern "C" fn scmi_power_protocol_init(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_power_protocol_init(const struct scmi_protocol_handle *ph)
    {
    int domain, ret;
    struct scmi_power_info *pinfo;
    dev_dbg(ph.dev, "Power Version %d.%d\n",
    PROTOCOL_REV_MAJOR(ph.version), PROTOCOL_REV_MINOR(ph.version));
    pinfo = devm_kzalloc(ph.dev, sizeof(*pinfo), GFP_KERNEL);
    if (!pinfo)
    return -ENOMEM;
    ret = scmi_power_attributes_get(ph, pinfo);
    if (ret)
    return ret;
    pinfo.dom_info = devm_kcalloc(ph.dev, pinfo.num_domains,
    sizeof(*pinfo.dom_info), GFP_KERNEL);
    if (!pinfo.dom_info)
    return -ENOMEM;
    for (domain = 0; domain < pinfo.num_domains; domain++) {
    struct power_dom_info *dom = pinfo.dom_info + domain;
    scmi_power_domain_attributes_get(ph, domain, dom,
    pinfo.notify_state_change_cmd);
    }
    return ph.set_priv(ph, pinfo);
    }
    static const struct scmi_protocol scmi_power = {
    .id = SCMI_PROTOCOL_POWER,
    .owner = THIS_MODULE,
    .instance_init = &scmi_power_protocol_init,
    .ops = &power_proto_ops,
    .events = &power_protocol_events,
    .supported_version = SCMI_PROTOCOL_SUPPORTED_VERSION,
    };
    DEFINE_SCMI_PROTOCOL_REGISTER_UNREGISTER(power, scmi_power)
