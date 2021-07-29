import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { hexToU8a } from '@polkadot/util';
import { Null, Text, Option, TypeRegistry } from '@polkadot/types';
import { AccountId } from '@polkadot/types/interfaces/runtime';
import contractFactory from "@redspot/patract/contractFactory";

const registry = new TypeRegistry();

const { getContractFactory, getRandomSigner } = patract;
const { api, getAddresses, getSigners } = network;
const { keyring } = network;

const TIMELOCK_ADMIN_ROLE = hexToU8a('0xb057ce3cb9051cf2581899ed54b0d6803c0e38e8338025ab3d1c21b4b4f6f8e3');
const PROPOSER_ROLE = hexToU8a('0xec2f41cc3cff7469794f1f07ae29218d852ff065e75c2d5265d76286e2bf8e3f');
const EXECUTOR_ROLE = hexToU8a('0x5b5d255344597471a6dec86e87d2617958ba78e21e675de9ed51aeec2e525345');

async function setup(deployer) {
    await api.isReady

    const sender = await getRandomSigner(deployer, "10000 UNIT");
    const contractFactory = await getContractFactory('timelock_controller', sender);
    const receiverFactory = await getContractFactory('timelock_controller_receiver', sender);

    return { sender, contractFactory, receiverFactory };
}

async function initTimelockController(deployer, min_delay, proposers, executors) {
    let { sender, contractFactory, receiverFactory } = await setup(deployer);

    let receiver = await receiverFactory.deploy('new');
    let contract = await contractFactory.deploy('new', min_delay, proposers, executors);

    return { sender, receiver, contract }
}

async function waitBlock(num) {
    const signerAddresses = await getAddresses();

    for (let i = 0; i < num; i++) {
        console.log("do a transfer ", i, "/", num);
        await api.tx.balances.transfer(signerAddresses[10], 1).signAndSend(signerAddresses[0]);
    }
}

async function shouldBehaveLikeTimelockController() {
    let logs = null;

    async function expectEventInLogs(contract, eventName, ...params) {
        await expect(logs).to.emit(contract, eventName).withArgs(...params);
    }

    before(async function () {
        const signerAddresses = await getAddresses();
        this.deployer = signerAddresses[0];

        this.alice = signerAddresses[1];
        this.bob = signerAddresses[2];
        this.carol = signerAddresses[3];
        this.dan = signerAddresses[4];
        this.eve = signerAddresses[5];

        const fee = 50000000000;

        await api.tx.balances.transfer(this.alice, fee).signAndSend(this.deployer);
        await api.tx.balances.transfer(this.bob, fee).signAndSend(this.deployer);
        await api.tx.balances.transfer(this.carol, fee).signAndSend(this.deployer);
        await api.tx.balances.transfer(this.dan, fee).signAndSend(this.deployer);
        await api.tx.balances.transfer(this.eve, fee).signAndSend(this.deployer);
    })

    beforeEach(async function () {
        let proposers = [this.alice, this.bob];
        let executors = [this.carol, this.dan];

        let { sender, receiver, contract } = await initTimelockController(this.deployer, 1, proposers, executors);

        this.owner = sender;

        await receiver.connect(sender).tx.addAcceptCaller(contract.address);
        await receiver.connect(sender).tx.setReceiveStatus(true);

        this.receiver = receiver;
        this.contract = contract;
    });

    context('check init is ok', function () {
        describe('roles to account is ok', function () {
            it('proposers is correct', async function () {
                expect((await this.contract.query.hasRole(PROPOSER_ROLE, this.alice)).output).to.equal(true);
                expect((await this.contract.query.hasRole(PROPOSER_ROLE, this.bob)).output).to.equal(true);
            });

            it('exectors is correct', async function () {
                expect((await this.contract.query.hasRole(EXECUTOR_ROLE, this.carol)).output).to.equal(true);
                expect((await this.contract.query.hasRole(EXECUTOR_ROLE, this.dan)).output).to.equal(true);
            });
        });
    });

    const data = hexToU8a('0x009123456789');
    const salt = hexToU8a('0x0101010101010101010101010101010101010101010101010101010101010101')

    context('schedule', function () {
        const delay = 5;

        describe('when not scheduled', function () {
            it('schedule new is ok', async function () {
                let id = (await this.contract.query.hashOperation(
                    this.receiver.address,
                    100,
                    data,
                    null,
                    salt,
                )).output

                console.log("schedule id is {}", id.toString());

                logs = this.contract.connect(this.alice).tx.schedule(
                    this.receiver.address,
                    100,
                    data,
                    null,
                    salt,
                    delay);
                await logs;

                expectEventInLogs("TimelockController", "CallScheduled", 
                    id,
                    this.receiver.address,
                    100,
                    data,
                    null,
                    delay)

                expect((await this.contract.query.isOperation(id)).output).to.equal(true);
                expect((await this.contract.query.isOperationPending(id)).output).to.equal(true);
                expect((await this.contract.query.isOperationReady(id)).output).to.equal(false);
                expect((await this.contract.query.isOperationDone(id)).output).to.equal(false);
            });
        });
    });

    context('executed', function () {
        const delay = 5;
        
        it('schedule new is ok', async function () {
            let id = (await this.contract.query.hashOperation(
                this.receiver.address,
                100,
                data,
                null,
                salt,
            )).output

            console.log("schedule id is {}", id.toString());

            logs = this.contract.connect(this.alice).tx.schedule(
                this.receiver.address,
                100,
                data,
                null,
                salt,
                delay);
            await logs;

            expectEventInLogs("TimelockController", "CallScheduled", 
                id,
                this.receiver.address,
                100,
                data,
                null,
                delay)

            expect((await this.contract.query.isOperation(id)).output).to.equal(true);
            expect((await this.contract.query.isOperationPending(id)).output).to.equal(true);
            expect((await this.contract.query.isOperationReady(id)).output).to.equal(false);
            expect((await this.contract.query.isOperationDone(id)).output).to.equal(false);

            await waitBlock(5);

            expect((await this.contract.query.isOperation(id)).output).to.equal(true);
            expect((await this.contract.query.isOperationPending(id)).output).to.equal(true);
            expect((await this.contract.query.isOperationReady(id)).output).to.equal(true);
            expect((await this.contract.query.isOperationDone(id)).output).to.equal(false);

            logs = this.contract.connect(this.carol).tx.execute(
                this.receiver.address,
                100,
                data,
                null,
                salt, {value: 100});
            await logs;

            expectEventInLogs("TimelockController", "CallExecuted", 
                id,
                this.receiver.address,
                100,
                data)
        });
    });
}

describe("TimelockController", async () => {
    after(() => {
        return api.disconnect();
    });

    await shouldBehaveLikeTimelockController();
});