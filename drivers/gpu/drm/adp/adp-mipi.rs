//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/adp/adp-mipi.c
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

pub const DSI_GEN_HDR: c_uint = 0x6c;
pub const DSI_GEN_PLD_DATA: c_uint = 0x70;
pub const DSI_CMD_PKT_STATUS: c_uint = 0x74;

pub const CMD_PKT_STATUS_TIMEOUT_US: c_int = 20000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct adp_mipi_drv_private {
    pub dsi: mipi_dsi_host,
    pub bridge: drm_bridge,
    pub next_bridge: *mut drm_bridge,
    pub mipi: *mut void __iomem,
}

#[no_mangle]
unsafe extern "C" fn adp_dsi_gen_pkt_hdr_write(adp: *mut adp_mipi_drv_private, hdr_val: u32) -> c_int {
    static int adp_dsi_gen_pkt_hdr_write(struct adp_mipi_drv_private *adp, u32 hdr_val)
    {
    int ret;
    u32 val, mask;
    ret = readl_poll_timeout(adp.mipi + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_CMD_FULL), 1000,
    CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(adp.dsi.dev, "failed to get available command FIFO\n");
    return ret;
    }
    writel(hdr_val, adp.mipi + DSI_GEN_HDR);
    mask = GEN_CMD_EMPTY | GEN_PLD_W_EMPTY;
    ret = readl_poll_timeout(adp.mipi + DSI_CMD_PKT_STATUS,
    val, (val & mask) == mask,
    1000, CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(adp.dsi.dev, "failed to write command FIFO\n");
    return ret;
    }
    return 0;
    }
    static int adp_dsi_write(struct adp_mipi_drv_private *adp,
    const struct mipi_dsi_packet *packet)
    {
    const u8 *tx_buf = packet.payload;
    let mut len: c_int = packet.payload_length, pld_data_bytes = sizeof(u32), ret;
    __le32 word;
    u32 val;
    while (len) {
    if (len < pld_data_bytes) {
    word = 0;
    memcpy(&word, tx_buf, len);
    writel(le32_to_cpu(word), adp.mipi + DSI_GEN_PLD_DATA);
    len = 0;
    } else {
    memcpy(&word, tx_buf, pld_data_bytes);
    writel(le32_to_cpu(word), adp.mipi + DSI_GEN_PLD_DATA);
    tx_buf += pld_data_bytes;
    len -= pld_data_bytes;
    }
    ret = readl_poll_timeout(adp.mipi + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_PLD_W_FULL), 1000,
    CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(adp.dsi.dev,
    "failed to get available write payload FIFO\n");
    return ret;
    }
    }
    word = 0;
    memcpy(&word, packet.header, sizeof(packet.header));
    return adp_dsi_gen_pkt_hdr_write(adp, le32_to_cpu(word));
    }
    static int adp_dsi_read(struct adp_mipi_drv_private *adp,
    const struct mipi_dsi_msg *msg)
    {
    int i, j, ret, len = msg.rx_len;
    u8 *buf = msg.rx_buf;
    u32 val;
// Wait end of the read operation
    ret = readl_poll_timeout(adp.mipi + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_RD_CMD_BUSY),
    1000, CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(adp.dsi.dev, "Timeout during read operation\n");
    return ret;
    }
    for (i = 0; i < len; i += 4) {
// Read fifo must not be empty before all bytes are read
    ret = readl_poll_timeout(adp.mipi + DSI_CMD_PKT_STATUS,
    val, !(val & GEN_PLD_R_EMPTY),
    1000, CMD_PKT_STATUS_TIMEOUT_US);
    if (ret) {
    dev_err(adp.dsi.dev, "Read payload FIFO is empty\n");
    return ret;
    }
    val = readl(adp.mipi + DSI_GEN_PLD_DATA);
    for (j = 0; j < 4 && j + i < len; j++)
    buf[i + j] = val >> (8 * j);
    }
    return ret;
    }
    static ssize_t adp_dsi_host_transfer(struct mipi_dsi_host *host,
    const struct mipi_dsi_msg *msg)
    {
    struct adp_mipi_drv_private *adp = mipi_to_adp(host);
    struct mipi_dsi_packet packet;
    int ret, nb_bytes;
    ret = mipi_dsi_create_packet(&packet, msg);
    if (ret) {
    dev_err(adp.dsi.dev, "failed to create packet: %d\n", ret);
    return ret;
    }
    ret = adp_dsi_write(adp, &packet);
    if (ret)
    return ret;
    if (msg.rx_buf && msg.rx_len) {
    ret = adp_dsi_read(adp, msg);
    if (ret)
    return ret;
    nb_bytes = msg.rx_len;
    } else {
    nb_bytes = packet.size;
    }
    return nb_bytes;
    }
#[no_mangle]
unsafe extern "C" fn adp_dsi_bind(dev: *mut device, master: *mut device, data: *mut c_void) -> c_int {
    static int adp_dsi_bind(struct device *dev, struct device *master, void *data)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn adp_dsi_unbind(dev: *mut device, master: *mut device, data: *mut c_void) {
    static void adp_dsi_unbind(struct device *dev, struct device *master, void *data)
    {
    }
    static const struct component_ops adp_dsi_component_ops = {
    .bind	= adp_dsi_bind,
    .unbind	= adp_dsi_unbind,
    };
    static int adp_dsi_host_attach(struct mipi_dsi_host *host,
    struct mipi_dsi_device *dev)
    {
    struct adp_mipi_drv_private *adp = mipi_to_adp(host);
    struct drm_bridge *next;
    int ret;
    next = devm_drm_of_get_bridge(adp.dsi.dev, adp.dsi.dev.of_node, 1, 0);
    if (IS_ERR(next))
    return PTR_ERR(next);
    adp.next_bridge = next;
    drm_bridge_add(&adp.bridge);
    ret = component_add(host.dev, &adp_dsi_component_ops);
    if (ret) {
    pr_err("failed to add dsi_host component: %d\n", ret);
    drm_bridge_remove(&adp.bridge);
    return ret;
    }
    return 0;
    }
    static int adp_dsi_host_detach(struct mipi_dsi_host *host,
    struct mipi_dsi_device *dev)
    {
    struct adp_mipi_drv_private *adp = mipi_to_adp(host);
    component_del(host.dev, &adp_dsi_component_ops);
    drm_bridge_remove(&adp.bridge);
    return 0;
    }
    static const struct mipi_dsi_host_ops adp_dsi_host_ops = {
    .transfer = adp_dsi_host_transfer,
    .attach = adp_dsi_host_attach,
    .detach = adp_dsi_host_detach,
    };
    static int adp_dsi_bridge_attach(struct drm_bridge *bridge,
    struct drm_encoder *encoder,
    enum drm_bridge_attach_flags flags)
    {
    struct adp_mipi_drv_private *adp =
    container_of(bridge, struct adp_mipi_drv_private, bridge);
    return drm_bridge_attach(encoder, adp.next_bridge, bridge, flags);
    }
    static const struct drm_bridge_funcs adp_dsi_bridge_funcs = {
    .atomic_create_state = drm_atomic_helper_bridge_create_state,
    .atomic_destroy_state = drm_atomic_helper_bridge_destroy_state,
    .atomic_duplicate_state = drm_atomic_helper_bridge_duplicate_state,
    .attach	= adp_dsi_bridge_attach,
    };
#[no_mangle]
unsafe extern "C" fn adp_mipi_probe(pdev: *mut platform_device) -> c_int {
    static int adp_mipi_probe(struct platform_device *pdev)
    {
    struct adp_mipi_drv_private *adp;
    adp = devm_drm_bridge_alloc(&pdev.dev, struct adp_mipi_drv_private,
    bridge, &adp_dsi_bridge_funcs);
    if (IS_ERR(adp))
    return PTR_ERR(adp);
    adp.mipi = devm_platform_ioremap_resource(pdev, 0);
    if (IS_ERR(adp.mipi)) {
    dev_err(&pdev.dev, "failed to map mipi mmio");
    return PTR_ERR(adp.mipi);
    }
    adp.dsi.dev = &pdev.dev;
    adp.dsi.ops = &adp_dsi_host_ops;
    adp.bridge.of_node = pdev.dev.of_node;
    adp.bridge.type = DRM_MODE_CONNECTOR_DSI;
    dev_set_drvdata(&pdev.dev, adp);
    return mipi_dsi_host_register(&adp.dsi);
    }
#[no_mangle]
unsafe extern "C" fn adp_mipi_remove(pdev: *mut platform_device) {
    static void adp_mipi_remove(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct adp_mipi_drv_private *adp = dev_get_drvdata(dev);
    mipi_dsi_host_unregister(&adp.dsi);
    }
    static const struct of_device_id adp_mipi_of_match[] = {
    { .compatible = "apple,h7-display-pipe-mipi", },
    { },
    };
    MODULE_DEVICE_TABLE(of, adp_mipi_of_match);
    static struct platform_driver adp_mipi_platform_driver = {
    .driver = {
    .name = "adp-mipi",
    .of_match_table = adp_mipi_of_match,
    },
    .probe = adp_mipi_probe,
    .remove = adp_mipi_remove,
    };
    module_platform_driver(adp_mipi_platform_driver);
    MODULE_DESCRIPTION("Apple Display Pipe MIPI driver");
    MODULE_LICENSE("GPL");
