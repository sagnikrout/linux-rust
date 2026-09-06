//! Automatically rewritten from C to Rust
//! Source: drivers/nvmem/imx-ocotp-scu.c
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
// i.MX8 OCOTP fusebox driver
//
// Copyright 2019 NXP
//
// Peng Fan <peng.fan@nxp.com>
//

pub const IMX_SIP_OTP_WRITE: c_uint = 0xc200000B;
    enum ocotp_devtype {
    IMX8QXP,
    IMX8QM,
    };

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocotp_region {
    pub start: u32,
    pub end: u32,
    pub flag: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocotp_devtype_data {
    pub devtype: c_int,
    pub nregs: c_int,
    pub num_region: u32,
    pub region: [ocotp_region; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ocotp_priv {
    pub dev: *mut device,
    pub data: *const ocotp_devtype_data,
    pub nvmem_ipc: *mut imx_sc_ipc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct imx_sc_msg_misc_fuse_read {
    pub hdr: imx_sc_rpc_msg,
    pub word: u32,
    pub __packed: },
    pub DEFINE_MUTEX(scu_ocotp_mutex): static,
    static struct ocotp_devtype_data imx8qxp_data = {
    .devtype = IMX8QXP,
    .nregs = 800,
    .num_region = 3,
    .region = {
    {0x10, 0x10f, ECC_REGION},
    {0x110, 0x21F, HOLE_REGION},
    {0x220, 0x31F, ECC_REGION},
    },
}

    static struct ocotp_devtype_data imx8qm_data = {
    .devtype = IMX8QM,
    .nregs = 800,
    .num_region = 2,
    .region = {
    {0x10, 0x10f, ECC_REGION},
    {0x1a0, 0x1ff, ECC_REGION},
    },
    };
#[no_mangle]
unsafe extern "C" fn in_hole(context: *mut c_void, index: u32) -> bool {
    static bool in_hole(void *context, u32 index)
    {
    struct ocotp_priv *priv = context;
    const struct ocotp_devtype_data *data = priv.data;
    int i;
    for (i = 0; i < data.num_region; i++) {
    if (data.region[i].flag & HOLE_REGION) {
    if ((index >= data.region[i].start) &&
    (index <= data.region[i].end))
    return true;
    }
    }
    return false;
    }
#[no_mangle]
unsafe extern "C" fn in_ecc(context: *mut c_void, index: u32) -> bool {
    static bool in_ecc(void *context, u32 index)
    {
    struct ocotp_priv *priv = context;
    const struct ocotp_devtype_data *data = priv.data;
    int i;
    for (i = 0; i < data.num_region; i++) {
    if (data.region[i].flag & ECC_REGION) {
    if ((index >= data.region[i].start) &&
    (index <= data.region[i].end))
    return true;
    }
    }
    return false;
    }
    static int imx_sc_misc_otp_fuse_read(struct imx_sc_ipc *ipc, u32 word,
    u32 *val)
    {
    struct imx_sc_msg_misc_fuse_read msg;
    struct imx_sc_rpc_msg *hdr = &msg.hdr;
    int ret;
    hdr.ver = IMX_SC_RPC_VERSION;
    hdr.svc = IMX_SC_RPC_SVC_MISC;
    hdr.func = IMX_SC_MISC_FUNC_OTP_FUSE_READ;
    hdr.size = 2;
    msg.word = word;
    ret = imx_scu_call_rpc(ipc, &msg, true);
    if (ret)
    return ret;
// val = msg.word;
    return 0;
    }
    static int imx_scu_ocotp_read(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct ocotp_priv *priv = context;
    u32 count, index, num_bytes;
    u32 *buf;
    void *p;
    int i, ret;
    index = offset;
    num_bytes = round_up(bytes, 4);
    count = num_bytes >> 2;
    if (count > (priv.data.nregs - index))
    count = priv.data.nregs - index;
    p = kzalloc(num_bytes, GFP_KERNEL);
    if (!p)
    return -ENOMEM;
    mutex_lock(&scu_ocotp_mutex);
    buf = p;
    for (i = index; i < (index + count); i++) {
    if (in_hole(context, i)) {
// buf++ = 0;
    continue;
    }
    ret = imx_sc_misc_otp_fuse_read(priv.nvmem_ipc, i, buf);
    if (ret) {
    mutex_unlock(&scu_ocotp_mutex);
    kfree(p);
    return ret;
    }
    buf++;
    }
    memcpy(val, (u8 *)p, bytes);
    mutex_unlock(&scu_ocotp_mutex);
    kfree(p);
    return 0;
    }
    static int imx_scu_ocotp_write(void *context, unsigned int offset,
    void *val, size_t bytes)
    {
    struct ocotp_priv *priv = context;
    struct arm_smccc_res res;
    u32 *buf = val;
    u32 tmp;
    u32 index;
    int ret;
// allow only writing one complete OTP word at a time
    if (bytes != 4)
    return -EINVAL;
    index = offset;
    if (in_hole(context, index))
    return -EINVAL;
    if (in_ecc(context, index)) {
    pr_warn("ECC region, only program once\n");
    mutex_lock(&scu_ocotp_mutex);
    ret = imx_sc_misc_otp_fuse_read(priv.nvmem_ipc, index, &tmp);
    mutex_unlock(&scu_ocotp_mutex);
    if (ret)
    return ret;
    if (tmp) {
    pr_warn("ECC region, already has value: %x\n", tmp);
    return -EIO;
    }
    }
    mutex_lock(&scu_ocotp_mutex);
    arm_smccc_smc(IMX_SIP_OTP_WRITE, index, *buf, 0, 0, 0, 0, 0, &res);
    mutex_unlock(&scu_ocotp_mutex);
    return res.a0;
    }
    static struct nvmem_config imx_scu_ocotp_nvmem_config = {
    .name = "imx-scu-ocotp",
    .add_legacy_fixed_of_cells = true,
    .read_only = false,
    .word_size = 4,
    .stride = 1,
    .owner = THIS_MODULE,
    .reg_read = imx_scu_ocotp_read,
    .reg_write = imx_scu_ocotp_write,
    };
    static const struct of_device_id imx_scu_ocotp_dt_ids[] = {
    { .compatible = "fsl,imx8qxp-scu-ocotp", (void *)&imx8qxp_data },
    { .compatible = "fsl,imx8qm-scu-ocotp", (void *)&imx8qm_data },
    { },
    };
    MODULE_DEVICE_TABLE(of, imx_scu_ocotp_dt_ids);
#[no_mangle]
unsafe extern "C" fn imx_scu_ocotp_probe(pdev: *mut platform_device) -> c_int {
    static int imx_scu_ocotp_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct ocotp_priv *priv;
    struct nvmem_device *nvmem;
    int ret;
    priv = devm_kzalloc(dev, sizeof(*priv), GFP_KERNEL);
    if (!priv)
    return -ENOMEM;
    ret = imx_scu_get_handle(&priv.nvmem_ipc);
    if (ret)
    return ret;
    priv.data = of_device_get_match_data(dev);
    priv.dev = dev;
    imx_scu_ocotp_nvmem_config.size = 4 * priv.data.nregs;
    imx_scu_ocotp_nvmem_config.dev = dev;
    imx_scu_ocotp_nvmem_config.priv = priv;
    nvmem = devm_nvmem_register(dev, &imx_scu_ocotp_nvmem_config);
    return PTR_ERR_OR_ZERO(nvmem);
    }
    static struct platform_driver imx_scu_ocotp_driver = {
    .probe	= imx_scu_ocotp_probe,
    .driver = {
    .name	= "imx_scu_ocotp",
    .of_match_table = imx_scu_ocotp_dt_ids,
    },
    };
    module_platform_driver(imx_scu_ocotp_driver);
    MODULE_AUTHOR("Peng Fan <peng.fan@nxp.com>");
    MODULE_DESCRIPTION("i.MX8 SCU OCOTP fuse box driver");
    MODULE_LICENSE("GPL v2");
