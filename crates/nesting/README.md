# Nesting NFT
Please refer to [RMRK abstract spec](https://github.com/rmrk-team/rmrk-spec/tree/master/standards/abstract) for details on RMRK standard.
> Note! Nesting module is not dependent on other RMRK modules and can be used as a stand alone module on top of PSP34

Nesting NFTs is a capability of an NFT to be the owner of one or more NFTs. The token which owns other tokens is called a parent token and the tokens nested under the parent are called children tokens. 
In this ink! implementation of RMRK contract, the child contract can be another RMRK contract or any token which implements PSP34 standard.
The parent contract can only be RMRK contract.

Some of the solutions for nesting implementations are therefore different than EVM implementation, however the desired functionality will be the same.

### SEND interaction (RMRK Specification requirement)
1. Send/add any RMRK based NFT to another RMRK NFT
1. Send/add any PSP34 NFT to RMRK NFT (ink! specific implementation")

### BURN interaction (RMRK Specification requirement)
1. There is no burn call in this implementation since the burn is implemented as a token transfer

#### Terminology
* Parent RMRK contract- RMRK contract which implements Nesting module
* Child Contract - Any contract based on PSP34, including other RMRK contract
* Parent token - A token from Parent contract, owning children tokens
* Children token - A token from a child contract, owned by parent token
* PSP34 - Polkadot Standard Proposal for NFTs in ink!
* Nesting - NFT's capability to own another NFT
* Accepted child token - Each child token needs to be accepted by the parent token owner before it can be considered as Accepted
* Pending child token - A token is considered pending before it is accepted
#### Implemented Nesting functions in ink! RMRK contract

* Add a child NFT (from different collection) to the NFT to NFT in this collection.

```
fn add_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
```
* Remove a child NFT (from different collection) from token_id in this
```
fn remove_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
```

* Accept a child NFT (from different collection) to be owned by parent token.
```
fn accept_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
```

* Reject a child NFT (from different collection).
```
fn reject_child(&mut self, parent_token_id: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
```
* Transfer the child NFT from one parent to another (in this collection).
```
fn transfer_child(&mut self, from: Id, to: Id, child_nft: ChildNft) -> Result<(), PSP34Error>;
```
* Read the number of children on the parent token.
```
fn children_balance(&self, parent_token_id: Id) -> Result<(u64, u64), PSP34Error>;
```


## Example interaction
In the examples there are 3 contracts. Each contract is one collection of NFTs. Two contracts are Based on RMRK standard and one is based on PSP34 standard.
Contracts:
1. RMRM-parent, cid=99
1. RMRK-child, cid=1
1. PSP34-vanilla, cid=2

* Pierre owns all green NFTs
* Hoon owns all purple NFTs
![](https://i.imgur.com/hMawAK4.png)

---
### 1. Example - Add a RMRK child to RMRK parent, same user
> PSP34 contract is not in picture but the same transactions apply to it as for RMRK-child

1. Pierre owns child NFT (1,1) and he also owns parent (99,1)
1. Pierre wants to send child (1,1) it to parent (99,1)
1. Pierre needs to enable alowance for cid=99 and calls `approve(99, 1)` on child contract
1. Pierre calls `add_child(1, (1,1))` in parent contract (cid=99) to add child(1,1) to parent (99,1)
2. Parent contract cid=99 internally calls `transfer()` in child contract (cid=1)
4. Contract cid=1 changes owner of (1,1) from Pierre to the contract=99. Uses Contract's address as a new owner (address = 9999)
5. NFT(1,1) is now child of (99,1). It is by default in state Approved since the owner initiated the action.
6. If and when Pierre wants to remove NFT(1,1) as a child, he calls `remove_child(1, (1,1))`

![](https://i.imgur.com/Xvw92p6.png)

---
### 2. Example - Add your NFT to other's RMRK NFT
> PSP34 contract is not in picture but same transactions apply to it as for RMRK-child
1. Pierre owns child NFT (1,1) and Hoon owns parent (99,1)
1. Pierre wants to send child (1,1) it to parent (99,1)
1. Pierre needs to enable alowance for cid=99 and calls `approve(99, 1)` on child contract
1. Pierre calls `add_child(1, (1,1))` in parent contract (cid=99) to add child(1,1) to parent (99,1)
2. Parent contract cid=99 internally calls `transfer()` in child contract (cid=1)
4. Contract cid=1 changes owner of (1,1) from Pierre to the contract=99. Uses Contract's address as a new owner (address = 9999)
5. NFT(1,1) is now PENDING child of (99,1).
6. If and when Hoon wants to accept or reject NFT(1,1) as a child, he calls `accept_child(1, (1,1))` or `reject_child(1, (1,1))`

![](https://i.imgur.com/o5zP0xt.png)

---
### 3. Adding child to a nested child
Adding child needs to be bottom up. You can't add child to already nested token. You need to firs remove the child, add a child to it and than add it back ot parent.

![](https://i.imgur.com/IwMvXDU.jpg)
