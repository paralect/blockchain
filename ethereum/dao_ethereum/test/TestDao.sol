// SPDX-License-Identifier: MIT
pragma solidity 0.8.18;

// These files are dynamically created at test time
import "truffle/Assert.sol";
  // See: 
  // https://github.com/trufflesuite/truffle/blob/develop/packages/resolver/solidity/Assert.sol
import "truffle/DeployedAddresses.sol";
import "../contracts/Dao.sol";

contract TestDao {

  function test_create_proposal() public {
    Dao dao = new Dao();
    Assert.equal(0, dao.num_proposals(), "should be 0");
    dao.create_proposal("foo");
    Assert.equal(1, dao.num_proposals(), "should be 1");

    // Assert.isTrue(keccak256(bytes(dao.proposals(0).name)) == keccak256("foo"), "aa");
  }

}
