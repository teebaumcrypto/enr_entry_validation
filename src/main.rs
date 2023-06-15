
use anyhow::Result;
use reth_primitives::{ForkId, ForkHash};
use reth_rlp::Decodable;
use reth_discv4::{Discv4Config, EnrForkIdEntry};
#[tokio::main]
async fn main() -> Result<()> {
    let fork: ForkId = ForkId {
        hash: ForkHash([220, 233, 108, 45]),
        next: 0u64,
    };

    let mut disc_conf = Discv4Config::default();
    disc_conf.add_eip868_pair("eth", EnrForkIdEntry::from(fork));
    let x = reth_discv4::test_utils::create_discv4_with_config(disc_conf).await;
    
    // local eip 868 enr is a private field, needs to be set manually to pub for execution
    println!("enr local: {:?}",x.1.local_eip_868_enr);


    let wrapped = reth_discv4::EnrWrapper::new(x.1.local_eip_868_enr.clone());
    println!("wrapped enr: {:?}",wrapped);

    let fork_entry_id = EnrForkIdEntry::decode(&mut wrapped.0.get(b"eth").unwrap());
    println!("entry forkid: {:?}",fork_entry_id);

    let fork_entry_id = ForkId::decode(&mut wrapped.0.get(b"eth").unwrap());
    println!("entry forkid: {:?}",fork_entry_id);


    Ok(())
}


