import { network } from "redspot";
const { api } = network;

const { shouldBehaveLikeERC721, shouldBehaveLikeERC721Metadata } = require("./erc721.behavior");

describe("ERC721", async () => {
    await shouldBehaveLikeERC721('ERC721', "erc721");
    await shouldBehaveLikeERC721Metadata('ERC721', "erc721");
});