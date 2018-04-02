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

volatile int EXTI0Flag = 0;

int main(void) {
  // Reset EXTI0 to 0x0000
  SYSCFG->EXTICR[0] &= ~SYSCFG_EXTICR1_EXTI0;
  // Enable for PA0
  SYSCFG->EXTICR[0] |= SYSCFG_EXTICR1_EXTI0_PA;

  // Configure PA0 to trigger an interrupt event on the EXTI0 line on a rising edge.
  EXTI->RTSR |= EXTI_RTSR_TR0;

  // Unmask the external interrupt line EXTI0 by setting the bit corresponding to the EXTI0 "bit 0" in the EXT_IMR register.
  EXTI->IMR |= EXTI_IMR_MR0;

  // Set priority for the `EXTI0` line to `1`.
  NVIC_SetPriority(EXTI0_1_IRQn, 1);

  // Enable the interrupt in the NVIC.
  NVIC_EnableIRQ(EXTI0_1_IRQn);

  ENABLE_GPIO_CLOCK;

  LEDPORT->MODER |= GPIOMODER;

  for (;;) {
  }

  return 0;
}

// See http://www.hertaville.com/external-interrupts-on-the-stm32f0.html.
void EXTI0_1_IRQHandler(void) {
  if ((EXTI->IMR & EXTI_IMR_MR0) && (EXTI->PR & EXTI_PR_PR0)) {
    while (GPIOA->IDR & GPIO_IDR_0) {}

    EXTI0Flag = EXTI0Flag == 0 ? 1 : 0;
    LEDPORT->BSRR = EXTI0Flag == 1 ? GPIO_SETTER : GPIO_RESETTER;

    EXTI->PR |= EXTI_PR_PR0;
  }
}