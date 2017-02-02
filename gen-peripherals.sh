#!/bin/bash

set -e

main() {
    local svd=STM32F100xx.svd

    svd2rust -i $svd afio > src/afio.rs

    svd2rust -i $svd tim6 > src/btim.rs
    sed -i 's/\(pub struct \)Tim6/\1BTim/' src/btim.rs

    svd2rust -i $svd tim2 > src/gptim.rs
    sed -i 's/\(pub struct \)Tim2/\1GpTim/' src/gptim.rs

    svd2rust -i $svd gpioa > src/gpio.rs
    sed -i 's/\(pub struct Gpio\)a/\1/' src/gpio.rs

    svd2rust -i $svd rcc > src/rcc.rs

    svd2rust -i $svd usart1 > src/usart.rs
    sed -i 's/\(pub struct Usart\)1/\1/' src/usart.rs

    set +e
    rustfmt src/*.rs
    set -e

    xargo build --target thumbv7m-none-eabi
}

main
