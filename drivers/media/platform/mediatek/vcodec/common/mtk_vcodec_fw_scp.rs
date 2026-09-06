//! Automatically rewritten from C to Rust
//! Source: drivers/media/platform/mediatek/vcodec/common/mtk_vcodec_fw_scp.c
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

#[no_mangle]
unsafe extern "C" fn mtk_vcodec_scp_load_firmware(fw: *mut mtk_vcodec_fw) -> c_int {
    static int mtk_vcodec_scp_load_firmware(struct mtk_vcodec_fw *fw)
    {
    return rproc_boot(scp_get_rproc(fw.scp));
    }
#[no_mangle]
unsafe extern "C" fn mtk_vcodec_scp_get_vdec_capa(fw: *mut mtk_vcodec_fw) -> c_uint {
    static unsigned int mtk_vcodec_scp_get_vdec_capa(struct mtk_vcodec_fw *fw)
    {
    return scp_get_vdec_hw_capa(fw.scp);
    }
#[no_mangle]
unsafe extern "C" fn mtk_vcodec_scp_get_venc_capa(fw: *mut mtk_vcodec_fw) -> c_uint {
    static unsigned int mtk_vcodec_scp_get_venc_capa(struct mtk_vcodec_fw *fw)
    {
    return scp_get_venc_hw_capa(fw.scp);
    }
    static void *mtk_vcodec_vpu_scp_dm_addr(struct mtk_vcodec_fw *fw,
    u32 dtcm_dmem_addr)
    {
    return scp_mapping_dm_addr(fw.scp, dtcm_dmem_addr);
    }
    static int mtk_vcodec_scp_set_ipi_register(struct mtk_vcodec_fw *fw, int id,
    mtk_vcodec_ipi_handler handler,
    const char *name, void *priv)
    {
    return scp_ipi_register(fw.scp, id, handler, priv);
    }
    static int mtk_vcodec_scp_ipi_send(struct mtk_vcodec_fw *fw, int id, void *buf,
    unsigned int len, unsigned int wait)
    {
    return scp_ipi_send(fw.scp, id, buf, len, wait);
    }
#[no_mangle]
unsafe extern "C" fn mtk_vcodec_scp_release(fw: *mut mtk_vcodec_fw) {
    static void mtk_vcodec_scp_release(struct mtk_vcodec_fw *fw)
    {
    scp_put(fw.scp);
    }
    static const struct mtk_vcodec_fw_ops mtk_vcodec_rproc_msg = {
    .load_firmware = mtk_vcodec_scp_load_firmware,
    .get_vdec_capa = mtk_vcodec_scp_get_vdec_capa,
    .get_venc_capa = mtk_vcodec_scp_get_venc_capa,
    .map_dm_addr = mtk_vcodec_vpu_scp_dm_addr,
    .ipi_register = mtk_vcodec_scp_set_ipi_register,
    .ipi_send = mtk_vcodec_scp_ipi_send,
    .release = mtk_vcodec_scp_release,
    };
    struct mtk_vcodec_fw *mtk_vcodec_fw_scp_init(void *priv, enum mtk_vcodec_fw_use fw_use)
    {
    struct mtk_vcodec_fw *fw;
    struct platform_device *plat_dev;
    struct mtk_scp *scp;
    if (fw_use == ENCODER) {
    struct mtk_vcodec_enc_dev *enc_dev = priv;
    plat_dev = enc_dev.plat_dev;
    } else if (fw_use == DECODER) {
    struct mtk_vcodec_dec_dev *dec_dev = priv;
    plat_dev = dec_dev.plat_dev;
    } else {
    pr_err("Invalid fw_use %d (use a reasonable fw id here)\n", fw_use);
    return ERR_PTR(-EINVAL);
    }
    scp = scp_get(plat_dev);
    if (!scp) {
    dev_err(&plat_dev.dev, "could not get vdec scp handle");
    return ERR_PTR(-EPROBE_DEFER);
    }
    fw = devm_kzalloc(&plat_dev.dev, sizeof(*fw), GFP_KERNEL);
    if (!fw) {
    scp_put(scp);
    return ERR_PTR(-ENOMEM);
    }
    fw.type = SCP;
    fw.ops = &mtk_vcodec_rproc_msg;
    fw.scp = scp;
    return fw;
    }
