pub mod block;

mod blockchain {
    use crate::blockchain::block::Block;
    use std::vec::Vec;

    pub struct Blockchain {
        chain: Vec<Block>,
    }

    impl Blockchain {
        pub fn add_block(&mut self, data: String) {
            let new_block: Block = Block::mine_new_block(&self.chain[self.chain.len() - 1], data);
            self.chain.push(new_block);
        }

        pub fn is_valid_chain(&self, blocks: Vec<Block>) -> bool {
            if stringify!(blocks[0]) != stringify!(Block::get_genesis_block()) {
                return false;
            }

            self.chain.windows(2).all(|pair| {
                let previous_block = pair[0];
                let current_block = pair[1];

                current_block.last_hash == previous_block.hash
                    && current_block.hash == Block::generate_hash2(&current_block)
            });

            true
        }

        pub fn replace_chain(&mut self, new_blocks: Vec<Block>) -> bool {
            if new_blocks.len() <= self.chain.len() {
                println!("not replacing ");
                return false;
            }

            if !&self.is_valid_chain(new_blocks) {
                println!("chain not valid , not replacing");
                return false;
            }
            self.chain = new_blocks;
            println!("chain replaced");
            return true;
        }
    }
}
