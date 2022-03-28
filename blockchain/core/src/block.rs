//? 定义区块头
pub struct BlockHeader{
    pub time: i64,
    pub tx_hash: String,
    pub pre_hash: String,
}
//? 区块的结构iu
pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String,
}