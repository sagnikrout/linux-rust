//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/turris-mox-rwtm.c
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
// Turris Mox rWTM firmware driver
//
// Copyright (C) 2019, 2024, 2025 Marek Behún <kabel@kernel.org>
//

//
// The macros and constants below come from Turris Mox's rWTM firmware code.
// This firmware is open source and it's sources can be found at
// https://gitlab.labs.nic.cz/turris/mox-boot-builder/tree/master/wtmi.
//
    enum {
    MOX_ECC_NUM_BITS	= 521,
    MOX_ECC_NUM_LEN		= DIV_ROUND_UP(MOX_ECC_NUM_BITS, 8),
    MOX_ECC_NUM_WORDS	= DIV_ROUND_UP(MOX_ECC_NUM_BITS, 32),
    MOX_ECC_SIG_LEN		= 2 * MOX_ECC_NUM_LEN,
    MOX_ECC_PUBKEY_LEN	= 1 + MOX_ECC_NUM_LEN,
    };

    enum mbox_cmd {
    MBOX_CMD_GET_RANDOM	= 1,
    MBOX_CMD_BOARD_INFO	= 2,
    MBOX_CMD_ECDSA_PUB_KEY	= 3,
    MBOX_CMD_HASH		= 4,
    MBOX_CMD_SIGN		= 5,
    MBOX_CMD_VERIFY		= 6,
    MBOX_CMD_OTP_READ	= 7,
    MBOX_CMD_OTP_WRITE	= 8,
    };
//
// struct mox_rwtm - driver private data structure
// @mbox_client:	rWTM mailbox client
// @mbox:		rWTM mailbox channel
// @hwrng:		RNG driver structure
// @reply:		last mailbox reply, filled in receive callback
// @buf:		DMA buffer
// @buf_phys:		physical address of the DMA buffer
// @busy:		mutex to protect mailbox command execution
// @cmd_done:		command done completion
// @has_board_info:	whether board information is present
// @serial_number:	serial number of the device
// @board_version:	board version / revision of the device
// @ram_size:		RAM size of the device
// @mac_address1:	first MAC address of the device
// @mac_address2:	second MAC address of the device
// @pubkey:		board ECDSA public key
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mox_rwtm {
    pub mbox_client: mbox_client,
    pub mbox: *mut mbox_chan,
    pub hwrng: hwrng,
    pub reply: armada_37xx_rwtm_rx_msg,
    pub buf: *mut c_void,
    pub buf_phys: dma_addr_t,
    pub busy: mutex,
    pub cmd_done: completion,
    pub has_board_info: bool,
    pub serial_number: u64,
    pub ram_size: int board_version,,
    pub mac_address2: [u8 mac_address1[ETH_ALEN],; ETH_ALEN],    pub pubkey: [u8; MOX_ECC_PUBKEY_LEN],
}

    static inline struct device *rwtm_dev(struct mox_rwtm *rwtm)
    {
    return rwtm.mbox_client.dev;
    }

    static ssize_t							\
    name##_show(struct device *dev, struct device_attribute *a,	\
    char *buf)						\
    {								\
    struct mox_rwtm *rwtm = dev_get_drvdata(dev);		\
    if (!rwtm.has_board_info)				\
    return -ENODATA;				\
    return sysfs_emit(buf, format, rwtm.name);		\
    }								\
#[no_mangle]
pub unsafe extern "C" fn DEVICE_ATTR_RO(_arg: name) -> static {
    static DEVICE_ATTR_RO(name)
    MOX_ATTR_RO(serial_number, "%016llX\n");
    MOX_ATTR_RO(board_version, "%i\n");
    MOX_ATTR_RO(ram_size, "%i\n");
    MOX_ATTR_RO(mac_address1, "%pM\n");
    MOX_ATTR_RO(mac_address2, "%pM\n");
    static struct attribute *turris_mox_rwtm_attrs[] = {
    &dev_attr_serial_number.attr,
    &dev_attr_board_version.attr,
    &dev_attr_ram_size.attr,
    &dev_attr_mac_address1.attr,
    &dev_attr_mac_address2.attr,
    core::ptr::null_mut()
    };
    ATTRIBUTE_GROUPS(turris_mox_rwtm);
#[no_mangle]
unsafe extern "C" fn mox_get_status(cmd: enum mbox_cmd, retval: u32) -> c_int {
    static int mox_get_status(enum mbox_cmd cmd, u32 retval)
    {
    if (MBOX_STS_CMD(retval) != cmd)
    return -EIO;
#[no_mangle]
pub unsafe extern "C" fn if(MBOX_STS_FAIL: MBOX_STS_ERROR(retval) ==) -> else {
    else if (MBOX_STS_ERROR(retval) == MBOX_STS_FAIL)
    return -(int)MBOX_STS_VALUE(retval);
#[no_mangle]
pub unsafe extern "C" fn if(MBOX_STS_BADCMD: MBOX_STS_ERROR(retval) ==) -> else {
    else if (MBOX_STS_ERROR(retval) == MBOX_STS_BADCMD)
    return -EOPNOTSUPP;
#[no_mangle]
pub unsafe extern "C" fn if(MBOX_STS_SUCCESS: MBOX_STS_ERROR(retval) !=) -> else {
    else if (MBOX_STS_ERROR(retval) != MBOX_STS_SUCCESS)
    return -EIO;
    else
    return MBOX_STS_VALUE(retval);
    }
#[no_mangle]
unsafe extern "C" fn mox_rwtm_rx_callback(cl: *mut mbox_client, data: *mut c_void) {
    static void mox_rwtm_rx_callback(struct mbox_client *cl, void *data)
    {
    struct mox_rwtm *rwtm = dev_get_drvdata(cl.dev);
    struct armada_37xx_rwtm_rx_msg *msg = data;
    if (completion_done(&rwtm.cmd_done))
    return;
    rwtm.reply = *msg;
    complete(&rwtm.cmd_done);
    }
    static int mox_rwtm_exec(struct mox_rwtm *rwtm, enum mbox_cmd cmd,
    struct armada_37xx_rwtm_tx_msg *msg,
    bool interruptible)
    {
    let mut _msg: armada_37xx_rwtm_tx_msg = {};
    int ret;
    if (!msg)
    msg = &_msg;
    msg.command = cmd;
    ret = mbox_send_message(rwtm.mbox, msg);
    if (ret < 0)
    return ret;
    if (interruptible) {
    ret = wait_for_completion_interruptible(&rwtm.cmd_done);
    if (ret < 0)
    return ret;
    } else {
    if (!wait_for_completion_timeout(&rwtm.cmd_done, HZ / 2))
    return -ETIMEDOUT;
    }
    return mox_get_status(cmd, rwtm.reply.retval);
    }
#[no_mangle]
unsafe extern "C" fn reply_to_mac_addr(mac: *mut u8, t1: u32, t2: u32) {
    static void reply_to_mac_addr(u8 *mac, u32 t1, u32 t2)
    {
    mac[0] = t1 >> 8;
    mac[1] = t1;
    mac[2] = t2 >> 24;
    mac[3] = t2 >> 16;
    mac[4] = t2 >> 8;
    mac[5] = t2;
    }
#[no_mangle]
unsafe extern "C" fn mox_get_board_info(rwtm: *mut mox_rwtm) -> c_int {
    static int mox_get_board_info(struct mox_rwtm *rwtm)
    {
    struct device *dev = rwtm_dev(rwtm);
    struct armada_37xx_rwtm_rx_msg *reply = &rwtm.reply;
    int ret;
    ret = mox_rwtm_exec(rwtm, MBOX_CMD_BOARD_INFO, core::ptr::null_mut(), false);
    if (ret == -ENODATA) {
    dev_warn(dev,
    "Board does not have manufacturing information burned!\n");
    } else if (ret == -EOPNOTSUPP) {
    dev_notice(dev,
    "Firmware does not support the BOARD_INFO command\n");
    } else if (ret < 0) {
    return ret;
    } else {
    rwtm.serial_number = reply.status[1];
    rwtm.serial_number <<= 32;
    rwtm.serial_number |= reply.status[0];
    rwtm.board_version = reply.status[2];
    rwtm.ram_size = reply.status[3];
    reply_to_mac_addr(rwtm.mac_address1, reply.status[4],
    reply.status[5]);
    reply_to_mac_addr(rwtm.mac_address2, reply.status[6],
    reply.status[7]);
    rwtm.has_board_info = true;
    pr_info("Turris Mox serial number %016llX\n",
    rwtm.serial_number);
    pr_info("           board version %i\n", rwtm.board_version);
    pr_info("           burned RAM size %i MiB\n", rwtm.ram_size);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn check_get_random_support(rwtm: *mut mox_rwtm) -> c_int {
    static int check_get_random_support(struct mox_rwtm *rwtm)
    {
    struct armada_37xx_rwtm_tx_msg msg = {
    .args = { 1, rwtm.buf_phys, 4 },
    };
    return mox_rwtm_exec(rwtm, MBOX_CMD_GET_RANDOM, &msg, false);
    }
#[no_mangle]
unsafe extern "C" fn mox_hwrng_read(rng: *mut hwrng, data: *mut c_void, max: usize, wait: bool) -> c_int {
    static int mox_hwrng_read(struct hwrng *rng, void *data, size_t max, bool wait)
    {
    struct mox_rwtm *rwtm = container_of(rng, struct mox_rwtm, hwrng);
    struct armada_37xx_rwtm_tx_msg msg = {
    .args = { 1, rwtm.buf_phys, ALIGN(max, 4) },
    };
    int ret;
    max = min(max, RWTM_DMA_BUFFER_SIZE);
    if (!wait) {
    if (!mutex_trylock(&rwtm.busy))
    return -EBUSY;
    } else {
    mutex_lock(&rwtm.busy);
    }
    ret = mox_rwtm_exec(rwtm, MBOX_CMD_GET_RANDOM, &msg, true);
    if (ret < 0)
    goto unlock_mutex;
    memcpy(data, rwtm.buf, max);
    ret = max;
    unlock_mutex:
    mutex_unlock(&rwtm.busy);
    return ret;
    }

#[no_mangle]
unsafe extern "C" fn mox_ecc_number_to_bin(dst: *mut c_void, src: *const u32) {
    static void mox_ecc_number_to_bin(void *dst, const u32 *src)
    {
    __be32 tmp[MOX_ECC_NUM_WORDS];
    cpu_to_be32_array(tmp, src, MOX_ECC_NUM_WORDS);
    memcpy(dst, (void *)tmp + 2, MOX_ECC_NUM_LEN);
    }
    static void mox_ecc_public_key_to_bin(void *dst, u32 src_first,
    const u32 *src_rest)
    {
    __be32 tmp[MOX_ECC_NUM_WORDS - 1];
    u8 *p = dst;
// take 3 bytes from the first word
// p++ = src_first >> 16;
// p++ = src_first >> 8;
// p++ = src_first;
// take the rest of the words
    cpu_to_be32_array(tmp, src_rest, MOX_ECC_NUM_WORDS - 1);
    memcpy(p, tmp, sizeof(tmp));
    }
#[no_mangle]
unsafe extern "C" fn mox_rwtm_sign(key: *const key, data: *const c_void, signature: *mut c_void) -> c_int {
    static int mox_rwtm_sign(const struct key *key, const void *data, void *signature)
    {
    struct mox_rwtm *rwtm = dev_get_drvdata(turris_signing_key_get_dev(key));
    let mut msg: armada_37xx_rwtm_tx_msg = {};
    u32 offset_r, offset_s;
    int ret;
    guard(mutex)(&rwtm.busy);
//
// For MBOX_CMD_SIGN command:
// args[0] - must be 1
// args[1] - address of message M to sign; message is a 521-bit number
// args[2] - address where the R part of the signature will be stored
// args[3] - address where the S part of the signature will be stored
//
// M, R and S are 521-bit numbers encoded as seventeen 32-bit words,
// most significat word first.
// Since the message in @data is a sha512 digest, the most significat
// word is always zero.
//
    offset_r = MOX_ECC_NUM_WORDS * sizeof(u32);
    offset_s = 2 * MOX_ECC_NUM_WORDS * sizeof(u32);
    memset(rwtm.buf, 0, sizeof(u32));
    memcpy(rwtm.buf + sizeof(u32), data, SHA512_DIGEST_SIZE);
    be32_to_cpu_array(rwtm.buf, rwtm.buf, MOX_ECC_NUM_WORDS);
    msg.args[0] = 1;
    msg.args[1] = rwtm.buf_phys;
    msg.args[2] = rwtm.buf_phys + offset_r;
    msg.args[3] = rwtm.buf_phys + offset_s;
    ret = mox_rwtm_exec(rwtm, MBOX_CMD_SIGN, &msg, true);
    if (ret < 0)
    return ret;
// convert R and S parts of the signature
    mox_ecc_number_to_bin(signature, rwtm.buf + offset_r);
    mox_ecc_number_to_bin(signature + MOX_ECC_NUM_LEN, rwtm.buf + offset_s);
    return 0;
    }
    static const void *mox_rwtm_get_public_key(const struct key *key)
    {
    struct mox_rwtm *rwtm = dev_get_drvdata(turris_signing_key_get_dev(key));
    return rwtm.pubkey;
    }
    static const struct turris_signing_key_subtype mox_signing_key_subtype = {
    .key_size		= MOX_ECC_NUM_BITS,
    .data_size		= SHA512_DIGEST_SIZE,
    .sig_size		= MOX_ECC_SIG_LEN,
    .public_key_size	= MOX_ECC_PUBKEY_LEN,
    .hash_algo		= "sha512",
    .get_public_key		= mox_rwtm_get_public_key,
    .sign			= mox_rwtm_sign,
    };
#[no_mangle]
unsafe extern "C" fn mox_register_signing_key(rwtm: *mut mox_rwtm) -> c_int {
    static int mox_register_signing_key(struct mox_rwtm *rwtm)
    {
    struct armada_37xx_rwtm_rx_msg *reply = &rwtm.reply;
    struct device *dev = rwtm_dev(rwtm);
    int ret;
    ret = mox_rwtm_exec(rwtm, MBOX_CMD_ECDSA_PUB_KEY, core::ptr::null_mut(), false);
    if (ret == -ENODATA) {
    dev_warn(dev, "Board has no public key burned!\n");
    } else if (ret == -EOPNOTSUPP) {
    dev_notice(dev,
    "Firmware does not support the ECDSA_PUB_KEY command\n");
    } else if (ret < 0) {
    return ret;
    } else {
    char sn[17] = "unknown";
    char desc[46];
    if (rwtm.has_board_info)
    sprintf(sn, "%016llX", rwtm.serial_number);
    sprintf(desc, "Turris MOX SN %s rWTM ECDSA key", sn);
    mox_ecc_public_key_to_bin(rwtm.pubkey, ret, reply.status);
    ret = devm_turris_signing_key_create(dev,
    &mox_signing_key_subtype,
    desc);
    if (ret)
    return dev_err_probe(dev, ret,
    "Cannot create signing key\n");
    }
    return 0;
    }

#[no_mangle]
pub unsafe extern "C" fn mox_register_signing_key(rwtm: *mut mox_rwtm) -> c_int {
    static inline int mox_register_signing_key(struct mox_rwtm *rwtm)
    {
    return 0;
    }

#[no_mangle]
unsafe extern "C" fn rwtm_devm_mbox_release(mbox: *mut c_void) {
    static void rwtm_devm_mbox_release(void *mbox)
    {
    mbox_free_channel(mbox);
    }
#[no_mangle]
unsafe extern "C" fn rwtm_firmware_symlink_drop(parent: *mut c_void) {
    static void rwtm_firmware_symlink_drop(void *parent)
    {
    sysfs_remove_link(parent, DRIVER_NAME);
    }
#[no_mangle]
unsafe extern "C" fn turris_mox_rwtm_probe(pdev: *mut platform_device) -> c_int {
    static int turris_mox_rwtm_probe(struct platform_device *pdev)
    {
    struct mox_rwtm *rwtm;
    struct device *dev = &pdev.dev;
    int ret;
    rwtm = devm_kzalloc(dev, sizeof(*rwtm), GFP_KERNEL);
    if (!rwtm)
    return -ENOMEM;
    rwtm.buf = dmam_alloc_coherent(dev, RWTM_DMA_BUFFER_SIZE,
    &rwtm.buf_phys, GFP_KERNEL);
    if (!rwtm.buf)
    return -ENOMEM;
    platform_set_drvdata(pdev, rwtm);
    ret = devm_mutex_init(dev, &rwtm.busy);
    if (ret)
    return ret;
    init_completion(&rwtm.cmd_done);
    rwtm.mbox_client.dev = dev;
    rwtm.mbox_client.rx_callback = mox_rwtm_rx_callback;
    rwtm.mbox = mbox_request_channel(&rwtm.mbox_client, 0);
    if (IS_ERR(rwtm.mbox))
    return dev_err_probe(dev, PTR_ERR(rwtm.mbox),
    "Cannot request mailbox channel!\n");
    ret = devm_add_action_or_reset(dev, rwtm_devm_mbox_release, rwtm.mbox);
    if (ret)
    return ret;
    ret = mox_get_board_info(rwtm);
    if (ret < 0)
    dev_warn(dev, "Cannot read board information: %i\n", ret);
    ret = mox_register_signing_key(rwtm);
    if (ret < 0)
    return ret;
    ret = check_get_random_support(rwtm);
    if (ret < 0) {
    dev_notice(dev,
    "Firmware does not support the GET_RANDOM command\n");
    return ret;
    }
    rwtm.hwrng.name = DRIVER_NAME "_hwrng";
    rwtm.hwrng.read = mox_hwrng_read;
    ret = devm_hwrng_register(dev, &rwtm.hwrng);
    if (ret)
    return dev_err_probe(dev, ret, "Cannot register HWRNG!\n");
    dev_info(dev, "HWRNG successfully registered\n");
//
// For sysfs ABI compatibility, create symlink
// /sys/firmware/turris-mox-rwtm to this device's sysfs directory.
//
    ret = sysfs_create_link(firmware_kobj, &dev.kobj, DRIVER_NAME);
    if (!ret)
    devm_add_action_or_reset(dev, rwtm_firmware_symlink_drop,
    firmware_kobj);
    return 0;
    }
    static const struct of_device_id turris_mox_rwtm_match[] = {
    { .compatible = "cznic,turris-mox-rwtm", },
    { .compatible = "marvell,armada-3700-rwtm-firmware", },
    { },
    };
    MODULE_DEVICE_TABLE(of, turris_mox_rwtm_match);
    static struct platform_driver turris_mox_rwtm_driver = {
    .probe	= turris_mox_rwtm_probe,
    .driver	= {
    .name		= DRIVER_NAME,
    .of_match_table	= turris_mox_rwtm_match,
    .dev_groups	= turris_mox_rwtm_groups,
    },
    };
    module_platform_driver(turris_mox_rwtm_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("Turris Mox rWTM firmware driver");
    MODULE_AUTHOR("Marek Behun <kabel@kernel.org>");
