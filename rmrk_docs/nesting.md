# Nesting NFT
please refer to [RMRK abstract spec](https://github.com/rmrk-team/rmrk-spec/tree/master/standards/abstract) for details

Nesting NFTs is a capability of an NFT to be the owner of one or more other NFTs. RMRK NFTs have this capability to own other NFTs. On top of that this ink! implementation aims to support PSP34 NFTs as child NFTs as well.
Some of the solutions for nesting inplementations are therefore a bit different than EVM implementation, however the functionality will be the same.

## SEND interaction (RMRK Specification requirement)
1. Send/add any RMRK based NFT to another RMRK NFT
1. Send/add any PSP34 NFT to RMRK NFT (ink! specific implementation")

#### Terminology
* Parent NFT - RMRK NFTs can be Parent if RMRK-contract is configured to accept nested child NFTs
* Child NFT - Any RMRK contract can be child. PSP34 can be child NFT
* PSP34 - Polkadot Standard Proposal for NFTs in ink!
* Nesting - NFT's capability to own another NFT

#### Implemented functions in ink! RMRK contract
* `add_nft(child_collection_id, child_token_id, parent_token_id)`
    * adds external child NFT to a Parent NFT in called contract
    * cross contract calls PSP34::transfer(to, token_id) in another contract
* `remove_nft(child_collection_id, child_token_id, parent_token_id)`
    * removes nested external child NFT from Parent NFT
    * cross contract calls PSP34::transfer(to, token_id) in another contract
* `accept_nft(child_collection_id, child_token_id, parent_token_id)`
* `reject_nft(child_collection_id, child_token_id, parent_token_id)`
    * cross contract calls PSP34::transfer(to, token_id) in another contract
* `transfer_nested_nft(child_collection_id, child_token_id, parent_token_id)`
* `mint_to_nft` - Not implemented. This function can be achieved by using mint() and add_nft



## Example interaction
In the examples there are 3 contracts. Each contract is one collection of NFTs. Two contracts are Based on RMRK standard and one is based on PSP34 standard.
Contracts:
1. RMRM-parent, cid=99
1. RMRK-child, cid=1
1. PSP34-vanilla, cid=2

Pierre owns all green NFTs
Hoon owns all purple NFTs
![](https://i.imgur.com/hMawAK4.png)

### 1. Example - Add your own NFT to your RMRK NFT
> PSP34 contract is not in picture but the same transactions apply to it as for RMRK-child
Pierre owns RMRK NFT (1,1) and wants to send it to RMRK NFT (99,1)
1. Pierre signs and calls `add_nft()` in contract with cid=99
2. Contract cid=99 signs `transfer()` call with caller (Pierre)
3. Contract cid=1 is calling `transfer()` in RMRK-child (cid=1)
4. Contract cid=1 changes owner of (1,1) from Pierre to the contract=99. Uses Contract's address as a new owner (address = 9999)
5. NFT(1,1) is now child of (99,1). It is by default in state Approved since the owner initiated the action.
6. If and when Pierre wants to remove NFT(1,1) as a child, he calls `remove_nft()`
7. Contract cid=99 signs the call and calls transfer()
8. Contract cid=1 changes nft(1,1) owner to be Pierre


![](https://i.imgur.com/mI7IwpX.png)


### 2. Example - Add your NFT to other's RMRK NFT

> PSP34 contract is not in picture but the same transactions apply to it as for RMRK-child


> PSP34 contract is not in picture but same transactions apply to it as for RMRK-child
Pierre owns RMRK NFT (1,1) and wants to send it to Hoon's RMRK NFT (99,2)
1. Pierre signs and calls `add_nft()` in contract with cid=99
2. Contract cid=99 signs `transfer()` call with caller (Pierre)
3. Contract cid=1 is calling `transfer()` in RMRK-child (cid=1)
5. Contract cid=1 changes owner of (1,1) from Pierre to the contract=99. Uses Contract's address as a new owner (address = 9999)
6. nft(99,2) has attached nft(1,1) in state Pending (until Hoon accepts/rejects)
8. Hoon calls `accept_nft()` to accept nft(1,1) as a child, of previously owned nft(99,1)
9. nft(1,1) is accepted as a child of nft(99,1)
10. Hoon calls `remove_nft()` to remove nft(1,1) as a child of nft(99,1)
11. Contract cid=99 signs and calls `transfer()` in RMRK-child (cid=1)
12. Contract cid=1 changes owner of (1,1) from contracts_address=9999 to Hoon.

![](https://i.imgur.com/d9dlLT8.png)

### 3. Example - Transfer nested RMRK NFT, accepted


![](https://i.imgur.com/y0SNep9.jpg)


### 4. Example - Transfer nested RMRK NFT, rejected

![](https://i.imgur.com/0rTQHPQ.png)


### 5. Adding child to a nested child

![](https://i.imgur.com/IwMvXDU.jpg)
