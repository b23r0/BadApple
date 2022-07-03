# BadApple

BadaApple character animation for GUI (Rust)

![image]( https://github.com/b23r0/BadApple/blob/main/image/badapple.gif)

# Features

* GUI
* Sync Music
* File size only 11M
* Linux/Windows/Mac/BSD support

# Implementation

## Step1 

Convert video to character drawing using OpenCV

https://github.com/b23r0/BadApple/blob/main/step1/video2char.py

# Step2

Compress character picture files, greatly reduce file size

https://github.com/b23r0/BadApple/blob/main/step2/compress.py

# Step3

Use Rust egui to draw each frame of animation.

# Original Video

https://www.bilibili.com/video/BV1Wb41177hQ?spm_id_from=333.337.search-card.all.click