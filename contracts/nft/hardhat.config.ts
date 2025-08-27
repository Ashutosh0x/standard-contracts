import "./tasks";
import "@zetachain/localnet/tasks";
import "@nomicfoundation/hardhat-toolbox";
import "@zetachain/toolkit/tasks";

import { HardhatUserConfig } from "hardhat/config";
import { getHardhatConfig } from "@zetachain/toolkit/client";

import "@nomiclabs/hardhat-ethers";
import "@openzeppelin/hardhat-upgrades";

const config: HardhatUserConfig = {
  ...getHardhatConfig({ accounts: [process.env.PRIVATE_KEY] }),
  solidity: {
    compilers: [
      {
        settings: {
          optimizer: {
            enabled: true,
            runs: 1000,
          },
        },
        version: "0.8.26",
      },
    ],
  },
  networks: {
    "base-sepolia": {
      url: process.env.RPC_URL || "https://sepolia.base.org",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      chainId: 84532,
    },
    "zeta-testnet": {
      url: process.env.RPC_URL || "https://rpc.ankr.com/zetachain_evm_testnet",
      accounts: process.env.PRIVATE_KEY ? [process.env.PRIVATE_KEY] : [],
      chainId: 7001,
    },
  },
};

export default config;
