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

    const sender = await getRandomSigner(Alice, "10000 UNIT");

    const contractFactory = await getContractFactory("access_control", sender.address);
    const contract = await contractFactory.deploy("new", true, Alice, Bob, Carol);
    const abi = artifacts.readArtifact("access_control");
    const receiver = await getRandomSigner();

    return { contractFactory, contract, abi, receiver, Alice, Bob, Carol, Dan };
  }

  it("initial status", async () => {
    const { contract, Alice, Bob, Carol, Dan } = await setup();
    expect((await contract.query.get()).output).to.equal(true);

    expect((await contract.query.hasRole(ROLE_EMPTY, Alice)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
    expect((await contract.query.hasRole(ROLE_SETTER, Alice)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);

    expect((await contract.query.hasRole(ROLE_EMPTY, Bob)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_FILTER, Bob)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_SETTER, Bob)).output).to.equal(true);
    expect((await contract.query.hasRole(ROLE_ADMIN, Bob)).output).to.equal(false);

    expect((await contract.query.hasRole(ROLE_EMPTY, Carol)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_FILTER, Carol)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_SETTER, Carol)).output).to.equal(false);
    expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

    expect((await contract.query.getRoleAdmin(ROLE_FILTER)).output).to.equal(ROLE_ADMIN);
    expect((await contract.query.getRoleAdmin(ROLE_SETTER)).output).to.equal(null);
    expect((await contract.query.getRoleAdmin(ROLE_EMPTY)).output).to.equal(null);
    expect((await contract.query.getRoleAdmin(ROLE_ADMIN)).output).to.equal(null);
  });

  context('grant should ok', async () => {

    it("no-admin grant role should error", async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);

      // should panic, TODO: a helper function
      try {
        (await contract.connect(Dan).tx.grantRole(ROLE_FILTER, Dan));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);
    });

    it("admin grant role should ok", async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      expect((await contract.query.hasRole(ROLE_FILTER, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      // have a role
      await expect(contract.connect(Carol).tx.grantRole(ROLE_FILTER, Dan))
        .to.emit(contract, 'RoleGranted')
        .withArgs(ROLE_FILTER, Dan, Carol);

      // should have the role
      expect((await contract.query.hasRole(ROLE_FILTER, Dan)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });

    it("grant to role without admin should error", async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_SETTER, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);

      // should panic, TODO: a helper function
      try {
        (await contract.connect(Dan).tx.grantRole(ROLE_SETTER, Dan));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      try {
        (await contract.connect(Alice).tx.grantRole(ROLE_SETTER, Dan));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_SETTER, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);
    });

    it('grant accounts a role multiple times should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      // should panic, TODO: a helper function
      try {
        (await contract.connect(Carol).tx.grantRole(ROLE_FILTER, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });
  });

  context('revoked should ok', async () => {
    it('revoked roles that are not had should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      // should panic, TODO: a helper function
      try {
        (await contract.connect(Carol).tx.revokeRole(ROLE_FILTER, Dan));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Dan)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });

    it('revoked granted role by admin should ok', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      await expect(contract.connect(Carol).tx.revokeRole(ROLE_FILTER, Alice))
        .to.emit(contract, 'RoleRevoked')
        .withArgs(ROLE_FILTER, Alice, Carol);

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });

    it('revoked granted role by no-admin should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Bob)).output).to.equal(false);

      // should panic, TODO: a helper function
      try {
        (await contract.connect(Bob).tx.revokeRole(ROLE_FILTER, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      try {
        (await contract.connect(Alice).tx.revokeRole(ROLE_FILTER, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Bob)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });


    it('revoked granted role muit-times should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      await expect(contract.connect(Carol).tx.revokeRole(ROLE_FILTER, Alice))
        .to.emit(contract, 'RoleRevoked')
        .withArgs(ROLE_FILTER, Alice, Carol);

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);


      try {
        (await contract.connect(Carol).tx.revokeRole(ROLE_FILTER, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });
  });

  context('renouncing should ok', async () => {
    it('renouncing roles not had should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      // should panic, TODO: a helper function
      try {
        (await contract.connect(Alice).tx.renounceRole(ROLE_ADMIN, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });

    it('renouncing granted role by self should ok', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      await expect(contract.connect(Alice).tx.renounceRole(ROLE_FILTER, Alice))
        .to.emit(contract, 'RoleRevoked')
        .withArgs(ROLE_FILTER, Alice, Alice);

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });

    it('renouncing granted role by other should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      // should panic, TODO: a helper function
      try {
        (await contract.connect(Carol).tx.renounceRole(ROLE_FILTER, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      try {
        (await contract.connect(Bob).tx.renounceRole(ROLE_FILTER, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });

    it('renouncing granted role muit-times should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      await expect(contract.connect(Alice).tx.renounceRole(ROLE_FILTER, Alice))
        .to.emit(contract, 'RoleRevoked')
        .withArgs(ROLE_FILTER, Alice, Alice);

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);

      try {
        (await contract.connect(Alice).tx.renounceRole(ROLE_FILTER, Alice));
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      // status should not change
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Alice)).output).to.equal(false);
      expect((await contract.query.hasRole(ROLE_ADMIN, Carol)).output).to.equal(true);
    });
  });

  context('ensure role should ok', async () => {
    it('sender roles not had should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_FILTER, Bob)).output).to.equal(false);

      expect((await contract.query.get()).output).to.equal(true);

      try {
        await contract.connect(Dan).tx.flip();
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      expect((await contract.query.get()).output).to.equal(true);
    });

    it('sender had role should ok', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_FILTER, Bob)).output).to.equal(false);

      expect((await contract.query.get()).output).to.equal(true);

      await contract.connect(Alice).tx.flip();

      expect((await contract.query.get()).output).to.equal(false);
    });

    it('sender had other role should error', async () => {
      const { contract, Alice, Bob, Carol, Dan } = await setup();

      // if not has the admin role, should be error
      expect((await contract.query.hasRole(ROLE_FILTER, Alice)).output).to.equal(true);
      expect((await contract.query.hasRole(ROLE_FILTER, Bob)).output).to.equal(false);

      expect((await contract.query.get()).output).to.equal(true);

      try {
        await contract.connect(Bob).tx.flip();
      } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
      }

      expect((await contract.query.get()).output).to.equal(true);
    });
  });
});



