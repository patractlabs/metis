import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { hexToU8a } from '@polkadot/util';
import { Null, Text, Option, TypeRegistry } from '@polkadot/types';
import { AccountId } from '@polkadot/types/interfaces/runtime';

const registry = new TypeRegistry();

const { getContractFactory, getRandomSigner } = patract;
const { api, getAddresses, getSigners } = network;
const { keyring } = network;

async function setup(contractName) {
    await api.isReady
    const signerAddresses = await getAddresses();
    const Alice = signerAddresses[0];
    const sender = await getRandomSigner(Alice, "10000 UNIT");
    const abi = artifacts.readArtifact(contractName);

    return { sender, signerAddresses };
}

async function shouldBehaveLikeERC777(errorPrefix, contractName) {
    // shouldSupportInterfaces([
    //  'ERC165',
    //  'ERC721',
    // ]);

    before(async function () {
        const { signerAddresses, sender } = await setup(contractName);

        this.sender = signerAddresses[0]
        this.owner = signerAddresses[0]
        this.newOwner = signerAddresses[1]
        this.approved = signerAddresses[2]
        this.anotherApproved = signerAddresses[3]
        this.operator = signerAddresses[4]
        this.other = signerAddresses[5]

        const fee = 50000000000;

        await api.tx.balances.transfer(this.newOwner, fee).signAndSend(this.owner);
        await api.tx.balances.transfer(this.approved, fee).signAndSend(this.owner);
        await api.tx.balances.transfer(this.anotherApproved, fee).signAndSend(this.owner);
        await api.tx.balances.transfer(this.operator, fee).signAndSend(this.owner);
        await api.tx.balances.transfer(this.other, fee).signAndSend(this.owner);
    })

    beforeEach(async function () {
        const { signerAddresses, sender } = await setup(contractName);
        const contractFactory = await getContractFactory(contractName, sender.address);
        let res = await contractFactory.deploy("new", "Test", "TST", 18, 10000000000);
        this.token = res;

        this.sender = signerAddresses[0]
        this.owner = signerAddresses[0]
        this.newOwner = signerAddresses[1]
        this.approved = signerAddresses[2]
        this.anotherApproved = signerAddresses[3]
        this.operator = signerAddresses[4]
        this.other = signerAddresses[5]
    });

    context('with minted tokens', function () {
        let init_amount = 100;

        beforeEach(async function () {
            await this.token.tx.mint(this.newOwner, init_amount);
            this.toWhom = this.other; // default to this.other for toWhom in context-dependent tests
        });

        describe('balanceOf', function () {
            context('when the given address owns some tokens', function () {
                it('returns the amount of tokens owned by the given address', async function () {
                    expect((await this.token.query.balanceOf(this.newOwner)).output).to.equal(init_amount);
                });
            });

            context('when the given address does not own any tokens', function () {
                it('returns 0', async function () {
                    expect((await this.token.query.balanceOf(this.other)).output).to.equal(0);
                });
            });
        });
    });
}


describe("ERC777", async () => {
    await shouldBehaveLikeERC777("ERC777", "erc777")
});