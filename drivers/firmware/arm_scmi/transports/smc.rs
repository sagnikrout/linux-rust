//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/transports/smc.c
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
// System Control and Management Interface (SCMI) Message SMC/HVC
// Transport driver
//
// Copyright 2020 NXP
//

//
// The shmem address is split into 4K page and offset.
// This is to make sure the parameters fit in 32bit arguments of the
// smc/hvc call to keep it uniform across smc32/smc64 conventions.
// This however limits the shmem address to 44 bit.
//
// These optional parameters can be used to distinguish among multiple
// scmi instances that are using the same smc-id.
// The page parameter is passed in r1/x1/w1 register and the offset parameter
// is passed in r2/x2/w2 register.
//

pub const SHMEM_SHIFT: c_int = 12;

//
// struct scmi_smc - Structure representing a SCMI smc transport
//
// @irq: An optional IRQ for completion
// @cinfo: SCMI channel info
// @shmem: Transmit/Receive shared memory area
// @io_ops: Transport specific I/O operations
// @shmem_lock: Lock to protect access to Tx/Rx shared memory area.
// Used when NOT operating in atomic mode.
// @inflight: Atomic flag to protect access to Tx/Rx shared memory area.
// Used when operating in atomic mode.
// @func_id: smc/hvc call function id
// @param_page: 4K page number of the shmem channel
// @param_offset: Offset within the 4K page of the shmem channel
// @cap_id: smc/hvc doorbell's capability id to be used on Qualcomm virtual
// platforms
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_smc {
    pub irq: c_int,
    pub cinfo: *mut scmi_chan_info,
    pub shmem: *mut scmi_shared_mem __iomem,
    pub io_ops: *mut scmi_shmem_io_ops,
// Protect access to shmem area
    pub shmem_lock: mutex,

    pub inflight: core::sync::atomic::AtomicI32,
    pub func_id: c_ulong,
    pub param_page: c_ulong,
    pub param_offset: c_ulong,
    pub cap_id: c_ulong,
}

    static struct scmi_transport_core_operations *core;
#[no_mangle]
unsafe extern "C" fn smc_msg_done_isr(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t smc_msg_done_isr(int irq, void *data)
    {
    struct scmi_smc *scmi_info = data;
    core.rx_callback(scmi_info.cinfo,
    core.shmem.read_header(scmi_info.shmem), core::ptr::null_mut());
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn smc_chan_available(of_node: *mut device_node, idx: c_int) -> bool {
    static bool smc_chan_available(struct device_node *of_node, int idx)
    {
    struct device_node *np __free(device_node) =
    of_parse_phandle(of_node, "shmem", 0);
    if (!np)
    return false;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn smc_channel_lock_init(scmi_info: *mut scmi_smc) {
    static inline void smc_channel_lock_init(struct scmi_smc *scmi_info)
    {
    if (IS_ENABLED(CONFIG_ARM_SCMI_TRANSPORT_SMC_ATOMIC_ENABLE))
    atomic_set(&scmi_info.inflight, INFLIGHT_NONE);
    else
    mutex_init(&scmi_info.shmem_lock);
    }
#[no_mangle]
unsafe extern "C" fn smc_xfer_inflight(xfer: *mut scmi_xfer, inflight: *mut core::sync::atomic::AtomicI32) -> bool {
    static bool smc_xfer_inflight(struct scmi_xfer *xfer, atomic_t *inflight)
    {
    int ret;
    ret = atomic_cmpxchg(inflight, INFLIGHT_NONE, xfer.hdr.seq);
    let mut ret: return = = INFLIGHT_NONE;
    }
    static inline void
    smc_channel_lock_acquire(struct scmi_smc *scmi_info,
    struct scmi_xfer *xfer __maybe_unused)
    {
    if (IS_ENABLED(CONFIG_ARM_SCMI_TRANSPORT_SMC_ATOMIC_ENABLE))
    spin_until_cond(smc_xfer_inflight(xfer, &scmi_info.inflight));
    else
    mutex_lock(&scmi_info.shmem_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn smc_channel_lock_release(scmi_info: *mut scmi_smc) {
    static inline void smc_channel_lock_release(struct scmi_smc *scmi_info)
    {
    if (IS_ENABLED(CONFIG_ARM_SCMI_TRANSPORT_SMC_ATOMIC_ENABLE))
    atomic_set(&scmi_info.inflight, INFLIGHT_NONE);
    else
    mutex_unlock(&scmi_info.shmem_lock);
    }
    static int smc_chan_setup(struct scmi_chan_info *cinfo, struct device *dev,
    bool tx)
    {
    struct device *cdev = cinfo.dev;
    let mut cap_id: c_ulong = ULONG_MAX;
    struct scmi_smc *scmi_info;
    let mut res: resource = {};
    u32 func_id;
    int ret;
    if (!tx)
    return -ENODEV;
    scmi_info = devm_kzalloc(dev, sizeof(*scmi_info), GFP_KERNEL);
    if (!scmi_info)
    return -ENOMEM;
    scmi_info.shmem = core.shmem.setup_iomap(cinfo, dev, tx, &res,
    &scmi_info.io_ops);
    if (IS_ERR(scmi_info.shmem))
    return PTR_ERR(scmi_info.shmem);
    ret = of_property_read_u32(dev.of_node, "arm,smc-id", &func_id);
    if (ret < 0)
    return ret;
    if (of_device_is_compatible(dev.of_node, "qcom,scmi-smc")) {
    let mut size: resource_size_t = resource_size(&res);
    void __iomem *ptr = (void __iomem *)scmi_info.shmem + size - 8;
// The capability-id is kept in last 8 bytes of shmem.
// +-------+ <-- 0
// | shmem |
// +-------+ <-- size - 8
// | capId |
// +-------+ <-- size
//
    memcpy_fromio(&cap_id, ptr, sizeof(cap_id));
    }
    if (of_device_is_compatible(dev.of_node, "arm,scmi-smc-param")) {
    scmi_info.param_page = SHMEM_PAGE(res.start);
    scmi_info.param_offset = SHMEM_OFFSET(res.start);
    }
    scmi_info.func_id = func_id;
    scmi_info.cap_id = cap_id;
    scmi_info.cinfo = cinfo;
    smc_channel_lock_init(scmi_info);
    cinfo.transport_info = scmi_info;
//
// If there is an interrupt named "a2p", then the service and
// completion of a message is signaled by an interrupt rather than by
// the return of the SMC call.
//
    scmi_info.irq = of_irq_get_byname(cdev.of_node, "a2p");
    if (scmi_info.irq > 0) {
    ret = request_irq(scmi_info.irq, smc_msg_done_isr,
    IRQF_NO_SUSPEND, dev_name(dev), scmi_info);
    if (ret) {
    dev_err(dev, "failed to setup SCMI smc irq\n");
    cinfo.transport_info = core::ptr::null_mut();
    scmi_info.cinfo = core::ptr::null_mut();
    return ret;
    }
    } else {
    cinfo.no_completion_irq = true;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn smc_chan_free(id: c_int, p: *mut c_void, data: *mut c_void) -> c_int {
    static int smc_chan_free(int id, void *p, void *data)
    {
    struct scmi_chan_info *cinfo = p;
    struct scmi_smc *scmi_info = cinfo.transport_info;
//
// Different protocols might share the same chan info, so a previous
// smc_chan_free call might have already freed the structure.
//
    if (!scmi_info)
    return 0;
// Ignore any possible further reception on the IRQ path
    if (scmi_info.irq > 0)
    free_irq(scmi_info.irq, scmi_info);
    cinfo.transport_info = core::ptr::null_mut();
    scmi_info.cinfo = core::ptr::null_mut();
    return 0;
    }
    static int smc_send_message(struct scmi_chan_info *cinfo,
    struct scmi_xfer *xfer)
    {
    struct scmi_smc *scmi_info = cinfo.transport_info;
    struct arm_smccc_res res;
//
// Channel will be released only once response has been
// surely fully retrieved, so after .mark_txdone()
//
    smc_channel_lock_acquire(scmi_info, xfer);
    core.shmem.tx_prepare(scmi_info.shmem, xfer, cinfo,
    scmi_info.io_ops.toio);
    if (scmi_info.cap_id != ULONG_MAX)
    arm_smccc_1_1_invoke(scmi_info.func_id, scmi_info.cap_id, 0,
    0, 0, 0, 0, 0, &res);
    else
    arm_smccc_1_1_invoke(scmi_info.func_id, scmi_info.param_page,
    scmi_info.param_offset, 0, 0, 0, 0, 0,
    &res);
// Only SMCCC_RET_NOT_SUPPORTED is valid error code
    if (res.a0) {
    smc_channel_lock_release(scmi_info);
    return -EOPNOTSUPP;
    }
    return 0;
    }
    static void smc_fetch_response(struct scmi_chan_info *cinfo,
    struct scmi_xfer *xfer)
    {
    struct scmi_smc *scmi_info = cinfo.transport_info;
    core.shmem.fetch_response(scmi_info.shmem, xfer,
    scmi_info.io_ops.fromio);
    }
    static void smc_mark_txdone(struct scmi_chan_info *cinfo, int ret,
    struct scmi_xfer *__unused)
    {
    struct scmi_smc *scmi_info = cinfo.transport_info;
    smc_channel_lock_release(scmi_info);
    }
    static const struct scmi_transport_ops scmi_smc_ops = {
    .chan_available = smc_chan_available,
    .chan_setup = smc_chan_setup,
    .chan_free = smc_chan_free,
    .send_message = smc_send_message,
    .mark_txdone = smc_mark_txdone,
    .fetch_response = smc_fetch_response,
    };
    static struct scmi_desc scmi_smc_desc = {
    .ops = &scmi_smc_ops,
    .max_rx_timeout_ms = 30,
    .max_msg = 20,
    .max_msg_size = SCMI_SHMEM_MAX_PAYLOAD_SIZE,
//
// Setting .sync_cmds_atomic_replies to true for SMC assumes that,
// once the SMC instruction has completed successfully, the issued
// SCMI command would have been already fully processed by the SCMI
// platform firmware and so any possible response value expected
// for the issued command will be immmediately ready to be fetched
// from the shared memory area.
//
    .sync_cmds_completed_on_ret = true,
    .atomic_enabled = IS_ENABLED(CONFIG_ARM_SCMI_TRANSPORT_SMC_ATOMIC_ENABLE),
    };
    static const struct of_device_id scmi_of_match[] = {
    { .compatible = "arm,scmi-smc" },
    { .compatible = "arm,scmi-smc-param" },
    { .compatible = "qcom,scmi-smc" },
    { /* Sentinel */ },
    };
    MODULE_DEVICE_TABLE(of, scmi_of_match);
    DEFINE_SCMI_TRANSPORT_DRIVER(scmi_smc, scmi_smc_driver, scmi_smc_desc,
    scmi_of_match, core);
    module_platform_driver(scmi_smc_driver);
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_AUTHOR("Nikunj Kela <quic_nkela@quicinc.com>");
    MODULE_DESCRIPTION("SCMI SMC Transport driver");
    MODULE_LICENSE("GPL");
