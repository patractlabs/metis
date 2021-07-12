import { expect } from "chai";
import { artifacts, network, patract } from "redspot";
import { hexToU8a } from '@polkadot/util';

const { getContractFactory, getRandomSigner } = patract;
const { api, getAddresses, getSigners } = network;

const { shouldBehaveLikeERC721, shouldBehaveLikeERC721Metadata, shouldBehaveLikeERC721Enumerable } = require("./erc721.behavior");

const firstTokenId = hexToU8a('0x0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a0a')
const secondTokenId = hexToU8a('0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b')
const nonExistentTokenId = hexToU8a('0x0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c0c')
const baseURI = 'https://api.com/v1/';


describe("ERC721", async () => {
  after(() => {
    return api.disconnect();
  });

  await shouldBehaveLikeERC721Metadata('ERC721', "erc721");
  await shouldBehaveLikeERC721('ERC721', "erc721");
});
