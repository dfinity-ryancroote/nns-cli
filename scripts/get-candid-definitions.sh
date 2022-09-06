#!/usr/bin/env bash

rm -r ./candid/
mkdir -p ./candid/
curl -sSL https://raw.githubusercontent.com/dfinity/ic/master/rs/nns/governance/canister/governance.did > candid/governance.did