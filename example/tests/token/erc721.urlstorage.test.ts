import { network } from "redspot";
const { api } = network;

const { shouldBehaveLikeERC721, shouldBehaveLikeERC721Metadata } = require("./erc721.behavior");

describe("ERC721UrlStorage", async () => {
  before(async function( ){
    console.log("start erc721_urlstorage")
  });
  await shouldBehaveLikeERC721Metadata('ERC721', "erc721_urlstorage");
  await shouldBehaveLikeERC721('ERC721', "erc721_urlstorage");
});