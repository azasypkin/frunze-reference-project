extern "C" {
#include "driver/uart.h"
#include "ets_sys.h"
#include "gpio.h"
#include "os_type.h"
#include "osapi.h"
#include "user_interface.h"
}

// ESP-12 modules have LED on GPIO2. Change to another GPIO
// for other boards.
static const int pin = 2;
static os_timer_t some_timer;

void some_timerfunc(void *arg) {
  // Do blinky stuff
  if (GPIO_REG_READ(GPIO_OUT_ADDRESS) & (1 << pin)) {
    // set gpio low
    gpio_output_set(0, (1 << pin), 0, 0);
  } else {
    // set gpio high
    gpio_output_set((1 << pin), 0, 0, 0);
  }

  os_printf("Hello From SDK 2.1 ");
}

extern "C" void ICACHE_FLASH_ATTR user_init() {
  // init gpio subsytem
  gpio_init();

  UART_SetBaudrate(0, 115200);
  // uart_init(BIT_RATE_115200, BIT_RATE_115200);
  // configure UART TXD to be GPIO1, set as output
  // PIN_FUNC_SELECT(PERIPHS_IO_MUX_U0TXD_U, FUNC_GPIO1);
  gpio_output_set(0, 0, (1 << pin), 0);

  // setup timer (500ms, repeating)
  os_timer_setfn(&some_timer, (os_timer_func_t *)some_timerfunc, NULL);
  os_timer_arm(&some_timer, 500, 1);
}

extern "C" uint32 ICACHE_FLASH_ATTR user_rf_cal_sector_set(void) {
  enum flash_size_map size_map = system_get_flash_size_map();
  uint32 rf_cal_sec = 0;

  switch (size_map) {
  case FLASH_SIZE_4M_MAP_256_256:
    rf_cal_sec = 128 - 5;
    break;

  case FLASH_SIZE_8M_MAP_512_512:
    rf_cal_sec = 256 - 5;
    break;

  case FLASH_SIZE_16M_MAP_512_512:
  case FLASH_SIZE_16M_MAP_1024_1024:
    rf_cal_sec = 512 - 5;
    break;

  case FLASH_SIZE_32M_MAP_512_512:
  case FLASH_SIZE_32M_MAP_1024_1024:
    rf_cal_sec = 1024 - 5;
    break;

  case FLASH_SIZE_64M_MAP_1024_1024:
    rf_cal_sec = 2048 - 5;
    break;
  case FLASH_SIZE_128M_MAP_1024_1024:
    rf_cal_sec = 4096 - 5;
    break;
  default:
    rf_cal_sec = 0;
    break;
  }

  return rf_cal_sec;
}