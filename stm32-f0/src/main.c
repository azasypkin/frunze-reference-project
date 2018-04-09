#include <stm32f051x8.h>
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

// Note intervals in milliseconds.
#define QUARTER_DOT_NOTE 450
#define QUARTER_NOTE 300
#define EIGHTH_NOTE 150
#define PLL_MUL_X 3

volatile int EXTI0Flag = 0;

// Scale and note definitions
const unsigned int Scales[3][12] = {
    {523,  554,  587,  622,  659,  698,  740,  784,  831,  880,  932,  988},
    {1046, 1108, 1174, 1244, 1318, 1397, 1480, 1568, 1662, 1760, 1865, 1975},
    {2093, 2217, 2349, 2489, 2637, 2794, 2960, 3136, 3322, 3520, 3729, 3951}};

volatile uint32_t G_tickValue = 0;

/**
  * @brief  delay_ms  delay for some time in ms unit(accurate)
  * @param  n_ms is how many ms of time to delay
  * @retval None
  */
void delayMs(uint32_t n_ms) {
  // SysTick interrupt each 1000 Hz with HCLK equal to 32MHz
  // - 30 to compensate the overhead of this sub routine
  SysTick_Config(8000 * PLL_MUL_X - 30);
  // Enable the SysTick Counter

  G_tickValue = n_ms;
  while (G_tickValue == n_ms);

  // SysTick interrupt each 1000 Hz with HCLK equal to 32MHz
  SysTick_Config(8000 * PLL_MUL_X);
  while (G_tickValue != 0);
}

/**
  * @brief   Sound_Play
  * @param  Note to play, delay for note duration
  * @retval None
  */
void playNote(uint16_t note, uint16_t delay) {
  TIM1->ARR = (SystemCoreClock / note) - 1;
  delayMs(delay);
}

/**
  * @brief  Enables or disables the TIM peripheral Main Outputs.
  * @param  TIMx: where x can be 1, 15, 16 or 17 to select the TIMx peripheral.
  * @param  newState: new state of the TIM peripheral Main Outputs.
  *          This parameter can be: ENABLE or DISABLE.
  * @retval None
  */
void togglePWM(FunctionalState newState) {
  /* Check the parameters */
  if (newState != DISABLE) {
    /* Enable the TIM Main Output */
    TIM1->BDTR |= TIM_BDTR_MOE;
  } else {
    /* Disable the TIM Main Output */
    TIM1->BDTR &= (uint16_t) (~((uint16_t) TIM_BDTR_MOE));
  }
}

/**
  * @brief   Ode_to_Joy  The first several notes of famous Beethoven's tune
  * @param  None
  * @retval None
  */
void Ode_to_Joy(void) {
  togglePWM(ENABLE);

  playNote(Scales[0][7], QUARTER_NOTE);       // G
  playNote(Scales[0][7], QUARTER_NOTE);       // G
  playNote(Scales[0][8], QUARTER_NOTE);       // A
  playNote(Scales[0][10], QUARTER_NOTE);       // B
  playNote(Scales[0][10], QUARTER_NOTE);       // B
  playNote(Scales[0][8], QUARTER_NOTE);       // A
  playNote(Scales[0][7], QUARTER_NOTE);       // G
  playNote(Scales[0][5], QUARTER_NOTE);       // F
  playNote(Scales[0][3], QUARTER_NOTE);       // D#
  playNote(Scales[0][3], QUARTER_NOTE);       // E
  playNote(Scales[0][5], QUARTER_NOTE);       // F
  playNote(Scales[0][7], QUARTER_NOTE);       // G
  playNote(Scales[0][7], QUARTER_DOT_NOTE);   // G.
  playNote(Scales[0][5], EIGHTH_NOTE);        // F
  playNote(Scales[0][5], QUARTER_DOT_NOTE);   // F.

  togglePWM(DISABLE);
}

/**
  * @brief  PWM_Config  Configure PA8 as Timer for PWM control
  * @param  None
  * @retval None
  */
void configPWM(void) {
  uint16_t TimerPeriod = 0;
  uint16_t Channel1Pulse = 0;

  /* Enable GPIOA clock */
  RCC->AHBENR |= RCC_AHBENR_GPIOAEN;

  /* TIM1 clock enable */
  RCC->APB2ENR |= RCC_APB2ENR_TIM1EN;

  /* Configure PA8 pin as TIM1 */

  // GPIO_InitStructure.GPIO_Pin = GPIO_Pin_8 && GPIO_InitStructure.GPIO_Mode = GPIO_Mode_AF;
  GPIOA->MODER |= GPIO_MODER_MODER8_1;

  // GPIO_InitStructure.GPIO_PuPd = GPIO_PuPd_NOPULL;
  GPIOA->PUPDR &= ~GPIO_PUPDR_PUPDR8;

  // GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
  GPIOA->OSPEEDR |= GPIO_OSPEEDER_OSPEEDR8;

  // Connect TIM1 Channels to PA8 Alternate Function 2
  // GPIO_PinAFConfig(GPIOA, GPIO_PinSource8, GPIO_AF_2);
  GPIOA->AFR[1] &= ~GPIO_AFRH_AFSEL8;
  GPIOA->AFR[1] |= 0x2U << GPIO_AFRH_AFSEL8_Pos;

  /**********************************/
  /* Compute the value to be set in ARR register to generate signal frequency at 17.57 Khz */
  TimerPeriod = (SystemCoreClock / 17570) - 1;
  /* Compute CCR1 value to generate a duty cycle at 50% for channel 1 and 1N */
  Channel1Pulse = (uint16_t) (((uint32_t) 5 * (TimerPeriod - 1)) / 10);

  /* Time Base configuration */
  // TIM_TimeBaseStructure.TIM_Prescaler = 0;
  TIM1->PSC = 0;

  // TIM_TimeBaseStructure.TIM_CounterMode = TIM_CounterMode_Up;
  TIM1->CR1 &= ~TIM_CR1_DIR;

  // TIM_TimeBaseStructure.TIM_ClockDivision = 0;
  TIM1->CR1 &= ~TIM_CR1_CKD;

  // TIM_TimeBaseStructure.TIM_Period = TimerPeriod;
  TIM1->ARR = TimerPeriod;

  // TIM_TimeBaseStructure.TIM_RepetitionCounter = 0;
  TIM1->RCR = 0;

  // TIM_OCInitStructure.TIM_OCMode = TIM_OCMode_PWM2;
  TIM1->CCMR1 |= TIM_CCMR1_OC1M_0 | TIM_CCMR1_OC1M_1 | TIM_CCMR1_OC1M_2;

  // TIM_OCInitStructure.TIM_OutputState = TIM_OutputState_Enable;
  TIM1->CCER |= TIM_CCER_CC1E;

  // TIM_OCInitStructure.TIM_OutputNState = TIM_OutputNState_Enable;
  TIM1->CCER |= TIM_CCER_CC1NE;

  // TIM_OCInitStructure.TIM_Pulse = Channel1Pulse;
  TIM1->CCR1 = Channel1Pulse;

  // TIM_OCInitStructure.TIM_OCPolarity = TIM_OCPolarity_Low;
  TIM1->CCER |= TIM_CCER_CC1P;

  // TIM_OCInitStructure.TIM_OCNPolarity = TIM_OCNPolarity_High;
  TIM1->CCER &= ~TIM_CCER_CC1NP;
  // TIM_OCInitStructure.TIM_OCIdleState = TIM_OCIdleState_Set;
  TIM1->CR2 |= TIM_CR2_OIS1;

  // TIM_OCInitStructure.TIM_OCNIdleState = TIM_OCIdleState_Reset;
  TIM1->CR2 &= ~TIM_CR2_OIS1N;

  /* TIM1 counter enable */
  TIM1->CR1 |= TIM_CR1_CEN;
}

void configEXT() {
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
}

void configLED() {
  ENABLE_GPIO_CLOCK;

  LEDPORT->MODER |= GPIOMODER;
}

int main(void) {
  configEXT();

  configPWM();

  configLED();

  Ode_to_Joy();

  for (;;) {
  }

  return 0;
}

void SysTick_Handler(void)
{
  if(G_tickValue)
    G_tickValue--;
}

// See http://www.hertaville.com/external-interrupts-on-the-stm32f0.html.
void EXTI0_1_IRQHandler(void) {
  if ((EXTI->IMR & EXTI_IMR_MR0) && (EXTI->PR & EXTI_PR_PR0)) {

    EXTI0Flag = EXTI0Flag == 0 ? 1 : 0;
    LEDPORT->BSRR = EXTI0Flag == 1 ? GPIO_SETTER : GPIO_RESETTER;

    EXTI->PR |= EXTI_PR_PR0;
  }
}