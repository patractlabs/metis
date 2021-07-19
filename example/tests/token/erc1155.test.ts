import { network } from "redspot";
const { api } = network;

const { shouldBehaveLikeERC1155 } = require("./erc1155.behavior");

describe("ERC1155", async () => {
    await shouldBehaveLikeERC1155('ERC1155', "erc1155");
});