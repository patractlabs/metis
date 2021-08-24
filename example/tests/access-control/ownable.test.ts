import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';
import { buildTx } from '@redspot/patract/buildTx'
import type { Text } from '@polkadot/types';

const { getContractFactory, getRandomSigner } = patract;

const { api, getAddresses, getSigners } = network;


async function expectRevert(promise, expectedError) {
  try {
    await promise;
  } catch (exp) {
    expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
  }
}

describe('ERC20', () => {
  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]));
    const signers = await getSigners();
    const addresses = await getAddresses();
    const Alice = signers[0];
    const sender = await getRandomSigner(Alice, one.muln(10000));
    const contractFactory = await getContractFactory('ownable_erc20', sender);
    const contract = await contractFactory.deploy('new', '1000');
    const abi = artifacts.readArtifact('ownable_erc20');
    const receiver = addresses[1];

    await buildTx(api.registry, api.tx.balances.transfer(receiver, 50000000000), addresses[0]);

    return { sender, contractFactory, contract, abi, receiver, Alice, one };
  }

  it('Transfer emits event', async () => {
    const { contract, sender, receiver } = await setup();

    console.log("contract ", contract.address.toHex())
    console.log("sender", sender.address)
    console.log("receiver", receiver)


    await expect(contract.tx.transfer(receiver, 7))
      .to.emit(contract, 'Transfer')
      .withArgs(sender.address, receiver, 7);
  });

  it('ERC20 metadatas', async () => {
    const { contract, sender, receiver } = await setup();

    const tokenName = await contract.query.name();
    await expect(tokenName.output as Text).to.equal('MetisTestToken');

    const tokenSymbol = await contract.query.symbol();
    await expect(tokenSymbol.output).to.equal('MET');

    const tokenDecimals = await contract.query.decimals();
    await expect(tokenDecimals.output).to.equal(18);
  });

  it('Owner initstate', async () => {
    const { contract, sender, receiver } = await setup();

    await expect((await contract.query.getOwnership()).output).to.equal(sender.address);
  });

  it('Owner renounce', async () => {
    const { contract, sender, receiver } = await setup();

    context('owner initial is ok', async function () {
      await expect((await contract.query.getOwnership()).output).to.equal(sender.address);
    });

    context('owner renounce not owner will failed', async function () {
      await expectRevert(
        contract.connect(receiver).tx.renounceOwnership(), '',
      );

      await expect((await contract.query.getOwnership()).output).to.equal(sender.address);
    });

    context('owner renounce by owner will ok', async function () {
      await contract.connect(sender.address).tx.renounceOwnership();

      await expect((await contract.query.getOwnership()).output).to.equal(null);
    });
  });

  it('Owner transfer_ownership', async () => {
    const { contract, sender, receiver } = await setup();

    context('owner initial is ok', async function () {
      await expect((await contract.query.getOwnership()).output).to.equal(sender.address);
    });

    context('owner transfer_ownership not owner will failed', async function () {
      await expectRevert(
        contract.connect(receiver).tx.transferOwnership(receiver), '',
      );

      await expect((await contract.query.getOwnership()).output).to.equal(sender.address);
    });

    context('owner transfer_ownership by owner will ok', async function () {
      await contract.connect(sender.address).tx.transferOwnership(receiver);

      await expect((await contract.query.getOwnership()).output).to.equal(receiver);
    });
  });
});
