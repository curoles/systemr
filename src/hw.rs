
struct OutputPort<T> {
    val: T,
}

struct InputPort<T> {
    val: T,
    driver: *const OutputPort<T>,
}

#[test]
fn input_port_construction() {
    let op = OutputPort::<u8>{val: 123};
    let ip = InputPort::<u8>{val: 7, driver: &op};
}
