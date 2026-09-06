//! Automatically rewritten from C to Rust
//! Source: drivers/misc/mrvl_cn10k_dpi.c
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
// Marvell Octeon CN10K DPI driver
//
// Copyright (C) 2024 Marvell.
//

// PCI device IDs
pub const PCI_DEVID_MRVL_CN10K_DPI_PF: c_uint = 0xA080;
pub const PCI_SUBDEVID_MRVL_CN10K_DPI_PF: c_uint = 0xB900;
// PCI BAR Number
pub const PCI_DPI_CFG_BAR: c_int = 0;
// MSI-X interrupts
pub const DPI_MAX_REQQ_INT: c_uint = 0x20;
pub const DPI_MAX_CC_INT: c_uint = 0x40;
// MBOX MSI-X interrupt vector index
pub const DPI_MBOX_PF_VF_INT_IDX: c_uint = 0x75;

pub const DPI_MAX_VFS: c_uint = 0x20;
pub const DPI_MAX_ENG_FIFO_SZ: c_uint = 0x20;
pub const DPI_MAX_ENG_MOLR: c_uint = 0x400;

    (DPI_REQQ_INT_INSTRFLT		| \
    DPI_REQQ_INT_RDFLT		| \
    DPI_REQQ_INT_WRFLT		| \
    DPI_REQQ_INT_CSFLT		| \
    DPI_REQQ_INT_INST_DBO		| \
    DPI_REQQ_INT_INST_ADDR_NULL	| \
    DPI_REQQ_INT_INST_FILL_INVAL	| \
    DPI_REQQ_INT_INSTR_PSN)

    (DPI_PF_RAS_EBI_DAT_PSN  | \
    DPI_PF_RAS_NCB_DAT_PSN  | \
    DPI_PF_RAS_NCB_CMD_PSN)
// Message fields in word_l of DPI mailbox structure

// Message fields in word_h of DPI mailbox structure

pub const DPI_CTL: c_uint = 0x10010ULL;
pub const DPI_DMA_CONTROL: c_uint = 0x10018ULL;
pub const DPI_PF_RAS: c_uint = 0x10308ULL;
pub const DPI_PF_RAS_ENA_W1C: c_uint = 0x10318ULL;
pub const DPI_MBOX_VF_PF_INT: c_uint = 0x16300ULL;
pub const DPI_MBOX_VF_PF_INT_W1S: c_uint = 0x16308ULL;
pub const DPI_MBOX_VF_PF_INT_ENA_W1C: c_uint = 0x16310ULL;
pub const DPI_MBOX_VF_PF_INT_ENA_W1S: c_uint = 0x16318ULL;

pub const DPI_WCTL_FIF_THR: c_uint = 0x17008ULL;
pub const DPI_EBUS_MAX_PORTS: c_int = 2;
pub const DPI_EBUS_MRRS_MIN: c_int = 128;
pub const DPI_EBUS_MRRS_MAX: c_int = 1024;
pub const DPI_EBUS_MPS_MIN: c_int = 128;
pub const DPI_EBUS_MPS_MAX: c_int = 1024;
pub const DPI_WCTL_FIFO_THRESHOLD: c_uint = 0x30;
pub const DPI_QUEUE_OPEN: c_uint = 0x1;
pub const DPI_QUEUE_CLOSE: c_uint = 0x2;
pub const DPI_REG_DUMP: c_uint = 0x3;
pub const DPI_GET_REG_CFG: c_uint = 0x4;
pub const DPI_QUEUE_OPEN_V2: c_uint = 0x5;
    enum dpi_mbox_rsp_type {
    DPI_MBOX_TYPE_CMD,
    DPI_MBOX_TYPE_RSP_ACK,
    DPI_MBOX_TYPE_RSP_NACK,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpivf_config {
    pub aura: u32,
    pub csize: u16,
    pub sso_pf_func: u16,
    pub npa_pf_func: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpipf_vf {
    pub vf_config: dpivf_config,
    pub setup_done: bool,
    pub this_vfid: u8,
}

// DPI device mailbox
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpi_mbox {
    pub work: work_struct,
// lock to serialize mbox requests
    pub lock: mutex,
    pub pf: *mut dpipf,
    pub pf_vf_data_reg: *mut u8 __iomem,
    pub vf_pf_data_reg: *mut u8 __iomem,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpipf {
    pub miscdev: miscdevice,
    pub reg_base: *mut void __iomem,
    pub pdev: *mut pci_dev,
    pub vf: [dpipf_vf; DPI_MAX_VFS],
// Mailbox to talk to VFs
    pub mbox: [*mut dpi_mbox; DPI_MAX_VFS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dpi_mbox_message {
    pub word_l: u64,
    pub word_h: u64,
}

#[no_mangle]
pub unsafe extern "C" fn dpi_reg_write(dpi: *mut dpipf, offset: u64, val: u64) {
    static inline void dpi_reg_write(struct dpipf *dpi, u64 offset, u64 val)
    {
    writeq(val, dpi.reg_base + offset);
    }
#[no_mangle]
pub unsafe extern "C" fn dpi_reg_read(dpi: *mut dpipf, offset: u64) -> u64 {
    static inline u64 dpi_reg_read(struct dpipf *dpi, u64 offset)
    {
    return readq(dpi.reg_base + offset);
    }
#[no_mangle]
unsafe extern "C" fn dpi_wqe_cs_offset(dpi: *mut dpipf, offset: u8) {
    static void dpi_wqe_cs_offset(struct dpipf *dpi, u8 offset)
    {
    u64 reg;
    reg = dpi_reg_read(dpi, DPI_DMA_CONTROL);
    reg &= ~DPI_DMA_CONTROL_WQECSDIS;
    reg |= DPI_DMA_CONTROL_ZBWCSEN | DPI_DMA_CONTROL_WQECSMODE1;
    reg |= DPI_DMA_CONTROL_WQECSOFF(offset);
    dpi_reg_write(dpi, DPI_DMA_CONTROL, reg);
    }
#[no_mangle]
unsafe extern "C" fn dpi_queue_init(dpi: *mut dpipf, dpivf: *mut dpipf_vf, vf: u8) -> c_int {
    static int dpi_queue_init(struct dpipf *dpi, struct dpipf_vf *dpivf, u8 vf)
    {
    let mut sso_pf_func: u16 = dpivf.vf_config.sso_pf_func;
    let mut npa_pf_func: u16 = dpivf.vf_config.npa_pf_func;
    let mut csize: u16 = dpivf.vf_config.csize;
    let mut aura: u32 = dpivf.vf_config.aura;
    unsigned long timeout;
    u64 reg;
    dpi_reg_write(dpi, DPI_DMAX_QRST(vf), DPI_DMA_QRST);
// Wait for a maximum of 3 sec
    timeout = jiffies + msecs_to_jiffies(3000);
    while (!time_after(jiffies, timeout)) {
    reg = dpi_reg_read(dpi, DPI_DMAX_QRST(vf));
    if (!(reg & DPI_DMA_QRST))
    break;
// Reset would take time for the request cache to drain
    usleep_range(500, 1000);
    }
    if (reg & DPI_DMA_QRST) {
    dev_err(&dpi.pdev.dev, "Queue reset failed\n");
    return -EBUSY;
    }
    dpi_reg_write(dpi, DPI_DMAX_IDS2(vf), 0);
    dpi_reg_write(dpi, DPI_DMAX_IDS(vf), 0);
    reg = DPI_DMA_IBUFF_CSIZE_CSIZE(csize) | DPI_DMA_IBUFF_CSIZE_NPA_FREE;
    dpi_reg_write(dpi, DPI_DMAX_IBUFF_CSIZE(vf), reg);
    reg = dpi_reg_read(dpi, DPI_DMAX_IDS2(vf));
    reg |= DPI_DMA_IDS2_INST_AURA(aura);
    dpi_reg_write(dpi, DPI_DMAX_IDS2(vf), reg);
    reg = dpi_reg_read(dpi, DPI_DMAX_IDS(vf));
    reg |= DPI_DMA_IDS_DMA_NPA_PF_FUNC(npa_pf_func);
    reg |= DPI_DMA_IDS_DMA_SSO_PF_FUNC(sso_pf_func);
    reg |= DPI_DMA_IDS_DMA_STRM(vf + 1);
    reg |= DPI_DMA_IDS_INST_STRM(vf + 1);
    dpi_reg_write(dpi, DPI_DMAX_IDS(vf), reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dpi_queue_fini(dpi: *mut dpipf, vf: u8) {
    static void dpi_queue_fini(struct dpipf *dpi, u8 vf)
    {
    dpi_reg_write(dpi, DPI_DMAX_QRST(vf), DPI_DMA_QRST);
// Reset IDS and IDS2 registers
    dpi_reg_write(dpi, DPI_DMAX_IDS2(vf), 0);
    dpi_reg_write(dpi, DPI_DMAX_IDS(vf), 0);
    }
#[no_mangle]
unsafe extern "C" fn dpi_mbox_intr_handler(irq: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t dpi_mbox_intr_handler(int irq, void *data)
    {
    struct dpipf *dpi = data;
    u64 reg;
    u32 vf;
    reg = dpi_reg_read(dpi, DPI_MBOX_VF_PF_INT);
    if (reg) {
    for (vf = 0; vf < pci_num_vf(dpi.pdev); vf++) {
    if (reg & BIT_ULL(vf))
    schedule_work(&dpi.mbox[vf].work);
    }
    dpi_reg_write(dpi, DPI_MBOX_VF_PF_INT, reg);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn queue_config(dpi: *mut dpipf, dpivf: *mut dpipf_vf, msg: *mut dpi_mbox_message) -> c_int {
    static int queue_config(struct dpipf *dpi, struct dpipf_vf *dpivf, struct dpi_mbox_message *msg)
    {
    let mut ret: c_int = 0;
    switch (DPI_MBOX_CMD(msg.word_l)) {
    case DPI_QUEUE_OPEN:
    case DPI_QUEUE_OPEN_V2:
    dpivf.vf_config.aura = DPI_MBOX_CBUF_AURA(msg.word_l);
    dpivf.vf_config.csize = DPI_MBOX_CMD(msg.word_l) == DPI_QUEUE_OPEN ?
    DPI_MBOX_CBUF_SIZE(msg.word_l) >> 3 :
    DPI_MBOX_CBUF_SIZE(msg.word_l);
    dpivf.vf_config.sso_pf_func = DPI_MBOX_SSO_PFFUNC(msg.word_l);
    dpivf.vf_config.npa_pf_func = DPI_MBOX_NPA_PFFUNC(msg.word_h);
    ret = dpi_queue_init(dpi, dpivf, DPI_MBOX_VFID(msg.word_l));
    if (!ret) {
    if (DPI_MBOX_WQES_COMPL(msg.word_h))
    dpi_wqe_cs_offset(dpi, DPI_MBOX_WQES_OFFSET(msg.word_h));
    dpivf.setup_done = true;
    }
    break;
    case DPI_QUEUE_CLOSE:
    memset(&dpivf.vf_config, 0, sizeof(struct dpivf_config));
    dpi_queue_fini(dpi, DPI_MBOX_VFID(msg.word_l));
    dpivf.setup_done = false;
    break;
    default:
    return -EINVAL;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dpi_pfvf_mbox_work(work: *mut work_struct) {
    static void dpi_pfvf_mbox_work(struct work_struct *work)
    {
    struct dpi_mbox *mbox = container_of(work, struct dpi_mbox, work);
    struct dpi_mbox_message msg;
    struct dpipf_vf *dpivf;
    struct dpipf *dpi;
    int vfid, ret;
    dpi = mbox.pf;
    memset(&msg, 0, sizeof(msg));
    mutex_lock(&mbox.lock);
    msg.word_l = readq(mbox.vf_pf_data_reg);
    if (msg.word_l == (u64)-1)
    goto exit;
    vfid = DPI_MBOX_VFID(msg.word_l);
    if (vfid >= pci_num_vf(dpi.pdev))
    goto exit;
    dpivf = &dpi.vf[vfid];
    msg.word_h = readq(mbox.pf_vf_data_reg);
    ret = queue_config(dpi, dpivf, &msg);
    if (ret < 0)
    writeq(DPI_MBOX_TYPE_RSP_NACK, mbox.pf_vf_data_reg);
    else
    writeq(DPI_MBOX_TYPE_RSP_ACK, mbox.pf_vf_data_reg);
    exit:
    mutex_unlock(&mbox.lock);
    }
// Setup registers for a PF mailbox
#[no_mangle]
unsafe extern "C" fn dpi_setup_mbox_regs(dpi: *mut dpipf, vf: c_int) {
    static void dpi_setup_mbox_regs(struct dpipf *dpi, int vf)
    {
    struct dpi_mbox *mbox = dpi.mbox[vf];
    mbox.pf_vf_data_reg = dpi.reg_base + DPI_MBOX_PF_VF_DATA0(vf);
    mbox.vf_pf_data_reg = dpi.reg_base + DPI_MBOX_PF_VF_DATA1(vf);
    }
#[no_mangle]
unsafe extern "C" fn dpi_pfvf_mbox_setup(dpi: *mut dpipf) -> c_int {
    static int dpi_pfvf_mbox_setup(struct dpipf *dpi)
    {
    int vf;
    for (vf = 0; vf < DPI_MAX_VFS; vf++) {
    dpi.mbox[vf] = devm_kzalloc(&dpi.pdev.dev, sizeof(*dpi.mbox[vf]), GFP_KERNEL);
    if (!dpi.mbox[vf])
    return -ENOMEM;
    mutex_init(&dpi.mbox[vf].lock);
    INIT_WORK(&dpi.mbox[vf].work, dpi_pfvf_mbox_work);
    dpi.mbox[vf].pf = dpi;
    dpi_setup_mbox_regs(dpi, vf);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dpi_pfvf_mbox_destroy(dpi: *mut dpipf) {
    static void dpi_pfvf_mbox_destroy(struct dpipf *dpi)
    {
    unsigned int vf;
    for (vf = 0; vf < DPI_MAX_VFS; vf++) {
    if (work_pending(&dpi.mbox[vf].work))
    cancel_work_sync(&dpi.mbox[vf].work);
    dpi.mbox[vf] = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn dpi_init(dpi: *mut dpipf) {
    static void dpi_init(struct dpipf *dpi)
    {
    unsigned int engine, port;
    u8 mrrs_val, mps_val;
    u64 reg;
    for (engine = 0; engine < DPI_MAX_ENGINES; engine++) {
    if (engine == 4 || engine == 5)
    reg = DPI_ENG_BUF_BLKS(16);
    else
    reg = DPI_ENG_BUF_BLKS(8);
    dpi_reg_write(dpi, DPI_ENGX_BUF(engine), reg);
    }
    reg = DPI_DMA_CONTROL_ZBWCSEN | DPI_DMA_CONTROL_PKT_EN | DPI_DMA_CONTROL_LDWB |
    DPI_DMA_CONTROL_O_MODE | DPI_DMA_CONTROL_DMA_ENB;
    dpi_reg_write(dpi, DPI_DMA_CONTROL, reg);
    dpi_reg_write(dpi, DPI_CTL, DPI_CTL_EN);
    mrrs_val = 2; /* 512B */
    mps_val = 1; /* 256B */
    for (port = 0; port < DPI_EBUS_MAX_PORTS; port++) {
    reg = dpi_reg_read(dpi, DPI_EBUS_PORTX_CFG(port));
    reg &= ~(DPI_EBUS_PORTX_CFG_MRRS(7) | DPI_EBUS_PORTX_CFG_MPS(7));
    reg |= DPI_EBUS_PORTX_CFG_MPS(mps_val) | DPI_EBUS_PORTX_CFG_MRRS(mrrs_val);
    dpi_reg_write(dpi, DPI_EBUS_PORTX_CFG(port), reg);
    }
    dpi_reg_write(dpi, DPI_WCTL_FIF_THR, DPI_WCTL_FIFO_THRESHOLD);
    }
#[no_mangle]
unsafe extern "C" fn dpi_fini(dpi: *mut dpipf) {
    static void dpi_fini(struct dpipf *dpi)
    {
    unsigned int engine;
    for (engine = 0; engine < DPI_MAX_ENGINES; engine++)
    dpi_reg_write(dpi, DPI_ENGX_BUF(engine), 0);
    dpi_reg_write(dpi, DPI_DMA_CONTROL, 0);
    dpi_reg_write(dpi, DPI_CTL, 0);
    }
#[no_mangle]
unsafe extern "C" fn dpi_free_irq_vectors(pdev: *mut c_void) {
    static void dpi_free_irq_vectors(void *pdev)
    {
    pci_free_irq_vectors((struct pci_dev *)pdev);
    }
#[no_mangle]
unsafe extern "C" fn dpi_irq_init(dpi: *mut dpipf) -> c_int {
    static int dpi_irq_init(struct dpipf *dpi)
    {
    struct pci_dev *pdev = dpi.pdev;
    struct device *dev = &pdev.dev;
    int i, ret;
// Clear all RAS interrupts
    dpi_reg_write(dpi, DPI_PF_RAS, DPI_PF_RAS_INT);
// Clear all RAS interrupt enable bits
    dpi_reg_write(dpi, DPI_PF_RAS_ENA_W1C, DPI_PF_RAS_INT);
    for (i = 0; i < DPI_MAX_REQQ_INT; i++) {
    dpi_reg_write(dpi, DPI_REQQX_INT(i), DPI_REQQ_INT);
    dpi_reg_write(dpi, DPI_REQQX_INT_ENA_W1C(i), DPI_REQQ_INT);
    }
    for (i = 0; i < DPI_MAX_CC_INT; i++) {
    dpi_reg_write(dpi, DPI_DMA_CCX_INT(i), DPI_DMA_CC_INT);
    dpi_reg_write(dpi, DPI_DMA_CCX_INT_ENA_W1C(i), DPI_DMA_CC_INT);
    }
    ret = pci_alloc_irq_vectors(pdev, DPI_MAX_IRQS, DPI_MAX_IRQS, PCI_IRQ_MSIX);
    if (ret != DPI_MAX_IRQS) {
    dev_err(dev, "DPI: Failed to alloc %d msix irqs\n", DPI_MAX_IRQS);
    return ret;
    }
    ret = devm_add_action_or_reset(dev, dpi_free_irq_vectors, pdev);
    if (ret) {
    dev_err(dev, "DPI: Failed to add irq free action\n");
    return ret;
    }
    ret = devm_request_irq(dev, pci_irq_vector(pdev, DPI_MBOX_PF_VF_INT_IDX),
    dpi_mbox_intr_handler, 0, "dpi-mbox", dpi);
    if (ret)
    return ret;
    dpi_reg_write(dpi, DPI_MBOX_VF_PF_INT_ENA_W1S, GENMASK_ULL(31, 0));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dpi_mps_mrrs_config(dpi: *mut dpipf, arg: *mut void __user) -> c_int {
    static int dpi_mps_mrrs_config(struct dpipf *dpi, void __user *arg)
    {
    struct dpi_mps_mrrs_cfg cfg;
    u8 mrrs_val, mps_val;
    u64 reg;
    if (copy_from_user(&cfg, arg, sizeof(struct dpi_mps_mrrs_cfg)))
    return -EFAULT;
    if (cfg.max_read_req_sz < DPI_EBUS_MRRS_MIN || cfg.max_read_req_sz > DPI_EBUS_MRRS_MAX ||
    !is_power_of_2(cfg.max_read_req_sz))
    return -EINVAL;
    if (cfg.max_payload_sz < DPI_EBUS_MPS_MIN || cfg.max_payload_sz > DPI_EBUS_MPS_MAX ||
    !is_power_of_2(cfg.max_payload_sz))
    return -EINVAL;
    if (cfg.port >= DPI_EBUS_MAX_PORTS)
    return -EINVAL;
// Make sure reserved fields are set to 0
    if (cfg.reserved)
    return -EINVAL;
    mrrs_val = fls(cfg.max_read_req_sz >> 8);
    mps_val = fls(cfg.max_payload_sz >> 8);
    reg = dpi_reg_read(dpi, DPI_EBUS_PORTX_CFG(cfg.port));
    reg &= ~(DPI_EBUS_PORTX_CFG_MRRS(0x7) | DPI_EBUS_PORTX_CFG_MPS(0x7));
    reg |= DPI_EBUS_PORTX_CFG_MPS(mps_val) | DPI_EBUS_PORTX_CFG_MRRS(mrrs_val);
    dpi_reg_write(dpi, DPI_EBUS_PORTX_CFG(cfg.port), reg);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dpi_engine_config(dpi: *mut dpipf, arg: *mut void __user) -> c_int {
    static int dpi_engine_config(struct dpipf *dpi, void __user *arg)
    {
    struct dpi_engine_cfg cfg;
    unsigned int engine;
    u8 *eng_buf;
    u64 reg;
    if (copy_from_user(&cfg, arg, sizeof(struct dpi_engine_cfg)))
    return -EFAULT;
// Make sure reserved fields are set to 0
    if (cfg.reserved)
    return -EINVAL;
    eng_buf = (u8 *)&cfg.fifo_mask;
    for (engine = 0; engine < DPI_MAX_ENGINES; engine++) {
    if (eng_buf[engine] > DPI_MAX_ENG_FIFO_SZ)
    return -EINVAL;
    dpi_reg_write(dpi, DPI_ENGX_BUF(engine), eng_buf[engine]);
    if (cfg.update_molr) {
    if (cfg.molr[engine] > DPI_MAX_ENG_MOLR)
    return -EINVAL;
    reg = DPI_DMA_ENG_EN_MOLR(cfg.molr[engine]);
    dpi_reg_write(dpi, DPI_DMA_ENGX_EN(engine), reg);
    } else {
// Make sure unused fields are set to 0
    if (cfg.molr[engine])
    return -EINVAL;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dpi_dev_ioctl(fptr: *mut file, cmd: c_uint, data: c_ulong) -> c_long {
    static long dpi_dev_ioctl(struct file *fptr, unsigned int cmd, unsigned long data)
    {
    void __user *arg = (void __user *)data;
    struct dpipf *dpi;
    int ret;
    dpi = container_of(fptr.private_data, struct dpipf, miscdev);
    switch (cmd) {
    case DPI_MPS_MRRS_CFG:
    ret = dpi_mps_mrrs_config(dpi, arg);
    break;
    case DPI_ENGINE_CFG:
    ret = dpi_engine_config(dpi, arg);
    break;
    default:
    ret = -ENOTTY;
    break;
    }
    return ret;
    }
    static const struct file_operations dpi_device_fops = {
    .owner = THIS_MODULE,
    .unlocked_ioctl = dpi_dev_ioctl,
    .compat_ioctl = compat_ptr_ioctl,
    };
#[no_mangle]
unsafe extern "C" fn dpi_probe(pdev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int dpi_probe(struct pci_dev *pdev, const struct pci_device_id *id)
    {
    struct device *dev = &pdev.dev;
    struct dpipf *dpi;
    int ret;
    dpi = devm_kzalloc(dev, sizeof(*dpi), GFP_KERNEL);
    if (!dpi)
    return -ENOMEM;
    dpi.pdev = pdev;
    ret = pcim_enable_device(pdev);
    if (ret) {
    dev_err(dev, "DPI: Failed to enable PCI device\n");
    return ret;
    }
    ret = pcim_iomap_regions(pdev, BIT(0) | BIT(4), KBUILD_MODNAME);
    if (ret) {
    dev_err(dev, "DPI: Failed to request MMIO region\n");
    return ret;
    }
    dpi.reg_base = pcim_iomap_table(pdev)[PCI_DPI_CFG_BAR];
// Initialize global PF registers
    dpi_init(dpi);
// Setup PF-VF mailbox
    ret = dpi_pfvf_mbox_setup(dpi);
    if (ret) {
    dev_err(dev, "DPI: Failed to setup pf-vf mbox\n");
    goto err_dpi_fini;
    }
// Register interrupts
    ret = dpi_irq_init(dpi);
    if (ret) {
    dev_err(dev, "DPI: Failed to initialize irq vectors\n");
    goto err_dpi_mbox_free;
    }
    pci_set_drvdata(pdev, dpi);
    dpi.miscdev.minor = MISC_DYNAMIC_MINOR;
    dpi.miscdev.name = KBUILD_MODNAME;
    dpi.miscdev.fops = &dpi_device_fops;
    dpi.miscdev.parent = dev;
    ret = misc_register(&dpi.miscdev);
    if (ret) {
    dev_err(dev, "DPI: Failed to register misc device\n");
    goto err_dpi_mbox_free;
    }
    return 0;
    err_dpi_mbox_free:
    dpi_pfvf_mbox_destroy(dpi);
    err_dpi_fini:
    dpi_fini(dpi);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn dpi_remove(pdev: *mut pci_dev) {
    static void dpi_remove(struct pci_dev *pdev)
    {
    struct dpipf *dpi = pci_get_drvdata(pdev);
    misc_deregister(&dpi.miscdev);
    pci_sriov_configure_simple(pdev, 0);
    dpi_pfvf_mbox_destroy(dpi);
    dpi_fini(dpi);
    pci_set_drvdata(pdev, core::ptr::null_mut());
    }
    static const struct pci_device_id dpi_id_table[] = {
    { PCI_DEVICE_SUB(PCI_VENDOR_ID_CAVIUM, PCI_DEVID_MRVL_CN10K_DPI_PF,
    PCI_VENDOR_ID_CAVIUM, PCI_SUBDEVID_MRVL_CN10K_DPI_PF) },
    { }  /* end of table */
    };
    static struct pci_driver dpi_driver = {
    .name = KBUILD_MODNAME,
    .id_table = dpi_id_table,
    .probe = dpi_probe,
    .remove = dpi_remove,
    .sriov_configure = pci_sriov_configure_simple,
    };
    module_pci_driver(dpi_driver);
    MODULE_DEVICE_TABLE(pci, dpi_id_table);
    MODULE_AUTHOR("Marvell.");
    MODULE_DESCRIPTION("Marvell Octeon CN10K DPI Driver");
    MODULE_LICENSE("GPL");
