#![allow(unused)]
//
// # Exercise Expectations
//
// The goal here to is to establish a basis for our technical interview,
// and be able to discuss the choices you made and why. We're far more interested
// in the discussion part than receiving a perfect implementation of the following
// exercise. The exercise is not designed to take too long of your personal time,
// and doesn't have to be completed fully (although, bonus point if it is). We estimate
// it should be achieveable to complete between 2 to 3 hours of dedicated time.
//
// Please keep this exercise private, and don't make your result available publically.
//
// ## Recap about Blockchain
//
// A blockchain is organised as sequence of blocks.
//
//      Block 0 (Genesis) <- Block 1 <- Block 2 <- ...
//
// Block i is a parent of Block i+1
// Block i+1 is a child of Block i
//
// Each block has a specific hash, that are considered unique, and each blocks contains a reference to
// its parent block's hash.
//
// The first block is called genesis, and doesn't have a parent; this is the oldest block in the chain.
// The latest block is often called the tip of the chain and is the yougest block added to the chain.
//
// # Block streaming protocol
//
// Design a simple wire protocol to stream a sequence of blocks.
// For the purposes of this exercise, each block is represented with a
// hash of the parent block (all zeros can be used in the genesis block),
// a block number, and an opaque data blob for content.

pub struct Block {
    // Block number, monotonically increasing as the chain grows.
    block_number: u64,
    // Hash of the parent block.
    parent_hash: [u8; 32],
    // Block content.
    content: Box<[u8]>,
}

pub struct BlockStream<R: AsyncRead> {
    io: R,
}

//******************************************************************************
// Part 1.1: define a function to read a stream of blocks from a generic
// asynchronous input source.
//******************************************************************************

use futures::io;
use futures::prelude::*;

pub fn read_blocks<R: AsyncRead>(io: R) -> BlockStream<R> {
    todo!()
}


// The returned object should be usable in an async loop like this:
//
//  async {
//      let mut stream = read_blocks(io);
//      while let Some(res) = stream.next().await {
//          let block: Block = res?;
//          // ...
//      }
//  }
//
// It is allowed to use available libraries from e.g. crates.io for the
// implementation, but BlockStream should not be an alias to a foreign type.

//********************************************************************************
// Part 1.2: Given a list of streamed block chains, return the block that is
// the common ancestor if it exists, or None if there is no common ancestor
// to all of the chains.
//
// The streams are assumed to yield blocks of valid chains in
// descendant-to-ancestor order, i.e. in each stream the block numbers decrease
// monotonically and a parent hash is identical to one computed
// from the blocks in the rest of the stream.
//********************************************************************************

pub async fn find_common_ancestor<R: AsyncRead>(
    blockchain_streams: &mut [BlockStream<R>]) -> Result<Option<Block>, io::Error> {
    todo!()
}


//********************************************************************************
// Part 2: Tests
//
// * Describe your process to tests some of the functions and properties.
// * Either in the form of valid rust code, or commented pseudo code.
//********************************************************************************


fn main() {

}