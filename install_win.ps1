cargo build --release
Move-Item -Path ./target/release/img_combiner -Destination .

Write-Output "`n[+] Removing artifacts...`n"
cargo clean

printf "`n[+] Now, you can run img_combiner like this:`n"
printf "`t./img_combiner <filepath_of_first_image> <filepath_of_second_image> <filepath_of_output_image> `n"
