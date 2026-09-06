//! Automatically rewritten from C to Rust
//! Source: drivers/hwtracing/stm/p_sys-t.c
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
// MIPI SyS-T framing protocol for STM devices.
// Copyright (c) 2018, Intel Corporation.
//

    enum sys_t_message_type {
    MIPI_SYST_TYPE_BUILD	= 0,
    MIPI_SYST_TYPE_SHORT32,
    MIPI_SYST_TYPE_STRING,
    MIPI_SYST_TYPE_CATALOG,
    MIPI_SYST_TYPE_RAW	= 6,
    MIPI_SYST_TYPE_SHORT64,
    MIPI_SYST_TYPE_CLOCK,
    MIPI_SYST_TYPE_SBD,
    };
    enum sys_t_message_severity {
    MIPI_SYST_SEVERITY_MAX	= 0,
    MIPI_SYST_SEVERITY_FATAL,
    MIPI_SYST_SEVERITY_ERROR,
    MIPI_SYST_SEVERITY_WARNING,
    MIPI_SYST_SEVERITY_INFO,
    MIPI_SYST_SEVERITY_USER1,
    MIPI_SYST_SEVERITY_USER2,
    MIPI_SYST_SEVERITY_DEBUG,
    };
    enum sys_t_message_build_subtype {
    MIPI_SYST_BUILD_ID_COMPACT32 = 0,
    MIPI_SYST_BUILD_ID_COMPACT64,
    MIPI_SYST_BUILD_ID_LONG,
    };
    enum sys_t_message_clock_subtype {
    MIPI_SYST_CLOCK_TRANSPORT_SYNC = 1,
    };
    enum sys_t_message_string_subtype {
    MIPI_SYST_STRING_GENERIC	= 1,
    MIPI_SYST_STRING_FUNCTIONENTER,
    MIPI_SYST_STRING_FUNCTIONEXIT,
    MIPI_SYST_STRING_INVALIDPARAM	= 5,
    MIPI_SYST_STRING_ASSERT		= 7,
    MIPI_SYST_STRING_PRINTF_32	= 11,
    MIPI_SYST_STRING_PRINTF_64	= 12,
    };
//
// enum sys_t_message_sbd_subtype - SyS-T SBD message subtypes
// @MIPI_SYST_SBD_ID32: SBD message with 32-bit message ID
// @MIPI_SYST_SBD_ID64: SBD message with 64-bit message ID
//
// Structured Binary Data messages can send information of arbitrary length,
// together with ID's that describe BLOB's content and layout.
//
    enum sys_t_message_sbd_subtype {
    MIPI_SYST_SBD_ID32 = 0,
    MIPI_SYST_SBD_ID64 = 1,
    };

    MIPI_SYST_ORIGIN(u >> 4))

    MIPI_SYST_SUBTYPE(t ## _ ## s))

    MIPI_SYST_SEVERITY(INFO)		| \
    MIPI_SYST_OPT_GUID)

    MIPI_SYST_SEVERITY(MAX))
//
// SyS-T and ftrace headers are compatible to an extent that ftrace event ID
// and args can be treated as SyS-T SBD message with 64-bit ID and arguments
// BLOB right behind the header without modification. Bits [16:63] coming
// together with message ID from ftrace aren't used by SBD and must be zeroed.
//
// 0       15  16   23  24     31  32   39  40  63
// ftrace: <event_id>  <flags>  <preempt>  <-pid->  <----> <args>
// SBD:    <------- msg_id ------------------------------> <BLOB>
//

    MIPI_SYST_SEVERITY(INFO)		| \
    MIPI_SYST_OPT_GUID)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_t_policy_node {
    pub uuid: uuid_t,
    pub do_len: bool,
    pub ts_interval: c_ulong,
    pub clocksync_interval: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sys_t_output {
    pub node: sys_t_policy_node,
    pub ts_jiffies: c_ulong,
    pub clocksync_jiffies: c_ulong,
}

#[no_mangle]
unsafe extern "C" fn sys_t_policy_node_init(priv: *mut c_void) {
    static void sys_t_policy_node_init(void *priv)
    {
    struct sys_t_policy_node *pn = priv;
    uuid_gen(&pn.uuid);
    }
#[no_mangle]
unsafe extern "C" fn sys_t_output_open(priv: *mut c_void, output: *mut stm_output) -> c_int {
    static int sys_t_output_open(void *priv, struct stm_output *output)
    {
    struct sys_t_policy_node *pn = priv;
    struct sys_t_output *opriv;
    opriv = kzalloc_obj(*opriv, GFP_ATOMIC);
    if (!opriv)
    return -ENOMEM;
    memcpy(&opriv.node, pn, sizeof(opriv.node));
    output.pdrv_private = opriv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn sys_t_output_close(output: *mut stm_output) {
    static void sys_t_output_close(struct stm_output *output)
    {
    kfree(output.pdrv_private);
    }
    static ssize_t sys_t_policy_uuid_show(struct config_item *item,
    char *page)
    {
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    return sprintf(page, "%pU\n", &pn.uuid);
    }
    static ssize_t
    sys_t_policy_uuid_store(struct config_item *item, const char *page,
    size_t count)
    {
    struct mutex *mutexp = &item.ci_group.cg_subsys.su_mutex;
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    int ret;
    mutex_lock(mutexp);
    ret = uuid_parse(page, &pn.uuid);
    mutex_unlock(mutexp);
    return ret < 0 ? ret : count;
    }
    CONFIGFS_ATTR(sys_t_policy_, uuid);
    static ssize_t sys_t_policy_do_len_show(struct config_item *item,
    char *page)
    {
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    return sprintf(page, "%d\n", pn.do_len);
    }
    static ssize_t
    sys_t_policy_do_len_store(struct config_item *item, const char *page,
    size_t count)
    {
    struct mutex *mutexp = &item.ci_group.cg_subsys.su_mutex;
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    int ret;
    mutex_lock(mutexp);
    ret = kstrtobool(page, &pn.do_len);
    mutex_unlock(mutexp);
    return ret ? ret : count;
    }
    CONFIGFS_ATTR(sys_t_policy_, do_len);
    static ssize_t sys_t_policy_ts_interval_show(struct config_item *item,
    char *page)
    {
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    return sprintf(page, "%u\n", jiffies_to_msecs(pn.ts_interval));
    }
    static ssize_t
    sys_t_policy_ts_interval_store(struct config_item *item, const char *page,
    size_t count)
    {
    struct mutex *mutexp = &item.ci_group.cg_subsys.su_mutex;
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    unsigned int ms;
    int ret;
    mutex_lock(mutexp);
    ret = kstrtouint(page, 10, &ms);
    mutex_unlock(mutexp);
    if (!ret) {
    pn.ts_interval = msecs_to_jiffies(ms);
    return count;
    }
    return ret;
    }
    CONFIGFS_ATTR(sys_t_policy_, ts_interval);
    static ssize_t sys_t_policy_clocksync_interval_show(struct config_item *item,
    char *page)
    {
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    return sprintf(page, "%u\n", jiffies_to_msecs(pn.clocksync_interval));
    }
    static ssize_t
    sys_t_policy_clocksync_interval_store(struct config_item *item,
    const char *page, size_t count)
    {
    struct mutex *mutexp = &item.ci_group.cg_subsys.su_mutex;
    struct sys_t_policy_node *pn = to_pdrv_policy_node(item);
    unsigned int ms;
    int ret;
    mutex_lock(mutexp);
    ret = kstrtouint(page, 10, &ms);
    mutex_unlock(mutexp);
    if (!ret) {
    pn.clocksync_interval = msecs_to_jiffies(ms);
    return count;
    }
    return ret;
    }
    CONFIGFS_ATTR(sys_t_policy_, clocksync_interval);
    static struct configfs_attribute *sys_t_policy_attrs[] = {
    &sys_t_policy_attr_uuid,
    &sys_t_policy_attr_do_len,
    &sys_t_policy_attr_ts_interval,
    &sys_t_policy_attr_clocksync_interval,
    core::ptr::null_mut(),
    };
#[no_mangle]
pub unsafe extern "C" fn sys_t_need_ts(op: *mut sys_t_output) -> bool {
    static inline bool sys_t_need_ts(struct sys_t_output *op)
    {
    if (op.node.ts_interval &&
    time_after(jiffies, op.ts_jiffies + op.node.ts_interval)) {
    op.ts_jiffies = jiffies;
    return true;
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn sys_t_need_clock_sync(op: *mut sys_t_output) -> bool {
    static bool sys_t_need_clock_sync(struct sys_t_output *op)
    {
    if (op.node.clocksync_interval &&
    time_after(jiffies,
    op.clocksync_jiffies + op.node.clocksync_interval)) {
    op.clocksync_jiffies = jiffies;
    return true;
    }
    return false;
    }
    static ssize_t
    sys_t_clock_sync(struct stm_data *data, unsigned int m, unsigned int c)
    {
    let mut header: u32 = CLOCK_SYNC_HEADER;
    let mut nil: c_uchar = 0;
    u64 payload[2]; /* Clock value and frequency */
    ssize_t sz;
    sz = data.packet(data, m, c, STP_PACKET_DATA, STP_PACKET_TIMESTAMPED,
    4, (u8 *)&header);
    if (sz <= 0)
    return sz;
    payload[0] = ktime_get_real_ns();
    payload[1] = NSEC_PER_SEC;
    sz = stm_data_write(data, m, c, false, &payload, sizeof(payload));
    if (sz <= 0)
    return sz;
    data.packet(data, m, c, STP_PACKET_FLAG, 0, 0, &nil);
    return sizeof(header) + sizeof(payload);
    }
#[no_mangle]
pub unsafe extern "C" fn sys_t_header(source: *mut stm_source_data) -> u32 {
    static inline u32 sys_t_header(struct stm_source_data *source)
    {
    if (source && source.type == STM_FTRACE)
    return SBD_HEADER;
    return DATA_HEADER;
    }
    static ssize_t sys_t_write_data(struct stm_data *data,
    struct stm_source_data *source,
    unsigned int master, unsigned int channel,
    bool ts_first, const void *buf, size_t count)
    {
    ssize_t sz;
    let mut nil: c_uchar = 0;
//
// Ftrace is zero-copy compatible with SyS-T SBD, but requires
// special handling of first 64 bits. Trim and send them separately
// to avoid damage on original ftrace buffer.
//
    if (source && source.type == STM_FTRACE) {
    u64 compat_ftrace_header;
    ssize_t header_sz;
    ssize_t buf_sz;
    if (count < sizeof(compat_ftrace_header))
    return -EINVAL;
// SBD only makes use of low 16 bits (event ID) from ftrace event
    compat_ftrace_header = *(u64 *)buf & 0xffff;
    header_sz = stm_data_write(data, master, channel, false,
    &compat_ftrace_header,
    sizeof(compat_ftrace_header));
    if (header_sz != sizeof(compat_ftrace_header))
    return header_sz;
    buf_sz = stm_data_write(data, master, channel, false,
    buf + header_sz, count - header_sz);
    if (buf_sz != count - header_sz)
    return buf_sz;
    sz = header_sz + buf_sz;
    } else {
    sz = stm_data_write(data, master, channel, false, buf, count);
    }
    if (sz <= 0)
    return sz;
    data.packet(data, master, channel, STP_PACKET_FLAG, 0, 0, &nil);
    return sz;
    }
    static ssize_t sys_t_write(struct stm_data *data, struct stm_output *output,
    unsigned int chan, const char *buf, size_t count,
    struct stm_source_data *source)
    {
    struct sys_t_output *op = output.pdrv_private;
    let mut c: c_uint = output.channel + chan;
    let mut m: c_uint = output.master;
    let mut header: u32 = sys_t_header(source);
    u8 uuid[UUID_SIZE];
    ssize_t sz;
// We require an existing policy node to proceed
    if (!op)
    return -EINVAL;
    if (sys_t_need_clock_sync(op)) {
    sz = sys_t_clock_sync(data, m, c);
    if (sz <= 0)
    return sz;
    }
    if (op.node.do_len)
    header |= MIPI_SYST_OPT_LEN;
    if (sys_t_need_ts(op))
    header |= MIPI_SYST_OPT_TS;
//
// STP framing rules for SyS-T frames:
// * the first packet of the SyS-T frame is timestamped;
// * the last packet is a FLAG.
//
// Message layout: HEADER / GUID / [LENGTH /][TIMESTAMP /] DATA
// HEADER
    sz = data.packet(data, m, c, STP_PACKET_DATA, STP_PACKET_TIMESTAMPED,
    4, (u8 *)&header);
    if (sz <= 0)
    return sz;
// GUID
    export_uuid(uuid, &op.node.uuid);
    sz = stm_data_write(data, m, c, false, uuid, sizeof(op.node.uuid));
    if (sz <= 0)
    return sz;
// [LENGTH]
    if (op.node.do_len) {
    let mut length: u16 = count;
    sz = data.packet(data, m, c, STP_PACKET_DATA, 0, 2,
    (u8 *)&length);
    if (sz <= 0)
    return sz;
    }
// [TIMESTAMP]
    if (header & MIPI_SYST_OPT_TS) {
    let mut ts: u64 = ktime_get_real_ns();
    sz = stm_data_write(data, m, c, false, &ts, sizeof(ts));
    if (sz <= 0)
    return sz;
    }
// DATA
    return sys_t_write_data(data, source, m, c, false, buf, count);
    }
    static const struct stm_protocol_driver sys_t_pdrv = {
    .owner			= THIS_MODULE,
    .name			= "p_sys-t",
    .priv_sz		= sizeof(struct sys_t_policy_node),
    .write			= sys_t_write,
    .policy_attr		= sys_t_policy_attrs,
    .policy_node_init	= sys_t_policy_node_init,
    .output_open		= sys_t_output_open,
    .output_close		= sys_t_output_close,
    };
#[no_mangle]
unsafe extern "C" fn sys_t_stm_init() -> c_int {
    static int sys_t_stm_init(void)
    {
    return stm_register_protocol(&sys_t_pdrv);
    }
#[no_mangle]
unsafe extern "C" fn sys_t_stm_exit() {
    static void sys_t_stm_exit(void)
    {
    stm_unregister_protocol(&sys_t_pdrv);
    }
    module_init(sys_t_stm_init);
    module_exit(sys_t_stm_exit);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("MIPI SyS-T STM framing protocol driver");
    MODULE_AUTHOR("Alexander Shishkin <alexander.shishkin@linux.intel.com>");
