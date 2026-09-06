//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/anx7411.c
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
// Driver for Analogix ANX7411 USB Type-C and PD controller
//
// Copyright(c) 2022, Analogix Semiconductor. All rights reserved.
//

pub const TCPC_ADDRESS1: c_uint = 0x58;
pub const TCPC_ADDRESS2: c_uint = 0x56;
pub const TCPC_ADDRESS3: c_uint = 0x54;
pub const TCPC_ADDRESS4: c_uint = 0x52;
pub const SPI_ADDRESS1: c_uint = 0x7e;
pub const SPI_ADDRESS2: c_uint = 0x6e;
pub const SPI_ADDRESS3: c_uint = 0x64;
pub const SPI_ADDRESS4: c_uint = 0x62;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anx7411_i2c_select {
    pub tcpc_address: u8,
    pub spi_address: u8,
}

pub const VID_ANALOGIX: c_uint = 0x1F29;
pub const PID_ANALOGIX: c_uint = 0x7411;
// TCPC register define
pub const ANALOG_CTRL_10: c_uint = 0xAA;
pub const STATUS_LEN: c_int = 2;
pub const ALERT_0: c_uint = 0xCB;

pub const MSG_LEN: c_int = 32;
pub const HEADER_LEN: c_int = 2;
pub const MSG_HEADER: c_uint = 0x00;
pub const MSG_TYPE: c_uint = 0x01;
pub const MSG_RAWDATA: c_uint = 0x02;
pub const MSG_LEN_MASK: c_uint = 0x1F;
pub const ALERT_1: c_uint = 0xCC;

pub const VBUS_THRESHOLD_H: c_uint = 0xDD;
pub const VBUS_THRESHOLD_L: c_uint = 0xDE;
pub const FW_CTRL_0: c_uint = 0xF0;

pub const VSAFE0: c_int = 0;

pub const FW_PARAM: c_uint = 0xF1;

pub const FW_CTRL_2: c_uint = 0xF7;

// SPI register define
pub const OCM_CTRL_0: c_uint = 0x6E;

pub const MAX_VOLTAGE: c_uint = 0xAC;
pub const MAX_POWER: c_uint = 0xAD;
pub const MIN_POWER: c_uint = 0xAE;
pub const REQUEST_VOLTAGE: c_uint = 0xAF;

pub const REQUEST_CURRENT: c_uint = 0xB1;

pub const CMD_SEND_BUF: c_uint = 0xC0;
pub const CMD_RECV_BUF: c_uint = 0xE0;
pub const REQ_VOL_20V_IN_100MV: c_uint = 0xC8;
pub const REQ_CUR_2_25A_IN_50MA: c_uint = 0x2D;
pub const REQ_CUR_3_25A_IN_50MA: c_uint = 0x41;
pub const DEF_5V: c_int = 5000;
pub const DEF_1_5A: c_int = 1500;

    enum anx7411_typec_message_type {
    TYPE_SRC_CAP = 0x00,
    TYPE_SNK_CAP = 0x01,
    TYPE_SNK_IDENTITY = 0x02,
    TYPE_SVID = 0x03,
    TYPE_SET_SNK_DP_CAP = 0x08,
    TYPE_PSWAP_REQ = 0x10,
    TYPE_DSWAP_REQ = 0x11,
    TYPE_VDM = 0x14,
    TYPE_OBJ_REQ = 0x16,
    TYPE_DP_ALT_ENTER = 0x19,
    TYPE_DP_DISCOVER_MODES_INFO = 0x27,
    TYPE_GET_DP_CONFIG = 0x29,
    TYPE_DP_CONFIGURE = 0x2A,
    TYPE_GET_DP_DISCOVER_MODES_INFO = 0x2E,
    TYPE_GET_DP_ALT_ENTER = 0x2F,
    };
pub const FW_CTRL_1: c_uint = 0xB2;

pub const FW_VER: c_uint = 0xB4;
pub const FW_SUBVER: c_uint = 0xB5;
pub const INT_MASK: c_uint = 0xB6;
pub const INT_STS: c_uint = 0xB7;

pub const SYSTEM_STSTUS: c_uint = 0xB8;
// 0: SINK off; 1: SINK on

// 0: VCONN off; 1: VCONN on

// 0: vbus off; 1: vbus on

// 1: host; 0:device

// 0: Chunking; 1: Unchunked

// 0: HPD low; 1: HPD high

pub const DATA_DFP: c_int = 1;
pub const DATA_UFP: c_int = 2;
pub const POWER_SOURCE: c_int = 1;
pub const POWER_SINK: c_int = 2;
pub const CC_STATUS: c_uint = 0xB9;

pub const PD_REV_INIT: c_uint = 0xBA;
pub const PD_EXT_MSG_CTRL: c_uint = 0xBB;

pub const NO_CONNECT: c_uint = 0x00;
pub const USB3_1_CONNECTED: c_uint = 0x01;
pub const DP_ALT_4LANES: c_uint = 0x02;
pub const USB3_1_DP_2LANES: c_uint = 0x03;
pub const CC1_CONNECTED: c_uint = 0x01;
pub const CC2_CONNECTED: c_uint = 0x02;
pub const SELECT_PIN_ASSIGMENT_C: c_uint = 0x04;
pub const SELECT_PIN_ASSIGMENT_D: c_uint = 0x08;
pub const SELECT_PIN_ASSIGMENT_E: c_uint = 0x10;
pub const SELECT_PIN_ASSIGMENT_U: c_uint = 0x00;
pub const REDRIVER_ADDRESS: c_uint = 0x20;
pub const REDRIVER_OFFSET: c_uint = 0x00;
pub const DP_SVID: c_uint = 0xFF01;
pub const VDM_ACK: c_uint = 0x40;
pub const VDM_CMD_RES: c_uint = 0x00;
pub const VDM_CMD_DIS_ID: c_uint = 0x01;
pub const VDM_CMD_DIS_SVID: c_uint = 0x02;
pub const VDM_CMD_DIS_MOD: c_uint = 0x03;
pub const VDM_CMD_ENTER_MODE: c_uint = 0x04;
pub const VDM_CMD_EXIT_MODE: c_uint = 0x05;
pub const VDM_CMD_ATTENTION: c_uint = 0x06;
pub const VDM_CMD_GET_STS: c_uint = 0x10;
pub const VDM_CMD_AND_ACK_MASK: c_uint = 0x5F;
pub const MAX_ALTMODE: c_int = 2;

    enum anx7411_psy_state {
// copy from drivers/usb/typec/tcpm
    ANX7411_PSY_OFFLINE = 0,
    ANX7411_PSY_FIXED_ONLINE,
// private
// PD keep in, but disconnct power to bq25700,
// this state can be active when higher capacity adapter plug in,
// and change to ONLINE state when higher capacity adapter plug out
//
    ANX7411_PSY_HANG = 0xff,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct typec_params {
    pub /: *mut *mut int request_current; / ma,
    pub /: *mut *mut int request_voltage; / mv,
    pub cc_connect: c_int,
    pub cc_orientation_valid: c_int,
    pub cc_status: c_int,
    pub data_role: c_int,
    pub power_role: c_int,
    pub vconn_role: c_int,
    pub dp_altmode_enter: c_int,
    pub cust_altmode_enter: c_int,
    pub role_sw: *mut usb_role_switch,
    pub port: *mut typec_port,
    pub partner: *mut typec_partner,
    pub typec_mux: *mut typec_mux_dev,
    pub typec_switch: *mut typec_switch_dev,
    pub amode: [*mut typec_altmode; MAX_ALTMODE],
    pub port_amode: [*mut typec_altmode; MAX_ALTMODE],
    pub data: typec_displayport_data,
    pub pin_assignment: c_int,
    pub caps: typec_capability,
    pub src_pdo: [u32; PDO_MAX_OBJECTS],
    pub sink_pdo: [u32; PDO_MAX_OBJECTS],
    pub caps_flags: u8,
    pub src_pdo_nr: u8,
    pub sink_pdo_nr: u8,
    pub sink_watt: u8,
    pub sink_voltage: u8,
}

pub const MAX_BUF_LEN: c_int = 30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_msg {
    pub msg_len: u8,
    pub msg_type: u8,
    pub buf: [u8; MAX_BUF_LEN],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anx7411_data {
    pub fw_version: c_int,
    pub fw_subversion: c_int,
    pub tcpc_client: *mut i2c_client,
    pub spi_client: *mut i2c_client,
    pub send_msg: fw_msg,
    pub recv_msg: fw_msg,
    pub intp_gpiod: *mut gpio_desc,
    pub connector_fwnode: *mut fwnode_handle,
    pub typec: typec_params,
    pub intp_irq: c_int,
    pub work: work_struct,
    pub workqueue: *mut workqueue_struct,
// Lock for interrupt work queue
    pub lock: mutex,
    pub psy_online: enum anx7411_psy_state,
    pub usb_type: enum power_supply_usb_type,
    pub psy: *mut power_supply,
    pub psy_desc: power_supply_desc,
    pub dev: *mut device,
    pub switch_node: *mut fwnode_handle,
    pub mux_node: *mut fwnode_handle,
}

    static u8 snk_identity[] = {
    LOBYTE(VID_ANALOGIX), HIBYTE(VID_ANALOGIX), 0x00, 0x82, /* snk_id_hdr */
    0x00, 0x00, 0x00, 0x00,                                 /* snk_cert */
    0x00, 0x00, LOBYTE(PID_ANALOGIX), HIBYTE(PID_ANALOGIX), /* 5snk_ama */
    };
    static u8 dp_caps[4] = {0xC6, 0x00, 0x00, 0x00};
    static int anx7411_reg_read(struct i2c_client *client,
    u8 reg_addr)
    {
    return i2c_smbus_read_byte_data(client, reg_addr);
    }
    static int anx7411_reg_block_read(struct i2c_client *client,
    u8 reg_addr, u8 len, u8 *buf)
    {
    return i2c_smbus_read_i2c_block_data(client, reg_addr, len, buf);
    }
    static int anx7411_reg_write(struct i2c_client *client,
    u8 reg_addr, u8 reg_val)
    {
    return i2c_smbus_write_byte_data(client, reg_addr, reg_val);
    }
    static int anx7411_reg_block_write(struct i2c_client *client,
    u8 reg_addr, u8 len, u8 *buf)
    {
    return i2c_smbus_write_i2c_block_data(client, reg_addr, len, buf);
    }
    static struct anx7411_i2c_select anx7411_i2c_addr[] = {
    {TCPC_ADDRESS1, SPI_ADDRESS1},
    {TCPC_ADDRESS2, SPI_ADDRESS2},
    {TCPC_ADDRESS3, SPI_ADDRESS3},
    {TCPC_ADDRESS4, SPI_ADDRESS4},
    };
#[no_mangle]
unsafe extern "C" fn anx7411_detect_power_mode(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_detect_power_mode(struct anx7411_data *ctx)
    {
    int ret;
    int mode;
    ret = anx7411_reg_read(ctx.spi_client, REQUEST_CURRENT);
    if (ret < 0)
    return ret;
    ctx.typec.request_current = ret * CURRENT_UNIT; /* 50ma per unit */
    ret = anx7411_reg_read(ctx.spi_client, REQUEST_VOLTAGE);
    if (ret < 0)
    return ret;
    ctx.typec.request_voltage = ret * VOLTAGE_UNIT; /* 100mv per unit */
    if (ctx.psy_online == ANX7411_PSY_OFFLINE) {
    ctx.psy_online = ANX7411_PSY_FIXED_ONLINE;
    ctx.usb_type = POWER_SUPPLY_USB_TYPE_PD;
    power_supply_changed(ctx.psy);
    }
    if (!ctx.typec.cc_orientation_valid)
    return 0;
    if (ctx.typec.cc_connect == CC1_CONNECTED)
    mode = CC1_RP(ctx.typec.cc_status);
    else
    mode = CC2_RP(ctx.typec.cc_status);
    if (mode) {
    typec_set_pwr_opmode(ctx.typec.port, mode - 1);
    return 0;
    }
    typec_set_pwr_opmode(ctx.typec.port, TYPEC_PWR_MODE_PD);
    return 0;
    }
    static int anx7411_register_partner(struct anx7411_data *ctx,
    int pd, int accessory)
    {
    struct typec_partner_desc desc;
    struct typec_partner *partner;
    if (ctx.typec.partner)
    return 0;
    desc.usb_pd = pd;
    desc.accessory = accessory;
    desc.identity = core::ptr::null_mut();
    partner = typec_register_partner(ctx.typec.port, &desc);
    if (IS_ERR(partner))
    return PTR_ERR(partner);
    ctx.typec.partner = partner;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_detect_cc_orientation(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_detect_cc_orientation(struct anx7411_data *ctx)
    {
    struct device *dev = &ctx.spi_client.dev;
    int ret;
    int cc1_rd, cc2_rd;
    int cc1_ra, cc2_ra;
    int cc1_rp, cc2_rp;
    ret = anx7411_reg_read(ctx.spi_client, CC_STATUS);
    if (ret < 0)
    return ret;
    ctx.typec.cc_status = ret;
    cc1_rd = ret & CC1_RD ? 1 : 0;
    cc2_rd = ret & CC2_RD ? 1 : 0;
    cc1_ra = ret & CC1_RA ? 1 : 0;
    cc2_ra = ret & CC2_RA ? 1 : 0;
    cc1_rp = CC1_RP(ret);
    cc2_rp = CC2_RP(ret);
// Debug cable, nothing to do
    if (cc1_rd && cc2_rd) {
    ctx.typec.cc_orientation_valid = 0;
    return anx7411_register_partner(ctx, 0, TYPEC_ACCESSORY_DEBUG);
    }
    if (cc1_ra && cc2_ra) {
    ctx.typec.cc_orientation_valid = 0;
    return anx7411_register_partner(ctx, 0, TYPEC_ACCESSORY_AUDIO);
    }
    ctx.typec.cc_orientation_valid = 1;
    ret = anx7411_register_partner(ctx, 1, TYPEC_ACCESSORY_NONE);
    if (ret) {
    dev_err(dev, "register partner\n");
    return ret;
    }
    if (cc1_rd || cc1_rp) {
    typec_set_orientation(ctx.typec.port, TYPEC_ORIENTATION_NORMAL);
    ctx.typec.cc_connect = CC1_CONNECTED;
    }
    if (cc2_rd || cc2_rp) {
    typec_set_orientation(ctx.typec.port, TYPEC_ORIENTATION_REVERSE);
    ctx.typec.cc_connect = CC2_CONNECTED;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_set_mux(ctx: *mut anx7411_data, pin_assignment: c_int) -> c_int {
    static int anx7411_set_mux(struct anx7411_data *ctx, int pin_assignment)
    {
    let mut mode: c_int = TYPEC_STATE_SAFE;
    switch (pin_assignment) {
    case SELECT_PIN_ASSIGMENT_U:
// default 4 line USB 3.1
    mode = TYPEC_STATE_MODAL;
    break;
    case SELECT_PIN_ASSIGMENT_C:
    case SELECT_PIN_ASSIGMENT_E:
// 4 line DP
    mode = TYPEC_STATE_SAFE;
    break;
    case SELECT_PIN_ASSIGMENT_D:
// 2 line DP, 2 line USB
    mode = TYPEC_MODE_USB3;
    break;
    default:
    mode = TYPEC_STATE_SAFE;
    break;
    }
    ctx.typec.pin_assignment = pin_assignment;
    return typec_set_mode(ctx.typec.port, mode);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_set_usb_role(ctx: *mut anx7411_data, role: enum usb_role) -> c_int {
    static int anx7411_set_usb_role(struct anx7411_data *ctx, enum usb_role role)
    {
    if (!ctx.typec.role_sw)
    return 0;
    return usb_role_switch_set_role(ctx.typec.role_sw, role);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_data_role_detect(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_data_role_detect(struct anx7411_data *ctx)
    {
    int ret;
    ret = anx7411_reg_read(ctx.spi_client, SYSTEM_STSTUS);
    if (ret < 0)
    return ret;
    ctx.typec.data_role = (ret & DATA_ROLE) ? TYPEC_HOST : TYPEC_DEVICE;
    ctx.typec.vconn_role = (ret & VCONN_STATUS) ? TYPEC_SOURCE : TYPEC_SINK;
    typec_set_data_role(ctx.typec.port, ctx.typec.data_role);
    typec_set_vconn_role(ctx.typec.port, ctx.typec.vconn_role);
    if (ctx.typec.data_role == TYPEC_HOST)
    return anx7411_set_usb_role(ctx, USB_ROLE_HOST);
    return anx7411_set_usb_role(ctx, USB_ROLE_DEVICE);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_power_role_detect(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_power_role_detect(struct anx7411_data *ctx)
    {
    int ret;
    ret = anx7411_reg_read(ctx.spi_client, SYSTEM_STSTUS);
    if (ret < 0)
    return ret;
    ctx.typec.power_role = (ret & SINK_STATUS) ? TYPEC_SINK : TYPEC_SOURCE;
    if (ctx.typec.power_role == TYPEC_SOURCE) {
    ctx.typec.request_current = DEF_1_5A;
    ctx.typec.request_voltage = DEF_5V;
    }
    typec_set_pwr_role(ctx.typec.port, ctx.typec.power_role);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_cc_status_detect(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_cc_status_detect(struct anx7411_data *ctx)
    {
    anx7411_detect_cc_orientation(ctx);
    anx7411_detect_power_mode(ctx);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_partner_unregister_altmode(ctx: *mut anx7411_data) {
    static void anx7411_partner_unregister_altmode(struct anx7411_data *ctx)
    {
    int i;
    ctx.typec.dp_altmode_enter = 0;
    ctx.typec.cust_altmode_enter = 0;
    for (i = 0; i < MAX_ALTMODE; i++)
    if (ctx.typec.amode[i]) {
    typec_unregister_altmode(ctx.typec.amode[i]);
    ctx.typec.amode[i] = core::ptr::null_mut();
    }
    ctx.typec.pin_assignment = 0;
    }
    static int anx7411_typec_register_altmode(struct anx7411_data *ctx,
    int svid, int vdo)
    {
    struct device *dev = &ctx.spi_client.dev;
    struct typec_altmode_desc desc;
    int err;
    int i;
    desc.svid = svid;
    desc.vdo = vdo;
    for (i = 0; i < MAX_ALTMODE; i++)
    if (!ctx.typec.amode[i])
    break;
    desc.mode = i + 1; /* start with 1 */
    if (i >= MAX_ALTMODE) {
    dev_err(dev, "no altmode space for registering\n");
    return -ENOMEM;
    }
    ctx.typec.amode[i] = typec_partner_register_altmode(ctx.typec.partner,
    &desc);
    if (IS_ERR(ctx.typec.amode[i])) {
    dev_err(dev, "failed to register altmode\n");
    err = PTR_ERR(ctx.typec.amode[i]);
    ctx.typec.amode[i] = core::ptr::null_mut();
    return err;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_unregister_partner(ctx: *mut anx7411_data) {
    static void anx7411_unregister_partner(struct anx7411_data *ctx)
    {
    if (ctx.typec.partner) {
    typec_unregister_partner(ctx.typec.partner);
    ctx.typec.partner = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn anx7411_update_altmode(ctx: *mut anx7411_data, svid: c_int) -> c_int {
    static int anx7411_update_altmode(struct anx7411_data *ctx, int svid)
    {
    int i;
    if (svid == DP_SVID)
    ctx.typec.dp_altmode_enter = 1;
    else
    ctx.typec.cust_altmode_enter = 1;
    for (i = 0; i < MAX_ALTMODE; i++) {
    if (!ctx.typec.amode[i])
    continue;
    if (ctx.typec.amode[i].svid == svid) {
    typec_altmode_update_active(ctx.typec.amode[i], true);
    typec_altmode_notify(ctx.typec.amode[i],
    ctx.typec.pin_assignment,
    &ctx.typec.data);
    break;
    }
    }
    return 0;
    }
    static int anx7411_register_altmode(struct anx7411_data *ctx,
    bool dp_altmode, u8 *buf)
    {
    int ret;
    int svid;
    int mid;
    if (!ctx.typec.partner)
    return 0;
    svid = DP_SVID;
    if (dp_altmode) {
    mid = buf[0] | (buf[1] << 8) | (buf[2] << 16) | (buf[3] << 24);
    return anx7411_typec_register_altmode(ctx, svid, mid);
    }
    svid = (buf[3] << 8) | buf[2];
    if ((buf[0] & VDM_CMD_AND_ACK_MASK) != (VDM_ACK | VDM_CMD_ENTER_MODE))
    return anx7411_update_altmode(ctx, svid);
    if ((buf[0] & VDM_CMD_AND_ACK_MASK) != (VDM_ACK | VDM_CMD_DIS_MOD))
    return 0;
    mid = buf[4] | (buf[5] << 8) | (buf[6] << 16) | (buf[7] << 24);
    ret = anx7411_typec_register_altmode(ctx, svid, mid);
    if (ctx.typec.cust_altmode_enter)
    ret |= anx7411_update_altmode(ctx, svid);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_parse_cmd(ctx: *mut anx7411_data, type: u8, buf: *mut u8, len: u8) -> c_int {
    static int anx7411_parse_cmd(struct anx7411_data *ctx, u8 type, u8 *buf, u8 len)
    {
    struct device *dev = &ctx.spi_client.dev;
    u8 cur_50ma, vol_100mv;
    switch (type) {
    case TYPE_SRC_CAP:
    cur_50ma = anx7411_reg_read(ctx.spi_client, REQUEST_CURRENT);
    vol_100mv = anx7411_reg_read(ctx.spi_client, REQUEST_VOLTAGE);
    ctx.typec.request_voltage = vol_100mv * VOLTAGE_UNIT;
    ctx.typec.request_current = cur_50ma * CURRENT_UNIT;
    ctx.psy_online = ANX7411_PSY_FIXED_ONLINE;
    ctx.usb_type = POWER_SUPPLY_USB_TYPE_PD;
    power_supply_changed(ctx.psy);
    break;
    case TYPE_SNK_CAP:
    break;
    case TYPE_SVID:
    break;
    case TYPE_SNK_IDENTITY:
    break;
    case TYPE_GET_DP_ALT_ENTER:
// DP alt mode enter success
    if (buf[0])
    anx7411_update_altmode(ctx, DP_SVID);
    break;
    case TYPE_DP_ALT_ENTER:
// Update DP altmode
    anx7411_update_altmode(ctx, DP_SVID);
    break;
    case TYPE_OBJ_REQ:
    anx7411_detect_power_mode(ctx);
    break;
    case TYPE_DP_CONFIGURE:
    anx7411_set_mux(ctx, buf[1]);
    break;
    case TYPE_DP_DISCOVER_MODES_INFO:
// Make sure discover modes valid
    if (buf[0] | buf[1])
// Register DP Altmode
    anx7411_register_altmode(ctx, 1, buf);
    break;
    case TYPE_VDM:
// Register other altmode
    anx7411_register_altmode(ctx, 0, buf);
    break;
    default:
    dev_err(dev, "ignore message(0x%.02x).\n", type);
    break;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn checksum(dev: *mut device, buf: *mut u8, len: u8) -> u8 {
    static u8 checksum(struct device *dev, u8 *buf, u8 len)
    {
    let mut ret: u8 = 0;
    u8 i;
    for (i = 0; i < len; i++)
    ret += buf[i];
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_read_msg_ctrl_status(client: *mut i2c_client) -> c_int {
    static int anx7411_read_msg_ctrl_status(struct i2c_client *client)
    {
    return anx7411_reg_read(client, CMD_SEND_BUF);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_wait_msg_empty(client: *mut i2c_client) -> c_int {
    static int anx7411_wait_msg_empty(struct i2c_client *client)
    {
    int val;
    return readx_poll_timeout(anx7411_read_msg_ctrl_status,
    client, val, (val < 0) || (val == 0),
    2000, 2000 * 150);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_send_msg(ctx: *mut anx7411_data, type: u8, buf: *mut u8, size: u8) -> c_int {
    static int anx7411_send_msg(struct anx7411_data *ctx, u8 type, u8 *buf, u8 size)
    {
    struct device *dev = &ctx.spi_client.dev;
    struct fw_msg *msg = &ctx.send_msg;
    u8 crc;
    int ret;
    size = min_t(u8, size, (u8)MAX_BUF_LEN);
    memcpy(msg.buf, buf, size);
    msg.msg_type = type;
// msg len equals buffer length + msg_type
    msg.msg_len = size + 1;
// Do CRC check for all buffer data and msg_len and msg_type
    crc = checksum(dev, (u8 *)msg, size + HEADER_LEN);
    msg.buf[size] = 0 - crc;
    ret = anx7411_wait_msg_empty(ctx.spi_client);
    if (ret)
    return ret;
    ret = anx7411_reg_block_write(ctx.spi_client,
    CMD_SEND_BUF + 1, size + HEADER_LEN,
    &msg.msg_type);
    ret |= anx7411_reg_write(ctx.spi_client, CMD_SEND_BUF,
    msg.msg_len);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_process_cmd(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_process_cmd(struct anx7411_data *ctx)
    {
    struct device *dev = &ctx.spi_client.dev;
    struct fw_msg *msg = &ctx.recv_msg;
    u8 len;
    u8 crc;
    int ret;
// Read message from firmware
    ret = anx7411_reg_block_read(ctx.spi_client, CMD_RECV_BUF,
    MSG_LEN, (u8 *)msg);
    if (ret < 0)
    return 0;
    if (!msg.msg_len)
    return 0;
    ret = anx7411_reg_write(ctx.spi_client, CMD_RECV_BUF, 0);
    if (ret)
    return ret;
    len = msg.msg_len & MSG_LEN_MASK;
    crc = checksum(dev, (u8 *)msg, len + HEADER_LEN);
    if (crc) {
    dev_err(dev, "message error crc(0x%.02x)\n", crc);
    return -ERANGE;
    }
    return anx7411_parse_cmd(ctx, msg.msg_type, msg.buf, len - 1);
    }
    static void anx7411_translate_payload(struct device *dev, __le32 *payload,
    u32 *pdo, int nr, const char *type)
    {
    int i;
    if (nr > PDO_MAX_OBJECTS) {
    dev_err(dev, "nr(%d) exceed PDO_MAX_OBJECTS(%d)\n",
    nr, PDO_MAX_OBJECTS);
    return;
    }
    for (i = 0; i < nr; i++)
    payload[i] = cpu_to_le32(pdo[i]);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_config(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_config(struct anx7411_data *ctx)
    {
    struct device *dev = &ctx.spi_client.dev;
    struct typec_params *typecp = &ctx.typec;
    __le32 payload[PDO_MAX_OBJECTS];
    int ret;
// Config PD FW work under PD 2.0
    ret = anx7411_reg_write(ctx.spi_client, PD_REV_INIT, PD_REV20);
    ret |= anx7411_reg_write(ctx.tcpc_client, FW_CTRL_0,
    UNSTRUCT_VDM_EN | DELAY_200MS |
    VSAFE1 | FRS_EN);
    ret |= anx7411_reg_write(ctx.spi_client, FW_CTRL_1,
    AUTO_PD_EN | FORCE_SEND_RDO);
// Set VBUS current threshold
    ret |= anx7411_reg_write(ctx.tcpc_client, VBUS_THRESHOLD_H, 0xff);
    ret |= anx7411_reg_write(ctx.tcpc_client, VBUS_THRESHOLD_L, 0x03);
// Fix dongle compatible issue
    ret |= anx7411_reg_write(ctx.tcpc_client, FW_PARAM,
    anx7411_reg_read(ctx.tcpc_client, FW_PARAM) |
    DONGLE_IOP);
    ret |= anx7411_reg_write(ctx.spi_client, INT_MASK, 0);
    ret |= anx7411_reg_write(ctx.spi_client, PD_EXT_MSG_CTRL, 0xFF);
    if (ret)
    return ret;
    if (typecp.caps_flags & HAS_SOURCE_CAP) {
    anx7411_translate_payload(dev, payload, typecp.src_pdo,
    typecp.src_pdo_nr, "source");
    anx7411_send_msg(ctx, TYPE_SRC_CAP, (u8 *)&payload,
    typecp.src_pdo_nr * 4);
    anx7411_send_msg(ctx, TYPE_SNK_IDENTITY, snk_identity,
    sizeof(snk_identity));
    anx7411_send_msg(ctx, TYPE_SET_SNK_DP_CAP, dp_caps,
    sizeof(dp_caps));
    }
    if (typecp.caps_flags & HAS_SINK_CAP) {
    anx7411_translate_payload(dev, payload, typecp.sink_pdo,
    typecp.sink_pdo_nr, "sink");
    anx7411_send_msg(ctx, TYPE_SNK_CAP, (u8 *)&payload,
    typecp.sink_pdo_nr * 4);
    }
    if (typecp.caps_flags & HAS_SINK_WATT) {
    if (typecp.sink_watt) {
    ret |= anx7411_reg_write(ctx.spi_client, MAX_POWER,
    typecp.sink_watt);
// Set min power to 1W
    ret |= anx7411_reg_write(ctx.spi_client, MIN_POWER, 2);
    }
    if (typecp.sink_voltage)
    ret |= anx7411_reg_write(ctx.spi_client, MAX_VOLTAGE,
    typecp.sink_voltage);
    if (ret)
    return ret;
    }
    if (!typecp.caps_flags)
    usleep_range(5000, 6000);
    ctx.fw_version = anx7411_reg_read(ctx.spi_client, FW_VER);
    ctx.fw_subversion = anx7411_reg_read(ctx.spi_client, FW_SUBVER);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_chip_standby(ctx: *mut anx7411_data) {
    static void anx7411_chip_standby(struct anx7411_data *ctx)
    {
    int ret;
    u8 cc1, cc2;
    struct device *dev = &ctx.spi_client.dev;
    ret = anx7411_reg_write(ctx.spi_client, OCM_CTRL_0,
    anx7411_reg_read(ctx.spi_client, OCM_CTRL_0) |
    OCM_RESET);
    ret |= anx7411_reg_write(ctx.tcpc_client, ANALOG_CTRL_10, 0x80);
// Set TCPC to RD and DRP enable
    cc1 = FIELD_PREP(TCPC_ROLE_CTRL_CC1, TCPC_ROLE_CTRL_CC_RD);
    cc2 = FIELD_PREP(TCPC_ROLE_CTRL_CC2, TCPC_ROLE_CTRL_CC_RD);
    ret |= anx7411_reg_write(ctx.tcpc_client, TCPC_ROLE_CTRL,
    TCPC_ROLE_CTRL_DRP | cc1 | cc2);
// Send DRP toggle command
    ret |= anx7411_reg_write(ctx.tcpc_client, TCPC_COMMAND,
    TCPC_CMD_LOOK4CONNECTION);
// Send TCPC enter standby command
    ret |= anx7411_reg_write(ctx.tcpc_client,
    TCPC_COMMAND, TCPC_CMD_I2C_IDLE);
    if (ret)
    dev_err(dev, "Chip standby failed\n");
    }
#[no_mangle]
unsafe extern "C" fn anx7411_work_func(work: *mut work_struct) {
    static void anx7411_work_func(struct work_struct *work)
    {
    int ret;
    u8 buf[STATUS_LEN];
    u8 int_change; /* Interrupt change */
    u8 int_status; /* Firmware status update */
    u8 alert0, alert1; /* Interrupt alert source */
    struct anx7411_data *ctx = container_of(work, struct anx7411_data, work);
    struct device *dev = &ctx.spi_client.dev;
    mutex_lock(&ctx.lock);
// Read interrupt change status
    ret = anx7411_reg_block_read(ctx.spi_client, INT_STS, STATUS_LEN, buf);
    if (ret < 0) {
// Power standby mode, just return
    goto unlock;
    }
    int_change = buf[0];
    int_status = buf[1];
// Read alert register
    ret = anx7411_reg_block_read(ctx.tcpc_client, ALERT_0, STATUS_LEN, buf);
    if (ret < 0)
    goto unlock;
    alert0 = buf[0];
    alert1 = buf[1];
// Clear interrupt and alert status
    ret = anx7411_reg_write(ctx.spi_client, INT_STS, 0);
    ret |= anx7411_reg_write(ctx.tcpc_client, ALERT_0, alert0);
    ret |= anx7411_reg_write(ctx.tcpc_client, ALERT_1, alert1);
    if (ret)
    goto unlock;
    if (alert1 & INTP_POW_OFF) {
    anx7411_partner_unregister_altmode(ctx);
    if (anx7411_set_usb_role(ctx, USB_ROLE_NONE))
    dev_err(dev, "Set usb role\n");
    anx7411_unregister_partner(ctx);
    ctx.psy_online = ANX7411_PSY_OFFLINE;
    ctx.usb_type = POWER_SUPPLY_USB_TYPE_C;
    ctx.typec.request_voltage = 0;
    ctx.typec.request_current = 0;
    power_supply_changed(ctx.psy);
    anx7411_chip_standby(ctx);
    goto unlock;
    }
    if ((alert0 & SOFTWARE_INT) && (int_change & OCM_BOOT_UP)) {
    if (anx7411_config(ctx))
    dev_err(dev, "Config failed\n");
    if (anx7411_data_role_detect(ctx))
    dev_err(dev, "set PD data role\n");
    if (anx7411_power_role_detect(ctx))
    dev_err(dev, "set PD power role\n");
    anx7411_set_mux(ctx, SELECT_PIN_ASSIGMENT_C);
    }
    if (alert0 & RECEIVED_MSG)
    anx7411_process_cmd(ctx);
    ret = (int_status & DATA_ROLE) ? TYPEC_HOST : TYPEC_DEVICE;
    if (ctx.typec.data_role != ret)
    if (anx7411_data_role_detect(ctx))
    dev_err(dev, "set PD data role\n");
    ret = (int_status & SINK_STATUS) ? TYPEC_SINK : TYPEC_SOURCE;
    if (ctx.typec.power_role != ret)
    if (anx7411_power_role_detect(ctx))
    dev_err(dev, "set PD power role\n");
    if ((alert0 & SOFTWARE_INT) && (int_change & CC_STATUS_CHANGE))
    anx7411_cc_status_detect(ctx);
    unlock:
    mutex_unlock(&ctx.lock);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_intr_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t anx7411_intr_isr(int irq, void *data)
    {
    struct anx7411_data *ctx = (struct anx7411_data *)data;
    queue_work(ctx.workqueue, &ctx.work);
    return IRQ_HANDLED;
    }
    static int anx7411_register_i2c_dummy_clients(struct anx7411_data *ctx,
    struct i2c_client *client)
    {
    int i;
    u8 spi_addr;
    for (i = 0; i < ARRAY_SIZE(anx7411_i2c_addr); i++) {
    if (client.addr == (anx7411_i2c_addr[i].tcpc_address >> 1)) {
    spi_addr = anx7411_i2c_addr[i].spi_address >> 1;
    ctx.spi_client = i2c_new_dummy_device(client.adapter,
    spi_addr);
    if (!IS_ERR(ctx.spi_client))
    return 0;
    }
    }
    dev_err(&client.dev, "unable to get SPI slave\n");
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_port_unregister_altmodes(adev: *mut typec_altmode) {
    static void anx7411_port_unregister_altmodes(struct typec_altmode **adev)
    {
    int i;
    for (i = 0; i < MAX_ALTMODE; i++)
    if (adev[i]) {
    typec_unregister_altmode(adev[i]);
    adev[i] = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn anx7411_port_unregister(typecp: *mut typec_params) {
    static void anx7411_port_unregister(struct typec_params *typecp)
    {
    fwnode_handle_put(typecp.caps.fwnode);
    anx7411_port_unregister_altmodes(typecp.port_amode);
    if (typecp.port)
    typec_unregister_port(typecp.port);
    if (typecp.role_sw)
    usb_role_switch_put(typecp.role_sw);
    }
    static int anx7411_usb_mux_set(struct typec_mux_dev *mux,
    struct typec_mux_state *state)
    {
    struct anx7411_data *ctx = typec_mux_get_drvdata(mux);
    struct device *dev = &ctx.spi_client.dev;
    int has_dp;
    has_dp = (state.alt && state.alt.svid == USB_TYPEC_DP_SID &&
    state.alt.mode == USB_TYPEC_DP_MODE);
    if (!has_dp)
    dev_err(dev, "dp altmode not register\n");
    return 0;
    }
    static int anx7411_usb_set_orientation(struct typec_switch_dev *sw,
    enum typec_orientation orientation)
    {
// No need set
    return 0;
    }
    static int anx7411_register_switch(struct anx7411_data *ctx,
    struct device *dev,
    struct fwnode_handle *fwnode)
    {
    let mut sw_desc: typec_switch_desc = { };
    sw_desc.fwnode = fwnode;
    sw_desc.drvdata = ctx;
    sw_desc.name = fwnode_get_name(fwnode);
    sw_desc.set = anx7411_usb_set_orientation;
    ctx.typec.typec_switch = typec_switch_register(dev, &sw_desc);
    if (IS_ERR(ctx.typec.typec_switch)) {
    dev_err(dev, "switch register failed\n");
    return PTR_ERR(ctx.typec.typec_switch);
    }
    return 0;
    }
    static int anx7411_register_mux(struct anx7411_data *ctx,
    struct device *dev,
    struct fwnode_handle *fwnode)
    {
    let mut mux_desc: typec_mux_desc = { };
    mux_desc.fwnode = fwnode;
    mux_desc.drvdata = ctx;
    mux_desc.name = fwnode_get_name(fwnode);
    mux_desc.set = anx7411_usb_mux_set;
    ctx.typec.typec_mux = typec_mux_register(dev, &mux_desc);
    if (IS_ERR(ctx.typec.typec_mux)) {
    dev_err(dev, "mux register failed\n");
    return PTR_ERR(ctx.typec.typec_mux);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_unregister_mux(ctx: *mut anx7411_data) {
    static void anx7411_unregister_mux(struct anx7411_data *ctx)
    {
    if (ctx.typec.typec_mux) {
    typec_mux_unregister(ctx.typec.typec_mux);
    ctx.typec.typec_mux = core::ptr::null_mut();
    fwnode_handle_put(ctx.mux_node);
    }
    }
#[no_mangle]
unsafe extern "C" fn anx7411_unregister_switch(ctx: *mut anx7411_data) {
    static void anx7411_unregister_switch(struct anx7411_data *ctx)
    {
    if (ctx.typec.typec_switch) {
    typec_switch_unregister(ctx.typec.typec_switch);
    ctx.typec.typec_switch = core::ptr::null_mut();
    fwnode_handle_put(ctx.switch_node);
    }
    }
    static int anx7411_typec_switch_probe(struct anx7411_data *ctx,
    struct device *dev)
    {
    int ret;
    ctx.switch_node = device_get_named_child_node(dev, "orientation_switch");
    if (!ctx.switch_node)
    return 0;
    ret = anx7411_register_switch(ctx, dev, ctx.switch_node);
    if (ret) {
    dev_err(dev, "failed register switch");
    fwnode_handle_put(ctx.switch_node);
    return ret;
    }
    ctx.mux_node = device_get_named_child_node(dev, "mode_switch");
    if (!ctx.mux_node) {
    dev_err(dev, "no typec mux exist");
    ret = -ENODEV;
    goto unregister_switch;
    }
    ret = anx7411_register_mux(ctx, dev, ctx.mux_node);
    if (ret) {
    dev_err(dev, "failed register mode switch");
    fwnode_handle_put(ctx.mux_node);
    ret = -ENODEV;
    goto unregister_switch;
    }
    return 0;
    unregister_switch:
    anx7411_unregister_switch(ctx);
    return ret;
    }
    static int anx7411_typec_port_probe(struct anx7411_data *ctx,
    struct device *dev)
    {
    struct typec_capability *cap = &ctx.typec.caps;
    struct typec_params *typecp = &ctx.typec;
    struct fwnode_handle *fwnode;
    const char *buf;
    int ret, i;
    fwnode = device_get_named_child_node(dev, "connector");
    if (!fwnode)
    return -EINVAL;
    ret = fwnode_property_read_string(fwnode, "power-role", &buf);
    if (ret) {
    dev_err(dev, "power-role not found: %d\n", ret);
    goto put_fwnode;
    }
    ret = typec_find_port_power_role(buf);
    if (ret < 0)
    goto put_fwnode;
    cap.type = ret;
    ret = fwnode_property_read_string(fwnode, "data-role", &buf);
    if (ret) {
    dev_err(dev, "data-role not found: %d\n", ret);
    goto put_fwnode;
    }
    ret = typec_find_port_data_role(buf);
    if (ret < 0)
    goto put_fwnode;
    cap.data = ret;
    ret = fwnode_property_read_string(fwnode, "try-power-role", &buf);
    if (ret) {
    dev_err(dev, "try-power-role not found: %d\n", ret);
    goto put_fwnode;
    }
    ret = typec_find_power_role(buf);
    if (ret < 0)
    goto put_fwnode;
    cap.prefer_role = ret;
// Get source pdos
    ret = fwnode_property_count_u32(fwnode, "source-pdos");
    if (ret > 0) {
    typecp.src_pdo_nr = min_t(u8, ret, PDO_MAX_OBJECTS);
    ret = fwnode_property_read_u32_array(fwnode, "source-pdos",
    typecp.src_pdo,
    typecp.src_pdo_nr);
    if (ret < 0) {
    dev_err(dev, "source cap validate failed: %d\n", ret);
    goto put_fwnode;
    }
    typecp.caps_flags |= HAS_SOURCE_CAP;
    }
    ret = fwnode_property_count_u32(fwnode, "sink-pdos");
    if (ret > 0) {
    typecp.sink_pdo_nr = min_t(u8, ret, PDO_MAX_OBJECTS);
    ret = fwnode_property_read_u32_array(fwnode, "sink-pdos",
    typecp.sink_pdo,
    typecp.sink_pdo_nr);
    if (ret < 0) {
    dev_err(dev, "sink cap validate failed: %d\n", ret);
    goto put_fwnode;
    }
    for (i = 0; i < typecp.sink_pdo_nr; i++) {
    ret = 0;
    switch (pdo_type(typecp.sink_pdo[i])) {
    case PDO_TYPE_FIXED:
    ret = pdo_fixed_voltage(typecp.sink_pdo[i]);
    break;
    case PDO_TYPE_BATT:
    case PDO_TYPE_VAR:
    ret = pdo_max_voltage(typecp.sink_pdo[i]);
    break;
    case PDO_TYPE_APDO:
    default:
    ret = 0;
    break;
    }
// 100mv per unit
    typecp.sink_voltage = max(5000, ret) / 100;
    }
    typecp.caps_flags |= HAS_SINK_CAP;
    }
    if (!fwnode_property_read_u32(fwnode, "op-sink-microwatt", &ret)) {
    typecp.sink_watt = ret / 500000; /* 500mw per unit */
    typecp.caps_flags |= HAS_SINK_WATT;
    }
    cap.fwnode = fwnode;
    ctx.typec.role_sw = usb_role_switch_get(dev);
    if (IS_ERR(ctx.typec.role_sw)) {
    dev_err(dev, "USB role switch not found.\n");
    ctx.typec.role_sw = core::ptr::null_mut();
    }
    ctx.typec.port = typec_register_port(dev, cap);
    if (IS_ERR(ctx.typec.port)) {
    ret = PTR_ERR(ctx.typec.port);
    ctx.typec.port = core::ptr::null_mut();
    dev_err(dev, "Failed to register type c port %d\n", ret);
    goto put_usb_role_switch;
    }
    typec_port_register_altmodes(ctx.typec.port, core::ptr::null_mut(), ctx,
    ctx.typec.port_amode,
    MAX_ALTMODE);
    return 0;
    put_usb_role_switch:
    if (ctx.typec.role_sw)
    usb_role_switch_put(ctx.typec.role_sw);
    put_fwnode:
    fwnode_handle_put(fwnode);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_typec_check_connection(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_typec_check_connection(struct anx7411_data *ctx)
    {
    int ret;
    ret = anx7411_reg_read(ctx.spi_client, FW_VER);
    if (ret < 0)
    return 0; /* No device attached in typec port */
// Clear interrupt and alert status
    ret = anx7411_reg_write(ctx.spi_client, INT_STS, 0);
    ret |= anx7411_reg_write(ctx.tcpc_client, ALERT_0, 0xFF);
    ret |= anx7411_reg_write(ctx.tcpc_client, ALERT_1, 0xFF);
    if (ret)
    return ret;
    ret = anx7411_cc_status_detect(ctx);
    ret |= anx7411_power_role_detect(ctx);
    ret |= anx7411_data_role_detect(ctx);
    ret |= anx7411_set_mux(ctx, SELECT_PIN_ASSIGMENT_C);
    if (ret)
    return ret;
    ret = anx7411_send_msg(ctx, TYPE_GET_DP_ALT_ENTER, core::ptr::null_mut(), 0);
    ret |= anx7411_send_msg(ctx, TYPE_GET_DP_DISCOVER_MODES_INFO, core::ptr::null_mut(), 0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_runtime_pm_suspend(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused anx7411_runtime_pm_suspend(struct device *dev)
    {
    struct anx7411_data *ctx = dev_get_drvdata(dev);
    mutex_lock(&ctx.lock);
    anx7411_partner_unregister_altmode(ctx);
    if (ctx.typec.partner)
    anx7411_unregister_partner(ctx);
    mutex_unlock(&ctx.lock);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_runtime_pm_resume(dev: *mut device) -> int __maybe_unused {
    static int __maybe_unused anx7411_runtime_pm_resume(struct device *dev)
    {
    struct anx7411_data *ctx = dev_get_drvdata(dev);
    mutex_lock(&ctx.lock);
// Detect PD connection
    if (anx7411_typec_check_connection(ctx))
    dev_err(dev, "check connection");
    mutex_unlock(&ctx.lock);
    return 0;
    }
    static const struct dev_pm_ops anx7411_pm_ops = {
    SET_SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend,
    pm_runtime_force_resume)
    SET_RUNTIME_PM_OPS(anx7411_runtime_pm_suspend,
    anx7411_runtime_pm_resume, core::ptr::null_mut())
    };
#[no_mangle]
unsafe extern "C" fn anx7411_get_gpio_irq(ctx: *mut anx7411_data) {
    static void anx7411_get_gpio_irq(struct anx7411_data *ctx)
    {
    struct device *dev = &ctx.tcpc_client.dev;
    ctx.intp_gpiod = devm_gpiod_get_optional(dev, "interrupt", GPIOD_IN);
    if (IS_ERR_OR_NULL(ctx.intp_gpiod)) {
    dev_err(dev, "no interrupt gpio property\n");
    return;
    }
    ctx.intp_irq = gpiod_to_irq(ctx.intp_gpiod);
    if (ctx.intp_irq < 0)
    dev_err(dev, "failed to get GPIO IRQ\n");
    }
    static enum power_supply_property anx7411_psy_props[] = {
    POWER_SUPPLY_PROP_USB_TYPE,
    POWER_SUPPLY_PROP_ONLINE,
    POWER_SUPPLY_PROP_VOLTAGE_MIN,
    POWER_SUPPLY_PROP_VOLTAGE_MAX,
    POWER_SUPPLY_PROP_VOLTAGE_NOW,
    POWER_SUPPLY_PROP_CURRENT_MAX,
    POWER_SUPPLY_PROP_CURRENT_NOW,
    };
    static int anx7411_psy_set_prop(struct power_supply *psy,
    enum power_supply_property psp,
    const union power_supply_propval *val)
    {
    struct anx7411_data *ctx = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    if (psp == POWER_SUPPLY_PROP_ONLINE)
    ctx.psy_online = val.intval;
    else
    ret = -EINVAL;
    power_supply_changed(ctx.psy);
    return ret;
    }
    static int anx7411_psy_prop_writeable(struct power_supply *psy,
    enum power_supply_property psp)
    {
    let mut psp: return = = POWER_SUPPLY_PROP_ONLINE;
    }
    static int anx7411_psy_get_prop(struct power_supply *psy,
    enum power_supply_property psp,
    union power_supply_propval *val)
    {
    struct anx7411_data *ctx = power_supply_get_drvdata(psy);
    let mut ret: c_int = 0;
    switch (psp) {
    case POWER_SUPPLY_PROP_USB_TYPE:
    val.intval = ctx.usb_type;
    break;
    case POWER_SUPPLY_PROP_ONLINE:
    val.intval = ctx.psy_online;
    break;
    case POWER_SUPPLY_PROP_VOLTAGE_NOW:
    case POWER_SUPPLY_PROP_VOLTAGE_MIN:
    case POWER_SUPPLY_PROP_VOLTAGE_MAX:
    val.intval = (ctx.psy_online) ?
    ctx.typec.request_voltage * 1000 : 0;
    break;
    case POWER_SUPPLY_PROP_CURRENT_NOW:
    case POWER_SUPPLY_PROP_CURRENT_MAX:
    val.intval = (ctx.psy_online) ?
    ctx.typec.request_current * 1000 : 0;
    break;
    default:
    ret = -EINVAL;
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_psy_register(ctx: *mut anx7411_data) -> c_int {
    static int anx7411_psy_register(struct anx7411_data *ctx)
    {
    struct power_supply_desc *psy_desc = &ctx.psy_desc;
    let mut psy_cfg: power_supply_config = {};
    char *psy_name;
    psy_name = devm_kasprintf(ctx.dev, GFP_KERNEL, "anx7411-source-psy-%s",
    dev_name(ctx.dev));
    if (!psy_name)
    return -ENOMEM;
    psy_desc.name = psy_name;
    psy_desc.type = POWER_SUPPLY_TYPE_USB;
    psy_desc.usb_types = BIT(POWER_SUPPLY_USB_TYPE_C)  |
    BIT(POWER_SUPPLY_USB_TYPE_PD) |
    BIT(POWER_SUPPLY_USB_TYPE_PD_PPS);
    psy_desc.properties = anx7411_psy_props;
    psy_desc.num_properties = ARRAY_SIZE(anx7411_psy_props);
    psy_desc.get_property = anx7411_psy_get_prop;
    psy_desc.set_property = anx7411_psy_set_prop;
    psy_desc.property_is_writeable = anx7411_psy_prop_writeable;
    ctx.usb_type = POWER_SUPPLY_USB_TYPE_C;
    ctx.psy = devm_power_supply_register(ctx.dev, psy_desc, &psy_cfg);
    if (IS_ERR(ctx.psy))
    dev_warn(ctx.dev, "unable to register psy\n");
    return PTR_ERR_OR_ZERO(ctx.psy);
    }
#[no_mangle]
unsafe extern "C" fn anx7411_i2c_probe(client: *mut i2c_client) -> c_int {
    static int anx7411_i2c_probe(struct i2c_client *client)
    {
    struct anx7411_data *plat;
    struct device *dev = &client.dev;
    int ret;
    if (!i2c_check_functionality(client.adapter, I2C_FUNC_SMBUS_I2C_BLOCK))
    return -ENODEV;
    plat = devm_kzalloc(dev, sizeof(*plat), GFP_KERNEL);
    if (!plat)
    return -ENOMEM;
    plat.tcpc_client = client;
    i2c_set_clientdata(client, plat);
    mutex_init(&plat.lock);
    ret = anx7411_register_i2c_dummy_clients(plat, client);
    if (ret) {
    dev_err(dev, "fail to reserve I2C bus\n");
    return ret;
    }
    ret = anx7411_typec_switch_probe(plat, dev);
    if (ret) {
    dev_err(dev, "fail to probe typec switch\n");
    goto free_i2c_dummy;
    }
    ret = anx7411_typec_port_probe(plat, dev);
    if (ret) {
    dev_err(dev, "fail to probe typec property.\n");
    ret = -ENODEV;
    goto free_typec_switch;
    }
    plat.intp_irq = client.irq;
    if (!client.irq)
    anx7411_get_gpio_irq(plat);
    if (!plat.intp_irq) {
    dev_err(dev, "fail to get interrupt IRQ\n");
    ret = -EINVAL;
    goto free_typec_port;
    }
    plat.dev = dev;
    plat.psy_online = ANX7411_PSY_OFFLINE;
    ret = anx7411_psy_register(plat);
    if (ret) {
    dev_err(dev, "register psy\n");
    goto free_typec_port;
    }
    INIT_WORK(&plat.work, anx7411_work_func);
    plat.workqueue = alloc_workqueue("anx7411_work",
    WQ_FREEZABLE | WQ_MEM_RECLAIM | WQ_PERCPU,
    1);
    if (!plat.workqueue) {
    dev_err(dev, "fail to create work queue\n");
    ret = -ENOMEM;
    goto free_typec_port;
    }
    ret = devm_request_threaded_irq(dev, plat.intp_irq,
    core::ptr::null_mut(), anx7411_intr_isr,
    IRQF_TRIGGER_FALLING |
    IRQF_ONESHOT,
    "anx7411-intp", plat);
    if (ret) {
    dev_err(dev, "fail to request irq\n");
    goto free_wq;
    }
    if (anx7411_typec_check_connection(plat))
    dev_err(dev, "check status\n");
    ret = devm_pm_runtime_enable(dev);
    if (ret)
    goto free_wq;
    return 0;
    free_wq:
    destroy_workqueue(plat.workqueue);
    free_typec_port:
    anx7411_port_unregister(&plat.typec);
    free_typec_switch:
    anx7411_unregister_switch(plat);
    anx7411_unregister_mux(plat);
    free_i2c_dummy:
    i2c_unregister_device(plat.spi_client);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn anx7411_i2c_remove(client: *mut i2c_client) {
    static void anx7411_i2c_remove(struct i2c_client *client)
    {
    struct anx7411_data *plat = i2c_get_clientdata(client);
    anx7411_partner_unregister_altmode(plat);
    anx7411_unregister_partner(plat);
    if (plat.workqueue)
    destroy_workqueue(plat.workqueue);
    i2c_unregister_device(plat.spi_client);
    anx7411_unregister_mux(plat);
    anx7411_unregister_switch(plat);
    anx7411_port_unregister(&plat.typec);
    }
    static const struct i2c_device_id anx7411_id[] = {
    { .name = "anx7411" },
    { }
    };
    MODULE_DEVICE_TABLE(i2c, anx7411_id);
    static const struct of_device_id anx_match_table[] = {
    {.compatible = "analogix,anx7411",},
    {},
    };
    MODULE_DEVICE_TABLE(of, anx_match_table);
    static struct i2c_driver anx7411_driver = {
    .driver = {
    .name = "anx7411",
    .of_match_table = anx_match_table,
    .pm = &anx7411_pm_ops,
    },
    .probe = anx7411_i2c_probe,
    .remove = anx7411_i2c_remove,
    .id_table = anx7411_id,
    };
    module_i2c_driver(anx7411_driver);
    MODULE_DESCRIPTION("Anx7411 USB Type-C PD driver");
    MODULE_AUTHOR("Xin Ji <xji@analogixsemi.com>");
    MODULE_LICENSE("GPL");
    MODULE_VERSION("0.1.5");
