//! Automatically rewritten from C to Rust
//! Source: drivers/misc/pci_endpoint_test.c
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
// Host side test driver to test endpoint functionality
//
// Copyright (C) 2017 Texas Instruments
// Author: Kishon Vijay Abraham I <kishon@ti.com>
//

pub const PCI_ENDPOINT_TEST_MAGIC: c_uint = 0x0;
pub const PCI_ENDPOINT_TEST_COMMAND: c_uint = 0x4;

pub const PCI_ENDPOINT_TEST_STATUS: c_uint = 0x8;

pub const PCI_ENDPOINT_TEST_LOWER_SRC_ADDR: c_uint = 0x0c;
pub const PCI_ENDPOINT_TEST_UPPER_SRC_ADDR: c_uint = 0x10;
pub const PCI_ENDPOINT_TEST_LOWER_DST_ADDR: c_uint = 0x14;
pub const PCI_ENDPOINT_TEST_UPPER_DST_ADDR: c_uint = 0x18;
pub const PCI_ENDPOINT_TEST_SIZE: c_uint = 0x1c;
pub const PCI_ENDPOINT_TEST_CHECKSUM: c_uint = 0x20;
pub const PCI_ENDPOINT_TEST_IRQ_TYPE: c_uint = 0x24;
pub const PCI_ENDPOINT_TEST_IRQ_NUMBER: c_uint = 0x28;
pub const PCI_ENDPOINT_TEST_FLAGS: c_uint = 0x2c;

pub const PCI_ENDPOINT_TEST_CAPS: c_uint = 0x30;

pub const PCI_ENDPOINT_TEST_DB_BAR: c_uint = 0x34;
pub const PCI_ENDPOINT_TEST_DB_OFFSET: c_uint = 0x38;
pub const PCI_ENDPOINT_TEST_DB_DATA: c_uint = 0x3c;
pub const PCI_DEVICE_ID_TI_AM654: c_uint = 0xb00c;
pub const PCI_DEVICE_ID_TI_J7200: c_uint = 0xb00f;
pub const PCI_DEVICE_ID_TI_AM64: c_uint = 0xb010;
pub const PCI_DEVICE_ID_TI_J721S2: c_uint = 0xb013;
pub const PCI_DEVICE_ID_LS1088A: c_uint = 0x80c0;
pub const PCI_DEVICE_ID_IMX8: c_uint = 0x0808;

    ((pdev).device == PCI_DEVICE_ID_TI_AM654)
pub const PCI_DEVICE_ID_RENESAS_R8A774A1: c_uint = 0x0028;
pub const PCI_DEVICE_ID_RENESAS_R8A774B1: c_uint = 0x002b;
pub const PCI_DEVICE_ID_RENESAS_R8A774C0: c_uint = 0x002d;
pub const PCI_DEVICE_ID_RENESAS_R8A774E1: c_uint = 0x0025;
pub const PCI_DEVICE_ID_RENESAS_R8A779F0: c_uint = 0x0031;
pub const PCI_DEVICE_ID_ROCKCHIP_RK3588: c_uint = 0x3588;
pub const PCI_DEVICE_ID_NVIDIA_TEGRA194_EP: c_uint = 0x1ad4;
pub const PCI_DEVICE_ID_NVIDIA_TEGRA234_EP: c_uint = 0x229b;
pub const PCI_ENDPOINT_TEST_BAR_SUBRANGE_NSUB: c_int = 2;
    static DEFINE_IDA(pci_endpoint_test_ida);

    miscdev)
    enum pci_barno {
    BAR_0,
    BAR_1,
    BAR_2,
    BAR_3,
    BAR_4,
    BAR_5,
    NO_BAR = -1,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_endpoint_test {
    pub pdev: *mut pci_dev,
    pub base: *mut void __iomem,
    pub bar: [*mut void __iomem; PCI_STD_NUM_BARS],
    pub irq_raised: completion,
    pub last_irq: c_int,
    pub num_irqs: c_int,
    pub irq_type: c_int,
// mutex to protect the ioctls
    pub mutex: mutex,
    pub miscdev: miscdevice,
    pub test_reg_bar: enum pci_barno,
    pub alignment: usize,
    pub ep_caps: u32,
    pub name: *const c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_endpoint_test_data {
    pub test_reg_bar: enum pci_barno,
    pub alignment: usize,
}

    static inline u32 pci_endpoint_test_readl(struct pci_endpoint_test *test,
    u32 offset)
    {
    return readl(test.base + offset);
    }
    static inline void pci_endpoint_test_writel(struct pci_endpoint_test *test,
    u32 offset, u32 value)
    {
    writel(value, test.base + offset);
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_irqhandler(irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    static irqreturn_t pci_endpoint_test_irqhandler(int irq, void *dev_id)
    {
    struct pci_endpoint_test *test = dev_id;
    u32 reg;
    reg = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_STATUS);
    if (reg & STATUS_IRQ_RAISED) {
    test.last_irq = irq;
    complete(&test.irq_raised);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_free_irq_vectors(test: *mut pci_endpoint_test) {
    static void pci_endpoint_test_free_irq_vectors(struct pci_endpoint_test *test)
    {
    struct pci_dev *pdev = test.pdev;
    pci_free_irq_vectors(pdev);
    test.irq_type = PCITEST_IRQ_TYPE_UNDEFINED;
    }
    static int pci_endpoint_test_alloc_irq_vectors(struct pci_endpoint_test *test,
    int type)
    {
    int irq;
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    switch (type) {
    case PCITEST_IRQ_TYPE_INTX:
    irq = pci_alloc_irq_vectors(pdev, 1, 1, PCI_IRQ_INTX);
    if (irq < 0) {
    dev_err(dev, "Failed to get Legacy interrupt\n");
    return irq;
    }
    break;
    case PCITEST_IRQ_TYPE_MSI:
    irq = pci_alloc_irq_vectors(pdev, 1, 32, PCI_IRQ_MSI);
    if (irq < 0) {
    dev_err(dev, "Failed to get MSI interrupts\n");
    return irq;
    }
    break;
    case PCITEST_IRQ_TYPE_MSIX:
    irq = pci_alloc_irq_vectors(pdev, 1, 2048, PCI_IRQ_MSIX);
    if (irq < 0) {
    dev_err(dev, "Failed to get MSI-X interrupts\n");
    return irq;
    }
    break;
    default:
    dev_err(dev, "Invalid IRQ type selected\n");
    return -EINVAL;
    }
    test.irq_type = type;
    test.num_irqs = irq;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_release_irq(test: *mut pci_endpoint_test) {
    static void pci_endpoint_test_release_irq(struct pci_endpoint_test *test)
    {
    int i;
    struct pci_dev *pdev = test.pdev;
    for (i = 0; i < test.num_irqs; i++)
    free_irq(pci_irq_vector(pdev, i), test);
    test.num_irqs = 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_request_irq(test: *mut pci_endpoint_test) -> c_int {
    static int pci_endpoint_test_request_irq(struct pci_endpoint_test *test)
    {
    int i;
    int ret;
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    for (i = 0; i < test.num_irqs; i++) {
    ret = request_irq(pci_irq_vector(pdev, i),
    pci_endpoint_test_irqhandler, IRQF_SHARED,
    test.name, test);
    if (ret)
    goto fail;
    }
    return 0;
    fail:
    switch (test.irq_type) {
    case PCITEST_IRQ_TYPE_INTX:
    dev_err(dev, "Failed to request IRQ %d for Legacy\n",
    pci_irq_vector(pdev, i));
    break;
    case PCITEST_IRQ_TYPE_MSI:
    dev_err(dev, "Failed to request IRQ %d for MSI %d\n",
    pci_irq_vector(pdev, i),
    i + 1);
    break;
    case PCITEST_IRQ_TYPE_MSIX:
    dev_err(dev, "Failed to request IRQ %d for MSI-X %d\n",
    pci_irq_vector(pdev, i),
    i + 1);
    break;
    }
    test.num_irqs = i;
    pci_endpoint_test_release_irq(test);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn bar_is_reserved(test: *mut pci_endpoint_test, bar: enum pci_barno) -> bool {
    static bool bar_is_reserved(struct pci_endpoint_test *test, enum pci_barno bar)
    {
    return test.ep_caps & BIT(bar + __fls(CAP_BAR0_RESERVED));
    }
    static const u32 bar_test_pattern[] = {
    0xA0A0A0A0,
    0xA1A1A1A1,
    0xA2A2A2A2,
    0xA3A3A3A3,
    0xA4A4A4A4,
    0xA5A5A5A5,
    };
    static int pci_endpoint_test_bar_memcmp(struct pci_endpoint_test *test,
    enum pci_barno barno,
    resource_size_t offset, void *write_buf,
    void *read_buf, int size)
    {
    memset(write_buf, bar_test_pattern[barno], size);
    memcpy_toio(test.bar[barno] + offset, write_buf, size);
    memcpy_fromio(read_buf, test.bar[barno] + offset, size);
    return memcmp(write_buf, read_buf, size);
    }
    static int pci_endpoint_test_bar(struct pci_endpoint_test *test,
    enum pci_barno barno)
    {
    resource_size_t bar_size, offset = 0;
    void *write_buf __free(kfree) = core::ptr::null_mut();
    void *read_buf __free(kfree) = core::ptr::null_mut();
    struct pci_dev *pdev = test.pdev;
    int buf_size;
    bar_size = pci_resource_len(pdev, barno);
    if (!bar_size)
    return -ENODATA;
    if (!test.bar[barno])
    return -ENOMEM;
    if (barno == test.test_reg_bar)
    bar_size = 0x4;
//
// Allocate a buffer of max size 1MB, and reuse that buffer while
// iterating over the whole BAR size (which might be much larger).
//
    buf_size = min(SZ_1M, bar_size);
    write_buf = kmalloc(buf_size, GFP_KERNEL);
    if (!write_buf)
    return -ENOMEM;
    read_buf = kmalloc(buf_size, GFP_KERNEL);
    if (!read_buf)
    return -ENOMEM;
    while (offset < bar_size) {
    if (pci_endpoint_test_bar_memcmp(test, barno, offset, write_buf,
    read_buf, buf_size))
    return -EIO;
    offset += buf_size;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn bar_test_pattern_with_offset(barno: enum pci_barno, offset: c_int) -> u32 {
    static u32 bar_test_pattern_with_offset(enum pci_barno barno, int offset)
    {
    u32 val;
// Keep the BAR pattern in the top byte.
    val = bar_test_pattern[barno] & 0xff000000;
// Store the (partial) offset in the remaining bytes.
    val |= offset & 0x00ffffff;
    return val;
    }
    static void pci_endpoint_test_bars_write_bar(struct pci_endpoint_test *test,
    enum pci_barno barno)
    {
    struct pci_dev *pdev = test.pdev;
    int j, size;
    size = pci_resource_len(pdev, barno);
    if (barno == test.test_reg_bar)
    size = 0x4;
    for (j = 0; j < size; j += 4)
    writel_relaxed(bar_test_pattern_with_offset(barno, j),
    test.bar[barno] + j);
    }
    static int pci_endpoint_test_bars_read_bar(struct pci_endpoint_test *test,
    enum pci_barno barno)
    {
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    int j, size;
    u32 val;
    size = pci_resource_len(pdev, barno);
    if (barno == test.test_reg_bar)
    size = 0x4;
    for (j = 0; j < size; j += 4) {
    let mut expected: u32 = bar_test_pattern_with_offset(barno, j);
    val = readl_relaxed(test.bar[barno] + j);
    if (val != expected) {
    dev_err(dev,
    "BAR%d incorrect data at offset: %#x, got: %#x expected: %#x\n",
    barno, j, val, expected);
    return -EIO;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_bars(test: *mut pci_endpoint_test) -> c_int {
    static int pci_endpoint_test_bars(struct pci_endpoint_test *test)
    {
    enum pci_barno bar;
    int ret;
// Write all BARs in order (without reading).
    for (bar = 0; bar < PCI_STD_NUM_BARS; bar++)
    if (test.bar[bar] && !bar_is_reserved(test, bar))
    pci_endpoint_test_bars_write_bar(test, bar);
//
// Read all BARs in order (without writing).
// If there is an address translation issue on the EP, writing one BAR
// might have overwritten another BAR. Ensure that this is not the case.
// (Reading back the BAR directly after writing can not detect this.)
//
    for (bar = 0; bar < PCI_STD_NUM_BARS; bar++) {
    if (test.bar[bar] && !bar_is_reserved(test, bar)) {
    ret = pci_endpoint_test_bars_read_bar(test, bar);
    if (ret)
    return ret;
    }
    }
    return 0;
    }
    static u8 pci_endpoint_test_subrange_sig_byte(enum pci_barno barno,
    unsigned int subno)
    {
    return 0x50 + (barno * 8) + subno;
    }
    static u8 pci_endpoint_test_subrange_test_byte(enum pci_barno barno,
    unsigned int subno)
    {
    return 0xa0 + (barno * 8) + subno;
    }
    static int pci_endpoint_test_bar_subrange_cmd(struct pci_endpoint_test *test,
    enum pci_barno barno, u32 command,
    u32 ok_bit, u32 fail_bit)
    {
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    let mut irq_type: c_int = test.irq_type;
    u32 status;
    if (irq_type < PCITEST_IRQ_TYPE_INTX ||
    irq_type > PCITEST_IRQ_TYPE_MSIX) {
    dev_err(dev, "Invalid IRQ type\n");
    return -EINVAL;
    }
    reinit_completion(&test.irq_raised);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_STATUS, 0);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE, irq_type);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, 1);
// Reuse SIZE as a command parameter: bar number.
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_SIZE, barno);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND, command);
    if (!wait_for_completion_timeout(&test.irq_raised,
    msecs_to_jiffies(1000)))
    return -ETIMEDOUT;
    status = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_STATUS);
    if (status & fail_bit)
    return (status & STATUS_NO_RESOURCE) ? -ENOSPC : -EIO;
    if (!(status & ok_bit))
    return -EIO;
    return 0;
    }
    static int pci_endpoint_test_bar_subrange_setup(struct pci_endpoint_test *test,
    enum pci_barno barno)
    {
    return pci_endpoint_test_bar_subrange_cmd(test, barno,
    COMMAND_BAR_SUBRANGE_SETUP,
    STATUS_BAR_SUBRANGE_SETUP_SUCCESS,
    STATUS_BAR_SUBRANGE_SETUP_FAIL);
    }
    static int pci_endpoint_test_bar_subrange_clear(struct pci_endpoint_test *test,
    enum pci_barno barno)
    {
    return pci_endpoint_test_bar_subrange_cmd(test, barno,
    COMMAND_BAR_SUBRANGE_CLEAR,
    STATUS_BAR_SUBRANGE_CLEAR_SUCCESS,
    STATUS_BAR_SUBRANGE_CLEAR_FAIL);
    }
    static int pci_endpoint_test_bar_subrange(struct pci_endpoint_test *test,
    enum pci_barno barno)
    {
    let mut nsub: u32 = PCI_ENDPOINT_TEST_BAR_SUBRANGE_NSUB;
    struct device *dev = &test.pdev.dev;
    size_t sub_size, buf_size;
    resource_size_t bar_size;
    void __iomem *bar_addr;
    void *read_buf = core::ptr::null_mut();
    int ret, clear_ret;
    size_t off, chunk;
    u32 i, exp, val;
    u8 pattern;
    if (!(test.ep_caps & CAP_SUBRANGE_MAPPING))
    return -EOPNOTSUPP;
//
// The test register BAR is not safe to reprogram and write/read
// over its full size. BAR_TEST already special-cases it to a tiny
// range. For subrange mapping tests, let's simply skip it.
//
    if (barno == test.test_reg_bar)
    return -EBUSY;
    bar_size = pci_resource_len(test.pdev, barno);
    if (!bar_size)
    return -ENODATA;
    bar_addr = test.bar[barno];
    if (!bar_addr)
    return -ENOMEM;
    ret = pci_endpoint_test_bar_subrange_setup(test, barno);
    if (ret)
    return ret;
    if (bar_size % nsub || bar_size / nsub > SIZE_MAX) {
    ret = -EINVAL;
    goto out_clear;
    }
    sub_size = bar_size / nsub;
    if (sub_size < sizeof(u32)) {
    ret = -EINVAL;
    goto out_clear;
    }
// Limit the temporary buffer size
    buf_size = min_t(size_t, sub_size, SZ_1M);
    read_buf = kmalloc(buf_size, GFP_KERNEL);
    if (!read_buf) {
    ret = -ENOMEM;
    goto out_clear;
    }
//
// Step 1: verify EP-provided signature per subrange. This detects
// whether the EP actually applied the submap order.
//
    for (i = 0; i < nsub; i++) {
    exp = (u32)pci_endpoint_test_subrange_sig_byte(barno, i) *
    0x01010101U;
    val = ioread32(bar_addr + (i * sub_size));
    if (val != exp) {
    dev_err(dev,
    "BAR%d subrange%u signature mismatch @%#zx: exp %#08x got %#08x\n",
    barno, i, (size_t)i * sub_size, exp, val);
    ret = -EIO;
    goto out_clear;
    }
    val = ioread32(bar_addr + (i * sub_size) + sub_size - sizeof(u32));
    if (val != exp) {
    dev_err(dev,
    "BAR%d subrange%u signature mismatch @%#zx: exp %#08x got %#08x\n",
    barno, i,
    ((size_t)i * sub_size) + sub_size - sizeof(u32),
    exp, val);
    ret = -EIO;
    goto out_clear;
    }
    }
// Step 2: write unique pattern per subrange (write all first).
    for (i = 0; i < nsub; i++) {
    pattern = pci_endpoint_test_subrange_test_byte(barno, i);
    memset_io(bar_addr + (i * sub_size), pattern, sub_size);
    }
// Step 3: read back and verify (read all after all writes).
    for (i = 0; i < nsub; i++) {
    pattern = pci_endpoint_test_subrange_test_byte(barno, i);
    for (off = 0; off < sub_size; off += chunk) {
    void *bad;
    chunk = min_t(size_t, buf_size, sub_size - off);
    memcpy_fromio(read_buf, bar_addr + (i * sub_size) + off,
    chunk);
    bad = memchr_inv(read_buf, pattern, chunk);
    if (bad) {
    let mut bad_off: usize = (u8 *)bad - (u8 *)read_buf;
    dev_err(dev,
    "BAR%d subrange%u data mismatch @%#zx (pattern %#02x)\n",
    barno, i, (size_t)i * sub_size + off + bad_off,
    pattern);
    ret = -EIO;
    goto out_clear;
    }
    }
    }
    out_clear:
    kfree(read_buf);
    clear_ret = pci_endpoint_test_bar_subrange_clear(test, barno);
    return ret ?: clear_ret;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_intx_irq(test: *mut pci_endpoint_test) -> c_int {
    static int pci_endpoint_test_intx_irq(struct pci_endpoint_test *test)
    {
    u32 val;
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE,
    PCITEST_IRQ_TYPE_INTX);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, 0);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND,
    COMMAND_RAISE_INTX_IRQ);
    val = wait_for_completion_timeout(&test.irq_raised,
    msecs_to_jiffies(1000));
    if (!val)
    return -ETIMEDOUT;
    return 0;
    }
    static int pci_endpoint_test_msi_irq(struct pci_endpoint_test *test,
    u16 msi_num, bool msix)
    {
    struct pci_dev *pdev = test.pdev;
    u32 val;
    int irq;
    irq = pci_irq_vector(pdev, msi_num - 1);
    if (irq < 0)
    return irq;
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE,
    msix ? PCITEST_IRQ_TYPE_MSIX :
    PCITEST_IRQ_TYPE_MSI);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, msi_num);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND,
    msix ? COMMAND_RAISE_MSIX_IRQ :
    COMMAND_RAISE_MSI_IRQ);
    val = wait_for_completion_timeout(&test.irq_raised,
    msecs_to_jiffies(1000));
    if (!val)
    return -ETIMEDOUT;
    if (irq != test.last_irq)
    return -EIO;
    return 0;
    }
    static int pci_endpoint_test_validate_xfer_params(struct device *dev,
    struct pci_endpoint_test_xfer_param *param, size_t alignment)
    {
    if (!param.size) {
    dev_dbg(dev, "Data size is zero\n");
    return -EINVAL;
    }
    if (param.size > SIZE_MAX - alignment) {
    dev_dbg(dev, "Maximum transfer data size exceeded\n");
    return -EINVAL;
    }
    return 0;
    }
    static int pci_endpoint_test_copy(struct pci_endpoint_test *test,
    unsigned long arg)
    {
    struct pci_endpoint_test_xfer_param param;
    void *src_addr;
    void *dst_addr;
    let mut flags: u32 = 0;
    bool use_dma;
    size_t size;
    dma_addr_t src_phys_addr;
    dma_addr_t dst_phys_addr;
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    void *orig_src_addr;
    dma_addr_t orig_src_phys_addr;
    void *orig_dst_addr;
    dma_addr_t orig_dst_phys_addr;
    size_t offset;
    let mut alignment: usize = test.alignment;
    let mut irq_type: c_int = test.irq_type;
    u32 src_crc32;
    u32 dst_crc32;
    int ret;
    ret = copy_from_user(&param, (void __user *)arg, sizeof(param));
    if (ret) {
    dev_err(dev, "Failed to get transfer param\n");
    return -EFAULT;
    }
    ret = pci_endpoint_test_validate_xfer_params(dev, &param, alignment);
    if (ret)
    return ret;
    size = param.size;
    use_dma = !!(param.flags & PCITEST_FLAGS_USE_DMA);
    if (use_dma)
    flags |= FLAG_USE_DMA;
    if (irq_type < PCITEST_IRQ_TYPE_INTX ||
    irq_type > PCITEST_IRQ_TYPE_MSIX) {
    dev_err(dev, "Invalid IRQ type option\n");
    return -EINVAL;
    }
    orig_src_addr = kzalloc(size + alignment, GFP_KERNEL);
    if (!orig_src_addr) {
    dev_err(dev, "Failed to allocate source buffer\n");
    return -ENOMEM;
    }
    get_random_bytes(orig_src_addr, size + alignment);
    orig_src_phys_addr = dma_map_single(dev, orig_src_addr,
    size + alignment, DMA_TO_DEVICE);
    ret = dma_mapping_error(dev, orig_src_phys_addr);
    if (ret) {
    dev_err(dev, "failed to map source buffer address\n");
    goto err_src_phys_addr;
    }
    if (alignment && !IS_ALIGNED(orig_src_phys_addr, alignment)) {
    src_phys_addr = PTR_ALIGN(orig_src_phys_addr, alignment);
    offset = src_phys_addr - orig_src_phys_addr;
    src_addr = orig_src_addr + offset;
    } else {
    src_phys_addr = orig_src_phys_addr;
    src_addr = orig_src_addr;
    }
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_LOWER_SRC_ADDR,
    lower_32_bits(src_phys_addr));
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_UPPER_SRC_ADDR,
    upper_32_bits(src_phys_addr));
    src_crc32 = crc32_le(~0, src_addr, size);
    orig_dst_addr = kzalloc(size + alignment, GFP_KERNEL);
    if (!orig_dst_addr) {
    dev_err(dev, "Failed to allocate destination address\n");
    ret = -ENOMEM;
    goto err_dst_addr;
    }
    orig_dst_phys_addr = dma_map_single(dev, orig_dst_addr,
    size + alignment, DMA_FROM_DEVICE);
    ret = dma_mapping_error(dev, orig_dst_phys_addr);
    if (ret) {
    dev_err(dev, "failed to map destination buffer address\n");
    goto err_dst_phys_addr;
    }
    if (alignment && !IS_ALIGNED(orig_dst_phys_addr, alignment)) {
    dst_phys_addr = PTR_ALIGN(orig_dst_phys_addr, alignment);
    offset = dst_phys_addr - orig_dst_phys_addr;
    dst_addr = orig_dst_addr + offset;
    } else {
    dst_phys_addr = orig_dst_phys_addr;
    dst_addr = orig_dst_addr;
    }
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_LOWER_DST_ADDR,
    lower_32_bits(dst_phys_addr));
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_UPPER_DST_ADDR,
    upper_32_bits(dst_phys_addr));
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_SIZE,
    size);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_FLAGS, flags);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE, irq_type);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, 1);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND,
    COMMAND_COPY);
    wait_for_completion(&test.irq_raised);
    dma_unmap_single(dev, orig_dst_phys_addr, size + alignment,
    DMA_FROM_DEVICE);
    dst_crc32 = crc32_le(~0, dst_addr, size);
    if (dst_crc32 != src_crc32)
    ret = -EIO;
    err_dst_phys_addr:
    kfree(orig_dst_addr);
    err_dst_addr:
    dma_unmap_single(dev, orig_src_phys_addr, size + alignment,
    DMA_TO_DEVICE);
    err_src_phys_addr:
    kfree(orig_src_addr);
    return ret;
    }
    static int pci_endpoint_test_write(struct pci_endpoint_test *test,
    unsigned long arg)
    {
    struct pci_endpoint_test_xfer_param param;
    let mut flags: u32 = 0;
    bool use_dma;
    u32 reg;
    void *addr;
    dma_addr_t phys_addr;
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    void *orig_addr;
    dma_addr_t orig_phys_addr;
    size_t offset;
    let mut alignment: usize = test.alignment;
    let mut irq_type: c_int = test.irq_type;
    size_t size;
    u32 crc32;
    int ret;
    ret = copy_from_user(&param, (void __user *)arg, sizeof(param));
    if (ret) {
    dev_err(dev, "Failed to get transfer param\n");
    return -EFAULT;
    }
    ret = pci_endpoint_test_validate_xfer_params(dev, &param, alignment);
    if (ret)
    return ret;
    size = param.size;
    use_dma = !!(param.flags & PCITEST_FLAGS_USE_DMA);
    if (use_dma)
    flags |= FLAG_USE_DMA;
    if (irq_type < PCITEST_IRQ_TYPE_INTX ||
    irq_type > PCITEST_IRQ_TYPE_MSIX) {
    dev_err(dev, "Invalid IRQ type option\n");
    return -EINVAL;
    }
    orig_addr = kzalloc(size + alignment, GFP_KERNEL);
    if (!orig_addr) {
    dev_err(dev, "Failed to allocate address\n");
    return -ENOMEM;
    }
    get_random_bytes(orig_addr, size + alignment);
    orig_phys_addr = dma_map_single(dev, orig_addr, size + alignment,
    DMA_TO_DEVICE);
    ret = dma_mapping_error(dev, orig_phys_addr);
    if (ret) {
    dev_err(dev, "failed to map source buffer address\n");
    goto err_phys_addr;
    }
    if (alignment && !IS_ALIGNED(orig_phys_addr, alignment)) {
    phys_addr =  PTR_ALIGN(orig_phys_addr, alignment);
    offset = phys_addr - orig_phys_addr;
    addr = orig_addr + offset;
    } else {
    phys_addr = orig_phys_addr;
    addr = orig_addr;
    }
    crc32 = crc32_le(~0, addr, size);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_CHECKSUM,
    crc32);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_LOWER_SRC_ADDR,
    lower_32_bits(phys_addr));
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_UPPER_SRC_ADDR,
    upper_32_bits(phys_addr));
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_SIZE, size);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_FLAGS, flags);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE, irq_type);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, 1);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND,
    COMMAND_READ);
    wait_for_completion(&test.irq_raised);
    reg = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_STATUS);
    if (!(reg & STATUS_READ_SUCCESS))
    ret = -EIO;
    dma_unmap_single(dev, orig_phys_addr, size + alignment,
    DMA_TO_DEVICE);
    err_phys_addr:
    kfree(orig_addr);
    return ret;
    }
    static int pci_endpoint_test_read(struct pci_endpoint_test *test,
    unsigned long arg)
    {
    struct pci_endpoint_test_xfer_param param;
    let mut flags: u32 = 0;
    bool use_dma;
    size_t size;
    void *addr;
    dma_addr_t phys_addr;
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    void *orig_addr;
    dma_addr_t orig_phys_addr;
    size_t offset;
    let mut alignment: usize = test.alignment;
    let mut irq_type: c_int = test.irq_type;
    u32 crc32;
    int ret;
    ret = copy_from_user(&param, (void __user *)arg, sizeof(param));
    if (ret) {
    dev_err(dev, "Failed to get transfer param\n");
    return -EFAULT;
    }
    ret = pci_endpoint_test_validate_xfer_params(dev, &param, alignment);
    if (ret)
    return ret;
    size = param.size;
    use_dma = !!(param.flags & PCITEST_FLAGS_USE_DMA);
    if (use_dma)
    flags |= FLAG_USE_DMA;
    if (irq_type < PCITEST_IRQ_TYPE_INTX ||
    irq_type > PCITEST_IRQ_TYPE_MSIX) {
    dev_err(dev, "Invalid IRQ type option\n");
    return -EINVAL;
    }
    orig_addr = kzalloc(size + alignment, GFP_KERNEL);
    if (!orig_addr) {
    dev_err(dev, "Failed to allocate destination address\n");
    return -ENOMEM;
    }
    orig_phys_addr = dma_map_single(dev, orig_addr, size + alignment,
    DMA_FROM_DEVICE);
    ret = dma_mapping_error(dev, orig_phys_addr);
    if (ret) {
    dev_err(dev, "failed to map source buffer address\n");
    goto err_phys_addr;
    }
    if (alignment && !IS_ALIGNED(orig_phys_addr, alignment)) {
    phys_addr = PTR_ALIGN(orig_phys_addr, alignment);
    offset = phys_addr - orig_phys_addr;
    addr = orig_addr + offset;
    } else {
    phys_addr = orig_phys_addr;
    addr = orig_addr;
    }
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_LOWER_DST_ADDR,
    lower_32_bits(phys_addr));
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_UPPER_DST_ADDR,
    upper_32_bits(phys_addr));
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_SIZE, size);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_FLAGS, flags);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE, irq_type);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, 1);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND,
    COMMAND_WRITE);
    wait_for_completion(&test.irq_raised);
    dma_unmap_single(dev, orig_phys_addr, size + alignment,
    DMA_FROM_DEVICE);
    crc32 = crc32_le(~0, addr, size);
    if (crc32 != pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_CHECKSUM))
    ret = -EIO;
    err_phys_addr:
    kfree(orig_addr);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_clear_irq(test: *mut pci_endpoint_test) -> c_int {
    static int pci_endpoint_test_clear_irq(struct pci_endpoint_test *test)
    {
    pci_endpoint_test_release_irq(test);
    pci_endpoint_test_free_irq_vectors(test);
    return 0;
    }
    static int pci_endpoint_test_set_irq(struct pci_endpoint_test *test,
    int req_irq_type)
    {
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    int ret;
    if (req_irq_type < PCITEST_IRQ_TYPE_INTX ||
    req_irq_type > PCITEST_IRQ_TYPE_AUTO) {
    dev_err(dev, "Invalid IRQ type option\n");
    return -EINVAL;
    }
    if (req_irq_type == PCITEST_IRQ_TYPE_AUTO) {
    if (test.ep_caps & CAP_MSI)
    req_irq_type = PCITEST_IRQ_TYPE_MSI;
#[no_mangle]
pub unsafe extern "C" fn if(CAP_MSIX: test->ep_caps &) -> else {
    else if (test.ep_caps & CAP_MSIX)
    req_irq_type = PCITEST_IRQ_TYPE_MSIX;
#[no_mangle]
pub unsafe extern "C" fn if(CAP_INTX: test->ep_caps &) -> else {
    else if (test.ep_caps & CAP_INTX)
    req_irq_type = PCITEST_IRQ_TYPE_INTX;
    else
// fallback to MSI if no caps defined
    req_irq_type = PCITEST_IRQ_TYPE_MSI;
    }
    if (test.irq_type == req_irq_type)
    return 0;
    pci_endpoint_test_release_irq(test);
    pci_endpoint_test_free_irq_vectors(test);
    ret = pci_endpoint_test_alloc_irq_vectors(test, req_irq_type);
    if (ret)
    return ret;
    ret = pci_endpoint_test_request_irq(test);
    if (ret) {
    pci_endpoint_test_free_irq_vectors(test);
    return ret;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_doorbell(test: *mut pci_endpoint_test) -> c_int {
    static int pci_endpoint_test_doorbell(struct pci_endpoint_test *test)
    {
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    let mut irq_type: c_int = test.irq_type;
    let mut ret: c_int = 0;
    enum pci_barno bar;
    u32 data, status;
    u32 addr;
    int left;
    if (!(test.ep_caps & CAP_DYNAMIC_INBOUND_MAPPING))
    return -EOPNOTSUPP;
    if (irq_type < PCITEST_IRQ_TYPE_INTX ||
    irq_type > PCITEST_IRQ_TYPE_MSIX) {
    dev_err(dev, "Invalid IRQ type\n");
    return -EINVAL;
    }
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE, irq_type);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, 1);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND,
    COMMAND_ENABLE_DOORBELL);
    left = wait_for_completion_timeout(&test.irq_raised, msecs_to_jiffies(1000));
    status = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_STATUS);
    if (!left || !(status & STATUS_DOORBELL_ENABLE_SUCCESS)) {
    dev_err(dev, "Failed to enable doorbell\n");
    return -EINVAL;
    }
    data = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_DB_DATA);
    addr = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_DB_OFFSET);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_TYPE, irq_type);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_IRQ_NUMBER, 1);
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_STATUS, 0);
    bar = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_DB_BAR);
    if (bar < BAR_0 || bar >= PCI_STD_NUM_BARS) {
    dev_err(dev, "BAR %d reported by endpoint out of range [0, %u]\n",
    bar, PCI_STD_NUM_BARS - 1);
    return -ERANGE;
    }
    writel(data, test.bar[bar] + addr);
    left = wait_for_completion_timeout(&test.irq_raised, msecs_to_jiffies(1000));
    status = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_STATUS);
    if (!left || !(status & STATUS_DOORBELL_SUCCESS)) {
    dev_err(dev, "Failed to trigger doorbell in endpoint\n");
// Store error code, but continue to disable doorbell.
    ret = -EINVAL;
    }
    pci_endpoint_test_writel(test, PCI_ENDPOINT_TEST_COMMAND,
    COMMAND_DISABLE_DOORBELL);
    wait_for_completion_timeout(&test.irq_raised, msecs_to_jiffies(1000));
    status |= pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_STATUS);
    if (!(status & STATUS_DOORBELL_DISABLE_SUCCESS)) {
    dev_err(dev, "Failed to disable doorbell\n");
    return -EINVAL;
    }
    return ret;
    }
    static long pci_endpoint_test_ioctl(struct file *file, unsigned int cmd,
    unsigned long arg)
    {
    let mut ret: c_int = -EINVAL;
    enum pci_barno bar;
    struct pci_endpoint_test *test = to_endpoint_test(file.private_data);
    struct pci_dev *pdev = test.pdev;
    mutex_lock(&test.mutex);
    reinit_completion(&test.irq_raised);
    test.last_irq = -ENODATA;
    switch (cmd) {
    case PCITEST_BAR:
    case PCITEST_BAR_SUBRANGE:
    bar = arg;
    if (bar <= NO_BAR || bar > BAR_5)
    goto ret;
    if (is_am654_pci_dev(pdev) && bar == BAR_0)
    goto ret;
    if (bar_is_reserved(test, bar)) {
    ret = -ENOBUFS;
    goto ret;
    }
    if (cmd == PCITEST_BAR)
    ret = pci_endpoint_test_bar(test, bar);
    else
    ret = pci_endpoint_test_bar_subrange(test, bar);
    break;
    case PCITEST_BARS:
    ret = pci_endpoint_test_bars(test);
    break;
    case PCITEST_INTX_IRQ:
    ret = pci_endpoint_test_intx_irq(test);
    break;
    case PCITEST_MSI:
    case PCITEST_MSIX:
    ret = pci_endpoint_test_msi_irq(test, arg, cmd == PCITEST_MSIX);
    break;
    case PCITEST_WRITE:
    ret = pci_endpoint_test_write(test, arg);
    break;
    case PCITEST_READ:
    ret = pci_endpoint_test_read(test, arg);
    break;
    case PCITEST_COPY:
    ret = pci_endpoint_test_copy(test, arg);
    break;
    case PCITEST_SET_IRQTYPE:
    ret = pci_endpoint_test_set_irq(test, arg);
    break;
    case PCITEST_GET_IRQTYPE:
    ret = test.irq_type;
    break;
    case PCITEST_CLEAR_IRQ:
    ret = pci_endpoint_test_clear_irq(test);
    break;
    case PCITEST_DOORBELL:
    ret = pci_endpoint_test_doorbell(test);
    break;
    }
    ret:
    mutex_unlock(&test.mutex);
    return ret;
    }
    static const struct file_operations pci_endpoint_test_fops = {
    .owner = THIS_MODULE,
    .unlocked_ioctl = pci_endpoint_test_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_get_capabilities(test: *mut pci_endpoint_test) {
    static void pci_endpoint_test_get_capabilities(struct pci_endpoint_test *test)
    {
    struct pci_dev *pdev = test.pdev;
    struct device *dev = &pdev.dev;
    test.ep_caps = pci_endpoint_test_readl(test, PCI_ENDPOINT_TEST_CAPS);
    dev_dbg(dev, "PCI_ENDPOINT_TEST_CAPS: %#x\n", test.ep_caps);
// CAP_UNALIGNED_ACCESS is set if the EP can do unaligned access
    if (test.ep_caps & CAP_UNALIGNED_ACCESS)
    test.alignment = 0;
    }
    static int pci_endpoint_test_probe(struct pci_dev *pdev,
    const struct pci_device_id *ent)
    {
    int ret;
    int id;
    char name[29];
    enum pci_barno bar;
    void __iomem *base;
    struct device *dev = &pdev.dev;
    struct pci_endpoint_test *test;
    struct pci_endpoint_test_data *data;
    let mut test_reg_bar: enum pci_barno = BAR_0;
    struct miscdevice *misc_device;
    if (pci_is_bridge(pdev))
    return -ENODEV;
    test = devm_kzalloc(dev, sizeof(*test), GFP_KERNEL);
    if (!test)
    return -ENOMEM;
    test.pdev = pdev;
    test.irq_type = PCITEST_IRQ_TYPE_UNDEFINED;
    data = (struct pci_endpoint_test_data *)ent.driver_data;
    if (data) {
    test_reg_bar = data.test_reg_bar;
    test.test_reg_bar = test_reg_bar;
    test.alignment = data.alignment;
    }
    init_completion(&test.irq_raised);
    mutex_init(&test.mutex);
    dma_set_mask_and_coherent(&pdev.dev, DMA_BIT_MASK(48));
    ret = pci_enable_device(pdev);
    if (ret) {
    dev_err(dev, "Cannot enable PCI device\n");
    return ret;
    }
    ret = pci_request_regions(pdev, DRV_MODULE_NAME);
    if (ret) {
    dev_err(dev, "Cannot obtain PCI resources\n");
    goto err_disable_pdev;
    }
    pci_set_master(pdev);
    for (bar = 0; bar < PCI_STD_NUM_BARS; bar++) {
    if (pci_resource_flags(pdev, bar) & IORESOURCE_MEM) {
    base = pci_ioremap_bar(pdev, bar);
    if (!base) {
    dev_err(dev, "Failed to read BAR%d\n", bar);
    WARN_ON(bar == test_reg_bar);
    }
    test.bar[bar] = base;
    }
    }
    test.base = test.bar[test_reg_bar];
    if (!test.base) {
    ret = -ENOMEM;
    dev_err(dev, "Cannot perform PCI test without BAR%d\n",
    test_reg_bar);
    goto err_iounmap;
    }
    pci_set_drvdata(pdev, test);
    id = ida_alloc(&pci_endpoint_test_ida, GFP_KERNEL);
    if (id < 0) {
    ret = id;
    dev_err(dev, "Unable to get id\n");
    goto err_iounmap;
    }
    snprintf(name, sizeof(name), DRV_MODULE_NAME ".%d", id);
    test.name = kstrdup(name, GFP_KERNEL);
    if (!test.name) {
    ret = -ENOMEM;
    goto err_ida_remove;
    }
    pci_endpoint_test_get_capabilities(test);
    misc_device = &test.miscdev;
    misc_device.minor = MISC_DYNAMIC_MINOR;
    misc_device.name = kstrdup(name, GFP_KERNEL);
    if (!misc_device.name) {
    ret = -ENOMEM;
    goto err_kfree_test_name;
    }
    misc_device.parent = &pdev.dev;
    misc_device.fops = &pci_endpoint_test_fops;
    pci_save_state(pdev);
    ret = misc_register(misc_device);
    if (ret) {
    dev_err(dev, "Failed to register device\n");
    goto err_kfree_name;
    }
    return 0;
    err_kfree_name:
    kfree(misc_device.name);
    err_kfree_test_name:
    kfree(test.name);
    err_ida_remove:
    ida_free(&pci_endpoint_test_ida, id);
    err_iounmap:
    for (bar = 0; bar < PCI_STD_NUM_BARS; bar++) {
    if (test.bar[bar])
    pci_iounmap(pdev, test.bar[bar]);
    }
    pci_release_regions(pdev);
    err_disable_pdev:
    pci_disable_device(pdev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_remove(pdev: *mut pci_dev) {
    static void pci_endpoint_test_remove(struct pci_dev *pdev)
    {
    int id;
    enum pci_barno bar;
    struct pci_endpoint_test *test = pci_get_drvdata(pdev);
    struct miscdevice *misc_device = &test.miscdev;
    if (sscanf(misc_device.name, DRV_MODULE_NAME ".%d", &id) != 1)
    return;
    if (id < 0)
    return;
    pci_endpoint_test_release_irq(test);
    pci_endpoint_test_free_irq_vectors(test);
    misc_deregister(&test.miscdev);
    kfree(misc_device.name);
    kfree(test.name);
    ida_free(&pci_endpoint_test_ida, id);
    for (bar = 0; bar < PCI_STD_NUM_BARS; bar++) {
    if (test.bar[bar])
    pci_iounmap(pdev, test.bar[bar]);
    }
    pci_release_regions(pdev);
    pci_disable_device(pdev);
    }
    static const struct pci_endpoint_test_data default_data = {
    .test_reg_bar = BAR_0,
    .alignment = SZ_4K,
    };
    static const struct pci_endpoint_test_data am654_data = {
    .test_reg_bar = BAR_2,
    .alignment = SZ_64K,
    };
    static const struct pci_endpoint_test_data j721e_data = {
    .alignment = 256,
    };
    static const struct pci_endpoint_test_data rk3588_data = {
    .alignment = SZ_64K,
    };
//
// If the controller's Vendor/Device ID are programmable, you may be able to
// use one of the existing entries for testing instead of adding a new one.
//
    static const struct pci_device_id pci_endpoint_test_tbl[] = {
    { PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_DRA74x),
    .driver_data = (kernel_ulong_t)&default_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_DRA72x),
    .driver_data = (kernel_ulong_t)&default_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_FREESCALE, 0x81c0),
    .driver_data = (kernel_ulong_t)&default_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_FREESCALE, PCI_DEVICE_ID_IMX8) },
    { PCI_DEVICE(PCI_VENDOR_ID_FREESCALE, PCI_DEVICE_ID_LS1088A),
    .driver_data = (kernel_ulong_t)&default_data,
    },
    { PCI_DEVICE_DATA(SYNOPSYS, EDDA, core::ptr::null_mut()) },
    { PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_AM654),
    .driver_data = (kernel_ulong_t)&am654_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_RENESAS, PCI_DEVICE_ID_RENESAS_R8A774A1) },
    { PCI_DEVICE(PCI_VENDOR_ID_RENESAS, PCI_DEVICE_ID_RENESAS_R8A774B1) },
    { PCI_DEVICE(PCI_VENDOR_ID_RENESAS, PCI_DEVICE_ID_RENESAS_R8A774C0) },
    { PCI_DEVICE(PCI_VENDOR_ID_RENESAS, PCI_DEVICE_ID_RENESAS_R8A774E1) },
    { PCI_DEVICE(PCI_VENDOR_ID_RENESAS, PCI_DEVICE_ID_RENESAS_R8A779F0),
    .driver_data = (kernel_ulong_t)&default_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_J721E),
    .driver_data = (kernel_ulong_t)&j721e_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_J7200),
    .driver_data = (kernel_ulong_t)&j721e_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_AM64),
    .driver_data = (kernel_ulong_t)&j721e_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_TI, PCI_DEVICE_ID_TI_J721S2),
    .driver_data = (kernel_ulong_t)&j721e_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_ROCKCHIP, PCI_DEVICE_ID_ROCKCHIP_RK3588),
    .driver_data = (kernel_ulong_t)&rk3588_data,
    },
    { PCI_DEVICE(PCI_VENDOR_ID_NVIDIA, PCI_DEVICE_ID_NVIDIA_TEGRA194_EP) },
    { PCI_DEVICE(PCI_VENDOR_ID_NVIDIA, PCI_DEVICE_ID_NVIDIA_TEGRA234_EP) },
    { }
    };
    MODULE_DEVICE_TABLE(pci, pci_endpoint_test_tbl);
    static pci_ers_result_t pci_endpoint_test_error_detected(struct pci_dev *pdev,
    pci_channel_state_t state)
    {
    if (state == pci_channel_io_perm_failure)
    return PCI_ERS_RESULT_DISCONNECT;
    return PCI_ERS_RESULT_NEED_RESET;
    }
#[no_mangle]
unsafe extern "C" fn pci_endpoint_test_slot_reset(pdev: *mut pci_dev) -> pci_ers_result_t {
    static pci_ers_result_t pci_endpoint_test_slot_reset(struct pci_dev *pdev)
    {
    pci_restore_state(pdev);
    return PCI_ERS_RESULT_RECOVERED;
    }
    static const struct pci_error_handlers pci_endpoint_test_err_handler = {
    .error_detected = pci_endpoint_test_error_detected,
    .slot_reset = pci_endpoint_test_slot_reset,
    };
    static struct pci_driver pci_endpoint_test_driver = {
    .name		= DRV_MODULE_NAME,
    .id_table	= pci_endpoint_test_tbl,
    .probe		= pci_endpoint_test_probe,
    .remove		= pci_endpoint_test_remove,
    .sriov_configure = pci_sriov_configure_simple,
    .err_handler    = &pci_endpoint_test_err_handler,
    };
    module_pci_driver(pci_endpoint_test_driver);
    MODULE_DESCRIPTION("PCI ENDPOINT TEST HOST DRIVER");
    MODULE_AUTHOR("Kishon Vijay Abraham I <kishon@ti.com>");
    MODULE_LICENSE("GPL v2");
