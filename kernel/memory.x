MEMORY
{
  RAM : ORIGIN = 0x80200000, LENGTH = 16M
}

REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

/* _memory_start = ORIGIN(RAM); */

pog_memory_end = ORIGIN(RAM) + LENGTH(RAM);
pog_etext = ADDR(.text) + SIZEOF(.text);
pog_srodata = ADDR(.rodata);
pog_erodata = ADDR(.rodata) + SIZEOF(.rodata);

/* PROVIDE(_stext = ORIGIN(REGION_TEXT));
PROVIDE(_stack_start = ORIGIN(REGION_STACK) + LENGTH(REGION_STACK));
PROVIDE(_max_hart_id = 0);
PROVIDE(_hart_stack_size = 2K);
PROVIDE(_heap_size = 0); */