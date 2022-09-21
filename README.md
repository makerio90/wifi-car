# wificar
program for controlling rc cars over wifi, written in rust.
has no acutal use, just for fun. i an fairly new to the rust language
so expect fairly slow updates at first.
## usage
currently the only way to use this is through the `shell` crate, 
although a web interface will take its place soon.
1. start by compileing and running the shell crate `cargo run --bin shell`\
(this implies that you already have cargo installed)
2. you should now see a `>` prompt. if you do, continue to #3
3. run `set <driver>` along with the name of a valid driver. these options are:\
   3.1. `demo`\
   3.2. `skidSteer`
4. run `info` you should now see info about the driver you have selected
5. run `enable` to enable the driver
6. run `drive <speed> <steer>` do drive. `speed` and `steer` should be a float between -1 to 1
7. fun `disable` and then `quit` to quit

**NOTE**: i am not geranteed to update these md files as i update the project.
please be aware that this info may be out of date.
