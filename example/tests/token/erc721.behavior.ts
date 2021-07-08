import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { hexToU8a } from '@polkadot/util';
import { Null, Text, Option, TypeRegistry } from '@polkadot/types';
import { AccountId } from '@polkadot/types/interfaces/runtime';

const registry = new TypeRegistry();

const { getContractFactory, getRandomSigner } = patract;
const { api, getAddresses, getSigners } = network;
const { keyring } = network;

const firstTokenId = hexToU8a('0x0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a')
const secondTokenId = hexToU8a('0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b')
const nonExistentTokenId = hexToU8a('0x0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c')
const baseURI = 'https://api.com/v1/';

const RECEIVER_MAGIC_VALUE = '0x150b7a02';

function addressFromHexString(hexStr: string) {
    const hexAddress = hexToU8a(hexStr);
    return keyring.encodeAddress(hexAddress);
}

const NONE_ACCOUNTID = new Option(registry, Text, undefined);
const ZERO_ADDRESS = addressFromHexString('0x0000000000000000000000000000000000000000000000000000000000000000')

async function expectRevert(promise, expectedError) {
    try {
        await promise;
    } catch (exp) {
        expect(exp.error.message).to.equal("contracts.ContractTrapped( Contract trapped during execution.)")
    }
}

// logs, 'Transfer', { from: this.owner, to: this.toWhom, tokenId: tokenId }
async function expectEventInLogs(logs, contract, eventName, ...params) {
    await expect(logs).to.emit(contract, eventName).withArgs(...params);
}

async function shouldBehaveLikeERC721(errorPrefix, contractName) {
    // shouldSupportInterfaces([
    //  'ERC165',
    //  'ERC721',
    // ]);

    async function setup() {
        await api.isReady
        const signerAddresses = await getAddresses();
        const Alice = signerAddresses[0];
        const sender = await getRandomSigner(Alice, "10000 UNIT");
        const abi = artifacts.readArtifact("erc721");

        return { sender, signerAddresses };
    }

    beforeEach(async function () {
        const { signerAddresses, sender } = await setup();
        const contractFactory = await getContractFactory(contractName, sender.address);
        let res = await contractFactory.deploy("new", "NFT", "NFT");
        this.token = res;

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
    });

    context('with minted tokens', function () {
        beforeEach(async function () {
            await this.token.tx.mint(this.owner, firstTokenId);
            await this.token.tx.mint(this.owner, secondTokenId);
            this.toWhom = this.other; // default to this.other for toWhom in context-dependent tests
        });

        describe('balanceOf', function () {
            context('when the given address owns some tokens', function () {
                it('returns the amount of tokens owned by the given address', async function () {
                    expect((await this.token.query.balanceOf(this.owner)).output).to.equal(2);
                });
            });

            context('when the given address does not own any tokens', function () {
                it('returns 0', async function () {
                    expect((await this.token.query.balanceOf(this.other)).output).to.equal(0);
                });
            });

            /*
            context('when querying the zero address', function () {
                it('throws', async function () {
                    await expectRevert(
                        this.token.balanceOf(ZERO_ADDRESS), 'ERC721: balance query for the zero address',
                    );
                });
            });
            */
        });

        describe('ownerOf', function () {
            context('when the given token ID was tracked by this token', function () {
                const tokenId = firstTokenId;

                it('returns the this.owner of the given token ID', async function () {
                    expect((await this.token.query.ownerOf(tokenId)).output).to.equal(this.owner);
                });
            });

            context('when the given token ID was not tracked by this token', function () {
                const tokenId = nonExistentTokenId;

                it('reverts', async function () {
                    await expectRevert(
                        this.token.query.ownerOf(tokenId), 'ERC721: this.owner query for nonexistent token',
                    );
                });
            });
        });

        describe('transfers', function () {
            const tokenId = firstTokenId;
            const data = '0x42';

            let logs = null;

            beforeEach(async function () {
                await this.token.connect(this.owner).tx.approve(this.approved, tokenId);
                await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);
            });

            const transferWasSuccessful = function () {
                it('transfers the ownership of the given token ID to the given address', async function () {
                    expect((await this.token.query.ownerOf(tokenId)).output).to.equal(this.toWhom);
                });

                it('emits a Transfer event', async function () {
                    await expectEventInLogs(logs, this.token, 'Transfer', this.owner, this.toWhom, tokenId);
                });

                it('clears the approval for the token ID', async function () {
                    expect((await this.token.query.getApproved(tokenId)).output).to.equal(null);
                });

                it('emits an Approval event', async function () {
                    // FIXME: redspot should support the null to none
                    await expectEventInLogs(logs, this.token, 'Approval', this.owner, NONE_ACCOUNTID, tokenId);
                });

                it('adjusts owners balances', async function () {
                    expect((await this.token.query.balanceOf(this.owner)).output).to.equal(1);
                });

                it('adjusts owners tokens by index', async function () {
                    if (!this.token.tokenOfOwnerByIndex) return;

                    expect((await this.token.query.tokenOfOwnerByIndex(this.toWhom, 0)).output).to.equal(tokenId);
                    expect((await this.token.query.tokenOfOwnerByIndex(this.owner, 0)).output).to.not.equal(tokenId);
                });
            };

            const shouldTransferTokensByUsers = function (transferFunction) {
                context('when called by the this.owner', function () {
                    beforeEach(async function () {
                        logs = transferFunction.call(this, this.token, this.owner, this.toWhom, tokenId, { from: this.owner });
                        await logs
                    });
                    transferWasSuccessful();
                });

                context('when called by the approved individual', function () {
                    beforeEach(async function () {
                        logs = transferFunction.call(this, this.token, this.owner, this.toWhom, tokenId, { from: this.approved });
                        await logs
                    });
                    transferWasSuccessful();
                });

                context('when called by the operator', function () {
                    beforeEach(async function () {
                        logs = transferFunction.call(this, this.token, this.owner, this.toWhom, tokenId, { from: this.operator });
                        await logs
                    });
                    transferWasSuccessful();
                });

                context('when called by the this.owner without an approved user', function () {
                    beforeEach(async function () {
                        await this.token.connect(this.owner).tx.approve(null, tokenId);

                        logs = transferFunction.call(this, this.token, this.owner, this.toWhom, tokenId, { from: this.operator });
                        await logs
                    });
                    transferWasSuccessful();
                });

                context('when sent to the this.owner', function () {
                    beforeEach(async function () {
                        logs = transferFunction.call(this, this.token, this.owner, this.owner, tokenId, { from: this.owner });
                        await logs
                    });

                    it('keeps ownership of the token', async function () {
                        expect((await this.token.query.ownerOf(tokenId)).output).to.equal(this.owner);
                    });

                    it('clears the approval for the token ID', async function () {
                        expect((await this.token.query.getApproved(tokenId)).output).to.equal(null);
                    });

                    it('emits only a transfer event', async function () {
                        await expectEventInLogs(logs, this.token, 'Transfer',
                            this.owner,
                            this.owner,
                            tokenId,
                        );
                    });

                    it('keeps the this.owner balance', async function () {
                        expect((await this.token.balanceOf(this.owner)).output).to.equal(2);
                    });


                    it('keeps same tokens by index', async function () {
                        if (!this.token.tokenOfOwnerByIndex) return;
                        const tokensListed = await Promise.all(
                            [0, 1].map(i => this.token.query.tokenOfOwnerByIndex(this.owner, i)),
                        );
                        expect(tokensListed).to.have.members(
                            [firstTokenId, secondTokenId],
                        );
                    });
                });

                context('when the address of the previous this.owner is incorrect', function () {
                    it('reverts', async function () {
                        await expectRevert(
                            transferFunction.call(this, this.token, this.other, this.other, tokenId, { from: this.owner }),
                            'ERC721: transfer of token that is not own',
                        );
                    });
                });

                context('when the sender is not authorized for the token id', function () {
                    it('reverts', async function () {
                        await expectRevert(
                            transferFunction.call(this, this.token, this.owner, this.other, tokenId, { from: this.other }),
                            'ERC721: transfer caller is not this.owner nor approved',
                        );
                    });
                });

                context('when the given token ID does not exist', function () {
                    it('reverts', async function () {
                        await expectRevert(
                            transferFunction.call(this, this.token, this.owner, this.other, nonExistentTokenId, { from: this.owner }),
                            'ERC721: operator query for nonexistent token',
                        );
                    });
                });

                context('when the address to transfer the token to is the zero address', function () {
                    it('reverts', async function () {
                        await expectRevert(
                            transferFunction.call(this, this.token, this.owner, ZERO_ADDRESS, tokenId, { from: this.owner }),
                            'ERC721: transfer to the zero address',
                        );
                    });
                });
            };

            describe('via transferFrom', function () {
                shouldTransferTokensByUsers(function (token, from, to, tokenId, opts) {
                    if (opts.from != null) {
                        return token.connect(opts.from).tx.transferFrom(from, to, tokenId);
                    } else {
                        return token.tx.transferFrom(from, to, tokenId);
                    }
                });
            });

            describe('via safeTransferFrom', function () {
                beforeEach(async function () {
                    const sender = await getRandomSigner(this.sender, "10000 UNIT");
                    const contractFactory = await getContractFactory("erc721_token_receiver", sender.address);
                    let res = await contractFactory.deploy("new");

                    console.log("1")
                    await res.connect(this.sender).tx.addAcceptToken(this.token.address);
                    console.log("1")
                    await res.connect(this.sender).tx.setReceiveStatus(true);
                    console.log("1")

                    this.receiver = res;
                    this.toWhom = this.receiver.address;
                });


                const safeTransferFromWithData = function (token, from, to, tokenId, opts) {
                    return token.connect(opts.from).tx.safeTransferFromWithData(from, to, tokenId, data);
                };

                const safeTransferFromWithoutData = function (token, from, to, tokenId, opts) {
                    return token.connect(opts.from).tx.safeTransferFrom(from, to, tokenId);
                };

                const shouldTransferSafely = function (transferFun, data) {
                    describe('to a user account', function () {
                        shouldTransferTokensByUsers(transferFun);
                    });

                    describe('to a valid receiver contract', function () {
                        shouldTransferTokensByUsers(transferFun);

                        it('calls onERC721Received', async function () {
                            logs = transferFun.call(this, this.owner, this.receiver.address, tokenId, { from: this.owner });
                            await logs;

                            await expectEventInLogs(logs, this.receiver, 'Erc721Received',
                                this.owner,
                                this.owner,
                                tokenId,
                                data,
                            );
                        });

                        it('calls onERC721Received from approved', async function () {
                            logs = transferFun.call(this, this.owner, this.receiver.address, tokenId, { from: this.approved });
                            await logs;

                            await expectEventInLogs(logs, this.receiver, 'Erc721Received',
                                this.approved,
                                this.owner,
                                tokenId,
                                data,
                            );
                        });

                        describe('with an invalid token id', function () {
                            it('reverts', async function () {
                                await expectRevert(
                                    transferFun.call(
                                        this,
                                        this.owner,
                                        this.receiver.address,
                                        nonExistentTokenId,
                                        { from: this.owner },
                                    ),
                                    'ERC721: operator query for nonexistent token',
                                );
                            });
                        });
                    });
                };

                describe('with data', function () {
                    shouldTransferSafely(safeTransferFromWithData, data);
                });

                describe('without data', function () {
                    shouldTransferSafely(safeTransferFromWithoutData, null);
                });

                describe('to a receiver contract returning unexpected value', function () {
                    it('reverts', async function () {
                        await this.receiver.connect(this.sender).tx.delAcceptToken(this.token.address);

                        await expectRevert(
                            this.token.safeTransferFrom(this.owner, this.receiver.address, tokenId, { from: this.owner }),
                            'ERC721: transfer to non ERC721Receiver implementer',
                        );
                    });
                });

                describe('to a receiver contract that panics', function () {
                    it('reverts', async function () {
                        await this.receiver.connect(this.sender).tx.setReceiveStatus(false);

                        await expectRevert(
                            this.token.safeTransferFrom(this.owner, this.receiver.address, tokenId, { from: this.owner }),
                            'ERC721: transfer to non ERC721Receiver implementer',
                        );
                    });
                });

                describe('to a contract that does not implement the required function', function () {
                    it('reverts', async function () {
                        const nonReceiver = this.token;
                        await expectRevert(
                            this.token.safeTransferFrom(this.owner, nonReceiver.address, tokenId, { from: this.owner }),
                            'ERC721: transfer to non ERC721Receiver implementer',
                        );
                    });
                });
            });
        });

        /*
        describe('safe mint', function () {
            const fourthTokenId = new BN(4);
            const tokenId = fourthTokenId;
            const data = '0x42';
 
            describe('via safeMint', function () { // regular minting is tested in ERC721Mintable.test.js and others
                it('calls onERC721Received — with data', async function () {
                    this.receiver = await ERC721ReceiverMock.new(RECEIVER_MAGIC_VALUE, Error.None);
                    const receipt = await this.token.safeMint(this.receiver.address, tokenId, data);
 
                    await expectEvent.inTransaction(receipt.tx, ERC721ReceiverMock, 'Received', {
                        from: ZERO_ADDRESS,
                        tokenId: tokenId,
                        data: data,
                    });
                });
 
                it('calls onERC721Received — without data', async function () {
                    this.receiver = await ERC721ReceiverMock.new(RECEIVER_MAGIC_VALUE, Error.None);
                    const receipt = await this.token.safeMint(this.receiver.address, tokenId);
 
                    await expectEvent.inTransaction(receipt.tx, ERC721ReceiverMock, 'Received', {
                        from: ZERO_ADDRESS,
                        tokenId: tokenId,
                    });
                });
 
                context('to a receiver contract returning unexpected value', function () {
                    it('reverts', async function () {
                        const invalidReceiver = await ERC721ReceiverMock.new('0x42', Error.None);
                        await expectRevert(
                            this.token.safeMint(invalidReceiver.address, tokenId),
                            'ERC721: transfer to non ERC721Receiver implementer',
                        );
                    });
                });
 
                context('to a receiver contract that reverts with message', function () {
                    it('reverts', async function () {
                        const revertingReceiver = await ERC721ReceiverMock.new(RECEIVER_MAGIC_VALUE, Error.RevertWithMessage);
                        await expectRevert(
                            this.token.safeMint(revertingReceiver.address, tokenId),
                            'ERC721ReceiverMock: reverting',
                        );
                    });
                });
 
                context('to a receiver contract that reverts without message', function () {
                    it('reverts', async function () {
                        const revertingReceiver = await ERC721ReceiverMock.new(RECEIVER_MAGIC_VALUE, Error.RevertWithoutMessage);
                        await expectRevert(
                            this.token.safeMint(revertingReceiver.address, tokenId),
                            'ERC721: transfer to non ERC721Receiver implementer',
                        );
                    });
                });
 
                context('to a receiver contract that panics', function () {
                    it('reverts', async function () {
                        const revertingReceiver = await ERC721ReceiverMock.new(RECEIVER_MAGIC_VALUE, Error.Panic);
                        await expectRevert.unspecified(
                            this.token.safeMint(revertingReceiver.address, tokenId),
                        );
                    });
                });
 
                context('to a contract that does not implement the required function', function () {
                    it('reverts', async function () {
                        const nonReceiver = this.token;
                        await expectRevert(
                            this.token.safeMint(nonReceiver.address, tokenId),
                            'ERC721: transfer to non ERC721Receiver implementer',
                        );
                    });
                });
            });
        });
        */

        describe('approve', function () {
            const tokenId = firstTokenId;

            let logs = null;

            const itClearsApproval = function () {
                it('clears approval for the token', async function () {
                    expect((await this.token.query.getApproved(tokenId)).output).to.equal(null);
                });
            };

            const itApproves = function (name) {
                it('sets the approval for the target address', async function () {
                    expect((await this.token.query.getApproved(tokenId)).output).to.equal(this[name]);
                });
            };

            const itEmitsApprovalEvent = function (name) {
                it('emits an approval event', async function () {
                    expectEventInLogs(logs, this.token, 'Approval',
                        this.owner,
                        this[name],
                        tokenId,
                    );
                });
            };

            context('when clearing approval', function () {
                context('when there was no prior approval', function () {
                    beforeEach(async function () {
                        logs = this.token.connect(this.owner).tx.approve(null, tokenId);
                        await logs;
                    });

                    itClearsApproval();
                    itEmitsApprovalEvent(null);
                });

                context('when there was a prior approval', function () {
                    beforeEach(async function () {
                        await this.token.connect(this.owner).tx.approve(this.approved, tokenId);
                        logs = this.token.connect(this.owner).tx.approve(null, tokenId);
                        await logs;
                    });

                    itClearsApproval();
                    itEmitsApprovalEvent(null);
                });
            });

            context('when approving a non-zero address', async function () {
                context('when there was no prior approval', async function () {
                    beforeEach(async function () {
                        logs = this.token.connect(this.owner).tx.approve(this.approved, tokenId);
                        await logs;
                    });

                    itApproves('approved');
                    itEmitsApprovalEvent('approved');
                });

                context('when there was a prior approval to the same address', function () {
                    beforeEach(async function () {
                        await this.token.connect(this.owner).tx.approve(this.approved, tokenId);
                        ({ logs } = await this.token.connect(this.owner).tx.approve(this.approved, tokenId));
                    });

                    itApproves('approved');
                    itEmitsApprovalEvent('approved');
                });

                context('when there was a prior approval to a different address', function () {
                    beforeEach(async function () {
                        await this.token.connect(this.owner).tx.approve(this.anotherApproved, tokenId);
                        ({ logs } = await this.token.connect(this.owner).tx.approve(this.anotherApproved, tokenId));
                    });

                    itApproves('anotherApproved');
                    itEmitsApprovalEvent('anotherApproved');
                });
            });

            context('when the address that receives the approval is the this.owner', function () {
                it('reverts', async function () {
                    await expectRevert(
                        this.token.connect(this.owner).tx.approve(this.owner, tokenId), 'ERC721: approval to current this.owner',
                    );
                });
            });

            context('when the sender does not own the given token ID', function () {
                it('reverts', async function () {
                    await expectRevert(this.token.connect(this.other).tx.approve(this.approved, tokenId),
                        'ERC721: approve caller is not this.owner nor approved');
                });
            });

            context('when the sender is approved for the given token ID', function () {
                it('reverts', async function () {
                    await this.token.connect(this.owner).tx.approve(this.approved, tokenId);
                    await expectRevert(this.token.connect(this.approved).tx.approve(this.anotherApproved, tokenId),
                        'ERC721: approve caller is not this.owner nor approved for all');
                });
            });

            context('when the sender is an operator', function () {
                beforeEach(async function () {
                    await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);
                    logs = this.token.connect(this.operator).tx.approve(this.approved, tokenId);
                    await logs;
                });

                itApproves('approved');
                itEmitsApprovalEvent('approved');
            });

            context('when the given token ID does not exist', function () {
                it('reverts', async function () {
                    await expectRevert(this.token.connect(this.operator).tx.approve(this.approved, nonExistentTokenId),
                        'ERC721: this.owner query for nonexistent token');
                });
            });
        });

        describe('setApprovalForAll', function () {
            context('when the operator willing to approve is not the this.owner', function () {
                context('when there is no operator approval set by the sender', function () {
                    it('approves the operator', async function () {
                        await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);

                        expect((await this.token.query.isApprovedForAll(this.owner, this.operator)).output).to.equal(true);
                    });

                    it('emits an approval event', async function () {
                        let logs = this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);
                        await logs;

                        expectEventInLogs(logs, this.token, 'ApprovalForAll',
                            this.owner,
                            this.operator,
                            true,
                        );
                    });
                });

                context('when the operator was set as not approved', function () {
                    beforeEach(async function () {
                        await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, false);
                    });

                    it('approves the operator', async function () {
                        await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);

                        expect((await this.token.query.isApprovedForAll(this.owner, this.operator)).output).to.equal(true);
                    });

                    it('emits an approval event', async function () {
                        let logs = this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);
                        await logs;

                        expectEventInLogs(logs, this.token, 'ApprovalForAll',
                            this.owner,
                            this.operator,
                            true,
                        );
                    });

                    it('can unset the operator approval', async function () {
                        await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, false);

                        expect((await this.token.query.isApprovedForAll(this.owner, this.operator)).output).to.equal(false);
                    });
                });

                context('when the operator was already approved', function () {
                    beforeEach(async function () {
                        await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);
                    });

                    it('keeps the approval to the given address', async function () {
                        await this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);

                        expect((await this.token.query.isApprovedForAll(this.owner, this.operator)).output).to.equal(true);
                    });

                    it('emits an approval event', async function () {
                        let logs = this.token.connect(this.owner).tx.setApprovalForAll(this.operator, true);
                        await logs;

                        expectEventInLogs(logs, this.token, 'ApprovalForAll',
                            this.owner,
                            this.operator,
                            true,
                        );
                    });
                });
            });

            context('when the operator is the this.owner', function () {
                it('reverts', async function () {
                    await expectRevert(this.token.connect(this.owner).tx.setApprovalForAll(this.owner, true),
                        'ERC721: approve to caller');
                });
            });
        });

        describe('getApproved', async function () {
            context('when token is not minted', async function () {
                // TODO: no reverts for query in ink!
                // it('reverts', async function () {
                //    await expectRevert(
                //      this.token.query.getApproved(nonExistentTokenId),
                //      'ERC721: approved query for nonexistent token',
                //     );
                // });
            });

            context('when token has been minted ', async function () {
                it('should return the zero address', async function () {
                    expect((await this.token.query.getApproved(firstTokenId)).output).to.equal(
                        null,
                    );
                });

                context('when account has been approved', async function () {
                    beforeEach(async function () {
                        await this.token.connect(this.owner).tx.approve(this.approved, firstTokenId);
                    });

                    it('returns approved account', async function () {
                        expect((await this.token.query.getApproved(firstTokenId)).output).to.equal(this.approved);
                    });
                });
            });
        });
    });

    describe('_mint(address, uint256)', function () {
        let logs = null;

        it('reverts with a null destination address', async function () {
            await expectRevert(
                this.token.tx.mint(ZERO_ADDRESS, firstTokenId), 'ERC721: mint to the zero address',
            );
        });

        context('with minted token', async function () {
            beforeEach(async function () {
                logs = this.token.tx.mint(this.owner, firstTokenId);
                await logs;
            });

            it('emits a Transfer event', function () {
                expectEventInLogs(this.logs, this.token, 'Transfer', ZERO_ADDRESS, this.owner, firstTokenId);
            });

            it('creates the token', async function () {
                expect((await this.token.query.balanceOf(this.owner)).output).to.equal(1);
                expect((await this.token.query.ownerOf(firstTokenId)).output).to.equal(this.owner);
            });

            it('reverts when adding a token id that already exists', async function () {
                await expectRevert(this.token.tx.mint(this.owner, firstTokenId), 'ERC721: token already minted');
            });
        });
    });

    describe('_burn', function () {
        let logs = null;

        it('reverts when burning a non-existent token id', async function () {
            await expectRevert(
                this.token.tx.burn(firstTokenId), 'ERC721: this.owner query for nonexistent token',
            );
        });

        context('with minted tokens', function () {
            beforeEach(async function () {
                await this.token.tx.mint(this.owner, firstTokenId);
                await this.token.tx.mint(this.owner, secondTokenId);
            });

            context('with burnt token', function () {
                beforeEach(async function () {
                    logs = this.token.tx.burn(firstTokenId);
                    await logs;
                });

                it('emits a Transfer event', function () {
                    expectEventInLogs(this.logs, this.token, 'Transfer', this.owner, ZERO_ADDRESS, firstTokenId);
                });

                it('emits an Approval event', function () {
                    expectEventInLogs(this.logs, this.token, 'Approval', this.owner, ZERO_ADDRESS, firstTokenId);
                });

                it('deletes the token', async function () {
                    expect((await this.token.query.balanceOf(this.owner)).output).to.equal('1');
                    await expectRevert(
                        this.token.query.ownerOf(firstTokenId), 'ERC721: this.owner query for nonexistent token',
                    );
                });

                it('reverts when burning a token id that has been deleted', async function () {
                    await expectRevert(
                        this.token.tx.burn(firstTokenId), 'ERC721: this.owner query for nonexistent token',
                    );
                });
            });
        });
    });

}

/*

function shouldBehaveLikeERC721Enumerable(errorPrefix, this.owner, newOwner, approved, anotherApproved, operator, this.other) {
    shouldSupportInterfaces([
        'ERC721Enumerable',
    ]);

    context('with minted tokens', function () {
        beforeEach(async function () {
            await this.token.mint(this.owner, firstTokenId);
            await this.token.mint(this.owner, secondTokenId);
            this.toWhom = this.other; // default to this.other for toWhom in context-dependent tests
        });

        describe('totalSupply', function () {
            it('returns total token supply', async function () {
                expect(await this.token.totalSupply()).to.bignumber.equal('2');
            });
        });

        describe('tokenOfOwnerByIndex', function () {
            describe('when the given index is lower than the amount of tokens owned by the given address', function () {
                it('returns the token ID placed at the given index', async function () {
                    expect(await this.token.tokenOfOwnerByIndex(this.owner, 0)).to.bignumber.equal(firstTokenId);
                });
            });

            describe('when the index is greater than or equal to the total tokens owned by the given address', function () {
                it('reverts', async function () {
                    await expectRevert(
                        this.token.tokenOfOwnerByIndex(this.owner, 2), 'ERC721Enumerable: this.owner index out of bounds',
                    );
                });
            });

            describe('when the given address does not own any token', function () {
                it('reverts', async function () {
                    await expectRevert(
                        this.token.tokenOfOwnerByIndex(this.other, 0), 'ERC721Enumerable: this.owner index out of bounds',
                    );
                });
            });

            describe('after transferring all tokens to another user', function () {
                beforeEach(async function () {
                    await this.token.transferFrom(this.owner, this.other, firstTokenId, { from: this.owner });
                    await this.token.transferFrom(this.owner, this.other, secondTokenId, { from: this.owner });
                });

                it('returns correct token IDs for target', async function () {
                    expect(await this.token.balanceOf(this.other)).to.bignumber.equal('2');
                    const tokensListed = await Promise.all(
                        [0, 1].map(i => this.token.tokenOfOwnerByIndex(this.other, i)),
                    );
                    expect(tokensListed.map(t => t.toNumber())).to.have.members([firstTokenId.toNumber(),
                    secondTokenId.toNumber()]);
                });

                it('returns empty collection for original this.owner', async function () {
                    expect(await this.token.balanceOf(this.owner)).to.bignumber.equal('0');
                    await expectRevert(
                        this.token.tokenOfOwnerByIndex(this.owner, 0), 'ERC721Enumerable: this.owner index out of bounds',
                    );
                });
            });
        });

        describe('tokenByIndex', function () {
            it('returns all tokens', async function () {
                const tokensListed = await Promise.all(
                    [0, 1].map(i => this.token.tokenByIndex(i)),
                );
                expect(tokensListed.map(t => t.toNumber())).to.have.members([firstTokenId.toNumber(),
                secondTokenId.toNumber()]);
            });

            it('reverts if index is greater than supply', async function () {
                await expectRevert(
                    this.token.tokenByIndex(2), 'ERC721Enumerable: global index out of bounds',
                );
            });

            [firstTokenId, secondTokenId].forEach(function (tokenId) {
                it(`returns all tokens after burning token ${tokenId} and minting new tokens`, async function () {
                    const newTokenId = new BN(300);
                    const anotherNewTokenId = new BN(400);

                    await this.token.burn(tokenId);
                    await this.token.mint(newOwner, newTokenId);
                    await this.token.mint(newOwner, anotherNewTokenId);

                    expect(await this.token.totalSupply()).to.bignumber.equal('3');

                    const tokensListed = await Promise.all(
                        [0, 1, 2].map(i => this.token.tokenByIndex(i)),
                    );
                    const expectedTokens = [firstTokenId, secondTokenId, newTokenId, anotherNewTokenId].filter(
                        x => (x !== tokenId),
                    );
                    expect(tokensListed.map(t => t.toNumber())).to.have.members(expectedTokens.map(t => t.toNumber()));
                });
            });
        });
    });

    describe('_mint(address, uint256)', function () {
        it('reverts with a null destination address', async function () {
            await expectRevert(
                this.token.mint(ZERO_ADDRESS, firstTokenId), 'ERC721: mint to the zero address',
            );
        });

        context('with minted token', async function () {
            beforeEach(async function () {
                ({ logs: this.logs } = await this.token.mint(this.owner, firstTokenId));
            });

            it('adjusts this.owner tokens by index', async function () {
                expect(await this.token.tokenOfOwnerByIndex(this.owner, 0)).to.bignumber.equal(firstTokenId);
            });

            it('adjusts all tokens list', async function () {
                expect(await this.token.tokenByIndex(0)).to.bignumber.equal(firstTokenId);
            });
        });
    });

    describe('_burn', function () {
        it('reverts when burning a non-existent token id', async function () {
            await expectRevert(
                this.token.burn(firstTokenId), 'ERC721: this.owner query for nonexistent token',
            );
        });

        context('with minted tokens', function () {
            beforeEach(async function () {
                await this.token.mint(this.owner, firstTokenId);
                await this.token.mint(this.owner, secondTokenId);
            });

            context('with burnt token', function () {
                beforeEach(async function () {
                    ({ logs: this.logs } = await this.token.burn(firstTokenId));
                });

                it('removes that token from the token list of the this.owner', async function () {
                    expect(await this.token.tokenOfOwnerByIndex(this.owner, 0)).to.bignumber.equal(secondTokenId);
                });

                it('adjusts all tokens list', async function () {
                    expect(await this.token.tokenByIndex(0)).to.bignumber.equal(secondTokenId);
                });

                it('burns all tokens', async function () {
                    await this.token.burn(secondTokenId, { from: this.owner });
                    expect(await this.token.totalSupply()).to.bignumber.equal('0');
                    await expectRevert(
                        this.token.tokenByIndex(0), 'ERC721Enumerable: global index out of bounds',
                    );
                });
            });
        });
    });
}

*/



function shouldBehaveLikeERC721Metadata(errorPrefix, contractName) {
    // TODO: interface test
    //shouldSupportInterfaces([
    //    'ERC721Metadata',
    //]);

    async function setup() {
        await api.isReady
        const signerAddresses = await getAddresses();
        const Alice = signerAddresses[0];
        const sender = await getRandomSigner(Alice, "10000 UNIT");
        const abi = artifacts.readArtifact("erc721");

        return { sender, signerAddresses };
    }

    beforeEach(async function () {
        const { signerAddresses, sender } = await setup();
        const contractFactory = await getContractFactory(contractName, sender.address);
        let res = await contractFactory.deploy("new", "NFTCoin", "NFT");
        this.token = res;

        this.sender = signerAddresses[0]
        this.owner = signerAddresses[0]
        this.newOwner = signerAddresses[1]
        this.approved = signerAddresses[2]
        this.anotherApproved = signerAddresses[3]
        this.operator = signerAddresses[4]
        this.other = signerAddresses[5]
    });

    describe('metadata', function () {
        it('has a name', async function () {
            expect((await this.token.query.name()).output).to.equal("NFTCoin");
        });

        it('has a symbol', async function () {
            expect((await this.token.query.symbol()).output).to.equal("NFT");
        });

        describe('token URI', function () {
            beforeEach(async function () {
                await this.token.tx.mint(this.owner, firstTokenId);
            });

            it('return empty string by default', async function () {
                expect((await this.token.query.tokenUrl(firstTokenId)).output).to.equal(null);
            });

            it('reverts when queried for non existent token id', async function () {
                await expectRevert(
                    this.token.query.tokenUrl(nonExistentTokenId), 'ERC721Metadata: URI query for nonexistent token',
                );
            });

            describe('base URI', function () {
                beforeEach(function () {
                    if (this.token.tx.setBaseUrl === undefined) {
                        this.skip();
                    }
                });

                it('base URI can be set', async function () {
                    await this.token.tx.setBaseUrl(baseURI);
                    expect((await this.token.query.baseURI()).output).to.equal(baseURI);
                });

                it('base URI is added as a prefix to the token URI', async function () {
                    await this.token.tx.setBaseUrl(baseURI);
                    expect((await this.token.query.tokenURI(firstTokenId)).output).to.equal(baseURI + firstTokenId.toString());
                });

                it('token URI can be changed by changing the base URI', async function () {
                    await this.token.tx.setBaseUrl(baseURI);
                    const newBaseURI = 'https://api.com/v2/';
                    await this.token.tx.setBaseUrl(newBaseURI);
                    expect((await this.token.query.tokenURI(firstTokenId)).output).to.equal(newBaseURI + firstTokenId.toString());
                });
            });
        });
    });

}



module.exports = {
    shouldBehaveLikeERC721,
    // shouldBehaveLikeERC721Enumerable,
    shouldBehaveLikeERC721Metadata,
};
