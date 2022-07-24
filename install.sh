#!/usr/bin/env bash

cargo build --release
mv -u ./target/release/img_combiner .

printf "\e[1;96m \n[+] Removing artifacts...\n \e[0m"
cargo clean

printf "\e[1;96m \n[+] Now, you can run img_combiner like this\n \e[0m"
printf "\e[1;93m \t./img_combiner <filepath_of_first_image> <filepath_of_second_image> <filepath_of_output_image> \e[0m"
