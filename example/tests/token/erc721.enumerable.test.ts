import { network, patract } from "redspot";
const { api } = network;

const { shouldBehaveLikeERC721, shouldBehaveLikeERC721Metadata } = require("./erc721.behavior");

describe("ERC721Enumerable", async () => {
  before(async function(){
    console.log("start erc721_enumerable")
  });


  await shouldBehaveLikeERC721Metadata('ERC721', "erc721_enumerable");
  await shouldBehaveLikeERC721('ERC721', "erc721_enumerable");
});