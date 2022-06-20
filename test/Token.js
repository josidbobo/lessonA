const { expect } = require("chai");

describe("Token Contract", function () {
  beforeEach( async function () {
    [owner, addr1, addr2, addr3] = await ethers.getSigners();

    const Token = await ethers.getContractFactory("Token");

    hardhatToken = await Token.deploy();
    await hardhatToken.deployed();

  });
  describe("Transactions", function () {
    beforeEach(async function (){
       // Transfer 50 tokens from owner to addr1
    await hardhatToken.transfer(addr1.address, 50);

    });
  it("Should transfer tokens from owner to another account", async function () {
    expect(await hardhatToken.balanceOf(addr1.address)).to.equal(50);
  });

  it("Should decrement the owner balance after transferring", async function (){
    // Deduct the transferred amount from owner
    expect(await hardhatToken.balanceOf(owner.address)).to.equal(999950);
  });

  it("Should transfer tokens from one address to another", async function (){
    // Transfer 50 tokens from addr1 to addr2
    await hardhatToken.connect(addr1).transfer(addr2.address, 50);
    expect(await hardhatToken.balanceOf(addr2.address)).to.equal(50);
  });
});

describe("Balances", function (){
  it("Should return zero for an account without tokens", async function (){
    expect(await hardhatToken.balanceOf(addr3.address)).to.be.equal(0);
  });

});

});
