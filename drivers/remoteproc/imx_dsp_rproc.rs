//! Automatically rewritten from C to Rust
//! Source: drivers/remoteproc/imx_dsp_rproc.c
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
// Copyright 2021 NXP

pub const DSP_RPROC_CLK_MAX: c_int = 5;
//
// Module parameters
//
    static unsigned int no_mailboxes;
    module_param_named(no_mailboxes, no_mailboxes, int, 0644);
    MODULE_PARM_DESC(no_mailboxes,
    "There is no mailbox between cores, so ignore remote proc reply after start, default is 0 (off).");
// Flag indicating that the remote is up and running

// Flag indicating that the host should wait for a firmware-confirmation response

pub const REMOTE_READY_WAIT_MAX_RETRIES: c_int = 500;
//
// This flag is set in the DSP resource table's features field to indicate
// that the firmware requires the host NOT to wait for a FW_CONFIRMATION response.
//

// att flags
// DSP own area

// DSP instruction area

// Definitions for i.MX8MP
// DAP registers
pub const IMX8M_DAP_DEBUG: c_uint = 0x28800000;

// DSP audio mix registers
pub const IMX8M_AudioDSP_REG0: c_uint = 0x100;
pub const IMX8M_AudioDSP_REG1: c_uint = 0x104;
pub const IMX8M_AudioDSP_REG2: c_uint = 0x108;
pub const IMX8M_AudioDSP_REG3: c_uint = 0x10c;

// Definitions for i.MX8ULP
pub const IMX8ULP_SIM_LPAV_REG_SYSCTRL0: c_uint = 0x8;

pub const IMX8ULP_SIP_HIFI_XRDC: c_uint = 0xc200000e;

    (uint32_t)'x' << 16 |	\
    (uint32_t)'p' << 8 |	\
    (uint32_t)'s')
//
// enum - Predefined Mailbox Messages
//
// @RP_MBOX_SUSPEND_SYSTEM: system suspend request for the remote processor
//
// @RP_MBOX_SUSPEND_ACK: successful response from remote processor for a
// suspend request
//
// @RP_MBOX_RESUME_SYSTEM: system resume request for the remote processor
//
// @RP_MBOX_RESUME_ACK: successful response from remote processor for a
// resume request
//
    enum imx_dsp_rp_mbox_messages {
    RP_MBOX_SUSPEND_SYSTEM			= 0xFF11,
    RP_MBOX_SUSPEND_ACK			= 0xFF12,
    RP_MBOX_RESUME_SYSTEM			= 0xFF13,
    RP_MBOX_RESUME_ACK			= 0xFF14,
    };
//
// struct imx_dsp_rproc - DSP remote processor state
// @regmap: regmap handler
// @run_stall: reset control handle used for Run/Stall operation
// @rproc: rproc handler
// @dsp_dcfg: device configuration pointer
// @clks: clocks needed by this device
// @cl: mailbox client to request the mailbox channel
// @cl_rxdb: mailbox client to request the mailbox channel for doorbell
// @tx_ch: mailbox tx channel handle
// @rx_ch: mailbox rx channel handle
// @rxdb_ch: mailbox rx doorbell channel handle
// @pd_list: power domain list
// @ipc_handle: System Control Unit ipc handle
// @rproc_work: work for processing virtio interrupts
// @pm_comp: completion primitive to sync for suspend response
// @flags: control flags
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_dsp_rproc {
    pub regmap: *mut regmap,
    pub run_stall: *mut reset_control,
    pub rproc: *mut rproc,
    pub dsp_dcfg: *const imx_dsp_rproc_dcfg,
    pub clks: [clk_bulk_data; DSP_RPROC_CLK_MAX],
    pub cl: mbox_client,
    pub cl_rxdb: mbox_client,
    pub tx_ch: *mut mbox_chan,
    pub rx_ch: *mut mbox_chan,
    pub rxdb_ch: *mut mbox_chan,
    pub pd_list: *mut dev_pm_domain_list,
    pub ipc_handle: *mut imx_sc_ipc,
    pub rproc_work: work_struct,
    pub pm_comp: completion,
    pub flags: u32,
}

//
// struct imx_dsp_rproc_dcfg - DSP remote processor configuration
// @dcfg: imx_rproc_dcfg handler
// @reset: reset callback function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_dsp_rproc_dcfg {
    pub dcfg: *const imx_rproc_dcfg,
    pub priv): *mut *mut int (reset)(struct imx_dsp_rproc,
}

//
// struct fw_rsc_imx_dsp - i.MX DSP specific info
//
// @len: length of the resource entry
// @magic_num: 32-bit magic number
// @version: version of data structure
// @features: feature flags supported by the i.MX DSP firmware
//
// This represents a DSP-specific resource in the firmware's
// resource table, providing information on supported features.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_rsc_imx_dsp {
    pub len: u32,
    pub magic_num: u32,
    pub version: u32,
    pub features: u32,
    pub __packed: },
    static const struct imx_rproc_att imx_dsp_rproc_att_imx8qm[] = {
// dev addr , sys addr  , size	    , flags
    { 0x596e8000, 0x556e8000, 0x00008000, ATT_OWN },
    { 0x596f0000, 0x556f0000, 0x00008000, ATT_OWN },
    { 0x596f8000, 0x556f8000, 0x00000800, ATT_OWN | ATT_IRAM},
    { 0x55700000, 0x55700000, 0x00070000, ATT_OWN },
// DDR (Data)
    { 0x80000000, 0x80000000, 0x60000000, 0},
}

    static const struct imx_rproc_att imx_dsp_rproc_att_imx8qxp[] = {
// dev addr , sys addr  , size	    , flags
    { 0x596e8000, 0x596e8000, 0x00008000, ATT_OWN },
    { 0x596f0000, 0x596f0000, 0x00008000, ATT_OWN },
    { 0x596f8000, 0x596f8000, 0x00000800, ATT_OWN | ATT_IRAM},
    { 0x59700000, 0x59700000, 0x00070000, ATT_OWN },
// DDR (Data)
    { 0x80000000, 0x80000000, 0x60000000, 0},
    };
    static const struct imx_rproc_att imx_dsp_rproc_att_imx8mp[] = {
// dev addr , sys addr  , size	    , flags
    { 0x3b6e8000, 0x3b6e8000, 0x00008000, ATT_OWN },
    { 0x3b6f0000, 0x3b6f0000, 0x00008000, ATT_OWN },
    { 0x3b6f8000, 0x3b6f8000, 0x00000800, ATT_OWN | ATT_IRAM},
    { 0x3b700000, 0x3b700000, 0x00040000, ATT_OWN },
// DDR (Data)
    { 0x40000000, 0x40000000, 0x80000000, 0},
    };
    static const struct imx_rproc_att imx_dsp_rproc_att_imx8ulp[] = {
// dev addr , sys addr  , size	    , flags
    { 0x21170000, 0x21170000, 0x00010000, ATT_OWN | ATT_IRAM},
    { 0x21180000, 0x21180000, 0x00010000, ATT_OWN },
// DDR (Data)
    { 0x0c000000, 0x80000000, 0x10000000, 0},
    { 0x30000000, 0x90000000, 0x10000000, 0},
    };
// Initialize the mailboxes between cores, if exists
    static int (*imx_dsp_rproc_mbox_init)(struct imx_dsp_rproc *priv);
// Reset function for DSP on i.MX8MP
#[no_mangle]
unsafe extern "C" fn imx8mp_dsp_reset(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx8mp_dsp_reset(struct imx_dsp_rproc *priv)
    {
    void __iomem *dap = ioremap_wc(IMX8M_DAP_DEBUG, IMX8M_DAP_DEBUG_SIZE);
    int pwrctl;
// Put DSP into reset and stall
    pwrctl = readl(dap + IMX8M_DAP_PWRCTL);
    pwrctl |= IMX8M_PWRCTL_CORERESET;
    writel(pwrctl, dap + IMX8M_DAP_PWRCTL);
// Keep reset asserted for 10 cycles
    usleep_range(1, 2);
    reset_control_assert(priv.run_stall);
// Take the DSP out of reset and keep stalled for FW loading
    pwrctl = readl(dap + IMX8M_DAP_PWRCTL);
    pwrctl &= ~IMX8M_PWRCTL_CORERESET;
    writel(pwrctl, dap + IMX8M_DAP_PWRCTL);
    iounmap(dap);
    return 0;
    }
// Reset function for DSP on i.MX8ULP
#[no_mangle]
unsafe extern "C" fn imx8ulp_dsp_reset(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx8ulp_dsp_reset(struct imx_dsp_rproc *priv)
    {
    struct arm_smccc_res res;
// Put DSP into reset and stall
    regmap_update_bits(priv.regmap, IMX8ULP_SIM_LPAV_REG_SYSCTRL0,
    IMX8ULP_SYSCTRL0_DSP_RST, IMX8ULP_SYSCTRL0_DSP_RST);
    regmap_update_bits(priv.regmap, IMX8ULP_SIM_LPAV_REG_SYSCTRL0,
    IMX8ULP_SYSCTRL0_DSP_STALL,
    IMX8ULP_SYSCTRL0_DSP_STALL);
// Configure resources of DSP through TFA
    arm_smccc_smc(IMX8ULP_SIP_HIFI_XRDC, 0, 0, 0, 0, 0, 0, 0, &res);
// Take the DSP out of reset and keep stalled for FW loading
    regmap_update_bits(priv.regmap, IMX8ULP_SIM_LPAV_REG_SYSCTRL0,
    IMX8ULP_SYSCTRL0_DSP_RST, 0);
    regmap_update_bits(priv.regmap, IMX8ULP_SIM_LPAV_REG_SYSCTRL0,
    IMX8ULP_SYSCTRL0_DSP_DBG_RST, 0);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_ready(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_ready(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    int i;
    if (!priv.rxdb_ch)
    return 0;
    for (i = 0; i < REMOTE_READY_WAIT_MAX_RETRIES; i++) {
    if (priv.flags & REMOTE_IS_READY)
    return 0;
    usleep_range(100, 200);
    }
    return -ETIMEDOUT;
    }
//
// imx_dsp_rproc_handle_rsc() - Handle DSP-specific resource table entries
// @rproc: remote processor instance
// @rsc_type: resource type identifier
// @rsc: pointer to the resource entry
// @offset: offset of the resource entry
// @avail: available space in the resource table
//
// Parse the DSP-specific resource entry and update flags accordingly.
// If the WAIT_FW_CONFIRMATION feature is set, the host must wait for the firmware
// to signal readiness before proceeding with execution.
//
// Return: RSC_HANDLED if processed successfully, RSC_IGNORED otherwise.
//
    static int imx_dsp_rproc_handle_rsc(struct rproc *rproc, u32 rsc_type,
    void *rsc, int offset, int avail)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    struct fw_rsc_imx_dsp *imx_dsp_rsc = rsc;
    struct device *dev = rproc.dev.parent;
    if (!imx_dsp_rsc) {
    dev_dbg(dev, "Invalid fw_rsc_imx_dsp.\n");
    return RSC_IGNORED;
    }
// Make sure resource isn't truncated
    if (sizeof(struct fw_rsc_imx_dsp) > avail ||
    sizeof(struct fw_rsc_imx_dsp) != imx_dsp_rsc.len) {
    dev_dbg(dev, "Resource fw_rsc_imx_dsp is truncated.\n");
    return RSC_IGNORED;
    }
//
// If FW_RSC_NXP_S_MAGIC number is not found then
// wait for fw_ready reply (default work flow)
//
    if (imx_dsp_rsc.magic_num != FW_RSC_NXP_S_MAGIC) {
    dev_dbg(dev, "Invalid resource table magic number.\n");
    return RSC_IGNORED;
    }
//
// For now, in struct fw_rsc_imx_dsp, version 0,
// only FEATURE_SKIP_FW_CONFIRMATION is valid.
//
// When adding new features, please upgrade version.
//
    if (imx_dsp_rsc.version > 0) {
    dev_warn(dev, "Unexpected fw_rsc_imx_dsp version %d.\n",
    imx_dsp_rsc.version);
    return RSC_IGNORED;
    }
    if (imx_dsp_rsc.features & FEATURE_SKIP_FW_CONFIRMATION)
    priv.flags &= ~WAIT_FW_CONFIRMATION;
    return RSC_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_mmio_start(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_mmio_start(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    const struct imx_rproc_dcfg *dcfg = priv.dsp_dcfg.dcfg;
    return regmap_update_bits(priv.regmap, dcfg.src_reg, dcfg.src_mask, dcfg.src_start);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_reset_ctrl_start(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_reset_ctrl_start(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    return reset_control_deassert(priv.run_stall);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_scu_api_start(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_scu_api_start(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    return imx_sc_pm_cpu_start(priv.ipc_handle, IMX_SC_R_DSP, true, rproc.bootaddr);
    }
//
// Start function for rproc_ops
//
// There is a handshake for start procedure: when DSP starts, it
// will send a doorbell message to this driver, then the
// REMOTE_IS_READY flags is set, then driver will kick
// a message to DSP.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_start(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_start(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    const struct imx_dsp_rproc_dcfg *dsp_dcfg = priv.dsp_dcfg;
    const struct imx_rproc_dcfg *dcfg = dsp_dcfg.dcfg;
    struct device *dev = rproc.dev.parent;
    int ret;
    if (!dcfg.ops || !dcfg.ops.start)
    return -EOPNOTSUPP;
    ret = dcfg.ops.start(rproc);
    if (ret) {
    dev_err(dev, "Failed to enable remote core!\n");
    return ret;
    }
    if (priv.flags & WAIT_FW_CONFIRMATION)
    return imx_dsp_rproc_ready(rproc);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_mmio_stop(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_mmio_stop(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    const struct imx_rproc_dcfg *dcfg = priv.dsp_dcfg.dcfg;
    return regmap_update_bits(priv.regmap, dcfg.src_reg, dcfg.src_mask, dcfg.src_stop);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_reset_ctrl_stop(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_reset_ctrl_stop(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    return reset_control_assert(priv.run_stall);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_scu_api_stop(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_scu_api_stop(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    return imx_sc_pm_cpu_start(priv.ipc_handle, IMX_SC_R_DSP, false, rproc.bootaddr);
    }
//
// Stop function for rproc_ops
// It clears the REMOTE_IS_READY flags
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_stop(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_stop(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    const struct imx_dsp_rproc_dcfg *dsp_dcfg = priv.dsp_dcfg;
    const struct imx_rproc_dcfg *dcfg = dsp_dcfg.dcfg;
    struct device *dev = rproc.dev.parent;
    let mut ret: c_int = 0;
    if (rproc.state == RPROC_CRASHED) {
    priv.flags &= ~REMOTE_IS_READY;
    return 0;
    }
    if (!dcfg.ops || !dcfg.ops.stop)
    return -EOPNOTSUPP;
    ret = dcfg.ops.stop(rproc);
    if (ret) {
    dev_err(dev, "Failed to stop remote core\n");
    return ret;
    }
    priv.flags &= ~REMOTE_IS_READY;
    return 0;
    }
//
// imx_dsp_rproc_sys_to_da() - internal memory translation helper
// @priv: private data pointer
// @sys: system address (DDR address)
// @len: length of the memory buffer
// @da: device address to translate
//
// Convert system address (DDR address) to device address (DSP)
// for there may be memory remap for device.
//
    static int imx_dsp_rproc_sys_to_da(struct imx_dsp_rproc *priv, u64 sys,
    size_t len, u64 *da)
    {
    const struct imx_dsp_rproc_dcfg *dsp_dcfg = priv.dsp_dcfg;
    const struct imx_rproc_dcfg *dcfg = dsp_dcfg.dcfg;
    int i;
// Parse address translation table
    for (i = 0; i < dcfg.att_size; i++) {
    const struct imx_rproc_att *att = &dcfg.att[i];
    if (sys >= att.sa && sys + len <= att.sa + att.size) {
    let mut offset: c_uint = sys - att.sa;
// da = att->da + offset;
    return 0;
    }
    }
    return -ENOENT;
    }
// Main virtqueue message work function
//
// This function is executed upon scheduling of the i.MX DSP remoteproc
// driver's workqueue. The workqueue is scheduled by the mailbox rx
// handler.
//
// This work function processes both the Tx and Rx virtqueue indices on
// every invocation. The rproc_vq_interrupt function can detect if there
// are new unprocessed messages or not (returns IRQ_NONE vs IRQ_HANDLED),
// but there is no need to check for these return values. The index 0
// triggering will process all pending Rx buffers, and the index 1 triggering
// will process all newly available Tx buffers and will wakeup any potentially
// blocked senders.
//
// NOTE:
// The current logic is based on an inherent design assumption of supporting
// only 2 vrings, but this can be changed if needed.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_vq_work(work: *mut work_struct) {
    static void imx_dsp_rproc_vq_work(struct work_struct *work)
    {
    struct imx_dsp_rproc *priv = container_of(work, struct imx_dsp_rproc,
    rproc_work);
    struct rproc *rproc = priv.rproc;
    mutex_lock(&rproc.lock);
    if (rproc.state != RPROC_RUNNING)
    goto unlock_mutex;
    rproc_vq_interrupt(priv.rproc, 0);
    rproc_vq_interrupt(priv.rproc, 1);
    unlock_mutex:
    mutex_unlock(&rproc.lock);
    }
//
// imx_dsp_rproc_rx_tx_callback() - inbound mailbox message handler
// @cl: mailbox client pointer used for requesting the mailbox channel
// @data: mailbox payload
//
// This handler is invoked by mailbox driver whenever a mailbox
// message is received. Usually, the SUSPEND and RESUME related messages
// are handled in this function, other messages are handled by remoteproc core
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_rx_tx_callback(cl: *mut mbox_client, data: *mut c_void) {
    static void imx_dsp_rproc_rx_tx_callback(struct mbox_client *cl, void *data)
    {
    struct rproc *rproc = dev_get_drvdata(cl.dev);
    struct imx_dsp_rproc *priv = rproc.priv;
    struct device *dev = rproc.dev.parent;
    let mut message: u32 = (u32)(*(u32 *)data);
    dev_dbg(dev, "mbox msg: 0x%x\n", message);
    switch (message) {
    case RP_MBOX_SUSPEND_ACK:
    complete(&priv.pm_comp);
    break;
    case RP_MBOX_RESUME_ACK:
    complete(&priv.pm_comp);
    break;
    default:
    schedule_work(&priv.rproc_work);
    break;
    }
    }
//
// imx_dsp_rproc_rxdb_callback() - inbound mailbox message handler
// @cl: mailbox client pointer used for requesting the mailbox channel
// @data: mailbox payload
//
// For doorbell, there is no message specified, just set REMOTE_IS_READY
// flag.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_rxdb_callback(cl: *mut mbox_client, data: *mut c_void) {
    static void imx_dsp_rproc_rxdb_callback(struct mbox_client *cl, void *data)
    {
    struct rproc *rproc = dev_get_drvdata(cl.dev);
    struct imx_dsp_rproc *priv = rproc.priv;
// Remote is ready after firmware is loaded and running
    priv.flags |= REMOTE_IS_READY;
    }
//
// imx_dsp_rproc_mbox_alloc() - request mailbox channels
// @priv: private data pointer
//
// Request three mailbox channels (tx, rx, rxdb).
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_mbox_alloc(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx_dsp_rproc_mbox_alloc(struct imx_dsp_rproc *priv)
    {
    struct device *dev = priv.rproc.dev.parent;
    struct mbox_client *cl;
    int ret;
    if (!of_property_present(dev.of_node, "mbox-names"))
    return 0;
    cl = &priv.cl;
    cl.dev = dev;
    cl.tx_block = true;
    cl.tx_tout = 100;
    cl.knows_txdone = false;
    cl.rx_callback = imx_dsp_rproc_rx_tx_callback;
// Channel for sending message
    priv.tx_ch = mbox_request_channel_byname(cl, "tx");
    if (IS_ERR(priv.tx_ch)) {
    ret = PTR_ERR(priv.tx_ch);
    dev_dbg(cl.dev, "failed to request tx mailbox channel: %d\n",
    ret);
    return ret;
    }
// Channel for receiving message
    priv.rx_ch = mbox_request_channel_byname(cl, "rx");
    if (IS_ERR(priv.rx_ch)) {
    ret = PTR_ERR(priv.rx_ch);
    dev_dbg(cl.dev, "failed to request rx mailbox channel: %d\n",
    ret);
    goto free_channel_tx;
    }
    cl = &priv.cl_rxdb;
    cl.dev = dev;
    cl.rx_callback = imx_dsp_rproc_rxdb_callback;
//
// RX door bell is used to receive the ready signal from remote
// after firmware loaded.
//
    priv.rxdb_ch = mbox_request_channel_byname(cl, "rxdb");
    if (IS_ERR(priv.rxdb_ch)) {
    ret = PTR_ERR(priv.rxdb_ch);
    dev_dbg(cl.dev, "failed to request mbox chan rxdb, ret %d\n",
    ret);
    goto free_channel_rx;
    }
    return 0;
    free_channel_rx:
    mbox_free_channel(priv.rx_ch);
    free_channel_tx:
    mbox_free_channel(priv.tx_ch);
    return ret;
    }
//
// imx_dsp_rproc_mbox_no_alloc()
//
// Empty function for no mailbox between cores
//
// Always return 0
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_mbox_no_alloc(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx_dsp_rproc_mbox_no_alloc(struct imx_dsp_rproc *priv)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_free_mbox(priv: *mut imx_dsp_rproc) {
    static void imx_dsp_rproc_free_mbox(struct imx_dsp_rproc *priv)
    {
    mbox_free_channel(priv.tx_ch);
    mbox_free_channel(priv.rx_ch);
    mbox_free_channel(priv.rxdb_ch);
    }
//
// imx_dsp_rproc_add_carveout() - request mailbox channels
// @priv: private data pointer
//
// This function registers specified memory entry in @rproc carveouts list
// The carveouts can help to mapping the memory address for DSP.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_add_carveout(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx_dsp_rproc_add_carveout(struct imx_dsp_rproc *priv)
    {
    const struct imx_dsp_rproc_dcfg *dsp_dcfg = priv.dsp_dcfg;
    const struct imx_rproc_dcfg *dcfg = dsp_dcfg.dcfg;
    struct rproc *rproc = priv.rproc;
    struct device *dev = rproc.dev.parent;
    struct device_node *np = dev.of_node;
    struct rproc_mem_entry *mem;
    int a, i = 0;
    u64 da;
// Remap required addresses
    for (a = 0; a < dcfg.att_size; a++) {
    const struct imx_rproc_att *att = &dcfg.att[a];
    if (!(att.flags & ATT_OWN))
    continue;
    if (imx_dsp_rproc_sys_to_da(priv, att.sa, att.size, &da))
    return -EINVAL;
// Register memory region
    mem = rproc_mem_entry_init(dev, core::ptr::null_mut(), (dma_addr_t)att.sa,
    att.size, da,
    rproc_mem_entry_ioremap_wc,
    rproc_mem_entry_iounmap,
    "dsp_mem");
    if (mem)
    rproc_coredump_add_segment(rproc, da, att.size);
    else
    return -ENOMEM;
    rproc_add_carveout(rproc, mem);
    }
    while (1) {
    int err;
    struct resource res;
    err = of_reserved_mem_region_to_resource(np, i++, &res);
    if (err)
    return 0;
//
// Ignore the first memory region which will be used vdev buffer.
// No need to do extra handlings, rproc_add_virtio_dev will handle it.
//
    if (strstarts(res.name, "vdev0buffer"))
    continue;
    if (imx_dsp_rproc_sys_to_da(priv, res.start, resource_size(&res), &da))
    return -EINVAL;
// Register memory region
    mem = rproc_mem_entry_init(dev, core::ptr::null_mut(), (dma_addr_t)res.start,
    resource_size(&res), da,
    rproc_mem_entry_ioremap_wc,
    rproc_mem_entry_iounmap,
    "%.*s", strchrnul(res.name, '@') - res.name, res.name);
    if (!mem)
    return -ENOMEM;
    rproc_coredump_add_segment(rproc, da, resource_size(&res));
    rproc_add_carveout(rproc, mem);
    }
    }
// Prepare function for rproc_ops
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_prepare(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_prepare(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    struct device *dev = rproc.dev.parent;
    int ret;
    ret = imx_dsp_rproc_add_carveout(priv);
    if (ret) {
    dev_err(dev, "failed on imx_dsp_rproc_add_carveout\n");
    return ret;
    }
    pm_runtime_get_sync(dev);
    return 0;
    }
// Unprepare function for rproc_ops
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_unprepare(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_unprepare(struct rproc *rproc)
    {
    pm_runtime_put_sync(rproc.dev.parent);
    return 0;
    }
// Kick function for rproc_ops
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_kick(rproc: *mut rproc, vqid: c_int) {
    static void imx_dsp_rproc_kick(struct rproc *rproc, int vqid)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    struct device *dev = rproc.dev.parent;
    int err;
    __u32 mmsg;
    if (!priv.tx_ch) {
    dev_err(dev, "No initialized mbox tx channel\n");
    return;
    }
//
// Send the index of the triggered virtqueue as the mu payload.
// Let remote processor know which virtqueue is used.
//
    mmsg = vqid;
    err = mbox_send_message(priv.tx_ch, (void *)&mmsg);
    if (err < 0)
    dev_err(dev, "%s: failed (%d, err:%d)\n", __func__, vqid, err);
    }
//
// Custom memory copy implementation for i.MX DSP Cores
//
// The IRAM is part of the HiFi DSP.
// According to hw specs only 32-bits writes are allowed.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_memcpy(dst: *mut c_void, src: *const c_void, size: usize) -> c_int {
    static int imx_dsp_rproc_memcpy(void *dst, const void *src, size_t size)
    {
    void __iomem *dest = (void __iomem *)dst;
    const u8 *src_byte = src;
    const u32 *source = src;
    u32 affected_mask;
    int i, q, r;
    u32 tmp;
// destination must be 32bit aligned
    if (!IS_ALIGNED((uintptr_t)dest, 4))
    return -EINVAL;
    q = size / 4;
    r = size % 4;
// copy data in units of 32 bits at a time
    for (i = 0; i < q; i++)
    writel(source[i], dest + i * 4);
    if (r) {
    affected_mask = GENMASK(8 * r, 0);
//
// first read the 32bit data of dest, then change affected
// bytes, and write back to dest.
// For unaffected bytes, it should not be changed
//
    tmp = readl(dest + q * 4);
    tmp &= ~affected_mask;
// avoid reading after end of source
    for (i = 0; i < r; i++)
    tmp |= (src_byte[q * 4 + i] << (8 * i));
    writel(tmp, dest + q * 4);
    }
    return 0;
    }
//
// Custom memset implementation for i.MX DSP Cores
//
// The IRAM is part of the HiFi DSP.
// According to hw specs only 32-bits writes are allowed.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_memset(addr: *mut c_void, value: u8, size: usize) -> c_int {
    static int imx_dsp_rproc_memset(void *addr, u8 value, size_t size)
    {
    void __iomem *tmp_dst = (void __iomem *)addr;
    let mut tmp_val: u32 = value;
    u32 affected_mask;
    int q, r;
    u32 tmp;
// destination must be 32bit aligned
    if (!IS_ALIGNED((uintptr_t)addr, 4))
    return -EINVAL;
    tmp_val |= tmp_val << 8;
    tmp_val |= tmp_val << 16;
    q = size / 4;
    r = size % 4;
    while (q--)
    writel(tmp_val, tmp_dst++);
    if (r) {
    affected_mask = GENMASK(8 * r, 0);
//
// first read the 32bit data of addr, then change affected
// bytes, and write back to addr.
// For unaffected bytes, it should not be changed
//
    tmp = readl(tmp_dst);
    tmp &= ~affected_mask;
    tmp |= (tmp_val & affected_mask);
    writel(tmp, tmp_dst);
    }
    return 0;
    }
//
// imx_dsp_rproc_elf_load_segments() - load firmware segments to memory
// @rproc: remote processor which will be booted using these fw segments
// @fw: the ELF firmware image
//
// This function loads the firmware segments to memory, where the remote
// processor expects them.
//
// Return: 0 on success and an appropriate error code otherwise
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_elf_load_segments(rproc: *mut rproc, fw: *const firmware) -> c_int {
    static int imx_dsp_rproc_elf_load_segments(struct rproc *rproc, const struct firmware *fw)
    {
    struct device *dev = &rproc.dev;
    const void *ehdr, *phdr;
    int i, ret = 0;
    u16 phnum;
    const u8 *elf_data = fw.data;
    let mut class: u8 = fw_elf_get_class(fw);
    let mut elf_phdr_get_size: u32 = elf_size_of_phdr(class);
    ehdr = elf_data;
    phnum = elf_hdr_get_e_phnum(class, ehdr);
    phdr = elf_data + elf_hdr_get_e_phoff(class, ehdr);
// go through the available ELF segments
    for (i = 0; i < phnum; i++, phdr += elf_phdr_get_size) {
    let mut da: u64 = elf_phdr_get_p_paddr(class, phdr);
    let mut memsz: u64 = elf_phdr_get_p_memsz(class, phdr);
    let mut filesz: u64 = elf_phdr_get_p_filesz(class, phdr);
    let mut offset: u64 = elf_phdr_get_p_offset(class, phdr);
    let mut type: u32 = elf_phdr_get_p_type(class, phdr);
    void *ptr;
    if (type != PT_LOAD || !memsz)
    continue;
    dev_dbg(dev, "phdr: type %d da 0x%llx memsz 0x%llx filesz 0x%llx\n",
    type, da, memsz, filesz);
    if (filesz > memsz) {
    dev_err(dev, "bad phdr filesz 0x%llx memsz 0x%llx\n",
    filesz, memsz);
    ret = -EINVAL;
    break;
    }
    if (offset + filesz > fw.size) {
    dev_err(dev, "truncated fw: need 0x%llx avail 0x%zx\n",
    offset + filesz, fw.size);
    ret = -EINVAL;
    break;
    }
    if (!rproc_u64_fit_in_size_t(memsz)) {
    dev_err(dev, "size (%llx) does not fit in size_t type\n",
    memsz);
    ret = -EOVERFLOW;
    break;
    }
// grab the kernel address for this device address
    ptr = rproc_da_to_va(rproc, da, memsz, core::ptr::null_mut());
    if (!ptr) {
    dev_err(dev, "bad phdr da 0x%llx mem 0x%llx\n", da,
    memsz);
    ret = -EINVAL;
    break;
    }
// put the segment where the remote processor expects it
    if (filesz) {
    ret = imx_dsp_rproc_memcpy(ptr, elf_data + offset, filesz);
    if (ret) {
    dev_err(dev, "memory copy failed for da 0x%llx memsz 0x%llx\n",
    da, memsz);
    break;
    }
    }
// zero out remaining memory for this segment
    if (memsz > filesz) {
    ret = imx_dsp_rproc_memset(ptr + filesz, 0, memsz - filesz);
    if (ret) {
    dev_err(dev, "memset failed for da 0x%llx memsz 0x%llx\n",
    da, memsz);
    break;
    }
    }
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_parse_fw(rproc: *mut rproc, fw: *const firmware) -> c_int {
    static int imx_dsp_rproc_parse_fw(struct rproc *rproc, const struct firmware *fw)
    {
    rproc_elf_load_rsc_table_optional(rproc, fw, dev_warn,
    "no resource table found for this firmware\n");
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_load(rproc: *mut rproc, fw: *const firmware) -> c_int {
    static int imx_dsp_rproc_load(struct rproc *rproc, const struct firmware *fw)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    const struct imx_dsp_rproc_dcfg *dsp_dcfg = priv.dsp_dcfg;
    struct rproc_mem_entry *carveout;
    int ret;
// Reset DSP if needed
    if (dsp_dcfg.reset)
    dsp_dcfg.reset(priv);
//
// Clear buffers after pm rumtime for internal ocram is not
// accessible if power and clock are not enabled.
//
    if (rproc.state == RPROC_OFFLINE) {
    list_for_each_entry(carveout, &rproc.carveouts, node) {
    if (carveout.va)
    memset(carveout.va, 0, carveout.len);
    }
    }
    ret = imx_dsp_rproc_elf_load_segments(rproc, fw);
    if (ret)
    return ret;
    return 0;
    }
    static const struct rproc_ops imx_dsp_rproc_ops = {
    .prepare	= imx_dsp_rproc_prepare,
    .unprepare	= imx_dsp_rproc_unprepare,
    .start		= imx_dsp_rproc_start,
    .stop		= imx_dsp_rproc_stop,
    .kick		= imx_dsp_rproc_kick,
    .load		= imx_dsp_rproc_load,
    .parse_fw	= imx_dsp_rproc_parse_fw,
    .handle_rsc	= imx_dsp_rproc_handle_rsc,
    .find_loaded_rsc_table = rproc_elf_find_loaded_rsc_table,
    .sanity_check	= rproc_elf_sanity_check,
    .get_boot_addr	= rproc_elf_get_boot_addr,
    };
//
// imx_dsp_attach_pm_domains() - attach the power domains
// @priv: private data pointer
//
// On i.MX8QM and i.MX8QXP there is multiple power domains
// required, so need to link them.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_attach_pm_domains(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx_dsp_attach_pm_domains(struct imx_dsp_rproc *priv)
    {
    struct device *dev = priv.rproc.dev.parent;
// A single PM domain is already attached.
    if (dev.pm_domain)
    return 0;
    return devm_pm_domain_attach_list(dev, core::ptr::null_mut(), &priv.pd_list);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_mmio_detect_mode(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_mmio_detect_mode(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    struct device *dev = rproc.dev.parent;
    struct regmap *regmap;
    regmap = syscon_regmap_lookup_by_phandle(dev.of_node, "fsl,dsp-ctrl");
    if (IS_ERR(regmap)) {
    dev_err(dev, "failed to find syscon\n");
    return PTR_ERR(regmap);
    }
    priv.regmap = regmap;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_reset_ctrl_detect_mode(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_reset_ctrl_detect_mode(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    struct device *dev = rproc.dev.parent;
    priv.run_stall = devm_reset_control_get_exclusive(dev, "runstall");
    if (IS_ERR(priv.run_stall)) {
    dev_err(dev, "Failed to get DSP runstall reset control\n");
    return PTR_ERR(priv.run_stall);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_scu_api_detect_mode(rproc: *mut rproc) -> c_int {
    static int imx_dsp_rproc_scu_api_detect_mode(struct rproc *rproc)
    {
    struct imx_dsp_rproc *priv = rproc.priv;
    return imx_scu_get_handle(&priv.ipc_handle);
    }
//
// imx_dsp_rproc_detect_mode() - detect DSP control mode
// @priv: private data pointer
//
// Different platform has different control method for DSP, which depends
// on how the DSP is integrated in platform.
//
// For i.MX8QXP and i.MX8QM, DSP should be started and stopped by System
// Control Unit.
// For i.MX8MP and i.MX8ULP, DSP should be started and stopped by system
// integration module.
//
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_detect_mode(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx_dsp_rproc_detect_mode(struct imx_dsp_rproc *priv)
    {
    const struct imx_dsp_rproc_dcfg *dsp_dcfg = priv.dsp_dcfg;
    const struct imx_rproc_dcfg *dcfg = dsp_dcfg.dcfg;
    if (dcfg.ops && dcfg.ops.detect_mode)
    return dcfg.ops.detect_mode(priv.rproc);
    return -EOPNOTSUPP;
    }
    static const char *imx_dsp_clks_names[DSP_RPROC_CLK_MAX] = {
// DSP clocks
    "core", "ocram", "debug", "ipg", "mu",
    };
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_clk_get(priv: *mut imx_dsp_rproc) -> c_int {
    static int imx_dsp_rproc_clk_get(struct imx_dsp_rproc *priv)
    {
    struct device *dev = priv.rproc.dev.parent;
    struct clk_bulk_data *clks = priv.clks;
    int i;
    for (i = 0; i < DSP_RPROC_CLK_MAX; i++)
    clks[i].id = imx_dsp_clks_names[i];
    return devm_clk_bulk_get_optional(dev, DSP_RPROC_CLK_MAX, clks);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_rproc_probe(pdev: *mut platform_device) -> c_int {
    static int imx_dsp_rproc_probe(struct platform_device *pdev)
    {
    const struct imx_dsp_rproc_dcfg *dsp_dcfg;
    struct device *dev = &pdev.dev;
    struct imx_dsp_rproc *priv;
    struct rproc *rproc;
    const char *fw_name;
    int ret;
    dsp_dcfg = of_device_get_match_data(dev);
    if (!dsp_dcfg)
    return -ENODEV;
    ret = rproc_of_parse_firmware(dev, 0, &fw_name);
    if (ret)
    return dev_err_probe(dev, ret, "failed to parse firmware-name property\n");
    rproc = devm_rproc_alloc(dev, "imx-dsp-rproc", &imx_dsp_rproc_ops,
    fw_name, sizeof(*priv));
    if (!rproc)
    return -ENOMEM;
    priv = rproc.priv;
    priv.rproc = rproc;
    priv.dsp_dcfg = dsp_dcfg;
// By default, host waits for fw_confirmation reply
    priv.flags |= WAIT_FW_CONFIRMATION;
    if (no_mailboxes)
    imx_dsp_rproc_mbox_init = imx_dsp_rproc_mbox_no_alloc;
    else
    imx_dsp_rproc_mbox_init = imx_dsp_rproc_mbox_alloc;
    dev_set_drvdata(dev, rproc);
    INIT_WORK(&priv.rproc_work, imx_dsp_rproc_vq_work);
    ret = imx_dsp_rproc_detect_mode(priv);
    if (ret)
    return dev_err_probe(dev, ret, "failed on imx_dsp_rproc_detect_mode\n");
// There are multiple power domains required by DSP on some platform
    ret = imx_dsp_attach_pm_domains(priv);
    if (ret < 0)
    return dev_err_probe(dev, ret, "failed on imx_dsp_attach_pm_domains\n");
// Get clocks
    ret = imx_dsp_rproc_clk_get(priv);
    if (ret)
    return dev_err_probe(dev, ret, "failed on imx_dsp_rproc_clk_get\n");
    init_completion(&priv.pm_comp);
    rproc.auto_boot = false;
    ret = devm_rproc_add(dev, rproc);
    if (ret)
    return dev_err_probe(dev, ret, "rproc_add failed\n");
    rproc_coredump_set_elf_info(rproc, ELFCLASS32, EM_XTENSA);
    return devm_pm_runtime_enable(dev);
    }
// pm runtime functions
#[no_mangle]
unsafe extern "C" fn imx_dsp_runtime_resume(dev: *mut device) -> c_int {
    static int imx_dsp_runtime_resume(struct device *dev)
    {
    struct rproc *rproc = dev_get_drvdata(dev);
    struct imx_dsp_rproc *priv = rproc.priv;
    int ret;
//
// There is power domain attached with mailbox, if setup mailbox
// in probe(), then the power of mailbox is always enabled,
// the power can't be saved.
// So move setup of mailbox to runtime resume.
//
    ret = imx_dsp_rproc_mbox_init(priv);
    if (ret) {
    dev_err(dev, "failed on imx_dsp_rproc_mbox_init\n");
    return ret;
    }
    ret = clk_bulk_prepare_enable(DSP_RPROC_CLK_MAX, priv.clks);
    if (ret) {
    dev_err(dev, "failed on clk_bulk_prepare_enable\n");
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_runtime_suspend(dev: *mut device) -> c_int {
    static int imx_dsp_runtime_suspend(struct device *dev)
    {
    struct rproc *rproc = dev_get_drvdata(dev);
    struct imx_dsp_rproc *priv = rproc.priv;
    clk_bulk_disable_unprepare(DSP_RPROC_CLK_MAX, priv.clks);
    imx_dsp_rproc_free_mbox(priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_load_firmware(fw: *const firmware, context: *mut c_void) {
    static void imx_dsp_load_firmware(const struct firmware *fw, void *context)
    {
    struct rproc *rproc = context;
    int ret;
//
// Same flow as start procedure.
// Load the ELF segments to memory firstly.
//
    ret = rproc_load_segments(rproc, fw);
    if (ret)
    goto out;
// Start the remote processor
    ret = rproc.ops.start(rproc);
    if (ret)
    goto out;
    rproc.ops.kick(rproc, 0);
    out:
    release_firmware(fw);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_suspend(dev: *mut device) -> c_int {
    static int imx_dsp_suspend(struct device *dev)
    {
    struct rproc *rproc = dev_get_drvdata(dev);
    struct imx_dsp_rproc *priv = rproc.priv;
    let mut mmsg: __u32 = RP_MBOX_SUSPEND_SYSTEM;
    int ret;
    if (rproc.state != RPROC_RUNNING)
    goto out;
//
// No channel available for sending messages;
// indicates no mailboxes present, so trigger PM runtime suspend
//
    if (!priv.tx_ch) {
    dev_dbg(dev, "No initialized mbox tx channel, suspend directly.\n");
    goto out;
    }
// No fw confirmation expected, so trigger PM runtime suspend
    if (!(priv.flags & WAIT_FW_CONFIRMATION)) {
    dev_dbg(dev, "No FW_CONFIRMATION needed, suspend directly.\n");
    goto out;
    }
    reinit_completion(&priv.pm_comp);
// Tell DSP that suspend is happening
    ret = mbox_send_message(priv.tx_ch, (void *)&mmsg);
    if (ret < 0) {
    dev_err(dev, "PM mbox_send_message failed: %d\n", ret);
    return ret;
    }
//
// DSP need to save the context at suspend.
// Here waiting the response for DSP, then power can be disabled.
//
    if (!wait_for_completion_timeout(&priv.pm_comp, msecs_to_jiffies(100)))
    return -EBUSY;
    out:
//
// The power of DSP is disabled in suspend, so force pm runtime
// to be suspend, then we can reenable the power and clocks at
// resume stage.
//
    return pm_runtime_force_suspend(dev);
    }
#[no_mangle]
unsafe extern "C" fn imx_dsp_resume(dev: *mut device) -> c_int {
    static int imx_dsp_resume(struct device *dev)
    {
    struct rproc *rproc = dev_get_drvdata(dev);
    let mut ret: c_int = 0;
    ret = pm_runtime_force_resume(dev);
    if (ret)
    return ret;
    if (rproc.state != RPROC_RUNNING)
    return 0;
//
// The power of DSP is disabled at suspend, the memory of dsp
// is reset, the image segments are lost. So need to reload
// firmware and restart the DSP if it is in running state.
//
    ret = request_firmware_nowait(THIS_MODULE, FW_ACTION_UEVENT,
    rproc.firmware, dev, GFP_KERNEL,
    rproc, imx_dsp_load_firmware);
    if (ret < 0) {
    dev_err(dev, "load firmware failed: %d\n", ret);
    goto err;
    }
    return 0;
    err:
    pm_runtime_force_suspend(dev);
    return ret;
    }
    static const struct dev_pm_ops imx_dsp_rproc_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(imx_dsp_suspend, imx_dsp_resume)
    RUNTIME_PM_OPS(imx_dsp_runtime_suspend, imx_dsp_runtime_resume, core::ptr::null_mut())
    };
    static const struct imx_rproc_plat_ops imx_dsp_rproc_ops_mmio = {
    .start		= imx_dsp_rproc_mmio_start,
    .stop		= imx_dsp_rproc_mmio_stop,
    .detect_mode	= imx_dsp_rproc_mmio_detect_mode,
    };
    static const struct imx_rproc_plat_ops imx_dsp_rproc_ops_reset_ctrl = {
    .start		= imx_dsp_rproc_reset_ctrl_start,
    .stop		= imx_dsp_rproc_reset_ctrl_stop,
    .detect_mode	= imx_dsp_rproc_reset_ctrl_detect_mode,
    };
    static const struct imx_rproc_plat_ops imx_dsp_rproc_ops_scu_api = {
    .start		= imx_dsp_rproc_scu_api_start,
    .stop		= imx_dsp_rproc_scu_api_stop,
    .detect_mode	= imx_dsp_rproc_scu_api_detect_mode,
    };
// Specific configuration for i.MX8MP
    static const struct imx_rproc_dcfg dsp_rproc_cfg_imx8mp = {
    .att		= imx_dsp_rproc_att_imx8mp,
    .att_size	= ARRAY_SIZE(imx_dsp_rproc_att_imx8mp),
    .ops		= &imx_dsp_rproc_ops_reset_ctrl,
    };
    static const struct imx_dsp_rproc_dcfg imx_dsp_rproc_cfg_imx8mp = {
    .dcfg		= &dsp_rproc_cfg_imx8mp,
    .reset          = imx8mp_dsp_reset,
    };
// Specific configuration for i.MX8ULP
    static const struct imx_rproc_dcfg dsp_rproc_cfg_imx8ulp = {
    .src_reg	= IMX8ULP_SIM_LPAV_REG_SYSCTRL0,
    .src_mask	= IMX8ULP_SYSCTRL0_DSP_STALL,
    .src_start	= 0,
    .src_stop	= IMX8ULP_SYSCTRL0_DSP_STALL,
    .att		= imx_dsp_rproc_att_imx8ulp,
    .att_size	= ARRAY_SIZE(imx_dsp_rproc_att_imx8ulp),
    .ops		= &imx_dsp_rproc_ops_mmio,
    };
    static const struct imx_dsp_rproc_dcfg imx_dsp_rproc_cfg_imx8ulp = {
    .dcfg		= &dsp_rproc_cfg_imx8ulp,
    .reset          = imx8ulp_dsp_reset,
    };
// Specific configuration for i.MX8QXP
    static const struct imx_rproc_dcfg dsp_rproc_cfg_imx8qxp = {
    .att		= imx_dsp_rproc_att_imx8qxp,
    .att_size	= ARRAY_SIZE(imx_dsp_rproc_att_imx8qxp),
    .ops		= &imx_dsp_rproc_ops_scu_api,
    };
    static const struct imx_dsp_rproc_dcfg imx_dsp_rproc_cfg_imx8qxp = {
    .dcfg		= &dsp_rproc_cfg_imx8qxp,
    };
// Specific configuration for i.MX8QM
    static const struct imx_rproc_dcfg dsp_rproc_cfg_imx8qm = {
    .att		= imx_dsp_rproc_att_imx8qm,
    .att_size	= ARRAY_SIZE(imx_dsp_rproc_att_imx8qm),
    .ops		= &imx_dsp_rproc_ops_scu_api,
    };
    static const struct imx_dsp_rproc_dcfg imx_dsp_rproc_cfg_imx8qm = {
    .dcfg		= &dsp_rproc_cfg_imx8qm,
    };
    static const struct of_device_id imx_dsp_rproc_of_match[] = {
    { .compatible = "fsl,imx8qxp-hifi4", .data = &imx_dsp_rproc_cfg_imx8qxp },
    { .compatible = "fsl,imx8qm-hifi4",  .data = &imx_dsp_rproc_cfg_imx8qm },
    { .compatible = "fsl,imx8mp-hifi4",  .data = &imx_dsp_rproc_cfg_imx8mp },
    { .compatible = "fsl,imx8ulp-hifi4", .data = &imx_dsp_rproc_cfg_imx8ulp },
    {},
    };
    MODULE_DEVICE_TABLE(of, imx_dsp_rproc_of_match);
    static struct platform_driver imx_dsp_rproc_driver = {
    .probe = imx_dsp_rproc_probe,
    .driver = {
    .name = "imx-dsp-rproc",
    .of_match_table = imx_dsp_rproc_of_match,
    .pm = pm_ptr(&imx_dsp_rproc_pm_ops),
    },
    };
    module_platform_driver(imx_dsp_rproc_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("i.MX HiFi Core Remote Processor Control Driver");
    MODULE_AUTHOR("Shengjiu Wang <shengjiu.wang@nxp.com>");
