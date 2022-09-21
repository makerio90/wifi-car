# shell
this is a subprogram of the man program `drivers`, used for testing perposes.
its usage is simple with only a few commands
```
 demo     :  run a demo sequence 
 disable  :  disables the driver 
 drive    :  set the Driver 
 enable   :  enables the driver 
 estop    :  estop the driver 
 help     :  Print this help 
 history  :  Print commands history or run a command from it 
 info     :  displays driver info 
 quit     :  Quit 
 set      :  set the driver 
```
## running
works just like any other rust crate. run `cargo run` localy or `cargo run --bin shell` if your in the parent directory.
## usage
to test a driver, simply run `set <driver>` along with the name of the driver you want (you can use `demo` to test). \
now try running `info`. you will see info about the current status of the driver.
run `enable` to enable the driver, and `drive <a> <s>` to drive.
`a` and `s` span from -1.0 -> 1.0, and are <u>A</u>ccelerate and <u>S</u>teer.
-1 accelrate is full reverse, 1 accelerate full throttle.
-1 steer is full left, 1 is right.
when you done, run `disable` and then `quit`
