//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/clock.c
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
// System Control and Management Interface (SCMI) Clock Protocol
//
// Copyright (C) 2018-2022 ARM Ltd.
//

// Updated only after ALL the mandatory features for that version are merged
pub const SCMI_PROTOCOL_SUPPORTED_VERSION: c_uint = 0x30000;
    enum scmi_clock_protocol_cmd {
    CLOCK_ATTRIBUTES = 0x3,
    CLOCK_DESCRIBE_RATES = 0x4,
    CLOCK_RATE_SET = 0x5,
    CLOCK_RATE_GET = 0x6,
    CLOCK_CONFIG_SET = 0x7,
    CLOCK_NAME_GET = 0x8,
    CLOCK_RATE_NOTIFY = 0x9,
    CLOCK_RATE_CHANGE_REQUESTED_NOTIFY = 0xA,
    CLOCK_CONFIG_GET = 0xB,
    CLOCK_POSSIBLE_PARENTS_GET = 0xC,
    CLOCK_PARENT_SET = 0xD,
    CLOCK_PARENT_GET = 0xE,
    CLOCK_GET_PERMISSIONS = 0xF,
    };

    enum clk_state {
    CLK_STATE_DISABLE,
    CLK_STATE_ENABLE,
    CLK_STATE_RESERVED,
    CLK_STATE_UNCHANGED,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_clock_protocol_attributes {
    pub num_clocks: __le16,
    pub max_async_req: u8,
    pub reserved: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_clock_attributes {
    pub attributes: __le32,
    pub name: [u8; SCMI_SHORT_NAME_MAX_SIZE],
    pub clock_enable_latency: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_clock_possible_parents {
    pub id: __le32,
    pub skip_parents: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_clock_possible_parents {
    pub num_parent_flags: __le32,
    pub possible_parents: [__le32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_clock_set_parent {
    pub id: __le32,
    pub parent_id: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_clock_config_set {
    pub id: __le32,
    pub attributes: __le32,
}

// Valid only from SCMI clock v2.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_clock_config_set_v2 {
    pub id: __le32,
    pub attributes: __le32,
pub const NULL_OEM_TYPE: c_int = 0;

    pub oem_config_val: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_clock_config_get {
    pub id: __le32,
    pub flags: __le32,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_clock_config_get {
    pub attributes: __le32,
    pub config: __le32,

    pub oem_config_val: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_clock_describe_rates {
    pub id: __le32,
    pub rate_index: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_clock_describe_rates {
    pub num_rates_flags: __le32,

    struct {
    pub value_low: __le32,
    pub value_high: __le32,
    pub rate: [}; ],
    ({				\
    pub \: typeof(X) x = (X);,
    pub \: le32_to_cpu((x).value_low) | (u64)le32_to_cpu((x).value_high) << 32;,
    })
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clock_set_rate {
    pub flags: __le32,

    pub id: __le32,
    pub value_low: __le32,
    pub value_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_resp_set_rate_complete {
    pub id: __le32,
    pub rate_low: __le32,
    pub rate_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_msg_clock_rate_notify {
    pub clk_id: __le32,
    pub notify_enable: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clock_rate_notify_payld {
    pub agent_id: __le32,
    pub clock_id: __le32,
    pub rate_low: __le32,
    pub rate_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clock_desc {
    pub id: u32,
    pub tot_rates: c_uint,
    pub r: scmi_clock_rates,
pub const RATE_MIN: c_int = 0;
pub const RATE_MAX: c_int = 1;
pub const RATE_STEP: c_int = 2;
    pub info: scmi_clock_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clock_info {
    pub num_clocks: c_int,
    pub max_async_req: c_int,
    pub notify_rate_changed_cmd: bool,
    pub notify_rate_change_requested_cmd: bool,
    pub cur_async_req: core::sync::atomic::AtomicI32,
    pub clkds: *mut scmi_clock_desc,

    int (*clock_config_set)(const struct scmi_protocol_handle *ph,
    u32 clk_id, enum clk_state state,
    enum scmi_clock_oem_config oem_type,
    pub atomic): u32 oem_val, bool,
    int (*clock_config_get)(const struct scmi_protocol_handle *ph,
    u32 clk_id, enum scmi_clock_oem_config oem_type,
    u32 *attributes, bool *enabled, u32 *oem_val,
    pub atomic): bool,
}

    static enum scmi_clock_protocol_cmd evt_2_cmd[] = {
    CLOCK_RATE_NOTIFY,
    CLOCK_RATE_CHANGE_REQUESTED_NOTIFY,
    };
    static inline struct scmi_clock_info *
    scmi_clock_domain_lookup(struct clock_info *ci, u32 clk_id)
    {
    if (clk_id >= ci.num_clocks)
    return ERR_PTR(-EINVAL);
    return CLOCK_INFO(ci, clk_id);
    }
    static int
    scmi_clock_protocol_attributes_get(const struct scmi_protocol_handle *ph,
    struct clock_info *ci)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_msg_resp_clock_protocol_attributes *attr;
    ret = ph.xops.xfer_get_init(ph, PROTOCOL_ATTRIBUTES,
    0, sizeof(*attr), &t);
    if (ret)
    return ret;
    attr = t.rx.buf;
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    ci.num_clocks = le16_to_cpu(attr.num_clocks);
    ci.max_async_req = attr.max_async_req;
    }
    ph.xops.xfer_put(ph, t);
    if (!ret) {
    if (!ph.hops.protocol_msg_check(ph, CLOCK_RATE_NOTIFY, core::ptr::null_mut()))
    ci.notify_rate_changed_cmd = true;
    if (!ph.hops.protocol_msg_check(ph,
    CLOCK_RATE_CHANGE_REQUESTED_NOTIFY,
    core::ptr::null_mut()))
    ci.notify_rate_change_requested_cmd = true;
    }
    return ret;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_clk_ipriv {
    pub dev: *mut device,
    pub clkd: *mut scmi_clock_desc,
}

    static void iter_clk_possible_parents_prepare_message(void *message, unsigned int desc_index,
    const void *priv)
    {
    struct scmi_msg_clock_possible_parents *msg = message;
    const struct scmi_clk_ipriv *p = priv;
    msg.id = cpu_to_le32(p.clkd.id);
// Set the number of OPPs to be skipped/already read
    msg.skip_parents = cpu_to_le32(desc_index);
    }
    static int iter_clk_possible_parents_update_state(struct scmi_iterator_state *st,
    const void *response, void *priv)
    {
    const struct scmi_msg_resp_clock_possible_parents *r = response;
    struct scmi_clk_ipriv *p = priv;
    u32 flags;
    flags = le32_to_cpu(r.num_parent_flags);
    st.num_returned = NUM_PARENTS_RETURNED(flags);
    st.num_remaining = NUM_PARENTS_REMAINING(flags);
//
// num parents is not declared previously anywhere so we
// assume it's returned+remaining on first call.
//
    if (!st.max_resources) {
    let mut num_parents: c_int = st.num_returned + st.num_remaining;
    p.clkd.info.parents = devm_kcalloc(p.dev, num_parents,
    sizeof(*p.clkd.info.parents),
    GFP_KERNEL);
    if (!p.clkd.info.parents)
    return -ENOMEM;
// max_resources is used by the iterators to control bounds
    st.max_resources = st.num_returned + st.num_remaining;
    }
    return 0;
    }
    static int iter_clk_possible_parents_process_response(const struct scmi_protocol_handle *ph,
    const void *response,
    struct scmi_iterator_state *st,
    void *priv)
    {
    const struct scmi_msg_resp_clock_possible_parents *r = response;
    struct scmi_clk_ipriv *p = priv;
    p.clkd.info.parents[st.desc_index + st.loop_idx] =
    le32_to_cpu(r.possible_parents[st.loop_idx]);
// Count only effectively discovered parents
    p.clkd.info.num_parents++;
    return 0;
    }
    static int scmi_clock_possible_parents(const struct scmi_protocol_handle *ph,
    u32 clk_id, struct clock_info *cinfo)
    {
    struct scmi_iterator_ops ops = {
    .prepare_message = iter_clk_possible_parents_prepare_message,
    .update_state = iter_clk_possible_parents_update_state,
    .process_response = iter_clk_possible_parents_process_response,
    };
    struct scmi_clock_desc *clkd = &cinfo.clkds[clk_id];
    struct scmi_clk_ipriv ppriv = {
    .clkd = clkd,
    .dev = ph.dev,
    };
    void *iter;
    iter = ph.hops.iter_response_init(ph, &ops, 0,
    CLOCK_POSSIBLE_PARENTS_GET,
    sizeof(struct scmi_msg_clock_possible_parents),
    &ppriv);
    if (IS_ERR(iter))
    return PTR_ERR(iter);
    return ph.hops.iter_response_run(iter);
    }
    static int
    scmi_clock_get_permissions(const struct scmi_protocol_handle *ph, u32 clk_id,
    struct scmi_clock_info *clk)
    {
    struct scmi_xfer *t;
    u32 perm;
    int ret;
    ret = ph.xops.xfer_get_init(ph, CLOCK_GET_PERMISSIONS,
    sizeof(clk_id), sizeof(perm), &t);
    if (ret)
    return ret;
    put_unaligned_le32(clk_id, t.tx.buf);
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    perm = get_unaligned_le32(t.rx.buf);
    clk.state_ctrl_forbidden = !(perm & CLOCK_STATE_CONTROL_ALLOWED);
    clk.rate_ctrl_forbidden = !(perm & CLOCK_RATE_CONTROL_ALLOWED);
    clk.parent_ctrl_forbidden = !(perm & CLOCK_PARENT_CONTROL_ALLOWED);
    }
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_clock_attributes_get(const struct scmi_protocol_handle *ph,
    u32 clk_id, struct clock_info *cinfo)
    {
    int ret;
    u32 attributes;
    struct scmi_xfer *t;
    struct scmi_msg_resp_clock_attributes *attr;
    struct scmi_clock_info *clk = CLOCK_INFO(cinfo, clk_id);
    ret = ph.xops.xfer_get_init(ph, CLOCK_ATTRIBUTES,
    sizeof(clk_id), sizeof(*attr), &t);
    if (ret)
    return ret;
    put_unaligned_le32(clk_id, t.tx.buf);
    attr = t.rx.buf;
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    let mut latency: u32 = 0;
    attributes = le32_to_cpu(attr.attributes);
    strscpy(clk.name, attr.name, SCMI_SHORT_NAME_MAX_SIZE);
// clock_enable_latency field is present only since SCMI v3.1
    if (PROTOCOL_REV_MAJOR(ph.version) >= 0x2)
    latency = le32_to_cpu(attr.clock_enable_latency);
    clk.enable_latency = latency ? : U32_MAX;
    }
    ph.xops.xfer_put(ph, t);
//
// If supported overwrite short name with the extended one;
// on error just carry on and use already provided short name.
//
    if (!ret && PROTOCOL_REV_MAJOR(ph.version) >= 0x2) {
    if (SUPPORTS_EXTENDED_NAMES(attributes))
    ph.hops.extended_name_get(ph, CLOCK_NAME_GET, clk_id,
    core::ptr::null_mut(), clk.name,
    SCMI_MAX_STR_SIZE);
    if (cinfo.notify_rate_changed_cmd &&
    SUPPORTS_RATE_CHANGED_NOTIF(attributes))
    clk.rate_changed_notifications = true;
    if (cinfo.notify_rate_change_requested_cmd &&
    SUPPORTS_RATE_CHANGE_REQUESTED_NOTIF(attributes))
    clk.rate_change_requested_notifications = true;
    if (PROTOCOL_REV_MAJOR(ph.version) >= 0x3) {
    if (SUPPORTS_PARENT_CLOCK(attributes))
    scmi_clock_possible_parents(ph, clk_id, cinfo);
    if (SUPPORTS_GET_PERMISSIONS(attributes))
    scmi_clock_get_permissions(ph, clk_id, clk);
    if (SUPPORTS_EXTENDED_CONFIG(attributes))
    clk.extended_config = true;
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rate_cmp_func(_r1: *const c_void, _r2: *const c_void) -> c_int {
    static int rate_cmp_func(const void *_r1, const void *_r2)
    {
    const u64 *r1 = _r1, *r2 = _r2;
    if (*r1 < *r2)
    return -1;
#[no_mangle]
pub unsafe extern "C" fn if(r2: *mut *mut r1 ==) -> else {
    else if (*r1 == *r2)
    return 0;
    else
    return 1;
    }
    static void iter_clk_describe_prepare_message(void *message,
    const unsigned int desc_index,
    const void *priv)
    {
    struct scmi_msg_clock_describe_rates *msg = message;
    const struct scmi_clk_ipriv *p = priv;
    msg.id = cpu_to_le32(p.clkd.id);
// Set the number of rates to be skipped/already read
    msg.rate_index = cpu_to_le32(desc_index);
    }

    ({								       \
// \
// A known quirk: a triplet is returned but num_returned != 3  \
// Check for a safe payload size and fix.		       \
// \
    if (st.num_returned != 3 && st.num_remaining == 0 &&	       \
    st.rx_len == sizeof(*r) + sizeof(__le32) * 2 * 3) {       \
    st.num_returned = 3;				       \
    st.num_remaining = 0;				       \
    } else {						       \
    dev_err(p.dev,					       \
    "Cannot fix out-of-spec reply !\n");	       \
    return -EPROTO;					       \
    }							       \
    })
    static int
    iter_clk_describe_update_state(struct scmi_iterator_state *st,
    const void *response, void *priv)
    {
    u32 flags;
    struct scmi_clk_ipriv *p = priv;
    const struct scmi_msg_resp_clock_describe_rates *r = response;
    flags = le32_to_cpu(r.num_rates_flags);
    st.num_remaining = NUM_REMAINING(flags);
    st.num_returned = NUM_RETURNED(flags);
    p.clkd.r.rate_discrete = RATE_DISCRETE(flags);
// Warn about out of spec replies ...
    if (!p.clkd.r.rate_discrete &&
    (st.num_returned != 3 || st.num_remaining != 0)) {
    dev_warn(p.dev,
    "Out-of-spec CLOCK_DESCRIBE_RATES reply for %s - returned:%d remaining:%d rx_len:%zd\n",
    p.clkd.info.name, st.num_returned, st.num_remaining,
    st.rx_len);
    SCMI_QUIRK(clock_rates_triplet_out_of_spec,
    QUIRK_OUT_OF_SPEC_TRIPLET);
    }
    if (!st.max_resources) {
    let mut tot_rates: c_uint = st.num_returned + st.num_remaining;
    p.clkd.r.rates = devm_kcalloc(p.dev, tot_rates,
    sizeof(*p.clkd.r.rates), GFP_KERNEL);
    if (!p.clkd.r.rates)
    return -ENOMEM;
// max_resources is used by the iterators to control bounds
    p.clkd.tot_rates = tot_rates;
    st.max_resources = tot_rates;
    }
    return 0;
    }
    static int
    iter_clk_describe_process_response(const struct scmi_protocol_handle *ph,
    const void *response,
    struct scmi_iterator_state *st, void *priv)
    {
    struct scmi_clk_ipriv *p = priv;
    const struct scmi_msg_resp_clock_describe_rates *r = response;
    p.clkd.r.rates[p.clkd.r.num_rates] = RATE_TO_U64(r.rate[st.loop_idx]);
// Count only effectively discovered rates
    p.clkd.r.num_rates++;
    return 0;
    }
    static int
    scmi_clock_describe_rates_get_full(const struct scmi_protocol_handle *ph,
    struct scmi_clock_desc *clkd)
    {
    int ret;
    void *iter;
    struct scmi_iterator_ops ops = {
    .prepare_message = iter_clk_describe_prepare_message,
    .update_state = iter_clk_describe_update_state,
    .process_response = iter_clk_describe_process_response,
    };
    struct scmi_clk_ipriv cpriv = {
    .clkd = clkd,
    .dev = ph.dev,
    };
//
// Using tot_rates as max_resources parameter here so as to trigger
// the dynamic allocation only when strictly needed: when trying a
// full enumeration after a lazy one tot_rates will be non-zero.
//
    iter = ph.hops.iter_response_init(ph, &ops, clkd.tot_rates,
    CLOCK_DESCRIBE_RATES,
    sizeof(struct scmi_msg_clock_describe_rates),
    &cpriv);
    if (IS_ERR(iter))
    return PTR_ERR(iter);
    ret = ph.hops.iter_response_run(iter);
    if (ret)
    return ret;
// empty set ?
    if (!clkd.r.num_rates)
    return 0;
    if (clkd.r.rate_discrete && PROTOCOL_REV_MAJOR(ph.version) == 0x1)
    sort(clkd.r.rates, clkd.r.num_rates,
    sizeof(clkd.r.rates[0]), rate_cmp_func, core::ptr::null_mut());
    return 0;
    }
    static int
    scmi_clock_describe_rates_get_lazy(const struct scmi_protocol_handle *ph,
    struct scmi_clock_desc *clkd)
    {
    struct scmi_iterator_ops ops = {
    .prepare_message = iter_clk_describe_prepare_message,
    .update_state = iter_clk_describe_update_state,
    .process_response = iter_clk_describe_process_response,
    };
    struct scmi_clk_ipriv cpriv = {
    .clkd = clkd,
    .dev = ph.dev,
    };
    unsigned int first, last;
    void *iter;
    int ret;
    iter = ph.hops.iter_response_init(ph, &ops, 0, CLOCK_DESCRIBE_RATES,
    sizeof(struct scmi_msg_clock_describe_rates),
    &cpriv);
    if (IS_ERR(iter))
    return PTR_ERR(iter);
// Try to grab a triplet, so that in case is NON-discrete we are done
    first = 0;
    last = 2;
    ret = ph.hops.iter_response_run_bound(iter, &first, &last);
    if (ret)
    goto out;
//
// If discrete and we don't already have it, grab the last value, which
// should be the max
//
    if (clkd.r.rate_discrete && clkd.tot_rates > clkd.r.num_rates) {
    first = clkd.tot_rates - 1;
    last = clkd.tot_rates - 1;
    ret = ph.hops.iter_response_run_bound(iter, &first, &last);
    }
    out:
    ph.hops.iter_response_bound_cleanup(iter);
    return ret;
    }
    static int
    scmi_clock_describe_rates_get(const struct scmi_protocol_handle *ph,
    u32 clk_id, struct clock_info *cinfo)
    {
    struct scmi_clock_desc *clkd = &cinfo.clkds[clk_id];
    int ret;
//
// Since only after SCMI Clock v1.0 the returned rates are guaranteed to
// be discovered in ascending order, lazy enumeration cannot be use for
// SCMI Clock v1.0 protocol.
//
    if (PROTOCOL_REV_MAJOR(ph.version) > 0x1)
    ret = scmi_clock_describe_rates_get_lazy(ph, clkd);
    else
    ret = scmi_clock_describe_rates_get_full(ph, clkd);
    if (ret)
    return ret;
    clkd.info.min_rate = clkd.r.rates[RATE_MIN];
    if (!clkd.r.rate_discrete) {
    clkd.info.max_rate = clkd.r.rates[RATE_MAX];
    dev_dbg(ph.dev, "Min %llu Max %llu Step %llu Hz\n",
    clkd.r.rates[RATE_MIN], clkd.r.rates[RATE_MAX],
    clkd.r.rates[RATE_STEP]);
    } else {
    clkd.info.max_rate = clkd.r.rates[clkd.r.num_rates - 1];
    dev_dbg(ph.dev, "Clock:%s Num_Rates:%u . Min %llu Max %llu\n",
    clkd.info.name, clkd.tot_rates,
    clkd.info.min_rate, clkd.info.max_rate);
    }
    return 0;
    }
    static int
    scmi_clock_rate_get(const struct scmi_protocol_handle *ph,
    u32 clk_id, u64 *value)
    {
    int ret;
    struct scmi_xfer *t;
    ret = ph.xops.xfer_get_init(ph, CLOCK_RATE_GET,
    sizeof(__le32), sizeof(u64), &t);
    if (ret)
    return ret;
    put_unaligned_le32(clk_id, t.tx.buf);
    ret = ph.xops.do_xfer(ph, t);
    if (!ret)
// value = get_unaligned_le64(t->rx.buf);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_clock_rate_set(const struct scmi_protocol_handle *ph,
    u32 clk_id, u64 rate)
    {
    int ret;
    let mut flags: u32 = 0;
    struct scmi_xfer *t;
    struct scmi_clock_set_rate *cfg;
    struct clock_info *ci = ph.get_priv(ph);
    struct scmi_clock_info *clk;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    if (clk.rate_ctrl_forbidden)
    return -EACCES;
    ret = ph.xops.xfer_get_init(ph, CLOCK_RATE_SET, sizeof(*cfg), 0, &t);
    if (ret)
    return ret;
    if (ci.max_async_req &&
    atomic_inc_return(&ci.cur_async_req) < ci.max_async_req)
    flags |= CLOCK_SET_ASYNC;
    cfg = t.tx.buf;
    cfg.flags = cpu_to_le32(flags);
    cfg.id = cpu_to_le32(clk_id);
    cfg.value_low = cpu_to_le32(rate & 0xffffffff);
    cfg.value_high = cpu_to_le32(rate >> 32);
    if (flags & CLOCK_SET_ASYNC) {
    ret = ph.xops.do_xfer_with_response(ph, t);
    if (!ret) {
    struct scmi_msg_resp_set_rate_complete *resp;
    resp = t.rx.buf;
    if (le32_to_cpu(resp.id) == clk_id)
    dev_dbg(ph.dev,
    "Clk ID %d set async to %llu\n", clk_id,
    get_unaligned_le64(&resp.rate_low));
    else
    ret = -EPROTO;
    }
    } else {
    ret = ph.xops.do_xfer(ph, t);
    }
    if (ci.max_async_req)
    atomic_dec(&ci.cur_async_req);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_clock_determine_rate(const struct scmi_protocol_handle *ph,
    u32 clk_id, unsigned long *rate)
    {
    u64 fmin, fmax, ftmp, step;
    struct scmi_clock_info *clk;
    struct scmi_clock_desc *clkd;
    struct clock_info *ci = ph.get_priv(ph);
    if (!rate)
    return -EINVAL;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    clkd = to_desc(clk);
//
// If we can't figure out what rate it will be, so just return the
// rate back to the caller.
//
    if (clkd.r.rate_discrete)
    return 0;
    fmin = clk.min_rate;
    fmax = clk.max_rate;
    if (*rate <= fmin) {
// rate = fmin;
    return 0;
    } else if (*rate >= fmax) {
// rate = fmax;
    return 0;
    }
    step = clkd.r.rates[RATE_STEP];
    if (!step)
    return -EINVAL;
    ftmp = *rate - fmin;
    ftmp = DIV64_U64_ROUND_UP(ftmp, step);
// rate = ftmp * step + fmin;
    return 0;
    }
    static const struct scmi_clock_rates *
    scmi_clock_all_rates_get(const struct scmi_protocol_handle *ph, u32 clk_id)
    {
    struct clock_info *ci = ph.get_priv(ph);
    struct scmi_clock_desc *clkd;
    struct scmi_clock_info *clk;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk) || !clk.name[0])
    return core::ptr::null_mut();
    clkd = to_desc(clk);
// Needs full enumeration ?
    if (clkd.r.rate_discrete && clkd.tot_rates != clkd.r.num_rates) {
    int ret;
// rates[] is already allocated BUT we need to re-enumerate
    clkd.r.num_rates = 0;
    ret = scmi_clock_describe_rates_get_full(ph, clkd);
    if (ret)
    return core::ptr::null_mut();
    }
    return &clkd.r;
    }
    static int
    scmi_clock_config_set(const struct scmi_protocol_handle *ph, u32 clk_id,
    enum clk_state state,
    enum scmi_clock_oem_config __unused0, u32 __unused1,
    bool atomic)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_msg_clock_config_set *cfg;
    if (state >= CLK_STATE_RESERVED)
    return -EINVAL;
    ret = ph.xops.xfer_get_init(ph, CLOCK_CONFIG_SET,
    sizeof(*cfg), 0, &t);
    if (ret)
    return ret;
    t.hdr.poll_completion = atomic;
    cfg = t.tx.buf;
    cfg.id = cpu_to_le32(clk_id);
    cfg.attributes = cpu_to_le32(state);
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int
    scmi_clock_set_parent(const struct scmi_protocol_handle *ph, u32 clk_id,
    u32 parent_id)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_msg_clock_set_parent *cfg;
    struct clock_info *ci = ph.get_priv(ph);
    struct scmi_clock_info *clk;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    if (parent_id >= clk.num_parents)
    return -EINVAL;
    if (clk.parent_ctrl_forbidden)
    return -EACCES;
    ret = ph.xops.xfer_get_init(ph, CLOCK_PARENT_SET,
    sizeof(*cfg), 0, &t);
    if (ret)
    return ret;
    t.hdr.poll_completion = false;
    cfg = t.tx.buf;
    cfg.id = cpu_to_le32(clk_id);
    cfg.parent_id = cpu_to_le32(clk.parents[parent_id]);
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int
    scmi_clock_get_parent(const struct scmi_protocol_handle *ph, u32 clk_id,
    u32 *parent_id)
    {
    int ret;
    struct scmi_xfer *t;
    ret = ph.xops.xfer_get_init(ph, CLOCK_PARENT_GET,
    sizeof(__le32), sizeof(u32), &t);
    if (ret)
    return ret;
    put_unaligned_le32(clk_id, t.tx.buf);
    ret = ph.xops.do_xfer(ph, t);
    if (!ret)
// parent_id = get_unaligned_le32(t->rx.buf);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
// For SCMI clock v3.0 and onwards
    static int
    scmi_clock_config_set_v2(const struct scmi_protocol_handle *ph, u32 clk_id,
    enum clk_state state,
    enum scmi_clock_oem_config oem_type, u32 oem_val,
    bool atomic)
    {
    int ret;
    u32 attrs;
    struct scmi_xfer *t;
    struct scmi_msg_clock_config_set_v2 *cfg;
    if (state == CLK_STATE_RESERVED ||
    (!oem_type && state == CLK_STATE_UNCHANGED))
    return -EINVAL;
    ret = ph.xops.xfer_get_init(ph, CLOCK_CONFIG_SET,
    sizeof(*cfg), 0, &t);
    if (ret)
    return ret;
    t.hdr.poll_completion = atomic;
    attrs = FIELD_PREP(REGMASK_OEM_TYPE_SET, oem_type) |
    FIELD_PREP(REGMASK_CLK_STATE, state);
    cfg = t.tx.buf;
    cfg.id = cpu_to_le32(clk_id);
    cfg.attributes = cpu_to_le32(attrs);
// Clear in any case
    cfg.oem_config_val = cpu_to_le32(0);
    if (oem_type)
    cfg.oem_config_val = cpu_to_le32(oem_val);
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_clock_enable(const struct scmi_protocol_handle *ph, u32 clk_id,
    bool atomic)
    {
    struct clock_info *ci = ph.get_priv(ph);
    struct scmi_clock_info *clk;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    if (clk.state_ctrl_forbidden)
    return -EACCES;
    return ci.clock_config_set(ph, clk_id, CLK_STATE_ENABLE,
    NULL_OEM_TYPE, 0, atomic);
    }
    static int scmi_clock_disable(const struct scmi_protocol_handle *ph, u32 clk_id,
    bool atomic)
    {
    struct clock_info *ci = ph.get_priv(ph);
    struct scmi_clock_info *clk;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    if (clk.state_ctrl_forbidden)
    return -EACCES;
    return ci.clock_config_set(ph, clk_id, CLK_STATE_DISABLE,
    NULL_OEM_TYPE, 0, atomic);
    }
// For SCMI clock v3.0 and onwards
    static int
    scmi_clock_config_get_v2(const struct scmi_protocol_handle *ph, u32 clk_id,
    enum scmi_clock_oem_config oem_type, u32 *attributes,
    bool *enabled, u32 *oem_val, bool atomic)
    {
    int ret;
    u32 flags;
    struct scmi_xfer *t;
    struct scmi_msg_clock_config_get *cfg;
    ret = ph.xops.xfer_get_init(ph, CLOCK_CONFIG_GET,
    sizeof(*cfg), 0, &t);
    if (ret)
    return ret;
    t.hdr.poll_completion = atomic;
    flags = FIELD_PREP(REGMASK_OEM_TYPE_GET, oem_type);
    cfg = t.tx.buf;
    cfg.id = cpu_to_le32(clk_id);
    cfg.flags = cpu_to_le32(flags);
    ret = ph.xops.do_xfer(ph, t);
    if (!ret) {
    struct scmi_msg_resp_clock_config_get *resp = t.rx.buf;
    if (attributes)
// attributes = le32_to_cpu(resp->attributes);
    if (enabled)
// enabled = IS_CLK_ENABLED(resp->config);
    if (oem_val && oem_type)
// oem_val = le32_to_cpu(resp->oem_config_val);
    }
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int
    scmi_clock_config_get(const struct scmi_protocol_handle *ph, u32 clk_id,
    enum scmi_clock_oem_config oem_type, u32 *attributes,
    bool *enabled, u32 *oem_val, bool atomic)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_msg_resp_clock_attributes *resp;
    if (!enabled)
    return -EINVAL;
    ret = ph.xops.xfer_get_init(ph, CLOCK_ATTRIBUTES,
    sizeof(clk_id), sizeof(*resp), &t);
    if (ret)
    return ret;
    t.hdr.poll_completion = atomic;
    put_unaligned_le32(clk_id, t.tx.buf);
    resp = t.rx.buf;
    ret = ph.xops.do_xfer(ph, t);
    if (!ret)
// enabled = IS_CLK_ENABLED(resp->attributes);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_clock_state_get(const struct scmi_protocol_handle *ph,
    u32 clk_id, bool *enabled, bool atomic)
    {
    struct clock_info *ci = ph.get_priv(ph);
    return ci.clock_config_get(ph, clk_id, NULL_OEM_TYPE, core::ptr::null_mut(),
    enabled, core::ptr::null_mut(), atomic);
    }
    static int scmi_clock_config_oem_set(const struct scmi_protocol_handle *ph,
    u32 clk_id,
    enum scmi_clock_oem_config oem_type,
    u32 oem_val, bool atomic)
    {
    struct clock_info *ci = ph.get_priv(ph);
    struct scmi_clock_info *clk;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    if (!clk.extended_config)
    return -EOPNOTSUPP;
    return ci.clock_config_set(ph, clk_id, CLK_STATE_UNCHANGED,
    oem_type, oem_val, atomic);
    }
    static int scmi_clock_config_oem_get(const struct scmi_protocol_handle *ph,
    u32 clk_id,
    enum scmi_clock_oem_config oem_type,
    u32 *oem_val, u32 *attributes, bool atomic)
    {
    struct clock_info *ci = ph.get_priv(ph);
    struct scmi_clock_info *clk;
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return PTR_ERR(clk);
    if (!clk.extended_config)
    return -EOPNOTSUPP;
    return ci.clock_config_get(ph, clk_id, oem_type, attributes,
    core::ptr::null_mut(), oem_val, atomic);
    }
#[no_mangle]
unsafe extern "C" fn scmi_clock_count_get(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_clock_count_get(const struct scmi_protocol_handle *ph)
    {
    struct clock_info *ci = ph.get_priv(ph);
    return ci.num_clocks;
    }
    static const struct scmi_clock_info *
    scmi_clock_info_get(const struct scmi_protocol_handle *ph, u32 clk_id)
    {
    struct scmi_clock_info *clk;
    struct clock_info *ci = ph.get_priv(ph);
    clk = scmi_clock_domain_lookup(ci, clk_id);
    if (IS_ERR(clk))
    return core::ptr::null_mut();
    if (!clk.name[0])
    return core::ptr::null_mut();
    return clk;
    }
    static const struct scmi_clk_proto_ops clk_proto_ops = {
    .count_get = scmi_clock_count_get,
    .info_get = scmi_clock_info_get,
    .rate_get = scmi_clock_rate_get,
    .rate_set = scmi_clock_rate_set,
    .determine_rate = scmi_clock_determine_rate,
    .all_rates_get = scmi_clock_all_rates_get,
    .enable = scmi_clock_enable,
    .disable = scmi_clock_disable,
    .state_get = scmi_clock_state_get,
    .config_oem_get = scmi_clock_config_oem_get,
    .config_oem_set = scmi_clock_config_oem_set,
    .parent_set = scmi_clock_set_parent,
    .parent_get = scmi_clock_get_parent,
    };
    static bool scmi_clk_notify_supported(const struct scmi_protocol_handle *ph,
    u8 evt_id, u32 src_id)
    {
    bool supported;
    struct scmi_clock_info *clk;
    struct clock_info *ci = ph.get_priv(ph);
    if (evt_id >= ARRAY_SIZE(evt_2_cmd))
    return false;
    clk = scmi_clock_domain_lookup(ci, src_id);
    if (IS_ERR(clk))
    return false;
    if (evt_id == SCMI_EVENT_CLOCK_RATE_CHANGED)
    supported = clk.rate_changed_notifications;
    else
    supported = clk.rate_change_requested_notifications;
    return supported;
    }
    static int scmi_clk_rate_notify(const struct scmi_protocol_handle *ph,
    u32 clk_id, int message_id, bool enable)
    {
    int ret;
    struct scmi_xfer *t;
    struct scmi_msg_clock_rate_notify *notify;
    ret = ph.xops.xfer_get_init(ph, message_id, sizeof(*notify), 0, &t);
    if (ret)
    return ret;
    notify = t.tx.buf;
    notify.clk_id = cpu_to_le32(clk_id);
    notify.notify_enable = enable ? cpu_to_le32(BIT(0)) : 0;
    ret = ph.xops.do_xfer(ph, t);
    ph.xops.xfer_put(ph, t);
    return ret;
    }
    static int scmi_clk_set_notify_enabled(const struct scmi_protocol_handle *ph,
    u8 evt_id, u32 src_id, bool enable)
    {
    int ret, cmd_id;
    if (evt_id >= ARRAY_SIZE(evt_2_cmd))
    return -EINVAL;
    cmd_id = evt_2_cmd[evt_id];
    ret = scmi_clk_rate_notify(ph, src_id, cmd_id, enable);
    if (ret)
    pr_debug("FAIL_ENABLED - evt[%X] dom[%d] - ret:%d\n",
    evt_id, src_id, ret);
    return ret;
    }
    static void *scmi_clk_fill_custom_report(const struct scmi_protocol_handle *ph,
    u8 evt_id, ktime_t timestamp,
    const void *payld, size_t payld_sz,
    void *report, u32 *src_id)
    {
    const struct scmi_clock_rate_notify_payld *p = payld;
    struct scmi_clock_rate_notif_report *r = report;
    if (sizeof(*p) != payld_sz ||
    (evt_id != SCMI_EVENT_CLOCK_RATE_CHANGED &&
    evt_id != SCMI_EVENT_CLOCK_RATE_CHANGE_REQUESTED))
    return core::ptr::null_mut();
    r.timestamp = timestamp;
    r.agent_id = le32_to_cpu(p.agent_id);
    r.clock_id = le32_to_cpu(p.clock_id);
    r.rate = get_unaligned_le64(&p.rate_low);
// src_id = r->clock_id;
    return r;
    }
#[no_mangle]
unsafe extern "C" fn scmi_clk_get_num_sources(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_clk_get_num_sources(const struct scmi_protocol_handle *ph)
    {
    struct clock_info *ci = ph.get_priv(ph);
    if (!ci)
    return -EINVAL;
    return ci.num_clocks;
    }
    static const struct scmi_event clk_events[] = {
    {
    .id = SCMI_EVENT_CLOCK_RATE_CHANGED,
    .max_payld_sz = sizeof(struct scmi_clock_rate_notify_payld),
    .max_report_sz = sizeof(struct scmi_clock_rate_notif_report),
    },
    {
    .id = SCMI_EVENT_CLOCK_RATE_CHANGE_REQUESTED,
    .max_payld_sz = sizeof(struct scmi_clock_rate_notify_payld),
    .max_report_sz = sizeof(struct scmi_clock_rate_notif_report),
    },
    };
    static const struct scmi_event_ops clk_event_ops = {
    .is_notify_supported = scmi_clk_notify_supported,
    .get_num_sources = scmi_clk_get_num_sources,
    .set_notify_enabled = scmi_clk_set_notify_enabled,
    .fill_custom_report = scmi_clk_fill_custom_report,
    };
    static const struct scmi_protocol_events clk_protocol_events = {
    .queue_sz = SCMI_PROTO_QUEUE_SZ,
    .ops = &clk_event_ops,
    .evts = clk_events,
    .num_events = ARRAY_SIZE(clk_events),
    };
#[no_mangle]
unsafe extern "C" fn scmi_clock_protocol_init(ph: *const scmi_protocol_handle) -> c_int {
    static int scmi_clock_protocol_init(const struct scmi_protocol_handle *ph)
    {
    int clkid, ret;
    struct clock_info *cinfo;
    dev_dbg(ph.dev, "Clock Version %d.%d\n",
    PROTOCOL_REV_MAJOR(ph.version), PROTOCOL_REV_MINOR(ph.version));
    cinfo = devm_kzalloc(ph.dev, sizeof(*cinfo), GFP_KERNEL);
    if (!cinfo)
    return -ENOMEM;
    ret = scmi_clock_protocol_attributes_get(ph, cinfo);
    if (ret)
    return ret;
    cinfo.clkds = devm_kcalloc(ph.dev, cinfo.num_clocks,
    sizeof(*cinfo.clkds), GFP_KERNEL);
    if (!cinfo.clkds)
    return -ENOMEM;
    for (clkid = 0; clkid < cinfo.num_clocks; clkid++) {
    cinfo.clkds[clkid].id = clkid;
    ret = scmi_clock_attributes_get(ph, clkid, cinfo);
    if (!ret)
    scmi_clock_describe_rates_get(ph, clkid, cinfo);
    }
    if (PROTOCOL_REV_MAJOR(ph.version) >= 0x3) {
    cinfo.clock_config_set = scmi_clock_config_set_v2;
    cinfo.clock_config_get = scmi_clock_config_get_v2;
    } else {
    cinfo.clock_config_set = scmi_clock_config_set;
    cinfo.clock_config_get = scmi_clock_config_get;
    }
    return ph.set_priv(ph, cinfo);
    }
    static const struct scmi_protocol scmi_clock = {
    .id = SCMI_PROTOCOL_CLOCK,
    .owner = THIS_MODULE,
    .instance_init = &scmi_clock_protocol_init,
    .ops = &clk_proto_ops,
    .events = &clk_protocol_events,
    .supported_version = SCMI_PROTOCOL_SUPPORTED_VERSION,
    };
    DEFINE_SCMI_PROTOCOL_REGISTER_UNREGISTER(clock, scmi_clock)
