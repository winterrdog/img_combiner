# img_combiner
A toy project that combines two images to form one image. I developed it in order to learn more about writing code in Rust.

# 📚 PREREQUISITES
* Target operating system: Microsoft Windows, A Unix-like OS e.g. MacOS, GNU/Linux, FreeBSD, OpenBSD, GhostBSD.
* **Rust** and **Cargo** should be installed. _In case they're not installed on your system, you can follow these [instructions](https://www.rust-lang.org/tools/install) on how to do that._
* Some commandline-Fu( _just a little bit_ ).

# ⚙👷 HOW IT WORKS
* You provide the program with 2 images and it does the image processing for you in order to produce a single image.
* It only has support for [these](https://github.com/image-rs/image/blob/master/README.md#supported-image-formats) image formats.
* For example, I made a sample run to combine images,  `sample_imgs/kid_smile.jpg` and `sample_imgs/woman.jpeg` , to produce `sample_imgs/out_foto.jpg` which looks like so:

![output_image](https://github.com/winterrdog/img_combiner/blob/main/sample_imgs/out_foto.jpg)

# 🔧🔨 USAGE 
1. Clone this repository( You can also just download it. )

    ```sh
    git clone https://github.com/winterrdog/img_combiner.git
    ```

2. You can install this program like so:

    - using `install_nix.sh`:
        - Open the terminal and paste this:
            ```sh
            cd img_combiner
            bash install_nix.sh
            ```
    
    - using `install_win` for Windows users:
        - Open the PowerShell console and paste this:
            ```powershell
            cd .\img_combiner
            .\install_win.ps1
            ```
    
# 📝 NOTES
* `img_combiner` is GPLv3 licensed, feel free to contribute something to the project even if it's a typo 😊, or take it a step further by forking and extending it.
