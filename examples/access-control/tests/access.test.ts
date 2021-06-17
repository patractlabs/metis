import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { hexToU8a } from '@polkadot/util';

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;

const ROLE_EMPTY = hexToU8a('0x0000000000000000000000000000000000000000000000000000000000000000')
const ROLE_FILTER = hexToU8a('0x0101010101010101010101010101010101010101010101010101010101010101')
const ROLE_SETTER = hexToU8a('0x0202020202020202020202020202020202020202020202020202020202020202')
const ROLE_ADMIN = hexToU8a('0x0303030303030303030303030303030303030303030303030303030303030303')

describe("access-control-flip", () => {
  after(() => {
    return api.disconnect();
  });


  async function setup() {
    await api.isReady
    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];
    const Bob = signerAddresses[1];
    const Cite = signerAddresses[2];
    const sender = await getRandomSigner(Alice, "100000 UNIT");
    const contractFactory = await getContractFactory("access_control", sender.address);
    const contract = await contractFactory.deploy("new", true, Alice, Bob, Cite);
    const abi = artifacts.readArtifact("access_control");
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice, Bob, Cite };
  }

  it("initial status", async () => {
    const { contract, sender, Alice, Bob, Cite } = await setup();
    expect((await contract.query.get()).output).to.equal(true);

    expect((await contract.query.hasRole(ROLE_EMPTY, Alice)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
    expect((await contract.query.hasRole(ROLE_SETTER, Alice)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);

    expect((await contract.query.hasRole(ROLE_EMPTY, Bob)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_FILTER, Bob)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_SETTER, Bob)).output).to.equal(true);
    expect((await contract.query.hasRole(ROLE_ADMIN, Bob)).output).to.equal(false);

    expect((await contract.query.hasRole(ROLE_EMPTY, Cite)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_FILTER, Cite)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_SETTER, Cite)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_ADMIN, Cite)).output).to.equal(true);

    expect((await contract.query.getRoleAdmin(ROLE_FILTER)).output).to.equal(ROLE_ADMIN);
    expect((await contract.query.getRoleAdmin(ROLE_SETTER)).output).to.equal(ROLE_ADMIN);
    expect((await contract.query.getRoleAdmin(ROLE_EMPTY)).output).to.equal(null);
    expect((await contract.query.getRoleAdmin(ROLE_ADMIN)).output).to.equal(null);
  });
});



