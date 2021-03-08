import type { Signer, Contract } from "ethers";

import { deploy1820 } from "../common";

import { network, ethers } from "hardhat";


const main = async () => {
  const [deployer] = await ethers.getSigners();

  // Deploy Channels
  const channels = await deployChannels();
  const basic = {
    inbound: channels.basic.inbound.address,
    outbound: channels.basic.outbound.address,
  };
  const incentivized = {
    inbound: channels.incentivized.inbound.address,
    outbound: channels.incentivized.outbound.address,
  };

  const ScaleCodec = await ethers.getContractFactory("ScaleCodec");
  const codec = await ScaleCodec.deploy();
  await codec.deployed();
  console.log("ScaleCodec deployed to: ", codec.address);

  // Deploy ETHApp
  const ethApp = await deployApp(
    "ETHApp",
    {
      ScaleCodec: codec.address
    },
    basic,
    incentivized
  );

  // Deploy ERC20App
  const erc20App = await deployApp(
    "ERC20App",
    {
      ScaleCodec: codec.address
    },
    basic,
    incentivized
  );

  // Deploy ERC1820 registry (For SnowDOT token)
  if (["hardhat", "localhost"].includes(network.name)) {
    await deploy1820(deployer);
    console.log("ERC1820 deployed to: 0x1820a4B7618BdE71Dce8cdc73aAB6C95905faD24");
  }

  // Deploy DOTApp
  const dotApp = await deployApp(
    "DOTApp",
    {
      ScaleCodec: codec.address
    },
    "Snowfork DOT",
    "SnowDOT",
    basic,
    incentivized
  );

  const addresses = {
    "BasicInboundChannel": basic.inbound,
    "BasicOutboundChannel": basic.outbound,
    "IncentivizedInboundChannel": incentivized.inbound,
    "IncentivizedOutboundChannel": incentivized.outbound,
    "ETHApp": ethApp.address,
    "ERC20App": erc20App.address,
    "DOTApp": dotApp.address,
  }

  console.log(addresses);
}


const deployChannels = async () => {
  const deploy = async (name: string, ...args) => {
    const factory = await ethers.getContractFactory(name);
    const ch = await factory.deploy(...args);
    await ch.deployed();
    return ch;
  }

  const channels = {
    basic: {
      inbound: await deploy("BasicInboundChannel"),
      outbound: await deploy("BasicOutboundChannel")
    },
    incentivized: {
      inbound: await deploy("IncentivizedInboundChannel"),
      outbound: await deploy("IncentivizedOutboundChannel")
    },
  }

  return channels;
}

const deployApp = async (name: string, libraries: any, ...args): Promise<Contract> => {
  const factory = await ethers.getContractFactory(
    name,
    {
      libraries,
    }
  );
  const app = await factory.deploy(...args);
  await app.deployed();

  return app;
}


main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
