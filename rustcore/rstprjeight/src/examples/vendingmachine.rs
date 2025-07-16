//State Pattern with Enums

enum VendingMachineState {
    Idle,
    ProcessingPayment,
    DispensingItem,
    OutOfOrder,
}

pub struct VendingMachine {
    state: VendingMachineState,
    items_available: u32,
}

impl VendingMachine {
    pub fn new() -> Self {
        VendingMachine {
            state: VendingMachineState::Idle,
            items_available: 10,
        }
    }

    pub fn insert_money(&mut self) {
        match self.state {
            VendingMachineState::Idle => {
                println!("Payment processing...");
                self.state = VendingMachineState::ProcessingPayment;
            }
            _ => println!("Cannot accept money in current state"),
        }
    }

    pub fn dispense_item(&mut self) {
        match self.state {
            VendingMachineState::ProcessingPayment if self.items_available > 0 => {
                println!("Dispensing item...");
                self.items_available -= 1;
                self.state = VendingMachineState::DispensingItem;
            }
            VendingMachineState::ProcessingPayment => {
                println!("Out of items!");
                self.state = VendingMachineState::OutOfOrder;
            }
            _ => println!("Cannot dispense in current state"),
        }
    }
}

// let mut machine = VendingMachine::new();
// machine.insert_money();
// machine.dispense_item();
