//! Automatically rewritten from C to Rust
//! Source: drivers/platform/surface/surface_aggregator_tabletsw.c
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
//
// Surface System Aggregator Module (SSAM) tablet mode switch driver.
//
// Copyright (C) 2022 Maximilian Luz <luzmaximilian@gmail.com>
//

// -- SSAM generic tablet switch driver framework. --------------------------
    struct ssam_tablet_sw;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_tablet_sw_state {
    pub source: u32,
    pub state: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_tablet_sw_ops {
    pub state): *mut *mut *mut int (get_state)(struct ssam_tablet_sw sw, struct ssam_tablet_sw_state,
    const char *(*state_name)(struct ssam_tablet_sw *sw,
    pub state): *const ssam_tablet_sw_state,
    bool (*state_is_tablet_mode)(struct ssam_tablet_sw *sw,
    pub state): *const ssam_tablet_sw_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_tablet_sw {
    pub sdev: *mut ssam_device,
    pub state: ssam_tablet_sw_state,
    pub update_work: work_struct,
    pub mode_switch: *mut input_dev,
    pub ops: ssam_tablet_sw_ops,
    pub notif: ssam_event_notifier,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_tablet_sw_desc {
    struct {
    pub name: *const c_char,
    pub phys: *const c_char,
    pub dev: },
    struct {
    pub event): *const *const *const u32 (notify)(struct ssam_event_notifier nf, struct ssam_event,
    pub state): *mut *mut *mut int (get_state)(struct ssam_tablet_sw sw, struct ssam_tablet_sw_state,
    const char *(*state_name)(struct ssam_tablet_sw *sw,
    pub state): *const ssam_tablet_sw_state,
    bool (*state_is_tablet_mode)(struct ssam_tablet_sw *sw,
    pub state): *const ssam_tablet_sw_state,
    pub ops: },
    struct {
    pub reg: ssam_event_registry,
    pub id: ssam_event_id,
    pub mask: enum ssam_event_mask,
    pub flags: u8,
    pub event: },
}

#[no_mangle]
unsafe extern "C" fn state_show(dev: *mut device, attr: *mut device_attribute, buf: *mut c_char) -> isize {
    static ssize_t state_show(struct device *dev, struct device_attribute *attr, char *buf)
    {
    struct ssam_tablet_sw *sw = dev_get_drvdata(dev);
    const char *state = sw.ops.state_name(sw, &sw.state);
    return sysfs_emit(buf, "%s\n", state);
    }
    static DEVICE_ATTR_RO(state);
    static struct attribute *ssam_tablet_sw_attrs[] = {
    &dev_attr_state.attr,
    core::ptr::null_mut(),
    };
    static const struct attribute_group ssam_tablet_sw_group = {
    .attrs = ssam_tablet_sw_attrs,
    };
#[no_mangle]
unsafe extern "C" fn ssam_tablet_sw_update_workfn(work: *mut work_struct) {
    static void ssam_tablet_sw_update_workfn(struct work_struct *work)
    {
    struct ssam_tablet_sw *sw = container_of(work, struct ssam_tablet_sw, update_work);
    struct ssam_tablet_sw_state state;
    int tablet, status;
    status = sw.ops.get_state(sw, &state);
    if (status)
    return;
    if (sw.state.source == state.source && sw.state.state == state.state)
    return;
    sw.state = state;
// Send SW_TABLET_MODE event.
    tablet = sw.ops.state_is_tablet_mode(sw, &state);
    input_report_switch(sw.mode_switch, SW_TABLET_MODE, tablet);
    input_sync(sw.mode_switch);
    }
#[no_mangle]
unsafe extern "C" fn ssam_tablet_sw_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused ssam_tablet_sw_resume(struct device *dev)
    {
    struct ssam_tablet_sw *sw = dev_get_drvdata(dev);
    schedule_work(&sw.update_work);
    return 0;
    }
    static SIMPLE_DEV_PM_OPS(ssam_tablet_sw_pm_ops, core::ptr::null_mut(), ssam_tablet_sw_resume);
#[no_mangle]
unsafe extern "C" fn ssam_tablet_sw_probe(sdev: *mut ssam_device) -> c_int {
    static int ssam_tablet_sw_probe(struct ssam_device *sdev)
    {
    const struct ssam_tablet_sw_desc *desc;
    struct ssam_tablet_sw *sw;
    int tablet, status;
    desc = ssam_device_get_match_data(sdev);
    if (!desc) {
    WARN(1, "no driver match data specified");
    return -EINVAL;
    }
    sw = devm_kzalloc(&sdev.dev, sizeof(*sw), GFP_KERNEL);
    if (!sw)
    return -ENOMEM;
    sw.sdev = sdev;
    sw.ops.get_state = desc.ops.get_state;
    sw.ops.state_name = desc.ops.state_name;
    sw.ops.state_is_tablet_mode = desc.ops.state_is_tablet_mode;
    INIT_WORK(&sw.update_work, ssam_tablet_sw_update_workfn);
    ssam_device_set_drvdata(sdev, sw);
// Get initial state.
    status = sw.ops.get_state(sw, &sw.state);
    if (status)
    return status;
// Set up tablet mode switch.
    sw.mode_switch = devm_input_allocate_device(&sdev.dev);
    if (!sw.mode_switch)
    return -ENOMEM;
    sw.mode_switch.name = desc.dev.name;
    sw.mode_switch.phys = desc.dev.phys;
    sw.mode_switch.id.bustype = BUS_HOST;
    sw.mode_switch.dev.parent = &sdev.dev;
    tablet = sw.ops.state_is_tablet_mode(sw, &sw.state);
    input_set_capability(sw.mode_switch, EV_SW, SW_TABLET_MODE);
    input_report_switch(sw.mode_switch, SW_TABLET_MODE, tablet);
    status = input_register_device(sw.mode_switch);
    if (status)
    return status;
// Set up notifier.
    sw.notif.base.priority = 0;
    sw.notif.base.fn = desc.ops.notify;
    sw.notif.event.reg = desc.event.reg;
    sw.notif.event.id = desc.event.id;
    sw.notif.event.mask = desc.event.mask;
    sw.notif.event.flags = SSAM_EVENT_SEQUENCED;
    status = ssam_device_notifier_register(sdev, &sw.notif);
    if (status)
    return status;
    status = sysfs_create_group(&sdev.dev.kobj, &ssam_tablet_sw_group);
    if (status)
    goto err;
// We might have missed events during setup, so check again.
    schedule_work(&sw.update_work);
    return 0;
    err:
    ssam_device_notifier_unregister(sdev, &sw.notif);
    cancel_work_sync(&sw.update_work);
    return status;
    }
#[no_mangle]
unsafe extern "C" fn ssam_tablet_sw_remove(sdev: *mut ssam_device) {
    static void ssam_tablet_sw_remove(struct ssam_device *sdev)
    {
    struct ssam_tablet_sw *sw = ssam_device_get_drvdata(sdev);
    sysfs_remove_group(&sdev.dev.kobj, &ssam_tablet_sw_group);
    ssam_device_notifier_unregister(sdev, &sw.notif);
    cancel_work_sync(&sw.update_work);
    }
// -- SSAM KIP tablet switch implementation. --------------------------------
pub const SSAM_EVENT_KIP_CID_COVER_STATE_CHANGED: c_uint = 0x1d;
    enum ssam_kip_cover_state {
    SSAM_KIP_COVER_STATE_DISCONNECTED  = 0x01,
    SSAM_KIP_COVER_STATE_CLOSED        = 0x02,
    SSAM_KIP_COVER_STATE_LAPTOP        = 0x03,
    SSAM_KIP_COVER_STATE_FOLDED_CANVAS = 0x04,
    SSAM_KIP_COVER_STATE_FOLDED_BACK   = 0x05,
    SSAM_KIP_COVER_STATE_BOOK          = 0x06,
    };
    static const char *ssam_kip_cover_state_name(struct ssam_tablet_sw *sw,
    const struct ssam_tablet_sw_state *state)
    {
    switch (state.state) {
    case SSAM_KIP_COVER_STATE_DISCONNECTED:
    return "disconnected";
    case SSAM_KIP_COVER_STATE_CLOSED:
    return "closed";
    case SSAM_KIP_COVER_STATE_LAPTOP:
    return "laptop";
    case SSAM_KIP_COVER_STATE_FOLDED_CANVAS:
    return "folded-canvas";
    case SSAM_KIP_COVER_STATE_FOLDED_BACK:
    return "folded-back";
    case SSAM_KIP_COVER_STATE_BOOK:
    return "book";
    default:
    dev_warn(&sw.sdev.dev, "unknown KIP cover state: %u\n", state.state);
    return "<unknown>";
    }
    }
    static bool ssam_kip_cover_state_is_tablet_mode(struct ssam_tablet_sw *sw,
    const struct ssam_tablet_sw_state *state)
    {
    switch (state.state) {
    case SSAM_KIP_COVER_STATE_DISCONNECTED:
    case SSAM_KIP_COVER_STATE_FOLDED_CANVAS:
    case SSAM_KIP_COVER_STATE_FOLDED_BACK:
    case SSAM_KIP_COVER_STATE_BOOK:
    return true;
    case SSAM_KIP_COVER_STATE_CLOSED:
    case SSAM_KIP_COVER_STATE_LAPTOP:
    return false;
    default:
    dev_warn(&sw.sdev.dev, "unknown KIP cover state: %d\n", state.state);
    return true;
    }
    }
    SSAM_DEFINE_SYNC_REQUEST_R(__ssam_kip_get_cover_state, u8, {
    .target_category = SSAM_SSH_TC_KIP,
    .target_id       = SSAM_SSH_TID_SAM,
    .command_id      = 0x1d,
    .instance_id     = 0x00,
    });
#[no_mangle]
unsafe extern "C" fn ssam_kip_get_cover_state(sw: *mut ssam_tablet_sw, state: *mut ssam_tablet_sw_state) -> c_int {
    static int ssam_kip_get_cover_state(struct ssam_tablet_sw *sw, struct ssam_tablet_sw_state *state)
    {
    int status;
    u8 raw;
    status = ssam_retry(__ssam_kip_get_cover_state, sw.sdev.ctrl, &raw);
    if (status < 0) {
    dev_err(&sw.sdev.dev, "failed to query KIP lid state: %d\n", status);
    return status;
    }
    state.source = 0;	/* Unused for KIP switch. */
    state.state = raw;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ssam_kip_sw_notif(nf: *mut ssam_event_notifier, event: *const ssam_event) -> u32 {
    static u32 ssam_kip_sw_notif(struct ssam_event_notifier *nf, const struct ssam_event *event)
    {
    struct ssam_tablet_sw *sw = container_of(nf, struct ssam_tablet_sw, notif);
    if (event.command_id != SSAM_EVENT_KIP_CID_COVER_STATE_CHANGED)
    return 0;	/* Return "unhandled". */
    if (event.length < 1)
    dev_warn(&sw.sdev.dev, "unexpected payload size: %u\n", event.length);
    schedule_work(&sw.update_work);
    return SSAM_NOTIF_HANDLED;
    }
    static const struct ssam_tablet_sw_desc ssam_kip_sw_desc = {
    .dev = {
    .name = "Microsoft Surface KIP Tablet Mode Switch",
    .phys = "ssam/01:0e:01:00:01/input0",
    },
    .ops = {
    .notify = ssam_kip_sw_notif,
    .get_state = ssam_kip_get_cover_state,
    .state_name = ssam_kip_cover_state_name,
    .state_is_tablet_mode = ssam_kip_cover_state_is_tablet_mode,
    },
    .event = {
    .reg = SSAM_EVENT_REGISTRY_SAM,
    .id = {
    .target_category = SSAM_SSH_TC_KIP,
    .instance = 0,
    },
    .mask = SSAM_EVENT_MASK_TARGET,
    },
    };
// -- SSAM POS tablet switch implementation. --------------------------------
    let mut tablet_mode_in_slate_state: static bool = true;
    module_param(tablet_mode_in_slate_state, bool, 0644);
    MODULE_PARM_DESC(tablet_mode_in_slate_state, "Enable tablet mode in slate device posture, default is 'true'");
pub const SSAM_EVENT_POS_CID_POSTURE_CHANGED: c_uint = 0x03;
pub const SSAM_POS_MAX_SOURCES: c_int = 4;
    enum ssam_pos_source_id {
    SSAM_POS_SOURCE_COVER = 0x00,
    SSAM_POS_SOURCE_SLS   = 0x03,
    };
    enum ssam_pos_state_cover {
    SSAM_POS_COVER_DISCONNECTED  = 0x01,
    SSAM_POS_COVER_CLOSED        = 0x02,
    SSAM_POS_COVER_LAPTOP        = 0x03,
    SSAM_POS_COVER_FOLDED_CANVAS = 0x04,
    SSAM_POS_COVER_FOLDED_BACK   = 0x05,
    SSAM_POS_COVER_BOOK          = 0x06,
    };
    enum ssam_pos_state_sls {
    SSAM_POS_SLS_LID_CLOSED = 0x00,
    SSAM_POS_SLS_LAPTOP     = 0x01,
    SSAM_POS_SLS_SLATE      = 0x02,
    SSAM_POS_SLS_TABLET     = 0x03,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ssam_sources_list {
    pub count: __le32,
    pub id: [__le32; SSAM_POS_MAX_SOURCES],
    pub __packed: },
    static const char *ssam_pos_state_name_cover(struct ssam_tablet_sw *sw, u32 state)
    {
    switch (state) {
    case SSAM_POS_COVER_DISCONNECTED:
    pub "disconnected": return,
    case SSAM_POS_COVER_CLOSED:
    pub "closed": return,
    case SSAM_POS_COVER_LAPTOP:
    pub "laptop": return,
    case SSAM_POS_COVER_FOLDED_CANVAS:
    pub "folded-canvas": return,
    case SSAM_POS_COVER_FOLDED_BACK:
    pub "folded-back": return,
    case SSAM_POS_COVER_BOOK:
    pub "book": return,
    default:
    pub state): dev_warn(&sw->sdev->dev, "unknown device posture for type-cover: %u\n",,
    pub "<unknown>": return,
    }
    }
    static const char *ssam_pos_state_name_sls(struct ssam_tablet_sw *sw, u32 state)
    {
    switch (state) {
    case SSAM_POS_SLS_LID_CLOSED:
    pub "closed": return,
    case SSAM_POS_SLS_LAPTOP:
    pub "laptop": return,
    case SSAM_POS_SLS_SLATE:
    pub "slate": return,
    case SSAM_POS_SLS_TABLET:
    pub "tablet": return,
    default:
    pub state): dev_warn(&sw->sdev->dev, "unknown device posture for SLS: %u\n",,
    pub "<unknown>": return,
    }
    }
    static const char *ssam_pos_state_name(struct ssam_tablet_sw *sw,
    const struct ssam_tablet_sw_state *state)
    {
    switch (state.source) {
    case SSAM_POS_SOURCE_COVER:
    pub state->state): return ssam_pos_state_name_cover(sw,,
    case SSAM_POS_SOURCE_SLS:
    pub state->state): return ssam_pos_state_name_sls(sw,,
    default:
    pub state->source): dev_warn(&sw->sdev->dev, "unknown device posture source: %u\n",,
    pub "<unknown>": return,
    }
    }
#[no_mangle]
unsafe extern "C" fn ssam_pos_state_is_tablet_mode_cover(sw: *mut ssam_tablet_sw, state: u32) -> bool {
    static bool ssam_pos_state_is_tablet_mode_cover(struct ssam_tablet_sw *sw, u32 state)
    {
    switch (state) {
    case SSAM_POS_COVER_DISCONNECTED:
    case SSAM_POS_COVER_FOLDED_CANVAS:
    case SSAM_POS_COVER_FOLDED_BACK:
    case SSAM_POS_COVER_BOOK:
    pub true: return,
    case SSAM_POS_COVER_CLOSED:
    case SSAM_POS_COVER_LAPTOP:
    pub false: return,
    default:
    pub state): dev_warn(&sw->sdev->dev, "unknown device posture for type-cover: %u\n",,
    pub true: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn ssam_pos_state_is_tablet_mode_sls(sw: *mut ssam_tablet_sw, state: u32) -> bool {
    static bool ssam_pos_state_is_tablet_mode_sls(struct ssam_tablet_sw *sw, u32 state)
    {
    switch (state) {
    case SSAM_POS_SLS_LAPTOP:
    case SSAM_POS_SLS_LID_CLOSED:
    pub false: return,
    case SSAM_POS_SLS_SLATE:
    pub tablet_mode_in_slate_state: return,
    case SSAM_POS_SLS_TABLET:
    pub true: return,
    default:
    pub state): dev_warn(&sw->sdev->dev, "unknown device posture for SLS: %u\n",,
    pub true: return,
    }
    }
    static bool ssam_pos_state_is_tablet_mode(struct ssam_tablet_sw *sw,
    const struct ssam_tablet_sw_state *state)
    {
    switch (state.source) {
    case SSAM_POS_SOURCE_COVER:
    pub state->state): return ssam_pos_state_is_tablet_mode_cover(sw,,
    case SSAM_POS_SOURCE_SLS:
    pub state->state): return ssam_pos_state_is_tablet_mode_sls(sw,,
    default:
    pub state->source): dev_warn(&sw->sdev->dev, "unknown device posture source: %u\n",,
    pub true: return,
    }
    }
#[no_mangle]
unsafe extern "C" fn ssam_pos_get_sources_list(sw: *mut ssam_tablet_sw, sources: *mut ssam_sources_list) -> c_int {
    static int ssam_pos_get_sources_list(struct ssam_tablet_sw *sw, struct ssam_sources_list *sources)
    {
    pub rqst: ssam_request,
    pub rsp: ssam_response,
    pub status: c_int,
    pub SSAM_SSH_TC_POS: rqst.target_category =,
    pub SSAM_SSH_TID_SAM: rqst.target_id =,
    pub 0x01: rqst.command_id =,
    pub 0x00: rqst.instance_id =,
    pub SSAM_REQUEST_HAS_RESPONSE: rqst.flags =,
    pub 0: rqst.length =,
    pub NULL: rqst.payload =,
    pub sizeof(*sources): *mut rsp.capacity =,
    pub 0: rsp.length =,
    pub )sources: *mut rsp.pointer = (u8,
    pub 0): status = ssam_retry(ssam_request_do_sync_onstack, sw->sdev->ctrl, &rqst, &rsp,,
    if (status)
    pub status: return,
// We need at least the 'sources->count' field.
    if (rsp.length < sizeof(__le32)) {
    pub small\n"): dev_err(&sw->sdev->dev, "received source list response is too,
    pub -EPROTO: return,
    }
// Make sure 'sources->count' matches with the response length.
    if (get_unaligned_le32(&sources.count) * sizeof(__le32) + sizeof(__le32) != rsp.length) {
    pub size\n"): dev_err(&sw->sdev->dev, "mismatch between number of sources and response,
    pub -EPROTO: return,
    }
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ssam_pos_get_source(sw: *mut ssam_tablet_sw, source_id: *mut u32) -> c_int {
    static int ssam_pos_get_source(struct ssam_tablet_sw *sw, u32 *source_id)
    {
    pub {}: ssam_sources_list sources =,
    pub status: c_int,
    pub &sources): status = ssam_pos_get_sources_list(sw,,
    if (status)
    pub status: return,
    if (get_unaligned_le32(&sources.count) == 0) {
    pub found\n"): dev_err(&sw->sdev->dev, "no posture sources,
    pub -ENODEV: return,
    }
//
// We currently don't know what to do with more than one posture
// source. At the moment, only one source seems to be used/provided.
// The WARN_ON() here should hopefully let us know quickly once there
// is a device that provides multiple sources, at which point we can
// then try to figure out how to handle them.
//
    pub 1): WARN_ON(get_unaligned_le32(&sources.count) >,
// source_id = get_unaligned_le32(&sources.id[0]);
    pub 0: return,
    }
    SSAM_DEFINE_SYNC_REQUEST_WR(__ssam_pos_get_posture_for_source, __le32, __le32, {
    .target_category = SSAM_SSH_TC_POS,
    .target_id       = SSAM_SSH_TID_SAM,
    .command_id      = 0x02,
    .instance_id     = 0x00,
#[no_mangle]
unsafe extern "C" fn ssam_pos_get_posture_for_source(sw: *mut ssam_tablet_sw, source_id: u32, posture: *mut u32) -> c_int {
    static int ssam_pos_get_posture_for_source(struct ssam_tablet_sw *sw, u32 source_id, u32 *posture)
    {
    pub cpu_to_le32(source_id): __le32 source_le =,
    pub 0: __le32 rspval_le =,
    pub status: c_int,
    status = ssam_retry(__ssam_pos_get_posture_for_source, sw.sdev.ctrl,
    pub &rspval_le): &source_le,,
    if (status)
    pub status: return,
// posture = le32_to_cpu(rspval_le);
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ssam_pos_get_posture(sw: *mut ssam_tablet_sw, state: *mut ssam_tablet_sw_state) -> c_int {
    static int ssam_pos_get_posture(struct ssam_tablet_sw *sw, struct ssam_tablet_sw_state *state)
    {
    pub source_id: u32,
    pub source_state: u32,
    pub status: c_int,
    pub &source_id): status = ssam_pos_get_source(sw,,
    if (status) {
    pub status): dev_err(&sw->sdev->dev, "failed to get posture source ID: %d\n",,
    pub status: return,
    }
    pub &source_state): status = ssam_pos_get_posture_for_source(sw, source_id,,
    if (status) {
    dev_err(&sw.sdev.dev, "failed to get posture value for source %u: %d\n",
    pub status): source_id,,
    pub status: return,
    }
    pub source_id: state->source =,
    pub source_state: state->state =,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn ssam_pos_sw_notif(nf: *mut ssam_event_notifier, event: *const ssam_event) -> u32 {
    static u32 ssam_pos_sw_notif(struct ssam_event_notifier *nf, const struct ssam_event *event)
    {
    pub notif): *mut *mut ssam_tablet_sw sw = container_of(nf, ssam_tablet_sw,,
    if (event.command_id != SSAM_EVENT_POS_CID_POSTURE_CHANGED)
    pub /: *mut *mut return 0; / Return "unhandled".,
    if (event.length != sizeof(__le32) * 3)
    pub event->length): dev_warn(&sw->sdev->dev, "unexpected payload size: %u\n",,
    pub SSAM_NOTIF_HANDLED: return,
    }
    static const struct ssam_tablet_sw_desc ssam_pos_sw_desc = {
    .dev = {
    .name = "Microsoft Surface POS Tablet Mode Switch",
    .phys = "ssam/01:26:01:00:01/input0",
    },
    .ops = {
    .notify = ssam_pos_sw_notif,
    .get_state = ssam_pos_get_posture,
    .state_name = ssam_pos_state_name,
    .state_is_tablet_mode = ssam_pos_state_is_tablet_mode,
    },
    .event = {
    .reg = SSAM_EVENT_REGISTRY_SAM,
    .id = {
    .target_category = SSAM_SSH_TC_POS,
    .instance = 0,
    },
    .mask = SSAM_EVENT_MASK_TARGET,
    },
}

// -- Driver registration. --------------------------------------------------
    static const struct ssam_device_id ssam_tablet_sw_match[] = {
    {
    SSAM_SDEV(KIP, SAM, 0x00, 0x01),
    .driver_data = (unsigned long)&ssam_kip_sw_desc,
    }, {
    SSAM_SDEV(POS, SAM, 0x00, 0x01),
    .driver_data = (unsigned long)&ssam_pos_sw_desc,
    },
    { },
    };
    MODULE_DEVICE_TABLE(ssam, ssam_tablet_sw_match);
    static struct ssam_device_driver ssam_tablet_sw_driver = {
    .probe = ssam_tablet_sw_probe,
    .remove = ssam_tablet_sw_remove,
    .match_table = ssam_tablet_sw_match,
    .driver = {
    .name = "surface_aggregator_tablet_mode_switch",
    .probe_type = PROBE_PREFER_ASYNCHRONOUS,
    .pm = &ssam_tablet_sw_pm_ops,
    },
    };
    module_ssam_device_driver(ssam_tablet_sw_driver);
    MODULE_AUTHOR("Maximilian Luz <luzmaximilian@gmail.com>");
    MODULE_DESCRIPTION("Tablet mode switch driver for Surface devices using the Surface Aggregator Module");
    MODULE_LICENSE("GPL");
