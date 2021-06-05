import BN from 'bn.js';
import { expect } from 'chai';
import { patract, network, artifacts } from 'redspot';
import type { Text } from '@polkadot/types';

const { getContractFactory, getRandomSigner } = patract;

const { api, getSigners } = network;

describe('ERC20', () => {
  after(() => {
    return api.disconnect();
  });

  async function setup() {
    const one = new BN(10).pow(new BN(api.registry.chainDecimals[0]));
    const signers = await getSigners();
    const Alice = signers[0];
    const sender = await getRandomSigner(Alice, one.muln(10000));
    const contractFactory = await getContractFactory('erc20ownable', sender);
    const contract = await contractFactory.deploy('new', '1000');
    const abi = artifacts.readArtifact('erc20ownable');
    const receiver = await getRandomSigner();

    return { sender, contractFactory, contract, abi, receiver, Alice, one };
  }


  it('Transfer emits event', async () => {
    const { contract, sender, receiver } = await setup();

    console.log("contract ", contract.address.toHex())
    console.log("sender", sender.address)
    console.log("receiver", receiver.address)


    await expect(contract.tx.transfer(receiver.address, 7))
      .to.emit(contract, 'Transfer')
      .withArgs(sender.address, receiver.address, 7);
  });

  it('ERC20 metadatas', async () => {
    const { contract, sender, receiver } = await setup();


    const tokenName = await contract.query.name();
    console.log("tokenName", tokenName)
    await expect(tokenName.output as Text).to.equal('8MetisTestToken');

    const tokenSymbol = await contract.query.symbol();
    await expect(tokenSymbol.output).to.equal('\fMET');

    const tokenDecimals = await contract.query.decimals();
    await expect(tokenDecimals.output).to.equal(18);
  });


});
