//! Automatically rewritten from C to Rust
//! Source: drivers/usb/typec/tcpm/wcove.c
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
// typec_wcove.c - WhiskeyCove PMIC USB Type-C PHY driver
//
// Copyright (C) 2017 Intel Corporation
// Author: Heikki Krogerus <heikki.krogerus@linux.intel.com>
//

// Register offsets
pub const WCOVE_CHGRIRQ0: c_uint = 0x4e09;
pub const USBC_CONTROL1: c_uint = 0x7001;
pub const USBC_CONTROL2: c_uint = 0x7002;
pub const USBC_CONTROL3: c_uint = 0x7003;
pub const USBC_CC1_CTRL: c_uint = 0x7004;
pub const USBC_CC2_CTRL: c_uint = 0x7005;
pub const USBC_STATUS1: c_uint = 0x7007;
pub const USBC_STATUS2: c_uint = 0x7008;
pub const USBC_STATUS3: c_uint = 0x7009;
pub const USBC_CC1: c_uint = 0x700a;
pub const USBC_CC2: c_uint = 0x700b;
pub const USBC_CC1_STATUS: c_uint = 0x700c;
pub const USBC_CC2_STATUS: c_uint = 0x700d;
pub const USBC_IRQ1: c_uint = 0x7015;
pub const USBC_IRQ2: c_uint = 0x7016;
pub const USBC_IRQMASK1: c_uint = 0x7017;
pub const USBC_IRQMASK2: c_uint = 0x7018;
pub const USBC_PDCFG2: c_uint = 0x701a;
pub const USBC_PDCFG3: c_uint = 0x701b;
pub const USBC_PDSTATUS: c_uint = 0x701c;
pub const USBC_RXSTATUS: c_uint = 0x701d;
pub const USBC_RXINFO: c_uint = 0x701e;
pub const USBC_TXCMD: c_uint = 0x701f;
pub const USBC_TXINFO: c_uint = 0x7020;
pub const USBC_RX_DATA: c_uint = 0x7028;
pub const USBC_TX_DATA: c_uint = 0x7047;
// Register bits
pub const USBC_CONTROL1_MODE_MASK: c_uint = 0x3;
pub const USBC_CONTROL1_MODE_SNK: c_int = 0;
pub const USBC_CONTROL1_MODE_SNKACC: c_int = 1;
pub const USBC_CONTROL1_MODE_SRC: c_int = 2;
pub const USBC_CONTROL1_MODE_SRCACC: c_int = 3;
pub const USBC_CONTROL1_MODE_DRP: c_int = 4;
pub const USBC_CONTROL1_MODE_DRPACC: c_int = 5;
pub const USBC_CONTROL1_MODE_TEST: c_int = 7;
pub const USBC_CONTROL1_CURSRC_MASK: c_uint = 0xc;

pub const USBC_CONTROL1_DRPTOGGLE_RANDOM: c_uint = 0xe0;

pub const USBC_RSLT_NOTHING: c_int = 0;
pub const USBC_RSLT_SRC_DEFAULT: c_int = 1;
pub const USBC_RSLT_SRC_1_5A: c_int = 2;
pub const USBC_RSLT_SRC_3_0A: c_int = 3;
pub const USBC_RSLT_SNK: c_int = 4;
pub const USBC_RSLT_DEBUG_ACC: c_int = 5;
pub const USBC_RSLT_AUDIO_ACC: c_int = 6;
pub const USBC_RSLT_UNDEF: c_int = 15;

pub const USBC_ORIENT_NORMAL: c_int = 1;
pub const USBC_ORIENT_REVERSE: c_int = 2;

pub const USBC_CC_STATUS_RD: c_int = 1;
pub const USBC_CC_STATUS_RA: c_int = 2;

    USBC_IRQ1_SHORT)

    USBC_IRQ2_RX_HR | USBC_IRQ2_RX_CR | \
    USBC_IRQ2_TX_SUCCESS | USBC_IRQ2_TX_FAIL)

pub const USBC_PDCFG3_DATAROLE_SHIFT: c_int = 1;
pub const USBC_PDCFG3_SOP_SHIFT: c_int = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wcove_typec {
    pub /: *mut *mut mutex lock; / device lock,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub guid: guid_t,
    pub vbus: bool,
    pub tcpc: tcpc_dev,
    pub tcpm: *mut tcpm_port,
}

    enum wcove_typec_func {
    WCOVE_FUNC_DRIVE_VBUS = 1,
    WCOVE_FUNC_ORIENTATION,
    WCOVE_FUNC_ROLE,
    WCOVE_FUNC_DRIVE_VCONN,
    };
    enum wcove_typec_orientation {
    WCOVE_ORIENTATION_NORMAL,
    WCOVE_ORIENTATION_REVERSE,
    };
    enum wcove_typec_role {
    WCOVE_ROLE_HOST,
    WCOVE_ROLE_DEVICE,
    };

    static int wcove_typec_func(struct wcove_typec *wcove,
    enum wcove_typec_func func, int param)
    {
    union acpi_object *obj;
    union acpi_object tmp;
    let mut argv4: union acpi_object = ACPI_INIT_DSM_ARGV4(1, &tmp);
    tmp.type = ACPI_TYPE_INTEGER;
    tmp.integer.value = param;
    obj = acpi_evaluate_dsm(ACPI_HANDLE(wcove.dev), &wcove.guid, 1, func,
    &argv4);
    if (!obj) {
    dev_err(wcove.dev, "%s: failed to evaluate _DSM\n", __func__);
    return -EIO;
    }
    ACPI_FREE(obj);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wcove_init(tcpc: *mut tcpc_dev) -> c_int {
    static int wcove_init(struct tcpc_dev *tcpc)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    int ret;
    ret = regmap_write(wcove.regmap, USBC_CONTROL1, 0);
    if (ret)
    return ret;
// Unmask everything
    ret = regmap_write(wcove.regmap, USBC_IRQMASK1, 0);
    if (ret)
    return ret;
    return regmap_write(wcove.regmap, USBC_IRQMASK2, 0);
    }
#[no_mangle]
unsafe extern "C" fn wcove_get_vbus(tcpc: *mut tcpc_dev) -> c_int {
    static int wcove_get_vbus(struct tcpc_dev *tcpc)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    unsigned int cc1ctrl;
    int ret;
    ret = regmap_read(wcove.regmap, USBC_CC1_CTRL, &cc1ctrl);
    if (ret)
    return ret;
    wcove.vbus = !!(cc1ctrl & USBC_CC_CTRL_VBUSOK);
    return wcove.vbus;
    }
#[no_mangle]
unsafe extern "C" fn wcove_set_vbus(tcpc: *mut tcpc_dev, on: bool, sink: bool) -> c_int {
    static int wcove_set_vbus(struct tcpc_dev *tcpc, bool on, bool sink)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    return wcove_typec_func(wcove, WCOVE_FUNC_DRIVE_VBUS, on);
    }
#[no_mangle]
unsafe extern "C" fn wcove_set_vconn(tcpc: *mut tcpc_dev, on: bool) -> c_int {
    static int wcove_set_vconn(struct tcpc_dev *tcpc, bool on)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    return wcove_typec_func(wcove, WCOVE_FUNC_DRIVE_VCONN, on);
    }
#[no_mangle]
unsafe extern "C" fn wcove_to_typec_cc(cc: c_uint) -> enum typec_cc_status {
    static enum typec_cc_status wcove_to_typec_cc(unsigned int cc)
    {
    if (cc & UCSC_CC_STATUS_SNK_RP) {
    if (cc & UCSC_CC_STATUS_PWRDEFSNK)
    return TYPEC_CC_RP_DEF;
#[no_mangle]
pub unsafe extern "C" fn if(UCSC_CC_STATUS_PWR_1P5A_SNK: cc &) -> else {
    else if (cc & UCSC_CC_STATUS_PWR_1P5A_SNK)
    return TYPEC_CC_RP_1_5;
#[no_mangle]
pub unsafe extern "C" fn if(UCSC_CC_STATUS_PWR_3A_SNK: cc &) -> else {
    else if (cc & UCSC_CC_STATUS_PWR_3A_SNK)
    return TYPEC_CC_RP_3_0;
    } else {
    switch (UCSC_CC_STATUS_RX(cc)) {
    case USBC_CC_STATUS_RD:
    return TYPEC_CC_RD;
    case USBC_CC_STATUS_RA:
    return TYPEC_CC_RA;
    default:
    break;
    }
    }
    return TYPEC_CC_OPEN;
    }
    static int wcove_get_cc(struct tcpc_dev *tcpc, enum typec_cc_status *cc1,
    enum typec_cc_status *cc2)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    unsigned int cc1_status;
    unsigned int cc2_status;
    int ret;
    ret = regmap_read(wcove.regmap, USBC_CC1_STATUS, &cc1_status);
    if (ret)
    return ret;
    ret = regmap_read(wcove.regmap, USBC_CC2_STATUS, &cc2_status);
    if (ret)
    return ret;
// cc1 = wcove_to_typec_cc(cc1_status);
// cc2 = wcove_to_typec_cc(cc2_status);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wcove_set_cc(tcpc: *mut tcpc_dev, cc: enum typec_cc_status) -> c_int {
    static int wcove_set_cc(struct tcpc_dev *tcpc, enum typec_cc_status cc)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    unsigned int ctrl;
    switch (cc) {
    case TYPEC_CC_RD:
    ctrl = USBC_CONTROL1_MODE_SNK;
    break;
    case TYPEC_CC_RP_DEF:
    ctrl = USBC_CONTROL1_CURSRC_UA_80 | USBC_CONTROL1_MODE_SRC;
    break;
    case TYPEC_CC_RP_1_5:
    ctrl = USBC_CONTROL1_CURSRC_UA_180 | USBC_CONTROL1_MODE_SRC;
    break;
    case TYPEC_CC_RP_3_0:
    ctrl = USBC_CONTROL1_CURSRC_UA_330 | USBC_CONTROL1_MODE_SRC;
    break;
    case TYPEC_CC_OPEN:
    ctrl = 0;
    break;
    default:
    return -EINVAL;
    }
    return regmap_write(wcove.regmap, USBC_CONTROL1, ctrl);
    }
#[no_mangle]
unsafe extern "C" fn wcove_set_polarity(tcpc: *mut tcpc_dev, pol: enum typec_cc_polarity) -> c_int {
    static int wcove_set_polarity(struct tcpc_dev *tcpc, enum typec_cc_polarity pol)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    return wcove_typec_func(wcove, WCOVE_FUNC_ORIENTATION, pol);
    }
#[no_mangle]
unsafe extern "C" fn wcove_set_current_limit(tcpc: *mut tcpc_dev, max_ma: u32, mv: u32) -> c_int {
    static int wcove_set_current_limit(struct tcpc_dev *tcpc, u32 max_ma, u32 mv)
    {
    return 0;
    }
    static int wcove_set_roles(struct tcpc_dev *tcpc, bool attached,
    enum typec_role role, enum typec_data_role data)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    unsigned int val;
    int ret;
    ret = wcove_typec_func(wcove, WCOVE_FUNC_ROLE, data == TYPEC_HOST ?
    WCOVE_ROLE_HOST : WCOVE_ROLE_DEVICE);
    if (ret)
    return ret;
    val = role;
    val |= data << USBC_PDCFG3_DATAROLE_SHIFT;
    val |= PD_REV20 << USBC_PDCFG3_SOP_SHIFT;
    return regmap_write(wcove.regmap, USBC_PDCFG3, val);
    }
#[no_mangle]
unsafe extern "C" fn wcove_set_pd_rx(tcpc: *mut tcpc_dev, on: bool) -> c_int {
    static int wcove_set_pd_rx(struct tcpc_dev *tcpc, bool on)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    return regmap_write(wcove.regmap, USBC_PDCFG2,
    on ? USBC_PDCFG2_SOP : 0);
    }
    static int wcove_pd_transmit(struct tcpc_dev *tcpc,
    enum tcpm_transmit_type type,
    const struct pd_message *msg,
    unsigned int negotiated_rev)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    let mut info: c_uint = 0;
    unsigned int cmd;
    int ret;
    ret = regmap_read(wcove.regmap, USBC_TXCMD, &cmd);
    if (ret)
    return ret;
    if (!(cmd & USBC_TXCMD_BUF_RDY)) {
    dev_warn(wcove.dev, "%s: Last transmission still ongoing!",
    __func__);
    return -EBUSY;
    }
    if (msg) {
    const u8 *data = (void *)msg;
    int i;
    for (i = 0; i < pd_header_cnt_le(msg.header) * 4 + 2; i++) {
    ret = regmap_write(wcove.regmap, USBC_TX_DATA + i,
    data[i]);
    if (ret)
    return ret;
    }
    }
    switch (type) {
    case TCPC_TX_SOP:
    case TCPC_TX_SOP_PRIME:
    case TCPC_TX_SOP_PRIME_PRIME:
    case TCPC_TX_SOP_DEBUG_PRIME:
    case TCPC_TX_SOP_DEBUG_PRIME_PRIME:
    info = type + 1;
    cmd = USBC_TXCMD_MSG;
    break;
    case TCPC_TX_HARD_RESET:
    cmd = USBC_TXCMD_HR;
    break;
    case TCPC_TX_CABLE_RESET:
    cmd = USBC_TXCMD_CR;
    break;
    case TCPC_TX_BIST_MODE_2:
    cmd = USBC_TXCMD_BIST;
    break;
    default:
    return -EINVAL;
    }
// NOTE Setting maximum number of retries (7)
    ret = regmap_write(wcove.regmap, USBC_TXINFO,
    info | USBC_TXINFO_RETRIES(7));
    if (ret)
    return ret;
    return regmap_write(wcove.regmap, USBC_TXCMD, cmd | USBC_TXCMD_START);
    }
    static int wcove_start_toggling(struct tcpc_dev *tcpc,
    enum typec_port_type port_type,
    enum typec_cc_status cc)
    {
    struct wcove_typec *wcove = tcpc_to_wcove(tcpc);
    unsigned int usbc_ctrl;
    if (port_type != TYPEC_PORT_DRP)
    return -EOPNOTSUPP;
    usbc_ctrl = USBC_CONTROL1_MODE_DRP | USBC_CONTROL1_DRPTOGGLE_RANDOM;
    switch (cc) {
    case TYPEC_CC_RP_1_5:
    usbc_ctrl |= USBC_CONTROL1_CURSRC_UA_180;
    break;
    case TYPEC_CC_RP_3_0:
    usbc_ctrl |= USBC_CONTROL1_CURSRC_UA_330;
    break;
    default:
    usbc_ctrl |= USBC_CONTROL1_CURSRC_UA_80;
    break;
    }
    return regmap_write(wcove.regmap, USBC_CONTROL1, usbc_ctrl);
    }
    static int wcove_read_rx_buffer(struct wcove_typec *wcove,
    struct pd_message *msg)
    {
    unsigned int info, val, len;
    u8 *buf = (u8 *)msg;
    int ret;
    int i;
    ret = regmap_read(wcove.regmap, USBC_RXINFO, &info);
    if (ret)
    return ret;
    len = min(USBC_RXINFO_RXBYTES(info), sizeof(*msg));
    for (i = 0; i < len; i++) {
    ret = regmap_read(wcove.regmap, USBC_RX_DATA + i, &val);
    if (ret)
    return ret;
    buf[i] = val;
    }
    return regmap_write(wcove.regmap, USBC_RXSTATUS,
    USBC_RXSTATUS_RXCLEAR);
    }
#[no_mangle]
unsafe extern "C" fn wcove_typec_irq(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t wcove_typec_irq(int irq, void *data)
    {
    struct wcove_typec *wcove = data;
    let mut usbc_irq1: c_uint = 0;
    let mut usbc_irq2: c_uint = 0;
    unsigned int cc1ctrl;
    int ret;
    mutex_lock(&wcove.lock);
// Read..
    ret = regmap_read(wcove.regmap, USBC_IRQ1, &usbc_irq1);
    if (ret)
    goto err;
    ret = regmap_read(wcove.regmap, USBC_IRQ2, &usbc_irq2);
    if (ret)
    goto err;
    ret = regmap_read(wcove.regmap, USBC_CC1_CTRL, &cc1ctrl);
    if (ret)
    goto err;
    if (!wcove.tcpm)
    goto err;
// ..check..
    if (usbc_irq1 & USBC_IRQ1_OVERTEMP) {
    dev_err(wcove.dev, "VCONN Switch Over Temperature!\n");
    wcove_typec_func(wcove, WCOVE_FUNC_DRIVE_VCONN, false);
// REVISIT: Report an error?
    }
    if (usbc_irq1 & USBC_IRQ1_SHORT) {
    dev_err(wcove.dev, "VCONN Switch Short Circuit!\n");
    wcove_typec_func(wcove, WCOVE_FUNC_DRIVE_VCONN, false);
// REVISIT: Report an error?
    }
    if (wcove.vbus != !!(cc1ctrl & USBC_CC_CTRL_VBUSOK))
    tcpm_vbus_change(wcove.tcpm);
// REVISIT: See if tcpm code can be made to consider Type-C HW FSMs
    if (usbc_irq2 & USBC_IRQ2_CC_CHANGE)
    tcpm_cc_change(wcove.tcpm);
    if (usbc_irq2 & USBC_IRQ2_RX_PD) {
    unsigned int status;
//
// FIXME: Need to check if TX is ongoing and report
// TX_DIREGARDED if needed?
//
    ret = regmap_read(wcove.regmap, USBC_RXSTATUS, &status);
    if (ret)
    goto err;
// Flush all buffers
    while (status & USBC_RXSTATUS_RXDATA) {
    struct pd_message msg;
    ret = wcove_read_rx_buffer(wcove, &msg);
    if (ret) {
    dev_err(wcove.dev, "%s: RX read failed\n",
    __func__);
    goto err;
    }
    tcpm_pd_receive(wcove.tcpm, &msg, TCPC_TX_SOP);
    ret = regmap_read(wcove.regmap, USBC_RXSTATUS,
    &status);
    if (ret)
    goto err;
    }
    }
    if (usbc_irq2 & USBC_IRQ2_RX_HR)
    tcpm_pd_hard_reset(wcove.tcpm);
// REVISIT: if (usbc_irq2 & USBC_IRQ2_RX_CR)
    if (usbc_irq2 & USBC_IRQ2_TX_SUCCESS)
    tcpm_pd_transmit_complete(wcove.tcpm, TCPC_TX_SUCCESS);
    if (usbc_irq2 & USBC_IRQ2_TX_FAIL)
    tcpm_pd_transmit_complete(wcove.tcpm, TCPC_TX_FAILED);
    err:
// ..and clear.
    if (usbc_irq1) {
    ret = regmap_write(wcove.regmap, USBC_IRQ1, usbc_irq1);
    if (ret)
    dev_WARN(wcove.dev, "%s failed to clear IRQ1\n",
    __func__);
    }
    if (usbc_irq2) {
    ret = regmap_write(wcove.regmap, USBC_IRQ2, usbc_irq2);
    if (ret)
    dev_WARN(wcove.dev, "%s failed to clear IRQ2\n",
    __func__);
    }
// REVISIT: Clear WhiskeyCove CHGR Type-C interrupt
    regmap_write(wcove.regmap, WCOVE_CHGRIRQ0, BIT(5));
    mutex_unlock(&wcove.lock);
    return IRQ_HANDLED;
    }
//
// The following power levels should be safe to use with Joule board.
//
    static const u32 src_pdo[] = {
    PDO_FIXED(5000, 1500, PDO_FIXED_DUAL_ROLE | PDO_FIXED_DATA_SWAP |
    PDO_FIXED_USB_COMM),
    };
    static const u32 snk_pdo[] = {
    PDO_FIXED(5000, 500, PDO_FIXED_DUAL_ROLE | PDO_FIXED_DATA_SWAP |
    PDO_FIXED_USB_COMM),
    PDO_VAR(5000, 12000, 3000),
    };
    static const struct property_entry wcove_props[] = {
    PROPERTY_ENTRY_STRING("data-role", "dual"),
    PROPERTY_ENTRY_STRING("power-role", "dual"),
    PROPERTY_ENTRY_STRING("try-power-role", "sink"),
    PROPERTY_ENTRY_U32_ARRAY("source-pdos", src_pdo),
    PROPERTY_ENTRY_U32_ARRAY("sink-pdos", snk_pdo),
    PROPERTY_ENTRY_U32("op-sink-microwatt", 15000000),
    { }
    };
#[no_mangle]
unsafe extern "C" fn wcove_typec_probe(pdev: *mut platform_device) -> c_int {
    static int wcove_typec_probe(struct platform_device *pdev)
    {
    struct intel_soc_pmic *pmic = dev_get_drvdata(pdev.dev.parent);
    struct wcove_typec *wcove;
    int irq;
    int ret;
    wcove = devm_kzalloc(&pdev.dev, sizeof(*wcove), GFP_KERNEL);
    if (!wcove)
    return -ENOMEM;
    mutex_init(&wcove.lock);
    wcove.dev = &pdev.dev;
    wcove.regmap = pmic.regmap;
    irq = platform_get_irq(pdev, 0);
    if (irq < 0)
    return irq;
    ret = guid_parse(WCOVE_DSM_UUID, &wcove.guid);
    if (ret)
    return ret;
    if (!acpi_check_dsm(ACPI_HANDLE(&pdev.dev), &wcove.guid, 0, 0x1f)) {
    dev_err(&pdev.dev, "Missing _DSM functions\n");
    return -ENODEV;
    }
    wcove.tcpc.init = wcove_init;
    wcove.tcpc.get_vbus = wcove_get_vbus;
    wcove.tcpc.set_vbus = wcove_set_vbus;
    wcove.tcpc.set_cc = wcove_set_cc;
    wcove.tcpc.get_cc = wcove_get_cc;
    wcove.tcpc.set_polarity = wcove_set_polarity;
    wcove.tcpc.set_vconn = wcove_set_vconn;
    wcove.tcpc.set_current_limit = wcove_set_current_limit;
    wcove.tcpc.start_toggling = wcove_start_toggling;
    wcove.tcpc.set_pd_rx = wcove_set_pd_rx;
    wcove.tcpc.set_roles = wcove_set_roles;
    wcove.tcpc.pd_transmit = wcove_pd_transmit;
    wcove.tcpc.fwnode = fwnode_create_software_node(wcove_props, core::ptr::null_mut());
    if (IS_ERR(wcove.tcpc.fwnode))
    return PTR_ERR(wcove.tcpc.fwnode);
    wcove.tcpm = tcpm_register_port(wcove.dev, &wcove.tcpc);
    if (IS_ERR(wcove.tcpm)) {
    fwnode_remove_software_node(wcove.tcpc.fwnode);
    return PTR_ERR(wcove.tcpm);
    }
    ret = devm_request_threaded_irq(&pdev.dev, irq, core::ptr::null_mut(),
    wcove_typec_irq, IRQF_ONESHOT,
    "wcove_typec", wcove);
    if (ret) {
    tcpm_unregister_port(wcove.tcpm);
    fwnode_remove_software_node(wcove.tcpc.fwnode);
    return ret;
    }
    platform_set_drvdata(pdev, wcove);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wcove_typec_remove(pdev: *mut platform_device) {
    static void wcove_typec_remove(struct platform_device *pdev)
    {
    struct wcove_typec *wcove = platform_get_drvdata(pdev);
    unsigned int val;
// Mask everything
    regmap_read(wcove.regmap, USBC_IRQMASK1, &val);
    regmap_write(wcove.regmap, USBC_IRQMASK1, val | USBC_IRQMASK1_ALL);
    regmap_read(wcove.regmap, USBC_IRQMASK2, &val);
    regmap_write(wcove.regmap, USBC_IRQMASK2, val | USBC_IRQMASK2_ALL);
    tcpm_unregister_port(wcove.tcpm);
    fwnode_remove_software_node(wcove.tcpc.fwnode);
    }
    static struct platform_driver wcove_typec_driver = {
    .driver = {
    .name		= "bxt_wcove_usbc",
    },
    .probe			= wcove_typec_probe,
    .remove			= wcove_typec_remove,
    };
    module_platform_driver(wcove_typec_driver);
    MODULE_AUTHOR("Intel Corporation");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("WhiskeyCove PMIC USB Type-C PHY driver");
    MODULE_ALIAS("platform:bxt_wcove_usbc");
