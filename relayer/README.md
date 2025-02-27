# Relayer

Relayer service that streams transactions from blockchain networks, packages data into messages, and sends the packets to the correlated bridge component.

Thanks to Chainsafe for their work on [ChainBridge](https://github.com/ChainSafe/ChainBridge). Our implementation is inspired by their design and incorporates some of their code.

- [Development](#development)
- [Configuration](#configuration)
  - [Secrets](#secrets)
- [Build](#build)
- [Run](#run)
- [Tests](#tests)

## Development

This project requires the following tools for day to day development:

- [Mage](https://magefile.org/): Used for build tasks
- [Revive](https://github.com/mgechev/revive): Used for linting instead of golint
- [Subkey](https://substrate.dev/docs/en/knowledgebase/integrate/subkey): Used for substrate key management

Please install them first.

Run `mage` to see a list of available tasks (building, testing, linting, etc).

To enable revive for linting in VS-code, add the following to your config:

```json
{
    "go.lintTool": "revive",
    "go.lintFlags": [
        "-config=${workspaceFolder}/revive.toml"
    ],
}
```

## Contract Bindings

The bindings in the [contracts](contracts/) directory for our Ethereum contracts are dynamically generated.

Make sure you have the following dependencies installed:

Install [jq](https://stedolan.github.io/jq/):

```bash
sudo apt install jq
```

Install [abigen](https://geth.ethereum.org/docs/dapp/native-bindings):

```
go install github.com/ethereum/go-ethereum/cmd/abigen
```

Compile the contracts in the [ethereum](../ethereum) directory:

```bash
truffle compile --all
```

Generate the bindings:

```bash
go generate ./...
```

## Configuration

Before running the relayer, it needs to be configured first. By default the configuration file is read from  `~/.config/artemis-relay/config.toml`, but this can be overriden by passing the `--config PATH` flag to the relayer binary.

Example Configuration:

```toml
[ethereum]
endpoint = "ws://localhost:8545/"
descendants-until-final = 3

[ethereum.channels.basic]
inbound = "0x992B9df075935E522EC7950F37eC8557e86f6fdb"
outbound = "0x2ffA5ecdBe006d30397c7636d3e015EEE251369F"

[ethereum.channels.incentivized]
inbound = "0xFc97A6197dc90bef6bbEFD672742Ed75E9768553"
outbound = "0xEDa338E4dC46038493b885327842fD3E301CaB39"

[substrate]
endpoint = "ws://127.0.0.1:9944/"
```

NOTE: For development and testing, we use our E2E test stack described [here](../test/README.md). It automatically generates a suitable configuration for testing.

### Secrets

The relayer requires secret keys for submitting transactions to both chains. It reads these keys from environment variables.

Example:

```bash
export ARTEMIS_ETHEREUM_KEY="0x4e9444a6efd6d42725a250b650a781da2737ea308c839eaccb0f7f3dbd2fea77"
export ARTEMIS_SUBSTRATE_KEY="//Relay"
```

## Build

```bash
mage build
```

## Run

Run the relayer with the configuration described in [Configuration](#configuration).

```bash
build/artemis-relay run --config config.toml
```

NOTE: On its first run, the relayer has to perform some initial computation relating to Ethereum PoW verification. This can take over 10 minutes to complete, and is not a sign that its stuck or frozen.

## Tests

To run both unit and integration tests, run the following command:

```bash
mage test
```
