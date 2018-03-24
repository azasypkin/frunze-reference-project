#ifdef STM32F0
#include "stm32f0xx.h"
#define LEDPORT (GPIOC)
#define LED1 (13)
#define ENABLE_GPIO_CLOCK (RCC->AHBENR |= RCC_AHBENR_GPIOCEN)
#define GPIOMODER (GPIO_CRH_MODE13_0)
#endif

void ms_delay(int ms)
{
  while (ms-- > 0) {
    volatile int x=500;
    while (x-- > 0)
        __asm("nop");
  }
}

//Alternates blue and green LEDs quickly
int main(void)
{
  ENABLE_GPIO_CLOCK; 		 					// enable the clock to GPIO
  GPIOC->MODER |= (1 << 16);

  for (;;) {
    GPIOC->BSRR = (1 << 8);
    ms_delay(100);
    GPIOC->BRR = (1 << 8);
    ms_delay(100);
  }

  return 0;
}