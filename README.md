# Sorodrop - Merkle Airdrop Tooling

Sorodrop is a scalable and cost-effective solution for conducting large-scale airdrops of tokens and NFTs on the Stellar network. By leveraging Merkle tree structures, Sorodrop ensures efficient distribution with minimal gas fees. It offers a user-friendly interface for automating airdrops through CSV uploads, supports multi-stage distributions, and includes clawback functionality to reclaim unclaimed assets. Sorodrop is fully open-source, allowing for customization and flexibility in airdrop execution.

## Features
- Efficient Distribution: Uses Merkle trees for large-scale airdrops.
- Cost-Effective: Minimizes gas fees.
- Automation: Easy CSV uploads for recipient addresses.
- Resource Optimization: Clawback functionality for unclaimed tokens/NFTs.
- Flexible Airdrops: Pause, resume, and stage airdrops as needed.
- Open-Source: Customizable and adaptable to project-specific needs.

## Getting Started

### Smart Contracts

// TODO: Add smart contract deployment instructions

### Scripts

Scripts folder contains some helper scripts that can be used to generate merkle root and merkle proofs for each recipient of an airdrop.

`Bun` is used to manage dependencies and run scripts. To check out Bun and install it, visit [their website](https://bun.sh/).

After installing Bun and project dependencies, you can run the following commands:

#### Build and Deploy Smart Contracts

```bash
bun deploy
```

#### Generate Merkle Root and Merkle Proofs for Recipients

1. Create a CSV file with recipient addresses and amounts.
2. Run the following command:

```bash
bun merkle
```

#### Generate Merkle Root and Merkle Proofs for Recipients with Test Accounts

```
bun merkle:test
```
