//! Automatically rewritten from C to Rust
//! Source: drivers/spi/spi-wpcm-fiu.c
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
// Copyright (C) 2022 Jonathan Neuschäfer

pub const FIU_CFG: c_uint = 0x00;
pub const FIU_BURST_BFG: c_uint = 0x01;
pub const FIU_RESP_CFG: c_uint = 0x02;
pub const FIU_CFBB_PROT: c_uint = 0x03;
pub const FIU_FWIN1_LOW: c_uint = 0x04;
pub const FIU_FWIN1_HIGH: c_uint = 0x06;
pub const FIU_FWIN2_LOW: c_uint = 0x08;
pub const FIU_FWIN2_HIGH: c_uint = 0x0a;
pub const FIU_FWIN3_LOW: c_uint = 0x0c;
pub const FIU_FWIN3_HIGH: c_uint = 0x0e;
pub const FIU_PROT_LOCK: c_uint = 0x10;
pub const FIU_PROT_CLEAR: c_uint = 0x11;
pub const FIU_SPI_FL_CFG: c_uint = 0x14;
pub const FIU_UMA_CODE: c_uint = 0x16;
pub const FIU_UMA_AB0: c_uint = 0x17;
pub const FIU_UMA_AB1: c_uint = 0x18;
pub const FIU_UMA_AB2: c_uint = 0x19;
pub const FIU_UMA_DB0: c_uint = 0x1a;
pub const FIU_UMA_DB1: c_uint = 0x1b;
pub const FIU_UMA_DB2: c_uint = 0x1c;
pub const FIU_UMA_DB3: c_uint = 0x1d;
pub const FIU_UMA_CTS: c_uint = 0x1e;
pub const FIU_UMA_ECTS: c_uint = 0x1f;
pub const FIU_BURST_CFG_R16: c_int = 3;

pub const SHM_FLASH_SIZE: c_uint = 0x02;

//
// I observed a typical wait time of 16 iterations for a UMA transfer to
// finish, so this should be a safe limit.
//
pub const UMA_WAIT_ITERATIONS: c_int = 100;
// The memory-mapped view of flash is 16 MiB long

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpcm_fiu_spi {
    pub dev: *mut device,
    pub clk: *mut clk,
    pub regs: *mut void __iomem,
    pub memory: *mut void __iomem,
    pub memory_size: usize,
    pub shm_regmap: *mut regmap,
}

#[no_mangle]
unsafe extern "C" fn wpcm_fiu_set_opcode(fiu: *mut wpcm_fiu_spi, opcode: u8) {
    static void wpcm_fiu_set_opcode(struct wpcm_fiu_spi *fiu, u8 opcode)
    {
    writeb(opcode, fiu.regs + FIU_UMA_CODE);
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_set_addr(fiu: *mut wpcm_fiu_spi, addr: u32) {
    static void wpcm_fiu_set_addr(struct wpcm_fiu_spi *fiu, u32 addr)
    {
    writeb((addr >>  0) & 0xff, fiu.regs + FIU_UMA_AB0);
    writeb((addr >>  8) & 0xff, fiu.regs + FIU_UMA_AB1);
    writeb((addr >> 16) & 0xff, fiu.regs + FIU_UMA_AB2);
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_set_data(fiu: *mut wpcm_fiu_spi, data: *const u8, nbytes: c_uint) {
    static void wpcm_fiu_set_data(struct wpcm_fiu_spi *fiu, const u8 *data, unsigned int nbytes)
    {
    int i;
    for (i = 0; i < nbytes; i++)
    writeb(data[i], fiu.regs + FIU_UMA_DB0 + i);
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_get_data(fiu: *mut wpcm_fiu_spi, data: *mut u8, nbytes: c_uint) {
    static void wpcm_fiu_get_data(struct wpcm_fiu_spi *fiu, u8 *data, unsigned int nbytes)
    {
    int i;
    for (i = 0; i < nbytes; i++)
    data[i] = readb(fiu.regs + FIU_UMA_DB0 + i);
    }
//
// Perform a UMA (User Mode Access) operation, i.e. a software-controlled SPI transfer.
//
    static int wpcm_fiu_do_uma(struct wpcm_fiu_spi *fiu, unsigned int cs,
    bool use_addr, bool write, int data_bytes)
    {
    let mut i: c_int = 0;
    let mut cts: u8 = FIU_UMA_CTS_EXEC_DONE | FIU_UMA_CTS_CS(cs);
    if (use_addr)
    cts |= FIU_UMA_CTS_A_SIZE;
    if (write)
    cts |= FIU_UMA_CTS_WR;
    cts |= FIU_UMA_CTS_D_SIZE(data_bytes);
    writeb(cts, fiu.regs + FIU_UMA_CTS);
    for (i = 0; i < UMA_WAIT_ITERATIONS; i++)
    if (!(readb(fiu.regs + FIU_UMA_CTS) & FIU_UMA_CTS_EXEC_DONE))
    return 0;
    dev_info(fiu.dev, "UMA transfer has not finished in %d iterations\n", UMA_WAIT_ITERATIONS);
    return -EIO;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_ects_assert(fiu: *mut wpcm_fiu_spi, cs: c_uint) {
    static void wpcm_fiu_ects_assert(struct wpcm_fiu_spi *fiu, unsigned int cs)
    {
    let mut ects: u8 = readb(fiu.regs + FIU_UMA_ECTS);
    ects &= ~BIT(cs);
    writeb(ects, fiu.regs + FIU_UMA_ECTS);
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_ects_deassert(fiu: *mut wpcm_fiu_spi, cs: c_uint) {
    static void wpcm_fiu_ects_deassert(struct wpcm_fiu_spi *fiu, unsigned int cs)
    {
    let mut ects: u8 = readb(fiu.regs + FIU_UMA_ECTS);
    ects |= BIT(cs);
    writeb(ects, fiu.regs + FIU_UMA_ECTS);
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wpcm_fiu_op_shape {
    pub op): *const *const bool (match)(struct spi_mem_op,
    pub op): *const *const *const int (exec)(struct spi_mem mem, struct spi_mem_op,
}

#[no_mangle]
unsafe extern "C" fn wpcm_fiu_normal_match(op: *const spi_mem_op) -> bool {
    static bool wpcm_fiu_normal_match(const struct spi_mem_op *op)
    {
// Opcode 0x0b (FAST READ) is treated differently in hardware
    if (op.cmd.opcode == 0x0b)
    return false;
    return (op.addr.nbytes == 0 || op.addr.nbytes == 3) &&
    op.dummy.nbytes == 0 && op.data.nbytes <= 4;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_normal_exec(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int wpcm_fiu_normal_exec(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct wpcm_fiu_spi *fiu = spi_controller_get_devdata(mem.spi.controller);
    int ret;
    wpcm_fiu_set_opcode(fiu, op.cmd.opcode);
    wpcm_fiu_set_addr(fiu, op.addr.val);
    if (op.data.dir == SPI_MEM_DATA_OUT)
    wpcm_fiu_set_data(fiu, op.data.buf.out, op.data.nbytes);
    ret = wpcm_fiu_do_uma(fiu, spi_get_chipselect(mem.spi, 0), op.addr.nbytes == 3,
    op.data.dir == SPI_MEM_DATA_OUT, op.data.nbytes);
    if (op.data.dir == SPI_MEM_DATA_IN)
    wpcm_fiu_get_data(fiu, op.data.buf.in, op.data.nbytes);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_fast_read_match(op: *const spi_mem_op) -> bool {
    static bool wpcm_fiu_fast_read_match(const struct spi_mem_op *op)
    {
    return op.cmd.opcode == 0x0b && op.addr.nbytes == 3 &&
    op.dummy.nbytes == 1 &&
    op.data.nbytes >= 1 && op.data.nbytes <= 4 &&
    op.data.dir == SPI_MEM_DATA_IN;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_fast_read_exec(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int wpcm_fiu_fast_read_exec(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    return -EINVAL;
    }
//
// 4-byte addressing.
//
// Flash view:  [ C  A  A  A   A     D  D  D  D]
// bytes:        13 aa bb cc  dd -> 5a a5 f0 0f
// FIU's view:  [ C  A  A  A][ C     D  D  D  D]
// FIU mode:    [ read/write][      read       ]
//
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_4ba_match(op: *const spi_mem_op) -> bool {
    static bool wpcm_fiu_4ba_match(const struct spi_mem_op *op)
    {
    return op.addr.nbytes == 4 && op.dummy.nbytes == 0 && op.data.nbytes <= 4;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_4ba_exec(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int wpcm_fiu_4ba_exec(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct wpcm_fiu_spi *fiu = spi_controller_get_devdata(mem.spi.controller);
    let mut cs: c_int = spi_get_chipselect(mem.spi, 0);
    wpcm_fiu_ects_assert(fiu, cs);
    wpcm_fiu_set_opcode(fiu, op.cmd.opcode);
    wpcm_fiu_set_addr(fiu, op.addr.val >> 8);
    wpcm_fiu_do_uma(fiu, cs, true, false, 0);
    wpcm_fiu_set_opcode(fiu, op.addr.val & 0xff);
    wpcm_fiu_set_addr(fiu, 0);
    if (op.data.dir == SPI_MEM_DATA_OUT)
    wpcm_fiu_set_data(fiu, op.data.buf.out, op.data.nbytes);
    wpcm_fiu_do_uma(fiu, cs, false, op.data.dir == SPI_MEM_DATA_OUT, op.data.nbytes);
    wpcm_fiu_ects_deassert(fiu, cs);
    if (op.data.dir == SPI_MEM_DATA_IN)
    wpcm_fiu_get_data(fiu, op.data.buf.in, op.data.nbytes);
    return 0;
    }
//
// RDID (Read Identification) needs special handling because Linux expects to
// be able to read 6 ID bytes and FIU can only read up to 4 at once.
//
// We're lucky in this case, because executing the RDID instruction twice will
// result in the same result.
//
// What we do is as follows (C: write command/opcode byte, D: read data byte,
// A: write address byte):
//
// 1. C D D D
// 2. C A A A D D D
//
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_rdid_match(op: *const spi_mem_op) -> bool {
    static bool wpcm_fiu_rdid_match(const struct spi_mem_op *op)
    {
    return op.cmd.opcode == 0x9f && op.addr.nbytes == 0 &&
    op.dummy.nbytes == 0 && op.data.nbytes == 6 &&
    op.data.dir == SPI_MEM_DATA_IN;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_rdid_exec(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int wpcm_fiu_rdid_exec(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct wpcm_fiu_spi *fiu = spi_controller_get_devdata(mem.spi.controller);
    let mut cs: c_int = spi_get_chipselect(mem.spi, 0);
// First transfer
    wpcm_fiu_set_opcode(fiu, op.cmd.opcode);
    wpcm_fiu_set_addr(fiu, 0);
    wpcm_fiu_do_uma(fiu, cs, false, false, 3);
    wpcm_fiu_get_data(fiu, op.data.buf.in, 3);
// Second transfer
    wpcm_fiu_set_opcode(fiu, op.cmd.opcode);
    wpcm_fiu_set_addr(fiu, 0);
    wpcm_fiu_do_uma(fiu, cs, true, false, 3);
    wpcm_fiu_get_data(fiu, op.data.buf.in + 3, 3);
    return 0;
    }
//
// With some dummy bytes.
//
// C A A A  X*  X D D D D
// [C A A A  D*][C D D D D]
//
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_dummy_match(op: *const spi_mem_op) -> bool {
    static bool wpcm_fiu_dummy_match(const struct spi_mem_op *op)
    {
// Opcode 0x0b (FAST READ) is treated differently in hardware
    if (op.cmd.opcode == 0x0b)
    return false;
    return (op.addr.nbytes == 0 || op.addr.nbytes == 3) &&
    op.dummy.nbytes >= 1 && op.dummy.nbytes <= 5 &&
    op.data.nbytes <= 4;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_dummy_exec(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int wpcm_fiu_dummy_exec(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct wpcm_fiu_spi *fiu = spi_controller_get_devdata(mem.spi.controller);
    let mut cs: c_int = spi_get_chipselect(mem.spi, 0);
    wpcm_fiu_ects_assert(fiu, cs);
// First transfer
    wpcm_fiu_set_opcode(fiu, op.cmd.opcode);
    wpcm_fiu_set_addr(fiu, op.addr.val);
    wpcm_fiu_do_uma(fiu, cs, op.addr.nbytes != 0, true, op.dummy.nbytes - 1);
// Second transfer
    wpcm_fiu_set_opcode(fiu, 0);
    wpcm_fiu_set_addr(fiu, 0);
    wpcm_fiu_do_uma(fiu, cs, false, false, op.data.nbytes);
    wpcm_fiu_get_data(fiu, op.data.buf.in, op.data.nbytes);
    wpcm_fiu_ects_deassert(fiu, cs);
    return 0;
    }
    static const struct wpcm_fiu_op_shape wpcm_fiu_op_shapes[] = {
    { .match = wpcm_fiu_normal_match, .exec = wpcm_fiu_normal_exec },
    { .match = wpcm_fiu_fast_read_match, .exec = wpcm_fiu_fast_read_exec },
    { .match = wpcm_fiu_4ba_match, .exec = wpcm_fiu_4ba_exec },
    { .match = wpcm_fiu_rdid_match, .exec = wpcm_fiu_rdid_exec },
    { .match = wpcm_fiu_dummy_match, .exec = wpcm_fiu_dummy_exec },
    };
    static const struct wpcm_fiu_op_shape *wpcm_fiu_find_op_shape(const struct spi_mem_op *op)
    {
    size_t i;
    for (i = 0; i < ARRAY_SIZE(wpcm_fiu_op_shapes); i++) {
    const struct wpcm_fiu_op_shape *shape = &wpcm_fiu_op_shapes[i];
    if (shape.match(op))
    return shape;
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_supports_op(mem: *mut spi_mem, op: *const spi_mem_op) -> bool {
    static bool wpcm_fiu_supports_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    if (!spi_mem_default_supports_op(mem, op))
    return false;
    if (op.cmd.dtr || op.addr.dtr || op.dummy.dtr || op.data.dtr)
    return false;
    if (op.cmd.buswidth > 1 || op.addr.buswidth > 1 ||
    op.dummy.buswidth > 1 || op.data.buswidth > 1)
    return false;
    return wpcm_fiu_find_op_shape(op) != core::ptr::null_mut();
    }
//
// In order to ensure the integrity of SPI transfers performed via UMA,
// temporarily disable (stall) memory accesses coming from the host CPU.
//
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_stall_host(fiu: *mut wpcm_fiu_spi, stall: bool) {
    static void wpcm_fiu_stall_host(struct wpcm_fiu_spi *fiu, bool stall)
    {
    if (fiu.shm_regmap) {
    int res = regmap_update_bits(fiu.shm_regmap, SHM_FLASH_SIZE,
    SHM_FLASH_SIZE_STALL_HOST,
    stall ? SHM_FLASH_SIZE_STALL_HOST : 0);
    if (res)
    dev_warn(fiu.dev, "Failed to (un)stall host memory accesses: %d\n", res);
    }
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_exec_op(mem: *mut spi_mem, op: *const spi_mem_op) -> c_int {
    static int wpcm_fiu_exec_op(struct spi_mem *mem, const struct spi_mem_op *op)
    {
    struct wpcm_fiu_spi *fiu = spi_controller_get_devdata(mem.spi.controller);
    const struct wpcm_fiu_op_shape *shape = wpcm_fiu_find_op_shape(op);
    wpcm_fiu_stall_host(fiu, true);
    if (shape)
    return shape.exec(mem, op);
    wpcm_fiu_stall_host(fiu, false);
    return -EOPNOTSUPP;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_adjust_op_size(mem: *mut spi_mem, op: *mut spi_mem_op) -> c_int {
    static int wpcm_fiu_adjust_op_size(struct spi_mem *mem, struct spi_mem_op *op)
    {
    if (op.data.nbytes > 4)
    op.data.nbytes = 4;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_dirmap_create(desc: *mut spi_mem_dirmap_desc) -> c_int {
    static int wpcm_fiu_dirmap_create(struct spi_mem_dirmap_desc *desc)
    {
    struct wpcm_fiu_spi *fiu = spi_controller_get_devdata(desc.mem.spi.controller);
    let mut cs: c_int = spi_get_chipselect(desc.mem.spi, 0);
    if (desc.info.op_tmpl.data.dir != SPI_MEM_DATA_IN)
    return -EOPNOTSUPP;
//
// Unfortunately, FIU only supports a 16 MiB direct mapping window (per
// attached flash chip), but the SPI MEM core doesn't support partial
// direct mappings. This means that we can't support direct mapping on
// flashes that are bigger than 16 MiB.
//
    if (desc.info.offset + desc.info.length > MAX_MEMORY_SIZE_PER_CS)
    return -EINVAL;
// Don't read past the memory window
    if (cs * MAX_MEMORY_SIZE_PER_CS + desc.info.offset + desc.info.length > fiu.memory_size)
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_direct_read(desc: *mut spi_mem_dirmap_desc, offs: u64, len: usize, buf: *mut c_void) -> isize {
    static ssize_t wpcm_fiu_direct_read(struct spi_mem_dirmap_desc *desc, u64 offs, size_t len, void *buf)
    {
    struct wpcm_fiu_spi *fiu = spi_controller_get_devdata(desc.mem.spi.controller);
    let mut cs: c_int = spi_get_chipselect(desc.mem.spi, 0);
    if (offs >= MAX_MEMORY_SIZE_PER_CS)
    return -ENOTSUPP;
    offs += cs * MAX_MEMORY_SIZE_PER_CS;
    if (!fiu.memory || offs >= fiu.memory_size)
    return -ENOTSUPP;
    len = min_t(size_t, len, fiu.memory_size - offs);
    memcpy_fromio(buf, fiu.memory + offs, len);
    return len;
    }
    static const struct spi_controller_mem_ops wpcm_fiu_mem_ops = {
    .adjust_op_size = wpcm_fiu_adjust_op_size,
    .supports_op = wpcm_fiu_supports_op,
    .exec_op = wpcm_fiu_exec_op,
    .dirmap_create = wpcm_fiu_dirmap_create,
    .dirmap_read = wpcm_fiu_direct_read,
    };
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_hw_init(fiu: *mut wpcm_fiu_spi) {
    static void wpcm_fiu_hw_init(struct wpcm_fiu_spi *fiu)
    {
// Configure memory-mapped flash access
    writeb(FIU_BURST_CFG_R16, fiu.regs + FIU_BURST_BFG);
    writeb(MAX_MEMORY_SIZE_TOTAL / (512 << 10), fiu.regs + FIU_CFG);
    writeb(MAX_MEMORY_SIZE_PER_CS / (512 << 10) | BIT(6), fiu.regs + FIU_SPI_FL_CFG);
// Deassert all manually asserted chip selects
    writeb(0x0f, fiu.regs + FIU_UMA_ECTS);
    }
#[no_mangle]
unsafe extern "C" fn wpcm_fiu_probe(pdev: *mut platform_device) -> c_int {
    static int wpcm_fiu_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct spi_controller *ctrl;
    struct wpcm_fiu_spi *fiu;
    struct resource *res;
    ctrl = devm_spi_alloc_host(dev, sizeof(*fiu));
    if (!ctrl)
    return -ENOMEM;
    fiu = spi_controller_get_devdata(ctrl);
    fiu.dev = dev;
    fiu.regs = devm_platform_ioremap_resource_byname(pdev, "control");
    if (IS_ERR(fiu.regs))
    return dev_err_probe(dev, PTR_ERR(fiu.regs),
    "Failed to map registers\n");
    fiu.clk = devm_clk_get_enabled(dev, core::ptr::null_mut());
    if (IS_ERR(fiu.clk))
    return PTR_ERR(fiu.clk);
    res = platform_get_resource_byname(pdev, IORESOURCE_MEM, "memory");
    fiu.memory = devm_ioremap_resource(dev, res);
    if (IS_ERR(fiu.memory))
    return dev_err_probe(dev, PTR_ERR(fiu.memory),
    "Failed to map flash memory window\n");
    fiu.memory_size = min_t(size_t, resource_size(res), MAX_MEMORY_SIZE_TOTAL);
    fiu.shm_regmap = syscon_regmap_lookup_by_phandle_optional(dev.of_node, "nuvoton,shm");
    wpcm_fiu_hw_init(fiu);
    ctrl.bus_num = -1;
    ctrl.mem_ops = &wpcm_fiu_mem_ops;
    ctrl.num_chipselect = 4;
//
// The FIU doesn't include a clock divider, the clock is entirely
// determined by the AHB3 bus clock.
//
    ctrl.min_speed_hz = clk_get_rate(fiu.clk);
    ctrl.max_speed_hz = clk_get_rate(fiu.clk);
    return devm_spi_register_controller(dev, ctrl);
    }
    static const struct of_device_id wpcm_fiu_dt_ids[] = {
    { .compatible = "nuvoton,wpcm450-fiu", },
    { }
    };
    MODULE_DEVICE_TABLE(of, wpcm_fiu_dt_ids);
    static struct platform_driver wpcm_fiu_driver = {
    .driver = {
    .name	= "wpcm450-fiu",
    .bus	= &platform_bus_type,
    .of_match_table = wpcm_fiu_dt_ids,
    },
    .probe      = wpcm_fiu_probe,
    };
    module_platform_driver(wpcm_fiu_driver);
    MODULE_DESCRIPTION("Nuvoton WPCM450 FIU SPI controller driver");
    MODULE_AUTHOR("Jonathan Neuschäfer <j.neuschaefer@gmx.net>");
    MODULE_LICENSE("GPL");
