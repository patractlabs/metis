import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { buildTx } from '@redspot/patract/buildTx'
import { hexToU8a } from '@polkadot/util';
import { Text, Option, TypeRegistry } from '@polkadot/types';

const registry = new TypeRegistry();

const { getContractFactory, getRandomSigner } = patract;
const { api, getAddresses, getSigners } = network;
const { keyring } = network;

function addressFromHexString(hexStr: string) {
  const hexAddress = hexToU8a(hexStr);
  return keyring.encodeAddress(hexAddress);
}

async function expectRevert(promise, expectedError) {
  try {
    await promise;
  } catch (exp) {
    expect(exp.error.message).to.equal("contracts.ContractTrapped")
  }
}

async function setup(contractName) {
  await api.isReady
  const signerAddresses = await getAddresses();
  const Alice = signerAddresses[0];
  const sender = await getRandomSigner(Alice, "10000 UNIT");
  const abi = artifacts.readArtifact(contractName);

  return { sender, signerAddresses };
}

function shouldBehaveLikeERC1155(errorPrefix, contractName) {
  const firstTokenId = hexToU8a('0x0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a')
  const secondTokenId = hexToU8a('0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b')
  const unknownTokenId = hexToU8a('0x0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c')

  const firstAmount = 1000;
  const secondAmount = 2000;

  const RECEIVER_SINGLE_MAGIC_VALUE = '0xf23a6e61';
  const RECEIVER_BATCH_MAGIC_VALUE = '0xbc197c81';

  const NONE_ACCOUNTID = new Option(registry, Text, undefined);
  const ZERO_ADDRESS = addressFromHexString('0x0000000000000000000000000000000000000000000000000000000000000000')

  before(async function () {
    const { signerAddresses, sender } = await setup(contractName);

    this.sender = signerAddresses[0]
    this.minter = signerAddresses[0]
    this.firstTokenHolder = signerAddresses[1]
    this.secondTokenHolder = signerAddresses[2]

    const fee = 50000000000;

    await buildTx(api.registry, api.tx.balances.transfer(this.firstTokenHolder, fee), this.sender);
    await buildTx(api.registry, api.tx.balances.transfer(this.secondTokenHolder, fee), this.sender);
  })

  beforeEach(async function () {
    const { signerAddresses, sender } = await setup(contractName);
    const contractFactory = await getContractFactory(contractName, sender.address);
    let res = await contractFactory.deploy("new", "url");
    this.token = res;
  });

  describe('like an ERC1155', function () {
    describe('balanceOf', function () {
      it('reverts when queried about the zero address', async function () {
        await expectRevert(
          this.token.balanceOf(ZERO_ADDRESS, firstTokenId),
          'ERC1155: balance query for the zero address',
        );
      });

      context('when accounts don\'t own tokens', function () {
        it('returns zero for given addresses', async function () {
          expect((await this.token.query.balanceOf(
            this.firstTokenHolder,
            firstTokenId,
          )).output).to.be.equal(0);

          expect((await this.token.query.balanceOf(
            this.secondTokenHolder,
            secondTokenId,
          )).output).to.be.equal(0);

          expect((await this.token.query.balanceOf(
            this.firstTokenHolder,
            unknownTokenId,
          )).output).to.be.equal(0);
        });
      });

      context('when accounts own some tokens', function () {
        beforeEach(async function () {
          await this.token.tx.mintTest(this.firstTokenHolder, firstTokenId, firstAmount, '0x');
          await this.token.tx.mintTest(
            this.secondTokenHolder,
            secondTokenId,
            secondAmount,
            '0x'
          );
        });

        it('returns the amount of tokens owned by the given addresses', async function () {
          expect((await this.token.query.balanceOf(
            this.firstTokenHolder,
            firstTokenId,
          )).output).to.be.equal(firstAmount);

          expect((await this.token.query.balanceOf(
            this.secondTokenHolder,
            secondTokenId,
          )).output).to.be.equal(secondAmount);

          expect((await this.token.query.balanceOf(
            this.firstTokenHolder,
            unknownTokenId,
          )).output).to.be.equal(0);
        });
      });
    });

    // shouldSupportInterfaces(['ERC165', 'ERC1155']);
  });
}

module.exports = {
  shouldBehaveLikeERC1155,
};
