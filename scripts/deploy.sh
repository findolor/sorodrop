#!/bin/bash

SOROBAN_RPC_URL="https://soroban-testnet.stellar.org"
SOROBAN_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
NETWORK="testnet"

cd ../contracts

echo "1. Addding testnet to soroban config"
soroban network add --global testnet \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"

echo "2. Creating new wallet called "sorodrop-wallet" "
soroban keys generate --global sorodrop-wallet \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE" \
  --network "$NETWORK"

export SOROSPLITS_WALLET=$(soroban keys address sorodrop-wallet)
echo "3. New wallet "sorosplit-wallet" created: $(echo $SOROSPLITS_WALLET) "

curl "https://friendbot.stellar.org/?addr=$(echo $SOROSPLITS_WALLET)"
echo "4. Funding the new wallet with friendbot. "

echo "5. Building contracts"
soroban contract build

echo "6. Deploying the airdrop contract to the network"
export AIRDROP_CONTRACT_ADDRESS=$(soroban contract deploy \
  --wasm ./target/wasm32-unknown-unknown/release/sorodrop_airdrop.wasm \
  --source sorodrop-wallet \
  --network testnet)

echo "9. Deploying the token contract to the network"
export TOKEN_CONTRACT_ADDRESS=$(soroban contract deploy \
  --wasm ./external_wasm/soroban_token_contract.wasm \
  --source sorodrop-wallet \
  --network testnet)

soroban contract invoke \
--source-account sorodrop-wallet \
--rpc-url "$SOROBAN_RPC_URL" \
--network-passphrase "$SOROBAN_NETWORK_PASSPHRASE" \
--network "$NETWORK" \
    --id "$TOKEN_CONTRACT_ADDRESS" \
    -- \
    initialize \
    --admin "$SOROSPLITS_WALLET" \
--decimal 7 \
--name "Custom Token" \
--symbol "CTK"

echo "10. Contract deployment complete."
echo ""

echo "Airdrop Contract Address: $AIRDROP_CONTRACT_ADDRESS"
echo "Token Contract Address: $TOKEN_CONTRACT_ADDRESS"

cd ../scripts
if [ ! -d "artifacts" ]; then
  mkdir artifacts
fi

printf "%s" "$SOROSPLITS_WALLET" > ./artifacts/sorosplits_wallet
printf "%s" "$AIRDROP_CONTRACT_ADDRESS" > ./artifacts/airdrop_contract_address
printf "%s" "$TOKEN_CONTRACT_ADDRESS" > ./artifacts/token_contract_address

exit 0