//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/arm_scmi/transports/optee.c
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
// Copyright (C) 2019-2021 Linaro Ltd.
//

    enum scmi_optee_pta_cmd {
//
// PTA_SCMI_CMD_CAPABILITIES - Get channel capabilities
//
// [out]    value[0].a: Capability bit mask (enum pta_scmi_caps)
// [out]    value[0].b: Extended capabilities or 0
//
    PTA_SCMI_CMD_CAPABILITIES = 0,
//
// PTA_SCMI_CMD_PROCESS_SMT_CHANNEL - Process SCMI message in SMT buffer
//
// [in]     value[0].a: Channel handle
//
// Shared memory used for SCMI message/response exhange is expected
// already identified and bound to channel handle in both SCMI agent
// and SCMI server (OP-TEE) parts.
// The memory uses SMT header to carry SCMI meta-data (protocol ID and
// protocol message ID).
//
    PTA_SCMI_CMD_PROCESS_SMT_CHANNEL = 1,
//
// PTA_SCMI_CMD_PROCESS_SMT_CHANNEL_MESSAGE - Process SMT/SCMI message
//
// [in]     value[0].a: Channel handle
// [in/out] memref[1]: Message/response buffer (SMT and SCMI payload)
//
// Shared memory used for SCMI message/response is a SMT buffer
// referenced by param[1]. It shall be 128 bytes large to fit response
// payload whatever message playload size.
// The memory uses SMT header to carry SCMI meta-data (protocol ID and
// protocol message ID).
//
    PTA_SCMI_CMD_PROCESS_SMT_CHANNEL_MESSAGE = 2,
//
// PTA_SCMI_CMD_GET_CHANNEL - Get channel handle
//
// SCMI shm information are 0 if agent expects to use OP-TEE regular SHM
//
// [in]     value[0].a: Channel identifier
// [out]    value[0].a: Returned channel handle
// [in]     value[0].b: Requested capabilities mask (enum pta_scmi_caps)
//
    PTA_SCMI_CMD_GET_CHANNEL = 3,
//
// PTA_SCMI_CMD_PROCESS_MSG_CHANNEL - Process SCMI message in a MSG
// buffer pointed by memref parameters
//
// [in]     value[0].a: Channel handle
// [in]     memref[1]: Message buffer (MSG and SCMI payload)
// [out]    memref[2]: Response buffer (MSG and SCMI payload)
//
// Shared memories used for SCMI message/response are MSG buffers
// referenced by param[1] and param[2]. MSG transport protocol
// uses a 32bit header to carry SCMI meta-data (protocol ID and
// protocol message ID) followed by the effective SCMI message
// payload.
//
    PTA_SCMI_CMD_PROCESS_MSG_CHANNEL = 4,
    };
//
// OP-TEE SCMI service capabilities bit flags (32bit)
//
// PTA_SCMI_CAPS_SMT_HEADER
// When set, OP-TEE supports command using SMT header protocol (SCMI shmem) in
// shared memory buffers to carry SCMI protocol synchronisation information.
//
// PTA_SCMI_CAPS_MSG_HEADER
// When set, OP-TEE supports command using MSG header protocol in an OP-TEE
// shared memory to carry SCMI protocol synchronisation information and SCMI
// message payload.
//
pub const PTA_SCMI_CAPS_NONE: c_int = 0;

    PTA_SCMI_CAPS_MSG_HEADER)
//
// struct scmi_optee_channel - Description of an OP-TEE SCMI channel
//
// @channel_id: OP-TEE channel ID used for this transport
// @tee_session: TEE session identifier
// @caps: OP-TEE SCMI channel capabilities
// @rx_len: Response size
// @mu: Mutex protection on channel access
// @cinfo: SCMI channel information
// @req: union for SCMI interface
// @req.shmem: Virtual base address of the shared memory
// @req.msg: Shared memory protocol handle for SCMI request and
// synchronous response
// @io_ops: Transport specific I/O operations
// @tee_shm: TEE shared memory handle @req or NULL if using IOMEM shmem
// @link: Reference in agent's channel list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_optee_channel {
    pub channel_id: u32,
    pub tee_session: u32,
    pub caps: u32,
    pub rx_len: u32,
    pub mu: mutex,
    pub cinfo: *mut scmi_chan_info,
    union {
    pub shmem: *mut scmi_shared_mem __iomem,
    pub msg: *mut scmi_msg_payld,
    pub req: },
    pub io_ops: *mut scmi_shmem_io_ops,
    pub tee_shm: *mut tee_shm,
    pub link: list_head,
}

//
// struct scmi_optee_agent - OP-TEE transport private data
//
// @dev: Device used for communication with TEE
// @tee_ctx: TEE context used for communication
// @caps: Supported channel capabilities
// @mu: Mutex for protection of @channel_list
// @channel_list: List of all created channels for the agent
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scmi_optee_agent {
    pub dev: *mut device,
    pub tee_ctx: *mut tee_context,
    pub caps: u32,
    pub mu: mutex,
    pub channel_list: list_head,
}

    static struct scmi_transport_core_operations *core;
// There can be only 1 SCMI service in OP-TEE we connect to
    static struct scmi_optee_agent *scmi_optee_private;
    static DEFINE_SCMI_TRANSPORT_SUPPLIER(scmi_optee_supplier);
// Open a session toward SCMI OP-TEE service with REE_KERNEL identity
#[no_mangle]
unsafe extern "C" fn open_session(agent: *mut scmi_optee_agent, tee_session: *mut u32) -> c_int {
    static int open_session(struct scmi_optee_agent *agent, u32 *tee_session)
    {
    struct device *dev = agent.dev;
    struct tee_client_device *scmi_pta = to_tee_client_device(dev);
    let mut arg: tee_ioctl_open_session_arg = { };
    int ret;
    memcpy(arg.uuid, scmi_pta.id.uuid.b, TEE_IOCTL_UUID_LEN);
    arg.clnt_login = TEE_IOCTL_LOGIN_REE_KERNEL;
    ret = tee_client_open_session(agent.tee_ctx, &arg, core::ptr::null_mut());
    if (ret < 0 || arg.ret) {
    dev_err(dev, "Can't open tee session: %d / %#x\n", ret, arg.ret);
    return -EOPNOTSUPP;
    }
// tee_session = arg.session;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn close_session(agent: *mut scmi_optee_agent, tee_session: u32) {
    static void close_session(struct scmi_optee_agent *agent, u32 tee_session)
    {
    tee_client_close_session(agent.tee_ctx, tee_session);
    }
#[no_mangle]
unsafe extern "C" fn get_capabilities(agent: *mut scmi_optee_agent) -> c_int {
    static int get_capabilities(struct scmi_optee_agent *agent)
    {
    let mut arg: tee_ioctl_invoke_arg = { };
    struct tee_param param[1] = { };
    u32 caps;
    u32 tee_session;
    int ret;
    ret = open_session(agent, &tee_session);
    if (ret)
    return ret;
    arg.func = PTA_SCMI_CMD_CAPABILITIES;
    arg.session = tee_session;
    arg.num_params = 1;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_OUTPUT;
    ret = tee_client_invoke_func(agent.tee_ctx, &arg, param);
    close_session(agent, tee_session);
    if (ret < 0 || arg.ret) {
    dev_err(agent.dev, "Can't get capabilities: %d / %#x\n", ret, arg.ret);
    return -EOPNOTSUPP;
    }
    caps = param[0].u.value.a;
    if (!(caps & (PTA_SCMI_CAPS_SMT_HEADER | PTA_SCMI_CAPS_MSG_HEADER))) {
    dev_err(agent.dev, "OP-TEE SCMI PTA doesn't support SMT and MSG\n");
    return -EOPNOTSUPP;
    }
    agent.caps = caps;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_channel(channel: *mut scmi_optee_channel) -> c_int {
    static int get_channel(struct scmi_optee_channel *channel)
    {
    struct device *dev = scmi_optee_private.dev;
    let mut arg: tee_ioctl_invoke_arg = { };
    struct tee_param param[1] = { };
    let mut caps: c_uint = 0;
    int ret;
    if (channel.tee_shm)
    caps = PTA_SCMI_CAPS_MSG_HEADER;
    else
    caps = PTA_SCMI_CAPS_SMT_HEADER;
    arg.func = PTA_SCMI_CMD_GET_CHANNEL;
    arg.session = channel.tee_session;
    arg.num_params = 1;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INOUT;
    param[0].u.value.a = channel.channel_id;
    param[0].u.value.b = caps;
    ret = tee_client_invoke_func(scmi_optee_private.tee_ctx, &arg, param);
    if (ret || arg.ret) {
    dev_err(dev, "Can't get channel with caps %#x: %d / %#x\n", caps, ret, arg.ret);
    return -EOPNOTSUPP;
    }
// From now on use channel identifer provided by OP-TEE SCMI service
    channel.channel_id = param[0].u.value.a;
    channel.caps = caps;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn invoke_process_smt_channel(channel: *mut scmi_optee_channel) -> c_int {
    static int invoke_process_smt_channel(struct scmi_optee_channel *channel)
    {
    struct tee_ioctl_invoke_arg arg = {
    .func = PTA_SCMI_CMD_PROCESS_SMT_CHANNEL,
    .session = channel.tee_session,
    .num_params = 1,
    };
    struct tee_param param[1] = { };
    int ret;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
    param[0].u.value.a = channel.channel_id;
    ret = tee_client_invoke_func(scmi_optee_private.tee_ctx, &arg, param);
    if (ret < 0 || arg.ret) {
    dev_err(scmi_optee_private.dev, "Can't invoke channel %u: %d / %#x\n",
    channel.channel_id, ret, arg.ret);
    return -EIO;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn invoke_process_msg_channel(channel: *mut scmi_optee_channel, msg_size: usize) -> c_int {
    static int invoke_process_msg_channel(struct scmi_optee_channel *channel, size_t msg_size)
    {
    struct tee_ioctl_invoke_arg arg = {
    .func = PTA_SCMI_CMD_PROCESS_MSG_CHANNEL,
    .session = channel.tee_session,
    .num_params = 3,
    };
    struct tee_param param[3] = { };
    int ret;
    param[0].attr = TEE_IOCTL_PARAM_ATTR_TYPE_VALUE_INPUT;
    param[0].u.value.a = channel.channel_id;
    param[1].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_INPUT;
    param[1].u.memref.shm = channel.tee_shm;
    param[1].u.memref.size = msg_size;
    param[2].attr = TEE_IOCTL_PARAM_ATTR_TYPE_MEMREF_OUTPUT;
    param[2].u.memref.shm = channel.tee_shm;
    param[2].u.memref.size = SCMI_SHMEM_MAX_PAYLOAD_SIZE;
    ret = tee_client_invoke_func(scmi_optee_private.tee_ctx, &arg, param);
    if (ret < 0 || arg.ret) {
    dev_err(scmi_optee_private.dev, "Can't invoke channel %u: %d / %#x\n",
    channel.channel_id, ret, arg.ret);
    return -EIO;
    }
// Save response size
    channel.rx_len = param[2].u.memref.size;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn scmi_optee_chan_available(of_node: *mut device_node, idx: c_int) -> bool {
    static bool scmi_optee_chan_available(struct device_node *of_node, int idx)
    {
    u32 channel_id;
    return !of_property_read_u32_index(of_node, "linaro,optee-channel-id",
    idx, &channel_id);
    }
#[no_mangle]
unsafe extern "C" fn scmi_optee_clear_channel(cinfo: *mut scmi_chan_info) {
    static void scmi_optee_clear_channel(struct scmi_chan_info *cinfo)
    {
    struct scmi_optee_channel *channel = cinfo.transport_info;
    if (!channel.tee_shm)
    core.shmem.clear_channel(channel.req.shmem);
    }
#[no_mangle]
unsafe extern "C" fn setup_dynamic_shmem(dev: *mut device, channel: *mut scmi_optee_channel) -> c_int {
    static int setup_dynamic_shmem(struct device *dev, struct scmi_optee_channel *channel)
    {
    let mut msg_size: usize = SCMI_SHMEM_MAX_PAYLOAD_SIZE;
    void *shbuf;
    channel.tee_shm = tee_shm_alloc_kernel_buf(scmi_optee_private.tee_ctx, msg_size);
    if (IS_ERR(channel.tee_shm)) {
    dev_err(channel.cinfo.dev, "shmem allocation failed\n");
    return -ENOMEM;
    }
    shbuf = tee_shm_get_va(channel.tee_shm, 0);
    memset(shbuf, 0, msg_size);
    channel.req.msg = shbuf;
    channel.rx_len = msg_size;
    return 0;
    }
    static int setup_static_shmem(struct device *dev, struct scmi_chan_info *cinfo,
    struct scmi_optee_channel *channel)
    {
    channel.req.shmem = core.shmem.setup_iomap(cinfo, dev, true, core::ptr::null_mut(),
    &channel.io_ops);
    if (IS_ERR(channel.req.shmem))
    return PTR_ERR(channel.req.shmem);
    return 0;
    }
    static int setup_shmem(struct device *dev, struct scmi_chan_info *cinfo,
    struct scmi_optee_channel *channel)
    {
    if (of_property_present(cinfo.dev.of_node, "shmem"))
    return setup_static_shmem(dev, cinfo, channel);
    else
    return setup_dynamic_shmem(dev, channel);
    }
#[no_mangle]
unsafe extern "C" fn scmi_optee_chan_setup(cinfo: *mut scmi_chan_info, dev: *mut device, tx: bool) -> c_int {
    static int scmi_optee_chan_setup(struct scmi_chan_info *cinfo, struct device *dev, bool tx)
    {
    struct scmi_optee_channel *channel;
    uint32_t channel_id;
    int ret;
    if (!tx)
    return -ENODEV;
    channel = devm_kzalloc(dev, sizeof(*channel), GFP_KERNEL);
    if (!channel)
    return -ENOMEM;
    ret = of_property_read_u32_index(cinfo.dev.of_node, "linaro,optee-channel-id",
    0, &channel_id);
    if (ret)
    return ret;
    cinfo.transport_info = channel;
    channel.cinfo = cinfo;
    channel.channel_id = channel_id;
    mutex_init(&channel.mu);
    ret = setup_shmem(dev, cinfo, channel);
    if (ret)
    return ret;
    ret = open_session(scmi_optee_private, &channel.tee_session);
    if (ret)
    goto err_free_shm;
    ret = tee_client_system_session(scmi_optee_private.tee_ctx, channel.tee_session);
    if (ret)
    dev_warn(dev, "Could not switch to system session, do best effort\n");
    ret = get_channel(channel);
    if (ret)
    goto err_close_sess;
// Enable polling
    cinfo.no_completion_irq = true;
    mutex_lock(&scmi_optee_private.mu);
    list_add(&channel.link, &scmi_optee_private.channel_list);
    mutex_unlock(&scmi_optee_private.mu);
    return 0;
    err_close_sess:
    close_session(scmi_optee_private, channel.tee_session);
    err_free_shm:
    if (channel.tee_shm)
    tee_shm_free(channel.tee_shm);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scmi_optee_chan_free(id: c_int, p: *mut c_void, data: *mut c_void) -> c_int {
    static int scmi_optee_chan_free(int id, void *p, void *data)
    {
    struct scmi_chan_info *cinfo = p;
    struct scmi_optee_channel *channel = cinfo.transport_info;
//
// Different protocols might share the same chan info, so a previous
// call might have already freed the structure.
//
    if (!channel)
    return 0;
    mutex_lock(&scmi_optee_private.mu);
    list_del(&channel.link);
    mutex_unlock(&scmi_optee_private.mu);
    close_session(scmi_optee_private, channel.tee_session);
    if (channel.tee_shm) {
    tee_shm_free(channel.tee_shm);
    channel.tee_shm = core::ptr::null_mut();
    }
    cinfo.transport_info = core::ptr::null_mut();
    channel.cinfo = core::ptr::null_mut();
    return 0;
    }
    static int scmi_optee_send_message(struct scmi_chan_info *cinfo,
    struct scmi_xfer *xfer)
    {
    struct scmi_optee_channel *channel = cinfo.transport_info;
    int ret;
    mutex_lock(&channel.mu);
    if (channel.tee_shm) {
    core.msg.tx_prepare(channel.req.msg, xfer);
    ret = invoke_process_msg_channel(channel,
    core.msg.command_size(xfer));
    } else {
    core.shmem.tx_prepare(channel.req.shmem, xfer, cinfo,
    channel.io_ops.toio);
    ret = invoke_process_smt_channel(channel);
    }
    if (ret)
    mutex_unlock(&channel.mu);
    return ret;
    }
    static void scmi_optee_fetch_response(struct scmi_chan_info *cinfo,
    struct scmi_xfer *xfer)
    {
    struct scmi_optee_channel *channel = cinfo.transport_info;
    if (channel.tee_shm)
    core.msg.fetch_response(channel.req.msg,
    channel.rx_len, xfer);
    else
    core.shmem.fetch_response(channel.req.shmem, xfer,
    channel.io_ops.fromio);
    }
    static void scmi_optee_mark_txdone(struct scmi_chan_info *cinfo, int ret,
    struct scmi_xfer *__unused)
    {
    struct scmi_optee_channel *channel = cinfo.transport_info;
    mutex_unlock(&channel.mu);
    }
    static const struct scmi_transport_ops scmi_optee_ops = {
    .chan_available = scmi_optee_chan_available,
    .chan_setup = scmi_optee_chan_setup,
    .chan_free = scmi_optee_chan_free,
    .send_message = scmi_optee_send_message,
    .mark_txdone = scmi_optee_mark_txdone,
    .fetch_response = scmi_optee_fetch_response,
    .clear_channel = scmi_optee_clear_channel,
    };
#[no_mangle]
unsafe extern "C" fn scmi_optee_ctx_match(ver: *mut tee_ioctl_version_data, data: *const c_void) -> c_int {
    static int scmi_optee_ctx_match(struct tee_ioctl_version_data *ver, const void *data)
    {
    return ver.impl_id == TEE_IMPL_ID_OPTEE;
    }
    static struct scmi_desc scmi_optee_desc = {
    .ops = &scmi_optee_ops,
    .max_rx_timeout_ms = 30,
    .max_msg = 20,
    .max_msg_size = SCMI_SHMEM_MAX_PAYLOAD_SIZE,
    .sync_cmds_completed_on_ret = true,
    };
    static const struct of_device_id scmi_of_match[] = {
    { .compatible = "linaro,scmi-optee", .data = &scmi_optee_supplier.th},
    { /* Sentinel */ },
    };
    DEFINE_SCMI_TRANSPORT_DRIVER(scmi_optee, scmi_optee_driver, scmi_optee_desc,
    scmi_of_match, core);
#[no_mangle]
unsafe extern "C" fn scmi_optee_service_probe(scmi_pta: *mut tee_client_device) -> c_int {
    static int scmi_optee_service_probe(struct tee_client_device *scmi_pta)
    {
    struct device *dev = &scmi_pta.dev;
    struct scmi_optee_agent *agent;
    struct tee_context *tee_ctx;
    int ret;
// Only one SCMI OP-TEE device allowed
    if (scmi_optee_private) {
    dev_err(dev, "An SCMI OP-TEE device was already initialized: only one allowed\n");
    return -EBUSY;
    }
    tee_ctx = tee_client_open_context(core::ptr::null_mut(), scmi_optee_ctx_match, core::ptr::null_mut(), core::ptr::null_mut());
    if (IS_ERR(tee_ctx))
    return -ENODEV;
    agent = devm_kzalloc(dev, sizeof(*agent), GFP_KERNEL);
    if (!agent) {
    ret = -ENOMEM;
    goto err;
    }
    agent.dev = dev;
    agent.tee_ctx = tee_ctx;
    INIT_LIST_HEAD(&agent.channel_list);
    mutex_init(&agent.mu);
    ret = get_capabilities(agent);
    if (ret)
    goto err;
// Ensure initialized scmi_optee_private is visible
    smp_mb();
    scmi_optee_private = agent;
    ret = scmi_transport_supplier_put(&scmi_optee_supplier.th, agent.dev);
    if (ret)
    goto err_put;
    return 0;
    err_put:
// Ensure cleared reference is visible before resources are released
    smp_store_mb(scmi_optee_private, core::ptr::null_mut());
    err:
    tee_client_close_context(tee_ctx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn scmi_optee_service_remove(scmi_pta: *mut tee_client_device) {
    static void scmi_optee_service_remove(struct tee_client_device *scmi_pta)
    {
    struct scmi_optee_agent *agent = scmi_optee_private;
    if (!scmi_optee_private)
    return;
    if (!list_empty(&scmi_optee_private.channel_list))
    return;
// Ensure cleared reference is visible before resources are released
    smp_store_mb(scmi_optee_private, core::ptr::null_mut());
    scmi_transport_supplier_put(&scmi_optee_supplier.th, agent.dev);
    tee_client_close_context(agent.tee_ctx);
    }
    static const struct tee_client_device_id scmi_optee_service_id[] = {
    {
    UUID_INIT(0xa8cfe406, 0xd4f5, 0x4a2e,
    0x9f, 0x8d, 0xa2, 0x5d, 0xc7, 0x54, 0xc0, 0x99)
    },
    { }
    };
    MODULE_DEVICE_TABLE(tee, scmi_optee_service_id);
    static struct tee_client_driver scmi_optee_service_driver = {
    .probe = scmi_optee_service_probe,
    .remove = scmi_optee_service_remove,
    .id_table = scmi_optee_service_id,
    .driver = {
    .name = "scmi-optee",
    },
    };
#[no_mangle]
unsafe extern "C" fn scmi_transport_optee_init() -> int __init {
    static int __init scmi_transport_optee_init(void)
    {
    int ret;
    ret = tee_client_driver_register(&scmi_optee_service_driver);
    if (ret)
    return ret;
    ret = platform_driver_register(&scmi_optee_driver);
    if (ret) {
    tee_client_driver_unregister(&scmi_optee_service_driver);
    return ret;
    }
    return ret;
    }
    module_init(scmi_transport_optee_init);
#[no_mangle]
unsafe extern "C" fn scmi_transport_optee_exit() -> void __exit {
    static void __exit scmi_transport_optee_exit(void)
    {
    platform_driver_unregister(&scmi_optee_driver);
    tee_client_driver_unregister(&scmi_optee_service_driver);
    }
    module_exit(scmi_transport_optee_exit);
    MODULE_AUTHOR("Etienne Carriere <etienne.carriere@foss.st.com>");
    MODULE_DESCRIPTION("SCMI OPTEE Transport driver");
    MODULE_LICENSE("GPL");
