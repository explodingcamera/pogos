// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
// static const MemMapEntry virt_memmap[] = {
//   [VIRT_DEBUG] =        {        0x0,         0x100 },
//   [VIRT_MROM] =         {     0x1000,        0xf000 },
//   [VIRT_TEST] =         {   0x100000,        0x1000 },
//   [VIRT_RTC] =          {   0x101000,        0x1000 },
//   [VIRT_CLINT] =        {  0x2000000,       0x10000 },
//   [VIRT_ACLINT_SSWI] =  {  0x2F00000,        0x4000 },
//   [VIRT_PCIE_PIO] =     {  0x3000000,       0x10000 },
//   [VIRT_PLATFORM_BUS] = {  0x4000000,     0x2000000 },
//   [VIRT_PLIC] =         {  0xc000000, VIRT_PLIC_SIZE(VIRT_CPUS_MAX * 2) },
//   [VIRT_APLIC_M] =      {  0xc000000, APLIC_SIZE(VIRT_CPUS_MAX) },
//   [VIRT_APLIC_S] =      {  0xd000000, APLIC_SIZE(VIRT_CPUS_MAX) },
//   [VIRT_UART0] =        { 0x10000000,         0x100 },
//   [VIRT_VIRTIO] =       { 0x10001000,        0x1000 },
//   [VIRT_FW_CFG] =       { 0x10100000,          0x18 },
//   [VIRT_FLASH] =        { 0x20000000,     0x4000000 },
//   [VIRT_IMSIC_M] =      { 0x24000000, VIRT_IMSIC_MAX_SIZE },
//   [VIRT_IMSIC_S] =      { 0x28000000, VIRT_IMSIC_MAX_SIZE },
//   [VIRT_PCIE_ECAM] =    { 0x30000000,    0x10000000 },
//   [VIRT_PCIE_MMIO] =    { 0x40000000,    0x40000000 },
//   [VIRT_DRAM] =         { 0x80000000,           0x0 },
// };

// Platform-Level Interrupt Controller (https://github.com/riscv/riscv-plic-spec)
pub const VIRT_CPUS_MAX: usize = 8;
pub const VIRT_PLIC_CONTEXT_BASE: usize = 0x200000;
pub const VIRT_PLIC_SIZE: usize = VIRT_PLIC_CONTEXT_BASE + (VIRT_CPUS_MAX * 0x1000);
pub const VIRT_PLIC: (usize, usize) = (0xc000000, VIRT_PLIC_SIZE);

pub const VIRT_TEST: (usize, usize) = (0x100000, 0x1000);
pub const VIRT_RTC: (usize, usize) = (0x101000, 0x1000); // real time clock
pub const VIRT_CLINT: (usize, usize) = (0x2000000, 0x10000); // core local interupt controller
pub const VIRT_UART0: (usize, usize) = (0x10000000, 0x100);

pub const MMIO_DEVICES: [(usize, usize); 5] =
    [VIRT_TEST, VIRT_RTC, VIRT_CLINT, VIRT_UART0, VIRT_PLIC];

pub const VIRT_VIRTIO: (usize, usize) = (0x10001000, 0x1000);
