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
    Assert.equal(dao.num_proposals(), 0, "should be 0");
    dao.create_proposal("foo");
    Assert.equal(dao.num_proposals(), 1, "should be 1");
    Assert.equal(dao.proposal_yes_votes(0), 0, "");
    Assert.isTrue(keccak256(bytes(dao.proposal_name(0))) == keccak256("foo"), "");
  }

  function test_change_proposal_name() public {
    Dao dao = new Dao();
    Assert.equal(dao.num_proposals(), 0, "should be 0");
    dao.create_proposal("foo");
    Assert.equal(dao.num_proposals(), 1, "should be 1");
    Assert.isTrue(keccak256(bytes(dao.proposal_name(0))) == keccak256("foo"), "");
    dao.change_proposal_name(0, "aaa");
    Assert.isTrue(keccak256(bytes(dao.proposal_name(0))) == keccak256("aaa"), "");
  }  

}
