import { expect } from "chai";
import { artifacts, network, patract } from "redspot";

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

describe("ERC721", () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    await api.isReady
    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];
    const sender = await getRandomSigner(Alice, "10000 UNIT");
    const contractFactory = await getContractFactory("erc721", sender.address);
    const contract = await contractFactory.deploy("new", "Test721", "TST");
    const abi = artifacts.readArtifact("erc721");
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice };
  }

  it("Assigns initial balance", async () => {
    const { contract, sender } = await setup();
  });
});
