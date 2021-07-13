import { network, patract } from "redspot";
const { api } = network;

const { shouldBehaveLikeERC721, shouldBehaveLikeERC721Metadata } = require("./erc721.behavior");

describe("ERC721Burnable", async () => {
  before(async function(){
    console.log("start erc721_burnable")
  });

  
  await shouldBehaveLikeERC721Metadata('ERC721', "erc721_burnable");
  await shouldBehaveLikeERC721('ERC721', "erc721_burnable");
});