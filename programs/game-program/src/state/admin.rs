use anchor_lang::prelude::*;

const MAX_OPERATORS: usize = 4;

#[account]
pub struct Admin {
    pub owner: Pubkey,
    pub operators: Vec<Pubkey>,
}

impl Admin {
    pub const SIZE: usize = 32 + 4 + 32 * MAX_OPERATORS;
    
    pub(crate) fn is_operator(&self, operator: &Pubkey) -> bool {
        self.operators.iter().any(|op| op == operator)
    }
    
    pub(crate) fn assign_operator(&mut self, operator: Pubkey) {
        if self.is_operator(&operator) {
            return;
        }
        self.operators.push(operator);
    }
    
    pub(crate) fn remove_operator(&mut self, operator: &Pubkey) {
        self.operators.retain(|op| op != operator);
    }
}
