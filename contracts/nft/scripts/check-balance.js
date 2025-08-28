const { ethers } = require("hardhat");

async function main() {
  const [deployer] = await ethers.getSigners();
  const balance = await ethers.provider.getBalance(deployer.address);

  console.log(`🔍 Checking deployer balance on ${hre.network.name}`);
  console.log(`📍 Deployer: ${deployer.address}`);
  console.log(`💰 Balance: ${ethers.utils.formatEther(balance)} ETH`);

  // Check if balance is sufficient for deployment (at least 0.01 ETH)
  const minBalance = ethers.utils.parseEther("0.01");
  
  if (balance.lt(minBalance)) {
    throw new Error(
      `❌ Insufficient funds: ${deployer.address} has ${ethers.utils.formatEther(balance)} ETH on ${hre.network.name}. ` +
      `Minimum required: 0.01 ETH. Please fund the account before retrying.`
    );
  }

  console.log(`✅ Sufficient balance confirmed! Ready for deployment.`);
}

main().catch((error) => {
  console.error(error.message);
  process.exit(1);
});
