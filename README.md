# linux_raw_input_rs

[![](http://meritbadge.herokuapp.com/linux_raw_input_rs)](https://crates.io/crates/linux_raw_input_rs)

Simple library for reading keyboard input directly from input events in linux systems.

```
linux_raw_input_rs = "0.1.2"
```

Example of code using it:
```
let device_path : String = get_input_devices().iter().nth(0).expect("Problem with iterator").to_string();
    let mut reader = InputReader::new(device_path);
    loop {
        let input = reader.current_state();
        if input.is_key_event(){
            println!("Key {:?} now has state {:?}", input.get_key(), input.event_type());
        }
    }
```
Root access is required for program to run