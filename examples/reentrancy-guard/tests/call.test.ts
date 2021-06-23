import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { hexToU8a } from '@polkadot/util';

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

describe("reentrancy-guard test", () => {
  after(() => {
    return api.disconnect();
  });

  before(async () => {
    await init();
  });

  async function init() {
    await api.isReady

    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];
    const Bob = signerAddresses[1];
    const Carol = signerAddresses[2];
    const Dan = signerAddresses[3];

    await api.tx.balances.transfer(Bob, 50000000000).signAndSend(Alice);
    await api.tx.balances.transfer(Carol, 50000000000).signAndSend(Alice);
    await api.tx.balances.transfer(Dan, 50000000000).signAndSend(Alice);
  }

  async function setup() {
    await api.isReady

    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];
    const Bob = signerAddresses[1];
    const Carol = signerAddresses[2];
    const Dan = signerAddresses[3];

    // depoly the flipper
    const sender = await getRandomSigner(Alice, "10000 UNIT");

    const contractFactory = await getContractFactory("flipper", sender.address);
    const contract = await contractFactory.deploy("new", true);
    const abi = artifacts.readArtifact("flipper");

    // depoly the caller
    const caller_sender = await getRandomSigner(Bob, "10000 UNIT");

    const callerContractFactory = await getContractFactory("caller", caller_sender.address);
    const caller = await callerContractFactory.deploy("new", contract.address);
    const caller_abi = artifacts.readArtifact("caller");

    return { contractFactory, contract, abi, Alice, caller, caller_abi, Bob, Carol, Dan };
  }

  it("initial status", async () => {
    const { contract, Alice, Bob, Carol, Dan } = await setup();
    expect((await contract.query.get()).output).to.equal(true);
  });

  context('call guard should ok', async () => {
    it("normal call should ok", async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      expect((await contract.query.get()).output).to.equal(true);
      await contract.tx.flip();
      expect((await contract.query.get()).output).to.equal(false);
    });

    it("call reentrant by self should failed", async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      expect((await contract.query.get()).output).to.equal(true);

      try {
        await contract.tx.flipPanic();
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      expect((await contract.query.get()).output).to.equal(true);
    });
  });

  context('call other to reentrant should ok', async () => {
    it("reentrant should failed", async () => {
      const { contract, caller, Alice, Bob, Carol, Dan } = await setup();

      await caller.connect(Bob).tx.setCallType(1);

      // caller will call flipper and to emit a callback
      expect((await contract.query.get()).output).to.equal(true);

      try {
        await caller.connect(Bob).tx.doSth();
      } catch (exp) {
        console.error(exp);
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      expect((await contract.query.get()).output).to.equal(true);
    });
  });
});



