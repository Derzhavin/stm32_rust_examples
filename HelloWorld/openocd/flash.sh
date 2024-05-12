openocd -f $1 -c "program ../target/thumbv7em-none-eabihf/debug/HelloWorld verify reset exit"
