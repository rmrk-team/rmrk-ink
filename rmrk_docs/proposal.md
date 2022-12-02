# Scope

The changes for RMRK 3.0 implementation as ink! smart contract.

# Goal

The main goal of this proposal is to simplify the RMRK contract handling for the contract owner and with it increase the RMRK contract usage. 

With this proposal the collection owner can evolve, add/replace resource or equip parent token by providing new collection of child tokens. The user then mints new child token, which brings new resource or new equipment for the parent token. The usage complexity for the owner is increasing with the number of tokens in the collection and with this proposal the evolution of the parent token can be simplified for owner. At the same time this proposal gives more flexibility for the child owner. The child contract can be configured to be mintable only by the owner of corresponding parent token, and can be offered as payable or free mint.

With the existing solution the collection contract owner needs to mint all tokens and add new resources for all tokens individually. After the minting and setting all resources, the frontend marketplace requires buy/sell functions for selling and buying tokens for end users. This is avoided in this proposal by having payable minting function which is callable by any user. The marketplace is then used for auctions and reselling of the tokens.

This proposal does not change or limit any or the core RMRK features. The changes are mainly in roles/responsibilities for all actors. The user experience should be improved or remain the same.

# Proposal

## Contract Creation

- Creation Changes
    - One Smart contract is one RMRK collection
    - The Smart contract deployer is the contract owner
    - The Smart contract is initialised with
        - Name
        - Symbol
        - Collection metadata (baseUri)
        - Max number of tokens
        - Token mint price
        - Royalty receiver
        - Royalty percentage
    - Compared changes:
        
        
        | RMRK 2.0 | This proposal |
        | --- | --- |
        | Each token gets its own metadata with mint call. Mint is callable only by the collection owner | Add uri for folder with metadata for all individual items. (BaseUri) |
        | The royalty owner for each token can be different account | The royalty owner is same for all tokens in the collection |

## Token Minting

- Minting changes
    - Minting is payable function
    - Any user can mint
    - Comparing changes:
        
        
        | RMRK 2.0 | This Proposal |
        | --- | --- |
        | Owner mints all tokens | Users mint tokens |
        | Minting is not payable | Minting is payable function |
        | mint() function is called with metadata | mint() function inherits metadata by concatenating baseUri and token id |

## Token Nesting

- Nesting changes
    - User calls add_child(). The child token needs to be approved for transfer in the child contract. The operator for child token in the child collection is the Parent collection address. This allows any smart contract which implements approve() and transfer() functions to be the child token. Main purpose of this change is to allow PSP34 tokens (ink! version of ERC721) to be nested by RMRK token.
    - The Child token needs to be approved if the caller of add_child() is not the owner of parent token.
    - This architecture allows parent token to hold fungible tokens as well.
    - Comparing changes:
    
    | RMRK 2.0 | This proposal |
    | --- | --- |
    | User calls mint_nft_to_nft() in child contract. The Child contract calls add_child() in the parent contract. | User calls approve()in child contract. User calls add_child(). Parent contract calls transfer() in child contract |
    | Can’t hold native tokens | Can hold native tokens |

## Multi Resources

- Resource handling changes
    - Each token has assigned basic resource during minting
    - New resource for the parent token is added by nesting new child token (from new children collection), with the call to add_child() function.
        - If the child token does not have field resource in the token metadata, it will not add  resource to the parent token.
    - Parent token holds the list of resources which can be changed by set_priority() call.
    - Resource replacing is handled by removing child resource token and adding new one
    - Resource id is represented as (collection_id, token_id)
    - Comparing changes:
    
    | RMRK 2.0 | This Proposal |
    | --- | --- |
    | Resource is added with add_resource call for each token individually. Only collection owner can add resource | Resource added by minting new token (different collection). Anyone can mint it. It can be payable function or for free. |
    | Resources are prioritised by set_priority() call | Resources are prioritised by set_priority() call |
    | Resource replacing is handled by adding and accepting new resource with same id | Resource is replaced by removing resource child token and adding new resource child token |
    | Each resource id has its own assigned number | The (collection, token) of the child token is the id  |
    

## Base

- Base changes
    - Each collection/contract has only one Base
    - All tokens in the collection inherit from collection Base

## Themes

No changes

## Equipping

No changes

# Examples

Based on the 2 Kingdoms example here is the example which shows the flow described in this document.

Example of RMRK usage for collection of Prof Of Attendance (POA) NFTs.