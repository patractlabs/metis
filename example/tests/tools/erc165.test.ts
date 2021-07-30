import { expect } from "chai";
import { artifacts, network, patract } from "redspot";

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

describe("ERC165Test", () => {
  after(() => {
    return api.disconnect();
  });

  let contract_name = "erc165";

  async function setup() {
    await api.isReady
    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];
    const sender = await getRandomSigner(Alice, "10000 UNIT");
    const contractFactory = await getContractFactory(contract_name, sender.address);
    const contract = await contractFactory.deploy("new", false);
    const abi = artifacts.readArtifact(contract_name);
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice };
  }

  it("Init state", async () => {
    const { contract, sender } = await setup();
    const result = await contract.query.get();
    expect(result.output).to.equal(false);
  });

  it("Check support interfaces", async () => {
    const { contract } = await setup();

    expect((await contract.query.supportsInterface(0x9bae9d5e ^ 0xed4b9d1b)).output).to.equal(true);
    expect((await contract.query.supportsInterface(0x633aa551 ^ 0x2f865bd9)).output).to.equal(true);
    expect((await contract.query.supportsInterface(0xe6113a8a)).output).to.equal(true);

    expect((await contract.query.supportsInterface(0xfffffffe)).output).to.equal(false);
    expect((await contract.query.supportsInterface(0xffffffff)).output).to.equal(false);
    expect((await contract.query.supportsInterface(12345)).output).to.equal(false);
  });

  it("Flip state", async () => {
    const { contract, sender } = await setup();

    expect((await contract.query.get()).output).to.equal(false);

    await expect(contract.tx.flip())
      .to.emit(contract, 'Flip')
      .withArgs(sender.address, true);;

    expect((await contract.query.get()).output).to.equal(true);

    await expect(contract.tx.flip())
      .to.emit(contract, 'Flip')
      .withArgs(sender.address, false);;

    expect((await contract.query.get()).output).to.equal(false);
  });

  it("Set state no change", async () => {
    const { contract, sender } = await setup();

    expect((await contract.query.get()).output).to.equal(false);

    // set false to false
    await expect(contract.tx.set(false))
      .to.emit(contract, 'Flip')
      .withArgs(sender.address, false);

    expect((await contract.query.get()).output).to.equal(false);

    // flip state
    await expect(contract.tx.flip())
      .to.emit(contract, 'Flip')
      .withArgs(sender.address, true);;

    expect((await contract.query.get()).output).to.equal(true);

    // set true to true
    await expect(contract.tx.set(true))
      .to.emit(contract, 'Flip')
      .withArgs(sender.address, true);

    expect((await contract.query.get()).output).to.equal(true);

  });

  it("Set state change", async () => {
    const { contract, sender } = await setup();

    expect((await contract.query.get()).output).to.equal(false);

    // set false to true
    await expect(contract.tx.set(true))
      .to.emit(contract, 'Flip')
      .withArgs(sender.address, true);

    expect((await contract.query.get()).output).to.equal(true);

    // set true to false
    await expect(contract.tx.set(false))
      .to.emit(contract, 'Flip')
      .withArgs(sender.address, false);

    expect((await contract.query.get()).output).to.equal(false);

  });
});
