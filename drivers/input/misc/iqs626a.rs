//! Automatically rewritten from C to Rust
//! Source: drivers/input/misc/iqs626a.c
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
// Azoteq IQS626A Capacitive Touch Controller
//
// Copyright (C) 2020 Jeff LaBundy <jeff@labundy.com>
//
// This driver registers up to 2 input devices: one representing capacitive or
// inductive keys as well as Hall-effect switches, and one for a trackpad that
// can express various gestures.
//

pub const IQS626_VER_INFO: c_uint = 0x00;
pub const IQS626_VER_INFO_PROD_NUM: c_uint = 0x51;
pub const IQS626_SYS_FLAGS: c_uint = 0x02;

pub const IQS626_SYS_FLAGS_PWR_MODE_SHIFT: c_int = 8;
pub const IQS626_HALL_OUTPUT: c_uint = 0x23;
pub const IQS626_SYS_SETTINGS: c_uint = 0x80;

pub const IQS626_SYS_SETTINGS_PWR_MODE_SHIFT: c_int = 11;
pub const IQS626_SYS_SETTINGS_PWR_MODE_MAX: c_int = 3;

pub const IQS626_SYS_SETTINGS_ULP_UPDATE_SHIFT: c_int = 8;
pub const IQS626_SYS_SETTINGS_ULP_UPDATE_MAX: c_int = 7;

pub const IQS626_MISC_A_TPx_LTA_UPDATE_SHIFT: c_int = 4;
pub const IQS626_MISC_A_TPx_LTA_UPDATE_MAX: c_int = 7;

pub const IQS626_MISC_A_GPIO3_SELECT_MAX: c_int = 7;

pub const IQS626_RATE_NP_MS_MAX: c_int = 255;
pub const IQS626_RATE_LP_MS_MAX: c_int = 255;
pub const IQS626_RATE_ULP_MS_MAX: c_int = 4080;
pub const IQS626_TIMEOUT_PWR_MS_MAX: c_int = 130560;
pub const IQS626_TIMEOUT_LTA_MS_MAX: c_int = 130560;

pub const IQS626_MISC_B_RESEED_UI_SEL_SHIFT: c_int = 6;
pub const IQS626_MISC_B_RESEED_UI_SEL_MAX: c_int = 3;

pub const IQS626_THRESH_SWIPE_MAX: c_int = 255;
pub const IQS626_TIMEOUT_TAP_MS_MAX: c_int = 4080;
pub const IQS626_TIMEOUT_SWIPE_MS_MAX: c_int = 4080;

pub const IQS626_CHx_ENG_0_ATI_MODE_MAX: c_int = 3;

pub const IQS626_CHx_ENG_1_PROJ_BIAS_SHIFT: c_int = 4;
pub const IQS626_CHx_ENG_1_PROJ_BIAS_MAX: c_int = 3;

pub const IQS626_CHx_ENG_1_SENSE_FREQ_SHIFT: c_int = 1;
pub const IQS626_CHx_ENG_1_SENSE_FREQ_MAX: c_int = 3;

pub const IQS626_CHx_ENG_2_LOCAL_CAP_SHIFT: c_int = 6;
pub const IQS626_CHx_ENG_2_LOCAL_CAP_MAX: c_int = 3;

pub const IQS626_CHx_ENG_2_SENSE_MODE_MAX: c_int = 15;

pub const IQS626_CHx_ENG_3_TX_FREQ_SHIFT: c_int = 4;
pub const IQS626_CHx_ENG_3_TX_FREQ_MAX: c_int = 3;

pub const IQS626_TPx_ATI_BASE_MIN: c_int = 45;
pub const IQS626_TPx_ATI_BASE_MAX: c_int = 300;

pub const IQS626_CHx_ATI_BASE_75: c_uint = 0x00;
pub const IQS626_CHx_ATI_BASE_100: c_uint = 0x40;
pub const IQS626_CHx_ATI_BASE_150: c_uint = 0x80;
pub const IQS626_CHx_ATI_BASE_200: c_uint = 0xC0;

pub const IQS626_CHx_ATI_TARGET_MAX: c_int = 2016;
pub const IQS626_CHx_THRESH_MAX: c_int = 255;

pub const IQS626_CHx_HYST_DEEP_SHIFT: c_int = 4;

pub const IQS626_CHx_HYST_MAX: c_int = 15;

pub const IQS626_FILT_STR_NP_TPx_SHIFT: c_int = 6;

pub const IQS626_FILT_STR_LP_TPx_SHIFT: c_int = 4;

pub const IQS626_FILT_STR_NP_CNT_SHIFT: c_int = 6;

pub const IQS626_FILT_STR_LP_CNT_SHIFT: c_int = 4;

pub const IQS626_FILT_STR_NP_LTA_SHIFT: c_int = 2;

pub const IQS626_FILT_STR_MAX: c_int = 3;

pub const IQS626_GEN_WEIGHT_MAX: c_int = 255;
pub const IQS626_MAX_REG: c_uint = 0xFF;
pub const IQS626_NUM_CH_TP_3: c_int = 9;
pub const IQS626_NUM_CH_TP_2: c_int = 6;
pub const IQS626_NUM_CH_GEN: c_int = 3;
pub const IQS626_NUM_CRx_TX: c_int = 8;
pub const IQS626_PWR_MODE_POLL_SLEEP_US: c_int = 50000;
pub const IQS626_PWR_MODE_POLL_TIMEOUT_US: c_int = 500000;

    enum iqs626_ch_id {
    IQS626_CH_ULP_0,
    IQS626_CH_TP_2,
    IQS626_CH_TP_3,
    IQS626_CH_GEN_0,
    IQS626_CH_GEN_1,
    IQS626_CH_GEN_2,
    IQS626_CH_HALL,
    };
    enum iqs626_rx_inactive {
    IQS626_RX_INACTIVE_VSS,
    IQS626_RX_INACTIVE_FLOAT,
    IQS626_RX_INACTIVE_VREG,
    };
    enum iqs626_st_offs {
    IQS626_ST_OFFS_PROX,
    IQS626_ST_OFFS_DIR,
    IQS626_ST_OFFS_TOUCH,
    IQS626_ST_OFFS_DEEP,
    };
    enum iqs626_th_offs {
    IQS626_TH_OFFS_PROX,
    IQS626_TH_OFFS_TOUCH,
    IQS626_TH_OFFS_DEEP,
    };
    enum iqs626_event_id {
    IQS626_EVENT_PROX_DN,
    IQS626_EVENT_PROX_UP,
    IQS626_EVENT_TOUCH_DN,
    IQS626_EVENT_TOUCH_UP,
    IQS626_EVENT_DEEP_DN,
    IQS626_EVENT_DEEP_UP,
    };
    enum iqs626_gesture_id {
    IQS626_GESTURE_FLICK_X_POS,
    IQS626_GESTURE_FLICK_X_NEG,
    IQS626_GESTURE_FLICK_Y_POS,
    IQS626_GESTURE_FLICK_Y_NEG,
    IQS626_GESTURE_TAP,
    IQS626_GESTURE_HOLD,
    IQS626_NUM_GESTURES,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_event_desc {
    pub name: *const c_char,
    pub st_offs: enum iqs626_st_offs,
    pub th_offs: enum iqs626_th_offs,
    pub dir_up: bool,
    pub mask: u8,
}

    static const struct iqs626_event_desc iqs626_events[] = {
    [IQS626_EVENT_PROX_DN] = {
    .name = "event-prox",
    .st_offs = IQS626_ST_OFFS_PROX,
    .th_offs = IQS626_TH_OFFS_PROX,
    .mask = IQS626_EVENT_MASK_PROX,
    },
    [IQS626_EVENT_PROX_UP] = {
    .name = "event-prox-alt",
    .st_offs = IQS626_ST_OFFS_PROX,
    .th_offs = IQS626_TH_OFFS_PROX,
    .dir_up = true,
    .mask = IQS626_EVENT_MASK_PROX,
    },
    [IQS626_EVENT_TOUCH_DN] = {
    .name = "event-touch",
    .st_offs = IQS626_ST_OFFS_TOUCH,
    .th_offs = IQS626_TH_OFFS_TOUCH,
    .mask = IQS626_EVENT_MASK_TOUCH,
    },
    [IQS626_EVENT_TOUCH_UP] = {
    .name = "event-touch-alt",
    .st_offs = IQS626_ST_OFFS_TOUCH,
    .th_offs = IQS626_TH_OFFS_TOUCH,
    .dir_up = true,
    .mask = IQS626_EVENT_MASK_TOUCH,
    },
    [IQS626_EVENT_DEEP_DN] = {
    .name = "event-deep",
    .st_offs = IQS626_ST_OFFS_DEEP,
    .th_offs = IQS626_TH_OFFS_DEEP,
    .mask = IQS626_EVENT_MASK_DEEP,
    },
    [IQS626_EVENT_DEEP_UP] = {
    .name = "event-deep-alt",
    .st_offs = IQS626_ST_OFFS_DEEP,
    .th_offs = IQS626_TH_OFFS_DEEP,
    .dir_up = true,
    .mask = IQS626_EVENT_MASK_DEEP,
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_ver_info {
    pub prod_num: u8,
    pub sw_num: u8,
    pub hw_num: u8,
    pub padding: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_flags {
    pub system: __be16,
    pub gesture: u8,
    pub padding_a: u8,
    pub states: [u8; 4],
    pub ref_active: u8,
    pub padding_b: u8,
    pub comp_min: u8,
    pub comp_max: u8,
    pub trackpad_x: u8,
    pub trackpad_y: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_ch_reg_ulp {
    pub thresh: [u8; 2],
    pub hyst: u8,
    pub filter: u8,
    pub engine: [u8; 2],
    pub ati_target: u8,
    pub padding: u8,
    pub ati_comp: __be16,
    pub rx_enable: u8,
    pub tx_enable: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_ch_reg_tp {
    pub thresh: u8,
    pub ati_base: u8,
    pub ati_comp: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_tp_grp_reg {
    pub hyst: u8,
    pub ati_target: u8,
    pub engine: [u8; 2],
    pub ch_reg_tp: [iqs626_ch_reg_tp; IQS626_NUM_CH_TP_3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_ch_reg_gen {
    pub thresh: [u8; 3],
    pub padding: u8,
    pub hyst: u8,
    pub ati_target: u8,
    pub ati_comp: __be16,
    pub engine: [u8; 5],
    pub filter: u8,
    pub rx_enable: u8,
    pub tx_enable: u8,
    pub assoc_select: u8,
    pub assoc_weight: u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_ch_reg_hall {
    pub engine: u8,
    pub thresh: u8,
    pub hyst: u8,
    pub ati_target: u8,
    pub ati_comp: __be16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_sys_reg {
    pub general: __be16,
    pub misc_a: u8,
    pub event_mask: u8,
    pub active: u8,
    pub reseed: u8,
    pub rate_np: u8,
    pub rate_lp: u8,
    pub rate_ulp: u8,
    pub timeout_pwr: u8,
    pub timeout_rdy: u8,
    pub timeout_lta: u8,
    pub misc_b: u8,
    pub thresh_swipe: u8,
    pub timeout_tap: u8,
    pub timeout_swipe: u8,
    pub redo_ati: u8,
    pub padding: u8,
    pub ch_reg_ulp: iqs626_ch_reg_ulp,
    pub tp_grp_reg: iqs626_tp_grp_reg,
    pub ch_reg_gen: [iqs626_ch_reg_gen; IQS626_NUM_CH_GEN],
    pub ch_reg_hall: iqs626_ch_reg_hall,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_channel_desc {
    pub name: *const c_char,
    pub num_ch: c_int,
    pub active: u8,
    pub events: [bool; ARRAY_SIZE(iqs626_events)],
}

    static const struct iqs626_channel_desc iqs626_channels[] = {
    [IQS626_CH_ULP_0] = {
    .name = "ulp-0",
    .num_ch = 1,
    .active = BIT(0),
    .events = {
    [IQS626_EVENT_PROX_DN] = true,
    [IQS626_EVENT_PROX_UP] = true,
    [IQS626_EVENT_TOUCH_DN] = true,
    [IQS626_EVENT_TOUCH_UP] = true,
    },
    },
    [IQS626_CH_TP_2] = {
    .name = "trackpad-3x2",
    .num_ch = IQS626_NUM_CH_TP_2,
    .active = BIT(1),
    .events = {
    [IQS626_EVENT_TOUCH_DN] = true,
    },
    },
    [IQS626_CH_TP_3] = {
    .name = "trackpad-3x3",
    .num_ch = IQS626_NUM_CH_TP_3,
    .active = BIT(2) | BIT(1),
    .events = {
    [IQS626_EVENT_TOUCH_DN] = true,
    },
    },
    [IQS626_CH_GEN_0] = {
    .name = "generic-0",
    .num_ch = 1,
    .active = BIT(4),
    .events = {
    [IQS626_EVENT_PROX_DN] = true,
    [IQS626_EVENT_PROX_UP] = true,
    [IQS626_EVENT_TOUCH_DN] = true,
    [IQS626_EVENT_TOUCH_UP] = true,
    [IQS626_EVENT_DEEP_DN] = true,
    [IQS626_EVENT_DEEP_UP] = true,
    },
    },
    [IQS626_CH_GEN_1] = {
    .name = "generic-1",
    .num_ch = 1,
    .active = BIT(5),
    .events = {
    [IQS626_EVENT_PROX_DN] = true,
    [IQS626_EVENT_PROX_UP] = true,
    [IQS626_EVENT_TOUCH_DN] = true,
    [IQS626_EVENT_TOUCH_UP] = true,
    [IQS626_EVENT_DEEP_DN] = true,
    [IQS626_EVENT_DEEP_UP] = true,
    },
    },
    [IQS626_CH_GEN_2] = {
    .name = "generic-2",
    .num_ch = 1,
    .active = BIT(6),
    .events = {
    [IQS626_EVENT_PROX_DN] = true,
    [IQS626_EVENT_PROX_UP] = true,
    [IQS626_EVENT_TOUCH_DN] = true,
    [IQS626_EVENT_TOUCH_UP] = true,
    [IQS626_EVENT_DEEP_DN] = true,
    [IQS626_EVENT_DEEP_UP] = true,
    },
    },
    [IQS626_CH_HALL] = {
    .name = "hall",
    .num_ch = 1,
    .active = BIT(7),
    .events = {
    [IQS626_EVENT_TOUCH_DN] = true,
    [IQS626_EVENT_TOUCH_UP] = true,
    },
    },
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iqs626_private {
    pub client: *mut i2c_client,
    pub regmap: *mut regmap,
    pub sys_reg: iqs626_sys_reg,
    pub ati_done: completion,
    pub keypad: *mut input_dev,
    pub trackpad: *mut input_dev,
    pub prop: touchscreen_properties,
    unsigned int kp_type[ARRAY_SIZE(iqs626_channels)]
    unsigned int kp_code[ARRAY_SIZE(iqs626_channels)]
    pub tp_code: [c_uint; IQS626_NUM_GESTURES],
    pub suspend_mode: c_uint,
}

    static noinline_for_stack int
    iqs626_parse_events(struct iqs626_private *iqs626,
    struct fwnode_handle *ch_node, enum iqs626_ch_id ch_id)
    {
    struct iqs626_sys_reg *sys_reg = &iqs626.sys_reg;
    struct i2c_client *client = iqs626.client;
    const char *ev_name;
    u8 *thresh, *hyst;
    unsigned int val;
    int i;
    switch (ch_id) {
    case IQS626_CH_ULP_0:
    thresh = sys_reg.ch_reg_ulp.thresh;
    hyst = &sys_reg.ch_reg_ulp.hyst;
    break;
    case IQS626_CH_TP_2:
    case IQS626_CH_TP_3:
    thresh = &sys_reg.tp_grp_reg.ch_reg_tp[0].thresh;
    hyst = &sys_reg.tp_grp_reg.hyst;
    break;
    case IQS626_CH_GEN_0:
    case IQS626_CH_GEN_1:
    case IQS626_CH_GEN_2:
    i = ch_id - IQS626_CH_GEN_0;
    thresh = sys_reg.ch_reg_gen[i].thresh;
    hyst = &sys_reg.ch_reg_gen[i].hyst;
    break;
    case IQS626_CH_HALL:
    thresh = &sys_reg.ch_reg_hall.thresh;
    hyst = &sys_reg.ch_reg_hall.hyst;
    break;
    default:
    return -EINVAL;
    }
    for (i = 0; i < ARRAY_SIZE(iqs626_events); i++) {
    if (!iqs626_channels[ch_id].events[i])
    continue;
    struct fwnode_handle *ev_node __free(fwnode_handle) = core::ptr::null_mut();
    if (ch_id == IQS626_CH_TP_2 || ch_id == IQS626_CH_TP_3) {
//
// Trackpad touch events are simply described under the
// trackpad child node.
//
    ev_node = fwnode_handle_get(ch_node);
    } else {
    ev_name = iqs626_events[i].name;
    ev_node = fwnode_get_named_child_node(ch_node, ev_name);
    if (!ev_node)
    continue;
    if (!fwnode_property_read_u32(ev_node, "linux,code",
    &val)) {
    iqs626.kp_code[ch_id][i] = val;
    if (fwnode_property_read_u32(ev_node,
    "linux,input-type",
    &val)) {
    if (ch_id == IQS626_CH_HALL)
    val = EV_SW;
    else
    val = EV_KEY;
    }
    if (val != EV_KEY && val != EV_SW) {
    dev_err(&client.dev,
    "Invalid input type: %u\n",
    val);
    return -EINVAL;
    }
    iqs626.kp_type[ch_id][i] = val;
    sys_reg.event_mask &= ~iqs626_events[i].mask;
    }
    }
    if (!fwnode_property_read_u32(ev_node, "azoteq,hyst", &val)) {
    if (val > IQS626_CHx_HYST_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel hysteresis: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    if (i == IQS626_EVENT_DEEP_DN ||
    i == IQS626_EVENT_DEEP_UP) {
// hyst &= ~IQS626_CHx_HYST_DEEP_MASK;
// hyst |= (val << IQS626_CHx_HYST_DEEP_SHIFT);
    } else if (i == IQS626_EVENT_TOUCH_DN ||
    i == IQS626_EVENT_TOUCH_UP) {
// hyst &= ~IQS626_CHx_HYST_TOUCH_MASK;
// hyst |= val;
    }
    }
    if (ch_id != IQS626_CH_TP_2 && ch_id != IQS626_CH_TP_3 &&
    !fwnode_property_read_u32(ev_node, "azoteq,thresh", &val)) {
    if (val > IQS626_CHx_THRESH_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel threshold: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    if (ch_id == IQS626_CH_HALL)
// thresh = val;
    else
// (thresh + iqs626_events[i].th_offs) = val;
    }
    }
    return 0;
    }
    static noinline_for_stack int
    iqs626_parse_ati_target(struct iqs626_private *iqs626,
    struct fwnode_handle *ch_node, enum iqs626_ch_id ch_id)
    {
    struct iqs626_sys_reg *sys_reg = &iqs626.sys_reg;
    struct i2c_client *client = iqs626.client;
    unsigned int val;
    u8 *ati_target;
    int i;
    switch (ch_id) {
    case IQS626_CH_ULP_0:
    ati_target = &sys_reg.ch_reg_ulp.ati_target;
    break;
    case IQS626_CH_TP_2:
    case IQS626_CH_TP_3:
    ati_target = &sys_reg.tp_grp_reg.ati_target;
    break;
    case IQS626_CH_GEN_0:
    case IQS626_CH_GEN_1:
    case IQS626_CH_GEN_2:
    i = ch_id - IQS626_CH_GEN_0;
    ati_target = &sys_reg.ch_reg_gen[i].ati_target;
    break;
    case IQS626_CH_HALL:
    ati_target = &sys_reg.ch_reg_hall.ati_target;
    break;
    default:
    return -EINVAL;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,ati-target", &val)) {
    if (val > IQS626_CHx_ATI_TARGET_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel ATI target: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// ati_target &= ~IQS626_CHx_ATI_TARGET_MASK;
// ati_target |= (val / 32);
    }
    if (ch_id != IQS626_CH_TP_2 && ch_id != IQS626_CH_TP_3 &&
    !fwnode_property_read_u32(ch_node, "azoteq,ati-base", &val)) {
    switch (val) {
    case 75:
    val = IQS626_CHx_ATI_BASE_75;
    break;
    case 100:
    val = IQS626_CHx_ATI_BASE_100;
    break;
    case 150:
    val = IQS626_CHx_ATI_BASE_150;
    break;
    case 200:
    val = IQS626_CHx_ATI_BASE_200;
    break;
    default:
    dev_err(&client.dev,
    "Invalid %s channel ATI base: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// ati_target &= ~IQS626_CHx_ATI_BASE_MASK;
// ati_target |= val;
    }
    return 0;
    }
    static int iqs626_parse_pins(struct iqs626_private *iqs626,
    struct fwnode_handle *ch_node,
    const char *propname, u8 *enable)
    {
    struct i2c_client *client = iqs626.client;
    unsigned int val[IQS626_NUM_CRx_TX];
    int error, count, i;
    if (!fwnode_property_present(ch_node, propname))
    return 0;
    count = fwnode_property_count_u32(ch_node, propname);
    if (count > IQS626_NUM_CRx_TX) {
    dev_err(&client.dev,
    "Too many %s channel CRX/TX pins present\n",
    fwnode_get_name(ch_node));
    return -EINVAL;
    } else if (count < 0) {
    dev_err(&client.dev,
    "Failed to count %s channel CRX/TX pins: %d\n",
    fwnode_get_name(ch_node), count);
    return count;
    }
    error = fwnode_property_read_u32_array(ch_node, propname, val, count);
    if (error) {
    dev_err(&client.dev,
    "Failed to read %s channel CRX/TX pins: %d\n",
    fwnode_get_name(ch_node), error);
    return error;
    }
// enable = 0;
    for (i = 0; i < count; i++) {
    if (val[i] >= IQS626_NUM_CRx_TX) {
    dev_err(&client.dev,
    "Invalid %s channel CRX/TX pin: %u\n",
    fwnode_get_name(ch_node), val[i]);
    return -EINVAL;
    }
// enable |= BIT(val[i]);
    }
    return 0;
    }
    static int iqs626_parse_trackpad(struct iqs626_private *iqs626,
    struct fwnode_handle *ch_node,
    enum iqs626_ch_id ch_id)
    {
    struct iqs626_sys_reg *sys_reg = &iqs626.sys_reg;
    struct i2c_client *client = iqs626.client;
    u8 *hyst = &sys_reg.tp_grp_reg.hyst;
    int error, count, i;
    unsigned int val;
    if (!fwnode_property_read_u32(ch_node, "azoteq,lta-update", &val)) {
    if (val > IQS626_MISC_A_TPx_LTA_UPDATE_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel update rate: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    sys_reg.misc_a &= ~IQS626_MISC_A_TPx_LTA_UPDATE_MASK;
    sys_reg.misc_a |= (val << IQS626_MISC_A_TPx_LTA_UPDATE_SHIFT);
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,filt-str-trackpad",
    &val)) {
    if (val > IQS626_FILT_STR_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel filter strength: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    sys_reg.misc_b &= ~IQS626_MISC_B_FILT_STR_TPx;
    sys_reg.misc_b |= val;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,filt-str-np-cnt",
    &val)) {
    if (val > IQS626_FILT_STR_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel filter strength: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// hyst &= ~IQS626_FILT_STR_NP_TPx_MASK;
// hyst |= (val << IQS626_FILT_STR_NP_TPx_SHIFT);
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,filt-str-lp-cnt",
    &val)) {
    if (val > IQS626_FILT_STR_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel filter strength: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// hyst &= ~IQS626_FILT_STR_LP_TPx_MASK;
// hyst |= (val << IQS626_FILT_STR_LP_TPx_SHIFT);
    }
    for (i = 0; i < iqs626_channels[ch_id].num_ch; i++) {
    u8 *ati_base = &sys_reg.tp_grp_reg.ch_reg_tp[i].ati_base;
    u8 *thresh = &sys_reg.tp_grp_reg.ch_reg_tp[i].thresh;
    char tc_name[10];
    scnprintf(tc_name, sizeof(tc_name), "channel-%d", i);
    struct fwnode_handle *tc_node __free(fwnode_handle) =
    fwnode_get_named_child_node(ch_node, tc_name);
    if (!tc_node)
    continue;
    if (!fwnode_property_read_u32(tc_node, "azoteq,ati-base",
    &val)) {
    if (val < IQS626_TPx_ATI_BASE_MIN ||
    val > IQS626_TPx_ATI_BASE_MAX) {
    dev_err(&client.dev,
    "Invalid %s %s ATI base: %u\n",
    fwnode_get_name(ch_node), tc_name, val);
    return -EINVAL;
    }
// ati_base = val - IQS626_TPx_ATI_BASE_MIN;
    }
    if (!fwnode_property_read_u32(tc_node, "azoteq,thresh",
    &val)) {
    if (val > IQS626_CHx_THRESH_MAX) {
    dev_err(&client.dev,
    "Invalid %s %s threshold: %u\n",
    fwnode_get_name(ch_node), tc_name, val);
    return -EINVAL;
    }
// thresh = val;
    }
    }
    if (!fwnode_property_present(ch_node, "linux,keycodes"))
    return 0;
    count = fwnode_property_count_u32(ch_node, "linux,keycodes");
    if (count > IQS626_NUM_GESTURES) {
    dev_err(&client.dev, "Too many keycodes present\n");
    return -EINVAL;
    } else if (count < 0) {
    dev_err(&client.dev, "Failed to count keycodes: %d\n", count);
    return count;
    }
    error = fwnode_property_read_u32_array(ch_node, "linux,keycodes",
    iqs626.tp_code, count);
    if (error) {
    dev_err(&client.dev, "Failed to read keycodes: %d\n", error);
    return error;
    }
    sys_reg.misc_b &= ~IQS626_MISC_B_TPx_SWIPE;
    if (fwnode_property_present(ch_node, "azoteq,gesture-swipe"))
    sys_reg.misc_b |= IQS626_MISC_B_TPx_SWIPE;
    if (!fwnode_property_read_u32(ch_node, "azoteq,timeout-tap-ms",
    &val)) {
    if (val > IQS626_TIMEOUT_TAP_MS_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel timeout: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    sys_reg.timeout_tap = val / 16;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,timeout-swipe-ms",
    &val)) {
    if (val > IQS626_TIMEOUT_SWIPE_MS_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel timeout: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    sys_reg.timeout_swipe = val / 16;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,thresh-swipe",
    &val)) {
    if (val > IQS626_THRESH_SWIPE_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel threshold: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    sys_reg.thresh_swipe = val;
    }
    sys_reg.event_mask &= ~IQS626_EVENT_MASK_GESTURE;
    return 0;
    }
    static noinline_for_stack int
    iqs626_parse_channel(struct iqs626_private *iqs626,
    struct fwnode_handle *ch_node, enum iqs626_ch_id ch_id)
    {
    struct iqs626_sys_reg *sys_reg = &iqs626.sys_reg;
    struct i2c_client *client = iqs626.client;
    u8 *engine, *filter, *rx_enable, *tx_enable;
    u8 *assoc_select, *assoc_weight;
    unsigned int val;
    int error, i;
    switch (ch_id) {
    case IQS626_CH_ULP_0:
    engine = sys_reg.ch_reg_ulp.engine;
    break;
    case IQS626_CH_TP_2:
    case IQS626_CH_TP_3:
    engine = sys_reg.tp_grp_reg.engine;
    break;
    case IQS626_CH_GEN_0:
    case IQS626_CH_GEN_1:
    case IQS626_CH_GEN_2:
    i = ch_id - IQS626_CH_GEN_0;
    engine = sys_reg.ch_reg_gen[i].engine;
    break;
    case IQS626_CH_HALL:
    engine = &sys_reg.ch_reg_hall.engine;
    break;
    default:
    return -EINVAL;
    }
    error = iqs626_parse_ati_target(iqs626, ch_node, ch_id);
    if (error)
    return error;
    error = iqs626_parse_events(iqs626, ch_node, ch_id);
    if (error)
    return error;
    if (!fwnode_property_present(ch_node, "azoteq,ati-exclude"))
    sys_reg.redo_ati |= iqs626_channels[ch_id].active;
    if (!fwnode_property_present(ch_node, "azoteq,reseed-disable"))
    sys_reg.reseed |= iqs626_channels[ch_id].active;
// engine |= IQS626_CHx_ENG_0_MEAS_CAP_SIZE;
    if (fwnode_property_present(ch_node, "azoteq,meas-cap-decrease"))
// engine &= ~IQS626_CHx_ENG_0_MEAS_CAP_SIZE;
// engine |= IQS626_CHx_ENG_0_RX_TERM_VSS;
    if (!fwnode_property_read_u32(ch_node, "azoteq,rx-inactive", &val)) {
    switch (val) {
    case IQS626_RX_INACTIVE_VSS:
    break;
    case IQS626_RX_INACTIVE_FLOAT:
// engine &= ~IQS626_CHx_ENG_0_RX_TERM_VSS;
    if (ch_id == IQS626_CH_GEN_0 ||
    ch_id == IQS626_CH_GEN_1 ||
    ch_id == IQS626_CH_GEN_2)
// (engine + 4) &= ~IQS626_CHx_ENG_4_RX_TERM_VREG;
    break;
    case IQS626_RX_INACTIVE_VREG:
    if (ch_id == IQS626_CH_GEN_0 ||
    ch_id == IQS626_CH_GEN_1 ||
    ch_id == IQS626_CH_GEN_2) {
// engine &= ~IQS626_CHx_ENG_0_RX_TERM_VSS;
// (engine + 4) |= IQS626_CHx_ENG_4_RX_TERM_VREG;
    break;
    }
    fallthrough;
    default:
    dev_err(&client.dev,
    "Invalid %s channel CRX pin termination: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
    }
// engine &= ~IQS626_CHx_ENG_0_LINEARIZE;
    if (fwnode_property_present(ch_node, "azoteq,linearize"))
// engine |= IQS626_CHx_ENG_0_LINEARIZE;
// engine &= ~IQS626_CHx_ENG_0_DUAL_DIR;
    if (fwnode_property_present(ch_node, "azoteq,dual-direction"))
// engine |= IQS626_CHx_ENG_0_DUAL_DIR;
// engine &= ~IQS626_CHx_ENG_0_FILT_DISABLE;
    if (fwnode_property_present(ch_node, "azoteq,filt-disable"))
// engine |= IQS626_CHx_ENG_0_FILT_DISABLE;
    if (!fwnode_property_read_u32(ch_node, "azoteq,ati-mode", &val)) {
    if (val > IQS626_CHx_ENG_0_ATI_MODE_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel ATI mode: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// engine &= ~IQS626_CHx_ENG_0_ATI_MODE_MASK;
// engine |= val;
    }
    if (ch_id == IQS626_CH_HALL)
    return 0;
// (engine + 1) &= ~IQS626_CHx_ENG_1_CCT_ENABLE;
    if (!fwnode_property_read_u32(ch_node, "azoteq,cct-increase",
    &val) && val) {
    let mut orig_val: c_uint = val--;
//
// In the case of the generic channels, the charge cycle time
// field doubles in size and straddles two separate registers.
//
    if (ch_id == IQS626_CH_GEN_0 ||
    ch_id == IQS626_CH_GEN_1 ||
    ch_id == IQS626_CH_GEN_2) {
// (engine + 4) &= ~IQS626_CHx_ENG_4_CCT_LOW_1;
    if (val & BIT(1))
// (engine + 4) |= IQS626_CHx_ENG_4_CCT_LOW_1;
// (engine + 4) &= ~IQS626_CHx_ENG_4_CCT_LOW_0;
    if (val & BIT(0))
// (engine + 4) |= IQS626_CHx_ENG_4_CCT_LOW_0;
    val >>= 2;
    }
    if (val & ~GENMASK(1, 0)) {
    dev_err(&client.dev,
    "Invalid %s channel charge cycle time: %u\n",
    fwnode_get_name(ch_node), orig_val);
    return -EINVAL;
    }
// (engine + 1) &= ~IQS626_CHx_ENG_1_CCT_HIGH_1;
    if (val & BIT(1))
// (engine + 1) |= IQS626_CHx_ENG_1_CCT_HIGH_1;
// (engine + 1) &= ~IQS626_CHx_ENG_1_CCT_HIGH_0;
    if (val & BIT(0))
// (engine + 1) |= IQS626_CHx_ENG_1_CCT_HIGH_0;
// (engine + 1) |= IQS626_CHx_ENG_1_CCT_ENABLE;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,proj-bias", &val)) {
    if (val > IQS626_CHx_ENG_1_PROJ_BIAS_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel bias current: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// (engine + 1) &= ~IQS626_CHx_ENG_1_PROJ_BIAS_MASK;
// (engine + 1) |= (val << IQS626_CHx_ENG_1_PROJ_BIAS_SHIFT);
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,sense-freq", &val)) {
    if (val > IQS626_CHx_ENG_1_SENSE_FREQ_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel sensing frequency: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// (engine + 1) &= ~IQS626_CHx_ENG_1_SENSE_FREQ_MASK;
// (engine + 1) |= (val << IQS626_CHx_ENG_1_SENSE_FREQ_SHIFT);
    }
// (engine + 1) &= ~IQS626_CHx_ENG_1_ATI_BAND_TIGHTEN;
    if (fwnode_property_present(ch_node, "azoteq,ati-band-tighten"))
// (engine + 1) |= IQS626_CHx_ENG_1_ATI_BAND_TIGHTEN;
    if (ch_id == IQS626_CH_TP_2 || ch_id == IQS626_CH_TP_3)
    return iqs626_parse_trackpad(iqs626, ch_node, ch_id);
    if (ch_id == IQS626_CH_ULP_0) {
    sys_reg.ch_reg_ulp.hyst &= ~IQS626_ULP_PROJ_ENABLE;
    if (fwnode_property_present(ch_node, "azoteq,proj-enable"))
    sys_reg.ch_reg_ulp.hyst |= IQS626_ULP_PROJ_ENABLE;
    filter = &sys_reg.ch_reg_ulp.filter;
    rx_enable = &sys_reg.ch_reg_ulp.rx_enable;
    tx_enable = &sys_reg.ch_reg_ulp.tx_enable;
    } else {
    i = ch_id - IQS626_CH_GEN_0;
    filter = &sys_reg.ch_reg_gen[i].filter;
    rx_enable = &sys_reg.ch_reg_gen[i].rx_enable;
    tx_enable = &sys_reg.ch_reg_gen[i].tx_enable;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,filt-str-np-cnt",
    &val)) {
    if (val > IQS626_FILT_STR_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel filter strength: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// filter &= ~IQS626_FILT_STR_NP_CNT_MASK;
// filter |= (val << IQS626_FILT_STR_NP_CNT_SHIFT);
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,filt-str-lp-cnt",
    &val)) {
    if (val > IQS626_FILT_STR_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel filter strength: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// filter &= ~IQS626_FILT_STR_LP_CNT_MASK;
// filter |= (val << IQS626_FILT_STR_LP_CNT_SHIFT);
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,filt-str-np-lta",
    &val)) {
    if (val > IQS626_FILT_STR_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel filter strength: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// filter &= ~IQS626_FILT_STR_NP_LTA_MASK;
// filter |= (val << IQS626_FILT_STR_NP_LTA_SHIFT);
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,filt-str-lp-lta",
    &val)) {
    if (val > IQS626_FILT_STR_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel filter strength: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// filter &= ~IQS626_FILT_STR_LP_LTA_MASK;
// filter |= val;
    }
    error = iqs626_parse_pins(iqs626, ch_node, "azoteq,rx-enable",
    rx_enable);
    if (error)
    return error;
    error = iqs626_parse_pins(iqs626, ch_node, "azoteq,tx-enable",
    tx_enable);
    if (error)
    return error;
    if (ch_id == IQS626_CH_ULP_0)
    return 0;
// (engine + 2) &= ~IQS626_CHx_ENG_2_LOCAL_CAP_ENABLE;
    if (!fwnode_property_read_u32(ch_node, "azoteq,local-cap-size",
    &val) && val) {
    let mut orig_val: c_uint = val--;
    if (val > IQS626_CHx_ENG_2_LOCAL_CAP_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel local cap. size: %u\n",
    fwnode_get_name(ch_node), orig_val);
    return -EINVAL;
    }
// (engine + 2) &= ~IQS626_CHx_ENG_2_LOCAL_CAP_MASK;
// (engine + 2) |= (val << IQS626_CHx_ENG_2_LOCAL_CAP_SHIFT);
// (engine + 2) |= IQS626_CHx_ENG_2_LOCAL_CAP_ENABLE;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,sense-mode", &val)) {
    if (val > IQS626_CHx_ENG_2_SENSE_MODE_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel sensing mode: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// (engine + 2) &= ~IQS626_CHx_ENG_2_SENSE_MODE_MASK;
// (engine + 2) |= val;
    }
    if (!fwnode_property_read_u32(ch_node, "azoteq,tx-freq", &val)) {
    if (val > IQS626_CHx_ENG_3_TX_FREQ_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel excitation frequency: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// (engine + 3) &= ~IQS626_CHx_ENG_3_TX_FREQ_MASK;
// (engine + 3) |= (val << IQS626_CHx_ENG_3_TX_FREQ_SHIFT);
    }
// (engine + 3) &= ~IQS626_CHx_ENG_3_INV_LOGIC;
    if (fwnode_property_present(ch_node, "azoteq,invert-enable"))
// (engine + 3) |= IQS626_CHx_ENG_3_INV_LOGIC;
// (engine + 4) &= ~IQS626_CHx_ENG_4_COMP_DISABLE;
    if (fwnode_property_present(ch_node, "azoteq,comp-disable"))
// (engine + 4) |= IQS626_CHx_ENG_4_COMP_DISABLE;
// (engine + 4) &= ~IQS626_CHx_ENG_4_STATIC_ENABLE;
    if (fwnode_property_present(ch_node, "azoteq,static-enable"))
// (engine + 4) |= IQS626_CHx_ENG_4_STATIC_ENABLE;
    i = ch_id - IQS626_CH_GEN_0;
    assoc_select = &sys_reg.ch_reg_gen[i].assoc_select;
    assoc_weight = &sys_reg.ch_reg_gen[i].assoc_weight;
// assoc_select = 0;
    if (!fwnode_property_present(ch_node, "azoteq,assoc-select"))
    return 0;
    for (i = 0; i < ARRAY_SIZE(iqs626_channels); i++) {
    if (fwnode_property_match_string(ch_node, "azoteq,assoc-select",
    iqs626_channels[i].name) < 0)
    continue;
// assoc_select |= iqs626_channels[i].active;
    }
    if (fwnode_property_read_u32(ch_node, "azoteq,assoc-weight", &val))
    return 0;
    if (val > IQS626_GEN_WEIGHT_MAX) {
    dev_err(&client.dev,
    "Invalid %s channel associated weight: %u\n",
    fwnode_get_name(ch_node), val);
    return -EINVAL;
    }
// assoc_weight = val;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iqs626_parse_prop(iqs626: *mut iqs626_private) -> c_int {
    static int iqs626_parse_prop(struct iqs626_private *iqs626)
    {
    struct iqs626_sys_reg *sys_reg = &iqs626.sys_reg;
    struct i2c_client *client = iqs626.client;
    unsigned int val;
    int error, i;
    u16 general;
    if (!device_property_read_u32(&client.dev, "azoteq,suspend-mode",
    &val)) {
    if (val > IQS626_SYS_SETTINGS_PWR_MODE_MAX) {
    dev_err(&client.dev, "Invalid suspend mode: %u\n",
    val);
    return -EINVAL;
    }
    iqs626.suspend_mode = val;
    }
    error = regmap_raw_read(iqs626.regmap, IQS626_SYS_SETTINGS, sys_reg,
    sizeof(*sys_reg));
    if (error)
    return error;
    general = be16_to_cpu(sys_reg.general);
    general &= IQS626_SYS_SETTINGS_ULP_UPDATE_MASK;
    if (device_property_present(&client.dev, "azoteq,clk-div"))
    general |= IQS626_SYS_SETTINGS_CLK_DIV;
    if (device_property_present(&client.dev, "azoteq,ulp-enable"))
    general |= IQS626_SYS_SETTINGS_ULP_AUTO;
    if (!device_property_read_u32(&client.dev, "azoteq,ulp-update",
    &val)) {
    if (val > IQS626_SYS_SETTINGS_ULP_UPDATE_MAX) {
    dev_err(&client.dev, "Invalid update rate: %u\n", val);
    return -EINVAL;
    }
    general &= ~IQS626_SYS_SETTINGS_ULP_UPDATE_MASK;
    general |= (val << IQS626_SYS_SETTINGS_ULP_UPDATE_SHIFT);
    }
    sys_reg.misc_a &= ~IQS626_MISC_A_ATI_BAND_DISABLE;
    if (device_property_present(&client.dev, "azoteq,ati-band-disable"))
    sys_reg.misc_a |= IQS626_MISC_A_ATI_BAND_DISABLE;
    sys_reg.misc_a &= ~IQS626_MISC_A_ATI_LP_ONLY;
    if (device_property_present(&client.dev, "azoteq,ati-lp-only"))
    sys_reg.misc_a |= IQS626_MISC_A_ATI_LP_ONLY;
    if (!device_property_read_u32(&client.dev, "azoteq,gpio3-select",
    &val)) {
    if (val > IQS626_MISC_A_GPIO3_SELECT_MAX) {
    dev_err(&client.dev, "Invalid GPIO3 selection: %u\n",
    val);
    return -EINVAL;
    }
    sys_reg.misc_a &= ~IQS626_MISC_A_GPIO3_SELECT_MASK;
    sys_reg.misc_a |= val;
    }
    if (!device_property_read_u32(&client.dev, "azoteq,reseed-select",
    &val)) {
    if (val > IQS626_MISC_B_RESEED_UI_SEL_MAX) {
    dev_err(&client.dev, "Invalid reseed selection: %u\n",
    val);
    return -EINVAL;
    }
    sys_reg.misc_b &= ~IQS626_MISC_B_RESEED_UI_SEL_MASK;
    sys_reg.misc_b |= (val << IQS626_MISC_B_RESEED_UI_SEL_SHIFT);
    }
    sys_reg.misc_b &= ~IQS626_MISC_B_THRESH_EXTEND;
    if (device_property_present(&client.dev, "azoteq,thresh-extend"))
    sys_reg.misc_b |= IQS626_MISC_B_THRESH_EXTEND;
    sys_reg.misc_b &= ~IQS626_MISC_B_TRACKING_UI_ENABLE;
    if (device_property_present(&client.dev, "azoteq,tracking-enable"))
    sys_reg.misc_b |= IQS626_MISC_B_TRACKING_UI_ENABLE;
    sys_reg.misc_b &= ~IQS626_MISC_B_RESEED_OFFSET;
    if (device_property_present(&client.dev, "azoteq,reseed-offset"))
    sys_reg.misc_b |= IQS626_MISC_B_RESEED_OFFSET;
    if (!device_property_read_u32(&client.dev, "azoteq,rate-np-ms",
    &val)) {
    if (val > IQS626_RATE_NP_MS_MAX) {
    dev_err(&client.dev, "Invalid report rate: %u\n", val);
    return -EINVAL;
    }
    sys_reg.rate_np = val;
    }
    if (!device_property_read_u32(&client.dev, "azoteq,rate-lp-ms",
    &val)) {
    if (val > IQS626_RATE_LP_MS_MAX) {
    dev_err(&client.dev, "Invalid report rate: %u\n", val);
    return -EINVAL;
    }
    sys_reg.rate_lp = val;
    }
    if (!device_property_read_u32(&client.dev, "azoteq,rate-ulp-ms",
    &val)) {
    if (val > IQS626_RATE_ULP_MS_MAX) {
    dev_err(&client.dev, "Invalid report rate: %u\n", val);
    return -EINVAL;
    }
    sys_reg.rate_ulp = val / 16;
    }
    if (!device_property_read_u32(&client.dev, "azoteq,timeout-pwr-ms",
    &val)) {
    if (val > IQS626_TIMEOUT_PWR_MS_MAX) {
    dev_err(&client.dev, "Invalid timeout: %u\n", val);
    return -EINVAL;
    }
    sys_reg.timeout_pwr = val / 512;
    }
    if (!device_property_read_u32(&client.dev, "azoteq,timeout-lta-ms",
    &val)) {
    if (val > IQS626_TIMEOUT_LTA_MS_MAX) {
    dev_err(&client.dev, "Invalid timeout: %u\n", val);
    return -EINVAL;
    }
    sys_reg.timeout_lta = val / 512;
    }
    sys_reg.event_mask = ~((u8)IQS626_EVENT_MASK_SYS);
    sys_reg.redo_ati = 0;
    sys_reg.reseed = 0;
    sys_reg.active = 0;
    for (i = 0; i < ARRAY_SIZE(iqs626_channels); i++) {
    struct fwnode_handle *ch_node __free(fwnode_handle) =
    device_get_named_child_node(&client.dev,
    iqs626_channels[i].name);
    if (!ch_node)
    continue;
    error = iqs626_parse_channel(iqs626, ch_node, i);
    if (error)
    return error;
    sys_reg.active |= iqs626_channels[i].active;
    }
    general |= IQS626_SYS_SETTINGS_EVENT_MODE;
//
// Enable streaming during normal-power mode if the trackpad is used to
// report raw coordinates instead of gestures. In that case, the device
// returns to event mode during low-power mode.
//
    if (sys_reg.active & iqs626_channels[IQS626_CH_TP_2].active &&
    sys_reg.event_mask & IQS626_EVENT_MASK_GESTURE)
    general |= IQS626_SYS_SETTINGS_EVENT_MODE_LP;
    general |= IQS626_SYS_SETTINGS_REDO_ATI;
    general |= IQS626_SYS_SETTINGS_ACK_RESET;
    sys_reg.general = cpu_to_be16(general);
    error = regmap_raw_write(iqs626.regmap, IQS626_SYS_SETTINGS,
    &iqs626.sys_reg, sizeof(iqs626.sys_reg));
    if (error)
    return error;
    iqs626_irq_wait();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iqs626_input_init(iqs626: *mut iqs626_private) -> c_int {
    static int iqs626_input_init(struct iqs626_private *iqs626)
    {
    struct iqs626_sys_reg *sys_reg = &iqs626.sys_reg;
    struct i2c_client *client = iqs626.client;
    int error, i, j;
    iqs626.keypad = devm_input_allocate_device(&client.dev);
    if (!iqs626.keypad)
    return -ENOMEM;
    iqs626.keypad.keycodemax = ARRAY_SIZE(iqs626.kp_code);
    iqs626.keypad.keycode = iqs626.kp_code;
    iqs626.keypad.keycodesize = sizeof(**iqs626.kp_code);
    iqs626.keypad.name = "iqs626a_keypad";
    iqs626.keypad.id.bustype = BUS_I2C;
    for (i = 0; i < ARRAY_SIZE(iqs626_channels); i++) {
    if (!(sys_reg.active & iqs626_channels[i].active))
    continue;
    for (j = 0; j < ARRAY_SIZE(iqs626_events); j++) {
    if (!iqs626.kp_type[i][j])
    continue;
    input_set_capability(iqs626.keypad,
    iqs626.kp_type[i][j],
    iqs626.kp_code[i][j]);
    }
    }
    if (!(sys_reg.active & iqs626_channels[IQS626_CH_TP_2].active))
    return 0;
    iqs626.trackpad = devm_input_allocate_device(&client.dev);
    if (!iqs626.trackpad)
    return -ENOMEM;
    iqs626.trackpad.keycodemax = ARRAY_SIZE(iqs626.tp_code);
    iqs626.trackpad.keycode = iqs626.tp_code;
    iqs626.trackpad.keycodesize = sizeof(*iqs626.tp_code);
    iqs626.trackpad.name = "iqs626a_trackpad";
    iqs626.trackpad.id.bustype = BUS_I2C;
//
// Present the trackpad as a traditional pointing device if no gestures
// have been mapped to a keycode.
//
    if (sys_reg.event_mask & IQS626_EVENT_MASK_GESTURE) {
    let mut tp_mask: u8 = iqs626_channels[IQS626_CH_TP_3].active;
    input_set_capability(iqs626.trackpad, EV_KEY, BTN_TOUCH);
    input_set_abs_params(iqs626.trackpad, ABS_Y, 0, 255, 0, 0);
    if ((sys_reg.active & tp_mask) == tp_mask)
    input_set_abs_params(iqs626.trackpad,
    ABS_X, 0, 255, 0, 0);
    else
    input_set_abs_params(iqs626.trackpad,
    ABS_X, 0, 128, 0, 0);
    touchscreen_parse_properties(iqs626.trackpad, false,
    &iqs626.prop);
    } else {
    for (i = 0; i < IQS626_NUM_GESTURES; i++)
    if (iqs626.tp_code[i] != KEY_RESERVED)
    input_set_capability(iqs626.trackpad, EV_KEY,
    iqs626.tp_code[i]);
    }
    error = input_register_device(iqs626.trackpad);
    if (error)
    dev_err(&client.dev, "Failed to register trackpad: %d\n",
    error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn iqs626_report(iqs626: *mut iqs626_private) -> c_int {
    static int iqs626_report(struct iqs626_private *iqs626)
    {
    struct iqs626_sys_reg *sys_reg = &iqs626.sys_reg;
    struct i2c_client *client = iqs626.client;
    struct iqs626_flags flags;
    __le16 hall_output;
    int error, i, j;
    u8 state;
    u8 *dir_mask = &flags.states[IQS626_ST_OFFS_DIR];
    error = regmap_raw_read(iqs626.regmap, IQS626_SYS_FLAGS, &flags,
    sizeof(flags));
    if (error) {
    dev_err(&client.dev, "Failed to read device status: %d\n",
    error);
    return error;
    }
//
// The device resets itself if its own watchdog bites, which can happen
// in the event of an I2C communication error. In this case, the device
// asserts a SHOW_RESET interrupt and all registers must be restored.
//
    if (be16_to_cpu(flags.system) & IQS626_SYS_FLAGS_SHOW_RESET) {
    dev_err(&client.dev, "Unexpected device reset\n");
    error = regmap_raw_write(iqs626.regmap, IQS626_SYS_SETTINGS,
    sys_reg, sizeof(*sys_reg));
    if (error)
    dev_err(&client.dev,
    "Failed to re-initialize device: %d\n", error);
    return error;
    }
    if (be16_to_cpu(flags.system) & IQS626_SYS_FLAGS_IN_ATI)
    return 0;
//
// Unlike the ULP or generic channels, the Hall channel does not have a
// direction flag. Instead, the direction (i.e. magnet polarity) can be
// derived based on the sign of the 2's complement differential output.
//
    if (sys_reg.active & iqs626_channels[IQS626_CH_HALL].active) {
    error = regmap_raw_read(iqs626.regmap, IQS626_HALL_OUTPUT,
    &hall_output, sizeof(hall_output));
    if (error) {
    dev_err(&client.dev,
    "Failed to read Hall output: %d\n", error);
    return error;
    }
// dir_mask &= ~iqs626_channels[IQS626_CH_HALL].active;
    if (le16_to_cpu(hall_output) < 0x8000)
// dir_mask |= iqs626_channels[IQS626_CH_HALL].active;
    }
    for (i = 0; i < ARRAY_SIZE(iqs626_channels); i++) {
    if (!(sys_reg.active & iqs626_channels[i].active))
    continue;
    for (j = 0; j < ARRAY_SIZE(iqs626_events); j++) {
    if (!iqs626.kp_type[i][j])
    continue;
    state = flags.states[iqs626_events[j].st_offs];
    state &= iqs626_events[j].dir_up ? *dir_mask
    : ~(*dir_mask);
    state &= iqs626_channels[i].active;
    input_event(iqs626.keypad, iqs626.kp_type[i][j],
    iqs626.kp_code[i][j], !!state);
    }
    }
    input_sync(iqs626.keypad);
//
// The following completion signals that ATI has finished, any initial
// switch states have been reported and the keypad can be registered.
//
    complete_all(&iqs626.ati_done);
    if (!(sys_reg.active & iqs626_channels[IQS626_CH_TP_2].active))
    return 0;
    if (sys_reg.event_mask & IQS626_EVENT_MASK_GESTURE) {
    state = flags.states[IQS626_ST_OFFS_TOUCH];
    state &= iqs626_channels[IQS626_CH_TP_2].active;
    input_report_key(iqs626.trackpad, BTN_TOUCH, state);
    if (state)
    touchscreen_report_pos(iqs626.trackpad, &iqs626.prop,
    flags.trackpad_x,
    flags.trackpad_y, false);
    } else {
    for (i = 0; i < IQS626_NUM_GESTURES; i++)
    input_report_key(iqs626.trackpad, iqs626.tp_code[i],
    flags.gesture & BIT(i));
    if (flags.gesture & GENMASK(IQS626_GESTURE_TAP, 0)) {
    input_sync(iqs626.trackpad);
//
// Momentary gestures are followed by a complementary
// release cycle so as to emulate a full keystroke.
//
    for (i = 0; i < IQS626_GESTURE_HOLD; i++)
    input_report_key(iqs626.trackpad,
    iqs626.tp_code[i], 0);
    }
    }
    input_sync(iqs626.trackpad);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn iqs626_irq(irq: c_int, context: *mut c_void) -> irqreturn_t {
    static irqreturn_t iqs626_irq(int irq, void *context)
    {
    struct iqs626_private *iqs626 = context;
    if (iqs626_report(iqs626))
    return IRQ_NONE;
//
// The device does not deassert its interrupt (RDY) pin until shortly
// after receiving an I2C stop condition; the following delay ensures
// the interrupt handler does not return before this time.
//
    iqs626_irq_wait();
    return IRQ_HANDLED;
    }
    static const struct regmap_config iqs626_regmap_config = {
    .reg_bits = 8,
    .val_bits = 16,
    .max_register = IQS626_MAX_REG,
    };
#[no_mangle]
unsafe extern "C" fn iqs626_probe(client: *mut i2c_client) -> c_int {
    static int iqs626_probe(struct i2c_client *client)
    {
    struct iqs626_ver_info ver_info;
    struct iqs626_private *iqs626;
    int error;
    iqs626 = devm_kzalloc(&client.dev, sizeof(*iqs626), GFP_KERNEL);
    if (!iqs626)
    return -ENOMEM;
    i2c_set_clientdata(client, iqs626);
    iqs626.client = client;
    iqs626.regmap = devm_regmap_init_i2c(client, &iqs626_regmap_config);
    if (IS_ERR(iqs626.regmap)) {
    error = PTR_ERR(iqs626.regmap);
    dev_err(&client.dev, "Failed to initialize register map: %d\n",
    error);
    return error;
    }
    init_completion(&iqs626.ati_done);
    error = regmap_raw_read(iqs626.regmap, IQS626_VER_INFO, &ver_info,
    sizeof(ver_info));
    if (error)
    return error;
    if (ver_info.prod_num != IQS626_VER_INFO_PROD_NUM) {
    dev_err(&client.dev, "Unrecognized product number: 0x%02X\n",
    ver_info.prod_num);
    return -EINVAL;
    }
    error = iqs626_parse_prop(iqs626);
    if (error)
    return error;
    error = iqs626_input_init(iqs626);
    if (error)
    return error;
    error = devm_request_threaded_irq(&client.dev, client.irq,
    core::ptr::null_mut(), iqs626_irq, IRQF_ONESHOT,
    client.name, iqs626);
    if (error) {
    dev_err(&client.dev, "Failed to request IRQ: %d\n", error);
    return error;
    }
    if (!wait_for_completion_timeout(&iqs626.ati_done,
    msecs_to_jiffies(2000))) {
    dev_err(&client.dev, "Failed to complete ATI\n");
    return -ETIMEDOUT;
    }
//
// The keypad may include one or more switches and is not registered
// until ATI is complete and the initial switch states are read.
//
    error = input_register_device(iqs626.keypad);
    if (error)
    dev_err(&client.dev, "Failed to register keypad: %d\n", error);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn iqs626_suspend(dev: *mut device) -> c_int {
    static int iqs626_suspend(struct device *dev)
    {
    struct iqs626_private *iqs626 = dev_get_drvdata(dev);
    struct i2c_client *client = iqs626.client;
    unsigned int val;
    int error;
    if (!iqs626.suspend_mode)
    return 0;
    disable_irq(client.irq);
//
// Automatic power mode switching must be disabled before the device is
// forced into any particular power mode. In this case, the device will
// transition into normal-power mode.
//
    error = regmap_update_bits(iqs626.regmap, IQS626_SYS_SETTINGS,
    IQS626_SYS_SETTINGS_DIS_AUTO, ~0);
    if (error)
    goto err_irq;
//
// The following check ensures the device has completed its transition
// into normal-power mode before a manual mode switch is performed.
//
    error = regmap_read_poll_timeout(iqs626.regmap, IQS626_SYS_FLAGS, val,
    !(val & IQS626_SYS_FLAGS_PWR_MODE_MASK),
    IQS626_PWR_MODE_POLL_SLEEP_US,
    IQS626_PWR_MODE_POLL_TIMEOUT_US);
    if (error)
    goto err_irq;
    error = regmap_update_bits(iqs626.regmap, IQS626_SYS_SETTINGS,
    IQS626_SYS_SETTINGS_PWR_MODE_MASK,
    iqs626.suspend_mode <<
    IQS626_SYS_SETTINGS_PWR_MODE_SHIFT);
    if (error)
    goto err_irq;
//
// This last check ensures the device has completed its transition into
// the desired power mode to prevent any spurious interrupts from being
// triggered after iqs626_suspend has already returned.
//
    error = regmap_read_poll_timeout(iqs626.regmap, IQS626_SYS_FLAGS, val,
    (val & IQS626_SYS_FLAGS_PWR_MODE_MASK)
    == (iqs626.suspend_mode <<
    IQS626_SYS_FLAGS_PWR_MODE_SHIFT),
    IQS626_PWR_MODE_POLL_SLEEP_US,
    IQS626_PWR_MODE_POLL_TIMEOUT_US);
    err_irq:
    iqs626_irq_wait();
    enable_irq(client.irq);
    return error;
    }
#[no_mangle]
unsafe extern "C" fn iqs626_resume(dev: *mut device) -> c_int {
    static int iqs626_resume(struct device *dev)
    {
    struct iqs626_private *iqs626 = dev_get_drvdata(dev);
    struct i2c_client *client = iqs626.client;
    unsigned int val;
    int error;
    if (!iqs626.suspend_mode)
    return 0;
    disable_irq(client.irq);
    error = regmap_update_bits(iqs626.regmap, IQS626_SYS_SETTINGS,
    IQS626_SYS_SETTINGS_PWR_MODE_MASK, 0);
    if (error)
    goto err_irq;
//
// This check ensures the device has returned to normal-power mode
// before automatic power mode switching is re-enabled.
//
    error = regmap_read_poll_timeout(iqs626.regmap, IQS626_SYS_FLAGS, val,
    !(val & IQS626_SYS_FLAGS_PWR_MODE_MASK),
    IQS626_PWR_MODE_POLL_SLEEP_US,
    IQS626_PWR_MODE_POLL_TIMEOUT_US);
    if (error)
    goto err_irq;
    error = regmap_update_bits(iqs626.regmap, IQS626_SYS_SETTINGS,
    IQS626_SYS_SETTINGS_DIS_AUTO, 0);
    if (error)
    goto err_irq;
//
// This step reports any events that may have been "swallowed" as a
// result of polling PWR_MODE (which automatically acknowledges any
// pending interrupts).
//
    error = iqs626_report(iqs626);
    err_irq:
    iqs626_irq_wait();
    enable_irq(client.irq);
    return error;
    }
    static DEFINE_SIMPLE_DEV_PM_OPS(iqs626_pm, iqs626_suspend, iqs626_resume);
    static const struct of_device_id iqs626_of_match[] = {
    { .compatible = "azoteq,iqs626a" },
    { }
    };
    MODULE_DEVICE_TABLE(of, iqs626_of_match);
    static struct i2c_driver iqs626_i2c_driver = {
    .driver = {
    .name = "iqs626a",
    .of_match_table = iqs626_of_match,
    .pm = pm_sleep_ptr(&iqs626_pm),
    },
    .probe = iqs626_probe,
    };
    module_i2c_driver(iqs626_i2c_driver);
    MODULE_AUTHOR("Jeff LaBundy <jeff@labundy.com>");
    MODULE_DESCRIPTION("Azoteq IQS626A Capacitive Touch Controller");
    MODULE_LICENSE("GPL");
