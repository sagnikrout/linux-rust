//! Automatically rewritten from C to Rust
//! Source: drivers/net/wireless/mediatek/mt76/mt792x_usb.c
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
// Copyright (C) 2023 MediaTek Inc.
//
// Author: Lorenzo Bianconi <lorenzo@kernel.org>
//

pub const MT792X_USB_TX_TIMEOUT_LIMIT: c_int = 50000;
pub const MT792X_USB_UDMA_IDLE_TIMEOUT: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn mt792xu_read32(dev: *mut mt76_dev, addr: u32, buf: *mut c_void) -> c_int {
    static int mt792xu_read32(struct mt76_dev *dev, u32 addr, void *buf)
    {
    return __mt76u_vendor_request(dev, MT_VEND_READ_EXT,
    USB_DIR_IN | MT_USB_TYPE_VENDOR,
    (u16)(addr >> 16), (u16)addr,
    buf, sizeof(__le32));
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_reset_work(work: *mut work_struct) {
    static void mt792xu_reset_work(struct work_struct *work)
    {
    struct mt792x_dev *dev =
    container_of(work, struct mt792x_dev, usb_reset_work);
    struct usb_interface *usb_intf = to_usb_interface(dev.mt76.dev);
    if (usb_intf && usb_get_intfdata(usb_intf) == dev)
    usb_queue_reset_device(usb_intf);
    atomic_set(&dev.usb_reset_pending, 0);
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_queue_usb_reset(dev: *mut mt792x_dev, err: c_int) {
    static void mt792xu_queue_usb_reset(struct mt792x_dev *dev, int err)
    {
    if (!atomic_xchg(&dev.usb_reset_pending, 1)) {
    dev_warn(dev.mt76.dev,
    "USB transport access failed (%d), queueing device reset\n",
    err);
    schedule_work(&dev.usb_reset_work);
    }
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_bus_hung_rr(mdev: *mut mt76_dev, offset: u32) -> u32 {
    static u32 mt792xu_bus_hung_rr(struct mt76_dev *mdev, u32 offset)
    {
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_bus_hung_wr(mdev: *mut mt76_dev, offset: u32, val: u32) {
    static void mt792xu_bus_hung_wr(struct mt76_dev *mdev, u32 offset, u32 val)
    {
    }
    static u32 mt792xu_bus_hung_rmw(struct mt76_dev *mdev, u32 offset,
    u32 mask, u32 val)
    {
    return 0;
    }
    static void mt792xu_bus_hung_write_copy(struct mt76_dev *mdev, u32 offset,
    const void *data, int len)
    {
    }
    static void mt792xu_bus_hung_read_copy(struct mt76_dev *mdev, u32 offset,
    void *data, int len)
    {
    memset(data, 0, len);
    }
    static const struct mt76_bus_ops mt792xu_bus_hung_ops = {
    .rr = mt792xu_bus_hung_rr,
    .wr = mt792xu_bus_hung_wr,
    .rmw = mt792xu_bus_hung_rmw,
    .write_copy = mt792xu_bus_hung_write_copy,
    .read_copy = mt792xu_bus_hung_read_copy,
    .type = MT76_BUS_USB,
    };
#[no_mangle]
unsafe extern "C" fn mt792xu_set_bus_hung(dev: *mut mt792x_dev) {
    static void mt792xu_set_bus_hung(struct mt792x_dev *dev)
    {
    atomic_set(&dev.mt76.bus_hung, true);
    if (READ_ONCE(dev.mt76.bus) == &mt792xu_bus_hung_ops)
    return;
    WRITE_ONCE(dev.mt76.bus, &mt792xu_bus_hung_ops);
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_ctrl_timeout(mdev: *mut mt76_dev, err: c_int) {
    static void mt792xu_ctrl_timeout(struct mt76_dev *mdev, int err)
    {
    struct mt792x_dev *dev = container_of(mdev, struct mt792x_dev, mt76);
    mt792xu_set_bus_hung(dev);
    mt792xu_queue_usb_reset(dev, err);
    }
#[no_mangle]
pub unsafe extern "C" fn mt792xu_reset_work_init(dev: *mut mt792x_dev) {
    void mt792xu_reset_work_init(struct mt792x_dev *dev)
    {
    INIT_WORK(&dev.usb_reset_work, mt792xu_reset_work);
    atomic_set(&dev.usb_reset_pending, 0);
    dev.mt76.usb.ctrl_timeout = mt792xu_ctrl_timeout;
    }
    EXPORT_SYMBOL_GPL(mt792xu_reset_work_init);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_reset_work_cleanup(dev: *mut mt792x_dev) {
    void mt792xu_reset_work_cleanup(struct mt792x_dev *dev)
    {
    cancel_work_sync(&dev.usb_reset_work);
    atomic_set(&dev.usb_reset_pending, 0);
    }
    EXPORT_SYMBOL_GPL(mt792xu_reset_work_cleanup);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_check_bus(dev: *mut mt792x_dev) -> c_int {
    int mt792xu_check_bus(struct mt792x_dev *dev)
    {
    int ret;
    mutex_lock(&dev.mt76.usb.usb_ctrl_mtx);
    ret = mt792xu_read32(&dev.mt76, MT_HW_CHIPID, dev.mt76.usb.data);
    mutex_unlock(&dev.mt76.usb.usb_ctrl_mtx);
    if (ret == sizeof(__le32))
    return 0;
    return ret < 0 ? ret : -EIO;
    }
    EXPORT_SYMBOL_GPL(mt792xu_check_bus);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_reset_on_bus_error(dev: *mut mt792x_dev) -> c_int {
    int mt792xu_reset_on_bus_error(struct mt792x_dev *dev)
    {
    int err;
// Once hung, the no-op bus ops stay installed until the queued USB
// reset re-probes the device. Do not clear bus_hung here, or the caller
// would run a full reset over dropped register I/O and report success.
//
    if (atomic_read(&dev.mt76.bus_hung))
    return -EIO;
    err = mt792xu_check_bus(dev);
    if (err) {
    mt792xu_set_bus_hung(dev);
    mt792xu_queue_usb_reset(dev, err);
    return err;
    }
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792xu_reset_on_bus_error);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_rr(dev: *mut mt76_dev, addr: u32) -> u32 {
    u32 mt792xu_rr(struct mt76_dev *dev, u32 addr)
    {
    u32 ret;
    mutex_lock(&dev.usb.usb_ctrl_mtx);
    ret = ___mt76u_rr(dev, MT_VEND_READ_EXT,
    USB_DIR_IN | MT_USB_TYPE_VENDOR, addr);
    mutex_unlock(&dev.usb.usb_ctrl_mtx);
    return ret;
    }
    EXPORT_SYMBOL_GPL(mt792xu_rr);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_wr(dev: *mut mt76_dev, addr: u32, val: u32) {
    void mt792xu_wr(struct mt76_dev *dev, u32 addr, u32 val)
    {
    mutex_lock(&dev.usb.usb_ctrl_mtx);
    ___mt76u_wr(dev, MT_VEND_WRITE_EXT,
    USB_DIR_OUT | MT_USB_TYPE_VENDOR, addr, val);
    mutex_unlock(&dev.usb.usb_ctrl_mtx);
    }
    EXPORT_SYMBOL_GPL(mt792xu_wr);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_rmw(dev: *mut mt76_dev, addr: u32, mask: u32, val: u32) -> u32 {
    u32 mt792xu_rmw(struct mt76_dev *dev, u32 addr, u32 mask, u32 val)
    {
    mutex_lock(&dev.usb.usb_ctrl_mtx);
    val |= ___mt76u_rr(dev, MT_VEND_READ_EXT,
    USB_DIR_IN | MT_USB_TYPE_VENDOR, addr) & ~mask;
    ___mt76u_wr(dev, MT_VEND_WRITE_EXT,
    USB_DIR_OUT | MT_USB_TYPE_VENDOR, addr, val);
    mutex_unlock(&dev.usb.usb_ctrl_mtx);
    return val;
    }
    EXPORT_SYMBOL_GPL(mt792xu_rmw);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_copy(dev: *mut mt76_dev, offset: u32, data: *const c_void, len: c_int) {
    void mt792xu_copy(struct mt76_dev *dev, u32 offset, const void *data, int len)
    {
    struct mt76_usb *usb = &dev.usb;
    int ret, i = 0, batch_len;
    const u8 *val = data;
    len = round_up(len, 4);
    mutex_lock(&usb.usb_ctrl_mtx);
    while (i < len) {
    batch_len = min_t(int, usb.data_len, len - i);
    memcpy(usb.data, val + i, batch_len);
    ret = __mt76u_vendor_request(dev, MT_VEND_WRITE_EXT,
    USB_DIR_OUT | MT_USB_TYPE_VENDOR,
    (offset + i) >> 16, offset + i,
    usb.data, batch_len);
    if (ret < 0)
    break;
    i += batch_len;
    }
    mutex_unlock(&usb.usb_ctrl_mtx);
    }
    EXPORT_SYMBOL_GPL(mt792xu_copy);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_mcu_power_on(dev: *mut mt792x_dev) -> c_int {
    int mt792xu_mcu_power_on(struct mt792x_dev *dev)
    {
    int ret;
    ret = mt76u_vendor_request(&dev.mt76, MT_VEND_POWER_ON,
    USB_DIR_OUT | MT_USB_TYPE_VENDOR,
    0x0, 0x1, core::ptr::null_mut(), 0);
    if (ret)
    return ret;
    if (!mt76_poll_msec(dev, MT_CONN_ON_MISC, MT_TOP_MISC2_FW_PWR_ON,
    MT_TOP_MISC2_FW_PWR_ON, 500)) {
    dev_err(dev.mt76.dev, "Timeout for power on\n");
    ret = -EIO;
    }
    return ret;
    }
    EXPORT_SYMBOL_GPL(mt792xu_mcu_power_on);
#[no_mangle]
unsafe extern "C" fn mt792xu_cleanup(dev: *mut mt792x_dev) {
    static void mt792xu_cleanup(struct mt792x_dev *dev)
    {
    clear_bit(MT76_STATE_INITIALIZED, &dev.mphy.state);
    skb_queue_purge(&dev.mt76.mcu.res_q);
    mt76u_queues_deinit(&dev.mt76);
    mt792xu_wfsys_reset(dev);
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_uhw_rr(dev: *mut mt76_dev, addr: u32) -> u32 {
    static u32 mt792xu_uhw_rr(struct mt76_dev *dev, u32 addr)
    {
    u32 ret;
    mutex_lock(&dev.usb.usb_ctrl_mtx);
    ret = ___mt76u_rr(dev, MT_VEND_DEV_MODE,
    USB_DIR_IN | MT_USB_TYPE_UHW_VENDOR, addr);
    mutex_unlock(&dev.usb.usb_ctrl_mtx);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_uhw_wr(dev: *mut mt76_dev, addr: u32, val: u32) {
    static void mt792xu_uhw_wr(struct mt76_dev *dev, u32 addr, u32 val)
    {
    mutex_lock(&dev.usb.usb_ctrl_mtx);
    ___mt76u_wr(dev, MT_VEND_WRITE,
    USB_DIR_OUT | MT_USB_TYPE_UHW_VENDOR, addr, val);
    mutex_unlock(&dev.usb.usb_ctrl_mtx);
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_dma_prefetch(dev: *mut mt792x_dev) {
    static void mt792xu_dma_prefetch(struct mt792x_dev *dev)
    {

    mt76_rmw(dev, MT_UWFDMA0_TX_RING_EXT_CTRL((_idx_)), \
    MT_WPDMA0_MAX_CNT_MASK | MT_WPDMA0_BASE_PTR_MASK, \
    FIELD_PREP(MT_WPDMA0_MAX_CNT_MASK, (_cnt_)) | \
    FIELD_PREP(MT_WPDMA0_BASE_PTR_MASK, (_base_)))
    DMA_PREFETCH_CONF(0, 4, 0x080);
    DMA_PREFETCH_CONF(1, 4, 0x0c0);
    DMA_PREFETCH_CONF(2, 4, 0x100);
    DMA_PREFETCH_CONF(3, 4, 0x140);
    DMA_PREFETCH_CONF(4, 4, 0x180);
    DMA_PREFETCH_CONF(16, 4, 0x280);
    DMA_PREFETCH_CONF(17, 4, 0x2c0);
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_wfdma_init(dev: *mut mt792x_dev) {
    static void mt792xu_wfdma_init(struct mt792x_dev *dev)
    {
    int i;
    mt792xu_dma_prefetch(dev);
    mt76_clear(dev, MT_UWFDMA0_GLO_CFG, MT_WFDMA0_GLO_CFG_OMIT_RX_INFO);
    mt76_set(dev, MT_UWFDMA0_GLO_CFG,
    MT_WFDMA0_GLO_CFG_OMIT_TX_INFO |
    MT_WFDMA0_GLO_CFG_OMIT_RX_INFO_PFET2 |
    MT_WFDMA0_GLO_CFG_FW_DWLD_BYPASS_DMASHDL |
    MT_WFDMA0_GLO_CFG_TX_DMA_EN |
    MT_WFDMA0_GLO_CFG_RX_DMA_EN);
    mt76_rmw(dev, MT_DMASHDL_REFILL, MT_DMASHDL_REFILL_MASK, 0xffe00000);
    mt76_clear(dev, MT_DMASHDL_PAGE, MT_DMASHDL_GROUP_SEQ_ORDER);
    mt76_rmw(dev, MT_DMASHDL_PKT_MAX_SIZE,
    MT_DMASHDL_PKT_MAX_SIZE_PLE | MT_DMASHDL_PKT_MAX_SIZE_PSE,
    FIELD_PREP(MT_DMASHDL_PKT_MAX_SIZE_PLE, 1) |
    FIELD_PREP(MT_DMASHDL_PKT_MAX_SIZE_PSE, 0));
    for (i = 0; i < 5; i++)
    mt76_wr(dev, MT_DMASHDL_GROUP_QUOTA(i),
    FIELD_PREP(MT_DMASHDL_GROUP_QUOTA_MIN, 0x3) |
    FIELD_PREP(MT_DMASHDL_GROUP_QUOTA_MAX, 0xfff));
    for (i = 5; i < 16; i++)
    mt76_wr(dev, MT_DMASHDL_GROUP_QUOTA(i),
    FIELD_PREP(MT_DMASHDL_GROUP_QUOTA_MIN, 0x0) |
    FIELD_PREP(MT_DMASHDL_GROUP_QUOTA_MAX, 0x0));
    mt76_wr(dev, MT_DMASHDL_Q_MAP(0), 0x32013201);
    mt76_wr(dev, MT_DMASHDL_Q_MAP(1), 0x32013201);
    mt76_wr(dev, MT_DMASHDL_Q_MAP(2), 0x55555444);
    mt76_wr(dev, MT_DMASHDL_Q_MAP(3), 0x55555444);
    mt76_wr(dev, MT_DMASHDL_SCHED_SET(0), 0x76540132);
    mt76_wr(dev, MT_DMASHDL_SCHED_SET(1), 0xFEDCBA98);
    mt76_set(dev, MT_WFDMA_DUMMY_CR, MT_WFDMA_NEED_REINIT);
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_dma_rx_evt_ep4(dev: *mut mt792x_dev) -> c_int {
    static int mt792xu_dma_rx_evt_ep4(struct mt792x_dev *dev)
    {
    if (!mt76_poll(dev, MT_UWFDMA0_GLO_CFG,
    MT_WFDMA0_GLO_CFG_RX_DMA_BUSY, 0, 1000))
    return -ETIMEDOUT;
    mt76_clear(dev, MT_UWFDMA0_GLO_CFG, MT_WFDMA0_GLO_CFG_RX_DMA_EN);
    mt76_set(dev, MT_WFDMA_HOST_CONFIG,
    MT_WFDMA_HOST_CONFIG_USB_RXEVT_EP4_EN);
    mt76_set(dev, MT_UWFDMA0_GLO_CFG, MT_WFDMA0_GLO_CFG_RX_DMA_EN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_epctl_rst_opt(dev: *mut mt792x_dev, reset: bool) {
    static void mt792xu_epctl_rst_opt(struct mt792x_dev *dev, bool reset)
    {
    u32 val;
// usb endpoint reset opt
// bits[4,9]: out blk ep 4-9
// bits[20,21]: in blk ep 4-5
// bits[22]: in int ep 6
//
    val = mt792xu_uhw_rr(&dev.mt76, MT_SSUSB_EPCTL_CSR_EP_RST_OPT);
    if (reset)
    val |= GENMASK(9, 4) | GENMASK(22, 20);
    else
    val &= ~(GENMASK(9, 4) | GENMASK(22, 20));
    mt792xu_uhw_wr(&dev.mt76, MT_SSUSB_EPCTL_CSR_EP_RST_OPT, val);
    }
#[no_mangle]
unsafe extern "C" fn mt792xu_wait_udma_idle(dev: *mut mt792x_dev) {
    static void mt792xu_wait_udma_idle(struct mt792x_dev *dev)
    {
    let mut mask: u32 = MT_WL_RX_BUSY | MT_WL_TX_BUSY;
    u32 val;
    mt76_set(dev, MT_UDMA_WLCFG_0, MT_WL_RX_FLUSH);
    if (mt76_poll_msec(dev, MT_UDMA_WLCFG_0, mask, 0,
    MT792X_USB_UDMA_IDLE_TIMEOUT))
    return;
    val = mt76_rr(dev, MT_UDMA_WLCFG_0);
    dev_warn(dev.mt76.dev,
    "UDMA busy before WFSYS reset: WLCFG0=0x%08x\n", val);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt792xu_wfsys_desc {
    pub rst_reg: u32,
    pub done_reg: u32,
    pub done_mask: u32,
    pub done_val: u32,
    pub delay_ms: u32,
    pub need_status_sel: bool,
}

    static const struct mt792xu_wfsys_desc mt7921_wfsys_desc = {
    .rst_reg = MT_CBTOP_RGU_WF_SUBSYS_RST,
    .done_reg = MT_UDMA_CONN_INFRA_STATUS,
    .done_mask = MT_UDMA_CONN_WFSYS_INIT_DONE,
    .done_val = MT_UDMA_CONN_WFSYS_INIT_DONE,
    .delay_ms = 0,
    .need_status_sel = true,
    };
    static const struct mt792xu_wfsys_desc mt7925_wfsys_desc = {
    .rst_reg = MT7925_CBTOP_RGU_WF_SUBSYS_RST,
    .done_reg = MT7925_WFSYS_INIT_DONE_ADDR,
    .done_mask = U32_MAX,
    .done_val = MT7925_WFSYS_INIT_DONE,
    .delay_ms = 20,
    .need_status_sel = false,
    };
#[no_mangle]
pub unsafe extern "C" fn mt792xu_dma_init(dev: *mut mt792x_dev, resume: bool) -> c_int {
    int mt792xu_dma_init(struct mt792x_dev *dev, bool resume)
    {
    int err;
    mt792xu_wfdma_init(dev);
    mt76_clear(dev, MT_UDMA_WLCFG_0, MT_WL_RX_FLUSH);
    mt76_set(dev, MT_UDMA_WLCFG_0,
    MT_WL_RX_EN | MT_WL_TX_EN |
    MT_WL_RX_MPSZ_PAD0 | MT_TICK_1US_EN);
    mt76_rmw(dev, MT_UDMA_WLCFG_1, MT_WL_TX_TMOUT_LMT,
    FIELD_PREP(MT_WL_TX_TMOUT_LMT,
    MT792X_USB_TX_TIMEOUT_LIMIT));
    mt76_set(dev, MT_UDMA_WLCFG_0, MT_WL_TX_TMOUT_FUNC_EN);
    mt76_clear(dev, MT_UDMA_WLCFG_0,
    MT_WL_RX_AGG_TO | MT_WL_RX_AGG_LMT);
    mt76_clear(dev, MT_UDMA_WLCFG_1, MT_WL_RX_AGG_PKT_LMT);
    if (resume)
    return 0;
    err = mt792xu_dma_rx_evt_ep4(dev);
    if (err)
    return err;
    mt792xu_epctl_rst_opt(dev, false);
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792xu_dma_init);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_wfsys_reset(dev: *mut mt792x_dev) -> c_int {
    int mt792xu_wfsys_reset(struct mt792x_dev *dev)
    {
    const struct mt792xu_wfsys_desc *desc = is_connac3(&dev.mt76) ?
    &mt7925_wfsys_desc :
    &mt7921_wfsys_desc;
    u32 val;
    int i;
    if (atomic_read(&dev.mt76.bus_hung))
    return -EIO;
    mt792xu_wait_udma_idle(dev);
    mt792xu_epctl_rst_opt(dev, false);
    val = mt792xu_uhw_rr(&dev.mt76, desc.rst_reg);
    val |= MT_CBTOP_RGU_WF_SUBSYS_RST_WF_WHOLE_PATH;
    mt792xu_uhw_wr(&dev.mt76, desc.rst_reg, val);
    if (desc.delay_ms)
    msleep(desc.delay_ms);
    else
    usleep_range(10, 20);
    val = mt792xu_uhw_rr(&dev.mt76, desc.rst_reg);
    val &= ~MT_CBTOP_RGU_WF_SUBSYS_RST_WF_WHOLE_PATH;
    mt792xu_uhw_wr(&dev.mt76, desc.rst_reg, val);
    if (desc.need_status_sel)
    mt792xu_uhw_wr(&dev.mt76, MT_UDMA_CONN_INFRA_STATUS_SEL, 0);
    for (i = 0; i < MT792x_WFSYS_INIT_RETRY_COUNT; i++) {
    val = mt792xu_uhw_rr(&dev.mt76, desc.done_reg);
    if ((val & desc.done_mask) == desc.done_val)
    break;
    msleep(100);
    }
    if (i == MT792x_WFSYS_INIT_RETRY_COUNT)
    return -ETIMEDOUT;
    return 0;
    }
    EXPORT_SYMBOL_GPL(mt792xu_wfsys_reset);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_init_reset(dev: *mut mt792x_dev) -> c_int {
    int mt792xu_init_reset(struct mt792x_dev *dev)
    {
    set_bit(MT76_RESET, &dev.mphy.state);
    wake_up(&dev.mt76.mcu.wait);
    skb_queue_purge(&dev.mt76.mcu.res_q);
    mt76u_stop_rx(&dev.mt76);
    mt76u_stop_tx(&dev.mt76);
    mt792xu_wfsys_reset(dev);
    clear_bit(MT76_RESET, &dev.mphy.state);
    return mt76u_resume_rx(&dev.mt76);
    }
    EXPORT_SYMBOL_GPL(mt792xu_init_reset);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_stop(hw: *mut ieee80211_hw, suspend: bool) {
    void mt792xu_stop(struct ieee80211_hw *hw, bool suspend)
    {
    struct mt792x_dev *dev = mt792x_hw_dev(hw);
    mt76u_stop_tx(&dev.mt76);
    mt792x_stop(hw, false);
    }
    EXPORT_SYMBOL_GPL(mt792xu_stop);
#[no_mangle]
pub unsafe extern "C" fn mt792xu_disconnect(usb_intf: *mut usb_interface) {
    void mt792xu_disconnect(struct usb_interface *usb_intf)
    {
    struct mt792x_dev *dev = usb_get_intfdata(usb_intf);
    if (!dev)
    return;
    set_bit(MT76_RESET, &dev.mphy.state);
    set_bit(MT76_MCU_RESET, &dev.mphy.state);
    clear_bit(MT76_STATE_RUNNING, &dev.mphy.state);
    wake_up(&dev.mt76.mcu.wait);
    skb_queue_purge(&dev.mt76.mcu.res_q);
    cancel_work_sync(&dev.reset_work);
    cancel_work_sync(&dev.init_work);
    mt76_worker_disable(&dev.mt76.tx_worker);
    mt792xu_reset_work_cleanup(dev);
    if (!test_bit(MT76_STATE_INITIALIZED, &dev.mphy.state)) {
    set_bit(MT76_REMOVED, &dev.mphy.state);
    return;
    }
    mt76_unregister_device(&dev.mt76);
    mt792xu_cleanup(dev);
    set_bit(MT76_REMOVED, &dev.mphy.state);
    usb_set_intfdata(usb_intf, core::ptr::null_mut());
    mt76_free_device(&dev.mt76);
    }
    EXPORT_SYMBOL_GPL(mt792xu_disconnect);
    MODULE_DESCRIPTION("MediaTek MT792x USB helpers");
    MODULE_LICENSE("Dual BSD/GPL");
    MODULE_AUTHOR("Lorenzo Bianconi <lorenzo@kernel.org>");
