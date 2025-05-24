pub trait PortVal: Copy + Default {}
impl<T: Copy + Default> PortVal for T {}

#[derive(Default)]
pub struct OutputPort<T: PortVal> {
    val: T,
}

pub struct InputPort<T: PortVal> {
    val: T,
    driver: *const OutputPort<T>,
}

impl<T: PortVal> Default for InputPort<T> {
    fn default() -> Self {
        Self {
            val: Default::default(),
            driver: std::ptr::null(),
        }
    }
}

#[test]
fn input_port_construction() {
    let op = OutputPort::<u8>{val: 123};
    let _ip = InputPort::<u8>{val: 7, driver: &op};
}

pub fn connect<T: PortVal>(out: &OutputPort<T>, inp: &mut InputPort<T>) {
    assert!(inp.driver.is_null());
    inp.driver = std::ptr::from_ref(out);
}

#[test]
fn input_port_connection() {
    let op = OutputPort::<u8>{val: 123};
    let mut ip : InputPort::<u8> = Default::default();
    connect(&op, &mut ip);
}

pub trait Module {
    fn run(&mut self);
}

#[derive(Default)]
pub struct Dff<T: PortVal> {
    inp: InputPort<T>,
    out: OutputPort<T>,
}

impl<T: PortVal> Dff<T> {
    pub fn new() -> Self {
        Self {..Default::default()}
    }
}

impl<T: PortVal> Module for Dff<T> {

    fn run(&mut self) {
        self.out.val = self.inp.val;
        //wait(1);
    }
}

#[test]
fn dff_input_port_connection() {
    let op = OutputPort::<u32>{val: 123};
    let mut dff : Dff::<u32> = Dff::<u32>::new();
    connect(&op, &mut dff.inp);//.register(scheduler, dff);
}