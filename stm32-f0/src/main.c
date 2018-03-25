#include "stm32f0xx.h"

#ifdef STM32F051x8
#define LEDPORT (GPIOC)
#define ENABLE_GPIO_CLOCK (RCC->AHBENR |= RCC_AHBENR_GPIOCEN)
// Set GPIOx_MODER (MODER8, 16 and 17 bits) for PC8 to 01 (general purpose output mode)
#define GPIOMODER (GPIO_MODER_MODER8_0)
#define GPIO_SETTER (GPIO_BSRR_BS_8)
#define GPIO_RESETTER (GPIO_BSRR_BR_8)
#elif STM32F042x6
#define LEDPORT (GPIOA)
#define ENABLE_GPIO_CLOCK (RCC->AHBENR |= RCC_AHBENR_GPIOAEN)
// Set GPIOx_MODER (MODER4, 8 and 9 bits) for PA4 to 01 (general purpose output mode)
#define GPIOMODER (GPIO_MODER_MODER4_0)
#define GPIO_SETTER (GPIO_BSRR_BS_4)
#define GPIO_RESETTER (GPIO_BSRR_BR_4)
#endif

void ms_delay(int ms)
{
  while (ms-- > 0) {
    volatile int x=500;
    while (x-- > 0)
        __asm("nop");
  }
}

int main(void)
{
  ENABLE_GPIO_CLOCK;

  LEDPORT->MODER |= GPIOMODER;

  for (;;) {
    LEDPORT->BSRR = GPIO_SETTER;
    ms_delay(1000);
    LEDPORT->BSRR = GPIO_RESETTER;
    ms_delay(1000);
  }

  return 0;
}