
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


    /* 
        OUTPUT
        enr local: Enr { id: Some("v4"), seq: 1,
        NodeId: 0xf9b8e4ade92a17aba1500d00b95a088df1aa5eedf96da562e98a6378314830fe, 
        signature: "7e44476bbb0d6ffb24090731257861a6f3b9eb4eec5331640132115046b2bbf45c1ed79422ca5e9efc53880d72d67377c5aa49047a461dfa1588ea2d216b9d71", 
        IpV4 UDP Socket: Some(0.0.0.0:61865), 
        IpV6 UDP Socket: None, 
        IpV4 TCP Socket: Some(0.0.0.0:0),
        IpV6 TCP Socket: None, 
        Other Pairs: [("eth", "c7c684dce96c2d80"), ("secp256k1", "a103323cb58a146df4265e4425f399b618d785c795cd46bbc6f59e74d5fb8a79adfb")] }
        
        wrapped enr: EnrWrapper(Enr { id: Some("v4"), seq: 1, 
        NodeId: 0xf9b8e4ade92a17aba1500d00b95a088df1aa5eedf96da562e98a6378314830fe,
         signature: "7e44476bbb0d6ffb24090731257861a6f3b9eb4eec5331640132115046b2bbf45c1ed79422ca5e9efc53880d72d67377c5aa49047a461dfa1588ea2d216b9d71", 
         IpV4 UDP Socket: Some(0.0.0.0:61865), 
         IpV6 UDP Socket: None, IpV4 TCP Socket: Some(0.0.0.0:0), 
         IpV6 TCP Socket: None, 
         Other Pairs: [("eth", "c7c684dce96c2d80"), ("secp256k1", "a103323cb58a146df4265e4425f399b618d785c795cd46bbc6f59e74d5fb8a79adfb")] })
        
        entry forkid: Err(UnexpectedString)
        entry forkid: Ok(ForkId { hash: ForkHash("dce96c2d"), next: 0 })
    
     */
    Ok(())
}


