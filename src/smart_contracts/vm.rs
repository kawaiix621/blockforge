pub struct VirtualMachine;

impl VirtualMachine {
    pub fn execute_contract(&self, contract: &str, input: &str) -> String {
        format!("Executing contract: {} with input: {}", contract, input)
    }
}
