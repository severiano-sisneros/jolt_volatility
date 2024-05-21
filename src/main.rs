use std::{error::Error, io};
use csv;
use alloy_primitives::{address, Address, BlockNumber, TxHash, TxIndex, U256};
use swap::SwapValues;

// Specify a struct named Event with the following fields: amount0in,amount0out,amount1in,amount1out,contract_address,evt_block_number,evt_block_time,evt_index,evt_tx_hash,sender,to
#[derive(Debug, serde::Deserialize)]
struct Event {
    amount0in: U256,
    amount0out: U256,
    amount1in: U256,
    amount1out: U256,
    contract_address: Address,
    evt_block_number: BlockNumber,
    evt_block_time: String,
    evt_index: TxIndex  ,
    evt_tx_hash: TxHash,
    sender: Address,
    to: Address,
}


fn read_events() -> Result<Vec<Event>, Box<dyn Error>> {
    println!("Reading events from file");
    let mut rdr = csv::Reader::from_path("uniswap-v2-events.csv")?;
    // let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut event_vec = Vec::new();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let event: Event = result?;
        event_vec.push(event);
    }

    Ok(event_vec)
}


pub fn main() {
    let usdt_weth_pool: Address = address!("0d4a11d5EEaaC28EC3F61d100daF4d40471f1852");
    let mut usdt_weth_swaps = Vec::new();
    let events = read_events().unwrap();
    println!("Num events = {:?}", events.len());
    for event in events.iter() {
        if event.contract_address == usdt_weth_pool {
            let swap = SwapValues::new(event.amount0in, event.amount0out, event.amount1in, event.amount1out);
            usdt_weth_swaps.push(swap);
        }
    }

    let (prove_volatility, verify_volatility) = guest::build_volatility();
    let test_vec = &usdt_weth_swaps[0..2];
    println!("length test_vec = {:?}", test_vec.len());
    println!("test_vec = {:?}", test_vec);
    let (output, proof) = prove_volatility(test_vec.to_vec());
    let is_valid = verify_volatility(proof);

    println!("output: {}", output);
    println!("valid: {}", is_valid);
}
