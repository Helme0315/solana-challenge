# Solana Developer Challenge

**# Problem Statement**

1. There are 3 MasterEdition NFTs (strength, agility, intelligence).

2. User can select 1 of them and can mint editions.

3. Before mint, user will input some attributes. (health, mana, power) The value of attributes varies per edition mint.

4. User can stake them into simple nft staking contract and user will get reputation point.

5. Reputation point is rely on staked NFTs(

rep point = sum of each nft's point.

if nft is strength:     nftPoint = health * 2 + mana + power,

if nft is agility:         nftPoint = health + mana + power * 2,

if nft is inteligence:  nftPoint = health + mana * 2 + power,

)

# Requirement

- Write solana program by using anchor framework and write unit tests. It must run as expected with the tests
- We need security-focus developer because security is very important for solana programs. The high point will be given if you focus on security.
- Code cleanliness and clarity