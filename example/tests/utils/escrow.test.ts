import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { hexToU8a } from '@polkadot/util';
import { textSpanOverlapsWith } from "typescript";

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

describe("escrow test", () => {
  after(() => {
    return api.disconnect();
  });

  before(async () => {
    await init();
  });

  async function init() {
    await api.isReady
  }

  async function setup() {
    await api.isReady

    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];

    const sender = await getRandomSigner(Alice, "10000 UNIT");

    const contractFactory = await getContractFactory("escrow", sender.address);
    const contract = await contractFactory.deploy("new");
    const abi = artifacts.readArtifact("escrow");
    const receiver = await getRandomSigner();

    return { contractFactory, contract, abi, receiver, Alice };
  }

  it("initial status", async () => {
    const { contract, Alice } = await setup();

    expect((await contract.query.depositsOf(Alice)).output).to.equal(0);
  });

  context('deposit should ok', async () => {
    it('deposit', async () => {
      const { contract, Alice } = await setup();
      const signerAddresses = await getAddresses();
      const Bob = signerAddresses[1];

      await api.tx.balances.transfer(Bob, 50000000000).signAndSend(Alice);

      await contract.tx.deposit(Bob);
      expect((await contract.query.depositsOf(Bob)).output).to.equal(0);

      await contract.tx.deposit(Bob, {value: 1000});
      expect((await contract.query.depositsOf(Bob)).output).to.equal(1000);

      await contract.tx.withdraw(Bob);
      expect((await contract.query.depositsOf(Bob)).output).to.equal(0);
    });
  });
});



