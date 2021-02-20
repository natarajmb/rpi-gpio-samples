# RPI GPIO Samples

This is a sample project created to learn about Raspberry Pi GPIO using Rust embedded.
Project code is targeted for Raspberry Pi 2B+ but should work with later once as well. One of the takeaways if you 
have reached here is one way to set up Rust cross compile as authoring is done on macOS while target platform is armv7. 

## Assumptions
- You have working Raspberry Pi
- Have a working password-less login between your macOS and Raspberry Pi
- macOS has a working Docker installation

## How to run

The target for the binary application is Raspberry Pi and also it's easier to source the cross compile tooling on linux 
environment. Hence, the code compile happens in a container based environment and finally the built binary is copied 
to the target Raspberry Pi.  

    make start-env
    make build
    make copy-bin
    
## Troubleshooting tips

- Check if pin numbering you are using in code is correct, [pinout](https://pinout.xyz/#) site comes in handy
- Rule out hardware issues use [gpiotest](https://elinux.org/R-Pi_Troubleshooting#Testing) script to test 
  correctness of hardware. Script requires [pigpio](http://abyz.me.uk/rpi/pigpio/download.html) library. 
