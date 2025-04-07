use embedded_hal::timer::CountDown;

pub fn debounce(input: impl Fn() -> bool, time: impl Fn() -> u32, ms: u32) -> bool {
    let mut last_change = time();
    let mut last_state =  input();

    while time() - last_change < ms {
        let current_state = input();

        if last_state != current_state {
            last_state = current_state;
            last_change = time()
        }
    }

    last_state
}

pub fn debounce_wait(input: impl Fn() -> bool, wait: impl Fn() -> bool) -> bool {
    let mut last_state =  input();

    while wait() {
        let current_state = input();

        if last_state != current_state {
            last_state = current_state;
        }
    }

    last_state
}

pub fn debounce_countdown<C: CountDown>(input: impl Fn() -> bool, wait: &mut C) -> bool 
where C::Time: From<fugit::Duration<u32, 1, 1000000>> {
    use fugit::ExtU32;

    let mut last_state = input();

    wait.start(10.millis());

    while wait.wait().is_ok() {}

    input()

    // while wait.wait().is_err() {
    //     let current_state = input();
    //
    //     if last_state != current_state {
    //         last_state = current_state;
    //     }
    // }
    //
    // last_state
}
