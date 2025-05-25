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

impl<T: PortVal> InputPort<T> {
    pub fn connect(&mut self, out: &OutputPort<T>) {
        assert!(self.driver.is_null());
        self.driver = std::ptr::from_ref(out);
    }

    pub fn update(&mut self) {
        assert!(!self.driver.is_null());
        unsafe { self.val = (*self.driver).val; }
    }
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

#[test]
fn input_port_connection() {
    let op = OutputPort::<u8>{val: 123};
    let mut _ip : InputPort::<u8> = Default::default();
    _ip.connect(&op);
}

pub struct PortPtrs {
    driver: *const u8,
    val: *const u8,
    size: usize,
}

pub trait Module {
    fn run(&mut self);
    fn register_port<T: PortVal>(&mut self, inp: *const InputPort<T>) {
        //copy_from(self, src: *const T, count: usize), cast_mut
        let _info = PortPtrs {
            //driver: std::ptr::addr_of!((*inp).val),
            driver: unsafe { &raw const (*inp).val as *const u8 },
            val: unsafe { &raw const (*inp).val as *const u8 },
            size: size_of::<T>(),
        };
    }
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
    let mut dff = Dff::<u32>::new();
    dff.inp.connect(&op);
    dff.register_port(&dff.inp);
}