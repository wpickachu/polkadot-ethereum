const BasicOutboundChannel = artifacts.require("BasicOutboundChannel");

const Web3Utils = require("web3-utils");
const ethers = require("ethers");
const BigNumber = web3.BigNumber;

const { confirmBasicChannelSend } = require("./helpers");

require("chai")
  .use(require("chai-as-promised"))
  .use(require("chai-bignumber")(BigNumber))
  .should();

describe("BasicOutboundChannel", function () {
  // Accounts
  let accounts;
  let appAddress;
  let origin;
  const payload = ethers.utils.formatBytes32String("arbitrary-payload");

  before(async function() {
    accounts = await web3.eth.getAccounts();
    appAddress = accounts[0];
    origin = accounts[1];
  });

  describe("submit messages", function () {
    beforeEach(async function () {
      this.channel = await BasicOutboundChannel.new();
    });

    it("should send messages out with the correct event and fields", async function () {
      const tx = await this.channel.submit(
        origin,
        payload,
        { from: appAddress, value: 0 }
      ).should.be.fulfilled;

      const rawLog = tx.receipt.rawLogs[0];
      confirmBasicChannelSend(rawLog, this.channel.address, appAddress, 1, payload)
    });

  });

});
