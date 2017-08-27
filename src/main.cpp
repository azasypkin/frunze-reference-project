extern "C" {
#include "ets_sys.h"
#include "gpio.h"
#include "os_type.h"
#include "osapi.h"
#include <uart.h>

void ets_timer_setfn(volatile ETSTimer *ptimer, ETSTimerFunc *pfunction,
                     void *parg);
void ets_timer_arm_new(volatile ETSTimer *ptimer, uint32_t milliseconds,
                       bool repeat_flag, bool);
void os_printf_plus(const char *s);
}

// ESP-12 modules have LED on GPIO2. Change to another GPIO
// for other boards.
static const int pin = 2;
static volatile os_timer_t some_timer;

void some_timerfunc(void *arg) {
  // Do blinky stuff
  if (GPIO_REG_READ(GPIO_OUT_ADDRESS) & (1 << pin)) {
    // set gpio low
    gpio_output_set(0, (1 << pin), 0, 0);
  } else {
    // set gpio high
    gpio_output_set((1 << pin), 0, 0, 0);
  }

  os_printf("Hello");
}

extern "C" void ICACHE_FLASH_ATTR user_init() {
  // init gpio subsytem
  gpio_init();

  // UART_SetBaudrate(0, UART_CLK_FREQ / 115200);
  uart_init(BIT_RATE_115200, BIT_RATE_115200);
  // configure UART TXD to be GPIO1, set as output
  // PIN_FUNC_SELECT(PERIPHS_IO_MUX_U0TXD_U, FUNC_GPIO1);
  gpio_output_set(0, 0, (1 << pin), 0);

  // setup timer (500ms, repeating)
  os_timer_setfn(&some_timer, (os_timer_func_t *)some_timerfunc, NULL);
  os_timer_arm(&some_timer, 500, 1);
}