#!/bin/bash

cargo cli user create --chat-id 413776157 --tg-id 413776157 --username KatasonovYP
cargo cli active create --user-id 0 --security-id MOEX --bought-price 10 --currency RUB --count 1
cargo cli notification create --portfolio-id 0 --active-id 0 --limit-upper 180 --limit-lower 160 --limit-type RUB
cargo cli notification create --portfolio-id 0 --active-id 0 --limit-upper 190 --limit-lower 150 --limit-type RUB
